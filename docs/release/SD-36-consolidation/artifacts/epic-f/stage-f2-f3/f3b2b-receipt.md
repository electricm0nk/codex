# F3b2b -- converter step 2: undeclared oracle variables; same-object reprints (SD-36 Epic F3)

The second step of the F2/F3 batch that regenerates `data/sheet_rules/**`, under the F1c
structural-diff protocol. `data/corpus/**` and `site/**` are untouched. F3b2
(`f3b2-converter-receipt.md`) left 10 prestige classes Unknown to the proficiency reader. This step
closes two mechanisms and names the rest. No rule names a class.

Code:

- `crates/codex-ingest/src/pcgen_import/sheet_rule/oracle_terms.rs` (new): the oracle's
  built-in-term test.
- `.../sheet_rule/ctx.rs`: `resolve_variable`, and the ambiguous-pair lookup in
  `resolve_rule_in_checked`.
- `.../sheet_rule/reprint.rs` (new): the supersession rule.
- `.../sheet_rule/mod.rs`: `build_index` records ambiguous candidates and applies the rule.
- `.../sheet_rule/closure.rs`: `PinnedTree::source_dates`, and a `TYPE:`-only disagreement no
  longer drops a child category's parent.
- `src/rules_core/sheet_rule.rs`: `Provenance::undeclared_in_pinned_tree`, serde-default and
  skipped when empty.

Every "before" figure is the F3b2 package (`b1fdc45fca`). The structural diff baseline is tranche/16
(`cc21cac195`). Both were extracted read-only with `git archive` into scratch.

## 1. Undeclared variables (mechanism H)

**The oracle's semantics** (pinned checkout `~/workspace/repos/pcgen`, `7f818006e3`). PCGen resolves
a formula term in three steps, in `VariableProcessor.lookupVariable`
(`code/src/java/pcgen/core/VariableProcessor.java:532-561`):

1. A data variable, which is `pc.hasVariable`.
2. An internal term, through `EvaluatorFactory.getTermEvaluator`. This is one regex anchored at the
   start of the term (`EvaluatorFactory.java:54`, `"^("`). It is compiled with no flags, so it is
   case-sensitive (`:78`). It is built from every `TermEvaluatorBuilderPCVar`/`EQVar` pattern plus
   the stat keys. An entire-term builder must match the whole term; the others match a prefix
   (`:123-133`).
3. An output token (`ExportHandler.java:1576-1645`). The text before the first `.` or `,` is looked
   up exactly in `TOKEN_MAP`, whose names are all upper case.

When none of the three answers, the term's text is parsed as a number, and a failure is "just zero"
(`VariableProcessor.java:394-402`: `valFloat = 0.0f; ... // Don't care, as it's just zero`).

**The rule.** `oracle_terms::undeclared_reads_as_zero` holds for a name that meets all of these:

- no row of the pinned tree declares it;
- it is one plain identifier;
- its first `.`/`,` segment has a lower-case letter, so no output token and no entire-term builder
  can match it;
- it starts with none of the prefix builders' literals (`BL`, `CL`, `COUNT[`, `EQTYPE`,
  `HASDEITY:`, `HASFEAT:`, `MODEQUIP`, `MOVE[`, `PC.`, `SKILLRANK`, `SKILLTOTAL`, `VARDEFINED:`,
  `WEIGHT.`, and the six stat keys).

Such a name evaluates as 0 in the oracle, so the converter's `Const(0)` is the oracle's value. The
reference is then:

- an informational `_defects/undeclared-in-pinned-tree.json` row;
- a `provenance.undeclared_in_pinned_tree` note on the record's rules;
- not a closure defect. `attest.rs`'s `CLOSURE_DEFECT_KINDS` is unchanged, and the new kind is not
  in it.

Anything else keeps the `undefined-variables` closure defect. That covers a possible built-in term,
and a token that is not one identifier (`RagePowersLVL%2`: the lexer did not split the formula).

**Checked against the brief's five names.** None is a built-in term, and none is declared by a
Pathfinder row. A grep of `data/pathfinder` for `DEFINE:<name>` finds 0 hits for each.

| name | first segment has lower case | prefix builder | declared in the pinned tree |
|---|---|---|---|
| `CasterLevel_Highest` | yes | none (`Ca`, not `CL`; `CASTERLEVEL` is an entire-term builder and case-sensitive) | no |
| `SecretLore` | yes | none | no (only `data/3e/.../srd_abilities_prestige.lst:59`) |
| `MetaforgedLVL` | yes | none | no |
| `MysticTheurgeLVL` | yes | none | no |
| `PaDTrueSeeingLvl` | yes | none | no |

