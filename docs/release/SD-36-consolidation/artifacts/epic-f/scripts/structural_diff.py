#!/usr/bin/env python3
"""Structural diff for the F1 converter change (epic-f-class-completion.md SS3.5, F1.6).

F1 adversarial finding 4: no structural-diff script existed on the branch, so the converter's
--check (which compares byte-for-byte and fails whole on the first difference) could not state
what actually moved between the on-disk baseline (data/sheet_rules/) and a fresh conversion --
only that something did. This script answers that question without ever writing to
data/sheet_rules itself: it reads the tracked baseline plus a SCRATCH directory a caller already
produced with

    cargo run --locked -j <n> -p codex-ingest --bin sheet_rule_convert -- --dump <scratch dir>

and reports:
  - the file-set delta (added / removed paths), split into rule files vs `_vars/*.json` vs other;
  - for every rule id present on BOTH sides, which top-level fields changed -- the diff contract
    (SS1 step 4 / SS3.5) allows only `granted_by` and `grants` to grow; anything else changing is
    named as an "unexpected field delta";
  - `granted_by` diffed as a SET of edges, never by length: an ADDED edge is the only change
    SS3.5 allows; a REMOVED one gates the run (F1 re-check round 1, finding 2: the previous
    length-diff missed same-length replacements entirely, and a pass-through allow-list accepted
    any change including deletion);
  - `grants` diffed by TARGET SIGNATURE, not raw JSON equality: F1 itself ships two sanctioned
    in-place transformations of an existing grant's WRAPPER or REPRESENTATION with its target
    held fixed -- F1-2 wraps a bare `FactGrant` in a `GatedFactGrant` around the SAME fact when a
    PRE-gate applies, and F1-3 expands a bare `Proficiency::WeaponGroup`/`WeaponTag` tag into an
    equivalent `WeaponSet`/`WeaponAllOf` naming the SAME tag (case-insensitive) with a resolved
    member list. Neither changes what is granted, only how; a raw-JSON diff cannot tell either
    apart from a removal, and flagged both as violations on the real corpus (F1 re-check round 2
    fix). `grant_signature()` reduces a grant to `(fact kind, target)`, gating-wrapper-agnostic
    and tag-case-insensitive for `Proficiency`; a signature present on the baseline side and
    ABSENT (by count) on the fresh side is a genuine loss and gates the run -- fabricated content
    (a different target under the same or a different shape) still gates, since its signature
    differs from anything the baseline held;
  - added `granted_by` EDGES, grouped by the TARGET rule's kind (the per-kind table SS3.1/F1.6
    ask the diff to reconcile against);
  - an explicit summary line for the record/rule/var_table counts so F1.6's acceptance command
    has one thing to read instead of re-deriving it by hand -- and a GATE failure, not just a
    printed pair, when `records` or `converted` moves against the baseline `_report.json` (F1
    re-check round 1, finding 2: a moved count previously printed but was never flagged);
  - EXPECTED delta classes, named, counted and explained rather than allowed blanket (F1
    re-check round 3, finding 1, ORCHESTRATOR RULING): `granted_by`/`grants` growth (F1's own
    declared purpose), `_vars/` tables added for a `GatedFactGrant`'s condition variables, the
    corpus's newly-held rule ids, and `provenance` deltas on the EXACT pinned record list in
    `structural_diff_expected_provenance_deltas.json` (the `current_class` closure-tracker fix
    re-attributing `SUBCLASSLEVEL` rows to their class chassis) -- a `provenance` delta on any
    record NOT on that list still gates, same as any delta to any other field.

`missing_grant_signatures` (round 3, finding 1) matches baseline grants to fresh ones with a
real maximum bipartite match (`_max_bipartite_match`, Kuhn's algorithm) over the `grant_covers`
edge, not a greedy order-dependent scan -- an identical multiset of grants presented in a
different order on the fresh side (e.g. a bare and a gated grant of one signature, swapped)
now matches fully instead of reporting a spurious loss. Baseline-side EXACT duplicates (two or
three grants restating the identical content under different book capitalization, e.g. three
`picaroon_weapon_proficiency` `WeaponGroup` grants folding to F1-3's one `WeaponSet`) are
collapsed to one representative (`_old_effects_deduplicated`) BEFORE matching, so a real dedup
is never counted as a loss of the collapsed duplicates -- a pair that only SHARES a signature
but genuinely differs (a different gate, or for a `WeaponSet` a different member list) is never
collapsed, so a genuine drop still cannot hide behind an untouched sibling (round 1's own
contract, unchanged).

Usage:
    python3 structural_diff.py <scratch_dump_dir> [--baseline data/sheet_rules] [--max-examples N] [--report-only]

Exit code: 1 when any of removed rule ids, unexpected field deltas, removed `granted_by` edges,
removed `grants`, or a moved `records`/`converted` count is non-zero -- the gate F1.6 reads. Pass
--report-only to keep the old report-and-exit-0 behavior for exploration.
"""
from __future__ import annotations

import argparse
import glob
import json
import os
import sys
from collections import Counter, defaultdict

ALLOWED_GROWING_FIELDS = {"granted_by", "grants"}

