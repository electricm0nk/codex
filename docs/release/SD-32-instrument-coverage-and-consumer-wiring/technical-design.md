---
canonical: true
owner: god-emporer
status: planning-ready (2026-08-13)
date: 2026-08-13
companion_to: ./epic-breakdown.md
---

# SD-32 Technical Design

## 1. The pipeline, and where the line is

```
  PCGen corpus (.lst)
        |
        v
  src/bin/v06_work_inventory.rs        <-- WRITABLE (E2, E5-F1, E6-F1)
    enumerate_file / refine_kind             the generator: enumerates units,
    classify()  -> status                    runs the probes, assigns status
    wiring_class determinator -> class        and wiring_class
    probe_equipment_effect_wiring()
    probe_feat_effect_wiring()
    probe_race_trait_corpus()
        |
        v
  docs/work-inventory.json             <-- generated output
        |
==================== READ-ONLY LINE ====================
        |
        v
  pf1e_dashboard_producer.py           <-- READ-ONLY, NO EXCEPTIONS
    doneness_verdict(wiring_class, status, kind)
    DONENESS_MEANING / NO_GROUNDING_PROBE / EXCLUDED_BOOKS
        |
        v
  /home/ubuntu/swarm-observer/PF1e-dashboard.json   <-- READ-ONLY
  /home/ubuntu/swarm-observer/PF1e-dashboard.html   <-- READ-ONLY
```

Everything above the line is the reality. Everything below it is the
measurement. This bundle works above the line only. `decisions.md §2` is the one
request to change something below it, and it is a request to the operator.

## 2. Surfaces this bundle touches

### 2.1 `probe_equipment_effect_wiring()` — E2

Current shape (`src/bin/v06_work_inventory.rs`):

- corpus roots: `OBSERVABLE_BOOK_DIRS`, six books.
- key universe: `crb_equipment_tables::equipment_tables()`,
  `apg::equipment_tables::EQUIPMENT_TABLE`, `acg::equipment_tables::equipment_tables()`,
  `beastiary1::equipment_tables::EQUIPMENT_TABLE` — **four** books.
- predicate: `equipment_key_is_wired(key, corpus)` — equip the item alone, ask
  `compute_equipment_effects`, accept if any of `armor_class_bonus`, `max_dex`,
  `spell_failure`, `armor_check_penalty`, `skill_bonus`, `ability_bonus`,
  `weapon_enhancement_bonus` is `Some`.

Eleven books ship a compiled `src/rules_core/rules_tables/<book>/equipment_tables.rs`:
`crb`, `apg`, `acg`, `beastiary1`, `pathfinder_unchained`, `advanced_race_guide`,
`ultimate_magic`, `ultimate_combat`, `ultimate_psionics`, `ultimate_equipment`,
`ultimate_intrigue`.

E2 widens roots and key universe to the full set, **enumerated from the module
tree** so a twelfth book cannot be silently omitted, and leaves
`equipment_key_is_wired()` byte-identical (`decisions.md §4`).

### 2.2 `compute_equipment_effects` and the effect shapes — E3

The 375 bucket-A1 units are items the probe already equips and which return
nothing. Two distinct causes, which E3-F1 must separate before E3-F2 writes
anything: the engine has no code for the effect shape, versus the engine has the
code and this item is not routed to it.

### 2.3 The wiring-class determinator and `token_closure_rows()` — E4

`token_closure_rows()` already exists in `src/bin/v06_work_inventory.rs` (it has
its own unit tests: `token_closure_rows_unions_base_row_and_mod_rows`). The
current wiring-class signals include `computed:pre_guard`, `derived:bonus`,
`static:literal_magnitudes_only`. The `display` verdict means the determinator
found no magnitude token anywhere on the unit — which the producer's own comment
documents as unreliable for units that inherit magnitude from mod rows
(`bloodrager_indomitable_will` is the worked example). E4 is the closure-aware
successor, gated on E4-F1's hand-labelled sample.

### 2.4 The `static` sweep and the `derived` check — E5/E6

Neither instrument exists. Both are named in the dashboard's own
`doneness_meaning`. Both are gated on `decisions.md §2`, because neither has a
`done` rung to land on.

## 3. Surfaces this bundle must NOT touch

| surface | why |
|---|---|
| `~/.hermes/.../pf1e_dashboard_producer.py` | it is the measurement; `decisions.md §1`, `§2` |
| `/home/ubuntu/swarm-observer/PF1e-dashboard.json`, `.html` | measurement output |
| `doneness_verdict()`, `DONENESS_MEANING`, `DONENESS_VALUES` | bucket definitions |
| `NO_GROUNDING_PROBE` | `decisions.md §6` — stale for `companion`, still not ours to edit |
| `EXCLUDED_BOOKS` | changing it changes every corpus-wide figure at once |
| `equipment_key_is_wired()`'s body | it is the bar E2 widens coverage of; `decisions.md §4` |
| `data/corpus/**` regeneration | regenerating destroys license/PI fields and `raw_tokens`; no test covers it |

## 4. Concurrency and environment

- One writer per tree. `git worktree add` plus a per-agent `CARGO_TARGET_DIR`
  per source tree. Never under `/tmp`. Delete it at the end.
- `git status --porcelain` before **every** git write; never `git add -A`;
  never `git stash` in this checkout (it is tree-wide even from a subdirectory).
- Full sweep needs ~24 G free; check disk first.
- `apps/desktop/src-tauri` is a **separate cargo workspace** — a root sweep does
  not cover it. Test it explicitly when an E3 change reaches it.
- `./scripts/verify.sh` FULL, exit code captured directly, never through a pipe.
  ~27 min median, tail to 78. Launch it early in the background to a log.

## 5. Verification of a doneness claim

A cycle claiming units moved re-runs the generator, then:

```
python3 docs/release/SD-32-instrument-coverage-and-consumer-wiring/artifacts/derive-movable-mass.py
```

which re-validates its own transcription of the producer's verdict table against
the live payload before printing. If that assertion fails, the producer's table
changed and **every count in the receipt is void** until the split is re-derived.
