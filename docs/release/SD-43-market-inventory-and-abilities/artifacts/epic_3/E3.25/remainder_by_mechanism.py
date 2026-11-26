"""E3.25: classify every row_key_misses.py hit by mechanism (run from the repo root with $PCGEN_CORPUS_ROOT):
python3 -I docs/release/SD-43-market-inventory-and-abilities/artifacts/epic_3/E3.25/remainder_by_mechanism.py"""
import glob, json, os, collections
root = os.environ["PCGEN_CORPUS_ROOT"]
lines = {}
decl = collections.defaultdict(list)
for p in glob.glob("data/corpus/*/*/*.json"):
    try: r = json.load(open(p))
    except Exception: continue
    d, s = r.get("data") or {}, r.get("source") or {}
    if ":" not in str(d.get("key") or "") or not s.get("path") or not s.get("line"): continue
    f = os.path.join(root, s["path"])
    if f not in lines:
        try: lines[f] = open(f, encoding="utf-8", errors="replace").read().split("\n")
        except OSError: lines[f] = []
    row = lines[f][s["line"]-1] if s["line"] <= len(lines[f]) else ""
    toks = [t for t in row.split("\t") if t]
    kv = dict(t.split(":", 1) for t in toks[1:] if ":" in t)
    if "KEY" in kv and "CATEGORY" in kv:
        shipped = any(isinstance(t, dict) and t.get("key") == "CATEGORY" for t in (d.get("raw_tokens") or []))
        rule = os.path.exists("data/sheet_rules/%s/%s/%s.json" % tuple([p.split("/")[2].replace("beastiary","bestiary"), p.split("/")[3], os.path.basename(p)[:-5]]))
        decl[(kv["CATEGORY"].strip().upper(), kv["KEY"].strip().upper())].append((d["key"], shipped, rule, f"{s['path'].rsplit('/',1)[-1]}:{s['line']}"))
refs = json.load(open("data/sheet_rules/_defects/unresolved-references.json"))
refs = refs if isinstance(refs, list) else refs.get("entries", [])
c = collections.Counter(); ex = {}
for x in refs:
    if not (isinstance(x, str) and "|" in x): continue
    pair = tuple(p.strip().upper() for p in x.split(": ", 1)[-1].split("|", 1))
    if pair not in decl: continue
    ds = decl[pair]
    ids = {d[0] for d in ds}
    if not any(d[2] for d in ds): m = "target has no rule file (not in population)"
    elif any(d[1] for d in ds): m = "target ships a CATEGORY token (category not empty)"
    elif len(ids) > 1: m = "pair declared by >1 record"
    else: m = "other"
    c[m] += 1; ex.setdefault(m, (x, ds[:3]))
print(sum(c.values()))
for k, v in c.most_common(): print(v, k, ex[k])
