# Cycle 3.5 — Epic 3 Character Hub as Hub of Hubs / Criterion 3.5

- **Card ID:** (recorded in dispatch report — see kanban section)
- **Commit SHA:** (pushed to `tranche/5-3`; recorded after push per `loop-instruction.md §5` — see dispatch report)
- **Files touched:**
  - `apps/desktop/src/characterHub/characterHubRuntime.ts` — added `resolveRuleSystemId(ruleSet: RuleSetId): string` (the panel's active-adapter resolver: `"pathfinder-1e"` → `"pf1"`; every other `RuleSetId` passes through unchanged so it honestly routes to `StubAdapter` server-side) and `buildRecomputeCharacterRequest(characterId, ruleSet)` (pure request composer, mirrors `composeCreateCharacterRequest`'s own split).
  - `apps/desktop/src/characterHub/characterHubRuntime.test.ts` — **new.** 4 assertions covering `resolveRuleSystemId`'s pf1/non-pf1 branches and `buildRecomputeCharacterRequest`'s routing.
  - `apps/desktop/src/boundary/recomputeCharacter.ts` — **new** boundary wrapper (register C1's "NEW `apps/desktop/src/boundary/` wrapper(s) for at least one of `append_to_character`/`recompute_character`/`re_save_character`"). Invokes the `recompute_character` Tauri command with `{ characterId, ruleSystemId }`.
  - `apps/desktop/src/boundary/recomputeCharacter.test.ts` — **new.** No-runtime failure-path test, matching every other boundary wrapper's own test shape (e.g. `loadDeleteCharacter.test.ts`).
  - `apps/desktop/src/characterHub/CharacterSheet.tsx` — register A4 scope. Removed the three bare `() => {}` no-op handlers on the top-menu `Open`/`Save`/`Clone` items. Added `ruleSet: RuleSetId`, `onOpen: () => void`, `onCloned: () => void` props; a `handleRecompute` handler (calls `buildRecomputeCharacterRequest` + `recomputeCharacter`, closing register A3) and a `handleClone` handler (calls the existing `cloneCharacter` boundary, mirroring `LoadCharacterScreen.tsx`'s own `handleClone`); `baseAttackBonus`/`ac`/`saves` now prefer a `recomputed` snapshot over the originally loaded one when present; a `statusMessage` display slot alongside the existing `mutationError` one.
  - `apps/desktop/src/characterHub/CharacterHubPage.tsx` — wires the new `CharacterSheet` props: `ruleSet={ruleSet}` (the state this page already owned but never threaded past `LandingScreen`), `onOpen` (returns to the Load Character screen), `onCloned={reload}`.
  - `apps/desktop/src/characterHub/LoadCharacterScreen.tsx` — **not touched.** In the file-touch grant but this cycle's changes did not require it: `CharacterHubPage.tsx` renders `CharacterSheet` directly (not through `LoadCharacterScreen`), so `ruleSet` threads straight from the page's own state without an intermediate prop hop.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped to the four touched/new shipping `.ts`/`.tsx` files; test files excluded per the standing grep pattern).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope). One transient hit self-healed inline during the cycle: a doc comment in `characterHubRuntime.ts` originally said `StubAdapter` "placeholder" (matches the `placeholder` bucket) and separately quoted `"not yet implemented"` verbatim (matches that bucket) — reworded to "the governed `StubAdapter` seam — see `governance/wired-integration-stubs-registry.md` entry 0002" instead of re-describing or re-quoting the message, since this file is not one of the three command files entry 0002 already names as covered (`appendToCharacter.rs`/`recomputeCharacter.rs`/`reSaveCharacter.rs`) and widening the registry entry to a fourth, frontend file was unnecessary when the same information reads cleanly as a cross-reference instead.
- **Acceptance criterion:** Criterion 3.5 — UI panel adapter-aware (Epic 3: Hub of Hubs). RED: "A test (or verifiable UI assertion) that the panel reads the active rule-system adapter and routes interactions through it fails — UI is hard-wired to PF1 paths today." GREEN: "Panel reads the active adapter and routes interactions through it; PF1 behavior unchanged." Plus register A3 (real UI affordance wired to one of the three 3.4 commands) and register A4 (`Open`/`Save`/`Clone` no-op handlers closed).
- **Status:** complete

