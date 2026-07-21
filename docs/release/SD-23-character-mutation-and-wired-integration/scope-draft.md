# SD-23 Scope Draft — Character Mutation and Wired Integration

> **⚠️ OPERATING METHOD — REQUIRED FOR THIS BUNDLE**
>
> This bundle is operated via `/loop 60m /batch /goal programs/codex/requirements/SD-23-character-mutation-and-wired-integration/loop-instruction.md`, **not** via ad-hoc single-task invocations.

## 1. Bundle identity

- **SD-N:** SD-23
- **Slug:** `SD-23-character-mutation-and-wired-integration`
- **Branch:** `tranche/5-1` (dash release from `tranche/5`)
- **Board:** `codex-tranche-5` (reused after SD-22 closure PR lands)
- **Active SD on parent trunk:** SD-22 (`tranche/5`, ~30 acceptance criteria, mid-execution as of 2026-07-20)
- **Bundle doctrine-of-record:** `../../governance/no-stub-mvp-doctrine.md` (active 2026-07-20)

## 2. Operator's Law (verbatim, 2026-07-20)

> "No more stub work. No more mock data. I expect everything from this point forward to be fully wired."

This bundle is the first launched under the Wired Integration doctrine. Every shipped cycle passes the four-check audit defined in `wired-integration-discipline/SKILL.md` §"Per-cycle audit" before marking `complete`.

## 3. Bundle goal

Close the UI/backend wiring seams left by SD-21 (Campaign Manager) and SD-22 (Content Ingest) and ship the first typed `mutate_saved_character` operation surface for character progression. Specifically:

1. **Campaign Manager simplification.** Drop the OAuth/Drive-API surface (operator directive 2026-07-20); keep the local-folder contract. Close the `driveActionSummary` / `CampaignMember.invited` stubs.
2. **Character mutation surface.** Build a typed operation table for mutating saved characters — load → mutate → recompute → re-save → return envelope. Operations: `level_up_character`, `add_equipment_selection`, `add_spell_selection`. Each operation is a Tauri command; each is tested end-to-end.
3. **Wired Integration Cleanup.** Apply the per-cycle audit doctrine across the codebase. Remediate any stubs not in the operator-granted registry. Author the Stubs Registry at `../../governance/wired-integration-stubs-registry.md`.
4. **Storage tier minimal fix.** Add `delete_character` and `import_character` Tauri commands on the existing file-based `SavedCharacterStore`. Closes the Load Character screen's no-op buttons. No database; no migration.
5. **Picker UI for adding equipment/spells.** New modal component (search input + filtered list + select) wired to the new Tauri commands and to the existing `Add Weapon` / `Add Armor` / `Add Spell` affordances on the character sheet.
6. **Level Up dialog wiring.** New `LevelUpDialog.tsx` component; `+` button on the Level box opens the dialog; Accept calls `level_up_character` and refreshes the character sheet's `detail` prop.

## 4. Out of scope (deferred to future bundles)

- **Database / storage-tiers convergence.** Operator ruling 2026-07-20: structural fix (Option B) deferred. Research artifact at `programs/codex/research/storage-tiers-convergence-2026-07-20.md`. Latent referential-integrity risk between campaigns (localStorage tier 2) and characters (file-store tier 1) is captured in `risks-and-open-questions.md`.
- **Stat-field promotion for added equipment/spells.** Picker adds items as identity-only entries; mechanical fields (damage, AC bonus, range/duration/save) are deferred. Items added in this bundle will appear in the saved character but won't yet affect computed combat stats.
- **Auto-granting spells/feats at level-up.** `level_up_character` takes the level; it does not choose specific known spells or bonus feats. Deferred to a future bundle once the corpus → generated-table promotion lands.

## 5. Epic structure (7 epics / 33 acceptance criteria / 16 closure gates)

| Epic | Name | Criteria | Fires | Depends on |
| --- | --- | --- | --- | --- |
| 1 | Code-Side Identifier Cleanup | 1-4 | FIRST (per doctrine) | — |
| 2 | Operator Pre-Launch | 5-6 | After Epic 1 | Epic 1 |
| 3 | Wired Integration Cleanup | 7-11 | After Epic 1 | Epic 1 |
| 4 | Campaign Manager Simplification | 12-15 | After Epic 3 | Epic 3 |
| 5 | Character Mutation Surface | 16-21 | After Epic 3 | Epic 3 |
| 6 | Storage Tier Minimal Fix (Delete / Import) | 22-24 | After Epic 3 | Epic 3 |
| 7 | Closure Epilogue | 25-30 | LAST | All |

Full criterion text and per-cycle story at `epic-breakdown.md`.

## 6. Operator rulings baked in

- **Slug:** `character-mutation-and-wired-integration` (operator-confirmed 2026-07-20).
- **Branch:** `tranche/5-1`, distinct from SD-22's `tranche/5`. Operator ruling 2026-07-20.
- **Board:** `codex-tranche-5`, reused after SD-22 closure PR lands. Operator override of the convention slug (which would have produced `codex-tranche-5-1`).
- **Google OAuth:** Dropped. No OAuth, no Drive API. "Drive folder" is a local folder the user configures. Operator ruling 2026-07-20.
- **Member invites:** Deleted entirely. `CampaignMember.invited` field removed from data model. Operator ruling 2026-07-20.
- **Storage tier fix:** Option A (minimal file-store). Operator ruling 2026-07-20. Option B (database) deferred.
- **Stubs exception mechanism:** `../../governance/wired-integration-stubs-registry.md` is the doctrine-of-record. Operator-confirmed permanent exceptions only.
- **Build counter inheritance:** Tranche-base 5 (same as SD-22). First concrete value `0.5.<current_build_at_SD22_closure_merge>`; full inheritance rule at `decisions.md` §3.
- **Default-assignee rule:** SD-23 cards minted with `--assignee tech-priest` for CODE lanes, `--assignee god-emporer` for OPS lanes. Never `default`, never `vanderspeigle`.

## 7. Pre-launch checklist (run before cycle 1)

1. SD-22 closure PR merged to develop (no SD-23 cycle launches until this is true).
2. `git branch tranche/5-1 origin/develop` (new dash branch off post-SD22-closure develop).
3. `codex-tranche-5` board exists and is reachable.
4. OAuth credentials present at `~/.config/gh/.claude_gh_token` (classic PAT for ruleset/branch-protection admin).
5. Working tree clean (`git status --porcelain` returns empty).
6. Doctrines referenced (`identifier-discipline.md`, `no-stub-mvp-doctrine.md`) are loaded by the loop's skill list.
7. Build counter at SD-22 closure captured in `decisions.md` §3 from develop's `Cargo.toml` workspace version.

## 8. Cross-references

- `loop-instruction.md` — operational cycle mechanics
- `epic-breakdown.md` — 7 epics / 33 criteria / per-cycle story
- `decisions.md` — decision log
- `risks-and-open-questions.md` — latent risks and deferred questions
- `acceptance-and-verification.md` — test-surface contract and closure gates
- `progress.md` — cycle log
- `../../governance/no-stub-mvp-doctrine.md` — parent doctrine
- `../../governance/wired-integration-stubs-registry.md` — operator-granted stub exceptions
- `../../research/storage-tiers-convergence-2026-07-20.md` — deferred structural question
