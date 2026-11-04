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
| Cards complete | 31 of 59 (`awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/ && $5 ~ /^ complete *$/' kanban.md \| awk 'END{print NR}'`) |
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
| E3 | E3.6 | complete | c896b64374 | artifacts/epic_3/E3.6_cycle_receipt.md |
| E3 | E3.MC | complete | 6adc8d39c3 | artifacts/epic_3/E3.MC_cycle_receipt.md |
| E4 | E4.1 | complete | 660246480a | artifacts/epic_4/E4.1_cycle_receipt.md |
| E7 | E7.1 | partial | 7cfaf2065f | artifacts/epic_7/E7.1_cycle_receipt.md |
| E7 | E7.1 re-dispatch | declined, ownedBy:E7.3 | this commit (docs only) | artifacts/epic_7/E7.1_cycle_receipt.md |
| E4 | E4.2 | complete | 960de1b200, 3d91eab9a9 + receipt commit | artifacts/epic_4/E4.2_cycle_receipt.md |
| E4 | E4.3 | complete | 2123327b7b + receipt commit | artifacts/epic_4/E4.3_cycle_receipt.md |
| E4 | E4.4 | complete | ab1300e371 + receipt commit | artifacts/epic_4/E4.4_cycle_receipt.md |
| E4 | E4.5 | complete | efdf8ce4b2 + receipt commit | artifacts/epic_4/E4.5_cycle_receipt.md |
| E4 | E4.6 | complete | 5c21256395 + receipt commit | artifacts/epic_4/E4.6_cycle_receipt.md |
| E4 | E4.7 | complete | b28d044e20 + receipt commit | artifacts/epic_4/E4.7_cycle_receipt.md |
| E4 | E4.MC | complete | 8aedb292b7 + receipt commit | artifacts/epic_4/E4.MC_cycle_receipt.md |
| E5 | E5.1 | complete | 5b24205733 + receipt commit | artifacts/epic_5/E5.1_cycle_receipt.md |
| E5 | E5.2 | complete | 0d1d7aa995 + receipt commit | artifacts/epic_5/E5.2_cycle_receipt.md |
| E5 | E5.3 | complete | ed89f74e53 + receipt commit | artifacts/epic_5/E5.3_cycle_receipt.md |
| E5 | E5.4 | complete | 3fe45c6f1c + receipt commit | artifacts/epic_5/E5.4_cycle_receipt.md |
| E5 | E5.MC | complete | 2521d50e0a + receipt commit | artifacts/epic_5/E5.MC_cycle_receipt.md |
| E6 | E6.1 | complete | 32ae8c1111 + receipt commit | artifacts/epic_6/E6.1_cycle_receipt.md |
| E6 | E6.2 | complete | 0c4d7ce308 + receipt commit | artifacts/epic_6/E6.2_cycle_receipt.md |

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

### Epic report — E3 (Market), merge check on `origin/tranche-19` @ `363b09f0e2`

**Cards done:** E3.1, E3.2, E3.3, E3.4, E3.5, E3.6, E3.MC. No card discovered: what was found is fixed in 6adc8d39c3.

