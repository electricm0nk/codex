# Cycle 1 — Epic 6 (PCGen exit) / AT-35-E6-003-SWEEP

This cycle was dispatched to take **the entire remaining live-PCGen surface in one batch**, with a
hard floor of **25 live files removed** and the standing rule that *"the count must fall because
the reads are gone"* — never by lowering the baseline, silencing the gate, or adding an exclusion
list.

It measured the remainder first, as step 1 requires, and found the two constraints **cannot both
be satisfied**. The floor is not merely hard on this remainder; it is **arithmetically unreachable
by code work**. This receipt is the under-floor report the dispatch mandates for that case
("report that and stop, exactly as an under-floor corpus cycle does"), plus the ruling request the
orchestrator needs to re-scope.

- **Commit SHA:** `<this cycle's commit>` — cycle start `359e5f050f4b3a50573fdf4473a1e4f8190b5c25`.
  This cycle writes **no source file**. Its commit carries the census script, this receipt, the
  two retro events, and the board rows. (A receipt cannot name the commit that carries it; the
  SHA is recorded in `progress.md`.)

- **Scope gate:**
  `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design, decisions.md §2)`.
  Run anyway, for the record — `python3 scripts/cycle_scope_gate.py --min 500`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  `remaining_non_done=0` — the corpus reached `DONE` at `AT-35-E5-005`. The 500-unit corpus floor
  does not apply. **This cycle's own 25-file floor is the one it failed**, and §8's escalate list
  names exactly that case: *"a cycle under the floor that is not the whole remainder"*.

- **Files touched:** **0 tracked source paths.** Three bundle/log paths only:
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle1_residue_shape_census.py`
    — the instrument this receipt's headline figure comes from. It runs the real
    `pcgen_residue_gate.py --check --list-files`, takes **its** file list, and splits **its** regex
    hits per file into `comment` (the left-stripped line starts `//`) and `code` (everything else).
  - `docs/retro/events/at-35-e6-003-sweep.jsonl` (+2 — one `correction`, one `deferral`).
  - This receipt, `progress.md`, `kanban.md`.

- **Identifier audit result:** **OK_NO_BUNDLE_TAGS.**
  ```bash
  git diff --unified=0 359e5f050f4b -- src/rules_core src/pcgen_import \
      apps/desktop/src-tauri/src apps/desktop/src \
      docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit \
      ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -cE '^\+.*\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'          -> 0
  ```
  For the record, the same grep over the **whole branch** (`merge-base HEAD origin/develop` =
  `fe5ae6cd4a5f3c65d5d10f4d523f00e33b04ac47`) returns `28`. Those are Epics 1–6's own prior
  commits, already audited in their own receipts, not this cycle's.

- **Wired-integration audit result:** **OK_NO_TOKENS.**
  ```bash
  git diff --unified=0 359e5f050f4b -- <same paths> \
    | grep -ciE '^\+.*\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'  -> 0
  ```
  Branch-wide, the same grep returns `90` — again prior cycles, not this one. This cycle adds no
  shipping code at all, so there is nothing for the four-check audit to find.

- **Acceptance criterion** (verbatim, `epic-breakdown.md`):

  > ### AT-35-E6-003 — the desktop crate and the prose renderer leave PCGen behind
  >
  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers of
  > `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc` is
  > deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  **Not met, and not advanced by this cycle.** `apps/desktop/` stands where cycle 12 left it: **1
  file / 33 hits** (`race_trait_picker.rs`). The sweep's own floor is what stopped the cycle; see
  **The finding** below.

- **Receipt rows (mechanical):**
  ```
  CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E6-003-SWEEP \
  python3 scripts/cycle_scope_gate.py --receipt --since 359e5f050f4b3a50573fdf4473a1e4f8190b5c25 \
    --before /tmp/wi-before-AT-35-E6-003-SWEEP.json --after docs/work-inventory.json
  since=359e5f050f4b3a50573fdf4473a1e4f8190b5c25 target_dir=/tmp/cargo-sd35-AT-35-E6-003-SWEEP residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=197
  ```
  `rust_lines_changed=0` and `builds_recorded=0` are the honest record: this cycle wrote no Rust
  and consumed no build budget. `pcgen_live_files` is **unchanged at 197** — it did not rise.

