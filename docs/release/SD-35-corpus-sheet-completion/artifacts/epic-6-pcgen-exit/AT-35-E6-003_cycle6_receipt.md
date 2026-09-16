# Cycle 6 — Epic 6 (PCGen exit) / AT-35-E6-003

Cycle 5 measured the converter, not converted-row coverage, as what blocks the compiled-table
catalogs, and named four converter items plus two catalogs as this cycle's scope. This cycle
took them in that order: the converter first, then the one catalog the converter unblocked.
`root apps/desktop` **9 files / 219 hits → 7 / 204**, and `monster_catalog.rs` and
`spell_catalog.rs` both reach zero.

- **Commit SHA:** `acfd1f7d99` — the three converter files, the two desktop catalogs, the reach
  gate's recorded finding, the regenerated `data/sheet_rules/` (175 files), the oracle harness's
  variable-name index, and this cycle's three retro events. Cycle start `3fd90ddc3f`. This
  receipt, the `progress.md` entry and the `kanban.md` row ride the following commit — a receipt
  cannot name the commit that carries it.
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
- **Files touched:** **9 tracked paths plus the generated package.**
  - **`src/pcgen_import/sheet_rule/prose.rs`** — `words_for_unlowerable` + `leaf_words` +
    `argument_or_words` (an unlowerable prose argument prints its words instead of deleting the
    row), `normalize_percentile_dice`, `scrub_inline_formula` / `rewrite_inline_formula`, and
    three new unit tests.
  - **`src/pcgen_import/sheet_rule/ctx.rs`** — `RecordCtx::current_under`, so a term that
    degrades inside a row records the same census `under` the row-level `Err` path recorded.
  - **`src/pcgen_import/sheet_rule/convert.rs`** — sets and clears `current_under` around
    `convert_token`.
  - **`apps/desktop/src-tauri/src/spell_catalog.rs`** — `serve_description` reads the converted
    package through `converted_prose`; `corpus_book_dir` is the closed-set book join;
    `converted_spell_prose_population` is the corpus-wide ratchet; six doc/test comments
    restated over our own schema; three assertions this cycle's own change moved.
  - **`apps/desktop/src-tauri/src/monster_catalog.rs`** — the five remaining ingest-format
    mentions (four `BONUS:`, one `PRE…:`) restated over our own schema. **File at zero.**
  - **`apps/desktop/src-tauri/src/reach_gate.rs`** — one `BARE_RECORD_FINDINGS` entry.
  - **`data/sheet_rules/**`** — regenerated whole: **175** files in the commit, of which **167**
    are rule files under `<book>/<kind>/` and 8 are the package's own indexes (2 of them new
    `_defects/` lists, 3 new `_vars/` entries) — plus
    **`scripts/oracle_harness/var_names.json`**, which the converter writes.
    (`git show --numstat acfd1f7d99 -- data/sheet_rules \| awk -F'\t' 'NF==3{n++} END{print n}'`)
  - **`docs/retro/events/at-35-e6-003.jsonl`** (+3).
  - **Zero `scripts/` changes. Zero `tests/` changes. Zero `data/corpus/` changes.**
  - **Not committed, and deliberately:** `.worktrees/ci-trait-choice` shows in an unfiltered
    `git status --porcelain` as untracked. It is a **git worktree** — another checkout of this
    repository — present before this cycle started. Committing it would commit a second tree.
- **Identifier audit result:** **OK_NO_BUNDLE_TAGS for this cycle; 33 matches against the
  develop base, pre-existing and not this cycle's.**
  ```bash
  BASE=fe5ae6cd4a5f3c65d5d10f4d523f00e33b04ac47      # git merge-base HEAD origin/develop
  SC="src/pcgen_import/sheet_rule apps/desktop/src-tauri/src apps/desktop/src src/rules_core \
      docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit"
  git diff --unified=0 "${BASE}...HEAD" -- $SC ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -cE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'            -> 33
  git diff --unified=0 3fd90ddc3f -- $SC ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -cnE '^\+.*\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'      -> 0
  ```
- **Wired-integration audit result:** **OK_NO_TOKENS.**
  ```bash
  git diff --unified=0 3fd90ddc3f -- $SC ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -cniE '^\+.*\b(STUB|MOCK|not yet implemented|todo|fixme|hack)\b' -> 0
  ```
  `placeholder` appeared once on an added line — a doc comment explaining the ingest format's
  positional references — and was reworded to `reference` rather than annotated, so the cycle
  diff carries the word **zero** times.
