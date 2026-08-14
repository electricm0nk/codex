---
canonical: true
owner: god-emporer
status: planning-ready (operator directive 2026-08-11)
date: 2026-08-11
canonical_branch: tranche/11
---

# SD-33 Forward-Scope Register

Work this bundle deliberately does not do. Entries here are **candidates**, not claims on any
successor — the "scope takeover" failure SD-27's register warns about by name.

## Deferred at authoring time

| # | Item | Why deferred | Natural owner |
|---|---|---|---|
| 1 | **Export to `.pcg`** | Import proves the format model first; export needs a faithful serializer and a round-trip oracle | A successor bundle, once §1's parser is proven |
| 2 | **`.pcp` party/campaign files** | Multi-character container; single characters are the useful unit | Successor |
| 3 | **Non-PF1e game modes** | Codex's engine is PF1e; parsing other modes would produce characters nothing can compute | Out of product scope until the engine widens |
| 4 | **PCGen templates** (`decisions.md §8`) | Codex has no template model | Engine lane |
| 5 | **Known-inert template allowlist** | Would let both fixtures import without acknowledgement; needs a ruling that the two fixture values are genuinely inert | Operator question 3 |
| 6 | **Wider fixture coverage** — multiclass, non-Human, high-level, non-CRB | `TR-31-007` forbids hand-authoring; needs real files generated from the PCGen checkout | Operator question 1 |
| 7 | **Temporary bonuses / `USETEMPMODS`** | The fixture's section is empty; no observed instance to model against | Successor, when a fixture carries one |
| 8 | **Deity/domain, spell selections** | Empty in the Fighter fixture; the Wizard fixture will exercise spells and may open this | Epic 4 may partially close it |

## Routed elsewhere — do not absorb

**Un-ingested corpus content is never SD-33's.** Every `UnresolvedReason::RecordNotIngested`
an import produces names content that SD-29 (all kinds but `class_feature`) or SD-30
(`class_feature`) owns. SD-33 reports these and stops.

A cycle that ingests a record to make an import succeed has violated `TR-31-001` and taken over
another bundle's scope, regardless of how small the record is.

## A note for whoever reads this next

Two entries in this register (5 and 6) are blocked on operator questions rather than on work.
If those questions are answered, they become cheap. They are listed as deferred rather than
open so that a successor does not mistake an unanswered question for undiscovered work.