# SD-36 Epic F1 re-check round 3, finding 1 (ORCHESTRATOR RULING): the `current_class` closure
# fix's `provenance.closure_rows` growth is an EXPECTED delta only for this EXACT, generated
# record list -- never a blanket allowance for the `provenance` field. See the data file's own
# `_purpose`/`_command` for how it was produced and how to regenerate it.
_EXPECTED_PROVENANCE_DELTAS_PATH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "structural_diff_expected_provenance_deltas.json")


def _load_expected_provenance_deltas() -> frozenset[str]:
    try:
        with open(_EXPECTED_PROVENANCE_DELTAS_PATH, "r", encoding="utf-8") as fh:
            data = json.load(fh)
    except (OSError, json.JSONDecodeError):
        return frozenset()
    records = data.get("records", [])
    assert len(records) == data.get("_count"), f"{_EXPECTED_PROVENANCE_DELTAS_PATH}: _count {data.get('_count')} != len(records) {len(records)} -- regenerate with _command"
    return frozenset(records)


EXPECTED_PROVENANCE_DELTA_RECORDS = _load_expected_provenance_deltas()

# SD-36 Epic F1 re-check round 3, finding 1 (ORCHESTRATOR RULING: "the ONE added rule -- name it
# and its cause"). A newly-held rule id is never gated (SS3.5 allows new content), but every one
# measured on this branch must be named and explained, not just counted. Verified (`git log
# --oneline -- crates/codex-ingest/src/pcgen_import/sheet_rule/closure.rs`): the `current_class`
# fix in commit 71c729e408 correctly attributes a 5th `BONUS:ABILITYPOOL|Psion Feat` occurrence
# from the Psion subclass block (`up_classes.lst`, previously misattributed off Psion by the
# pre-fix closure build) to the Psion chassis, alongside the four it already held -- same value
# and gate shape as its siblings (`ClassLevel(psion)/5 + 1`, applicable at levels 1-20).
KNOWN_ADDED_RULE_CAUSES = {
    "ultimate_psionics:class:psion#bonus5": (
        "the `current_class` closure fix (commit 71c729e408) correctly attributes a 5th "
        "`BONUS:ABILITYPOOL|Psion Feat` occurrence in the Psion subclass block to the Psion "
        "chassis, alongside the four it already held (same value/gate shape: "
        "ClassLevel(psion)/5 + 1, levels 1-20)"
    ),
}

# SD-36 Epic F1 stage 6 (merge-readiness blocker 2, F1.6 vs the tranche/16 merge baseline). The
# comma-split `BONUS:VAR` index fix (`closure.rs`, stage-5 receipt §1a) does not only change
# RENDERED lines (that population is stage5/merge-readiness-blockers-receipt.md's own
# denominator) -- it also changes 35 on-disk RECORD FIELDS and adds 6 new `#spell1_<spell>`
# sub-rule ids that a rendered-line diff against the census population never reaches (bestiary
# monsters, prestige classes, and drow/gnome innate SLAs are all off the 34-family census list).
# Each is the SAME mechanism, named and reachability-checked per family in the pinned data file's
# own `families` list -- never a blanket allowance: a delta off the pinned (rule id, field) list,
# or a new rule id off the pinned list, still gates like any other.
_BONUS_VAR_SPLIT_DELTAS_PATH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "structural_diff_bonus_var_split_record_deltas.json")


def _load_bonus_var_split_deltas() -> tuple[frozenset[tuple[str, str]], dict[str, str]]:
    try:
        with open(_BONUS_VAR_SPLIT_DELTAS_PATH, "r", encoding="utf-8") as fh:
            data = json.load(fh)
    except (OSError, json.JSONDecodeError):
        return frozenset(), {}
    field_deltas = data.get("field_deltas", [])
    assert len(field_deltas) == data.get("_field_delta_count"), f"{_BONUS_VAR_SPLIT_DELTAS_PATH}: _field_delta_count {data.get('_field_delta_count')} != len(field_deltas) {len(field_deltas)} -- regenerate with _command_to_reproduce"
    new_rule_ids = data.get("new_rule_ids", {})
    assert len(new_rule_ids) == data.get("_new_rule_id_count"), f"{_BONUS_VAR_SPLIT_DELTAS_PATH}: _new_rule_id_count {data.get('_new_rule_id_count')} != len(new_rule_ids) {len(new_rule_ids)} -- regenerate with _command_to_reproduce"
    return frozenset((rid, field) for rid, field in field_deltas), dict(new_rule_ids)


EXPECTED_BONUS_VAR_SPLIT_FIELD_DELTAS, BONUS_VAR_SPLIT_NEW_RULE_CAUSES = _load_bonus_var_split_deltas()
KNOWN_ADDED_RULE_CAUSES.update(BONUS_VAR_SPLIT_NEW_RULE_CAUSES)

