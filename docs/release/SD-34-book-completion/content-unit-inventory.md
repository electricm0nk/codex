---
canonical: true
owner: god-emporer
bundle_id: SD-34
date: 2026-08-26
---

# SD-34 Content-Unit Inventory

The measured baseline and **the command that re-derives every figure in this package**.

**Provenance:** measured 2026-08-27 against `origin/develop` at `ea2b3396f2` — the merge commit of
SD-33's closure PR #377. The bare SHA is the stable reference: `origin/tranche/13` was deleted
after the merge. Reads `docs/work-inventory.json`. Setup:

```bash
git show ea2b3396f2:docs/work-inventory.json > /tmp/wi.json
```

Unlike this package's first draft, these figures are **not** provisional. They were derived
by execution and the key one — that ingestion is complete — was proved directly (§2).

**Re-measured once already.** The first measurement ran at the parent commit of SD-33's final
fold; the fold regenerated the inventory and moved four figures (DONE +87, B −65, M −42, D +20,
`not-ingested` −45, all of 49,438). Every figure below was re-run at the merged tip. The same
re-run is owed at the `tranche/14` cut if that SHA differs from `ea2b3396f2`.

## 1. Population

**49,438** inventory units across **37** books. **51,505** JSON files under `data/corpus/` at
`ea2b3396f2`, of which **48,881** sit outside the two non-inventory directories `beastiary/` and
`core_essentials/`. The **39** on-disk book directories collapse to **37** inventory books because
`core_essentials` units are re-attributed to their true source book
(`resolve_true_book_for_core_essentials`, `src/bin/v06_work_inventory.rs:1091`).

```bash
python3 -c "
import json, collections
wi = json.load(open('/tmp/wi.json'))['units']
print('units:', len(wi), 'books:', len(collections.Counter(u['book'] for u in wi)))"

git ls-tree -r --name-only ea2b3396f2 -- data/corpus | grep -c '\.json$'            # 51505
git ls-tree -r --name-only ea2b3396f2 -- data/corpus | grep '\.json$' \
  | grep -vE '^data/corpus/(beastiary|core_essentials)/' | wc -l                     # 48881
git ls-tree -d --name-only ea2b3396f2 -- data/corpus/ | wc -l                        # 39
```

## 2. Ingestion is COMPLETE — the proof

Every unit was read from a real source line. This is the check whose absence caused this
package's first draft to report "52.7% not ingested" (26,047 of 49,438 at the time) in error.

```bash
python3 -c "
import json
wi = json.load(open('/tmp/wi.json'))['units']
ni = [u for u in wi if u['status'] == 'not-ingested']
ok = [u for u in ni if u.get('source_file') and u.get('source_line')]
print(f'status not-ingested: {len(ni)}')
print(f'  ...carrying a real source_file + source_line: {len(ok)} ({100*len(ok)/len(ni):.1f}%)')"
```

Output: **26,002**, of which **26,002 of 26,002 (100.0%)** carry a real `source_file` and `source_line`.

Sample:
```
advanced_class_guide:class_feature:aberrant_bloodline   acg_abilities_class.lst:156
```

**`not-ingested` is a misnomer for "the engine does not hold this record".** Its evidence
strings are all engine-side:

```bash
python3 -c "
import json, collections
wi = json.load(open('/tmp/wi.json'))['units']
ni = [u for u in wi if u['status'] == 'not-ingested']
for k, v in collections.Counter(u['evidence'] for u in ni).most_common(6):
    print(f'  {v:6} | {k}')"
```

| Evidence | Units |
|---|---:|
| `no_explanation_id_and_no_diagnostic_names_this_feature` | 4,388 |
| `ability_content_has_no_engine_table` | 4,337 |
| `class_feature_owner_matched_by_name_but_record_not_held_by_engine` | 3,574 |
| `class_feature_option_pool_record_with_magnitude_not_held_by_engine` | 3,052 |
| `template_content_has_no_engine_table` | 2,248 |
| `class_feature_option_pool_record_not_held_by_engine` | 1,733 |

`race_trait_record_loaded_but_never_applies` states it outright: **loaded**, then not applied.
Renaming the field is AT-34-E1-005.

## 3. The Completion Atlas — the bundle's central figure

