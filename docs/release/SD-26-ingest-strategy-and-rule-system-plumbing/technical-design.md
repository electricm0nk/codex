# SD-26 — Technical Design

## 1. Architectural posture

SD-26 is **mechanical-fanout + comparator + audit**. The comparator + normalization + parity writer land at E2; the JSON cache + book stub manifest fan out across E3 + E4; the audit on doctrine-cost reduction is E5.

## 2. Oracle-harness comparator (Epic 2)

### 2.1 Comparator shape

```rust
// src/oracle_validation/comparator.rs
pub fn compare(
    pcgen: &NormalizedOutput,
    codex: &SelectedDimensions,
    normalization: &[NormalizationRule],
) -> ComparisonResult {
    let matches = pcgen.dim_values.iter()
        .zip(&codex.dimensions)
        .filter(|(p, c)| normalization_rules_match(p, c, normalization))
        .map(|(p, c)| (p.id.clone(), p.value, c.value_i16))
        .collect();
    let mismatches = pcgen.dim_values.iter()
        .zip(&codex.dimensions)
        .filter(|(p, c)| !normalization_rules_match(p, c, normalization))
        .map(|(p, c)| DimensionMismatch {
            dimension_id: p.id.clone(),
            pcgen_value: p.value,
            codex_value: c.value_i16,
            normalization_rule_id: first_failing_rule(p, c, normalization),
        })
        .collect();
    ComparisonResult { matches, mismatches }
}
```

### 2.2 Normalization rule examples

- PCGen `0` vs Codex `0` → match.
- PCGen trailing whitespace in spell names → stripped before comparison.
- PCGen's spell level field "1" → normalized to integer 1.

The full rule set lives in `src/oracle_validation/normalization.rs` as a `Vec<NormalizationRule>`.

### 2.3 Parity report

`parity_report_<case-id>.md` per case:

```markdown
# Oracle parity report: <case-id>

## Summary
<pass/fail counts>

## Per-Dimension Comparison
| Dimension | PCGen | Codex | Match | Notes |
|---|---|---|---|---|
| combat.baseline_melee_attack_bonus | 0 | 0 | yes | — |
| defense.baseline_armor_class | 14 | 14 | yes | — |
| defense.total_save.fortitude | 2 | 2 | yes | — |
| ... |

## Normalization Rules Used
- trailing-whitespace-strip (per `normalization.rs:N`)
- integer-coercion (per `normalization.rs:N`)

## Discovered Deltas
(listed per mismatch)
```

The pilot case (PF1 CRB Human Fighter level 1) upgrades `current_claim_status` from `not_yet_grounded` to `oracle_checked` after the comparator passes.

## 3. JSON cache (Epic 3)

> **Superseded by SD-25 execution findings — see `decisions.md §11` for the full evidence and citations.** SD-25's Epic 7 (equipment/spell corpus intake) actually ran real per-book field-completion work against CRB/APG/Bestiary-1 after this section was originally drafted (2026-07-21, from a prior conversation, before SD-25 executed). The schema and storage sections below are corrected in place; `decisions.md §11` carries the full reasoning, evidence table, and citations back to the real SD-25 cycle receipts — read that section before authoring E3's cycle docs.

### 3.1 Shape (B per `decisions.md §7`, corrected per `decisions.md §11.1`/`§11.2`)

```json
{
  "population": "in_scope",
  "completeness": "chassis_only",
  "ingested_at": "<ISO-8601, stamped at JSON-write time — NOT derived from git log; see decisions.md §11.1>",
  "data": {
    "class_id": "alchemist",
    "maxlevel": 20,
    "bab": "level*3/4",
    "save_fort": "level/2+2",
    "save_ref": "level/2+2",
    "save_will": "level/3"
  },
  "source": {
    "kind": "lst_token",
    "path": "pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst",
    "sha256": "<64-hex>",
    "line": 11,
    "record_key": "CLASS:Alchemist"
  }
}
```

`completeness: full` adds per-level features (Bombs, Discoveries, Mutagen for Alchemist). `completeness: chassis_plus_extract` adds the BAB/save chassis + the spell slots per level but no named features.

**`source` is a discriminated union, not always the `lst_token` shape above** — SD-25 proved a large, real fraction of CRB/APG/Bestiary-1's now-completed equipment/spell fields have no LST-token provenance at all (web-second-sourced, `.COPY=`-inherited, same-book fallback, or a corrected ingestion-bug fix). The other four `source.kind` values (`lst_inherited_copy`, `lst_corrected_ingest`, `web_second_source`, `same_book_fallback`) and their required fields are specified in full in `decisions.md §11.2`, with the real, measured proportion of each kind per book/field. **Do not build E3's cycles against the single-shape `lst_token`-only schema above without reading that section — for APG equipment description, 0/338 records would even be representable, since the entire populated field is web-sourced.**

### 3.2 Storage paths

`data/corpus/<book>/<content_kind>/<content_id>.json`. Per-book:

- `data/corpus/core_rulebook/class/{fighter,wizard,cleric,rogue,sorcerer,barbarian,bard,druid,monk,paladin,ranger}.json`
- `data/corpus/core_rulebook/spell/{level_0,level_1,...level_9}/*.json`
- `data/corpus/core_rulebook/equipment/{weapon,armor,shield,wondrous,consumable}/*.json`
- Similar for APG, ACG, Bestiary 1.

