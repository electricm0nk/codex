#!/usr/bin/env python3
"""SD-37 E0.3 fail-closed sum check for docs/work-inventory.starfinder-1e.json.

Exit 0 only if every sum closes and the awk implementation (B) agrees with the inventory (A) per book
and per disposition. Exit 1 on ANY mismatch (all mismatches printed; last line is the verdict).
Exit 2 on usage / unreadable input.
"""
import argparse, collections, json, os, subprocess, sys

# CUI F-6 anchors (content-unit-inventory.md): in-scope rows and the three named exclusions.
ANCHOR_ROWS_IN_SCOPE = 12718
ANCHOR_EXCLUDED = {'core _society (SFS guide core mods)': 15, 'starfinder_society_rules (SSRGG)': 177,
                   'lpj_design/infinite_space (LPJ9304)': 37}
HERE = os.path.dirname(os.path.abspath(__file__))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--inventory', default='docs/work-inventory.starfinder-1e.json')
    ap.add_argument('--no-cross-check', action='store_true', help='skip implementation B (the awk script)')
    a = ap.parse_args()
    try:
        inv = json.load(open(a.inventory))
        books, units, excl, tot = inv['books'], inv['units'], inv['excluded'], inv['totals']
    except Exception as e:
        print(f'UNREADABLE {a.inventory}: {e}', file=sys.stderr); return 2
    bad = []
    def chk(ok, msg):
        if not ok: bad.append(msg)

    chk(tot['units'] == len(units), f"totals.units {tot['units']} != len(units) {len(units)}")
    ids = collections.Counter(u['id'] for u in units)
    dups = [i for i, n in ids.items() if n > 1]
    chk(not dups, f'duplicate unit ids: {len(dups)} e.g. {dups[:3]}')
    kinds = collections.Counter(u['kind'] for u in units)
    chk(dict(kinds) == tot['by_kind'], 'totals.by_kind != recount of units by kind')
    per_book_units = collections.Counter(u['book'] for u in units)
    rows_sum = 0
    for b in books:
        d = b['dispositions']
        unit_disp = sum(v for k, v in d.items() if k.startswith('unit_'))
        chk(sum(d.values()) == b['rows'], f"{b['id']}: dispositions sum {sum(d.values())} != rows {b['rows']}")
        chk(unit_disp == b['units'], f"{b['id']}: unit_* dispositions {unit_disp} != books.units {b['units']}")
        chk(per_book_units.get(b['id'], 0) == b['units'], f"{b['id']}: units listed {per_book_units.get(b['id'], 0)} != books.units {b['units']}")
        rows_sum += b['rows']
    chk(set(per_book_units) <= {b['id'] for b in books}, 'a unit names a book that is not in books[]')
    chk(rows_sum == tot['rows_in_scope'], f"sum(books.rows) {rows_sum} != totals.rows_in_scope {tot['rows_in_scope']}")
    chk(tot['rows_in_scope'] == ANCHOR_ROWS_IN_SCOPE, f"rows_in_scope {tot['rows_in_scope']} != F-6 anchor {ANCHOR_ROWS_IN_SCOPE}")
    chk({e['name']: e['rows'] for e in excl} == ANCHOR_EXCLUDED, f"excluded {[(e['name'], e['rows']) for e in excl]} != F-6 anchors {ANCHOR_EXCLUDED}")
    chk(tot['rows_excluded'] == sum(e['rows'] for e in excl), 'totals.rows_excluded != sum(excluded.rows)')
    chk(tot['rows_in_scope'] + tot['rows_excluded'] == 12947, 'in-scope + excluded != 12,947 (F-6 total)')

    if not a.no_cross_check:
        r = subprocess.run(['bash', os.path.join(HERE, 'sf_work_inventory_awk.sh')], capture_output=True, text=True)
        if r.returncode != 0:
            bad.append(f'implementation B failed rc={r.returncode}: {r.stderr.strip()[:200]}')
        else:
            B = collections.defaultdict(dict); BX = {}
            for ln in r.stdout.splitlines():
                p = ln.split('\t')
                if p[0] == 'EXCLUDED': BX[p[1]] = int(p[2])
                else: B[p[0]][p[1]] = int(p[2])
            for b in books:
                bb = B.get(b['id'], {})
                chk(bb.get('rows') == b['rows'], f"{b['id']}: B rows {bb.get('rows')} != A rows {b['rows']}")
                for k, v in b['dispositions'].items():
                    chk(bb.get(k) == v, f"{b['id']}: B {k} {bb.get(k)} != A {v}")
            chk(set(B) == {b['id'] for b in books}, 'B book set != A book set')
            chk(BX == {e['name']: e['rows'] for e in excl}, f'B excluded {BX} != A excluded')
            chk(sum(v for bb in B.values() for k, v in bb.items() if k.startswith('unit_')) == tot['units'],
                'B unit total != totals.units')
    for m in bad: print('MISMATCH ' + m)
    if bad:
        print(f'FAIL {len(bad)} mismatch(es)'); return 1
    print(f"OK units {tot['units']} rows_in_scope {tot['rows_in_scope']} excluded {tot['rows_excluded']} (A==B)")
    return 0


if __name__ == '__main__':
    sys.exit(main())
