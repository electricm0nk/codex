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

---

# Cycle — wave-9 regeneration and attribution, second dispatch (2026-08-30, later same day)

**This filename was dispatched a second time**, against a different set of four lanes (M
`skill_content`, V-ledger corpus-wide bucket-V rebuild, UC `AT-34-E4-002` cycle 3, and this
cycle itself) than the ones the section above documents (gate-widening/owner-matched/with-
magnitude/option-pool). Rather than overwrite that prior, already-committed and unrelated
receipt, this section is appended below it — the two document genuinely different cycles that
happen to share a `wave9` filename; the section above is preserved verbatim.

**Status: complete.** Single mandatory regeneration-and-attribution cycle closing this wave's
four dispatched lanes — `docs/work-inventory.json` regenerated exactly once via the required
three-pass pipeline, for all of them, per `workflow-instruction.md`'s shared-regen protocol.

## 0. Pre-work: folded two lanes' uncommitted duplicate-dispatch cleanup

Before touching anything, `git status --porcelain` in this shared checkout showed staged-but-
uncommitted deletions/edits left by the UC and V-ledger lanes (their own duplicate-dispatch
fold: removing a superseded `AT-34-E3-005` second-lane confirmation receipt/kanban row/retro
event, and a superseded `AT-34-E4-002` cycle-3 manifest/receipt/retro event, both already fully
captured by earlier landing commits). Rather than discard or stash this work (forbidden by the
shared-checkout rules), it was committed on its own first (`fbc37abaed`, later rebased to
`782584b4b3`) so it could not be lost, before rebasing onto `origin/tranche/14`.

`git fetch origin tranche/14 && git rebase origin/tranche/14` then pulled in two commits not yet
in this worktree: `4d27d70551` and `409ada6cda` (the M lane's classifier-wiring engine change and
its own doc/atlas update). The rebase produced two real conflicts — `completion-atlas.json`
(`derived_at` only, resolved to the newer SHA, moot once regenerated below) and `progress.md`
(the M lane's new cycle section needed reinserting alongside this cycle's own fold-deletion of
the two superseded sections) — both resolved by hand, verified line-by-line against both sides.

## 1. Baseline

Snapshot taken at `HEAD` (`782584b4b3`, post-rebase, pre-pipeline): `docs/work-inventory.json`,
49,438 units.

## 2. Three-pass pipeline, in order, timed

| Pass | Command | Result |
|---|---|---|
| 1 | `cargo run --locked --bin corpus_literal_sweep -- --json-out sweep.json` | CLEAN — 48,708 records examined of 51,482 read, 413,336 tokens compared, 51,469 digests checked, **0 findings** |
| 2 | `cargo run --locked --bin derived_evaluator_fixture_check -- --json-out fixture.json` | **0 failed, 0 not-ingested** — 1,839 units cleared over 2,580 fixture rows |
| 3 | `CORPUS_LITERAL_SWEEP_REPORT=... DERIVED_FIXTURE_CHECK_REPORT=... cargo run --locked --bin v06_work_inventory` | ran to completion, `docs/work-inventory.json` rewritten, 49,438 units |

`--allow-stamp-loss` never passed; both upstream reports existed before pass 3 ran, so the guard
never had cause to refuse it.

**Wall time, this cycle's own measurement (the figure this wave shape exists to produce):**
pass 1 start `09:18:14` to pass 3 completion `09:42:45` — **1,471 seconds (24m 31s)** for the full
three-pass pipeline, corpus-wide (49,438 units), on this box. (One earlier attempt at pass 3 hit
the 10-minute foreground command cap with zero output and was killed with no side effect —
re-run in the background; the timed figure above is the successful run only.)

## 3. Whole-corpus before/after diff, by unit id — the headline finding

```
before: 49438 units    after: 49438 units    added: 0    removed: 0
changed (status or evidence, any field): 0
raw docs/work-inventory.json diff: 1 line (generated_at commit SHA only)
```

**Zero units moved.** This is not a null result from a broken pipeline — pass 1 and pass 2 both
ran clean and fresh, and pass 3 executed the full regeneration end-to-end; the diff against the
pre-pipeline baseline is a single metadata line (`generated_at`). Every one of the 49,438 unit
records — `status`, `evidence`, `wiring_class`, and every other field — is byte-identical
before and after.

**The reason, traced by commit:** two of this wave's lanes had already regenerated and
committed `docs/work-inventory.json` themselves, correctly, before this cycle ever ran:

- `cfd9c6d3d9` (V-ledger, "rebuild the lost corpus-wide bucket-V ledger and commit it") —
  `docs/work-inventory.json` changed by itself, 13,406 lines, in that same commit.
- `4d27d70551` (M, "wire the classifier's grounding check, 76 core_rulebook units closed") —
  `docs/work-inventory.json` changed by itself, 160 lines, in that same commit.

Only the UC lane (`0007792438`, `AT-34-E4-002` cycle 3) made zero Rust/corpus changes and
correctly left the file untouched.

