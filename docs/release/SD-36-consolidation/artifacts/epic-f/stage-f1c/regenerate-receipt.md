# SD-36 Epic F1c -- regenerate receipt

The receipt for the F1c converter batch (spec: `../../../epic-f-class-completion.md` §3, Review
log findings 1 and 15). It covers defects D1-D8 as landed on `sd36/epic-f1c`:

| commit | step |
|---|---|
| 1e6b2db9ee | F1c-1, D1: type-selector grants convert |
| 5979ef4668 | F1c-2, D2: a line's condition gates only its own line |
| e61473e9c9 | F1c-3, D3 / D4 / D6 and PREABILITY `[<key>]` |
| af70b72679 | D5: stale static rows corrected; structural diff pinned |
| ea4d64eca8 | F1c-4, D7: always-held globals |
| 641691e283 | F1c-5, D8: variable-pool picks |

Baseline: `tranche/16` at 8057263014, checked out read-only as a detached worktree. It was removed
after the run. All figures were measured on 2026-09-23 at 641691e283.

## 1. Gates

| gate | command | result |
|---|---|---|
| converter freshness | `cargo run --locked --quiet -j 8 -p codex-ingest --bin sheet_rule_convert -- --check` | exit 0 |
| residue | `python3 scripts/pcgen_residue_gate.py --check --closure` | `verdict=PASS` (shipped_scanned 69,717, hits 0) |
| structural diff | `python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py data/sheet_rules --baseline <tranche/16 worktree>/data/sheet_rules` | `verdict=PASS`, exit 0 |
| frozen status | `python3 scripts/site/check_frozen_status.py --check` | OK, frozen at 100% (49,450 units) |
| corpus bundle | `node scripts/gen-corpus-bundle.mjs` | `files_copied=14029`; the tree stays clean |
| reader vs static, test (a) | `cargo test --locked -j 8 -p codex-ingest --test class_weapon_proficiency_via_converter every_static_row_equals_the_reader_answer -- --test-threads=8 --nocapture` | **42 of 42** static rows reproduced at level 1, 0 disagreements |
| reader remainder | `cargo test --locked -j 8 --lib every_census_class_has_a_known_proficiency_answer -- --test-threads=8 --nocapture` | 1 passed; Unknown 17 of 93 walked, all prestige |
| census | `cargo run --locked -j 8 --bin class_census -- --json docs/release/SD-36-consolidation/artifacts/epic-f/census-f1c.json` | `ids=135 computed=61 blocked=0` |
| status table | `python3 scripts/gen_class_status_table.py --json <census-f1c.json>` then `--check` | OK; `docs/architecture/status.md` already matched and did not change |

Package counts from the structural diff, tranche/16 -> now:

- records: 49,450 -> 49,450. converted: 49,450 -> 49,450. refused: 0 -> 0. The frozen count did not move.
- rules_written: 71,869 -> 73,016 (+1,147 rule ids; 0 of them with no named cause).
- var_tables: 5,910 -> 6,204 (+294 `_vars/` tables).
- `_defects/` files: +2 (`pool-member-unconverted.json`, `pool-pick-collision.json`, both from D8).
- Removed rule ids 0, removed `granted_by` edges 0, removed grants 0, unexpected field deltas 0.

## 2. Structural-diff delta classes

Each class is pinned by exact (rule id, field) pairs and new ids in
`../scripts/structural_diff_f1c_deltas.json`. `structural_diff.py` re-runs each class's own shape
check on every pinned pair. Any delta the pins do not cover gates the run.

