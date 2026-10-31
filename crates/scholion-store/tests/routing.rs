//! Plan 8 Task 3: agent profiles, the backlog and the handoff tags.

use scholion_core::routing::*;
use scholion_core::*;
use scholion_store::PgStore;
use scholion_testutil::TestDb;

async fn store() -> (TestDb, PgStore) {
    let db = TestDb::new().await;
    let store = PgStore::connect(&db.url).await.unwrap();
    (db, store)
}

fn profile(name: &str, best: &[&str], can: &[&str], cost: i32) -> Profile {
    Profile {
        name: name.into(),
        best: best.iter().map(|s| s.to_string()).collect(),
        can: can.iter().map(|s| s.to_string()).collect(),
        cost_rank: cost,
        aliases: vec![],
    }
}

fn new_item(repo: &str, title: &str, needs: &[&str]) -> NewBacklogItem {
    NewBacklogItem {
        repo: repo.into(),
        title: title.into(),
        goal: format!("goal of {title}"),
        needs: needs.iter().map(|s| s.to_string()).collect(),
        prefer: None,
        priority: 0,
        reserve_secs: DEFAULT_RESERVE_SECS,
        posted_by: "me".into(),
    }
}

fn new_task() -> NewTask {
    NewTask {
        repo: "demo".into(),
        title: "Pass it on".into(),
        goal: "goal".into(),
        owner_agent: "claude".into(),
    }
}

#[tokio::test]
async fn profiles_are_upserted_and_listed_cheapest_first() {
    let (_db, s) = store().await;
    s.put_profile(&profile("claude", &["code"], &["docs"], 4))
        .await
        .unwrap();
    s.put_profile(&profile("muse", &[], &["code"], 1))
        .await
        .unwrap();
    s.put_profile(&profile("grok", &["image"], &[], 2))
        .await
        .unwrap();
    // a second put replaces the first
    let mut c = profile("claude", &["code", "tests"], &[], 3);
    c.aliases = vec!["claude-code".into()];
    s.put_profile(&c).await.unwrap();

    let all = s.list_profiles().await.unwrap();
    let names: Vec<&str> = all.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(names, ["muse", "grok", "claude"]);
    assert_eq!(all[2], c);
}

#[tokio::test]
async fn blank_tags_and_names_are_refused() {
    let (_db, s) = store().await;
    let err = s
        .put_profile(&profile("grok", &["image", " "], &[], 2))
        .await
        .unwrap_err();
    assert!(matches!(err, AvError::Invalid(_)), "{err:?}");
    let err = s.put_profile(&profile("", &[], &[], 2)).await.unwrap_err();
    assert!(matches!(err, AvError::Invalid(_)), "{err:?}");
    let err = s
        .post_backlog(new_item("demo", "t", &[""]))
        .await
        .unwrap_err();
    assert!(matches!(err, AvError::Invalid(_)), "{err:?}");
    assert!(s.list_profiles().await.unwrap().is_empty());
}

#[tokio::test]
async fn tags_are_stored_lowercase_and_trimmed() {
    let (_db, s) = store().await;
    s.put_profile(&profile("grok", &[" Image "], &["UI"], 2))
        .await
        .unwrap();
    let e = s
        .post_backlog(new_item("demo", "t", &["IMAGE"]))
        .await
        .unwrap();
    let p = &s.list_profiles().await.unwrap()[0];
    assert_eq!(
        (p.best.clone(), p.can.clone()),
        (vec!["image".into()], vec!["ui".into()])
    );
    assert_eq!(e.item.needs, ["image"]);
}

#[tokio::test]
async fn the_backlog_lists_oldest_first_and_filters_by_repo_and_state() {
    let (_db, s) = store().await;
    let a = s
        .post_backlog(new_item("demo", "first", &[]))
        .await
        .unwrap();
    let b = s
        .post_backlog(new_item("other", "second", &["image"]))
        .await
        .unwrap();
    let c = s
        .post_backlog(new_item("demo", "third", &[]))
        .await
        .unwrap();
    assert_eq!(a.state, BacklogState::Queued);
    assert_eq!(b.item.needs, ["image"]);
    assert_eq!(b.posted_by, "me");

    let ids = |v: Vec<BacklogEntry>| v.into_iter().map(|e| e.item.id).collect::<Vec<_>>();
    assert_eq!(
        ids(s.list_backlog(None, None).await.unwrap()),
        [a.item.id, b.item.id, c.item.id]
    );
    assert_eq!(
        ids(s.list_backlog(Some("demo"), None).await.unwrap()),
        [a.item.id, c.item.id]
    );
    assert!(s.take_backlog(a.item.id, "muse").await.unwrap());
    assert_eq!(
        ids(s
            .list_backlog(None, Some(BacklogState::Queued))
            .await
            .unwrap()),
        [b.item.id, c.item.id]
    );
}

