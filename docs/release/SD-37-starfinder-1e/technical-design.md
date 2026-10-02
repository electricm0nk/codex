---
canonical: true
owner: operator
bundle_id: SD-37
date: 2026-10-02
---

# SD-37 Technical Design

This is the architectural surface SD-37 changes. Source: the engine-reuse research report of
2026-10-02, re-checked against `tranche/17` @ `20bf84a3b2` where a figure is cited (`CUI F-n`).
Names marked *(proposed)* are design choices that the owning card confirms or replaces in its
receipt.

## 1. What carries over and what does not

| Layer | Same types as PF? | SD-37 treatment |
|---|---|---|
| `.pcc`/`.lst` parsing, include graph (`crates/codex-ingest/src/pcgen_import/{pcc.rs,include_resolver.rs,lst_parser/**}`) | yes | Reuse. E3.1 adds SF's campaign-loaded game-mode files (`STAT:`, `SIZE:`, `SAVE:`, `ALIGNMENT:`, `VARIABLE:`, `DATATABLE:`, `DYNAMIC:`, `GLOBALMODIFIER:`) and `system/gameModes/Starfinder` (CUI F-18). |
| Sheet-rule IR (`src/rules_core/sheet_rule.rs`: `SheetRule`, `Expr`, `SheetLine`, `Rat`, `EvalContext`) | mostly | Additive variants (E2.1). `Other(String)` is **not** used as a back door. |
| Converter (`crates/codex-ingest/src/pcgen_import/sheet_rule/**`, `bin/sheet_rule_convert.rs`) | PF-parameterised | System parameter (E1.3); formula-system reader (E3.2); SF mapping table (E3.3). |
| Compute spine (`pilot_compute/`, `combat.rs`, `PilotSnapshot`) | **no** (single AC, touch, CMB/CMD, hit die) | Not ported. A generic SF chassis reader in the `generic_class_chassis.rs` pattern (E4). |
| `rules_tables/` (180,883 lines, CUI F-10) | PF content only | Becomes a data package (E4a). SF never reads it. |
| Desktop shell, save envelope (`game_system`, `src/saved_character/mod.rs:34`), landing (`'starfinder-1e'`, `LandingScreen.tsx:11`) | yes | E6 routes the existing seam to a real adapter. |

## 2. Game-system partition (E1)

- A `GameSystem` id, `pathfinder-1e` | `starfinder-1e`, defined in one module *(proposed
  `src/rules_core/game_system.rs`)*. It reuses the desktop's literal ids.
- **Package roots per system** (`decisions.md §7`):
  - PF: `data/sheet_rules`, `data/corpus`, `data/converted` (unchanged).
  - SF: `data/starfinder-1e/sheet_rules`, `data/starfinder-1e/corpus`.
- **Runtime resolution.** Today `corpus_loader::live_sheet_rules()` joins
  `env!("CARGO_MANIFEST_DIR")/data/sheet_rules` (`src/rules_core/corpus_loader.rs:360`, read
  2026-10-02), and the core crate has no override. The desktop's
  `character_hub::sheet_rule_package()` resolves through `authoring_workbench::codex_repo_root()`
  (`CODEX_REPO_ROOT` → packaged resources → baked path). E1.1 gives the core reader the same
  runtime resolution, keyed by system. One `OnceLock` per system replaces the single global one.
- **Book registries.** The 19 BOOKS consts (CUI F-13) become a per-system registry, or are reached
  only through one. Pools such as `OptionSet::Rules{pool:"feat"}` resolve within a system, so an SF
  character is never offered PF feats.
- **`(kind, slug)` collisions** are namespaced by system (SD-h).
- **Proof:** PF byte-identical render hashes for Aldric and Elowen, plus the structural diff
  (49,450 records unmoved, CUI F-15).

## 3. Schema (E2)

- `BonusTarget::{Eac, Kac, Stamina, Resolve}` and `Expr::KeyAbilityMod`. `STACKING_TYPES` becomes
  per system (today it is a PF const: "Game-rule constant (Pathfinder)"). An SF `SpellKind` (no
  arcane/divine split; levels 0–6). `CharacterFacts` gains `theme` and `key_ability`.
  `ClassChassis` gains `hp_per_level`, `stamina_per_level` and `key_ability`, read off converted
  rows the same way `hit_die` is.
