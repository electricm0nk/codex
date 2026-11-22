#!/usr/bin/env python3
"""SD-37 E4.2: an independent census of the Internal ABILITYPOOL members (implementation B).

Reads the pinned oracle's Starfinder `.lst` files directly (no converter code): every
`ABILITYCATEGORY:<C> ... CATEGORY:Internal TYPE:<t>` that some row of an in-scope book picks from
with `BONUS:ABILITYPOOL|<C>|...` on a row that is not itself a `CATEGORY:Internal` helper, and every `CATEGORY:Internal` row of an in-scope book whose
TYPE carries all of `<t>`'s tags. One option per (book, category, row name).

usage (repo root): python3 E4.2_pool_option_census.py <converted package dir>
Prints the oracle-side option count (distinct ids, and rows before id collisions) and the
package's `pool_option` files; exit 0 when the distinct ids equal the package's files.
"""
import glob, os, re, subprocess, sys

pkg = sys.argv[1]
env = subprocess.run(["bash", "scripts/fetch-pcgen-oracle.sh", "--check", "--quiet"], capture_output=True, text=True).stdout
root = re.search(r"PCGEN_REPO_DIR=['\"]?([^'\"\n]+)", env).group(1)
BOOKS = ["core", "armory", "character_operations_manual", "pact_worlds", "near_space", "alien_archive", "alien_archive_2", "alien_archive_3"]


def slug(s):
    return re.sub(r"_+", "_", re.sub(r"[^a-z0-9]+", "_", s.lower())).strip("_")


cats, picked, members = {}, set(), []
for b in BOOKS:
    for f in sorted(glob.glob(os.path.join(root, "data/starfinder/paizo", b, "*.lst"))):
        for line in open(f, encoding="utf-8", errors="replace"):
            t = [x for x in line.rstrip("\n").split("\t") if x.strip()]
            if not t or t[0].startswith("#"):
                continue
            head = t[0].strip()
            toks = [(x.split(":", 1)[0], x.split(":", 1)[1]) for x in t[1:] if ":" in x]
            if head.startswith("ABILITYCATEGORY:"):
                d = dict(toks)
                if d.get("CATEGORY") == "Internal" and d.get("TYPE"):
                    cats[head.split(":", 1)[1].strip()] = {x.upper() for x in d["TYPE"].split(".") if x}
                continue
            # A companion modifier row (`scr_companionmods.lst`, the drone's `FOLLOWER:` rows) is a
            # companion's, not a character record's: its picks are not counted either.
            internal_row = ("CATEGORY", "Internal") in toks or head.startswith("CATEGORY=Internal|") or "companionmods" in f
            for k, v in toks:
                # A pick counts only on a row an inventory unit can stand for: never on another
                # `CATEGORY:Internal` helper (`Playable Race Selected` picks `Home Planet`).
                if k == "BONUS" and v.startswith("ABILITYPOOL|") and not internal_row:
                    picked.add(v.split("|")[1].strip())
            if ("CATEGORY", "Internal") in toks and not head.endswith(".MOD"):
                types = [v for k, v in toks if k == "TYPE"]
                members.append((b, head.split(".COPY=")[0].strip(), {x.upper() for x in (types[-1] if types else "").split(".") if x}))
ids, rows = set(), 0
for c, tags in cats.items():
    if c not in picked:
        continue
    for b, name, types in members:
        if tags <= types:
            rows += 1
            ids.add(f"{b}:pool_option:{slug(c)}_{slug(name)}")
files = len(glob.glob(os.path.join(pkg, "*/pool_option/*.json")))
print(f"internal_categories={len(cats)} picked={len([c for c in cats if c in picked])} member_rows={rows} distinct_option_ids={len(ids)} package_pool_option_files={files}")
sys.exit(0 if len(ids) == files else 1)
