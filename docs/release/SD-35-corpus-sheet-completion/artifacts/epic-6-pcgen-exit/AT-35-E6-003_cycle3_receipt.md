# Cycle 3 — Epic 6 (PCGen exit) / AT-35-E6-003

Cycle 2 named three converter-side mechanisms blocking the nine `render_pcgen_desc` call sites
and sequenced all nine behind them. **That holds for the catalogs whose roster is a compiled
table, and not for the catalogs whose roster is the corpus directory itself.**
`companion_pool_catalog.rs` is the second kind: this cycle swapped it to the converted package
with **0 of its 407 served rows lost and 52 gained**, and **50 corpus records now reach a player
who could not read them before**. One `apps/desktop` file is at zero; the remainder is 11.

- **Commit SHA:** `77b10113a6` — the swap, the reach-gate follow-through, this cycle's five retro
  events, and the `figure-provenance` self-heal on cycle 2's own receipt. Cycle start
  `6e38002402`. A later commit carries this receipt and the `progress.md` / `kanban.md` rows (a
  receipt cannot name the commit that carries it).
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
- **Files touched:** **5** — 2 Rust, 3 bookkeeping.
  - **`apps/desktop/src-tauri/src/companion_pool_catalog.rs`** — the swap, its doc comment, the
    two moved tests, the new `converted_id` join and the new population ratchet.
  - **`apps/desktop/src-tauri/src/reach_gate.rs`** — 50 records deleted from
    `UNREACHED_RECORD_FINDINGS`, and 6 of the 7 companion narratives rewritten with their new
    counts and their real remaining reason (`advanced_race_guide`'s was rewritten in the same
    pass).
  - **`docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003_cycle2_receipt.md`**
    — the `figure-provenance` self-heal (see **Notes**).
  - **`docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`** — the
    `derived_at` stamp the atlas check rewrites.
  - **`docs/retro/events/at-35-e6-003.jsonl`** (+5).
  - **Zero `src/` files changed. Zero `scripts/` files changed. Zero `data/` files changed. Zero
    `tests/` files changed.** `git status --porcelain -- src/ scripts/ data/ tests/` empty at
    every checkpoint, which is why the desktop crate and the frontend correctly ran **here**
    (`apps/` was the whole code surface) and the root workspace's full-suite run did not
    (`workflow-instruction.md §6` step 3: `--no-fail-fast` is required when `src/` or the
    classifier changed — neither did).
- **Identifier audit result:** **OK_NO_BUNDLE_TAGS for this cycle; 22 matches against the develop
  base, pre-existing and not this cycle's.**
  ```bash
  CS=6e38002402807dcadc8c769e4a4d167ac811e767
  SC="src/rules_core/pilot_compute src/rules_core/feat_prereqs src/rules_core/cache_gen \
      src/pcgen_import apps/desktop/src-tauri/src apps/desktop/src src/bin \
      docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit"
  git diff --unified=0 $CS -- $SC ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '^\+.*\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
  ->  (no output)  OK_NO_BUNDLE_TAGS
  BASE=fe5ae6cd4a5f3c65d5d10f4d523f00e33b04ac47      # git merge-base HEAD origin/develop
  git diff --unified=0 "${BASE}...HEAD" -- $SC … > /tmp/e6003c3/base.diff
  grep -cE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' /tmp/e6003c3/base.diff   -> 22
  awk '/(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8})/{n++} END{print n+0}' /tmp/e6003c3/base.diff -> 22
  ```
  Two independent implementations agree (`AGENTS.md` §Concurrency: derive counts with `awk`, not
  `grep -o`). The base figure is larger than cycle 2's 1 because this cycle's `<scoped paths>` is
  the epic's full file-touch set (`src/bin`, `apps/desktop/src` included), not cycle 2's narrower
  list. **This cycle's contribution is 0.**
