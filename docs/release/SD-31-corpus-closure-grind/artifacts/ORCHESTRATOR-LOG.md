---
canonical: true
owner: sd31-orchestrator
purpose: Running dispatch log for the SD-31 orchestrating session (Opus-high, Workflow-tool dispatch).
started: 2026-08-15
---

# SD-31 — Orchestrator Dispatch Log

One row per Workflow launch. The orchestrator never implements directly; this file is the record of
what it dispatched, against what board state, and what came back.

## Baseline at orchestration start (2026-08-15)

Re-derived by replaying the dashboard producer's own `doneness_verdict()` over the live
`docs/work-inventory.json` (`generated_at 2026-08-15T01:34:18Z`), not transcribed:

```
python3 -c "
import json, sys, collections
sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
d = json.load(open('docs/work-inventory.json'))
U = [u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
c = collections.Counter(P.doneness_verdict(u.get('wiring_class'),u.get('status'),u.get('kind')) for u in U)
print(len(U), dict(c), round(100*c['done']/len(U),2))
"
```

- denominator **38,521**; `done` **5,837** = **15.15 %**
- `not-started` 20,895 · `held` 6,916 · `unmeasurable` 3,989 · `in-progress` 848 · `deferred` 36

| kind | total | done | done % | held | not-started |
|---|---:|---:|---:|---:|---:|
| class_feature | 15,472 | 25 | 0.2 % | 88 | 11,703 |
| equipment | 6,208 | 2,626 | 42.3 % | 2,327 | 962 |
| race_trait | 3,447 | 266 | 7.7 % | 247 | 2,934 |
| monster_ability | 3,107 | 334 | 10.7 % | 1,295 | 1,478 |
| spell | 2,843 | 47 | 1.7 % | 1,103 | 1,561 |
| feat | 2,610 | 1,178 | 45.1 % | 89 | 973 |
| companion | 1,696 | 416 | 24.5 % | 506 | 774 |
| equipment_modifier | 1,580 | 911 | 57.7 % | 19 | 228 |
| monster | 1,270 | 7 | 0.6 % | 1,235 | 28 |
| class | 185 | 27 | 14.6 % | 0 | 158 |
| race | 103 | 0 | 0.0 % | 7 | 96 |

Branch `tranche/11` at `9bdd463b8`, pushed. Oracle pin `7f818006e371188e5717fd18d74d18a420747fc6`,
`verify.sh --only preflight-oracle` PASS. Box: 24 cores / 167 GiB / 968 G at **16 % used** →
headroom to the 90 % floor = 720 G → **full-gate concurrency cap 8** (720 ÷ 82 G accumulated-primary
footprint = 8.8; CPU bound is 12; disk binds).

## Dispatches

