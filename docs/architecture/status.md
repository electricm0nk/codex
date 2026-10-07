# Status

> Scope: what is real, working product surface today across the whole repo, and what is stubbed, partially wired, or deferred — superseding the root README's "Current state" section.
> Last verified: **2026-10-07 against `tranche/17` (`b99c3d4b02`, SD-37 closure truth-up)**: Starfinder 1e rows added
> (Posture, Real today, Known gaps); the Pathfinder rules tables are a data package, so the table evidence cells
> below point at `src/rules_core/rules_catalog/`; the Tauri command count is 86. Starfinder figures are the release
> notes' `R-n` figures, re-derived by `python3 docs/release/SD-37-starfinder-1e/artifacts/epic_7/E7.6_check.py --plant`
> (`RESULT PASS 20 figures` on this tree). Pathfinder class-census, roster and catalog figures were not re-derived this
> pass and are the SD-36 Epic F closure values; the PF seed render hashes are unmoved (R-7).
> Maintenance: pre-PR truth-up cycle per [README.md](./README.md) §Maintenance contract — fires before every PR via the architecture-truth-up skill

## Posture

Codex is a real, wired, end-to-end PF1e character-sheet product across the
whole corpus it has ingested, not a single-class proof harness. The engine
reads all **37** books of the converted sheet-rule package
(`ls -d data/sheet_rules/*/ | grep -v '/_' | wc -l` → 37; 49,450 records,
0 refused, `data/sheet_rules/_report.json`). The corpus-ingest pipeline, the
deterministic compute chassis, the boundary contract, and every persistence
store are real, tested, and exercised end to end by `cargo test --locked`
and `npm test`.

