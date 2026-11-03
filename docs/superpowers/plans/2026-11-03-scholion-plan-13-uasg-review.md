# Plan 13: what to take from the "Unified Agent State Graph" proposal

**Why:** an outside proposal (ChatGPT, "Agent-native VCS: data model và algorithms", UASG) designs
a system with the same goal as Scholion from scratch: its own Merkle DAG, blob/chunk store
(BLAKE3, R2, Redis), a typed delta protocol, ten knowledge record types, structural and semantic
compaction, and an incremental context assembler. Its stated primary objective is minimal storage
and compute. This plan records which parts Scholion already has, which it rejects and why, and the
three small changes worth making. The code wins where this text and the code disagree.

## What the measurements already say

The proposal optimises storage of code and knowledge objects. Scholion measured where the bytes
and seconds go, and neither was there:

| Measurement | Result | Source |
|---|---|---|
| jj store + Postgres + Qdrant per task | ≈ 2–3 MB | Plan 12, dogfood run |
| cargo `target/` per task | ≈ 20 GB → 4.3 GB (lean profile) | Plan 12 |
| index copy at task start (tokio, 100 agents) | 13.5 s start, 17/100 merged, ~11 GB DB → 96 ms, 100/100, 183 MB | `bench/results/2026-10-30-e2-overlay` |
| relay, `av` vs `NOTES.md` | −1.8% / −2.5% cost per accepted task, same acceptance (30/30) | `bench/results/2026-10-29-relay-lean`, `2026-10-31-relay-image` |

