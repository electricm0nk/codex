#!/usr/bin/env bash
# E0.3 acceptance self-test. Run from the repo root.
#  1. the check passes on the real inventory (exit 0)
#  2. a planted off-by-one (drop one unit; bump one total; swap one kind) each make it exit non-zero
set -u
A=docs/release/SD-37-starfinder-1e/artifacts/epic_0
INV=docs/work-inventory.starfinder-1e.json
CHECK="python3 $A/sf_work_inventory_check.py"
fail=0
$CHECK --inventory "$INV" >/dev/null 2>&1 && echo "PASS real inventory exits 0" || { echo "FAIL real inventory did not exit 0"; fail=1; }
T=$(mktemp -d)
python3 - "$INV" "$T" <<'P'
import json,sys
inv=json.load(open(sys.argv[1])); t=sys.argv[2]
m=json.loads(json.dumps(inv)); m['units'].pop(); json.dump(m,open(t+'/drop_unit.json','w'))
m=json.loads(json.dumps(inv)); m['totals']['units']+=1; json.dump(m,open(t+'/bump_total.json','w'))
m=json.loads(json.dumps(inv)); b=m['books'][0]; k=next(iter(b['dispositions'])); b['dispositions'][k]+=1; json.dump(m,open(t+'/bump_bucket.json','w'))
m=json.loads(json.dumps(inv)); m['units'][0]['book']=m['units'][-1]['book'] if m['units'][0]['book']!=m['units'][-1]['book'] else 'x'; json.dump(m,open(t+'/move_unit.json','w'))
m=json.loads(json.dumps(inv)); m['units'].append(dict(m['units'][0])); m['totals']['units']+=1; json.dump(m,open(t+'/dup_unit.json','w'))
m=json.loads(json.dumps(inv)); m['excluded'][0]['rows']+=1; json.dump(m,open(t+'/excluded_rows.json','w'))
P
for f in drop_unit bump_total bump_bucket move_unit dup_unit excluded_rows; do
  $CHECK --inventory "$T/$f.json" >"$T/$f.out" 2>&1; rc=$?
  if [ $rc -eq 1 ]; then echo "PASS planted $f exits $rc: $(tail -1 "$T/$f.out")"; else echo "FAIL planted $f exited $rc (want 1)"; fail=1; fi
done
rm -rf "$T"
exit $fail
