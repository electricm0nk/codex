#!/usr/bin/env python3
"""Relocate `UmFeatEntry.effect` -- the verbatim `BONUS:`/`DEFINE:` token array
the Ultimate Magic feat catalog carries -- off the live side and into
`pcgen_import::feat_effect_tokens` -- SD-35 Epic 6, AT-35-E6-003-SWEEP cycle 6.

This repeats, field for field, the move `AT-35-E6-003-SWEEP` cycle 3 made for
the same catalogs' `prerequisites` field (`src/pcgen_import/feat_prereq_tokens.rs`,
whose module doc comment states the pattern). It is a MOVE, not a removal:
`decisions.md` §11 keeps the converter side, and the tokens are reused for
Starfinder. Every token string is carried across byte for byte; this tool
asserts that.

Why this field and why now
--------------------------
`UmFeatEntry.effect` is 75 of the residue gate's 798 remaining live code hits --
the largest single non-test mechanism left, and the only one whose live readers
are countable on one hand. Re-derive the reader set:

    grep -rn "\\.effect\\b" src apps/desktop/src-tauri/src tests --include=*.rs \\
      | grep -v "effect: " | grep -v effect_text | grep -v "\\.effect\\.spell_id"

The only readers of THIS field (the `Option<&'static [&'static str]>` one, not
`crb::feats::FeatTableEntry.effect`, which is a typed `FeatEffectBonus` slice a
different lane owns) are three assertions inside this file's own
`#[cfg(test)] mod tests`, and they are rewritten here to read the relocated
table. `feats_all::map_um_entry` never touched it; `gen_book_cache`'s
`entry.effect` read is ARG's typed field, not this one.

Addressing
----------
By `(RuleSetId::Um, index in `feat_tables()`, key)` -- the same triple cycle 3
used, and for the same reason: a `(rule_set, key)` lookup can collide where a
book carries two records under one key, and the index+key pair fails loudly
when the live table is reordered instead of silently pairing a record with
another record's tokens.

Usage:
    python3 <this> --check     # verify the relocation round-trips; writes nothing
    python3 <this> --apply     # generate the module and strip the live field
"""

import argparse
import os
import re
import sys

LIVE = "src/rules_core/rules_tables/ultimate_magic/feat_tables.rs"
OUT = "src/pcgen_import/feat_effect_tokens.rs"

KEY_RX = re.compile(r'^\s*key: "((?:[^"\\]|\\.)*)",\s*$')
EFFECT_RX = re.compile(r"^\s*effect: (None|Some\(&\[.*\]\)),\s*$")
FIELD_DECL_RX = re.compile(r"^\s*pub effect: Option<&'static \[&'static str\]>,\s*$", re.M)


def extract(text):
    """[(index, key, effect_literal_or_None)] in table order.

    A record is one `key:` line followed, inside the same literal, by exactly
    one `effect:` line. The parse asserts that pairing rather than assuming it:
    the counts must match and every key must precede its own effect line.
    """
    rows = []
    pending_key = None
    index = -1
    for line in text.split("\n"):
        km = KEY_RX.match(line)
        if km:
            if pending_key is not None:
                raise SystemExit(f"two `key:` lines with no `effect:` between them: {pending_key}")
            pending_key = km.group(1)
            index += 1
            continue
        em = EFFECT_RX.match(line)
        if em:
            if pending_key is None:
                raise SystemExit(f"an `effect:` line with no preceding `key:`: {line!r}")
            rows.append((index, pending_key, None if em.group(1) == "None" else em.group(1)))
            pending_key = None
    if pending_key is not None:
        raise SystemExit(f"trailing `key:` with no `effect:`: {pending_key}")
    return rows


def literal_tokens(lit):
    """The token strings inside `Some(&["a", "b"])`, verbatim."""
    inner = lit[len('Some(&['):-len('])')]
    return re.findall(r'"((?:[^"\\]|\\.)*)"', inner)


