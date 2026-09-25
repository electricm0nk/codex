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


# SD-36 Epic F1c (defects D1-D6, commits 1e6b2db9ee / 5979ef4668 / e61473e9c9): the F1c converter
# batch's field deltas and added ids, each classified into ONE named mechanism by
# `f1c_delta_pins.py` (this directory; see its docstring) and pinned as exact (rule id, field)
# pairs / exact ids in `structural_diff_f1c_deltas.json`. A pinned pair is accepted only when its
# class's own shape check below ALSO holds on the two records being compared -- never a blanket
# allowance: an off-list pair, or a pinned pair failing its check, still gates.
_F1C_DELTAS_PATH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "structural_diff_f1c_deltas.json")
_LINE_FIELDS = ("value", "target", "bonus_type", "applies")
_MULTISET_FIELDS = ("also", "prose")
F1C_CLASS_CAUSES = {
    "d2_line_split": "D2: a record's first line carried its own condition; the record gets a Text principal with only the record's gates and the line moves, condition intact, to a new #<suffix> sibling",
    "d4_closure_complete": "D4: closure_complete=true attested on a class principal whose whole closure is defect-free (sheet_rule/attest.rs)",
    "d4_pi_reclosure": "D4 soundness: 21 product-identity class records re-keyed from the codex-named placeholder to the class's own base row, so their continuation rows, level lines and .MOD rows convert",
    "d3_unchained_class": "D3: a Pathfinder Unchained class principal (TakenOnClass <base>), one per class-selection ability",
    "d6_weapon_choice": "D6: a CHOOSE:WEAPONPROFICIENCY option list resolved to oracle weapon names, or a pick linked to its child pool's one member",
    "f1c3_preability_bracket": "F1c-3: a PREABILITY `[<key>]` item no longer converts to an unholdable MissingRule alternative",
    "d7_always_held": "D7: always_held=true attested on the principal of a record every character holds unconditionally -- the target of an unconditional ABILITY|AUTOMATIC grant on a STAT/SAVE row (sheet_rule/always_held.rs)",
    "d8_pool_pick": "D8: a record that raises an ability category's POOL variable offers the pick -- offers {id: <the record>, count: Var(<pool variable>), from: Rules {pool, tags}} on its principal (sheet_rule/pool_pick.rs)",
}


def _f1c_ms(v: object) -> Counter:
    return Counter(json.dumps(e, sort_keys=True) for e in (v or []))


def has_bracket_missing_rule(v: object) -> bool:
    """Whether a field's JSON holds a `MissingRule` named `[<key>]` -- the pre-F1c conversion of a
    PREABILITY bracket item as an alternative that can never be held."""
    return '"name": "[' in json.dumps(v, sort_keys=True)


def d2_split_conserves(old: dict, new: dict, sib: dict | None) -> bool:
    """The D2 split's shape: the fresh principal is Text; the new sibling carries the OLD
    principal's line exactly (value/target/bonus_type/applies -- applies may differ only by a
    removed PREABILITY bracket MissingRule, the one composed case); and the `also`/`prose`
    multisets are conserved (old == fresh principal + sibling). Nothing dropped, only moved."""
    if not isinstance(sib, dict) or new.get("value") != "Text":
        return False
    for k in _LINE_FIELDS:
        if sib.get(k) == old.get(k):
            continue
        if k == "applies" and has_bracket_missing_rule(old.get(k)) and not has_bracket_missing_rule(sib.get(k)):
            continue
        return False
    return all(_f1c_ms(old.get(f)) == _f1c_ms(new.get(f)) + _f1c_ms(sib.get(f)) for f in _MULTISET_FIELDS)


def d6_shape(old: object, new: object) -> str | None:
    """`resolved` when a Weapons option list is rewritten to (other) weapon names, `linked` when
    an absent offer becomes a `Rules {pool, tags}` link; anything else is not the D6 class."""
    of = old.get("from") if isinstance(old, dict) else None
    nf = new.get("from") if isinstance(new, dict) else None
    if isinstance(of, dict) and isinstance(nf, dict) and "Weapons" in of and "Weapons" in nf:
        return "resolved"
    if old is None and isinstance(nf, dict) and "Rules" in nf:
        return "linked"
    return None


def d8_shape(rid: str, old: object, new: object, rule: dict | None) -> bool:
    """D8: an absent offer becomes the record's own pool pick -- `id` the rule itself, `count`
    exactly one variable (the pool variable), `from` a `Rules` set with no extra requirement.
    D6's `linked` pick is told apart by its count: a D6 link counts the pick rule's own `Pool`
    value (`count == value`), a D8 pick counts the pool variable."""
    if old is not None or not isinstance(new, dict) or "#" in rid:
        return False
    count, frm = new.get("count"), new.get("from")
    target = (rule or {}).get("target")
    if isinstance(target, dict) and "Pool" in target and (rule or {}).get("value") == {"Number": count}:
        return False
    return (
        new.get("id") == rid
        and isinstance(count, dict) and set(count) == {"Var"}
        and isinstance(frm, dict) and set(frm) == {"Rules"}
        and frm["Rules"].get("requires") == "Always"
        and isinstance(frm["Rules"].get("pool"), str) and isinstance(frm["Rules"].get("tags"), list)
    )


def _load_f1c_deltas() -> tuple[dict[tuple[str, str], str], dict[str, str], dict[str, str]]:
    try:
        with open(_F1C_DELTAS_PATH, "r", encoding="utf-8") as fh:
            data = json.load(fh)
    except (OSError, json.JSONDecodeError):
        return {}, {}, {}
    pairs: dict[tuple[str, str], str] = {}
    new_ids: dict[str, str] = {}
    for name, c in data.get("classes", {}).items():
        assert len(c["field_deltas"]) == c["_field_delta_count"], f"{_F1C_DELTAS_PATH}: {name} _field_delta_count mismatch -- regenerate with _command"
        assert len(c["new_rule_ids"]) == c["_new_rule_id_count"], f"{_F1C_DELTAS_PATH}: {name} _new_rule_id_count mismatch -- regenerate with _command"
        for rid, field in c["field_deltas"]:
            assert (rid, field) not in pairs, f"{_F1C_DELTAS_PATH}: ({rid}, {field}) pinned in two classes"
            pairs[(rid, field)] = name
        for rid in c["new_rule_ids"]:
            new_ids[rid] = name
    return pairs, new_ids, dict(data.get("d2_splits", {}))


F1C_FIELD_DELTA_CLASS, F1C_NEW_RULE_ID_CLASS, F1C_D2_SPLITS = _load_f1c_deltas()
KNOWN_ADDED_RULE_CAUSES.update({rid: F1C_CLASS_CAUSES[name] for rid, name in F1C_NEW_RULE_ID_CLASS.items()})


