---
title: GE-02 Technical Requirements
stc_id: STC-CODEX-GE-02
artifact_type: technical-requirements
status: accepted
scope: programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages
source_stc: ./README.md
source_artifacts:
  - ../../plans/spec-domains/GE-02-canonical-rules-model-and-content-packages.md
  - ../GE-01-legacy-corpus-and-conversion-matrix/README.md
  - ../GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-corpus-inventory.csv
  - ../GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-token-taxonomy.csv
  - ../GE-01-legacy-corpus-and-conversion-matrix/artifacts/conversion-matrix.csv
  - ../GE-01-legacy-corpus-and-conversion-matrix/artifacts/unsupported-token-ledger.csv
  - ../GE-01-legacy-corpus-and-conversion-matrix/references/oracle-surface-inventory.md
---

# GE-02 Technical Requirements

## Objective
Define the normative requirements for the Codex canonical rules model and content-package substrate required by the PF1 Core Rulebook Human Fighter level 1 pilot.

## Normative language
- **MUST** means required for GE-02 closure or for any downstream implementation that claims to satisfy GE-02.
- **SHOULD** means expected unless a later decision surface records a justified deviation.
- **MUST NOT** means prohibited for this STC.

## TR-02-001 — Canonical substrate posture
Codex MUST treat GE-02 as the canonical-model boundary between legacy PCGen source syntax and future native authoring/runtime behavior.

The canonical model MUST be:
- semantic rather than a direct LST token mirror
- package-oriented and version-aware
- stable-ID driven
- provenance-bearing
- able to distinguish human-authored source content from compiled runtime IR
- strict about unsupported, lossy, deferred, or unresolved behavior visibility

Codex MUST NOT treat GE-02 as:
- a parser implementation plan
- an engine implementation plan
- a new container for PCGen LST syntax
- proof of parity or conversion success by itself

## TR-02-002 — Upstream GE-01 input contract
GE-02 MUST use GE-01 as the governed pilot-source input surface.

At minimum, GE-02 MUST consume the following GE-01 surfaces:
- `pilot-corpus-inventory.csv` for source-package, include-edge, and pilot object-class requirements
- `pilot-token-taxonomy.csv` for pilot-critical semantic families and downstream ownership
- `conversion-matrix.csv` for target Codex concept homes, support disposition, lossiness, provenance, and validation obligations
- `unsupported-token-ledger.csv` for explicit unresolved modeling debt
- `oracle-surface-inventory.md` for source-truth and later comparison surfaces

GE-02 MUST NOT invent pilot object scope or token criticality when GE-01 already states the governed pilot posture.

## TR-02-003 — Pilot boundary
GE-02 MUST bound its first model requirements to the PF1 Core Rulebook Human Fighter level 1 pilot while keeping adjacent and deferred surfaces visible.

The first pilot model MUST cover, at minimum:
- Core Rulebook source package identity and include composition
- Human race and Human race-trait composition
- Fighter class identity, level-1 progression, class features, saves, BAB, proficiencies, and skill budget surfaces
- ability scores and save-stat bindings
- skills, class-skill relations, and representative skill fields
- feats, prerequisites, and choice-enabled feat behavior where pilot-adjacent
- equipment, equipment-to-proficiency references, armor/weapon fields, and basic mechanical effects
- effects, grants, formulas, prerequisites, choice sets, diagnostics, and provenance for those surfaces

GE-02 MUST record adjacent non-pilot domains as deferred rather than silently absorbing broad Pathfinder scope.

## TR-02-004 — Source package and manifest requirements
Codex MUST define source package requirements capable of representing campaign/source composition without cloning PCC syntax.

A package manifest model MUST support:
- package identity
- package version or source revision
- game system identity
- source/book identity where available
- include or dependency edges
- source-file lineage back to PCGen PCC/LST inputs when imported
- validation status and diagnostics summary
- authoring format versus compiled cache/IR boundary

The package model MUST represent the GE-01 `core_rulebook.pcc` include graph as package composition semantics, not as raw PCC text embedded in the canonical model.

## TR-02-005 — Stable ID requirements
Codex MUST define stable object ID requirements before implementation work claims canonical content support.

Stable IDs MUST be:
- deterministic within a package and object kind
- durable across import reruns when source identity remains stable
- independent from absolute local filesystem paths
- namespaced by package or equivalent source authority
- able to preserve legacy aliases or source names without making those aliases the only canonical identity
- suitable for future native-authored content as well as imported PCGen content

GE-02 MUST carry open decision pressure around exact ID syntax until an implementation-facing decision record or handoff fixes it.

## TR-02-006 — Pilot object kind homes
GE-02 MUST define model homes for every pilot-critical GE-01 target concept.

