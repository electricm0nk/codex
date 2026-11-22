#!/usr/bin/env python3
"""Extract real PCGen `CLASS:` chassis data (hit die, BAB progression,
good/poor save classification, level ceiling) for every **real base class**
whose source book this repo has actually ingested, but that has no dispatch
arm anywhere in `compute_class_chassis` today.

SD-32 Epic 3 (`epic-3-class-reachability`, `acceptance-and-verification.md`
AT-32-E3-001), second half: "The 18 real base classes without tables ...
feed this epic from Epic 4." (`epic-breakdown.md`'s own quoted figure).
This script is the re-derive command for that population, and its output
(`tests/fixtures/rules_core/untabled-base-class-chassis.json`) is the
fixture `src/rules_core/pilot_compute/untabled_base_class_chassis.rs` loads
at compile time via `include_str!`.

**This script's own run found 20, not 18** -- see the cycle receipt this
script's own commit ships with for the re-derive command and the retro
correction logged against the 18 figure.

Method, mechanical rather than curated:

1. Walk every `.lst` file under `$PCGEN_CORPUS_ROOT` (the pinned oracle).
2. A `CLASS:<Name>` line whose `TYPE:` field contains `Base` and `PC` but
   NOT `NPC` and NOT `Prestige` names a real player base class. Names
   starting with `Ex-` are excluded (operator ruling B5, still open --
   `decisions.md §7`).
3. Keep only classes whose source file sits under a book directory this
   repo has actually ingested (`data/corpus/<book>/` exists).
4. Drop any class whose slug (`class:<slug>`, same convention as every
   other class-id family in this codebase -- lowercase, non-alphanumerics
   collapsed to `_`) is already recognized by ANY existing dispatch arm in
   `compute_class_chassis` (`src/rules_core/pilot_compute/mod.rs`): the CRB
   `table_class_id` eleven, `ApgClassId::ALL`, `AcgClassId::ALL`,
   `PuClassId::ALL`, `UcClassId::ALL`. That allowlist is transcribed once,
   below, from those five enums' own source -- not re-derived by parsing
   Rust, because there is no single machine-readable list; it is
   cross-checked by `tests/` asserting no fixture entry's `class_id`
   collides with any of those five families' `from_class_id_str`.
5. For each surviving class, extract:
   - `hit_die` from its `HD:<n>` field.
   - `max_level` from its `MAXLEVEL:<n>` field.
   - `bab_progression` from its `BONUS:COMBAT|BASEAB|<formula>|...` field:
     `*3/4` -> ThreeQuarter, `/2` or `*1/2` -> Half, no multiplier -> Full.
     Where a class carries more than one BASEAB line gated by a
     `PREVAREQ:<Flag>,0` / `PREVAREQ:<Flag>,1` pair (Vigilante's
     `VigilanteFullBAB` talent toggle), the `,0` (default-off) line is kept
     -- the character has not taken the alternate talent unless they choose
     to, so the default progression is the honest baseline.
   - `good_saves` (fortitude, reflex, will) from `BONUS:SAVE|BASE.<X>|...`
     fields: a formula containing `/2+2` is a good save, `/3` is a poor
     save. PCGen allows one `BONUS:SAVE` line to cover more than one save
     column (comma-joined, e.g. `BASE.Fortitude,BASE.Will`) -- the
     classification applies to every column named on that line.

Run: `PCGEN_CORPUS_ROOT=<oracle>/data python3 scripts/census_untabled_base_classes.py`
(or let the script resolve `$PCGEN_CORPUS_ROOT`/`$PCGEN_REPO_DIR` itself,
matching every other corpus command in this bundle).
"""

from __future__ import annotations

import json
import os
import re
import sys
from pathlib import Path

# Transcribed from the five existing dispatch families' own source, per
# method step 4 above. Any class whose slug appears here is already
# reachable and is deliberately excluded from this script's output.
ALREADY_DISPATCHED_SLUGS = {
    # CRB (`table_class_id` + the Fighter/Wizard bespoke arms)
    "fighter", "wizard", "rogue", "ranger", "paladin", "sorcerer", "cleric",
    "druid", "barbarian", "bard", "monk",
    # APG (`ApgClassId::ALL`)
    "alchemist", "cavalier", "inquisitor", "oracle", "summoner", "witch",
    # ACG (`AcgClassId::ALL`)
    "arcanist", "bloodrager", "brawler", "hunter", "investigator", "shaman",
    "skald", "slayer", "swashbuckler", "warpriest",
    # UC (`UcClassId::ALL`)
    "gunslinger", "ninja", "samurai",
    # PU (`PuClassId::ALL`) -- these are replacement variants of CRB
    # classes, not distinct base classes, and never appear as a bare
    # `TYPE:Base.PC` `CLASS:` name under this method anyway (their corpus
    # names are e.g. "Barbarian (Unchained)"), but listed for completeness.
    "unchained_barbarian", "unchained_monk", "unchained_rogue",
    "unchained_summoner",
}


def resolve_corpus_root() -> Path:
    env = os.environ.get("PCGEN_CORPUS_ROOT")
    if env:
        return Path(env)
    repo_dir = os.environ.get("PCGEN_REPO_DIR")
    if repo_dir:
        return Path(repo_dir) / "data"
    raise SystemExit(
        "PCGEN_CORPUS_ROOT (or PCGEN_REPO_DIR) is not set -- export it per "
        "workflow-instruction.md §2.1 before running this script."
    )


def ingested_books(repo_root: Path) -> set[str]:
    corpus_dir = repo_root / "data" / "corpus"
    return {p.name for p in corpus_dir.iterdir() if p.is_dir()}


