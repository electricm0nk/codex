#!/usr/bin/env python3
"""SD-32 T9-onboarding-cause-closure — the ONE shared implementation of
`scrub_name_pi_tokens`, extracted the way `scripts/codex_neutral_name.py`
was extracted, after two independently-maintained copies (`ingest_ability.py`,
`ingest_generic_kind.py`) drifted and shipped a real leak.

**The leak this fixes (found live, corpus-wide cross-check).**
`ingest_generic_kind.py`'s copy already added an alphanumeric-normalized
substring check for a record's OWN name/key being concatenated into another
token's value with no separator (`RedMantisAssassinLVL`), because PCGen's
`DEFINE`/`BONUS` tokens frequently build a variable identifier that way. The
`ingest_ability.py` copy this was forked from was **never updated** with that
fix, and 576 already-shipped `ability` records went through the unfixed copy.

**A second, more severe instance of the same defect class was found doing
the honest re-derivation this cycle was scoped to do (`decisions.md §17a`):
the 60-term `PI_BLACKLIST_TERMS` scan (`normalized_term_hit`) has the
IDENTICAL word-boundary blind spot for a BLACKLISTED term (not just the
record's own name) concatenated PascalCase-style into another token's value**
— e.g. a `TYPE:` token naming a choice-ability variable by concatenating a
deity's name directly onto a suffix with no separator. `normalized_term_hit`
requires `(?<![a-z0-9])term(?![a-z0-9])`, which is deliberately word-bounded
to avoid a short-blacklist-term-inside-an-ordinary-word false-positive
`ogl-pi-blacklist.md §4` already recorded — but that same boundary rule means a value like
`"<Deity>AspectChoice"` never matches, because the character immediately
after the deity's name is a letter, not a boundary. This was found live in
an ALREADY-SHIPPED, ALREADY-RENAMED `ability` record (a §24 `codex_named_unit_*`
file) whose `TYPE` token still carried the deity's name in plain text despite
the record's `NAME`/`DESC` having been correctly redacted — the exact shape
`decisions.md §24` exists to prevent (`§24b`-2: "the PI original appears
nowhere that ships"). This module fixes BOTH shapes with the SAME
alphanumeric-normalized substring technique, bounded to needles/terms of at
least 6 normalized characters so a short, generic string (an abbreviation, a
3-4-letter blacklist term already covered separately by the word-bounded
scan) cannot over-redact by coincidence.

**Both checks are additive, never a replacement.** The original
space-preserving substring check and the original word-bounded
`normalized_term_hit` scan still run first; the alphanumeric-normalized scan
only adds coverage for the concatenated-identifier shape neither of those two
can see.

Every ingest path that redacts a record's raw tokens for Product Identity
must import `scrub_name_pi_tokens` from HERE — never re-define it locally.
`decisions.md §17` names this exact duplication-drift shape as the failure
`§24b`'s screen must not repeat.

**`decisions.md §26` (2026-08-23): this module is now also the ONE shared
home for `PI_BLACKLIST_TERMS`, `canonicalize`, and `normalized_term_hit`** —
the case-fold + bounded-OCR-confusion + word-boundary blacklist scan that
`decisions.md §19a` amendment 3b mandates. Before this cycle, three review
scripts (`sd32_t9_pi_review_companion_monsterability.py`,
`sd32_t9_pi_review_feat_equipment.py`, `sd32_t9_pi_review_spell.py`) plus
`sd32_t9_pi_exposure_audit.py` each carried an independent copy of the term
list and/or the fold function — a duplication-drift shape identical to the
one this module's `scrub_name_pi_tokens` was already extracted to fix. All
four now import from here; none re-defines it.
"""
from __future__ import annotations

import re

# `src/rules_core/shape_b_v1.rs::REDACTED_PI_MARKER` / `PI_MARKER_REDACTED` —
# the same literal every generator in this program uses.
REDACTED_PI_MARKER = "[redacted PI]"
PI_MARKER_REDACTED = "redacted"

