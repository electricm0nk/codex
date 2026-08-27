# AT-34-E1-002 — the atlas fails closed on six conditions

Six RED→GREEN mutation proofs, one per condition, run live against
`scripts/completion_atlas.py` and (where the condition is population-scoped)
the real committed `docs/work-inventory.json` / `completion-atlas.json`.
Every mutation was reverted in the same cycle; `git status --porcelain` is
clean of any residual mutation. Re-derive: reapply the described one-line
change, run the command shown, observe the RED line, revert, observe GREEN.

## Condition 1 — `unclassified != 0`

**Mutation:** in `_bucket_of`, changed `if status == "not-started":` to
`if status == "not-started-MUTATED":` so all 19 real `Z`-bucket units fall
through to `None`.

```
$ python3 scripts/completion_atlas.py --check
population=49438 buckets=10 unclassified=19 overlap=0
...
exit=1                                              # RED, for the intended reason
```

Reverted; re-run:

```
$ python3 scripts/completion_atlas.py --check
population=49438 buckets=10 unclassified=0 overlap=0
...
exit=0                                              # GREEN
```

## Condition 2 — `overlap != 0`

**Mutation:** in `partition`, changed `if uid in seen: overlap_ids.append(uid)`
to a dead `if False:` branch, then partitioned a synthetic two-unit list
sharing one `id`.

```python
units = [{"id": "dup", "status": "grounded", "evidence": "x"},
         {"id": "dup", "status": "grounded", "evidence": "x"}]
CA.partition(units)["overlap_ids"]
```

- Mutated: `[]` (RED — the mechanism silently swallowed a real duplicate id)
- Reverted: `['dup']` (GREEN)

## Condition 3 — a unit in `DONE` whose evidence does not support it

**Mutation:** set `_DONE_VIOLATION_MARKERS = ()`, then ran
`_done_evidence_violations` on a synthetic `DONE` unit whose evidence is the
literal bucket-A marker `has_no_engine_table`.

```python
units = [{"id": "g1", "status": "grounded", "evidence": "has_no_engine_table"}]
CA._done_evidence_violations(units)
```

- Mutated: `[]` (RED — a DONE unit carrying an unfinished-bucket marker went undetected)
- Reverted: `['g1']` (GREEN)

**Real-corpus false-positive check (not a mutation):** `explanation_id` is
deliberately excluded from `_DONE_VIOLATION_MARKERS` — 245 real `DONE` units
legitimately carry it
(`explanation_id_observed_and_corpus_record_carries_real_description`, etc.),
confirmed by:

```
$ python3 -c "
import json,collections
d=json.load(open('docs/work-inventory.json'))
hits=sum(1 for u in d['units'] if u.get('status') in ('grounded','text-complete') and 'explanation_id' in (u.get('evidence') or ''))
print(hits)"
245
```

Live acceptance evidence: `python3 scripts/completion_atlas.py --check` on
the real, unmutated inventory reports `done_evidence_violations=0`.

## Condition 4 — a bucket with no named clearing mechanism

**Mutation:** set `BUCKET_DEFINITIONS["Z"]["clears"] = ""`.

```
$ python3 scripts/completion_atlas.py --check
...
missing_clearing_mechanisms=1
  missing_clearing_mechanism: Z
exit=1                                              # RED
```

Reverted; re-run:

```
$ python3 scripts/completion_atlas.py --check
...
missing_clearing_mechanisms=0
exit=0                                              # GREEN
```

## Condition 5 — a `derived_at` SHA that is not an ancestor of `HEAD` (staleness gate)

**Mutation:** hand-edited the committed
`artifacts/epic-1-atlas/completion-atlas.json`'s `derived_at` field to
`0000000000000000000000000000000000dead` (a SHA that does not resolve in
this repo's history), then ran `--check`, which reads the file as it stood
**before** overwriting it.

```
$ python3 scripts/completion_atlas.py --check
...
stale_derived_at=True
  staleness: derived_at '0000000000000000000000000000000000dead' is not an ancestor of HEAD
exit=1                                              # RED
```

Restored the real committed artifact; re-run:

```
$ python3 scripts/completion_atlas.py --check
...
stale_derived_at=False
exit=0                                              # GREEN
```

## Condition 6 — a bucket's `file:line` citation does not resolve, or its content no longer matches

**Mutation:** changed bucket `A`'s citation `must_contain` from
`"has_no_engine_table"` to `"this_marker_is_not_on_that_line"` — the line
number (`src/bin/v06_work_inventory.rs:9658`) still resolves, proving this
asserts on cited **content**, not just path/line
(`risks-and-open-questions.md §10`).

```
$ python3 scripts/completion_atlas.py --check
...
citation_failures=1
  citation_failure: A: src/bin/v06_work_inventory.rs:9658 no longer contains 'this_marker_is_not_on_that_line'
exit=1                                              # RED
```

Reverted; re-run:

```
$ python3 scripts/completion_atlas.py --check
...
citation_failures=0
exit=0                                              # GREEN
```

Additional coverage in `scripts/tests/test_completion_atlas.py` (permanent,
committed): missing citation, out-of-range line number, nonexistent file —
each independently proven to raise a `citation_failures` entry with the
matching reason string.

## Live, unmutated acceptance run

```
$ python3 scripts/completion_atlas.py --check
population=49438 buckets=10 unclassified=0 overlap=0
  DONE: 12265
  A: 8463
  B: 11921
  C: 4388
  D: 1230
  M: 2455
  V: 8330
  U: 321
  X: 46
  Z: 19
done_evidence_violations=0
missing_clearing_mechanisms=0
stale_derived_at=False
citation_failures=0
$ echo $?
0
```

All six conditions verified fail-closed and the live corpus check passes
clean at HEAD.
