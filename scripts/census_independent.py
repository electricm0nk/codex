#!/usr/bin/env python3
"""Independent census walker for SD-32 Gate 0 (AT-32-G0-001 / AT-32-G0-002).

Why this exists
----------------
SD-31 wave 1 found a single-level directory join that missed every nested
`.lst` file, stranding ~1,707 real units in in-scope books -- invisible to
the census until someone tripped over it. `scripts/census_independent.py`
exists to catch that class of blind spot mechanically: it walks the pinned
PCGen oracle **independently of `src/bin/v06_work_inventory.rs`** (the
walker being audited) and diffs the result against `docs/work-inventory.json`.
An independent walk is the point -- reusing the enumerator under audit
cannot detect that enumerator's own blind spots.

Structure: reader / analyser / reporter (`technical-design.md` Gate 0), so
the seam is present and reusable for a second reader later.

1. `discover_book_dirs()` -- reader. Walks
   `$PCGEN_CORPUS_ROOT/pathfinder` and enumerates every directory that is a
   PCGen "book": an immediate child of one of the five `paizo/` product
   categories (`roleplaying_game`, `player_companion`, `campaign_setting`,
   `adventure_path`, `gamemastery_cards`), or an immediate child of any
   non-`paizo` publisher directory -- provided it recursively contains at
   least one `.pcc` file (PCGen's own convention for "this directory is a
   loadable data source"). Re-derive the count with:

       find "$PCGEN_CORPUS_ROOT/pathfinder/paizo/roleplaying_game" \\
            "$PCGEN_CORPUS_ROOT/pathfinder/paizo/player_companion" \\
            "$PCGEN_CORPUS_ROOT/pathfinder/paizo/campaign_setting" \\
            "$PCGEN_CORPUS_ROOT/pathfinder/paizo/adventure_path" \\
            "$PCGEN_CORPUS_ROOT/pathfinder/paizo/gamemastery_cards" \\
            -mindepth 1 -maxdepth 1 -type d

   NOTE -- discovered figure vs. the bundle's stated "158-book" claim
   (`scope-draft.md`, `acceptance-and-verification.md AT-32-G0-001`,
   `technical-design.md`): this walker's own reproducible definition above
   yields **186** book directories against the pinned oracle (verified
   2026-08-22, `scripts/pcgen-oracle-pin.env`), not 158. No document in
   this bundle carries a command that reproduces 158; it appears to be an
   uncited figure. This is logged as a correction
   (`scripts/retro.py correction --subject scope-draft.md --claimed 158
   --actual 186 --verified-by "python3 scripts/census_independent.py"`)
   rather than silently reconciled -- AGENTS.md rule 9 and standing lesson
   2 (`workflow-instruction.md §9`).

2. `classify_scope()` -- analyser. Maps each discovered directory to the
   `docs/work-inventory.json` book roster by basename. Every directory not
   matched to a roster book id is EXCLUDED, and every exclusion is bucketed
   into one of a fixed set of named reasons (`_EXCLUSION_REASONS` below).
   A directory that cannot be bucketed at all is `unexplained` -- the
   AT-32-G0-001 gate variable, which must be `0`.

3. `count_objects()` -- reader/counter, restricted to in-scope
   directories. Walks every `*.lst` file and classifies each data row
   using the stated per-kind object-definition rules below (AT-32-G0-002):

   * A row is a **new unit** of its file's kind unless its identity field
     (the first tab-separated column) ends in `.MOD` (a continuation of an
     existing object elsewhere -- not counted as a unit) or `.FORGET`
     (a removal directive -- not a unit).
   * A row whose identity field contains `.COPY=` is a **derived unit**:
     a genuinely new named object cloned from an existing one. It is
     counted as a unit of its file's kind and tallied separately under
     `copy_derivation` so the derivation is visible, not hidden inside the
     plain count.
   * Comment lines (`#...`), blank lines, and pure-directive lines (first
     field contains `:` before any tab -- e.g. `SOURCELONG:...`) are not
     rows and are skipped.
   * Kind is assigned by filename per `_classify_kind_by_filename()`. Ten
     kinds are counted (`feat`, `class`, `spell`, `monster`,
     `monster_ability`, `equipment`, `equipment_modifier`, `companion`,
     `race`, `race_trait` -- the exact list in AT-32-G0-002). Files whose
     content is real, named, narrative objects but does not map cleanly
     onto one of the ten (template rows, class features, domains,
     deities, kits, languages, powers, or an `_abilities.lst` row whose
     `CATEGORY:` tag is not `FEAT`) are **not** force-fit into a kind --
     they are named and counted under `kind_unenumerable`, per
     AT-32-G0-002's explicit requirement that this category, if it
     exists, is "named and counted -- not pretended to be zero." Files
     that are pure engine/system wiring (datacontrols, datatables,
     variables, globalmodifiers, stat/align/save tables, proficiency
     tables) carry no discrete named objects at all and are skipped
     entirely as `non_object_files`, listed separately so the exclusion
     is auditable rather than silent.

   DISCOVERY -- AT-32-G0-002's ten-kind list omits `class_feature`, which
   is the single largest kind in `docs/work-inventory.json`
   (`totals.by_kind.class_feature` = 15,439, the largest of all eleven
   kinds the live inventory actually tracks). This walker does not guess
   which of the ten `class_feature` rows belong to; every `*_abilities_class*`
   file is filed under `kind_unenumerable["class_feature"]` and the count
   is reported plainly rather than silently absorbed into `feat` or
   dropped. Filed as a `## DISCOVERED` forward in `progress.md`.

CLI
---
    python3 scripts/census_independent.py --pcgen-root "$PCGEN_CORPUS_ROOT" \\
        --inventory docs/work-inventory.json \\
        --output artifacts/gate-0-census-closure/diff.json

Writes `--output` (the machine-checked diff, `jq '.unexplained'` must be
`"0"`... `0` as an int -- AT-32-G0-001's own verification command) and, next
to it, `excluded-directories.md` -- the human-readable per-directory
justification AT-32-G0-001 requires.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
from collections import Counter, defaultdict
from dataclasses import dataclass, field
from typing import Dict, List, Optional

PAIZO_CATEGORIES = (
    "roleplaying_game",
    "player_companion",
    "campaign_setting",
    "adventure_path",
    "gamemastery_cards",
)

# Books whose primary content is monster stat blocks encoded as PCGen
# "Race" LST rows (verified against the corpus 2026-08-22: e.g.
# `bestiary/b1_races.lst` holds monster stat blocks, not PC races -- PCGen
# has no separate "Monster" LST kind, it reuses RACE with a monster
# RACETYPE). Any book NOT in this set uses `*_races.lst` for true PC races.
MONSTER_BOOK_IDS = frozenset(
    {
        "bestiary",
        "bestiary_2",
        "bestiary_3",
        "bestiary_4",
        "bestiary_5",
        "bestiary_6",
        "bonus_bestiary",
        "monster_codex",
        "inner_sea_bestiary",
    }
)

TEN_KINDS = (
    "feat",
    "class",
    "spell",
    "monster",
    "monster_ability",
    "equipment",
    "equipment_modifier",
    "companion",
    "race",
    "race_trait",
)

# SD-32 card 15 (`decisions.md §12b`): kinds added after AT-32-G0-002's
# original ten-kind list, one at a time as each is proven safe to land in
# `src/bin/v06_work_inventory.rs`'s producer -- see that file's `Kind` enum
# doc comments for the corpus proof behind each addition. Kept separate from
# `TEN_KINDS` rather than folded in, so AT-32-G0-002's own criterion text
# ("the ten kinds") stays a truthful description of what it originally named;
# `ALL_KINDS` below is the walker's own live list.
ADDED_KINDS = (
    "skill",  # 170 units, 10 `*_skills.lst` files -- `15-card-15-other-kinds-memo.md` §7a.
    # SD-32 `decisions.md §17`: landed together, through the same generic
    # `SIMPLE_FILENAME_KINDS` mechanism `src/bin/v06_work_inventory.rs`'s
    # `file_kind()` uses for `skill` -- a filename-substring rule with no
    # row-content carve-out, book-attribution logic, or duplicate-identity
    # handling. `15-card-15-other-kinds-memo.md` §1-5 is the per-kind proof.
    "template",  # 2,343 units, `*_templates.lst` -- memo §1.
    "deity",  # 460 units, `*_deities.lst` -- memo §2.
    "power",  # 421 units, `up_powers.lst` (Ultimate Psionics only) -- memo §3.
    "domain",  # 183 units, `*_domains.lst` (the domain HEADER record) -- memo §4.
    "language",  # 143 units, `*_languages.lst`, 100% text-only (F0) -- memo §5.
    # SD-32 card 15 (`decisions.md §12b`, largest remaining bucket): a bare
    # (non-`_race`, non-`_class`, non-`_companion`/`_familiar`)
    # `*abilities*.lst` row whose `CATEGORY:` tag is not `FEAT`. Unlike the
    # six kinds above, this is NOT a filename-only rule -- the same file
    # mixes real, distinct objects with facets/pick-list entries/duplicates
    # of content already counted elsewhere, so the per-row disposition test
    # ported from `15-card-15-ability-category-classify.py` (see
    # `_ABILITY_CONTENT_RE`/`_ABILITY_GATEWAY_RE` and `count_objects`'s
    # `row_dependent` branch) decides kind-vs-not-an-object per row, not
    # per file. See `15-card-15-ability-category-memo.md` for the disposition
    # rules this ports. The memo's own headline figure (5,108 A / 778 B of
    # 5,886) does NOT reproduce live: re-deriving at this cycle's pin found
    # `ability_category:Internal` had grown 839->879 (an unrelated,
    # already-landed reroute, `decisions.md §14c`) AND a real
    # census/inventory disagreement the memo never caught -- 6 in-scope
    # `*_abilities_familiar*.lst` files (97 rows) that `src/bin/
    # v06_work_inventory.rs`'s `file_kind` already routes to the tracked
    # `companion` kind were falling into this branch here, so the memo's
    # 5,108/778 unknowingly double-counted them against `companion`. Fixed
    # by routing `_companion`/`_familiar` abilities files to `kind:companion`
    # (matching Rust's own order) BEFORE this branch runs. Live figure at
    # the card-15-ability cycle's pin: **5,028 (A) / 801 (B)** of 5,829
    # (5,926 minus the 97 companion rows) -- re-derive with
    # `python3 scripts/census_independent.py ...`; `counts_by_kind['ability']`
    # / `sum(v for k,v in kind_unenumerable.items() if
    # k.startswith('ability_category:'))`.
    "ability",
    # SD-32 `decisions.md §25` (operator: "In. We do not defer - we
    # complete."): the PF1e chargen Trait mechanic -- a bare `*abilities*.lst`
    # row whose `TYPE:` is `Trait` or starts with `Trait.`, checked BEFORE
    # the `ability`/`feat` per-row disposition above (`_row_is_pf1_trait`).
    # 566 units across 6 already-registered books at the pinned oracle SHA;
    # see that function's own doc comment for the corpus proof.
    "trait",
)

ALL_KINDS = TEN_KINDS + ADDED_KINDS

# System/engine wiring files -- no discrete named narrative objects live in
# these, so they are skipped entirely rather than filed as
# kind_unenumerable (which is reserved for real, named content objects
# that simply don't map onto the ten kinds).
NON_OBJECT_FILENAME_TOKENS = (
    "datacontrol",
    "datatable",
    "variable",
    "globalmodifier",
    "_stats",
    "_align",
    "_dynamic",
    "_saves",
    "biosetting",
    "abilitycategor",
    "profs_weapon",
    "profs_armor",
    "profs_shield",
    # SD-32 card 15 (`decisions.md §12b`): `ce__sizes.lst` is PF1e's fixed
    # 9-variant size table (Fine..Colossal), not corpus content -- already
    # engine-covered byte-for-byte by `src/rules_core/size.rs`'s
    # `SizeCategory` enum. Proven by class in `artifacts/gate-0-census-
    # closure/15-card-15-other-kinds-memo.md` §7b, not assumed from the
    # filename alone.
    "__sizes",
)


@dataclass
class BookDir:
    rel_path: str
    book_id: str
    category: str  # e.g. "paizo/roleplaying_game" or "<publisher>"


@dataclass
class ScopeResult:
    in_scope: List[BookDir] = field(default_factory=list)
    excluded: List[Dict[str, str]] = field(default_factory=list)
    unexplained: List[str] = field(default_factory=list)


def discover_book_dirs(pcgen_root: str) -> List[BookDir]:
    """Reader stage. See module docstring section 1 for the rule."""
    pathfinder_root = os.path.join(pcgen_root, "pathfinder")
    found: List[BookDir] = []

    def has_pcc(d: str) -> bool:
        for dirpath, _dirnames, filenames in os.walk(d):
            for fn in filenames:
                if fn.lower().endswith(".pcc"):
                    return True
        return False

    paizo_root = os.path.join(pathfinder_root, "paizo")
    for category in PAIZO_CATEGORIES:
        cat_dir = os.path.join(paizo_root, category)
        if not os.path.isdir(cat_dir):
            continue
        for name in sorted(os.listdir(cat_dir)):
            p = os.path.join(cat_dir, name)
            if os.path.isdir(p) and has_pcc(p):
                found.append(
                    BookDir(
                        rel_path=os.path.relpath(p, pathfinder_root),
                        book_id=name,
                        category=f"paizo/{category}",
                    )
                )

    if os.path.isdir(pathfinder_root):
        for pub in sorted(os.listdir(pathfinder_root)):
            pub_dir = os.path.join(pathfinder_root, pub)
            if pub == "paizo" or not os.path.isdir(pub_dir):
                continue
            for name in sorted(os.listdir(pub_dir)):
                p = os.path.join(pub_dir, name)
                if os.path.isdir(p) and has_pcc(p):
                    found.append(
                        BookDir(
                            rel_path=os.path.relpath(p, pathfinder_root),
                            book_id=name,
                            category=pub,
                        )
                    )

    return found


def _exclusion_reason(bd: BookDir, in_scope_ids: set) -> Optional[str]:
    """Returns a justification string for an excluded directory, or None
    if the directory does not match any known exclusion bucket (the
    AT-32-G0-001 'unexplained' case -- must never fire for a directory that
    is legitimately out of scope; a fired case is a real finding)."""
    if bd.book_id in in_scope_ids:
        return None  # not excluded at all
    if bd.category == "paizo/adventure_path":
        return "adventure-path content; not part of the roster (scope-draft.md)"
    if bd.category == "paizo/player_companion":
        return "player companion supplemental line; not part of the roster (scope-draft.md)"
    if bd.category == "paizo/campaign_setting":
        return "campaign-setting sourcebook not selected into additional_book_dirs"
    if bd.category == "paizo/gamemastery_cards":
        return "card-deck product; no rule-bearing LST content modelled"
    if bd.category == "paizo/roleplaying_game":
        if bd.book_id == "beginner_box":
            return "excluded via EXCLUDED_BOOKS (HANDOFF.md); introductory boxed-set duplicate of core_rulebook content"
        if bd.book_id == "core_essentials":
            return "shared_library scope (docs/work-inventory.json); Core Essentials residuals deleted per decisions.md §16"
        return None  # unexplained: a roleplaying_game book missing from the roster is a real gap
    if bd.category != "paizo/roleplaying_game" and bd.category.startswith("paizo/") is False:
        # non-paizo publisher
        if bd.rel_path == "dreamscarred_press/ultimate_psionics":
            return None  # should already be in_scope_ids; defensive branch
        return "third-party publisher content outside the Paizo + curated-additional roster (decisions.md/scope-draft.md)"
    return None


def classify_scope(book_dirs: List[BookDir], inventory: dict) -> ScopeResult:
    """Analyser stage. See module docstring section 2."""
    in_scope_ids = {b["id"] for b in inventory.get("books", [])}
    result = ScopeResult()
    for bd in book_dirs:
        if bd.book_id in in_scope_ids:
            result.in_scope.append(bd)
            continue
        reason = _exclusion_reason(bd, in_scope_ids)
        if reason is None:
            result.unexplained.append(bd.rel_path)
        else:
            result.excluded.append(
                {"path": bd.rel_path, "book_id": bd.book_id, "reason": reason}
            )
    return result


def _classify_kind_by_filename(basename: str, book_id: str):
    """Returns (bucket, key) where bucket is one of
    'kind' (one of TEN_KINDS), 'kind_unenumerable', or 'non_object_file'.
    See module docstring section 3 for the full rule table."""
    b = basename.lower()

    for token in NON_OBJECT_FILENAME_TOKENS:
        if token in b:
            return ("non_object_file", basename)

    # SD-32 `decisions.md §17`: moved from `kind_unenumerable["template_row"]`
    # to a tracked kind once `Kind::Template` landed in the Rust producer
    # (`src/bin/v06_work_inventory.rs`'s `SIMPLE_FILENAME_KINDS`), so the
    # walker and this census agree per `decisions.md §12b`'s acceptance bar
    # -- same move `skill` made below. Renamed `template_row` -> `template`:
    # the `_row` suffix only ever distinguished the untracked-bucket name
    # from a hypothetical future kind; there is no longer a distinction to
    # make. See `15-card-15-other-kinds-memo.md` §1.
    if "template" in b:
        return ("kind", "template")

    if "companion" in b:
        return ("kind", "companion")

    if "equipmod" in b:
        return ("kind", "equipment_modifier")
    if "equip" in b:
        return ("kind", "equipment")

    if "spell" in b:
        return ("kind", "spell")

    if "feat" in b:
        return ("kind", "feat")

    if "abilit" in b:
        if "_race" in b:
            if book_id in MONSTER_BOOK_IDS:
                return ("kind", "monster_ability")
            return ("kind", "race_trait")
        if "_class" in b:
            # SD-32 card 15 (`decisions.md §14c` item 4): a
            # `_abilities_class.lst` row carrying `CATEGORY:Internal` is
            # NOT uniformly PCGen bookkeeping -- the class_feature memo's
            # original blanket-exclusion claim (all 2,614 -> "not an
            # object") was tested by class and found wrong for 90.7% of the
            # population (2,371/2,614 carry independent content or a
            # resolved gateway token). Row-level disposition happens in
            # `count_objects()`'s `row_dependent_class_feature` branch via
            # `_row_is_bare_internal_marker()`; only a bare tracker row
            # (no content, no gateway) reroutes to
            # `ability_category:Internal` -- everything else stays counted
            # as `class_feature`. Full per-row proof:
            # `artifacts/gate-0-census-closure/
            # 15-card-15-category-internal-classify.py` / `-summary.md`.
            return ("row_dependent_class_feature", None)
        # SD-32 card 15-ability (`decisions.md §12b`): `src/bin/
        # v06_work_inventory.rs`'s `file_kind` checks `_abilities_companion`/
        # `_abilities_familiar` BEFORE falling through to a bare-abilities
        # kind, so those files' rows are already counted under the tracked
        # `companion` kind (e.g. `b3_abilities_familiar.lst`,
        # `ce_abilities_familiar_cr.lst`) -- discovered as a real
        # census/inventory disagreement this cycle (97 rows across 6
        # in-scope files were falling into `row_dependent` here while Rust
        # already enumerated them as `companion`). Matched to Rust's own
        # order so the two walkers agree per this card's acceptance bar,
        # not because companion abilities are a new finding.
        if "_companion" in b or "_familiar" in b:
            return ("kind", "companion")
        # bare abilities file: row-level CATEGORY: tag decides (handled by caller)
        return ("row_dependent", None)

    if "class" in b:
        return ("kind", "class")

    # SD-32 `decisions.md §17`: was `"kit" in b`, which false-positived on
    # `kitsune_races.lst` -- "Kitsune" the race NAME contains the substring
    # "kit", so the entire file (real `race`-kind content) was wrongly
    # bucketed here ahead of the `race` check below. Narrowed to `_kits`
    # (the real filename convention every genuine kit file uses --
    # `cr_kits.lst`, `b1_kits_race.lst`, ...) so `kitsune_races.lst` now
    # falls through to the `race` branch, matching what
    # `src/bin/v06_work_inventory.rs`'s `file_kind` already did all along
    # (it never had a "kit" branch at all -- `decisions.md §12b`'s "the two
    # must agree" bar). No genuine kit-file content is lost: every
    # `*_kits.lst` file uses PCGen's `STARTPACK:`-block format, whose rows
    # all carry a `:` in their own first field and are therefore already
    # skipped as directive lines by `_parse_lst_rows` -- verified live,
    # every real `_kits.lst`/`_kits_race.lst`/`_kits_companion.lst` file in
    # scope contributes 0 rows under either the old or the new rule; only
    # the misclassified `kitsune_races.lst` row (1 unit) moves, from
    # `kind_unenumerable["kit"]` to `kind["race"]` where it always belonged.
    if "_kits" in b:
        return ("kind_unenumerable", "kit")
    # SD-32 `decisions.md §17`: same generic-path move as `template` above --
    # `language`/`deity`/`domain`/`power` become tracked kinds once their
    # `Kind::` variants land in `SIMPLE_FILENAME_KINDS`. See
    # `15-card-15-other-kinds-memo.md` §2-5.
    if "language" in b:
        return ("kind", "language")
    if "deit" in b:
        return ("kind", "deity")
    if "domain" in b:
        return ("kind", "domain")
    if "power" in b:
        return ("kind", "power")

    if "race" in b:
        if book_id in MONSTER_BOOK_IDS and "_pc" not in b:
            return ("kind", "monster")
        return ("kind", "race")

    # SD-32 card 15 (`decisions.md §12b`): moved out of `unclassified:<file>`
    # once `Kind::Skill` landed in the Rust producer
    # (`src/bin/v06_work_inventory.rs`), so the walker and the inventory
    # agree per this card's own acceptance bar. See `ADDED_KINDS` above.
    if "skill" in b:
        return ("kind", "skill")

    return ("kind_unenumerable", f"unclassified:{basename}")


# SD-32 card 15 (`decisions.md §14c` item 4): the class_feature lane's
# original blanket rule -- every `_abilities_class.lst` row carrying
# `CATEGORY:Internal` is "not an object" -- was tested by class against the
# same per-row disposition method the sibling `ability_category` lane used
# on the *other* `ability_category:Internal` rows (bare `*abilities*.lst`
# files) and found wrong for the great majority: of the 2,614 such rows,
# 2,371 (90.7%) carry independent mechanical/narrative content of their own
# or a gateway (`ABILITY:...|AUTOMATIC|<target>`) token that resolves to an
# already-real object -- proven by class, not asserted (re-derive command
# and full per-row breakdown:
# `artifacts/gate-0-census-closure/15-card-15-category-internal-classify.py`
# / `-summary.md`). Only a row that is `CATEGORY:Internal` AND carries none
# of the fields below AND carries no `ABILITY:...|AUTOMATIC|` gateway token
# is a genuine PCGen bookkeeping marker with zero payload of its own -- 40
# of the 2,614, all bare "<Name> Tracker" / "<Name> Qualifier" rows whose
# only fields are `CATEGORY:`/`KEY:`/`TYPE:`/`VISIBLE:`/`SOURCEPAGE:`.
#
# This field list is deliberately wide (AGENTS.md concurrency rule: "a grep
# filtered to BONUS/PRE hides STACK/MULT and other application-governing
# fields") -- a narrower DEFINE:/BONUS:-only test (`shape_ledger`'s formula
# extraction, the class_feature memo's original standard) misses real
# non-formula payload such as `DR:` (names a class-feature-specific
# damage-reduction variable the engine's DR machinery reads) or `SPELLLEVEL:`
# (a class-level-to-spell mapping) -- both present on rows the original memo
# cited as its own worked (B) examples.
_ROW_CONTENT_FIELD_RE = re.compile(
    r"DEFINE:|BONUS[A-Z]*:|DESC:|ASPECT:|CSKILL:|MOVE:|AUTO:|TEMPLATE:|SPROP:|QUALITY:|SR:|DR:|SAB:|VISION:|"
    r"SPELLKNOWN[A-Z]*:|TEMPBONUS:|CHOOSE:|NATURALATTACKS:|COMPANIONLIST:|ADD:|FOLLOWERS:|UDAM:|UMULT:|"
    r"SELECT:|COST:|MOVECLONE:|SPELLS:|SERVESAS:|DEFINESTAT:|UNENCUMBEREDMOVE:|BENEFIT:|SPELLLEVEL:|CMB:"
)
_ROW_GATEWAY_FIELD_RE = re.compile(r"ABILITY:[^\t]+\|AUTOMATIC\|")


def _row_is_bare_internal_marker(line: str) -> bool:
    """True only for a `CATEGORY:Internal` row that carries neither a
    content-bearing field nor a gateway (`ABILITY:...|AUTOMATIC|`) token --
    the narrow, provable (B) class. See the comment above
    `_ROW_CONTENT_FIELD_RE` for the re-derive command and the count this
    excludes (40 of 2,614 at the pinned oracle SHA)."""
    return not _ROW_CONTENT_FIELD_RE.search(line) and not _ROW_GATEWAY_FIELD_RE.search(
        line
    )


def _row_category_tag(line: str) -> Optional[str]:
    for field_ in line.split("\t"):
        f = field_.strip()
        if f.upper().startswith("CATEGORY:"):
            return f.split(":", 1)[1].strip()
    return None


def _key_field(line: str) -> Optional[str]:
    for field_ in line.split("\t"):
        f = field_.strip()
        if f.upper().startswith("KEY:"):
            return f.split(":", 1)[1].strip()
    return None


def _row_type_tag(line: str) -> Optional[str]:
    for field_ in line.split("\t"):
        f = field_.strip()
        if f.upper().startswith("TYPE:"):
            return f.split(":", 1)[1].strip()
    return None


def _row_is_pf1_trait(line: str) -> bool:
    """SD-32 `decisions.md §25` (`kind: trait` epic): a bare `*abilities*.lst`
    row whose `TYPE:` value is exactly `Trait` or starts with `Trait.` is a
    PF1e chargen Trait record (`TYPE:Trait.RaceTrait.Oread Race Trait`,
    `TYPE:Trait.Combat`, ...) -- structurally identical to the `Kind::Ability`
    bare-row population but a genuinely distinct game mechanic (a
    character-creation Trait, picked at level 1, is not a racial special
    ability). Ported byte-identical to `src/bin/v06_work_inventory.rs`'s
    `refine_kind`'s `Kind::Ability` arm so the two walkers agree
    (`decisions.md §12b`). Tested corpus-wide against the pinned oracle: 566
    hits across 6 already-registered books (`advanced_players_guide`,
    `core_rulebook`, `ultimate_campaign`, `ultimate_psionics`,
    `inner_sea_gods`, `inner_sea_races`), 0 false positives against any
    other already-tracked kind's own TYPE: vocabulary (none of which use a
    bare or dot-led `Trait` first segment)."""
    t = _row_type_tag(line)
    if t is None:
        return False
    return t == "Trait" or t.startswith("Trait.")


# SD-32 card 15-ability (`decisions.md §12b`): ported unchanged from
# `artifacts/gate-0-census-closure/15-card-15-ability-category-classify.py`
# (the memo lane's own adjudicated per-row classifier for the bare
# `*abilities*.lst` -> `ability_category:*` population -- NOT the same
# population as `_ROW_CONTENT_FIELD_RE` above, which classifies the
# DIFFERENT, already-resolved `_abilities_class.lst` `CATEGORY:Internal`
# reroute). Deliberately the memo's own narrower field list, not
# `_ROW_CONTENT_FIELD_RE`'s wider one -- the memo's per-bucket rulings
# (`15-card-15-ability-category-memo.md`) were reviewed and written against
# these exact patterns (e.g. `Ability Focus`'s 272-row B-picklist ruling
# rests on "zero fields beyond CATEGORY:/TYPE:" under this list); porting a
# wider list would silently redecide rulings no decision document approved.
_ABILITY_CONTENT_RE = re.compile(
    r"DEFINE:|BONUS[A-Z]*:|DESC:|ASPECT:|CSKILL:|MOVE:|AUTO:|TEMPLATE:|SPROP:|QUALITY:|SR:|DR:|SAB:|VISION:"
)
_ABILITY_GATEWAY_RE = re.compile(r"ABILITY:[^\t]+\|AUTOMATIC\|")

# Tracked kinds the ability-category duplicate check joins against -- exactly
# the memo's own list (`15-card-15-ability-category-classify.py`'s Pass 1).
_ABILITY_DUPLICATE_CHECK_KINDS = frozenset(
    {
        "feat",
        "class",
        "spell",
        "monster",
        "monster_ability",
        "equipment",
        "equipment_modifier",
        "companion",
        "race",
        "race_trait",
    }
)


def _collect_tracked_keys(pathfinder_root: str, in_scope: List[BookDir]) -> Dict[str, set]:
    """KEY: field values for every unit already counted under one of
    `_ABILITY_DUPLICATE_CHECK_KINDS`, used ONLY for the ability_category
    B-duplicate join (`15-card-15-ability-category-memo.md` "the shared-name
    hazard" -- KEY:-field-only, never a bare-identity fallback: a shared
    *display name* is not proof of a shared *thing*). Ported from
    `15-card-15-ability-category-classify.py`'s own Pass 1, unchanged."""
    tracked: Dict[str, set] = defaultdict(set)
    for bd in in_scope:
        book_dir = os.path.join(pathfinder_root, bd.rel_path)
        for dirpath, _dirnames, filenames in os.walk(book_dir):
            for fn in sorted(filenames):
                if not fn.lower().endswith(".lst"):
                    continue
                full = os.path.join(dirpath, fn)
                bucket, key = _classify_kind_by_filename(fn, bd.book_id)
                if bucket == "row_dependent":
                    for identity, raw in _parse_lst_rows(full):
                        cat = _row_category_tag(raw)
                        if cat and cat.upper() == "FEAT":
                            ident_upper = identity.upper()
                            if ident_upper.endswith((".FORGET", ".MOD")):
                                continue
                            kf = _key_field(raw)
                            if kf:
                                tracked["feat"].add(kf)
                    continue
                if bucket != "kind" or key not in _ABILITY_DUPLICATE_CHECK_KINDS:
                    continue
                for identity, raw in _parse_lst_rows(full):
                    ident_upper = identity.upper()
                    if ident_upper.endswith((".FORGET", ".MOD")):
                        continue
                    kf = _key_field(raw)
                    if kf:
                        tracked[key].add(kf)
    return tracked


