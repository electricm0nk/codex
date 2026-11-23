"""E3.8 figures. Run from the repo root after
`eval "$(bash scripts/fetch-pcgen-oracle.sh --check --quiet)"`:
    python3 -I docs/release/SD-43-market-inventory-and-abilities/artifacts/epic_3/E3.8/figures.py
(a) oracle favored class bonus options: a non-comment, non-.MOD/.COPY/.FORGET line of a Paizo
    pathfinder .lst whose CATEGORY is Special Ability and whose TYPE list holds FavoredClassBonus;
(b) package options: a data/sheet_rules record whose tags hold FavoredClassBonus."""
import glob, json, os, re

root = os.environ["PCGEN_CORPUS_ROOT"]
oracle = 0
for path in glob.glob(os.path.join(root, "pathfinder", "paizo", "**", "*.lst"), recursive=True):
    with open(path, encoding="utf-8", errors="replace") as handle:
        for line in handle:
            if line.startswith("#"):
                continue
            fields = line.rstrip("\n").split("\t")
            if re.search(r"\.(MOD|COPY|FORGET)", fields[0]):
                continue
            types = [f[5:] for f in fields if f.startswith("TYPE:")]
            if "CATEGORY:Special Ability" in fields and any("FavoredClassBonus" in t.split(".") for t in types):
                oracle += 1
package = 0
labels = []
for path in glob.glob("data/sheet_rules/*/*/*.json"):
    try:
        records = json.load(open(path))
    except Exception:
        continue
    if isinstance(records, list):
        for record in records:
            if isinstance(record, dict) and "FavoredClassBonus" in (record.get("tags") or []):
                package += 1
                labels.append(record["label"])
print(f"oracle favored class bonus options {oracle}")
print(f"package favored class bonus options {package} {sorted(labels)}")