# SD-36 Epic F1 stage 5 (population run, ORCHESTRATOR spec 3.5 step 1): the NATURALATTACKS
# suffix fix (rule-gap-receipt.md: `acc.lines.len()` replacing the per-occurrence-local index)
# renumbers the WHOLE `#natural<N>` id family on every record whose accumulator held any lines
# before its NATURALATTACKS arm ran -- not only the 306 previously-colliding groups. That is a
# much bigger raw id churn (1,290 old ids replaced by 1,659 new ones, net +369 -- the 369
# previously-shadowed rules rule-gap-receipt.md named) than a naive read of "369 changed ids"
# suggests, PLUS 6 ids where the old and new numbering coincidentally reuse the same suffix
# number for DIFFERENT attack content (a record's whole family shifted, and that one slot
# happened to land on a shared number both sides already used).
#
# This is still a single, mechanical, content-preserving rename, not 1,955 separate defects:
# `naturalattacks_rename_scan.py` (this directory) proves it by loading every touched record
# file on BOTH sides, filtering to `#natural<N>`-suffixed entries, and asserting the MULTISET of
# every field OTHER than `id` is identical between the old and new array for that file -- nothing
# added, nothing dropped, nothing changed in content, only which numeric suffix a given attack's
# id carries. The exact, pinned id lists it produces are committed beside this script
# (`structural_diff_naturalattacks_renames.json`); regenerate with the command named in that
# file's own `_command` field.
_NATURALATTACKS_RENAMES_PATH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "structural_diff_naturalattacks_renames.json")


def _load_naturalattacks_renames() -> tuple[frozenset[str], frozenset[str], frozenset[str]]:
    try:
        with open(_NATURALATTACKS_RENAMES_PATH, "r", encoding="utf-8") as fh:
            data = json.load(fh)
    except (OSError, json.JSONDecodeError):
        return frozenset(), frozenset(), frozenset()
    old_ids = data.get("old_ids", [])
    new_ids = data.get("new_ids", [])
    content_shift_ids = data.get("content_shift_ids", [])
    assert len(old_ids) == data.get("_old_count"), f"{_NATURALATTACKS_RENAMES_PATH}: _old_count mismatch -- regenerate with _command"
    assert len(new_ids) == data.get("_new_count"), f"{_NATURALATTACKS_RENAMES_PATH}: _new_count mismatch -- regenerate with _command"
    assert len(content_shift_ids) == data.get("_content_shift_count"), f"{_NATURALATTACKS_RENAMES_PATH}: _content_shift_count mismatch -- regenerate with _command"
    return frozenset(old_ids), frozenset(new_ids), frozenset(content_shift_ids)


NATURALATTACKS_RENAMED_OLD_IDS, NATURALATTACKS_RENAMED_NEW_IDS, NATURALATTACKS_CONTENT_SHIFT_IDS = _load_naturalattacks_renames()
# The only fields a `#natural<N>` rename ever touches on a shared id (an attack's own printed
# text and its dice/value) -- an id in NATURALATTACKS_CONTENT_SHIFT_IDS with any OTHER field
# delta (a `when` gate, `subject`, `print`, `granted_by`, ...) is not covered and still gates.
_NATURALATTACKS_CONTENT_SHIFT_ALLOWED_FIELDS = {"label", "value"}


def edge_diff(old_list: object, new_list: object) -> tuple[list[str], list[str]]:
    """Diff two `granted_by`/`grants` lists as SETS of edges (each edge serialized to a stable
    JSON key), never by length or shallow equality. Returns `(removed, added)` -- both sorted --
    where `removed` is the gate-relevant half: an edge/grant present on the baseline side and
    absent on the fresh side, whether it was dropped outright or its content was replaced by a
    different value (a replacement is a removal of the old key plus an addition of the new one,
    and the removal half is what SS3.5 never allows)."""
    old_keys = {json.dumps(e, sort_keys=True) for e in (old_list or [])}
    new_keys = {json.dumps(e, sort_keys=True) for e in (new_list or [])}
    return sorted(old_keys - new_keys), sorted(new_keys - old_keys)


def _fact_of(effect: object) -> object:
    """The `Fact` a `grants` entry (an `Effect`) carries, unwrapping `GatedFactGrant` -- the
    gating wrapper F1-2 adds is not part of what was granted, only when."""
    if not isinstance(effect, dict):
        return None
    if "FactGrant" in effect:
        return effect["FactGrant"]
    gated = effect.get("GatedFactGrant")
    if isinstance(gated, dict):
        return gated.get("fact")
    return None


def grant_signature(effect: object) -> tuple:
    """A `grants` entry reduced to `(fact kind, target)` -- gating-wrapper-agnostic for GROUPING
    purposes (F1-2: `FactGrant` vs `GatedFactGrant` around the same fact share a signature) and,
    for a `Proficiency` fact, tag-case-insensitive with `WeaponGroup`/`WeaponTag`/`WeaponAllOf`/
    `WeaponSet` all folded to the same `("prof_tag", <lowercased tag text>)` shape (F1-3: a bare
    tag expanding to a `WeaponSet`/`WeaponAllOf` naming the identical tag shares a signature).
    Anything this function does not recognize (a non-`Proficiency` fact, a `Weapon`/`ArmorGroup`/
    `ShieldGroup`/`Chosen`/`DeityFavoredWeapon` proficiency, or a non-fact `Effect` such as
    `FactDeclare`) falls back to an exact serialized match -- unrecognized shapes get no
    transformation leniency, only the two named above do.

    Signature EQUALITY alone is never enough to call a fresh grant a non-removal, though: it is
    the necessary first half of [`grant_covers`]'s check, which also re-tests the gate and, for a
    `WeaponSet`, the member list -- see that function's docstring (SD-36 Epic F1 re-check round
    2, finding 2: this function alone folded a gate deletion and a `WeaponSet` member swap or
    wipe to "no change", since it looks at target identity only, never at whether the gate or the
    member content survived)."""
    fact = _fact_of(effect)
    if not isinstance(fact, dict):
        return ("raw", json.dumps(effect, sort_keys=True))
    prof = fact.get("Proficiency") if isinstance(fact, dict) else None
    if isinstance(prof, dict):
        if "WeaponGroup" in prof and isinstance(prof["WeaponGroup"], str):
            return ("prof_tag", prof["WeaponGroup"].lower())
        if "WeaponTag" in prof and isinstance(prof["WeaponTag"], str):
            return ("prof_tag", prof["WeaponTag"].lower())
        if "WeaponAllOf" in prof and isinstance(prof["WeaponAllOf"], list):
            return ("prof_tag", ".".join(str(t) for t in prof["WeaponAllOf"]).lower())
        if "WeaponSet" in prof and isinstance(prof["WeaponSet"], dict):
            return ("prof_tag", str(prof["WeaponSet"].get("label", "")).lower())
        return ("prof_other", json.dumps(prof, sort_keys=True))
    return ("fact", json.dumps(fact, sort_keys=True))


