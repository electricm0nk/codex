# Cycle AT-35-E6-003-RULED cycle 16 — Epic 6 (PCGen exit) / AT-35-E6-003-RULED

- **Commit SHA:** `16f7fac9b0` (the code, the regenerated converted artifact, the cycle-16 census
  script and its JSON, two retro events, and the folded shared-checkout artifacts), cycle start
  `40f5d4f67a`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design;
  decisions.md §2, workflow-instruction.md §6 step 1)`

  It ran anyway, at the cycle's start tree `40f5d4f67a`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the same tree and passed at exactly
  cycle 15's closing figure — nothing drifted between the two cycles:
  ```
  live_files=6 live_hits=10 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

- **Files touched:**

  **The live side — it renders from settled data now, and names no converter:**
  - `src/rules_core/desc_template.rs` — **new, +~350, live side.** `DescTemplate` (an ordered
    `DescOp` list, the argument texts its slots are keyed by, and the literal escape pairs to
    decode at the end), `DescArgument` (`Literal`, or `Named` with the one
    `<name><sign><integer>` offset shape the corpus contains as its fallback), and
    `DescTemplate::render(&BTreeMap<String, i64>) -> RenderedDescription`. **Pure serde plus a
    walk over an op list.** Everything value-dependent that the request-time renderer did stays
    here and is reproduced term for term: the drop-and-report rule, the `+`/`-` swallowed by a
    dropped slot, the "a per-cent sign needs a subject" check against the text already built,
    the whitespace collapse that runs only when something was dropped, and the escape decode
    that runs after it. The census asserts by text that this file names `pcgen_import` nowhere
    in shipping code — a live renderer that named the converter would just be the old boundary
    under a new name.
  - `src/rules_core/pilot_compute/class_feature_grant_consumer.rs` — **five hits → ZERO; the
    file is cleared.** `resolved_description_for` and
    `resolved_description_for_formula_only_desc_argument` look the record's settled template up
    in `record_vars::package().desc_templates` and render it against the chain they already
    resolved. `PcgenDisplayValues` is replaced by the `BTreeMap<String, i64>` those functions
    were already building; `desc_token_arguments(&record.raw_description)` is replaced by
    `template.args()`. **Neither function's refusal contract, seeds, guards or return shape
    changed** — the same `dropped_args.is_empty()` test, the same `leaked_markup` check, the
    same `None`.
  - `src/rules_core/record_vars.rs` — `RecordVarPackage` gains
    `desc_templates: BTreeMap<String, DescTemplate>`, `#[serde(default)]` like its
    `desc_arguments` sibling. No existing field was added to, removed from or renamed.

  **The converter side — it does the same reading, once, at authoring time:**
  - `src/pcgen_import/desc_template_convert.rs` — **new, +~350, converter side.** The settling
    scan. It is `pcgen_desc::render_pcgen_desc_with_values`'s **own scan, branch for branch**,
    with the value-dependent half lifted out, and it reuses that module's own readings rather
    than restating them: `split_prose_and_args` (the argument tail taken from the right),
    `is_percentile_dice_notation`, and `PCGEN_ENTITIES`. Every branch decidable from the source
    text alone — an escape, an escaped digit the row did supply, a keyword substitution, a bare
    marker, dice notation, a per-cent already attached to a literal number, which argument a
    slot names, whether the row supplied that argument at all — is evaluated here and frozen
    into an op.
  - `src/pcgen_import/pcgen_desc.rs` — three items widened from private to `pub(crate)`
    (`split_prose_and_args`, `is_percentile_dice_notation`, `PCGEN_ENTITIES`) so the settling
    reuses them. **No function body changed**, and the module stays converter-side.
  - `src/pcgen_import/class_feature_vars.rs` — `build` fills `desc_templates` inside the corpus
    walk it already runs, under
    `class_feature_record_tokens_pre_gate_safe`'s **own guards, term for term**: both `name`
    and `class` present, a present-but-bad description (`.CLEAR`, a PI marker) claims nothing,
    an absent description is admitted as an empty template. The keying rule is first record
    wins, which is that table's rule, over the same book-sorted walk.
  - `src/pcgen_import/mod.rs`, `src/rules_core/mod.rs` — the two new modules registered.
  - `src/bin/gen_record_vars.rs` — `desc_templates=<n>` added to the summary line. Summary text
    only; `--check` proves the artifact bytes are unchanged by it.

  **The data — one regenerated artifact, and not one corpus record rewritten:**
  - `data/converted/record_vars.json` — 3.78 MB → 8.91 MB, `desc_templates` = **16,508
    records**, 11,786 of them carrying ops; op census `text=16138 arg=4315
    percent_or_drop=181 missing_arg=28`. **`git status --porcelain` lists no file under
    `data/corpus/`**: every ingested record, its license block and its `pi_*` stamps are
    byte-identical, and no corpus walk, count or licence gate moved.

  **Tests (all four whole-corpus, none a fixture roster):**
  - `pcgen_import::desc_template_convert::tests::every_described_class_feature_record_settles_to_the_same_prose_the_renderer_produced`
    — every described `class_feature` record under `data/corpus/`, six value environments each,
    comparing `text` **and** `dropped_args`.
  - `…::every_other_record_kinds_description_settles_the_same_way_too` — every other record
    kind's `description`, two environments each.
  - `…::the_shapes_the_renderer_documents_settle_the_way_it_describes_them` — the thirteen
    shapes the renderer's own module doc names, kept readable beside the exhaustive sweeps.
  - `class_feature_grant_consumer::tests::the_settled_description_artifact_covers_every_record_this_module_serves`
    — the served table's keys against the artifact's, **key for key, both directions**.
  - `class_feature_grant_consumer::tests::every_served_record_renders_the_same_sentence_from_its_settled_description`
    — the live table's own records rendered both ways, six environments.
  - Ten unit tests in `rules_core::desc_template` state each live behaviour on its own.

  - `…/AT-35-E6-003-RULED_cycle16_runtime_import_census.py` / `.json` — **new.** Imports cycle
    15's census whole (which imports cycle 14's, … back to cycle 1's), **widens no regex and
    narrows none**, and adds six assertions of its own.
  - `docs/retro/events/at-35-e6-003-ruled.jsonl` — 1 `correction`, 1 `deferral`.
  - `progress.md`, `kanban.md`, this receipt.
  - **Folded from the shared checkout, not authored work:**
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    (`derived_at` restamped by this cycle's `completion_atlas.py --check`) and
    `docs/retro/events/sd31-transcribe.jsonl` (appended events from another session).
    Committed rather than filtered away, per the standing "clean tree = unfiltered
    `git status` empty" rule.

  **`docs/work-inventory.json` is byte-identical**, which is correct: Epic 6 closes zero corpus
  units and no corpus record changed.

- **Identifier audit result:** OK_NO_BUNDLE_TAGS. Over this cycle's own added lines
  (`git diff 40f5d4f67a -- src/ tests/ apps/desktop/src-tauri/src/ | grep '^+' | grep -v '^+++'`,
  plus the two new Rust files in full), `grep -cE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  prints **0**. First run, no self-heal. The prescribed tranche-wide form of the same grep
  (`git diff --unified=0 fe5ae6cd4a...HEAD -- <scoped paths>`) prints **162**, every one of them
  a pre-existing `sd<N>_` **test-file name** or doc citation landed by an earlier cycle, unchanged
  here (cycle 15 recorded the same figure at 155; the rise is this cycle's own diff lines quoting
  `sd35_race_trait_prose_…`-style test names in prose, not new tags).

- **Wired-integration audit result:** OK_NO_TOKENS, after **one self-healed violation**. The
  first run of the prescribed grep over this cycle's own lines returned a single hit — the word
  *placeholder* in `desc_template.rs`'s module doc, describing the renderer's contract rather
  than marking a stub. Reworded to *slot*; the re-run prints `OK_NO_TOKENS`. Nothing in this
  cycle's shipping code returns a "would have…" string, serves fixture data, or leaves a handler
  empty: every new code path is exercised by the whole-corpus proofs above, on the real corpus.

- **Acceptance criterion** (verbatim, `epic-breakdown.md` `### AT-35-E6-003`, the criterion this
  `-RULED` variant carries under rulings B15/B16 — `decisions.md` §18, §19):

  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers
  > of `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc`
  > is deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  The evidence sentence's first clause is **met and held**: `root apps/desktop files=0 hits=0`,
  `apps_desktop_hits=0 evidence_sentence_met=YES`. Its second and third clauses run at the epic
  wrap-up: **this cycle wrote no line under `apps/`** (`git diff --name-only 40f5d4f67a..HEAD --
  apps/` is empty), so the desktop crate and frontend are not re-run here, per §6 step 3. The
  `render_pcgen_desc` clause is now true of the **live side** in the strongest sense the gate can
  state: `pattern render_pcgen_desc files=0 hits=0`, and the consumer that called
  `render_pcgen_desc_with_values` names it nowhere in shipping code
  (`class_feature_grant_consumer_shipping_pcgen_import_hits=0 (was 5)`). The renderer itself is
  **kept on the converter side**, which `decisions.md` §11 requires.

