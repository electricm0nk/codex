# Cycle AT-33-E6-001 (attempt 7) — epic-6-closure / AT-33-E6-001

- **Commit SHA:** recorded on landing (see `progress.md` entry `sd33-r6-acceptance-scan`)
- **Scanned tree:** clean detached worktree at `origin/tranche/13` = `7d439876b7`
  (`.worktrees/sd33-r6-scan`). The shared checkout at `/home/ubuntu/workspace/repos/codex`
  was NOT used for the scan — see "Environment finding" below.
- **Files touched:** this receipt; `progress.md`; `kanban.md` (row 19 notes only, status stays
  `blocked-escalated`); `docs/retro/events/sd33-r6-acceptance-scan.jsonl`.

## Gate result: **FAIL** (attempt 7). Seventh consecutive correct halt. One shortfall.

Attempt 6 recorded **four** shortfalls. **3 of 4 are CLOSED this wave**, each verified by
execution below, and each behind real work rather than an edited expectation. **1 of 4
survives unchanged**: the Rust lib suite is still RED, and kanban row 14 is still `complete`
over a suite this bundle's own commit turned red.

The dispatch that launched this attempt named only three of attempt 6's four shortfalls.
Shortfall 4 was not in that list and was not in the "closed, do not re-litigate" list either.
It is inside this criterion's own stated scope ("the Rust suite is green for whatever
`src/rules_core/` changed this wave"; "`complete`-with-a-deferred-half BLOCKS"), so it is
scanned and reported here.

### Figures

| Figure | Value | Denominator | Re-derive |
|---|---:|---|---|
| Blessed units carrying an oracle row | 8,330 | of 8,330 blessed units | Check 1 command below |
| Units missing an oracle row | 0 | of 8,330 blessed units | Check 1 command below |
| `fixture-verified` rows | 1,741 | of 1,741 fixture-verified units | Check 1 command below |
| `literal-verified` rows | 6,589 | of 6,589 literal-verified units | Check 1 command below |
| Examined units at `disagree` | 0 | of 8,330 examined units | `box_ledger.py --check` below |
| Duplicate `unit_id` | 0 | of 8,330 rows | Check 1 command below |
| Reasonless `unverifiable` | 0 | of 7,519 unverifiable rows | consistency audit below |
| Method-change rows re-run | 21 | of 21 derived-affected rows | Check 3 below |
| AC-isolator re-run rows reaching the closure artifact | 66 | of 66 re-run rows | Check 3 below |
| Active `## Open blockers` entries | 0 | of 0 entries in that section | Check 2 below |
| Denominator gate | 0 violations | of 53 files checked | `verify.sh --only denominator-gate` |
| work-inventory `unknown` | 0 | of 49,438 work-inventory units | `jq` below |
| Kanban rows `complete` | 18 | of 18 rows 1-18 | `kanban.md` table |
| **Executed lib tests passing** | **2,832** | **of 2,836 executed lib tests** | **Shortfall 1 below** |
| **Executed lib tests failing** | **4** | **of 2,836 executed lib tests** | **Shortfall 1 below** |
| Corpus records | 7,808 | of 7,808 equipment/equipment_modifier records at the cut | corpus audit below |

### Four buckets

- **Closure 0** — no `docs/work-inventory.json` `status` field changed this cycle.
- **Reclassification 0** — no unit moved kind or population.
- **Reachability 0** — no unit newly rowed by this cycle (a scan does not row units).
- **Instrument-correction 0** — no instrument changed; two live detection probes were
  planted and removed (denominator gate, `box_ledger`), leaving no residue.

---

## Shortfall 1 (attempt 6's Shortfall 4, UNCLOSED) — the Rust lib suite is RED

```
$ cargo test --locked --lib
test result: FAILED. 2832 passed; 4 failed; 14 ignored; 0 measured; 0 filtered out; finished in 28.58s

failures:
    rules_core::equipment_resolver::tests::catalog_rows_span_every_ingested_book_with_their_real_counts
    rules_core::pilot_compute::formula_interpreter_corpus_wide::tests::a_subset_run_trips_the_population_mismatch_check
    rules_core::pilot_compute::formula_interpreter_corpus_wide::tests::corpus_wide_scan_population_matches_the_closed_gate1_census
    rules_core::pilot_compute::formula_interpreter_corpus_wide::tests::f1_population_matches_the_current_true_formula_bearing_count_not_the_stale_sd32_census
```

