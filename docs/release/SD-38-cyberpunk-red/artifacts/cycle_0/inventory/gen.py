import re,sys,math
P='/home/ubuntu/workspace/inbox/cyberpunk-red/pages/'
roles=[]
cur=None;tabs={}
for p in ('075','076','077','078'):
    for l in open(P+f'p{p}.txt',encoding='utf8'):
        m=re.match(r'^\s*▶ (Rockerboy|Solo|Netrunner|Tech|Medtech|Media|Lawman|Exec|Fixer|Nomad)\s*$',l)
        if m: cur=m.group(1); tabs.setdefault(cur,[]); continue
        if l.strip().startswith('▶ Method #2'): cur=None
        m=re.match(r'^\s*(\d+)((?:\s+\d+){10})\s*$',l)
        if m and cur: tabs[cur].append(list(map(int,m.group(2).split())))
out=[]
allv=[v for r in tabs.values() for row in r for v in row]
out.append(f"ROLES={list(tabs)} rows={[len(v) for v in tabs.values()]} min={min(allv)} max={max(allv)}")
cols="INT REF DEX TECH COOL WILL LUCK MOVE BODY EMP".split()
for c,i in zip(cols,range(10)):
    vs=[row[i] for r in tabs.values() for row in r]
    out.append(f"{c}: min {min(vs)} max {max(vs)}")
# per role roll tables with sums
md=[]
for role,rows in tabs.items():
    md.append(f"\n**{role}** (p075-p078; roll = 1d10 row)\n\n| Roll | "+" | ".join(cols)+" | Sum |\n|"+"---|"*12)
    for i,row in enumerate(rows,1):
        md.append(f"| {i} | "+" | ".join(map(str,row))+f" | {sum(row)} |")
    sums=[sum(r) for r in rows]
    md.append(f"\nRow sums for {role}: min {min(sums)}, max {max(sums)} (derived with sum of the 10 values per row)")
open(P.replace('pages/','')+'/../../../.claude/dummy','w') if False else None
open('/tmp/claude-1000/-home-ubuntu-workspace-repos-codex/f61b82f6-6418-42af-8873-d3df5da582e6/scratchpad/stat_tables.md','w').write("\n".join(md))
print("\n".join(out))
# HP table check
lines=[l for l in open(P+'p080.txt',encoding='utf8') if re.match(r'^\s*(?:[A-Z]*\s+)?\d+(\s+\d+){14}\s*$',l.strip()) or re.match(r'^\s*\d+(\s+\d+){14}\s*$',l)]
rows=[]
for l in open(P+'p080.txt',encoding='utf8'):
    m=re.match(r'^\s*(\d+)((?:\s+\d+){14})\s*$',l)
    if m: rows.append((int(m.group(1)),list(map(int,m.group(2).split()))))
print("HP rows",[r[0] for r in rows])
bad=0;n=0
for will,vals in rows:
    for body,v in zip(range(2,16),vals):
        n+=1
        f=10+5*math.ceil((body+will)/2)
        if f!=v: bad+=1;print("MISMATCH",will,body,v,f)
print("HP cells",n,"mismatches vs 10+5*ceil((BODY+WILL)/2):",bad)
# HP rows missing: WILL 9? table shows WILL 2..10 = 9 rows