So: git/jj already content-address code (the proposal's priority 1 is free), the two real storage
costs (index copies, build output) are fixed by Plans 7 and 12, and the open question is the one
the proposal's own MVP names last, "token cost per successful task falls without lowering the
success rate", against a strong baseline. That is Plan 6, not a new engine.

## Already in Scholion under another name

| UASG | Scholion |
|---|---|
| `knowledge_records` + typed facts | `records` (intent, decision, attempt, note, handoff, review, validation) |
| `SUPERSEDES` edge, validity hard filter | `supersedes` / `superseded_by`; superseded records never enter a pack |
| evidence + temporal validity | anchors with `content_hash` (migration 0005); packs flag `anchor_stale` / `changed` |
| knowledge reconstructed at a commit | partial: `GET /v1/repos/{name}/why` (line → commit → task → records) answers for a line on main now, not for an arbitrary past commit; "records of tasks merged before X" is one query, no Merkle root needed |
| task state machine, attempt identity | `lifecycle.rs`; claim generations + `flow::fence` (`X-Av-Generation`) |
| CAS on branch head | `main` advances only via `merge` under the merge lock |
| outbox, eventual indexes | `outbox` rows in the write's transaction; `worker.rs` drains into Qdrant |
| selective, rebuildable embeddings | only records, conventions, code chunks; `av reindex` rebuilds Qdrant |
| commit-keyed cache that never invalidates | `derived` (migration 0018) |
| Incremental Context Assembly, context package | `scholion-pack`: seeds → candidates → RRF → budget split → tiers; receipts (`rcp:`) |
| reuse of context across turns | session ledger: later packs reference instead of resend |
| mandatory constraints | `required` conventions with `check_command` |
| raw tool logs kept briefly | never stored: validations are attestations, the daemon runs no commands |
| reachability GC, temporary workspaces | Plan 11 (`av gc`, retention sweep), Plan 12 (`target/` removed at merge) |

## Rejected

- **Own Merkle DAG, blob/chunk store, BLAKE3, R2, Redis.** jj-lib is the only VCS layer
  (`scholion-vcs`), already content-addressed; a second one doubles the storage it means to save.
  The measured per-task store cost is 2–3 MB.
- **Delta envelope with `FILE_*` / `FACT_*` op namespaces, `knowledge_root_hash` per commit.** Code
  deltas are jj commits; knowledge changes are append-only rows. A knowledge Merkle root buys
  nothing a `created_at <= merge time` query does not.
- **Structural compaction snapshots.** Postgres rows and the jj op log are the materialised state;
  nothing replays a delta chain.
- **Ten record types as integer enums, entity keys like `database.engine`.** Agents do not keep a
  taxonomy or a key space consistent, and the field data (203 pack exposures, 1 expand) says they
  barely use what exists. Anchors (path, symbol) are the key code knowledge already has.
- **Pack-result cache keyed by (commit, knowledge root, task, policy, budget).** A pack takes
  35–133 ms (E2 overlay); the token side is already the session ledger.
- **`tenant_id`, CRDTs, LLM merge.** Tenancy belongs to the commercial edition through `ext.rs`;
  one daemon owns state, so there is nothing to converge; merges stay deterministic.

## Task 1: finish Plan 6 first (E3, E5, E6)

The proposal's success criteria are Plan 6's claims C1–C3. E3 has Haiku pilots on the 25-file
`bench/tasks-tallyho` (`bench/results/2026-10-27-haiku`, `2026-10-28-haiku-hook`): after the
fixes, av matches git on cost and tokens (every interval contains zero) and saves 9–12% of turns.
What is still open is a large repo, a git-worktree baseline for many agents, and review. Run, in
order: E3 on `bench/tasks-scholion` (20k lines of Rust, where a pack can save navigation), E5
multi-agent with a git-worktree baseline, E6 planted review defects. Tasks 2–4 are small and each
names the eval that would show it matters. Task 2 is a correctness fix and needs no eval to
start; Task 3 starts once E5 has a pre-registration, and Task 4 waits for E5's result.

- [ ] E3 on `bench/tasks-scholion`: `PREREG.md` and run (`bench/README.md`)
- [ ] E5 `PREREG.md` and run
- [ ] E6 `PREREG.md` and run

## Task 2: idempotent record writes

`POST /v1/tasks/{id}/records` (`api.rs::add_record`) has no dedupe: an MCP or hook client that
times out and retries writes the record twice, and both show in packs. The proposal's §4.3 is right
here.

- [ ] Failing test in `crates/scholion-daemon/tests/api.rs`: two posts with the same
  `Idempotency-Key` header return the same record id and leave one row.
- [ ] Migration `0029_record_idempotency.sql` (check `main`'s highest number first): nullable
  `idempotency_key text`, unique on `(task_id, idempotency_key)` where not null; a line for it in
  `tests/migrations_frozen.rs`.
- [ ] `add_record_with` returns the existing row on a repeated key, and skips the outbox enqueue
  in that case (the enqueue shares the insert's transaction), so Qdrant never embeds the record
  twice. A key reused with a different summary or kind is 422 (`IllegalState`), not an overwrite.
- [ ] CLI and MCP `record` send a key derived from (session, task, local sequence).
- [ ] Shows in: E2 (deterministic), and the swarm test with an injected retry.

## Task 3: flag overlapping decisions from other tasks in the review bundle

The review bundle (`packer.rs` passes `task_records(task.id)` to `bundle.rs::record_items`)
shows only the reviewed task's own records. When the task's decision is anchored to a symbol or
file that another task's live (unsuperseded) decision also anchors, and the other task is merged or
in review, the reviewer is not told. Shared anchors find overlap, not contradiction: the reviewer
judges whether the two disagree. The proposal's §5.6 policy fits: preserve both, flag, pick
neither. The owner's side exists already: the `PreToolUse` hook shows other tasks' records about a
file (`GET /tasks/{id}/records/at`), once per session. The gap is at review time.

- [ ] Failing test: task A merges a decision anchored to `greet`; task B, in review, records a
  decision on `greet`; B's review bundle carries a required item naming A's decision and its task.
- [ ] Deterministic selection only (shared anchors, not text similarity); no LLM; never blocks
  approve or merge. The reviewer resolves it, and either side supersedes with a new record.
- [ ] Shows in: E5 (agents on overlapping code) and E6 (add one planted contradicting decision).

## Task 4: a derived evidence basis on records

The proposal separates LLM assertion, file-backed, test-verified and human-approved knowledge.
Agents will not label this reliably, so compute it instead of asking for it: `asserted` (task
open), `reviewed` (its task was approved and merged), `checked` (merged with required checks
passing at the merged head), and `changed` already exists when the anchored code moved since. Show
it in the record's handle text and use it as a score multiplier in `score.rs`.

- [ ] Failing golden test in `scholion-pack/tests/golden/` for the three labels.
- [ ] No migration: derived from task status, review verdicts and validation records at merge.
- [ ] Only after E5: if E5 shows no record misled an agent, drop this task.

## How to tell it worked

- Task 1: three result directories under `bench/results/` with `PREREG.md` written before the run.
- Task 2: a retried record post leaves one row; swarm runs show no duplicate summaries per task.
- Task 3: in E6, the planted overlapping decision is named in the reviewer's bundle in every session.
- Task 4: if run, the relay or E5 report shows no worse acceptance and the labels in its packs.
