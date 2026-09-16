#!/usr/bin/env python3
"""Convert the companion catalogs' verbatim `PRE<FAMILY>:` guard strings into
this crate's own typed schema -- SD-35 Epic 6, `AT-35-E6-003-SWEEP` cycle 9.

Why this exists
---------------
`decisions.md` §11: nothing on the live side may read a PCGen token. Three
companion-chassis fields still stored one whole:

    CompanionDescriptionVariant.conditions : &[&str]   -- 24 guard strings
    CompanionAbilityRecord.ability_grants  : &[&str]   --  3 guard tails
    NaturalAttackDamageBonus.formula       : &str      --  3 guard tails

Every one is a `PRE<FAMILY>:<argument>` gate (negated with a leading `!`). This
is the same shape cycle 7 converted for `FeatEffectBonus`, and it reuses cycle
7's `EffectCondition` / `ConditionItem` types verbatim rather than declaring a
second vocabulary for the same thing.

What changes, and what deliberately does not
--------------------------------------------
* `conditions` becomes `&'static [EffectCondition]`.
* `ability_grants` becomes `&'static [CompanionAbilityGrant]`, whose
  `conditions` field carries the guard the pipe-tail used to hold. The grant's
  own three head fields (`kind|mode|name`) were already this crate's own
  vocabulary -- `"Special Ability"`, `"AUTOMATIC"`, a feature name -- so they
  are split, not reinterpreted.
* `NaturalAttackDamageBonus` gains `conditions`; `formula` keeps only the
  formula half. `max(0,(STR/2))|PREVARLT:MasterLevel,7` shipped the guard into
  a field whose doc comment promises "the token's trailing formula half".
* **Nothing is re-interpreted.** No count is inferred, no comparison operator is
  parsed out of a family name, no guard is evaluated. The conversion is a
  lossless re-shaping, and the round-trip table it writes to
  `src/pcgen_import/companion_pcgen_guards.rs` is the proof: every converted
  guard is rebuilt character for character from the live typed form.
* The book cache stays byte-identical. `src/bin/gen_book_cache.rs` serialises
  `v.conditions` onto the wire; it now renders the same string from the typed
  form via `pcgen_import::rebuild_condition`, which is converter-side code and
  is never scanned by the residue gate.

Run:
    python3 .../AT-35-E6-003-SWEEP_cycle9_type_companion_guards.py --check
    python3 .../AT-35-E6-003-SWEEP_cycle9_type_companion_guards.py --apply
"""

import argparse
import glob
import os
import re
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", ".."))

GUARD = re.compile(r"^(?P<neg>!?)PRE(?P<family>[A-Z]+):(?P<arg>.*)$", re.S)
STRING = re.compile(r'"((?:[^"\\]|\\.)*)"')
RECORD_KEY = re.compile(
    r'\bCompanion(?:Ability|Class)?Record \{\s*key: "((?:[^"\\]|\\.)*)"'
)

ROUND_TRIP_REL = "src/pcgen_import/companion_pcgen_guards.rs"


# --- shared with cycle 7's converter; same grammar, same reading ------------
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


def rust_str(value: str) -> str:
    return '"' + value + '"'


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


def rebuild_guard(guard: dict) -> str:
    """The inverse of `parse_guard` -- the verbatim ingest token, rebuilt."""
    parts: list[str] = []
    for facet, value in guard["items"]:
        parts.append(value if facet is None else f"{facet}={value}")
    for alt in guard["alternatives"]:
        parts.append("[" + rebuild_guard(alt) + "]")
    prefix = "!" if guard["negated"] else ""
    return f"{prefix}PRE{guard['family']}:" + ",".join(parts)


def render_guard(guard: dict) -> str:
    """One-line `EffectCondition` literal. These slices sit inside already-long
    generated rows, so a multi-line rendering would not read better."""
    items = ", ".join(
        "ConditionItem { facet: %s, value: %s }"
        % ("None" if facet is None else f"Some({rust_str(facet)})", rust_str(value))
        for facet, value in guard["items"]
    )
    alts = ", ".join(render_guard(a) for a in guard["alternatives"])
    return (
        "EffectCondition { negated: %s, family: %s, items: &[%s], alternatives: &[%s] }"
        % (
            "true" if guard["negated"] else "false",
            rust_str(guard["family"]),
            items,
            alts,
        )
    )


def companion_files() -> list[str]:
    return sorted(
        glob.glob(
            os.path.join(REPO, "src/rules_core/rules_tables/**/companion_data.rs"),
            recursive=True,
        )
    )


_REGISTRY = re.compile(
    r'CompanionBook \{\s*corpus_book: "([^"]+)",\s*companions: super::(\w+)::'
)


