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
* **Shape 3 (bare own-named `CLASS:` row, no group-separator prefix).**
  Structurally identical to shape 2 -- same category prefix, same
  leading-column level -- but the field's payload target does not repeat
  "`<ClassName> ~ `" before the feature name; it is the bare feature name
  itself (`psion`'s own `CLASS:Psion` block grants `ABILITY:Psion Class
  Feature|AUTOMATIC|Psion Manifesting`, not `...|AUTOMATIC|Psion ~ Psion
  Manifesting`). Found re-deriving SD-32 card 11 (T12)'s `psion`
  remainder: confirmed by a direct oracle check (`grep -c "Psion ~ "
  up_classes.lst up_abilities_class.lst` -> `0` and `7`, every one of the
  7 a false-positive substring match inside a DIFFERENT class's own group
  name, e.g. `Ascendant Psion ~ Hide Mind`) that this is genuinely a third
  convention, not a repeat of the `CATEGORY=Class` vs `CATEGORY=CLASS`
  case-sensitivity bug that closed 6 of the earlier "7 classes need a
  third shape" false lead. One mechanical rule change captures it with no
  per-class branching: a target is this class's own-named group if it
  EITHER starts with "`<ClassName> ~ `" (shapes 1/2's explicit prefix) OR
  contains no "` ~ `" group separator at all (shape 3's implicit, bare
  own-name). A target containing "` ~ `" that does NOT start with the
  class's own name belongs to a DIFFERENT group (another class's own-named
  group, or a discipline/archetype pool member reached only through a
  chosen-pick chain) and stays excluded -- this widening adds coverage, it
  does not loosen the existing own-named-group boundary.

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
        # Shape 2's marker used to require the payload to ALSO repeat
        # "<ClassName> ~ " before the feature name. Widened to the bare
        # category+AUTOMATIC prefix so shape 3's own-named-but-unprefixed
        # targets (`psion`'s "Psion Manifesting") are found by the same
        # scan -- the group-membership test below (not this marker) is what
        # still keeps pool/other-group targets out.
        shape2_marker = f"ABILITY:{display_name} Class Feature|AUTOMATIC|"
        for path in lst_files:
            with open(path, encoding="utf-8", errors="replace") as f:
                lines = f.readlines()
            for lineno, raw_line in enumerate(lines, 1):
                # Strip the trailing line ending before any field split --
                # a target that is the LAST tab-separated field on its line
                # (true of every shape 3 hit found so far, e.g. `psion`'s
                # own `... AUTOMATIC|Psion Manifesting`) otherwise carries a
                # literal trailing `\n` into its own `KEY:`, corrupting the
                # fixture's `key`/`name` fields for exactly the population
                # this shape exists to capture correctly.
                line = raw_line.rstrip("\n").rstrip("\r")
                # `CATEGORY=` casing varies in the corpus itself (`Class` and
                # `CLASS` both occur -- confirmed for Kineticist/Medium/
                # Mesmerist/Occultist/Psychic/Spiritualist, all `occult_
                # adventures`, all `CATEGORY=CLASS|<Name>.MOD` uppercase).
                # Match case-insensitively so this one mechanical pass covers
                # both casings with no per-class branching.
                is_shape1 = f"category=class|{display_name.lower()}.mod" in line.lower()
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
                    is_own_named_explicit = key.startswith(f"{display_name} ~ ")
                    is_own_named_bare = " ~ " not in key
                    if not is_own_named_explicit and not is_own_named_bare:
                        continue  # a different group entirely; pool grants excluded
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
                        shape = "class_level_table" if is_own_named_explicit else "class_level_table_bare"
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