- **PCGen residue:** `python3 scripts/pcgen_residue_gate.py --check`, unchanged start to end:
  ```
  pattern raw_tokens files=1 hits=8
  pattern raw_bonus_chains files=0 hits=0
  pattern PcgenFormulaEvaluator files=0 hits=0
  pattern render_pcgen_desc files=5 hits=56
  pattern bonus_stack_reader files=0 hits=0
  pattern pre_tokens files=0 hits=0
  pattern BONUS: files=116 hits=2066
  pattern DEFINE: files=24 hits=114
  pattern PRE[A-Z]+: files=107 hits=7877
  pattern SAB: files=0 hits=0
  pattern DESC: files=103 hits=430
  pattern %CHOICE files=6 hits=60
  pattern %LIST files=15 hits=120
  pattern TYPE= files=59 hits=716
  root src/rules_core files=196 hits=11414
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=1 hits=33
  identifier_files=6 identifier_hits=64
  live_files=197 live_hits=11447 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

- **Oracle parity:** N/A — no live path was touched and no `Number` mapping was added.

---

## The finding

`python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle1_residue_shape_census.py`

```
live_files=197
comment_hits=2686 code_hits=3628
files_comment_only=114
files_with_code_hits=83
files_still_hitting_after_every_code_read_removed=187
max_files_clearable_by_code_work_alone=10
```

Read those last two lines carefully. **If every single live-side code read of the ingest format in
this repository were removed tonight, 187 of the 197 files would still hit the gate**, because
their remaining hits are doc-comments. The most a code-only sweep can ever clear is **10 files** —
less than half the cycle's 25-file floor, and 10 of the 197 files it was told to take whole.

**114 of the 197 live files carry no code hit at all.** Their code has *already* left PCGen. What still hits
the gate in them is provenance prose of exactly this shape:

```rust
// src/rules_core/rules_tables/acg/class_arcanist.rs — 3 hits, all comments
//! - `BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")/2` — poor/half BAB.
//! - `BONUS:SAVE|BASE.Will|classlevel("APPLIEDAS=NONEPIC")/2+2` — good Will save.
//! - `BONUS:SAVE|BASE.Fortitude,BASE.Reflex|classlevel("APPLIEDAS=NONEPIC")/3` — poor Fort/Ref.
```

That is not a read. It is the record of **where the number in the table below it came from** — the
audit trail `AGENTS.md` rule 9 ("every figure you write down carries the command that produced
it") exists to demand.

### Why this is a ruling request and not a cycle the sweep could have finished

`scripts/pcgen_residue_gate.py`'s own module doc states the rationale for counting comments:

> A mention inside a comment or a doc string counts — the ruling is "nothing left of pcgen", and a
> comment explaining a PCGen token on the live side **is a sign the code next to it still needs
> one**.

For the 114 comment-only files, **that sign is false and demonstrably so** — the census proves the
code next to those comments needs nothing. The gate's premise held when it was written (2026-09-07,
against a 260-file baseline of mostly-live reads); Epics 1–6 have since drained the code side and
left the prose, and the instrument has not been re-validated against that new population. This is
the `validate-proxies-against-known-truth` failure shape: a proxy still making a confident claim
in a region where it was never tested.

So the sweep faced two doors, and the dispatch closes both:

1. **Delete the ~2,686 provenance comments.** This clears the floor and would clear most of the
   epic. It is also precisely "making the number fall by means other than the reads going away" —
   the reads in those files went away *cycles ago*; only the audit trail would be going away now.
   Destroying 2,686 lines of derivation provenance to move a grep count is the forbidden shape,
   and it is irreversible in every surface a reader actually consults.
2. **Do code work only.** Ceiling of 10 files, hard. Under the floor by 15.

Neither is a judgment call a cycle may make alone, which is why this stops here rather than
grinding a 23rd ~2.8-file cycle.

### What the orchestrator needs to decide

**Does a live-side doc-comment that quotes an ingest-format token count as a PCGen read for
`AT-35-E6-004`'s `--closure` mode?**

- **If yes** (the ruling stands as written): Epic 6 must budget a **provenance relocation** pass —
  the comments move to the converter side or to an artifact next to the converted rule they
  explain, so the derivation survives where a reader can still find it. That is a real, bounded,
  mechanical body of work (187 files / 2,686 lines) and it should be dispatched as such, with a
  file floor it can actually meet. It must not be dispatched as "a code sweep", because it is not
  one — that framing is what produced this cycle.
- **If no**: `pcgen_residue_gate.py` gains a **comment-aware read** — not an exclusion list, not a
  baseline change, but the same distinction this census script already implements, applied inside
  the gate and pinned by `scripts/tests/test_pcgen_residue_gate.py`. The live count then reads
  `code_files=83 code_hits=3628`, and `AT-35-E6-004`'s closure target becomes a surface a code
  sweep can genuinely take. **Note this is an instrument change and `instrument-correction is not
  closure`:** the 83 remaining code-bearing files would still be the epic's real work, and the
  ruling would have to say so explicitly so the drop from 197 to 83 is never read as progress.

This cycle takes **no** position on which. Either is defensible; only the operator may pick, and
picking one silently from inside a cycle is how a gate gets quietly weakened.

### The 83 code-bearing files, by mechanism

Named so the next dispatch can group by mechanism rather than by file, whichever way the ruling
goes. Counts are `code` hits from the census.

| mechanism | what the code actually reads | files | code hits |
|---|---|---|---|
| `prerequisites: Some(&["PRE…"])` on the static feat/archetype tables | **dead on the live side.** `feat_prereqs::evaluate_catalog_feat_prerequisites` reads the **converted** `Applies` gate via `converted_gate::verdicts`; the field's only non-test consumer is `src/pcgen_import/cache_gen/hand_authored_feat_dump.rs`, which is converter side. This is a **move to `src/pcgen_import/`**, not a rewrite. | ~30 | ~2,400 |
| `FeatEffectBonus { qualifiers: &["VAR", …, "TYPE=Base", "PREABILITY:…"] }` | live `BONUS:` qualifier arrays read by `feat_effects.rs` / `equipment_effects*.rs` | ~12 | ~250 |
| `render_pcgen_desc` / `render_pcgen_desc_tokens` / `render_pcgen_desc_with_values` | live `DESC:` rendering. Replacement already exists and is proven: `sheet_rule_catalog::catalog_description_or_fields`, built and shipped by cycle 12 against `reference_library_catalog.rs`. Real call sites: `class_feature_pool_catalog.rs` (×3), `pilot_compute/class_feature_grant_consumer.rs` (×2), `pilot_compute/mod.rs` (×1), `race_resolver.rs` (×1). | 7 | 56 |
| `description: Some("…|VarName")` argument tails in the static tables | the `%N` tail `render_pcgen_desc` was built to strip; the converter already substitutes it | ~20 | ~400 |
| `raw_tokens` | `apps/desktop/src-tauri/src/race_trait_picker.rs` — the last desktop file | 1 | 8 |

The two largest single files are `src/rules_core/rules_tables/feat_gap_tables.rs` (601 code, 1
comment) and `src/rules_core/pilot_compute/mod.rs` (435 code, **885 comment**).

---

- **Movement, four buckets:**
  - **closure:** none. No unit moved; Epic 6 moves none by design.
  - **relabel:** none.
  - **reachability:** none.
  - **instrument-correction:** **one, and it is this cycle's whole product.**
    `pcgen_residue_gate.py`'s `live_files` was being read — by the dispatch, and by 22 prior
    cycles' cadence — as a count of live PCGen **reads**. It is a count of files containing PCGen
    **text** — 2,686 of the 6,314 hit lines in those 197 files are provenance prose sitting next to
    code that is already clean. Emitted
    as a `correction` retro event (`claimed=197 actual=10`, verified by the census script).
    Per `instrument-correction-is-not-closure`: this correction closes **nothing**. The 83
    code-bearing files are exactly as unfinished as they were before it was written.

- **Refused tokens:** none. No token type was refused, because no conversion was attempted. The
  remainder this cycle did not take is **197 live files**, not a token population; it is named by
  path in full by
  `python3 scripts/pcgen_residue_gate.py --check --list-files` and, with its comment/code split,
  by the census script committed alongside this receipt. The `deferral` retro event names it.

- **Discoveries:** **one** — that 114 of the residue gate's 197 live files are provenance prose
  rather than live reads, and that the epic's remaining code surface has a hard ceiling of 10 clearable
  files. Neither `token-coverage.json` nor the completion atlas predicts this; neither measures
  the live side. Emitted as the `correction` event above.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=197 live_hits=11447` | all files under the five live roots | `python3 scripts/pcgen_residue_gate.py --check` |
  | `comment_hits=2686 code_hits=3628` | the 6,314 gate-regex hits on one line each in those 197 files (the gate's own `live_hits=11447` counts every regex match on a line; this census counts lines) | `python3 …/AT-35-E6-003-SWEEP_cycle1_residue_shape_census.py` |
  | `files_comment_only=114` | of 197 live files | same |
  | `files_with_code_hits=83` | of 197 live files | same |
  | `max_files_clearable_by_code_work_alone=10` | of 197 live files | same |
  | `197 - 187 = 10` | — | `files_still_hitting_after_every_code_read_removed=187`, same script |
  | `apps/desktop = 1 file / 33 hits` | the desktop root only | `python3 scripts/pcgen_residue_gate.py --check` |
  | branch-wide identifier/token audit `28` / `90` | `merge-base HEAD origin/develop`…HEAD over the scoped paths | the two greps above |
  | `prerequisites` has no live consumer | all of `src/` + `apps/desktop/src-tauri/src/` | `grep -rn "prerequisites" src apps/desktop/src-tauri/src --include=*.rs \| grep -vE "rules_tables/\|^src/pcgen_import/"` — every hit is `failing_prerequisites` (a different field), a test, or converter side |

- **Build scope verified:** **not run, deliberately, and this is the one place the cycle departs
  from §6 step 3's checklist.** The cycle changed no Rust, no corpus record, no classifier and no
  generated artifact; `git diff --stat` over `src/`, `apps/`, `data/` and `scripts/` is empty. A
  full workspace suite here would verify a tree byte-identical to `359e5f050f4b`, which cycle 12
  already verified green (575 desktop tests, 101/101 frontend files, 19 on-screen tests). Spending
  the cycle's whole build budget to re-prove an unchanged tree is the `one-pass-verify` rule read
  backwards. The gates that **do** read the tree rather than the build were run, and are quoted
  above: `cycle_scope_gate.py --min 500`, `cycle_scope_gate.py --receipt`, and
  `pcgen_residue_gate.py --check` (start and end, identical). Last green build of this tree:
  cycle 12, at `798bf8ebdad4ba25124bbfde40dd6e858bfdab40`.

- **Sweep population:** N/A — no corpus record changed, so `corpus_literal_sweep` does not apply.

- **Oracle pin:** N/A — no figure in this receipt came from the pinned corpus.

- **Status:** **blocked-escalated.**
  §8's escalate list names this case twice over: *"a cycle under the floor that is not the whole
  remainder"*, and *"a launch gate not actually met"* — `AT-35-E6-004`'s closure mode cannot reach
  `live_files=0` by any code work, and no cycle has been dispatched against the 2,686 comment hits
  that make up the difference. The blocker is a **ruling**, which `AGENTS.md`'s blocker discipline
  puts squarely in disposition 2: escalate, name what blocks you, name the precondition you need,
  then stop. Named above, in one question.

- **Notes:** The dispatch's diagnosis of Epic 6 — *"the per-file cadence is a defect, not a
  style"* — is right, and the cause is now measured: the epic is being driven by an instrument
  whose numerator is prose in 114 of its 197 files, so every cycle's code work moves the count by
  about one file and looks
  like grinding because it **is** grinding. The fix is a ruling and a correctly-shaped dispatch,
  not a bigger batch of the same work.

- **Next-cycle scope:** blocked on the ruling. Once it lands, both branches are already scoped:
  - ruling **yes** → dispatch the provenance relocation, 187 files / 2,686 lines, floor set in
    files-per-cycle against that population, grouped by directory.
  - ruling **no** → one cycle to make `pcgen_residue_gate.py` comment-aware (pinned by its own
    unit test, baseline **not** touched, and the ruling written into the gate's module doc so the
    197→83 drop is never misread as progress), then dispatch the 83 code-bearing files grouped by
    the five mechanisms tabled above. The `prerequisites`-field move alone is ~30 files and ~2,400
    of the 3,628 code hits, and it is a relocation to `src/pcgen_import/` with **zero** net
    deletion of function bodies.
