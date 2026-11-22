# Cycle epic2-2.3-parity_report — Epic 2 Oracle-Harness Comparator / Criterion 2.3

- **Card ID:** t_c9b7b0b4 (receipt only, minted post-hoc as a done-receipt on board `codex-tranche-5`, assignee `operator`, completed — not a live claim)
- **Commit SHA:** 7566d0625d0bf1d8e6e0bfd57c47d8f639bd45c9
- **Files touched:**
  - `src/oracle_validation/parity_report.rs` (new)
  - `src/oracle_validation/mod.rs` (registers `pub mod parity_report;`, updates module doc)
  - `tests/sd26_parity_report.rs` (new, authored this cycle)
  - `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/artifacts/epic_2/parity_report-cycle_receipt.md` (this file)
  - `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/progress.md`
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (diff-scoped dual-audit gate, `BASE_BRANCH...HEAD`)
- **Wired-integration audit result:** `OK_NO_TOKENS` (diff-scoped dual-audit gate, `BASE_BRANCH...HEAD`)
- **Acceptance criterion:** "Criterion 2.3 — `src/oracle_validation/parity_report.rs`. Generates `parity_report_<case-id>.md`. Concurrency: serial." (`epic-breakdown.md` Epic 2). Verification row: "Manual review + cargo test" (`acceptance-and-verification.md` row 2.3 parity_report).
- **Status:** complete
- **Notes:**
  - Read the just-landed `src/oracle_validation/comparator.rs` (2.1), `normalization.rs` (2.2), and `pcgen_runner.rs` (2.4) — all already on `tranche/5-4` — plus `technical-design.md §2.3`'s Markdown shape sketch (Summary / Per-Dimension Comparison / Normalization Rules Used / Discovered Deltas) and `scope-draft.md §1.2`'s output path (`artifacts/oracle_validation/parity_report_<case-id>.md`), which is more specific than `technical-design.md §2.3`'s bare filename and than `content-unit-inventory.md`'s one-line summary — followed `scope-draft.md` as the authoritative path since it is the only doc that states a directory.
  - `render_parity_report(case_id, &ComparisonResult, &[NormalizationRule]) -> String` is a **pure renderer** over Criterion 2.1's already-produced `ComparisonResult` and Criterion 2.2's `NormalizationRule` slice — it does not run the comparator or the normalization engine itself, matching how `comparator.rs`'s and `normalization.rs`'s own module docs describe 2.3 as a downstream consumer of their public surface, not a producer.
  - `write_parity_report(output_dir, case_id, &ComparisonResult, &[NormalizationRule]) -> io::Result<PathBuf>` writes the rendered document to `<output_dir>/parity_report_<case-id>.md`, creating `output_dir` if needed, and `default_parity_report_dir()` resolves the real default (`artifacts/oracle_validation/`) via the same `CODEX_REPO_ROOT`-override-then-`CARGO_MANIFEST_DIR` order of truth `pcgen_runner.rs::codex_repo_root` already establishes — kept the resolution convention consistent across Epic 2 rather than inventing a second one.
  - **Judgment call — one-sided mismatch reasons rendered as legible English, not raw enum names.** `comparator.rs`'s `MismatchReason` has three variants (`ValueMismatch`, `MissingFromCodex`, `MissingFromPcgen`) documented as a real, expected case (PCGen/Codex dimension sets are not guaranteed identical). The report's "Discovered Deltas" section renders these as "value mismatch" / "missing from Codex" / "missing from PCGen" rather than the Rust variant names, since this is a human-facing Markdown document (the criterion's own verification row calls for "Manual review").
  - **What manual review would confirm** (per the acceptance-and-verification.md row 2.3 "Manual review + cargo test" verification): (1) that a real generated report against the actual GE06 pilot case (once 2.5's verification cycle runs the full comparator + runner pipeline end-to-end) reads as genuinely legible Markdown to a human reviewer — table alignment, dash rendering, section ordering matching `technical-design.md §2.3`'s sketch; (2) that the Normalization Rules Used section's rule listing stays accurate as `normalization.rs`'s rule set grows past the current two defaults; (3) that `artifacts/oracle_validation/` is an acceptable durable location for these per-case reports (vs., e.g., being gitignored or being a scratch/ephemeral directory) once 2.5 starts writing real ones. This cycle's `cargo test` proves the renderer/writer mechanics; it cannot substitute for a human looking at an actual rendered report file.
  - Ran `cargo test --locked --test sd26_parity_report` (10/10, new), `cargo test --locked --lib` (157/157), `cargo test --locked --test sd26_comparator` (4/4), `cargo test --locked --test sd26_normalization` (7/7), `cargo test --locked --test sd26_pcgen_runner` (6/6), and `cargo test --locked --test sd26_identifier_discipline_audit` (1/1) to confirm no regressions.
  - Hermes kanban CLI was available this cycle (`hermes kanban --board codex-tranche-5 create ... --assignee operator --initial-status blocked`, then `hermes kanban complete t_c9b7b0b4`) — minted and completed as a receipt-only card per step 8, not a live claim.
- **Discovery forwards:** None new.
- **Next-cycle plan:** Epic 2 Criterion 2.5 (verification cycle for the pilot case) — wires the full comparator (2.1) + normalization (2.2) + parity-report (2.3) + PCGen runner (2.4) pipeline against the real pilot case (`tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`), producing a real `parity_report_pf1_..._level1.md` via this cycle's `write_parity_report` and asserting the pilot fixture's `current_claim_status` upgrades from `not_yet_grounded` to `oracle_checked`.

## Verification transcript

```text
$ cargo test --locked --test sd26_parity_report
running 10 tests
test default_parity_report_dir_ends_with_artifacts_oracle_validation ... ok
test render_parity_report_all_matched_reports_pass ... ok
test render_parity_report_any_mismatch_reports_fail ... ok
test render_parity_report_discovered_deltas_section_lists_each_mismatch ... ok
test render_parity_report_includes_case_id_heading ... ok
test render_parity_report_one_sided_mismatch_reasons_are_legible ... ok
test render_parity_report_summary_counts_matches_and_mismatches ... ok
test render_parity_report_lists_normalization_rules_used ... ok
test write_parity_report_writes_the_named_file_to_the_given_directory ... ok
test render_parity_report_per_dimension_table_lists_both_sides_and_match_status ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ cargo test --locked --lib
test result: ok. 157 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

$ cargo test --locked --test sd26_comparator
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ cargo test --locked --test sd26_normalization
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ cargo test --locked --test sd26_pcgen_runner
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 36.91s

$ cargo test --locked --test sd26_identifier_discipline_audit
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s
```
