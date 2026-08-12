---
title: PCGen Corpus Ingestion Follow-up — Tranche 2.7
status: active
scope: programs/codex
artifact_type: epic-tranche-source-stc
date: 2026-07-10
operator: Todd Hintzmann
parent_tranche: tranche-2.7
child_spec_domains:
  - SD-17-pcgen-corpus-include-graph-resolution
  - (future SDs may extend this list as the corpus scope grows)
doctrine:
  - programs/codex/doctrine/support-state-vocabulary.md
  - programs/codex/doctrine/documentation-control-plane.md
related:
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md
  - programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/
  - programs/codex/assumptions/tranche-3-starting-assumptions-2026-07-10.md
---

# Tranche 2.7 — PCGen Corpus Ingestion Follow-up

## Objective

Make the full PCGen corpus (specifically the Pathfinder 1e Core Rulebook subset at `pathfinder/paizo/roleplaying_game/core_essentials/`, and the surrounding source-derived LST corpus at `pathfinder/paizo/roleplaying_game/`) reachable through the codex rules engine by completing the three slices the GE-03 ingestion pipeline was scoped for but did not deliver.

## Background and framing

GE-03 (pcgen-import-pipeline-and-provenance) shipped one slice of the ingestion pipeline: the PCC entry-file parser (`src/pcgen_import/pcc.rs`). It proves the parser shape — `PCC:@target` directives are recognized as include edges, line numbers are preserved, malformed directives produce structured diagnostics. It does **not** prove that the full corpus can flow into the rules engine.

Tranche-1 closed on the GE-03 PCC parser alone. Tranche-2 was supposed to expand breadth across the core races and classes; it did not, because the ingestion pipeline that would have fed that breadth work was incomplete. The PCC parser alone is one slice deep. The remaining slices are the missing pieces.

This tranche owns those remaining slices. It does not own content breadth — that remains SD-13 / `codex-tranche-2-6`. It does not own release engineering — that closed with tranche-2.5 / SD-16. It does not own UI — the operator is building UI directly per `assumptions/tranche-3-starting-assumptions-2026-07-10.md`.

## Scope

Three sequential slices, in this order:

1. **Include-graph resolution.** Walk `PCC:@target` directives recursively from a PCC entry-file to assemble the full flat include set. The result is a deterministic list of LST files reachable from the corpus root.
2. **LST object-declaration parsing.** Parse each LST file's object declarations (`CLASS:`, `RACE:`, `SPELL:`, `EQUIP:`, and related object kinds) into structured records. This is the second half of "ingest a real PCC."
3. **Canonical-IR conversion.** Transform parsed LST records into the rules engine's canonical internal representation. This is what feeds the rules-core compute path and what the SD-13 slice workers ultimately consume.

## Why three slices in order

- **A without B:** you resolve the include graph but cannot read the LST files you discover.
- **B without C:** you parse LSTs but the engine has no semantic content to consume.
- **C without A or B:** you have a converter but nothing to convert.

A is the floor. C is the ceiling. B is the bridge.

## Concurrency with `codex-tranche-2-6`

The operator-driven claude-code loop on `codex-tranche-2-6` continues in parallel with tranche-2-7 work. Two concurrency rules apply:

1. **Tranche-2-7 reads but does not write the SD-13 matrix.** The matrix (`programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`) is read-only by design. Tranche-2-7 evidence about corpus reachability flows through its own artifacts, not through matrix edits.
2. **Tranche-2-7's slice C may shift the canonical IR shape.** When slice C lands, the canonical IR for the rules engine may change. Slice workers on `codex-tranche-2-6` that depend on the IR should name the IR version they consume in their handoff. If the IR version changes during the parallel phase, those handoffs need to be re-baselined before the slice is merged.

The canonical IR is owned by GE-04 (rules-engine-and-explainability-core). Tranche-2-7's slice C authors IR conversion; the IR shape itself is a GE-04 contract. Slice C produces a write-up of the IR shape as a contract artifact under `SD-17/artifacts/canonical-ir-contract.md` so future work has a single reference.

## Authority and ownership

| Slice | Owner | Notes |
|---|---|---|
| A. Include-graph resolution | tech-priest (Claude Code) | Bounded parser extension under `src/pcgen_import/`. |
| B. LST object-declaration parsing | tech-priest (Claude Code) | Bounded parser extension under `src/pcgen_import/`. New test fixtures from the corpus. |
| C. Canonical-IR conversion | tech-priest (Claude Code) | Bridges parsed LST records into GE-04's IR shape. Authoring the IR contract is C's first deliverable. |

This tranche does not own code authorship for the IR shape itself; that belongs to GE-04. Slice C produces a converter from parsed LST records into whatever IR shape GE-04 has published.

## Non-goals

