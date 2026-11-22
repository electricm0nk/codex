#!/usr/bin/env python3
"""SD-43 E0.1 -- Abilities census (read-only over the corpus, the engine source and the pinned PCGen oracle).

Answers, with numbers a reader can re-derive by running this file:
  * the seven Abilities sub-tabs (Feats, Racial Abilities, Traits, Archetype, Background,
    Class Features, Misc): which PCGen ABILITYCATEGORY lines feed each one (PCGen titles a sub-tab
    with the category's DISPLAYLOCATION -- AbilityCategory.getType() returns it), how many corpus
    records land in each, by book; where a selection is saved today, budget rows, prerequisite path;
  * every Class Features pool (BONUS:ABILITYPOOL targets on class-granted records) and its members;
  * every class- or archetype-granted choice by mechanism (intake 22c), counted by two independent
    implementations that must agree;
  * pool size per level for Rogue, Barbarian, Ninja (the tokens);
  * the data-gap list (sub-tabs / player-facing pools with zero selectable records).

Usage (from anywhere inside the repo):
    python3 -I <this file>            # write abilities-census.json and abilities-census.md beside it
    python3 -I <this file> --check    # recompute; exit 0 only if every check holds and the
                                      # written JSON equals the recomputation

Exit codes: 0 ok; 1 a check failed; 2 an input is missing (corpus, pool census script, oracle off-pin).
Nothing here is imported by live code; the oracle is only read.
"""
from __future__ import annotations

import argparse
import collections
import glob
import json
import os
import re
import subprocess
import sys

SUBTABS = ["Feats", "Racial Abilities", "Traits", "Archetype", "Background", "Class Features", "Misc"]
# Ability-shaped corpus kinds (the ones PCGen loads from *_abilities*.lst / *_feats.lst).
ABILITY_KINDS = ["ability", "class_feature", "feat", "feat_generic", "race_trait",
                 "race_trait_generic", "trait_generic"]
# Kinds whose records can grant a class choice (22c): class records, class features, Class/Archetype abilities.
GRANT_KINDS = {"class", "class_feature", "ability"}
SELECTABLE_VISIBLE = {"YES", "QUALIFY"}

HERE = os.path.dirname(os.path.abspath(__file__))
OUT_JSON = os.path.join(HERE, "abilities-census.json")
OUT_MD = os.path.join(HERE, "abilities-census.md")


def die(code: int, msg: str):
    print(f"abilities_census: {msg}", file=sys.stderr)
    sys.exit(code)


def run(cmd, cwd, check=True):
    p = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True)
    if check and p.returncode != 0:
        die(2, f"command failed ({p.returncode}): {' '.join(cmd)}\n{p.stderr.strip()}")
    return p


def repo_root() -> str:
    p = subprocess.run(["git", "rev-parse", "--show-toplevel"], cwd=HERE, capture_output=True, text=True)
    if p.returncode != 0:
        die(2, "not inside a git checkout")
    return p.stdout.strip()


# ---------------------------------------------------------------- oracle
def resolve_oracle(root: str) -> dict:
    pin = {}
    pin_file = os.path.join(root, "scripts/pcgen-oracle-pin.env")
    if not os.path.isfile(pin_file):
        die(2, f"missing {pin_file}")
    for line in open(pin_file, encoding="utf-8"):
        m = re.match(r"^([A-Z_]+)=(\S+)", line)
        if m:
            pin[m.group(1)] = m.group(2)
    # `--check --quiet` verifies the checkout against the pin and prints the two export lines.
    # (The script has no `--print-env` flag; see the E0.1 receipt and its retro correction.)
    p = run(["bash", "scripts/fetch-pcgen-oracle.sh", "--check", "--quiet"], root, check=False)
    if p.returncode != 0:
        die(2, f"oracle not on pin: fetch-pcgen-oracle.sh --check exit {p.returncode}: {p.stderr.strip()}")
    env = dict(re.findall(r"^export (PCGEN_[A-Z_]+)=(.*)$", p.stdout, re.M))
    repo_dir, corpus_root = env.get("PCGEN_REPO_DIR", ""), env.get("PCGEN_CORPUS_ROOT", "")
    if not repo_dir or not os.path.isdir(os.path.join(repo_dir, ".git")) and not os.path.isfile(os.path.join(repo_dir, ".git")):
        die(2, "PCGEN_REPO_DIR did not resolve to a git checkout")  # an empty -C would read THIS repo
    head = run(["git", "-C", repo_dir, "rev-parse", "HEAD"], root).stdout.strip()
    if head != pin.get("PCGEN_ORACLE_SHA"):
        die(2, f"oracle HEAD {head} != PCGEN_ORACLE_SHA {pin.get('PCGEN_ORACLE_SHA')}")
    return {"repo_dir": repo_dir, "corpus_root": corpus_root, "head": head, "pin": pin["PCGEN_ORACLE_SHA"]}


def split_lst(line: str) -> list[str]:
    return [t for t in line.rstrip("\n").split("\t") if t.strip()]


