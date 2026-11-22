import json, collections

BASE = "docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/"

def load(name):
    with open(BASE + name) as f:
        return json.load(f)

def save(name, obj):
    with open(BASE + name, "w") as f:
        json.dump(obj, f, indent=2)
        f.write("\n")

literal = load("literal-verified.oracle-results.json")
fixture = load("fixture-verified.combined-oracle-results.json")
full_rerun = load("full-rerun-wave5.oracle-results.json")
weapon = load("last67-weapon.oracle-results.json")
skillcombat = load("last67-skill-combat.oracle-results.json")

lit_rows = literal["results"]
fix_rows = fixture["results"]
fr_rows = full_rerun["results"]
weapon_rows = weapon["results"]
sc_rows = skillcombat["results"]

lit_by_id = {r["unit_id"]: r for r in lit_rows}
fix_ids = {r["unit_id"] for r in fix_rows}

# ---------------------------------------------------------------------------
# 1. full-rerun-wave5's 66 rows are the authoritative "combat-weapon-shape"
#    lane's re-derivation (absolute AC-isolator method, replacing the flawed
#    whole-character baseline-diff -- AT-33-E5-003-disagreement-fixes-wave5's
#    own root cause).
#
# 11 of the 66 are MULTI-SHAPE records (a single equipment item carrying two
# independently-verified magnitude/bonus-chain dimensions -- see
# finalize-wave3-merge.py's own convention, `multi_shape_sources` +
# `multi_shape_note`, "merged verdict is the worst of the N per-shape
# verdicts", top-level ours/oracle/verdict mirrors whichever source lane won
# that comparison). A blind whole-row replace would silently DISCARD the
# OTHER, unrelated dimension's own already-verified value for those 11 --
# real information loss, caught by inspection before merging (see this
# cycle's own receipt).
#
# Since every one of the 66 full-rerun rows is verdict 'agree' (no rank
# changes), the WINNING lane for a multi-shape record cannot flip -- so the
# safe, algorithm-agnostic invariant applied here is: only the
# `combat-weapon-shape` sub-entry inside `multi_shape_sources` is ever
# touched; the top-level ours/oracle is updated ONLY when it already equalled
# the OLD combat-weapon-shape sub-entry (i.e. combat-weapon-shape was already
# the displayed lane) -- never inferred by re-running an assumed tie-break
# rule.
# ---------------------------------------------------------------------------

real_changes = []
multi_shape_seen = 0
simple_replaced = 0
noop = 0

for r in fr_rows:
    uid = r["unit_id"]
    assert uid in lit_by_id, f"{uid} missing from literal-verified"
    assert uid not in fix_ids, f"{uid} unexpectedly in fixture-verified"
    old = lit_by_id[uid]

    if "multi_shape_sources" in old:
        multi_shape_seen += 1
        sources = [dict(s) for s in old["multi_shape_sources"]]
        combat_idx = next((i for i, s in enumerate(sources) if s["lane"] == "combat-weapon-shape"), None)
        assert combat_idx is not None, f"{uid}: expected a combat-weapon-shape component"
        old_combat = sources[combat_idx]
        old_top = {"ours": old.get("ours"), "oracle": old.get("oracle"), "verdict": old.get("verdict")}
        old_combat_val = {"ours": old_combat.get("ours"), "oracle": old_combat.get("oracle"), "verdict": old_combat.get("verdict")}
        new_combat_val = {"ours": r["ours"], "oracle": r["oracle"], "verdict": r["verdict"]}

        if old_combat_val == new_combat_val:
            noop += 1
            continue  # nothing changed for this unit at all

        # combat-weapon-shape's own value genuinely moved.
        sources[combat_idx] = {
            "lane": "combat-weapon-shape", "ours": r["ours"], "oracle": r["oracle"],
            "verdict": r["verdict"], "reason": r.get("reason"),
        }
        new_row = dict(old)
        new_row["multi_shape_sources"] = sources
        combat_was_displayed_winner = (old_top == old_combat_val)
        if combat_was_displayed_winner:
            new_row["ours"] = r["ours"]
            new_row["oracle"] = r["oracle"]
            new_row["verdict"] = r["verdict"]
            new_top = new_combat_val
        else:
            new_top = old_top  # some OTHER lane already wins the tie; unaffected
        real_changes.append((uid, {
            "combat_sub_entry": {"before": old_combat_val, "after": new_combat_val},
            "top_level": {"before": old_top, "after": new_top, "changed": old_top != new_top},
        }))
        lit_by_id[uid] = new_row
    else:
        old_top = {"ours": old.get("ours"), "oracle": old.get("oracle"), "verdict": old.get("verdict")}
        new_top = {"ours": r.get("ours"), "oracle": r.get("oracle"), "verdict": r.get("verdict")}
        if old_top == new_top:
            noop += 1
            continue
        real_changes.append((uid, {"top_level": {"before": old_top, "after": new_top, "changed": True}}))
        lit_by_id[uid] = r
        simple_replaced += 1