**2,832 of 2,836 executed lib tests pass; 4 of 2,836 fail.** The suite grew by 8 tests since
attempt 6 (2,828 executed then, 2,836 now — wave 6's new RED→GREEN tests), and the same 4
tests fail. Attributed per `AGENTS.md`'s "attribute every `test result: FAILED` line" rule —
not bucketed as environmental.

**3 of 4 are SD-33's own Epic 4 debt, unchanged.** All three share one panic, raised through
a real shell-out from the Rust test to `scripts/shape_ledger.py`:

```
ValueError: doneness: unmapped 'ambiguous' + 'unmeasurable'
  scripts/observer/pf1e_dashboard_producer.py:4031 _doneness_verdict_uncapped
  <- scripts/coverage_ledger.py:202 unit_verdict
  <- scripts/shape_ledger.py:981 main
```

The `(wiring_class='ambiguous', status='unmeasurable')` pair still exists on **11 of 49,438**
work-inventory units, and nothing has mapped it:

```
$ python3 -c "import json,collections
u=json.load(open('docs/work-inventory.json'))['units']
c=collections.Counter((x.get('wiring_class'),x.get('status')) for x in u if x.get('status')=='unmeasurable' or x.get('wiring_class')=='ambiguous')
print('ambiguous+unmeasurable:', c[('ambiguous','unmeasurable')], 'of', len(u), 'work-inventory units')"
ambiguous+unmeasurable: 11 of 49438 work-inventory units

$ git log --oneline f652db7ac7..HEAD -- docs/work-inventory.json
00ca087775 fix(sd33): AT-33-E4-002 -- 4,224 unknown units reclassified to zero

$ git log --oneline 9e90d0694e..HEAD -- scripts/observer/pf1e_dashboard_producer.py scripts/coverage_ledger.py scripts/shape_ledger.py
(empty — the mapper is untouched since attempt 6)
```

`docs/work-inventory.json` still has exactly one commit on this branch, `00ca087775`
(`AT-33-E4-002`), SD-33's own Epic 4 deliverable. **Kanban row 14 (`unknown-to-zero`,
`AT-33-E4-002`) is `complete` over a suite its own commit turned red**, and this criterion's
own words make that blocking: `complete`-with-a-deferred-half is not `complete`. It is the
`count-change-needs-a-sweep-not-just-a-build` shape — the reclassification compiled clean and
left another module's mapper unmapped. `progress.md`'s own status line concedes it in writing:
"Epic 4's own `cargo test --locked --lib` debt … is out of this row's scope and remains for
its own owner." Naming an owner is not closing a blocker.

**1 of 4 is inherited from the `tranche/13` cut, and still red.**
`catalog_rows_span_every_ingested_book_with_their_real_counts` asserts `left: 8119, right:
8100` at `src/rules_core/equipment_resolver.rs:863` — a hardcoded catalog count 19 above the
live corpus. Re-confirmed inherited rather than caused by wave 6's corpus regeneration: the
equipment/equipment_modifier record count is **7,808 of 7,808** unchanged from the cut and no
file was added or removed.

```
$ git ls-tree -r --name-only f652db7ac7 -- data/corpus | grep -E '/(equipment|equipment_modifier)/.*\.json$' | wc -l
7808
$ git ls-tree -r --name-only HEAD -- data/corpus | grep -E '/(equipment|equipment_modifier)/.*\.json$' | wc -l
7808
$ git diff --name-status f652db7ac7..HEAD -- data/corpus | cut -f1 | sort | uniq -c
    137 M
```

**The fix is one mapping entry plus a count reconciliation, both small and both named.** This
is a blocker, not a deferral: `AGENTS.md`'s own test — was this scope in the Definition of Done
when the work was scoped? — answers yes for row 14's own commit.

---

## Prior shortfalls CLOSED this wave (verified by execution, not by report)

### Attempt 6 Shortfall 1 — 39 of 8,330 unexamined → **CLOSED, 0 of 8,330 unexamined**

