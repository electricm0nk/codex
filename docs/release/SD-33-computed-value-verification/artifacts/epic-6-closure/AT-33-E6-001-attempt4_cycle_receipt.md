# Cycle AT-33-E6-001 (attempt 4) — Epic 6 Closure epilogue / AT-33-E6-001

- **Commit SHA:** recorded on landing (see `progress.md` entry `AT-33-E6-001-attempt4`)
- **Files touched:** `docs/release/SD-33-computed-value-verification/artifacts/epic-6-closure/AT-33-E6-001-attempt4_cycle_receipt.md`, `docs/release/SD-33-computed-value-verification/progress.md`, `docs/release/SD-33-computed-value-verification/kanban.md`
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** *(verbatim, `epic-breakdown.md`)* "Every criterion `AT-33-E1-001` … `AT-33-E5-003` is `complete`, and every `kanban.md` card is `complete`. **A card at `returned-to-backlog`, `in-progress`, or `complete`-with-a-deferred-half blocks closure.**"

## Verdict: **FAIL** — the bundle is short. No retrospective, no sweep, no PR.

This is attempt 4. Attempts 1, 2 and 3 also failed, each correctly. Remediation
wave 3 closed a large part of attempt 3's shortfall but did not close it, and
introduced a second, new shortfall.

## Shortfall 1 — 75 of the 8,330 blessed units carry no oracle row

Attempt 3 found 391 of 8,330 unrowed. Wave 3's three shape lanes moved that to
75 of 8,330. That is real movement and it is not closure.

Re-derived by set difference, not by count — the `literal-verified` plus
`fixture-verified` id set from `docs/work-inventory.json` minus every `unit_id`
in the merged results file:

```
$ python3 -c "
import json,collections
wi=json.load(open('docs/work-inventory.json'))['units']
lit={u['id'] for u in wi if u.get('status')=='literal-verified'}
fix={u['id'] for u in wi if u.get('status')=='fixture-verified'}
d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json'))
miss=sorted((lit|fix)-{r['unit_id'] for r in d['results']})
print(len(lit),len(fix),len(miss))
print(dict(collections.Counter(m.split(':')[1] for m in miss)))"
6589 1741 75
{'equipment': 61, 'equipment_modifier': 14}
```

Row counts, all three results files, against their stated denominators:

| file | rows | distinct `unit_id` | denominator | short by |
|---|---|---|---|---|
| `fixture-verified.combined-oracle-results.json` | 1,741 | 1,741 | 1,741 | 0 — **CLOSED** |
| `literal-verified.oracle-results.json` | 6,514 | 6,514 | 6,589 | 75 |
| `AT-33-E5-003.combined-oracle-results.json` | 8,255 | 8,255 | 8,330 | 75 |

```
$ python3 -c "
import json,collections
for p,pop in [('fixture-verified.combined-oracle-results.json',1741),
              ('literal-verified.oracle-results.json',6589),
              ('AT-33-E5-003.combined-oracle-results.json',8330)]:
  d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/'+p))
  r=d['results']; print(p,'rows',len(r),'distinct',len({x['unit_id'] for x in r}),'pop',pop,
    dict(collections.Counter(x['verdict'] for x in r)))"
fixture-verified.combined-oracle-results.json rows 1741 distinct 1741 pop 1741 {'agree': 396, 'unverifiable': 1345}
literal-verified.oracle-results.json rows 6514 distinct 6514 pop 6589 {'agree': 339, 'unverifiable': 6149, 'disagree': 26}
AT-33-E5-003.combined-oracle-results.json rows 8255 distinct 8255 pop 8330 {'agree': 735, 'unverifiable': 7494, 'disagree': 26}
```

`kanban.md` row 17 (`AT-33-E5-002`) is `in-progress` and states this gap
honestly. That honesty is correct conduct and it is still a blocking shortfall
under §11 step 1: there is no "complete or filed".

