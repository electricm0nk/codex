# Cycle AT-35-E3-004_cycle2 — Epic 3 — Place and surface / AT-35-E3-004

Cycle 2 is a **re-dispatch of an already-`complete` criterion**, and unlike a pure re-verification
it found real work: the ledger cycle 1 wrote was **three cycles stale**. `EPIC-3_wrapup_fix_cycle`,
`AT-35-E3-002_cycle2` and `AT-35-E3-003_cycle2` all landed after cycle 1's `verified_at` sha, so
the artifact's own `completeness` claim — "the epic's cycle set is closed at this sha: 5 receipts,
5 rows" — was false at HEAD. This cycle transcribes the three missing rows, adds its own, and
replaces the one-time completeness claim with a `staleness_rule` that names the re-derive command.

- **Commit SHA:** `<pinned by the follow-up docs commit>` — the docs commit carrying this receipt,
  `artifacts/epic-3-place-and-surface/rate-ledger.json`, the `progress.md` entry, the `kanban.md`
  row 15 update, `docs/retro/events/at-35-e3-004-c2.jsonl` and two generated side effects the gate
  scripts stamped (`docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
  `derived_at`, `docs/retro/events/sd31-transcribe.jsonl` append). Cycle start
  `031e61595abadc8b47a67d8e6adf70532a429b81` on `tranche/15`. **No file under `src/**`,
  `tests/**`, `data/**`, `scripts/**` or `apps/**` changed** — `git diff --name-only 031e615959..HEAD`
  lists only `docs/`.
- **Scope gate:** `SCOPE_GATE: EXEMPT (ledger cycle — records this epic's per-cycle rows; closes
  zero units by design)` — `decisions.md §2` / `workflow-instruction.md §6` step 1's floor
  exemption, and the flag this cycle was dispatched with. The exemption is legitimate here: the
  cycle moves no unit. It is also **not needed to hide a shortfall** — the unfiltered gate was run
  anyway and agrees the population is empty: `python3 scripts/cycle_scope_gate.py --min 500` at
  `031e615959` → `scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER` (exit 0,
  `scoped_by_bucket=` and `scoped_by_kind=` both empty). **Nothing is exempt from the residue
  check:** `python3 scripts/pcgen_residue_gate.py --check` at start →
  `live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS` (exit 0).
- **Files touched:**
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-3-place-and-surface/rate-ledger.json`
    (the criterion's artifact: `verified_at` re-pinned, `staleness_rule` added, 4 `cycles` rows
    added, `totals` re-summed)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-3-place-and-surface/AT-35-E3-004_cycle2_receipt.md` (this file)
  - `docs/release/SD-35-corpus-sheet-completion/progress.md` (prepended entry)
  - `docs/release/SD-35-corpus-sheet-completion/kanban.md` (row 15 Notes pointer)
  - `docs/retro/events/at-35-e3-004-c2.jsonl` (this cycle's events)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` — **generated,
    not hand-edited**: `completion_atlas.py --check` re-stamps `derived_at`
    `a8c193f055` → `031e615959`; `git diff` on the file is exactly 1 insertion / 1 deletion
  - `docs/retro/events/sd31-transcribe.jsonl` — an append the gate run emitted, folded rather than
    left as tree litter
- **Identifier audit result:** OK_NO_BUNDLE_TAGS —
  `git diff --unified=0 fe5ae6cd4a...HEAD -- <Epic 3 file-touch set> ':!**/__tests__/**' ':!**/*.test.*' | grep -cE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  → **0**, over the whole set since the merge base, not just this cycle's diff.
- **Wired-integration audit result:** OK_NO_TOKENS on code. The same grep
  (`\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b`) restricted to the code paths
  in the file-touch set — `src/pcgen_import/sheet_rule/`, `src/rules_core/class_feature_pool_catalog.rs`,
  `src/bin/v06_work_inventory.rs` — returns **0**. Over the full set including `data/` and `docs/`
  it returns **23**, every one pre-existing and non-code: the three rulebook-prose strings inside
  `data/sheet_rules/**` `ProsePiece::Text` (Tophet "hack or smash", Plant Growth "hack or force",
  Courtly Companion "not yet implemented" — corpus text, recorded by AT-35-E2-002's correction
  `1788844812035-at-35-e2-002-7cbeb2`), the `engine_diagnostic:vacuous_placeholder_row_no_corpus_content_to_render`
  evidence strings in `docs/work-inventory.json`, and the prior Epic 3 receipts' own quotations of
  the audit pattern. The count grew from the 9 an earlier receipt recorded purely because each new
  receipt that quotes the pattern adds to it. **No stub, inline mock or `"Would …"` string in
  shipping code.** This cycle wrote no code at all.
- **Acceptance criterion:** verbatim from `epic-breakdown.md`, `### AT-35-E3-004 — the rate ledger`:
  "`artifacts/epic-3-place-and-surface/rate-ledger.json`: per cycle — mechanism, scoped population,
  units closed, units relabeled, wall time, `rust_lines_changed / units_closed`, `builds_recorded`
  (must be 1), `pcgen_live_files` (must not rise)."
- **Receipt rows (mechanical):** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=1 pcgen_live_files=253`
  — the literal last line of
  `python3 scripts/cycle_scope_gate.py --receipt --since 031e61595abadc8b47a67d8e6adf70532a429b81 --before /tmp/wi-before-AT-35-E3-004.json --after docs/work-inventory.json --target-dir /tmp/cargo-sd35-AT-35-E3-004`
  (`residue_gate=present`; `closed_by_kind=` empty; `relabeled_moves=` empty;
  `regressed=0 added=0 dropped=0`). `closed=0` over a scoped population of `0` is the whole
  population, not a shortfall — **no `deferral` event is owed** and none was emitted. `ratio=n/a`
  because the denominator is zero, not because the ratio was not taken. **`builds_recorded=1`, on
  target, and deliberately unlike cycle 1's `0`:** cycle 1 skipped `sheet_rule_convert --check` on
  the grounds that a docs-only diff cannot move it; this cycle ran it anyway and therefore paid one
  build, which the receipt tool counted mechanically from the target dir. The ledger row and this
  receipt agree on 1; the divergence from cycle 1 is named in the row's `note` so a reader does not
  read it as a transcription error.
- **PCGen residue:** `live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS`
  — identical at cycle start and at HEAD. **Not risen.** It is *below* the AT-35-E1-005 baseline of
  260/12736; the 7-file reduction is Epic 6 AT-35-E6-001's, not this cycle's, which wrote no code.
  Nothing new on the live side reads a PCGen token, and no converter, parser, generator or oracle
  file was deleted (`decisions.md §11`).
- **Oracle parity:** N/A — this cycle added no `Number` mapping, no mapping row of any kind, and
  touched no Epic 6 live path. Nothing entered the oracle-comparable set.
- **Movement, four buckets:**
  - **closure (into DONE, by id-set):** none. `closed_by_kind=` is empty; the corpus was already
    `DONE 49438 of 49438` at cycle start.
  - **relabel (bucket to bucket):** none. `relabeled_moves=` is empty.
  - **reachability:** unchanged. `shape_engine_boundary.py --check` →
    `magnitude_bearing=26396 not_held_by_engine=0`.
  - **instrument-correction:** one, and it is this cycle's deliverable — `rate-ledger.json`'s
    `verified_at.completeness` claimed the epic's cycle set was closed at 5 receipts / 5 rows; at
    HEAD there are 8 committed receipts and this cycle's makes 9. Emitted as correction
    `1789047488243-at-35-e3-004-c2-f20947`. **No prior row's figures changed** — the five rows
    written at cycle 1 are byte-identical after this edit; only the header block and the new rows
    moved.
- **Refused tokens:** none — this cycle scoped no units and refused none. (The standing
  corpus-wide refusal set is unchanged at 142, `sheet_rule_convert --check` →
  `records=49438 converted=49296 refused=142`, and `token_coverage.py --check` →
  `refused_non_done=0`, i.e. no refusal blocks a non-DONE unit.)
- **Discoveries:** one, and it is the reason this cycle existed. **A ledger criterion is a
  per-wave re-dispatch, not a one-time write.** Cycle 1 correctly verified and correctly recorded
  its completeness — pinned to `697b7780ea` — and then the epic took three more cycles. A
  completeness claim pinned to a sha is false the moment the next cycle lands, and nothing in the
  bundle re-opened the card. This is an *atlas-shaped* discovery in `workflow-instruction.md §8`'s
  taxonomy (a category the instrument did not predict), so it is a `correction` retro event plus
  the artifact re-derivation, not a `forward-scope-register.md` entry: the criterion's own text
  already says "per cycle", so the obligation was in the Definition of Done at launch. The
  mechanism that prevents recurrence is written into the artifact itself as `staleness_rule`, with
  the one-line re-derive check (`ls …/*_receipt.md | wc -l` must equal `len(cycles)`), so a reader
  or the Epic 7 scan can falsify the claim without reading this receipt.
- **Figures + their re-derive commands:**
  - **9 ledger rows against 9 receipts (8 committed + this one)** —
    `ls docs/release/SD-35-corpus-sheet-completion/artifacts/epic-3-place-and-surface/*_receipt.md | wc -l` → 8 before this commit;
    `python3 -c "import json;print(len(json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-3-place-and-surface/rate-ledger.json'))['cycles']))"` → 9
  - **totals `9 1404 0 535 11`** (rows, units_closed, units_relabeled, rust_lines_changed,
    builds_recorded) —
    `python3 -c "import json;c=json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-3-place-and-surface/rate-ledger.json'))['cycles'];print(len(c),sum(x['units_closed'] for x in c),sum(x['units_relabeled'] for x in c),sum(x['rust_lines_changed'] for x in c),sum(x['builds_recorded'] for x in c))"`
  - **`rust_lines_changed=0` on all four newly-added rows** — `git show --numstat --format= <sha> -- '*.rs' | awk '{a+=$1;d+=$2}END{print a+0,d+0}'`
    for `2dc322ae32`, `7d237da322`, `033970e470` and this cycle's own commit → `0 0` each. The
    `EPIC-3_wrapup_fix_cycle` row's 0 is **re-derived here, not transcribed**: that receipt carries
    no `Receipt rows (mechanical)` line because it is not a criterion cycle.
  - **corpus `DONE 49438 of 49438`, every other bucket 0 of 49,438** — `python3 scripts/completion_atlas.py --check`
    (`population=49438 buckets=10 unclassified=0 overlap=0`, `done_evidence_violations=0`,
    `missing_clearing_mechanisms=0`, `stale_derived_at=False`, `citation_failures=0`)
  - **token coverage `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1 verdict=PASS`**
    of 231 token types — `python3 scripts/token_coverage.py --check`
  - **`magnitude_bearing=26396 not_held_by_engine=0` of 49,438 units** — `python3 scripts/shape_engine_boundary.py --check`
  - **engine tables `population=0 kinds=0`, `citation_failures=0`** — `python3 scripts/missing_engine_tables.py --check`
  - **0 files of the whole `data/sheet_rules/` tree carry a PCGen token** —
    `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → 0
  - **denominator gate 0 violations of 76 files checked** —
    `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'`
  - **PI sweep PASS, 11 hits over 11 baseline rows** — `scripts/verify.sh --only pi-sweep`, 1 min 31 s
  - **converter `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS (20.2s)`** —
    `cargo run --locked --release --bin sheet_rule_convert -- --check`, cold build 2 min 18 s
  - **`pcgen_live_files=253`, `live_hits=12256`, against baseline 260/12736** —
    `python3 scripts/pcgen_residue_gate.py --check`
- **Build scope verified:** `cargo test --locked --no-run`, `--lib`, `--no-fail-fast`, `clippy`,
  `corpus_literal_sweep`, `v06_work_inventory` and the desktop/frontend suites were **NOT run and
  are not claimed.** `workflow-instruction.md §6` step 3 requires the workspace suite "when `src/`
  or the classifier changed" and `corpus_literal_sweep` "only when corpus records changed"; this
  cycle changed neither, and changed no Rust at all (`rust_lines_changed=0`), so there is no target
  whose result it could move. The one build it did pay is `sheet_rule_convert --check` at
  `031e615959` with `CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E3-004`, `CARGO_INCREMENTAL=0` —
  clean compile, exit 0, `verdict=PASS`. Desktop crate and frontend: **epic cadence** — no `apps/`
  path touched (`decisions.md §3`). The standing full-gate figures are the Epic 5 wrap-up
  correction cycle's **49 of 49 stages PASS in 3,916 s**
  (`artifacts/epic-5-residues/EPIC-5_wrapup_correction_cycle_receipt.md`).
- **Sweep population:** **N/A — no corpus record changed** (`git status --porcelain -- data/`
  empty throughout), so `corpus_literal_sweep` was not run and no sweep figure is claimed. The
  standing figure is AT-35-E3-002 cycle 1's **48,706 records examined of 51,476 read, 0 findings**,
  and it is not restated as current.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` — carried in every
  `data/sheet_rules/**` record's `provenance.oracle_pin`, and read by `sheet_rule_convert --check`
  through `$PCGEN_CORPUS_ROOT`. **No figure in this receipt was newly derived from the pinned
  corpus.**
- **Status:** **complete.** The criterion's artifact exists at
  `artifacts/epic-3-place-and-surface/rate-ledger.json` and now carries, for **every** Epic 3 cycle
  including this one, all eight fields the criterion names — mechanism, scoped population, units
  closed, units relabeled, wall time, `rust_lines_changed / units_closed` (as `ratio`, `null` where
  the divisor is 0), `builds_recorded` and `pcgen_live_files` — each transcribed from that cycle's
  own receipt, or re-derived from git where the cycle is not a criterion cycle and has no receipt
  row to transcribe. The criterion's **unit** population is 0 at HEAD (it moves no units by
  design). The two constraint clauses are **reported, not silently satisfied**: `builds_recorded`
  is 1 on five cycles, 0 on two and 3 on two — the two 3s are named as real overruns in their
  `note` rows per `decisions.md §3` — and `pcgen_live_files` **did not rise on any Epic 3 cycle**
  (260 for the first five, 253 for the last four, a fall attributable to Epic 6).
- **Notes:** (a) The dispatch numbered this cycle 1; `AT-35-E3-004_cycle1_receipt.md` already
  exists at `9a728d891f`, so this receipt is filed as cycle 2 and cycle 1 is left untouched —
  overwriting it would have destroyed the very record this criterion exists to keep. (b) The two
  generated side effects (SD-34 atlas `derived_at`, `sd31-transcribe.jsonl`) are folded into this
  cycle's commit rather than left as tree litter; neither was hand-edited. (c) `.worktrees/` stays
  untracked — it is a registered worktree container, not litter, and is not this cycle's to fold.
- **Next-cycle scope:** **criterion at zero, with a standing re-dispatch condition.**
  `python3 scripts/cycle_scope_gate.py --min 500` → `scoped=0 remaining_non_done=0
  verdict=PASS_WHOLE_REMAINDER`; Epic 3 has no successor cycle on any bucket and all four of its
  criteria are `complete`. The one condition that would re-open this card is mechanical and now
  written into the artifact: **if another Epic 3 receipt is ever committed, `ls
  docs/release/SD-35-corpus-sheet-completion/artifacts/epic-3-place-and-surface/*_receipt.md | wc -l`
  stops equalling `len(cycles)` and the ledger owes a row.** The Epic 7 final-acceptance scan
  should run that comparison rather than trusting `verified_at`.