def f1c_delta_holds(name: str, rid: str, field: str, old: dict, new: dict, fresh_rules: dict[str, dict]) -> bool:
    """Re-run the pinned class's own shape check on the two records actually being compared."""
    o, n = old.get(field), new.get(field)
    if name == "d2_line_split":
        return d2_split_conserves(old, new, fresh_rules.get(F1C_D2_SPLITS.get(rid, "")))
    if name == "d4_closure_complete":
        return o is None and n is True
    if name == "d4_pi_reclosure":
        return field != "provenance" or _provenance_delta_is_closure_rows_growth_only(o, n)
    if name == "d6_weapon_choice":
        return d6_shape(o, n) is not None
    if name == "f1c3_preability_bracket":
        return has_bracket_missing_rule(o) and not has_bracket_missing_rule(n)
    if name == "d7_always_held":
        return field == "always_held" and o is None and n is True and "#" not in rid
    if name == "d8_pool_pick":
        return field == "offers" and d8_shape(rid, o, n, new)
    return False


# SD-36 Epic F3b2 (converter step on sd36/epic-f2-f3): the F3b2 package delta classes against the
# tranche/16 package, each classified by `f3b2_delta_pins.py` (this directory) and pinned as exact
# (rule id, field, pinned value) triples in `structural_diff_f3b2_deltas.json`. A pinned pair is
# accepted only when its class's own shape check holds on the two records being compared AND the
# fresh field equals the pinned value (the skill-ranks number, `true`, or the fresh field's
# sha256). An off-list pair, a failed shape, or a moved value still gates.
_F3B2_DELTAS_PATH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "structural_diff_f3b2_deltas.json")
F3B2_CLASS_CAUSES = {
    "f3b2_skill_ranks": "F3b2 (1): a class line's STARTSKILLPTS is written as the principal's StatBlock \"Skill ranks per level\" prose row (one added row, a plain number)",
    "f3b2_closure_complete": "F3b2 (2): closure_complete=true attested on a class principal whose closure is now defect-free because its references to placeholder-keyed records resolve",
    "f3b2_placeholder_key_resolved": "F3b2 (2): a reference to a product-identity record by the KEY its oracle row declares resolves (the record was indexed only under its codex-named placeholder corpus key): MissingRule -> Rule, nothing else in the field moves",
}
F3B2_SKILL_LABEL = {"StatBlock": "Skill ranks per level"}


def f3b2_field_sha(v: object) -> str:
    import hashlib

    return hashlib.sha256(json.dumps(v, sort_keys=True).encode("utf-8")).hexdigest()


def f3b2_skill_ranks_row(old_prose: object, new_prose: object) -> str | None:
    """The added row's number when the fresh prose is the old prose plus exactly one
    `StatBlock "Skill ranks per level"` row of one plain-digit Text piece, and the old prose had
    no such row; else None."""
    old_ms, new_ms = _f1c_ms(old_prose), _f1c_ms(new_prose)
    if any(json.loads(k).get("family") == F3B2_SKILL_LABEL for k in old_ms):
        return None
    added = new_ms - old_ms
    if old_ms - new_ms or sum(added.values()) != 1:
        return None
    row = json.loads(next(iter(added)))
    pieces = row.get("pieces")
    if row.get("family") != F3B2_SKILL_LABEL or not isinstance(pieces, list) or len(pieces) != 1:
        return None
    text = pieces[0].get("Text") if isinstance(pieces[0], dict) else None
    return text if isinstance(text, str) and text.isdigit() else None


def f3b2_missing_to_rule(old: object, new: object) -> int:
    """How many `{"MissingRule": ...}` nodes of `old` became `{"Rule": <id>}` in `new`, when the two
    are otherwise identical; -1 when anything else differs."""
    if isinstance(old, dict) and isinstance(new, dict):
        if set(old) == {"MissingRule"} and set(new) == {"Rule"} and isinstance(new["Rule"], str):
            return 1
        if set(old) != set(new):
            return -1
        total = 0
        for k in old:
            n = f3b2_missing_to_rule(old[k], new[k])
            if n < 0:
                return -1
            total += n
        return total
    if isinstance(old, list) and isinstance(new, list):
        if len(old) != len(new):
            return -1
        total = 0
        for a, b in zip(old, new):
            n = f3b2_missing_to_rule(a, b)
            if n < 0:
                return -1
            total += n
        return total
    return 0 if old == new else -1


# SD-36 Epic F3b2b (converter step 2 on sd36/epic-f2-f3): the F3b2b package delta classes
# against the tranche/16 package, classified by `f3b2b_delta_pins.py` (this directory) and pinned as
# exact (rule id, field, pinned value) triples in `structural_diff_f3b2b_deltas.json`, with the same
# acceptance rule as F3b2: the class shape holds on the two records AND the fresh value equals the
# pin. Checked before the F3b2 pins (a pair F3b2b moved is attributed to F3b2b).
_F3B2B_DELTAS_PATH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "structural_diff_f3b2b_deltas.json")
F3B2B_CLASS_CAUSES = {
    "f3b2b_undeclared_note": "F3b2b (1): provenance gains undeclared_in_pinned_tree -- the variables the record reads that no row of the pinned tree declares and that are not oracle built-in terms, read as Const(0), the oracle's own value (VariableProcessor.java:394-402); nothing else in provenance moves",
    "f3b2b_closure_complete": "F3b2b (1)(2): closure_complete=true attested on a class principal whose closure is now defect-free -- its undeclared-variable reads are the oracle's 0, not closure defects, or its twin-printing reference resolves to the newest printing",
}


def f3b2b_undeclared_note(old: object, new: object) -> list[str] | None:
    """The added `undeclared_in_pinned_tree` list when `new` is `old` plus exactly that key (a
    non-empty, sorted, duplicate-free list of names); else None."""
    if not isinstance(old, dict) or not isinstance(new, dict) or "undeclared_in_pinned_tree" in old:
        return None
    names = new.get("undeclared_in_pinned_tree")
    if not isinstance(names, list) or not names or not all(isinstance(x, str) and x for x in names):
        return None
    if names != sorted(set(names)):
        return None
    rest = {k: v for k, v in new.items() if k != "undeclared_in_pinned_tree"}
    return names if rest == old else None


def f3b2b_classify(rid: str, field: str, old: dict, new: dict) -> tuple[str, object] | None:
    """The one F3b2b class a (rule id, field) delta belongs to, with the value to pin; None when it
    fits none."""
    o, n = old.get(field), new.get(field)
    if field == "provenance":
        names = f3b2b_undeclared_note(o, n)
        if names is not None:
            return "f3b2b_undeclared_note", names
    if field == "closure_complete" and o is None and n is True and "#" not in rid and kind_of(rid) == "class":
        return "f3b2b_closure_complete", True
    return None


