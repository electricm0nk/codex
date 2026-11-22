#!/usr/bin/env python3
"""Type the character's own selection out of every shipped `FeatEffectBonus`
qualifier chain -- SD-35 `AT-35-E6-003-SWEEP` cycle 12, `decisions.md` §11.

The job
-------
`rules_tables::{crb,apg,acg,advanced_race_guide}::feat_data::*` ship
`FeatEffectBonus { qualifiers: &[...] }` rows whose qualifier list is the
corpus `BONUS:` chain, pipe-split. Ten of those rows carry the ingest format's
marker for "whatever the player picked":

    WEAPONPROF=%LIST                                category slot (0)
    %LIST                                           target slot (1), SKILL bonus
    SCHOOL.%LIST                                    target slot (1), DC bonus
    var("SKILLRANK=%LIST")                          value slot (2)
    count("ABILITIES","TYPE=FavoredClassBonus")     value slot (2)

Twelve `pcgen_residue_gate.py` hits across four files, and the largest single
reachable job left on this criterion (`…_cycle11_receipt.md`, "Next-cycle
scope"). This script performs the conversion cycle 7 performed for the
stacking label and the guards: the marker slot leaves `qualifiers`, the
fact it carried is named in this crate's own `EffectSelection` schema, and the
verbatim pre-conversion chain is written converter-side as the round-trip
oracle.

What it writes
--------------
1. every `FeatEffectBonus { ... }` literal under `src/rules_core/rules_tables/`
   and `src/rules_core/damage_total.rs`, plus the two test files, gains a
   `selection:` field -- `None` for the 282 unaffected rows, `Some(<variant>)`
   for the 10 that carried a marker, with the marker slot dropped
   from `qualifiers`;
2. `src/pcgen_import/feat_effect_selections.rs` -- the verbatim chains and the
   slot each marker sat in, with the round-trip test.

Modes
-----
    --apply   rewrite the files
    --check   re-derive from the tree and exit 1 if any shipped qualifier slot
              still carries an ingest selection marker, or if any
              converted row's recorded chain does not rebuild

The conversion table below is DATA READ OFF THE CORPUS ROWS THEMSELVES, not a
hand-guessed mapping: `--check` re-reads the shipped tables and fails if a row
it does not know about turns up carrying a marker.
"""

from __future__ import annotations

import argparse
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[5]

# The ingest spellings that mean "the character's own choice", mapped to the
# live-side variant name and the qualifier slot the marker occupied.
# Keyed by the EXACT qualifier-slot source text as it appears in the shipped
# table (Rust source form, `\\\"` escapes included).
INGEST_MARKERS: dict[str, tuple[str, int]] = {
    '"WEAPONPROF=%LIST"': ("ChosenWeapon", 0),
    '"%LIST"': ("ChosenSkill", 1),
    '"SCHOOL.%LIST"': ("ChosenSpellSchool", 1),
    '"var(\\\\\\"SKILLRANK=%LIST\\\\\\")"': ("ChosenSkillRanks", 2),
    '"count(\\\\\\"ABILITIES\\\\\\",\\\\\\"TYPE=FavoredClassBonus\\\\\\")"': (
        "FavoredClassBonusCount",
        2,
    ),
}

# Every file holding `FeatEffectBonus { ... }` literals.
TARGET_GLOBS = (
    "src/rules_core/rules_tables/**/feat_data/*.rs",
    "src/rules_core/rules_tables/crb/feats.rs",
    "src/rules_core/rules_tables/advanced_race_guide/feats.rs",
    "src/rules_core/damage_total.rs",
    "tests/v06_apg_acg_feat_catalog.rs",
    "tests/sd19_feat_catalog.rs",
)

# Matches the field in EITHER layout the converter emits: appended inline on a
# one-line literal, or on its own indented line inside a multi-line one. A
# regex that only matched the inline form silently re-read a multi-line row as
# "no selection" and reset it to `None` -- caught by the round-trip oracle.
SELECTION_FIELD = re.compile(
    r",\s*selection: (?:None|Some\(EffectSelection::(?P<variant>[A-Za-z]+)\)),?"
)


