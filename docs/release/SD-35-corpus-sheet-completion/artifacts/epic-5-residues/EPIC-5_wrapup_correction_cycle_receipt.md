# Cycle EPIC-5-WRAPUP-FIX — Epic 5 Residues / wrap-up correction cycle

- **Commit SHA:** `<COMMIT_SHA>` (cycle start `00e44eee02`)
- **Scope gate:** `SCOPE_GATE: EXEMPT (wrap-up correction cycle)` — `decisions.md §2`. A wrap-up
  fix cycle closes zero units by design; Epic 5's populations were emptied by Epic 3
  (`26bdfa8d5b`, `406003afc3`, `51f91bba11`) and `cycle_scope_gate` has returned
  `PASS_WHOLE_REMAINDER` with `remaining_non_done=0` corpus-wide since.
- **Files touched:**
  - `scripts/publish-site-dashboard.sh` — the `--check-pin` control
  - `scripts/tests/test_publish_site_dashboard.sh` — 5 new self-test cases
  - `scripts/verify.sh` — new `site-dashboard-pin` stage, registered in both stage sets
  - `scripts/verify-baselines.env` — four measured floors advanced
  - `site/dashboard/PF1e-dashboard.json`, `site/dashboard/PF1e-dashboard.json.last-good`,
    `site/status-data.json` — regenerated, the red stage's actual fix
  - `site/dashboard/inventory-pin.json` — new, the pin the control compares against
  - `docs/release/SD-35-corpus-sheet-completion/workflow-instruction.md` — §6 step 3 push gate
  - `docs/release/SD-35-corpus-sheet-completion/kanban.md`, `progress.md`, this receipt
  - `docs/retro/events/at-35-e5-wrapup-fix.jsonl` — new
- **Identifier audit result:** OK_NO_BUNDLE_TAGS — with two machine-generated echoes named. The
  two `grep` hits are both the string
  `tests/sd27_alternate_racial_trait_reachability.rs's …`, a **pre-existing test filename**
  appearing inside the regenerated `site/dashboard/PF1e-dashboard.json`, on **removed** lines.
  Not a bundle tag written into code by this cycle.
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** not a criterion — this is `workflow-instruction.md §10` step 0, the
  correction cycle for Epic 5's wrap-up gate, which came back RED with exactly one failing stage.
- **Receipt rows (mechanical):**
  `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=253`
- **PCGen residue:** `live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS`
  — identical at cycle start and cycle end. This cycle wrote no Rust and touched no live path.
- **Oracle parity:** N/A — no `Number` mapping added, no live path touched.
- **Movement, four buckets:** closure none / relabel none / reachability none /
  instrument-correction — the whole cycle. One stale generated artifact republished, one recurring
  incident key given its first mechanism, four measured floors advanced.
- **Refused tokens:** none — this cycle converted nothing.
- **Discoveries:** none unpredicted by `token-coverage.json` or the atlas.

## What was red, and what actually fixed it

The gate worker's report (`EPIC-5_wrapup_gate_report.md`, already folded to the branch in
`5a7d476542`) recorded **47 of 48 stages PASS, 1 FAIL** at `c3500e7984`. The failing stage was
`site-dashboard-check`: `site/dashboard/PF1e-dashboard.json is STALE`.

**Verified against the repo before fixing, per the dispatch.** Reproduced independently on the
shared checkout at HEAD `00e44eee02` — the same single line, exit 1. The fix is
`./scripts/publish-site-dashboard.sh`, because the red stage is not a stale *pin on a live
figure*: it is a **generated artifact that had genuinely diverged from its input**. Regenerating
it is fixing the thing, not copying a number into an assertion. Nothing was silenced, skipped,
ignore-listed, or lowered.

## The control (AGENTS.md rule 8)

`site-dashboard-json-stale-after-inventory-move` fired **3 times** across the SD-35 epic wrap-ups
and accounts for **7 failing runs** of the `site-dashboard-check` stage. Every time the
disposition was "regenerate it in the wrap-up correction cycle" — a chore, which rule 8 says is
not a control. This cycle built the mechanism.

