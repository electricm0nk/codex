"""Re-derives artifacts/epic-5-forward-plan/capability-register.json (AT-34-E5-002).

Run from anywhere inside the repo:
    python3 docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/build_capability_register.py

Evidence bar (epic-breakdown.md, AT-34-E5-002): "Beyond the `power` table: anything Epics 3 or
4 proved is required and does not exist." Per capability: what it is, which buckets and books it
unblocks, its population, and whether SD-34 built it.

Two population sources, both cited explicitly per-capability (decisions.md §12 L2 -- never carry
a number forward, re-derive it):

  1. "live" -- computed in this script directly from docs/work-inventory.json / the live
     completion_atlas.py partition, at HEAD, every run.
  2. "cited" -- a static, named-record count that does not change without a corpus edit,
     recorded once by an earlier, already-verified SD-34 cycle receipt and reproduced here with
     its exact source. `verify_capability_register.py` checks these still resolve to real,
     named units in the live corpus (never a bare number trusted on its own).

This artifact is read-only against the rest of the repo (workflow-instruction.md §3): it names
capabilities, it does not build any of them. Every entry's `built_by_sd34` is `false` by
construction -- if a future cycle builds one, delete its row here rather than flip the flag,
since a built capability is no longer "must still be built."
"""
import json, subprocess, sys, os, collections

REPO = subprocess.run(["git", "rev-parse", "--show-toplevel"], capture_output=True, text=True, check=True).stdout.strip()
sys.path.insert(0, os.path.join(REPO, "scripts"))
import completion_atlas as ca

HEAD = subprocess.run(["git", "rev-parse", "HEAD"], cwd=REPO, capture_output=True, text=True, check=True).stdout.strip()

inv = json.load(open(f"{REPO}/docs/work-inventory.json"))
units = inv["units"]

pres = ca.partition(units)
assert not pres["unclassified_ids"] and not pres["overlap_ids"], "live atlas must be clean before pricing capabilities on it"
assert pres["examined"] == 49438, pres["examined"]


def cat_x(evidence: str) -> str:
    """Categorize a bucket-X (deferred-with-reason) unit's evidence string into the named
    capability it maps to, per decisions.md §17 (the choice-filter ruling) and atlas-defects.md
    (dispatch-only rows, vacuous placeholders -- both already correctly resting in X, no
    capability needed)."""
    if "combat_style_feat_pool.option." in evidence or "bloodline_feat_pool.option." in evidence \
            or "progression_subchoices_unresolved" in evidence:
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


def deep_subsystem_name(evidence: str) -> str:
    # engine_diagnostic:class_feature.<source>.<class>.<subsystem>...:claim_blocking=false
    body = evidence.split("engine_diagnostic:", 1)[-1]
    parts = body.split(".")
    if len(parts) >= 3 and parts[0] == "class_feature":
        return parts[2]  # the class/subsystem token, e.g. "bardic_performance_execution" owner
    return body


x_units = [u for u in units if u.get("status") == "deferred-with-reason"]
by_cat = collections.defaultdict(list)
for u in x_units:
    by_cat[cat_x(u.get("evidence") or "")].append(u)

choice_filter_units = by_cat["per_character_choice_filter"]
choice_filter_books = collections.Counter(u["book"] for u in choice_filter_units)

advancement_units = by_cat["companion_mount_advancement_table"]
advancement_books = collections.Counter(u["book"] for u in advancement_units)

deep_units = by_cat["class_feature_deep_subsystem"]
deep_books = collections.Counter(u["book"] for u in deep_units)
deep_sub = collections.Counter()
for u in deep_units:
    name = deep_subsystem_name(u.get("evidence") or "")
    deep_sub[name] += 1

marker_x_units = by_cat["marker_stripping"]

# Sanity: every X unit lands in exactly one category, and the "no capability needed" categories
# plus the four real capability categories exhaust the bucket.
assert sum(len(v) for v in by_cat.values()) == len(x_units) == pres["counts"].get("X", 0)

