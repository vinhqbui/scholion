//! `run --agent-user NAME`: the agent CLI runs as a user that can read only its own working
//! copy, so a session cannot read the grading tests, the bench tree or an earlier session.

use std::process::Command;

use crate::Result;

/// The user the agent CLI runs as.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentUser {
    pub name: String,
    pub uid: u32,
    pub gid: u32,
}

/// `id FLAG [NAME]` as a number.
fn id(flag: &str, name: Option<&str>) -> Result<u32> {
    let out = Command::new("id")
        .arg(flag)
        .args(name)
        .output()
        .map_err(|e| format!("id: {e}"))?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_owned());
    }
    String::from_utf8_lossy(&out.stdout)
        .trim()
        .parse()
        .map_err(|e| format!("id {flag}: {e}"))
}

impl AgentUser {
    /// Looks the user up by name (`id -u`, `id -g`). Root is refused: it reads everything.
    pub fn resolve(name: &str) -> Result<AgentUser> {
        let unknown = |e: String| format!("--agent-user {name}: no such user ({e})");
        let uid = id("-u", Some(name)).map_err(unknown)?;
        let gid = id("-g", Some(name)).map_err(unknown)?;
        if uid == 0 {
            return Err(format!(
                "--agent-user {name}: that user is root, which can read everything"
            ));
        }
        Ok(AgentUser {
            name: name.to_owned(),
            uid,
            gid,
        })
    }
}

/// The user this process runs as.
pub fn harness_uid() -> Result<u32> {
    id("-u", None)
}