**This is the mismatch this wave's own dispatch brief asked to be reported plainly, not smoothed
over: the brief's premise — "four lanes just landed engine changes and deliberately did not
regenerate `docs/work-inventory.json`" — did not hold for two of the four.** The V-ledger and M
lanes each ran their own regeneration and committed it inline with their engine/data change,
correctly and (per this cycle's independent full pipeline re-run) exactly right — 0 drift, 0
disagreement, byte-identical. This cycle's actual, real contribution for those two lanes is
**verification, not correction**: an independent, from-scratch three-pass pipeline run confirming
their self-regeneration was not a hand-patch or a partial update that happened to look plausible,
but the genuine article. The wave-shape's own premise (a single shared regen absorbing four
lanes' deferred work) was correct in spirit and remains the right shared-checkout discipline
going forward, but this specific wave had less deferred regeneration debt than dispatched.

## 4. Movement, four buckets (against the wave's own true baseline)

| Bucket | Count | Detail |
|---|---|---|
| Closure (reached DONE) | 0, this cycle | 76 `skill_content` units and the bucket-V corpus-wide widening (256 units net across two commits) reached DONE in `4d27d70551`/`cfd9c6d3d9` themselves, already reflected in this cycle's baseline snapshot — not movement attributable to this cycle's own pipeline run |
| Reclassification (moved between non-DONE buckets) | 0 | — |
| Reachability | 0 | — |
| Instrument-correction | 0 | `completion_atlas.py --check` (corpus-wide): `citation_failures=0`, `done_evidence_violations=0`, `missing_clearing_mechanisms=0`, `stale_derived_at=False` — no citation drift from this wave's line insertions |

A `B`→`X` move is reclassification, never closure; none occurred this cycle (0 movement of any
kind this cycle's own pipeline run produced).

## 5. Atlas checks

`python3 scripts/completion_atlas.py --book core_rulebook --check`:
`population=6701 unclassified=0 overlap=0` — `DONE:4330 B:470 C:357 D:366 M:972 V:81 U:10 X:115`.

`python3 scripts/completion_atlas.py --check` (corpus-wide):
`population=49438 buckets=10 unclassified=0 overlap=0` —
`DONE:24242 A:449 B:11769 C:4338 D:2955 M:5038 V:256 U:202 X:170 Z:19`.
`citation_failures=0` both scopes — no `scripts/completion_atlas.py` `file:line` citation
re-derivation was needed this cycle.

## 6. Build verification

`cargo test --locked --no-run` (full workspace): exit 0, 160s.
`cargo test --locked --no-run --manifest-path apps/desktop/src-tauri/Cargo.toml` (desktop crate,
separate cargo workspace, tested explicitly): exit 0, 148s. Both run **after** this cycle's own
regeneration commit, per protocol.

## 7. Commit

Fold-first commit `782584b4b3` (UC/V-ledger duplicate-dispatch cleanup, pre-existing staged
work), rebase onto `origin/tranche/14` (M lane's `4d27d70551`/`409ada6cda`), then this cycle's own
regeneration + atlas restamp + kanban/progress update, committed and pushed together.

---

# Cycle — wave-15 regeneration and attribution, third dispatch (2026-08-30, later same day)

**This filename was dispatched a third time**, against a different set of dispatched lanes
(UC `AT-34-E4-002` cycle 3, C `AT-34-E3-002` Monk Unarmed Damage Medium, M `AT-34-E3-003`
equipment `DAMAGE:`-token widening) than either of the two sections above. Per the same
precedent the second dispatch set, that section is preserved verbatim and this one is appended
below it rather than overwriting anything.

**Status: complete.** Single mandatory regeneration-and-attribution cycle closing this wave's
dispatched lanes — `docs/work-inventory.json` regenerated exactly once via the required
three-pass pipeline, per `workflow-instruction.md`'s shared-regen protocol.

## 0. "Four lanes" — the dispatch brief's own boilerplate, corrected

The dispatch prompt's fixed template text says "Four lanes just landed engine changes." Only
**three** lane summaries were actually supplied (UC, C, M), and `git log b939abcd4b..HEAD`
together with `sd-34-dispatch.workflow.js`'s own wave-14/15 lane list confirms only three ran
this wave — the wording is stale boilerplate carried over from an earlier four-lane wave shape
(the same script's own comment block documents the shape change), not a fourth lane whose
report went missing. Named here per this wave's own "an unpredicted step is a defect" rule so
a future cycle does not waste time hunting for a phantom fourth lane.

## 1. Rebase and baseline

`git fetch origin tranche/14 && git rebase origin/tranche/14`: fast-forward, no conflicts,
landing at `7147fd86ab` (10 commits ahead of this cycle's dispatch-time HEAD `b939abcd4b`).
Baseline snapshot taken from that **already-rebased** HEAD per this cycle's own instruction
order (fetch/rebase first, snapshot second) — `docs/work-inventory.json`, 49,438 units. This
matters below: two lanes' own inline self-regenerations are already inside this baseline.

## 2. Three-pass pipeline, in order, timed

| Pass | Command | Result |
|---|---|---|
| 1 | `corpus_literal_sweep --json-out /tmp/sweep.json` | CLEAN — 48,708 of 51,482 examined, 413,336 tokens compared, 51,469 digests checked, **0 findings**. `real 4m9.446s` |
| 2 | `derived_evaluator_fixture_check --json-out /tmp/fixture.json` | **0 failed, 0 not-ingested** — 1,839 units cleared over 2,580 fixture rows. `real 0m19.484s` |
| 3 | `v06_work_inventory` (both report env vars set, no `--allow-stamp-loss`) | ran to completion, `docs/work-inventory.json` rewritten, 49,438 units. `real 13m32.058s` |

`--allow-stamp-loss` never passed; both upstream reports existed before pass 3 ran.

**Total wall time for the three-pass pipeline: 1,080.988s (18m00.988s)** — the figure this wave
shape exists to produce, run foreground/backgrounded-and-monitored, not estimated.

## 3. Whole-corpus before/after diff, by unit id

```
before: 49438 units    after: 49438 units    added: 0    removed: 0
changed (status or evidence, any field): 37
```

**Two lanes' movement is already inside the baseline, and this cycle's independent re-run
reproduces both exactly, 0 drift:**

- **C** (`50790d6bf9`, Monk Unarmed Damage Medium): the 6 targeted units, byte-identical
  `status`/`evidence` before vs. after this cycle's own pipeline run. `core_rulebook` bucket C:
  351 before this cycle, 351 after.
- **UC** (`e8ac310280`, flat `BONUS:SKILL` trait capability): the 36 corpus-wide `grounded`
  trait units (31 `ultimate_campaign` + 5 `advanced_players_guide`), byte-identical before vs.
  after. `kind=trait, status=grounded` count: 36 before, 36 after.

**Only the M lane (equipment `DAMAGE:`-token widening, deliberately un-self-regenerated per
the dispatch brief) produced real movement.** All 37 changed units trace to it.

## 4. M's own stated expectations, checked against the live regen — confirmed exactly

M's own receipt (`AT-34-E3-003_m_bucket_equipment_cycle_receipt.md`) predicted, from a local
regen it ran and then `git restore`d before committing: 31 corpus-wide closures across 9 named
books, 0 closures in `core_rulebook` from the closure-only equipment shape, and 6
`ultimate_equipment` bucket-V evidence-string reclassifications. This cycle's own independent,
from-scratch pipeline run reproduces every one of those figures exactly:

- **31 closures**, unit for unit the same 9-book split M named: `core_rulebook` 14,
  `ultimate_equipment` 5, `bestiary_3` 4, `inner_sea_races` 2, `ultimate_psionics` 2,
  `advanced_class_guide` 1, `advanced_players_guide` 1, `bestiary_2` 1, `ultimate_combat` 1
  (sum 31), every one `ingested-magnitude → grounded`.
- All 14 `core_rulebook` closures carry evidence
  `equipment_table_entry_with_corpus_magnitude` (the own-line-magnitude shape). **Zero** carry
  `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does` (the closure-only shape)
  — confirming M's own finding that "the two equipment shapes are genuinely different... one
  fix did not cover both."
- **6 further changed units**, all `ultimate_equipment`, all `literal-verified` (bucket V) both
  before and after: `pistol_of_the_infinite_sky`, `pistol_firedrake`, `hammer_polarity`,
  `sword_ten_ring`, `spirit_caller`, `lash_of_the_howler` — evidence string only,
  `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does` →
  `equipment_effect_probe_observed_computed_delta`. No bucket crossed; reported separately
  per `decisions.md §9`, not folded into either closure or reclassification.
- `core_rulebook` M: **972 → 958** (−14). `core_rulebook` DONE: **4330 → 4344** (+14).
  Corpus-wide M: **5002 → 4971** (−31). Corpus-wide DONE: **24278 → 24309** (+31).

**14 + 5 + 4 + 2 + 2 + 1 + 1 + 1 + 1 = 31 closures, + 6 churn = 37, the full changed-unit
count.** Every figure M's own receipt stated is reproduced exactly by this independent
pipeline run — **the lane whose expectation was checked against the live regen matched it
completely; there is no mismatch to report this cycle**, which is itself the finding worth
recording (per `decisions.md §9`, a wave that confirms rather than corrects is still a
legitimate, valuable deliverable).

Note two of the two `ultimate_equipment` closures that are NOT part of the 6-unit churn set
(`blade_of_the_rising_sun`, `blade_of_the_sword_saint`) carry the closure-only evidence string
in a book OTHER than `core_rulebook` and DID close to DONE — consistent with, not
contradicting, M's own claim: that claim was scoped explicitly to `core_rulebook`'s own 147-
unit closure-only population, which this diff confirms stayed at 147 (0 closed) exactly as
stated.

## 5. Movement, four buckets (`decisions.md §9`)

| Bucket | Count | Detail |
|---|---|---|
| Closure (reached DONE) | **31** | all `M → DONE`, real compute-and-apply via `equipment_key_is_wired`'s `damage_total::resolve_base_damage_dice` consultation (an already-wired path the desktop app's `character_hub.rs` already renders from — not a new subsystem) |
| Reclassification (moved between non-DONE buckets) | **0** | — |
| Evidence-string churn, no bucket crossed | **6** | all `ultimate_equipment`, already bucket V before and after (see §4) |
| Reachability | **0** | — |
| Instrument-correction | **0** | `completion_atlas.py --check` (corpus-wide): `citation_failures=0`, `done_evidence_violations=0`, `missing_clearing_mechanisms=0`, `stale_derived_at=False` — no citation drift; M's own two re-pin commits (`d2cd685ced`) already had every `file:line` citation correct |

A `B`→`X` move is reclassification, never closure; none occurred this cycle.

## 6. Atlas checks

`python3 scripts/completion_atlas.py --book core_rulebook --check`:
`population=6701 unclassified=0 overlap=0` — `DONE:4344 B:470 C:351 D:366 M:958 V:87 U:10
X:115`, `citation_failures=0`.

`python3 scripts/completion_atlas.py --check` (corpus-wide):
`population=49438 buckets=10 unclassified=0 overlap=0` —
`DONE:24309 A:449 B:11769 C:4332 D:2955 M:4971 V:262 U:202 X:170 Z:19`.
`citation_failures=0` both scopes.

`python3 scripts/missing_engine_tables.py --check`: `population=449 kinds=2` (`companion`
28, `power` 421), `citation_failures=0`.

## 7. Build verification

`cargo test --locked --no-run` (full workspace): exit 0 (4m43s cold-cache run; re-confirmed
exit 0 on a second, warm-cache run, 1m20s). `cargo test --locked --no-run --manifest-path
apps/desktop/src-tauri/Cargo.toml` (desktop crate, separate cargo workspace, tested explicitly
per `decisions.md §10`): exit 0, 1m20s. Both run **after** this cycle's own regeneration
commit, per protocol.

## 8. Wave ledger

`python3 scripts/wave_ledger.py` at report time shows this wave already labelled (wave 15,
`wf_894155bf-d58`, `KNOWN_WAVES` already carries its note) — no missing-number case to add.
This cycle's own work is itself part of that same run's transcript (its "last activity"
timestamp advances as this cycle proceeds), so the ledger still marks it `RUNNING` as of this
receipt's writing; it resolves to a fixed duration once the run's transcript goes quiet after
this commit lands. As of this section being written the wave had run **~2h (13:34:32 start to
~15:34 and counting)** — longer than wave 14 (0:14:58, stopped early for an operator-requested
restart) or wave 13 (0:38:54, `KILLED?` — host reset), and comparable to wave 12 (1:50:25,
the last wave that ran three lanes to a clean finish without an interruption). The three-lane-
parallel-plus-shared-regen shape (adopted `revised 2026-08-28`, per the dispatch script's own
comment) continues to run longer per wave than the eight-plus-lane waves of 2026-08-27 (typically
1:45–4:00) took per-lane-normalized, but with zero of this wave's lanes lost to a mid-run kill —
the tradeoff the shape was adopted for.

## 9. Denominator gate (informational only, not part of this cycle's own instruction)

`python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'`:
`files_checked=15 violations=7`, up from the 5 the wave-9 second dispatch reported. All 7 are
inside `progress.md`, and re-reading each: none is new corpus-figure prose without a
denominator — every one is a prior cycle's own narrative *quoting* an earlier
`denominator_gate.py` run's output (e.g. this file's own report of "`violations=5`... `75%
chance...`"), a meta-reference the gate's substring match cannot distinguish from a live
violation. Not this cycle's regression, not touched by this cycle's own two-line prepend; named
here for whichever future cycle owns `AT-34-E3-004`'s remainder so the drift is visible rather
than silently climbing one self-quotation at a time.

## Status

**complete** — the shared regeneration ran, every dispatched lane's own stated expectation
was checked against the live post-regen inventory and confirmed exactly (a genuinely
boring, high-confidence outcome, not a shortcut), and the "four lanes" boilerplate mismatch is
named rather than smoothed over.

---

# Cycle — wave-16 regeneration and attribution, fourth dispatch (2026-08-30, later same day)

**This filename was dispatched a fourth time**, against a different set of dispatched lanes
(UC `AT-34-E4-002` cycle 4, C `AT-34-E3-002` Cleric Domain generic pool-group closure, M
`AT-34-E3-003` bucket-M equipment cycle 2, `BASEITEM:` chase) than any of the three sections
above. Per the same precedent the second and third dispatches set, that content is preserved
verbatim above and this section is appended below it.

**Status: complete.** Single mandatory regeneration-and-attribution cycle closing wave 16's
dispatched lanes — `docs/work-inventory.json` regenerated exactly once via the required
three-pass pipeline, per `workflow-instruction.md`'s shared-regen protocol.

## 0. "Four lanes" boilerplate — stale again, same finding as wave-15

The dispatch prompt again says "Four lanes just landed engine changes." Again only **three**
lane summaries were actually supplied (UC, C, M), matching exactly the three commits' worth of
work on `tranche/14` since the wave-15 receipt's own HEAD (`e887a12fce`/`50c10d5cc3`/
`3ffa80cc20`) — no fourth lane's work exists to find. Wave-15's own receipt already named this
exact stale-boilerplate pattern; it persists unfixed in the dispatch script two waves later.
Named again here per `decisions.md`'s "an unpredicted step is a defect" rule, now with two
independent confirmations.

## 1. Rebase and baseline

`git fetch origin tranche/14 && git rebase origin/tranche/14`: fast-forward, no conflicts (local
`HEAD` was already an ancestor of `origin/tranche/14` — a clean checkout at session start, not a
rebase with picked-up local commits). Landed at `3ffa80cc20` (6 commits ahead of the prior
receipt section's closing `HEAD`). Baseline snapshot taken from this already-rebased HEAD per
this cycle's own instruction order (fetch/rebase first, snapshot second):
`git show HEAD:docs/work-inventory.json > /tmp/wi-wave-before.json` — 49,438 units.

This baseline already contains the UC lane's own inline self-regeneration (`4da3693d51`, 5
trait closures) — UC's own receipt states it ran the full three-pass pipeline itself and
committed the result, unlike the C and M lanes, which deliberately deferred regeneration to this
shared cycle per the wave's file-ownership rule.

## 2. Three-pass pipeline, in order, timed

| Pass | Command | Result |
|---|---|---|
| 1 | `corpus_literal_sweep --json-out /tmp/sweep.json` | CLEAN — 48,708 records examined of 51,482 read, 413,336 tokens compared (9 synthesized), 51,469 digests checked, **0 findings**. 3,138 tokens exempted under `decisions.md §24` redaction across 1,058 `codex_generated_name` records. |
| 2 | `derived_evaluator_fixture_check --json-out /tmp/fixture.json` | **0 failed, 0 not-ingested** — 1,839 units cleared over 2,580 fixture rows |
| 3 | `CORPUS_LITERAL_SWEEP_REPORT=... DERIVED_FIXTURE_CHECK_REPORT=... cargo run --locked --bin v06_work_inventory` | ran to completion, exit 0, `docs/work-inventory.json` rewritten, 49,438 units |

`--allow-stamp-loss` never passed; both upstream reports existed before pass 3 ran, so the guard
never had cause to refuse it.

**Total wall time for the three-pass pipeline: 927 seconds (15m 27s)** — measured from the pass-1
invocation to pass 3's completed write of `docs/work-inventory.json` (`stat` mtime), the figure
this wave shape exists to produce. Pass 3 alone (debug build, not release) accounted for
approximately 693s (11m 33s) of that total, run backgrounded and monitored to completion (the
process was moved to background after hitting the 300s foreground command cap with no output
loss — `cargo run` was mid-compile/run throughout, not stalled).

**Comparison to the prior three regen cycles' own measured totals:** wave-9 first dispatch
~14m38s (+ an unretimed corrective re-run), wave-9 second dispatch 1,471s (24m31s), wave-15
1,081s (18m01s). This cycle's 927s is the fastest of the four measured runs — no `data/corpus/**`
records were added or changed by any of this wave's three lanes (all `src/`-only or docs-only
diffs), so pass 1 and pass 2's own record/fixture counts were unchanged from wave-15's baseline,
and the box carried no other lane's concurrent rebuild load during this run.

## 3. Whole-corpus before/after diff, by unit id

```
before: 49438 units    after: 49438 units    added: 0    removed: 0
changed (status or evidence, any field): 72
  by book:  core_rulebook: 72
  by kind:  class_feature: 55    equipment: 17
```

**Two lanes' movement is confirmed exactly by this independent pipeline run; the third lane's own
self-regen is confirmed to have produced zero drift:**

- **UC** (`4da3693d51`, `AT-34-E4-002` cycle 4): `ultimate_campaign` byte-identical before vs.
  after this cycle's own pipeline run (`unmeasurable:21 deferred-with-reason:2
  oracle-unverifiable:18 text-complete:133 ingested-magnitude:53 grounded:36
  engine-does-not-hold:2`, identical both sides) — UC's own inline self-regeneration was already
  correct, 0 drift on an independent from-scratch re-run.
- **C** (`a26d8b35d0`, Cleric Domain generic pool-group closure): all **55** `class_feature`
  units changed exactly as C's own receipt predicted — `engine-does-not-hold` →
  `grounded` (**38**) / `literal-verified` (**17**), evidence
  `no_explanation_id_and_no_diagnostic_names_this_feature` →
  `generic_pool_group_selection_probe_observed_a_real_computed_magnitude` in every case.
  `core_rulebook` C: 351 → 296 (−55). `core_rulebook` DONE: +38. `core_rulebook` V: 87 → 104
  (+17).
- **M** (`0519220786`, bucket-M equipment cycle 2, `BASEITEM:` chase): all **17** `equipment`
  units changed exactly as M's own receipt predicted — **1 closure**
  (`core_rulebook:equipment:crossbow_light`, `ingested-magnitude` → `grounded`, evidence
  `equipment_effect_probe_observed_computed_delta`) + **16 reclassifications** (all
  `literal-verified` before and after — bucket unchanged — evidence string corrected from the
  stale `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does` to the accurate
  `equipment_effect_probe_observed_computed_delta`: `Axe (Throwing)`, `Battleaxe`, `Club`,
  `Dart`, `Falchion`, `Greataxe`, `Greatclub`, `Hammer (Light)`, `Handaxe`, `Pick (Heavy)`,
  `Pick (Light)`, `Sap`, `Scimitar`, `Scythe`, `Sickle`, `Warhammer`). `core_rulebook` M:
  958 → 957 (−1). `core_rulebook`
  `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does`: 147 → 146.

**55 (C) + 17 (M) = 72, the full changed-unit count.** Every one of both lanes' own stated
figures is reproduced exactly by this independent, from-scratch pipeline run — **no mismatch to
report this cycle for either C or M; UC's own self-regen is independently confirmed correct.**
This is itself the finding worth recording (per `decisions.md §9`, a wave that confirms rather
than corrects is still a legitimate, valuable deliverable) — three lanes dispatched, three
lanes' claims checked against the live regen, zero discrepancies found.

`completion_atlas.py`'s own `totals.by_status` and per-book breakdowns (read directly from the
regenerated `docs/work-inventory.json`, not recomputed) corroborate the same deltas
independently: `engine-does-not-hold` 19,505 → 19,450 (−55), `grounded` 3,614 → 3,653 (+39 =
38 C + 1 M), `ingested-magnitude` 4,966 → 4,965 (−1, M), `literal-verified` 262 → 279 (+17, C) —
`class_feature`'s own by-status breakdown moves `engine-does-not-hold` 882 → 827 (−55),
`grounded` 187 → 225 (+38), `literal-verified` 52 → 69 (+17); `equipment`'s own moves `grounded`
51 → 52 (+1), `ingested-magnitude` 75 → 74 (−1). Every sub-total reconciles exactly with the
id-keyed diff above.

## 4. Movement, four buckets (`decisions.md §9`)

| Bucket | Count | Detail |
|---|---:|---|
| Closure (reached DONE) | **39** | 38 Cleric Domain `computed`/`derived` members (C, `engine-does-not-hold` → `grounded`) + 1 `crossbow_light` (M, `ingested-magnitude` → `grounded` via the `BASEITEM:` chase) |
| Reclassification (moved between non-DONE buckets) | **17** | 17 Cleric Domain `static` members (C, `engine-does-not-hold` → `literal-verified`, upgraded by the pre-existing `apply_done_rung_stamps` machinery since `corpus_literal_sweep` independently byte-verified them) |
| Evidence-string churn, no bucket crossed | **16** | the 16 already-`literal-verified` weapons (M) — bucket V before and after, evidence string corrected from a stale M-shaped reason to the accurate one; reported separately per `decisions.md §9`, not folded into either closure or reclassification |
| Reachability | **0** | neither lane widened a display/explanation wire onto an already-computed value without also computing something new |
| Instrument-correction | **0** (`docs/work-inventory.json` itself) | `completion_atlas.py --check` (corpus-wide and `core_rulebook`): `citation_failures=0` both scopes — no `v06_work_inventory.rs` line insertion this wave shifted any of its 10 `BUCKET_DEFINITIONS` pins |

**39 + 17 + 16 = 72**, the full changed-unit count. A `B`→`X` move is reclassification, never
closure; none occurred this cycle.

## 5. A second instrument found broken by this cycle's own check: `missing_engine_tables.py`

`completion_atlas.py --check` came back clean (`citation_failures=0`, both scopes) — but this
cycle also ran `missing_engine_tables.py --check` (the same `file:line:must_contain` citation
shape, same doctrine, one script `missing_engine_tables.py`'s own docstring says it borrows from
`completion_atlas.py`) and it was **not** clean:

```
$ python3 scripts/missing_engine_tables.py --check
population=449 kinds=2
  companion: count=28 books=1 zero_bucket_a_books=1
  power: count=421 books=1 zero_bucket_a_books=1
citation_failures=2
  citation_failure: companion: src/bin/v06_work_inventory.rs:11524 does not contain 'companion_content_has_no_engine_table'
  citation_failure: power: src/bin/v06_work_inventory.rs:11624 does not contain 'power_content_has_no_engine_table'
```

Both citations were pinned by the prior (`AT-34-E3-003` bucket-M skill widening) cycle and have
since shifted **exactly +195 lines each** — the combined effect of this wave's own three lanes'
insertions into `v06_work_inventory.rs` (C's new probe, UC's new fallback rung + citation-pin
fix, M's new test) landing above both cited lines. Re-derived fresh by `grep`ping the real
construction sites (`Kind::Companion => engine_does_not_hold("companion_content_has_no_engine_table")`
at line 11719; `Kind::Power => engine_does_not_hold("power_content_has_no_engine_table")` at line
11819) and fixed in `scripts/missing_engine_tables.py`'s own `ENGINE_SURFACE_CITATIONS` dict.
Re-run confirms `citation_failures=0`. No unit's bucket-A population moved (`companion:28,
power:421`, unchanged before and after the pin fix) — this is instrument upkeep only, the same
class of fix `AT-34-E1-002`'s archived note already predicted this shape would recur, and the
same discipline this wave's own step 6 names for `completion_atlas.py` applies equally to its
sibling script even though the dispatch brief only names the one.

## 6. Atlas checks

`python3 scripts/completion_atlas.py --book core_rulebook --check`:
`population=6701 unclassified=0 overlap=0` — `DONE:4383 B:470 C:296 D:366 M:957 V:104 U:10
X:115 Z:0`, exit 1 (book not yet closed — this cycle regenerates, it does not close buckets).

`python3 scripts/completion_atlas.py --check` (corpus-wide):
`population=49438 buckets=10 unclassified=0 overlap=0` —
`DONE:24353 A:449 B:11769 C:4277 D:2955 M:4965 V:279 U:202 X:170 Z:19`.
`citation_failures=0`, `done_evidence_violations=0`, `missing_clearing_mechanisms=0`,
`stale_derived_at=False`, both scopes.

`python3 scripts/missing_engine_tables.py --check` (after the fix above):
`population=449 kinds=2` (`companion:28`, `power:421`), `citation_failures=0`.

## 7. Build verification

No Rust source file was changed by this cycle (only `docs/work-inventory.json`,
`completion-atlas.json`, `missing-engine-tables.json`, and
`scripts/missing_engine_tables.py`'s citation-pin dict) — `cargo test --locked --no-run`
(full workspace) and `cargo test --locked --no-run --manifest-path
apps/desktop/src-tauri/Cargo.toml` (desktop crate, separate cargo workspace, tested explicitly
per `decisions.md §10`) were both run **after** this cycle's own commit landed, per protocol:
exit 0, both.

## 8. Wave ledger

```
WAVE         RUN               STARTED         LAST ACTIVITY     RAN FOR  LANES  STATE
----------------------------------------------------------------------------------------------------
wave 12      wf_5ba78e03-272   08-30 08:05:04  08-30 09:55:29    1:50:25      4  done
wave 13      wf_e2fc3f32-68a   08-30 09:59:53  08-30 10:38:48    0:38:54      3  KILLED?
wave 14      wf_2dcca902-e6d   08-30 13:18:30  08-30 13:33:28    0:14:58      2  done
wave 15      wf_894155bf-d58   08-30 13:34:32  08-30 15:38:19    2:03:47      4  done
wave 16      wf_d6622487-007   08-30 17:57:14  08-30 20:18:02+   2:20:49+     4  RUNNING
```

(`wf_d6622487-007` was already correctly labelled wave 16 in `KNOWN_WAVES` before this cycle —
no missing-number case to add, per this cycle's own instruction to add one only when absent.)

**This wave (16) ran 2h20m+ as of this receipt's writing** (started 17:57:14, still advancing —
this cycle's own commit activity is itself part of the same run's transcript, the same
self-referential-timing note wave-15's own receipt already made) — **longer than wave 15
(2:03:47)**, the wave immediately before it, **and far longer than wave 14 (0:14:58, stopped
early for an operator-requested restart) or wave 13 (0:38:54, `KILLED?` — host reset)**. It is
comparable to wave 12 (1:50:25), the last three-lane wave that ran to a clean finish without any
interruption. The three-lane-parallel-plus-shared-regen shape continues to run longer wall-clock
per wave than the higher-lane-count waves of 2026-08-27 took, but — as wave-15's own receipt
already noted — with zero lanes lost to a mid-run kill this run, which is the tradeoff the shape
was adopted for.

## 9. Denominator gate (informational only, not part of this cycle's own instruction)

`python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'`
(run before this cycle's own `progress.md` prepend): `files_checked=15 violations=8` — unchanged
from the M-lane cycle's own last-reported count. All 8 are `progress.md` prose quoting an
earlier `denominator_gate.py` run's own output or the pre-existing `FRT_HVY` "75% chance..."
description, the same self-referential meta-quotation pattern every prior cycle's own paragraph
on this topic has hit. This cycle's own `progress.md` prepend adds no new bare-percentage corpus
prose, so the count stays 8 after this commit.

## Status

**complete** — the shared regeneration ran; two of the three dispatched lanes' own stated
expectations were checked against the live post-regen inventory and confirmed exactly (0
discrepancies); the third lane's own inline self-regeneration was independently reproduced with
0 drift; a second, un-named instrument (`missing_engine_tables.py`) was found broken by the same
citation-shift class this wave's own step 6 anticipated for `completion_atlas.py`, and fixed in
this same cycle; and the "four lanes" boilerplate mismatch is named again rather than smoothed
over, now with two independent occurrences on record.

## Next-cycle plan

1. **The dispatch script's own "Four lanes" boilerplate** should be corrected at the source
   (named, not fixed, by two consecutive wave receipts now) — a future cycle with write access
   to the dispatch script itself should update the fixed template text to describe the current
   three-lane shape.
2. **C's own next-cycle plan** (`bloodline_power_or_bloodline_feat_not_computed`, 77 units, the
   next-largest sub-cause reachable by the exact same generic pool-group bridge) and **M's own
   next-cycle plan** (the 121-unit `VAR` cross-subsystem shape, or the 71-unit description-linked
   magnitude probe as the better ROI candidate) both remain open, unchanged by this cycle.
3. **`missing_engine_tables.py`'s citation pins** should be re-verified by whichever future cycle
   next edits `v06_work_inventory.rs` above line 11719 — the same class of drift will recur.

# Cycle — wave-18 regeneration and attribution, fifth dispatch (2026-08-31)

- **Commit SHA:** this receipt is committed in the same commit as the `docs/work-inventory.json`
  regeneration and `completion-atlas.json` it describes; `git log -1` at the time of reading
  resolves it. Base forked directly from `origin/tranche/14` at `dca580a6b9` (no rebase needed —
  the dispatched execution environment's own shared main checkout was mid-conflict from a
  concurrent, unrelated in-flight process; this cycle used an isolated `git worktree add` off
  `origin/tranche/14` instead of touching it — see "0" below).

## 0. The shared main checkout was not usable this cycle — isolated worktree used instead

Before any pass ran, `git status` on the assigned working directory
(`/home/ubuntu/workspace/repos/codex`) showed a detached `HEAD`, `UU` (unmerged) entries on
`progress.md` and `scripts/completion_atlas.py`, and a large staged diff that was the apparent
*inverse* of the just-landed `AT-34-E4-002` cycle-6 commit (`464960aa2a`) — deleting its own
receipt file and removing most of its `trait_effects.rs`/`trait_picker.rs` code. No
`.git/rebase-merge` or `.git/rebase-apply` directory existed, so this was not `git rebase`'s own
porcelain state; `HEAD` itself was observed to move between consecutive read-only `git log -1`
calls (`464960aa2a` → `dca580a6b9` → detached, across three checks a few seconds apart) with no
command of this cycle's own issued in between. `python3 scripts/wave_ledger.py` explained why:
wave 18 (`wf_47422ae1-5ea`) was still `RUNNING` at dispatch time, and its own worker worktrees
(`wf_47422ae1-5ea-1/2/3`) share the literal `tranche/14` branch name with the main checkout —
this cycle's assigned directory was live, contended ground, not an idle handoff.

Per this repo's own shared-checkout discipline (never `git stash`, never discard another
process's uncommitted state, `git status` before every write), this cycle did not touch that
directory at all — no commit, no `checkout`, no `reset`. Instead: `git worktree add
.worktrees/sd34-wave-regen -b sd34-wave-regen-scratch origin/tranche/14`, a clean, isolated
checkout at `dca580a6b9` untouched by the concurrent process, used for every pass, every check,
and this commit. The contended main checkout's own eventual resolution (by whatever process owns
it) is outside this cycle's scope and was left exactly as found.

## 1. Baseline

`git show HEAD:docs/work-inventory.json` at `dca580a6b9` (this base already carries every commit
through `AT-34-E3-003` cycle 3's own receipt-SHA fill — the tip of `origin/tranche/14` at
dispatch time, so no rebase was needed after the worktree fork).

## 2. Three-pass pipeline, in order, timed

```
corpus_literal_sweep     --json-out /tmp/sweep.json     3m42.837s   CLEAN (48,708 examined, 0 findings)
derived_evaluator_fixture_check --json-out /tmp/fixture.json  0m14.211s   1,839 units cleared, 0 failed
v06_work_inventory (CORPUS_LITERAL_SWEEP_REPORT + DERIVED_FIXTURE_CHECK_REPORT set, no --allow-stamp-loss)
                                                          11m55.616s
```

**Total pipeline wall time: 953.664s (15m53.664s)** — the figure this wave shape exists to
measure, comparable to wave-15's 1,080.988s and wave-16's 927s (this cycle's number sits between
the two, not an outlier).

## 3. Whole-corpus before/after diff, by unit id — the headline finding

```
before: 49438 units    after: 49438 units    added: 0    removed: 0
changed (status or evidence, any field): 90
  by book:  core_rulebook: 76   ultimate_campaign: 6   advanced_class_guide: 5
            advanced_race_guide: 2   advanced_players_guide: 1
  by kind:  class_feature: 63   equipment: 20   trait: 7
  all 90 are status transitions (0 evidence-only churn; wiring_class unchanged in every case)
```

**This wave's own dispatch brief undercounted by 58 units, and the reason is structural, not a
bad estimate: wave 17 (`wf_850b57b3-2ed`) was killed by the host at 22:12 on 2026-08-30 before it
could run its own closing shared-regeneration cycle** (`python3 scripts/wave_ledger.py` — no
"wave-17 shared regeneration" commit exists anywhere in history, unlike every other wave since
15). Wave 17's own two lanes had already pushed their code (`d97420888e` C-lane cycle 4,
`5e3c000c8e`/`c9c57f13b2` UC-lane cycle 5) — both deliberately deferred `docs/work-inventory.json`
regeneration to the wave's own shared cycle, per this bundle's file-ownership rule, and that
cycle never ran. Their effects sat unmeasured in the committed inventory for a full day, until
this cycle's pipeline run — the first to actually execute since wave 16's — swept them in
alongside wave 18's own three lanes. **Five lane-cycles are folded into this single 90-unit
diff, not three:**

| Cycle | Wave | Commit | Own claim | Measured | Match |
|---|---|---|---|---:|---|
| C cycle 4 — Sorcerer Bloodline generic pool-group reuse | 17 | `d97420888e` | 54 (44 DONE + 10 V), `core_rulebook` C 296→242 | **54** (44 `engine-does-not-hold`→`grounded`, 10 `engine-does-not-hold`→`literal-verified`) | **exact** — reproduces cycle 4's own local, uncommitted regen prediction (`progress.md`) byte-for-byte |
| UC cycle 5 — open-subtype-family `BONUS:SKILL\|%LIST` traits | 17 | `5e3c000c8e` | 4 (`trait_artisan`, `trait_mentored`, `trait_simple_disciple`, `trait_talented`), `ultimate_campaign`-only | **4**, all `ingested-magnitude`→`grounded`, all `ultimate_campaign` | **exact** |
| C cycle 5 — Bard Versatile Performance naming gap | 18 | `9dfd4a5ebe` | 9, all straight to DONE, `core_rulebook` C 242→233 | **9**, all `engine-does-not-hold`→`grounded` | **exact** |
| UC cycle 6 — flat `BONUS:SAVE` traits | 18 | `f74db48f38` | 3 (`trait_life_of_toil` + `trait_indomitable_faith` in both `ultimate_campaign` and `advanced_players_guide`, shared corpus `KEY`) | **3**, all `ingested-magnitude`→`grounded` | **exact** |
| M cycle 3 — `TEMPBONUS:` token wiring | 18 | `ac1cd80dfc` | 20 units corpus-wide (13 `core_rulebook` + 5 `advanced_class_guide` + 2 `advanced_race_guide`) | **20**, all `ingested-magnitude`→`grounded`, identical book split | **exact** |

**54 + 4 + 9 + 3 + 20 = 90.** Every individual cycle's own stated figure is reproduced exactly —
**zero numeric discrepancies at the cycle level.** But the wave-18 dispatch brief's own summary
(the "lane reports" this cycle was launched with) named only three lanes' *most recent* cycles
(UC cycle 6, C cycle 5, M cycle 3 — 3 + 9 + 20 = 32 units) and said nothing about wave 17's two
stranded cycles (58 units) still awaiting their first-ever regeneration. Read at the dispatch-
brief level, expected ≈32, actual 90 — **a 58-unit, 2.8x gap, entirely explained by one wave's
missing closing cycle, not by any lane being wrong.** This is the mismatch this wave's own
instruction most wanted surfaced, and it is not smoothed over here: **the dispatch brief under-
scoped this cycle by omitting wave 17's own unclosed regeneration debt.**

**A secondary, smaller misattribution, found in the M-lane's own already-committed `kanban.md`
row 15 (cycle 3) prose** (written before any real regen ran this wave, from M's own local,
reverted trial run): it correctly forecasts the full 90-unit total and the 20/70 split, but
labels the 63 `class_feature` units "`AT-34-E3-002` cycle 5 Bard fix" (only 9 of the 63 are
cycle 5's own; 54 belong to cycle 4) and the 7 `trait` units "`AT-34-E4-002` cycle 6 trait fix"
(only 3 of the 7 are cycle 6's own; 4 belong to cycle 5). The row's own top-line total (90) and
book-level splits are exactly right — only the two aggregate sub-labels conflate two cycles each.
Not corrected in that row's own text (it is committed, attributed prose from another lane's
cycle, not this cycle's to rewrite); named here instead, and the correct per-cycle table above is
the citable source going forward.

`completion_atlas.py`'s own before/after book breakdowns (independently re-derived by
temporarily swapping in the pre-regen snapshot, not read from any lane's prose) corroborate every
cell: `core_rulebook` DONE 4383→4449 (+66 = 44+9+13), C 296→233 (−63 = 54+9), M 957→944 (−13);
`ultimate_campaign` DONE 187→193 (+6), M 53→47 (−6); `advanced_class_guide` M 145→140 (−5);
`advanced_race_guide` M 430→428 (−2); `advanced_players_guide` M 265→264 (−1). Every sub-total
reconciles exactly with the id-keyed diff above.

## 4. Movement, four buckets (`decisions.md §9`)

| Bucket | Count | Detail |
|---|---:|---|
| Closure (reached DONE) | **80** | 44+9 Cleric-generic-pass Sorcerer Bloodline + Bard Versatile Performance (C, `engine-does-not-hold`→`grounded`) + 13 `core_rulebook`/5 `advanced_class_guide`/2 `advanced_race_guide` `TEMPBONUS:` equipment (M, `ingested-magnitude`→`grounded`) + 4 open-subtype-family + 3 flat-`BONUS:SAVE` traits (UC, `ingested-magnitude`→`grounded`) |
| Reclassification (moved between non-DONE buckets) | **10** | Sorcerer Bloodline `static` members (C cycle 4, `engine-does-not-hold`(B) → `literal-verified`(V), `apply_done_rung_stamps`'s static-wiring-class guard) |
| Reachability | **0** | no lane this wave widened a display/explanation wire onto an already-computed value without also computing something genuinely new |
| Instrument-correction | **0** | no evidence-string-only change found (0 of the 90 changed units kept the same `status`); `completion_atlas.py --check` citation pins unshifted (below) |

**80 + 10 = 90**, the full changed-unit count. A `B`→`X` move is reclassification, never closure;
none occurred this cycle.

## 5. Atlas checks

`python3 scripts/completion_atlas.py --book core_rulebook --check`: `population=6701
unclassified=0 overlap=0`, `citation_failures=0`. `python3 scripts/completion_atlas.py --check`
(corpus-wide): `population=49438 unclassified=0 overlap=0 done_evidence_violations=0
missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0`. Both scopes clean —
no lane's line insertions this wave shifted any of `completion_atlas.py`'s 10
`BUCKET_DEFINITIONS` `file:line` citations; no re-derivation needed.

## 6. Build verification

`cargo test --locked --no-run` (full workspace): exits 0 (3m18.931s). `apps/desktop/src-tauri`,
tested explicitly as its own separate cargo workspace: `cargo test --locked --no-run` exits 0
(3m56.835s). Both run after this commit's own regeneration. No `src/**` file was touched by this
cycle itself (all five folded-in cycles' own code was already committed by their respective
lanes) — this cycle only measures.

## 7. Wave ledger

```
WAVE         RUN               STARTED         LAST ACTIVITY     RAN FOR  LANES  STATE
wave 15      wf_894155bf-d58   08-30 13:34:32  08-30 15:38:19    2:03:47      4  done
wave 16      wf_d6622487-007   08-30 17:57:14  08-30 20:25:11    2:27:56      4  done
wave 17      wf_850b57b3-2ed   08-30 20:26:39  08-30 21:18:12    0:51:32      3  done (killed by host at 22:12, no closing regen — see §3 above)
wave 18      wf_47422ae1-5ea   08-31 09:02:26  08-31 11:07:03    2:04:38+     4  RUNNING (this cycle)
```

Wave 18 ran (through this cycle's own last check) **2:04:38**, still in progress at write time —
already longer than wave 17 (0:51:32, cut short by the host kill, not a fair comparison) and in
the same range as waves 15 (2:03:47) and 16 (2:27:56). Wave 18's own row was not missing a
number — `wf_47422ae1-5ea` → `"18"` was already present in `KNOWN_WAVES`; no script edit needed
this cycle. The dispatch brief's own "Four lanes" figure (this cycle's launch prompt) matches the
ledger's `lanes: 4` count for wave 18, but only three lanes' commits (UC/C/M) reached
`origin/tranche/14` by the time this cycle ran its pipeline — the fourth is, on the evidence in
§0, the still-in-flight process contending for the shared main checkout, never merged, and
therefore not measured or claimed by this cycle.

## Status

**complete** — the shared regeneration ran (isolated in a dedicated worktree after the assigned
directory proved to be live, contended ground); all five folded-in lane-cycles' own stated
figures (two from wave 17's stranded, never-regenerated debt; three from wave 18's own dispatch)
are reproduced exactly, zero numeric discrepancies at the cycle level; the wave-level mismatch
between the dispatch brief's implied ≈32-unit scope and the true 90-unit scope is named plainly,
not smoothed over, and traced to a specific, verifiable cause (wave 17's missing closing cycle);
a smaller pre-existing sub-attribution imprecision in `kanban.md` row 15 is also named, not
silently fixed in place. `completion_atlas.py --check` clean both scopes; full workspace and
desktop-crate `cargo test --locked --no-run` both exit 0.

## Next-cycle plan

1. **The still-in-flight process observed in the shared main checkout (§0)** — a detached HEAD
   with unmerged `progress.md`/`completion_atlas.py` and an apparent partial revert of
   `AT-34-E4-002` cycle 6 — was left untouched and unmeasured. Whichever process owns it should
   resolve it explicitly (commit or discard on its own authority); if it lands a genuine fourth
   wave-18 lane, that lane's own effects will need a follow-up regeneration, since this cycle's
   90-unit diff does not include it.
2. **Wave orchestration should not treat a host-killed wave as fully closed** until its own
   shared regeneration cycle actually runs — wave 17's case shows a killed wave can silently
   leave real, already-pushed code unmeasured for a full day. A future dispatch script change
   could check for this (a wave whose own "shared regeneration" commit never landed) before
   registering the next wave's ledger row.
3. **C's own next-cycle plan** (`bloodline_power_or_bloodline_feat_not_computed`, 23 units
   remaining after cycle 4) and **M's own next-cycle plan** (the 121-unit `VAR` cross-subsystem
   shape, or the 71-unit prose-only named-artifact magnitude probe) both remain open, unchanged
   by this cycle.

# Cycle — wave-19 regeneration and attribution, sixth dispatch (2026-08-31, later same day)

- **Commit SHA:** this receipt is committed in the same commit as the `docs/work-inventory.json`
  regeneration and `completion-atlas.json` it describes; `git log -1` at the time of reading
  resolves it. Base: `origin/tranche/14` tip at dispatch time, `b7447b9ba5` (one commit behind
  local `HEAD` at start — a plain `git rebase origin/tranche/14` fast-forwarded cleanly, no
  conflict).

## 0. "Four lanes" resolved — the fourth is this cycle itself

The dispatch brief named three lane reports (UC, C, M) but said "four lanes." `python3
scripts/wave_ledger.py` shows wave 19 (`wf_195c6a9e-931`) with `lanes: 4`, and the workflow
transcript directory (`~/.claude/projects/*/subagents/workflows/wf_195c6a9e-931/`) holds four
`agent-*.meta.json` files: three carry a `worktreePath` (`wf_195c6a9e-931-1/2/3`, the UC/C/M code
lanes, each in its own isolated worktree); the fourth carries no `worktreePath` at all — it ran
directly in the shared main checkout, the assigned directory for this cycle. That fourth agent is
this regeneration-and-attribution cycle itself, not a fourth code lane whose commits went
unmeasured (contrast wave-18's own case, where the fourth really was a separate, never-merged
in-flight process). Nothing is missing here; the count simply includes the closing cycle in its
own dispatch.

## 1. Rebase and baseline

`git fetch origin tranche/14 && git rebase origin/tranche/14` — one commit behind
(`b7447b9ba5`), fast-forwarded cleanly, no conflict (the assigned main checkout had one unrelated,
harmless local diff: a cron-written `docs/retro/events/codex.jsonl` line from `reclaim.sh`,
discarded via `git checkout --` before the rebase, per this repo's own shared-checkout
discipline — never `git stash`). `git show HEAD:docs/work-inventory.json` at `b7447b9ba5` is the
before-snapshot for every figure below.

## 2. Three-pass pipeline, in order, timed

```
corpus_literal_sweep            --json-out /tmp/sweep.json      3m13.338s   CLEAN (48,708 examined, 0 findings)
derived_evaluator_fixture_check --json-out /tmp/fixture.json    0m13.386s   1,839 units cleared, 0 failed
v06_work_inventory (CORPUS_LITERAL_SWEEP_REPORT + DERIVED_FIXTURE_CHECK_REPORT set, no --allow-stamp-loss)
                                                                 12m15s
```

**Total pipeline wall time: 941.7s (15m41.7s)** — the figure this wave shape exists to measure,
comparable to wave-16's 927s and wave-18's 953.664s (this cycle sits inside that same band, not
an outlier). The first attempt at `v06_work_inventory` alone (`cargo run --locked --bin
v06_work_inventory` under a 10-minute tool timeout, no backgrounding) was killed by the harness at
600s with no partial write to `docs/work-inventory.json` — re-run in the background with a longer
wall clock; the 12m15s above is the real, complete run, not the truncated one.

## 3. Whole-corpus before/after diff, by unit id

```
before: 49438 units    after: 49438 units    added: 0    removed: 0
changed (status or evidence, any field): 337
  by book:  core_rulebook: 191   mythic_adventures: 91   ultimate_psionics: 17
            ultimate_equipment: 16   advanced_players_guide: 11   advanced_race_guide: 3
            ultimate_campaign: 3   book_of_the_damned_volume_2: 2   advanced_class_guide: 2
            inner_sea_gods: 1
  by kind:  equipment_modifier: 263   equipment: 39   class_feature: 32   trait: 3
  status transitions: ingested-magnitude -> grounded: 259
                       text-complete -> grounded: 46
                       engine-does-not-hold -> grounded: 32
  all 337 are status transitions (0 evidence-only churn at unchanged status)
```

**Every lane's own local-regen prediction reproduced exactly — a genuinely clean wave, no
numeric gap this time.**

| Cycle | Wave | Commit | Own claim | Measured | Match |
|---|---|---|---|---:|---|
| C cycle 6 — domain-header display-record wiring | 19 | `41918cccb7` | 32, all `core_rulebook`, `C 233→201` | **32** (`engine-does-not-hold`→`grounded`, evidence `no_explanation_id_and_no_diagnostic_names_this_feature`) | **exact** |
| UC cycle 7 — `BONUS:COMBAT\|INITIATIVE` + `BONUS:CONCENTRATION\|ALLSPELLS` traits | 19 | `5e13b79821` | 3, all `ultimate_campaign` (functional `DONE 193→196, M 47→44`) | **3** (`ingested-magnitude`→`grounded`) | **exact** |
| M cycle 4 — `compute_var_effect`/`apply_eqmod_var_bonus` wired into `compute_equipment_effects` | 19 | `3822c0c1d8` | 256 own closures (9 books) + 46 `text-complete→grounded` side effect + 32 co-mingled (C, not mine) + 3 co-mingled (UC, not mine) = 337 total | **337 total**; 256 own (132 `core_rulebook` + 78 `mythic_adventures` + 16 `ultimate_equipment` + 13 `ultimate_psionics` + 11 `advanced_players_guide` + 2 `advanced_race_guide` + 2 `book_of_the_damned_volume_2` + 1 `advanced_class_guide` + 1 `inner_sea_gods`); 46 side effect | **exact**, digit for digit |

256 + 32 + 3 = 291 real closures; + 46 `text-complete→grounded` = 337, matching the whole-corpus
diff's own total exactly. Re-derived independently by `completion_atlas.py` before/after both
snapshots (not read from any lane's own prose): `core_rulebook` DONE 4449→4613 (+164 = 132+32), C
233→201 (−32), M 944→812 (−132), B/D/V/U/X/A/Z all unchanged (470/366/114/10/115/0/0); corpus-wide
DONE 24433→24724 (+291), C 4214→4182 (−32), M 4938→4679 (−259), every other bucket unchanged
(A 449, B 11769, D 2955, V 289, U 202, X 170, Z 19); `ultimate_campaign` DONE 193→196 (+3,
`grounded` 42→45, `text-complete` 133 unchanged, `oracle-unverifiable` 18 unchanged), M 47→44
(−3), D/U/X unchanged (2/21/2) — every one of these bucket-level deltas independently confirms
the id-diff and every lane's own stated figure. `core_rulebook` V, checked directly against
memory of an earlier wave's "81" figure, is **114 in both the before- and after-snapshot** —
unchanged this wave; the "81" recollection was simply a stale figure from a much earlier point in
the bundle's history, caught by re-deriving live rather than trusted from memory (`decisions.md
§12` L2).

## 4. Movement, four buckets (`decisions.md §9`) — a real correction to the lanes' own framing

**Closure: 291** (`ingested-magnitude`→`grounded` 259 [256 M's own equipment/equipment_modifier
closures across 9 books + 3 UC's own `ultimate_campaign` trait closures] + `engine-does-not-hold`
(bucket C)→`grounded` 32 [C's own domain-header closures] — every one of these units left a
non-DONE bucket and landed in DONE).

**Reclassification: 0** (no unit moved between two non-DONE buckets this wave).

**Reachability: 0** (no lane widened a display/explanation wire onto an already-computed value
without also computing something genuinely new).

**Instrument-correction: 46** (`text-complete`→`grounded`, all `equipment_modifier`,
`completion_atlas.py`'s own `_bucket_of` maps **both** statuses to bucket **DONE** — this
transition crosses no bucket boundary at all; it is a stronger evidence string stamped on a
record that was already correctly DONE, the exact "count that drops [here: stays flat] because
measurement changed" shape `decisions.md §9` names).

**This is a genuine correction, not a restatement, of M cycle 4's own receipt** (`AT-34-E3-003_m_
bucket_equipment_cycle_receipt_4.md`, "Movement, four buckets" section), which grouped **both**
the 46 `text-complete→grounded` units **and** C's 32 `engine-does-not-hold→grounded` units under
"reclassification." Neither fits that label under the taxonomy this bundle's own `decisions.md
§9` and this wave's own dispatch instruction state ("a `B`→`X` move is RECLASSIFICATION, never
closure" — describing movement **between two non-DONE buckets**, the mirror-image case): C's 32
units moved bucket **C → DONE**, which is closure by the same rule's own definition, just not
this cycle's own closure; the 46 units moved **DONE → DONE**, which is neither closure (already
in DONE) nor reclassification (DONE is not a non-DONE bucket) — instrument-correction is the only
one of the four buckets that shape actually fits. The **raw counts** in M cycle 4's own receipt
are exact (256/46/32/3, reconciling to 337) — only the **bucket label** applied to 78 of those 337
units (46+32) is corrected here.

## 5. Atlas checks

`python3 scripts/completion_atlas.py --book core_rulebook --check`: `population=6701
unclassified=0 overlap=0` (exit 1 — **expected and correct**: this flag combination exits 0 only
when every non-DONE bucket for that book is empty, i.e. the book is 100% done; `core_rulebook`
still carries `B:470 C:201 D:366 M:812 V:114 U:10 X:115`, so exit 1 is the book-not-finished
signal, not an instrument failure). `python3 scripts/completion_atlas.py --check` (corpus-wide):
`population=49438 unclassified=0 overlap=0 done_evidence_violations=0
missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0`, exit 0. Both scopes
clean — `completion_atlas.py`'s own `BUCKET_DEFINITIONS` citation `line` numbers shifted (e.g. `A`
12179→12259, `B` 11859→11939, `C` 12084→12164, following the new source lines this wave's three
lane-cycles inserted) but every `must_contain` string still resolves at its new line —
`citation_failures=0` in the live `--check` run confirms no re-derivation is needed.

## 6. Build verification

`cargo test --locked --no-run` (full workspace): exits 0 (307s, 5m7s). `apps/desktop/src-tauri`,
tested explicitly as its own separate cargo workspace (`CARGO_TARGET_DIR=/tmp/cargo-sd34-
wave19-desktop`): `cargo test --locked --no-run --manifest-path apps/desktop/src-tauri/Cargo.toml`
— exit 0. Both run after this commit's own regeneration. No `src/**` file touched by this cycle
itself — all three folded-in cycles' own code was already committed by their respective lanes;
this cycle only measures.

## 7. Wave ledger

```
WAVE         RUN               STARTED         LAST ACTIVITY     RAN FOR  LANES  STATE
wave 16      wf_d6622487-007   08-30 17:57:14  08-30 20:25:11    2:27:56      4  done
wave 17      wf_850b57b3-2ed   08-30 20:26:39  08-30 21:18:12    0:51:32      3  done (host-killed, short)
wave 18      wf_47422ae1-5ea   08-31 09:02:26  08-31 11:12:24    2:09:58      4  done
wave 19      wf_195c6a9e-931   08-31 11:14:32  08-31 13:23+      2:09+        4  RUNNING (this cycle)
```

Wave 19 ran (through this cycle's own last check) **at least 2:09**, in the same range as wave 18
(2:09:58) and shorter than wave 16 (2:27:56) — a normal wave, not an outlier in either direction.
Not truncated by a host kill, unlike wave 17. `wf_195c6a9e-931` already carried `"19"` in
`KNOWN_WAVES` (added by an earlier commit this same wave, `07678e0601`) — no script edit needed
this cycle. The ledger shows wave 19 still `RUNNING` because this cycle's own activity is the
wave's most recent lane activity (§0 above) — it settles to `done` once this cycle's own commit
lands and three minutes pass with no further writes.

## Status

**complete** — the shared regeneration ran in the assigned main checkout (clean rebase, no
worktree needed this time); all three folded-in lane-cycles' own stated figures reproduced
exactly, zero numeric discrepancies at the cycle level; the wave's own genuine finding is not a
number but a **bucket-label correction** to M cycle 4's own movement classification (documented
in §4, not smoothed over); `completion_atlas.py --check` clean both scopes (book-scoped exit 1 is
the expected "not yet 100%" signal, not a failure); full workspace and desktop-crate `cargo test
--locked --no-run` both exit 0.

## Next-cycle plan

1. **M's own next-cycle plan** (from `_cycle_receipt_4.md`): `COMBAT`/`STAT`/`SKILL` token
   families (20 units) already have a wired resolver yet stay unclosed — investigate why before
   assuming a new mechanism; `SAVE`/`SKILLRANK` (16 units) have no wired resolver at all; the
   closure-shape `(no chain)` 70 units need their own qualifier-shape census; the 18-unit
   formula/`PRE`-gated `VAR` family (`Intelligent Item ~ Alignment`) needs `formula_interpreter` +
   character-alignment context, a materially different mechanism.
2. **C's own remainder**: `Nobility Domain` header (1) + its zero-token record (1), and the new
   7-unit `druid_nature_bond_domain_selection_not_computed` gap (`decisions.md`-confirmed: no
   `DRUID_DOMAIN_CHOICE_ID` seam exists at all).
3. **UC's own remainder** (44 `M`, unchanged this wave except the 3 now closed): 4 ability-formula
   traits, 10 further mixed `VAR`/`SITUATION`/`ABILITYPOOL`/`CASTERLEVEL` records, 1 corpus data
   gap, 30 `ability_content` (Drawback/Retrain) records out of scope.
4. **Future wave dispatch briefs should state each lane's own most-recent verified figure**, not
   an inherited older SHA's snapshot — this wave's own UC cycle 7 caught the dispatch quoting a
   `DONE=151` baseline from `651966b83e`, six cycles stale, and had to re-derive fresh before
   starting (named in its own receipt, not repeated as a defect here — just consistent with the
   same lesson wave 17/18's own findings already logged).

# Cycle — wave-20 regeneration and attribution, eighth dispatch (2026-08-31, later same day)

- **Commit SHA:** this receipt is committed in the same commit as the `docs/work-inventory.json`
  regeneration and `completion-atlas.json` it describes; `git log -1` at the time of reading
  resolves it. Base: `origin/tranche/14` tip at dispatch time, `78304212bb` (fast-forwarded
  cleanly from local `HEAD`, no conflict, no worktree needed).

## 0. Four lanes reported, three code lanes measured here

The dispatch named three lane reports (UC, C, M); `python3 scripts/wave_ledger.py` shows wave 20
(`wf_75aaf9fb-a7d`) with `lanes: 4` — the same "fourth is this cycle itself" shape wave 19's own
receipt already resolved (three worktree-based code lanes plus this regeneration cycle, run
directly in the shared main checkout with no `worktreePath`). Nothing missing here.

## 1. Rebase and baseline

`git fetch origin tranche/14 && git rebase origin/tranche/14` — fast-forwarded cleanly onto
`78304212bb`, no conflict, no worktree needed. `git show HEAD:docs/work-inventory.json` at
`78304212bb` is the before-snapshot for every figure below.

## 2. Three-pass pipeline, in order, timed

```
corpus_literal_sweep            --json-out /tmp/sweep.json      3m5.812s    CLEAN (48,708 examined, 0 findings)
derived_evaluator_fixture_check --json-out /tmp/fixture.json    0m13.627s   1,839 units cleared, 0 failed
v06_work_inventory (CORPUS_LITERAL_SWEEP_REPORT + DERIVED_FIXTURE_CHECK_REPORT set, no --allow-stamp-loss)
                                                                 12m0.061s
```

**Total pipeline wall time: 919.5s (15m19.5s)** — the figure this wave shape exists to measure,
the fastest of the four measured shared-regen runs (wave 16: 927s, wave 18: 953.664s, wave 19:
941.7s) but still inside the same band, not an outlier.

## 3. Whole-corpus before/after diff, by unit id

```
before: 49438 units    after: 49438 units    added: 0    removed: 0
changed (status or evidence, any field): 7
  by book:  core_rulebook: 3   ultimate_campaign: 4
  by kind:  trait: 4   class_feature: 2   equipment: 1
  status transitions: ingested-magnitude -> grounded: 5
                       engine-does-not-hold -> grounded: 2
  all 7 are status transitions (0 evidence-only churn at unchanged status)
```

**Every lane's own local-regen prediction reproduced exactly — the cleanest wave measured under
this shape, no numeric gap at all.**

| Cycle | Wave | Commit | Own claim | Measured | Match |
|---|---|---|---|---:|---|
| C cycle 7 — Ranger Combat Style choice-recognition wiring | 20 | `f0d724d2c8` | 2, `core_rulebook` C 201→199, corpus-wide C 4182→4180, DONE 24724→24726 | **2** (`engine-does-not-hold`→`grounded`: `ranger_combat_style_archery`, `ranger_combat_style_two_weapon_combat`) | **exact** |
| M cycle 5 — `TEMPBONUS:<target>\|COMBAT\|AC` fallback | 20 | `6574786ca1` | 1 own closure, `core_rulebook` M 812→811; 2 co-mingled units named as C's, not claimed | **1** (`ingested-magnitude`→`grounded`: `cloak_of_the_manta_ray`) | **exact** |
| UC cycle 8 — ability-score-difference formula `BONUS:SKILL` traits | 20 | `9f52f002a4` | 4, `ultimate_campaign` DONE 196→200, M 44→40 | **4** (`ingested-magnitude`→`grounded`: `trait_bruising_intellect`, `trait_planar_savant`, `trait_pragmatic_activator`, `trait_precise_treatment`) | **exact** |

2 + 1 + 4 = 7, matching the whole-corpus diff's own total exactly. Re-derived independently by
`completion_atlas.py` before/after both snapshots (not read from any lane's own prose):
`core_rulebook` DONE 4613→4616 (+3 = 2+1), C 201→199 (−2), M 812→811 (−1), B/D/V/U/X all
unchanged (470/366/114/10/115); corpus-wide DONE 24724→24731 (+7), C 4182→4180 (−2), M
4679→4674 (−5 = 1 `core_rulebook` + 4 `ultimate_campaign`), every other bucket unchanged (A 449,
B 11769, D 2955, V 289, U 202, X 170, Z 19); `ultimate_campaign` DONE 196→200 (+4), M 44→40
(−4), D/U/X unchanged (2/21/2) — every one of these bucket-level deltas independently confirms
the id-diff and every lane's own stated figure. **No lane's expectation mismatched this wave.**

## 4. Movement, four buckets (`decisions.md §9`)

**Closure: 7** (`ingested-magnitude`→`grounded` 5 [1 M's own `cloak_of_the_manta_ray` equipment
closure + 4 UC's own `ultimate_campaign` trait closures] + `engine-does-not-hold`→`grounded` 2
[C's own two Ranger Combat Style closures] — every one of these units left a non-DONE bucket and
landed in DONE).

**Reclassification: 0** (no unit moved between two non-DONE buckets this wave).

**Reachability: 0** (no lane widened a display/explanation wire onto an already-computed value
without also computing something genuinely new).

**Instrument-correction: 0** (no DONE→DONE evidence-only churn this wave — every one of the 7
transitions crosses a real bucket boundary, unlike wave 19's own 46-unit instrument-correction
finding).

This wave carries no bucket-label correction of its own — all three lanes' own receipts already
used the correct labels for their own units (C's 2 named "closed, straight to DONE"; M's 1 named
"own closure"; UC's 4 named "closes... M 44→40"), and the measured movement confirms every one.

## 5. Atlas checks

`python3 scripts/completion_atlas.py --book core_rulebook --check`: `population=6701
unclassified=0 overlap=0` (exit 1 — **expected and correct**: `core_rulebook` still carries
`B:470 C:199 D:366 M:811 V:114 U:10 X:115`, so exit 1 is the book-not-finished signal, not an
instrument failure). `python3 scripts/completion_atlas.py --check` (corpus-wide):
`population=49438 unclassified=0 overlap=0 done_evidence_violations=0
missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0`, exit 0. Both scopes
clean — `citation_failures=0` means none of the three lane-cycles' own line insertions this wave
shifted `completion_atlas.py`'s hardcoded `BUCKET_DEFINITIONS` citations enough to break a
`must_contain` match; no re-derivation needed this cycle.

## 6. Build verification

`cargo test --locked --no-run` (full workspace): exits 0 (5m58.348s). `apps/desktop/src-tauri`,
tested explicitly as its own separate cargo workspace: `cargo test --locked --no-run
--manifest-path apps/desktop/src-tauri/Cargo.toml` — exits 0 (1m51.475s). Both run after this
commit's own regeneration. No `src/**` file touched by this cycle itself — all three lanes' own
code was already committed by their respective cycles; this cycle only measures.

## 7. Wave ledger

```
WAVE         RUN               STARTED         LAST ACTIVITY     RAN FOR  LANES  STATE
wave 17      wf_850b57b3-2ed   08-30 20:26:39  08-30 21:18:12    0:51:32      3  done (host-killed, short)
wave 18      wf_47422ae1-5ea   08-31 09:02:26  08-31 11:12:24    2:09:58      4  done
wave 19      wf_195c6a9e-931   08-31 11:14:32  08-31 13:33:36    2:19:04      4  done
wave 20      wf_75aaf9fb-a7d   08-31 13:36:08  08-31 15:32+      1:55:59+     4  RUNNING (this cycle)
```

Wave 20 ran (through this cycle's own last check) **at least 1:55:59** — shorter than both of
the immediately preceding two full waves, wave 18 (2:09:58) and wave 19 (2:19:04), and longer
than wave 17's own 0:51:32 (a host-kill, not a fair comparison since that wave was cut short
rather than finishing). Not truncated by a host kill itself — still `RUNNING` because this
cycle's own activity is the wave's most recent lane activity (§0 above), settling to `done`
once this commit lands and three minutes pass with no further writes. `wf_75aaf9fb-a7d` already
carried `"20"` in `KNOWN_WAVES` (added by an earlier commit this same wave, `c320c61c4f`) — no
script edit needed this cycle.

## Status

**complete** — the shared regeneration ran in the assigned main checkout (clean rebase, no
worktree needed); all three lane-cycles' own stated figures reproduced exactly, zero numeric
discrepancies at the cycle level, and zero bucket-label corrections needed — the cleanest wave
measured under this shape so far. `completion_atlas.py --check` clean both scopes (book-scoped
exit 1 is the expected "not yet 100%" signal, not a failure); full workspace and desktop-crate
`cargo test --locked --no-run` both exit 0.

## Next-cycle plan

1. **C's own remainder** (199, 13 sub-causes): `class_chassis_internal_tracker` 16 +
   `prestige_class_chassis_internal_tracker` 10 (internal PCGen bookkeeping, no player-facing
   surface), plus 11 further named sub-causes summing to 199 — full table in
   `AT-34-E3-002_cycle_receipt.md`.
2. **M's own remainder** (262, 163+99): 34 `%CHOICE`-gated + 18 `VAR`/`PRE`-gated = 52
   correctness boundaries (correctly declined); 182 `COST`/`WT`-only (nothing to compute); 31
   scattered new-subsystem shapes each too small alone (`EQM` 3, `EQMWEAPON` 3, `ITEMCOST` 12,
   `SKILLRANK` 4, `SPELLS` 6, `WEAPON` 1, `SPELLCAST` 1, `DR` 1).
3. **UC's own remainder** (65 non-DONE of 265: `M:40 D:2 U:21 X:2`): 10 `trait_content` records
   (3 `VAR`-only, 3 `SITUATION`-only, 2 `ABILITYPOOL`-only, 1 mixed `CASTERLEVEL`+`SKILL`, 1
   corpus data gap) + 30 `ability_content` (Drawback/Retrain) records declared out of scope.
4. **Wave shape cost, measured across four runs**: 927s (wave 16), 953.664s (wave 18), 941.7s
   (wave 19), 919.5s (wave 20 — this cycle, the fastest so far). All four sit inside a roughly
   920–955s band regardless of how many units the wave actually moved (72, 90, 337, 7
   respectively) — the pipeline's cost is dominated by the fixed cost of walking the full
   49,438-unit corpus three times, not by the size of any one wave's own movement.

# Cycle — wave-21 regeneration and attribution, ninth dispatch (2026-08-31, later same day)

- **Commit SHA:** this receipt is committed in the same commit as the `docs/work-inventory.json`
  regeneration and `completion-atlas.json` it describes; `git log -1` at the time of reading
  resolves it. Base: `origin/tranche/14` tip at dispatch time, `ea61100552` (fast-forwarded
  cleanly from local `HEAD`, no conflict, no worktree needed).

## 0. Four lanes named, three code lanes measured here

The dispatch brief named "UC (partial) / C (partial) / M (partial)" plus this cycle itself — the
same "fourth is this cycle itself" shape wave 19/20's own receipts already resolved. UC's own
named finding (the desktop trait picker's `list_available_character_traits` chaining only 4 of 7
tables, leaving Tactician/Arcane Temper/Desperate Resolve/Bruising Intellect/Planar
Savant/Pragmatic Activator/Precise Treatment unselectable for a brand-new character) is a UI
reachability gap on ALREADY-`grounded` records — it moves no `docs/work-inventory.json` bucket by
construction, and correctly moves none below. The commit that actually landed engine-classifier
movement for UC this wave is `AT-34-E4-002` cycle 9 (`af2c8ae1d7`, `ultimate_campaign`
`BONUS:SITUATION` trait slice), not the picker finding itself. Three code lanes measured: C
`AT-34-E3-002` cycle 8 (Favored Class Bonus generalized to five sibling classes, `2d9ae34f89`), M
`AT-34-E3-003` bucket-M equipment cycle 6 (encumbrance `WT:`/`COST:` wiring, `acd6a6a5e3`), UC
`AT-34-E4-002` cycle 9 (`af2c8ae1d7`). Also folded in: `38e10d066b`, a standalone classifier fix
(landed ahead of all three lanes' own commits) prefixing 15 of 170 deferral evidence strings with
`engine_diagnostic:` — its own committed-inventory test (`the_committed_inventory_is_well_formed_
and_uses_only_declared_statuses`) was RED against the stale committed inventory until this
regeneration baked its effect in; see §7 below.

## 1. Rebase and baseline

`git fetch origin tranche/14 && git rebase origin/tranche/14` — fast-forwarded cleanly onto
`ea61100552`, no conflict, no worktree needed. `git show HEAD:docs/work-inventory.json` at
`ea61100552` is the before-snapshot for every figure below.

## 2. Three-pass pipeline, in order, timed

```
corpus_literal_sweep            --json-out /tmp/sweep.json      ~3m20.5s   CLEAN (48,708 examined, 0 findings)
derived_evaluator_fixture_check --json-out /tmp/fixture.json    ~13s       1,839 units cleared, 0 failed
v06_work_inventory (CORPUS_LITERAL_SWEEP_REPORT + DERIVED_FIXTURE_CHECK_REPORT set, no --allow-stamp-loss)
                                                                 ~12m7s
```

**Total pipeline wall time: ~940.5s (~15m40.5s)** — timed from binary-build and report-write file
mtimes (no `time` wrapper on this cycle's own invocations, unlike prior cycles' receipts; the
per-pass split is therefore approximate, the total is accurate to within a few seconds). Sits
inside the same ~920–955s band as the four prior measured shared-regen runs (wave 16: 927s, wave
18: 953.664s, wave 19: 941.7s, wave 20: 919.5s) — not an outlier, confirming the fixed-cost-of-
walking-the-corpus finding a fifth time.

## 3. Whole-corpus before/after diff, by unit id — the headline finding

```
before: 49438 units    after: 49438 units    added: 0    removed: 0
changed (status or evidence, any field): 247

  Real bucket-boundary transitions: 230
    ingested-magnitude -> grounded : 224  (221 equipment + 3 trait)
    engine-does-not-hold -> grounded :  6  (6 class_feature)

  Evidence-only, same-status churn: 17
    deferred-with-reason -> deferred-with-reason : 15  (38e10d066b's own engine_diagnostic: prefix)
    literal-verified -> literal-verified          :  2  (mythic_adventures equipment, see §4)
```

**Every lane's own stated figure reproduced exactly — no lane mismatch this wave.**

| Cycle | Commit | Own claim | Measured | Match |
|---|---|---|---:|---|
| C cycle 8 — Favored Class Bonus generalized to 5 sibling classes | `2d9ae34f89` | 6, `core_rulebook` C 199→193 | **6** (`engine-does-not-hold`→`grounded`: `barbarian`, `fighter`, `monk`, `paladin`, `rogue`, `wizard` class_feature) | **exact** |
| M cycle 6 — encumbrance `WT:`/`COST:` wired into the equipment probe | `acd6a6a5e3` | 221 corpus-wide (33 `core_rulebook`), pending the next regen | **221** (`ingested-magnitude`→`grounded`, equipment, by book: `adventurers_guide` 61, `advanced_class_guide` 41, `core_rulebook` 33, `inner_sea_temples` 32, `ultimate_equipment` 15, `inner_sea_gods` 14, `inner_sea_magic` 6, `ultimate_psionics` 5, `advanced_players_guide` 4, `bestiary` 3, `advanced_race_guide` 3, `occult_adventures` 2, `ultimate_intrigue` 1, `ultimate_wilderness` 1) | **exact**, per-book too |
| UC cycle 9 — `BONUS:SITUATION` trait slice | `af2c8ae1d7` | 3, `ultimate_campaign` DONE 200→203, M 40→37 | **3** (`ingested-magnitude`→`grounded`: `trait_almost_human`, `trait_self_taught_scholar`, `trait_trustworthy`) | **exact** |

6 + 221 + 3 = 230, matching the whole-corpus diff's own real-transition total exactly. Re-derived
independently by `completion_atlas.py` before/after both snapshots (not read from any lane's own
prose): `core_rulebook` DONE 4616→4655 (+39 = 33 equipment + 6 class_feature), C 199→193 (−6), M
811→778 (−33), B/D/V/U/X/Z all unchanged (470/366/114/10/115/0); `ultimate_campaign` DONE
200→203 (+3), M 40→37 (−3), D/U/X unchanged (2/21/2); corpus-wide DONE 24731→24961 (+230), C
4180→4174 (−6), M 4674→4450 (−224), every other bucket unchanged (A 449, B 11769, D 2955, V 289,
U 202, X 170, Z 19). **No lane's expectation mismatched this wave.**

## 4. A real side-finding this cycle's own regen surfaced, claimed by nobody

The 2 `literal-verified`→`literal-verified` evidence-only entries are `mythic_adventures`
equipment (`black_iron_axe`, `aegis`) — DONE-bucket both before and after, evidence text shifting
from `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does` to
`equipment_effect_probe_observed_computed_delta`. Neither M's cycle-6 receipt nor any other
lane's receipt claims these two; M's own cycle-6 receipt scoped its measurement to status
transitions only (the 221/33 figures above) and did not report this evidence-string side effect
of widening `equipment_key_is_wired`. No bucket boundary crossed — instrument-correction, not
closure, not this cycle's own work either (a byproduct of M's widened probe now firing on two
already-DONE records with a more specific reason). Logged here since it was found by this
regeneration, not by any lane's own inspection.

## 5. Movement, four buckets (`decisions.md §9`)

**Closure: 230** (224 `ingested-magnitude`→`grounded` [221 equipment M's own + 3 trait UC's own]
+ 6 `engine-does-not-hold`→`grounded` [class_feature, C's own] — every one of these units left a
non-DONE bucket and landed in DONE).

**Reclassification: 0** (no unit moved between two non-DONE buckets this wave).

**Reachability: 0** (no lane widened a display/explanation wire onto an already-computed value
without also computing something genuinely new this wave).

**Instrument-correction: 17** (15 `deferred-with-reason`→`deferred-with-reason` evidence-string
fixes, `38e10d066b`'s own two-classifier-site `engine_diagnostic:` prefix, folded in by this
regeneration for the first time — this is the red test's own fix; plus the 2 `mythic_adventures`
DONE-DONE evidence-string corrections in §4).

This wave carries no bucket-LABEL correction of its own — all three lanes' own receipts already
used the correct labels (C: "6 closed, all straight to DONE"; M: "221 proven closure-eligible
pending the next regen"; UC: "3 [...] `ingested-magnitude`→`grounded`"), and the measured
movement confirms every one.

## 6. Atlas checks

`python3 scripts/completion_atlas.py --book core_rulebook --check`: `population=6701
unclassified=0 overlap=0` (exit 1 — **expected and correct**, `core_rulebook` still carries
`B:470 C:193 D:366 M:778 V:114 U:10 X:115`, book-not-finished, not an instrument failure).
`python3 scripts/completion_atlas.py --check` (corpus-wide): `population=49438 unclassified=0
overlap=0 done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False
citation_failures=0`, exit 0. Both scopes clean — `citation_failures=0` means none of this wave's
three lane-cycles' own line insertions shifted `completion_atlas.py`'s hardcoded
`BUCKET_DEFINITIONS` citations enough to break a `must_contain` match this time (each lane's own
receipt already re-derived its own citation pins after its own insertion, including resolving one
rebase conflict between C cycle 8 and UC cycle 9's concurrent insertions into the same file) — no
re-derivation needed this cycle.

## 7. The red test

`38e10d066b` landed ahead of this wave's three code lanes, prefixing 15 of 170 deferral evidence
strings (two `classify()` sites) with `engine_diagnostic:`, but the committed
`docs/work-inventory.json` at that commit still carried the old strings, so
`tests/v06_work_inventory.rs::the_committed_inventory_is_well_formed_and_uses_only_declared_
statuses` was RED. This regeneration bakes the fix in (the 15 `deferred-with-reason`→
`deferred-with-reason` evidence-only changes in §3 above are exactly those 15 strings, all
`core_rulebook`, all `companion`/`class_feature` kind). Re-run after committing:

```
$ cargo test --locked --test v06_work_inventory the_committed_inventory_is_well_formed_and_uses_only_declared_statuses -- --nocapture
running 1 test
test the_committed_inventory_is_well_formed_and_uses_only_declared_statuses ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 16 filtered out; finished in 2.96s
```

**GREEN.** No hand-edit — the fix came exclusively from the guarded three-pass regeneration.

## 8. Build verification

`cargo test --locked --no-run` (full workspace): exits 0. `apps/desktop/src-tauri`, tested
explicitly as its own separate cargo workspace: `cargo test --locked --no-run --manifest-path
apps/desktop/src-tauri/Cargo.toml` — exits 0. Both run after this commit's own regeneration. No
`src/**` file touched by this cycle itself — all three lanes' own code was already committed by
their respective cycles; this cycle only measures.

## 9. Wave ledger

```
WAVE         RUN               STARTED         LAST ACTIVITY     RAN FOR   LANES  STATE
wave 18      wf_47422ae1-5ea   08-31 09:02:26  08-31 11:12:24    2:09:58       4  done
wave 19      wf_195c6a9e-931   08-31 11:14:32  08-31 13:33:36    2:19:04       4  done
wave 20      wf_75aaf9fb-a7d   08-31 13:36:08  08-31 15:34:01    1:57:52       4  done
wave 21      wf_e22a7b7d-419   08-31 15:39:11  08-31 17:49+      2:10:19+      4  RUNNING (this cycle)
```

Wave 21 ran (through this cycle's own last check) **at least 2:10:19** — the longest of the last
four full waves (wave 18: 2:09:58, wave 19: 2:19:04 is still the longest overall, wave 20:
1:57:52), still inside the same rough 2-hour band the last several full (non-host-killed) waves
have settled into. `wf_e22a7b7d-419` already carried `"21"` in `KNOWN_WAVES` (added by an earlier
commit this same wave, `6f67df49c7`) — no script edit needed this cycle. Still `RUNNING` because
this cycle's own activity is the wave's most recent lane activity (§0 above), settling to `done`
once this commit lands and three minutes pass with no further writes.

## Status

**complete** — the shared regeneration ran in the assigned main checkout (clean rebase, no
worktree needed); all three code lanes' own stated figures reproduced exactly (6 / 221 / 3, per-
book too for M), zero numeric discrepancies at the cycle level, zero bucket-label corrections
needed. The red test (`the_committed_inventory_is_well_formed_and_uses_only_declared_statuses`)
is confirmed GREEN post-regeneration, closed exclusively through the guarded pipeline, no hand-
edit. `completion_atlas.py --check` clean both scopes (book-scoped exit 1 is the expected
"not yet 100%" signal, not a failure); full workspace and desktop-crate `cargo test --locked
--no-run` both exit 0. One real side-finding surfaced and logged, claimed by nobody (§4).

## Next-cycle plan

1. **C's own remainder** (193, 13 sub-causes): `class_chassis_internal_tracker` 16 +
   `prestige_class_chassis_internal_tracker` 10 (internal PCGen bookkeeping, no player-facing
   surface) + `favored_class_bonus_choice_no_seam__npc_classes` 5 (Adept, Aristocrat, Commoner,
   Expert, Warrior — genuine engine gap, no bounded chassis seam exists) + 10 further named sub-
   causes, full table in `AT-34-E3-002_cycle_receipt.md`.
2. **M's own remainder** (`core_rulebook` 229 of 262−33): 34 `%CHOICE`-gated + 18 `VAR`/`PRE`-
   gated = 52 correctness boundaries (correctly declined); 182 `COST`/`WT`-only (nothing to
   compute, now genuinely re-confirmed by this wave's own encumbrance fix reaching every record
   that HAS a `WT:` token); 31 scattered new-subsystem shapes each too small alone.
3. **UC's own remainder** (62 non-DONE of 265: `M:37 D:2 U:21 X:2`): 3 `VAR`-only `trait_content`
   records (now the largest single group), 2 `ABILITYPOOL`-only, 1 mixed `CASTERLEVEL`+`SKILL`,
   1 corpus data gap, 30 `ability_content` (Drawback/Retrain) records declared out of scope. Also
   the desktop trait picker's own named gap (7 `grounded` traits across two slices unselectable
   for a new character) — a UI reachability finding, not a `docs/work-inventory.json` bucket, and
   not this cycle's territory to close.
4. **Wave shape cost, measured across five runs**: 927s (wave 16), 953.664s (wave 18), 941.7s
   (wave 19), 919.5s (wave 20), ~940.5s (wave 21 — this cycle, approximate). All five sit inside
   the same ~920–955s band regardless of how many units the wave actually moved (72, 90, 337, 7,
   230 respectively) — the pipeline's cost is dominated by the fixed cost of walking the full
   49,438-unit corpus three times, not by the size of any one wave's own movement.

# Cycle — wave-22 regeneration and attribution, tenth dispatch (2026-08-31/09-01, later same day)

- **Commit SHA:** this receipt is committed in the same commit as the `docs/work-inventory.json`
  regeneration and `completion-atlas.json` it describes; `git log -1` at the time of reading
  resolves it. Base: `origin/tranche/14` tip at dispatch time, `451a5d6837` (fast-forwarded
  cleanly from local `HEAD`, no conflict, no worktree needed).

## 0. "Four lanes" named, three code lanes measured — same stale-boilerplate shape as waves 15/19/20/21

The dispatch brief again named "UC (partial) / C (partial) / M (partial)" as if a fourth lane
also reported — the same "fourth is this cycle itself" framing prior waves' own receipts already
resolved. `git log --oneline a0cbc2388a..HEAD` shows exactly three code-bearing commits since
wave 21's own shared regen: C `AT-34-E3-002` cycle 9 (`2c81e4bfce`, Ranger Favored Enemy exact-
slug identity record), UC `AT-34-E4-002` cycle 10 (`c1cbfa0698` picker-gap fix + `7714a6a5ef`
eighth trait slice, Eldritch Delver), M `AT-34-E3-003` bucket-M equipment cycle 7
(`7e59387d9f`, exhaustive remainder census, explicitly **0 code change**). The dispatch brief's
own second staleness this wave: its red-test instruction ("Commit `38e10d066b` ... Run that test
after regenerating") describes a fix wave 21's own shared regen already baked in and confirmed
GREEN nine dispatches ago (§7 of that section, this same file) — not new work for this cycle.
Re-run anyway per instruction (§7 below); still GREEN, as expected, not newly fixed.

UC's own picker-gap fix (`c1cbfa0698`, chaining `INITIATIVE_TRAIT_BONUSES`/
`CONCENTRATION_TRAIT_BONUSES`/`ABILITY_DIFF_SKILL_TRAIT_BONUSES` into
`list_available_character_traits`, closing the gap cycle 9 found and wave 22's own dispatch
brief handed back as this cycle's explicit first task) moves no `docs/work-inventory.json`
bucket by construction — it makes seven **already-`grounded`** traits selectable in the desktop
character creator, a UI reachability fix, not a new engine computation. The commit that actually
moved an inventory bucket for UC this wave is the eighth-trait-slice commit (`7714a6a5ef`,
Eldritch Delver, mixed `CASTERLEVEL`+`SKILL`).

## 1. Rebase and baseline

`git fetch origin tranche/14 && git rebase origin/tranche/14` — fast-forwarded cleanly onto
`451a5d6837`, no conflict, no worktree needed. `git show HEAD:docs/work-inventory.json` at
`451a5d6837` is the before-snapshot for every figure below.

## 2. Three-pass pipeline, in order, timed

```
corpus_literal_sweep            --json-out /tmp/sweep.json      194s (3m14s)   CLEAN (48,708 examined of 51,482 read, 0 findings)
derived_evaluator_fixture_check --json-out /tmp/fixture.json     14s           1,839 units cleared over 2,580 fixture rows, 0 failed
v06_work_inventory (CORPUS_LITERAL_SWEEP_REPORT + DERIVED_FIXTURE_CHECK_REPORT set, no --allow-stamp-loss)
                                                                 729s (12m9s)
```

**Total pipeline wall time: 937s (~15m37s)**, timed from `date -u +%s` markers bracketing each
pass. Sits inside the same ~920–955s band as the five prior measured shared-regen runs (wave 16:
927s, wave 18: 953.664s, wave 19: 941.7s, wave 20: 919.5s, wave 21: ~940.5s) — a sixth
confirmation that the pipeline's cost is fixed by the corpus walk, not by wave size.

## 3. Whole-corpus before/after diff, by unit id — the headline finding

```
before: 49438 units    after: 49438 units    added: 0    removed: 0
changed (status or evidence, any field): 2

  Real bucket-boundary transitions: 2
    engine-does-not-hold -> grounded : 1  (core_rulebook class_feature)
    ingested-magnitude   -> grounded : 1  (ultimate_campaign trait)

  Evidence-only, same-status churn: 0
```

**Both units, and only both:**

| Unit id | Before | After |
|---|---|---|
| `core_rulebook:class_feature:ranger_favored_enemy` | `engine-does-not-hold` / `no_explanation_id_and_no_diagnostic_names_this_feature` | `grounded` / `explanation_id_observed_in_a_real_computation` |
| `ultimate_campaign:trait:trait_eldritch_delver` | `ingested-magnitude` / `trait_content_table_holds_record_magnitude_not_yet_computed` | `grounded` / `trait_content_magnitude_computed_and_verified_by_fixture_execution_flat_2` |

**Every lane's own stated figure reproduced exactly — no lane mismatch this wave, and no side-
finding claimed by nobody either (unlike wave 21's `mythic_adventures` evidence-only pair — this
wave's diff has zero evidence-only churn at all):**

| Cycle | Commit | Own claim | Measured | Match |
|---|---|---|---:|---|
| C cycle 9 — Ranger Favored Enemy exact-slug identity record | `2c81e4bfce` | 1, `core_rulebook` C 193→192 | **1** (`ranger_favored_enemy`, `engine-does-not-hold`→`grounded`) | **exact** |
| UC cycle 10 — eighth trait slice, mixed `CASTERLEVEL`+`SKILL` (Eldritch Delver) | `7714a6a5ef` | 1, `ultimate_campaign` DONE 203→204, M 37→36 | **1** (`trait_eldritch_delver`, `ingested-magnitude`→`grounded`) | **exact** |
| M cycle 7 — exhaustive remainder census | `7e59387d9f` | **0** (explicitly "0 code change") | **0** | **exact** |
| UC picker-gap fix — 3 trait tables chained into the desktop picker | `c1cbfa0698` | 0 (UI reachability, no bucket claimed) | **0** | **exact** |

1 + 1 + 0 = 2, matching the whole-corpus diff's own real-transition total exactly. Re-derived
independently via `completion_atlas.py` before/after both snapshots, not read from any lane's own
prose: `core_rulebook` DONE 4655→4656 (+1), C 193→192 (−1), B/D/M/V/U/X/Z all unchanged
(470/366/778/114/10/115/0); `ultimate_campaign` DONE 203→204 (+1), M 37→36 (−1), D/U/X
unchanged (2/21/2); corpus-wide DONE 24961→24963 (+2), C 4174→4173 (−1), M 4450→4449 (−1),
every other bucket unchanged (A 449, B 11769, D 2955, V 289, U 202, X 170, Z 19).

**No lane's expectation mismatched this wave** — the first wave since 21 where every figure,
membership included, reproduced digit for digit on the first check.

## 4. Side-findings

None this cycle. Zero evidence-only, same-status churn in the whole-corpus diff (§3), and no
unit outside the two claimed above moved at all.

## 5. Movement, four buckets (`decisions.md §9`)

**Closure: 2** (`ranger_favored_enemy` `engine-does-not-hold`→`grounded` [C's own], and
`trait_eldritch_delver` `ingested-magnitude`→`grounded` [UC's own] — both left a non-DONE bucket
and landed in DONE).

**Reclassification: 0.**

**Reachability: 0** (the UC picker-gap fix is a real reachability improvement in the desktop UI,
but it touches no `docs/work-inventory.json` bucket — the seven traits it unlocks were already
`grounded` before this wave; nothing to attribute here).

**Instrument-correction: 0** in `docs/work-inventory.json` itself this wave. (`completion_atlas.py`'s
own `V`-bucket citation line drifted 13250→13262 from wave 22's `v06_work_inventory.rs`
insertions — see §6 — a real instrument fix, but scoped to `scripts/completion_atlas.py`'s own
citation table, not a unit-bucket movement in the inventory this section tracks.)

2 = 2, the full changed-unit count.

## 6. Atlas checks

`python3 scripts/completion_atlas.py --check` (corpus-wide), run **before** any fix:
`population=49438 unclassified=0 overlap=0 done_evidence_violations=0
missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=1` —
`citation_failure: V: src/bin/v06_work_inventory.rs:13250 no longer contains 'literal-verified'`,
exit 1. Re-derived by direct `grep -n '"literal-verified"' src/bin/v06_work_inventory.rs`: the
`item.verdict.status = "literal-verified";` assignment the `V` bucket's citation targets sits at
line **13262** now (drifted from 13250 by wave 22's own `AT-34-E4-002` cycle 10 insertions into
`v06_work_inventory.rs` — the eighth `.or_else` classifier fallback and its doc-comment/test
additions). Fixed in `scripts/completion_atlas.py` (one line, the `V` bucket's `citation.line`
13250→13262, comment updated to name the cause). Re-run: `citation_failures=0`, exit 0. All ten
`BUCKET_DEFINITIONS` citations now re-checked and resolve at this wave's own HEAD.

`python3 scripts/completion_atlas.py --book core_rulebook --check`: `population=6701
unclassified=0 overlap=0`, `DONE:4656 A:0 B:470 C:192 D:366 M:778 V:114 U:10 X:115` (exit 1 —
expected, book not yet 100% done, not an instrument failure).

`python3 scripts/completion_atlas.py --book ultimate_campaign --check`: `population=265
unclassified=0 overlap=0`, `DONE:204 A:0 B:0 C:0 D:2 M:36 V:0 U:21 X:2 Z:0` (exit 1 — expected,
same reason).

## 7. The red test

The dispatch brief re-described `38e10d066b`'s `engine_diagnostic:` prefix fix and its red test
as if newly outstanding — it is not; wave 21's own shared regen (this same file, ninth section,
§7) already baked the fix in and confirmed GREEN nine dispatches ago. Re-run per this cycle's
own instruction anyway, at this wave's post-regen HEAD:

```
$ cargo test --locked --test v06_work_inventory the_committed_inventory_is_well_formed_and_uses_only_declared_statuses
running 1 test
test the_committed_inventory_is_well_formed_and_uses_only_declared_statuses ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 16 filtered out; finished in 3.15s
```

**GREEN**, as expected — not a new fix this cycle, a confirmation that wave 21's fix has not
regressed across nine subsequent dispatches. No hand-edit involved either way.

## 8. Build verification

`cargo test --locked --no-run` (full workspace): exits 0. `apps/desktop/src-tauri`, tested
explicitly as its own separate cargo workspace: `cargo test --locked --no-run --manifest-path
apps/desktop/src-tauri/Cargo.toml` — exits 0 (`Finished` in 1m47s). Both run after this commit's
own regeneration and the `completion_atlas.py` citation fix. No `src/**` file touched by this
cycle itself — both lanes' own code was already committed by their respective cycles; this cycle
only measures (plus the one-line `scripts/completion_atlas.py` citation-pin fix, `scripts/**`,
not `src/**`).

## 9. Wave ledger

```
WAVE         RUN               STARTED         LAST ACTIVITY     RAN FOR   LANES  STATE
wave 19      wf_195c6a9e-931   08-31 11:14:32  08-31 13:33:36    2:19:04       4  done
wave 20      wf_75aaf9fb-a7d   08-31 13:36:08  08-31 15:34:01    1:57:52       4  done
wave 21      wf_e22a7b7d-419   08-31 15:39:11  08-31 17:52:56    2:13:44       4  done
wave 22      wf_56c5bcae-8f5   08-31 18:33:18  08-31 20:13+      1:40:23+      4  RUNNING (this cycle)
```

`wf_56c5bcae-8f5` already carried `"22"` in `KNOWN_WAVES` (added by an earlier commit this same
wave, `15485e5197`) — no script edit needed this cycle, and no run id is missing a wave number.
Wave 22 ran (through this cycle's own last check before commit) **at least 1:40:23** — shorter so
far than the last three full waves (wave 19: 2:19:04, wave 20: 1:57:52, wave 21: 2:13:44), still
`RUNNING` because this cycle's own activity is the wave's most recent lane activity, settling to
`done` once this commit lands and three minutes pass with no further writes.

## Status

**complete** — the shared regeneration ran in the assigned main checkout (clean rebase, no
worktree needed); both code-bearing lanes' own stated figures reproduced exactly (1 / 1, and M's
own explicit 0), zero numeric or membership discrepancies, zero bucket-label corrections needed.
The red test is confirmed GREEN (already fixed at wave 21, not newly fixed here — the dispatch
brief's own re-description of it is stale, named in §0). `completion_atlas.py --check` found and
this cycle fixed one real citation drift (`V` bucket, 13250→13262), now clean at both scopes
(corpus-wide `citation_failures=0`; book-scoped exit 1 is the expected "not yet 100%" signal, not
a failure). Full workspace and desktop-crate `cargo test --locked --no-run` both exit 0.

## Next-cycle plan

1. **C's own remainder** (192, 13 sub-causes summing exactly, one row split off this wave's own
   closure): full table in `AT-34-E3-002_cycle_receipt.md`.
2. **M's own remainder** (`core_rulebook` 229 of 229, unchanged — 0 code change this wave): 7
   named mechanisms summing exactly (choice-gated 104, cost-only-no-weight-deliberately-excluded
   44, untraced-closure 28, VAR-formula-needs-evaluator 18, thin-no-raw-tokens 17,
   ITEMCOST-pricing-formula-no-consumer 9, real-type-excluded-or-new-mechanic 9).
3. **UC's own remainder** (61 non-DONE of 265: `M:36 D:2 U:21 X:2`): 3 genuinely distinct
   `VAR`-only records (a cross-cutting luck-bonus modifier, an eidolon mechanic, 6 missing
   channel-energy DC totals), 2 `ABILITYPOOL`-only, 1 corpus data gap = 6 `trait_content`, 30
   `ability_content` records (Drawback/Retrain) declared out of scope.
4. **Wave shape cost, measured across six runs**: 927s (wave 16), 953.664s (wave 18), 941.7s
   (wave 19), 919.5s (wave 20), ~940.5s (wave 21), 937s (wave 22 — this cycle). All six sit
   inside the same ~920–955s band regardless of how many units the wave actually moved (72, 90,
   337, 7, 230, 2 respectively) — wave 22's own 2-unit movement is the smallest of the six and
   still cost the same ~937s, the strongest confirmation yet that this pipeline's wall time is
   fixed-cost, not movement-proportional.
