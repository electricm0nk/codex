#!/usr/bin/env python3
"""Move the ingest tail off five shipped prose strings -- SD-35 Epic 6,
`AT-35-E6-003-SWEEP` cycle 9.

Why this exists
---------------
Five `description:` / `benefit:` strings in the shipped tables carried an ingest
tail glued to the end of the sentence a player reads on a paper character sheet:

    "…rip magical defenses from your enemy.  PREABILITY:1,CATEGORY=FEAT,…"
    "…as if your effective cleric level were 2 levels lower.|PREABILITY:1,…"
    "…a +%1 enhancement bonus to her base speed.|BeastmorphSpeed|PREVAREQ:…"

That is a sheet-rule defect first and a `decisions.md` §11 defect second: the
sheet prints the rule's words, and `PREVAREQ:BeastmorphProgression,1` is not
words. The tail is two things -- the `%N` argument's variable name, and the
guard gating the token -- and neither is prose.

What this does
--------------
The same move cycle 2 made for `ArchetypeSwapEntry`'s prerequisite tokens: the
tail is **relocated**, not deleted, to `src/pcgen_import/prose_ingest_tails.rs`,
keyed by (table, record key) so any consumer that ever needs it is a lookup
away. The live string keeps the prose and only the prose. No struct gains a
field: 1,853 `ArchetypeGrant` literals and ~3,000 feat literals would have to be
rewritten to carry a field that, today, nothing reads -- and inventing a field
nothing reads is the fabrication this bundle's own rules forbid.

`%1` itself is deliberately left alone. It is a pre-existing shape the residue
gate does not count and a different mechanism (the `%N` substitution the
converter owns); pretending to fix it here by guessing a value would be worse
than leaving it visible.

Run:
    python3 .../AT-35-E6-003-SWEEP_cycle9_prose_tail_relocate.py --check
    python3 .../AT-35-E6-003-SWEEP_cycle9_prose_tail_relocate.py --apply
"""

import argparse
import os
import re
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", ".."))

OUT_REL = "src/pcgen_import/prose_ingest_tails.rs"

# (relative path, record key, field, the prose that is KEPT, the tail that MOVES)
#
# Each row was read out of the file it names; the split point is the first
# element of the pipe- (or, for the one row that glued it on with spaces,
# whitespace-) separated tail that is not prose. Nothing is paraphrased: the
# kept half is a prefix of the original string, character for character, and
# the check below proves it.
ROWS = [
    (
        "src/rules_core/rules_tables/ultimate_combat/feat_tables.rs",
        "Skilled Driver",
        "benefit",
        "You gain a +4 bonus on driving checks with your chosen vehicle.",
        "%LIST",
    ),
    (
        "src/rules_core/rules_tables/ultimate_combat/feat_tables.rs",
        "Dispelling Fist",
        "description",
        "By focusing on your knowledge of magic and spells that negate its powers, you use your bare hands to rip magical defenses from your enemy.",
        "PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike",
    ),
    (
        "src/rules_core/rules_tables/ultimate_magic/feat_tables.rs",
        "Versatile Channeler",
        "benefit",
        "You may choose to channel positive energy as if your effective cleric level were 2 levels lower than normal.",
        "PREABILITY:1,CATEGORY=Special Ability,Versatile Channeler ~ Positive Energy",
    ),
    (
        "src/rules_core/rules_tables/acg/archetype_tables.rs",
        "Mutagenic Mauler Brawler ~ Beastmorph",
        "description",
        "A mutagenic mauler gains additional abilities when using her mutagen. She gains low-light vision and a +%1 enhancement bonus to her base speed.",
        "BeastmorphSpeed|PREVAREQ:BeastmorphProgression,1",
    ),
    (
        "src/rules_core/rules_tables/ultimate_magic/archetype_tables.rs",
        "Internal Alchemist ~ Disease Resistance",
        "description",
        "You gain a +%1 bonus on all saving throws against disease.",
        "AlchemistPoisonResistanceBonus|!PREABILITY:1,CATEGORY=Special Ability,Poison Immunity ~ Alchemist",
    ),
    (
        "src/rules_core/rules_tables/ultimate_psionics/archetype_tables.rs",
        "Thoughtsinger ~ Collective",
        "description",
        "Join %1 minds, plus your own, within %2 feet; can manifest some powers through collective.",
        "ThoughtsingerCollectiveMinds|ThoughtsingerCollectiveRange|!PREABILITY:1,CATEGORY=Internal,Thoughtsinger ~ Collective Range Unlimited",
    ),
]

# The one row whose prose half is not a character-for-character prefix of the
# original: `"...your chosen vehicle (chosen vehicle: %1).|%LIST"` restated the
# selection twice, once in words and once as the ingest slot. The words are
# kept; the slot and the parenthetical that only existed to hold it are not.
LITERAL_REWRITES = {
    (
        "src/rules_core/rules_tables/ultimate_combat/feat_tables.rs",
        "Skilled Driver",
    ): (
        'benefit: Some("You gain a +4 bonus on driving checks with your chosen vehicle (chosen vehicle: %1).|%LIST")',
        'benefit: Some("You gain a +4 bonus on driving checks with your chosen vehicle.")',
    ),
}