def _gate_of(effect: object) -> str | None:
    """`None` for a bare `FactGrant` (ungated); the serialized `when` expression for a
    `GatedFactGrant`. Two gated grants compare equal here only when their `when` expressions are
    identical -- a gate that survives with DIFFERENT wording is not the same condition, even
    though `grant_signature` (target-only) cannot tell them apart."""
    if isinstance(effect, dict):
        gated = effect.get("GatedFactGrant")
        if isinstance(gated, dict):
            return json.dumps(gated.get("when"), sort_keys=True)
    return None


def _weapon_set_of(fact: object) -> dict | None:
    if not isinstance(fact, dict):
        return None
    prof = fact.get("Proficiency")
    if isinstance(prof, dict) and isinstance(prof.get("WeaponSet"), dict):
        return prof["WeaponSet"]
    return None


def grant_covers(old_effect: object, new_effect: object) -> bool:
    """Whether `new_effect` is an allowed EVOLUTION of `old_effect`, never a genuine content
    loss, under SS3.5's two sanctioned transformations plus the two directions/leniencies those
    transformations actually need (SD-36 Epic F1 re-check round 2, finding 2 -- the prior
    `grant_signature`-only check was wrapper-agnostic and label-only in BOTH directions, so it
    could not fail on either mutation below even though neither is a shape F1's own work
    produces):

      - target: `grant_signature(old_effect) == grant_signature(new_effect)` (unchanged from
        before -- a different target, under the same or a different shape, is never covered).
      - gate: a bare baseline grant (`old_effect` carries no `when`) may cover a gated OR a bare
        fresh grant -- F1-2's sanctioned wrap goes bare -> gated, never the other direction. A
        GATED baseline grant covers only a fresh grant carrying the SAME `when` expression:
        `GatedFactGrant -> FactGrant` (the gate deleted) and a `when` swapped for a different one
        between two gated grants are both a removal of the baseline's own condition, never an
        allowed evolution.
      - `WeaponSet` content: when BOTH sides resolve a `WeaponSet` fact under the folded label,
        the baseline's member list must be a SUBSET of the fresh one -- F1-3's bare-tag ->
        `WeaponSet` expansion still covers (the baseline side then carries no member list at all,
        so this check does not apply), and a real oracle-driven set GROWTH still covers, but a
        replaced or emptied member list under the same unchanged label does not.
    """
    if grant_signature(old_effect) != grant_signature(new_effect):
        return False
    old_gate = _gate_of(old_effect)
    if old_gate is not None and old_gate != _gate_of(new_effect):
        return False
    old_ws = _weapon_set_of(_fact_of(old_effect))
    new_ws = _weapon_set_of(_fact_of(new_effect))
    if old_ws is not None and new_ws is not None:
        old_members = set(old_ws.get("members") or [])
        new_members = set(new_ws.get("members") or [])
        if not old_members.issubset(new_members):
            return False
    return True


def _fact_shape_of(effect: object) -> str:
    """The literal shape key of a grant's fact -- e.g. `"WeaponGroup"`, `"WeaponTag"`,
    `"WeaponAllOf"`, `"WeaponSet"` for a `Proficiency`, or the fact's own dict key otherwise.
    Unlike `grant_signature`, this does NOT fold the four `Proficiency` spellings together: it
    exists so `_grants_are_duplicates` can require the SAME shape as well as the same folded
    signature (SD-36 Epic F1 polish backlog item 3) -- `grant_signature`'s fold is deliberately
    coarser (SS3.5 sanctions a bare tag EXPANDING to a `WeaponSet` naming the same tag as a
    non-duplicate evolution, handled separately by `grant_covers`), but two BASELINE grants of
    different shapes (e.g. `WeaponGroup("a")` and `WeaponTag("a")`) are not the same fact
    restated twice and must never collapse as if they were."""
    fact = _fact_of(effect)
    if not isinstance(fact, dict):
        return "raw"
    prof = fact.get("Proficiency") if isinstance(fact, dict) else None
    if isinstance(prof, dict):
        for key in ("WeaponGroup", "WeaponTag", "WeaponAllOf", "WeaponSet"):
            if key in prof:
                return f"Proficiency.{key}"
        return "Proficiency.other:" + json.dumps(prof, sort_keys=True)
    return "fact:" + json.dumps(fact, sort_keys=True)