def _load_f3b2b_deltas() -> dict[tuple[str, str], tuple[str, object]]:
    try:
        with open(_F3B2B_DELTAS_PATH, "r", encoding="utf-8") as fh:
            data = json.load(fh)
    except (OSError, json.JSONDecodeError):
        return {}
    pins: dict[tuple[str, str], tuple[str, object]] = {}
    for name, c in data.get("classes", {}).items():
        assert name in F3B2B_CLASS_CAUSES, f"{_F3B2B_DELTAS_PATH}: unknown class {name}"
        assert len(c["pins"]) == c["_count"], f"{_F3B2B_DELTAS_PATH}: {name} _count mismatch -- regenerate with _command"
        for rid, field, value in c["pins"]:
            assert (rid, field) not in pins, f"{_F3B2B_DELTAS_PATH}: ({rid}, {field}) pinned twice"
            pins[(rid, field)] = (name, value)
    return pins


F3B2B_PINS = _load_f3b2b_deltas()


def _load_f3b2b_required_edges() -> list[tuple[str, str]]:
    """The `granted_by` edges the F3b2b step added (a reference that now resolves), each pinned as
    (target rule id, edge key): growth is otherwise unguarded, so losing one of these must gate."""
    try:
        with open(_F3B2B_DELTAS_PATH, "r", encoding="utf-8") as fh:
            data = json.load(fh)
    except (OSError, json.JSONDecodeError):
        return []
    edges = data.get("required_added_edges", {})
    pins = [(rid, key) for rid, key in edges.get("pins", [])]
    assert len(pins) == edges.get("_count", 0), f"{_F3B2B_DELTAS_PATH}: required_added_edges _count mismatch -- regenerate with _command"
    return pins


F3B2B_REQUIRED_EDGES = _load_f3b2b_required_edges()


def f3b2b_delta_holds(rid: str, field: str, old: dict, new: dict) -> str | None:
    """The pinned F3b2b class when (rid, field) is pinned, its shape holds and the fresh value is
    the pinned one; else None."""
    pinned = F3B2B_PINS.get((rid, field))
    if pinned is None:
        return None
    got = f3b2b_classify(rid, field, old, new)
    return pinned[0] if got == pinned else None


def f3b2_classify(rid: str, field: str, old: dict, new: dict) -> tuple[str, object] | None:
    """The one F3b2 class a (rule id, field) delta belongs to, with the value to pin; None when it
    fits none."""
    o, n = old.get(field), new.get(field)
    if field == "prose":
        ranks = f3b2_skill_ranks_row(o, n)
        if ranks is not None and "#" not in rid:
            return "f3b2_skill_ranks", ranks
    if field == "closure_complete" and o is None and n is True and "#" not in rid and kind_of(rid) == "class":
        return "f3b2_closure_complete", True
    if field in ("applies", "prose") and f3b2_missing_to_rule(o, n) > 0:
        return "f3b2_placeholder_key_resolved", f3b2_field_sha(n)
    return None


def _load_f3b2_deltas() -> dict[tuple[str, str], tuple[str, object]]:
    try:
        with open(_F3B2_DELTAS_PATH, "r", encoding="utf-8") as fh:
            data = json.load(fh)
    except (OSError, json.JSONDecodeError):
        return {}
    pins: dict[tuple[str, str], tuple[str, object]] = {}
    for name, c in data.get("classes", {}).items():
        assert name in F3B2_CLASS_CAUSES, f"{_F3B2_DELTAS_PATH}: unknown class {name}"
        assert len(c["pins"]) == c["_count"], f"{_F3B2_DELTAS_PATH}: {name} _count mismatch -- regenerate with _command"
        for rid, field, value in c["pins"]:
            assert (rid, field) not in pins, f"{_F3B2_DELTAS_PATH}: ({rid}, {field}) pinned twice"
            pins[(rid, field)] = (name, value)
    return pins


F3B2_PINS = _load_f3b2_deltas()


def f3b2_delta_holds(rid: str, field: str, old: dict, new: dict) -> str | None:
    """The pinned F3b2 class when (rid, field) is pinned, its shape holds and the fresh value is the
    pinned one; else None."""
    pinned = F3B2_PINS.get((rid, field))
    if pinned is None:
        return None
    got = f3b2_classify(rid, field, old, new)
    return pinned[0] if got == pinned else None


# SD-36 Epic F3c3 (converter step 3): PCGen `SUBCLASS:` lines convert to a class choice (a
# `<class id>#subclass` sibling on the class record) whose options are new `subclass` rules. Every
# rule id this ADDS against tranche/16 is pinned by `f3c3_delta_pins.py` as (class, owning class
# principal, sha256 of the whole rule), so a changed, dropped or unpinned subclass rule gates. Also
# pinned, because growth is otherwise unguarded: the `granted_by` edges the options hand out
# (fresh vs the prior step's package), the `_vars/` contributions they add, and the row counts of
# the two new `_defects/` files. A pin applies only when its owning class principal is in the
# fresh tree as a converted record (it states its oracle pin; the synthetic fixtures of
# structural_diff_test.py carry no such principal).
_F3C3_DELTAS_PATH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "structural_diff_f3c3_deltas.json")
F3C3_CLASS_CAUSES = {
    "f3c3_subclass_choice": "F3c3: a class's SUBCLASS lines become ONE choice sibling <class id>#subclass (offers Rules{pool: subclass, tags: [<Class> Subclass]}, count 1, print false)",
    "f3c3_subclass_option": "F3c3: one subclass rule per SUBCLASS line (and its #<n> line siblings), granted by the class's choice, carrying the line's CSKILL / SUBCLASSLEVEL grants",
}


def f3c3_rule_sha(rule: object) -> str:
    return f3b2_field_sha(rule)


def f3c3_classify_added(rid: str, rule: dict) -> str | None:
    """The F3c3 class of a rule id added against tranche/16, or None."""
    if rid.endswith("#subclass") and kind_of(rid) == "class" and isinstance(rule.get("offers"), dict):
        return "f3c3_subclass_choice"
    if kind_of(rid) == "subclass":
        return "f3c3_subclass_option"
    return None