def book_of(path: str) -> str:
    """The book's name as `COMPANION_BOOKS` states it, not its directory name.

    Three directories disagree with the registry (`crb` -> `core_rulebook`,
    `apg` -> `advanced_players_guide`, `beastiary1` -> `beastiary`), and the
    round-trip test walks the registry. Derived from `companion_chassis.rs`
    itself rather than hardcoded, so a fourth divergence cannot appear silently.
    """
    rel = os.path.relpath(path, os.path.join(REPO, "src/rules_core/rules_tables"))
    directory = rel.split(os.sep)[0]
    chassis = open(
        os.path.join(REPO, "src/rules_core/rules_tables/companion_chassis.rs"),
        encoding="utf-8",
    ).read()
    mapping = {m.group(2): m.group(1) for m in _REGISTRY.finditer(chassis)}
    return mapping.get(directory, directory)


def record_key_at(src: str, pos: int) -> str:
    """The enclosing record's `key`, whichever of the three record types it is.

    Addressing a converted guard by (book, key, field, index) is what makes the
    round-trip table checkable against the live tables rather than positional.
    Scanning forward for the LAST key that opens before `pos` is correct because
    the three record kinds are never nested inside one another.
    """
    best = ""
    for m in RECORD_KEY.finditer(src):
        if m.start() > pos:
            break
        best = m.group(1)
    return best


def SLICE_BODY(field: str) -> re.Pattern:
    """`<field>: &[ ... ]`, stopping at the FIRST `]`.

    Deliberately not a bracket-balancing scan: no registered guard string
    contains a bracket, and a negated character class that also matches
    newlines would run past the closing bracket into the next field (it did,
    and swallowed `source_file:` -- caught before anything shipped).
    """
    return re.compile(re.escape(field) + r": &\[([^\]]*)\]")


# --- the three conversions -------------------------------------------------
def convert_conditions(src: str, book: str, rows: list) -> str:
    """`conditions: &["PRE…", …]` -> `conditions: &[EffectCondition { … }]`."""

    def one(m: re.Match) -> str:
        inner = m.group(1)
        # Already typed. The slice body regex stops at the first `]`, which for
        # a converted slice lands inside the nested `items: &[`, so without this
        # guard a second run reads `family: "VARLT"` as an ingest string. Being
        # re-runnable is what makes `--check` mean anything.
        if "EffectCondition" in inner:
            return m.group(0)
        values = STRING.findall(inner)
        if not values:
            return m.group(0)
        key = record_key_at(src, m.start())
        out = []
        for i, v in enumerate(values):
            if not GUARD.match(v):
                raise SystemExit(f"{book}/{key}: conditions entry is not a guard: {v!r}")
            g = parse_guard(v)
            if rebuild_guard(g) != v:
                raise SystemExit(f"{book}/{key}: guard does not round-trip: {v!r}")
            rows.append((book, key, "conditions", i, v))
            out.append(render_guard(g))
        return "conditions: &[" + ", ".join(out) + "]"

    return SLICE_BODY("conditions").sub(one, src)


def convert_ability_grants(src: str, book: str, rows: list) -> str:
    """`ability_grants: &["kind|mode|name|PRE…", …]` -> typed grants."""

    def one(m: re.Match) -> str:
        inner = m.group(1)
        if "CompanionAbilityGrant" in inner:
            return m.group(0)
        values = STRING.findall(inner)
        if not values:
            return "ability_grants: &[]"
        key = record_key_at(src, m.start())
        out = []
        for i, v in enumerate(values):
            parts = v.split("|")
            head, guards = [], []
            for p in parts:
                if GUARD.match(p):
                    g = parse_guard(p)
                    if rebuild_guard(g) != p:
                        raise SystemExit(f"{book}/{key}: grant guard lost: {p!r}")
                    guards.append(g)
                else:
                    head.append(p)
            if len(head) != 3:
                raise SystemExit(f"{book}/{key}: grant head is not kind|mode|name: {v!r}")
            if guards:
                rows.append((book, key, "ability_grants", i, "|".join(parts[len(head):])))
            out.append(
                "CompanionAbilityGrant { kind: %s, mode: %s, name: %s, conditions: &[%s] }"
                % (
                    rust_str(head[0]),
                    rust_str(head[1]),
                    rust_str(head[2]),
                    ", ".join(render_guard(g) for g in guards),
                )
            )
        return "ability_grants: &[" + ", ".join(out) + "]"

    return SLICE_BODY("ability_grants").sub(one, src)


