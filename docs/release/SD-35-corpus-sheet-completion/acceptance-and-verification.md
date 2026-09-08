---
canonical: true
owner: god-emporer
bundle_id: SD-35
date: 2026-09-07
---

# SD-35 Acceptance and Verification

The closure gates, the command that proves each criterion, and the artifact each produces.
**A criterion is met by its command's output, never by a lane's account of it.**

## 1. Per-criterion verification map

| Criterion | Verifying command | Artifact |
|---|---|---|
| AT-35-E1-001 | `python3 -m unittest scripts/tests/test_cycle_scope_gate.py` (under-floor exits 1, at-floor exits 0, whole-remainder exits 0); `scripts/verify.sh --only cycle-scope-gate-selftest` | `scripts/cycle_scope_gate.py`, the selftest stage |
| AT-35-E1-002 | RED→GREEN transcript: cited function moved 50 lines → all three `--check`s green; one cited condition changed → all three fail. `scripts/verify.sh --list` shows the new stages | the anchor tables; `artifacts/epic-1-tax-cut/citation-anchor-proofs.md` |
| AT-35-E1-003 | `artifacts/epic-1-tax-cut/build-time.json` before/after cold `cargo test --locked --no-run` wall time with commands; `cargo test -- --list` name-by-name diff, count unchanged | `build-time.json`, the consolidated binaries, the moved baselines |
| AT-35-E1-004 | `scripts/verify.sh --only denominator-gate` default run lists every SD-35 `.md`, `violations=0`; `--only figure-provenance` exit 0 | the widened default scope |
| AT-35-E1-005 | `python3 -m unittest scripts/tests/test_pcgen_residue_gate.py` (planted `raw_tokens` read fails; removed passes; `--closure` fails at baseline); `scripts/verify.sh --only pcgen-residue-gate`; `scripts/pcgen-residue-baseline.env` committed with the real first count | `scripts/pcgen_residue_gate.py`, the baseline file |
| AT-35-E1-006 | `test -f docs/retro/sd34-book-completion-retrospective.md`; `grep -c sd34-book-completion-retrospective docs/release/SD-34-book-completion/references/README.md docs/release/SD-35-corpus-sheet-completion/references/README.md` → ≥1 each; the row map sums against `completion_atlas.py --book core_rulebook --check` + `--book ultimate_campaign --check` at `4c6c57eb9f`; SD-34 `progress.md` status reads closed-by-fold | `docs/retro/sd34-book-completion-retrospective.md`, `artifacts/epic-1-tax-cut/sd34-open-row-map.json` |
| AT-35-E2-001 | `cargo run --locked --bin sheet_rule_convert -- --check` → `records=49438 converted=<n> refused=<n>` summing; `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` → 0; per-kind converter gates green | `src/bin/sheet_rule_convert.rs`, `src/pcgen_import/sheet_rule/`, `data/sheet_rules/`, `data/sheet_rules/_refused.json` |
| AT-35-E2-002 | `cargo test --locked --lib sheet_rule` (one test per value form); 19 per-kind on-screen frontend tests green; `pcgen_residue_gate.py --check` unchanged from baseline | `src/rules_core/sheet_rule.rs`, the section in `CharacterSheet.tsx`, the tests |
| AT-35-E2-003 | grep census: `grep -rln "oracle-unverifiable" src scripts apps tests \| wc -l` before == `grep -rln "sheet-complete" ... \| wc -l` after; `completion_atlas.py --check` `unclassified=0 overlap=0 done_evidence_violations=0` | the status, the rung, every consumer diff |
| AT-35-E2-004 | `python3 scripts/token_coverage.py --check` exit 0 with sums; RED→GREEN on a planted double-count; `scripts/verify.sh --only token-coverage` | `artifacts/epic-2-sheet-rule/token-coverage.json`, `scripts/token_coverage.py`, its test |
| AT-35-E2-005 | `cycle_scope_gate.py --receipt` rows; `completion_atlas.py --check` before/after; `token-coverage.json` re-derived; wall time recorded; oracle comparison `compared=<n> agree=<n> disagree=<n>` with `PCGEN_ORACLE_SHA`, every disagreement named | `artifacts/epic-2-sheet-rule/AT-35-E2-005_cycle1_receipt.md`, `oracle-parity-epic2.json` |
| AT-35-E3-001 | `python3 scripts/completion_atlas.py --by-kind` → `class_feature` B at 0 | receipts + atlas |
| AT-35-E3-002 | atlas → B at 0 for every kind | receipts + atlas |
| AT-35-E3-003 | atlas → C at 0; refused-token report for any residue moved to Epic 4 | receipts + atlas |
| AT-35-E3-004 | `artifacts/epic-3-place-and-surface/rate-ledger.json`: one row per cycle, sums equal the epic's movement, `builds_recorded == 1`, `pcgen_live_files` non-increasing | `rate-ledger.json` |
| AT-35-E4-001 | atlas → M at 0; `token-coverage.json` every compute-bearing token with a mapping row or a named refusal; per-cycle oracle comparison | receipts + ledger |
| AT-35-E4-002 | atlas → V at 0; harness receipt with `PCGEN_ORACLE_SHA`, per-unit cost on the first 50 stated before the run, `oracle_disagreement=<n> of 392` with every disagreement named | `artifacts/epic-4-resolve-and-verify/oracle-run-receipt.md` |
| AT-35-E4-003 | as E3-004 | `artifacts/epic-4-resolve-and-verify/rate-ledger.json` |
| AT-35-E5-001 | `python3 scripts/missing_engine_tables.py --check` → `population=0`; per table a refusal transcript and a success transcript | `artifacts/epic-5-residues/table-proofs.md` |
| AT-35-E5-002 | atlas → D at 0; `completion_atlas.py --by-evidence` every sub-cause at 0, each named with mechanism in the receipt | receipts |
| AT-35-E5-003 | atlas → U and Z at 0; `corpus_literal_sweep` examined-count moved by exactly the `beginner_box` record delta | receipts |
| AT-35-E5-004 | atlas → X at 0; a desktop test: a level-3 fixture's option list excludes a failed-prereq option and includes a met one | the filter, the test |
| AT-35-E5-005 | `completion_atlas.py --check` → `DONE=49438 of 49438`, every other bucket 0, exit 0; capability register re-derived with no row in a third state | `artifacts/epic-5-residues/completion-manifest.json`, `capability-register-closed.json` |
| AT-35-E6-001 | `pcgen_residue_gate.py --check` → `PcgenFormulaEvaluator`, `bonus_stack_reader`, `pre_tokens` at 0 live hits; oracle comparison agrees before and after; full workspace suite green | `artifacts/epic-6-pcgen-exit/oracle-parity-before.json`, receipts |
| AT-35-E6-002 | `pcgen_residue_gate.py --check` → 0 `raw_tokens` hits under `src/rules_core/`; `gen_book_cache` output byte-identical on one book before and after the relocation | receipts, the diff transcript |
| AT-35-E6-003 | `pcgen_residue_gate.py --check` → 0 hits under `apps/desktop/`; desktop crate + frontend suites green; the 19 on-screen tests pass | receipts |
| AT-35-E6-004 | `python3 scripts/pcgen_residue_gate.py --check --closure` → `live_files=0 live_hits=0 verdict=PASS`; oracle comparison at epic end agrees with epic start; desktop `cargo tree` shows no converter-module dependency | `artifacts/epic-6-pcgen-exit/oracle-parity-after.json`, `AT-35-E6-004_cycle_receipt.md` |
| AT-35-E7-001 | the full scan (§3) | `artifacts/epic-7-closure/AT-35-E7-001_cycle_receipt.md` |
| AT-35-E7-002 | retrospective written **and cited from `references/README.md` in the same cycle** | `docs/retro/sd35-corpus-sheet-completion-retrospective.md` |
| AT-35-E7-003 | sweep counts found vs removed; arch docs; graphify; PR; release notes | `receipts.md`, `release-notes.md` |