def _load_f3c3() -> dict:
    try:
        with open(_F3C3_DELTAS_PATH, "r", encoding="utf-8") as fh:
            data = json.load(fh)
    except (OSError, json.JSONDecodeError):
        return {"added_rules": {}, "required_added_edges": [], "var_contributions": [], "defect_rows": {}, "owner": ""}
    added: dict[str, tuple[str, str, str]] = {}
    for name, c in data.get("added_rules", {}).items():
        assert name in F3C3_CLASS_CAUSES, f"{_F3C3_DELTAS_PATH}: unknown class {name}"
        assert len(c["pins"]) == c["_count"], f"{_F3C3_DELTAS_PATH}: {name} _count mismatch -- regenerate with _command"
        for rid, owner, sha in c["pins"]:
            assert rid not in added, f"{_F3C3_DELTAS_PATH}: {rid} pinned twice"
            added[rid] = (name, owner, sha)
    edges = data.get("required_added_edges", {})
    assert len(edges.get("pins", [])) == edges.get("_count", 0), f"{_F3C3_DELTAS_PATH}: required_added_edges _count mismatch"
    contribs = data.get("var_contributions", {})
    assert len(contribs.get("pins", [])) == contribs.get("_count", 0), f"{_F3C3_DELTAS_PATH}: var_contributions _count mismatch"
    return {
        "added_rules": added,
        "required_added_edges": [(rid, key) for rid, key in edges.get("pins", [])],
        "var_contributions": [(rel, key) for rel, key in contribs.get("pins", [])],
        "defect_rows": dict(data.get("defect_rows", {})),
        "owner": data.get("owner", ""),
    }


F3C3 = _load_f3c3()


def f3c3_check(fresh_rules: dict[str, dict], base_rules: dict[str, dict], added_rule_ids: list[str], fresh_other: dict[str, bytes]) -> tuple[Counter, list[str]]:
    """(pinned added rule ids found per class, failure lines) for the F3c3 pins (module comment)."""
    found: Counter = Counter()
    failures: list[str] = []

    def converted_principal(rid: str) -> bool:
        # A real converted principal states its oracle pin; the synthetic fixtures' do not.
        return bool((fresh_rules.get(rid, {}).get("provenance") or {}).get("oracle_pin"))

    for rid in added_rule_ids:
        cls = f3c3_classify_added(rid, fresh_rules[rid])
        pin = F3C3["added_rules"].get(rid)
        if pin is None:
            if cls is not None:
                failures.append(f"F3c3 unpinned {cls} rule {rid}")
            continue
        if cls != pin[0]:
            failures.append(f"F3c3 {rid}: class {cls} != pinned {pin[0]}")
        elif f3c3_rule_sha(fresh_rules[rid]) != pin[2]:
            failures.append(f"F3c3 {rid}: content moved from its pinned sha256")
        else:
            found[pin[0]] += 1
    added = set(added_rule_ids)
    for rid, (cls, owner, _sha) in F3C3["added_rules"].items():
        if converted_principal(owner) and rid not in added and rid not in base_rules:
            failures.append(f"F3c3 pinned {cls} rule missing: {rid}")
    for rid, key in F3C3["required_added_edges"]:
        if rid not in fresh_rules and rid not in base_rules:
            continue
        have = {json.dumps(e, sort_keys=True) for e in (fresh_rules.get(rid, {}).get("granted_by") or [])}
        if key not in have:
            failures.append(f"F3c3 pinned granted_by edge missing on {rid}: {key}")
    active = converted_principal(F3C3["owner"])
    for rel, key in F3C3["var_contributions"]:
        if not active:
            continue
        try:
            table = json.loads(fresh_other.get(rel, b"{}") or b"{}")
        except json.JSONDecodeError:
            table = {}
        have = {json.dumps(c, sort_keys=True) for c in table.get("contributions", [])}
        if key not in have:
            failures.append(f"F3c3 pinned contribution missing in {rel}: {key}")
    for rel, count in F3C3["defect_rows"].items():
        if not active:
            continue
        try:
            rows = json.loads(fresh_other.get(rel, b"null") or b"null")
        except json.JSONDecodeError:
            rows = None
        if not isinstance(rows, list) or len(rows) != count:
            failures.append(f"F3c3 {rel}: {len(rows) if isinstance(rows, list) else 'absent'} rows, pinned {count}")
    return found, failures


# SD-36 Epic F3c4b (converter step 4): an ability-category pick row converts as an OPTION of the
# choice that picks it (`pool_option` rules, `crates/codex-ingest/.../sheet_rule/pool_option.rs`),
# and a CATEGORY-less record is found under the category its own source row declares (one `.MOD`
# object filed per book resolves to every fragment). Pinned by `f3c4b_delta_pins.py` into
# `structural_diff_f3c4b_deltas.json`:
#   added_rules      every added `pool_option` rule (and `#` sibling) by sha256 of the whole rule;
#   field_deltas     every (rule id, field) that moves against tranche/16, by sha256 of the new
#                    value, in one of two classes whose SHAPE is also checked here:
#                    f3c4b_gate_term_resolves (old == new except `MissingRule` terms now `Rule`),
#                    f3c4b_closure_complete (a class principal newly attested complete);
#   replaced_edges   a removed `granted_by` edge a parameterised reference (`Power Attack (Flurry)`)
#                    had fallen back to on the base record, allowed only when the `pool_option` the
#                    reference now names exactly carries the same granter and `when`;
#   required_added_edges / var_contributions / defect_rows  as F3c3 (vs the prior step's package).
_F3C4B_DELTAS_PATH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "structural_diff_f3c4b_deltas.json")
F3C4B_CLASS_CAUSES = {
    "f3c4b_pool_option": "F3c4b: an ability-category pick row no inventory unit stands for becomes a pool_option rule granted by the choice that picks it (Granter::Choice), carrying its BONUS:VAR / ABILITY / PRE",
    "f3c4b_gate_term_resolves": "F3c4b: a gate term naming a pick row (or a CATEGORY-less record under its row's declared category) was MissingRule and now resolves to that Rule; nothing else in the field moves",
    "f3c4b_closure_complete": "F3c4b: closure_complete=true attested on a class principal whose closure is now defect-free (its Internal|CMB / tracker reference resolves to the .MOD-row records of that one object)",
}


def f3c4b_missing_to_rule(old: object, new: object) -> bool:
    """old == new, except positions where old is {"MissingRule": ...} and new is {"Rule": <id>}."""
    if isinstance(old, dict) and isinstance(new, dict):
        if set(old) == {"MissingRule"} and set(new) == {"Rule"} and isinstance(new["Rule"], str):
            return True
        return set(old) == set(new) and all(f3c4b_missing_to_rule(old[k], new[k]) for k in old)
    if isinstance(old, list) and isinstance(new, list):
        return len(old) == len(new) and all(f3c4b_missing_to_rule(a, b) for a, b in zip(old, new))
    return old == new


def f3c4b_classify_field(rid: str, field: str, old: dict, new: dict) -> str | None:
    if field == "closure_complete" and kind_of(rid) == "class" and "#" not in rid and new.get(field) is True and old.get(field) is not True:
        return "f3c4b_closure_complete"
    if field in ("applies", "grants", "prose", "value", "offers") and old.get(field) != new.get(field) and f3c4b_missing_to_rule(old.get(field), new.get(field)):
        return "f3c4b_gate_term_resolves"
    return None


