---
canonical: true
owner: operator
bundle_id: SD-37
artifact_type: artifacts-index
---

# SD-37 Artifacts

These are the per-card receipts and evidence. Each card writes
`epic_<n>/<card-id>_cycle_receipt.md` (schema in `../workflow-instruction.md §7`), plus every log
or data file its receipt cites. Those files are copied into this tree before the card returns,
never left in tmpfs scratch.

| Directory | Holds | Expected key artifacts |
|---|---|---|
| `cycle_0/` | C0.0–C1 | `sd37-workflow.js` (written at launch); C0.1 sweep receipt; `C0.2_cycle_receipt.md` (review findings); C1 bump receipt (14-file list) |
| `epic_0/` | E0.1–E0.4 | fresh-clone proof log; licence matrix diff; SF denominator + sum-check output; `seed-builds.md` (full SF seed builds, written first); `seed-hand-values.md` (every row has an SRD URL); the independent reviewer's per-row verdicts |
| `epic_1/` | E1.1–E1.MC | `pf-render-hashes.txt` (Aldric, Elowen × before/after); structural-diff output |
| `epic_2/` | E2.1–E2.MC | PF round-trip log; schema regeneration diff |
| `epic_3/` | E3.1–E3.MC | include-resolution listing; token-coverage (1,954 SF `MODIFY*` tokens, mapped + refused); `token-mapping/sf-mapping-table.v1.json` *(proposed path)*; planted-mutation log; per-book `_report.json` summaries |
| `epic_4/` | E4.1–E4.MC | SF seed renders vs hand values; mutation log |
| `epic_4a/` | E4a.1–E4a.MC | importer re-point before/after counts (two implementations); `.lst` burn-down before/after; D6 status re-derivation; PF hash pair; Bestiary 1 count before/after |
| `epic_5/` | E5.1–E5.MC | per-seed printed-feature diffs; residue gate output |
| `epic_6/` | E6.1–E6.MC | ui-smoke logs and screenshots; real-store count + sha256 before/after |
| `epic_7/` | E7.1–E7.9 | parity log + "not covered" list; `verify-e7.2.log`; closure scan output; DEF-1 check output; sweep counts; graphify SHA |

Each directory starts empty except for a `.gitkeep`
(`workflow-instruction.md §1` item 8).
