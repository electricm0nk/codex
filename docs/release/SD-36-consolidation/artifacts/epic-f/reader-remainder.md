# SD-36 Epic F1 -- proficiency-reader remainder

The named remainder of `every_census_class_has_a_known_proficiency_answer`
(`src/rules_core/rules_tables/crb/weapon_tables.rs`). That test walks every class in
`codex::rules_core::class_census::census()` (135) that has no static `CLASS_WEAPON_PROFICIENCIES`
row, asks the converted-record reader (`class_proficiency_sheet_rules::class_weapon_proficiency_view`)
at every level `1..=max_level`, and passes only when the set of classes answering Unknown at any
level equals the `| class:` rows of the table below -- no silent tolerance in either direction. A
class that gains an answer must leave this table in the same commit; a class that loses one fails
the test by name.

Command: `cargo test --locked -j 8 --lib every_census_class_has_a_known_proficiency_answer -- --nocapture`
(prints the population line and every Unknown class with the reader's reason). Measured 2026-09-23
on sd36/epic-f1c after F1c-3 (Unchained class records, closure-complete attestation, weapon-choice
pools linked; package regenerated with `sheet_rule_convert -- --write`, `--check` exit 0).

## Measured

- Census classes: **135** (61 non-prestige + 74 prestige). With a static row: **42** (all non-prestige).
  Walked by the reader: **93** (19 non-prestige + 74 prestige).
- Known at every level: **76 of 93** (19 of the 19 non-prestige; 57 of the 74 prestige).
- Unknown: **17 of 93** -- **0 non-prestige** and **17 prestige** (prestige classes are Blocked alone
  until F2 regardless -- census `prestige_alone_blocked` = 74 of 74).
- F1c-1 (2026-09-22) closed mechanism A (grant-by-type): 65 -> 60 Unknown.
- F1c-2 (2026-09-23, D2: a line's condition gates only its own line) did not move this table; it
  moved the static-row parity (27 -> 32 of 42 reproduced at level 1).
- F1c-3 (2026-09-23) moved it 60 -> 17:
  - **D4, closure-complete attestation** (`SheetRule::closure_complete`, written by
    `crates/codex-ingest/src/pcgen_import/sheet_rule/attest.rs`): true iff every rule the class line
    reaches through `Granter::Class` / `Granter::Rule` edges converted with zero closure defects
    (`unresolved-references`, `ambiguous-parent-category-target`, `grant-by-type`,
    `undefined-variables`, `unrecognized-proficiency-tag`, or a grant onto an unconverted target).
    Attested: **56 of 189** class principals (`python3 -c` over `data/sheet_rules/*/class/*.json`,
    first rule's `closure_complete`). The reader answers Known(empty) only when the attestation holds
    AND no rule the class line reaches at or below the level leads to a weapon grant.
  - **A product-identity class's own rows** (found while checking the attestation for soundness):
    21 of 189 class records ship a codex-named placeholder corpus key, and the converter keyed their
    closure and class id on it -- their `CLASS:<name>` continuation rows (prerequisites, class
    skills, the header `ABILITY:` grants) were silently dropped, and their level-line grants hung off
    a class id no seed names. Both now key on the class's own row / unit slug. Golden Legionnaire
    and Red Mantis Assassin (mechanism C before) read Simple + Martial from their header grant; the
    newly read rows name 38 references the corpus does not carry, now `unresolved-references`
    rows (7,469 -> 7,503: +38 read for the first time, -4 `PREABILITY` `[<key>]` exclusion rows now
    converted as exclusions), and Red Mantis Assassin's `Class Feature|RMA Weapon Proficiencies`
    is an `ambiguous-parent-category-target` row in both printings (28 -> 30).
  - **D6, commoner**: its one-simple-weapon pick is linked at ingest to the Simple-tier weapon list
    its pool's one member offers (`pool_link.rs`); the reader carries it as a weapon pick decided by
    the character's recorded choice (mechanism F closed).
  - Mechanisms B, C and D of the previous table (lion_blade, golden_legionnaire,
    red_mantis_assassin, divine_scion, exalted) re-traced against the pinned oracle: Lion Blade's
    `inner_sea_intrigue` class lines (`isi_classes.lst:43-63`) grant no proficiency record (the
    `~ Weapon and Armor Proficiency` row is `teog_classes.lst`'s, a book outside the population);
    Divine Scion's row states "gains no additional weapon or armor proficiencies"
    (`ism_abilities_class.lst:29`): both read Known(empty) under the attestation.
- Evidence for each mechanism: the converted package's own `granted_by` edges and
  `data/sheet_rules/_defects/*.json` restricted to the class's closure (test-side and converter-side
  only; live code never reads `_defects/`). Re-derive with
  `python3 <scratch>/closure_defects.py <slug>...` (same walk as `attest.rs`).

## Mechanisms

| code | mechanism | classes | detail | closes in |
|---|---|---|---|---|
| A | grant-by-type not converted | 0 | CLOSED by F1c-1 | converter -- done |
| B | proficiency record not converted | 0 | re-traced in F1c-3: Lion Blade's own class lines grant none | -- |
| C | proficiency record not linked to the class | 0 | CLOSED by F1c-3: the product-identity class's header `ABILITY:` row is read | converter -- done |
| D | proficiency record converted without its grant | 0 | re-traced in F1c-3: Divine Scion's row grants none; Exalted's states it in DESC only (now G) | -- |
| E | no weapon grant anywhere; attestation missing | 0 | CLOSED by F1c-3 (D4): 54 of 54 were attested or re-classified G/H | converter -- done |
| F | proficiency pick into an unlinked pool | 0 | CLOSED by F1c-3 (D6) | converter -- done |
| G | attestation false: an unresolved reference in the class closure | 13 | the class closure names a record the corpus does not carry (`_defects/unresolved-references.json`), so the converter cannot attest that no weapon grant was lost | corpus: ingest the named records, or the converter: resolve the reference |
| H | attestation false: an undefined variable in the class closure | 4 | a rule in the closure reads a variable no oracle row defines (`_defects/undefined-variables.json`) | converter: variable declaration repair |

Open outside this table (Known, not Unknown, so the test does not list it): Red Mantis Assassin
reads Simple + Martial from its header grant, while its closure also carries the ambiguous
`Class Feature|RMA Weapon Proficiencies` reference (two converted records answer it,
`inner_sea_world_guide` and `adventurers_guide`); the reader's contract counts what the closure
grants and does not yet turn a Known answer Unknown on an unresolved reference whose oracle target
carries a weapon grant (spec §3.4's full `closure_complete` wording). Mechanism: ambiguous
parent-category target in a Known closure; closes in the converter's ambiguity rule.

Every mechanism closes in the converter or the corpus + a package regeneration. None is a per-class
special case in live code, and none is closed by a new Rust row.

## Remainder (the test reads the `| class:` rows)

| class | family | mechanism | evidence |
|---|---|---|---|
| class:aldori_swordlord | prestige | G: attestation false, unresolved reference in closure | 10 `unresolved-references` rows on the class record itself: its level-line `Special Ability\|Aldori Swordlord ~ <feature>` grants (e.g. `~ Adaptive Tactics`) name records the corpus does not carry; read since F1c-3 (continuation rows of a product-identity class) |
| class:bellflower_tiller | prestige | G: attestation false, unresolved reference in closure | `unresolved-references`: `Special Ability\|Bellflower Tiller ~ Bellflower Crop` on the class record |
| class:cyphermage | prestige | G: attestation false, unresolved reference in closure | two books declare the class (`adventurers_guide`, `inner_sea_magic`) under one class id, so the closure is their union; `inner_sea_magic:class:cyphermage` carries `unresolved-references` `Cyphermage Class Feature\|Cyphermage ~ Cypher Lore` |
| class:diabolist | prestige | G: attestation false, unresolved reference in closure | `unresolved-references`: `diabolist_imp_companion` names `Special Ability\|Hunter's Bond ~ Companion` |
| class:dragon_disciple | prestige | G: attestation false, unresolved reference in closure | `unresolved-references`: `dragon_disciple_dragon_bite` names `Internal\|Bite` |
| class:exalted | prestige | G: attestation false, unresolved reference in closure | `unresolved-references` `FEAT\|skill focus (knowledge (religion))` on the class record, and 2 `undefined-variables` (`IsProfane`, `IsSacred`) on `exalted_vitality`. Note: the oracle row `Exalted ~ Weapon and Armor Proficiency` (`isg_abilities.lst:92`) states the deity's favored weapon in its DESC only, with no grant token |
| class:harrower | prestige | G: attestation false, unresolved reference in closure | 2 `unresolved-references` on the class record: `Special Ability\|Blessing of the Harrow`, `\|Harrow Casting` |
| class:hellknight | prestige | G: attestation false, unresolved reference in closure | `unresolved-references`: `Special Ability\|Hellknight Armor ~ HK` on the class record (`adventurers_guide` and `inner_sea_world_guide` both) |
| class:hellknight_signifer | prestige | G: attestation false, unresolved reference in closure | `unresolved-references` `Special Ability\|Hellknight Signifer ~ Signifer Armor Training` on the class record; `undefined-variables` `CasterLevel_Highest` on `arcane_armor_mastery` |
| class:loremaster | prestige | H: attestation false, undefined variable in closure | `undefined-variables`: `SecretLore` on `loremaster_secret_lore` |
| class:magaambyan_arcanist | prestige | G: attestation false, unresolved reference in closure | 11 `unresolved-references` on the class record: its level-line `Special Ability\|Magaambyan Arcanist ~ <feature>` grants |
| class:metaforge | prestige | H: attestation false, undefined variable in closure | `undefined-variables`: `MetaforgedLVL` on `metaforge_crystallized_mind_blade` |
| class:mystic_theurge | prestige | H: attestation false, undefined variable in closure | `undefined-variables`: `MysticTheurgeLVL` on `core_rulebook:class_feature:mystic_theurge` |
| class:pathfinder_delver | prestige | H: attestation false, undefined variable in closure | `undefined-variables`: `PaDTrueSeeingLvl` on `pathfinder_delver_true_seeing` |
| class:rivethun_emissary | prestige | G: attestation false, unresolved reference in closure | `unresolved-references`: `FEAT\|Spirit Beacon` on the class record |
| class:sanguine_angel | prestige | G: attestation false, unresolved reference in closure | 4 `unresolved-references` on the class record (`Special Ability\|Sanguine Angel ~ <feature>`) |
| class:steel_falcon | prestige | G: attestation false, unresolved reference in closure | 2 `unresolved-references` on the class record: `Special Ability\|Steel Falcon ~ Talmandor's Blessing`, `~ Talmandor's Fury` |
