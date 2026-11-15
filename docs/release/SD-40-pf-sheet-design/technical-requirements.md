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
| P-9 | The print golden suite is green on the base | `cd apps/desktop/src-tauri && cargo test --locked -j 8 print_golden -- --test-threads=8` → 24 passed (30 after E4.1) | E1.1 (baseline) |

## 2. Normative requirements

| ID | Requirement | Verified by |
|---|---|---|
| NR-1 | No PCGen template, XSL, token or Java in `src/`, `apps/desktop/**`; the reference render lives under `artifacts/` only | `pcgen-residue-gate` stage; E0.1's residue check (`git diff --name-only <base>..HEAD -- apps src crates data resources \| awk 'END{print NR}'` → 0) |
| NR-2 | Every number printed comes from an explanations row or the saved DTO; no Typst look and no TSX re-derives a number | E2.MC, E3.MC review; `grep -rn 'fn compute\|calc(' apps/desktop/src-tauri/src/print/ \| awk 'END{print NR}'` → 0; `grep -rnE '\bcalc\.[a-z]+\(|\beval\(' apps/desktop/src-tauri/resources/print/ \| grep -v -e 'calc.min(boxes, header.values.len())' -e 'calc.min(size, 10pt)' -e 'calc.min(1, b.rows.len())' -e 'calc.max(1, calc.ceil(measure(' -e 'calc.max(lines, used.sum(' -e 'calc.max(0, 3 - items(b)' -e 'calc.max(0, b.rows.len() - 1)' -e 'calc.odd(feats.len())' -e 'calc.min(1, starts.len())' -e 'calc.max(totals.len(), 1)' -e 'calc.max(0, calc.floor(room / slot))' \| awk 'END{print NR}'` → 0 (widened by E3.MC for four E3 layout-only hits no receipt ran the grep for: `looks/feats.typ` `calc.odd(feats.len())` (an empty cell closes an odd feat line, E3.1), `looks/gear.typ` `calc.max(totals.len(), 1)` (the totals grid's column count, E3.2), `looks/spells.typ` `calc.min(1, starts.len())` (where each class's rows end, E3.3), `looks/bio.typ` `calc.max(0, calc.floor(room / slot))` (how many ruled lines fit to the page foot, E3.4), none a rules number; corrected by E2.MC; widened by E2.MC3 for four layout-only hits no receipt named: `looks/conditional-bonuses.typ` `calc.max(1, calc.ceil(measure(…).height / slot))` (how many ruled lines a row's words take) and `calc.max(lines, used.sum(default: 0) + blank-min)` (the box's line count, E2.20), `looks/weapons.typ` `calc.max(0, 3 - …)` (blank weapon lines up to three, E2.22), `looks/saves.typ` `calc.max(0, b.rows.len() - 1)` (the column count, E2.18), none a rules number; widened by E2.MC2 for `looks/name-campaign.typ`'s two layout-only hits, E2.5's `calc.min(1, b.rows.len())` (which rows follow the name) and E2.4's `calc.min(size, 10pt)` (the wrapped name's font size), neither a rules number: the earlier `\b(calc|eval)\(` never matches Typst's arithmetic module, `calc.min(`/`calc.round(`; the one allowed hit, `looks/weapons.typ` `calc.min(boxes, …)`, bounds a slice of captions and prints no number; any other hit is named in the receipt and read) |
| NR-3 | PDF output deterministic for identical input | golden: two renders byte-equal (exists) |
| NR-4 | A4 and Letter lay out with Typst diagnostics = 0 | golden (exists); every look card quotes `warnings: []` |
| NR-5 | SF1e and CPR golden text and page counts equal C1's for Basic, Spellbook-equivalent and Full | G-2 |
| NR-6 | Every PF print block id is in `block_vocabulary.json`; every `ui`-sourced id exists in the registry or tab list | `print_pf_blocks` + `sheetLayout.test.ts` (G-3) |
| NR-7 | Every look has a template file and every template a variant | `print_pf_blocks_every_look_has_its_template` |
| NR-8 | Page 1 of Aldric holds the eleven page-1 blocks of `decisions.md §23.3` whole on one page, both papers; a page-1 list too long for its cell continues on page 2, no block leaves page 1 | G-4 |
| NR-9 | Sections are data: each is a list in `layout/pf1e.rs`; the dialog offers exactly the eight (four on SF1e/CPR; Notes off by default, `decisions.md §24.9`), each section on its own pages under its title, remembered per character, first time 1 + 2 + 3 (`decisions.md §23.4`, §24.3.7) | G-5 |
| NR-10 | Page counts are recorded, not capped: every look card's receipt lists each PF case's page count before → after on both papers; no test asserts a page-count ceiling (`decisions.md §10`, §22) | receipts' "Golden deltas" rows; G-6 |
| NR-11 | Every look/layout card's receipt has a "Looked at" row naming its PNGs and the reference compared | `grep -L 'Looked at' artifacts/epic_[1-4]/*_cycle_receipt.md` prints only the non-look cards named in G-8 |
| NR-12 | Write-in space exists where `decisions.md §11` lists it | E2.MC, E3.MC, E5.2 |
| NR-13 | PF seed renders: no existing field or explanations row changes value; only E2.6–E2.8 and E2.14's named fields and rows are added (`decisions.md §23.2`) | G-1 (rewritten) |
| NR-14 | The 14 stale ui-smoke rows (F-21) pass by name under `npm run ui-smoke`; none deleted, skipped or marked manual | E0.2; G-9 |
| NR-15 | Page 1 prints totals only; every breakdown it dropped prints on the Show-the-math pages; the AC prints all seven parts from engine rows (`decisions.md §23.1` item 4) | E2.10–E2.12, E3.5 tests; E2.MC2, E3.MC |
| NR-16 | The new engine values (conditional notes, AC parts, remaining lb, languages, weapon damage total) are computed in Rust once; the screen's TypeScript copies of remaining lb and languages are removed (AGENTS.md twin rule) | E2.6–E2.8, E2.14 tests and greps |

## 3. Out of scope

DEF-1…DEF-4 and FSR-C2, C3, C5, C6, C8 (`forward-scope-register.md`; FSR-C1 is Committed as COM-9, card E0.2; FSR-C4 as COM-10); any converter change; any engine change beyond E2.6–E2.8 and E2.14's named rows and fields; any book
content; SF1e and CPR looks; the Hermes kanban (retired).
