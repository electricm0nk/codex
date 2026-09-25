# F3c4 (second attempt): sorcerer bloodlines print from the converted record (SD-36 Epic F3, engine)

This is the engine half of FS-17. F3c4's first attempt (`713bcfba74`, `f3c4-receipt.md`) stopped
because no converted path reached a bloodline's lines (0 of 287). F3c4b (`07f02c6387`,
`f3c4b-receipt.md`) converted the `CATEGORY:Sorcerer Bloodline` pick rows as `pool_option` rules, which
made 269 of 287 lines reachable through the pick. This step links the character's pick to that option
and lets the engine use it.

**Invariants:**

- No converter change. `data/corpus/**`, `data/sheet_rules/**` and `site/**` are untouched
  (`git diff --quiet -- data/corpus data/sheet_rules site` is clean).
- The record count stays **49,450** (`check_frozen_status.py --check`: OK).
- `python3 scripts/pcgen_residue_gate.py --check --closure` is `verdict=PASS`.

## 1. The rule: one link from a Path-A pick to its converted option

The problem: the character records its bloodline in the legacy id space,
`choice:sorcerer_bloodline -> bloodline:draconic`. The package offers the pick as
`Granter::Choice(core_rulebook:class_feature:sorcerer_standard_bloodline_selection)` on
`core_rulebook:pool_option:sorcerer_bloodline_draconic_bloodline`. Nothing connected the two.

`sheet_rule::link_path_a_picks` connects them with one rule for every pool. It is the inverse of the
census's `carrier_pick_for_option` (F3c2). A pick `choice:<pool> -> <ns>:<member>` names:

1. the converted record whose slug is `<pool>_<member>` and which carries the pool as its own tag
   (`Sorcerer Bloodline`, the oracle key `Sorcerer Bloodline ~ Draconic`);
2. the option that grants that record (`Granter::Rule(<option>)`) and that is itself granted by a
   choice the character is offered (`Granter::Choice(<chooser>)`).

A chooser is **offered** (`sheet_rule::chooser_offered`) in either of two cases:

- the held set holds it;
- it is a standalone choice rule with no granter whose gate states a condition that includes. For
  example, `sorcerer_standard_bloodline_selection` is gated `Holds(sorcerer_standard_bloodline)`.

A standalone chooser gated `Always` is never offered, because nothing ties it to the character. The
first sweep showed why this matters: `arcanist_bloodline_development_selection` is gated `Always`, and
admitting it made Ectoplasm, Ghoul and Psychic ambiguous, so they did not link.

When no option answers a pick, or more than one does, the pick is not linked. The engine never
guesses.

**Where the link is used:**

- `CharacterFacts::with_linked_picks` records the link under the chooser. It is called by the sheet
  (`with_sheet_rules`), the feat-prerequisite facts (`feat_prereqs.rs`), the desktop's feat options
  (`character_hub.rs`) and the census's held-set dump.
- `sheet_rule_package::linked_picks(input)` gives the pilot compute the same links. It builds the held
  set from the character's race and class levels, and caches the result per (race, classes, picks).
  Each link says whether the held set holds the option once the pick is recorded (`option_held`).

## 2. What changed in the engine

### The class-skill union reads the character's own picks

`class_skill_view_for(input, class, level)` returns the class's converted class skills plus the
character's linked picks, counted only where this class offers the chooser.
`selected_skill_class_skill` uses it.

Worked example, hand-worked on the census fixture: an Aquatic sorcerer's Swim is
1 rank + Str 4 + class skill 3 (CRB p.87) − ACP 2 (CRB p.150) = **6**. F3c4 predicted a flip without
this change would print 3.

### The Sorcerer module yields to the converted record

The module's only change is a new branch (`class_sorcerer_wizard.rs`), which runs for a bloodline the
module does not model when its option is held. It follows the F1b precedent: a bespoke number is
kept wherever the module computes one, and the record prints the text. The branch:

- emits `class_feature.sorcerer.bloodline.converted_record`, a recognition record worth +0 that names
  the option and the chooser;
- does not emit the blocker.

The held set then holds the option, the record and its lines at the levels the record states:

- the class skill;
- bonus spells;
- the bonus feat tracker;
- arcana;
- the powers.

Some cases still block:

- **Linked, but the option is not held.** The module adds
  `class_feature.sorcerer.bloodline.converted_option_not_held` (claim-blocking) to the existing
  blocker. This covers Imperious and Kobold, whose pick rows are race-gated (FS-19).
