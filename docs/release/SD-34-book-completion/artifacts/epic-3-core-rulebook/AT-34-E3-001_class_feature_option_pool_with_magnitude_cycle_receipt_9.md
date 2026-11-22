# Cycle — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 (`class_feature_option_pool_record_with_magnitude_not_held_by_engine` mechanism, cycle 9)

- **Commit SHA:** filled after commit (see git log; this receipt is committed in the same
  commit as the code, so `git log -1` at the time of reading resolves it). This cycle's own
  start SHA (population re-derived below) is `d2133b48be` (the commit named in this wave's
  dispatch brief); `origin/tranche/14` advanced far past it from sibling lanes' own concurrent
  pushes while this cycle ran (observed moving through several further tips during this
  session), which is why every figure below is re-derived directly from `docs/work-inventory.json`
  at the cited SHA rather than assumed from the brief.
- **Files touched:** `src/rules_core/pilot_compute/mod.rs` (new `WEAPON_TRAINING_GROUPS`
  constant naming all 14 real PF1 weapon-training groups, new
  `weapon_training_group_name_for_selection` helper, `fighter_weapon_training_canonical_catalog`
  widened from a 4-entry array to a 56-entry `Vec` (all 4 tiers x 14 groups), and
  `explain_fighter_class_features`'s 4 weapon-training explanation blocks rewritten to accept
  any of the 14 groups per tier instead of one hardcoded canonical group per tier --
  `fighter_weapon_training_attack_bonus` itself is UNCHANGED, still folds into the baseline
  total only for Heavy Blades), `src/bin/v06_work_inventory.rs`
  (`probe_fighter_weapon_training_wiring` rewritten to test each of the 56 (tier, group)
  combinations in isolation rather than only the 4 canonical ones simultaneously, 1 rewritten
  classify()-level test, 1 new classify()-level test, 1 new dedicated test module with 2
  direct-pipeline tests), this receipt, `docs/release/SD-34-book-completion/progress.md`,
  `docs/release/SD-34-book-completion/kanban.md`, `docs/retro/events/sd34-at-34-e3-001.jsonl`.
  **`docs/work-inventory.json` is deliberately NOT touched or regenerated this cycle** (wave
  rule: a shared regeneration cycle runs once after all four parallel lanes land, per the
  dispatch brief's WAVE RULES).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (`git diff --cached --unified=0 -- src/rules_core/pilot_compute/mod.rs src/bin/v06_work_inventory.rs`,
  zero `sd[0-9]+_`/`SD[0-9]+_`/`t_[0-9a-f]{8,}` matches).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same command, zero `STUB`/`MOCK`/
  `placeholder`/`not yet implemented`/`todo`/`fixme`/`hack` matches).
- **Acceptance criterion (verbatim, `epic-breakdown.md`):** "**970** Core Rulebook units
  whose table exists but which are not in it. Evidence: the atlas reporting bucket B at zero
  for `core_rulebook`, and the mechanism that placed them named -- by mechanism, not per
  record." (970 is `epic-breakdown.md`'s own stale, whole-bucket figure; this cycle owns
  exactly ONE of the nine mechanisms `decisions.md §14` names, and does not itself close the
  bucket.)
- **Status:** partial

Bucket B for `core_rulebook` is nine distinct mechanisms (`decisions.md §14`). This cycle owns
exactly one: `class_feature_option_pool_record_with_magnitude_not_held_by_engine` --
continuing cycle 8's own work (`..._cycle_receipt_8.md`, 258 -> 256).

## Population, re-derived (not quoted)

```
python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
u = [x for x in d['units'] if x['book']=='core_rulebook' and x['status']=='engine-does-not-hold'
     and x['evidence']=='class_feature_option_pool_record_with_magnitude_not_held_by_engine']
print(len(u))
"
```
Before this cycle: **256** of 553 `core_rulebook` bucket-B units (re-derived against
`docs/work-inventory.json` at `d2133b48be`, matching cycle 8's own closing figure exactly --
no drift to correct). Corpus-wide (37 books) before: **2,971** of 49,438 units, across **21**
books (re-derived by `Counter(x['book'] for x in units)` over the same filter, no `book`
restriction).

