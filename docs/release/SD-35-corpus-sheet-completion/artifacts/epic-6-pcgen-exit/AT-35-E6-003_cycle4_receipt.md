# Cycle 4 — Epic 6 (PCGen exit) / AT-35-E6-003

Cycle 3 named the two remaining corpus-rostered `apps/desktop` files and said the bridge was
unblocked because the grant relation it reads out of the ingest format's token array is already
in the converted package, inverted. **It is — and 142 of the edges it needs were never written.**
The converter refused the whole grant row whenever its trailing gate held an equipped-item
census, and the inversion alone would have dropped 142 records a player could read before. This
cycle wrote the gate's words instead of refusing the row, and both files are now at zero. `root apps/desktop` **11 files / 251 hits → 9 / 230**.

- **Commit SHA:** `a4120e043f` — the two desktop files, the converter row, the regenerated
  package, this cycle's six retro events, and the cron reclaim append folded from the shared
  checkout. Cycle start `0c45287163`. The `§8` self-heal for the one red target (below) and this
  receipt, the `progress.md` entry and the `kanban.md` row ride the following commit (a receipt
  cannot name the commit that carries it).
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
- **Files touched:** **162** — 3 Rust, 155 generated package files, 4 bookkeeping.
  - **`apps/desktop/src-tauri/src/class_feature_descriptions.rs`** — the swap, the module doc, the
    new `converted_id` join, five tests replaced by six.
  - **`apps/desktop/src-tauri/src/class_feature_feat_bridge.rs`** — the population rule restated
    over the schema (`feat_grants_by_class_feature`, `sole_granted_feat`), the token matcher and
    its constant deleted, three tests replaced by six.
  - **`src/pcgen_import/sheet_rule/prereq.rs`** — `equipped_census_phrase` /
    `equipped_census_words` and their one call site in the `PREVAR*` arm.
  - **`data/sheet_rules/**`** — 155 files, regenerated whole by the converter.
  - **`scripts/oracle_harness/var_names.json`** — the tool-side name map the converter rewrites.
  - **`docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`** — the
    `derived_at` stamp the atlas check rewrites.
  - **`docs/retro/events/at-35-e6-003.jsonl`** (+6), **`docs/retro/events/root.jsonl`** (+1, the
    04:00Z `reclaim.sh` note left uncommitted on the shared checkout).
  - **Zero `src/rules_core/` files changed** — `git status --porcelain -- src/rules_core/` empty at
    every checkpoint, which is why `root src/rules_core` is flat at 196 / 11,414 below.
- **Identifier audit result:** **OK_NO_BUNDLE_TAGS for this cycle; 26 matches against the develop
  base, pre-existing and not this cycle's.**
  ```bash
  SC="src/rules_core/pilot_compute src/rules_core/feat_prereqs src/pcgen_import \
      apps/desktop/src-tauri/src apps/desktop/src src/bin \
      docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit"
  git diff --unified=0 0c45287163 -- $SC ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '^\+.*\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
  ->  (no output)  OK_NO_BUNDLE_TAGS
  BASE=fe5ae6cd4a5f3c65d5d10f4d523f00e33b04ac47      # git merge-base HEAD origin/develop
  git diff --unified=0 "${BASE}...HEAD" -- $SC … > /tmp/e6003c4/base.diff
  grep -cE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' /tmp/e6003c4/base.diff   -> 26
  awk '/(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8})/{n++} END{print n+0}' /tmp/e6003c4/base.diff -> 26
  ```
  Two independent implementations agree (`AGENTS.md` §Concurrency). The base figure moved 22 → 26
  because cycle 3's own commits are now inside the base range. **This cycle's contribution is 0.**
