# SD-36 Epic F4d: ui-smoke rows for the widened Create picker and prestige level-up (receipt)

Scope: `epic-f-class-completion.md` §6, acceptance F4.4 in `epic-breakdown.md`. Branch
`sd36/epic-f4-f5`, worktree `/home/ubuntu/workspace/worktrees/codex-epic-f4`, on top of F4c
(`9e533fd37b`). No `data/**` change. No Rust change.

Logs in this directory: `f4d-red.log`, `f4d-green.log`, `f4d-verify.log`. UI evidence:
`docs/release/SD-36-consolidation/artifacts/ui-smoke/f4/`, with the final run's `results.json`,
`run.log` and one `.png` per row. `regression/` holds the regression pass. `run1/` to `run3/` hold
the earlier red runs: their `results.json`, `run.log` and `*.FAILED.png`. Their green screenshots
were dropped because the final run supersedes them, so the `screenshot` paths in those older
`results.json` files point at files that no longer exist.

## 1. Result

All rows ran on **one app launch**: debug `tauri dev` with the DEV probe on,
`RUN_DESKTOP_AGENT=sd36-f4d`, `CARGO_TARGET_DIR=/home/ubuntu/workspace/worktrees/codex-epic-f4-target`.
Run 1 launched it. Runs 2 to 4 and the regression pass report `Reusing live app`. The regression
pass ran without `--keep`, so it stopped the app. No app process was left running afterwards.

Command (from `apps/desktop`):
`RUN_DESKTOP_AGENT=sd36-f4d node scripts/ui-smoke/run.mjs [--keep] --only <ids> --out <dir>`

**New rows: 7 of 7 green** (the 6 F4.4 rows + 1 accept row, §3). Source: `ui-smoke/f4/results.json`, summary line `7/7 green, 0 red, 0 blocked, 0 manual, 0 not-run (M = 7 rows)`.

| row | status | what the snapshot shows |
|---|---|---|
| `create-character-samurai` | green | sheet `F4 Smoke Alpha — Samurai 1`, CLASS `Samurai 1`, HP 12 / 12 |
| `create-character-magus` | green | sheet `F4 Smoke Bravo — Magus 1`, HP 10 / 10 |
| `create-character-warrior` | green | sheet `F4 Smoke Charlie — Warrior 1`, HP 12 / 12 |
| `create-character-kineticist` | green | sheet `F4 Smoke Delta — Kineticist 1` |
| `create-character-inquisitor-generic` | green | sheet `F4 Smoke Echo — Inquisitor 1` (`class:inquisitor`, not the census-only `ex_inquisitor`) |
| `level-up-fighter6-into-arcane-archer` | green | Level up on Human Fighter 6 → Arcane Archer. The row prints `Entry requirements — not all met (you may still take it)`, the 4 requirement lines (3 unmet, `base attack bonus at least 6 — met`), `Arcane Archer 1 (new class) — character level 7` and `Hit die: d10 · Skill points: 5`. Accept stays enabled (§9.2). |
| `level-up-fighter6-into-arcane-archer-accept` | green | Accept → level-7 feat picker → CRB Alertness → sheet `Fighter 6 / Arcane Archer 1`, level 7, BAB +7, HP 60 / 60, Feats tab lists `Alertness` with its CRB text |

**Regression: 4 of 4 green.** The rows are `create-character-render`,
`create-character-fill-and-submit`, `create-character-back` and `sheet-action-level-up-dialog`.
Source: `ui-smoke/f4/regression/results.json`.

Each create row:

- picks Race = Human (CRB) and the class **by option value** (the class id), so no option label or
  support suffix is guessed;
- submits, backs out, and loads the character by a unique name (Load Character lists newest first);
- asserts the **sheet** prints the held-class line.

A blocked build never reaches the Load list, so it could not pass here.

## 2. RED → GREEN, run by run

| run | new-row result | red rows and root cause | fix |
|---|---|---|---|
| 1 | 4 / 7 | **kineticist**: `target not found: 'Load'`. **level-up**: `expected string missing: 'Add a prestige class (74)'`. **accept**: `forbidden string present: 'failure'` + `'Loading' still present` | see below |
| 2 | 5 / 7 | kineticist (same); accept: CRB Alertness text missing | spec fix for level-up took; see below |
| 3 | 6 / 7 | accept: CRB Alertness text missing at snapshot time, present on screen after | harness + frontend fixes took; see below |
| 4 (final) | **7 / 7** | none | — |

Root causes, each with its fix:

1. **Harness defect: an unacknowledged command was sent a second time (kineticist).** Loading the
   Kineticist sheet holds the webview for more than 2 s. Measured by hand on the same launch: 4 s
   after `click Load` the Load screen was still up, and the sheet was up by 15 s. The runner's retry
   loop treated the missing 2 s acknowledgement as a failure and **re-sent** `click Load`. The first
   click had already run, so every re-send answered `no element named 'Load'` for the whole 150 s
   budget, while the sheet sat on screen. That is reproduced: a solo re-run left the probe showing
   `F4 Smoke Delta — Kineticist 1`, with `lastCommand` `no element named 'Load'`.
   - **Fix:** new `sendUntilOk` in `lib/commandChannel.mjs`. It re-sends only when the webview
     *answered* not-found. When no answer arrives, it keeps waiting for that same command id until
     the deadline.
   - `run.mjs`'s click, type, select and key all use it.
   - Test: `lib/commandChannel.test.mjs`, with a fake frontend in a child process. RED with the old
     loop shape: `the late acknowledgement is the result`. GREEN after the fix: 3 of 3 cases. It
     checks that a late ack means one send, that not-found is re-sent until it appears, and that
     the deadline bounds the call.
