---
canonical: true
owner: operator
bundle_id: SD-42
slug: pcgen-import-and-campaign-folder
canonical_branch: tranche-22
target_version: 0.22.0
date: 2026-11-16
status: planning-ready
---

# SD-42 Technical Requirements

> **`decisions.md §20` wins where this file differs.**

## 1. Pre-launch prerequisites (each with its check; C1 runs P-1, P-2, P-4, P-7, P-8 and pastes the output; the others at the card named)

| ID | Prerequisite | Check | Closes at |
|---|---|---|---|
| P-1 | `tranche-22` exists on origin at `tranche-21`'s tip of 2026-11-16 | `git merge-base --is-ancestor 1046daefc0 origin/tranche-22` | C1 |
| P-2 | Working tree clean; `sd41-main` untouched | `git status --porcelain \| awk 'END{print NR}'` → 0; `git worktree list` recorded | C1 |
| P-3 | **SD-41 merged and `tranche-22` caught up** (the Phase-2 precondition, R25) | `git merge-base --is-ancestor <SD-41 merge sha> HEAD` in `sd42-main`; `kanban.md` E4.1's `SD-41 merged` token replaced by the SHA | E4.1 (the lead) |
| P-4 | Doctrine present | `test -f docs/governance/no-stub-mvp-doctrine.md && test -f docs/doctrine-external/identifier-discipline.md` | C1 |
| P-5 | Disk ≥ 24 G free before any full sweep; cargo lanes on the box < 4 | `df -h /`; `pgrep -x cargo \| awk 'END{print NR}'` | every cargo card |
| P-6 | The PCGen oracle at the pin with the sample files | `source <(bash scripts/fetch-pcgen-oracle.sh --print-env 2>/dev/null \|\| true); git -C "${PCGEN_REPO_DIR:-$HOME/workspace/repos/pcgen}" rev-parse HEAD` = `PCGEN_ORACLE_SHA` in `scripts/pcgen-oracle-pin.env`; `ls "${PCGEN_REPO_DIR:-$HOME/workspace/repos/pcgen}/code/testsuite/PCGfiles/" \| grep -cE '^(pf_Cleric\|pf_Paladin\|pf_Rogue\|pf_goldielocks\|sf_mechanic\|sf_mechanic_drone\|sf_soldier)\.pcg$'` → 7 | E1.1 |
| P-7 | Release-notes pointer resolves after C1 | `python3 tools/release/resolve_release_notes.py --repo-root .` exit 0 | C1 |
| P-8 | The operator's file is present on this machine (local proof only) | `test -f "${CODEX_SD42_PCG:-/home/ubuntu/sd42-input/Aonghus Macleod.pcg}" && echo PRESENT \|\| echo ABSENT` (ABSENT is not a failure of C1; it is a failure of E4.5 and E5.G) | C1 (recorded), E4.5 |
| P-9 | The PCGen engine runs (parity) | `java -version` exits 0; `test -x "${PCGEN_REPO_DIR:-$HOME/workspace/repos/pcgen}/gradlew"`; the `pcgen_engine_runnable` guard in `crates/codex-ingest/tests/common/mod.rs:18` returns Ok (the card quotes it) | E1.7 |
| P-10 | The npm registry is reachable for the new packages, and their licences are MIT | `npm view @codemirror/view license` → MIT (and the other ten, `decisions.md §18`) | E3.1 |
| P-11 | `pdftoppm`, `pdftotext`, `pdfinfo` present; the `run-desktop` skill's screenshot path works with an isolated `XDG_DATA_HOME` | `which pdftoppm pdftotext pdfinfo` → three paths; one screenshot under `artifacts/epic_2/E2.MC/` | E2.MC, E4.2 |
| P-12 | `tauri-plugin-opener` is a Cargo dependency (it is: `apps/desktop/src-tauri/Cargo.toml:29`) | `grep -c 'tauri-plugin-opener' apps/desktop/src-tauri/Cargo.toml` → 1 | E2.5 |

## 2. Normative requirements

