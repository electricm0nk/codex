# Cycle 8 — Epic 6 (PCGen exit) / AT-35-E6-003

Cycle 7 handed this cycle a named remainder of **179 hits across 6 `apps/desktop` files** and a
plan that put the two prose-shaped readers first. Both took the swap and both reached **zero**:
`feat_catalog.rs` (30 hits) and `companion_catalog.rs` (52). `root apps/desktop` **6 files / 179
hits → 4 / 97**. The swap gained text rather than costing it: **+1** feat description and **+156**
companion abilities that now show a reader something.

The four files left all read the ingest format **structurally** — `raw_tokens`,
`raw_bonus_chains`, `PRE`-token parsing — not for prose, so none of them takes this shape. That is
this cycle's named remainder, and it is one mechanism, not four.

- **Commit SHA:** `526173470e` — `companion_catalog.rs`, `reach_gate.rs` and the four frontend
  files. `5a3a67c2dd` before it carries `feat_catalog.rs`. Cycle start `d96f36be6d`. This
  receipt, the `progress.md` entry, the `kanban.md` row and this cycle's two retro events ride
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
- **Files touched:** **7 tracked source paths, plus this receipt and the board rows. No
  generated package, no corpus record, no inventory, no `src/`, no `scripts/`.**
  - **`apps/desktop/src-tauri/src/feat_catalog.rs`** — `row_description` +
    `corpus_book_dir(RuleSetId)` added and wired into `map_catalog_entry` and
    `feat_description_by_exact_name`; the run-time rewriter call sites deleted;
    `feat_descriptions_are_rendered_and_otherwise_byte_identical` (a 340-line test pinning that
    rewriter's output) replaced by `converted_feat_prose_population`, a per-book ratchet plus a
    leak sweep, and two new tests; four pinned-text assertions re-derived against the converted
    words; two ingest-format mentions in comments restated over our own schema.
  - **`apps/desktop/src-tauri/src/companion_catalog.rs`** — `serve_ability_description` swapped
    to the converted package; `render_desc_token`, `serve_desc_condition`, `spell_out_variable`,
    `spell_out_alignment`, `serve_desc_variant`, `serve_desc_variants` and
    `CompanionDescriptionVariantDto` **deleted**; the `description_variants` wire field removed;
    `residual_is_structurally_explained`'s reason 3 restated over the converted package (it was
    reproducing a gate `companion_pool_catalog.rs` retired in cycle 3); two tests replaced by
    `converted_companion_prose_population`; 24 ingest-format mentions in doc and test comments
    restated over our own schema.
  - **`apps/desktop/src-tauri/src/reach_gate.rs`** — the companion payload predicate loses its
    `description_variants` clause in both places it appears: one field now answers what two used
    to.
  - **`apps/desktop/src/boundary/loadCompanionCatalog.ts`**,
    **`…/companionCatalog/CompanionCatalogScreen.tsx`**,
    **`…/companionCatalog/companionCatalogRuntime.ts`**,
    **`…/companionCatalog/CompanionCatalogScreen.test.ts`** — the `descriptionVariants` DTO,
    its interface, its render block and its four fixture rows removed.
  - **`docs/retro/events/at-35-e6-003.jsonl`** (+2: one `correction`, one `deferral`), this
    receipt, the `progress.md` entry and the `kanban.md` row (row 54).
  - **Not committed, and deliberately:** `.worktrees/ci-trait-choice` shows in an unfiltered
    `git status --porcelain` as untracked. It is a **git worktree** — another checkout of this
    repository — present before this cycle started. Committing it would commit a second tree.
- **Identifier audit result:** **OK_NO_BUNDLE_TAGS for this cycle; 28 matches against the develop
  base, pre-existing and not this cycle's.**
  ```bash
  BASE=fe5ae6cd4a5f3c65d5d10f4d523f00e33b04ac47      # git merge-base HEAD origin/develop
  SC="src/rules_core src/pcgen_import apps/desktop/src-tauri/src apps/desktop/src \
      docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit"
  git diff --unified=0 "${BASE}...HEAD" -- $SC ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -cE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'            -> 28
  git diff --unified=0 d96f36be6d -- $SC ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -cE '^\+.*\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'      -> 0
  ```
