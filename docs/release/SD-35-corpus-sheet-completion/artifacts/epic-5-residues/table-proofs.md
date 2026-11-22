# AT-35-E5-001 — table proofs: the refusal/success transcript pair

The artifact `acceptance-and-verification.md` names for `AT-35-E5-001`
("`missing_engine_tables.py --check` → `population=0`; **per table a refusal
transcript and a success transcript**").

Two tables are in scope, and they were bucket A's entire population at SD-35's
launch — `scripts/missing_engine_tables.py`'s `ENGINE_SURFACE_CITATIONS` names
exactly these two kinds and no others:

| Table | Book | Bucket-A units at the `tranche/15` cut |
|---|---|---|
| `power` | `ultimate_psionics` | 421 of 449 |
| `companion` | `bestiary` | 28 of 449 |

Both are now served by the **live sheet-rule package**, which loads
`SheetRule.applies` — a typed gate tree — and never a source token
(`decisions.md §11`; `epic-breakdown.md` `### AT-35-E5-001`: "The tables load
`SheetRule.applies`, not tokens").

## 1. The transcript pair

Produced by the read-only mode added for this criterion. It writes nothing,
classifies nothing, and moves no unit on any board — the same contract
`--epic2-table-transcript` has:

```
cargo run --locked --bin v06_work_inventory -- --epic5-table-transcript
```

Output at `HEAD` (verbatim):

```
kind=power book=ultimate_psionics location=data/sheet_rules/ultimate_psionics/power/*.json records=447 sample="ultimate_psionics:power:ability_as_one" -> HELD label="Ability As One" applies=Always sheet_line=words printed="" prose_len=314 pcgen_markers_in_record=0
kind=power book=ultimate_psionics location=data/sheet_rules/ultimate_psionics/power/*.json records=447 sample="ultimate_psionics:power:___a_key_no_corpus_record_carries___" -> REFUSED (absent key)
kind=companion book=bestiary location=data/sheet_rules/bestiary/companion/*.json records=450 sample="bestiary:companion:air_elemental_air_mastery" -> HELD label="Air Mastery" applies=Always sheet_line=words printed="" prose_len=89 pcgen_markers_in_record=0
kind=companion book=bestiary location=data/sheet_rules/bestiary/companion/*.json records=450 sample="bestiary:companion:___a_key_no_corpus_record_carries___" -> REFUSED (absent key)
```

**The success half** takes each table's *first record by sorted rule id* — read
off the live package, never a hand-picked key (`decisions.md §4`: a per-kind
gate that reads the live directory, not a per-unit fixture with a hand-derived
expected value) — resolves it, and renders it through the live evaluator
(`render_for_probe`, the same call the `sheet-complete` rung makes) for the
deterministic probe character. `sheet_line=words` is one of `decisions.md §1`'s
three legal forms: this record's rule is the rule's words, which under the sheet
rule is a finished sheet line, not a shortfall.

**The refusal half** asks the same table for a key no record carries and
requires a *named* refusal. Fail-closed, three ways:

- a resolve on the absent key prints `REFUSAL_CHECK_FAILED (fabricated match)`;
- an id the package indexes that will not resolve prints `SUCCESS_CHECK_FAILED`;
- a table that loaded **no** records prints `TABLE_EMPTY (fail-closed: the table
  loaded no records)` rather than an empty transcript, so a table that silently
  stopped loading cannot read as a clean run.

Each of those three strings fails the tests in §3.

## 2. Figures, and the denominators they are over

The record counts in the transcript and the unit counts in the inventory are
**two different populations**, and conflating them is exactly the error shape
`decisions.md §2` exists to catch. Stated separately:

| Figure | Value | Population it is over | Re-derive |
|---|---|---|---|
| `power` record files | 421 | files in `data/sheet_rules/ultimate_psionics/power/` | `ls data/sheet_rules/ultimate_psionics/power/*.json \| wc -l` |
| `power` rules in the package | 447 | principal rules **plus** their `#suffix` siblings (421 + 26) | the transcript's `records=` field |
| `power` **units** in the inventory | 421 | `kind=power`, `book=ultimate_psionics` in `docs/work-inventory.json` | command A below |
| `companion` record files | 154 | files in `data/sheet_rules/bestiary/companion/` | `ls data/sheet_rules/bestiary/companion/*.json \| wc -l` |
| `companion` rules in the package | 450 | principal rules **plus** siblings (154 + 296) | the transcript's `records=` field |
| `companion` **units** in the inventory | 154 | `kind=companion`, `book=bestiary` in `docs/work-inventory.json` | command A below |

Command A:

```
python3 - <<'EOF'
import json, collections
u = json.load(open('docs/work-inventory.json'))['units']
for kind, book in (('power','ultimate_psionics'), ('companion','bestiary')):
    r = [x for x in u if x.get('kind') == kind and x.get('book') == book]
    print(kind, book, 'units=%d' % len(r), dict(collections.Counter(x['status'] for x in r)))
    print('  ', dict(collections.Counter(x['evidence'] for x in r).most_common(4)))
EOF
```

Its output at `HEAD`:

```
power ultimate_psionics units=421 {'sheet-complete': 421}
   {'sheet_rule_rendered:words': 412, 'sheet_rule_rendered:number': 9}
companion bestiary units=154 {'text-complete': 27, 'grounded': 66, 'oracle-unverifiable': 33, 'sheet-complete': 28}
   {'bestiary_1_companion_resolve_returned_a_real_record': 66, 'oracle_verdict_unverifiable_bucket_v_consolidated_at34_e3_005': 33, 'sheet_rule_rendered:words': 28, 'companion_held_and_corpus_record_carries_real_description': 27}
```

**This is the closure, per unit set, not merely in aggregate.** All 421 of the
421 `power` units the criterion names are `sheet-complete` with a rendered sheet
line (412 `words` + 9 `number` = 421). And of the 154 `companion` units in
`bestiary`, exactly **28** are `sheet-complete` via `sheet_rule_rendered:words`
— the same 28 the criterion names as the `companion` widening; the other 126 of
the 154 were already DONE by other rungs before Epic 5 and were never bucket A.

## 3. The tests behind the transcript, and their RED half

Three tests read the **live** `data/sheet_rules/` directory (never a
hand-written per-unit fixture), in
`src/bin/v06_work_inventory.rs`, module `apply_sheet_complete_rung_tests`:

| Test | What it fails on |
|---|---|
| `both_epic5_tables_hold_a_real_record_and_render_it` | either table emptying; a success half that does not resolve; no rendered sheet-line form |
| `both_epic5_tables_refuse_an_absent_key_rather_than_fabricate` | a fabricated match for a key no record carries |
| `epic5_table_records_carry_a_typed_applies_and_no_source_tokens` | a source-format token surviving conversion into a record the live side reads (`workflow-instruction.md §12` row 36) |

```
cargo test --locked --bin v06_work_inventory -j 6 epic5
```

GREEN at `HEAD`:

```
running 3 tests
test apply_sheet_complete_rung_tests::both_epic5_tables_refuse_an_absent_key_rather_than_fabricate ... ok
test apply_sheet_complete_rung_tests::both_epic5_tables_hold_a_real_record_and_render_it ... ok
test apply_sheet_complete_rung_tests::epic5_table_records_carry_a_typed_applies_and_no_source_tokens ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 633 filtered out
```

### RED → GREEN: each guard proved able to fail

A passing fail-closed test that *cannot* fail is the shape `AGENTS.md` rule 7
warns about, so each guard was mutated and observed failing before being
reverted.

**Mutation 1 — make the "absent" key one a record actually carries**
(`EPIC5_ABSENT_KEY` → `"ability_as_one"`), and **mutation 2 — add a marker the
records really contain** (`EPIC5_PCGEN_MARKERS` += `"label"`), applied together:

```
---- both_epic5_tables_refuse_an_absent_key_rather_than_fabricate stdout ----
power/ultimate_psionics: absent key was not refused: kind=power book=ultimate_psionics
location=data/sheet_rules/ultimate_psionics/power/*.json records=447
sample="ultimate_psionics:power:ability_as_one" -> REFUSAL_CHECK_FAILED (fabricated match)

---- epic5_table_records_carry_a_typed_applies_and_no_source_tokens stdout ----
assertion `left == right` failed: ultimate_psionics:power:ability_as_one: a source-format
token survived conversion into a record the live side reads
  left: 1
 right: 0

test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 633 filtered out
```

**Mutation 3 — point a table at a book that does not exist**
(`EPIC5_TABLES` `("power", "ultimate_psionics")` → `("power",
"a_book_that_does_not_exist")`), which is the "table silently stopped loading"
case:

```
---- both_epic5_tables_refuse_an_absent_key_rather_than_fabricate stdout ----
power/a_book_that_does_not_exist: absent key was not refused: kind=power
book=a_book_that_does_not_exist location=data/sheet_rules/a_book_that_does_not_exist/power/*.json
records=0 -> TABLE_EMPTY (fail-closed: the table loaded no records)

---- epic5_table_records_carry_a_typed_applies_and_no_source_tokens stdout ----
power/a_book_that_does_not_exist: no records to check -- the table emptied

test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 633 filtered out
```

All three mutations reverted; the suite is GREEN above at the committed source.

## 4. Bucket A at zero

```
python3 scripts/missing_engine_tables.py --check
```

```
population=0 kinds=0
citation_failures=0
```

`citation_failures=0` is load-bearing here and is **not** a contradiction of
`population=0`: the two `engine_does_not_hold("<kind>_content_has_no_engine_table")`
arms are still present and still resolvable in
`src/bin/v06_work_inventory.rs` (`AT-35-E1-002`'s content anchors resolve against
the live file on every run). The arms did not go away — **no unit reaches them
any more**, because the `sheet-complete` rung fires first on every `power` and
`companion` unit that used to fall through to them. That is the honest shape: the
fall-through refusal is still there to catch a future record the tables do not
hold, and it currently catches none.

Corroborated by the atlas over the whole corpus:

```
python3 scripts/completion_atlas.py --check
```

```
population=49438 buckets=10 unclassified=0 overlap=0
  DONE: 49438
  A: 0
  ...
done_evidence_violations=0
missing_clearing_mechanisms=0
citation_failures=0
```

## Cross-references

- `epic-breakdown.md` `### AT-35-E5-001` — the criterion and its Evidence sentence.
- `acceptance-and-verification.md` row `AT-35-E5-001` — names this file as the artifact.
- `decisions.md §1` (the sheet rule), `§4` (no per-unit proof machinery), `§11` (no PCGen in live code).
- `AT-35-E5-001_cycle1_receipt.md` — this cycle's receipt.
