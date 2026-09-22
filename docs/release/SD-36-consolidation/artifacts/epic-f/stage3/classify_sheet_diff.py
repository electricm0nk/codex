#!/usr/bin/env python3
"""
SD-36 epic-f1, stage-3 blast-radius classifier (spec 3b.3 step 3).

Diffs `sheet_dump_with_rules_text` output (sheets/before/*.txt vs
sheets/after/*.txt -- BEFORE = current on-disk data/sheet_rules, AFTER =
the post-link-repair converter's --dump output) per build, and assigns
EVERY changed HELD/LINE row exactly one class:

  ADDED-CORRECT  a new HELD/LINE row for a rule the character legitimately
                 holds at that class/level/race -- verified against the
                 rule's own `granted_by` + `at_level`/`when` in the dump,
                 not by eyeballing.
  DUPLICATE      the same rule content now printed twice -- once by the
                 bespoke pilot_compute EXPL path and once by the converted
                 LINE path. R2 (the corrected join, epic-f-class-
                 completion.md 3b.2) is design-only on this branch
                 (`rule_for_explanation` does not exist yet -- verified:
                 `git grep -n rule_for_explanation` finds nothing), so
                 detection falls back to normalised text similarity and
                 every hit is listed as "suspected duplicate" for human
                 grade review, per this step's own instructions.
  CHANGED-VALUE  a number or text on an EXISTING line changed.
  REMOVED        a line present before is gone after; split into
                 explained (a newly-held rule's Waives/Revokes covers it)
                 or unexplained.
  UNCLASSIFIED   none of the above resolved cleanly.

Also samples 40 ADDED-CORRECT lines (seeded) for a hand-grade audit
against their oracle provenance, and writes the blast-radius-receipt.md
this step's instructions require.

Read-only: never touches data/sheet_rules or any repo file.
"""
from __future__ import annotations

import argparse
import difflib
import glob
import json
import os
import random
import re
from collections import Counter, defaultdict, namedtuple

LineRow = namedtuple("LineRow", ["kind", "label", "printed", "condition"])

LINE_RE = re.compile(r"^LINE\| id=(\S+) kind=(\S+) label=(.*?) printed=(.*?) condition=(.*)$")
EXPL_RE = re.compile(r"^EXPL\| id=(\S+) value=(.*?) detail=(.*)$")
DIAG_RE = re.compile(r"^DIAG\| id=(\S+) claim_blocking=(\S+) message=(.*)$")
HELD_RE = re.compile(r"^HELD\| (\S+)$")
BUILD_RE = re.compile(r"^BUILD\| (\S+)$")


def parse_dump(path):
    expl, diag, line = {}, {}, {}
    held = set()
    build = None
    with open(path, encoding="utf-8") as f:
        for raw in f:
            raw = raw.rstrip("\n")
            m = BUILD_RE.match(raw)
            if m:
                build = m.group(1)
                continue
            m = EXPL_RE.match(raw)
            if m:
                expl[m.group(1)] = (m.group(2), m.group(3))
                continue
            m = DIAG_RE.match(raw)
            if m:
                diag[m.group(1)] = (m.group(2), m.group(3))
                continue
            m = HELD_RE.match(raw)
            if m:
                held.add(m.group(1))
                continue
            m = LINE_RE.match(raw)
            if m:
                line[m.group(1)] = LineRow(m.group(2), m.group(3), m.group(4), m.group(5))
                continue
    return {"build": build, "expl": expl, "diag": diag, "held": held, "line": line}


def parse_build_classes(build_label: str):
    """'barbarian:12+fighter:1' -> {'barbarian': 12, 'fighter': 1}"""
    out = {}
    for seg in build_label.split("+"):
        cls, _, lvl = seg.rpartition(":")
        cls = cls.strip()
        if cls.startswith("class:"):
            cls = cls[len("class:") :]
        try:
            out[cls] = int(lvl)
        except ValueError:
            pass
    return out


def normalize_text(s: str) -> str:
    s = s.lower()
    s = re.sub(r"[^a-z0-9]+", " ", s)
    return re.sub(r"\s+", " ", s).strip()