## 2. Standing gates — green at every cycle

```bash
python3 scripts/cycle_scope_gate.py --min 500 <scope flags>     # exit 0 — BEFORE the cycle starts (from AT-35-E1-001)
python3 scripts/pcgen_residue_gate.py --check                   # not above baseline (from AT-35-E1-005); --closure from AT-35-E6-004
python3 scripts/completion_atlas.py --check                     # unclassified=0 overlap=0 citation_failures=0
python3 scripts/token_coverage.py --check                       # from AT-35-E2-004 onward
cargo run --locked --bin sheet_rule_convert -- --check          # from AT-35-E2-001 onward; ids agree with the corpus
python3 scripts/shape_engine_boundary.py --check                # content-anchored from AT-35-E1-002
python3 scripts/missing_engine_tables.py --check
python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md'   # explicit until AT-35-E1-004
scripts/verify.sh --only pi-sweep
cargo run --locked --bin corpus_literal_sweep                   # 0 findings (only when corpus records changed)
cargo test --locked --no-run                                    # exit 0
cargo test --locked --lib
cargo test --locked --no-fail-fast -j 6                         # when src/ or the classifier changed
```

**Once per epic and once before the PR:** `scripts/verify.sh` — all stages (`decisions.md §3`).

**Do not narrow any gate's scope to make a cycle pass.** Every gate prints the population it
examined; a PASS without a stated population is a vacuous pass.

## 3. The final-acceptance scan (AT-35-E7-001)

