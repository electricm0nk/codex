"""SD-32 card 15 (this cycle) — per-case hand review of the 134 still-colliding
`class_feature` residual groups `15-card-15-duplicate-identity-memo.md`'s
next-cycle plan named: the 39 `TYPE:*Choice`-suffixed fallback-key collision
groups and the 16 keyed (`KEY:`-carrying) collision groups.

Per `decisions.md` SD-31 Decision 17: a bounded, evidenced per-case review,
never a live adjacency/heuristic auto-classifier. This script's job is only
to surface the SAME evidence a human reviewer would read by hand -- each
group's own ABILITY:AUTOMATIC grant target(s), TYPE:, CATEGORY:, and whether
the members' granted targets collapse to fewer distinct targets than there
are rows (the Decision-17 signature: two+ pickers granting the identical
target feature via two class/archetype gates) -- for a human/agent to read
and decide, group by group. It does not decide anything itself.

Run:
    export PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data
    python3 docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-residual-group-review.py
"""
import sys
import os
import json
from collections import defaultdict

sys.path.insert(0, "scripts")
import census_independent as ci  # noqa: E402


def ability_targets(fields: list[str]) -> list[str]:
    out = []
    for f in fields:
        fs = f.strip()
        if fs.upper().startswith("ABILITY:") and "AUTOMATIC" in fs.upper():
            parts = fs.split("|")
            if len(parts) >= 3:
                out.append(parts[-1])
    return out


def category_of(fields: list[str]):
    for f in fields:
        fs = f.strip()
        if fs.upper().startswith("CATEGORY:"):
            return fs.split(":", 1)[1]
    return None


def type_of(fields: list[str]):
    for f in fields:
        fs = f.strip()
        if fs.upper().startswith("TYPE:"):
            return fs.split(":", 1)[1]
    return None


def key_of(fields: list[str]):
    for f in fields:
        fs = f.strip()
        if fs.upper().startswith("KEY:"):
            return fs.split(":", 1)[1].strip()
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
                        rows_by_book_key[(bd.book_id, key_field, has_key)].append((bd.book_id, fn, lineno, fields))

    fallback_groups = {k: v for k, v in rows_by_book_key.items() if len(v) > 1 and k[2] is False}
    keyed_groups = {k: v for k, v in rows_by_book_key.items() if len(v) > 1 and k[2] is True}

    def content_sig(fields):
        CONTENT_PREFIXES = [
            "DEFINE:", "BONUS", "DESC:", "ASPECT:", "CSKILL:", "MOVE:", "AUTO:", "TEMPLATE:",
            "SPROP:", "QUALITY:", "SR:", "DR:", "SAB:", "VISION:", "SPELLKNOWN", "TEMPBONUS:",
            "CHOOSE:", "NATURALATTACKS:", "COMPANIONLIST:", "ADD:", "FOLLOWERS:", "UDAM:",
            "UMULT:", "SELECT:", "COST:", "MOVECLONE:", "SPELLS:", "SERVESAS:", "DEFINESTAT:",
            "UNENCUMBEREDMOVE:", "BENEFIT:", "SPELLLEVEL:", "CMB:", "ABILITY:",
        ]
        parts = [f.strip() for f in fields if f.strip() and any(f.strip().upper().startswith(p) for p in CONTENT_PREFIXES)]
        return "\t".join(sorted(parts))

    print("=== FALLBACK (no KEY:) groups, distinct-content, ALL-Choice-typed (the 39) ===\n")
    choice_group_count = 0
    for k, rows in sorted(fallback_groups.items()):
        sigs = defaultdict(list)
        for r in rows:
            sigs[content_sig(r[3])].append(r)
        if len(sigs) == 1:
            continue  # byte-identical, already correctly left to collapse
        types = [type_of(r[3]) for r in rows]
        if not all(t and t.endswith("Choice") for t in types):
            continue  # this is one of the 25 already-rescued groups
        choice_group_count += 1
        book, key, _ = k
        print(f"--- group {choice_group_count}: book={book!r} key={key!r} rows={len(rows)} ---")
        targets_seen = defaultdict(list)
        for (b, fn, ln, fields) in rows:
            cat = category_of(fields)
            typ = type_of(fields)
            tgts = ability_targets(fields)
            print(f"    {fn}:{ln}  CATEGORY={cat!r}  TYPE={typ!r}  grants={tgts}")
            for t in tgts:
                targets_seen[t].append((fn, ln))
        distinct_targets = len(targets_seen)
        print(f"    -> {len(rows)} rows, {distinct_targets} distinct non-internal ABILITY:AUTOMATIC target(s)")
        if distinct_targets < len([r for r in rows if ability_targets(r[3])]):
            print("    SIGNAL: two or more rows grant the SAME target -- Decision-17 duplicate-chooser shape")
        print()

    print(f"\nTotal Choice-typed distinct-content fallback groups: {choice_group_count}\n")

    print("=== KEYED (has KEY:) collision groups (the 16) ===\n")
    for k, rows in sorted(keyed_groups.items()):
        book, key, _ = k
        sigs = defaultdict(list)
        for r in rows:
            sigs[content_sig(r[3])].append(r)
        print(f"--- book={book!r} KEY={key!r} rows={len(rows)} distinct_content_sigs={len(sigs)} ---")
        for (b, fn, ln, fields) in rows:
            cat = category_of(fields)
            typ = type_of(fields)
            print(f"    {fn}:{ln}  CATEGORY={cat!r}  TYPE={typ!r}")
        print()


if __name__ == "__main__":
    main()
