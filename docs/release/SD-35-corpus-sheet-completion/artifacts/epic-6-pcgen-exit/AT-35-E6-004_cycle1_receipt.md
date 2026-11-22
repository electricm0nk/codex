# Cycle 1 — Epic 6 (PCGen exit) / AT-35-E6-004

- **Commit SHA:** `72103a69bc` (cycle start `96fa840c6b`)
- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 closure cycle — closes zero units by design, decisions.md §2)`.
  Run anyway for the record:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  `remaining_non_done=0` — the corpus reached `DONE 49438 of 49438` at `AT-35-E5-005`. This cycle
  moves no unit; it certifies the gate and wires its closure mode.
- **Files touched:** 3 edited —
  `scripts/pcgen_residue_gate.py` (new `mask_non_code`; `cfg_test_ranges` now brace-matches over
  code characters only), `scripts/tests/test_pcgen_residue_gate.py` (+2 RED→GREEN tests),
  `scripts/verify.sh` (the `pcgen-residue-gate` stage's default mode is now `--check --closure`).
  Plus this receipt, `AT-35-E6-004_cycle1_sheet-parity-after.json`,
  `AT-35-E6-004_cycle1_independent_residue_census.py`, `progress.md`, `kanban.md`, the retro event
  log, and the `completion-atlas.json` `derived_at` stamp its own `--check` run rewrote.
  **No file under `src/`, `apps/` or `data/` changed.**
- **Identifier audit result:** OK_NO_BUNDLE_TAGS.
  `git diff --unified=0 HEAD -- scripts/pcgen_residue_gate.py scripts/tests/test_pcgen_residue_gate.py scripts/verify.sh | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  → no match. Over the epic-wide base
  (`git merge-base HEAD origin/develop` = `fe5ae6cd4a`) the same grep returns 172 lines, all
  predecessor cycles' and all either test **file names** (`tests/sd34_wave51_…`) or doc prose,
  already audited in their own receipts.
- **Wired-integration audit result:** OK_NO_TOKENS. Same diff, `grep -nE
  '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` → no match. Epic-wide the
  grep returns 161 lines / 31 **added** lines under `src/` + `apps/`; every one is doc-comment
  prose about the ingest format's *positional placeholder* (`%1`, `%2`), not a stub. Re-derive:
  `git diff --unified=0 fe5ae6cd4a...HEAD -- src/rules_core src/pcgen_import apps/desktop src/bin ':!**/__tests__/**' ':!**/*.test.*' | grep -E '^\+' | grep -v '^+++' | grep -iE '…'`
- **Acceptance criterion** (verbatim, `epic-breakdown.md`):

  > ### AT-35-E6-004 — the gate reads zero
  >
  > **"Zero" means zero CODE hits** — operator ruling B14, 2026-09-11 (`decisions.md §17`). A
  > live-side doc comment that quotes an ingest-format token is provenance, not a read, and the
  > gate no longer counts one. […] The gate's own `live_files=` / `live_hits=` lines are now
  > code-only counts; nothing else about this criterion changes, and the resulting 197→81 drop is
  > an instrument correction that closes nothing.
  >
  > **Evidence:** `python3 scripts/pcgen_residue_gate.py --check --closure` → `live_files=0
  > live_hits=0 verdict=PASS`, wired as the stage's closure mode from this cycle on. The oracle
  > comparison at the end of the epic agrees with the one at its start. `cargo tree` for the
  > desktop crate shows no dependency on the converter modules. **And the tool side is intact**
  > (`decisions.md §11`, what is kept): `cargo build --locked --bin sheet_rule_convert --bin
  > gen_book_cache` exits 0; `python3 scripts/oracle_harness/run.py --help` exits 0;
  > `scripts/pcgen-oracle-pin.env` unchanged; `git diff --stat <epic-6-start-sha>..HEAD --
  > src/pcgen_import scripts/oracle_harness src/oracle_validation` shows moves and additions,
  > **zero net deletions of function bodies** (a moved file is not a deleted one).

- **Receipt rows (mechanical):**
  ```
  since=96fa840c6b65c60223af0ff61179d350ecd76ac7 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=1 pcgen_live_files=0
  ```