The scan checks **work**, never reports:

1. **Count rows and derive SETS, not sizes.** Subtract id-sets between the launch inventory and
   HEAD's.
2. **Re-run every headline command yourself.** Subagent recaps quote stale figures.
3. **Read commit diffs.** Distinguish a real fix from an edited expectation.
4. **Re-derive failure attribution from `git`** against the `tranche/15` cut SHA.
5. **Grep the closure instruments for hardcoded exclusion lists.** Carve-outs hide in code —
   including the residue gate's own path list: a live path quietly added to its allow-list is
   the carve-out this bundle would most want to make.
6. **Verify at the widest build scope**, counting targets executed. **The full `verify.sh` runs
   here.**
7. **Read `## Open blockers`.** Any active entry BLOCKS.
8. **Enumerate open deferrals.** None may defer DoD scope.
9. **Re-prove the gates still fail.** Plant a violation in each new gate (`cycle_scope_gate`,
   `pcgen_residue_gate`, `token_coverage`, `sheet_rule_convert --check`, a moved anchor),
   confirm the catch, remove the probe, confirm zero residue.
10. **Check every cycle receipt carries a scope-gate line** with `verdict=PASS` or
    `PASS_WHOLE_REMAINDER`, `builds_recorded == 1`, and a non-increasing `pcgen_live_files`.
11. **Check every build-scope row names the SHA it ran at**, and no later commit in the same
    cycle regenerated the inventory or `data/sheet_rules/`.

### 3a. Deliverable-integrity checks — specific to this bundle

- **`completion_atlas.py --check` at HEAD → `DONE=49438 of 49438`.**
- **`pcgen_residue_gate.py --check --closure` at HEAD → `live_files=0 live_hits=0`.** Then an
  independent grep by the scan itself, not the gate's own pattern list: `grep -rn 'raw_tokens\|PcgenFormulaEvaluator\|render_pcgen_desc' src/rules_core src/saved_character src/campaign src/homebrew_authoring apps/desktop` → no output.
- **`sheet_rule_convert --check` at HEAD → ids agree with the corpus; `_refused.json` empty.**
- **`grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → 0.**
- **`token_coverage.py --check` at HEAD → zero refused units.**
- **The completion manifest's evidence pointers resolve** on an independently drawn sample of at
  least 200 units across all 19 kinds; for each sampled `sheet-complete` unit, load its
  `SheetRule`, evaluate it for the probe character, and confirm the recorded form.
- **The oracle parity artifacts** (`oracle-parity-epic2.json`, `-before.json`, `-after.json`)
  each name `PCGEN_ORACLE_SHA` and show `disagree=0`, or every disagreement resolved by a
  named commit.
- **The tool side is intact** (`decisions.md §11`, what is kept): `cargo build --locked --bin
  sheet_rule_convert --bin gen_book_cache` exits 0; `python3 scripts/oracle_harness/run.py --help`
  exits 0; `scripts/pcgen-oracle-pin.env` and `scripts/fetch-pcgen-oracle.sh` present;
  `src/pcgen_import/` holds the relocated parser and generators. A converter or oracle file
  deleted during the bundle is a blocking shortfall — Starfinder is next.
- **The 19 on-screen tests exist and pass** in the frontend run.
- **`build-time.json` shows after < before**, both measured cold.
- **The capability register has no third state.**
- **Every kanban row `complete`** with its receipt path resolving.

**If anything is short: STOP.** No retrospective, no sweep, **no PR**. Report what is short
with the command that shows it.

**Do not manufacture a shortfall either.** If the work is genuinely done, PASS it.

## 4. Closure gate sequence

1. Final-acceptance scan PASSES (full `verify.sh` inside it)
2. Retrospective written **and cited** in the same cycle
3. Full worktree/branch sweep, counts found vs removed
4. Architecture docs → graphify → PR → merge-conflict resolution (`../template/template.md §6`)
5. Release notes + version confirmation

**The operator merges `tranche/15` → `develop`.** No dispatched agent merges.

## 5. What does NOT satisfy a criterion

- A lane's `status: complete` unsupported by the mechanical receipt rows
- A cycle receipt with no `cycle_scope_gate.py` line
- A cycle whose `pcgen_live_files` rose
- A live path added to the residue gate's allow-list
- A converter, parser, generator, or oracle-harness file deleted "because PF1e is done"
- A `SheetRule` file carrying a PCGen token, formula string, or variable name
- A percentage without its denominator in the same construct
- A filed `## Open blockers` entry
- A bucket-to-bucket move reported as closure
- A per-unit fixture with a hand-derived expected value offered as the on-screen proof
- A refused-token residue described as "the rest"
- A gate passing because its population was narrowed
- "The engine cannot model X" offered as a reason a unit is not done
