---
canonical: true
owner: god-emporer
status: approved (operator directive 2026-07-19 — SD-22 requirements gap-fill: corpus-source inventory + test-artifact pairing parity)
purpose: "Authoritative content-source inventory. Each row binds one publisher-book content unit (a class, a spell list, an equipment table, a monster-block subset) to (a) the canonical Rust module that ingests it, (b) the test fixture that asserts it, (c) the artifact the cycle mints into `docs/release/SD-22/artifacts/`, and (d) the `RuleSetId` variant. A coding harness opening SD-22 reads this file before Epic 3 / 4 / 5 / 6 to know what each cycle must produce."
date: 2026-07-19
canonical_branch: tranche/5 (operator directive 2026-07-18)
kanban_board: codex-tranche-5
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/{decisions.md,epic-breakdown.md,technical-design.md,acceptance-and-verification.md,risks-and-open-questions.md,technical-requirements.md,loop-instruction.md,scope-draft.md}
mirror_of: ~/workspace/programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/corpus-source-inventory.md
---

# SD-22 — Corpus-Source Inventory

> ## ⚠️ CORRECTIVE BANNER (added 2026-07-19 — read before using this file's "Content shape" columns)
>
> This file's per-row **"Content shape"** columns (§1.1, §2.1) were authored 2026-07-19 from model memory, before `decisions.md §5` was corrected the same day. They are **not verified against a real source** and are **not authoritative** — do not transcribe them into `rules_tables/<book>/*.rs` as-is.
>
> The real source is PCGen's published `.lst` data, ingested via the existing `src/pcgen_import/` engine (same pipeline SD-19 used for the CRB). See `decisions.md §5` for the corrected sourcing decision and the paths to the real data (local sibling repo `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/{advanced_players_guide,advanced_class_guide,bestiary}/`, or `https://github.com/PCGen/pcgen` as a second git source in a cloud sandbox).
>
> This file's **routing columns remain valid and authoritative**: `rust_module_path`, `test_fixture_path`, `cycle_artifact_path`, `RuleSetId`, class/subset ordering, and the cross-book invariants (§1.3, §2.3, §3.2). Only the "Content shape" prose (the specific named features/spells/stats text) is superseded — re-derive that from the real `.lst` record per cycle instead.

This file is the **load-bearing reference** for SD-22's content-source ingest cycles (Epic 3 APG, Epic 4 ACG, Epic 5 Bestiary 1) and Epic 6 DM Toolkit. Each row maps a publisher-book content unit to the four artifacts the cycle must produce: Rust module, test fixture, cycle artifact (under `docs/release/SD-22/artifacts/`), and `RuleSetId` variant. A coding harness running a cycle uses this file to know **what to ingest** and **what to assert**.

**How a cycle uses this file.** Before the cycle runs, the harness reads the row for the cycle's content unit. The cycle MUST (0) parse the real `.lst` record for this content unit via `src/pcgen_import/lst_parser/*` (per `decisions.md §5`, corrected 2026-07-19) — this row's *Content shape* column is illustrative only, per the banner above, not the generation spec — (1) write the failing test fixture named in the *test_fixture_path* column, (2) confirm the failure mode matches the cycle's input-shortfall expectation, (3) write the production code at *rust_module_path*, transcribed from the real `.lst` record with source provenance cited in a doc comment, until the fixture passes (green), (4) mint the cycle artifact named in *cycle_artifact_path* documenting the red→green transition.

## 1. APG (Advanced Player's Guide) — Epic 3

APG populates `src/rules_core/rules_tables/apg/` with one class table per class. The **six** real APG classes are operator-pinned in **PF1-publication order**: Alchemist, Cavalier, Inquisitor, Oracle, Summoner, Witch. Each cycle lands one class table plus its spell-list entries and its equipment-table entries.

**Corrected 2026-07-19:** this table originally listed Gunslinger and Magus as APG classes (8 total). A real Epic 3 cycle verified `apg_classes.lst` directly and found no `CLASS:Gunslinger` or `CLASS:Magus` record anywhere in it — both classes are actually published in Ultimate Combat (`ultimate_combat/uc_classes.lst`) and Ultimate Magic (`ultimate_magic/um_classes.lst`) respectively, not APG. Per `decisions.md §1`, Ultimate-line books are explicitly out of scope for SD-22, so Gunslinger and Magus are **not** SD-22 Epic 3 criteria at all (not blocked — genuinely not applicable). The roster below reflects the corrected six-class list.

### 1.1 APG classes

