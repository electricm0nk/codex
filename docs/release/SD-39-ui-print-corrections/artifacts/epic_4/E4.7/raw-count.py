"""The second count of E4.7's settled rows, off the raw chains, not the settled bundles. Run from the repo root."""
import collections, glob, json
rows = collections.Counter()
for path in glob.glob('data/corpus/*/equipment/**/*.json', recursive=True):
    data = json.load(open(path))['data']
    for chain in data.get('raw_bonus_chains') or []:
        q = chain['qualifiers']
        if len(q) < 3 or any(x.startswith(('PRE', '!PRE')) and not (x.startswith('PREVAREQ:Disable') and x.endswith(',0')) for x in q[3:]):
            continue
        if q[0] == 'MOVEMULT' and q[1] in ('TYPE.All', 'TYPE=All', 'TYPE.Walk', 'TYPE=Walk'):
            rows['speed_percent'] += 1
            continue
        try:
            int(q[2].removesuffix('+Global_LuckBonus'))
        except ValueError:
            continue
        for name in q[1].split(','):
            if q[0] == 'SKILL' and name == 'ALL':
                rows['skill_set:all'] += 1
            elif q[0] == 'SKILL' and name.startswith(('TYPE.', 'TYPE=')):
                rows['skill_set:' + name[5:].lower()] += 1
            elif q[0] == 'VAR' and name in ('CMB', 'CMD'):
                rows[name.lower()] += 1
print(sorted(rows.items()), sum(rows.values()))
