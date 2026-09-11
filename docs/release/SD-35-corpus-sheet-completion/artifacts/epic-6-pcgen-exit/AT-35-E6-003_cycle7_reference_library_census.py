#!/usr/bin/env python3
"""The reference-library census behind `AT-35-E6-003_cycle7_reference-library-blocker.md`.

SD-35 `AT-35-E6-003` cycle 7. Answers one question over the whole live corpus, never a fixture
(`decisions.md §4`): for every record `reference_library_catalog.rs` serves, what does the
converted package in `data/sheet_rules/` hold for it?

It reproduces `apps/desktop/src-tauri/src/converted_prose.rs`'s four-step join exactly --
book-exact id, then the source row both sides record, then package-wide by name
(`SheetRulePackage::find`'s order: `core_rulebook` first, else the lexicographically first book),
then one trailing printed qualifier dropped -- and `reference_library_catalog.rs`'s own three
content tiers, so the "before" and "after" columns are the two real code paths and not an
approximation of them.

Run from the repo root:

    python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003_cycle7_reference_library_census.py

Reads only `data/corpus/` and `data/sheet_rules/`; writes nothing.
"""

import collections
import json
import os
import sys

# `reference_library_catalog.rs::REFERENCE_LIBRARY_KIND_DIRS`, verbatim.
KIND_DIRS = [
    "ability",
    "class_generic",
    "deity",
    "domain",
    "feat_generic",
    "language",
    "monster_generic",
    "power",
    "race_generic",
    "skill",
    "template",
    "trait_generic",
]

# `reference_library_catalog.rs::ADMIN_TOKEN_KEYS`, verbatim.
ADMIN_TOKEN_KEYS = {
    "SOURCEPAGE",
    "SOURCEWEB",
    "SOURCELONG",
    "SOURCESHORT",
    "NAMEISPI",
    "KEY",
}

# `sheet_rule_catalog::DESCRIPTION_FAMILIES`, verbatim.
DESCRIPTION_FAMILIES = {"Desc", "Benefit", "Special"}

PLACEHOLDER_PREFIX = "Codex-Named Unit ("


def slug(name):
    """`sheet_rule::slug`, verbatim."""
    out = []
    pending = False
    for ch in name:
        if ch.isascii() and ch.isalnum():
            if pending and out:
                out.append("_")
            pending = False
            out.append(ch.lower())
        else:
            pending = True
    return "".join(out)


def base_name(key):
    """`converted_prose::base_name`, verbatim."""
    trimmed = key.rstrip()
    if not trimmed.endswith(")"):
        return None
    open_at = trimmed.rfind(" (")
    if open_at < 0:
        return None
    return trimmed[:open_at].rstrip() or None


def load_package(root):
    """`(rules, closure_index, by_name)` for `data/sheet_rules/`.

    `rules[id]` is `(states_description, states_stat_block, states_a_fact)` -- the three things a
    live reader could serve. `states_a_fact` is the disjunction the facts renderer would read:
    a value other than `Text`, a bonus target, a grant, a choice, a gate other than `Always`, or
    a tag.
    """
    rules = {}
    closure = collections.defaultdict(dict)
    ambiguous = collections.defaultdict(set)
    by_name = collections.defaultdict(list)
    sheet_rules = os.path.join(root, "data/sheet_rules")
    for book in sorted(os.listdir(sheet_rules)):
        book_dir = os.path.join(sheet_rules, book)
        if book.startswith("_") or not os.path.isdir(book_dir):
            continue
        for kind in sorted(os.listdir(book_dir)):
            kind_dir = os.path.join(book_dir, kind)
            if not os.path.isdir(kind_dir):
                continue
            for name in sorted(os.listdir(kind_dir)):
                with open(os.path.join(kind_dir, name)) as handle:
                    for rule in json.load(handle):
                        described = stated = False
                        for segment in rule.get("prose") or []:
                            family = segment["family"]
                            family = family if isinstance(family, str) else next(iter(family))
                            if family in DESCRIPTION_FAMILIES:
                                described = True
                            else:
                                stated = True
                        facts = (
                            rule.get("value") != "Text"
                            or bool(rule.get("target"))
                            or bool(rule.get("grants"))
                            or bool(rule.get("offers"))
                            or rule.get("applies") != "Always"
                            or bool(rule.get("tags"))
                        )
                        rule_id = rule["id"]
                        was = rules.get(rule_id, (False, False, False))
                        rules[rule_id] = (
                            was[0] or described,
                            was[1] or stated,
                            was[2] or facts,
                        )
                        prov = rule.get("provenance") or {}
                        book_kind = (prov.get("book", book), prov.get("kind", kind))
                        for row in prov.get("closure_rows") or []:
                            if ":" not in row:
                                continue
                            path, line = row.rsplit(":", 1)
                            if not line.isdigit():
                                continue
                            stem = path.rsplit("/", 1)[-1]
                            if not stem:
                                continue
                            suffix = stem.replace(".", "_") + "_" + line
                            slot = closure[book_kind]
                            if slot.get(suffix) == rule_id:
                                pass
                            elif suffix in slot:
                                ambiguous[book_kind].add(suffix)
                            else:
                                slot[suffix] = rule_id
                        by_name[(kind, slug(rule.get("label", "")))].append(rule_id)
                        by_name[(kind, slug(rule_id.split(":", 2)[2]))].append(rule_id)
    # A suffix two rules share resolves to neither, by construction.
    for book_kind, suffixes in ambiguous.items():
        for suffix in suffixes:
            closure[book_kind].pop(suffix, None)
    return rules, closure, by_name


