# Cycle 5 — Epic 6 (PCGen exit) / AT-35-E6-003

Cycle 4 named the nine compiled-table-rostered `apps/desktop` files as this cycle's scope and
"converted-row coverage" as their mechanism. This cycle built that join, pointed it at three of
the nine, shipped the one that came out green, and **measured the two that did not against the
live package**. Converted-row coverage is no longer the binding blocker; the converter's own
prose is. `root apps/desktop` **9 files / 230 hits → 9 / 219**.

- **Commit SHA:** `eddc6fc703` — the new join module, the monster-catalog swap, the residue
  comment rewrites, the measurement artifact, this cycle's three retro events, and two appends
  folded from the shared checkout (`sd31-transcribe.jsonl` +1, the atlas `derived_at` stamp).
  Cycle start `2ca48799ff`. This receipt, the `progress.md` entry and the `kanban.md` row ride
  the following commit — a receipt cannot name the commit that carries it.
- **Scope gate:**
  `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero units by design, decisions.md §2)`.
  Run anyway, for the record — `python3 scripts/cycle_scope_gate.py --min 500`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  `remaining_non_done=0` — the corpus reached `DONE 49438 of 49438` at `AT-35-E5-005`. Epic 6
  moves no unit; it takes the ingest format off the live side.
- **Files touched:** **7** — 3 Rust, 4 bookkeeping.
  - **`apps/desktop/src-tauri/src/converted_prose.rs`** (new, 330 lines) — the four-step join
    from a catalog row to a converted record, with six of its own tests.
  - **`apps/desktop/src-tauri/src/monster_catalog.rs`** — `serve_ability_description` swapped to
    the converted package, the leak panic removed (nothing left to leak), seven doc/test comments
    restated over our own schema, and the new corpus-wide population ratchet.
  - **`apps/desktop/src-tauri/src/main.rs`** — one `mod converted_prose;` line.
  - **`docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003_cycle5_converter-prose-blocker.md`**
    — the measurement (see **Discoveries**).
  - **`docs/retro/events/at-35-e6-003.jsonl`** (+3), **`docs/retro/events/sd31-transcribe.jsonl`**
    (+1, folded), **`docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`**
    (the `derived_at` stamp the atlas check rewrites).
  - **Zero `src/` files changed. Zero `scripts/` files changed. Zero `data/` files changed. Zero
    `tests/` files changed.** `apps/` was the whole code surface, which is why the desktop crate
    ran here and the root workspace's `--no-fail-fast` run correctly did not
    (`workflow-instruction.md §6` step 3).
  - **Not committed, and deliberately:** `.worktrees/ci-trait-choice` shows in an unfiltered
    `git status --porcelain` as untracked. It is a **git worktree** — another checkout of this
    repository — present before this cycle started. Committing it would commit a second tree.
- **Identifier audit result:** **OK_NO_BUNDLE_TAGS for this cycle; 31 matches against the develop
  base, pre-existing and not this cycle's.**
  ```bash
  BASE=fe5ae6cd4a5f3c65d5d10f4d523f00e33b04ac47      # git merge-base HEAD origin/develop
  SC="src/rules_core/pilot_compute src/rules_core/feat_prereqs src/pcgen_import \
      apps/desktop/src-tauri/src apps/desktop/src src/bin \
      docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit"
  git diff --unified=0 "${BASE}...HEAD" -- $SC ':!**/__tests__/**' ':!**/*.test.*' > /tmp/base.diff
  grep -cE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' /tmp/base.diff   -> 31
  # this cycle's own added lines:
  git diff --unified=0 2ca48799ff -- $SC ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '^\+.*\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'         -> (no output)
  ```
- **Wired-integration audit result:** **OK_NO_TOKENS.** No `STUB`, `MOCK`, `not yet implemented`,
  `todo`, `fixme` or `hack` on any line this cycle added (`grep -nEi` over the cycle diff: no
  output). `placeholder` appears **15** times in the cycle diff and every one is the ingest
  format's own vocabulary — the `%N` positional placeholders this cycle is removing the live
  reader of — never a stand-in for unwritten code. The base diff carries 81 of the same word for
  the same reason.
- **Acceptance criterion** (verbatim, `epic-breakdown.md`):

  > ### AT-35-E6-003 — the desktop crate and the prose renderer leave PCGen behind
  >
  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers of
  > `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc` is
  > deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  **Not met.** `apps/desktop/` is at 9 files / 219 hits, not zero. The desktop suite is green and
  the on-screen tests pass; the zero-hits half is what remains.
- **Receipt rows (mechanical):**
  ```
  python3 scripts/cycle_scope_gate.py --receipt --since 2ca48799ff91f644ca1784bc036fa0ce6e59748f \
    --before /tmp/wi-before-AT-35-E6-003.json --after docs/work-inventory.json
  since=2ca48799ff91f644ca1784bc036fa0ce6e59748f residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=462 ratio=n/a builds_recorded=0 pcgen_live_files=205
  ```
  `closed=0` is correct and by design (`decisions.md §2`): Epic 6 moves no unit.
  `pcgen_live_files=205` is **flat**, not raised — the new module adds a file to the tree but
  zero hits to the gate.
