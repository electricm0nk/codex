import os,sys,glob
S=os.path.expanduser("~/workspace/repos/pcgen/data/starfinder")
INSCOPE=["paizo/core","paizo/armory","paizo/character_operations_manual","paizo/pact_worlds","paizo/near_space","paizo/alien_archive","paizo/alien_archive_2","paizo/alien_archive_3"]
terms=sys.argv[1].split(",")
files=[]
for b in INSCOPE:
    for root,dirs,fs in os.walk(os.path.join(S,b)):
        if "_society" in root.split(os.sep): continue
        files+= [os.path.join(root,f) for f in fs if f.endswith(".lst")]
rows=0; hits={t:0 for t in terms}; anyhit=0
for f in sorted(files):
    for line in open(f,encoding="latin-1"):
        if not line.strip() or line.startswith("#"): continue
        if line.startswith(("SOURCELONG","SOURCESHORT","SOURCEWEB","SOURCEDATE")): continue
        rows+=1; h=False
        for t in terms:
            if t in line: hits[t]+=1; h=True
        anyhit+=h
print("files",len(files),"rows",rows,"rows_with_any_hit",anyhit)
for t in terms: print(f"{hits[t]:6d}  {t}")
