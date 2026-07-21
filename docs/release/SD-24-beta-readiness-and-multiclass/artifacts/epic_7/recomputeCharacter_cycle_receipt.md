# Cycle recomputeCharacter — Epic 7 (Tauri command-surface repair) / Criterion 7.2

- **Card ID:** t_placeholder7 (placeholder — backfilled once the real kanban card ID is known)
- **Commit SHA:** (recorded in the same-cycle commit that adds this file; see `git log` for the commit touching `apps/desktop/src-tauri/src/characterHub/` + `apps/desktop/src-tauri/src/main.rs`)
- **Files touched:**
  - `apps/desktop/src-tauri/src/characterHub/recomputeCharacter.rs` (new — command implementation + inline `#[cfg(test)]` unit tests)
  - `apps/desktop/src-tauri/src/characterHub/mod.rs` (new — submodule root; required scaffolding to stand up the `characterHub/` dir named by this criterion's file scope; declares the submodule via `#[path = "recomputeCharacter.rs"]` so the on-disk filename stays camelCase per the epic's own TS-side naming convention while the Rust module identifier stays snake_case)
  - `apps/desktop/src-tauri/src/main.rs` (one-line-plus registration: `mod characterHub;` declaration, `use characterHub::recompute_character::recompute_character;` import, and the command's entry in `tauri::generate_handler![...]`)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped to this cycle's touched files: `apps/desktop/src-tauri/src/characterHub/*`, `apps/desktop/src-tauri/src/main.rs`)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope). **Note:** the repo-wide gate (merge-base `origin/develop`...`HEAD`, unscoped) surfaces one pre-existing false-positive hit on the literal English word "hack" inside real PF1 SRD spell flavor text (`src/rules_core/rules_tables/crb/spell_list.rs:738`, the "Plant Growth" spell description — "creatures must hack or force a way through"). This line landed in commit `96d085f` (Epic 6's CRB field-completion cycle), predates this cycle entirely, is outside this cycle's granted file scope, and is real corpus text (not a stub/mock/placeholder token) — removing or editing it to dodge the grep would violate Epic 6's "100% real corpus text, no fabrication" doctrine. Recorded here rather than silently ignored; not attributable to Criterion 7.2.
- **Acceptance criterion (verbatim, `epic-breakdown.md` Criterion 7.2):**
  > **Files touched:** new `apps/desktop/src-tauri/src/characterHub/recomputeCharacter.rs`.
  > **RED:** recomputing after a level-up does not refresh derived stats.
  > **GREEN:** recomputing refreshes BAB / saves / skill points / caster level / etc. against the new level.
- **Status:** complete
- **Notes:**
  - **Plan-vs-reality:** `technical-design.md §3.2` sketches `recomputeCharacter { characterId } -> { success, character, error? }` as illustrative pseudocode. The real, existing Tauri command-naming convention in this codebase is snake_case matching the Rust fn name exactly (confirmed via `apps/desktop/src/boundary/*.ts`'s real `invoke('export_character', ...)`/`invoke('save_character_portrait', ...)` call sites) — so the registered command is `recompute_character`, not literal camelCase `recomputeCharacter`. The response DTO is a new, self-contained `RecomputeCharacterResponse { success, character: Option<CharacterSnapshotDto>, error: Option<String> }` — a fresh type, not a reuse of `character_hub.rs`'s `CreateCharacterResponse` enum, because this module cannot see `character_hub.rs`'s private mapping helpers (`map_snapshot_dto`, `resolve_character_root`, etc. are not `pub`) and touching `character_hub.rs` to widen their visibility is outside this cycle's granted file scope. `CharacterSnapshotDto` carries every derived stat the real `PilotSnapshot` view model exposes today (BAB, base saves, baseline melee attack bonus, baseline armor class, total saves); "skill points" / "caster level" named in the acceptance text's illustrative list are not yet distinct typed fields on `PilotSnapshot` (caster-level arithmetic today only exists as `ComputationExplanation` records inside `pilot_compute.rs`, not a summary field) — the DTO carries the real fields that exist rather than inventing placeholder ones for fields that don't, consistent with the no-stub doctrine.
  - **Semantics chosen:** `recompute_character` is a read-and-recompute operation only — it loads the saved `CharacterInput` from disk via `SavedCharacterStore::load`, runs it through the real engine (`build_pilot_headless_receipt` + `PilotViewModel`), and returns the freshly-derived stats. It does **not** mutate or re-save the envelope; that combined load-mutate-recompute-resave shape belongs to `character_hub.rs`'s existing mutate-ops (`level_up_character`, etc.) and to this Epic's own `appendToCharacter`/`reSaveCharacter` (Criteria 7.1/7.3, not yet landed as of this cycle). This is the standalone "refresh" capability the Tauri command surface was missing — the gap Criterion 7.2's RED text names.
  - **Never-fabricates-success guard:** if the saved build does not reach `HeadlessReceiptStatus::Computed` (e.g. a level outside the engine's currently supported range), the response is `success: false, character: None, error: Some("character_not_computable: <real diagnostic messages>")` — never a fabricated `Computed`-shaped payload. A missing saved character returns `success: false, error: Some("character_not_found")`.
- **Discovery forwards:** none this cycle.
- **Next-cycle plan:** Criterion 7.1 (`appendToCharacter`) is the natural next Epic 7 cycle — it can reuse `recompute_character_at_root`'s pattern (or call the now-registered `recompute_character` command directly from a future frontend flow) for its own load-append-recompute-re-save round trip. Criterion 7.3 (`reSaveCharacter`) should follow, since its revision-id-increment logic is the piece `recompute_character` deliberately left alone.

## RED -> GREEN evidence

**RED (real, captured):** the first test run against the initial fixture (a *reduced* Fighter loadout missing the `item:shield`/`power_attack` equipment-selection entries `pilot_compute.rs`'s `unmet_combat_posture_conditions` requires verbatim) produced 2 genuine failures for the intended reason — the build fell back to `Blocked` instead of `Computed`, so `recompute_character_at_root` correctly refused to fabricate success:

```
running 4 tests
test characterHub::recompute_character::tests::recompute_character_at_root_returns_an_honest_failure_when_the_saved_build_is_blocked ... ok
test characterHub::recompute_character::tests::recompute_character_at_root_returns_character_not_found_for_a_missing_saved_character ... ok
test characterHub::recompute_character::tests::recompute_character_at_root_returns_fresh_derived_stats_for_a_saved_character ... FAILED
test characterHub::recompute_character::tests::recompute_character_at_root_reflects_a_level_bump_that_landed_on_disk_without_a_recompute ... FAILED

---- ...returns_fresh_derived_stats_for_a_saved_character stdout ----
thread '...' panicked at src/characterHub/recomputeCharacter.rs:294:9:
a Computed build must report success

---- ...reflects_a_level_bump_that_landed_on_disk_without_a_recompute stdout ----
thread '...' panicked at src/characterHub/recomputeCharacter.rs:332:9:
assertion failed: response.success

test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 113 filtered out; finished in 0.11s
```

Root cause confirmed by reading `unmet_combat_posture_conditions` in `src/rules_core/pilot_compute.rs`: the combat-baseline pillar requires the exact deterministic equipment posture (longsword equipped, chain shirt equipped, shield absent, power_attack item selected-inactive) that `character_hub::compose_character_input` uses — the test fixture had only 2 of the 4 required equipment-selection entries.

**GREEN (real, captured):** widening the test fixture to the full 4-entry equipment posture (matching `compose_character_input` exactly) made all 4 tests pass:

```
running 4 tests
test characterHub::recompute_character::tests::recompute_character_at_root_returns_character_not_found_for_a_missing_saved_character ... ok
test characterHub::recompute_character::tests::recompute_character_at_root_returns_fresh_derived_stats_for_a_saved_character ... ok
test characterHub::recompute_character::tests::recompute_character_at_root_returns_an_honest_failure_when_the_saved_build_is_blocked ... ok
test characterHub::recompute_character::tests::recompute_character_at_root_reflects_a_level_bump_that_landed_on_disk_without_a_recompute ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 113 filtered out; finished in 0.00s
```

**Structural RED (pre-cycle absence):** before this cycle, no `recompute_character`/`recomputeCharacter` symbol existed anywhere in `apps/desktop/src-tauri/src/` (confirmed via `grep -rn "recompute" apps/desktop/src-tauri/src/` returning zero matches against `origin/tranche/5-2` prior to this commit) and no such command was registered in `main.rs`'s `invoke_handler!` list — the Tauri command surface had no way to independently ask for a saved character's freshly-recomputed derived stats, exactly the gap this criterion closes.

**Full regression suites (real, captured):**

```
$ cargo test --locked --bin codex-desktop      (apps/desktop/src-tauri)
test result: ok. 117 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test --locked --lib                    (root codex crate)
test result: ok. 154 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

`cargo build --locked` (src-tauri) also succeeds cleanly (7m37s cold build after a fresh worktree checkout; only 3 pre-existing `dead_code` warnings on `character_hub.rs`'s unrelated `SavedCharacterMutationOp*` table, not introduced by this cycle).