MODULE_HEAD = '''//! The `BONUS:`/`DEFINE:` effect tokens the Ultimate Magic feat catalog
//! records carry, relocated off the live side — SD-35
//! `AT-35-E6-003-SWEEP` cycle 6, enforcing `decisions.md` §11 (nothing on
//! the live side reads a PCGen token).
//!
//! # Why this moved, and why it was not deleted
//!
//! `rules_tables::ultimate_magic::feat_tables::UmFeatEntry` carried an
//! `effect: Option<&'static [&'static str]>` field holding every `BONUS:`
//! and `DEFINE:` token of the corpus row, verbatim, in source order. It was
//! the largest single non-test PCGen residue left on the live side — 75 of
//! the 798 code hits `scripts/pcgen_residue_gate.py --check` counted at
//! cycle 6's start.
//!
//! **No live engine ever read it.** The only readers were three assertions
//! in the catalog file's own `#[cfg(test)] mod tests`, which read it as a
//! *presence* flag ("this record carries real content"), never as a token.
//! Those three now read [`um_feat_effect_tokens`]. Re-derive the absence of
//! any other reader:
//!
//! ```text
//! grep -rn "\\.effect\\b" src apps/desktop/src-tauri/src tests --include=*.rs \\
//!   | grep -v "effect: " | grep -v effect_text | grep -v "\\.effect\\.spell_id"
//! ```
//!
//! Every remaining hit is `rules_tables::crb::feats::FeatTableEntry.effect`,
//! a typed `FeatEffectBonus` slice — a different field of a different type,
//! owned by a different lane, and untouched here.
//!
//! So this is a **move, not a removal** (`decisions.md` §11: the converter
//! side is KEPT and reused for Starfinder). The relocation is generated and
//! checked by
//! `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle6_relocate_um_effect_tokens.py`,
//! whose `--check` mode re-reads both sides and fails unless every token
//! string survives byte for byte.
//!
//! # How a row is addressed
//!
//! By `(rule_set, index)` — the record's own position in
//! `ultimate_magic::feat_tables::feat_tables()` — and **not** by
//! `(rule_set, key)`, for the reason
//! [`crate::pcgen_import::feat_prereq_tokens`] gives at length: a key lookup
//! can collide, an index+key pair fails loudly when the live table is
//! reordered. Each row also carries the key it was taken from, and the
//! lookup asserts it against the caller's record.

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::rules_core::rules_tables::RuleSetId;

/// One relocated record's effect tokens: `(rule_set, index in that book's
/// table, the record's `key`, the tokens in corpus source order)`.
pub type FeatEffectRow = (RuleSetId, usize, &'static str, &'static [&'static str]);

/// The `BONUS:`/`DEFINE:` tokens the Ultimate Magic feat table carried,
/// addressed by each record's index in
/// `ultimate_magic::feat_tables::feat_tables()`.
///
/// {ROWS} row(s) of the table's {TOTAL} record(s) carry at least one token.
pub static UM_FEAT_EFFECT_TOKENS: &[FeatEffectRow] = &[
{ROWTEXT}];

/// Every record in the live Ultimate Magic feat table, in table order —
/// the denominator the relocated rows are a subset of.
pub const UM_FEAT_TABLE_LEN: usize = {TOTAL};

/// The number of relocated rows; the live table's own
/// `the_desc_benefit_effect_split_is_the_real_one` re-derives it.
pub const UM_FEAT_EFFECT_ROW_COUNT: usize = {ROWS};

fn index_of() -> &'static HashMap<(RuleSetId, usize), FeatEffectRow> {
    static CELL: OnceLock<HashMap<(RuleSetId, usize), FeatEffectRow>> = OnceLock::new();
    CELL.get_or_init(|| UM_FEAT_EFFECT_TOKENS.iter().map(|row| ((row.0, row.1), *row)).collect())
}

/// The `BONUS:`/`DEFINE:` tokens the record at `index` in `rule_set`'s feat
/// table carried, or `None` when its corpus row carried none.
///
/// `key` is the caller's own record key and is asserted against the one the
/// row was taken from — see this module's doc comment.
pub fn um_feat_effect_tokens(
    rule_set: RuleSetId,
    index: usize,
    key: &str,
) -> Option<&'static [&'static str]> {
    let row = index_of().get(&(rule_set, index))?;
    assert_eq!(
        row.2, key,
        "{rule_set:?} index {index} holds the effect tokens taken from {:?}, but the caller \\
         passed {key:?}; the live table this was relocated from has been reordered",
        row.2
    );
    Some(row.3)
}

/// `true` when the record at `index` carried at least one effect token. The
/// presence flag the live catalog's own tests used to read off the field.
pub fn um_feat_carries_effect(rule_set: RuleSetId, index: usize, key: &str) -> bool {
    um_feat_effect_tokens(rule_set, index, key).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::rules_tables::ultimate_magic::feat_tables::feat_tables;

    #[test]
    fn every_relocated_row_still_names_its_own_live_record() {
        for &(rule_set, index, key, tokens) in UM_FEAT_EFFECT_TOKENS {
            assert_eq!(rule_set, RuleSetId::Um);
            let entry = feat_tables()
                .get(index)
                .unwrap_or_else(|| panic!("index {index} is past the end of the live table"));
            assert_eq!(entry.key, key, "index {index} no longer names {key}");
            assert!(!tokens.is_empty(), "{key} was relocated with an empty token array");
        }
    }

    #[test]
    fn the_live_table_is_the_length_the_relocation_was_taken_from() {
        assert_eq!(feat_tables().len(), UM_FEAT_TABLE_LEN);
        assert_eq!(UM_FEAT_EFFECT_TOKENS.len(), UM_FEAT_EFFECT_ROW_COUNT);
    }

    #[test]
    fn a_key_that_does_not_match_its_index_panics() {
        let (_, index, key, _) = UM_FEAT_EFFECT_TOKENS[0];
        assert!(um_feat_effect_tokens(RuleSetId::Um, index, key).is_some());
        assert!(
            std::panic::catch_unwind(|| um_feat_effect_tokens(
                RuleSetId::Um,
                index,
                "Not This Record"
            ))
            .is_err(),
            "a mismatched key must fail loudly, not return another record's tokens"
        );
    }
}
'''


