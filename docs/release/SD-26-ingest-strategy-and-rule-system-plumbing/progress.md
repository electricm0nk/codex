# SD-26 — Progress

> **Operating method:** see `./scope-draft.md` and `scripts/workflow-dispatch.sh`. This file is created on cycle 0 of Epic 2 with the deterministic seed. The orchestrator reads `## TODO` + `## DISCOVERED` and dispatches the highest-priority unclaimed item.

## Status matrix (placeholder)

| Criterion | State | Cycle ID | Commit SHA | Notes |
|---|---|---|---|---|
| 1.1 Source-code identifier audit | complete | epic1-1.1-identifier-audit | 74d9402 | Tree already clean (RED returned 0 hits, per SD-24's prior remediation); extended standing regression guard to scripts/+data/ (see receipt) |
| 2.1 comparator | complete | epic2-2.1-comparator | 744cd71 | `compare()` + `NormalizedOutput`/`ComparisonResult` land in `src/oracle_validation/comparator.rs`; see receipt |
| 2.2 normalization | complete | epic2-2.2-normalization | a87bc0d | `normalize()`/`normalize_dimension_value()` rule engine (trim-then-integer-coercion) lands in `src/oracle_validation/normalization.rs`, producing `comparator.rs`'s `NormalizedOutput`; see receipt |
| 2.3 parity_report | complete | epic2-2.3-parity_report | 7566d06 | `render_parity_report()`/`write_parity_report()`/`default_parity_report_dir()` land in `src/oracle_validation/parity_report.rs`, rendering a real `parity_report_<case-id>.md` from `comparator.rs`'s `ComparisonResult` + `normalization.rs`'s rule set; see receipt |
| 2.4 pcgen_runner | complete | epic2-2.4-pcgen_runner | 20ab8c9 | `run_pcgen_character()`/`PcgenRunOutput` land in `src/oracle_validation/pcgen_runner.rs`, wrapping `scripts/pcgen-run-character.sh` + `scripts/pcgen-normalize-output.py`; `to_normalized_output()` bridges directly into `comparator.rs`'s `NormalizedOutput` (normalizer already emits typed values); see receipt |
| 2.5 verification cycle | not-started | — | — | — |
| 3.1 core_rulebook cache | not-started | — | — | parallel: yes |
| 3.2 advanced_players_guide cache | not-started | — | — | parallel: yes |
| 3.3 advanced_class_guide cache | not-started | — | — | parallel: yes |
| 3.4 beastiary cache | not-started | — | — | parallel: yes |
| 4.1 research epic | not-started | — | — | serial |
| 4.2..4.22 per-book | not-started | — | — | spawned dynamically (21 books) |
| 5.1 doctrine-cost audit | not-started | — | — | serial |
| 6.1 Final criterion scan | not-started | — | — | Sonnet |
| 6.2 Architecture closure pipeline | not-started | — | — | Opus |
| 6.3 Release notes | not-started | — | — | Haiku |
| 6.4 Build version (→ 0.5.99) | not-started | — | — | Haiku |
| 6.5 PR + merge | not-started | — | — | Sonnet |

## TODO (deterministic seed)
2.5, 3.1–3.4, 4.1, 5.1, 6.1–6.5

## DONE
1.1 (commit 74d9402)
2.1 (commit 744cd71)
2.2 (commit a87bc0d)
2.4 (commit 20ab8c9)
2.3 (commit 7566d06)

## DISCOVERED
(empty — populated by per-class residue + structural discoveries)

## Cycle log

| Cycle ID | Criterion | Commit SHA | Result |
|---|---|---|---|
| epic1-1.1-identifier-audit | 1.1 Source-code identifier audit | 74d9402 | complete — audited tree already clean (SD-24 prior remediation); added `tests/sd26_identifier_discipline_audit.rs` extending the standing regression guard to `scripts/` + `data/` (previously uncovered, ahead of Epic 3/4 populating `data/corpus/`+`data/stubs/`). RED/GREEN proven via temporary synthetic-leak injection since no real leak existed to remediate. Dual-audit gate: `OK_NO_BUNDLE_TAGS` / `OK_NO_TOKENS`. |
| epic2-2.1-comparator | 2.1 comparator | 744cd71 | complete — implemented `compare(canon_pcg: &NormalizedOutput, codex: &SelectedParityDimensions) -> ComparisonResult` in `src/oracle_validation/comparator.rs`; defined `NormalizedOutput`/`NormalizedDimensionValue` mirroring `SelectedDimension`'s shape (normalization.rs/2.2 not yet built). Authored `tests/sd26_comparator.rs` (did not exist) covering agreement, value mismatch, and both one-sided-dimension cases against the real GE06 pilot receipt. RED (module missing) -> GREEN (4/4 tests pass) -> `cargo test --locked --lib` 157/157 pass. Dual-audit gate: `OK_NO_BUNDLE_TAGS` / `OK_NO_TOKENS`. |
| epic2-2.2-normalization | 2.2 normalization | a87bc0d | complete — implemented `normalize(raw: &RawPcgenOutput, rules: &[NormalizationRule]) -> NormalizedOutput` (+ `normalize_dimension_value`) in `src/oracle_validation/normalization.rs`, authoring `RawDimensionValue`/`RawPcgenOutput` (2.4's expected producer shape, not yet built) and the two `technical-design.md §2.2` default rules (`trailing-whitespace-strip`, `integer-coercion`, applied in that order). Authored `tests/sd26_normalization.rs` (did not exist), including an end-to-end test composing `normalize()` output straight into `comparator::compare()`. RED (module missing) -> GREEN (7/7 tests pass) -> `cargo test --locked --test sd26_comparator` 4/4 + `cargo test --locked --lib` 157/157 pass, no regressions. Dual-audit gate: `OK_NO_BUNDLE_TAGS` / `OK_NO_TOKENS`. |
| epic2-2.4-pcgen_runner | 2.4 pcgen_runner | 20ab8c9 | complete — implemented `run_pcgen_character(character_pcg: &Path, options: &PcgenRunOptions) -> Result<PcgenRunOutput, PcgenRunnerError>` in `src/oracle_validation/pcgen_runner.rs`, wrapping the real `scripts/pcgen-run-character.sh` (PCGen Gradle invocation) + `scripts/pcgen-normalize-output.py` (typed-JSON normalizer) pair. Judgment call: since the normalizer script already emits typed `value_string`/`value_i16` dimensions (not raw text), `PcgenRunOutput::to_normalized_output()` bridges directly into `comparator.rs`'s `NormalizedOutput` rather than round-tripping through `normalization.rs`'s `RawPcgenOutput` — see receipt for full reasoning. Authored `tests/sd26_pcgen_runner.rs` (did not exist), including a real end-to-end PCGen invocation against the same substitute `pf_Paladin.pcg` fixture SD-25's `pcgen_runner_smoke.rs` uses. RED (module missing) -> GREEN (6/6 tests pass) -> `cargo test --locked --lib` 157/157, `sd26_comparator` 4/4, `sd26_normalization` 7/7, `pcgen_runner_smoke` 2/2, identifier-discipline audits 1/1 each pass, no regressions. Dual-audit gate: `OK_NO_BUNDLE_TAGS` / `OK_NO_TOKENS`. Minted + completed hermes kanban receipt card `t_7ad1a31b` on `codex-tranche-5` (assignee `operator`). |
| epic2-2.3-parity_report | 2.3 parity_report | 7566d06 | complete — implemented `render_parity_report(case_id: &str, comparison: &ComparisonResult, normalization_rules_used: &[NormalizationRule]) -> String` + `write_parity_report(output_dir: &Path, ...) -> io::Result<PathBuf>` + `default_parity_report_dir() -> PathBuf` in `src/oracle_validation/parity_report.rs`, rendering `technical-design.md §2.3`'s Summary / Per-Dimension Comparison / Normalization Rules Used / Discovered Deltas Markdown shape from a real `ComparisonResult` (2.1) + rule set (2.2), and writing it to `artifacts/oracle_validation/parity_report_<case-id>.md` per `scope-draft.md §1.2`. Pure renderer: does not itself run the comparator or normalization engine. Authored `tests/sd26_parity_report.rs` (did not exist, 10 tests) covering the case-id heading, summary counts, per-dimension table rows (matched + mismatched), normalization-rules listing, discovered-deltas listing (incl. one-sided `MissingFromCodex`/`MissingFromPcgen` reasons rendered as legible English), pass/fail result line, and a real on-disk write/read round trip. RED (module missing) -> GREEN (10/10 tests pass) -> `cargo test --locked --lib` 157/157, `sd26_comparator` 4/4, `sd26_normalization` 7/7, `sd26_pcgen_runner` 6/6, identifier-discipline audit 1/1 pass, no regressions. Dual-audit gate: `OK_NO_BUNDLE_TAGS` / `OK_NO_TOKENS`. Verification per acceptance-and-verification.md row 2.3 ("Manual review + cargo test"): cargo test proves renderer/writer mechanics; manual review (deferred to 2.5, once a real pilot-case report exists) should confirm the rendered Markdown reads well to a human and that `artifacts/oracle_validation/` is an acceptable durable location. Minted + completed hermes kanban receipt card `t_c9b7b0b4` on `codex-tranche-5` (assignee `operator`). |

## Open blockers
(empty)

---

*Per `loop-instruction.md §6 step 7`: the orchestrator updates this file in place via the concurrent-write protocol.*
