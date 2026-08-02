---
title: GE-01 Technical Design
stc_id: STC-CODEX-GE-01
artifact_type: technical-design
status: active
scope: programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix
source_stc: ./README.md
last_reviewed_at: 2026-06-19
---

# GE-01 Technical Design

## Purpose
This design operationalizes the GE-01 source STC without collapsing it into implementation code. It defines the structure Codex should use to reason about legacy corpus truth, conversion coverage, unsupported behavior, provenance, and oracle discovery before a coding harness is allowed to act.

## Design posture
- architecture style: `documentation-first control plane with downstream parser/import subsystem boundaries`
- migration posture: `pilot-first, loss-aware, evidence-backed`
- provenance posture: `file -> include chain -> line/token span when available -> conversion rule -> diagnostic outcome`
- diagnostics posture: `strict about visibility; tolerant only when loss is recorded explicitly`

## Context and constraints
- Codex is a new project. The legacy PCGen repo is reference material, not the implementation surface.
- The Codex repo exists remotely at `https://github.com/electricm0nk/codex.git`, and the local checkout is now grounded at `/home/ubuntu/workspace/repos/codex`.
- The pilot boundary remains PF1 Core Rulebook Human Fighter level 1.
- GE-01 must not solve the canonical rules-model problem in full; it must define the migration-control plane that later epics will use.
- Unknowns must remain visible. Planning accuracy is more important than cosmetic completeness.

## Proposed system shape
GE-01 should be treated as a control-plane design composed of six documentary entities:

1. **Corpus Inventory** — identifies the PCC and LST surfaces relevant to the pilot.
2. **Token Taxonomy** — groups legacy tokens and semantic constructs by meaning and risk.
3. **Conversion Matrix** — states how each token family is intended to map into Codex concepts.
4. **Unsupported-Token Ledger** — records what is not handled, is lossy, or is intentionally deferred.
5. **Provenance Contract** — defines the minimum lineage data downstream tooling must preserve.
6. **Oracle Surface Catalog** — identifies how legacy PCGen behavior can later be queried or compared.

These entities belong in the requirements and planning surface first. Later implementation may mirror them in code or data, but must not invent a different structure without review.

## Data flow
1. **Legacy reference scan**
   - inspect PCC/LST files, loader surfaces, and token documentation in the read-only PCGen repo
2. **Documentary normalization**
   - turn discovered facts into inventory rows, taxonomy entries, matrix rows, ledger entries, and oracle notes
3. **Downstream routing**
   - assign each requirement to later epics for parser, translator, provenance, and validation implementation
4. **Future handoff boundary**
   - because the Codex repo checkout is now grounded, GE-01 may produce bounded planning-only bridge artifacts for documentary review or downstream epic clarification rather than coding directly from this STC
   - code-authorizing importer/parser/handler/CLI execution authority begins under GE-03 after GE-03 has its own source STC and a bounded follow-on handoff

## Component boundaries

### Corpus Inventory
- responsibilities:
  - represent pilot-relevant PCC/LST files and include edges
  - mark required vs adjacent vs unresolved files
- inputs:
  - source spec domain
  - live PCGen file paths
  - research findings
- outputs:
  - inventory records consumable by downstream implementation epics
- must not own:
  - canonical model design
  - parsing code

### Token Taxonomy
- responsibilities:
  - classify legacy token families and semantic constructs by meaning, risk, and pilot criticality
- inputs:
  - listfile documentation
  - source files
  - research findings
- outputs:
  - taxonomy entries that drive matrix coverage and risk analysis
- must not own:
  - final conversion algorithm

### Conversion Matrix
- responsibilities:
  - map legacy constructs to intended Codex concepts
  - record support disposition, lossiness, provenance expectations, and validation requirements
- inputs:
  - inventory
  - taxonomy
  - design assumptions from GE-02/GE-03 boundaries
- outputs:
  - matrix rows used to block counterfeit coverage claims
- must not own:
  - implementation-specific parser code

### Unsupported-Token Ledger
- responsibilities:
  - record unsupported, deferred, or intentionally ignored constructs
  - route them to the right downstream owner
