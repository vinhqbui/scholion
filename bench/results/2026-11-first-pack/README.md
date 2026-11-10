# Does a task's own lesson reach its agent? (E4 at scale, follow-up)

2026-11-10. No agent ran and nothing was spent. This is the diagnosis that
`2026-11-e4-scale/README.md` left open ("The lesson is often not in av's first pack").

## Question

E4 at scale reported the task's own lesson in `pack.md` in 12/30 sessions at N = 0, 12/30 at
N = 50 and 6/30 at N = 500 noise notes. Is that what the agents were given, and if not, what were
they given?

## Method

`scholion-bench first-packs` sets each of the 10 relay tasks up exactly as `run --arms av` does
(base tree, relay commits, the pool and the seeded noise as an earlier task's records), waits for
the repo's outbox to drain, and then asks the daemon for what the two hooks print:

- the session-start pack (`GET /v1/tasks/{id}/context`, budget 800, `code=signatures`, with map);
- the pre-read answer for each of the task's `relevant_files`, in order, under the same session.

The lesson is looked up by record id in the pack's receipt, so a shortened summary still counts.
A pack is deterministic for a given state, so one pass per level stands for the 3 reps of E4.

- A record is **shown in full** at the `summary` tier, the `full` tier, or in a pre-read answer:
  all three carry its whole summary (200 characters at most).
- At the **`handle`** tier the summary is cut to 70 characters.
- **Listed** means it is only in the pack's list of omitted items (also cut to 70 characters).

Setup: daemon, `scholion-bench` and `av` built from this branch at `19879a8` (the packer is
`main`'s, `d4f43c5`), Postgres 16, Qdrant v1.13.6, TEI cpu-1.7.2 with `BAAI/bge-small-en-v1.5`,
noise seed 1: the same stack and embedder as the sweep.

```bash
COMMON="--av-url http://127.0.0.1:7450 --work-root /root/bench-work --import-dir /root/bench-work/src --import-mount /root/bench-work/src"
for N in 0 50 500; do
  scholion-bench first-packs bench/tasks-relay --noise $N --out bench/results/2026-11-first-pack/before/n$N $COMMON
done
python3 bench/results/2026-11-first-pack/semrank.py bench/results/2026-11-first-pack/before/n500/first-packs.jsonl \n  bench/tasks-relay <run id> > bench/results/2026-11-first-pack/before/n500/semantic-rank.txt
```

## Result

Tasks out of 10 whose own lesson is in each place:

| noise notes | pack at task start: shown | at task start: shown or listed | hook pack: summary | hook pack: handle | hook pack: listed only | pre-read | shown in full by either hook | shown at all |
|---|---|---|---|---|---|---|---|---|
| 0 | 4 | 4 | 2 | 6 | 1 | 7 | 9 | 10 |
| 50 | 2 | 4 | 3 | 6 | 1 | 6 | 9 | 10 |
| 500 | 1 | 2 | 1 | 5 | 2 | 0 | 1 | 6 |

Per task: `before/n*/first-packs.md`. The packs themselves: `before/n*/packs/`.

## Findings

1. **The sweep's count measured a pack no agent read.** "Shown or listed at task start" is 4, 4
   and 2 of 10, which is the sweep's 12/30, 12/30 and 6/30 exactly. That pack is the answer to
   `POST /v1/tasks`. The harness asks for it right after it posts the records and before they are
   embedded (`start_av_task`, then `wait_for_index`), so semantic search cannot find them yet.
   The agent reads the session-start hook's pack, which is built after the index is ready.
2. **Without noise, delivery is good.** At N = 0 and N = 50 the lesson is shown in full to 9 of
   10 tasks and in some form to all 10. The tenth, `relay-quote-split`, gets a 70-character
   handle that stops before the part that matters ("`Money.allocate(n)` splits exactly").
3. **At N = 500 it breaks: 1 of 10 in full, 4 of 10 not at all.** Two separate causes:
   - **The pre-read hook ignores the task.** `records_at` orders a file's records by kind, then
     newest first, and stops at 400 tokens (7 or 8 records). With 500 notes spread over the repo's
     files, the lesson was in none of the 14 answers. At N = 0 the same hook delivered 7 lessons.
   - **In the pack, being anchored on a named symbol outranks being about the task.**
     `semantic-rank.txt`: by embedding similarity to the goal, the lesson is the best-matching
     record of 512 for 6 tasks (rank 2, behind the task's own intent), and rank 5, 6,
     14 and 17 for the rest. But a record anchored on a symbol the goal names gets
     `SEED_BONUS` 1.0, and similarity is worth at most 0.30. So noise such as "New optional
     parameters of `ValidationError` are keyword-only" takes the summary slots and the lesson gets
     a handle (`relay-customer-region`) or is cut (`relay-quote-split`, `relay-store-count`).
   - The two lessons at rank 14 and 17 (`relay-refund-event`, `relay-order-ids`) are outside the
     8 record hits a pack of 800 tokens asks for, so they are not candidates at all.
4. **The History section shows 4 records** in most 800-token packs, whatever the number of
   candidates (up to 203 omitted).

## What this does not show

- Whether showing the lesson changes outcomes: every `notes` and `av` session of the sweep was
  accepted at every level, lesson or not.
- Anything about other repositories, embedders or real records. The noise is 15 templates over
  one repo, and every noise note names its symbol, which the embedder rewards.
- What the agents of the sweep were given by the pre-read hook: that output was not kept, and the
  run's volumes are gone. This replays the same inputs on the same stack, with new record ids and
  timestamps.
