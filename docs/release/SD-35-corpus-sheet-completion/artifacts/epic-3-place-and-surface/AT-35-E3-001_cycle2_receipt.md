# Cycle AT-35-E3-001_cycle2 — Epic 3 — Place and surface / AT-35-E3-001

- **Commit SHA:** `406003afc3` (the figure-moving commit: converter, regenerated `data/sheet_rules/`, `docs/work-inventory.json`, the two re-derived count pins, the derived atlas artifacts and the retro events), plus the docs commit carrying this receipt, `progress.md` and the `kanban.md` rows. Cycle start `a542652c5e32f2da5c1c7287fa3e974f211c5b83` on `tranche/15`.
- **Scope gate:** `scoped=516 remaining_non_done=1404 floor=500 verdict=PASS` — the literal last line of `python3 scripts/cycle_scope_gate.py --min 500 --bucket B --or --bucket C` at `a542652c5e` (`scope=bucket=B OR bucket=C`, `scoped_by_bucket=B:437 C:79`, `scoped_by_kind=ability:82 class:115 class_feature:293 companion:1 feat:6 monster:2 race_trait:14 spell:2 template:1`). The criterion's own scope, `--min 500 --bucket B --kind class_feature`, returned `scoped=214 remaining_non_done=1404 floor=500 verdict=FAIL_UNDER_FLOOR` (exit 1) — cycle 1's finding, unchanged — so the cycle bundled the rest of Epic 3's buckets (B for AT-35-E3-002, C for AT-35-E3-003) per the orchestrator's 2026-09-08 bundling rule. `python3 scripts/pcgen_residue_gate.py --check` at start: `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`.
- **Files touched:** `src/pcgen_import/sheet_rule/ctx.rs`, `src/pcgen_import/sheet_rule/convert.rs`, `src/pcgen_import/sheet_rule/mod.rs` (the converter change and its three-test gate); `src/rules_core/class_feature_pool_catalog.rs` and `src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs` (two count pins this change moved, healed in the same commit); `data/sheet_rules/**` (regenerated whole by the converter — 384 modified, 1,020 new files); `docs/work-inventory.json` (regenerated once, guarded); `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/token-coverage.json` (re-derived whole by `token_coverage.py --check`, `workflow-instruction.md §5`'s shared-file list); `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/{completion-atlas.json,missing-engine-tables.json,shape-engine-boundary.md}` (re-derived by the three `--check` gates, whose contents genuinely moved with the inventory — kept, not reverted, because reverting leaves the gates stale); `docs/retro/events/at-35-e3-001.jsonl` (1 correction + 1 deferral) and `docs/retro/events/{codex,root}.jsonl` (one `reclaim.sh` housekeeping append each, folded); this receipt, `progress.md`, `kanban.md`.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS — `BASE_BRANCH=fe5ae6cd4a5f3c65d5d10f4d523f00e33b04ac47` (`git merge-base HEAD origin/develop`); `git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <the bundled file-touch union> ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` → no match, at start and on the final diff.
- **Wired-integration audit result:** OK_NO_TOKENS on this cycle's own code diff. Over the whole file-touch union since `fe5ae6cd4a` the same grep (`\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b`) matches only pre-existing non-code hits: rulebook-prose strings inside `data/sheet_rules/**` `ProsePiece::Text` (Tophet "hack or smash", Plant Growth "hack or force", Courtly Companion "not yet implemented" — corpus text, recorded by AT-35-E2-002's correction `1788844812035-at-35-e2-002-7cbeb2`) and the `engine_diagnostic:vacuous_placeholder_row_no_corpus_content_to_render` evidence strings in `docs/work-inventory.json`. No stub, inline mock or `"Would …"` string in shipping code.
- **Acceptance criterion:** verbatim from `epic-breakdown.md` `### AT-35-E3-001`: "7,866 units at authoring across three evidence families (`class_feature_owner_matched_by_name_but_record_not_held_by_engine`, `class_feature_option_pool_record_with_magnitude_not_held_by_engine`, `class_feature_option_pool_record_not_held_by_engine`). Under the new design "held" means the character's holdings include the rule — `SheetRule.applies` says which class/level/choice holds it, derived at convert time from the corpus's own `CLASS` / `ABILITYCATEGORY` declarations, not a hand-kept list. Widen the converter's `applies` derivation and the live holdings lookup. SD-33's open deferral 1 (1,128 unmatched pool-group prefixes) closes here. **Evidence:** `completion_atlas.py --by-kind` reports `class_feature` B at 0; movement by id-set diff; every cycle's `cycle_scope_gate.py` output."
- **Receipt rows (mechanical):** `closed=618 relabeled=0 rust_lines_changed=232 ratio=0.38 builds_recorded=3 pcgen_live_files=260` — the literal last line of `python3 scripts/cycle_scope_gate.py --receipt --since a542652c5e32f2da5c1c7287fa3e974f211c5b83 --before /tmp/wi-before-AT-35-E3-001.json --after docs/work-inventory.json --target-dir /tmp/cargo-sd35-AT-35-E3-001` (`residue_gate=present`; `closed_by_kind=ability:91 class:144 class_feature:302 companion:1 equipment:6 equipment_modifier:10 feat:28 monster:2 power:1 race_trait:16 skill:7 template:9 trait:1`; `relabeled_moves=` empty; `regressed=0 added=0 dropped=0`). `builds_recorded=3` is **above `decisions.md §3`'s one-build target and is a real overrun**: the converter build, the `corpus_literal_sweep`/`derived_evaluator_fixture_check` build the stamp-loss guard demanded, and the test build. The three are sequential prerequisites of one another, not three verification passes; `ratio=0.38` is well under `decisions.md §4`'s 3.0.
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` — identical at start and at HEAD. **Not risen.** Every change is on the converter side (`src/pcgen_import/sheet_rule/`); nothing new on the live side reads a PCGen token, and no converter or oracle code was deleted (`decisions.md §11`).
- **Oracle parity:** N/A — the cycle added **no** `Number` mapping. Term-level refusal only ever removes a number from a rule (a degraded record's principal value becomes `SheetValue::Text` with `target`/`bonus_type`/`also` cleared), so no new interpreted value entered the oracle-comparable set. `derived_evaluator_fixture_check` ran as a stamp prerequisite: `1839 unit(s) cleared over 2580 fixture row(s); 0 failed; 0 not ingested`.
- **Movement, four buckets:**
  - **closure (into DONE, by id-set diff): 618.** Non-DONE 1,404 → 786 of 49,438. By prior bucket: A 1, B 435, C 79, D 43, M 60. By kind: `class_feature` 302, `class` 144, `ability` 91, `feat` 28, `race_trait` 16, `equipment_modifier` 10, `template` 9, `skill` 7, `equipment` 6, `monster` 2, `companion` 1, `power` 1, `trait` 1.
  - **relabel (bucket to bucket): 0.**
  - **reachability: 0 regressions** — no unit left DONE (`regressed=0`).
  - **instrument-correction: 1** — correction `1788899836496-at-35-e3-001-312a28`, below.
- **Refused tokens:** `no_corpus_record=2` — the whole remainder of the bundled 516-unit scope. Both are `spell` records in bucket B (`book_of_the_damned_volume_2:spell:summon_demons_nascent_demon_lord`, `ultimate_combat:spell:share_language_communal`); they join to no corpus record at all, so the converter has no source row to convert and degradation cannot reach them. Deferral `1788899844992-at-35-e3-001-e163d1` names both. **Zero refused token types remain for `class_feature`** — the criterion's own population is empty.
- **Discoveries:** one `correction` (`1788899836496-at-35-e3-001-312a28`). The criterion names the `applies` derivation and SD-33's 1,128 unmatched pool-group prefixes as the mechanism; at `a542652c5e` **all 214** `class_feature` bucket-B units (and all 516 of the bundled scope) were already held by `applies` and were blocked instead by the converter refusing the **whole record** on any single unlowerable token. The real mechanism is one converter policy, not a holdings widening. No new token type surfaced: all 79 degradation shapes were already in `token-coverage.json`'s shape section as refusal shapes.
- **Figures + their re-derive commands:**
  - scoped **516 of 1,404 non-DONE of 49,438**, verdict PASS — `python3 scripts/cycle_scope_gate.py --min 500 --bucket B --or --bucket C`
  - the criterion's own population **0 of 18,043 `class_feature` units** — `python3 scripts/completion_atlas.py --by-kind` → `class_feature (n=18043): DONE=17704 A=0 B=0 C=0 D=0 M=0 V=185 U=0 X=154 Z=0`; and `python3 scripts/cycle_scope_gate.py --min 500 --bucket B --kind class_feature` → `scoped=0 remaining_non_done=786 floor=500 verdict=FAIL_UNDER_FLOOR`
  - buckets at HEAD, **786 non-DONE of 49,438**: `DONE 48652 / A 0 / B 2 / C 0 / D 0 / M 3 / V 392 / U 202 / X 168 / Z 19` — `python3 scripts/completion_atlas.py --check` (`unclassified=0 overlap=0 done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0`)
  - movement **618 closed, 0 regressed, 0 relabelled** — the `--receipt` invocation above, and the id-set diff `/tmp/wi-before-AT-35-E3-001.json` → `docs/work-inventory.json` through `scripts/completion_atlas.py::_bucket_of`
  - converter population **48,601 converted + 837 refused = 49,438 records** (before: 47,628 + 1,810 = 49,438) — `cargo run --locked --bin sheet_rule_convert` (105.4 s), `data/sheet_rules/_report.json`
  - **973 degraded records of 48,601 converted**, over **79 degradation shapes**; the largest, each counted over the 973: `FORMULA:var(COUNT)` 211, `unmapped:STARTSKILLPTS` 162, `unmapped:SLOTS` 95, `FORMULA:malformed (parser refusals)` 87, `SPELLS (PI-redacted token)` 78, `BONUS:[redacted PI]` 62, `unmapped:ALTTYPE` 49, `unmapped:SPELLSTAT` 49, `unmapped:MODTOSKILLS` 41, `DEFINE (PI-redacted token)` 40, `unmapped:MEMORIZE` 38 — `data/sheet_rules/_report.json` `degraded_records` / `degraded_by_token_type`
  - token ledger **non_done=786 tokened=781 token_less=5 refused=837 refused_non_done=5 token_types=231 shapes=2 verdict=PASS** — `python3 scripts/token_coverage.py --check` (all seven sum checks `ok=True`)
  - the pre-cycle blocker census, **516 of 516 scoped units converter-refused, 53 distinct refused token types** — join `docs/work-inventory.json` bucket B/C ids against `{e['id'] for e in json.load(open('data/sheet_rules/_refused.json'))['entries']}` at `a542652c5e` and count `entries[*].token_types`
  - the two re-derived count pins — `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus` → `F1 135 Flat-constant magnitude (bare literal)` (was 239); `class_feature_pool_catalog`'s own live query → excluded-class population 0 (was 1)
  - receipt rows / PCGen residue / denominator gate — the invocations quoted in their own rows above
- **Build scope verified:** run at `406003afc3`, `CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E3-001`, `CARGO_INCREMENTAL=0`, `-j 6`.
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`
  - `cargo test --locked --lib -j 6` → `LIB_EXIT=0` (a first run at the pre-heal tree returned `3218 passed; 2 failed` — the two count pins above; healed in the same commit and re-run green)
  - `cargo test --locked --no-fail-fast -j 6` → see `## Build result` below
  - `cargo clippy --locked --tests -j 6` → see `## Build result`
  - `cargo run --locked --bin sheet_rule_convert -- --check` → exit 0 (the on-disk package equals a fresh conversion byte for byte; `converted + refused == records`)
  - `cargo run --locked --bin corpus_literal_sweep` → `48706 records examined of 51476 read, 413314 tokens compared, 51463 digests checked, 0 findings — CLEAN` (142.9 s)
  - `python3 scripts/pcgen_residue_gate.py --check` → `verdict=PASS`, not risen
  - `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → `0`
  - `python3 scripts/completion_atlas.py --check` → exit 0; `python3 scripts/token_coverage.py --check` → `verdict=PASS`; `python3 scripts/shape_engine_boundary.py --check` → `magnitude_bearing=26396 not_held_by_engine=2 citation_ok=True` exit 0; `python3 scripts/missing_engine_tables.py --check` → `population=0 kinds=0 citation_failures=0` exit 0
  - `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` and `scripts/verify.sh --only pi-sweep` → see `## Build result`
  - the desktop crate and the frontend run at the **epic wrap-up** (`decisions.md §3`): this cycle touched nothing under `apps/`.