- inputs:
  - matrix gaps
  - discovery findings
- outputs:
  - explicit work surface for unresolved migration burden
- must not own:
  - silent failure handling

### Provenance Contract
- responsibilities:
  - define minimum lineage fields preserved through future parsing and translation
- inputs:
  - reference architecture findings
  - source-file inspection
- outputs:
  - explicit provenance obligations for downstream import work
- must not own:
  - final storage engine choice

### Oracle Surface Catalog
- responsibilities:
  - identify legacy PCGen comparison surfaces and their trust limits
- inputs:
  - legacy runtime research
  - loader/runtime docs
  - CLI or export capabilities where grounded
- outputs:
  - validation-surface inventory for GE-05 and later harness work
- must not own:
  - final automated harness implementation

## Data and schema notes
Key documentary entities:

- **InventoryRecord**
  - `legacy_path`
  - `kind` (`pcc`, `lst`, `doc`, `loader-surface`)
  - `pilot_relevance`
  - `object_classes`
  - `include_edges`
  - `evidence_status`

- **TokenFamilyRecord**
  - `legacy_family`
  - `object_class`
  - `meaning`
  - `pilot_criticality`
  - `semantic_risk`
  - `downstream_owner`

- **MatrixRow**
  - `legacy_source`
  - `legacy_meaning`
  - `codex_target_concept`
  - `support_disposition`
  - `lossiness_class`
  - `provenance_requirement`
  - `validation_requirement`
  - `notes`

- **LedgerEntry**
  - `legacy_construct`
  - `source_reference`
  - `reason`
  - `severity`
  - `owner`
  - `mitigation`
  - `review_state`

- **OracleSurface**
  - `surface_name`
  - `entry_mode`
  - `evidence_type`
  - `automation_level`
  - `limitations`
  - `trust_level`