def load_ability_categories(corpus_root: str, repo_dir: str) -> tuple[list[dict], dict]:
    """Every ABILITYCATEGORY definition line in the Pathfinder data and game mode."""
    files = sorted(glob.glob(os.path.join(corpus_root, "pathfinder", "**", "*.lst"), recursive=True))
    files += sorted(glob.glob(os.path.join(repo_dir, "system", "gameModes", "Pathfinder", "*.lst")))
    pcc_refs = 0
    for f in glob.glob(os.path.join(corpus_root, "pathfinder", "**", "*.pcc"), recursive=True):
        for line in open(f, encoding="utf-8", errors="replace"):
            if line.startswith("ABILITYCATEGORY:"):
                pcc_refs += 1
    defs = []
    for f in files:
        for n, line in enumerate(open(f, encoding="utf-8", errors="replace"), 1):
            if not line.startswith("ABILITYCATEGORY:"):
                continue
            toks = split_lst(line)
            d = {"name": toks[0].split(":", 1)[1].strip()}
            for t in toks[1:]:
                if ":" in t:
                    k, v = t.split(":", 1)
                    d.setdefault(k.strip(), v.strip())
            rel = os.path.relpath(f, corpus_root) if f.startswith(corpus_root) else "gameMode:" + os.path.relpath(f, repo_dir)
            defs.append({
                "name": d["name"], "file": rel, "line": n,
                "parent": d.get("CATEGORY"), "types": [x for x in d.get("TYPE", "").split(".") if x],
                "abilitylist": [x for x in d.get("ABILITYLIST", "").split("|") if x],
                "visible": d.get("VISIBLE"), "editable": d.get("EDITABLE"),
                "displaylocation": d.get("DISPLAYLOCATION"),
            })
    return defs, {"pcc_reference_lines": pcc_refs, "lst_files_scanned": len(files)}


# ---------------------------------------------------------------- corpus, implementation A (json)
def tok_values(raw, key):
    return [t.get("value") for t in raw or [] if isinstance(t, dict) and t.get("key") == key and isinstance(t.get("value"), str)]


def load_corpus(root: str) -> list[dict]:
    recs = []
    for f in sorted(glob.glob(os.path.join(root, "data/corpus/*/*/**/*.json"), recursive=True)):
        rel = os.path.relpath(f, root)
        parts = rel.split("/")
        book, kind = parts[2], parts[3]
        if kind.startswith("_") or kind not in set(ABILITY_KINDS) | GRANT_KINDS | {"domain"}:
            continue
        try:
            doc = json.load(open(f, encoding="utf-8"))
        except Exception:
            die(1, f"malformed json {rel}")
        data = doc.get("data") or {}
        raw = data.get("raw_tokens")
        cats = tok_values(raw, "CATEGORY")
        cat = cats[0] if cats else ("FEAT" if kind in ("feat", "feat_generic") else None)
        types = set()
        for v in tok_values(raw, "TYPE"):
            types.update(x for x in v.split(".") if x)
        # Records ingested without raw TYPE tokens carry the type in a typed field instead:
        # feat `category` ("Combat", "Psionic"), race_trait / class_feature `type_tokens`.
        if kind == "feat" and isinstance(data.get("category"), str):
            types.update(x for x in data["category"].split(".") if x)
        if isinstance(data.get("type_tokens"), list):
            types.update(x for x in data["type_tokens"] if isinstance(x, str) and x)
        types = {x.lower() for x in types}  # PCGen compares TYPE case-insensitively
        key = data.get("key") if isinstance(data.get("key"), str) else (doc.get("source") or {}).get("record_key")
        recs.append({"path": rel, "book": book, "kind": kind, "key": key or "", "category": cat,
                     "types": types, "raw": raw if isinstance(raw, list) else [],
                     "source_dir": os.path.dirname((doc.get("source") or {}).get("path") or "")})
    return recs


def class_names(recs) -> set[str]:
    out = set()
    for r in recs:
        if r["kind"] == "class":
            k = r["key"].replace("CLASS:", "").replace(" ~ Unchained Class", "")
            out.add(k)
            if r["key"].endswith(" ~ Unchained Class"):
                out.add("Unchained " + k)
    return out


def archetype_names(recs) -> set[str]:
    out = set()
    for r in recs:
        if r["category"] == "Archetype":
            parts = [x.strip() for x in r["key"].split(" ~ ")]
            out.update({parts[0], parts[1] if len(parts) > 1 else parts[0], parts[-1]})
    return out


def granted_by(key: str, category, classes, archetypes) -> str:
    owner = key.split(" ~ ", 1)[0].strip()
    if category == "Archetype":
        return "archetype"
    if owner in classes:  # a class name wins over a same-named archetype (Investigator, Brawler)
        return "class"
    return "archetype" if owner in archetypes else "other"


def is_grant_record(kind, category) -> bool:
    return kind in ("class", "class_feature") or (kind == "ability" and category in ("Class", "Archetype"))


def choose_target(v: str):
    segs = [s for s in v.split("|") if not s.startswith("NUMCHOICES=")]
    return segs


def choice_sites_a(recs, classes, archetypes) -> set[tuple]:
    sites = set()
    for r in recs:
        if not is_grant_record(r["kind"], r["category"]):
            continue
        gb = granted_by(r["key"], r["category"], classes, archetypes)
        for t in r["raw"]:
            if not isinstance(t, dict) or not isinstance(t.get("value"), str):
                continue
            k, v = t.get("key"), t["value"]
            if k == "BONUS" and v.startswith("ABILITYPOOL|"):
                for pool in v.split("|")[1].split(","):
                    sites.add((r["path"], "abilitypool", pool.strip(), gb))
            elif k == "BONUS" and v.startswith("DOMAIN|NUMBER"):
                sites.add((r["path"], "domain_count", "Domain", gb))
            elif k == "CHOOSE":
                segs = choose_target(v)
                if not segs or segs[0] == "NOCHOICE":
                    continue
                if segs[0] == "ABILITYSELECTION" and len(segs) > 1:
                    sites.add((r["path"], "abilityselection", segs[1].strip(), gb))
                else:
                    sites.add((r["path"], "parameter_choose", segs[0].split("=")[0], gb))
            elif k == "ADD":
                sites.add((r["path"], "add_token", v.split("|")[0].split("(")[0], gb))
    return sites


# ---------------------------------------------------------------- implementation B (regex over the raw file text)
STR = r'"((?:[^"\\]|\\.)*)"'
TOK_RE = re.compile(r'"key"\s*:\s*' + STR + r'\s*,\s*"value"\s*:\s*' + STR)
RK_RE = re.compile(r'"record_key"\s*:\s*' + STR)


