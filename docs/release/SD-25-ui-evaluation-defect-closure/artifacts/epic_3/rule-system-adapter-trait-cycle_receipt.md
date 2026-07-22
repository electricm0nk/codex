# Cycle 3.1 — Epic 3 Character Hub as Hub of Hubs / Criterion 3.1

- **Card ID:** (reported alongside; see kanban section of the dispatch report)
- **Commit SHA:** (pushed to `tranche/5-3`; see final report — recorded after push per `loop-instruction.md §5`)
- **Files touched:**
  - NEW `apps/desktop/src-tauri/src/rule_system_adapter.rs` (493 lines: trait definition + inline `#[cfg(test)] mod tests`)
  - `apps/desktop/src-tauri/src/main.rs` — one module-declaration line (`mod rule_system_adapter;`), alphabetically ordered between `mod race_catalog;` and `mod spell_catalog;`
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (diff scoped to the two touched files, per this cycle's file-touch grant)
- **Wired-integration audit result:** `OK_NO_TOKENS` (diff scoped to the two touched files). One self-healed collision during authoring: a doc comment read "no placeholder return values" (describing what the inline test deliberately avoids) and tripped the `placeholder` token; reworded in-place to "every returned value comes from real compute/persistence, none invented" before the final audit — no `## DISCOVERED`/exception entry needed since it resolved inline (same self-heal class as SD-24 register item A9's `hack` collision).
- **Acceptance criterion:** Criterion 3.1 — `RuleSystemAdapter` trait definition (`epic-breakdown.md` Epic 3): file `apps/desktop/src-tauri/src/rule_system_adapter.rs` (new); `parallel: yes`; methods `chassis_resolve`, `level_up`, `save_character`, `append_to_character`, `recompute`, `list_saved_characters`, `load_saved_character`.
- **Status:** complete

## RED → GREEN evidence

**RED** (`cargo check --tests -p codex-desktop`, before the trait was defined — file contained only a `#[cfg(test)] mod tests` block referencing `super::RuleSystemAdapter` in an `impl` and a `Box<dyn super::RuleSystemAdapter>` construction, no trait definition present):

```
error[E0405]: cannot find trait `RuleSystemAdapter` in module `super`
 --> src/rule_system_adapter.rs:13:17
   |
13 |     impl super::RuleSystemAdapter for Probe {
   |                 ^^^^^^^^^^^^^^^^^ not found in `super`

error[E0405]: cannot find trait `RuleSystemAdapter` in module `super`
 --> src/rule_system_adapter.rs:21:37
   |
21 |         let adapter: Box<dyn super::RuleSystemAdapter> = Box::new(Probe);
   |                                     ^^^^^^^^^^^^^^^^^ not found in `super`

error: could not compile `codex-desktop` (bin "codex-desktop" test) due to 2 previous errors
```

**GREEN** (full trait defined, object-safe — no generic methods — with the exact seven-method surface `epic-breakdown.md` §3.1 names; `cargo test rule_system_adapter -p codex-desktop`):

```
running 1 test
test rule_system_adapter::tests::dyn_rule_system_adapter_constructs_and_dispatches_every_method ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 125 filtered out; finished in 0.01s
```

Full crate suite re-run after GREEN (`cargo test -p codex-desktop`, no path filter): **126 passed; 0 failed** — the new test plus every pre-existing test in `codex-desktop`.

## Design notes

- **Object safety is deliberate.** No generic (`impl Trait` / `<T>`) methods on `RuleSystemAdapter` — the cycle's RED text requires constructing a `dyn RuleSystemAdapter`, and criterion 3.4 (Tauri command-surface routing) needs to hold `Box<dyn RuleSystemAdapter>`/`&dyn RuleSystemAdapter` and dispatch on a runtime `rule_system_id: String`. `save_character`'s mutate-style cousins in `character_hub.rs` (`mutate_saved_character_at_root`) take `impl FnOnce(&mut CharacterInput)`, which is not object-safe; this trait instead mirrors the narrower, already-object-safe `_at_root` free-function shapes (`re_save_character_at_root`, `append_to_character_at_root`, `recompute_character_at_root`) that criterion 3.2's `Pf1Adapter` will wrap.
- **Every method signature is grounded in a real, already-landed free function**, verified by reading `character_hub.rs` and `characterHub::{appendToCharacter,recomputeCharacter,reSaveCharacter}` before drafting the trait, not invented from the epic-breakdown's one-line method list alone:
  - `chassis_resolve` ↔ `pilot_compute::compute_pilot_base_chassis`
  - `level_up` ↔ `level_up::compute_level_up_grants` (widened per the A2 carry-forward below)
  - `save_character` ↔ `characterHub::reSaveCharacter::re_save_character_at_root`
  - `append_to_character` ↔ `characterHub::appendToCharacter::append_to_character_at_root`
  - `recompute` ↔ `characterHub::recomputeCharacter::recompute_character_at_root`
  - `list_saved_characters` ↔ `SavedCharacterStore::list_all` + `character_hub`'s own summary projection
  - `load_saved_character` ↔ `SavedCharacterStore::load` + `character_hub`'s own load/compute/project pipeline
- **SD-24 carry-forward register A2 (shared with criterion 3.2):** `compute_level_up_grants(character, from_level, to_level)` dispatches on a single implicit class read off `character.chosen.class_levels`'s `[class_level]` single-element slice pattern — any multiclass mix falls through to the wildcard arm and returns `LevelUpPlan::default()`. This trait's `level_up` takes an explicit `deltas: &[ClassLevelDelta]` (new `pub struct ClassLevelDelta { class_id, from_level, to_level }`) instead of inferring one implied class/level pair, so a caller can state a per-class multiclass delta unambiguously. `compute_level_up_grants`'s own single-class dispatch gap is **not** fixed by this cycle (out of this criterion's file-touch grant — `level_up.rs` is not in scope) — that widening/routing is explicitly deferred to criterion 3.2's `Pf1Adapter` implementation, which this trait's signature now makes expressible. The inline test's `TestPf1Delegate::level_up` demonstrates the intended composition shape (iterating `deltas` and merging each sub-plan) end-to-end, though today it still only grounds a single-class character per delta (unchanged `compute_level_up_grants` per-delta behavior) until 3.2 lands the real multiclass routing.
- **The inline test (`TestPf1Delegate`) is a real-delegating proof, not a mechanical stub.** All seven methods are exercised through a `Box<dyn RuleSystemAdapter>`, and every method body calls this crate's real functions against a real on-disk `SavedCharacterStore` fixture and the real compute engine (`build_pilot_headless_receipt`, `PilotViewModel`, `compute_pilot_with_corpus` against `corpus_fixtures::corpus_fixture_bundle()`) — no fabricated return values anywhere in the test path. `list_saved_characters`/`load_saved_character` re-derive `character_hub.rs`'s own (private, not part of this cycle's file-touch grant) summary/snapshot DTO mapping inline, since those helper functions are `fn`-private to `character_hub.rs` and this cycle cannot touch that file; criterion 3.2's `Pf1Adapter` extraction is where that projection logic is formally extracted and reused instead of re-derived a second time. The RED/GREEN assertions cover: chassis resolution reaching the grounded Fighter level-1 chassis (`base_attack_bonus == 1`), a real Fighter 1→2 level-up producing a non-empty automatic-features grant, a real disk load producing a `Some` snapshot, the seeded character appearing in a real disk listing, a validated equipment append against the real corpus (`"Dagger (Base)"`, the same known-good fixture key `appendToCharacter.rs`'s own tests use), a real recompute succeeding, and both branches of the real revision-conflict guard (`save_character` succeeding with the correct `expected_revision_id` and advancing `.rev.1` → `.rev.2`, then failing honestly with `error: "revision_conflict"` on a stale one).
- **Bug caught during authoring (test-only, not a code defect):** the first draft of the test derived `characters_root` as `root.parent()` of a bare `std::env::temp_dir().join(...)` path — since that path's parent is `/tmp` itself, `SavedCharacterStore::list_all` attempted to scan the entire shared `/tmp` directory tree on this multi-process dev host and hung (observed: >5 minutes wall time, low CPU — I/O-bound directory traversal, not a deadlock). Fixed by nesting the seeded character under a dedicated per-test `characters_root` tempdir (`characters_root.join(character_id)`), matching the real production layout (`characters_root/{character_id}/`). Confirmed fixed: full suite now runs in 0.13s. No production code was affected — this was purely a test-fixture-layout bug in the new inline test, caught and fixed before GREEN was recorded.
- **No files outside the grant were touched.** `character_hub.rs`, `characterHub/*.rs`, and `level_up.rs` are all read-only inputs to this cycle's design work — none were edited. This keeps the cycle genuinely low-conflict-risk for the sibling parallel cycles (3.2 Pf1Adapter extraction, 3.3 StubAdapter, 3.5 UI panel) per the dispatch brief's own note.

## Discovery forwards

None. This cycle's own scope fully absorbed carry-forward item A2's signature-design half; the routing/extraction half remains explicitly assigned to criterion 3.2 (already tracked in `epic-breakdown.md` and `sd24-carry-forward-register.md`, not a new discovery).

## Next-cycle plan

- **Criterion 3.2 (Pf1Adapter extraction):** move `character_hub.rs`'s Pf1-specific logic into `apps/desktop/src-tauri/src/pf1_adapter.rs` as a concrete `impl RuleSystemAdapter for Pf1Adapter`. Reuse this trait's exact method signatures verbatim (do not redesign them). Complete the A2 multiclass widening at `compute_level_up_grants`'s call site (or add a dedicated multiclass entry point) so `Pf1Adapter::level_up` genuinely composes a multiclass plan, not just accepts the shape. Also carries register A5 (fold revision-advancing into `mutate_saved_character_at_root`) — confirm with the operator per that item's own note.
- **Criterion 3.3 (StubAdapter):** implement `RuleSystemAdapter` for a future-system stub at `stub_adapter.rs`; register the "Would render for system X; not yet implemented" string in `governance/wired-integration-stubs-registry.md` in the same commit per that cycle's own doc — this is a pre-registered exception, not a dual-audit violation, but only if the registry entry lands together with the code.
- **Criterion 3.4 (Tauri command routing):** wire `append_to_character.rs`, `recompute_character.rs`, `re_save_character.rs` to accept `rule_system_id: String` and dispatch through `Box<dyn RuleSystemAdapter>` (a small registry keyed by `rule_system_id()` — Pf1Adapter today, StubAdapter for future systems). Depends on 3.1-3.3 all landing first (declared `parallel: no` in `epic-breakdown.md`).
