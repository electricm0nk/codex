---
title: GE-08 Package File Lifecycle Requirements
stc_id: STC-CODEX-GE-08
artifact_type: generated-artifact
status: draft
scope: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts
source_stc: ../README.md
related:
  - ./ge08-e1-minimum-proof-object-selection-2026-06-22.md
  - ./homebrew-authoring-surface-specification.md
  - ../technical-requirements.md
  - ../technical-design.md
  - ../../GE-02-canonical-rules-model-and-content-packages/artifacts/content-package-layout-specification.md
  - ../../GE-02-canonical-rules-model-and-content-packages/artifacts/provenance-source-map-specification.md
---

# Package File Lifecycle Requirements

## Core decision
The first GE-08 proof package is a deterministic directory-backed YAML source bundle, not an opaque database, not one giant text blob, and not a runtime cache pretending to be authored content.

That decision is the point of this artifact. The first proof object already exists conceptually from GE08-E1. What remained ambiguous was the package substrate. The answer is now fixed:
- source authority lives in human-authored YAML files grouped by canonical section
- package-local stable IDs survive create/edit/save/load/diff/export cycles
- validation state and diagnostics travel with the source bundle as structured data
- compiled/runtime artifacts are derived only and must never become the source of truth
- export produces the same logical authored bundle shape that local save/load uses, minus any local-only cache material

## Scope of this lifecycle
This artifact defines the lifecycle for the first truthful GE-08 authored package only:
- one authored `SourcePackage`
- one authored `Feat`
- one authored `Effect`
- optional one authored `Prerequisite`
- provenance and diagnostics sufficient to validate, preview, explain, diff, and exchange that package honestly

It does not authorize broader package registry policy, plugin ABI work, arbitrary scripting, or product-visible editor UX.

## Package root layout
The first proof package MUST use a directory root with deterministic logical sections aligned to GE-02.

```text
<package-root>/
  manifest.yaml
  objects/
    feats/
      <feat-stable-id>.yaml
  rules/
    effects/
      <effect-stable-id>.yaml
    prerequisites/
      <prerequisite-stable-id>.yaml        # optional
  metadata/
    provenance.yaml
    diagnostics.yaml
```

### Layout rules
- `manifest.yaml` is required.
- `objects/feats/` is required for the first proof package, because the selected proof object is feat-like.
- `rules/effects/` is required for the first proof package, because the authored feat must contribute one bounded armor-class modifier through an explicit effect record.
- `rules/prerequisites/` is optional only when no prerequisite parity is represented. If prerequisite parity is claimed, the prerequisite record MUST live here as structured source.
- `metadata/provenance.yaml` is required.
- `metadata/diagnostics.yaml` is required even when empty, because diagnostics are first-class package state.
- no `compiled_ir/`, cache directory, binary blob, or preview receipt may be treated as authored-source authority

## Concrete first-proof package shape
The first proof package SHOULD be documentary-compatible with the GE08-E1 selected package identity and bounded pilot case.

Illustrative package root:

```text
pf1.homebrew.proof.guard-stance/
  manifest.yaml
  objects/
    feats/
      feat.homebrew.guard_stance.yaml
  rules/
    effects/
      effect.homebrew.guard_stance.ac_bonus.yaml
    prerequisites/
      prerequisite.homebrew.guard_stance.dex13.yaml   # optional
  metadata/
    provenance.yaml
    diagnostics.yaml
```

The illustrative stable IDs above are documentary examples, not a new authority surface for exact string syntax. What is normative is the boundary:
- package ID is package-scoped and durable
- object/effect/prerequisite IDs are stable within that package
- the feat record references the effect record by stable ID
- any prerequisite parity is structured and referenced, not inlined as free-form prose

## Manifest schema
`manifest.yaml` MUST contain, at minimum:

| Field | Required meaning |
|---|---|
| `schema_version` | Version of the authored package file contract itself. |
| `package_id` | Stable package identity independent of local paths. |
| `package_title` | Human-facing package name. |
| `game_system_id` | Target rules system identity, e.g. `pf1`. |
| `package_version` | Author-facing content version/release identifier. |
| `depends_on` | Required dependency list; for the first proof package this includes the PF1 CRB package authority. |
| `supported_object_kinds` | Declared authored object kinds present in the package, initially `Feat` and `Effect`, with `Prerequisite` only if used. |
| `validation_state` | Current normalized package state such as `draft`, `valid`, `invalid`, or `deferred`. |
| `provenance_policy` | Declares that authored records must emit structured package-local provenance and must not rely on absolute local paths. |
| `proof_binding` | Records the bounded intended proof target, including the GE-06-derived case and the Human bonus feat substitution posture. |
|
The manifest MUST NOT embed compiled preview output, desktop-shell state, or opaque local cache material.

## Record-shape rules

