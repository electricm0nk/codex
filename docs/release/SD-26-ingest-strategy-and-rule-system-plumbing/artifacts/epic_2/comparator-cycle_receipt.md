# Cycle epic2-2.1-comparator — Epic 2 Oracle-Harness Comparator / Criterion 2.1

- **Card ID:** t_6ffcc5109c6fb18e (receipt only, minted post-hoc as a done-receipt — not a live claim)
- **Commit SHA:** <filled in after push — see commit log>
- **Files touched:**
  - `src/oracle_validation/comparator.rs` (new)
  - `src/oracle_validation/mod.rs` (registers `pub mod comparator;`, updates module doc)
  - `tests/sd26_comparator.rs` (new)
  - `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/artifacts/epic_2/comparator-cycle_receipt.md` (this file)
  - `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/progress.md`
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (diff-scoped dual-audit gate, `BASE_BRANCH...HEAD`)
- **Wired-integration audit result:** `OK_NO_TOKENS` (diff-scoped dual-audit gate, `BASE_BRANCH...HEAD`)
- **Acceptance criterion:** "Criterion 2.1 — `src/oracle_validation/comparator.rs` — `compare(canon_pcg: &NormalizedOutput, codex: &SelectedDimensions) -> ComparisonResult`. Concurrency: serial." (`epic-breakdown.md` Epic 2). Verification row: "`cargo test --locked --test sd26_comparator`" (`acceptance-and-verification.md` row 2.1).
- **Status:** complete
- **Notes:**
  - **Test file did not exist and was authored this cycle** (`tests/sd26_comparator.rs`), per the task brief — it is a real behavioral proof of `compare()`, not a placeholder: it exercises agreement, a single flipped numeric value (`ValueMismatch`), a PCGen-only dimension (`MissingFromCodex`), and a Codex-only dimension (`MissingFromPcgen`). The Codex side of the test is the real `SelectedParityDimensions::from_receipt` output built from the actual GE06 pilot headless receipt (PF1 CRB Human Fighter level 1 fixture), not a synthetic stand-in — only the PCGen side is hand-built (there is no PCGen runner yet; that's Criterion 2.4).
  - **Judgment call — `NormalizedOutput` type authored this cycle, not deferred.** The criterion's signature names `NormalizedOutput` as an input type, but `normalization.rs` (Criterion 2.2, which per `technical-design.md §2.2` owns "the full rule set... as a `Vec<NormalizationRule>`") does not exist yet. Rather than block 2.1 on 2.2, `NormalizedOutput` and `NormalizedDimensionValue` are defined in `comparator.rs` itself, mirroring `SelectedDimension`'s `id` / `value_string: Option<String>` / `value_i16: Option<i16>` shape from `selected_parity_dimensions.rs` so the two sides align by ID without a schema-translation step. This is documented in the module doc comment as the stable Epic 2 contract; 2.2 is expected to produce `NormalizedOutput` values (via its rule engine) rather than redefine the type.
  - **Judgment call — 2-arg signature per `epic-breakdown.md`, not the 3-arg form sketched in `technical-design.md §2.1`.** `technical-design.md` shows `compare(pcgen, codex, normalization: &[NormalizationRule])`, anticipating Criterion 2.2's rule engine. Since the explicit criterion text supplied for this cycle pins the 2-arg signature (`compare(canon_pcg: &NormalizedOutput, codex: &SelectedDimensions) -> ComparisonResult`), this cycle implements exactly that. `compare()` currently does direct equality (`value_i16 == value_i16 && value_string == value_string`); the module doc comment flags that Criterion 2.2 owns refining match semantics (whitespace trimming, integer coercion) as a normalization step that produces the `NormalizedOutput` this function already accepts — no signature change anticipated.
  - **Type-name note:** the criterion's prose says `codex: &SelectedDimensions`; the real existing type (per `selected_parity_dimensions.rs`, already on disk and read before implementing) is `SelectedParityDimensions`. Used the real type name — `SelectedDimensions` does not exist anywhere in the codebase and the criterion prose is evidently a paraphrase.
  - `ComparisonResult` design also covers one-sided-dimension cases (a dimension present on only one side) via `MismatchReason::MissingFromCodex` / `MissingFromPcgen`, beyond the minimal `matches`/`mismatches` sketch in `technical-design.md §2.1` — needed because a real PCGen output and a real Codex selected-dimension set are not guaranteed to carry identical dimension sets, and silently dropping unmatched dimensions would hide real parity gaps from Criterion 2.3's report writer.
  - Ran `cargo test --locked --lib` (157 passed) to confirm no regression to the existing `oracle_validation` module or its consumers.
- **Discovery forwards:** None new.
- **Next-cycle plan:** Epic 2 Criterion 2.2 (`src/oracle_validation/normalization.rs`) — the rule engine (`Vec<NormalizationRule>`) that will construct `NormalizedOutput` values from raw PCGen capture and refine match semantics beyond this cycle's direct-equality baseline.

## Verification transcript

```text
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
