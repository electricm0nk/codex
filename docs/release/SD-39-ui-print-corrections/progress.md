---
canonical: true
bundle_id: SD-39
slug: ui-print-corrections
canonical_branch: tranche-19
target_version: 0.19.0
date: 2026-11-01
status: planning-ready
---

# SD-39 Progress

## Summary

| Field | Value |
|---|---|
| Cards complete | 9 of 53 (`awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/ && $5 ~ /^ complete *$/' kanban.md \| awk 'END{print NR}'`) |
| Run id | — (written at launch, `workflow-instruction.md §2.4` item 9) |
| Pinned `tranche-19` SHA | `49d598c13a1bd19ffd02106e23fa0f7671083a55` (C1 version commit 0.19.0; cut at C0 from `origin/develop` @ `2b14020bc7`) |

## Cycle 0 — package authored (2026-11-01, planning session, Fable 5.1)

- Source: the operator's 12 dictated defects and 16 grill answers (`artifacts/cycle_0/defects-and-rulings-2026-11-01.md`).
- Repo facts checked on `develop` @ `5ef2f17b67` local / `2b14020bc7` origin: no PDF crate, no fonts,
  one `window.print()` site, prerequisites already enforced on feat add, no budget or duplicate guard,
  `ActiveState` already has worn/carried values, converter parses and drops `SLOTS`, 14 version surfaces,
  peer worktree `fix-perform-install` present.
- Author calls without an operator ruling: `README.md §6` rows 1–9.
- Operator ruling 2026-11-01: "commit now without review". The lead cut `tranche-19` from `origin/develop` and committed this package as C0; C1 still owns the version bump.
- **UNATTENDED MODE acknowledged 2026-11-01:** the operator launched and walked away; rulings R17–R20 taken first; `decisions.md §16`, §24, §25. E9.4 is the one planned operator wait.
- Lead re-read the package against the repo before dispatch; 9 corrections in `decisions.md §25`.
- Merged `fix-perform-install` worktree and local branch removed (R20; 0 dirty files, 0 unmerged commits). `sd39-main` created; the main checkout is back on `develop`.

### Placeholder gate output (author, 2026-11-01)

Command: `grep -rhoE '<[a-z_-]+>' docs/release/SD-39-ui-print-corrections/*.md | sort | uniq -c | sort -rn`
run 2026-11-01 before commit. Every token is in the `workflow-instruction.md §9` allowed list
(schema placeholders, command arguments a card binds, message/pattern placeholders such as `<pool>`,
`<price>`, `<gold>`); zero bugs. Deferred values: `(E1.0)`, `(E2.4)`, `(E7.1)` figures in
`content-unit-inventory.md`, `0.19.<run>`, the run id.

## Cycle log

| Cycle | Card | Status | SHA | Receipt |
|---|---|---|---|---|
| C1 | C1 | blocked-escalated | 49d598c13a | artifacts/cycle_1/C1_cycle_receipt.md |
| E1 | E1.0 | complete | 5014fb84a9 | artifacts/epic_1/E1.0_cycle_receipt.md |
| E1 | E1.4 | complete | 18ca8b657c | artifacts/epic_1/E1.4_cycle_receipt.md |
| E1 | E1.1 | complete | 1de994845f | artifacts/epic_1/E1.1_cycle_receipt.md |
| E1 | E1.2 | complete | 073780a2bb | artifacts/epic_1/E1.2_cycle_receipt.md |
| E1 | E1.3 | complete | 42a1c821b3 | artifacts/epic_1/E1.3_cycle_receipt.md |
| E1 | E1.5 | complete | ec9f0becf4 | artifacts/epic_1/E1.5_cycle_receipt.md |
| E1 | E1.MC | complete | this commit | artifacts/epic_1/E1.MC_cycle_receipt.md |

## Epic reports

(The merge-check card of each epic writes a one-screen block here: cards done, figures with commands,
seed deltas, open blockers.)

### Epic report — E1 (guards), merge check on `origin/tranche-19` @ `d3d215d4d9`