- **Wired-integration audit result:** **OK_NO_TOKENS for this cycle; 28 matches against the
  develop base by `grep -c`, 36 by an unanchored `awk`, all pre-existing, none a stub marker.**
  ```bash
  git diff --unified=0 $CS -- $SC ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '^\+.*\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'
  ->  (no output)  OK_NO_TOKENS
  grep -cE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' /tmp/e6003c3/base.diff -> 28
  awk '/(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)/{n++} END{print n+0}' /tmp/e6003c3/base.diff -> 36
  ```
  The two figures differ because `grep -E` applies `\b` word boundaries and the `awk` form does
  not; both are stated with the command that produced them. The matches are the domain word
  `placeholder` naming the source format's own non-citation and its list counterpart, attributed
  by AT-35-E6-001/002/003's earlier cycles. Text unchanged, count unchanged by this cycle.
- **Acceptance criterion** (verbatim, `epic-breakdown.md` `### AT-35-E6-003`):
  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers of
  > `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc` is
  > deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  | clause | at HEAD | met? |
  |---|---|---|
  | desktop `raw_tokens` readers read `SheetRule.applies`/`prose` | 5 files still read the array; unchanged this cycle | **no** |
  | `render_pcgen_desc` deleted from the live side | 13 live files / 100 hits (was 14 / 106) | **no** |
  | **zero hits under `apps/desktop/`** | **11 files / 251 hits** (was 12 / 259) | **no** |
  | desktop crate suite green | **`576 passed; 0 failed; 0 ignored`** — ran here, `apps/` was touched | **yes** |
  | frontend suite green | **`101/101 test files passed`** — ran here | **yes** |
  | the 19 on-screen tests still pass | inside the 576 above; `apps/` suite green with 0 failures | **yes** |

  **Status `partial`**, remainder named below.
- **Receipt rows (mechanical):**
  ```
  since=6e38002402807dcadc8c769e4a4d167ac811e767 target_dir=/tmp/cargo-sd35-AT-35-E6-003 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=356 ratio=n/a builds_recorded=0 pcgen_live_files=207
  ```
  `closed=0` / `relabeled=0` is correct: the unit population was already `0` non-DONE at cycle
  start. `ratio` is `n/a`, a division by zero, never `0.0`. `builds_recorded=0` is correct and is
  not a missing build: the counter reads the cycle's **root-workspace** `CARGO_TARGET_DIR`, and
  this cycle's builds were the **desktop crate's**, in its own separate workspace and its own
  target dir (`/tmp/cargo-sd35-AT-35-E6-003-desktop`, five full builds; plus a sixth in
  `/tmp/cargo-sd35-AT-35-E6-003-desktop-clippy`). `pcgen_live_files` **fell 208 → 207**, which
  `§8` makes the one non-negotiable direction.