# missing-engine-tables.json is Epic 1's own artifact (AT-34-E1-003) -- read it rather than
# re-deriving its query here, since re-implementing "which kinds have zero engine table in any
# book" a second way is exactly the kind of second, independently-drifting implementation
# AGENTS.md's "derive counts two ways" rule warns against; this script instead PROVES the cited
# figures still match the live atlas below.
missing_tables_path = os.path.join(REPO, "docs/release/SD-34-book-completion/artifacts/epic-1-atlas/missing-engine-tables.json")
missing_tables = json.load(open(missing_tables_path))
power_count = missing_tables["kinds"]["power"]["count"]
power_book = list(missing_tables["kinds"]["power"]["by_book"].keys())[0]
companion_count = missing_tables["kinds"]["companion"]["count"]
companion_book = list(missing_tables["kinds"]["companion"]["by_book"].keys())[0]

# Cross-check against a live, independent bucket-A query (status == engine-does-not-hold AND
# evidence contains 'has_no_engine_table') so this script does not simply echo the cited file.
a_units = [u for u in units if u.get("status") == "engine-does-not-hold" and "has_no_engine_table" in (u.get("evidence") or "")]
a_by_kind = collections.Counter(u["kind"] for u in a_units)
assert a_by_kind.get("power", 0) == power_count, (a_by_kind.get("power"), power_count)
assert a_by_kind.get("companion", 0) == companion_count, (a_by_kind.get("companion"), companion_count)

capabilities = []

capabilities.append({
    "id": "power_engine_table",
    "what": "An engine table for the `power` kind (psionics powers) -- zero engine table exists "
            "for this kind in any book. AT-34-E2-001 built the other 8 of the 9 kinds that "
            "needed one (technical-design.md §4: 'Epic 2 builds 8 of 9'); this is the ninth, "
            "left for Epic 5. AT-34-E5-003 costs it; nothing in SD-34 builds it.",
    "buckets_unblocked": ["A"],
    "books_unblocked": [power_book],
    "population": power_count,
    "population_source": "live",
    "re_derive_command": "python3 scripts/missing_engine_tables.py --check  (or: status==engine-does-not-hold AND evidence contains 'has_no_engine_table' AND kind=='power', over docs/work-inventory.json)",
    "built_by_sd34": False,
})

capabilities.append({
    "id": "companion_table_shape_widening",
    "what": "The `companion_chassis` table (built in SD-29) exists but does not cover the "
            "specific `companion`-kind record shape held by this book -- every one of this "
            "book's `companion` units sits in bucket A even though a real companion table "
            "exists for other books. Widening the table's coverage (not building a new table) "
            "is the missing capability.",
    "buckets_unblocked": ["A"],
    "books_unblocked": [companion_book],
    "population": companion_count,
    "population_source": "live",
    "re_derive_command": "status==engine-does-not-hold AND evidence contains 'has_no_engine_table' AND kind=='companion', over docs/work-inventory.json",
    "built_by_sd34": False,
})

capabilities.append({
    "id": "per_character_choice_filter",
    "what": "Operator ruling, decisions.md §17: at level-up the UI queries the backend for the "
            "valid choices a SPECIFIC character may take; the backend must filter the full "
            "option pool against that character. Half-built: "
            "list_class_feature_pool_options() (apps/desktop/src-tauri/src/class_feature_pool_picker.rs) "
            "returns the whole static, unfiltered list; evaluate_feat_prerequisites / "
            "evaluate_catalog_feat_prerequisites / character_prereq_facts "
            "(src/rules_core/feat_prereqs.rs) can judge a prerequisite against character facts. "
            "The missing piece is the query joining them: given this character, which options "
            "are valid. No cycle may move a unit here into DONE on the strength of the "
            "unfiltered count alone (decisions.md §17 amendment).",
    "buckets_unblocked": ["X"],
    "books_unblocked": sorted(choice_filter_books.keys()),
    "population": len(choice_filter_units),
    "population_by_book": dict(sorted(choice_filter_books.items())),
    "population_source": "live",
    "re_derive_command": "status=='deferred-with-reason' AND evidence matches 'combat_style_feat_pool.option.' / 'bloodline_feat_pool.option.' / 'progression_subchoices_unresolved', over docs/work-inventory.json",
    "built_by_sd34": False,
})

