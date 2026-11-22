import json, sys, math, re
dump=sys.argv[1]
sk={r["name"]:r for r in json.load(open(sys.argv[2]+"/skill.json"))}
def slug(t): return re.sub(r"_+","_",re.sub(r"[^a-z0-9]+","_",t.lower())).strip("_")
seeds=[("00000000-0000-0000-0000-000000000004","Jax Moreno","Streetrat Solo"),("00000000-0000-0000-0000-000000000005","Nyx Calder","Complete Package Netrunner"),("00000000-0000-0000-0000-000000000006","Imani Okafor","Edgerunner Medtech")]
L=["INT","REF","DEX","TECH","COOL","WILL","LUCK","MOVE","BODY","EMP"]
for cid,name,kind in seeds:
    d=f"{dump}/{cid}"
    ld=json.load(open(d+"/loaded.json"))
    rows={e['id']:e['value'] for e in ld['explanations']}
    print(f"\n### {name} — {kind} (`{cid}`)\n")
    print("**STATs** (p. 72):\n")
    print("| "+" | ".join(L)+" |"); print("|"+"---|"*10)
    print("| "+" | ".join(str(rows['cpr.stat.'+s.lower()]) for s in L)+" |")
    ch=open(d+"/cpr_sheet_choices.txt").read().splitlines()
    print("\n**Skills** (level → Skill Base = current STAT + level, p. 80/128; every other skill is level 0):\n")
    print("| Skill | Level | Base |"); print("|---|---|---|")
    for l in ch:
        k,_,v=l.partition(" = ")
        if k.startswith("skill."):
            key=k[6:]; base=key.split(" (")[0] if key not in sk else key
            stat=sk[base]["stat"]
            cur={"REF":rows["cpr.armor.ref"],"DEX":rows["cpr.armor.dex"],"EMP":rows["cpr.derived.emp_current"]}.get(stat, rows["cpr.stat."+stat.lower()])
            rest=slug(base)+("."+slug(key[len(base)+2:-1]) if key!=base else "")
            got=rows["cpr.skill."+rest]; assert got==cur+int(v),(key,got,cur,v)
            print(f"| {key} | {v} | {stat} {cur} + {v} = {got} |")
    print("\n**Saved sheet choices** (`cpr_sheet_choices.txt`, non-skill lines):\n")
    for l in ch:
        if not l.startswith("skill."): print(f"- `{l}`")
    rc=open(d+"/cpr_role_choices.txt").read().splitlines()
    print("\n**Role Ability sub-choices** (`cpr_role_choices.txt`): " + (", ".join(f"`{l}`" for l in rc) if rc else "none (the Netrunner's Interface has no sub-choice at Rank 4)"))
    print("\n**Kit** (`cpr_kit.txt`, section | quantity | item):\n")
    for l in open(d+"/cpr_kit.txt").read().splitlines():
        print(f"- {l.partition(' = ')[2]}")
    print("\n**Lifepath** (`cpr_lifepath.txt`, table | entry | column | answer):\n")
    for l in open(d+"/cpr_lifepath.txt").read().splitlines():
        parts=l.partition(" = ")[2].split(" | ")
        print(f"- {parts[0]} [{parts[1]}] {parts[2]}: {parts[3]}")
