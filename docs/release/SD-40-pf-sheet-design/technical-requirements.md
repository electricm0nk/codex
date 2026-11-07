---
canonical: true
owner: operator
bundle_id: SD-40
slug: pf-sheet-design
canonical_branch: tranche-20
target_version: 0.20.0
date: 2026-11-07
status: planning-ready
---

# SD-40 Technical Requirements

> **`decisions.md §21` wins where this file differs.**

## 1. Pre-launch prerequisites (each with its check; C1 runs P-1, P-2, P-5, P-8 and pastes the output; the others at the card named)

| ID | Prerequisite | Check | Closes at |
|---|---|---|---|
| P-1 | `tranche-20` exists on origin at `tranche-19`'s tip of 2026-11-07 | `git ls-remote --heads origin tranche-20 \| awk '{print $1}'` = `d4489b3557a1fa6f59be53ffc16561990c7e6715` or a descendant (`git merge-base --is-ancestor d4489b3557 origin/tranche-20`) | C1 |
| P-2 | Working tree clean; `sd39-main` untouched | `git status --porcelain \| wc -l` → 0; `git worktree list` recorded | C1 |
| P-3 | PCGen oracle on pin and the installed distribution present | `bash scripts/fetch-pcgen-oracle.sh --check --quiet` exit 0; `test -x "$PCGEN_REPO_DIR/build/install/pcgen/pcgen"` | E0.1 |
| P-4 | Java runs PCGen headless | `cd "$PCGEN_REPO_DIR/build/install/pcgen" && JAVA_OPTS=-Djava.awt.headless=true ./pcgen --help` prints the `-E/-o/-c` options (2026-11-07: OpenJDK 25, Temurin) | E0.1 |
| P-5 | Doctrine present | `test -f docs/governance/no-stub-mvp-doctrine.md && test -f docs/doctrine-external/identifier-discipline.md` | C1 |
| P-6 | Disk ≥ 24 G free before any full sweep | `df -h /` | every cargo card |
| P-7 | `pdftoppm`, `pdftotext`, `pdfinfo` present (poppler) | `which pdftoppm pdftotext pdfinfo` → three paths | E1.2 |
| P-8 | Release-notes pointer resolves after C1 | `python3 tools/release/resolve_release_notes.py --repo-root .` exit 0 | C1 |
| P-9 | The print golden suite is green on the base | `cd apps/desktop/src-tauri && cargo test --locked -j 8 print_golden -- --test-threads=8` → 24 passed | E1.1 (baseline) |

## 2. Normative requirements

| ID | Requirement | Verified by |
|---|---|---|
| NR-1 | No PCGen template, XSL, token or Java in `src/`, `apps/desktop/**`; the reference render lives under `artifacts/` only | `pcgen-residue-gate` stage; E0.1's residue check (`git diff --name-only <base>..HEAD -- apps src crates data resources \| awk 'END{print NR}'` → 0) |
| NR-2 | Every number printed comes from an explanations row or the saved DTO; no Typst look and no TSX re-derives a number | E2.MC, E3.MC review; `grep -rn 'fn compute\|calc(' apps/desktop/src-tauri/src/print/ \| awk 'END{print NR}'` → 0; `grep -rnE '\b(calc|eval)\(' apps/desktop/src-tauri/resources/print/ \| awk 'END{print NR}'` → 0 |
| NR-3 | PDF output deterministic for identical input | golden: two renders byte-equal (exists) |
| NR-4 | A4 and Letter lay out with Typst diagnostics = 0 | golden (exists); every look card quotes `warnings: []` |
| NR-5 | SF1e and CPR golden text and page counts equal C1's for Basic, Spellbook-equivalent and Full | G-2 |
| NR-6 | Every PF print block id is in `block_vocabulary.json`; every `ui`-sourced id exists in the registry or tab list | `print_pf_blocks` + `sheetLayout.test.ts` (G-3) |
| NR-7 | Every look has a template file and every template a variant | `print_pf_blocks_every_look_has_its_template` |
| NR-8 | Page 1 of Aldric Basic holds the five step-1 blocks on one page, both papers | G-4 |
| NR-9 | Presets are data: adding one is a list in `layout/pf1e.rs`; the dialog offers exactly the four | G-5 |
| NR-10 | Density ceilings hold on both papers (`pages_max` per case) | G-6 |
| NR-11 | Every look/layout card's receipt has a "Looked at" row naming its PNGs and the reference compared | `grep -L 'Looked at' artifacts/epic_[1-4]/*_cycle_receipt.md` prints only the non-look cards named in G-8 |
| NR-12 | Write-in space exists where `decisions.md §11` lists it | E2.MC, E3.MC, E5.2 |
| NR-13 | PF seed renders unchanged (no DTO, no engine value moves) | G-1 |

## 3. Out of scope

DEF-1…DEF-4 and FSR-C1…C6 (`forward-scope-register.md`); any engine or converter change; any book
content; SF1e and CPR looks; the Hermes kanban (retired).