def f3c4b_classify_added(rid: str, rule: dict) -> str | None:
    return "f3c4b_pool_option" if kind_of(rid) == "pool_option" else None


def _load_f3c4b() -> dict:
    empty = {"added_rules": {}, "field_deltas": {}, "replaced_edges": [], "required_added_edges": [], "var_contributions": [], "defect_rows": {}, "owner": ""}
    try:
        with open(_F3C4B_DELTAS_PATH, "r", encoding="utf-8") as fh:
            data = json.load(fh)
    except (OSError, json.JSONDecodeError):
        return empty
    added: dict[str, tuple[str, str, str]] = {}
    for name, c in data.get("added_rules", {}).items():
        assert name in F3C4B_CLASS_CAUSES, f"{_F3C4B_DELTAS_PATH}: unknown class {name}"
        assert len(c["pins"]) == c["_count"], f"{_F3C4B_DELTAS_PATH}: {name} _count mismatch -- regenerate with _command"
        for rid, owner, sha in c["pins"]:
            assert rid not in added, f"{_F3C4B_DELTAS_PATH}: {rid} pinned twice"
            added[rid] = (name, owner, sha)
    fields: dict[tuple[str, str], tuple[str, str]] = {}
    for name, c in data.get("field_deltas", {}).items():
        assert name in F3C4B_CLASS_CAUSES, f"{_F3C4B_DELTAS_PATH}: unknown class {name}"
        assert len(c["pins"]) == c["_count"], f"{_F3C4B_DELTAS_PATH}: {name} _count mismatch"
        for rid, field, sha in c["pins"]:
            fields[(rid, field)] = (name, sha)
    out = dict(empty)
    for key in ("replaced_edges", "required_added_edges", "var_contributions"):
        block = data.get(key, {})
        assert len(block.get("pins", [])) == block.get("_count", 0), f"{_F3C4B_DELTAS_PATH}: {key} _count mismatch"
        out[key] = [tuple(p) for p in block.get("pins", [])]
    out.update({"added_rules": added, "field_deltas": fields, "defect_rows": dict(data.get("defect_rows", {})), "owner": data.get("owner", "")})
    return out


F3C4B = _load_f3c4b()


def f3c4b_field_delta_holds(rid: str, field: str, old: dict, new: dict) -> str | None:
    pin = F3C4B["field_deltas"].get((rid, field))
    if pin is None:
        return None
    if f3c4b_classify_field(rid, field, old, new) != pin[0] or f3b2_field_sha(new.get(field)) != pin[1]:
        return None
    return pin[0]


def f3c4b_replacement_holds(rid: str, key: str, fresh_rules: dict[str, dict]) -> bool:
    """The shape of a replaced edge: an edge on `rid` that a PARAMETERISED reference
    (`Power Attack (Flurry)`) fell back to (the base record, `prereq.rs` `resolve_holdable_rule`),
    now carried -- same granter, same `when` -- by the `pool_option` rule the reference names
    exactly (its label is `<rid's label> (...`)."""
    edge = json.loads(key)
    base_label = (fresh_rules.get(rid) or {}).get("label") or ""
    if not base_label:
        return False
    want = json.dumps({"by": edge.get("by"), "when": edge.get("when")}, sort_keys=True)
    for r in fresh_rules.values():
        if kind_of(r.get("id", "")) != "pool_option" or not str(r.get("label", "")).startswith(base_label + " ("):
            continue
        if want in {json.dumps(e, sort_keys=True) for e in (r.get("granted_by") or [])}:
            return True
    return False


def f3c4b_edge_replaced(rid: str, key: str, fresh_rules: dict[str, dict]) -> bool:
    """A pinned removed edge whose replacement holds (`f3c4b_replacement_holds`)."""
    return (rid, key) in set(F3C4B["replaced_edges"]) and f3c4b_replacement_holds(rid, key, fresh_rules)


def f3c4b_check(fresh_rules: dict[str, dict], base_rules: dict[str, dict], added_rule_ids: list[str], fresh_other: dict[str, bytes]) -> tuple[Counter, list[str]]:
    """(pinned added rule ids found per class, failure lines) for the F3c4b pins (module comment)."""
    found: Counter = Counter()
    failures: list[str] = []
    active = bool((fresh_rules.get(F3C4B["owner"], {}).get("provenance") or {}).get("oracle_pin"))
    for rid in added_rule_ids:
        cls = f3c4b_classify_added(rid, fresh_rules[rid])
        pin = F3C4B["added_rules"].get(rid)
        if pin is None:
            if cls is not None:
                failures.append(f"F3c4b unpinned {cls} rule {rid}")
            continue
        if cls != pin[0]:
            failures.append(f"F3c4b {rid}: class {cls} != pinned {pin[0]}")
        elif f3b2_field_sha(fresh_rules[rid]) != pin[2]:
            failures.append(f"F3c4b {rid}: content moved from its pinned sha256")
        else:
            found[pin[0]] += 1
    added = set(added_rule_ids)
    if active:
        for rid, (cls, _owner, _sha) in F3C4B["added_rules"].items():
            if rid not in added and rid not in base_rules:
                failures.append(f"F3c4b pinned {cls} rule missing: {rid}")
    for rid, key in F3C4B["required_added_edges"]:
        if rid not in fresh_rules and rid not in base_rules:
            continue
        have = {json.dumps(e, sort_keys=True) for e in (fresh_rules.get(rid, {}).get("granted_by") or [])}
        if key not in have:
            failures.append(f"F3c4b pinned granted_by edge missing on {rid}: {key}")
    # A pinned field delta must still be there: withdrawing it (e.g. an attestation reverting to
    # the tranche/16 value) is otherwise invisible to a diff against tranche/16.
    if active:
        for (rid, field), (cls, sha) in F3C4B["field_deltas"].items():
            if rid in fresh_rules and f3b2_field_sha(fresh_rules[rid].get(field)) != sha:
                failures.append(f"F3c4b pinned {cls} field delta withdrawn or moved: {rid}: {field}")
    for rel, key in F3C4B["var_contributions"]:
        if not active:
            continue
        try:
            table = json.loads(fresh_other.get(rel, b"{}") or b"{}")
        except json.JSONDecodeError:
            table = {}
        if key not in {json.dumps(c, sort_keys=True) for c in table.get("contributions", [])}:
            failures.append(f"F3c4b pinned contribution missing in {rel}: {key}")
    for rel, count in F3C4B["defect_rows"].items():
        if not active:
            continue
        # A later step's pin of the same file supersedes this count (F3c5: fewer unresolved
        # references); that step's check gates it.
        if rel in _F3C5_DEFECT_FILES():
            continue
        try:
            rows = json.loads(fresh_other.get(rel, b"null") or b"null")
        except json.JSONDecodeError:
            rows = None
        if not isinstance(rows, list) or len(rows) != count:
            failures.append(f"F3c4b {rel}: {len(rows) if isinstance(rows, list) else 'absent'} rows, pinned {count}")
    return found, failures