def choice_sites_b(root: str) -> set[tuple]:
    texts = {}
    for f in glob.glob(os.path.join(root, "data/corpus/*/*/**/*.json"), recursive=True):
        rel = os.path.relpath(f, root)
        kind = rel.split("/")[3]
        if kind in GRANT_KINDS:
            texts[rel] = (kind, open(f, encoding="utf-8").read())

    def unq(s):
        return json.loads('"' + s + '"')

    parsed = {}
    for rel, (kind, txt) in texts.items():
        toks = [(unq(a), unq(b)) for a, b in TOK_RE.findall(txt)]
        rk = RK_RE.search(txt)
        cat = next((v for k, v in toks if k == "CATEGORY"), None)
        parsed[rel] = (kind, unq(rk.group(1)) if rk else "", cat, toks)
    classes = set()
    archetypes = set()
    for rel, (kind, rk, cat, _) in parsed.items():
        if kind == "class":
            base = rk.replace("CLASS:", "").replace(" ~ Unchained Class", "")
            classes.add(base)
            if rk.endswith(" ~ Unchained Class"):
                classes.add("Unchained " + base)
    # archetype names come from every ability-shaped kind, as in A
    for f in glob.glob(os.path.join(root, "data/corpus/*/*/**/*.json"), recursive=True):
        rel = os.path.relpath(f, root)
        if rel.split("/")[3] not in set(ABILITY_KINDS) | GRANT_KINDS:
            continue
        txt = parsed[rel][3] if rel in parsed else None
        if txt is None:
            t = open(f, encoding="utf-8").read()
            toks = [(unq(a), unq(b)) for a, b in TOK_RE.findall(t)]
            rk_m = RK_RE.search(t)
            rk = unq(rk_m.group(1)) if rk_m else ""
        else:
            toks, rk = txt, parsed[rel][1]
        if next((v for k, v in toks if k == "CATEGORY"), None) == "Archetype":
            parts = [x.strip() for x in rk.split(" ~ ")]
            archetypes.update({parts[0], parts[1] if len(parts) > 1 else parts[0], parts[-1]})
    sites = set()
    for rel, (kind, rk, cat, toks) in parsed.items():
        if not (kind in ("class", "class_feature") or (kind == "ability" and cat in ("Class", "Archetype"))):
            continue
        owner = rk.split(" ~ ", 1)[0].strip()
        gb = ("archetype" if cat == "Archetype" else "class" if owner in classes
              else "archetype" if owner in archetypes else "other")
        for k, v in toks:
            m = re.match(r"ABILITYPOOL\|([^|]*)", v) if k == "BONUS" else None
            if m:
                for pool in m.group(1).split(","):
                    sites.add((rel, "abilitypool", pool.strip(), gb))
                continue
            if k == "BONUS" and re.match(r"DOMAIN\|NUMBER", v):
                sites.add((rel, "domain_count", "Domain", gb))
            elif k == "CHOOSE":
                m2 = re.match(r"(?:NUMCHOICES=[^|]*\|)?([^|=]*)(?:=[^|]*)?(?:\|([^|]*))?", v)
                head = m2.group(1)
                if head in ("", "NOCHOICE"):
                    continue
                if head == "ABILITYSELECTION" and m2.group(2) is not None:
                    sites.add((rel, "abilityselection", m2.group(2).strip(), gb))
                else:
                    sites.add((rel, "parameter_choose", head, gb))
            elif k == "ADD":
                sites.add((rel, "add_token", re.split(r"[|(]", v)[0], gb))
    return sites


# ---------------------------------------------------------------- category matching
def matches(rec, d) -> bool:
    if rec["category"] is None or d["parent"] is None or rec["category"].lower() != d["parent"].lower():
        return False
    if not d["types"] and not d["abilitylist"]:
        return True
    return bool(rec["types"] & {x.lower() for x in d["types"]}) or rec["key"] in d["abilitylist"]


def selectable(d) -> bool:
    return (d["visible"] or "").upper() in SELECTABLE_VISIBLE and (d["editable"] or "").upper() == "YES"


# ---------------------------------------------------------------- engine evidence (git grep)
def ggrep_count(root, pattern, paths) -> int:
    p = subprocess.run(["git", "grep", "-c", "-E", pattern, "--"] + paths, cwd=root, capture_output=True, text=True)
    return sum(int(l.rsplit(":", 1)[1]) for l in p.stdout.splitlines() if ":" in l)


RS = ["src/*.rs", "apps/desktop/src-tauri/src/*.rs"]
ENGINE = {
    # sub-tab: (saved_field claim, evidence pattern, budget claim, budget pattern, prereq claim, prereq pattern)
    "Feats": ("ChosenCharacterState.selected_feats", r"pub selected_feats: Vec<String>",
              "feats.budget.<pool>.{granted,spent,remaining} rows (feat_budget.rs)", r"feats\.budget",
              "evaluate_*feat*_prerequisites + add_feat_selection_enforcing_prerequisites_at_root",
              r"fn add_feat_selection_enforcing_prerequisites_at_root"),
    "Racial Abilities": ("ChosenCharacterState.selected_choices with RACE_ALTERNATE_TRAIT_CHOICE_ID (alternates, creation only)",
                         r"RACE_ALTERNATE_TRAIT_CHOICE_ID: &str", None, r"racial[a-z_]*\.budget|race_trait_budget", None,
                         r"fn add_[a-z_]*(racial|race_trait)[a-z_]*enforcing"),
    "Traits": ("ChosenCharacterState.selected_traits", r"pub selected_traits: Vec<String>", None,
               r"traits\.budget|trait_budget", None, r"fn add_[a-z_]*trait[a-z_]*enforcing"),
    "Archetype": ("ChosenCharacterState.selected_choices with ARCHETYPE_CHOICE_ID (engine only, no UI)",
                  r"ARCHETYPE_CHOICE_ID: &str", None, r"archetype[a-z_]*\.budget", None,
                  r"fn add_[a-z_]*archetype[a-z_]*enforcing"),
    "Background": (None, r"selected_background|background_selection", None, r"background[a-z_]*\.budget", None,
                   r"fn add_[a-z_]*background[a-z_]*enforcing"),
    "Class Features": ("ChosenCharacterState.selected_choices, per-class choice ids only (no generic pool field)",
                       r"_CHOICE_ID: &str = \"choice:", None, r"abilities\.budget|pool_budget|class_feature[a-z_]*\.budget",
                       None, r"fn add_[a-z_]*(pool|class_feature|ability)[a-z_]*enforcing"),
    "Misc": (None, r"selected_misc|misc_selection", None, r"misc[a-z_]*\.budget", None, r"fn add_[a-z_]*misc[a-z_]*enforcing"),
}


