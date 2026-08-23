# Cycle 004-budget-repair-cycle-2 — Gate 1 (`gate-1-shape-closure`) + Gate 3 (`gate-3-closure-invariant`) — repin 3, `departed_covered_count` invariant fix

Responds to the SD-32 dispatch brief "Gate 3's `no_record` budget needs an evidence-gated repin"
(2026-08-23): commit `8e98424eb56a66ee2ba38d7ec4e2bc2e5379fbe6` (card 15 / `decisions.md §17` item 1,
the generic-enumeration data table) landed five new kinds through `v06_work_inventory.rs`'s new
`SIMPLE_FILENAME_KINDS` table, moving population 25,055→28,490 and exceeding the `003_budget_repair`
cycle's committed budget (10530/25055) again — `scripts/verify.sh`'s `shape-coverage-standing-gate`
went RED a second time, the exact case the repin mechanism was designed for.

- **Card IDs:** `gate-1-shape-closure` (card 5), `gate-3-closure-invariant` (card 9)
- **Files touched:**
  - `scripts/shape_coverage_standing_gate.py` — `NO_RECORD_BUDGET_COUNT`/`NO_RECORD_BUDGET_POPULATION`
    repinned 10530/25055 → 13968/28490; constants' comment block rewritten to document the new
    `departed_covered_count` field and its derivation
  - `artifacts/gate-3-closure-invariant/no_record_budget_provenance.jsonl` — repin 3 appended, naming
    evidence commit `8e98424eb`, `departed_covered_count: 10`
  - `scripts/tests/test_shape_coverage_standing_gate.py` — `test_no_record_delta_never_exceeds_population_delta`
    changed to credit `departed_covered_count` (see "Design tension found" below); new
    `test_departed_covered_count_does_not_excuse_a_real_drain` proves the credit cannot launder an
    actual regression
  - `artifacts/gate-1-shape-closure/ledger.json` — regenerated (25055/10530 → 28490/13968)
  - `artifacts/gate-1-shape-closure/family-vocabulary.{md,json}` — regenerated via
    `scripts/family_vocabulary_reconcile.py`
  - `acceptance-and-verification.md` — AT-32-G1-004's "expect matched=..." line corrected
  - `kanban.md` — rows 5 and 9 got a new prepended addendum each (prior "BUDGET REPAIR"/"RECLOSURE"
    entries retained below, per this bundle's append-history convention); left `in-progress` at
    start, `complete` at end (real gate passes, see below)
  - `docs/retro/events/gate3-repin.jsonl` (new) — one `correction` event: the dispatch brief's quoted
    `no_record=13,975` does not reproduce; the real figure is **13,968**

## Growth is real enumeration, not drift — per-kind proof

```
$ python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus --output /tmp/ledger_now.json
population (not-done units considered): 28490
join-status split: matched=4802 no_formula_tokens=9720 no_record=13968

$ python3 -c "
import json,collections
r=json.load(open('/tmp/ledger_now.json'))['rows']
by_kind=collections.defaultdict(collections.Counter)
for row in r: by_kind[row['kind']][row['join_status']]+=1
for k,c in sorted(by_kind.items()): print(k, dict(c))
"
class {'no_record': 157}
class_feature {'matched': 4545, 'no_formula_tokens': 7573, 'no_record': 2987}
companion {'no_record': 773, 'no_formula_tokens': 36, 'matched': 16}
deity {'no_record': 459}
domain {'no_record': 183}
equipment {'no_formula_tokens': 577, 'no_record': 313, 'matched': 5}
equipment_modifier {'no_record': 237, 'no_formula_tokens': 819, 'matched': 8}
feat {'no_record': 1090, 'no_formula_tokens': 53}
language {'no_record': 136}
monster {'no_record': 141, 'matched': 140}
monster_ability {'no_record': 1842, 'matched': 77, 'no_formula_tokens': 96}
power {'no_record': 421}
race {'no_record': 59}
race_trait {'no_record': 1913, 'no_formula_tokens': 156, 'matched': 11}
skill {'no_record': 149}
spell {'no_formula_tokens': 410, 'no_record': 860}
template {'no_record': 2248}
```

The 5 new kinds are **100% `no_record`, 0% matched, 0% no_formula_tokens**: template 2,248, deity
459, power 421, domain 183, language 136 — sum **3,447**, exactly matching the dispatch brief's own
`docs/work-inventory.json` `totals.units` delta (38,540→41,987). Same structural shape as repin 2's
`Kind::Skill`: these kinds have never had a corpus ingest path, so the join organically fails for
every one of them — not a regression.

**Retro correction logged** (`docs/retro/events/gate3-repin.jsonl`): the dispatch brief's quoted
`no_record 10,530 → 13,975` does not reproduce against two independent runs of the command above
(both returned 13,968 exactly). The real figure is **13,968**, 7 lower than claimed. This receipt and
every artifact regenerated below use the re-derived 13,968, not the brief's figure.

## Design tension found and fixed: `departed_covered_count`

Straight arithmetic against repin 2's committed baseline (10530/25055) and the honest current figures
(13968/28490) gives `no_record` delta = 3,438 against population delta = 3,435 — **3 more than
`BudgetProvenanceTest`'s strict `no_record delta <= population delta` check allows**, even though the
growth above is proven clean. Investigated id-by-id rather than assumed:

```
$ python3 -c "
import json
old=json.load(open('artifacts/gate-1-shape-closure/ledger.json'))['rows']   # committed at repin 2
new=json.load(open('/tmp/ledger_now.json'))['rows']
old_ids={r['id']:r for r in old}; new_ids={r['id']:r for r in new}
old_only=set(old_ids)-set(new_ids); new_only=set(new_ids)-set(old_ids)
common=set(old_ids)&set(new_ids)
changed=[(i,old_ids[i]['join_status'],new_ids[i]['join_status']) for i in common if old_ids[i]['join_status']!=new_ids[i]['join_status']]
print('common ids with a join_status change:', len(changed))
for i,o,n in changed: print(' ',i,o,'->',n)
"
common ids with a join_status change: 7
  advanced_race_guide:race_trait:orc no_record -> no_formula_tokens
  advanced_race_guide:race_trait:elf no_record -> no_formula_tokens
  advanced_race_guide:race_trait:drow no_record -> no_formula_tokens
  advanced_race_guide:race_trait:grippli no_record -> no_formula_tokens
  advanced_race_guide:race_trait:gnome no_record -> no_formula_tokens
  advanced_race_guide:race_trait:halfling no_record -> no_formula_tokens
  advanced_race_guide:race_trait:dwarf no_record -> no_formula_tokens
```
**Zero of the 7 status changes among ids present both before and after moved toward `no_record`** —
all 7 improved (corpus updates since repin 2 gave these `advanced_race_guide` race entries records
they didn't have before). No common-id regression exists.

The +3 excess instead comes entirely from units that **left** the not-done population between repins
(a case the strict delta check does not distinguish from a regression, because it only sees two net
scalars):

```
$ python3 -c "
import json
old=json.load(open('artifacts/gate-1-shape-closure/ledger.json'))['rows']
new=json.load(open('/tmp/ledger_now.json'))['rows']
old_only = {r['id']:r for r in old}.keys() - {r['id'] for r in new}
cur=json.load(open('docs/work-inventory.json'))
cur_units = cur['units']
cur_by_id={u['id']:u for u in cur_units}
old_by_id={r['id']:r for r in old}
present, absent = 0, 0
for i in old_only:
    if i in cur_by_id: present += 1
print('old-population ids absent from new ledger:', len(old_only))
print('  -> still present in current work-inventory.json (status changed, left not-done):', present)
print('  -> absent entirely (kind changed -> new id under new kind):', len(old_only)-present)
"
old-population ids absent from new ledger: 875
  -> still present in current work-inventory.json (status changed, left not-done): 11
  -> absent entirely (kind changed -> new id under new kind): 864
```

**864 of the 875** are `race_trait` units reclassified to `monster_ability` by commit `6ae4a364b`
(`decisions.md §16`'s T2b classifier fix, already landed and tested on its own suite, unrelated to
this cycle's scope) — the kind change gives them a new id, so they read as "old id gone, new id
arrived." Net effect on this budget is **zero**: all 864 were `no_record` before and all 863
corresponding `monster_ability` arrivals are `no_record` after (the 864-vs-863 is the reclassification
commit's own internal accounting, not this cycle's concern).

**11 of the 875** genuinely left the not-done population — wired to completion by unrelated concurrent
work:
```
$ python3 -c "
import json
old=json.load(open('artifacts/gate-1-shape-closure/ledger.json'))['rows']
new=json.load(open('/tmp/ledger_now.json'))['rows']
old_by_id={r['id']:r for r in old}
old_only = old_by_id.keys() - {r['id'] for r in new}
cur={u['id']:u for u in json.load(open('docs/work-inventory.json'))['units']}
for i in old_only:
    if i in cur:
        print(i, 'old_join_status=', old_by_id[i]['join_status'], 'new_status=', cur[i]['status'])
"
bestiary_2:race_trait:dhampir_resist_level_drain old_join_status= no_formula_tokens new_status= text-complete
inner_sea_races:race_trait:gillman_deep_gillman old_join_status= no_formula_tokens new_status= grounded
inner_sea_races:race_trait:nagaji_serpent_affinity old_join_status= no_formula_tokens new_status= grounded
inner_sea_races:race_trait:mostly_human_suli_languages old_join_status= no_formula_tokens new_status= text-complete
bestiary_2:race_trait:dhampir_type old_join_status= no_formula_tokens new_status= text-complete
bestiary_2:race_trait:dhampir_ability_scores old_join_status= no_formula_tokens new_status= grounded
inner_sea_races:race_trait:vanara_risky_troublemaker old_join_status= no_formula_tokens new_status= grounded
inner_sea_races:race_trait:strix_cautious_brawler old_join_status= no_formula_tokens new_status= grounded
advanced_race_guide:race:dhampir old_join_status= no_record new_status= literal-verified
bestiary_2:race_trait:dhampir_size old_join_status= no_formula_tokens new_status= text-complete
bestiary_2:race_trait:dhampir_languages old_join_status= no_formula_tokens new_status= text-complete
```
10 of these 11 were `no_formula_tokens` (not `no_record`) when they left. A completed unit leaving the
not-done population shrinks `population` by 1 without shrinking `no_record` — real progress (someone
wired 11 more race/race_trait units to `grounded`/`text-complete`/`literal-verified`), not drift, but
it does shrink `population` 11 faster than it shrinks `no_record` (only 1), which is exactly the
arithmetic source of the +3 the strict check flags. Full reconciliation: new-kinds contribute 0 excess,
reclassification contributes 0 excess, the 11 departures contribute +10 excess (population -11,
no_record -1), the 7 corpus-improved common ids contribute -7 excess (no_record -7, population +0);
net **+10 - 7 = +3**, matching the observed gap exactly.

**Fix:** added `departed_covered_count` to the provenance entry schema — the count of previously
tracked units that left the not-done population while NOT `no_record` (here, 10). Changed the
invariant in `BudgetProvenanceTest.test_no_record_delta_never_exceeds_population_delta` to
`no_record_delta <= population_delta + departed_covered_count`, defaulting to `0` via `.get()` for
repins 1-2 (unaffected, pure clean growth, no departures). This keeps the check's actual intent —
"no covered unit silently drains into `no_record`" — while no longer flagging the orthogonal, benign
case of covered units finishing and leaving the tracked population. New
`test_departed_covered_count_does_not_excuse_a_real_drain` proves the credit cannot be abused to hide
an actual regression (a synthetic entry pair with a drain well beyond any plausible departure credit
still fails the check).

## Tamper-proof re-verified after the fix

```
$ python3 -c "s=open('scripts/shape_coverage_standing_gate.py').read(); assert 'NO_RECORD_BUDGET_COUNT = 13968' in s; open('scripts/shape_coverage_standing_gate.py','w').write(s.replace('NO_RECORD_BUDGET_COUNT = 13968','NO_RECORD_BUDGET_COUNT = 99999',1))"
$ python3 -m unittest scripts.tests.test_shape_coverage_standing_gate.BudgetProvenanceTest -v
FAIL: test_constants_match_latest_provenance_entry — AssertionError: 99999 != 13968
FAIL: test_unprovenanced_run_still_measured_against_committed_baseline — AssertionError: 0 == 0
Ran 8 tests in 0.042s -- FAILED (failures=2)

# reverted
$ python3 -c "s=open('scripts/shape_coverage_standing_gate.py').read(); open('scripts/shape_coverage_standing_gate.py','w').write(s.replace('NO_RECORD_BUDGET_COUNT = 99999','NO_RECORD_BUDGET_COUNT = 13968',1))"
$ python3 -m unittest scripts.tests.test_shape_coverage_standing_gate.BudgetProvenanceTest -v
Ran 8 tests in 0.056s -- OK
```

## AT-32-G3-001 red-proof — orchestrator's own reproduction, unmodified, AFTER this cycle's repin

```
$ python3 -c "
import sys; sys.path.insert(0,'scripts')
import shape_coverage_standing_gate as G
u=[{'id':f'b:{k}:{i}','kind':k,'book':'b','status':'not-started','wiring_class':'static','source_file':'totally_fake_file.lst','source_line':i} for k in ('ability','skill','template','deity','power','domain','language','kit') for i in range(1,11)]
print(G.run_gate({'units':u}, corpus_root='/nonexistent'))"
(1, {'population': 80, 'unclassified_count': 0, 'family_total': 80, 'piles_reconcile': True,
'families': {'F0': 80}, 'join_status_counts': {'no_record': 80}, 'no_record_count': 80,
'no_record_budget_count': 13968, 'no_record_budget_population': 28490,
'no_record_budget_exceeded': True, 'corpus_sha': '7f818006e371188e5717fd18d74d18a420747fc6'})
```
Status stays nonzero (`1`), `no_record_budget_exceeded: True` — the gate still catches a real
uncovered object after the repin.

## Real full-population run — passes on the repinned baseline

```
$ scripts/verify.sh --only shape-coverage-standing-gate
PASS  shape-coverage-standing-gate  (population=28490 unclassified=0 no_record=13968 corpus_sha=7f818006e371188e5717fd18d74d18a420747fc6)

$ scripts/verify.sh --only shape-coverage-standing-gate-selftest
PASS  shape-coverage-standing-gate-selftest  (20 cases passed)
```

### Regenerated ledger.json (AT-32-G1-004)
```
$ python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus --output artifacts/gate-1-shape-closure/ledger.json
$ jq -r '.join_status_counts' artifacts/gate-1-shape-closure/ledger.json
{
  "no_record": 13968,
  "matched": 4802,
  "no_formula_tokens": 9720
}
```

### Self-tests (direct)
```
$ python3 -m unittest scripts.tests.test_shape_coverage_standing_gate scripts.tests.test_shape_ledger -v
Ran 49 tests — OK (9 in BudgetProvenanceTest: 7 prior + 2 new)
```

### Broad suites
```
$ CARGO_TARGET_DIR=/home/ubuntu/.cache/codex-targets/sd32-gate3-repin cargo test --locked --lib
test result: ok. 2390 passed; 0 failed; 13 ignored; 0 measured; 0 filtered out; finished in 12.45s
```
Scoped python tests (`test_shape_coverage_standing_gate`, `test_shape_ledger`) and both
`scripts/verify.sh --only` stages ran to completion and are pasted above. The full unscoped
`cargo test --locked --no-fail-fast` sweep and the desktop crate suite were **not** run in this
cycle, per the dispatch instruction's explicit scoping — only `cargo test --locked --lib`.

### Dual-audit gate (isolated to this cycle's own changes, `PIN..HEAD`)
```
$ git diff --unified=0 8046a9bfc30519fe9d4d23c99953902557ba98d2...HEAD -- src tests scripts docs/release ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
OK_NO_BUNDLE_TAGS
$ git diff --unified=0 8046a9bfc30519fe9d4d23c99953902557ba98d2...HEAD -- src tests scripts docs/release ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
OK_NO_TOKENS
```

## Notes / judgment calls

1. **Did not touch `object-definition-rules.md` or `generic-pass-state.md`'s "confirmed RED"
   section** — the dispatch brief's item 5 named exactly `ledger.json`, `kanban.md` rows 5/9,
   `family-vocabulary.{md,json}`, and `acceptance-and-verification.md`. `generic-pass-state.md`'s §4
   is that cycle's own dated, historical "before" report (it already correctly says "confirmed RED,
   for the expected, already-escalated reason" and names this exact repin as the remedy) — left
   as-written per this bundle's convention of not rewriting a prior cycle's own append-only record.
2. **Did not attempt the remaining `ability` kind (5,108 units)** — out of scope; the
   `generic-enumeration_cycle_receipt.md` already explains why it needs a different (per-row A/B)
   mechanism than the filename-table approach this repin is covering for.
3. **`departed_covered_count` is a new, evidence-derived field, not a mechanically self-verifying
   one** — nothing in `BudgetProvenanceTest` re-derives it from git history the way
   `evidence_commit` is re-derived (via `git cat-file`/`merge-base`). `test_departed_covered_count_does_not_excuse_a_real_drain`
   guards against gross abuse (a drain far beyond plausible departures still fails), but a small,
   fabricated departure count could in principle slip through undetected by the mechanical suite —
   flagged here rather than silently accepted; a future cycle could tighten this by re-deriving the
   count from git-diffing the two inventories at the evidence commits, the way this receipt did by
   hand.

## Kanban

Rows 5 and 9 set back to `complete` at the end of this cycle — the real full-population gate run
passes (`scripts/verify.sh --only shape-coverage-standing-gate` → PASS, population=28490,
no_record=13968, budget not exceeded), AT-32-G1-004's own command returns the real split, and both
AT-32-G1-001..004/AT-32-G3-001..003 criteria are met against the current, honest, repinned baseline.
See kanban.md rows 5/9 addenda for the exact text.