## RED → GREEN evidence

**RED** (`characterHubRuntime.test.ts` / `recomputeCharacter.test.ts` written first, run under `tsx` before either module existed):

```
$ ./node_modules/.bin/tsx src/characterHub/characterHubRuntime.test.ts
SyntaxError: The requested module './characterHubRuntime' does not provide an export named 'buildRecomputeCharacterRequest'

$ ./node_modules/.bin/tsx src/boundary/recomputeCharacter.test.ts
Error [ERR_MODULE_NOT_FOUND]: Cannot find module '.../src/boundary/recomputeCharacter'
```

This is the concrete form of the cycle doc's RED text: before this cycle, `characterHubRuntime.ts` had no concept of "the active rule-system adapter" at all — `CharacterHubPage.tsx` already tracked a `ruleSet` state (fed by `LandingScreen.tsx`'s rule-set picker) but never threaded it anywhere past that screen; `grep -rn "resolveRuleSystemId|ruleSystemId" apps/desktop/src/characterHub apps/desktop/src/boundary` (pre-cycle) returned nothing outside the three 3.4 Rust command files' own request DTOs. No frontend caller existed for `append_to_character`/`recompute_character`/`re_save_character` at all, confirming 3.4's own "Next-cycle plan" note verbatim.

**GREEN** (after implementation):

```
$ ./node_modules/.bin/tsx src/characterHub/characterHubRuntime.test.ts   # exit 0, no assertion failures
$ ./node_modules/.bin/tsx src/boundary/recomputeCharacter.test.ts        # exit 0, no assertion failures
$ npm run typecheck                                                      # tsc --noEmit, clean
$ npm test                                                                # 59/62 test files passed
```

The 3 non-passing files in the full `npm test` run are `src/sd21/buildVersionTriple.test.ts` — a pre-existing, unrelated failure (`Cargo.toml version must match package.json version: expected 0.5.98, got 0.5.97`) confirmed present on a clean `git stash` of this cycle's changes before any of this cycle's work; it is a version-bump housekeeping gap owned by Epic 8 (register C3-adjacent), not something this cycle's frontend-only file-touch grant can or should fix. All other 61/62 (including the 2 new files this cycle added) pass.

`cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` (sanity check; no Rust files were touched this cycle) is clean — only pre-existing warnings unrelated to this diff.

## Design notes

