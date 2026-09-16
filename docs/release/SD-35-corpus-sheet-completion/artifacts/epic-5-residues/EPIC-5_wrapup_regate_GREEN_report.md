# Epic 5 — Residues: wrap-up gate report (re-gate, GREEN)

`workflow-instruction.md §10` steps 0–3, run by the isolated read-only worker
(`§2` worker split, `decisions.md §3`). **This worker pushed nothing and committed nothing.**
The orchestrator hands this file to the next cycle agent to commit.

This is the **re-gate** of Epic 5. The first run (`EPIC-5_wrapup_gate_report.md`, worker
`AT-35-E5-WRAPUP`, HEAD `c3500e7984`) came back **RED** at 47 of 48 stages on
`site-dashboard-check`. The wrap-up correction cycle (`EPIC-5_wrapup_correction_cycle_receipt.md`,
`6e4b1f7b4e`) republished the feed and added the `site-dashboard-pin` control. This run
re-verifies at HEAD.

- **Worker:** `AT-35-E5-WRAPUP-REGATE` (`$RETRO_ACTOR` `epic5-wrapup` on the verify.sh-derived
  event), worktree `/home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_291be5c8-5f3-33`
- **HEAD gated:** `1e982aa94b4ecd2034d5c611b7433f1c32589f68`
  (`chore(sd35): fold the derived_at re-stamp the final atlas confirmation wrote`), re-derived
  with `git rev-parse HEAD` and equal to `origin/tranche/15` at launch
- **`CARGO_TARGET_DIR`:** `/tmp/cargo-sd35-epic5-wrapup`, `CARGO_INCREMENTAL=0`
- **Verdict:** **GREEN** — **49 of 49 stages PASS, 0 FAIL**. Status returned to the
  orchestrator: `complete`.

---

## 0. The full gate, once

    scripts/verify.sh          # every stage, no --only; mode: full, jobs: 2

- **Console log:** `/tmp/verify-epic5.log`
- **Per-stage log directory:** `/tmp/codex-verify-nKVmwT`
- **Wall time:** **5,221 s = 87 min 1 s** — verify.sh's own `duration_seconds` on the
  `verification` event it emitted at `2026-09-10T19:54:27Z`
  (`docs/retro/events/epic5-wrapup.jsonl`, id `1789070067091-epic5-wrapup-51f5d4`). The worker's
  own `date +%s` bracket read 5,227 s (`1789064840` → log mtime `1789070067`); the 6 s is this
  worker's own setup before `verify.sh` took its first timestamp. Comparable: the Epic 5 first
  gate ran 5,053 s, the Epic 4 wrap-up gate 5,432 s.
- **Result line:** `RESULT: PASS` / `logs in /tmp/codex-verify-nKVmwT`

### Stage table — 49 stages, 49 PASS / 0 FAIL

Re-derived with `grep -E '^    (PASS|FAIL)' /tmp/verify-epic5.log`.