def find_literals(text: str) -> list[tuple[int, int]]:
    """Byte spans of every `FeatEffectBonus { ... }` literal, brace-balanced."""
    spans: list[tuple[int, int]] = []
    needle = "FeatEffectBonus {"
    start = text.find(needle)
    while start != -1:
        # `pub struct FeatEffectBonus {` is the type declaration, not a literal.
        if text[:start].rstrip().endswith("struct"):
            start = text.find(needle, start + len(needle))
            continue
        i = start + len(needle) - 1  # at the '{'
        depth = 0
        j = i
        in_str = False
        escaped = False
        while j < len(text):
            ch = text[j]
            if in_str:
                if escaped:
                    escaped = False
                elif ch == "\\":
                    escaped = True
                elif ch == '"':
                    in_str = False
            elif ch == '"':
                in_str = True
            elif ch == "{":
                depth += 1
            elif ch == "}":
                depth -= 1
                if depth == 0:
                    break
            j += 1
        spans.append((start, j + 1))
        start = text.find(needle, j + 1)
    return spans


QUALS = re.compile(r"qualifiers: &\[(?P<body>[^\]]*)\]")


def split_slots(body: str) -> list[str]:
    """Split a qualifier-array body into its slot source texts."""
    slots: list[str] = []
    depth = 0
    in_str = False
    escaped = False
    cur = ""
    for ch in body:
        if in_str:
            cur += ch
            if escaped:
                escaped = False
            elif ch == "\\":
                escaped = True
            elif ch == '"':
                in_str = False
            continue
        if ch == '"':
            in_str = True
            cur += ch
            continue
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
        if ch == "," and depth == 0:
            slots.append(cur.strip())
            cur = ""
            continue
        cur += ch
    if cur.strip():
        slots.append(cur.strip())
    return slots


def slot_text(slot: str) -> str:
    """One qualifier slot's string contents, with the literal's own layout
    whitespace collapsed -- a multi-line literal must not leak line breaks into
    the recorded chain."""
    return " ".join(slot.strip().strip('"').split())


def classify(slots: list[str]) -> tuple[str | None, int | None]:
    """The selection variant this chain carries, and the slot it sits in."""
    for index, slot in enumerate(slots):
        hit = INGEST_MARKERS.get(slot)
        if hit is None:
            continue
        variant, expected_slot = hit
        if index != expected_slot:
            raise SystemExit(
                f"marker {slot} found at slot {index}, expected {expected_slot} "
                f"-- the conversion is position-dependent and this chain breaks it"
            )
        return variant, index
    return None, None


def convert_literal(src: str) -> tuple[str, str | None, str | None]:
    """Rewritten literal, the variant it carried, the verbatim chain."""
    m = QUALS.search(src)
    if m is None:
        raise SystemExit(f"no qualifiers array in: {src[:120]}")
    slots = split_slots(m.group("body"))
    variant, index = classify(slots)
    verbatim = "|".join(slot_text(s) for s in slots) if variant else None

    if variant is not None:
        assert index is not None
        kept = slots[:index] + slots[index + 1 :]
        new_quals = "qualifiers: &[" + ", ".join(kept) + "]"
        src = src[: m.start()] + new_quals + src[m.end() :]
        field = f"selection: Some(EffectSelection::{variant})"
    else:
        # Idempotence: a re-run sees an ALREADY-converted row, whose qualifier
        # list no longer carries the marker. Preserve the field it already
        # has and re-derive the chain from it, rather than resetting it to
        # `None` and silently undoing the conversion.
        existing = SELECTION_FIELD.search(src)
        kept_variant = existing.group("variant") if existing else None
        if kept_variant is not None:
            variant = kept_variant
            marker, position = MARKER_BY_VARIANT[variant]
            chain = [slot_text(s) for s in slots]
            chain.insert(position, marker)
            verbatim = "|".join(chain)
            field = f"selection: Some(EffectSelection::{variant})"
        else:
            field = "selection: None"

    # Drop the already-present selection field; `field` replaces it.
    src = re.sub(
        r",\s*selection: (?:None|Some\(EffectSelection::[A-Za-z]+\)),?", "", src
    )
    # Append the field, preserving the literal's own layout: a one-line row
    # stays one line, a multi-line row gets its own indented line. Getting this
    # wrong produces source that still compiles in some shapes and not others,
    # so it is done by construction rather than by a trailing-space assumption.
    assert src.endswith("}")
    inner = src[:-1]
    stripped = inner.rstrip()
    trailing = inner[len(stripped) :]
    if "\n" in trailing:
        indent = trailing.rsplit("\n", 1)[-1]
        if not stripped.endswith(","):
            stripped += ","
        src = f"{stripped}\n{indent}    {field},{trailing}}}"
    else:
        if stripped.endswith(","):
            stripped = stripped[:-1]
        src = f"{stripped}, {field} }}"
    return src, variant, verbatim


