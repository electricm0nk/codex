# Brief 1 — cycle receipt (create-character fixes, F1–F6)

Branch `ui-brief1` (worktree `.worktrees/brief1`), base `tranche-17-ui` @ 5f020b221b.
Status: F2–F6 done. F1 did not reproduce (guard added). One open finding: Meticulous's −2 is in the
engine's skill totals but the sheet's skill rows do not read those totals (see Blockers).

## Commits

| sha | subject |
|---|---|
| e6f67b45c7 | fix(brief1-F4): read class_chassis spells-per-day rows |
| 04d87eaa22 | feat(brief1-F3/F5): wizard spellbook size, draft known-spell rows, drawbacks + trait count check |
| 21a41f3bc7 | feat(brief1-F6): one Options/Selected pair per spell level, shared pools |
| 253031fb4e | feat(brief1-F2/F3/F5): known-spell quotas, drawback trait limits, racial trait counts |
| 2d935014b7 | test(brief1-F1): forbid stray 'H2' on the Create screen smoke row |
| ca4b4c3017 | docs(brief1): cycle receipt + retro events (first pass) |
| bacfd9c43a | feat(brief1-F3): drawbacks in the trait picker, Meticulous -2 on untrained skill totals |
| 7a7228e611 | feat(brief1-F2): ManageBox manageLabel; Racial Traits button reads 'Swap racial traits' |

`git log --oneline tranche-17-ui..ui-brief1`

## Scope corrections

- The brief named `src/rules_core/class_sorcerer_wizard.rs` and `class_wizard_prepared_spellbook.rs`;
  they live in `src/rules_core/pilot_compute/`. Lead confirmed those are the granted files.
- Lead granted (2026-10-11, option A) `apps/desktop/src-tauri/src/trait_picker.rs` (+ tests) and
  `src/rules_core/skill_allocation.rs` (+ tests): the trait DTO is built in `trait_picker.rs`, and
  Meticulous's untrained −2 is a skill-total change.
- Lead granted `apps/desktop/src/characterHub/ManageBox.tsx` + `ManageBox.test.ts` for an optional
  `manageLabel` prop only. No ui-smoke row clicks the Create screen's Manage buttons
  (`grep -n '"Manage"' apps/desktop/scripts/ui-smoke/spec.json` → no match), so no row changed.
- `tests/sd13_wizard_level1_prepared_spell_baseline.rs` was edited as the brief's "+ tests".

## Files changed

