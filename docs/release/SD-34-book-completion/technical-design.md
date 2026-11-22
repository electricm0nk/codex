---
canonical: true
owner: god-emporer
bundle_id: SD-34
date: 2026-08-26
---

# SD-34 Technical Design

The instruments SD-34 builds and the ones it inherits. Every path is verified present at
`origin/develop` `ea2b3396f2` (SD-33's PR #377 merge commit) and must be re-verified at the `tranche/14` cut.

## 1. `scripts/completion_atlas.py` — the bundle's central new instrument

New, Epic 1. Reads `docs/work-inventory.json` and places every unit in exactly one of the ten
buckets fixed by `decisions.md §2`.

```
python3 scripts/completion_atlas.py --check
  -> population=49438 buckets=10 unclassified=0 overlap=0   (exit 0)

python3 scripts/completion_atlas.py --by-book
  -> per-book rows: units per bucket, with % and denominator

python3 scripts/completion_atlas.py --book <slug> --check
  -> per-bucket counts for one book; exit 0 only when every non-DONE bucket is 0
```

**Fail-closed on six conditions** (AT-34-E1-002), each with a RED->GREEN proof:

1. `unclassified != 0` — a unit in no bucket is a tool defect, not a data finding
2. `overlap != 0` — a unit in two buckets
3. a unit in `DONE` whose evidence does not support it
4. a bucket with no named clearing mechanism
5. a `derived_at` SHA that is not an ancestor of `HEAD` (staleness gate)
6. a bucket whose definition does not cite the `file:line` that emits the evidence strings it
   keys on — or whose citation no longer resolves at `HEAD`. **Assert on the cited content, not
   only the path and line**, so a refactor that moves code without changing line counts is
   caught (`risks-and-open-questions.md §10`).

Condition 4 is what stops the atlas degrading into a taxonomy nobody can act on: a bucket must
name the thing that empties it. Condition 6 is `decisions.md §12` L1 made mechanical.

**The atlas is SD-34's partition — the role `THE-BOX.md` played for SD-33.** There is no
`THE-BOX.md` in this bundle. `scripts/box_ledger.py --check` is inherited read-only as an
independent second partition of the same 49,438 units; it must keep passing and is never
written by SD-34 (`decisions.md §2`).

**Population printing.** Every invocation prints the population it examined alongside its
result. SD-33's corpus sweep was green on records whose `raw_tokens` were empty — its
population was "tokens the record claims", and a record claiming nothing cannot mismatch.

### Bucket derivation, as implemented

Derived from `status` plus `evidence`, not from `status` alone — the second field is what
distinguishes A from B from C:

| Bucket | Condition |
|---|---|
| `DONE` | `status in {grounded, text-complete}` |
| `V` | `status in {literal-verified, fixture-verified}` |
| `M` | `status == ingested-magnitude` |
| `A` | `status == not-ingested` and evidence contains `has_no_engine_table` |
| `B` | `status == not-ingested` and evidence contains `not_held_by_engine` / `absent_from` / `not_modelled` |
| `C` | `status == not-ingested` and evidence contains `explanation_id` / `diagnostic` |
| `D` | `status == not-ingested`, none of the above — **sub-causes enumerated, not shrugged** |
| `U` | `status == unmeasurable` |
| `X` | `status == deferred-with-reason` |
| `Z` | `status == not-started` |

**Note the coupling to `status` string values.** AT-34-E1-005 renames `not-ingested`; the
atlas must be updated in the same cycle or its A/B/C/D arms silently fall through to
`UNCLASSIFIED` — which the fail-closed check will catch, by design.

## 2. Inherited instruments — used, not rebuilt

| Instrument | Path | SD-34 use |
|---|---|---|
| Shape engine | `src/rules_core/pilot_compute/formula_interpreter*.rs`, `src/bin/formula_interpreter.rs` | Computes magnitudes. **Already works** — F1..F9, 10,626 of 11,652 recognised. SD-34 consumes it, does not extend it. |
| Oracle harness | `scripts/oracle_harness/{run,compare,oracle_export}.py` | Clears bucket `V`. Do not fork. |
| Oracle pin | `scripts/pcgen-oracle-pin.env` | Ground truth SHA. **`~/workspace/repos/pcgen` is forbidden** — the fetch script's default `--dest` resolves there and a `preflight-oracle` PASS against it fails silently. |
| Box ledger | `scripts/box_ledger.py` | SD-33's partition of the full inventory; complementary axis, not replaced. |
| Denominator gate | `scripts/denominator_gate.py`, wired in `scripts/verify.sh` | Live. **Default scope is SD-33's folder** — run with the explicit SD-34 glob until AT-34-E1-006 widens the default (`decisions.md §3`). Do not narrow. |
| Corpus literal sweep | `src/rules_core/corpus_literal_sweep.rs`, `src/bin/corpus_literal_sweep.rs` | Guards every corpus regeneration in Epics 2 and 3. |
| Work inventory | `src/bin/v06_work_inventory.rs`, `docs/work-inventory.json` | The unit population and the source of `status`/`evidence`. Epic 1 renames its status field. |
| Retro log | `scripts/retro.py`, `docs/retro/schema.json` | Event emission every cycle; `deferrals.open` trustworthy post-SD-32 fix. |

## 3. The shape-engine boundary — stated once, as fact

**A shape engine turns a formula string into a number.** `formula_interpreter` covers F1..F9:
population 11,652, recognised 10,626, refused 240, unjoined 786. It refuses rather than
guesses:

```
"var(\"CL=Arcanist\")" -> unrecognised function "var" — refusing rather than guessing its semantics
```

**It does not place the record, attach it, or display it.** The engine's own promotion ladder
is the authority (`src/bin/v06_work_inventory.rs:9595`) — four conditions, none of them
"a value was computed":

```rust
if has_real_description
    && is_display_wiring_class_for_promotion(wc_class)
    && !universal_sheet_modifier
    && facts.class_feature_pool_catalog_holds(&unit.source_book, &unit.key)
```

Fail the last and the verdict is `class_feature_owner_matched_by_name_but_record_not_held_by_engine`.

**Measured consequence:** 26,396 units carry magnitude tokens; 13,119 of those 26,396 are
still not held by the engine. AT-34-E1-004 commits this with its line number re-verified at
HEAD.

## 4. Engine tables — Epic 2's build surface

Bucket A is 8,463 units across 9 kinds with no table. The existing probe/table surface lives
in `src/bin/v06_work_inventory.rs` — the working precedents to follow:

```
probe_feat_effect_wiring          probe_equipment_effect_wiring
probe_race_trait_corpus           probe_spell_effect_wiring
probe_reachable_race_traits       probe_class_effect_wiring
probe_race_creation_roster        probe_class_feature_effect_wiring
```

Epic 2 builds **8 of the 9**: the seven the Core Rulebook exercises — `ability` (471 core),
`template` (262), `skill` (110), `domain` (34), `language` (22), `deity` (21), `companion`
(14) — plus `trait` (487 units, 154 of them in Ultimate Campaign).

**`power` (421 units) is the one table not built.** Every unit is inside `ultimate_psionics`,
which has all eight non-DONE buckets occupied — the table alone would not close that book.
Costed in Epic 5 from the measured build rate (`decisions.md §7`).

**A table must be fail-closed** (AT-34-E2-002): a real record, or a named refusal. Never a
fabricated or defaulted entry.

**"No table needed" is a legitimate outcome, and must be proven.** If every unit of a kind is
`display`-class with a rendered description as its terminal state, no magnitude table is
required — but that is established with counts, never assumed to save work.

## 5. Clearing the other buckets

| Bucket | Mechanism | Where it lives |
|---|---|---|
| `B` (11,921; 970 core, 0 in Ultimate Campaign) | place the record in the table that already exists | the catalog/pool builders in `v06_work_inventory.rs` and the engine's own tables |
| `C` (4,388; 370 core) | wire the explanation/display path so the player sees it | the explanation-id surface named in the promotion ladder |
| `M` (2,455; 512 core) | run the existing compute path and apply the result | shape engine + the apply path |
| `V` (8,330; 2,582 core) | run through the SD-33 oracle harness | `scripts/oracle_harness/` |
| `D` (1,230; 119 core) | per named sub-cause — e.g. `class_feature_of_unmodelled_corpus_class:*` | varies; enumerated by AT-34-E1-001 |
| `U` (321; 58 core) | per named sub-cause — instrument correction, or a proven statement that no verdict is possible | 270 `text_only_but_corpus_record_carries_no_description_to_show_a_player`, 51 `feat_served_description_is_a_placeholder_marker_not_prose`; kinds `equipment_modifier` 140, `equipment` 119, `feat` 62; enumerated by AT-34-E1-001 (SD-33 register C1.1) |

`D`'s largest sub-causes are already visible: `class_feature_no_dedicated_magnitude_id_matched_the_record_slug`
(156), `race_trait_record_loaded_but_never_applies` (44), and a family of
`class_feature_of_unmodelled_corpus_class:<name>` entries — `divine_scion` (46), `warrior` (44),
`phrenic_slayer` (31), `sighted_seeker` (25), `phantom` (24), `adept` (23). Those are unmodelled
classes, which is a fifth distinct mechanism and exactly the kind of thing the atlas exists to
name up front.

## 6. Cost measurement — Epics 2, 3 and 4's real output

Three ledgers, all **measured, never estimated**:

- `artifacts/epic-2-tables/table-build-rate.json` — per table: wall time, lines changed, what
  dominated, and whether the kind's shape made it cheaper or dearer. **The spread matters more
  than the average** — a single blended number across eight tables cannot honestly price
  `power`.
- `artifacts/epic-3-core-rulebook/step-cost-ledger.json` — per bucket, on a deep book.
- `artifacts/epic-4-ultimate-campaign/step-cost-ledger.json` — the same, on a shallow one.

Two books of opposite shape give Epic 5 a **range** rather than one blended rate, and a stated
account of which divergences are explained by book shape rather than noise.

This is Epic 5's only input for pricing the remaining 35 books. A bucket cleared without its cost
recorded has delivered half its value, which is why AT-34-E3-004 is a criterion rather than a
note.

**Throughput discipline, inherited from SD-33 at real cost:** measure per-unit cost on a
sample and project before any population-scoped run. A method proven at n=1 is not a method
proven at n=8,330 — SD-33's Epic 5 reached 32 of 8,330 by carrying a one-character-per-unit
method into a population, and it cost four remediation waves.

## 7. Corpus regeneration — Epics 2 and 3

`data/corpus/**` is **never hand-edited**. Guarded generator path only. Three hazards, all
previously observed here:

1. regeneration can destroy **license/PI metadata and `raw_tokens`** — verify per record;
   never pass `--allow-stamp-loss`
2. a **record-count change compiles clean** while leaving other files' hardcoded assertions
   red — grep old **and** new numbers across `tests/`, `src/`, `apps/`, `scripts/`
3. a **shallow glob lies** — use recursive search

`corpus_literal_sweep` is the guard, independently re-deriving each record's token closure
from the pinned oracle `.lst` via `.MOD`-chain walking. Run it after every regeneration.

## 8. Build scope

`decisions.md §10`. Three separate things, all required:

```bash
cargo test --locked --no-run                       # does everything COMPILE?
cargo test --locked                                # full workspace; count targets EXECUTED
cd apps/desktop/src-tauri && cargo test --locked   # SEPARATE cargo workspace
```

A single broken test target hid 543 of 543 integration suites behind a compile error during
SD-33 while the lib suite reported 2,836 passing.

**Inherited baseline:** SD-33 closed with 29 of 599 workspace suites carrying 46 of 8,034
failures, proven pre-existing at the `tranche/13` cut and registered forward (the figure was
31 / 49 of 8,026 through SD-33's attempt 10; its retrospective §5 records the correction). Re-derive at the
`tranche/14` cut. **A failure outside that set is SD-34's**, and "pre-existing" is proven
against the cut SHA with `git`, never asserted.