At minimum, the canonical object taxonomy MUST include homes for:
- `SourcePackage`
- `Race`
- `RaceTrait` or equivalent trait carrier/composition concept
- `Class`
- `ClassFeature` or equivalent grant carrier
- `Feat`
- `Skill`
- `Equipment`
- `Proficiency`
- `AbilityScore`
- `Save`
- `Effect`
- `Prerequisite`
- `Formula`
- `ChoiceSet`
- `Diagnostic`
- `ProvenanceRecord` / `SourceMapEntry`
- `CompiledRuntimeIR` or equivalent runtime cache boundary

The taxonomy MUST preserve the distinction between object identity, object fields, relations between objects, grants/effects, and executable/evaluable expressions.

## TR-02-007 — Effect and grant model requirements
Codex MUST define effect/grant model requirements strong enough to represent pilot behavior without hiding PCGen carrier indirection.

The effect model MUST support:
- automatic grants such as proficiencies and class features
- conditional grants and suppressions
- numeric modifications such as BAB, save, skill, AC, attack, and equipment effects
- source owner and trigger context
- target object or statistic
- stacking/combination posture as an explicit open or downstream rule when not resolved
- lineage to source files, tokens, and GE-01 matrix/ledger posture

Effect requirements MUST NOT collapse `ABILITY`, `AUTO`, `BONUS`, `CSKILL`, or trait replacement surfaces into undifferentiated prose.

## TR-02-008 — Prerequisite model requirements
Codex MUST define prerequisite model requirements capable of representing boolean predicate structure without flattening it into text.

The prerequisite model MUST support:
- predicate operators or equivalent structured composition
- object references and type/category references
- multi-branch prerequisite algebra such as `PREMULT`
- proficiency prerequisites and equivalent pilot-adjacent gates
- explicit unresolved/deferred posture when full semantics are not yet implemented
- readable diagnostics explaining unmet or unevaluated prerequisites

The exact expression/evaluation technology remains deferred, but the model MUST preserve prerequisite structure strongly enough that GE-03 can parse and GE-04 can later evaluate it.

## TR-02-009 — Formula and value model requirements
Codex MUST define formula/value model requirements for pilot progression, ability scores, saves, and skill budget surfaces.

The formula model MUST support:
- symbolic variables and references
- level-dependent values
- formulas preserved from legacy source where exact semantics are not yet converted
- typed value targets such as stats, saves, skills, attack, AC, and equipment fields
- source lineage for formula-bearing tokens
- downgrade paths when a formula is carried as unresolved rather than executable

GE-02 MUST NOT translate formula-bearing behavior into guessed literal numbers except inside later explicit fixtures that document their source and calculation path.

## TR-02-010 — Choice-set requirements
Codex MUST define choice-set requirements for selectable or repeatable options.

The choice-set model MUST support:
- choice source object
- allowed target set or selector expression
- cardinality/repeatability posture
- prerequisite interactions
- selected value provenance
- diagnostics for unavailable or unresolved choices

The Martial Weapon Proficiency `CHOOSE + MULT` surface from GE-01 MUST remain visible as choice-model debt until fully specified.

## TR-02-011 — Type, tag, and selector requirements
Codex MUST define how overloaded legacy `TYPE` and selector surfaces map into canonical classification and selection concepts.

The model MUST distinguish:
- semantic type/category used for rules behavior
- display or grouping facets
- selector inputs such as `TYPE=Craft` or proficiency groups
- legacy tags preserved only for provenance/debugging

Codex MUST NOT treat every legacy type string as a durable canonical category without review.

## TR-02-012 — Provenance and source-map requirements
Codex MUST define provenance requirements that allow later debugging, coverage review, and oracle comparison.

A provenance/source-map model MUST preserve, when available:
- package/source identity
- PCC include path
- LST source file path
- entry/object name
- line, token span, or equivalent structured location
- legacy token family or construct
- canonical target object and field/effect written
- support disposition and lossiness class
- linked diagnostics when behavior is unresolved, lossy, unsupported, deferred, or intentionally ignored

If first implementation cannot capture maximum span precision, the downgrade MUST be explicit and auditable.

## TR-02-013 — Authoring format versus compiled IR boundary
GE-02 MUST define a hard boundary between human-authored canonical content and compiled runtime IR/cache.

The authoring model MUST optimize for:
- reviewability
- stable IDs
- source-package versioning
- provenance and diagnostics readability
- future native authoring

The compiled IR/cache MUST optimize for:
- deterministic runtime loading
- efficient rule evaluation
- normalized references
- validation outcomes and diagnostic retention

Codex MUST NOT make the compiled IR the only source of truth for authored package content.

## TR-02-014 — Validation and diagnostics requirements
Codex MUST define validation requirements before content packages are trusted.

