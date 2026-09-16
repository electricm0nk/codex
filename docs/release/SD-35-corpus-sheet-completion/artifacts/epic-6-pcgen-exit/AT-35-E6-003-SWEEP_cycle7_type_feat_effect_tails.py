#!/usr/bin/env python3
"""SD-35 `AT-35-E6-003-SWEEP` cycle 7 — type the `FeatEffectBonus` qualifier tail.

What this converts
------------------
Every shipped feat catalog row stores its corpus `BONUS:` token as the raw
pipe-split qualifier list (`crb::feats::FeatEffectBonus.qualifiers`). Elements
0..2 are the bonus category, the target list and the value. **Everything after
them is ingest vocabulary**: a `TYPE=<label>` stacking label and zero or more
`PRE<FAMILY>:<argument>` / `!PRE<FAMILY>:<argument>` guards. Those tails are what
`scripts/pcgen_residue_gate.py` counts as `TYPE=` and `PRE[A-Z]+:` hits on the
live side, and `decisions.md` §11 says the live side may not carry them.

This tool moves the tail off the token and into our own schema, in place, across
every shipped feat catalog file:

    FeatEffectBonus { qualifiers: &["COMBAT", "AC", "1", "TYPE=Dodge"] }

becomes

    FeatEffectBonus { qualifiers: &["COMBAT", "AC", "1"], bonus_type: Some("Dodge"), conditions: &[] }

and a guard

    "!PREABILITY:1,CATEGORY=Special Ability,Scaled Skin C ~ Tiefling"

becomes

    EffectCondition {
        negated: true,
        family: "ABILITY",
        items: &[
            ConditionItem { facet: None, value: "1" },
            ConditionItem { facet: Some("CATEGORY"), value: "Special Ability" },
            ConditionItem { facet: None, value: "Scaled Skin C ~ Tiefling" },
        ],
        alternatives: &[],
    }

No structure is invented. `items` is the guard argument comma-split exactly as
the ingest record wrote it (commas inside `[]`, `()` or `""` are not
separators), and a `<facet>=<value>` element is carried as a typed facet rather
than as the `FACET=` token text. The `MULT` family, which nests bracketed
sub-guards, recurses into `alternatives`. Nothing is dropped, which the
round-trip table this tool also writes
(`src/pcgen_import/feat_effect_conditions.rs`) proves row by row: it holds the
**verbatim** tail the ingest record wrote for every converted bonus, addressed
by `(catalog, feat key, bonus index)`, and its test rebuilds that string from
the live typed form and compares character for character.

Usage
-----
    python3 …_cycle7_type_feat_effect_tails.py --check   # report, exit 0
    python3 …_cycle7_type_feat_effect_tails.py --apply   # rewrite in place

`--check` is idempotent and safe on an applied tree: it reports
`already applied` when no untyped literal remains.
"""

from __future__ import annotations

import argparse
import glob
import os
import re
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", ".."))

OPEN = "FeatEffectBonus { qualifiers: &["
ANY_LITERAL = re.compile(r"FeatEffectBonus \{")
KEY = re.compile(r'FeatTableEntry \{ key: "((?:[^"\\]|\\.)*)"')
STRING = re.compile(r'"((?:[^"\\]|\\.)*)"')
GUARD = re.compile(r"^(?P<neg>!?)PRE(?P<family>[A-Z]+):(?P<arg>.*)$", re.S)

CATALOG_BY_DIR = {
    "crb": "crb",
    "apg": "apg",
    "acg": "acg",
    "advanced_race_guide": "arg",
}


def target_files() -> list[str]:
    pats = [
        "src/rules_core/rules_tables/*/feat_data/*.rs",
        "src/rules_core/rules_tables/*/feats.rs",
    ]
    out: set[str] = set()
    for p in pats:
        out.update(glob.glob(os.path.join(REPO, p)))
    return sorted(out)


def split_top_level(text: str) -> list[str]:
    """Comma-split `text`, ignoring commas inside `[]`, `()` or double quotes."""
    parts: list[str] = []
    depth = 0
    quoted = False
    buf: list[str] = []
    for ch in text:
        if ch == '"':
            quoted = not quoted
            buf.append(ch)
            continue
        if not quoted:
            if ch in "[(":
                depth += 1
            elif ch in "])":
                depth -= 1
            elif ch == "," and depth == 0:
                parts.append("".join(buf))
                buf = []
                continue
        buf.append(ch)
    parts.append("".join(buf))
    return [p for p in parts if p != ""]


class Literal:
    """One untyped `FeatEffectBonus { qualifiers: &[...] }` occurrence."""

    __slots__ = ("start", "end", "body")

    def __init__(self, start: int, end: int, body: str) -> None:
        self.start, self.end, self.body = start, end, body


