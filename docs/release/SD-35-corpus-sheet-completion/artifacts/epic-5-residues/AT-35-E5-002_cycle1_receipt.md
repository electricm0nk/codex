# Cycle AT-35-E5-002_cycle1 — Epic 5 Residues / AT-35-E5-002

- **Commit SHA:** `7e517b9521` (the sub-cause census script + retro event), `<docs-sha>` (this receipt,
  `progress.md`, `kanban.md`). Cycle start `00d0611e87`.
- **Scope gate:**
  ```
  inventory=docs/work-inventory.json
  scope=bucket=D
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  Run as `python3 scripts/cycle_scope_gate.py --min 500 --bucket D`. The dispatch's mandatory
  bundling instruction is moot and no bundling was done: `remaining_non_done=0` is the **whole
  corpus**, not merely bucket D, so the gate returns `PASS_WHOLE_REMAINDER`, not
  `FAIL_UNDER_FLOOR`, and there was nothing anywhere to bundle in. Same line with no scope flags.
  Not an exemption claim — the gate ran and passed.
- **Files touched:**
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-002_bucket_d_sub_causes.py` (new — the
    command behind every figure below; reads each historical `docs/work-inventory.json` out of git
    and partitions it with the **live** `scripts/completion_atlas.py` bucket rule)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-002_cycle1_receipt.md` (new — this file)
  - `docs/retro/events/at-35-e5-002.jsonl` (new)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` — `derived_at`
    stamp only, moved by running `completion_atlas.py --check`
  - `docs/release/SD-35-corpus-sheet-completion/progress.md`, `docs/release/SD-35-corpus-sheet-completion/kanban.md`

  **No Rust, no `data/`, no `apps/`, no `scripts/` file changed.** This cycle moves no unit; it pays
  the unpaid half of the criterion's Evidence sentence.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — on
  `git diff --unified=0 "$(git merge-base HEAD origin/develop)...HEAD"` over the epic's scoped
  paths, and on this cycle's own new files in isolation (`git add -N` first — an untracked file is
  invisible to `git diff`; `AT-35-E4-003`'s correction `1788959112531-at-35-e4-003-d78240`).
- **Wired-integration audit result:** `OK_NO_TOKENS` **on this cycle's own change** — 0 hits in the
  new script, and exactly **1** in this receipt — line 176, the **Figures** table's own re-derive
  command, which has to quote the token alternation to be runnable. Self-referential audit prose,
  the same shape `AT-35-E5-001_cycle1_receipt.md` and `AT-35-E4-003`'s receipt recorded; no stub in
  shipping code, and this cycle ships no code at all. The bundle-wide diff over the epic's
  scoped paths reports **11** hits, every one of them pre-existing and attributed:
  `data/sheet_rules/` 3, `docs/work-inventory.json` 3 (the same three corpus records in both
  files — `core_rulebook:spell:plant_growth`, `bestiary_3:monster_ability:tophet_swallow_whole`,
  `ultimate_intrigue:class_feature:courtly_hunter_courtly_companion`, all English rule prose
  transcribed from the corpus), and `artifacts/epic-5-residues/` 5, all five inside
  `AT-35-E5-001_cycle1_receipt.md`'s own audit row, which quotes the token list in order to
  characterise it. `src/rules_core/`, the picker, `LevelUpDialog.tsx` and
  `data/corpus/beginner_box/` report **0** each.
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
  since=00d0611e8715aa656eba03daed19d799735ab0af target_dir=... residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=260
  ```
  `closed=0` is correct and expected: the criterion's 1,982 units closed across two earlier
  cycles (see **Movement**). `ratio=n/a` and `builds_recorded=0` follow — nothing outside `docs/`
  changed, so no build was owed; the one build actually run is under **Build scope verified**.
- **PCGen residue:**
  ```
  live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Unchanged from `AT-35-E5-001_cycle1_receipt.md`'s 260. Nothing on the live side changed at all.
- **Oracle parity:** N/A — no `Number` mapping added and no live path touched. The converter ran
  only in `--check` mode (the §6 step 3 gate), which regenerates nothing.

## The criterion's Evidence, paid in full

### Clause 1 — D at 0

`python3 scripts/completion_atlas.py --check` at HEAD:
`population=49438 buckets=10 unclassified=0 overlap=0`, `DONE 49438`, `D 0` (and every other
bucket 0), `done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False
citation_failures=0`, exit 0.

### Clause 2 — every sub-cause named, with its mechanism and count

Bucket D's population is enumerated at the `tranche/15` cut (`4c6c57eb9f`, the state the criterion
was written against) and traced unit-by-unit to its status at HEAD. **The 58 per-class
`class_feature_of_unmodelled_corpus_class:<class>` strings are collapsed into one family row**
because they are one mechanism, not 58 (`decisions.md §7`: one generic class chassis, never sixty
hand-written functions); the per-class breakdown is in the census output's cut column.