- Engine: `src/rules_core/pilot_compute/class_wizard_prepared_spellbook.rs` (the brief's
  `src/rules_core/class_wizard_prepared_spellbook.rs` lives under `pilot_compute/`),
  `src/rules_core/trait_effects.rs`, `tests/sd13_wizard_level1_prepared_spell_baseline.rs`
  (guard allowlist widened for the new `spellbook_size` rows; read as the brief's "+ tests").
- Engine (second pass): `src/rules_core/skill_allocation.rs`.
- Desktop Rust: `apps/desktop/src-tauri/src/character_hub.rs`, `apps/desktop/src-tauri/src/trait_picker.rs`.
- Frontend (second pass): `ManageBox.tsx`.
- Frontend: `spellsPerDayModel.ts`, `spellDialogModel.ts`, `transferListModel.ts`,
  `TransferListDialog.tsx`, `manageItems.ts`, `creationRacialTraitsPreview.ts`,
  `CreateCharacterForm.tsx`, `boundary/draftSpellOptions.ts`, `boundary/loadCharacterTraits.ts`.
- `apps/desktop/scripts/ui-smoke/spec.json`, regenerated `docs/testing/ui-smoke-inventory.md`.

## Tests added

- Rust engine: `wizard_spellbook_size_tests` (5), `trait_effects::drawback_tests` (5, incl. a
  corpus-parity test over every `Drawback ~` file and one over Meticulous's BONUS tokens),
  `skill_allocation::meticulous_drawback_tests` (2).
- Rust desktop (second pass): `trait_picker::tests::every_drawback_is_offered_and_flagged_and_no_trait_is`;
  `returns_every_flat_and_choice_skill_trait` now expects 53 + 17 = 70.
- TS (second pass): `ManageBox.test.ts` (`verifiesTheButtonCanBeRelabelled`).
- Rust desktop: `the_draft_spell_options_state_a_sorcerers_spells_known_per_level`,
  `the_draft_spell_options_state_a_wizards_spellbook_size_and_it_grows_with_levels`,
  `create_refuses_more_traits_than_the_rule_allows`.
- TS: `spellsPerDayModel.test.ts` (Sorcerer chassis ids, sheet side), `spellDialogModel.test.ts`
  (rewritten: sorcerer known, wizard pools, cleric read-only, witch gap, reload key),
  `transferListModel.test.ts` (shared limits), `TransferListBody.test.ts` (pair per group; single
  pair kept), `manageItems.test.ts` (drawback limits), `creationRacialTraitsPreview.test.ts` (count).

## Red evidence (TDD)

- TS: each new test was run before its implementation and failed for the intended reason, e.g.
  `npx tsx src/characterHub/spellsPerDayModel.test.ts` → `expected paladin:1:1:total sorcerer:1:4:total, got ` ;
  `transferListModel.test.ts` → `the 2nd-level group has room but the shared pool does not: expected false, got true`;
  `TransferListBody.test.ts` → `one pair per spell level: expected 2, got 0`; the others failed on the missing export.
- Rust desktop red run (`cargo test -p codex-desktop --bin codex-desktop -- the_draft_spell_options create_refuses_more_traits`,
  log `/home/ubuntu/cargo-target/brief1/logs/red-desktop.log`): `1 passed; 3 failed` — sorcerer/wizard
  `known` empty (`left: None right: Some(4)` / `Some(5)`), three traits `Saved` instead of `Blocked`.
- Rust engine: written with the implementation; proved by mutation instead (drop the `2*(level-1)`
  term and the explanation push) → `cargo test --lib wizard_spellbook_size_tests`: `2 passed; 3 failed`;
  file restored after (log `root-mutation.log`).

## Red evidence, second pass (F3 completion, F2 label)

- `cargo test --lib -- meticulous drawback_tests` (log `red-f3-root.log`): `6 passed; 1 failed` —
  `0 ranks in Swim: left Some(0) right Some(-2)`.
- `cargo test -p codex-desktop --bin codex-desktop -- trait_picker` (log `red-f3-picker.log`):
  `drawback:attached is offered` panicked; count `left: 53 right: 70`.
- `npx tsx src/characterHub/ManageBox.test.ts` → `Error: the new label`.

## Verification, second pass (after all F3/F2 edits)

| command | result |
|---|---|
| `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` (repo root, log `root-full-2.log`) | 299 suites ok, 0 failed; 6538 passed, 0 failed |
| `cargo test --locked -j 8 --no-fail-fast -p codex-desktop -- --test-threads=8` (log `desktop-full-2.log`) | 1 FAILED: `Running unittests src/main.rs` → `trait_picker::tests::every_flat_option_id_is_recognized_by_the_compute_path` ("drawback:attached ... not recognized by skill_bonuses_from_traits"). The guard now skips drawbacks (covered by their own test, which also asserts `trait_effects::is_drawback`). |
| same, rerun (log `desktop-full-3.log`) | 2 suites ok, 0 failed; 785 passed, 0 failed |
| `npm run typecheck` / `npm test` | exit 0 / `154/154 test files passed.` |

Live app (Xvfb, `RUN_DESKTOP_AGENT=brief1`, `CODEX_REPO_ROOT=<worktree>`): Traits dialog lists
70 options, 17 marked "(drawback)", status "Traits: 2 of 2 · Drawbacks: 1 of 1 remaining". A
third trait was refused until Meticulous was taken; after that "Traits: 0 of 3 · Drawbacks: 0 of
1", and the `>` button stayed disabled for Vain. Creating a Dwarf Fighter 1 with Acrobat, Bastard,
Beast Bond and Meticulous answered "Meticulous Test is ready / Your character was computed and
saved." The Racial Traits button reads "Swap racial traits". No "H2" in the page text. Screenshot
`/home/ubuntu/cargo-target/brief1/f3/traits-drawback.png`. The smoke row itself was red on its
first cold launch ("'Loading' still present after the row's wait budget": the class catalog took
longer than the row waits); the same happened on the first pass. Not caused by this change.

## Verification, first pass (run once, after the first batch of edits)

| command | result |
|---|---|
| `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` (repo root) | 299 suites ok, 0 failed; 6535 passed, 0 failed |
| `cargo test --locked -j 8 --no-fail-fast -p codex-desktop -- --test-threads=8` (apps/desktop/src-tauri) | 2 suites ok, 0 failed; 784 passed, 0 failed |
| `npm run typecheck` | exit 0 |
| `npm test` | `154/154 test files passed.` |
| `npm run ui-smoke:doc` | `Wrote docs/testing/ui-smoke-inventory.md (103 rows, 18 screens).` |

Counts derived with `awk '/^test result/{p+=$4; fl+=$6} END{print p, fl}' <log>` and
`awk '/^test result: ok/{ok++} /^test result: FAILED/{f++} END{print ok, f}' <log>`
(logs under `/home/ubuntu/cargo-target/brief1/logs/`). A first root run without `--no-fail-fast`
stopped at `tests/sd13_wizard_level1_prepared_spell_baseline.rs` (1 failure: its "no fabricated
spell explanation" guard rejected the new `class_spell.wizard.spellbook_size.*` rows); the guard
was widened and the full run above is the one that counts. No `test result: FAILED` line remains.

Live app check (Xvfb, `RUN_DESKTOP_AGENT=brief1`, `CODEX_REPO_ROOT` set to the worktree — without
it the app read the corpus from the custom target dir and failed "No race could be read"): a
level-5 Wizard (Int 10) Spells dialog showed one pair per level 0–3, "21 of 21" cantrips, level 1
"11 of 11", level 2 "6 of 6", pools "level 1 or higher: 11 of 11 · level 2 or higher: 6 of 6",
box "32 of 32 remaining". Screenshot `/home/ubuntu/cargo-target/brief1/f1/wizard5-spells.png`.

## Decisions

- **Drawbacks (F3):** 17 records (`grep -rli "Drawback ~" data/corpus/ultimate_campaign | wc -l` = 17),
  id `drawback:<file slug>`, descriptions verbatim (`%%` read as `%`). 16 are situational and are
  printed as text. Meticulous's −2 (corpus: `BONUS:SKILL|<skill>|-2|!PRESKILL:1,<skill>=1` on every
  skill) is `Drawback::untrained_skill_penalty` and `allocate_skill_ranks` adds it to every skill
  total it reports at 0 ranks (allocated at 0, or present only through a trait bonus). A skill with
  no reported total gets none: the engine reports no total for it at all.
- **Count rule:** traits ≤ 2, or ≤ 3 with exactly one drawback; drawbacks ≤ 1. Checked in
  `create_character_at_root` only (not in `build_create_input`), so draft dialogs keep computing.
- **Wizard spellbook (F5):** `spell_level_0` = every level-0 entry of the wizard list
  (21 in the live app); `spell_level_1_and_higher` = max(0, 3 + Int mod) + 2 × (level − 1);
  `spell_level_k_and_higher` (k ≥ 2) = 2 × wizard levels 2..L whose highest castable level ≥ k.
  The UI applies every bound as a shared pool. Opposed schools are not removed.
- **No picker (read-only spells per day):** cleric, druid, paladin, ranger, warpriest, shaman,
  antipaladin. Casters with per-day rows but no known rows (witch, arcanist, alchemist,
  investigator, hunter, skald, …) are also read-only, with a sentence saying the engine has no
  known count for them. This removes the old per-day picker for those classes (operator ruling:
  quota = known, not per day).
- **F6:** grouped pairs only when `pairPerGroup` is set (spells). Traits use group limits for
  enforcement but keep one pair.
- **F2:** count label `N innate · M replaced by alternates`; button "Swap racial traits".
- **F1:** not reproduced. Spec row now carries `forbid: ["H2"]`. An exact heading-text check
  needs `run.mjs` (out of scope).

## Four-check Wired Integration audit (`tranche-17-ui...HEAD`)

```
1: OK_NO_TOKENS
2: OK_NO_NOOP_HANDLERS
3: OK_NO_MOCK_LEAKS
4: OK_NO_WOULD_STRINGS
```

## What the proof does NOT cover

- Multiclass casters (wizard rows exist only via `wizard_level_in_mix`; sorcerer rows only for single-class).
- Cleric/Druid/Paladin/Ranger known-spell semantics beyond "shows per day as text".
- Opposed-school cantrip exclusion; Int too low to cast a level.
- Meticulous on the screen: see Blockers. Engine totals are tested; the sheet's skill rows are not.
- F1: one dev build on Xvfb; the operator's build was not inspected.
- Bloodrager/Bard known rows are parsed by the same regex but only Sorcerer is tested end to end.

## Blockers / findings

1. **Meticulous's −2 does not reach the skill rows the player reads.** It is computed into
   `SkillTotals` (`allocate_skill_ranks`), but the desktop sheet does not read those totals:
   `CharacterSheet.tsx:702` computes each row on the frontend as ability + ranks + class skill +
   feat bonus (`skillModifier`), with engine values only for Climb/Intimidate/Swim from
   `selected_skill_modifiers` (a different engine path). `grep -rln "printed_sheet_cell_map\|PrintedSheetCell"
   apps/desktop` finds no consumer either. This is not new: the existing trait skill bonuses
   (Acrobat's +1 etc.) are missing from those rows for the same reason. Closing it needs the sheet
   to render the engine's skill totals: write scope to `CharacterSheet.tsx`, the skills DTO in
   `character_hub.rs`/`pf1_adapter.rs`, and their tests. Escalated; not done here.
2. **F1 exact heading assertion** needs `apps/desktop/scripts/ui-smoke/run.mjs` (the runner only
   matches page text). The row's `forbid: ["H2"]` is the guard that is in scope.