| Figure | Value | Command |
|---|---|---|
| Root tests | 6,729 passed, 0 failed, 300 result lines, exit 0 | `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8`; `awk` and Python agree |
| `codex-ingest` tests | 1,842 passed, **1 failed**, 177 result lines; after the fix that suite 21 passed. Crate not re-run whole | same, with `-p codex-ingest`; then `--test rules_core_pilot_compute_corpus_via_converter` |
| Desktop tests | 997 passed, 0 failed, 2 result lines (991 before this card's 6) | same, in `apps/desktop/src-tauri` |
| Named filters | `market_category_filter` 6, `enhancement_price_table` 4, `market_buy_sell` 18, `market_held` 2, `enhancement_rows` 10, `market_attach_price` 20, `market_shell` 18; 0 `FAILED` | `awk` over the desktop log |
| Frontend | typecheck exit 0; 182/182 test files; inventory 152 rows | `cd apps/desktop && npm run typecheck && npm test && npm run ui-smoke:doc` |
| ui-smoke | 20 of 20 `sheet-market-*`; 14 of 14 tab and changed rows | `npm run ui-smoke -- --only <ids>`, `E3.MC/smoke-*-results.json` |
| Sheet greps | `Add Weapon\|Add Armor\|Add Gear` 0, 0, 0; `MarketDialog` 3, 3, 3 | `grep -c` on the 3 sheets |
| Rows bought and sold through the save path | PF 5,880 + 259 at top bonus, SF 2,308, CPR 250: 0 refused | `E3.MC/probes-*.rs.txt`, temporary tests, not committed as tests; CPR's is `market_shell_every_row_bought_is_on_the_sell_list` |
| Armor cross-type | 0 of 63 | the test, and Python over the dump |
| Mutations | 7 of 8 red; the survivor is equivalent | `E3.MC/mutations.txt` |
| Unpriced rows, refused in paid modes | 1,448 of 5,880 | Python over the dump |

**Seed deltas (base = `f80c0fcfda`, built in its own tree; head = the fixed tree):** PF Aldric, Elowen, Tomas: `sha256.txt` identical. SF x4 and CPR x5: `diff -r` empty, files on both sides.

**Refuted, all fixed here:** a `codex-ingest` test red on the branch since E3.4, in a crate its receipt said it had not run; Export then Import dropped every paid `+N`; the seeded `item:longsword` and `item:chain_shirt` took a free `+5` past E3.6's charge; 14 of 253 Cyberpunk rows could not be sold, 3 of them decks §25 said were not listed. **Holds:** the Armor list, every row bought, no mint in 24 walks, R-11 on every tab, the twins.

**Retro (`scripts/retro.py summary --since 2026-11-02T06:10:42-05:00 --json`, `E3.MC/retro-summary.json`):** 21 corrections, 2 incidents (`fresh-tree-empty-corpus-bundle`, `spec-json-append-conflict`, one each, 0 recurring), 0 deferrals, 1 rework. Subject `decisions.md` fired twice, a file and not a defect, so no new check under R-W. Written before this card's derived verify line.

**Open blockers:** none on E3. **Open, for the lead, no operator ruling needed:** E1.6 is still `ready` and nothing dispatches it (E2's report). The mode switch (Cashless buy, Standard sale pays out) stands on all three systems; no card rules on it.

### Epic report — E4 (body slots and worn items), merge check on `origin/tranche-19` @ `d13e602b7a`

**Cards done:** E4.1, E4.2, E4.3, E4.4, E4.5, E4.6, E4.7, E4.MC. E4.5, E4.6 and E4.7 were discovered in the epic. E4.MC discovered no card: what it found is fixed in 8aedb292b7.

| Figure | Value | Command |
|---|---|---|
| Root tests | 8,602 passed, 0 failed, 70 ignored, 479 result lines, exit 0 | `cargo test --locked -j 8 --no-fail-fast --workspace -- --test-threads=8`; `awk` and Python agree |
| Desktop tests | 1,059 passed, 0 failed, 2 ignored, 2 result lines, exit 0 | same, in `apps/desktop/src-tauri` |
| Named filters, desktop + root | `slot_conflict` 14 + 2, `worn_vs_carried` 8 + 5, `equipment_panel` 4 + 1, `custom_item_bonuses` 11 + 0, `worn_item_applies` 24 + 8; 0 `FAILED` | `awk` over the two logs |
| The worn tests off a staged packaged layout | 24 passed, 0 failed | `CODEX_DESKTOP_RESOURCE_DIR=<stage> cargo test --locked -j 8 --bins worn_item_applies`, `E4.MC/packaged-worn.txt` |
| Frontend | typecheck exit 0; 183/183 test files; `ui-smoke:doc` no file changed | `cd apps/desktop && npm run typecheck && npm test && npm run ui-smoke:doc` |
| ui-smoke | 26 of 26: `sheet-market-*`, `sheet-equipment-*`, `sheet-custom-item-*` | `npm run ui-smoke -- --only <ids>`, `E4.MC/smoke-results.json` |
| Settled worn-item rows | 488 on 9,040 records | Python over `data/corpus/*/_settled/equipment.json`; `gen_settled_corpus --check` drifted=0 |
| Mutations | 6 of 6 red | `E4.MC/mutations.txt` |

**Seed deltas (base = `91537a6951`, built in its own tree; head = `8aedb292b7`):** PF Aldric, Elowen, Tomas: `sha256.txt` identical. SF x4 and CPR x5: `diff -r` empty, 13 files on each side (`E4.MC/seed-deltas.txt`).

**Refuted, all fixed here:** a worn deflection or insight bonus left CMD alone (CRB p.199); a worn Headband of Vast Intelligence +2 raised Elowen's skill budget 30 to 35 and her spellbook 15 to 16, where the book grants ranks in named skills; a clone and an export lost `custom.json`, worn custom items with it; a worn Muleback Cords or Heavyload Belt left the load limits alone, because `unsettled-chains.py` counts only the heads it names. **Holds:** the slot check in both directions with custom items, Recompute against the load, saved states through an export, the bundle's fields, every mutation.

**Retro (`scripts/retro.py summary --since 2026-11-02T22:12:59-05:00 --json`, `E4.MC/retro-summary.json`):** 30 corrections (E4 25, E7.1 5, the window it overlapped), 3 incidents (`ui-smoke-marker-beats-reload` 2, `ui-smoke-driver-orphaned-app` 1), 0 deferrals, 3 reworks. No key fired more than twice, so no new check under R-W. `ui-smoke-marker-beats-reload` is one more firing from needing one. Written before this card's derived verify line.

**Open blockers:** none on E4. **Open, for the operator (no card stops on it):** the required Longsword. It fills the main hand and the engine refuses stowing the last one, so a character holding it can never take a bought Greatsword in hand: it stays carried, with the reason (E4.1's receipt, `decisions.md §25`). R18's check is right; whether a character may give up the Longsword is a ruling. **Open, for the lead, no operator ruling needed:** E1.6 is still `ready` and nothing dispatches it (E2's and E3's reports).

### Epic report — E5 (level ledger), merge check on `origin/tranche-19` @ `d119283068`

**Cards done:** E5.1, E5.2, E5.3, E5.4, E5.MC. E5.4 was discovered in the epic. E5.MC discovered no card: what it found is fixed in 2521d50e0a.

| Figure | Value | Command |
|---|---|---|
| G-8, desktop | `ledger_stamp` 8, `ledger_migration` 12, `progression_ledger` 12; 0 `FAILED` | `for t in ledger_stamp ledger_migration progression_ledger; do cargo test --locked -j 8 $t -- --test-threads=8; done` in `apps/desktop/src-tauri`, `E5.MC/g8.txt` |
| G-8, root | `ledger_migration` 2 passed, 0 failed | `cargo test --locked -j 8 --lib ledger_migration -- --test-threads=8` |
| Desktop tests | 1,091 passed, 0 failed, 2 ignored, 2 result lines, exit 0 | `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` in `apps/desktop/src-tauri`; `awk` and Python agree |
| Root tests | Not re-run; E5.2's 8,604 passed, 0 failed, 70 ignored stands | `git diff --stat 0d1d7aa995 HEAD -- src crates data tests scripts` prints nothing |
| Frontend | typecheck exit 0; 184/184 test files; `ui-smoke:doc` no file changed, 159 rows | `cd apps/desktop && npm run typecheck && npm test && npm run ui-smoke:doc` |
| ui-smoke | 4 of 4: `sheet-progression-ledger-aldric`, `-drawer-open`, `-close-x`, `-close-escape` | `node scripts/ui-smoke/run.mjs --only <ids>`, `E5.MC/smoke-results.json` |
| Staged packaged layout | Rows and listing identical to the checkout, 1,972 lines, four characters | `set_data_root` in the test itself, `E5.MC/packaged-ledger.txt` |
| Roster | 137 classes x 4 levels, 30 unknown feats added: rows with no level equal the budget's `over_by` on all 548; no panic, no error | `E5.MC/roster.txt`, `probes.rs.diff.txt` |
| Mutations | 15 of 15 red | `E5.MC/mutations.txt` |
| E5.4's test | 40 of 40 by `--exact` | `E5.MC/e54-forty-runs.txt` |

**Seed deltas (base = `460561601f`, built in its own tree; head = `2521d50e0a`):** PF Aldric, Elowen, Tomas: `diff -r` empty, `sha256.txt` identical. SF x4 and CPR x5: `diff -r` empty. 13 non-empty files (`E5.MC/seed-deltas.txt`). `sheet_rule_convert --check` exit 0 on both systems.

**Refuted, both fixed here:** a clone or an export lost `hit_points.json`, so the drawer's hit die went blank and the clone's hit points changed; a `ledger.json` cut short mid-write refused every later change on that character. **Corrected, nothing lost:** E5.1's receipt says `re_save` reaches the one mutation path; it saves on its own and changes nothing the ledger records. **Holds:** the stamp through every writer of a Pathfinder character (`grep -rn 'SavedCharacterStore::save('`), multiclass ranks and spell slots on five mixes, malformed saves (no class, level 0, an unknown class) with no panic, the Restore path (it writes only where the directory is gone). **E5.2's 4 failures in 27:** the dice are not biased: 5,386 of 200,000 sequential seeds roll 21 or more (`E5.MC/rollsim.py.txt`, which reproduces seed 38's 21), and the shipped path rolled 12 of 300 from the clock, one process a run (`E5.MC/fresh-rolls.txt`). Against 35 of 1296, 4 or more in 27 has a chance of 0.0057, so that run reads as a tail and E5.4's mechanism stands.

**Retro (`scripts/retro.py summary --since 2026-11-03T23:02:19-05:00 --json`, `E5.MC/retro-summary.json`):** 12 corrections (E5.1 3, E5.2 3, E5.3 3, E5.4 1, this card 2), 1 incident (`clock-seeded-test`), 0 deferrals, 0 reworks. No key fired more than once, so no new check under R-W. Package documents were wrong in 7 of the 12 subjects, as the corrections name them.

**Open blockers:** none on E5. **Open, for the lead, no operator ruling needed and no DoD row I can name, so no card:** `load_saved_character` is a sync command and a first load computes once a level on it, where `list_progression_ledger` was made async for the same cost; Level 20 read 8.4 s mean, 15.3 s max in a debug build with other work running, and no release figure was taken. `bio.json` is not carried by a clone or an export (`clone_character`, `ImportedCharacterFileDto`), and print's Bio mode (R17, E8) reads it. Only creation saves a hit die (`save_character_hit_points_at_root` has no level-up caller), so levels gained later show none. E4.MC's "staged" worn tests did not install the data root (retro correction above) and were not re-run this way. E1.6 is still `ready` and nothing dispatches it (`kanban.md`).

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
| E3.6 | A `+N` at or below the one a copy holds | Costs nothing, returns nothing; the copy stays what it was | A refusal, which the row does not ask for |
| E3.6 | A flat-priced modifier under Cashless | No money moves | The old charge in every mode, from before the request had a mode |
| E3.MC | Attach Modifier on a seeded `item:*` id | Charged by the row its record is on | Free, as E3.6's row said, which left every starting sword a free `+5` |
| E3.MC | Cyberdecks the shop files under `gear` | Off the Market | Buying one onto the sheet, a deck the row rules out |
| E4.MC | A worn Intelligence bonus and the skill budget | The saved score budgets skill points, languages and a spellbook; DCs and spells per day move | The raised score, which pays points per level that stowing the item takes back |
| E4.MC | An Armor Class penalty on an item that also states CMD | Counted once | Twice, -4 on a figurine that says -2 |
| E4.MC | Heavyload Belt's `LOADMULT\|TYPE=SIZE\|3` | Added to the size's multiplier, as the pinned oracle does | Printing only its name, which leaves a stated number unread |
| E5.1 | What an ability increase is, with no such selection on the Pathfinder sheet | A positive `ability:<name>` grant, one row a point | The score moving, which reads a lifted curse as an increase |
| E5.1 | Where the stamp lives | The one mutation path, by what is held before and after | A call at each command, a list a new command can miss |
| E5.1 | Rows held before a character's first ledger | Kept with no level and marked, E5.2's seam | Only the new row, which would stop E5.2 ever filling the rest |
| E5.2 | A load that cannot build the ledger | Shows the sheet, writes nothing, says so on stderr, tries again next load | Failing a load for a file no sheet number reads |
| E5.2 | Feats when first-come strands one | Earliest free slot, a feat moves only so a later one fits | First-come, whose rows would disagree with the budget's `over_by` |
| E5.3 | Where a level's class features come from | The sheet's own `class_feature` lines at that level | The engine's explanations, most of which only a checkout holds |
| E5.3 | An ability increase with no row | Says `none recorded`, and is not counted unspent | Counting it, when the sheet offers no pick to spend it on |
| E5.3 | Which levels show an open slot | The latest, with totals from the budget, so they sum to the sheet's badge | Rows per level per pool, which a late choice makes read as two wrong levels |
| E5.MC | A saved hit die on a clone and an export | Carried, as `custom.json` and `ledger.json` are | Left blank, which is honest in the drawer and changes the sheet's hit points |
| E5.MC | A first load's cost on the main thread | Left as it is, named for the lead | `load_saved_character` made async, which moves every system's load on a figure not taken in release |

## Open blockers

(none)

## Run handle

| Run id | START | Script path | Started |
|---|---|---|---|
| wf_e22fdf36-bc3 | C1 | session `workflows/scripts/sd39-ui-print-corrections-wf_e22fdf36-bc3.js` | 2026-11-01; stopped at C1 on a too-wide acceptance grep; the lead cleared it |
