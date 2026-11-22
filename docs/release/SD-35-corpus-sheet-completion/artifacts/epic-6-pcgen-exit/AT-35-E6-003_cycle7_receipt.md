# Cycle 7 — Epic 6 (PCGen exit) / AT-35-E6-003

Cycle 6 handed this cycle a list of five structural readers and a converter blocker in front of
`feat_catalog.rs`. This cycle found the blocker was in front of the wrong door: a compiled-table
catalog does not need the converter to reach zero residue, because the ruling forbids reading the
ingest **format** on the live side, not serving a stored string that is already the record's plain
words. `equipment_catalog.rs` took the whole batch on that shape and reached **zero**, gaining
**620** descriptions rather than losing any. `root apps/desktop` **7 files / 204 hits → 6 / 179**.

- **Commit SHA:** `e9ba387746` — `equipment_catalog.rs`, the reference-library blocker artifact,
  this cycle's four retro events, and the atlas `derived_at` stamp the gate run rewrote. Cycle
  start `308879390a`. This receipt, the `progress.md` entry and the `kanban.md` row ride the
  following commit — a receipt cannot name the commit that carries it.
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
- **Files touched:** **8 tracked paths. No generated package, no corpus record, no inventory.**
  - **`apps/desktop/src-tauri/src/equipment_catalog.rs`** — `serve_description` (and with it the
    `render_pcgen_desc` import) deleted; `row_description` + `converted_description` +
    `corpus_book_dir` / `corpus_book_dir_opt` added and wired into all 12 mappers; 21 ingest-format
    mentions in doc and test comments restated over our own schema; 22 count assertions this
    cycle's own change moved, each re-derived; the new
    `converted_equipment_prose_population` ratchet.
  - **`…/artifacts/epic-6-pcgen-exit/AT-35-E6-003_cycle7_reference-library-blocker.md`** (new) and
    **`…/AT-35-E6-003_cycle7_reference_library_census.py`** (new — the census that produced every
    figure in it, reproducing `converted_prose.rs`'s four-step join and
    `reference_library_catalog.rs`'s three tiers, so the measurement is re-runnable rather than
    quoted), plus this receipt, the `progress.md` entry and the `kanban.md` row (row 53).
  - **`docs/retro/events/at-35-e6-003.jsonl`** (+4), **`docs/retro/events/sd31-transcribe.jsonl`**
    (+1, this cycle's own `verify.sh --only pi-sweep` run),
    **`docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`** (the
    `derived_at` stamp `completion_atlas.py --check` rewrites).
  - **Zero `src/` changes. Zero `scripts/` changes. Zero `tests/` changes.**
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
  git diff --unified=0 308879390a -- $SC ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -cE '^\+.*\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'      -> 0
  ```
- **Wired-integration audit result:** **OK_NO_TOKENS.**
  ```bash
  git diff --unified=0 308879390a -- $SC ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -ciE '^\+.*\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' -> 0
  ```
  `placeholder` appeared once on an added line — a doc comment promising the field is never a
  fabricated one — and was reworded to `stand-in` rather than annotated, so the cycle diff carries
  the word **zero** times.
- **Acceptance criterion** (verbatim, `epic-breakdown.md`):

  > ### AT-35-E6-003 — the desktop crate and the prose renderer leave PCGen behind
  >
  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers of
  > `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc` is
  > deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  **Not met.** `apps/desktop/` is at 6 files / 179 hits, not zero. The desktop suite is green
  (`578 passed; 0 failed`) and the 19 on-screen tests pass; the zero-hits half is what remains.
- **Receipt rows (mechanical):**
  ```
  python3 scripts/cycle_scope_gate.py --receipt --since 308879390a1df65a75c2c25d4af79b5283f31ec8 \
    --before /tmp/wi-before-AT-35-E6-003.json --after docs/work-inventory.json
  since=308879390a1df65a75c2c25d4af79b5283f31ec8 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=245 ratio=n/a builds_recorded=1 pcgen_live_files=202
  ```
  `closed=0` is correct and by design (`decisions.md §2`): Epic 6 moves no unit.
  `pcgen_live_files` **fell** 203 → 202.
- **PCGen residue:** `python3 scripts/pcgen_residue_gate.py --check`
  ```
  root src/rules_core files=196 hits=11414
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=6 hits=179
  identifier_files=11 identifier_hits=115
  live_files=202 live_hits=11593 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  At cycle start: `root apps/desktop files=7 hits=204`, `identifier_files=12
  identifier_hits=120`, `live_files=203 live_hits=11618`. **−1 file, −25 hits, never above the
  baseline.**
- **Oracle parity:** N/A — no `Number` mapping row was added, no mapping-table row changed, and
  `data/sheet_rules/` was not regenerated (no converter file changed; `git status --porcelain --
  data/sheet_rules src/pcgen_import` empty at every checkpoint). The live-path change is a
  *source* change — which record the catalog asks for its words — and is proved by the
  corpus-wide census in **Movement** below rather than by a per-record comparison.
- **Movement, four buckets:**
  - **closure (into DONE):** none — Epic 6 closes no unit by design.
  - **relabel:** none.
  - **reachability:** on the Equipment Catalog screen, of **8,119** served rows, described rows
    **4,769 → 5,389, +620.** Per book: `CRB` 2218 → 2647, `UE` 448 → 573, `ISG` 97 → 139,
    `MYTHIC` 97 → 116, `UC` 102 → 105, `BB` 13 → 15, `AG` 18 → 19 (**+621 across the seven that
    rose**); `ACG` 312 → 307, `UPSI` 406 → 403, `UW` 57 → 56 (**−9 across the three that fell**);
    fourteen books unchanged, and the books not individually pinned account for the remaining +8.
    The 9 that fell are rows the converted package holds under no rule **and** whose compiled-table
    string still carries an unresolved marker, so the old path showed a half-rendered sentence and
    the new path shows nothing — the same disposition the rest of the crate takes. **Counts, not
    id-set diffs; the ratchet is a floor, not an identity.**
  - **instrument-correction:** one, recorded as a `correction` event — cycle 6's next-cycle scope
    read the compiled-table catalogs as blocked on converter coverage (`core_essentials`, 11 rows;
    9 `Pu` rows). They are not: a stored string that is already plain words is not an ingest-format
    read, and keeping it costs nothing and loses nothing.
- **Refused tokens:** **179 hits across 6 `apps/desktop` files**, by pattern —
  `` `DESC:` ``=48, `` `PRE[A-Z]+:` ``=42, `` `BONUS:` ``=26, `` `render_pcgen_desc` ``=25,
  `` `raw_tokens` ``=24, `` `raw_bonus_chains` ``=10, `` `TYPE=` ``=4; by file —
  `companion_catalog.rs`=52, `race_trait_picker.rs`=33, `feat_catalog.rs`=30,
  `intelligent_item_catalog.rs`=28, `raceCreationCoverage.test.ts`=21,
  `reference_library_catalog.rs`=15. Both partitions sum to 179 of 179, and both agree with the
  gate's own `root apps/desktop files=6 hits=179`. **7 distinct token types** — under
  `workflow-instruction.md §8`'s limit of 10. Recorded as a `deferral` event with its reason, and
  a `correction` event against that event's own first (un-re-derived) partition.

  `reference_library_catalog.rs` is refused in a second sense: it was **measured and refused**,
  not merely skipped. Swapping it costs 1,150 of the 9,679 descriptions it serves — 489 corpus
  records that are in no `docs/work-inventory.json` unit, 563 that resolve book-exact to a rule
  stating nothing, 81 by source row, 17 by name. Full measurement in
  `AT-35-E6-003_cycle7_reference-library-blocker.md`; a second `deferral` event names it.
- **Discoveries:** **three.**
  1. **A compiled-table catalog reaches zero residue without any converter work.** §11 forbids
     reading a PCGen token, formula string or `raw_tokens` on the live side. A compiled table's
     stored string that carries none of those is English, and serving it is not a violation —
     *parsing* it at run time was. `row_description` takes the converted record first and keeps
     the stored words only when they need no rewriting. This is the pattern the next four files
     use, and it removes `core_essentials` from the critical path entirely.
  2. **The reference library is a different shape and does not take that pattern**, because its
     tier 3 *is* a token dump rather than a stored sentence. Measured, not assumed: 1,150 rows.
  3. **A renderer for a converted rule's stated facts closes 3,488 of the 4,523 tier-3 rows** —
     built and measured this cycle, and **deliberately not shipped**: with nothing consuming it
     it would be dead code, and the file it was built for does not clear the bar anyway. It
     belongs in the cycle that swaps that file.
- **Figures + their re-derive commands:**
  | figure | denominator | command |
  |---|---|---|
  | `apps/desktop` 6 files / 179 hits (from 7 / 204); `live_files` 202 (from 203) | the live roots' source files | `python3 scripts/pcgen_residue_gate.py --check` |
  | 179 hits by token type and by file, both summing to 179 | the same 6 files | `python3 -c "import importlib.util,os,re,collections;spec=importlib.util.spec_from_file_location('g','scripts/pcgen_residue_gate.py');g=importlib.util.module_from_spec(spec);spec.loader.exec_module(g);pats={**g.IDENTIFIER_PATTERNS,**g.TOKEN_SYNTAX_PATTERNS};res=collections.defaultdict(collections.Counter);[res[os.path.join(r,f)].update({n:len(re.findall(p,open(os.path.join(r,f),errors='replace').read()))for n,p in pats.items()})for r,d,fs in os.walk('apps/desktop')if not set(r.split(os.sep))&g.EXCLUDED_DIR_NAMES for f in fs if os.path.splitext(f)[1] in g.SOURCE_EXTENSIONS];print({k:{a:b for a,b in v.items() if b}for k,v in res.items() if sum(v.values())})"` |
  | equipment catalog 8,119 rows served, 5,389 with a description, and the per-book split | rows in `build_equipment_catalog()` | `cd apps/desktop/src-tauri && cargo test --locked -j 6 converted_equipment_prose_population -- --nocapture` |
  | 4,769 described before the swap | the same 8,119 rows | `git show 308879390a:apps/desktop/src-tauri/src/equipment_catalog.rs \| grep -A 3 'description.is_some()).count()'` — the pre-swap value of `description_coverage_is_pinned_per_book`'s own total assertion |
  | 8,119 rows / old 4,769 / converted-only 4,964 / +631 gained / −436 lost by the **converted-only** lens (before the clean-stored-string fallback was added) | the same 8,119 rows | measured by a `converted_equipment_prose_census` test that existed only in this cycle's working tree and was replaced by the ratchet before the commit; the numbers are **not re-derivable from any committed tree** and are recorded here as the measurement that justified adding the fallback. `git log -1 --stat e9ba387746 -- apps/desktop/src-tauri/src/equipment_catalog.rs` names the commit that replaced it |
  | reference library 9,697 records, 9,679 served today, 8,534 under the converted package, 1,150 lost split 489 no_rule / 563 book_exact / 81 source_row / 17 by_name | every record under the twelve `REFERENCE_LIBRARY_KIND_DIRS` | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003_cycle7_reference_library_census.py` |
  | `data/sheet_rules/` source markers: 0 | every generated rule file | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
- **Build scope verified:** run at `e9ba387746`, with `CARGO_TARGET_DIR` per tree
  (`/tmp/cargo-sd35-AT-35-E6-003` for the root workspace, `…-desktop` for the desktop crate —
  `AGENTS.md`: one directory per agent **per source tree**).
  ```
  cargo test --locked --no-run -j 6                  NO_RUN_EXIT=0 (every test binary linked)
  cargo test --locked --lib -j 6                     ok. 3299 passed; 0 failed; 15 ignored
  cargo test --locked --no-fail-fast -j 6            414 targets, 8810 passed, 0 failed, 68 ignored, 0 FAILED suites, 0 `^error` lines — identical to cycle 6 on every field
  cd apps/desktop/src-tauri && cargo test --locked -j 6
                                                     ok. 578 passed; 0 failed; 0 ignored
  cd apps/desktop/src-tauri && cargo clippy --locked --tests -j 6
                                                     0 warnings, 0 errors (one `needless_borrow`
                                                     this cycle raised, fixed in the same cycle)
  cargo clippy --locked --tests -j 6 (root workspace) 0 warnings, 0 errors
  python3 scripts/pcgen_residue_gate.py --check      verdict=PASS (live_hits 11618 -> 11593)
  grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l      0
  python3 scripts/completion_atlas.py --check        missing_clearing_mechanisms=0 citation_failures=0
  python3 scripts/token_coverage.py --check          non_done=0 refused=142 refused_non_done=0 shapes=1 verdict=PASS
  python3 scripts/shape_engine_boundary.py --check   magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True
  python3 scripts/missing_engine_tables.py --check   population=0 kinds=0 citation_failures=0
  python3 scripts/denominator_gate.py --check '…/*.md' '…/artifacts/**/*.md'      files_checked=102 violations=0
  scripts/verify.sh --only pi-sweep                  RESULT: PASS
  ```
  `sheet_rule_convert -- --check` was **not** re-run: no converter file and no corpus record
  changed (`git status --porcelain -- src/pcgen_import data/corpus data/sheet_rules` empty at
  every checkpoint), and the standing source-marker grep over the unchanged package reads 0.
  `corpus_literal_sweep` was **not** run, for the same reason. `v06_work_inventory` was **not**
  re-run: its inputs are unchanged and the inventory is already at `DONE 49438 of 49438`. The
  frontend suite was **not** run: no file under `apps/desktop/src/` changed.
- **Sweep population:** N/A — no corpus record changed.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`. No figure in this
  receipt is derived from the pinned corpus.
- **Status:** **partial**
- **What this proof does not cover** (`AGENTS.md` rule 7): the equipment swap is proved by
  **counts** of rows carrying text before and after, per book, plus the crate's cross-catalog
  leak sweeps and its own on-screen tests — **not** by a served-id-set diff, so a one-for-one
  swap inside a book's total would read as no change. The 9 rows that fell are identified by the
  per-book deltas, not by key. `row_description`'s refusal test is a single-character check for an
  unresolved marker over text that is otherwise treated as English; a stored string carrying some
  *other* ingest-format residue would pass it, and only the crate's `leaked_pcgen_syntax` sweep
  (which runs over every served description and is green) rules that out. Nothing here proves
  anything about the other five files.
- **Notes:** `reference_library_catalog.rs` was measured and **refused**, not skipped — see its
  artifact. The facts renderer built for it was reverted rather than shipped unused.
- **Next-cycle scope:** **AT-35-E6-003 cycle 8.** (1) `feat_catalog.rs` (30) and
  `companion_catalog.rs` (52) on this cycle's `row_description` shape — neither needs converter
  work, and `core_essentials` is off the critical path; (2) `intelligent_item_catalog.rs` (28) and
  `race_trait_picker.rs` (33), which read `raw_tokens` structurally and need `SheetRule.applies` /
  `grants` rather than a description source; (3)
  `apps/desktop/src/characterHub/raceCreationCoverage.test.ts` (21); (4)
  `reference_library_catalog.rs` (15) **last**, with the facts renderer and the inventory question
  its artifact names. Scope flags:
  `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero units by design, decisions.md §2)`.
  Target: `root apps/desktop files=0 hits=0`.
