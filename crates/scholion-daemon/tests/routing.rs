//! Plan 8 Task 4: agent profiles, the backlog and `next` over the HTTP API.

mod common;
use common::*;

async fn put_agent(e: &Env, name: &str, best: &[&str], can: &[&str], cost: i32) {
    let (status, body) = call(
        &e.app,
        "PUT",
        &format!("/v1/agents/{name}"),
        Some(json!({"best": best, "can": can, "cost_rank": cost})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
}

async fn post_item(e: &Env, repo: &str, title: &str, needs: &[&str], reserve: i64) -> Value {
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/backlog",
        Some(
            json!({"repo": repo, "title": title, "goal": "Make hello louder",
                    "needs": needs, "reserve_secs": reserve}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body
}

async fn next(e: &Env, agent: &str) -> (StatusCode, Value) {
    call(
        &e.app,
        "POST",
        "/v1/backlog/next",
        Some(json!({"agent": agent, "session": "s-next"})),
    )
    .await
}

async fn backlog(e: &Env, query: &str) -> Vec<Value> {
    let (status, body) = call(&e.app, "GET", &format!("/v1/backlog{query}"), None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body["items"].as_array().unwrap().clone()
}

#[tokio::test]
async fn profiles_are_set_and_listed_cheapest_first() {
    let e = env().await;
    put_agent(&e, "claude", &["code"], &["docs"], 4).await;
    put_agent(&e, "muse", &[], &["code"], 1).await;
    let (status, body) = call(&e.app, "GET", "/v1/agents", None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let names: Vec<&str> = body["agents"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a["name"].as_str().unwrap())
        .collect();
    assert_eq!(names, ["muse", "claude"]);
}

#[tokio::test]
async fn a_blank_tag_in_a_profile_is_a_400() {
    let e = env().await;
    let (status, _) = call(
        &e.app,
        "PUT",
        "/v1/agents/grok",
        Some(json!({"best": [" "], "cost_rank": 2})),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn backlog_items_are_posted_listed_and_dropped() {
    let e = env().await;
    let a = post_item(&e, "demo", "first", &["Image"], 600).await;
    post_item(&e, "demo", "second", &[], 600).await;
    assert_eq!(a["state"], "queued");
    assert_eq!(a["needs"], json!(["image"]));
    assert_eq!(backlog(&e, "").await.len(), 2);
    assert_eq!(backlog(&e, "?repo=other").await.len(), 0);

    let id = a["id"].as_str().unwrap();
    let (status, _) = call(&e.app, "DELETE", &format!("/v1/backlog/{id}"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(backlog(&e, "?state=queued").await.len(), 1);
    assert_eq!(backlog(&e, "?state=dropped").await.len(), 1);
    // dropping twice, or an unknown id
    let (status, _) = call(&e.app, "DELETE", &format!("/v1/backlog/{id}"), None).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    let (status, _) = call(
        &e.app,
        "DELETE",
        "/v1/backlog/00000000-0000-0000-0000-000000000000",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn next_with_nothing_queued_says_so() {
    let e = env().await;
    let (status, body) = next(&e, "claude").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body, json!({"task": null, "queued": 0}));
}

#[tokio::test]
async fn next_gives_an_agent_the_item_that_fits_it() {
    let e = env().await;
    put_agent(&e, "muse", &[], &["code"], 1).await;
    put_agent(&e, "claude", &["code"], &[], 4).await;
    let item = post_item(&e, "demo", "louder hello", &["code"], 0).await;

    // reserve 0: both can take it, but claude ranks first (best at code)
    let (status, body) = next(&e, "claude").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["from"], "backlog");
    assert_eq!(body["task"]["owner_agent"], "claude");
    assert_eq!(body["task"]["title"], "louder hello");
    assert!(body["pack"].is_object(), "{body}");
    assert!(ws_dir(&e, &body).is_dir());

    let items = backlog(&e, "").await;
    assert_eq!(items[0]["state"], "taken");
    assert_eq!(items[0]["taken_by"], "claude");
    assert_eq!(items[0]["task_id"], body["task"]["id"]);
    assert_eq!(items[0]["id"], item["id"]);

    // nothing is left for the next agent
    let (_, again) = next(&e, "muse").await;
    assert_eq!(again, json!({"task": null, "queued": 0}));
}

#[tokio::test]
async fn inside_the_reserve_window_only_the_top_agent_takes_an_item() {
    let e = env().await;
    put_agent(&e, "muse", &[], &["code"], 1).await;
    put_agent(&e, "claude", &["code"], &[], 4).await;
    post_item(&e, "demo", "untagged", &[], 600).await;
    // untagged work: muse is the cheapest, so claude has to wait
    let (_, body) = next(&e, "claude").await;
    assert_eq!(body, json!({"task": null, "queued": 1}));
    let (_, body) = next(&e, "muse").await;
    assert_eq!(body["task"]["owner_agent"], "muse");
}

#[tokio::test]
async fn an_item_nobody_can_do_stays_queued() {
    let e = env().await;
    put_agent(&e, "muse", &[], &["code"], 1).await;
    put_agent(&e, "claude", &["code"], &[], 4).await;
    post_item(&e, "demo", "draw it", &["image"], 0).await;
    for agent in ["muse", "claude"] {
        let (_, body) = next(&e, agent).await;
        assert_eq!(body, json!({"task": null, "queued": 1}), "{agent}");
    }
    assert_eq!(backlog(&e, "?state=queued").await.len(), 1);
}

#[tokio::test]
async fn an_item_that_cannot_start_is_requeued_and_does_not_block_the_others() {
    let e = env().await;
    put_agent(&e, "muse", &[], &["code"], 1).await;
    // older, so it is tried first, but the repo does not exist
    post_item(&e, "no-such-repo", "broken", &[], 0).await;
    post_item(&e, "demo", "fine", &[], 0).await;
    let (status, body) = next(&e, "muse").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["task"]["title"], "fine");
    let queued = backlog(&e, "?state=queued").await;
    assert_eq!(queued.len(), 1);
    assert_eq!(queued[0]["title"], "broken");
    assert_eq!(queued[0]["taken_by"], Value::Null);
}

#[tokio::test]
async fn an_alias_resolves_to_the_profile_name_that_owns_the_task() {
    let e = env().await;
    let (status, body) = call(
        &e.app,
        "PUT",
        "/v1/agents/grok",
        Some(json!({"best": ["image"], "can": [], "cost_rank": 2,
                    "aliases": ["grok-shell-av"]})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    post_item(&e, "demo", "look at it", &["image"], 600).await;
    let (_, body) = next(&e, "grok-shell-av").await;
    assert_eq!(body["task"]["owner_agent"], "grok");
}

async fn hand_off_with(e: &Env, t: &Value, extra: Value) -> (StatusCode, Value) {
    let mut b = json!({"agent": "claude", "done": "hello shouts", "remaining": "draw the logo",
                       "broken": "", "questions": ""});
    for (k, v) in extra.as_object().unwrap() {
        b[k] = v.clone();
    }
    call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/handoff", id(t)),
        Some(b),
    )
    .await
}

#[tokio::test]
async fn a_handoff_with_tags_goes_to_the_agent_that_fits() {
    let e = env().await;
    put_agent(&e, "muse", &[], &["code"], 1).await;
    put_agent(&e, "grok", &["image"], &[], 2).await;
    let t = start_task(&e, "Make hello louder").await;
    edit_greeter(&e, &t);
    record_decision(&e, &t).await;
    let (status, body) = hand_off_with(&e, &t, json!({"needs": ["Image"]})).await;
    assert_eq!(status, StatusCode::OK, "{body}");

    // muse cannot do image work: the handed-off task is not offered to it
    let (_, body) = next(&e, "muse").await;
    assert_eq!(body["task"], Value::Null);

    let (status, body) = next(&e, "grok").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["from"], "handoff");
    assert_eq!(body["task"]["owner_agent"], "grok");
    assert_eq!(body["task"]["id"], t["task"]["id"]);
    assert_eq!(body["generation"], 2);
    assert!(body["bundle"].is_object(), "{body}");

    // claimed: the tags are gone, nothing is left to pull
    let (_, again) = next(&e, "grok").await;
    assert_eq!(again["task"], Value::Null);
}

#[tokio::test]
async fn a_handoff_with_a_blank_tag_is_refused_and_leaves_the_task_open() {
    let e = env().await;
    let t = start_task(&e, "Make hello louder").await;
    edit_greeter(&e, &t);
    record_decision(&e, &t).await;
    let (status, body) = hand_off_with(&e, &t, json!({"needs": [" "]})).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
    let (_, task) = call(&e.app, "GET", &format!("/v1/tasks/{}", id(&t)), None).await;
    assert_eq!(task["task"]["status"], "open", "{task}");
}

#[tokio::test]
async fn a_handoff_without_tags_is_not_pulled_by_next() {
    let e = env().await;
    put_agent(&e, "muse", &[], &["code"], 1).await;
    let t = start_task(&e, "Make hello louder").await;
    edit_greeter(&e, &t);
    record_decision(&e, &t).await;
    let (status, _) = handoff(&e, &t, "claude").await;
    assert_eq!(status, StatusCode::OK);
    let (_, body) = next(&e, "muse").await;
    assert_eq!(body["task"], Value::Null);
}

#[tokio::test]
async fn the_backlog_names_the_agent_that_ranks_first_for_each_item() {
    let e = env().await;
    put_agent(&e, "muse", &[], &["code"], 1).await;
    put_agent(&e, "claude", &["code"], &[], 4).await;
    post_item(&e, "demo", "untagged", &[], 600).await;
    post_item(&e, "demo", "code", &["code"], 600).await;
    post_item(&e, "demo", "image", &["image"], 600).await;
    let items = backlog(&e, "").await;
    let best = |title: &str| {
        items
            .iter()
            .find(|i| i["title"] == title)
            .map(|i| i["best_agent"].clone())
            .unwrap()
    };
    assert_eq!(best("untagged"), "muse"); // cheapest
    assert_eq!(best("code"), "claude"); // best at code
    assert_eq!(best("image"), Value::Null); // nobody can do it
}
