# Brief 3 — Starter characters (items 14–15)

STATUS: APPROVED by operator 2026-10-11. Independent of Briefs 1–2 (Rust only + docs); can run in parallel in its own worktree.

## Objective
Ship three starter characters: caster (Elowen, Wizard 5), fighter (Aldric, Fighter 3), dual class (NEW: Cleric 3 / Fighter 2). Find and fix why Aldric is missing on the operator's machine.

## Repo / workdir
`/home/ubuntu/workspace/repos/codex`, `apps/desktop/src-tauri/src/character_hub.rs` (`starter_seeds()` ~2096; markers ~1986–1994). Own worktree + own `CARGO_TARGET_DIR`; delete it afterwards.

## Findings
- VERIFIED: seeding runs in `main.rs` `.setup()` (~221) → `seed_default_character_if_needed` → `seed_one_starter`. Skipped if its marker file exists. Errors are only `eprintln!`'d — a failed Aldric is silent.
- VERIFIED: a deleted seed stays deleted by design (test `a_deleted_seed_does_not_come_back`; `docs/architecture/persistence.md:283-290`).
- VERIFIED on THIS machine: `~/.local/share/io.electricm0nk.codex` has only marker 1 and one character (Aldric). The installed build is 0.6.0 and predates Elowen (commit 8f3b15a8d2, 2026-09-27). The operator's data dir is different; we have not seen it.
- INFERRED (two candidates): (a) Aldric's marker exists but his dir was deleted → marker blocks reseed; (b) Aldric's build failed → only stderr.
- Multiclass gap: `create_character` applies `additional_levels` via `apply_level_up` (lines ~1687–1689), but `seed_one_starter` calls `compose_character_input` directly and skips that loop (lines ~2200–2210). `starter_seed_request` hard-codes Human, empty `additional_levels`.

## Tasks

### S0 — Diagnose Aldric (before any fix)
Operator runs on their machine: `ls -a ~/.local/share/io.electricm0nk.codex | grep default_character` and `ls ~/.local/share/io.electricm0nk.codex/characters`. Marker 1 present + folder `…0001` absent → cause (a). Both absent → cause (b); read stderr for `starter character 'Aldric Ironhand' did not compute`.

### S1 — Surface seed failures
Test first: a seed that fails to compute produces a visible error (not only stderr). Smallest change: return the collected errors to the UI/diagnostics.

