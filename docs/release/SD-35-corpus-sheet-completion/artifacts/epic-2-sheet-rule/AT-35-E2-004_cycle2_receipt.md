# Cycle AT-35-E2-004_cycle2 — Epic 2 — Sheet rule / AT-35-E2-004

**This is a re-dispatch of an already-closed criterion, not new work.** AT-35-E2-004 landed at
`344f18d1e1` (receipt `AT-35-E2-004_cycle1_receipt.md`; `kanban.md` row 10 `complete`). This cycle
re-verified every clause of the criterion's `Evidence:` sentence at HEAD `9f1b27dcdf`, changed no
code, no data and no script, and closes zero units — the same shape as the three re-dispatch
re-verifications already on this branch (`969d5b9402` AT-35-E2-001, `ca14f2363f` AT-35-E2-002,
`9f1b27dcdf` AT-35-E2-003). It **corrects one stale figure in the cycle-1 receipt**: the ledger's
batch-floor column, which AT-35-E2-005's inventory regeneration legitimately moved, and which
later cycles would otherwise scope from.

- **Commit SHA:** `9f1b27dcdf10da1e43519bacc54c6eabcf6f1b62` is the tree verified — unchanged by
  this cycle apart from `docs/`. The docs-only commit carrying this receipt, `progress.md`,
  `kanban.md` and the retro event is `8a5b19e7bbee771ed2da9be5287313659ce0182d` on `tranche/15` (pinned here by the
  follow-up commit, the pattern `ca14f2363f` set on this branch). Cycle start `9f1b27dcdf`.
- **Scope gate:** `SCOPE_GATE: EXEMPT (ledger-building cycle — closes zero units by design)`
  (`decisions.md §2`; the ledger names the population later cycles scope, and the pass that moves
  units is AT-35-E2-005, which has since run — so the criterion is additionally already at zero and
  the re-verification moves nothing by construction). `python3 scripts/pcgen_residue_gate.py
  --check` at cycle start (`9f1b27dcdf`): `live_files=260 live_hits=12736 baseline_files=260
  baseline_hits=12736 verdict=PASS`.
- **Files touched:** `artifacts/epic-2-sheet-rule/AT-35-E2-004_cycle2_receipt.md` (this file, new),
  `progress.md`, `kanban.md`, `docs/retro/events/at-35-e2-004.jsonl` (1 correction appended), plus
  the two live `docs/retro/events/sd31-transcribe.jsonl` appends folded from the shared checkout
  (the standing "clean tree = unfiltered `git status` empty" rule). **No `src/`, no `scripts/`, no
  `data/`, no `apps/`, no `tests/`, no `docs/work-inventory.json`.** The atlas run re-stamped
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`'s `derived_at`
  (a one-line stamp, outside this epic's file-touch set); reverted with `git checkout --`, the same
  disposition AT-35-E2-001/002/003 recorded.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS —
  `git diff --unified=0 fe5ae6cd4a...HEAD -- <Epic 2 file-touch set> scripts/verify.sh tests/sheet_rule_convert_gate.rs ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  over the whole Epic 2 diff since `develop` prints nothing.
- **Wired-integration audit result:** **9 diff lines / 4 files**, every one rulebook prose or the
  source's own editorial wording — **none in `src/`, `scripts/`, `apps/` or `tests/`**. Attributed
  by file (`git diff --unified=0 fe5ae6cd4a...HEAD -- <the same paths> | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'`,
  then bucketed by the `+++` header): `data/sheet_rules/bestiary_3/monster_ability/tophet_swallow_whole.json` 1
  ("hack or smash its way out"), `data/sheet_rules/core_rulebook/spell/plant_growth.json` 1
  ("hack or force a way through"), `data/sheet_rules/ultimate_intrigue/class_feature/courtly_hunter_courtly_companion.json` 1
  ("[Change to magical beast … not yet implemented]" — PCGen's own bracketed editorial note in the
  rules text), `docs/work-inventory.json` 6 (the three `empty_selection_standard_*` records, whose
  `reason` field says PCGen's row is a CHOOSE-menu **placeholder** — 3 removed + 3 added lines of
  the same three records). The first three are the same three lines AT-35-E2-002 recorded
  (`1788844812035-at-35-e2-002-7cbeb2`); the other six entered the diff when AT-35-E2-005
  regenerated the inventory, and match AT-35-E2-003 cycle 2's "9 diff lines" accounting exactly.