#[tokio::test]
async fn concurrent_takes_of_one_item_have_one_winner() {
    let (_db, s) = store().await;
    let item = s.post_backlog(new_item("demo", "t", &[])).await.unwrap();
    let agents: Vec<String> = (0..8).map(|n| format!("agent-{n}")).collect();
    let results =
        futures::future::join_all(agents.iter().map(|a| s.take_backlog(item.item.id, a))).await;
    let won: Vec<usize> = results
        .iter()
        .enumerate()
        .filter(|(_, r)| *r.as_ref().unwrap())
        .map(|(i, _)| i)
        .collect();
    assert_eq!(won.len(), 1, "{results:?}");
    let e = &s.list_backlog(None, None).await.unwrap()[0];
    assert_eq!(e.state, BacklogState::Taken);
    assert_eq!(e.taken_by.as_deref(), Some(agents[won[0]].as_str()));
    assert!(e.taken_at.is_some());
}

#[tokio::test]
async fn an_untaken_item_is_queued_again_and_a_linked_one_names_its_task() {
    let (_db, s) = store().await;
    let item = s.post_backlog(new_item("demo", "t", &[])).await.unwrap();
    assert!(s.take_backlog(item.item.id, "muse").await.unwrap());
    assert!(!s.take_backlog(item.item.id, "codex").await.unwrap());
    s.untake_backlog(item.item.id).await.unwrap();
    let e = &s.list_backlog(None, None).await.unwrap()[0];
    assert_eq!(
        (e.state, e.taken_by.as_deref()),
        (BacklogState::Queued, None)
    );

    assert!(s.take_backlog(item.item.id, "codex").await.unwrap());
    let task = s.create_task(new_task()).await.unwrap();
    s.link_backlog_task(item.item.id, task.id).await.unwrap();
    let e = &s.list_backlog(None, None).await.unwrap()[0];
    assert_eq!(e.task_id, Some(task.id));
    // a taken item cannot be untaken once it has a task
    assert!(s.untake_backlog(item.item.id).await.is_err());
}

#[tokio::test]
async fn only_a_queued_item_can_be_dropped() {
    let (_db, s) = store().await;
    let a = s.post_backlog(new_item("demo", "a", &[])).await.unwrap();
    let b = s.post_backlog(new_item("demo", "b", &[])).await.unwrap();
    s.drop_backlog(a.item.id).await.unwrap();
    assert!(!s.take_backlog(a.item.id, "muse").await.unwrap());
    assert!(matches!(
        s.drop_backlog(a.item.id).await.unwrap_err(),
        AvError::IllegalState(_)
    ));
    assert!(s.take_backlog(b.item.id, "muse").await.unwrap());
    assert!(matches!(
        s.drop_backlog(b.item.id).await.unwrap_err(),
        AvError::IllegalState(_)
    ));
    assert!(matches!(
        s.drop_backlog(uuid::Uuid::new_v4()).await.unwrap_err(),
        AvError::NotFound(_)
    ));
}

#[tokio::test]
async fn a_handoff_can_carry_tags_until_it_is_claimed() {
    let (_db, s) = store().await;
    let t = s.create_task(new_task()).await.unwrap();
    // only a handed-off task can carry them
    let err = s
        .set_handoff_needs(t.id, &["image".to_string()], None)
        .await
        .unwrap_err();
    assert!(matches!(err, AvError::IllegalState(_)), "{err:?}");

    s.transition(t.id, lifecycle::Event::Handoff).await.unwrap();
    s.set_handoff_needs(t.id, &["Image".to_string()], Some("grok"))
        .await
        .unwrap();
    let h = s.handed_off_with_needs(Some("demo")).await.unwrap();
    assert_eq!(h.len(), 1);
    assert_eq!(h[0].task.id, t.id);
    assert_eq!(h[0].needs, ["image"]);
    assert_eq!(h[0].prefer.as_deref(), Some("grok"));
    assert!(
        s.handed_off_with_needs(Some("other"))
            .await
            .unwrap()
            .is_empty()
    );

    s.claim_task(t.id, "grok", 600).await.unwrap();
    assert!(s.handed_off_with_needs(None).await.unwrap().is_empty());
}

#[tokio::test]
async fn a_handoff_without_tags_is_not_listed() {
    let (_db, s) = store().await;
    let t = s.create_task(new_task()).await.unwrap();
    s.transition(t.id, lifecycle::Event::Handoff).await.unwrap();
    assert!(s.handed_off_with_needs(None).await.unwrap().is_empty());
}
