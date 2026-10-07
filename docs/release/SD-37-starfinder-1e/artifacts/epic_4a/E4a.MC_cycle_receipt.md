# Cycle E4a.MC — E4a / adversarial merge check

- **Card ID:** E4a.MC   **Model:** opus (Opus 5.5)   **RETRO_ACTOR:** sd37-e4a-mc   **Attempt:** 3 of 3 (attempt 1 = partial on `ae5c7f3c3a`, receipt at `git show 949a8dbf62:docs/release/SD-37-starfinder-1e/artifacts/epic_4a/E4a.MC_cycle_receipt.md`; attempt 2 = declined, owned_by E4a.4a)
- **Commit SHA:** the `docs(sd37,e4a.mc)` attempt-3 commit carrying this receipt (`git log -1 --format=%H -- docs/release/SD-37-starfinder-1e/artifacts/epic_4a/E4a.MC_cycle_receipt.md`).   **Base SHA:** `291484aa093bc389b1883d73c321ad4af1fce4bb` (= `origin/tranche/17` at start, E4a.4a's commit; wrong-base control `BASE_OK`; `git fetch origin tranche/17 && git rebase origin/tranche/17` → "Current branch tranche/17 is up to date.")   **Oracle SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/fetch-pcgen-oracle.sh --check` inside the verify script; the generator re-runs read it)
- **Files touched:** this receipt; `artifacts/epic_4a/E4a.MC_attempt3_verify.sh` (the one-pass script); `artifacts/epic_4a/E4a.MC_logs/attempt3/**`; `kanban.md` (E4a.MC row); `progress.md` (cycle-log row); `docs/retro/events/sd37-e4a-mc.jsonl`. No code, no data (the package sha256 set is identical before and after, below).
- **Acceptance criterion (verbatim from epic-breakdown.md):** E4a.MC — Adversarial merge check. Opus re-runs E4a.2–E4a.4a's commands on `origin/tranche/17` and plants one mutation (one table JSON row edited) that must flip a PF hash or the round-trip test.
- **Status:** **complete.** Every E4a acceptance command holds on `291484aa09`, each with a second implementation; all three planted package-row mutations turn a gate red (golden digest, PF hash, loader round trip); one plant per generator is undone by that generator's own re-run (6 of 6); attempt 1's F1/F2 are closed below; desktop full suite 700 of 700 after one harness re-run (attributed below).

## RED / GREEN

Merge check: no code change, so no RED→GREEN. The RED side of this card is the planted mutations; each must turn a gate red.

## Acceptance commands re-run on `origin/tranche/17` = `291484aa09` (`E4a.MC_logs/attempt3/acceptance.log`, `acceptance_py.log`)

Commands copied from `epic-breakdown.md` with `\|` unescaped; the second column is an independent Python `os.walk` (skipping `target`/`node_modules`/`.git`).

| Card | Command (shell) | Shell result | Python result |
|---|---|---|---|
| E4a.2 | `for r in src crates apps/desktop/src-tauri tests; do grep -rlE 'rules_tables::' $r --include='*.rs' \| awk '!/src\/rules_core\/rules_tables\//' \| awk 'END{print NR}'; done` | `0 0 0 0` | `[0, 0, 0, 0]` |
| E4a.3 | `test ! -e src/rules_core/rules_tables \|\| grep -rcF '.lst' src/rules_core/rules_tables \| awk -F: '{s+=$2} END{print s+0}'` | no output, rc 0 (module absent) | `module_exists False` |
| E4a.3 | `grep -rcF '.lst' data/rules_tables \| awk -F: '{s+=$2} END{print s+0}'` | `0` | `package_lst_lines 0` |
| E4a.3 (reported only) | `grep -rcF '.lst' src/rules_core --exclude-dir=rules_tables \| awk -F: '{s+=$2} END{print s+0}'` | `1117` (= E4a.3's figure) | — |
| E4a.4 | `test ! -e src/rules_core/rules_tables \|\| find src/rules_core/rules_tables -name '*.rs' \| awk 'END{exit NR>0}'` | rc 0 | `rs_left 0` |
| E4a.4a | `grep -rnE '"src/rules_core/rules_tables/\|f"src/rules_core/rules_tables/' crates/codex-ingest/src/bin scripts --include='*.rs' --include='*.py' \| awk -F: '{print $1}' \| sort -u \| awk '!/scripts\/tests\//' \| awk 'END{print NR}'` | `0` | `generators 0 []` |
| E4a.4a | all six generators re-run against the pinned oracle: 4 Rust bins (`gen_feat_gap_tables`, `gen_equipment_gap_tables`, `ingest_spells`, `ingest_class_spell_levels_arg --verify --emit`) + `transcribe_monster_tables.py` × 22 books + `transcribe_companion_tables.py` × 16 books = 42 invocations (`proof_rerun.log`, exit 0) | `git status --porcelain --untracked-files=all data/rules_tables crates/codex-ingest/src/pcgen_import src/rules_core` → `status_lines=0` (`proof_status.log`) | package sha256 set (sorted `sha256sum` of every file under `data/rules_tables`, hashed) `f699c1d6…dd27`, 281 files, before = after (`package_sha_before.log`, `package_sha_after.log`) |
| E4a.4a | golden digest test | `every_catalog_table_view_and_lookup_matches_its_golden_digest` + `rules_data_package` filter: 11 passed, 0 failed (`golden_clean.log`) | — |
| E4a.4 / E4a.4a | PF hash pair (E1.4 harness, `PF_SEED_RENDER_OUT=… cargo test --locked -j 8 --bins pf_seed_render_hash`, `apps/desktop/src-tauri`) | Aldric `1d830682210a0e6e216093a3125593edf1f25539f38547fc832a56c050a5a569`, Elowen `8d1a711c8519b62b489f47f097d683922568a645f4daabc9c95933c1b2b100f2` (`pf_clean.log`, `pf-clean/sha256.txt`) | the desktop full run's own dump: same two hashes (`seed_lines/pf-sha256.txt`) |
| E4a.1 / E4a.4 | `cargo run --locked -j 8 -q -p codex --bin rules_tables_package -- --check` | `tables=281 rows_bytes=13291625 pi_files=9 verdict=PASS` (`package_check.log`) | 281 files in the sha set above |
| E4a.1 | `bash scripts/verify.sh -j 8 --only tauri-resources-tracked --only rules-schema-check --only pi-sweep --only clippy` | `RESULT: PASS` — resources 9, 3 schemas byte-equal, pi-sweep 12 hits / 11 baseline rows / 0 stamps disagree, clippy root 0 / desktop 0 / ingest 0 (`verify_stages.log`) | — |

Not re-run here (attempt 1 ran them on `ae5c7f3c3a` and E4a.4a changed no `src/`, `apps/` or `data/` file — `git diff --name-only 1a694aa4d5 291484aa09` lists only `crates/codex-ingest/**` (7), `scripts/**` (4) and `docs/**`): the 15 PF catalog dumps, Bestiary 1 46/326/1243, and the cross-epic build of the pre-E4a.1 tree. The package being byte-identical to E4a.4's (`--check` bytes 13,291,625 = attempt 1 = E4a.3/E4a.4) carries them.

## Generator plants: one per generator, each undone by its own re-run (`gen_plants.log`, `gen_plant_g4b.log`)

Each plant edits one string in a file E4a.4a's two plants did not touch; the generator is re-run alone; `git status --porcelain <file>` must be empty again.

| # | File | Generator | sha256 before → planted → after |
|---|---|---|---|
| G1 | `feat_gap_tables/ADVANCED_RACE_GUIDE_FEAT_GAP_ROWS.json` | `gen_feat_gap_tables` | `71fbc7e6b3bc` → `29c266da71b1` → `71fbc7e6b3bc` |
| G2 | `equipment_gap_tables/ADVANCED_CLASS_GUIDE_GAP_ROWS.json` | `gen_equipment_gap_tables` | `65ffab0f9a35` → `ad6cd83be6fb` → `65ffab0f9a35` |
| G3 | `advanced_race_guide/class_spell_levels/ARG_CLASS_SPELL_LEVELS.json` | `ingest_class_spell_levels_arg --emit` | `028b49eded15` → `cba6ae0d86a2` → `028b49eded15` |
| G4 | `advanced_race_guide/spell_list/SPELL_LIST.json` | `ingest_spells` | **not restored** — my plant was on a table `ingest_spells` does not write (below); reverted with `git checkout` |
| G4b | `ultimate_magic/spell_list/SPELL_LIST.json` | `ingest_spells` | `6118c98115fe` → `aec931c005d4` → `6118c98115fe` |
| G5 | `bestiary_3/monster_data/MONSTERS.json` | `transcribe_monster_tables.py bestiary_3` | `28df9ad53808` → `bf229ab4a6d8` → `28df9ad53808` |
| G6 | `bestiary_3/companion_data/COMPANIONS.json` | `transcribe_companion_tables.py bestiary_3` | `752964a009c3` → `46d4201afa91` → `752964a009c3` |

**G4 attribution:** `ingest_spells` writes 21 books (`grep -o '^ingest_spells \[[a-z_0-9]*\]' plant-G4-spells-ingest_spells.out | sort -u` → 21); the package has 27 `spell_list` tables (`ls -d data/rules_tables/*/spell_list | awk 'END{print NR}'` → 27). The 6 with no generator among the six: `acg advanced_race_guide apg bestiary_6 crb ultimate_intrigue` (`spell_list_generator_coverage.log`; every file naming them is a reader/registry under `src/rules_core`). They are E4a.4a's stated "Does not cover: generators outside the six"; the golden digests guard their content. Not an E4a defect; reported.

## Planted package-row mutations (each restored; package sha set before = after)

| # | Mutation (one JSON row field) | Gate | Result |
|---|---|---|---|
| M1 | `bestiary/monster_data/MONSTERS.json`, first row `name` → `PLANTED …` (`5d669f77…` → `0687c9cf…`) | golden digests | **FAILED** (rc 101), naming `bestiary/monster_data/MONSTERS`, `bestiary/monsters_static`, `bestiary/monsters`, `lookup:book_registries` (`mut_monster_golden.log`) |
| M2 | `crb/class_tables/class_tables.json` line 1648, Wizard level 5 `fort_save` 1 → 6 (a field neither attempt 1's Will plant nor E4a.2's BAB plant used) | PF hash pair | **Elowen moved** `8d1a711c…00f2` → `16053a51d5317376a18a4dc300e5feff6d50a49de56d42410aaf84620707054c`; Aldric unchanged; 12 changed lines = the Fortitude base (1→6) and total (2→7) in the values block and their two explanation rows (`mut_wizard_fort_pf.log`, `mut-wizard-fort-elowen.diff`, `pf-mut-wizard-fort/sha256.txt`) |
| M3 | same row, `fort_save` → `"one"` (wrong JSON type) | loader round trip | **FAILED** `every_table_round_trips_through_the_loader` (rc 101) (`mut_type_roundtrip.log`) |

The criterion (one row edit flips a PF hash or the round-trip test) holds on M2 and M3 independently; M1 shows the golden oracle covers a table no seed reads.

## Attempt 1's findings — dispositions

- **F1 — closed, no change needed.** Aldric's Fighter base saves come from `class_fighter.rs`'s progression formula (BAB = L, Fort = L/2+2, Ref = Will = L/3), not the package row. The formula predates the epic (`git show ddcb983967:src/rules_core/pilot_compute/class_fighter.rs` carries it at lines 485–487; `git log -S'will: level_value / 3'` → `a68bb09eb2`, SD-36). It equals the package's `crb/class_tables` Fighter rows at **20 of 20** levels on all four columns (predicate: integer formula value = row value; `f1-fighter-formula-vs-package.log`, Python over the JSON). E4a moved no Fighter value and introduced no divergence; the package row stays guarded by the golden digests (attempt 1's M-a). Which source a class reads is not an E4a criterion.
- **F2 — closed.** The single cross-epic catalog byte-change (Crocodile `groundingNote` `.lst` → stem) is decisions.md §19's mandated strip (E4a.3), is now named in E4a.4a's receipt (Does not cover), and the monster transcriber reproduces it: the 42 re-runs leave `bestiary/monster_data` unchanged.
- **Correction to E4a.4a's receipt:** it says "the desktop does not depend on codex-ingest" and so ran only the PF pair on the desktop. `codex-ingest` is a **dev-dependency** of `apps/desktop/src-tauri` (`Cargo.toml` line 40, `[dev-dependencies]`), so E4a.4a's new `rules_package_out` module is in the desktop test build. Closed here by running the whole desktop suite (below). Retro correction emitted.

## Build scope verified (`E4a.MC_attempt3_verify.sh`, one pass)

| Scope | Command | Result | Log |
|---|---|---|---|
| desktop (whole suite) | `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` in `apps/desktop/src-tauri`, with `SF_SEED_LINES_OUT` + `PF_SEED_RENDER_OUT` | 1 `test result` line: 699 passed, **1 failed**, 2 ignored, `Running unittests src/main.rs (…codex_desktop-…)` | `desktop_full.log` |
| — attribution | the 1 failure: `sf_sheet_print::tests::sf_seed_printed_features_equal_the_features_the_converted_records_grant` panicked at `sf_sheet_print.rs:438` "writes the seed's lines: NotFound": my script `rm -rf`'d the `SF_SEED_LINES_OUT` directory and did not recreate it (E7.1's script does `mkdir -p`). Harness error, not code. Re-run with the directory created: `cargo test --locked -j 8 --bins sf_sheet_print` → 13 passed, 0 failed | | `desktop_sf_sheet_print_rerun.log`; retro rework emitted |
| root lib (filtered) | golden + `rules_data_package` | 11 passed / 0 failed | `golden_clean.log` |
| codex-ingest | `cargo test -p codex-ingest --test table_generators_write_the_data_package` | 3 passed / 0 failed | `epic_tests.log` |
| Python | `python3 -m unittest scripts/tests/test_transcribe_monster_tables.py scripts/tests/test_transcribers_write_the_data_package.py` | Ran 50, OK | `python_tests.log` |
| E3 gate (codex-ingest changed by E4a.4a, §21) | `sheet_rule_convert -- --check`; `pcgen_residue_gate.py --check --closure` | `records=49450 converted=49450 refused=0 … verdict=PASS`; `live_hits=0 verdict=PASS` | `converter_check.log`, `residue.log` |
| verify.sh | four stages above | PASS | `verify_stages.log` |

