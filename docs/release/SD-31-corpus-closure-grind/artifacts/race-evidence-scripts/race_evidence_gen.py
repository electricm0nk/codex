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
CE_RACES = os.path.join(ROOT, "pathfinder/paizo/roleplaying_game/core_essentials/races")

# -- Step 1: for each race slug, find its RACE: base declaration line and name --
race_slugs = sorted(os.listdir(CE_RACES))
race_name_of_slug = {}
race_decl_line = {}
for slug in race_slugs:
    racesfile = os.path.join(CE_RACES, slug, f"{slug}_races.lst")
    if not os.path.isfile(racesfile):
        # filename doesn't always keep the slug's underscores (e.g. half_elf -> halfelf_races.lst)
        cand = [f for f in os.listdir(os.path.join(CE_RACES, slug)) if f.endswith("_races.lst")]
        if len(cand) == 1:
            racesfile = os.path.join(CE_RACES, slug, cand[0])
        else:
            continue
    if not os.path.isfile(racesfile):
        continue
    with open(racesfile, errors="replace") as fh:
        for i, line in enumerate(fh, 1):
            if line.startswith("#") or not line.strip():
                continue
            # first tab-delimited token before any tab is the RACE name
            name = line.split("\t")[0].strip()
            if name:
                race_name_of_slug[slug] = name
                race_decl_line[slug] = (racesfile, i, line.rstrip("\n"))
                break

print(f"# {len(race_name_of_slug)} races with a base RACE: declaration", file=sys.stderr)

# -- Step 2: per book, compute SOURCEDATE (already have book_sourcedate in build module) --
date_cache = {}
def sourcedate(book):
    if book not in date_cache:
        date_cache[book] = build.book_sourcedate(book)[0]
    return date_cache[book]

IGNORE_PREFIXES = ("SOURCE", "TYPE")

def parse_mod_tags(rest):
    tags = [t for t in rest.split("\t") if t.strip()]
    return tags

results = {}
for slug, name in race_name_of_slug.items():
    esc = re.escape(name)
    mod_re = re.compile(rf'^{esc}\.MOD\t(.*)$')
    citations = []  # (book, sourcedate, sourcepage, file, line, extra_nonsource_tags)
    for book, reldir in build.BOOK_DIRS.items():
        if book == "core_essentials":
            continue
        bookdir = os.path.join(ROOT, reldir)
        if not os.path.isdir(bookdir):
            continue
        for dirpath, dirs, files in os.walk(bookdir):
            for fn in files:
                if not fn.endswith(".lst"):
                    continue
                path = os.path.join(dirpath, fn)
                if "/_pfs/" in path:
                    # Pathfinder Society legal-for-play companion list -- a
                    # PACKAGING duplicate within the SAME book's own release,
                    # not a second book printing; excluded so it cannot be
                    # mistaken for a genuine second citer.
                    continue
                with open(path, errors="replace") as fh:
                    for lineno, line in enumerate(fh, 1):
                        if f"{name}.MOD" not in line:
                            continue
                        m = mod_re.match(line.rstrip("\n"))
                        if not m:
                            continue
                        tags = parse_mod_tags(m.group(1))
                        sp = next((t.split(":", 1)[1] for t in tags if t.startswith("SOURCEPAGE:")), None)
                        nonsource = [t for t in tags if not any(t.startswith(p) for p in IGNORE_PREFIXES)]
                        citations.append({
                            "book": book, "source_date": sourcedate(book), "source_page": sp,
                            "file": os.path.relpath(path, ROOT), "line": lineno,
                            "nonsource_tags": nonsource,
                        })
    results[slug] = {"name": name, "citations": citations}

with_multi = {s: r for s, r in results.items() if len(r["citations"]) >= 2}
print(f"# {len(with_multi)} races cited (RACE:.MOD) by 2+ in-mandate books", file=sys.stderr)

json.dump(results, open(OUT_DIR + "/race-citations.json", "w"), indent=2)
print("wrote race-citations.json", file=sys.stderr)
