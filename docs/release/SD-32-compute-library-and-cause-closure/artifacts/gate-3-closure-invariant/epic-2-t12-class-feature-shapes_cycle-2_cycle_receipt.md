# Cycle epic-2-t12-class-feature-shapes-cycle2 — Gate 3 (closure invariant) / Card 11 (row 15's T12 remainder)

- **Card ID:** 11 (`epic-2-cause-closure`), row 15's named T12 sub-scope: 101 magnitude-bearing
  `untabled_base_class_feature_roster` records across 18 classes, remaining after the prior cycle
  (`epic-2-t12-class-feature-shapes_cycle-1`) closed Antipaladin's 7.
- **Commit SHA:** see `git log -1` at push time (rebased onto `origin/tranche/12` before pushing, §5)
- **Files touched:**
  - `src/rules_core/rules_tables/ultimate_psionics/cryptic_features.rs` — **new**: 6 real per-feature
    compute functions for Cryptic
  - `src/rules_core/rules_tables/ultimate_psionics/dread_features.rs` — **new**: 6 real per-feature
    compute functions for Dread
  - `src/rules_core/rules_tables/ultimate_psionics/marksman_features.rs` — **new**: 5 real per-feature
    compute functions for Marksman
  - `src/rules_core/rules_tables/ultimate_psionics/psychic_warrior_features.rs` — **new**: 3 real
    per-feature compute functions for Psychic Warrior
  - `src/rules_core/rules_tables/ultimate_psionics/soulknife_features.rs` — **new**: 4 real per-feature
    compute functions for Soulknife
  - `src/rules_core/rules_tables/ultimate_psionics/mod.rs` — registered the five new modules
  - `src/rules_core/pilot_compute/mod.rs` — 5 new `ground_<class>_class_features` functions, dispatch
    wiring in `compute_class_chassis`'s `untabled_base_class_chassis::resolve` arm, 6 end-to-end wiring
    tests (5 per-class level-20 magnitude tests + 1 combined below-min-level absence test)
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 11 cycle entry prepended
  - `docs/retro/events/sd31-transcribe.jsonl` — append-only auto-log line from this cycle's own
    `scripts/verify.sh --only preflight-oracle` bootstrap run (not hand-edited)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped diff, this cycle's touched files only — no
  bundle/wave/tranche identifier leaks in new code)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope — no stub markers, no placeholder
  returns; every new function is a real formula transcribed from the corpus's own `BONUS:VAR`/`DEFINE`
  tokens and reaches `compute_pilot_base_chassis` through the real dispatch path, proven by the wiring
  tests)
- **Acceptance criterion:** re-derive the 101-record/18-class remainder honestly, look for shared
  compute shapes across classes before writing per-class code (§17), close as many whole classes as
  possible end-to-end following the Antipaladin template, and report exactly which classes remain.
