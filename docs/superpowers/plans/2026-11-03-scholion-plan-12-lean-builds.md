# Plan 12: lean builds (storage per successful task)

**Goal:** the disk a Rust task costs is bounded, small and accounted for. The first dogfood run
(`D:\Projects\agent-vcs\dogfood\2026-11-03`, report and `fable-storage-brief.md`) measured where
the bytes went; this plan takes them back, in order of payoff over effort. The north-star metric is
**infrastructure cost per successful task** (merged, check green, review passed), and storage is
its biggest term.

**What the run measured** (5 Rust tasks on scholion, 3–4 agents at once, Docker on Windows):

| Store | Per task | Note |
|---|---|---|
| jj store, Postgres, Qdrant | ≈ 2–3 MB | content-addressed, tiny |
| Workspace checkout | 6 MB | |
| **cargo target dir** | **≈ 20 GB** | 14 GB of 96 test binaries (73% debug info), 3–4 GB `incremental/`, 2.8 GB dependency rlibs, 0.3 GB build scripts |

Build output is 99.99% of the disk. The storage problem is a build-artifact problem, not a
context-storage problem. Two further findings: a target dir **shared by workspaces of one repo is
unsafe** (cargo names a path package's artifacts by its path relative to the workspace root, so
both workspaces produce the same names and `cargo test` in one ran the other's binary; Plan 11's
`.cache/cargo-target` had exactly this flaw), and a docs-only task paid the full build because the
required check's glob matched every path.

Anatomy of the same tree built two ways (scholion at `aedb7e3`, `cargo test --workspace
--no-run`, measured 2026-11-03 in the build container):

| | default profile | lean (`debug = "line-tables-only"`, `incremental = false`) | lean + `package."*".debug = false` (shipped) |
|---|---|---|---|
| total | **21 GB** | **6.7 GB** | **4.3 GB** |
| test/bin executables | 14 GB (65) | 4.9 GB (63) | 3.0 GB (63) |
| `incremental/` | 4.0 GB | 0 | 0 |
| dependency rlib + rmeta | 2.7 GB | 1.4 GB | 1.3 GB |
| build scripts | 270 MB | 240 MB | ≈ 240 MB |

## Task 1: one target dir per task, lean profile — done (this plan's first PR)

