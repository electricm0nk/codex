# Cycle AT-35-E6-005-SHIPPED-DATA cycle 1 — Epic 6 PCGen exit / AT-35-E6-005-SHIPPED-DATA

- **Commit SHA:** 2f824171b5acb9f270d1c2745fcfae1c8c191383  (cycle start `fdc90243f43df3ddc56cdd361d28bd1f2e150e40`)
- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 instrument + shipped-data cleanup cycle — closes zero corpus units by design, decisions.md §2)`. Run anyway, literal output:
  `scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER` (`python3 scripts/cycle_scope_gate.py --min 500`)
- **Files touched:**
  - `scripts/pcgen_residue_gate.py` (shipped-data class)
  - `scripts/tests/test_pcgen_residue_gate.py` (`TestShippedDataIsScanned`, 9 new tests)
  - `src/bin/gen_desktop_fixture_corpus.rs` (input root split; shipped equipment document)
  - `src/bin/gen_settled_corpus.rs` (fixture book read from `fixtures_src/`, bundle written to the shipped root)
  - `apps/desktop/src-tauri/src/corpus_fixtures.rs` (`FIXTURE_SOURCES` removed — the inputs no longer ship)
  - `apps/desktop/src-tauri/fixtures_src/**` — **6 `git mv` renames** (4 `.txt`, 2 ingest-format `.json`)
  - `apps/desktop/src-tauri/resources/corpus_fixtures/equipment/*.json` (regenerated, token arrays gone)
  - `apps/desktop/src-tauri/resources/corpus_fixtures/spell/*.json` (regenerated, `source.path` follows the inputs)
  - `docs/release/SD-35-corpus-sheet-completion/{decisions.md §20, epic-breakdown.md, progress.md, kanban.md, artifacts/epic-6-pcgen-exit/}`
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** verbatim at `epic-breakdown.md` `### AT-35-E6-005-SHIPPED-DATA` (added this cycle from operator ruling B17, `decisions.md §20`).
- **Receipt rows (mechanical):** `closed=0 relabeled=0 rust_lines_changed=172 ratio=n/a builds_recorded=1 pcgen_live_files=0`
  (`python3 scripts/cycle_scope_gate.py --receipt --since fdc90243f4 --before /tmp/wi-before-…json --after docs/work-inventory.json`)
- **PCGen residue:**
  - at cycle start, instrument as it was: `live_files=0 live_hits=0 baseline_files=260 baseline_hits=12736 verdict=PASS`
  - **instrument corrected** (shipped-data class added, data untouched): `shipped_data_files=6 shipped_data_hits=30 shipped_scanned=15 / live_files=6 live_hits=30 verdict=FAIL`
  - at HEAD, data cleaned: `shipped_data_files=0 shipped_data_hits=0 shipped_scanned=11 / live_files=0 live_hits=30→0 verdict=PASS`
    (`python3 scripts/pcgen_residue_gate.py --check --closure --list-files`)
- **Oracle parity:** N/A — no `Number` mapping added and no live compute path touched. The tool side is untouched: `git diff --stat fdc90243f4 -- src/pcgen_import scripts/oracle_harness src/oracle_validation` is **empty**, `^-\s*(pub )?fn ` count = **0** (zero net deletions of function bodies), `scripts/pcgen-oracle-pin.env` unchanged.
- **Movement, four buckets:**
  - closure: **none** (zero corpus units in scope)
  - relabel: none
  - reachability: none
  - **instrument correction: `live_files` 0 → 6, `live_hits` 0 → 30.** The gate scanned source only and printed `root apps/desktop files=0 hits=0` while six files under `bundle.resources` shipped PCGen text. The jump is the instrument becoming honest about a defect that was always there; it closes nothing and **is not netted against the cleanup**. The cleanup is the separate 6 → 0 / 30 → 0 fall, caused by moving the converter inputs out of `bundle.resources` and regenerating.
- **Refused tokens:** none
- **Discoveries:** the dispatch's census named **5** shipped files; the re-derivation found **6** — `equip_longsword.txt` is also a raw `.lst` row under a `bundle.resources` directory entry and was not named (`correction 1789385567973`). Second: Tauri v2 copies a **directory** resource recursively, so the four `.txt` files at the top of `corpus_fixtures/` shipped even though only the `spell/`, `equipment/` and `_settled/` subdirectories are listed individually — the gate walks directory entries recursively for the same reason.
- **Figures + their re-derive commands:**
  | figure | value | command | denominator |
  |---|---|---|---|
  | shipped files scanned | 11 | `python3 scripts/pcgen_residue_gate.py --check --closure` | every file under `bundle.resources` in `apps/desktop/src-tauri/tauri.conf.json`, binary/oversize excluded |
  | shipped data hits before | 30 over 6 files | same, at the instrument-correction commit | the 15 files then under `bundle.resources` |
  | shipped data hits after | 0 over 0 files | same, at HEAD | the 11 files now under `bundle.resources` |
  | independent grep of `bundle.resources` | 0 lines | `cd apps/desktop/src-tauri && for f in $(find resources -type f); do grep -cE '\bBONUS:\|\bDEFINE:\|\bPRE[A-Z]+:\|\bSAB:\|\bDESC:\|%CHOICE\|%LIST\|\bTYPE=\|raw_tokens\|raw_bonus_chains' "$f"; done` | all 11 shipped files |
  | gate unit tests | 39 passed, 0 failed | `python3 -m unittest scripts.tests.test_pcgen_residue_gate` | 30 pre-existing + 9 new |
  | `data/sheet_rules/` token leaks | 0 | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` | every file under `data/sheet_rules/` |
  | tool-side fn bodies deleted | 0 | `git diff fdc90243f4 -- src/pcgen_import scripts/oracle_harness src/oracle_validation \| grep -cE '^-\s*(pub )?(async )?fn '` | the three kept trees |
- **Build scope verified:** `--no-run` exit **0** / 413 executables; lib **3390 passed; 0 failed; 16 ignored**; full workspace `--no-fail-fast` **FULL_EXIT=0**, 419 targets run / 420 result lines, **8,919 passed; 0 failed; 69 ignored**, `FAILED` lines **0**; desktop crate (`apps/` was touched, so it ran here) **570 passed; 0 failed**, the three `corpus_fixtures` bundle tests green in a targeted re-run; clippy **0 warnings** on the desktop crate and on the two touched bins. Frontend not run: no `.ts`/`.tsx` file is in this cycle's diff. Run at SHA `2f824171b5acb9f270d1c2745fcfae1c8c191383`.
- **Sweep population:** N/A — no `data/corpus/` record changed. `gen_settled_corpus` re-derived all 48 bundles over 40 roots and reported `drifted=0`, so no corpus record moved and `corpus_literal_sweep` has nothing new to examine.
- **Oracle pin:** unchanged (`scripts/pcgen-oracle-pin.env` not in this cycle's diff); no figure here came from the pinned corpus.
- **Status:** complete
- **Notes:** This is a **MOVE, not a deletion** (`decisions.md §11`). Six files were relocated with `git mv` to `apps/desktop/src-tauri/fixtures_src/`, outside `bundle.resources`: a converter input is kept for Starfinder, it is simply not shipped. No fixture was deleted, no regex weakened, no resource entry dropped, and `EXCLUDED_PREFIXES` is still empty. The shipped `equipment/*.json` records keep their filenames because `corpus_loader::load_equipment_corpus` enumerates them for the settled bundle's KEY and reads every value from `_settled/equipment.json` — proven by reading that function, not assumed — so they now carry the settled identity fields and no token array.
- **Next-cycle scope:** criterion at zero. `AT-35-E7-001`'s shortfall S5 is cleared; S1–S4 are untouched by this cycle and remain Epic 7's.
