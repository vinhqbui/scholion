# Agent integrations

How each agent CLI reaches the `av` MCP server, and against which release of that CLI the wiring
was last checked. `av init --for <agent>` writes the config (`crates/scholion-cli/src/agents.rs`);
the details, and what was and was not checked, are in each folder's README.

| Agent | `av init` | Config file | MCP entry | Hooks av installs | Instructions file | Verified |
|---|---|---|---|---|---|---|
| Claude Code | `--for claude` | `~/.claude/settings.json` | `claude mcp add` | `SessionStart`, `PreToolUse`, `PostToolUse`, `Stop` (`claude-code/install.sh`) | `CLAUDE.md` | the CLI the benchmark runs (`bench/`) |
| OpenAI Codex CLI | `--for codex` | `$CODEX_HOME/config.toml` | `[mcp_servers.av]` `url`, `http_headers`, `bearer_token_env_var` | none (Codex has hooks; untrusted until reviewed) | `AGENTS.md` | against CLI 0.166.0 on 2026-11-04 ([codex](codex/README.md)) |
| Google Gemini CLI | `--for gemini` | `~/.gemini/settings.json` | `mcpServers.av` `httpUrl`, `headers` | none (Gemini has hooks; not run without a key) | `GEMINI.md` | against CLI 0.66.0 on 2026-11-04, config and MCP connection only, no key ([gemini](gemini/README.md)) |
| xAI Grok Build | `--for grok` | `~/.grok/config.toml` | `[mcp_servers.av]` `url`, `headers` | `SessionStart`, `Stop` (`~/.grok/hooks/av.json`) | `AGENTS.md` | against CLI 1.0.70 on 2026-10-31 ([grok](grok/README.md)) |
| Cursor Agent | `--for cursor` | `~/.cursor/mcp.json` | `mcpServers.av` `url`, `headers` | none | `AGENTS.md` | against `cursor-agent` 2026.10.22 on 2026-10-31 ([cursor](cursor/README.md)) |
| Muse Code | `--for mcp-json --path FILE` | unverified | `mcpServers.av` (key names unverified) | none | unverified | `muse --help` only, 2026-10-31 ([muse](muse/README.md)) |
| Any other CLI | `--for mcp-json --path FILE` | yours | `mcpServers.av` `type`, `url`, `headers` | none | yours | n/a; wrap it with [`generic/av-wrap.sh`](generic/av-wrap.sh) |

Agents without av hooks call `start_task` and the other tools themselves; give them the text in
[`agents-instructions.md`](agents-instructions.md). `generic/av-wrap.sh <agent>` writes the start
pack to a file before the agent starts and checkpoints after it exits.
