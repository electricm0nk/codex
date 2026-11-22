# Cycle 1 — Epic 4, Resolve and verify / AT-35-E4-003

- **Commit SHA:** `ea9650ffc9` (cycle start `e7f66b1f80`; the litter fold `59346e8fd3` — the
  atlas `derived_at` re-stamp `completion_atlas.py --check` makes as a side effect, plus one
  live `sd31-transcribe.jsonl` append from another session on the shared checkout — and the
  docs-row commit that pins this line follow it)
- **Scope gate:**
  ```
  SCOPE_GATE: EXEMPT (ledger cycle — records this epic's per-cycle rows; closes zero units by design)
  ```
  `decisions.md §2` / `workflow-instruction.md §6` step 1's floor exemption. The exemption is
  claimed on the "closes zero units by design" clause, not on scope: this cycle writes one JSON
  artifact and three docs rows and moves no unit. For the record, the gate was run anyway and
  reports the same thing the two preceding Epic 4 cycles' gates did — there is nothing left to
  scope:
  ```
  inventory=docs/work-inventory.json
  scope=bucket=M
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
- **Files touched:**
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/rate-ledger.json` (new — the deliverable)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/AT-35-E4-003_cycle1_receipt.md` (this file)
  - `docs/release/SD-35-corpus-sheet-completion/progress.md` (prepended entry, status matrix)
  - `docs/release/SD-35-corpus-sheet-completion/kanban.md` (row 18)
  - `docs/retro/events/at-35-e4-003.jsonl` (the cycle's retro events)

  No Rust, no `data/`, no `scripts/`, no `apps/` path was written.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
  ```
  BASE_BRANCH=fe5ae6cd4a5f3c65d5d10f4d523f00e33b04ac47   # git merge-base HEAD origin/develop
  git diff --unified=0 "${BASE_BRANCH}...HEAD" -- src/pcgen_import/sheet_rule/ data/sheet_rules/ \
    scripts/oracle_harness/ docs/work-inventory.json \
    docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/ \
    ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
  ```
  → `OK_NO_BUNDLE_TAGS`, before and after this cycle's diff.
- **Wired-integration audit result:** **4 hits, all in this receipt's own prose, none in code or
  data.** Run over the scoped paths with this cycle's two new files `git add -N`'d so the diff
  actually contains them (an untracked file is invisible to `git diff`, which is how a first pass
  read a false `OK_NO_TOKENS`):
  ```
  git diff --unified=0 e7f66b1f80 -- <scoped paths> ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'
  ```
  → lines 48, 50, 52 and 61 of this file: the three phrases quoted below while dispositioning the
  epic-wide hits, plus the audit grep's own pattern string on line 61. Self-referential, exactly
  the shape `AT-35-E4-001_cycle1_receipt.md` and `AT-35-E4-002_cycle1_receipt.md` already carry.
  Nothing this cycle wrote is a stub, an inline mock, or a `"Would …"` string; it wrote no code
  at all.

  The epic-wide grep over the same scoped paths (`fe5ae6cd4a...HEAD`) returns the 9 pre-existing
  hits both Epic 4 receipts already dispositioned, and this cycle added none of them. Re-derived
  at HEAD, they are, by class:
  - **6 hits are Pathfinder rules prose carried verbatim into `data/sheet_rules/`** — *"creatures
    must hack or force a way through"* (`core_rulebook:spell:plant_growth`), *"attempt to hack or
    smash its way out"* (`bestiary_3:monster_ability:tophet_swallow_whole`), the bracketed
    converter note *"[Change to magical beast and stacking restriction not yet implemented]"*
    (`ultimate_intrigue:class_feature:courtly_hunter_courtly_companion`), and PCGen's own
    CHOOSE-menu *"no selection" placeholder* rows removed from `docs/work-inventory.json` by an
    earlier cycle (three `-` lines).
  - **3 hits are the audit grep's own pattern string quoted inside prior receipts**
    (`AT-35-E4-001_cycle1_receipt.md`, `AT-35-E4-002_cycle1_receipt.md`).

  None is a stub, an inline mock, or a `"Would …"` string in shipping code.
- **Acceptance criterion** (verbatim, `epic-breakdown.md`):
  > ### AT-35-E4-003 — the rate ledger
  >
  > `artifacts/epic-4-resolve-and-verify/rate-ledger.json`, same shape as E3-004.

  and, by that reference, E3-004's shape verbatim:
  > `artifacts/epic-3-place-and-surface/rate-ledger.json`: per cycle — mechanism, scoped
  > population, units closed, units relabeled, wall time, `rust_lines_changed / units_closed`,
  > `builds_recorded` (must be 1), `pcgen_live_files` (must not rise).
- **Receipt rows (mechanical):**
  ```
  since=e7f66b1f80029b3d8eb0c8892d614943fe5491cd residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=260
  ```
  `closed=0` / `relabeled=0` is correct and expected: the unit population was **already** 0
  non-DONE when the cycle started, and this cycle is a docs-only ledger cycle. `ratio` is `n/a`,
  a division by zero, never `0.0`. `rust_lines_changed=0` and `builds_recorded=0` because no
  Rust file was written and so no cargo build was owed (`workflow-instruction.md §6` step 3
  ties the build to a figure-moving change; the `AT-35-E3-004_cycle1` precedent).
- **PCGen residue:**
  ```
  live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Identical at start and at end, and identical to `AT-35-E4-002_cycle1_receipt.md`'s. Nothing
  live-side was touched at all.
