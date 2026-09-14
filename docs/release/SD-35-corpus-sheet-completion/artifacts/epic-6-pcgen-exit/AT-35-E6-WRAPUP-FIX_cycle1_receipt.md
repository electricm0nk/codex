# Cycle AT-35-E6-WRAPUP-FIX cycle 1 — Epic 6, PCGen exit / wrap-up correction cycle

Runs `workflow-instruction.md §10 step 0`: the Epic 6 wrap-up gate came back RED on 4 of 49
stages, and every red must be fixed before Epic 7's **second** dispatch. This cycle runs LOCAL on
the shared checkout and **does** commit and push, unlike the isolated gate worker, which pushed
nothing.

- **Commit SHA:** `<filled at commit>`
- **Cycle start SHA:** `77e8d3919a`
- **Scope gate:** `SCOPE_GATE: EXEMPT (wrap-up correction cycle)` — `decisions.md §2` /
  `workflow-instruction.md §6 step 1`: a wrap-up fix cycle closes zero units by design. **Not**
  exempt from the residue check, which ran at start and at end (below).
- **Files touched:**
  - `apps/desktop/src-tauri/src/equipment_catalog.rs` (clippy fix)
  - `scripts/verify-baselines.env` (`BASELINE_DESKTOP_TESTS` 576 → 570, with attribution)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-004_cycle1_receipt.md` (2 provenance rows)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-004_cycle1_b15_skipline_delta.py` (**new** — the re-derive command those 2 rows now cite)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/EPIC-6_wrapup_gate_report.md` (**new** — the gate worker's report, folded, with one correction block)
  - `site/dashboard/PF1e-dashboard.json`, `site/dashboard/PF1e-dashboard.json.last-good`, `site/dashboard/inventory-pin.json`, `site/status-data.json` (+ `site/status-data/*.json`) — republished feed
  - 5 folded gate reports + 11 retro event logs recovered from worktrees (listed under **Worktree sweep**)
  - `docs/retro/events/at-35-e6-wrapup-fix.jsonl` (**new** — this cycle's 5 events)
  - `docs/release/SD-35-corpus-sheet-completion/kanban.md`, `progress.md`
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — on **this cycle's own diff**
  (`git diff --unified=0 77e8d3919a -- <touched paths> ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`, no output).
  Note: the same grep over `${BASE_BRANCH}...HEAD` prints 3 hits, all pre-existing
  `tests/sd18_*`/`tests/sd13_*` filenames inside `verify-baselines.env`'s historical log, none of
  them this cycle's.
- **Wired-integration audit result:** `OK_NO_TOKENS` in code. The same grep reports 2 `placeholder`
  hits, both inside the **regenerated** `site/dashboard/PF1e-dashboard.json` at
  `/retrospective/corrections/by_subject -> "this cycle's own first no-placeholder gate": 1` — a
  retro `--subject` string the producer echoes into the feed's retrospective block. It is derived
  data, not shipping code, and it is new in the feed only because the feed was republished.
- **Acceptance criterion:** not a criterion cycle. `workflow-instruction.md §10 step 0`: *"Any red
  stage is fixed in a wrap-up correction cycle before the next epic's second cycle dispatches; the
  fix cycle is exempt from the batch floor, never from the residue check."*
- **Receipt rows (mechanical):**
  `closed=0 relabeled=0 rust_lines_changed=201 ratio=n/a builds_recorded=0 pcgen_live_files=0`
  (`python3 scripts/cycle_scope_gate.py --receipt --since 77e8d3919a --before /tmp/wi-before-AT-35-E6-WRAPUP-FIX.json --after docs/work-inventory.json`).
  `ratio=n/a` because `closed=0` by design. The 201 Rust lines are the single
  `vec_init_then_push` rewrite (21 `push` calls → one `vec![]` literal, plus the comment reindent
  the literal forced); no behaviour changed and no test changed.
- **PCGen residue:**
  - at start: `live_files=0 live_hits=0 baseline_files=260 baseline_hits=12736 verdict=PASS`
  - at end: `live_files=0 live_hits=0 baseline_files=260 baseline_hits=12736 verdict=PASS`
  - **unchanged — did not rise.** (`python3 scripts/pcgen_residue_gate.py --check`)
- **Oracle parity:** N/A — this cycle added no `Number` mapping and touched no live rules path.
  The one Rust edit is a test-module `Vec` literal in the desktop crate.
- **Movement, four buckets:** closure **0** / relabel **0** / reachability **0** /
  instrument-correction **4** (one per red stage). Nothing about the corpus moved; this cycle
  repaired instruments and package prose.
- **Refused tokens:** none — this cycle converted no records.
- **Discoveries:** one, and it is a process discovery rather than a mechanism one — see
  **Worktree sweep** below: the §10 step 2 sweep is a *destructive* step with no fold-first
  control, and 6 of the 8 Epic 6 worktrees were holding gate reports and retro logs that had never
  been folded. Emitted as incident `1789348192547-at-35-e6-wrapup-fix-55cb8e`
  (key `unfolded-gate-artifacts-die-with-the-worktree`). No new refused token type, kind, or
  remaining-step category surfaced.

## The four red stages, each fixed at its source

**1. `site-dashboard-check` — STALE FEED.** *Fixed by regenerating, not by re-pinning a number.*
`./scripts/publish-site-dashboard.sh` ran the real producer for ~19 min and rewrote the feed.
The recorded input pin is **unchanged** (`5a0a0787312b5181d41214cb52abcd6e0c250fc409a75675ed6e839b4142e36f`
before and after) — which *confirms* the gate worker's diagnosis rather than contradicting it:
`site-dashboard-pin` passed while `site-dashboard-check` failed because the pin watches ONE input
(`docs/work-inventory.json`) and the feed also derives from the unit ledgers and owner-state. This
is the residue `§6 step 3` documents in writing when it added `--check-pin`. **Nothing was
re-pinned and no number was copied into an assertion**; the producer re-derived the feed from its
real inputs (`books=30 overall_pct=95.0 items=46074`, re-derived from the committed feed by the
command in the figures table below, not from the producer's console line).

**2. `figure-provenance` — 2 violations, both prose in a command column.** Both rows are in
`AT-35-E6-004_cycle1_receipt.md` lines 230–231. **The values were right; the commands were not
runnable.** Rather than delete the rows or soften the gate, this cycle wrote the command that
re-derives them — `AT-35-E6-004_cycle1_b15_skipline_delta.py`, which loads the **pre-B15** gate
out of git by SHA so the "before" figure does not depend on the working tree — **ran it**, and
confirmed it reproduces every figure the two rows claim, exactly. See the figures table.

**3. `desktop` — floor 576, measured 570, ZERO failures.** Six cases gone, not broken. Attributed
before the floor was touched, by a method independent of `verify.sh`'s own count: enumerate every
`#[test]`/`#[tokio::test]` fn at `6e4b1f7b4e` (the commit that set 576) and at HEAD and diff the
name sets. **576 and 570 — agreeing with the stage — and the net −6 decomposes into 37 retired
and 31 written.** Every one of the 37 guarded a function Epic 6 **deleted** (`sole_feat_grant_target`
and its nine cases, `translate_condition`, `simplify_formula`, `format_base_ego_price_bands`,
`alignment_name`, `safe_description`, `resolve_description`, `render_clean`,
`is_real_description_value`, `render_desc_token`, `serve_desc_*`, `spell_out_*`,
`is_internal_flag_chain`, `variable_name_is_flag_shaped`), and every population-shaped guard among
them was **replaced**, not dropped. The floor was then lowered to 570 **with the full attribution,
the re-derive commands, the replacement mapping and the eight deleting SHAs written into
`verify-baselines.env`'s own log** — which is the only form `§8` permits. It is a `check_floor`, so
570 cannot mask a later regression; it re-arms the trap at the tree that actually ships.

**4. `clippy` — desktop 1 against a ceiling of 0.** `clippy::vec_init_then_push` at
`equipment_catalog.rs:889`. `let mut pinned: Vec<(&str, usize)> = Vec::new();` + 21
`pinned.push(...)` calls → one `vec![...]` literal. **No ceiling was raised and no lint was
allowed.** Verified clean at the widest scope for the crate:
`cargo clippy --locked --tests -j 6` in `apps/desktop/src-tauri` → `Finished dev profile in 2m 20s`,
**zero warnings**.

## Worktree sweep (`§10 step 2`) — the step the gate worker could not run

The gate worker was worktree-isolated and its harness refused every git op against a sibling
worktree, so it could not run `git -C <sibling> status --porcelain` — the check that protects
uncommitted work — and correctly refused to remove anything blind. This cycle is not isolated and
ran that check on all 8.

**All 8 were dirty.** Comparing every listed path against the main checkout
(`SAME-AS-REPO` / `DIFFERS` / `ABSENT-IN-REPO`) showed that **6 of the 8 held gate reports and
retro event logs that had never been folded into the repo**. Removing them as the sweep step says
would have destroyed all of it permanently and unrecoverably. Folded first, then removed:

| recovered | from |
|---|---|
| `artifacts/epic-4-resolve-and-verify/EPIC-4_regate_report.md` + `docs/retro/events/at-35-e4-regate.jsonl` | wf-30 |
| `artifacts/epic-5-residues/EPIC-5_wrapup_regate_report.md` + `at-35-e5-regate.jsonl` + `epic5-regate.jsonl` | wf-41 |
| `artifacts/epic-5-residues/EPIC-5_wrapup_regate_GREEN_report.md` + `at-35-e5-wrapup-regate.jsonl` + `epic5-wrapup.jsonl` | wf-33 |
| `artifacts/epic-2-sheet-rule/EPIC-2_wrapup_regate3_report.md` + `at-35-e2-regate3.jsonl` + `epic2-wrapup-regate2.jsonl` | wf-19 |
| `artifacts/epic-3-place-and-surface/EPIC-3_wrapup_gate_rerun_2026-09-10.md` + `at-35-e3-wrapup-rerun.jsonl` | wf-23 |
| `artifacts/epic-4-resolve-and-verify/EPIC-4_wrapup_regate_2026-09-10_report.md` + `epic4-wrapup-regate-0910.jsonl` | wf-27 |

Two different Epic 5 regate reports **collided on one path** (wf-41's "wrap-up **re-gate** report"
and wf-33's "re-gate, GREEN"); both were preserved, wf-33's under
`EPIC-5_wrapup_regate_GREEN_report.md`. Two `DIFFERS` append-only logs were **union-merged by event
id** rather than overwritten: `at-35-e5-wrapup.jsonl` (0 new) and `at-35-e3-wrapup.jsonl`
(**1 event recovered**). wf-17's copy of `EPIC-2_wrapup_regate2_report.md` was the **older** one
(the repo copy carries denominators it lacks), so it was correctly left alone.

**FOUND 8, REMOVED 6, RETAINED 2.** All 8 had **0 unmerged commits** and none was locked.

**RETAINED, needing an operator ruling — neither is this cycle's to fold or discard:**
- **`wf_291be5c8-5f3-30`** carries uncommitted **live Rust WIP** across 13 `src/` and
  `apps/desktop/` files, two of them `ABSENT-IN-REPO`:
  `src/rules_core/pilot_compute/bonus_stack_reader.rs` and
  `src/rules_core/pilot_compute/formula_interpreter.rs`. This is an Epic 4 regate worker's
  unfinished work of unknown provenance. Deleting it destroys code; folding it pushes unreviewed
  Rust onto `tranche/15`. **Escalated.**
- **`wf_291be5c8-5f3-19`** carries a modified
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` that differs
  **substantively** from the repo copy, not merely in `derived_at`. **Escalated.**

Recorded as note `1789348156572-at-35-e6-wrapup-fix-9de917`.

## Figures + their re-derive commands

| figure | value | command | denominator |
|---|---|---|---|
| PCGen residue, start and end | `live_files=0 live_hits=0 baseline_files=260 baseline_hits=12736 verdict=PASS`, identical both times | `python3 scripts/pcgen_residue_gate.py --check` | the 5 live roots, `.rs .ts .tsx .js .jsx .mjs .cjs`, comments and `#[cfg(test)]` excluded |
| figure-provenance violations | `violations=2` → `violations=0` | `python3 scripts/denominator_gate.py --check-provenance` | `figures_examined=606`, `files_checked=266` |
| unsourced-figure gate | `violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` | `files_checked=149` |
| desktop test names, at the commit that set the floor and at HEAD | **576** and **570** | the `git ls-tree` + `awk '/#\[(tokio::)?test\]/'` enumeration quoted verbatim in `scripts/verify-baselines.env`'s `BASELINE_DESKTOP_TESTS` block | every `.rs` under `apps/desktop/src-tauri/src` and `.../tests` at `6e4b1f7b4e` and at `HEAD` |
| retired vs written desktop tests | `removed=37 added=31` (net −6) | `comm -23 /tmp/T-6e4b1f7b4e.txt /tmp/T-HEAD.txt \| wc -l` and `comm -13 ... \| wc -l`, on the two files the enumeration above writes | the same 576 / 570 name sets |
| desktop clippy warnings | **1** → **0** | `cd apps/desktop/src-tauri && cargo clippy --locked --tests -j 6` | the desktop crate, `--tests` included |
| B15 gate skip, before and after | `b15_gate_skipped_lines_before=91053` → `b15_gate_skipped_lines_after=90433`; `independent_skipped_lines=90433` agrees | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-004_cycle1_b15_skipline_delta.py` | `files_with_cfg_test_item=200` |
| shipping lines the pre-B15 gate could hide | `transaction_rs_shipping_lines_hidden_before=618`; `total_shipping_lines_hidden_before=620` over `files_hiding_shipping_lines_before=3` | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-004_cycle1_b15_skipline_delta.py` | `transaction_rs_total_lines=2885` |
| feed input pin, before and after the republish | `5a0a0787312b5181d41214cb52abcd6e0c250fc409a75675ed6e839b4142e36f`, unchanged | `./scripts/publish-site-dashboard.sh --check-pin` | `docs/work-inventory.json`, the one input the pin watches |
| republished feed population | `books=30 overall_pct=95.0 items=46074` | `python3 -c "import json,glob; d=json.load(open('site/status-data.json')); print('books=%d overall_pct=%s items=%d' % (len(d['books']), d['overall']['pct'], sum(len(k['items']) for f in glob.glob('site/status-data/*.json') for k in json.load(open(f))['kinds'])))"` | `site/status-data.json` + the 30 book-detail files under `site/status-data/` |
| full gate, this cycle's own run | `RESULT: PASS` `VERIFY_EXIT=0`, **49 of 49 stages PASS, 0 FAIL**, 7,364 s | `./scripts/verify.sh` (every stage, no `--only`), then `grep -c '^==>' <log>` / `grep -cE '^    PASS' <log>` / `grep -cE '^    FAIL' <log>` | the 49 stages `verify.sh` defines |
| test-binary files Epic 6 added | **6**, and the `BASELINE_ROOT_TEST_BINARIES` move is 413 → **419** | `git diff --name-status 6e4b1f7b4e HEAD -- 'tests/*.rs' 'src/bin/*.rs' 'apps/desktop/src-tauri/src/bin/*.rs' \| grep -E '^[AD]'` | every `tests/*.rs` and `src/bin/*.rs` path, added or deleted, since 413 was recorded |
| worktrees found / removed / retained | **8 / 6 / 2** | `git worktree list` before and after; `git -C <each> status --porcelain` for the check | the 8 Epic 6 worktrees named in the gate report |
| disk reclaimed by the sweep | `.claude/worktrees/` 15G → **9.4G** | `du -sh .claude/worktrees/` | the shared checkout's worktree dir |
| this cycle's Rust lines | `rust_lines_changed=201` | `python3 scripts/cycle_scope_gate.py --receipt --since 77e8d3919a --before /tmp/wi-before-AT-35-E6-WRAPUP-FIX.json --after docs/work-inventory.json` | the whole cycle diff |

## Corrections this cycle emitted against the gate report

The gate report is testimony, and two of its claims did not survive re-derivation. Both are
recorded against the report, and the report itself carries an inline correction block at the first:

1. **The deleting commits were wrong.** The report named `4b77b31ee3, 1bbeb8ce2a, db1fe3a04c,
   1ebbe4b9bf`. All four are real Epic 6 PCGen-removal commits; **none deleted any of the 37
   retired tests.** Pickaxe attribution names eight others: `a4120e043f`, `77b10113a6`,
   `526173470e`, `5a3a67c2dd`, `81d8199a06`, `798bf8ebda`, `ef7d54daf1`, `592ea43501`.
   Correction `1789347711856-at-35-e6-wrapup-fix-f20460`.
2. **The `.gitignore` escalation named the wrong file and the wrong path.** The report asks the
   operator for a one-line `.worktrees/` entry in `.gitignore`. But `.worktrees/` **does not exist**
   at this repo root — the directory is `.claude/worktrees/` — and it **is** already excluded here,
   by `**/.claude/worktrees/` at **`.git/info/exclude:11`**, which is **local-only and never
   committed**. So the proposed line would have fixed nothing, and the real gap is different: the
   exclusion protects this one checkout and no other clone.
   Correction `1789348220016-at-35-e6-wrapup-fix-b8db63`. **Still an operator ruling** (`AGENTS.md`
   rule 4 — `.gitignore` is outside this cycle's granted write scope), but now for the right
   change: move `.claude/worktrees/` from `.git/info/exclude` into the **committed** `.gitignore`.

- **Build scope verified:** the **full `scripts/verify.sh`, every stage, no `--only`**, run by this
  cycle itself on the shared checkout at `952b313bbb` + the baseline edit below.
  **`RESULT: PASS`, `VERIFY_EXIT=0`, 49 of 49 stages PASS, 0 FAIL.** Wall time
  **7,364 s (2 h 02 m 44 s)**, 2026-09-14T01:26:01Z → 2026-09-14T03:28:45Z. Log:
  `<scratchpad>/verify-e6-fix.log`. **All four formerly-red stages are green:**
  `PASS site-dashboard-check (site/dashboard/PF1e-dashboard.json is current)`,
  `PASS figure-provenance (files_checked=274 figures_examined=613 violations=0)`,
  `PASS desktop (570 passed)`,
  `PASS clippy (root:0 desktop:0 warnings, 0 errors)`.
  Widest scope covered: `root-lib` 3390 passed; `root-full` 8919 passed across 419 suites with all
  365 `tests/*.rs` suites executed; the separate desktop crate 570 passed; `frontend-test`,
  `frontend-typecheck`, `reach`, `corpus-sweep` and `class-dump` all green.
  The run's own **BASELINE NOTES** block flagged three stale root floors — all **growth**, none a
  failure — and this cycle raised them **on its own measurement**:
  `BASELINE_ROOT_LIB_TESTS` 3261 → 3390, `BASELINE_ROOT_FULL_TESTS` 8772 → 8919,
  `BASELINE_ROOT_TEST_BINARIES` 413 → 419, the last attributed exhaustively to the six test-binary
  files Epic 6 added (`git diff --name-status 6e4b1f7b4e HEAD -- 'tests/*.rs' 'src/bin/*.rs' 'apps/desktop/src-tauri/src/bin/*.rs' | grep -E '^[AD]'`).
- **Sweep population:** N/A — no corpus record changed this cycle.
- **Oracle pin:** N/A — no figure came from the pinned corpus.
- **Status:** **complete.** Every red stage the Epic 6 wrap-up gate reported is fixed at its
  source and proven green by a full 49-of-49 gate run this cycle performed itself. Nothing was
  silenced, skipped, ignored or hidden; the one floor that moved down moved with its attribution.
- **Notes:** The batch-floor exemption is claimed under `decisions.md §2`; the residue check was
  run at both ends and did not move. No stage was silenced, no skip added, no ignore list widened,
  and the one baseline that moved moved **down** to the value a green run measures, with its
  attribution in the file rather than in a report.
- **Next-cycle scope:** Epic 6's four reds are closed. Two escalations remain for the operator
  (the two retained worktrees, and the `.gitignore` move); neither blocks Epic 7.
