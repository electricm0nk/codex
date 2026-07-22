# Cycle 3.4 — Epic 3 Character Hub as Hub of Hubs / Criterion 3.4

- **Card ID:** (reported alongside; see kanban section of the dispatch report)
- **Commit SHA:** (pushed to `tranche/5-3`; see final report — recorded after push per `loop-instruction.md §5`)
- **Files touched:**
  - `apps/desktop/src-tauri/src/characterHub/appendToCharacter.rs` — added `rule_system_id: String` to `AppendToCharacterRequest`; added `resolve_rule_system_adapter` + `append_to_character_via_rule_system`; `#[tauri::command] append_to_character` now dispatches through the trait; two new tests.
  - `apps/desktop/src-tauri/src/characterHub/recomputeCharacter.rs` — same shape: `rule_system_id: String` on `RecomputeCharacterRequest`; `resolve_rule_system_adapter` + `recompute_character_via_rule_system`; command wrapper updated; two new tests.
  - `apps/desktop/src-tauri/src/characterHub/reSaveCharacter.rs` — same shape: `rule_system_id: String` on `ReSaveCharacterRequest`; `resolve_rule_system_adapter` + `re_save_character_via_rule_system`; command wrapper updated; two new tests.
  - `governance/wired-integration-stubs-registry.md` — widened entry `0002`'s file/line and audit-grep-impact scope to also cover these three files' `resolve_rule_system_adapter` doc comments and `*_routes_unknown_id_to_stub_adapter` tests (same governed stub, new legitimate call sites — not a new stub, so no new numbered entry).
  - `apps/desktop/src-tauri/src/main.rs` — **not touched.** The cycle doc's file-touch grant named `append_to_character.rs`/`recompute_character.rs`/`re_save_character.rs` at `apps/desktop/src-tauri/src/`, but those paths do not exist there — the real files are `apps/desktop/src-tauri/src/characterHub/{appendToCharacter,recomputeCharacter,reSaveCharacter}.rs` (confirmed by `find`; `character_hub.rs` vs `characterHub/` naming split is documented at `loop-instruction.md §4`). Touched the real files. `main.rs`'s `tauri::command` registration list and `use` imports needed no changes — the command function signatures (`fn append_to_character(app: AppHandle, request: RequestDto)`) are unchanged; the new `rule_system_id` field lives inside each existing request DTO, not as a new function parameter.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped to the three touched Rust files).
- **Wired-integration audit result:** documented exception, not `OK_NO_TOKENS` verbatim — `not yet implemented` hits inside each file's `resolve_rule_system_adapter` doc comment and `*_routes_unknown_id_to_stub_adapter` test assertions. Every hit is a reference to `StubAdapter`'s already-registered message (governance entry `0002`, originally landed at criterion 3.3), now reached from three new real call sites rather than a new stub being introduced. Entry `0002` is widened in this same commit to name these exact files/hit-sites as covered, per this repo's own precedent (`cycles/3_3.md` GREEN: "must be paired with a governance/wired-integration-stubs-registry.md entry in the SAME commit"). No other forbidden-pattern token (`STUB`, `MOCK`, `placeholder`, `todo`, `fixme`, `hack`) appears anywhere in the scoped diff.
- **Acceptance criterion:** Criterion 3.4 — Tauri command surface routes through the hub-of-hubs (Epic 3): all three commands (`appendToCharacter`, `recomputeCharacter`, `reSaveCharacter`) accept a `rule_system_id: String` argument and dispatch through the `RuleSystemAdapter` trait; existing behavior for PF1 unchanged; an unknown `rule_system_id` routes to `StubAdapter`.
- **Status:** complete

## RED → GREEN evidence

