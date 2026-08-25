#!/usr/bin/env python3
"""SD-32 card 11, T2a-residual census (measurement only, decisions.md §13).

Re-derives, at time of use, the population of `class_feature` corpus records
whose `data.class` is a category label (an option-pool / archetype-tag name)
rather than either:
  (a) one of the 34 engine-dispatched class names, or
  (b) a real corpus-declared class name (kind == "class" in
      docs/work-inventory.json) that is simply not yet modelled by the
      engine -- that population is T12, not T2a-residual.

This is exactly the population the T2a+T12 combined cycle
(artifacts/gate-3-closure-invariant/epic-2-t2a-t12_cycle-1_cycle_receipt.md)
left open as "~2,775" after its own fix landed (commit 985e24c1e). This
script re-derives it fresh against the corpus as this cycle found it -- it
does not trust that receipt's number.

Usage:
    python3 scripts/sd32-t2a-residual-census.py [--groups] [--consumer-check]

Output (default): total counts (total records, non-null-class, dispatched,
corpus-declared-undispatched (T12), residual-category-label (T2a-residual)).

--groups: also print every distinct residual category label with its count
and the set of books it appears in (one line per label, sorted by count
descending).
"""
import argparse
import glob
import json
import os
import sys
from collections import defaultdict

DISPATCHED = [
    'Barbarian', 'Bard', 'Cleric', 'Druid', 'Fighter', 'Monk', 'Paladin', 'Ranger', 'Rogue',
    'Sorcerer', 'Wizard', 'Arcanist', 'Bloodrager', 'Brawler', 'Hunter', 'Investigator', 'Shaman', 'Skald',
    'Slayer', 'Swashbuckler', 'Warpriest', 'Alchemist', 'Cavalier', 'Inquisitor', 'Oracle', 'Summoner',
    'Witch', 'Gunslinger', 'Ninja', 'Samurai', 'Unchained Barbarian', 'Unchained Monk', 'Unchained Rogue',
    'Unchained Summoner',
]
DL = [d.lower() for d in DISPATCHED]


def is_dispatched(v):
    v = v.strip().lower()
    return any(v == d or v.startswith(d + ' ') or v.endswith(' ' + d) for d in DL)


def load_corpus_class_names(inventory_path):
    """Every kind=='class' unit's name, lowercased -> natural-case spelling.
    Same population src/rules_core/cache_gen/class_feature.rs's
    corpus_class_names_from_inventory_json() reads (and the same fact
    v06_work_inventory.rs's corpus_class_names is built from)."""
    doc = json.load(open(inventory_path))
    out = {}
    for u in doc.get('units', []):
        if u.get('kind') == 'class':
            name = u.get('name')
            if name:
                out[name.strip().lower()] = name
    return out


def is_corpus_declared_class(v, corpus_classes):
    return v.strip().lower() in corpus_classes


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--groups', action='store_true', help='print per-label residual breakdown')
    ap.add_argument('--consumer-check', action='store_true',
                     help='cross-check which residual labels are also read by a consumer for a different purpose')
    args = ap.parse_args()

    corpus_classes = load_corpus_class_names('docs/work-inventory.json')

    total = nn = disp = corpus_undispatched = residual = 0
    residual_by_label = defaultdict(lambda: {'count': 0, 'books': set()})

    for p in sorted(glob.glob('data/corpus/*/class_feature/**/*.json', recursive=True)):
        base = os.path.basename(p)
        if base.startswith('manifest'):
            continue
        try:
            d = json.load(open(p))
        except Exception:
            continue
        data = d.get('data')
        if not isinstance(data, dict):
            continue
        total += 1
        c = data.get('class')
        if c is None:
            continue
        nn += 1
        if is_dispatched(c):
            disp += 1
            continue
        if is_corpus_declared_class(c, corpus_classes):
            corpus_undispatched += 1
            continue
        # Residual: neither a dispatched class nor any corpus-declared class.
        residual += 1
        book = p.split('/')[2]
        residual_by_label[c]['count'] += 1
        residual_by_label[c]['books'].add(book)

    print(f"total {total}  non-null-class {nn}  dispatched {disp}  "
          f"corpus-declared-undispatched(T12-overlap-shape) {corpus_undispatched}  "
          f"residual-category-label(T2a-residual) {residual}")

    if args.groups:
        print()
        print(f"{'count':>6}  {'#books':>6}  label  ->  books")
        for label, info in sorted(residual_by_label.items(), key=lambda kv: -kv[1]['count']):
            books = ','.join(sorted(info['books']))
            print(f"{info['count']:>6}  {len(info['books']):>6}  {label!r}  ->  {books}")
        print()
        print(f"distinct residual labels: {len(residual_by_label)}")


if __name__ == '__main__':
    main()
