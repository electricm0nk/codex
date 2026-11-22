# Cycle 3.3 — Epic 3 Character Hub as Hub of Hubs / Criterion 3.3

- **Card ID:** (reported alongside; see kanban section of the dispatch report)
- **Commit SHA:** (pushed to `tranche/5-3`; see final report — recorded after push per `loop-instruction.md §5`)
- **Files touched:**
  - NEW `apps/desktop/src-tauri/src/stub_adapter.rs` (`StubAdapter` — full `RuleSystemAdapter` implementation + inline `#[cfg(test)] mod tests`, two tests)
  - `apps/desktop/src-tauri/src/main.rs` — one module-declaration line (`mod stub_adapter;`), alphabetically ordered between `mod spell_catalog;` and `mod support_state_matrix_bridge;`
  - `governance/wired-integration-stubs-registry.md` — new entry `0002` (required registry entry, landed in the same commit per `cycles/3_3.md` GREEN)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (this cycle's own diff, `git diff --cached` scoped to the two touched Rust files — no `sd[0-9]+_`/`t_[0-9a-f]{8,}` hits). Note: the standing bundle-wide `BASE_BRANCH...HEAD` command (unscoped by this cycle) also surfaces two pre-existing `-mod sd13_support_state_matrix;` / `-use sd13_support_state_matrix::{...}` deletion lines in `main.rs` — these are criterion 1.1's already-landed, already-audited rename (`sd13_support_state_matrix.rs` → `support_state_matrix_bridge.rs`, commit `62c4785`, 404 documented hits) surfacing because `main.rs` is a shared cumulative file across the whole `tranche/5-3` branch, not new violations introduced by this cycle.
- **Wired-integration audit result:** `OK_NO_WOULD_STRINGS` as a **registered exception**, not a violation. This cycle's own diff (`git diff --cached` scoped to `stub_adapter.rs` + `main.rs`) hits the `not yet implemented` bucket exactly where expected — every hit is inside `stub_adapter.rs` (the doc comment, the `would_render_message()` builder, every trait-method arm that surfaces the message, and the two inline tests asserting the exact string). Zero hits in `main.rs` (module-declaration line only) or anywhere else. Per `cycles/3_3.md` RE-AUDIT: this is the doctrine's own named exception, and it is only GREEN because `governance/wired-integration-stubs-registry.md` entry `0002` lands in this same commit with the operator-granted justification (`epic-breakdown.md` §3.3 verbatim: "the future-system rollout is operator-pinned").
- **Acceptance criterion:** Criterion 3.3 — `StubAdapter` future-system stub (`epic-breakdown.md` Epic 3): new `apps/desktop/src-tauri/src/stub_adapter.rs`; returns "Would render for system X; not yet implemented" results; requires a `governance/wired-integration-stubs-registry.md` entry with operator-granted justification; `parallel: yes`.
- **Status:** complete

## RED → GREEN evidence

**RED** (`cargo check --tests --bin codex-desktop`, before `StubAdapter` was defined — the file's `use` block + `#[cfg(test)] mod tests` referencing `StubAdapter::new(...)` was present with the struct/impl block temporarily removed to prove the RED state):

```
error[E0433]: cannot find type `StubAdapter` in this scope
  --> src/stub_adapter.rs:83:60
   |
83 |         let adapter: Box<dyn RuleSystemAdapter> = Box::new(StubAdapter::new("starfinder"));
   |                                                            ^^^^^^^^^^^ use of undeclared type `StubAdapter`

error[E0433]: cannot find type `StubAdapter` in this scope
   --> src/stub_adapter.rs:144:23
    |
144 |         let adapter = StubAdapter::new("fifth-edition");
    |                       ^^^^^^^^^^^ use of undeclared type `StubAdapter`

error: could not compile `codex-desktop` (bin "codex-desktop" test) due to 2 previous errors
```

This is the RED the cycle doc names: "A test asserting a non-PF1 `rule_system_id` resolves to an adapter fails — no stub adapter exists."

**GREEN** (`StubAdapter` implemented for the full seven-method `RuleSystemAdapter` surface; `cargo test --bin codex-desktop stub_adapter::`):

```
running 2 tests
test stub_adapter::tests::stub_adapter_message_names_its_own_system_id ... ok
test stub_adapter::tests::non_pf1_rule_system_id_resolves_to_a_stub_adapter ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 126 filtered out; finished in 0.00s
```

Full crate suite re-run after GREEN (`cargo test --bin codex-desktop`, no path filter): **128 passed; 0 failed** — the two new tests plus every pre-existing test in `codex-desktop` (126, matching criterion 3.1's own post-GREEN count).

## Design notes

- **Trait-shape fidelity to `rule_system_adapter.rs` (criterion 3.1).** `StubAdapter` implements all seven methods (`rule_system_id`, `chassis_resolve`, `level_up`, `save_character`, `append_to_character`, `recompute`, `list_saved_characters`, `load_saved_character`) with no redesign of the trait's own signatures.
- **`rule_system_id(&self) -> &'static str` constrains the id to a compile-time literal.** `StubAdapter` stores `system_id: &'static str` (not an owned `String`) so it satisfies the trait's `&'static str` return type without `Box::leak` or other unsafe tricks — callers construct one `StubAdapter` per not-yet-built system with a literal id (e.g. `StubAdapter::new("starfinder")`), the same pattern `Pf1Adapter` will use for `"pf1"`. This mirrors how criterion 3.4's future registry is expected to key adapters by a fixed set of known `rule_system_id` literals, not arbitrary runtime strings.
- **Every method reports "not yet implemented" through whatever channel its own return type honestly provides — never fabricated data:**
  - `save_character`, `append_to_character`, `list_saved_characters`, `load_saved_character` (all `Result<_, String>`) → `Err("Would render for system {id}; not yet implemented")`.
  - `recompute` (`RecomputeCharacterResponse { success, character, error }`, not `Result`) → `success: false, character: None, error: Some(...)`, the same honest-failure shape the real Pf1 free functions already use for real failures.
  - `chassis_resolve` (`PilotBaseChassisComputation`, not `Result`, no `Default` impl) → every numeric field is zeroed exactly the way `compute_pilot_base_chassis`'s own "unsupported chassis" arm already zeros an unsupported case (verified by reading that function before writing this one, `pilot_compute.rs:4573-4741`), with a `claim_blocking: true` `ComputationDiagnostic` carrying the stub message — not a fabricated chassis.
  - `level_up` (`LevelUpPlan`, derives `Default`) → `LevelUpPlan::default()`. `LevelUpPlan` carries no message-capable field of its own (out of this cycle's file-touch grant to add one), so the honest report is the same empty default `level_up::compute_level_up_grants` itself already returns for an unrecognized dispatch — consistent with that struct's own "compose, don't fabricate" doc comment, not a silent gap introduced here.
- **The registry entry is the doctrine-mandated pairing, not an afterthought.** `governance/wired-integration-stubs-registry.md` entry `0002` lands in the same commit as this file per `cycles/3_3.md`'s explicit non-self-healable warning ("Without the registry entry, the §6 dual-audit failure is NOT self-healable for this cycle"). The entry quotes the operator-pinned justification verbatim from `epic-breakdown.md` §3.3 / `cycles/3_3.md`, since the future-system rollout directive is authored directly into those bundle docs rather than a separate live operator utterance for this specific cycle.
- **Test input construction reuses the crate's own composition entrypoint.** `stub_character_input()` calls `character_hub::compose_character_input(&CreateCharacterRequest { ... })` — the same real, already-landed function `rule_system_adapter.rs`'s own trait test uses — rather than hand-constructing a `CharacterInput` (which has no `Default` impl and several non-trivial nested fields). This keeps the stub's test honest: it exercises the trait's real method signatures against a genuinely-built input, not an ad hoc fixture.
- **No files outside the grant were touched.** `rule_system_adapter.rs`, `character_hub.rs`, and the `characterHub::*` command modules are read-only inputs to this cycle (their public types are imported, never edited). This keeps the cycle low-conflict-risk for the sibling parallel cycles (3.1 already landed; 3.2 Pf1Adapter extraction, 3.5 UI panel still pending).

## Discovery forwards

None. Criterion 3.3's own scope (the stub implementation + its registry entry) is fully self-contained; no new `## DISCOVERED` items surfaced during this cycle.

## Next-cycle plan

- **Criterion 3.2 (Pf1Adapter extraction, parallel sibling):** unaffected by this cycle — no shared files touched beyond the trait definition (already landed at 3.1) and `main.rs`'s module-declaration list (additive, alphabetically ordered, low conflict risk on rebase).
- **Criterion 3.4 (Tauri command routing, depends on 3.1–3.3):** once 3.2 also lands, 3.4 wires `append_to_character.rs`, `recompute_character.rs`, `re_save_character.rs` to accept `rule_system_id: String` and dispatch through a small registry keyed by `rule_system_id()` — `Pf1Adapter` for `"pf1"`, `StubAdapter::new(<literal>)` for any future system id this registry pre-declares. `StubAdapter` itself needs no further changes for 3.4 to consume it.
- **Future real-system cycles:** per the registry entry's own "Remediation cycle" field, the moment a real adapter for a given `rule_system_id` lands, criterion 3.4's registry routes that id to the real implementation instead of `StubAdapter` — this stub is superseded per-system, not bulk-removed.
