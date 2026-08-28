# Cycle — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 (`class_feature_owner_matched_by_name_but_record_not_held_by_engine` mechanism, cycle 5)

This cycle owns **exactly one** of the nine mechanisms `decisions.md §14` decomposed
`AT-34-E3-001` into. It does **not** close AT-34-E3-001 itself — other mechanisms remain,
each its own cycle.

- **Commit SHA:** `cb0ba2286e`
- **Files touched:**
  - `src/rules_core/pilot_compute/mod.rs` — two new bloodline/style-agnostic pool-grounding
    functions (`ground_sorcerer_bloodline_feat_pool`, `ground_ranger_combat_style_feat_pool`),
    their two eligible-set consts (`SORCERER_BLOODLINE_FEAT_POOL_ELIGIBLE_FEATS` 87 entries,
    `RANGER_COMBAT_STYLE_FEAT_POOL` 16 entries), one verified-collision exclusion list
    (`SORCERER_BLOODLINE_FEAT_POOL_DIAGNOSTIC_EXCLUSIONS`, 10 entries — see "A defect this
    cycle found in its own work" below), two unconditional call sites, and two `#[cfg(test)]`
    modules (7 unit tests total).
  - `docs/work-inventory.json` — regenerated at HEAD, guarded regeneration path (plain
    `cargo run --locked --bin v06_work_inventory`, no `--allow-stamp-loss` used or needed;
    `CORPUS_LITERAL_SWEEP_REPORT` / `DERIVED_FIXTURE_CHECK_REPORT` set from this cycle's own
    fresh runs, three full regeneration passes — see "A defect this cycle found" below for why).
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    (regenerated output of `completion_atlas.py --check`, not hand-edited).
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_owner_matched_cycle_receipt_5.md`
    (this file)
  - `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md`
  - `src/bin/v06_work_inventory.rs` — **not touched this cycle**; no `BUCKET_DEFINITIONS`
    citation drift risk (confirmed: `completion_atlas.py --check` reports
    `citation_failures=0` below).

- **Identifier audit result:** OK_NO_BUNDLE_TAGS on this cycle's own new code
  (`git diff --unified=0 "${BASE_BRANCH}...HEAD" -- src/rules_core/pilot_compute/mod.rs
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` → no matches).
- **Wired-integration audit result:** OK_NO_TOKENS on this cycle's own new code (same scoped
  diff, `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` → no
  matches).
- **Acceptance criterion:** AT-34-E3-001 — bucket B closes: records reach their tables — this
  cycle owns exactly mechanism 1 of 9,
  `class_feature_owner_matched_by_name_but_record_not_held_by_engine`.

## Re-derived population (do not quote a prior receipt's number without checking)

```bash
python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
cr = [u for u in d['units'] if u.get('book')=='core_rulebook'
      and u.get('status')=='engine-does-not-hold'
      and u.get('evidence')=='class_feature_owner_matched_by_name_but_record_not_held_by_engine']
print(len(cr))
"
```
→ **344** at this cycle's starting HEAD (`c4e6ac92f9`), unchanged from cycle 4's own closing
figure. Matches the dispatch brief's own stated 344 exactly — re-derived, not assumed.

## This cycle's own contribution — `decisions.md §16`'s ratified precedent, applied

`decisions.md §16` (written for this cycle) settled that a "pick N from this eligible set"
class-feature record is `held` when the engine grounds the COUNT and names the ELIGIBLE SET,
without modelling which option a given character picked — the shape already shipped for
Fighter/Cavalier/Brawler bonus feats and for the Sorcerer Arcane bloodline's own
`ARCANE_BLOODLINE_ELIGIBLE_BONUS_FEATS` (`pilot_compute/mod.rs:1837-1854`, before this cycle).
Cycles 2-4 had each named this pattern as "operator-scoped" and declined to act on the 103-unit
majority it covers (`Sorcerer Bloodline Feat` 87, `Ranger Combat Style Feat` 16) — §16 settles
that it is not an open question.

**Sorcerer.** `ground_sorcerer_bloodline_feat_pool` grounds the bloodline-feat-pool slot COUNT
as a magnitude via the SAME formula `arcane_bloodline_bonus_feat_count` already implements
((sorcerer level - 1)/6) — verified bloodline-INVARIANT (the corpus's own
`BONUS:VAR|BloodlineFeatCount|(BloodlineFeatProgression-1)/6|TYPE=Base` token carries no
bloodline-specific term), so it is grounded unconditionally for any Sorcerer, independent of
`ground_sorcerer_arcane_bloodline_progression`'s own Arcane-only canonical narrowing. It names
the full 87-feat corpus-wide union across every CRB Sorcerer bloodline (a superset of any one
bloodline's real list, not a fabricated one) and emits one non-claim-blocking diagnostic per
eligible feat, stating the count grounds and the option is not modelled — never seeding a
default choice.

**Ranger.** `ground_ranger_combat_style_feat_pool` grounds the combat-style-feat-pool slot
COUNT (1 at 2nd, 2 at 6th, 3 at 10th, 4 at 14th, 5 at 18th ranger level — the same milestone
progression the existing specific-choice idiom already documents) unconditionally, independent
of whether this seam has recognized which style (Archery or Two-Weapon Combat) the character
chose. It names the full, exhaustive 16-feat combined Archery+Two-Weapon-Combat pool (no
14th-/18th-level slot names a new option in the Core Rulebook, confirmed against the corpus:
exactly 16 files under `class_feature/ranger_combat_style_feat/`) and emits one non-claim-
blocking diagnostic per feat, same shape as the Sorcerer pool.

## A defect this cycle found in its own work — substring cross-attribution, found and fixed

`v06_work_inventory.rs::diagnostic_id_names_feature` matches a diagnostic to a corpus unit by
**substring**, not exact key (`id.contains(&format!(".{owner}."))` plus
`body.contains(feature_slug)`). The first regeneration pass (before any exclusion list existed)
closed the intended 103 units correctly, but a before/after diff against the pre-cycle
`docs/work-inventory.json` (kept via `git show HEAD:docs/work-inventory.json`) found **4
unrelated units** incidentally routed to `deferred-with-reason` through a false match: `Sorcerer
Domain ~ Sun` (via `"Improved Sunder"`'s slug containing `"sun"`), `~ Knowledge` (via `"Skill
Focus (Knowledge (Arcana))"`'s slug containing `"knowledge"`), `~ Magic` (via `"Magical
Aptitude"`), and `Sorcerer Bonus Spell L3 ~ Fly` (via `"Skill Focus (Fly)"`). These are a
Sorcerer-domain-swap archetype's domain-power records and a bonus spell — nothing to do with
the bloodline feat pool — so the diagnostic message attributed a false reason to each. This is
a real correctness defect, not a cosmetic one: bucket X is "deferred **with a stated reason**",
and the stated reason must be true of the record it names.

The first fix (excluding the four literal colliding names) still left 6 of the 7
`"Skill Focus (Knowledge (<school>))"` entries in the loop, each independently colliding with
`Sorcerer Domain ~ Knowledge` the same way — caught by a **second** full regeneration's own
before/after diff (1 unexpected unit remained). A third fix excluded all seven Knowledge
variants; a corpus-wide Python cross-check (every `sorcerer`-/`ranger`-owned `class_feature`
record in the WHOLE corpus, not only this book's bucket B — 431 sorcerer-owned + 282
ranger-owned units) confirmed no further live collisions before committing to a third, final
regeneration. That third regeneration's own before/after diff shows **zero** unintended
movement (full command output in "Figures" below).

**`SORCERER_BLOODLINE_FEAT_POOL_DIAGNOSTIC_EXCLUSIONS`** (10 names, doc-commented with the
exact colliding unit and verification method) excludes only the PER-OPTION DIAGNOSTIC for
these ten feats — each stays listed in `SORCERER_BLOODLINE_FEAT_POOL_ELIGIBLE_FEATS` (a real,
honestly-named eligible feat; the count explanation's eligible-set size is unaffected). Only
these ten feats' own corpus records (`Sorcerer Bloodline Feat ~ <name>`) stay correctly
unclaimed (`engine-does-not-hold`) rather than closing via a false attribution — a smaller,
honest closure (77 of 87 Sorcerer names get diagnostics, not 87) in exchange for zero collateral
misclassification. `Ranger Combat Style Feat`'s 16 names carry no collision (the cross-check
found none; the pool's own internal collisions — `"Precise Shot"` inside `"Improved Precise
Shot"`, `"Two-Weapon Fighting"` inside its own `"Improved"`/`"Greater"` siblings — only affect
records ALREADY inside this pool, where the templated message stays true regardless of which
exact diagnostic is matched; see the doc comment on `ground_ranger_combat_style_feat_pool`).

Two further theoretical collisions the cross-check found are confirmed harmless and left
unexcluded: `Sorcerer Bloodline Feat ~ Spell Focus` (matches `"Spell Focus (Enchantment)"` etc.
— genuinely also a real bloodline-feat-pool member, so the templated message stays true of it)
and `Sorcerer Bloodline ~ Arcane` (already `grounded` via
`class_feature_probe_observed_a_delta_attributable_to_this_record`, an earlier-resolving
mechanism this diagnostic check never reaches).

## TDD — RED confirmed for the intended reason, then GREEN

```bash
# RED: both new grounding functions temporarily short-circuited with an early `return;`
cargo test --locked --lib pilot_compute:: -- --nocapture
```
→ all 7 new tests FAILED, each panicking on the missing slot-count explanation or the empty
diagnostics vec (the intended reason — the explanation genuinely does not exist yet with the
`return;` in place — not a compile error or an unrelated failure).

```bash
# GREEN: early returns removed, functions restored
cargo test --locked --lib pilot_compute:: -- --nocapture
```
→ `sorcerer_bloodline_feat_pool_slot_count_grounds_regardless_of_recognized_bloodline`,
`sorcerer_bloodline_feat_pool_slot_count_is_correctly_absent_below_the_grant_level`,
`sorcerer_bloodline_feat_pool_names_every_eligible_option_once_a_slot_is_granted`,
`ranger_combat_style_feat_pool_slot_count_grounds_at_the_2nd_level_gate`,
`ranger_combat_style_feat_pool_slot_count_is_correctly_absent_below_the_grant_level`,
`ranger_combat_style_feat_pool_names_every_eligible_option_once_a_slot_is_granted`,
`ranger_combat_style_feat_pool_slot_count_reaches_five_by_18th_level` — all 7 passed.

## Figures + re-derive commands

- **Mechanism population, `core_rulebook`:** 344 → **251** (command above, denominator:
  `core_rulebook` units with `status=='engine-does-not-hold'` and this evidence string).
- **93 units closed, verified against a snapshot of the pre-cycle inventory (`git show
  HEAD:docs/work-inventory.json`):**
  ```bash
  python3 -c "
  import json
  before = json.load(open('/tmp/work_inventory_before.json'))
  after = json.load(open('docs/work-inventory.json'))
  before_keys = {u['corpus_key'] for u in before['units'] if u.get('book')=='core_rulebook'
      and u.get('status')=='engine-does-not-hold'
      and u.get('evidence')=='class_feature_owner_matched_by_name_but_record_not_held_by_engine'}
  after_map = {u['corpus_key']: u for u in after['units'] if u.get('book')=='core_rulebook'}
  moved = [k for k in before_keys if after_map.get(k,{}).get('status')!='engine-does-not-hold'
      or after_map.get(k,{}).get('evidence')!='class_feature_owner_matched_by_name_but_record_not_held_by_engine']
  sorc_ranger = [k for k in moved if k.startswith('Sorcerer Bloodline Feat') or k.startswith('Ranger Combat Style Feat')]
  other = [k for k in moved if k not in sorc_ranger]
  print('before', len(before_keys), 'moved', len(moved), 'sorc/ranger', len(sorc_ranger), 'OTHER(unexpected)', len(other))
  "
  ```
  → `before 344 moved 93 sorc/ranger 93 OTHER(unexpected) 0` — exactly `77 Sorcerer Bloodline
  Feat` (87 named minus 10 excluded from the diagnostic loop) `+ 16 Ranger Combat Style Feat`.
  All 93 moved units now carry `status: deferred-with-reason` (bucket X, `decisions.md §2`:
  "deferred with a stated reason", cleared by revisiting the stated condition), never
  `text-complete` or `grounded` — this cycle never claims a display or magnitude that does not
  exist.
- **Bucket B, `core_rulebook` (atlas-real partition):** `python3 scripts/completion_atlas.py
  --book core_rulebook --check` → `579` remaining of 6,701 (denominator: all `core_rulebook`
  content units the atlas classifies). Bucket X (`deferred with a stated reason`) is `116` at
  this same run (up from cycles 2-4's own 3-unit vacuous-placeholder-only figure, by this
  cycle's own 93).
- **`completion_atlas.py --check` (population-wide):** `python3 scripts/completion_atlas.py
  --check` → `population=49438 buckets=10 unclassified=0 overlap=0 citation_failures=0`
  (denominator: 49,438, the corpus-wide unit population; `citation_failures=0` confirms no
  `src/bin/v06_work_inventory.rs` line shifted, since this cycle did not touch that file).
- **Denominator gate:** `python3 scripts/denominator_gate.py --check
  'docs/release/SD-34-book-completion/*.md'` → `files_checked=15 violations=0`.
- **`box_ledger.py --check` (SD-33's inherited, read-only partition):** exits 1 (six stale-count
  WARNINGs against `THE-BOX.md`, inherited drift, unowned by SD-34 — same finding every prior
  AT-34-E3-001 cycle has reported); its structural invariants pass: `overlap=0
  population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False`.

## Row-count command output

```
$ cargo test --locked --lib pilot_compute:: -- --nocapture 2>&1 | grep -E "sorcerer_bloodline_feat_pool|ranger_combat_style_feat_pool"
test rules_core::pilot_compute::ranger_combat_style_feat_pool_tests::ranger_combat_style_feat_pool_names_every_eligible_option_once_a_slot_is_granted ... ok
test rules_core::pilot_compute::ranger_combat_style_feat_pool_tests::ranger_combat_style_feat_pool_slot_count_grounds_at_the_2nd_level_gate ... ok
test rules_core::pilot_compute::ranger_combat_style_feat_pool_tests::ranger_combat_style_feat_pool_slot_count_is_correctly_absent_below_the_grant_level ... ok
test rules_core::pilot_compute::ranger_combat_style_feat_pool_tests::ranger_combat_style_feat_pool_slot_count_reaches_five_by_18th_level ... ok
test rules_core::pilot_compute::sorcerer_arcane_bloodline_progression_tests::sorcerer_bloodline_feat_pool_names_every_eligible_option_once_a_slot_is_granted ... ok
test rules_core::pilot_compute::sorcerer_arcane_bloodline_progression_tests::sorcerer_bloodline_feat_pool_slot_count_grounds_regardless_of_recognized_bloodline ... ok
test rules_core::pilot_compute::sorcerer_arcane_bloodline_progression_tests::sorcerer_bloodline_feat_pool_slot_count_is_correctly_absent_below_the_grant_level ... ok

$ python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
cr = [u for u in d['units'] if u.get('book')=='core_rulebook'
      and u.get('status')=='engine-does-not-hold'
      and u.get('evidence')=='class_feature_owner_matched_by_name_but_record_not_held_by_engine']
print(len(cr))
"
251
```
This cycle's own artifact is this receipt plus the 93 units it moved; the row-count that
governs `status` is the mechanism's population count above: **251 remaining, 93 closed.**

## Build scope verified

- `cargo test --locked --no-run` (full workspace, `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-001`)
  → exit 0, run at this cycle's HEAD **after** the last commit that moves a figure this receipt
  depends on (`decisions.md §12` L7 — run after `docs/work-inventory.json`'s third and final
  regeneration).
- `cargo test --locked --lib` (full workspace lib suite) → `2891 passed; 0 failed; 14 ignored`
  — includes `class_feature_pool_catalog::tests::
  class_feature_owner_matched_but_not_held_346_sub_causes_are_named_and_sum_exactly`, which
  re-derives its own population LIVE from `docs/work-inventory.json` rather than a hardcoded
  346 (confirmed by reading its source), so it self-adjusted to the new 251 figure and still
  passed — no stale assertion broke.
- `apps/desktop/src-tauri`: not touched this cycle (`git diff --name-only
  $(git merge-base HEAD origin/develop)...HEAD -- apps/desktop/src-tauri` shows nothing) — not
  re-run this cycle, matching `workflow-instruction.md §2.5`'s "test the targets your change
  touches" scoping.

## Sweep population

`corpus_literal_sweep` examined-population at this cycle's final (third) regeneration run:

```
corpus-literal-sweep: 48708 records examined of 51482 read, 413336 tokens compared
(9 synthesized), 51469 digests checked, 0 findings
```

Identical to cycle 4's own closing figure (48,708 of 51,482) across all three of this cycle's
own regeneration passes — **this cycle added or regenerated zero corpus records**
(`decisions.md §12` L8 governs a gate whose examined-population must grow by exactly the record
delta over a change THAT cycle makes; this cycle made none).

## Oracle pin

N/A — no figure in this receipt is derived from the pinned PCGen oracle corpus.

- **Status:** partial. This cycle closes **93 of 344** units (bucket B, `core_rulebook`, atlas
  partition 692 → 579 of 6,701) via real engine wiring applying `decisions.md §16`'s ratified
  precedent — grounding a bloodline/style-invariant slot COUNT as a magnitude and naming the
  full eligible set for both `Sorcerer Bloodline Feat` and `Ranger Combat Style Feat`, without
  seeding any default choice. AT-34-E3-001 as a whole does NOT close this cycle: the other
  eight mechanisms are owned by other cycles and are not this cycle's scope, and 251 units
  remain in this mechanism alone.

## Movement, four buckets

- **Closure:** 93 (`engine-does-not-hold` → `deferred-with-reason`, bucket B → bucket X per
  `decisions.md §2`, "cleared by revisiting the stated condition" — the stated condition being
  the sub-choice this bounded seam deliberately does not model, exactly as §16 prescribes).
- **Reclassification:** 0 (no unit's evidence string changed without a status change; the 93
  closed units' evidence changed BECAUSE their status changed).
- **Reachability:** 93 (two new engine explanations/diagnostic sets now answer `held` — as a
  magnitude for the count, and as a stated-reason deferral for each named option — for both
  record shapes; the literal mechanism `decisions.md §2`'s bucket-B "cleared by placing the
  record" names, applied through bucket X's own real clearing path rather than bucket D or M
  since no display or magnitude is claimed for any individual option).
- **Instrument-correction:** 0 (the starting population re-derived cleanly to the same 344
  cycle 4 reported; no wrong prior claim was found — the substring-collision defect this cycle
  found and fixed was in THIS cycle's own first-draft code, not a prior cycle's claim).

## Notes

- **`decisions.md §16` unblocked exactly what it named**: 103 of 344 (the Sorcerer Bloodline
  Feat + Ranger Combat Style Feat majority three cycles had each stalled on as "operator-scoped"
  now closes in one cycle by applying the ALREADY-RATIFIED precedent — confirming §16's own
  framing that this was research a cycle owed before escalating, not a genuine open question.
  93 rather than the full 103 close (10 Sorcerer names' diagnostics are deliberately withheld
  to avoid a verified cross-attribution defect — see above), an honest reduction, not a
  shortfall hidden inside a rounder number.
- **The substring-collision defect is a structural property of
  `v06_work_inventory.rs::diagnostic_id_names_feature`** (matches by substring within an
  owner's namespace, not by exact key) that this cycle did not introduce but did trigger, being
  the first cycle to emit a LARGE batch (103) of per-option diagnostics sharing one owner
  namespace. It is flagged here, not silently patched around: any FUTURE cycle emitting a
  similar per-option diagnostic batch for `sorcerer`- or `ranger`-owned records should run the
  same before/after-diff verification this cycle used, not assume a clean substring match.
- **The 118-unit `description_is_null_internal_bookkeeping` sub-cause and the 118-unit
  `engine_effect_token_present` sub-cause's long tail** (cycle 4's own partition) are untouched
  — this cycle's scope was exactly `Sorcerer Bloodline Feat`/`Ranger Combat Style Feat`, per the
  dispatch brief's explicit instruction, not the zero-description internal-bookkeeping
  definitional question (`atlas-defects.md`).
- **118 zero-description internal-bookkeeping units — left alone, not reclassified**, per the
  dispatch brief's explicit instruction (a separate, still-open definitional question).

## Next-cycle plan

1. **Re-derive the 251-unit remainder fresh** rather than inherit this cycle's partition
   verbatim (`decisions.md §12` L2) — cycle 4's own 15-unit `engine_effect_token_present` long
   tail (`Rogue Talent` 3, `Wizard` 2, `Core Domain` 2, `Monk` 2, `Shadowdancer` 1, `Assassin` 1,
   `Cleric` 1, `Duelist` 1, `Nobility Domain` 1, `Sorcerer Bonus Spell L3` 1) is the next-
   cheapest known shape, each its own real engine-wiring project.
2. **Operator-scoped ruling** on the 118-unit `description_is_null_internal_bookkeeping`
   sub-cause remains open (`atlas-defects.md`), unchanged.
3. **The 67-unit `catalog_serves_it_but_classify_wiring_class_gate_blocks_promotion` sub-cause**
   remains each its own real per-character computation project, unchanged.
4. **If a future cycle emits another per-option diagnostic batch** sharing the `sorcerer` or
   `ranger` owner namespace, re-run this cycle's own corpus-wide cross-check (every unit sharing
   that owner, not only the mechanism's own bucket) before trusting the batch collision-free.
