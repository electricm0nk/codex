# Cycle AT-35-E5-003_cycle1 — Epic 5 Residues / AT-35-E5-003

- **Commit SHA:** `6555dec327` (the converter scrub, its gate, the census script and the retro
  events), `<DOCS_SHA>` (this receipt, `progress.md`, `kanban.md`). Cycle start
  `aca9ac83babc7664f0581306cba2dbc62d22cce5`.
- **Scope gate:**
  ```
  inventory=docs/work-inventory.json
  scope=bucket=U|Z
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  Run as `python3 scripts/cycle_scope_gate.py --min 500 --bucket U --bucket Z`. The dispatch's
  mandatory-bundling instruction is moot and no bundling was done: `remaining_non_done=0` is the
  **whole corpus**, not merely U and Z, so the gate returns `PASS_WHOLE_REMAINDER`, not
  `FAIL_UNDER_FLOOR`, and there was nothing anywhere to bundle in. The same line comes back with
  no scope flags at all. Not an exemption claim — the gate ran and passed.
- **Files touched:**
  - `src/pcgen_import/sheet_rule/prose.rs` — `strip_editorial_not_implemented_markers` /
    `scrub_editorial_markers`, wired into both `DESC`-like and positional prose conversion
  - `src/pcgen_import/sheet_rule/convert.rs` — the same scrub on every line's label
  - `tests/sheet_rule_convert_gate.rs` — `package_prose_carries_no_upstream_editorial_marker`,
    the live-package gate (the converter gate belongs in the converter's own gate file; that
    file is where every other per-kind live-package gate already lives)
  - `data/sheet_rules/` — **166 rule files** regenerated through the converter, plus the new
    `_defects/editorial-marker-in-prose.json` (161 entries). No hand edit; no `data/corpus/`
    file changed at all.
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-003_buckets_u_z.py` (new — the
    command behind every figure below)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-003_cycle1_receipt.md` (new — this file)
  - `docs/retro/events/at-35-e5-003.jsonl` (new — 2 `correction` events)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` —
    `derived_at` stamp only, moved by running `completion_atlas.py --check`
  - `scripts/verify-baselines.env` — `BASELINE_ROOT_LIB_TESTS` 3220 → 3223 and
    `BASELINE_ROOT_FULL_TESTS` 8730 → 8734, each **+ exactly this cycle's own new tests**
    (3 lib + 1 integration). `workflow-instruction.md §8` names a count assertion moved by the
    cycle's own deliberate change as self-healable, in the same commit.
    `BASELINE_ROOT_TEST_BINARIES` (412) is untouched: `git diff --name-status aca9ac83ba..HEAD --
    'src/bin/*.rs' 'tests/*.rs'` lists one path and it is an `M`, not an `A`.
  - `docs/release/SD-35-corpus-sheet-completion/progress.md`, `docs/release/SD-35-corpus-sheet-completion/kanban.md`

  **Nothing on the live side changed** (`src/rules_core`, `src/saved_character`, `src/campaign`,
  `src/homebrew_authoring`, `apps/desktop` all untouched) — the fix is on the converter side,
  where `decisions.md §11` requires it.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — on this cycle's own diff over
  `src/ tests/ data/sheet_rules docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues`
  (`git add -N` first, so the new script is visible to `git diff`). The bundle-wide grep over
  `fe5ae6cd4a...HEAD` on the epic's scoped paths returns **1** hit, inside
  `AT-35-E1-004_cycle2`'s own audit row, which quotes the pattern in order to characterise it —
  the self-referential shape `AT-35-E5-001` and `AT-35-E5-002` recorded.
- **Wired-integration audit result:** `OK_NO_TOKENS` **on this cycle's own change**, and the
  cycle **removes** one of the bundle's standing hits: the single match on this cycle's own diff
  is a **deleted** line — `ultimate_intrigue:class_feature:courtly_hunter_courtly_companion`'s
  prose, whose `[Change to magical beast and stacking restriction not yet implemented]` aside is
  exactly what the scrub takes out. `AT-35-E5-002_cycle1_receipt.md` counted that record among
  the bundle-wide 11; it is now gone from `data/sheet_rules/`. The receipt below quotes the
  token list once, in this row, to characterise it.
- **Acceptance criterion** (verbatim, `epic-breakdown.md` `### AT-35-E5-003`):
  > ### AT-35-E5-003 — buckets U and Z reach zero
  >
  > U 202: per sub-cause, the instrument correction or a proven statement that the record carries
  > nothing a player reads. Z 19: `beginner_box` gets a compiled rule set through the guarded
  > generator path, then converts.
  >
  > **Evidence:** U and Z at 0; `corpus_literal_sweep` examined-count moved by exactly the
  > `beginner_box` record delta.
