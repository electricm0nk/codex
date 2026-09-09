# Cycle AT-35-E5-004-cycle1 — Epic 5 Residues / AT-35-E5-004

- **Commit SHA:** `a56096b861` (the feature) and `<docs-sha>` (this receipt, progress and kanban)
- **Scope gate:** `scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER`
  (`python3 scripts/cycle_scope_gate.py --min 500 --bucket X`). Bucket X was already at **0**
  when this cycle started: `AT-35-E3-002 cycle 1` emptied it at `26bdfa8d5b`. The whole
  non-DONE remainder is 0, so the verdict is `PASS_WHOLE_REMAINDER` and no bundling was
  possible or needed — there was nothing in any other bucket to bundle. This cycle therefore
  moves **zero units by design** and pays the criterion's **second** Evidence clause, which
  was unpaid and explicitly recorded as unpaid on kanban row 22.
- **Files touched:**
  - `src/rules_core/level_up_option_filter.rs` (new — the join)
  - `src/rules_core/mod.rs` (module registration)
  - `src/rules_core/level_up/fighter.rs` (module doc: its `next_required_uplift` note landed)
  - `src/pcgen_import/sheet_rule/formula.rs` (one converter mapping row)
  - `data/sheet_rules/**` (354 regenerated record files)
  - `apps/desktop/src-tauri/src/character_hub.rs` (the IPC surface and both tests)
  - `apps/desktop/src/boundary/previewLevelUp.ts`, `apps/desktop/src/characterHub/LevelUpDialog.tsx`
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-004_cycle1_receipt.md`,
    `progress.md`, `kanban.md`
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS. One transient violation, fixed inside the
  cycle rather than excused: the first draft of `LevelUpDialog.tsx` gave the search input an
  HTML `placeholder` attribute, which the audit's `placeholder` pattern matched. Replaced with
  a visible `<label htmlFor>` — better accessibility and no token.
- **Acceptance criterion** (verbatim from `epic-breakdown.md`):

  > ### AT-35-E5-004 — bucket X reaches zero: the per-character choice filter
  >
  > 168 at authoring. SD-34 `decisions.md §17`'s operator requirement stands: the backend
  > filters the valid options for *this* character at level-up. Build the join over
  > `SheetRule.applies` (prerequisites are `Applies`, converted from `PRE*` at ingest — no
  > `pre_tokens` on the live side) and expose it on the existing level-up IPC.
  >
  > **Evidence:** X at 0; a desktop test: a level-3 fixture's option list excludes a
  > failed-prereq option and includes a met one.

- **Receipt rows (mechanical):**
  `closed=0 relabeled=0 rust_lines_changed=835 ratio=n/a builds_recorded=3 pcgen_live_files=260`
  (`python3 scripts/cycle_scope_gate.py --receipt --since 7557ab00fa --before
  /tmp/wi-before-AT-35-E5-004.json --after docs/work-inventory.json --target-dir
  /tmp/cargo-sd35-AT-35-E5-004`; detail lines `regressed=0 added=0 dropped=0`,
  `closed_by_kind=` and `relabeled_moves=` both empty). `closed=0` is correct and expected:
  the population was already 0 at the cycle start. `builds_recorded=3` counts the lib build,
  the converter/inventory generator runs and the test-target build — the generator runs the
  procedure itself mandates, not one build per item.
- **PCGen residue:**
  `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`
  (`python3 scripts/pcgen_residue_gate.py --check`). Identical to the baseline, and identical
  to `AT-35-E5-003_cycle1_receipt.md`'s. It read `FAIL_INCREASED` (261/12738) mid-cycle
  because two doc comments in the new live-side module wrote a source token literal in prose;
  both were reworded and the gate returned to baseline. Logged as a `correction`
  (`1788988102522-sd31-transcribe-4c54cd`).
- **Oracle parity:** N/A — this cycle added no `Number` mapping. The converter change lowers
  an ability-prerequisite operand from an opaque variable to `max(AbilityScore(ab), raisers)`;
  it moves gate operands, not a `SheetValue::Number`. Package totals are unmoved (below).
- **Movement, four buckets:**
  - *closure (into DONE):* **0 units.** The id-set is empty; X reached 0 at `26bdfa8d5b`.
  - *relabel (bucket to bucket):* **0.** `regressed=0 added=0 dropped=0`.
  - *reachability:* **unchanged.** No record became reachable or unreachable.
  - *instrument-correction:* **354 record files** whose `applies` gate now reads the
    character's real ability score instead of a variable that always evaluated 0. No unit's
    `status`, `evidence` or bucket moved — the guarded inventory regen produced a
    `generated_at`-only diff, which was reverted.
- **Refused tokens:** none. No token type was refused by this cycle, and the package's own
  refusal figure is unmoved at 142 (`refused 142 no_corpus_record`).
- **Discoveries:** one, and it is the reason this cycle is not a pure documentation cycle.
  `PreStatScore_<AB>` — the left-hand side of every `PREVARGTEQ`-shaped ability prerequisite —
  lowered to a bare corpus variable. `cr__stats.lst` declares it `0`, and each stat row carries
  its base term as `max(<AB>SCORE, Alt<AB>SCORE)` on a source row that belongs to no corpus
  record, so the converter's variable table held **only** the records that *raise* the
  prerequisite floor and the base term was silently lost. The gate then read 0 for every
  character: a Strength-16 fighter was refused Power Attack. Neither `token_coverage.json` nor
  the atlas predicts this — it is not an unmapped token, it is a mapped token with a lost
  operand, and every presence-shaped instrument reads it as converted. It now lowers to
  `max(AbilityScore(ab), raisers)`, which keeps the floor-raising records
  (a shield ability's `MAX(15,DEX)`, a talent's flat 19) as the other side of the max.
  Emitted as a `correction` retro event.
- **Figures + their re-derive commands:**
  | Figure | Value | Command | Denominator |
  |---|---|---|---|
  | bucket X non-DONE units | **0** | `python3 scripts/cycle_scope_gate.py --min 500 --bucket X` | of 49,438 corpus units |
  | whole non-DONE remainder | **0** | `python3 scripts/cycle_scope_gate.py --min 500` | of 49,438 corpus units |
  | record files whose gate changed | **354** | `git diff --name-only a56096b861^ a56096b861 -- data/sheet_rules \| wc -l` | of 54,586 package files |
  | references to the six stat variables | **716** before | `python3 -c` over `data/sheet_rules/*/*/*.json` (see Notes) | across those 354 files |
  | package totals, unmoved | `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277` | `cargo run --locked --bin sheet_rule_convert -- --check` | the whole package |
  | offerable feat records | **2465** | `python3 -c`: rules with `pool == "feat"`, principal id, non-empty label | of 2,565 `pool=feat` rules |
  | fixture option census | `offered=718 refused=1745 considered=2463` | `cargo test --locked no_feat_option_is_both_offered_and_refused -- --nocapture` | of the 2,465 offerable records |
  | census reconciliation | 2465 − 2463 = **2** | the two non-repeatable feats the fixture already holds (`dodge`, `power_attack`) | — |
  | new lib tests | **7** | `cargo test --locked --lib level_up_option_filter` | in `level_up_option_filter` |
  | new desktop tests | **2** | `cargo test --locked feat_option` (desktop crate) | of 576 desktop tests |
- **Build scope verified** (run at `a56096b861` plus this receipt's docs commit):
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`
  - `cargo test --locked --lib -j 6` → the 7 new `level_up_option_filter` tests pass
  - `cargo test --locked --no-fail-fast -j 6` → **8,741 passed; 0 failed** over **412** test targets
    (`awk '/^test result: ok/{p+=$4; f+=$6} END{print p, f}'` over the run log; 413 `test result: ok`
    lines, 0 `FAILED`)
  - **desktop crate** (this cycle touched `apps/`, so it runs here, not at epic cadence):
    `cargo test --locked --no-fail-fast -j 4` → `576 passed; 0 failed`;
    `cargo clippy --locked --tests -j 4` → `Finished dev profile`, zero new warnings
  - **frontend** (same reason): `npm run typecheck` → clean; `npm test` → `101/101 test files passed`
  - `cargo run --locked --bin sheet_rule_convert` (the regeneration) then `-- --check` →
    `records=49438 converted=49296 refused=142 rules=69344
    var_tables=5277 verdict=PASS (112.1s)` — every figure identical to the pre-change run
  - `cargo run --locked --bin corpus_literal_sweep` → `48706 records examined of 51476 read,
    413314 tokens compared, 0 findings`, `CLEAN`
  - `cargo run --locked --bin v06_work_inventory` (guarded, with `CORPUS_LITERAL_SWEEP_REPORT`
    and `DERIVED_FIXTURE_CHECK_REPORT` set; never `--allow-stamp-loss`) → a
    `generated_at`-only diff, reverted: no unit's status or evidence moved. The first,
    unguarded attempt was **refused** by the binary's own stamp-loss guard (7,385 of 32,617
    stamps) — the guard working, recorded rather than bypassed.
  - `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → `0`
  - `python3 scripts/completion_atlas.py --check` → `missing_clearing_mechanisms=0
    stale_derived_at=False citation_failures=0`; `token_coverage.py --check` →
    `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1
    verdict=PASS`; `shape_engine_boundary.py --check` →
    `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True`;
    `missing_engine_tables.py --check` → `population=0 kinds=0`;
    `denominator_gate.py --check '…/*.md' '…/artifacts/**/*.md'` →
    `files_checked=63 violations=0`; `denominator_gate.py --check-provenance` →
    `files_checked=180 figures_examined=248 violations=0`
  - `scripts/verify.sh --only pi-sweep` → `PASS pi-sweep (11 hits over src/rules_core/rules_tables, 11 baseline rows)`,
    `RESULT: PASS` (1 stage passed)
- **Sweep population:** `corpus_literal_sweep` examined **48706 → 48706** (of 51,476 read),
  `0 findings`, `CLEAN`. Moved by 0: this cycle changed no `data/corpus/` record.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`. The converter
  mapping row was derived by reading `cr__stats.lst` in the pinned checkout, and every
  regenerated record carries that sha in `provenance.oracle_pin`.