| ID | Requirement | Enforced by |
|---|---|---|
| NR-1 | **One stack per `EQUIPNAME` line.** The importer never emits more than one `equipmentSelections` row per line; `quantity` is `u32` with no cap; a `0` line is not held | E1.2 tests; E4.1's held-list test (`decisions.md §5`) |
| NR-2 | **The engine owns every number.** No TS and no Typst sums a weight, a price or rolls a die; the three copy overrides are read in one engine place each; `properties` is text | `grep -nE 'weight.*\+=\|\* *quantity' apps/desktop/src/**/*.ts*` read by every MC; `decisions.md §24` |
| NR-3 | **No PCGen in live code.** The sidecar is the only `.pcg` reader; the oracle is a test input; `pcgen-residue-gate` green | G-7 |
| NR-4 | **The operator's file never enters the repo** — not as a fixture, a seed, a screenshot, a log or a receipt line; receipts carry counts | the pre-commit grep in the prefix (`workflow-instruction.md §2.1`: `git diff --cached -S"$SD42_CHAR" --stat -- . ':!docs/release/SD-42*'` must be empty, with `SD42_CHAR` the test character's name set locally by the lead); G-8 |
| NR-5 | **A new character every import**; no existing root is read or written by `import_pcg` | E4.1 test |
| NR-6 | **The folder is the only truth** for a campaign; no localStorage campaign model, no `.config/<name>.json` writer, no `drive_*` command remains | E2.3: `grep -rn 'codex.campaigns\|drive_list_campaigns\|drive_save_campaign\|write_campaign_drive' apps/desktop/src apps/desktop/src-tauri/src \| awk 'END{print NR}'` → 0 |
| NR-7 | **Codex never rewrites a file it did not change.** Reads are tolerant (SD-o); writes touch only the file the user edited or the frontmatter key Codex owns (`pins`), preserving the rest of the file byte for byte | E2.7, E2.8 tests; E3.1 round-trip test |
| NR-8 | **Byte-stable editor.** Open + save with no edit = identical bytes (EOL, trailing newline, BOM kept) | E3.1 `roundTrip.test.ts` |
| NR-9 | **Paths from the UI are relative to the campaign folder**; `..` and absolute paths are refused by the Rust side | E2.1 test |
| NR-10 | **Every write is atomic** (temp + rename) and a sync-client rename collision is retried once (SD-r) | E2.1 test |
| NR-11 | **The local save never depends on the backup.** A `Characters/` write failure, counter mismatch or missing folder is a warning on the sheet; the local save has already succeeded | E2.10, E4.3 tests |
| NR-12 | **A stand-in never writes the backup** | E2.10 test |
| NR-13 | **Member #1 exists** for every campaign Codex creates or migrates; the Manager's count is ≥ 1 | E2.2, E2.4 tests |
| NR-14 | **New dependencies are the eleven named in `decisions.md §18` plus `@tauri-apps/plugin-opener`**, all MIT; `package.json` diff lists nothing else | E3.1, E2.5 receipts: `git diff <base>..HEAD -- apps/desktop/package.json` |
| NR-15 | **No stubs.** Every tab, button, pin and menu does what it says (the four-check audit, every cycle) | `workflow-instruction.md §6` step 5 |
| NR-16 | **Every figure carries its command and denominator** | receipts (R13, R-N) |
| NR-17 | **Parity differences are fixed or explained; a stale explanation fails** | E1.7's gate |
| NR-18 | **Obsidian-compatible files:** frontmatter is `---` YAML; links are `[[Page]]`; images `![](Attachments/x)`; one file per entry; no file Codex writes needs Codex to read | E2.x tests open each written file with the tolerant parser and with a plain-text assertion |

## 3. Out of scope

Roles, secrets, invites, any network call; merging stand-in changes; Cyberpunk RED import; companions as
attached records; Obsidian rendering of pins; a file watcher beyond the editor's mtime check; a sanitiser
dependency (raw HTML is escaped, never rendered); any change to the Market or the gear screen beyond showing
a copy's name, properties and overrides.