Validation MUST cover:
- package manifest structure
- stable ID uniqueness and reference resolution
- object kind and required-field validity
- effect target validity
- prerequisite/formula/choice-set parseability or explicit unsupported posture
- provenance/source-map completeness appropriate to imported content
- unsupported/lossy/deferred behavior reporting

Diagnostics MUST be machine-readable enough for tools and human-readable enough for review.

## TR-02-015 — Pilot representation requirement
The GE-02 model requirements MUST be sufficient to describe the PF1 Core Rulebook Human Fighter level 1 pilot conceptually without direct LST syntax copying.

At minimum, the pilot representation MUST be able to describe:
- package/source lineage for Core Rulebook input material
- Human race identity and trait composition
- Fighter class identity, level-1 progression, saves, BAB, class skills, proficiencies, and class features
- ability score and save foundations
- representative skills and class-skill semantics
- representative weapon and armor equipment with proficiency references
- a pilot feat choice and its prerequisite/effect/choice interactions where in pilot scope
- unresolved/deferred semantics as diagnostics or ledger-linked debt rather than hidden omissions

## TR-02-016 — Schema example requirements
GE-02 MUST include enough documentary schema examples or skeletons to prove the model homes are coherent, but MUST NOT freeze final production schema syntax unless a later decision record accepts it.

Examples SHOULD cover:
- source package manifest
- race object
- class object
- skill object
- equipment object
- effect/grant record
- prerequisite record
- formula record
- choice-set record
- provenance/source-map record
- diagnostic record

Examples MAY live in `technical-design.md` until implementation repo-local schema files are authorized.

## TR-02-017 — Downstream routing rule
GE-02 MUST route later implementation work into bounded downstream epics rather than smearing code intent across this source STC.

At minimum, downstream work MUST be decomposable into:
- package manifest and stable ID design/implementation
- pilot object schemas
- effect/grant model
- prerequisite and formula model
- choice-set and selector model
- provenance/source-map and diagnostics
- validation and fixture strategy
- compiled IR boundary and runtime loading handoff to GE-04

## TR-02-018 — Produced artifacts
GE-02 MUST produce both the source-STC control bundle and the concrete generated documentary artifacts that the spec domain actually owes.

The source-STC control bundle MUST contain:
- `README.md`
- `technical-requirements.md`
- `technical-design.md`
- `acceptance-and-verification.md`
- `risks-and-open-questions.md`
- `epic-breakdown.md`
- `references/ge01-governed-inputs.md`
- `references/ge03-importer-dependency-contract.md`

The generated GE-02 documentary output set MUST contain:
- `artifacts/canonical-model-specification.md`
- `artifacts/content-package-layout-specification.md`
- `artifacts/pilot-object-examples.yaml`
- `artifacts/provenance-source-map-specification.md`
- `artifacts/expression-language-decision-criteria.md`
- `artifacts/compiled-ir-boundary-definition.md`
- `artifacts/content-validation-and-diagnostics-specification.md`

These outputs MUST live under `programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/`.

This requirement is deliberately non-recursive: GE-02 is not complete merely because its STC describes itself. GE-02 is complete only when the named model/package/example/provenance/expression/IR/validation artifacts exist and are linked from the controlling documents.

## TR-02-019 — GE-03 importer dependency contract
GE-02 MUST provide a downstream contract that states what GE-03 importer planning may consume from the accepted GE-02 artifact set.

That contract MUST map:
- GE-01 pilot evidence and ledger pressure into importer parse/conversion obligations
- GE-02 canonical model homes into GE-03 conversion-handler targets
- GE-02 content package layout expectations into importer output/package planning
- GE-02 provenance/source-map fields into parser and converter source-lineage requirements
- GE-02 validation and diagnostic classes into unsupported-token and conversion-report requirements
- GE-02 expression-language and compiled-IR boundaries into explicit non-authority constraints

The contract MUST NOT claim that GE-02 settles final production schema syntax, expression/evaluator implementation, runtime engine behavior, importer source-span precision, branch/worktree policy, repo write scope, verification commands, or code readiness.

The accepted contract lives at `references/ge03-importer-dependency-contract.md`.

## Success definition
GE-02 succeeds when Codex has a canonical-model planning surface strong enough to say:
- which concrete generated artifact defines each GE-02 output surface
- what semantic homes pilot source concepts belong to
- how content package identity and stable IDs will be constrained
- how effects, prerequisites, formulas, and choices remain structured rather than prose-only
- how imported content preserves source lineage and diagnostics
- where authored source content ends and compiled runtime IR begins
- which downstream implementation epics exist and which decisions remain deliberately open
- what GE-03 may consume from GE-02 without allowing the importer to redefine canonical model authority

If those answers still require invention, GE-02 is not complete.