def catalog_of(path: pathlib.Path) -> str:
    parts = path.parts
    for name, catalog in (
        ("crb", "crb"),
        ("apg", "apg"),
        ("acg", "acg"),
        ("advanced_race_guide", "arg"),
    ):
        if name in parts:
            return catalog
    return "other"


FEAT_KEY = re.compile(r'FeatTableEntry \{ key: "((?:[^"\\]|\\.)*)"')


def walk(path: pathlib.Path):
    """Yield `(span, feat_key, index_within_entry, literal_source)` per literal."""
    text = path.read_text()
    entry_starts = [(m.start(), m.group(1)) for m in FEAT_KEY.finditer(text)]

    def entry_for(pos: int) -> tuple[str, int]:
        key, base = "", 0
        for start, k in entry_starts:
            if start <= pos:
                key, base = k, start
            else:
                break
        return key, base

    index_in_entry: dict[int, int] = {}
    for start, end in find_literals(text):
        key, base = entry_for(start)
        idx = index_in_entry.get(base, 0)
        index_in_entry[base] = idx + 1
        yield (start, end), key, idx, text[start:end]


def rewrite(path: pathlib.Path) -> list[dict]:
    """`--apply`: convert every literal in `path`; return the converted rows."""
    text = path.read_text()
    rows: list[dict] = []
    out: list[str] = []
    last = 0
    for (start, end), key, idx, src in walk(path):
        new_src, variant, verbatim = convert_literal(src)
        if variant is not None:
            rows.append(
                {
                    "catalog": catalog_of(path),
                    "feat_key": key,
                    "index": idx,
                    "variant": variant,
                    "chain": verbatim,
                    "file": str(path.relative_to(ROOT)),
                }
            )
        out.append(text[last:start])
        out.append(new_src)
        last = end
    out.append(text[last:])
    new_text = ensure_import("".join(out))
    if new_text != text:
        path.write_text(new_text)
    return rows


IMPORT = re.compile(r"^use (?P<path>[A-Za-z_:][A-Za-z_0-9:]*)::\{(?P<names>[^}]*)\};$", re.M)


def ensure_import(text: str) -> str:
    """A file that now names `EffectSelection` must import it.

    The name is added to the existing grouped `use` that already brings in
    `FeatEffectBonus` from the same module, keeping the list alphabetical --
    never a second `use` line for one name.
    """
    if "EffectSelection::" not in text:
        return text
    for m in IMPORT.finditer(text):
        names = [n.strip() for n in m.group("names").split(",") if n.strip()]
        if "FeatEffectBonus" not in names or "EffectSelection" in names:
            continue
        names.append("EffectSelection")
        names.sort()
        replacement = f"use {m.group('path')}::{{{', '.join(names)}}};"
        return text[: m.start()] + replacement + text[m.end() :]
    return text