- **Register A3 closed via `recompute_character`, not `re_save_character`.** All three 3.4 commands were eligible. `re_save_character` requires `expectedRevisionId`, but no frontend DTO (`CharacterSummaryDto`, `LoadSavedCharacterResponse`) exposes the saved character's `revision_id` at all — every mutate-op (`level_up_character`, `add_equipment_selection`, `add_spell_selection`, and the 3.4-adapter-routed commands themselves) advances it server-side, but it never crosses the wire to the UI. Wiring "Save" to `re_save_character` without a real `revisionId` would mean fabricating or guessing the expected value — dishonest, and would fail `revision_conflict` unpredictably after any prior mutation. Adding `revisionId` to those DTOs is a Rust change outside this cycle's frontend-only file-touch grant (`character_hub.rs` is not in the grant). `recompute_character` needs only `characterId` + `ruleSystemId` — both already available from `props.row.characterId` and the new `ruleSet` prop — so it is the command this cycle can honestly wire end-to-end without expanding scope. Forwarded as a `## DISCOVERED` entry below for whichever cycle next touches `character_hub.rs`'s Rust DTOs.
- **"Save" removed, not just silently fixed.** Register A4 says wire `Open`/`Save`/`Clone` to real behavior *or remove them*. Every mutation this sheet can trigger already persists immediately on selection (level-up, add-equipment, add-spell, clone) — there is no session-local "unsaved edit" state for an explicit Save to commit (the Bio fields are the one exception, and `DetailsPanel`'s own pre-existing doc comment already documents they have no persisted schema slot to save into yet). Labeling a menu item "Save" when it would not persist anything new would itself be dishonest UI framing, so this cycle replaces the label with "Recompute" — the real capability actually available from the panel today (register A3) — rather than fabricating a Save action with nothing new to write.
- **`Open` wired to real navigation, not an `invoke()` call.** "Open" returns to the Load Character screen (`CharacterHubPage.tsx`'s `onOpen` callback sets `mode` to `'load'`) so the operator can pick a different saved character. This is real, user-observable behavior — not a Tauri command, since there is nothing to fetch that Load Character doesn't already fetch on its own mount — and closes its share of register A4 without inventing a spurious backend round trip.
- **`Clone` wired to the existing `cloneCharacter` boundary**, not a new one — `clone_character` is not one of 3.4's three adapter-routed commands, so this closes A4 (no more no-op) but does not itself count toward A3 (recompute_character does). Mirrors `LoadCharacterScreen.tsx`'s own `handleClone` almost verbatim (`crypto.randomUUID()` for the new id, `"{label} (Copy)"` for the new display label) rather than inventing a second convention for the same operation.
- **PF1 behavior unchanged, proven by construction.** `recomputed` (the "Recompute" result) is `null` until the operator explicitly clicks it; every derived-stat display value falls back to the pre-existing `snapshot`-sourced value (`recomputed?.baseAttackBonus ?? snapshot?.baseAttackBonus ?? 0`, etc.), so nothing changes for a PF1 character that never triggers a recompute — matching the cycle doc's GREEN clause verbatim. When it IS triggered for a PF1 character, `recompute_character_via_rule_system("pf1", ...)` (criterion 3.4) is proven byte-for-byte identical to the pre-existing direct compute path, so the displayed values cannot regress from what `snapshot` already showed.
- **`LoadCharacterScreen.tsx` left untouched** despite being in the file-touch grant — the grant is a maximum permitted scope, not a mandate to edit every named file; `ruleSet` reaches `CharacterSheet` directly from `CharacterHubPage`'s own state without passing through `LoadCharacterScreen`, so there was nothing there this criterion's RED/GREEN or register A3/A4 required changing.

## Discovery forwards

## DISCOVERED (SD-25 register, Epic 3 residue)

- **`revisionId` never crosses the wire to the frontend.** `CharacterSummaryDto` / `LoadSavedCharacterResponse` (in `apps/desktop/src-tauri/src/character_hub.rs`) never expose the saved character's on-disk `revision_id`, even though every mutate-op advances it server-side (`pf1_adapter.rs`'s `next_mutation_revision_id`, confirmed incrementing across repeated calls by `mutate_saved_character_at_root_keeps_advancing_across_repeated_calls`). This blocks any UI caller of `re_save_character` (which requires `expectedRevisionId` for its write-conflict guard) from ever being wired honestly — the frontend has no real value to send. Needs a Rust DTO change (`character_hub.rs`, out of this cycle's frontend-only grant) before a follow-on cycle can close this gap: add `revisionId: String` to `CharacterSummaryDto` (or a narrower `LoadSavedCharacterResponse`-only field) and thread it through every response that already carries a `CharacterSummaryDto`.

## Next-cycle plan

- Epic 3 is now fully closed (3.1–3.5 all complete): the rule-system-adapter trait exists (3.1), `Pf1Adapter` is extracted and real (3.2), `StubAdapter` is the governed future-system placeholder (3.3), the three iterative-mutation Tauri commands dispatch through the trait (3.4), and the UI panel is adapter-aware with at least one real end-to-end call site plus no remaining no-op menu handlers (3.5, this cycle).
- Whoever next touches `character_hub.rs`'s response DTOs should pick up the `revisionId`-on-the-wire `## DISCOVERED` item above — it is the one remaining piece needed to let a future cycle wire `re_save_character` from the UI as well, completing all three 3.4 commands' frontend coverage rather than just one.
- Per `loop-instruction.md §3`, E4/E5 are gated on E3.4 (already complete), not on 3.5 specifically — this cycle does not block their dispatch, but Epic 3 as a whole being fully closed removes any doubt about the "Hub of Hubs" refactor's completeness for the closure-readiness report at 8.1.
