#!/usr/bin/env python3
"""SD-32 card 11, T9 -- per-record PI review of the `spell` kind's 352
`uncertain` units (`decisions.md §18`).

**Read-only.** Transcribes nothing, ingests nothing, changes no corpus
data. Does not amend `docs/governance/ogl-pi-blacklist.md` (status stays
`DRAFT`). Extends `scripts/sd32_t9_pi_exposure_audit.py` -- does not
redo it: this script imports that script's own classification so the
`spell`-kind `uncertain`/`clear` sets it reviews are byte-identical to
the audit's, then applies two things the audit's own §5/§8 named as out
of its scope:

1. A **per-record read** of the `spell` kind's 352 `uncertain` records'
   free-text tags (`DESC:`, the only free-text tag `spell` rows ever
   carry), looking for a named PI proper noun the 57-term blacklist list
   does not enumerate -- not just re-running the existing term scan.
2. A **normalized (case-folded + bounded OCR-confusion) re-scan of the
   `spell` kind's `clear` (349) and `uncertain` (352) buckets**, per
   `decisions.md §18` point 2 and `ogl-pi-blacklist.md §4`'s recorded
   `Cayden CaiLean` / `lrori` incident -- the exact-substring scan's own
   documented blind spot.

Re-derive (from repo root, oracle bootstrapped to the repo-local slot):

    cargo build --locked --release --bin v06_work_inventory
    "$CARGO_TARGET_DIR/release/v06_work_inventory" --stdout-only \\
        > fresh_inventory.json
    python3 scripts/sd32_t9_pi_review_spell.py fresh_inventory.json \\
        --corpus-root "$PCGEN_CORPUS_ROOT"

Method notes on the normalized scan (recorded so it is inspectable, not
just its output):

- **Case-fold only** catches `Cayden CaiLean` (mixed-case variant of
  `Cayden Cailean`) -- casefold makes both `cayden cailean`.
- **Case-fold alone does NOT catch `lrori`** -- casefold of `lrori` is
  still `lrori`, casefold of the blacklist term `Irori` is `irori`; the
  glyphs `l` (lowercase L) and `i` are different characters even after
  casefolding. A bounded OCR-confusion table collapsing `l`/`1` (both
  commonly OCR'd for lowercase `i` in scanned-PDF-derived text) to a
  single canonical character is required, and is applied to BOTH the row
  and the term before comparing.
- **`|` is deliberately excluded from the confusion table.** It is
  PCGen's own literal field/sub-value delimiter in these raw rows (e.g.
  `FACTSET:Deity|Cayden CaiLean`), not an OCR artifact of the prose.
  Folding it into `i` glues adjacent tokens together and was caught here
  producing a false NEGATIVE on the very `Cayden CaiLean` incident this
  scan exists to catch -- recorded so a future cycle does not re-add it.
- **Word-boundary matching, not bare substring, on the normalized text.**
  A short blacklist term case-folds into an ordinary English word: `Nex`
  (Golarion place name) is a substring of `next` once folded to
  lowercase. The *original* exact-substring scan is case-SENSITIVE, so
  `next` never hit `Nex` (capital N protected it) -- but a naive
  case-folded re-scan reopens that hole. This script confirmed the false
  positive on 5 spell records (`Quickened True Strike`,
  `True Skill`, `Violent Accident`, `Endothermic Touch`, `Spellsteal`,
  all via "...your **next** attack"/"...**next**..." style prose) before
  adding the boundary check, then re-ran clean. Left in this docstring
  because a future cycle re-implementing this scan from scratch is
  likely to hit the exact same false-positive class.
"""
from __future__ import annotations

import argparse
import importlib.util
import json
import re
import sys
from pathlib import Path

_HERE = Path(__file__).resolve().parent
_AUDIT_PATH = _HERE / "sd32_t9_pi_exposure_audit.py"
_spec = importlib.util.spec_from_file_location("sd32_t9_pi_exposure_audit", _AUDIT_PATH)
audit = importlib.util.module_from_spec(_spec)
assert _spec.loader is not None
_spec.loader.exec_module(audit)  # reuse its EVIDENCE_FAMILIES / PI_BLACKLIST_TERMS / classify_row / resolve_source_file / read_row verbatim

sys.path.insert(0, str(_HERE))
from pi_scrub import normalized_term_hits as _shared_normalized_term_hits  # noqa: E402

# ---------------------------------------------------------------------------
# Normalized (case-fold + bounded OCR-confusion + word-boundary) term scan
# ---------------------------------------------------------------------------
#
# `decisions.md §26`: this file previously carried its own `ocr_normalize`
# fold and its own bare-substring-then-word-boundary re-implementation of
# the scan (drifted from `sd32_t9_pi_review_feat_equipment.py`'s and
# `sd32_t9_pi_review_companion_monsterability.py`'s copies -- the exact
# duplication shape `decisions.md §17` names, and the live cause of the
# `bard_s_escape.json` false positive this cycle found: the drifted copy
# had no guard against the "Jarn"->"jam" OCR-fold collision with an
# ordinary English word). The fold + word-boundary matcher now lives in
# the shared `pi_scrub.normalized_term_hit`, imported above. This function
# keeps ONLY what is genuinely this call site's own: reporting hits that
# are NEW versus the exact-substring scan `sd32_t9_pi_exposure_audit.py`
# already ran, not every hit.


