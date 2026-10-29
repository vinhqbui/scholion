# Beat NOTES.md — the only metric that matters next

Date: 2026-10-29.
Status: **proposal for second look** (not yet approved).
Author: **Auto (Composer)** — Cursor cloud agent. Claude: tear this apart.

---

## The point

Scholion exists to spend fewer tokens per solved task than the next-best thing an
agent already has.

Lean av (same day) proved we can stop *taxing* the agent. Ritual MCP close-out
was 85 % of the first E4 overrun. We killed it. Result:

| arm   | accepted | cost / accepted |
|-------|----------|-----------------|
| cold  | 18/30    | $0.022          |
| notes | 30/30    | $0.013          |
| av    | 30/30    | $0.013 (−1.8 %) |

Parity with a plain file is the **floor**, not the product.

A ten-line `NOTES.md` is free to read whole. Av's selective delivery only earns
its infrastructure when the file is too big to skim. Until we beat that case,
Scholion is an expensive notebook with a daemon.

**Mission for this chapter:** make av cheaper per accepted task than `NOTES.md`
when there are hundreds of mostly-irrelevant notes — and do it without
reintroducing ritual.

If we cannot clear that bar, we delete surface area until we can, or we admit
the product is adoption theater.

---

## First principles (delete everything that fails these)

1. **Tokens are the product.** Latency, elegance, and feature count are
   downstream. If a change does not cut tokens per accepted task (or raise
   acceptance at equal cost), it does not ship in this chapter.
2. **Count the whole bill.** Cache writes, cache re-reads, hook text, Bash
   discovery, thinking about the pack, then thinking again about the file the
   pack already named — all of it. Reports that exclude cache reads are lying
   politely.
3. **Do not paste what the agent will Read anyway.** Claude Code refuses to
   edit unread files. Code *bodies* in a start pack are paid twice. Signatures
   and handles are the default; bodies are on demand.
4. **One budget law.** Every pack entrypoint obeys the same default. A "lean"
   `get_context` next to a fat `start_task` is self-sabotage.
5. **Progressive fetch or die.** Handle → summary → full was the original
   design. Evidence: ~203 pack exposures, **one** `expand`. Either make
   `expand` the cheap path or delete the progressive story from the marketing.
6. **Measure before you believe.** Cap at 800 without recall@800 is faith.
   Faith is not an engineering process.

---

## What lean already settled (do not reopen)

Done. Do not spend another design cycle on these:

- MCP off by default for Claude Code; `--av-mcp` for the comparison.
- No manual `checkpoint` after edits; `submit` snapshots.
- Inline records on `submit`; multi-record JSON; stop-hook one-call form.
- Default `get_context` budget 800.
- Bench `path_prepend` so agents can run pytest.
- E4 lean result: cost parity with notes, clear win over cold.

Lean is closed. The next work is **pack economics and selective delivery**.

---

## The proposal — four bets, in order

### Bet 1: One budget. Everywhere. 800 unless you ask.

**Problem.** MCP `get_context` defaults to 800. MCP `start_task` still
documents 4000. Bundles/claim paths still default 4000. Agents with one fat
ask undo the whole lean win.

**Change.**

