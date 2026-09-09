# Cycle AT-35-E5-005_cycle1 — Epic 5 — Residues / AT-35-E5-005

- **Commit SHA:** `103693b365` (the artifacts) plus `__DOCS_SHA__` (this receipt, `progress.md`, `kanban.md`). This cycle writes no code and moves no unit: three
  re-derivation scripts, their three artifacts, the re-stamped atlas artifact, the retro events,
  this receipt, `progress.md` and `kanban.md`). Cycle start
  `ac2165b393abec7ecf910599e0d5bd3139660af1` on `tranche/15`.
- **Scope gate:** `scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER` — the
  literal last line of `python3 scripts/cycle_scope_gate.py --min 500` at `ac2165b393`
  (`scope=(whole remainder)`, `scoped_by_bucket=` empty, `scoped_by_kind=` empty). The dispatch
  flagged this cycle `SCOPE_GATE: EXEMPT (closure-accounting cycle — it proves zero remains)`;
  the gate was run anyway and returns the stronger statement, that the remainder it would have
  scoped is **empty**. `python3 scripts/pcgen_residue_gate.py --check` at start:
  `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`.
- **Files touched:**
  `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-005_completion_manifest.py`
  (new), `.../AT-35-E5-005_capability_register.py` (new),
  `.../AT-35-E5-005_desc_without_prose.py` (new), `.../completion-manifest.json` (new, 49,438
  rows), `.../capability-register-rederived.json` (new, 11 rows),
  `.../desc-without-prose.json` (new, 10 rows),
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (`derived_at`
  re-stamped by `completion_atlas.py --check`'s own write — kept, not reverted, because reverting
  leaves the gate stale; the same disposition `7557ab00fa` took),
  `docs/retro/events/at-35-e5-005.jsonl` (2 corrections + 1 deferral), this receipt,
  `progress.md`, `kanban.md`. **No file under `src/`, `apps/`, `data/`, `tests/` or `scripts/`
  was written.**
