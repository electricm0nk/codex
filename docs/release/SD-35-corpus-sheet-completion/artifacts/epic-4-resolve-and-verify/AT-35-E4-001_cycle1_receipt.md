# Cycle 1 — Epic 4, Resolve and verify / AT-35-E4-001

- **Commit SHA:** `9bae2cfa1f`
- **Scope gate:**
  ```
  inventory=docs/work-inventory.json
  scope=bucket=M
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  Bucket M was already 0 at `07e29075b4`, and so was every other bucket, so the bundled scope
  the dispatch mandates **is** the whole remainder — `PASS_WHOLE_REMAINDER`, not a floor
  exemption and not an under-floor cycle. No other criterion's units were available to bundle
  in because none exist.
- **Files touched:**
  - `src/pcgen_import/sheet_rule/table.rs` (24 mapping rows, one head alias, the row-count
    assertions 249 → 273 / 245 → 269)
  - `src/pcgen_import/sheet_rule/convert.rs` (the arms those rows name)
  - `src/pcgen_import/sheet_rule/prereq.rs` (`PRESPELLSCHOOL` / `PRESPELLSCHOOLSUB`; the
    redacted nested sub-token)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/token-mapping/mapping-table.v1.json`
    (the same 24 rows — the Rust table is a transcription of this file)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/token-coverage.json` (regenerated)
  - `data/sheet_rules/**` (376 regenerated files + 8 new `_vars/` tables)
  - `docs/work-inventory.json` (regenerated, guarded)
  - `tests/sheet_rule_convert_gate.rs` (the census gate's stale assertion, self-healed)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/AT-35-E4-001_cycle1_sheet-parity.json` (this cycle's oracle run)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (the
    atlas instrument's own `derived_at` stamp, rewritten by `completion_atlas.py --check`)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS.
  `BASE_BRANCH=fe5ae6cd4a5f3c65d5d10f4d523f00e33b04ac47` (`git merge-base HEAD origin/develop`);
  `git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <the file-touch set> ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  → no match, and the same grep over this cycle's own diff
  (`git diff --unified=0 -- src/pcgen_import/sheet_rule/ …/token-mapping/`) → no match.
- **Wired-integration audit result:** OK_NO_TOKENS on this cycle's own diff
  (`\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b` over
  `git diff --unified=0 -- src/pcgen_import/sheet_rule/ …/token-mapping/` → no match). Over the
  whole file-touch set since `fe5ae6cd4a` the same grep matches only **generated Paizo prose
  inside `data/sheet_rules/`** — *"creatures must hack or force a way through"* (`plant_growth`),
  *"[Change to magical beast and stacking restriction not yet implemented]"* (a bracketed
  editorial note in the upstream `ui_abilities_class.lst:393` DESC), PCGen's own
  *"no selection"* placeholder rows — plus the word `placeholder` inside a mapping-table `rule`
  string. Identical to the finding `AT-35-E3-002_cycle1_receipt.md` recorded; no shipping code
  path is involved.
- **Acceptance criterion** (verbatim, `epic-breakdown.md § AT-35-E4-001`):
  > 4,334 units at authoring: ability 1,483, race_trait 697, spell 558, feat 518,
  > equipment_modifier 443, template 305, trait 123, equipment 99, domain 67, skill 29, deity 9,
  > race 3. Each cycle adds **converter mapping rows** for one token family (`BONUS:SKILL|…`,
  > `BONUS:SAVE|…`, `DR:`, `SR:`, `SPELLS:`, `TEMPBONUS:` → `Text` with its condition, `%CHOICE`
  > aliases → `Choice`), re-runs the converter, regenerates. Every new `Number` mapping is
  > oracle-checked on the fixture roster in the same cycle.
  >
  > **Evidence:** `completion_atlas.py --check` reports M at 0; `token-coverage.json` shows every
  > compute-bearing token type with a mapping row or a named refusal with count; the oracle
  > comparison per cycle with disagreements named.
  >
  > **Inherited from AT-35-E2-005 (2026-09-08, `decisions.md §16`, `### AT-35-E2-005-DISPOSITION`):**
  > the **659** converter-refused non-DONE units of 1,404 at `38b67db94e` … This criterion's bar
  > is therefore "M at 0 **and** the refused set at 0", not M alone.
- **Receipt rows (mechanical):**
  ```
  since=07e29075b4453605f1dcdd06f21ae4b1fd7deef1 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=131 ratio=n/a builds_recorded=0 pcgen_live_files=260
  ```
  `closed=0` / `relabeled=0` is correct and expected: the unit population was **already** 0
  non-DONE when the cycle started (`completion_atlas.py --check` → `DONE: 49438`). `ratio` is
  `n/a`, a division by zero, never `0.0`. `builds_recorded=0` is the counter's reading; this
  cycle did pay four cargo builds and four corpus-scale runs (wall times below) — the counter
  tracks the repo's build-number file, which a converter/inventory cycle does not bump.
- **PCGen residue:**
  ```
  live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Unchanged at start and at end. Every change this cycle made is on the **converter** side
  (`src/pcgen_import/`); nothing new on the live side reads a PCGen token.
- **Oracle parity:** `compared=146 agree=145 disagree=1` (lines) and
  `compared=382 agree=376 disagree=6` (chassis), `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  — `AT-35-E4-001_cycle1_sheet-parity.json`. **No new mapping row is a `Number`**, so no new
  Number mapping was owed an oracle check; the run was made anyway because 127 units stopped
  degrading and began rendering a magnitude. Comparable **lines rose 42 → 146** and agreements
  **41 → 145**; the disagreement set is byte-identical to
  `epic-2-sheet-rule/oracle-parity/sheet-parity.json`'s — **7 disagreements, all pre-existing,
  0 introduced, 0 fixed**:
  - `deterministic_human_fighter_l1` `target:WeaponAttack:{"Chosen":"core_rulebook:feat:weapon_focus"}` ours=0 oracle=1 (`WEAPON.0.TOTALHIT-ATTACK.MELEE.TOTAL`)
  - `halfling_fighter_l1` `save.fortitude.total` / `save.reflex.total` / `save.will.total` ours=2/1/2 oracle=3/2/3 (`CHECK.{0,1,2}.TOTAL`)
  - `human_paladin_l10` `save.fortitude.total` / `save.reflex.total` / `save.will.total` ours=6/3/9 oracle=9/6/12 (`CHECK.{0,1,2}.TOTAL`)

  These are AT-35-E4-002's population (the halfling's racial save bonus and the paladin's
  divine grace are chassis rows, not sheet-rule lines); this cycle neither caused nor cleared
  them.
- **Movement, four buckets:**
  - **closure (into DONE):** 0 — the population was already 0 non-DONE at the cycle start.
  - **relabel (bucket to bucket):** 0 unit changed bucket. **127 units changed evidence** inside
    `sheet-complete`: 126 `sheet_rule_rendered:words` → `sheet_rule_rendered:number` and 1
    → `sheet_rule_rendered:dice`. Those records previously **degraded** — the converter dropped
    their partly-read magnitudes and printed words — because a head in their closure had no
    table row, not because any term was unreadable.
  - **reachability:** `degraded_records` **974 → 603** (371 records). All 25 degradation shapes
    the 25 unmapped heads produced are gone; `degraded_by_token_type` now contains **no**
    `unmapped:*` key. `refused` is unchanged at **142**, all `no_corpus_record`.
  - **instrument-correction:** `token-coverage.json` `unmapped_token_types` **25 → 0** and
    `token_types` 232 → 231 (the `unmapped:GLOBALVAR:ABILITY` census key folds into `ABILITY`).
    One `correction` retro event (`1788937257113-at-35-e4-001-faa72b`).
- **Refused tokens:** **none** — this cycle added no refusal and cleared none. `_refused.json`
  is unchanged at 142 records, one shape (`no_corpus_record`), `refused_non_done=0`. No mapping
  row added here is `REFUSE`; the two PI shapes it touched are `decisions.md §15` R2's
  omit-and-stamp, which is not a refusal.
- **Discoveries:**
  1. **The unmapped-head set was a table gap, not a rules gap.** 25 token types carried by 974
     records had no mapping row and no named refusal; every one of them reads cleanly once the
     row exists. Emitted as the `correction` event above.
  2. **Two PI paths degraded where `decisions.md §15` R2 says print-the-words.** A `PREMULT`
     body whose nested sub-token is redacted whole (no `<HEAD>:<body>` to split) fell to
     `prereq.rs`'s `unmapped:` arm instead of `Situational{"requirement withheld"}`. The
     top-level path had always done it right; only the nested one was wrong (4 records).
  3. **`PRESPELLSCHOOL` had a live arm that no token could reach.** `prereq.rs:546` handled it
     for nested `PREMULT` bodies while the top-level path refused it for want of a table row —
     dead code that a compiler `unreachable_pattern` warning surfaced the moment the row landed.
- **Figures + their re-derive commands** (denominator: **49,438 units** / **49,438 corpus
  records** in `docs/work-inventory.json`, all books, unless stated):
  | figure | value | command |
  |---|---|---|
  | bucket M | **0** of 49,438 | `python3 scripts/completion_atlas.py --check` |
  | every non-DONE bucket | **0** (`DONE: 49438`) | same |
  | unmapped token types, before → after | **25 → 0** of 232 → 231 | `python3 scripts/token_coverage.py --check` |
  | converter-refused records | **142**, one shape, `refused_non_done=0` | same |
  | degraded records, before → after | **974 → 603** | `python3 -c "import json;print(json.load(open('data/sheet_rules/_report.json'))['degraded_records'])"` |
  | mapping-table rows, before → after | **249 → 273** (245 → 269 distinct) | `python3 -c "import json;r=json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/token-mapping/mapping-table.v1.json'))['rows'];print(len(r),len({x['token_type'] for x in r}))"` |
  | new rows that are `Number` | **0** of 24 | `python3 -c "import json;print([r['token_type'] for r in json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/token-mapping/mapping-table.v1.json'))['rows'] if r.get('provenance')=='synthesis' and r['maps_to'].startswith('Number')])"` |
  | units whose evidence changed | **127** of 49,438 (126 → `number`, 1 → `dice`) | diff `docs/work-inventory.json` against the cycle-start copy on `(status, evidence)` |
  | PCGen live-side files | **260** (baseline 260) | `python3 scripts/pcgen_residue_gate.py --check` |
  | `data/sheet_rules/` token leaks | **0** | `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ \| wc -l` |
  | oracle lines compared / agree / disagree | **146 / 145 / 1** | `scripts/oracle_harness/sheet_parity.py compare --ours <ours.json> --exports …/oracle-parity/exports --output …` |
  | corpus sweep | 48,706 records examined of 51,476 read, **0 findings**, `CLEAN` | `cargo run --locked --bin corpus_literal_sweep` |
  | derived-evaluator fixtures | 1,839 units over 2,580 rows, **0 failed** | `cargo run --locked --bin derived_evaluator_fixture_check` |

  Wall times paid (each `/usr/bin/time -f`): `corpus_literal_sweep` **154.39 s**,
  `derived_evaluator_fixture_check` **12.80 s**, `v06_work_inventory` **755.88 s**,
  `sheet_rule_convert` **110.9 s**, `sheet_rule_parity` **17.9 s**. The first, unguarded
  inventory run was **refused by the stamp-loss guard** (it would have dropped 7,385 of 32,617
  stamps) — the guard working as designed; the re-run supplied
  `CORPUS_LITERAL_SWEEP_REPORT` and `DERIVED_FIXTURE_CHECK_REPORT` and wrote cleanly.
  `--allow-stamp-loss` was never passed.
- **Build scope verified**, run at `9bae2cfa1f`:
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`
  - `cargo test --locked --lib -j 6` → `ok. 3220 passed; 0 failed; 14 ignored`
  - `cargo test --locked --no-fail-fast -j 6` → **8,754 passed, 0 failed, 67 ignored over 412 suites** after the self-heal below. The first run had **1 failure of 8,727**: `tests/sheet_rule_convert_gate.rs::token_census_names_the_row_for_every_token_and_the_head_under_each_refusal` asserted *`unmapped:STARTSKILLPTS` degrades the Arcanist* — an assertion pinning the very table gap this cycle closed, self-healed in the same commit (`workflow-instruction.md §8`, "a count assertion your own change moved"). It now asserts the new truth **and** the criterion's own bar: no census entry anywhere carries an `unmapped:` token type. Re-run of that suite: `28 passed; 0 failed`
  - `cargo clippy --locked --tests -j 6` → no `warning:` or `error:` line
  - `cargo run --locked --bin sheet_rule_convert -- --check` → exit 0
  - `cargo run --locked --bin corpus_literal_sweep` → `CLEAN`, 0 findings
  - `python3 scripts/pcgen_residue_gate.py --check` → PASS, 260 files (baseline 260)
  - `python3 scripts/completion_atlas.py --check` → `DONE: 49438`, every other bucket 0
  - `python3 scripts/token_coverage.py --check` → `verdict=PASS`, `unmapped_token_types=0`
  - `python3 scripts/shape_engine_boundary.py --check` → `not_held_by_engine=0`
  - `python3 scripts/missing_engine_tables.py --check` → `population=0 citation_failures=0`
  - `python3 scripts/denominator_gate.py --check …` → `files_checked=52 violations=0`
  - `scripts/verify.sh --only pi-sweep` → `PASS  pi-sweep  (11 hits over src/rules_core/rules_tables, 11 baseline rows)`
  - the desktop crate and the frontend did **not** run: this cycle touched no path under
    `apps/`. They run at the Epic 4 wrap-up.
- **Sweep population:** `corpus_literal_sweep` examined **48,706** records of 51,476 read before
  and after (no corpus record was written this cycle — `data/corpus/**` is untouched), 413,314
  tokens compared, 51,463 digests checked, **0 findings**.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`). Token semantics for the 24 new rows were read from the
  pinned checkout resolved through `$PCGEN_CORPUS_ROOT`, read-only.
- **Status:** complete
- **Notes:** The 25th head, `GLOBALVAR:ABILITY`, is a **head alias** onto the existing `ABILITY`
  row rather than a 25th mapping row — it is the ingest's own key for an `ABILITY` token body a
  global-variable contributor row added to a record, so it reads under the row that already
  describes it (`table.rs::row_for_head`, the `PRERACETYPE` precedent). Hence 24 rows for 25
  types. Blockers `B3`, `B4`, `B6`, `B8` are named as this criterion's or AT-35-E2-00x's owners
  in `blockers.md`; **B6**'s two PI residue paths are closed here (the nested redacted `PRE`
  sub-token, 4 records; the `[redacted PI]` head, which now carries a row). **B8**'s
  equipment/encumbrance shapes stay as they are: they degrade because a term is genuinely
  unreadable, not for want of a row, and under the sheet rule those records print their words
  and are DONE. **B3**'s `ABILITYCATEGORY` reader and **B4**'s character facts are unchanged —
  both are already-closed conditions in the sense that matters here (every unit they touch is
  DONE), and neither is reachable from a token type left unmapped.
- **Next-cycle scope:** criterion at zero — bucket M at 0, the converter-refused non-DONE set at
  0, and `token-coverage.json` `unmapped_token_types` at 0. No criterion other than
  AT-35-E4-001 was emptied by this cycle (every other bucket was already at 0 before it
  started), so no other kanban row changes state here.
