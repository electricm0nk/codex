#!/usr/bin/env python3
"""SD-37 E5.1: the features the converted Starfinder records grant each SF seed at its level.

An implementation independent of the engine's held-set fixpoint (`sheet_rule::held_set`): it
reads the package JSON directly and walks `granted_by` edges from the seed's own records.

Population ("a feature"): a principal rule (no `#` in its id) with `print: true`, of kind race,
class, ability, feat, pool_option or template, whose `pool` is not `weapon` (`CATEGORY:Weapon`
abilities are per-weapon attack/damage terms, printed on a weapon's line, not as features).

Roots: the race record, the class record, the class's level-1 BaseClass template, every
`selected_feats` id (theme, racial picks, feats) and every `selected_choices` selection that is
a rule id. A rule is granted when one of its `granted_by` entries names a granted rule (`Rule`)
or the seed's class at a level <= the seed's class level (`Class`), and both that entry's `when`
and the rule's own `applies` hold. Gates are read only over Const / ClassLevel / Level leaves;
a gate with any other leaf (a `Var`, a `Holds`) is UNDECIDED and goes to the hand table below,
never guessed. `Not`/`Holds` gates on archetype facts are read as open (no seed takes an
archetype) -- the receipt names this as not covered.

Usage: python3 E5.1_seed_features.py <repo root>          -> prints the fixture table
       python3 E5.1_seed_features.py <repo root> --check  -> exit 1 if the table in
                                                             E5.1-seed-features.md differs
"""
import glob
import json
import os
import sys

SEEDS = {
    # seed id: (class slug, level, race id, selected_feats, rule-id choice selections)
    # Read from apps/desktop/src-tauri/src/{rule_system_adapter.rs (sf_soldier_3_input), sf_adapter.rs (seeds)}.
    # E5.2: every seed's feats from seed-builds.md (E0.4) added to its selected_feats.
    "SF-Soldier-3": ("soldier", 3, "core:race:human",
                     ["core:ability:mercenary", "core:ability:2_racial_stat_bonus",
                      "core:feat:weapon_focus", "core:feat:quick_draw", "core:feat:deadly_aim",
                      "core:feat:coordinated_shot"], []),
    "SF-Mystic-5": ("mystic", 5, "core:race:lashunta",
                    ["core:ability:priest", "core:ability:empath", "core:ability:2_racial_bonus_to_skill",
                     "core:ability:lashunta_subrace_damaya", "core:feat:spell_penetration",
                     "core:feat:spell_focus", "core:feat:quick_draw"], []),
    "SF-Technomancer-5": ("technomancer", 5, "core:race:android",
                          ["core:ability:scholar", "core:feat:spell_penetration", "core:feat:spell_focus",
                           "core:feat:mobility", "core:feat:quick_draw"],
                          ["core:pool_option:scholar_theme_chosen_skill_physical_science"]),
    "SF-Envoy-3": ("envoy", 3, "core:race:ysoki",
                   ["core:ability:icon", "core:feat:mobility", "core:feat:quick_draw"], []),
}

FEATURE_KINDS = {"race", "class", "ability", "feat", "pool_option", "template"}

# Gates this walk cannot read (a Var leaf), decided by hand from the SRD. Key: (seed, rule id).
HAND = {
    # Empath connection powers: SRD https://www.aonsrd.com/Connections.aspx?ItemName=Empath --
    # "Empathy (1st)", "Greater Mindlink (3rd)", then 6th, 9th, 12th, 15th, 18th. The gate's Var is
    # "Mystic Highest Connection Power LVL" (_vars/v7935441f0ec9295f.json); a Mystic 5 has 1st and 3rd.
    ("SF-Mystic-5", "core:ability:empath_connection_power_empathy"): (True, "SRD Empath: Empathy (1st)"),
    ("SF-Mystic-5", "core:ability:empath_connection_power_greater_mindlink"): (True, "SRD Empath: Greater Mindlink (3rd)"),
    ("SF-Mystic-5", "core:ability:empath_connection_power_emotionsense"): (False, "SRD Empath: Emotionsense (6th) > 5"),
    ("SF-Mystic-5", "core:ability:empath_connection_power_discern_lies"): (False, "SRD Empath: Discern Lies (9th) > 5"),
    ("SF-Mystic-5", "core:ability:empath_connection_power_greater_emotionsense"): (False, "SRD Empath: Greater Emotionsense (12th) > 5"),
    ("SF-Mystic-5", "core:ability:empath_connection_power_retrocognition"): (False, "SRD Empath: Retrocognition (15th) > 5"),
    ("SF-Mystic-5", "core:ability:empath_connection_power_empathic_mastery"): (False, "SRD Empath: Empathic Mastery (18th) > 5"),
}