## External dependencies and references
- `programs/codex/plans/spec-domains/GE-01-legacy-corpus-and-conversion-matrix.md` — strategic source artifact
- `programs/codex/requirements/GE-00-program-governance-and-scope/technical-requirements.md` — inherited governance rules
- `programs/codex/research/pcgen-port-findings-2026-06-17.md` — PCGen-as-oracle migration posture
- `programs/codex/research/codex-reference-architecture-2026-06-17.md` — provenance, parser pipeline, matrix, ledger, and validation architecture
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc` — grounded pilot PCC reference
- `/home/ubuntu/workspace/repos/pcgen/docs/listfilepages/listfileimportanttoknow.html` — token-family reference surface
- `/home/ubuntu/workspace/repos/pcgen/code/src/java/pcgen/persistence/lst/GenericLoader.java` — loader-behavior reference surface
- `/home/ubuntu/workspace/repos/pcgen/code/src/java/pcgen/persistence/lst/CampaignSourceEntry.java` — campaign include/reference surface

## Design decisions already fixed
- Codex replaces legacy project naming in the new authority surface.
- The legacy PCGen repo is read-only reference material.
- GE-01 is a source STC, not an execution handoff.
- The rich STC bundle is mandatory here: requirements, design, acceptance, risks, and epic breakdown remain separate.
- The conversion matrix and unsupported-token ledger are first-class control-plane artifacts, not side comments.

## Deferred design decisions
- exact branch/worktree choice and write scope for the future code-authorizing handoff
- final canonical content schema and storage representation
- exact parser and AST implementation details
- exact expression-language choice for formulas and prerequisites
- exact automation mechanism for legacy oracle comparison
- exact source-span precision achievable in the first parser milestone

## Failure modes and observability
- **Failure mode:** token families are inventoried too broadly and the pilot explodes in scope.
  - **Required signal:** every taxonomy entry carries pilot criticality and downstream owner.
- **Failure mode:** unsupported behavior disappears into prose.
  - **Required signal:** every unresolved construct has either a matrix row or a ledger row.
- **Failure mode:** provenance is hand-waved until debugging becomes impossible.
  - **Required signal:** provenance obligations are explicit before parser implementation begins.
- **Failure mode:** Codex implementation begins from this STC without grounded code-authorizing handoff facts.
  - **Required signal:** no code-authorizing implementation handoff exists until local repo/workdir, branch or worktree, and write scope are verified; planning-only bridges must remain explicitly non-authorizing.
- **Failure mode:** parity claims are made on vibes.
  - **Required signal:** acceptance rules require named evidence surfaces and matrix coverage.

## Verification implications
`acceptance-and-verification.md` must prove that this design yields:

- a complete source-STC bundle
- explicit pilot inventory and taxonomy expectations
- a non-optional conversion matrix and unsupported-token ledger
- a provenance model strong enough for later debugging and comparison
- a named oracle-discovery surface for downstream validation work
- an implementation decomposition that does not invent missing runtime facts

## Change constraints
- Do not assume the Codex repo layout beyond the currently grounded checkout without an explicit execution handoff.
- Do not collapse requirements into design or design into execution handoff.
- Do not rewrite legacy source truth to fit a preferred future architecture.
- Do not allow matrix or ledger structure to drift into ad hoc free-form notes.


## Closure Addendum — 2026-06-19

The documentary control-plane design is accepted for the GE-01 pilot closure boundary. The artifacts now provide the minimum governed substrate needed for downstream GE-02 modeling and later GE-03 parser planning without inventing missing pilot facts.

Design limits remain explicit:
- this bundle does not define the final canonical Codex rules model;
- this bundle does not implement an importer;
- this bundle does not implement the oracle harness;
- this bundle does not authorize coding work without a later coding-route handoff.


## Design Addendum — 2026-08-02 — `wiring_class`, a second axis on every corpus unit

**Decision.** GE-01 owns a second classification axis, `wiring_class`, orthogonal to the work-inventory `status` axis. It states what *kind of evidence* would prove a unit done, and it is determined mechanically from the PCGen record with no per-unit human judgement. The normative definition and the determination rules live in `artifacts/wiring-class-determination.md`, with a dependency-free reference determinator alongside it at `artifacts/wiring-class-determination.py`. Downstream packages cite that artifact; none of them restates it.

**Why it belongs to GE-01.** The class is a property of the *legacy record*, derived from its token shape, exactly like the token taxonomy and the conversion matrix already in this bundle's artifacts. GE-02 carries the resulting field on the canonical object; GE-04 owns the evaluator one class needs; GE-09 owns aggregation and audit.

**The classes.** `display` (no magnitude token), `static` (literal constants only), `derived` (a deterministic function of a character or item scalar), `computed` (conditional guard, temporary effect, or player choice), plus `ambiguous` for determination failure. Strict lattice, highest bar wins; a unit's full signal set is retained in `wiring_class_signals` so dual-class records stay legible.

**Correction to the framing this addendum answers.** The proposal that prompted it named three kinds. The corpus partitions into four: 64.2% of the 4,050 stalled `ingested-magnitude` units are **static datum** — an item's `COST:`/`WT:`, a constant on its own row — which is neither bespoke-wiring work nor formula evaluation. Recorded here rather than only in the artifact, because a three-way split is the shape a future reader would otherwise assume.

**Determination gaps this exposed in the current generator, both real and both in `MAGNITUDE_TOKENS`'s blind spot.** PCGen carries scaling magnitudes in places the magnitude-token list does not look: as parenthesised expressions inside `DESC:`/`DURATION:` (`(min(10,CASTERLEVEL))d6` on *Fireball*), and as the keyword ranges `Close`/`Medium`/`Long`, which are caster-level functions (474 of 1,067 stalled spells). A determinator that scans only the magnitude tokens misses every scaling spell in the corpus.

**Constraint on any implementation.** The determinator MUST read `MAGNITUDE_TOKENS` from the work-inventory generator rather than forking the list. Two copies would drift and then disagree about which units have a magnitude at all.

**Evidence.** All figures re-derived 2026-08-02 from `docs/work-inventory.json` (`generated_at 2026-08-02T04:02:12Z`) and the PCGen corpus tree; every command is recorded inline in `artifacts/wiring-class-determination.md`.