def find_by_name(by_name, kind, name_slug):
    """`SheetRulePackage::find`'s order."""
    ids = by_name.get((kind, name_slug))
    if not ids:
        return None
    ids = sorted(set(ids))
    for rule_id in ids:
        if rule_id.startswith("core_rulebook:"):
            return rule_id
    return ids[0]


def resolve(rules, closure, by_name, book_dir, kind, key):
    """`converted_prose::description_for`'s four steps. Returns `(step, states)`."""
    book = "bestiary" if book_dir == "beastiary" else book_dir
    rule_id = f"{book}:{kind}:{slug(key)}"
    if rule_id in rules:
        return "book_exact", rules[rule_id]
    if key.startswith(PLACEHOLDER_PREFIX) and key.endswith(")"):
        token = key[len(PLACEHOLDER_PREFIX) : -1]
        for suffix, found in closure.get((book, kind), {}).items():
            if token.endswith(suffix):
                return "source_row", rules[found]
    found = find_by_name(by_name, kind, slug(key))
    if found:
        return "by_name", rules[found]
    base = base_name(key)
    if base:
        rule_id = f"{book}:{kind}:{slug(base)}"
        if rule_id in rules:
            return "base_exact", rules[rule_id]
        found = find_by_name(by_name, kind, slug(base))
        if found:
            return "base_by_name", rules[found]
    return None, None


def is_real(value):
    """`reference_library_catalog::is_real_description_value`, verbatim."""
    trimmed = (value or "").strip()
    return bool(trimmed) and trimmed.lower() not in (".clear", ".clearall", "[redacted pi]")


def served_today(data):
    """The three tiers `reference_library_catalog::resolve_description` walks."""
    description = data.get("description")
    if isinstance(description, str) and is_real(description):
        return 1
    for token in data.get("raw_tokens") or []:
        if token.get("key") == "DESC" and is_real(token.get("value")):
            return 2
    for token in data.get("raw_tokens") or []:
        if token.get("key") not in ADMIN_TOKEN_KEYS and (token.get("value") or "").strip():
            return 3
    return 0


def main():
    root = os.path.abspath(os.path.join(os.path.dirname(__file__), "../../../../.."))
    rules, closure, by_name = load_package(root)
    totals = collections.Counter()
    lost_by_step = collections.Counter()
    lost_by_kind = collections.Counter()
    corpus = os.path.join(root, "data/corpus")
    for book in sorted(os.listdir(corpus)):
        book_dir = os.path.join(corpus, book)
        if not os.path.isdir(book_dir):
            continue
        for kind_dir in KIND_DIRS:
            path = os.path.join(book_dir, kind_dir)
            if not os.path.isdir(path):
                continue
            kind = kind_dir[: -len("_generic")] if kind_dir.endswith("_generic") else kind_dir
            for walk_root, _, names in os.walk(path):
                for name in sorted(names):
                    if not name.endswith(".json"):
                        continue
                    with open(os.path.join(walk_root, name)) as handle:
                        data = json.load(handle)["data"]
                    key = data.get("key") or data.get("name") or ""
                    totals["records"] += 1
                    tier = served_today(data)
                    if tier:
                        totals["served_today"] += 1
                    step, states = resolve(rules, closure, by_name, book, kind, key)
                    if states is None:
                        bucket = "none"
                    elif states[0]:
                        bucket = "description"
                    elif states[1]:
                        bucket = "stat_block"
                    elif states[2]:
                        bucket = "facts"
                    else:
                        bucket = "none"
                    totals[bucket] += 1
                    if tier and bucket == "none":
                        lost_by_step[step or "no_rule"] += 1
                        lost_by_kind[kind_dir] += 1
    served_after = totals["description"] + totals["stat_block"] + totals["facts"]
    print(f"records={totals['records']} served_today={totals['served_today']}")
    print(
        f"converted: description={totals['description']} stat_block={totals['stat_block']} "
        f"facts={totals['facts']} none={totals['none']} served_after={served_after}"
    )
    print(f"lost={sum(lost_by_step.values())} by_join_step={dict(lost_by_step)}")
    print(f"lost by corpus kind dir={dict(lost_by_kind)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