| # | Stage | Result | Measured line |
|---|---|---|---|
| 1 | preflight-disk | PASS | disk budget OK (79% used, 310 G available of the 1.5 T root filesystem) |
| 2 | preflight-oracle | PASS | oracle at pin `7f818006e371188e5717fd18d74d18a420747fc6` |
| 3 | oracle-pin-selftest | PASS | 11 passed, 0 failed |
| 4 | producer-selftest | PASS | 30 cases passed |
| 5 | pi-redaction-selftest | PASS | 49 cases passed |
| 6 | provenance-selftest | PASS | 32 cases passed |
| 7 | site-dashboard-selftest | PASS | 13 passed, 0 failed |
| 8 | **site-dashboard-pin** | **PASS** | `docs/work-inventory.json` matches the pin the feed was published from — **the stage the correction cycle added; first full-gate run** |
| 9 | **site-dashboard-check** | **PASS** | `site/dashboard/PF1e-dashboard.json` is current — **the stage that was RED in the first Epic 5 gate** |
| 10 | site-dashboard-pi-gate | PASS | 22 files scanned against 1,612 declared-PI names, zero leaked |
| 11 | build-public-status-selftest | PASS | 37 cases passed |
| 12 | site-public-status-check | PASS | `site/status-data.json` and `site/status-data/*.json` are current |
| 13 | site-public-status-pi-gate | PASS | 31 files scanned against 1,612 declared-PI names, zero leaked |
| 14 | site-asset-stamp-check | PASS | `site/*.html` cache-busting stamps match `site/styles.css` |
| 15 | reachability-audit-selftest | PASS | 11 cases passed |
| 16 | reachability-audit | PASS | reachable ceiling 100.00% (49,438 of 49,438) |
| 17 | groundtruth-guard-selftest | PASS | 17 cases passed |
| 18 | supersession-gate-selftest | PASS | 16 cases passed |
| 19 | shape-coverage-standing-gate-selftest | PASS | 20 cases passed |
| 20 | shape-coverage-standing-gate | PASS | population=2485 unclassified=0 no_record=0 corpus_sha=`7f818006e3…` |
| 21 | cycle-scope-gate-selftest | PASS | 51 cases passed |
| 22 | shape-engine-boundary-selftest | PASS | 15 cases passed |
| 23 | shape-engine-boundary | PASS | magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True |
| 24 | missing-engine-tables | PASS | population=0 kinds=0 citation_failures=0 |
| 25 | denominator-gate | PASS | files_checked=272 violations=0 |
| 26 | **figure-provenance** | **PASS** | files_checked=202 figures_examined=402 violations=0 — **the key that went red at the Epic 2, 3 and 4 wrap-ups** |
| 27 | pcgen-residue-gate | PASS | live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS |
| 28 | token-coverage-selftest | PASS | 14 cases passed |
| 29 | token-coverage | PASS | non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1 verdict=PASS |
| 30 | pi-sweep | PASS | 11 hits over `src/rules_core/rules_tables`, 11 baseline rows |
| 31 | declared-pi-audit | PASS | clean |
| 32 | audit-selftest | PASS | 28 passed, 0 failed |
| 33 | reclaim-selftest | PASS | 13 passed, 0 failed |
| 34 | driver-selftest | PASS | 7 passed, 0 failed |
| 35 | corpus-sweep-selftest | PASS | 15 passed, 0 failed |
| 36 | corpus-trap-audit-selftest | PASS | 14 passed, 0 failed |
| 37 | root-lib | PASS | 3,261 passed |
| 38 | root-full | PASS | 8,772 passed across 413 suites, all 361 `tests/*.rs` suites executed |
| 39 | desktop | PASS | 576 passed |
| 40 | reach | PASS | 32 passed |
| 41 | corpus-sweep | PASS | 48,706 records examined of 51,476 read, 413,314 tokens compared (9 synthesized), 51,463 digests checked, **0 findings** |
| 42 | sheet-rules-check | PASS | records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS (112.9 s) |
| 43 | corpus-trap-audit | PASS | records_examined=27634; defects `wiring-class-mismatch=0 disabled-line=165 key-differs-from-name=650 mod-record=2117 shared-name-distinct-records=249`; traps=407 — all defect kinds at their registered counts |
| 44 | supersession-gate | PASS | 116 objects, all clean |
| 45 | frontend-install | PASS | `npm ci` |
| 46 | frontend-test | PASS | 101/101 files |
| 47 | frontend-typecheck | PASS | `tsc --noEmit` clean |
| 48 | clippy | PASS | root:0 desktop:0 warnings, 0 errors |
| 49 | class-dump | PASS | 31/31 computing |

**No red stage. No wrap-up correction cycle is owed by Epic 5.** The stage count rose 48 → 49
because the Epic 5 correction cycle registered `site-dashboard-pin`; `root-full`'s measured floor
rose 8,727 → 8,772 over the same interval.

---

## 1. Retro summary — `retro.py summary --since <epic start> --json`

The dispatch named `--since 53296d80f0`; `retro.py` takes a time, not a sha, so the window is
that commit's own timestamp:

    git log -1 --format='%ci' 53296d80f0            # -> 2026-09-07 21:24:13 -0400
    python3 scripts/retro.py summary --since 2026-09-07T21:24:13-04:00 --json

