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
| Cards complete | 56 of 66 (`awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/ && $5 ~ /^ complete *$/' kanban.md \| awk 'END{print NR}'`) |
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
| E6 | E6.MC | complete | 356654dd52 + receipt commit | artifacts/epic_6/E6.MC_cycle_receipt.md |
| E1 | E1.6 | complete | 2ce249f1fa + receipt commit | artifacts/epic_1/E1.6_cycle_receipt.md |
| E7 | E7.2 | complete | 7bf3f968ac + receipt commit | artifacts/epic_7/E7.2_cycle_receipt.md |
| E7 | E7.3 | complete (closes E7.1) | f6ed94b1e7 + receipt commit | artifacts/epic_7/E7.3_cycle_receipt.md |
| E7 | E7.4 | complete | a4150fc4b1 + receipt commit | artifacts/epic_7/E7.4_cycle_receipt.md |
| E7 | E7.5 | complete | a44e177d18 + receipt commit | artifacts/epic_7/E7.5_cycle_receipt.md |
| E7 | E7.6 | complete | ac9bb659ae + receipt commit | artifacts/epic_7/E7.6_cycle_receipt.md |
| E7 | E7.MC | complete | c8ba870af8, e65cef12fd + receipt commit | artifacts/epic_7/E7.MC_cycle_receipt.md |
| E8 | E8.1 | complete | f8c29c3eeb, 784ae6fde5, dd4006a843 + receipt commit (row added by E8.2) | artifacts/epic_8/E8.1_cycle_receipt.md |
| E8 | E8.2 | complete | ed2000b229 + receipt commit | artifacts/epic_8/E8.2_cycle_receipt.md |
| E8 | E8.3 | complete | 3fc88b76a2 + receipt commit | artifacts/epic_8/E8.3_cycle_receipt.md |
| E8 | E8.4 | complete | c226f02337 + receipt commit | artifacts/epic_8/E8.4_cycle_receipt.md |
| E8 | E8.5 | complete | 043be0aa10 + receipt commit | artifacts/epic_8/E8.5_cycle_receipt.md |
| E8 | E8.6 | complete | 76dd143469, 53303ba64e + receipt commit | artifacts/epic_8/E8.6_cycle_receipt.md |
| E8 | E8.7 | complete | 49a156e423, bac3dd4ab9 + receipt commit | artifacts/epic_8/E8.7_cycle_receipt.md |
| E8 | E8.MC | complete | 9cd987c1dd + receipt commit | artifacts/epic_8/E8.MC_cycle_receipt.md |
| E8 | E8.8 | complete | 6994fad450 + receipt commit | artifacts/epic_8/E8.8_cycle_receipt.md |
| E7 | E7.7 | complete | 5dfc4c48ff + receipt commit | artifacts/epic_7/E7.7_cycle_receipt.md |
| E9 | E9.1 | complete | 250f547e96 + receipt commit | artifacts/epic_9/E9.1_cycle_receipt.md |

## Epic reports

(The merge-check card of each epic writes a one-screen block here: cards done, figures with commands,
seed deltas, open blockers.)

### Epic report — E1 (guards), merge check on `origin/tranche-19` @ `d3d215d4d9`

**Cards done:** E1.0, E1.1, E1.2, E1.3, E1.4, E1.5, E1.MC. **Not closed:** E1.6, discovered here, `ready`, row-order next. The epic's guards are not finished until it lands. *(E1.6 landed 2026-11-04: Swashbuckler and Gunslinger 1 at 4, 5 at 20; Hunter 6 at 18. Dragon Disciple and Sentinel still read no pool, named in its receipt.)*

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

### Epic report — E6 (overrides), merge check on `origin/tranche-19` @ `f91eebe370`

**Cards done:** E6.1, E6.2, E6.MC. E6.MC discovered no card: what it found is fixed in 356654dd52.