- **Not linked** (an invented bloodline). The bloodline-agnostic blocker stands unchanged.

Arcane's bespoke branch is untouched:

- Its tests stay green.
- The lib test list filtered to `arcane|sorcerer` is byte-identical before and after (51 lines,
  `cargo test --locked -j 8 --lib -- --list`).
- The census's Arcane sorcerer row does not move (§5).

### The SD-32 generic pool-group pass yields for a linked, held pick

`push_generic_pool_group_selection_magnitude` evaluates every `<group> ~ <member>` corpus record at the
character's level with no level gate. It also reads members that belong to other classes' records,
such as `Draconic Bloodline ~ Bloodrager` and `~ Crossblooded`.

For a selection whose pick links to a held option, the pass now prints nothing. The held set holds
each member line only at the level the record's own grant gate states, and the sheet prints the
line's value from the record.

The member resolver was extracted unchanged as `generic_pool_group_member_lines`, so the scan in §4
can measure what the pass would have printed.

## 3. RED to GREEN (`f3c4-red.log`; GREEN in `f3c4-verify.log`)

RED was run on the F3c4b engine (`07f02c6387`) with the new test file. GREEN was run on the final
tree. All tests are in `tests/sd36_sorcerer_bloodline_record.rs`.

| test | RED | GREEN |
|---|---|---|
| `a_sorcerer_with_the_draconic_bloodline_computes_at_level_5` | Blocked on `arcane_bond_and_bloodline_progression.unsupported` | ok: Computed; BAB 2; base saves 1/1/4; HP 32; AC 18; Perception held; Dragon Resistances "+1"; no Breath Weapon, Wings or Power of Wyrms |
| `a_draconic_sorcerer_9_holds_breath_weapon_and_the_second_natural_armor_step` | Breath Weapon not held (no link) | ok: Breath Weapon held; Dragon Resistances "+2"; no Power of Wyrms |
| `an_aquatic_sorcerer_prints_swim_6_on_the_census_fixture` | Blocked | ok: Swim 6, Climb 3, Intimidate 3 at levels 1, 5 and 20 |
| `a_draconic_sorcerer_5_prints_no_breath_weapon_dc` | `...breath_weapon.sorcererdraconicbreathweapondc = 14` (and dice 10, times 1) | ok |
| `a_draconic_sorcerer_5_prints_no_power_of_wyrms_blindsense` | `...power_of_wyrms.blindsenserange = 60` | ok |
| `a_draconic_sorcerer_5_prints_no_natural_armor_2` | `...dragon_resistances...naturalarmorbonus = 2` | ok (the bespoke value is 1) |
| `an_arcane_sorcerer_5_prints_no_arcane_apotheosis` | `...arcane_apotheosis.sorcererarcanebloodlinepower3 = -1` | ok (still Computed) |
| `an_arcane_sorcerer_still_prints_swim_3`, `an_invented_bloodline_stays_blocked_by_name` | ok (controls) | ok |
| `the_class_skill_union_reads_the_characters_bloodline_pick` | added after RED; it uses the new `class_skill_view_for` | ok: Draconic has Perception; Aquatic has Swim; Arcane has neither; sorcerer's own skills kept |
| `every_bloodline_prints_the_class_skill_bonus_its_record_grants` | added after RED | ok: 32 records; Climb/Swim +3 printed exactly when the record's own class-skill edge names the skill |

**The hand-worked oracle** is in the test module doc:

- Draconic sorcerer 5 (CRB p.71-72 Table 3-14; p.75):
  - BAB +2; base saves Fort +1, Ref +1, Will +4.
  - HP = d6 max 6 + Con 2 at 1st, then 4 × (average 4 + 2) = **32**.
  - Class skill: Perception.
  - Dragon Resistances: +1 natural armor at 3rd, +2 at 9th.
  - Breath Weapon comes at 9th, Wings at 15th, Power of Wyrms at 20th.
- Aquatic Swim (APG p.136; CRB p.87, p.150): 1 + 4 + 3 − 2 = **6**.

## 4. Counts, with denominators and commands

### Bloodline sweep (`f3c4-bloodline-sweep-after.md`, `.log`)

The sweep is `artifacts/epic-f/scripts/f3c4b_bloodline_sweep.rs`, unchanged. It was copied to
`tests/zz_probe_f3c4.rs`, run with `cargo test --locked -j 8 --test zz_probe_f3c4 -- --nocapture
--test-threads=8`, then deleted.