NAD = re.compile(
    r'NaturalAttackDamageBonus \{ attack: "((?:[^"\\]|\\.)*)", formula: "((?:[^"\\]|\\.)*)" \}'
)


def convert_natural_attacks(src: str, book: str, rows: list) -> str:
    """Split the guard tail off `formula` into a new `conditions` field.

    Every literal gains the field, guard or not: a struct field is not optional,
    and `conditions: &[]` is the honest reading for a bonus that is unguarded.
    """

    def one(m: re.Match) -> str:
        attack, formula = m.group(1), m.group(2)
        parts = formula.split("|")
        head = [p for p in parts if not GUARD.match(p)]
        guards = []
        for p in parts:
            if GUARD.match(p):
                g = parse_guard(p)
                if rebuild_guard(g) != p:
                    raise SystemExit(f"{book}: natural-attack guard lost: {p!r}")
                guards.append(g)
        if guards:
            key = record_key_at(src, m.start())
            rows.append(
                (book, key or attack, "natural_attack_damage_bonuses", 0,
                 "|".join(p for p in parts if GUARD.match(p)))
            )
        return (
            'NaturalAttackDamageBonus { attack: "%s", formula: "%s", conditions: &[%s] }'
            % (attack, "|".join(head), ", ".join(render_guard(g) for g in guards))
        )

    return NAD.sub(one, src)


ROUND_TRIP_HEADER = '''//! The verbatim ingest guard tails the shipped companion catalogs used to
//! carry, kept on the converter side as the round-trip oracle for the typed
//! live form.
//!
//! SD-35 `AT-35-E6-003-SWEEP` cycle 9 (`decisions.md` §11 -- no PCGen in live
//! code). Before this cycle `CompanionDescriptionVariant.conditions` stored its
//! corpus `PRE<FAMILY>:` gate as a string, `CompanionAbilityRecord.ability_grants`
//! stored the gate pipe-appended to the grant, and `NaturalAttackDamageBonus.formula`
//! stored the gate pipe-appended to the formula. All three now carry
//! `EffectCondition` in this crate's own schema (cycle 7's types, reused).
//!
//! This table is the proof that the conversion lost nothing: one row per
//! converted guard, holding the **verbatim** tail the ingest record wrote, plus
//! a test that rebuilds it from the live typed form and compares it character
//! for character. It is converter-side data -- nothing under a live root reads
//! it -- and it is KEPT, not deleted, because Starfinder ingests the same
//! format (`decisions.md` §11, what is kept).
//!
//! Generated by
//! `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle9_type_companion_guards.py`.
//! Do not hand-edit; re-run the generator.

use crate::rules_core::rules_tables::crb::feats::EffectCondition;

/// One converted companion guard's verbatim ingest tail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompanionGuardTail {
    /// The book directory the catalog lives in: `crb`, `ultimate_magic`, ...
    pub book: &'static str,
    /// The enclosing `CompanionAbilityRecord`'s `key`.
    pub ability_key: &'static str,
    /// Which field carried it: `conditions`, `ability_grants`,
    /// `natural_attack_damage_bonuses`.
    pub field: &'static str,
    /// 0-based index within that field's slice.
    pub index: usize,
    /// The guard exactly as the ingest record wrote it, pipe-joined.
    pub tail: &'static str,
}

/// Rebuild one typed condition into the verbatim ingest token it came from.
///
/// Converter-side, and public because `src/bin/gen_book_cache.rs` needs it to
/// keep the book cache byte-identical across this conversion: the wire format
/// carries the ingest string, and the live tables no longer do.
pub fn rebuild_condition(condition: &EffectCondition) -> String {
    let mut parts: Vec<String> = Vec::new();
    for item in condition.items {
        match item.facet {
            Some(facet) => parts.push(format!("{facet}={}", item.value)),
            None => parts.push(item.value.to_string()),
        }
    }
    for alternative in condition.alternatives {
        parts.push(format!("[{}]", rebuild_condition(alternative)));
    }
    format!(
        "{}PRE{}:{}",
        if condition.negated { "!" } else { "" },
        condition.family,
        parts.join(",")
    )
}

/// Every converted guard, in book source order.
pub const COMPANION_GUARD_TAILS: &[CompanionGuardTail] = &[
'''

