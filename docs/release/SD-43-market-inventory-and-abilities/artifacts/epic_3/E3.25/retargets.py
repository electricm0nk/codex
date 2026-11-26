"""E3.25: per rule file changed vs a base commit, the `"Rule":"<id>"` references gained and lost (a lost one is a
retarget or a removal). Run from the repo root: python3 -I .../E3.25/retargets.py <base-commit>"""
import subprocess, re, collections, sys, json
BASE = sys.argv[1] if len(sys.argv) > 1 else "HEAD"
files = [l for l in subprocess.run(["git","diff","--name-only",BASE],capture_output=True,text=True).stdout.splitlines() if l.startswith("data/sheet_rules/") and not l.startswith("data/sheet_rules/_")]
rx = re.compile(r'"Rule":"([^"]+)"')
lost = collections.Counter(); gained = 0; nf=0
for f in files:
    old = subprocess.run(["git","show",BASE+":"+f],capture_output=True,text=True).stdout
    new = open(f).read()
    o, n = collections.Counter(rx.findall(old)), collections.Counter(rx.findall(new))
    d = o - n
    if d: nf += 1; lost.update({(f,k):v for k,v in d.items()})
    gained += sum((n-o).values())
print("files", len(files), "Rule refs gained", gained, "Rule refs lost", sum(lost.values()), "files losing", nf)
for k,v in list(lost.items())[:10]: print(v,k)
byf = collections.Counter()
for (f,k),v in lost.items(): byf[f]+=v
print("granted_by losses in granted_by vs other:")
for f,v in byf.most_common(): 
    rec=json.load(open(f)) if False else None
    print(v, f)
