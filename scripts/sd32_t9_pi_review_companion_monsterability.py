#!/usr/bin/env python3
"""SD-32 card 11, T9 -- per-record PI review of the `companion` and
`monster_ability` uncertain buckets (`decisions.md §18`).

**Read-only.** Transcribes nothing, ingests nothing, changes no corpus
data. Does not amend `docs/governance/ogl-pi-blacklist.md` (status stays
`DRAFT`). This script's output is evidence for the operator's decision on
`decisions.md §18`, not a decision itself.

**Extends, does not redo,** `scripts/sd32_t9_pi_exposure_audit.py`. Reuses
that script's population filter, term list, and free-text-tag heuristic
verbatim (imported, not re-typed) so the two scripts cannot drift on what
counts as "uncertain" in the first place. What this script adds on top:

1. A **normalized re-scan** (case-fold + a small OCR-confusion table for
   the error class `ogl-pi-blacklist.md §4` recorded: `l`/`I`/`1`/`!`
   folded together, `0`/`o` folded together, `rn` folded to `m`) run
   against BOTH the `clear` and `uncertain` buckets for `companion` and
   `monster_ability` -- the exact re-check `decisions.md §18` item 2
   calls for. A normalized hit that the exact-substring scan missed is
   reported as `newly_blocked` (if it was `clear`) or left in
   `uncertain` (if it was already there) with the hit recorded.
2. A **per-record content classifier** for the surviving uncertain rows,
   applied to the row's actual `DESC:`/`SPECIALS:`/`SA:`/`BENEFIT:` prose
   (PCGen `%N` substitution-variable suffixes stripped before scanning --
   those are template variable names, not prose, and were inflating an
   earlier naive capitalized-word scan with noise like
   `BreathWeaponConeAcidTimes`):

   - **clear**: the prose contains no capitalized proper-noun-shaped
     token outside a curated allowlist of generic game/anatomy/mechanic
     words, AND no lowercase creature-species reference is present (see
     below) -- i.e. the row is pure game mechanic (dice, numbers, stock
     SRD-named spells/feats/skills), the OGL §1(d)/(e) mechanic
     exclusion.
   - **still_undecidable**: the prose names a specific creature species
     (a lowercase `a/an <word>` pattern whose head noun is not a generic
     term) -- e.g. "a jinushigami wields...", "a bandersnatch can move
     ...". Whether an individual Paizo-original bestiary creature's own
     name is itself Product Identity is a legal judgment this script
     cannot make (some derive from public-domain folklore, e.g.
     "bandersnatch" from Carroll; others are Paizo-original coinages);
     flagging it for a human reader is the honest disposition per the
     blacklist's own DRAFT banner ("stop and ask ... rather than
     guessing"). Also still_undecidable: any row with a capitalized token
     outside the allowlist that isn't resolved by the normalized-term
     re-scan.
   - **blocked**: only via the normalized re-scan hit (item 1).

This script does NOT expand `PI_BLACKLIST_TERMS` itself and does NOT
decide that any creature-species name is or is not PI -- see the memo
this script's output feeds
(`artifacts/gate-3-closure-invariant/t9-pi-review-companion-monsterability.md`)
for the operator-facing proposed rule.

Re-derive:
    cargo build --locked --release --bin v06_work_inventory
    PCGEN_CORPUS_ROOT=<repo>/.../pcgen/data \\
        <target>/release/v06_work_inventory --stdout-only > fresh_inventory.json
    python3 scripts/sd32_t9_pi_review_companion_monsterability.py fresh_inventory.json \\
        --corpus-root <repo>/.../pcgen/data --json-out review_out.json
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from collections import defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import sd32_t9_pi_exposure_audit as audit  # noqa: E402  (reuse, not re-type)
from pi_scrub import normalized_term_hit  # noqa: E402

KINDS_IN_SCOPE = ("companion", "monster_ability")

# --- normalized re-scan (decisions.md §18 item 2 / §4's recorded incident) ---
#
# `decisions.md §26`: the fold + word-boundary matcher itself
# (`canonicalize`/`normalized_term_hit`, formerly this file's own local
# `canonicalize`/`_CANON_TERMS`/loop) moved to the shared `pi_scrub.py` --
# this file no longer defines its own copy. `normalized_scan` below keeps
# ONLY the narrowing that is genuinely this call site's own: scoping the
# scan to prose tags, never the whole raw row.


def normalized_scan(free_text: str) -> str | None:
    """Return the first blacklist term whose canonicalized form appears,
    word-bounded, in the canonicalized free-text PROSE (DESC/SPECIALS/SA/
    BENEFIT values only -- NOT the raw row). Two deliberate narrowings
    versus a naive whole-row substring scan, both found necessary by
    running this exact check first and reading what it hit:

    1. **Scoped to prose, not the whole row.** The whole raw PCGen row
       also contains `KEY:`/`DEFINE:`/`BONUS:` camelCase variable-name
       soup (e.g. `SmiteEvilDamageBonus`) that concatenates unrelated
       words with no space -- scanning it produced a false "Geb" hit
       inside `...damageBonus` (`d-a-m-a-g-E-b-o-n-u-s` contains `geb`
       after fold) and a false "Nex" hit inside similar identifiers
       elsewhere. PI (per OGL §1(e)) lives in flavour prose, not in a
       schema's own internal variable names -- scoping to the free-text
       tags is the correct fix, not a narrowing for convenience.
    2. **Word-bounded, not raw substring.** Even scoped to prose, `Nex`
       and `Geb` (3-letter blacklist terms) are substrings of ordinary
       English words ("next", "together" is fine but e.g. a var-free
       sentence could still contain "nexus", "Geb" inside "budgeted", ...).
       A word-boundary regex match on the canonicalized text is the
       correct standard here -- it is what already protects the exact
       scan in `sd32_t9_pi_exposure_audit.py` for THIS reason: that
       script's `PI_BLACKLIST_TERMS` substring check on the raw row is
       itself exposed to the identical whole-word-vs-substring risk for
       short terms, which this review surfaces as its own finding (see
       memo) rather than silently working around only in the new script.
       Word-boundary matching alone is not always sufficient -- see
       `pi_scrub.py`'s own `_RN_FOLD_EXEMPT_TERMS_CASEFOLD` for the
       "Jarn"/"jam" collision `decisions.md §26` found and fixed, which
       survives word-boundary matching because the fold-collision IS a
       whole word.

    Narrowing 2 (word-boundary matching, plus the `§26` "Jarn"/"jam" fold
    guard) now lives in the shared `pi_scrub.normalized_term_hit`, imported
    above. This function keeps ONLY narrowing 1 (the prose-scoping) --
    callers pass ALREADY-extracted free text, never a raw row.
    """
    return normalized_term_hit(free_text)


# --- per-record content classifier for surviving `uncertain` rows ---

_GENERIC_CAPWORDS = set(
    """
    The A An You Your It Its This That These Those Add As At In On Of For With
    Reflex Fortitude Will Armor Class Dexterity Strength Constitution Intelligence Wisdom Charisma
    DC Str Dex Con Int Wis Cha AC CMB CMD HP DR SR CR CL
    Bluff Disguise Stealth Perception Acrobatics Climb Swim Fly Ride Diplomacy Intimidate
    Sleight Hand Escape Artist Knowledge Handle Animal Survival Craft Perform Profession
    Sense Motive Disable Device
    Fire Cold Acid Electricity Sonic Magic Spell Cone Line Reach Poison Injury
    Small Large Medium Tiny Huge Gargantuan Colossal Fine Diminutive
    Standard Move Full Swift Immediate Free Round Action Level Bonus Penalty Damage
    Attack Attacks All Whenever Each One Two Three Four Five Once Any Some
    Eidolon Eidolons Companion Companions Master Summoner Fort Binder
    Special Ability Abilities Evolution Evolutions Choose Training Training's
    Hit Dice Die Combat Maneuver Defense Improved Greater Normal Feats Feat
    Spells Spell-Like Constant Undead Negative Energy Command Although Likewise
    Bodyguard Patrol Heroic Defiance Recovery Way Harm's In
    Rush Overrun Reposition Sunder Trip Steal Bull Dirty Trick
    Alertness Mobility Toughness Multiattack
    Because Otherwise Alternatively Additionally Finally Since Though Whether Until
    Effects Targets Increase No Not Only Anyone Even Yet There's Bigger Some Like
    Nimble Moves Wind Stance Lightning Reflexes Charge Through Spring
    First World They Draconic Precocious Racer Tracker Wrecker Bully Daredevil
    Sage Sages Figment Figments Mascot Protector Pilferer Prankster Infiltrator
    Ambusher Ambushers Auspice Auspices Verdant Aberrant Augmented Deathtouched Feytouched
    Soulbound Plant Aligned Camouflage Bestiary GM's Beast Speech Photosynthesis
    Persuasive Nimble's
    """.split()
) | {
    # --- decisions.md §19c widening (2026-08-23), named by category ---
    # (4) SRD-open spell names cited by "Imp Companion Trick" (book_of_the_damned_volume_1)
    #     rows that grant the imp a spell-like use of a core spell -- OGL per §2.2's own
    #     "spell level names" exclusion; "Imp" itself is the classic SRD-open devil monster
    #     name (same posture as Bestiary 1's "Owlbear"/"Goblin", ogl-pi-blacklist.md §2.1).
    "Detect", "Evil", "Law", "Doom", "Ghost", "Sound", "Mage", "Message", "Close", "Open",
    "Prestidigitation", "Curse", "Water", "Disk", "Floating", "Grease", "Hold", "Portal",
    "Identify", "Image", "Silent", "Servant", "Unseen", "Ventriloquism", "Bleed",
    "Deathwatch", "Imp",
    # (5) feat/ability names, page-citation and PCGen-boilerplate tokens read in full
    #     row context -- none is a Golarion proper noun:
    "Chapter", "Blow", "Intercept", "Granted", "Disruptive", "Antagonize", "Ferocious",
    "Intimidating", "Prowess", "Hunter", "Tenacious", "Harvesting", "Poisons", "Expertise",
    "Power", "Familiar", "Heal", "Core", "Pathfinder", "RPG", "Rulebook", "Acrobatic",
    "Steps", "CHANGES", "RANK", "Focus", "APG", "NOT", "IMPLEMENTED",
    # (6) equipment materials named explicitly in decisions.md §19c -- SRD-open crafting
    #     materials, not PI:
    "Adamantine", "Mithral",
    # --- SD-32 T9-onboarding-cause-closure widening (2026-08-23, decisions.md §20/§19c) ---
    # Re-derived the companion `no_record` residual (217, `scripts/shape_ledger.py`) against
    # `ingest_companion.py`'s own 217 `still_undecidable` skip-list and read every flagged
    # capitalized token in full row context. All are ordinary anatomy/ability-tag/PCGen-
    # boilerplate words, none a setting-specific proper noun:
    "Bite", "Claws", "Ex", "Using", "When", "Skill", "Eldritch", "Cooperative", "Crafting",
    # --- decisions.md §27b — EVERYTHING, resolved 2026-08-23 ---
    # "Shaitan" -- was left deliberately OFF this allowlist pending operator review
    # (`advanced_race_guide:Stone Curse`'s PRERACE field, RACETYPE=Shaitan Binder
    # Eidolon). The operator spot-check RESOLVED it: `t9-pi-review-companion-
    # monsterability.md` §7, `advanced_race_guide:Earth Glide (Shaitan Binder
    # Eidolon) — clear. "Shaitan" here is the genie-subtype term from the core
    # Bestiary's elemental taxonomy (djinn/efreeti/shaitan/marid), not a
    # Golarion-specific name.` `scripts/pi_scrub.py`'s own canonical blacklist scan
    # independently confirms zero hits on either affected row's full text. This was
    # the only capitalized-token holdout in the companion residual; allowlisting it
    # closes the last 2 `companion` `no_record` units (`decisions.md §20`/`§27b`).
    "Shaitan",
}

# lowercase words that precede a creature-species mention but are not
# themselves species names (generic role/anatomy/mechanic nouns) -- used
# to suppress false positives in the `a/an <noun>` species-reference scan.
_GENERIC_LOWER_NOUNS = set(
    """
    action attack bonus check creature target foe weapon effect ability
    round day week standard swift move full free save roll point die dice
    item object structure trick command signal alarm area campsite
    companion familiar animal master handler owner rider mount
    spell-like spell feat skill save class level score type subtype
    example instance case result condition state form shape size
    breath weapon cone line burst radius square feet foot inch
    number amount duration range effect source target user wielder
    creature's animal's companion's master's handler's
    """.split()
) | {
    # --- decisions.md §19c widening (2026-08-23), named by category, verified against
    # full row context (scripts/tests/test_sd32_t9_pi_normalization_and_inheritance.py
    # does not re-derive this list mechanically -- it is a documented human read, per
    # the sign-off package §4.2's own recommendation "(b) a human reads the flagged
    # subset directly") ---
    # (1) core save/ability-score/movement/class mechanic vocabulary (OGL §2.2, already
    #     game-mechanical), not proper nouns:
    "reflex", "swim", "eidolon", "intelligence", "undead", "magus", "draconic", "nonanimal",
    # (2) published PF1e Familiar/Companion Archetype names (Ultimate Wilderness) -- OGL
    #     mechanic subclass names, not Golarion place/deity/NPC proper nouns:
    "ambassador", "bodyguard", "daredevil", "egotist", "emissary", "infiltrator", "mascot",
    "mauler", "pilferer", "prankster", "protector", "totem", "valet",
    # (3) ordinary English function/qualifier words the "a/an/the <noun>" species-reference
    #     heuristic false-positives on -- read in full row context, none is a species name:
    "successful", "purpose", "long", "opponent", "tricks", "black", "pair", "bull",
    "additional", "same", "total", "chosen", "single", "start", "particular", "arcane",
    "different", "time", "augmented", "gore", "skilled", "climb", "normal", "overrun",
    "aberrant", "deathtouched", "feytouched", "verdant", "combat-trained", "full-round",
    "following", "specific", "saving", "effects", "armor", "touch", "bite", "spells",
    # --- SD-32 T9-onboarding-cause-closure widening (2026-08-23, decisions.md §20/§19c) ---
    # Re-derived the companion `no_record` residual (217, `scripts/shape_ledger.py`
    # `--inventory docs/work-inventory.json`) against `ingest_companion.py`'s own 217
    # `still_undecidable` `pi_skipped_records` and read every flagged lowercase term in
    # full row context. Categories, named per decisions.md §19c's binding condition:
    # (a) combat/mechanic action vocabulary -- not species names:
    "charge", "damage", "link", "benefits", "benefit", "handle", "prerequisite",
    "prerequisites", "attacker", "attacking", "primary", "second", "melee", "natural",
    "reposition", "sunder", "trip", "disarm", "drag", "grapple", "steal", "defend", "turn",
    "withdraw", "intercepted", "unarmed", "ranged", "combat",
    # (b) ability-score/save/skill/class vocabulary (core OGL mechanic terms, incl. core
    #     PF1e class names -- cleric/druid/wizard/rogue/fighter/shaman/witch are all
    #     open-content base or APG classes, not Paizo-original proper nouns):
    "charisma", "constitution", "fortitude", "cleric", "outsider", "druid", "wizard",
    "rogue", "fighter", "shaman", "witch", "magical", "resistance", "immunity",
    # (c) anatomy/creature-part vocabulary, generic across any species, not a species
    #     name itself:
    "sting", "tail", "tentacle", "wing", "claws",
    # (d) generic-animal words that are common English nouns, not setting-specific
    #     proper nouns (same posture as "Owlbear"/"Goblin", ogl-pi-blacklist.md §2.1):
    "boar", "raven", "porcupine", "shadow", "shadows", "ghost",
    # (e) magic/spellcraft mechanic vocabulary:
    "incorporeal", "supernatural", "scrying", "divination", "nondetection", "antimagic",
    "polymorph", "spelllike",
    # (f) ordinary adjectives/adverbs/connectors the "a/an/the <noun>" heuristic
    #     false-positives on -- read in full row context, none is a species name:
    "at-will", "selected", "devastating", "surface", "usual", "invisible", "medium",
    "light", "common", "caster", "living", "basic", "major", "minor", "powerful",
    "improved", "typical", "surprise", "adjacent", "path", "list", "other", "base",
    "next", "ally", "character", "poison", "deity", "spark", "negative", "giant",
    "competence", "divine", "team", "grab", "barrier", "stone", "victim", "costly",
    "past", "wish", "words", "young", "quickened", "forced", "official", "threatened",
    "off-hand", "two-handed", "heavy", "jump", "running", "shield", "requisite", "fear",
    "like", "random", "variety", "scent", "conversing", "communication", "blade",
    "language", "will", "broken", "ritual", "special", "universal", "environment",
    "space", "beginning", "unnatural", "enemy", "concentration", "disruptive",
    "antagonize", "onset", "affliction", "harmful", "results", "archetype",
    "teleportation", "wild", "designated", "flying", "specified", "latch", "person",
    "unattended", "bury", "cocoon", "escape", "helpless", "place", "smells", "perform",
    "guide", "guided", "locations", "presence", "dirty", "hostile-seeming", "menacing",
    "venom", "vial", "disguise", "mundane", "observer", "opposed", "ordinary",
    "recipient", "creatures", "event", "nearest", "safe", "silk", "separate", "soil",
    "injury", "loss", "birthmark", "danger", "fray", "partially", "strange", "trace",
    "faint", "first", "primal", "wider", "abilities", "advantage", "tendency",
    "variable", "failed", "failure", "mind-affecting", "original", "success", "loyalty",
    "paragon", "real", "relationship-the", "throne", "font", "moral", "subtle", "very",
    "adventuring", "centerpiece", "entire", "bloodthirsty", "thrill", "tumor", "soul",
    "consummate", "ball", "permanent", "course", "heart", "larger", "constant",
    "bluff", "false", "hunter", "precocious", "unlearned", "actual", "finer", "subject",
    "fragment", "tracker", "harmless", "willing", "teamwork", "sunlight", "construct",
    "inanimate", "empathic", "diabolist", "auspice", "bully", "racer", "sage",
    "alertness", "soulbound", "figment", "wrecker", "ambusher",
    # (g) second pass, verified against a live `ingest_companion.py --dry-run` re-run
    #     (not just the earlier static skip-list read) -- same posture, ordinary
    #     mechanic/anatomy/adjective words, not species names:
    "acrobatics", "mile", "water", "blindsight", "human", "stealth", "purposes",
    "perception", "stimulus", "tenacious", "immediate", "strength", "last", "trainer",
    "plant", "wisdom", "valid", "break", "focus", "cooperative",
    # `decisions.md §27b` — resolved 2026-08-23, same operator ruling the
    # `_GENERIC_CAPWORDS` comment above cites: "shaitan" is a generic genie-subtype
    # term (djinn/efreeti/shaitan/marid), not a Golarion-specific proper noun.
    "shaitan",
    # `decisions.md §27b`, same closure: `advanced_race_guide:Earth Glide`'s own DESC
    # ("A burrowing Shaitan Binder Eidolon can pass through dirt, gravel, or other
    # loose or porous solid matter as easily as a fish swims through water...") trips
    # the `a/an/the <noun>` species-reference heuristic on three ordinary English
    # words, none a species name -- the exact false-positive class this file's other
    # widenings already document and correct. `scripts/pi_scrub.py`'s canonical
    # blacklist scan independently confirms zero hits on this row's full DESC text.
    "burrowing", "fish", "solid",
}


def strip_pcgen_vars(value: str) -> str:
    """DESC:text|Var1|Var2 -- the %N substitution vars after the first
    `|` are PCGen variable names (CamelCase identifiers), not prose."""
    return value.split("|", 1)[0]


def extract_free_text(row: str) -> str:
    parts = []
    for tok in row.split("\t"):
        tok = tok.strip()
        for prefix in audit.FREE_TEXT_TAG_PREFIXES:
            if tok.upper().startswith(prefix):
                parts.append(strip_pcgen_vars(tok[len(prefix):]))
    return " ".join(parts).strip()


_CAPWORD_RE = re.compile(r"\b[A-Z][a-zA-Z']+\b")
_SPECIES_REF_RE = re.compile(r"\b(?:a|an|the)\s+([a-z][a-z\-]{3,})\b")
# Roman numerals (spell-level suffixes: "beast shape III", "summon monster IX") are not
# proper nouns -- decisions.md §19c widening, same fix `sd32_t9_pi_review_spell.py`
# already applies for the `spell` kind.
_ROMAN_NUMERAL_ONLY_RE = re.compile(r"^M{0,4}(CM|CD|D?C{0,3})(XC|XL|L?X{0,3})(IX|IV|V?I{0,3})$")
# Sentences within one DESC/SPECIALS value are period/!/?-delimited; the
# FIRST word of the whole text and of every sentence after one of these is
# capitalized by ordinary English convention and is not evidence of a
# proper noun. Split on sentence boundaries and drop each sentence's first
# word before scanning for capitalization -- without this, "Except where
# noted...", "Fast healing...", "Unless otherwise stated..." (all ordinary
# sentence-initial capitals in ability rules text) were misread as
# proper-noun candidates, a false-positive class this review caught and
# fixed by reading actual flagged rows (see memo §method note).
_SENTENCE_SPLIT_RE = re.compile(r"(?<=[.!?])\s+")


def _non_initial_words(text: str) -> list[str]:
    out = []
    for sentence in _SENTENCE_SPLIT_RE.split(text):
        words = sentence.split()
        out.extend(words[1:])  # drop the sentence-initial word
    return out


def classify_uncertain_content(free_text: str) -> tuple[str, str]:
    """(disposition, reason) for a row already in the `uncertain` bucket,
    given its free text with PCGen vars stripped and no normalized
    blacklist-term hit. `still_undecidable` is a legitimate first-class
    result, not a fallback to avoid."""
    if not free_text.strip():
        return "still_undecidable", "free-text tag present but resolved value empty -- cannot read content"

    scan_text = " ".join(_non_initial_words(free_text))
    capwords = [
        w for w in _CAPWORD_RE.findall(scan_text)
        if w.strip("'") not in _GENERIC_CAPWORDS and not _ROMAN_NUMERAL_ONLY_RE.match(w)
    ]
    if capwords:
        return "still_undecidable", f"capitalized token(s) outside generic allowlist: {sorted(set(capwords))[:6]}"

    species_hits = []
    for m in _SPECIES_REF_RE.finditer(free_text.lower()):
        noun = m.group(1)
        if noun not in _GENERIC_LOWER_NOUNS and len(noun) >= 4:
            species_hits.append(noun)
    if species_hits:
        return "still_undecidable", f"lowercase creature-species-shaped reference(s): {sorted(set(species_hits))[:6]}"

    return "clear", "no proper-noun-shaped token, no creature-species reference -- pure game mechanic (§2.2)"


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("inventory_json")
    ap.add_argument("--corpus-root", required=True)
    ap.add_argument("--json-out", default=None)
    args = ap.parse_args()

    units = audit.load_units(args.inventory_json)
    t9 = audit.t9_units(units)
    index = audit.build_basename_index(args.corpus_root)

    rows_by_unit = []
    for u in t9:
        if u["kind"] not in KINDS_IN_SCOPE:
            continue
        path, reason = audit.resolve_source_file(index, u["book"], u["source_file"])
        if path is None:
            rows_by_unit.append({**u, "row": "", "resolve_note": "not_found"})
            continue
        row = audit.read_row(path, u["source_line"])
        rows_by_unit.append({**u, "row": row, "resolved_path": path, "resolve_note": reason})

    results = []
    newly_blocked = []
    newly_uncertain = []
    still_undecidable_examples = defaultdict(list)
    clear_examples = defaultdict(list)
    blocked_examples = defaultdict(list)

    for u in rows_by_unit:
        row = u["row"]
        exact_bucket, exact_reason = audit.classify_row(row)
        free_text_for_scan = extract_free_text(row)
        norm_hit = normalized_scan(free_text_for_scan)

        if exact_bucket == "blocked":
            final_bucket, final_reason = "blocked", exact_reason
        elif norm_hit is not None:
            final_bucket = "blocked"
            final_reason = f'normalized-scan hit (case-fold + OCR-fold): "{norm_hit}"'
            if exact_bucket == "clear":
                newly_blocked.append({**u, "reason": final_reason})
            else:
                newly_blocked.append({**u, "reason": final_reason, "was": "uncertain"})
        elif exact_bucket == "uncertain" and u["kind"] == "monster_ability":
            # decisions.md §19b (operator ruling, 2026-08-23): a monster_ability row
            # carrying no PI declaration and no term-list hit is not PI merely because
            # its text names a Paizo-original creature -- the row's own declaration
            # governs, not the content classifier's embedded-creature-name heuristic.
            # This resolves the 954-unit "embedded-creature-name problem" (sign-off
            # package §4.1) to `clear`, superseding this script's own prior
            # `classify_uncertain_content` disposition for this kind only (`companion`
            # is unaffected -- §19c's allowlist widening, not §19b's declaration rule,
            # governs that kind).
            final_bucket = "clear"
            final_reason = (
                "decisions.md §19b: monster_ability row's own PCGen declaration governs; "
                "no NAMEISPI/DESCISPI declaration and no term-list hit found -- not PI by "
                "association with an embedded creature name alone"
            )
        elif exact_bucket == "uncertain":
            content_bucket, content_reason = classify_uncertain_content(free_text_for_scan)
            final_bucket, final_reason = content_bucket, content_reason
            if content_bucket == "still_undecidable":
                # not a bucket change from the audit's own "uncertain" --
                # tracked separately below as "still not resolved", not
                # reported as newly_uncertain (that's for clear->uncertain
                # moves only, per the dispatch brief's ask).
                pass
        else:  # exact_bucket == "clear"
            final_bucket, final_reason = "clear", exact_reason

        rec = {**u, "exact_bucket": exact_bucket, "final_bucket": final_bucket, "final_reason": final_reason}
        results.append(rec)

        label = f"{rec['book']}:{rec['kind']}:{rec['name']}"
        if final_bucket == "blocked" and len(blocked_examples[rec["kind"]]) < 10:
            blocked_examples[rec["kind"]].append(f"{label} ({final_reason})")
        if final_bucket == "still_undecidable" and len(still_undecidable_examples[rec["kind"]]) < 10:
            still_undecidable_examples[rec["kind"]].append(f"{label} ({final_reason})")
        if final_bucket == "clear" and exact_bucket == "uncertain" and len(clear_examples[rec["kind"]]) < 10:
            clear_examples[rec["kind"]].append(f"{label} ({final_reason})")

    print("=== per-record review: companion + monster_ability ===")
    by_kind_final = defaultdict(lambda: defaultdict(int))
    by_kind_exact = defaultdict(lambda: defaultdict(int))
    for r in results:
        by_kind_final[r["kind"]][r["final_bucket"]] += 1
        by_kind_exact[r["kind"]][r["exact_bucket"]] += 1

    for k in KINDS_IN_SCOPE:
        print(f"\n-- {k} --")
        print(f"  audit-script (exact scan) buckets: {dict(by_kind_exact[k])}")
        print(f"  this review's final buckets:       {dict(by_kind_final[k])}")

    print(f"\nTotal newly_blocked (normalized scan hit, exact scan missed): {len(newly_blocked)}")
    for nb in newly_blocked[:20]:
        print(f"  {nb['book']}:{nb['kind']}:{nb['name']} -- {nb['reason']} (was {nb.get('was','clear')})")

    print("\n=== still_undecidable examples (up to 10 per kind) ===")
    for k in KINDS_IN_SCOPE:
        print(f"-- {k} --")
        for e in still_undecidable_examples[k]:
            print(f"  {e}")

    print("\n=== newly-clear examples (uncertain -> clear this review, up to 10 per kind) ===")
    for k in KINDS_IN_SCOPE:
        print(f"-- {k} --")
        for e in clear_examples[k]:
            print(f"  {e}")

    print("\n=== blocked examples (up to 10 per kind, includes normalized-scan finds) ===")
    for k in KINDS_IN_SCOPE:
        print(f"-- {k} --")
        for e in blocked_examples[k]:
            print(f"  {e}")

    if args.json_out:
        with open(args.json_out, "w", encoding="utf-8") as f:
            json.dump(results, f, indent=2)
        print(f"\nFull per-unit review written to {args.json_out}")


if __name__ == "__main__":
    main()
