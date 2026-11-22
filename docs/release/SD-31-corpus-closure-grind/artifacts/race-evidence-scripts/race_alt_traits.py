import os as _os
OUT_DIR = _os.path.dirname(_os.path.abspath(__file__))
import os, re, sys, types, json, collections

sys.modules['pf1e_dashboard_producer'] = types.ModuleType('stub')
import importlib.util
spec = importlib.util.spec_from_file_location(
    "build", "docs/release/SD-31-corpus-closure-grind/artifacts/supersession_register_build.py")
build = importlib.util.module_from_spec(spec)
spec.loader.exec_module(build)

ROOT = "/home/ubuntu/workspace/repos/pcgen/data"

r = json.load(open(OUT_DIR + "/race-citations.json"))
race_names = {v["name"] for v in r.values()}

key_decl_re = re.compile(r'^([A-Za-z0-9 \'\-]+)\s+KEY:([A-Za-z0-9\'\- ]+) ~ ([A-Za-z0-9\'\-/&,\.\(\) ]+?)\s+CATEGORY')

per_race_per_book = collections.defaultdict(lambda: collections.defaultdict(list))

for book, reldir in build.BOOK_DIRS.items():
    if book == "core_essentials":
        continue
    bookdir = os.path.join(ROOT, reldir)
    if not os.path.isdir(bookdir):
        continue
    for dirpath, dirs, files in os.walk(bookdir):
        if "/_pfs/" in dirpath + "/":
            continue
        for fn in files:
            if not fn.endswith(".lst"):
                continue
            path = os.path.join(dirpath, fn)
            with open(path, errors="replace") as fh:
                for lineno, line in enumerate(fh, 1):
                    if ".MOD" in line.split("\t")[0]:
                        continue  # only NEW declarations, not citations of existing keys
                    m = key_decl_re.match(line)
                    if not m:
                        continue
                    race_part = m.group(2).strip()
                    trait_name = m.group(3).strip()
                    if race_part not in race_names:
                        continue
                    per_race_per_book[race_part][book].append({
                        "trait": trait_name, "file": os.path.relpath(path, ROOT), "line": lineno,
                    })

out = {race: {book: traits for book, traits in books.items()} for race, books in per_race_per_book.items()}
json.dump(out, open(OUT_DIR + "/race-alt-traits.json", "w"), indent=2)

total_races_with_alts = len(out)
total_traits = sum(len(traits) for books in out.values() for traits in books.values())
print(f"races with alternate-trait content in a non-home book: {total_races_with_alts}")
print(f"total new alternate-trait KEY declarations found: {total_traits}")
for race in sorted(out)[:5]:
    print(race, {b: len(t) for b, t in out[race].items()})
