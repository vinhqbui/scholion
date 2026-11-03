//! Plan 11: where a task's build output goes. A cargo `target/` of this workspace is 8 GB with
//! line tables and over 20 GB with full debug info, and a task used to keep it in its workspace
//! until the retention period had passed. Now every task workspace of a repo whose root has a
//! `Cargo.toml` gets its own `.cargo/config.toml` (`scholion_core::build_dir`) that sends cargo's
//! output to the task's scratch dir, `<ws_root>/<repo>/.scratch/<slug>/cargo-target`, which the
//! merge or abandon removes at once (`disk::reclaim_task`). `scholion-vcs` leaves the file out of
//! snapshots while it is av's.
//!
//! One target dir per task, not one per repo: cargo names a workspace crate's artifacts by its
//! path relative to the workspace root, so two checkouts of one crate collide in a shared target
//! dir and cargo runs the test binary one task built for the other. The per-repo file an earlier
//! av wrote at `<ws_root>/<repo>/.cargo/config.toml` is removed at start-up, with the cache it
//! pointed at.
//!
//! Cargo reads `.cargo/config.toml` in the working directory and every parent, so the file covers
//! builds started anywhere in the workspace; a relative `target-dir` resolves against the parent of
//! `.cargo`, the workspace root, on whatever machine runs cargo, so no host path is needed.
//! `CARGO_TARGET_DIR` in the environment still wins, and the repo's own `.cargo/config.toml`,
//! when it has one, is never touched.

use std::path::Path;

use scholion_core::build_dir::{self, LEGACY_MARKER};
use scholion_core::{AvError, Result};

use crate::AppState;
use crate::disk::under_ws_root;

/// Writes the task's cargo config into the workspace at `ws_dir` when the build cache is on and
/// the workspace has a root `Cargo.toml`, unless a file that is not av's is already there (the
/// repo's own). Returns whether it wrote.
pub(crate) fn ensure_task_config(st: &AppState, ws_dir: &Path, slug: &str) -> Result<bool> {
    if !st.build_cache || !ws_dir.join("Cargo.toml").is_file() {
        return Ok(false);
    }
    let path = ws_dir.join(build_dir::CARGO_CONFIG);
    let want = build_dir::cargo_config(slug);
    match std::fs::read_to_string(&path).ok() {
        Some(current) if current == want || !build_dir::is_av(&current) => return Ok(false),
        _ => {}
    }
    let write = || -> std::io::Result<()> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        std::fs::write(&path, want)
    };
    write().map_err(|e| AvError::Vcs(format!("writing {}: {e}", path.display())))?;
    Ok(true)
}

/// `ensure_task_config`, with a failure logged instead of returned (the task goes on; cargo then
/// builds into the workspace's `target/`, reclaimed with it).
pub(crate) fn ensure_task_config_logged(st: &AppState, ws_dir: &Path, slug: &str) {
    match ensure_task_config(st, ws_dir, slug) {
        Ok(true) => tracing::debug!(%slug, "wrote the task's .cargo/config.toml"),
        Ok(false) => {}
        Err(e) => {
            tracing::warn!(%slug, error = %e, "could not write the task's .cargo/config.toml")
        }
    }
}

/// Removes the per-repo `<ws_root>/<repo>/.cargo/config.toml` an earlier av wrote (recognised by
/// its first line) and the `<ws_root>/<repo>/.cache` it pointed at. A file that is not av's, and
/// its cache, stay. Returns the bytes freed, `None` when there was nothing of av's.
pub(crate) fn remove_legacy(st: &AppState, repo: &str) -> Result<Option<u64>> {
    let config = under_ws_root(st, &[repo, ".cargo", "config.toml"])?;
    let ours = std::fs::read_to_string(&config)
        .is_ok_and(|text| text.lines().next() == Some(LEGACY_MARKER));
    if !ours {
        return Ok(None);
    }
    let io = |what: &str, e: std::io::Error| AvError::Vcs(format!("removing {what}: {e}"));
    std::fs::remove_file(&config).map_err(|e| io(&config.display().to_string(), e))?;
    // the directory, when the file was all there was in it
    let _ = std::fs::remove_dir(config.parent().unwrap_or(&config));
    let cache = under_ws_root(st, &[repo, ".cache"])?;
    let bytes = crate::disk::bytes_of(&cache);
    match std::fs::remove_dir_all(&cache) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(io(&cache.display().to_string(), e)),
    }
    Ok(Some(bytes))
}

/// `remove_legacy`, logged (imports and start-up go on).
pub(crate) fn remove_legacy_logged(st: &AppState, repo: &str) {
    match remove_legacy(st, repo) {
        Ok(Some(bytes)) => tracing::info!(
            %repo,
            freed_bytes = bytes,
            "removed the shared build cache an older av wrote; tasks now build into their scratch dir"
        ),
        Ok(None) => {}
        Err(e) => tracing::warn!(%repo, error = %e, "could not remove the old shared build cache"),
    }
}
