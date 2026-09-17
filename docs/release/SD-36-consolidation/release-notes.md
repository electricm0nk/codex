---
canonical: true
owner: operator
bundle_id: SD-36
date: 2026-09-15
---

# SD-36 Release Notes

Final measured baselines (before/after) and bundle summary.

---

## Build 0.16.0

**Release date:** 2026-09-15 (tranche/16 cut)

**What changed:** Dashboard freeze, PCGen wall, bloat cuts.

---

## Measured baselines (re-derived at closure)

| Figure | Before (SD-35) | After (SD-36) | Command | Change |
|---|---|---|---|---|
| src lines | 465,469 | <measure-at-b-end> | `find src -name '*.rs' -exec cat {} + \| wc -l` | down ~40k (B, C1) |
| tests lines | 182,070 | <measure-at-c2-end> | `find tests -name '*.rs' -exec cat {} + \| wc -l` | down ~49k (B, C2) |
| test entries | 8,723 | <measure-at-c2-end> | `cargo test --locked -- --list \| wc -l` | same or +5 (ingest suites) |
| root binaries | 419 | 412 | `scripts/verify-baselines.env` | down 7 (B deletions) |
| root full tests | 8,926 | <measure-at-epic-end> | `scripts/verify-baselines.env` | down ~50 (B, A) |
| ingest binaries | 0 | 47 | (new crate) | up 47 (A moves) |
| ingest tests | 0 | ~82 | (new crate) | up ~82 (A moves) |
| public status | 95.0% of 37,892 | 100.0% of 49,450 | `site/status-data.json` | frozen at 100% (B) |
| pilot_compute/mod.rs | 88,828 | <4,000 | `wc -l src/rules_core/pilot_compute/mod.rs` | down ~85k (C1 split) |
| sd18_widening/sd13_progression | ~75,000 | ~26,000 | `find tests/sd18_widening tests/sd13_progression -name '*.rs' \| xargs cat \| wc -l` | down ~49k (C2 rewrite) |

---

## Summary

**SD-36 consolidates the codebase after SD-35:**

1. **Epic B** — Froze the PF1e status page at 100% complete (49,450 units). Deleted 55,827 lines of supporting machinery (`v06_work_inventory`, `support_state_matrix`, `reach_gate`, dashboard cron jobs).

2. **Epic A** — Walled off PCGen (converter + oracle) in dedicated `crates/codex-ingest` crate. Desktop lists it dev-dep only. Build gate enforces the wall.

3. **Epic C** — Cut bloat: split `pilot_compute/mod.rs` into 36 submodules, consolidated path helpers, rewrote table-driven tests as table→row→test structure.

4. **Epic D** — Closure: refreshed architecture docs, wrote retrospective, published `tranche/16` → `develop` PR.

**Result:** Clean codebase, PCGen walled off by build not script, disk reclaimed, CI green.

---

## Deferred work

See `forward-scope-register.md` for five deferred capability rows (semantic dedups, SD-34 correctness, rules_tables data-package, producer extraction, CI oracle fetch).

---

