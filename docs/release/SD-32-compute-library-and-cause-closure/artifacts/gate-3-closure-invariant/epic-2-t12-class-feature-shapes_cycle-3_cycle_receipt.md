# Cycle epic-2-t12-class-feature-shapes-cycle3 — Gate 3 (closure invariant) / Card 11 (row 15's T12 remainder)

- **Card ID:** 11 (`epic-2-cause-closure`), row 15's named T12 sub-scope: after cycle 2 closed 24 more
  (Cryptic, Dread, Marksman, Psychic Warrior, Soulknife) joining Antipaladin's 7 (31/108 total), this
  cycle's target was the 77-record/14-class remainder, prioritizing the four `ultimate_psionics` classes
  cycle 2 named and left for next (Aegis, Tactician, Vitalist, Wilder).
- **Commit SHA:** see `git log -1` at push time (rebased onto `origin/tranche/12` before pushing, §5)
- **Files touched:**
  - `src/rules_core/rules_tables/ultimate_psionics/aegis_features.rs` — **new**: 7 real per-feature
    compute functions for Aegis
  - `src/rules_core/rules_tables/ultimate_psionics/tactician_features.rs` — **new**: 6 real per-feature
    compute functions for Tactician
  - `src/rules_core/rules_tables/ultimate_psionics/vitalist_features.rs` — **new**: 6 real per-feature
    compute functions for Vitalist
  - `src/rules_core/rules_tables/ultimate_psionics/wilder_features.rs` — **new**: 5 real per-feature
    compute functions for Wilder
  - `src/rules_core/rules_tables/ultimate_psionics/mod.rs` — registered the four new modules
  - `src/rules_core/pilot_compute/mod.rs` — 4 new `ground_<class>_class_features` functions, dispatch
    wiring in `compute_class_chassis`'s `untabled_base_class_chassis::resolve` arm, 4 new end-to-end
    wiring tests (level-20 magnitude tests) plus 4 new cases added to the shared
    `each_new_class_lacks_its_highest_gated_magnitude_one_level_early` table
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 11 cycle entry prepended
  - `docs/retro/events/sd31-transcribe.jsonl` — append-only auto-log line from this cycle's own
    `scripts/verify.sh --only preflight-oracle` bootstrap run (not hand-edited)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped diff, this cycle's touched files only — no
  bundle/wave/tranche identifier leaks in new code)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope — no stub markers, no placeholder
  returns; every new function is a real formula transcribed from the corpus's own `BONUS:VAR`/
  `BONUS:ABILITYPOOL` tokens and reaches `compute_pilot_base_chassis` through the real dispatch path,
  proven by the wiring tests)
- **Acceptance criterion:** re-derive the 77-record/14-class remainder honestly, close the four
  remaining `ultimate_psionics` classes cycle 2 named as its next-cycle plan end-to-end following the
  established template, and report exactly which classes remain.