**Package-wide.** The figures are row counts of `data/sheet_rules/_defects/{undefined-variables,undeclared-in-pinned-tree}.json`
(`python3 -c "import json; print(len(json.load(open('<file>'))))"`), with records and names
counted over each row's `"<record>: <name>"` split.

| figure | F3b2 | F3b2b |
|---|---:|---:|
| `undefined-variables` rows (closure defect) | 737 | **89** |
| `undeclared-in-pinned-tree` rows (informational) | 0 | **648** (325 records, 160 names) |

- **648 of the 737 H-shape rows clear.** Each of them moved: the F3b2 set minus the F3b2b set
  equals the new informational set exactly, and the F3b2b set is a subset of the F3b2 set.
- The 89 that stay, by mechanism:
  - 87 name a possible built-in term or output token: `CRITMULT` 26, `LIST` 18, `SHIELDACCHECK` 14,
    `COST` 4, `ARMORACCHECK` 4, `BASEAB` 3, `SR` 2, `BONUS.COMBAT.AC.NaturalArmor` 2,
    `BASE.Fortitude`, `BASE.Reflex`, `DAMAGE`, `DAMAGE.Melee`, `TOHIT`, `TOTALAB`, `ACHECK`,
    `REACH.VAL`, `PREVARGTEQ`, `CON.`, and 4 `TYPE.Slayer...`.
  - 2 are unsplit formulas (`RagePowersLVL%2`, `SurgeLVL%2`).
- 829 rules carry the provenance note (records plus their `#` siblings).

## 2. Same-object reprints (mechanism T) and the category-parent drop

**The parent drop.** `closure.rs` used one "ambiguous" set for a child category whose declarations
disagree on the parent and for one whose declarations disagree only on `TYPE:`. Both lost the parent.
The two are now separate sets. A `TYPE:` disagreement drops only the member filter
(`ability_category_type`). Cyphermage Class Feature (`ism_abilitycategories.lst:56`,
`ag_abilitycategories.lst:7`) now keeps `Special Ability`.

Package effect: `unresolved-references` goes from 7,120 to **7,119**. Cyphermage's row left, 0 rows
were added, and no other reference moved.

**The supersession rule** (`reprint.rs`; recorded in `decisions.md` §12.1 with the ruling's
citation). An ambiguous pair resolves to its newest printing when every candidate is a printing of
the same object:

- **Kind and book:** the candidates share a kind, and none is in `mythic_adventures` or
  `pathfinder_unchained` (the amendment: the default answer there is "variant").
- **Identity fields:** name, `KEY:` and `CATEGORY:` agree across the candidates.
- **Same object:** every `DESC:` is present and the descriptions are prefix-ordered, or the rows are
  token-identical apart from `SOURCE*`.
- **Date:** exactly one candidate's book carries the latest `.pcc` `SOURCEDATE:`.

**Measured** (`_defects/ambiguous-parent-category-target.json`):

- The population under the repaired parent map is 31 rows: F3b2's 30, plus Cyphermage's, which is
  reachable now that its parent is kept.
- **13 of 31 resolve.**
  - 10 Advanced Race Guide racial-subtype choices. The targets were first printed in Advanced
    Player's Guide (2010-08). The descriptions are equal after normalisation, and they resolve to
    ARG (2012-06).
  - Red Mantis Assassin's `RMA Weapon Proficiencies`, in both class printings. The rows are
    token-identical (`iswg_abilities_class.lst:151`, 2011-03, and `ag_abilities_class.lst:419`,
    2017-06).
  - Cyphermage's `Cyphermage ~ Cypher Lore`. The ISM text (2011-07) is a prefix of the AG text
    (2017-06).
- **18 stay, named:**
  - 17 ARG reprints of APG racial traits whose text was reworded. Mechanism: *reworded reprint --
    identity not provable by the prefix or token test*.
  - 1 same-book double declaration: `Master Of Many Styles ~ Perfect Style` (`uc_abilities_class.lst:1096`
    and `support/uc_abilities_class_ag.lst:80`, both 2011-01). Mechanism: *no newest printing*.
- **Edges added: 11 `granted_by`.** 10 are on `race_trait` and 1 is on the AG `rma_weapon_proficiencies`.
  Cyphermage's resolved grant lands on an edge the Adventurer's Guide class printing already carried.
  Both printings share the class id `cyphermage`, so the edge set is unchanged.

## 3. Named, not closed

- **(3) Option-carrying reference** (exalted). Mechanism: *option-carrying reference needs an
  option-carrying `Holdable`*.
  - Package-wide, **203 of 7,119** `unresolved-references` rows carry a nested parameter. The
    re-derive command is in `forward-scope-register.md` FS-14.
  - F3b2's measurement stands: the naive split made `_vars/v3ecc4923bf829539` ("Dwarven Waraxe
    Exotic Use") hold for any Exotic Weapon Proficiency, a wrong number. It was not re-attempted.
  - It is recorded in `reader-remainder.md` (G-O) and `forward-scope-register.md` (FS-14).
