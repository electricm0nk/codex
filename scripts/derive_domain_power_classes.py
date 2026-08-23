#!/usr/bin/env python3
"""Derives, from the PCGen oracle itself, which class(es) a "Domain Power ~ <X>"
class_feature record is granted through.

SD-32 card 11, decision `decisions.md §23a`: `Domain Power`'s 172 records carry no
per-record class name in their own `TYPE:`/`PRE*:` tokens (confirmed corpus-wide by
`artifacts/gate-3-closure-invariant/epic-2-t2a-residual-alias-tier_cycle-1_cycle_receipt.md`).
The real link lives one hop upstream: every "Domain Power ~ <X>" ability is granted
*to a character* by a class-namespaced chooser record shaped
`"<Prefix> Domain ~ <domain name>"` (CATEGORY:Internal), via an
`ABILITY:...|AUTOMATIC|Domain Power ~ <X>|...` token on that chooser record. The
prefix names which class's domain-access mechanism the grant runs through:

  - `Core Domain ~ <domain>`       -- the base PCGen "DOMAIN" facet. Verified
    (this script's own `--show-owners` mode, and the class .lst files directly)
    to be wired to TWO classes: Cleric (`cr_classes.lst` CLASS:Cleric
    `BONUS:DOMAIN|NUMBER|ClericDomainCount`, `BONUS:VAR|ClericDomainCount|2`) and
    Paladin via the Sacred Servant archetype (`apg_abilities_class.lst` KEY:"Sacred
    Servant ~ Spells", `BONUS:VAR|PaladinDomainCount|1|TYPE=Base`) -- base Paladin's
    own `PaladinDomainCount` DEFINEs to 0 and is never raised outside that archetype
    ability, so "Paladin" here specifically means the Sacred Servant build.
  - `Druid Domain ~ <domain>`      -- Druid's own domain-swap mechanism
    (`cr_abilities_class.lst` line ~779, DEFINE/BONUS wired to `DruidLVL`).
  - `Inquisitor Domain ~ <domain>` -- Inquisitor's own Inquisition-adjacent domain
    grant (`apg_abilities_class.lst`, wired to `InquisitorLVL`-shaped variables).

A single "Domain Power ~ <X>" record can be referenced by more than one prefix (a
domain power available to both Cleric and Paladin-Sacred-Servant, or shared further
with Druid/Inquisitor for domains those classes can also select) -- this script
reports the full set, not a single winner, per `decisions.md §1a`/§23a: forcing a
multi-owner label to one class would be a relabelled shape, not a closed one.

Usage:
    python3 scripts/derive_domain_power_classes.py --corpus-root "$PCGEN_CORPUS_ROOT" [--json out.json]

Prints, and optionally writes as JSON, a mapping:
    { "<domain power name>": { "owners": ["Cleric", "Paladin", ...], "no_grant_found": bool } }
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

PREFIX_TO_CLASSES = {
    "Core": ["Cleric", "Paladin"],
    "Druid": ["Druid"],
    "Inquisitor": ["Inquisitor"],
}
# The base-facet fallback for a bare (non-class-prefixed) domain/subdomain
# grant record -- same owners as an explicit "Core Domain ~" grant, since
# both run through the identical `BONUS:DOMAIN|NUMBER` facet (see module
# doc comment).
BARE_GRANT_CLASSES = ["Cleric", "Paladin"]

CHOOSER_KEY_RE = re.compile(r"^(Core|Druid|Inquisitor) Domain ~ ")
ABILITY_GRANT_RE = re.compile(r"ABILITY:[^\t]*?\|AUTOMATIC\|Domain Power ~ ([^|\t]+)")
# A record's *effective* key is its explicit `KEY:` token when present
# (PCGen lets the first field be a display name distinct from the KEY --
# e.g. `Chaos<tab>...<tab>KEY:Inquisitor Domain ~ Chaos<tab>...`), else the
# first tab-delimited field itself.
EXPLICIT_KEY_RE = re.compile(r"(?:^|\t)KEY:([^\t]+)")


def scan(corpus_root: Path) -> dict[str, dict]:
    """Walks every .lst file once. For each line, resolves an owning-class
    set for any `"Domain Power ~ <X>"` target it grants:

      - first field matches `"<Prefix> Domain ~ ..."` (Core/Druid/Inquisitor)
        -> that prefix's classes;
      - otherwise, if the line still grants a "Domain Power ~ <X>" via an
        AUTOMATIC ABILITY: token and the first field is not itself a `.MOD`
        line targeting a feat category (`CATEGORY=FEAT|...` -- an unrelated
        cross-reference, not a domain grant point) -> the bare-grant
        fallback classes.

    Returns {domain_power_key_suffix: {"owners": sorted[str], "granted_by": sorted[str]}}.
    """
    result: dict[str, set[str]] = {}
    granted_by: dict[str, set[str]] = {}
    for lst_path in corpus_root.rglob("*.lst"):
        try:
            text = lst_path.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        for line in text.splitlines():
            if not line or line.startswith("#"):
                continue
            targets = ABILITY_GRANT_RE.findall(line)
            if not targets:
                continue
            first_field = line.split("\t", 1)[0].strip()
            key_m = EXPLICIT_KEY_RE.search(line)
            effective_key = key_m.group(1).strip() if key_m else first_field
            m = CHOOSER_KEY_RE.match(effective_key)
            if m:
                classes = PREFIX_TO_CLASSES[m.group(1)]
            elif "CATEGORY=" not in effective_key:
                classes = BARE_GRANT_CLASSES
            else:
                continue
            for target in targets:
                target = target.strip()
                result.setdefault(target, set()).update(classes)
                granted_by.setdefault(target, set()).add(effective_key)
    return {
        name: {
            "owners": sorted(owners),
            "granted_by": sorted(granted_by[name]),
        }
        for name, owners in result.items()
    }


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--corpus-root", required=True, type=Path)
    ap.add_argument("--json", type=Path, default=None)
    ap.add_argument("--check-names", type=Path, default=None,
                     help="newline-delimited file of Domain Power record names to "
                          "report resolved/unresolved counts for")
    args = ap.parse_args()

    mapping = scan(args.corpus_root)

    if args.json:
        args.json.write_text(json.dumps(mapping, indent=2, sort_keys=True) + "\n")

    if args.check_names:
        raw_names = [n.strip() for n in args.check_names.read_text().splitlines() if n.strip()]
        names = [n[len("Domain Power ~ "):] if n.startswith("Domain Power ~ ") else n for n in raw_names]
        resolved_single = 0
        resolved_multi = 0
        unresolved = []
        for n in names:
            entry = mapping.get(n)
            if not entry or not entry["owners"]:
                unresolved.append(n)
            elif len(entry["owners"]) == 1:
                resolved_single += 1
            else:
                resolved_multi += 1
        print(f"total {len(names)} single-owner {resolved_single} multi-owner {resolved_multi} "
              f"unresolved {len(unresolved)}")
        if unresolved:
            print("UNRESOLVED:")
            for n in unresolved:
                print(f"  {n}")
    else:
        print(json.dumps(mapping, indent=2, sort_keys=True))

    return 0


if __name__ == "__main__":
    sys.exit(main())