def _grants_are_duplicates(a: object, b: object) -> bool:
    """Whether two BASELINE grants restate the identical content -- the same target signature,
    the same FACT SHAPE (not merely the same folded signature -- SD-36 Epic F1 polish backlog
    item 3: `grant_signature` deliberately folds `WeaponGroup`/`WeaponTag`/`WeaponAllOf`/
    `WeaponSet` of the same tag text to one `("prof_tag", ...)` value for GROUPING and
    `grant_covers` purposes, so a cross-shape baseline pair like `WeaponGroup("a")` +
    `WeaponTag("a")` must not collapse here even though they share a signature), the same gate,
    and (for a `WeaponSet`) the identical member list, never merely an overlapping one -- so
    collapsing them loses nothing (SD-36 Epic F1 re-check round 3, finding 1). A pair that only
    SHARES a signature but genuinely differs (a different shape, a different gate, or a
    `WeaponSet` with a different member list) is never a duplicate here; it stays two distinct
    entries so a real drop of one of them cannot hide behind the other. Tag-case variants of the
    SAME shape (e.g. two `WeaponGroup` grants differing only in book capitalization, such as
    `picaroon_weapon_proficiency`'s three grants) still collapse, since `_fact_shape_of` is
    case-preserving on the KEY, not the tag text, and `grant_signature` already lowercases the
    tag text for the equality check above."""
    if grant_signature(a) != grant_signature(b) or _fact_shape_of(a) != _fact_shape_of(b) or _gate_of(a) != _gate_of(b):
        return False
    a_ws = _weapon_set_of(_fact_of(a))
    b_ws = _weapon_set_of(_fact_of(b))
    if a_ws is None and b_ws is None:
        return True
    a_members = set((a_ws or {}).get("members") or [])
    b_members = set((b_ws or {}).get("members") or [])
    return a_members == b_members


def _old_effects_deduplicated(old_effects: list) -> list:
    """Collapse an exact-duplicate run within the BASELINE side itself, before it is ever matched
    against the fresh side (SD-36 Epic F1 re-check round 3, finding 1): several baseline grants
    that restate the identical content under different book capitalization (e.g. three
    `picaroon_weapon_proficiency` bare `WeaponGroup` grants -- two byte-identical, the third a
    case variant -- that F1-3 folds to the ONE fresh `WeaponSet` grant naming them all) are the
    SAME proficiency stated more than once, not several proficiencies; reading that fold as
    `removed grants: 2` calls a real dedup a loss. Keeps one representative per
    [`_grants_are_duplicates`] equivalence class, in first-seen order."""
    reps: list = []
    for effect in old_effects:
        if not any(_grants_are_duplicates(effect, rep) for rep in reps):
            reps.append(effect)
    return reps


def _max_bipartite_match(old_effects: list, new_effects: list) -> set[int]:
    """The maximum 1:1 matching between baseline (`old_effects`, already duplicate-collapsed)
    and fresh effects under [`grant_covers`] as the edge predicate -- Kuhn's augmenting-path
    algorithm, not a single greedy left-to-right scan (SD-36 Epic F1 re-check round 3, finding
    1): the greedy scan was order-dependent, so an identical MULTISET of grants presented in a
    different order on the fresh side (a bare and a gated grant of one signature, swapped) could
    report a spurious loss when the greedy scan's first pick used up a fresh grant a LATER
    baseline grant needed instead. An augmenting-path search can always re-route an earlier
    match to make room, so it finds the true maximum regardless of list order. Returns the set
    of OLD indices the matching covers."""
    adj = [[j for j, new in enumerate(new_effects) if grant_covers(old, new)] for old in old_effects]
    match_of_new: dict[int, int] = {}

    def try_match(i: int, visited: set[int]) -> bool:
        for j in adj[i]:
            if j in visited:
                continue
            visited.add(j)
            if j not in match_of_new or try_match(match_of_new[j], visited):
                match_of_new[j] = i
                return True
        return False

    for i in range(len(old_effects)):
        try_match(i, set())
    return set(match_of_new.values())


def missing_grant_signatures(old_list: object, new_list: object) -> list[tuple[tuple, int]]:
    """Every baseline grant (after [`_old_effects_deduplicated`] collapses exact duplicates)
    with no covering fresh grant left after [`_max_bipartite_match`]'s real maximum match --
    [`grant_covers`] decides coverage, not bare signature equality -- grouped by signature as
    `(signature, missing_count)` for reporting (SD-36 Epic F1 re-check rounds 2 and 3, finding
    2 then finding 1), so a grant dropped from a rule that still carries an unrelated grant of
    the matching signature is still named (F1 re-check round 1, finding 2's own gate contract: a
    genuine loss must never hide behind an untouched sibling grant), so is a gate deleted or a
    `WeaponSet` emptied/replaced under an unchanged label and target (round 2), and an
    exact-duplicate collapse on the baseline side itself is never miscounted as that same kind
    of loss (round 3)."""
    old_effects = _old_effects_deduplicated(list(old_list or []))
    new_effects = list(new_list or [])
    matched = _max_bipartite_match(old_effects, new_effects)
    missing: Counter = Counter()
    for i, old in enumerate(old_effects):
        if i not in matched:
            missing[grant_signature(old)] += 1
    return sorted(missing.items())