**Why it kept happening:** `site/dashboard/PF1e-dashboard.json` is rendered from
`docs/work-inventory.json`. A cycle regenerates the inventory (§6 step 3 mandates it whenever
corpus records change) and does not republish the feed. `--check` catches that correctly, but it
runs the real ~4,000-line producer and costs about **15 minutes** — so it only ever sat in the
~90-minute epic wrap-up gate, long after the offending cycle had pushed. The gap was never
detection logic; it was **detection latency**.

**What was built:**

1. A real publish now records `sha256(docs/work-inventory.json)` into
   `site/dashboard/inventory-pin.json` **in the same run that renders the feed**, so nothing
   writes one without the other.
2. `./scripts/publish-site-dashboard.sh --check-pin` re-hashes that one file and compares.
   Milliseconds — no producer, no corpus, no cargo.
3. Wired as `verify.sh` stage **`site-dashboard-pin`** in both `ALL_STAGES` and `QUICK_STAGES`
   (48 → 49 stages), and as a **push-blocking line in `workflow-instruction.md` §6 step 3** —
   the per-cycle gate, which is where the divergence is actually created.
4. The full `--check` now runs the pin first and **fails fast**, so the wrap-up gate stops paying
   15 minutes to learn what one hash already knew.

**What this proof does not cover (AGENTS.md rule 7).** The pin watches **one** input. A feed made
stale by a unit-ledger edit or an owner-state manifest change hashes clean under `--check-pin` and
is caught only by the full `--check`. The pin narrows the window on the recorded cause; it does
not replace the full check, and **both stages stay in `verify.sh`** for exactly that reason. An
absent pin file is a **failure**, never a silent pass — that always-green shape is self-test
case 13.

**RED→GREEN preserved.** The 5 new cases in `scripts/tests/test_publish_site_dashboard.sh` were
written first and failed for the intended reason (`--check-pin` fell through to the real-run
branch), then passed against the implementation: **13 of 13**, up from 8.

## Baselines advanced

**Four** floors were behind the tree, not the three the gate worker's report named — it omitted
`BASELINE_ROOT_TEST_BINARIES` (412 recorded, 413 measured), and three of the figures it did quote
had already moved because HEAD advanced six commits past `c3500e7984` while Epic 6 landed. Copying
its three numbers would have re-armed the same trap on a fourth. Recorded as correction
`1789024087133-at-35-e5-wrapup-fix-d6d8f0`. The 413th binary is attributed exhaustively to
`src/bin/gen_record_vars.rs`, the only `tests/*.rs` or `src/bin/*.rs` file **added** between
`c3500e7984` and `00e44eee02` (`ac38c5bf3c`). Measured this run: `BASELINE_ROOT_LIB_TESTS`
3223 → 3261, `BASELINE_ROOT_FULL_TESTS` 8734 → 8772, `BASELINE_ROOT_TEST_BINARIES` 412 → 413,
`BASELINE_DESKTOP_TESTS` 574 → 576. They are **floors** (`check_floor`, `verify.sh:1617/1691/1724`
— a run fails only when measured is *below* them), so advancing them to a value **this cycle's own
green run measured** cannot mask a regression: the same run that measured X is the evidence that
the floor X holds. They are re-pinned on this run's measurement, not copied from the gate worker's
earlier one, because HEAD moved past `c3500e7984` while Epic 6 landed.

## Two infrastructure defects the gate worker logged — disposition

Both are recorded incidents with named controls, and **neither is built here**: this cycle's brief
names one control (the site-dashboard key), and `AGENTS.md` rule 3 forbids the detour. Each fired
**once**, so rule 8's recurrence trigger is not met. Named so the orchestrator can schedule them:

- **`wrong-base-worktree`** (event `1788995389793-at-35-e5-wrapup-156d6c`) — named control: a
  `preflight-base` stage running `git rev-list --count HEAD..@{upstream}`, nonzero = fail.