# SD-36 Epic F3c5 (converter step 5): a `CATEGORY:Internal` natural-attack helper row no inventory
# unit stands for converts as `Fact::NaturalAttack(<attack>)` on the rule that grants it
# (`sheet_rule/natural_attack.rs`). Pinned by `f3c5_delta_pins.py` into
# `structural_diff_f3c5_deltas.json`:
#   required_added_grants  every `grants` entry the fresh package carries that the F3c4b package did
#                          not -- each a (Gated)FactGrant of a NaturalAttack fact -- by exact JSON;
#                          a dropped one fails (`grants` growth is otherwise unpinned);
#   added_rules            f3c5_line_sibling: a record that now carries the fact is no longer a
#                          single-line principal (SD-36 Epic E CONV-02), so its one line moves,
#                          value / target / gate intact, to a `#weapon<n>` sibling (sha256 pinned);
#   field_deltas           f3c5_line_split (applies / target / value on that record's principal:
#                          a Text principal with no target whose gate terms are a subset of the
#                          sibling's) and f3c5_closure_complete (a class principal newly attested:
#                          Dragon Disciple, whose one closure defect was `Internal|Bite`), by sha256
#                          of the new value;
#   added_var_tables       `_vars/` tables first written because a fact's `when` reads them, by sha256;
#   defect_rows            `_defects/` row counts (supersedes the F3c4b count of the same file).
_F3C5_DELTAS_PATH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "structural_diff_f3c5_deltas.json")
F3C5_CLASS_CAUSES = {
    "f3c5_line_sibling": "F3c5: a record that now carries its Internal natural-attack helper as a NaturalAttack fact is no longer a single-line principal (CONV-02), so its one line moves, value/target/gate intact, to a #weapon<n> sibling",
    "f3c5_line_split": "F3c5: that record's principal becomes a Text principal with no target, gated by a subset of the sibling's gate terms (the line's own level term moves with the line)",
    "f3c5_closure_complete": "F3c5: closure_complete=true attested on a class principal whose one closure defect was an Internal natural-attack helper reference (Dragon Disciple: Internal|Bite)",
}


def _is_natural_attack_grant(effect: object) -> bool:
    if not isinstance(effect, dict):
        return False
    if isinstance(effect.get("FactGrant"), dict):
        return set(effect["FactGrant"]) == {"NaturalAttack"}
    g = effect.get("GatedFactGrant")
    return isinstance(g, dict) and isinstance(g.get("fact"), dict) and set(g["fact"]) == {"NaturalAttack"}


def _gate_terms(applies: object) -> list[str]:
    if isinstance(applies, dict) and set(applies) == {"All"} and isinstance(applies["All"], list):
        return [json.dumps(t, sort_keys=True) for t in applies["All"]]
    if applies in (None, "Always"):
        return []
    return [json.dumps(applies, sort_keys=True)]


def f3c5_sibling_of(rid: str, fresh_rules: dict[str, dict]) -> str | None:
    """The `<rid>#weapon<n>` sibling of a principal that carries a NaturalAttack fact, if any."""
    principal = fresh_rules.get(rid) or {}
    if "#" in rid or not any(_is_natural_attack_grant(e) for e in (principal.get("grants") or [])):
        return None
    sibs = [r for r in fresh_rules if r.startswith(rid + "#weapon")]
    return sibs[0] if len(sibs) == 1 else None


def f3c5_classify_added(rid: str, fresh_rules: dict[str, dict]) -> str | None:
    if "#weapon" in rid and f3c5_sibling_of(rid.split("#")[0], fresh_rules) == rid:
        return "f3c5_line_sibling"
    return None


def f3c5_classify_field(rid: str, field: str, old: dict, new: dict, fresh_rules: dict[str, dict]) -> str | None:
    if field == "closure_complete" and kind_of(rid) == "class" and "#" not in rid and new.get(field) is True and old.get(field) is not True:
        return "f3c5_closure_complete"
    sib_id = f3c5_sibling_of(rid, fresh_rules)
    if sib_id is None or field not in ("applies", "target", "value"):
        return None
    sib = fresh_rules[sib_id]
    if field == "value" and new.get("value") == "Text" and sib.get("value") == old.get("value"):
        return "f3c5_line_split"
    if field == "target" and new.get("target") is None and sib.get("target") == old.get("target"):
        return "f3c5_line_split"
    if field == "applies" and set(_gate_terms(new.get("applies"))) <= set(_gate_terms(sib.get("applies"))) and f3c4b_missing_to_rule(old.get("applies"), sib.get("applies")):
        return "f3c5_line_split"
    return None


def _load_f3c5() -> dict:
    empty = {"added_rules": {}, "field_deltas": {}, "required_added_grants": [], "added_var_tables": [], "defect_rows": {}, "owner": ""}
    try:
        with open(_F3C5_DELTAS_PATH, "r", encoding="utf-8") as fh:
            data = json.load(fh)
    except (OSError, json.JSONDecodeError):
        return empty
    added: dict[str, tuple[str, str, str]] = {}
    for name, c in data.get("added_rules", {}).items():
        assert name in F3C5_CLASS_CAUSES, f"{_F3C5_DELTAS_PATH}: unknown class {name}"
        assert len(c["pins"]) == c["_count"], f"{_F3C5_DELTAS_PATH}: {name} _count mismatch -- regenerate with _command"
        for rid, owner, sha in c["pins"]:
            added[rid] = (name, owner, sha)
    fields: dict[tuple[str, str], tuple[str, str]] = {}
    for name, c in data.get("field_deltas", {}).items():
        assert name in F3C5_CLASS_CAUSES, f"{_F3C5_DELTAS_PATH}: unknown class {name}"
        assert len(c["pins"]) == c["_count"], f"{_F3C5_DELTAS_PATH}: {name} _count mismatch"
        for rid, field, sha in c["pins"]:
            fields[(rid, field)] = (name, sha)
    out = dict(empty)
    for key in ("required_added_grants", "added_var_tables"):
        block = data.get(key, {})
        assert len(block.get("pins", [])) == block.get("_count", 0), f"{_F3C5_DELTAS_PATH}: {key} _count mismatch"
        out[key] = [tuple(p) for p in block.get("pins", [])]
    out.update({"added_rules": added, "field_deltas": fields, "defect_rows": dict(data.get("defect_rows", {})), "owner": data.get("owner", "")})
    return out