capabilities.append({
    "id": "companion_mount_advancement_table",
    "what": "A level-based stat-progression (advancement) table for animal companions / "
            "eidolons / mounts / familiars as the master character levels -- distinct from "
            "whether the companion RECORD is held at all (that is the companion_table_shape_"
            "widening and companion_absent bucket-B mechanisms). This capability is the "
            "per-level scaling math itself; it is currently absent for every class whose "
            "companion/mount advances.",
    "buckets_unblocked": ["X"],
    "books_unblocked": sorted(advancement_books.keys()),
    "population": len(advancement_units),
    "population_by_book": dict(sorted(advancement_books.items())),
    "population_source": "live",
    "re_derive_command": "status=='deferred-with-reason' AND evidence contains 'advancement_absent', over docs/work-inventory.json",
    "built_by_sd34": False,
})

capabilities.append({
    "id": "class_feature_deep_subsystem_modelling",
    "what": "A cluster of distinct class-feature subsystems that are modelled only at the "
            "basic/granted level; the deeper mechanic each one names is not modelled at all "
            "(e.g. bardic performance variants beyond the base performance, summoner eidolon "
            "evolutions, oracle mystery revelations beyond the base, shaman spirit powers "
            "beyond the base, unchained barbarian/rogue Improved Uncanny Dodge's corpus-cited "
            "shape, arcanist exploits, bloodrager other features). Each named sub-mechanism "
            "below is its own buildable capability, not one undifferentiated blob -- grouped "
            "here because they share one shape (basic grant exists, named deeper mechanic does "
            "not), not because they are the same fix.",
    "buckets_unblocked": ["X"],
    "books_unblocked": sorted(deep_books.keys()),
    "population": len(deep_units),
    "population_by_book": dict(sorted(deep_books.items())),
    "sub_mechanisms": [{"name": k, "count": v} for k, v in sorted(deep_sub.items(), key=lambda kv: -kv[1])],
    "population_source": "live",
    "re_derive_command": "status=='deferred-with-reason' AND evidence starts with 'engine_diagnostic:class_feature.' AND not matched by the choice_filter/advancement/dispatch/vacuous/marker patterns above, over docs/work-inventory.json",
    "built_by_sd34": False,
})

capabilities.append({
    "id": "marker_stripping_for_pcgen_editorial_markers",
    "what": "PCGen's own `[Not Implemented]`/PI-in-description editorial marker is currently "
            "served verbatim inside the rendered description text alongside real, substantial "
            "mechanical text (Goal/Completion Benefit clauses). AT-34-E4-001 proved the atlas's "
            "own `unmeasurable`/DONE verdict is correct and deliberate for this shape "
            "(SD31-E2-F3-002's uniform-demotion decision) -- this is a SEPARATE, product-facing "
            "question the atlas verdict does not answer: should the marker be stripped from "
            "what the player actually reads. Filed as a named forward candidate rather than "
            "decided unilaterally by one cycle (AT-34-E4-002 receipt).",
    "buckets_unblocked": ["U"],
    "books_unblocked": ["ultimate_campaign"],
    "population": 21,
    "population_note": "21 of ultimate_campaign's feat_tables U-bucket units, confirmed by a "
                        "dedicated corpus-wide test (AT-34-E4-001) to all carry the marker; a "
                        "wider, project-wide occurrence count of the SAME marker string "
                        "(~392, per that receipt's own corpus-wide grep) is a separate, "
                        "UNSIZED population this register does not claim as this capability's "
                        "true scope -- named as an open question in the notes, not blended in.",
    "population_source": "cited",
    "cited_from": "artifacts/epic-4-ultimate-campaign/AT-34-E4-001_cycle_receipt.md; artifacts/epic-4-ultimate-campaign/AT-34-E4-002_cycle_receipt.md",
    "built_by_sd34": False,
})