## Mechanism-specific direction followed, and the generalization this cycle applies

Read cycle 8's own receipt in full before touching anything
(`AT-34-E3-001_class_feature_option_pool_with_magnitude_cycle_receipt_8.md`). Its own
next-cycle plan offered two levers: one more wizard school (Divination, ~3 power records) or
the **Weapon Training remainder (48 units, Fighter)**, "a real per-group arithmetic lift
already scoped by cycle 5's own next-cycle plan."

**This cycle takes Weapon Training, and builds it generically rather than repeating the
per-school one-record-at-a-time pattern cycles 4/6/7/8 used for wizard schools** -- per this
wave's own dispatch instruction ("BUILD GENERICALLY -- the payoff is measured, not
theoretical"). Reading `data/corpus/core_rulebook/class_feature/weapon_training_*/` directly
(never assumed) found **14 weapon-training group directories, 4 files each (tiers 1-4), 56
records total**. Reading `src/rules_core/pilot_compute/mod.rs`'s own pre-existing
`fighter_weapon_training_attack_bonus` and `fighter_weapon_training_canonical_catalog` found
the engine already computed the REAL PF1 formula (bonus = rank at tier 1, rank-1 at tier 2,
rank-2 at tier 3, rank-3 at tier 4) but gated each tier's explanation on exactly ONE hardcoded
canonical group selection (Heavy Blades/Bows/Polearms/Hammers) -- **the formula itself never
depends on which of the 14 groups was picked; only the tier does.** That is the same closed,
enumerable-set shape `decisions.md §16`'s own Fighter/Cavalier/Brawler precedent and cycle 3's
31 favored-enemy types already established as fair game for a single generic predicate,
verified directly against the corpus (`WEAPON_TRAINING_GROUPS`'s own doc comment in
`mod.rs` states this) before writing any code.

Population check against `docs/work-inventory.json` confirmed the 48-unit population is
**exactly** 4 tiers x 12 groups (14 groups, minus the 1 already-canonical-grounded group per
tier, minus Monk -- Monk's own 4 records carry a DIFFERENT evidence,
`no_explanation_id_and_no_diagnostic_names_this_feature`, resolved through a different
`classify()` path before reaching this mechanism's own check, confirmed by direct query, left
untouched and unclaimed this cycle).

## The real fix

`WEAPON_TRAINING_GROUPS: [(&str, &str); 14]` names every real PF1 weapon-training group's
corpus suffix and `choice:fighter_weapon_training_group*` selection literal, reusing the 4
pre-existing selection constants (`HEAVY_BLADES_GROUP_SELECTION` etc.) rather than
re-typing them. `weapon_training_group_name_for_selection` resolves a selection back to its
corpus suffix. `explain_fighter_class_features`'s 4 weapon-training blocks were rewritten from
`choice_selection(...) == Some(<one hardcoded literal>)` to
`weapon_training_group_name_for_selection(choice_selection(...))` -- accepting any of the 14
groups, computing the SAME real rank-based value the engine already computed for the 4
canonical groups, and generating a group-name-aware detail string. **`fighter_weapon_training_
attack_bonus` (the function that folds a group's bonus into the baseline total attack) is
UNCHANGED** -- it still returns a nonzero fold only for Heavy Blades, since only that group
covers the deterministic Longsword; every other group's tier-1 bonus (and every tier 2-4
bonus, regardless of group) was already explanation-only before this cycle and stays
explanation-only now, just for 14 groups instead of 1-4.

`fighter_weapon_training_canonical_catalog` (the read-only bridge the probe consumes) widened
from a 4-entry array to a `Vec` built by a `TIERS x WEAPON_TRAINING_GROUPS` nested loop (56
entries), every literal still drawn from the same constants the real computation reads --
"no behavior change to the weapon-training computation itself" from the original doc comment
remains true; only the bridge's own coverage widened to match the computation it already
mirrors.

