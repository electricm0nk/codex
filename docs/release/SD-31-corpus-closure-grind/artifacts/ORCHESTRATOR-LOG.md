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