- **Wired-integration audit result:** **OK_NO_TOKENS.**
  ```bash
  git diff --unified=0 d96f36be6d -- $SC ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -ciE '^\+.*\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' -> 0
  ```
  `placeholder` appeared once on an added line — a doc comment describing what the retired
  renderer used to delete — and was reworded to `a scaling term` rather than annotated, so the
  cycle diff carries the word **zero** times. Self-healed under `workflow-instruction.md §8`'s
  single-token clause, in the same cycle.
- **Acceptance criterion** (verbatim, `epic-breakdown.md`):

  > ### AT-35-E6-003 — the desktop crate and the prose renderer leave PCGen behind
  >
  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers of
  > `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc` is
  > deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  **Not met.** `apps/desktop/` is at 4 files / 97 hits, not zero. The desktop suite is green
  (`578 passed; 0 failed`), the frontend suite is green (`101/101 test files passed`) and the 19
  on-screen tests pass; the zero-hits half is what remains.
- **Receipt rows (mechanical):**
  ```
  CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E6-003 \
  python3 scripts/cycle_scope_gate.py --receipt --since d96f36be6dea4b8e0e7d108d2d3b7b2e50336fdb \
    --before /tmp/wi-before-AT-35-E6-003.json --after docs/work-inventory.json
  since=d96f36be6dea4b8e0e7d108d2d3b7b2e50336fdb target_dir=/tmp/cargo-sd35-AT-35-E6-003 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=1274 ratio=n/a builds_recorded=2 pcgen_live_files=200
  ```
  `closed=0` is correct and by design (`decisions.md §2`): Epic 6 moves no unit.
  `pcgen_live_files` **fell** 202 → 200. `builds_recorded=2` counts the root workspace's target
  directory only — one session for `--no-run` + `--lib` + `--no-fail-fast`, one for
  `sheet_rule_convert --check` + `clippy`. The desktop crate builds in its own directory
  (`/tmp/cargo-sd35-AT-35-E6-003-desktop`, `AGENTS.md`: one directory per agent **per source
  tree**) and is not counted by this row.