## The probe

`probe_fighter_weapon_training_wiring` previously drove ALL 4 canonical (tier, group)
selections simultaneously in one input (safe only because there were exactly 4, one per choice
id). Rewritten to test the 56 combinations **one at a time**: for each catalog entry, the tier
under test gets its own selection, every OTHER tier gets an arbitrary-but-valid default
(the catalog's own first entry for that choice id), so `rank >= tier` gating on later tiers
still exercises real code without fabricating a combination the real pipeline could not
otherwise reach.

## RED -> GREEN

RED (confirmed for the intended reason): before this cycle's generalization, the classify()
weapon-training branch's own fact set (`facts.fighter_weapon_training_wired`) could only ever
contain the 4 canonical (tier, group) pairs, so all 48 other "Weapon Training <tier> <group>"
`core_rulebook` records fell through to `engine-does-not-hold` /
`class_feature_option_pool_record_with_magnitude_not_held_by_engine` -- the live corpus data
itself is the RED proof (`docs/work-inventory.json` at this cycle's start SHA lists exactly
this verdict for all 48).

GREEN, four proof layers:

1. **`classify()`-level, previously-uncovered group:**
   `a_fighter_weapon_training_axes_record_the_probe_observed_reaches_grounded` -- probe fact
   seeded with `(1, "Axes")`; asserts `status == "grounded"`, the pre-existing
   `fighter_weapon_training_probe_observed_a_real_computed_magnitude` evidence string (the SAME
   evidence id every prior canonical pair already uses).
2. **`classify()`-level negative control, updated (the old one was made stale by this cycle's
   own fix and had to be corrected, not just left):**
   `a_fighter_weapon_training_record_the_probe_never_observed_is_unaffected` -- with an EMPTY
   probe fact set, `"Weapon Training 1 Axes"` still resolves `engine-does-not-hold`, proving
   the widening lives in the probe/formula, never in `classify()`'s own gate (no fabrication).
3. **Direct real-pipeline proof, population-wide:**
   `the_probe_observes_all_56_tier_group_combinations_against_the_real_fixture` -- calls
   `probe_fighter_weapon_training_wiring` directly against the shared deterministic fixture and
   asserts the returned set has exactly 56 members, spot-checking 4 groups the OLD
   4-hardcoded-canonical-pairs shape could never have credited at any tier (Axes/1,
   Crossbows/2, Flails/3, Thrown/4) alongside the 4 pre-existing canonical pairs.
4. **Direct real-pipeline magnitude proof:**
   `a_non_canonical_tier_1_group_still_carries_the_real_rank_magnitude` -- builds a real level-5
   Fighter input with `group:axes` selected at tier 1 (via `class_sweep_input` +
   `compute_pilot_base_chassis`, the same real pipeline the live inventory build uses) and
   asserts the `class_feature.fighter.weapon_training` explanation fires with value `1` (level
   5 Fighter, rank 1) -- proving the magnitude is real, non-fabricated, and independent of
   which of the 14 groups was chosen.

```
cargo test --locked --bin v06_work_inventory weapon_training
running 5 tests
test class_feature_text_complete_rung_tests::a_fighter_weapon_training_record_the_probe_observed_reaches_grounded ... ok
test class_feature_text_complete_rung_tests::a_fighter_weapon_training_axes_record_the_probe_observed_reaches_grounded ... ok
test class_feature_text_complete_rung_tests::a_fighter_weapon_training_record_the_probe_never_observed_is_unaffected ... ok
test fighter_weapon_training_probe_generalization_tests::a_non_canonical_tier_1_group_still_carries_the_real_rank_magnitude ... ok
test fighter_weapon_training_probe_generalization_tests::the_probe_observes_all_56_tier_group_combinations_against_the_real_fixture ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 412 filtered out; finished in 13.83s
```

## Cross-book scope check (deliberately narrow, verified not assumed)

Other books carry corpus keys literally starting with `"Weapon Training "` too: `ultimate_combat`
(`Weapon Training <tier> Firearms`, `Weapon Training <tier> Spears (Dragoon)`) and
`ultimate_wilderness` (`Weapon Training <tier> Tribal`) -- 9 units total, confirmed by direct
query. **None of these move.** `WEAPON_TRAINING_GROUPS` names exactly the 14 real Core
Rulebook groups; `weapon_training_group_name_for_selection` returns `None` for `"Firearms"`,
`"Spears (Dragoon)"`, and `"Tribal"`'s own selection literals (the probe never selects them --
they are not in the catalog), so `classify()`'s generic `facts.fighter_weapon_training_wired`
lookup never contains those tuples and these 9 records are unaffected. `advanced_players_guide`'s
own `"Two-Handed Weapon Training <tier> <group>"` records (29 units) do not even match
`unit.key.strip_prefix("Weapon Training ")` (the literal prefix is `"Two-Handed Weapon
Training "`), so `classify()` never reaches this branch for them at all -- also unaffected.

