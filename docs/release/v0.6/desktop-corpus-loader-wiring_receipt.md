# Fix + infrastructure receipt — desktop corpus loader (Finding 4) and shield-as-weapon fix

- **Findings:** `docs/release/v0.6/book-agnostic-backend-gaps-scoping.md`, Finding 4 (desktop
  runtime corpus reachability) and a real, standalone fix found along the way.
- **Status:** loader infrastructure real, tested, committed. Live wiring attempted and reverted
  (Finding 5 blocks it — see scoping doc). One standalone engine fix landed.

## What landed

- `src/rules_core/corpus_loader.rs` (commit `cc6177ae`) — real, book-agnostic loader reading
  `data/corpus/<book>/equipment/**/*.json`, reconstructing `EquipmentRecord`-shaped values so
  every existing resolver works unchanged. 3 tests, including a full 6-book integration test
  (CRB Padded Armor's AC/max-dex + ARG Dogslicer's weight both resolve correctly).
- `apps/desktop/src-tauri/src/corpus_full.rs` — desktop-side wiring, proven correct in isolation.
  Merges real equipment (all 6 books) with the existing fixture's 2 real spells. 2 tests pass
  from within the actual `codex-desktop` binary.
- `src/rules_core/equipment_effects.rs`'s `is_weapon_record` — fixed to exclude shields from the
  single-weapon ambiguity check (a shield's own real shield-bash `DAMAGE:` token was being counted
  as a second wielded weapon). New regression test using the real Heavy Wooden Shield's token
  shape. **This fix is real, correct, and independent of Finding 5** — it stands regardless of
  when/how the loader gets wired live.

## What did not land, and why

Wiring `corpus_full::full_corpus_bundle()` into the 8 real desktop call sites
(`character_hub.rs` ×6, `pf1_adapter.rs` ×1, `characterHub/recomputeCharacter.rs` ×1) was built,
then **reverted** after it caused 36 real test failures. Root-caused precisely (not guessed): the
canonical "Human Fighter, Longsword + Chain Shirt + Shield" test character's `item:longsword`
legacy-namespace lookup resolved to the wrong of two colliding real corpus records (see scoping
doc Finding 5) once real corpus data reached `equipment_id_resolve` for the first time — a
pre-existing resolver-ambiguity gap, not a defect in the loader itself. Systematic corpus check
found 277 total collision groups (141 between two-or-more genuinely real items), too large and
too architecturally significant to patch under this session's time pressure.

**Operator directive: stop here.** Reverted the 3 call-site files to their original state
(`git checkout --`, clean revert, no partial state). Loader infrastructure kept — real, tested,
and ready to wire in once Finding 5 is resolved.

## Verification

- `cargo test --lib rules_core::corpus_loader` (root crate) — 3/3 passed.
- `cargo test corpus_full` (desktop crate, from within `codex-desktop`) — 2/2 passed.
- `cargo test --lib rules_core::equipment_effects` — 24/24 passed (23 pre-existing + 1 new shield
  regression test).
- Full desktop crate suite, post-revert: **271 passed / 0 failed** — confirms revert left a fully
  clean baseline, not a partial/broken state.
- Full root workspace suite: **5,386 passed / 2 failed** — both pre-existing, environment-path-
  dependent (`/home/ubuntu/workspace/...`), unchanged baseline every prior receipt this session
  has documented. The shield fix introduces zero regressions.
- Dual-audit gate (`identifier-discipline` + `wired-integration`) — clean.

## What's needed before the loader can go live

Per scoping doc Finding 5: a real design decision on `equipment_id_resolve`'s fallback-matching
ambiguity, sized at 277 real collision groups (136 fixable by preferring populated records over
thin duplicates; 141 needing the parenthetical qualifier threaded through, a bigger change). Not
started this session.
