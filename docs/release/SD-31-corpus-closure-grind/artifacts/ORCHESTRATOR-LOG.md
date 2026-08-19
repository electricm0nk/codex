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
| 4 | 2026-08-16 | wave `sd31-w4-cachegen` (agents: `sd31-cachegen-cf` direct to `tranche/11`, `sd31-pi-fix`, `sd31-sweep-attrib`, `sd31-monster-widen`, `sd31-spell-reach`, `sd31-cachegen-rest` (equipment_gap) on their own branches, three Opus adversarial reviewers, integration `sd31-w4-integrate`) | `SD31-E5-F1-001` (`class_feature` cache-gen, 12,431 records across 21 books, landed direct to `tranche/11`); `SD31-PI-REPAIR-001` (declared-PI reader wired into `ultimate_equipment`/`ingest_races`, own worktree); `SD31-E6-F3-002` (corpus_literal_sweep book-attribution fix, race off zero, own worktree); `SD31-E6-F1-002` (monster caster-level seam production wiring + `stat_adjustments` widening, own worktree); `SD31-E6-F2-002` (Ultimate Magic spell ingest + 881-record citation repair, own worktree); `SD31-E6-F5-002` (`equipment_gap` cache-gen, 697 records across 8 books, own worktree). Three Opus adversarial reviews (class_feature+equipment_gap paired; PI-repair+sweep-attrib paired; monster+spell paired) surfacing 23 CONFIRMED findings, gaming verdicts CLEAN on all 6 targets, PI verdicts EXPOSED on 2 of 3 review passes (class_feature's NAME blacklist hole; the corpus-wide raw_tokens DESC leak the pi-fix branch's own new gate did not check); this integration cycle (`SD31-W4-INTEGRATE-001`) merging all 5 remaining branches, fixing every CONFIRMED PI finding first, then every other CONFIRMED finding reachable within budget, the one sanctioned guarded regen, the standing audit, and the wave's full gate (launched twice — see `progress.md` for why) | All 5 branches merged onto `tranche/11` (the equipment_gap branch merged last, after discovering mid-cycle it had been omitted from the first pass — corrected before further work assumed its presence). `OPEN-ISSUES.md`'s row-number collisions (6 rows claimed "46", 4 claimed "47", 2 claimed "48") renumbered 50-66; a "Needs an operator ruling" summary section added. **PI findings fixed first, both SAFETY-CRITICAL**: class_feature's NAME blacklist-scan hole (14 records exposed, 2 unmarked — fixed, re-verified 0 exposed) and a corpus-wide raw_tokens DESC leak spanning every prior PI-screening cycle (413 records, not the review's own 32-record figure against a smaller pre-merge tree — fixed, `declared-pi-audit: CLEAN` re-verified). Also fixed: the UPSI `corpus_literal_sweep` abort that had silently prevented the sweep from EVER completing over equipment_gap's 697 records (widened `book_dir_of` for Dreamscarred Press's shorter path shape); 3 corpus-fidelity records reverted to a safe empty state pending a root-cause parser fix; 3 disabled-`#`-row equipment records; the LICENSE.json double-count across 4 books plus 6 entirely missing LICENSE.json artifacts; 2 pre-existing (confirmed predating this wave's own start) test-pin defects both traced to the same 8 ACG records. Guarded regen committed: board `done` **7,355 → 7,603 (+248)**, 19.09% → **19.74%**, denominator unchanged at 38,521, zero stamp loss, second run changes only `generated_at`. Standing audit: reachable ceiling unchanged at **98.95%**, same 9 dead-end cells, all Epic-2-owned. Trap report: `TRAP_EXIT=2` (unchanged sign), wiring-class-mismatch count 950→1,191 — NOT root-caused within budget, per-kind breakdown shows zero from `class_feature`/`equipment` despite those being the wave's largest deltas, logged for the next dispatch (`OPEN-ISSUES.md` row 65). Full gate: see `progress.md` cycle `SD31-W4-INTEGRATE-001` for `VERIFY_EXIT` and stage summary (run 1 caught 2 pre-existing test-pin defects and a clippy-ceiling drift, all fixed and independently re-verified before run 2). `kanban.md`: `epic-0`/`epic-6` rows updated honestly — none promoted to `COMPLETE` this wave. |
| 5 | 2026-08-16 | wave `sd31-w5-grind` (agents: `sd31-book-attrib`, `sd31-e4-classwire`, `sd31-spell-lists`, `sd31-monster-ability`, `sd31-equip-residual` on their own branches/worktrees, three Opus adversarial reviewers, integration `sd31-w5-integrate`) | `SD31-ATTRIB-001` (book attribution, 1,610-unit `core_essentials` relabel, own worktree, merged first per dispatch since siblings' `held` units were stuck on the labelling defect it fixes); `SD31-E4-F1-001` (Slayer archetype-supersession wiring, 15 records, own worktree); `SD31-E6-F2-003` (Occult Adventures spell ingest, 144 records, own worktree); `SD31-E6-F9-001` (monster_ability row-34 fix + 1,616-record raw_tokens enrichment, own worktree); `SD31-E6-F5-003` (equipment residual, 620 new records across 4 books, own worktree). Three Opus adversarial reviews (D7-PROSE solo; book-attrib+classwire paired; spell-lists+monster-ability+equipment triple) surfacing 15 CONFIRMED findings across all deliverables (D7-PROSE's own prior-cycle work included), gaming verdicts CLEAN on every target, PI verdicts CLEAN on every target (first wave with zero PI exposure in three); this integration cycle (`SD31-W5-INTEGRATE-001`) merging all 5 branches, fixing 4 of the 15 CONFIRMED findings in code (chosen for being cheap/TDD-provable/no-ruling-needed), correcting 2 false claims and answering the operator's own row-68 question directly in `OPEN-ISSUES.md`, logging the remaining findings (equipment mis-citation, the systemic `corpus_literal_sweep` typed-field gap, the flat-magnitude race_trait ruling question) with remedy and owning epic, the one sanctioned guarded regen, the standing audit, and the wave's full gate | All 5 branches merged onto `tranche/11`, content proven by grep not status. `OPEN-ISSUES.md`'s row-number collisions (three separate lanes independently landed on 69-71/69-72 against three different bases) renumbered 78-91 with every cross-reference preserved; a PRE-EXISTING collision from the already-merged wave-4-adjacent `SD31-E4-F1-001` (its own rows had silently landed on 69-71 too, since that merge auto-resolved with no conflict flagged) caught by an explicit post-merge duplicate-number sweep and fixed alongside. 4 CONFIRMED findings fixed in code with TDD: the race_trait text-complete rung's PI-redaction-placeholder gap (146→141 promotions), the `gathlain` book-attribution over-claim (moved to the ambiguous set per the lane's own disambiguation test), `engine_book_for`'s `unit.book`/`unit.source_book` inconsistency at the reconciliation-aggregate call site, and the clippy failure blocking `SD31-D7-PROSE-001`'s own gate. Guarded regen committed (this cycle's own fixes plus the fully-merged tip): board `done` **7,603 → 7,340 (−263)**, 19.74% → **19.05%** — net of `SD31-D7-PROSE-001`'s anti-gaming description-completeness fix landing for the first time at a fully-merged tip (a further −1,060-shaped demotion corpus-wide, most visible in `equipment_modifier` 920→84), partly offset by this wave's real gains (`equipment` +342, `monster_ability` +102, `race_trait` +141 net of the PI-placeholder fix). 3 stamp losses traced one record deep and confirmed as the correct, deliberate propagation of that same fix (3 `ultimate_equipment` records with null corpus descriptions correctly demoted off a bogus `literal-verified`), not a report gap — `--allow-stamp-loss` used once, second run changes only `generated_at`. Standing audit: reachable ceiling unchanged at **98.95%**, same 9 dead-end cells, all Epic-2-owned. Trap report: `TRAP_EXIT=2` (unchanged sign), wiring-class-mismatch count **1,191 (unchanged)** — matches row 65's baseline exactly, confirming no regression. Full gate: see `progress.md` cycle `SD31-W5-INTEGRATE-001` for `VERIFY_EXIT` and stage summary. `kanban.md`: `epic-0`/`epic-4`/`epic-6` rows updated honestly — none promoted to `COMPLETE` this wave. |
| 6 | 2026-08-16 | wave `sd31-w6-grind` (agents: `sd31-e4-classwire2`, `sd31-attrib-finish`, `sd31-spell-monster`, `sd31-equip-repair`, `sd31-companion-feat` on their own branches/worktrees, three Opus adversarial reviewers, integration `sd31-w6-integrate`) | `SD31-E4-F1-002` (Gunslinger base chassis + supersession wiring, own worktree); `SD31-ATTRIB-002` (row-68/73 finish + 516-unit further-attribution finding, own worktree); `SD31-E6-F2-004` (monster raw_tokens rung + Ultimate Combat as the spell catalog's 8th book, own worktree); `SD31-E6-F5-004` (equipment mis-citation repair + `corpus_literal_sweep` typed-field cross-check, own worktree); `SD31-E6-F7-001` (companion raw_tokens enrichment + render-readiness report, own worktree); `SD31-D7-PROSE-002` (Decision 7's done-bar extended to `monster_ability` + the 247-unit description under-claim recovery, landed direct to `tranche/11` before this wave's dispatch). Three Opus adversarial reviews (D7-PROSE-002 solo; E4-F1-002+ATTRIB-002 paired; E6-F2-004+E6-F5-004+E6-F7-001 triple) surfacing 22 CONFIRMED findings across all six deliverables, gaming verdicts NOT GAMED on every target, PI verdicts CLEAN except one CONTRACT VIOLATION WITH NO EXPOSURE (E6-F7-001's companion enrichment called neither SD-30 PI contract from its production path); this integration cycle (`SD31-W6-INTEGRATE-001`) merging all 5 remaining branches, fixing the PI finding first, then every other CONFIRMED finding reachable within budget (a cross-lane join-key bug, a character-specific-description-argument leak, an equipment `%CHOICE` leak, a self-caught false positive in that same new check, a stale pinned test inherited from the equipment-repair branch, and an operator-facing ARG=1 over-claim), sizing (not fixing) the flat-magnitude question's true blast radius, discharging a self-identified design-tradeoff deferral once the gate itself proved the exposure live (widened `render_pcgen_desc` to drop `%<KEYWORD>` tokens at the root), the one sanctioned guarded regen (run four times — three to find and fix its own defects before committing), the standing audit, and the wave's full gate | All 5 branches merged onto `tranche/11`, content proven by grep not status. Two of five `progress.md` merges required a from-scratch re-do after catching diff3-scrambled section bodies before committing (not after) — see `progress.md` `SD31-W6-INTEGRATE-001` §1 for the extract-and-splice method used instead. `OPEN-ISSUES.md`'s five-way row-number collision (94-96 claimed by every lane against the shared base) renumbered 96-106 with a new row 99 and every cross-reference fixed in the same commit; rows 107/108 added for the sized flat-magnitude question and the equipment leak-guard fix (108 was briefly logged as a deferred design tradeoff, then superseded in place within the same cycle once the gate proved the deferral wrong). PI finding fixed via TDD (6 tests, 2 mutation proofs, confirmed RED before the fix); the companion join-key fix required tracing a first-cut regression one record deep and correcting to a two-field OR-join (7 CRB races' stamps were the collateral the branch's own suggested one-line fix would have cost); the equipment leak-check self-caught and fixed its own raw-vs-rendered-text false positive (3 units briefly wrongly demoted, corrected before the board figure was ever committed). Guarded regen committed at the fully-fixed tip (`2ae22bdae`, after `554005fcc`'s intermediate content was superseded by the `render_pcgen_desc` fix): board `done` **7,340 → 9,488 (+2,148)**, 19.05% → **24.63%**, denominator unchanged at 38,521, zero stamp loss (after tracing and fixing an initial 12-stamp-loss refusal, not overriding it), second run changes only `generated_at`. Of the +2,148, **257 units are recovered from wave 5's own description-completeness demotion** (real descriptions the old check could not see) and **2,688 are genuinely new real paths built by this wave's lanes and this cycle's own fixes — 0 units regressed off the pre-demotion baseline population**, both re-derived by exact `id`-set diff against the pre-demotion (`37c0e5666`) and post-demotion (`c6c8d3cfe`) tips, not estimated. Standing audit: reachable ceiling unchanged at **98.95%**, same 9 dead-end cells, all Epic-2-owned, `ambiguous` population unchanged at 404. Trap report: `TRAP_EXIT=2` (unchanged sign), wiring-class-mismatch count **1,191 (unchanged)** — matches row 65's baseline exactly, re-verified twice (mid-wave and at the fully-fixed tip). Full gate: see `progress.md` cycle `SD31-W6-INTEGRATE-001` §6 for `VERIFY_EXIT` and stage summary — launched three times: run 1 caught a genuine `root-full` regression inherited from `SD31-E6-F5-004` (a stale pinned test assertion the branch's own gate never reached) plus one expected stale-tree failure from this cycle's own mid-run fix landing; run 2 caught a real, live `%CHOICE` leak reaching the desktop equipment catalog via a pre-existing pinned test, fixed at the root; run 3 at the fully-fixed tip: **`VERIFY_EXIT=0`, `RESULT: PASS`, 23/23 stages green** (root-lib 1894, root-full 6685/563 suites, desktop 448, reach 27, corpus-sweep 23859/0 findings, clippy root:47/desktop:7/0 errors, class-dump 31/31). Baseline floors raised to the final measured actuals in a separate DoD-item-7 commit (`d2ff2963f`). `kanban.md`: `epic-0`/`epic-4`/`epic-6` rows updated honestly — none promoted to `COMPLETE` this wave. |
| 7 | 2026-08-16 | wave `sd31-w7-grind` (agents: `sd31/dissolve-core-essentials`, `sd31/d10-supersession-register`, `sd31/classwire3-e4f1-003`, `sd31-spell-racetrait-e6-f2-005`, `sd31/feat-equip-class-e6-f8-001` on their own branches/worktrees, adversarial reviewers, integration `sd31-w7-integrate`) | Five lanes: `SD31-D9-DISSOLVE-001` (core_essentials re-attribution, source-line-aware SOURCELONG scan); `SD31-D10-REGISTER-001` (Supersession Register, 117 objects proposed); `SD31-E4-F1-003` (Ninja base chassis + Scout archetype supersession); `SD31-E6-F2-005` (660 new spell records across 4 books, 93 units held->done); `SD31-E6-F8-001` (15 ce_feats.lst gap rows reachable via RuleSetId::Ce). Adversarial review found 8 CONFIRMED findings across all five: 2 PI-contract gaps (raw_tokens never screened; gen_feat_gap_tables.rs missing the declared-PI reader), 2 denominator-gate defects (the Supersession Register's oracle re-derivation was dead code -- 3 planted fabrication mutations all passed pre-fix; one bad entry, companion corpus_key "1"), 2 prose done-bar over-claims (2 class_feature units missed by the wave-6 hand-check; 7 of 11 banked ce_feats units carrying an undischarged flat magnitude), plus a missing VERIFY_EXIT and a missing DoD-8 on the feat-equip-class branch; this integration cycle (`SD31-W7-INTEGRATE-001`) merged all five branches in the mandated dissolution-first order, fixed every CONFIRMED finding in strict precedence order (PI first, denominator second), the one sanctioned guarded regen, the standing audit, the full gate, and DoD-8 | All 5 branches merged onto `tranche/11`, content proven by grep not status (`progress.md` `SD31-W7-INTEGRATE-001` §1). `OPEN-ISSUES.md`'s row-number collisions (each lane independently landing at 110-113 against a shifting base, one auto-merged clean and STILL collided, caught by an explicit post-merge duplicate-number sweep rather than by a conflict marker) renumbered 112-124. **PI findings fixed first** (commit `7c0398f9a`): both `enrich_spell_raw_tokens.rs` and `gen_feat_gap_tables.rs` now run both SD-30 contracts; 0 exposure before or after, the gap was in the guard. **Denominator findings fixed second** (commit `247b32dba`): the Supersession Register's gate no longer falls back to a cached `raw_lines` guess -- a missing citation is now a hard violation; all 3 of the review's own fabrication mutations now exit 1 where they exited 0 before; the bad `"1"` entry is gone (117->116 objects, 135->134 redundant units); register left PROPOSED, not wired into the live denominator. Prose done-bar corrections (commit `8a3ad0cb0`): `CLASS_FEATURE_FLAT_MAGNITUDE_PENDING_RULING` +2, new `FEAT_FLAT_MAGNITUDE_PENDING_RULING` (7 entries) closing the ce_feats PROXY WARNING gap. Guarded regen committed (`1306d3d0c`): board `done` **9,488 -> 9,780 (+292)**, 24.63% -> **25.39%**, denominator unchanged at 38,521, zero units regressed off `done`. First run refused on a 46-unit apparent stamp loss, traced to a genuine id-rename side effect of this wave's own D9-dissolve fix (core_essentials:-prefixed ids 1,610->128 corpus-wide) rather than real content loss, all 46 verified not sampled; second and third runs byte-identical modulo `generated_at`. Standing audit: reachable ceiling unchanged at **98.95%**, same 9 dead-end cells, all Epic-2-owned. Trap report: `TRAP_EXIT=2` (unchanged sign), wiring-class-mismatch count **1,192 (not 1,191 -- the mandate's own stated figure is stale; row 65's own correction is what reproduces)**, unchanged from wave 6. Full gate: **`VERIFY_EXIT=0`, `RESULT: PASS`, 25/25 stages green** (root-lib 1,909, root-full 6,741/564 suites, desktop 455, reach 27 with a claim, corpus-sweep 24,519/0 findings, supersession-gate 116 objects all clean, frontend 99/99, clippy root:47/desktop:7 warnings/0 errors, class-dump 31/31); baselines raised in a separate commit (`da9bed2dd`) per DoD item 7. DoD-8: real corpus BENEFIT text for Awesome Blow and Multiattack rendered live on the Feat Catalog picker, byte-matching the pinned oracle, committed as screenshots. `kanban.md`: wave-7 status section appended, naming Ninja's `modelled_class_books()` blocker and the register's not-yet-applied status honestly -- no epic promoted to `COMPLETE` this wave. ~122GB reclaimed at cycle close. |
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


### Board after wave 4 (measured at `37c0e5666`, guarded regen committed)

| figure | after wave 3 | after wave 4 | delta |
|---|---:|---:|---:|
| `done` / 38,521 | 7,355 (19.09 %) | **7,603 (19.74 %)** | **+248** |
| reachable ceiling | 98.95 % | **98.95 %** | unchanged |
| `ambiguous` dead-end population | 404 | **404** | unchanged |
| `class_feature` done | 25 | **39** | +14 |
| `equipment` done | 3,904 | **4,022** | +118 |
| `equipment_modifier` done | 917 | **920** | +3 |
| `race` done | 0 | **7** | +7 |
| `race_trait` done | 484 | **489** | +5 |
| `spell` done | 56 | **157** | +101 |
| `held` | 5,596 | **5,596** | unchanged |
| `not-started` | 20,546 | **20,277** | −269 |

Every kind's own delta reconciles exactly with its landing lane: `class_feature` +14 (the class_feature
cache-gen lever, corrected to exclude the 14 PI-exposed records this integration cycle deleted and
re-verified against 0 new exposure — `class_feature` is the ONLY kind change from that lane, confirming
wave 3's finding that a corpus dump cannot manufacture grounding for a kind the engine does not already
ground); `equipment`/`equipment_modifier` +118/+3 (equipment_gap's 697 new records); `race`/`race_trait`
+7/+5 (sweep-attrib's `corpus_literal_sweep` book-attribution fix, off zero for `race`); `spell` +101
(spell-reach's Ultimate Magic ingest + 881-record citation repair). `monster`, `companion`, `feat`,
`monster_ability`, `class` are all UNCHANGED — the monster-widen lane's own receipt honestly reports
`units_moved_to_done: 0` (it wired a production caller and widened `MonsterStatBlock`, real capability
work, but landed no new fixture this cycle), and this integration cycle's own PI/fidelity fixes moved
zero units to `done` by design (deletions and redactions never create `done`). `held` is IDENTICAL
before and after at the total level (5,596 both), a coincidence of offsetting per-kind movement, not a
stall — every kind that gained `done` lost the identical count from either `held` or `not-started`.
Full per-kind table and every command: `progress.md` cycle `SD31-W4-INTEGRATE-001`.

### Board after wave 7 (measured at this cycle's guarded-regen commit, `SD31-W7-INTEGRATE-001`)

| figure | after wave 6 | after wave 7 | delta |
|---|---:|---:|---:|
| `done` / 38,521 | 9,488 (24.63 %) | **9,780 (25.39 %)** | **+292** |
| reachable ceiling | 98.95 % | **98.95 %** | unchanged |
| `ambiguous` dead-end population | 404 | **404** | unchanged |
| `class` done | 27 | **27** | unchanged |
| `class_feature` done | 39 | **69** | **+30** |
| `companion` done | 515 | **680** | **+165** |
| `equipment` done | 4,513 | **4,513** | unchanged |
| `equipment_modifier` done | 228 | **228** | unchanged |
| `feat` done | 1,165 | **1,169** | **+4** |
| `monster` done | 840 | **840** | unchanged |
| `monster_ability` done | 1,365 | **1,365** | unchanged |
| `race` done | 7 | **7** | unchanged |
| `race_trait` done | 630 | **630** | unchanged |
| `spell` done | 159 | **252** | **+93** |
| `held` | 3,193 | **2,912** | **−281** |
| `not-started` | 19,915 | **19,900** | **−15** |
| `unmeasurable` | 5,123 | **5,127** | **+4** |
| `in-progress` | 766 | **766** | unchanged |
| `deferred` | 36 | **36** | unchanged |

+292 = class_feature +30, companion +165, feat +4, spell +93 — exactly the five merged lanes' own
claimed movement, net of this integration cycle's own two done-bar corrections (class_feature 32→30,
feat 11→4). Zero units regressed off `done` (verified by full `id`-set diff, not sampled). Denominator
unchanged at 38,521 — see §6 for the Supersession Register's proposed-but-not-applied effect.

### Board after wave 6 (measured at this cycle's guarded-regen commit, `SD31-W6-INTEGRATE-001`)

| figure | after wave 5 | after wave 6 | delta |
|---|---:|---:|---:|
| `done` / 38,521 | 7,340 (19.05 %) | **9,488 (24.63 %)** | **+2,148** |
| reachable ceiling | 98.95 % | **98.95 %** | unchanged |
| `ambiguous` dead-end population | 404 | **404** | unchanged |
| `class` done | 27 | **27** | unchanged |
| `class_feature` done | 39 | **39** | unchanged |
| `companion` done | 416 | **515** | **+99** |
| `equipment` done | 4,364 | **4,513** | **+149** |
| `equipment_modifier` done | 84 | **228** | **+144** |
| `feat` done | 1,165 | **1,165** | unchanged |
| `monster` done | 14 | **840** | **+826** |
| `monster_ability` done | 438 | **1,365** | **+927** |
| `race` done | 7 | **7** | unchanged |
| `race_trait` done | 630 | **630** | unchanged |
| `spell` done | 156 | **159** | **+3** |
| `held` | 4,936 | **3,193** | **−1,743** |
| `not-started` | 20,061 | **19,915** | **−146** |
| `unmeasurable` | 5,381 | **5,123** | **−258** |
| `in-progress` | 767 | **766** | **−1** |
| `deferred` | 36 | **36** | unchanged |

**This wave's headline is the largest single-wave gain of the program to date, and it is split two ways
per the standing "don't blur a recovery with a demotion" rule.** Of the +2,148: **257 units are
recovered from wave 5's own anti-gaming description-completeness demotion** (144 `equipment_modifier` +
112 `equipment` + 1 `spell` — real descriptions the old raw-`.lst`-closure-only check could not see,
recovered by `SD31-D7-PROSE-002`'s second-source rung, which landed direct to `tranche/11` between
waves 5 and 6). **2,688 are genuinely new real paths that did not exist even before the demotion**
(1,029 `monster_ability` + 826 `monster` + 591 `equipment` + 141 `race_trait` + 99 `companion` + 2
`spell` — `SD31-D7-PROSE-002`'s `monster_ability` rung extension, `SD31-E6-F2-004`'s monster
`raw_tokens` enrichment, `SD31-E6-F7-001`'s companion `raw_tokens` enrichment plus this integration
cycle's own join-key fix that unlocked 34 of them, and the ordinary residue of prior waves' ingest
lanes reaching `done` for the first time as their own supporting fixes landed). **Zero units regressed
off the pre-demotion baseline population of 6,543** — re-derived by exact `id`-set arithmetic against
the pre-demotion (`37c0e5666`, wave 4's own commit) and post-demotion (`c6c8d3cfe`, wave 5's own
commit) tips, not estimated: `6,543 (baseline, unchanged) + 257 (recovered) + 2,688 (new) = 9,488`,
exact. `class`/`class_feature`/`race`/`race_trait`/`feat` are all UNCHANGED this wave — none of the
five merged lanes touched those kinds' doneness paths, and this integration cycle's own fixes were
scoped to `monster_ability`/`equipment`/`equipment_modifier`/`companion`. Full per-kind table, every
command, and the guarded-regen trace (including the mid-cycle stamp-loss refusal this wave traced and
fixed rather than overrode): `progress.md` cycle `SD31-W6-INTEGRATE-001`.

### Board after wave 5 (measured at this cycle's guarded-regen commit, `SD31-W5-INTEGRATE-001`)

| figure | after wave 4 | after wave 5 | delta |
|---|---:|---:|---:|
| `done` / 38,521 | 7,603 (19.74 %) | **7,340 (19.05 %)** | **−263** |
| reachable ceiling | 98.95 % | **98.95 %** | unchanged |
| `ambiguous` dead-end population | 404 | **404** | unchanged |
| `class` done | 27 | **27** | unchanged |
| `class_feature` done | 39 | **39** | unchanged |
| `companion` done | 416 | **416** | unchanged |
| `equipment` done | 4,022 | **4,364** | **+342** |
| `equipment_modifier` done | 920 | **84** | **−836** |
| `feat` done | 1,176 | **1,165** | **−11** |
| `monster` done | 14 | **14** | unchanged |
| `monster_ability` done | 336 | **438** | **+102** |
| `race` done | 7 | **7** | unchanged |
| `race_trait` done | 489 | **630** | **+141** |
| `spell` done | 157 | **156** | **−1** |
| `held` | 5,596 | **4,936** | **−660** |
| `not-started` | 20,277 | **20,061** | **−216** |
| `unmeasurable` | 4,223 | **5,381** | **+1,158** |
| `in-progress` | 786 | **767** | **−19** |
| `deferred` | 36 | **36** | unchanged |

This wave's headline is a NET figure with two forces pulling opposite directions, both real. **Pulling
down:** `SD31-D7-PROSE-001`'s own anti-gaming description-completeness fix (`closure_has_real_description`
gating every `text_only`->`text-complete` branch, landed on `tranche/11` before this wave started but
only now visible against a fully-merged, cross-checked tip) demotes any unit whose corpus record has
no real prose to show a player — `equipment_modifier` -836 is nearly all of it (836 of 1,060 pre-existing
false promotions this fix targets were `equipment_modifier`, per that cycle's own row 70/`OPEN-ISSUES`
breakdown), `feat` -11 and `spell` -1 the rest; most of the demoted units land in `unmeasurable`, which
explains +1,158 there. **Pulling up:** this wave's real ingest work — `equipment` +342 (net of both
`SD31-E6-F5-003`'s 620 new records AND the SAME description-completeness fix demoting some of
`equipment`'s own pre-existing false promotions, so the gross new-record gain is larger than the net),
`monster_ability` +102 (`SD31-E6-F9-001`'s raw_tokens enrichment, reproduces the lane's own figure
exactly), `race_trait` +141 (`SD31-D7-PROSE-001`'s new zero-magnitude rung, 146 promotions net of this
integration cycle's own PI-redaction-placeholder fix, -5). `class`/`class_feature`/`companion`/`monster`/
`race` are all UNCHANGED — `SD31-E4-F1-001`'s real Slayer wiring lands on `held`, not `done` (declined to
flip the `display`+`grounded`->`held` cell, per its own adversarial-review-confirmed anti-gaming
discipline), and `SD31-ATTRIB-001`'s 1,610-unit relabel is a pure reporting-field move with zero
`doneness_verdict` transitions by construction (re-verified independently by two Opus reviewers).
Full per-kind table, every command, and the stamp-loss trace: `progress.md` cycle `SD31-W5-INTEGRATE-001`.

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

| 8 | 2026-08-16 | wave `sd31-w8-grind` (agents: `sd31/classwire4-e4f1-004`, `sd31/racetrait/SD31-E6-F4-002`, `sd31/equipmod-e6f6-001`, `sd31/spell-held-SD31-E6-F2-006`, `sd31/attrib-evidence-003` on their own branches/worktrees, three Opus adversarial reviewers, integration `sd31-w8-integrate`) | Five lanes: `SD31-E4-F1-004` (Samurai base chassis + Challenge/Resolve/Bonus-Feat wiring, third and final UC class); `SD31-E6-F4-002` (Advanced Race Guide 6-race chassis batch — Catfolk/Kitsune/Ratfolk/Strix/Suli/Wayang); `SD31-E6-F6-001` (equipment_modifier `.COPY=` inheritance recovery, 338 raw credit); `SD31-E6-F2-006` (spell caster-level-linear DURATION seam, 898 fixture-verified promotions); `SD31-ATTRIB-003` (per-race citation evidence table + site-dashboard `--check` gate). Three Opus adversarial reviews returned **one GAMED verdict** (`SD31-E6-F6-001`, confined to the `.COPY=`-vs-`wiring_class.rs` gap) and **one FAIL PI verdict** (`SD31-ATTRIB-003`, the `site/dashboard/units/*.json` shard exposure) across 15 CONFIRMED findings; this integration cycle fixed the GAMED gap and the confirmed FAIL at the source, fixed 3 more CONFIRMED findings (the equipment universal-modifier gate, the %-hole leak at both the detector and render path), corrected 2 receipts, and DISCOVERED one further, PRE-EXISTING PI exposure of its own (the public dashboard's manifest/roadmap content) while performing the mandated site-dashboard publish step | All 5 branches merged onto `tranche/11` in the dispatched order, content proven by grep not status (`progress.md` `SD31-W8-INTEGRATE-001` §1). `OPEN-ISSUES.md` row-number collisions renumbered 130-140 across all five merges, checked for duplicates BY NUMBER after every single merge. **PI fixed first**: `site/dashboard/units/` (261 NAMEISPI:YES names) not committed at all (commit `89b4dbba6`); two confirmed `--check`-gate bugs fixed TDD/mutation-proven in the same commit. **GAMED fixed second, at the source**: `wiring_class::build_copy_base_index` added and threaded through every `token_closure_rows` call site (commit `4fc65df96`) — mutation-proven; its scope, being corpus-wide rather than `equipment_modifier`-scoped, ALSO corrected an independent pre-existing gaming shape on plain `equipment` (−141 net, no merged lane's credit) and 2 `spell` `.COPY=` records, fully traced by exact trap-report set-diff (34 new mismatches, 0 unexplained, `OPEN-ISSUES.md` row 148). Two more confirmed findings fixed (commits `5613a53d9`/`2851dd335`/`5dd42c5b3`): the bare-`%` leak, at BOTH the detector and (found only by re-running the equipment catalog's own tests after the detector-only fix) the render path; the universal-vs-conditional gate's missing `Kind::Equipment` arm. Guarded regen committed (`70da6356b`): board `done` **9,780 → 10,759 (+979)**, 25.39% → **27.93%**, denominator unchanged at 38,521, zero stamp loss, second run byte-identical modulo `generated_at`. Standing audit: reachable ceiling unchanged at **98.95%** (38,115/38,521), same 9 dead-end cells, all Epic-2-owned. Trap report: `TRAP_EXIT=2` (unchanged sign), wiring-class-mismatch **1,191 → 1,225 (+34, fully traced, see above)**. Public feed republished (`4d1770933`) — `site/dashboard/units/` again deliberately not committed; a SECOND, pre-existing PI exposure in the top-level feed's own manifest content discovered and logged (`OPEN-ISSUES.md` row 149, RULING-NEEDED, PRECEDENCE-1 — now the single most urgent open item in the package). Full gate: see `progress.md` cycle `SD31-W8-INTEGRATE-001` for `VERIFY_EXIT` and stage summary. `kanban.md`: wave-8 status section appended, naming `UNIVERSAL_MODIFIER_CUES`' recall gap as the next largest lever — no epic promoted to `COMPLETE` this wave. |

| 9 | 2026-08-16/17 | wave `sd31-w9-grind` (agents: `sd31/e4-classsplit-wire5`, `sd31/racetrait2-SD31-E6-F4-003`, `sd31/feat-companion-e6-f8-002`, `sd31/monster2-SD31-E6-F9-002`, `sd31/equip-class-SD31-E6-F10-001` on their own branches, three Opus adversarial reviewers, integration `sd31-w9-integrate`) | Five branches: `SD31-E4-F1-005` (`pilot_compute.rs` -> per-class-module split for Slayer/Ultimate Combat + owed universal size-bonus wiring, own worktree); `SD31-E6-F4-003` (ARG's 6-race alternate-trait batch, 24 records, own worktree); `SD31-E6-F8-002` (feat/companion gap-table generation, 242-unit held-mass trace, own worktree); `SD31-E6-F9-002` (`transcribe_monster_tables.py` truncate-before-compute fix, script-only, own worktree); `SD31-E6-F10-001` (Inner Sea Gods spell book onboard, 92 records, own worktree). Three Opus adversarial reviews (split+racetrait paired; feat-companion solo; monster2+equip-class paired) surfacing 23 CONFIRMED findings, gaming verdicts NOT GAMED on every target (with real over-claims to correct, not a rule moved to admit more cells), PI verdicts FAILED on ONE target (equip-class: one verbatim deity-name-typo exposure) and CLEAN on the other four; this integration cycle (`SD31-W9-INTEGRATE-001`) merging all 5 branches (the `pilot_compute` split FIRST, confirming zero board-verdict movement attributable to the move itself before merging anything else), fixing the PI exposure first per precedence, then a pre-existing (wave-8, not this wave's) `decisions.md §10` AMENDMENT gap the class_feature explanation-id matcher had, wiring Strix's 4 disclosed-but-unwired alternate-trait bonuses per §8, and the feat rung's placeholder-served-description gap, plus 2 cosmetic shipped-prose-drift fixes, the one sanctioned guarded regen, the standing audit, and the wave's full gate | All 5 branches merged onto `tranche/11`, content proven by `git log origin/tranche/11..<branch>` per branch, not status. `OPEN-ISSUES.md`'s five-way row-number collisions (every lane independently appended starting at 150 against a shared base) renumbered sequentially 150-162 with zero duplicates by number, plus rows 163-166 appended for this cycle's own fixes; "Needs an operator ruling" refreshed with an explicit pointer to the live one (row 140, race attribution). **PI fixed first**: `pick_your_poison.json`'s `Cayden CaiLean` deity-name-typo exposure (oracle's own miscapitalization surviving the exact-substring blacklist scan) and its weaker `abstemiousness.json` "lrori" sibling, both closed at the source (`pi_screening::PI_BLACKLIST_TERMS` 55->57) and re-verified 0 occurrences corpus-wide. **class_feature matcher fixed**: the exact-suffix explanation-id branch had no `group == owner` guard (unlike its sibling fallback) and matched on trailing SUBSTRING not trailing dot-SEGMENT — closed 3 confirmed defects (16 archetype/Unchained-variant units credited off a base class's id, `Bloodrager ~ Raging` off a NEGATION explanation, `Hunter ~ Animal Focus`'s evidence naming the wrong record), `class_feature` `done` corrected 82->73 (-9) as the honest, intended consequence. **Strix's 4 alternate traits wired** to `pilot_compute.rs` (values re-derived byte-faithful against the oracle), closing the lane's own honestly-disclosed, left-RED test gap — no board movement (already `done` via a load-only probe, now legitimately so). **Feat rung fixed**: 9 records (7 `[redacted PI]`, 2 PCGen's own `[NOT IMPLEMENTED]` marker) demoted off `done` to `unmeasurable` for serving a placeholder marker instead of prose, the identical defect wave 5 already fixed for `race_trait`, ported to feat. Guarded regen committed: board `done` **10,759 -> 10,958 (+199)**, 27.9302% -> **28.4468%**, denominator unchanged at 38,521; 6 stamp losses (all pre-existing §10-violating `pathfinder_unchained`/`ultimate_combat` `class_feature` units) traced one record deep and confirmed as the correct, intended demotion, `--allow-stamp-loss` used once. Standing audit: reachable ceiling unchanged at **98.95%**, 9 dead-end cells unchanged, all `ambiguous`-owned. Trap report: `TRAP_EXIT=2` (unchanged sign), baseline reproduces exactly at **1 mod-record, 1,225 wiring-class-mismatch** (NOT the stale 1,191 the dispatch prompt quoted — that figure was wave-5/6/7's, wave 8 moved it deliberately and traced all 34; the dispatch prompt's own baseline was stale, corrected here — the fourth orchestrator figure error this package has recorded). Full gate: see `progress.md` cycle `SD31-W9-INTEGRATE-001` for `VERIFY_EXIT` and stage summary (launched three times: run 1 stale after this cycle's own source fixes landed, deliberately killed rather than trusted; run 2 caught a real `sd24_wired_integration_audit` false-positive this cycle's own new `reason` string prose tripped, fixed and re-verified; run 3 is the tip this receipt reports against). `kanban.md`: `epic-4-mechanism`/`epic-6-ingest-lanes` rows updated honestly with the wave's own board-delta figures and owning OPEN-ISSUES rows for what remains open — neither promoted to `COMPLETE` this wave. |

| 10 | 2026-08-17 | wave `sd31-w10-grind` (agents: `sd31/e4-f2-chooser-SD31-E4-F2-001`, `sd31/e5-f1-003-inventory-gaps`, `sd31/racetrait3-SD31-E6-F4-004`, `sd31/mab-companion-SD31-E6-F9-003`, `sd31/equip-class2-SD31-E6-F10-002`, `sd31/spell-feat-e6-f2-007` on their own branches, two Opus adversarial review passes, integration `sd31-w10-integrate`) | Six branches: `SD31-E4-F2-001` (the chooser-primitive proof-of-concept, `archetype_resolver::chooser_option_selected` wired to Oracle Battle Mystery/Battlecry, own worktree); `SD31-E5-F1-003` (`class_feature` registry widening — `PuClassId` registered in `modelled_class_books()`, two owner/diagnostic matcher bugs fixed, own worktree); `SD31-E6-F4-004` (ARG follow-on 4-race chassis batch — Gillman/Nagaji/Vanara/Vishkanya, own worktree); `SD31-E6-F9-003` (monster `SLA_CL` literal-override + `.key` resolution fix, own worktree); `SD31-E6-F10-002` (`core_essentials`->`bestiary` equipment re-attribution, own worktree); `SD31-E6-F2-007` (Mythic Adventures onboarded as the feat catalog's 18th book, own worktree). Two Opus adversarial review passes surfacing **16 CONFIRMED findings, zero GAMED, zero PI FAIL**, across all six lanes; this integration cycle (`SD31-W10-INTEGRATE-001`) merged all six in the mandated interaction order (chooser then inventory-gaps first, since they interact), fixed the two bulk-admits-units findings before the regen ran, logged the other 14 to `OPEN-ISSUES.md` rows 180-185 with remedy and owning epic, corrected 3 lane receipts' own re-derivable figures, the one sanctioned guarded regen, the standing audit + public feed, and the wave's full gate | All 6 branches merged onto `tranche/11`, content proven by `git log origin/tranche/11..<branch>` per branch, not status. `OPEN-ISSUES.md`'s six-way row-number collision (every lane independently appended starting at 167 against a shared base) renumbered sequentially 167-179 with zero duplicates by number, plus rows 180-185 appended for this cycle's own findings; "Needs an operator ruling" refreshed with a one-line-per-item quick-reference index. **Fixed before the regen ran, precedence-3 (admits units in bulk)**: the chooser lane's Battlecry duration grounded the raw Charisma SCORE (18) instead of the MODIFIER (4) it should — fixed at `pilot_compute/mod.rs`, test corrected 18->4 (`116fbbed0`), sequenced strictly before the inventory-gaps matcher fix that makes the unit probe-reachable; and 159 of the spell+feat lane's 358 ingested Mythic feat rows were PCGen `VISIBLE:EXPORT` display-plumbing twins served as ungated, independently-selectable duplicate feats (142 already `done`, 40% of the wave's original headline) — fixed at the generator (skip `VISIBLE:EXPORT` rows), confirmed present in the merge commit (`4750084bf`). The remaining 14 findings (guard-reachability, a `slug()` apostrophe under-credit, a test-hardening gap, two book-filing/directory-naming splits, a superseded DoD-1/2) logged as `OPEN-ISSUES.md` rows 180-185, none denominator- or gaming-shaped — none rushed under this cycle's own merge/regen/gate time budget. Guarded regen committed (`f3b74c51a`): board `done` **10,958 -> 11,229 (+271)**, 28.4468% -> **29.1503%**, denominator unchanged at 38,521, zero stamp loss (406 total id-level moves, all traced by direction — real credit vs. honest correction — not blurred). Standing audit: reachable ceiling unchanged at **98.946%** (98.95%), same 9 dead-end cells (406 units), all `ambiguous`-owned. Trap report (re-run fresh this cycle, `SD31-W10-INTEGRATE-001-trap-report.log`): `TRAP_EXIT=2` (unchanged sign), **1 mod-record / 1,225 wiring-class-mismatch — byte-identical to wave 9's own baseline**, confirmed not worsened. Public feed republished (`f307afcf8`) — `site/dashboard/units/` again deliberately not committed; the top-level feed's own row-149 exposure spot-checked and confirmed UNCHANGED (not worsened), still awaiting the operator's redaction ruling. Full gate: launched twice — first attempt correctly failed `site-dashboard-check` on a genuine cycle-timing self-inflict (regen+publish landed on top of the merged tip mid-gate, not the pre-existing row-153 defect); second, truly-final-tip run **`VERIFY_EXIT=0`, `RESULT: PASS`, 27/27 stages green** (root-lib 1,968, root-full 6,910, desktop unchanged, reach 27/27 with a claim, corpus-sweep 24,741/0 findings, supersession-gate 116 objects clean, frontend 99/99, clippy root:52/desktop:7/0 errors — AT ceiling not over it, class-dump 31/31); baselines reconciled in a separate commit (`46af650b6`) with `--show-actuals`. `kanban.md`: "Wave 10 integration status" section appended, stating plainly that the chooser primitive is real, reachable production code (confirmed non-test-only call chain to the desktop entry point) but currently unlocks only ONE consumer with a statically-unreachable safety guard and no player-facing picker yet — no epic promoted to `COMPLETE` this wave. |
| 11 | 2026-08-17 | wave `sd31-w11-grind` (agents: `sd31/e5-f1-004-pool-match`, `sd31/pools-wire-SD31-E4-F2-002`, race lane dispatched `null` (nothing to merge), `sd31-mab-comp2/SD31-E6-F9-004`, `sd31/equip-class3-SD31-E6-F10-003`, `sd31/spell-feat2-SD31-E6-F2-008` on their own branches, three Opus adversarial review passes, integration `sd31-w11-integrate`) | Five real branches: `SD31-E5-F1-004` (`CLASS_FEATURE_POOLS` pool-name matching + `slug()` apostrophe handling, closing row 168/181, own worktree); `SD31-E4-F2-002` (4 more Oracle Mystery pools wired through `chooser_option_selected` + Mystery-picker Path A seed + DoD-8, own worktree); `SD31-E6-F9-004` (monster_ability/companion root-cause traces, own worktree); `SD31-E6-F10-003` (equipment gap widen to 8 more books, own worktree); `SD31-E6-F2-008` (spell RANGE caster-level formula seam, own worktree). Three Opus adversarial review passes surfacing **16 CONFIRMED findings, zero GAMED, one MIXED PI verdict** (a pre-existing cross-book declared-PI propagation on `Elysian Shield`, not introduced this wave) across all five lanes; this integration cycle (`SD31-W11-INTEGRATE-001`) merged all five in the mandated order (the two class_feature lanes first, since they interact), fixed the one precedence-3 finding (the pool matcher's same-class-slot bulk-credit defect) before the regen ran, logged the other 15 to `OPEN-ISSUES.md` rows 195-203 with remedy and owning epic, the one sanctioned guarded regen, the standing audit + public feed, the wave's full gate, and DoD-8 | All 5 real branches merged onto `tranche/11`, content proven by `git log origin/tranche/11..<branch>` per branch, not status; the race lane's absence confirmed (both prior race branches 0 commits ahead). `OPEN-ISSUES.md`'s repeated row-186 collisions (each of the last four lanes independently landed a row 186 against a shifting base) renumbered sequentially 186-194 with zero duplicates by number across four separate merge conflicts, plus rows 195-203 appended for this cycle's own findings; "Needs an operator ruling" refreshed, adding row 197 (Elysian Shield cross-book PI). **Fixed before the regen ran, precedence-3 (admits units in bulk)**: 20 of the pool-match lane's own 38 credited `class_feature` units (`Shaman Wandering Spirit ~ *` / `Secondary Shaman Wandering Spirit ~ *`) were grounded on a byte-identical computation to the DIFFERENT corpus record `Shaman Spirit ~ *` — the matcher's two existing guards were cross-CLASS-scoped only and structurally blind to a same-class DIFFERENT-SLOT collision (~533 units corpus-wide exposed, including 44 `Unchained Rogue Talent` units decisions.md §10's own AMENDMENT forbids); fixed with a third guard (`CLASS_FEATURE_POOL_SLOT_QUALIFIERS`) plus a permanent regression test (`9cd7144bb`), landed BEFORE the regen ran so the wrong 20 never reached committed board state. The remaining 15 findings (a receipt-figure correction, a RULING-NEEDED cross-book PI item, an inherited negative-duration screenshot, two overstated monster_ability/companion root-cause traces, a tautological spell-seam mutation test, an equipment receipt-figure correction, and lane A's absence) logged as `OPEN-ISSUES.md` rows 196/197/198/199/200/201/202/203, none denominator- or gaming-shaped. Guarded regen committed (`65400ddc9`): board `done` **11,229 -> 11,828 (+599)**, 29.1503% -> **30.7053%**, denominator unchanged at 38,521, zero stamp loss (38,540 raw ids identical, full id-set diff), zero `done`->non-`done` regressions anywhere (one honest `in-progress`->`not-started` demotion, the Elysian Shield PI fix, never touched `done`). Standing audit: reachable ceiling unchanged at **98.95%** (38,115/38,521), same 9 dead-end cells, all `ambiguous`-owned. Trap report (re-run fresh this cycle, `SD31-W11-INTEGRATE-001-trap-report.log`): `TRAP_EXIT=2` (unchanged sign), **1 mod-record / 1,225 wiring-class-mismatch — byte-identical to wave 10's own baseline**, confirmed not worsened. Public feed republished (`75dd74dbb`) — `site/dashboard/units/` again deliberately not committed; the top-level feed's own row-149 exposure re-checked directly (7 named strings all still present) and disclosed explicitly in the receipt rather than quietly published. Full gate: launched early, kept alive through the whole cycle, **`VERIFY_EXIT=0`, `RESULT: PASS`, 27/27 stages green** (root-lib 1,984, root-full 6,945/565 suites, desktop 459, reach 27/27, corpus-sweep 25,163/0 findings, supersession-gate 116 objects clean, frontend 99/99, clippy root:52/desktop:7/0 errors — AT ceiling not over it, class-dump 31/31); baselines reconciled in a separate commit (`aa6e7b3ef`) with `--show-actuals`. DoD-8: a fresh Dwarf Shaman 1's Actions tab shows exactly ONE Spirit slot rendering (Life Spirit Channel Uses/Dice/DC), no `Wandering Spirit` duplicate anywhere — direct on-screen confirmation of the withdrawn-20-units finding, committed as `artifacts/SD31-W11-INTEGRATE-001/shaman-actions-tab.png`. `kanban.md`: "Wave 11 integration status" section appended, stating plainly that the chooser primitive now has 5 wired pools (not 1) but the pool-matcher fix mostly unblocked a different, smaller 18-unit slice (Shaman/Witch/Sorcerer) than the mandate's own ~1,974-unit expectation — no epic promoted to `COMPLETE` this wave. |

| 12 | 2026-08-17 | wave `sd31-w12-grind` (agents: `sd31/pool-consumers/SD31-E4-F2-003`, `sd31/feat-matcher-SD31-E6-F8-003`, `sd31/racetrait5-SD31-E6-F4-006`, `worktree-wf_091c1ff2-4bf-3` (transcription, `SD31-E6-F9-005`), `sd31/equip-class4/SD31-E6-F10-004`, `sd31/spell3-E6-F2-009` on their own branches/worktrees, integration `sd31-w12-integrate`) | Six real lanes (no null lane this wave): `SD31-E4-F2-003` (Barbarian Superstition rage power wired as a real class_feature pool consumer-delta representative); `SD31-E6-F8-003` (`mod_only_rescue` 249-unit phantom-duplicate root-cause trace + 2-book feat gap lane); `SD31-E6-F4-006` (11 new race_trait records across 4 races + the load-only-evidence audit the mandate's own prompt cited verbatim); `SD31-E6-F9-005` (168 new monster_ability records across beastiary/bestiary_2, a wholesale-regen data-loss hazard fix, a `WiringClassIndex` fix); `SD31-E6-F10-004` (5 more equipment books, 481 new records, a per-record blacklist pre-filter); `SD31-E6-F2-009` (2 tautological mutation-proof test fixes, a 14-unit spell `.COPY=` ingest gap characterized). This integration cycle (`SD31-W12-INTEGRATE-001`) merged all six in the mandated order (the two class_feature lanes first, since they interact), fixed a confirmed PI exposure first (precedence 1), then a confirmed race_trait credited-on-insufficient-evidence defect (precedence 2), the one sanctioned guarded regen, the standing audit + public feed, and the wave's full gate | All 6 real branches merged onto `tranche/11`, content proven by `git log origin/tranche/11..<branch>` per branch, not status. `OPEN-ISSUES.md`'s row-204 collision (all four remaining lanes independently landed rows there against the shifting base) renumbered sequentially 204-218 with zero duplicates by number across five separate merge conflicts, plus rows 219-221 appended for this cycle's own findings. **PI EXPOSURE FIXED, precedence 1**: `enrich_equipment_raw_tokens.rs` (the only writer of shipped `raw_tokens`) had NO PI screening at all — 28 `inner_sea_gods` records shipped a blacklisted deity/place name verbatim in `raw_tokens` while `description` was correctly redacted. Fixed in the production path (both SD-30 contracts now run on every token value), the sibling "Mutation proof" test in `gen_equipment_gap_tables.rs` confirmed unable to fail and rewired to drive the real production function (mutation-verified), all 28 already-shipped records remediated in place, `corpus_literal_sweep` re-run clean after the fix (`OPEN-ISSUES.md` row 219). **UNITS-CREDITED-ON-INSUFFICIENT-EVIDENCE FIXED, precedence 2**: the `!universal_sheet_modifier` refusal was checked only inside the `text_only` arm (a real universal magnitude walked past it to unconditional `computed`+`grounded` board `done`), and the load-only race_trait evidence question the mandate's own prompt named verbatim was closed for the unambiguous population — a new, table-derived (not hand-typed) `race_ids_with_a_magnitude_consumer()` check demotes any `computed` race_trait credit whose race has zero engine seam, 262 units net (`OPEN-ISSUES.md` row 221). Guarded regen committed: board `done` **11,828 -> 11,829 (+1 net)**, 30.7053% -> **30.7079%**, denominator unchanged at 38,521, zero stamp loss, second run changes only `generated_at`. The +1 net hides real motion in both directions, fully reconciled by id: `race_trait` -251 (this cycle's own demotion fix), `equipment` +244 (equip-class4), `feat` +5 (feat-matcher), `monster_ability` +3 (transcription lane — this CONFIRMS by live regen, not narrative, the "+3" figure adversarial review flagged as unverified in the dispatch text). Standing audit: reachable ceiling unchanged at **98.95%** (38,115/38,521), same 9 dead-end cells, all `ambiguous`-owned. Trap report: `TRAP_EXIT=2` (unchanged sign), **1 mod-record / 1,225 wiring-class-mismatch — byte-identical to wave 11's own baseline**, confirmed not worsened. Public feed republished — `site/dashboard/units/` again deliberately not committed; row-149's exposure re-checked directly (all 7 sampled names still present) and disclosed explicitly rather than quietly published. Full gate: launched early, kept alive, VERIFY_EXIT and stage summary see `progress.md` cycle `SD31-W12-INTEGRATE-001` §5. `kanban.md`: "Wave 12 integration status" section appended, stating plainly that this wave's real headline is the demotion (a correction, not a setback) landing in the same regen as this wave's real new content — no epic promoted to `COMPLETE` this wave. |
## Board after wave 8 (`SD31-W8-INTEGRATE-001`, 2026-08-16)

Re-derived live with the producer's own `doneness_verdict()` over the committed `docs/work-inventory.json`
(the same command as the baseline table above), not transcribed from any lane's own receipt:

| kind | total | done (wave 7) | done (wave 8) | delta |
|---|---:|---:|---:|---:|
| class | 185 | 27 (14.6%) | 27 (14.6%) | +0 |
| class_feature | 15,472 | 69 (0.4%) | 82 (0.5%) | +13 |
| companion | 1,696 | 680 (40.1%) | 680 (40.1%) | +0 |
| equipment | 6,208 | 4,513 (72.7%) | 4,372 (70.4%) | **−141** |
| equipment_modifier | 1,580 | 228 (14.4%) | 380 (24.1%) | +152 |
| feat | 2,610 | 1,169 (44.8%) | 1,176 (45.1%) | +7 |
| monster | 1,270 | 840 (66.1%) | 840 (66.1%) | +0 |
| monster_ability | 2,951 | 1,365 (46.3%) | 1,366 (46.3%) | +1 |
| race | 103 | 7 (6.8%) | 7 (6.8%) | +0 |
| race_trait | 3,603 | 630 (17.5%) | 680 (18.9%) | +50 |
| spell | 2,843 | 252 (8.9%) | 1,149 (40.4%) | +897 |
| **TOTAL** | **38,521** | **9,780 (25.39%)** | **10,759 (27.93%)** | **+979** |

Denominator unchanged all package (38,521). Reachable ceiling unchanged (98.95%, 38,115/38,521).
`equipment`'s −141 is a demotion this cycle's own `.COPY=` gaming fix caused (§2 above) — it belongs
to no merged lane's credit and predates the whole package; see `progress.md`'s `SD31-W8-INTEGRATE-001`
receipt §3 for the movement separated by cause, not blurred.

## Board after wave 9 (`SD31-W9-INTEGRATE-001`, 2026-08-17)

Re-derived live with the producer's own `doneness_verdict()` over the committed `docs/work-inventory.json`
(the same command as every prior wave's table), not transcribed from any lane's own receipt:

| kind | total | done (wave 8) | done (wave 9) | delta |
|---|---:|---:|---:|---:|
| class | 185 | 27 (14.6%) | 27 (14.6%) | +0 |
| class_feature | 15,472 | 82 (0.5%) | 73 (0.5%) | **−9** |
| companion | 1,696 | 680 (40.1%) | 680 (40.1%) | +0 |
| equipment | 6,208 | 4,372 (70.4%) | 4,372 (70.4%) | +0 |
| equipment_modifier | 1,580 | 380 (24.1%) | 380 (24.1%) | +0 |
| feat | 2,610 | 1,176 (45.1%) | 1,352 (51.8%) | +176 |
| monster | 1,270 | 840 (66.1%) | 840 (66.1%) | +0 |
| monster_ability | 2,951 | 1,366 (46.3%) | 1,366 (46.3%) | +0 |
| race | 103 | 7 (6.8%) | 7 (6.8%) | +0 |
| race_trait | 3,603 | 680 (18.9%) | 704 (19.5%) | +24 |
| spell | 2,843 | 1,149 (40.4%) | 1,157 (40.7%) | +8 |
| **TOTAL** | **38,521** | **10,759 (27.93%)** | **10,958 (28.4468%)** | **+199** |

Denominator unchanged all package (38,521). Reachable ceiling unchanged (98.95%, 38,115/38,521).

**Separating the two directions, not blurring them** (this wave's own binding rule): the `feat`
+176, `race_trait` +24 and `spell` +8 are real new work landing (feat/companion gap-table
generation, ARG's alternate-trait batch, Inner Sea Gods spell ingest). `class_feature`'s **−9** is
the honest, intended consequence of a real check finding a real over-credit — a pre-existing
(wave 8, not this wave's) `decisions.md §10` AMENDMENT gap that let 16 archetype/Unchained-variant
units and `Bloodrager ~ Raging` ground off the wrong explanation id, closed this cycle. Neither
direction is reportable as the other; both are correct.

**What the class_feature grounding analysis found, in one paragraph:** the tool that decides
whether a `class_feature` record counts as "wired" was matching too loosely — it accepted an
engine explanation as proof for a feature whenever the explanation's ID merely ENDED with the
feature's name, and whenever the record's class NAME merely CONTAINED the engine's class name as
a substring. Both looseness let genuinely different things get credited for each other: an
archetype variant (e.g. "Unchained Rogue") got credited off its unrelated BASE class's own feature
of the same name (base "Rogue"), and one record ("Bloodrager ~ Raging", meaning "is the character
currently raging") got credited off an engine fact that means the exact opposite ("is NOT currently
raging"). This wave tightened the match to require the class to match exactly and the explanation's
own final identifying segment to match exactly, not merely contain or end with the right text.
Fixing it correctly LOWERED the board by 9 units for `class_feature` — a drop, not a gain, and the
mandate's own standing rule is that a drop caused by a real check working correctly must never be
"recovered" by loosening anything. It is reported here as a finding, not a setback.


## Board after wave 10 (`SD31-W10-INTEGRATE-001`, 2026-08-17)

Re-derived live with the producer's own `doneness_verdict()` over the committed `docs/work-inventory.json`
(the same command as every prior wave's table), not transcribed from any lane's own receipt:

| kind | total | done (wave 9) | done (wave 10) | delta |
|---|---:|---:|---:|---:|
| class | 185 | 27 (14.6%) | 27 (14.6%) | +0 |
| class_feature | 15,472 | 73 (0.5%) | 119 (0.8%) | +46 |
| companion | 1,696 | 680 (40.1%) | 680 (40.1%) | +0 |
| equipment | 6,208 | 4,372 (70.4%) | 4,372 (70.4%) | +0 |
| equipment_modifier | 1,580 | 380 (24.1%) | 380 (24.1%) | +0 |
| feat | 2,610 | 1,352 (51.8%) | 1,470 (56.3%) | +118 |
| monster | 1,270 | 840 (66.1%) | 910 (71.7%) | +70 |
| monster_ability | 2,951 | 1,366 (46.3%) | 1,366 (46.3%) | +0 |
| race | 103 | 7 (6.8%) | 7 (6.8%) | +0 |
| race_trait | 3,603 | 704 (19.5%) | 741 (20.6%) | +37 |
| spell | 2,843 | 1,157 (40.7%) | 1,157 (40.7%) | +0 |
| **TOTAL** | **38,521** | **10,958 (28.4468%)** | **11,229 (29.1503%)** | **+271** |

Denominator unchanged all package (38,521). Reachable ceiling unchanged (98.946% / 98.95%,
`ambiguous` population 406 units).

**Separating the two directions, not blurring them:** `feat` +118, `monster` +70, `class_feature`
+46 and `race_trait` +37 are real new work landing (Mythic feat gap-table generation net of the
`VISIBLE:EXPORT` fix, the SLA_CL literal-override fix independently re-derived 1,196/1,196 against
the pinned oracle, `PuClassId` chassis registration, the 4-race ARG follow-on chassis batch). The
`class_feature` net of +46 is itself the sum of +46 real credit and a set of honest corrections
that moved zero units into `done` (−21 `unmeasurable`->`not-started` registry-gap
characterization, +12/+4/+1 capped at `held`/`deferred`, −1 a disclosed regression) — the full
406-move, id-level table with every direction traced is in `progress.md`'s `SD31-W10-INTEGRATE-001`
receipt §3.

**Is the chooser primitive real, and what did it unlock — the operator's own question, answered
plainly:** Yes, it is real — `archetype_resolver::chooser_option_selected` has a genuine,
non-test production call chain reaching the desktop app's own entry point
(`build_pilot_headless_receipt` in `pf1_adapter.rs`), and both its corpus option pools transcribe
the pinned oracle exactly with zero invented entries. This wave used it for exactly ONE real
consumer — Oracle's Battle Mystery/Battlecry revelation — now correctly grounding a
Charisma-MODIFIER-scaled duration after this cycle fixed a fabricated-magnitude defect the
review caught before any unit could pay out on it. What it did NOT yet unlock, stated without
hedging: the primitive's own advertised corpus-pool membership guard is statically unreachable
at both of its current call sites (both pass compile-time constants that are always members of
themselves), and no player-facing Mystery picker exists in the desktop frontend, so the corrected
value cannot be driven on screen today — both logged as named followups (`OPEN-ISSUES.md` rows
180/185), not silently accepted. The primitive is the right foundation for the mandate's own
largest-named cause (~4,520 `class_feature` units sitting on option-pool content with no chooser
at all, across 1,847 distinct pool names) — this wave proved the shape works for one pool; the
next-highest-leverage step is wiring 4 more Oracle mysteries plus Sorcerer's/Arcanist's own
bloodlines and exploits through the same primitive, which is this receipt's own top-ranked
followup below.

## Board after wave 11 (`SD31-W11-INTEGRATE-001`, 2026-08-17)

Re-derived live with the producer's own `doneness_verdict()` over the committed `docs/work-inventory.json`
(the same command as every prior wave's table), not transcribed from any lane's own receipt:

| kind | total | done (wave 10) | done (wave 11) | delta |
|---|---:|---:|---:|---:|
| class | 185 | 27 (14.5946%) | 27 (14.5946%) | +0 |
| class_feature | 15,472 | 119 (0.7691%) | 137 (0.8855%) | +18 |
| companion | 1,696 | 680 (40.0943%) | 680 (40.0943%) | +0 |
| equipment | 6,208 | 4,372 (70.4253%) | 4,754 (76.5786%) | +382 |
| equipment_modifier | 1,580 | 380 (24.0506%) | 380 (24.0506%) | +0 |
| feat | 2,610 | 1,470 (56.3218%) | 1,470 (56.3218%) | +0 |
| monster | 1,270 | 910 (71.6535%) | 910 (71.6535%) | +0 |
| monster_ability | 2,951 | 1,366 (46.2894%) | 1,366 (46.2894%) | +0 |
| race | 103 | 7 (6.7961%) | 7 (6.7961%) | +0 |
| race_trait | 3,603 | 741 (20.5662%) | 741 (20.5662%) | +0 |
| spell | 2,843 | 1,157 (40.6964%) | 1,356 (47.6961%) | +199 |
| **TOTAL** | **38,521** | **11,229 (29.1503%)** | **11,828 (30.7053%)** | **+599** |

Denominator unchanged all package (38,521). Reachable ceiling unchanged (98.95%, `ambiguous`
population 406 units).

**Separating the two directions, not blurring them:** all +599 is real new credit landing this
wave — there is no correction-driven demotion out of `done` anywhere (zero `done`->non-`done`
transitions confirmed by full id-set diff). `class_feature`'s own +18 is itself the wave's most
scrutinized number: the pool-match lane's merged commit originally computed +38, this integration
cycle's own adversarial review CONFIRMED 20 of those 38 were credited on a byte-identical
computation to a DIFFERENT corpus record (`Shaman Spirit ~ *` vs. `Shaman Wandering Spirit ~ *`),
and the fix that withdraws them landed BEFORE the guarded regen ran — so the committed +18 is the
correct number outright, never a number that was corrected downward after being briefly wrong on
the board.

**Does the chooser now pay out — the operator's own question, answered plainly, with the number:**
Yes, but modestly, and mostly through a DIFFERENT lane than the mandate's own headline expected.
The mandate framed this wave around unblocking `chooser_option_selected` via the pool-matcher/slug
fixes (~1,974 units "one predicate fix away"). What actually happened: the pool-matcher fix
genuinely widened recognition to 249 previously-unmatchable corpus groups, but of those, only **18
units** currently produce an attributable delta on the sheet (10 primary `Shaman Spirit`, 3 `Witch
Hex`, 2 `Sorcerer Bloodline`, 3 apostrophe joins) — recognition (matching a corpus group to a pool)
and grounding (the matched member producing an observably different sheet value) are two different
bars, and the `--class-feature-probe` diagnostic reports 863 of the newly-recognised groups'
members still decline as `NoConsumerDelta`. Separately, the picker lane wired 4 MORE Oracle Mystery
pools through `chooser_option_selected` (Stone/Waves/Wind/Heavens, joining wave 10's Battle) — this
moved **zero board units** this wave, the same honest zero wave 10 reported for the primitive's
first wiring, because the units those pools would ground are gated behind the same recognition bar
the pool-matcher fix addresses for OTHER classes, not Oracle. So: the chooser primitive is real,
now has 5 wired pools (not 1), and the fix meant to unblock it in bulk instead unblocked a mostly-
unrelated 18-unit slice of other classes' pools — the ~4,520-unit option-pool population the
mandate named as the single largest lever is still there, now sitting on a genuinely fixed matcher,
waiting on the per-pool consumer-delta wiring `OPEN-ISSUES.md` row 186 names as the real remaining
gap.

## Board after wave 12 (`SD31-W12-INTEGRATE-001`, 2026-08-17)

Re-derived live with the producer's own `doneness_verdict()` over the committed `docs/work-inventory.json`
(the same command as every prior wave's table), not transcribed from any lane's own receipt:

| kind | total | done (wave 11) | done (wave 12) | delta |
|---|---:|---:|---:|---:|
| class | 185 | 27 (14.5946%) | 27 (14.5946%) | +0 |
| class_feature | 15,472 | 137 (0.8855%) | 137 (0.8855%) | +0 |
| companion | 1,696 | 680 (40.0943%) | 680 (40.0943%) | +0 |
| equipment | 6,208 | 4,754 (76.5786%) | 4,998 (80.5090%) | +244 |
| equipment_modifier | 1,580 | 380 (24.0506%) | 380 (24.0506%) | +0 |
| feat | 2,610 | 1,470 (56.3218%) | 1,475 (56.5134%) | +5 |
| monster | 1,270 | 910 (71.6535%) | 910 (71.6535%) | +0 |
| monster_ability | 2,951 | 1,366 (46.2894%) | 1,369 (46.3911%) | +3 |
| race | 103 | 7 (6.7961%) | 7 (6.7961%) | +0 |
| race_trait | 3,603 | 741 (20.5662%) | 490 (13.5998%) | **-251** |
| spell | 2,843 | 1,356 (47.6961%) | 1,356 (47.6961%) | +0 |
| **TOTAL** | **38,521** | **11,828 (30.7053%)** | **11,829 (30.7079%)** | **+1** |

Denominator unchanged all package (38,521). Reachable ceiling unchanged (98.95%, `ambiguous`
population unchanged).

**Separating the two directions, not blurring them — this wave's real headline is the demotion,
not the +1.** `race_trait`'s -251 is this cycle's own fix (§2 of the progress.md receipt): a
confirmed universal-sheet-modifier gate hole and a confirmed load-only-evidence defect, both
CONFIRMED by the wave-12 dispatch's own review context, both fixed by demoting rather than arguing,
exactly per the mandate's stated precedence-2 standard ("the board has taken honest decreases
before and that is the standard"). The +244/+5/+3 on the other side of the ledger is real new
content from three of the four remaining lanes, landing in the SAME regen as the correction — not
sequenced separately, not blurred together: `-251 + 244 + 5 + 3 = +1`, exactly the net board
movement, with every unit's direction traced to a named cause.

**Does the option-pool chooser now pay out more — the operator's own standing question, answered
plainly, with the number:** No further movement this wave. Barbarian Superstition (this wave's only
new class_feature pool-consumer wiring) landed **0 board units** — the cross-variant collision it
found between the base-Barbarian and Unchained-Barbarian Superstition records is correctly still
refused by `classify()`'s pre-existing book-attribution guard, which the lane did not ask to
weaken (checked directly — no such instruction exists anywhere in this repo's committed content).
The chooser primitive's wired-pool count is unchanged from wave 11 (5, not more). The
~4,271-unit remaining option-pool population (Domain/Blessing/most-of-Bloodline/Mystery, needing
real per-pool consumer-delta wiring) is unchanged. A DIFFERENT lever surfaced this wave instead:
feat-matcher's `mod_only_rescue` finding, a real, exact, zero-exception 249-unit cross-kind
phantom-duplicate population that would shrink the `feat` kind and the board DENOMINATOR by
roughly the same amount if applied — PROPOSED, not applied, `OPEN-ISSUES.md` row 205, needing an
operator ruling under the same propose-then-rule pathway Decision 9/10 established.
| 13 | 2026-08-17 | wave `sd31-w13` (agents: `sd31-provenance`, `sd31-register-race`, `sd31-fixture-seam`, `sd31-racetrait6`, `sd31-cf-pools`, `sd31-ingest6`, three `sd31-w13-refute-*` review agents, integration `sd31-w13-integrate`) | Decision 12/14 (public-feed PI redaction + provenance schema); Decision 13 direction correction applied to the Supersession Register + owed race-branch evidence table; class_feature `derived_evaluator_fixture_check` seam (the mandate's own named highest-leverage gap); Changeling/Samsaran race chassis closing `arg_races.lst`'s 37-row roster; Unchained Barbarian's own Rage Power chooser + PU-wide roster-id false-grounding audit; the 13 `.COPY=` racial spell-like-ability spell variants (Decision 15); three Opus-effort adversarial reviews (14 CONFIRMED findings total); this integration cycle (`SD31-W13-INTEGRATE-001`) merging all six lanes, fixing 10 of 14 CONFIRMED findings, running the sanctioned guarded regen, and the wave's full gate | Board: 11,829/38,521 (30.71%) -- UNCHANGED, a genuine net-zero reconciliation (+8 class_feature fixture-verified, +7 race_trait chassis, -15 class_feature PU-roster-id correction, 0 elsewhere). **The fixture seam now genuinely lets a `derived` unit reach `done`** -- 8 real units, the mandate's own named binding constraint, closed for the first time. PI: 3 leaked declared-PI names found and fixed (`Bow of Erastil`/`Legendsbane`/`Witherfang`), 20 more found and redacted by the same fix, `site-dashboard-pi-gate` CLEAN and mutation-proven on the exact 3 names. Reachable ceiling 98.95% (unchanged). Trap report 1 mod-record/1,225 wiring-class-mismatch (byte-identical baseline). See `progress.md` cycle `SD31-W13-INTEGRATE-001` for `VERIFY_EXIT` and full stage summary. |

## Board after wave 14 (`SD31-W14-INTEGRATE-001`, 2026-08-18)

Re-derived live with the producer's own `doneness_verdict()` over the committed
`docs/work-inventory.json` (the same command as every prior wave's table), not transcribed from
any lane's own receipt. Oracle pin `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.
Dashboard `status_sources_agree` is **false**, so per loop-instruction override 9 the figures below
are named by source: `work_inventory.by_doneness` / `mandate_headline`, stamp
`doneness_source_generated_at 2026-08-19T01:10:47Z` — which agrees field-for-field with the
independent per-unit replay.

| kind | total | done (wave 13) | done (wave 14) | delta |
|---|---:|---:|---:|---:|
| class | 185 | 27 (14.5946%) | 27 (14.5946%) | +0 |
| class_feature | 15,472 | 130 (0.8402%) | 130 (0.8402%) | +0 |
| companion | 1,696 | 680 (40.0943%) | 684 (40.3302%) | +4 |
| equipment | 6,208 | 4,998 (80.5090%) | 5,303 (85.4220%) | **+305** |
| equipment_modifier | 1,580 | 380 (24.0506%) | 421 (26.6456%) | +41 |
| feat | 2,610 | 1,475 (56.5134%) | 1,459 (55.9004%) | **-16** |
| monster | 1,270 | 910 (71.6535%) | 910 (71.6535%) | +0 |
| monster_ability | 2,951 | 1,369 (46.3911%) | 1,456 (49.3392%) | **+87** |
| race | 103 | 7 (6.7961%) | 34 (33.0097%) | **+27** |
| race_trait | 3,603 | 497 (13.7941%) | 497 (13.7941%) | +0 |
| spell | 2,843 | 1,356 (47.6961%) | 1,356 (47.6961%) | +0 |
| **TOTAL** | **38,521** | **11,829 (30.7079%)** | **12,277 (31.8709%)** | **+448** |

Denominator unchanged all package (38,521); 0 unit ids added, 0 removed. Reachable ceiling
unchanged (98.95 %; the `ambiguous` population is still 406 and `SD31-E2-F3-002-marker` proved
this wave that the dead end is structural, not an engineering gap). **`race`'s reachable ceiling
is now 100 %** — the first wave in six in which that kind moved at all.

**The largest single-wave board gain of the package so far (+448, +1.16 pp), and the two
directions are kept apart, not netted.** Up: `SD31-E6-F5-005` +346 (equipment provenance
narrowing, which put 412 already-shipped records inside `corpus_literal_sweep`'s population for
the first time), `SD31-CE-COMPANION-001` +91 (Core Essentials re-attribution), `SD31-E1-F3-001`
+27 (the `race` verdict re-pointed at the product). Down: `SD31-E2-F3-002-marker` **-16**, the
wave's honest decrease — 23 `feat` units demoted because the string a player actually reads opens
with PCGen's own admission that the rule is not mechanised, and the case-sensitive detector had
been giving opposite verdicts to the same marker on letter case alone. `+346 + 91 + 27 − 16 =
+448`, and every one of the 575 units that changed bucket is traced to a named `evidence` token
in `progress.md`'s `SD31-W14-INTEGRATE-001` receipt §2, with no residue.

**Two lanes were dispatched onto the same card and one branch was not merged.**
`worktree-wf_1ad13e3b-085-2` and `-3` both worked the Core Essentials removal without either
knowing, and took contradictory routes on the same 102 companion records. `-3` followed
`decisions.md §9`'s named signal (the `.lst`'s own `SOURCELONG:` header), discharged the card and
reached a green gate; `-2` blocked on the card's central deliverable. `-3` was merged; `-2` was
not, for supersession, not for gaming — its findings are preserved as `OPEN-ISSUES.md` rows 262
and 263 rather than discarded, and its one unique deliverable was verified as no longer owed
(`grep -rl 'core_essentials:' data/corpus/` → 0 on the merged tip). Wave 15 must allocate cards
to lanes explicitly.

**Three units were WITHDRAWN by this integration cycle, and 13 more were never banked.** The
adversarial reviews returned 14 CONFIRMED findings across the four merged lanes; nine are fixed in
code here. The two that moved numbers: (i) the `decisions.md §9` re-attribution widening credited
13 `bestiary` `monster_ability` units off a DIFFERENT row whose key equalled their bare name —
`holds_key` is `contains(key) || contains(name)` and a `<Group> ~ <Facet>` row's name is its bare
facet — fixed with a strict `holds_unit_by_key` for the one caller that mints credit, taking that
lane's collateral from -189 to **-176** on the `not-ingested` axis; and (ii) three `computed`
races (Aasimar, Tiefling, Changeling) reached `done` straight from `grounded` with no second
check, on the same roster observation that for a `static` race merely unblocks an independent
byte-verification — withdrawn, taking that lane from +30 to **+27**. A mutation replacing
`race_creation_chassis`'s entire body with an unconditional `Ok(..)` left the board identical,
which is what settled it.

| 14 | 2026-08-18 | wave `sd31-w14` (lanes: `SD31-CE-COMPANION-001`, `SD31-E6-F5-005`, `SD31-E1-F3-001`, `SD31-E2-F3-002-marker`, plus an unmerged second Core Essentials lane; four adversarial reviews; integration `sd31-w14-integrate`) | Core Essentials finally removed as an engine book — `rules_tables/core_essentials/` deleted, 102 companion rows re-filed under the books their own `SOURCELONG:` headers name, `reach_gate`'s companion claim from 102 unreachable to zero; per-FIELD provenance (`description_source` on `CorpusRecordV1`) narrowing 412 equipment records from a web citation to the pinned oracle row; the `race` verdict re-pointed at the product's own creation roster, breaking a six-wave stall; the not-implemented marker detector widened case-insensitively (an honest -16); this integration cycle merging four of five lanes, fixing 9 of 14 CONFIRMED findings, withdrawing 3 units and refusing 13, running the sanctioned guarded regen, publishing the feeds, and running the wave's full gate | Board: **12,277/38,521 (31.87 %), +448 — the package's largest single-wave gain**, every unit traced to a named cause in both directions (+346 equipment, +91 Core Essentials, +27 race, -16 feat). `race` 7 → 34 done, reachable ceiling for that kind now 100 %. Reachable ceiling overall 98.95 % (unchanged). Corpus sweep CLEAN (26,105 examined, 0 findings); `derived_evaluator_fixture_check` 1,276 / 1,276, 0 failed. Two new durable gates: `sd31_lst_provenance_repair_is_durable` (a cache regeneration silently reverting the narrowing is now RED) and `reattribution_widening_tests`; `sd27_book_license_record_counts`'s two guards no longer mask every book after the first. `core_essentials` still carries 128 unattributable units — `OPEN-ISSUES.md` row 263, needs an operator ruling, and until it is zero `core_essentials` must NOT be excluded. See `progress.md` cycle `SD31-W14-INTEGRATE-001` for `VERIFY_EXIT` and the full stage summary. |


## Board after wave 15 (`SD31-W15-INTEGRATE-001`, 2026-08-19)

Re-derived live with the producer's own `doneness_verdict()` over the committed
`docs/work-inventory.json` (`generated_at` 2026-08-19T14:06:36Z) — the same command as every prior
wave's table, never transcribed from a lane's own receipt. Oracle pin
`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`, checked first per loop-instruction
override 8. Both figures below come from the per-unit replay over that document, which is the
source `doneness_source_generated_at` names.

| kind | total | done (wave 14) | done (wave 15) | delta |
|---|---:|---:|---:|---:|
| class | 185 | 27 (14.5946%) | 27 (14.5946%) | +0 |
| class_feature | 15472 | 130 (0.8402%) | 134 (0.8661%) | +4 |
| companion | 1696 | 684 (40.3302%) | 801 (47.2288%) | +117 |
| equipment | 6208 | 5303 (85.4220%) | 5311 (85.5509%) | +8 |
| equipment_modifier | 1580 | 421 (26.6456%) | 438 (27.7215%) | +17 |
| feat | 2610 | 1459 (55.9004%) | 1459 (55.9004%) | +0 |
| monster | 1270 | 910 (71.6535%) | 973 (76.6142%) | +63 |
| monster_ability | 2951 | 1456 (49.3392%) | 1548 (52.4568%) | +92 |
| race | 103 | 34 (33.0097%) | 34 (33.0097%) | +0 |
| race_trait | 3603 | 497 (13.7941%) | 520 (14.4324%) | +23 |
| spell | 2843 | 1356 (47.6961%) | 1503 (52.8667%) | +147 |
| **TOTAL** | **38521** | **12277 (31.8709%)** | **12748 (33.0936%)** | **+471** |

By doneness bucket: `done` 12,748 · `held` 1,254 · `in-progress` 1,342 · `not-started` 18,030 ·
`unmeasurable` 5,109 · `deferred` 38. Denominator unchanged all package (38,521); 0 unit ids added,
0 removed. Reachable ceiling unchanged (98.95 %).

**The largest single-wave board gain of the package, and the first with movement in ONE direction
only.** `+471` (+1.22 pp), beating wave 14's `+448`. Every one of the 471 units that changed bucket
is traced to a named `evidence` token in `progress.md`'s `SD31-W15-INTEGRATE-001` receipt §2, with
no residue: spell 147, companion 117, monster_ability 92, monster 63, equipment_modifier 17 +
equipment 8, race_trait 23, class_feature 4. **Zero demotions and zero withdrawals** — the first
wave in four with none, and that is a finding rather than a boast: the four adversarial reviews
found no wrongly-taken credit, and both PARTIAL verdicts state explicitly that the units stand. The
one candidate for a decrease (~11 `equipment_modifier` `.COPY=` aliases whose base record was
already `done`) is an operator question, row 288, and was not netted out unilaterally.

**Three new derived-evaluator seams, tripling what the `derived` done rung can reach.** The
`derived` wiring class caps at `held` until a unit carries a `fixture-verified` stamp, and before
this wave five families could produce one. Now eight: `monster_sla` (PF1's spell-like-ability save
DC run BACKWARDS to derive the granted spell's LEVEL, pinned against that spell's own record in a
DIFFERENT FILE), `monster_ability` (the Universal Monster Rule's `10 + ½ racial HD`, cross-checked
against the owner's own `MONSTERCLASS:` row), and `companion` (CRB p.182's single-natural-attack
1½× Strength rule). Fixture coverage went 1,276 → **1,699 units cleared over 2,364 rows, 0 failed**.

**A defect the merge itself created, caught by the integration gate and by nothing else.** Two
lanes were green in isolation; one added a field to `MonsterStatBlock` while the other built a
literal of it, and `root-full` went `cargo exit 101; 0 passed across 0 suites` on the merge (run 1,
`RESULT: FAIL`, `VERIFY_EXIT=1`). Run 2 after the fix: `RESULT: PASS`, `VERIFY_EXIT=0`, 34/34. Both
logs are committed. Six green lane gates are not a green wave.

**Three of the wave's four new fixture generators would have ERASED their own committed rows on a
second run** — selecting `status == "grounded"` while stamping rewrites covered units to
`fixture-verified`, then replacing the array rather than merging. The review confirmed one; this
cycle re-ran every generator and found a third (`companion`, all 117 rows). Fixed, and now proven:
**all eight families re-derive BYTE-IDENTICALLY from the pinned oracle on the stamped tree.**

**A gate that could only pass in one checkout was fixed rather than baselined away.** The published
feeds recorded the absolute filesystem path of whichever tree published them, which was the single
differing leaf in a 1.3 MB payload after `site-dashboard-check`'s own scrub — so the stage reported
STALE for every other checkout, and a home directory was being committed into the
Cloudflare-published `site/`. Now repo-relative, proven both ways: it still FAILS on a real content
change, and it now PASSES from a different checkout.

**Operator rulings §16 and §17 are recorded, not executed.** Both change the denominator (~87
deletions and up to 180 chooser-pair removals), this wave's dispatch freezes it, and `§16` itself
says to execute against re-derived figures. Written into `decisions.md` with their acceptance
criteria and owed as wave-16 cards. `core_essentials` therefore still carries its 128 residual
units and must still NOT be added to `EXCLUDED_BOOKS`.

| 15 | 2026-08-19 | wave `sd31-w15` (lanes: `SD31-W15-MONSTER-SLA-001`, `SD31-W15-MONSTER-ABILITY-001`, `SD31-W15-COMPANION-001`, `SD31-W15-SPELL-CF-001`, `SD31-W15-EQUIPMOD-001..006`, `SD31-W15-RACETRAIT-001`; three adversarial reviews returning four lane verdicts; integration `sd31-w15-integrate`) | Three NEW derived-evaluator seams (monster spell-like-ability save DC → spell level; monster_ability Universal Monster Rule save-DC base; companion single-natural-attack 1½× Strength), taking the `derived` done rung from five families to eight; the `kind=spell` RANGE generator un-blinded from a LEXICOGRAPHIC `wiring_class_reason` tie-break that had hidden 151 units; `probe_equipment_effect_wiring` reading `data/corpus/*/equipment/` instead of a hand-maintained 13-book-short list, and the equipment probe consulted ABOVE the `text-complete` rung; the character-creation chassis now NAMING the record whose ability magnitude it read; this integration cycle merging all six lanes (none GAMED), fixing four CONFIRMED findings plus one the merge itself created, logging six, folding in operator rulings §16/§17, running the sanctioned guarded regen, publishing the feeds and running the wave's full gate twice | Board: **12,748/38,521 (33.09 %), +471 — the package's largest single-wave gain**, and the first with movement in ONE direction only: zero demotions, zero withdrawals, every unit traced to a named `evidence` token. Fixture coverage 1,276 → **1,699 units over 2,364 rows, 0 failed**. Stamps 7,629 → 8,052, zero lost; the stamp guard proven able to fire (a bare regen exits 1 refusing to drop 8,052 stamps). Gate run 1 `FAIL`/`VERIFY_EXIT=1` on a merge-created compile break invisible to both lanes; run 2 `PASS`/`VERIFY_EXIT=0`, 34/34, root-full 7,115 across 573 suites, desktop 469 (tested explicitly as a separate workspace), both PI gates zero leaked vs 1,612 declared-PI names, clippy ceilings unchanged. Three of four new fixture generators would have erased their own rows on re-run — fixed, and all eight families now re-derive byte-identically. `site-dashboard-check` no longer path-pinned to one checkout. Denominator, race attribution, the Supersession Register (116 objects clean, PROPOSED-NOT-APPLIED) and the empty Structural Exclusion Register all untouched. See `progress.md` cycle `SD31-W15-INTEGRATE-001` for `VERIFY_EXIT` and the full stage summary. |
| 16 | 2026-08-19 | wave `sd31-w16` (lanes: `ruling-16-ce` core_essentials deletion execution, `ruling-17-dupes` duplicate-chooser display-name dedup, `seam-monster` investigation (banked 0), `seam-monster-ability` formula sub-seam, `seam-companion-spell` companion Climb/Swim seam, `equipment-modifier` armor-enhancement probe widening; three adversarial reviews returning six lane verdicts; integration `sd31-w16-integrate`) | Executed BOTH standing operator rulings against re-derived figures: §16 (`core_essentials` residual 128→0, `decisions.md §9` discharged, 12 re-attributed + 116 deleted) and §17 (33 confirmed duplicate-chooser units removed, not the 180-unit heuristic bound, plus the drill-down disambiguator); two new derived-evaluator seams (`companion` Climb/Swim skill-ability-diff, `monster_ability` full-formula save-DC); `equipment_modifier`'s armor-enhancement probe widened past a standalone-resolution gap; this integration cycle merging all six lanes (none GAMED), fixing 3 CONFIRMED adversarial-review findings plus 1 the merge itself created (caught by `root-full`, not any lane), logging 4 more, running the sanctioned guarded regen, publishing the feeds with a forced-fresh cache, and running the wave's full gate twice | Board: **12,864/38,372 (33.52 %), +116 done, -149 denominator — the first wave with denominator movement since the freeze, both changes operator-directed deletions of content ruled never in scope, never a cost exclusion.** Every unit of both movements traced to a named cause in both directions (progress.md §2/§3): -116 net ruling §16, -33 ruling §17, +65 equipment_modifier armor-enhancement widening, +45 companion Climb/Swim seam, +6 monster_ability formula seam. `core_essentials` confirmed absent from the published book list and from `docs/work-inventory.json`'s `books` map entirely. Fixture coverage 1,699→**1,750 units over 2,504 rows, 0 failed**. Stamps 8,052→**8,103**. A gate hole the companion lane's own bar check carried (arithmetic-only, never checking WHICH abilities) was found by review and fixed at integration, not left open. Gate run 1 `FAIL`/`VERIFY_EXIT=1` on a merge-created consequence invisible to any lane (ruling §16 correctly drove `core_essentials` to zero contribution, which a pre-existing roster-completeness test had never anticipated for a `shared_library`-scoped book); run 2 `PASS`/`VERIFY_EXIT=0`, 34/34 stages (root-lib 2084, root-full 7144/573 suites, desktop 470, reach 29, corpus-sweep 0 findings, clippy root:51/desktop:7 warnings/0 errors, class-dump 31/31). Race attribution, the Supersession Register (PROPOSED-NOT-APPLIED), and the empty Structural Exclusion Register all untouched. See `progress.md` cycle `SD31-W16-INTEGRATE-001` for the full stage summary. |
## Board after wave 16 (`SD31-W16-INTEGRATE-001`, 2026-08-19)

Re-derived live with the producer's own `doneness_verdict()` over the committed
`docs/work-inventory.json` (`generated_at` 2026-08-19T18:06:59Z) — the same command as every prior
wave's table, never transcribed from a lane's own receipt. Oracle pin
`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`, checked first per loop-instruction
override 8. **Denominator moved for the first time since the freeze — 38,521 → 38,372 — from two
operator-directed deletions (rulings §16/§17), reported separately from doneness movement in
`progress.md` `SD31-W16-INTEGRATE-001` §2/§3.**

| kind | total (wave 15) | total (wave 16) | done (wave 15) | done (wave 16) | delta |
|---|---:|---:|---:|---:|---:|
| class | 185 | 185 | 27 (14.5946%) | 27 (14.5946%) | +0 |
| class_feature | 15,472 | 15,439 | 134 (0.8661%) | 134 (0.8681%) | +0 |
| companion | 1,696 | 1,696 | 801 (47.2288%) | 846 (49.8821%) | **+45** |
| equipment | 6,208 | 6,208 | 5,311 (85.5509%) | 5,311 (85.5509%) | +0 |
| equipment_modifier | 1,580 | 1,580 | 438 (27.7215%) | 503 (31.8354%) | **+65** |
| feat | 2,610 | 2,610 | 1,459 (55.9004%) | 1,459 (55.9004%) | +0 |
| monster | 1,270 | 1,270 | 973 (76.6142%) | 973 (76.6142%) | +0 |
| monster_ability | 2,951 | 2,942 | 1,548 (52.4568%) | 1,554 (52.8212%) | **+6** |
| race | 103 | 95 | 34 (33.0097%) | 34 (35.7895%) | +0 |
| race_trait | 3,603 | 3,504 | 520 (14.4324%) | 520 (14.8402%) | +0 |
| spell | 2,843 | 2,843 | 1,503 (52.8667%) | 1,503 (52.8667%) | +0 |
| **TOTAL** | **38,521** | **38,372** | **12,748 (33.0936%)** | **12,864 (33.5244%)** | **+116** |

`race`/`race_trait`/`class_feature`/`monster_ability` total-column drops are the denominator
change (ruling §16's 128→12 net + ruling §17's -33), not a doneness loss — no `done` unit appears
in either deletion. `companion`/`equipment_modifier`/`monster_ability` done-column gains are three
independent seams (see the row-16 entry above and `progress.md` §3).

### What wave 16 changed in the architecture, not just in the counts

* **`decisions.md §9`'s condition is discharged.** `core_essentials` no longer appears as a key in
  `docs/work-inventory.json`'s `books` map at all — the label that has been tracked and reported
  since SD-31's very first wave reached zero and stayed there, confirmed by a production-path
  ceiling assertion (`CORE_ESSENTIALS_RESIDUAL_DELETION_CEILING`), not just by a test.
* **The denominator is no longer frozen in fact, only in principle** (Decision 6's cost-exclusion
  ban still holds absolutely): both changes this wave were operator-directed deletions of content
  ruled never to have existed in scope, executed against re-derived per-row evidence rather than
  the ruling's own upper-bound estimates in both cases (12/1 vs. ~41 rescued for §16; 33 vs. 180 for
  §17) — a smaller, more honestly-derived number in both directions.
* **A merge-created consequence (not a lane defect) was caught by the gate, exactly as the process
  is designed to catch it.** `every_corpus_book_appears_in_the_inventory` had never been wrong
  before this wave — no lane could have caught it in isolation, since it only fires once
  `core_essentials`'s residual reaches true zero at a fully-merged tip. Fixed by exempting
  `shared_library`-scoped books (of which `core_essentials` is the only one) from the
  roster-completeness check, on the ground that zero self-owned units is that scope's intended
  success state, not an omission.

## Board after wave 17 (`SD31-W17-INTEGRATE-001`, 2026-08-19)

Re-derived live with the producer's own `doneness_verdict()` over the committed
`docs/work-inventory.json` — the same command as every prior wave's table, never transcribed from
a lane's own receipt. Oracle pin `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.
**Denominator UNCHANGED — 38,372 = 38,372 — as required this wave (no operator-signed Structural
Exclusion Register entry was needed or written).**

| kind | total (wave 16) | total (wave 17) | done (wave 16) | done (wave 17) | delta |
|---|---:|---:|---:|---:|---:|
| class | 185 | 185 | 27 (14.5946%) | 27 (14.5946%) | +0 |
| class_feature | 15,439 | 15,439 | 134 (0.8679%) | 134 (0.8679%) | +0 |
| companion | 1,696 | 1,696 | 846 (49.8821%) | 871 (51.3561%) | **+25** |
| equipment | 6,208 | 6,208 | 5,311 (85.5509%) | 5,312 (85.5670%) | **+1** |
| equipment_modifier | 1,580 | 1,580 | 503 (31.8354%) | 503 (31.8354%) | +0 |
| feat | 2,610 | 2,610 | 1,459 (55.9004%) | 1,459 (55.9004%) | +0 |
| monster | 1,270 | 1,270 | 973 (76.6142%) | 973 (76.6142%) | +0 |
| monster_ability | 2,942 | 2,942 | 1,554 (52.8212%) | 1,556 (52.8892%) | **+2** |
| race | 95 | 95 | 34 (35.7895%) | 34 (35.7895%) | +0 |
| race_trait | 3,504 | 3,504 | 520 (14.8402%) | 520 (14.8402%) | +0 |
| spell | 2,843 | 2,843 | 1,503 (52.8667%) | 1,503 (52.8667%) | +0 |
| **TOTAL** | **38,372** | **38,372** | **12,864 (33.5244%)** | **12,892 (33.5974%)** | **+28** |

`+25 companion` (new DESC-embedded save-DC formula seam), `+2 monster_ability` (owner-resolver
bare-leading-field fallback), `+1 equipment` (`maul_of_the_titans`, off a sound weapon-enhancement
roll-order widening). `class_feature`'s 809-unit internal reclassification (`unknown` →
`not-ingested`/`deferred-with-reason`) carries **zero** done-eligible movement, by design. A
5-unit `equipment_modifier` claim (Amulet of Mighty Fists) was refused at merge time — see
`progress.md` `SD31-W17-INTEGRATE-001` §4/§6.

### What wave 17 changed in the architecture, not just in the counts

* **A shipped widening that would have applied a magic item's own scoped bonus to every equipped
  weapon was caught by adversarial review and reverted before merge, not after.** `equipmods.rs`'s
  `WEAPONPROF=TYPE.Natural` recognition (intended for the Amulet of Mighty Fists family) had no
  field to carry the natural-attack scope the corpus token states, and the live consumer applies
  the bonus indiscriminately. This is the wave's one "refuse and re-credit" case — the lane's work
  was not GAMED (the reviewer's own words), but the specific mechanism was wrong and is now logged
  as a real engineering follow-up (`OPEN-ISSUES.md` row 309) rather than shipped.
* **A reported "811 of 3,864" characterization figure was measured against a different predicate
  than the one shipped in code** — the lane's own report used a plural-tolerant marker match while
  `class_feature_type_facet_owner_candidates` was singular-only, so a default run would have
  silently recovered 510, not 811. Fixed at merge time so the shipped code's live behavior matches
  its own documented figure (`OPEN-ISSUES.md` row 311).
* **The `monster` seam lane's census of its own 253-unit "exhaustive, none viable" population was
  found materially wrong** (16 of 236 units miscategorized into the wrong bucket, a second
  arithmetic error, and an 11-unit previously-unexamined sub-population) — no code or board change
  resulted (the lane had banked 0 and carried no branch), but the corrected census is now the
  record for the next `monster`-kind cycle to start from (`OPEN-ISSUES.md` row 310).