The 75 ids are enumerated in
`artifacts/epic-5-reverification/finalize-wave3-missing-literal-shapes.json` and
reproduced by the command above. Their shapes: `WEAPON` 23, `SKILL` 17,
`WEAPONPROF` 15, `COMBAT` 7, `VAR` 5, `EQMWEAPON` 3, `SITUATION` 2, `EQM` 1,
`MOVEADD` 1, `STAT` 1 — of 75. Wave 3's `combat-weapon-shape` lane returned
`blocked-escalated` at 82 of its own 125-unit population and named the 43 it did
not reach; the residual is that lane's honest remainder plus the `SKILL`-shape
resolver gap the `var` lane named.

## Shortfall 2 — 26 unresolved `disagree` rows, NEW this wave

Attempt 3 recorded 0 `disagree`. Wave 3's lanes produced 26 of 8,255 examined
units at `disagree`. All 26 are root-caused in `progress.md`; **none is fixed and
none is escalated to an operator ruling.** `AT-33-E5-003` reads, verbatim: "every
disagreement is a named defect, **fixed or escalated**" and "A filed blocker does
not satisfy this criterion."

Root-cause is genuinely named and genuinely open: 21 of the 26 share one engine
gap — `compute_arms_armor_effect` / `compute_var_effect` do not resolve and sum a
base item's `EQMOD:`-referenced modifier record's own separate `BONUS:` chain;
3 are a harness baseline-diff methodology limit; 1 is an unhandled `PRE`-gated
conditional chain; 1 is undiagnosed. A named root cause is not a fix.

The fail-closed instrument agrees, and it is not the default invocation that
proves it — the default oracle-results path is Epic 2's, which does not exist,
so the bare `--check` reports `oracle_disagreement=0` and exits 0. Pointed at the
real merged file it fires:

```
$ python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json
uncovered=0 overlap=0 population=49438 oracle_disagreement=26 unverifiable_done=0 stale=False
ORACLE_DISAGREEMENT: inner_sea_races:equipment:armor_of_grim_triumph, ... (+16 more)
$ echo $?
1
```

This doubles as the live proof the disagree path still has teeth: it returns
`disagree` on 26 real cases from the current, unmodified batch path.

**Instrument observation, not a blocking finding:** `box_ledger.py --check` with
no `--oracle-results` argument exits 0 while 26 real disagreements stand, because
`DEFAULT_ORACLE_RESULTS` points at `artifacts/epic-2-oracle-harness/oracle-results.json`,
a file Epic 2 never produced. The gate is wired but its default aim misses the
bundle's own evidence. Recommend re-pointing `DEFAULT_ORACLE_RESULTS` at
`artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json` in the
cycle that fixes the engine gap, so the closure instrument fails closed by default
rather than on request.

## Shortfall 3 — kanban cards not at `complete`

