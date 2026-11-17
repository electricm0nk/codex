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
| E1 import converter (lane `import`) | 8 | 1 (E1.1) | E1.1 complete; E1.2 next |
| E2 campaign folder (lane `campaign`) | 14 | 0 | waiting on C1 |
| E3 editor (lane `editor`) | 4 | 0 | waiting on C1 |
| E4 desktop wiring (Phase 2) | 7 | 0 | waiting on E1.MC, E2.MC, E3.MC, SD-41 merged |
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

## Epic reports

(Each merge check appends its Epic report block here: cards, counts before → after, retro summary for the
epic's window, what it looked at, what it fixed.)

## Lead's dispatch log

(Per card: the R21 RED output, the R22 `git grep` of producers and second call sites, the H rows it rests on
(R28), `git reflog -3` and `git worktree list` before dispatch.)

## Open blockers

(none)