- **PCGen residue** (`python3 scripts/pcgen_residue_gate.py --check`, at `77b10113a6`):
  ```
  root src/rules_core files=196 hits=11414
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=11 hits=251
  identifier_files=16 identifier_hits=138
  live_files=207 live_hits=11665 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  `verdict=PASS`. `root src/rules_core` is **flat at 196 / 11,414** — zero `src/` files changed,
  so the whole fall is provably `apps/desktop`'s. The instrument itself was not touched
  (`git status --porcelain -- scripts/` empty).
- **Oracle parity:** **N/A for this cycle.** No `Number` mapping was added, no converter row
  changed, and `data/sheet_rules/` is byte-identical (`sheet_rule_convert -- --check`
  `verdict=PASS`, `git status --porcelain -- data/` empty). Epic 6 touched a live path, so the row
  is owed an answer: the change is a **read-side substitution** — the same records, the same
  package, a different reader — and the per-record before/after key-set diff below is the parity
  evidence for it. `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`, unchanged.
- **Movement, four buckets:**
  - **closure (into DONE, by id-set):** 0 — the unit population was already 0 non-DONE.
  - **relabel (bucket to bucket):** 0.
  - **reachability:** **+50 records.** Fifty corpus records now appear on
    `list_companion_catalog` that did not before, deleted from `reach_gate.rs`'s
    `UNREACHED_RECORD_FINDINGS` by the gate's own instruction. This is the bucket that moved.
  - **instrument-correction:** 0. `scripts/pcgen_residue_gate.py` was not touched.
- **What was swapped, and how the absence of a loss is proved.**
  `companion_pool_catalog.rs` served a pool member's description by resolving the ingest format's
  own description string at run time, refusing the record when a term it could not compute was
  still standing. It now reads the converted record's prose through
  `sheet_rule_catalog::catalog_description` (AT-35-E6-003 cycle 2's module). The join is
  `<book>:companion:<slug of the record's own key>` — **not** the on-disk file stem, which differs
  for every `" ~ "`-grouped row (`aid.json` carries `"Animal Trick ~ Aid"`, and the converted
  record is `animal_trick_aid.json`). Getting that wrong the first time collapsed the served
  population from 407 to 4, which is exactly why the before/after diff below is the gate and an
  aggregate count would not have been.

  | path | Ultimate Wilderness, *Pilferer ~ Sneak* |
  |---|---|
  | ingest format | `…a +%1 competence bonus…\|MasterLevel/2` |
  | the old run-time resolver | **refused** — the record reached no player at all |
  | `catalog_description` over the converted rule | *"A pilferer gains a + …competence bonus on Sleight of Hand and Stealth checks."* with the term named |

  **The no-loss proof, per record.** The module's own loader was asked for its served-key set on
  either side of the swap and the two sets were diffed:
  ```bash
  # a temporary census test on each side of the swap, writing every served declared key
  cd apps/desktop/src-tauri && cargo test --locked -j 6 companion_pool_catalog -- --nocapture
  python3 -c "b=set(open('/tmp/e6003c3/before.txt').read().split('\n'));a=set(open('/tmp/e6003c3/after.txt').read().split('\n'));print(len(b),len(a),len(b-a),len(a-b))"
  ->  407 459 0 52
  ```
  **Lost 0. Gained 52.** The 52 are, by book: `ultimate_wilderness` 16, `advanced_players_guide`
  14, `ultimate_magic` 10, `core_rulebook` 7, `advanced_race_guide` 2,
  `book_of_the_damned_volume_1` 2, `bestiary` 1 — every one a row whose bonus stands on a term
  nobody can settle without a character.

  The ratchet that replaces the temporary census is permanent:
  `the_served_pool_population_never_falls_below_its_recorded_floor` asserts `declared >= 459` and
  `summary >= 25` and that no served row reaches the wire with an empty description.
  `the_refuse_gate_is_provably_live_over_the_converted_package` is the mutation-proof, asked
  **corpus-wide over the live package rather than of a fixture** (`decisions.md §4`): of the
  converted `companion` rules, **795** state descriptive prose and **3,943** state none, so both
  arms of `catalog_description` are reached.
- **The reach gate followed, and its narratives were re-derived rather than re-worded.**
  `reach_gate::unreached_records_are_exactly_the_recorded_findings` names the records that now
  reach, one family at a time, and 50 were deleted from `UNREACHED_RECORD_FINDINGS`. The six
  companion narratives that carried a count were rewritten, and each new "what remains" reason
  was **verified record by record against the converted package**, not asserted:

  | book | unreached before | after | of what remains, state no descriptive prose in the converted package |
  |---|---:|---:|---|
  | Bestiary 1 | 6 | 5 | 2 of 5 (the other 3 are structurally refused: one owned-ability row, four `.MOD` fragments) |
  | Advanced Race Guide | 9 | 7 | **7 of 7** |
  | Advanced Player's Guide | 137 | 123 | **123 of 123** |
  | Core Rulebook | 31 | 24 | **24 of 24** |
  | Ultimate Magic | 106 | 96 | **96 of 96** |
  | Book of the Damned Vol. 1 | 4 | 2 | **2 of 2** |
  | Ultimate Wilderness | 42 | 26 | **26 of 26** |

  That table is the cycle's finding, not a formality: the standing narrative said these rows were
  refused because a formula could not be resolved without a character. **They are not.** They
  carry no sentence at all in the converted package, which is a converter gap
  (`src/pcgen_import/sheet_rule/`), not a consumer gap — and `decisions.md §11` puts it on the
  converter side.