capabilities.append({
    "id": "monster_class_hit_dice_progression_modelling",
    "what": "Two PCGen monster-class definitions (`Companion`, `Shadow Companion` in "
            "cr_classes_companion.lst) are hit-dice level-progression constructs, not a "
            "creature and not an ability. Modelling this shape (a monster/companion CLASS, "
            "distinct from a companion RECORD) is a genuinely new record type -- a standing "
            "SD-29 architecture decision (decisions.md §65.1) this bundle does not widen.",
    "buckets_unblocked": ["B"],
    "books_unblocked": ["core_rulebook"],
    "population": 2,
    "population_source": "cited",
    "cited_from": "artifacts/epic-3-core-rulebook/AT-34-E3-001_companion_absent_cycle_receipt.md",
    "verification_note": "Not independently re-derivable by a live evidence-string query this "
                          "cycle -- the two named records (`Companion`, `Shadow Companion`) do "
                          "not carry a distinct evidence key separating them from ordinary "
                          "class_feature units in the current live partition. A named, static "
                          "count from an already-verified prior cycle receipt; the next lane "
                          "that touches this mechanism should pin a live citation the way "
                          "AT-34-E1-002 condition 6 requires for atlas buckets.",
    "built_by_sd34": False,
})

capabilities.append({
    "id": "master_side_ability_pool_record_type_or_cross_book_ownership",
    "what": "14 `ce_abilities_familiar_cr.lst` master-side familiar special-ability-pool rows, "
            "reattributed to core_rulebook -- a real generic Familiar table exists, but this "
            "book registers NO familiar creature (all 38 of its creatures are Animal "
            "Companions), so `companion_chassis`'s same-book ownership invariant has nothing to "
            "attach these rows to. Closing this needs either a cross-book ownership shape "
            "(Shape 8, not yet built) or a dedicated master-side ability-pool record type.",
    "buckets_unblocked": ["B"],
    "books_unblocked": ["core_rulebook"],
    "population": 14,
    "population_source": "cited",
    "cited_from": "artifacts/epic-3-core-rulebook/AT-34-E3-001_companion_absent_cycle_receipt.md",
    "verification_note": "Same caveat as monster_class_hit_dice_progression_modelling above -- "
                          "a named, static count from an already-verified prior cycle receipt, "
                          "not resolvable by a live evidence-string query this cycle.",
    "built_by_sd34": False,
})

capabilities.append({
    "id": "corpus_content_extraction_for_uncaptured_records",
    "what": "atlas-defects.md #2, meaning 3: some records whose `description` is null and whose "
            "`raw_tokens` carry only structural entries are genuine content gaps -- the corpus "
            "itself never captured the record's published rules text (example: "
            "`Witch Hex ~ Hag's Eye`, advanced_players_guide, 1 of 1 records corpus-wide "
            "bearing that name, no description anywhere). Recovering this content is a "
            "corpus-extraction capability this bundle has not built. The population is "
            "explicitly UNSIZED -- atlas-defects.md states plainly that splitting the 517-record "
            "'no description, structural tokens only' shape into its three distinct meanings "
            "(menu placeholder / cross-record pointer / genuine gap) by EVIDENCE rather than by "
            "shape is required before meaning 3's count is known, and that work has not run.",
    "buckets_unblocked": ["B (candidate; no bucket names this shape yet)"],
    "books_unblocked": ["at least advanced_players_guide; corpus-wide re-derivation not yet run"],
    "population": None,
    "population_status": "UNMEASURED -- explicitly not sized; do not blend into any total",
    "population_source": "cited",
    "cited_from": "artifacts/epic-3-core-rulebook/atlas-defects.md #2",
    "built_by_sd34": False,
})