- **Receipt rows (mechanical):**
  ```
  since=40f5d4f67a97592595325604ba4de0116f97e58b target_dir=/tmp/cargo-sd35-AT-35-E6-003-RULED residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=982 ratio=n/a builds_recorded=4 pcgen_live_files=5
  ```
  `closed=0` is correct and expected: Epic 6 closes zero corpus units by design and no corpus
  record changed, so `docs/work-inventory.json` is byte-identical before and after.
  `pcgen_live_files=5` is the **file** count, down one from cycle 15's 6 — the third consecutive
  cycle to clear a whole file. The **hit** count moved further: 10 → 5.
  `rust_lines_changed=982` is the committed Rust diff, the two new modules included; the 5.1 MB
  the converted artifact gained is generated, not written by hand, and is not counted as source.

- **PCGen residue:** `live_files=5 live_hits=5 baseline_files=260 baseline_hits=12736
  verdict=PASS` — files down 1 from cycle 15's 6, hits down 5 from 10, **and the instrument was
  not touched this cycle** (`git diff --name-only 40f5d4f67a..HEAD -- scripts/` is empty).
  Per-root:
  ```
  root src/rules_core         files=6 hits=10  ->  files=5 hits=5
  root src/saved_character    files=0 hits=0
  root src/campaign           files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop           files=0 hits=0   (unchanged; no Rust line under apps/ was written)
  ```
  Not gate-gaming: nothing renamed to duck a regex, no path exempted, no rebaseline, and the
  renderer group's regex is **byte-identical to cycle 15's** — it simply matches nothing under a
  live root any more. The `use`-collapse still available in `source_content_payload` was
  **refused for the tenth cycle running**. The gate's own self-test is green (`Ran 27 tests, OK`).

