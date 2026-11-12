# SD-41 artifacts

Per-epic receipts and evidence. Each dispatched card writes
`epic_<n>/<card>_cycle_receipt.md` (schema: `../workflow-instruction.md §7`) and keeps its images, PDFs,
logs and outputs beside it under `epic_<n>/<card>/`.

| Directory | Holds |
|---|---|
| `cycle_0/` | `operator-brief-2026-11-12.md` (the request and the nine answers); `C0_cycle_receipt.md`; `sd41-workflow.js` (written at launch) |
| `cycle_1/` | C1's receipt |
| `epic_0/` | `census.py` (run with `python3 -I` from the repo root) and `census.txt`, every figure with its command; the SD-40 overlap in §7 |
| `epic_1/` | E1.x receipts; E1.MC's partition line and the Phase-1 resume receipt |
| `epic_2/` | E2.x receipts; E2.MC's `.pcg` population probe output |
| `epic_3/E3.3/` | the ui-smoke `results.json` of the 13 named rows |
| `epic_3/E3.MC/` | four gear-screen screenshots (SD-l, SD-n) |
| `epic_4/E4.1/` | the Inventory page PNGs the card looked at |
| `epic_4/E4.G/` | the 4 Inventory PDFs (Aldric, Elowen × A4, Letter) and ≥ 9 screenshots the operator looks at; the signed receipt |
| `epic_5/` | closure receipts; E5.2's revisit-command outputs |

Figures in receipts carry their commands and denominators (`AGENTS.md` rule 9). Image size is a control
(`decisions.md` SD-n): `du -sk artifacts/epic_<n>/<card>` ≤ 2048.