- All additive: serde enums, so existing PF JSON round-trips byte-equal (E2.1's gate).
- **Published contract:** `schemas/rules/sheet_rule.schema.json`, generated from the serde types.
  A `verify.sh` stage, `rules-schema-check`, fails on drift. Today `schemas/` holds only
  `schemas/update/*` (`ls schemas`).

## 4. Starfinder converter (E3)

```
$PCGEN_REPO_DIR/data/starfinder (12 .pcc, 131 .lst; CUI F-1, F-2) + system/gameModes/Starfinder
  └─ pcc.rs → include_resolver.rs (+ SF game-mode includes, E3.1)
     → lst_parser/** → formula-system reader (MODIFY/MODIFYOTHER/CHANNEL/DATATABLE, E3.2; 1,954 tokens, CUI F-9)
     → SF mapping table (E3.3) → data/starfinder-1e/{corpus,sheet_rules} (E3.4 CRB, E3.5 seven more)
  └─ gates: --check, token_coverage (SF), pcgen_residue_gate --closure, PI screening (SF term set), structural_diff.py
```

**The overloaded-field hazard** (`decisions.md §8`): `HP|ALTHP` = HP; `HP|CURRENTMAX` (+ `HD:1`
per level, hypothesis) = Stamina; `COMBAT|AC` split by `TYPE=EAC_Armor`/`KAC_Armor`; and
`FACT:KeyAbilityScore`. The SF mapping table holds one oracle row per field. A field without a row
is a named refusal.

**Include structure:** SF's PCCs load game-mode-level files from the campaign. PF keeps those under
`system/gameModes/Pathfinder`. That is SD-35's "second `.pcc` include structure".

**Not covered by the oracle** (E7.1 must list these): starship rules (absent); the Resolve formula
(no direct row); books after COM/Near Space (not in the pinned tree, **estimate**, inferred from
the 12-`.pcc` list); and anything in a `STATUS:BETA` PCC that PCGen itself computes wrongly. A
parity PASS covers only what PCGen's SF game mode computes.

## 5. SF chassis compute (E4) — generic, no per-class modules

A generic SF chassis reader over converted data *(proposed `src/rules_core/pilot_compute/sf_chassis.rs`)*:

| Sheet value | Rule (planner's statement; E0.4 transcribes and cites the SRD for each) | Data source |
|---|---|---|
| BAB | full = level; ¾ = floor(¾ × level) | `BONUS:COMBAT\|BASEAB` |
| Fort/Ref/Will base | good = floor(level/2)+2; poor = floor(level/3) | `BONUS:SAVE\|BASE.*` |
| HP | race HP + class HP × level | race row + `HP\|ALTHP` |
| Stamina | (class SP + Con mod) × level | `HD` + `HP\|CURRENTMAX` (E3.3 settles) |
| Resolve | max(1, floor(level/2)) + key ability mod | hand-transcribed (SRD) |
| EAC / KAC | 10 + armour EAC/KAC bonus + Dex mod (capped by the armour's max Dex) | armour rows `TYPE=EAC_Armor/KAC_Armor` |
| Skills | ranks + ability mod + class-skill bonus (when ranks ≥ 1) + armour check penalty where it applies | `scr_skills.lst`, class/theme class skills |
| Spells | known/per day by class table, levels 0–6; DC = 10 + spell level + key ability mod | `scr_classes.lst`, `scr_spells.lst` |
| Bulk / credits | sum of item bulk (L = 1/10); credits from item cost | `QUALITY:BULK`, item cost |

The rule statements in this table are the planner's recollection. **They are not evidence.**
E0.4's SRD transcription is the fixture source, and any disagreement is settled for the SRD.

The SF adapter *(proposed `apps/desktop/src-tauri/src/sf_adapter.rs`)* implements
`RuleSystemAdapter`. Its trait methods currently return PF structs
(`chassis_resolve -> PilotBaseChassisComputation`). E4.6 either returns a system-neutral result
(sheet lines + named totals) or returns the same `ComputationExplanation` list. E4.6 records which
it chose, and Pf1Adapter's output stays byte-identical.

## 6. `rules_tables` → data package (E4a)

- Format: JSON, one file per table, shaped by the existing serde types *(proposed root
  `data/rules_tables/`)*. Schema: `schemas/rules/rules_tables.schema.json`. Licence/PI stamped per
  file (`pi_screening::classify_field`; PI sweep baseline `docs/governance/pi-sweep-baseline.tsv`).
- Loader: a new file *(proposed `src/rules_core/rules_data_package.rs`)*. It is **never**
  `corpus_loader.rs` (fence).
- Bundle path: added to `tauri.conf.json` resources; the `tauri-resources-tracked` stage covers it.
- 252 importers re-pointed in one dispatch (CUI F-12). Afterwards the Rust tables are removed, or
  each kept file is named with its reason.
- Proof: PF byte-identical renders + catalogs; the Bestiary 1 monster count equal before and after.

## 7. Desktop (E6)

- Landing → `starfinder-1e` → `StarfinderAdapter`. Stub registry 0002 is retired for SF
  (`docs/governance/wired-integration-stubs-registry.md`).
- Creation flow: race → theme → class → SF point buy (`abilityScoreMethods.ts` gains the SF table,
  read from the engine, not hand-kept).
- Sheet layout: SP/HP/RP and EAC/KAC. No CMB/CMD/Touch/Flat-Footed (today `CharacterSheet.tsx`
  prints those for PF). Every number comes from an engine explanation row.
- Catalogs: new SF catalog files reading `data/starfinder-1e/sheet_rules`.
- ui-smoke: six seeds, isolated `XDG_DATA_HOME` (`apps/desktop/scripts/ui-smoke/run.mjs` already
  isolates; E6.6 adds the SF rows to `spec.json`).

## 8. Verification surfaces

- `verify.sh` (51 stages, CUI F-16) gains: SF `preflight-oracle` coverage (E0.1),
  `rules-schema-check` (E2.2), SF `sheet-rules-check`/`token-coverage` (E3), and SF seed fixtures
  (E4). E7.2 re-derives `scripts/verify-baselines.env`. A second system moves most count-pinning
  baselines (memory `book-onboarding-tax-is-per-file-not-per-record`).
- Oracle parity: `scripts/oracle_harness/**`, `scripts/pcgen-run-character.sh`. Whether the runner
  can select the Starfinder game mode is **unverified** (`risks-and-open-questions.md` R-4). E7.1
  adds game-mode selection if it is absent.
