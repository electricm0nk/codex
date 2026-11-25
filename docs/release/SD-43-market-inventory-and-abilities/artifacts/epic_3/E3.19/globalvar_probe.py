# Proxy: CATEGORY=Class|<C>.MOD rows' ABILITY:...|AUTOMATIC|<x ~ y> keys in each in-scope book's
# *_abilities_globalvar.lst, whose (class, key) the committed data/class_feature_grants/<book>/ lacks.
import glob, json, os, sys
oracle, repo = sys.argv[1], sys.argv[2]
books = {"advanced_class_guide": "acg", "advanced_players_guide": "apg", "core_rulebook": "cr",
         "ultimate_magic": "um", "ultimate_combat": "uc", "advanced_race_guide": "arg", "bestiary_4": "b4"}
for book, pre in books.items():
    f = f"{oracle}/pathfinder/paizo/roleplaying_game/{book}/{pre}_abilities_globalvar.lst"
    if not os.path.exists(f): continue
    have = set()
    for j in glob.glob(f"{repo}/data/class_feature_grants/{book}/*.json"):
        have |= {(r["class"], r["key"]) for r in json.load(open(j))}
    raw = set()
    for line in open(f, encoding="utf-8", errors="replace"):
        fs = line.rstrip("\r\n").split("\t")
        if not fs[0].startswith("CATEGORY=Class|") or not fs[0].endswith(".MOD"): continue
        cls = fs[0][len("CATEGORY=Class|"):-4].split(" ~ ")[0]
        for t in fs[1:]:
            segs = t.split("|")
            if not t.startswith("ABILITY:") or len(segs) < 3 or segs[1] != "AUTOMATIC": continue
            for s in segs[2:]:
                if s.startswith("PRE") or s.startswith("!PRE"): break
                if " ~ " in s: raw.add((cls, s))
    print(book, "raw", len(raw), "absent", len(raw - have))