def reread(path: pathlib.Path) -> tuple[list[dict], int]:
    """`--check`: re-derive the converted rows from the tree as it stands, and
    count qualifier slots that still carry an ingest marker."""
    rows: list[dict] = []
    leaks = 0
    for _span, key, idx, src in walk(path):
        m = QUALS.search(src)
        if m is None:
            raise SystemExit(f"no qualifiers array in: {src[:120]}")
        slots = split_slots(m.group("body"))
        for slot in slots:
            if slot in INGEST_MARKERS:
                print(f"LEAK {path.relative_to(ROOT)}: {slot}")
                leaks += 1
        fm = SELECTION_FIELD.search(src)
        if fm is None:
            raise SystemExit(
                f"{path.relative_to(ROOT)}: literal carries no `selection:` field: {src[:120]}"
            )
        variant = fm.group("variant")
        if variant is None:
            continue
        marker, position = MARKER_BY_VARIANT[variant]
        chain_slots = [slot_text(s) for s in slots]
        chain_slots.insert(position, marker)
        rows.append(
            {
                "catalog": catalog_of(path),
                "feat_key": key,
                "index": idx,
                "variant": variant,
                "chain": "|".join(chain_slots),
                "file": str(path.relative_to(ROOT)),
            }
        )
    return rows, leaks


ORACLE_HEADER = '''//! The verbatim ingest qualifier chains the shipped feat catalogs used to
//! carry a selection marker in, kept converter-side as the round-trip
//! oracle for the typed live form.
//!
//! SD-35 `AT-35-E6-003-SWEEP` cycle 12 (`decisions.md` §11 -- no PCGen in live
//! code; `epic-breakdown.md` `### AT-35-E6-003`). Before this cycle ten
//! `FeatEffectBonus` rows across four shipped tables stored the ingest
//! format's marker for "whatever the player picked" -- `WEAPONPROF=%LIST`
//! in the category slot, `%LIST` / `SCHOOL.%LIST` in the target slot,
//! `var("SKILLRANK=%LIST")` and `count("ABILITIES","TYPE=FavoredClassBonus")`
//! in the value slot. Twelve of `scripts/pcgen_residue_gate.py`'s code hits,
//! in `crb/feat_data/{combat,general}.rs`, `acg/feat_data/combat.rs` and
//! `advanced_race_guide/feat_data/general.rs`.
//!
//! The live tables now carry
//! [`EffectSelection`](crate::rules_core::rules_tables::crb::feats::EffectSelection)
//! instead, and the marker slot is gone from `qualifiers`.
//!
//! This table is the proof that the conversion lost nothing: one row per
//! converted bonus, holding the **verbatim** pre-conversion chain and the slot
//! the marker sat in, and a test that rebuilds that chain from the live
//! typed form and compares it element for element. It is converter-side data
//! -- nothing under a live root reads it -- and it is kept, not deleted,
//! because Starfinder ingests the same format (`decisions.md` §11, what is
//! kept).
//!
//! Generated by
//! `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle12_selection_typing.py`.
//! Do not hand-edit; re-run the generator.

use crate::rules_core::rules_tables::crb::feats::EffectSelection;

/// One converted bonus's verbatim pre-conversion qualifier chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeatEffectSelectionRow {
    /// Which shipped feat catalog the bonus lives in: `crb`, `apg`, `acg`, `arg`.
    pub catalog: &'static str,
    /// The feat record's `key`.
    pub feat_key: &'static str,
    /// 0-based index of the bonus within that feat's `effect` slice.
    pub index: usize,
    /// The typed form the live side now carries.
    pub selection: EffectSelection,
    /// 0-based qualifier slot the ingest marker occupied.
    pub slot: usize,
    /// The marker exactly as the ingest record wrote it.
    pub marker: &'static str,
    /// The whole pre-conversion chain, pipe-joined, verbatim.
    pub chain: &'static str,
}

/// The ingest spelling each typed selection was converted from, and the
/// qualifier slot it occupied. This is the inverse of the conversion, and the
/// only place in the repository that still knows it.
pub const fn ingest_marker(selection: EffectSelection) -> (&'static str, usize) {
    match selection {
        EffectSelection::ChosenWeapon => ("WEAPONPROF=%LIST", 0),
        EffectSelection::ChosenSkill => ("%LIST", 1),
        EffectSelection::ChosenSpellSchool => ("SCHOOL.%LIST", 1),
        EffectSelection::ChosenSkillRanks => ("var(\\\\\\"SKILLRANK=%LIST\\\\\\")", 2),
        EffectSelection::FavoredClassBonusCount => {
            ("count(\\\\\\"ABILITIES\\\\\\",\\\\\\"TYPE=FavoredClassBonus\\\\\\")", 2)
        }
    }
}

/// Every converted chain, in catalog source order.
pub const FEAT_EFFECT_SELECTIONS: &[FeatEffectSelectionRow] = &[
'''

