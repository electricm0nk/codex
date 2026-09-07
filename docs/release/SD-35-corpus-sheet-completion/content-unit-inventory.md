---
canonical: true
owner: god-emporer
bundle_id: SD-35
date: 2026-09-07
---

# SD-35 Content-Unit Inventory

The measured baseline and **the command that re-derives every figure in this package**.

## 0. Provenance, and the re-measure owed at the cut

Measured 2026-09-07 against `tranche/14` HEAD `5f6b18f4e3`, from `docs/work-inventory.json`
(`generated_at: 2026-09-07T13:06:14Z`, `generated_by: cargo run --bin v06_work_inventory`).
Bucket letters come from `scripts/completion_atlas.py::_bucket_of()`; the atlas's own
`completion-atlas.json` at the same HEAD reproduces every count below bit-for-bit
(`unclassified=0 overlap=0 done_evidence_violations=0`).

**SD-34 is still running.** Every figure here is re-run at the `tranche/15` cut and the cut's
numbers replace these throughout the package (`decisions.md §7`). Setup at the cut:

```bash
git show <tranche-15-cut-sha>:docs/work-inventory.json > /tmp/wi.json
python3 scripts/completion_atlas.py --check      # the authoritative bucket totals
```

Every command below reads `docs/work-inventory.json` at HEAD unless it says `/tmp/wi.json`.

## 1. Population and the remainder

```bash
python3 -c "
import json, collections
d = json.load(open('docs/work-inventory.json')); wi = d['units']
DONE = {'grounded','text-complete','oracle-agree','oracle-unverifiable'}
print('units:', len(wi), 'books:', len(collections.Counter(u['book'] for u in wi)))
print('DONE:', sum(1 for u in wi if u['status'] in DONE), 'remaining:', sum(1 for u in wi if u['status'] not in DONE))"
```

| | Units |
|---|---:|
| Corpus | 49,438 across 37 books |
| DONE | 26,123 of 49,438 |
| Remaining | 23,315 of 49,438 |

**The DONE set gains `sheet-complete` at AT-35-E2-003.** Until then this command's set is the
atlas's.

## 2. Remainder by bucket

```bash
python3 scripts/completion_atlas.py --check
```