**30 of 32** bloodlines are Computed single-class at every level 1..=20. The figure was 1 of 32 at
F3c4 and at F3c4b. The remainder, by mechanism:

- **2 of 32, Imperious and Kobold (FS-19).** The pick links, but the option's own race-template gate
  never holds. They are Blocked on `class_feature.sorcerer.bloodline.converted_option_not_held` and
  the bespoke blocker, at 20 of 20 levels.
- **Lines not held at the level their own gate states: 20 of 287.** These are the same two
  bloodlines' 18 lines (FS-19), plus Draconic and Abyssal Claws, which are held from 7 instead of 1
  (FS-18, a converter single-line principal). No sheet total reads them.

### Generic pool-pass scan (`f3c4-pool-pass-scan.log`)

The script is `artifacts/epic-f/scripts/f3c4_pool_pass_scan.rs`. It was appended temporarily to
`pool_groups.rs`, run with `cargo test --locked -j 8 --lib -- --test-threads=8 --nocapture
f3c4_pool_pass_scan` (2,405 s), then removed.

**Denominators:**

- Every pool the pass prints: 6 callers.
- Every selection whose slug resolves to a real corpus group through the pass's own
  `real_pool_group_for_selection_slug`. Candidates are each corpus group name slugged, with and
  without the pool suffix.
- Levels 1..=20.
- A human fighter-fixture character with that class.

| pool | selections | linked + held (pass yields) | member values before | after | on linked selections: held at that level | **NOT held (the shape)** | unlinked, still printed |
|---|---:|---:|---:|---:|---:|---:|---:|
| Sorcerer Bloodline | 53 | 30 | 9,800 | 1,040 | 6,938 | **1,822** | 1,040 |
| Bloodrager Bloodline | 12 | 10 | 1,880 | 220 | 1,522 | **138** | 220 |
| Cleric Domain | 73 | 0 | 2,960 | 2,960 | -- | -- | 2,960 |
| Shaman Spirit | 14 | 0 | 1,240 | 1,240 | -- | -- | 1,240 |
| Warpriest Blessing | 37 | 0 | 0 | 0 | -- | -- | 0 |
| Cavalier Order | 2 | 0 | 0 | 0 | -- | -- | 0 |

**The shape is common.** On the linked selections, **1,960 of 10,420** member values the pass printed
were at a level, or for a record, that the held set does not hold. Examples at Draconic 5 include
Breath Weapon DC 14, Power of Wyrms blindsense 60, Wings, `~ Bloodrager`, `~ Crossblooded` and
`~ Standard`. Arcane 5 printed Arcane Apotheosis −1.

**Held is not the same as correct.** A member held at that level can still carry a wrong pass value.
At Draconic 5, Dragon Resistances is held, but the pass printed natural armor 2 where the record and
the book give 1. The yield removes those values too. The number of held-but-wrong values was not
measured separately.

**The remainder is named as FS-21.** For a selection the package does not link, the held set cannot
decide a member's level, so the pass still prints. That is 5,460 member values: all Cleric and
Shaman selections, 23 sorcerer groups (wildblooded and other groups with no
`sorcerer_bloodline_<x>` option, plus Imperious and Kobold) and 2 bloodrager groups. None of them
feeds a sheet total.

**The Cavalier figure is a floor.** The Cavalier count of 2 selections comes from the scan's
candidate rule. `Order of the <X>` names that do not slug to an order selection are not enumerated.

### Census

The command is `cargo run --locked -j 8 --bin class_census -- --json <scratch>/census-f3c4.json`,
and the output was copied to `artifacts/epic-f/census-f3c4.json`:

```
ids=137 computed=63 blocked=0
prestige_swept=74 prestige_alone_blocked=74 prestige_mix_computed=67 prestige_mix_unknown=0
mix_panel_swept=185 mix_panel_computed=185 mix_panel_blocked=0
```

A per-key diff against `census-f3c3.json` finds 13 keys changed. One is `generated_at`. The other 12
are `prestige/18` (dragon_disciple), `mixes/0/blocking_diagnostics[1]`: the sorcerer bloodline blocker
is gone from its 10 levels. No other row moves, and that includes the Arcane sorcerer.

