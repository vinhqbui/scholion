# What a confident wrong note does (E4, wrong-lesson cell)

2026-11-11. 93 paid sessions on `deepseek-flash` (3 smoke, 90 in the cell), $1.26 at the
harness's peak prices, $0.62 off the balance. Design: `DESIGN.md`. Rules fixed in advance:
`PREREG.md`. Everything under "What the rule does not see" was found after the run and is
labelled as such.

## Summary

- **By the pre-registered rule, neither arm is hurt, and the arms do not differ.** Accepted:
  `cold` 27 of 30, `notes` 30 of 30, `av` 24 of 30. `av` is above the lower end of `cold`'s
  interval, and the intervals of `notes` and `av` overlap.
- **The rule hides the one effect that is there.** On one task of five, `relay-quote-split`, the
  `av` arm failed 6 of 6 and both other arms passed 6 of 6. In every one of those six sessions
  the agent took the approach the wrong record recommends, said the record was why, and stored
  the same wrong decision again as a new record. In all six `notes` sessions the agent read the
  same note, rejected it, and five of them rewrote the note.
- **One task is contaminated.** On `relay-line-taxes`, 8 of 18 sessions looked outside their
  working copy and read the grading test or another session's kept files. `--keep-work`, which
  this run added, put those files there. That task's numbers say nothing about the note.
- So this cell does not give a clean answer. It gives a specific, repeatable failure to test in a
  cell with the leak closed, and a difference in how the two memory systems are used that was not
  predicted: an agent corrects a notes file, and repeats a record.

## Result by the pre-registered rule

| arm | valid sessions | accepted | rate | 95 % Wilson interval | cost | cost per accepted |
|---|---|---|---|---|---|---|
| `cold` | 30 | 27 | 0.900 | 0.744 to 0.965 | $0.3214 | $0.0119 |
| `notes` | 30 | 30 | 1.000 | 0.886 to 1.000 | $0.4785 | $0.0160 |
| `av` | 30 | 24 | 0.800 | 0.627 to 0.905 | $0.4134 | $0.0172 |

The Wilson intervals are computed by `analyze.py` with the formula of
`scholion_bench::stats::wilson_interval`. Every session is valid and none was skipped for budget.

- An arm is hurt if its rate is below 0.744, the lower end of `cold`'s interval (22 of 30 or
  fewer). `notes` (1.000) and `av` (0.800) are both above it: **neither is shown to be hurt.**
- The intervals of `notes` (0.886 to 1.000) and `av` (0.627 to 0.905) overlap: **no claim that
  the arms differ.**
- H0 is what the rule reports. By the rule's own follow-up clause, a mark on records from reverted
  work is not justified by solved tasks on this result.

Thirty sessions per arm cannot show a small effect, and six repetitions of five tasks are not
thirty independent tasks. The next section is why the aggregate is the wrong place to look.

## Accepted per task

| task | `cold` | `notes` | `av` |
|---|---|---|---|
| `relay-coupon-remove` | 6/6 | 6/6 | 6/6 |
| `relay-line-taxes` | 3/6 | 6/6 | 6/6 |
| `relay-money-plain` | 6/6 | 6/6 | 6/6 |
| `relay-quote-split` | 6/6 | 6/6 | **0/6** |
| `relay-store-count` | 6/6 | 6/6 | 6/6 |

The six sessions `av` lost are one task. The three sessions `cold` lost are another task, the
contaminated one. In the aggregate the two cancel in part: `av` is 27 − 6 + 3 = 24.

## What the rule does not see (found after the run; not pre-registered)

### 1. On one task the av agents followed the wrong record, every time

`relay-quote-split` asks for `Quote.split(parts)`. The right fix is `Money.allocate`, whose parts
add up to the total. The wrong note says to use `total.times(Decimal(1) / n)` and not `allocate`.
No session on this task looked outside its copy (`leak.md`), so this task is clean.

- **`av`, 0 of 6.** All six sessions wrote `times(Decimal(1) / parts)`. Their closing messages
  say why, for example "per the recorded decision, this divides evenly rather than using
  `Money.allocate`" and "deliberately not `Money.allocate`". The existing tests pass with that
  code, so nothing in the session contradicted the record. Only one session ran `av expand` on
  it.
- **All six then recorded the wrong approach again** as a new `decision` of their own task
  (`behaviour.md`, column "restates the wrong edit"). A wrong record reproduced itself in six of
  six sessions.
