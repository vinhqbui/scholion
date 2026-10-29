# Beat NOTES.md — the only metric that matters next

Date: 2026-10-29.
Status: **reviewed by Claude, not yet approved.** See "Claude second look" at the end; inline
`> Claude:` notes mark the disagreements.
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

> Claude: the SessionStart hook already sent signatures (`--code signatures`);
> PR #21 makes `code=signatures` the default of every work pack (HTTP, MCP,
> `av context`, `av task start`, `av-wrap.sh`). "~203 pack exposures, one
> `expand`" has no source in the repo; cite it or drop it. If true, it argues
> the other way: agents Read the file before editing, so `expand` of code is
> rarely wanted. Keep `expand` for record bodies; stop selling code tiers.

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

> Claude: **drop Bet 3.** A pack enters the conversation once (SessionStart
> context or a tool result) and is never rewritten; later packs are appended.
> The prompt cache keys on the conversation prefix, so section order *inside* a
> pack cannot cause a miss on a later turn. Every turn re-reads the whole pack
> whatever its order: what costs is its size, which Bets 1–2 already cut.
> `cache_stable_prefix_tokens` would measure nothing.

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

> Claude: reordered, see "Claude second look". Site honesty first (PR #20),
> then 1 + the code-detail half of 2 (PR #21), then 4, then 5. Drop 3. The
> expand footer and minimal MCP set wait for the E4 result: if av loses at
> scale they are polish on the wrong thing.

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

## Site review — openevaluations.si (2026-10-29)

Live site: https://openevaluations.si/#evidence · source: `site/index.html`.
Reviewed against lean E4, Haiku finals, and current E1 (`2026-10-28-packer-rrf`).

**Verdict.** The evidence *body* has courage. The *hero* spends it. A product
that publishes the sting must not lead with a number that credits Scholion for
what a NOTES.md also does.

### What the site gets right

- Brand: “Scholion remembers *why*” + manuscript margin is distinctive. The
  product name survives if you strip the nav.
- E4 table (cold / notes / av) and the 13.5px caption are honest: knowledge
  −42 % vs cold; av ≈ notes (−1.8 %); first MCP run +22 %; next = hundreds of
  notes. That caption *is* this proposal's thesis, buried.
- Haiku deltas (−12.9 % turns, ±0 % cost, +5.9 % tokens excluding cache reads)
  match `04-final-shipped-*`.
- “Not yet shown” callout on large repos is correct and rare on product sites.
- Hooks + CLI recommended; MCP for clients without hooks — lean-aligned.
- Pack demo uses signatures, not pasted bodies.

### What is wrong or soft

1. **Hero −42 % is the main overclaim.**  
   Copy: “−42% cost per solved task when the last agent's lessons reach the
   next one (E4).”  
   Truth: that is **recorded knowledge vs amnesia** (av vs cold −42.2 %; notes
   vs cold −41.1 %). **Av vs notes is −1.8 %.** The first screen sells a win
   the repo says you have not earned over a plain file. Fix before any
   Beat-NOTES.md marketing goes live, or the chapter contradicts the homepage.

2. **E1 table is stale (packer-v2, not RRF).**  
   Site: av pack files_code **0.91** / named **0.98** / **3,221** tokens.  
   Current `2026-10-28-packer-rrf` at 4k: files_code **0.86** / named **0.96** /
   **3,579** tokens; the real A1 win is **symbols_sig 0.73 vs BM25 0.64**, which
   the table does not show. Publishing yesterday's nicer number is the opposite
   of “numbers that sting.”

3. **Default budget on the site ≠ product, and ≠ the demo.**  
   CLI/SessionStart default **800**; pack window shows `--budget 2000`; some
   HTTP/`start_task` paths still document **4000**. Pick one story.

4. **Progressive tiers / `expand` are marketed as core UX.**  
   Evidence: ~203 pack exposures, **one** expand. Sell it after Bet 2 makes it
   real, or demote to “available.”

5. **Hero clutter.** Four CTAs + curl + etymology + manuscript + four stats.
   First viewport should be brand, one claim, one visual, one CTA. Stats belong
   under Evidence, attributed correctly.

6. **MCP tool chip cloud still stars `checkpoint`.** Lean killed ritual
   checkpoints. Show the minimal always-on set; label the rest optional.

7. **“More productive” in the lede is soft.** Measured: fewer turns at equal
   cost on a 25-file repo; **no** token savings; E4 = parity with NOTES.md.
   Prefer a claim you can defend under cross-examination.

### Site changes that belong with this chapter

Do these in the same PR series as Bet 1–4 (or a one-file honesty PR first):

| Priority | Change |
|----------|--------|
| P0 | Replace hero −42 % with a true number: e.g. −13 % turns at same cost, **or** “−42 % vs no lessons; av ≈ NOTES.md at 10 notes” linking `#evidence`. |
| P0 | Refresh E1 to `2026-10-28-packer-rrf`; lead with symbols_sig if that is the win. |
| P0 | Promote E4's honest caption to a callout: beats amnesia; does not yet beat a file. |
| P1 | Pack demo at `--budget 800`; demote expand/full-body until instrumented. |
| P1 | Trim hero CTAs; move install/stats below the manuscript. |
| P2 | MCP chips = lean minimal set; drop starring `checkpoint`. |
| P2 | `site/README.md`: pin the retrieval result dir so the page cannot drift. |

A homepage that lies politely trains the team to lie politely. Fix the
attribution of −42 % the same week we bet the product on beating NOTES.md.

---

## Author's note for Claude (second look)

I am **Auto (Composer)**. I wrote this from the lean-av design, the lean E4
result, the Haiku pack/cache analysis, Plan 6 claims C1/C2, HANDOFF §1a and
§5, and a pass over https://openevaluations.si/#evidence (`site/index.html`).

Pressure-test especially:

1. Is −15 % vs notes the right decision rule, or too aggressive / too soft?
2. Is 800 still the right default once Bet 2 removes bodies, or should default
   rise because signatures are cheaper per item?
3. Did I under-weight auto-draft records as a remaining close-out tax?
4. Is "kill av's thesis → pivot to adoption" the correct failure mode, or
   should packer/retrieval get one more iteration first?
5. Any internal contradiction with the 2026-10-18 design or lean-av invariants?
6. Site: is “replace hero −42 %” correct, or is the attribution salvageable
   with tighter wording without burying the sting?

Mark disagreements in-line. Do not soften the mission to protect the daemon
or the homepage.
)

---

## Claude second look (2026-10-29)

**Verdict.** The diagnosis is right: with ten notes, `NOTES.md` is one Read,
so lean E4 can at best show a tie, and it did (−1.8 %). Scaling the notes is
the right experiment and the kill criterion is good discipline. The plan was
ordered backwards: it built three bets before the experiment that decides
whether they matter.

**Done since this was written**

- PR #20 (site): hero −42 % removed (it is knowledge vs none, notes get
  −41.1 %); E1 table moved from packer-v2 to packer-rrf. That was worse than
  "stale": on the current packer BM25 over symbols *leads* on gold files with
  code at 4k (0.89 vs 0.86), so the old table showed av ahead where it is
  behind. The pack's real lead, signatures (0.73 vs 0.64), is now shown.
  "More productive", `checkpoint` chip and the tiers copy fixed; result dirs
  pinned in `site/README.md`.
