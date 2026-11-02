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
| Cards complete | 17 of 54 (`awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/ && $5 ~ /^ complete *$/' kanban.md \| awk 'END{print NR}'`) |
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
| E2 | E2.1 | complete | 7d935657cb | artifacts/epic_2/E2.1_cycle_receipt.md |
| E2 | E2.2 | complete | f321378d43 | artifacts/epic_2/E2.2_cycle_receipt.md |
| E2 | E2.3 | complete | c10cbc057c | artifacts/epic_2/E2.3_cycle_receipt.md |
| E2 | E2.4 | complete | 22688208ec | artifacts/epic_2/E2.4_cycle_receipt.md |
| E2 | E2.MC | complete | this commit | artifacts/epic_2/E2.MC_cycle_receipt.md |
| E3 | E3.1 | complete | 93c596cc1c | artifacts/epic_3/E3.1_cycle_receipt.md |
| E3 | E3.2 | complete | 61aad6b9ab | artifacts/epic_3/E3.2_cycle_receipt.md |
| E3 | E3.3 | complete | f905c625d2 | artifacts/epic_3/E3.3_cycle_receipt.md |
| E3 | E3.4 | complete | af1e860a9c | artifacts/epic_3/E3.4_cycle_receipt.md |
| E3 | E3.5 | complete | 9efc712f1d | artifacts/epic_3/E3.5_cycle_receipt.md |

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

### Epic report — E2 (layout), merge check on `origin/tranche-19` @ `cb790d950f`

**Cards done:** E2.1, E2.2, E2.3, E2.4, E2.MC. No card discovered.

| Figure | Value | Command |
|---|---|---|
| Frontend | typecheck exit 0; 177/177 test files; inventory 133 rows, 18 screens | `cd apps/desktop && npm run typecheck && npm test && npm run ui-smoke:doc` |
| E2.4's rows + the two it edited | 6 of 6 green | `npm run ui-smoke -- --only <ids>`, `E2.MC/final-rows-results.json` |
| Rows E2.4 left blocked, run alone | 49 green, 3 red, of 52 | `npm run ui-smoke -- --only <52 ids>`, `E2.MC/smoke52-results.json` |
| Full suite, two runs added | 113 green, 17 red, 3 manual, of 133 | `python3` counting `status` in `E2.4/full-suite-results.json` + that file. Separate sessions, never one run |
| The 17 reds | 11 created character not in Load (the row never presses Add level), 4 old level select, 1 `character-class`, 1 `Magic Items` | their `reason` fields; none in a file E2 changed |
| E2's Rust, data, lockfile | 0 files | `git diff --name-only becaabd23c..cb790d950f -- '*.rs' data 'Cargo.*' \| awk 'END{print NR}'` |
| Audit tests | sd24 wired 5, sd24 identifier 1, sd26 identifier 1; 0 failed | `cargo test --locked -j 8 --test <each> -- --test-threads=8` |
| Combat row wrap | beside at 1400 wide, under at 1300; default window 1920 | `xdotool windowsize` on the running app, `E2.MC/width-*.png` |

**Seed deltas (base = `becaabd23c`, built in its own tree; head = `cb790d950f`):** PF Aldric, Elowen, Tomas: `sha256.txt` identical. SF x4 and CPR x5: `diff -r` empty, files on both sides (`E2.MC/seed-deltas.txt`).

**Refuted, both fixed here:** E2.1's acceptance grep printed 1, matching the `Show Progression` label; the package's command is corrected (`decisions.md §25`). E2.4's title row stayed green with the rail heading put back to `Progression`; it now forbids `PROGRESSION`, and the same mutation goes red. **Holds:** click-outside and focus return with a real pointer, the wrap, the column placement at 1920, and no moved panel on the SF or CPR sheets (R15 has nothing to move there).

**Retro (`scripts/retro.py summary --since 2026-11-02T02:00:47-05:00 --json`):** 7 corrections (E2.1, E2.2, E2.3 one each; E2.MC 4), 0 incidents, 0 deferrals, no repeated subject, so no new check under R-W. Package rows again the most common subject: 4 of 7.

**Open, for the lead, no operator ruling needed:** E1.6 is `ready` and nothing will dispatch it. The run script drains a prefix only before that prefix's own merge check, and E1.MC discovered it. Drain `E1` before the next merge check, or E9.5's scan stops the run on it at closure.

## Decisions taken on safe defaults

| Card | Decision point | Default taken | Alternative not taken |
|---|---|---|---|
| E3.1 | Which list a magic suit of armor is on | Magic item only: a magic marker wins | Armor as well, which would also put tagged rods on Weapon |
| E3.1 | An arms row with no type tags | Gear, still buyable | A kind guessed from its name |
| E3.1 | Unpriced template twins, `Codex-Named Unit` rows | Hidden | Shown, as the old picker did |
| E3.2 | A `+N` sale's refund when the bonus was attached free through Attach Modifier | The copy's offer price, what a `+N` sells for | A record of what was paid, which needs a new field; the free attach is the defect, E3.6 |
| E3.2 | How a sale names a copy | `bonus` on the request: `0` plain, `N` a `+N`, absent the first | A list index, which the sheet's rows do not carry |
| E3.3 | State a Market weapon is saved in | Carried, and the status line says so: the engine refuses a second wielded weapon | Wielded, which refuses every weapon on a Computed character; a retry after the refusal, which is E4.1's rule in the client |
| E3.3 | How many rows the dialog renders | The first 200 by name, with a line saying so; search reaches the rest | All of the Magic item list at once (3,680 rows, E3.1's receipt) |
| E3.4 | A `+N` on an item the demo corpus did not hold | Bundle the Market's arms and armor and the 15 modifiers, as converted records | A modifier counted with its host missing, which reads a `+1 Breastplate` as AC +1; loading the whole corpus, which the desktop's own header rules out |
| E3.4 | A second real weapon in hand | Allowed: the Melee number is the Longsword's, others have their own row | Keep the refusal, which left a bought Longsword impossible to hold |
| E3.4 | Offers that cannot show | Stop offering them, by a rule reading the same corpus | Named exceptions in a test, which leaves the charge standing |
| E3.5 | How Starfinder honours three price modes when it derives credits from what is held | One saved `selected_choices` row the engine adds to Credits remaining | Hiding the control there, or a sidecar `compute_sheet` cannot see |
| E3.5 | Cyberpunk RED starter items the shop has no name for | On the Sell list, for nothing, and the line says so | A hand-kept name table (R2) |

## Open blockers

(none)

## Run handle

| Run id | START | Script path | Started |
|---|---|---|---|
| wf_e22fdf36-bc3 | C1 | session `workflows/scripts/sd39-ui-print-corrections-wf_e22fdf36-bc3.js` | 2026-11-01; stopped at C1 on a too-wide acceptance grep; the lead cleared it |