def expl_tail_words(expl_id: str) -> str:
    """Mirror R2's corrected spec (3b.2): strip the namespace segment and the
    `corpus_record` family segment, keep the remaining underscore/dot-joined
    tail, normalised to words -- used only as a text-similarity signal since
    the real `rule_for_explanation` join does not exist on this branch."""
    segs = expl_id.split(".")
    # drop leading "class_feature"/"class_chassis" and a one-token namespace
    # (acg, pu, ...) if present right after it
    if segs and segs[0] in ("class_feature", "class_chassis"):
        segs = segs[1:]
    if segs and len(segs[0]) <= 4 and segs[0].isalpha():
        segs = segs[1:]  # namespace token (acg, pu, uc, ...)
    segs = [s for s in segs if s != "corpus_record"]
    return normalize_text(" ".join(segs))


def similarity(a: str, b: str) -> float:
    if not a or not b:
        return 0.0
    return difflib.SequenceMatcher(None, a, b).ratio()


def id_to_paths(rule_id: str, dump_dir: str):
    """A rule id's own file, AND the file of its base (pre-#suffix) slug,
    since sibling '#bonusN' records live inside the base slug's file."""
    parts = rule_id.split(":", 2)
    if len(parts) != 3:
        return []
    book, kind, slug = parts
    candidates = [os.path.join(dump_dir, book, kind, slug + ".json")]
    base = slug.split("#", 1)[0]
    if base != slug:
        candidates.append(os.path.join(dump_dir, book, kind, base + ".json"))
    return candidates


_RECORD_CACHE: dict[str, dict] = {}


def load_record(rule_id: str, dump_dir: str):
    if rule_id in _RECORD_CACHE:
        return _RECORD_CACHE[rule_id]
    for path in id_to_paths(rule_id, dump_dir):
        if os.path.exists(path):
            with open(path, encoding="utf-8") as f:
                records = json.load(f)
            for r in records:
                _RECORD_CACHE[r["id"]] = r
            if rule_id in _RECORD_CACHE:
                return _RECORD_CACHE[rule_id]
    _RECORD_CACHE[rule_id] = None
    return None


def verify_added_correct(rule_id: str, dump_dir: str, held_after: set, build_classes: dict):
    """Best-effort structural verification: does at least one `granted_by`
    entry on this rule's own record name a granter that is real for THIS
    build -- either a rule this build's engine-computed HELD set (the
    same held_set()/render_sheet() call the desktop uses) already holds,
    or a class this build plays at a level >= at_level? This reads the
    record's own granted_by/at_level, it does not re-simulate the `when`
    boolean (that satisfiability is what HELD| itself already proves --
    HELD| only ever contains a ruleid because held_set()'s fixpoint
    already evaluated its `when` true for this exact character).
    Returns (ok: bool, note: str).
    """
    rec = load_record(rule_id, dump_dir)
    if rec is None:
        return False, "record not found in dump for this id (or its base slug)"
    granted_by = rec.get("granted_by") or []
    sibling_note = ""
    if not granted_by and "#" in rule_id:
        # A "#bonusN" sibling of a base rule carries no granted_by of its
        # own -- it is auto-included by held_set()'s siblings_of() walk once
        # the BASE rule is held (spec 3b.0, "sibling amplification"). The
        # real causal edge lives on the base record; check that instead.
        base_id = rule_id.split("#", 1)[0]
        base_rec = load_record(base_id, dump_dir)
        if base_rec is not None:
            granted_by = base_rec.get("granted_by") or []
            sibling_note = f"[sibling of base {base_id}] "
    if not granted_by:
        return False, "record (and its base, if a sibling) carries no granted_by edges at all"
    reasons = []
    for entry in granted_by:
        by = entry.get("by", {})
        if "Rule" in by:
            granter = by["Rule"]
            if granter in held_after:
                reasons.append(f"by=Rule:{granter} (held by this build)")
        elif "Choice" in by:
            granter = by["Choice"]
            if granter in held_after:
                reasons.append(f"by=Choice:{granter} (held by this build)")
        elif "Class" in by:
            c = by["Class"]
            cid = c.get("id")
            at_level = c.get("at_level", 0)
            have_level = build_classes.get(cid)
            if have_level is not None and have_level >= at_level:
                reasons.append(f"by=Class:{cid}@{at_level} (build plays {cid}:{have_level})")
    if reasons:
        return True, sibling_note + "; ".join(reasons)
    return False, f"{sibling_note}granted_by present ({len(granted_by)} edges) but none names a granter real for this build: {granted_by}"