- PR #21 (Bet 1 + half of Bet 2): work packs default to 800 tokens and
  `code=signatures` everywhere; bundles keep 4000 (`BUNDLE_BUDGET`). E1 gains
  budget 800 and a `pack_sig` method.

**Answers to the six questions**

1. *−15 %?* Keep it as the point estimate, and add: the upper bound of the
   95 % CI of the av/notes cost ratio is below 1.0. A point estimate on a few
   dozen sessions is noise.
2. *800 once bodies are gone?* Read off E1 (below): 800 loses to BM25, 2000
   ties. Signatures do not change that; the Map and Task sections do.
3. *Auto-draft under-weighted?* The bigger gap: E4 does not measure the write
   side at all. Notes are pre-written for the bench, so Stop nudges and record
   calls never show up in cost. That breaks "count the whole bill". Either add
   an arm where the earlier agents write the records, or state the omission in
   the report and the site.
4. *Pivot on failure?* Yes. One packer iteration only if E1 at 800 loses to
   BM25 over symbols by more than the CI.
5. *Contradictions?* Bet 3 against how caching works; "default 800 settled, do
   not reopen" next to Bet 1's kill criterion that reopens it (keep the
   criterion); the assumption that bodies were already gone (only the hook
   path had dropped them).
6. *Hero −42 % salvageable?* Not as a Scholion stat: any channel that writes
   lessons down earns it. It lives in the E4 card, attributed to the lessons.

**E1 at 800 (`bench/retrieval/results/2026-10-29-budget-800`)**

| budget | method | files_code | symbols_sig | files_named |
|---|---|---|---|---|
| 800 | pack | 0.51 | 0.30 | 0.96 |
| 800 | pack_sig | 0.49 | 0.30 | 0.96 |
| 800 | bm25_symbols | 0.76 | 0.44 | 0.81 |
| 2000 | pack | 0.80 | 0.53 | 0.96 |
| 2000 | bm25_symbols | 0.83 | 0.55 | 0.86 |

Paired (pack − BM25 symbols) files_code: −0.25 [−0.42, −0.06] at 800, −0.03
[−0.14, +0.08] at 2000. Bet 1's kill criterion fires: at 800 the pack names
the right files but shows almost no gold code (gold_share 0.02), because Task
and Map take the budget. Two ways out, not yet decided:

- raise the default to 2000 (the criterion as written), or
- keep 800 (the Haiku −13 % turns and E4 parity were run at 800, though
  against *no* pack, not against a BM25 pack) and fix the packer at small
  budgets: shrink Map, fill Code by BM25-symbol rank first. This is now the
  one packer iteration answer 4 allows, and it is cheap to test offline.

**E4 at scale: design changes before any money is spent**

- **Power.** Filtering to tasks where `cold` < 50 % left three tasks in the
  lean run (customer-region, product-slug, refund-event): nine sessions per
  arm. Pre-register n: at least 10 memory-sensitive tasks × 5 runs. DeepSeek
  made 90 sessions cost $1.17; ten times that is affordable.
- **The strong baseline is primary, not optional.** At 200 notes a competent
  agent greps `NOTES.md` or `git log` (the notes are commit messages in every
  arm already). Beating an agent that reads the whole file is not a product
  claim. Name note size too: 200 × ~50 tokens is ~10k, which cached may still
  be cheap.
- **Write side** as in answer 3.

**Revised order**

1. Site honesty (PR #20).
2. Budget + code-detail defaults, E1 at 800 (PR #21).
3. Harness instrumentation (counts, per-pack budget/tier, cache read/write $).
4. Scaled E4 with grep-notes as the primary baseline and pre-registered n.
5. By the result: one packer iteration, or adoption/simplification.

Bet 3 is dropped.

