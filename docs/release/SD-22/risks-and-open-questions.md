---
canonical: true
owner: god-emporer
status: approved (operator review 2026-07-15; operator directives 2026-07-17 expanded scope to APG + ACG; operator clarification 2026-07-18: "ACG, APG are the two advanced guides"; branch + board pinned 2026-07-18 to tranche/5 / codex-tranche-5; override flags A–D defaulted; bundle marked planning-ready)
date: 2026-07-15
canonical_branch: tranche/5 (operator directive 2026-07-18)
kanban_board: codex-tranche-5 (operator directive 2026-07-18)
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/decisions.md
---

# SD-22 — Risks and Open Questions

This file enumerates the risks, blockers, and open questions specific to SD-22. Structured to mirror SD-21's risks docs.

## Self-healable conditions (resolve inline, exit GREEN)

| Condition | Detection | Self-heal |
|---|---|---|
| Working tree dirty at cycle start | `git status --porcelain \| wc -l` returns non-zero | Run `git stash` (if unfinished) or `git checkout -- .` (stray edit noise); re-verify clean; retry |
| A merge conflict on a structured-data file in `rules_tables/apg/` / `acg/` / `beastiary1/` when two cycles touch adjacent files | Merge conflict on the structured-data file | Resolve inline if mechanical (ordering, an extra row); escalate to operator if semantic |
| A cycle's RED test fails because the `RuleSetId` enum's variant isn't yet wired into the resolver | `tests/sd22_<book>_class_table_resolve` fails for a book that hasn't been ingested yet | Route to Open Blockers; operator decides whether the cycle is the right time to ingest that book |
| A cycle's RED test fails because the parser is mis-parsing a publisher's structured-data file | Per-file parse error on the cycle's `<book>/class_<class>.rs` load | Surface the file path and parse error in the load result; route to Open Blockers if the parse is unfixable per-cycle |
| Markdown file on disk has a stale `nonce` (from a Drive sync edge case, if SD-22 surfaces this) | `CampaignSnapshot.nonce != saved_nonce` on load | Engine surfaces "stale nonce, please save again"; doesn't trigger conflict log unless the *content* also differs |
| Per-class Epic 3+4 ordering collision | Two cycles both try to land the same APG/ACG class table | Defer the second cycle until the first cycle's tests are green; surface as `## Open blockers` if the conflict indicates a real per-book issue |

## Non-self-healable conditions (write to `## Open blockers`, exit FAIL)

| Condition | Detection | Why not self-heal |
|---|---|---|
| The DM-toolkit encounter-math (`Encounter::new`) returns a result inconsistent with the canonical Paizo encounter table | DM-toolkit deterministic test (criterion 20) fails | Rules-engine correctness violation; a PF1 player will catch it within 30 seconds; cycle can't fix the algorithm alone |
| A published source-book content record is loaded but its `RuleSetId` variant doesn't match the cycle's expected book | Per-class test fails with `RuleSetId` mismatch | Wrong-book attribution; the resolver changes; cycle can't fix this alone |
| The `codex-tranche-5` kanban board doesn't exist at cycle-1 launch | Epic 2 criterion 3 cycle fails on board lookup | Operator-side setup; bundle can't proceed until the board is created (or in this case, until the prior dead-state board is repurposed) |
| Two `claude` processes both touch `src/rules_core/rules_tables/<book>/` | `ps -eo pid,etime,stat,cmd \| grep claude` shows multiple in-flight on the same file set | Structural: one-lane-at-a-time rule |
| Cargo test regresses on a row other than the one the cycle touched | Full suite regresses after a cycle's change | Sibling-preservation is a hard rule |
| Progress doc and live matrix disagree on a row's `evidence_tier` (not just stale snapshot) | Cycle's expected vs. actual differ | Manual operator reconciliation required |
| The DM-toolkit encounters.rs requires Paizo-published PF1 encounter math that requires licensing compliance | Operator-side legal review | The bundle can't ship with non-PF1-compatible rules; operator calls legal |

## Override flags (durable; patched when operator accepts a default)

### Flag A — APG 9-class per-cycle ordering

**Default chosen**: alphabetical by class name (Alchemist → Cavalier → Gunslinger → Inquisitor → Magus → Oracle → Summoner → Witch). One cycle per class.

**Override alternatives:**
- *Bestiary-style ordering* (by level progression: low-level classes first, high-level last)
- *PF1-publication ordering* (the order Paizo published APG classes in the book)
- *Tier-of-evidence ordering* (classes with strongest published evidence first)

**Override cost**: ~30 minutes; affects the cycle-ordering block of `epic-breakdown.md`.

### Flag B — ACG class per-cycle ordering

**Default chosen**: alphabetical by class name (Alchemist → Arcanist → Bloodrager → Brawler → Hunter → Investigator → Shaman → Skald → Swashbuckler → Warpriest).

**Override alternatives:**
- *Hybrid ordering* (intersperse APG and ACG classes by shared-class-priority, e.g. Alchemist appears twice across the two books)
- *Theme-grouped ordering* (casters, martial, hybrid)
- *PF1-publication ordering*

**Override cost**: ~30 minutes; affects the cycle-ordering block of `epic-breakdown.md`.

### Flag C — Bestiary 1 monster-block ordering

**Default chosen**: alphabetical by monster name within each CR band (CR 1/8 → CR 30, alphabetical within each band).

