---
canonical: true
owner: card15-duplicate-identity
status: fix landed (narrow, validated) + a major correction of the dispatch brief's own worked example
date: 2026-08-23
---

# Card 15 — the `duplicate_identity` collision residual: fix, and a correction

`15-card-15-internal-duplicate-identity-memo.md` pinned the cause of the `class_feature`
residual to `v06_work_inventory.rs`'s corpus-wide `(kind, key)` `duplicate_identity` collapse:
a `Kind::ClassFeature` row with no `KEY:` field falls back to its bare display name as its
identity, and two genuinely distinct records sharing a display name collide, with only the
first surviving. This cycle's dispatch brief named the fix site correctly and asked for the
identity to carry whatever actually distinguishes the colliding rows — and, per `§17a`, to
validate the instrument before trusting it. Doing that validation surfaced a correction that
changes what "distinguishes them" safely means for most of this population.

**Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6`.

## §17a re-derivation — before touching anything

Re-ran `15-card-15-class-feature-residual-cause-pin.py` against the pinned oracle and the
current (post-`is_internal_category`-fix) `docs/work-inventory.json`: **180 non-internal
residual rows, 158 collide** on `(book, key)` with another `class_feature` row in the same
book, **22 do not** (cause still unpinned for those). Both figures reproduce exactly. The
brief's own "207" combined figure (180 + 27 internal-collision-losers) also reproduces.

## The fix — `disambiguate_class_feature_fallback_collisions` (`src/bin/v06_work_inventory.rs`)

For a `Kind::ClassFeature` row with no declared `KEY:` field (`u.key == u.name`, the exact
fallback signature `enumerate_file` produces), when its bare key collides with another such
row in the same book, `CATEGORY:` is the field that reliably distinguishes them — validated
against every fallback collision group in the corpus (`15-card-15-duplicate-identity-key-
validation.py`, this directory): **64 groups, 164 rows, 0 byte-identical-content** (every
fallback collision here is a genuinely distinct record, never a restatement). `CATEGORY:`
disambiguates all 64 cleanly; `TYPE:` alone fails on 40/64.

The first row in file order keeps its bare key (id stability, mirroring `unit_id`'s own
"non-colliding units keep the id they have always had"); a later row whose `CATEGORY:` has not
been seen for this key gets a disambiguated key (`"<key> ~ <category>"`); a later row whose
category HAS been seen collapses to that bucket's key exactly as before (a true restatement —
this shape doesn't occur in the validated 64, but the mechanism handles it correctly either
way, proven by test).

`CATEGORY:Internal` rows never open a new bucket — an Internal-tagged row is PCGen's own
bookkeeping/tracker convention (`is_internal_category`'s domain), not a second player-facing
object; its key is left untouched so it competes for the SAME identity as any non-Internal
sibling exactly as `duplicate_identity` already does. Only rows with NO declared `KEY:` field
are touched: a row that DOES carry an explicit `KEY:` and still collides (e.g. two PFS-legal
variants sharing one author-declared `KEY:`, 16 groups/32 rows) is a corpus-author-declared
identity choice, out of scope — re-validated that `CATEGORY:` fails to disambiguate 10/16 of
those groups, confirming that leaving them on today's collapse-to-first behaviour is correct.

## The correction — the dispatch brief's own worked example is NOT safe to rescue as-is

The brief's worked example — `advanced_class_guide`'s four "Aberrant Bloodline" rows, one per
class — was the FIRST thing tested against the real corpus once the `CATEGORY:` mechanism was
built. Before landing it, `src/bin/v06_work_inventory.rs` was found to already carry a
**pre-existing, operator-confirmed, 33-id allowlist**
(`DUPLICATE_CHOOSER_DISPLAY_NAME_UNIT_IDS`, SD-31 `decisions.md` Decision 17) of bare
`class_feature` rows already proven, case by case, to be a PICKER option beside its own real
feature row (PCGen's `CHOOSE:`-pool idiom — the operator's own worked example, `cr_abilities_
class.lst:2333` `KEY:Sorcerer Bloodline ~ Aberrant`, the feature, beside `:2334` `Aberrant
Bloodline`, the picker), not a second distinct object — and `apply_duplicate_chooser_removal`
removes those exact 33 ids from the final inventory, with a hard `exit(1)` if the removed count
ever drifts from 33 (this bundle's own drift guard).

Tracing the FIRST candidate this cycle's mechanism was about to rescue —
`ultimate_magic:class_feature:accursed_bloodline` (`um_abilities_class.lst:566`,
`TYPE:SorcererBloodlineChoice`) — confirmed it is already ON that 33-id list. Its fallback-key
sibling this cycle's mechanism would have rescued (`um_abilities_class.lst:2070`,
`CATEGORY:Crossblooded Bloodline`) turned out to be the **identical Sorcerer feature**,
reachable through a second archetype prerequisite gate (`ABILITY:...|Sorcerer Bloodline ~
Accursed`, the same target line 566 references; both rows share the `Sorcerer_Accursed_*`
variable family and reference the same `Accursed Bloodline ~ Feat Tracker` row) — the identical
duplicate-chooser shape, simply not yet on the hand-reviewed list because it was previously
invisible (dropped by `duplicate_identity`, so Decision 17's own audit never saw it).

Decision 17's own text is explicit that this is deliberate, not an oversight to fix with a
smarter filter: *"A bounded, evidenced list rather than a live adjacency filter, on purpose: a
generic 'same name, adjacent line' rule would silently sweep in any FUTURE same-shaped
collision no human reviewed."* Building a live heuristic (adjacency, shared referenced KEY,
etc.) here would be exactly the thing that sentence forbids — a per-case hand review, like
Decision 17's own, is the only sound way to tell "genuinely different classes, different
mechanics" (the ACG Bloodrager/Blood-Arcanist/Crossblooded-Rager pattern this memo's OWN
predecessor described) apart from "the same feature, a second archetype gate" (the ultimate_magic
pattern above) — and the two shapes are not reliably distinguishable by `CATEGORY:`,
`TYPE:`, or field-content diffing alone (both pairs have real, non-byte-identical field-level
differences).

**Consequence:** every fallback-key collision group whose members ALL carry a `TYPE:` facet
ending in `"Choice"` is EXCLUDED from this cycle's rescue (39 of the 64 validated groups,
including the brief's own flagship example) — left untouched, same disposition as
`CATEGORY:Internal`. The remaining 25 groups (a `TYPE:FavoredClass` tracker row colliding with
an unrelated `TYPE:Class` chassis-selector row, one pair per class — e.g. `core_rulebook`'s
`Barbarian`: line 68 is the Favored Class Bonus tracker, line 98 is the unrelated class chassis
row) are hand-verified genuinely distinct content and ARE rescued.

## Population, before and after — both directions proved

```bash
export PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data
export CORPUS_LITERAL_SWEEP_REPORT=<fresh corpus_literal_sweep --json-out report>
export DERIVED_FIXTURE_CHECK_REPORT=<fresh derived_evaluator_fixture_check --json-out report>
cargo run --locked --bin v06_work_inventory
```

| | before | after | delta |
|---|---:|---:|---:|
| `totals.units` | 49,516 | 49,540 | **+24** |
| `class_feature` | 18,032 | 18,056 | **+24** |
| every other kind | — | byte-identical | 0 |

**Both directions, proved by physical-location diff** (`(book, source_file, source_line)`
triples, `pre` vs `post`): **0 physical locations lost, 24 gained, 0 duplicate ids in post, 0
duplicate physical locations in post.** 5 ids renamed (re-suffixed, `unit_id`'s own existing
slug-collision disambiguation — `barbarian_class`, `bard_class`, `cleric_class`,
`sorcerer_class`, `wizard_class`, each re-suffixed because a new slug-colliding sibling landed
in the same book+kind) — every one confirmed still present under its new id at the SAME
physical location. **Full `status` distribution diffed**: `literal-verified` 6,506 → 6,506,
`fixture-verified` 1,741 → 1,741, `grounded` 2,515 → 2,515, `text-complete` 3,858 → 3,858,
`deferred-with-reason` 46 → 46, `ingested-magnitude` 1,404 → 1,404, `not-started` 19 → 19,
`unknown` 4,264 → 4,264 — every stamp preserved exactly. Only `not-ingested` grew (29,163 →
29,187, +24), matching the 24 new units exactly (all `status: not-ingested`, enumerated not
engineered).

`apply_duplicate_chooser_removal`'s drift guard confirms the exclusion worked: the regen
completed without its `exit(1)` (which fires the moment the removed-33 count drifts), meaning
this cycle's fix created no new candidate for that allowlist — the risky population is
genuinely untouched, not merely hoped to be.

## Re-derived residual after this cycle

```bash
export PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data
python3 docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-class-feature-residual-cause-pin.py
```

**156 non-internal residual (was 180, −24 rescued)**: 134 still collide (was 158 — the 39
`*Choice`-typed groups plus the 16 keyed-collision groups, deliberately not rescued this
cycle), 22 unexplained (unchanged — the SAME rows the prior cycle already flagged as genuinely
unpinned; this cycle did not touch or investigate them, per the dispatch brief's item 6). The
27 newly-internal-turned-content collision-losers (unaffected by this fix, since Internal-tagged
rows never get a new bucket) are unchanged. **Total pinned-cause residual: 183** (156 + 27),
reconciled with `scripts/card15_reconcile.py` (`equals_total_this_run: True,
remaining_undisposed: 0`, `18,992` total).

## §16 — a unit moved out of a shape is not a unit closed

All 24 newly-landed units are `status: not-ingested` — enumerated, not engineered. No unit was
removed from any shape; the 5 renamed units are traced above with their physical location
confirmed unchanged.

## §15 — Product Identity

No record disposed this cycle was transcribed, ingested, or scored against
`ogl-pi-blacklist.md` — enumeration only. No PI-screening question arises at this layer.

## The 21/22 genuinely-unpinned rows — not investigated this cycle

The dispatch brief's item 6 asked either to pin these or report precisely what is unknown, and
NOT to fold them into the collision fix on assumption. They were not investigated this cycle —
the cause-pinning and the `*Choice` correction consumed the cycle's budget. Full list: the
`15-card-15-class-feature-residual-cause-pin.py` re-run above prints the current 22 by name
(all `advanced_players_guide`/`core_rulebook`/`adventurers_guide`/`ultimate_psionics` bloodline
or bloodline-adjacent rows). A plausible next check the prior memo already named and this cycle
did not perform: whether they collide with a row of a DIFFERENT `Kind::` in the same book, or a
cross-book `core_essentials`-style resolution — neither checked here.

## Next-cycle plan

1. **The 134 still-colliding, still-not-rescued residual (39 `*Choice`-shaped fallback groups +
   16 keyed-collision groups)** needs the SAME per-case hand review Decision 17 did for its own
   33 confirmed ids — not a smarter automatic heuristic (Decision 17's own text forbids that).
   Real work: for each collision, determine whether the colliding sibling is (a) a picker beside
   its own real feature (add to `DUPLICATE_CHOOSER_DISPLAY_NAME_UNIT_IDS`, correctly excluded)
   or (b) a genuinely distinct feature for a different class/archetype (rescue it, following this
   cycle's own disambiguation mechanism once a human/agent has actually looked at the row pair).
2. **The 22 genuinely-unpinned residual rows** — pin the cause or report precisely what remains
   unknown; do not fold into either of the above without evidence.
3. Card 15 reaches `complete` only when `total_kind_unenumerable_units` reaches 0 (unaffected by
   this cycle) and the `duplicate_identity` residual above is closed by class.