- **Acceptance criterion:** `scripts/token_coverage.py --check` writes
  `artifacts/epic-2-sheet-rule/token-coverage.json`: per top-level PCGen token type — units carrying
  it (non-DONE, all 37 books), units converted, units refused **because of this token**, and the
  converter mapping row that handles it. Sums checked: every non-DONE unit appears under at least
  one token or under `token-less`; the refused set across all tokens equals `_refused.json`'s id
  set. **This is a tool-side script reading the corpus; it is allowed to read PCGen.**
  **Evidence:** `--check` exits 0 with sums printed; RED→GREEN on a planted double-count. Wired
  into `verify.sh` as `token-coverage`. The remainder is named by token type from here on.
- **Receipt rows (mechanical):** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a
  builds_recorded=2 pcgen_live_files=260` — the literal last line of
  `python3 scripts/cycle_scope_gate.py --receipt --since 9f1b27dcdf10da1e43519bacc54c6eabcf6f1b62 --before /tmp/wi-before-AT-35-E2-004.json --after docs/work-inventory.json --target-dir /tmp/cargo-sd35-AT-35-E2-004`
  (full output also `closed_by_kind=` empty, `relabeled_moves=` empty, `regressed=0 added=0
  dropped=0`). `rust_lines_changed=0` is the point of this receipt: nothing outside `docs/` was
  written. `builds_recorded=2` is this cycle's verification chain (the `--no-run` build and the
  clippy check against a cold `CARGO_TARGET_DIR`).
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736
  verdict=PASS` — identical at cycle start, at the end of the chain, to cycle 1's, and to all three
  preceding re-dispatches'. Nothing on the live side was read or written; the ledger, its script and
  its census are tool-side (`scripts/`, `src/pcgen_import/`, `data/sheet_rules/`), which is exactly
  the allowance the criterion's own text grants.