- **(4) Oracle-data gaps.** Each stays Unknown and is named in `reader-remainder.md` with its oracle
  line:
  - diabolist: `Hunter's Bond ~ Companion` is declared nowhere. The only occurrence is a
    `PREABILITY` at `uw_feats.lst:65`.
  - dragon_disciple: `Internal|Bite` is declared at `ce_abilities_race.lst:249`, but no inventory
    unit stands for it.
  - rivethun_emissary: `FEAT|Spirit Beacon` at `ag_classes.lst:362`. The oracle declares only the
    three suffixed keys, `ag_feats.lst:60-62`.

## 4. RED -> GREEN

| stage | command | result | log |
|---|---|---|---|
| RED | `cargo test --locked -j 8 -p codex-ingest --test sheet_rule_convert_gate -- --test-threads=8 an_undeclared a_same_object` | 0 of 2. `ability_category_parent["CYPHERMAGE CLASS FEATURE"]` was `None`, expected `Some("SPECIAL ABILITY")`. Loremaster's `SecretLore` was still an `undefined-variables` row. | `f3b2b-red.log` |
| GREEN | same file, whole, inside the ingest suite (§6) | 43 of 43. The two new tests pass, with the defect rows compared as a set because the package writes each row once. | `f3b2b-verify.log` |
| unit | `cargo test --locked -j 8 -p codex-ingest --lib oracle_terms -- --test-threads=8` | 3 of 3 (the five brief names are not built-in; `CRITMULT`, `CLevel`, `STRBonus`, `DAMAGE.Melee` may be; `RagePowersLVL%2` is not judged) | -- |

A first `--write` put `RagePowersLVL%2` into a provenance note, and `--check` refused it (a
source-format literal, `ultimate_psionics/class_feature/raging_surge_bonus_rage.json`). The rule then
gained its plain-identifier condition, and the rewrite passed `--check`.

## 5. Regeneration protocol (F1c)

| gate | command | result |
|---|---|---|
| write | `cargo run --locked --quiet -j 8 -p codex-ingest --bin sheet_rule_convert -- --write` | exit 0; records 49,450, converted 49,450, refused 0, rules 73,016, var tables 6,205 |
| freshness | `... -- --check` | `verdict=PASS`, exit 0 |
| pins | `python3 .../scripts/f3b2b_delta_pins.py <tranche/16>/data/sheet_rules data/sheet_rules .../scripts/structural_diff_f3b2b_deltas.json <F3b2>/data/sheet_rules` | 0 unexplained. Pinned: `f3b2b_undeclared_note` 829 (the list), `f3b2b_closure_complete` 9 (`true`), `required_added_edges` 11 |
| structural diff | `python3 .../scripts/structural_diff.py data/sheet_rules --baseline <tranche/16>/data/sheet_rules` | `verdict=PASS`, exit 0 (`f3b2b-structural-diff.txt`) |
| planted mutations | 7 single-file mutations of a copy of the fresh package | 7 of 7 FAIL, both controls PASS (`f3b2b-planted-mutations.txt`) |
| diff self-test | `python3 .../scripts/structural_diff_test.py` | 32 of 32 (+1: a pinned note passes only its exact shape) |
| residue | `python3 scripts/pcgen_residue_gate.py --check --closure` | `verdict=PASS`, shipped_scanned 69,720 (+1, the new defect file), hits 0 |
| frozen | `python3 scripts/site/check_frozen_status.py --check` | OK, frozen at 100% (49,450 units) |
| bundle | `node scripts/gen-corpus-bundle.mjs` | `files_copied=14029`; tree unchanged |
| clippy | `cargo clippy --locked -j 8 -p codex-ingest --tests -- -D warnings`; `cargo clippy --locked --tests -j 8 -- -D warnings` | clean, clean |

**New diff guard: added edges are pinned.** The structural diff forbids only removals against
tranche/16, so a first-pass planted mutation that dropped an F3b2b-resolved edge passed. The pins
file now also carries `required_added_edges`: every `granted_by` edge that is in the fresh package
and absent from the F3b2 package. `structural_diff.py` fails on any that is missing (M5 below). F3b2b
pins are checked before F3b2's.

The seven mutations:

- M1: the pinned note gains a name.
- M2: `oracle_pin` moves beside the pinned note.
- M3: `closure_complete` is planted on diabolist.
- M4: a note is planted on power_attack.
- M5: the resolved RMA edge is dropped.
- M6: cyphermage cypher lore's class edges are dropped.
- M7: the records count moves to 49,449.

**Field deltas vs tranche/16:**

