# SD-40 artifacts

Per-epic receipts and evidence. Each dispatched card writes
`epic_<n>/<card>_cycle_receipt.md` (schema: `../workflow-instruction.md §7`) and keeps its PNGs, PDFs,
logs and outputs beside it under `epic_<n>/<card>/`.

| Directory | Holds |
|---|---|
| `cycle_0/` | `operator-brief-2026-11-07.md` (the brief with the operator's rulings); `C0_cycle_receipt.md`; `sd40-workflow.js` (written at launch) |
| `cycle_1/` | C1's receipt |
| `epic_0/E0.1/` | the **reference picture**: `aldric.pcg` (hand-authored, tool-side), `aldric-pcgen-std.pdf`, `aldric-pcgen-simple.pdf` (and an Elowen spellbook if cheap), their 60 dpi PNGs, `block-map.md`; ≤ 4 MB; **never shipped, never read by a test** |
| `epic_1/` … `epic_4/` | receipts; `<card>/*.png` the card looked at (60 dpi, ≤ 12 pages, ≤ 2 MB per card) |
| `epic_2/E2.G/` | the two Aldric page-1 PDFs (A4, Letter) and PNGs the operator prints; the signed receipt |
| `epic_5/E5.2/` | the ten PDFs of the full set and their PNGs; the signed receipt |
| `epic_5/` | closure receipts; E5.3's revisit-command outputs |

Figures in receipts carry their commands and denominators (`AGENTS.md` rule 9). PNG size is a control
(`decisions.md` SD-p): `du -sk artifacts/epic_<n>/<card>` ≤ 2048 except `epic_0/E0.1` ≤ 4096.
