---
title: v0.8 — Orchestrator ticket queue (triaged from scout-gap-audit.md)
status: active
branch: tranche/14-ui
date: 2026-09-01
---

# Ticket queue

Triaged per brief §3.2 from `scout-gap-audit.md`. A punch-list line is not a ticket; each entry
below carries its own objective, write scope, and verification command. Teammates self-claim from
this file. **No teammate commits** (§3.3) — report done, `qa` verifies, orchestrator commits.

## Orchestrator rulings on scout's open questions

- **Q1 (multiclass at creation) → shape (a).** Creation stays single-class; multiclass is reached
  through the existing level-up flow. No `CreateCharacterRequest` class list this sprint.
- **Q2 (creating above level 1) → shape (a).** Create at level 1, then run the real level-up flow
  per level, collecting choices each step. Reuses what exists; no batch-choices step.
- **Q3 (aging) → drop the preview.** The engine has no aging model, so persisting it would author
  rules in TS (§3.3 forbids). Remove the age effect from the Calculated column so the form agrees
  with what is saved. (Ticket F-12.)
- **Q4 (pre-save ripple) → create-then-see is acceptable this sprint.** No unsaved-build preview
  compute. Item 17 stays on the punch list, undispatched.
- **Q5–Q12 → deferred.** They gate items that sit behind the backend tickets below; re-ask once
  B-1..B-6 have landed and we know the real wire shapes.

Blockers B1–B12 stand as scout filed them. None are tickets; none get worked around in TS.

---

## Backend queue (`apps/desktop/src-tauri/src/**`, `backend` owns)

Verification for all: `npm run tauri:check` from `apps/desktop/`, plus
`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` where the ticket adds a test.

- **B-1 — `playerName` on the bio sidecar.** Audit item 1 (backend half). Add `playerName` to
  `UpdateCharacterBioRequest` / `CharacterBioDto` / `load_character_bio` in `character_hub.rs`.
  Unblocks F-1's player-name field.
- **B-2 — Expose real ability scores on `load_saved_character`.** Audit item 16. The persisted
  `authoritative_character_input` holds them; `PilotSnapshotDto` carries modifiers only, so the
  sheet prints `10 + 2·mod` and every odd score displays one low. Add the scores to the response.
- **B-3 — Expose persisted skill allocations on `load_saved_character`.** Audit item 19. Surface
  `chosen.skill_allocations` so the sheet can seed from real state instead of the Climb/
  Intimidate/Swim constant.
- **B-4 — Trait add/remove commands.** Audit item 33. State already lives in
  `ChosenCharacterState.selected_traits`; wrap add/remove mirroring
  `add_feat_selection`/`remove_feat_selection`. Register in `main.rs`.
- **B-5 — Equipment active-state command.** Audit item 39. `EquipmentSelection` already carries
  the enum; everything bought is hard-coded `EquippedActive` with no way to stow. Add a
  state-change command.
- **B-6 — `additional_choices` on `CreateCharacterRequest`.** Audit item 12. Use the same
  `SelectedChoiceDto` shape `LevelUpCharacterRequest` already uses. This ticket is the wire only —
  no pickers, no per-pool `list_*_options` commands yet; report back what pools already have a
  list command so the follow-on can be sized. Largest of the six; claim it last.

## Frontend queue (`apps/desktop/src/**`, `frontend` owns)

Verification for all: `npm run typecheck` and `npm test` from `apps/desktop/`.

- **F-1 — Persist the eight bio fields entered at creation.** Audit item 2. `update_character_bio`
  already accepts alignment/deity/sex/age/eyes/hair/height/weight; the form drops all of them.
  Call it after a `saved` outcome. (Player name waits on B-1.)
- **F-2 — Render standard racial traits on race pick.** Audit item 5. The
  `resolve_race_alternate_selection` response already carries `appliedTraits` including default
  rows; only alternates are rendered. A player cannot see Hardy / Stonecunning before committing.
- **F-3 — Render selected traits on the sheet.** Audit item 32. `selectedTraits` is loaded and
  carried through every refresh with zero render sites.
- **F-4 — Defense tab: base vs. total saves.** Audit item 58. `snapshot.baseSaves` and
  `totalSaves` both arrive today; replace the "coming soon" line with the real rows.
- **F-5 — `Overrides` tab placeholder.** Audit item 57. A visible tab with no behavior behind it
  violates the no-stub doctrine. Remove the tab (do not invent a feature to fill it).
- **F-6 — Gear picker beyond `ArmsArmor`.** Audit item 36. Both pickers hard-code
  `WEAPONS_AND_ARMOR_CATEGORY`; the catalog also serves `General`, `MagicItems`, `Cloth`,
  `Backpack`. A player cannot buy rope, rations, or a holy symbol.
- **F-7 — Cost and weight on picker rows.** Audit item 37. `list_equipment` entries already carry
  both; today the player learns the price from their gold balance dropping.
- **F-8 — Export in the sheet's ☰ menu.** Audit item 56. Export exists on the Load screen only.
- **F-9 — Show melee attack bonus.** Audit item 40 (first half only). `baselineMeleeAttackBonus`
  arrives on every load and is never rendered. Ranged/per-weapon totals are blocker B5 — do not
  compute them in TS.
- **F-10 — Don't offer "Add Spell" to non-casters.** Audit item 45. Routing falls back to all
  1185 records for a Fighter. The engine's `known: false` flag is the signal; Magus/Summoner/
  Oracle are the honest exception.
- **F-11 — Class preview next to the class select.** Audit item 9. BAB / base saves / skill points
  per level from `list_class_catalog`, which already returns them per class per level. Display
  only — no rule math authored in TS.
- **F-12 — Drop the age effect from the Calculated column.** Audit item 7, per ruling Q3 above.
  The form previews an aging modifier that `handleSubmit` never submits, so the saved character
  disagrees with what the player saw.

## Not dispatched

Items 17, 24, 26, 29, 30, 42, 46–50, 53, 59 and everything gated on B-6's follow-on pickers stay
on the punch list until the backend queue lands. §3.5 DM Toolkit remains unreached.

---

## Known-red baseline (established, not caused by this sprint)

`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` is **575 passed / 1 failed** and
that one red is the standing bar for every backend ticket. Anything beyond it is a real regression.

The failure is
`corpus_ingest_diagnostic::tests::the_two_ingested_books_totals_reconcile_with_their_license_artifacts`
(`src/corpus_ingest_diagnostic.rs:1508`): the diagnostic pins 127 rules_tables records + 1144
corpus-only records = 1271 for `pathfinder_unchained`, while a live walk of the corpus finds 1267.

Independently established three ways: reported by `backend` during B-2, confirmed by `qa` reading
`git status`/`git diff --stat` (the file is untouched in the working tree), and reproduced by the
orchestrator on a clean `git worktree` checked out at commit 656326195e with zero sprint code
present — identical numbers, 1271 vs 1267.

It is **not fixable inside this sprint's write scope**: the corpus data lives outside
`apps/desktop/**`, and the assertion's own message says "re-derive corpus_only_records fresh
(decisions.md §17a), never repin without proof." Repinning the count to silence it is exactly the
unverified change AGENTS.md forbids. No commit message on this branch may attribute it to a ticket.

Caution for anyone re-running it: `cargo test ... | tail` reports shell exit 0 even when the test
fails, because the pipeline takes `tail`'s status. Read the `test result:` line, not `$?`.