# ---------------------------------------------------------------------------
# Shared blacklist term list + normalized (case-fold + bounded-OCR-confusion +
# word-boundary) scan. `decisions.md §19a` amendment 3b / `§26`.
# ---------------------------------------------------------------------------

# Script-side copy, byte-identical across the four call sites this module now
# unifies (verified before the merge: `sd32_t9_pi_exposure_audit.py` and
# `sd32_t9_pi_review_feat_equipment.py` both asserted `len == 60` over an
# identical ordered list). `ogl-pi-blacklist.md §2.3c`: this list used to lag
# the Rust production constant `src/rules_core/pi_screening.rs::
# PI_BLACKLIST_TERMS` by one term (`§12b`'s twin-implementation divergence,
# found by the `class_feature` lane and closed here, `decisions.md §12b`/
# `§20`): that copy's trailing per-book addition (`ogl-pi-blacklist.md`'s
# Inner Sea Gods equipment per-book-override, added by the
# `pi-key-rawtokens-screen` follow-up cycle) had been folded into the Rust
# copy only, deliberately deferred here because bumping the RUST list
# triggers corpus regeneration for the writer paths that import it — but this
# module is a READ-ONLY review/audit tool, not a generator, so folding the
# term in here carries none of that risk and closes the divergence rather
# than perpetuating it (`decisions.md §22`: an inherited inconsistency is
# ours to resolve). Re-scanned corpus-wide before folding in (both the
# shipped `data/corpus/**` and the pinned PCGen oracle, case-sensitive,
# lowercase-possessive form only — the properly-capitalized form is already
# caught by the base "Gozreh" entry at index 9): **zero** hits beyond the one
# `isg_equip.lst:232` leak the Rust-side addition's own verification already
# found and which is already redacted on disk
# (`data/corpus/inner_sea_gods/equipment/wayfinder_of_zephyrs.json`). Widening
# this list therefore changes no review-script output on the current corpus;
# it only prevents recurrence.
PI_BLACKLIST_TERMS = [
    "Iomedae", "Sarenrae", "Asmodeus", "Cayden Cailean", "Abadar", "Calistria", "Desna", "Erastil", "Gorum", "Gozreh",
    "Irori", "Lamashtu", "Nethys", "Norgorber", "Pharasma", "Rovagug", "Shelyn", "Torag", "Urgathoa", "Zon-Kuthon",
    "Golarion", "Absalom", "Cheliax", "Varisia", "Andoran", "Taldor", "Osirion", "Katapesh", "Ustalav", "Numeria",
    "Mwangi", "Tian Xia", "Avistan", "Garund", "Sarkoris", "Worldwound", "Vudra", "Kyonin", "Molthune", "Nidal",
    "Nirmathas", "Qadira", "Razmiran", "Rahadoum", "Galt", "Isger", "Lastwall", "Brevoy", "Druma", "Irrisen",
    "Jalmeray", "Thuvia", "Geb", "Nex",
    "Jarn",
    "Cayden CaiLean",
    "lrori",
    # decisions.md §19a amendment 3d (operator-approved 2026-08-23):
    "Aldori",
    "Magaambya",
    "Magaambyan",
    # `ogl-pi-blacklist.md`'s Inner Sea Gods equipment per-book-override
    # (`pi-key-rawtokens-screen` follow-up cycle, 2026-08-23): the pinned
    # oracle's own lowercase-possessive spelling of the deity name at index 9,
    # byte-identical to `pi_screening.rs::PI_BLACKLIST_TERMS`'s trailing entry.
    "gozreh's",
]
assert len(PI_BLACKLIST_TERMS) == 61, "term list drifted -- expected 57 + Aldori/Magaambya/Magaambyan + the ISG equipment lowercase-possessive addition (decisions.md §19a 3d, §12b)"

SOFT_HYPHEN = "­"

# decisions.md §19a amendment 3b, verbatim rule: case-fold + a BOUNDED
# OCR-confusion table (l/I/1/! -> one canonical char, 0/o collapsed, rn -> m),
# WORD-BOUNDARY matching (not bare substring). The PCGen field delimiter "|"
# is NEVER folded (folding it produces a false NEGATIVE on the Cayden
# CaiLean incident itself — confirmed by direct test).
_FOLD_TABLE = str.maketrans({"l": "i", "1": "i", "!": "i", "0": "o"})

