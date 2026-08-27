#!/usr/bin/env python3
"""sd33-r3-statsave: full-record classification of this lane's 160-unit
population (re-derived from wave-2's own census files -- see receipt for the
subtraction command). Reads the WHOLE corpus record for every unit (not a
filtered BONUS/PRE view) and decides, per unit, whether ANY existing
equipment_effects resolver (general::compute_general_effect [SKILL],
magic_items::compute_magic_items_effect [STAT], equipmods::compute_equipmods_effect
[WEAPON+TYPE=Enhancement] / resolve_spell_resistance_bonus [SR], arms_armor's
ACCHECK/MAXDEX/SPELLFAILURE/BONUS:COMBAT|AC) would return Some(...) for it --
mirroring compute_equipment_effects' own unconditional four-resolver dispatch
(src/rules_core/equipment_effects.rs) exactly.
"""
import json, glob, sys

D = 'docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification'

mine = json.load(open('/tmp/mine_pop.json'))['mine']
mine_ids = [uid for uid, labs in mine]

inv = json.load(open('docs/work-inventory.json'))
units_by_id = {u['id']: u for u in inv['units']}

book_index = {}
for path in glob.glob('data/corpus/*/equipment/**/*.json', recursive=True):
    parts = path.split('/')
    book_index[(parts[2], parts[-1][:-5])] = path
BOOK_ALIASES = {'bestiary': 'beastiary'}
for (book, key), path in list(book_index.items()):
    for canon, actual in BOOK_ALIASES.items():
        if book == actual:
            book_index.setdefault((canon, key), path)


def resolver_probe(chains):
    """Mirror compute_equipment_effects' 4 resolvers' find_map logic. Returns
    a dict of which resolver(s) would return Some, and the raw qualifiers
    matched (first hit only, exactly like the real .find_map order)."""
    hits = {}
    for c in chains:
        q = c['qualifiers']
        if len(q) >= 3 and q[0] == 'SKILL' and 'skill' not in hits:
            hits['skill'] = q
        if len(q) >= 3 and q[0] == 'STAT' and 'stat' not in hits:
            hits['stat'] = q
        if q[0] == 'WEAPON' and any('TYPE=Enhancement' in x or x == 'TYPE=Enhancement' for x in q) and 'weapon_enh' not in hits:
            hits['weapon_enh'] = q
        if q[0] == 'COMBAT' and len(q) >= 2 and q[1] == 'AC' and 'combat_ac' not in hits:
            hits['combat_ac'] = q
    return hits


results = []
missing = []
for uid in mine_ids:
    u = units_by_id.get(uid)
    if u is None:
        missing.append(uid)
        continue
    parts = uid.split(':')
    path = book_index.get((parts[0], parts[2]))
    if path is None:
        missing.append(uid)
        continue
    rec = json.load(open(path))['data']
    chains = rec.get('raw_bonus_chains', [])
    # also grab ACCHECK/MAXDEX/SPELLFAILURE literal tokens if present (arms_armor path
    # reads these off separate record fields, not raw_bonus_chains -- check both)
    arms_armor_tokens = {
        'accheck': rec.get('armor_check_penalty'),
        'maxdex': rec.get('max_dex'),
        'spellfailure': rec.get('spell_failure'),
    }
    has_arms_armor = any(v not in (None, '') for v in arms_armor_tokens.values())
    hits = resolver_probe(chains)
    all_labels = sorted(set(c['qualifiers'][0] if c['qualifiers'][0] != 'STAT' else 'STAT_multi_or_other_slot' for c in chains))
    results.append({
        'unit_id': uid,
        'book': parts[0],
        'corpus_path': path,
        'chains': chains,
        'labels': all_labels,
        'resolver_hits': list(hits.keys()),
        'resolver_hit_detail': hits,
        'has_arms_armor_tokens': has_arms_armor,
        'has_any_probe': bool(hits) or has_arms_armor,
        'stack_mult_fields': {k: rec.get(k) for k in ('stack', 'mult', 'STACK', 'MULT') if k in rec},
    })

print('mine_ids count:', len(mine_ids), 'missing:', len(missing), missing[:5])
print('has_any_probe True:', sum(1 for r in results if r['has_any_probe']))
print('has_any_probe False (no_probe_surface candidates):', sum(1 for r in results if not r['has_any_probe']))

from collections import Counter
probe_kind = Counter()
for r in results:
    if r['has_any_probe']:
        probe_kind[tuple(sorted(r['resolver_hits'])) or ('arms_armor',)] += 1
print('probe kind breakdown (of has_any_probe):', probe_kind)

json.dump({'results': results}, open(f'{D}/statsave-full-classification.json', 'w'), indent=1)
print('wrote', f'{D}/statsave-full-classification.json')