### S2 — Third seed, Cleric 3 / Fighter 2
- Add `StarterSeed.additional_levels` and apply it in `seed_one_starter` (same `apply_level_up` path as `create_character`).
- New marker `.default_character_seeded_3`, id `00000000-0000-0000-0000-000000000003`, pushed after the existing two.
- Skill ranks: documented formula `(class ranks/level + Int mod + 1 Human) × levels` (Aldric (2+2+1)×3 = 15; Elowen (2+4+1)×5 = 35). Cleric and Fighter are both 2/level. Max ranks per skill = character level (5). Zero unallocated points (mirror `aldric_loads_with_zero_unallocated_skill_points`).
- Cleric spells: pick from the real catalog via `record_and_prepare_spells`, validated against `SPELL_LIST` (mirror `the_fireball_seed_id_is_the_converted_crb_record`). Open: domains/deity have no seed precedent.
- Feats: current loadout is fixed for all classes (Power Attack, Dodge, Weapon Focus longsword; longsword + chain shirt). Fighter bonus feat / human feat choices must be checked against the create path.
- Suggestion only (name/scores are the operator's call): "Brother Tomas Vane", Human, Str 14, Dex 10, Con 14, Int 10, Wis 16, Cha 12.
- RISK (plain): it is untested whether Cleric 3 / Fighter 2 reaches `Computed`. If it does not, that is an engine blocker, to be escalated with the exact failing receipt — not worked around.

### S2b — "Restore starter characters" button (operator ruling 2026-10-11)
- Keep the delete rule: a deleted starter stays deleted on normal launch. The button is the ONLY way back.
- Rust: new Tauri command `restore_starter_characters`. For each seed in `starter_seeds()` whose character dir is missing, build and save it (same `seed_one_starter` path, marker ignored), then write its marker. A seed whose dir exists is NEVER touched (pin with the existing rule `a_seed_whose_id_already_exists_is_never_overwritten`). Returns the labels restored. A seed that fails to compute returns its error to the UI (same path as S1), never `success: true`.
- Register the command in `main.rs` `invoke_handler` (a command that is not registered does nothing; this is the "closed under the change" check).
- TS: new boundary file `apps/desktop/src/boundary/restoreStarterCharacters.ts`; button on the character list screen (`LoadCharacterScreen.tsx`, heading at line 350). After the call, reload the list. Message: `Restored: Aldric Ironhand` or `All starter characters are already here.`
- Tests first: Rust (deleted seed returns; existing seed untouched; failing seed reports its error; marker rewritten); TS model test for the message text; ui-smoke row for the button.
- Cost: the full seed set computes in one pass, no extra engine work beyond S2.

### S3 — Update every surface that counts seeds
Live files (from `grep -rln 'Elowen\|Aldric' . --exclude-dir={node_modules,target,graphify-out,.git}`; do NOT edit `docs/release` or `docs/retro` history): `character_hub.rs` (constants, `starter_seeds()`, `starter_seed_tests`, comments ~1990, ~2141); `apps/desktop/src/testSupport/starterSeedListWire.ts`; `docs/release/SD-36-consolidation/artifacts/epic-f/stage-f6/f6e-starter-seed-list-wire.json` (regenerate with `CODEX_WRITE_F6E_WIRE=1` + the Rust test — outside the obvious scope; needs operator write approval); `apps/desktop/scripts/ui-smoke/spec.json` (`load-seed-list-both` ~635, `open-seed-aldric` ~5255) + generated `docs/testing/ui-smoke-inventory.md`; `docs/architecture/persistence.md` 283–290; `docs/architecture/desktop-app.md` 218–250; `appDataIsolation.mjs` 12–13; `apps/desktop/.claude/skills/run-desktop/SKILL.md:122`. Frontend tests mentioning the names (not checked for hard-coded counts): `buildCharacterHubListSurface.test.ts`, `skillsModel.test.ts`, `abilityScoresModel.test.ts`, `composeCreateCharacterRequest.test.ts`, `buildCreateCharacterOutcomeSurface.test.ts`. `previewData.ts:14-23` describes Aldric as Fighter 3 / Wizard 1 — hand-written preview data that disagrees with the seed; flag, do not silently change.

## Tests (Rust, `starter_seed_tests`, `temp_app_data_dir`)
- Fresh install → labels `[Aldric, Elowen, <new>]`, classes Fighter 3 / Wizard 5 / Cleric 3 + Fighter 2, three markers.
- Existing install with markers 1+2 gains the third; Aldric's dir byte-identical (`snapshot_dir`).
- Deleted third seed stays deleted.
- Aldric-missing regression test per S0 result.

## Allowed write scope
`apps/desktop/src-tauri/src/character_hub.rs` (+ `main.rs` for S1 and command registration), new `apps/desktop/src/boundary/restoreStarterCharacters.ts` (+ test), `apps/desktop/src/characterHub/LoadCharacterScreen.tsx` (+ test), `apps/desktop/src/testSupport/starterSeedListWire.ts`, the wire JSON, `ui-smoke/spec.json`, generated inventory, the two architecture docs, `appDataIsolation.mjs`, `run-desktop/SKILL.md`. Not `docs/release|retro` history.

## Non-goals
No change to the delete rule on normal launch (the button is the only way back). No new race/class support. No cleric domain modelling beyond what the seed needs.

## Verification (once; batch Rust)
`cd apps/desktop/src-tauri && cargo test starter_seed_tests` (package name unverified; confirm in `Cargo.toml`), scratch `CARGO_TARGET_DIR`; `npm test` in `apps/desktop`; four-check Wired Integration audit. UNCOVERED: the operator's real app-data dir; a built app launch; ui-smoke (needs a built app).

## Models
Opus 5.5 for S2 (engine risk). Sonnet 5.5 for S1/S3. Haiku 4.5 for docs/housekeeping.

## Open questions
1. (RESOLVED 2026-10-11: add the Restore button, S2b.)
2. (RESOLVED 2026-10-11: operator accepts "Brother Tomas Vane", Human, Str 14 Dex 10 Con 14 Int 10 Wis 16 Cha 12.)
3. (RESOLVED 2026-10-11: operator approves regenerating `docs/release/SD-36-consolidation/artifacts/epic-f/stage-f6/f6e-starter-seed-list-wire.json`.)

## Remaining before dispatch
- Operator approves this brief as a whole.
- S0 (operator runs the two `ls` commands) is a diagnosis input, not a blocker for S1–S3; S2b makes Aldric recoverable either way.