capabilities.append({
    "id": "cross_record_content_ownership_resolution",
    "what": "atlas-defects.md #2, meaning 2: 'pointer' rows carry no description of their own "
            "but grant/reference content that lives on a different, same-named record elsewhere "
            "in the corpus (examples: `Duergar ~ Stability` -- 14 of 19 same-named records "
            "corpus-wide carry a real description; `Triaxian ~ Keen Senses` -- 11 of 21). This "
            "is a cross-record ownership resolution capability the atlas does not currently "
            "name (neither bucket A's 'no table' nor bucket B's 'not placed' describes 'the "
            "content exists, on a DIFFERENT record'). Population UNSIZED for the same reason as "
            "corpus_content_extraction_for_uncaptured_records above.",
    "buckets_unblocked": ["B (candidate; no bucket names this shape yet)"],
    "books_unblocked": ["at least core_rulebook (Duergar, Triaxian examples); corpus-wide re-derivation not yet run"],
    "population": None,
    "population_status": "UNMEASURED -- explicitly not sized; do not blend into any total",
    "population_source": "cited",
    "cited_from": "artifacts/epic-3-core-rulebook/atlas-defects.md #2",
    "built_by_sd34": False,
})

# decisions.md §19 (operator ruling, 2026-08-29): §17's disposition principle extended to
# bucket V. AT-34-E3-005's consolidated bucket-V oracle ledger dispositioned 2,712 of
# core_rulebook's 2,793 bucket-V units; 130 of those carried the WEAKEST of the two dispositions
# ("unverifiable" -- AT-33-E1-003's own probe-surface census already proved `ability`/
# `template`/`companion` carry no engine compute table at all, so there is nothing for the
# oracle harness to compare against). §19 requires this be carried here as a named capability
# ("build those probes") rather than a closed question -- it is dispositioned, but it is not the
# same as verified-correct. `src/bin/v06_work_inventory.rs`'s `load_bucket_v_oracle_dispositions`
# already reads a *list* of ledger paths, not just core_rulebook's, so this population is computed
# live against the WHOLE corpus and will grow automatically the day a second book's ledger lands
# at one of those paths -- no code change needed here when that happens. As of this cycle no
# second ledger has landed (a salvaged corpus-wide ledger claim did not survive its own session
# uncommitted, `bucket_v_widen_infra_cycle_receipt.md`), so this population is core_rulebook-only
# today; the kind list in "what" is descriptive prose, re-checked against `population_by_kind`
# each run, not itself the gate.
bucket_v_no_probe_surface_units = [
    u for u in units
    if u.get("status") == "oracle-unverifiable"
    and "AT-33-E1-003 probe-surface census" in (u.get("reason") or "")
]
bucket_v_no_probe_surface_by_kind = collections.Counter(u["kind"] for u in bucket_v_no_probe_surface_units)
bucket_v_no_probe_surface_by_book = collections.Counter(u["book"] for u in bucket_v_no_probe_surface_units)

capabilities.append({
    "id": "oracle_probe_surface_for_no_table_kinds",
    "what": "decisions.md §19: bucket-V units dispositioned as `unverifiable` because "
            "AT-33-E1-003's probe-surface census already proved certain kinds carry no engine "
            "compute table at all -- there is no formula-evaluator probe on the engine side to "
            "compare against any PCGen oracle export, structurally, not from a harness timeout "
            "or gap. The kinds actually observed in this population are named in "
            "`population_by_kind` below -- re-check that field each run rather than trusting "
            "this prose list. Infrastructure exists (`load_bucket_v_oracle_dispositions`) to "
            "merge a second, corpus-wide ledger alongside core_rulebook's own once one is built "
            "and landed; none has landed yet, so this population is core_rulebook-only today. "
            "This is the WEAKEST of the two oracle dispositions §19 makes (the ruling's own "
            "words): it says the instrument was never built, not that the oracle cannot express "
            "the value. Building a probe for these kinds is a named forward capability, not a "
            "closed question.",
    "buckets_unblocked": ["V"],
    "books_unblocked": sorted(bucket_v_no_probe_surface_by_book.keys()),
    "population": len(bucket_v_no_probe_surface_units),
    "population_by_kind": dict(sorted(bucket_v_no_probe_surface_by_kind.items())),
    "population_source": "live",
    "re_derive_command": "status=='oracle-unverifiable' AND reason contains 'AT-33-E1-003 probe-surface census', over docs/work-inventory.json",
    "cited_from": "artifacts/epic-3-core-rulebook/bucket-v/AT-34-E3-005_bucket_v_consolidation_cycle_receipt.md; decisions.md §19",
    "built_by_sd34": False,
})