def normalized_term_hits(row: str) -> list[str]:
    """Blacklist terms hit under the normalized scan but NOT already an
    exact-substring hit -- i.e. genuinely NEW findings the production
    scan (and the audit's classify_row) would miss today."""
    exact_hits = {t for t in audit.PI_BLACKLIST_TERMS if t in row}
    return [t for t in _shared_normalized_term_hits(row) if t not in exact_hits]


# ---------------------------------------------------------------------------
# Per-record free-text proper-noun triage (spell kind only carries DESC:)
# ---------------------------------------------------------------------------

_FREE_TEXT_TAG_PREFIXES = ("DESC:", "BENEFIT:", "SPECIALS:", "SA:")

# Ordinary D&D/Pathfinder mechanical vocabulary, ability scores, skills,
# citation-footer tokens, and PCGen's own row-format leftovers that
# capitalize mid-sentence in this corpus's DESC: prose -- established by
# running this triage over the full 352-record uncertain set and reading
# every resulting word's context (see this cycle's memo §2 method note).
# Anything NOT on this list that word_re finds is a genuine candidate the
# per-record review must read by hand.
_COMMON_OK = {
    "You", "Your", "This", "The", "A", "An", "If", "When", "While", "After",
    "Before", "Since", "Once", "Whenever", "Anytime", "Additionally",
    "However", "Otherwise", "Although", "Regardless", "During", "Upon",
    "Every", "Each", "Any", "Anyone", "All", "These", "Those", "Such",
    "They", "Its", "For", "With", "Does", "Doing", "Ask", "Assume",
    "Choose", "Attempt", "Use", "Cause", "Create", "Creating", "Give",
    "Send", "Deal", "Deals", "Determine", "Divide", "Escape", "Find",
    "Fight", "Fighting", "Gain", "Grants", "Grapple", "Grappled", "Guide",
    "Hand", "Handle", "Heal", "Hit", "Immediately", "Induce", "Inspire",
    "Instantly", "Kill", "Land", "Local", "Make", "Meld", "Momentarily",
    "Multiple", "Open", "People", "Perceive", "Player's", "Protect",
    "Punish", "Randomly", "Recorporeal", "Research", "Respect", "Restrain",
    "Ride", "Rip", "Roll", "Rounds", "Seal", "Sense", "Share", "Slaughter",
    "Slowly", "Spell", "Spells", "Steal", "Succeeding", "Supresses",
    "Surroundings", "Target", "Target's", "Targets", "Temporarily",
    "Transform", "Transforming", "Trick", "Trust", "Turn", "Under",
    "Unattended", "Undead", "Wish", "Wood", "Ward", "True", "Triggered",
    "Threats", "Sleight", "Slough", "Sightless", "Spined", "Spines",
    "Prehistoric", "Shark-Blooded", "Four-Armed", "Multiattack",
    "Multiweapon", "Control", "Curse", "Material", "Calls", "Call",
    "Improved", "Distort", "Disturb", "Dismissing", "Changing", "Bizarre",
    "Bleeding", "Blind", "Bluff", "Carve", "Caster", "CASTERLEVEL",
    "DisplayFullSpell", "PRERULE", "Dice", "Dimension", "Effect",
    "Eliminate", "Emotion", "Enlarge", "Erupting", "Fahrenheit",
    "Failure", "Fear", "Fleeting", "Floating", "Focus", "Force", "Four",
    "Graft", "Hate", "Hellish", "Holy", "Its", "Lust", "Motive",
    "Painfully", "Partially", "Plane", "Quieted", "Shadow", "Since",
    "Cinders", "Dreams", "Creature", "Creatures", "Summoned",
    "Ability", "Advanced", "Affected", "Animal", "Animate", "Anoint",
    "Armed", "Artist", "Blooded", "Cone", "Core", "Rulebook",
    "Pathfinder", "RPG", "Bestiary", "Intrigue", "Abyss", "Abyssal",
    "Acrobatics", "Charisma", "Dexterity", "Constitution", "Intelligence",
    "Strength", "Stealth", "Diplomacy", "Intimidate", "Knowledge",
    "Survival", "Disguise", "CMB", "CMD", "Small", "Medium", "Large",
    "Huge", "Gargantuan", "Colossal", "Tiny", "Diminutive", "Fine",
    "Fortitude", "Reflex", "Will",
    "Climb", "Drain", "Unholy", "Ultimate", "Perception", "Strands",
    "Shark", "Dousing",
}

_ROMAN_NUMERAL_RE = re.compile(r"^M{0,4}(CM|CD|D?C{0,3})(XC|XL|L?X{0,3})(IX|IV|V?I{0,3})$")

