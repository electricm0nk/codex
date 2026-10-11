# Brief 1 — cycle receipt (create-character fixes, F1–F6)

Branch `ui-brief1` (worktree `.worktrees/brief1`), base `tranche-17-ui` @ 5f020b221b.
Status: **NOT complete.** F3 is blocked on write scope (see Blockers). F1 did not reproduce.

## Commits

| sha | subject |
|---|---|
| e6f67b45c7 | fix(brief1-F4): read class_chassis spells-per-day rows |
| 04d87eaa22 | feat(brief1-F3/F5): wizard spellbook size, draft known-spell rows, drawbacks + trait count check |
| 21a41f3bc7 | feat(brief1-F6): one Options/Selected pair per spell level, shared pools |
| 253031fb4e | feat(brief1-F2/F3/F5): known-spell quotas, drawback trait limits, racial trait counts |
| 2d935014b7 | test(brief1-F1): forbid stray 'H2' on the Create screen smoke row |

`git log --oneline tranche-17-ui..ui-brief1`

## Files changed

- Engine: `src/rules_core/pilot_compute/class_wizard_prepared_spellbook.rs` (the brief's
  `src/rules_core/class_wizard_prepared_spellbook.rs` lives under `pilot_compute/`),
  `src/rules_core/trait_effects.rs`, `tests/sd13_wizard_level1_prepared_spell_baseline.rs`
  (guard allowlist widened for the new `spellbook_size` rows; read as the brief's "+ tests").
- Desktop Rust: `apps/desktop/src-tauri/src/character_hub.rs`.
- Frontend: `spellsPerDayModel.ts`, `spellDialogModel.ts`, `transferListModel.ts`,
  `TransferListDialog.tsx`, `manageItems.ts`, `creationRacialTraitsPreview.ts`,
  `CreateCharacterForm.tsx`, `boundary/draftSpellOptions.ts`, `boundary/loadCharacterTraits.ts`.
- `apps/desktop/scripts/ui-smoke/spec.json`, regenerated `docs/testing/ui-smoke-inventory.md`.

## Tests added

- Rust engine: `wizard_spellbook_size_tests` (5), `trait_effects::drawback_tests` (4, incl. a
  corpus-parity test over every `Drawback ~` file).
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

## Verification (run once, after all edits)

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
  id `drawback:<file slug>`, descriptions verbatim (`%%` read as `%`). All 17 are printed text; none
  is folded into a total. 16 are situational. Meticulous (−2 on untrained skills) does feed sheet
  totals but needs `skill_allocation.rs` (out of scope) — see Blockers.
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
- **F2:** count label `N innate · M replaced by alternates`. Button text unchanged (see Blockers).
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
- Drawbacks never reach the picker (trait_picker.rs not changed) — the UI and server paths are
  tested only with fixture options / direct ids.
- F1: one dev build on Xvfb; the operator's build was not inspected.
- Bloodrager/Bard known rows are parsed by the same regex but only Sorcerer is tested end to end.

## Blockers (escalated to the lead, waiting)

1. **F3 drawbacks never shown to the player.** The trait DTO is `CharacterTraitOptionDto` in
   `apps/desktop/src-tauri/src/trait_picker.rs` (`list_available_character_traits`, line 174), not
   in `character_hub.rs`. Need write scope there to add `is_drawback` and append `DRAWBACKS`.
2. **Meticulous −2** needs `src/rules_core/skill_allocation.rs` to apply to untrained skill totals.
3. **F2 button rename** needs `apps/desktop/src/characterHub/ManageBox.tsx` (a `manageLabel` prop).
4. **F1 exact heading assertion** needs `apps/desktop/scripts/ui-smoke/run.mjs`.