**Prestige mixes: 67 of 74, not the 68 the brief expected.** Dragon Disciple's carrier mix (sorcerer 5
with the Draconic pick, prestige 1..=10) now carries ONE blocker, down from two:
`combat.baseline_weapon_proficiency_unknown`.

- **Why it remains:** its converted closure is not attested complete. Its one closure defect is
  `Internal|Bite` (`ce_abilities_race.lst:249`), a natural-attack helper row outside the inventory
  (mechanism N, FS-20).
- **Why no engine change can decide it:** a proficiency answer over an incomplete closure is Unknown
  by doctrine. Assuming the helper grants no proficiency would be a guess about a row the engine
  cannot read.
- **The named remainder, 7 of 74:** the 6 FS-15 save-formula rows and dragon_disciple (FS-20).

**Baselines:**

- `BASELINE_CENSUS_PRESTIGE_MIX_COMPUTED` stays 67, with a dated 2026-09-25 re-measure reason in
  `scripts/verify-baselines.env`.
- `scripts/check_class_census_baselines.py` with the env floors (137/63/74/185/67): OK.
- `python3 scripts/gen_class_status_table.py --check --json .../census-f3c4.json`: OK.

## 5. Moved pins (retro-logged, `docs/retro/events/sd36-f3c4-executor.jsonl`)

- **`class_census::tests::f3c_carriers_named_through_at_least_any_and_not_reach_the_engine`:**
  dragon_disciple goes from 2 blockers to 1, `combat.baseline_weapon_proficiency_unknown`.
- **`pool_groups` generic pass tests** (celestial, abyssal, bloodrager undead, bloodrager arcane): the
  pins were "generic count > 0". They now assert a count of 0 and non-empty member lines from the held
  set (`held_member_lines`). The test names are unchanged.
- **`tests/sd13_sorcerer_level1_spell_baseline.rs::sorcerer_level1_with_non_arcane_bloodline_choice_stays_bloodline_agnostic`:**
  Draconic now yields: there is no blocker, and the recognition record names the Draconic option and
  no Arcane fact. The bloodline-agnostic blocker is pinned on an unlinkable bloodline instead.
- **The brief's expected figure** of "prestige mixes 67 → 68": measured at 67 (§4).

## 6. Verify (`f3c4-verify.log`, final tree)

| command | result |
|---|---|
| `cargo test --locked -j 8 --lib -- --test-threads=8 sorcerer` | 44 passed, 0 failed |
| `... --lib ... multiclass` | 37 passed, 0 failed |
| `... --lib ... class_census` | 33 passed, 0 failed |
| `cargo test --locked -j 8 --lib -- --test-threads=8` | 2,723 passed, 0 failed, 6 ignored |
| `--test sd36_sorcerer_bloodline_record` (new) | 11 passed |
| `--test sd36_bloodline_pick_option` / `sd36_class_skill_from_record` / `sd36_multiclass_any_class` | 2 / 5 / 11 passed |
| `--test sd13_sorcerer_level1_spell_baseline` | first pass 14 of 15: `sorcerer_level1_with_non_arcane_bloodline_choice_stays_bloodline_agnostic` (Draconic now yields; pin moved, §5), then 15 passed |
| lib `--list` filtered `arcane\|sorcerer`, before vs after | identical (51 lines) |
| `cargo clippy --locked --tests -j 8 -- -D warnings` (root) | first pass `items_after_test_module` (fixed), then clean |
| `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` (root, final tree) | 291 result lines, **6,369 passed, 0 failed**, 27 ignored |
| `cargo test --locked -j 8 --no-fail-fast --manifest-path apps/desktop/src-tauri/Cargo.toml -- --test-threads=8` | 615 passed, 0 failed |
| `python3 scripts/pcgen_residue_gate.py --check --closure` | `verdict=PASS` (identifier 0, shipped 0 of 70,043, live 0) |
| `python3 scripts/site/check_frozen_status.py --check` | OK, frozen at 100% (49,450 units) |
| `git diff --quiet -- data/corpus data/sheet_rules site` | clean |
| census baselines / status table | OK / OK (§4) |

## 7. What stays open, by mechanism

- **FS-19:** Imperious and Kobold, race-template gate. Blocked by name.
- **FS-18:** Claws single-line principal, held late. Text only.
- **FS-20:** Dragon Disciple `Internal|Bite`. Prestige mixes stay 67 of 74.
- **FS-21:** the generic pass on unlinked selections, 5,460 member values. Its level gate is not
  decidable until those pools have converted pick options.