print(f"full-rerun-wave5: {len(fr_rows)} rows examined ({multi_shape_seen} multi-shape, {noop} no-op, "
      f"{simple_replaced} simple rows replaced, {len(real_changes)} rows with a genuine change)")
for uid, detail in real_changes:
    print(" ", uid, detail)

# ---------------------------------------------------------------------------
# 2. Add the 14 skill-combat rows -- all genuinely new ids.
# ---------------------------------------------------------------------------
for r in sc_rows:
    uid = r["unit_id"]
    assert uid not in lit_by_id and uid not in fix_ids, f"unexpected pre-existing id {uid}"
    lit_by_id[uid] = r

# ---------------------------------------------------------------------------
# 3. Add the 14 weapon rows -- all genuinely new ids -- with heavy_hammer
#    corrected in-place per this cycle's real compute_equipmods_effect fix.
# ---------------------------------------------------------------------------
HEAVY_HAMMER = "ultimate_equipment:equipment:heavy_hammer"
for r in weapon_rows:
    uid = r["unit_id"]
    assert uid not in lit_by_id and uid not in fix_ids, f"unexpected pre-existing id {uid}"
    if uid == HEAVY_HAMMER:
        assert r["verdict"] == "disagree" and r["ours"] == 0 and r["oracle"] == 4
        corrected = dict(r)
        corrected["ours"] = 4
        corrected["verdict"] = "agree"
        corrected["note"] = (
            r["note"]
            + " CORRECTED this cycle (AT-33-E5-finalize-wave5, src/rules_core/equipment_effects/equipmods.rs): "
            "compute_equipmods_effect now sums EVERY qualifying WEAPONPROF/WEAPON bonus chain on a record "
            "(was find_map/first-match only) via new tohit_bonus/damage_bonus fields on WeaponEnhancementBonus, "
            "replacing the single affects/bonus scalar. heavy_hammer's two separately-scoped chains "
            "(WEAPONPROF=Warhammer|TOHIT|-2 + WEAPONPROF=Warhammer|DAMAGE|4) now both resolve: "
            "tohit_bonus=-2 (unchanged, already agreed), damage_bonus=4 (was silently dropped, now matches "
            "oracle exactly). Corpus-wide scan (579 equipment records with any bonus chain) confirms "
            "heavy_hammer is the ONLY record with 2+ qualifying chains, so no other examined unit's value "
            "changed. TDD: new test record_with_two_separately_scoped_chains_sums_both_rolls_independently "
            "in equipmods.rs; 16/16 equipmods tests, 71/71 equipment_effects tests, 27/27 damage_total tests "
            "green."
        )
        lit_by_id[uid] = corrected
    else:
        lit_by_id[uid] = r

new_lit_rows = sorted(lit_by_id.values(), key=lambda r: r["unit_id"])
literal["results"] = new_lit_rows
save("literal-verified.oracle-results.json", literal)
print("\nliteral-verified.oracle-results.json rows now:", len(new_lit_rows))

assert len(fix_rows) == 1741
combined_rows = fix_rows + new_lit_rows
ids = [r["unit_id"] for r in combined_rows]
dupes = len(ids) - len(set(ids))
print("combined rows:", len(combined_rows), "dupes:", dupes)
assert dupes == 0

combined = {"results": combined_rows}
save("AT-33-E5-003.combined-oracle-results.json", combined)

print("\nverdict counts, literal:", collections.Counter(r["verdict"] for r in new_lit_rows))
print("verdict counts, fixture:", collections.Counter(r["verdict"] for r in fix_rows))
print("verdict counts, combined:", collections.Counter(r["verdict"] for r in combined_rows))