- **Receipt rows (mechanical):**
  ```
  since=aca9ac83babc7664f0581306cba2dbc62d22cce5 target_dir=/tmp/cargo-sd35-at-35-e5-003 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=187 ratio=n/a builds_recorded=6 pcgen_live_files=260
  ```
  `closed=0` is correct and expected: the criterion's 221 units closed in one earlier cycle
  (`26bdfa8d5b`, see **Movement**). This cycle moves no unit; it pays the criterion's unpaid
  Evidence obligations and fixes the defect that paying them exposed. **`builds_recorded=6` is a
  real overrun of `decisions.md §3`'s one-build target**, and it is sequential-prerequisite
  shaped, not sloppiness: the RED gate run and the GREEN one are two builds by definition; the
  live-evaluator render was run before and after the fix (two release builds of
  `sheet_rule_bucket_v_render`) because the census figures had to be re-derived on the fixed
  package; and the guarded inventory regen demands its own. The one-build target assumes a cycle
  that knows its change at the start; this one found the defect by measuring.
- **PCGen residue:**
  ```
  live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Run at cycle start **and** at the end, unchanged, and unchanged from
  `AT-35-E5-002_cycle1_receipt.md`'s 260. The converter-side fix cannot move it: the residue
  gate's roots are `src/rules_core`, `src/saved_character`, `src/campaign`,
  `src/homebrew_authoring` and `apps/desktop`, and this cycle wrote none of them.
- **Oracle parity:** N/A — no `Number` mapping row was added and no live path was touched. The
  change removes text from `prose` and from five `label`s; it moves no magnitude, so there is no
  value for the harness to compare differently. `data/sheet_rules/_report.json` after the run:
  `records=49438 converted=49296 refused=142 rules_written=69344 var_tables=5277
  degraded_records=603` — identical to `AT-35-E5-002_cycle1_receipt.md`'s figures on every field.

## The criterion's Evidence, paid in full

### Clause 1 — U and Z at 0

`python3 scripts/completion_atlas.py --check` at HEAD: `population=49438 buckets=10
unclassified=0 overlap=0`, `DONE 49438`, `U 0` and `Z 0` (and every other bucket 0),
`done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False
citation_failures=0`, exit 0.

Independently, from the criterion's own census script:
`python3 …/AT-35-E5-003_buckets_u_z.py` → `bucket_U_at_HEAD=0 bucket_Z_at_HEAD=0 verdict=PASS`.

### Clause 2 — U, per sub-cause: the instrument correction, or the record proven to carry nothing a player reads

Bucket U's 202 units at the `tranche/15` cut (`4c6c57eb9f`) carry **two** evidence strings, but
the first covers two materially different sub-causes — an item with no `DESC:` and a feat with no
`DESC:` — which the units' own `reason` field separates. Bucket Z's 19 carry one. **Four
sub-causes in all**, and no unit is outside them.

Re-derive the whole table with three commands:

```
python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-003_buckets_u_z.py --transitions
python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-003_buckets_u_z.py --proof
cargo run --locked --release --bin sheet_rule_bucket_v_render -- --units <the 221 cut-state ids> --output /tmp/lines.json \
  && python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-003_buckets_u_z.py --rendered /tmp/lines.json