2. **App defect: the Feats tab printed CRB Alertness as `Alertness (Mythic)` (accept row).**
   - The catalog serves 145 Mythic Adventures records whose `key` is their base feat's key. For
     example, `Alertness (Mythic)` has key `Alertness` (`src/rules_core/rules_tables/feat_gap_tables.rs:454`).
   - `featsTabModel.resolveSelectedFeatEntries` kept the **last** record per identity, so a
     character holding `"Alertness"` was shown the Mythic record and its text.
   - The engine's key lookup is first-match over the same catalog order. `feat_gap_rows_for` says gap
     rows are chained after the hand-authored tables "so a first-match key lookup keeps resolving to
     the hand-authored record". `spellsTabModel` already joins first-wins.
   - **Fix:** the feat join is first-wins too, one rule with no per-book case.
   - Test: `featsTabModel.test.ts`, `verifiesASharedKeyResolvesToTheFirstCatalogRecordLikeTheEngine`.
     RED: `expected Alertness, got Alertness (Mythic)`. GREEN after the fix.
   - Run 1 also picked the Mythic row itself, because a bare `Alertness` target prefix-matches both
     picker rows. That run ended with an `Add Weapon` picker open, whose armour text holds
     "arcane spell **failure**" and "**Loading** a crossbow", and those tripped the global forbid.
     The row now targets `AlertnessCRB` (a picker row's name is the feat name followed by its book,
     with no separator).
3. **Spec defects (fixed in `spec.json`, no app change):**
   - The optgroup label `Add a prestige class (74)` is not in `innerText`, so it is no longer
     asserted. The 74-prestige figure is pinned by F4c's wire test (`f4c-level-up-fighter6-wire.json`).
   - The accept row's marker is now the CRB Alertness text, so the row's settle wait covers the
     Feats tab's async catalog load. Run 3's snapshot was read before the tab resolved. The same
     text was on screen when checked right after.
4. **`--only` took one id.** It now takes a comma list, run in the order given. An unknown, empty
   or repeated id is an error, never a shorter run (`lib/rowSelection.mjs` +
   `rowSelection.test.mjs`). RED at HEAD:
   `Error: --only create-character-render,create-character-back: no such row id in spec.json`.

## 3. The seventh row

`level-up-fighter6-into-arcane-archer-accept` is not in F4.4's list of six. F4c carried forward
that the prestige Accept path (`level_up_character` with a prestige class id, then
`addFeatSelection` for the level-7 feat) had never been driven. That path is what exposed defect 2.
It is counted in the headline: 7 of 7.

## 4. Counts (denominator, command)

| figure | value | source |
|---|---|---|
| spec rows | **76** (69 + 7) across 18 screens, 3 manual (unchanged) | `npm run ui-smoke:doc` → `docs/testing/ui-smoke-inventory.md`; a second run left the file byte-identical (md5 `0248ad24…`) |
| new rows green, final run | **7 of 7** | `ui-smoke/f4/results.json` |
| regression rows green | **4 of 4** | `ui-smoke/f4/regression/results.json` |
| Mythic feat records sharing a base key | 145 of 160 Mythic rows carry `name: "… (Mythic)"` | `grep -o 'category: "Mythic", name: "[^"]*(Mythic)"' src/rules_core/rules_tables/feat_gap_tables.rs \| wc -l` (160 = `grep -c 'category: "Mythic"'`) |
| frontend suite | 126 / 126 files passed; typecheck exit 0 | `cd apps/desktop && npm run typecheck && npm test` (`f4d-verify.log`) |
| harness lib tests | rowSelection, commandChannel, resultsSkeleton: 3 / 3 pass | `node scripts/ui-smoke/lib/<name>.test.mjs` (`f4d-verify.log`) |

No Rust file changed, so the root and desktop cargo suites were not re-run. F4c's 621 / 621
desktop run stands.

## 5. Carried forward (named by mechanism, not done here)

- **Mythic feats reuse their base feat's key.** Picking `Alertness (Mythic)` in any feat picker
  records `"Alertness"`, which the engine and now the sheet resolve to CRB Alertness. So a Mythic
  pick is silently the base feat. The level-7 picker offers Mythic rows to a non-mythic character.
  The fix is a distinct key for the 145 records in the generated `feat_gap_tables.rs`
  (`gen_feat_gap_tables`), or keeping Mythic rows out of the character pickers. That is a
  generator step, not this one.
- **Perception on the accept row's sheet reads +1 with Alertness held** (Wis +1, 0 ranks). CRB
  Alertness gives +2 on Perception and Sense Motive. The skills panel does not fold held-feat skill
  bonuses. This was observed on `level-up-fighter6-into-arcane-archer-accept.png` and not traced
  further in this step.
- **The level-up feat picker is titled `Pick a feat — level 1`** for the character-level-7 feat. It
  prints the new class level (`CharacterSheet.tsx:4144`), which fits a class bonus feat but not the
  universal odd-level feat.
- **The app profile keeps every smoke character.** Each run mints 7 more saved characters
  (`F4 Smoke Alpha`…`Golf`), as `create-character-fill-and-submit` already does. Two by-hand probe
  characters (`F4 Probe India`, `F4 Probe Juliet`) were made on this launch while diagnosing.
