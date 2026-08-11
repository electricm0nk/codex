#!/usr/bin/env python3
"""SD-27 E2.0.7 — advanced_players_guide license retro-fit (heuristic pass).

Re-emits every Shape B record under data/corpus/advanced_players_guide/ with
the v1 license fields (`license`, `pi_field`, `pi_marker`) per
docs/governance/ogl-pi-blacklist.md and src/rules_core/shape_b_v1.rs.

This is a bounded, heuristic first-pass screening against a documented,
short list of known Golarion/Pathfinder Product-Identity proper nouns (the
20 canonical core deities plus a sampled set of known setting place names)
run as a plain substring scan over each record's `description` field. It is
NOT an exhaustive human legal review — it will not catch PI that isn't a
proper noun on this list (e.g. a paraphrased reference to a named person or
place, or a proper noun this list doesn't happen to include).

Classification rules (docs/governance/ogl-pi-blacklist.md §2.2/§2.3):
  - class/*.json records: only mechanical fields (class_id, maxlevel,
    chassis/*) -- §2.2 blanket-OGL, no free text field at all.
    -> license: "OGL", pi_field: null, pi_marker: null
  - spell/*.json, equipment/*.json records: have a `description` field
    (§2.3, per-value judgment required).
    -> scan description for a blacklist-term hit (substring, case-sensitive
       on the proper noun's canonical capitalization)
    -> hit:    license "PI-REDACTED", pi_field "description",
               pi_marker "redacted", description replaced with the literal
               marker string "[redacted PI]"
    -> no hit (including description: null): license "OGL", pi_field: null,
               pi_marker: null
"""
import json
import glob
import sys

REDACTED_MARKER = "[redacted PI]"

# The 20 canonical core deities named in the task brief.
DEITIES = [
    "Iomedae", "Sarenrae", "Asmodeus", "Cayden Cailean", "Abadar",
    "Calistria", "Desna", "Erastil", "Gorum", "Gozreh", "Irori",
    "Lamashtu", "Nethys", "Norgorber", "Pharasma", "Rovagug", "Shelyn",
    "Torag", "Urgathoa", "Zon-Kuthon",
]

# Additional clearly-named Golarion proper nouns sampled for while
# reviewing this book's real records (setting place names/regions).
# None of these were found to actually appear in APG's spell/equipment
# description text during this cycle's authoring -- included as part of
# the documented, bounded scan list per the task brief, not because a hit
# was observed.
EXTRA_PROPER_NOUNS = [
    "Golarion", "Absalom", "Cheliax", "Varisia", "Andoran", "Taldor",
    "Osirion", "Katapesh", "Ustalav", "Numeria", "Mwangi", "Tian Xia",
    "Avistan", "Garund", "Sarkoris", "Worldwound", "Vudra", "Kyonin",
    "Molthune", "Nidal", "Nirmathas", "Qadira", "Razmiran", "Rahadoum",
    "Galt", "Isger", "Lastwall", "Brevoy", "Druma", "Irrisen",
    "Jalmeray", "Thuvia", "Geb", "Nex",
]

BLACKLIST_TERMS = DEITIES + EXTRA_PROPER_NOUNS

BOOK_DIR = "data/corpus/advanced_players_guide"


def classify_description(desc):
    """Returns (license, pi_field, pi_marker, new_desc)."""
    if desc is None:
        return "OGL", None, None, desc
    for term in BLACKLIST_TERMS:
        if term in desc:
            return "PI-REDACTED", "description", "redacted", REDACTED_MARKER
    return "OGL", None, None, desc


def process_file(path):
    with open(path, "r", encoding="utf-8") as f:
        record = json.load(f)

    data = record["data"]
    if "description" in data:
        license_val, pi_field, pi_marker, new_desc = classify_description(data.get("description"))
        data["description"] = new_desc
    else:
        # No free-text field at all (e.g. class chassis records) -- §2.2
        # blanket-OGL, mechanical fields only.
        license_val, pi_field, pi_marker = "OGL", None, None

    record["license"] = license_val
    record["pi_field"] = pi_field
    record["pi_marker"] = pi_marker

    with open(path, "w", encoding="utf-8") as f:
        json.dump(record, f, indent=2, ensure_ascii=False)
        f.write("\n")

    return license_val


def main():
    files = sorted(glob.glob(f"{BOOK_DIR}/**/*.json", recursive=True))
    counts = {"OGL": 0, "PI-REDACTED": 0, "PI": 0}
    for path in files:
        license_val = process_file(path)
        counts[license_val] = counts.get(license_val, 0) + 1

    total = len(files)
    redacted = counts.get("PI-REDACTED", 0)
    print(f"Processed {total} records under {BOOK_DIR}")
    print(f"  OGL:          {counts.get('OGL', 0)}")
    print(f"  PI-REDACTED:  {redacted}")
    print(f"  PI:           {counts.get('PI', 0)}")
    return total, redacted


if __name__ == "__main__":
    main()