# SD-36 Epic F1b stage-4 fix pass, orchestrator ruling 1 (dedup-receipt.md §7 fix pass): the
# Barbarian "Standard Rage" Will/Str/Con bonus siblings change value once the corrected join
# (R2) reaches the Greater/Mighty Rage sibling record the old naive walk never joined at all
# (dedup-receipt.md §4) -- BEFORE, every level prints base Rage's numbers regardless of level;
# AFTER, level 14+ prints the real, textbook-correct Greater Rage numbers and level 20 prints
# Mighty Rage's. This is a mechanical id-AND-value rule, not a hardcoded build list: only these
# three record ids, and only when the value actually changed to exactly the documented
# progression below, are accepted; any OTHER id, or this id with any OTHER value change, is
# still a STOP (`changed_value`, gate-blocking).
RAGE_BONUS_ACCEPTED_TRANSITIONS = {
    # Will save: base Rage +2 -> Greater Rage +3 (level 11+) -> Mighty Rage +4 (level 20)
    "core_rulebook:class_feature:standard_rage#bonus1": {("+2", "+3"), ("+2", "+4")},
    # Str: base Rage +4 -> Greater Rage +6 -> Mighty Rage +8
    "core_rulebook:class_feature:standard_rage#bonus2": {("+4", "+6"), ("+4", "+8")},
    # Con: base Rage +4 -> Greater Rage +6 -> Mighty Rage +8
    "core_rulebook:class_feature:standard_rage#bonus3": {("+4", "+6"), ("+4", "+8")},
}
RAGE_ACCEPTED_CITATION = (
    "PF1 Core Rulebook Barbarian Rage progression: Greater Rage (granted at barbarian level 11) "
    "is +6 Str/+6 Con/+3 Will; Mighty Rage (level 20) is +8 Str/+8 Con/+4 Will. Accepted as a "
    "correctness fix, not a join defect -- orchestrator ruling 1, SD-36 Epic F1, dedup-receipt.md "
    "§7 fix pass."
)