- **Refused tokens:** **none.** This cycle added no converter mapping row and cleared none; the
  refused set is unchanged at **142** records, one shape, `refused_non_done=0`
  (`token_coverage.py --check`). `§8`'s "more than 10 distinct refused token types" escalation
  does not apply. The remainder below is live-side ingest-format usage, not a converter refusal.
- **Discoveries: three, each also a `correction` retro event (`§7`).**
  1. **Cycle 2's sequencing was too wide** — `correction 1789098048562-at-35-e6-003-6f0cbe`.
     Converted-row coverage blocks the catalogs whose roster is a **compiled table** (spell,
     equipment). It does not block the catalogs whose roster is the **corpus directory itself**,
     where the join is 1:1 per record. `companion_pool_catalog.rs` was one of those and shipped
     this cycle with zero loss. `class_feature_descriptions.rs` and `class_feature_feat_bridge.rs`
     are the same kind and are cycle 4's scope.
  2. **Cycle 2's spell-miss list was 5 by shape and 4 by re-derivation** —
     `correction 1789098030763-at-35-e6-003-18120e`. `core_rulebook:spell:nondetection_self_only`
     **does** exist in the converted package and resolves; the misses are exactly the three
     `Threefold Aspect` variants plus `Wall of Thorms`. Separately, cycle 2's artifact names
     `advanced_players_guide/spell/wall_of_thorns.json` as the converted twin — **no file of that
     name exists**; the record is absent from the package entirely, because its three sibling
     variants and it are corpus records the unit inventory does not carry, and the converter's
     population is the inventory (`sheet_rule::load_population`).
  3. **The `d %` spacing is a source typo the converter copied, not a converter defect** —
     `correction 1789098039650-at-35-e6-003-3629d4`. The source record's own description carries
     `Distance off target is d %% of the distance` at that one site and `d%%` at the other two. The
     record is at `data/corpus/core_rulebook/spell/level_5/teleport.json`, not
     `spell/teleport.json` as cycle 2's re-derive block implies. The fix is still converter-side —
     a source-typo normalisation at ingest — but it is not a conversion bug, and a cycle hunting
     one in `prose.rs` would have found nothing.
- **The remainder — 11 files, 251 hits.**
  `deferral 1789098061295-at-35-e6-003-577d2c`. By file, re-derived at HEAD:
  `companion_catalog.rs`=52, `race_trait_picker.rs`=33, `feat_catalog.rs`=30,
  `intelligent_item_catalog.rs`=28, `equipment_catalog.rs`=25, `raceCreationCoverage.test.ts`=21,
  `monster_catalog.rs`=16, `reference_library_catalog.rs`=15, `class_feature_feat_bridge.rs`=11,
  `spell_catalog.rs`=10, `class_feature_descriptions.rs`=10. By pattern:
  `DESC:`=79, `render_pcgen_desc`=44, `PRE[A-Z]+:`=46, `BONUS:`=31, `raw_tokens`=28,
  `raw_bonus_chains`=10, `%CHOICE`=5, `TYPE=`=5, `%LIST`=3.
- **What cycle 4 should take, and why it is not blocked.**
  `class_feature_feat_bridge.rs` computes "this class feature's whole content is a grant of one
  named feat" by reading the ingest token array. **That relation is already in the converted
  package, in the other direction**: `advanced_players_guide:feat:improved_steal` carries 13
  `granted_by` entries, among them
  `{"by":{"Rule":"advanced_class_guide:class_feature:monk_bonus_feat_improved_steal"},"when":"Always"}`
  — verified at HEAD with `jq` over `data/sheet_rules/advanced_players_guide/feat/improved_steal.json`.
  Inverting `granted_by` gives the bridge its population without a token read and without a
  matcher. `class_feature_descriptions.rs` is corpus-rostered like the pool catalog was. Both are
  cycle 4's.