| Figure | Value | Command |
|---|---|---|
| G-7, desktop | `overrides_` 16 of 16; `admitted_tag_` 13; `::override_` 19; 0 `FAILED` | `cargo test --locked -j 8 overrides_ -- --test-threads=8` in `apps/desktop/src-tauri`, counted by `awk` and Python on the sweep log |
| Desktop tests | 1,139 passed, 0 failed, 2 ignored, 2 result lines, exit 0 | `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` in `apps/desktop/src-tauri`; `awk` and Python agree |
| Root tests | 6,751 passed, 0 failed, 27 ignored, 301 result lines, exit 0 | the same at the root; `awk` and Python agree |
| Frontend | typecheck exit 0; 185/185 test files; `ui-smoke:doc` 164 rows | `cd apps/desktop && npm run typecheck && npm test && npm run ui-smoke:doc` |
| ui-smoke | 46 of 48 chosen rows green across a run and a re-run of 4; 2 red and stale (FSR-C2). E6.2's 3 rows and the 2 new ones green | `node scripts/ui-smoke/run.mjs --only <ids>`, `E6.MC/smoke-results.json`, `E6.MC/smoke2-results.json`. The full suite was not run |
| CRB Market weapons a Wizard is refused | 1 of 164 before, 122 of 164 after; Improvised rows left out, which refused everyone before and no one after | a walk of `list_market`, `E6.MC/market-census.txt` |
| Mutations | 21 of 21 red; the frontend affordance red before its fix | `E6.MC/mutations.txt` |

**Seed deltas (base = `c94921873f`, built in its own tree; head = `356654dd52`):** PF Aldric, Elowen, Tomas: `diff -r` empty, `sha256.txt` identical. SF x4 and CPR x5: `diff -r` empty. 13 non-empty files (`E6.MC/seed-deltas.txt`). `crates` and `data` unchanged, so no `sheet_rule_convert` run.

**Refuted, all fixed here:** the Market holds a weapon as `Longsword (Base)`, which no weapon lookup read. So the weapon flag had one real weapon to refuse (`Armor Spikes`), and, since E3, a worn Market weapon had no attack row and owed no -4. A race's own weapons and a deity's weapon were refused, so were Improvised rows for everyone and `Mind Shield` for a Fighter. Add Spell stayed withheld from a non-caster with the flag on, so that branch of the backend had no way in. A worn custom item moved onto a taken slot under the flag went untagged. With the flag off, what it let on refused every later change to any other worn custom item. **E6.1's 16 tests passed through all of this**: they ran on the literal `Rapier`, a key no screen sends. Each now has a test on the shipped key, and a mechanism at the source (`normalize_weapon_identity`), not a caution. **Holds:** the flags and tags through a file round trip, feats past budget and prerequisite through later changes, spells under the flag across 9 casting and non-casting classes, alignment on 20 classes, a tag going with its copy; E6.1's 35 of 189 and 37 of 2,774, E6.2's 102.

**Retro (`scripts/retro.py summary --since 2026-11-04T06:50:46-05:00 --json`, `E6.MC/retro-summary.json`):** 8 corrections (E6.1 1, E6.2 1, this card 6), 1 incident (`test-key-not-the-shipped-key`, silent), 0 deferrals, 0 reworks. No key fired more than once, so no new check under R-W; the incident has its mechanism at the source. 6 of the 8 subjects are receipts or package rows, as the corrections name them.

**Open blockers:** none on E6. **Open, for the lead, no operator ruling needed and no DoD row I can name, so no card:** an Elf Wizard's Longsword, or a Cleric's own weapon, is no longer refused and still reads -4 on the attack row, with words saying nothing grants it. `character_weapon_proficiency` runs inside the compute and reads classes and three feats; the race resolver it would need is in the desktop crate. The wrong number predates this bundle. The two stale ui-smoke rows are FSR-C2's. `append_to_character` wears with no check and no screen calls it. E1.6 is still `ready` and nothing dispatches it (`kanban.md`).

### Epic report — E7 (`.pcg` import, gear, proficiency), merge check on `origin/tranche-19` @ `6f5bdef517`, fixes in `c8ba870af8` and `e65cef12fd`

**Cards done:** E7.1, E7.2, E7.3, E7.4, E7.5, E7.6, E7.MC. **Discovered: E7.7** (a weapon stating no damage dice, `Net (Base)`, has no attack row), `ready`, 63 cards.

