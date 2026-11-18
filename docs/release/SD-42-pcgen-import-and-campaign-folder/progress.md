---
canonical: true
bundle_id: SD-42
slug: pcgen-import-and-campaign-folder
canonical_branch: tranche-22
target_version: 0.22.0
date: 2026-11-16
status: planning-ready
---

# SD-42 Progress

## Summary

| Epic | Cards | Complete | Status |
|---|---|---|---|
| C | 2 | 2 (C0, C1) | C complete |
| E1 import converter (lane `import`) | 11 | 11 (E1.1–E1.10, E1.MC) | E1 complete; import lane ended |
| E2 campaign folder (lane `campaign`) | 14 | 0 | waiting on C1 |
| E3 editor (lane `editor`) | 4 | 4 (E3.1–E3.3, E3.MC) | E3 complete; editor lane ended |
| E4 desktop wiring (Phase 2) | 15 | 0 | waiting on E1.MC, E2.MC, E3.MC, SD-41 merged |
| E5 gate + closure | 5 | 0 | waiting on E4.MC |

Re-derive: `awk -F'|' '$2 ~ /^ (C|E)[0-9]/ {gsub(/ /,"",$2); split($2,a,"."); n[a[1]]++; if ($5 ~ /complete/) c[a[1]]++} END {for (e in n) print e, n[e], c[e]+0}' kanban.md`.

## Run handle

(The lead writes the Workflow run id and the script path here the moment each run starts: Phase 1, Phase 2,
the post-gate run. A run with parallel lanes is never resumed, R9.)

## Cycle 0 — package authored (2026-11-16, planning session, Fable 5.1)

- **Tree:** `.worktrees/sd42-plan`, branch `tranche-22` cut from `origin/tranche-21` @ `1046daefc0` while SD-41 runs there (49 of 56 cards complete: E3.12, E4.MC, E4.G, E5.1–E5.4 open).
- **Inputs:** the operator's intake and rulings (`artifacts/cycle_0/operator-brief-2026-11-16.md`); the SD-41 package as the house template; the standing rules (paper-sheet, no PCGen in live code, the SD-36 Epic F changes); the code named in `references/README.md`; the operator's `.pcg` read for **counts only**; the 7 PCGen sample files at the pinned oracle (`7f818006e371188e5717fd18d74d18a420747fc6` = the pin).
- **Method:** scope-draft → decisions → technical-design → technical-requirements → epic-breakdown → workflow-instruction (the chassis order); the import defects diagnosed by reading `map.rs`, `catalog.rs`, `wire.rs`, `character_hub.rs` — not by running the app — and recorded as hypotheses H1–H7 (`technical-design.md §1`) for E1.1 to prove.
- **Figures:** `artifacts/epic_0/census.json` via `python3 -I artifacts/epic_0/census.py`; 37 cards (`awk -F'|' '$2 ~ /^ (C|E)[0-9]/' kanban.md | awk 'END{print NR}'`); same IDs (`SAME_IDS`); 23 rulings traced (every `decisions.md §3` row → a traceability row).
- **Checks run:** stale-path audit → 0; the player's name absent from the package (grep with the name set locally → 0); the character's name only in this package's docs and the local path.
- **Retro:** `docs/retro/events/sd42-plan.jsonl` — 1 `note` (the package), 2 `deferral` (the split not taken; roles/secrets).
- **Status:** C0 complete.

### Placeholder gate output (author, 2026-11-16)

`grep -rhoE '<[a-z_-]+>' docs/release/SD-42-pcgen-import-and-campaign-folder/*.md | sort -u` →
`<ability> <author> <base> <campaign> <campaign_dir> <card-id> <card> <cmd> <date> <device> <die> <dir> <epic-start> <epic> <field> <file> <filter> <folder> <head> <id> <item> <keys> <lane> <letter> <list> <log> <m> <n> <name> <next> <output> <parent> <path> <pidfile> <pkg> <reason> <result> <row> <run> <sample> <script> <sha> <slug> <text> <title> <type> <user> <value> <violations> `
— every token is in `workflow-instruction.md §9`'s list (a) (schema or command placeholders) or (b) (deferred values with a named resolution point).

