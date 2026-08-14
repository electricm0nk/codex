---
canonical: true
owner: god-emporer
status: planning-ready (operator directive 2026-08-11)
date: 2026-08-11
canonical_branch: tranche/11
mirror_of: ./README.md
---

# SD-31 Scope Draft

## The shape

One capability, three layers, one verification spine.

**Capability:** open a PCGen `.pcg` character file and get that character in Codex — or a clear,
actionable explanation of why not.

**Layers** (`technical-design.md §2`): syntax → typed records → corpus references. Each fails
only in its own vocabulary.

**Spine** (`acceptance-and-verification.md §2`): the same `.pcg` run through real headless PCGen
and diffed dimension-by-dimension against Codex's recomputation.

## In scope

- `.pcg` parsing for `GAMEMODE:Pathfinder_RPG`.
- Reference resolution for race, class/level, stats, alignment, skills, feats (including
  `APPLIEDTO` parameters), languages, equipment and equipmods, and the `EQUIPSET` tree.
- A fidelity report distinguishing resolved / unresolved / exempt, with reasons.
- `import_pcgen_character` IPC command reusing the existing import invariants.
- Player surface: import affordance, mapping-review screen, acknowledgement path.
- Oracle parity for both vendored fixtures.

## Out of scope

- Export to `.pcg` (import only).
- `.pcp` party/campaign files.
- Non-PF1e game modes — refused with a named diagnostic, not partially parsed.
- PCGen templates (`§8`).
- **Any corpus widening.** If a `.pcg` names content Codex has not ingested, that is SD-29's or
  SD-30's lane. SD-31 reports it and defers it; it never ingests.

## The boundary that matters

SD-31 imports **inputs, never outputs** (`decisions.md §6`). The `.pcg` carries PCGen's computed
values; the importer reads them only to verify parity. Everything the user sees is recomputed by
the Codex engine.

Without that rule, an imported sheet's numbers come from PCGen and its edits come from Codex —
two engines disagreeing inside one character, discovered only after the user has trusted it.

## Relationship to the in-flight bundles

| Bundle | Branch | Owns | SD-31 overlap |
|---|---|---|---|
| SD-29 | `tranche/9` | every corpus kind except `class_feature` | none — `TR-31-001` |
| SD-30 | `tranche/10` | `class_feature` corpus-wide | none — `TR-31-001` |
| SD-31 | `tranche/11` | the `.pcg` import path | — |

SD-31 is a **consumer** of what SD-29 and SD-30 ingest: the more corpus they land, the fewer
`RecordNotIngested` entries an import produces. The dependency runs one way and requires no
coordination.
