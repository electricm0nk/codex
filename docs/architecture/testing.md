# Testing

> Scope: the full verification command set for this repo, test conventions, the fixture grammar, and how to run corpus-gated tests — this file doubles as the "how do I verify my change" runbook.
> Last verified: 2026-07-20 against ef9012bf5de8
> Maintenance: updated at SD closure — see [README.md](./README.md) §Maintenance contract

## Quick reference: what to run for a given change

| You changed... | Run this |
|---|---|
| `src/**/*.rs` (rules-core) | `cargo test --locked` from repo root |
| `apps/desktop/src-tauri/src/**/*.rs` | `cd apps/desktop/src-tauri && cargo test --locked` |
| `apps/desktop/src/**/*.ts(x)` | `cd apps/desktop && npm run typecheck && npm test` |
| any `.rs` you're about to commit | `cargo clippy --locked --tests -- -D warnings` |
| `scripts/release/*.py`, `scripts/release/*.sh` | the matching standalone script under [Standalone scripts](#standalone-scripts) |
| `tools/release/*.py` | the matching `tools/release/test_*.py` |
| a release-notes/manifest doctrine change | `scripts/tranche/tests/test_validate_tranche_notes.py` and `tests/sd16-e5-f1/test_branch_promotion_guard.sh` |

None of the standalone scripts below are wired into `npm test` or `cargo test` — each is invoked directly, and none of them appear in `apps/desktop/scripts/run-tests.mjs`'s glob (`*.test.ts` only) or in a Cargo test target.

## The full command set

### Root Rust crate

```
cargo test --locked
```
Runs the workspace's default test targets: unit tests inside `src/` plus every integration test file under `tests/*.rs`. As of this verification there are **426** files matching `tests/*.rs` (`ls tests/*.rs | wc -l`). Files are named by originating slice — `ge06_*`, `ge08_*`, `sd13_*`, `sd17_*`, `sd19_*`, `sd20_*`, `sd22_*`, `golden_case_*`, `pcc_*`, `character_*` — one behavior per file (see [Test conventions](#test-conventions)). The crate itself (`Cargo.toml:1-4`, name `codex`) has no `[workspace]` table, so this is a single standalone crate, not a Cargo workspace.

```
cargo clippy --locked --tests -- -D warnings
```
Lints the crate including its test targets, failing the build on any warning. Verified to run clean against the current tree (`Finished 'dev' profile ... target(s)`).

### Desktop shell (Tauri/Rust side)

```
cd apps/desktop/src-tauri && cargo test --locked
```
`apps/desktop/src-tauri` is a separate crate (`apps/desktop/src-tauri/Cargo.toml`, name `codex-desktop`) that depends on the root crate via a path dependency (`codex = { path = "../../.." }`). Its tests are **inline `#[cfg(test)]` modules**, not separate `tests/*.rs` files — 12 source files currently carry one: `sd13_support_state_matrix.rs`, `sd19_spell_catalog.rs`, `sd19_corpus.rs`, `sd19_race_catalog.rs`, `sd19_equipment_catalog.rs`, `main.rs`, `character_hub.rs`, `sd16_browser_handoff.rs`, `campaign_drive.rs`, `ge08_workbench.rs`, `sd19_class_catalog.rs`, `update/transaction.rs` (confirmed via `grep -rl "#\[cfg(test)\]" apps/desktop/src-tauri/src/`).

### Desktop frontend (TypeScript)

```
cd apps/desktop && npm run typecheck
```
Runs `tsc --noEmit` (`apps/desktop/package.json` `scripts.typecheck`).
**Currently red on a clean checkout**: after a fresh `npm ci`, `tsc --noEmit`
exits non-zero — `@types/node` is not a declared dependency
(`apps/desktop/package.json` lists no `@types/node`), so the
`node:fs`/`node:path`/`node:url` imports in
`apps/desktop/src/sd16/feedback/docCommentHygiene.test.ts` and the
`apps/desktop/src/sd21/` / `apps/desktop/src/sd22/` doc-guard test files
fail type resolution (TS2307),
and `apps/desktop/src/sd16/update/fetch.ts` /
`apps/desktop/src/sd16/update/index.ts` carry duplicate-identifier
declarations (TS2440/TS2300). No CI workflow typechecks `tranche/5` pushes
(see [release-pipeline.md](./release-pipeline.md)), but the same red state
is already blocking publishes from `develop`: the three most recent
`publish-tester-release.yml` runs (2026-07-19 through 2026-07-20) all failed
at the `test` job's `Typecheck frontend` step, so every downstream publish
job was skipped and no release shipped from those pushes.

```
cd apps/desktop && npm test
```
Runs `node scripts/run-tests.mjs` (`apps/desktop/package.json` `scripts.test`) — **not vitest**. `apps/desktop/scripts/run-tests.mjs` recursively globs every `src/**/*.test.ts` file, then `spawnSync`s each one individually through `tsx` (`apps/desktop/scripts/run-tests.mjs:1-45`):

```js
// Runs every self-executing src/**/*.test.ts script via tsx and reports a
// summary. Each test file exits non-zero on its first failed assertion.
```

Each test file is a self-executing script (no test-framework `describe`/`it` wrapper); it asserts directly and exits non-zero on the first failed assertion. There are currently **48** matching files (`find apps/desktop/src -iname "*.test.ts" | wc -l`). The runner prints `PASS <file>` / `FAIL <file>` per file and a `<n>/<total> test files passed.` summary line, exiting non-zero if any failed.

### Standalone scripts

None of these are auto-discovered by `cargo test` or `npm test`. Run each directly:

| Command | What it verifies |
|---|---|
| `python3 tools/release/test_check_release_manifest.py` | `tools/release/check_release_manifest.py` (legacy manifest schema + tranche coherence). `unittest`, 9 test methods on a `TmpRepo` fixture class; invokes the validator as a subprocess. Verified: `Ran 9 tests ... OK`. |
| `python3 tools/release/test_check_release_manifest_against_dev_schema.py` | The dev-schema shim (`check_release_manifest_against_dev_schema.py`). `unittest`, 2 test methods. Verified: `Ran 2 tests ... OK`. |
| `python3 tools/release/test_emit_channel_index.py` | `tools/release/emit_channel_index.py`. `unittest`, 3 test methods; one (`test_happy_path_alpha`) does a local `import jsonschema` scoped inside the test function specifically so the module can be collected without `jsonschema` installed (`tools/release/test_emit_channel_index.py:106`, `# local import so the RED phase doesn't need jsonschema`). Verified: `Ran 3 tests ... OK`. |
| `bash scripts/release/test-promotion-gates.test.sh` | `scripts/release/promote-alpha-to-beta.sh` and `promote-beta-to-stable.sh` against a stubbed `gh` on `PATH`; asserts every individual gate (`G_ALPHA_RELEASE_EXISTS`, `G_NOTES_VALIDATED`, `G_MANIFEST_VALID`, `G_NO_RELEASE_BLOCKERS`, `G_SHELL_CONSUMED_ALPHA`, and the beta/stable equivalents) fails and passes correctly, and that neither script ever calls `gh pr create` (final assertion greps the full call log). Verified: `ALL PROMOTION-GATE TESTS PASSED`. |
| `bash scripts/release/__tests__/test-write-release-manifest.test.sh` | `scripts/release/write_release_manifest.py` + `validate_manifest.py` round-trip: a good fixture validates, a malformed AppImage sha256 is rejected, and writer output validates. Verified: `SD16-E4-F3b: bash self-test PASS`. |
| `python3 scripts/tranche/tests/test_validate_tranche_notes.py` | `scripts/tranche/validate-tranche-notes.py` — manifest field validation, `release_notes_path` containment, and release-notes section/order/non-empty checks. `unittest`, 9 test methods. Verified: `Ran 9 tests ... OK`. |
| `bash tests/sd16-e5-f1/test_branch_promotion_guard.sh` | `tools/ci/branch-promotion-guard.sh`'s `verify_promotion_source` — the exact function the `allow-only-*` GitHub Actions workflows execute at PR time. Verified: `branch promotion guard tests: 11 passed, 0 failed`. |
| `python3 scripts/release/check_promotion_evidence.py --self-test` | The promotion-evidence gate's own built-in RED-GREEN test harness (`_run_self_tests`, ~30 `_t_*` functions covering release-notes validation, manifest binding, PR-body keys, evidence parsing, and full lane evaluation). This is also the first step `promotion-gates.yml` runs on every PR before evaluating a real one. Verified: `SELF-TEST PASSED`. |

All `jsonschema`-based validators (`validate_manifest.py`, `emit_channel_index.py`, `check_release_manifest_against_dev_schema.py`, and the test files that exercise them) need the `jsonschema` pip package; CI pins `jsonschema==4.21.1` (e.g. `.github/workflows/publish-tester-release.yml:324`, `.github/workflows/check-release-manifest.yml:82`). It is already importable in this workspace (`python3 -c "import jsonschema"` exits 0).

## Test conventions

- **Integration tests live flat under `tests/*.rs`**, one behavior per file, named by originating slice — e.g. `tests/ge06_pilot_base_computation.rs`, `tests/sd13_barbarian_level10_progression.rs`, `tests/sd20_equipment_effects_parity.rs`, `tests/golden_case_fixture_schema.rs`. This provenance-naming pattern makes it possible to `cargo test --test <name>` a single slice's behavior in isolation, and to `grep` the test suite by originating SD/GE without any test-registry file.
- **RED → GREEN is the expected posture.** `AGENTS.md`'s non-negotiable rule 1 requires writing or updating a failing test before changing production code, confirming it fails for the intended reason, then implementing the smallest change to pass (`AGENTS.md:34-39`). Several of the standalone scripts document this explicitly in their own headers — e.g. `scripts/release/__tests__/test-write-release-manifest.test.sh:6-9` ("Writes the test FIRST, runs it RED, then implementation, then GREEN, then refactor"), and `tools/release/test_emit_channel_index.py:106`'s local-import comment exists specifically so the RED phase (test written, `jsonschema` not yet installed) still collects.
- **Sibling preservation**: `tests/*.rs`'s one-file-per-slice naming is what makes this norm mechanically checkable — running `cargo test --locked` after a change touches every prior slice's file, not just the one you're editing, so a regression in an older row (e.g. an SD-13 barbarian fixture breaking because a shared rules-table changed) surfaces immediately rather than only at the next full-suite run. `AGENTS.md`'s rule 1 reinforces this at the process level: "Run the relevant test set after each meaningful change" (`AGENTS.md:38`).

## The fixture grammar (`tests/fixtures/rules_core/`)

There are **246** files under `tests/fixtures/rules_core/` (`ls tests/fixtures/rules_core/ | wc -l`), each a flat `key=value` deterministic-input file. The loader is `load_character_input_fixture` in `src/rules_core/character_input.rs:168-190`:

```rust
pub fn load_character_input_fixture(input: &str) -> CharacterInputLoadResult {
    let mut parsed = ParsedFixture::default();

    for raw_line in input.lines() {
        let line = raw_line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            parsed.diagnostics.push(diagnostic(
                "fixture_line",
                format!("invalid character input line missing '=': {raw_line}"),
            ));
            continue;
        };

        apply_fixture_field(key.trim(), value.trim(), &mut parsed);
    }
    ...
```

Grammar rules, read directly off the parser (`character_input.rs:168-233`):

- **One `key=value` pair per line.** No nesting, no indentation-sensitivity, no sections.
- **Blank lines and `#`-prefixed lines are comments**, skipped entirely — fixtures use this heavily for prose provenance/rationale (see the two example fixtures below).
- **Lists are expressed by repeating the key**, not by a list syntax. `apply_fixture_field` (`character_input.rs:218-233`) routes each key to a handler; keys that model multi-valued state (`feat`, `skill`, `equipment`, `choice`, `spell`, `provenance`) each push onto a `Vec` rather than overwrite, so a fixture with five rage-power choices has five separate `choice=` lines. (`ability` also repeats — six lines per fixture — but its handler `apply_ability_score` (`character_input.rs:261-308`) assigns each line to the named field of a fixed-shape `AbilityScores` struct rather than pushing onto a `Vec`.)
- **Unknown keys are a diagnostic, not a silent ignore** — the `unknown` match arm at `character_input.rs:233` records a diagnostic rather than dropping the line.
- **Naming convention**: `pf1_<race>_<class>_level<N>_<slice>_deterministic_input.txt`, e.g. `tests/fixtures/rules_core/pf1_dwarf_fighter_level1_sd13_deterministic_input.txt`. Some fixtures append a further qualifier before `_deterministic_input`, e.g. `pf1_human_barbarian_level10_sd13_five_rage_powers_deterministic_input.txt`.

Real example (`tests/fixtures/rules_core/pf1_dwarf_fighter_level1_sd13_deterministic_input.txt`):
```
case_id=pf1-crb-dwarf-fighter-level1
source_package_id=pf1.core_rulebook
race_id=race:dwarf
class_level=class:fighter:1
ability=strength:14
ability=dexterity:12
ability=constitution:16
ability=intelligence:10
ability=wisdom:10
ability=charisma:6
provenance=programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md
```

A second example showing the repeated-key list idiom for `choice=` (`tests/fixtures/rules_core/pf1_human_barbarian_level10_sd13_five_rage_powers_deterministic_input.txt`):
```
case_id=pf1-crb-human-barbarian-level10-five-rage-powers
...
choice=choice:level_1_character_feat:feat:power_attack
choice=choice:human_bonus_feat:feat:dodge
choice=choice:barbarian_rage_power:power:animal_fury
choice=choice:barbarian_rage_power_2:power:clear_mind
choice=choice:barbarian_rage_power_3:power:guarded_stance
choice=choice:barbarian_rage_power_4:power:knockback
choice=choice:barbarian_rage_power_5:power:powerful_blow
choice=choice:human_ability_bonus:ability:strength
```

Fixtures are explicit about what they do *not* claim: comment blocks routinely state which downstream compute paths stay "claim-blocked" for that fixture (e.g. the dwarf-fighter fixture's header notes the Fighter combat baseline stays blocked because the bounded compute path is grounded only for `race:human`). This is chosen input only — no derived/computed values are ever present in a fixture file.

## Corpus-gated tests

Some integration tests validate parsing against the real PCGen data corpus (a separate checkout, not vendored into this repo) rather than the hand-written fixtures above. They read `PCGEN_CORPUS_ROOT` and are gated by **two different mechanisms that coexist in the codebase**:

**Pattern A — `#[ignore]`-attributed, hard-fails if run without the env var set.** This is the majority pattern across the `sd22_*_resolves.rs` and several `sd17_*` files:

```rust
#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
fn resolves_real_core_rulebook_pcc_from_local_pcgen_corpus() {
    let corpus_root = PathBuf::from(
        std::env::var("PCGEN_CORPUS_ROOT")
            .expect("PCGEN_CORPUS_ROOT must point at a local pcgen/data checkout"),
    );
    ...
```
(`tests/sd17_a_include_graph.rs:275-281`)

A plain `cargo test --locked` reports these as `... ignored` and does not execute them. To run them with the corpus present, per the exact commands recorded in `docs/release/SD-22/artifacts/acg/class_arcanist_cycle_receipt.md` and sibling receipts:
```
PCGEN_CORPUS_ROOT=/path/to/pcgen/data cargo test --locked --test sd22_acg_class_arcanist_resolves -- --include-ignored
```
`--include-ignored` runs both normal and `#[ignore]`d tests in that binary; `--ignored` (also seen in the receipts, e.g. `cargo test --locked --test sd17_b_monster_stat_block -- --ignored`) runs *only* the ignored ones.

**Pattern B — plain `#[test]` (no `#[ignore]`), runtime file-existence check with a graceful `eprintln!` skip.** Used in `tests/sd17_b5_equipment.rs` and `tests/sd17_b_metadata_kinds.rs`:

```rust
#[test]
fn real_corpus_cr_equip_arms_armor_parses_with_line_numbers_preserved() {
    let corpus_root = std::env::var("PCGEN_CORPUS_ROOT")
        .unwrap_or_else(|_| "/home/ubuntu/workspace/repos/pcgen/data".to_string());
    let path = std::path::PathBuf::from(corpus_root)
        .join("pathfinder/paizo/roleplaying_game/core_rulebook/cr_equip_arms_armor.lst");
    if !path.is_file() {
        eprintln!(
            "skipping: real cr_equip_arms_armor.lst not at {}",
            path.display()
        );
        return;
    }
    ...
```
(`tests/sd17_b5_equipment.rs:481-497`)

These run under a plain `cargo test --locked` with no extra flags — no `PCGEN_CORPUS_ROOT` needed for the test to pass; it just self-skips (prints to stderr and returns early — counted as a pass by `cargo test`) if the corpus isn't found at the env var or the hardcoded default path.

A third variant (`tests/sd17_b_races_and_abilities.rs`) wraps the same env-or-default-then-exists check in an `Option`-returning helper (`corpus_root()`, lines 27-33) and early-returns with `eprintln!` at each call site — functionally the same graceful-skip behavior as Pattern B, factored differently.

**When documenting a new corpus-gated test, match the pattern of the file you're adding to** — `sd22_*` and most `sd17_*` class/spell/equipment files use `#[ignore]` (Pattern A); the `sd17_b5_equipment.rs` / `sd17_b_metadata_kinds.rs` / `sd17_b_races_and_abilities.rs` files use the graceful-skip variants (Patterns B/C). Both patterns default to the sandbox path `/home/ubuntu/workspace/repos/pcgen/data` when `PCGEN_CORPUS_ROOT` is unset, before failing (Pattern A) or skipping (Patterns B/C).

## Desktop test support

Three shared modules under `apps/desktop/src/testSupport/` back the `*.test.ts` suite:

- **`apps/desktop/src/testSupport/asserts.ts`** — the two primitives every self-executing test file uses: `assertEqual<T>(actual, expected, message)` and `assert(condition, message)`, both throwing `Error` on failure (which is what makes a `tsx`-run file exit non-zero). Extracted because, per its own doc comment, "every test file previously carried its own identical copy of these" (`asserts.ts:1-6`).
- **`apps/desktop/src/testSupport/makeSurface.ts`** — `makeSurface(overrides = {})` returns one canonical, fully-populated `Sd11TesterWorkbenchSurface` fixture, then shallow-spreads `overrides` on top. Its doc comment explains why this exists: "Several test files previously carried their own copies; the copies drifted when the SD-12 release-truth bridge added new required auto-captured evidence fields, which silently broke the submit-flow tests" (`makeSurface.ts:3-10`). Tests that need to vary a nested field (e.g. `status.build`) pass a whole replacement nested object, since the spread is shallow.
- **`apps/desktop/src/testSupport/makeCharacterSummary.ts`** — same single-canonical-fixture pattern for `CharacterSummaryDto`, explicitly "mirroring `makeSurface.ts`'s pattern" (`makeCharacterSummary.ts:6-7`).

## Wire fixtures (`tests/fixtures/wire/`)

All eight files live under `tests/fixtures/wire/sd20/`: `boundary_contract_parity.json`, `damage_total_parity.json`, `equipment_effects_parity.json`, `feat_prereqs_parity.json`, `human_fighter_level_1_tabletop.json`, `level_up_parity.json`, `skill_allocation_parity.json`, `spellbook_parity.json`.

Each is read from disk at test-run time by exactly one Rust integration test, via a `CARGO_MANIFEST_DIR`-relative path and a hand-rolled JSON parser (no `serde_json` dependency in these test files). Example (`tests/sd20_equipment_effects_parity.rs:236-241`):
```rust
fn load_fixture() -> Json {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/fixtures/wire/sd20/equipment_effects_parity.json");
    let raw = fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read fixture at {path:?}: {err}"));
    parse_json(&raw)
}
```
The other seven fixtures have the identical loader shape in their own test file (`tests/sd20_contract_boundary_parity.rs`, `tests/sd20_tabletop_readiness_integration.rs`, `tests/sd20_spellbook_parity.rs`, `tests/sd20_level_up_parity.rs`, `tests/sd20_damage_total_parity.rs`, `tests/sd20_skill_allocation_parity.rs`, `tests/sd20_feat_prereqs_parity.rs`), each pointing at its own JSON file. These fixtures are consumed exclusively by `tests/*.rs` — nothing under `src/` or `apps/desktop/src-tauri/src/` reads them.

## Update fixtures (`tests/fixtures/update/`)

Nine files, documented by their own README (`tests/fixtures/update/README.md`): `alpha.json`, `alpha.full-manifest.json`, `beta.json`, `stable.json`, `channel-index.bad-tag.json`, `release-manifest.bad-path.json`, `update-manifest.json`, `update-manifest.missing-signature-allowed.json`, plus the README itself. The README states this set is "consumed by BOTH the Python release-tooling lane (via `python -m jsonschema -i`) and the TypeScript shell-parser lane (via `parseChannelIndex.ts` / `parseUpdateManifest.ts`)" (`tests/fixtures/update/README.md:3-6`), and carries a fixture-to-AV-id mapping table naming which schema rule each fixture exercises (positive vs. negative case) — e.g. `alpha.full-manifest.json` is a NEGATIVE fixture proving `additionalProperties: false` rejects a channel-index that smuggles manifest fields onto it.

Unlike the wire fixtures, these are **not read from disk by any test at run time**. Their two live connections are:
- `apps/desktop/tsconfig.json`'s `include` array literally globs them (`"../../tests/fixtures/update/**/*.json"`, `apps/desktop/tsconfig.json:23`) — this affects the TypeScript project's compilation/typecheck scope, not test execution.
- The TS parser tests (`apps/desktop/src/sd16/update/parseChannelIndex.test.ts`, `parseUpdateManifest.test.ts`) inline byte-for-byte copies of these fixtures as JSON string literals rather than reading the files, and document a manual duplication discipline: if you edit a fixture here, the Python lane and the TS lane must both be re-run and their verdicts must agree (`tests/fixtures/update/README.md:24-32`, "Duplication discipline"). This is a documented-but-manual parity contract, not an automated one.

## Related docs

- [release-pipeline.md](./release-pipeline.md) — the CI workflows and scripts these tests gate (promotion gates, manifest schema validators, publish pipeline).
- [conventions.md](./conventions.md) — repo-wide coding and TDD conventions this file cross-references.
- [overview.md](./overview.md) — system-level architecture context.
- [status.md](./status.md) — current SD/tranche state.
