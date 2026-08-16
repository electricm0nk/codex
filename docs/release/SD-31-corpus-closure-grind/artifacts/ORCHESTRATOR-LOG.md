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

| 3 | 2026-08-15 | wave `sd31-w3-grind` (agents: `sd31-race-lane`, `sd31-e6-seam`, `sd31-e6-equipment`, `sd31-e6-spell-mab`, `sd31-e2-close`, `sd31-e3-measure`, three Opus adversarial reviewers, integration `sd31-w3-integrate`) | Six kind/measurement lanes dispatched concurrently: `SD31-E6-F4-001` (Skinwalker chassis follow-on + `race`-kind root cause, own worktree); `SD31-E6-F11-002` (`monster` derived-evaluator seam, own worktree); `SD31-E6-F5-001` (`ultimate_equipment` book onboard, own worktree); `SD31-E6-F2-001`/F9 (`spell`/`monster_ability`, own worktree); `SD31-E2-F3-001` (`ambiguous` dead-end closure: classifier fixes, `PLUS:` sweep rejected, SER proposal, own worktree); `SD31-E3-F1-001` (Epic 3 class inventory + hand-verification, landed direct to `tranche/11`, no worktree). Three Opus adversarial reviews (equipment+spell paired; seam+ambiguous paired; class_feature+race paired) surfacing 17 CONFIRMED findings across all five ingest/capability deliverables, all gaming verdicts CLEAN; this integration cycle (`SD31-W3-INTEGRATE-001`) merging all five worktree branches, fixing/logging all 17 CONFIRMED findings, the one sanctioned guarded regen, the standing audit, and the wave's full gate | All five worktree branches merged onto `tranche/11`, content proven by grep not status (`progress.md` `SD31-W3-INTEGRATE-001` §1). `OPEN-ISSUES.md`'s five-way row-anchor collision (every lane independently appended rows 22-24/25 at the same anchor) resolved by sequential renumbering 22→45 with every cross-reference fixed. 7 of 17 CONFIRMED findings fixed in code/data with tests this cycle (D7 `SPELLS:` slash false-positive, `miser_s_mask`'s wrong cost/weight, a false PI-screening claim in `bestiary_5/LICENSE.json`, the Epic-3 clearance table's undisclosed 5-class filter gap, a broken kanban.md table, the `OPEN-ISSUES.md` renumbering itself, and the `verify-baselines.env` 37+2→39+3 accounting); 10 logged to `OPEN-ISSUES.md` (rows 38-45) with remedy + owning epic — none silently dropped. 17 retro correction/note events emitted, each `--verified-by`. Guarded regen committed (`c9c85c181`): board `done` 6,076 → **7,355 (+1,279)**, 15.77% → **19.09%**, denominator unchanged at 38,521; `--allow-stamp-loss` used once, only after tracing the exact 2 losses one record deep (a genuine, correct `static`→`derived` reclassification from the already-merged `SPELLS:` field addition, not a report gap). Standing audit: reachable ceiling 98.94% → **98.95%** (+0.01pp), 9 dead-end cells unchanged, all `ambiguous`, all Epic-2-owned, `AUDIT_EXIT=0`. Full gate: **VERIFY_EXIT=0, 22/22 stages PASS** (root-lib 1816, root-full 6465/552 suites, desktop 445, reach 27, corpus-sweep 0 findings, frontend 99/99, clippy 46+7 warnings/0 errors, class-dump 31/31) — `progress.md` cycle `SD31-W3-INTEGRATE-001` §5 has the full log excerpt; baselines raised in a separate commit per DoD item 7. `kanban.md`: `epic-0`/`epic-1`/`epic-2`/`epic-6` rows updated honestly — none promoted to `COMPLETE` this wave (Epic 2 stays gated on the 404-unit `ambiguous` dead-end and the operator's SER/done-bar ruling; Epic 6 stays open on its own residual `not-started` mass and the row-38/39/40/44 rulings). |
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


### Wave budget — `sd31-w3-grind` (dispatcher-computed, before fan-out)

Computed at `tranche/11` tip `a272396d3`, after hand-reclaiming two non-live wave-2 target dirs.
Wave 2's own integration gate was still on its final `clippy` stage in
`/home/ubuntu/cargo-targets/sd31-w2-integrate` at dispatch time, so that dir was **left alone** —
every PID under `pgrep -fa 'verify.sh|cargo '` was checked against `/proc/<pid>/environ`'s
`CARGO_TARGET_DIR` before anything was removed, per the never-`pkill`-a-shared-pattern rule.

```
pgrep -fa 'verify.sh|cargo '                       # 4 live PIDs, all CARGO_TARGET_DIR=sd31-w2-integrate
tr '\0' '\n' < /proc/<pid>/environ | grep CARGO_TARGET_DIR   # per PID, before removing anything
rm -rf /home/ubuntu/cargo-targets/sd31-e2-wiringfix-v2       # 28 G, its gate had finished PASS
rm -rf /home/ubuntu/cargo-targets/sd31-w2-refute-wiringfix   # 129 M, read-only agent, finished
df -B1G /                                          # 968 total / 184 used / 785 avail / 19%
```

| quantity | value | how |
|---|---:|---|
| filesystem / used | 968 G / **184 G (19 %)** | `df -B1G /`, post-reclaim |
| headroom to the 90 %-used floor | **687.2 G** | `0.90 × 968 − 184` |
| conservative full-gate footprint | 83 G | accumulated-primary `target/`; a fresh per-agent dir measured 28–29 G this wave |
| cap — concurrent full-gate agents (disk) | **8** | `687.2 ÷ 83 = 8.3`, floored |
| cap — concurrent full-gate agents (CPU) | 12 | `nproc 24 ÷ -j 2`; not binding |
| **this wave dispatches** | **6 build agents** (+3 read-only Opus verifiers, which do not count) | `6 × 83 = 498 G` against 687 G headroom |

**Wave 2's gate outcome, for the record.** The wiringfix cycle's re-run
(`artifacts/SD31-E2-F2-001-wiringfix-verify-v2.log`) closed **`RESULT: PASS`, `VERIFY_EXIT=0`**,
resolving the "terminal exit code not obtained" follow-up it returned. The integration cycle's own
gate passed every stage through `frontend-typecheck` (root-lib 1795, root-full 6430 across 549
suites, desktop 445, reach 27, corpus-sweep 0 findings, frontend 99/99) and was still running
`clippy`, the final stage, when wave 3 was dispatched.

### Board after wave 2 (measured at `a272396d3`, not carried)

| figure | wave-1 baseline | after wave 2 | delta |
|---|---:|---:|---:|
| `done` / 38,521 | 5,837 (15.15 %) | **6,076 (15.77 %)** | **+239** |
| reachable ceiling | 94.53 % | **98.94 %** | **+4.41 pts (+1,700 units)** |
| `ambiguous` dead-end population | 2,109 | **409** | **−1,700** |
| `race_trait` done | 266 (7.7 %) | **478 (13.9 %)** | +212 |
| `held` | 6,916 | 6,790 | −126 |
| `not-started` | 20,895 | 20,737 | −158 |

The reachable ceiling is now the binding fact for planning: **98.94 %** of the board can reach
`done` with today's engine capability, so from here the gap is overwhelmingly *grind*, not
*capability*. Wave 3 is sized accordingly — six lanes, each targeting the largest not-done mass its
kind carries.


### Board after wave 3 (measured at `c9c85c181`, guarded regen committed)

| figure | after wave 2 | after wave 3 | delta |
|---|---:|---:|---:|
| `done` / 38,521 | 6,076 (15.77 %) | **7,355 (19.09 %)** | **+1,279** |
| reachable ceiling | 98.94 % | **98.95 %** | **+0.01 pt** |
| `ambiguous` dead-end population | 409 | **404** | −5 |
| `equipment` done | 2,650 | **3,904** | +1,254 |
| `equipment_modifier` done | 911 | **917** | +6 |
| `spell` done | 47 | **56** | +9 |
| `race_trait` done | 478 | **484** | +6 |
| `monster` done | 7 | **14** | +7 |
| `held` | 6,790 | **5,596** | −1,194 |
| `not-started` | 20,737 | **20,546** | −191 |

The reachable ceiling barely moved (98.94% → 98.95%) — this wave was pure grind against an already
largely-reachable board, exactly as the wave-2 handoff predicted. `equipment` (`ultimate_equipment`
book onboard) supplied the overwhelming majority of the wave's movement; `spell`/`monster`/`race_trait`
each landed smaller, real gains; `ambiguous` shrank by 5 from the ambiguous lane's OWN `SD31-E2-F3-001` classifier fixes (three of the
six named gaps: `SPELLS:` field scanning, case-insensitive `classlevel(...)`, `+`-then-`(` arithmetic),
landed before this integration cycle merged it — this cycle's own D7 fix (the slash-in-spell-name
false positive within `SPELLS:` scanning) is orthogonal to the `ambiguous` bucket and left it
unchanged at 404, as expected (it corrects `static` vs `derived`, not `ambiguous` membership). Full per-kind table and every command: `progress.md` cycle
`SD31-W3-INTEGRATE-001` §3.

