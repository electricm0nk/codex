# Cycle AT-35-E3-003_cycle1 — Epic 3 — Place and surface / AT-35-E3-003

- **Commit SHA:** `0e0298d7fe` (the control test and its comment in
  `src/bin/v06_work_inventory.rs` — the only code this cycle commits) and `<DOCS_SHA>` (this
  receipt, `rate-ledger.json`, `progress.md`, `kanban.md`, `docs/retro/events/at-35-e3-003.jsonl`,
  and the atlas artifact's re-derived `derived_at` stamp). Cycle start
  `7216215725f095ec8ee93406ba873d96db1ea054` on `tranche/15`.
- **Scope gate:** `scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER` — the
  literal last line of `python3 scripts/cycle_scope_gate.py --min 500 --bucket C` at
  `7216215725` (`scope=bucket=C`, `scoped_by_bucket=` empty, `scoped_by_kind=` empty). **The
  criterion's own population is already zero at cycle start**, and so is the whole remainder, so
  the mandated bundling ladder (this cycle's dispatch, orchestrator re-scope 2026-09-08) had
  nothing to bundle: there is no other bucket to add, and taking the whole remainder — which the
  gate did — is the verdict it returned. `python3 scripts/pcgen_residue_gate.py --check` at
  start: `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`.
- **Files touched:** `src/bin/v06_work_inventory.rs` (one `#[cfg(test)]` control test,
  `size_is_deliberately_absent_the_sheet_rule_superseded_register_c1_8`, plus a one-line pointer
  comment under `CLASS_FEATURE_ID_MAGNITUDE_SUFFIXES`; **the suffix list itself is byte-identical
  to the committed one** — `git diff --stat src/bin/v06_work_inventory.rs` → `37 insertions(+)`,
  **0 deletions**, so `docs/work-inventory.json` is unchanged and was not regenerated);
  `docs/retro/events/at-35-e3-003.jsonl` (2 corrections, the second correcting the first);
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (one field,
  `derived_at` `5a361c9dc4…` → `7216215725…`, re-derived by this cycle's own
  `completion_atlas.py --check`; kept rather than reverted, because reverting leaves the gate
  stale — the same disposition AT-35-E3-002's receipt recorded); this receipt, `rate-ledger.json`,
  `progress.md`, `kanban.md`.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS —
  `BASE_BRANCH=fe5ae6cd4a5f3c65d5d10f4d523f00e33b04ac47` (`git merge-base HEAD origin/develop`);
  `git diff --unified=0 "${BASE_BRANCH}...HEAD" -- src/pcgen_import/sheet_rule/ src/rules_core/class_feature_pool_catalog.rs src/bin/v06_work_inventory.rs data/sheet_rules/ docs/work-inventory.json docs/release/SD-35-corpus-sheet-completion/artifacts/epic-3-place-and-surface/ ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  → no match, at start and on the final diff. (This cycle's own added lines name `SD-34` and
  `C1.8` in prose, neither of which is in the pattern class, and carry no bundle-tagged
  identifier.)
- **Wired-integration audit result:** OK_NO_TOKENS on this cycle's own diff. Over the whole Epic 3
  file-touch set since `fe5ae6cd4a` the same grep
  (`\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b`) returns only the
  pre-existing non-code hits AT-35-E3-001's and AT-35-E3-002's receipts already name and account
  for: 3 rulebook-prose strings inside `data/sheet_rules/**` `ProsePiece::Text` (Tophet "hack or
  smash", Plant Growth "hack or force", Courtly Companion "not yet implemented" — corpus text,
  recorded by AT-35-E2-002's correction `1788844812035-at-35-e2-002-7cbeb2`), the
  `engine_diagnostic:vacuous_placeholder_row_no_corpus_content_to_render` evidence strings in
  `docs/work-inventory.json` (3 removed `-` lines), and the prior receipts' own quotations of the
  audit pattern — **11 matches** over the set before this receipt is committed, and this receipt's
  own quotation of the pattern (this sentence) raises that count again, exactly as every prior
  Epic 3 receipt recorded of itself. **Restricted to this cycle's own diff**
  (`git diff --unified=0 7216215725..HEAD -- <the Epic 3 set>`) the same grep returns
  **OK_NO_TOKENS**. **No stub, inline mock or `"Would …"` string in shipping code**, and this
  cycle adds none — its only code is a `#[cfg(test)]` assertion.
- **Acceptance criterion:** verbatim from `epic-breakdown.md` `### AT-35-E3-003`: "**AT-35-E3-003
  — bucket C reaches zero.** 4,180 units at authoring, all `class_feature`, evidence
  `no_explanation_id_and_no_diagnostic_names_this_feature`. Under the sheet rule a held rule with
  a line is `sheet-complete` regardless of explanation id; the classifier's C rung is replaced.
  Any C unit that does not convert is refused by token type and moves to Epic 4's ledger.
  **Evidence:** `completion_atlas.py --check` reports C at 0; the refused report for any residue."
- **Receipt rows (mechanical):** `closed=0 relabeled=0 rust_lines_changed=37 ratio=n/a builds_recorded=1 pcgen_live_files=260`
  (`python3 scripts/cycle_scope_gate.py --receipt --since 7216215725f095ec8ee93406ba873d96db1ea054 --before /tmp/wi-before-AT-35-E3-003.json --after docs/work-inventory.json`,
  `target_dir=/tmp/cargo-sd35-AT-35-E3-003 residue_gate=present`, with
  `closed_by_kind=` and `relabeled_moves=` both empty and `regressed=0 added=0 dropped=0`).
  `ratio` is `n/a`, not `0.0`: zero units closed is a division by zero
  (`rate-ledger.json`'s own `reading_rule`). **This cycle closes zero units by design** — its
  criterion's population was already zero when it started; it is a verification cycle, not a
  no-op, and the verification is the deliverable.
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`
  — identical at start and end, and identical to AT-35-E3-002's receipt. Not raised.
- **Oracle parity:** N/A — this cycle added no `Number` mapping and touched no live path. The
  standing figure is AT-35-E2-005's `compared=12 agree=1 disagree=11 unverifiable=42` at
  `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`; AT-35-E4-002 owns the corpus-wide
  run.
- **Movement, four buckets:**
  - **closure (into DONE, by id-set): 0.** Bucket C was already at 0 at cycle start.
  - **relabel (bucket to bucket): 0.**
  - **reachability: 0.**
  - **instrument-correction: 0 units, 1 instrument.** No unit's classification changed. The
    classifier gained a control test that pins a deliberate exclusion (below); the suffix list it
    guards is unchanged.
- **Refused tokens:** **none.** `cargo run --locked --bin sheet_rule_convert -- --check` →
  `kind class_feature: records=18043 converted=18043 refused=0` — **every** record of the only
  kind bucket C ever contained converts, so the criterion's "any C unit that does not convert is
  refused by token type and moves to Epic 4's ledger" clause has an empty residue. Corpus-wide the
  same run reports `records=49438 converted=49296 refused=142 rules=69344 var_tables=5269
  verdict=PASS (116.7s)`; all 142 are a single token type, `no_corpus_record`
  (`data/sheet_rules/_refused.json` `by_token_type`), spread over `race` 27, `race_trait` 104 and
  `feat` 11, and **none of them is non-DONE** — `python3 scripts/token_coverage.py --check` →
  `refused_non_done=0`. **No residue belongs to this criterion.**
- **Discoveries:** **one, and it is a correction of an inherited instruction, not of the atlas.**
  SD-34 `forward-scope-register.md` C1.8 carried a named one-line census fix into SD-35 and
  `progress.md` (AT-35-E1-006's entry) assigned it to **this criterion**: add `"size"` to
  `CLASS_FEATURE_ID_MAGNITUDE_SUFFIXES` so the engine's real
  `class_chassis.monk.ki_pool_size` grounds `core_rulebook:class_feature:monk_ki_pool`, then
  stuck at `engine-does-not-hold`. This cycle **applied it, measured it, and reverted it.** The
  one-liner was authored against the pre-sheet-rule ladder, where grounding was the only road to
  DONE. Under `decisions.md §1` the unit is already DONE on a *stronger* rung — `sheet-complete` /
  `sheet_rule_rendered:words`. Adding the word makes the older suffix-strip rung win **first**, so
  the unit is measurably **demoted**: exactly **1 unit of 49,438** changes,
  `sheet-complete`/`sheet_rule_rendered:words` → `grounded`/
  `explanation_id_observed_after_known_magnitude_suffix_strip`. Both are DONE
  (`completion_atlas.py::_bucket_of` line 292), so **no bucket moves and bucket C stays 0 either
  way** — but one of the 32,617 `DONE_RUNG_STAMP_STATUSES` verification stamps is lost, and the
  regenerator's own stamp-loss guard refuses the write naming exactly that unit. **C1.8 is
  superseded by the sheet rule, not outstanding.** Both retro `correction` events are emitted,
  the second `--corrects` the first: `1788929025647-at-35-e3-003-247627` (the first reading) and
  `1788929587859-at-35-e3-003-be4117` (the measured reversal). The finding is kept out by a
  **control test**, not a comment (`AGENTS.md` rule 8):
  `class_feature_id_magnitude_suffix_strip_tests::size_is_deliberately_absent_the_sheet_rule_superseded_register_c1_8`
  asserts both that `"size"` is absent from the list and that `class_chassis.monk.ki_pool_size`
  does not ground `ki_pool`.
- **Figures + their re-derive commands:**
  - **bucket C = 0 of a 49,438-unit corpus** — `python3 scripts/completion_atlas.py --check` →
    `population=49438 buckets=10 unclassified=0 overlap=0`, `DONE: 49438`, `C: 0` (and A/B/D/M/V/U/X/Z
    all 0), `done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False
    citation_failures=0`, exit 0. **This is the criterion's evidence sentence, run at HEAD.**
  - **bucket C = 0 in every one of the 19 kinds, `class_feature` included** — `python3
    scripts/completion_atlas.py --by-kind` → the `class_feature` row reads
    `DONE=18043(100.0%)` of 18,043 `class_feature` units, and `C=0` of that same 18,043. Every
    one of the 19 kind rows reads `C=0` — 0 of that kind's own `n`, and 0 of the 49,438-unit
    corpus.
  - **the C evidence string is extinct in the live inventory** — `grep -c
    'no_explanation_id_and_no_diagnostic_names_this_feature' docs/work-inventory.json` → **0**.
    The rung itself is **kept** (`src/bin/v06_work_inventory.rs:16366`, `scripts/completion_atlas.py:177`,
    and its two ladder-position assertions at `:27542` and `:30750`): the criterion says the C
    rung is *replaced*, and a rung with no unit on it is the proof, not a rung deleted.
  - **C's 4,180 authoring population, accounted for in full: 4,101 + 79 = 4,180.** 4,180 is the
    launch-gate figure (`workflow-instruction.md §1` item 9, `completion_atlas.py --check` at the
    cut). **4,101** were closed by AT-35-E2-005's `sheet-complete` rung — the by-prior-bucket
    breakdown of its 21,911 stamps, `progress.md` "from B 11,152 / M 4,271 / **C 4,101** / D 1,939
    / A 448". The remaining **79** were closed by AT-35-E3-001 cycle 2 at `406003afc3` — its own
    receipt's `closure … By prior bucket: A 1, B 435, **C 79**, D 43, M 60`. Nothing in C was
    refused, relabelled sideways, or carved out.
  - **`class_feature` refused = 0 of 18,043** — `cargo run --locked --bin sheet_rule_convert -- --check`.
  - **no PCGen token syntax in the generated package** —
    `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → **0**.
  - **1 unit of 49,438 would change if C1.8's word were added** — `v06_work_inventory
    --stdout-only` with the word applied, diffed by id against the committed inventory (the run
    never touched `docs/work-inventory.json`; `git status --porcelain` confirmed it unmodified).
- **Build scope verified:** `cargo test --locked --no-run -j 6` → **NO_RUN_EXIT=0**, 0 `error`
  lines. `cargo test --locked --no-fail-fast -j 6` → **411 test binaries executed (+1 doc-test target = 412 `test result:` lines), 8,726 passed, 0 failed, 0 failing suites, FULL_EXIT=0**. Counted two independent ways that agree (`awk '/^     Running/{n++}'` = 411 and `grep -c 'Executable' ` on the `--no-run` log = 411; `awk '/^test result: ok/{s+=$4}'` = 8726). The launch baseline's **590** targets / 8,656 passed became **408** at AT-35-E1-003's test-family tax cut (its own receipt: "590 before \u2192 408 after"); the +3 since are Epic 2's and Epic 3's own gate binaries. **Not a count this cycle moved**, run at
  `7216215725` plus this cycle's test-only addition. `cargo test --locked --bin v06_work_inventory
  class_feature_id_magnitude_suffix_strip_tests` → **8 passed, 0 failed** (RED first: the
  now-reverted grounding assertion failed at `src/bin/v06_work_inventory.rs:27937` for the
  intended reason, then the control test replaced it and passes). Desktop crate and frontend:
  **epic cadence** — this cycle touched no `apps/` path. `cargo clippy`: **not run, and stated as
  such** — this cycle's only Rust is a `#[cfg(test)]` assertion inside an existing test module and
  a comment; no target's non-test code changed.
- **Sweep population:** `cargo run --locked --release --bin corpus_literal_sweep` → **48,706
  records examined of 51,476 read, 413,314 tokens compared (9 synthesized), 51,463 digests
  checked, 0 findings, CLEAN**; 3,138 tokens exempted under `decisions.md §24` across 1,058
  `codex_generated_name` records. Unmoved from AT-35-E3-002's figure — **no corpus record
  changed** in this cycle, and the sweep was run only as the stamp guard's prerequisite for the
  measurement described under Discoveries.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` — carried in every
  `data/sheet_rules/**` record's `provenance.oracle_pin`; no figure in this receipt was newly
  derived from the pinned corpus.
- **Status:** **complete.** Bucket C is 0 at HEAD by the criterion's own evidence command, the
  refused report for the residue is empty, and the one carried obligation the package assigned to
  this criterion is dispositioned with a measurement and a control test.
- **Notes:** (a) The criterion's population was emptied by AT-35-E3-001 cycle 2's bundled cycle
  (`406003afc3`) and `kanban.md` row 14 already pointed there; this cycle is the criterion's own
  verification at HEAD and its own receipt, which row 14 now also cites. **No card was emptied by
  this cycle** — it moved no units, so no other criterion's row changes. (b) The one real finding
  is C1.8's supersession, above: a carried one-liner was half-wrong by the time its named owner
  reached it, which is the `re-test-a-hazard-before-repeating-it` lesson arriving as a measurement
  rather than as a warning. (c) `builds_recorded=1`, on target (`decisions.md §3`).
- **Next-cycle scope:** **criterion at zero.** `python3 scripts/cycle_scope_gate.py --min 500
  --bucket C` → `scoped=0 remaining_non_done=0 verdict=PASS_WHOLE_REMAINDER`; the whole corpus
  remainder is 0, so Epic 3 has no successor cycle on any bucket. The open Epic 4/5 rows (17, 18,
  22, 23) are instrument- and artifact-shaped, not unit-shaped.
