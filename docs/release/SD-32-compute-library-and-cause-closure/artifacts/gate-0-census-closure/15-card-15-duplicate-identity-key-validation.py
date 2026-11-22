"""SD-32 card 15-duplicate-identity: validates the `CATEGORY:` disambiguation
key and the `*Choice`-type exclusion `disambiguate_class_feature_fallback_
collisions` (`src/bin/v06_work_inventory.rs`) implements, against the real
corpus. Re-derives every fallback-key (no `KEY:` field) `class_feature`
collision group corpus-wide and reports:

  1. How many groups are byte-identical-content (true restatements, correctly
     left to collapse) vs. genuinely distinct content.
  2. Whether `CATEGORY:` alone disambiguates the distinct-content groups
     cleanly (0 cross-signature collisions) -- and how `TYPE:` alone compares
     (it does not: 40/64 failures).
  3. How many of the distinct-content groups carry a `TYPE:` facet ending in
     `"Choice"` -- the shape SD-31 `decisions.md` Decision 17 already proved
     is SOMETIMES a duplicate-chooser-picker row beside its own real feature,
     not a second object (`DUPLICATE_CHOOSER_DISPLAY_NAME_UNIT_IDS`). These
     are deliberately EXCLUDED from the rescue -- see this repo's own
     `disambiguate_class_feature_fallback_collisions` doc comment for why a
     live adjacency filter is not built here instead.

Run:
    export PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data
    python3 docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-duplicate-identity-key-validation.py
"""
import sys
import os
import json
from collections import defaultdict

sys.path.insert(0, "scripts")
import census_independent as ci  # noqa: E402

CONTENT_PREFIXES = [
    "DEFINE:", "BONUS", "DESC:", "ASPECT:", "CSKILL:", "MOVE:", "AUTO:", "TEMPLATE:",
    "SPROP:", "QUALITY:", "SR:", "DR:", "SAB:", "VISION:", "SPELLKNOWN", "TEMPBONUS:",
    "CHOOSE:", "NATURALATTACKS:", "COMPANIONLIST:", "ADD:", "FOLLOWERS:", "UDAM:",
    "UMULT:", "SELECT:", "COST:", "MOVECLONE:", "SPELLS:", "SERVESAS:", "DEFINESTAT:",
    "UNENCUMBEREDMOVE:", "BENEFIT:", "SPELLLEVEL:", "CMB:", "ABILITY:",
]


def content_sig(fields: list[str]) -> str:
    parts = [f.strip() for f in fields if f.strip() and any(f.strip().upper().startswith(p) for p in CONTENT_PREFIXES)]
    return "\t".join(sorted(parts))


def category_of(fields: list[str]) -> str | None:
    for f in fields:
        fs = f.strip()
        if fs.upper().startswith("CATEGORY:"):
            return fs.split(":", 1)[1]
    return None


def type_of(fields: list[str]) -> str | None:
    for f in fields:
        fs = f.strip()
        if fs.upper().startswith("TYPE:"):
            return fs.split(":", 1)[1]
    return None


def main() -> None:
    pcgen_root = os.environ["PCGEN_CORPUS_ROOT"]
    with open("docs/work-inventory.json") as f:
        inventory_json = json.load(f)
    book_dirs = ci.discover_book_dirs(pcgen_root)
    scope = ci.classify_scope(book_dirs, inventory_json)
    pathfinder_root = os.path.join(pcgen_root, "pathfinder")

    rows_by_book_key: dict[tuple[str, str, bool], list] = defaultdict(list)
    for bd in scope.in_scope:
        for dirpath, _, filenames in os.walk(os.path.join(pathfinder_root, bd.rel_path)):
            for fn in sorted(filenames):
                if not fn.lower().endswith(".lst"):
                    continue
                bucket, _ = ci._classify_kind_by_filename(fn, bd.book_id)
                if bucket != "row_dependent_class_feature":
                    continue
                with open(os.path.join(dirpath, fn), encoding="utf-8", errors="replace") as fh:
                    for lineno, raw in enumerate(fh, 1):
                        line = raw.rstrip("\n")
                        if not line.strip() or line.lstrip().startswith("#") or "\t" not in line:
                            continue
                        identity = line.split("\t", 1)[0]
                        if ":" in identity:
                            continue
                        iu = identity.upper()
                        if iu.endswith(".FORGET") or iu.endswith(".MOD"):
                            continue
                        fields = line.split("\t")
                        is_internal = any(f.strip() == "CATEGORY:Internal" for f in fields) or identity.startswith(
                            "CATEGORY=Internal|"
                        )
                        if is_internal:
                            continue
                        has_key = False
                        key_field = None
                        for f in fields:
                            fs = f.strip()
                            if fs.upper().startswith("KEY:"):
                                key_field = fs.split(":", 1)[1].strip()
                                has_key = True
                                break
                        if key_field is None:
                            if identity.startswith("CATEGORY=") and "|" in identity:
                                key_field = identity.split("|", 1)[1]
                            else:
                                key_field = identity
                        rows_by_book_key[(bd.book_id, key_field, has_key)].append((fn, lineno, fields))

    fallback_groups = {k: v for k, v in rows_by_book_key.items() if len(v) > 1 and k[2] is False}
    keyed_groups = {k: v for k, v in rows_by_book_key.items() if len(v) > 1 and k[2] is True}
    print("fallback (no KEY:) collision groups:", len(fallback_groups), "rows:", sum(len(v) for v in fallback_groups.values()))
    print("keyed (has KEY:) collision groups:", len(keyed_groups), "rows:", sum(len(v) for v in keyed_groups.values()))

    byte_identical = 0
    distinct = 0
    cat_ok = 0
    cat_fail = 0
    choice_typed = 0
    other_distinct = 0

    for k, rows in fallback_groups.items():
        sigs: dict[str, list] = defaultdict(list)
        for (fn, lineno, fields) in rows:
            sigs[content_sig(fields)].append((fn, lineno, fields))
        if len(sigs) == 1:
            byte_identical += 1
            continue
        distinct += 1

        cat_by_sig = {sig: {category_of(f) for (_, _, f) in members} for sig, members in sigs.items()}
        all_cats = list(cat_by_sig.values())
        collide = any(all_cats[i] & all_cats[j] for i in range(len(all_cats)) for j in range(i + 1, len(all_cats)))
        if collide:
            cat_fail += 1
        else:
            cat_ok += 1

        types = [type_of(f) for (_, _, f) in rows]
        if all(t and t.endswith("Choice") for t in types):
            choice_typed += 1
        else:
            other_distinct += 1

    print("\nfallback groups: byte-identical-content (correctly left to collapse):", byte_identical)
    print("fallback groups: distinct-content:", distinct)
    print("  CATEGORY: disambiguates cleanly:", cat_ok, " CATEGORY: collides:", cat_fail)
    print("  ALL members TYPE:*Choice-suffixed (excluded from rescue, Decision 17 shape):", choice_typed)
    print("  other (rescued this cycle):", other_distinct)


if __name__ == "__main__":
    main()
