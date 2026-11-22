import json,glob,collections,re
settled={}
for f in glob.glob('data/corpus/*/_settled/equipment.json'):
    d=json.load(open(f))
    for k,e in d['records'].items():
        r=e['record']; settled.setdefault(r['identity'],[]).append(r)
heads=collections.Counter(); ex=collections.defaultdict(set); miss=collections.defaultdict(list)
for path in glob.glob('data/corpus/*/equipment/**/*.json', recursive=True):
    data=json.load(open(path))['data']
    id_=data.get('key')
    for chain in data.get('raw_bonus_chains') or []:
        q=chain['qualifiers']
        if len(q)<3: continue
        if any(x.startswith(('PRE','!PRE')) for x in q[3:]): continue
        flat=re.fullmatch(r'-?\d+(\+Global_LuckBonus)?',q[2]) is not None
        if not flat: continue
        h=q[0]
        if h in('COMBAT','HP','MOVEADD','POSTMOVEADD','MOVEMULT','ABILITYPOOL','DR','CONCENTRATION','SPELLCAST','CASTERLEVEL','LOADMULT','SLOTS','MISC','MONSKILLPTS','SKILLRANK','RANGEADD','EQMWEAPON','WEAPON','FEAT','SPELLKNOWN','SPELLPOINTCOST') or h.startswith('WEAPONPROF'):
            heads[f'{h}|{q[1]}']+=1; ex[f'{h}|{q[1]}'].add(id_)
        if h in('SAVE','STAT','SKILL'):
            rs=settled.get(id_) or []
            targets=[b for r in rs for b in r['stated_bonuses']]
            if not targets and not any(r.get('var_bonuses') or r.get('ability_score_bonus') or r.get('skill_check_bonus') for r in rs):
                miss[h].append((id_,q[1],q[2],[x for x in q[3:]]))
for k,v in sorted(heads.items()): print(v,k,sorted(ex[k])[:3])
for h,v in miss.items():
    print(h,len(v)); [print('  ',x) for x in v[:30]]