- **Oracle parity:** N/A — no `Number` mapping row was added and no live path was touched. The
  converter's conversion is byte-stable at HEAD: `cargo run --locked -j 6 --bin sheet_rule_convert
  -- --check` → `records=49438 converted=47628 refused=1810 rules=66514 var_tables=5081
  verdict=PASS (110.6s)`, identical to cycle 1's `--check` line.
- **Movement, four buckets:** closure 0 / relabel 0 / reachability 0 / instrument-correction 1
  (retro correction `1788889623100-at-35-e2-004-c2ecac`).
- **Refused tokens:** the ledger's remainder at HEAD, by the token type each refusal arose under, **non-DONE units** (a unit refused under k token types counts under each; **49 token types, sum with multiplicity 850, over 659 distinct non-DONE refused units of 1,404 non-DONE**): `ABILITY=200, unmapped:STARTSKILLPTS=119, SPELLS (PI-redacted token)=66, BONUS:[redacted PI]=62, BONUS:VAR=60, DEFINE (PI-redacted token)=40, DESC=40, unmapped:MODTOSKILLS=37, unmapped:SPELLSTAT=23, BONUS:COMBAT=19, unmapped:MEMORIZE=19, BONUS:SKILL=15, ASPECT:<display sub-key>=13, unmapped:SPELLLIST=12, BONUS:EQM=11, BONUS:STAT=11, BONUS:ITEMCOST=10, BONUS:MOVEADD=9, BONUS:SITUATION=9, BONUS:MISC=5, token-less=5, unmapped:KNOWNSPELLS=5, PREVARGTEQ=4, PREVARNEQ=4, TEMPBONUS=4, [redacted PI] token=4, unmapped:NUMPAGES=4, unmapped:SPELLBOOK=4, BENEFIT=3, BONUS:HP=3, BONUS:WEAPONPROF=<name>=3, HITDIE (%-step: %+1, %/4)=3, unmapped:BONUSSPELLSTAT=3, ASPECT:CheckCount / ASPECT:CheckType=2, BONUS:ABILITYPOOL=2, BONUS:SKILLRANK=2, unmapped:DOMAIN=2, unmapped:PRESPELLSCHOOL=2, ADD=1, ASPECT:NAME=1, BONUS:DR=1, BONUS:EQMWEAPON=1, BONUS:PCLEVEL=1, BONUS:SAVE=1, DR=1, NATURALATTACKS=1, PREVAREQ=1, SIZE (formula)=1, unmapped:ITEMCREATE=1` —
  `python3 -c "import json; L=json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/token-coverage.json'))['token_types']; print(', '.join(f'{t}={v[\"refused_because_of_this_token_non_done\"]}' for t,v in sorted(L.items(), key=lambda kv:(-kv[1]['refused_because_of_this_token_non_done'], kv[0])) if v['refused_because_of_this_token_non_done']))"`.
  **This list is identical, type for type and count for count, to cycle 1's** — the *denominator*
  moved (non-DONE 23,315 → 1,404) but the refused set did not, which is the strongest single
  statement this re-verification can make: AT-35-E2-005 closed 21,911 units and **not one of them
  came out of the refused set**. All statuses: 1,810 refused of 49,438. **This cycle scoped no unit,
  so no `deferral` event is owed** (`workflow-instruction.md §2.3`).
- **Discoveries:** (1) **The cycle-1 receipt's "44 token types carry ≥500 non-DONE units" is 7 at
  HEAD** — the one correction below. The seven are `TYPE` 1126, `CATEGORY` 1002, the
  `SOURCEPAGE / SOURCELONG / SOURCESHORT / SOURCEWEB / SOURCEDATE / SOURCELINK` family 913, `KEY`
  904, `DESC` 605, `ABILITY` 564, `BONUS:VAR` 502; the other 37 fell below the `decisions.md §2`
  floor when the inventory was regenerated. Only **164 of 231** token types carry any non-DONE unit
  at all. This is not a defect in cycle 1's figure — it was right at its tree — but it is exactly
  the shape of stale scoping figure `AGENTS.md` rule 9 exists for, so the ledger is now the only
  admissible source for a batch scope. (2) **The refused set is invariant under AT-35-E2-005** (the
  Refused tokens row above): the 659 non-DONE refused units are the *same* 659, so every remaining
  refusal is still live and none was closed as a side effect. That makes the ledger's remainder a
  standing, not a decaying, work list. (3) Nothing else moved: `token_types=231`, `shapes=81`,
  `unmapped_token_types=24`, `refused=1810` and the census's 49,438 entries are byte-identical to
  cycle 1's — `--check` rewrote no file (`git status --porcelain` after the run listed neither
  `token-coverage.json` nor `data/sheet_rules/_tokens.json`).
- **Figures + their re-derive commands:**
  - the ledger, verbatim (denominator: the 49,438 units of `docs/work-inventory.json` at
    `9f1b27dcdf`, DONE per `completion_atlas.py`): **`non_done=1404 tokened=1399 token_less=5
    refused=1810 refused_non_done=659 token_types=231 shapes=81 verdict=PASS`** —
    `python3 scripts/token_coverage.py --check` (last line; **2.47 s**, `time`), with all six named
    sub-checks printing `ok=True`: `population` (`census_entries=49438 inventory_units=49438
    report_records=49438`), `double_count` (`duplicate_records=0`), `coverage` (`non_done=1404
    tokened=1399 token_less=5 uncovered=0`), `refused_set` (`census_refused=1810 refused_json=1810
    union_over_token_types=1810`), `shape_totals` (`shapes=81`), `partition` (`token_types=231`)
  - **RED→GREEN on a planted double-count**, the criterion's named evidence — `python3 -m unittest
    -v scripts/tests/test_token_coverage.py` runs
    `RedGreen.test_a_planted_duplicate_census_entry_fails_the_check` and
    `RedGreen.test_a_planted_duplicate_token_on_one_record_fails_the_check` (each plants the
    duplicate, asserts `verdict=FAIL_DOUBLE_COUNT` with exit 1, removes it, asserts `verdict=PASS`
    with exit 0); whole file **`Ran 14 tests in 0.071s … OK`**
  - **wired into `verify.sh` as `token-coverage`** — `grep -n 'token-coverage' scripts/verify.sh`
    finds it in both `ALL_STAGES` (:110) and `QUICK_STAGES` (:111) with its paired self-test, and
    `scripts/verify.sh --only token-coverage-selftest --only token-coverage` → `PASS
    token-coverage-selftest (14 cases passed)`, `PASS token-coverage (non_done=1404 tokened=1399
    token_less=5 refused=1810 refused_non_done=659 token_types=231 shapes=81 verdict=PASS)`,
    `RESULT: PASS`. Stage count **48**, unchanged — `scripts/verify.sh --list | tail -n +2 | wc -l`
  - the per-type row shape, i.e. "the converter mapping row that handles it": every one of the 231
    entries of `token_types` carries `mapping_row` (`family` + `maps_to`), `carrying`,
    `carrying_non_done`, `converted_non_done`, `refused_non_done`,
    `refused_because_of_this_token`, `refused_because_of_this_token_non_done` and `refusal_shapes`
    — e.g. `BONUS:VAR` → `{"family":"bonus","maps_to":"Number(Expr) -- a contribution to a corpus
    variable, never a sheet line"}`, `carrying=10599 carrying_non_done=502 converted_non_done=242
    refused_non_done=260`; the file's top-level keys are `denominators, derived_by, inputs,
    reading_rule, refusal_shapes, schema, sum_checks, token_types, unmapped_token_types, verdict`
    (`python3 -c "import json; print(sorted(json.load(open('.../token-coverage.json')).keys()))"`)
  - types carrying ≥500 non-DONE **7** of 231 (44 at cycle 1) — `python3 -c "import json;
    print(sum(1 for v in json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/token-coverage.json'))['token_types'].values() if v['carrying_non_done']>=500))"`;
    types carrying any non-DONE **164**, same command with `>=1`
  - unmapped token types **24** of 231 — `python3 -c "import json;
    print(len(json.load(open('.../token-coverage.json'))['unmapped_token_types']))"`; refusal
    strings **81** — `python3 -c "import json;
    print(len(json.load(open('data/sheet_rules/_refused.json'))['by_token_type']))"`; shapes with a
    non-DONE record **69** of 81
  - the census: **49,438 entries, 49,443 lines, 14,631,801 bytes** —
    `python3 -c "import json; print(len(json.load(open('data/sheet_rules/_tokens.json'))['entries']))"`,
    `wc -lc data/sheet_rules/_tokens.json`; source-format literal scan **0** files —
    `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l`
  - `completion_atlas.py --check` → `population=49438 buckets=10 unclassified=0 overlap=0`,
    `DONE 48034 / A 1 / B 437 / C 79 / D 43 / M 63 / V 392 / U 202 / X 168 / Z 19`,
    `done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False
    citation_failures=0`; `shape_engine_boundary.py --check` → `magnitude_bearing=26396
    not_held_by_engine=363 citation_ok=True`; `missing_engine_tables.py --check` → `population=1
    kinds=1 (power 1) citation_failures=0`; `denominator_gate.py --check` over the package globs →
    `files_checked=43 violations=0` at verification time (**44** once this receipt is written);
    `scripts/verify.sh --only pi-sweep` → `RESULT: PASS`; `corpus_literal_sweep` **skipped** — no
    corpus record changed (`§6` step 3's guard)
- **Build scope verified:** `cargo test --locked --no-run -j 6` exit 0
  (`CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E2-004`, `CARGO_INCREMENTAL=0`, cold target dir,
  **2 min 48 s** wall, `/usr/bin/time -v`, 0 compiler warnings);
  `cargo test --locked --lib -j 6` → **3217 passed; 0 failed; 14 ignored** (54.62 s — the same
  3217 AT-35-E2-003 cycle 2 read, so no lib test moved since);
  `cargo test --locked --no-fail-fast -j 6` → **412 test binaries, 412 `test result: ok`, 0 FAILED;
  8,721 passed, 0 failed, 67 ignored**, `FULL_EXIT=0`, 0 compiler warnings (regex over every
  `test result:` line of `chain-full.log`, agreeing with `grep -c '^test result:'` → 412);
  `cargo clippy --locked --tests -j 6 --lib --bin v06_work_inventory --bin sheet_rule_convert --test
  sheet_rule_convert_gate` → **0 warnings** (1 m 21 s). `apps/` untouched, so the desktop crate and
  the frontend stay at epic cadence (`§6` step 3). Whole chain 13:46:29 → 14:33:11 (**46 min 42 s**),
  every step exit 0 — `/tmp/cargo-sd35-AT-35-E2-004/chain.log`, run on the working tree whose
  content is `9f1b27dcdf`.
- **Sweep population:** N/A — no corpus record changed, and `docs/work-inventory.json` was neither
  regenerated nor edited (`--before` and `--after` are the same file content; `closed=0`).
- **Oracle pin:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`,
  unchanged) — the converter read the pinned tree for the `-- --check` verification above, and the
  census the ledger reads was derived from it.
- **Status:** complete
- **Notes:** The criterion was at zero on arrival, so this cycle re-derived rather than re-did. The
  one judgment call worth recording: **the correction is filed against the cycle-1 receipt's
  batch-floor figure, not against the ledger**, because the ledger recomputed correctly — the
  figure was a snapshot of a moving quantity written into prose, which is the failure mode
  `AGENTS.md` rule 9 names. Prior receipts are not edited; the correction event and this receipt
  carry it.
- **Next-cycle scope:** criterion at zero (the ledger, its `--check`, its 14-case self-test and its
  two `verify.sh` stages all exist and are green at HEAD; the remainder is named by token type
  above). Epic 2 is closed on the board (rows 7–11 and 30 all `complete`); the next scope is Epic 3,
  AT-35-E3-001, whose batch must be proved against the ledger's **current** `carrying_non_done`
  column — the seven ≥500 types above — never against the cycle-1 list.