```
$ python3 - <<'PY'
import re
for l in open('docs/release/SD-33-computed-value-verification/kanban.md'):
    m=re.match(r'\|\s*(\d+)\s*\|\s*`([^`]+)`\s*\|\s*(\d+)\s*\|\s*([A-Z0-9-]+)\s*\|\s*([a-z-]+)',l)
    if m and m.group(5)!='complete': print(m.group(1),m.group(2),m.group(4),m.group(5))
PY
17 reverify-literal-verified AT-33-E5-002 in-progress
18 disagreement-resolution AT-33-E5-003 in-progress
19 final-acceptance-scan AT-33-E6-001 blocked-escalated
20 retrospective-written-and-cited AT-33-E6-002 not-started
21 sweep-archdocs-graphify-pr AT-33-E6-003 not-started
```

Rows 1–16 are `complete`. Rows 17 and 18 are Epic 5 and block. Rows 19–21 are
Epic 6's own and are gated on rows 17/18.

## What attempt 3 flagged and wave 3 CLOSED — re-verified, not re-investigated

**Row 16 at 1,741 of 1,741.** Confirmed above: 1,741 rows, 1,741 distinct
`unit_id`, 396 agree / 1,345 unverifiable / 0 disagree.

**Zero reasonless `unverifiable`, zero duplicate `unit_id`.** Every
`unverifiable` row across all three files carries a populated `reason`:

```
$ python3 -c "
import json
b='docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/'
for f in ['fixture-verified.combined-oracle-results.json','literal-verified.oracle-results.json','AT-33-E5-003.combined-oracle-results.json']:
    r=json.load(open(b+f))['results']
    bad=[x for x in r if x['verdict']=='unverifiable' and not (x.get('reason') or '').strip()]
    ids=[x['unit_id'] for x in r]
    print(f,len(r),'reasonless',len(bad),'dupes',len(ids)-len(set(ids)))"
fixture-verified.combined-oracle-results.json 1741 reasonless 0 dupes 0
literal-verified.oracle-results.json 6514 reasonless 0 dupes 0
AT-33-E5-003.combined-oracle-results.json 8255 reasonless 0 dupes 0
```

**Denominator gate: scope genuinely widened, teeth intact.** `DEFAULT_GLOBS` now
carries the receipts glob, `progress.md`, and seven bundle-root documents
(`README.md`, `decisions.md`, `epic-breakdown.md`, `release-notes.md`,
`scope-draft.md`, `kanban.md`, `THE-BOX.md`). It was not narrowed back. The only
matcher change in the widening commit `14b42a4a1d` is one added exemption,
`FALSE_100_IDIOM_RE`:

```
FALSE_100_IDIOM_RE = re.compile(r"\bfalse[\s-]100%", re.IGNORECASE)
```

It blanks the literal idiom naming the anti-pattern — the phrase, not a measured
figure — and blanks only that one token of the line, leaving a genuine percentage
on the same line still caught (its own unit test
`test_idiom_does_not_shadow_a_real_percentage_on_the_same_line` proves that). Detection re-proven live inside the real widened scope, then the probe
removed:

```
$ printf 'probe: coverage reached 41%% this wave\n' > .../artifacts/epic-6-closure/_probe_cycle_receipt.md
$ python3 scripts/denominator_gate.py --check | tail -3
VIOLATION .../_probe_cycle_receipt.md:1: probe: coverage reached 41% this wave
files_checked=37
violations=1

$ printf 'probe: coverage reached 100%%\n' > .../_probe_cycle_receipt.md
$ python3 scripts/denominator_gate.py --check | tail -3
VIOLATION .../_probe_cycle_receipt.md:1: probe: coverage reached 100%
files_checked=37
violations=1

$ printf 'probe: 8,255 of 8,330 units examined\n' > .../_probe_cycle_receipt.md
$ python3 scripts/denominator_gate.py --check | tail -2
files_checked=37
violations=0

$ rm .../_probe_cycle_receipt.md && bash scripts/verify.sh --only denominator-gate
    PASS  denominator-gate  (files_checked=36 violations=0)
RESULT: PASS
```

A bare percentage fails, a bare hundred-percent token fails, the corrected form
passes, baseline is 0 violations of 36 files checked.

**No carve-outs hidden in closure instruments.**

```
$ grep -nE "EXCLUD|SKIP_|IGNORE_|_BOOKS *=|EXEMPT" scripts/box_ledger.py scripts/denominator_gate.py scripts/verify.sh scripts/shape_ledger.py scripts/coverage_ledger.py
scripts/coverage_ledger.py:207:    excluded_books: frozenset[str] = frozenset(P.EXCLUDED_BOOKS),
scripts/shape_ledger.py:109:# EXCLUDED_BOOKS, same doneness_verdict) so Gate 1's population matches the
$ grep -n "^EXCLUDED_BOOKS" scripts/observer/pf1e_dashboard_producer.py
3519:EXCLUDED_BOOKS: frozenset[str] = frozenset()
```

The one referenced list is empty at source. No book is carved out.

**Work inventory has no `unknown`.**
```
$ jq '[.units[]|select(.status=="unknown")]|length' docs/work-inventory.json
0
```

**Epic 3's artifact at the SD-33 path; SD-32's untouched.**
```
$ ls -la docs/release/SD-33-.../artifacts/epic-3-engine-coverage/formula_interpreter.corpus-wide.json
-rw-rw-r-- 1 ubuntu ubuntu 18860 Aug 25 00:49 ...
$ git log --oneline -1 -- docs/release/SD-32-*/artifacts/gate-2-engines/
d5cbf1f801 docs(sd32): card 10 epic-1-compute-library complete — kanban + progress receipt
```
SD-32's evidence directory's last touch is an SD-32 commit. No SD-33 commit
touched it.

**Deferral posture: 2 open of 8 recorded, 0 covering DoD scope.** Both open
deferrals carry a named revisit condition and both are capability deferrals
outside the Definition of Done — widening `REGISTERED_POOL_GROUPS` for
`not-ingested` `class_feature` units (never in SD-33's DoD), and a one-line
`unmeasurable` recognition in `pf1e_dashboard_producer.py` (outside AT-33-E4's
granted write scope, disclosed rather than silently skipped).
```
$ python3 scripts/retro.py summary --since 2026-08-24 --json | python3 -c "import json,sys;d=json.load(sys.stdin)['deferrals'];print('open',d['open'])"
open 2
```

**Every kanban-cited receipt exists and carries §7's figures and four-buckets rows.**
```
$ python3 -c "
import re,os
paths=sorted(set(re.findall(r'artifacts/[A-Za-z0-9./_-]*_cycle_receipt\.md',open('kanban.md').read())))
print(len(paths),'cited', len([p for p in paths if not os.path.isfile(p)]),'missing')"
20 cited 0 missing
$ for f in artifacts/*/*_cycle_receipt.md; do grep -q "Figures + their re-derive commands" "$f" && grep -q "Movement, four buckets" "$f" || echo "SHORT $f"; done
(no output)
```

## Figures + their re-derive commands

Every command is inline above, beside its figure. Denominators: 8,330 blessed
units (1,741 fixture + 6,589 literal); 8,255 of 8,330 rowed; 75 of 8,330 unrowed;
26 of 8,255 examined at `disagree`; 21 of those 26 on one named engine gap;
16 of 21 kanban cards at `complete`; 2 of 8 deferrals open; 36 of 36 gate-scoped
files at 0 violations.

## Status: blocked-escalated

## Movement, four buckets

- **Closure:** none banked by this cycle. This cycle writes no code and verifies only.
- **Reclassification:** none.
- **Reachability:** confirms wave 3 moved rowed units from 7,939 of 8,330 to 8,255 of 8,330 — 316 units of real forward movement, independently re-derived here.
- **Instrument-correction:** one found — `box_ledger.py`'s `DEFAULT_ORACLE_RESULTS` aims at a path Epic 2 never produced, so the bare `--check` exits 0 while 26 real disagreements stand. Reported, not fixed (this is a scanning cycle).

## Notes

Wave 3's lanes reported honestly. The `combat-weapon-shape` lane returned
`blocked-escalated` at 82 of its 125-unit population rather than claiming
`complete`, and the finalize cycle kept rows 17 and 18 `in-progress` rather than
rounding the 75-unit gap away. That is exactly the conduct the wave-2 over-claim
made necessary, and it is why this scan can state the shortfall precisely instead
of discovering it.

Two distinct things remain, and they are different kinds of work: rowing 75
units (a throughput remainder, mostly `WEAPON`/`SKILL`/`WEAPONPROF` shapes, one
of which needs the `equipment_id_resolve` widening the `var` lane named), and
fixing 26 disagreements (one real engine gap under 21 of them). Neither is a
judgment call; both are cycles.

## Next-cycle plan

1. Fix the engine gap under 21 of the 26 disagreements — resolve and sum a base
   item's `EQMOD:`-referenced modifier record's own `BONUS:` chain in
   `compute_arms_armor_effect` / `compute_var_effect`, TDD from corpus-verbatim
   fixtures; then re-run every unit the harness already judged, per
   `AT-33-E5-003`'s own re-run clause.
2. Diagnose and resolve the remaining 5 of 26 (3 harness baseline-diff, 1
   `PRE`-gated chain, 1 undiagnosed).
3. Row the 75 of 8,330 — widen `equipment_id_resolve` to match a JSON corpus
   record's own `key` field for keyless `OUTPUTNAME` records, then run the
   `WEAPON`/`WEAPONPROF`/`SKILL`/`COMBAT` tail with the `var` lane's
   book-homogeneous batching.
4. Re-point `box_ledger.py`'s `DEFAULT_ORACLE_RESULTS` at the merged Epic 5 file.
5. Re-run AT-33-E6-001 as attempt 5.