def _parse_lst_rows(path: str):
    """Yields (identity, raw_line) for every real object row in an LST
    file, per the comment/blank/directive-line skip rule in the module
    docstring."""
    try:
        with open(path, "r", encoding="utf-8", errors="replace") as fh:
            for raw in fh:
                line = raw.rstrip("\n")
                if not line.strip():
                    continue
                if line.lstrip().startswith("#"):
                    continue
                if "\t" not in line:
                    continue
                identity = line.split("\t", 1)[0]
                if ":" in identity:
                    continue  # directive line, e.g. SOURCELONG:...
                yield identity, line
    except OSError:
        return


def count_objects(pathfinder_root: str, in_scope: List[BookDir]) -> dict:
    """Reader/counter stage. See module docstring section 3."""
    counts_by_kind: Counter = Counter()
    counts_by_book_kind: Dict[str, Counter] = defaultdict(Counter)
    kind_unenumerable: Counter = Counter()
    kind_unenumerable_by_book: Dict[str, Counter] = defaultdict(Counter)
    mod_continuation = 0
    copy_derivation = 0
    forget_directive = 0
    non_object_files: List[str] = []
    files_by_kind_example: Dict[str, str] = {}

    # SD-32 card 15-ability: built once, up front -- the `row_dependent`
    # branch's B-duplicate disposition needs every tracked kind's KEY: set
    # before it can classify a single ability_category row. See
    # `_collect_tracked_keys`'s own docstring.
    tracked_keys = _collect_tracked_keys(pathfinder_root, in_scope)

    for bd in in_scope:
        book_dir = os.path.join(pathfinder_root, bd.rel_path)
        for dirpath, _dirnames, filenames in os.walk(book_dir):
            for fn in sorted(filenames):
                if not fn.lower().endswith(".lst"):
                    continue
                full = os.path.join(dirpath, fn)
                bucket, key = _classify_kind_by_filename(fn, bd.book_id)

                if bucket == "non_object_file":
                    non_object_files.append(os.path.relpath(full, pathfinder_root))
                    continue

                for identity, raw_line in _parse_lst_rows(full):
                    if bucket == "row_dependent":
                        cat = _row_category_tag(raw_line)
                        if _row_is_pf1_trait(raw_line):
                            # SD-32 `decisions.md §25`: PF1e chargen Trait,
                            # checked before the FEAT redirect below -- a
                            # Trait row's own `CATEGORY:` is always `Special
                            # Ability`, never `FEAT`, so this ordering never
                            # actually contends with it; stated explicitly
                            # so a future CATEGORY vocabulary change cannot
                            # silently invert the priority.
                            row_bucket, row_key = "kind", "trait"
                        elif cat and cat.upper() == "FEAT":
                            row_bucket, row_key = "kind", "feat"
                        else:
                            # SD-32 card 15-ability (`decisions.md §12b`):
                            # ported per-row A/B disposition, not a filename
                            # rule -- see `_ABILITY_CONTENT_RE`'s doc
                            # comment and `15-card-15-ability-category-memo.md`.
                            # Priority order matches the memo exactly:
                            # B-duplicate (KEY: collides with an
                            # already-tracked kind) beats A (independent
                            # content) beats B-gateway (resolves to another
                            # real row) beats B-picklist (neither).
                            kf = _key_field(raw_line)
                            collided = False
                            if kf:
                                for tk_kind, tk_keys in tracked_keys.items():
                                    if kf in tk_keys:
                                        collided = True
                                        break
                            if collided:
                                row_bucket, row_key = (
                                    "kind_unenumerable",
                                    f"ability_category:{cat or 'UNKNOWN'}",
                                )
                            elif _ABILITY_CONTENT_RE.search(raw_line):
                                row_bucket, row_key = "kind", "ability"
                            else:
                                row_bucket, row_key = (
                                    "kind_unenumerable",
                                    f"ability_category:{cat or 'UNKNOWN'}",
                                )
                    elif bucket == "row_dependent_class_feature":
                        cat = _row_category_tag(raw_line)
                        if cat and cat.upper() == "INTERNAL" and _row_is_bare_internal_marker(
                            raw_line
                        ):
                            # Card 15 §14c item 4: a genuine PCGen bookkeeping
                            # marker -- CATEGORY:Internal with no content
                            # field and no gateway token of its own (proven
                            # by class, not by the blanket file-kind rule;
                            # see `_row_is_bare_internal_marker`'s docstring).
                            # File it under the same bucket the bare-`abilit`
                            # branch above already uses for this marker.
                            row_bucket, row_key = (
                                "kind_unenumerable",
                                "ability_category:Internal",
                            )
                        else:
                            # Either not CATEGORY:Internal at all, or
                            # CATEGORY:Internal but carrying real content or
                            # a resolved gateway -- a real class_feature
                            # object (or a proven facet of one), per card
                            # 15's per-row classifier.
                            row_bucket, row_key = "kind_unenumerable", "class_feature"
                    else:
                        row_bucket, row_key = bucket, key

                    ident_upper = identity.upper()
                    if ident_upper.endswith(".FORGET"):
                        forget_directive += 1
                        continue
                    if ident_upper.endswith(".MOD"):
                        mod_continuation += 1
                        continue
                    is_copy = ".COPY=" in ident_upper

                    if row_bucket == "kind":
                        counts_by_kind[row_key] += 1
                        counts_by_book_kind[bd.book_id][row_key] += 1
                        files_by_kind_example.setdefault(
                            row_key, os.path.relpath(full, pathfinder_root)
                        )
                    else:
                        kind_unenumerable[row_key] += 1
                        kind_unenumerable_by_book[bd.book_id][row_key] += 1

                    if is_copy:
                        copy_derivation += 1

    return {
        "counts_by_kind": dict(counts_by_kind),
        "counts_by_book_kind": {k: dict(v) for k, v in counts_by_book_kind.items()},
        "kind_unenumerable": dict(kind_unenumerable),
        "kind_unenumerable_by_book": {
            k: dict(v) for k, v in kind_unenumerable_by_book.items()
        },
        "mod_continuation": mod_continuation,
        "copy_derivation": copy_derivation,
        "forget_directive": forget_directive,
        "non_object_files": non_object_files,
        "files_by_kind_example": files_by_kind_example,
        "total_counted_units": sum(counts_by_kind.values()),
        "total_kind_unenumerable_units": sum(kind_unenumerable.values()),
    }