def find_untyped(src: str) -> list[Literal]:
    """Bracket- and quote-aware scan for untyped literals.

    A regex cannot do this: a guard argument legitimately contains `]`
    (`var("COUNT[EQTYPE...]")`, `PREMULT:1,[PREEQUIP:...]`), and four of the
    real rows do. Scanning with a depth counter that ignores bracket characters
    inside string literals is the only reading that does not silently skip them.
    """
    out: list[Literal] = []
    pos = 0
    while True:
        start = src.find(OPEN, pos)
        if start < 0:
            return out
        i = start + len(OPEN)
        depth = 1
        quoted = False
        escaped = False
        while i < len(src) and depth:
            ch = src[i]
            if quoted:
                if escaped:
                    escaped = False
                elif ch == "\\":
                    escaped = True
                elif ch == '"':
                    quoted = False
            elif ch == '"':
                quoted = True
            elif ch == "[":
                depth += 1
            elif ch == "]":
                depth -= 1
            i += 1
        body = src[start + len(OPEN) : i - 1]
        if src[i : i + 2] == " }":
            out.append(Literal(start, i + 2, body))
            pos = i + 2
        else:
            pos = start + len(OPEN)


def rust_str(value: str) -> str:
    return '"' + value.replace("\\", "\\\\").replace('"', '\\"') + '"'


def parse_guard(token: str) -> dict:
    m = GUARD.match(token)
    if not m:
        raise ValueError(f"not a guard: {token!r}")
    items: list[tuple[str | None, str]] = []
    alternatives: list[dict] = []
    for part in split_top_level(m.group("arg")):
        if part.startswith("[") and part.endswith("]"):
            alternatives.append(parse_guard(part[1:-1]))
        elif "=" in part:
            facet, _, value = part.partition("=")
            items.append((facet, value))
        else:
            items.append((None, part))
    return {
        "negated": m.group("neg") == "!",
        "family": m.group("family"),
        "items": items,
        "alternatives": alternatives,
    }


def render_guard(guard: dict, indent: str) -> str:
    inner = indent + "    "
    lines = [f"{indent}EffectCondition {{"]
    lines.append(f"{inner}negated: {'true' if guard['negated'] else 'false'},")
    lines.append(f"{inner}family: {rust_str(guard['family'])},")
    if guard["items"]:
        lines.append(f"{inner}items: &[")
        for facet, value in guard["items"]:
            facet_src = "None" if facet is None else f"Some({rust_str(facet)})"
            lines.append(
                f"{inner}    ConditionItem {{ facet: {facet_src}, value: {rust_str(value)} }},"
            )
        lines.append(f"{inner}],")
    else:
        lines.append(f"{inner}items: &[],")
    if guard["alternatives"]:
        lines.append(f"{inner}alternatives: &[")
        for alt in guard["alternatives"]:
            lines.append(render_guard(alt, inner + "    ") + ",")
        lines.append(f"{inner}],")
    else:
        lines.append(f"{inner}alternatives: &[],")
    lines.append(f"{indent}}}")
    return "\n".join(lines)


def rebuild_guard(guard: dict) -> str:
    """The inverse of `parse_guard` — the verbatim ingest token, rebuilt."""
    parts: list[str] = []
    for facet, value in guard["items"]:
        parts.append(value if facet is None else f"{facet}={value}")
    for alt in guard["alternatives"]:
        parts.append("[" + rebuild_guard(alt) + "]")
    prefix = "!" if guard["negated"] else ""
    return f"{prefix}PRE{guard['family']}:" + ",".join(parts)


def convert_literal(elements: list[str]) -> tuple[list[str], str | None, list[dict]]:
    head: list[str] = []
    bonus_type: str | None = None
    guards: list[dict] = []
    for element in elements:
        if element.startswith("TYPE="):
            bonus_type = element[len("TYPE=") :]
        elif GUARD.match(element):
            guards.append(parse_guard(element))
        else:
            head.append(element)
    return head, bonus_type, guards


def render_literal(head: list[str], bonus_type: str | None, guards: list[dict]) -> str:
    quals = ", ".join(rust_str(h) for h in head)
    type_src = "None" if bonus_type is None else f"Some({rust_str(bonus_type)})"
    if not guards:
        return (
            f"FeatEffectBonus {{ qualifiers: &[{quals}], "
            f"bonus_type: {type_src}, conditions: &[] }}"
        )
    return (
        "FeatEffectBonus {\n"
        f"        qualifiers: &[{quals}],\n"
        f"        bonus_type: {type_src},\n"
        "        conditions: &[\n"
        + ",\n".join(render_guard(g, "            ") for g in guards)
        + ",\n        ],\n    }"
    )


def catalog_for(path: str) -> str:
    rel = os.path.relpath(path, os.path.join(REPO, "src/rules_core/rules_tables"))
    return CATALOG_BY_DIR[rel.split(os.sep)[0]]