def load(root):
    rules = {}
    for f in sorted(glob.glob(os.path.join(root, "data/starfinder-1e/sheet_rules/*/*/*.json"))):
        for r in json.load(open(f, encoding="utf-8")):
            rules[r["id"]] = r
    return rules


def leaf(e, cls, level):
    if isinstance(e, dict):
        if "Const" in e:
            return e["Const"]
        if "ClassLevel" in e:
            return level if e["ClassLevel"] == cls else 0
    if e == "Level":
        return level
    return None


def gate(w, cls, level):
    """True / False, or None when a leaf cannot be read here."""
    if w == "Always":
        return True
    (k, v), = w.items()
    if k == "Compare":
        a, b = leaf(v["lhs"], cls, level), leaf(v["rhs"], cls, level)
        if a is None or b is None:
            return None
        return {"Gte": a >= b, "Gt": a > b, "Lte": a <= b, "Lt": a < b, "Eq": a == b, "Ne": a != b}[v["op"]]
    if k == "All":
        parts = [gate(x, cls, level) for x in v]
        if False in parts:
            return False
        return None if None in parts else True
    if k in ("Not", "Holds"):
        return True  # archetype / fact gates: open for these seeds (no archetype taken)
    return None


def template_of(rules, cls):
    want = {"Compare": {"lhs": {"ClassLevel": cls}, "op": "Gte", "rhs": {"Const": 1}}}
    found = [r["id"] for r in rules.values()
             if r["id"].split(":")[1] == "template" and "#" not in r["id"]
             and "BaseClass" in (r.get("tags") or []) and r["applies"] == want]
    assert len(found) == 1, (cls, found)
    return found[0]


def walk(rules, seed):
    cls, level, race, feats, choices = SEEDS[seed]
    granted = {race, "core:class:" + cls, template_of(rules, cls), *feats, *choices}
    undecided = set()
    changed = True
    while changed:
        changed = False
        for r in rules.values():
            rid = r["id"]
            if "#" in rid or rid in granted:
                continue
            for g in r.get("granted_by") or []:
                by = g["by"]
                src = ("Rule" in by and by["Rule"] in granted) or (
                    "Class" in by and by["Class"]["id"] == cls and by["Class"]["at_level"] <= level)
                if not src:
                    continue
                ok = gate(g["when"], cls, level)
                ok_applies = gate(r["applies"], cls, level)
                if ok is None or ok_applies is None:
                    if (seed, rid) in HAND:
                        ok, ok_applies = HAND[(seed, rid)][0], True
                    else:
                        undecided.add(rid)
                        continue
                if ok and ok_applies:
                    granted.add(rid)
                    changed = True
                    break
    features = sorted(i for i in granted
                      if rules[i].get("print") and rules[i].get("pool") != "weapon"
                      and i.split(":")[1] in FEATURE_KINDS)
    return features, sorted(undecided - granted)


def table(root):
    rules = load(root)
    out = ["| Seed | Rule id | Label | Granted by |", "|---|---|---|---|"]
    undecided_all = []
    for seed in SEEDS:
        features, undecided = walk(rules, seed)
        undecided_all += [(seed, u) for u in undecided]
        for rid in features:
            by = "; ".join(
                (f"Rule {g['by']['Rule']}" if "Rule" in g["by"] else
                 f"Class {g['by']['Class']['id']} {g['by']['Class']['at_level']}" if "Class" in g["by"] else
                 next(iter(g["by"])))
                for g in (rules[rid].get("granted_by") or [])[:2]) or "seed root"
            hand = HAND.get((seed, rid))
            if hand:
                by += f" (hand: {hand[1]})"
            out.append(f"| {seed} | `{rid}` | {rules[rid]['label']} | {by} |")
    return out, undecided_all


def main():
    root = sys.argv[1]
    rows, undecided = table(root)
    if undecided:
        print("UNDECIDED (add to HAND):", undecided)
        sys.exit(1)
    if "--check" in sys.argv:
        md = open(os.path.join(root, "docs/release/SD-37-starfinder-1e/artifacts/epic_5/E5.1-seed-features.md"), encoding="utf-8").read()
        have = [l for l in md.splitlines() if l.startswith("| SF-")]
        want = [l for l in rows if l.startswith("| SF-")]
        if have != want:
            print(f"DIFFERS: fixture {len(have)} rows, walk {len(want)} rows")
            for l in sorted(set(have) ^ set(want)):
                print("  ", l)
            sys.exit(1)
        print(f"OK fixture = walk ({len(want)} rows)")
        return
    print("\n".join(rows))
    counts = {}
    for l in rows[2:]:
        counts[l.split(" | ")[0][2:]] = counts.get(l.split(" | ")[0][2:], 0) + 1
    print("\nper seed:", counts, "total:", sum(counts.values()))


if __name__ == "__main__":
    main()