- `build_cache.rs` writes, for a repo whose trunk has a root `Cargo.toml`,
  `<ws_root>/<repo>/.cargo/config.toml` with `[build] incremental = false`, `[profile.dev]
  debug = "line-tables-only"`, `[profile.dev.package."*"] debug = false` (dependencies carry no
  debug info; the repo's own crates keep file:line), and **no `target-dir`**. Each workspace builds into its own
  `target/` (cargo's default), which `remove_workspace` deletes with the workspace. A file whose
  first line starts with `# written by av:` is rewritten, so the Plan 11 file goes at start-up.
- `scholion-vcs` adds `/target/` to `AGENT_IGNORES`: a workspace's top-level build output is never
  snapshotted, even in a repo whose `.gitignore` forgot it.
- `GET /v1/disk` reports `build_bytes` per workspace and in totals; `POST /v1/gc` reports it per
  reclaimed workspace; `av disk` has a `BUILD` column; the dashboard's Disk card shows it.
- Tests: `two_workspaces_of_one_repo_never_share_a_target_dir` (runs `cargo metadata` in two
  workspaces of one imported cargo repo), `disk_and_gc_count_a_workspaces_build_output_apart`,
  `build_output_is_never_snapshotted_even_when_the_repo_does_not_ignore_it`.

Expected effect on the run above: ≈ 20 → ≈ 4.5 GB per task (−78%), no false validations, and a
reviewer validating in the owner's workspace still gets a warm build.

## Task 2: checks scoped to what changed

Conventions already carry a `scope_glob`, and `flow::applicable_checks` keeps only the required
conventions whose glob matches a changed path. So "docs-only → no build" needs no code: the
required convention's scope must be `crates/**` (or similar), not `**`. To do:

- [ ] `bench/` and `examples/routing-smoke` set-up scripts and the dogfood brief use a scoped
      glob for the full bar; `docs/reference.md` says so (done in the first PR's text).
- [ ] A change inside one crate: `cargo test -p <crate>` plus its reverse dependencies. The daemon
      has the changed paths at submit and the workspace's crate graph (`Cargo.toml` members); a
      check command may carry `{crates}` which `av validate` expands to `-p a -p b`. Needs a
      design note first: the reverse-dependency closure must come from `cargo metadata`, not
      guesses, or a green check can miss a dependant.

## Task 3: `av disk` and the sweep own build dirs outside `ws_root`

The run's agents built into `CARGO_TARGET_DIR=/target/ws-<slug>` (a Docker volume, for speed on a
bind mount). Nothing of av's knew those dirs; they were deleted by hand.

- [ ] A task may register a build dir (`POST /v1/tasks/{id}/build-dir`, or the session-start hook
      posts `CARGO_TARGET_DIR` when set) so `av disk` counts it under the task and the reclaim
      removes it with the workspace. Deletes stay under a root the daemon was given
      (`AV_BUILD_ROOT`), never arbitrary paths.
- [ ] The footprint already shows a session's `target` dirs outside any workspace; `outside`
      entries that name a task's registered build dir move under the task.

## Task 4: workspace slots with warm targets (the 2× concurrency lever)

N stable workspace directories per repo (N = concurrency), each with its own warm `target/`. A new
task is checked out *into* a free slot (jj rewrites the files and bumps mtimes, so cargo's
freshness is right). Disk becomes N × slot, bounded by concurrency rather than by the number of
tasks, and every build is incremental-warm (the run saw a warm review build in 4 min against
10–15 cold).

- [ ] Design note: the workspace path is today `<ws_root>/<repo>/<slug>` and stored per task; a
      slot means a stable path and a task ↔ slot mapping, plus the invariant that a slot holds one
      open task at a time (`lock_task` is per task, so a slot lock is new).
- [ ] Measure before deciding: with Task 1, is the cold build (and its 6.7 GB) still the long pole?

## Task 5: share dependencies (design, 2026-11-03; measured, not yet built)

### What was measured (build container, main 5bea076, lean config, `cargo test --workspace --no-run`)

| | size | time (16 cores, warm registry) |
|---|---|---|
| full bar, fresh `target/` | 2.4 GB | 38 s on the container fs, 76 s on the `D:\ws` 9p bind mount |
| of which the repo's own crates (libs, bins, test binaries) | 1.1 GB | |
| of which dependencies (`deps/` rlibs, `build/` outputs, their fingerprints) | 1.3 GB | |
| the same bar in a fresh `target/` **seeded with hard links to the dependency artifacts** | 1.2 GB on disk (both trees together 3.6 GB, not 4.8) | 15 s; cargo compiled the 11 own crates and 0 dependencies; `cargo test -p scholion-core` ran its own binaries |

Facts the design rests on, each checked on this build:
- Hard links work on `D:\ws` from the agents' container (9p/drvfs: same inode, link count 2).
- A fingerprint dir `.fingerprint/<pkg>-<hash>` and the artifacts `deps/*-<hash>.*` share the
  hash (176/200 sampled; the other 24 are build-script units, whose output is `build/<pkg>-<hash>/`).
- Six build scripts (`ring`, `psm`, `tree-sitter` and its three grammars) write
  `rustc-link-search=<absolute target>/build/<pkg>-<hash>/out` into `build/<pkg>-<hash>/output`.
  A seeded workspace links against the **template's** path, so the template must keep its path
  and its entries for as long as any workspace links to it (add-only, see below).
- `sccache` saves compile time only: every task's `target/` still holds its own copy of each
  rlib. For disk it is no answer; it is also not installed.
- Cargo 1.97 records registry crates with `CheckDepInfo` like path crates (13 units of 559 are
  `Precalculated`), so "is this artifact shareable" cannot be read from the fingerprint. It comes
  from workspace membership instead: a unit of a package that is not a member of the repo's Cargo
  workspace is shareable. Path packages' fingerprints hash their path **relative to the workspace
  root**, which is why run 1's shared target dir ran another task's test binary and why own crates
  must never enter the template.

### The mechanism

One **template** per repo, `<ws_root>/<repo>/.cache/deps-target/` (Plan 11's `.cache` dir, so
`av disk` already lists it as `build cache` and `av gc --build-cache` already removes it), holding
only dependency artifacts, laid out like a `target/` (`debug/deps`, `debug/.fingerprint`,
`debug/build`). The daemon writes it with file operations only; it still never runs cargo.

- **Seed** (in `prepare_workspace`, after `create_workspace`, when the repo has a root `Cargo.toml`
  and a template exists): hard-link every template entry into the new `<ws>/target/`, directories
  re-created, files linked. If `link` fails (another filesystem, no support) the workspace starts
  empty; never copy, that would cost the 1.3 GB the task is meant to save. `AV_BUILD_CACHE=off`
  skips it.
- **Harvest** (in `reclaim_build`, before `<ws>/target` is removed at merge/abandon): for each unit
  dir `<ws>/target/debug/.fingerprint/<pkg>-<hash>/` that is **(a)** of a package outside the
  workspace's member set and **(b)** a `lib-*`, `build-script-*` or `run-build-script-*` unit (never
  `bin-`, `test-`, `integration-test-`, `example-`, `bench-`: those are executed), link its
  fingerprint dir, its `deps/*-<hash>.*` files and its `build/<pkg>-<hash>/` dir into the template
  **if absent**. Add-only: nothing in the template is replaced or removed except by
  `av gc --build-cache` (the whole dir), which is refused while any workspace of the repo has a
  `target/` (its binaries link against the template's `out` dirs).
- **Member set**: the trunk's root `Cargo.toml` `[workspace].members`/`exclude` globs resolved
  against the trunk tree (the daemon has it in jj), each member's `[package].name`; a root
  `[package]` counts too. Names compare with `-` and `_` folded. Parsed with the `toml` crate.
- **Why hard links are safe here**: cargo never rewrites an artifact it judges fresh, and a
  dependency's unit is fresh in every workspace with the same rustc, profile, features and
  RUSTFLAGS, because all of those are in the hash; a change to any of them gives a new hash and a
  new file beside the old one. The one way a shared inode could be overwritten is a build script
  re-run by `rerun-if-env-changed` in one workspace (e.g. `CC` set there), so `build/` is
  **copied**, not linked, at seed time: it is small (the six `out` dirs are grammars and `ring`'s
  objects) and its `output` files still name the template's `out` paths. `deps/` and
  `.fingerprint/` are linked.
- **Reporting**: the template shows under the repo's `build cache` line in `av disk` (exists); a
  workspace's `BUILD` column stays its `target/` size counted alone, which is what the task would
  cost by itself. The totals line gains `(N shared)` once the sweep sums inodes once (`st_nlink`).

Expected per task for this repo: 1.2 GB while open (from 2.4), a 15 s instead of 38 s full build
on a warm template; the template 1.3 GB per repo, once. Run 1's 20 GB per task becomes 1.2 GB
plus 1.3 GB per repo.

### Steps (TDD, one PR each)

- [ ] `build_cache::members(st, repo) -> Option<BTreeSet<String>>`: parse the trunk's workspace.
      Tests: this repo's eleven `scholion-*` names; a repo with a root `[package]` only; a repo
      without `Cargo.toml` is `None`.
- [ ] `build_cache::harvest(ws_target, template, members) -> Harvested {units, bytes}`: pure file
      operations on a fixture `target/` built by the test (fake `.fingerprint` dirs and `deps`
      files, no cargo). Tests: a `lib-serde` unit is linked; `test-lib-scholion_core`, `bin-av` and
      a member's lib are not; an entry already in the template is left as is; `build/<pkg>-<hash>/`
      is linked whole.
- [ ] `reclaim_build` harvests before it removes; the event `build:reclaimed` gains `shared_bytes`.
      Test in `tests/disk.rs`: after a merge the template holds the fixture's dependency entries
      and the workspace's `target/` is gone.
- [ ] `prepare_workspace` seeds; `AV_BUILD_CACHE=off` skips. Tests: a new workspace of a repo with
      a template has the linked entries with link count 2 (`MetadataExt::nlink`) and a copied
      `build/`; a repo without a template starts empty.
- [ ] `av gc --build-cache` refuses while a workspace of the repo has a `target/`, and says which.
- [ ] Live check in the build container like Task 1's: two tasks, the first builds the bar and
      merges, the second starts after the harvest and builds the bar in ~15 s with 1.2 GB.
- [ ] Measure link creation on the 9p mount (1354 + 556 links) before calling it done there; if
      it is slow, seed only `deps/` (the rlibs) and let the fingerprints be rebuilt.
## Task 6: scholion's own test binaries

63 integration-test binaries, each linking everything. Merging each crate's `tests/*.rs` into one
binary per crate (`tests/main.rs` with `mod` lines) would cut both disk and link time for the
dogfood repo itself. A repo change, not a product change; cheap, but it changes every
`cargo test --test NAME` invocation in `CLAUDE.md` and `CONTRIBUTING.md`.

## How to tell it worked

Run the same five backlog items again on this daemon and compare, per merged task: GB peak in
target dirs (`av disk` now reports it), minutes per validation, validations failed for reasons
outside the code (0 expected from the shared-target class), review rounds. `scholion-bench`'s
plain-git arm gives the baseline without av.

## Live check of Task 1 (2026-11-03, daemon from this branch, build container)

Imported this repo (205 files, 7 s), started two tasks, built in both at once: `cargo test
--workspace --no-run` in one, `cargo test -p scholion-core --no-run` in the other, with the
daemon's config in force and no `CARGO_TARGET_DIR`.

- `cargo metadata` in each workspace resolved `<ws_root>/scholion/<slug>/target`: two different
  dirs, both inside their workspace.
- `av disk` while both were open: `build-the-whole-workspace` 4.6 GB (build 4.5 GB),
  `build-one-crate` 95 MB (build 90 MB); store 5 MB.
- `cargo test -p scholion-core --offline` in the first workspace ran its own warm binaries: 23
  passed.
- `POST /abandon` on both with `AV_WS_RETAIN_SECS=0` answered `workspace: removed`; `av disk`
  afterwards: workspaces 0 B, build output 0 B.

So with this branch a Rust task's disk is its own `target/` (4.5 GB for this repo's full bar,
2.4 GB once #72 lands), it is counted, and it is gone with the task.

## Live check of main after #71–#75 (2026-11-03 15:17 UTC, daemon from 16b1dd7, build container)

The same two-task run, now with every merged default in force (per-task `target/`, lean profile,
one test binary per crate, build output removed at merge) and the default retention (3600 s).

- `cargo metadata` in each workspace: `<ws_root>/scholion/<slug>/target`, two different dirs.
- Both built at once (`cargo test --workspace --no-run` in one, `-p scholion-core` in the other),
  416 crates, 37 s on 16 cores with a warm registry. `av disk` while both were open:
  `full-bar-in-one-task` 2.6 GB (build 2.6 GB), `one-crate-in-another` 95 MB (build 90 MB).
- In the first: an edit, a decision record, submit, `cargo test -p scholion-core --offline` on its
  own binaries (23 passed), approve, merge. The merge answered `status: merged, workspace: kept,
  build: removed`; the feed has `build:reclaimed {slug, bytes: 2884986242}`.
- `av disk` after the merge: the merged workspace 5 MB with no build column, the open one
  unchanged. Abandoning the second answered `workspace: kept, build: removed` too; totals then:
  workspaces 10 MB, build output 0 B, store 6 MB.

So on main a Rust task's build output is 2.6 GB for this repo's full bar (21 GB in run 1), it
shows in `av disk` while the task is open, and it is gone the moment the task is merged or
abandoned; the 5 MB workspace waits for the retention.

## The live stack (2026-11-03, after #71–#73 merged)

- The compose daemon (`deploy/docker-compose.yml`, project `scholion`, `:7420`) was rebuilt from
  `main` (ae19a2b) and recreated with the run's mounts (`AV_WS_MOUNT='D:\ws'`,
  `AV_IMPORT_MOUNT='C:\Users\vinhq\projects'`, `AV_WS_ROOT='D:/ws'` for the host root, the same
  `UID:GID`). At start-up it rewrote `D:\ws\scholion\.cargo\config.toml` from Plan 11's shared
  target dir to the lean per-task config, and the sweep reclaimed run 1's five merged workspaces.
  The old image is kept as `scholion-daemon:pre-plan12` for rollback. In bash, pass the ids
  through `env UID=… GID=… docker compose …`: `UID` is read-only in the shell.
- **Two things the daemon cannot do, both done by hand on 2026-11-03 ~15:40 UTC (`sch-ws` recreated as below; the bar is now `conv:7` with the scope below, `conv:1` removed):**
  1. The agents' container `sch-ws` exports `CARGO_TARGET_DIR=/target`. The environment beats
     every cargo config, so agents there would still share one target dir. Recreate it without
     that variable (builds then land in `/ws/scholion/<slug>/target`, reclaimed with the task), or
     give each agent `CARGO_TARGET_DIR=/target/ws-<slug>` as run 1's lead did (then `av disk`
     does not see those dirs: Task 3):
     ```
     docker rm -f sch-ws
     docker run -d --init --name sch-ws --network sch-test --add-host host.docker.internal:host-gateway \
       -v sch-cargo:/usr/local/cargo/registry -v sch-target:/target -v 'D:\ws:/ws' \
       -e AV_TEST_DATABASE_URL=postgres://postgres:pg@sch-test-pg:5432/postgres \
       -e AV_URL=http://host.docker.internal:7420 -e AV_FOOTPRINT=off -w /ws rust:1.97.1 sleep infinity
     ```
  2. The live repo's required full-bar convention (`conv:1`) is scoped to `**`, so a docs-only
     task runs the whole bar. Replace it with one scoped to the code
     (`{crates/**,Cargo.toml,Cargo.lock,rust-toolchain.toml}`; globset takes braces), and drop the
     old one:
     ```
     av convention add --repo scholion --required \
       --scope '{crates/**,Cargo.toml,Cargo.lock,rust-toolchain.toml}' \
       --check 'cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace' \
       'The bar: fmt, clippy -D warnings and cargo test --workspace all green. Build in the workspace itself (its own target/); never point CARGO_TARGET_DIR at a dir another task uses.'
     av convention rm 1 --repo scholion
     ```
  Builds on the `D:\ws` bind mount (9p) take 2× the container-fs time (76 s vs 38 s for the full
  bar), measured in the build container; tolerable, so Task 3 stays after Task 5.
- The live repo's trunk is run 1's import (before #72), so a task's full bar there costs about
  4.3 GB until the repo is re-imported from current `main`; after that, 2.4 GB.
- Docker's `D:\DockerData\DockerDesktopWSL\disk\docker_data.vhdx` is 175 GB for about 15 GB of
  volumes. It shrinks only with Docker Desktop stopped: `wsl --shutdown`, then in an elevated
  shell `diskpart` → `select vdisk file=…` → `attach vdisk readonly` → `compact vdisk` →
  `detach vdisk`. Needs someone at the machine for the UAC prompt.