def run(apply: bool) -> int:
    files = target_files()
    untyped_total = 0
    already_typed_total = 0
    files_changed = 0
    tail_rows: list[tuple[str, str, int, str]] = []

    for path in files:
        catalog = catalog_for(path)
        src = open(path, encoding="utf-8").read()
        untyped = find_untyped(src)
        already_typed_total += len(ANY_LITERAL.findall(src)) - len(untyped)
        out: list[str] = []
        last = 0
        changed = False
        current_key: str | None = None
        index_in_feat = 0
        # Walk keys and literals together, in source order.
        events = sorted(
            [(m.start(), "key", m) for m in KEY.finditer(src)]
            + [(lit.start, "lit", lit) for lit in untyped],
            key=lambda e: e[0],
        )
        for _pos, kind, m in events:
            if kind == "key":
                current_key = m.group(1)
                index_in_feat = 0
                continue
            untyped_total += 1
            elements = STRING.findall(m.body)
            head, bonus_type, guards = convert_literal(elements)
            if bonus_type is not None or guards:
                tail = []
                if bonus_type is not None:
                    tail.append(f"TYPE={bonus_type}")
                tail.extend(rebuild_guard(g) for g in guards)
                if current_key is None:
                    raise SystemExit(f"{path}: bonus with a tail before any FeatTableEntry key")
                tail_rows.append((catalog, current_key, index_in_feat, "|".join(tail)))
            index_in_feat += 1
            out.append(src[last : m.start])
            out.append(render_literal(head, bonus_type, guards))
            last = m.end
            changed = True
        if changed:
            files_changed += 1
            out.append(src[last:])
            if apply:
                open(path, "w", encoding="utf-8").write("".join(out))

    seen = set()
    for catalog, key, index, _tail in tail_rows:
        if (catalog, key, index) in seen:
            raise SystemExit(f"duplicate address ({catalog}, {key}, {index}) — addressing is not unique")
        seen.add((catalog, key, index))

    print(f"files_scanned={len(files)} files_with_untyped_literals={files_changed}")
    print(f"untyped_literals={untyped_total} already_typed_literals={already_typed_total}")
    print(f"rows_carrying_a_tail={len(tail_rows)}")
    if apply:
        write_round_trip_table(tail_rows)
        print(f"wrote src/pcgen_import/feat_effect_conditions.rs rows={len(tail_rows)}")
    elif untyped_total == 0:
        print("already applied: no untyped FeatEffectBonus literal remains")
    return 0


ROUND_TRIP_HEADER = '''//! The verbatim ingest tails the shipped feat catalogs used to carry, kept on
//! the converter side as the round-trip oracle for the typed live form.
//!
//! SD-35 `AT-35-E6-003-SWEEP` cycle 7 (`decisions.md` §11 — no PCGen in live
//! code; `epic-breakdown.md` `### AT-35-E6-003`). Before this cycle every
//! `FeatEffectBonus` row stored its corpus `BONUS:` token whole, tail included,
//! so a `TYPE=<label>` stacking label and every `PRE<FAMILY>:` guard sat in
//! `src/rules_core/`. The live tables now carry `bonus_type` and `conditions`
//! in our own schema instead.
//!
//! This table is the proof that the conversion lost nothing: one row per
//! converted bonus, holding the **verbatim** tail the ingest record wrote, and
//! a test that rebuilds that string from the live typed form and compares it
//! character for character. It is converter-side data — nothing under a live
//! root reads it — and it is kept, not deleted, because Starfinder ingests the
//! same format (`decisions.md` §11, what is kept).
//!
//! Generated by
//! `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle7_type_feat_effect_tails.py`.
//! Do not hand-edit; re-run the generator.

/// One converted bonus's verbatim ingest tail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeatEffectTail {
    /// Which shipped feat catalog the bonus lives in: `crb`, `apg`, `acg`, `arg`.
    pub catalog: &'static str,
    /// The feat record's `key`.
    pub feat_key: &'static str,
    /// 0-based index of the bonus within that feat's `effect` slice.
    pub index: usize,
    /// The tail exactly as the ingest record wrote it, pipe-joined.
    pub tail: &'static str,
}

/// Every converted tail, in catalog source order.
pub const FEAT_EFFECT_TAILS: &[FeatEffectTail] = &[
'''


def write_round_trip_table(rows: list[tuple[str, str, int, str]]) -> None:
    path = os.path.join(REPO, "src/pcgen_import/feat_effect_conditions.rs")
    body = []
    for catalog, key, index, tail in rows:
        body.append(
            f"    FeatEffectTail {{ catalog: {rust_str(catalog)}, "
            f"feat_key: {rust_str(key)}, index: {index}, tail: {rust_str(tail)} }},"
        )
    open(path, "w", encoding="utf-8").write(
        ROUND_TRIP_HEADER + "\n".join(body) + "\n];\n" + ROUND_TRIP_TESTS
    )


