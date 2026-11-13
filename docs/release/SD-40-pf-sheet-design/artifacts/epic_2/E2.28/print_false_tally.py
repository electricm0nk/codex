import json,glob,re,sys
fams={"SaveBonus","AllSaveBonus","Save Bonus","RefSaveBonus","WillSaveBonus","ModifyAC"}
n={True:0,False:0}; cm={True:0,False:0}; rows=[]
for f in sorted(glob.glob(sys.argv[1]+'/data/sheet_rules/*/*/*.json')):
    d=json.load(open(f))
    rules=d if isinstance(d,list) else d.get('rules',[d])
    for r in rules:
        if not isinstance(r,dict) or 'id' not in r: continue
        for seg in r.get('prose',[]) or []:
            fam=seg.get('family'); name=fam.get('Aspect') if isinstance(fam,dict) else None
            if name in fams:
                n[r.get('print',True)]+=1
                if not r.get('print',True): rows.append((r['id'],name))
            if name=='CombatBonus':
                t=''.join(p.get('Text','') for p in seg.get('pieces',[]) if isinstance(p,dict) and isinstance(p.get('Text'),str)).lower()
                if re.search(r'\bcmb\b|\bcmd\b|combat maneuver',t):
                    cm[r.get('print',True)]+=1
                    if not r.get('print',True): rows.append((r['id'],name))
print('six families by print:',n); print('combat maneuver CombatBonus by print:',cm)
for x in rows: print(*x)