```bash
python3 -c "
import json, collections
wi = json.load(open('/tmp/wi.json'))['units']
def bucket(u):
    s, e = u['status'], (u.get('evidence') or '')
    if s in ('grounded','text-complete'): return 'DONE'
    if s in ('literal-verified','fixture-verified'): return 'V'
    if s == 'ingested-magnitude': return 'M'
    if s == 'not-ingested':
        if 'has_no_engine_table' in e: return 'A'
        if 'not_held_by_engine' in e or 'absent_from' in e or 'not_modelled' in e: return 'B'
        if 'explanation_id' in e or 'diagnostic' in e: return 'C'
        return 'D'
    return {'unmeasurable':'U','deferred-with-reason':'X','not-started':'Z'}.get(s,'UNCLASSIFIED')
c = collections.Counter(bucket(u) for u in wi)
for k, v in sorted(c.items(), key=lambda x: -x[1]):
    print(f'  {k:14} {v:6}  {100*v/len(wi):5.1f}%')
print('  sum:', sum(c.values()), '== 49438:', sum(c.values()) == len(wi))"
```

| Bucket | What remains | Units of 49,438 |
|---|---|---:|
| DONE | nothing | 12,265 of 49,438 (24.8%) |
| B | table exists, record not in it | 11,921 of 49,438 (24.1%) |
| A | engine has no table for this kind | 8,463 of 49,438 (17.1%) |
| V | verified by proxy, never by the oracle | 8,330 of 49,438 (16.8%) |
| C | held and computed, never surfaced | 4,388 of 49,438 (8.9%) |
| M | magnitude ingested, never computed/applied | 2,455 of 49,438 (5.0%) |
| D | other engine gap | 1,230 of 49,438 (2.5%) |
| U | instrument cannot express a verdict | 321 of 49,438 (0.6%) |
| X | deferred with reason | 46 of 49,438 (0.1%) |
| Z | not started | 19 of 49,438 (0.0%) |
| **unclassified** | — | **0** |

Sums to 49,438 exactly. `scripts/completion_atlas.py` (AT-34-E1-001) formalises this and makes
`unclassified != 0` a hard error.

## 4. Bucket A — the nine engine tables that do not exist

```bash
python3 -c "
import json, collections
wi = json.load(open('/tmp/wi.json'))['units']
A = [u for u in wi if u['status']=='not-ingested' and 'has_no_engine_table' in (u.get('evidence') or '')]
for k, v in collections.Counter(u['kind'] for u in A).most_common():
    cr = sum(1 for u in A if u['kind']==k and u['book']=='core_rulebook')
    print(f'  {k:12} {v:5} total  {cr:4} core_rulebook')
print('  TOTAL', len(A), 'across', len(set(u['kind'] for u in A)), 'kinds')"
```

| Kind | Units | Core Rulebook |
|---|---:|---:|
| ability | 4,337 | 471 |
| template | 2,248 | 262 |
| trait | 487 | 0 |
| deity | 459 | 21 |
| power | 421 | 0 |
| domain | 183 | 34 |
| skill | 149 | 110 |
| language | 136 | 22 |
| companion | 43 | 14 |
| **Total** | **8,463** | **934** |

**Seven of nine are exercised by the Core Rulebook** — only `trait` and `power` sit outside it.
An earlier draft of this document said six; it missed the 21 `deity` units. Re-deriving the
per-kind, per-book coverage is what found the error *and* the second vehicle book
(`decisions.md §7`).

`trait` is supplied by Ultimate Campaign (§6a below), leaving **`power` as the only table
costed rather than built** — all 421 of its units are inside `ultimate_psionics`.

## 5. The shape-engine boundary

```bash
python3 -c "
import json
wi = json.load(open('/tmp/wi.json'))['units']
fb = [u for u in wi if u.get('magnitude_token_count',0) > 0]
ni = [u for u in fb if u['status'] == 'not-ingested']
print(f'magnitude-bearing units (shape-engine input): {len(fb)} of {len(wi)}')
print(f'  ...engine still does not hold them: {len(ni)} of {len(fb)}')"
```

**26,396** units carry magnitude tokens. **13,119 of those 26,396** are still not held by the
engine — half the shape engines' own feedstock is stuck downstream of them.

The shape engine's own coverage, from SD-33's committed artifact:

```bash
git show ea2b3396f2:docs/release/SD-33-computed-value-verification/artifacts/epic-3-engine-coverage/formula_interpreter.corpus-wide.json \
  | python3 -c "
import json, sys
d = json.load(sys.stdin)
print(d['scope'], d['total_population'], 'recognised', d['total_recognised_units'],
      'refused', d['total_refused_units'], 'unjoined', d['total_unjoined_units'])"
```

F1..F9, **11,652** population, **10,626** recognised, **240** refused, **786** unjoined.

## 6. Core Rulebook — the vehicle

**6,701** units, the most-blocked book in the corpus.

| Bucket | Units | % of 6,701 |
|---|---:|---:|
| V | 2,582 of 6,701 | 38.5% |
| DONE | 1,150 of 6,701 | 17.2% |
| B | 970 of 6,701 | 14.5% |
| A | 934 of 6,701 | 13.9% |
| M | 512 of 6,701 | 7.6% |
| C | 370 of 6,701 | 5.5% |
| D | 119 of 6,701 | 1.8% |
| U | 58 of 6,701 | 0.9% |
| X | 6 of 6,701 | 0.1% |

**Every bucket except Z is present**, which is why completing this one book measures the real
cost of every step type in the atlas.

## 6a. Ultimate Campaign — the second vehicle

```bash
python3 -c "
import json, collections
wi = json.load(open('/tmp/wi.json'))['units']
uc = [u for u in wi if u['book'] == 'ultimate_campaign']
print('units:', len(uc))
for (k, s), v in collections.Counter((u['kind'], u['status']) for u in uc).most_common():
    print(f'  {k:12} {s:22} {v:4}')"
```

| Kind | Status | Units |
|---|---|---:|
| trait | not-ingested (bucket A) | 154 |
| ability | not-ingested (bucket A) | 88 |
| feat | unmeasurable (bucket U) | 21 |
| feat | deferred-with-reason (bucket X) | 2 |

**265 units, four rows, no B/C/D/M/V at all.** 242 of 265 clear on two tables, one of which
(`ability`) the Core Rulebook already needs. The cleanest book in the corpus and the best
table-to-book ratio available anywhere.

Which books carry the two non-core kinds:

```bash
python3 -c "
import json, collections
wi = json.load(open('/tmp/wi.json'))['units']
for k in ('trait', 'power'):
    per = collections.Counter(u['book'] for u in wi if u['kind'] == k)
    print(k, sum(per.values()), dict(per.most_common(6)))"
```

`trait` — 487 units: `ultimate_campaign` 154, `inner_sea_gods` 115, `inner_sea_races` 96,
`advanced_players_guide` 90, `ultimate_psionics` 32.
`power` — **421 units, every one in `ultimate_psionics`.** No other book has one.

## 7. The 35 remaining books

```bash
python3 -c "
import json, collections
wi = json.load(open('/tmp/wi.json'))['units']
per = collections.Counter(u['book'] for u in wi if u['status'] not in ('grounded','text-complete'))
for k, v in per.most_common(10): print(f'  {k:26} {v:6}')
print('  TOTAL non-DONE:', sum(per.values()), 'of', len(wi))"
```

**37,173 of 49,438** units are non-DONE corpus-wide. Top books:

| Book | Non-DONE units |
|---|---:|
| core_rulebook | 5,551 |
| advanced_players_guide | 3,004 |
| advanced_class_guide | 2,792 |
| ultimate_psionics | 2,781 |
| ultimate_magic | 2,628 |
| bestiary | 1,619 |
| ultimate_wilderness | 1,483 |
| ultimate_equipment | 1,475 |
| ultimate_combat | 1,468 |
| advanced_race_guide | 1,420 |

Pricing these is Epic 5, using rates **measured** in Epics 2, 3 and 4.

## 8. Figures deliberately NOT stated here

- **How long any of this takes.** No per-unit cost has been measured for any SD-34 mechanism.
  Producing those rates is AT-34-E3-004's entire job; quoting one now would make an estimate
  load-bearing before the wave that establishes it.
- **How many books SD-34 will complete.** Two are vehicles; Epic 5 prices the rest and the
  count falls out.

## 9. Hazards this document's own authoring hit

- **A shallow glob lies here.** `data/corpus/<book>/equipment/*.json` returns **zero** — the
  files sit one level deeper under `.../equipment/<subdir>/`. Use recursive search and state
  the search used.
- **A status field's name is not its meaning.** `not-ingested` cost this package a wrong
  headline reported to the operator. Read the code that writes a field before quoting it.