def engine_evidence(root):
    out = {}
    for tab, (saved, sp, budget, bp, prereq, pp) in ENGINE.items():
        sc, bc, pc = ggrep_count(root, sp, RS), ggrep_count(root, bp, RS), ggrep_count(root, pp, RS)
        out[tab] = {
            "saved_field": saved if sc else None, "saved_field_grep": {"pattern": sp, "matches": sc},
            "budget": (budget if bc else None) if budget else (f"none ({bc} matches)" if bc == 0 else f"UNEXPECTED {bc} matches"),
            "budget_grep": {"pattern": bp, "matches": bc},
            "prereq_add_path": (prereq if pc else None) if prereq else ("none" if pc == 0 else f"UNEXPECTED {pc} matches"),
            "prereq_grep": {"pattern": pp, "matches": pc},
        }
    out["_generic_prereq_evaluator"] = {
        "evaluate_applies (level_up_option_filter.rs over SheetRule.applies)": ggrep_count(root, r"evaluate_applies", ["src/rules_core/level_up_option_filter.rs"]),
        "note": "a generic evaluator over converted PRE* (SheetRule.applies) exists; only the feat add path enforces prerequisites today",
    }
    return out


# ---------------------------------------------------------------- pool size worked examples
def worked_pools(root, recs, oracle):
    out = {}
    for cls, feature, pool, grant_book in [("Rogue", "Rogue ~ Rogue Talents", "Rogue Talent", "core_rulebook"),
                                           ("Barbarian", "Barbarian ~ Rage Powers", "Rage Power", "core_rulebook"),
                                           ("Ninja", "Ninja ~ Ninja Trick", "Ninja Trick", "ultimate_combat")]:
        rec = next((r for r in recs if r["key"] == feature and r["kind"] == "class_feature"), None)
        if rec is None:
            die(2, f"worked class feature missing from corpus: {feature}")
        toks = [f"{t['key']}:{t['value']}" for t in rec["raw"] if t.get("key") in ("DEFINE", "BONUS")
                and ("ABILITYPOOL|" in t["value"] or "LVL" in t["value"])]
        grants_file = os.path.join(root, "data/class_feature_grants", grant_book, cls.lower() + ".json")
        rows = json.load(open(grants_file)) if os.path.isfile(grants_file) else []
        row = next((x for x in rows if x.get("key") == feature), None)
        oracle_level = None
        for f in glob.glob(os.path.join(oracle["corpus_root"], rec["source_dir"], "*.lst")):
            for line in open(f, encoding="utf-8", errors="replace"):
                if f"|{feature}|" in line:
                    m = re.search(r"\|" + re.escape(feature) + r"\|[^\t]*PREVARGTEQ:\w+_CFP_Level,(\d+)", line)
                    if m:
                        oracle_level = int(m.group(1))
        base = next((t["value"] for t in rec["raw"] if t.get("key") == "BONUS" and t["value"].startswith(f"ABILITYPOOL|{pool}|")
                     and "|PRE" not in t["value"]), None)
        m = re.match(r"ABILITYPOOL\|[^|]*\|(\w+)/2$", base or "")
        per_level = {L: L // 2 for L in (1, 2, 3, 4, 6, 10, 20)} if m else None
        out[cls] = {
            "feature_record": rec["path"], "pool": pool, "tokens": toks,
            "pool_formula": base, "pool_at_level (floor(L/2); the -1 rows apply only when an archetype sets the Replace var)": per_level,
            "grant_level_in_data/class_feature_grants": row["level"] if row else None,
            "grant_level_in_oracle_class_mod_line": oracle_level,
        }
    return out


# ---------------------------------------------------------------- the census
def ranked(counter, n=None) -> dict:
    """most_common with a name tie-break, so the JSON is byte-stable across runs (set order is hash-seeded)."""
    return dict(sorted(counter.items(), key=lambda kv: (-kv[1], kv[0]))[:n])


def build(root) -> dict:
    for p in ("data/corpus", "scripts/census_class_feature_pool_population.py", "src/rules_core/character_input.rs"):
        if not os.path.exists(os.path.join(root, p)):
            die(2, f"missing input {p}")
    oracle = resolve_oracle(root)
    defs, pcc = load_ability_categories(oracle["corpus_root"], oracle["repo_dir"])
    recs = load_corpus(root)
    by_path = {r["path"]: r for r in recs}
    scope_dirs = sorted({r["source_dir"] for r in recs if r["source_dir"]})
    book_roots = sorted({"/".join(d.split("/")[:4]) for d in scope_dirs})

    def in_scope(d):
        return d["file"].startswith("gameMode:") or any(d["file"].startswith(b + "/") for b in book_roots)

    for d in defs:
        d["in_scope"] = in_scope(d)
    scoped = [d for d in defs if d["in_scope"]]

    def cat_summary(ds):
        return {
            "definition_lines": len(ds), "distinct_names": len({d["name"] for d in ds}),
            "visible_QUALIFY_or_YES": sum((d["visible"] or "").upper() in SELECTABLE_VISIBLE for d in ds),
            "editable_YES": sum((d["editable"] or "").upper() == "YES" for d in ds),
            "visible_and_editable (player-editable)": sum(selectable(d) for d in ds),
            "visible_and_editable_distinct_names": len({d["name"] for d in ds if selectable(d)}),
            "by_displaylocation": ranked(collections.Counter(d["displaylocation"] or "(none)" for d in ds if selectable(d))),
        }

    pcgen = {
        "oracle_head": oracle["head"], "pin": oracle["pin"],
        "tab_title_source": "pcgen.core.AbilityCategory.getType() returns String.valueOf(getDisplayLocation()) "
                            "(code/src/java/pcgen/core/AbilityCategory.java:808-811); AbilitiesInfoTab.AbilityTabsModel "
                            "builds one sub-tab per distinct type over the character's active categories "
                            "(code/src/java/pcgen/gui2/tabs/AbilitiesInfoTab.java:102-118)",
        "abilitycategory_lines_all_pathfinder_incl_pcc_refs": len(defs) + pcc["pcc_reference_lines"],
        "pcc_reference_lines": pcc["pcc_reference_lines"],
        "all_pathfinder": cat_summary(defs),
        "corpus_books_only": cat_summary(scoped),
        "corpus_book_roots": book_roots,
    }

    # record -> sub-tabs through in-scope, player-editable categories
    sel_defs = [d for d in scoped if selectable(d)]
    by_parent = collections.defaultdict(list)
    for d in sel_defs:
        by_parent[(d["parent"] or "").lower()].append(d)
    rec_tabs = {}
    ability_recs = [r for r in recs if r["kind"] in ABILITY_KINDS]
    for r in ability_recs:
        tabs = {}
        for d in by_parent.get((r["category"] or "").lower(), []):
            if matches(r, d):
                tabs.setdefault(d["displaylocation"] or "(none)", set()).add(d["name"])
        rec_tabs[r["path"]] = tabs
    evidence = engine_evidence(root)
    subtabs = {}
    other_locs = collections.Counter()
    for r in ability_recs:
        for loc in rec_tabs[r["path"]]:
            if loc not in SUBTABS:
                other_locs[loc] += 1
    # categories something in the corpus grants picks into (any record's ABILITYPOOL / ABILITYSELECTION target),
    # plus the base FEAT category: the narrower "a character can be given picks here" span.
    granted_cats = {"FEAT"}
    for r in recs:
        for t in r["raw"]:
            v = t.get("value") if isinstance(t, dict) and isinstance(t.get("value"), str) else ""
            if t.get("key") == "BONUS" and v.startswith("ABILITYPOOL|"):
                granted_cats.update(x.strip() for x in v.split("|")[1].split(","))
            elif t.get("key") == "CHOOSE" and "ABILITYSELECTION|" in v:
                granted_cats.add(v.split("ABILITYSELECTION|", 1)[1].split("|")[0].strip())
    for tab in SUBTABS:
        members = [r for r in ability_recs if tab in rec_tabs[r["path"]]]
        granted_members = [r for r in members if rec_tabs[r["path"]][tab] & granted_cats]
        cats = sorted({d["name"] for d in sel_defs if d["displaylocation"] == tab})
        cats_used = collections.Counter(c for r in members for c in rec_tabs[r["path"]][tab])
        subtabs[tab] = {
            "pcgen_categories_player_editable": len(cats),
            "pcgen_categories_with_corpus_members": len(cats_used),
            "top_categories_by_members": ranked(cats_used, 15),
            "corpus_records": len(members),
            "corpus_records_in_pick_granted_categories": len(granted_members),
            "pick_granted_categories_with_members": len({c for r in granted_members for c in rec_tabs[r["path"]][tab] & granted_cats}),
            "record_predicate": "a record counts when a player-editable (VISIBLE QUALIFY|YES and EDITABLE:YES) category of "
                                "this DISPLAYLOCATION in the corpus books spans it: same parent CATEGORY, and a shared TYPE "
                                "(OR over the '.'-list) or a key in ABILITYLIST, or no filter; one record can count under "
                                "several sub-tabs. 'pick-granted' narrows to categories some corpus record grants picks into "
                                "(BONUS:ABILITYPOOL / CHOOSE:ABILITYSELECTION target) plus FEAT",
            "corpus_records_by_book": dict(sorted(collections.Counter(r["book"] for r in members).items())),
            "corpus_records_by_kind": dict(sorted(collections.Counter(r["kind"] for r in members).items())),
            "data_gap": len(members) == 0,
            **evidence[tab],
        }
    subtabs["Class Features"]["pool_census_reference"] = None  # filled below

    # Class Features: reproduce the pool-population census, two ways
    p = run([sys.executable, "-I", "scripts/census_class_feature_pool_population.py", "--json"], root)
    ref = json.loads(p.stdout)
    groups = collections.Counter(r["key"].split(" ~ ", 1)[0] for r in recs
                                 if r["kind"] == "class_feature" and " ~ " in r["key"])
    subtabs["Class Features"]["pool_census_reference"] = {
        "command": "python3 scripts/census_class_feature_pool_population.py --json",
        "script_groups": ref["distinct_group_qualified_names"], "script_records": ref["total_group_qualified_records"],
        "this_census_groups": len(groups), "this_census_records": sum(groups.values()),
        "predicate": "data.key contains ' ~ ' over data/corpus/*/class_feature/**/*.json; group = text before ' ~ '",
    }

    classes, archetypes = class_names(recs), archetype_names(recs)
    sites_a = choice_sites_a(recs, classes, archetypes)
    sites_b = choice_sites_b(root)
    cat_by_name = collections.defaultdict(list)
    for d in defs:
        cat_by_name[d["name"]].append(d)

    def pool_members(name):
        ds = [d for d in cat_by_name.get(name, []) if d["in_scope"]] or cat_by_name.get(name, [])
        if not ds:
            return None, None, None
        d = ds[0]
        n = sum(1 for r in ability_recs if matches(r, d))
        return d, n, selectable(d)

    pools = {}
    for path, mech, target, gb in sorted(sites_a):
        if mech not in ("abilitypool", "abilityselection"):
            continue
        e = pools.setdefault(target, {"granting_records": 0, "granted_by": collections.Counter(), "mechanisms": set()})
        e["granting_records"] += 1
        e["granted_by"][gb] += 1
        e["mechanisms"].add(mech)
    pool_rows = {}
    for name, e in sorted(pools.items()):
        d, n, sel = pool_members(name)
        pool_rows[name] = {
            "mechanisms": sorted(e["mechanisms"]), "granting_records": e["granting_records"],
            "granted_by": dict(e["granted_by"]),
            "abilitycategory": None if d is None else {"file": d["file"], "line": d["line"], "parent": d["parent"],
                                                        "types": d["types"], "displaylocation": d["displaylocation"],
                                                        "visible": d["visible"], "editable": d["editable"], "in_scope": d["in_scope"]},
            "member_records": n, "player_facing": bool(sel),
        }

    def rollup(sites):
        fine = collections.Counter((m, gb) for _, m, _, gb in sites)
        four = collections.Counter()
        for (m, gb), n in fine.items():
            if gb == "archetype":
                four["archetype_granted"] += n
            elif gb == "class" and m == "abilitypool":
                four["catalog_pool_group"] += n
            elif gb == "class" and m in ("abilityselection", "domain_count"):
                four["level_1_pick"] += n
            else:
                four["other"] += n
        return fine, four

    fine_a, four_a = rollup(sites_a)
    fine_b, four_b = rollup(sites_b)
    grouped_paths = {r["path"] for r in recs if r["kind"] == "class_feature" and " ~ " in r["key"]}
    inside = collections.Counter()
    for path, m, _, gb in sites_a:
        inside[("inside" if path in grouped_paths else "outside") + "_1913_group_population"] += 1
    level1_inside = collections.Counter(
        ("inside" if path in grouped_paths else "outside")
        for path, m, _, gb in sites_a if gb == "class" and m in ("abilityselection", "domain_count"))
    member_inside = collections.Counter()
    for name, row in pool_rows.items():
        d, _, _ = pool_members(name)
        if d is None:
            continue
        for r in ability_recs:
            if matches(r, d):
                member_inside["inside" if r["path"] in grouped_paths else "outside"] += 1
    class_granted = {
        "unit": "a choice site = one (granting record file, mechanism, target) triple; repeated tokens on one record "
                "for the same target (e.g. Rogue Talents' -1 adjustments) collapse to one site",
        "granting_records_predicate": "kind class or class_feature, or kind ability with CATEGORY Class/Archetype",
        "granted_by_predicate": "owner = key text before ' ~ '; archetype if CATEGORY Archetype or owner is an "
                                "archetype name; class if owner is a class record name; else other",
        "mechanisms": {
            "abilitypool": "BONUS:ABILITYPOOL|<category>|<formula> (talents, tricks, rage powers, bloodline, mystery, order...)",
            "abilityselection": "CHOOSE:ABILITYSELECTION|<category>|... (e.g. Sorcerer ~ Standard Bloodline Selection)",
            "domain_count": "BONUS:DOMAIN|NUMBER|n (cleric/paladin/shaman/druid domain picks; selectable = domain records)",
            "parameter_choose": "any other CHOOSE (WEAPONPROFICIENCY, SKILL, SPELLS, CLASS, STRING...)",
            "add_token": "ADD:<x>| (SPELLCASTER, EQUIP, ABILITY, LANGUAGE, VFEAT)",
        },
        "by_mechanism_and_grantor_A": {f"{m}/{gb}": n for (m, gb), n in sorted(fine_a.items())},
        "by_mechanism_and_grantor_B": {f"{m}/{gb}": n for (m, gb), n in sorted(fine_b.items())},
        "by_mechanism": dict(sorted(four_a.items())),
        "by_mechanism_B": dict(sorted(four_b.items())),
        "total": len(sites_a), "total_B": len(sites_b),
        "implementations_agree": sites_a == sites_b,
        "open_check_22c": {
            "question": "are choices not modeled as a catalog pool (bloodline/domain/school/mystery/order at level 1) "
                        "inside the 1,913-group / 16,348-record figure?",
            "choice_sites_by_granting_record": dict(inside),
            "level_1_pick_sites_by_granting_record": dict(level1_inside),
            "pool_member_records_by_membership": dict(member_inside),
            "domain_records_outside (kind domain)": sum(1 for r in recs if r["kind"] == "domain"),
        },
    }

    # For a zero-member pool, does the oracle give it members the corpus does not carry? PCGen often types
    # members through `<record>.MOD  TYPE:<x>` lines, which the corpus does not apply.
    gap_types = {}
    for name, row in pool_rows.items():
        if row["member_records"] == 0 and row["abilitycategory"]:
            for t in row["abilitycategory"]["types"]:
                gap_types.setdefault(t.lower(), set()).add(name)
    oracle_hits = collections.defaultdict(lambda: {"lines": 0, "mod_lines": 0})
    if gap_types:
        for f in glob.glob(os.path.join(oracle["corpus_root"], "pathfinder", "**", "*.lst"), recursive=True):
            relf = os.path.relpath(f, oracle["corpus_root"])
            if not any(relf.startswith(b + "/") for b in book_roots):
                continue
            for line in open(f, encoding="utf-8", errors="replace"):
                if line.startswith("ABILITYCATEGORY:") or "TYPE:" not in line:
                    continue
                for tok in split_lst(line):
                    if tok.startswith("TYPE:"):
                        for t in tok[5:].split("."):
                            for name in gap_types.get(t.lower(), ()):
                                oracle_hits[name]["lines"] += 1
                                oracle_hits[name]["mod_lines"] += ".MOD" in split_lst(line)[0]
    for name, row in pool_rows.items():
        if row["member_records"] == 0 and row["abilitycategory"]:
            row["oracle_lines_typed_into_category"] = dict(oracle_hits.get(name, {"lines": 0, "mod_lines": 0}))

    data_gaps = []
    for tab in SUBTABS:
        if subtabs[tab]["data_gap"]:
            data_gaps.append({"subtab": tab, "pool": None, "sentence": f"The corpus holds no {tab} records to select."})
    for name, row in pool_rows.items():
        if any(g == "class" or g == "archetype" for g in row["granted_by"]) and row["player_facing"] and row["member_records"] == 0:
            oh = row.get("oracle_lines_typed_into_category", {"lines": 0, "mod_lines": 0})
            data_gaps.append({"subtab": "Class Features", "pool": name,
                              "sentence": f"The corpus holds no {name} records to select.",
                              "cause": ("ingest: the oracle types members into it on {lines} line(s) ({mod_lines} "
                                        ".MOD) that the corpus does not apply").format(**oh) if oh["lines"]
                                       else "the books in the corpus define no members"})
    no_category = sorted(n for n, r in pool_rows.items() if r["abilitycategory"] is None)

    return {
        "card": "SD-43 E0.1",
        "command": "python3 -I docs/release/SD-43-market-inventory-and-abilities/artifacts/epic_0/abilities_census.py",
        "corpus_records_scanned": {k: sum(1 for r in recs if r["kind"] == k) for k in sorted({r["kind"] for r in recs})},
        "pcgen": pcgen,
        "subtabs": subtabs,
        "other_displaylocations_in_scope (records)": ranked(other_locs),
        "generic_prereq_evaluator": evidence["_generic_prereq_evaluator"],
        "class_feature_pools": {
            "count": len(pool_rows),
            "player_facing": sum(r["player_facing"] for r in pool_rows.values()),
            "not_player_facing (VISIBLE:NO or EDITABLE:NO)": sum(1 for r in pool_rows.values() if r["abilitycategory"] and not r["player_facing"]),
            "no_abilitycategory_in_pathfinder_data": no_category,
            "pools": pool_rows,
        },
        "class_granted_choices": class_granted,
        "pool_size_per_level": worked_pools(root, recs, oracle),
        "data_gap_list": data_gaps,
    }


# ---------------------------------------------------------------- outputs
def to_md(c) -> str:
    L = ["# SD-43 E0.1 — Abilities census", "",
         f"Generated by `{c['command']}` (oracle HEAD `{c['pcgen']['oracle_head']}` = pin). Every number below is "
         "re-derived by that command; `--check` recomputes and compares.", "",
         "## Seven sub-tabs", "",
         "| Sub-tab | PCGen categories (player-editable) | with corpus members | corpus records spanned | in pick-granted categories | saved today | budget | prereq add path | data gap |",
         "|---|---|---|---|---|---|---|---|---|"]
    for tab in SUBTABS:
        s = c["subtabs"][tab]
        L.append(f"| {tab} | {s['pcgen_categories_player_editable']} | {s['pcgen_categories_with_corpus_members']} | "
                 f"{s['corpus_records']} | {s['corpus_records_in_pick_granted_categories']} ({s['pick_granted_categories_with_members']} categories) | {s['saved_field'] or 'none'} | {s['budget']} | {s['prereq_add_path'] or 'none'} | "
                 f"{'yes' if s['data_gap'] else 'no'} |")
    L += ["", "Predicate: " + c["subtabs"]["Feats"]["record_predicate"] + ". PCGen TYPE filters are broad (core "
          "`Racial Trait` spans every `SpecialQuality` ability), so 'spanned' is the category reach, not what one "
          "character can pick. Records by book per sub-tab and the top categories are in the JSON (`subtabs.<tab>`).", "",
          "Other DISPLAYLOCATIONs in the corpus books (outside the operator's seven): "
          + ", ".join(f"{k} {v}" for k, v in c["other_displaylocations_in_scope (records)"].items()), "",
          "## PCGen mapping (Q-22.2)", "", c["pcgen"]["tab_title_source"] + ".", ""]
    for scope in ("all_pathfinder", "corpus_books_only"):
        s = c["pcgen"][scope]
        L.append(f"- **{scope}**: {s['definition_lines']} ABILITYCATEGORY definition lines, {s['distinct_names']} names; "
                 f"VISIBLE QUALIFY|YES {s['visible_QUALIFY_or_YES']}; EDITABLE:YES {s['editable_YES']}; both "
                 f"{s['visible_and_editable (player-editable)']} lines / {s['visible_and_editable_distinct_names']} names. "
                 f"By DISPLAYLOCATION: " + ", ".join(f"{k} {v}" for k, v in s["by_displaylocation"].items()))
    L.append(f"- `grep '^ABILITYCATEGORY:'` over data/pathfinder + game mode, .pcc references included: "
             f"{c['pcgen']['abilitycategory_lines_all_pathfinder_incl_pcc_refs']} lines ({c['pcgen']['pcc_reference_lines']} are .pcc file references).")
    r = c["subtabs"]["Class Features"]["pool_census_reference"]
    g = c["class_granted_choices"]
    L += ["", "## Class Features", "",
          f"Pool-population reference reproduced: script {r['script_groups']} groups / {r['script_records']} records; "
          f"this census {r['this_census_groups']} / {r['this_census_records']}.", "",
          f"Pools (ABILITYPOOL / ABILITYSELECTION targets on class-granted records): {c['class_feature_pools']['count']}; "
          f"player-facing {c['class_feature_pools']['player_facing']}; not player-facing "
          f"{c['class_feature_pools']['not_player_facing (VISIBLE:NO or EDITABLE:NO)']}; with no ABILITYCATEGORY in the "
          f"Pathfinder data {len(c['class_feature_pools']['no_abilitycategory_in_pathfinder_data'])}.", "",
          "### Class-granted choices by mechanism (intake §22c)", "", g["unit"] + ".", "",
          "| Mechanism | choice sites (A) | (B) |", "|---|---|---|"]
    for k in sorted(set(g["by_mechanism"]) | set(g["by_mechanism_B"])):
        L.append(f"| {k} | {g['by_mechanism'].get(k, 0)} | {g['by_mechanism_B'].get(k, 0)} |")
    L += [f"| **total** | **{g['total']}** | **{g['total_B']}** |", "",
          f"Implementations agree (site sets equal): {g['implementations_agree']}.", "",
          "| Mechanism / grantor | sites |", "|---|---|"]
    L += [f"| {k} | {v} |" for k, v in g["by_mechanism_and_grantor_A"].items()]
    L += ["", "Grantor `other` = the owner (key text before ' ~ ') is neither a corpus class nor an archetype: a pool "
          "member that grants further picks (e.g. `Rogue Talent ~ ...`), a domain, a prestige class absent from the "
          "class corpus (Golden Legionnaire, Twilight Talon). It is a grantor bucket, not a reachability verdict."]
    oc = g["open_check_22c"]
    L += ["", "**Open check (22c):** " + oc["question"], "",
          f"- all choice sites by granting record: {oc['choice_sites_by_granting_record']}",
          f"- level-1 pick sites (class-granted abilityselection + domain_count): {oc['level_1_pick_sites_by_granting_record']}",
          f"- pool member records: {oc['pool_member_records_by_membership']}",
          f"- domain records (kind `domain`, never inside the figure): {oc['domain_records_outside (kind domain)']}", "",
          "## Pool size per level (worked classes)", ""]
    for cls, w in c["pool_size_per_level"].items():
        L += [f"### {cls} — {w['pool']}", "", f"- record `{w['feature_record']}`",
              f"- formula `{w['pool_formula']}`; grant level in `data/class_feature_grants`: "
              f"{w['grant_level_in_data/class_feature_grants']}; in the oracle's class .MOD line: {w['grant_level_in_oracle_class_mod_line']}",
              f"- pool at level: {w['pool_at_level (floor(L/2); the -1 rows apply only when an archetype sets the Replace var)']}",
              "- tokens:", ""] + [f"  - `{t}`" for t in w["tokens"]] + [""]
    L += ["## Data-gap list", "", f"{len(c['data_gap_list'])} entries.", ""]
    L += [f"- {d['subtab']}{' / ' + d['pool'] if d['pool'] else ''}: {d['sentence']}"
          + (f" Cause: {d['cause']}." if d.get("cause") else "") for d in c["data_gap_list"]]
    n_ingest = sum(1 for d in c["data_gap_list"] if str(d.get("cause", "")).startswith("ingest"))
    L += ["", f"{n_ingest} of {len(c['data_gap_list'])} entries are ingest gaps (the oracle has members the corpus "
          "does not carry), not book gaps: under decisions.md §18 they are engine/ingest work, not on-screen notices, "
          "unless the lead rules otherwise."]
    L.append("")
    return "\n".join(L)


def checks(c) -> list[str]:
    fails = []
    for tab in SUBTABS:
        if c["subtabs"].get(tab) is None:
            fails.append(f"sub-tab {tab} null")
    r = c["subtabs"]["Class Features"]["pool_census_reference"]
    if (r["script_groups"], r["script_records"]) != (r["this_census_groups"], r["this_census_records"]):
        fails.append(f"pool census mismatch {r}")
    if (r["script_groups"], r["script_records"]) != (1913, 16348):
        fails.append(f"pool census moved from 1,913 / 16,348: {r['script_groups']} / {r['script_records']}")
    g = c["class_granted_choices"]
    if not g["implementations_agree"] or g["by_mechanism"] != g["by_mechanism_B"]:
        fails.append("class_granted_choices: implementations A and B disagree")
    if sum(g["by_mechanism"].values()) != g["total"]:
        fails.append("class_granted_choices: mechanisms do not sum to the total")
    for tab, s in c["subtabs"].items():
        for k in ("budget", "prereq_add_path"):
            if isinstance(s.get(k), str) and s[k].startswith("UNEXPECTED"):
                fails.append(f"{tab}.{k}: {s[k]}")
    if c["subtabs"]["Feats"]["saved_field"] is None or c["subtabs"]["Traits"]["saved_field"] is None:
        fails.append("saved-field evidence for Feats/Traits not found")
    return fails


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--check", action="store_true")
    a = ap.parse_args()
    root = repo_root()
    c = build(root)
    text = json.dumps(c, indent=1, sort_keys=False, default=sorted) + "\n"
    fails = checks(c)
    if a.check:
        if not os.path.isfile(OUT_JSON):
            die(2, f"missing {OUT_JSON}; run without --check first")
        if open(OUT_JSON, encoding="utf-8").read() != text:
            fails.append("abilities-census.json is stale against the recomputation")
        if not os.path.isfile(OUT_MD) or open(OUT_MD, encoding="utf-8").read() != to_md(c):
            fails.append("abilities-census.md is stale against the recomputation")
    else:
        open(OUT_JSON, "w", encoding="utf-8").write(text)
        open(OUT_MD, "w", encoding="utf-8").write(to_md(c))
    for f in fails:
        print("FAIL:", f, file=sys.stderr)
    print("abilities_census:", "OK" if not fails else f"{len(fails)} check(s) failed")
    return 1 if fails else 0


if __name__ == "__main__":
    sys.exit(main())