**Not run:** the full root workspace sweep. E4a.4a ran it on the same code (`291484aa09` is its commit: 474 result lines, 8,346 passed, 0 failed), this card changed no code, and E7.2 owns the next widest-scope pass (R-V).

## Seed deltas

- Aldric (Fighter 3): unchanged — `1d830682…a569` rendered on HEAD (twice: `pf_clean`, desktop run) = E1.4.
- Elowen (Wizard 5 + Fireball): unchanged — `8d1a711c…00f2` rendered on HEAD = E1.4 (moves only under M2, then restored).
- SF-Soldier-3 / SF-Mystic-5 / SF-Technomancer-5 / SF-Envoy-3: **rendered** (desktop `sf_sheet_print` with `SF_SEED_LINES_OUT`); printed lines byte-identical to E7.1's: `diff -r artifacts/epic_7/E7.1_logs/seed_lines artifacts/epic_4a/E4a.MC_logs/attempt3/seed_lines` → only the extra `pf-sha256.txt`; 51 / 72 / 63 / 42 lines (awk `END{print NR}`), sha256 prefixes `1132b37dbd39` / `eac7ca0c6136` / `d9ebfa252de3` / `0526f8b4ff07`. The 160-of-160 hand-value fixtures are in the desktop suite that passed.

## Figures