| Figure | Value | Command |
|---|---|---|
| E7 commands | `pcg_import_fixtures` 12 of 12; root `selected_skill` 28, `race_weapon_proficiency` 8; desktop `no_required_gear` 11, `left_behind` 11, `race_weapon_proficiency` 7, `character_money_` 8. 0 failed. `bash scripts/verify.sh --only pcgen-residue-gate --only crate-wall` `RESULT: PASS`; in `apps/desktop`, the sidecar built as the Linux publish job builds it, then `npx tauri build --config src-tauri/tauri.sidecar.json --bundles deb` exit 0, the deb holding `usr/bin/pcg_import`, `usr/bin/codex-desktop` and `usr/lib/Codex/data/` with `cyberpunk-red` and `starfinder-1e` (`dpkg-deb -c`). | each row's own command, `awk` over the log |
| Root tests | 6,771 passed, 0 failed, 27 ignored, 301 result lines, exit 0, at c8ba870af8 | `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8`; `awk` and Python agree |
| Desktop tests | 1,180 passed, 0 failed, 2 ignored, 2 result lines, exit 0, at e65cef12fd's tree (E7.6's 1,172 plus 5 and 3) | the same in `apps/desktop/src-tauri` |
| `codex-ingest` | 1,857 passed, 1 failed, 43 ignored, 178 result lines, exit 101, at c8ba870af8. The one, `v06_wizard_pilot_case_verification::full_pipeline_runs_end_to_end_for_the_wizard_pilot_case`, failed on PCGen's gradle download during the internet outage (`Unable to process url: https://api.adoptium.net/...`); after it, `cargo test --locked -p codex-ingest -j 8 --test v06_wizard_pilot_case_verification` 3 passed, 0 failed | `cargo test --locked -p codex-ingest -j 8 --no-fail-fast -- --test-threads=8` |
| Frontend | typecheck 0; 187/187 test files; `ui-smoke:doc` 168 rows | `cd apps/desktop && npm run typecheck && npm test && npm run ui-smoke:doc` |
| ui-smoke | 40 of 40 chosen rows green at their last run: the 37 that touch Gear, Market, Custom and overrides 35/37, the 2 red the rows the harness stalled on before each auto-relaunch, both green re-run alone (`smoke-rerun-results.json`); `load-character-import-pcgen` and `-opens` (`smoke-import-results.json`); `sheet-bio-deity-moves-melee` (`smoke-deity-results.json`), red three times while its steps were wrong | `node scripts/ui-smoke/run.mjs --only <ids>`, `E7.MC/smoke-results.json`. The full suite was not run |
| `.pcg` fixtures through the real sidecar and import path | 37 saved, 26 refused by the engine, 2 refused by the reader, 0 that saved and do not load, of 65 | `mcp_p5_every_fixture`, `E7.MC/probes-before.txt` |
| Packaged tree | 63 of 63 converted files equal the checkout's, under a tree shaped as `tauri.conf.json` maps the resources; desktop `race_weapon_proficiency` 7 of 7 passed. The tree's first cut held no `data/cyberpunk-red` or `data/starfinder-1e`, and 4 failed on `cpr_creation.read_failed`, a fault of the tree, not the app; rebuilt with every `bundle.resources` mapping of `tauri.conf.json` under it | `pcg_import --data-root`, `E7.MC/packaged-tree.txt`; `CODEX_REPO_ROOT=<tree> cargo test --bins race_weapon_proficiency` |
| No gear | 63 of 63 base classes compute with everything stowed, left behind, none held, and a Greatsword alone; 74 prestige classes are not first classes | `mcp_p6_every_class_no_gear`, `E7.MC/probes-before.txt` |
| Deity weapons | 442 of 464 declarations on the CRB table not refused for a Cleric of that deity, 0 refused; the other 22 name a weapon off the table (Net, Bite, War Razor and 4 more) | `mcq_p1_deity_census`, `E7.MC/probes-before.txt` |
| Row against refusal | 71 of 71 attack-bearing `(Base)` Market rows agree, of 77, on 9 characters: Wizard of 7 core races, Cleric of Iomedae, Cleric of none. The 6 are 5 ammunition rows and Net | `mcq_p2`, 4 before the fixes and 5 after, `E7.MC/probes-before.txt`, `market-census-after.txt` |
| Mutations | 7 of 7 red, 1 first run on the wrong filter and re-aimed | `E7.MC/mutations.txt` |
| `sheet_rule_convert --check` | exit 0, Pathfinder and Starfinder | G-1 |