ORACLE_TESTS = '''];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::rules_tables::advanced_race_guide::feats as arg_feats;
    use crate::rules_core::rules_tables::{acg, apg, crb};

    /// Rebuild one live bonus's pre-conversion chain: re-insert the ingest
    /// marker at the slot it came out of.
    fn rebuilt_chain(qualifiers: &[&str], selection: EffectSelection) -> String {
        let (marker, slot) = ingest_marker(selection);
        let mut parts: Vec<String> = qualifiers.iter().map(|q| (*q).to_string()).collect();
        parts.insert(slot, marker.to_string());
        parts.join("|")
    }

    /// Every live bonus carrying a selection, as
    /// `(catalog, feat key, index, selection, rebuilt chain)`.
    fn live_selections() -> Vec<(String, String, usize, EffectSelection, String)> {
        let mut out = Vec::new();
        for (catalog, entries) in [
            ("crb", crb::feats::feat_tables()),
            ("apg", apg::feats::feat_tables()),
            ("acg", acg::feats::feat_tables()),
        ] {
            for entry in entries {
                let Some(bonuses) = entry.effect else { continue };
                for (index, bonus) in bonuses.iter().enumerate() {
                    if let Some(selection) = bonus.selection {
                        out.push((
                            catalog.to_string(),
                            entry.key.to_string(),
                            index,
                            selection,
                            rebuilt_chain(bonus.qualifiers, selection),
                        ));
                    }
                }
            }
        }
        for entry in arg_feats::feat_tables() {
            let Some(bonuses) = entry.effect else { continue };
            for (index, bonus) in bonuses.iter().enumerate() {
                if let Some(selection) = bonus.selection {
                    out.push((
                        "arg".to_string(),
                        entry.key.to_string(),
                        index,
                        selection,
                        rebuilt_chain(bonus.qualifiers, selection),
                    ));
                }
            }
        }
        out
    }

    /// The live typed form reproduces every converted ingest chain exactly.
    ///
    /// This is the whole proof of the cycle-12 conversion: if a slot had been
    /// dropped, re-ordered or re-spelled on the way into `src/rules_core/`,
    /// the rebuilt chain would differ here.
    #[test]
    fn every_converted_chain_round_trips_from_the_live_typed_form() {
        assert!(
            !FEAT_EFFECT_SELECTIONS.is_empty(),
            "the round-trip table must not be empty -- an empty table proves nothing"
        );
        let live = live_selections();
        for row in FEAT_EFFECT_SELECTIONS {
            let found = live
                .iter()
                .find(|(c, k, i, _, _)| c == row.catalog && k == row.feat_key && *i == row.index)
                .unwrap_or_else(|| {
                    panic!(
                        "no live bonus with a selection at {}/{}#{}",
                        row.catalog, row.feat_key, row.index
                    )
                });
            assert_eq!(found.3, row.selection, "{}/{}#{} selection changed", row.catalog, row.feat_key, row.index);
            assert_eq!(
                found.4, row.chain,
                "{}/{}#{} chain did not round-trip",
                row.catalog, row.feat_key, row.index
            );
        }
    }

    /// Every live bonus that carries a selection is addressed by this table,
    /// and no row addresses a bonus that has none. Without this the round-trip
    /// test could pass while silently ignoring a converted row.
    #[test]
    fn the_table_addresses_exactly_the_live_bonuses_that_carry_a_selection() {
        let mut live: Vec<(String, String, usize)> = live_selections()
            .into_iter()
            .map(|(c, k, i, _, _)| (c, k, i))
            .collect();
        let mut listed: Vec<(String, String, usize)> = FEAT_EFFECT_SELECTIONS
            .iter()
            .map(|r| (r.catalog.to_string(), r.feat_key.to_string(), r.index))
            .collect();
        live.sort();
        listed.sort();
        assert_eq!(listed, live);
    }

    /// Each recorded row's `marker`/`slot` pair is the one
    /// `ingest_marker` returns for its variant -- so the table and the
    /// inverse function can never drift apart.
    #[test]
    fn the_recorded_marker_matches_the_inverse_function() {
        for row in FEAT_EFFECT_SELECTIONS {
            let (marker, slot) = ingest_marker(row.selection);
            assert_eq!(row.marker, marker, "{}/{}", row.catalog, row.feat_key);
            assert_eq!(row.slot, slot, "{}/{}", row.catalog, row.feat_key);
        }
    }
}
'''


