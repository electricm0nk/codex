# SD-31 Progress Log

## Split from SD-30 (2026-08-14)

Package created by operator ruling 2026-08-14 ("split phase 3 and phase 4 into their own SD's; SD-31
and SD-32"). Epics 4, 5, 6, 10, 11, and the grind-lane half of 14 moved from
`SD-30-class-feature-archetype-bundle/` (renumbered 1-6 here — see `epic-breakdown.md`'s map). No card
was `IN-FLIGHT` at split time (verified against SD-30's `kanban.md` immediately before the move); every
moved card carries forward as `READY`/`READY (gated on ...)`, state unchanged.

SD-30's own per-cycle receipts for this scope, recorded before the split, remain in
`SD-30-class-feature-archetype-bundle/progress.md` as history — not copied here. Cycles claiming a card
in this package's `kanban.md` from this point forward append their receipts below.

No cycles claimed yet.

## 2026-08-15 — Launch-readiness remediation Step 1: S1-doneness-bar (RETRO_ACTOR sd31-ready-s1)

**Scope.** Plan Step 1 of
`~/.claude/plans/conduct-a-launch-readines-zesty-ripple.md` — blockers B1/B3. Landed Decision 5 (the
mandate denominator, operator ruling 2026-08-15) in `decisions.md`; the doneness bar in
`epic-breakdown.md` Epic 9-F1 and the Completion gate; `AT-31-103` in
`acceptance-and-verification.md`; relabelled `AT-31-005`'s `done+held` floors as progress floors, not
closure criteria; updated the exit gate checklist; updated `README.md` Purpose/Exit statement to name
the doneness bar. Doc-only step — no Rust/Python/shell production code touched.

**Started from HEAD** `56512485cbd89594c832e976c18e47adf1820987` on `tranche/10`. Tree was clean at
start (`git status --porcelain` empty); confirmed before any write.

**Figures re-derived this cycle (every one reproduced, not transcribed):**

1. Strict mandate denominator/doneness (command in `AT-31-103`, `decisions.md §5`):
   ```
   python3 -c "import json,sys,collections; sys.path.insert(0,'scripts/observer'); import
   pf1e_dashboard_producer as P; U=[u for u in json.load(open('docs/work-inventory.json'))['units'] if
   u.get('book') not in P.EXCLUDED_BOOKS]; c=collections.Counter(P.doneness_verdict(u.get('wiring_class'),
   u.get('status'),u.get('kind')) for u in U); print(c, len(U))"
   ```
   → `Counter({'not-started': 20895, 'held': 6916, 'done': 5837, 'unmeasurable': 3989,
   'in-progress': 848, 'deferred': 36}) 38521`. **done=5,837, denominator=38,521, 15.15 %** — matches
   the plan's expected figure exactly.
2. Old/secondary headline (in-scope books only, minus `unmeasurable`/`deferred`, replaying the live
   dashboard's `inScopeUnits()`/`usableDenom()` logic from `~/swarm-observer/PF1e-dashboard.html` over
   `docs/work-inventory.json`'s `books[].scope == "in_scope"`):
   ```
   python3 -c "
   import json, sys, collections
   sys.path.insert(0,'scripts/observer')
   import pf1e_dashboard_producer as P
   d = json.load(open('docs/work-inventory.json'))
   in_scope_ids = {b['id'] for b in d['books'] if b.get('scope') == 'in_scope'}
   U = [u for u in d['units'] if u.get('book') in in_scope_ids]
   c = collections.Counter(P.doneness_verdict(u.get('wiring_class'), u.get('status'), u.get('kind')) for u in U)
   denom = len(U) - c['unmeasurable'] - c['deferred']
   print(c, 'denom', denom, 'pct', round(c['done']/denom*100,2))
   "
   ```
   → `done=5,837, denom=30,402, pct=19.20` — matches the plan's cited B3 figure exactly, and confirms
   the source of the 30,402/19.20% figure is the book-level `scope` field (30 `in_scope` of 38 books;
   7 `future_state`, 1 `out_of_scope` = `beginner_box`).
3. Book-scope roster: `Counter({'in_scope': 30, 'future_state': 7, 'out_of_scope': 1})` — 38 total
   books, 37 non-`beginner_box`, matching the operator ruling's "37 non-`beginner_box` books" text
   exactly.
4. Invariance check: the 7 `future_state` books already carry **4,094 units** inside
   `docs/work-inventory.json` today (`adventurers_guide`, `inner_sea_faiths`, `inner_sea_magic`,
   `inner_sea_taverns`, `inner_sea_temples`, `mythic_adventures`, `occult_adventures`) — confirms
   Epic 7 onboarding those books moves units toward `done` inside the existing 38,521 denominator,
   never widens it.

**Not independently re-derived this cycle:** the reachable-ceiling figure (36,412/38,521 = 94.53 %)
cited in `decisions.md §5` from the readiness plan's blocker B1 — `scripts/reachability_audit.py` is
Epic 0's own not-yet-built deliverable (explicitly out of scope for this pass, per the plan's
"Explicitly NOT in this pass"), so that figure is carried by source citation, not reproduced by me this
cycle. Flagged, not silently presented as re-derived.

**Retro.** One correction emitted for the denominator ambiguity
(`docs/retro/events/sd31-ready-s1.jsonl`, event id `1786802682843-sd31-ready-s1-51d39b`):
subject "SD-31 exit gate / AT-31-102 (pre-remediation)", claimed 30,402 vs actual 38,521 as the binding
mandate denominator, `--verified-by` the two commands above. No standalone `retro.py decision` event —
`decisions.md §5` itself is that record, dated and attributed.

**Verification.** Doc-only step; ran `./scripts/verify.sh --only preflight-disk` only, per the step
brief (no Rust/Python/shell production code changed):
```
==> preflight-disk — disk budget check before any build starts
    repo filesystem (/home/ubuntu/workspace/repos/codex, mounted at /): 40% used, 580G available
    scratch-log filesystem (/tmp/codex-verify-DsNt8Z, mounted at /): 40% used, 580G available
    PASS  preflight-disk  (disk budget OK)
SUMMARY
  passed:  1  preflight-disk
RESULT: PASS
```
`VERIFY_EXIT=0`.

**Files changed:** `decisions.md` (+Decision 5), `epic-breakdown.md` (Epic 9-F1 + Completion gate gain
the doneness bar), `acceptance-and-verification.md` (+AT-31-103, AT-31-005 relabelled, exit checklist
updated), `README.md` (Purpose/Exit statement name the doneness bar), `docs/retro/events/sd31-ready-s1.jsonl`
(+1 correction event), this file.

**Status:** complete for this step's stated scope. Steps 2-6 of the plan (cards for unowned kinds,
oracle pin, dashboard import, drift sweep, pre-launch checklist) are separate steps, not owned by this
receipt.

## 2026-08-15 — Launch-readiness remediation Step 2: S2-cards (RETRO_ACTOR sd31-ready-s2)

**Scope.** Plan Step 2 of `~/.claude/plans/conduct-a-launch-readines-zesty-ripple.md` — blocker B2
(six not-done kinds, 9,894 units, no card) and the coupled parts of B4 (the 5,273-unit held
static/derived residual, the 1,243-unit `display|grounded` widening of AT-31-010) and the 36
`deferred-with-reason` units carried with zero sign-off. Doc-only step — no Rust/Python/shell
production code touched.

**Started from HEAD** `85aaa0c77ad91e027105000b295634e18df1885f` on `tranche/10`. Tree was clean at
start (`git status --porcelain` empty); confirmed before any write.

**Figures re-derived this cycle (every one reproduced, not transcribed — commands and outputs also
recorded in the package files themselves, repeated here for the receipt):**

1. **The six-kind ladder** (`epic-breakdown.md` Epic 6 F5-F10 preamble):
   ```
   python3 -c "
   import json, sys, collections
   sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
   d = json.load(open('docs/work-inventory.json'))
   U = [u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
   for k in ('equipment','monster_ability','feat','companion','equipment_modifier','class'):
       units = [u for u in U if u.get('kind')==k]
       verdict = lambda u: P.doneness_verdict(u.get('wiring_class'),u.get('status'),k)
       c = collections.Counter(verdict(u) for u in units)
       print(k, 'total', len(units), 'done', c.get('done',0), 'not_done', len(units)-c.get('done',0))
   "
   ```
   → `equipment 6,208/2,626 done/3,582 not-done` · `monster_ability 3,107/334/2,773` ·
   `feat 2,610/1,178/1,432` · `companion 1,696/416/1,280` · `equipment_modifier 1,580/911/669` ·
   `class 185/27/158`. **Sum 9,894 — matches the plan's expected figure exactly**, per-kind and in
   total. Per-kind `(wiring_class, status)` cell breakdowns for the not-done population also
   re-derived and recorded in `epic-breakdown.md` F5-F10 (per-cell command in `s2_ladder.py`,
   scratchpad, not committed — the per-kind commands embedded in the package files are the
   reproducible record).
2. **Epic 6-F1 `monster` rewrite** (fixture-coverage lane, not ingest): `monster` total 1,270,
   `{'held': 1235, 'done': 7, 'not-started': 28}`, of which **1,229 is the single cell
   `derived|grounded`**. Command recorded in `epic-breakdown.md` F1.
3. **Epic 6-F11 held static/derived residual**: **5,273** held `static`/`derived` units corpus-wide
   (`equipment 2,284`, `monster 1,232`, `spell 1,061`, `companion 322`, `monster_ability 304`,
   `class_feature 33`, `equipment_modifier 19`, `feat 17`, `race_trait 1`) — matches the plan's B4
   figure exactly. Fixture coverage: **94 entries** (`tests/fixtures/rules_core/derived-evaluator-
   fixtures.json`, `entries` array length). Held-derived eligible population re-derived at **2,792**,
   not the **2,879** the code comments at `src/bin/v06_work_inventory.rs:4585` and
   `derived_evaluator_fixture_check.rs:14` state (stale, corpus drift since authored) — correction
   emitted (below).
4. **AT-31-010 widening**: `display|grounded` corpus-wide = **1,243** units, all `held`
   (`monster_ability 981`, `companion 182`, `class_feature 54`, `race_trait 23`, `feat 3`) — matches
   the plan figure exactly. Command in `acceptance-and-verification.md` AT-31-010.
5. **Epic 5-F4 the 36 deferred units**: re-derived at **36** (34 `class_feature`, 2 `feat`), full
   per-book breakdown and per-unit `id` list recorded in `epic-breakdown.md` Epic 5-F4 — matches the
   plan's "34 class_feature, 2 feat" exactly.

**Package changes:**

- `epic-breakdown.md` Epic 6: F1 (`monster`) rewritten as a fixture-coverage lane with the
  re-derivation and a correction note; F5 `equipment`, F6 `equipment_modifier`, F7 `companion`, F8
  `feat` (routes the SD-30 E0-F3 217-unit probe-fixture residue — `docs/release/SD-30-class-feature-
  archetype-bundle/artifacts/sd30-e0-f3-unknown-residue/`, cited by its own bucket breakdown: 194
  `PREABILITY` chooser-prereq + 23 `PRESTAT`/`PRESKILL` = 217, remedy = widen `PROBE_CLASSES`/
  `PROBE_SELECTIONS` and `feat_probe_input`'s stripped-fixture shape in `src/bin/v06_work_inventory.
  rs:128/138/1560`), F9 `monster_ability`, F10 `class` added, each with its ladder, shape, moving
  instrument/lane, and acceptance; F11 held static/derived residual added, owner named (extends
  `corpus_literal_sweep` + `derived-evaluator-fixtures.json`). Epic 5 gains F4 (the 36 deferred
  units, full list, disposition per unit: 6 to a named "build the missing consumer or propose
  exclusion" path, 27 to the existing option-pool/Tier-1 disposition already satisfied, 1 (`brawler`)
  to a **PROPOSED** (unsigned) Structural Exclusion Register entry, 2 to an ordinary transcription
  fix). Epic 2-F3 and the Completion gate gain the AT-31-010 widening cross-reference. Epic 6/Epic 5
  bullets in the Completion gate updated to name the new seeds.
- `acceptance-and-verification.md`: AT-31-010 widened to also bind the 1,243-unit `display|grounded`
  population (Decision 1(e)'s other named target), with its own Given/When/Then and re-derivation
  command; exit-gate checklist line updated.
- `kanban.md`: `epic-6-ingest-lanes` row lists F1-F11 with the rewrite/new-seed notes;
  `epic-5-chassis-sweep` row gains F4.
- `forward-scope-register.md` G1.3: struck through in place (original text preserved), superseded
  with a dated note pointing to Epic 6 F5-F11 and Epic 5-F4 — doc convention observed, not silently
  rewritten.
- `risks-and-open-questions.md` open question 1: struck through in place, resolved with the same
  dated cross-reference.
- `README.md` In-scope: six-kind bullet added, each kind's not-done figure cited.

**PROPOSED Structural Exclusion Register entry (not signed — a cycle may only propose, per
`decisions.md §3`):** `advanced_class_guide:class_feature:brawler` — missing capability: transient
combat-state representation (helpless/immobilized flags feeding AC-bonus suppression). Recorded in
`epic-breakdown.md` Epic 5-F4 with all four `AT-31-100` fields except operator sign-off; **not**
copied into `acceptance-and-verification.md`'s live register table (that table's own convention is
signed entries only — this cycle's proposal lives with its full reasoning in Epic 5-F4 and awaits
either a real consumer or operator sign-off before promotion to the register proper).

**Retro.** Two corrections emitted (`docs/retro/events/sd31-ready-s2.jsonl`):
- `1786803206624-sd31-ready-s2-19eed7` — Epic 6-F1's pre-remediation "1,242 grounded / 7 done, 0.6 %"
  ingest framing → actual: 1,229 of 1,235 held units are the single cell `derived|grounded`, a
  fixture-coverage gap.
- `1786803206746-sd31-ready-s2-919f72` — the "94 of 2,879" held-derived comment in
  `v06_work_inventory.rs:4585` / `derived_evaluator_fixture_check.rs:14` → actual 2,792 (corpus
  drift), fixture entries confirmed at 94 (unchanged).

**Verification.** Doc-only step; ran `./scripts/verify.sh --only preflight-disk` only (no Rust/
Python/shell production code changed):
```
==> preflight-disk — disk budget check before any build starts
    repo filesystem (/home/ubuntu/workspace/repos/codex, mounted at /): 40% used, 580G available
    scratch-log filesystem (/tmp/codex-verify-J4Red8, mounted at /): 40% used, 580G available
    PASS  preflight-disk  (disk budget OK)
SUMMARY
  passed:  1  preflight-disk
RESULT: PASS
```
`VERIFY_EXIT=0`.

**Not independently re-derived this cycle:** the SD-30 E0-F3 artifact's own 217/100/50-unit
`feat`-residue bucket split (194 `PREABILITY` + 23 `PRESTAT`/`PRESKILL` = 217; 68+16+16=100
option-pool; 50 unclustered) is cited from that artifact's own committed JSON/README, not
independently re-run this cycle — the artifact's own classifier script
(`characterize_feat_unknown.py`) was read but not re-executed. The corpus-wide `feat` `unknown` total
it depends on (367) *was* independently re-derived this cycle (§ figures above, feat ladder) and
matches the artifact's own count exactly, which is the cross-check available without re-running a
script against the external PCGen oracle tree.

**Files changed:** `epic-breakdown.md`, `acceptance-and-verification.md`, `kanban.md`,
`forward-scope-register.md`, `risks-and-open-questions.md`, `README.md`,
`docs/retro/events/sd31-ready-s2.jsonl` (+2 correction events), this file.

**Status:** complete for this step's stated scope. Steps 3-6 of the plan (oracle pin, dashboard
import, drift sweep D1-D14, pre-launch checklist + adversarial verify) are separate steps, not owned
by this receipt.
