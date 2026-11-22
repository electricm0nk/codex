# SD-43 artifacts

Per-epic receipts and evidence. Each dispatched card writes `<epic>/<card-id>_cycle_receipt.md` (schema:
`../receipts.md §2`) and keeps its images, PDFs, logs and outputs beside it under `<epic>/<card-id>/`.
Directories are created by the first card that writes into them (git keeps no empty directory). Nothing here may
hold the report API key or an operator's personal data; receipts carry commands and counts.

| Directory | Holds |
|---|---|
| `cycle_0/` | `C0_cycle_receipt.md` (the placeholder gate's full output); `sd43-workflow.js` (written by the lead at launch, `../workflow-instruction.md §2.4`); C1's receipt; C2's receipt and `C2-graphify-receipt.md` (command, exit code, node counts before/after) |
| `epic_0/` | `abilities_census.py` + `abilities-census.json` / `.md` (E0.1, incl. the **data-gap list** `../forward-scope-register.md §2` cites); `market-counts.json` (E0.2); `placeholder_names.py` + `placeholder-names.json` (E0.3); `gear-repro.md` + `gear-repro/` screenshots (E0.4); `rest-and-house-rules.md` (E0.5). Scripts run with `python3 -I` from the repo root |
| `epic_1/` | E1.x receipts; E1.MC screenshots (gear sections, Market lists and a filter, sell window); the stats sampling table (E1.9) |
| `epic_2/` | E2.x receipts; layout screenshots at full, −40 px and smallest height (E2.1, E2.2); the bio-page `pdftotext` excerpt (E2.MC) |
| `epic_3/` | E3.x receipts; E3.MC screenshots on three seeds; the data-gap notice count beside E0.1's list; the E3.5 evaluated/unknown token-shape table |
| `epic_4/` | `proposed-names.md` (E4.1, for the operator at E4.G); E4.G's receipt quoting the reply; E4.2's Market screenshot |
| `epic_5/` | E5.x receipts; report screens and Developer tab screenshots |
| `epic_6/` | E6.x receipts; relocation, House Rules (PF/SF/CPR), export/import screenshots |
| `epic_7/` | E7.1's ui-smoke counts; E7.2's closure-readiness report (G-CL, G-R, fresh-install observations per artifact, revisit commands); E7.3/E7.4 receipts (PR number, merge SHA, publish run) |

`../receipts.md §1` holds the closure scripts' YAML receipts (truth-up, claims critic, graphify).
