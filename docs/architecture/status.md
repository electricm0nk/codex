# Status

> Scope: what is real, working product surface today across the whole repo, and what is stubbed, partially wired, or deferred — superseding the root README's "Current state" section.
> Last verified: **2026-09-20 against `tranche/16`, HEAD `424e93e93c`** (SD-36 consolidation docs-truth pass: corrected the Posture section's stale Fighter-1-3 ceiling claim against a fresh `v06_class_state_dump` run and the class/prestige/multiclass compute-path code). Earlier pass on the same date (HEAD `b22ea9e113`) trimmed the file substantially (from ~1,400 lines to the shape below): the wave-by-wave corpus-coverage narrative (SD-29 through SD-31 wave 27, plus the SD-33 `unknown`-reaches-zero account) documented the history of an instrument — *src/bin/v06_work_inventory.rs*, *support_state_matrix.rs*, *reach_gate.rs* (italicized because SD-36 Epic B deleted all three; none of these paths exists in this checkout any more) — that SD-36 Epic B **deleted outright** (55,827 lines removed: `git show --stat` on the Epic B commit, or `docs/release/SD-36-consolidation/epic-breakdown.md`'s own per-file line counts). Per this doc set's own rule ("obsolete statements are REMOVED, not annotated as deprecated") and the maintenance contract's instruction that release-bundle narrative belongs under `docs/release/`/`docs/retro/`, not here, that history is removed from this file rather than kept as a growing appendix — it remains readable at `docs/retro/` and in the superseded commits' own diffs for anyone who needs the historical account. The one corpus-completion figure that is still current-state truth is the frozen public status site — see "Corpus coverage" below.
> Maintenance: pre-PR truth-up cycle per [README.md](./README.md) §Maintenance contract — fires before every PR via the architecture-truth-up skill

## Posture

Codex is a real, wired, end-to-end PF1e character-sheet product across the
whole 38-book corpus it has ingested, not a single-class proof harness. The
corpus-ingest pipeline, the deterministic compute chassis, the boundary
contract, and every persistence store are real, tested, and exercised end to
end by `cargo test --locked` and `npm test`. **All 31 fully-tabled classes —
every Core Rulebook, Advanced Player's Guide, Advanced Class Guide, and
Pathfinder Unchained class — reach a fully `Computed` character-sheet
receipt at every level 1-20, with zero blocked levels**:
`cargo run --locked --bin v06_class_state_dump`, run 2026-09-20,
reports `class_count=31, computed_count=31, blocked_count=0, max_level=20`
across barbarian, bard, cleric, druid, fighter, monk, paladin, ranger,
rogue, sorcerer, wizard (CRB); alchemist, cavalier, inquisitor, oracle,
summoner, witch (APG); arcanist, bloodrager, brawler, hunter, investigator,
shaman, skald, slayer, swashbuckler, warpriest (ACG); and the 4 Unchained
classes. **That dump sweeps class and level only — race is held fixed to a
single Human fixture** (its own `input_posture` field;
`src/bin/v06_class_state_dump.rs:307-330` never varies race), so it proves
the level-1-20 range for that one race, not a race sweep. Race-creation
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
(`character_hub.rs:6007-6021`), not for every class at every level. Two of
Ultimate Combat's three classes (Gunslinger, Ninja) reach `Computed` too,
and so do 9 of 27 "untabled" exotic/NPC base classes (Kineticist, Medium,
Mesmerist, Occultist, Vigilante, Psychic, Spiritualist, Psion, Shifter).
Multiclass grounds BAB/save stacking, a hit-point total and each class's own
feature lines for any mix of classes that each have a chassis and a Good/Poor
save source, with at least one non-prestige class (SD-36 F3b,
`pilot_compute/multiclass_fold.rs`); class skill points print Unknown (no
converted class record states skill ranks per level). The desktop app ships a real, end-to-end character-creation,
leveling, equipment, spellcasting, DM-toolkit, encounter-builder, and
campaign-manager surface, independently verified at 66 of 69 automated UI
flows green (`docs/release/SD-36-consolidation/artifacts/ui-smoke/final/RECEIPT.md`,
dated 2026-09-18); the other 3 rows are native OS file dialogs outside
browser-automation's reach, not feature gaps.

