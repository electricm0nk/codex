# Cycle epic2-2.2-normalization — Epic 2 Oracle-Harness Comparator / Criterion 2.2

- **Card ID:** t_pending (see report; kanban card minted post-hoc as a done-receipt — not a live claim)
- **Commit SHA:** a87bc0d
- **Files touched:**
  - `src/oracle_validation/normalization.rs` (new)
  - `src/oracle_validation/mod.rs` (registers `pub mod normalization;`, updates module doc)
  - `tests/sd26_normalization.rs` (new)
  - `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/artifacts/epic_2/normalization-cycle_receipt.md` (this file)
  - `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/progress.md`
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (diff-scoped dual-audit gate, `BASE_BRANCH...HEAD`)
- **Wired-integration audit result:** `OK_NO_TOKENS` (diff-scoped dual-audit gate, `BASE_BRANCH...HEAD`)
- **Acceptance criterion:** "Criterion 2.2 — `src/oracle_validation/normalization.rs`. Small rule engine. Concurrency: serial." (`epic-breakdown.md` Epic 2). Verification row: "`cargo test --locked --test sd26_normalization`" (`acceptance-and-verification.md` row 2.2 normalization).
- **Status:** complete
- **Notes:**
  - **Read `comparator.rs` (Criterion 2.1) first**, per the cycle brief, to confirm the exact `NormalizedOutput` / `NormalizedDimensionValue` shape this module must produce. Its module doc explicitly says normalization is "a pre-comparison step on the PCGen side" that "does not need to change [`compare`'s] signature or the shape of `ComparisonResult`" — this cycle honors that: `normalize()` is a standalone reduction from raw text to `NormalizedOutput`, called *before* `compare()`, not a new parameter threaded into it.
  - **`technical-design.md §2.1`'s 3-arg `compare(pcgen, codex, normalization: &[NormalizationRule])` sketch was explicitly rejected already in the 2.1 receipt** in favor of the epic-breakdown's 2-arg form. This cycle stays consistent with that landed decision rather than retrofitting a 3rd parameter onto `compare`.
  - **Judgment call — authored `RawDimensionValue` / `RawPcgenOutput` this cycle.** Neither type existed yet (Criterion 2.4's `pcgen_runner.rs`, the expected producer of raw PCGen text captures, hasn't landed). Modeled on the same pattern the 2.1 receipt used for `NormalizedOutput`/`NormalizedDimensionValue`: define the minimal real shape now (`id: String`, `raw_value: String`), document it as the input contract 2.4 is expected to produce, and let a later cycle wire the actual PCGen-invocation producer without needing to change this module's public surface.
  - **Rule set implements exactly `technical-design.md §2.2`'s two named examples**: `trailing-whitespace-strip` (trim leading/trailing whitespace) and `integer-coercion` (parse the already-trimmed string as `i16`, promoting the value from string to numeric on success). `default_normalization_rules()` returns them in that order — order matters, since integer-coercion parsing `"  17  "` before trimming would fail the `i16::parse` and silently leave the value as a string, which the `default_rules_apply_trim_before_integer_coercion` test proves is not what happens.
  - Rules thread a working `(Option<String>, Option<i16>)` pair through in sequence rather than being independent single-pass transforms, so later rules see earlier rules' output — needed for the trim-then-coerce ordering above.
  - An empty rule slice leaves the raw string completely untouched (proven by `empty_rule_set_leaves_the_raw_string_untouched`), confirming the engine is a real pass-through-by-default rule pipeline, not a hardcoded transform.
  - Added an end-to-end test (`normalized_raw_output_composes_end_to_end_with_the_comparator`) that runs `normalize()` output straight into `comparator::compare()` and asserts `all_matched()` — proving the two modules actually compose, not just that each compiles in isolation.
  - Ran `cargo test --locked --lib` (157 passed) and `cargo test --locked --test sd26_comparator` (4 passed) to confirm no regression to the existing `oracle_validation` module or its consumers.
- **Discovery forwards:** None new.
- **Next-cycle plan:** Epic 2 Criterion 2.3 (`src/oracle_validation/parity_report.rs`) — the `parity_report_<case-id>.md` writer, which per `technical-design.md §2.3` renders a "Normalization Rules Used" section citing rule IDs (`trailing-whitespace-strip`, `integer-coercion`) by `normalization.rs:N` line reference; this cycle's `NormalizationRule.id` field is the stable handle that report writer will read.

## Verification transcript

```text
$ cargo test --locked --test sd26_normalization
running 7 tests
test default_rules_apply_trim_before_integer_coercion ... ok
test non_numeric_value_remains_a_trimmed_string_after_default_rules ... ok
test integer_coercion_rule_converts_numeric_strings_to_i16 ... ok
test empty_rule_set_leaves_the_raw_string_untouched ... ok
test normalize_reduces_a_full_raw_output_dimension_for_dimension ... ok
test normalized_raw_output_composes_end_to_end_with_the_comparator ... ok
test trim_whitespace_rule_strips_leading_and_trailing_whitespace ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ cargo test --locked --test sd26_comparator
running 4 tests
test a_codex_only_dimension_is_reported_as_missing_from_pcgen ... ok
test a_pcgen_only_dimension_is_reported_as_missing_from_codex ... ok
test all_dimensions_agree_yields_no_mismatches ... ok
test a_disagreeing_value_is_reported_as_value_mismatch ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ cargo test --locked --lib
test result: ok. 157 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```