def slugify(name: str) -> str:
    slug = re.sub(r"[^a-z0-9]+", "_", name.lower())
    return slug.strip("_")


def classify_bab(formula: str) -> str | None:
    if "*3/4" in formula:
        return "ThreeQuarter"
    if "/2" in formula or "*1/2" in formula:
        return "Half"
    return "Full"


def extract(corpus_root: Path, repo_root: Path) -> dict[str, dict]:
    books = ingested_books(repo_root)
    # A class name (e.g. "Psion", "Soulknife") can appear in MORE than one
    # PCGen source book -- Dreamscarred Press republished several psionics
    # classes across `psionics_unleashed` (not ingested here) and
    # `ultimate_psionics` (ingested). Collect every candidate path per name,
    # not just the first one `os.walk` happens to visit, so an
    # ingested-book occurrence is never shadowed by an uningested one.
    base_name_paths: dict[str, list[Path]] = {}
    class_lines: dict[tuple[Path, str], list[str]] = {}

    for dirpath, _dirs, files in os.walk(corpus_root):
        for fn in files:
            if not fn.endswith(".lst"):
                continue
            path = Path(dirpath) / fn
            try:
                text = path.read_text(encoding="utf-8", errors="replace")
            except OSError:
                continue
            for line in text.splitlines():
                if not line.startswith("CLASS:"):
                    continue
                fields = line.split("\t")
                name = fields[0][len("CLASS:"):]
                class_lines.setdefault((path, name), []).append(line)
                type_field = next((f for f in fields[1:] if f.startswith("TYPE:")), None)
                if type_field is None:
                    continue
                type_parts = set(type_field[len("TYPE:"):].split("."))
                if (
                    "Base" in type_parts
                    and "PC" in type_parts
                    and "NPC" not in type_parts
                    and "Prestige" not in type_parts
                    and not name.startswith("Ex-")
                ):
                    base_name_paths.setdefault(name, []).append(path)

    base_names: dict[str, Path] = {}
    for name, paths in base_name_paths.items():
        ingested_path = next(
            (p for p in paths if next((part for part in p.relative_to(corpus_root).parts if part in books), None) is not None),
            None,
        )
        if ingested_path is not None:
            base_names[name] = ingested_path

    result: dict[str, dict] = {}
    for name, path in sorted(base_names.items()):
        slug = slugify(name)
        if slug in ALREADY_DISPATCHED_SLUGS:
            continue
        try:
            rel = path.relative_to(corpus_root)
        except ValueError:
            rel = path
        parts = rel.parts
        matched_book = next((p for p in parts if p in books), None)
        if matched_book is None:
            continue

        hit_die = None
        max_level = None
        bab_default = None
        bab_alt_flags: dict[str, str] = {}
        good = {"fortitude": None, "reflex": None, "will": None}

        for line in class_lines.get((path, name), []):
            for field in line.split("\t")[1:]:
                if field.startswith("HD:") and hit_die is None:
                    hit_die = int(field[len("HD:"):])
                elif field.startswith("MAXLEVEL:") and max_level is None:
                    max_level = int(field[len("MAXLEVEL:"):])
                elif field.startswith("BONUS:COMBAT|BASEAB|"):
                    parts2 = field.split("|")
                    formula = parts2[2] if len(parts2) > 2 else ""
                    # Detect a conditional-flag toggle line
                    # (PREVAREQ:<Flag>,0 / PREVAREQ:<Flag>,1) -- keep the
                    # `,0` (default-off) line as the baseline progression.
                    m = re.search(r"PREVAREQ:(\w+),([01])$", field)
                    if m:
                        flag_val = m.group(2)
                        if flag_val == "0":
                            bab_default = classify_bab(formula)
                    elif bab_default is None:
                        bab_default = classify_bab(formula)

        for line in class_lines.get((path, name), []):
            for field in line.split("\t")[1:]:
                if not field.startswith("BONUS:SAVE|BASE."):
                    continue
                parts2 = field.split("|")
                if len(parts2) < 3:
                    continue
                columns = parts2[1]
                formula = parts2[2]
                is_good = "/2+2" in formula
                is_poor = "/3" in formula and "/2+2" not in formula
                for col in columns.split(","):
                    key = col.strip().removeprefix("BASE.").lower()
                    if key in good and good[key] is None:
                        if is_good:
                            good[key] = True
                        elif is_poor:
                            good[key] = False

        if hit_die is None or max_level is None or bab_default is None:
            continue
        if any(v is None for v in good.values()):
            continue

        result[name] = {
            "class_id": f"class:{slug}",
            "display_name": name,
            "source_book": matched_book,
            "source_file": str(rel),
            "hit_die": hit_die,
            "max_level": max_level,
            "bab_progression": bab_default,
            "good_saves": good,
        }
    return result


def main() -> int:
    repo_root = Path(__file__).resolve().parent.parent
    corpus_root = resolve_corpus_root()
    result = extract(corpus_root, repo_root)

    out_path = repo_root / "tests" / "fixtures" / "rules_core" / "untabled-base-class-chassis.json"
    out_path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "_comment": (
            "Generated by scripts/census_untabled_base_classes.py. "
            "Do not hand-edit -- re-run the script against the pinned oracle instead."
        ),
        "entries": [result[name] for name in sorted(result)],
    }
    out_path.write_text(json.dumps(payload, indent=2, sort_keys=False) + "\n", encoding="utf-8")

    print(f"population={len(result)}", file=sys.stderr)
    print(f"wrote {out_path}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