### Feat record
The first-proof feat file MUST carry:
- stable feat ID
- package-local ownership reference
- display/name fields
- object kind explicitly identifying it as a feat-like authored object
- references to attached effect IDs
- references to prerequisite IDs if present
- bounded semantic intent sufficient to identify that it substitutes for `Dodge` in the targeted proof case

### Effect record
The first-proof effect file MUST carry:
- stable effect ID
- owning feat reference
- typed target showing that the affected derived family is armor class
- bounded modifier payload sufficient to express the one conservative proof contribution
- stacking or combination posture only if required by the chosen semantics
- no arbitrary script body

### Prerequisite record
If prerequisite parity is represented, the prerequisite file MUST carry:
- stable prerequisite ID
- owning feat reference
- structured predicate meaning equivalent to the bounded requirement, such as `DEX >= 13`
- no free-form text as the only semantic carrier

## Versioning boundaries
The first package lifecycle MUST distinguish three different version surfaces.

### 1. `schema_version`
`schema_version` changes only when the package file contract changes.

It MUST NOT change merely because:
- a feat display name changed
- an effect value changed
- diagnostics changed
- the package was saved again on another machine

### 2. `package_version`
`package_version` changes when the authored content version meaningfully changes.

It SHOULD change when:
- exported content semantics change
- a collaborator needs a new portable revision
- an accepted edit changes the authored feat/effect/prerequisite meaning

It MUST NOT be forced to change on every local save.

### 3. Stable IDs
Stable IDs are not version numbers. They identify package-local records across edits.

A stable ID MUST survive:
- field edits
- save/reload cycles
- diff/review cycles
- export/import on another machine

A stable ID MUST NOT be regenerated just because a file moved, a title changed, or the package was re-exported.

## Provenance expectations
GE-02 provenance obligations still apply even though this package is natively authored rather than imported from PCGen.

`metadata/provenance.yaml` MUST record, at minimum:
- `source_package_id`
- `canonical_target_id`
- `canonical_target_field` or equivalent target surface
- authored file path relative to package root
- structured source location within the authored file when available
- support disposition and lossiness posture when any authored construct is deferred or narrowed
- linked diagnostic references when the authored record is blocked, unsupported, or partially represented

For native-authored records:
- the source-system identity SHOULD be `codex-authored` or equivalent
- legacy `pcc_path` / `lst_path` fields are absent unless the authored record intentionally cites legacy evidence for comparison
- absolute local filesystem paths MUST NOT be required for provenance to remain meaningful after export

The package dependency edge to the PF1 CRB package MUST remain explicit in the manifest rather than being inferred from hidden workspace state.

## Deterministic serialization rules
Diffability is a first-order requirement, not an aesthetic preference.

The package source form MUST therefore be serialized deterministically:
- stable file paths are derived from section + stable ID
- map/object keys use a canonical order
- record lists use deterministic ordering by stable ID or accepted equivalent rule
- save operations MUST NOT inject save-time-only noise such as machine names, temp paths, or wall-clock timestamps into authored source files
- line endings and encoding MUST be normalized so a cross-machine export does not produce counterfeit diffs

## Lifecycle states
The first proof package moves through a small honest state machine.

| State | Meaning | Preview eligible | Export eligible |
|---|---|---|---|
| `draft` | Package shell or edited source exists but has not yet passed required validation. | No | No |
| `valid` | Required files exist, references resolve, and no claim-blocking diagnostics remain. | Yes | Yes |
| `invalid` | Required structure is broken or claim-blocking diagnostics exist. | No | No |
| `deferred` | Package is structurally understood but still carries accepted unsupported/deferred semantics. | No for proof claims | No |

`validation_state` in the manifest is the package-wide summary. Detailed reasons live in `metadata/diagnostics.yaml`.

## Lifecycle behavior

### Create
Create has two honest outcomes.

#### A. Package shell creation
A new package may be created as a shell containing:
- `manifest.yaml`
- required directories
- empty `metadata/provenance.yaml`
- empty `metadata/diagnostics.yaml`

A shell package is saveable as `draft`, but it is not previewable, importable as a proof artifact, or exportable as a portable proof package.

#### B. Proof-complete draft creation
A package becomes a proof-complete draft when it contains:
- one authored feat record
- one authored effect record
- provenance entries for each authored record
- optional prerequisite record if prerequisite parity is claimed
- manifest dependency on the PF1 CRB package
- proof-binding metadata naming the bounded GE-06-derived case

At this point the package may be validated. It is still not previewable or exportable until validation succeeds.

### Edit
Editing MUST preserve package identity and stable IDs unless the user intentionally creates a new record.

Edits MUST follow these rules:
- changing a display field does not regenerate the record ID
- changing an effect payload does not regenerate the owning feat ID
- removing a referenced record without repairing references MUST surface diagnostics rather than silently rewriting the graph
- unsupported semantics MUST be preserved as explicit diagnostic-bearing debt rather than dropped on save
- the package MUST remain interpretable even if edited into an invalid state