# `decisions.md §26`: "Jarn" is the ONLY blacklist term containing the
# substring "rn", so it is the ONLY term the rn->m fold ever applies to
# today. Folded, "Jarn" canonicalizes to "jam" — an ordinary, extremely
# common English word that occurs constantly in genuine OGL prose ("...out
# of a tight jam...", by coordinate:
# `data/corpus/advanced_players_guide/spell/bard_s_escape.json`
# `data.raw_tokens[DESC]`, a `license: OGL` record with no PI content).
# Word-boundary matching does NOT prevent this: "jam" is itself a
# boundary-clean whole word, so the false positive survives the
# word-boundary guard that fixes the unrelated "Nex"/"next" class.
#
# "Jarn" has never been recorded as a scanned-OCR artifact (unlike
# Irori/lrori, Cayden Cailean/CaiLean, `ogl-pi-blacklist.md §4`) — it was
# found as a correctly, plainly spelled NPC name in ACG's own prose
# (`ogl-pi-blacklist.md §4`, ACG override entry). No known corpus
# occurrence depends on the rn->m fold to catch "Jarn"; disabling that one
# substitution for this one term therefore removes no proven real-OCR
# coverage. Case-fold and the l/1/!/0 fold still apply to "Jarn" — a
# plainly-spelled, mixed-case, or punctuation-noised occurrence still
# hits. Only the rn->m substitution is skipped, and ONLY when comparing
# against this specific term (the haystack's OWN rn->m fold is likewise
# skipped for this one comparison, so a literal, un-OCR'd "Jarn" in text
# still canonicalizes to "jarn", not "jam", and still matches).
_RN_FOLD_EXEMPT_TERMS_CASEFOLD = {"jarn"}

# `decisions.md §26`-adjacent (t9-onboarding cycle, 2026-08-23,
# corpus_literal_sweep unblock): the SAME false-positive class §26 fixed for
# the rn->m fold recurs for the l/1/!->i fold: "Galt" (a Golarion nation) is
# the only blacklist term containing "l", and its `_FOLD_TABLE` substitution
# canonicalizes it to "gait" — an ordinary English word ("his gait more
# deliberate...") that occurs in genuine OGL prose. Word-boundary matching
# does not help (as with Jarn/jam): "gait" is itself a boundary-clean whole
# word. Found live re-deriving `corpus_literal_sweep` against the pinned
# oracle: `advanced_players_guide/class_feature/shifter_s_blessing/
# form_of_the_cat.json`'s DESC, and three sibling `class_feature` records
# whose KEY/ABILITY token restates a "<Name>'s Gait"/"Steady Gait"-shaped
# ability name. "Galt" has never been recorded as a scanned-OCR artifact
# (unlike Irori/lrori, Cayden Cailean/CaiLean) -- it is always spelled
# correctly in the oracle wherever it is genuinely the nation, so exempting
# it from the l/1/!/0 fold removes no proven real-OCR coverage: a literal,
# correctly-spelled "Galt" still matches via case-fold alone (both sides of
# the comparison skip the SAME fold for this one term, symmetric with the
# rn-fold exemption above).
_CHAR_FOLD_EXEMPT_TERMS_CASEFOLD = {"galt"}


def canonicalize(s: str, *, apply_rn_fold: bool = True, apply_char_fold: bool = True) -> str:
    """Case-fold + bounded OCR-confusion fold. `apply_rn_fold=False` skips
    ONLY the rn->m substitution — used for the `_RN_FOLD_EXEMPT_TERMS_CASEFOLD`
    carve-out (`decisions.md §26`). `apply_char_fold=False` skips ONLY the
    `l`/`1`/`!`->`i`, `0`->`o` table — used for the `_CHAR_FOLD_EXEMPT_TERMS_CASEFOLD`
    carve-out (the Galt/gait collision, same class as §26's Jarn/jam). Every
    other fold still applies in both cases."""
    s = s.casefold().replace(SOFT_HYPHEN, "-")
    if apply_rn_fold:
        s = s.replace("rn", "m")
    if apply_char_fold:
        s = s.translate(_FOLD_TABLE)
    return s


