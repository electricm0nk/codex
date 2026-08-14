---
canonical: true
owner: god-emporer
status: planning-ready (operator directive 2026-08-11)
date: 2026-08-11
canonical_branch: tranche/11
---

# SD-31 Decisions

## §1 — Bundle exists (operator directive, 2026-08-11)

The operator directed a PCGen character-sheet import capability as parallel work alongside
SD-29 (`tranche/9`) and SD-30 (`tranche/10`), both in flight.

## §2 — Branch and version

`tranche/11`, cut from `develop`. Build version target `0.11.<build>`, following the
tranche-digit convention. The digit bumps on the branch cut, not on closure.

## §3 — Dispatch

Local-file dispatch via `kanban.md` + `progress.md`. No Hermes board, consistent with SD-29 and
SD-30 (every hermes profile carries a standing daemon that auto-claims ready cards).

## §4 — Write partition is derived, not asserted

`TR-31-001` is the subtraction of `TR-29-001` and `TR-30-001` from the repo, and Epic 1 re-derives
it at cycle time rather than trusting this document. Both sibling bundles are live and their
partitions can widen. A partition inherited from a planning doc is exactly the kind of decayed
inventory this repo has been burned by before.

## §5 — New top-level module, not a `pcgen_import` submodule

`src/pcgen_character/` is separate from `src/pcgen_import/`. Two reasons, in priority order:

1. It keeps SD-31's edits out of a tree the sibling bundles read.
2. The concerns genuinely differ — `pcgen_import` ingests rules data at build time from a
   trusted checkout; `pcgen_character` reads a user's file at runtime, with untrusted input and
   user-facing errors.

Reuse is by idiom, not linkage. Sharing the tokenizer would save modest work and put SD-31
inside SD-29's read surface.

## §6 — Import inputs, never outputs

A `.pcg` carries PCGen's computed values (`HITPOINTS:10`, `SKILLSGAINED:3`). The importer reads
them **only** for parity verification. Nothing computed by PCGen reaches
`CreateCharacterRequest`; the Codex engine recomputes everything.

Copying computed values would produce a character whose current numbers came from one engine
and whose future edits come from another — a disagreement that surfaces only after the user has
already trusted the sheet. Enforced structurally by `AC-5`, not by review.

## §7 — Refuse rather than degrade

An import with mechanically significant unresolved references returns `Blocked` and persists
nothing, mirroring `create_character`'s existing invariant. Lossy import requires an explicit
acknowledgement that defaults false at every layer, and is only reachable after the fidelity
report has been displayed.

This is `no-stub-mvp-doctrine.md` applied to import: a character that claims to be your PCGen
character must actually be your PCGen character.

## §8 — `TEMPLATESAPPLIED` unsupported in v1

The vendored fixture carries two benign values (`Creature with Class Levels`, `Human`), but
templates can carry real mechanics and Codex has no template model. v1 declares the token kind
unsupported and names it in the fidelity report rather than special-casing the benign two and
silently accepting a dangerous third later. An allowlist for known-inert templates is forward
scope.

## §9 — The shared test-count baseline is not SD-31's

Three bundles editing one integer conflicts on every merge. SD-31 records per-cycle deltas in
receipts; the tranche-merge cycle reconciles once (`TR-31-003`).

## §10 — Oracle parity is the definition of done

Not "the tests pass" but "PCGen and Codex compute the same character from the same file," via
the existing `pcgen_runner` + `comparator` pair. Import defects are invisible by nature; the
oracle is the only instrument that catches a plausible-but-wrong mapping.

## §11 — Scope excludes export

Import only. `.pcg` export, `.pcp` party files, and non-PF1e game modes are out of scope and
recorded in the forward-scope register.