**This is not the whole corpus-wide class picture** — it is the 31
fully-tabled classes plus the 11 more this doc used to omit. The full,
corpus-wide census (135 distinct class ids, all engine registries merged,
42 of them reaching `Computed`) is the "Class/level compute coverage —
corpus-wide" table below, and that table is the SOURCE OF TRUTH for these
figures — but it is not the *only* place a class head-count appears. Four
other files keep a local headline number in sync by hand rather than only
linking here, because each needs the figure inline for its own sentence to
read (a README opening claim, a desktop create-picker fact, an instrument's
own scope, a dispatch-history figure): README.md's opening posture
paragraph (line ~11: "31 fully-tabled classes", "61 of 61 non-prestige
class ids") and its create-picker/"Known limitations" section (lines ~186, 205,
209: "31 fully-tabled classes", "18 of 27"); `desktop-app.md`'s Create-flow
section (line ~412: `CLASS_OPTIONS` offers **31** classes);
`rules-data-tables.md`'s state-dump description (line ~668: `v06_class_state_dump`
sweeps all **31** classes); and `rules-engine.md`'s dispatch-history section
(lines ~494-498: "27 \"untabled\" base classes", "78 conventional PC
classes"). These four are the ones to update together with this table
whenever the census changes — find all of them with:
`grep -rn '31 fully-tabled classes\|61 of 61 non-prestige\|61 of 135\|135 distinct class ids\|78 conventional\|27 "untabled" base classes\|CLASS_OPTIONS.*31\|sweeps all \*\*31\*\*' README.md docs/architecture/*.md`.

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
instrument (`tests/zz_class_census.rs`, built, run once, then deleted).
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
| ...`Computed` in their deterministic carrier mix | **67** | of 74 | `prestige_mix_computed` |
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
| Prestige | see per-class `books` in the census JSON (11 source books) | 74 | n/a alone (never a legitimate measurement — see headline numbers: 67 of 74 `Computed` in carrier mix) |
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
- **Untabled exotic + CRB NPC (27 = 20+7; 26 `Computed`)**:
  `untabled_base_class_features.rs`
  (`all_27_untabled_classes_pass_the_chassis_gate_at_every_real_level`;
  `every_untabled_class_outside_the_named_reader_remainder_reaches_computed`;
  `a_class_whose_converted_closure_is_incomplete_still_reports_proficiency_unknown`
  — Commoner, the 1 still blocked on weapon proficiency: its one-simple-weapon
  pick goes into a pool the package links no member to, so every weapon
  outside its automatic proficiencies is Unknown, 2026-09-22 reader batch
  blocker 2. Antipaladin and Magus closed 2026-09-22 by SD-36 Epic F1c-1: their
  `TYPE=WeaponProfMartial` grant-by-type now converts). Classes with no
  static `CLASS_WEAPON_PROFICIENCIES` row (42 rows, unchanged) are answered by
  `class_proficiency_sheet_rules::class_weapon_proficiency_view`; the
  census-wide remainder (60 of the 93 classes it walks, 1 non-prestige + 59
  prestige) is named with a mechanism per class in
  `docs/release/SD-36-consolidation/artifacts/epic-f/reader-remainder.md`,
  pinned by `weapon_tables::every_census_class_has_a_known_proficiency_answer`.
- **Prestige (74 ingested of 131 named; all 74 with a converted chassis row
  dispatched since SD-36 Epic F2a -- 56 before it, the CRB/APG 18 not; 0
  `Computed`)**: `prestige_class_entry_gate.rs:1-30`; `python3 -c "import
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
- **Desktop picker (31 of 42 `Computed` classes offered)**:
  `apps/desktop/src/characterHub/characterHubModel.ts:409` `CLASS_OPTIONS`,
  31 entries — all 31 fully-tabled classes, none of the 11
  additionally-`Computed` ones.
- **Character level capped at 20** (PF1's own rule, not an engine gap):
  engine refuses level 21+ (`combat.rs:2141-2150`); desktop level-up picker
  filters it out (`apps/desktop/src/characterHub/LevelUpDialog.tsx:65-70`).

**Named exceptions** (the four lists this table's headline numbers stand
for):
- **Blocked only on weapon-proficiency data (0 of 135)** (was 19 of 135
  before SD-36 F1/F1c): the converted-record proficiency reader now answers
  every non-prestige class from `data/sheet_rules` (census `ids=135
  computed=61 blocked=0`, 2026-09-24). Samurai and the 18 untabled-family
  classes formerly listed here all reach `Computed`.
- **`Computed` but not in the desktop Create picker (11 of 42)**:
  Gunslinger, Ninja (Ultimate Combat); Kineticist, Medium, Mesmerist,
  Occultist, Psion, Psychic, Shifter, Spiritualist, Vigilante (untabled
  exotic). A UI-surface gap, not an engine gap.
- **Prestige in a carrier mix (67 of 74 `Computed`)**: census
  `prestige_mix_computed=67 prestige_mix_unknown=0`, 2026-09-24, SD-36 F3c2
  (`artifacts/epic-f/census-f3c2.json`). The carrier chooser walks an `AtLeast`
  clause's branches in oracle order and takes the first that translates to a
  carrier + level, reads `HighestSpellLevel(Any)` as "at least 1 of Arcane,
  Divine", and reads a `Not` of a spell-kind term as a prohibition; each row's
  `carrier_reason` names the branch taken. Of the other 7: 6 are Blocked on
  `multiclass.save_shape.unrecognized`, an oracle `BONUS:SAVE` formula defect
  (Evangelist, Exalted, Mammoth Rider, Pure Legion Enforcer, Sentinel, Ulfen
  Guard), closable only by a book-cited override (`forward-scope-register.md`
  FS-15); 1 is Blocked in its sorcerer mix (Dragon Disciple: a gate branch that
  names `ClassLevel(<class>) >= n` makes that class the carrier since F3c2, and
  the gate-named draconic bloodline is seeded as the sorcerer's pick; the mix
  stops on `combat.baseline_weapon_proficiency_unknown` -- Dragon Disciple's
  converted closure grants no weapon proficiency and carries no closure-complete
  attestation, because its one closure defect is a reference to an oracle row
  outside the 49,450-unit inventory (`Internal|Bite`, `ce_abilities_race.lst:249`;
  re-checked F3c3) -- and on the Sorcerer seam, which grounds the Arcane bloodline
  only).
  A prestige class alone is never `Computed` (74 of 74 Blocked, the game rule).
- **Multiclass scope**: every family can mix since SD-36 F3b, with two named
  save-source remainders: the 4 Pathfinder Unchained classes (no CRB table row
  and no converted chassis record: `multiclass.save_shape.unknown`) and the 6
  prestige records with an Unrecognized save (`multiclass.save_shape.unrecognized`).

**Refuted claims — corrected here, not restated anywhere else in this
repo**:
- ~~"139 classes hold a chassis"~~ — that summed registry *populations*
  without removing overlap (31+3+27+78 = 139); the real distinct-with-chassis
  count is **117 of 135** (subtracting the 19+3 = 22 ids `generic_class_chassis`'s
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
  classes do; the reason none reach `Computed` is a missing gate arm
  (above), not an absent chassis.

### Capability matrix: content kinds, corpus-wide

The corpus spans **38** tracked Pathfinder 1e books, reconciled across three
different countable things (none of which is itself 38): the `RuleSetId`
enum has **37** variants (`awk '/pub enum RuleSetId/,/^}/' src/rules_core/rules_tables/mod.rs
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
| Feats | 2,227 entries (1,578 hand-authored + 649 corpus-gap rows) | 23 books | `src/rules_core/rules_tables/feats_all.rs:722-767,819` |
| Monsters | 330 Bestiary-1 creatures (46 hand-modelled + 284 chassis-only) + per-book counts | **22** books, exhaustive (`book_display_name`/`book_wire_code` each have exactly 22 match arms and panic on an unregistered book) | `apps/desktop/src-tauri/src/monster_catalog.rs:216-262` |
| Races / race traits | `RACE_CATALOG_BOOKS` | 6 books: CRB, B1, B2, B5, B6, ARG | `apps/desktop/src-tauri/src/race_catalog.rs:170` |
| Classes / class features | 135 distinct ids corpus-wide (all engine registries merged); 117 of 135 have a real chassis, 61 of 135 (61 of 61 non-prestige) reach `Computed` | 18 source books carry class content | see "Class/level compute coverage — corpus-wide" above for the full breakdown, set algebra, and re-derive commands — not restated here |

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
- **The Tauri command count is 75**, re-derived directly from
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
compute receipt. Both are now wide (100% of 49,450 units; 61 of 135 distinct
class ids — every one of the 61 non-prestige ids, all swept and
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
| Rule-table catalogs | **37** `RuleSetId` variants spanning the 38-book tracked corpus (see "Capability matrix: content kinds, corpus-wide" above, this same file, for the re-derived denominator and reconciliation) — see [rules-data-tables.md](./rules-data-tables.md) for per-book ceiling detail | [rules-data-tables.md](./rules-data-tables.md) |
| Character Hub | Create, load, clone, portrait upload/load/delete, JSON export, recompute, plus (new since the last full pass) equipment purchase/attach, feat/trait selection, skill allocation, bio/money/HP tracking — all real engine compute + real persistence | [desktop-app.md](./desktop-app.md) |
| DM Toolkit | Real encounter builder + DM console export (`rate_encounter`, `export_dm_console`), consuming the real `Encounter`/party-CR compute below | [desktop-app.md](./desktop-app.md) |
| Rule-system adapter seam (hub-of-hubs) | `RuleSystemAdapter` trait is the object-safe seam three commands (`append_to_character`/`recompute_character`/`re_save_character`) dispatch through on a `rule_system_id`: `"pf1"` resolves to the real `Pf1Adapter`; any other id resolves to the governed `StubAdapter` (registered exception 0002, `docs/governance/wired-integration-stubs-registry.md`). All other character-mutation commands call PF1 free functions directly and do not go through this seam | [desktop-app.md](./desktop-app.md) §"Rule-system adapter seam" |
| Corpus-ingest diagnostic | `corpus_ingest_diagnostic` Tauri command reports real ingested-record-kind counts per book, counted from the tables actually compiled into the binary | [desktop-app.md](./desktop-app.md) |
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
| Campaign conflict merge | Conflict detection is real and tested (nonce-based); resolution is local-wins with both copies preserved under `conflicts/<timestamp>/` — there is no merge UI. | [persistence.md](./persistence.md) §"Conflict detection" |

### Core engine: compute coverage and proof surfaces

| Item | Status | Where (re-verified) |
|---|---|---|
| Class/level compute coverage | Wide, not narrow, corpus-wide — 61 of 135 distinct class ids (61 of 61 non-prestige) reach `Computed`; full breakdown, per-family table, and the four named-exception lists (blocked-only-on-weapon-proficiency, computed-but-not-in-picker, prestige with/without chassis, multiclass scope) live in "Class/level compute coverage — corpus-wide" above; not restated here. | see table above |
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