def _write_excluded_directories_md(scope: ScopeResult, path: str) -> None:
    lines = [
        "# Excluded directories -- SD-32 Gate 0 census (AT-32-G0-001)",
        "",
        "Generated by `scripts/census_independent.py`. Every directory the",
        "independent walker discovered but that is not part of the",
        "`docs/work-inventory.json` book roster, with its justification.",
        "An `unexplained` entry (a directory with no bucketed reason) is a",
        "real finding, not a formatting gap.",
        "",
        f"**Unexplained: {len(scope.unexplained)}**",
        "",
    ]
    if scope.unexplained:
        lines.append("## Unexplained (gate-failing)")
        lines.append("")
        for p in scope.unexplained:
            lines.append(f"- `{p}`")
        lines.append("")
    lines.append(f"## Excluded, justified ({len(scope.excluded)})")
    lines.append("")
    lines.append("| Directory | Reason |")
    lines.append("|---|---|")
    for row in sorted(scope.excluded, key=lambda r: r["path"]):
        lines.append(f"| `{row['path']}` | {row['reason']} |")
    lines.append("")
    lines.append(f"## In scope ({len(scope.in_scope)})")
    lines.append("")
    for bd in sorted(scope.in_scope, key=lambda b: b.rel_path):
        lines.append(f"- `{bd.rel_path}`")
    lines.append("")
    with open(path, "w", encoding="utf-8") as fh:
        fh.write("\n".join(lines))