Re-derive both tables with one command:
`python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-002_bucket_d_sub_causes.py --transitions`

| # | Sub-cause (evidence string at the cut) | Kind | Count at `4c6c57eb9f` | Mechanism that cleared it | Where its units stand at HEAD |
|---|---|---|---|---|---|
| 1 | `template_content_table_holds_zero_magnitude_record_pending_wiring_class_review` | template | 595 | converter renders the corpus record as a sheet line (`AT-35-E3-001` cycle 2 term-level degradation, then `AT-35-E3-002`) | 530 `sheet_rule_rendered:words`, 65 `sheet_rule_rendered:dice` |
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

`not_sheet_complete_at_HEAD=0 verdict=PASS` on the transitions run: **no D unit was lost, dropped,
renamed away or relabelled into another non-DONE bucket.** Every one of the 1,982 ids present at
the cut is present at HEAD carrying `status=sheet-complete`.

A rendered line was spot-checked for one live unit per family (14 of 14 have a real
`data/sheet_rules/<book>/<kind>/<key>.json`; e.g. `bestiary_6:deity:tawil_at_umr` →
`value=Text` with a `FactDeclare` Symbol row and `provenance.pi.declared=["name"]` — the R2
PI-redacted shape, not an empty rule).

### The census by state

| State | SHA | D | non-DONE |
|---|---|---|---|
| `tranche/15` cut | `4c6c57eb9f` | 1982 | 23315 |
| AT-35-E2-005 first corpus-wide conversion | `51f91bba11` | 43 | 1404 |
| AT-35-E3-001 cycle 2 (term-level degradation) | `406003afc3` | 0 | 786 |
| AT-35-E3-002 (whole remainder) | `26bdfa8d5b` | 0 | 0 |
| HEAD | `00d0611e87` | 0 | 0 |

Re-derive: `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-002_bucket_d_sub_causes.py`

- **Movement, four buckets:**
  - *closure (into DONE, by id-set):* none **this cycle**. The criterion's 1,982 closed in two
    earlier cycles: 1,939 at `51f91bba11` (`AT-35-E2-005`'s first corpus-wide conversion, which
    took D 1,982 → 43) and the last 43 at `406003afc3` (`AT-35-E3-001` cycle 2's term-level
    degradation, D 43 → 0). Both are ratios of the same mechanism — the converter renders the
    corpus record instead of refusing it.
  - *relabel (bucket to bucket):* none. The transitions run proves **zero** D units landed in a
    non-DONE bucket.
  - *reachability:* none.
  - *instrument-correction:* none to any counting instrument. One **document** correction, below.
- **Refused tokens:** none. No converter ran in this cycle; `token_coverage.py --check` reports
  `refused_non_done=0` at HEAD.