- Default budget **800** for: `get_context`, `start_task`, SessionStart hook,
  claim/handoff first packs, CLI `av context`, dashboard preview (already
  aligned in #18).
- Bundle/PR packs keep a separate, documented default (review needs more) —
  but agent *work* packs are 800.
- Tool text: "small by default; pass `budget` only when you know you need
  more." Soft-cap warning in the receipt when requested budget > 2000
  (`receipt.warnings += "large_budget"`).
- E1 gains a **800-token** column (and signatures-only variant). CI gate adds
  a floor at 800 on the fixture. No paid agent run ships a new default until
  recall@800 is measured on real retrieval tasks.

**Kill criterion.** If recall@800 on E1 real repos is worse than BM25 by more
than a small epsilon, raise the default to the smallest budget that ties BM25
— do not pretend 800 is sacred.

### Bet 2: Handle-first packs. Expand is the product.

**Problem.** Haiku showed: pack bodies are cache-written once, re-read every
turn, and then the agent Reads the same file to edit. Pack cost ≈ turn saved.
Signatures-only already won on Haiku; we did not finish the job.

**Change.**

- SessionStart and default `get_context` emit **handles + signatures** for
  code; decision/attempt **summaries** (≤200 chars), not bodies.
- Bodies only via `expand(id)` / `read_symbol(id)` / explicit
  `code=full` / `budget` large enough that the allocator upgrades tiers.
- Pack footer: one line naming the expand command for the top omitted or
  handle-only items (so the agent does not re-`get_context` with 4000).
- MCP for non-hook clients: **always-on minimal set** —
  `get_context`, `record`, `submit`, `expand` (and `read_symbol` if distinct).
  Everything else deferred. ToolSearch must not eat the lean win for clients
  that still use MCP.
- Instrument: `expand` calls per session, fraction of code tokens that were
  bodies vs signatures, requested budgets histogram.

**Kill criterion.** If after handle-first, agents spam `get_context` with
large budgets instead of `expand`, the tool text and footer failed — fix copy
and schema before adding features.

### Bet 3: Stable cache prefix. Stop rewriting the bill every turn.

**Problem.** Prompt cache charges write once and read every turn. A pack whose
prefix thrashs every call burns the saving. HANDOFF B8 already named this;
lean ignored it because ritual was louder.

**Change.**

- Section order for agent packs becomes cache-stable within a session:
  1. conventions (byte-stable while scope unchanged)
  2. map (stable until seeds change)
  3. task/intent (stable after start)
  4. decisions & dead ends (append-only growth at the end of the section)
  5. code (most volatile — last)
  6. live work (last)
- Ledger references stay short and append-only so earlier bytes do not shift.
- Receipt reports `cache_stable_prefix_tokens` (estimated) so we can see it in
  bench rows.

**Kill criterion.** If prefix stability does not move cache-read cost in a
paired Haiku or DeepSeek ablation, drop the reorder — do not keep complexity
for aesthetics.

### Bet 4: E4 at scale — the only eval that can crown or kill av.

**Problem.** Ten notes fit in one Read. Half the relay tasks pass `cold`
(traps the model avoids alone). Cost/accepted mixes "memory tasks" with
"model can solo." Lean's success criterion cannot show av < notes by
construction.

**Change.**

- New task set (or scaled relay): **≥200 notes/records**, most irrelevant to
  the active task; gold lessons still required for acceptance on
  memory-sensitive tasks.
- Filter reporting: primary metric = cost per accepted on **memory-sensitive**
  tasks only (those where `cold` acceptance < 50 % in the pilot). Trap tasks
  are exploratory.
- Arms: `cold`, `notes`, `av` (hooks+CLI). Optional `notes-grep` (agent told
  to search the file) as a stronger baseline if plain `notes` cheats by
  reading a truncated head.
- Harness columns (first-class, not transcript archaeology):
  - counts: checkpoint, record, submit, status, get_context, expand,
    Bash-`av`, ToolSearch
  - per pack: requested budget, rendered tokens, code tier, ledger hits
  - cache write / cache read tokens and $ (stop excluding reads from the
    honesty pass)
  - stop-nudge fire rate; inline-record submit rate
- Pre-register in `bench/results/<date>/PREREG.md`: primary = cost/accepted on
  memory-sensitive tasks; decision rule = av ≤ notes − 15 % or we do not claim
  victory.

**Kill criterion.** If at scale av still cannot beat notes on cost/accepted
for memory tasks, the next move is **adoption/simplification** (git-notes
export, `--local` no-daemon) — not more packer knobs. The install cost must
match the measured gain.

---

## Explicitly out of scope (this chapter)

- Auto-drafting records from the diff (high value; separate spec — do not
  smuggle it in and dilute the eval).
- Git-native records / no-daemon solo mode (option B) — runs **after** the
  scale eval answers whether av's delivery beats a file.
- PageRank map / cross-encoder rerank — only if E1@800 loses to baselines;
  then they are packer fixes, not product features.
- E3 Haiku reruns, Anthropic-budget work.
- Mergiraf, more languages, GitHub PR bridge, per-agent tokens — real, later.

---

## Success criterion (one number)

On the scaled E4 (memory-sensitive tasks, DeepSeek or cheaper sweep first):

> **av cost per accepted task ≤ 0.85 × notes cost per accepted task**,
> with av acceptance ≥ notes acceptance,
> and cold clearly worse on those same tasks.

Secondary (must not regress): lean ritual stays dead — median
`checkpoint` calls from the agent = 0; ToolSearch for av tools = 0 on the
hooks-only path.

If we miss: publish the miss in HANDOFF, cut scope, do not rename the goal.

---

## Delivery shape

One design → one implementation plan → one PR series off `main` (not stacked
unless forced):

1. Budget unification + E1@800 gate (no agent money).
2. Handle-first + expand footer + minimal MCP surface.
3. Stable pack prefix + receipt field.
4. Harness instrumentation.
5. Scaled E4 run; result in `bench/results/` and HANDOFF whatever it shows.

TDD at every seam. `cargo fmt`, clippy `-D warnings`, `cargo test --workspace`
green before each commit. Agent-facing text says `av`.

---

## Why this and not a pile of ideas

HANDOFF already lists twenty improvements. Most are real. Most do not answer
the only question customers will ask: *why not a NOTES.md?*

This proposal answers that question with a killable experiment. Everything
else waits on the result.

Ambition without a kill criterion is cosplay. Ship the experiment.

---

## Author's note for Claude (second look)

I am Auto (Composer). I wrote this from the lean-av design, the lean E4
result, the Haiku pack/cache analysis, Plan 6 claims C1/C2, and HANDOFF §1a
and §5.

Pressure-test especially:

1. Is −15 % vs notes the right decision rule, or too aggressive / too soft?
2. Is 800 still the right default once Bet 2 removes bodies, or should default
   rise because signatures are cheaper per item?
3. Did I under-weight auto-draft records as a remaining close-out tax?
4. Is "kill av's thesis → pivot to adoption" the correct failure mode, or
   should packer/retrieval get one more iteration first?
5. Any internal contradiction with the 2026-10-18 design or lean-av invariants?

Mark disagreements in-line. Do not soften the mission to protect the daemon.
)