- **Figures + their re-derive commands:** every row carries its own command.
  The unit denominator where one applies is the whole corpus, all books — **49,438** (`jq '.units | length' docs/work-inventory.json`).

  | figure | value | command | denominator |
  |---|---|---|---|
  | `apps/desktop` residue, files / hits | **11 / 251** (was 12 / 259) | `python3 scripts/pcgen_residue_gate.py --check`, `root apps/desktop` line | 310 live `apps/desktop` source files the gate scans |
  | live PCGen files / hits | **207 / 11,665** (was 208 / 11,673) | `python3 scripts/pcgen_residue_gate.py --check`, last line | 49,438 units |
  | `root src/rules_core` files / hits | **196 / 11,414**, flat | `python3 scripts/pcgen_residue_gate.py --check`, `root src/rules_core` line | 196 files |
  | identifier files / hits | **16 / 138** (was 17 / 144) | `python3 scripts/pcgen_residue_gate.py --check`, `identifier_files=` line | as above |
  | served declared pool rows, before the swap | **407** | `cd apps/desktop/src-tauri && cargo test --locked -j 6 companion_pool_catalog -- --nocapture` at `6e38002402` | 1,696 converted `companion` records |
  | served declared pool rows, after the swap | **459** | `cd apps/desktop/src-tauri && cargo test --locked -j 6 companion_pool_catalog -- --nocapture` at `77b10113a6` | 1,696 converted `companion` records |
  | of the 407, lost | **0** | `python3 -c "b=set(open('/tmp/e6003c3/before.txt').read().split());a=set(open('/tmp/e6003c3/after.txt').read().split());print(len(b-a))"` | 407 rows served before |
  | gained | **52** | `python3 -c "b=set(open('/tmp/e6003c3/before.txt').read().split());a=set(open('/tmp/e6003c3/after.txt').read().split());print(len(a-b))"` | 459 rows served after |
  | served mechanical-summary rows | **25**, flat | `cd apps/desktop/src-tauri && cargo test --locked -j 6 companion_pool_catalog -- --nocapture`, `POOL_CENSUS` line | 484 served rows |
  | converted `companion` rules stating descriptive prose | **795 of 4,738 = 16.8 %** | `cd apps/desktop/src-tauri && cargo test --locked -j 6 the_refuse_gate_is_provably_live -- --nocapture`, `COMPANION_RULES` line | 4,738 `companion` rules the LOADER holds (1,696 records, siblings included). **4,751 objects are written to disk** — `python3 -c "import json,os;print(sum(len(json.load(open(os.path.join(d,f)))) for b in os.listdir('data/sheet_rules') for d in ['data/sheet_rules/%s/companion'%b] if os.path.isdir(d) for f in os.listdir(d)))"` — 13 more, the `#natural<N>` id collision `correction 1789094375625-at-35-e6-003-af8026` named, still open |
  | records deleted from `UNREACHED_RECORD_FINDINGS` | **50** | `git show --stat 77b10113a6 -- apps/desktop/src-tauri/src/reach_gate.rs` and the gate's own failure messages, family by family | 1,696 companion records |
  | spell table keys resolving a converted record | **1,711 of 1,715 = 99.8 %**, flat | `python3 - < AT-35-E6-003_cycle2_converted-row-coverage.md`'s §2 block, `report('src/rules_core/rules_tables/*/spell_list.rs','spell')` | 1,715 table keys |
  | equipment table keys resolving a converted record | **2,727 of 3,446 = 79.1 %**, flat | `python3 - < AT-35-E6-003_cycle2_converted-row-coverage.md`'s §2 block, `report('src/rules_core/rules_tables/*/equipment_data/*.rs','equipment')` | 3,446 table keys |
  | `data/sheet_rules/` token leaks | **0** | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` | 49,438 units |
  | atlas | `population=49438 buckets=10 unclassified=0 overlap=0`; `DONE 49438`; `done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0`, `EXIT=0` | `python3 scripts/completion_atlas.py --check` | 49,438 units |
  | token coverage | `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1 verdict=PASS` | `python3 scripts/token_coverage.py --check` | 49,438 units |
  | shape/engine boundary | `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True`, `EXIT=0` | `python3 scripts/shape_engine_boundary.py --check` | 49,438 units |
  | missing engine tables | `population=0 kinds=0 citation_failures=0`, `EXIT=0` | `python3 scripts/missing_engine_tables.py --check` | 49,438 units |
  | denominator gate | `files_checked=97 violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` | 97 bundle docs |
  | figure provenance | `files_checked=214 figures_examined=501 violations=0` (from **11**, all in cycle 2's receipt) | `python3 scripts/denominator_gate.py --check-provenance` | 501 figures |
  | dashboard input pin | `input pin matches docs/work-inventory.json`, `EXIT=0` | `./scripts/publish-site-dashboard.sh --check-pin` | 1 pinned input |
  | PI sweep | `RESULT: PASS`, 1 stage, 11 hits over 11 baseline rows | `scripts/verify.sh --only pi-sweep` | `src/rules_core/rules_tables` |
  | test binaries linked | **413**, flat | `grep -c '^  Executable' /tmp/e6003c3/norun.log` | 414 test targets |
  | `--no-run` errors / warnings | **0** | `grep -cE '^(error\|warning)' /tmp/e6003c3/norun.log` | that log |
  | library suite | `ok. 3296 passed; 0 failed; 15 ignored` (44.50 s), `LIB_EXIT=0` — identical to cycle 2 | `cargo test --locked --lib -j 6` | the library tests |
  | desktop crate suite | `ok. 576 passed; 0 failed; 0 ignored` (88.50 s) | `cd apps/desktop/src-tauri && cargo test --locked -j 6` | 576 desktop tests |
  | frontend suite | `101/101 test files passed` | `cd apps/desktop && npm test` | 101 test files |
  | desktop clippy | **exit 0, 0 warnings, 0 errors** | `cd apps/desktop/src-tauri && cargo clippy --locked --tests -j 6` (own target dir), `grep -cE '^(error\|warning)' /tmp/e6003c3/clippy.log` -> 0 | desktop-crate targets |
  | sheet-rule package | `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS (117.3s)` — identical to cycles 1 and 2 on every field | `cargo run --locked --bin sheet_rule_convert -- --check` | 49,438 units |
- **Build scope verified**, all at `77b10113a6`, `CARGO_INCREMENTAL=0`, `-j 6`:
  - `cargo test --locked --no-run -j 6` (root workspace, `/tmp/cargo-sd35-AT-35-E6-003`) →
    **`NO_RUN_EXIT=0`, 413 `Executable` lines, 0 errors, 0 warnings** — flat across every cycle of
    this epic, as a cycle that adds no test target must leave it.
  - `cargo test --locked --lib -j 6` → `ok. 3296 passed; 0 failed; 15 ignored` (44.50 s),
    `LIB_EXIT=0` — identical to cycle 2, as a cycle that changes no `src/` file must leave it.
  - `cargo test --locked --no-fail-fast -j 6` (root workspace) → **not run, by `§6` step 3's own
    condition**: it is required "when `src/` or the classifier changed", and this cycle changed
    neither (`git status --porcelain -- src/ scripts/ tests/` empty at every checkpoint; the
    commit lists no such path). The 413 test binaries all **linked** at this tree and the library
    suite is green and unchanged. This is the one command on `§6`'s list this cycle did not run,
    and this is the reason.
  - **Desktop crate and frontend ran HERE, not at epic cadence, because `apps/` was the whole code
    surface** (`decisions.md §3`): `cd apps/desktop/src-tauri && cargo test --locked -j 6` →
    `ok. 576 passed; 0 failed; 0 ignored` (88.50 s); `cd apps/desktop && npm test` →
    `101/101 test files passed`. The criterion's 19 on-screen tests are inside the 576 and the
    suite has zero failures.
  - `cargo clippy --locked --tests -j 6` on the desktop crate (own target dir,
    `/tmp/cargo-sd35-AT-35-E6-003-desktop-clippy`) → **exit 0, 0 warnings, 0 errors**.
  - `cargo run --locked --bin sheet_rule_convert -- --check` →
    `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS (117.3s)`.
    `git status --porcelain -- data/` empty afterwards.
  - `scripts/verify.sh --only pi-sweep` → `RESULT: PASS`, 1 stage, `RETRO_ACTOR` exported in the
    same shell invocation.
- **Sweep population:** N/A — no corpus record changed (`git status --porcelain -- data/` empty at
  every checkpoint), so `corpus_literal_sweep` was correctly not run (`§6` step 3's guard), and
  with it `v06_work_inventory`, which rebuilds its verification stamps from that sweep's report
  and would refuse to write without it (`--allow-stamp-loss` is forbidden).
  `git status --porcelain -- docs/work-inventory.json` is empty and the `--receipt` rows above
  were computed against the file on disk.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`, unchanged by this cycle).
- **Status:** **partial.** `root apps/desktop` **12 files / 259 hits → 11 / 251**, and
  `pcgen_live_files` **208 → 207**. One file is at zero and 50 corpus records reach a player who
  could not read them before. Ten files remain, plus one frontend test.
- **Notes:**
  - **`figure-provenance` self-heal.** `python3 scripts/denominator_gate.py --check-provenance`
    was **red at cycle start with 11 violations, every one in AT-35-E6-003 cycle 2's own
    receipt** — ten table rows whose command column read "same" or "the artifact's python block"
    (not a resolvable command) and one figure whose command sat on the **next** line. That is
    incident key `figure-provenance-command-on-next-line`, the key
    `workflow-instruction.md §6` added this command to every cycle's gate to catch. It blocked
    this cycle's push, so it was fixed here: each row now names a runnable command. The values
    were not touched.
  - **What this cycle's proof does not cover** (`AGENTS.md` rule 7). The before/after key-set diff
    proves no record **stopped** being served. It does not prove the served **text** is identical
    for the 407 that survived — the whole point is that some of it changed, and only *Pilferer ~
    Sneak*, *Animal Trick ~ Aid* and *Companion Bonus Skill* were read end to end by eye (the
    latter two are pinned by tests asserting their exact sentence). It proves nothing about the
    other ten files. And `the_served_pool_population_never_falls_below_its_recorded_floor` is a
    floor, not an identity: a cycle that lost a row and gained a different one would pass it.
  - **`git status --porcelain` is non-empty for every cycle on `tranche/15`** because the shared
    checkout carries an untracked, un-gitignored `.worktrees/` directory holding another session's
    live git worktree. `incident 1789098061429-at-35-e6-003-6170e4`, key
    `untracked-worktrees-dir-on-shared-checkout`, **6th recurrence**; unchanged, still needs a
    one-line `.gitignore` ruling. `AGENTS.md` rule 8: a warning carried forward six times is a
    missing mechanism.
- **Next-cycle scope:** **AT-35-E6-003 cycle 4** — the two remaining corpus-rostered files,
  `class_feature_descriptions.rs` (10 hits) and `class_feature_feat_bridge.rs` (11 hits), the
  latter by inverting the converted package's own `granted_by` rather than reading a token array.
  Then `companion_catalog.rs` (52), the largest single file left. Scope flags:
  `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero units by design, decisions.md §2)`. Target:
  `root apps/desktop files=0 hits=0`.
