//! Plan 8: agent profiles, the backlog and `next` (HTTP). The rules that pick an item for an
//! agent are `scholion_core::routing`; this module only moves data and starts or claims tasks.

use std::collections::HashSet;

use axum::extract::{Path, Query, State};
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use chrono::Utc;
use scholion_core::routing::{self, BacklogItem, BacklogState, NewBacklogItem, Profile};
use scholion_core::*;
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::AppState;
use crate::api::{create_started, first_pack_request, pack_for_task};
use crate::error::ApiResult;
use crate::flow::claim_as;

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/v1/agents", get(list_agents))
        .route("/v1/agents/{name}", put(put_agent))
        .route("/v1/backlog", post(post_backlog).get(list_backlog))
        .route("/v1/backlog/next", post(next))
        .route("/v1/backlog/{id}", delete(drop_backlog))
}

async fn list_agents(State(st): State<AppState>) -> ApiResult<Value> {
    Ok(Json(json!({ "agents": st.store.list_profiles().await? })))
}

#[derive(Deserialize)]
struct AgentBody {
    #[serde(default)]
    best: Vec<String>,
    #[serde(default)]
    can: Vec<String>,
    #[serde(default = "default_cost_rank")]
    cost_rank: i32,
    #[serde(default)]
    aliases: Vec<String>,
}

fn default_cost_rank() -> i32 {
    5
}

async fn put_agent(
    State(st): State<AppState>,
    Path(name): Path<String>,
    Json(b): Json<AgentBody>,
) -> ApiResult<Value> {
    let name = name.trim().to_owned();
    st.store
        .put_profile(&Profile {
            name: name.clone(),
            best: b.best,
            can: b.can,
            cost_rank: b.cost_rank,
            aliases: b.aliases,
        })
        .await?;
    let stored = st
        .store
        .list_profiles()
        .await?
        .into_iter()
        .find(|p| p.name == name);
    Ok(Json(json!({ "agent": stored })))
}

#[derive(Deserialize)]
struct PostBacklog {
    repo: String,
    title: String,
    goal: String,
    #[serde(default)]
    needs: Vec<String>,
    prefer: Option<String>,
    #[serde(default)]
    priority: i32,
    reserve_secs: Option<i64>,
    posted_by: Option<String>,
}

async fn post_backlog(
    State(st): State<AppState>,
    Json(b): Json<PostBacklog>,
) -> ApiResult<routing::BacklogEntry> {
    let entry = st
        .store
        .post_backlog(NewBacklogItem {
            repo: b.repo,
            title: b.title,
            goal: b.goal,
            needs: b.needs,
            prefer: b.prefer,
            priority: b.priority,
            reserve_secs: b.reserve_secs.unwrap_or(routing::DEFAULT_RESERVE_SECS),
            posted_by: b.posted_by.unwrap_or_else(|| "user".into()),
        })
        .await?;
    Ok(Json(entry))
}

#[derive(Deserialize)]
struct ListBacklog {
    repo: Option<String>,
    state: Option<String>,
}

async fn list_backlog(
    State(st): State<AppState>,
    Query(q): Query<ListBacklog>,
) -> ApiResult<Value> {
    let state = q
        .state
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(str::parse::<BacklogState>)
        .transpose()?;
    let items = st.store.list_backlog(q.repo.as_deref(), state).await?;
    // the agent each item is reserved for first, so a client need not know the ranking rules
    let profiles = st.store.list_profiles().await?;
    let items: Vec<Value> = items
        .iter()
        .map(|e| {
            let mut v = json!(e);
            v["best_agent"] = json!(routing::rank(&e.item, &profiles).first());
            v
        })
        .collect();
    Ok(Json(json!({ "items": items })))
}