```
$ python3 -c "import json,collections
wi=json.load(open('docs/work-inventory.json'))['units']
pop={u['id'] for u in wi if u.get('status') in ('literal-verified','fixture-verified')}
for p,n in [('fixture-verified.combined-oracle-results.json',1741),
            ('literal-verified.oracle-results.json',6589),
            ('AT-33-E5-003.combined-oracle-results.json',8330)]:
  d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/'+p))
  k='results' if 'results' in d else [x for x in d if isinstance(d[x],list)][0]
  r=d[k]; ids=[x.get('unit_id') for x in r]
  print(p,'rows',len(r),'distinct',len(set(ids)),'pop',n,dict(collections.Counter(x.get('verdict') for x in r)))
c=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json'))['results']
miss=sorted(pop-{x['unit_id'] for x in c}); print('MISSING',len(miss))"

fixture-verified.combined-oracle-results.json rows 1741 distinct 1741 pop 1741 {'agree': 396, 'unverifiable': 1345}
literal-verified.oracle-results.json rows 6589 distinct 6589 pop 6589 {'agree': 415, 'unverifiable': 6174}
AT-33-E5-003.combined-oracle-results.json rows 8330 distinct 8330 pop 8330 {'agree': 811, 'unverifiable': 7519}
MISSING 0
```

Every population equals its denominator exactly; **0 duplicate `unit_id` of 8,330 rows**.
The 39 newly-rowed units (23 weapon-shape + 11 skill-combat-shape + 7 eqm-shape, with 2
unit_ids deduped across a weapon/skill-combat dispatch-partition overlap) break down
**7 `agree` and 32 `unverifiable` of 39 new rows**, every one carrying a populated reason.

### Attempt 6 Shortfall 2 — 1 of 8,291 `disagree` → **CLOSED, 0 of 8,330 `disagree`**

```
$ python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json; echo "EXIT=$?"
uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False
EXIT=0
```

`advanced_race_guide:equipment:rending_claw_blades` moved `disagree` → `agree` behind **two
real defects fixed in `src/rules_core/`**, diff read, not a report taken on trust:

1. `eqmod_referenced_records` read only a record's FIRST `EQMOD:` token via `.find()`. This
   record carries two, so the token naming the real `+1 Weapon` modifier was never inspected.
   Fixed to scan every `EQMOD:` token.
2. `compute_equipment_effects`'s weapon path never folded `EQMOD:`-referenced modifier
   records' own weapon-enhancement chains in at all, unlike the AC dimension's already-shipped
   `resolve_category_effect` pattern. Fixed via a new
   `equipmods::apply_eqmod_weapon_enhancement_bonus`.

The combining rule is per-dimension **MAX, not sum** — the base chain and the referenced
modifier's chain carry the identical `TYPE=Enhancement` qualifier, and Pathfinder's same-type
stacking rule takes the higher. The commit message records that a naive sum was written first
and traded the fixed disagreement for a new one; that is the correct discovery order, and the
test's own comment carries the corrected expectation rather than a silently-adjusted one.
RED→GREEN test `eqmod_referenced_modifier_sums_into_weapon_enhancement_bonus_across_two_eqmod_tokens`
uses real verbatim corpus tokens.

### Attempt 6 Shortfall 3 — 2 of 66 re-run rows unmerged → **CLOSED, 66 of 66 reaching the artifact**

```
$ python3 <compare full-rerun-wave5.oracle-results.json against the combined file, per row>
full-rerun-wave5.oracle-results.json rows 66
  not in combined: 0 []
  value mismatch vs combined: 0
```

All **66 of 66** wave-5 re-run rows are present in the closure artifact with byte-identical
`ours`/`oracle` values. `ring_of_unquenchable_passions` (ours 5→1, oracle 5→1) and
`goblin_plate` (ours 6→10, oracle 6→10) both moved on **both** sides, which is the signature
of a propagated method-corrected measurement, not of an expectation edited to fit.

---

## Check 2 — the `## Open blockers` section holds no entry

```
$ python3 <extract the real '## Open blockers' heading by line, strip <details> archives, count '###' entries>
section lines 279 to 365
ACTIVE ### entries outside <details>: 0
```