- **No content breadth.** Tranche-2-7 does not lift any SD-13 matrix row. It enables SD-13's slice workers to consume the corpus; it does not do the consumption itself.
- **No UI.** The operator is building UI directly per the assumptions document. Tranche-2-7 does not touch UI surfaces.
- **No release engineering.** Tranche-2-7 does not invoke the publish lane. Tranche-2-7 slices ship to `develop` via PR per the codex workflow.
- **No PCC parser rewrite.** GE-03's PCC parser is the trunk-side authority. Tranche-2-7 extends it but does not replace it.
- **No IR shape redesign.** GE-04 owns the canonical IR shape. Tranche-2-7 slice C authors a converter from parsed LST records to the existing IR shape.

## Acceptance

Tranche-2-7 closes when SD-17 is fully landed: Slices A, B (1–6), C, D, and E all merged to `tranche/2-7`, every acceptance gate green, and the canonical source-IR shape lives at the parser/rules-core boundary as a first-class surface that tranche-3 can immediately consume.

Specifically:

1. Slice A lands: every PCGen PCC entry-file under the test corpus resolves to a deterministic flat include set of LST files.
2. Slice B lands: every LST file in the resolved set parses into structured records for every object kind the LST grammar supports (all six B-family parsers).
3. Slice C lands: the LST-to-IR converter exists and projects every parsed record into a structured form.
4. Slice D lands: the unified record-aggregate enum lives at the parser surface, not inside the converter.
5. Slice E lands: the canonical source-IR types live under `src/rules_core/source_content.rs` (not in the converter), and the converter projects into the canonical types — closing the rule that corpus-derived records have a stable canonical home before any consumer is built.
6. Every card above references no closed spec domain as a producer or publisher of artifacts; the canonical home is owned by tranche-2-7 as the active collecting tranche.
7. The parser, resolver, and converter do not regress the GE-03 PCC parser's existing test suite.
8. Concurrency with `codex-tranche-2-6` is preserved. If the canonical source-IR shape changes during the parallel phase, affected handoffs on `codex-tranche-2-6` are re-baselined.

What is **NOT** in scope for tranche-2-7 closure (explicitly deferred to the consumer):

- Composing `SourcePackageContent` with the existing chosen-state `CharacterInput` into a single input the rules engine evaluates against. This is a consumer-side decision and belongs to whichever active tranche consumes the corpus-side records.

## Definition of Done for SD-17

SD-17 closes when tranche-3 can begin execution with the full ability to import LST files end-to-end. Specifically: a tranche-3 worker can hand a PCC entry-file path to the ingest pipeline, receive a `SourcePackageContent` back, and feed that record set (together with a chosen `CharacterInput`) into the rules compute path without further SD-17 work. Until SD17-D and SD17-E land on `tranche/2-7`, that path is not exercisable, and tranche-3 execution must wait on tranche-2-7 close.

## Scope discipline

Per operator directive (2026-07-12): no slice's scope is narrowed on size-of-work grounds. If a slice's full scope is too big to fit in one slice, the right move is more slices, not a narrower scope. Narrowing is permitted only when the initial slice's output will inform the second slice's scope.

The three slices' full scopes cover the entire PF1 corpus: every PCC entry-file for Slice A, every LST object kind for Slice B, every parsed record for Slice C. The source STC bundles under `programs/codex/requirements/SD-17-pcgen-corpus-include-graph-resolution/` document the full scope per slice.

## Branch shape (operator doctrine, 2026-07-12)

Slice branches target the **tranche branch** (`tranche/2-7`), not `develop` directly. The promotion chain is:

```
slice/*  ── PR ──▶  tranche/2-7  ── PR (operator) ──▶  develop
```

Specifically:

- **`tranche/2-7`** is the slice-merge target. It is created from `develop` once at the start of the tranche and lives for the tranche's lifetime.
- **Slice PRs target `tranche/2-7`**, never `develop`. Per `devops/tranche-branch-governance`, this is the slice-PR ownership split.
- **The operator opens the `tranche/2-7 -> develop` promotion PR.** It is operator-driven, not worker-driven, because it is the promotion that crosses the governance boundary from slice-merge territory into the global develop branch.
- **`tranche/2-7` has no PR review requirement** per operator directive (matches `tranche/3`'s configuration). Workers commit directly; the operator reviews the cumulative tranche-level promotion to develop.
- **Do not push slice branches to `develop` directly.** The first slice of tranche-2-6 (and similar) that did so created a doctrine violation; the OPS card `t_230c4a14` on `codex-tranche-2-7` is the explicit procedure to recover.

If you find yourself minting slice cards whose bodies say `branch: develop`, that is a flag: the tranche branch has not been created or you forgot to update the body. The first action is to confirm `tranche/2-7` exists on origin; the second is to update the body before any worker dispatches.

## See also

- `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/` — the content-breadth epic this tranche enables.
- `programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/` — the ingestion umbrella this tranche extends.
- `programs/codex/doctrine/support-state-vocabulary.md` — the two-axis vocabulary that tranche-2-7's own slice handoffs use to describe their state.
- `programs/codex/assumptions/tranche-3-starting-assumptions-2026-07-10.md` — the operator's starting assumptions for tranche-3.