**Classes, corpus-wide** (`cargo run --locked -j 8 --bin class_census -- --json <path>`,
2026-09-26, `docs/release/SD-36-consolidation/artifacts/epic-f/stage-f4-f5/census-f5.json`):
the census merges every engine registry into **137** distinct class ids. **Every
one of the 63 non-prestige ids reaches a fully `Computed` receipt at every
level of its own sweep (63 of 63, 0 blocked)** — the classes of the Core
Rulebook, Advanced Player's Guide, Advanced Class Guide, Pathfinder
Unchained, Ultimate Combat, Occult Adventures, Ultimate Intrigue, Ultimate
Magic, Ultimate Psionics and Ultimate Wilderness alike, and the CRB NPC
classes. The **74** prestige ids are Blocked taken alone, by the game rule
(74 of 74, `prestige_class.requires_base_class_levels`), and **68 of 74**
reach `Computed` in their carrier mix; the named exception is 6 of 74 whose
oracle save formula states no PF1 save (`multiclass.save_shape.unrecognized`,
`forward-scope-register.md` FS-15). The multiclass mix panel is **185 of
185** `Computed`. The census holds race fixed to one Human fixture (its own
`input_posture` field), so it proves every level, not every race. Race-creation
breadth is a separate, corpus-wide figure: 39 race records are ingested
across 6 books (CRB, Bestiary 1, Bestiary 2, Bestiary 5, Bestiary 6, ARG —
the 39 is pinned at `race_catalog.rs:548`, the 6-book list at `:170`). A
narrower instrument, `raceCreationCoverage.test.ts`, loads only 3 of those 6
books (CRB/B1/ARG) and pins **30** chassis records on disk (`:383`); of
those 30, **18** have a complete creation chassis (`:576`), and the other 12
are accounted for as outside the converted package (`:582`'s own
accounting: 18 + 12 = 30, the test's own denominator). The 9 races from
Bestiary 2/5/6 sit outside this instrument's scope entirely and were not
measured either way by it — **18-of-30, not 18-of-39**; the only test that sweeps the entire
offered race roster does so for Fighter, levels 1-3 only
(`character_hub.rs:6007-6021`), not for every class at every level.
Multiclass grounds BAB/save stacking, a hit-point total, skill points from
each class's converted ranks, and each class's own feature lines for any mix
of classes that each have a chassis and a Good/Poor save source, with at
least one non-prestige class (SD-36 F3, `pilot_compute/multiclass_fold.rs`).
The desktop Create picker reads the engine's class roster and offers **59**
classes: the 63 Computed non-prestige ids less the 4 Ex-* states
(ex_antipaladin, ex_barbarian, ex_inquisitor, ex_paladin), census-only by
operator ruling (census `roster_offered=59`). Prestige classes are offered at
level-up with their entry requirements printed met/unmet, never blocked. The desktop app ships a real, end-to-end character-creation,
leveling, equipment, spellcasting, DM-toolkit, encounter-builder, and
campaign-manager surface, independently verified by 76 automated UI-smoke
rows: 66 of the first 69 green (`docs/release/SD-36-consolidation/artifacts/ui-smoke/final/RECEIPT.md`,
dated 2026-09-18; the other 3 are native OS file dialogs outside
browser-automation's reach, not feature gaps) and the 7 Epic F4 class-roster
and prestige level-up rows 7 of 7 green, with 4 of 4 regression rows
(`artifacts/ui-smoke/f4/results.json`, 2026-09-26).

**Starfinder 1e** is a second game system on the same engine
(`src/rules_core/game_system.rs`). Its converted package holds **8,582** records from eight books, 0 refused,
30 degraded (`jq -r '[.records,.converted,.refused,.degraded_records]|map(tostring)|join(" ")' data/starfinder-1e/sheet_rules/_report.json`
→ `8582 8582 0 30`, release-notes R-5). Every sheet total comes from the `sf_*` readers over the records the character
holds, checked against **160** SRD hand values for four seeds (Soldier 3, Mystic 5, Technomancer 5, Envoy 3; R-12) and
against a real PCGen oracle run of 14 characters plus one drone: 782 oracle fields compared, 36 differ, 36 explained,
0 unexplained (`sf_oracle_parity`, receipt `docs/release/SD-37-starfinder-1e/artifacts/epic_7/E7.1_cycle_receipt.md`;
the 36 and the 1,413 engine fields the oracle exports no value for are re-derived as R-13). Starfinder has **no
class census**: the Pathfinder census above says nothing about it, and the Starfinder denominators are the roster of
four seeds and ten level-1 class builds, not every class at every level.

**Where these figures live.** The full, corpus-wide census is the "Class/level
compute coverage — corpus-wide" table below, and that table is the SOURCE OF
TRUTH for them. Four other files keep a local headline number in sync by
hand, because each needs the figure inline for its own sentence to read:
README.md's opening posture paragraph ("63 of 63 non-prestige", "137
distinct class ids", "68 of 74") and its create-picker / "Known limitations"
sections ("59 classes", "the other 59"); `desktop-app.md`'s Create-flow
section (the roster offers **59** classes); `rules-data-tables.md`'s
state-dump description (`v06_class_state_dump` sweeps all **31** of its own
four registries — a scope statement about that instrument, not a class
total); and `rules-engine.md`'s dispatch section §3d (137 ids, 63 of 63, 68
of 74, 185 of 185). Update all five together whenever the census changes —
find the current-figure sites with:
`grep -rn '63 of 63 non-prestige\|137 distinct class ids\|68 of 74\|roster offers \*\*59\*\*\|59 of 63\|sweeps all \*\*31\*\*' README.md docs/architecture/*.md`
and prove no retired figure survives with the grep below (it must print
nothing; each alternative is a figure this table once replaced — the Epic F
baseline, the pre-F2a count, the hardcoded picker, the untabled remainder
before F1c, the reader remainder before F3, the prestige-mix count before
F3c5 — and a bracketed digit keeps the command from matching its own line):
`grep -rnE '4[2] of 135|6[1] of 61|computed=6[1]|ids=13[5] |[9] of 27 "untabled"|2[6] .Computed.\)|6[0] of the 93|6[7] of 74|All 3[1] fully-tabled classes|offers 3[1] of|3[1] of the 63|all 3[1] classes|1[1]-class UI-surface gap' README.md docs/architecture/*.md`
(2026-09-26: 0 hits).

**What has changed since the last full pass**: several desktop-facing actions
this doc used to describe as session-local or inert are now real, persisted
mutations — see "Corrections since the last pass" below. Character *creation*
coverage and character *editing* coverage (money, HP, bio, equipment
purchase, feats, traits, skills) are now both wide, not one narrow and one
broad as an earlier pass of this doc stated (that "Fighter-1-3 only" claim
was false — see the capability matrix and evidence below).

### Class/level compute coverage — corpus-wide

**This is the one class-coverage table in this repo, and it is the source
of truth for these figures.** Every other doc states the posture in words
and links here rather than restating the full breakdown — README.md,
desktop-app.md, rules-data-tables.md, and rules-engine.md each also keep
one or more local headline numbers of their own in sync with this table by
hand (see the file/line/figure list and the grep command above this table's
introductory paragraph) — see "Refuted claims" below for the wrong figures
("139 classes", "16 of 74", "none of the 78", "no prestige class has a
chassis") this table replaces everywhere.

**Provenance history:** measured 2026-09-20 by a one-time registry-merge
instrument (a temporary `zz_class_census` test file, built, run once, then deleted).
SD-36 Epic F batch F0 (2026-09-21) replaced that throwaway instrument with
a **permanent** one — `src/rules_core/class_census.rs` +
`src/bin/class_census.rs`, exercised every run by `scripts/verify.sh`'s
`class-census` stage — and F0e (below) replaced the hand-written table
itself with one generated mechanically from that instrument's own `--json`
output, so this table can never again silently drift from what the engine
actually measures.

<!-- class-census:begin -->
**Generated table — do not hand-edit this region.** Produced by `python3 scripts/gen_class_status_table.py` from a fresh `cargo run --locked -j 2 --bin class_census -- --json /tmp/census.json` sweep (`class_census`'s own `source_of_truth`: `codex::rules_core::pilot_compute::build_pilot_headless_receipt`). Re-derive and check for drift with `python3 scripts/gen_class_status_table.py --check` (also run as part of `scripts/verify.sh`'s `class-census` stage). Where this generated table disagrees with an earlier hand-written version of this section, the census wins — see the F0e commit body (`feat(sd36,epic-f0e): status.md class table generated from the census; F0 closed`) for what changed and why.

**Headline numbers** (each states its own denominator and the exact census JSON field it is read from; re-derive with `cargo run --locked -j 2 --bin class_census -- --json /tmp/census.json`, which also prints these same numbers to stdout as `ids=... computed=... blocked=...` / `prestige_swept=... prestige_alone_blocked=... prestige_mix_computed=...` / `mix_panel_swept=... mix_panel_computed=... mix_panel_blocked=...`):

| Quantity | Count | Denominator | Census JSON field |
|---|---|---|---|
| Distinct class ids, corpus-wide, across all engine registries | **137** | — | `ids` |
| Non-prestige ids actually swept (`ids` minus the 74 prestige ids, never swept alone here) | **63** | of 137 | `non_prestige_swept` |
| ...reach `Computed` at every swept level (non-prestige) | **63** | of 63 | `computed` |
| ...reach `Computed` at no level (non-prestige) | **0** | of 63 | `blocked` |
| Prestige ids swept (never measured alone — see the carrier rule below) | **74** | of 137 total ids | `prestige_swept` |
| ...Blocked alone (negative control) | **74** | of 74 | `prestige_alone_blocked` |
| ...`Computed` in their deterministic carrier mix | **68** | of 74 | `prestige_mix_computed` |
| Multiclass mix-panel rows swept (existing negative-control inputs, re-used) | **185** | — | `mix_panel_swept` |
| ...reach `Computed` | **185** | of 185 | `mix_panel_computed` |
| ...stay `Blocked` | **0** | of 185 | `mix_panel_blocked` |

**Per-family breakdown** (a partition of the merged census: every non-prestige id is in exactly one family row above, every prestige id is in the Prestige row; the census JSON reports no per-id chassis field, so no Chassis column is printed here — see this script's own module docstring for why the old hand-counted 117-of-135 / 56-of-74 chassis figures are retired rather than carried forward unmeasured):

| Family | Book(s) | Ids | `Computed` (all swept levels, alone) |
|---|---|---|---|
| CRB | core_rulebook | 11 | 11 |
| APG | advanced_players_guide | 6 | 6 |
| ACG | advanced_class_guide | 10 | 10 |
| Pathfinder Unchained | pathfinder_unchained | 4 | 4 |
| Ultimate Combat | ultimate_combat | 3 | 3 |
| Untabled exotic base classes | advanced_players_guide, occult_adventures, ultimate_intrigue, ultimate_magic, ultimate_psionics, ultimate_wilderness | 20 | 20 |
| CRB NPC / Ex-* classes | core_rulebook | 7 | 7 |
| generic_class_chassis-only (unclaimed by any of the eight canonical sources) | advanced_players_guide | 2 | 2 |
| Prestige | see per-class `books` in the census JSON (11 source books) | 74 | n/a alone (never a legitimate measurement — see headline numbers: 68 of 74 `Computed` in carrier mix) |
| **Total** | | **137** (63 non-prestige + 74 prestige) | **63** of 63 non-prestige ids Computed alone (prestige carrier-mix result kept separate, per headline numbers above — the bin's own `--json` output never folds the two together) |
<!-- class-census:end -->

Row-by-row evidence:
- **CRB/APG/ACG/Unchained (31, all `Computed`)**: `cargo run --locked --bin
  v06_class_state_dump` → `class_count=31, computed_count=31,
  blocked_count=0, max_level=20`.
- **Ultimate Combat (3; all 3 `Computed`)**: `combat.rs`, test
  `all_three_uc_classes_reach_computed_status_at_level_5` (Samurai closed by
  the SD-36 Epic F1 converted-record proficiency reader, 2026-09-22 — its
  `Samurai` weapon set reaches the katana).
- **Untabled exotic + CRB NPC (27 = 20+7; 27 `Computed`)**:
  `untabled_base_class_features.rs`
  (`all_27_untabled_classes_pass_the_chassis_gate_at_every_real_level`;
  `every_untabled_class_outside_the_named_reader_remainder_reaches_computed`).
  Commoner closed by SD-36 Epic F1c-3 (D6: its one-simple-weapon pick is linked
  at ingest to the Simple-tier list its pool offers); Antipaladin and Magus
  closed by F1c-1 (their `TYPE=WeaponProfMartial` grant-by-type converts).
  Classes with no static `CLASS_WEAPON_PROFICIENCIES` row (42 rows,
  unchanged) are answered by
  `class_proficiency_sheet_rules::class_weapon_proficiency_view`; the
  census-wide remainder (3 of the 95 classes it walks, 0 non-prestige + 3
  prestige: diabolist, exalted, rivethun_emissary) is named with a mechanism per class in
  `docs/release/SD-36-consolidation/artifacts/epic-f/reader-remainder.md`,
  pinned by `weapon_tables::every_census_class_has_a_known_proficiency_answer`.
- **Prestige (74 ingested of 131 named; all 74 with a converted chassis row
  dispatched since SD-36 Epic F2a -- 56 before it, the CRB/APG 18 not; 0 of 74
  `Computed` alone, by the game rule; 68 of 74 `Computed` in a carrier mix)**: `prestige_class_entry_gate.rs:1-30`; `python3 -c "import
  json;print(len(json.load(open('tests/fixtures/rules_core/prestige-class-entry-requirements.json'))['entries']))"`
  → 74; `generic_class_chassis.rs`'s `every_conventional_class_in_class_family_books_resolves`
  (78 over its original 14 books, of which 56 are these prestige rows — the
  other 22 are the 19-of-20 untabled-exotic overlap + all 3 UC classes, already
  counted in their own rows above, not double-counted here; 122 since SD-36
  Epic F2a appended `core_rulebook`/`advanced_players_guide`: +18 prestige, +24
  base classes a bespoke arm already owns, +2 APG `Ex-*` classes counted in the
  `GenericOnly` row above — `artifacts/epic-f/stage-f2-f3/f2a-census-before-after.md`); `has_supported_class_chassis` (`class_shared_core.rs`) gained a
  generic class-family arm in SD-36 Epic F2a, but it EXCLUDES `Prestige`-tagged
  records by the game rule (a prestige class cannot be a character's first
  class), so a real chassis still never becomes single-class `Computed` for any
  of the 74; F3's multiclass gate is where it folds in.
- **Multiclass (every class with a chassis, SD-36 F3b)**:
  `is_supported_multiclass_mix` (`class_occult_and_psionic.rs`) admits a mix
  when every member passes `multiclass_fold::multiclass_member` (the isolated
  single-class input passes `has_supported_class_chassis`, or a prestige class
  has a converted row at that level; every save is Good/Poor from the CRB table
  or the converted record) and one member is not prestige. A member that
  cannot join is named (`multiclass.class_unsupported`,
  `multiclass.save_shape.{degraded,unrecognized,unknown}`); a prestige-only mix
  states `prestige_class.requires_base_class_levels`. Proved by
  `tests/sd36_multiclass_any_class.rs` (four hand-worked mixes,
  `artifacts/epic-f/stage-f2-f3/f3b-hand-worked.md`) and
  `tests/sd21_multiclass_fighter_wizard_chassis_computes.rs`.
- **Desktop picker (59 of 63 `Computed` non-prestige classes offered)**:
  the Create picker reads `list_class_creation_roster`
  (`apps/desktop/src-tauri/src/character_hub.rs`), which serves
  `class_census::class_creation_roster()` — census `roster_offered=59`
  (`cargo run --locked -j 8 --bin class_census -- --json <path>`, 2026-09-26,
  `artifacts/epic-f/stage-f4-f5/census-f5.json`). The hardcoded
  31-row list survives only as `CLASS_OPTIONS_FALLBACK`, used when the roster
  command fails, with the failure printed. Prestige classes are offered at
  level-up (`list_level_up_class_options`) with entry requirements printed
  met/unmet, never blocked (SD-36 Epic F §9); an option the multiclass gate refuses is shown disabled with its named blocker (SD-36 F6c: 10 of 133 options on a Fighter 6, 0 refused without the blocker shown).
- **Character level capped at 20** (PF1's own rule, not an engine gap):
  engine refuses level 21+ (`combat.rs:2141-2150`); desktop level-up picker
  filters it out (`apps/desktop/src/characterHub/LevelUpDialog.tsx:65-70`).

**Named exceptions** (the four lists this table's headline numbers stand
for):
- **Blocked only on weapon-proficiency data (0 of 137)** (was 19 of the
  Epic F baseline's 135 before SD-36 F1/F1c; census `ids=137 computed=63
  blocked=0`, 2026-09-26): the converted-record proficiency reader answers
  every non-prestige class from `data/sheet_rules`. Samurai and the 18
  untabled-family classes formerly listed here all reach `Computed`.
- **`Computed` but not in the desktop Create picker (0 of the 59 offered-eligible;
  4 of 63 by ruling)**: measured from the roster — census `roster_reason`
  is `offered` for 59 of 63 non-prestige ids and `ex_state` for the other 4
  (ex_antipaladin, ex_barbarian, ex_inquisitor, ex_paladin), census-only by
  operator ruling (SD-36 Epic F §9); 0 of 63 carry `not_computed` or
  `hit_die_absent`. Pinned by
  `class_census::tests::no_computed_class_is_unoffered_without_a_named_reason`.
- **Prestige in a carrier mix (68 of 74 `Computed`)**: census
  `prestige_mix_computed=68 prestige_mix_unknown=0`, 2026-09-25, SD-36 F3c5
  (`artifacts/epic-f/census-f3c5.json`); re-measured unchanged 2026-09-26
  (`artifacts/epic-f/stage-f4-f5/census-f5.json`).
  - **How the carrier is chosen.** The carrier chooser walks an `AtLeast`
    clause's branches in oracle order and takes the first branch that
    translates to a carrier and a level. It reads `HighestSpellLevel(Any)` as
    "at least 1 of Arcane, Divine", and it reads a `Not` of a spell-kind term
    as a prohibition. Each row's `carrier_reason` names the branch taken.
  - **Dragon Disciple has computed in its sorcerer-5 mix since F3c5.** Its
    carrier comes from a gate branch that names `ClassLevel(sorcerer) >= 1`,
    with the draconic bloodline seeded as the sorcerer's pick. Its one closure
    defect, `Internal|Bite` (`ce_abilities_race.lst:249`), is a natural-attack
    helper row. That row now converts as `Fact::NaturalAttack("Bite")` on
    Dragon Bite (`sheet_rule/natural_attack.rs`), so the closure is attested.
    The proficiency reader answers Known(empty): "Dragon disciples gain no
    proficiency with any weapon or armor" (CRB p.380).
  - **The other 6 are Blocked on `multiclass.save_shape.unrecognized`.** This
    is an oracle `BONUS:SAVE` formula defect, in Evangelist, Exalted, Mammoth
    Rider, Pure Legion Enforcer, Sentinel and Ulfen Guard. It can be closed
    only by a book-cited override (`forward-scope-register.md` FS-15).
  A prestige class alone is never `Computed` (74 of 74 Blocked, the game rule).
- **Multiclass scope**: every family can mix since SD-36 F3b, with two named
  save-source remainders: the 4 Pathfinder Unchained classes (no CRB table row
  and no converted chassis record: `multiclass.save_shape.unknown`) and the 6
  prestige records with an Unrecognized save (`multiclass.save_shape.unrecognized`).

**Refuted claims — corrected here, not restated anywhere else in this
repo**:
- ~~"139 classes hold a chassis"~~ — that summed registry *populations*
  without removing overlap (31+3+27+78 = 139); the real distinct-with-chassis
  count was **117 of 135** at SD-36 F0 (subtracting the 19+3 = 22 ids `generic_class_chassis`'s
  78 shares with the untabled-exotic and Ultimate-Combat rows already
  counted above).
- ~~"16 of the 74 prestige classes have a chassis"~~ — the real count is
  **56 of 74**.
- ~~"none of the [generic_class_chassis] 78 reach Computed"~~ — **11 of the
  78 do** (via an earlier-precedence dispatch arm — `class_occult_and_psionic.rs:770-1063`'s
  dispatch order checks `untabled_base_class_chassis`/`UcClassId` before
  `generic_class_chassis::resolve` ever runs — so `generic_class_chassis::resolve`
  itself is never actually invoked for those 11); all 11 are already counted
  in the Ultimate-Combat and untabled-exotic rows above, not a new count.
- ~~"no prestige class has a chassis"~~ — **56 of 74** ingested prestige
  classes do (all 74 since SD-36 Epic F2a); the reason none reached `Computed`
  then was a missing gate arm, not an absent chassis — F2a added the arm, and
  a prestige class is now Blocked alone by the game rule and `Computed` in a
  carrier mix (68 of 74).

### Capability matrix: content kinds, corpus-wide

The corpus spans **38** tracked Pathfinder 1e books, reconciled across three
different countable things (none of which is itself 38): the `RuleSetId`
enum has **37** variants (`awk '/pub enum RuleSetId/,/^}/' src/rules_core/rules_catalog/mod.rs
| grep -cE "^\s+[A-Z][A-Za-z0-9_]*,\s*$"` → 37, including `Ce` for
`core_essentials`); `docs/work-inventory.json`'s `totals.by_book` tracks
**37** book keys (`python3 -c "import json;print(len(json.load(open('docs/work-inventory.json'))['totals']['by_book']))"`
→ 37) — it contains `beginner_box` (19 units, no dedicated `RuleSetId` of its
own) but does **not** contain `core_essentials` (a real `RuleSetId` variant
and a real `data/corpus/core_essentials/` directory, folded into other
books' race data rather than tracked as its own `by_book` key), so
`by_book`'s 37 = `RuleSetId`'s 37 minus `core_essentials` plus
`beginner_box`; `data/corpus/`'s 39 directories collapse to the same 38
distinct books because `beastiary` (Bestiary 1's chassis half) and
`bestiary` are one book served under one display name
(`apps/desktop/src-tauri/src/monster_catalog.rs:222-231`) — 39 minus that
one duplicate = 38, the correct headline denominator.

| Content kind | Live count | Books served | Evidence |
|---|---|---|---|
| Equipment | 8,119 entries | **29** distinct books | `apps/desktop/src-tauri/src/equipment_catalog.rs:440-475` (11 hand-mapped) ∪ `tests/equipment_gap_tables.rs:23-130` (27 non-zero gap-lane books) |
| Spells | 2,481 entries | **26** distinct books, exhaustive (every `book_entries(BOOK_*)` call is non-zero and the 26 values sum to exactly 2,481, zero remainder) | `apps/desktop/src-tauri/src/spell_catalog.rs:946-974` |
| Feats | 2,227 entries (1,578 hand-authored + 649 corpus-gap rows) | 23 books | `src/rules_core/rules_catalog/feats_all.rs:601,626` (the `1578` and `2227` assertions) |
| Monsters | 330 Bestiary-1 creatures (46 hand-modelled + 284 chassis-only) + per-book counts | **22** books, exhaustive (`book_display_name`/`book_wire_code` each have exactly 22 match arms and panic on an unregistered book) | `apps/desktop/src-tauri/src/monster_catalog.rs:216-262` |
| Races / race traits | `RACE_CATALOG_BOOKS` | 6 books: CRB, B1, B2, B5, B6, ARG | `apps/desktop/src-tauri/src/race_catalog.rs:170` |
| Classes / class features | 137 distinct ids corpus-wide (all engine registries merged); 63 of 137 (63 of 63 non-prestige) reach `Computed` alone; 68 of 74 prestige ids reach `Computed` in their carrier mix (74 of 74 Blocked alone, the game rule) | 18 source books carry class content | see "Class/level compute coverage — corpus-wide" above for the full breakdown, set algebra, and re-derive commands — not restated here |

**Exact book rosters for the three rows above that used to read "11+", "at
least 8", and "~20"** (each denominator is a count of book codes with a
non-zero contribution, not an approximation):
- **Equipment (29)**: 11 hand-mapped book codes (`equipment_catalog.rs:440-475`:
  CRB, APG, ACG, B1, ARG, PU, UI, UE, UM, UPSI, UC) ∪ 27 non-zero gap-lane
  books (`tests/equipment_gap_tables.rs:23-130`'s `EXPECTED_PER_BOOK`, which
  lists 28 books and asserts their rows total 1,973 — 27 of the 28 are
  non-zero, UM is 0 there because UM is already hand-mapped: ACG, AG, APG,
  ARG, B1, B2, B3, B4, BB, BOTD2, CRB, HA, ISC, ISG, ISI, ISM, ISR, ISTEM,
  ISWG, MC, MYTHIC, OA, UC, UE, UI, UPSI, UW). Union = 29 (PU and UM are
  hand-mapped-only, not in the gap lane; every other hand-mapped book also
  appears in the gap lane).
- **Spells (26)**: `spell_catalog.rs:946-974` — CRB 664, APG 297, ACG 144,
  ARG 93, UI 101, UM 269, OA 144, UC 146, ISG 96, UW 61, AG 49, ISF 3, ISM
  39, ISTEM 21, HA 70, B1 111, B4 55, BOTD1 4, BOTD2 8, ISI 25, ISR 29, ISWG
  14, MC 24, MYTHIC 10, UE 1, UMWP 3 — 26 books, sum = 2,481 exactly.
- **Monsters (22, exhaustive)**: Bestiary 1 (wire-coded as "beastiary"),
  Bestiary 2-6, Bonus Bestiary, Monster Codex, Book of the Damned Volume
  1/2, Inner Sea World Guide/Bestiary/Gods, Ultimate Psionics/Wilderness/
  Intrigue/Magic, Pathfinder Unchained, Advanced Race Guide, Mythic
  Adventures, Occult Adventures, **Horror Adventures** (the book the
  previous "~20" list dropped).

Companion catalog and intelligent-item catalog book counts were not
individually re-verified this pass (both exist and are real — see "Real
today" below — but their per-book breadth was not re-derived against the
same test-pinned standard as the rows above).

## Corrections since the last pass

- **The DM Toolkit is real**, not a `StubScreen.tsx` placeholder — a real
  encounter builder and DM-console export, backed by the real
  `Encounter::new`/`party_challenge_rating` compute this doc already listed
  as real. See [desktop-app.md](./desktop-app.md).
- **Skill-allocation and level-up dialog acceptance are now real, persisted
  mutations** (`set_skill_allocations`, `level_up_character` via
  `preview_level_up`), not in-memory-only `useState`/no-op closures.
- **The character sheet's `☰ Menu` has no bare no-ops left** — `Open`,
  `Recompute`, `Clone`, `Export`, `Print` are all wired to real handlers.
- **`StubScreen.tsx` is unreferenced dead code**, not a live placeholder for
  any current screen.
- **The Tauri command count is 86** (77 at the prior pass; SD-37 added the eight Starfinder commands listed in
  [desktop-app.md](./desktop-app.md)), re-derived directly from
  `generate_handler![...]` (see [desktop-app.md](./desktop-app.md)); a saved
  character's on-disk bundle can now hold up to six files, not two (four new
  sidecar files: bio/money/HP/portrait — see [persistence.md](./persistence.md)).
- **The PCGen converter now lives behind a real crate boundary.** SD-36 Epic
  A moved the code formerly at *src/pcgen_import/* and *src/oracle_validation/* (neither path exists
  at that location any more) into
  `crates/codex-ingest`, a `[dev-dependencies]`-only crate from
  `apps/desktop/src-tauri`'s own `Cargo.toml` (verified: `codex-ingest` does
  not appear under that manifest's `[dependencies]` section) — the "PCGen
  wall" is now a real crate/dependency-graph boundary, not only a residue
  gate over string patterns. See [corpus-ingest.md](./corpus-ingest.md).
- **`src/rules_core/pilot_compute/mod.rs` is split into per-class submodule
  files** (SD-36 Epic C1) rather than one 88,000-line file; the whole
  directory is ~100,900 lines across many files today (`find … -name '*.rs' |
  xargs wc -l`), the largest single file ~6,160 lines. Call sites are
  unaffected (`pilot_compute::` paths still resolve the same way).

## Corpus coverage

**The corpus-completion instrument this section used to report against —
`v06_work_inventory`/`docs/work-inventory.json`'s per-unit classifier,
*support_state_matrix.rs*, and *apps/desktop/src-tauri/src/reach_gate.rs* (all three deleted) —
was deleted outright by SD-36 Epic B**, not refactored or superseded by a
successor instrument. `docs/work-inventory.json` itself is now frozen
(unchanged content, recorded once at `docs/work-inventory.FROZEN.md`) and is
no longer regenerated by anything.

**The one live, current-state corpus-completion figure** is the frozen
public status snapshot:

```
$ python3 -c "import json;print(json.load(open('site/status-data.json'))['overall'])"
{'done': 49450, 'partial': 0, 'not_started': 0, 'denominator': 49450, 'pct': 100.0, ...}
```

`scripts/site/check_frozen_status.py --check` gates this file never drifting; per-book detail lives
under `site/status-data/`, and this is the same JSON the public `campaign-codex.org` site (deployed by
[release-pipeline.md](./release-pipeline.md)'s `deploy-site.yml`) renders. **This is a statement about
the corpus reaching a rendered sheet line under the sheet rule** (`docs/release/SD-35-corpus-sheet-completion/decisions.md
§1`: a record is done when it renders a line a player could write — one final number, dice in final
form, or the rule's own words; "the engine cannot model X" is a number to report, not an exemption).
**It is not the same measurement as the Posture section's class/level `Computed`-receipt matrix
above** — this figure counts corpus *content units* (one per race/spell/feat/equipment row/etc.)
against the sheet rule; the Posture section counts *class/level combinations* reaching a full
compute receipt. Both are now wide (100% of 49,450 units; 63 of 137 distinct
class ids — every one of the 63 non-prestige ids, all swept and
`Computed` at every level 1-20, not a single tested level — see "Class/level
compute coverage — corpus-wide" above for the full breakdown), but they
measure different things and must not be conflated.

The detailed wave-by-wave history of how this number was reached (SD-29 through SD-33, dozens of
integration cycles) is retired along with the instrument that produced it; it is not reproduced here.
Anyone who needs it can read the superseded commits under `docs/release/SD-29-*` through
`docs/release/SD-33-*` and `docs/retro/`.

## Real today

| Area | What works | Where |
|---|---|---|
| Corpus-ingest pipeline | `.pcc`/`.lst` parsing through canonical `SourcePackageContent` projection, now behind the `crates/codex-ingest` crate boundary (dev-dependency only from the desktop shell) | [corpus-ingest.md](./corpus-ingest.md) |
| Pilot compute + boundary contract | `compute_pilot_base_chassis` → `compute_pilot_with_corpus` → `to_pilot_receipt` → `printed_sheet_cell_map`, fail-honest throughout; `pilot_compute/` is now split into per-class submodule files | [rules-engine.md](./rules-engine.md) |
| Per-domain engines | Spellbook, skill allocation, feat prerequisites, equipment effects, damage total, level-up | [rules-engine.md](./rules-engine.md) |
| Pathfinder rules-table package | `data/rules_tables/` (281 files, 24,583 rows) loaded through `src/rules_core/rules_catalog/`; no compiled copy exists; 429 golden digests pin every table and lookup; `rules_tables_package --check` is the normaliser gate (see [rules-data-tables.md](./rules-data-tables.md)) | [rules-data-tables.md](./rules-data-tables.md) |
| Rule-table catalogs | **37** `RuleSetId` variants spanning the 38-book tracked corpus (see "Capability matrix: content kinds, corpus-wide" above, this same file, for the re-derived denominator and reconciliation) — see [rules-data-tables.md](./rules-data-tables.md) for per-book ceiling detail | [rules-data-tables.md](./rules-data-tables.md) |
| Character Hub | Create, load, clone, portrait upload/load/delete, JSON export, recompute, plus (new since the last full pass) equipment purchase/attach, feat/trait selection, skill allocation, bio/money/HP tracking — all real engine compute + real persistence | [desktop-app.md](./desktop-app.md) |
| DM Toolkit | Real encounter builder + DM console export (`rate_encounter`, `export_dm_console`), consuming the real `Encounter`/party-CR compute below | [desktop-app.md](./desktop-app.md) |
| Rule-system adapter seam (hub-of-hubs) | `RuleSystemAdapter` trait is the object-safe seam three commands (`append_to_character`/`recompute_character`/`re_save_character`) dispatch through on a `rule_system_id`: `"pf1"` resolves to the real `Pf1Adapter`, `"starfinder-1e"` to the real `StarfinderAdapter`; any other id resolves to the governed `StubAdapter` (registered exception 0002, `docs/governance/wired-integration-stubs-registry.md`). `list_saved_characters` dispatches on the selected system and `load_saved_character` on the envelope's `game_system`. The other Pathfinder character-mutation commands call PF1 free functions directly and do not go through this seam | [desktop-app.md](./desktop-app.md) §"Rule-system adapter seam" |
| Starfinder 1e engine | Generic chassis over the converted package, no per-class/race/skill table: BAB, saves, HP/Stamina/Resolve (`sf_chassis`), EAC/KAC and initiative (`sf_defense`), skills (`sf_skills`), spells per day/known/DCs for levels 0-6 (`sf_spells`), credits and bulk (`sf_loadout`), ability scores with the 5/10/15/20 increases (`sf_abilities`), melee/ranged attack and per-weapon attack/damage (`sf_attack`); each total is an `sf.*` explanation row listing every term | [rules-engine.md](./rules-engine.md) §"Starfinder 1e: the generic chassis" |
| Starfinder 1e desktop surface | Under the Starfinder chip: creation flow (race, theme, class, point buy), sheet (`StarfinderCharacterSheet`), level-up dialog, feats/spells/gear dialog, six catalogs; every number on the sheet is an engine explanation row, enforced by `starfinderSheet.test.ts`; 14 of the 103 ui-smoke rows are Starfinder rows (`python3 -c "import json;d=json.load(open('apps/desktop/scripts/ui-smoke/spec.json'));print(len(d['rows']),sum(1 for r in d['rows'] if 'starfinder' in r['id']))"` → `103 14`, R-14) | [desktop-app.md](./desktop-app.md) |
| Starfinder 1e printed sheet | `sf_sheet_print` prints race, theme, class-feature, feat, spell, carried-item, upgrade, fusion and augmentation lines as `sheet_rule::render_sheet` prints them over the held records, with named refusals for an unprintable selection; a Mechanic's drone prints as its own block of terms (`sf_drone_print`) | [desktop-app.md](./desktop-app.md) |
| Corpus-ingest diagnostic | `corpus_ingest_diagnostic` Tauri command reports real ingested-record-kind counts per book, counted from the package-backed rules catalog tables (`ClassId::ALL`, `SPELL_LIST`, `equipment_tables()`, ...) | [desktop-app.md](./desktop-app.md) |
| PCGen runner scaffolding | `scripts/pcgen-run-character.sh` drives the real headless PCGen Gradle batch-export; wrapped by `oracle_validation::pcgen_runner::run_pcgen_character` (now under `crates/codex-ingest`) | [testing.md](./testing.md) |
| Campaign manager (local) | Create/edit/list campaigns and their assets, backed by `CampaignStore` on disk; nonce-based conflict detection with local-wins + preserved-conflict-copy resolution. `localStorage` remains the actual frontend source of truth; `write_campaign_drive_artifacts` is a one-way write-through mirror | [persistence.md](./persistence.md) |
| Update eligibility / restore / verify | `is_install_eligible`, `perform_restore_previous`, `verify_relaunch_artifact` — all real, tested Tauri commands | [update-and-feedback.md](./update-and-feedback.md) |
| Feedback composers + browser handoff | Bug/enhancement draft composition, evidence capture/redaction, the governed GitHub-issue browser handoff, and a controlled-defect SHA-256 mismatch test harness | [update-and-feedback.md](./update-and-feedback.md) |
| Release pipeline | Multi-platform publish, dual manifest validation, channel-index push, branch-promotion gates, plus a separate `deploy-site.yml` lane for the public status site | [release-pipeline.md](./release-pipeline.md) |
| IPC bridge liveness | `load_backend_health` returns the real crate version and compile-time git SHA | [desktop-app.md](./desktop-app.md) |
| Homebrew authoring workbench | The Guard Stance proof package's validate/persist/preview round trip, read-only bridged to the desktop tester workbench | [homebrew-and-oracle.md](./homebrew-and-oracle.md) |
| Encounter difficulty / party CR compute | `Encounter::new` and `party_challenge_rating` are real, grounded compute, now reachable through the real DM Toolkit UI (see above — no longer blocked behind a stub screen) | [rules-engine.md](./rules-engine.md) |
| Multiclass base-chassis dispatch + fold | `compute_multiclass_base_chassis` grounds BAB/save stacking (exact fractions, floored once) for any mix of classes with a chassis and a Good/Poor save source (SD-36 F3b); `multiclass_fold::explain_multiclass_fold` adds the hit-point total, Unknown skill points (named), each class's own lines re-scoped `multiclass.<class>.*`, and prestige entry requirements printed, never enforced — four hand-worked mixes in `tests/sd36_multiclass_any_class.rs` | [rules-engine.md](./rules-engine.md) §"Multiclass base-chassis dispatch" |
| Repo-resident JSON corpus cache | `data/corpus/<book>/**/*.json` — see [rules-data-tables.md](./rules-data-tables.md) for the current book/file-count figures (not re-derived here); a sanitized runtime mirror of the same data ships in the desktop installer as `resources/corpus_bundle/` (64 MiB, 14,029 JSON files) — see [desktop-app.md](./desktop-app.md) |

## Known gaps and stubs, by area

### Desktop app: character sheet and update actions

| Item | Status | Where (re-verified) |
|---|---|---|
| `perform_install` | Always returns `Err("...not wired: downloading the AppImage artifact requires an HTTP client...")`; its TS caller `installAction.ts::performInstall` has zero production call sites in `Ui.tsx`'s composed panels. Doubly inert. | `apps/desktop/src-tauri/src/update/transaction.rs`; [update-and-feedback.md](./update-and-feedback.md) |
| `perform_retention_sweep` | Real, tested body (`perform_retention_sweep_impl`), but still not in `main.rs`'s `generate_handler!` list — unreachable from the frontend. | `apps/desktop/src-tauri/src/update/transaction.rs`; `apps/desktop/src-tauri/src/main.rs` |
| `drive_list_campaigns` / `drive_load_campaign` / `drive_save_campaign` / `drive_delete_campaign` | Registered and unit-tested, but no frontend file invokes any of them — `campaignModel.ts` uses `localStorage` as the real source of truth; only `write_campaign_drive_artifacts` (one-way mirror) is called. | `apps/desktop/src-tauri/src/campaign_drive.rs`; `apps/desktop/src/campaign/campaignModel.ts` |
| `append_to_character` / `re_save_character` | Registered and unit-tested, but no `boundary/*.ts` wrapper and zero `invoke()` call sites exist anywhere in `apps/desktop/src`. Their sibling `recompute_character` **is** wired to a real UI affordance (`CharacterSheet.tsx`'s `☰ Menu` → Recompute). | `apps/desktop/src-tauri/src/characterHub/appendToCharacter.rs`, `.../reSaveCharacter.rs` |
| Character-sheet bio fields | Persisted, not session-local — corrected since the last pass. `update_character_bio`/`load_character_bio` round-trip through `bio.json`. | [persistence.md](./persistence.md) |
| DM Toolkit UI | Real, not a stub — corrected since the last pass. See "Corrections" above. | [desktop-app.md](./desktop-app.md) |
| Skill allocation / level-up dialog acceptance | Real, persisted mutations — corrected since the last pass. | [desktop-app.md](./desktop-app.md) |
| Hit points on the 59-class roster | **Closed by SD-36 F6b** (`docs/release/SD-36-consolidation/artifacts/epic-f/stage-f6/f6b-receipt.md`): **0 of 59** offered classes print HP `Unknown` (was 5 of 59: monk and four Unchained). One rule, `pilot_compute::hit_die_source`: the source that computes a class's hit points (bespoke class module first, then the converted record, a class-selection class reading its base class line) is the die the roster serves and the sheet prints; Monk prints d8 (FS-23 sheet side). Denominator/command: `classRoster.test.ts` (`0 of 59 roster classes: HP Unknown`); census `roster_offered=59`, 0 `hit_die_absent`. | `src/rules_core/pilot_compute/hit_die_source.rs`; `apps/desktop/src/characterHub/classRoster.ts` |
| Class skills on the 59-class roster | **Hand table deleted by SD-36 F6a** (`docs/release/SD-36-consolidation/artifacts/epic-f/stage-f6/f6a-receipt.md`): the Skills panel reads class skills from the engine (`list_class_facts`, `class_facts_sheet_rules.rs`) — **50 of 59** roster classes answered at every level (was 12 of 59 from a 12-row hand table). Remainder **9 of 59**, one mechanism, printed `Class skills Unknown for <Class> (<reason>)` with no +3: the ACG classes arcanist, brawler, hunter, investigator, shaman, skald, slayer, swashbuckler, warpriest, whose `Class\|<Class>` grant is an unresolved reference in the converted package — a converter step. Feat skill bonuses fold from the record (F6b: 40 of 43 skill-bonus feats, Alertness included). Command: `cargo test --locked -j 8 --lib class_facts_sheet_rules -- --test-threads=8` (`every_roster_class_answer_is_counted`). Re-confirmed unchanged by SD-36 F7c (`class_skill` scan, `f7c-receipt.md` §(a)); F7 also removed one non-skill id from the list served (Samurai's `samurai_mount`), 0 of 148 converted class-skill lists now name a non-skill id. | `src/rules_core/pilot_compute/class_facts_sheet_rules.rs`; `apps/desktop/src/characterHub/skillsModel.ts` |
| Multiclass mix: 4 Unchained base classes | **Named by SD-36 F6c, unchanged by F7** (`docs/release/SD-36-consolidation/forward-scope-register.md` FS-27): Unchained Barbarian, Monk, Rogue and Summoner can be taken alone but refuse `multiclass.save_shape.unknown` when added into a mix — the mix gate's save-shape reader checks only the CRB class table and a converted chassis record, not the bespoke `pathfinder_unchained::class_chassis` module these four use. Level Up shows the blocker; Accept is disabled. | `src/rules_core/pilot_compute/multiclass_fold.rs`; `src/rules_core/rules_catalog/pathfinder_unchained/class_chassis.rs` |
| Reference-library catalog: `Chosen` field summaries | **Named by SD-36 F6c, unchanged by F7** (`forward-scope-register.md` FS-28): 33 catalog field summaries (`sheet_rule_catalog::fact_words`/`weapon_words`) print a `Chosen` grant's raw record id, not its label — these describers take no `SheetRulePackage` to resolve one against, unlike the sheet/Level-Up printer fixed in F6c/F7c. Not a sheet or Level Up line. | `src/rules_core/sheet_rule_catalog.rs` |
| Caster level and Martial weapon tier | **Closed by SD-36 F6a** (`docs/release/SD-36-consolidation/artifacts/epic-f/stage-f6/f6a-receipt.md`): hand-kept desktop class tables **0** (was 3: `MARTIAL_WEAPON_CLASSES`, `CASTER_CLASSES`, `CLASS_SKILLS`, all deleted). Weapon proficiency and caster level come from the engine (`list_class_facts`): **59 of 59** roster classes answered at every level for each (37 cast at some level, 22 cast nothing). The only per-class desktop rows left are `CLASS_OPTIONS_FALLBACK` (31 rows), read only when the roster command fails, with the failure printed. | `src/rules_core/pilot_compute/class_facts_sheet_rules.rs`; `apps/desktop/src/characterHub/classFactsModel.ts` |
| Class labels on list screens | `LoadCharacterScreen` and the campaign sheets call `formatHeldClasses` without subscribing to the class catalog: they show the id-derived label until the catalog first loads (on the first Create form or sheet mount), then the served label (SD-36 F4c receipt §6). | `apps/desktop/src/characterHub/` |
| Campaign conflict merge | Conflict detection is real and tested (nonce-based); resolution is local-wins with both copies preserved under `conflicts/<timestamp>/` — there is no merge UI. | [persistence.md](./persistence.md) §"Conflict detection" |

### Starfinder 1e

| Item | Status | Where |
|---|---|---|
| Drone totals | A Mechanic's drone prints its **terms** only. The oracle's drone totals (EAC/KAC, initiative, saves, attacks, two skills) have no engine total, and PCGen's drone skill totals look wrong against the SRD. Whether drone totals are sheet totals is an operator ruling, not decided. Drone modifications past level 1 are not covered | `apps/desktop/src-tauri/src/sf_drone_print.rs`; `docs/release/SD-37-starfinder-1e/release-notes.md` §Known Issues |
| Multiclass | Taking a level in a second class is refused by the engine's `sf_chassis.multiclass_key_ability` (`REFUSED_MULTICLASS_KEY_ABILITY`); the E6.5 receipt records it as not covered | `src/rules_core/pilot_compute/sf_chassis.rs`; `apps/desktop/src-tauri/src/sf_level_up.rs` |
| Attack-bonus reads in printed prose | `sf_defense::held` leaves the base-attack fact at 0 for every reader but `sf_attack`, so a printed line computed from BAB reads 0 (the Soldier's Deadly Aim line prints "an additional 0"; E7.1 receipt). The sheet's attack totals use `sf_attack` and are unaffected | `src/rules_core/pilot_compute/sf_defense.rs`, `sf_attack.rs` |
| Feat counts | Printed, not enforced | `apps/desktop/src-tauri/src/sf_choices.rs` |
| Degraded records | 30 of 8,582 converted records carry a degraded token, each named by token type in `_report.json` | `data/starfinder-1e/sheet_rules/_report.json` |
| Starship | Planned capability deferral, not built; its revisit condition was checked at closure (`decisions.md §17`) | `docs/release/SD-37-starfinder-1e/forward-scope-register.md` |
| Licence sign-off | Every Starfinder licence-matrix row has `operator_sign_off` false (11 rows, R-16); only the operator signs | `docs/governance/license-matrix.md` |
| Render-hash harness | None for Starfinder; the four seeds are checked by tests (R-12), not by hash. The two Pathfinder seeds have the hash pair (R-7) | `apps/desktop/src-tauri/src/pf_seed_render_hash.rs` |

### Core engine: compute coverage and proof surfaces

| Item | Status | Where (re-verified) |
|---|---|---|
| Class/level compute coverage | Wide, not narrow, corpus-wide — 63 of 137 distinct class ids (63 of 63 non-prestige) reach `Computed`; 68 of 74 prestige ids `Computed` in a carrier mix; full breakdown, per-family table, and the four named-exception lists (blocked-only-on-weapon-proficiency, computed-but-not-in-picker, prestige with/without chassis, multiclass scope) live in "Class/level compute coverage — corpus-wide" above; not restated here. | see table above |
| Oracle-parity comparator | The in-crate harness (`oracle_validation::comparator::compare`, now under `crates/codex-ingest`) exists and is tested — normalizes PCGen output, reports per-dimension matches/mismatches, renders a real `PASS`/`FAIL` report. A *passing* end-to-end parity claim is a separate, further-out question this doc does not re-verify this pass. | `crates/codex-ingest/src/oracle_validation/` |
| Bestiary 1 monster parser | `monster_stat_block.rs`'s row parser is still unwired — no `ParsedLstRecord`/`SourceContentPayload` variant references it outside its own test file (0 hits, re-confirmed this pass). Bestiary 1 table content is hand-transcribed, not parsed through the canonical-IR path. | grep across `src/rules_core/source_content.rs` / ingest converter |
| Failure-owner classifier | `pilot_failure.rs`'s `primary_owner` still only ever returns `OracleGap` (on `Computed`) or `EngineFlaw` (on `Blocked`) — the other two `PrimaryOwner` variants remain unreachable from the current receipt surface (re-confirmed this pass). | `src/rules_core/pilot_failure.rs` |
| Spellbook magnitude — a disconnected twin | `contract::build_pilot_receipt` wires `spellbook::compute_spellbook_coverage` into `PilotReceipt.spellbook`, but nothing in the shipped desktop app reaches it (`grep -rn build_pilot_receipt apps/desktop/src-tauri/src` still returns 0 hits, re-confirmed this pass). The app instead gates on `pf1_adapter::resolve_unified_pilot_snapshot`. | `src/rules_core/contract.rs`; `apps/desktop/src-tauri/src/pf1_adapter.rs` |
| Per-item corpus equipment stats | `pilot_compute_corpus.rs`'s `DerivedEquipmentStats` is still constructed via `default()` at its call sites — real per-item stats are computed separately by `equipment_effects.rs` (re-confirmed this pass). | `src/rules_core/pilot_compute_corpus.rs` |
| Homebrew content breadth | Guard Stance is still the only authored package content the authoring format ships; no second package constructor exists. | `src/homebrew_authoring/mod.rs` |
| Future-state books (`book_stub`) | **33** out-of-scope Paizo books (`data/stubs/*.json`, up from 21 at the last pass — `find data/stubs -name '*.json' | wc -l`) are registered as honest future-state placeholders — each carries only identity/registration metadata and no rule data. | `data/stubs/*.json`; `docs/governance/wired-integration-stubs-registry.md` |

### Release pipeline: CI coverage gaps

| Item | Status | Where (re-verified) |
|---|---|---|
| No concurrency guard on publish | `publish-tester-release.yml` still declares no `concurrency:` block; two rapid pushes to `develop` can run two concurrent `finalize` jobs, each pushing to the shared `update-index` branch. | `.github/workflows/publish-tester-release.yml` |
| No tranche/16-scoped CI workflow | `tranche-3-ci.yml` remains the only tranche-specific workflow, scoped to `tranche/3` only. No `tranche-16-ci.yml` or equivalent exists. | `.github/workflows/` (only `tranche-3-ci.yml` matches `tranche*`) |
| `check-release-manifest.yml`'s stale path globs | Two of six `paths:` globs (`apps/desktop/src/sd16/**`, `apps/desktop/src/sd17/**`) resolve to nothing in this checkout; the other four were fixed since the last pass. | [release-pipeline.md](./release-pipeline.md) §"Promotion gates chain" item 5 |

This doc is the first one every SD closure re-checks — a stub graduating to
real, tested behavior is the most common architectural-doc change, and it
must be reflected here before it is reflected anywhere else. See
[README.md](./README.md) §Maintenance contract for the update procedure.