HEADER = '''//! The ingest tails five shipped prose strings used to carry, relocated off the
//! live side -- SD-35 `AT-35-E6-003-SWEEP` cycle 9, enforcing `decisions.md`
//! §11 and the sheet rule (`decisions.md` §1).
//!
//! # Why these moved, and why they were not deleted
//!
//! Five `description:` / `benefit:` strings in `rules_tables` ended with an
//! ingest tail glued to the sentence a player reads on a printed character
//! sheet -- a `%N` argument's variable name, a `PRE<FAMILY>:` guard, or both:
//!
//! ```text
//! "...rip magical defenses from your enemy.  PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike"
//! "...a +%1 enhancement bonus to her base speed.|BeastmorphSpeed|PREVAREQ:BeastmorphProgression,1"
//! ```
//!
//! The sheet prints the rule's words; `PREVAREQ:BeastmorphProgression,1` is not
//! words. The live strings now hold the prose and only the prose.
//!
//! This is a **move, not a removal**, on cycle 2's own precedent
//! (`archetype_swap_prereq_tokens`): the tails are the provenance of each
//! converted gate (`AGENTS.md` rule 9) and they are KEPT for Starfinder
//! (`decisions.md` §11, what is kept). Keyed by (table, record key), so a
//! consumer that ever needs one is a lookup away.
//!
//! Generated by
//! `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle9_prose_tail_relocate.py`.
//! Do not hand-edit; re-run the generator.

/// One relocated ingest tail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProseIngestTail {
    /// The shipped table the prose lives in, as a repo-relative path.
    pub table: &'static str,
    /// The record's own corpus key.
    pub record_key: &'static str,
    /// Which prose field carried it: `description` or `benefit`.
    pub field: &'static str,
    /// The prose the live table kept, character for character.
    pub prose: &'static str,
    /// The tail the live table no longer carries, verbatim and pipe-joined.
    pub tail: &'static str,
}

/// Every relocated tail.
pub const PROSE_INGEST_TAILS: &[ProseIngestTail] = &[
'''

TESTS = '''];

#[cfg(test)]
mod tests {
    use super::*;

    /// Every recorded tail is a real ingest tail, and every recorded prose half
    /// is what the live table now ships.
    ///
    /// The prose halves are compared against the shipped tables by the
    /// generator's own `--check` mode rather than at run time: a `rules_tables`
    /// read from here would be a live-side module reading converter-side data,
    /// which is the direction this whole epic exists to prevent.
    #[test]
    fn every_row_carries_prose_on_one_side_and_ingest_vocabulary_on_the_other() {
        assert!(!PROSE_INGEST_TAILS.is_empty());
        for row in PROSE_INGEST_TAILS {
            assert!(
                row.tail.contains("PRE") || row.tail.contains("%LIST"),
                "{}: recorded tail is neither a guard nor a selection slot: {:?}",
                row.record_key,
                row.tail
            );
            assert!(
                !row.prose.contains("PRE")
                    && !row.prose.contains('|')
                    && !row.prose.contains("%LIST"),
                "{}: the kept prose still carries ingest vocabulary: {:?}",
                row.record_key,
                row.prose
            );
            assert!(
                row.field == "description" || row.field == "benefit",
                "{}: unexpected field {:?}",
                row.record_key,
                row.field
            );
        }
    }

    #[test]
    fn the_tails_are_addressed_uniquely() {
        let mut seen = std::collections::HashSet::new();
        for row in PROSE_INGEST_TAILS {
            assert!(
                seen.insert((row.table, row.record_key, row.field)),
                "{}/{} {} recorded twice",
                row.table,
                row.record_key,
                row.field
            );
        }
    }
}
'''


def rust_str(value: str) -> str:
    return '"' + value.replace("\\", "\\\\").replace('"', '\\"') + '"'


def run(apply: bool) -> int:
    changed = 0
    missing = []
    for path, key, field, prose, tail in ROWS:
        full = os.path.join(REPO, path)
        src = open(full, encoding="utf-8").read()
        # The whole shipped literal, prose + separator + tail. The separator is
        # `|` for four rows and two spaces for the one that glued it on that way;
        # both are matched, neither is assumed.
        pattern = re.compile(
            re.escape(f'{field}: Some("{prose}')
            + r'(\s+|\|)'
            + re.escape(f'{tail}")')
        )
        m = pattern.search(src)
        if m is None and (path, key) in LITERAL_REWRITES:
            was, now = LITERAL_REWRITES[(path, key)]
            if was in src:
                m = None
                if apply:
                    src = src.replace(was, now, 1)
                    open(full, "w", encoding="utf-8").write(src)
                changed += 1
                continue
            if now in src:
                continue
        if not m:
            if f'{field}: Some("{prose}")' in src:
                continue  # already converted
            missing.append((path, key, field))
            continue
        changed += 1
        if apply:
            src = src[: m.start()] + f'{field}: Some("{prose}")' + src[m.end():]
            open(full, "w", encoding="utf-8").write(src)
    if missing:
        for path, key, field in missing:
            print(f"NOT FOUND: {path} {key} {field}")
        return 2
    print(f"rows={len(ROWS)} rows_changed={changed}")
    if apply:
        body = "".join(
            "    ProseIngestTail { table: %s, record_key: %s, field: %s, prose: %s, tail: %s },\n"
            % (rust_str(p), rust_str(k), rust_str(f), rust_str(pr), rust_str(t))
            for p, k, f, pr, t in ROWS
        )
        with open(os.path.join(REPO, OUT_REL), "w", encoding="utf-8") as fh:
            fh.write(HEADER + body + TESTS)
        print(f"wrote {OUT_REL} rows={len(ROWS)}")
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
