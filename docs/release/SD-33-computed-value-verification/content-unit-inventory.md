---
canonical: true
owner: god-emporer
bundle_id: SD-33
date: 2026-08-24
---

# SD-33 Content-Unit Inventory

Per-content-unit N-tuple: **module → test fixture → cycle artifact → command**. Paths verified 2026-08-24 at `1d6ae1e72b` per `workflow-instruction.md §4`; `(new)` marks an Epic deliverable confirmed not to collide with an existing name.

## Epic 1 — Instruments

| Unit | Module | Fixture | Artifact | Command |
|---|---|---|---|---|
| Box partition | `scripts/box_ledger.py` `(new)` | inline population fixtures | `THE-BOX.md` | `python3 scripts/box_ledger.py --check` |
| Fail-closed proofs | `scripts/box_ledger.py` `(new)` | five mutation cases | `artifacts/epic-1-instruments/box-ledger-mutation-proofs.md` | one RED→GREEN per condition |
| Probe census | `src/bin/v06_work_inventory.rs` (read-only) | — | `artifacts/epic-1-instruments/probe-surface-census.json` | committed generator command |
| Denominator gate | `scripts/verify.sh` | malformed + corrected receipt | `artifacts/epic-1-instruments/denominator-gate-proof.md` | `scripts/verify.sh --only denominator-gate` |

## Epic 2 — Oracle harness

| Unit | Module | Fixture | Artifact | Command |
|---|---|---|---|---|
| Headless build | `scripts/oracle_harness/` `(new)` | — | `artifacts/epic-2-oracle-harness/build-transcript.md` | the real build command |
| Character round-trip | `scripts/oracle_harness/` `(new)` | authored `.pcg` + export template | `artifacts/epic-2-oracle-harness/roundtrip/` | the export command |
| Comparison contract | `scripts/oracle_harness/` `(new)` | `agree` / `disagree` / `unverifiable` cases | `artifacts/epic-2-oracle-harness/harness-fixtures/` | fixture suite |
| Path ruling | — | — | Epic 2 closing receipt + `progress.md` | stated, and escalated if Path B |

**Fixture discipline:** the expected value in any oracle fixture is transcribed from bytes the harness's own read path does **not** touch. A fixture built from the file the harness reads is a mirror, not a check — it will validate a fabricated value.

## Epic 3 — Engine coverage

| Unit | Module | Fixture | Artifact | Command |
|---|---|---|---|---|
| Gap root-cause | `src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs` | traced coordinates | `artifacts/epic-3-engine-coverage/coverage-gap-rootcause.md` | per-family comparison |
| F1 closure | `src/rules_core/pilot_compute/formula_interpreter.rs` | family fixtures | per-family table in receipt | corpus-wide run |
| F2–F9 closure | as above | as above | as above | as above |
| 100% + denominator | `src/bin/formula_interpreter.rs` | — | regenerated `formula_interpreter.corpus-wide.json` | `README.md §4` row G → `0` |

## Epic 4 — Unknown classification

| Unit | Module | Fixture | Artifact | Command |
|---|---|---|---|---|
| `unknown` root-cause | `src/bin/v06_work_inventory.rs` | — | `artifacts/epic-4-unknown-classification/unknown-rootcause.md` | cause before count |
| Classification to zero | `src/bin/v06_work_inventory.rs` | `doneness_verdict()` cases | updated `docs/work-inventory.json` | `jq '[.units[]\|select(.status=="unknown")]\|length'` → `0` |
| No effort-named buckets | `scripts/box_ledger.py` `(new)` | — | `THE-BOX.md` groups | `box_ledger.py --check` |

**Regeneration hazard:** `docs/work-inventory.json` is a generated board. Regenerate only via its guarded path, and replay `doneness_verdict()` at both ends to report movement. A generator and its producer change in the **same commit**.

## Epic 5 — Re-verification

| Unit | Module | Fixture | Artifact | Command |
|---|---|---|---|---|
| 1,741 fixture-verified | Epic 2's harness | — | `artifacts/epic-5-reverification/fixture-verified-rows.jsonl` | per-unit `(ours, oracle, verdict)` |
| 6,589 literal-verified | Epic 2's harness | — | `artifacts/epic-5-reverification/literal-verified-rows.jsonl` | as above |
| Disagreement resolution | varies by defect | RED→GREEN per fix | `progress.md` entries | one commit or escalation each |

## Epic 6 — Closure epilogue

| Unit | Artifact | Command |
|---|---|---|
| Final-acceptance scan | closure receipt | every criterion + every kanban card `complete` |
| Retrospective | `docs/retro/sd33-computed-value-verification-retrospective.md` | cited from `references/README.md` same cycle |
| Sweep / arch-docs / graphify / PR | `receipts.md` | `../template/template.md §6` |

## Cross-cutting units

| Unit | Where | Note |
|---|---|---|
| PI screening | all corpus writes | `technical-requirements.md` R5 — coordinates only in receipts, never a term |
| Corpus write path | all corpus writes | R6 — guarded generator only; never `--allow-stamp-loss` |
| Desktop workspace | `apps/desktop/src-tauri` | separate cargo workspace; test explicitly |