**Seed deltas (base = `2a726046d6`, E6.MC's close, built in its own tree; head = `c8ba870af8`):** PF Aldric, Elowen, Tomas: 1 path each, `/explanations/17/detail` of `combat.weapon_attack_bonus.longsword`, text, the stale "task #80" clause. No number moves. SF x4 and CPR x5: `diff -r` empty (`E7.MC/seed-deltas.txt`). So every E7 card's "none" holds at the epic's width, this card's one text path apart.

**Refuted, all fixed here:** an attack row read Strength for every bow, crossbow and javelin. A shield stand-in worn first was the Melee number (`pf_Cleric.pcg` read -2 with a Morningstar row of 2). A Fighter's shield bash read -4 and "nothing grants" while the refusal let him wear it. A clone lost its deity, and with it the favored weapon. A clone and an Export then Import lost the character's money (36,345 cp read 0). `DEITY:None` became a deity named None. **E7.4's 9 and E7.6's 5 tests passed through all of this**: they asserted Melee and melee weapons, and neither reached clone. **Holds:** E7.1's 37/26/2; the sidecar's output under a packaged tree; 63 of 63 classes with no gear; left behind (a mutation of each reader goes red); the deity row writes nothing to the ledger; deity weapons by proficiency name, `Sword (Short)` and `Mace (Heavy)` included; the holders Cleric, Inquisitor and Warpriest by deity.

**Retro (`scripts/retro.py summary --since 2026-11-03T00:00:00 --actor sd39-e7.1 ... --actor sd39-e7.mc --json`, `E7.MC/retro-summary.json`):** 24 corrections (E7.1 5, E7.2 2, E7.3 3, E7.4 3, E7.5 1, E7.6 2, this card 8, one of them correcting another of mine), 2 incidents (`wrote-in-lead-checkout`, `network-outage-mid-run`, once each), 0 deferrals, 0 reworks. No key fired more than once, so no new check under R-W. 23 of the 24 subjects are receipts, package rows or my own retro line

**Open blockers:** none on E7. The two ui-smoke rows that went red in the long run (`sheet-market-open-weapons`, `sheet-equipment-lists`) were the rows the harness stalled on before each auto-relaunch, and are green re-run alone. E8.1 depends on E7.MC, not E7.7, so the script's `drainBoard` must run E7.7 before E8.

**Sweep (`workflow-instruction.md §10` step 3):** `sd39-*` worktrees other than `sd39-main`: 1 found (`sd39-e7mc`, detached at c8ba870af8, merged, unlocked, its one edit the same blob as e65cef12fd's `spec.json`), 1 removed; `cargo-target/sd39-*` other than `sd39-main`: 1 found (40 G), 1 removed. `git worktree list | grep -c sd39` → 1; `ls /home/ubuntu/cargo-target | grep -c sd39` → 1.


### Epic report — E8 (print), merge check on `origin/tranche-19` @ `bc1d909905`, fixes in `9cd987c1dd`

**Cards done:** E8.1, E8.2, E8.3, E8.4, E8.5, E8.6, E8.7, E8.MC. **Discovered: E8.8** (Class Features figures and Damage Reduction on the basic PF sheet), `ready`; E9.1 now depends on it. 66 cards.

| Figure | Value | Command |
|---|---|---|
| E8 commands | print_smoke 8, print_pf_blocks 11, print_golden 24, print_sf 4, print_cpr 4; 0 failed | `cd apps/desktop/src-tauri && cargo test --locked -j 8 print:: -- --test-threads=8`, per-suite awk over `E8.MC/desktop-sweep.txt` |
| NR-2 | 0 lines | `grep -rn 'fn compute\|calc(' apps/desktop/src-tauri/src/print/ \| awk 'END{print NR}'` (the package's `grep -n` read no file; corrected) |
| G-9 | 0 lines | `grep -rn 'window.print\|printLayout' apps/desktop/src \| awk 'END{print NR}'` |
| Desktop tests | 1,235 passed, 0 failed, 2 ignored, 2 result lines, at 9cd987c1dd | `cd apps/desktop/src-tauri && cargo test --locked -j 8 --no-fail-fast -- --test-threads=8`; awk and Python agree |
| Skill totals on the basic PF sheet | 35 of 35 per seed (Aldric, Elowen, Tomas); were 7, 10, 6 | `E8.MC/probe.rs.txt` → `pf-probe-report.txt`; `print_pf_blocks_every_skills_panel_total_prints_ranked_or_not` |
| Clippy, sd24 | desktop clippy exit 0; `verify.sh --only clippy` PASS; sd24 5 passed | §6 steps 4 and 6 |

**Seed deltas (base = `bc1d909905`, built in its own tree; head = `9cd987c1dd`):** PF ×3, SF ×4, CPR ×5: none. Over the epic, E8.6 added 2 and E8.7 35 additive rows per PF seed (their receipts); no SF or CPR path moved in any E8 card.

**Refuted:** the basic PF sheet printed only ranked skills (28, 25, 29 of 35 totals missing), no money and no Size or Vision, all fixed here; it prints no Class Features figure (Elowen's force missile 7 a day) and no Damage Reduction, card E8.8. The package's NR-2 grep could not fail. **Held:** NR-2 by reading every non-test line of `print/`; SF and CPR every engine row once; golden 24 (6 PF page counts moved by the added rows); E8.6 and E8.7 seeds additive only.

**Retro (`scripts/retro.py summary --since 2026-11-05T15:00:00-05:00 --json`, `E8.MC/retro-summary.json`):** 8 corrections (E8.1 1 caught by E8.1, E8.2 2, E8.3 2, E8.5 1, this card 2), 2 incidents (`sd24` lockfile token, `pkill -f` killing its own shell, once each), 5 reworks, 7 verifications, 0 deferrals. No key fired more than twice, so no new check under R-W. 5 of 8 subjects are this package's own rows or design text.

**Open blockers:** none on E8. Not E8's: **E7.7 is still `ready`** (`awk -F'|' '$2 ~ /E7.7/ {print $5}' kanban.md`), though E7's report said the script must run it before E8; it must land before E9.5.

**Sweep (`workflow-instruction.md §10` step 3):** `sd39-*` worktrees other than `sd39-main`: 1 found (`sd39-e8mc-base`, this card's base render, detached at bc1d909905, clean, unlocked), 1 removed; `cargo-target/sd39-*` other than `sd39-main`: 1 found, 1 removed. `git worktree list | grep -c sd39` → 1; `ls /home/ubuntu/cargo-target | grep -c sd39` → 1.

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
| E6.MC | An item a race or deity may grant | Not refused, since an unknown never refuses | Refused until the engine holds a deity, which refuses a Cleric the book allows |
| E6.MC | An Improvised Weapon row | Never refused: anyone may pick one up, and the sheet states the penalty | Refused for all, a Fighter included |
| E6.MC | A shield or suit stating no kind | An unknown, not refused, for a Wizard too | Refused for all, a Fighter included |
| E7.6 | A Cleric with no deity recorded, or one no rule holds | -4, refused, and the row prints why: the row and the refusal read one answer. Reverses E6.MC above | Unknown, which claim-blocks every Cleric holding the starting Longsword |
| E7.6 | Where the deity's weapon and a race's Martial weapons live | Declared by the converter on the record, one more run, 469 files (§5's revisit, G-2 amended) | A hand-kept table; reading the corpus live, which a packaged app does not ship |
| E7.MC | A table shield's bash | A row, with the shield's own proficiency | No row, which the corpus path gives a stand-in shield and the table path never did; left two-sided and named in the receipt |
| E7.MC | A thrown Dagger | Strength, since the table says it is melee and the sheet records no throw | Two rows per weapon |
| E7.MC | A clone's and an export's money | Carried, as the other sidecar files are | Left for the lead, which this card's first draft did |
| E8.1 | Where the print's fonts and templates live at run time | Compiled into the binary (`include_bytes!`, `include_str!`): the same bytes in a test, a dev run and every installer | A `bundle.resources` lookup, unproven for a packaged build by any E8.1 test |
| E8.1 | Hit Points and Speed on the printed Combat block | Left out, and discovered card E8.6 gives each one backend value | Printing them by re-deriving the screen's TypeScript arithmetic in Rust (NR-2), or printing the backend durability max HP, which reaches max HP by another rule (no saved rolls, no grants, three classes) |
| E8.1 | Which fonts | Liberation Sans 2.1.5, OFL 1.1, the face `technical-design.md §1` names, copied from the box's `fonts-liberation` 1:2.1.5-3 with its licence | DejaVu (Bitstream Vera licence, not OFL); Typst's own embedded fonts (a feature that pulls more than one family) |
| E8.2 | When an override tag prints | On every sheet, on the record it let in | Only in "full" (all three options), which would hide on a basic sheet how a record got there |
| E8.2 | Skill totals the engine does not hold | Ranks printed, the total `—`, discovered card E8.7 gives each one backend value | Re-adding the Skills panel's TypeScript sum in Rust (NR-2) |
| E8.2 | Where the option blocks print | Each on pages of its own, after the sheet | Run on under Actions, where an option with nothing to print would not change the page count |
| E8.2 | Where the campaigns' descriptions come from | The caller sends them (`PrintRequest.campaigns`); the backend cannot read the frontend's `localStorage` | A new backend campaign store, which R17 ("no new field") rules out |
| E8.3 | The SF and CPR print order | Each screen's order: Starfinder's sections as `buildStarfinderSheet` pushes them; Cyberpunk RED's boxes column by column as `cyberpunkSheetLayout.ts` stacks them (left, then right) | A new order made up for paper, which E9.4's eyeball can still ask for |
| E8.3 | Critical Injuries on the CPR print | The injuries the character has, under their table, with the engine's words | The screen's whole checklist with unticked boxes, which holds no value of the character's |
| E8.4 | Where Settings, Print paper lives | A Print tab the Settings dialog draws itself | A row in Appearance (paper is not appearance), or `App.tsx` passing the panel (no card owns it) |
| E8.4 | Campaign notes on a print without the Bio box | None sent | Sending them always, which the backend would not print |
| E8.6 | Where the one value lives | Two `explanations` rows of the load response (`hit_points.max`, `speed.land`), pushed after the worn-item rows; Show the math prints their detail | A new response field, which the print's derivation lines would not see |
| E8.6 | Each level's floor with no saved rolls | At least 1 hp per level (the engine's `hit_points_from_die`; the saved-roll path already did) | The retired `maxHitPoints`' floor on the sum only; differs only at Con modifier ≤ −(die average) |
| E8.6 | The HP tracker's own Max | Left on `load_character_durability`'s rule (the card's "does not cover") and named in the receipt | Pointing the tracker at `hit_points.max`, a change to the tracker's current-HP default no card scoped |
| E8.7 | Where each skill total lives | One `skill.total.<skill>` `explanations` row per Skills panel row (35), the panel's own rule ported to Rust; Show the math prints its detail | The engine's `allocate_skill_ranks` totals, whose narrower class-skill lists would move the screen's values |
| E8.7 | The allocation dialog's total for an unsaved draft | The saved row while a row's ranks are unchanged, `on save` once changed, none on Create | A TypeScript preview sum (NR-2), or a preview command no card scoped |
| E8.8 | Where the shared label rule lives | The panel's `splitId`/`humanise` ported once into the print (`feats.rs`), held to one fixture of every seed's panel rows by a Rust and a TS test | The label on the load response (a field on every class row and the panel rewired), or a hand-kept table (ruled out) |
| E8.8 | How a Class Features row reads on paper | `<class>: <label> <value>`, `Chassis` where no held class owns it (the panel's gutter); the row's detail in Show the math | The detail under every row on the basic sheet |

## Open blockers

(none)

## Run handle

| Run id | START | Script path | Started |
|---|---|---|---|
| wf_e22fdf36-bc3 | C1 | session `workflows/scripts/sd39-ui-print-corrections-wf_e22fdf36-bc3.js` | 2026-11-01; stopped at C1 on a too-wide acceptance grep; the lead cleared it |
| wf_921af6dd-933 | E1.0 | same | 2026-11-01; stopped at E1.MC: the script skipped discovered card E1.5; script fixed (`drainBoard`) |
| wf_62657bd1-ded | E1.MC | same | 2026-11-01; stopped 2026-11-04 at E7.1: a re-dispatch declined, ownedBy:E7.3; the script had no E7.3 and did not reach E1.6; script fixed (`drainEarlier`, E7.3 in the E7 list) |