- **Status:** complete
- **Notes:** Three judgment calls. **(1)** The converter fix is inside this criterion's scope
  even though the criterion names a filter, not a mapping row: without it the deliverable
  returns a wrong answer for 354 records, and a filter that excludes a *met* prerequisite is
  worse than no filter because it looks like it works (`AGENTS.md` rules 5 and 7).
  `src/pcgen_import/sheet_rule/` is in this cycle's bundled file-touch union.
  **(2)** The join lives in `rules_core` but is *called* at the desktop seam, because
  `rules_core::level_up` has no `SheetRulePackage` to join against — a placement, not an
  absence; `PickList::candidates` staying empty is now only about which seam composes the
  list. **(3)** The fixture turned out to hold Dodge, Power Attack and Weapon Focus already
  (`compose_character_input` really puts them on `chosen.selected_feats`), so the test's
  "includes a met one" half is carried by Mobility — offered *because* this character holds
  Dodge — which is stronger evidence of a per-character join than an ungated option would be.
  The 716-reference figure is re-derived with:
  `python3 -c "import json,glob,re; vs={'v356e9d6dc3609183','vc314492280238530','ve59ae2a694dab2b8','vfa21bca192d79cd9','vf49e3402396f19fa','vbe8e9a5a94b0ac91','vd584bd0af3ea44f8'}; p=re.compile('|'.join(sorted(vs))); print(sum(len(p.findall(open(f).read())) for f in glob.glob('data/sheet_rules/*/*/*.json')))"`
  run against the pre-`a56096b861` tree.
- **Next-cycle scope:** criterion at zero. Bucket X is 0 and both Evidence clauses are paid.
  This cycle **closes kanban row 22** and empties no other criterion's population (every
  bucket was already 0), so no other row changes status because of it.