def run(pcgen_root: str, inventory_path: str, output_path: str) -> dict:
    with open(inventory_path, "r", encoding="utf-8") as fh:
        inventory = json.load(fh)

    book_dirs = discover_book_dirs(pcgen_root)
    scope = classify_scope(book_dirs, inventory)
    counts = count_objects(os.path.join(pcgen_root, "pathfinder"), scope.in_scope)

    diff = {
        "discovered_book_dirs": len(book_dirs),
        "in_scope_book_dirs": len(scope.in_scope),
        "excluded_book_dirs": len(scope.excluded),
        "unexplained": len(scope.unexplained),
        "unexplained_directories": scope.unexplained,
        **counts,
    }

    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    with open(output_path, "w", encoding="utf-8") as fh:
        json.dump(diff, fh, indent=2, sort_keys=True)
        fh.write("\n")

    excluded_md_path = os.path.join(os.path.dirname(output_path), "excluded-directories.md")
    _write_excluded_directories_md(scope, excluded_md_path)

    return diff


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--pcgen-root", required=True, help="$PCGEN_CORPUS_ROOT")
    parser.add_argument("--inventory", required=True, help="docs/work-inventory.json")
    parser.add_argument("--output", required=True, help="diff.json output path")
    args = parser.parse_args(argv)

    diff = run(args.pcgen_root, args.inventory, args.output)
    print(
        f"discovered={diff['discovered_book_dirs']} "
        f"in_scope={diff['in_scope_book_dirs']} "
        f"excluded={diff['excluded_book_dirs']} "
        f"unexplained={diff['unexplained']}"
    )
    if diff["unexplained"]:
        print("UNEXPLAINED DIRECTORIES:", diff["unexplained_directories"], file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
