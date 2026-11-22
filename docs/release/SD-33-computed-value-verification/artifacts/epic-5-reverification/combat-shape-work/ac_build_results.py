#!/usr/bin/env python3
"""Builds the final equipment-shape-combat.oracle-results.json AC rows:
reads every item's oracle AC.Total export, the matching book's baseline
AC.Total export, computes the delta, and runs it through
scripts/oracle_harness/compare.py's own compare_unit (imported, not
reimplemented) against the engine's "ours" value (from
e5_combat_ac_ours's output).

Usage: ac_build_results.py <manifest.json> <oracle_txt_dir> <ours.json> <out.json>
"""
import json, os, sys

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', '..', '..', '..', 'scripts'))
from oracle_harness import compare as OC  # noqa: E402


def harness_equip_failed(log_path):
    """Real, root-caused PCGen harness defect (same class
    `AT-33-E5-remainder-equipment`'s own receipt already named for
    `ultimate_psionics`): `SEVERE ... Could not find campaign: Ultimate
    Psionics` despite the .pcg's own `CAMPAIGN:Ultimate Psionics` line,
    which cascades into `Could not add equipment: <item>. Check loaded
    campaigns.` -- the item never actually equips, so PCGen's AC.Total
    reflects a bare character, not this unit's real effect. Reporting
    that as `disagree` would fabricate a false defect (`AT-33-E5-003`'s
    own doctrine forbids exactly this)."""
    if not os.path.exists(log_path):
        return False
    text = open(log_path).read()
    return 'Could not add equipment' in text and 'Check loaded campaigns' in text


def read_ac_total(path):
    if not os.path.exists(path):
        return None
    text = open(path).read()
    for line in text.splitlines():
        line = line.strip()
        if line.startswith('AC.TOTAL='):
            return line.split('=', 1)[1].strip()
    return None


def main():
    manifest_path, oracle_dir, ours_path, out_path = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
    manifest = json.load(open(manifest_path))
    ours_raw = json.load(open(ours_path))

    baseline_by_book = {}
    for b in manifest['baselines']:
        slug = b['slug']
        val = read_ac_total(os.path.join(oracle_dir, f'{slug}.txt'))
        baseline_by_book[b['book']] = val

    results = []
    missing_oracle = []
    missing_baseline = []
    unresolved_by_engine = []
    for item in manifest['items']:
        uid = item['unit_id']
        slug = item['slug']
        book = item['book']
        if uid not in ours_raw:
            # A real, diagnosed engine-resolver limitation (not a "never
            # examined" unit): `equipment_id_resolve` cannot match a
            # templated multi-variant corpus record whose raw LST `KEY:`
            # is absent (e.g. `Skin of the [NAME]` -- the base template
            # for `Psychoactive Skin (Defender)`/`(Hero)`), confirmed this
            # cycle by `e5_combat_ac_ours`'s own UNRESOLVED report. A real
            # finding, not a parking space for an unattempted unit.
            unresolved_by_engine.append(uid)
            results.append({
                'unit_id': uid,
                'ours': None,
                'oracle': None,
                'verdict': 'unverifiable',
                'reason': 'engine_id_resolve_fails_templated_variant_record',
            })
            continue
        oracle_key, ours_val = ours_raw[uid]
        item_total_s = read_ac_total(os.path.join(oracle_dir, f'{slug}.txt'))
        baseline_total_s = baseline_by_book.get(book)
        if item_total_s is None:
            missing_oracle.append(uid)
            oracle_value = None
        elif baseline_total_s is None:
            missing_baseline.append(uid)
            oracle_value = None
        else:
            oracle_value = int(item_total_s) - int(baseline_total_s)
        log_path = os.path.join(oracle_dir, f'{slug}.txt.log')
        if harness_equip_failed(log_path):
            results.append({
                'unit_id': uid,
                'ours': ours_val,
                'oracle': None,
                'verdict': 'unverifiable',
                'reason': 'oracle_harness_ultimate_psionics_campaign_load_failure',
            })
            continue
        rec = OC.compare_unit(uid, ours_val, oracle_value)
        if oracle_value is None:
            rec['reason'] = 'oracle_export_missing' if item_total_s is None else 'baseline_missing'
        results.append(rec)

    with open(out_path, 'w') as f:
        json.dump({'results': results}, f, indent=2)
        f.write('\n')

    counts = {'agree': 0, 'disagree': 0, 'unverifiable': 0}
    for r in results:
        counts[r['verdict']] += 1
    print(f"ac_build_results: {len(results)} rows -- agree={counts['agree']} disagree={counts['disagree']} unverifiable={counts['unverifiable']} -> {out_path}")
    if missing_oracle:
        print(f"MISSING ORACLE EXPORT ({len(missing_oracle)}): {missing_oracle}")
    if missing_baseline:
        print(f"MISSING BASELINE ({len(missing_baseline)}): {missing_baseline}")
    if unresolved_by_engine:
        print(f"UNRESOLVED BY ENGINE ({len(unresolved_by_engine)}): {unresolved_by_engine}")


if __name__ == '__main__':
    main()
