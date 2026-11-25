"""PROXY (E3.23 discovery): unresolved references (`data/sheet_rules/_defects/unresolved-references.json`) whose
`Category|Key` is the CATEGORY + KEY of a corpus record's own oracle row while that record's `data.key` is its codex id
(so no index answers the row KEY). Run from the repo root with $PCGEN_CORPUS_ROOT set:
python3 -I docs/release/SD-43-market-inventory-and-abilities/artifacts/epic_3/E3.23/row_key_misses.py"""
import glob, json, os
root = os.environ["PCGEN_CORPUS_ROOT"]
lines = {}
declared = set()
for p in glob.glob("data/corpus/*/*/*.json"):
    try:
        r = json.load(open(p))
    except Exception:
        continue
    d, s = r.get("data") or {}, r.get("source") or {}
    if ":" not in str(d.get("key") or "") or not s.get("path") or not s.get("line"):
        continue
    f = os.path.join(root, s["path"])
    if f not in lines:
        try:
            lines[f] = open(f, encoding="utf-8", errors="replace").read().split("\n")
        except OSError:
            lines[f] = []
    row = lines[f][s["line"] - 1] if s["line"] <= len(lines[f]) else ""
    toks = [t for t in row.split("\t") if t]
    kv = dict(t.split(":", 1) for t in toks[1:] if ":" in t)
    if "KEY" in kv and "CATEGORY" in kv:
        declared.add((kv["CATEGORY"].strip().upper(), kv["KEY"].strip().upper()))
refs = json.load(open("data/sheet_rules/_defects/unresolved-references.json"))
refs = refs if isinstance(refs, list) else refs.get("entries", [])
hit = [x for x in refs if isinstance(x, str) and "|" in x and tuple(p.strip().upper() for p in x.split(": ", 1)[-1].split("|", 1)) in declared]
print(f"unresolved references: {len(refs)}; naming a codex-id-keyed record's own row CATEGORY|KEY: {len(hit)}")
for x in hit[:5]:
    print(x)