**Override alternatives:**
- *Alphabetical by environment* (group by terrain: forest / desert / urban / etc., then alphabetical)
- *PF1-publication ordering* (Paizo's Bestiary 1 page order)
- *By monster-type* (humanoid, beast, dragon, outsider, etc.)

**Override cost**: ~30 minutes.

### Flag D — DM-toolkit math surface scope

**Default chosen**: `Encounter::new` (encounter difficulty) + `party_challenge_rating` (party CR). Two functions, two modules (`encounters.rs`, `party_cr.rs`).

**Override alternatives:**
- *Single-module with both functions* (`dm_toolkit.rs` containing both `Encounter::new` and `party_challenge_rating`; simpler, but harder to test in isolation)
- *Three-module split* (`encounter_difficulty`, `party_cr`, `monster_role` adding monster-role-attack-bonus logic; richer surface but more work)
- *Full DM-toolkit GUI* (out of scope; that's `SD-23`)

**Override cost**: ~1 hour (re-doing the modular split); affects `technical-design.md` and `epic-breakdown.md`.

## Architectural questions (Q1–Q5 OPEN, defer to operator review)

### Q1 — APG/ACG class table schema: per-class Rust module or per-feature table?

**Default chosen**: per-class Rust module (`rules_tables/apg/class_<class>.rs`) with structured data entries per level per the APG's class table.

**Override alternatives:**
- *Per-feature tables* (separate `feat_table.rs`, `spell_table.rs`, `equipment_table.rs` per book) — more reusable across classes but harder to keep per-class in one place
- *Single combined module per book* (`rules_tables/apg.rs` containing all classes) — simpler but doesn't scale

**Override cost**: ~1 hour (re-doing the table layout).

### Q2 — DM-toolkit's encounter-math precision

**Default chosen**: PF1's "Encounter Building" rules with a per-monster CR-weight table (per the canonical Paizo example).

**Override alternatives:**
- *Linear scaling* (simple per-monster CR sum)
- *Logarithmic scaling* (handles extreme party-vs-monster mismatches)

**Override cost**: ~4 hours (re-deriving the math + tests).

### Q3 — Cross-book resolver priority order

**Default chosen**: per SD-21 §12 doctrine: APG → CRB → ACG → Bestiary1 (priority for cross-book fallback; independent per `RuleSetId` for content reads).

**Override alternatives:**
- *CRB-first* (legacy CRB is the canonical source)
- *Per-class priority* (e.g. Wizard-class from any book → CRB first, then APG)

**Override cost**: ~30 minutes (changing the priority order in `equipment_id_resolve` / `spell_id_resolve`).

### Q4 — Bestiary 1: full 300+ monsters or first 100?

**Default chosen**: 300+ monsters (full ingest); one cycle per monster-block subset.

**Override alternatives:**
- *100-monster minimum* (1 cycle per monster for 100 cycles) — faster to closure but incomplete
- *CR-band-by-CR-band* (low CR first, high CR later) — player-facing value incremental

**Override cost**: ~30 minutes (changing the per-cycle work unit).

### Q5 — Build version number for SD-22's first release

**Default chosen**: `0.5.<current_build>` (per the operator's 2026-07-17 `<major>.<tranche-base>.<build>` amendment, applied symmetrically). Major stays `0` until first main-publish; tranche is `5` because `tranche/5` is the active branch base; build is the next monotonic counter value after the last committed build on `tranche/5`.

**Override alternatives:**
- *Reset build to 0* (treat SD-22's launch as a fresh counter) — but operator's amendment said "monotonic across all builds across all branches — never resets"
- *Use SD-21's last build value* — operator-pinned at cycle launch

**Override cost**: 0; the operator pins the value at SD-22 cycle launch.

No remaining open architectural questions for SD-22. Future-class concerns (DM-toolkit GUI, Ultimate-line book ingest, multi-DM campaigns) are deferred to future bundles (SD-23+, SD-22 expansions).

## Open judgments deferred to next SD (Epic 9 evaluator's parking lot)

Per operator directive 2026-07-19 (Epic 9 — Closure Readiness doctrine): when Epic 9's evaluator encounters a state that looks suspicious but isn't a clean shortfall (e.g., "a rule-table entry looks wrong but a unit test passes," "an ingested book is technically correct but the cross-book resolver returns a counter-intuitive fall-through," "Epic 6's deterministic test passes against a Paizo canonical example but the example chosen is ambiguous in the source book"), Epic 9 does **not** self-heal it. Epic 9 logs the judgment here for the *next* SD's audit (e.g. SD-23) to pick up — the operator-judgment-call rule is the doctrine boundary that separates "fix in-bundle" from "defer to next bundle."

Each log entry has this shape:

```
### Judgment-<n> — <one-line summary>
- Surfaced by: Epic 9 cycle on `<YYYY-MM-DDTHH:MM:SSZ>` during eval cycle #<N>
- Criterion nearest to the observation: <criterion-NN>
- Why Epic 9 didn't self-heal: <one-line reason — typically "technically correct but counter-intuitive" or "passes unit test but reads suspect">
- Recommended next-SD action: <one-line — typically a new capability-slice in SD-23 Epic 1 (Identifier Cleanup if identifier-shaped), Epic 9 (Closure Readiness if audit-shaped), or another Epic per scope>
- Status: deferred
```

(Epic 9 adds entries to this section as it runs; the next bundle's audit reads them on first cycle.)

## Cross-reference

- `acceptance-and-verification.md` — closure gates (gates 1-16).
- `decisions.md` — the 3-item decision record (§1 scope, §2 tranche/5 + codex-tranche-5, §3 deferred shape decisions).
- `epic-breakdown.md` — 30 acceptance criteria grouped into 8 epics.
- `technical-design.md` — content-source ingest patterns + DM-toolkit architecture.
- `./scope-draft.md` — canonical handoff.
- `./loop-instruction.md` — loop body.
