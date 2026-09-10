# Cycle AT-35-E5-002_cycle2 — Epic 5 Residues / AT-35-E5-002

- **Commit SHA:** `a7c2645913` (cycle start — the reclaim daemon's folded retro append), then this
  receipt + `progress.md` + `kanban.md` + the two retro corrections. Row 20 was already `complete`
  at dispatch; this is a **re-dispatch of an already-closed card** and it redoes nothing.
- **Scope gate:**
  ```
  inventory=docs/work-inventory.json
  scope=bucket=D
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  Run as `python3 scripts/cycle_scope_gate.py --min 500 --bucket D`, exit 0. The same line with no
  scope flags (`scope=(whole remainder)`) is identical. **The dispatch's mandatory-bundling
  instruction is moot**: `remaining_non_done=0` is the whole corpus, not merely bucket D, so the
  gate returns `PASS_WHOLE_REMAINDER` rather than `FAIL_UNDER_FLOOR` and there is nothing anywhere
  to bundle in. Not an exemption claim — the gate ran and passed.
- **Files touched:**
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-002_cycle2_receipt.md` (new — this file)
  - `docs/retro/events/at-35-e5-002.jsonl` (2 `correction` events appended)
  - `docs/release/SD-35-corpus-sheet-completion/progress.md`, `docs/release/SD-35-corpus-sheet-completion/kanban.md`
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` — `derived_at`
    stamp only, moved by running `completion_atlas.py --check`
  - `docs/retro/events/root.jsonl` (the reclaim daemon's 16:00Z append, folded at cycle start),
    `docs/retro/events/sd31-transcribe.jsonl` (`verify.sh --only pi-sweep`'s own run event)

  **No Rust, no `data/`, no `apps/`, no `scripts/` file changed.** This cycle moves no unit.
- **Identifier audit result:** **6 matches, all pre-existing, none this cycle's.** All six are
  references to **existing test filenames** in prose or doc comments, which the pattern
  `sd[0-9]+_` cannot distinguish from a bundle-tagged identifier: `src/rules_core/` 2
  (`tests/sd27_feat_prerequisite_enforcement.rs` on a removed line,
  `tests/sd34_wave51_racial_sla_catalog_matches_the_corpus.rs` on an added one) and
  `artifacts/epic-5-residues/` 4 (three of them inside `AT-35-E5-003`/`AT-35-E5-004` receipt rows
  that quote `tests/sd27_alternate_racial_trait_reachability.rs` while characterising it as
  pre-existing). At cycle 1's commit `f0a7e62e36` the same audit returned **0** — every one of the
  six landed in the later Epic 5 cycles, not here. `OK_NO_BUNDLE_TAGS` **on this cycle's own
  change**: this cycle adds no identifier at all — the **3** matches this receipt itself
  contributes are the audit row above quoting those same pre-existing test filenames in order to
  characterise them, self-referential audit prose of the shape `AT-35-E5-001_cycle1_receipt.md`
  and `AT-35-E4-003`'s receipt already recorded.
- **Wired-integration audit result:** **17 matches at HEAD, all pre-existing, 0 from this cycle**,
  attributed exhaustively:
  - `data/sheet_rules/` **2** — English rule prose transcribed from the corpus, on added lines:
    `bestiary_3:monster_ability:tophet_swallow_whole`, `core_rulebook:spell:plant_growth`.
  - `docs/work-inventory.json` **3** — **removed** lines. They are PCGen's own CHOOSE-menu "no
    selection" **placeholder** row reason strings for the Barbarian, Monk and Rogue classes; they
    were present in `develop`'s base `fe5ae6cd4a` and were **deleted** by SD-35. `git show
    HEAD:docs/work-inventory.json | grep -c` returns **0**.
  - `artifacts/epic-5-residues/` **12** — receipt prose in six files that quotes the token
    alternation in order to characterise it: `AT-35-E5-001_cycle1` 5, `AT-35-E5-001_cycle2` 1,
    `AT-35-E5-002_cycle1` 1, `AT-35-E5-003_cycle1` 2, `AT-35-E5-004_cycle1` 1,
    `AT-35-E5-005_cycle1` 2.
  - `src/rules_core/`, `class_feature_pool_picker.rs`, `LevelUpDialog.tsx`,
    `data/corpus/beginner_box/` — **0** each.

  No stub in shipping code; this cycle ships no code. The **4** further matches this receipt
  itself contributes are the audit row above and the **Figures** table's own re-derive command,
  both of which have to quote the token alternation to be readable and runnable — the same
  self-referential audit prose `AT-35-E5-001_cycle1_receipt.md`, `AT-35-E5-002_cycle1_receipt.md`
  and `AT-35-E4-003`'s receipt each recorded. **Cycle 1's row is corrected** — see
  **Discoveries**.
- **Acceptance criterion** (verbatim, `epic-breakdown.md` `### AT-35-E5-002`):
  > ### AT-35-E5-002 — bucket D reaches zero, sub-cause by sub-cause
  >
  > 1,982 at authoring: template 595, class_feature 471, deity 408, race_trait 183, ability 108,
  > language 81, domain 80, class 29, skill 21, trait 6. The largest sub-cause,
  > `class_feature_of_unmodelled_corpus_class` (634 units, 60 classes), is **one generic class
  > chassis converted from corpus `CLASS` records** (hit die, progressions, class skills, level
  > table) into our schema — not sixty hand-written functions (`decisions.md §7`). `deity` renders
  > as `Text`. Every sub-cause enumerated by `completion_atlas.py --by-evidence`.
  >
  > **Evidence:** D at 0; every sub-cause named with its mechanism and count.
- **Receipt rows (mechanical):**
  ```
  since=a7c2645913bba331718ada7bba2f5774ee3dd4b4 target_dir=... residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=253
  ```
  `closed=0` is correct and expected — the criterion's 1,982 units closed across two earlier
  cycles (see **Movement**), and cycle 1 already paid the second Evidence clause.
  `pcgen_live_files=253`, down from cycle 1's `260`: Epic 6's fall, never a rise.
- **PCGen residue:**
  ```
  live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Below cycle 1's `260 / 12736` — `AT-35-E6-001` took `PcgenFormulaEvaluator`, `bonus_stack_reader`
  and `pre_tokens` to `files=0 hits=0`. Never above the previous receipt's.
- **Oracle parity:** N/A — no `Number` mapping added, no live path touched. The converter ran only
  in `--check` mode, which regenerates nothing.

## The criterion's Evidence, re-derived at HEAD

### Clause 1 — D at 0

`python3 scripts/completion_atlas.py --check`, exit 0:
`population=49438 buckets=10 unclassified=0 overlap=0`; `DONE 49438`; `A 0 B 0 C 0 D 0 M 0 V 0
U 0 X 0 Z 0`; `done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False
citation_failures=0`.

### Clause 2 — every sub-cause named, with its mechanism and count

Re-run of cycle 1's census at HEAD, **byte-identical in every count**:
`python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-002_bucket_d_sub_causes.py --transitions`
→ `families=14 d_units=1982 … not_sheet_complete_at_HEAD=0 verdict=PASS`.

The 58 per-class `class_feature_of_unmodelled_corpus_class:<class>` strings stay collapsed into
one family row: they are one mechanism, not 58 (`decisions.md §7` — one generic class chassis,
never sixty hand-written functions).

| # | Sub-cause (evidence string at the cut) | Kind | Count at `4c6c57eb9f` | Mechanism that cleared it | Where its units stand at HEAD |
|---|---|---|---|---|---|
| 1 | `template_content_table_holds_zero_magnitude_record_pending_wiring_class_review` | template | 595 | converter renders the corpus record as a sheet line (`AT-35-E3-001` cycle 2 term-level degradation, then `AT-35-E3-002`) | 530 `sheet_rule_rendered:words`, 65 `dice` |
| 2 | `class_feature_of_unmodelled_corpus_class:*` (58 distinct classes) | class_feature | 446 | the generic class chassis over corpus `CLASS` records — one conversion, not 58 functions | 319 `words`, 127 `number` |
| 3 | `deity_content_table_holds_zero_magnitude_record_pending_wiring_class_review` | deity | 408 | renders as `Text`, exactly as the criterion specifies | 408 `words` |
| 4 | `race_trait_generic_table_holds_zero_magnitude_record_pending_wiring_class_review` | race_trait | 157 | converter renders the record | 153 `words`, 3 `dice`, 1 `number` |
| 5 | `ability_content_table_holds_zero_magnitude_record_pending_wiring_class_review` | ability | 108 | converter renders the record | 108 `words` |
| 6 | `language_content_table_holds_zero_magnitude_record_pending_wiring_class_review` | language | 81 | converter renders the record (name + `Spoken`/`Written`/`Read` tags) | 81 `words` |
| 7 | `domain_content_table_holds_zero_magnitude_record_pending_wiring_class_review` | domain | 80 | converter renders the record | 80 `words` |
| 8 | `class_modelled_but_no_observed_delta_on_the_rendered_snapshot` | class | 29 | the class chassis puts hit die / progressions / class skills on the sheet, so the line exists whether or not a probe observed a delta | 29 `number` |
| 9 | `class_feature_no_dedicated_magnitude_id_matched_the_record_slug` | class_feature | 25 | slug-match was the wrong question; the converted rule carries the value | 20 `words`, 5 `number` |
| 10 | `skill_content_table_holds_zero_magnitude_record_pending_wiring_class_review` | skill | 21 | converter renders the record | 21 `words` |
| 11 | `race_trait_skinwalker_change_shape_option_resolves_real_kin_pool_but_no_activation_mechanism_computes_its_magnitude` | race_trait | 19 | under the sheet rule an unactivatable option prints its own words; no activation engine is owed | 13 `words`, 6 `number` |
| 12 | `trait_content_table_holds_zero_magnitude_record_pending_wiring_class_review` | trait | 6 | converter renders the record | 5 `words`, 1 `number` |
| 13 | `race_trait_record_loaded_but_never_applies` | race_trait | 6 | the record is printed on the sheet; "never applies" was an engine-reachability claim, not a sheet claim | 6 `words` |
| 14 | `race_trait_template_bonus_language_grant_verified_but_has_no_upstream_activation_gate` | race_trait | 1 | same as row 11 | 1 `words` |
| | **total** | | **1982** | | **1982 `sheet-complete`, 0 anywhere else** |

`not_sheet_complete_at_HEAD=0 verdict=PASS`: **no D unit was lost, dropped, renamed away or
relabelled into another non-DONE bucket.** Every one of the 1,982 ids present at the cut is
present at HEAD carrying `status=sheet-complete`.

`completion_atlas.py --by-evidence` at HEAD enumerates one bucket only — `bucket=DONE
population=49438 distinct_evidence=155`, headed by `sheet_rule_rendered:words` 17,728 — because D
is empty. The historical enumeration the criterion's last sentence asks for is what the census
script above reproduces, reading each historical inventory out of git and partitioning it with the
**live** atlas bucket rule.

**Rendered-line spot check, re-run** (cycle 1's example, unchanged at HEAD):
`data/sheet_rules/bestiary_6/deity/tawil_at_umr.json` → `value=Text`, `applies=Always`,
`print=true`, one `grants[0].FactDeclare {name: "Symbol", value: "Black spiral inside of a
hexagon"}`, `provenance.pi.declared=["name"]` (the R2 PI-redacted shape),
`provenance.oracle_pin=7f818006e…`. A real rule, not an empty one.

### The census by state

| State | SHA | D | non-DONE |
|---|---|---|---|
| `tranche/15` cut | `4c6c57eb9f` | 1982 | 23315 |
| AT-35-E2-005 first corpus-wide conversion | `51f91bba11` | 43 | 1404 |
| AT-35-E3-001 cycle 2 (term-level degradation) | `406003afc3` | 0 | 786 |
| AT-35-E3-002 (whole remainder) | `26bdfa8d5b` | 0 | 0 |
| HEAD (this cycle) | `a7c2645913` | 0 | 0 |

- **Movement, four buckets:**
  - *closure (into DONE, by id-set):* none this cycle. The criterion's 1,982 closed in two earlier
    cycles: 1,939 at `51f91bba11` (`AT-35-E2-005`'s first corpus-wide conversion, D 1,982 → 43) and
    the last 43 at `406003afc3` (`AT-35-E3-001` cycle 2's term-level degradation, D 43 → 0).
  - *relabel (bucket to bucket):* none. The transitions run proves zero D units landed in a
    non-DONE bucket.
  - *reachability:* none.
  - *instrument-correction:* none to any counting instrument. Two **document** corrections, below.
- **Refused tokens:** none. `token_coverage.py --check` → `refused_non_done=0` at HEAD (142 refused
  records, all DONE; `census_refused=142 refused_json=142 union_over_token_types=142`).
- **Discoveries:** two, both document figures, neither a token type nor an atlas category, and
  neither reached code or an instrument.
  1. **The dispatch prompt's census is stale** (`1789061620111-at-35-e5-002-cb557c`). It states
     bucket D at 1,982 "at authoring" and a live remainder of `A:1 B:437 C:79 D:43 M:63 U:202
     V:392 X:168 Z:19`, and calls bundling mandatory. At HEAD every bucket is **0**, kanban row 20
     already read `complete`, and the gate returns `PASS_WHOLE_REMAINDER`. Recurrence key
     `stale-census-in-dispatch-prompt` — the **6th** criterion re-dispatched after closure
     (`AT-35-E4-002`, `AT-35-E4-003`, `AT-35-E5-001`, `AT-35-E3-003`, `AT-35-E2-005`, this one).
  2. **Cycle 1's wired-integration audit row is wrong on its `docs/work-inventory.json` clause**
     (`1789061629075-at-35-e5-002-6bfd84`). It reported 11 hits and said the inventory's 3 were
     "the same three corpus records" as `data/sheet_rules/`'s 3. They are not: they are **removed**
     lines carrying PCGen's own CHOOSE-menu "no selection" placeholder reason strings for
     Barbarian, Monk and Rogue, inherited from `develop`'s base and deleted by SD-35 — the
     inventory carries **0** at HEAD. `data/sheet_rules/` carries **2**, not 3 (the
     `courtly_hunter_courtly_companion` editorial marker went with `AT-35-E3-002` cycle 2's
     removal of the marker from the package). Cycle 1's total was also measured before its own
     receipt was committed: 11 pre-commit, 12 at `f0a7e62e36`. The corrected reading changes
     nothing about the verdict — every hit was and is pre-existing prose, and no stub ships.
- **Figures + their re-derive commands:**
  | Figure | Value | Denominator | Command |
  |---|---|---|---|
  | bucket D at HEAD | 0 | of 49,438 corpus units | `python3 scripts/completion_atlas.py --check` |
  | corpus DONE at HEAD | 49438 | of 49,438 | `python3 scripts/completion_atlas.py --check` |
  | distinct DONE evidence strings at HEAD | 155 | of 49,438 DONE units | `python3 scripts/completion_atlas.py --by-evidence` |
  | `sheet_rule_rendered:words` at HEAD | 17728 | of 49,438 DONE units | `python3 scripts/completion_atlas.py --by-evidence` |
  | bucket D at the `tranche/15` cut | 1982 | of 49,438 | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-002_bucket_d_sub_causes.py --at 4c6c57eb9f` |
  | D sub-cause families at the cut | 14 | of 1,982 D units (58 per-class strings collapsed to one family) | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-002_bucket_d_sub_causes.py --transitions` |
  | D units `sheet-complete` at HEAD | 1982 | of the 1,982 present at the cut | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-002_bucket_d_sub_causes.py --transitions` |
  | D units not `sheet-complete` at HEAD | 0 | of 1,982 | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-002_bucket_d_sub_causes.py --transitions` |
  | `class_feature_of_unmodelled_corpus_class` units at the cut | 446 | of 1,982 D units | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-002_bucket_d_sub_causes.py --transitions` |
  | distinct classes in that family | 58 | of the 446 units | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-002_bucket_d_sub_causes.py --at 4c6c57eb9f` |
  | remaining non-DONE at HEAD | 0 | of 49,438 | `python3 scripts/cycle_scope_gate.py --min 500 --bucket D` |
  | converter-refused non-DONE at HEAD | 0 | of 142 refused records | `python3 scripts/token_coverage.py --check` |
  | converter token types at HEAD | 231 | mapped, `uncovered=0` | `python3 scripts/token_coverage.py --check` |
  | bucket A population | 0 | of 0 kinds cited | `python3 scripts/missing_engine_tables.py --check` |
  | magnitude-bearing units not held by the engine | 0 | of 26,396 magnitude-bearing | `python3 scripts/shape_engine_boundary.py --check` |
  | PCGen live files | 253 | baseline 260 | `python3 scripts/pcgen_residue_gate.py --check` |
  | PCGen live hits | 12256 | baseline 12736 | `python3 scripts/pcgen_residue_gate.py --check` |
  | source-format markers in `data/sheet_rules/` | 0 | files matching | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | converter records converted | 49296 | of 49,438 records, `refused=142` | `cargo run --locked --bin sheet_rule_convert -- --check` |
  | identifier-audit matches over the epic's scoped paths | 6 | all pre-existing test filenames, 0 new identifiers from this cycle (3 self-referential quotes in this receipt) | `git diff --unified=0 "$(git merge-base HEAD origin/develop)...HEAD" -- <scoped paths> \| grep -cE '\b(sd[0-9]+_\|SD[0-9]+_\|Sd[0-9]+\|t_[0-9a-f]{8,})'` |
  | wired-integration matches over the epic's scoped paths | 17 | all pre-existing, 0 shipping-code hits from this cycle (4 self-referential quotes in this receipt) | `git diff --unified=0 "$(git merge-base HEAD origin/develop)...HEAD" -- <scoped paths> \| grep -cE '\b(STUB\|MOCK\|placeholder\|not yet implemented\|todo\|fixme\|hack)\b'` |
  | wired-integration matches in `docs/work-inventory.json` at HEAD | 0 | of the 3 in `develop`'s base, all deleted | `git show HEAD:docs/work-inventory.json \| grep -cE '\b(STUB\|MOCK\|placeholder\|not yet implemented\|todo\|fixme\|hack)\b'` |
  | rust lines changed | 0 | this cycle, since `a7c2645913` | `python3 scripts/cycle_scope_gate.py --receipt --since a7c2645913bba331718ada7bba2f5774ee3dd4b4 --before /tmp/wi-before-at-35-e5-002.json --after docs/work-inventory.json` |
  | denominator-gate figures examined | 378 | `violations=0`, with this receipt in | `python3 scripts/denominator_gate.py --check-provenance` |
  | root filesystem in use at build time | 79% | of 1.5T | `df -h /` |
- **Build scope verified:** `CARGO_TARGET_DIR=/tmp/cargo-sd35-at-35-e5-002`, `CARGO_INCREMENTAL=0`, `-j 6`.
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0` (cold, wall clock **2:55.83**, max RSS 2,447,372 kB)
  - `cargo run --locked --bin sheet_rule_convert -- --check` → exit 0,
    `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS (111.6s)`
    — identical to cycle 1's line
  - `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → `0`
  - `completion_atlas.py --check` exit 0; `token_coverage.py --check` → `verdict=PASS`;
    `shape_engine_boundary.py --check` → `not_held_by_engine=0`; `missing_engine_tables.py --check`
    → `population=0 kinds=0`; `pcgen_residue_gate.py --check` → `verdict=PASS`;
    `denominator_gate.py --check '…/*.md' '…/artifacts/**/*.md'` → `files_checked=81 violations=0`;
    `denominator_gate.py --check-provenance` → `files_checked=198 figures_examined=365 violations=0`;
    `./scripts/publish-site-dashboard.sh --check-pin` → pin matches
    (`5a0a0787312b5181d41214cb52abcd6e0c250fc409a75675ed6e839b4142e36f`), exit 0
  - `scripts/verify.sh --only pi-sweep` → `RESULT: PASS` (1 stage), logs `/tmp/codex-verify-GiAKsh`
  - `cargo clippy`: not run — no Rust target changed, so there is no touched target to lint
  - **Full workspace suite: not re-run, by attribution, not by omission.** `git diff --name-only
    b90f5e90c2..HEAD` lists only `docs/` paths, and this cycle changes no Rust, no `data/`, no
    `apps/` and no `scripts/` file at all — there is no change for a suite run to attribute a
    failure to. The last full green is the Epic 5 wrap-up correction cycle's **49 of 49 PASS** in
    3,916 s. Precedent: cycle 1 and `AT-35-E5-001_cycle2_receipt.md`, both docs-only, recorded
    `builds_recorded=0` on the same reasoning.
  - desktop crate + frontend: **epic cadence** — this cycle touched no `apps/` path.
- **Sweep population:** N/A — no corpus record changed, so `corpus_literal_sweep` was not re-run
  (`workflow-instruction.md §6` step 3 runs it only when corpus records changed).
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` — carried in every
  converted record's `provenance.oracle_pin`; no figure in this receipt was derived from the
  pinned corpus.
- **Status:** complete
- **Notes:** A re-dispatch of an already-`complete` card (kanban row 20, cycle 1 `f0a7e62e36`).
  Both Evidence clauses re-derived at HEAD and both hold; the census re-ran identical in all 14
  family rows and all 1,982 unit transitions, so nothing was redone. The cycle's two products are
  the corrections: the dispatch's stale census, and cycle 1's mis-attributed
  `docs/work-inventory.json` wired-audit clause. No other criterion's population was emptied by
  this cycle, so no other kanban row was closed.
- **Next-cycle scope:** criterion at zero.
