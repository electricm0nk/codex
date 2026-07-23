# Cycle epic2-2.4-pcgen_runner — Epic 2 Oracle-Harness Comparator / Criterion 2.4

- **Card ID:** t_7ad1a31b (receipt only, minted post-hoc as a done-receipt on board `codex-tranche-5`, assignee `operator`, completed — not a live claim)
- **Commit SHA:** 20ab8c9562768b886b2a572ce5307f2ea5e02c20
- **Files touched:**
  - `src/oracle_validation/pcgen_runner.rs` (new)
  - `src/oracle_validation/mod.rs` (registers `pub mod pcgen_runner;`, updates module doc)
  - `tests/sd26_pcgen_runner.rs` (new)
  - `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/artifacts/epic_2/pcgen_runner_rust-cycle_receipt.md` (this file)
  - `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/progress.md`
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (diff-scoped dual-audit gate, `BASE_BRANCH...HEAD`)
- **Wired-integration audit result:** `OK_NO_TOKENS` (diff-scoped dual-audit gate, `BASE_BRANCH...HEAD`)
- **Acceptance criterion:** "Criterion 2.4 — `src/oracle_validation/pcgen_runner.rs`. Rust-side wrapper around the PCGen runner. Concurrency: serial." (`epic-breakdown.md` Epic 2). Verification row: "`cargo test --locked --test sd26_pcgen_runner`" (`acceptance-and-verification.md` row 2.4 pcgen_runner).
- **Status:** complete
- **Notes:**
  - **Read `scripts/pcgen-run-character.sh` and `scripts/pcgen-normalize-output.py` first**, per the cycle brief, plus `comparator.rs`'s and `normalization.rs`'s module docs (the only places SD-26's own docs mention this criterion — `technical-design.md` has no distinct "pcgen_runner" section; its §2 covers only 2.1/2.2/2.3). This does not depend on 2.1/2.2's own code changes landing first (per the brief), only on the existing SD-25 scripts, which were already on disk and unmodified by this cycle.
  - **Judgment call — bridges directly to `comparator::NormalizedOutput` instead of `normalization::RawPcgenOutput`.** The 2.2 receipt's module doc speculated 2.4 would be "the expected producer of `RawPcgenOutput`" (raw, single-string-per-dimension text). Having now actually read `pcgen-normalize-output.py`, that script already performs its own raw-XML → typed-value reduction (`_signed_int`, explicit `value_string` XOR `value_i16` fields) — its JSON output shape is already field-for-field identical to `NormalizedDimensionValue`, not a raw-text shape. Round-tripping an already-parsed `i16` back through a string just to feed `normalization.rs`'s trim/integer-coercion rules would be a redundant, slightly lossy hop, not a real second normalization step. `PcgenRunOutput::to_normalized_output()` is therefore a direct field carry into Criterion 2.1's comparator input. `normalization.rs`'s rule engine and `RawPcgenOutput`/`RawDimensionValue` types are untouched and remain available for raw, not-yet-typed text captures that don't come through this script pair.
  - Exposed `parse_normalized_output(json_text: &str) -> Result<PcgenRunOutput, PcgenRunnerError>` as a standalone pure function (no process spawn, no filesystem access) so the JSON-shape/error-mapping contract is unit-testable independent of a real PCGen invocation — `run_pcgen_character` calls it internally after shelling out to both real scripts.
  - `PcgenRunnerError` carries the real underlying script's real exit status and stderr for every failure branch (spawn failure, non-zero exit, unreadable output, malformed JSON) rather than collapsing to a generic error — consistent with `no-stub-mvp-doctrine.md`'s "real failures over fake success."
  - Repo-root resolution (`codex_repo_root()`) follows the same `CODEX_REPO_ROOT` env-override-then-`CARGO_MANIFEST_DIR` order of truth `apps/desktop/src-tauri/src/ge08_workbench.rs::codex_repo_root` already establishes elsewhere in this repo, rather than inventing a new convention.
  - Ran the real end-to-end test (`run_pcgen_character_runs_the_real_pcgen_engine_end_to_end`) against the same substitute `pf_Paladin.pcg` fixture SD-25's `pcgen_runner_smoke.rs` uses (no real `.pcg` exists yet for the pilot case; see that test's own `## DISCOVERED` history) — no PCGen output mocked, stubbed, or fabricated; genuine PCGen Gradle run each time (~38-40s wall clock).
  - Ran `cargo test --locked --lib` (157/157), `cargo test --locked --test sd26_comparator` (4/4), `cargo test --locked --test sd26_normalization` (7/7), `cargo test --locked --test pcgen_runner_smoke` (2/2, SD-25's own scripts unaffected), and `cargo test --locked --test sd26_identifier_discipline_audit` + `sd24_identifier_discipline_audit` (1/1 each) to confirm no regressions.
  - Hermes kanban CLI was available this cycle (`hermes kanban --board codex-tranche-5 create ... --assignee operator --initial-status blocked`, then `hermes kanban complete t_7ad1a31b`) — minted and completed as a receipt-only card per step 8, not a live claim.
- **Discovery forwards:** None new (the `RawPcgenOutput`-vs-direct-`NormalizedOutput` bridging choice above is a judgment call resolved this cycle, not forwarded).
- **Next-cycle plan:** Epic 2 Criterion 2.3 (`src/oracle_validation/parity_report.rs`) — the `parity_report_<case-id>.md` writer. It can now read real `PcgenRunOutput.diagnostics`/`case_id`/`legacy_route`/`claim_tier_floor` fields from this cycle's wrapper (in addition to `normalization.rs`'s rule IDs) for its "Summary" and "Discovered Deltas" sections, then Criterion 2.5 (pilot-case verification cycle) wires the full comparator + runner + report pipeline against the real pilot case end to end.

## Verification transcript

```text
$ cargo test --locked --test sd26_pcgen_runner
running 6 tests
test parse_normalized_output_parses_the_normalizer_scripts_real_json_shape ... ok
test parse_normalized_output_reports_malformed_json_as_a_typed_error ... ok
test run_pcgen_character_reports_a_missing_character_file_without_shelling_out ... ok
test to_normalized_output_composes_end_to_end_with_the_comparator ... ok
test wrapped_scripts_resolve_to_real_files_in_this_checkout ... ok
test run_pcgen_character_runs_the_real_pcgen_engine_end_to_end ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 38.46s

$ cargo test --locked --lib
test result: ok. 157 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

$ cargo test --locked --test sd26_comparator
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ cargo test --locked --test sd26_normalization
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ cargo test --locked --test pcgen_runner_smoke
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 38.16s

$ cargo test --locked --test sd26_identifier_discipline_audit --test sd24_identifier_discipline_audit
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s
```