This is the **whole-bundle** window (SD-35 launch → now), not Epic 5 alone, as dispatched.

### Counts

| Figure | Value | Re-derive |
|---|---|---|
| Events, total | 284 | `.events.total` from the command above |
| — `verification` | 112 (19 failed; fail rate 0.1696) | `.verification` |
| — `correction` | 87 | `.corrections.total` |
| — `incident` | 30 (0 silent; 507 min lost) | `.incidents` |
| — `deferral` | 22 (21 open, 1 resolved) | `.deferrals` |
| — `resolution` | 15 | `.events.by_type` |
| — `note` | 14 | `.events.by_type` |
| — `rework` | 4 | `.rework.total` |
| — `near_miss` | 0 (0 escaped) | `.near_misses` |
| Commits joined | 163, 1.74 events/commit | `.git_join` |

**Corrections caught before:** implementation 16, merge 7, release 4, brief 2, plus three named
one-offs (the Epic 2 wrap-up gate; the Epic 7 final-acceptance scan; the closure retrospective
that would have read 12 phantom `disk-full` incidents as a missing control). **Zero escaped
near-misses.**

**Failing verify stages over the whole window** (`.verification.by_failing_stage`):
`figure-provenance` 13, `site-dashboard-check` 6, `reachability-audit-selftest` 2,
`shape-engine-boundary-selftest` 2, `denominator-gate` 1. The top two are exactly the two keys
whose controls landed in the Epic 3/4 and Epic 5 correction cycles, and both are green here.

### Recurring incident keys and their controls (`§10` step 1: 3+ must produce a named mechanical control or a named escalation)

Re-derived over `docs/retro/events/*.jsonl` for `ts >= 2026-09-08`:

| Key | Count | Mechanical control, or escalation |
|---|---|---|
| `disk-full` | 6 | **Control exists and these events are its own output.** Every one is emitted by the `codex`/`root` reclaim daemon with an `automated reclaim:` resolution naming what it freed; `verify.sh` stage 1 `preflight-disk` is the pre-build guard. Not unhandled failures. |
| `epic-wrapup-gate-red` | 4 (E2, E3, E4, E5) | **Control = the union of the next two rows**, since every occurrence's cause was one or both of them. **Green in this run**, which is the first evidence the union holds. |
| `site-dashboard-json-stale-after-inventory-move` | 4 | **Control landed:** `./scripts/publish-site-dashboard.sh --check-pin` in the `§6` step 3 push gate (nonzero exit blocks the push, < 1 s, no build), backed by `site/dashboard/inventory-pin.json` and the new `site-dashboard-pin` verify stage — `EPIC-5_wrapup_correction_cycle_receipt.md`. Stages 8 and 9 both PASS here. |
| `figure-provenance-command-on-next-line` | 3 | **Control landed:** `python3 scripts/denominator_gate.py --check-provenance` in the `§6` step 3 push gate — a *different flag* from the `--check` that already ran per-cycle, documented at `workflow-instruction.md §6` step 3. Stage 26 PASS here. |
| `wrong-base-worktree` | **4** | **NO mechanical control. ESCALATED.** Occurrences: `AT-35-E2-REGATE`, `at-35-e2-regate2`, `AT-35-E5-WRAPUP`, and **this worker** — my worktree was created at `85287761d5`, a develop-line merge, with no SD-35 package and no `scripts/pcgen_residue_gate.py` in it. Caught on the turn's first orientation command, before a build was spent; cleared with `git fetch origin tranche/15 && git reset --hard origin/tranche/15` inside my own worktree. Incident `1789065663476-at-35-e5-wrapup-regate-d1f5f0`. **Proposed control:** `scripts/worktree_base_gate.sh` — exits nonzero unless `git merge-base --is-ancestor origin/tranche/15 HEAD` holds — run as the first command of every dispatched cycle and carried as a receipt row, the same shape as the two controls that closed the rows above. |
| `stale-census-in-dispatch-prompt` + `duplicate-criterion-dispatch` | 4 keyed events, and 9 criteria self-reported | **NO mechanical control. ESCALATED.** `kanban.md` rows 37–41 each record a criterion re-dispatched *after its own card already read `complete`*, the 5th through 9th such; `.rework.by_cause` carries "orchestrator dispatch list not reconciled against kanban.md before dispatch — the criterion's row already read 'complete' with a receipt path". Cost is real but bounded: every re-dispatch re-derived its Evidence at HEAD and found nothing to redo (`closed=0 rust_lines_changed=0` on all five). **Proposed control:** generate the orchestrator's dispatch list from `kanban.md` rows whose status is not `complete`, and re-derive the census figures in the prompt at dispatch time rather than transcribing them. |
| `duplicate-criterion-dispatch` (alone) | 2 | Below the 3+ bar; folded into the row above, which is the same mechanism. |