F3C5 = _load_f3c5()


def f3c5_field_delta_holds(rid: str, field: str, old: dict, new: dict, fresh_rules: dict[str, dict]) -> str | None:
    pin = F3C5["field_deltas"].get((rid, field))
    if pin is None:
        return None
    if f3c5_classify_field(rid, field, old, new, fresh_rules) != pin[0] or f3b2_field_sha(new.get(field)) != pin[1]:
        return None
    return pin[0]


def f3c5_check(fresh_rules: dict[str, dict], base_rules: dict[str, dict], added_rule_ids: list[str], fresh_other: dict[str, bytes]) -> tuple[Counter, list[str]]:
    """(pinned added rule ids found per class, failure lines) for the F3c5 pins (module comment)."""
    found: Counter = Counter()
    failures: list[str] = []
    active = bool((fresh_rules.get(F3C5["owner"], {}).get("provenance") or {}).get("oracle_pin"))
    for rid in added_rule_ids:
        cls = f3c5_classify_added(rid, fresh_rules)
        pin = F3C5["added_rules"].get(rid)
        if pin is None:
            if cls is not None:
                failures.append(f"F3c5 unpinned {cls} rule {rid}")
            continue
        if cls != pin[0]:
            failures.append(f"F3c5 {rid}: class {cls} != pinned {pin[0]}")
        elif f3b2_field_sha(fresh_rules[rid]) != pin[2]:
            failures.append(f"F3c5 {rid}: content moved from its pinned sha256")
        else:
            found[pin[0]] += 1
    if not active:
        return found, failures
    added = set(added_rule_ids)
    for rid, (cls, _owner, _sha) in F3C5["added_rules"].items():
        if rid not in added and rid not in base_rules:
            failures.append(f"F3c5 pinned {cls} rule missing: {rid}")
    for (rid, field), (cls, sha) in F3C5["field_deltas"].items():
        if rid in fresh_rules and f3b2_field_sha(fresh_rules[rid].get(field)) != sha:
            failures.append(f"F3c5 pinned {cls} field delta withdrawn or moved: {rid}: {field}")
    for rid, key in F3C5["required_added_grants"]:
        have = {json.dumps(e, sort_keys=True) for e in (fresh_rules.get(rid, {}).get("grants") or [])}
        if key not in have:
            failures.append(f"F3c5 pinned NaturalAttack grant missing on {rid}: {key}")
    for rel, sha in F3C5["added_var_tables"]:
        raw = fresh_other.get(rel)
        if raw is None:
            failures.append(f"F3c5 pinned _vars table missing: {rel}")
        elif f3b2_field_sha(json.loads(raw)) != sha:
            failures.append(f"F3c5 pinned _vars table moved: {rel}")
    for rel, count in F3C5["defect_rows"].items():
        try:
            rows = json.loads(fresh_other.get(rel, b"[]") or b"[]")
        except json.JSONDecodeError:
            rows = None
        if not isinstance(rows, list) or len(rows) != count:
            failures.append(f"F3c5 {rel}: {len(rows) if isinstance(rows, list) else 'absent'} rows, pinned {count}")
    return found, failures