**RED** (`cargo check --tests -p codex-desktop`, `appendToCharacter.rs`'s two new tests written first, referencing `append_to_character_via_rule_system` before it existed):

```
error[E0425]: cannot find function `append_to_character_via_rule_system` in this scope
error[E0425]: cannot find function `append_to_character_via_rule_system` in this scope
error[E0425]: cannot find function `append_to_character_via_rule_system` in this scope
error: could not compile `codex-desktop` (bin "codex-desktop" test) due to 3 previous errors
```

(3 hits: the two new test bodies plus the updated `#[tauri::command]` wrapper's own call site, all written before the function was defined.)

This is the RED the cycle doc names: "A test invoking each command with `rule_system_id: 'pf1'` and asserting dispatch through the `RuleSystemAdapter` trait fails — commands call PF1 logic directly today." The same dispatch-seam pattern (`resolve_rule_system_adapter` + `<command>_via_rule_system`, each command's wrapper switched to call it, two new tests: one asserting `"pf1"` dispatches through the trait and matches the pre-existing direct-call result exactly, one asserting an unknown id routes to `StubAdapter`'s honest error) was then applied identically to `recomputeCharacter.rs` and `reSaveCharacter.rs` — the RED state for those two files is the same missing-function shape, demonstrated once here rather than three times since it is the identical mechanical gap in all three.

**GREEN** (`cargo test -p codex-desktop appendToCharacter` / `recomputeCharacter` / `reSaveCharacter`, after implementation):

```
running 6 tests   (appendToCharacter)
test characterHub::appendToCharacter::tests::append_to_character_via_rule_system_dispatches_pf1_through_the_trait ... ok
test characterHub::appendToCharacter::tests::append_to_character_via_rule_system_routes_unknown_id_to_stub_adapter ... ok
... (4 pre-existing tests, unchanged, still pass)
test result: ok. 6 passed; 0 failed

running 6 tests   (recomputeCharacter)
test characterHub::recomputeCharacter::tests::recompute_character_via_rule_system_dispatches_pf1_through_the_trait ... ok
test characterHub::recomputeCharacter::tests::recompute_character_via_rule_system_routes_unknown_id_to_stub_adapter ... ok
... (4 pre-existing tests, unchanged, still pass)
test result: ok. 6 passed; 0 failed

running 5 tests   (reSaveCharacter)
test characterHub::reSaveCharacter::tests::re_save_character_via_rule_system_dispatches_pf1_through_the_trait ... ok
test characterHub::reSaveCharacter::tests::re_save_character_via_rule_system_routes_unknown_id_to_stub_adapter ... ok
... (3 pre-existing tests, unchanged, still pass)
test result: ok. 5 passed; 0 failed
```

Full crate suite re-run after GREEN (`cargo test -p codex-desktop`, no path filter): **140 passed; 0 failed** (baseline before this cycle: 134 passed — confirmed by running the same command against the pre-cycle commit; +6 new tests, 0 regressions).

## Design notes

- **Existing behavior for PF1 unchanged, proven, not just asserted.** Each `<command>_via_rule_system("pf1", ...)` dispatch is proven identical to the pre-existing direct `<command>_at_root(...)` call — `recompute_character_via_rule_system`'s pf1 test asserts full `PartialEq` equality between the two response values; `appendToCharacter`/`reSaveCharacter`'s pf1 tests assert the same on-disk persisted outcome (equipment count / revision_id) the direct call produces. This is possible because `Pf1Adapter`'s trait methods (criterion 3.2) are themselves thin wrappers over the exact same `_at_root` functions — routing "through the trait" for `"pf1"` is provably a no-op behavior change, only an indirection change.
- **Pre-existing inline tests untouched and still pass.** The `_at_root` functions' own signatures were not changed; every test that called them directly before this cycle (`append_to_character_at_root_appends_real_item_and_persists_when_computed`, `re_save_character_at_root_increments_revision_on_each_call`, `recompute_character_at_root_returns_fresh_derived_stats_for_a_saved_character`, etc.) is untouched in this diff and still passes, satisfying the cycle doc's GREEN clause verbatim.
- **`Box::leak` for the unknown-id path, not a pre-declared literal set.** `StubAdapter::new` requires `&'static str` (criterion 3.3's own design, out of this cycle's file-touch grant to change), but the Tauri command's `rule_system_id` is a runtime `String` from the wire. Rather than hardcoding a fixed set of anticipated future-system literals (which the stub-adapter cycle's own "Next-cycle plan" speculated about but which would silently swallow any `rule_system_id` string *not* on that hardcoded list — the opposite of "an unknown rule_system_id routes to StubAdapter"), each `resolve_rule_system_adapter` leaks the caller's string once per call via `Box::leak(other.to_owned().into_boxed_str())` to genuinely satisfy `'static` for *any* runtime string, not just a pre-declared few. This is the same `Box::leak`-to-`'static` idiom already used in this crate at `corpus_fixtures.rs` and in `codex::rules_core::equipment_resolver` / `damage_total.rs`. The leak is bounded by how many *distinct* not-yet-supported `rule_system_id` strings ever get dispatched in a process's lifetime (an operator/test-driven, low-cardinality set today — there is still no frontend caller at all, see below), not by call volume; flagged here explicitly as a judgment call rather than silently applying the pattern.
- **Duplication across the three files is deliberate, matching established crate precedent.** `resolve_rule_system_adapter` is defined once per file (not shared via a new module) because the file-touch grant for this cycle does not include `rule_system_adapter.rs`/`pf1_adapter.rs`/`stub_adapter.rs` (only the three command files, per `cycles/3_4.md`). `pf1_adapter.rs`'s own `next_mutation_revision_id` (criterion 3.2) already established this "parallel, functionally-identical implementation rather than a shared one — not a behavior fork, the same rule applied in two call sites" pattern in this codebase for the identical reason.
- **Register A3 carry-forward confirmed still true.** `grep -rn "appendToCharacter|recomputeCharacter|reSaveCharacter|append_to_character|recompute_character|re_save_character" apps/desktop/src/` returns nothing — zero frontend callers exist for any of the three commands, exactly as `cycles/3_4.md`'s carry-forward note states. Routing them through the trait is necessary but not sufficient; the caller gap remains criterion 3.5's to close.
- **File-touch grant path correction.** `cycles/3_4.md` names `apps/desktop/src-tauri/src/append_to_character.rs` etc. (flat, snake_case); the real files live at `apps/desktop/src-tauri/src/characterHub/{appendToCharacter,recomputeCharacter,reSaveCharacter}.rs` (nested, camelCase module dir with camelCase filenames, `#[allow(non_snake_case)] mod characterHub;` in `main.rs`). This mirrors `loop-instruction.md §4`'s own note about the `character_hub.rs` vs `characterHub/` split and is treated as a path-verification correction, not a scope expansion — same three logical commands, same criterion.

## Discovery forwards

None new. This cycle's own scope (routing three existing commands through the already-landed trait) is fully self-contained.

## Next-cycle plan

- **Criterion 3.5 (UI layer, `apps/desktop/src/characterHub/`):** per register A3's carry-forward note, these three commands still have zero frontend callers — no `boundary/*.ts` wrapper, no `invoke()` call site. 3.5 (or a follow-on cycle in its scope) needs to (a) add `rule_system_id` to whatever wire-boundary wrappers get built for these three commands, defaulting to `"pf1"` for the only rule system the UI currently supports, and (b) decide whether/how an operator-facing rule-system picker ever sends a non-`"pf1"` id at all before `StubAdapter`'s honest-failure path is reachable from the UI.
- **Future real-system cycles:** the moment a real adapter for some `rule_system_id` other than `"pf1"` lands, each file's `resolve_rule_system_adapter` match needs a new arm for that literal (mirroring the existing `"pf1" => Box::new(Pf1Adapter)` arm) — three files to update in lockstep, not one, since the grant kept the resolver un-shared.