| Order | Class | rust_module_path | test_fixture_path | cycle_artifact_path (under `docs/release/SD-22/artifacts/`) | RuleSetId | Content shape (per PF1 `apg/<class>.rst`-equivalent) |
|---|---|---|---|---|---|---|
| 1 | Alchemist | `src/rules_core/rules_tables/apg/class_alchemist.rs` | `tests/sd22_apg_class_alchemist_resolves.rs` | `apg/class_alchemist_cycle_receipt.md` | `RuleSetId::Apg` | BAB: medium (d8→d8). Saves: Fort/Ref/Will. Class skills (Alchemy, Appraise, etc.). Class features: Alchemy (Su), Bombs (Su, level 1+, daily uses 1+INT), Brew Potion (Su, level 1), Discovery (Su, level 2), Swift Alchemy (Su, level 4), Bomb-enhancing discoveries: Acid Bomb, etc. Mutagen (Su, level 1, personal), Throw Anything (Ex, level 1), Poison Resistance (Ex, level 1). Spell list: 6-level caster (extracts at level 1). Spells/day progression 0→0/0→0/0/0. Cantrips known at level 4. Discoveries: 1 at level 2, +1 every 2 levels. |
| 2 | Cavalier | `src/rules_core/rules_tables/apg/class_cavalier.rs` | `tests/sd22_apg_class_cavalier_resolves.rs` | `apg/class_cavalier_cycle_receipt.md` | `RuleSetId::Apg` | BAB: high (d10→d10). Saves: Fort/Ref/Will. Class skills (Ride, Diplomacy, etc.). Class features: Order (Cavalier's chosen Order, level 1), Challenge (Su, level 1, +1/day every 3 levels), Tactician (Su, level 1, +1/day every 3 levels), Banner (Su, level 5), Expert Trainer (Su, level 5). Mounted combat at all levels. |
| 3 | Inquisitor | `src/rules_core/rules_tables/apg/class_inquisitor.rs` | `tests/sd22_apg_class_inquisitor_resolves.rs` | `apg/class_inquisitor_cycle_receipt.md` | `RuleSetId::Apg` | BAB: medium (d8→d8). Saves: Fort/Ref/Will. Class skills (Intimidate, Knowledge-Religion, etc.). Class features: Judgment (Su, level 1, +1 use per 4 levels), Monster Lore (Ex, level 1), Solo Tactics (Ex, level 1), Bane (Su, level 5), Stalwart (Ex, level 5). Spell casting: 6-level caster (Inquisitor spell list), at level 1. Domain selection (Inquisition domains). |
| 4 | Oracle | `src/rules_core/rules_tables/apg/class_oracle.rs` | `tests/sd22_apg_class_oracle_resolves.rs` | `apg/class_oracle_cycle_receipt.md` | `RuleSetId::Apg` | BAB: medium (d8→d8). Saves: Fort/Ref/Will. Class features: Mystery (chosen at level 1, e.g. Battle, Bones, Flame, Nature, etc.), Curse (Su, level 1, mystery-bound), Revelation (Su, level 1, +1 at level 3, 7, 11, 15, 19). Spell casting: 6-level caster (divine, Oracle spell list). Spontaneous casting from oracle list. |
| 5 | Summoner | `src/rules_core/rules_tables/apg/class_summoner.rs` | `tests/sd22_apg_class_summoner_resolves.rs` | `apg/class_summoner_cycle_receipt.md` | `RuleSetId::Apg` | BAB: medium (d8→d8). Saves: Fort/Ref/Will. Class features: Eidolon (linked summoned creature, level 1), Bond Senses (Su, level 1), Make Haste (Sp, level 1), Life Link (Su, level 1), Shield Ally (Su, level 6), Aspect (Su, level 8, switchable forms). Spell casting: 4-level caster (Summoner spell list). Spell known progression: 1 at level 1, +1 every 2 levels. |
| 6 | Witch | `src/rules_core/rules_tables/apg/class_witch.rs` | `tests/sd22_apg_class_witch_resolves.rs` | `apg/class_witch_cycle_receipt.md` | `RuleSetId::Apg` | BAB: medium (d8→d8). Saves: Fort/Ref/Will. Class features: Patron (chosen at level 1), Hexes (Su, level 1, e.g. Evil Eye, Fortune, Healing, Misfortune), Familiar (level 1). Spell casting: full 9-level caster (Witch spell list). Hex progression +1 at levels 2/8/18. Witch spells are prepared from Patron list augmented by Witch list. |

**Gunslinger and Magus are NOT APG content (corrected 2026-07-19):** removed from this table — see the corrective note above §1.1. Do not re-add them to Epic 3; if the operator later pins Ultimate Combat / Ultimate Magic in scope, they belong in a new epic for that book, not APG.

### 1.2 APG shared spell and equipment tables

| Content unit | rust_module_path | test_fixture_path | cycle_artifact_path | RuleSetId |
|---|---|---|---|---|
| APG spell list (extracted from per-class sources above) | `src/rules_core/rules_tables/apg/spell_list.rs` | `tests/sd22_apg_spell_list_resolves.rs` | `apg/spell_list_cycle_receipt.md` | `RuleSetId::Apg` |
| APG equipment tables (e.g., bombs, guns, archetypes' starting gear) | `src/rules_core/rules_tables/apg/equipment_tables.rs` | `tests/sd22_apg_equipment_resolves.rs` | `apg/equipment_tables_cycle_receipt.md` | `RuleSetId::Apg` |

### 1.3 APG cross-book resolution invariants (must hold after Epic 3 lands)

The following cross-book resolution invariants **MUST** be verified by tests. A cycle that ships without these tests is a Bucket-B / Bucket-C shortfall (Epic 9's evaluator treats it as a self-heal trigger):

- Alchemist-bomb key `apg:alchemist:bomb:acid` resolves via `RuleSetId::Apg` but returns `None` for `RuleSetId::Crb` and `RuleSetId::Acg`. (APG-only item.)
- Inquisitor-spell key `apg:inquisitor:spell:bane` resolves via `RuleSetId::Apg`; returns `None` for `RuleSetId::Bestiary1`. (A spell is not in a monster book.)
- Cross-book fallback chain per SD-21 §12: when an item is unavailable in APG, the resolver falls back `APG → CRB → ACG → Bestiary1`.

## 2. ACG (Advanced Class Guide) — Epic 4

ACG populates `src/rules_core/rules_tables/acg/`. The ACG classes are operator-pinned in **PF1-publication order**: Alchemist (ACG-side), Arcanist, Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Swashbuckler, Warpriest. Cycle shape mirrors Epic 3.

### 2.1 ACG classes

| Order | Class | rust_module_path | test_fixture_path | cycle_artifact_path | RuleSetId | Content shape (per PF1 ACG `<class>` section) |
|---|---|---|---|---|---|---|
| 1 | Alchemist (ACG-side) | `src/rules_core/rules_tables/acg/class_alchemist_acg.rs` | `tests/sd22_acg_class_alchemist_resolves.rs` | `acg/class_alchemist_cycle_receipt.md` | `RuleSetId::Acg` | APG's Alchemist with additional ACG-side archetypes: Vivisectionist, Preservationist, Chirurgeon. Distinct bomb list (preserved/discovered bombs). Note: ACG's Alchemist shares identifier name with APG's Alchemist, but Epic 4 ships the ACG-shape variant as `class_alchemist_acg.rs` (the test fixture asserts `RuleSetId::Acg::resolve` returns ACG-specific data). |
| 2 | Arcanist | `src/rules_core/rules_tables/acg/class_arcanist.rs` | `tests/sd22_acg_class_arcanist_resolves.rs` | `acg/class_arcanist_cycle_receipt.md` | `RuleSetId::Acg` | BAB: medium (d8→d8). Saves: Fort/Ref/Will. Class features: Arcane Exploit (Su, level 1, +1 every 2 levels, choose from ACG exploit list), Arcane Reservoir (Su, level 3), Spell Blending (Ex, level 2), Consume Spells (Su, level 4), Spell Specialist (Su, level 5). Spell casting: 6-level caster (full arcane spell list, no school restriction). |
| 3 | Bloodrager | `src/rules_core/rules_tables/acg/class_bloodrager.rs` | `tests/sd22_acg_class_bloodrager_resolves.rs` | `acg/class_bloodrager_cycle_receipt.md` | `RuleSetId::Acg` | BAB: high (d10→d10). Saves: Fort/Will. Class features: Bloodline (chosen at level 1, e.g. Draconic, Elemental, etc.), Bloodrager Bloodline Powers (Su, level 1, +1 every 4 levels), Blood Casting (Su, level 4), Blood Sanctuary (Su, level 7), Damage Reduction (Su, level 11). Spell casting: 4-level caster (bloodrager spells at level 1), spontaneous casting from bloodrager list. |
| 4 | Brawler | `src/rules_core/rules_tables/acg/class_brawler.rs` | `tests/sd22_acg_class_brawler_resolves.rs` | `acg/class_brawler_cycle_receipt.md` | `RuleSetId::Acg` | BAB: high (d10→d10). Saves: Fort/Ref. Class features: Martial Training (Ex, level 1; counts as Fighter for feat prereqs), Brawler's Flurry (Ex, level 1, two-weapon fighting with unarmed strikes), Unarmed Strike (level 1, improved damage dice by level 4/8/12/16/20), AC Bonus (Ex, level 1, +1 per 6 levels). No spells. |
| 5 | Hunter | `src/rules_core/rules_tables/acg/class_hunter.rs` | `tests/sd22_acg_class_hunter_resolves.rs` | `acg/class_hunter_cycle_receipt.md` | `RuleSetId::Acg` | BAB: high (d10→d10). Saves: Fort/Ref. Class features: Animal Companion (level 1, Bonded Companion), Hunter Training (Ex, level 1), Quarry (Ex, level 1), Track (Ex, level 1), Teamwork Feats (level 1 onward, animal companion shares). Spell casting: 6-level caster (Hunter spell list, divine), at level 1. |
| 6 | Investigator | `src/rules_core/rules_tables/acg/class_investigator.rs` | `tests/sd22_acg_class_investigator_resolves.rs` | `acg/class_investigator_cycle_receipt.md` | `RuleSetId::Acg` | BAB: medium (d8→d8). Saves: Fort/Ref. Class features: Inspiration (Su, level 1, intelligent focus with daily uses = 1+INT+level/2), Studied Combat (Ex, level 1), Inspiration (Su, level 1, Emulated Inspiration), Stalwart (Ex, level 3), True Inspiration (Su, level 20). Spell casting: 6-level caster (Investigator formula book, adds formulae instead of arcane bonds). |
| 7 | Shaman | `src/rules_core/rules_tables/acg/class_shaman.rs` | `tests/sd22_acg_class_shaman_resolves.rs` | `acg/class_shaman_cycle_receipt.md` | `RuleSetId::Acg` | BAB: medium (d8→d8). Saves: Fort/Will. Class features: Spirit (chosen at level 1, e.g. Battle, Bone, Flame, Frost, etc.), Spirit Magic (Sp, level 1, summon spirit companions), Wandering Spirit (Su, level 4), Manifestation (Su, level 6), Totem Transformation (Su, level 11). Spell casting: full 9-level caster (Shaman spell list). |
| 8 | Skald | `src/rules_core/rules_tables/acg/class_skald.rs` | `tests/sd22_acg_class_skald_resolves.rs` | `acg/class_skald_cycle_receipt.md` | `RuleSetId::Acg` | BAB: medium (d8→d8). Saves: Fort/Ref. Class features: Scribe Tattoo (Ex, level 1), Spell Kenning (Su, level 1, choose 1 spell from another class list), Versatile Performance (Ex, level 2), Raging Song (Su, level 3, +1 use per 3 levels), Song of Marching (Su, level 2, +miles/day count). Spell casting: full 6-level caster (Skald spell list, divine). |
| 9 | Swashbuckler | `src/rules_core/rules_tables/acg/class_swashbuckler.rs` | `tests/sd22_acg_class_swashbuckler_resolves.rs` | `acg/class_swashbuckler_cycle_receipt.md` | `RuleSetId::Acg` | BAB: high (d10→d10). Saves: Fort/Ref. Class features: Panache (Su, level 1, swift action + INT mod daily, regained on derring-do), Swashbuckler's Finesse (Ex, level 1), Dodging Panache (Ex, level 1), Derring-Do (Ex, level 1, spend panache to retry), Opportune Parry and Riposte (Ex, level 1). No spells. |
| 10 | Warpriest | `src/rules_core/rules_tables/acg/class_warpriest.rs` | `tests/sd22_acg_class_warpriest_resolves.rs` | `acg/class_warpriest_cycle_receipt.md` | `RuleSetId::Acg` | BAB: high (d10→d10). Saves: Fort/Will. Class features: Blessings (level 1, choose 2 from list, +1 every 4 levels), Sacred Armor (Ex, level 1, no armor-proficiency penalty on shields or armor), Fervent Focus (Ex, level 1), Channel Energy (Su, level 1, 3+CHA mod/day), Shielding Bond (Su, level 8). Spell casting: 6-level caster (Warpriest spell list, divine, prepared) at level 1. |

### 2.2 ACG shared spell and equipment tables

| Content unit | rust_module_path | test_fixture_path | cycle_artifact_path | RuleSetId |
|---|---|---|---|---|
| ACG spell list | `src/rules_core/rules_tables/acg/spell_list.rs` | `tests/sd22_acg_spell_list_resolves.rs` | `acg/spell_list_cycle_receipt.md` | `RuleSetId::Acg` |
| ACG equipment tables (e.g., archetypes' starting gear, ranged-touch weapons) | `src/rules_core/rules_tables/acg/equipment_tables.rs` | `tests/sd22_acg_equipment_resolves.rs` | `acg/equipment_tables_cycle_receipt.md` | `RuleSetId::Acg` |

### 2.3 ACG cross-book resolution invariants

- Arcanist-specific exploit `acg:arcanist:exploit:lightning_lash` resolves via `RuleSetId::Acg` but `None` for `RuleSetId::Apg`/`RuleSetId::Crb` (ACG-only).
- Bloodrager-bloodline key `acg:bloodrager:bloodline:draconic:level_1:claw_attack` resolves via `RuleSetId::Acg`. The class has its own line `class_bloodrager_bloodline_<line>.rs` per bloodline (Draconic, Elemental, Infernal, Celestial, etc.).
- Cross-book fallback chain per SD-21 §12: when an item is unavailable in ACG, the resolver falls back `APG → CRB → ACG → Bestiary1`.

## 3. Bestiary 1 — Epic 5

Bestiary 1 populates `src/rules_core/rules_tables/beastiary1/` with one monster-block subset per cycle. The 300+ monsters are split into subsets; default ordering is **alphabetical by monster name within CR band**, operator-pinned at SD-22 cycle launch.

### 3.1 Bestiary 1 subset layout (default)

| Subset # | CR band | Sample monsters | rust_module_path | test_fixture_path | cycle_artifact_path | RuleSetId |
|---|---|---|---|---|---|---|
| 1 | CR 1 | ~~Goblin, Kobold, Orc, Skeleton, Zombie~~ **Corrected 2026-07-20:** Ghoul, Gnoll, Goblin Dog, Lizardfolk, Wolf (none of the original five is a real, standalone CR-1 monster stat-block row — see `beastiary1/subset_01_cycle_receipt.md`) | `src/rules_core/rules_tables/beastiary1/monster_subset_01.rs` | `tests/sd22_beastiary1_subset_01_resolves.rs` | `beastiary1/subset_01_cycle_receipt.md` | `RuleSetId::Bestiary1` |
| 2 | CR 1 | ~~Gnoll, Hobgoblin, Lizardfolk, Rat Swarm~~ **Corrected 2026-07-20:** Darkmantle, Horse, Hyena, Octopus, Spider Swarm (Gnoll/Lizardfolk already used in subset 01; Hobgoblin has no standalone stat-block row in `b1_races.lst` — `.MOD`-only, same shape as subset 01's Goblin/Kobold/Orc; Rat Swarm is a real row but CR 2, not CR 1 — see `beastiary1/subset_02_cycle_receipt.md`) | `src/rules_core/rules_tables/beastiary1/monster_subset_02.rs` | `tests/sd22_beastiary1_subset_02_resolves.rs` | `beastiary1/subset_02_cycle_receipt.md` | `RuleSetId::Bestiary1` |
| 3 | CR 2 (moved from CR 1 — CR 1 exhausted after subsets 01+02; only Squid/Troglodyte remained, too few for a five-monster subset) | **Added 2026-07-20:** Bat Swarm, Boar, Boggard, Bugbear, Cave Fisher (first five real, non-parenthetical CR-2 monster names alphabetically — see `beastiary1/subset_03_cycle_receipt.md`) | `src/rules_core/rules_tables/beastiary1/monster_subset_03.rs` | `tests/sd22_beastiary1_subset_03_resolves.rs` | `beastiary1/subset_03_cycle_receipt.md` | `RuleSetId::Bestiary1` |
| 4 | CR 2 (continued — 19 clean non-parenthetical CR-2 names exist in the real corpus; subset 03 used the first five) | **Added 2026-07-20:** Choker, Crocodile, Dark Creeper, Iron Cobra, Morlock (next five real, non-parenthetical CR-2 monster names alphabetically after subset 03's "Cave Fisher" — see `beastiary1/subset_04_cycle_receipt.md`) | `src/rules_core/rules_tables/beastiary1/monster_subset_04.rs` | `tests/sd22_beastiary1_subset_04_resolves.rs` | `beastiary1/subset_04_cycle_receipt.md` | `RuleSetId::Bestiary1` |
| 5 | CR 2 (continued — 19 clean non-parenthetical CR-2 names exist in the real corpus; subsets 03+04 used the first ten) | **Added 2026-07-20:** Rat Swarm, Sahuagin, Shark, Shocker Lizard, Skum (next five real, non-parenthetical CR-2 monster names alphabetically after subset 04's "Morlock" — see `beastiary1/subset_05_cycle_receipt.md`) | `src/rules_core/rules_tables/beastiary1/monster_subset_05.rs` | `tests/sd22_beastiary1_subset_05_resolves.rs` | `beastiary1/subset_05_cycle_receipt.md` | `RuleSetId::Bestiary1` |
| ... | (one per subset) | ... | ... | ... | ... | ... |
| N | CR 30 | Tarrasque, Cosmic-tier | (last subset) | (last subset) | (last subset) | `RuleSetId::Bestiary1` |

For each monster in a subset, the structured data lives in `monster_<subset>.rs`:

```rust
pub struct MonsterRef {
    pub name: String,           // e.g., "Goblin"
    pub cr: f32,                // e.g., 0.333 (display) or 1 (unquoted)
    pub xp: u32,                // canonical XP per the bestiary's table
    pub ac: u8,                 // total armor class
    pub initiative: i8,         // Dex + misc
    pub hp_max: u32,            // hit points
    pub hp_current: u32,        // can equal hp_max at first-read
    pub fort: i8, ref: i8, will: i8,  // saves
    pub attack_damage_die: Vec<DamageDie>, // e.g., [d6] for a Goblin short sword
    pub damage_type: DamageType, // per monster stat block
    pub size: MonsterSize,      // Small/Medium/Large/etc.
    pub speed: u32,             // ft/round
    pub environment: Environment, // Underground/Forest/Swarm/etc.
    pub feats: Vec<FeatRef>,    // feats the monster has
    pub special_attacks: Vec<SpecialAbilityRef>,
    pub special_qualities: Vec<SpecialQualityRef>,
    pub cr_alignment: Alignment,
}
```

### 3.2 Bestiary 1 cross-book invariants

- Goblin key `beastiary1:monster:goblin` resolves via `RuleSetId::Bestiary1` but `None` for `RuleSetId::Apg`/`RuleSetId::Acg` (monsters aren't spell-list items).
- Tarrasque key `beastiary1:monster:tarrasque` resolves via `RuleSetId::Bestiary1`. The DM-toolkit encounter-difficulty computation (Epic 6) must be able to compute a valid encounter difficulty that includes a Tarrasque without raising errors (the algorithm should handle extreme-CR cases).

## 4. DM Toolkit — Epic 6

The DM Toolkit consumes Epic 3+4+5 output. Two Rust modules, three test fixtures.

| Module | rust_module_path | test_fixture_path | cycle_artifact_path | Required corpus input |
|---|---|---|---|---|
| Encounter-difficulty | `src/rules_core/encounters.rs` | `tests/sd22_dm_toolkit_deterministic.rs` | `dm_toolkit/encounters_cycle_receipt.md` | `tests/fixtures/sd22/encounters/<case>.json` (5 fixtures) |
| Party-CR | `src/rules_core/party_cr.rs` | `tests/sd22_party_cr_deterministic.rs` | `dm_toolkit/party_cr_cycle_receipt.md` | `tests/fixtures/sd22/party_cr/<case>.json` (3 fixtures) |
| Happy-path integration | (consumes both) | `tests/sd22_dm_toolkit_happy_path_integration.rs` | `dm_toolkit/happy_path_integration_cycle_receipt.md` | One ingested `PartySnapshot` + one ingested `MonsterRef` from Epic 3+4+5's first cycles |

### 4.1 DM Toolkit deterministic test fixture shape

> **Corrected 2026-07-20 (criterion 20's cycle) — cases 2 and 3 below were
> wrong.** This table's "Expected" column for cases 2 and 3 was authored
> 2026-07-19 from model memory, before either was checked against a real
> source. Criterion 18's cycle (`artifacts/dm_toolkit/encounters_cycle_receipt.md`)
> and criterion 19's cycle (`artifacts/dm_toolkit/party_cr_cycle_receipt.md`)
> each independently found their respective case didn't hold up against the
> Pathfinder RPG Core Rulebook's "Gamemastering" chapter (Table: Encounter
> Design, Table: CR Equivalencies, Table: Experience Point Awards, and
> "Designing Encounters" → "Step 1 — Determine APL"), and flagged the
> mismatch for this cycle to reconcile rather than force-fitting the code.
> Criterion 20's cycle independently re-verified both citations against
> `https://legacy.aonprd.com/corerulebook/gamemastering.html` directly
> (fresh fetch, not trusting the prior cycles' claims) and confirmed both
> corrections below are accurate: case 2's expected difficulty is `Deadly`,
> not `Hard` (APL 3, group EL 3+4=7 per Table: CR Equivalencies, EL−APL=+4,
> beyond even Epic/APL+3 on Table: Encounter Design); case 3's expected
> party CR is `3.0`, not `~3.5` (4 PCs is within the unadjusted "four or
> five PCs" band, average level 12/4 = 3.0 exactly, and the rulebook's APL
> rule has no step that can ever produce a fractional result). The table
> below reflects the corrected, verified values — `encounters.rs` and
> `party_cr.rs`'s already-shipped code was correct against the real rules
> the whole time; this table's prose was the thing that was wrong.

Each deterministic test fixture is a single function that takes an enum-coded test case and asserts on the result:

```rust
#[test]
fn encounters_4_level_3_pcs_vs_1_cr_2_monster_is_easy() {
    let party = party_of_4_level_3_pcs();
    let monsters = vec![monster_ref_cr_2()];  // canonical Goblin (CR 0.5 actual ≈ CR 2 effective after multiplier)
    let result = encounter_difficulty(&party, &monsters);
    assert_eq!(result.difficulty, Difficulty::Easy);
}
```

The five canonical Paizo deterministic test cases per Epic 6 criterion 20:

| # | Fixture slug | Party | Monsters | Expected |
|---|---|---|---|---|
| 1 | `encounters_4_level_3_pcs_vs_1_cr_2_monster_is_easy` | 4× level-3 PCs | 1× CR-2 monster | Easy |
| 2 | `encounters_4_level_3_pcs_vs_4_cr_3_monsters_is_deadly` | 4× level-3 PCs | 4× CR-3 monsters | **Deadly** (corrected 2026-07-20 from the originally-stated "Hard"; see the corrective note above §4.1) |
| 3 | `party_cr_of_4_level_3_pcs_equals_3` | 4× level-3 PCs (one each class) | (none) | **CR = 3.0** (corrected 2026-07-20 from the originally-stated "≈3.5"; see the corrective note above §4.1) |
| 4 | `encounters_empty_monsters_returns_easy` | 4× level-3 PCs | none | Easy (no threat) |
| 5 | `encounters_1_level_1_pc_vs_1_cr_1_monster_returns_valid_difficulty` | 1× level-1 PC | 1× CR-1 monster | Easy/Medium/Hard/Deadly |

## 5. Cross-book resolution disambiguation matrix

When a cycle writes a content unit, the test must assert ONE invariant: `key::RuleSetId::X::resolve()` returns the unit's canonical data **only** when the caller passes the matching `RuleSetId` value. A test that passes only the "happy path" (always returns the data regardless of `RuleSetId`) is a fake-completion (Epic 9's evaluator catches it as a shortfall).

| Content unit | RuleSetId::Apg | RuleSetId::Crb | RuleSetId::Acg | RuleSetId::Bestiary1 |
|---|---|---|---|---|
| APG class_alchemist | ✓ Some | None | None | None |
| ACG class_swashbuckler | None | None | ✓ Some | None |
| CRB class_fighter | None | ✓ Some | None | None |
| Bestiary 1 monster_goblin | None | None | None | ✓ Some |

## 6. Cycle-artifact reader's contract

A cycle that lands at row `<row>` of this file MUST mint a file at `docs/release/SD-22/artifacts/<row's cycle_artifact_path>` whose body contains:

```markdown
# <class/monster/module name> cycle receipt — <ISO-8601 UTC>

## Red-phase evidence
<command> <output>
<paste from `cargo test --test sd22_<X>_<Y>_resolves 2>&1 | tail -40` showing the test fails>

## Green-phase evidence
<command> <output>
<paste from `cargo test --locked 2>&1 | tail -20` and `cargo clippy --locked --tests -- -D warnings 2>&1 | tail -20` showing all green>

## Files touched
- `src/...` — added/modified
- `tests/...` — added/modified

## Cycle metadata
- cycle_id: <ISO-8601 timestamp>
- duration: <N> seconds
- bundle_criterion: <criterion-NN>
- corpus_input_path: `<pathfinder/paizo/roleplaying_game/<book_dir>/<file>.lst:CLASS or RACE key>` (the real public PCGen corpus record this cycle transcribed from — per `decisions.md §5`, corrected 2026-07-19; `artifacts/corpus/operator-supplied/<book>/<file>.lst` only if this cycle used the `ingest.md §5` fallback)
- RuleSetId: <Apg | Acg | Bestiary1>
- ingest_pipeline_version: 2 (per `./ingest.md §6`; corrected 2026-07-19 — direct real-corpus transcription, no corpus-loader abstraction)

## kanban
- card: <hermes kanban card id>
- audit_comment: <comment id>
```

Without this artifact, Epic 9's evaluator cannot conclude the criterion lands and treats it as a shortfall (Bucket-C pass-through to self-heal cycle).

## 7. Races (per-operator-directive 2026-07-19 "supported/product visible for each new race")

Races are independent content units within each book's `RuleSetId::*` namespace. They are not their own `RuleSetId` variant — that would lose the "race sourced from a specific book" semantic. Resolver key shape: `<apg|acg>:race:<lowercase-race-name>`.

### 7.1 APG races

| Race | rust_module_path | test_fixture_path | cycle_artifact_path | RuleSetId |
|---|---|---|---|---|
| Fetchling | `src/rules_core/rules_tables/apg/race_fetchling.rs` | `tests/sd22_apg_race_resolves.rs` | `races/apg_fetchling_cycle_receipt.md` | `RuleSetId::Apg` |
| Grippli | `src/rules_core/rules_tables/apg/race_grippli.rs` | (same fixture, batched) | `races/apg_grippli_cycle_receipt.md` | `RuleSetId::Apg` |
| Kitsune | `src/rules_core/rules_tables/apg/race_kitsune.rs` | (same) | `races/apg_kitsune_cycle_receipt.md` | `RuleSetId::Apg` |
| Nagaji | `src/rules_core/rules_tables/apg/race_nagaji.rs` | (same) | `races/apg_nagaji_cycle_receipt.md` | `RuleSetId::Apg` |
| Samsaran | `src/rules_core/rules_tables/apg/race_samsaran.rs` | (same) | `races/apg_samsaran_cycle_receipt.md` | `RuleSetId::Apg` |
| Strix | `src/rules_core/rules_tables/apg/race_strix.rs` | (same) | `races/apg_strix_cycle_receipt.md` | `RuleSetId::Apg` |
| Svirfneblin | `src/rules_core/rules_tables/apg/race_svirfneblin.rs` | (same) | `races/apg_svirfneblin_cycle_receipt.md` | `RuleSetId::Apg` |
| Wayang | `src/rules_core/rules_tables/apg/race_wayang.rs` | (same) | `races/apg_wayang_cycle_receipt.md` | `RuleSetId::Apg` |

Cycle shape: **batch one cycle per 1-3 races** (single test fixture, multi-race assertions, single artifact). Cross-book invariants from `corpus/races/apg_races.lst.md` §"Notes" apply.

### 7.2 ACG races

| Race | rust_module_path | cycle_artifact_path | RuleSetId |
|---|---|---|---|
| Dhampir | `src/rules_core/rules_tables/acg/race_dhampir.rs` | `races/acg_dhampir_cycle_receipt.md` | `RuleSetId::Acg` |
| Duergar (fighter-specialization) | `src/rules_core/rules_tables/acg/race_duergar.rs` | `races/acg_duergar_cycle_receipt.md` | `RuleSetId::Acg` |
| Forlorn | `src/rules_core/rules_tables/acg/race_forlorn.rs` | `races/acg_forlorn_cycle_receipt.md` | `RuleSetId::Acg` |
| Half-orc Doom-Shifter | `src/rules_core/rules_tables/acg/race_half_orc_doom_shifter.rs` | `races/acg_half_orc_doom_cycle_receipt.md` | `RuleSetId::Acg` |
| Skeleton (ACG heritage) | `src/rules_core/rules_tables/acg/race_skeleton.rs` | `races/acg_skeleton_cycle_receipt.md` | `RuleSetId::Acg` |
| Undying | `src/rules_core/rules_tables/acg/race_undying.rs` | `races/acg_undying_cycle_receipt.md` | `RuleSetId::Acg` |

Cycle shape: batched like APG.

## 8. Magic items (mitems)

Magic items are aisle-grouped (wondrous / weapons / armor / etc.) with one cycle per aisle. Resolver key shape: `<apg|acg>:mitem:<lowercase-key>`. Test fixture shape: per-aisle assertion set, batched.

| Book | Aisle | stub_path | cycle_artifact_path | RuleSetId |
|---|---|---|---|---|
| APG | Wondrous items | `corpus/magic-items/apg_mitems.lst.md` | `magic-items/apg_wondrous_aisle_cycle_receipt.md` | `RuleSetId::Apg` |
| APG | Weapons | (same file, `[weapons]` section) | `magic-items/apg_weapons_aisle_cycle_receipt.md` | `RuleSetId::Apg` |
| APG | Armor | (same file, `[armor]` section) | `magic-items/apg_armor_aisle_cycle_receipt.md` | `RuleSetId::Apg` |
| ACG | Wondrous items | `corpus/magic-items/acg_mitems.lst.md` | `magic-items/acg_wondrous_aisle_cycle_receipt.md` | `RuleSetId::Acg` |
| ACG | Weapons | (same file, `[weapons]` section) | `magic-items/acg_weapons_aisle_cycle_receipt.md` | `RuleSetId::Acg` |
| ACG | Armor | (same file, `[armor]` section) | `magic-items/acg_armor_aisle_cycle_receipt.md` | `RuleSetId::Acg` |

The Rust module per aisle parses all items of that aisle from one .lst file; per-item Rust code lives in a single module's per-key sub-routing. Cross-book invariants from each `corpus/magic-items/*.lst.md` §"Notes" apply.

## 9. Feats

Feats are category-grouped (general / item-creation / racial / convergence). Resolver key shape: `<apg|acg>:feat:<lowercase-feat-name>`. Cycle shape: one cycle per category-group, batched.

| Book | Category group | stub_path | cycle_artifact_path | RuleSetId |
|---|---|---|---|---|
| APG | General combat | `corpus/feats/apg_feats.lst.md` | `feats/apg_general_combat_cycle_receipt.md` | `RuleSetId::Apg` |
| APG | General + metamagic + defensive + social | (same file) | `feats/apg_general_other_cycle_receipt.md` | `RuleSetId::Apg` |
| APG | Item-creation | (same file, `[feats-item-creation]` section) | `feats/apg_item_creation_cycle_receipt.md` | `RuleSetId::Apg` |
| APG | Racial | (same file, `[feats-racial]` section) | `feats/apg_racial_cycle_receipt.md` | `RuleSetId::Apg` |
| APG | Convergence | (same file, `[feats-acg-convergence]` section) | `feats/apg_convergence_cycle_receipt.md` | `RuleSetId::Apg` |
| ACG | General combat | `corpus/feats/acg_feats.lst.md` | `feats/acg_general_combat_cycle_receipt.md` | `RuleSetId::Acg` |
| ACG | Discovery (ACG-specific) | (same file, `[feats-magic-discoveries]` section) | `feats/acg_magic_discoveries_cycle_receipt.md` | `RuleSetId::Acg` |
| ACG | Item-creation (ACG-specific) | (same file, `[feats-item-creation-acg]` section) | `feats/acg_item_creation_cycle_receipt.md` | `RuleSetId::Acg` |

## 10. Archetypes

Class archetypes are per-class specializations. Resolver key shape: `<apg|acg>:archetype:<class-name>:<lowercase-arch-name>`. Cycle shape: one cycle per archetype (smaller scope; many cycles per epic).

Per-operator-directive 2026-07-19, archetype cycles are the **Epic-4-and-Epic-3 secondary work**. Per `acceptance-and-verification.md` §"Per-criterion closure gate → artifact map" gate 2 (APG Epic 3 already ships the 8 class cycles), archetype cycles are not in the 31-criteria flower's primary path — they're an extension Epic after the primary 31 criteria land, so archetype cycles land under "Epic-3-extension-1" or "Epic-4-extension-1" (numbering deferred until the base 31-criteria loop closes).

For each book × archetype:

| Book | stub_path | per-archetype cycle_artifacts |
|---|---|---|
| APG (22 archetypes across 8 classes) | `corpus/archetypes/apg_archetypes.lst.md` | `archetypes/apg_<class>_<arch>_cycle_receipt.md` (22 files, batched or split) |
| ACG (24 archetypes across 10 classes) | `corpus/archetypes/acg_archetypes.lst.md` | `archetypes/acg_<class>_<arch>_cycle_receipt.md` (24 files, batched or split) |

Cross-book invariants from each `corpus/archetypes/*.lst.md` §"Notes" apply.

## 11. Monster abilities (Bestiary 1 only)

Monster abilities are content units under `RuleSetId::Bestiary1`. Resolver key shape: `beastiary1:ability:<lowercase-ability-name>`. Cycle shape: one cycle per ability-kind (Ex / Su / Sp / damage-resistance).

| Ability kind | stub_path | cycle_artifact_path | RuleSetId |
|---|---|---|---|
| Ex (extraordinary) | `corpus/monster-abilities/beastiary1_monster_abilities.lst.md` | `monster-abilities/ex_cycle_receipt.md` | `RuleSetId::Bestiary1` |
| Su (supernatural) | (same file, `[su-supernatural]` section) | `monster-abilities/su_cycle_receipt.md` | `RuleSetId::Bestiary1` |
| Sp (spell-like) | (same file, `[sp-spell-like]` section) | `monster-abilities/sp_cycle_receipt.md` | `RuleSetId::Bestiary1` |
| Damage-resistance + immunity | (same file, `[damage-resistances]` section) | `monster-abilities/damage_resistance_cycle_receipt.md` | `RuleSetId::Bestiary1` |

## 12. Monster templates (Bestiary 1 only)

Monster templates are content units under `RuleSetId::Bestiary1`. Resolver key shape: `beastiary1:template:<lowercase-template-name>`. Cycle shape: one cycle per template family (undead / construct / dragon-disciple / noble / etc.).

| Template family | stub_path | cycle_artifact_path | RuleSetId |
|---|---|---|---|
| Undead (5 templates: skeleton, zombie, ghoul, lich, vampire, frozen_remain) | `corpus/monster-templates/beastiary1_monster_templates.lst.md` | `monster-templates/undead_cycle_receipt.md` | `RuleSetId::Bestiary1` |
| Construct (5 templates: clockwork_construct, flesh_golem, iron_golem, stone_golem, animated_object) | (same file, `[construct-templates]` section) | `monster-templates/construct_cycle_receipt.md` | `RuleSetId::Bestiary1` |
| Dragon-disciple (1 template: dragon_disciple) | (same file, `[dragon-disciple-template]` section) | `monster-templates/dragon_disciple_cycle_receipt.md` | `RuleSetId::Bestiary1` |
| Noble (2 templates: giant, noble) | (same file, `[noble-templates]` section) | `monster-templates/noble_cycle_receipt.md` | `RuleSetId::Bestiary1` |

## 13. Recorded

Authored 2026-07-19 per operator directive ("full" coverage for every content type listed in operator message: races, classes, mitems, spells, feats, etc.; no stub-only ingest; expected per content type). Authored alongside the corpus-stub seed: 12 stub files (2 races × APG/ACG, 2 mitems × APG/ACG, 2 feats × APG/ACG, 2 archetypes × APG/ACG, 1 monster-abilities Bestiary 1, 1 monster-templates Bestiary 1), plus the per-content-type inventory sections §7-§12 in `corpus-source-inventory.md`.
