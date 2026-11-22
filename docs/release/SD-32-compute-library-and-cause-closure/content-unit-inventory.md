---
canonical: true
owner: god-emporer
status: planning-ready (chassis completed 2026-08-22; populated at Gate 0 close)
date: 2026-08-22
---

# SD-32 Content-Unit Inventory

Per-content-unit N-tuple: **(kind, book, unit_id, family, engine_or_card_id, expected_artifact)**.

At chassis time, this file is the **shape** of the inventory, not its content. The actual
unit-level entries are populated when Gate 0 closes — the Gate 0 census walk produces the
authoritative row for every unit in the closed census, and this file becomes the per-unit map
into the gate/cycle that handles it.

## Schema

| Column | Type | Source | Notes |
|---|---|---|---|
| `kind` | enum | `docs/work-inventory.json` `kind` field | One of: `feat`, `class`, `class_feature`, `spell`, `monster`, `monster_ability`, `equipment`, `equipment_modifier`, `companion`, `race`, `race_trait`. |
| `book` | string | `docs/work-inventory.json` `book` field | Path under `data/corpus/<publisher>/<book>/`. |
| `unit_id` | string | `docs/work-inventory.json` `unit_id` field | Stable across cycles; canonicalised at Gate 0. |
| `family` | enum (F1..F10) | Gate 1 mapping | One of the ten semantic families from SD-31 wave 31, or `unclassified` (zero is the Gate 1 closure target). |
| `engine_or_card_id` | string | Gate 2 / Epic 1-4 mapping | Engine that emits this unit's values (Gate 2) or card that handles it (Epic 2 cause closure, Epic 3 class reachability, Epic 4 book onboarding). |
| `expected_artifact` | path | Cycle receipt's `artifacts/<gate>/<cycle-id>_cycle_receipt.md` | Where this unit's evidence lives. |

## Populated at Gate 0 close

The full table is produced by:

```bash
# After Gate 0 closes, regenerate the inventory
cargo run --locked --bin v06_work_inventory  # writes docs/work-inventory.json

# Append the per-unit rows from the live inventory
python3 - <<'PY'
import json, csv
inv = json.load(open('docs/work-inventory.json'))
with open('docs/release/SD-32-compute-library-and-cause-closure/content-unit-inventory.md', 'a') as f:
    f.write('\n\n## Per-unit rows (populated at Gate 0 close)\n\n')
    f.write('| kind | book | unit_id | family | engine_or_card_id | expected_artifact |\n')
    f.write('|---|---|---|---|---|---|\n')
    for u in inv['units']:
        f.write(f"| {u['kind']} | {u['book']} | {u['unit_id']} | _pending Gate 1_ | _pending Gate 2_ | _pending cycle_ |\n")
PY
```

**Do not hand-edit the populated table.** It is regenerated from `docs/work-inventory.json` and
the Gate 1/2 mappings on every closure cycle that updates the inventory. Hand-edits are a class
of stale figure the standing discipline forbids.

## Aggregate counts (re-derived at every cycle that quotes them)

At chassis time, the starting baseline is `epic-breakdown.md`'s figures (24,914 not-done units;
3,201 compute-library ceiling; 77 prestige classes; 4 unbuilt books). These are *the starting
baseline*, not a frozen value — every cycle that quotes a number re-derives against the live
`docs/work-inventory.json` and names the command.

## Cross-references

- `docs/work-inventory.json` — the live source of truth.
- `epic-breakdown.md` — the ceiling figures per epic.
- `acceptance-and-verification.md` — AT-32-G0-002 ("kind-unenumerable" category, if any) and
  AT-32-G1-001 (family coverage).
- `SD-31-corpus-closure-grind/artifacts/THE-BOX.md` — the SD-31 inventory (the predecessor's
  not-done map; this bundle inherits its 46-group partition).