Escalation note emitted: `1789070128336-at-35-e5-wrapup-regate-d4e0e1`.

### `rust_lines_changed / units_closed` above 3.0 (`decisions.md §4`)

Re-derived mechanically over every receipt:

```
grep -h 'Receipt rows (mechanical)' docs/release/SD-35-corpus-sheet-completion/artifacts/epic-*/*receipt*.md
```
parsed for `closed=` / `rust_lines_changed=`, with the ratio recomputed rather than read.

**No cycle in SD-35 exceeded 3.0.** Only **three** cycles closed units at all, and all three are
an order of magnitude under the bar:

| Cycle | closed | rust_lines_changed | ratio (recomputed) |
|---|---|---|---|
| `AT-35-E2-005` cycle 1 | 21,911 | 337 | **0.015** |
| `AT-35-E3-002` cycle 1 | 786 | 266 | **0.338** |
| `AT-35-E3-001` cycle 2 | 618 | 232 | **0.375** |
| every other cycle | 0 | — | undefined (`ratio=n/a`) |

Closure total 23,315, which is the whole scoped non-DONE remainder.

**What the lines bought, for the zero-closure cycles that spent the most Rust** — `§10` step 1
asks the ratio question, and with the denominator zero the honest answer is the reading:

| Cycle | rust_lines_changed | What the lines bought |
|---|---|---|
| `AT-35-E2-001` c1 | 6,463 | The converter itself — the PCGen→`data/sheet_rules/` translator every later closure runs through. One tool, 49,438 units of reach. |
| `AT-35-E6-001` c1–c4 | 613 + 2,173 + 1,168 + 2,520 = 6,474 | The PCGen exit: live-side reads removed, converter/oracle side kept. `pcgen_live_files` 260 → **253** over this epic, and stage 27 confirms it at HEAD. Lines that *delete* a dependency have no unit denominator by construction. |
| `AT-35-E1-003` c1 | 1,906 (+1,000/−906) | The build-time tax cut — test-file consolidation, measured before and after in `artifacts/epic-1-tax-cut/build-time.json`. It bought the ~87-minute gate this report is written from. |
| `AT-35-E2-002` c1 | 1,818 | The derived-value evaluator and its per-kind on-screen tests — the mechanism `sheet-complete` is defined against. |
| `AT-35-E5-004` c1 | 844 | `src/rules_core/level_up_option_filter.rs`, served on `preview_level_up` as `featOptions`/`refusedFeatOptions` — bucket X's actual filter, plus the converter defect it exposed (354 files where `PreStatScore_<AB>` read 0). |

The pattern is consistent with `decisions.md §4`: SD-35's Rust went into **mechanisms**, and the
closure came in three corpus-wide passes, not per-unit grinding. No cycle is in breach.

### Deferrals

21 open of 22. **Epic 5 hands on exactly one of its own**, and it is named by mechanism, not
exempted: `AT-35-E5-005_desc_without_prose.py --check` exits 1 until 0 on a **10-unit**
converter-side residue (`desc-without-prose.json`), deferrals `1788994100821-at-35-e5-005-5973cb`
and its cycle-2 re-affirmation `1789064249278-at-35-e5-005-579de8`. Under the sheet rule this is
not a carve-out: the units are DONE (`token-coverage` stage 29 reads `non_done=0`), and the
residue is a *rendering-quality* gate that stays red until the converter emits prose. The
remaining open deferrals are the SD-34 inheritance and the refused-token families, which
`AT-35-E4-001` and the Epic 7 closure scan own.