ROUND_TRIP_TESTS = '''];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::rules_tables::companion_chassis::COMPANION_BOOKS;

    /// Every guard the live catalogs now carry in typed form, rebuilt into the
    /// ingest string it was converted from.
    ///
    /// Compared as a sorted multiset rather than in table order: the table is
    /// written in file-glob order and the live walk is in `COMPANION_BOOKS`
    /// order, so an order-sensitive comparison would be asserting the two
    /// orders agree, which is not the property under test. What is under test
    /// is that the same guards, and only those, survive the conversion.
    fn live_tails() -> Vec<(String, String, String, String)> {
        let mut out = Vec::new();
        for book in COMPANION_BOOKS {
            for ability in book.companion_abilities {
                for variant in ability.description_variants {
                    for condition in variant.conditions {
                        out.push((
                            book.corpus_book.to_string(),
                            ability.key.to_string(),
                            "conditions".to_string(),
                            rebuild_condition(condition),
                        ));
                    }
                }
            }
            for class in book.companion_classes {
                for grant in class.ability_grants {
                    if grant.conditions.is_empty() {
                        continue;
                    }
                    out.push((
                        book.corpus_book.to_string(),
                        class.key.to_string(),
                        "ability_grants".to_string(),
                        grant
                            .conditions
                            .iter()
                            .map(rebuild_condition)
                            .collect::<Vec<_>>()
                            .join("|"),
                    ));
                }
            }
            for companion in book.companions {
                for bonus in companion.natural_attack_damage_bonuses {
                    if bonus.conditions.is_empty() {
                        continue;
                    }
                    out.push((
                        book.corpus_book.to_string(),
                        companion.key.to_string(),
                        "natural_attack_damage_bonuses".to_string(),
                        bonus
                            .conditions
                            .iter()
                            .map(rebuild_condition)
                            .collect::<Vec<_>>()
                            .join("|"),
                    ));
                }
            }
        }
        out.sort();
        out
    }

    fn recorded_tails() -> Vec<(String, String, String, String)> {
        let mut out: Vec<_> = COMPANION_GUARD_TAILS
            .iter()
            .map(|r| {
                (
                    r.book.to_string(),
                    r.ability_key.to_string(),
                    r.field.to_string(),
                    r.tail.to_string(),
                )
            })
            .collect();
        out.sort();
        out
    }

    #[test]
    fn every_converted_guard_round_trips_from_the_live_typed_form() {
        assert_eq!(
            live_tails(),
            recorded_tails(),
            "the live typed form no longer rebuilds the verbatim ingest guards this \
             table recorded -- the conversion lost or invented something"
        );
    }

    #[test]
    fn the_table_is_not_empty_and_every_row_names_a_real_guard() {
        let rows = COMPANION_GUARD_TAILS;
        assert!(!rows.is_empty(), "no guard was recorded at all");
        for row in rows {
            assert!(
                row.tail.contains("PRE"),
                "{}/{} {} recorded a tail that is not a guard: {:?}",
                row.book,
                row.ability_key,
                row.field,
                row.tail
            );
            assert!(!row.book.is_empty() && !row.ability_key.is_empty());
        }
    }
}
'''


def write_round_trip(rows: list) -> None:
    body = "".join(
        '    CompanionGuardTail { book: %s, ability_key: %s, field: %s, index: %d, tail: %s },\n'
        % (rust_str(b), rust_str(k), rust_str(f), i, rust_str(t))
        for b, k, f, i, t in rows
    )
    with open(os.path.join(REPO, ROUND_TRIP_REL), "w", encoding="utf-8") as fh:
        fh.write(ROUND_TRIP_HEADER + body + ROUND_TRIP_TESTS)


def run(apply: bool) -> int:
    rows: list = []
    changed = 0
    for path in companion_files():
        book = book_of(path)
        src = open(path, encoding="utf-8").read()
        out = convert_conditions(src, book, rows)
        out = convert_ability_grants(out, book, rows)
        out = convert_natural_attacks(out, book, rows)
        if out != src:
            changed += 1
            if apply:
                with open(path, "w", encoding="utf-8") as fh:
                    fh.write(out)
    guards = len(rows)
    print(f"companion_files={len(companion_files())} files_changed={changed} guards_converted={guards}")
    if apply:
        if not rows:
            # Everything is already typed, so this run harvested nothing and
            # writing would truncate a good table to zero rows. `--apply` is a
            # one-shot conversion: to regenerate the table, revert the
            # `companion_data.rs` files to their pre-conversion state first.
            print(
                "REFUSED: nothing left to convert, so the round-trip table was "
                "NOT rewritten (it would have been emptied)"
            )
            return 0
        write_round_trip(rows)
        print(f"wrote {ROUND_TRIP_REL} rows={guards}")
        return 0
    return 0 if changed == 0 else 1


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--apply", action="store_true")
    ap.add_argument("--check", action="store_true")
    args = ap.parse_args()
    if args.apply == args.check:
        ap.error("pass exactly one of --apply / --check")
    return run(args.apply)


if __name__ == "__main__":
    sys.exit(main())