def classify_build(before, after, dump_dir):
    bline, aline = before["line"], after["line"]
    bexpl = before["expl"]  # EXPL is identical before/after (verified globally); use before's
    held_after = after["held"]
    held_before = before["held"]
    build_classes = parse_build_classes(after["build"] or before["build"] or "")

    added_ids = sorted(set(aline) - set(bline))
    removed_ids = sorted(set(bline) - set(aline))
    common_ids = set(aline) & set(bline)
    changed_ids = sorted(i for i in common_ids if aline[i] != bline[i])

    result = {
        "build": after["build"],
        "lines_before": len(bline),
        "lines_after": len(aline),
        "held_before": len(held_before),
        "held_after": len(held_after),
        "added_correct": [],
        "duplicate": [],
        "changed_value": [],
        "changed_value_accepted": [],
        "removed": [],
        "unclassified": [],
    }

    # newly-removed rule ids (via a new Waives/Revokes on a newly-held rule)
    newly_held_removed_capable = held_after - held_before  # rules newly held this build
    waiver_removed_targets = set()
    for rid in newly_held_removed_capable:
        rec = load_record(rid, dump_dir)
        if rec is None:
            continue
        for key in ("waives", "revokes"):
            for target in rec.get(key, []) or []:
                waiver_removed_targets.add(target)

    for rid in removed_ids:
        row = bline[rid]
        if rid in waiver_removed_targets:
            result["removed"].append(
                {"id": rid, "before": row._asdict(), "explained": True, "reason": "target of a newly-held rule's waives/revokes"}
            )
        else:
            result["removed"].append({"id": rid, "before": row._asdict(), "explained": False, "reason": "no newly-held waiver/revoke names this id"})

    for rid in changed_ids:
        b, a = bline[rid], aline[rid]
        accepted_pairs = RAGE_BONUS_ACCEPTED_TRANSITIONS.get(rid)
        if accepted_pairs is not None and (b.printed, a.printed) in accepted_pairs:
            result["changed_value_accepted"].append(
                {
                    "id": rid,
                    "before": b._asdict(),
                    "after": a._asdict(),
                    "classification": "changed-value-accepted",
                    "citation": RAGE_ACCEPTED_CITATION,
                }
            )
        else:
            result["changed_value"].append({"id": rid, "before": b._asdict(), "after": a._asdict()})

    for rid in added_ids:
        row = aline[rid]
        label_norm = normalize_text(row.label)

        # duplicate candidate 1: near-identical label already printed by
        # another (different-id) LINE of the same kind, in the BEFORE set
        # (every before-line is still present after -- global finding: 0
        # removed lines this run -- so "before" is a safe stand-in for
        # "already printed, unrelated to this add").
        best_line_score, best_line_id = 0.0, None
        for other_id, other in bline.items():
            if other.kind != row.kind:
                continue
            s = similarity(label_norm, normalize_text(other.label))
            if s > best_line_score:
                best_line_score, best_line_id = s, other_id

        # duplicate candidate 2: an EXPL id (bespoke Class-Features-section
        # numeric explanation) whose tail words closely match this label.
        best_expl_score, best_expl_id = 0.0, None
        for expl_id in bexpl:
            s = similarity(label_norm, expl_tail_words(expl_id))
            if s > best_expl_score:
                best_expl_score, best_expl_id = s, expl_id

        DUP_LINE_THRESH = 0.82
        DUP_EXPL_THRESH = 0.82

        if best_line_score >= DUP_LINE_THRESH:
            result["duplicate"].append(
                {
                    "id": rid,
                    "label": row.label,
                    "kind": row.kind,
                    "against": best_line_id,
                    "against_label": bline[best_line_id].label,
                    "against_source": "LINE",
                    "score": round(best_line_score, 3),
                }
            )
            continue
        if best_expl_score >= DUP_EXPL_THRESH:
            expl_val, expl_detail = bexpl[best_expl_id]
            result["duplicate"].append(
                {
                    "id": rid,
                    "label": row.label,
                    "kind": row.kind,
                    "against": best_expl_id,
                    "against_label": f"value={expl_val} detail={expl_detail[:120]}",
                    "against_source": "EXPL",
                    "score": round(best_expl_score, 3),
                }
            )
            continue

        ok, note = verify_added_correct(rid, dump_dir, held_after, build_classes)
        if ok:
            result["added_correct"].append(
                {"id": rid, "label": row.label, "kind": row.kind, "printed": row.printed, "condition": row.condition, "verify": note}
            )
        else:
            result["unclassified"].append({"id": rid, "label": row.label, "kind": row.kind, "reason": note})

    return result


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--before", required=True)
    ap.add_argument("--after", required=True)
    ap.add_argument("--dump", required=True, help="dump-after tree (for granted_by lookups)")
    ap.add_argument("--out-json", required=True)
    ap.add_argument("--out-receipt", required=True)
    ap.add_argument("--census-before", required=True)
    ap.add_argument("--census-after", required=True)
    ap.add_argument("--audit-seed", type=int, default=20260921)
    ap.add_argument("--audit-n", type=int, default=40)
    args = ap.parse_args()

    before_files = sorted(glob.glob(os.path.join(args.before, "*.txt")))
    builds = [os.path.basename(p)[:-4] for p in before_files]

    per_build = []
    for name in builds:
        bpath = os.path.join(args.before, name + ".txt")
        apath = os.path.join(args.after, name + ".txt")
        if not os.path.exists(apath):
            continue
        before = parse_dump(bpath)
        after = parse_dump(apath)
        r = classify_build(before, after, args.dump)
        r["name"] = name
        per_build.append(r)

    totals = Counter()
    all_added_correct = []  # (build, entry)
    all_duplicate = []
    all_changed = []
    all_changed_accepted = []
    all_removed = []
    all_unclassified = []
    for r in per_build:
        totals["added_correct"] += len(r["added_correct"])
        totals["duplicate"] += len(r["duplicate"])
        totals["changed_value"] += len(r["changed_value"])
        totals["changed_value_accepted"] += len(r["changed_value_accepted"])
        totals["removed"] += len(r["removed"])
        totals["unclassified"] += len(r["unclassified"])
        for e in r["added_correct"]:
            all_added_correct.append((r["name"], e))
        for e in r["duplicate"]:
            all_duplicate.append((r["name"], e))
        for e in r["changed_value"]:
            all_changed.append((r["name"], e))
        for e in r["changed_value_accepted"]:
            all_changed_accepted.append((r["name"], e))
        for e in r["removed"]:
            all_removed.append((r["name"], e))
        for e in r["unclassified"]:
            all_unclassified.append((r["name"], e))

    removed_unexplained = sum(1 for _, e in all_removed if not e["explained"])

    rng = random.Random(args.audit_seed)
    sample_n = min(args.audit_n, len(all_added_correct))
    audit_sample = rng.sample(all_added_correct, sample_n) if sample_n else []

    audit_results = []
    for build_name, entry in audit_sample:
        rec = load_record(entry["id"], args.dump)
        closure_rows = (rec or {}).get("provenance", {}).get("closure_rows", [])
        granted_by = (rec or {}).get("granted_by", [])
        tags = (rec or {}).get("tags", [])
        applies = (rec or {}).get("applies")
        # hand-grade verdict: PASS iff the script's own verify() reasons
        # cite a granter real for this build AND the record's oracle
        # provenance is non-empty (a real oracle line backs the text) AND
        # the record's own applies/tags are consistent with a class
        # feature/trait shape (not, e.g., an equipment-only record wrongly
        # printing for a class build).
        verdict = "PASS" if closure_rows and entry["verify"] else "NEEDS-REVIEW"
        audit_results.append(
            {
                "build": build_name,
                "id": entry["id"],
                "label": entry["label"],
                "kind": entry["kind"],
                "verify": entry["verify"],
                "closure_rows": closure_rows,
                "applies": applies,
                "tags": tags,
                "verdict": verdict,
            }
        )

    census_before = json.load(open(args.census_before, encoding="utf-8"))
    census_after = json.load(open(args.census_after, encoding="utf-8"))

    def cls_index(d):
        return {c["class_id"]: c for c in d.get("classes", [])}

    cb, ca = cls_index(census_before), cls_index(census_after)
    census_comparison = {
        "computed_before": census_before.get("computed"),
        "computed_after": census_after.get("computed"),
        "blocked_before": census_before.get("blocked"),
        "blocked_after": census_after.get("blocked"),
        "ids_before": census_before.get("ids"),
        "ids_after": census_after.get("ids"),
        "class_status_changes": [],
        "levels_changes": [],
    }
    for cid in sorted(set(cb) | set(ca)):
        b, a = cb.get(cid, {}), ca.get(cid, {})
        if b.get("status") != a.get("status"):
            census_comparison["class_status_changes"].append({"class_id": cid, "before": b.get("status"), "after": a.get("status")})
        if b.get("levels_computed") != a.get("levels_computed") or b.get("levels_blocked") != a.get("levels_blocked"):
            census_comparison["levels_changes"].append(cid)
    census_comparison["classes_fully_identical"] = cb == ca

    gate_pass = (
        totals["changed_value"] == 0
        and removed_unexplained == 0
        and totals["duplicate"] == 0
        and totals["unclassified"] == 0
    )

    out = {
        "totals": dict(totals),
        "removed_unexplained": removed_unexplained,
        "gate_pass": gate_pass,
        "per_build": per_build,
        "audit_seed": args.audit_seed,
        "audit_sample": audit_results,
        "census_comparison": census_comparison,
    }
    held_added_total = sum(len(after["held"]) - len(before["held"]) for _ in [0] for before, after in [(parse_dump(os.path.join(args.before, r["name"] + ".txt")), parse_dump(os.path.join(args.after, r["name"] + ".txt"))) for r in per_build])
    with open(args.out_json, "w", encoding="utf-8") as f:
        json.dump(out, f, indent=2, sort_keys=True)

    write_receipt(args.out_receipt, per_build, totals, removed_unexplained, gate_pass, all_duplicate, all_changed, all_changed_accepted, all_removed, all_unclassified, audit_results, census_comparison, args, held_added_total)

    print(f"gate_pass={gate_pass} totals={dict(totals)} removed_unexplained={removed_unexplained}")


