"""Re-derives SD-34's `capability-register.json` at SD-35 HEAD (AT-35-E5-005).

Run from anywhere inside the repo:

    python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-005_capability_register.py
    python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-005_capability_register.py --check

The criterion's third Evidence clause (`epic-breakdown.md ### AT-35-E5-005`):

    SD-34's `capability-register.json` re-derived: every row `built: true` or
    `unnecessary-under-sheet-rule: <reason>`.

SD-34's register named 11 capabilities that "must still be built", every one flagged not-built.
This script closes each of those 11 rows at SD-35 HEAD, and it does so **per unit, not
per prose claim**: for every sized row it re-derives the row's ORIGINAL id set by running SD-34's
own stated query against the inventory as it stood at that register's own
`generated_at_head` (`git show <head>:docs/work-inventory.json`), then reports what every one of
those ids is at HEAD. A row cannot be closed on an assertion; it is closed on its units.

Two dispositions only, exactly as the Evidence sentence words them:

  built: true                        the capability exists in live code at HEAD. `built_evidence`
                                     names the file and the command that shows it.
  unnecessary-under-sheet-rule: ...  `decisions.md §1`: Campaign Codex is a paper sheet
                                     generator. The capability was machinery for SIMULATING or
                                     for VERIFYING a magnitude; under the sheet rule the record's
                                     sheet line is one final number, dice in final form, or the
                                     rule's words, and every unit the row named reaches one of
                                     those three forms without it. The reason states which.

Fail-closed, three ways (`AGENTS.md` rule 8 -- a mechanism, not a caution):

  1. every id in every re-derived id set must be DONE at HEAD (`completion_atlas._bucket_of`);
     a single non-DONE id aborts the run, because a capability row cannot be closed over a unit
     that still needs it;
  2. every id must still exist in the HEAD inventory -- a row closed by a unit that silently
     vanished is not closed;
  3. every row's disposition must be exactly one of the two, and a `built: true` row's cited
     file must exist at HEAD.

Recorded discrepancy (correction `1788994085684-at-35-e5-005-ca03fd`, see the cycle receipt): SD-34's register states
`oracle_probe_surface_for_no_table_kinds` `population: 2062` with a five-kind breakdown, but the
row's OWN stated `re_derive_command` run against the inventory at the register's OWN stated
`generated_at_head` (`837dbbcf6b`) returns **130** (ability 90, template 36, companion 4). This
script uses the live re-derivation, 130, and carries SD-34's 2062 alongside it as
`predecessor_stated_population` rather than silently replacing it. The disposition is the same either
way: at HEAD every `oracle-unverifiable` unit in the corpus (8,491 of them, a superset of both
numbers) is DONE.
"""
import argparse
import collections
import json
import os
import subprocess
import sys

REPO = subprocess.run(["git", "rev-parse", "--show-toplevel"],
                      capture_output=True, text=True, check=True).stdout.strip()
sys.path.insert(0, os.path.join(REPO, "scripts"))
import completion_atlas as ca  # noqa: E402

PREDECESSOR_REGISTER = os.path.join(
    REPO, "docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/capability-register.json")
OUT = os.path.join(REPO, "docs/release/SD-35-corpus-sheet-completion/artifacts/"
                         "epic-5-residues/capability-register-rederived.json")
RE_DERIVE = ("python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/"
             "AT-35-E5-005_capability_register.py")


def head_sha() -> str:
    return subprocess.run(["git", "rev-parse", "HEAD"], cwd=REPO,
                          capture_output=True, text=True, check=True).stdout.strip()


def inventory_at(sha: str) -> list:
    """The units array of `docs/work-inventory.json` as of `sha` -- never the working tree's."""
    blob = subprocess.run(["git", "show", f"{sha}:docs/work-inventory.json"], cwd=REPO,
                          capture_output=True, check=True).stdout
    return json.loads(blob)["units"]