def _term_needs_rn_fold(term: str) -> bool:
    return term.casefold() not in _RN_FOLD_EXEMPT_TERMS_CASEFOLD


def _term_needs_char_fold(term: str) -> bool:
    return term.casefold() not in _CHAR_FOLD_EXEMPT_TERMS_CASEFOLD


_CANON_TERMS: list[tuple[str, str, bool, bool]] = [
    (
        term,
        canonicalize(
            term,
            apply_rn_fold=_term_needs_rn_fold(term),
            apply_char_fold=_term_needs_char_fold(term),
        ),
        _term_needs_rn_fold(term),
        _term_needs_char_fold(term),
    )
    for term in PI_BLACKLIST_TERMS
]


def normalized_term_hit(free_text: str) -> str | None:
    """First blacklist term whose canonicalized form appears, WORD-BOUNDED, in
    the canonicalized free text. Word-boundary matching is mandatory, not
    optional: a case-fold-only scan without it collides the 3-letter term
    "Nex" with the ordinary English word "next" — `decisions.md §19a`'s
    recorded trap, found independently by two of the three T9 review lanes.

    Each term is matched against a haystack canonicalized with THAT term's
    own `apply_rn_fold` policy (see `_RN_FOLD_EXEMPT_TERMS_CASEFOLD` above) —
    not one shared haystack — so exempting "Jarn" from the rn->m fold cannot
    silently also un-fold the haystack for every OTHER term's comparison,
    and a literal, un-OCR'd "Jarn" in text is still folded consistently with
    the term side (both un-rn-folded), so the plain-spelling catch this term
    was added for (`ogl-pi-blacklist.md §4`, ACG override) still works.

    See `scripts/tests/test_sd32_t9_pi_normalization_and_inheritance.py` for
    the RED proof (word-boundary guard removed -> "next" false-positives;
    rn->m exemption removed -> "a tight jam" false-positives) and the GREEN
    fix, plus the recorded incident strings (`Cayden CaiLean`, `lrori`)
    still resolving correctly with both guards in place.
    """
    hits = normalized_term_hits(free_text)
    return hits[0] if hits else None


def normalized_term_hits(free_text: str) -> list[str]:
    """Every blacklist term whose canonicalized form appears, WORD-BOUNDED, in
    the canonicalized free text — the same scan `normalized_term_hit` runs,
    but reporting every hit rather than only the first. A caller that needs
    "is this row blocked at all" wants `normalized_term_hit`; a caller that
    reports which terms hit (e.g. for a human-facing receipt) wants this."""
    if not free_text.strip():
        return []
    text_casefold = free_text.casefold().replace(SOFT_HYPHEN, "-")
    canon_text_cache: dict[tuple[bool, bool], str] = {}

    def canon_text_for(needs_rn_fold: bool, needs_char_fold: bool) -> str:
        key = (needs_rn_fold, needs_char_fold)
        if key not in canon_text_cache:
            t = text_casefold
            if needs_rn_fold:
                t = t.replace("rn", "m")
            if needs_char_fold:
                t = t.translate(_FOLD_TABLE)
            canon_text_cache[key] = t
        return canon_text_cache[key]

    hits = []
    for term, canon_term, needs_rn_fold, needs_char_fold in _CANON_TERMS:
        if not canon_term:
            continue
        canon_text = canon_text_for(needs_rn_fold, needs_char_fold)
        if re.search(r"(?<![a-z0-9])" + re.escape(canon_term) + r"(?![a-z0-9])", canon_text):
            hits.append(term)
    return hits

# The minimum normalized-character length a needle/term must reach before the
# alphanumeric-normalized (no-separator) check applies to it. Below this
# bound, a short generic word or abbreviation risks over-redacting on
# coincidence (`decisions.md §24b`'s own "Abb"/"RMA" test case). The
# word-bounded `normalized_term_hit` scan above still covers short blacklist
# terms (the 3-4 normalized-character entries in `PI_BLACKLIST_TERMS`) at
# their ordinary, separated occurrences.
_MIN_NORMALIZED_NEEDLE_LEN = 6