A naive `sed -n '/## Open blockers/,$p' progress.md` FALSE-MATCHES on the frontmatter `status:`
line, which quotes the string "`## Open blockers` is now empty" — it prints the whole document
and looks alarming. The real heading is at line 279. The section carries prose plus one
`<details>`-wrapped historical entry explicitly labelled "CLEARED … kept for audit trail", and
**0 of 0 active entries**. The archived entry is history, not a parked blocker: the thing it
requested a ruling on was fixed, and the fix is verified above.

## Check 4 — disagreements resolved, not hidden

Diffed every row of the closure artifact between attempt 6 (`9e90d0694e`) and `HEAD`:

- **Rows dropped: 0** of 8,291 carried-over rows.
- **`agree` → `disagree`: 0.** **`agree` → `unverifiable`: 0.** **`disagree` → `unverifiable`: 0.**
  No disagreeing unit was reclassified or deleted.
- **Verdict changed on 18 of 8,291** carried-over rows: 17 `unverifiable` → `agree`, 1
  `disagree` → `agree` (`rending_claw_blades`).
- **Oracle value changed on 20 of 8,291** carried-over rows. **18 of those 20 moved from
  `None` to a real value** — a first-time live oracle capture, which is not an edit. **2 of
  those 20** are the Shortfall-3 propagation, and both moved on both sides.

**The `ours` side was independently derived, not fitted to the oracle.** The 4 campaign-key
rows whose `ours` was corrected were cross-checked against the separately-committed engine
output file `combat-shape-work-wave5/e5_ac_isolator.output.json`:

| unit | combined `ours` before → after | oracle | engine file `ours` |
|---|---|---:|---:|
| `plate_of_the_juggernaut` | 9 → 11 | 11 | 11 |
| `shadow_shirt` | 4 → 6 | 6 | 6 |
| `skinwalker_s_leather` | 3 → 5 | 5 | 5 |
| `leather_of_confined_spaces` | 2 → 5 | 5 | 5 |

Both sides were derived independently and then agreed. Correcting a stale `ours` at the same
time as capturing a real oracle value is the legitimate case — comparing the stale `ours`
against the fresh oracle would have manufactured a false `disagree`.

**Internal consistency audit** (independent of `box_ledger`, which gates on the `verdict`
field alone):

- **`verdict=agree` but `ours != oracle`: 0 of 811 `agree` rows.**
- **Reasonless `unverifiable`: 0 of 7,519 `unverifiable` rows.**
- `verdict=unverifiable` with both sides present and unequal: **61 of 7,519** — all 61 carry
  reason `var_gated_by_unbuilt_class_feature_zero_on_generic_baseline` (60 established by
  `AT-33-E5-shape-var`, plus 1 added this wave), a documented non-comparison where PCGen
  exports 0 on a baseline that never builds the gating class feature. Consistent with the
  deferral posture attempt 6 confirmed closed; re-verified, not re-investigated.

**`disagree` capability re-proven live on the current batch path.** A first probe that changed
only `ours` did NOT trip the gate — `box_ledger`'s `oracle_disagreement` is defined as rows
with `verdict == "disagree"` (`scripts/box_ledger.py:219`), so the correct probe flips the
verdict:

```
$ <copy of the current closure artifact, verdict flipped to "disagree" on one named unit>
uncovered=0 overlap=0 population=49438 oracle_disagreement=1 unverifiable_done=0 stale=False
ORACLE_DISAGREEMENT: core_rulebook:equipment:rod_thunder_and_lightning
EXIT=1
```

Probe removed; the real artifact reports `oracle_disagreement=0`, exit 0. Recorded for the
next scanner: `box_ledger` trusts the `verdict` field and never recomputes it from
`ours`/`oracle`, so the separate consistency audit above is load-bearing, not redundant.

## Check 3 — the method change's re-run happened and covered its set

`method_change_rerun_verified: true`. All three wave-5 corrections carrying the "re-run
everything it already judged" obligation are discharged, and the affected set for each was
**derived by execution**, not assumed:

| Correction | Affected set, with denominator | Rows re-run | Coverage |
|---|---|---:|---|
| AC isolator `a68fbeea3d` | 66 of that script's own 82-item manifest | 66 | **66 of 66** — independently re-verified row-by-row against the combined file, 0 mismatches |
| Campaign-key `9df1c0b514` | 14 of 14 rows carrying `oracle_harness_ultimate_psionics_campaign_load_failure` | 14 | **14 of 14** |
| Identity-resolve `9df1c0b514` | 5 of 209 examined rows intersecting a corpus-wide 436-of-7,807-record affected population | 5 | **5 of 5** |