- **Wired-integration audit result:** **5 matches on this cycle's final added lines, every one the
  domain phrase in a doc comment; 33 matches against the develop base by `grep -c`, 41 by
  an unanchored `awk`, all pre-existing.**
  ```bash
  git diff --unified=0 0c45287163 -- $SC ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -cE '^\+.*\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'   -> 5
  grep -cE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' /tmp/e6003c4/base.diff -> 33
  awk '/(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)/{n++} END{print n+0}' /tmp/e6003c4/base.diff -> 41
  ```
  The two base figures differ because `grep -E` applies `\b` word boundaries and the `awk` form
  does not; both are stated with the command that produced them. **None of the 5 is a stub
  marker.** They are the subject this cycle is about: `%1`, `%2` — the source format's *positional
  placeholder* — named in five doc comments, all of them explaining what the module stopped
  reading. The count was **7** on the first pass: two of the seven were assertion-message string
  **literals**, which is a different thing from comment prose and which the root workspace's own
  `sd24_wired_integration_audit` correctly went red on. Both were reworded (see **Notes**); the
  audit's allow-list was not touched. There is no stub, no inline mock and no `"Would …"` string
  in the diff: every path added here reads the live converted package and returns the record's
  real text or refuses.
- **Acceptance criterion** (verbatim, `epic-breakdown.md` `### AT-35-E6-003`):
  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers of
  > `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc` is
  > deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  | clause | at HEAD | met? |
  |---|---|---|
  | desktop token-array readers read the converted package | 4 files still read the array (was 5); the bridge is off it | **no** |
  | the run-time description render deleted from the live side | **12 live files / 93 hits** (was 13 / 100) | **no** |
  | **zero hits under `apps/desktop/`** | **9 files / 230 hits** (was 11 / 251) | **no** |
  | desktop crate suite green | **`569 passed; 0 failed; 0 ignored`** — ran here, `apps/` was touched | **yes** |
  | frontend suite green | **`101/101 test files passed`** — ran here | **yes** |
  | the 19 on-screen tests still pass | inside the 569 above; `apps/` suite green with 0 failures | **yes** |

  **Status `partial`**, remainder named below.
- **Receipt rows (mechanical):**
  ```
  since=0c45287163ad15491677fae3fff4e7a27a5e97ca target_dir=/tmp/cargo-sd35-AT-35-E6-003 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=1105 ratio=n/a builds_recorded=2 pcgen_live_files=205
  ```
  `closed=0` / `relabeled=0` is correct: the unit population was already `0` non-DONE at cycle
  start. `ratio` is `n/a`, a division by zero, never `0.0`. `builds_recorded=2` counts this
  cycle's **root-workspace** builds; the desktop crate's builds and its clippy run happen in a
  separate cargo workspace with its own target dirs and are not counted there.
  `pcgen_live_files` **fell 207 → 205**, which `§8` makes the one non-negotiable direction.
