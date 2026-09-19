---
canonical: true
bundle_id: SD-36
---

# SD-36 Receipts

Closure-gate receipt blocks and cycle receipts. Populated as the bundle runs.

---

## Architecture-truth-up receipt

(Populated at Epic D step 2)

```yaml
architecture_truth_up:
  bundle: SD-36
  integration_target: develop
  documents_touched: []
  exit_code: null
  timestamp: null
```

---

## Graphify-update receipt

(Populated at Epic D step 3)

```yaml
graphify_update:
  bundle: SD-36
  integration_target: develop
  exit_code: null
  timestamp: null
```

---

## Epic A evidence (criteria A5, A10 — 2026-09-19)

### A5 — `CARGO_MANIFEST_DIR` path rewrites

`git grep -n 'CARGO_MANIFEST_DIR' -- crates` finds exactly one live, non-comment
use: `crates/codex-ingest/src/lib.rs`'s own `pub fn repo_root()`, whose entire
job IS resolving `CARGO_MANIFEST_DIR` (this crate's own manifest dir, two
levels under the repo root, joined with `../..`) into the real repo root
for every other caller in the crate to use instead of a bare
`env!("CARGO_MANIFEST_DIR")` or a `"."`/relative-path fallback. The remaining
hits are doc comments explaining that resolver's own provenance (`bonus_stack_reader.rs`,
`corpus_trap_baseline.rs`, `parity_report.rs`, `pcgen_runner.rs`, two test
files) — not further definitions. This is the single-definition shape the
criterion intends: one canonical resolver, every caller routed through it.

`git grep -c 'crate::pcgen_import' -- crates/codex-ingest` shows in-crate
callers already spelled `crate::pcgen_import::…` (the crate's own internal
path), never `codex::pcgen_import` or `codex_ingest::pcgen_import` from
inside the crate itself — correct for code living inside the crate.

This cycle's own contribution (previously 18 uncommitted files, reviewed and
correctness-checked): 8 `crates/codex-ingest/src/bin/*.rs` tool bins and 6
test files that read `data/corpus` via a bare relative path or a
`CODEX_REPO_ROOT`/`"."` fallback (both silently wrong once the crate's
manifest dir stopped being the repo root) now resolve it via
`codex_ingest::repo_root()` / `crate::repo_root()`. Verified line-by-line;
no behavior change beyond the path source.

### A2 — residue gate `--check --closure` does NOT exit zero (open gap)

`grep 'lst_file' scripts/pcgen_residue_gate.py` and the identifier-scan part
of the acceptance command both hold: `lst_file files=0 hits=0` and
`codex_ingest files=0 hits=0` — Epic A's own code-wall patterns are clean.
But `python3 scripts/pcgen_residue_gate.py --check --closure` itself exits 1
(`verdict=FAIL`, `live_files=49885 live_hits=181711`), because the gate's
shipped-data class (ruling B17) folds in every file the Tauri bundle ships,
and `apps/desktop/src-tauri/tauri.conf.json` now bundles
`"../../../data/corpus/": "data/corpus/"` whole (this tranche's separate
desktop-corpus-root commit, `217f712bab`, explicitly out of this cycle's
write scope to undo). The recorded ratchet baseline
(`scripts/pcgen-residue-baseline.env`, `files=260 hits=12736`, 2026-09-07)
predates that bundling change by 10 days, so even plain `--check` (not just
`--closure`) now fails too (`verdict=FAIL_INCREASED`). Logged as
`scripts/retro.py correction` (`docs/retro/events/sd31-transcribe.jsonl`).
**Not fixed here** — resolving it means either the corpus ships redacted
(provenance fields stripped before bundling) or the residue gate's
shipped-data scan is deliberately re-scoped for a corpus that is meant to
ship, both of which are decisions past Epic A's "PCGen crate wall" scope.

### A10 — `codex-ingest` baselines (first recording)

See `scripts/verify-baselines.env`'s SD-36 Epic A section for the full
derivation. Headline: `BASELINE_INGEST_FULL_TESTS=1673`,
`BASELINE_INGEST_TEST_BINARIES=157`, `BASELINE_CLIPPY_WARNINGS_INGEST=0`,
measured from `cargo test --locked --no-fail-fast -p codex-ingest -j 6` and
`cargo clippy --locked --tests -j 6` (in `crates/codex-ingest`). The test run
is NOT clean: 6 suites each fail exactly 1 test, all six panicking inside
`pcgen-run-character.sh` with an identical `JAVA_HOME is set to an invalid
directory` error — `~/.sdkman/candidates/java/current` is an empty directory
on this box, not a symlink to a real JDK. Verified as a pre-existing host
condition, not an Epic A regression: `/usr/bin/java` works, the failure is
identical regardless of which crate hosts the file, and none of the six
files' content changed beyond the path move. Logged as
`scripts/retro.py incident` (`docs/retro/events/sd31-transcribe.jsonl`,
`recurrence_key=sdkman-java-current-broken`). Net effect: `verify.sh`'s
`ingest-full` stage will report FAIL (cargo exit 101) on this box until
either the box's `JAVA_HOME` is fixed, or these 6 tests gain the same
checkout-unavailable-skip pattern `sd27_feat_prerequisite_enforcement.rs`
already uses for `PCGEN_CORPUS_ROOT`. Neither fix is in this cycle's write
scope (`~/.sdkman` is outside the repo; the skip-pattern rewrite is new
production-test-logic scope beyond the A5/A9/A10 batch this cycle inherited).

---

## Cycle receipts

| Cycle | Epic | Status | Baseline command | Output |
|---|---|---|---|---|
| — | — | — | — | — |

---

## Closure block (Epic D, steps 2–5)

(Populated at closure. Records: architecture-truth-up, graphify, merge-conflict resolution, if any.)

| Step | Status | Notes |
|---|---|---|
| Acceptance criteria 100% | awaiting | all epics → complete |
| Retrospective written | awaiting | Epic D step 1 (workflow-instruction §11) |
| Worktree sweep | awaiting | Epic D step 1 (workflow-instruction §11) |
| Architecture docs updated | awaiting | Step 2 receipt above |
| Graphify run | awaiting | Step 3 receipt above |
| PR open and merged | awaiting | Step 5/6 |

---