**What moved:** 20 of 21 rows changed value; 17 of 21 changed verdict (all
`unverifiable` → `agree`); **0 of 21 moved `agree` → `disagree`**.

That zero was treated with suspicion rather than relief, as this criterion requires. It is
corroborated three ways: (a) the whole-artifact attempt-6→HEAD diff above independently finds
**0 of 8,291** `agree` → `disagree` transitions; (b) the 4 stale-`ours` rows the re-run DID
surface are the same failure class, caught and corrected on both sides; (c) the re-run's own
`ours` values reconcile to the committed engine output file. The re-run did not silently cover
a subset.

## Check 5 — the newly-covered shapes are real comparisons

| Shape | Finding |
|---|---|
| `WEAPON\|DAMAGEMULT` fractional | **No truncation.** 3 of 3 Advanced Class Guide units (`duelist_s_comate`, `rapier_of_battlefield_movement`, `swashbuckler_s_rapier`) recorded `unverifiable` / `no_comparable_export_token`, inheriting `AT-33-E5-last67-weapon`'s rule. Stated at the weapon receipt's lines 58, 197, 304. |
| bare `WEAPON\|TOHIT,DAMAGE,ATTACKS` | **Applied consistently.** The TOHIT-is-comparable rule is stated at the weapon receipt's lines 205, 298, 308-314 and applied across the family; the two units carrying a real oracle magnitude with no engine resolver (`crossbow_double` oracle −4, `rod_withering` oracle 1) are recorded `unverifiable` / `no_resolver` with `ours` live-confirmed null by a real `compute_equipment_effects` call — declined, not fabricated. |
| flurry units | **Declined, with the reason stated.** `flurry_of_fists` and `flurry_of_strikes` are `no_resolver`: `BONUS:WEAPON\|ATTACKS\|<VarName>` is a variable-named formula meaningful only in class context. Attack COUNT was NOT claimed as a comparable quantity, so no false `agree` was manufactured. Both were independently rowed by two lanes with identical verdicts and deduped. |
| `EQM*` modifiers | **Genuinely different mechanism.** The eqm lane used EQMOD baked into a homebrew LST item at load time, explicitly distinguished from wave 5's proven-untrustworthy `.pcg`-time `CUSTOMIZATION:` block; a host item is named per modifier and is the same on both sides. Two previously-unwired engine resolvers landed RED→GREEN (`EQMWEAPON\|DAMAGESIZE`, `EQM\|WEIGHTDIV`). |
| psionics | **0 units** called `unverifiable` for "book not in oracle". Every psionics row carries a real shape reason (`no_resolver`, `no_probe_surface`), each live-confirmed via `e5_last39_skill_combat_ours`. The 14 `ultimate_psionics` campaign-load failures became real live captures rather than a book-level excuse. |
| `EQM\|WEIGHTDIV` fractional | **No truncation** — `resolve_eqm_weightdiv_effect` returns `f32`. `material_darkleaf_cloth_clothing` is `ours=4.0` vs `oracle=4`, `agree`. The lane states honestly (`AGENTS.md` Rule 7) that its own `8/2=4` case is an exact integer, so no-truncation is asserted in the doc comment and NOT exercised by a fractional test case. Accepted as declared scope, recorded here as a narrow proof. |

## Re-verified CLOSED (re-verified, not re-investigated)

