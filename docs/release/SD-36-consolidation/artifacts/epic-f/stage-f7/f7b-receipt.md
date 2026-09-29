# F7b receipt: prose formulas print as the rule's words; out-of-inventory condition lines are not printed

Stage F7, step F7b (worklist item F7-5). Branch `sd36/epic-f7-sheet-visible`, on F7a `c724805053`.
This is a converter step: `data/sheet_rules/**` was regenerated under the structural-diff protocol
(§5). The baseline is a read-only `git archive` of `/home/ubuntu/workspace/repos/codex` `tranche/16`
HEAD `809b95f769`, `data/sheet_rules` only. That package is byte-identical to this branch's package
before F7b. `data/corpus/**` and `site/**` were not touched.

## 1. The defect, on Elowen's sheet

The Spells tab prints a prepared spell's effect text from the Spell Catalog entry
(`spellsTabModel.ts`, `effectText` = the matched entry's `description`). Before F7b, Elowen's
Fireball read (`f7b-red-desktop.log`):

```
If not available to a character: A fireball spell ... deals (min(10,CASTERLEVEL))d6 points of fire damage to every creature within the area.
A fireball spell ... deals 1d6 points of fire damage per caster level [maximum 10d6] to every creature within the area. ...
If requires Fireball from mythic spell (no record in the corpus): Mythic: The damage dealt increases to 1d10 ...
```

That is three wrong lines:

1. **A source formula printed as text.** `(min(10,CASTERLEVEL))d6` is PCGen's formula
   syntax, written into the words of the source's short-form description
   (`cr_spells.lst:233`, `DESC:...|!PRERULE:1,DisplayFullSpell`).
2. **A line no character is ever shown.** That short form is gated to show only when the full
   text is switched off. The converter decides the gate as `Never`. The evaluator never prints
   such a line, but the catalog printed it under "If not available to a character:".
3. **A condition sentence about a record outside the inventory.** The mythic paragraph
   (`ma_spells.lst:116`) is gated on holding the mythic spell "Fireball", which the converted
   package does not hold (`Holdable::MissingRule`, mechanism E).

The full text also printed the source's bracket escape, `[maximum 10d6]`, where the book prints
`(maximum 10d6)`.

## 2. Measurement first (the tranche/16 package)

**Denominator: 73,363 converted rules (49,450 records), 113,601 prose `Text` pieces.**

Shape scan (`scan0`: regexes over every printed `Text` piece and label; strings / rules):

| Shape | Strings | Rules | Reading |
|---|---:|---:|---|
| `CASTERLEVEL` | 2,315 | 1,996 | source variable, always a formula |
| `min(` | 130 | 123 | formula function |
| `max(` | 5 | 5 | formula function |
| `)d<n>` | 58 | 58 | formula dice count |
| `TL`, `VAR`, `if(` | 0 | 0 | none |
| `CL` (word) | 239 | 232 | the book's own abbreviation ("CL 7th", "(CL 12th; concentration +18)"); inside a formula 0 times |
| `HD` (word) | 374 | 252 | the book's own abbreviation ("4 HD of creatures"); inside a formula 0 times |

The converter's rule (§3) read every parenthesised group and function call in those strings.
It found **2,361 formulas in 2,317 strings on 1,997 records**, in 57 distinct shapes. 2,359 of
them name `CASTERLEVEL`, some beside a camel-case source variable (`ConjurationSummonersCharmBonus`,
`ConjurationNaturalistsCharmBonus`). The other two are the misspelling `(CASTERLELVEL)` and
`(ConjurationSummonersCharmBonus+2)`. 1,713 are a bare `(CASTERLEVEL)`, 344 are
`(CASTERLEVEL*N)`, and 54 count dice (`)d<n>`). The
formulas sit mostly in spell stat-block lines (Duration, Target or area) and in the source's
short-form spell descriptions.

A 110-record population was hidden from the scan: records the pre-F7b words rewrite
(`scrub_inline_formula`, `_defects/inline-formula-in-prose.json`) had already turned into words
such as "(a rules variable plus hit dice)".

**The residue gate did not catch any of this.** `pcgen_residue_gate.py` counts token heads
(`BONUS:`, `PRE…:`, `%CHOICE`). None of these strings carries one, so the gate read 0. With
the new class (§6), the gate reads **2,361 hits in 1,997 shipped files, `verdict=FAIL`** on this
package (`f7b-red-residue-gate.log`).

## 3. One rule (converter, `sheet_rule/prose.rs::lower_prose_formulas`)

**Finding a formula** (`formula.rs::is_prose_formula`, `prose.rs::prose_formula_spans`). A
parenthesised group, or a function call, is a formula when all three hold:

- It parses whole under the converter's own formula grammar.
- Every identifier in it is one of: a formula function in call position (`min`, `max`, `floor`,
  `ceil`); a formula leaf spelled the source's way (`CASTERLEVEL`, `CL`, `HD`, `STR`, …); or a
  source variable's own spelling (camel case, or four or more upper-case letters).
- It carries a mark no English aside carries: a function call, a leaf or variable name of four or
  more characters, or an operator beside a short leaf. The operator must be in the source's tight
  spelling (`(HD+2)`) or come right before the group (`1d8+(TL)`).

The book's asides are left alone: `(DC 15)`, `(APG)`, `(CL 12th)`, `Headband (CHA) +4`,
`Dusk Kamadan (CR +1)`, `(DC10+HD)`, `(and/or)` (pinned in
`prose_formula_spans_find_source_formulas_and_leave_the_books_asides`).

**Converting it.** The span goes through the formula-to-`Expr` converter that every `%N`
argument uses (`convert_prose_formula`) and becomes one of the typed pieces the live printers
already render:

- `(min(10,CASTERLEVEL))d6` becomes the new `ProsePiece::DiceCount { count, sides }`.
- `1d8+(min(5,CASTERLEVEL))` becomes `ProsePiece::Dice` with that modifier.
- Anything else becomes `ProsePiece::Slot`.

Each formula is one `inline-formula-in-prose` defect row. A name DEFINEd nowhere that the oracle
provably reads as 0 (`undeclared-in-pinned-tree`) keeps that reading, exactly as a `%N` slot does
(the nine Naturalist summon durations, §5). Any other undefined name refuses. That span is an
`inline-formula-unconverted` row, and the words print without it, never with the token.

**Printing it** (`sheet_rule_catalog.rs::dice_count_words`, and the evaluator). A `Min` with one
constant side is the cap. `term`, `k*term` and `term/k` print as "1d6 per caster level", "2d6 per
caster level" and "1d6 per 2 caster levels". Anything else prints as "a number of dN equal to
<words>". For a character, the evaluator prints final dice (`5d6`). The existing `Slot` and
`Dice` printers are unchanged (words with no character, the number for a character), with one
addition: a `%` right after a slot printed as words prints as the word "percent" (§7).

**Spell text brackets.** In a spell's or psionic power's text the source writes the book's
parentheses as brackets, because a parenthesis there delimits a formula. With the formulas
lowered, `[`/`]` print as `(`/`)` (one `bracket-escape-in-spell-text` row per field). The Core
Rulebook, p.284, prints "(maximum 10d6)". Brackets in any other kind of record are untouched.

**The old words rewrite is replaced.** `scrub_inline_formula` turned an all-upper-case group into
words ("(70 plus caster level) percent", "(a rules variable plus hit dice)"). Its 110 records now
go through the same rule. `(DC10+HD)` is the book's own "DC 10 + HD" run together, so it is no
longer rewritten.

## 4. Out-of-inventory condition lines (`prose.rs::decide_out_of_inventory`)

A prose line's condition that names a record outside the converted inventory
(`Holdable::MissingRule`) is decided the way the evaluator decides it: the record is never held.
The term becomes `Never`, its negation `Always`, and the conjunctions fold.

- If the line's condition is then `Never`, the line is not emitted. This is one
  `prose-line-out-of-inventory` row: **315 lines on 314 records** (274 spells, mostly the mythic
  paragraphs; 25 equipment; 5 feats; 5 class features; 2 equipment modifiers; 2 abilities; 1
  race trait).
- If the condition folds to a smaller condition or to none, the line prints with no
  "(no record in the corpus)" sentence. This is one `prose-condition-out-of-inventory` row:
  **37 lines on 36 records**. An example is the amulet of the blooded's line for a wearer
  without the bloodline.

This is the same reading as F1c's undecided-fact rule: an undecidable term is not a printable
sentence.

**A line decided `Never` is never a condition sentence either** (live catalog,
`sheet_rule_catalog.rs::render`). The population is the source's display switch. The source
writes each spell's description twice, a short form gated `!PRERULE:1,DisplayFullSpell` and the
full text gated `PRERULE:1,DisplayFullSpell` (2,259 and 2,266 source rows; the ability twin
`DisplayFullAbility` has 219 and 214). The converter decides the switch on, so every short form
is `Never`: 2,532 lines on 2,063 records of the tranche/16 package. The sheet evaluator never prints a `Never` line. The
catalog printed each one as "If not available to a character: ...", and that is the line that
carried Fireball's formula.

The rule text now prints once:

- When the record states its description in another line, the short form does not print.
- When the short form is the record's only description line, it prints as the record's words,
  with no condition. There are 168 such records: 160 magus spellblend class features that carry
  a spell's short form, 7 spells and 1 monster ability. An example is ACG *Sunder Breaker*,
  whose full DESC is glued into its `SOURCELINK:` field in the source (`acg_spells.lst:121`).

This is pinned by `a_line_decided_never_prints_once_and_never_as_a_condition`. A first version
dropped every `Never` line. The desktop suite caught the loss of those 168 descriptions (§8,
pass 1), and that version was replaced by this one.

**Two records now print no prose at all.** Their only stated lines are decided never:
`mythic_adventures:spell:elemental_body_iiimod` (its Mythic text) and
`advanced_race_guide:equipment:amulet_of_channeled_life` (its one line is gated on the dhampir's
Negative Energy Affinity, a record the package does not hold). Without a further change, the
converter's description fallback would have printed those same words with the condition
dropped. It now does not (`mod.rs`, `prose_decided_never`).
`a_converted_record_never_drops_the_description_its_corpus_row_states` names the one of the two
whose corpus row states a description (`elemental_body_iiimod`) as **withheld, 1**, and reports
**dropped 0 of 7,629**.

## 5. Converter step (structural-diff protocol)

| Gate | Command | Result |
|---|---|---|
| write | `cargo run --locked --quiet -j 8 -p codex-ingest --bin sheet_rule_convert -- --write` | exit 0; records 49,450, converted 49,450, refused 0; rules 73,363; var tables 6,211 (all unmoved); files 56,008 |
| pins | `python3 .../scripts/f7b_delta_pins.py <tranche/16 pkg> data/sheet_rules .../structural_diff_f7b_deltas.json` | 2,317 (rule id, field) deltas on 2,308 records, 0 unexplained |
| structural diff | `python3 .../scripts/structural_diff.py data/sheet_rules --baseline <tranche/16 pkg>` | **`verdict=PASS`** (`f7b-structural-diff.txt`): F7b 2,317 of 2,317 pinned deltas; unexpected field deltas 0; removed edges 0; removed grants 0; records 49,450 -> 49,450; `_defects/` +4 -0 |
| planted mutations | `python3 .../scripts/f7b_planted_mutations.py <scratch> . <tranche/16 pkg>` | **8 of 8 FAIL, both controls PASS** (`f7b-planted-mutations.txt`) |
| diff self-test | `python3 .../scripts/structural_diff_test.py` | 39 of 39 (+1: `test_f7b_shapes_classify_exactly_the_f7b_prose_deltas`) |

**Delta classes** (`f7b_shapes.py`). Each pin is (rule id, field, class, sha256 of the new value).
`f7b_shapes.py` re-checks each class's shape against the baseline record. It restates the
converter's rule in Python, so if the two ever disagree, the shape check fails. A record can
carry several classes.

| Class | Field deltas | What it checks |
|---|---:|---|
| `formula_render` | 1,929 | every formula span in the old words is a typed piece now. The words around it, and every segment's family, condition and flags, are identical. Every pre-existing slot expression survives, in order |
| `bracket_escape` | 1,095 | in spell/power text only, `[`/`]` became `(`/`)`, and nothing else changed |
| `out_of_inventory_line` | 314 | exactly the lines whose out-of-inventory condition decides `Never` are gone |
| `out_of_inventory_condition` | 36 | each such condition is exactly its decided form |
| `former_words` | 110 | a record on the baseline's words-rewrite list: same family, condition and flag sequence, and no formula left |
| `formula_unconverted` | 1 | `advanced_race_guide:spell:binding_earth_mass`, Target: `(CASTERLELVEL) creatures or objects` became `creatures or objects` |
| `formula_oracle_zero_note` (`provenance`) | 9 | `undeclared_in_pinned_tree` gains the name, and nothing else in `provenance` moves (the nine Naturalist summon records) |

Bracket rows (1,358 rows on 1,144 records) outnumber the `bracket_escape` class (1,095 records)
for two reasons. First, 45 records are counted under `former_words`. Second, 4 records have their
description declared product identity and withheld after conversion (for example
`adventurers_guide:spell:preserve_grace`), so the lowered text never prints.

**Planted mutations** (each one FAILs):

- M1: Fireball's cap moves from 10d6 to 20d6.
- M2: a lowered duration is spelled as the source formula again.
- M3: Fireball's mythic line prints again.
- M4: brackets are escaped in a feat.
- M5: an ordinary gated line is dropped (Deadly Aim).
- M6: the words around a pinned delta change.
- M7: a decided condition is left undecided.
- M8: the oracle-zero note is withdrawn.

**Examples** (package after F7b):

| Record, line | Before | After (piece) | Catalog, no character | Sheet, caster level 5 |
|---|---|---|---|---|
| `core_rulebook:spell:fireball`, short form (gated `Never`) | `(min(10,CASTERLEVEL))d6` | `DiceCount{Min(10, CasterLevel), 6}` | not printed (`Never`) | not printed |
| `occult_adventures:spell:primal_regression`, Duration | `(CASTERLEVEL) minutes` | `Slot(CasterLevel)` + ` minutes` | "caster level minutes" | "5 minutes" |
| `core_rulebook:spell:cure_light_wounds`, short form (`Never`) | `1d8+(min(5,CASTERLEVEL))` | `Dice{1d8, Min(5, CasterLevel)}` | not printed | not printed |
| `inner_sea_world_guide:feat:godless_healing`, Desc | `1d8+(character level)` (old words) | `Dice{1d8, Level}` | "1d8 plus character level" | "1d8+5" (level 5) |
| `advanced_class_guide:spell:naturalist_summon_nature_s_ally_i`, Duration | `(ConjurationNaturalistsCharmBonus+(CASTERLEVEL)) minutes [D]` | `Slot(CasterLevel)` + ` minutes (D)` | "caster level minutes (D)" | "5 minutes (D)" |

## 6. The residue gate now has a formula-shape class

`scripts/pcgen_residue_gate.py`, `PROSE_FORMULA_PATTERN`. For every shipped JSON document that is
a list of converted rules, the class counts the formula-shaped groups in each printed string
(prose `Text` pieces and the label), using the converter's structural test restated. Hits fold
into `live_files` / `live_hits`, which `--closure` reads.

- RED, tranche/16 package: `files=1997 hits=2361`, `verdict=FAIL` (`f7b-red-residue-gate.log`).
- GREEN, this package: `live_files=0 live_hits=0`, `verdict=PASS` (§8).
- `TestProseFormulaClass` (2 tests): a planted formula is a hit and its typed piece is not; the
  book's asides are not hits.

## 7. RED -> GREEN: `fireball_prints_its_damage_as_prose`

`apps/desktop/src-tauri/src/spell_catalog.rs`. The test reads the CRB Fireball entry the Spells
tab prints. It requires the CRB p.284 sentence "deals 1d6 points of fire damage per caster level
(maximum 10d6) to every creature within the area". It also forbids `CASTERLEVEL`, `min(`, `))d`,
`[maximum`, "no record in the corpus", "If requires", "not available to a character" and
"Mythic:".

- RED on F7a (`f7b-red-desktop.log`): the three lines of §1.
- GREEN after regeneration (`f7b-green-desktop.log`): one paragraph, the CRB text. It was
  hand-checked against the CRB p.284 Fireball description: "(maximum 10d6)", "the range
  (distance and height)", no mythic text.

The converter and printer unit tests were written with the code they pin (the functions did not
exist before):

- `prose_formula_spans_find_source_formulas_and_leave_the_books_asides`
- `dice_around_a_formula_are_read_as_dice`
- `an_out_of_inventory_condition_is_decided_never_printed`
- `counted_dice_print_per_level_with_their_cap` (catalog words, and `5d6` for a level-5
  character)
- `a_line_decided_never_prints_once_and_never_as_a_condition`
- `a_percentage_whose_number_is_words_says_percent`: "(70+CASTERLEVEL)% chance" (ISWG
  *Ancestral Memory*) prints as "70 plus caster level percent chance" with no character; on a
  sheet the number prints before the `%`

The live slot gate
(`every_unsettled_slot_in_the_live_package_renders_as_words_not_as_the_characterless_zero`) now
skips `Never` lines when it looks for a rule's unsettled slot. It had flagged 66 rules whose only
unsettled slot sits in a short form that also has a full form. That short form prints on no
screen, so the evaluator and the catalog render those rules the same way; that is not a
characterless zero on a screen.

## 8. Verification (one pass, after all changes): `f7b-verify.log`

Before pass 1, every `.rs` source and `Cargo.toml` in the worktree was touched. The census's
"before" build (§9) ran in a second worktree on the shared target directory, and cargo then
reused that worktree's `codex` artifacts for this one: equal relative paths, newer mtimes. A
first attempt at pass 1 caught it: `codex-ingest` did not compile against a `codex` without
`DiceCount`. That attempt and the census "after" run taken beside it were discarded; neither is
cited here.

| Gate | Command | Result |
|---|---|---|
| freshness | `cargo run --locked --quiet -j 8 -p codex-ingest --bin sheet_rule_convert -- --check` | exit 0, `verdict=PASS`; records 49,450, converted 49,450, refused 0, rules 73,363, var tables 6,211 |
| residue | `python3 scripts/pcgen_residue_gate.py --check --closure` | `live_files=0 live_hits=0 verdict=PASS`; shipped files scanned 70,049 (the prose-formula class included) |
| frozen | `python3 scripts/site/check_frozen_status.py --check` | OK, frozen at 100% (49,450 units) |
| bundle | `node scripts/gen-corpus-bundle.mjs` | `files_copied=14029`; no tracked change outside the F7b set |
| gate unit tests | `python3 scripts/tests/test_pcgen_residue_gate.py` | 48 of 48 (+2: `TestProseFormulaClass`) |
| diff self-test | `python3 .../scripts/structural_diff_test.py` | 39 of 39 |
| convert gate | `cargo test --locked -j 8 -p codex-ingest --test sheet_rule_convert_gate -- --test-threads=8 --nocapture` (run before the passes, on the written package) | 48 of 48; description gate: dropped 0 of 7,629, withheld 1 (§4) |
| workspace | `cargo test --locked -j 8 --workspace --no-fail-fast -- --test-threads=8` | pass 1: 461 test-result lines, 8,188 passed, 0 failed, 70 ignored. Pass 2: **8,189 passed, 0 failed, 70 ignored** (`f7b-suite-workspace.log`) |
| desktop | `cargo test --locked -j 8 --manifest-path apps/desktop/src-tauri/Cargo.toml -- --test-threads=8` | pass 1: 633 passed, **5 failed** (below). Pass 2: **638 passed, 0 failed** (`f7b-suite-desktop.log`) |
| clippy, root and ingest | `cargo clippy --locked -j 8 --workspace --all-targets -- -D warnings` | exit 0, 0 warnings (both passes) |
| clippy, desktop | `cargo clippy --locked -j 8 --tests --manifest-path apps/desktop/src-tauri/Cargo.toml -- -D warnings` | exit 0, 0 warnings (both passes) |
| class status table | `python3 scripts/gen_class_status_table.py --check` | OK (ids 137, computed 63, prestige swept 74, mix panel 185 of 185), both passes |
| census | `cargo run --locked -j 8 --quiet --bin class_census -- --json` | §9: 0 differing leaves, both passes |

The frontend (`apps/desktop/src/**`) is untouched, so `npm test` was not run. The desktop app was
not launched.

**Fixture protocol, pass 1's five desktop failures.** All five are class B: a genuine defect,
fixed at the root, no count re-baselined.

- `spell_catalog::no_served_spell_description_carries_raw_pcgen_syntax` and
  `equipment_catalog::no_catalog_serves_a_description_carrying_raw_pcgen_syntax`: ISWG *Ancestral
  Memory* printed "70 plus caster level% chance" (the old words rewrite used to carry the sign).
  Fixed in the catalog printer (`a_percentage_whose_number_is_words_says_percent`).
- `spell_catalog::converted_spell_prose_population` (2,408 of 2,481 against the floor 2,410),
  `spell_catalog::crb_acg_and_arg_records_are_always_fully_populated` ("Sunder Breaker has no
  description") and `class_feature_descriptions::the_served_population_never_falls_below_its_recorded_floor`:
  the first catalog rule dropped every `Never` line, which took the only description of the 168
  records in §4. The print-once rule replaced it. Pass 2 serves 2,410 of 2,481 spell descriptions,
  which equals the floor.

## 9. Census

`class_census --json` at F7a (a detached worktree at `c724805053`, removed after the run) and on
this branch, compared leaf by leaf with `generated_at` excluded, over classes 63 rows, prestige 74
rows and mix panel 185 rows: **0 differing leaves** (`f7b-census-diff.txt`; the after file is
`../census-f7b.json`). The counts are unchanged: ids 137, computed 63, prestige swept 74 (mix
computed 68), mix panel 185 of 185.

## 10. Remainder, by mechanism

- **`binding_earth_mass` Target prints "creatures or objects" with no count.** The source
  misspells the formula (`CASTERLELVEL`, a name DEFINEd nowhere, which the oracle does not
  provably read as 0). The row is named in `_defects/inline-formula-unconverted.json`. The count
  can come only from a book-cited override (ARG, *Binding Earth, Mass*).
- **Two records print no prose.** Their only lines are gated on records outside the inventory:
  the mythic `elemental_body_iiimod`, and `amulet_of_channeled_life`, gated on the dhampir's
  Negative Energy Affinity. They close when the converted inventory holds those records.
- **With no character, a lowered slot prints its expression's words**, for example "caster level
  minutes". A character's sheet prints the number. A per-level phrasing of `Slot` ("1 minute per
  caster level") would need the unit that follows the slot, which the printer does not read; the
  `DiceCount` printer does this for dice only.
- F7-3, F7-4, F7-6, F7-7, F7-9 and F7-10 are later F7 steps and are not touched here.

## 11. Artifacts

- `f7b-red-desktop.log`, `f7b-green-desktop.log`, `f7b-red-residue-gate.log`, `f7b-verify.log`
- `f7b-structural-diff.txt`, `f7b-planted-mutations.txt`, `f7b-census-diff.txt`,
  `../census-f7b.json`
- `../scripts/f7b_shapes.py`, `../scripts/f7b_delta_pins.py`,
  `../scripts/f7b_planted_mutations.py`, `../scripts/structural_diff_f7b_deltas.json`;
  `structural_diff.py` and its self-test were extended
- `scripts/pcgen_residue_gate.py`, `scripts/tests/test_pcgen_residue_gate.py`
