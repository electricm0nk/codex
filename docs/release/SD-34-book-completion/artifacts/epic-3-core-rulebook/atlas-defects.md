---
canonical: true
owner: god-emporer
bundle_id: SD-34
date: 2026-08-27
---

# Atlas defects — Epic 3 (Core Rulebook)

Per `decisions.md §2`: "Any remaining step discovered that the atlas did not predict is a
DEFECT IN THE ATLAS." An empty file would be an excellent result; this file is not empty.
Each entry names the discovery, its `correction` retro event, and the atlas re-derivation that
followed.

## 1. Vacuous PCGen placeholder rows inside `class_feature_option_pool_record_not_held_by_engine`

**Discovered:** AT-34-E3-001, `class_feature_option_pool_record_not_held_by_engine` mechanism,
Cycle 3 (`AT-34-E3-001_class_feature_option_pool_cycle_receipt.md`).

**What the atlas predicted.** `decisions.md §2`'s ten-bucket taxonomy treats every bucket-B
unit as a real content gap: a record the engine should hold and currently does not. Cycle 2's
own next-cycle plan named 5 sub-causes of this one mechanism's remaining 55 units, including
"vacuous placeholder rows (3)" with a flag that this shape needed a ruling before disposition.

**What the corpus actually holds.** `Empty Selection ~ Standard {Barbarian, Monk, Rogue}`
(`data/corpus/core_rulebook/class_feature/empty_selection/*.json`) are PCGen's own "no
selection" default rows for a `CHOOSE`-menu, not Pathfinder rules content: `data.description`
is JSON `null`, `data.raw_tokens` carries exactly three structural entries (`KEY`, `CATEGORY`,
`TYPE`) and nothing else — no `DESC:`, no mechanical token of any kind. There is genuinely
nothing to compute and nothing to display; "the engine does not hold a record for this" is not
what is wrong with these three rows — the corpus's own record carries no content for any engine
to hold in the first place. This is an unpredicted verdict shape: neither "real content not yet
held" (bucket B's own predicted meaning) nor any of the ten buckets' existing "cleared by"
column describes "nothing to clear, by the corpus's own construction."

**Disposition.** `status: "deferred-with-reason"` (bucket `X`, `decisions.md §2`'s own "deferred
with a stated reason ... cleared by revisiting the stated condition") — never `text-complete`
(no text exists to render) and never left in bucket B (there is no gap to place a record into).
A new, closed, named-key lookup —
`class_feature_pool_catalog::VACUOUS_PLACEHOLDER_CLASS_FEATURES` — matches only these 3 exact
keys, proven against the live corpus by
`vacuous_placeholder_rows_are_genuinely_empty_in_the_committed_corpus`
(`src/rules_core/class_feature_pool_catalog.rs`). Deliberately a named list, never a shape
predicate: this mechanism's own Cycle 2 receipt already recorded a near-miss where gating a
sibling rung on record SHAPE ALONE promoted 188 unrelated corpus-wide records before being
caught and reverted pre-commit; an independent corpus-wide scan for this same
"`description` null, `raw_tokens` ⊆ {KEY, CATEGORY, TYPE}" shape found 41 matches across 6
other books, none of them vacuous in the same way (witch hex sub-features, uncanny-dodge
trackers, BWBI wondrous-item slots, ...) — those 41 are untouched by this table.

**Retro event:** `docs/retro/events/sd34-at-34-e3-001.jsonl`, `correction`
(`--verified-by "python3 scripts/completion_atlas.py --book core_rulebook --check"`).

**Atlas re-derivation:** `python3 scripts/completion_atlas.py --book core_rulebook --check`
re-run at this cycle's own HEAD; `class_feature_option_pool_record_not_held_by_engine`'s share
of bucket B drops by exactly 3 (55 → 52), and the 3 units now report under bucket `X`.
