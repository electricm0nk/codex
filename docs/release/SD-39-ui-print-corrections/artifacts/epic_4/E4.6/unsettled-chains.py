"""Flat standing chains left unsettled after E4.6, from the raw corpus. Run from the repo root."""
import collections, glob, json
counts, items = collections.Counter(), collections.defaultdict(list)
for path in glob.glob('data/corpus/*/equipment/**/*.json', recursive=True):
    data = json.load(open(path))['data']
    for chain in data.get('raw_bonus_chains') or []:
        q = chain['qualifiers']
        if len(q) < 3 or any(x.startswith(('PRE', '!PRE')) and not (x.startswith('PREVAREQ:Disable') and x.endswith(',0')) for x in q[3:]):
            continue
        names = q[1].split(',')
        if q[0] == 'MOVEMULT':
            key = 'multiplier ' + q[1]
        else:
            try:
                int(q[2].removesuffix('+Global_LuckBonus'))
            except ValueError:
                continue
            if q[0] == 'SKILL' and any(n in ('ALL', 'LIST') or n.startswith('TYPE') for n in names):
                key = 'skill wildcard'
            elif q[0] == 'VAR' and any(n in ('CMB', 'CMD') for n in names):
                key = 'bare ' + next(n for n in names if n in ('CMB', 'CMD'))
            elif q[0] == 'VAR' and any(n.startswith(('CMB_', 'CMD_')) for n in names):
                key = 'one maneuver'
            elif q[0] == 'COMBAT' and q[1].startswith('TOHIT.'):
                key = 'one kind of attack'
            elif q[0] == 'COMBAT' and q[1] == 'ATTACK,AC':
                key = 'AC half of ATTACK,AC'
            else:
                continue
        counts[key] += 1
        items[key].append(data.get('key'))
for key, count in sorted(counts.items()):
    print(key, count, items[key][:10])
print('total', sum(counts.values()))