**This worker closed no units and refused no tokens** — it is a read-only gate, exempt from the
500-unit floor by `decisions.md §2` and outside the deferral rule for scoped populations.

---

## 2. Worktree sweep — Epic 5's worktrees

    df -h /                # run before the sweep and again after; identical both times
    git worktree list      # 19 entries

- **Disk** (`df -h /`): 1.2 T used of the 1.5 T `/dev/sda1` root filesystem, 312 G available.

Epic 5's receipts name 13 worktrees (`grep -rhoE 'wf_291be5c8-5f3-[0-9]+' artifacts/epic-5-residues/`):
`-2 -3 -4 -5 -7 -14 -15 -17 -21 -24 -27 -30 -35`.

**Merge check — every one is safe to prune.** `git branch --contains <head>` lists `tranche/15`
for all 16 distinct tranche/15 worktree HEADs (17 worktree entries, `-17` and `-41` share
`f1f547a41e`), including `-35` (`c3500e7984`, the first Epic 5 gate worker). **None carries an
unmerged commit. None is `locked`** except my own `-33`, which git marks locked while this turn
runs.

**Nothing was removed, and this is a limitation of the worker, not a carve-out.** This agent runs
worktree-isolated: `git -C <other worktree>` and any compound git form are refused by the tool
wall ("a worktree-isolated agent's git operations must target its own worktree"), so
`git worktree remove` on another agent's worktree is not executable from here. The survey is the
part I can do; **the prune is handed to the orchestrator** with the verified list:

> **Epic 5's own, prunable now** (merged into `tranche/15`, unlocked) — the 13 named by Epic 5's
> receipts: `wf_291be5c8-5f3-{2,3,4,5,7,14,15,17,21,24,27,30,35}`.
> Also merged and unlocked, but **outside this epic's sweep scope** (`§10` step 2 says this
> epic's worktrees only): `-19`, `-23`, `-41` — leave them to their own epic's wrap-up.
> Do **not** remove `wf_291be5c8-5f3-33` (this worker, locked) or
> `.worktrees/ci-trait-choice` (branch `fix/trait-choice-set-id-roundtrip`, not a
> tranche/15 worktree).

Disk stands at 1.2 T used of the 1.5 T root filesystem with 312 G available — no pressure forced
the issue, so deferring the prune costs nothing but the clutter.

---

## 3. No PR

`§10` step 3. None opened. The PR is Epic 7's, after the final-acceptance scan.

---

## PCGen residue gate

    python3 scripts/pcgen_residue_gate.py --check

Literal final line:

```
live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS
```

Never above the previous receipt's (`EPIC-5_wrapup_correction_cycle_receipt.md` carries the same
253/12256), and 7 files below the 260 baseline — Epic 6's removals. Per-root detail from the same
run: `src/campaign` files=0 hits=0, `src/homebrew_authoring` files=0 hits=0,
`apps/desktop` files=51 hits=477, `identifier_files=54 identifier_hits=343`. The converter,
parser, generators and oracle harness are untouched and stay — they are Starfinder's inputs
(`decisions.md §11`).

---

## Retro events emitted by this worker

- `incident` `1789065663476-at-35-e5-wrapup-regate-d1f5f0` — `wrong-base-worktree`, 4th occurrence
- `note` `1789070128336-at-35-e5-wrapup-regate-d4e0e1` — the two escalated keys and their proposed controls
- `verification` `1789070067091-epic5-wrapup-51f5d4` — derived, emitted by `verify.sh` itself: `full`, `PASS`, 49 stages, 5,221 s

Both live in `docs/retro/events/at-35-e5-wrapup-regate.jsonl` and
`docs/retro/events/epic5-wrapup.jsonl` **in this worker's worktree only**. The orchestrator folds
them with this report.
