#!/usr/bin/env python3
"""Demote `CASTER_LEVEL_RULES`' verbatim `token:` / `resolution:` fields into
`//` provenance comments -- SD-35 Epic 6, AT-35-E6-003-SWEEP cycle 6.

`CasterLevelRule` carried two `&'static str` fields holding the class's
`BONUS:CASTERLEVEL` token and the `BONUS:VAR` chain it names, verbatim, and
`ground_caster_level_records` interpolated BOTH of them into the
`ComputationExplanation.detail` it pushes. That `detail` is carried to the
desktop crate as `ExplanationDto.detail` and printed on the Character Hub
sheet, so every casting character's sheet was printing, in full:

    Wizard caster level at Wizard level 10: 10, transcribed from the corpus's
    own `BONUS:CASTERLEVEL|Wizard|Caster_Level_BL_Stripped_Wizard`
    (core_rulebook/cr_classes.lst:281). That token's variable resolves through
    cr_classes.lst:277 BONUS:VAR|Caster_Level_BL_Stripped_Wizard|...

That is the same sheet-rule defect cycles 4 and 5 cleared in
`ComputationExplanation.detail` elsewhere and in the shipped tables
(`decisions.md` §1: a sheet line is one final number, dice in final form, or
the rule's words -- never ingest vocabulary). It was additionally PINNED by a
test that REQUIRED the token in the rendered text
(`tests/v06_caster_level_every_casting_class.rs`), which is why four earlier
cycles walked past it.

The transform, per row
----------------------
  * the `token:` and `resolution:` field lines are deleted from the struct
    literal and re-emitted VERBATIM, byte for byte, as `//` provenance lines
    immediately above the row -- provenance changes custody, it is not lost,
    which is what `AGENTS.md` rule 9 asks for and what operator ruling B14
    (`decisions.md` §17) says is the right home for it;
  * a `names_class_level_directly: bool` replaces the `resolution.is_empty()`
    test the renderer used to make -- the four ACG classes whose rule names
    the class level with no intermediate variable;
  * the `token_source` field is KEPT. `core_rulebook/cr_classes.lst:281` is a
    source reference a reader can chase, not ingest syntax, and the residue
    gate does not count it. The sheet keeps citing where the rule came from.

The renderer's prose is rewritten by hand in the same commit; this tool owns
only the table, which is the mechanical half.

Usage:
    python3 <this> --check     # census only, writes nothing
    python3 <this> --apply
"""

import argparse
import re
import sys

LIVE = "src/rules_core/pilot_compute/mod.rs"
START = "const CASTER_LEVEL_RULES: &[CasterLevelRule] = &[\n"


def join_continuations(lines, i):
    """(value, next_index) for a `field: "..."` line and its `\\`-continuations."""
    parts = [lines[i]]
    while parts[-1].rstrip().endswith("\\"):
        i += 1
        parts.append(lines[i])
    joined = parts[0].rstrip()
    for p in parts[1:]:
        joined = joined[:-1] + p.strip()
    m = re.match(r'^\s*\w+: "((?:[^"\\]|\\.)*)",\s*$', joined)
    if m is None:
        raise SystemExit(f"unparsed field line: {joined!r}")
    return m.group(1), i + 1


def transform(text):
    head, sep, rest = text.partition(START)
    if not sep:
        raise SystemExit("CASTER_LEVEL_RULES not found -- already transformed?")
    end = rest.index("\n];\n")
    body, tail = rest[:end + 1], rest[end + 1:]

    lines = body.split("\n")
    out, i, rows = [], 0, 0
    while i < len(lines):
        line = lines[i]
        if line.strip() != "CasterLevelRule {":
            out.append(line)
            i += 1
            continue
        # Collect the row's fields, holding `token`/`resolution` back.
        fields, token, resolution = [], None, None
        j = i + 1
        while lines[j].strip() != "},":
            stripped = lines[j].strip()
            if stripped.startswith("token: "):
                token, j = join_continuations(lines, j)
                continue
            if stripped.startswith("resolution: "):
                resolution, j = join_continuations(lines, j)
                continue
            fields.append(lines[j])
            j += 1
        if token is None or resolution is None:
            raise SystemExit(f"row at line {i} is missing token/resolution")
        indent = line[: len(line) - len(line.lstrip())]
        out.append(f"{indent}// Provenance (ingest tokens, demoted out of the rendered sheet line")
        out.append(f"{indent}// -- SD-35 AT-35-E6-003-SWEEP cycle 6). Verbatim, byte for byte:")
        out.append(f"{indent}//   {token}")
        if resolution:
            out.append(f"{indent}//   {resolution}")
        else:
            out.append(f"{indent}//   (the token names the class level `CL` directly -- no chain)")
        out.append(line)
        out.extend(fields)
        out.append(
            f"{indent}    names_class_level_directly: {'true' if not resolution else 'false'},"
        )
        out.append(lines[j])
        rows += 1
        i = j + 1
    return head + START + "\n".join(out) + tail, rows


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--apply", action="store_true")
    ap.add_argument("--check", action="store_true")
    args = ap.parse_args()
    text = open(LIVE, encoding="utf-8").read()
    if START not in text:
        print("already transformed (the table's own const is gone)")
        return 0
    if not re.search(r"^\s*token: \"", text, re.M):
        # Applied tree: the const is still there, its two token fields are not.
        # Report what the transform left, so the receipt's re-derive command
        # prints a figure rather than a spurious parse failure.
        rows = len(re.findall(r"^        names_class_level_directly: ", text, re.M))
        prov = len(re.findall(r"^    //   BONUS:CASTERLEVEL\|", text, re.M))
        print(f"already applied: rows={rows} provenance_token_lines={prov}")
        return 0 if rows == prov == 17 else 1
    new, rows = transform(text)
    print(f"rows={rows}")
    if args.apply:
        open(LIVE, "w", encoding="utf-8").write(new)
        print(f"applied to {LIVE}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