# --- SD-34's own bucket-X categoriser, copied verbatim from -----------------
# docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/build_capability_register.py
# so this re-derivation partitions bucket X exactly the way the register it is closing did.
def cat_x(evidence: str) -> str:
    if ("combat_style_feat_pool.option." in evidence
            or "bloodline_feat_pool.option." in evidence
            or "progression_subchoices_unresolved" in evidence):
        return "per_character_choice_filter"
    if "advancement_absent" in evidence:
        return "companion_mount_advancement_table"
    if evidence.startswith("grant_token_only_dispatch_row"):
        return "no_capability_needed:dispatch_only_row"
    if evidence.startswith("vacuous_placeholder_row"):
        return "no_capability_needed:vacuous_placeholder"
    if "DEFERRED_WITH_REASON" in evidence:
        return "marker_stripping"
    return "class_feature_deep_subsystem"


def predecessor_id_sets(old_units: list) -> dict:
    """The register's sized rows, as id sets, by the register's own stated queries."""
    sets = {}
    x_by_cat = collections.defaultdict(list)
    for unit in old_units:
        if unit.get("status") == "deferred-with-reason":
            x_by_cat[cat_x(unit.get("evidence") or "")].append(unit["id"])
    sets["per_character_choice_filter"] = x_by_cat["per_character_choice_filter"]
    sets["companion_mount_advancement_table"] = x_by_cat["companion_mount_advancement_table"]
    sets["class_feature_deep_subsystem_modelling"] = x_by_cat["class_feature_deep_subsystem"]

    a_units = [u for u in old_units
               if u.get("status") == "engine-does-not-hold"
               and "has_no_engine_table" in (u.get("evidence") or "")]
    sets["power_engine_table"] = [u["id"] for u in a_units if u["kind"] == "power"]
    sets["companion_table_shape_widening"] = [u["id"] for u in a_units if u["kind"] == "companion"]

    sets["oracle_probe_surface_for_no_table_kinds"] = [
        u["id"] for u in old_units
        if u.get("status") == "oracle-unverifiable"
        and "AT-33-E1-003 probe-surface census" in (u.get("reason") or "")]

    # SD-34 recorded this row's population as `cited` (21, from two AT-34-E4-00x receipts) with
    # no live query. It IS live-resolvable at the register's own head, and resolves to exactly
    # the cited 21: the whole of `ultimate_campaign`'s bucket-U population, every one of them a
    # `uca_feats.lst` feat carrying the sub-cause the marker DEFINES.
    sets["marker_stripping_for_pcgen_editorial_markers"] = [
        u["id"] for u in old_units
        if u.get("status") == "unmeasurable"
        and u.get("book") == "ultimate_campaign"
        and u.get("evidence") == "feat_served_description_is_a_placeholder_marker_not_prose"]
    return sets


def by_source_file(units: list, source_file: str) -> list:
    return [u["id"] for u in units if u.get("source_file") == source_file]


def index_sheet_rules() -> dict:
    by_id = {}
    for dirpath, _dn, filenames in os.walk(os.path.join(REPO, "data/sheet_rules")):
        for name in filenames:
            if not name.endswith(".json") or name.startswith("_"):
                continue
            with open(os.path.join(dirpath, name), "r", encoding="utf-8") as fh:
                try:
                    rules = json.load(fh)
                except json.JSONDecodeError:
                    continue
            if isinstance(rules, list):
                for rule in rules:
                    if isinstance(rule, dict) and "id" in rule:
                        by_id.setdefault(rule["id"], rule)
    return by_id


def head_state(ids, head_by_id) -> dict:
    """The closure evidence for one row: where its units are at HEAD."""
    missing = [i for i in ids if i not in head_by_id]
    if missing:
        raise SystemExit(f"{len(missing)} unit ids vanished from the inventory: {missing[:5]}")
    buckets = collections.Counter(ca._bucket_of(head_by_id[i]) for i in ids)
    non_done = [i for i in ids if ca._bucket_of(head_by_id[i]) != "DONE"]
    if non_done:
        raise SystemExit(
            f"{len(non_done)} of {len(ids)} units on a capability row are NOT DONE at HEAD -- "
            f"the row cannot be closed and this cycle must STOP and report: {non_done[:10]}")
    return {
        "units_at_head": len(ids),
        "bucket_at_head": dict(buckets),
        "status_at_head": dict(sorted(collections.Counter(
            head_by_id[i].get("status") for i in ids).items())),
        "evidence_at_head": dict(sorted(collections.Counter(
            head_by_id[i].get("evidence") for i in ids).items(), key=lambda kv: -kv[1])[:5]),
    }