### Wave budget — `sd31-w4-cachegen` (dispatcher-computed, before fan-out)

Computed at `tranche/11` tip `9e715b96e`. No live build on the box (`pgrep -fa 'verify.sh|cargo '`
returned only a dead PID), so all three orphaned wave-2/3 target dirs were removed by hand.

```
pgrep -fa 'verify.sh|cargo '                 # no live gate
rm -rf /home/ubuntu/cargo-targets/{sd31-w2-integrate,sd31-w2-integrate-desktop,sd31-w3-integrate}
df -B1G /                                    # 968 total / 160 used / 809 avail / 17%   (freed 61 G)
```

| quantity | value | how |
|---|---:|---|
| filesystem / used | 968 G / **160 G (17 %)** | `df -B1G /`, post-reclaim |
| headroom to the 90 %-used floor | **711.2 G** | `0.90 × 968 − 160` |
| conservative full-gate footprint | 83 G | accumulated-primary `target/` |
| cap — concurrent full-gate agents (disk) | **8** | `711.2 ÷ 83 = 8.6`, floored |
| **this wave dispatches** | **6 build agents** (+3 read-only Opus verifiers) | `6 × 83 = 498 G` against 711 G |

### The repeatable lever wave 3 proved — and the map of how much of it is left

Wave 3's single largest result was not an ingest cycle. The equipment lane found that
`ultimate_equipment` had **an already-shipped, already-catalog-wired `rules_tables` module (1,369
equipment + 180 equipment_modifier records) and no `data/corpus/` directory at all**. It built
`cache_gen::ultimate_equipment` to dump that table to real, PI-screened, citation-verified JSON,
and **1,264 units reached `done`** — 99 % of the wave's entire board movement, with no new corpus
ingest whatsoever. The records were always there; nothing was reading them.