- Generators writing into the removed directory: 0 (shell) / 0 (Python).
- Generator plants restored by their own generator: 6 of 6 generators (G1, G2, G3, G4b, G5, G6); G4 is a mis-aimed plant (table outside the generator's 21 books), not a failure.
- Package: 281 files, sha set `f699c1d6…dd27` before = after (sorted `sha256sum` hashed; `find … | awk 'END{print NR}'` → 281); 13,291,625 row bytes (`--check`).
- Spell-list tables with no generator among the six: 6 of 27 (`comm -23` of the package's `spell_list` dirs vs `ingest_spells`' log; `spell_list_generator_coverage.log`).
- Desktop: 699 + 1 harness-attributed failure, re-run 13 of 13 in that binary's `sf_sheet_print` set.
- **Raw row-count output:** `0 0 0 0`; `0`; `0`; `1117`; `0`; Python `[0, 0, 0, 0]`, `0`, `0`, `0 []`; `status_lines=0`.

## Audits

- This card's diff is docs, logs and a docs-side script only (no `src`, `crates`, `apps` path): run after the local commit and before the push (§6 step 4): `OK_NO_BUNDLE_TAGS`, `OK_NO_TOKENS`, `OK_NO_NOOP_HANDLERS`, `OK_NO_MOCK_LEAKS`, `OK_NO_WOULD_STRINGS` (vacuous: no `src`, `crates` or `apps` path in this diff).
- Epic-range audit: attempt 1 (`ddcb983967...ae5c7f3c3a`) found 3 pre-existing moved lines, none new. E4a.4a's own range re-run here (`118a07c7c0...291484aa09`, `e4a4a-range-audit.log`): `OK_NO_BUNDLE_TAGS` (crates + scripts), `OK_NO_TOKENS`; the three desktop/`src` checks are vacuous (E4a.4a touched no `apps/` or `src/` path).

## Structural diff

Not a converter cycle (no converter change here). E3's gate was re-run because E4a.4a changed `crates/codex-ingest`: `--check` PASS 49,450 records, residue PASS (above). E4a.4a's structural diff (`verdict=PASS`, 49,450 → 49,450, planted removal → FAIL) is not re-run; the `--check` record count agrees with it.

## retro.py summary since epic start (`scripts/retro.py summary --since 2026-10-06T16:38:00-04:00 --json` → `E4a.MC_logs/attempt3/retro-summary.json`, after this card's events)

32 events: correction 7, deferral 3, incident 3, note 3, resolution 2, rework 4, verification 10. Corrections by corrector: sd37-e4a-2 ×2, sd37-e4a-3 ×1, sd37-e4a-4a ×3, sd37-e4a-mc ×1; **no subject repeats** (`by_subject` all 1). Incidents: `dispatch-before-discovered-dependency` ×2 (attempts 1 and 2 of this card; the control is commit `118a07c7c0`, "script runs discovered cards next" — R-W's "more than twice" not reached), `verify-without-cargo-target-dir` ×1. Verification: 10 runs, 1 failed (clippy, E4a.2-era). Deferrals: **1 open** — E4a.4a's `1791369784650-sd37-e4a-4a-fc217e` (411 `.COPY=` spell variants never shipped; revisit "a PF content bundle"); it is not in any E4a criterion (the package never carried them; E4a.4a's DoD is byte-identity), so it is a capability deferral, not a blocker on this epic. The E4a.4 deferral (the six generators) is **resolved** by this card (`resolution` event).

## Does not cover

- The full root workspace sweep (E4a.4a's run on the same code stands; E7.2 re-runs).
- Catalog dumps, Bestiary 1 counts and the pre-E4a.1 tree were not rebuilt in attempt 3 (attempt 1 did; the package and `src/` are unchanged since — `git diff --name-only 1a694aa4d5 291484aa09`).
- Generators outside the six, and the 6 `spell_list` tables with no generator at all: their content is guarded by the golden digests only, and nothing regenerates them.
- Byte-identity covers the pinned oracle and today's `docs/work-inventory.json`.
- Mutations are three single-field edits in two files; the golden coverage of the other 279 files rests on its 429-row manifest.
- The packaged app was not built or launched.
- Elowen's render still prints the provenance string `rules_tables::crb::class_tables::class_tables()` (module removed; SD-i kept 44 literal sites by ruling) — reported, not changed.

## Safe defaults taken

None from §12.1. Process choice: re-ran only what E4a.4a could have moved (generators, package, golden, PF, desktop suite, E3 gate) plus every acceptance command, instead of rebuilding the pre-epic tree again (alternative not taken: repeat attempt 1's cross-epic build, which E4a.4a's diff cannot reach).

## Retro events emitted

`docs/retro/events/sd37-e4a-mc.jsonl` (attempt 3): correction ×1 (E4a.4a receipt: desktop dev-dependency), rework ×1 (seed-lines directory in my script), resolution ×1 (E4a.4 generators deferral), note ×1 (F1/F2 dispositions). `verify.sh` emitted its own derived verification event.

## Next-cycle plan

E4a is closed; E7.2 (widest-scope verify) is next.