- **PCGen residue:** `python3 scripts/pcgen_residue_gate.py --check`
  ```
  root src/rules_core files=196 hits=11414
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=4 hits=97
  identifier_files=9 identifier_hits=98
  live_files=200 live_hits=11511 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  At cycle start: `root apps/desktop files=6 hits=179`, `identifier_files=11
  identifier_hits=115`, `live_files=202 live_hits=11593`. **−2 files, −82 hits, never above the
  baseline.**
- **Oracle parity:** N/A — no `Number` mapping row was added, no mapping-table row changed, and
  `data/sheet_rules/` was not regenerated (no converter file and no corpus record changed;
  `git status --porcelain -- src/pcgen_import data/corpus data/sheet_rules` empty at every
  checkpoint). `sheet_rule_convert -- --check` was run anyway and exits 0 over the unchanged
  package. The live-path change is a *source* change — which record a catalog asks for its words
  — and is proved by the corpus-wide before/after censuses in **Movement** below rather than by
  a per-record comparison.
- **Movement, four buckets:**
  - **closure (into DONE):** none — Epic 6 closes no unit by design.
  - **relabel:** none.
  - **reachability:** two gains, both measured corpus-wide.
    - **Feats.** Of **2,227** served rows, described rows **2,161 → 2,162, +1**. The count
      understates the change: the converter joins a record's descriptive fields, so rows that
      served only their first sentence now serve the feat's actual benefit. Three pinned
      examples, each re-derived and now asserted: ARG's `Angel Wings` gains its fly speed and
      manoeuvrability clause; APG's `Extra Hex` gains "You gain one additional hex…"; ACG's
      `Extra Panache` gains its two-points-per-day clause and its stacking note. **Counts, not
      id-set diffs; the ratchet is a floor, not an identity.**
    - **Companion abilities.** Of **3,570** served rows, rows that show a reader text
      **2,824 → 2,980, +156.** Before the swap, 2,963 rows carried `Some(..)` — but **152** of
      those were the empty string, an empty paragraph on screen, so only 2,811 showed anything,
      plus 13 more whose text lived only in `descriptionVariants`. The DC the retired renderer
      deleted for want of a character to settle it is now printed as the rule's own words
      (`decisions.md §1` form 3): Ultimate Wilderness's `Spitting Cobra ~ Poison` read
      "…save Fort DC ;" and now reads "…Fort DC 10 plus hit dice divided by 2 plus Constitution
      modifier."
  - **instrument-correction:** one, recorded as a `correction` event — the converter renders
    `mythic_adventures:feat:prophetic_visionary` as "…increases by a rules variable%", a literal
    percent sign left against a letter, which this crate's own sweep reads as a gap. Named in
    `feat_catalog::row_description`'s doc comment, refused at the live boundary rather than
    exempted from the crate-wide sweep, and reported here as a **converter-side** finding.
- **Refused tokens:** **97 hits across 4 `apps/desktop` files**, by pattern —
  `PRE[A-Z]+:`=27, `raw_tokens`=24, `DESC:`=14, `BONUS:`=11, `raw_bonus_chains`=10,
  `render_pcgen_desc`=8, `TYPE=`=3; by file — `race_trait_picker.rs`=33,
  `intelligent_item_catalog.rs`=28, `raceCreationCoverage.test.ts`=21,
  `reference_library_catalog.rs`=15. Both partitions sum to 97 of 97, and both agree with the
  gate's own `root apps/desktop files=4 hits=97`. **7 distinct token types** — under
  `workflow-instruction.md §8`'s limit of 10. Recorded as a `deferral` event with its reason.

  The refusal is **one mechanism, not four files**: each of the four reads the ingest format
  *structurally* rather than for prose — `race_trait_picker.rs` reads `raw_tokens` to find an
  alternate's self-exclusion flags, `intelligent_item_catalog.rs` reads `raw_tokens` and
  `raw_bonus_chains` to build a component's mechanics, `raceCreationCoverage.test.ts` reads both
  to re-derive a race's ability adjustments, and `reference_library_catalog.rs`'s third content
  tier *is* a token dump. None of them takes the description-swap shape the two files this cycle
  closed did. They need **a renderer for a converted rule's stated facts** — `value` + `target` +
  `bonus_type`, `grants`, `offers`, `applies`, `tags` — which `AT-35-E6-003` cycle 7 built,
  measured (it closes 3,488 of `reference_library_catalog.rs`'s 4,523 tier-3 rows) and
  deliberately did not ship because nothing consumed it yet. Cycle 9 has four consumers for it.
  `reference_library_catalog.rs` also still carries cycle 7's measured refusal (1,150
  descriptions lost by a naive swap), unchanged by this cycle.
- **Discoveries:** **three.**
  1. **The converted package states a conditional rules family better than the run-time path
     did, and that is what let the wire shrink.** A companion ability whose text is stated once
     per condition arrives as ONE prose block, each variant under the condition that selects it
     — and carrying the save DC the run-time renderer deleted. So `descriptionVariants` was not
     ported, it was **removed**: leaving it would have printed every variant twice on screen,
     once inside `description` and once below it. The discovery is that a converter this good
     removes wire fields rather than filling them, and the reach gate's own payload predicate
     lost a clause for the same reason.
  2. **A "served description" count is not a "row that shows text" count.** 152 companion
     abilities carried `Some("")` before the swap. Every population figure in this cycle is
     therefore counted on non-empty text, and the ratchets assert non-emptiness per row rather
     than counting `Option::is_some`. A cycle that had reported `2,963 → 2,980, +17` would have
     been arithmetically correct and materially wrong.
  3. **`leaked_pcgen_syntax` is the right refusal predicate for a compiled-table fallback, and a
     bare `contains('%')` is not.** Cycle 7's `equipment_catalog::row_description` used the
     latter, which throws away real English percentages — a rulebook sentence naming a chance or a
     reduction as a per-cent figure reads as an unresolved marker to it. The
     feat tables carry those, so this cycle's fallback uses the crate's own predicate — a
     refusal, not a reader — which exempts a digit-preceded sign while still refusing a
     positional or keyword marker and a bare `%`. `equipment_catalog.rs` still uses the narrower
     test and may be losing rows to it; **not measured this cycle**, and named here so cycle 9
     measures rather than assumes.
- **Figures + their re-derive commands:**
  | figure | denominator | command |
  |---|---|---|
  | `apps/desktop` 4 files / 97 hits (from 6 / 179); `live_files` 200 (from 202); `live_hits` 11,511 (from 11,593) | the live roots' source files | `python3 scripts/pcgen_residue_gate.py --check` |
  | 97 hits by token type and by file, both summing to 97 | the same 4 files | `python3 -c "import importlib.util,os,re,collections;spec=importlib.util.spec_from_file_location('g','scripts/pcgen_residue_gate.py');g=importlib.util.module_from_spec(spec);spec.loader.exec_module(g);pats={**g.IDENTIFIER_PATTERNS,**g.TOKEN_SYNTAX_PATTERNS};res=collections.defaultdict(collections.Counter);[res[os.path.join(r,f)].update({n:len(re.findall(p,open(os.path.join(r,f),errors='replace').read()))for n,p in pats.items()})for r,d,fs in os.walk('apps/desktop')if not set(r.split(os.sep))&g.EXCLUDED_DIR_NAMES for f in fs if os.path.splitext(f)[1] in g.SOURCE_EXTENSIONS];print({k:{a:b for a,b in v.items() if b}for k,v in res.items() if sum(v.values())})"` |
  | feat catalog 2,227 rows served, 2,162 with a description, and the per-book split | rows in `build_feat_catalog()` | `cd apps/desktop/src-tauri && cargo test --locked -j 6 converted_feat_prose_population -- --nocapture` |
  | 2,161 described before the swap | the same 2,227 rows | `git show d96f36be6d:apps/desktop/src-tauri/src/feat_catalog.rs \| grep 'assert_eq!(with_description'` — the pre-swap value of `feat_descriptions_are_rendered_and_otherwise_byte_identical`'s own total assertion |
  | companion catalog 3,570 abilities served, 2,980 with non-empty text | abilities in `build_companion_catalog()` | `cd apps/desktop/src-tauri && cargo test --locked -j 6 converted_companion_prose_population -- --nocapture` |
  | 2,824 with text before the swap (2,811 non-empty `description` + 13 variants-only), and 2,963 `Some(..)` of which 152 empty | the same 3,570 abilities | measured at this cycle's own intermediate commit `5a3a67c2dd` in a throwaway `git worktree` (`/tmp/sd35-c8-before`, removed after), by adding four counters and a `println!` to `no_served_description_leaks_pcgen_syntax`. **Not re-derivable from any committed tree** — the commit it was measured at is committed, the four-line instrumentation was not. Re-derive by `git worktree add <dir> 5a3a67c2dd`, printing `described`, and counting per ability `description` non-empty / `description_variants` non-empty |
  | the one converter finding, `mythic_adventures:feat:prophetic_visionary` | every served feat description | `cd apps/desktop/src-tauri && cargo test --locked -j 6 converted_feat_prose_population -- --nocapture` prints a `leak …` line per offending row; zero after `row_description`'s refusal |
  | `data/sheet_rules/` source markers: 0 | every generated rule file | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | rust_lines_changed 1,274 | `*.rs` since `d96f36be6d` | `CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E6-003 python3 scripts/cycle_scope_gate.py --receipt --since d96f36be6d --before /tmp/wi-before-AT-35-E6-003.json --after docs/work-inventory.json` |
- **Build scope verified:** run at `526173470e`, with `CARGO_TARGET_DIR` per tree
  (`/tmp/cargo-sd35-AT-35-E6-003` for the root workspace, `…-desktop` for the desktop crate —
  `AGENTS.md`: one directory per agent **per source tree**).
  ```
  cargo test --locked --no-run -j 6                  NO_RUN_EXIT=0 (every test binary linked)
  cargo test --locked --lib -j 6                     ok. 3299 passed; 0 failed; 15 ignored
  cargo test --locked --no-fail-fast -j 6            413 targets, 8810 passed, 0 failed, 68 ignored,
                                                     0 FAILED suites — identical to cycle 7 on every field
  cargo clippy --locked --tests -j 6 (root)          0 warnings, 0 errors
  cargo run --locked --bin sheet_rule_convert -- --check   EXIT=0 over the unchanged package
  cd apps/desktop/src-tauri && cargo test --locked -j 6
                                                     ok. 578 passed; 0 failed; 0 ignored
  cd apps/desktop/src-tauri && cargo clippy --locked --tests -j 6
                                                     0 warnings, 0 errors
  cd apps/desktop && npm run -s typecheck            EXIT=0
  cd apps/desktop && npm run -s test                 101/101 test files passed
  python3 scripts/pcgen_residue_gate.py --check      verdict=PASS (live_hits 11593 -> 11511)
  grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l      0
  python3 scripts/completion_atlas.py --check        DONE 49438 of 49438, missing_clearing_mechanisms=0 citation_failures=0
  python3 scripts/token_coverage.py --check          non_done=0 refused=142 refused_non_done=0 shapes=1 verdict=PASS
  python3 scripts/shape_engine_boundary.py --check   magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True
  python3 scripts/missing_engine_tables.py --check   population=0 kinds=0 citation_failures=0
  python3 scripts/denominator_gate.py --check '…/*.md' '…/artifacts/**/*.md'      files_checked=104 violations=0
  python3 scripts/denominator_gate.py --check-provenance    files_checked=221 figures_examined=549 violations=0
  ./scripts/publish-site-dashboard.sh --check-pin    input pin matches docs/work-inventory.json
  scripts/verify.sh --only pi-sweep                  RESULT: PASS (11 hits, 11 baseline rows)
  ```
  The desktop crate **and** the frontend ran here rather than at the epic wrap-up, because this
  cycle touched `apps/` on both sides (`workflow-instruction.md §6` step 3).
  `corpus_literal_sweep` was **not** run: no corpus record changed
  (`git status --porcelain -- data/corpus` empty at every checkpoint). `v06_work_inventory` was
  **not** re-run: its inputs are unchanged and the inventory is already at `DONE 49438 of 49438`.
- **Sweep population:** N/A — no corpus record changed.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`. No figure in this
  receipt is derived from the pinned corpus.