```

| # | Sub-cause (evidence + `reason` at the cut) | Kind | Count at `4c6c57eb9f` | Branch | The disposition |
|---|---|---|---|---|---|
| 1 | `text_only_but_corpus_record_carries_no_description_to_show_a_player`, item `reason` | equipment 119 + equipment_modifier 21 | 140 | **instrument correction** | "No `DESC:` in the token closure" was measured on one field, and a sheet line is more than a description. All 140 convert; 69 render label **plus the rule's words** (the converter finds them in the closure or in a second-source `description`, `AT-35-E3-002`'s `description_only_rules`), 4 render a magnitude, 67 render the item's **name**, which is what a player writes on an equipment line |
| 2 | `feat_served_description_is_a_placeholder_marker_not_prose` | feat | 51 | **instrument correction, and the defect it named was real** | The served text was not clean player prose because upstream PCGen's own `[NOT IMPLEMENTED]`-family admission was inside it. **30 of the 51 still printed that marker at cycle start** — see **Discoveries**. This cycle strips it in the converter; all 51 now render the rule's own words (43) or the feat's name (8), and none carries a marker |
| 3 | `text_only_but_corpus_record_carries_no_description_to_show_a_player`, feat `reason` | feat | 11 | **7 instrument correction, 4 proven to carry nothing a player reads** | 1 renders label + prose, 6 render the feat's name. The other **4** (`ultimate_combat:feat:gundarme_bonus_feat`, `ultimate_magic:feat:skill_focus_intimidate` / `_knowledge_arcana` / `_swim`) carry the source record's own `print: false` and are **correctly absent from the sheet**: the corpus record's `description` is `null`, the converted rule's `prose` is `null`, and the record is a mechanical linkage row. For those four the criterion's second branch holds literally — there is nothing a player reads, proven from the record, not asserted |
| 4 | `no_compiled_rule_set_for_book` (bucket Z) | equipment | 19 | **the compiled rule set, through the guarded generator path** | see Clause 3 |
| | **total** | | **221** | | **221 `sheet-complete` at HEAD, 0 anywhere else** |

`--transitions` → `sub_causes=4 uz_units=221`, every destination `sheet-complete /
sheet_rule_rendered:{words,number}`, `not_sheet_complete_at_HEAD=0 verdict=PASS`: **no U or Z
unit was lost, dropped, renamed away or relabelled into another non-DONE bucket.**

`--proof` → `missing_rule_file=0 rules_without_a_label=0 verdict=PASS`: every one of the 221 has
a real `data/sheet_rules/<book>/<kind>/<key>.json` with a non-empty label.

`--rendered` (the **live evaluator**, `render_sheet` over `data/sheet_rules/` — the same path the
desktop's `sheet_lines_for` takes) →
`totals label + magnitude=7 label + magnitude + prose=3 label + prose=124 label only=83
not-printed (source print:false)=4`, `editorial_marker_on_a_rendered_line=0 verdict=PASS`.

### Clause 3 — Z: `beginner_box` gets a compiled rule set through the guarded generator path, then converts

`data/sheet_rules/beginner_box/equipment/` holds **19** compiled rules, one per Z unit, written by
`cargo run --locked --bin sheet_rule_convert` — the guarded generator path — at `72ad0be010`
(`git log --oneline 4c6c57eb9f..HEAD -- data/sheet_rules/beginner_box` names that one commit and
no other). **No `data/corpus/` file was hand-edited, then or now**: `git diff --name-only
4c6c57eb9f..HEAD -- data/corpus/beginner_box` is empty.

The evidence string `no_compiled_rule_set_for_book` was literally true before that commit and is
literally false after it. All 19 render: 11 label + prose, 5 label + magnitude, 2 label only,
1 label + magnitude + prose.

### Clause 4 — `corpus_literal_sweep`'s examined count moved by exactly the `beginner_box` record delta

```
python3 …/AT-35-E5-003_buckets_u_z.py --sweep-delta
book=beginner_box corpus_records_before=19 corpus_records_after=19 record_delta=0
  corpus_files_changed=0 compiled_rule_files_at_head=19