def _F3C5_DEFECT_FILES() -> set[str]:
    return set(F3C5["defect_rows"])


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
    f1c_deltas: dict[str, list[tuple[str, str]]] = defaultdict(list)
    f3b2_deltas: dict[str, list[tuple[str, str]]] = defaultdict(list)
    f3b2b_deltas: dict[str, list[tuple[str, str]]] = defaultdict(list)
    f3c4b_deltas: dict[str, list[tuple[str, str]]] = defaultdict(list)
    f3c5_deltas: dict[str, list[tuple[str, str]]] = defaultdict(list)
    f3c4b_replaced: list[tuple[str, str]] = []
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
            # SD-36 Epic F3b2: an exact pinned (rule id, field, value) triple whose class shape
            # holds on these two records -- checked BEFORE the F1c pins, so a pair both name (the
            # 9 product-identity class principals F1c pinned for `prose`) is attributed to the
            # mechanism that moved it now, under its stricter pinned-value check.
            # SD-36 Epic F3b2b: the same rule, one step later, checked first.
            f3c5_class = f3c5_field_delta_holds(rid, field, old, new, fresh_rules)
            if f3c5_class is not None:
                f3c5_deltas[f3c5_class].append((rid, field))
                continue
            f3c4b_class = f3c4b_field_delta_holds(rid, field, old, new)
            if f3c4b_class is not None:
                f3c4b_deltas[f3c4b_class].append((rid, field))
                continue
            f3b2b_class = f3b2b_delta_holds(rid, field, old, new)
            if f3b2b_class is not None:
                f3b2b_deltas[f3b2b_class].append((rid, field))
                continue
            f3b2_class = f3b2_delta_holds(rid, field, old, new)
            if f3b2_class is not None:
                f3b2_deltas[f3b2_class].append((rid, field))
                continue
            # SD-36 Epic F1c: an exact pinned (rule id, field) pair of a named F1c class whose
            # own shape check holds on these two records (checked BEFORE the older pins, so a
            # pair both lists name is attributed to the mechanism that moved it now).
            f1c_class = F1C_FIELD_DELTA_CLASS.get((rid, field))
            if f1c_class is not None and f1c_delta_holds(f1c_class, rid, field, old, new, fresh_rules):
                f1c_deltas[f1c_class].append((rid, field))
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
            if f3c4b_edge_replaced(rid, e, fresh_rules):
                f3c4b_replaced.append((rid, e))
                continue
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
    print(f"  added _vars/ tables: {len(added_vars)}  (condition variables an added GatedFactGrant's `when` now references, and the pool variables a D8 pick's `count` references)")
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
    f1c_added = Counter(F1C_NEW_RULE_ID_CLASS[r] for r in added_rule_ids if r in F1C_NEW_RULE_ID_CLASS)
    for name, cause in F1C_CLASS_CAUSES.items():
        pairs = f1c_deltas.get(name, [])
        print(f"  F1c {name}: {len(pairs)} field deltas on {len(set(r for r, _ in pairs))} records, {f1c_added.get(name, 0)} added rule ids -- {cause} (see structural_diff_f1c_deltas.json)")
    for name, cause in F3B2_CLASS_CAUSES.items():
        pairs = f3b2_deltas.get(name, [])
        pinned = sum(1 for (n, _) in F3B2_PINS.values() if n == name)
        print(f"  F3b2 {name}: {len(pairs)} of {pinned} pinned field deltas on {len(set(r for r, _ in pairs))} records -- {cause} (see structural_diff_f3b2_deltas.json)")
    for name, cause in F3B2B_CLASS_CAUSES.items():
        pairs = f3b2b_deltas.get(name, [])
        pinned = sum(1 for (n, _) in F3B2B_PINS.values() if n == name)
        print(f"  F3b2b {name}: {len(pairs)} of {pinned} pinned field deltas on {len(set(r for r, _ in pairs))} records -- {cause} (see structural_diff_f3b2b_deltas.json)")
    f3c3_found, f3c3_failures = f3c3_check(fresh_rules, base_rules, added_rule_ids, fresh["other_files"])
    for name, cause in F3C3_CLASS_CAUSES.items():
        pinned = sum(1 for (n, _o, _s) in F3C3["added_rules"].values() if n == name)
        print(f"  F3c3 {name}: {f3c3_found.get(name, 0)} of {pinned} pinned added rule ids (content sha256 held) -- {cause} (see structural_diff_f3c3_deltas.json)")
    print(f"  F3c3 pinned added granted_by edges: {len(F3C3['required_added_edges'])}; pinned _vars/ contributions: {len(F3C3['var_contributions'])}; pinned _defects/ row counts: {F3C3['defect_rows']}")
    f3c4b_found, f3c4b_failures = f3c4b_check(fresh_rules, base_rules, added_rule_ids, fresh["other_files"])
    for name, cause in F3C4B_CLASS_CAUSES.items():
        if name == "f3c4b_pool_option":
            pinned = sum(1 for (n, _o, _s) in F3C4B["added_rules"].values() if n == name)
            print(f"  F3c4b {name}: {f3c4b_found.get(name, 0)} of {pinned} pinned added rule ids (content sha256 held) -- {cause} (see structural_diff_f3c4b_deltas.json)")
        else:
            pairs = f3c4b_deltas.get(name, [])
            pinned = sum(1 for (n, _s) in F3C4B["field_deltas"].values() if n == name)
            print(f"  F3c4b {name}: {len(pairs)} of {pinned} pinned field deltas on {len(set(r for r, _ in pairs))} records -- {cause} (see structural_diff_f3c4b_deltas.json)")
    print(f"  F3c4b replaced self-edges: {len(f3c4b_replaced)} of {len(F3C4B['replaced_edges'])} pinned; pinned added granted_by edges: {len(F3C4B['required_added_edges'])}; pinned _vars/ contributions: {len(F3C4B['var_contributions'])}; pinned _defects/ row counts: {F3C4B['defect_rows']}")
    f3c5_found, f3c5_failures = f3c5_check(fresh_rules, base_rules, added_rule_ids, fresh["other_files"])
    for name, cause in F3C5_CLASS_CAUSES.items():
        if name == "f3c5_line_sibling":
            pinned = sum(1 for (n, _o, _s) in F3C5["added_rules"].values() if n == name)
            print(f"  F3c5 {name}: {f3c5_found.get(name, 0)} of {pinned} pinned added rule ids (content sha256 held) -- {cause} (see structural_diff_f3c5_deltas.json)")
        else:
            pairs = f3c5_deltas.get(name, [])
            pinned = sum(1 for (n, _s) in F3C5["field_deltas"].values() if n == name)
            print(f"  F3c5 {name}: {len(pairs)} of {pinned} pinned field deltas on {len(set(r for r, _ in pairs))} records -- {cause} (see structural_diff_f3c5_deltas.json)")
    print(f"  F3c5 pinned NaturalAttack grants: {len(F3C5['required_added_grants'])}; pinned added _vars/ tables: {len(F3C5['added_var_tables'])}; pinned _defects/ row counts: {F3C5['defect_rows']}")
    if added_rule_ids:
        unnamed = [r for r in added_rule_ids if r not in KNOWN_ADDED_RULE_CAUSES and r not in F3C3["added_rules"] and r not in F3C4B["added_rules"] and r not in F3C5["added_rules"]]
        print(f"  added rule ids: {len(added_rule_ids)} ({len(unnamed)} with no named cause)")
        for rid in added_rule_ids[: args.max_examples]:
            cause = KNOWN_ADDED_RULE_CAUSES.get(rid) or (F3C3_CLASS_CAUSES[F3C3["added_rules"][rid][0]] if rid in F3C3["added_rules"] else F3C4B_CLASS_CAUSES[F3C4B["added_rules"][rid][0]] if rid in F3C4B["added_rules"] else F3C5_CLASS_CAUSES[F3C5["added_rules"][rid][0]] if rid in F3C5["added_rules"] else "cause not yet named -- explain before treating this as expected")
            print(f"    {rid}: {cause}")
        if len(added_rule_ids) > args.max_examples:
            print(f"    ... and {len(added_rule_ids) - args.max_examples} more")
    print()
    print(f"unexpected field deltas: {len(unexpected_field_deltas)}")
    for rid, field in unexpected_field_deltas[: args.max_examples]:
        print(f"  {rid}: {field}")
    if len(unexpected_field_deltas) > args.max_examples:
        print(f"  ... and {len(unexpected_field_deltas) - args.max_examples} more")
    # SD-36 Epic F3b2b: every pinned added edge must still be there. A pinned target absent from
    # BOTH trees is not this package (the synthetic fixtures of structural_diff_test.py); a target
    # the baseline has and the fresh tree lost is already a removed rule id, and is counted here too.
    missing_f3b2b_edges: list[tuple[str, str]] = []
    for rid, key in F3B2B_REQUIRED_EDGES:
        if rid not in fresh_rules and rid not in base_rules:
            continue
        have = {json.dumps(e, sort_keys=True) for e in (fresh_rules.get(rid, {}).get("granted_by") or [])}
        if key not in have:
            missing_f3b2b_edges.append((rid, key))
    print(f"F3b2b pinned added granted_by edges: {len(F3B2B_REQUIRED_EDGES) - len(missing_f3b2b_edges)} of {len(F3B2B_REQUIRED_EDGES)} present (see structural_diff_f3b2b_deltas.json)")
    for rid, key in missing_f3b2b_edges[: args.max_examples]:
        print(f"  MISSING {rid}: {key}")
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
    if missing_f3b2b_edges:
        failures.append(f"missing F3b2b pinned edges: {len(missing_f3b2b_edges)}")
    if removed_grants:
        failures.append(f"removed grants: {len(removed_grants)}")
    if f3c3_failures:
        for line in f3c3_failures[: args.max_examples]:
            print(f"  {line}")
        failures.append(f"F3c3 pin failures: {len(f3c3_failures)}")
    if f3c4b_failures:
        for line in f3c4b_failures[: args.max_examples]:
            print(f"  {line}")
        failures.append(f"F3c4b pin failures: {len(f3c4b_failures)}")
    if f3c5_failures:
        for line in f3c5_failures[: args.max_examples]:
            print(f"  {line}")
        failures.append(f"F3c5 pin failures: {len(f3c5_failures)}")
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