- **Acceptance criterion** (verbatim, `epic-breakdown.md`):

  > ### AT-35-E6-003 — the desktop crate and the prose renderer leave PCGen behind
  >
  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers of
  > `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc` is
  > deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  **Not met.** `apps/desktop/` is at 7 files / 204 hits, not zero. The desktop suite is green
  (`577 passed; 0 failed`) and the on-screen tests pass; the zero-hits half is what remains.
- **Receipt rows (mechanical):**
  ```
  python3 scripts/cycle_scope_gate.py --receipt --since 3fd90ddc3ffad76c9f37172b6bd098a869d4f8cf \
    --before /tmp/wi-before-AT-35-E6-003.json --after docs/work-inventory.json
  since=3fd90ddc3ffad76c9f37172b6bd098a869d4f8cf residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=537 ratio=n/a builds_recorded=0 pcgen_live_files=203
  ```
  `closed=0` is correct and by design (`decisions.md §2`): Epic 6 moves no unit.
  `pcgen_live_files` **fell** 205 → 203.
- **PCGen residue:** `python3 scripts/pcgen_residue_gate.py --check`
  ```
  root src/rules_core files=196 hits=11414
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=7 hits=204
  identifier_files=12 identifier_hits=120
  live_files=203 live_hits=11618 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  At cycle start: `root apps/desktop files=9 hits=219`, `identifier_files=13
  identifier_hits=123`, `live_files=205 live_hits=11633`. **−2 files, −15 hits, never above the
  baseline.**
- **Oracle parity:** N/A — no `Number` mapping row was added and no mapping-table row changed.
  The converter change is on the **prose** side only: it replaces a dropped row with words and
  never produces a magnitude. Proved package-wide rather than per record by
  `sheet_rule_convert -- --check` (`records=49438 converted=49296 refused=142`, identical to the
  cycle's start; `rules` 69,344 → 69,346 and `var_tables` 5,277 → 5,281, the two extra rules and
  four extra variable tables being arguments the converter now reaches because the argument
  before them no longer aborts the row) and by the source-marker grep over `data/sheet_rules/`
  reading 0.
- **Movement, four buckets:**
  - **closure (into DONE):** none — Epic 6 closes no unit by design.
  - **relabel:** none.
  - **reachability:** on the Spell Catalog screen, of 2,481 served rows: **25 gain** a
    description they never had, **48 stop serving the literal `[redacted PI]` marker**, **1**
    (`ACG :: Discern Next of Kin`) stops serving text the corpus record declares product
    identity (`decisions.md §15` R2), and **2 lose** one (see **Refused tokens**). Described
    rows 2,436 → **2,410**. In the converted package, **167 rule files** changed in the
    regeneration — prose recovered from a refused argument, the two `%` rewrites, and three
    newly-reachable variable references; converted `feat` rules with **no prose at all**
    fell 487 → 477.
    These are **counts**, not id-set diffs; the ratchet is a floor, not an identity.
  - **instrument-correction:** one, recorded as a `correction` event — cycle 5's blocker
    artifact §5 read `core_rulebook:spell:teleport`'s bare `%` as a converted variable escaping;
    it is the source row's own `d %%`, a percentile-dice notation written with a stray space.
- **Refused tokens:** **204 hits across 7 `apps/desktop` files**, by pattern —
  `` `DESC:` ``=62, `` `PRE[A-Z]+:` ``=42, `` `BONUS:` ``=27, `` `render_pcgen_desc` ``=30,
  `` `raw_tokens` ``=24, `` `raw_bonus_chains` ``=10, `` `%CHOICE` ``=5, `` `TYPE=` ``=4;
  by file — `companion_catalog.rs`=52, `race_trait_picker.rs`=33, `feat_catalog.rs`=30,
  `intelligent_item_catalog.rs`=28, `equipment_catalog.rs`=25, `raceCreationCoverage.test.ts`=21,
  `reference_library_catalog.rs`=15. Both partitions sum to 204 of 204, and both agree with the
  gate's own `root apps/desktop files=7 hits=204`. **8 distinct token types** — under
  `workflow-instruction.md §8`'s limit of 10. Recorded as a `deferral` event with its reason.

  Two corpus records are refused in a second sense — they lose a served description and get
  none back: `advanced_players_guide:spell:wall_of_thorms` and
  `mythic_adventures:spell:elemental_body_iiimod`. Both are real corpus records
  (`in_scope`/`full`, `name: null`) that are **not units of `docs/work-inventory.json`**, which
  is the converter's own population (`sheet_rule::load_population` walks the inventory's units),
  so the converted package holds them under no id and no name. Named in
  `reach_gate::BARE_RECORD_FINDINGS`, in `spell_catalog`'s APG null-field assertion, and in a
  `deferral` event; **reported, not excused** — admitting them moves the bundle-wide denominator
  49,438 and is inventory scope, not this criterion's.
- **Discoveries:** **three.**
  1. **Cycle 5's `%`-leak item 1 was mis-diagnosed** (see **instrument-correction**). The fix is
     at ingest (`normalize_percentile_dice`), not a widened live-side leak check.
  2. **A formula can be written into the prose BODY, not as a slot**, where no argument
     conversion can reach it: `inner_sea_world_guide:spell:ancestral_memory` states
     `(70+CASTERLEVEL)%%` inside its sentence. `rewrite_inline_formula` prints it as words,
     percent sign included, under a closed vocabulary — a group only qualifies when its whole
     content is formula punctuation **and** it names a leaf `leaf_words` knows, so an ordinary
     parenthetical aside is never touched.
  3. **The `FORMULA:CL-no-owner` population is 61 records, not the 487+634 prose-less rules**
     cycle 5's §6 named as its upper bound. Most prose-less converted rules carry no description
     token at all; only 61 were losing one to a refused argument, 10 of them `feat`.
- **Figures + their re-derive commands:**
  | figure | denominator | command |
  |---|---|---|
  | `apps/desktop` 7 files / 204 hits (from 9 / 219); `live_files` 203 (from 205) | the live roots' source files | `python3 scripts/pcgen_residue_gate.py --check` |
  | 204 hits by token type and by file, both summing to 204 | the same 7 files | `python3 -c "import importlib.util,os,re,collections;spec=importlib.util.spec_from_file_location('g','scripts/pcgen_residue_gate.py');g=importlib.util.module_from_spec(spec);spec.loader.exec_module(g);pats={**g.IDENTIFIER_PATTERNS,**g.TOKEN_SYNTAX_PATTERNS};res=collections.defaultdict(collections.Counter);[res[os.path.join(r,f)].update({n:len(re.findall(p,open(os.path.join(r,f),errors='replace').read()))for n,p in pats.items()})for r,d,fs in os.walk('apps/desktop')if not set(r.split(os.sep))&g.EXCLUDED_DIR_NAMES for f in fs if os.path.splitext(f)[1] in g.SOURCE_EXTENSIONS];print({k:{a:b for a,b in v.items() if b}for k,v in res.items() if sum(v.values())})"` |
  | spell catalog 2,481 rows served, 2,410 with a description | rows in `spell_resolver::spell_catalog_rows()` | `cd apps/desktop/src-tauri && cargo test --locked -j 6 converted_spell_prose_population -- --nocapture` |
  | 2,436 described before the swap; 25 gained, 48 `[redacted PI]`, 1 PI, 2 real losses | the same 2,481 rows | the cycle 5 census, `AT-35-E6-003_cycle5_converter-prose-blocker.md` §3, re-confirmed by the after figure above (2436 − 51 + 25 = 2410) |
  | converted `feat` rules with no prose at all: 487 → 477 | rules in `data/sheet_rules/*/feat/*.json` | `python3 -c "import json,glob;rs=[r for f in glob.glob('data/sheet_rules/*/feat/*.json') for r in json.load(open(f))];print(sum(1 for r in rs if not r.get('prose')),'of',len(rs))"` |
  | 167 rule files changed in the regeneration | rule files under `data/sheet_rules/<book>/` | `git show --numstat acfd1f7d99 -- data/sheet_rules \| awk -F'\t' 'NF==3 && $3 !~ /_defects\|_vars\|_report\|_tokens\|_refused/ {n++} END{print n}'` |
  | converter: 49,438 records, 49,296 converted, 142 refused, 69,346 rules, 5,281 var tables | the corpus census | `cargo run --locked --bin sheet_rule_convert -- --check` |
  | `data/sheet_rules/` source markers: 0 | every generated rule file | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | identifier audit 33 base matches, 0 this cycle's | the develop-base diff over `<scoped paths>` | in the Identifier audit row above |
- **Build scope verified:** run at `acfd1f7d99`, with `CARGO_TARGET_DIR` per tree
  (`/tmp/cargo-sd35-AT-35-E6-003` for the root workspace, `…-desktop` for the desktop crate —
  `AGENTS.md`: one directory per agent **per source tree**).
  ```
  cargo test --locked --no-run -j 6                  NO_RUN_EXIT=0 (every test binary linked)
  cargo test --locked --lib -j 6                     ok. 3299 passed; 0 failed; 15 ignored
  cargo test --locked --no-fail-fast -j 6            FULL_EXIT=0; 414 targets, 8810 passed, 0 failed, 68 ignored, 0 FAILED suites
  cd apps/desktop/src-tauri && cargo test --locked -j 6
                                                     ok. 577 passed; 0 failed; 0 ignored
  cd apps/desktop/src-tauri && cargo clippy --locked --tests -j 6
                                                     0 warnings, 0 errors
  python3 scripts/pcgen_residue_gate.py --check      verdict=PASS (live_hits 11633 -> 11618)
  cargo run --locked --bin sheet_rule_convert -- --check   exit 0; records=49438 converted=49296 refused=142 rules=69346 var_tables=5281 verdict=PASS
  cargo clippy --locked --tests -j 6 (root workspace)  0 warnings, 0 errors (one `manual_strip` this cycle raised, fixed in the same cycle)
  grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l      0
  python3 scripts/completion_atlas.py --check        missing_clearing_mechanisms=0 citation_failures=0
  python3 scripts/token_coverage.py --check          non_done=0 refused=142 refused_non_done=0 shapes=1 verdict=PASS
  python3 scripts/shape_engine_boundary.py --check   magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True
  python3 scripts/missing_engine_tables.py --check   population=0 kinds=0 citation_failures=0
  python3 scripts/denominator_gate.py --check '…/*.md' '…/artifacts/**/*.md'      files_checked=101 violations=0
  python3 scripts/denominator_gate.py --check-provenance   files_checked=218 figures_examined=542 violations=0
  ./scripts/publish-site-dashboard.sh --check-pin     input pin matches docs/work-inventory.json
  scripts/verify.sh --only pi-sweep                  RESULT: PASS
  ```
  The frontend suite was **not** run: no file under `apps/desktop/src/` changed
  (`raceCreationCoverage.test.ts` is untouched). `corpus_literal_sweep` was **not** run: no
  corpus record changed (`git status --porcelain -- data/corpus/` empty at every checkpoint).
  `v06_work_inventory` was **not** re-run: its inputs (`data/corpus/`) are unchanged and the
  inventory is already at `DONE 49438 of 49438`; `publish-site-dashboard.sh --check-pin`
  confirms the published feed still matches the committed inventory.
- **Sweep population:** N/A — no corpus record changed.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`. No figure in this
  receipt is derived from the pinned corpus; the `Befuddling Strike` and `Teleport` source rows
  quoted in the narrative come from cycle 5's artifact and from `data/corpus/`.