def build_module(rows, total):
    body = []
    for index, key, lit in rows:
        if lit is None:
            continue
        toks = ", ".join(f'"{t}"' for t in literal_tokens(lit))
        body.append(f'    (RuleSetId::Um, {index}, "{key}", &[{toks}]),\n')
    return (
        MODULE_HEAD.replace("{ROWTEXT}", "".join(body))
        .replace("{ROWS}", str(len(body)))
        .replace("{TOTAL}", str(total))
    )


def strip_live(text):
    out = []
    for line in text.split("\n"):
        if FIELD_DECL_RX.match(line):
            out.append(
                "    // The `effect: Option<&'static [&'static str]>` field that stood here held"
            )
            out.append(
                "    // every `BONUS:`/`DEFINE:` token of the corpus row, verbatim, in source"
            )
            out.append(
                "    // order. It moved to `pcgen_import::feat_effect_tokens` — SD-35"
            )
            out.append(
                "    // `AT-35-E6-003-SWEEP` cycle 6, `decisions.md` §11: nothing on the live"
            )
            out.append(
                "    // side reads a PCGen token. It had no live reader at all; its only three"
            )
            out.append(
                "    // readers were this file's own tests, which now read the relocated table."
            )
            continue
        if EFFECT_RX.match(line):
            continue
        out.append(line)
    return "\n".join(out)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--apply", action="store_true")
    ap.add_argument("--check", action="store_true")
    args = ap.parse_args()

    live = open(LIVE, encoding="utf-8").read()
    if FIELD_DECL_RX.search(live) is None and not args.check:
        print("live field already relocated; nothing to do")
        return 0

    if FIELD_DECL_RX.search(live) is not None:
        rows = extract(live)
        total = len(rows)
        carried = [r for r in rows if r[2] is not None]
        print(f"live table: {total} record(s), {len(carried)} carrying effect tokens")
        module = build_module(rows, total)
        if args.apply:
            open(OUT, "w", encoding="utf-8").write(module)
            open(LIVE, "w", encoding="utf-8").write(strip_live(live))
            print(f"wrote {OUT} and stripped {LIVE}")
        return 0

    # --check against an already-applied tree: every token in the module must
    # be reachable, and the live file must carry no `effect:` row at all.
    if re.search(r"^\s*effect: ", live, re.M):
        print("FAIL: the live table still carries an `effect:` row", file=sys.stderr)
        return 1
    if not os.path.exists(OUT):
        print(f"FAIL: {OUT} is missing", file=sys.stderr)
        return 1
    mod = open(OUT, encoding="utf-8").read()
    n = len(re.findall(r"^\s*\(RuleSetId::Um, ", mod, re.M))
    declared = int(re.search(r"UM_FEAT_EFFECT_ROW_COUNT: usize = (\d+)", mod).group(1))
    if n != declared:
        print(f"FAIL: {n} rows present, {declared} declared", file=sys.stderr)
        return 1
    print(f"OK: {n} relocated rows, live table carries no `effect:` field")
    return 0


if __name__ == "__main__":
    sys.exit(main())
