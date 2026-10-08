#!/usr/bin/env python3
"""SD-37 E4.2: an independent census of the Internal selection hop, from the oracle's own rows.

Implementation B (the converter is implementation A). Reads the pinned oracle's Starfinder
`.lst` files directly (no converter code): every `CATEGORY:Internal` row (base row and `.MOD`s)
reached by an `ABILITY:Internal|AUTOMATIC|...` grant on a `CATEGORY:Race` ability row of an
in-scope book, whose every `ABILITY:` grant is a plain AUTOMATIC one (no `%LIST`, no `TYPE=`),
and counts the target keys those grants name.

usage (repo root): python3 E4.2_hop_census.py <converted package dir> <baseline package dir>
Prints `hop_rows=<n> hop_targets=<n>` (oracle side) and, for each package, the `granted_by`
edges whose granter is a converted Race-category ability record (`pool: race`). Exit 0 when the
edges the change added (new - baseline) equal the oracle's hop target count.
"""
import glob, json, os, re, subprocess, sys
from collections import Counter

pkg, base = sys.argv[1], sys.argv[2]
env = subprocess.run(["bash", "scripts/fetch-pcgen-oracle.sh", "--check", "--quiet"], capture_output=True, text=True).stdout
root = re.search(r"PCGEN_REPO_DIR=['\"]?([^'\"\n]+)", env).group(1)
BOOKS = ["core", "armory", "character_operations_manual", "pact_worlds", "near_space", "alien_archive", "alien_archive_2", "alien_archive_3"]


def rows():
    for b in BOOKS:
        for f in sorted(glob.glob(os.path.join(root, "data/starfinder/paizo", b, "*.lst"))):
            for line in open(f, encoding="utf-8", errors="replace"):
                t = [x for x in line.rstrip("\n").split("\t") if x.strip()]
                if t and not t[0].startswith("#"):
                    yield b, t


internal = {}  # KEY upper -> [ABILITY values]
race_abilities = []  # (book, name, [ABILITY values])
for b, t in rows():
    head = t[0].strip()
    toks = [(x.split(":", 1)[0], x.split(":", 1)[1]) for x in t[1:] if ":" in x]
    abil = [v for k, v in toks if k == "ABILITY"]
    if head.startswith("CATEGORY=Internal|") and head.endswith(".MOD"):
        internal.setdefault(head[len("CATEGORY=Internal|"):-4].strip().upper(), []).extend(abil)
    elif ("CATEGORY", "Internal") in toks:
        key = next((v for k, v in toks if k == "KEY"), head).strip().upper()
        internal.setdefault(key, []).extend(abil)
    elif ("CATEGORY", "Race") in toks:
        race_abilities.append((b, head, abil))


def plain(v):
    f = v.split("|")
    targets = [x for x in f[2:] if not x.startswith("PRE") and not x.startswith("!PRE")]
    return len(f) >= 3 and f[1].strip().upper() == "AUTOMATIC" and all("%LIST" not in x and not x.startswith("TYPE") for x in targets)


hop_rows, targets = 0, Counter()
for b, name, abil in race_abilities:
    for v in abil:
        f = v.split("|")
        if f[0].strip().upper() != "INTERNAL" or len(f) < 3:
            continue
        for key in [x.strip().upper() for x in f[2:] if not x.startswith("PRE") and not x.startswith("!PRE")]:
            inner = internal.get(key)
            if not inner or not all(plain(x) for x in inner):
                continue
            hop_rows += 1
            for x in inner:
                for tk in [y.strip() for y in x.split("|")[2:] if y.strip() and not y.startswith("PRE") and not y.startswith("!PRE")]:
                    targets[(b, name, tk.upper())] += 1
print(f"race_ability_rows={len(race_abilities)} hop_rows={hop_rows} hop_targets={sum(targets.values())}")

def race_edges(pkg):
    ids, edges = set(), 0
    for f in glob.glob(os.path.join(pkg, "*/ability/*.json")):
        for r in json.load(open(f)):
            if "#" not in r["id"] and r.get("pool") == "race":
                ids.add(r["id"])
    for f in glob.glob(os.path.join(pkg, "*/*/*.json")):
        try:
            rs = json.load(open(f))
        except Exception:
            continue
        if not isinstance(rs, list) or not rs or not isinstance(rs[0], dict):
            continue
        for g in rs[0].get("granted_by") or []:
            if g.get("by", {}).get("Rule") in ids:
                edges += 1
    return len(ids), edges


(n_new, e_new), (n_old, e_old) = race_edges(pkg), race_edges(base)
print(f"race_ability_records={n_new} (baseline {n_old}) race_ability_edges={e_new} (baseline {e_old}) added={e_new - e_old}")
sys.exit(0 if e_new - e_old == sum(targets.values()) else 1)