- **Status:** **partial**
- **What this proof does not cover** (`AGENTS.md` rule 7): the Spell Catalog swap is proved by
  **counts** of rows carrying text before and after plus the suite's own on-screen and
  cross-catalog syntax tests — not by a served-id-set diff, so a one-for-one swap inside the
  2,481 would read as no change. The 25 gains and the 51 losses come from cycle 5's census of a
  tree that was then reverted; this cycle re-derived only the **after** figure (2,410) and
  checked that it reconciles. `words_for_unlowerable`'s vocabulary is closed and tested on four
  shapes; a leaf outside it prints `a rules variable`, which is honest but tells the player
  less than the book does. Nothing here proves anything about the other seven files.
- **Notes:** `feat_catalog.rs` was **not** swapped. The converter fix recovered its ACG group
  (10 rows), but 20 real losses remain — 11 Core Essentials rows, for which `data/sheet_rules/`
  has no directory at all, and 9 Pathfinder Unchained rows the converter wrote with `prose: []`
  for a different reason. Swapping it would ship those 20 losses; the `core_essentials`
  conversion is the next cycle's first item.
- **Next-cycle scope:** **AT-35-E6-003 cycle 7.** (1) `core_essentials` converted (11 feat rows
  and every other family that book carries), then the 9 `Pu` rows diagnosed, then
  `feat_catalog.rs` (30) with the cycle 5 §4 census re-run; (2) the five structural readers,
  which need `SheetRule.applies`/`grants` rather than prose —
  `companion_catalog.rs` (52), `race_trait_picker.rs` (33), `intelligent_item_catalog.rs` (28),
  `equipment_catalog.rs` (25), `reference_library_catalog.rs` (15); (3)
  `apps/desktop/src/characterHub/raceCreationCoverage.test.ts` (21), blocked on a `race_trait`
  `target` gap cycle 2 measured. Scope flags:
  `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero units by design, decisions.md §2)`.
  Target: `root apps/desktop files=0 hits=0`.