def load_tree(root: str) -> dict[str, object]:
    """Every rule file under `root` (`<book>/<kind>/<key>.json`) as `{relpath: [rule dict, ...]}`.
    `_vars/`, `_defects/`, `_report.json`, et. are walked too but kept as opaque byte-diff targets
    (they are census/side-channel files, not the rule-id-keyed package the diff contract governs).
    """
    rule_files: dict[str, list] = {}
    other_files: dict[str, bytes] = {}
    for dirpath, _dirnames, filenames in os.walk(root):
        for fn in filenames:
            if fn in ("GENERATED",):
                continue
            full = os.path.join(dirpath, fn)
            rel = os.path.relpath(full, root).replace(os.sep, "/")
            if rel.startswith("_vars/") or rel.startswith("_defects/") or rel in ("_refused.json", "_tokens.json", "_report.json"):
                with open(full, "rb") as fh:
                    other_files[rel] = fh.read()
                continue
            try:
                with open(full, "r", encoding="utf-8") as fh:
                    data = json.load(fh)
            except (OSError, json.JSONDecodeError):
                continue
            if isinstance(data, list):
                rule_files[rel] = data
    return {"rule_files": rule_files, "other_files": other_files}


def rules_by_id(rule_files: dict[str, list]) -> dict[str, dict]:
    out: dict[str, dict] = {}
    for rules in rule_files.values():
        for r in rules:
            if isinstance(r, dict) and "id" in r:
                out[r["id"]] = r
    return out


def kind_of(rule_id: str) -> str:
    parts = rule_id.split(":")
    return parts[1] if len(parts) >= 2 else "?"


def diff_rule(old: dict, new: dict) -> list[str]:
    """Field names OTHER than the two allowed-to-grow fields that differ between the two rule
    dicts (shallow -- SS3.5 wants a NAMED field, not a value diff; a deep JSON diff is a separate,
    noisier tool). `granted_by`/`grants` are excluded here -- they get their own set-based
    `edge_diff` treatment below, since a length-preserving replacement inside either field would
    otherwise pass this shallow-inequality check silently only when nothing else changed, or be
    lumped in as an "unexpected field delta" when something else did, neither of which names the
    edge/grant that was actually removed."""
    keys = (set(old.keys()) | set(new.keys())) - ALLOWED_GROWING_FIELDS
    return sorted(k for k in keys if old.get(k) != new.get(k))