- **Corpus SHA:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`)
  — every formula in this cycle was re-derived directly against this pinned oracle checkout (bootstrapped
  fresh into this worktree's repo-local slot, `docs/release/SD-32-compute-library-and-cause-closure/
  artifacts/corpus/operator-supplied/pcgen/`, per this bundle's operator directive — never
  `~/workspace/repos/pcgen`), no `data/corpus/**` write.
- **Status:** complete (this cycle's scope only — does not close row 15 or card 11; 53 of 108
  magnitude-bearing records across 10 classes remain, plus `psion`'s third convention)
- **Discovery forwards:** none new.
- **Next-cycle plan:** the remaining 53 magnitude-bearing records across 10 classes (Kineticist 6,
  Magus 5, Medium 7, Mesmerist 10, Occultist 6, Psychic 4, Shifter 5, Spiritualist 3, Vigilante 7),
  worked the same way this cycle worked Aegis/Tactician/Vitalist/Wilder; `occult_adventures`'s
  Kineticist/Medium/Mesmerist/Occultist/Psychic/Spiritualist share `oa_abilities_class.lst` (36
  records/6 classes, one source file), the next highest-leverage single oracle-reading pass, leaving
  Magus (`ultimate_magic`), Shifter (`ultimate_wilderness`), and Vigilante (`ultimate_intrigue`) as
  single-class single-book tails; `psion`'s genuinely-third convention, once scoped by a cycle that
  walks its chained per-discipline `ABILITY:` indirection.

---

## 1. Re-deriving the remainder (§17a — re-derive, don't inherit)

```bash
python3 -c "
import json, collections
d = json.load(open('tests/fixtures/rules_core/untabled-base-class-feature-roster.json'))
mag = [e for e in d['entries'] if not e['text_only']]
print('total:', len(mag))
print(collections.Counter(e['class_id'] for e in mag))
"
```
```
total: 108
Counter({'mesmerist': 10, 'aegis': 7, 'antipaladin': 7, 'medium': 7, 'vigilante': 7, 'cryptic': 6,
'dread': 6, 'kineticist': 6, 'occultist': 6, 'tactician': 6, 'vitalist': 6, 'magus': 5, 'marksman': 5,
'shifter': 5, 'wilder': 5, 'psychic': 4, 'soulknife': 4, 'psychic_warrior': 3, 'spiritualist': 3})
```
Confirmed **108 across 19 classes**, unchanged — no sibling lane touched this fixture since the base
commit. Antipaladin (7), Cryptic (6), Dread (6), Marksman (5), Psychic Warrior (3), Soulknife (4) are
closed (31 total). This cycle's target: Aegis (7), Tactician (6), Vitalist (6), Wilder (5) = 24, the
next-cycle plan the prior cycle's own receipt named.

## 2. Oracle bootstrap (a fresh worktree's slot is empty)

`scripts/verify.sh --only preflight-oracle` initially resolved against a stale default
(`$HOME/workspace/repos/pcgen`), which this bundle's own directive forbids referencing. Bootstrapped
the repo-local slot per `artifacts/corpus/operator-supplied/README.md`:
```bash
export PCGEN_REPO_DIR="$PWD/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen"
export PCGEN_CORPUS_ROOT="$PCGEN_REPO_DIR/data"
scripts/fetch-pcgen-oracle.sh --dest "$PCGEN_REPO_DIR"
```
```
pcgen-oracle: OK 7f818006e371188e5717fd18d74d18a420747fc6 .../artifacts/corpus/operator-supplied/pcgen
```
All formulas below were re-derived directly against `up_classes.lst`/`up_abilities_class.lst` in this
repo-local checkout.

## 3. Every one of the 24 new records fits the four established shapes (plus one real two-term variant)

- **Flat/constant:** `astral_repair_hp` = 2, `psychic_enervation_percent` = 15, `surge_blast_range_feet`
  = 30.
- **`level`-scaled linear/floor-division:** `damage_reduction` (Aegis) = floor((level+4)/3),
  `form_astral_suit_custom_points` = 2+level+floor((level+1)/5) (two stacking `BONUS:VAR` rows on the
  same target variable, PCGen sums them), `craftsman_bonus` = floor((level+2)/4),
  `reconfigure_times_per_day` = floor((level-1)/2), `cannibalize_suit_times_per_day` =
  floor((level-10)/2), `improved_share_powers` = 1+floor((level+1)/6), `teamwork_feats_bonus_pool` =
  floor(level/6), `health_sense_level` = level (no ability term at all), `wild_surge_bonus` =
  1+floor((level+1)/4), `elude_attack_ac_bonus` = floor((level+2)/4), `surging_euphoria_duration_rounds`
  = same formula as `wild_surge_bonus` (the record's own `BONUS:VAR` reads the already-computed
  `WildSurge` variable directly).
- **`level + ability_modifier`:** `steal_health_damage` (Vitalist) = level+WIS, `steal_life_dc`
  (Vitalist) = 10+WIS+level/2 (matches Antipaladin's `cruelty_dc`/Marksman's `cover_fire_dc` exactly).
- **`ability_modifier`-only:** `augment_suit_duration_rounds` (Aegis) = INT,
  `coordinated_strike_times_per_day` (Tactician) = 3+INT, `strategy_times_per_day` (Tactician) = 3+CHA
  (reads the class's *secondary* stat, not its prime stat), `master_strategist_bonus` (Tactician) =
  INT, `transfer_wounds_times_per_day` (Vitalist) = 3+WIS, `request_aid_times_per_day` (Vitalist) =
  3+WIS.
- **A real two-term variant seen twice, not a new exclusion-worthy shape (`decisions.md §17`/`§27b`):**
  `collective_minds` in both Tactician and Vitalist = `max(ability_modifier, level/2)` — the greater of
  the class's prime/relevant stat and half class level. A genuinely different combining rule from the
  four above, but still a pure function of level and one ability modifier; implemented and tested like
  every other formula rather than treated as a blocker.

No record in this cycle needed a shape genuinely unmodelable by "a pure function of level and at most
one ability modifier" — the two `max()` cases are the only variation from the four the prior cycles
established, and both are ordinary compute, not novelty that blocks closure.

## 4. Four classes closed end-to-end, real compute + real wiring

Built real per-feature compute functions for all magnitude-bearing records of Aegis (7), Tactician (6),
Vitalist (6), and Wilder (5) — every formula transcribed from the corpus's own already-ingested
`BONUS:VAR`/`BONUS:ABILITYPOOL` tokens (`data/corpus/ultimate_psionics/class_feature/<class>/*.json`),
cross-checked against the oracle's own `up_classes.lst`/`up_abilities_class.lst` for every referenced
class-level variable (`AegisPrimeStat`=INT, `AegisCL`/`AegisDRLVL`=level; `TacticianPrimeStat`=INT,
`TacticianSecondaryStat`=CHA; `VitalistPrimeStat`=WIS; `WilderML`=level, no ability modifier used by any
of Wilder's five records).

Judgment calls, documented in the modules' own doc comments rather than silently made:

- **Tactician's `Collective`**: the roster's own tracked "var" field for this record is a `PREABILITY`
  gate clause the census script's last-`|`-segment heuristic mis-picked (the same class of quirk the
  prior cycle documented for Cryptic's Enhanced Disruption and Soulknife's Psychic Strike). Grounded the
  record's real first-substituted magnitude, `TacticianCollectiveMinds`, instead.
- **Vitalist's `Health Sense` and Tactician's `Teamwork Feats`**: both have `var: None` in the roster (no
  `%N`-substituted `DESC` text to name a var from) but each carries a real `BONUS:VAR`/
  `BONUS:ABILITYPOOL` token on the same corpus record. Grounded those tokens as the records' own
  magnitudes rather than treating them as text-only, consistent with `decisions.md §27b`.
- **Aegis's `Reconfigure`, Vitalist's `Request Aid`**: each record carries two independent `BONUS:VAR`
  tokens (e.g. `ReconfigurePoints`/`ReconfigureTimes`); grounded exactly the one the roster's own `var`
  field names, per the same discipline the prior cycle documented for Marksman/Soulknife.

Wired into `compute_class_chassis` via four new `else if` arms on `class_level.class_id`, immediately
following the existing five-class arm, each gated by the class's own `"class:<name>"` id (confirmed
against `untabled-base-class-chassis.json`'s own `class_id` field for every one of the four).

## 5. RED→GREEN, proven live at both altitudes (§1a)

Mutated `vitalist_features::steal_life_dc` to add `+99`:
```
test ...vitalist_features::tests::steal_life_dc_combines_level_and_wisdom_at_the_capstone_range ... FAILED
  left: Some(120)
 right: Some(21)
test ...vitalist_level_20_reaches_every_real_magnitude_with_a_real_wisdom_score ... FAILED
  left: 122
 right: 23
```
Both the unit-level compute test and the end-to-end wiring test failed for the intended reason.
Reverted, re-ran the full targeted scope:
```bash
cargo test --locked --lib -- aegis_features tactician_features vitalist_features wilder_features \
  aegis_level_20 tactician_level_20 vitalist_level_20 wilder_level_20 each_new_class_lacks \
  antipaladin cryptic_ dread_ marksman_ psychic_warrior_ soulknife_
```
`75/75 green`, no regressions in any pre-existing Antipaladin/Cryptic/Dread/Marksman/Psychic
Warrior/Soulknife test.

## 6. Sweep (§3)

`grep -rn "31/108\|31 of 108\|24 of 101\|101-record\|77 magnitude" tests/ src/ scripts/ apps/` — no
hits; this cycle changed which records are *closed*, not the roster's own fixed total (108/19), so no
count-pinning file needed updating.

## 7. Scope discipline

Did not attempt: the remaining 53 magnitude-bearing records across 10 classes (named above, by class
and count); `psion`'s genuinely-third convention (untouched this cycle); row 15's own 27,847
kind-unenumerable-object scope (untouched, different sub-lane). No class was filed as out of scope —
every remaining class is named with its exact count for the next cycle (`decisions.md §27b`: cost,
awkwardness, and the small `max()` compute variant found this cycle are not grounds for exclusion; this
cycle closed all four of its targeted classes whole and is handing off the other 10 whole, not
fragmenting any of them).

Row 11 left `in-progress` per dispatch instruction.

`df -h /`: reported in the dispatch's final report.