- **PCGen residue:** at or below every predecessor receipt — zero on both axes, in both modes.
  ```
  pattern raw_tokens files=0 hits=0
  pattern raw_bonus_chains files=0 hits=0
  pattern PcgenFormulaEvaluator files=0 hits=0
  pattern render_pcgen_desc files=0 hits=0
  pattern bonus_stack_reader files=0 hits=0
  pattern pre_tokens files=0 hits=0
  pattern BONUS: files=0 hits=0
  pattern DEFINE: files=0 hits=0
  pattern PRE[A-Z]+: files=0 hits=0
  pattern SAB: files=0 hits=0
  pattern DESC: files=0 hits=0
  pattern %CHOICE files=0 hits=0
  pattern %LIST files=0 hits=0
  pattern TYPE= files=0 hits=0
  pattern pcgen_import files=0 hits=0
  root src/rules_core files=0 hits=0
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=0 hits=0
  identifier_files=0 identifier_hits=0
  live_files=0 live_hits=0 verdict=PASS
  ```
  `python3 scripts/pcgen_residue_gate.py --check --closure` → exit 0. Ratchet mode
  (`--check`) prints the same breakdown with `baseline_files=260 baseline_hits=12736 verdict=PASS`.

- **Did the gate earn its zero?** This criterion's job is to catch a green gate that does not
  measure what it claims, so the zero was **not** taken on the gate's own word. Three checks:

  1. **B14, B15 and B16 are actually implemented**, not merely documented. `code_only` skips a
     line whose left-stripped form starts with `//` and scans every other line whole (B14);
     `cfg_test_ranges` + `_live_lines` skip the `#[cfg(test)]` **item region** (B15);
     `RUNTIME_IMPORT_PATTERNS = {"pcgen_import": r"\bpcgen_import\b"}` is in `PATTERNS` and out of
     `IDENTIFIER_PATTERNS` (B16). `EXCLUDED_PREFIXES` is the empty tuple — **no live path is
     carved out** — and all five roots including `apps/desktop` are scanned.
     `python3 -m unittest scripts.tests.test_pcgen_residue_gate` → **Ran 29 tests, OK**.
  2. **An independent census, different algorithm, agrees.**
     `AT-35-E6-004_cycle1_independent_residue_census.py` re-reads the live side with a
     character-level Rust scanner that understands string literals, raw strings, char literals and
     block comments — nothing borrowed from the gate but its root list and its patterns:
     ```
     INDEPENDENT live_files=0 live_hits=0
     over_skip_hits_hidden_by_gate=0
     lines_gate_counts_that_independent_calls_test=0
     gate_skipped_lines=90433 independent_skipped_lines=90433 files_with_boundary_mismatch=0
     ```
  3. **The wiring fails on a real re-entry.** A one-line live-side read planted at
     `src/rules_core/at35_e6_004_closure_probe.rs` →
     `FAIL pcgen-residue-gate (live_files=1 live_hits=1 verdict=FAIL)`, `RESULT: FAIL`; probe
     removed → `PASS … (live_files=0 live_hits=0 verdict=PASS)`. Stage-level RED→GREEN, executed,
     not narrated.

- **Defect found and fixed (the reason check 2 exists):** before this cycle the two censuses
  **disagreed**: `gate_skipped_lines=91053` against `independent_skipped_lines=90433`,
  `files_with_boundary_mismatch=3`. `cfg_test_ranges` counted braces line by line, so
  `b"not-json-{garbage"` inside the first `#[cfg(test)]` module of
  `apps/desktop/src-tauri/src/update/transaction.rs` left the counter one `{` deep at that
  module's closing brace (line 1868); the skip ran on to the next test module's brace at 2885 and
  **blanked 618 lines of shipping code** (1869–2486). Nothing in that window reads PCGen, so the
  verdict was right **by luck** — the exact `validate-proxies-against-known-truth` /
  `AGENTS.md` rule 7 shape this criterion exists to catch. Fixed by brace-matching over
  `mask_non_code()` output; the mirror case (a `// }` in prose ending a region early, which
  over-counts) is fixed with it. RED→GREEN pinned by
  `TestCfgTestRegionsAreNotLiveCode::test_a_brace_inside_a_test_string_does_not_swallow_shipping_code`
  and `…::test_a_brace_in_a_comment_inside_the_region_does_not_end_it_early` (both FAIL before the
  fix, for the right reason: `BONUS: 0 != 1` and `SAB: 1 != 0`). Retro
  `correction 1789337005325`. **620 lines of shipping code returned to the scanned surface and the
  count stayed at zero.**