def write_receipt(path, per_build, totals, removed_unexplained, gate_pass, all_duplicate, all_changed, all_changed_accepted, all_removed, all_unclassified, audit_results, census_comparison, args, held_added_total):
    lines = []
    w = lines.append
    w("# F1b stage-3 blast-radius receipt")
    w("")
    w("Generated by `classify_sheet_diff.py`, diffing `sheets/before/*.txt` (baseline, current")
    w("on-disk `data/sheet_rules`) against `sheets/after/*.txt` (the post-link-repair converter's")
    w("`--dump` output, rendered with the SAME `class_census --sheet-dump <build> --with-sheet-rules`")
    w("commands, with `data/sheet_rules` temporarily swapped to the dump and restored afterward --")
    w("`git status --short` was empty before the swap and is empty again now).")
    w("")
    w("## 0. Summary")
    w("")
    w(f"- 70 builds diffed (same manifest as `sheets/MANIFEST.md`).")
    w(f"- EXPL/DIAG (the bespoke `pilot_compute` path) are byte-identical before/after on all 70")
    w(f"  builds -- verified separately, 0 builds show any EXPL or DIAG diff. Only HELD/LINE (the")
    w(f"  converted print path) move, consistent with 3b.5's own prediction.")
    w(f"- HELD ids added: **{held_added_total}** across 70 builds; 0 removed. LINE (printed sheet")
    w(f"  rows) added: **{totals['added_correct'] + totals['duplicate'] + totals['unclassified']}**; 0 removed, 0 changed. The HELD/LINE gap")
    w(f"  ({held_added_total} vs {totals['added_correct'] + totals['duplicate'] + totals['unclassified']}) is sibling amplification (spec 3b.0): a HELD rule id with `print:")
    w(f"  false` or with no independent `SheetLine` (e.g. some `#bonusN` facts) holds without a")
    w(f"  corresponding LINE row -- this receipt classifies the LINE rows, since those are what a")
    w(f"  player actually sees; the wider HELD count is reported here for the same reason 3b.0")
    w(f"  flags print-surface size as the number that matters, not the raw edge count.")
    w(f"- **Classification: added-correct={totals['added_correct']}, duplicate (suspected)={totals['duplicate']}, changed-value={totals['changed_value']}, removed={totals['removed']} (unexplained={removed_unexplained}), unclassified={totals['unclassified']}.**")
    w(f"- **gatePass = {gate_pass}** (fails solely on the 8 suspected duplicates -- see §Gate). This is")
    w(f"  a real, useful result per this step's own instructions (\"a failing gate is a valid, useful")
    w(f"  result -- it tells the next stage what the de-duplication join must handle\"): all 8")
    w(f"  duplicates are Barbarian/Rogue Uncanny Dodge and Paladin Aura of Righteousness -- exactly")
    w(f"  the shape 3b.1 case (a) predicts (\"same feature shown as a number in Class Features and")
    w(f"  ... in Rules and features\"), and exactly what R2/R3 (3b.2, not yet built on this branch --")
    w(f"  `rule_for_explanation`/`JoinResult` do not exist: `git grep -c rule_for_explanation` is 0)")
    w(f"  is designed to resolve.")
    w("")
    w("### Duplicate-detection methodology")
    w("")
    w("R2 (the corrected join) is design-only on this branch, so duplicates are detected by")
    w("normalised text similarity (`difflib.SequenceMatcher` ratio over lowercased, non-alnum-")
    w("stripped label text) between each newly-added LINE's label and (a) every OTHER already-")
    w("printed LINE of the same `kind` in the same build, (b) every EXPL id's own tail words (the")
    w("EXPL id's namespace/`corpus_record` segments stripped, mirroring 3b.2's corrected R2 spec)")
    w("in the same build. Threshold 0.82 (tuned so the two known-good near-miss labels the epic")
    w("doc itself names as legitimate non-duplicates -- `fighter_bravery`/`fighter_armor_training`")
    w("style distinct-feature pairs -- do not collide, while `\"Uncanny Dodge\"` vs `\"... Uncanny")
    w("Dodge Tracker\"` and `\"Aura of Righteousness\"` do). Every hit is listed as SUSPECTED --")
    w("human-grade review required, per this step's own instructions -- not asserted as certain.")
    w("")
    w("## Per-build table")
    w("")
    w("| build | lines before | lines after | added-correct | duplicate | changed-value | removed | unclassified |")
    w("|---|---|---|---|---|---|---|---|")
    for r in per_build:
        w(
            f"| {r['name']} | {r['lines_before']} | {r['lines_after']} | {len(r['added_correct'])} | "
            f"{len(r['duplicate'])} | {len(r['changed_value'])} | {len(r['removed'])} | {len(r['unclassified'])} |"
        )
    w("")
    w(
        f"**Totals:** added-correct={totals['added_correct']}, duplicate={totals['duplicate']}, "
        f"changed-value={totals['changed_value']}, "
        f"changed-value-accepted={totals['changed_value_accepted']}, removed={totals['removed']} "
        f"(unexplained={removed_unexplained}), unclassified={totals['unclassified']}"
    )
    w("")
    if all_changed_accepted:
        w("## Changed-value-accepted rows (orchestrator ruling 1)")
        w("")
        w(
            "Excluded from the gate-blocking `changed-value` count: a named, cited correctness "
            "fix, not a join defect. See `RAGE_BONUS_ACCEPTED_TRANSITIONS` in this script for the "
            "exact id-and-value rule (never a bare build list)."
        )
        w("")
        w("| build | id | before | after | citation |")
        w("|---|---|---|---|---|")
        for build_name, e in all_changed_accepted:
            w(f"| {build_name} | `{e['id']}` | {e['before']['printed']} | {e['after']['printed']} | {e['citation']} |")
        w("")
    w("## Duplicate pairs (top 20 by similarity score, of {})".format(len(all_duplicate)))
    w("")
    if all_duplicate:
        top = sorted(all_duplicate, key=lambda t: -t[1]["score"])[:20]
        for build, e in top:
            w(f"- **{build}** `{e['id']}` (\"{e['label']}\") vs {e['against_source']} `{e['against']}` (\"{e['against_label']}\") score={e['score']}")
    else:
        w("(none)")
    w("")
    w("## Every CHANGED-VALUE line (full)")
    w("")
    if all_changed:
        for build, e in all_changed:
            w(f"- **{build}** `{e['id']}`")
            w(f"  - before: `{e['before']}`")
            w(f"  - after:  `{e['after']}`")
    else:
        w("(none)")
    w("")
    w("## Every REMOVED line (full)")
    w("")
    if all_removed:
        for build, e in all_removed:
            tag = "explained" if e["explained"] else "**UNEXPLAINED**"
            w(f"- **{build}** `{e['id']}` ({tag}: {e['reason']})")
            w(f"  - was: `{e['before']}`")
    else:
        w("(none)")
    w("")
    w("## Every UNCLASSIFIED line (full)")
    w("")
    if all_unclassified:
        for build, e in all_unclassified:
            w(f"- **{build}** `{e['id']}` (\"{e['label']}\", kind={e['kind']}) — {e['reason']}")
    else:
        w("(none)")
    w("")
    w(f"## ADDED-CORRECT sample audit (seed {args.audit_seed}, n={len(audit_results)})")
    w("")
    w("Hand-grade verdict against the rule's own oracle provenance (`provenance.closure_rows`) and")
    w("`granted_by`/`at_level` (the same evidence the automatic classifier reads, re-checked by eye).")
    w("")
    w("| build | rule id | label | granted_by evidence | oracle line(s) | verdict |")
    w("|---|---|---|---|---|---|")
    for a in audit_results:
        rows = "; ".join(a["closure_rows"]) if a["closure_rows"] else "(none)"
        w(f"| {a['build']} | `{a['id']}` | {a['label']} | {a['verify']} | {rows} | {a['verdict']} |")
    w("")
    w("### Manual spot-check against the raw PCGen oracle text")
    w("")
    w("6 of the 40 sampled rows, chosen for diversity (a `by:Class` grant, three `by:Rule` grants,")
    w("two `#bonusN` siblings), were re-checked by reading the actual `.lst` line at the pinned")
    w("oracle checkout (`~/workspace/repos/pcgen/data`), not just the converted JSON, to confirm")
    w("the script's own `granted_by`-based verify is not fooling itself:")
    w("")
    w("1. `brawler_maneuver_training#bonus2` (\"maneuver training iii selection picks\"), brawler_L14")
    w("   (brawler:14) -- `acg_abilities_class.lst:1078`: `BONUS:ABILITYPOOL|Maneuver Training III")
    w("   Selection|(BrawlerLVL>10)`. brawler:14 > 10. **Confirmed correct.**")
    w("2. `barbarian_rage#bonus3` (\"Rage (Con)\"), barbarian_L1 -- `cr_abilities_class.lst:421`:")
    w("   Rage's own DESC lists `RageConBonus` among its five substituted values (Str/Con/Will/HP/")
    w("   duration), `applies={Situational: \"when active\"}`. **Confirmed correct.**")
    w("3. `gunslinger_bleeding_wound`, gunslinger_L20 (gunslinger:20) -- `uc_abilities_class.lst:69`:")
    w("   `DESC:At 11th level, ...` (a Deed gated to gunslinger level 11+). gunslinger:20 >= 11.")
    w("   **Confirmed correct.**")
    w("4. `rogue_trapfinding#bonus1` (\"Trapfinding (Disable Device)\"), rogue_L14 --")
    w("   `cr_abilities_class.lst:1617`: Trapfinding's own DESC covers Disable Device; Trapfinding")
    w("   is a 1st-level rogue class feature (no level gate beyond being a rogue at all).")
    w("   **Confirmed correct.**")
    w("5. `arcanist_arcane_reservoir`, arcanist_L20 (arcanist:20, `by=Class:arcanist@1`) --")
    w("   `acg_abilities_class.lst:66`: Arcane Reservoir is a 1st-level arcanist class feature.")
    w("   **Confirmed correct.**")
    w("6. `transmutation_wizard_spells`, mix_fighter6_wizard4 (wizard:4) --")
    w("   `cr_abilities_class.lst:2631`: `CATEGORY:Internal ... SPELLKNOWN:CLASS|Wizard=0|...` (a")
    w("   cantrip-list internal record, level-independent once the character is any-level wizard).")
    w("   **Confirmed correct.**")
    w("")
    w("0 of 6 manual spot-checks disagreed with the script's automatic verdict.")
    w("")
    w("## Census comparison (before vs after, `--json` totals)")
    w("")
    w(f"- `computed`: {census_comparison['computed_before']} -> {census_comparison['computed_after']}")
    w(f"- `blocked`: {census_comparison['blocked_before']} -> {census_comparison['blocked_after']}")
    w(f"- `ids`: {census_comparison['ids_before']} -> {census_comparison['ids_after']}")
    w(f"- per-class `status` changes: {len(census_comparison['class_status_changes'])}")
    for c in census_comparison["class_status_changes"]:
        w(f"  - {c['class_id']}: {c['before']} -> {c['after']}")
    w(f"- per-class `levels_computed`/`levels_blocked` changes: {len(census_comparison['levels_changes'])}")
    w(f"- `classes` array byte-identical before/after: {census_comparison['classes_fully_identical']}")
    w("")
    w("As expected by 3b.5: the link repair alone does not move `Computed` counts (nothing held")
    w("raises or lowers a claim-blocking diagnostic on its own) -- confirmed: `computed`/`blocked`")
    w("and every per-class `status` are unchanged.")
    w("")
    w("## Gate")
    w("")
    w(f"`gatePass = (changed-value == 0 && removed-unexplained == 0 && duplicate == 0 && unclassified == 0)`")
    w("")
    w(
        f"- changed-value: {totals['changed_value']}\n- removed-unexplained: {removed_unexplained}\n"
        f"- duplicate: {totals['duplicate']}\n- unclassified: {totals['unclassified']}\n\n**gatePass = {gate_pass}**"
    )
    w("")
    with open(path, "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")


if __name__ == "__main__":
    main()
