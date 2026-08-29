# Cycle — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 — wave-9 shared `docs/work-inventory.json` regeneration

- **Commit SHA:** this receipt is committed in the same commit as the code/docs it describes;
  `git log -1` at the time of reading resolves it. Base rebased onto `origin/tranche/14` at
  `557b202dd2` before any pass ran.
- **Files touched:** `docs/work-inventory.json` (regenerated, twice — see Discoveries),
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
  (regenerated output of `completion_atlas.py --check`), `scripts/completion_atlas.py` (10
  `BUCKET_DEFINITIONS` `file:line` citations re-derived and fixed, twice),
  `src/bin/v06_work_inventory.rs` (restored a wiring block + two tests silently deleted by a
  sibling lane's rebase — see Discoveries), this receipt, `docs/release/SD-34-book-completion/
  progress.md`, `docs/release/SD-34-book-completion/kanban.md`.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` —
  `git diff --unified=0 -- docs/work-inventory.json scripts/completion_atlas.py src/bin/v06_work_inventory.rs | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  → zero matches.
- **Wired-integration audit result:** `OK_NO_TOKENS` — same diff,
  `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` → zero matches.
- **Acceptance criterion:** this cycle implements no `epic-breakdown.md` criterion directly —
  it is the wave's own shared regeneration cycle, paid once for four parallel lanes
  (gate-widening, owner-matched, with-magnitude, option-pool) that deliberately deferred
  regenerating `docs/work-inventory.json` to avoid paying the three-pass pipeline's cost four
  times. Evidence: the whole-corpus before/after diff below, and the four lanes' own
  expectation-vs-actual table.

## Why this cycle exists

Four lanes landed engine changes on `tranche/14` and did not regenerate
`docs/work-inventory.json` (the three-pass pipeline is the throughput bottleneck; paying it
once for all four lanes rather than four times is the wave's own design). Their engine
changes were committed and unmeasured until this cycle ran the pipeline and diffed the result.

## Procedure and per-pass wall time

Environment: `RETRO_ACTOR=sd34-wave9-regen`, `CARGO_TARGET_DIR=/tmp/cargo-sd34-wave9regen`,
`CARGO_INCREMENTAL=0`, fresh target directory (no stale artifacts from any sibling lane).

```bash
git fetch origin tranche/14 && git rebase origin/tranche/14   # fast-forward, no conflicts
git show HEAD:docs/work-inventory.json > /tmp/wi-wave9-before.json   # snapshot before any pass
```

| Pass | Command | Wall time |
|---|---|---|
| 1 | `cargo run --locked --bin corpus_literal_sweep -- --json-out /tmp/sweep.json` | **3m24.871s** (`real`) |
| 2 | `cargo run --locked --bin derived_evaluator_fixture_check -- --json-out /tmp/fixture.json` | **0m13.097s** (`real`) |
| 3 | `CORPUS_LITERAL_SWEEP_REPORT=/tmp/sweep.json DERIVED_FIXTURE_CHECK_REPORT=/tmp/fixture.json cargo run --locked --bin v06_work_inventory` (no `--allow-stamp-loss`) | **11m0.074s** (`real`), first run |

Total first pass through the pipeline: **~14m38s**. Pass 1 output:
`corpus-literal-sweep: 48708 records examined of 51482 read, 413336 tokens compared (9
synthesized), 51469 digests checked, 0 findings`, `CLEAN`. Pass 2 output:
`derived-evaluator-fixture-check: 1839 unit(s) cleared over 2580 fixture row(s); 0 failed; 0
not ingested`.

A second, corrective run of pass 3 followed the restoration described below (source changed
after the first regeneration, so the inventory had to be re-derived again). Its wall time was
not separately captured with `time`, but ran against an already-warm target directory
(recompiling one changed file) and completed in well under 3 minutes.

## Whole-corpus before/after diff, by unit id

```python
import json
before = json.load(open('/tmp/wi-wave9-before.json'))
after = json.load(open('docs/work-inventory.json'))
bi = {u['id']: u for u in before['units']}
ai = {u['id']: u for u in after['units']}
print('before', len(bi), 'after', len(ai))
print('added', len(set(ai)-set(bi)), 'removed', len(set(bi)-set(ai)))
changed = [uid for uid in set(bi)&set(ai)
           if bi[uid]['status'] != ai[uid]['status'] or bi[uid]['evidence'] != ai[uid]['evidence']]
print('changed', len(changed))
```

Result: **49,438 ids before, 49,438 after, 0 added, 0 removed, 79 changed** (status or
evidence), **all 79 in `core_rulebook`**. No other book's units were touched — consistent
with all four lanes' own file-touch sets (`src/rules_core/pilot_compute/`, `src/rules_core/
class_feature_pool_catalog.rs`, `src/rules_core/rules_tables/crb/`) being Core-Rulebook-scoped
mechanisms.

## A rebase regression found by this cycle's own regen, not by inspection

The first pass-3 run showed the `class_feature_option_pool_record_not_held_by_engine`
mechanism population **unchanged at 34** (before 34, after 34, identical id set), directly
contradicting the option-pool lane's own commit message (`a183d70c76`: "New join function +
lookup table + one new `v06_work_inventory.rs` fallback rung move all 9 keys bucket B -> D").

Investigation: `git log --oneline a183d70c76..HEAD -- src/bin/v06_work_inventory.rs` showed
exactly one intervening commit, `534c9c2a61` ("generalize Weapon Training to all 14 groups,
256->208 (expected)"). `git show 534c9c2a61 -- src/bin/v06_work_inventory.rs` confirmed this
commit's diff **deletes** the option-pool lane's entire `wizard_school_spell_list_key_owner`
`classify()` arm and both of its dedicated tests
(`a_wizard_school_spell_list_row_verified_against_the_join_leaves_bucket_b`,
`an_unlisted_wizard_spells_shaped_key_still_falls_to_the_generic_fallback`) — an artifact of
resolving its own rebase conflict against the option-pool lane's freshly-landed commit.
Nothing in `534c9c2a61`'s commit message, its own receipt, or its subject line mentions
removing anything; the subject describes only the Weapon Training generalization it did add.
This is the exact "innocent commit message, destructive contents" failure shape this bundle's
own retrospective doctrine warns about — confirmed here by `git show --stat` / `git diff`
against the parent commit, not assumed from the subject line.

**Fix, in this cycle:** restored the deleted `classify()` arm (the `if class_feature_pool_
catalog::wizard_school_spell_list_key_owner(&unit.key).is_some()` block, verbatim from the
pre-deletion diff) and both deleted tests, verbatim, into `src/bin/v06_work_inventory.rs`.
`class_feature_pool_catalog::wizard_school_spell_list_key_owner` and
`WIZARD_SCHOOL_SPELL_LIST_KEY_OWNER` themselves were never deleted (they live in a different
file, `class_feature_pool_catalog.rs`, untouched by `534c9c2a61`) — only the CALLER wiring and
its tests were lost.

```
$ cargo test --locked --bin v06_work_inventory wizard_school_spell
test class_feature_text_complete_rung_tests::a_wizard_school_spell_list_row_verified_against_the_join_leaves_bucket_b ... ok
test result: ok. 1 passed; 0 failed

$ cargo test --locked --bin v06_work_inventory
test result: ok. 419 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
(419, not the pre-deletion 418/416 baseline various receipts cited — the two restored tests
plus the with-magnitude lane's own net test-count change both land in this same file.)

`cargo test --locked --no-run` (workspace) and `cargo test --locked --no-run --manifest-path
apps/desktop/src-tauri/Cargo.toml` both re-run after the restoration: **exit 0**, both.

Pass 3 was then re-run in full (`CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT`
still set, no `--allow-stamp-loss`) to produce the final `docs/work-inventory.json` used for
every figure below.

## Attribution — expected vs actual, per lane

### with-magnitude (`class_feature_option_pool_record_with_magnitude_not_held_by_engine`)

**Expected:** 256 -> 208 (48 closed), Weapon Training generalized from 4 hardcoded canonical
groups to all 14 real PF1 groups. Explicitly stated Monk's 4 records carry a different
evidence path and would NOT move this cycle.

**Actual:** 48 units closed exactly as predicted — `engine-does-not-hold` / `class_feature_
option_pool_record_with_magnitude_not_held_by_engine` -> `grounded` / `fighter_weapon_
training_probe_observed_a_real_computed_magnitude`. **Plus 4 more**: `weapon_training_{1,2,3,4}
_monk` (the "Monk" PF1 weapon-training GROUP — a weapon category, unrelated to the Monk
CLASS or its `is_monk_pillar_id` LevelUpPlan filter) closed to the same `grounded` verdict via
a different pre-existing `no_explanation_id_and_no_diagnostic_names_this_feature` evidence
path the lane's own population check did not enumerate. **52 real closures, not 48** — a
better outcome than predicted, not a discrepancy to be concerned about (the generic fix's own
`WEAPON_TRAINING_GROUPS` table legitimately includes the Monk weapon group; the lane's own
receipt's "Monk carries a DIFFERENT evidence... left untouched and unclaimed" was a correct
description of the STARTING evidence string, not a correct prediction of the ENDING one).

**Verdict: confirmed, and better than predicted.**

### option-pool (`class_feature_option_pool_record_not_held_by_engine`)

**Expected (per the lane's own commit message):** 9 units move bucket B -> D via the new
`wizard_school_spell_list_key_owner` join (not closure to DONE — the lane's own commit and
receipt are explicit that this only certifies the record is HELD by real tables, not that it
displays).

**Actual, before this cycle's restoration:** 34 -> 34, unchanged — the fix had been silently
deleted by a sibling lane's rebase (see above). **After restoration:** 9 units move exactly as
described, `engine-does-not-hold` / `class_feature_option_pool_record_not_held_by_engine` ->
`engine-does-not-hold` / `class_feature_wizard_school_spell_list_held_by_wizard_spell_list_and_
spell_list_join`.

**Verdict: confirmed only because this cycle found and repaired the regression. Without that
repair, this lane's stated result would have been silently false in the shipped inventory.**

### owner-matched (`class_feature_owner_matched_by_name_but_record_not_held_by_engine`,
non-excluded remainder)

**Expected:** 0 of its own 24-unit population (18 null-description, 6 real-description but
gate-refused) moves.

**Actual:** confirmed directly — re-checked all 6 named gate-refused units
(`arcane_trickster_invisible_thief`, `rage_power_knockback`, `rogue_talent_bleeding_attack`,
`rogue_talent_finesse_rogue`, `rogue_talent_improved_evasion`, `rogue_talent_skill_mastery`)
byte-for-byte identical before/after (same `status`, same `evidence`); the 18 null-description
units are a subset of the same unchanged population (this mechanism's total 242 -> 242, 218
excluded-class + 24 non-excluded-class, ids identical throughout).

**Verdict: confirmed exactly.**

### gate-widening (`class_feature_owner_matched_by_name_but_record_not_held_by_engine`,
excluded-class sub-cause, `decisions.md §18`)

**Expected:** "A floor of 5 units move for certain" — `Bard ~ Bardic Knowledge`, `Bard ~ Lore
Master`, `Paladin ~ Holy Champion`, `Paladin ~ Lay on Hands`, `Sorcerer ~ Spells` — plus an
unconfirmed, larger tail from the `already_admitted` population; 0 for Druid/Monk.

**Actual:** **refuted for 4 of the 5 named units.** Direct lookup of all 5:

| Unit | Before | After |
|---|---|---|
| `bard_bardic_knowledge` | `grounded` / `explanation_id_observed_in_a_real_computation` | unchanged |
| `bard_lore_master` | `grounded` / `explanation_id_observed_in_a_real_computation` | unchanged |
| `paladin_holy_champion` | `literal-verified` / `explanation_id_observed_in_a_real_computation` | unchanged |
| `paladin_lay_on_hands` | `fixture-verified` / `explanation_id_observed_in_a_real_computation` | unchanged |
| `sorcerer_spells` | `engine-does-not-hold` / `no_explanation_id_and_no_diagnostic_names_this_feature` | `engine-does-not-hold` / `class_feature_no_dedicated_magnitude_id_matched_the_record_slug` |

The 4 Bard/Paladin units were **already grounded/verified before this wave started**, via a
code path entirely outside `class_feature_grant_consumer.rs` — the gate-widening lane's own
receipt named this exact possibility ("the collision guard correctly protecting a real
hand-wired magnitude, not a defect"), and it is what happened: `already_computed_slugs`
(or an equivalent hand-wired path) already produced these, unaffected either way by the
citation-gate widening. Only `sorcerer_spells` changed status, and it **reclassified** (C -> D,
not a closure), landing alongside **4 different, un-named units** the receipt never mentioned:
`wizard_arcane_bond`, `cleric_aura`, `paladin_detect_evil`, `wizard_bonus_feats` (same C -> D
reclassification, same new evidence string). **5 units reclassified in total — the predicted
COUNT, but not the predicted MEMBERSHIP.**

Separately, **2 units the receipt did not name as moving DID close to DONE**:
`wizard_cantrips` and `cleric_spontaneous_casting`, both `engine-does-not-hold` /
`class_feature_owner_matched_by_name_but_record_not_held_by_engine` -> `text-complete` /
`explanation_id_observed_and_corpus_record_carries_real_description` — real closures the gate
widening produced that its own receipt did not claim credit for (consistent with its own
"Wizard and Cleric contribute zero newly-resolved records to this specific census... their
emissions come from the `already_admitted` bucket, which this census does not itemize by
class" — this cycle's regen is exactly that itemization).

0 Druid/Monk movement confirmed (neither appears in the 79-unit changed set at all).

One further, unrelated reclassification landed in this same diff: `bard_bardic_performance`
(`deferred-with-reason` / an `engine_diagnostic:...` evidence string -> `engine-does-not-hold`
/ `class_feature_no_dedicated_magnitude_id_matched_the_record_slug`) — not predicted by any
lane's receipt, not investigated further this cycle (out of this cycle's own narrow scope;
named here per `decisions.md §2`'s "an unpredicted step is a defect" rule so a future cycle
does not have to rediscover it).

**Verdict: predicted count (5) coincidentally matched actual count (5), but membership
differed entirely; the lane's own receipt under-claimed 2 real closures it did not itemize.**

### 9 units of evidence-string churn with no bucket crossed (not attributed to any single lane)

9 already-`text-complete` (DONE) units changed `evidence` string only, `status` unchanged:
`sorcerer_cantrips`, `paladin_aura_of_resolve`, `wizard_spells__d203917b8596067b`, `wizard_
bonus_languages`, `paladin_aura_of_courage`, `bard_armored_casting`, `paladin_divine_health`,
`bard_cantrips`, `cleric_orisons`. All moved `class_feature_pool_catalog_serves_a_rendered_
description` -> `explanation_id_observed_and_corpus_record_carries_real_description` — a side
effect of the gate-widening lane's citation-based rewrite reaching units that were already
admitted before the widening (their status never crossed a bucket boundary, so this is
reported separately from both closure and reclassification per `decisions.md §9`'s own
discipline against folding a non-movement into either bucket).

## Movement, four buckets (`decisions.md §9`)

- **Closure:** 54 — 48 + 4 Weapon Training (with-magnitude) to `grounded`; 2 owner-matched
  units (gate-widening) to `text-complete`.
- **Reclassification:** 16 — 9 wizard-school-join (option-pool, B -> D); 5 gate-widening
  (C -> D); 1 owner-matched (B -> D); 1 bard-performance (X -> D).
- **Evidence-string churn, no bucket crossed:** 9 (see above) — reported separately, not
  folded into either closure or reclassification.
- **Instrument-correction:** 0. No prior wrong count was found in the inventory itself; the
  option-pool discrepancy is a code regression (fixed above), not a measurement error.

**54 + 16 + 9 = 79**, the full changed-unit count.

## `completion_atlas.py` — corpus-wide and per-book

```
$ python3 scripts/completion_atlas.py --check
population=49438 buckets=10 unclassified=0 overlap=0
  DONE: 14740  A: 449  B: 11771  C: 4344  D: 3071  M: 5114  V: 9558  U: 202  X: 170  Z: 19
done_evidence_violations=0
missing_clearing_mechanisms=0
stale_derived_at=False
citation_failures=0
```

All 10 `BUCKET_DEFINITIONS` `file:line` citations were re-derived by direct `grep` against
`v06_work_inventory.rs`'s live line numbers and fixed **twice** in this cycle: once after the
shared regeneration surfaced `citation_failures=10` (every citation shifted by the with-
magnitude lane's own insertions, verified line-by-line against the pre-with-magnitude commit
to confirm each new target was the SAME semantic anchor, not merely "a nearby line"), and a
second time after this cycle's own 41-line restoration shifted four of them again (`A`, `B`,
`C`, `V`).

```
$ python3 scripts/completion_atlas.py --book core_rulebook --check
book=core_rulebook population=6701 unclassified=0 overlap=0
  DONE: 1502  A: 0  B: 472  C: 363  D: 398  M: 1048  V: 2793  U: 10  X: 115  Z: 0
```
(exit 1 — the book is not yet closed; this cycle regenerates, it does not close buckets.)

```
$ python3 scripts/completion_atlas.py --book ultimate_campaign --check
book=ultimate_campaign population=265 unclassified=0 overlap=0
  DONE: 130  A: 0  B: 0  C: 0  D: 5  M: 89  V: 18  U: 21  X: 2  Z: 0
```
(exit 1 for the same reason; 0 of this book's units appear in the 79-unit changed set — none
of the four lanes touch `ultimate_campaign`.)

## Denominator gate

```
$ python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'
files_checked=15
violations=5
```
All 5 are pre-existing verbatim-quoted corpus prose in `progress.md` (`FRT_HVY`'s own "75%
chance..." description, quoted for context by earlier cycles), already flagged by the
already-merged `AT-34-E3-004` cycle and every subsequent cycle that has run this check.
Unchanged by this cycle — no new `.md` prose containing a bare percentage was added.

## Build scope verified

`cargo test --locked --no-run` (workspace): exit 0, run at this cycle's final HEAD (post-
restoration). `cargo test --locked --no-run --manifest-path apps/desktop/src-tauri/Cargo.toml`:
exit 0, explicit desktop-crate run (`decisions.md §10`). `cargo test --locked --bin v06_work_
inventory`: 419/419 passed.

## Sweep population

Pass 1 (`corpus_literal_sweep`) examined 48,708 of 51,482 records read, 0 findings, `CLEAN`.
This cycle did not add or edit any `data/corpus/**` record — the sweep's own examined-count
is unrelated to the source-code restoration above (no record delta to reconcile against).

## Oracle pin

N/A — no figure in this receipt came from the pinned PCGen oracle corpus.

## Row-count command output

This cycle's own "artifact" is `docs/work-inventory.json` itself; its row count is the
`population=49438` line above, unchanged (0 added, 0 removed) — the count that matters here is
the **changed-unit** count (79), derived by the before/after diff script quoted above.

## Status

**complete** — the shared regeneration ran, the pre-existing rebase regression was found and
repaired in the same cycle (not merely reported), and every lane's expectation was checked
against the live post-regen inventory, per this wave's own instruction ("an expectation that
turns out wrong is a useful finding, not a failure").

## Notes

- **This bundle's own `decisions.md`/`workflow-instruction.md` doctrine already names this
  exact failure shape** ("innocent commit message, destructive contents" — a subject line that
  does not mention removing anything, but whose diff does). The mechanical guard this bundle
  already has —
  `git diff --cached --numstat` before every commit — would not have caught this one, because
  the DELETING commit (`534c9c2a61`) was itself a normal-looking source diff with real
  insertions alongside the silent deletions; only a before/after diff of the DERIVED artifact
  (`docs/work-inventory.json`) against the lane's own stated expectation surfaced it. This is
  the strongest argument yet for why this wave's shared-regeneration cycle is not just a
  throughput optimization: it is also the first point at which four lanes' claims are checked
  against each other's actual, combined effect on the live corpus, rather than each lane's
  isolated unit tests.
- **Discrepancy between the with-magnitude lane's own commit message and the actual result is
  in this cycle's favor** (52 real closures vs. 48 predicted) and needs no further action.

## Next-cycle plan

1. **AT-34-E6-001 (final-acceptance scan)** should re-derive this wave's own figures at HEAD
   rather than trusting this receipt, per its own standing instruction.
2. **`bard_bardic_performance`'s new D-bucket reclassification** (unpredicted by any lane) is
   unexplained; a future AT-34-E3-001 cycle on the Bard-mechanism sub-cause should investigate
   whether it is a genuine side effect of the citation-gate widening or an unrelated concurrent
   change.
3. **Core Rulebook bucket B (472) and D (398)** remain open across the mechanism's own
   un-worked remainder (companion/mount registration, proficiency possession-tracking, the
   remaining wizard-opposition-school clusters, Domain Power/Domain Base) — unchanged in scope
   from prior cycles' own next-cycle plans, only the live counts shift with this cycle's
   regeneration.