sized = [c for c in capabilities if isinstance(c.get("population"), int)]
unsized = [c for c in capabilities if c.get("population") is None]

out = {
    "criterion": "AT-34-E5-002",
    "generated_at_head": HEAD,
    "re_derive_command": "python3 docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/build_capability_register.py",
    "scope_note": "Evidence bar (epic-breakdown.md): 'Beyond the power table: anything Epics 3 "
                  "or 4 proved is required and does not exist.' Ordinary bucket-B content-"
                  "placement work (place a record in an EXISTING table) is already named and "
                  "priced generically by AT-34-E5-001's forward-plan.json and is NOT repeated "
                  "here -- this register is capabilities: new engine machinery, not yet built, "
                  "that a bucket's ordinary clearing mechanism cannot supply.",
    "capabilities": capabilities,
    "summary": {
        "capability_count": len(capabilities),
        "sized_capability_count": len(sized),
        "unsized_capability_count": len(unsized),
        "total_population_named_sized_only": sum(c["population"] for c in sized),
        "built_by_sd34_count": sum(1 for c in capabilities if c["built_by_sd34"]),
        "not_built_count": sum(1 for c in capabilities if not c["built_by_sd34"]),
        "note": f"{sum(1 for c in capabilities if not c['built_by_sd34'])} of {len(capabilities)} "
                "capabilities named here are NOT built by SD-34 -- this register's whole purpose "
                "is naming what still must be built, not building it (epic-5-forward-plan's "
                "file-touch set is read-only against the rest of the repo).",
    },
    "x_bucket_reconciliation": {
        "live_bucket_X_population": pres["counts"].get("X", 0),
        "per_character_choice_filter": len(choice_filter_units),
        "companion_mount_advancement_table": len(advancement_units),
        "class_feature_deep_subsystem_modelling": len(deep_units),
        "no_capability_needed_dispatch_only_row": len(by_cat["no_capability_needed:dispatch_only_row"]),
        "no_capability_needed_vacuous_placeholder": len(by_cat["no_capability_needed:vacuous_placeholder"]),
        "marker_stripping_x_units": len(marker_x_units),
        "sum_check": len(choice_filter_units) + len(advancement_units) + len(deep_units)
                     + len(by_cat["no_capability_needed:dispatch_only_row"])
                     + len(by_cat["no_capability_needed:vacuous_placeholder"])
                     + len(marker_x_units),
        "note": "Every live bucket-X unit is accounted for by exactly one row above -- the two "
                "'no_capability_needed' rows are atlas-defects.md's own dispatch-only-row and "
                "vacuous-placeholder shapes, already correctly resting in X with no further "
                "engine capability required (named here so the reconciliation is total, not "
                "because they belong in the capability list).",
    },
}

OUT_PATH = os.path.join(REPO, "docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/capability-register.json")
json.dump(out, open(OUT_PATH, "w"), indent=2)
print("WROTE capability-register.json")
print("capabilities:", len(capabilities), "sized:", len(sized), "unsized:", len(unsized))
print("total_population_named_sized_only:", sum(c["population"] for c in sized))
print("X reconciliation sum_check vs live X:", out["x_bucket_reconciliation"]["sum_check"], "vs", pres["counts"].get("X", 0))
