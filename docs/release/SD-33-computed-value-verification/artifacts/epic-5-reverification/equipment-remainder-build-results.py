#!/usr/bin/env python3
"""Assembles `equipment-remainder.oracle-results.json` -- the per-unit
`(ours, oracle, verdict)` rows for the equipment-remainder lane's 494-unit
population (SD-33 remediation wave 2: 448 `other_bonus_shape` equipment +
46 `equipment_modifier`, `AT-33-E5-001`/`002`'s named remainder).

Three real sources, each produced by real execution this cycle (no
fabricated rows):

1. `equipmod-no-chain.json` -- the `equipment_modifier` units whose real
   corpus record (read whole, not filtered to BONUS tokens) carries an
   EMPTY `raw_bonus_chains` -- genuinely no computable magnitude. Verdict
   `unverifiable`, reason `no_bonus_chain` (`AT-33-E5-002`'s own
   established vocabulary for the identical equipment finding).

2. `skill-oracle-results.json` -- `scripts/oracle_harness/run.py`'s own
   output comparing the equipment `other_bonus_shape` single-skill
   sub-population's real, live PCGen `SKILL.<name>.MISC` export against
   `compute_general_effect`'s real, live engine value. Rows carried
   through unchanged (already `{"unit_id","ours","oracle","verdict"}`).

Every OTHER unit in the 494-unit population (the 14 equipment_modifier
units with a real but unhandled bonus-chain shape, the 27 multi-skill/ALL
equipment SKILL-shape units, and the 331 other_bonus_shape units in a
shape this cycle did not reach) carries NO row here -- deliberately, per
`AT-33-E5-001`/`002`'s own precedent: an unexamined unit is neither
`agree` nor `unverifiable`.
"""
import json, sys

no_chain = json.load(open(sys.argv[1]))          # equipmod-no-chain.json: list of unit_ids
skill_results = json.load(open(sys.argv[2]))      # skill-oracle-results.json: {"results": [...]}
output_path = sys.argv[3]

results = []
for unit_id in no_chain:
    results.append({
        "unit_id": unit_id,
        "ours": None,
        "oracle": None,
        "verdict": "unverifiable",
        "reason": "no_bonus_chain",
    })

for r in skill_results["results"]:
    row = dict(r)
    if row["verdict"] == "unverifiable":
        # Real, corpus-confirmed reason (AT-33-E5-remainder-equipment's own
        # cycle investigation): every SKILL-shape `unverifiable` row this
        # cycle is a `BONUS:SKILL|TYPE.<x>|...` / `TYPE=<x>` qualifier (a
        # skill-TYPE selector applying to a whole subskill family -- e.g.
        # `TYPE.Perform`, `TYPE=Knowledge` -- not a single named skill).
        # `compute_general_effect` stores this literal qualifier string as
        # `skill_bonus.skill` (same un-split-verbatim pattern already named
        # for the multi-skill/`ALL` and multi-ability-STAT shapes), and
        # PCGen's own `SKILL.<name>.MISC` token has no matching single
        # skill by that name to query -- a real, examined absence, not an
        # unattempted unit.
        row["reason"] = "skill_type_qualifier_no_literal_skill_name"
    results.append(row)

with open(output_path, "w") as f:
    json.dump({"results": results}, f, indent=2)
    f.write("\n")

from collections import Counter
c = Counter(r["verdict"] for r in results)
print(f"equipment-remainder: {len(results)} rows -> {output_path}")
print(dict(c))