- **PCGen residue:** `python3 scripts/pcgen_residue_gate.py --check`
  ```
  root src/rules_core files=196 hits=11414
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=9 hits=219
  identifier_files=13 identifier_hits=123
  live_files=205 live_hits=11633 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  At cycle start: `root apps/desktop files=9 hits=230`, `identifier_files=14 identifier_hits=127`,
  `live_files=205 live_hits=11644`. **−11 hits, −1 identifier file, never above the baseline.**
- **Oracle parity:** N/A — no `Number` mapping was added, no converter row changed, and
  `data/sheet_rules/` is byte-identical to the cycle's start (`git status --porcelain -- data/`
  empty at every checkpoint). The live path this cycle changed serves prose, which
  `sheet_rule_convert -- --check` and the package-wide source-marker grep prove for the whole
  package rather than per record.
- **Movement, four buckets:**
  - **closure (into DONE):** none — Epic 6 closes no unit by design.
  - **relabel:** none.
  - **reachability:** **net +1 monster ability** carries text on the wire (3,455 → 3,456 of the
    same 3,509 served rows). **Net**, measured as a count and not as an id-set diff: +1 is
    consistent with one gained and none lost, and equally with four gained and three lost. The
    per-record churn was not enumerated (`AGENTS.md` rule 7 — see **Notes**).
  - **instrument-correction:** two, both recorded as `correction` events — cycle 2's
    converted-row-coverage diagnosis (see **Discoveries**) and this cycle's own first
    loss count.
- **Refused tokens:** **219 hits across 9 `apps/desktop` files**, by pattern —
  `` `DESC:` ``=69, `` `PRE[A-Z]+:` ``=43, `` `render_pcgen_desc` ``=33, `` `BONUS:` ``=31,
  `` `raw_tokens` ``=24, `` `raw_bonus_chains` ``=10, `` `%CHOICE` ``=5, `` `TYPE=` ``=4;
  by file — `companion_catalog.rs`=52, `race_trait_picker.rs`=33, `feat_catalog.rs`=30,
  `intelligent_item_catalog.rs`=28, `equipment_catalog.rs`=25,
  `raceCreationCoverage.test.ts`=21, `reference_library_catalog.rs`=15, `spell_catalog.rs`=10,
  `monster_catalog.rs`=5. Both partitions sum to 219 of 219, and both agree with the gate's own
  `root apps/desktop files=9 hits=219`. **8 distinct token types** — under
  `workflow-instruction.md §8`'s limit of 10.
- **Discoveries:** **four, three of them mechanism-shaped.**
  1. **Cycle 2's diagnosis is superseded.** Converted-row coverage is not what blocks the
     compiled-table catalogs. `convert_desc_like` converts each `|`-argument through
     `convert_formula`; `CL` on a record with no owning class returns
     `Err("FORMULA:CL-no-owner")` (`src/pcgen_import/sheet_rule/formula.rs:501-506`), the `?`
     propagates, and `convert_token`'s caller refuses **the whole DESC row**. The description is
     not degraded — it is dropped. 30 feat rows and 2 spell rows lose a description on the swap.
     `correction 1789106039024-at-35-e6-003-df19bf`.
  2. **48 rows of the Spell Catalog were serving the literal string `[redacted PI]`**, and one
     more (`ACG :: Discern Next of Kin`) was serving text the corpus record declares product
     identity. Removing them is a PI fix that the converted package performs for free.
     `correction 1789106039171-at-35-e6-003-ec568c`.
  3. **`core_essentials` is compiled but not converted** — `data/sheet_rules/` has no such
     directory, so 11 Core Essentials feat rows can never resolve.
  4. **Two corpus records are converted-and-then-absent, reported by nothing:**
     `advanced_players_guide:spell:wall_of_thorms` and
     `mythic_adventures:spell:elemental_body_iiimod` are `in_scope`/`full` in `data/corpus/`,
     hold no record in `data/sheet_rules/`, and are **not** among `_refused.json`'s 142 entries
     (all of which are `no_corpus_record`).

  All four are recorded with their re-derive commands in
  `AT-35-E6-003_cycle5_converter-prose-blocker.md`.
- **Figures + their re-derive commands:**
  | figure | denominator | command |
  |---|---|---|
  | `apps/desktop` 9 files / 219 hits (from 9 / 230) | the live roots' source files | `python3 scripts/pcgen_residue_gate.py --check` |
  | monster abilities served 3,509; with a description 3,456 (from 3,455) | abilities the Monster Catalog serves | `cd apps/desktop/src-tauri && cargo test --locked -j 6 converted_ability_prose_population -- --nocapture` |
  | spell catalog 2,481 rows, 2,436 described before / 2,410 after, 25 gained, 48 `[redacted PI]`, 1 PI, 2 real losses | rows in `spell_resolver::spell_catalog_rows()` | the cycle's own before/after census, reproduced in §3 of the blocker artifact |
  | feat catalog 2,227 rows, 2,161 described before / 2,124 after, 1 gained, 8 `[redacted PI]`, 30 real losses | rows in `all_feat_tables()` | §4 of the blocker artifact |
  | 487 of 2,883 converted `feat` rules and 634 of 3,102 `spell` rules carry no prose at all | rules in `data/sheet_rules/` | `python3 -c "import json,glob;rs=[r for f in glob.glob('data/sheet_rules/*/feat/*.json') for r in json.load(open(f))];print(sum(1 for r in rs if not r.get('prose')),'of',len(rs))"` (and the same with `spell`) |
  | converter: 49,438 records, 49,296 converted, 142 refused, 69,346 rules, all refusals `no_corpus_record` | the corpus census | `cargo run --locked --bin sheet_rule_convert -- --check`; `python3 -c "import json;print(json.load(open('data/sheet_rules/_refused.json'))['by_token_type'])"` |
  | identifier audit 31 base matches | the develop-base diff over `<scoped paths>` | in the Identifier audit row above |
- **Build scope verified:** run at `eddc6fc703`, with `CARGO_TARGET_DIR` per tree
  (`/tmp/cargo-sd35-AT-35-E6-003` for the root workspace, `…-desktop` for the desktop crate —
  `AGENTS.md`: one directory per agent **per source tree**).
  ```
  cargo test --locked --no-run -j 6                  NO_RUN_EXIT=0 (0 error lines; every test binary linked)
  cargo test --locked --lib -j 6                     ok. 3296 passed; 0 failed; 15 ignored
  cd apps/desktop/src-tauri && cargo test --locked -j 6
                                                     ok. 576 passed; 0 failed; 0 ignored
  cd apps/desktop/src-tauri && cargo clippy --locked --tests -j 6
                                                     0 warnings, 0 errors
  python3 scripts/pcgen_residue_gate.py --check      verdict=PASS (live_hits 11644 -> 11633)
  cargo run --locked --bin sheet_rule_convert -- --check   exit 0; records=49438 converted=49296 refused=142
  grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l      0
  python3 scripts/completion_atlas.py --check        missing_clearing_mechanisms=0 citation_failures=0
  python3 scripts/token_coverage.py --check          non_done=0 refused=142 refused_non_done=0 shapes=1 verdict=PASS
  python3 scripts/shape_engine_boundary.py --check   magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True
  python3 scripts/missing_engine_tables.py --check   population=0 kinds=0 citation_failures=0
  python3 scripts/denominator_gate.py --check '…/*.md' '…/artifacts/**/*.md'      files_checked=100 violations=0
  scripts/verify.sh --only pi-sweep                  RESULT: PASS (11 hits, 11 baseline rows)
  ```
  `cargo test --locked --no-fail-fast -j 6` was **not** run: `workflow-instruction.md §6` requires
  it when `src/` or the classifier changed, and neither did (`git status --porcelain -- src/
  scripts/ data/ tests/` empty at every checkpoint). The frontend suite was not run: no file
  under `apps/desktop/src/` changed.
- **Sweep population:** N/A — no corpus record changed, so `corpus_literal_sweep` was correctly
  not run (`workflow-instruction.md §6` step 3).
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`. Read once, to
  quote the ACG `DESC:` row that names the blocker (`acg_feats.lst:20`); no figure in this
  receipt is derived from the pinned corpus.
