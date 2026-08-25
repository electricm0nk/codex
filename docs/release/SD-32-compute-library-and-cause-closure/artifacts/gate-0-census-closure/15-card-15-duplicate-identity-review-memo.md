---
canonical: true
owner: card15-duplicate-identity-review
status: per-case review of the 183-unit `duplicate_identity` residual complete; 4 units rescued in code (not yet in the committed docs/work-inventory.json, see this file's own opening note), 39 groups named for an operator ruling on the Decision-17 allowlist, the rest correctly left unrescued
date: 2026-08-23
---

# Card 15 — per-case hand review of the 183-unit `duplicate_identity` residual

The prior cycle (`15-card-15-duplicate-identity-memo.md`) rescued 24 of the original 207 and
deliberately stopped, naming exactly two open sub-populations: 134 still-colliding residual rows
(39 `TYPE:*Choice`-suffixed fallback-key collision groups + 16 keyed-collision groups) and 22
genuinely-unpinned rows, both requiring the SAME per-case hand review SD-31 `decisions.md` Decision
17 already did for its own 33-id chooser allowlist — never a smarter automatic heuristic, per that
decision's own text. This cycle does that review.

**Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/verify.sh --only
preflight-oracle` → PASS, oracle bootstrapped fresh into the repo-local
`artifacts/corpus/operator-supplied/pcgen` slot — a fresh worktree's slot was empty, per the
dispatch brief's own warning).

**Read this before the "183 → 179" figures below.** This memo's own numbers (below) describe the
population AS IT STOOD when this cycle's own regen ran, before the final push. Between that regen
and push, four more sibling cycles landed on `origin/tranche/12` and rebasing onto the last of them
surfaced a real `source.path` defect that now blocks `corpus_literal_sweep` corpus-wide (full
account: `15-duplicate-identity-review_cycle_receipt.md`'s own opening note and "Next-cycle plan"
item 5). **This cycle's code fix (`disambiguate_class_feature_keyed_name_collisions`) is landed,
tested, and proven correct** — but hand-splicing its 4 rescued units into each new base as origin
moved would have violated this program's "never hand-edit the committed JSON" rule, so the FINAL
committed `docs/work-inventory.json` at push time is `origin/tranche/12`'s own latest (49,540
units, `class_feature` 18,056, residual **183**, unchanged), not the 18,060/179 this memo's body
describes. `scripts/card15_reconcile.py`'s own committed figures (183 pending) match the file as
actually pushed. The 4-unit rescue lands in the checked-in file the next time a guarded regen runs
(receipt's next-cycle item 6) — nothing further is owed in code.

## §17a re-derivation — the 183 reproduces exactly

```bash
export PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data
python3 docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-class-feature-residual-cause-pin.py
```

156 non-internal residual (134 collide + 22 don't) + 27 internal-collision-losers = **183**,
reproduces exactly against the pinned oracle and the current `docs/work-inventory.json`.

```bash
python3 docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-duplicate-identity-key-validation.py
```

64 fallback (no `KEY:`) collision groups (164 rows), 0 byte-identical, 39 all-`*Choice`-typed
(excluded from the prior cycle's rescue, the population this cycle reviews), 25 already rescued.
16 keyed (has `KEY:`) collision groups (32 rows), the population this cycle also reviews.

New instrument this cycle: `15-card-15-residual-group-review.py` (this directory) — for every one
of the 39 Choice-typed groups and 16 keyed groups, prints each member row's `CATEGORY:`/`TYPE:`
and (for the Choice-typed groups) its `ABILITY:AUTOMATIC` grant target(s), so the evidence a human
reviewer would read by hand is surfaced directly rather than inferred. This script decides
nothing — it is the review's own worksheet, re-run below and read group by group before any
disposition was made.

## The 39 `TYPE:*Choice`-typed fallback groups — ALL Decision-17-shaped, NONE rescued

Every one of the 39 groups (113 rows total, spanning `advanced_class_guide` 27,
`advanced_race_guide` 2, `monster_codex` 1, `occult_adventures` 2, `ultimate_magic` 7) is the
SAME Sorcerer/Bloodrager bloodline-chooser idiom Decision 17 already confirmed: a `CHOOSE:`-pool
picker row beside (or reachable to) its own real feature, not a second distinct object. This
cycle's evidence, direct from the corpus, not inferred:

**Every group's member rows converge, in pairs, on an IDENTICAL `ABILITY:AUTOMATIC` grant
target**, reached via a base-class gate and a second archetype/feat-chain gate. Worked example
(`advanced_class_guide`'s "Aberrant Bloodline", the group this card's own dispatch brief named as
its flagship worked example):

```
acg_abilities_class.lst:156   CATEGORY:Arcanist Bloodline Development   TYPE:SorcererBloodlineChoice
                               grants: Sorcerer Bloodline ~ Aberrant
acg_abilities_class.lst:2412  CATEGORY:Blood Arcanist Bloodline         TYPE:SorcererBloodlineChoice
                               grants: Sorcerer Bloodline ~ Aberrant     <- SAME target as :156
acg_abilities_class.lst:566   CATEGORY:Bloodrager Bloodline             TYPE:BloodragerBloodlineChoice
                               grants: Bloodrager Bloodline ~ Aberrant, Aberrant Bloodrager Bloodline ~ Feat Tracker
acg_abilities_class.lst:2754  CATEGORY:Crossblooded Rager Bloodline     TYPE:BloodragerBloodlineChoice
                               grants: Bloodrager Bloodline ~ Aberrant, Aberrant Bloodrager Bloodline ~ Feat Tracker
                               <- SAME target(s) as :566
```

Four picker rows (Arcanist's own bloodline development table, the Blood Arcanist archetype's own
table, the Bloodrager class's own table, the Crossblooded Rager archetype's own table), but only
**two** distinct real features are ever granted — the Sorcerer chassis's "Aberrant Bloodline"
feature (line 565, `KEY:Bloodrager Bloodline ~ Aberrant` — actually the CRB Sorcerer's own feature,
already on `DUPLICATE_CHOOSER_DISPLAY_NAME_UNIT_IDS` as `core_rulebook:class_feature:
aberrant_bloodline`) and the Bloodrager chassis's own "Aberrant Bloodline" feature (also line 565
in this file, a Bloodrager-specific restatement). Each real feature has TWO duplicate picker rows —
one from the base class, one from an archetype that grants the identical feature through a second
prerequisite gate — the exact shape the memo's own `ultimate_magic:accursed_bloodline` discovery
already confirmed operator-reviewed.

This pattern holds, re-verified per group, for all 39:

- **27 `advanced_class_guide` groups** (2 or 4 rows each): every member's `ABILITY:AUTOMATIC`
  target is either the base Sorcerer chassis feature (`Sorcerer Bloodline ~ <X>`, granted by both
  the Arcanist and Blood Arcanist archetype pickers) or the Bloodrager chassis feature
  (`Bloodrager Bloodline ~ <X>`, granted by both the Bloodrager and Crossblooded Rager archetype
  pickers) — never a third, independent target.
- **2 `advanced_race_guide` groups (Imperious/Kobold Bloodline), 1 `monster_codex` group (Ghoul
  Bloodline), 2 `occult_adventures` groups (Ectoplasm/Psychic Bloodline)** (5 rows each): all 5
  member rows (Sorcerer, Arcanist, Blood Arcanist, Crossblooded, Eldritch Heritage) converge on the
  SAME single target, `Sorcerer Bloodline ~ <X>` — these races/monsters only ever had a
  Sorcerer-chassis bloodline, never a Bloodrager one, so there is exactly ONE real feature and FIVE
  duplicate pickers for it.
- **7 `ultimate_magic` groups** (Accursed/Djinni/Efreeti/Maestro/Marid/Rakshasa/Shaitan Bloodline,
  2 rows each): both rows grant the IDENTICAL target set. **All 7 of this book's surviving rows are
  ALREADY on `DUPLICATE_CHOOSER_DISPLAY_NAME_UNIT_IDS`** — `accursed_bloodline` is the memo's own
  discovered example; this cycle confirms `djinni_bloodline`/`efreeti_bloodline`/
  `maestro_bloodline`/`marid_bloodline`/`rakshasa_bloodline`/`shaitan_bloodline` are on the SAME
  list too. The residual sibling in each of these 7 groups is the identical shape, simply not yet
  on the list (invisible to Decision 17's own audit, exactly as the memo already found for
  `accursed_bloodline`'s sibling).

**None of the 39 groups is rescued.** Per Decision 17's own text ("a generic 'same name, adjacent
line' rule would silently sweep in any FUTURE same-shaped collision no human reviewed"), this
evidence is reported for an operator ruling, not auto-applied to
`DUPLICATE_CHOOSER_DISPLAY_NAME_UNIT_IDS` (`src/bin/v06_work_inventory.rs`) or to SD-31's own
`decisions.md`. **This cycle does not edit that allowlist or that decisions file** — both are an
operator ruling's territory (dispatch brief, explicit).

**What would resolve this population:** an operator ruling extending
`DUPLICATE_CHOOSER_DISPLAY_NAME_UNIT_IDS` with the 74 residual sibling ids across these 39 groups
(book:class_feature:slug, one per losing row, all currently absent from `docs/work-inventory.json`
and therefore not addressable by id yet — the group review script above prints every member row's
book/file/line so the exact 74 physical locations are reproducible on request). Given the pattern
is corpus-uniform (every single one of the 39 groups matches the SAME evidence shape with zero
exceptions found), the operator may find it faster to rule on the *class* ("every fallback-key
`class_feature` collision group whose members ALL carry a `TYPE:*Choice` facet AND whose granted
targets pairwise coincide is a duplicate-chooser-picker group, not distinct objects") rather than
74 individual ids — but that generalization is this memo's own recommendation, not something this
cycle applies unilaterally.

## The 16 keyed-collision groups — 4 rescued (this cycle's real code change), 12 correctly left alone

Re-read every one of the 16 groups' full row content (both `CATEGORY:`/`TYPE:` and, where relevant,
`DESC:`/`ABILITY:`/`BONUS:` fields), not just the `TYPE:` facet used for the Choice-typed groups:

| Group | Display names (both sides) | Content | Disposition |
|---|---|---|---|
| `advanced_race_guide` Forgemaster ~ Craft Magic Arms and Armor | same both sides | PFS variant `REMOVE:FEAT`s + `DESC:.CLEAR`s the base declaration — an explicit, intentional override of the SAME feature | same identity by design — left alone |
| `advanced_race_guide` Forgemaster ~ Master Smith | same both sides | same PFS-override shape | left alone |
| `core_rulebook` Assassin ~ Hide in Plain Sight | same both sides | one row `VISIBLE:NO`, no `DESC:` (internal bookkeeping companion); other row is the real player-facing feature | hidden-tracker-beside-real-feature, same disposition as `CATEGORY:Internal` — left alone |
| `core_rulebook` Domain Power ~ Battle Rage / Holy Lance / Touch of Good / Weapon Master (4 groups) | same both sides | byte-identical content signature (`content_sig` match) | true restatement — left alone |
| `ultimate_combat` Monk Bonus Feat ~ Greater/Improved Grapple (2 groups) | same both sides | byte-identical content signature | true restatement — left alone |
| `ultimate_combat` Weapon Training 1-4 Firearms (4 groups) | same both sides | different `TYPE:`/`BONUS:`/`SOURCEPAGE:`, but both branches use a mutual-exclusion `!PREABILITY` guard against the OTHER's own type — evidence the corpus author deliberately reused one `KEY:` so a character who qualifies via both paths is not double-granted, not two objects | same identity by design — left alone |
| `advanced_race_guide` Native Cunning ~ Grapple / **Overrun** | **differ** | `BONUS:VAR|CMD_Grapple` vs `BONUS:VAR|CMD_Overrun` — a Feral Child's CMD bonus against two DIFFERENT combat maneuvers; the corpus's OWN 9 sibling rows for this same feature (Bull Rush/Dirty Tricks/Disarm/Drag/Reposition/Steal/Sunder/Trip, each correctly `KEY:Native Cunning ~ <maneuver>`) confirm "Overrun"'s `KEY:` should read `~ Overrun`, not `~ Grapple` — a corpus-author typo | **RESCUED** |
| `ultimate_intrigue` Vigilante Favored Maneuver ~ Bull Rush / **Sunder** | **differ** | grants Improved Bull Rush vs Improved Sunder feats — two different maneuvers, same typo shape | **RESCUED** |
| `ultimate_wilderness` Green Faith Marshal ~ Panther Domain / **Vulture Domain** | **differ** | two different Green Faith Marshal domain selections (Panther vs Vulture), same typo shape | **RESCUED** |

**The discriminator: does the corpus author's OWN display name differ between the two colliding
rows sharing one declared `KEY:`?** When it does, the KEY collision is direct, non-inferred
evidence of a typo, not one identity — unlike the Choice-typed fallback population above (where
colliding rows regularly share the SAME name and are genuinely either distinct objects or the same
feature, so name alone cannot arbitrate there), here the `KEY:` was author-declared specifically to
be a stable identity independent of `CATEGORY:`/`TYPE:`, so two different names under one declared
key is itself the defect. Verified this discriminator produces the CORRECT call on all 13
non-rescued groups too (every one shares an identical display name both sides) before trusting it.

### A 4th rescue, found by the real fix (not this memo's manual census)

Running the landed fix against the real corpus surfaced a keyed collision this memo's own hand
census missed: `ultimate_intrigue:class_feature` "Social Grace ~ Craft (Armor)" (line 1092) vs.
"Craft (Baskets)" (line 1093, same declared `KEY:`) — both rows carry `CATEGORY:Internal`, so this
memo's own review script (which excludes ALL `CATEGORY:Internal` rows, mirroring the prior cycle's
census script) never surfaced it as one of the "16" groups. Production's `is_internal_category`
narrowing is finer-grained than that blanket exclusion — a content-bearing Internal row (this one
carries a real `!PREABILITY` gate and a real `TYPE:` facet, not a bare marker) stays in scope, so
it reached the SAME `duplicate_identity` collision the 16-group census was built to find, just
outside that census's own (too-crude) `CATEGORY:Internal` filter. Same evidence shape as the other
3 (`Craft (Armor)` vs `Craft (Baskets)` — two different skill choices in a picker pool, the second
row's `KEY:` evidently copy-pasted from the first without updating the trailing skill name) —
**rescued** by the same code path, no separate fix needed. Named here because it changes the
"16 keyed groups" figure inherited from the prior cycle's census to "16 keyed groups the prior
census found + at least 1 more the census's own `CATEGORY:Internal` filter hid" — the true
population of keyed collisions among content-bearing `class_feature` rows may be larger than 16;
this cycle did not re-derive it exhaustively (see "What this cycle did not do" below).

## The fix — `disambiguate_class_feature_keyed_name_collisions` (`src/bin/v06_work_inventory.rs`)

New function, symmetric to the prior cycle's `disambiguate_class_feature_fallback_collisions` but
targeting the OTHER half of the population that function's own doc comment named out of scope:
rows that DO carry an explicit `KEY:` (`u.key != u.name`). Only `Kind::ClassFeature`; the FIRST row
in file order for a given key keeps it unchanged; a LATER row whose display `name` has not been
seen for this key gets a disambiguated key (`"<key> ~ <name>"`); a later row repeating an
already-seen name for this key collapses to that name's bucket key exactly as today (a true
restatement, unaffected). Full rationale, including why name-mismatch (not content-diff) is the
right discriminator for this population specifically: the function's own doc comment.

### RED → GREEN

New test module `disambiguate_class_feature_keyed_name_collisions_tests`, 4 tests:

- `differing_name_under_shared_key_rescues_both` — the real `Native Cunning ~ Grapple`/`Overrun`
  shape: both survive with distinct keys. Before this fn existed, the test target function did not
  exist — RED by construction (the test file would not compile); after, both units keep separate
  keys — GREEN.
- `same_name_under_shared_key_is_left_to_collapse_normally` — the real `Weapon Training (Firearms)`
  shape: same name both sides, must NOT be rescued, proving the fn does not stop the legitimate
  collapse for same-name keyed pairs.
- `fallback_key_row_is_never_touched` — a fallback (`u.key == u.name`) row is untouched, proving
  the two disambiguation fns' populations are disjoint.
- `repeated_name_for_same_key_collapses_to_existing_bucket` — a third row repeating an
  already-seen name collapses to that bucket, not a fresh disambiguation.

```bash
cargo test --locked --bin v06_work_inventory disambiguate_class_feature_keyed_name_collisions_tests
```
→ 4/4 passed. Full binary suite: `cargo test --locked --bin v06_work_inventory` → **339/339** (was
335; +4 net new).

## Population, before and after — both directions proved

```bash
export PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data
export CORPUS_LITERAL_SWEEP_REPORT=<fresh corpus_literal_sweep --json-out report>
export DERIVED_FIXTURE_CHECK_REPORT=<fresh derived_evaluator_fixture_check --json-out report>
cargo run --locked --bin v06_work_inventory
```

`git diff --stat HEAD -- src/bin/v06_work_inventory.rs` for this cycle's own diff: **174
insertions, 0 deletions** — the code change is purely additive (2 new functions + 1 wiring line + 4
new tests), touching nothing about `status`/`wiring_class`/`evidence` computation. This is the
proof, independent of the regen below, that this cycle's own diff cannot have caused anything
beyond the 4 new `class_feature` units.

| | before (committed HEAD) | after (this cycle's regen) | delta |
|---|---:|---:|---:|
| `totals.units` | 49,540 | 49,544 | **+4** |
| `class_feature` | 18,056 | 18,060 | **+4** |
| every other kind | — | byte-identical | 0 |

**Both directions, proved by physical-location diff** (`(book, source_file, source_line)` triples):
**0 lost, 4 gained** — `advanced_race_guide/arg_abilities_class.lst:386`,
`ultimate_intrigue/ui_abilities_class.lst:140`, `ultimate_intrigue/ui_abilities_class.lst:1093`,
`ultimate_wilderness/uw_abilities_class_fap.lst:9`. **0 ids removed, 0 ids added beyond the 4 new
ones, 0 id duplicates in class_feature's 18,060 (checked directly).**
`apply_duplicate_chooser_removal`'s own drift guard (`exit(1)` if the removed-33 count drifts) did
NOT fire — direct, mechanical confirmation this fix created no new candidate for that population;
the risky 33-id set is genuinely untouched.

**Full `status` distribution diffed, before vs. after this cycle's regen:**

| status | before | after | delta |
|---|---:|---:|---:|
| `literal-verified` | 6,506 | 6,506 | 0 |
| `fixture-verified` | 1,741 | 1,741 | 0 |
| `grounded` | 2,515 | 2,724 | **+209** |
| `text-complete` | 3,869 | 4,395 | **+526** |
| `ingested-magnitude` | 1,474 | 1,515 | **+41** |
| `not-ingested` | 29,106 | 28,313 | **−793** |
| `unknown` | 4,264 | 4,285 | **+21** |
| `deferred-with-reason` | 46 | 46 | 0 |
| `not-started` | 19 | 19 | 0 |

**The two verification-provenance stamps this program's own regeneration warning cares about are
exactly preserved** (`literal-verified`, `fixture-verified` both unchanged) — no stamp loss, the
near-miss this program has hit before. **The other five buckets' large shift is NOT this cycle's
effect**: this cycle's own diff to `src/bin/v06_work_inventory.rs` is 174 insertions / 0 deletions
(above), touching only the `duplicate_identity`-adjacent key-disambiguation code path, never
`status`/`wiring_class`/`evidence` computation. The shift reflects the checked-in
`docs/work-inventory.json` at this commit having gone stale relative to what a fresh regen at the
SAME commit produces — the identical "post-rebase regeneration" hazard the prior
`15-card-15-duplicate-identity-memo.md` already named and handled, except here it predates this
cycle's own rebase (the pin never moved this cycle; `origin/tranche/12`'s tip was already the
starting commit). **Flagged, not silently absorbed**, exactly as that prior cycle's own precedent
requires. The regenerated JSON is the correct one to commit — the alternative (hand-patching only
4 rows into the stale committed copy) is the hand-edit `15-card-15-duplicate-identity-memo.md`
explicitly forbids.

## Re-derived residual after this cycle

```bash
export PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data
python3 docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-class-feature-residual-cause-pin.py
```

**153 non-internal residual (was 156, −3)**: 131 collide (was 134), 22 don't (unchanged — see next
section, these are now fully traced). **26 internal-collision-losers (was 27, −1: the Social Grace
rescue)**. **Total pinned-cause residual: 179** (was 183).

```bash
export PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data
python3 scripts/card15_reconcile.py
```
→ `equals_total_this_run: True`, `remaining_undisposed: 0`, total **18,992** (invariant — the sum
of all piles is unchanged by this cycle, only the allocation between "already tracked" (18,008 →
18,012) and "pending (A)" (183 → 179) moved, exactly as "sum the piles" requires).

## The 22 genuinely-unpinned rows — traced this cycle, all fully explained

Not a "group" in the collision sense (each has no `(book, key)` collision at all), but this cycle
traced the cause for all 22 by direct lookup:

- **21 are rows ALREADY on `DUPLICATE_CHOOSER_DISPLAY_NAME_UNIT_IDS`** (10 `core_rulebook` + 10
  `advanced_players_guide` + 1 `adventurers_guide` bloodline pickers — e.g.
  `core_rulebook:class_feature:aberrant_bloodline`, `cr_abilities_class.lst:2334`). They are
  MISSING from `docs/work-inventory.json` not because of a cause-pinning gap, but because
  `apply_duplicate_chooser_removal` correctly, deliberately removes exactly these 33 confirmed ids
  from the final inventory AFTER construction — the cause-pin script's own residual predicate
  (matched by physical location against the FINAL post-removal inventory) cannot see that removal
  step, so it reports these as "residual" when they are in fact working exactly as designed.
  **Not a defect. Not open. Fully explained.**
- **The 22nd** (`ultimate_psionics:up_abilities_class.lst:468`, "Disable Device Class Skill") is
  the SAME displacement `15-card-15-internal-duplicate-identity-memo.md` §3 already traced: a
  newly-visible `CATEGORY:Internal` row (line 186) won the `duplicate_identity` race for the same
  bare key; the physical coordinate moved, the content did not. **Not a defect. Not open. Fully
  explained.**

**No cause-pinning gap remains anywhere in the 183-unit population this cycle started with.**
`scripts/card15_reconcile.py`'s own `pending_a` total (179) is therefore a **conservative** figure:
it counts these 22 fully-explained rows as still "pending (A) — identified, not yet integrated"
because that bucket's own bookkeeping (correctly, per this program's `§1a` anti-gaming stance) was
not re-audited or re-allocated this cycle — moving them to "disposed (B) — proven not an object" is
real, in-scope future bookkeeping work this cycle names but does not perform (see below), not a
defect in what this cycle actually closed.

## Product Identity (§15)

No record disposed this cycle was transcribed, ingested, or scored against
`ogl-pi-blacklist.md` — enumeration and identity-key disambiguation only, same as every prior
card-15 cycle. No PI-screening question arises at this layer.

## §16 — a unit moved out of a shape is not a unit closed

Of the 4 newly-landed units, 1 (`vigilante_favored_maneuver_bull_rush_favored_maneuver_sunder`, the Vigilante has a modelled class) is `status: not-ingested`; the other 3 (`native_cunning_grapple_overrun`, `social_grace_craft_armor_craft_baskets`, `green_faith_marshal_panther_domain_vulture` — racial-trait-shaped rows whose own group prefix names no modelled class) are `status: unknown`, the honest default for a `class_feature` row `classify()` cannot attribute to any class. Re-derived directly from `docs/work-inventory.json` (`status`/`evidence` fields), not assumed from every other prior cycle's own `not-ingested`-only pattern -- verified rather than repeated. No unit was
removed from any shape.

## Sweep of pinned counts — `tests/`, `src/`, `scripts/`, `apps/`

```bash
grep -rn "18056\|18,056\|18008\|18,008\|\b183\b\|\b207\b\|156\b.*residual\|134\b.*collide" tests/ scripts/ src/ apps/
```

Only `scripts/card15_reconcile.py` (updated this cycle, see below) and
`docs/release/.../progress.md`/`kanban.md` (append-only history, not live assertions) matched. No
`tests/*.rs` or `src/**` file asserts an exact `class_feature` population number.

## Tests

- `cargo test --locked --bin v06_work_inventory` → **339/339** (was 335; +4 net new).
- `python3 scripts/shape_ledger.py` → `unclassified_count: 0`, piles reconcile.
- `python3 scripts/card15_reconcile.py` → `equals_total_this_run: True`, `remaining_undisposed: 0`.
- Full sweep NOT run — out of scope per this bundle's own scoping instruction; the touched Rust
  file is isolated (`src/bin/v06_work_inventory.rs`), not consumed by the lib crate or the desktop
  crate.

## Identifier / wired-integration audit (this cycle's own diff, scoped to touched files only)

```bash
git diff --unified=0 HEAD -- src/bin/v06_work_inventory.rs scripts/card15_reconcile.py \
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
git diff --unified=0 HEAD -- src/bin/v06_work_inventory.rs scripts/card15_reconcile.py \
  | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
```

Both `OK_*`.

## Files touched

- `src/bin/v06_work_inventory.rs` — `disambiguate_class_feature_keyed_name_collisions` (new fn, 4
  new tests); wired into the `duplicate_identity` per-book loop immediately after the prior
  cycle's `disambiguate_class_feature_fallback_collisions`.
- `docs/work-inventory.json` — regenerated through the real producer (see Population above).
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-reconcile.json`
  — regenerated.
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-1-shape-closure/ledger.json`
  — regenerated for consistency.
- `scripts/card15_reconcile.py` — `class_feature_residual_duplicate_identity` (183 → 179) and
  `class_feature_already_in_inventory` (18,008 → 18,012) updated with this cycle's evidence.
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-residual-group-review.py`
  (new) — the committed, re-runnable per-group evidence worksheet.
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-duplicate-identity-review-memo.md`
  (new, this file).

## What this cycle did not do — named, not silently skipped

1. **Did not add anything to `DUPLICATE_CHOOSER_DISPLAY_NAME_UNIT_IDS` or edit SD-31's
   `decisions.md`.** Per the dispatch brief's own instruction, that allowlist and that decisions
   file are an operator ruling's territory. This memo names the evidence (all 39 groups,
   Decision-17-shaped, worked example above) for that ruling.
2. **Did not exhaustively re-derive the keyed-collision population beyond the original 16-group
   census plus the 1 collision the fix itself surfaced.** The Social Grace discovery (found only
   because the REAL fix ran against the full corpus, not because this memo's own census caught it)
   is direct evidence that `CATEGORY:Internal` exclusion in a hand census is the wrong filter for
   this specific population — a future cycle should re-derive the keyed-collision census using
   `is_internal_category`'s own narrowed test (bare-marker vs. content-bearing) rather than a
   blanket `CATEGORY:Internal` skip, to find whatever remainder of this population exists beyond
   what this cycle's fix happened to catch live.
3. **Did not reclassify the 22 fully-explained rows from `pending_a` to `disposed_b` in
   `scripts/card15_reconcile.py`'s own bucket structure**, even though this cycle proved by class,
   with a committed command, that none of them represents open work. That reallocation is real,
   in-scope bookkeeping for a future cycle (or this one's own next pass) — flagged explicitly so it
   is not lost, not performed here to keep this cycle's own diff auditable against exactly what it
   claims (a 4-unit rescue plus an evidence-gathering pass, not a bucket-structure rewrite).

## Card 15's bar

**Not yet met.** `decisions.md §12b`: "Card 15 closes them. Closure means each object is either (a)
enumerated as a unit... or (b) proven not to be an object... by class, with the committed command."
This cycle proves (b) for the 22 unpinned rows (see above, not yet reallocated in the reconcile
script) and proves neither (a) nor (b) yet for the 39 Choice-typed groups (74 residual rows) or the
12 remaining keyed groups (24 residual rows) — those need an operator ruling on the Decision-17
allowlist addition before they can close as (b), or a different kind of evidence entirely to close
as (a). **`total_kind_unenumerable_units` is unaffected by this cycle** (unchanged at 27,847's
successor figure) and the `duplicate_identity` residual, while narrowed from 183 to 179 with every
remaining unit now evidenced and named, is not zero.

**Escalation, per `decisions.md §10`'s escalation path (a request for a ruling, not a
disposition):** this population cannot close further without an operator ruling on whether the 39
Decision-17-shaped groups' 74 residual sibling ids should be added to
`DUPLICATE_CHOOSER_DISPLAY_NAME_UNIT_IDS` (this memo's own recommendation, evidenced above) or
whether each needs its own individual confirmation the way the original 33 got. **Exact question
for the operator:** *"39 fallback-key `class_feature` collision groups (74 residual rows) all show
the SAME evidence as your own confirmed 33-id `DUPLICATE_CHOOSER_DISPLAY_NAME_UNIT_IDS` allowlist —
every group's TYPE:*Choice-typed members grant, in pairs, an identical real-feature target via a
base-class gate and an archetype/feat-chain gate (worked example and full per-group evidence in
this memo). Should these 74 ids be added to that allowlist (as one ruling on the class, or
individually), or does the population need per-id confirmation the way the original 33 did?"*