### 3.3 Generation strategy (added, per `decisions.md §11.3`)

**E3's cycles generate the cache by serializing the CURRENT state of the completed Rust `rules_tables` modules (`src/rules_core/rules_tables/{crb,apg,beastiary1}/...`) — they do NOT re-parse the raw PCGen LST corpus from scratch.** SD-25 already did real, hand-verified field-completion work directly in those Rust modules (fixing ingestion bugs, applying `.COPY=`-inheritance, web-second-sourcing genuinely corpus-absent fields); re-parsing raw LST would silently regress every one of those fixes and could not recover the web-sourced content at all (it has no LST line to re-derive from). Each E3 cycle reads its book's module via the same public accessors `corpus_ingest_diagnostic.rs` (SD-25 Epic 5) already calls (`ClassId::ALL`, `SPELL_LIST`, `equipment_tables()`, etc.), serializes to §3.1's corrected shape, and — per `decisions.md §11.6` — should account for or fix two known corpus-hygiene defects before/during generation: CRB `equipmods.rs`'s 314 duplicate-key shell records, and `beastiary1::mod.rs`'s missing `MonsterId::ALL` constant (both currently force downstream consumers to hand-maintain workarounds).

**Real, measured coverage ceilings per book/field** (not every book reaches 100%, and this is expected — see `decisions.md §11.4` for the full table with citations): CRB equipment description 2021/2977 (67.9%); APG equipment description 331/338 (97.9%); APG spell full-text 284/297 (95.6%); Bestiary-1 equipment 4/4 (100%, but the real record count is 4, not the ~7 originally estimated — verify against the live corpus before writing any cycle doc's acceptance criterion). ACG (advanced_class_guide) was not touched by this SD-25 pass and its real ceiling should be independently verified before E3.3 assumes any number.

## 4. Book stub manifest (Epic 4)

### 4.1 Per-book cycle shape

Each of 21 cycles:

1. Verifies the PCGen corpus directory exists at `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/<book>/` (the 21 future-state books).
2. Writes `data/stubs/<book>.json` with `{book_id: <book>, book_name: <display>, planned_resolution_bundle: "SD-27", content_kind_counts: null, registered_at: <ISO-8601>}`.
3. Adds an entry to `docs/governance/wired-integration-stubs-registry.md` in the `book_stub` kind shape.

### 4.2 Stubs Registry format change (Criterion 4.1)

The `book_stub` kind is new. It coexists with the existing `codex-stub` kind (entry #0001 browser-preview fallback). The format:

```
### book_stub — entry #NNNN — <book_id>
- book_id: <book-slug>
- book_name: <book-display-name>
- status: stubbed
- planned_resolution_bundle: SD-27
- registered_by: claude-code
- registered_at: <ISO-8601>
- operator_granted: true
```

## 5. Doctrine-cost reduction (Epic 5)

### 5.1 Audit method

Single audit cycle that measures the per-class cycle floor on the SD-26 JSON cache + SD-25 PCGen runner pipeline. Operates against real cases (`tests/fixtures/rules_core/pf1_*_level*_*.txt`).

**Measured metrics per cycle:**
- Total cycle time (operator clock).
- Per-step time: RED setup, GREEN setup, dual-audit grep, doc-comment write, artifact write, commit + push, progress.md update.

**Targets post-cut:**
- RED + GREEN + dual-audit: ~5 minutes (unchanged — load-bearing).
- Doc-comment write: 0 minutes (cut; JSON cache's SHA frontmatter carries the durable audit trail).
- Artifact write: 30 seconds (cut to RED → GREEN + dual-audit + duration_seconds only — not 135 lines).
- Commit + push: 60 seconds (unchanged — load-bearing for the per-cycle concurrent-write protocol).
- Progress.md update: 30 seconds (cut to one row per cycle).

**Total target:** ~6 minutes floor per class. A reduction from ~20-40 minutes per prior receipts.

### 5.2 What does NOT cut

- RED → GREEN is load-bearing (TDD mandate).
- Dual-audit gate is load-bearing (operator-pinned 2026-07-20).
- Concurrent-write protocol is load-bearing (template §5).
- Operator identity in commits is load-bearing.

## 6. Closure (Epic 6)

Standard sub-pipeline: architecture-truth-up + graphify-update + PR + merge. Per-criterion subagent tiering:
- 6.1 = Sonnet
- 6.2 = Opus
- 6.3 = Haiku
- 6.4 = Haiku
- 6.5 = Sonnet

## 7. Cross-reference

- `./scope-draft.md §1`
- `./decisions.md §7` — JSON schema (Shape B)
- `./decisions.md §8` — Stubs Registry `book_stub` kind
- `./content-unit-inventory.md`
- `./loop-instruction.md §6` — per-cycle procedure
- `../SD-25-ui-evaluation-defect-closure/technical-design.md` — PCGen runner + Hub-of-Hubs interface (SD-25 outputs that SD-26 consumes)
- `src/oracle_validation/` — Oracle-harness schema (E2 reads + extends)
- `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/` — 26 PF1 book corpus