**Cards done:** E1.0, E1.1, E1.2, E1.3, E1.4, E1.5, E1.MC. **Not closed:** E1.6, discovered here, `ready`, row-order next. The epic's guards are not finished until it lands.

| Figure | Value | Command |
|---|---|---|
| Root tests | 6,724 passed, 0 failed, 300 result lines, exit 0 | `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8`; `awk` and Python over the log agree |
| `codex-ingest` tests | 1,843 passed, 0 failed, 177 result lines, exit 0 | same, with `-p codex-ingest` |
| Desktop tests | 918 passed, 0 failed, 2 result lines, exit 0 | same, in `apps/desktop/src-tauri` |
| Named filters, in those logs | `slot_of` 5, `feat_budget` 10 root + 11 desktop, `feat_duplicate` 14; 0 `FAILED` | Python regex over the three logs |
| Frontend | typecheck exit 0; 176/176 test files | `cd apps/desktop && npm run typecheck && npm test` |
| ui-smoke, E1.5's 3 rows | 3 of 3 green | `npm run ui-smoke -- --only <ids>`, `E1.MC/smoke-results.json` |
| ui-smoke, 8 rows chosen for feats or level-up | 6 green, 2 red: both `level-up-fighter6-*`, at the Create form's old level select, step 7, so the feat step never ran. E1 changed no select in that form; FSR-C2 stale rows, E2.4 owns them | same file |
| Equipment corpus | 7,386 records, 2,906 with a table tag, 1,617 with a `Slot:` line, every line value in `slot_from_words` | a new Python walk, `E1.MC` receipt |
| `repeatable` = `MULT:YES` | 2,483 agree, 0 disagree, of 2,483 principal `pool == feat` records | a new Python walk over the pinned oracle's rows (E1.0's test reads 2,927 rows, all rows of the feat files) |
| Enhanceable | 554 principals | the same tag list in Python |
| Generator round trip | `data/equipment_types.json` unchanged by a re-run; `sheet_rule_convert --check` exit 0 for both systems; `pcgen-residue-gate` PASS | `gen_equipment_types`, then `git status --porcelain` |
| G-2 | 0 files under `data/sheet_rules` | `git diff --name-only 49d598c13a..HEAD -- data/sheet_rules \| awk 'END{print NR}'` |
| Create offers what saves | 189 saved and none over budget; 222 refused, all `prestige_class.requires_base_class_levels`; 0 other refusals. 137 census classes × (Human 1, Elf 1, Human 5), picking the number the form allows | `E1.MC/probe-tests.rs.txt`, `E1.MC/book-and-create-probe.txt` |

**Seed deltas (base = C1 `49d598c13a`, built in its own tree; head = `d3d215d4d9`):** PF Aldric, Elowen, Tomas: 27 paths, 9 each, all `additive-field`, all `explanations/feats.budget.*`, no existing value changed (`E1.MC/seed-deltas-pf.txt`). SF ×4 and CPR ×5: `diff -r` empty, files present on both sides.

**Refuted:** E1.1's "every pool the class catalog and race grant". Swashbuckler and Gunslinger read no pool at any level, so an Elf Swashbuckler 4 (level 2 + bonus 1 by the book) is refused its third feat; Dragon Disciple and Sentinel are the same miss in prose; a Hunter reads twice the book. All in E1.6.

**Retro (`scripts/retro.py summary --since 2026-11-01T15:44:40-05:00 --json`):** 14 corrections, 6 verification runs with 0 failed, 0 incidents, 0 reworks, 0 deferrals. No recurrence key, so no new check under R-W. Most corrections are of the package's own text again: `lead` 4, package rows 5.

**Open blockers:** none for the operator. E9.4's eyeball still has the only look at chip pixels and the badge.

## Decisions taken on safe defaults

| Card | Decision point | Default taken | Alternative not taken |
|---|---|---|---|

## Open blockers

(none)

## Run handle

| Run id | START | Script path | Started |
|---|---|---|---|
| wf_e22fdf36-bc3 | C1 | session `workflows/scripts/sd39-ui-print-corrections-wf_e22fdf36-bc3.js` | 2026-11-01; stopped at C1 on a too-wide acceptance grep; the lead cleared it |