- **Status:** **partial**
- **What this proof does not cover** (`AGENTS.md` rule 7): both swaps are proved by **counts** of
  rows carrying non-empty text before and after, plus the crate's cross-catalog leak sweep and
  its own on-screen tests — **not** by a served-id-set diff, so a one-for-one swap inside a
  book's total would read as no change. The feat swap has per-book floors, which close that hole
  book by book; the companion swap has a single corpus-wide floor and **does not**, so a gain in
  one book could mask a loss in another there. The three feat rows whose gain is asserted by
  name are pinned; the other rows' gains are counted, not identified. `row_description`'s refusal
  is `leaked_pcgen_syntax` over text otherwise treated as English; a converted rendering carrying
  some *other* residue of the ingest format would pass it, and only the crate's
  `no_catalog_serves_a_description_carrying_raw_pcgen_syntax` sweep (green, over every served
  description in every catalog) rules that out. Nothing here proves anything about the other four
  files, and Discovery 3's suspicion about `equipment_catalog.rs` is stated, not measured.
- **Notes:** `descriptionVariants` was removed from the wire rather than left empty, which is a
  breaking shape change to one Tauri command's response. The only consumers are in this
  repository (`CompanionCatalogScreen.tsx` and its fixtures); both were updated in the same
  commit and the frontend suite and `tsc --noEmit` are green.