| Bucket | Meaning (`completion_atlas.py`'s own) | Units of 23,315 |
|---|---|---:|
| B | table exists, record not placed in it | 11,589 |
| M | ingested-magnitude: engine holds real numbers, no consumer delta observed | 4,334 |
| C | held and computed, never surfaced (explanation/display wiring) | 4,180 |
| D | other engine gap, per-unit sub-cause | 1,982 |
| A | engine has no table for this kind | 449 |
| V | literal-verified / fixture-verified, never oracle-checked | 392 |
| U | unmeasurable, named reason per unit | 202 |
| X | deferred-with-reason | 168 |
| Z | not-started (no compiled rule set) | 19 |

Sums to 23,315. Bucket A is exactly `missing-engine-tables.json`'s population: `power` 421
(all `ultimate_psionics`), `companion` 28 (all `bestiary`). Bucket Z is all `beginner_box`.

## 3. Remainder by kind, and kind × bucket

```bash
python3 -c "
import json, collections, sys
sys.path.insert(0, 'scripts'); from completion_atlas import _bucket_of
wi = json.load(open('docs/work-inventory.json'))['units']
rem = [u for u in wi if _bucket_of(u) != 'DONE']
by_kind = collections.Counter(u['kind'] for u in rem)
for k, v in by_kind.most_common(): print(f'{k:20} {v:6}')
xt = collections.Counter((u['kind'], _bucket_of(u)) for u in rem)
for (k, b), v in sorted(xt.items()): print(f'{k:20} {b} {v:6}')"
```

| Kind | Remaining of 23,315 | A | B | C | D | M | V | U | X | Z |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| class_feature | 12,856 | 0 | 7,866 | 4,180 | 471 | 0 | 185 | 0 | 154 | 0 |
| ability | 2,066 | 0 | 475 | 0 | 108 | 1,483 | 0 | 0 | 0 | 0 |
| template | 1,992 | 0 | 1,092 | 0 | 595 | 305 | 0 | 0 | 0 | 0 |
| race_trait | 1,351 | 0 | 319 | 0 | 183 | 697 | 152 | 0 | 0 | 0 |
| feat | 1,073 | 0 | 490 | 0 | 0 | 518 | 1 | 62 | 2 | 0 |
| spell | 952 | 0 | 391 | 0 | 0 | 558 | 3 | 0 | 0 | 0 |
| companion | 674 | 28 | 634 | 0 | 0 | 0 | 0 | 0 | 12 | 0 |
| equipment_modifier | 473 | 0 | 0 | 0 | 0 | 443 | 9 | 21 | 0 | 0 |
| power | 421 | 421 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| deity | 417 | 0 | 0 | 0 | 408 | 9 | 0 | 0 | 0 | 0 |
| equipment | 353 | 0 | 74 | 0 | 0 | 99 | 42 | 119 | 0 | 19 |
| class | 147 | 0 | 118 | 0 | 29 | 0 | 0 | 0 | 0 | 0 |
| domain | 147 | 0 | 0 | 0 | 80 | 67 | 0 | 0 | 0 | 0 |
| trait | 129 | 0 | 0 | 0 | 6 | 123 | 0 | 0 | 0 | 0 |
| language | 114 | 0 | 33 | 0 | 81 | 0 | 0 | 0 | 0 | 0 |
| race | 59 | 0 | 56 | 0 | 0 | 3 | 0 | 0 | 0 | 0 |
| skill | 51 | 0 | 1 | 0 | 21 | 29 | 0 | 0 | 0 | 0 |
| monster | 27 | 0 | 27 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| monster_ability | 13 | 0 | 13 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

`class_feature` bucket B (7,866) is the single largest homogeneous swath — a third of the
whole remainder — and is AT-35-E3-001's population.

## 4. Remainder by book (top 15 of 37)

```bash
python3 -c "
import json, collections, sys
sys.path.insert(0, 'scripts'); from completion_atlas import _bucket_of
wi = json.load(open('docs/work-inventory.json'))['units']
c = collections.Counter(u['book'] for u in wi if _bucket_of(u) != 'DONE')
for k, v in c.most_common(15): print(f'{k:28} {v:6}')"
```

| Book | Remaining of 23,315 |
|---|---:|
| advanced_class_guide | 2,366 |
| advanced_players_guide | 2,207 |
| ultimate_psionics | 2,165 |
| ultimate_magic | 1,924 |
| core_rulebook | 1,529 |
| bestiary | 1,322 |
| ultimate_wilderness | 1,134 |
| ultimate_combat | 1,133 |
| advanced_race_guide | 1,010 |
| occult_adventures | 1,006 |
| adventurers_guide | 895 |
| inner_sea_gods | 811 |
| horror_adventures | 801 |
| ultimate_intrigue | 757 |
| mythic_adventures | 664 |

The other 22 books sum to 5,191 (smallest `bonus_bestiary` at 3). **Books are reported, not
targeted** (`scope-draft.md §7`); cycles run across all 37.

## 5. The token vocabulary — why this is small

The inventory carries no per-unit token-type list, only `magnitude_token_count`. Two
measurements exist:

**(a) Fable review TOKEN-MODEL** (`../SD-34-book-completion/artifacts/fable-review/TOKEN-MODEL.md`,
measured at `920cb53307`, denominator 22,369 non-DONE units in the 35 non-vehicle books):
189 distinct top-level token types over 181,274 instances; 28 types cover 90% of 181,274 instances;
only ~139 types are compute-bearing at all, 41.7% of 181,274 instances; `BONUS:VAR`/`DEFINE` shapes the
existing interpreter already handles are present on 6,708 of the 22,369 units. Genuinely
choice-bearing units: 1,669 of 22,366 joinable.

**(b) This package's authoring join** (2026-09-07, declaration line only, not closure-aware —
a lower bound): of 21,671 remaining units whose `source_file`/`source_line` resolved against
the pinned corpus, 13,002 carry only tokens from {`BONUS:`, `PRE*`, `DEFINE:`, `COST:`, `SAB:`};
5,956 show no magnitude or `PRE*` token on their declaration line at all.

```bash
python3 -c "
import json
wi = json.load(open('docs/work-inventory.json'))['units']
DONE = {'grounded','text-complete','oracle-agree','oracle-unverifiable'}
rem = [u for u in wi if u['status'] not in DONE]
print('magnitude_token_count>0:', sum(1 for u in rem if (u.get('magnitude_token_count') or 0) > 0), 'of', len(rem))
print('magnitude_token_count==0:', sum(1 for u in rem if (u.get('magnitude_token_count') or 0) == 0), 'of', len(rem))"
```