- **Oracle parity:** N/A — this cycle added no `Number` mapping and touched no live path. The
  epic's parity figures are transcribed into the ledger from the two cycles that produced them:
  `AT-35-E4-001_cycle1` `compared=146 agree=145 disagree=1` (lines) / `compared=382 agree=376
  disagree=6` (chassis), and `AT-35-E4-002_cycle1` `compared=392 oracle_agree=184
  oracle_disagreement=10 oracle_unverifiable=198`, both at
  `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.
- **Movement, four buckets:**
  - **closure (into DONE):** 0 — the population was already 0 non-DONE at the cycle start
    (`completion_atlas.py --check` → `DONE: 49438` of 49,438, every other bucket 0).
  - **relabel (bucket to bucket):** 0.
  - **reachability:** unchanged — `degraded_records` and `_refused.json` were not written.
  - **instrument-correction:** one, and it is in the **audit instrument**, not the corpus or the
    ledger. Both preceding ledger rows transcribed with 0 discrepancies against their committed
    receipts, so no ledger `correction` was owed; the recorded correction
    (`1788959112531-at-35-e4-003-d78240`) is that `§6` step 2's audit greps read an empty diff
    for a cycle whose whole output is new files — see **Discoveries**.
- **Refused tokens:** **none.** This cycle added no converter mapping row, no refusal and no
  `_refused.json` entry; `token_coverage.py --check` reports `refused=142 refused_non_done=0
  token_types=231`, identical to both preceding Epic 4 receipts.
- **Discoveries:** one, and it is about the **audit instrument**, not the corpus. The ledger's
  rows are exactly the two committed receipts' rows plus this cycle's own; no token type, kind,
  or remaining-step category surfaced that `token-coverage.json` or the atlas had not already
  recorded. But `workflow-instruction.md §6` step 2's two audit greps run over
  `git diff <base>...HEAD`, and **a cycle whose entire output is new files sees an empty diff** —
  an untracked file is invisible to `git diff` until it is added. A cycle that runs the audit
  before committing, as step 2 and step 4 both direct, therefore reads `OK_NO_TOKENS` for a diff
  that contains nothing at all. Caught here by re-running with `git add -N` on this cycle's two
  new files, which turned a false `OK_NO_TOKENS` into the 4 real (self-referential) hits recorded
  above. Emitted as a `correction` retro event (`1788959112531-at-35-e4-003-d78240`,
  `docs/retro/events/at-35-e4-003.jsonl`); the
  mechanism a later cycle should build is `git add -N` in step 2's snippet, not a caution
  (`AGENTS.md` rule 8).
- **Figures + their re-derive commands.** The scoped denominator, its Epic-4 start pin, and the
  epic's authoring-time population — all previously stated inline in this sentence without a
  re-derive command — are now the first three rows of the table, each with the command that
  produces it:

  | figure | value | command |
  |---|---|---|
  | scoped denominator at Epic 4's first cycle start (`07e29075b4`) | **0 units non-DONE of 49,438** | `python3 -c "import json;print(json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/rate-ledger.json'))['totals']['denominator'])"` |
  | total units in the corpus at that same pin `07e29075b4` | **49,438** | `git show 07e29075b4:docs/work-inventory.json > /tmp/wi-e4start.json && python3 -c "import json;print(json.load(open('/tmp/wi-e4start.json'))['totals']['units'])"` |
  | Epic 4's authoring-time population (M 4,334 + V 392), closed by Epic 3 before Epic 4 began | **4,726** | `python3 -c "import json,re;d=json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/rate-ledger.json'))['totals']['denominator'];print(re.search(r'4,334 . V 392 = ([0-9,]+)',d).group(1))"` |
  | Epic 4 cycle receipts on disk before this cycle | **2** | `ls docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/*_receipt.md \| wc -l` |
  | ledger rows written | **3** (2 transcribed + this cycle's own) | `python3 -c "import json;print(len(json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/rate-ledger.json'))['cycles']))"` |
  | transcription discrepancies | **0** | `grep -hE 'closed=[0-9]+ relabeled' docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/*_receipt.md` compared against the ledger's `units_closed` / `units_relabeled` / `rust_lines_changed` / `builds_recorded` / `pcgen_live_files` fields |
  | epic totals — cycles / closed / relabeled / rust lines / builds | **3 / 0 / 0 / 302 / 1** | `python3 -c "import json;c=json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/rate-ledger.json'))['cycles'];print(len(c),sum(x['units_closed'] for x in c),sum(x['units_relabeled'] for x in c),sum(x['rust_lines_changed'] for x in c),sum(x['builds_recorded'] for x in c))"` → `3 0 0 302 1` |
  | `ratio_over_the_epic` | **null** (0 units closed — a division by zero, never `0.0`) | same command; `units_closed` sums to 0 |
  | corpus at HEAD | **DONE 49,438 of 49,438**, every other bucket 0 | `python3 scripts/completion_atlas.py --check` |
  | PCGen live-side files | **260** (baseline 260), start and end | `python3 scripts/pcgen_residue_gate.py --check` |
  | `data/sheet_rules/` token leaks | **0** | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | package docs scanned by the denominator gate | **56 files, 0 violations** | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` |
- **Build scope verified**, run at cycle start `e7f66b1f80` (the tree this cycle leaves
  byte-identical outside `docs/`):
  - `cargo test --locked --no-run`, `--lib`, `--no-fail-fast`, `cargo clippy`,
    `sheet_rule_convert -- --check` and `corpus_literal_sweep` — **not run, and not owed**: this
    cycle wrote no Rust, no `data/`, no `scripts/` and no corpus record, so no cargo target and
    no generated artifact changed (`git diff --stat e7f66b1f80..HEAD -- src/ scripts/ data/ apps/`
    is empty). `workflow-instruction.md §6` step 3 ties the build to a figure-moving change; the
    `AT-35-E3-004_cycle1` precedent is the same shape. The last full-suite result standing at
    this sha is `AT-35-E4-002_cycle1`'s: **8,727 passed, 0 failed, 67 ignored over 412 targets**
    at `2645a3c85a`.
  - `python3 scripts/pcgen_residue_gate.py --check` → `verdict=PASS`, 260 files (baseline 260)
  - `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → `0`
  - `python3 scripts/completion_atlas.py --check` → `DONE: 49438`, every other bucket 0,
    `done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False
    citation_failures=0`, exit 0
  - `python3 scripts/token_coverage.py --check` → `non_done=0 tokened=0 token_less=0 refused=142
    refused_non_done=0 token_types=231 shapes=1 verdict=PASS`, exit 0
  - `python3 scripts/shape_engine_boundary.py --check` → `magnitude_bearing=26396
    not_held_by_engine=0 citation_ok=True`, exit 0
  - `python3 scripts/missing_engine_tables.py --check` → `population=0 kinds=0
    citation_failures=0`, exit 0
  - `python3 scripts/denominator_gate.py --check …` → `files_checked=56 violations=0`, exit 0
  - `scripts/verify.sh --only pi-sweep` → `RESULT: PASS` (`passed: 1  pi-sweep`)
  - the desktop crate and the frontend did **not** run: this cycle touched no path under
    `apps/`. They run at the Epic 4 wrap-up (`workflow-instruction.md §10`).
- **Sweep population:** N/A — no corpus record was written this cycle.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`, unchanged). No figure in this receipt was newly derived from
  the pinned corpus; the two parity figures the ledger carries were derived from it by the
  cycles that own them.
- **Status:** complete
- **Notes:** Epic 4's rate ledger is a ledger of an epic whose unit population was already zero
  when it began. That is the honest reading and the ledger says so in `totals.denominator`
  rather than presenting `units_closed=0` as a shortfall: Epic 3's `AT-35-E3-002_cycle1` took
  the whole remainder (786 units) to DONE, so Epic 4's two cycles paid the *evidence* clauses of
  their criteria — the mapping table's unmapped-head set to zero, and bucket V's 392 units
  through the oracle harness — without a bucket to move. `ratio_over_the_epic` is `null` for the
  same reason, never `0.0`. The Epic 4 wrap-up gate (`§10`) has not run at this sha and is not a
  `cycles` row; the Epic 3 precedent is identical.
- **Next-cycle scope:** criterion at zero. Epic 4's three criteria are `complete`; the next step
  is the Epic 4 wrap-up gate (`workflow-instruction.md §10`), then Epic 5.