async fn drop_backlog(State(st): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Value> {
    st.store.drop_backlog(id).await?;
    Ok(Json(json!({ "id": id, "state": "dropped" })))
}

#[derive(Deserialize)]
struct NextBody {
    repo: Option<String>,
    agent: String,
    session: String,
    budget: Option<u32>,
    family: Option<AgentFamily>,
}

/// The one item that fits the caller, made into its task: a queued backlog item is started (a
/// workspace and a first pack, like `start_task`), a handed-off task that names tags is claimed
/// (the handoff bundle, like `claim_task`). With nothing to give it returns `{"task": null,
/// "queued": N}`: N queued items, none of which the caller may take now.
async fn next(State(st): State<AppState>, Json(b): Json<NextBody>) -> ApiResult<Value> {
    let agent = b.agent.trim().to_owned();
    if agent.is_empty() {
        return Err(AvError::Invalid("agent must not be empty".into()).into());
    }
    let profiles = st.store.list_profiles().await?;
    let agent = routing::resolve(&profiles, &agent)
        .map(|p| p.name.clone())
        .unwrap_or(agent);

    let queued = st
        .store
        .list_backlog(b.repo.as_deref(), Some(BacklogState::Queued))
        .await?;
    let handed = st.store.handed_off_with_needs(b.repo.as_deref()).await?;
    // A handed-off task is a candidate under its own id, the reserve window starting when its
    // tags were set.
    let mut items: Vec<BacklogItem> = queued.iter().map(|e| e.item.clone()).collect();
    let mut handoffs: HashSet<Uuid> = HashSet::new();
    for h in &handed {
        handoffs.insert(h.task.id.0);
        items.push(BacklogItem {
            id: h.task.id.0,
            repo: h.task.repo.clone(),
            title: h.task.title.clone(),
            goal: h.task.goal.clone(),
            needs: h.needs.clone(),
            prefer: h.prefer.clone(),
            priority: 0,
            reserve_secs: routing::DEFAULT_RESERVE_SECS,
            created_at: h.since,
        });
    }

    // An item that cannot be made into a task goes back to the queue and the next one is tried,
    // so one bad item (a repo that is gone) cannot block the rest. The first error is returned
    // only when nothing else worked.
    let mut first_err: Option<AvError> = None;
    for item in routing::order_for(&agent, &items, &profiles, Utc::now()) {
        let made = if handoffs.contains(&item.id) {
            claim_as(&st, TaskId(item.id), &agent, b.budget, b.family)
                .await
                .map(|mut v| {
                    v["from"] = json!("handoff");
                    v
                })
        } else {
            start_from_backlog(&st, item, &agent, &b).await
        };
        match made {
            Ok(v) => return Ok(Json(v)),
            // someone else got there first
            Err(AvError::IllegalState(_)) => {}
            Err(e) => {
                first_err.get_or_insert(e);
            }
        }
    }
    match first_err {
        Some(e) => Err(e.into()),
        None => Ok(Json(json!({ "task": null, "queued": queued.len() }))),
    }
}

/// Takes `item` for `agent` and starts its task. Waits for a start turn first, like `start_task`,
/// and makes the task on a task of its own, so a client that leaves cannot stop it half done.
async fn start_from_backlog(
    st: &AppState,
    item: &BacklogItem,
    agent: &str,
    b: &NextBody,
) -> Result<Value> {
    let turn = st
        .starts
        .clone()
        .acquire_owned()
        .await
        .map_err(|e| AvError::Store(format!("start queue closed: {e}")))?;
    if !st.store.take_backlog(item.id, agent).await? {
        return Err(AvError::IllegalState("the item was taken".into()));
    }
    let task = {
        let st = st.clone();
        let (item, agent) = (item.clone(), agent.to_owned());
        tokio::spawn(async move {
            let _turn = turn;
            match create_started(&st, item.repo, item.title, item.goal, agent, PackMode::Full).await
            {
                Ok(task) => {
                    st.store.link_backlog_task(item.id, task.id).await?;
                    Ok(task)
                }
                Err(e) => {
                    // no task exists, so the item can be offered again
                    let _ = st.store.untake_backlog(item.id).await;
                    Err(e)
                }
            }
        })
        .await
        .map_err(|e| AvError::Store(format!("task start stopped: {e}")))??
    };
    let req = first_pack_request(task.id, b.budget, b.session.clone());
    let (task, pack) = pack_for_task(st, task.id, req).await?;
    Ok(json!({ "task": task, "pack": pack, "from": "backlog", "backlog_id": item.id }))
}
