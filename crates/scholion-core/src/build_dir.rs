//! Where a task's build output goes. Every task workspace of a Rust repo gets its own
//! `.cargo/config.toml`, written by the daemon, that sends cargo's output to the task's scratch
//! dir (`<ws_root>/<repo>/.scratch/<slug>/cargo-target`): one target per task, so two tasks'
//! builds never see each other's artifacts (a target dir shared between checkouts of one crate
//! runs one task's test binary for another), and the whole of it goes with the merge or abandon
//! instead of sitting in the workspace for the retention period. `scholion-vcs` leaves the file
//! out of snapshots when it is av's, so it never becomes part of a task's changes.

/// The file, relative to the workspace root.
pub const CARGO_CONFIG: &str = ".cargo/config.toml";

/// The dir under `<ws_root>/<repo>` that holds every task's scratch dir.
pub const SCRATCH: &str = ".scratch";

/// The task's cargo target dir inside its scratch dir.
pub const CARGO_TARGET: &str = "cargo-target";

/// First line of the file av writes; a file starting otherwise is the repo's or the agent's and
/// is left alone (and snapshotted like any other file).
pub const MARKER: &str = "# written by av: this task's cargo output goes to its scratch dir.";

/// First line of the per-repo file an earlier av wrote at `<ws_root>/<repo>/.cargo/config.toml`,
/// pointing every workspace of the repo at one shared `<ws_root>/<repo>/.cache/cargo-target`.
/// Removed at start-up: cargo believes a binary another checkout built is up to date.
pub const LEGACY_MARKER: &str =
    "# written by av: one cargo target for every task workspace of this repo.";

/// The `.cargo/config.toml` for the task workspace `slug`. A relative `target-dir` resolves
/// against the parent of `.cargo` (the workspace root) wherever cargo runs, so no host path is
/// needed. Line tables are kept for file:line in panics and backtraces; full debug info is about
/// 80 % of each test binary.
pub fn cargo_config(slug: &str) -> String {
    let target = format!("../{SCRATCH}/{slug}/{CARGO_TARGET}");
    format!(
        "{MARKER}\n\
         # Removed with the task. Delete this file, or set AV_BUILD_CACHE=off before `av up`, to opt out.\n\
         [build]\n\
         target-dir = \"{}\"\n\
         [profile.dev]\n\
         debug = \"line-tables-only\"\n",
        toml_escape(&target)
    )
}

/// Whether `text` is a file av wrote (either generation).
pub fn is_av(text: &str) -> bool {
    matches!(text.lines().next(), Some(MARKER) | Some(LEGACY_MARKER))
}

fn toml_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_config_sends_output_to_the_tasks_scratch_dir() {
        let text = cargo_config("fix-login");
        assert!(text.starts_with(MARKER), "{text}");
        assert!(
            text.contains("target-dir = \"../.scratch/fix-login/cargo-target\""),
            "{text}"
        );
        assert!(text.contains("debug = \"line-tables-only\""), "{text}");
        assert!(is_av(&text));
    }

    #[test]
    fn both_generations_are_avs_and_a_users_file_is_not() {
        assert!(is_av(&format!("{LEGACY_MARKER}\n[build]\n")));
        assert!(!is_av("[build]\ntarget-dir = \"/elsewhere\"\n"));
        assert!(!is_av(""));
    }
}
