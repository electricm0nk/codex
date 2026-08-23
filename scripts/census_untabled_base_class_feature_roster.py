#!/usr/bin/env python3
"""Extract, generically, the corpus's own automatic class-feature grant
rows for the `untabled_base_class_chassis` registry's classes -- the
mechanism SD-32 card 11's `epic-2-t12-modelled-class-books` cycle receipt
named as the real remaining lever ("a `push_pu_class_feature_records`-shaped
generic roster mechanism ... one fixture schema, one push function, reused
across every class").

# Method, mechanical, not curated

Two generic PCGen shapes grant a base class's own-named class features, both
extracted here with no per-class branching:

* **Shape 1 (`.MOD` virtual ability).** A `CATEGORY=Class|<ClassName>.MOD`
  line: one `ABILITY:<Category>|AUTOMATIC|<ClassName> ~ <Feature>|...` field
  per feature, gated by a `PREVARGTEQ:<Var>_CFP_Level,<N>` clause naming the
  minimum class level.
* **Shape 2 (`CLASS:` level-table row).** A row of the class's own `CLASS:
  <ClassName>` level table -- the row's first tab-separated field is the
  level number itself (PCGen's own table convention) -- carrying an
  `ABILITY:<ClassName> Class Feature|AUTOMATIC|<ClassName> ~ <Feature>` field.
  No `.MOD` marker is present; the level comes from the row's own leading
  column, not a `PREVARGTEQ` clause. Found by re-deriving the T12 attribution
  gap (`decisions.md` card 11): the 17 chassis-registered classes shape 1
  found no data for turned out to use this shape instead, confirmed for 10
  of them (`aegis`, `cryptic`, `dread`, `marksman`, `psychic_warrior`,
  `shifter`, `soulknife`, `tactician`, `vitalist`, `wilder`) -- see the
  `--summary` output for the remaining honest gap.

Both shapes are matched by the literal substring
`ABILITY:<ClassName> Class Feature|AUTOMATIC|<ClassName> ~ ` (or, for shape 1,
by the co-occurring `CATEGORY=Class|<ClassName>.MOD` marker) -- no
class-specific parsing, one mechanical pass per shape, reused across every
class either shape covers.

Restricted (deliberately) to records whose own key's group segment is the
class's OWN display name (`"<ClassName> ~ <Feature>"`) -- the same
"own-named group" population `docs/release/.../decisions.md §13`'s T12 shape
partitions from the pool-shaped groups (`Vigilante Talent`, `Magus Arcana`,
...). Pool-shaped groups need per-pool verification
(`class_feature_pool_catalog.rs`'s own construction discipline) and are
explicitly out of this script's scope -- extending it to guess at a pool's
real owner would be exactly the "guess to make the number smaller" this
program's `decisions.md §1a`/§3 rule out.

# Coverage, honestly reported, not assumed universal

Not every registry class uses this `.MOD` shape -- some classes (this run
found Cryptic, Psion, Kineticist, ... at zero matches) grant class features
through a different progression shape this script does not parse. Its own
`--summary` output below names exactly which classes it found data for and
which it did not, so nothing here is silently assumed complete.

Run: `PCGEN_CORPUS_ROOT=<oracle>/data python3 scripts/census_untabled_base_class_feature_roster.py`
(or let the script resolve `$PCGEN_REPO_DIR`/`$PCGEN_CORPUS_ROOT` itself, matching every other
corpus command in this bundle). Writes
`tests/fixtures/rules_core/untabled-base-class-feature-roster.json`.
"""
from __future__ import annotations

import glob
import json
import os
import re
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

REGISTRY_FIXTURE = os.path.join(
    REPO_ROOT, "tests", "fixtures", "rules_core", "untabled-base-class-chassis.json"
)
OUT_FIXTURE = os.path.join(
    REPO_ROOT, "tests", "fixtures", "rules_core", "untabled-base-class-feature-roster.json"
)

MOD_RE = re.compile(r"CATEGORY=Class\|([A-Za-z ]+)\.MOD\b")
LEVEL_RE = re.compile(r"PREVARGTEQ:\w*_?CFP_?Level,(\d+)")


def resolve_oracle_root() -> str:
    root = os.environ.get("PCGEN_CORPUS_ROOT")
    if root:
        return root
    repo_dir = os.environ.get("PCGEN_REPO_DIR")
    if repo_dir:
        return os.path.join(repo_dir, "data")
    raise SystemExit(
        "PCGEN_CORPUS_ROOT or PCGEN_REPO_DIR must be set -- see scripts/fetch-pcgen-oracle.sh"
    )