## Cycle log

| Date | Card | Status | SHA | Receipt | Notes |
|---|---|---|---|---|---|
| 2026-11-16 | C0 | complete | this commit | `artifacts/cycle_0/C0_cycle_receipt.md` | package authored |
| 2026-11-17 | C1 | complete | 589815b65f | `artifacts/cycle_0/C1_cycle_receipt.md` | 0.22.0 on the 14 surfaces; current-release points at SD-42 notes |
| 2026-11-17 | E3.1 | complete | 75e9feca1d | `artifacts/epic_3/E3.1_cycle_receipt.md` | 11 MIT packages pinned; 9 fixtures byte-stable; 10 live-preview + 4 reading-view cases; npm test 201/201 |
| 2026-11-17 | E3.2 | complete | 931098912b | `artifacts/epic_3/E3.2_cycle_receipt.md` | `[[` completion + link clicks/Ctrl-Enter; 4 wikilinks cases; npm test 202/202 |
| 2026-11-17 | E1.1 | complete | the commit adding the E1.1 receipt | `artifacts/epic_1/E1.1_cycle_receipt.md` | diagnosis test 1 passed; 7 TSVs + summary.json; H1 H3 H4 H7 proved, H2 proved (narrower), H5 refuted, H6 not exercised; no discovered card |
| 2026-11-17 | E1.2 | complete | the commit adding the E1.2 receipt (code d148cb1378, 2e17cc889b) | `artifacts/epic_1/E1.2_cycle_receipt.md` | one stack per place; `equipmentSets` on the wire; 6/6 `pcg_import_stacks`, 15/15 `pcg_import`; samples 20→21, 29→31, 41→43 stacks, pieces conserved; discovered E1.8 (bundle pieces) |
| 2026-11-17 | E1.3 | complete | the commit adding the E1.3 receipt (code 0b494a0bfe) | `artifacts/epic_1/E1.3_cycle_receipt.md` | `CopyFacts` + 4 fields (SD-41 file byte-identical); `eqmod_option_key` (137 CRB aliases + catalog-form pass-through); name/SIZE/_DAMAGE/SPROP/COST/WT read; unknown → property + warning; 6/6 `pcg_import_custom`, 2/2 `copy_facts_`, root `--lib` 3144 passed |
| 2026-11-17 | E2.1 | complete | 039dd404a5 | `artifacts/epic_2/E2.1_cycle_receipt.md` | folder model + 9 commands + known folders; desktop filter fixed to `campaign_folder::` (§20) |
| 2026-11-17 | E1.4 | complete | the commit adding the E1.4 receipt (code d7cacc4343) | `artifacts/epic_1/E1.4_cycle_receipt.md` | unmapped line → Create item (weightLbs/priceGp from WT/COST; containerCapacityLbs = what the file puts inside) held at its place; text-only Worn/Carried stats removed; Camel = gear; 3/3 `pcg_import_unmapped`, codex-ingest `--lib` 720 passed |
| 2026-11-17 | E1.5 | complete | the commit adding the E1.5 receipt (code f4b23af04b) | `artifacts/epic_1/E1.5_cycle_receipt.md` | `BioWire.notes`: `## Imported from PCGen (<file>, <ISO date>)` + 9 kinds, once each, file order; 3/3 `pcg_import_notes`, codex-ingest `--lib` 721 passed; discovered E1.9 (diagnosis test rewrites E1.1 artifacts, 3rd time) |
| 2026-11-17 | E2.2 | complete | 2c4fed1ad0 | `artifacts/epic_2/E2.2_cycle_receipt.md` | creator = member #1; typed member list; Manager over known folders; Edit saves via new `save_campaign_json`; `npm test` 204/204 |
| 2026-11-17 | E3.3 | complete | the commit adding the E3.3 receipt (code 084394b746) | `artifacts/epic_3/E3.3_cycle_receipt.md` | toolbar 6 + autosave 4 + imageDrop 3 cases; `save_attachment` 1 passed; desktop full 0 failed; npm test 207/207; SD-k append-only conflict in campaign_folder.rs kept both (retro incident) |
| 2026-11-17 | E3.MC | complete | the commit adding the E3.MC receipt (fix 05aa922dcd) | `artifacts/epic_3/E3.MC_cycle_receipt.md` | adversarial: reading view rendered `[t](<java\tscript:…>)` as a live `javascript:` link — fixed in `safeUrl`, 5th reading-view case; fixture `party-resources.md` ≠ E2.1 starter — replaced; licences MIT 11 of 11; npm test 207/207; no screenshot (no tab mounts the editor) |
| 2026-11-17 | E2.3 | complete | the commit adding the E2.3 receipt (code 80692e2ac0) | `artifacts/epic_2/E2.3_cycle_receipt.md` | Drive path deleted (NR-6 grep 70 → 0, `DRIVE_GONE`); Manager: Open folder…, Forget, not-a-campaign offer; sheet reads the folder; membership dialog over known folders; desktop 1373 → 1366; npm test 206/206; discovered E2.14 |
| 2026-11-17 | E1.6 | complete | the commit adding the E1.6 receipt (code 826b421762) | `artifacts/epic_1/E1.6_cycle_receipt.md` | `GAMEMODE:Starfinder` → `Catalog::load_sf` (load_sheet_rules), `gameSystem` starfinder-1e; sf_soldier + sf_mechanic compute (SF engine), drone refused by name (Q-A); 3/3 `pcg_import_sf`, codex-ingest full 1903 passed / 0 failed |
| 2026-11-17 | E1.7 | complete | the commit adding the E1.7 receipt (code 75f26c7ebb) | `artifacts/epic_1/E1.7_cycle_receipt.md` | parity harness `scripts/oracle_harness/pcg_import_parity/` (run.sh, compare.py, pcg-gear.txt.ftl, 7 exports, explained.tsv 39 rows) + `pcg_import_parity` 1/1: 7 of 7 samples, 515 fields, 257 equal, 258 explained; operator file 159 fields, 102 equal, 57 explained (local ledger); PF totals desktop-only → discovered E4.13; converter-side rows → discovered E1.10 |
| 2026-11-17 | E2.4 | complete | the commit adding the E2.4 receipt (code bef3e6c2c8) | `artifacts/epic_2/E2.4_cycle_receipt.md` | `migrate_campaign` + `src/campaign/migrate.rs` (localStorage record or `.config/` folder → new folder; backup first; `.json.bak`; idempotent); migration screen from the Manager + Open folder…; root lib 3155 → 3159; desktop 1508 passed; npm test 207/207 |
| 2026-11-17 | E1.8 | complete | the commit adding the E1.8 receipt (code d3546f488b) | `artifacts/epic_1/E1.8_cycle_receipt.md` | `place_copies`: a mapped stack holds `QUANTITY × record_base_quantity` pieces (the engine's `copy_bundles` divisor); `stacks_bundle` 2/2 (synthetic 2×10 bullets = 20 pieces, 10 lb = WT × QUANTITY; pf_Rogue, pf_Cleric 20 pieces); parity compares gear in PCGen's bundles, 257/258 unchanged |
| 2026-11-17 | E1.9 | complete | the commit adding the E1.9 receipt (code ea52e721f4) | `artifacts/epic_1/E1.9_cycle_receipt.md` | diagnosis test writes to CARGO_TARGET_TMPDIR unless PCG_DIAGNOSIS_REFRESH=1; 2 passed; E1.1 dir status 4 → 0 |
| 2026-11-17 | E2.5 | complete | the commit adding the E2.5 receipt (code 20257d121c) | `artifacts/epic_2/E2.5_cycle_receipt.md` | help dialog (7 sections, live tree via `list_campaign_tree`, template otherwise, Obsidian link via `openUrl`); `opener:default`; Help button on Manager, sheet, Create, Edit, Migration; npm 208/208; desktop 1508 passed 0 failed |
| 2026-11-17 | E2.6 | complete | the commit adding the E2.6 receipt (code fad0509fa6) | `artifacts/epic_2/E2.6_cycle_receipt.md` | Party Resources tab mounts `MarkdownEditor` on `Party Resources.md`; "Create starter file" via `create_party_resources_starter` (never overwrites); `[[links]]` open in the tab; root `party_resources_` 2 passed; desktop 1509 passed 0 failed; npm 209/209 |
| 2026-11-17 | E1.10 | complete | the commit adding the E1.10 receipt | `artifacts/epic_1/E1.10_cycle_receipt.md` | code b56d3a43c8; `E1.10` ledger rows 22 → 0: SF selection lines become engine picks (Rapid Response, Opening Volley), pieces in no `EQUIPSET` entry Not Carried, the harness names a weapon by its copy and lists engine-held feats (4 rows deleted); 1 codex-rule, 5 oracle-departs (3 oracle runs), 12 → discovered E4.14 (engine/data); operator's local ledger 9 rows re-classified (counts only); codex-ingest 187 suites, 1907 passed |
| 2026-11-17 | E2.7 | complete | the commit adding the E2.7 receipt (code e9914545d3) | `artifacts/epic_2/E2.7_cycle_receipt.md` | `src/campaign/frontmatter.rs` (tolerant YAML subset; `set_key`/`write_key` change one key's lines) + `adventure_log.rs` (`list` newest session first, ranges by start, SD-o listed by filename; `new_entry` refuses a taken name); `list_adventure_log`/`new_adventure_log_entry`; `AdventureLogTab` list + "+" + editor; root `adventure_log_` 5, `frontmatter_` 7; desktop 1510 passed 0 failed; npm 210/210 |
| 2026-11-17 | E1.MC | complete | the commit adding the E1.MC receipt | `artifacts/epic_1/E1.MC_cycle_receipt.md` | adversarial: Notes lacked every line kept as a custom feat (81 over 6 samples) and a Create item's `CUSTOMIZATION` (Q2/Q5) — both fixed; partition command corrected (branch-wide 81 → lane-own 0); codex-ingest 187 suites / 1912 passed / 0 failed; root lib 3156 passed; residue PASS; import lane ended |

## Epic reports

(Each merge check appends its Epic report block here: cards, counts before → after, retro summary for the
epic's window, what it looked at, what it fixed.)

### Epic 3 — editor (E3.MC, 2026-11-17)

- **Cards:** 4 of 4 complete (E3.1 `75e9feca1d`, E3.2 `931098912b`, E3.3 `084394b746`, E3.MC `05aa922dcd`); no discovered card. Re-derive: `awk -F'|' '$2 ~ /^ E3\./ && $5 ~ /complete/' kanban.md | awk 'END{print NR}'` → 4.
- **Counts before → after (this check):** reading-view cases 4 → 5; `npm test` 207 of 207 test files → 207 of 207 (`cd apps/desktop && npm test`, last line); editor test files 7 of 7 PASS; licences MIT 11 of 11 (`artifacts/epic_3/E3.MC/licences.txt`); round trip 9 of 9 fixtures exact on the unedited save, `party-resources.md` now byte-equal to E2.1's starter.
- **Retro (window `2026-11-17T06:59:28-05:00` → now, actors sd42-e3.1/.2/.3/.mc, sd42-editor):** `python3 scripts/retro.py summary --since 2026-11-17T06:59:28-05:00 --actor sd42-e3.1 --actor sd42-e3.2 --actor sd42-e3.3 --actor sd42-e3.mc --actor sd42-editor` → 4 events: 2 correction (E3.MC: reading-view `safeUrl`; the fixture), 1 incident (E3.3 rebase conflict in `campaign_folder.rs`, SD-k), 1 verification (0 failing stages). No key fired more than once (R-W: no new control owed).
- **Looked at:** nothing rendered — no tab mounts `MarkdownEditor` (`git grep -n "MarkdownEditor\|editor/" -- apps/desktop/src ':!apps/desktop/src/editor' | awk 'END{print NR}'` → 0). E2.6 and E2.MC take the first screenshots.
- **Fixed:** a stored-XSS path in the Reading view (a `javascript:` scheme split by a tab or prefixed by a control char passed `safeUrl`; a shared campaign folder is untrusted input); the starter fixture.
- **Not fixed (named):** the task-box widget toggles text on a read-only (> 2 MB) file (unsaved; no DOM harness to prove a fix); 0 of the editor's DOM event paths run under a test — E2.6/E2.MC/E5.G are their first exercise.

### Epic 1 — import converter (E1.MC, 2026-11-17)

- **Cards:** 11 of 11 complete (E1.1–E1.10, E1.MC); discovered during the epic: E1.8, E1.9, E1.10 (landed), E4.13, E4.14 (Phase 2). Re-derive: `awk -F'|' '$2 ~ /^ E1\./ && $5 ~ /complete/' kanban.md | awk 'END{print NR}'` → 11.
- **Counts before → after (E1.1's diagnosis → the final converter; `artifacts/epic_1/E1.MC/`):** engine rows 48→22, 42→32, 57→44 (pf_Cleric/Paladin/Rogue, one row per line, not per copy); stacks per sample 21 / 31 / 43 / 4 / 22 / 4 over 20 / 29 / 41 / 4 / 22 / 4 lines (`final-converter-wire-counts.tsv`); sf_mechanic custom equipment records 22 → 3, sf_soldier 4 → 0 (SF records read); warnings pf_Cleric 5→0, pf_Paladin 11→3, pf_Rogue 11→2; parity 7 of 7 samples equal-or-explained (`pcg_import_parity` ok). Operator's file: 89 lines → 90 stacks, 333 pieces (counts only).
- **Retro (actors sd42-e1.1 … sd42-e1.10, sd42-e1.mc, sd42-import; `python3 scripts/retro.py summary --since 2026-11-16 --actor sd42-e1.2 … --actor sd42-e1.mc --actor sd42-import`):** before this card 33 events — 18 verification (0 failing stages), 10 corrections (5 against the E1.2/E1.3 rows of `epic-breakdown.md`), 5 incidents; this card adds 3 corrections + 1 incident. Recurring key: `diagnosis-test-rewrites-frozen-artifacts` ×4 — its control is E1.9 (the test writes under `CARGO_TARGET_TMPDIR`), re-checked here: 2 runs, 0 E1.1 files dirtied (R-W met). New key `pgrep-wait-loop-matches-itself` ×1 (fired once; the memory note already names the control).
- **Looked at:** nothing rendered (the import lane has no screen; E4.1/E4.2 take the first screenshots).
- **Fixed by this check:** Q5 — every line kept as a custom feat is now a Notes line (81 lines over 6 samples, 21 in the operator's file were missing); Q2/Q5 — a Create item keeps its `CUSTOMIZATION` fields as stats and Notes lines (1 of the operator's 4, 1 of sf_mechanic's 3); the package's partition command (branch-wide → the lane's own commits).
- **Not fixed (named):** Create-item eqmods are text, not priced; a `TEMPLATESAPPLIED:[NAME:Human]` line names its custom feat `[NAME:Human]`; the diagnosis instrument's `verdict` (PF gate) and `data_fields_not_read` (static) columns no longer measure the converter — E1.1's frozen evidence, not rewritten.

## Lead's dispatch log

(Per card: the R21 RED output, the R22 `git grep` of producers and second call sites, the H rows it rests on
(R28), `git reflog -3` and `git worktree list` before dispatch.)

## Open blockers

(none)