- **Status:** **partial**
- **What this proof does not cover** (`AGENTS.md` rule 7): the monster swap is proved by a
  **count** of abilities carrying text before and after, not by the served-id-set diff cycles 3
  and 4 used, so a one-for-one swap inside the 3,509 would read as net +1. The suite's own
  on-screen tests and the cross-catalog syntax sweeps stayed green, which is evidence and not the
  same thing. The ratchet is a floor, not an identity. The spell and feat measurements in
  **Discoveries** describe a tree that was then reverted and prove nothing about the shipped HEAD.
  Nothing here proves anything about the other eight files.
- **Notes:** `spell_catalog.rs` and `feat_catalog.rs` were swapped, measured and **reverted**.
  Both go red on the desktop suite (11 of 576) for a converter-side cause, and
  `workflow-instruction.md §8` lists RED→GREEN not preserved as non-self-healable; cycle 2 set
  the revert precedent for exactly this shape. The measurement is the deliverable, not the
  revert: it replaces a three-cycle-old diagnosis with a named, re-derivable mechanism.
  `deferral 1789106039299-at-35-e6-003-380aef`.
- **Next-cycle scope:** **AT-35-E6-003 cycle 6, on the CONVERTER first.** (1) `FORMULA:CL-no-owner`
  prints the term's words rather than refusing the DESC row — the same correction cycle 4 made
  for `PREVARLT`'s equipped-item census, one level over; (2) the three bare-`%` leaks
  (`core_rulebook:spell:teleport`, `inner_sea_world_guide:spell:ancestral_memory`, feat
  `Prophetic Visionary`); (3) `core_essentials` converted; (4) the two converted-and-then-absent
  spell records. Then `spell_catalog.rs` (10) and `feat_catalog.rs` (30) with the §3/§4 censuses
  re-run. Scope flags:
  `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero units by design, decisions.md §2)`.
  Target: `root apps/desktop files=0 hits=0`.