def _normalize(s: str) -> str:
    """Needle-side normalization: strip EVERY non-alphanumeric character,
    including whitespace. Used to build the short, known needle/term forms
    (`_NORM_BLACKLIST_TERMS`, `scrub_name_pi_tokens`'s own `norm_needles`) —
    a multi-word deity/place name (`"Cayden Cailean"`) must normalize to its
    no-separator form (`"caydencailean"`) so it can still be found embedded
    in a genuinely no-separator concatenated identifier. Never use this on
    haystack VALUES being scanned for a hit — see `_normalize_haystack`."""
    return re.sub(r"[^a-z0-9]", "", s.lower())


def _normalize_haystack(s: str) -> str:
    """Value-side normalization for the concatenated-form checks (3/4):
    strips punctuation the way `_normalize` does, but PRESERVES real
    whitespace as a hard separator.

    `decisions.md §26`-adjacent incident (t9-onboarding cycle, 2026-08-23,
    `data/corpus/inner_sea_magic/ability/hidden_wand.json`): the blacklist
    term "Andoran" false-positived on ordinary prose "...activate a wand
    (or any similar..." because the OLD haystack normalization deleted the
    real spaces between "wand", "or", and "any", manufacturing the substring
    "andorany" out of three separate, real English words. Checks 3/4 exist
    to catch a term truly joined with NO separator at all — a PCGen
    `BONUS`/`DEFINE` variable identifier or a `TYPE:` value never contains
    whitespace to begin with, so preserving whitespace here costs that
    genuine catch nothing (there is none to preserve away) while it stops
    natural-language prose whose words merely happen to concatenate into a
    term once whitespace is deleted from being treated as a no-separator
    join it never was."""
    return re.sub(r"[^a-z0-9\s]", "", s.lower())


_NORM_BLACKLIST_TERMS: list[tuple[str, str]] = [
    (term, _normalize(term))
    for term in PI_BLACKLIST_TERMS
    if len(_normalize(term)) >= _MIN_NORMALIZED_NEEDLE_LEN
]


def blacklist_term_hit_including_concatenated(value: str) -> str | None:
    """`normalized_term_hit(value)` (word-bounded, OCR-normalized), OR — if
    that finds nothing — an alphanumeric-normalized (no-separator) substring
    match against `PI_BLACKLIST_TERMS`, bounded to `_MIN_NORMALIZED_NEEDLE_LEN`
    normalized characters, with real whitespace in `value` still acting as a
    boundary (`_normalize_haystack`) — see that function's docstring for why.

    This is the SAME check-4 the module docstring above describes, exposed as
    its own function so every caller that scans a token value against the
    blacklist — not only `scrub_name_pi_tokens`'s renamed-record branch — gets
    the concatenated-term coverage. `normalized_term_hit` alone cannot see a
    blacklisted term concatenated PascalCase-style into another token's value
    (e.g. a `TYPE:` token naming a variable by joining a blacklisted term
    directly onto a suffix with no separator) because its word-boundary
    requirement fails the moment the term is immediately followed by another
    letter — found live in an already-shipped record whose `TYPE:` token
    carried exactly this shape, un-redacted, on an otherwise-clean (non-
    renamed) record.

    Returns the matched term, or `None`."""
    if not value:
        return None
    hit = normalized_term_hit(value)
    if hit:
        return hit
    norm_value = _normalize_haystack(value)
    if not norm_value:
        return None
    for term, canon_term in _NORM_BLACKLIST_TERMS:
        if canon_term in norm_value:
            return term
    return None