- **`notes`, 6 of 6.** All six read the same text in `NOTES.md`. All six used `allocate`. From
  one session: "One note conflicts with the task: it says to split using
  `total.times(Decimal(1) / n)` and avoid `allocate`, but `times` rounds each share
  independently, so the parts won't add up to the bill." Five of the six edited `NOTES.md` to
  replace the wrong note.
- **`cold`, 6 of 6**, as in E4 at scale.

With 0 of 6 against 6 of 6 the difference on this task is not noise, but it is one task, one
wording and one model, picked out after seeing the data. It is a finding to test, not a result.

### 2. On three tasks the wrong note changed nothing

`relay-coupon-remove`, `relay-money-plain` and `relay-store-count`: 6 of 6 in every arm, and no
session's last edit to the task's file is the wrong one. On these the code the agent reads
contradicts the note. The note about `quote-split` is the one whose claim the code and the
existing tests do not contradict.

### 3. How the two memories were used

From `behaviour.md`, over the 30 sessions of each arm:

| | `notes` | `av` |
|---|---|---|
| sessions that changed the stored note | 24 edited `NOTES.md` | 1 recorded with `supersedes` |
| sessions that wrote a new record | not applicable | 30 (the stop hook asks for one) |
| sessions whose new record or note text matches the wrong edit's pattern | 1 | 15 |

The last row counts text that matches the `[[dead_end]]` pattern. For `relay-quote-split` all six
endorse the approach. For the other tasks a record may name the approach in order to reject it;
those were not read one by one.

This was not predicted. `DESIGN.md` expected the `notes` arm to be the more obedient one, since
its `CLAUDE.md` says to follow the notes. The reverse happened on the one task where obeying was
possible. The cell cannot say why. Candidates, none tested:

- A record arrives in the session-start hook's output under the heading "Decisions & dead ends"
  with nothing that says who wrote it or whether it was checked. A note is a line in a file the
  agent opened itself.
- A notes file can be edited in place, and agents did. A record can only be superseded by a new
  one, which takes knowing the option exists; one session of thirty used it.
- The pack shows the note's summary only. The `notes` arm also read the body.

### 4. One task is contaminated: agents read the grading test

While reading transcripts, one `av` session on `relay-line-taxes` said: "I found the actual
grading tests in the bench tree". `leak.py` then classified every tool call of all 90 sessions
(`leak.md`):

| arm | sessions | exposed | names a hidden test | names another session's files | names the source tree |
|---|---|---|---|---|---|
| `cold` | 30 | 1 | 1 | 1 | 1 |
| `notes` | 30 | 3 | 3 | 2 | 1 |
| `av` | 30 | 4 | 4 | 4 | 2 |

- All 8 exposed sessions are on `relay-line-taxes`, the hardest task of the five, and all 8 were
  accepted. No session on another task is exposed.
- **Cause.** Sessions run as root in the harness container, where the work root holds every
  earlier session's directory and `/src` holds the bench tree with the grading tests. This run
  added `--keep-work`, so each earlier session's copy stayed on disk with the grading test the
  verify step had copied into it and with that session's solution.
- **Effect.** `relay-line-taxes` rose from 3 of 6 in `cold` to 6 of 6 in both note arms. With 7
  of those 12 sessions exposed, that rise cannot be put down to the note. Among sessions that
  were not exposed the task reads `cold` 2 of 5, `notes` 3 of 3, `av` 2 of 2, which is too few to
  say anything.
- With exposed sessions set apart (post hoc): `cold` 26 of 29, `notes` 27 of 27, `av` 20 of 26.
- The classes are by what a command names, not by what it printed, so a session counted as
  exposed may have learned nothing, and a session that found the test some other way is missed.

**What this means for earlier runs.** The same container layout was used for E4 at scale: one
container, sessions as root, the source clone with the grading tests on the same filesystem, and
earlier sessions' event streams under the work root. That sweep did not keep working copies, and
its volumes are gone, so its transcripts cannot be checked. Its `cold` arm had the same access
and solved 3 of 15 on the decision tasks, so widespread use of the leak there is unlikely, but it
is not excluded.

## Secondary measures as registered

- **Sessions that wrote the wrong edit** (`analysis.md`): `cold` 3 (none accepted), `notes` 0,
  `av` 6 (2 accepted, 4 not). `cold`'s three are `relay-line-taxes`, where the wrong edit is the
  obvious first try. Two further `av` sessions failed `relay-quote-split` with the wrong approach
  in a form the pattern did not match.