- **Closure mode is now the stage's mode.** `scripts/verify.sh`'s `run_pcgen_residue_gate` reads
  `${PCGEN_RESIDUE_GATE_CLOSURE:-1}` — was `:-0`. `scripts/verify.sh --only pcgen-residue-gate` →
  `==> pcgen-residue-gate — python3 scripts/pcgen_residue_gate.py --check --closure` /
  `PASS pcgen-residue-gate (live_files=0 live_hits=0 verdict=PASS)` / `RESULT: PASS`. The ratchet
  remains reachable as `PCGEN_RESIDUE_GATE_CLOSURE=0` (verified: label drops to `--check`,
  `RESULT: PASS`) as a diagnostic aid, never the shipping posture. The baseline file is untouched.

- **Oracle parity:** **run at HEAD, and it agrees with the epic's start.**
  `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`,
  unchanged since `c3500e7984`).
  ```
  sheet_parity: lines compared=156 agree=154 disagree=2 unverifiable=67;
                chassis compared=382 agree=376 disagree=6 unverifiable=140;
                characters=29 lines=505
  ```
  | | at AT-35-E6-001 (epic start) | at HEAD (epic end) |
  |---|---|---|
  | pin | `7f818006e3` | `7f818006e3` |
  | roster | 29 characters | **identical list** |
  | chassis compared/agree/disagree | 382 / 376 / 6 | **382 / 376 / 6** |
  | lines compared/agree/disagree | 146 / 145 / 1 | 156 / 154 / 2 |
  | disagreements | 7 | 8 — **all 7 still present, same `ours`/`oracle` values; `gone_since_start=0`** |

  The 10 extra lines compared are lines Epics 2–6 made renderable, not movement in an existing
  number: agreement rose with them (145 → 154) and **no disagreement was cleared or created by
  this cycle** — the document is field-for-field identical to
  `AT-35-E6-003-SWEEP_cycle13_sheet-parity-after.json` (`head doc == prev doc: True`). The one
  disagreement new *since the epic's start* is `half_elf_fighter_l1 target:Pool:favored_class
  ours=1 oracle=2`, named and attributed in SWEEP cycle 13's receipt, not this cycle's. Artifact:
  `AT-35-E6-004_cycle1_sheet-parity-after.json`.

- **The tool side is intact** (`decisions.md §11` — KEPT for Starfinder):
  ```
  cargo build --locked --bin sheet_rule_convert --bin gen_book_cache   -> TOOLSIDE_EXIT=0  (Finished dev profile in 1m16s)
  python3 scripts/oracle_harness/run.py --help                         -> ORACLE_HELP_EXIT=0
  git diff --stat c3500e7984..HEAD -- scripts/pcgen-oracle-pin.env     -> (empty; pin unchanged)
  git diff --stat c3500e7984..HEAD -- src/pcgen_import scripts/oracle_harness src/oracle_validation
                                                                       -> 64 files changed, 38465 insertions(+), 236 deletions(-)
  ```
  **Zero net deletions of function bodies**, proved by name rather than by diffstat — a diffstat
  cannot tell a move from a removal. Over `src/pcgen_import/**`, `scripts/oracle_harness/**` and
  `src/oracle_validation/**`, every `fn`/`def` name defined at `c3500e7984` was re-derived at
  `HEAD`:
  ```
  tool_side_files start=39 head=88
  fn/def definitions start=536 head=1652
  names_present_at_start_and_absent_at_HEAD=0
  ```
  The single file git reports as `D` — `src/pcgen_import/source_content_payload.rs` (145 lines,
  `c094391246`) — is a **split**, not a removal: all five of its definitions
  (`b6_metadata_kind_to_canonical`, `canonical_metadata_kind_inner_to_b6`, `SourceContentPayload`,
  `kind_token`, `source_slice`) live at `src/pcgen_import/ir_content_payload.rs`, with the
  converter-free half at `src/rules_core/source_content.rs`. Git's `-M` did not pair them because
  the content went to two places.

- **`cargo tree` for the desktop crate — and what it can and cannot prove.**
  ```
  codex-desktop v0.15.0 (apps/desktop/src-tauri)
  ├── base64 v0.22.1
  ├── codex v0.1.0 (/home/ubuntu/workspace/repos/codex)
  ├── serde / serde_json / sha2 / uuid
  └── tauri v2.11.5, tauri-plugin-dialog, tauri-plugin-opener
  ```
  No converter crate is a dependency — but `src/pcgen_import/` is a **module of the `codex`
  crate**, not a crate, so `cargo tree` is crate-grained and **cannot** settle module-level
  independence. Stating it as if it could would be the same false-confidence move B16 was ruled
  over. The module-level proof is the gate plus the independent census:
  `root apps/desktop files=0 hits=0`, `pattern pcgen_import files=0 hits=0`, and a full-text grep
  of `apps/desktop/src-tauri/src` + `apps/desktop/src` finds **166** occurrences of `pcgen`, all
  of them in `//!`/`///`/`//` provenance comments or `#[cfg(test)]` regions — zero executable
  reads. One of the 166 is a rustdoc intra-doc link
  (`race_trait_picker.rs:65` → `[codex::pcgen_import::pcgen_desc::PcgenDisplayValues]`): a
  documentation reference resolved by rustdoc, not a run-time read, and provenance under B14.

- **Movement, four buckets:**
  - **closure (into DONE, by id-set):** none. Epic 6 closes no corpus unit; `closed=0`.
  - **relabel (bucket to bucket):** none. `regressed=0 added=0 dropped=0`;
    `docs/work-inventory.json` is byte-identical to `96fa840c6b`'s.
  - **reachability:** none moved. The live side was already at zero when this cycle opened.
  - **instrument-correction:** the B15 region-end fix. `gate_skipped_lines` 91,053 → 90,433 —
    **620 lines of shipping code returned to the scanned surface** across 3 files, and the count
    did not move off zero. This closes nothing and is never to be reported as progress
    (`instrument-correction-is-not-closure`).
- **Refused tokens:** none. `token_coverage.py --check` → `refused=142 refused_non_done=0`; every
  refused token sits on a unit already `DONE` under the sheet rule (it renders as words). Zero
  units were in this cycle's scope, so no `deferral` event is owed.
- **Discoveries:** one, and it is instrument-shaped, not mechanism- or scope-shaped: a
  `#[cfg(test)]` region end computed by a line-level brace counter can silently blank shipping
  code. Emitted as `correction 1789337005325` and fixed in this cycle rather than filed. No new
  token type, kind, or remaining-step category surfaced; `token-coverage.json` and the atlas
  predicted this cycle exactly.
- **Figures + their re-derive commands:**

  | figure | value | command | denominator |
  |---|---|---|---|
  | live PCGen surface, closure mode | `live_files=0 live_hits=0 verdict=PASS` | `python3 scripts/pcgen_residue_gate.py --check --closure` | the 5 live roots, `.rs .ts .tsx .js .jsx .mjs .cjs`, comments and `#[cfg(test)]` excluded |
  | independent census | `live_files=0 live_hits=0`, `files_with_boundary_mismatch=0` | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-004_cycle1_independent_residue_census.py` | same population, different algorithm |
  | gate lines skipped by B15 | `b15_gate_skipped_lines_before=91053` → `b15_gate_skipped_lines_after=90433` (independent agrees: `independent_skipped_lines=90433`) | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-004_cycle1_b15_skipline_delta.py` | `files_with_cfg_test_item=200` — the live files carrying a `#[cfg(test)]` item, same run |
  | shipping lines the old gate could hide | `transaction_rs_shipping_lines_hidden_before=618` (`total_shipping_lines_hidden_before=620` over `files_hiding_shipping_lines_before=3`) | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-004_cycle1_b15_skipline_delta.py` | `transaction_rs_total_lines=2885`, same run |
  | gate unit tests | Ran **29** tests, OK (27 before, +2 this cycle) | `python3 -m unittest scripts.tests.test_pcgen_residue_gate -v` | the gate's own suite |
  | `pcgen` references in the desktop crate | **166**, all comment or `#[cfg(test)]` | `grep -rniE 'pcgen' apps/desktop/src-tauri/src apps/desktop/src --include=*.rs --include=*.ts --include=*.tsx \| wc -l` | the desktop crate + frontend |
  | tool-side function bodies lost | **0** (536 → 1,652 definitions, 39 → 88 files) | the fn/def name census quoted above, `git ls-tree` at `c3500e7984` vs `HEAD` | `src/pcgen_import` + `scripts/oracle_harness` + `src/oracle_validation` |
  | oracle parity at HEAD | 156 / 154 / 2 lines, 382 / 376 / 6 chassis, 29 characters | `sheet_rule_parity --roster <roster> --output <ours>` then `python3 scripts/oracle_harness/sheet_parity.py compare --ours <ours> --exports <exports> --output <out>` | the 29-character fixture roster at `artifacts/epic-2-sheet-rule/oracle-parity/roster`, pin `7f818006e3` |
  | converter `--check` | exit 0, 8 kinds, `refused` unchanged | `cargo run --locked --bin sheet_rule_convert -- --check` | `data/sheet_rules/` |
  | ingest tokens shipped in `data/sheet_rules/` | **0** | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` | every converted rule file |

- **Build scope verified:** run at `96fa840c6b` + this cycle's working tree. **No Rust source
  changed this cycle** (`rust_lines_changed=0`), so the compiled surface is `96fa840c6b`'s,
  re-proved rather than assumed, all on a cold `CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E6-004`:
  `cargo build --locked --bin sheet_rule_convert --bin gen_book_cache` → `TOOLSIDE_EXIT=0`;
  `cargo build --locked --release --bin sheet_rule_parity` → `PARITY_BUILD_EXIT=0`;
  `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`, **419 test executables** linked;
  `cargo test --locked --lib -j 6` → `LIB_EXIT=0`, `test result: ok. 3390 passed; 0 failed; 16
  ignored`. `cargo test --no-fail-fast` is **not** re-run: its trigger is "src/ or the classifier
  changed" and neither did. `cargo clippy` is not run: it has no
  input this cycle, since the diff is three files and none of them is Rust. The desktop crate and
  frontend run at the epic wrap-up (`apps/` untouched here). Gates run at HEAD:
  `completion_atlas.py --check` → `missing_clearing_mechanisms=0 citation_failures=0`;
  `token_coverage.py --check` → `verdict=PASS`; `shape_engine_boundary.py --check` →
  `magnitude_bearing=26396 not_held_by_engine=0`; `missing_engine_tables.py --check` →
  `population=0 citation_failures=0`; `denominator_gate.py --check` → `files_checked=148
  violations=0`; `scripts/verify.sh --only pi-sweep` → `PASS (11 hits, 11 baseline rows)`.
- **Sweep population:** N/A — no corpus record changed (`git diff --name-only` touches nothing
  under `data/corpus/`), so `corpus_literal_sweep` is not re-run.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`, unchanged since
  `c3500e7984`.
- **Status:** complete.
- **Notes:** the compile and test evidence carries no information *about this diff* — three
  non-Rust files — and is quoted because a closure cycle should not certify a tree it never built.
  One judgment call: `cargo tree` is reported for what it can actually settle (no converter
  **crate** in the desktop dependency graph) rather than for what the Evidence sentence reads as
  (no converter **module**), because `src/pcgen_import/` is a module of the `codex` crate and no
  crate-grained tool can separate the two. The module-level claim is carried by the gate, the
  independent census and the desktop-crate grep instead.
- **Next-cycle scope:** criterion at zero. Epic 6 is `complete`; Epic 7 (`AT-35-E7-001`) is the
  next dispatch, and it re-runs `pcgen_residue_gate.py --check --closure` at HEAD as one of its
  own gates — which is now also the `pcgen-residue-gate` stage's default mode.