Output at authoring: **13,017 of 23,315** carry at least one magnitude token; **10,298 of
23,315** carry none. Under `decisions.md §1` the second group needs only its prose rendered;
the first group needs the converter's mapping rows, added by token type.

**`scripts/token_coverage.py` (AT-35-E2-004) replaces both measurements** with a closure-aware,
per-token, per-unit ledger re-derived every cycle. Until it exists, (a) is the figure to quote
and (b) is a sanity check.

## 6. The instruments, and their state at authoring

```bash
python3 scripts/completion_atlas.py --check            # population=49438 buckets=10 unclassified=0 overlap=0 citation_failures=0
python3 scripts/shape_engine_boundary.py --check       # citation_ok=True  (was stale before wave 51 — decisions.md §9 L4)
python3 scripts/missing_engine_tables.py --check       # population=449 citation_failures=0
python3 scripts/box_ledger.py --check                  # uncovered=0 overlap=0 population=49438
grep -c '' scripts/verify.sh                           # ALL_STAGES at :110 — 40 stages at authoring
```

`shape_engine_boundary.py`'s own figures at authoring: **26,396** units carry a magnitude token
(of 49,438); **8,784 of 26,396** are still `engine-does-not-hold`. Under the sheet rule that
second figure is a conversion question, not a placement question.

**The live-side PCGen residue — Epic 6's population.** A coarse grep at authoring
(`decisions.md §11`; exact baseline recorded by AT-35-E1-005's first run):

```bash
grep -rlE 'PcgenFormulaEvaluator|render_pcgen_desc|raw_tokens|bonus_stack_reader|pre_tokens' \
  src/rules_core src/saved_character src/campaign src/homebrew_authoring apps/desktop/src-tauri/src | wc -l
for pat in PcgenFormulaEvaluator render_pcgen_desc raw_tokens bonus_stack_reader pre_tokens; do
  echo "$pat live=$(grep -rl "$pat" src/rules_core apps/desktop/src-tauri/src | wc -l) tools=$(grep -rl "$pat" src/bin scripts tests | wc -l)"; done
```

| Surface | Live-side files (rules_core + desktop crate) | Tool-side files (bins, scripts, tests) |
|---|---:|---:|
| any of the five | 78 | — |
| `raw_tokens` | 60 | 74 |
| `render_pcgen_desc` | 17 | 7 |
| `PcgenFormulaEvaluator` | 14 | 11 |
| `bonus_stack_reader` | 7 | 5 |
| `pre_tokens` | 6 | 6 |

Of the 78, the `src/rules_core/cache_gen/**` generators are converter code on the wrong side
(relocate, AT-35-E6-002); the desktop `*_catalog.rs` / picker / bridge files and the
`rules_core` evaluator callers are real live readers (replace, AT-35-E6-001, E6-003). The gate
splits them exactly.

## 7. Figures deliberately NOT stated here

- **Hours.** No rate under the sheet rule has been measured. AT-35-E2-005 measures the first
  corpus-wide pass; AT-35-E3-004 and AT-35-E4-003 record rates per mechanism. The fable review's
  brackets (interpreter route ~85–350h to 66.2% of 22,369; hand-modeling 3,900–31,000h) are
  quoted in `scope-draft.md §3` as that review's **estimates**, with its denominator, and are not
  load-bearing here.
- **Units per cycle beyond the floor.** 500 is a floor. The distribution is a closure
  deliverable (AT-35-E6-002).
- **How many books close, or when.** Books fall out of S1.

## 8. Hazards this document's own authoring hit

- **Three denominators are live** — 49,438 / 23,315 / 22,369 (`decisions.md §8`). Every figure
  above names its own.
- **The inventory agent's token join was declaration-line-only.** It is labeled a lower bound
  everywhere it is quoted; `token_coverage.py` is the real instrument.
- **`find -newermt` lies on this box** — freshness was checked with a Python mtime comparison.
- **The residue grep is coarse.** It counts files naming any of five identifiers; a comment or a
  doc string counts. AT-35-E1-005's gate counts hits by pattern and records the exact baseline;
  quote that, not 78, once it exists.