- **Discoveries:** one, and it is a figure in the criterion's own text, not a token or an atlas
  category. The criterion's scope note (and the dispatch prompt that quotes it) states
  `class_feature_of_unmodelled_corpus_class` at **634 units over 60 classes**. Derived from the
  live inventory at the cut it is **446 units over 58 distinct classes** — the other 25 of
  `class_feature`'s 471 D units at the cut carry
  `class_feature_no_dedicated_magnitude_id_matched_the_record_slug` (row 9), a different
  sub-cause, and 471 ≠ 634 by inspection of the criterion's own by-kind list. Emitted as retro
  `correction` `1788976580859-at-35-e5-002-5363c6`. The wrong figure never reached code or an
  instrument — no mechanism consumed it — and it did not change what the chassis had to do
  (`decisions.md §7`'s "one chassis, not sixty functions" holds at 58 as at 60). The other nine
  by-kind figures in the criterion text re-derive **exactly** (template 595, class_feature 471,
  deity 408, race_trait 183, ability 108, language 81, domain 80, class 29, skill 21, trait 6,
  summing to 1,982).
- **Figures + their re-derive commands:**
  | Figure | Value | Denominator | Command |
  |---|---|---|---|
  | bucket D at HEAD | 0 | of 49,438 corpus units | `python3 scripts/completion_atlas.py --check` |
  | corpus DONE | 49438 | of 49438 | `python3 scripts/completion_atlas.py --check` |
  | bucket D at the `tranche/15` cut | 1982 | of 49,438 | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-002_bucket_d_sub_causes.py --at 4c6c57eb9f` |
  | D sub-cause families at the cut | 14 | of 1,982 D units (58 per-class strings collapsed to one family) | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-002_bucket_d_sub_causes.py --transitions` |
  | D units `sheet-complete` at HEAD | 1982 | of the 1,982 present at the cut | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-002_bucket_d_sub_causes.py --transitions` |
  | D units not `sheet-complete` at HEAD | 0 | of 1,982 | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-002_bucket_d_sub_causes.py --transitions` |
  | `class_feature_of_unmodelled_corpus_class` units at the cut | 446 | of 1,982 D units | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-002_bucket_d_sub_causes.py --transitions` |
  | distinct classes in that family | 58 | of the 446 units | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-002_bucket_d_sub_causes.py --at 4c6c57eb9f` |
  | D after `AT-35-E2-005` | 43 | of 1,404 non-DONE at `51f91bba11` | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-002_bucket_d_sub_causes.py --at 51f91bba11` |
  | converter-refused non-DONE at HEAD | 0 | of 0 non-DONE | `python3 scripts/token_coverage.py --check` |
  | bucket A population | 0 | of 2 kinds cited | `python3 scripts/missing_engine_tables.py --check` |
  | magnitude-bearing units not held by the engine | 0 | of 26,396 magnitude-bearing | `python3 scripts/shape_engine_boundary.py --check` |
  | PCGen live files | 260 | baseline 260 | `python3 scripts/pcgen_residue_gate.py --check` |
  | source-format markers in `data/sheet_rules/` | 0 | files matching | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | rust lines changed | 0 | this cycle, since `00d0611e87` | `python3 scripts/cycle_scope_gate.py --receipt --since 00d0611e87 --before /tmp/wi-before-at-35-e5-002.json --after docs/work-inventory.json` |
  | wired-integration hits over the epic's scoped paths | 11 | all pre-existing, 0 from this cycle | `git diff --unified=0 "$(git merge-base HEAD origin/develop)...HEAD" -- <scoped paths> \| grep -cE '\b(STUB\|MOCK\|placeholder\|not yet implemented\|todo\|fixme\|hack)\b'` |
- **Build scope verified:** `CARGO_TARGET_DIR=/tmp/cargo-sd35-at-35-e5-002`, `CARGO_INCREMENTAL=0`, `-j 6`.
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0` (cold, wall clock **2:59.19**, max RSS 2,390,392 kB)
  - `cargo run --locked --bin sheet_rule_convert -- --check` → exit 0,
    `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS (112.5s)`,
    `refused 142 no_corpus_record` — identical to `AT-35-E5-001_cycle1_receipt.md`'s line
  - `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → `0`
  - `python3 scripts/completion_atlas.py --check` → exit 0; `token_coverage.py --check` → `verdict=PASS`;
    `shape_engine_boundary.py --check` → `not_held_by_engine=0`; `missing_engine_tables.py --check`
    → `population=0 kinds=0`; `pcgen_residue_gate.py --check` → `verdict=PASS`;
    `denominator_gate.py --check '…/*.md' '…/artifacts/**/*.md'` → `files_checked=62 violations=0`;
    `denominator_gate.py --check-provenance` → `files_checked=179 figures_examined=238 violations=0`
  - `cargo clippy`: not run — no Rust target changed, so there is no touched target to lint
  - `scripts/verify.sh --only pi-sweep` → `RESULT: PASS` (1 stage passed)
  - **Full workspace suite: not re-run, by attribution, not by omission.** The last full
    `cargo test --locked --no-fail-fast -j 6` green is the Epic 4 wrap-up correction cycle's
    48-of-48 gate re-run at `20812af147`'s parent range; `git diff --name-only 7a0bf64bbf..HEAD |
    grep -v '^docs/'` lists exactly one path, `scripts/verify-baselines.env` (a baseline number,
    not compiled), and **this cycle changes no Rust, no `data/`, no `apps/` and no `scripts/`
    file at all**. There is no change for a suite run to attribute a failure to. Precedent:
    `AT-35-E2-005-DISPOSITION_cycle2_receipt.md`, a docs-only cycle, recorded
    `builds_recorded=0` on the same reasoning.
  - desktop crate + frontend: **epic cadence** — this cycle touched no `apps/` path.
- **Sweep population:** N/A — no corpus record changed, so `corpus_literal_sweep` was not re-run
  (`workflow-instruction.md §6` step 3 runs it only when corpus records changed).
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` — carried in every
  converted record's `provenance.oracle_pin`; no figure in this receipt was derived from the
  pinned corpus.
- **Status:** complete
- **Notes:** Same shape as `AT-35-E5-001` cycle 1. The criterion's first Evidence clause (D at 0)
  was already true at dispatch — bucket D emptied at `406003afc3` — and kanban row 20 already read
  `complete`, but the second clause ("every sub-cause named with its mechanism and count") had no
  artifact behind it: `progress.md` named only the **43** sub-causes standing at the start of
  Epic 3, not the **1,982** the criterion is written against, and named no mechanism for any of
  them. This cycle pays that clause and leaves the first untouched. No other criterion's
  population was emptied by this cycle, so no other kanban row was closed. The dispatch's
  mandatory-bundling instruction did not apply: `remaining_non_done=0`.
- **Next-cycle scope:** criterion at zero.