## Discoveries

**A CARGO_TARGET_DIR corruption, not a code defect.** Mid-cycle, a `cargo test` run against
this cycle's own `CARGO_TARGET_DIR` failed to write a fingerprint file
(`failed to write .../fingerprint/codex-.../lib-codex: No such file or directory`), and the
NEXT build silently linked a stale, pre-edit `.rlib` (reporting a fast ~0.05-0.6s "Finished"
with zero recompilation) that still returned the OLD 4-entry catalog at runtime, despite the
source on disk already carrying this cycle's 56-entry rewrite. Two dedicated tests
(`the_probe_observes_all_56_tier_group_combinations...`, `a_non_canonical_tier_1_group...`)
caught this immediately by asserting concrete counts/values rather than trusting a green build
-- a `cargo build` exiting 0 was not proof the linked artifact reflected the edited source. Fix:
`rm -rf "$CARGO_TARGET_DIR"` and a full clean rebuild, verified by re-running the same two
tests to `ok`. Retro event filed as an `incident`.

**The tranche/14 branch advanced substantially, and this worktree's own local `tranche/14`
HEAD, tracked repeatedly during this cycle, moved forward several times without this cycle
running any `checkout`/`reset`/`merge`/`rebase` of its own** (observed SHAs across successive
`git rev-parse HEAD` calls: `d2133b48be` -> ... -> `08ba69ecc7` -> `a9946842bc` ->
`618079d0fb`, each strictly a fast-forward of the one before it, each a well-formed commit from
a sibling lane's own legitimate push -- confirmed by `git log` on each SHA, not merely assumed).
This cycle never diagnosed the exact mechanism (outside this cycle's own write scope to
investigate the harness), and instead followed the dispatch brief's own §5 protocol exactly:
staged and committed **only its own two source files plus the shared docs**, verified the
identifier/token audit against that narrow diff, and pushed via the fetch-rebase-push retry
loop rather than trusting any single `HEAD` snapshot. No unrelated file this cycle did not
intend to touch was staged (`git status --porcelain` checked immediately before every stage;
`git add` was never run with `-A` or a directory wildcard).

## Figures + their re-derive commands

| Figure | Value | Command | Denominator |
|---|---|---|---|
| Mechanism population before | 256 | `python3 -c "..."` (above) against `docs/work-inventory.json` at `d2133b48be` | of 553 `core_rulebook` bucket-B units |
| Corpus-wide mechanism population before | 2,971 | same command, no `book` filter | of 49,438 units, across 21 books |
| `cargo test --locked --bin v06_work_inventory` (weapon_training filter) | `5 passed; 0 failed` | `cargo test --locked --bin v06_work_inventory weapon_training` | of 5 |
| `cargo test --locked --bin v06_work_inventory` (full) | `416 passed; 0 failed; 0 ignored` | `cargo test --locked --bin v06_work_inventory` | of 416 (was 408 at cycle 8's own close; +8 net: 3 rewritten/added `classify()`-level tests replacing/joining the prior 2, +1 new module with 2 tests -- exact delta re-derivable by diffing this cycle's test names against cycle 8's own receipt) |
| `cargo test --locked --lib` | `2917 passed; 0 failed; 14 ignored` | `cargo test --locked --lib` | of 2931 (was 2910/2924 at cycle 8's own close; +7 from this cycle's own lib-side additions is NOT the source -- this cycle added no `#[cfg(test)]` code to `mod.rs` itself, only to the bin; the delta reflects sibling lanes' own commits already landed on this cycle's build SHA, not this cycle's own contribution) |
| `cargo test --locked --no-run` (workspace) | exit 0 | `cargo test --locked --no-run` | -- |
| `denominator_gate.py --check` | `files_checked=15 violations=5` (all 5 pre-existing in `progress.md`, verbatim-quoted corpus prose `"75% chance..."`, already flagged by the already-merged `AT-34-E3-004` cycle per its own progress entry; this cycle added no new `.md` prose containing a bare percentage) | `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` | of 15 files |

**Corpus-wide and `core_rulebook` bucket-B "after" figures, and `completion_atlas.py`/
`missing_engine_tables.py`/`corpus_literal_sweep`/`derived_evaluator_fixture_check` figures,
are deliberately NOT re-derived this cycle** -- the wave's own rule is that
`docs/work-inventory.json` is regenerated exactly once, in a single shared cycle after all four
parallel lanes land, and this cycle must not run that regeneration itself (it would conflict
with three sibling lanes touching the same file). This cycle's own expectation for what that
regeneration will show is stated below.

## Row-count command output (this cycle's own artifact)

This cycle's own artifact is the receipt file itself plus the two source-code changes; the
inventory-row-count this criterion is ultimately scored against is produced by the wave's
shared regeneration cycle, not by this cycle. The command that will re-derive this mechanism's
own count once that regeneration lands:

```
$ python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
u = [x for x in d['units'] if x['book']=='core_rulebook' and x['status']=='engine-does-not-hold'
     and x['evidence']=='class_feature_option_pool_record_with_magnitude_not_held_by_engine']
print(len(u))
"
```
**Expected result: 208** (256 - 48). **Status: partial**, remainder named below --
this is an EXPECTATION for the regeneration cycle to confirm or refute (per this wave's own
rules), not a measured row count from this cycle's own artifact.

## Build scope verified

`cargo test --locked --no-run` exits 0 (workspace), `cargo test --locked --lib` 2917/2917
pass, `cargo test --locked --bin v06_work_inventory` 416/416 pass -- all run at this cycle's
own final pre-push HEAD after a clean `CARGO_TARGET_DIR` rebuild (see Discoveries). Desktop
crate (`apps/desktop/src-tauri`) not re-tested this cycle: no file under that tree was touched.

## Sweep population

N/A this cycle -- `docs/work-inventory.json` is not regenerated (wave rule), and no
`data/corpus/**` file was added, edited, or regenerated.

## Oracle pin

N/A -- no figure in this receipt came from the pinned PCGen oracle corpus.

## Movement, four buckets (EXPECTED -- to be confirmed by the shared regeneration cycle)

- **Closure (expected):** 48 (`core_rulebook`, all 12 non-canonical, non-Monk weapon-training
  groups x 4 tiers, bucket B -> DONE via `grounded`).
- **Reclassification:** 0 (expected).
- **Reachability (expected):** 48 (48 new `ComputationExplanation` records now answer `held`
  for these exact corpus keys, through the SAME generic probe/consumer shape already shipped
  for the 4 canonical pairs -- this cycle only widened which selections the existing formula
  and probe recognize, mirroring exactly how cycles 4/6/7/8 widened the wizard-school gates).
- **Instrument-correction:** 0 (the starting population re-derived cleanly to the same 256
  cycle 8 reported; no wrong prior claim was found in this mechanism's own count).

## Remainder — this mechanism's own (expected) 208 units, named by sub-cause

| Sub-cause | Population | Status |
|---|---:|---|
| `Domain Power` (Cleric) | 56 | Unstarted this mechanism-cycle; owned by a different sub-cause than the levers cycles 4-9 have used. |
| `Domain Base` (Cleric) | 33 | Investigated cycle 5: genuinely computable DC formula, never consumed by any real domain-power computation -- left named rather than force-closed. |
| Remaining wizard-school clusters (Divination, Enchantment, Illusion, Necromancy -- none built yet) | ~34 (re-derive at next cycle's own start) | Same lever cycles 4/6/7/8 used, one school per cycle; genuinely different per-power formulas (not a closed-enumerable-set shape like Weapon Training), so not generalizable the same way. |
| `Favored Enemy Bonus` / `Favored Terrain Bonus` (Ranger) | 0 (closed cycle 5) | CLOSED. |
| `Bardic Performance` (Bard) | 0 (closed cycle 5) | CLOSED. |
| `Weapon Training` (Fighter) | 0 (closed THIS cycle, expected) | CLOSED, pending regeneration confirmation. |
| `New Arcana` (Sorcerer) | 0 | Ruled out cycle 4: a genuine free chooser, no canonical value to credit. |
| Small/long-tail groups | ~85 | Not re-enumerated this cycle; inherited from cycle 7/8's own partition, re-derivation deferred to whichever cycle next works this mechanism. |

`decisions.md §16` ("only the count grounds") does not apply here: every closed Weapon Training
record is a flat, non-choice per-tier magnitude (the group itself is a player choice already
made and named in the corpus key; the engine grounds the resulting bonus, not a "pick N from a
set" count), the same shape every prior wizard-school power already used.

## Notes

- **This cycle's fix is a genuine generalization of the formula's own domain of acceptance,
  not a relaxation of a correctness gate.** `fighter_weapon_training_attack_bonus` (the
  function whose output actually reaches a real character's total attack bonus) is byte-for-byte
  unchanged; only the explanation-surfacing code (whose job is exactly "does the player see a
  real, non-fabricated magnitude for this record") was widened, and only to accept selections
  PF1's own rules already make legal.
- **`CANONICAL_FIGHTER_FEAT_CHOICES`'s own claim-blocking validator is deliberately left
  unchanged this cycle.** It still claim-blocks a real player's non-canonical weapon-training
  group selection at these 4 choice ids (e.g. `group:axes` at tier 1) as a "non-canonical feat
  choice" diagnostic -- this is a PRE-EXISTING constraint on the deterministic-fixture proof
  surface, not something this cycle's probe-driven synthetic inputs are subject to (the probe
  bypasses real seeding entirely), and widening it to accept 14 legal values per slot instead
  of one is a genuinely separate, larger change to a different function with a different job
  (bounding which builds this proof surface accepts at all, not which explanations a given
  build produces). Named here so a future cycle does not assume it was silently widened too.
- **Bucket-U-cycle payoff comparison, stated as instructed:** the dispatch brief's own stated
  baseline was "a book-scoped predicate moved 40 [where a generic one moved 110]." This
  cycle's own generic predicate is expected to move 48 `core_rulebook` units in one cycle,
  versus the ~2-9 units per cycle the book-scoped, one-school-at-a-time pattern (cycles 4, 6,
  7, 8) delivered on this SAME mechanism -- a comparable order-of-magnitude payoff from
  recognizing the closed-enumerable-set shape before writing code, not from more code.

## Next-cycle plan

1. **Confirm this cycle's expected 48-unit closure** once the wave's shared regeneration
   lands; if the actual delta disagrees with 48, that is itself a finding (per this wave's own
   instruction: "An expectation that turns out wrong is a useful finding, not a failure").
2. Re-derive this mechanism's full remainder partition fresh before picking the next lever
   (`decisions.md §12` L2) -- do not carry this receipt's own remainder table forward
   unverified.
3. `Domain Power` (56) and `Domain Base` (33) remain the two largest named sub-causes,
   unstarted by any lever used so far; a future cycle should investigate whether either has a
   comparably cheap real-formula path, or a comparably generalizable closed-enumerable-set
   shape.