- **Identifier audit result:** OK_NO_BUNDLE_TAGS —
  `BASE_BRANCH=fe5ae6cd4a5f3c65d5d10f4d523f00e33b04ac47` (`git merge-base HEAD origin/develop`);
  `git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <Epic 5's file-touch set> ':!**/__tests__/**' ':!**/*.test.*' | grep -cE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  → `0`, at start and on the final diff.
- **Wired-integration audit result:** OK_NO_TOKENS on this cycle's own diff. Over Epic 5's whole
  file-touch set since `fe5ae6cd4a` the same grep
  (`\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b`) returns **14** matches at
  start and **16** once this receipt itself is committed. The **2** added are this receipt's own
  two lines — the sentence you are reading, which quotes the pattern, and the line below naming
  the Tophet / Plant Growth rulebook prose. **0 code hits were added by this cycle**, which ships
  no code; the other 14 were already
  named by AT-35-E3-001's and AT-35-E5-003's receipts: rulebook-prose strings inside
  `data/sheet_rules/**` `ProsePiece::Text` (Tophet "hack or smash", Plant Growth "hack or force"),
  earlier receipts' own audit sentences quoting the pattern, and **removed** (`-`) inventory lines
  carrying the `vacuous_placeholder_row` evidence string. No stub, inline mock or `"Would …"`
  string in shipping code — this cycle ships no code at all.
- **Acceptance criterion:** verbatim from `epic-breakdown.md` `### AT-35-E5-005`:
  "**AT-35-E5-005 — the corpus reaches 49,438 of 49,438, and the capability register is closed.**
  **Evidence:** `completion_atlas.py --check` → `DONE=49438 of 49438`, every other bucket zero.
  `artifacts/epic-5-residues/completion-manifest.json` — one row per unit. SD-34's
  `capability-register.json` re-derived: every row `built: true` or
  `unnecessary-under-sheet-rule: <reason>`."
- **Receipt rows (mechanical):** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=1 pcgen_live_files=260`
  — the literal last line of
  `python3 scripts/cycle_scope_gate.py --receipt --since ac2165b393abec7ecf910599e0d5bd3139660af1 --before /tmp/wi-before-AT-35-E5-005.json --after docs/work-inventory.json`.
  `closed=0` is correct and is the point of the cycle: the population was already zero at the
  cycle start, and a closure-accounting cycle proves that rather than moving it.
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`
  — identical at start and at HEAD. **Not risen.** This cycle writes no live-side code.
- **Oracle parity:** N/A — no `Number` mapping was added and no live path was touched. The
  bundle's bucket-V oracle comparison was paid once by AT-35-E4-002
  (`compared=392 oracle_agree=184 oracle_disagreement=10 oracle_unverifiable=198` at
  `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`).
- **Movement, four buckets:**
  - **closure (into DONE, by id-set): 0.** Non-DONE was 0 of 49,438 at the cycle start and 0 at
    HEAD.
  - **relabel (bucket to bucket): 0.**
  - **reachability: 0 regressions** — no unit left DONE.
  - **instrument-correction: 2.** Both below, both emitted as `correction` retro events with
    `--verified-by`.
- **Refused tokens:** none — the cycle refuses no token, converts nothing and scopes no units.
  **It does report a named, summing residue that is NOT a refused token and NOT a carve-out:**
  `desc_token_present_but_no_prose_on_the_sheet_rule=10` (see Discoveries, and
  `desc-without-prose.json`).
- **Discoveries:** two, each a `correction` event.
  1. **`1788994085684-at-35-e5-005-ca03fd`** — SD-34's `capability-register.json` states
     `oracle_probe_surface_for_no_table_kinds` `population: 2062` (ability 745, companion 104,
     monster 843, monster_ability 244, template 126). The row's **own** stated
     `re_derive_command`, run against the inventory at the register's **own** stated
     `generated_at_head` (`837dbbcf6b`), returns **130** (ability 90, template 36, companion 4).
     The re-derived register carries both — `predecessor_stated_population: 2062` beside
     `rederived_id_set_size: 130` — rather than silently replacing one with the other. The
     row's disposition is unaffected: at HEAD all **8,491** `oracle-unverifiable` units in the
     corpus, a superset of both figures, are DONE.
  2. **`1788994100695-at-35-e5-005-9a36f1`** — the manifest's `sheet_rule_content` column
     (this cycle's own new instrument) found **10 of 23,315** `sheet-complete` units whose
     evidence reads `sheet_rule_rendered:words` while the `SheetRule` carries **no words**: no
     prose pieces, no principal value, and — for the 5 that are pointer rows — a `granted_by`
     rule that is equally empty. Their corpus records carry a real `DESC` token with 224–829
     characters of published rules text. The sheet prints a bare label where the rulebook prints
     a paragraph, so those 10 lines are not `decisions.md §1` form 3. The other **10,276** of the
     10,286 label-bearing rows (8,380 `label_only` + 1,906 `label_only_with_granted_by`, less 5
     hollow in each) are correct: their corpus record has no description upstream at all
     (`description: ""`, no `DESC` token), so the feature's NAME is the finished sheet line.
     **The fix is converter-side** (`src/pcgen_import/sheet_rule/`), which is **not in Epic 5's
     file-touch set** (`workflow-instruction.md §3`); editing it from a closure-accounting cycle
     is §8's "two live cycles on conflicting files". Measured, named, gated and handed on
     instead: deferral `1788994100821-at-35-e5-005-5973cb`, and
     `AT-35-E5-005_desc_without_prose.py --check` **exits 1 until the count is 0**, so the next
     converter cycle inherits a red gate rather than a note.
- **Figures + their re-derive commands:**
  | Figure | Value | Command | Denominator |
  |---|---|---|---|
  | corpus partition | `DONE 49438 / A 0 / B 0 / C 0 / D 0 / M 0 / V 0 / U 0 / X 0 / Z 0` | `python3 scripts/completion_atlas.py --check` | of 49,438 corpus units (`population=49438 buckets=10 unclassified=0 overlap=0`) |
  | manifest rows | **49438** | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-005_completion_manifest.py --check` | of 49,438 corpus units, one row each |
  | manifest non-DONE rows | **0** | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-005_completion_manifest.py --check` | of the 49,438 manifest rows |
  | manifest rows by status | `sheet-complete 23315 / text-complete 11599 / oracle-unverifiable 8491 / grounded 5222 / oracle-agree 811` | `python3 -c "import json;print(json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/completion-manifest.json'))['summary']['by_status'])"` | of the 49,438 manifest rows |
  | manifest rows by sheet-rule content | `prose 29213 / label_only 8380 / prose+value 6117 / value_only 3680 / label_only_with_granted_by 1906 / no_rule 142` | `python3 -c "import json;print(json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/completion-manifest.json'))['summary']['by_sheet_rule_content'])"` | of the 49,438 manifest rows |
  | manifest coverage | **37 books, 19 kinds, 155 distinct evidence strings** | `python3 -c "import json;s=json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/completion-manifest.json'))['summary'];print(s['books'],s['kinds'],s['distinct_evidence_strings'])"` | of the 49,438 manifest rows |
  | capability-register rows closed | `rows=11 built=5 unnecessary_under_sheet_rule=6 still_open=0 units_covered=11055` | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-005_capability_register.py --check` | of SD-34's 11 capability rows |
  | capability-register units not DONE | **0** | `python3 -c "import json;print(json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/capability-register-rederived.json'))['summary']['units_covered_not_done_at_head'])"` | of the 11,055 units the 11 rows name |
  | SD-34 row population reproduced exactly | **10 of 11** rows | `python3 -c "import json;d=json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/capability-register-rederived.json'));print([c['id'] for c in d['capabilities'] if isinstance(c['predecessor_stated_population'],int) and c['predecessor_stated_population']!=c['rederived_id_set_size']])"` | of the 9 rows SD-34 sized (the two UNSIZED rows are sized here for the first time) |
  | the one row that does not reproduce | `oracle_probe_surface_for_no_table_kinds` **2062 stated, 130 re-derived** | `git show 837dbbcf6b:docs/work-inventory.json \| python3 -c "import json,sys;u=json.load(sys.stdin)['units'];print(len([x for x in u if x.get('status')=='oracle-unverifiable' and 'AT-33-E1-003 probe-surface census' in (x.get('reason') or '')]))"` | of the 2,327 `oracle-unverifiable` units at `837dbbcf6b` |
  | the two rows SD-34 left UNSIZED, now sized | `cross_record_content_ownership_resolution` **1906**, `corpus_content_extraction_for_uncaptured_records` **8380** | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-005_capability_register.py --check` | of the 49,438 manifest rows |
  | hollow words-form sheet lines (this cycle's residue) | **10** (ability 3, class_feature 7; 5 of them pointer rows whose granter is equally empty) | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-005_desc_without_prose.py` | of the 23,315 `sheet-complete` units |
  | label-only sheet lines that are CORRECT | **8375** of `label_only` (**10276** across both label-bearing shapes) | `python3 -c "import json;m=json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/completion-manifest.json'));d=json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/desc-without-prose.json'));print(m['summary']['by_sheet_rule_content']['label_only']-sum(1 for u in d['units'] if not u['granted_by']))"` | of the 8,380 `label_only` manifest rows |
  | package source-format markers | **0** | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` | of 54,586 package files (`find data/sheet_rules -name '*.json' \| wc -l`) |
  | upstream editorial not-implemented markers | **0** | `grep -rlEi '\[(not implemented\|ml bonus not implemented)' data/sheet_rules/ \| wc -l` | of 54,586 package files (`find data/sheet_rules -name '*.json' \| wc -l`) |
  | token ledger | `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1 verdict=PASS` | `python3 scripts/token_coverage.py --check` | of 49,438 corpus units |
  | engine boundary | `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True` | `python3 scripts/shape_engine_boundary.py --check` | of 49,438 corpus units |
  | missing engine tables | `population=0 kinds=0 citation_failures=0` | `python3 scripts/missing_engine_tables.py --check` | of the 2 kinds `ENGINE_SURFACE_CITATIONS` names |
  | PCGen residue | `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` | `python3 scripts/pcgen_residue_gate.py --check` | of the 260-file live-side baseline |
- **Build scope verified:** run at the cycle's code HEAD `ac2165b393` (this cycle adds **no**
  Rust, so its docs commits cannot move a build result), `CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E5-005`
  emptied first, `CARGO_INCREMENTAL=0`, `-j 6`. See `## Build result` below for the literal
  output.
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`
  - `cargo run --locked --bin sheet_rule_convert -- --check` → `CONV_CHECK_EXIT=0`
  - `cargo clippy --locked --tests -j 6` → `CLIPPY_EXIT=0`
  - `cargo test --locked --no-fail-fast -j 6` — **deliberately not run, and this is the rule, not
    an omission**: `workflow-instruction.md §6` step 3 requires it "when `src/` or the classifier
    changed", and this cycle changed neither (`git diff --name-only ac2165b393..HEAD -- src tests scripts data apps` is empty). The last full-workspace run is
    AT-35-E5-004's, **8,741 passed / 0 failed over 412 targets**.
  - `cargo run --locked --bin corpus_literal_sweep` — not run: no corpus record changed
    (`git status --porcelain -- data/corpus` empty throughout).
  - the desktop crate and the frontend run at the **epic wrap-up** (`decisions.md §3`): this
    cycle touched nothing under `apps/`.
- **Sweep population:** N/A — `corpus_literal_sweep` did not run; no corpus record changed. Its
  last run is AT-35-E3-001's, `48,706 records examined of 51,476 read, 0 findings, CLEAN`.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` — every
  `data/sheet_rules/**` record this cycle read carries it as `provenance.oracle_pin`; no figure
  here was taken from the pinned tree directly.
- **Status:** complete — the criterion's population is **0 of 49,438** at HEAD and all three of
  its Evidence clauses are paid: the atlas prints `DONE 49438` with every other bucket `0`;
  `completion-manifest.json` carries one row per unit, 49,438 of them, none non-DONE; and all
  **11** of SD-34's capability rows are closed, **5** `built: true` and **6**
  `unnecessary-under-sheet-rule: <reason>`, with **0** still open. The 10-unit residue is
  reported as a number and handed to a converter-side cycle with a red `--check` gate — it is
  **not** an exemption and no unit was excluded from any denominator on its account.
- **Notes:** (a) The manifest imports `completion_atlas._bucket_of` rather than re-implementing
  bucket derivation, so the manifest and the atlas cannot disagree; the three fail-closed
  assertions in `AT-35-E5-005_completion_manifest.py` (all-DONE, population equals the atlas's
  `examined`, histogram equals `partition()`'s counts) are what make a silently-wrong manifest
  impossible rather than merely unlikely. (b) Two of SD-34's rows were `population_source:
  "cited"` with a `verification_note` asking the next lane to pin a live citation; both are
  pinned here, on the inventory's own `source_file` field
  (`cr_classes_companion.lst` → 2 units, `ce_abilities_familiar_cr.lst` → 14), and both resolve
  to exactly the cited counts. A third, `marker_stripping_for_pcgen_editorial_markers`, is also
  live-resolvable and resolves to exactly its cited 21. (c) `still_open=0` is the register's
  closure: SD-34 wrote "11 of 11 capabilities named here are NOT built"; at SD-35 HEAD 5 were
  built and the other 6 were machinery the sheet rule does not need, and no unit rests on any
  of them.
- **Next-cycle scope:** criterion at zero; Epic 5 at zero.
  `python3 scripts/cycle_scope_gate.py --min 500` → `scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER`.
  **One named hand-off, not a carve-out:** a converter-side cycle owning
  `src/pcgen_import/sheet_rule/` takes the 10 units in
  `artifacts/epic-5-residues/desc-without-prose.json` and turns
  `AT-35-E5-005_desc_without_prose.py --check` green (currently exit 1). Epic 6 owns that path.

## Build result

Run at `ac2165b393`, `CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E5-005` (created empty),
`CARGO_INCREMENTAL=0`, `-j 6`.

```
cargo test --locked --no-run -j 6                        NO_RUN_EXIT=0
                                                        (412 test executables linked)
cargo run --locked --bin sheet_rule_convert -- --check   records=49438 converted=49296 refused=142
                                                        rules=69344 var_tables=5277 verdict=PASS (117.5s)
                                                        refused    142  no_corpus_record
                                                        CONV_CHECK_EXIT=0
cargo clippy --locked --tests -j 6                       Finished `dev` profile in 1m 26s
                                                        0 lines matching '^warning|^error'
                                                        CLIPPY_EXIT=0
```