- **Delivery:** the task's wrong note reached the agent in 30 of 30 `notes` sessions (whole, in a
  tool result; 28 by a read of `NOTES.md`) and 30 of 30 `av` sessions (whole, in the session-start
  hook's output). It is in no kept copy's `git log` (30 of 30 `notes` copies checked) and in no
  `cold` session's event stream.
- **Cost:** per accepted task `cold` $0.0119, `notes` $0.0160, `av` $0.0172. Median session cost
  from the harness (`wrong/report.md`): `notes` $0.013 ($0.011 to $0.015), `av` $0.011 ($0.009 to
  $0.012).

## Deviations from the pre-registration

- **The leak above.** The pre-registration says `--keep-work` "changes nothing a session sees".
  That was wrong.
- The delivery check on the first session of each arm passed as written. The first `av` session's
  hook counts were session-start 1, pre-read 3, post-edit 1, stop 2.
- `leak.py` and `behaviour.py` were written after the run. `analyze.py` was committed before it.
- No row was edited, dropped or re-run. `rows.jsonl`, `meta.json` and `report.md` are as the
  harness wrote them. `meta.json`'s `av_rev` reads `unknown`, as the pre-registration said it
  would; the build is the tree of commit `8121f29`.

## Spend

| | |
|---|---|
| harness total (peak prices) | **$1.2557** (smoke $0.0424, `cold` $0.3214, `notes` $0.4785, `av` $0.4134) |
| DeepSeek balance before | $3.30 (2026-11-11 17:05 UTC) |
| balance after | $2.68 (18:19 UTC, 2 minutes after the last session; $3.15 at 17:28 after `cold`) |
| balance delta | **$0.62** |

The run was inside DeepSeek's off-peak window, so the bill is about half the harness figure. No
stopping rule was reached.

## Exact commands

In `lp-harness` from `/src` (this branch), the key passed in with `docker exec -e
DEEPSEEK_API_KEY` and never written down, each command under a subreaper:

```bash
export PATH=/target/release:$PATH
unset ANTHROPIC_API_KEY
export ANTHROPIC_BASE_URL=https://api.deepseek.com/anthropic
export ANTHROPIC_AUTH_TOKEN="$DEEPSEEK_API_KEY"
export ANTHROPIC_DEFAULT_HAIKU_MODEL=deepseek-flash ANTHROPIC_SMALL_FAST_MODEL=deepseek-flash
export CLAUDE_CODE_SUBAGENT_MODEL=deepseek-flash
P="--price-per-mtok 0.30,0.30,0.006,1.20"
OUT=bench/results/2026-11-wrong-lesson
COMMON="--av-url http://127.0.0.1:7450 --work-root /root/bench-work --import-dir /root/bench-work/src --import-mount /root/bench-work/src"
scholion-bench run bench/tasks-relay/order-ids.toml --arms cold,notes,av --jit-hooks --reps 1 $P \
  --session-cap-usd 0.08 --budget-usd 0.30 --keep-work --out $OUT/smoke $COMMON --confirm
scholion-bench run bench/tasks-relay-wrong --arms cold --jit-hooks --reps 6 $P \
  --session-cap-usd 0.08 --budget-usd 0.60 --keep-work --out $OUT/cold $COMMON --confirm
scholion-bench run bench/tasks-relay-wrong --arms notes,av --jit-hooks --reps 6 $P \
  --session-cap-usd 0.08 --budget-usd 2.00 --keep-work --out $OUT/wrong $COMMON --confirm
```

Setup: `PREREG.md`, "Differences from the E4-at-scale setup". Session times: smoke 17:10 UTC,
`cold` 17:13 to 17:28, `notes` and `av` 17:28 to 18:17.

## Files

- `cold/`, `wrong/`, `smoke/`: the harness's `rows.jsonl`, `meta.json`, `report.md`.
- `first-packs/`: the free replay before the run.
- `analysis.md` from `analyze.py`; `leak.md` from `leak.py`; `behaviour.md` from `behaviour.py`;
  `patterns.py`, the free check of the dead-end patterns.
- The session directories (event streams, kept copies) are in the build container and are not
  committed.

## What to do next

1. **Close the leak before any other paid cell.** Run the agent as a user that can read only its
   own working copy, with the bench tree and earlier sessions out of reach, and check it with a
   session whose task is to find the grading test.
2. **Run this cell again on the closed harness**, pre-registered, with the per-task comparison on
   `relay-quote-split` named in advance and more tasks of its kind: a wrong note that the code
   and the visible tests do not contradict.
3. **Then test a change to how a record is presented**, not a new store feature: say in the pack
   that a record is an earlier agent's claim to check against the code, and tell the agent it can
   supersede one. The same cell measures it, and the true-lesson relay tasks check that it does
   not cost the 30 of 30 that true lessons give.
