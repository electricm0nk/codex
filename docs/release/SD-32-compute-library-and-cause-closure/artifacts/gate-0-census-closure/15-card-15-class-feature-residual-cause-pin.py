"""SD-32 card 15-internal (`decisions.md §12b`): pins the cause of the
`class_feature` 179-unit residual `15-card-15-class-feature-memo.md` §3 left
unpinned ("root cause not fully pinned within this cycle's budget ... named
as a hypothesis, not a proven mechanism").

Re-derives the 179-row residual the same way that memo's own §5 script does
(census walk over `row_dependent_class_feature`-bucketed files, minus
`.MOD`/`.FORGET`, minus `CATEGORY:Internal`, minus rows already matched to
`docs/work-inventory.json` by (book, source_file, source_line)), then tests
whether each residual row's own computed identity (`KEY:` field if present,
else display name -- the SAME rule `v06_work_inventory.rs`'s `key`
computation and its corpus-wide `duplicate_identity` (kind, key) dedup use)
collides with another `class_feature` row in the same book.

Run:
    export PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data
    python3 docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-class-feature-residual-cause-pin.py
"""
import sys, os, json
from collections import Counter, defaultdict

sys.path.insert(0, "scripts")
import census_independent as ci  # noqa: E402


def main() -> None:
    pcgen_root = os.environ["PCGEN_CORPUS_ROOT"]
    with open("docs/work-inventory.json") as f:
        inventory_json = json.load(f)
    book_dirs = ci.discover_book_dirs(pcgen_root)
    scope = ci.classify_scope(book_dirs, inventory_json)
    pathfinder_root = os.path.join(pcgen_root, "pathfinder")

    inv_lookup = {
        (u["book"], u["source_file"], u["source_line"])
        for u in inventory_json["units"]
        if u.get("kind") == "class_feature"
    }

    census_rows = []
    internal_rows = 0

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
                            continue  # directive line
                        iu = identity.upper()
                        if iu.endswith(".FORGET") or iu.endswith(".MOD"):
                            continue
                        fields = line.split("\t")
                        is_internal = any(
                            f.strip() == "CATEGORY:Internal" for f in fields
                        ) or identity.startswith("CATEGORY=Internal|")
                        if is_internal:
                            internal_rows += 1
                            continue
                        key_field = None
                        for f in fields:
                            fs = f.strip()
                            if fs.upper().startswith("KEY:"):
                                key_field = fs.split(":", 1)[1].strip()
                                break
                        if key_field is None:
                            if identity.startswith("CATEGORY=") and "|" in identity:
                                key_field = identity.split("|", 1)[1]
                            else:
                                key_field = identity
                        census_rows.append((bd.book_id, fn, lineno, identity, key_field, line))

    matched = [r for r in census_rows if (r[0], r[1], r[2]) in inv_lookup]
    residual = [r for r in census_rows if (r[0], r[1], r[2]) not in inv_lookup]

    print("total non-internal class_feature rows:", len(census_rows))
    print("  minus CATEGORY:Internal:", internal_rows)
    print("matched (already in inventory):", len(matched))
    print("residual (not in inventory):", len(residual))

    key_count_by_book = defaultdict(Counter)
    for r in census_rows:
        key_count_by_book[r[0]][r[4]] += 1

    dup_key_residuals = []
    for r in residual:
        book, fn, lineno, identity, key_field, line = r
        if key_count_by_book[book][key_field] > 1:
            dup_key_residuals.append(r)

    print(
        "residual rows whose (book, key) collides with >=1 other class_feature "
        "row in the same book (the duplicate_identity trap's own predicate):",
        len(dup_key_residuals),
        "/",
        len(residual),
    )
    print("unexplained residual (no key collision found -- cause still unpinned):",
          len(residual) - len(dup_key_residuals))

    for r in dup_key_residuals[:10]:
        print("  example:", r[0], r[1], r[2], repr(r[3]), "key=", repr(r[4]))


if __name__ == "__main__":
    main()