def _provenance_delta_is_closure_rows_growth_only(old_prov: object, new_prov: object) -> bool:
    """Whether a `provenance` delta on a pinned record matches the narrow contract
    `structural_diff_expected_provenance_deltas.json` itself states: this grows ONLY
    `provenance.closure_rows` (no other field). The pinned-record allowance in `main()` used to
    accept ANY change to the whole `provenance` object once `rid` was on the pinned list --
    including a corrupted `book`, `oracle_pin` or `converter_version`, or `closure_rows`
    SHRINKING (SD-36 Epic F1 polish backlog item 2). Requires BOTH sides to be dicts, the set of
    differing subkeys to be exactly `{"closure_rows"}`, and the baseline `closure_rows` list to
    be a subset of the fresh one -- anything else (a different differing subkey, or a
    `closure_rows` value that is not a superset) is not covered and falls through to gate like
    any other field delta."""
    if not isinstance(old_prov, dict) or not isinstance(new_prov, dict):
        return False
    keys = (set(old_prov.keys()) | set(new_prov.keys()))
    differing = {k for k in keys if old_prov.get(k) != new_prov.get(k)}
    if differing != {"closure_rows"}:
        return False
    old_rows = set(old_prov.get("closure_rows") or [])
    new_rows = set(new_prov.get("closure_rows") or [])
    return old_rows.issubset(new_rows)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("scratch_dir", help="a directory `sheet_rule_convert --dump <dir>` wrote (never data/sheet_rules)")
    ap.add_argument("--baseline", default="data/sheet_rules", help="the tracked on-disk package (default: data/sheet_rules)")
    ap.add_argument("--max-examples", type=int, default=10)
    ap.add_argument(
        "--report-only",
        action="store_true",
        help="keep the pre-gate behavior: always print the report and exit 0, for exploration",
    )
    args = ap.parse_args()

    baseline_root = os.path.abspath(args.baseline)
    scratch_root = os.path.abspath(args.scratch_dir)
    if baseline_root == scratch_root:
        print("refusing: scratch_dir must not be the tracked baseline itself", file=sys.stderr)
        return 2

    base = load_tree(baseline_root)
    fresh = load_tree(scratch_root)

    base_paths = set(base["rule_files"]) | set(base["other_files"])
    fresh_paths = set(fresh["rule_files"]) | set(fresh["other_files"])
    added_paths = sorted(fresh_paths - base_paths)
    removed_paths = sorted(base_paths - fresh_paths)

    added_vars = [p for p in added_paths if p.startswith("_vars/")]
    added_defects = [p for p in added_paths if p.startswith("_defects/")]
    added_rule_files = [p for p in added_paths if p in fresh["rule_files"]]
    added_other = [p for p in added_paths if p not in added_vars and p not in added_defects and p not in added_rule_files]
    removed_vars = [p for p in removed_paths if p.startswith("_vars/")]
    removed_defects = [p for p in removed_paths if p.startswith("_defects/")]
    removed_rule_files = [p for p in removed_paths if p in base["rule_files"]]
    removed_other = [p for p in removed_paths if p not in removed_vars and p not in removed_defects and p not in removed_rule_files]

    base_rules = rules_by_id(base["rule_files"])
    fresh_rules = rules_by_id(fresh["rule_files"])
    base_ids = set(base_rules)
    fresh_ids = set(fresh_rules)
    added_rule_ids_raw = sorted(fresh_ids - base_ids)
    removed_rule_ids_raw = sorted(base_ids - fresh_ids)
    # SD-36 Epic F1 stage 5: the NATURALATTACKS suffix-fix rename -- an exact, pinned id list
    # (see the module-level comment above), never a blanket allowance. Anything off these two
    # pinned sets still gates as an ordinary added/removed rule id.
    naturalattacks_removed_ids = sorted(set(removed_rule_ids_raw) & NATURALATTACKS_RENAMED_OLD_IDS)
    naturalattacks_added_ids = sorted(set(added_rule_ids_raw) & NATURALATTACKS_RENAMED_NEW_IDS)
    added_rule_ids = sorted(set(added_rule_ids_raw) - NATURALATTACKS_RENAMED_NEW_IDS)
    removed_rule_ids = sorted(set(removed_rule_ids_raw) - NATURALATTACKS_RENAMED_OLD_IDS)

    unexpected_field_deltas: list[tuple[str, str]] = []
    expected_provenance_deltas: list[str] = []
    naturalattacks_content_shift_deltas: list[tuple[str, str]] = []
    expected_bonus_var_split_deltas: list[tuple[str, str]] = []
    added_edges_by_target_kind: Counter[str] = Counter()
    added_edges_total = 0
    removed_granted_by: list[tuple[str, str]] = []
    removed_grants: list[tuple[str, tuple, int]] = []
    added_grants_total = 0
    for rid in sorted(base_ids & fresh_ids):
        old, new = base_rules[rid], fresh_rules[rid]
        for field in diff_rule(old, new):
            # SD-36 Epic F1 re-check round 3, finding 1 (ORCHESTRATOR RULING): a `provenance`
            # delta is expected ONLY for a record on the pinned, generated list -- never a
            # blanket allowance for the field. A record off that list, or any OTHER field on
            # ANY record (including one on the list), still gates.
            if field == "provenance" and rid in EXPECTED_PROVENANCE_DELTA_RECORDS and _provenance_delta_is_closure_rows_growth_only(
                old.get("provenance"), new.get("provenance")
            ):
                expected_provenance_deltas.append(rid)
                continue
            # SD-36 Epic F1 stage 5: a `#natural<N>` id the old and new numbering both happen
            # to reuse for different attack content (module-level comment above) -- pinned by
            # id AND restricted to the two fields that rename can ever touch; any other field
            # delta on this same id, or any delta on an id off the pinned list, still gates.
            if rid in NATURALATTACKS_CONTENT_SHIFT_IDS and field in _NATURALATTACKS_CONTENT_SHIFT_ALLOWED_FIELDS:
                naturalattacks_content_shift_deltas.append((rid, field))
                continue
            # SD-36 Epic F1 stage 6 (merge-readiness blocker 2): a field the comma-split
            # BONUS:VAR index fix (stage5 receipt §1a) newly resolves through to a real,
            # multi-contribution Var/Compare{Var} -- pinned by EXACT (rule id, field) pair (see
            # the module-level comment above); any other field delta on this same id, or any
            # delta on a pair off the pinned list, still gates.
            if (rid, field) in EXPECTED_BONUS_VAR_SPLIT_FIELD_DELTAS:
                expected_bonus_var_split_deltas.append((rid, field))
                continue
            unexpected_field_deltas.append((rid, field))

        edges_removed, edges_added = edge_diff(old.get("granted_by"), new.get("granted_by"))
        for e in edges_removed:
            removed_granted_by.append((rid, e))
        if edges_added:
            added_edges_by_target_kind[kind_of(rid)] += len(edges_added)
            added_edges_total += len(edges_added)

        for sig, missing_n in missing_grant_signatures(old.get("grants"), new.get("grants")):
            removed_grants.append((rid, sig, missing_n))
        _, grants_added_raw = edge_diff(old.get("grants"), new.get("grants"))
        added_grants_total += len(grants_added_raw)

    base_report = json.loads(base["other_files"].get("_report.json", b"{}") or b"{}")
    fresh_report = json.loads(fresh["other_files"].get("_report.json", b"{}") or b"{}")
    moved_counts = [
        (key, base_report.get(key), fresh_report.get(key))
        for key in ("records", "converted")
        if base_report.get(key) != fresh_report.get(key)
    ]

    print("== file set ==")
    print(f"  rule files:  +{len(added_rule_files)} -{len(removed_rule_files)}  (+{len(added_rule_ids)} -{len(removed_rule_ids)} rule ids)")
    print(f"  _vars/:      +{len(added_vars)} -{len(removed_vars)}")
    print(f"  _defects/:   +{len(added_defects)} -{len(removed_defects)}")
    print(f"  other:       +{len(added_other)} -{len(removed_other)}  {added_other + removed_other}")
    print()
    print("== counts (baseline -> fresh) ==")
    for key in ("records", "converted", "refused", "rules_written", "var_tables"):
        marker = "  *** MOVED ***" if key in ("records", "converted") and base_report.get(key) != fresh_report.get(key) else ""
        print(f"  {key}: {base_report.get(key)} -> {fresh_report.get(key)}{marker}")
    print()
    print("== added granted_by edges by TARGET kind ==")
    for kind, n in sorted(added_edges_by_target_kind.items(), key=lambda kv: (-kv[1], kv[0])):
        print(f"  {kind}: {n}")
    print(f"  TOTAL: {added_edges_total}")
    print(f"  added grants (total): {added_grants_total}")
    print()
    print("== expected delta classes (named, counted, explained -- never a blanket allowance) ==")
    print(f"  added granted_by edges: {added_edges_total}  (F1's own declared purpose -- a bare AUTO reference now resolving through its parent ability category; SS3.5 permits growth)")
    print(f"  added grants: {added_grants_total}  (F1-2 GatedFactGrant wrapping / F1-3 WeaponSet expansion of a previously-bare selector; see grant_signature/grant_covers)")
    print(f"  added _vars/ tables: {len(added_vars)}  (condition variables an added GatedFactGrant's `when` now references)")
    print(f"  added _defects/ files: {len(added_defects)}")
    print(f"  provenance deltas on the pinned current_class-fix record list: {len(expected_provenance_deltas)} of {len(EXPECTED_PROVENANCE_DELTA_RECORDS)} pinned records (see structural_diff_expected_provenance_deltas.json)")
    print(
        f"  naturalattacks suffix-fix rename: {len(naturalattacks_removed_ids)} of {len(NATURALATTACKS_RENAMED_OLD_IDS)} "
        f"pinned old ids -> {len(naturalattacks_added_ids)} of {len(NATURALATTACKS_RENAMED_NEW_IDS)} pinned new ids "
        f"(net +{len(naturalattacks_added_ids) - len(naturalattacks_removed_ids)}), plus "
        f"{len(naturalattacks_content_shift_deltas)} field deltas on {len(set(r for r, _ in naturalattacks_content_shift_deltas))} "
        f"of {len(NATURALATTACKS_CONTENT_SHIFT_IDS)} pinned same-id content-shift ids "
        f"(see structural_diff_naturalattacks_renames.json -- content-preserved per file, not a genuine loss)"
    )
    print(
        f"  bonus_var_index comma-split fix (stage5 §1a): {len(expected_bonus_var_split_deltas)} of "
        f"{len(EXPECTED_BONUS_VAR_SPLIT_FIELD_DELTAS)} pinned (rule id, field) record deltas -- a "
        f"flattened Const/Never/bare-AbilityMod now resolving to its real multi-contribution Var "
        f"(see structural_diff_bonus_var_split_record_deltas.json for the per-family mechanism and "
        f"reachability)"
    )
    if added_rule_ids:
        print(f"  added rule ids: {len(added_rule_ids)}")
        for rid in added_rule_ids[: args.max_examples]:
            cause = KNOWN_ADDED_RULE_CAUSES.get(rid, "cause not yet named -- explain before treating this as expected")
            print(f"    {rid}: {cause}")
        if len(added_rule_ids) > args.max_examples:
            print(f"    ... and {len(added_rule_ids) - args.max_examples} more")
    print()
    print(f"unexpected field deltas: {len(unexpected_field_deltas)}")
    for rid, field in unexpected_field_deltas[: args.max_examples]:
        print(f"  {rid}: {field}")
    if len(unexpected_field_deltas) > args.max_examples:
        print(f"  ... and {len(unexpected_field_deltas) - args.max_examples} more")
    if added_rule_ids:
        print(f"new rule ids: {len(added_rule_ids)} {added_rule_ids[: args.max_examples]}")
    if removed_rule_ids:
        print(f"removed rule ids: {len(removed_rule_ids)} {removed_rule_ids[: args.max_examples]}")
    print()
    print(f"removed granted_by edges: {len(removed_granted_by)}  (never allowed -- SS3.5 permits growth only)")
    for rid, e in removed_granted_by[: args.max_examples]:
        print(f"  {rid}: {e}")
    if len(removed_granted_by) > args.max_examples:
        print(f"  ... and {len(removed_granted_by) - args.max_examples} more")
    print(f"removed grants: {len(removed_grants)}  (a grant SIGNATURE lost with no replacement of the same target -- gating-wrapper and WeaponGroup/WeaponTag/WeaponAllOf/WeaponSet-tag-case changes are NOT counted here, only a genuine loss or a fabricated replacement)")
    for rid, sig, n in removed_grants[: args.max_examples]:
        print(f"  {rid}: {sig} x{n}")
    if len(removed_grants) > args.max_examples:
        print(f"  ... and {len(removed_grants) - args.max_examples} more")

    failures: list[str] = []
    if removed_rule_ids:
        failures.append(f"removed rule ids: {len(removed_rule_ids)}")
    if removed_vars:
        failures.append(f"removed _vars/ files: {len(removed_vars)}")
    if removed_defects:
        failures.append(f"removed _defects/ files: {len(removed_defects)}")
    if unexpected_field_deltas:
        failures.append(f"unexpected field deltas: {len(unexpected_field_deltas)}")
    if removed_granted_by:
        failures.append(f"removed granted_by edges: {len(removed_granted_by)}")
    if removed_grants:
        failures.append(f"removed grants: {len(removed_grants)}")
    for key, old_v, new_v in moved_counts:
        failures.append(f"{key} moved: {old_v} -> {new_v}")

    print()
    if failures:
        print(f"verdict=FAIL ({'; '.join(failures)})")
    else:
        print("verdict=PASS")

    if args.report_only:
        return 0
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