_WORD_RE = re.compile(r"\b[A-Z][a-zA-Z']{2,}\b")


def extract_freetext(row: str) -> str:
    parts = []
    for tok in row.split("\t"):
        tok = tok.strip()
        for prefix in _FREE_TEXT_TAG_PREFIXES:
            if tok.upper().startswith(prefix) and len(tok) > len(prefix) + 3:
                parts.append(tok[len(prefix):])
    return " | ".join(parts)


def unlisted_proper_noun_candidates(freetext: str) -> list[str]:
    words = _WORD_RE.findall(freetext)
    return sorted(set(
        w for w in words
        if w not in _COMMON_OK and not _ROMAN_NUMERAL_RE.match(w)
    ))


# ---------------------------------------------------------------------------
# Driver
# ---------------------------------------------------------------------------

def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("inventory_json")
    ap.add_argument("--corpus-root", required=True)
    ap.add_argument("--json-out", default=None, help="full per-record spell review, written here")
    args = ap.parse_args()

    units = audit.load_units(args.inventory_json)
    t9 = audit.t9_units(units)
    spells = [u for u in t9 if u["kind"] == "spell"]
    print(f"spell kind, re-derived T9 population: {len(spells)}")

    index = audit.build_basename_index(args.corpus_root)

    records = []
    for u in spells:
        path, resolve_note = audit.resolve_source_file(index, u["book"], u["source_file"])
        if path is None:
            row = ""
            bucket, why = "uncertain", "source file not found"
        else:
            row = audit.read_row(path, u["source_line"])
            bucket, why = audit.classify_row(row)
        records.append({**u, "bucket": bucket, "classify_reason": why, "row": row, "resolve_note": resolve_note})

    by_bucket = {"blocked": [], "clear": [], "uncertain": []}
    for r in records:
        by_bucket[r["bucket"]].append(r)
    print(f"blocked={len(by_bucket['blocked'])} clear={len(by_bucket['clear'])} uncertain={len(by_bucket['uncertain'])}")

    # --- per-record review of the uncertain bucket -----------------------
    print()
    print("=== per-record review: uncertain bucket ===")
    review_blocked = []
    review_clear = []
    review_undecidable = []
    for r in by_bucket["uncertain"]:
        ft = extract_freetext(r["row"])
        suspects = unlisted_proper_noun_candidates(ft)
        norm_hits = normalized_term_hits(r["row"])
        if norm_hits:
            r["disposition"] = "blocked"
            r["disposition_reason"] = f"normalized-scan term hit not caught by exact scan: {norm_hits}"
            review_blocked.append(r)
        elif suspects:
            r["disposition"] = "still_undecidable"
            r["disposition_reason"] = f"unlisted candidate proper noun(s) in DESC: {suspects}"
            review_undecidable.append(r)
        else:
            r["disposition"] = "clear"
            r["disposition_reason"] = "DESC: read; generic OGL-shaped game-mechanic prose, no proper noun outside the 57-term blacklist found"
            review_clear.append(r)
    print(f"reviewed={len(by_bucket['uncertain'])} -> blocked={len(review_blocked)} clear={len(review_clear)} still_undecidable={len(review_undecidable)}")
    for r in review_undecidable:
        print(f"  STILL_UNDECIDABLE: {r['book']}:{r['name']} -- {r['disposition_reason']}")
    for r in review_blocked:
        print(f"  BLOCKED (normalized): {r['book']}:{r['name']} -- {r['disposition_reason']}")

    # --- clear-bucket recheck (normalized scan only) ----------------------
    print()
    print("=== clear-bucket recheck (normalized scan) ===")
    clear_newly_blocked = []
    for r in by_bucket["clear"]:
        norm_hits = normalized_term_hits(r["row"])
        if norm_hits:
            clear_newly_blocked.append((r, norm_hits))
    print(f"clear bucket rechecked={len(by_bucket['clear'])} newly_blocked={len(clear_newly_blocked)}")
    for r, hits in clear_newly_blocked:
        print(f"  NEWLY_BLOCKED: {r['book']}:{r['name']} -- {hits}")

    # --- .MOD/.COPY question -----------------------------------------------
    modcopy = [r for r in spells if ".MOD" in r["name"] or ".COPY" in r["name"] or ".FORGET" in r["name"]]
    print()
    print(f".MOD/.COPY-shaped spell units: {len(modcopy)}")

    if args.json_out:
        out = {
            "blocked": by_bucket["blocked"],
            "clear_original": by_bucket["clear"],
            "uncertain_reviewed": {
                "blocked": review_blocked,
                "clear": review_clear,
                "still_undecidable": review_undecidable,
            },
            "clear_bucket_recheck_newly_blocked": [r for r, _ in clear_newly_blocked],
        }
        Path(args.json_out).write_text(json.dumps(out, indent=1))
        print(f"\nFull review written to {args.json_out}")


if __name__ == "__main__":
    main()