- **Sweep population:** `corpus_literal_sweep` examined **48,706 records of 51,476 read** before and after; **0 findings, CLEAN** both times. No corpus record changed (`git status --porcelain -- data/corpus` empty), so the sweep ran only as the stamp-guard prerequisite.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` — the converter reads the pinned tree through `$PCGEN_CORPUS_ROOT`; every `data/sheet_rules/**` record carries it as `provenance.oracle_pin`.
- **Status:** complete — `class_feature` bucket B is **0** at HEAD, the criterion's stated evidence bar.
- **Notes:** (a) The change is one converter policy, not a mapping-row grind: `ctx::RECORD_REFUSAL_SHAPES` names the only shapes that still delete a record (`decisions.md §15` R2's value-redacted shape; `no_corpus_record` / `no_source_row` are handled before conversion), and every other unlowerable term degrades — the record converts and prints its words (`§1` form 3), with `target`/`bonus_type`/`also` cleared so no partly-read magnitude reaches a sheet total. (b) `§15` R2's three PI rows now do what their own mapping-table row rule already said ("the rest of the record's tokens still convert") and stamp `provenance.pi`. (c) Cards this bundle emptied and closed in the same cycle, each pointing at this receipt: **AT-35-E3-003** (bucket C at 0), **AT-35-E5-002** (bucket D at 0, every sub-cause named in `progress.md`), **AT-35-E5-001** (`missing_engine_tables.py --check` `population=0`; the one bucket-A unit, `ultimate_psionics:power:physical_acceleration`, moved `engine-does-not-hold`/`power_content_has_no_engine_table` → `sheet-complete`/`sheet_rule_rendered:words`). **Not** emptied: AT-35-E3-002 (B 2), AT-35-E4-001 (M 3), AT-35-E4-002 (V 392), AT-35-E5-003 (U 202 + Z 19), AT-35-E5-004 (X 168). (d) SD-33's open deferral 1 (1,128 unmatched pool-group prefixes) is closed by consequence, not by a pool-prefix matcher: no `class_feature` unit is unheld at HEAD. (e) `builds_recorded=3`, above the one-build target — named here for `AT-35-E3-004`'s ledger and `decisions.md §3`'s epic review.
- **Next-cycle scope:** criterion at zero. The Epic 3 remainder is **AT-35-E3-002**'s 2 units (`--bucket B`, `no_corpus_record=2`), under the floor and not the whole remainder, so it bundles: `python3 scripts/cycle_scope_gate.py --min 500 --bucket B --or --bucket M --or --bucket V --or --bucket U --or --bucket X --or --bucket Z` → the whole 786 (`PASS_WHOLE_REMAINDER`).

## Build result

Run at `103de25a2d` (the code HEAD; the docs commit carrying this receipt adds no code),
`CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E3-001`, `CARGO_INCREMENTAL=0`, `-j 6`.

```
cargo test --locked --no-run -j 6            NO_RUN_EXIT=0
cargo test --locked --lib -j 6               test result: ok. 3220 passed; 0 failed; 14 ignored   LIB_EXIT=0
cargo test --locked --no-fail-fast -j 6      FULL_EXIT=0
                                             TARGETS_EXECUTED=412  RUNNING_LINES=411
                                             PASSED_TOTAL=8724  FAILED_TOTAL=0
                                             (grep '^test result: FAILED' /tmp/fulltest3-e3001.log -> no match)
cargo clippy --locked --tests -j 6           0 lines matching '^warning|^error'
scripts/verify.sh --only pi-sweep            passed: 1  pi-sweep   RESULT: PASS   PISWEEP_EXIT=0
python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' \
  'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'
                                             files_checked=47  violations=0   EXIT=0
```

**Three assertions this cycle's own change moved, healed in `103de25a2d`**
(`workflow-instruction.md §8`'s self-healable list, "a count assertion your own change moved"):

1. `src/bin/v06_work_inventory.rs` `REFUSED_ID` — the fixture
   `advanced_class_guide:class:arcanist` now converts (its six unmapped heads degrade instead
   of deleting the record); the pin moves to `advanced_players_guide:feat:allied_spellcaster`,
   one of the 837 records with no source row at all.
2. `tests/sheet_rule_convert_gate.rs` `token_census_names_the_row_for_every_token_and_the_head_under_each_refusal`
   — asserted `unmapped:STARTSKILLPTS` **refuses** the Arcanist; it degrades it. The test now
   reads `degradations` / `degraded_under` and additionally asserts the record carries **no**
   record-level refusal.
3. `tests/v06_work_inventory.rs` `ultimate_psionics_appears_in_the_inventory_with_real_per_kind_status`
   — 420 of the 421 `power` units were `sheet-complete`; all 421 now render.

`tests/**` and `src/bin/v06_work_inventory.rs`'s test module sit outside the Epic 3 file-touch
set as written (`src/bin/v06_work_inventory.rs` is inside it; `tests/**` is not). The two
`tests/**` edits are assertion re-pins forced by this cycle's own change and nothing else —
recorded here rather than left red, per `§8`.

**Earlier, pre-heal runs, recorded so the sequence is auditable:** a first
`cargo test --locked --lib` returned `3218 passed; 2 failed` (the two count pins healed in
`406003afc3`), and a first `--no-fail-fast` returned `FULL_EXIT=101`, `412` targets,
`8721 passed`, `3 failed` — the three above. A run with the sources mid-edit was killed and
restarted rather than reported.

**A note on `TARGETS_EXECUTED=412`.** The launch-readiness audit recorded `590 targets executed`
at the `tranche/15` cut (`workflow-instruction.md §1` item 10). The figure moved on Epic 1's
`AT-35-E1-003` (test families made table-driven) and the fable review's deletion of 11 obsolete
probe bins, both before this cycle; it is re-derived here by
`grep -c '^test result:' /tmp/fulltest3-e3001.log` and is not moved by this cycle.