### Save
Save is a source-authority action, not a preview action.

On save, the package system MUST:
1. write the authored YAML files in deterministic form
2. recompute validation state
3. rewrite `metadata/diagnostics.yaml` to reflect current normalized diagnostics
4. update manifest `validation_state`
5. preserve authored provenance rather than regenerating unstable identities

Save MAY persist an invalid package as a local draft, because refusing to save invalid source would hide work-in-progress. But an invalid saved package MUST remain visibly invalid and non-exportable.

### Load / reload
Load rehydrates the authored source bundle from disk.

On load, the package system MUST:
1. read `manifest.yaml` first
2. check `schema_version` compatibility
3. load the required logical sections
4. validate cross-file references and required fields
5. restore manifest `validation_state` only as a hint, then recompute actual current validation truth
6. refuse to treat any compiled/runtime artifact as source authority even if such material exists nearby

If required files are missing, references are broken, or schema compatibility fails, load MUST produce structured diagnostics rather than inventing defaults silently.

### Diff / review
Diff operates on the authored source bundle.

The canonical review surface for the first proof package is the set of authored YAML files under the package root. Diff MUST therefore:
- compare source files, not derived caches
- remain stable under no-op save/reload cycles
- show stable-ID continuity across edits
- make dependency, provenance, and diagnostic changes reviewable in source form

A package that cannot produce a stable source diff has not satisfied the GE-08 proof boundary.

### Export
Export produces a portable authored package bundle for another workspace or reviewer.

Export MUST:
- preserve the same logical source layout as the local authored package
- include manifest, authored object files, provenance, and diagnostics
- exclude compiled/runtime cache material and machine-local-only state
- normalize paths so the bundle can be reloaded elsewhere without absolute path dependency

Export MUST be refused when:
- manifest-required fields are missing
- stable IDs or references are broken
- claim-blocking diagnostics remain
- schema-version compatibility cannot be stated honestly

### Import
Import is distinct from ordinary reload.

- reload = reopen the same local authored source bundle
- import = accept an exported bundle from another location or actor into local authored-package storage

Import MUST:
- validate the incoming bundle as if it were freshly loaded
- preserve source stable IDs unless the operator explicitly chooses to fork the package under a new package ID
- keep provenance meaningful after relocation
- refuse silent merge with an existing package carrying the same package ID but incompatible content/version state

## Negative-case obligations
The lifecycle is incomplete unless it names counterfeit-success refusal cases.

Minimum blocking cases:
- missing `manifest.yaml`
- duplicate stable IDs
- effect record present but unreferenced or feat reference broken
- prerequisite parity claimed but stored only as free-form prose
- provenance entries missing for authored feat/effect records
- diagnostics file omitted or malformed
- export attempt while manifest state is `invalid` or `deferred`

## What is portable now versus deferred
Portable now:
- the authored package source tree above
- package identity and dependency edge
- one bounded feat-like record with one bounded effect
- optional simple structured prerequisite
- machine-readable diagnostics and provenance
- deterministic diff/review surface

Explicitly deferred:
- public package distribution/trust policy
- registry signatures or marketplace metadata
- plugin-backed behavior as ordinary authoring
- final editor UX flows or desktop-shell storage policy
- broad multi-object package composition beyond what this first proof requires
- runtime cache format as an exchange surface

## Documentary route consequence
This artifact is documentary-only. It does not authorize repo edits by itself.

It does, however, remove the schema/lifecycle ambiguity honestly enough that a later bounded execution handoff could target a narrow headless implementation slice instead of another planning loop.

If that later handoff is created, the narrowest honest initial repo surface is:
- `/home/ubuntu/workspace/repos/codex/src/homebrew_authoring/mod.rs`
- `/home/ubuntu/workspace/repos/codex/src/homebrew_authoring/package_manifest.rs`
- `/home/ubuntu/workspace/repos/codex/src/homebrew_authoring/package_store.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge08_package_file_lifecycle.rs`
- `/home/ubuntu/workspace/repos/codex/tests/fixtures/ge08/guard-stance-package/`

Suggested verification commands for that later headless slice:
- `cargo test ge08_package_file_lifecycle -- --nocapture`
- `cargo test ge08_guard_stance_package_round_trip -- --nocapture`

Non-goals for that later slice:
- no GE-07 product-visible editor work
- no plugin runtime
- no broad formula authoring
- no public registry/distribution flow
- no preview/explanation execution beyond proving the package can be created, saved, loaded, diffed, and exported honestly

## Final rule
The lesser approach would have left "package format TBD" and called it prudence.

That would be delay disguised as caution. The first proof package is now fixed as a deterministic YAML source bundle with explicit manifest, feat/effect records, provenance, diagnostics, and a refusal-first lifecycle. Later implementation may proceed narrowly from that truth or not at all.