| class | field deltas | rule ids touched | new rule ids | examples |
|---|---|---|---|---|
| **D1 type grants** (`ABILITY:<cat>\|AUTOMATIC\|TYPE=<tag>` -> one edge onto every converted record of the category carrying the tag) | growth only: `granted_by` +1,958 unique edges (1,942 rule + 16 class-line; reconciled in §3) | 301 source records with rule edges, plus 2 class records with class-line edges | 0 | `advanced_players_guide:class_feature:antipaladin` (+8 edges: `weapon_prof_auto/simple/martial`, `armor_prof_*`, `shield_prof*`, under its own `Antipaladin_CF_WeaponProficiencies == 0` gate); `ultimate_magic:class_feature:magus` (+4); `ultimate_combat:class_feature:gunslinger_proficiencies` (+4) |
| **D2 line split** (the record gets a Text principal; the first line's condition moves to a `#<suffix>` sibling) | 3,830 | 1,108 | 1,108 | `core_rulebook:class_feature:fighter_class` (the level-20 Weapon Mastery gate is on `#bonus0` only); `occult_adventures:class_feature:medium_class`; `advanced_class_guide:class:arcanist#bonus0` |
| **D3 Unchained** (a class principal `TakenOnClass <base>` per class-selection ability) | 0 | 0 | 4 | `pathfinder_unchained:class:unchained_barbarian`, `:unchained_monk`, `:unchained_rogue` (and `:unchained_summoner`) |
| **D4 attestation** (`closure_complete: true` on a class principal whose closure is defect-free) | 56 | 56 | 0 | `advanced_class_guide:class:ex_warpriest`, `advanced_players_guide:class:battle_herald`, `inner_sea_intrigue:class:lion_blade` |
| **D4 product-identity reclosure** (21 PI class records keyed on their own base row, not the codex placeholder) | 303 | 84 rule ids on 21 class records | 35 (21 split siblings, 14 continuation lines) | `adventurers_guide:class:golden_legionnaire` (1 -> 11 closure rows; reads Simple + Martial from its header `ABILITY:` row); `adventurers_guide:class:aldori_swordlord`; `inner_sea_world_guide:class:harrower` (PRETEXT in PRE syntax now converts) |
| **D6 weapon-choice offers** (option list resolved to oracle weapon names, or the pick linked to its child pool's one member) | 32 (25 resolved, 7 linked) | 32 rule ids on 30 records | 0 | `core_rulebook:class_feature:single_simple_weapon_proficiency` (`!PC[TYPE Simple]` -> 39 Simple names); `core_rulebook:class_feature:weapon_and_armor_proficiency_commoner` (linked); `advanced_class_guide:class_feature:adept_blade_weapon` |
| **PREABILITY `[<key>]`** (an exclusion item no longer converts to an unholdable `MissingRule` alternative) | 7 | 7 rule ids on 4 records | 0 | `pathfinder_unchained:class_feature:monk_unchained_class` (+ `#bonus1-3`, `applies`); `advanced_race_guide:feat:desperate_swing` (`prose`); `occult_adventures:class_feature:medium_inspiring_call` (`prose`; see follow-up F-1) |
| **D7 always-held** (`always_held: true` on the target of an unconditional `ABILITY\|AUTOMATIC` grant on a STAT/SAVE row) | 16 | 16 | 0 | `core_rulebook:class_feature:default`, `advanced_players_guide:ability:default`, `pathfinder_unchained:class_feature:default` |
| **D8 pool picks** (a record raising a category's POOL variable offers the pick; members get `Choice` edges) | 86 (`offers`) + 5,083 `Choice` edges | 86 | 0 | `advanced_players_guide:ability:summoner` (Summoner Class Selection -> Standard / Unchained class); `core_rulebook:class_feature:evocation_school`; `occult_adventures:class_feature:kineticist_infusion` |

Added grants: 216. These are FactGrants on the 21 PI class records (their class skills, read for
the first time by the D4 reclosure). `_vars/` +294: 248 tables added by F1c-1 to F1c-3 (af70b72679
receipt), plus 46 D8 pool variables (641691e283).

## 3. D1 edge reconciliation: 2,118 edges, and the 39-edge gap

The af70b72679 receipt counted the non-D8 added edges by **source** as 1,955 + 16 + 173 + 9 + 4 =
2,157. The structural diff counts 2,118. Those buckets were counted per list entry, but the
structural diff diffs each target's `granted_by` as a **set** (`edge_diff`, one JSON key per edge).
When a target's new list holds the same (source, gate) edge twice, the set counts it once. There
are 39 such repeats.

Re-counted by source over every rule id present on both sides. Command: an inline scan of
`data/sheet_rules` against the tranche/16 package. A D1 source is a record whose
`_defects/grant-by-type.json` rows were resolved: 589 of 613 rows, on 303 records. A PI source is
one of the 21 reclosed class records. Unchained sources are the 4 D3 class ids.

| bucket (by source) | per list entry | unique (set) | repeats |
|---|---|---|---|
| D1, `Granter::Rule` from a TYPE= source record | 1,955 | 1,942 | 13 |
| D1, class line (`ex_antipaladin`, `low_templar`) | 16 | 16 | 0 |
| PI reclosure, class line | 173 | 147 | 26 |
| PI reclosure, `Granter::Rule` | 9 | 9 | 0 |
| D3 Unchained, class line | 4 | 4 | 0 |
| **non-D8 total** | **2,157** | **2,118** | **39** |
| D8 `Granter::Choice` (after af70b72679) | 5,083 | 5,083 | 0 |
| **structural-diff total** | 7,240 | **7,201** | 39 |

The buckets sum exactly to 2,118 (non-D8) and 7,201 (the structural diff's TOTAL). Rule ids that
are new on this side carry 0 `granted_by` edges, so they add nothing.

The 39 repeats come from two mechanisms:

- **26 PI class-line repeats.** One class id is declared by two books: `hellknight` (9) and
  `red_mantis_assassin` (17), each in `adventurers_guide` and `inner_sea_world_guide`. Both
  records' level lines grant the same class feature at the same level under the same gate, so the
  target lists the edge twice. Example: `adventurers_guide:class_feature:mantis_doom`,
  `Class{red_mantis_assassin, at_level 9}` x2.
- **13 D1 rule repeats.** The oracle row states the selector twice on one record, or states two
  selectors whose tags the same target carries. Each statement writes the same edge.
  - `uc_abilities_class.lst:36` (Gunslinger ~ Proficiencies) carries
    `TYPE=WeaponProfMartial|TYPE=ArmorProfLight` in two `ABILITY:` tokens.
  - `cr_abilities_class.lst:562` (Cleric ~ Weapon and Armor Proficiency) repeats
    `Shield Prof|TYPE=ArmorProfMedium`.
  - `acg_abilities_class.lst:1383` (Shaman) carries `TYPE=ArmorProfLight` and
    `TYPE=ArmorProfMedium`. Both hit `armor_prof_light`, which is tagged ArmorProfLight,
    ArmorProfMedium and ArmorProfHeavy (`cr_abilities_class.lst:2804`).
  - Per source: picaroon_weapon_proficiency 3, barbarian_standard_class_full 2,
    barbarian_standard_ex_class 2, cleric_weapon_and_armor_proficiency 2, gunslinger_proficiencies 2,
    shaman_weapon_and_armor_proficiency 1, sohei_weapon_and_armor_proficiency 1.

A repeated identical edge cannot change what is held: a record is held or not, and the
attestation walk folds edges into sets (`attest.rs`). The repeats change no sheet value. They are recorded here
as follow-up F-2 (package tidiness), not as a defect in any total.

## 4. Census, before and after

tranche/16 (8057263014, `census-f1-reader.json`) -> sd36/epic-f1c (641691e283, `census-f1c.json`).
The committed `census-f1-reader.json` on this branch is byte-identical to `census-f1c.json` except
for `generated_at`.

| figure | before | after |
|---|---|---|
| ids | 135 | 135 |
| computed (of 61 non-prestige) | 58 | **61** |
| blocked (non-prestige) | 3 (antipaladin, magus, commoner) | **0** |
| prestige_alone_blocked (of 74; negative control, expected) | 74 | 74 |
| prestige_mix_computed (of 74) | 0 | 0 |
| prestige_mix_unknown (of 74) | 7 | 11 |
| mix_panel_computed (of 185) | 185 | 185 |
| reader-vs-static (of 42) | not measured here (tranche/16 pinned its disagreements) | 42 |
| reader remainder (Unknown of 93 walked) | 65 (3 non-prestige + 62 prestige) | 17 (0 non-prestige + 17 prestige) |

What moved:

- **antipaladin and magus: Blocked -> Computed** (D1).
- **commoner: Blocked -> Computed** (D6).
- **prestige_mix_unknown rose 7 -> 11.** The D4 reclosure now reads the real prerequisite rows of
  the PI classes. For harrower, hellknight_signifer, pathfinder_savant and storm_kindler those rows
  state the spell-level term only inside an AtLeast clause, so no carrier can be named. Their
  carrier went `fighter` -> none. Before, the dropped rows left an empty gate that read "met", and
  the census defaulted to fighter.
- **magaambyan_arcanist's carrier went fighter -> wizard** for the same reason. 12 other PI
  classes keep carrier fighter, but their entry gate now reads unmet (10) or unknown (2:
  rivethun_emissary, westcrown_devil) instead of met. Every prestige mix is Blocked both before and
  after (68 -> 64 mixes; the 4 carrier-less classes have none).
- **The baseline did not rise.** `BASELINE_CENSUS_COMPUTED` stays 61: 61 is the ceiling
  (`non_prestige_swept` = 61). It already sat at 61 from e61473e9c9. `scripts/verify-baselines.env`
  gains a dated re-measure note.

## 5. Reader remainder by mechanism

The full table is `../reader-remainder.md`, regenerated in this step. The non-prestige remainder
is empty. 17 prestige classes answer Unknown. Evidence was re-derived with
`python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/closure_defects.py <slug>...`,
committed in this step because the earlier scratch copy was lost.

| mechanism | classes | defect rows in their closures | classes |
|---|---|---|---|
| G: attestation false, unresolved reference in the class closure | 13 | 41 (38 unresolved-references, 3 undefined-variables) | aldori_swordlord, bellflower_tiller, cyphermage, diabolist, dragon_disciple, exalted, harrower, hellknight, hellknight_signifer, magaambyan_arcanist, rivethun_emissary, sanguine_angel, steel_falcon |
| H: attestation false, undefined variable in the class closure | 4 | 4 undefined-variables | loremaster, metaforge, mystic_theurge, pathfinder_delver |

Attested `closure_complete`: 56 of 189 class principals.

## 6. Follow-ups (named, not fixed here)

- **F-1, pre-existing: medium_inspiring_call drops a negation.** A `PREABILITY` `[<key>]` exclusion
  that was meant as a negation is lost.
  - Oracle line: `oa_abilities_class.lst:868`, `KEY:Medium ~ Inspiring Call`,
    `DESC:move|PREABILITY:1,CATEGORY=Special Ability,Medium ~ Decisive Strike,[Medium ~ Legendary Marshal]`.
    The next variant is `DESC:swift|PREABILITY:1,...,Medium ~ Legendary Marshal`, and `standard` is
    gated `!PREABILITY` on both. So the oracle means "move when Decisive Strike and not Legendary
    Marshal".
  - The F1c-3 exclusion rule (`prereq.rs` `exclude_from_tag_count`) subtracts an excluded key only
    from a TYPE= tag count. Against a named list it leaves the term unchanged. So the `move` piece
    converts to `Holds(medium_decisive_strike)` and the `[Legendary Marshal]` item is dropped.
  - Effect: a medium holding both abilities prints "As a move swift action". The DESC variants are
    printed rule text, not a sheet total.
  - Pre-existing: on tranche/16 the same item was an unholdable `MissingRule` alternative inside
    `AtLeast 1 of [Decisive Strike, MissingRule]`, which evaluates the same. F1c-3 changed the
    representation, not the behavior.
  - Closes in: the converter. A bracketed item in a `PREABILITY` whose other items are named
    records should become a `Not(Holds(<key>))` conjunct, with one rule and an oracle-pinned test on
    this row.
- **F-2: repeated `granted_by` edges.** 39 repeats: 26 from a class id declared by two books, 13
  from a selector stated twice on one oracle record (§3). They change no value. They close in the
  converter by de-duplicating each target's `granted_by` on write, which is a package-wide
  regeneration.
- **G / H remainder**: 17 prestige classes, closing in the corpus (ingest the named records) or the
  converter (reference resolution, variable declaration repair). See `../reader-remainder.md`.
