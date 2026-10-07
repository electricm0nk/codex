# Cycle E4a.MC — E4a / adversarial merge check

- **Card ID:** E4a.MC   **Model:** opus (Opus 5.5)   **RETRO_ACTOR:** sd37-e4a-mc   **Attempt:** 1 of 3
- **Commit SHA:** the `docs(sd37,e4a.mc)` commit carrying this receipt (`git log -1 --format=%H -- docs/release/SD-37-starfinder-1e/artifacts/epic_4a/E4a.MC_cycle_receipt.md`).   **Base SHA:** `ae5c7f3c3a77d02626b3933e6d538e818c0e703d` (= `origin/tranche/17` at start; wrong-base control `BASE_OK`; `git rebase origin/tranche/17` → "Current branch tranche/17 is up to date.")   **Oracle SHA:** `7f818006e3…` (no corpus figure quoted)
- **Epic base used for cross-epic parity:** `ddcb983967` (`655f476834^`, the tree before E4a.1), built in a throwaway worktree with its own `CARGO_TARGET_DIR` (removed afterwards), harness `pf_catalog_dump_hash.rs` copied in and declared `#[cfg(test)] mod` in `main.rs` (that tree only, never committed).
- **Files touched:** this receipt, `artifacts/epic_4a/E4a.MC_logs/**`, `kanban.md` (E4a.MC row), `progress.md` (cycle-log row + DISCOVERED row), `docs/retro/events/sd37-e4a-mc.jsonl`, `docs/retro/events/codex-sd37.jsonl` (verify.sh's derived verification event). No code, no data.
- **Acceptance criterion (verbatim from epic-breakdown.md):** E4a.MC — Adversarial merge check. Acceptance: Opus re-runs E4a.2–E4a.4's commands on `origin/tranche/17` and plants one mutation (one table JSON row edited) that must flip a PF hash or the round-trip test.
- **Status:** **partial.** Everything that has landed holds (below). The epic is **not** complete: the kanban row's own dependency **E4a.4a** (six table generators still write compiled `.rs` into the removed `src/rules_core/rules_tables`) is `ready`, not run. This merge check cannot cover a card that has not landed.

## Why partial (the remainder, by sub-cause)

| Sub-cause | Items | Evidence |
|---|---|---|
| Dependency not landed: E4a.4a (kanban `Depends on: E4a.4, E4a.4a`; E4a.4a status `ready`) | 1 card = 6 generators still writing into the removed directory | `grep -rnE '"src/rules_core/rules_tables/\|f"src/rules_core/rules_tables/' crates/codex-ingest/src/bin scripts --include='*.rs' --include='*.py' \| awk -F: '{print $1}' \| sort -u` → `gen_equipment_gap_tables.rs`, `gen_feat_gap_tables.rs`, `ingest_class_spell_levels_arg.rs`, `ingest_spells.rs`, `scripts/transcribe_companion_tables.py`, `scripts/transcribe_monster_tables.py` (+ `scripts/tests/test_transcribe_monster_tables.py`, a test) — 6 generators, matches E4a.4's figure |
| **Total remaining** | **1 card (6 generators)** | |

Once E4a.4a lands, the re-run of this card needs only: E4a.4a's generator re-runs (package byte-identical), the golden test, and the PF pair. Everything else below is on a tree E4a.4a must leave byte-identical in `data/rules_tables`.

## RED / GREEN

Merge check: no code change, so no RED→GREEN. The "RED" side of this card is the planted mutations (below), each of which must turn a gate red.

## Re-run acceptance commands on `origin/tranche/17` = `ae5c7f3c3a` (`E4a.MC_logs/acceptance.log`)

| Card | Command | Result | Second implementation |
|---|---|---|---|
| E4a.2 | `for r in src crates apps/desktop/src-tauri tests; do grep -rlE 'rules_tables::' $r --include='*.rs' \| awk '!/src\/rules_core\/rules_tables\//' \| awk 'END{print NR}'; done` | `0 0 0 0` | Python `os.walk` (skip `target`/`node_modules`) → `[0, 0, 0, 0]` |
| E4a.3 | `test ! -e src/rules_core/rules_tables \|\| grep -rcF '.lst' src/rules_core/rules_tables \| awk …` | no output (module absent), exit 0 | — |
| E4a.3 | `grep -rcF '.lst' data/rules_tables \| awk -F: '{s+=$2} END{print s+0}'` | `0` | Python over 281 files → `0` lines |
| E4a.4 | `test ! -e src/rules_core/rules_tables \|\| find src/rules_core/rules_tables -name '*.rs' \| awk 'END{exit NR>0}'` | exit 0 | Python `os.walk` → 0 `.rs` |
| E4a.4 | PF pair, E1.4 harness (`PF_SEED_RENDER_OUT=… cargo test --locked -j 8 --bins pf_seed_render_hash -- --test-threads=8 --nocapture`, apps/desktop/src-tauri) | Aldric `1d830682210a0e6e216093a3125593edf1f25539f38547fc832a56c050a5a569`, Elowen `8d1a711c8519b62b489f47f097d683922568a645f4daabc9c95933c1b2b100f2` (`pf-base.log`, `pf-head/sha256.txt`) | `diff` vs E4a.4's `pf-render-before/sha256.txt` → empty |
| E4a.4 | 15 PF catalog dumps (`PF_CATALOG_DUMP_OUT=… cargo test --locked -j 8 --bins pf_catalog_dump_hash …`) | 15 hashes (`catalog-head/sha256.txt`) | `diff` vs E4a.4's `catalog-before/sha256.txt` → empty |
| E4a.4 | Bestiary 1 | harness: `beastiary1_keys=46 book_b1=326 all=1243` | Python over `list_monster_catalog.json`: all 1243, `Counter(book)['B1']` 326, keys 46 |
| E4a.1 | loader round trip + package tests (`cargo test --locked -j 8 --lib -- --test-threads=8 golden_digest rules_data_package`) | 11 passed, 0 failed (`clean-golden-and-package.log`) | — |
| E4a.1 | `bash scripts/verify.sh -j 8 --only tauri-resources-tracked --only rules-schema-check --only pi-sweep` | `RESULT: PASS`; pi-sweep `12 hits … 11 baseline rows, 0 package stamps disagree` (`verify-stages.log`) | — |
| E4a.1/4 | `cargo run --locked -j 8 --bin rules_tables_package -- --check` | `tables=281 rows_bytes=13291625 pi_files=9 verdict=PASS` (= E4a.3/E4a.4's bytes) | `find data/rules_tables -name '*.json' \| awk 'END{print NR}'` → 281 |
| E4a.3/4 | `cargo test --locked -j 8 --test rules_tables_citation_burn_down --test rules_catalog_feature_facts_read_the_package --test pi_table_sweep` | 3 / 1 / 8 passed, 0 failed (`epic-tests.log`) | — |
| drift | `git diff --stat d268b87734 HEAD -- src/rules_core/rules_catalog/golden_digests.txt data/rules_tables`; `git diff --stat ea24b4b887 HEAD -- data/rules_tables` | both empty: the digests are the ones written from the compiled module, and the package is byte-for-byte E4a.3's | — |

## Planted mutations (each restored; `mutation-before.sha` = `mutation-after.sha`)

| # | Mutation (one JSON row field, `data/rules_tables/crb/class_tables/class_tables.json`) | Gate | Result |
|---|---|---|---|
| M-a | Fighter level 3 `will_save` 1 → 7 (line 674) | golden digests | **FAILED** `every_catalog_table_view_and_lookup_matches_its_golden_digest`, naming `crb/class_tables/class_tables` (`mutation-golden-and-package.log`) |
| M-a | same | PF pair | **did not move**: Aldric `1d830682…a569` (`pf-mutation.log`, `pf-mutation-fighter/sha256.txt`) — see finding F1 |
| M-a | same | round trip | passed (expected: the round trip checks the file against itself, not against an outside oracle; the golden digests are that oracle since E4a.4) |
| M-b | Wizard level 5 `will_save` 4 → 9 (line 1650; a different field from E4a.2's BAB plant) | PF pair | **Elowen moved** `8d1a711c…00f2` → `28c3cefde71665038af7b9a64f6cab971eb751b059e234a51c38b2de82a131c8`; Aldric unchanged; the diff is exactly the 4 Will lines (base 4→9, total 5→10, two explanation details) (`pf-mutation-wizard-will.log`, `pf-mutation-wizard/sha256.txt`) |

The criterion (one row edit flips a PF hash or the oracle test) holds twice: M-a flips the golden oracle, M-b flips Elowen's hash.

## Cross-epic parity (pre-E4a.1 `ddcb983967` vs `ae5c7f3c3a`) — beyond the receipts

E4a.4 compared catalogs only across its own commits (`d268b87734` → `a0998838a1`), by which time E4a.2 had already re-pointed the importers and E4a.3 had rewritten citations. This card built the tree from before E4a.1 and dumped both harnesses there (`pf-epicbase.log`).

- PF pair: Aldric and Elowen hashes identical to HEAD (`diff pf-epicbase/sha256.txt pf-head/sha256.txt` → empty).
- Catalogs: **14 of 15** byte-identical. `list_monster_catalog` differs: epic base `e159c2b3…4d29`, HEAD `5edf5b81…f2ea`; `diff` of the pretty JSON → **2 changed lines** (`monster-catalog-epicbase-vs-head.diff`): the Crocodile `groundingNote` citations `b1_abilities_race.lst:244` / `.lst:248` → `b1_abilities_race:244` / `:248`. `git log -S'b1_abilities_race.lst:248' ddcb983967..HEAD` → `ea24b4b887` (E4a.3). This is the extension strip decisions.md §19 mandates (target 0 `.lst` in the shipped package), not a parity defect; but it is a user-visible catalog text change that no E4a receipt names (finding F2).
- Bestiary 1 at epic base: `beastiary1_keys=46 book_b1=326 all=1243` = HEAD.

## Findings

- **F1 (reported, not fixed; outside E4a's criterion):** Aldric's Fighter base Will save is not read from the package's `crb/class_tables` row (M-a moved the golden digest but not Aldric's render; Aldric's explanation reads "Fighter level 3 base Will save = 1" with no table citation, while Elowen's cites the class table). PF Fighter saves have a second source; the package row for Fighter is guarded only by the golden test. Changing which source the Fighter render reads would move nothing today and is not an E4a criterion. Retro note emitted.
- **F2 (reported):** the one cross-epic catalog byte-change above. Retro note emitted.
- **F3 (confirmed, owned):** the six generators (E4a.4a) — the remainder.
- Elowen's render still prints `rules_tables::crb::class_tables::class_tables()` as a provenance string (the 44 sites E4a.2/E4a.4 kept under SD-i); the module no longer exists. Operator-owned rename (SD-i), already recorded.

## Figures

- Package: 281 table files (`find data/rules_tables -name '*.json' | awk 'END{print NR}'`; Python agrees) — 13,291,625 row bytes (`rules_tables_package --check`).
- Catalog parity across the epic: 14 / 15 dumps identical — predicate: `sha256` of each zero-argument PF catalog command's pretty JSON equal on `ddcb983967` and `ae5c7f3c3a`; `diff` of the two `sha256.txt` files → 1 differing name; `cmp` over the 15 JSON files → 1 differs.
- **Raw row-count output:** `0 0 0 0`; `0`; `0`; exit 0; Python `[0, 0, 0, 0]`, `0`, `0`.

## Build scope verified

Desktop `apps/desktop/src-tauri` bin tests (the two harnesses) on HEAD and on the epic base; root `--lib` filtered (11 tests) and three root integration tests; `rules_tables_package --check`; three `verify.sh` stages. **Not run:** the full root / codex-ingest / desktop sweeps. This card changed no code; E4a.4 ran them on `a0998838a1` (root 297 suites, ingest 1,828/0, desktop 700/0), and E7.2 owns the next widest-scope pass (R-V). Logs: `artifacts/epic_4a/E4a.MC_logs/`.

## Audits

- This card's diff is docs/logs only (no `src`, `crates`, `apps` path): every audit vacuously prints `OK_*` (run after the commit; see the commit's push record in `progress.md`).
- **Epic-range audit** (`ddcb983967...ae5c7f3c3a`, `epic-audit.log`): identifier audit 2 hits, token audit 1 hit, `OK_NO_NOOP_HANDLERS`, `OK_NO_MOCK_LEAKS`, `OK_NO_WOULD_STRINGS`. All 3 hits are lines the pre-epic tree already carried (`git grep -cF "<line>" ddcb983967 -- src` = 1 and `HEAD -- src` = 1 for each): two doc comments citing `tests/sd24_equipment_coverage_audit.rs`, one "never a placeholder string" doc line. Moves, not new tags or stubs.

## Structural diff

Not a converter cycle; no converter change.

## Seed deltas

- Aldric (Fighter 3): unchanged — `1d830682…a569` on HEAD = epic base = E1.4.
- Elowen (Wizard 5 + Fireball): unchanged — `8d1a711c…00f2` on HEAD = epic base = E1.4.
- SF-Soldier-3 / SF-Mystic-5 / SF-Technomancer-5 / SF-Envoy-3: not re-rendered by this card. Direct check: 0 of 15 `sf_*.rs` files (`src/rules_core/pilot_compute/sf_*.rs`, `apps/desktop/src-tauri/src/sf_*.rs`) name `rules_catalog` or `rules_data_package`; this is a direct-import check, not a transitive one. E4a.2/E4a.3 `cmp`'d their printed lines against E6.MC.

## retro.py summary since epic start (`scripts/retro.py summary --since 2026-10-06T16:38:00-04:00 --json`, `E4a.MC_logs/retro-summary.json`, before this card's events)

15 events: correction 3, deferral 2, incident 1, rework 3, verification 6. Corrections by corrector: sd37-e4a-2 ×2, sd37-e4a-3 ×1; repeat subjects 0. Incidents: `verify-without-cargo-target-dir` ×1, recurring 0. Rework causes 3, each distinct. Verification: 6 runs, 1 failed (clippy). **No key fires more than once** (R-W: no new control owed). Deferrals open 2: the E4a.2 one (types/impls/formulas must be placed before "no `.rs` left") was done by E4a.4 but never marked — this card emitted a `resolution` for it; the E4a.4 one (generators) stays open = E4a.4a.

## Does not cover

- **E4a.4a** (not landed): no generator was run; whether the six can regenerate the package byte-identically is untested.
- The full root / ingest / desktop suites were not re-run here (E4a.4's results stand; E7.2 re-runs).
- Catalog commands that take arguments (`list_feats`, `list_spells`, `list_equipment`, creation commands) and `list_reference_library_catalog` are not dumped, on either tree.
- Only two mutations, both in `crb/class_tables`; the golden test's coverage of the other 280 files rests on E4a.4's 429-row manifest, not re-planted here.
- The packaged app (resource-root binding) was not built or launched.
- The SF seeds were not rendered.

## Safe defaults taken

- None from §12.1. Process choice: run the full adversarial check on the landed cards now rather than decline outright, so the re-run after E4a.4a is narrow. Alternative not taken: return `declined` with `owned_by=E4a.4a` and no check.

## Retro events emitted

`docs/retro/events/sd37-e4a-mc.jsonl`: resolution ×1 (the E4a.2 deferral), incident ×1 (`dispatch-before-discovered-dependency`: this card was dispatched with E4a.4a `ready`), note ×2 (F1, F2).

## Next-cycle plan

Dispatch E4a.4a. Then re-run E4a.MC (attempt 2): E4a.4a's generator re-runs leave `data/rules_tables` byte-identical (`git diff --stat` empty, `rules_tables_package --check` PASS), the golden test, the PF pair, and the generator grep → 0.