| Item | Result | Command |
|---|---|---|
| Row 16 | **1,741 of 1,741** rows, 0 disagree | Check 1 command |
| Denominator gate | **0 violations of 53 files checked**, exit 0; scope widened from 47 of 47 files at attempt 6, matcher untouched (`git log 9e90d0694e..HEAD -- scripts/denominator_gate.py` empty) | `scripts/verify.sh --only denominator-gate` |
| Gate detection | **Re-proven live** — probe planted in a scanned `*_cycle_receipt.md`, `files_checked` 53→54 and `violations` 0→1 with the bare hundred-percent token named; probe removed, back to 53 files / 0 violations, `git status` clean of residue | probe + `denominator_gate.py --check` |
| Reasonless `unverifiable` | **0 of 7,519** | consistency audit |
| Duplicate `unit_id` | **0 of 8,330** | Check 1 command |
| work-inventory `unknown` | **0 of 49,438 units** | `jq '[.units[]\|select(.status=="unknown")]\|length' docs/work-inventory.json` |
| Hardcoded exclusion lists | **0 in the closure instruments** — `denominator_gate.py` and `box_ledger.py` carry none; `coverage_ledger.py`'s `EXCLUDED_BOOKS` default is SD-32's already-emptied `frozenset()` | grep of the three instruments |
| Epic 3 artifact | at the SD-33 path (`artifacts/epic-3-engine-coverage/`); SD-32's `gate-2-engines` untouched (`git log f652db7ac7..HEAD -- docs/release/SD-32*/` empty) | `ls` + `git log` |
| Receipts at kanban-stated paths | **0 missing** of every `*_cycle_receipt.md` path the kanban cites | path-existence loop |
| Corpus integrity | **7,808 of 7,808** records unchanged; **137 of 137** changed files are modifies; **0 of 137** lost license/PI metadata; **0 of 137** shrank `raw_tokens` | corpus audit above |
| Deferral posture | forward-scope register rows are C-series forward scope owned by future bundles; **0 defer DoD scope** | `forward-scope-register.md` |

## Criterion / card status at attempt 7

| Row | Criterion | Status | Blocks? |
|---|---|---|---|
| 1-13, 15 | `AT-33-E1-001`..`AT-33-E3-004`, `AT-33-E4-001`, `AT-33-E4-003` | complete | no |
| **14** | **`AT-33-E4-002`** | **complete** | **YES — `complete` over a suite its own commit `00ca087775` turned red (Shortfall 1)** |
| 16 | `AT-33-E5-001` | complete | no — 1,741 of 1,741, 0 disagree |
| 17 | `AT-33-E5-002` | complete | no — 6,589 of 6,589 rowed, 0 unrowed |
| 18 | `AT-33-E5-003` | complete | no — 0 of 8,330 disagree, `## Open blockers` empty |
| 19 | `AT-33-E6-001` | blocked-escalated | this card |
| 20-21 | `AT-33-E6-002`, `AT-33-E6-003` | not-started | Epic 6's own, gated on row 19 |

## Environment finding (not a gate item, but a hazard to record)

The shared checkout at `/home/ubuntu/workspace/repos/codex` was, at the start of this scan,
**8 commits behind `origin/tranche/13`** and carried **154 entries in `git status --porcelain`**
that this agent did not create — including a STAGED revert of the corpus-extraction fix (139
modified corpus files restored to their pre-`fbc945f198` content, `src/bin/enrich_equipment_raw_tokens.rs`
reverted, and 7 wave-6 receipts/retro files staged as deleted). Per `AGENTS.md`'s
"One writer per tree" rule, this agent did not write there. The entire scan ran in a clean
detached worktree at `origin/tranche/13` = `7d439876b7`. **Had the scan run in the shared
checkout it would have measured a tree that does not exist on any branch** — the corpus fix
absent, the wave-6 receipts absent — and produced a confidently wrong FAIL. Recorded because
this is the second consecutive wave in which shared-checkout state, not the work, was the
thing most likely to produce a false reading.

## Disposition

**Gate FAIL. No retrospective, no sweep, no PR.** Kanban row 19 stays `blocked-escalated`.

One item stands between this bundle and closure:

1. **Shortfall 1** — map `('ambiguous', 'unmeasurable')` in
   `scripts/observer/pf1e_dashboard_producer.py::_doneness_verdict_uncapped` (11 of 49,438
   units carry the pair, all from row 14's own commit), and reconcile
   `src/rules_core/equipment_resolver.rs:863`'s hardcoded `8100` against the live catalog's
   `8119`. Then `cargo test --locked --lib` green, and row 14 is honestly `complete`.

Everything else this criterion checks is closed and was verified by execution this cycle.

- **Status:** blocked-escalated
- **Movement, four buckets:** closure 0, reclassification 0, reachability 0,
  instrument-correction 0 (see "Four buckets" above).