verdict=PASS
```

The delta is **0**, and the sweep's examined count moved by **0**:
`cargo run --locked --bin corpus_literal_sweep` at HEAD reports **`48706 records examined of
51476 read, 413314 tokens compared (9 synthesized), 51463 digests checked, 0 findings`,
`CLEAN`** — the identical number `progress.md` records before and after `AT-35-E3-002`. The
clause is met exactly, and the reason it is met at zero is the mechanism: a compiled rule set
lands in `data/sheet_rules/`, never in `data/corpus/`, so the sweep's population — corpus
records — cannot move. The 19 `beginner_box` records were already in it.

## Movement, four buckets

- *closure (into DONE, by id-set):* none **this cycle**. The criterion's 221 units closed in one
  earlier cycle, `26bdfa8d5b` (`AT-35-E3-002`, the whole-remainder cycle: U 202 → 0, Z 19 → 0),
  by the classifier's promotable-status widening — `unmeasurable` and `not-started` are
  pre-sheet-rule holding pens, and the rung's own three conditions still gated every promotion.
- *relabel (bucket to bucket):* none. `--transitions` proves **zero** U or Z units landed in a
  non-DONE bucket.
- *reachability:* none.
- *instrument-correction:* none to any counting instrument. One **package** correction: 166 rule
  files lose upstream PCGen's editorial not-implemented admission (161 in `prose`, 5 in `label`).
  Statuses are untouched by it — the guarded inventory regen produced a `generated_at`-only diff,
  which was reverted (the precedent `AT-35-E2-005` cycles 2–4 set).

- **Refused tokens:** none. `python3 scripts/token_coverage.py --check` at HEAD →
  `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1
  verdict=PASS`. The 142 refusals are all already-DONE records (`no_corpus_record`), unchanged
  from `AT-35-E5-002_cycle1_receipt.md`.
- **Discoveries (2, both emitted as `correction` retro events in
  `docs/retro/events/at-35-e5-003.jsonl`):**
  1. **The U sub-cause that is *defined* by the placeholder marker was closed while the marker
     was still on the sheet.** `AT-35-E3-002` dispositioned the 51 units of
     `feat_served_description_is_a_placeholder_marker_not_prose` to `sheet-complete`; **30 of
     them, and 166 package files corpus-wide, still printed upstream PCGen's own editorial
     admission** — `[NOT IMPLEMENTED]`, `[Not Implemented]`, `(NOT IMPLEMENTED)`, `[ML bonus not
     implemented.]`, and the mismatched-closer `[NOT IMPLEMENTED}` that
     `monster_codex:feat:vampiric_companion` ships — on the rendered line, plus five
     `mythic_adventures` templates carrying it in the record **name**
     (`Mythic Simple Template ~ Agile (Not Implemented)`). That is a statement about **PCGen's
     automation**, not the rule's words, and a paper sheet must never print it
     (`decisions.md §1`). It is leakage of the same class the existing `FORBIDDEN_LITERALS`
     scrub already removes, and the existing source-format gate did not look for it. Fixed on
     the converter side and gated: `1788980300753-at-35-e5-003-25094e`.
  2. **Four of the 221 never reach the sheet, and that is the criterion's second branch, not a
     gap.** `1788980300891-at-35-e5-003-e6ac04`; row 3 of the sub-cause table.
- **Figures + their re-derive commands:**
  | Figure | Value | Denominator | Command |
  |---|---|---|---|
  | bucket U at HEAD | 0 | of 49,438 corpus units | `python3 scripts/completion_atlas.py --check` |
  | bucket Z at HEAD | 0 | of 49,438 corpus units | `python3 scripts/completion_atlas.py --check` |
  | corpus DONE | 49438 | of 49438 | `python3 scripts/completion_atlas.py --check` |
  | bucket U at the `tranche/15` cut | 202 | of 49,438 | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-003_buckets_u_z.py --at 4c6c57eb9f` |
  | bucket Z at the `tranche/15` cut | 19 | of 49,438 | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-003_buckets_u_z.py --at 4c6c57eb9f` |
  | U/Z sub-causes | 4 | of the 221 cut-state U+Z units | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-003_buckets_u_z.py --transitions` |
  | U/Z units `sheet-complete` at HEAD | 221 | of the 221 present at the cut | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-003_buckets_u_z.py --transitions` |
  | U/Z units not `sheet-complete` at HEAD | 0 | of 221 | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-003_buckets_u_z.py --transitions` |
  | U/Z units with no rule file | 0 | of 221 | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-003_buckets_u_z.py --proof` |
  | U/Z units whose rendered line shows the rule's words | 127 | of 221 (124 label+prose, 3 label+magnitude+prose) | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-003_buckets_u_z.py --rendered <lines.json>` |
  | U/Z units whose rendered line is the label alone | 83 | of 221 | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-003_buckets_u_z.py --rendered <lines.json>` |
  | U/Z units the source's own `print: false` keeps off the sheet | 4 | of 221 | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-003_buckets_u_z.py --rendered <lines.json>` |
  | editorial marker on a rendered U/Z line | 0 | of 221 | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-003_buckets_u_z.py --rendered <lines.json>` |
  | package files carrying the editorial marker, before | 166 | of 49,296 rule files | `cargo test --locked --test sheet_rule_convert_gate package_prose_carries_no_upstream_editorial_marker` at `aca9ac83ba` (RED) |
  | package files carrying the editorial marker, after | 0 | of 49,296 rule files | `cargo test --locked --test sheet_rule_convert_gate package_prose_carries_no_upstream_editorial_marker` at HEAD (GREEN) |
  | package files this cycle rewrote | 166 | of 49,296 rule files | `git diff --stat aca9ac83ba..HEAD -- data/sheet_rules` |
  | `editorial-marker-in-prose` defect entries | 161 | of 49,438 records converted | `python3 -c "import json;print(len(json.load(open('data/sheet_rules/_defects/editorial-marker-in-prose.json'))))"` |
  | `beginner_box` corpus records, cut → HEAD | 19 → 19 | delta 0 | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-003_buckets_u_z.py --sweep-delta` |
  | `beginner_box` compiled rule files | 19 | of 19 Z units | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-003_buckets_u_z.py --sweep-delta` |
  | `corpus_literal_sweep` records examined | 48706 | of 51,476 read; 0 findings | `cargo run --locked --bin corpus_literal_sweep` |
  | converter-refused non-DONE at HEAD | 0 | of 0 non-DONE | `python3 scripts/token_coverage.py --check` |
  | bucket A population | 0 | of 2 kinds cited | `python3 scripts/missing_engine_tables.py --check` |
  | magnitude-bearing units not held by the engine | 0 | of 26,396 magnitude-bearing | `python3 scripts/shape_engine_boundary.py --check` |
  | PCGen live files | 260 | baseline 260 | `python3 scripts/pcgen_residue_gate.py --check` |
  | source-format markers in `data/sheet_rules/` | 0 | files matching | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | rust lines changed | 187 | this cycle, since `aca9ac83ba` | `python3 scripts/cycle_scope_gate.py --receipt --since aca9ac83babc7664f0581306cba2dbc62d22cce5 --before /tmp/wi-before-at-35-e5-003.json --after docs/work-inventory.json` |
- **Build scope verified:** `CARGO_TARGET_DIR=/tmp/cargo-sd35-at-35-e5-003`, `CARGO_INCREMENTAL=0`, `-j 6`.
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`
  - `cargo test --locked --lib -j 6` → `3223 passed; 0 failed; 14 ignored`
  - `cargo test --locked --no-fail-fast -j 6` → **8734 passed; 0 failed; 67 ignored**, over **412** test binaries plus doc-tests
    (413 `test result:` lines), exit 0, run at `6555dec327`
  - `cargo clippy --locked --tests -j 6` → `Finished dev profile` with **zero warnings** on the touched targets
  - `cargo run --locked --bin sheet_rule_convert` (the regeneration) then
    `-- --check` → `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277
    verdict=PASS (120.5s)`, `refused 142 no_corpus_record` — the refusal figure identical to
    `AT-35-E5-002_cycle1_receipt.md`
  - `cargo run --locked --bin corpus_literal_sweep` → `48706 records examined of 51476 read,
    0 findings`, `CLEAN` — run because the converter changed, though no corpus record did; it is
    also the guarded inventory regen's required input
  - `cargo run --locked --bin v06_work_inventory` (guarded, with `CORPUS_LITERAL_SWEEP_REPORT`
    and `DERIVED_FIXTURE_CHECK_REPORT` set; never `--allow-stamp-loss`) → 12 m 22 s, and the
    result is a **`generated_at`-only diff**, reverted: no unit's status or evidence moved
  - `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → `0`
  - `python3 scripts/completion_atlas.py --check` → exit 0; `token_coverage.py --check` →
    `verdict=PASS`; `shape_engine_boundary.py --check` → `not_held_by_engine=0`;
    `missing_engine_tables.py --check` → `population=0 kinds=0`; `pcgen_residue_gate.py --check`
    → `verdict=PASS`; `denominator_gate.py --check '…/*.md' '…/artifacts/**/*.md'` →
    `files_checked=62 violations=0`; `denominator_gate.py --check-provenance` →
    `files_checked=179 figures_examined=238 violations=0`
  - `scripts/verify.sh --only pi-sweep` → `PASS pi-sweep (11 hits over src/rules_core/rules_tables, 11 baseline rows)`,
    `RESULT: PASS` (1 stage passed)
  - desktop crate + frontend: **epic cadence** — this cycle touched no `apps/` path.
- **Sweep population:** `corpus_literal_sweep` examined **48706 → 48706** (of 51,476 read),
  `0 findings`, `CLEAN`. Moved by 0, which is exactly the `beginner_box` record delta.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` — carried in every
  converted record's `provenance.oracle_pin` and unchanged by this cycle. No figure in this
  receipt was derived from the pinned corpus.
- **Status:** complete
- **Notes:** Same shape as `AT-35-E5-001` and `AT-35-E5-002` cycle 1 — the criterion's population
  was already 0 at dispatch and kanban row 21 already read `complete`, but its per-sub-cause
  obligation had no artifact behind it. Paying it found a real defect on the sheet, which this
  cycle fixed in the converter rather than recording as a note. No other criterion's population
  was emptied by this cycle, so no other kanban row was closed. The dispatch's mandatory-bundling
  instruction did not apply: `remaining_non_done=0`.
- **Next-cycle scope:** criterion at zero.