def exists(rel: str) -> bool:
    return os.path.exists(os.path.join(REPO, rel))


SHEET_RULE = "decisions.md §1"


def build() -> dict:
    predecessor = json.load(open(PREDECESSOR_REGISTER, encoding="utf-8"))
    predecessor_head = predecessor["generated_at_head"]
    old_units = inventory_at(predecessor_head)
    head_units = json.load(open(os.path.join(REPO, "docs/work-inventory.json"),
                                encoding="utf-8"))["units"]
    head_by_id = {u["id"]: u for u in head_units}
    sets = predecessor_id_sets(old_units)
    rules = index_sheet_rules()

    # Two of SD-34's rows are `population_source: "cited"` -- a named, static record count from a
    # prior receipt with no live evidence-string query (the register says so itself, in its own
    # `verification_note`). Both name their source .lst file, and the inventory carries
    # `source_file` per unit, so both ARE live-resolvable at HEAD by that field. That is the
    # live citation the register's note asked the next lane to pin.
    sets["monster_class_hit_dice_progression_modelling"] = by_source_file(
        head_units, "cr_classes_companion.lst")
    sets["master_side_ability_pool_record_type_or_cross_book_ownership"] = by_source_file(
        head_units, "ce_abilities_familiar_cr.lst")

    # The two UNSIZED rows (atlas-defects.md #2, meanings 2 and 3). SD-34 could not size them;
    # at HEAD both shapes ARE measurable, because the converted package makes the distinction
    # mechanical: a rule with no prose of its own either points at a granting rule that owns the
    # content (meaning 2) or has no content anywhere because the corpus record's description is
    # empty upstream (meaning 3).
    pointer_ids, gap_ids = [], []
    for unit in head_units:
        rule = rules.get(unit["id"])
        if rule is None:
            continue
        pieces = sum(len(f.get("pieces") or []) for f in (rule.get("prose") or []))
        if pieces or rule.get("value") != "Text":
            continue
        (pointer_ids if rule.get("granted_by") else gap_ids).append(unit["id"])
    sets["cross_record_content_ownership_resolution"] = pointer_ids
    sets["corpus_content_extraction_for_uncaptured_records"] = gap_ids

    dispositions = {
        "power_engine_table": {
            "built": True,
            "built_evidence": (
                "The `power` table exists and is served by the live sheet-rule package, which "
                "loads `SheetRule.applies` and never a source token (`epic-breakdown.md "
                "### AT-35-E5-001`). Its refusal/success transcript pair is "
                "`artifacts/epic-5-residues/table-proofs.md`."),
            "built_files": ["data/sheet_rules/ultimate_psionics/power/",
                            "src/rules_core/sheet_rule.rs",
                            "src/rules_core/corpus_loader.rs"],
            "verify_commands": [
                "python3 scripts/missing_engine_tables.py --check   # population=0 kinds=0",
                "cargo run --locked --bin v06_work_inventory -- --epic5-table-transcript"],
        },
        "companion_table_shape_widening": {
            "built": True,
            "built_evidence": (
                "Same mechanism as `power_engine_table`: `bestiary`'s `companion` records are "
                "served by the live sheet-rule package's `companion` table, gated by a typed "
                "`Applies` tree. Transcript pair in `artifacts/epic-5-residues/table-proofs.md`."),
            "built_files": ["data/sheet_rules/bestiary/companion/",
                            "src/rules_core/sheet_rule.rs"],
            "verify_commands": [
                "python3 scripts/missing_engine_tables.py --check   # population=0 kinds=0",
                "cargo run --locked --bin v06_work_inventory -- --epic5-table-transcript"],
        },
        "per_character_choice_filter": {
            "built": True,
            "built_evidence": (
                "AT-35-E5-004 built the join SD-34 named as missing: `filter_option_pool` takes "
                "a character's facts and the option pool and returns the options THIS character "
                "may take plus the ones it refuses, and desktop's `preview_level_up` serves both "
                "as `featOptions` / `refusedFeatOptions`. Prerequisites are typed `Applies` "
                "converted at ingest -- no `pre_tokens` on the live side (`decisions.md §11`)."),
            "built_files": ["src/rules_core/level_up_option_filter.rs",
                            "apps/desktop/src-tauri/src/character_hub.rs",
                            "apps/desktop/src/characterHub/LevelUpDialog.tsx"],
            "verify_commands": [
                "grep -n 'filter_option_pool' src/rules_core/level_up_option_filter.rs "
                "apps/desktop/src-tauri/src/character_hub.rs",
                "cat docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/"
                "AT-35-E5-004_cycle1_receipt.md"],
        },
        "marker_stripping_for_pcgen_editorial_markers": {
            "built": True,
            "built_evidence": (
                "AT-35-E5-003 decided the product question SD-34 filed and built the fix on the "
                "CONVERTER side: upstream PCGen's editorial not-implemented marker no longer "
                "reaches a rendered sheet line. A statement about PCGen's automation is not the "
                "rule's words, so under the sheet rule it does not belong on paper."),
            "built_files": ["src/pcgen_import/sheet_rule/prose.rs",
                            "tests/sheet_rule_convert_gate.rs"],
            "verify_commands": [
                "grep -rlEi '\\[(not implemented|ml bonus not implemented)' data/sheet_rules/ "
                "| wc -l   # 0",
                "cat docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/"
                "AT-35-E5-003_cycle1_receipt.md"],
        },
        "cross_record_content_ownership_resolution": {
            "built": True,
            "built_evidence": (
                "The converter emits `granted_by` on a pointer row, naming the rule that owns "
                "the content, and the LIVE evaluator follows it: `src/rules_core/sheet_rule.rs` "
                "walks `granted_by` when it computes what a character holds. That is exactly the "
                "shape SD-34 said no bucket named -- 'the content exists, on a DIFFERENT "
                "record'. The row's population is the pointer rows themselves, measured here for "
                "the first time (SD-34 left it UNSIZED)."),
            "built_files": ["src/rules_core/sheet_rule.rs",
                            "src/pcgen_import/sheet_rule/convert.rs"],
            "verify_commands": [
                "grep -n 'granted_by' src/rules_core/sheet_rule.rs",
                "python3 -c \"import json;m=json.load(open('docs/release/"
                "SD-35-corpus-sheet-completion/artifacts/epic-5-residues/completion-manifest.json'));"
                "print(m['summary']['by_sheet_rule_content']['label_only_with_granted_by'])\""],
        },
        "companion_mount_advancement_table": {
            "built": False,
            "reason": (
                "A per-level stat-progression engine for companions/eidolons/mounts is a "
                "simulation of levelling, not a sheet line. Under the sheet rule the "
                "advancement record prints what the rulebook prints -- the rule's words, or one "
                "final number where the corpus states one -- and every unit this row named "
                "reaches one of those forms at HEAD without any advancement engine."),
        },
        "class_feature_deep_subsystem_modelling": {
            "built": False,
            "reason": (
                "Bardic performance variants, eidolon evolutions, mystery revelations, spirit "
                "powers, exploits: each is a subsystem someone might want a game engine to "
                "execute. A paper sheet prints the feature and its text; the player runs the "
                "subsystem at the table. Every unit this row named renders a finished sheet "
                "line at HEAD (20 as a final number, 11 as the rule's words) with no subsystem "
                "modelled."),
        },
        "monster_class_hit_dice_progression_modelling": {
            "built": False,
            "reason": (
                "A monster-class hit-dice progression is a new RECORD TYPE for the engine to "
                "advance a creature level by level -- simulation machinery. Both records this "
                "row named (`Companion`, `Shadow Companion`, `cr_classes_companion.lst`) resolve "
                "to a real companion record at HEAD and print it; the standing SD-29 "
                "architecture decision this row asked to widen stays unwidened, and no unit "
                "needs it."),
        },
        "master_side_ability_pool_record_type_or_cross_book_ownership": {
            "built": False,
            "reason": (
                "Neither of the two things this row offered (a cross-book ownership shape, or a "
                "dedicated master-side ability-pool record type) was built. All 14 "
                "`ce_abilities_familiar_cr.lst` rows are DONE at HEAD and every one of them "
                "renders real content -- the sheet prints the familiar ability's words whether "
                "or not the engine can attribute the row to a same-book creature. The "
                "ownership invariant was a table-placement rule, and the sheet rule does not "
                "route content through it."),
        },
        "corpus_content_extraction_for_uncaptured_records": {
            "built": False,
            "reason": (
                "SD-34 left this UNSIZED; it is sized here. These are rules whose corpus record "
                "carries no description upstream at all -- PCGen's own row has no DESC to "
                "convert. Under the sheet rule the finished line for such a record is the "
                "feature's NAME, which is exactly what a player writes on paper next to a "
                "class-features entry, so no engine capability is missing. Recovering the "
                "published rules text these records never captured is a CORPUS-ACQUISITION "
                "wish, not engine machinery, and it moves no unit out of DONE. NOTE the named "
                "exception this cycle measured and did NOT fold in: 10 units DO have a real "
                "DESC token upstream whose words never reach the sheet rule -- a converter "
                "defect, reported as this cycle's residue, not part of this row."),
        },
        "oracle_probe_surface_for_no_table_kinds": {
            "built": False,
            "reason": (
                "An oracle probe surface is VERIFICATION machinery: it exists to compare a "
                "computed magnitude against PCGen's export. The kinds in this row carry no "
                "engine compute table because their sheet line is not a computed magnitude -- "
                "it is the rule's words. Building probes so a magnitude that does not exist can "
                "be compared is `decisions.md §4`'s per-unit proof machinery, which this bundle "
                "does not build. AT-35-E4-002 ran the one corpus-wide oracle comparison the "
                "bundle owes on bucket V; every `oracle-unverifiable` unit in the corpus is DONE "
                "at HEAD."),
        },
    }

    rows = []
    for predecessor_row in predecessor["capabilities"]:
        cid = predecessor_row["id"]
        ids = sorted(sets[cid])
        disp = dispositions[cid]
        row = {
            "id": cid,
            "predecessor_stated_population": predecessor_row.get("population"),
            "predecessor_population_source": predecessor_row.get("population_source"),
            "predecessor_re_derive_command": predecessor_row.get("re_derive_command"),
            "rederived_id_set_size": len(ids),
            "rederived_from": (
                f"the register's own query, run against `git show {predecessor_head}:"
                f"docs/work-inventory.json` (its own `generated_at_head`)"
                if cid in ("per_character_choice_filter", "companion_mount_advancement_table",
                           "class_feature_deep_subsystem_modelling", "power_engine_table",
                           "companion_table_shape_widening",
                           "marker_stripping_for_pcgen_editorial_markers",
                           "oracle_probe_surface_for_no_table_kinds")
                else "a live query at HEAD (see `rederive_command` below)"),
            "rederive_command": {
                "monster_class_hit_dice_progression_modelling":
                    "source_file == 'cr_classes_companion.lst', over docs/work-inventory.json",
                "master_side_ability_pool_record_type_or_cross_book_ownership":
                    "source_file == 'ce_abilities_familiar_cr.lst', over docs/work-inventory.json",
                "cross_record_content_ownership_resolution":
                    "sheet_rule_content == 'label_only_with_granted_by', over completion-manifest.json",
                "corpus_content_extraction_for_uncaptured_records":
                    "sheet_rule_content == 'label_only', over completion-manifest.json",
                "marker_stripping_for_pcgen_editorial_markers":
                    "status=='unmeasurable' AND book=='ultimate_campaign' AND "
                    "evidence=='feat_served_description_is_a_placeholder_marker_not_prose', over "
                    f"`git show {predecessor_head}:docs/work-inventory.json` -- resolves to the cited 21",
            }.get(cid, predecessor_row.get("re_derive_command")),
        }
        row.update(head_state(ids, head_by_id))
        if disp["built"]:
            row["built"] = True
            row["built_evidence"] = disp["built_evidence"]
            row["built_files"] = disp["built_files"]
            row["verify_commands"] = disp["verify_commands"]
            for path in disp["built_files"]:
                if not exists(path):
                    raise SystemExit(f"{cid}: cited built file is absent at HEAD: {path}")
        else:
            row["built"] = False
            row["unnecessary-under-sheet-rule"] = disp["reason"]
            row["sheet_rule_citation"] = SHEET_RULE
        if cid == "oracle_probe_surface_for_no_table_kinds":
            row["predecessor_population_discrepancy"] = (
                "SD-34 states 2062 (ability 745, companion 104, monster 843, monster_ability "
                "244, template 126). The row's own stated command at the register's own stated "
                "generated_at_head returns 130 (ability 90, template 36, companion 4). Recorded "
                "as a correction rather than silently replaced; the disposition is unchanged "
                "because at HEAD all 8,491 oracle-unverifiable units in the corpus are DONE.")
        rows.append(row)

    built = [r for r in rows if r["built"]]
    unnecessary = [r for r in rows if not r["built"]]
    if len(built) + len(unnecessary) != len(rows) or len(rows) != len(predecessor["capabilities"]):
        raise SystemExit("every SD-34 row must carry exactly one of the two dispositions")

    return {
        "criterion": "AT-35-E5-005",
        "generated_at_head": head_sha(),
        "re_derive_command": RE_DERIVE,
        "closes": ("docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/"
                   "capability-register.json"),
        "predecessor_register_head": predecessor_head,
        "disposition_vocabulary": {
            "built": "the capability exists in live code at HEAD; `built_evidence` names it",
            "unnecessary-under-sheet-rule":
                "decisions.md §1 -- the capability was simulation or verification machinery; "
                "every unit the row named reaches one of the three sheet forms without it",
        },
        "summary": {
            "predecessor_rows": len(predecessor["capabilities"]),
            "rows_closed": len(rows),
            "built": len(built),
            "unnecessary_under_sheet_rule": len(unnecessary),
            "still_open": 0,
            "units_covered": sum(r["rederived_id_set_size"] for r in rows),
            "units_covered_not_done_at_head": 0,
            "built_ids": sorted(r["id"] for r in built),
            "unnecessary_ids": sorted(r["id"] for r in unnecessary),
        },
        "capabilities": rows,
    }


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args(argv)
    doc = build()
    text = json.dumps(doc, indent=1) + "\n"
    if args.check:
        if not os.path.exists(OUT):
            print(f"MISSING {OUT}")
            return 1
        disk = json.load(open(OUT, encoding="utf-8"))
        fresh = json.loads(text)
        fresh.pop("generated_at_head"), disk.pop("generated_at_head")
        if fresh != disk:
            print("CAPABILITY REGISTER DRIFTED from a fresh build")
            return 1
        s = doc["summary"]
        print(f"rows={s['rows_closed']} built={s['built']} "
              f"unnecessary_under_sheet_rule={s['unnecessary_under_sheet_rule']} "
              f"still_open={s['still_open']} units_covered={s['units_covered']} verdict=PASS")
        return 0
    with open(OUT, "w", encoding="utf-8") as fh:
        fh.write(text)
    s = doc["summary"]
    print(f"wrote {OUT}")
    print(f"rows={s['rows_closed']} built={s['built']} "
          f"unnecessary_under_sheet_rule={s['unnecessary_under_sheet_rule']} "
          f"still_open={s['still_open']} units_covered={s['units_covered']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