| # | Launched | Workflow / runId | Scope | Outcome |
|---|----------|------------------|-------|---------|
| 1 | 2026-08-15 | wave `sd31-w1-foundation` (agents: `sd31-e0-audit`, `sd31-e2-groundtruth`, `sd31-w1-refute-e0`, `sd31-w1-refute-e2`, integration `sd31-w1-integrate`) | Epic 0 build+baseline (`SD31-E0-F1-001`); Epic 2-F1 ground-truth sample on worktree branch `sd31/e2-groundtruth` (`SD31-E2-F1-001`); two Opus adversarial reviews of both deliverables; this integration cycle (`SD31-W1-INTEGRATE-001`) merging the worktree branch, fixing all 9 CONFIRMED review findings, re-running Epic 0's audit at the integrated tip, and the wave's full gate | Epic 0: `scripts/reachability_audit.py` built/wired/self-tested, baseline committed, `kanban.md` `epic-0-reachability-audit` → `COMPLETE`, re-invoked clean at the merged tip (94.53 % ceiling unchanged, `AUDIT_EXIT=0`). Epic 2: F1 sample (150 units) merged onto `tranche/11` (commit `ce0f534a9`) but **not closable as-is** — 105/150 labels unevidenced, headline agreement figures withdrawn, `epic-2-verdict-paths` stays `READY`, gated on re-labelling (`OPEN-ISSUES.md` row 3, `BLOCKER`). All 9 CONFIRMED findings fixed or logged with a named owner and remedy (commit `4d33ea331`); 3 REFUTED E0 findings correctly left unactioned. Full gate: see `progress.md` cycle `SD31-W1-INTEGRATE-001` for `VERIFY_EXIT` and stage summary. |
| 2 | 2026-08-15 | wave `sd31-w2-ambiguous-and-capability` (agents: `sd31-e2-wiringfix`, `sd31-e2-relabel`, `sd31-e1-chassis` on their own branches, three Opus adversarial reviewers, integration `sd31-w2-integrate`) | `SD31-E2-F2-001-wiringfix` (Findings A/B/C fixed in `wiring_class.rs`, landed directly on `tranche/11` before this integration cycle started); `SD31-E2-F1-002-relabel` (105-unit canned-evidence repair + 35-unit widening draw, worktree branch); `SD31-E1-F1-001` (race chassis design + Bestiary 2 six-race batch, worktree branch); three Opus adversarial reviews (one per deliverable) surfacing 14 CONFIRMED findings across all three, none `GAMED`; this integration cycle (`SD31-W2-INTEGRATE-001`) merging both remaining worktree branches, fixing all 14 CONFIRMED findings (headline: the D3 wiringfix's own 55-unit anti-gaming over-shoot, `has_arith_scoped`/`has_scalar_or_arith_for_token`), the one sanctioned guarded regen, the standing audit, and the wave's full gate | Both worktree branches merged, content proven by grep not status (`progress.md` `SD31-W2-INTEGRATE-001` §1). All 14 CONFIRMED findings fixed forward across 6 commits (`ed2d7adbb`..`dfb56996d`); zero `GAMED` verdicts, so no revert needed. Re-derived the ground-truth sample's engine snapshot against the merged tip (Finding 9/12's remedy): 167/185 agree. Guarded regen committed (`faa14e9fa`): board `done` 5,837 → 6,076 (+239, 15.15% → 15.77%), denominator unchanged at 38,521, zero stamp loss confirmed, second run changes only `generated_at`. Standing audit (`4486255e0`): reachable ceiling unchanged at 98.94% (this cycle's fix moves units between `derived`/`static`, not into/out of `ambiguous`); vs. wave 1's baseline, 94.53% → 98.94%, dead-end units 2,109 → 409, `race` reachability 52.43% → 100.00% and `race_trait` 79.98% → 99.56% (both Epic 1's doing). Full gate: see `progress.md` cycle `SD31-W2-INTEGRATE-001` for `VERIFY_EXIT` and stage summary. `kanban.md`: `epic-1-race-chassis`, `epic-2-verdict-paths`, `epic-6-ingest-lanes` rows updated honestly — none promoted to `COMPLETE` (Epic 2 stays gated on F3's `ambiguous` dead-end closure, 409 units unchanged; Epic 6-F11 landed 0 new fixtures, does not close on one fixture batch). |

## Wave budgets

### Wave 1 (cycle `SD31-W1-PREFLIGHT-001`, sd31-w1-preflight, 2026-08-15)

Dispatches 2 concurrent full-gate agents. Re-measured box and budget arithmetic per SD-30
`loop-instruction.md` "Concurrency and resource budget" methodology:

```
nproc                        # 24
free -h                      # 167Gi total / 6.9Gi used / 154Gi free / 161Gi available
df -B1G /                    # 968 total / 151 used / 818 avail / 16%
du -sh /home/ubuntu/cargo-targets/* target 2>/dev/null
#   cargo-targets/ empty — no prior agent target dir present
#   83G  target               (primary checkout's accumulated tree)
```

| quantity | value | how |
|---|---:|---|
| cores | 24 | `nproc` |
| RAM | 167 Gi total, 154 Gi free | `free -h` |
| filesystem | 968 G | `df -B1G /` |
| currently used | 151 G (16 %) | `df -B1G /` |
| `preflight-disk` refuses at | 90 % used or < 20 G free | `verify.sh:243-244` |
| headroom to 90 % floor | **720.2 G** | `0.90 × 968 − 151` |
| headroom to 20 G-free floor | 798 G | `818 − 20` |
| binding headroom | **720.2 G** (90 % floor) | `min(720.2, 798)` |
| full-gate `CARGO_TARGET_DIR` footprint | 83 G | `du -sh target` (accumulated primary; no fresh-agent sample available — `cargo-targets/` empty this cycle) |
| concurrent full-gate agents (disk) | **8** | `720.2 ÷ 83 = 8.68` -> floor 8 |
| concurrent full-gate agents (CPU) | 12 | `24 ÷ 2` default `-j`; not binding |
| RAM headroom | ample | not binding |
| binding constraint | disk | 8 < 12 |
| **CAP** | **8** | smaller of the bounds |

**This wave dispatches 2. 2 ≤ 8 — budget admits 2**, with headroom to spare up to the cap of 8.

**`reclaim.sh` this cycle:** dry run then `--apply`, **0.0 B reclaimed** (25 candidates, all correctly
skipped — too-young verify-logs, unmerged branches, one non-cargo-target stray dir). Confirmed
`reclaim.sh`'s `cargo-target` category scans only `SCRATCHPAD_ROOT` (`/tmp/claude-1000`) and
`CACHE_ROOT` (`$HOME/.cache`) — never `/home/ubuntu/cargo-targets/`, the directory this package
mandates every agent's `CARGO_TARGET_DIR` live under. Known gap, not fixed this cycle (out of this
cycle's scope; noted for a future hardening card).

**Board baseline re-derive (same cycle):** replayed `doneness_verdict()` over live
`docs/work-inventory.json` (`generated_at 2026-08-15T01:34:18Z`) — denominator 38,521, `done` 5,837
(15.15 %), matching this file's "Baseline at orchestration start" table exactly on every figure
(overall breakdown and all 11 per-kind rows). No hard stop; both reads resolve to the same unchanged
snapshot. Full detail and per-command re-derivation: `progress.md` cycle `SD31-W1-PREFLIGHT-001`.

### Wave budget — `sd31-w2-ambiguous-and-capability` (dispatcher-computed, before fan-out)

Computed by the orchestrating session per SD-30 `loop-instruction.md` "Concurrency and resource
budget" rule 2 (*"the budget is checked before the fan-out, not by each agent afterwards"*), at
`tranche/11` tip `a3acc8e80`, immediately after reclaiming wave 1's orphaned target dir.

```
df -B1G /                                   # 968 total / 179 used / 789 avail / 19%   (pre-reclaim)
du -sh target /home/ubuntu/cargo-targets/*  # 83G target ; 28G cargo-targets/sd31-w1-integrate (orphaned)
pgrep -fa 'verify.sh|cargo test|cargo build'   # no live build -> safe to remove
rm -rf /home/ubuntu/cargo-targets/sd31-w1-integrate
df -B1G /                                   # 968 total / 151 used / 817 avail / 16%   (post-reclaim, +28G)
```

| quantity | value | how |
|---|---:|---|
| filesystem / used | 968 G / **151 G (16 %)** | `df -B1G /`, post-reclaim |
| headroom to the 90 %-used floor | **720.2 G** | `0.90 × 968 − 151` |
| headroom to the 20 G-free floor | 797 G | `817 − 20` |
| binding headroom | **720.2 G** | 90 % floor binds |
| measured full-gate `CARGO_TARGET_DIR` footprint | **83 G** | `du -sh target` |
| cap — concurrent full-gate agents (disk) | **8** | `720.2 ÷ 83 = 8.68`, floored |
| cap — concurrent full-gate agents (CPU) | 12 | `nproc 24 ÷ -j 2`; not binding |
| **this wave dispatches** | **4 build agents, at most 3 of them full-gate** | within the cap of 8 |

Wave 1's lesson applied: `reclaim.sh` does not scan `/home/ubuntu/cargo-targets/`
(`OPEN-ISSUES.md` note, `SD31-W1-PREFLIGHT-001`), so the dispatcher reclaims that root by hand
before each fan-out until a cycle fixes the script.

### Why this wave targets what it targets

Wave 1's two Opus adversarial reviews surfaced the largest structural lever on the board, which no
planning pass had found (`OPEN-ISSUES.md` row 1):

```
python3 -c "
import json,collections,sys
sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
amb=[u for u in U if (u.get('wiring_class') or 'ambiguous')=='ambiguous']
print(len(amb), collections.Counter(u.get('wiring_class_reason') for u in amb).most_common())
"
# -> 2109  [('no_corpus_line', 1707), ('prose_scaling_phrase', 291), ('prose_ability_scaling', 111)]
```

`ambiguous` is the board's only wiring class that reaches `done` from **no** status — every one of
those 2,109 units is a structural dead-end (`SD31-E0-F1-001-baseline.json`, 9 dead-end cells, all
`ambiguous|*`). **1,707 of them (80.9 %) carry `wiring_class_reason == no_corpus_line`**, and wave 1
proved by recursive glob that **100 % of those rows genuinely exist** — they are missed only because
`wiring_class::CorpusLines::line()` joins one directory level and these books nest their `.lst`
files. That is a bug fix, not a reclassification, and it is the single highest-leverage change
available on the board today. It is dispatched here as this wave's primary card, with an Opus
adversarial reviewer whose explicit job is to prove the fix resolves each unit to its **correct**
row rather than merely to *a* row — the anti-gaming risk on a change of this size is exactly that.

