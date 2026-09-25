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
2026-09-25 on sd36/epic-f2-f3 after the F3c5 converter step (Internal natural-attack helper rows
convert as `Fact::NaturalAttack` on the granting rule; package `--check` exit 0, records 49,450).
Result: 1 passed; `census classes: 137; with a static row: 42; walked by the reader: 95; Known at
every level: 92; Unknown: 3`.

Per-class evidence is re-derived from the committed package, read-only, with
`python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/closure_defects.py <slug>...`
(the same walk as `attest.rs`; it prints each closure-defect row attributed to a record in the
class's closure). Run over the 3 classes below it lists 3 rows, all `unresolved-references` -- see
the count table under Mechanisms. Run over the 4 classes of the F3b2b table it listed 4 rows; F3c5
closed 1 of them (dragon_disciple's `Internal|Bite`). Run over the 10 classes of the F3b2 table it listed 12 rows; F3b2b
closed 8 of them (7 undeclared-variable rows, 1 twin-printing reference). Run over the 17 classes of
the F1c-3 table it listed 45 rows; F3b2 closed 33 of them.

## Measured

- Census classes: **137** (63 non-prestige + 74 prestige; F2a added the two APG Ex-* ids). With a
  static row: **42** (all non-prestige). Walked by the reader: **95** (21 non-prestige + 74 prestige).
- Known at every level: **92 of 95** (21 of the 21 non-prestige; 71 of the 74 prestige).
- Unknown: **3 of 95** -- **0 non-prestige** and **3 prestige** (a prestige class alone is Blocked
  regardless -- census `prestige_alone_blocked` = 74 of 74).
- **F3c5 (2026-09-25) moved it 4 -> 3.** Mechanism G-N closed without a new record: a
  `CATEGORY:Internal` natural-attack helper row no inventory unit stands for (`Bite`,
  `ce_abilities_race.lst:249`; `ABILITYCATEGORY:Internal VISIBLE:NO EDITABLE:NO`,
  `system/gameModes/Pathfinder/miscinfo.lst:303`) whose whole object carries nothing but one
  natural attack's bookkeeping converts as `Fact::NaturalAttack(<attack>)` on the rule that grants
  it (`sheet_rule/natural_attack.rs`). Package-wide: 771 helper rows no unit stands for, 770
  classified natural-attack-only answering 771 pairs, 0 named; `unresolved-references` 6,932 ->
  6,252 (680 rows, all naming a helper). The class that left this table: dragon_disciple ("Dragon
  disciples gain no proficiency with any weapon or armor", CRB p.380), now Known(empty) with its
  closure attested. `closure_complete` attested on 136 of 189 class principals (135 before).
- **F3b2b (2026-09-24) moved it 10 -> 4.** Two mechanisms closed, neither by class:
  - **H, undeclared variable (5 classes).** PCGen evaluates a formula term that no loaded row
    declares and that is not a built-in term as 0 (`VariableProcessor.java:394-402` in the pinned
    checkout: the term's text fails `Float.parseFloat`, "Don't care, as it's just zero"; built-in
    terms are the case-sensitive, start-anchored `TermEvaluatorBuilderPCVar`/`EQVar` patterns,
    `EvaluatorFactory.java:54,78,123-133`, and upper-case output tokens, `ExportHandler.java:1576-1645`).
    None of `CasterLevel_Highest`, `SecretLore`, `MetaforgedLVL`, `MysticTheurgeLVL`,
    `PaDTrueSeeingLvl`, `IsProfane`, `IsSacred` is built-in or declared by a row of the pinned
    tree, so the converter's `Const(0)` IS the oracle's value, the closure is complete under oracle
    semantics, and the reference is an informational `_defects/undeclared-in-pinned-tree.json` row
    plus a `provenance.undeclared_in_pinned_tree` note (`sheet_rule/oracle_terms.rs`, `ctx.rs`).
    Package-wide: **648 of the 737** `undefined-variables` rows move (325 records, 160 names); the
    89 left are names the oracle might read as a built-in term (`CRITMULT` 26, `LIST` 18,
    `SHIELDACCHECK` 14, ...) or tokens that are not one plain identifier (`RagePowersLVL%2`),
    and stay closure defects.
  - **G-T, twin printing (cyphermage).** Its child category keeps its parent (a `TYPE:`-only
    disagreement no longer drops it), and the standing supersession ruling resolves the target to
    its newest printing (`decisions.md` §12.1; `sheet_rule/reprint.rs`): package-wide 13 of 31
    ambiguous-target rows resolve.
  - Classes that left this table, each now Known with no weapon grant in its attested closure (the
    pinned oracle's class rows grant none): cyphermage, hellknight_signifer, loremaster
    ("gains no proficiency with any weapon or armor", CRB p.385), metaforge, mystic_theurge
    ("gains no proficiency with any weapon or armor", CRB p.387), pathfinder_delver.
    `closure_complete` attested on 9 more class principals (64 -> 73 of 189; cyphermage in both
    printings, psychic_detective and adaptive_warrior besides the five H classes).
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
| G | attestation false: an unresolved reference in the class closure | 3 | the closure names a record the converter cannot resolve; sub-mechanism per class below (G-P CLOSED by F3b2, 7 classes; G-T CLOSED by F3b2b, 1 class; G-N CLOSED by F3c5, 1 class) | per row, below |
| H | attestation false: an undefined variable in the class closure | 0 | CLOSED by F3b2b: an undeclared non-built-in variable is the oracle's 0 (`VariableProcessor.java:394-402`), so it is no longer a closure defect (informational `_defects/undeclared-in-pinned-tree.json`) | converter -- done |

G sub-mechanisms (F3b2 re-trace against the pinned oracle, `7f818006e3`):

- **G-P, placeholder-keyed target** -- CLOSED by F3b2 (33 rows, 7 classes).
- **G-T, twin printing + dropped category parent** (cyphermage) -- CLOSED by F3b2b: the parent is
  kept on a `TYPE:`-only disagreement, and `Cyphermage ~ Cypher Lore` (`ism_abilities_class.lst:8`,
  `SOURCEDATE:2011-07`; `ag_abilities_class.lst:103`, `SOURCEDATE:2017-06`; the older DESC a
  prefix of the newer) resolves to the Adventurer's Guide printing (`decisions.md` §12.1).
- **G-U, target declared nowhere in the pinned tree** (diabolist): `Special Ability|Hunter's Bond ~ Companion`
  on `diabolist_imp_companion`. No row of the pinned tree declares that KEY (grep over
  `data/pathfinder`: only `PREABILITY` references, `uw_feats.lst:65`). Closes in: the oracle data.
- **G-N, target declared but not ingested** (dragon_disciple) -- CLOSED by F3c5: `Internal|Bite`
  on `dragon_disciple_dragon_bite` names a natural-attack helper row
  (`core_essentials/ce_abilities_race.lst:249`) no inventory unit stands for; it now converts as
  `Fact::NaturalAttack("Bite")` on Dragon Bite (`sheet_rule/natural_attack.rs`), with no new record
  (frozen population 49,450).
- **G-O, option dropped** (exalted): `FEAT|skill focus (knowledge (religion))`. The parameter split
  splits at the last ` (` and misses a nested option. Splitting at the balanced group was measured
  (F3b2 first pass) and reverted: `Holdable` carries no option, so the reference would hold
  `Skill Focus` with ANY option. Measured effect: `_vars/v3ecc4923bf829539` ("Dwarven Waraxe Exotic
  Use") would read `Holds(exotic_weapon_proficiency)` for any Exotic Weapon Proficiency -- a wrong
  computed number. Its 2 former H rows (`IsProfane`, `IsSacred`: declared only on a
  commented-out row, `isg_abilities.lst:85`) read as the oracle's 0 since F3b2b. Mechanism:
  *option-carrying reference needs an option-carrying `Holdable`*; package-wide 203 of 7,119
  `unresolved-references` rows carry a nested parameter (`forward-scope-register.md` FS-14, with
  the re-derive command). Closes in: an option-carrying `Holdable` (schema).
- **G-K, a key no record declares** (rivethun_emissary): `FEAT|Spirit Beacon` in the class's
  `PREABILITY` (`ag_classes.lst:362`). The oracle declares only `Spirit Beacon (Fey)`,
  `(Undead)`, `(Outsiders)` (`ag_feats.lst:60-62`), and PCGen's own `PREABILITY` test compares the
  exact key (`PrerequisiteUtilities.passesAbilityTest`), so the oracle's prerequisite can never pass
  in PCGen either. Closes in: the oracle data.

Counts (`closure_defects.py` over the 3 classes):

| mechanism | classes | defect rows in their closures | of which `unresolved-references` | of which `undefined-variables` |
|---|---|---|---|---|
| G | 3 | 3 | 3 | 0 |
| H | 0 | 0 | 0 | 0 |
| total | 3 prestige, 0 non-prestige | 3 | 3 | 0 |

Attested `closure_complete`: 136 of 189 class principals at F3c5 (135 at F3c4b, 73 at F3b2b, 64
before F3b2b, 56 before F3b2).

Red Mantis Assassin (was open outside this table): its `Class Feature|RMA Weapon Proficiencies`
reference, ambiguous between the `inner_sea_world_guide` and `adventurers_guide` printings
(token-identical rows, `iswg_abilities_class.lst:151`, `ag_abilities_class.lst:419`), resolves to the
Adventurer's Guide printing since F3b2b (`decisions.md` §12.1), so its closure now holds the
`Light.Martial` set the oracle grants. Its closure still carries 2 defect rows (`SHIELDACCHECK`, a
possible built-in term, and `Internal|CMB`), so it stays unattested; it reads Known from its header
grant.

None of the remainder is closed by a per-class special case in live code or a new Rust row.

## Remainder (the test reads the `| class:` rows)

| class | family | mechanism | evidence |
|---|---|---|---|
| class:diabolist | prestige | G-U: target declared nowhere in the pinned tree | `unresolved-references`: `diabolist_imp_companion` names `Special Ability\|Hunter's Bond ~ Companion` |
| class:exalted | prestige | G-O: option-carrying reference needs an option-carrying `Holdable` | `unresolved-references` `FEAT\|skill focus (knowledge (religion))` on the class record (`isg_classes.lst:27`, its `PREABILITY`); F3b2 measured the naive split as a wrong number (FS-14) |
| class:rivethun_emissary | prestige | G-K: a key no record declares | `unresolved-references`: `FEAT\|Spirit Beacon` on the class record (`ag_classes.lst:362`) |
