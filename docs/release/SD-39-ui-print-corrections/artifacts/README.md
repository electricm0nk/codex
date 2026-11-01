# SD-39 artifacts

Per-epic receipts and evidence. Each dispatched card writes
`epic_<n>/<card>_cycle_receipt.md` (schema: `../workflow-instruction.md §7`) and keeps any logs or
outputs beside it under `epic_<n>/<card>/`.

| Directory | Holds |
|---|---|
| `cycle_0/` | `defects-and-rulings-2026-11-01.md` (the operator's transcript); `sd39-workflow.js` (written at launch) |
| `epic_1/` … `epic_8/` | receipts for E1–E8; `epic_1/E1.0/key-diff.txt`; `epic_7/E7.1/import-report-*.md`; `epic_8/E8.5/` the 24 golden PDFs' sha256 list (PDFs git-ignored if > 5 MiB total) |
| `epic_9/` | closure receipts; `E9.4/` the three PDFs the operator prints and the signed receipt |

Figures in receipts carry their commands and denominators (`AGENTS.md` rule 9).
