---
title: GE-02 Use of GE-01 Governed Inputs
stc_id: STC-CODEX-GE-02
artifact_type: reference
status: accepted
scope: programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/references
source_stc: ../README.md
---

# GE-02 Use of GE-01 Governed Inputs

## Purpose
Record exactly how GE-02 uses GE-01's accepted inventory, taxonomy, matrix, ledger, and oracle surfaces. This prevents GE-02 from becoming an abstract model fantasy detached from the pilot corpus.

## Input surfaces

| Surface | Path | Verified content | GE-02 use |
|---|---|---:|---|
| Pilot corpus inventory | `../../GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-corpus-inventory.csv` | 66 rows | Establishes package/include graph, pilot source files, object classes, and required-vs-adjacent posture. |
| Pilot token taxonomy | `../../GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-token-taxonomy.csv` | 26 rows | Establishes token-family criticality, semantic risk, and downstream owners for model homes. |
| Conversion matrix | `../../GE-01-legacy-corpus-and-conversion-matrix/artifacts/conversion-matrix.csv` | 29 rows | Maps legacy constructs to target Codex concepts, support disposition, lossiness, provenance, and validation obligations. |
| Unsupported-token ledger | `../../GE-01-legacy-corpus-and-conversion-matrix/artifacts/unsupported-token-ledger.csv` | 13 rows | Establishes unresolved high-risk canonical-model debt that must not be hidden. |
| Oracle surface inventory | `../../GE-01-legacy-corpus-and-conversion-matrix/references/oracle-surface-inventory.md` | grounded + candidate surfaces | Establishes source-truth and later parity/comparison surfaces without claiming behavior oracle readiness. |

## Canonical homes derived from GE-01

| GE-01 source pressure | GE-02 canonical home | Notes |
|---|---|---|
| `core_rulebook.pcc` root and include directives | `SourcePackage` manifest and dependency graph | Must preserve package/source lineage without storing raw PCC syntax as the canonical model. |
| Human `RACE` entry | `Race` + race-trait composition | Human race semantics are not a single flat row; carrier indirection must remain visible. |
| Fighter `CLASS` entry | `Class`, `ClassFeature`, `Formula`, `Effect` | Fighter level-1 progression includes formula and grant pressure. |
| `SKILL` rows and `CSKILL` carriers | `Skill`, `Relation/Selector`, `Effect` | Class-skill groups and `TYPE` selectors must not be pre-expanded blindly. |
| Armor/weapon/equipment rows | `Equipment`, `Proficiency`, `Effect` | Equipment-to-proficiency references and armor/weapon stats require structured model homes. |
| `WEAPONPROF / ARMORPROF / SHIELDPROF` | `Proficiency` and selector/category model | Proficiency concepts are referenced by feats, equipment, and class grants. |
| `ABILITY` carrier rows | `ClassFeature`, `RaceTrait`, `Effect/Grant` | Carrier rows must not disappear into prose. |
| `AUTO` grants | `Effect/Grant` | Automatic grant semantics require explicit source owner, target, and condition posture. |
| `BONUS`, `DEFINE`, `VAR` | `Formula / Value Expression` + `Effect` | Formula-bearing behavior must be preserved as structure, even if runtime evaluation is deferred. |
| `PRE*`, `PREMULT`, `PREPROFWITH*` | `Prerequisite` | Boolean/predicate structure must be preserved for later parser and engine work. |
| `CHOOSE + MULT` | `ChoiceSet` | Repeatable selectable options require cardinality and selector semantics. |
| `TYPE` and type selectors | `Relation / Selector` taxonomy | Legacy type strings require review before becoming canonical categories. |
| source paths, lines, token spans | `ProvenanceRecord` / `SourceMapEntry` | Required for debugging, coverage review, and oracle comparison. |
| unsupported/deferred matrix and ledger entries | `Diagnostic` | Required to prevent silent loss or counterfeit parity. |

## High-risk ledger pressures carried into GE-02

GE-02 must preserve these unresolveds as structured model pressure:
- multi-branch prerequisite algebra (`PREMULT / PREPROFWITH*`)
- archetype-conditioned proficiency suppression
- Fighter progression formulas and variables
- Human racial-trait carrier indirection
- Human trait replacement flags and `PREFACT` gates
- base-stat formulas and derived variables
- Fighter skill-point variable chain
- class-skill selectors using explicit skills and `TYPE` groups
- equipment-to-proficiency references
- `CHOOSE + MULT` repeatable proficiency choice behavior

## Oracle posture
GE-01 identified file-level and documentation-level oracle surfaces, including:
- Core Rulebook campaign root and include graph
- Human race subtree and Human racial-trait carriers
- proficiency feat surface
- Fighter class-skill and proficiency carriers
- core skills, base stats, saves, and listfile documentation
- generic loader and campaign source-entry implementation surfaces

GE-02 uses these as source-truth and comparison-pressure surfaces only. GE-02 does not claim runtime oracle automation exists or that parity has been proven.

## Closure judgment
The GE-01 governed input set is sufficient to generate and close the GE-02 source STC as `planning-ready` for the pilot boundary. The remaining uncertainties are model-design open questions, not blockers to creating the source STC.