- **`cross-worktree-codemod-contamination`** (event `1788996810915-at-35-e5-wrapup-e2468e`) —
  agent worktrees live *under* the shared checkout at `.claude/worktrees/`, so any codemod
  anchored at the repo root walks into every other agent's tree. Named control: create worktrees
  outside the repo root, or forbid `.`-anchored codemods in §6.

## Figures + their re-derive commands

| Figure | Value | Re-derive command |
| --- | --- | --- |
| Failing stages at the gate worker's run, of 48 | 1 | `grep -n 'stages passed' docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/EPIC-5_wrapup_gate_report.md` |
| `--check` wall time, one run, this checkout | 904 s | `time ./scripts/publish-site-dashboard.sh --check` |
| Self-test cases, of which passing | 13 of 13 | `bash scripts/tests/test_publish_site_dashboard.sh` |
| Self-test cases before this cycle, of 13 | 8 | `git show 00e44eee02:scripts/tests/test_publish_site_dashboard.sh \| grep -c '^# --- [0-9]'` |
| `verify.sh` stages, all sets | 49 | `bash -c 'source <(grep "^ALL_STAGES=" scripts/verify.sh); echo ${#ALL_STAGES[@]}'` |
| PCGen live files, of the 260-file baseline | 253 | `python3 scripts/pcgen_residue_gate.py --check` |
| Units closed by this cycle, of 0 scoped | 0 | `python3 scripts/cycle_scope_gate.py --receipt --since 00e44eee02 --before /tmp/wi-before-e5fix.json --after docs/work-inventory.json` |
| Rust lines changed by this cycle | 0 | `git diff --stat 00e44eee02 -- '*.rs'` |
| Firings of `site-dashboard-json-stale-after-inventory-move` before this cycle, as `incident` events | 3 | `python3 -c "import json,glob;print(sum(1 for f in glob.glob('docs/retro/events/*.jsonl') for l in open(f) if l.strip() and json.loads(l).get('type')=='incident' and 'site-dashboard-json-stale-after-inventory' in l))"` |
| `docs/work-inventory.json` sha256 the feed is pinned to | `5a0a0787312b5181d41214cb52abcd6e0c250fc409a75675ed6e839b4142e36f` | `sha256sum docs/work-inventory.json` |

- **Build scope verified:** full `scripts/verify.sh -j 4`, all **49** stages, run ONCE at `00e44eee02` plus this cycle's working tree. **`RESULT: PASS` — 49 of 49 green, 0 red**, 3,916 s = 65 min 16 s; per-stage logs `/tmp/codex-verify-00NKWe`. `root-lib` 3261 passed; `root-full` 8772 passed across 413 suites, **all 361 `tests/*.rs` suites executed**, 0 failed; `desktop` 576; `reach` green; `clippy` root:0 desktop:0; `corpus-sweep` 0 findings; `sheet-rules-check` `records=49438 converted=49296 refused=142`; `reachability-audit` 100.00% (49,438 of 49,438); `token-coverage` `non_done=0 refused_non_done=0`; `class-dump` 31/31 computing; `frontend-test` 101/101. The two stages this cycle is about: **`site-dashboard-check` PASS** ("is current" — it was the sole red stage) and the new **`site-dashboard-pin` PASS**. `figure-provenance` PASS `files_checked=188 figures_examined=334 violations=0`, which covered this receipt and the progress entry.
- **Sweep population:** N/A — no corpus record changed.
- **Oracle pin:** N/A — no figure came from the pinned corpus.
- **Status:** complete
- **Notes:** The full `scripts/verify.sh` was started once, killed two minutes in, and restarted —
  deliberately. The first start preceded this receipt, and `figure-provenance` and
  `denominator-gate` scan receipts; a gate that runs before the document it is meant to check is
  the exact shape that let three earlier wrap-ups go red on `figure-provenance`. The certified run
  below is the one that saw the finished tree.
- **Next-cycle scope:** Epic 5 closed. Epic 5's own deferral
  (`1788994100821-at-35-e5-005-5973cb`, the 10 DESC-without-prose units) stays correctly gated,
  not excused — `AT-35-E5-005_desc_without_prose.py --check` exits 1 until it reaches 0, and
  Epic 6 owns it converter-side.
