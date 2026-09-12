#!/usr/bin/env python3
"""Type the ingest guard tail out of `CompanionRecord::external_ability_refs`.

SD-35 `AT-35-E6-003-SWEEP` cycle 13 (`decisions.md` §11 — no PCGen in live
code).

Before this cycle three CRB creature rows carried the ingest guard the corpus
appended to an ability grant as if it were a fourth *ability name* in
`external_ability_refs`:

    external_ability_refs: &["Animal Traits Output", "Scent",
                             "Gylptodon Companion Natural Attack",
                             "!PRETEMPLATE:1,Gylptodon Companion Advancement"],

`apps/desktop/src-tauri/src/companion_catalog.rs` serves that slice straight to
the player, so the token was on screen, not merely in a source file.

This script does two things, in two passes so the middle state is a real RED:

  --apply-field   add `external_ability_ref_conditions: &[]` to every
                  `CompanionRecord` literal in the tree (the new chassis field).
                  The guard strings are still in place after this pass; the
                  standing gate `sd35_rendered_prose_carries_no_ingest_vocabulary`
                  is RED here, which is the point.
  --apply-guards  strip the guard element from the three arrays and populate
                  `external_ability_ref_conditions` with cycle 7's typed
                  `EffectCondition` schema.

`--check` reports what each pass would do without writing.

The verbatim pre-conversion arrays are pinned on the converter side in
`src/pcgen_import/companion_pcgen_guards.rs`
(`COMPANION_EXTERNAL_ABILITY_REFS_BEFORE`) with a round-trip test that rebuilds
them from the live typed form. Nothing under a live root reads that table.
"""

from __future__ import annotations

import argparse
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parents[5]
TABLES = REPO / "src" / "rules_core" / "rules_tables"

NEW_FIELD = "external_ability_ref_conditions"

# The whole corpus population of guarded external ability refs, re-derived with
#   grep -rn external_ability_refs --include=*.rs src/ | grep -E '!?PRE[A-Z]+:'
# → 3 hits, all in crb/companion_data.rs. Each entry is
#   (companion key, guarded ability name, verbatim ingest tail).
GUARDS = [
    (
        "Companion (Hippopotamus)",
        "Hippopotamus Companion Natural Attack",
        "!PRETEMPLATE:1,Hippopotamus Companion Advancement",
    ),
    (
        "Companion (Megafauna (Arsinoitherium))",
        "Arsinoitherium Companion Natural Attack",
        "!PRETEMPLATE:1,Arsinoitherium Companion Advancement",
    ),
    (
        "Companion (Megafauna (Gylptodon))",
        "Gylptodon Companion Natural Attack",
        "!PRETEMPLATE:1,Gylptodon Companion Advancement",
    ),
]

OPENER = re.compile(r"^\s*([A-Za-z_][A-Za-z0-9_]*)\s*\{\s*$")
REFS_LINE = re.compile(r"^(\s*)external_ability_refs:\s*&\[(.*)\],\s*$")


def companion_data_files() -> list[pathlib.Path]:
    return sorted(TABLES.glob("*/companion_data.rs"))


def enclosing_literal(lines: list[str], idx: int) -> str | None:
    """The nearest `Ident {` opener above `idx`, by backwards scan.

    These tables are generated one field per line with each record opened by a
    bare `    CompanionRecord {`, so the nearest opener above a field line is
    that field's own struct. Checked, not assumed: the caller rejects any name
    other than `CompanionRecord`, so a shape this walk misreads produces a
    skipped record, never a wrong edit.
    """
    for j in range(idx - 1, -1, -1):
        m = OPENER.match(lines[j])
        if m:
            return m.group(1)
    return None


def split_refs(body: str) -> list[str]:
    """Split an `&[...]` body of string literals. No escapes occur in this data."""
    if not body.strip():
        return []
    out = re.findall(r'"((?:[^"\\]|\\.)*)"', body)
    if len(out) != body.count('"') // 2:
        raise SystemExit(f"unparsed external_ability_refs body: {body!r}")
    return out


def render_refs(refs: list[str]) -> str:
    return "&[" + ", ".join(f'"{r}"' for r in refs) + "]"


def condition_literal(tail: str) -> str:
    """Cycle 7's `EffectCondition` schema for one `!?PRE<FAMILY>:<arg>` tail."""
    negated = tail.startswith("!")
    rest = tail[1:] if negated else tail
    if not rest.startswith("PRE") or ":" not in rest:
        raise SystemExit(f"not a PRE token: {tail!r}")
    family, argument = rest[3:].split(":", 1)
    items = ", ".join(
        f'ConditionItem {{ facet: None, value: "{part}" }}' for part in argument.split(",")
    )
    return (
        f"EffectCondition {{ negated: {'true' if negated else 'false'}, "
        f'family: "{family}", items: &[{items}], alternatives: &[] }}'
    )


