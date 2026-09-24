# SD-36 Epic F1 -- proficiency-reader remainder

The named remainder of `every_census_class_has_a_known_proficiency_answer`
(`src/rules_core/rules_tables/crb/weapon_tables.rs`). That test walks every class in
`codex::rules_core::class_census::census()` (137 since F2a) that has no static
`CLASS_WEAPON_PROFICIENCIES` row, asks the converted-record reader
(`class_proficiency_sheet_rules::class_weapon_proficiency_view`) at every level `1..=max_level`, and
passes only when the set of classes answering Unknown at any level equals the `| class:` rows of the
table below -- no silent tolerance in either direction. A class that gains an answer must leave this
table in the same commit; a class that loses one fails the test by name.

Command: `cargo test --locked -j 8 --lib every_census_class_has_a_known_proficiency_answer -- --test-threads=8 --nocapture`
(prints the population line and every Unknown class with the reader's reason). Regenerated
2026-09-24 on sd36/epic-f2-f3 after the F3b2 converter step (placeholder-keyed records indexed under
their declared KEY; package `--check` exit 0, records 49,450). Result: 1 passed;
`census classes: 137; with a static row: 42; walked by the reader: 95; Known at every level: 85;
Unknown: 10`.

Per-class evidence is re-derived from the committed package, read-only, with
`python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/closure_defects.py <slug>...`
(the same walk as `attest.rs`; it prints each closure-defect row attributed to a record in the
class's closure). Run over the 10 classes below it lists 12 rows: 5 `unresolved-references` and
7 `undefined-variables` -- see the count table under Mechanisms. Run over the 17 classes of the
F1c-3 table it listed 45 rows; F3b2 closed 33 of them (every one a reference to a product-identity
record, see Measured).

## Measured

- Census classes: **137** (63 non-prestige + 74 prestige; F2a added the two APG Ex-* ids). With a
  static row: **42** (all non-prestige). Walked by the reader: **95** (21 non-prestige + 74 prestige).
- Known at every level: **85 of 95** (21 of the 21 non-prestige; 64 of the 74 prestige).
- Unknown: **10 of 95** -- **0 non-prestige** and **10 prestige** (a prestige class alone is Blocked
  regardless -- census `prestige_alone_blocked` = 74 of 74).
- **F3b2 (2026-09-24) moved it 17 -> 10.** Mechanism closed: *a reference to a product-identity
  record missed because the record was indexed only under its codex-named placeholder corpus key*
  (`Codex-Named Unit (class_feature_adventurers_guide_ag_abilities_class_lst_9)`), not the KEY its
  own oracle row declares (`Aldori Swordlord ~ Adaptive Tactics`, `ag_abilities_class.lst:9`). The
  converter (`sheet_rule/mod.rs` `build_index`) now also indexes a placeholder-keyed record under
  its row's declared KEY, after every corpus key and only where no corpus key already answers the
  pair (a reprint's real key keeps its target). Package-wide: `_defects/unresolved-references.json`
  7,503 -> 7,120 (383 rows resolved, 0 added, 0 of them parenthesised); 73 rule fields go
  `MissingRule` -> `Rule`; 310 `granted_by` edges added; `closure_complete` attested on 8 more class
  principals (56 -> 64 of 189). Classes that left this table, each now Known with no weapon grant in
  its converted closure (the pinned oracle's class rows grant none): aldori_swordlord (10 rows
  closed), bellflower_tiller (1), harrower (2), hellknight (2, both printings), magaambyan_arcanist
  (11), sanguine_angel (4), steel_falcon (2). hellknight_signifer's reference row also closed (1);
  it stays, on its undefined variable.
- F1c-1 (2026-09-22) closed mechanism A (grant-by-type): 65 -> 60 Unknown.
- F1c-2 (2026-09-23, D2: a line's condition gates only its own line) did not move this table; it
  moved the static-row parity (27 -> 32 of 42 reproduced at level 1).
- **Non-prestige remainder: empty** (0 of 21 walked non-prestige classes Unknown; with the 42
  static rows, which reader-vs-static test (a) reproduces 42 of 42 at level 1, all 63 non-prestige
  classes have a proficiency answer -- census `computed` 63 of 63, `blocked` 0,
  `artifacts/epic-f/census-f3b.json`; 61 of 61 at F1c, `census-f1c.json`).
- The D5 stale-row correction, F1c-4 (D7, always-held globals) and F1c-5 (D8, variable-pool picks),
  all after F1c-3, did not move this table (17 before and after). They moved static-row parity: D5 to
  41 of 42 (af70b72679), D7 none (ea4d64eca8), D8 41 -> 42 of 42 (641691e283; summoner, which has a
  static row and so is not walked here).
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
  `python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/closure_defects.py <slug>...`
  (same walk as `attest.rs`; committed with this regeneration -- the earlier scratch copy was lost).

## Mechanisms

| code | mechanism | classes | detail | closes in |
|---|---|---|---|---|
| A | grant-by-type not converted | 0 | CLOSED by F1c-1 | converter -- done |
| B | proficiency record not converted | 0 | re-traced in F1c-3: Lion Blade's own class lines grant none | -- |
| C | proficiency record not linked to the class | 0 | CLOSED by F1c-3: the product-identity class's header `ABILITY:` row is read | converter -- done |
| D | proficiency record converted without its grant | 0 | re-traced in F1c-3: Divine Scion's row grants none; Exalted's states it in DESC only (now G) | -- |
| E | no weapon grant anywhere; attestation missing | 0 | CLOSED by F1c-3 (D4) | converter -- done |
| F | proficiency pick into an unlinked pool | 0 | CLOSED by F1c-3 (D6) | converter -- done |
| G | attestation false: an unresolved reference in the class closure | 5 | the closure names a record the converter cannot resolve; sub-mechanism per class below (G-P, a placeholder-keyed target, CLOSED by F3b2: 7 classes left) | per row, below |
| H | attestation false: an undefined variable in the class closure | 5 | a rule in the closure reads a variable NO row of the pinned oracle declares (`_defects/undefined-variables.json`); PCGen itself reads such a name as 0 (`PlayerCharacter.getVariable`: no `VariableKey`, so `getVariableValue` of the bare name, bonuses excluded) | the oracle data (declare the variable); not a converter defect |

G sub-mechanisms (F3b2 re-trace against the pinned oracle, `7f818006e3`):

- **G-P, placeholder-keyed target** -- CLOSED by F3b2 (33 rows, 7 classes).
- **G-T, twin printing + dropped category parent** (cyphermage): `Cyphermage Class Feature|Cyphermage ~ Cypher Lore`
  (`ism_classes.lst:79`). Two units declare the target (`adventurers_guide` and `inner_sea_magic`
  `class_feature:cyphermage_cypher_lore`), so `(Special Ability, CYPHERMAGE ~ CYPHER LORE)` is
  ambiguous by the F1 finding-4 rule (never guess). The child category is also dropped from the
  parent map, because its two declarations (`ism_abilitycategories.lst:56`, `ag_abilitycategories.lst:7`)
  agree on the parent (`Special Ability`) but not on `TYPE`, and `closure.rs` removes the parent for
  a TYPE disagreement. Repairing that alone would turn this row into an
  `ambiguous-parent-category-target` row, still a closure defect. Closes in: a ruling on which of two
  printings a same-book reference names (a converter rule), not taken here.
- **G-U, target declared nowhere in the pinned tree** (diabolist): `Special Ability|Hunter's Bond ~ Companion`
  on `diabolist_imp_companion`. No row of the pinned tree declares that KEY (grep over
  `data/pathfinder`: only `PREABILITY` references, `uw_feats.lst:65`). Closes in: the oracle data.
- **G-N, target declared but not ingested** (dragon_disciple): `Internal|Bite` on
  `dragon_disciple_dragon_bite`. The row exists (`core_essentials/ce_abilities_race.lst:249`,
  `CATEGORY:Internal`, carrying `BONUS:WEAPONPROF=Bite|TOHIT|-5`) but no inventory unit stands for
  it (frozen population 49,450). Closes in: the corpus (ingest the record).
- **G-O, option dropped** (exalted): `FEAT|skill focus (knowledge (religion))`. The parameter split
  splits at the last ` (` and misses a nested option. Splitting at the balanced group was measured
  (F3b2 first pass) and reverted: `Holdable` carries no option, so the reference would hold
  `Skill Focus` with ANY option. Measured effect: `_vars/v3ecc4923bf829539` ("Dwarven Waraxe Exotic
  Use") would read `Holds(exotic_weapon_proficiency)` for any Exotic Weapon Proficiency -- a wrong
  computed number. Exalted also carries 2 H rows (`IsProfane`, `IsSacred`: declared only on a
  commented-out row, `isg_abilities.lst:85` `#DEFINE:IsSacred|0 DEFINE:IsProfane|0`). Closes in: an
  option-carrying `Holdable` (schema) plus the oracle data.
- **G-K, a key no record declares** (rivethun_emissary): `FEAT|Spirit Beacon` in the class's
  `PREABILITY` (`ag_classes.lst:362`). The oracle declares only `Spirit Beacon (Fey)`,
  `(Undead)`, `(Outsiders)` (`ag_feats.lst:60-62`), and PCGen's own `PREABILITY` test compares the
  exact key (`PrerequisiteUtilities.passesAbilityTest`), so the oracle's prerequisite can never pass
  in PCGen either. Closes in: the oracle data.

Counts (`closure_defects.py` over the 10 classes; a class is G when its closure carries any
unresolved reference, H when it carries only undefined variables):

| mechanism | classes | defect rows in their closures | of which `unresolved-references` | of which `undefined-variables` |
|---|---|---|---|---|
| G | 5 | 7 | 5 | 2 (exalted) |
| H | 5 | 5 | 0 | 5 |
| total | 10 prestige, 0 non-prestige | 12 | 5 | 7 |

Attested `closure_complete`: 64 of 189 class principals (56 before F3b2).

Open outside this table (Known, not Unknown, so the test does not list it): Red Mantis Assassin
reads Simple + Martial from its header grant, while its closure also carries the ambiguous
`Class Feature|RMA Weapon Proficiencies` reference (two converted records answer it,
`inner_sea_world_guide` and `adventurers_guide`); the reader's contract counts what the closure
grants and does not yet turn a Known answer Unknown on an unresolved reference whose oracle target
carries a weapon grant (spec §3.4's full `closure_complete` wording). Mechanism: ambiguous
parent-category target in a Known closure; closes in the converter's ambiguity rule.

None of the remainder is closed by a per-class special case in live code or a new Rust row.

## Remainder (the test reads the `| class:` rows)

| class | family | mechanism | evidence |
|---|---|---|---|
| class:cyphermage | prestige | G-T: twin printing + dropped category parent | `unresolved-references` `inner_sea_magic:class:cyphermage: Cyphermage Class Feature\|Cyphermage ~ Cypher Lore`; two books declare the class under one id, so the closure is their union |
| class:diabolist | prestige | G-U: target declared nowhere in the pinned tree | `unresolved-references`: `diabolist_imp_companion` names `Special Ability\|Hunter's Bond ~ Companion` |
| class:dragon_disciple | prestige | G-N: target declared but not ingested | `unresolved-references`: `dragon_disciple_dragon_bite` names `Internal\|Bite` (`ce_abilities_race.lst:249`, no inventory unit) |
| class:exalted | prestige | G-O + H: option dropped; variables declared only on a commented row | `unresolved-references` `FEAT\|skill focus (knowledge (religion))` on the class record; `undefined-variables` `IsProfane`, `IsSacred` on `exalted_vitality` |
| class:hellknight_signifer | prestige | H: undefined variable | `undefined-variables` `CasterLevel_Highest` on `core_rulebook:feat:arcane_armor_mastery` (read by `PREVARGTEQ` in 23 oracle files, declared by none); its `Signifer Armor Training` reference closed in F3b2 |
| class:loremaster | prestige | H: undefined variable | `undefined-variables`: `SecretLore` on `loremaster_secret_lore` (`cr_abilities_class.lst:3017`; declared only in `data/3e`, outside the Pathfinder tree) |
| class:metaforge | prestige | H: undefined variable | `undefined-variables`: `MetaforgedLVL` on `metaforge_crystallized_mind_blade` (`up_abilities_class.lst:1248`; the class row `up_classes.lst:812` declares no level variable) |
| class:mystic_theurge | prestige | H: undefined variable | `undefined-variables`: `MysticTheurgeLVL` on `core_rulebook:class_feature:mystic_theurge` (`cr_abilities_class.lst:117`; the class row `cr_classes.lst:448` declares none) |
| class:pathfinder_delver | prestige | H: undefined variable | `undefined-variables`: `PaDTrueSeeingLvl` on `pathfinder_delver_true_seeing` (`ag_abilities_class.lst:387`; the class row declares `PaDLVL` only) |
| class:rivethun_emissary | prestige | G-K: a key no record declares | `unresolved-references`: `FEAT\|Spirit Beacon` on the class record (`ag_classes.lst:362`) |