- **Oracle parity:** N/A for the pinned PCGen oracle — no `Number` mapping was added, no
  converter mapping row changed, and `data/sheet_rules/` is byte-identical
  (`sheet_rule_convert -- --check` → `verdict=PASS`). The parity this change required is **four
  new whole-corpus proofs**, and they are the reason the swap is safe rather than plausible.
  Each renders the real records **both ways** — once through the settled template, once through
  `render_pcgen_desc_with_values` on the same stored string — and compares `text` **and**
  `dropped_args` field for field, because a settling that silently resolved something the
  renderer refused would pass a text-only comparison and then print a guess on a sheet. Six
  value environments per record are derived from that record's **own** slots (nothing bound,
  everything bound, every other one bound in each parity, a negative binding, and only the
  non-offset names bound), which is what exercises the sign swallowing and the whitespace
  collapse against surviving text. **All four green, 0 disagreements**, on populations the tests
  assert floors for (`compared >= 3_000` described class_feature records with `with_a_slot >=
  200`; `compared >= 5_000` descriptions across `>= 5` other kinds; the served table
  `>= 10_000` keys). Kept alongside them and unchanged: `class_feature_prose_parity_census`, the
  cycle-2 measurement of the **different**, still-refused substitution (the converted rule's own
  prose, 97,332 of 660,320).

- **Movement, four buckets:**
  - **closure:** 5 live `pcgen_import` hits and **1 whole file**, named by row and asserted by
    file **and symbol** in the census (`closed_by_cycle16=4` symbol tuples covering the 5 hits,
    `cleared_files_by_cycle16=1`) — `class_feature_grant_consumer.rs`'s two
    `PcgenDisplayValues::new`, two `render_pcgen_desc_with_values` and one
    `desc_token_arguments`. The census asserts the file names `pcgen_desc`,
    `PcgenDisplayValues`, `render_pcgen_desc_with_values`, `desc_token_arguments` and
    `raw_tokens` **nowhere** in shipping code rather than inferring it from the gate's file
    list. **Zero corpus units**, as Epic 6 closes none.
  - **relabel:** **none**, and booked explicitly as none (`relabelled_by_cycle16=0`). The calls
    did not move to another live file; they left the live side for an authoring-time producer,
    so all of `10 → 5` is closure.
  - **reachability:** none.
  - **instrument-correction:** none. `scripts/pcgen_residue_gate.py` and
    `scripts/pcgen-residue-baseline.env` are absent from this cycle's diff. (The SD-34 atlas
    artifact restamped by `completion_atlas.py --check` is *that* instrument re-deriving its own
    output on an unchanged claim; it closes nothing and is folded, not claimed.)

