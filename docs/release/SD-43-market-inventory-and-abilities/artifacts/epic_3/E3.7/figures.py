import json,glob,re,collections,sys
def slug(s): return re.sub(r'_+','_',re.sub(r'[^a-z0-9]+','_',s.lower())).strip('_')
recs={}
for f in glob.glob('data/sheet_rules/*/class_feature/*.json'):
    for r in json.load(open(f)):
        if r.get('pool')=='archetype' and '#' not in r['id'] and '_archetype_' in r['id']: recs[r['id'].split(':',2)[2]]=r
cat=[]
for f in glob.glob('data/rules_tables/*/archetype_tables/archetype_swap_tables.json'):
    d=json.load(open(f)); rows=d if isinstance(d,list) else next(v for v in d.values() if isinstance(v,list)); cat+=rows
mapped=[r for r in cat if f"{slug(r['subject'])}_archetype_{slug(r['archetype_name'])}" in recs]
un=collections.Counter(r['subject'] for r in cat if r not in mapped)
def tags(r): return [x for x in r.get('tags',[]) if not x.endswith('Archetype') and not x.upper().startswith('PFS') and not x.startswith('[')]
catrep={f"{slug(r['subject'])}_archetype_{slug(r['archetype_name'])}":(r.get('replaces') or []) for r in cat}
noslot=[s for s,r in recs.items() if not tags(r) and not catrep.get(s)]
print('archetype records',len(recs)); print('catalog entries',len(cat),'mapped',len(mapped),'unmapped by subject',dict(un))
print('records with no replaced feature (tags or catalog)',len(noslot))