def pass_field(write: bool) -> int:
    """Add `external_ability_ref_conditions: &[]` to every CompanionRecord."""
    touched = 0
    for path in companion_data_files():
        lines = path.read_text().splitlines(keepends=True)
        out: list[str] = []
        added = 0
        for idx, line in enumerate(lines):
            out.append(line)
            m = REFS_LINE.match(line)
            if not m:
                continue
            if enclosing_literal(lines, idx) != "CompanionRecord":
                continue
            if idx + 1 < len(lines) and NEW_FIELD in lines[idx + 1]:
                continue
            out.append(f"{m.group(1)}{NEW_FIELD}: &[],\n")
            added += 1
        if added and write:
            path.write_text("".join(out))
        if added:
            print(f"{path.relative_to(REPO)}: {added} CompanionRecord literals")
        touched += added
    print(f"{NEW_FIELD} added to {touched} literals")
    return touched


def pass_guards(write: bool) -> int:
    """Strip the guard element and populate the typed field, in CRB."""
    path = TABLES / "crb" / "companion_data.rs"
    lines = path.read_text().splitlines(keepends=True)
    by_key = {key: (ability, tail) for key, ability, tail in GUARDS}
    done = 0
    current_key: str | None = None
    for idx, line in enumerate(lines):
        km = re.match(r'^\s*key:\s*"((?:[^"\\]|\\.)*)",\s*$', line)
        if km:
            current_key = km.group(1)
            continue
        m = REFS_LINE.match(line)
        if not m or current_key not in by_key:
            continue
        if enclosing_literal(lines, idx) != "CompanionRecord":
            continue
        ability, tail = by_key[current_key]
        refs = split_refs(m.group(2))
        if tail not in refs:
            continue
        if ability not in refs:
            raise SystemExit(f"{current_key}: guard cites {ability!r}, which the row does not list")
        refs = [r for r in refs if r != tail]
        indent = m.group(1)
        lines[idx] = f"{indent}external_ability_refs: {render_refs(refs)},\n"
        assert NEW_FIELD in lines[idx + 1], "run --apply-field first"
        lines[idx + 1] = (
            f"{indent}{NEW_FIELD}: &[ExternalAbilityRefCondition {{ "
            f'ability: "{ability}", conditions: &[{condition_literal(tail)}] }}],\n'
        )
        print(f"{current_key}: {tail}  ->  typed")
        done += 1
    if done and write:
        path.write_text("".join(lines))
    print(f"{done} of {len(GUARDS)} guards typed")
    return done


def emit_before_table() -> None:
    """The `COMPANION_EXTERNAL_ABILITY_REFS_BEFORE` rows, for hand-placement."""
    path = TABLES / "crb" / "companion_data.rs"
    lines = path.read_text().splitlines(keepends=True)
    by_key = {key: (ability, tail) for key, ability, tail in GUARDS}
    current_key = None
    for idx, line in enumerate(lines):
        km = re.match(r'^\s*key:\s*"((?:[^"\\]|\\.)*)",\s*$', line)
        if km:
            current_key = km.group(1)
            continue
        m = REFS_LINE.match(line)
        if not m or current_key not in by_key:
            continue
        refs = split_refs(m.group(2))
        ability, tail = by_key[current_key]
        if tail not in refs:
            refs = refs + [tail]  # already converted: rebuild the pre-state
        print(
            f'    ExternalAbilityRefsBefore {{ book: "core_rulebook", '
            f'companion_key: "{current_key}", refs: {render_refs(refs)} }},'
        )


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--apply-field", action="store_true")
    parser.add_argument("--apply-guards", action="store_true")
    parser.add_argument("--emit-before-table", action="store_true")
    args = parser.parse_args(argv)
    if args.emit_before_table:
        emit_before_table()
        return 0
    if args.apply_field:
        return 0 if pass_field(write=True) >= 0 else 1
    if args.apply_guards:
        return 0 if pass_guards(write=True) == len(GUARDS) else 1
    if args.check:
        pass_field(write=False)
        return 0
    parser.error("one of --check, --apply-field, --apply-guards, --emit-before-table")
    return 2


if __name__ == "__main__":
    sys.exit(main())