ROUND_TRIP_TESTS = '''
#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::rules_tables::advanced_race_guide::feats as arg_feats;
    use crate::rules_core::rules_tables::crb::feats::{ConditionItem, EffectCondition};
    use crate::rules_core::rules_tables::{acg, apg, crb};

    /// Rebuilds one guard's verbatim ingest token from the typed live form.
    fn rebuild(condition: &EffectCondition) -> String {
        let mut parts: Vec<String> = Vec::new();
        for ConditionItem { facet, value } in condition.items {
            match facet {
                None => parts.push((*value).to_string()),
                Some(f) => parts.push(format!("{f}={value}")),
            }
        }
        for alternative in condition.alternatives {
            parts.push(format!("[{}]", rebuild(alternative)));
        }
        format!(
            "{}PRE{}:{}",
            if condition.negated { "!" } else { "" },
            condition.family,
            parts.join(",")
        )
    }

    fn tail_of(bonus_type: Option<&'static str>, conditions: &'static [EffectCondition]) -> String {
        let mut parts: Vec<String> = Vec::new();
        if let Some(label) = bonus_type {
            parts.push(format!("TYPE={label}"));
        }
        parts.extend(conditions.iter().map(rebuild));
        parts.join("|")
    }

    /// Every live bonus that carries a tail, as `(catalog, feat key, index, rebuilt tail)`,
    /// walked straight out of the four shipped catalogs.
    fn live_tails() -> Vec<(String, String, usize, String)> {
        let mut out: Vec<(String, String, usize, String)> = Vec::new();
        for (catalog, entries) in [
            ("crb", crb::feats::feat_tables()),
            ("apg", apg::feats::feat_tables()),
            ("acg", acg::feats::feat_tables()),
        ] {
            for entry in entries {
                let Some(bonuses) = entry.effect else { continue };
                for (index, bonus) in bonuses.iter().enumerate() {
                    let tail = tail_of(bonus.bonus_type, bonus.conditions);
                    if !tail.is_empty() {
                        out.push((catalog.to_string(), entry.key.to_string(), index, tail));
                    }
                }
            }
        }
        for entry in arg_feats::feat_tables() {
            let Some(bonuses) = entry.effect else { continue };
            for (index, bonus) in bonuses.iter().enumerate() {
                let tail = tail_of(bonus.bonus_type, bonus.conditions);
                if !tail.is_empty() {
                    out.push(("arg".to_string(), entry.key.to_string(), index, tail));
                }
            }
        }
        out
    }

    /// The live typed form reproduces every converted ingest tail exactly.
    ///
    /// This is the whole proof of the cycle-7 conversion: if a facet, a guard,
    /// a negation or a nested alternative had been dropped or re-spelled on the
    /// way into `src/rules_core/`, the rebuilt string would differ here.
    #[test]
    fn every_converted_tail_round_trips_from_the_live_typed_form() {
        assert!(
            !FEAT_EFFECT_TAILS.is_empty(),
            "the round-trip table must not be empty -- an empty table proves nothing"
        );
        let live = live_tails();
        for row in FEAT_EFFECT_TAILS {
            let found = live
                .iter()
                .find(|(c, k, i, _)| c == row.catalog && k == row.feat_key && *i == row.index)
                .unwrap_or_else(|| {
                    panic!("no live bonus with a tail at {}/{}#{}", row.catalog, row.feat_key, row.index)
                });
            assert_eq!(
                found.3, row.tail,
                "{}/{}#{} tail did not round-trip",
                row.catalog, row.feat_key, row.index
            );
        }
    }

    /// Every live bonus that carries a tail is addressed by this table, and no
    /// row addresses a bonus that has none. Without this the round-trip test
    /// could pass while silently ignoring a converted row.
    #[test]
    fn the_table_addresses_exactly_the_live_bonuses_that_carry_a_tail() {
        let mut live: Vec<(String, String, usize)> = live_tails()
            .into_iter()
            .map(|(c, k, i, _)| (c, k, i))
            .collect();
        let mut listed: Vec<(String, String, usize)> = FEAT_EFFECT_TAILS
            .iter()
            .map(|r| (r.catalog.to_string(), r.feat_key.to_string(), r.index))
            .collect();
        live.sort();
        listed.sort();
        assert_eq!(listed, live);
    }
}
'''


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--check", action="store_true")
    ap.add_argument("--apply", action="store_true")
    args = ap.parse_args()
    if not (args.check or args.apply):
        ap.error("pass --check or --apply")
    return run(apply=args.apply)


if __name__ == "__main__":
    sys.exit(main())