- **Refused tokens:** `source_content_payload=3, ingest_record_tokens=1, trait_and_pool_tokens=1`
  — **5 hits / 5 files, summing, all under `src/rules_core/`.** Three groups, under this cycle's
  flag-cap of 10, and one group fewer than cycle 15 had. Emitted as a `deferral` retro event
  naming each mechanism and the measured number it is refused on.

- **Discoveries:**
  - **The gap cycle 2 measured was never the gap in the way.** Cycles 2 through 15 carried the
    `renderer` group forward on one number — 97,332 of 660,320 — and that number is about a
    **content** substitution: rendering the converted RULE's prose instead of the record's
    stored description. Removing the converter CALL never required making those two agree. The
    stored description could be settled **as itself**, and then the only thing to prove was that
    the settling renders what the renderer rendered — which is a mechanical property, provable
    exhaustively on the real corpus in an afternoon. Fourteen cycles measured the wrong
    precondition. The lesson generalises past this group: *a refusal carried forward on a
    measured number is still only a refusal of the option that was measured.*
  - **Cycle 15's "last converter call of any kind" was wrong, and the gate is why it went
    unnoticed.** Three shipping calls remain in `derived_evaluator_fixture_check.rs`
    (`ingest_record::first_token_value` ×2, `ingest_record::token_values`), at lines 668, 1011
    and 1380. The gate counts that file's `use` line — **one** hit — so a call count read off
    the gate's hit count is wrong by construction. This is the same shape ruling B16 was written
    for: the instrument is a proxy, and a proxy read as the thing it proxies for is where this
    program keeps losing figures. Logged as a `correction` with the command that establishes it,
    and the cycle-16 census now **asserts those three by file and symbol**, so the claim cannot
    be restated wrongly again — and fails loudly if they ever do leave, rather than going stale.
  - **The two halves of the split fall out of the source text, not out of taste.** Every branch
    of the renderer's scan is either decidable from the source alone or decidable only from the
    text already built, and the line between them is sharp: the two `%`-exemptions that read
    *the source prose* settle at authoring time, while the one that reads *the
    output* (`%%` after a slot that may have dropped) must stay live. Getting that line wrong in
    either direction is a rendered sheet line, and it is exactly what the six value environments
    per record are there to catch. (The two source-side exemptions are percentile-dice notation
    and a per-cent sign already attached to a literal number; both look only at the source prose
    behind the marker, never at the output.)
  - **An escape can be formed across a dropped slot, so "which escapes to decode" cannot be
    decided per literal segment.** The decode runs over the finished text; a slot dropped
    between `&n` and `l;` would form one that no single settled segment contains. Every escape
    begins with `&` and a filled slot only ever contributes digits, so the template carries all
    five pairs whenever the prose contains an `&` and none otherwise — airtight, and cheap.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=5 live_hits=5`, was `6 / 10` | every non-comment, non-`#[cfg(test)]` line under the five live roots | `python3 scripts/pcgen_residue_gate.py --check` |
  | 5 hits split `source_content_payload=3, ingest_record_tokens=1, trait_and_pool_tokens=1`; `gate_agreement=OK (5 == 5)` | the same 5 hits, classified | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle16_runtime_import_census.py` |
  | `closed_by_cycle16=4 relabelled_by_cycle16=0 cleared_files_by_cycle16=1`; `class_feature_grant_consumer_shipping_pcgen_import_hits=0 (was 5)`; `desc_template=live_serde_only converter_names_in_reader=0`; `desc_template_convert=present reused_renderer_readings=3`; cycles 10/11/12/14/15's cleared files still clear | the 10 hits at cycle start vs the 5 at HEAD | the census's own row, file and symbol assertions, same command |
  | `remaining_shipping_converter_calls=3` at `derived_evaluator_fixture_check.rs:668, :1011, :1380` | every shipping (non-comment, non-`#[cfg(test)]`) line of that file | the same census command |
  | `settled_descriptions=16508 non_empty=11786 arg=4315 missing_arg=28 percent_or_drop=181 text=16138` | every `class_feature` record under `data/corpus/` with a `name`, a `class` and an admissible description | the same census command |
  | `desc_templates=16508 … verdict=PASS` (the shipped artifact is not stale) | the whole conversion, re-run and compared byte for byte | `cargo run --locked --bin gen_record_vars -- --check` |
  | the four whole-corpus parity proofs green, 0 disagreements each; floors asserted at `compared >= 3_000` / `with_a_slot >= 200`, `compared >= 5_000` over `>= 5` kinds, and `>= 10_000` served keys | every `data/corpus/<book>/**/*.json` record carrying a `description`, and the live served table | `cargo test --locked --lib -j 6` (`desc_template_convert::tests::*`, `class_feature_grant_consumer::tests::{the_settled_description_artifact_covers_every_record_this_module_serves,every_served_record_renders_the_same_sentence_from_its_settled_description}`) |
  | `records=49438 converted=49296 refused=142 rules=70135 var_tables=5293 verdict=PASS` — `data/sheet_rules/` byte-identical | every corpus record the converter reads | `cargo run --locked --bin sheet_rule_convert -- --check` |
  | 0 files under `data/sheet_rules/` carry an ingest-format literal | the whole `data/sheet_rules/` tree | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | 3,380 lib tests pass, 16 ignored (cycle 15 recorded 3,365 passing; the delta is **exactly** this cycle's fifteen new tests — ten in `desc_template`, three in `desc_template_convert`, two in the consumer) | the crate's own unit tests | `cargo test --locked --lib -j 6` |
  | 8,909 tests pass across 419 targets + 1 Doc-tests, 0 failed, 69 ignored (cycle 15 recorded 8,894; the delta is the same fifteen tests, reached through the lib) | every target in the root workspace | `cargo test --locked --no-fail-fast -j 6` |
  | clippy clean: 0 warnings, 0 errors | every target and every test target in the root workspace | `cargo clippy --locked --tests -j 6 2>&1 \| grep -cE '^(warning\|error)'` |
  | the residue gate's own self-test green, `Ran 27 tests, OK` — B15 and B16 still pinned | the gate's test suite | `python3 -m unittest scripts.tests.test_pcgen_residue_gate` |

- **Build scope verified:** root workspace in `/tmp/cargo-sd35-AT-35-E6-003-RULED`, once, at the
  final tree: `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`; `cargo test --locked --lib
  -j 6` → 3,380 passed / 0 failed; `cargo test --locked --no-fail-fast -j 6` → `FULL_EXIT=0`,
  8,909 passed / 0 failed across 419 targets + Doc-tests. **ONE verification pass, no red, no
  re-run** — `decisions.md` §3 as written. The desktop crate and frontend run at the **epic
  wrap-up**, not here, because this cycle wrote no line under `apps/`
  (`git diff --name-only 40f5d4f67a..HEAD -- apps/` is empty). One change landed after the
  workspace run started — `gen_record_vars.rs`'s summary-line addition — and it moves no figure
  (`--check` proves the artifact bytes unchanged); the final tree was re-proved with
  `cargo test --locked --no-run -j 6` (exit 0) and `cargo clippy --locked --tests -j 6`
  (0 warnings) at the end.

- **Sweep population:** N/A — `corpus_literal_sweep` did **not** run, and correctly so: it is
  required "only when corpus records changed", and **no file under `data/corpus/` was written**
  (`git diff --name-only 40f5d4f67a..HEAD -- data/corpus/` is empty). The one `data/` file this
  cycle changed is the generated `data/converted/record_vars.json`, which carries no corpus
  record, no licence block and no `pi_*` stamp. For the same reason
  `cargo run --locked --bin v06_work_inventory` did not run: with no corpus record changed the
  inventory cannot move, and `docs/work-inventory.json` is byte-identical before and after.

- **Oracle pin:** N/A — no figure in this receipt came from the pinned PCGen corpus.

- **Status:** `partial`.

- **Notes:** The `source_content_payload` trim — repointing an import at
  `rules_core::source_content`'s own re-export of the same enum, for `−1` and zero change in what
  the module depends on — is **refused for the tenth cycle running**, on the same reasoning
  cycles 7 through 15 gave.

- **Next-cycle scope:** **AT-35-E6-003-RULED cycle 17**, `SCOPE_GATE: EXEMPT (Epic 6 cycle)`, on
  the 5-hit remainder. The largest piece is **`source_content_payload` (3)** and it clears the
  way cycle 13 cleared `Equipment`: convert the four remaining `SourceContentPayload` variants
  that still borrow a parser entry type (`Class`, `SpellcastingClass`, `Race`/`Ability`,
  `Metadata`), after which the enum itself moves to the live side and all three hits go together.
  `ingest_record_tokens` (1, `derived_evaluator_fixture_check`) is a **relocation** cycle, not a
  settling one — the file is a 5,754-line verification module holding **three** shipping
  converter calls behind its single counted hit, and it leaves `src/rules_core/` with its
  bar-check reports or not at all; moving only the calls behind a name the gate does not match
  is the blind-spot shape ruling B16 exists to punish and must not be done.
  `trait_and_pool_tokens` (1, `class_feature_pool_catalog`) stays refused on cycle 7's corrected
  number (1,870 of 18,043 P1, 864 P2, 89 P3).