def render_oracle(rows: list[dict]) -> str:
    # Only the four SHIPPED catalogs are addressable by `(catalog, key, index)`
    # in `live_selections()`. A `FeatEffectBonus` literal built inside a
    # `#[cfg(test)]` fixture (`damage_total.rs`, `tests/*.rs`) is converted the
    # same way -- it must be, or the fixture would stop exercising the shipped
    # shape -- but it is not a corpus row and has no place in the round-trip
    # table, which asserts it addresses EXACTLY the live catalog rows.
    rows = [r for r in rows if r["catalog"] != "other"]
    order = {"crb": 0, "apg": 1, "acg": 2, "arg": 3}
    rows = sorted(rows, key=lambda r: (order.get(r["catalog"], 9), r["feat_key"], r["index"]))
    body = []
    for r in rows:
        marker, slot = MARKER_BY_VARIANT[r["variant"]]
        body.append(
            "    FeatEffectSelectionRow {{ catalog: \"{catalog}\", feat_key: \"{key}\", "
            "index: {index}, selection: EffectSelection::{variant}, slot: {slot}, "
            "marker: \"{marker}\", chain: \"{chain}\" }},\n".format(
                catalog=r["catalog"],
                key=r["feat_key"],
                index=r["index"],
                variant=r["variant"],
                slot=slot,
                marker=marker,
                chain=r["chain"],
            )
        )
    return ORACLE_HEADER + "".join(body) + ORACLE_TESTS


MARKER_BY_VARIANT = {
    variant: (spelling.strip('"'), slot)
    for spelling, (variant, slot) in INGEST_MARKERS.items()
}


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--apply", action="store_true")
    ap.add_argument("--check", action="store_true")
    args = ap.parse_args()
    if args.apply == args.check:
        ap.error("pass exactly one of --apply / --check")

    paths: list[pathlib.Path] = []
    for glob in TARGET_GLOBS:
        paths.extend(sorted(ROOT.glob(glob)))
    paths = [p for p in paths if p.is_file()]

    oracle = ROOT / "src/pcgen_import/feat_effect_selections.rs"
    rows: list[dict] = []
    leaks = 0

    if args.apply:
        for path in paths:
            rows.extend(rewrite(path))
    else:
        for path in paths:
            path_rows, path_leaks = reread(path)
            rows.extend(path_rows)
            leaks += path_leaks

    literals = sum(len(find_literals(p.read_text())) for p in paths)
    print(f"files={len(paths)} literals={literals} converted_rows={len(rows)}")
    for r in sorted(rows, key=lambda r: (r["catalog"], r["feat_key"], r["index"])):
        print(f"  {r['catalog']}/{r['feat_key']}#{r['index']} -> {r['variant']}  [{r['chain']}]")

    rendered = render_oracle(rows)
    if args.apply:
        oracle.write_text(rendered)
        print(f"wrote {oracle.relative_to(ROOT)}")
        return 0

    drift = 0 if oracle.exists() and oracle.read_text() == rendered else 1
    verdict = "PASS" if leaks == 0 and drift == 0 else "FAIL"
    print(f"leaks={leaks} oracle_drift={drift} verdict={verdict}")
    return 1 if leaks or drift else 0


if __name__ == "__main__":
    sys.exit(main())
