#!/usr/bin/env python3
"""Assemble AT-33-E5-002's full per-unit (ours, oracle, verdict) result set
for the 6,589-unit literal-verified population (SD-33 remediation cycle).

Five real, mutually-exclusive, execution-derived groups (sum == 6,589):

  1. equipment stat-shape (41)         -- live oracle round-trip this cycle,
                                           agree=41 disagree=0 (stat41.oracle-results.json)
  2. equipment no-bonus-chain (4,681)  -- unverifiable, reason=no_bonus_chain
  3. monster/monster_ability/companion
     (1,090)                          -- unverifiable, reason=no_probe_surface
                                           (AT-33-E1-003, re-confirmed by count this cycle)
  4. equipment other-bonus-shape (448) -- UNEXAMINED this cycle (real probe,
                                           different shape, not fed to the
                                           harness) -- excluded from this
                                           output; reported separately as
                                           remaining scope.
  5. spell/equipment_modifier/race/
     class_feature/race_trait (329)   -- UNEXAMINED this cycle, same reason
                                           as group 4 -- excluded here.

Only groups 1-3 produce a per-unit verdict record (5,812 total). Groups 4-5
(777 units) are NOT written as records here -- an unexamined unit is neither
`agree` nor `unverifiable`; recording it under any verdict would be exactly
the "we did not look" bucket AT-33-E4-003/this cycle's brief forbids.
"""
import json, sys

stat_results = json.load(open(sys.argv[1]))['results']  # stat41.oracle-results.json
partition = json.load(open(sys.argv[2]))  # equipment_partition_v2.json
out_path = sys.argv[3]

records = list(stat_results)
assert len(records) == 41, f"expected 41 stat-shape records, got {len(records)}"
assert all(r['verdict'] == 'agree' for r in records)

for unit_id in partition['no_bonus_chain']:
    records.append({
        'unit_id': unit_id,
        'ours': None,
        'oracle': None,
        'verdict': 'unverifiable',
        'reason': 'no_bonus_chain: raw_bonus_chains is empty for this corpus record -- '
                  'compute_equipment_effects resolves no bonus to compare (mundane gear whose '
                  'only literal-verified magnitude is cost/weight, not a rules effect)',
    })

import json as _json
inv = _json.load(open('docs/work-inventory.json'))
for u in inv['units']:
    if u.get('status') == 'literal-verified' and u.get('kind') in ('monster', 'monster_ability', 'companion'):
        records.append({
            'unit_id': u['id'],
            'ours': None,
            'oracle': None,
            'verdict': 'unverifiable',
            'reason': 'no_probe_surface: AT-33-E1-003 probe-surface census finds probe_exists=false '
                      f"for kind={u['kind']} (category: presence_only) -- our engine holds no "
                      'magnitude-producing computation for this kind to compare against an oracle value',
        })

with open(out_path, 'w') as f:
    json.dump({'results': records}, f, indent=2)

from collections import Counter
c = Counter(r['verdict'] for r in records)
print(f"total records: {len(records)} -- {dict(c)}")
print('wrote', out_path)