- **Corpus SHA:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`)
  — every formula in this cycle was re-derived directly against this pinned oracle checkout, no
  `data/corpus/**` write.
- **Status:** complete (this cycle's scope only — does not close row 15 or card 11; 77 of 108
  magnitude-bearing records across 14 classes remain, plus `psion`'s third convention)
- **Discovery forwards:** none new — this cycle worked entirely off the prior cycle's own
  `psion`/case-fold discoveries.
- **Next-cycle plan:** the remaining 77 magnitude-bearing records across 14 classes (Aegis 7,
  Kineticist 6, Magus 5, Medium 7, Mesmerist 10, Occultist 6, Psychic 4, Shifter 5, Spiritualist 3,
  Tactician 6, Vigilante 7, Vitalist 6, Wilder 5), worked one class at a time the same way this cycle
  worked Cryptic/Dread/Marksman/Psychic Warrior/Soulknife (six of `occult_adventures`' classes —
  Kineticist, Medium, Mesmerist, Occultist, Psychic, plus Spiritualist — share `oa_abilities_class.lst`
  the way this cycle's five shared `up_abilities_class.lst`, and are the next highest-leverage single
  oracle-reading pass); `psion`'s genuinely-third convention, once scoped by a cycle that walks its
  chained per-discipline `ABILITY:` indirection.

---

## 1. Re-deriving the remainder (§17a — re-derive, don't inherit)

Re-ran the prior cycle's own re-derive command against the regenerated fixture (no `data/corpus/**`
write this cycle, so the fixture is unchanged from the base commit):

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
Confirmed **108 across 19 classes**, unchanged from the prior cycle's own re-derivation — the prior
cycle's fixture regen already landed, and no sibling lane has touched this fixture since (`git log -1
--format=%H -- tests/fixtures/rules_core/untabled-base-class-feature-roster.json` at this cycle's base
commit is still the prior cycle's own commit). Antipaladin (7) is closed; this cycle's target is the
other 101.

## 2. Source book grouping — looking for shared shapes before writing per-class code (§17)

Cross-referenced each class's own `source_book` in `tests/fixtures/rules_core/untabled-base-class-
chassis.json` against the 101-record breakdown, to find the highest-leverage single oracle-reading pass:

```
ultimate_psionics:  aegis 7, cryptic 6, dread 6, marksman 5, psychic_warrior 3, soulknife 4,
                     tactician 6, vitalist 6, wilder 5   (= 48 records, 9 classes, one source file:
                     up_abilities_class.lst / up_classes.lst)
occult_adventures:  kineticist 6, medium 7, mesmerist 10, occultist 6, psychic 4, spiritualist 3
                     (= 36 records, 6 classes, one source file: oa_abilities_class.lst)
ultimate_magic:     magus 5   (1 class, 1 record own file)
ultimate_wilderness: shifter 5   (1 class)
ultimate_intrigue:  vigilante 7   (1 class)
```

`ultimate_psionics` is the single largest cluster (48 of 101, 9 classes, one source file pair). This
cycle worked five of its nine classes (Cryptic, Dread, Marksman, Psychic Warrior, Soulknife — 24 of the
48) as one oracle-reading pass against `up_abilities_class.lst`/`up_classes.lst`, picked because they
were the five with the fewest "None"-var (choice-only, no independent `BONUS:VAR`) roster rows to
resolve by hand, keeping the pass mechanical rather than judgment-heavy. Aegis, Tactician, Vitalist, and
Wilder remain in this same book, named for the next cycle.

## 3. Four distinct compute shapes closed the 24 new records, no fifth needed

Every one of this cycle's 24 records (plus a re-check against the prior cycle's 7 Antipaladin records)
resolves to one of four shapes:

1. **Flat/constant** — no level term at all (`disrupt_pattern_range_feet` = 30,
   `aura_of_fear_penalty` = -4, `fear_incarnate_damage_reduction` = 10,
   `ranged_specialist_critical_multiplier_bonus` = 1, `psychic_strike_die_size` = 8,
   `quick_draw_uses_per_round` = 1, `eternal_warrior_uses_per_day` = 1).
2. **`level`-scaled linear/floor-division** — `trapmaker_bonus` = level,
   `devastating_touch_bonus_damage` = level, `warriors_path_level` = level,
   `form_mind_blade_level` = level, `altered_defense_damage_reduction` = (level+3)/4,
   `enhanced_disruption_bonus_dice` = (level-1)/2, `hidden_pattern_stealth_bonus` =
   2*min(3,(level+1)/3), `unchanging_pattern_power_resistance` = 12+level, `fearsome_insight_bonus` =
   max(1,level/2), `wind_reader_uses_per_day` = 3+level, `evade_arrows_ac_bonus`/
   `favored_weapon_base_bonus` = (level+2)/4, `pathweaving_uses_per_day` = (level-12)/3,
   `enhanced_mind_blade_max_enhancement_bonus` = min(level/3,5).
3. **`level + ability_modifier`** — `terror_uses_per_day` = level+CHA, `cover_fire_dc` =
   10+DEX+level/2 (matches Antipaladin's `cruelty_dc`/`channel_negative_energy_dc` shape exactly).
4. **`ability_modifier`-only** — `shadow_twin_uses_per_day` = CHA (no level term at all, the one
   record this cycle found where the granting book scales a "uses per day" purely by stat, not level —
   worth flagging for the next cycle's classes, since it is easy to assume "uses per day" is always
   shape 3).

No record in either cycle (31 closed total) needed a shape outside these four. This is the number the
dispatch brief asked to be reported: **the 101-record remainder needs at most these same four shapes**,
not a new mechanism per class — confirmed empirically by this cycle's own 24, all of which fell into
shapes already used by Antipaladin's 7.

## 4. Five classes closed end-to-end, real compute + real wiring

Built real per-feature compute functions for all magnitude-bearing records of Cryptic (6), Dread (6),
Marksman (5), Psychic Warrior (3), and Soulknife (4) — every formula transcribed from the corpus's own
already-ingested `BONUS:VAR`/`DEFINE` tokens (`data/corpus/ultimate_psionics/class_feature/<class>/
*.json`), not from memory of the printed rulebook. Full formula table and source-line citations are in
each module's own doc comments (`src/rules_core/rules_tables/ultimate_psionics/{cryptic,dread,
marksman,psychic_warrior,soulknife}_features.rs`).

Two judgment calls, documented in the modules' own doc comments rather than silently made:

- **Marksman's `Favored Weapon` and Soulknife's `Form Mind Blade`** both have empty/no-magnitude
  `DESC:` text in the corpus (a `VISIBLE:NO` choice-grant row in one case, a pure-grant row in the
  other) but each carries a real internal `BONUS:VAR` token (`FavoredWeaponBase`, `MndBladeLVL`) that
  feeds a downstream feature. Grounded those tokens as the record's own magnitude rather than treating
  the record as text-only — consistent with `decisions.md §27b`'s "F0 reached by fallthrough is not an
  answer": these are real measured values, not placeholders.
- **Soulknife's `Psychic Strike`** and **Cryptic's `Enhanced Disruption`**: the roster's own census
  script picks the *last* `%N`-substituted variable name from a multi-substitution `DESC:` string as the
  record's tracked var (confirmed by inspecting the prior cycle's roster JSON), which is not always the
  level-scaled one (e.g. Psychic Strike's roster var is the flat `PsychicStrikeDieType`, not the scaled
  `PsychicStrikeDice`). Ground exactly the var the roster names, documented in the doc comment which
  companion token carries the level-scaled quantity, so a future reader is not misled into thinking the
  census missed something.

Wired into `compute_class_chassis` via five new `else if` arms on `class_level.class_id`, immediately
following the existing Antipaladin arm, each gated by the class's own `"class:<name>"` id (confirmed
against `untabled-base-class-chassis.json`'s own `class_id` field for every one of the five).

## 5. RED→GREEN, proven live (§1a)

Mutated `dread_features::terror_uses_per_day` to add `+99`:
```
test ...dread_features::tests::terror_uses_combine_level_and_charisma ... FAILED
  left: Some(104)
 right: Some(5)
test ...dread_level_20_reaches_every_real_magnitude_with_a_real_charisma_score ... FAILED
  left: 122
 right: 23
```
Both the unit-level compute test and the end-to-end wiring test failed for the intended reason.
Reverted, re-ran:
```bash
cargo test --locked --lib -- cryptic_features dread_features marksman_features \
  psychic_warrior_features soulknife_features cryptic_level_20 dread_level_20 marksman_level_20 \
  psychic_warrior_level_20 soulknife_level_20 each_new_class_lacks
```
39/39 green (also re-ran the full `untabled_base_class*`/`antipaladin` suite: 39/39, no regressions —
command and full output in this cycle's transcript).

## 6. Sweep (§3)

No pinned count in `tests/`/`src/`/`scripts/`/`apps/` references the 108/101/80 figures by a literal
that would go stale from this cycle's change — this cycle changed which records are *closed*, not the
roster's own total, so no count-pinning file needed updating. Grepped `class_feature.untabled.` id
prefixes for the five new classes across `src/`/`tests/` to confirm no other file assumed a fixed set
of ids for these classes; none did.

## 7. Scope discipline

Did not attempt: the remaining 77 magnitude-bearing records across 14 classes (named above, by class
and count); `psion`'s genuinely-third convention (still sized, not closed — untouched this cycle); the
pool-shaped groups `census_untabled_base_class_feature_roster.py`'s own doc comment already excludes by
design; row 15's own 27,847 kind-unenumerable-object scope (untouched, different sub-lane). No class was
filed as out of scope — every remaining class is named with its exact count for the next cycle
(`decisions.md §27b`: cost/awkwardness/"needs a new mechanism" are not grounds for exclusion; this
cycle closed 5 whole classes and is handing off the other 14 whole, not fragmenting any of them).

Row 11 left `in-progress` per dispatch instruction.

`df -h /`: reported in the dispatch's final report.