def load_registry_classes() -> dict[str, dict]:
    with open(REGISTRY_FIXTURE, encoding="utf-8") as f:
        data = json.load(f)
    out = {}
    for entry in data["entries"]:
        class_id = entry["class_id"].removeprefix("class:")
        out[entry["display_name"]] = {
            "class_id": class_id,
            "display_name": entry["display_name"],
            "source_book": entry["source_book"],
        }
    return out


def load_corpus_descriptions(repo_root: str) -> dict[str, dict]:
    """Map corpus `KEY:` -> {name, description, wiring_class} from the
    already-ingested `data/corpus/*/class_feature/**/*.json` records, so the
    fixture carries the same name/description text the corpus already holds
    (not re-derived a second time)."""
    out = {}
    for path in glob.glob(os.path.join(repo_root, "data", "corpus", "*", "class_feature", "**", "*.json"), recursive=True):
        try:
            with open(path, encoding="utf-8") as f:
                rec = json.load(f)
        except (json.JSONDecodeError, OSError):
            continue
        data = rec.get("data")
        if not isinstance(data, dict):
            continue
        key = data.get("key")
        if not key:
            continue
        out[key] = {
            "name": data.get("name") or key,
            "description": data.get("description") or "",
            "wiring_class": rec.get("wiring_class") or "",
        }
    return out


def main() -> None:
    oracle_root = resolve_oracle_root()
    registry = load_registry_classes()
    descriptions = load_corpus_descriptions(REPO_ROOT)

    found: dict[str, list[dict]] = {}
    zero_matches: list[str] = []

    lst_files = glob.glob(os.path.join(oracle_root, "**", "*.lst"), recursive=True)

    for display_name, meta in registry.items():
        class_id = meta["class_id"]
        rows: list[dict] = []
        seen_keys: set[str] = set()
        shape2_marker = f"ABILITY:{display_name} Class Feature|AUTOMATIC|{display_name} ~ "
        for path in lst_files:
            with open(path, encoding="utf-8", errors="replace") as f:
                lines = f.readlines()
            for lineno, line in enumerate(lines, 1):
                is_shape1 = f"CATEGORY=Class|{display_name}.MOD" in line
                is_shape2 = shape2_marker in line
                if not is_shape1 and not is_shape2:
                    continue
                fields = line.split("\t")
                # Shape 2's level comes from the row's own leading tab field
                # (PCGen's `CLASS:` level-table convention), not a
                # PREVARGTEQ clause -- only meaningful when that shape is in
                # play and the field actually parses as an integer.
                shape2_level = None
                if is_shape2:
                    first = fields[0].strip()
                    if first.isdigit():
                        shape2_level = int(first)
                for field in fields:
                    if not field.startswith("ABILITY:"):
                        continue
                    parts = field.split("|")
                    if len(parts) < 3:
                        continue
                    key = parts[2]
                    if not key.startswith(f"{display_name} ~ "):
                        continue  # own-named group only; pool grants excluded
                    if key in seen_keys:
                        continue
                    min_level = None
                    shape = None
                    if is_shape1:
                        level_match = LEVEL_RE.search(field)
                        if level_match:
                            min_level = int(level_match.group(1))
                            shape = "mod_ability"
                    if min_level is None and is_shape2 and shape2_level is not None:
                        min_level = shape2_level
                        shape = "class_level_table"
                    if min_level is None:
                        continue  # no reliably-parsed level -- skip, don't guess
                    seen_keys.add(key)
                    desc = descriptions.get(key, {})
                    rel_path = os.path.relpath(path, oracle_root)
                    rows.append(
                        {
                            "class_id": class_id,
                            "key": key,
                            "name": desc.get("name", key.split(" ~ ", 1)[-1]),
                            "description": desc.get("description", ""),
                            "text_only": desc.get("wiring_class", "") == "display",
                            "min_level": min_level,
                            "source_file": rel_path,
                            "source_line": lineno,
                            "source_shape": shape,
                        }
                    )
        if rows:
            rows.sort(key=lambda r: (r["min_level"], r["key"]))
            found[class_id] = rows
        else:
            zero_matches.append(class_id)

    total = sum(len(rows) for rows in found.values())
    out = {
        "entries": [row for rows in found.values() for row in rows],
    }
    os.makedirs(os.path.dirname(OUT_FIXTURE), exist_ok=True)
    with open(OUT_FIXTURE, "w", encoding="utf-8") as f:
        json.dump(out, f, indent=2, sort_keys=True)
        f.write("\n")

    print(f"total records: {total}")
    print(f"classes with data: {sorted(found.keys())}")
    print(f"classes with NO `.MOD`-shaped own-named grant found (not covered by this mechanism): {sorted(zero_matches)}")
    for class_id, rows in sorted(found.items()):
        print(f"  {class_id}: {len(rows)}")


if __name__ == "__main__":
    main()