- F3b2's pins unchanged: 176 skill ranks, 8 closure_complete, 73 placeholder-key.
- F3b2b: 829 provenance notes and 9 closure_complete.
- 0 unexpected, 0 removed edges, 0 removed grants.
- 321 added `granted_by` edges: F3b2's 310 plus these 11.

## 6. Verify and the moved pins (the 1-failure fixture protocol)

**Suites:**

- Ingest suite, `cargo test --locked -j 8 -p codex-ingest --no-fail-fast -- --test-threads=8`:
  1,753 passed, 2 failed, 43 ignored. Both failures are in `class_records_and_closure_attestation.rs`
  and were stale before this step:
  - `every_census_class_has_a_converted_class_record` pinned 135 census ids. The census has had 137
    since F2a (`028cde4e3f`, the two APG Ex-* ids). Re-pinned to 137.
  - `the_real_package_attests_complete_closures_only` asserted Aldori Swordlord unattested.
    F3b2 (`b1fdc45fca`) resolved its ten placeholder-keyed references and attested it (its receipt,
    §2). Its negative example is now Diabolist, which is still unattested: G-U, the
    `Hunter's Bond ~ Companion` key no oracle row declares.
  - After the re-pin: 5 of 5, attested 73 of 189.
- Root suite, `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8`: 288 test binaries,
  6,339 passed, 0 failed, 27 ignored.
- Desktop, `cargo test --locked -j 8 --no-fail-fast --manifest-path apps/desktop/src-tauri/Cargo.toml -- --test-threads=8`:
  615 of 615.
- Every log excerpt is in `f3b2b-verify.log`.

**Moved pins in this step:**

- `tests/sd36_multiclass_any_class.rs`: 9 of 9.
  - **Wizard 5 / Loremaster 2 now reaches Computed.** Its oracle row is now `blocked_only_by: None`.
    The hand-worked numbers are unchanged (BAB 3, base saves 2/2/6, totals 4/4/7, HP 44, skill
    points 18, all from `f3b-hand-worked.md`). Loremaster "gains no proficiency with any weapon or
    armor" (CRB p.385), and its closure is attested now that `SecretLore` is the oracle's 0.
  - `a_class_with_no_proficiency_answer_cannot_undo_another_class_s_grant` needs a class with no
    answer beside Fighter. It moves from Loremaster (now answered) to Dragon Disciple (G-N, still
    Unknown). The mix is still Computed.
- `every_census_class_has_a_known_proficiency_answer` (reader remainder): 10 remainder rows before,
  4 after. `reader-remainder.md` was regenerated in the same commit.

**Reader remainder**
(`cargo test --locked -j 8 --lib every_census_class_has_a_known_proficiency_answer -- --test-threads=8 --nocapture`):
`census classes: 137; with a static row: 42; walked by the reader: 95; Known at every level: 91; Unknown: 4`.

- **Prestige Unknown: 10 -> 4 of 74.** Non-prestige Unknown stays 0 of 21 walked.
- Left the table:
  - cyphermage: T.
  - hellknight_signifer, loremaster, metaforge, mystic_theurge, pathfinder_delver: H.
  - Each is Known with no weapon grant in its attested closure. A walk of the converted package
    finds 0 weapon-proficiency grants in each closure.
- Remaining:
  - diabolist: G-U.
  - dragon_disciple: G-N.
  - exalted: G-O. Its H rows are gone.
  - rivethun_emissary: G-K.
- Class principals attested `closure_complete`: **73 of 189** (64 before). The 9 new ones are
  cyphermage (both printings), hellknight_signifer, loremaster, metaforge, mystic_theurge,
  pathfinder_delver, psychic_detective and adaptive_warrior.

**Census** (`cargo run --locked -j 8 --bin class_census -- --json <scratch>/census-f3b2b.json`):

| figure | F3b2 | F3b2b |
|---|---:|---:|
| ids | 137 | 137 |
| computed (non-prestige) | 63 of 63 | 63 of 63 |
| blocked | 0 | 0 |
| prestige alone Blocked | 74 of 74 | 74 of 74 |
| prestige carrier mix Computed | 57 of 74 | **59 of 74** (cyphermage via Wizard 5; mystic_theurge via Wizard 5 and Cleric 5) |
| prestige carrier mix Blocked | 6 | 4 (exalted, mammoth_rider, sentinel, ulfen_guard: all `multiclass.save_shape.unrecognized`) |
| prestige carrier mix Unknown (no nameable carrier) | 11 | 11 |
| mix panel Computed | 185 of 185 | 185 of 185 |

`combat.baseline_weapon_proficiency_unknown` now blocks no census mix.
`BASELINE_CENSUS_PRESTIGE_MIX_COMPUTED` is not added; the brief leaves it to F3c.
