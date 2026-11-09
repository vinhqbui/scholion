# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.
It is loaded into every session, so it stays short: each module's detail is in
`docs/architecture.md` (read the section for the module you touch), the user-facing reference in
`docs/reference.md`. When a doc and the code disagree, the code wins.

## What this is

Scholion: context-first version control for coding agents. Each task is an isolated jj workspace;
decisions, attempts (dead ends) and notes are records anchored to code; a deterministic,
token-budgeted context pack hands any agent what it needs. CLI `scholion`, alias `av` (same
program); env vars use the `AV_` prefix. The spec and older plans use the earlier name `agentvcs`
and crates `av-*`. Design: `docs/superpowers/specs/2026-10-18-agentvcs-design.md`; plans in
`docs/superpowers/plans/` (their checkboxes and signatures lag the code). Current release:
`release/0.0.3` (`CHANGELOG.md`; add an `## Unreleased` entry for user-visible changes).

## Commands

Rust is pinned to 1.97.1 (`rust-toolchain.toml`); do not change it in an unrelated change.

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace                       # all green is the bar before any commit
cargo test -p scholion-daemon --test it api::submit_  # one crate's test binary, module / name filter
cargo test -p scholion-semantic --test it qdrant::NAME -- --exact --nocapture
cargo build --release -p scholion-cli           # -> target/release/scholion and av
scripts/smoke.sh                             # end-to-end against the Docker stack; --down also deletes volumes
docker compose -f deploy/docker-compose.yml up -d --build   # needs `export UID GID=$(id -g)` first
```

Test environment:
- Postgres tests use `scholion_testutil::TestDb`: `AV_TEST_DATABASE_URL` if set, else an embedded
  user-space Postgres (downloaded on first run). No Docker needed.
- Qdrant tests silently pass in ~0 s unless `AV_TEST_QDRANT_URL` (gRPC, `http://127.0.0.1:6334`)
  is set: a green run does not prove Qdrant works.
- `warm_build_under_300ms` is `#[ignore]`d (timing). Nightly-only workflows: `nightly-scale.yml`,
  `e4-smoke.yml` (needs `LLM_API_KEY`).
- Each crate's `tests/*.rs` are modules of one test binary `it` (`tests/main.rs`,
  `autotests = false`): **add a `mod` line for a new test file.**

## Module map

Cargo workspace `crates/*`, edition 2024. A daemon (axum, `:7420`) owns all state; the CLI is a
thin HTTP client.

| Crate | Role | Must know |
|---|---|---|
| `scholion-core` | types, `AvError`, traits (`Vcs`, `IntentStore`, `Semantic`), pure lifecycle and routing rules | illegal transition = `IllegalState` → 422; malformed input = `Invalid` → 400 |
| `scholion-vcs` | **the only crate that touches jj-lib** (`=0.46.0`); one jj workspace per task; `patch.rs` renders/applies git patches | call `rebase_descendants()` after any rewrite; `ChangeId` is the jj *commit* id, so every snapshot/rebase makes a new one |
| `scholion-store` | Postgres `IntentStore` (sqlx), migrations `migrations/000N_*.sql` | check `main`'s highest number before adding one; a merged migration is never edited (checksums; add a line to `tests/migrations_frozen.rs`) |
| `scholion-index` | tree-sitter symbols, refs, skeletons, chunks in Postgres | refs link by bare name to every same-named symbol |
| `scholion-semantic` | embedders and Qdrant (derived data, `av reindex`) | `Embedder::dimension` is async and fallible |
| `scholion-pack` | deterministic context packer, no LLM | golden output in `tests/golden/` |
| `scholion-daemon` | `wiring.rs` (config), `api.rs` (routes under `/v1`), `flow.rs` (lifecycle ops), `routing_api.rs` (`next`, backlog), `disk.rs`, `mcp.rs`, `ext.rs`, … | see below |
| `scholion-agentrun` | runs agent CLIs (`AgentCommand`), host-wide slots | knows no tasks or daemons |
| `scholion-bench` | benchmark harness; only `run --confirm` spends money | `bench/README.md` |
| `scholion-cli` | clap subcommands mirroring the API, hooks, `av up`/`init`/`watch`/`remote` | finds its task via the nearest `.av-task` |
| `scholion-testutil` | `TestDb` only | |

Daemon rules:
- Workspace operations on one task take `AppState::lock_task` first, then `flow::fence` (claim
  generation, header `X-Av-Generation`).
- Lifecycle op tests go in `tests/api.rs`, concurrency in `tests/swarm.rs`; helpers in
  `tests/common/mod.rs`.
- `ext.rs` is the extension surface the Commercial edition (agentvcs) builds on: keep it stable;
  a change there is a breaking change downstream.
- Deletes on disk go only through `under_ws_root`.
- The MCP tool list is size-guarded in `tests/mcp.rs`.

`integrations/` holds agent glue (Claude Code hooks, a generic wrapper); see
`docs/architecture.md`.

## Invariants

- Record summaries ≤ 200 chars; records are append-only (newer ones *supersede*).
- `submit`/`handoff` need an intent plus at least one decision/attempt/note/handoff record.
- `main` advances only via `merge`, `revert` (adds a commit), or fast-forward from the backup
  remote.
- The daemon never runs repository commands: validations are attestations reported by clients;
  a remote checkpoint's patch is applied in process (`scholion-vcs::patch`), never `git apply`.
- Workspace paths returned to clients are host paths (`ws_host_root`), not container paths.
- Auth is the optional shared `AV_TOKEN` (every route but `/healthz` and `/ui`); a non-loopback
  `AV_LISTEN` needs it or `AV_LISTEN_OPEN=1`.

## Working conventions

- `main` is the base. One branch and one PR per change, off `main` (not stacked).
- TDD: failing test first, then implementation, then fmt + clippy + tests. See `CONTRIBUTING.md`.
- Agent-facing text (pack output, MCP tool descriptions, hook messages) says `av`, not `scholion`.