- **Next-cycle scope:** **AT-35-E6-003 cycle 9.** Build the **facts renderer** cycle 7 measured
  and did not ship — a converted rule's `value`/`target`/`bonus_type`, `grants`, `offers`,
  `applies` and `tags` in words — then take its four consumers in this order:
  (1) `race_trait_picker.rs` (33), whose four `raw_tokens` spellings of an alternate's
  self-exclusion guard are exactly the `Not { Holds { Fact { name: "<X>_Replace<Y>" } } }` the
  converted `applies` already carries — verified by reading
  `data/sheet_rules/core_rulebook/race_trait/dwarf_ability_scores.json`;
  (2) `intelligent_item_catalog.rs` (28); (3)
  `apps/desktop/src/characterHub/raceCreationCoverage.test.ts` (21), which needs the same facts
  from the package's JSON rather than from corpus `raw_tokens`; (4)
  `reference_library_catalog.rs` (15) **last**, with cycle 7's measured 1,150-description
  blocker and the inventory question its artifact names. Also measure, do not assume, whether
  `equipment_catalog.rs`'s narrower `contains('%')` fallback test is costing it rows
  (Discovery 3). Scope flags:
  `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero units by design, decisions.md §2)`.
  Target: `root apps/desktop files=0 hits=0`.