That is a *repeatable* shape, and this is how much of it remains, re-derived at this tip:

```
ls src/rules_core/rules_tables/ | wc -l        # 38 book modules shipped
ls src/rules_core/cache_gen/                   # acg.rs apg.rs beastiary1.rs mod.rs ultimate_equipment.rs
ls -d data/corpus/*/class_feature | wc -l      # 1
for d in data/corpus/*/; do echo "$(basename $d) : $(ls $d | grep -v LICENSE | tr '\n' ',')"; done
```

- **38 shipped `rules_tables` book modules; 5 `cache_gen` modules.**
- **Exactly ONE book (`pathfinder_unchained`) has a `class_feature` corpus directory** — out of the
  23 in-scope `class_feature` books. `class_feature` is 15,472 units, **40 % of the whole board**,
  and it sits at 25 done (0.2 %).
- `OPEN-ISSUES.md` row 11 independently reached the same place from the other direction: of 2,481
  `static`-held units, **2,367 have no `data/corpus/<book>/<kind>/` directory at all**, and zero
  overlap `corpus_literal_sweep`'s verified set.

Wave 4 is therefore built around that lever rather than around fresh ingest: two lanes doing
cache-gen dumps (one dedicated to `class_feature` alone), plus the four highest-unit-count repairs
wave 3's adversarial reviews left owned but unfixed.

### Board after wave 3 (measured at `9e715b96e`, not carried)

| figure | w1 baseline | after w2 | **after w3** | w3 delta |
|---|---:|---:|---:|---:|
| `done` / 38,521 | 5,837 (15.15 %) | 6,076 (15.77 %) | **7,355 (19.09 %)** | **+1,279** |
| reachable ceiling | 94.53 % | 98.94 % | 98.95 % | +0.01 |
| `ambiguous` | 2,109 | 409 | 404 | −5 |
| `held` | 6,916 | 6,790 | **5,596** | −1,194 |
| `equipment` done | 2,626 (42.3 %) | 2,650 (42.7 %) | **3,904 (62.9 %)** | +1,254 |
| `class_feature` done | 25 (0.2 %) | 25 (0.2 %) | 25 (0.2 %) | 0 |

Three Opus adversarial reviews returned **zero GAMED verdicts** across five deliverables and 17
CONFIRMED findings; 7 were fixed in code before integration and 10 logged. Full gate at the
integrated tip: `VERIFY_EXIT=0`, 22/22 stages.

`class_feature` has not moved in three waves. It is now the entire remaining problem, and wave 4
puts two lanes on it.

