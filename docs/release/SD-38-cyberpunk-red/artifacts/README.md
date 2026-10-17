---
canonical: true
owner: operator
bundle_id: SD-38
slug: cyberpunk-red
canonical_branch: tranche-18
target_version: 0.18.0
date: 2026-10-13
artifact_type: artifacts-index
status: draft — authored under operator-away defaults; operator rulings on licence and scope are on record
---

# SD-38 Artifacts

Per-card receipts and evidence. Each card writes `epic_<n>/<card-id>_cycle_receipt.md` (schema:
`../workflow-instruction.md §7`) plus every log or data file its receipt cites, copied into this tree
before the card returns. **Never** a PDF, a page-text file or a page PNG (`../decisions.md §11`):
image checks are cited by page number and crop box.

| Directory | Holds | Expected key artifacts |
|---|---|---|
| `cycle_0/` | C0, C1 | `inventory/` (below); `sd38-workflow.js` (written at launch); C1 receipt (A and B version-surface lists, the 15-file bump commit) |
| `cycle_0/inventory/` | the seven inventories and what their commands run | `roles.md`, `creation.md`, `skills.md`, `gear.md`, `cyberware.md`, `combat.md`, `netrunning.md`; `skills.tsv`; `gen.py`, `sr.py` (run with `python3 -I`; `gen.py` also writes `stat_tables.md` to its original session path, and a copy is here) |
| `epic_0/` | E0.1–E0.MC | `tauri-resources-tracked` plant log; LICENSE.json check; adapter RED/GREEN logs; `pf-render/{base,head}/sha256.txt`; `sf-lines/{base,head}/` |
| `epic_1/` | E1.0–E1.V | MANIFEST sum check (two implementations); per-lane image-check crop lists; re-derived counts; E1.8's GM-table inventory; `E1.V_verdicts.tsv` (sample, seed, verdict per field) |
| `epic_2/` | E2.1–E2.MC | fixture provenance (page or inventory section per fixture); plant logs; p121 branch evidence |
| `epic_3/` | E3.1–E3.MC | seeded-roller logs; three builds' sheet-vs-engine comparisons; ui-smoke logs |
| `epic_4/` | E4.1–E4.MC | row-mapping test output (+1000 shift); `CPR_SHEET_LINES_OUT` lines; ui-smoke logs; `ui-smoke:doc` diff |
| `epic_5/` | E5.1–E5.MC | improvement round trips; real store count + sha256 before/after |
| `epic_6/` | E6.1–E6.MC | per-kind catalog counts vs MANIFEST; deck calculator checks |
| `epic_7/` | E7.1–E7.11 | `cpr-seed-builds.md`; packaged-root logs; fresh-install observations; widest-verify logs; closure scan output; DEF revisit output; sweep counts; graphify SHA |

Each epic directory starts empty except for a `.gitkeep` (`../workflow-instruction.md §1` item 8).
