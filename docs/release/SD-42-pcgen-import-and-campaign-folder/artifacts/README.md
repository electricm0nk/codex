# SD-42 artifacts

Per-epic receipts and evidence. Each dispatched card writes `epic_<n>/<card>_cycle_receipt.md` (schema:
`../workflow-instruction.md §7`) and keeps its images, PDFs, logs and outputs beside it under
`epic_<n>/<card>/`. **Nothing under this directory may carry a line of the operator's `.pcg`** — counts only
(`../decisions.md §10`; the PII check in `../workflow-instruction.md §6` step 5).

| Directory | Holds |
|---|---|
| `cycle_0/` | `operator-brief-2026-11-16.md` (the intake and rulings Q2–Q25, verbatim); `C0_cycle_receipt.md`; `sd42-workflow.js` (written at launch); C1's receipt |
| `epic_0/` | `census.py` (run with `python3 -I` from the repo root) and `census.json`, every figure with its key |
| `epic_1/` | E1.x receipts; `E1.1/<sample>.tsv` + `summary.json` (the 7 samples' per-line tables; the operator's stays local); `E1.MC/summary.json` (the re-run on the final converter) |
| `epic_2/` | E2.x receipts; `E2.MC/*.png` (every Manager and tab screen, looked at) |
| `epic_3/` | E3.x receipts; the licence loop output |
| `epic_4/` | E4.x receipts; `E4.4/results.json` (the smoke rows); `E4.5/` (the proof runs' counts); `E4.MC/*.png` |
| `epic_5/` | `E5.G/` (≥ 12 PNGs, 1 PDF, the signed receipt); E5.1's logs; the closure-readiness report |

`receipts.md` (the package root) is created by the closure scripts (E5.3, E5.4).
