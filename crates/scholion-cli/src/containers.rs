//! Containers are footprint too. Everything a session starts through docker carries the labels
//! `av.task=<task id>` and `av.agent=<agent>` (the session-start hook exports them as
//! `AV_DOCKER_LABELS`); the Stop hook counts what carries the task's label and `av gc
//! --containers` removes what finished tasks left. The daemon never runs docker: all of it is
//! here, in the CLI, and only when `docker` is on `PATH`.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// The label naming the task a container, image or volume belongs to.
pub const TASK_LABEL: &str = "av.task";
/// The label naming the agent that started it.
pub const AGENT_LABEL: &str = "av.agent";

/// The flags for `docker run`/`docker build`: `--label av.task=<task> --label av.agent=<agent>`.
pub fn labels(task: &str, agent: &str) -> String {
    format!("--label {TASK_LABEL}={task} --label {AGENT_LABEL}={agent}")
}

/// The `docker` executable on `PATH`, if any. Looked up, not run: a hook must cost nothing when
/// there is none.
pub fn docker() -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    let names: &[&str] = if cfg!(windows) {
        &["docker.exe", "docker"]
    } else {
        &["docker"]
    };
    std::env::split_paths(&path)
        .filter(|dir| !dir.as_os_str().is_empty())
        .flat_map(|dir| names.iter().map(move |n| dir.join(n)))
        .find(|p| p.is_file())
}

/// Runs `docker args…` and returns its stdout, or `None` when it fails, exits non-zero or has not
/// ended by `deadline` (then it is killed: a hung docker must not eat a hook's time budget).
pub fn run(docker: &Path, args: &[&str], deadline: Instant) -> Option<String> {
    let mut child = Command::new(docker)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .ok()?;
    let mut stdout = child.stdout.take()?;
    let reader = std::thread::spawn(move || {
        let mut text = String::new();
        let _ = std::io::Read::read_to_string(&mut stdout, &mut text);
        text
    });
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let text = reader.join().ok()?;
                return status.success().then_some(text);
            }
            Ok(None) if Instant::now() >= deadline => {
                let _ = child.kill();
                let _ = child.wait();
                return None;
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(20)),
            Err(_) => return None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_name_the_task_and_the_agent() {
        assert_eq!(
            labels("0b2c", "grok"),
            "--label av.task=0b2c --label av.agent=grok"
        );
    }
}