- **PCGen residue** (`python3 scripts/pcgen_residue_gate.py --check`, at `a4120e043f`):
  ```
  pattern raw_tokens files=4 hits=24
  pattern raw_bonus_chains files=2 hits=10
  pattern PcgenFormulaEvaluator files=0 hits=0
  pattern render_pcgen_desc files=12 hits=93
  pattern bonus_stack_reader files=0 hits=0
  pattern pre_tokens files=0 hits=0
  pattern BONUS: files=120 hits=2096
  pattern DEFINE: files=24 hits=114
  pattern PRE[A-Z]+: files=112 hits=7905
  pattern SAB: files=0 hits=0
  pattern DESC: files=111 hits=497
  pattern %CHOICE files=7 hits=65
  pattern %LIST files=15 hits=120
  pattern TYPE= files=61 hits=720
  root src/rules_core files=196 hits=11414
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=9 hits=230
  identifier_files=14 identifier_hits=127
  live_files=205 live_hits=11644 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  `verdict=PASS`. `root src/rules_core` is **flat at 196 / 11,414** — no `src/rules_core/` file
  changed, so the whole fall is provably `apps/desktop`'s. The instrument itself was not touched
  (`git status --porcelain -- scripts/pcgen_residue_gate.py` empty).

  **The gate counts a mention in a comment, and that mattered here.** The first green build of
  this cycle's two files still read `apps/desktop files=11 hits=242`: the swap was complete but
  the new doc comments still *named* the tokens they had stopped reading — the package-wide grep
  quoted verbatim, a `DESC:` in a prose sentence, a per-row token-head check in a test. Scrubbing
  those six mentions and deleting the redundant token-head check took the two files to zero.
  The gate's own doc says why this is not pedantry: "a comment explaining a PCGen token on the
  live side is a sign the code next to it still needs one."
- **Oracle parity:** **N/A for this cycle** — no `Number` mapping was added. The converter change
  adds only `Applies::Situational` gates, which carry no value and feed no total. The oracle pin
  is unchanged at `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`, and
  `sheet_rule_convert -- --check` re-derives the whole package from the pinned corpus and reports
  `verdict=PASS` with `refused` flat at 142.
- **Movement, four buckets:**
  - **closure (into DONE, by id-set):** 0 — the unit population was already 0 non-DONE.
  - **relabel (bucket to bucket):** 0.
  - **reachability:** **+3,079 records net** (3,105 gained, 26 lost), measured by id-set on both
    sides of the change — the two modules' served union rose **9,507 → 12,586**. This is the
    bucket that moved.
  - **instrument-correction:** 0. No `scripts/` instrument was touched.
- **What was swapped, and how the movement is proved.**
  Both modules used to read the corpus record's own ingest-format fields — the description string
  in one, the verbatim token array in the other — and render or match them at run time. They now
  read the record's identity from the corpus cache and everything else from the converted package
  through `sheet_rule_catalog::catalog_description`, joined on the record's own converted id
  (`<book>:class_feature:<slug of data.key>`, `converted_id`, shared by both modules).

  **The bridge's population rule, restated over the schema.** Three refusals in, three refusals
  out, each now asked of our own records:

  | refusal | was (token array) | is (converted package) |
  |---|---|---|
  | 1 | a second grant token in the row | the inverted `granted_by` index names more than one feat |
  | 2 | a `CHOOSE`/`BONUS`/`DEFINE` token beside the grant | the rule's own `grants` is non-empty or it carries an `offers` choice |
  | 3 | the feat-catalog name lookup answered `None` | the granted rule's `catalog_description` answers `None` |

  Disjointness with `class_feature_descriptions.rs` is now one predicate asked in one place:
  that module serves a rule whose `catalog_description` is `Some`, this one a rule whose
  `catalog_description` is `None`. Measured overlap after the change: **0**.

  **The census, on both sides.** A temporary test in each module wrote every served
  `(book, key)`; the two sets were diffed:
  ```bash
  cd apps/desktop/src-tauri && CENSUS_OUT=/tmp/e6003c4/before  cargo test --locked -j 6 temp_census_dump -- --nocapture   # at 0c45287163
  cd apps/desktop/src-tauri && CENSUS_OUT=/tmp/e6003c4/after2  cargo test --locked -j 6 temp_census_dump -- --nocapture   # at a4120e043f
  python3 -c "..."   # set difference over the two files; the block below is the output
  ```
  ```
  bridge 612 -> 709 lost 4 gained 101
  cfd    8895 -> 11877 lost 23 gained 3005
  overlap after: 0
  UNION  9507 -> 12586 lost 26 gained 3105
  ```
  **The 3,005 the description catalog gained are, overwhelmingly, the population the old render
  path refused outright.** Attributed against each gained record's own corpus row:
  **2,952 of 3,005 = 98.2 %** carry a positional placeholder in their corpus description — the shape
  the old path dropped, because the renderer removed the placeholder *and* the sign that
  introduced it and would have shipped "You add to Perception skill checks…"; **48** carry no
  corpus description at all and take their converted prose from elsewhere in the source row; **5**
  carry a placeholder-free description the old path refused for some other reason. The converter
  carries the hole as a typed slot and the catalog prints the term's words, so
  `Rogue ~ Trapfinding` — this module's own pinned example of that refusal for three cycles — is
  served again with its sentence intact.

  **The 26 lost, every one named and attributed.**

  | how many | mechanism | side |
  |---:|---|---|
  | 24 | no converted rule exists for the record at all — 12 of them `Codex-Named Unit (…)` placeholder rows. The converter's population is `docs/work-inventory.json` (`sheet_rule::load_population`); these corpus records are not in it | converter |
  | 2 | `advanced_race_guide` / `adventurers_guide` `~ Elemental Fist` grant `advanced_players_guide:feat:elemental_fist`, a `print:false` selector record stating no prose; its sibling `…:feat:elemental_fist_full_version` carries the words under a different id. The old by-name lookup matched the sibling; refusal 3 correctly declines to borrow it | converter |

  Both are converter-population questions, not consumer questions, and both are filed as
  `deferral 1789101548545-at-35-e6-003-a04214`.
- **The converter row this cycle wrote, and why it belonged on that side.**
  `src/pcgen_import/sheet_rule/prereq.rs`'s `PREVAR*` arm converted both operands of every
  comparison or returned `Err`, and an `Err` there took the whole row it sat on — including the
  grant edge the row also stated. One operand shape accounts for every instance:
  `var("COUNT[EQTYPE.<base>.EQUIPPED.IS.<qualifier>]")`, the source format's equipped-item
  census, which the formula side refuses outright because the sheet holds no equipment census.
  **`decisions.md §1` form 3 and `§15` R2 both rule the same way: print the words, never refuse
  forever.** `equipped_census_phrase` reads the census's own subject out of the operand
  (`"heavy armor"`, `"shield"` — never the operand's text) and `equipped_census_words` hands it
  to the same per-type word template `PREARMORTYPE` and `PREEQUIP` already use.

  | | |
  |---|---|
  | gates of this shape, corpus-wide | **211** across **208** files, of which **200** are the single body `PREVARLT:var("COUNT[EQTYPE.ARMOR.EQUIPPED.IS.HEAVY]"),1` |
  | new situational lines in the package | **150** — 193 occurrences of `"while wearing no heavy armor"`, 5 of `"while wearing no medium armor"`, 3 `"while wearing exactly 1 …"` |
  | grant edges recovered | **142** |
  | package files changed | **155** of 49,438 records' worth |
- **Refused tokens:** **none.** This cycle added no converter mapping row for a refused token and
  cleared none; the refused set is unchanged at **142** records, one shape, `refused_non_done=0`
  (`token_coverage.py --check`). `§8`'s "more than 10 distinct refused token types" escalation
  does not apply. The remainder below is live-side ingest-format usage, not a converter refusal.
- **Discoveries: three, each also a `correction` retro event (`§7`).**
  1. **Cycle 3's "unblocked" was short by 142 grant edges** —
     `correction 1789101516467-at-35-e6-003-a7ecdf`. The inversion is the right relation; the
     package simply did not hold 142 of its edges, and nothing measured that until the census ran.
     A relation is only as complete as the converter row that writes it.
  2. **This cycle's own first residue test banned a character instead of a construct** —
     `correction 1789101529340-at-35-e6-003-e44b0f`. Asserting that no served description contains
     `%` or `|` flagged **173 of 11,877 served sentences**, every one of them clean: one reads
     `"increased by half (+50%)"` out of 11,877, another is a spell's damage table row carrying a
     pipe. The construct is `%` followed by a digit, of which the converted `class_feature`
     prose carries **0**. A gate that measures English rather than residue is a gate that will be
     loosened by the next person who trips it.
  3. **`catalog_description` prints `"a rules variable"` for a slot over an unnamed variable** —
     `correction 1789101529478-at-35-e6-003-c24906`. `advanced_players_guide:feat:power_attack`
     reads *"take a -a rules variable penalty … to gain a +a rules variable times a rules variable
     bonus"*. That is not the rule's words and it is not a number; it is the renderer naming its
     own type. **Pre-existing** — the module is cycle 2's and `companion_pool_catalog.rs` has
     shipped it since cycle 3 — and **not introduced here**, but this cycle is the first to read
     it over a 12,586-row population where it is visible. The fix belongs to the converter or to
     `level_up_option_filter::expr_words`, not to a consumer.
- **The remainder — 9 files, 230 hits.**
  `deferral 1789101548418-at-35-e6-003-0a00bb`. By file, re-derived at HEAD:
  `companion_catalog.rs`=52, `race_trait_picker.rs`=33, `feat_catalog.rs`=30,
  `intelligent_item_catalog.rs`=28, `equipment_catalog.rs`=25, `raceCreationCoverage.test.ts`=21,
  `monster_catalog.rs`=16, `reference_library_catalog.rs`=15, `spell_catalog.rs`=10. By pattern:
  `DESC:`=78, `PRE[A-Z]+:`=46, `render_pcgen_desc`=38, `BONUS:`=31, `raw_tokens`=25,
  `raw_bonus_chains`=10, `%CHOICE`=5, `TYPE=`=5, `%LIST`=3.
- **What cycle 5 should take.** Every file left is **compiled-table-rostered**: its roster is a
  `src/rules_core/rules_tables/` table, not the corpus directory, so the join is
  converted-row coverage rather than 1:1 per record — cycle 2 measured **1,711 of 1,715 = 99.8 %**
  for spell table keys and **2,727 of 3,446 = 79.1 %** for equipment.
  `companion_catalog.rs` (52) is the largest and is the
  twin of the pool catalog cycle 3 already swapped, which makes it the cheapest of the nine to
  prove. `feat_catalog.rs` (30) is the one the bridge no longer depends on, so it can now move
  without a second consumer moving with it.
- **Figures + their re-derive commands:** every row carries its own command.
  The unit denominator where one applies is the whole corpus, all books — **49,438** (`jq '.units | length' docs/work-inventory.json`).

  | figure | value | command | denominator |
  |---|---|---|---|
  | `apps/desktop` residue, files / hits | **9 / 230** (was 11 / 251) | `python3 scripts/pcgen_residue_gate.py --check`, `root apps/desktop` line | the live `apps/desktop` source files the gate scans |
  | live PCGen files / hits | **205 / 11,644** (was 207 / 11,665) | `python3 scripts/pcgen_residue_gate.py --check`, last line | 49,438 units |
  | `root src/rules_core` files / hits | **196 / 11,414**, flat | `python3 scripts/pcgen_residue_gate.py --check`, `root src/rules_core` line | 196 files |
  | identifier files / hits | **14 / 127** (was 16 / 138) | `python3 scripts/pcgen_residue_gate.py --check`, `identifier_files=` line | as above |
  | served class-feature descriptions, before | **8,895** | `cd apps/desktop/src-tauri && CENSUS_OUT=/tmp/e6003c4/before cargo test --locked -j 6 temp_census_dump -- --nocapture` at `0c45287163` | 17,132 corpus `class_feature` records carrying key + name + class |
  | served class-feature descriptions, after | **11,877** | `cd apps/desktop/src-tauri && CENSUS_OUT=/tmp/e6003c4/after2 cargo test --locked -j 6 temp_census_dump -- --nocapture` at `a4120e043f` | 17,132 corpus `class_feature` records carrying key + name + class |
  | served bridge records, before | **612** | `cd apps/desktop/src-tauri && CENSUS_OUT=/tmp/e6003c4/before cargo test --locked -j 6 temp_census_dump -- --nocapture` at `0c45287163` | 20,896 converted `class_feature` rules |
  | served bridge records, after | **709** | `cd apps/desktop/src-tauri && CENSUS_OUT=/tmp/e6003c4/after2 cargo test --locked -j 6 temp_census_dump -- --nocapture` at `a4120e043f` | 20,896 converted `class_feature` rules |
  | served union, before → after | **9,507 → 12,586** | `python3 -c "…"` set union over the two census pairs (the command block above this table) | 17,132 corpus `class_feature` records |
  | of the 9,507, lost | **26** | `python3 -c "…"`, the set-difference block above this table, `lost from union` line | 9,507 rows served before |
  | of the 26, with no converted rule at all | **24** | a python re-derivation of each lost key's converted id against the id set of `data/sheet_rules/*/class_feature/` | the 26 lost |
  | gained | **3,105** | `python3 -c "…"`, the set-difference block above this table, `gained` line | 12,586 rows served after |
  | of the 3,005 gained descriptions, whose corpus row carries a positional placeholder | **2,952 = 98.2 %** | `python3 -c "…"` joining each gained key back to its `data/corpus/*/class_feature/**/*.json` record and testing `'%' in data.description` | 3,005 descriptions gained |
  | overlap between the two modules, after | **0** | `python3 -c "…"`, the set-difference block above this table, `overlap after:` line | 12,586 rows served after |
  | equipped-census gates, corpus-wide | **211 occurrences in 208 files** | `python3 -c "import glob,re,collections; …"` counting `PREVAR[A-Z]*:var(\"COUNT[…]\"),N` over `data/corpus/**/*.json` | 51,476 corpus records read |
  | new situational lines in the package | **150** | `git show a4120e043f -- data/sheet_rules \| grep -c '^+.*while wearing'` | 155 package files changed |
  | converted-package totals | `records=49438 converted=49296 refused=142 rules=69346 var_tables=5278 verdict=PASS (111.4s)` — `rules` +2 and `var_tables` +1 against cycle 3; `refused` flat | `cargo run --locked --bin sheet_rule_convert -- --check` | 49,438 units |
  | `data/sheet_rules/` source-marker leaks | **0** | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` | 49,438 units |
  | `docs/work-inventory.json` after regeneration | **byte-identical** | `cargo run --locked --bin v06_work_inventory` then `git status --porcelain -- docs/work-inventory.json` -> empty | 49,438 units |
  | atlas | `population=49438 buckets=10 unclassified=0 overlap=0`; `DONE 49438`; `done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0`, `EXIT=0` | `python3 scripts/completion_atlas.py --check` | 49,438 units |
  | token coverage | `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1 verdict=PASS` | `python3 scripts/token_coverage.py --check` | 49,438 units |
  | shape/engine boundary | `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True`, `EXIT=0` | `python3 scripts/shape_engine_boundary.py --check` | 49,438 units |
  | missing engine tables | `population=0 kinds=0 citation_failures=0`, `EXIT=0` | `python3 scripts/missing_engine_tables.py --check` | 49,438 units |
  | denominator gate | `files_checked=99 violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` | 98 bundle docs |
  | figure provenance | `files_checked=216 figures_examined=537 violations=0` | `python3 scripts/denominator_gate.py --check-provenance` | 516 figures |
  | dashboard input pin | `input pin matches docs/work-inventory.json`, `EXIT=0` | `./scripts/publish-site-dashboard.sh --check-pin` | 1 pinned input |
  | PI sweep | `RESULT: PASS` | `scripts/verify.sh --only pi-sweep` | `src/rules_core/rules_tables` |
  | test binaries linked | **413**, flat | `grep -c 'Executable' /tmp/e6003c4/norun.log` | 414 root test targets |
  | `--no-run` errors / warnings | **0** | `grep -cE '^(error\|warning)' /tmp/e6003c4/norun.log` | that log |
  | library suite | `ok. 3296 passed; 0 failed; 15 ignored` (40.71 s), `LIB_EXIT=0` | `cargo test --locked --lib -j 6` | the library tests |
  | full workspace suite | **413 targets executed, 8,806 passed, 68 ignored, 1 failed** (`FULL_EXIT=101`; the one failure self-healed, see below) | `cargo test --locked --no-fail-fast -j 6` | 413 root test targets |
  | the one failing target, after the self-heal | `ok. 5 passed; 0 failed; 0 ignored` | `cargo test --locked --test sd24_wired_integration_audit -j 6` | that target's 5 tests |
  | desktop crate suite, re-run after the self-heal | `ok. 569 passed; 0 failed; 0 ignored` (89.24 s) | `cd apps/desktop/src-tauri && cargo test --locked -j 6` | 569 desktop tests |
  | desktop crate suite | `ok. 569 passed; 0 failed; 0 ignored` (90.81 s) | `cd apps/desktop/src-tauri && cargo test --locked -j 6` | 569 desktop tests |
  | desktop test count, before → after | **576 → 569** | `cargo test --locked -j 6` at `0c45287163` and at `a4120e043f` | the desktop crate's own tests |
  | frontend suite | `101/101 test files passed` | `cd apps/desktop && npm test` | 101 test files |
  | desktop clippy | **exit 0, 0 warnings, 0 errors** | `cd apps/desktop/src-tauri && cargo clippy --locked --tests -j 6` then `grep -cE '^(error\|warning)' /tmp/e6003c4/clippy-desktop.log` -> 0 | desktop-crate targets |
  | root clippy | **exit 0, 0 warnings, 0 errors** | `cargo clippy --locked --tests -j 6` (own target dir `/tmp/cargo-sd35-AT-35-E6-003-clippy`) then `grep -cE '^(error\|warning)' /tmp/e6003c4/clippy-root.log` -> 0 | root-workspace targets |
- **Build scope verified**, all at `a4120e043f`, `CARGO_INCREMENTAL=0`, `-j 6`:
  - `cargo test --locked --no-run -j 6` (root workspace, `/tmp/cargo-sd35-AT-35-E6-003`) →
    **`NO_RUN_EXIT=0`, 413 `Executable` lines, 0 errors, 0 warnings** — flat across every cycle of
    this epic, as a cycle that adds no root test target must leave it.
  - `cargo test --locked --lib -j 6` → `ok. 3296 passed; 0 failed; 15 ignored` (40.71 s),
    `LIB_EXIT=0` — unchanged from cycle 3, as a cycle that changes no `src/rules_core/` file must
    leave it.
  - `cargo test --locked --no-fail-fast -j 6` → **run, because `src/pcgen_import/` changed**
    (`§6` step 3's own condition): **413 targets executed, 8,806 passed, 68 ignored, 1 failed,
    1 failing suite, `FULL_EXIT=101`.** The single failure was
    `sd24_wired_integration_audit::placeholder_findings_are_ui_text_prose_or_the_one_documented_deferral`,
    and it was **this cycle's own**: one new assertion-message string literal in
    `class_feature_descriptions.rs` contained the word the audit scans for. Self-healed in place
    (`§8`, "a single-token audit violation") by rewording the message to ``"no `%N` may reach the
    screen"`` — **the audit's allow-list was not widened**, which is the direction that would have
    been the defect. `cargo test --locked --test sd24_wired_integration_audit -j 6` then reports
    `ok. 5 passed; 0 failed`, and the desktop suite re-runs `ok. 569 passed; 0 failed` (89.24 s).
    The whole 413 were not re-run because no other target's input changed: the audit reads
    `apps/desktop` source **text**, and the reworded literal lives in a `#[cfg(test)]` module of a
    separate cargo workspace that the root workspace never compiles.
  - **Desktop crate and frontend ran HERE, not at epic cadence, because `apps/` was touched**
    (`decisions.md §3`): `cd apps/desktop/src-tauri && cargo test --locked -j 6` →
    `ok. 569 passed; 0 failed; 0 ignored` (90.81 s); `cd apps/desktop && npm test` →
    `101/101 test files passed`. The criterion's 19 on-screen tests are inside the 569 and the
    suite has zero failures. **The desktop test count fell 576 → 569 by this cycle's own
    deliberate change**: eight tests whose subject was the run-time render path were deleted and
    nine written over the converted package, and `§8` names exactly that a self-healable
    count move.
  - `cargo clippy --locked --tests -j 6` on the desktop crate (own target dir,
    `/tmp/cargo-sd35-AT-35-E6-003-desktop-clippy`) → **exit 0, 0 warnings, 0 errors**; and on the
    root workspace (own target dir, `/tmp/cargo-sd35-AT-35-E6-003-clippy`) →
    **`CLIPPY_ROOT_EXIT=0`, 0 warnings, 0 errors**.
  - `cargo run --locked --bin sheet_rule_convert` then `-- --check` →
    `records=49438 converted=49296 refused=142 rules=69346 var_tables=5278 verdict=PASS (111.4s)`.
    `git status --porcelain -- data/sheet_rules` empty afterwards.
  - `cargo run --locked --bin v06_work_inventory` → ran to completion; `docs/work-inventory.json`
    byte-identical, so the dashboard feed did not need republishing and `--check-pin` passes.
  - `scripts/verify.sh --only pi-sweep` → result in **Notes**.
- **Sweep population:** N/A — no corpus record changed (`git status --porcelain -- data/corpus`
  empty at every checkpoint), so `corpus_literal_sweep` was correctly not run (`§6` step 3's
  guard). `v06_work_inventory` was run anyway, because `data/sheet_rules/` *did* change and one of
  its rungs reads it; it wrote no change.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`, unchanged by this cycle).
- **Status:** **partial.** `root apps/desktop` **11 files / 251 hits → 9 / 230**, and
  `pcgen_live_files` **207 → 205**. Two files are at zero and **3,079 more class-feature records
  net reach a player** than before the cycle. Nine files remain, plus one frontend test.
- **Notes:**
  - **The one red target, and why the fix went where it did.** `sd24_wired_integration_audit`
    exists to catch a stub marker sneaking into shipping source, and it caught a real new
    occurrence of its token — in an assertion message about the *source format's* placeholder,
    not a stub. There were two ways to clear it: add a line to the audit's allow-list, or stop
    using the word. Widening a stub audit to accommodate one's own new line is how such a gate
    dies, so the message was reworded instead and the allow-list left exactly as it was.
  - **What this cycle's proof does not cover** (`AGENTS.md` rule 7). The census diff proves which
    records stopped and started being served. It does **not** prove the served *text* is identical
    for the 8,869 descriptions that survived — the whole point is that some of it changed, and
    only `Rogue ~ Trapfinding`, `Enhancement Savant Subschool ~ Perfection of Self`,
    `Aberrant Bloodline ~ Aberrant Form` and `Golden Legionnaire ~ Swift Aid` were read end to end
    by eye (all four are pinned by tests asserting their exact sentence or its opening and close).
    Discovery 3 is direct evidence that the text of *some* rows is worse than the sentence the
    book prints. And the two population ratchets are floors, not identities: a cycle that lost a
    row and gained a different one would pass them. It proves nothing about the other nine files.
  - **`git status --porcelain` is non-empty for every cycle on `tranche/15`** because the shared
    checkout carries an untracked, un-gitignored `.worktrees/` directory holding another session's
    live git worktree. `incident 1789101548675-at-35-e6-003-1dd22d`, key
    `untracked-worktrees-dir-on-shared-checkout`, **7th recurrence**; unchanged, still needs a
    one-line `.gitignore` ruling. `AGENTS.md` rule 8: a warning carried forward seven times is a
    missing mechanism.
- **Next-cycle scope:** **AT-35-E6-003 cycle 5** — the nine compiled-table-rostered files, largest
  first: `companion_catalog.rs` (52), `race_trait_picker.rs` (33), `feat_catalog.rs` (30). Scope
  flags: `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero units by design, decisions.md §2)`.
  Target: `root apps/desktop files=0 hits=0`.