def scrub_name_pi_tokens(
    tokens: list[dict[str, str]], name: str, key: str, neutral_name: str | None = None
) -> tuple[list[dict[str, str]], bool]:
    """`decisions.md §24b`-2: "The PI original appears nowhere that ships."

    `neutral_name`, when given, is the record's own `§24`-derived
    (`codex_neutral_name.neutral_name`) coordinate-only name. It enables
    NARROWER redaction (see the substitution branch below) for a value whose
    ONLY hit is a plain, space-preserving self-reference to this record's own
    name/key (never for a blacklisted term or a concatenated-identifier
    match) -- so a genuine `BONUS:`/`DEFINE:` game-mechanical value survives
    with only the self-referencing PI span replaced, instead of being wiped
    whole. Omitting it (the default) reproduces the exact prior
    full-redaction behaviour, byte for byte.

    A record whose NAME is PI can carry that same name again inside another
    token's VALUE (most concretely a `KEY:` field that restates the row's own
    full original key verbatim, or a `~`-delimited segment of `key`, since a
    `'<Concept> ~ <Deity>'`-shaped key embeds the PI term as one segment, not
    the whole string). `name`/`key` are used ONLY to build the redaction
    needle set here — they are never written into the returned record.

    Returns `(scrubbed_tokens, any_redacted)`. Never mutates the input.

    Four independent checks run per token value, any one of which redacts it:

    1. The word-bounded, OCR-normalized 61-term blacklist scan
       (`normalized_term_hit`) — catches an ordinary, separated occurrence of
       a blacklisted deity/place name.
    2. A space-preserving case-insensitive substring check against the
       record's own `name`/`key` (and `~`-split / whitespace-split segments
       of `key`) — catches an ordinary, separated occurrence of the record's
       own PI name.
    3. **Identifier-form check for the record's own identity.** PCGen's own
       `DEFINE`/`BONUS` tokens frequently concatenate the record's name into
       a variable identifier with no separator at all (`RedMantisAssassinLVL`,
       `WestcrownDevilLVL`) — a space-preserving substring check never matches
       a value with no spaces. Every needle from check 2 is therefore ALSO
       checked in a fully alphanumeric-normalized form (`[^a-z0-9]` stripped)
       against the same normalization of the value, bounded to needles of at
       least `_MIN_NORMALIZED_NEEDLE_LEN` normalized characters.
    4. **Identifier-form check for the 61-term blacklist.** The identical
       concatenation shape, but for a BLACKLISTED term rather than the
       record's own identity — e.g. a `TYPE:` token naming a choice-ability
       variable `"<Deity>AspectChoice"` with no separator. `normalized_term_hit`
       (check 1) cannot see this: its word-boundary requirement fails the
       moment the term is directly followed by another letter. Bounded the
       same way as check 3, using `_NORM_BLACKLIST_TERMS`.
    """
    needles: set[str] = set()
    norm_needles: set[str] = set()

    def add_needle(s: str) -> None:
        s = s.strip()
        if not s:
            return
        # `decisions.md` T9-onboarding-cause-closure fix (2026-08-23): both
        # needle sets are gated on the SAME `_MIN_NORMALIZED_NEEDLE_LEN`
        # floor `norm_needles` already used alone. Before this fix, `needles`
        # (the space-preserving check) had NO length floor, so a short,
        # generic word survived as a standalone needle. Gating on the
        # NORMALIZED length (not the raw string length) keeps the decision
        # independent of spacing/punctuation, matching how `norm_needles`
        # already decides.
        normalized = _normalize(s)
        if len(normalized) < _MIN_NORMALIZED_NEEDLE_LEN:
            return
        needles.add(s.lower())
        norm_needles.add(normalized)

    for s in (name, key):
        add_needle(s)
    if key:
        # `decisions.md` T9-onboarding-cause-closure fix (2026-08-23): the
        # per-WORD split below this comment used to also add every
        # individual word of a `~`-delimited segment as its OWN needle
        # (`for word in re.split(r"[\s()]+", segment): add_needle(word)`).
        # That over-generalises: PCGen's `KEY` schema is frequently
        # `<Category-or-Group> ~ <Specific>` (real shape: `"Trait ~ <a
        # PI-bearing trait name>"`, `"Temp Bonus ~ <a PI-bearing role/deity
        # name>"` -- never a real name repeated here, per `decisions.md
        # §24b`-2), and the individual words making up the group/descriptor
        # half --
        # "Trait", "Temp", "Bonus", "Evangelist", "Sentinel", "Exalted" --
        # are ordinary PCGen/Pathfinder rules vocabulary, not this record's
        # PI. Splitting to word granularity turned those into standalone
        # needles that matched an unrelated, non-PI, already-blacklist-clear
        # BONUS/DEFINE value merely because it happened to contain the same
        # common word (worst example found live: EVERY `Trait ~ <Name>`
        # record's own `TYPE:Trait...`/`BONUS:...TYPE=Trait` token matched
        # the single word "Trait", redacting the record's real mechanical
        # formula for a reason that had nothing to do with its PI content --
        # `decisions.md §24b`-2 requires removing the PI original, never
        # authorises redacting an unrelated generic term that happens to
        # share a word with it). Genuine self-reference (the record's own
        # FULL name/key, or a full `~`-segment, appearing verbatim in a
        # token value -- e.g. an `ABILITYPOOL` value restating the record's
        # own pool name) is still caught by the two `add_needle` calls
        # below, unaffected by removing the word-level loop; so is every
        # blacklisted term, via the independent `blacklist_term_hit_
        # including_concatenated` check (checks 1+4), which does not derive
        # its needles from `name`/`key` at all.
        for segment in re.split(r"\s*~\s*", key):
            add_needle(segment)

    scrubbed = []
    any_redacted = False
    for t in tokens:
        value = t["value"]
        if not value:
            scrubbed.append(dict(t))
            continue

        norm_value = _normalize_haystack(value)  # check 3, see that function's docstring
        value_lower = value.lower()

        blacklist_hit = blacklist_term_hit_including_concatenated(value)  # checks 1+4
        space_preserving_hit = any(needle in value_lower for needle in needles)  # check 2
        norm_only_hit = (  # check 3
            bool(norm_value) and any(n in norm_value for n in norm_needles)
        )
        identity_hit = space_preserving_hit or norm_only_hit

        # `decisions.md` T9-onboarding-cause-closure fix (2026-08-23, row 17's
        # remaining 21): "a BONUS:/DEFINE: value is a game rule, not Product
        # Identity, and §24 never authorised destroying mechanics." When the
        # ONLY reason a value would be wiped is a plain, space-preserving
        # self-reference to the record's OWN name/key (check 2 -- never a
        # blacklisted term, and never the concatenated-identifier check 3,
        # whose match position cannot be re-located in the original,
        # un-normalized string), and the caller supplies the record's own
        # `§24`-derived neutral name, redact ONLY the matched self-reference
        # span(s) -- replacing each with the neutral name -- rather than the
        # whole token. This is possible only for check 2 because that is the
        # sole check whose match is a literal, case-insensitive substring of
        # the ORIGINAL value with a locatable span; checks 1/3/4 match a
        # NORMALIZED (OCR-folded / alnum-stripped) form that does not map
        # back to a contiguous original-string span. A value that ALSO hits
        # the blacklist (checks 1/4) or the norm-only identity form (check 3)
        # is never narrowed -- it stays a full redaction, unchanged from the
        # pre-existing behaviour.
        if (
            neutral_name
            and space_preserving_hit
            and not norm_only_hit
            and not blacklist_hit
        ):
            narrowed_value = value
            narrowed = False
            for needle in sorted(needles, key=len, reverse=True):
                pattern = re.compile(re.escape(needle), re.IGNORECASE)
                if pattern.search(narrowed_value):
                    narrowed_value = pattern.sub(neutral_name, narrowed_value)
                    narrowed = True
            if narrowed:
                scrubbed.append({"key": t["key"], "value": narrowed_value})
                any_redacted = True
                continue
            # Matched via `needles` but the case-insensitive re-scan found no
            # span (should not happen given `space_preserving_hit` is True) --
            # fail safe to a full redaction rather than ship un-narrowed PI.

        if blacklist_hit or identity_hit:
            scrubbed.append({"key": t["key"], "value": REDACTED_PI_MARKER})
            any_redacted = True
        else:
            scrubbed.append(dict(t))
    return scrubbed, any_redacted
