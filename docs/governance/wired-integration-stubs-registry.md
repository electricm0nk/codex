---
title: Wired Integration Stubs Registry
stc_id: GOV-WIRED-INTEGRATION-STUBS-REGISTRY
canonical: true
owner: Todd Hintzmann
scope: universal
status: active
review_state: accepted
last_reviewed_at: 2026-07-22
canonical_source: ~/workspace/repos/codex/docs/governance/wired-integration-stubs-registry.md (this file)
workspace_citation: ~/workspace/governance/docs/wired-integration-stubs-registry.md
supersedes: (none — first issuance)
upstream_targets:
  - ./no-stub-mvp-doctrine.md (parent doctrine; in-repo)
  - ~/workspace/governance/doctrine/no-stub-mvp-doctrine.md (parent doctrine; workspace citation)
  - ~/workspace/governance/agents/CLAUDE.md
  - ~/workspace/governance/agents/AGENTS.md
related_artifacts:
  - ./no-stub-mvp-doctrine.md (parent doctrine; in-repo)
date: 2026-07-20
---

# Wired Integration Stubs Registry

The doctrine of record for any given stub. Per `no-stub-mvp-doctrine.md` §"Stubs are the exception, not the rule," no stub may ship without an entry here.

## How to use this registry

When a stub is proposed (by an operator directive, a planned cycle, or a defensive audit find), the cycle authoring it must:

1. Add a numbered entry below with all required fields filled.
2. Reference the entry from the bundle's `risks-and-open-questions.md` if the stub has a remediation cycle, or document the permanent-exception rationale inline.
3. If the bundle is closed without remediation, the entry remains here indefinitely as a permanent exception, and the audit grep remains scoped to exclude it via `epic-breakdown.md` §"Audit exclusions."

The operator's verbatim directive is required for every entry — exceptions are operator-granted, not self-asserted.

### The `book_stub` kind

Entries #0001 and #0002 are **code-pattern** stubs — a source-code location that returns a
placeholder/`Would ...` value instead of computing a real one. `book_stub` is a different shape:
a **data-completeness gap**, not a code stub. There is no source file "returning" a fake value;
there is a book of PF1 rules content that genuinely has not been ingested into the corpus yet,
and the registry's job is to record that gap honestly (per `book_id`) rather than let it be
silently missing or, worse, backfilled with fabricated data.

`book_stub` entries adapt the same seven-field shape #0001/#0002 use, with the two file-specific
fields swapped for data-specific equivalents:

| #0001/#0002 field | `book_stub` field | Meaning for `book_stub` |
|---|---|---|
| File / line | **Book / manifest path** | `<book_id>` plus its manifest at `data/stubs/<book_id>.json` |
| Stub pattern | **What's missing** | The book has no `data/corpus/<book_id>/` content at all — zero class/spell/equipment/monster records ingested. The manifest's `content_kind_counts: null` is the honest signal (not `0`, which would falsely claim the counting was done and came up empty) |
| Justification (operator verbatim) | *(same field, same requirement)* | Cites the operator directive that scopes future-state books out of this bundle |
| Audit-grep impact | *(same field)* | For `book_stub`, ordinarily "None" — the manifest JSON and registry prose don't contain the dual-audit's forbidden code tokens (`STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack`) or bundle-tag pattern, so no exclusion is needed. If a book_id or book_name ever collides with a forbidden token, note the exclusion explicitly here |
| Bundle-of-record | *(same field)* | SD-26, Epic 4, plus the criterion number that registered the book |
| Remediation cycle | *(same field)* | Repeats the manifest's own `planned_resolution_bundle` value, so the two stay in sync |
| Status | *(same field)* | `Registered stub <date>` (not `Accepted`, to distinguish a data gap from a granted code-stub exception — both are operator-granted, but `book_stub` entries have no code to "accept") |

Each `book_stub` entry's manifest (`data/stubs/<book_id>.json`) uses the shape specified in
`docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/content-unit-inventory.md §2.1`:
`{book_id, book_name, planned_resolution_bundle, content_kind_counts: null, registered_at: <ISO-8601>}`.

## Registry entries

### 0001 — Browser-preview fallback in character hub runtime

- **File / line:** `apps/desktop/src/characterHub/characterHubRuntime.ts:17-18`
- **Stub pattern:** Returns `buildPreviewListSurface()` (sample character) when `!hasTauriRuntime()`.
- **Justification (operator verbatim, 2026-07-20):** Browser preview needs a sample character so the Load → sheet flow stays walkable without the desktop backend. The fallback path is the test surface for mappers without a Tauri runtime; the production desktop path uses real `loadListSavedCharacters()`. Permanent exception.
- **Audit-grep impact:** The browser-preview branch's `return buildPreviewListSurface();` is permitted to remain in the diff indefinitely. No defensive cleanup cycle needed.
- **Bundle-of-record:** SD-23 (registry created with the bundle).
- **Remediation cycle:** None — permanent exception.
- **Status:** Accepted 2026-07-20.

### 0002 — `StubAdapter` future-rule-system placeholder

- **File / line:** `apps/desktop/src-tauri/src/stub_adapter.rs` (whole file — doc comment, the `would_render_message` builder, and every trait-method arm that surfaces it via a diagnostic/`error` field/`Err`). Widened 2026-07-21 (criterion 3.4) to also cover the three Tauri command-surface files that now dispatch through `StubAdapter` via `resolve_rule_system_adapter` and assert its exact message in their own tests: `apps/desktop/src-tauri/src/characterHub/appendToCharacter.rs`, `.../recomputeCharacter.rs`, `.../reSaveCharacter.rs` (each file's own `resolve_rule_system_adapter` fn doc comment plus their `*_via_rule_system_routes_unknown_id_to_stub_adapter` tests).
- **Stub pattern:** Every `RuleSystemAdapter` method on `StubAdapter` reports `"Would render for system {system_id}; not yet implemented"` (the wired-integration doctrine's forbidden "Would ..." pattern, matched by the dual-audit grep's `not yet implemented` bucket) instead of computing a real result, for any `rule_system_id` this codebase has not built a real adapter for yet.
- **Justification (operator-pinned, per `docs/release/SD-25-ui-evaluation-defect-closure/epic-breakdown.md` §Criterion 3.3 and `cycles/3_3.md`):** "returns 'Would render for system X; not yet implemented' results. Wired-integration doctrine forbids 'Would …' strings in *shipping code* — this stub gets an entry in `governance/wired-integration-stubs-registry.md` with the operator-granted justification (the future-system rollout is operator-pinned)." Criterion 3.4's Tauri command surface must have a `dyn RuleSystemAdapter` to hand back for a not-yet-built rule system's id rather than refuse to route at all; `StubAdapter` is that seam's honest placeholder until a real adapter for that system lands, at which point that system's real adapter replaces this dispatch entry — it never silently swaps in fabricated data. Criterion 3.4 (2026-07-21) is exactly this foreseen dispatch wiring: `append_to_character`/`recompute_character`/`re_save_character` each resolve an unrecognized `rule_system_id` to `StubAdapter` and their own tests assert its literal message to prove the routing is real, not stubbed out — the same governed stub, now genuinely reached from three new call sites rather than a second stub being created.
- **Audit-grep impact:** any `not yet implemented` / `Would` hit inside `stub_adapter.rs`, or inside the three command files' `resolve_rule_system_adapter` doc comments / `*_routes_unknown_id_to_stub_adapter` test bodies listed above, is permitted to remain in the diff. No defensive cleanup cycle needed for these files; the exclusion is scoped to this named set, not project-wide.
- **Bundle-of-record:** SD-25, Epic 3 "Character Hub as Hub of Hubs," criteria 3.3 (stub definition) and 3.4 (dispatch wiring, widened this entry rather than opening a new one — same stub, no new pattern).
- **Remediation cycle:** None per rule system that never gets a real adapter; superseded per-system the moment that system's real `RuleSystemAdapter` implementation lands (mirrors `Pf1Adapter`'s criterion 3.2 precedent) and criterion 3.4's `resolve_rule_system_adapter` in each command file routes that `rule_system_id` to the real implementation instead.
- **Status:** Accepted 2026-07-21.

### 0003 — `book_stub`: `advanced_race_guide` not yet ingested

- **Book / manifest path:** `advanced_race_guide` — `data/stubs/advanced_race_guide.json`
- **What's missing:** No `data/corpus/advanced_race_guide/` content exists. This PF1 sourcebook (Advanced Race Guide) has not been ingested into the corpus at all — zero class, spell, equipment, or other content-kind records. `content_kind_counts: null` in the manifest (not `0`) because no counting pass has run against this book; `null` means "not yet measured," `0` would falsely claim measurement happened and found nothing.
- **Justification (operator verbatim, 2026-07-21 17:39:26):** Per `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/README.md §3`, the operator directive establishes that the JSON cache split "honors the operator's 'in-scope books no stubs, future-state books knowingly stub' doctrine" — the 4 in-scope PF1 books (Core Rulebook, Advanced Player's Guide, Advanced Class Guide, Bestiary 1) get real JSON cache builds with no stubs (Epic 3), while the 21 future-state PF1 books, of which `advanced_race_guide` is the pilot instance, are registered as known, honest gaps rather than silently missing or fabricated (Epic 4). Same operator directive, same timestamp, establishes the scope-cross posture generally (`README.md §5`, "Why scope-cross").
- **Audit-grep impact:** None. `advanced_race_guide.json`'s field names and values, and this entry's prose, contain none of the dual-audit's forbidden code tokens (`STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack`) or the bundle-tag pattern. No exclusion needed.
- **Bundle-of-record:** SD-26, Epic 4 "Book Stub Manifest," criterion 4.1 (kind definition, this pilot entry) — the remaining 20 future-state books land under criteria 4.2-4.22, each opening its own numbered entry following this same shape.
- **Remediation cycle:** `SD-27+ (unscheduled)` — matches the manifest's own `planned_resolution_bundle` field. Per `risks-and-open-questions.md §5` ("Deferrals"): concrete rule-system implementations and further corpus ingest land in SD-27+; no specific bundle number is committed yet, so neither the manifest nor this entry fabricates one.
- **Status:** Registered stub 2026-07-22.

### 0004 — `book_stub`: `adventurers_guide` not yet ingested

- **Book / manifest path:** `adventurers_guide` — `data/stubs/adventurers_guide.json`
- **What's missing:** No `data/corpus/adventurers_guide/` content exists. This PF1 sourcebook (Adventurer's Guide, Paizo, 2017) has not been ingested into the corpus at all — zero class, spell, equipment, or other content-kind records. `content_kind_counts: null` in the manifest (not `0`) because no counting pass has run against this book; `null` means "not yet measured," `0` would falsely claim measurement happened and found nothing.
- **Justification (operator verbatim, 2026-07-21 17:39:26):** Per `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/README.md §3`, the operator directive establishes that the JSON cache split "honors the operator's 'in-scope books no stubs, future-state books knowingly stub' doctrine" — the 4 in-scope PF1 books (Core Rulebook, Advanced Player's Guide, Advanced Class Guide, Bestiary 1) get real JSON cache builds with no stubs (Epic 3), while the 21 future-state PF1 books, of which `adventurers_guide` is one, are registered as known, honest gaps rather than silently missing or fabricated (Epic 4). Same operator directive, same timestamp, establishes the scope-cross posture generally (`README.md §5`, "Why scope-cross").
- **Audit-grep impact:** None. `adventurers_guide.json`'s field names and values, and this entry's prose, contain none of the dual-audit's forbidden code tokens (`STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack`) or the bundle-tag pattern. No exclusion needed.
- **Bundle-of-record:** SD-26, Epic 4 "Book Stub Manifest," criterion 4.3.
- **Remediation cycle:** `SD-27+ (unscheduled)` — matches the manifest's own `planned_resolution_bundle` field. Per `risks-and-open-questions.md §5` ("Deferrals"): concrete rule-system implementations and further corpus ingest land in SD-27+; no specific bundle number is committed yet, so neither the manifest nor this entry fabricates one.
- **Status:** Registered stub 2026-07-23.

### 0005 — `book_stub`: `beginner_box` not yet ingested

- **Book / manifest path:** `beginner_box` — `data/stubs/beginner_box.json`
- **What's missing:** No `data/corpus/beginner_box/` content exists. This PF1 sourcebook (Beginner Box, Paizo, 2011) has not been ingested into the corpus at all — zero class, spell, equipment, or other content-kind records. `content_kind_counts: null` in the manifest (not `0`) because no counting pass has run against this book; `null` means "not yet measured," `0` would falsely claim measurement happened and found nothing.
- **Justification (operator verbatim, 2026-07-21 17:39:26):** Per `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/README.md §3`, the operator directive establishes that the JSON cache split "honors the operator's 'in-scope books no stubs, future-state books knowingly stub' doctrine" — the 4 in-scope PF1 books (Core Rulebook, Advanced Player's Guide, Advanced Class Guide, Bestiary 1) get real JSON cache builds with no stubs (Epic 3), while the 21 future-state PF1 books, of which `beginner_box` is one, are registered as known, honest gaps rather than silently missing or fabricated (Epic 4). Same operator directive, same timestamp, establishes the scope-cross posture generally (`README.md §5`, "Why scope-cross").
- **Audit-grep impact:** None. `beginner_box.json`'s field names and values, and this entry's prose, contain none of the dual-audit's forbidden code tokens (`STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack`) or the bundle-tag pattern. No exclusion needed.
- **Bundle-of-record:** SD-26, Epic 4 "Book Stub Manifest," criterion 4.2.
- **Remediation cycle:** `SD-27+ (unscheduled)` — matches the manifest's own `planned_resolution_bundle` field. Per `risks-and-open-questions.md §5` ("Deferrals"): concrete rule-system implementations and further corpus ingest land in SD-27+; no specific bundle number is committed yet, so neither the manifest nor this entry fabricates one.
- **Status:** Registered stub 2026-07-23.

### 0006 — `book_stub`: `bestiary_2` not yet ingested

- **Book / manifest path:** `bestiary_2` — `data/stubs/bestiary_2.json`
- **What's missing:** No `data/corpus/bestiary_2/` content exists. This PF1 sourcebook (Bestiary 2, Paizo, December 2010) has not been ingested into the corpus at all — zero class, spell, equipment, or other content-kind records. `content_kind_counts: null` in the manifest (not `0`) because no counting pass has run against this book; `null` means "not yet measured," `0` would falsely claim measurement happened and found nothing.
- **Justification (operator verbatim, 2026-07-21 17:39:26):** Per `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/README.md §3`, the operator directive establishes that the JSON cache split "honors the operator's 'in-scope books no stubs, future-state books knowingly stub' doctrine" — the 4 in-scope PF1 books (Core Rulebook, Advanced Player's Guide, Advanced Class Guide, Bestiary 1) get real JSON cache builds with no stubs (Epic 3), while the 21 future-state PF1 books, of which `bestiary_2` is one, are registered as known, honest gaps rather than silently missing or fabricated (Epic 4). Same operator directive, same timestamp, establishes the scope-cross posture generally (`README.md §5`, "Why scope-cross").
- **Audit-grep impact:** None. `bestiary_2.json`'s field names and values, and this entry's prose, contain none of the dual-audit's forbidden code tokens (`STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack`) or the bundle-tag pattern. No exclusion needed.
- **Bundle-of-record:** SD-26, Epic 4 "Book Stub Manifest," criterion 4.5.
- **Remediation cycle:** `SD-27+ (unscheduled)` — matches the manifest's own `planned_resolution_bundle` field. Per `risks-and-open-questions.md §5` ("Deferrals"): concrete rule-system implementations and further corpus ingest land in SD-27+; no specific bundle number is committed yet, so neither the manifest nor this entry fabricates one.
- **Status:** Registered stub 2026-07-23.

### 0007 — `book_stub`: `bestiary_3` not yet ingested

- **Book / manifest path:** `bestiary_3` — `data/stubs/bestiary_3.json`
- **What's missing:** No `data/corpus/bestiary_3/` content exists. This PF1 sourcebook (Bestiary 3, Paizo, January 2012) has not been ingested into the corpus at all — zero class, spell, equipment, or other content-kind records. `content_kind_counts: null` in the manifest (not `0`) because no counting pass has run against this book; `null` means "not yet measured," `0` would falsely claim measurement happened and found nothing.
- **Justification (operator verbatim, 2026-07-21 17:39:26):** Per `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/README.md §3`, the operator directive establishes that the JSON cache split "honors the operator's 'in-scope books no stubs, future-state books knowingly stub' doctrine" — the 4 in-scope PF1 books (Core Rulebook, Advanced Player's Guide, Advanced Class Guide, Bestiary 1) get real JSON cache builds with no stubs (Epic 3), while the 21 future-state PF1 books, of which `bestiary_3` is one, are registered as known, honest gaps rather than silently missing or fabricated (Epic 4). Same operator directive, same timestamp, establishes the scope-cross posture generally (`README.md §5`, "Why scope-cross").
- **Audit-grep impact:** None. `bestiary_3.json`'s field names and values, and this entry's prose, contain none of the dual-audit's forbidden code tokens (`STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack`) or the bundle-tag pattern. No exclusion needed.
- **Bundle-of-record:** SD-26, Epic 4 "Book Stub Manifest," criterion 4.4.
- **Remediation cycle:** `SD-27+ (unscheduled)` — matches the manifest's own `planned_resolution_bundle` field. Per `risks-and-open-questions.md §5` ("Deferrals"): concrete rule-system implementations and further corpus ingest land in SD-27+; no specific bundle number is committed yet, so neither the manifest nor this entry fabricates one.
- **Status:** Registered stub 2026-07-23.

### 0008 — `book_stub`: `bestiary_4` not yet ingested

- **Book / manifest path:** `bestiary_4` — `data/stubs/bestiary_4.json`
- **What's missing:** No `data/corpus/bestiary_4/` content exists. This PF1 sourcebook (Bestiary 4, Paizo, October 2013) has not been ingested into the corpus at all — zero class, spell, equipment, or other content-kind records. `content_kind_counts: null` in the manifest (not `0`) because no counting pass has run against this book; `null` means "not yet measured," `0` would falsely claim measurement happened and found nothing.
- **Justification (operator verbatim, 2026-07-21 17:39:26):** Per `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/README.md §3`, the operator directive establishes that the JSON cache split "honors the operator's 'in-scope books no stubs, future-state books knowingly stub' doctrine" — the 4 in-scope PF1 books (Core Rulebook, Advanced Player's Guide, Advanced Class Guide, Bestiary 1) get real JSON cache builds with no stubs (Epic 3), while the 21 future-state PF1 books, of which `bestiary_4` is one, are registered as known, honest gaps rather than silently missing or fabricated (Epic 4). Same operator directive, same timestamp, establishes the scope-cross posture generally (`README.md §5`, "Why scope-cross").
- **Audit-grep impact:** None. `bestiary_4.json`'s field names and values, and this entry's prose, contain none of the dual-audit's forbidden code tokens (`STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack`) or the bundle-tag pattern. No exclusion needed.
- **Bundle-of-record:** SD-26, Epic 4 "Book Stub Manifest," criterion 4.6.
- **Remediation cycle:** `SD-27+ (unscheduled)` — matches the manifest's own `planned_resolution_bundle` field. Per `risks-and-open-questions.md §5` ("Deferrals"): concrete rule-system implementations and further corpus ingest land in SD-27+; no specific bundle number is committed yet, so neither the manifest nor this entry fabricates one.
- **Status:** Registered stub 2026-07-23.

### 0009 — `book_stub`: `bestiary_5` not yet ingested

- **Book / manifest path:** `bestiary_5` — `data/stubs/bestiary_5.json`
- **What's missing:** No `data/corpus/bestiary_5/` content exists. This PF1 sourcebook (Bestiary 5, Paizo, December 2015) has not been ingested into the corpus at all — zero class, spell, equipment, or other content-kind records. `content_kind_counts: null` in the manifest (not `0`) because no counting pass has run against this book; `null` means "not yet measured," `0` would falsely claim measurement happened and found nothing.
- **Justification (operator verbatim, 2026-07-21 17:39:26):** Per `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/README.md §3`, the operator directive establishes that the JSON cache split "honors the operator's 'in-scope books no stubs, future-state books knowingly stub' doctrine" — the 4 in-scope PF1 books (Core Rulebook, Advanced Player's Guide, Advanced Class Guide, Bestiary 1) get real JSON cache builds with no stubs (Epic 3), while the 21 future-state PF1 books, of which `bestiary_5` is one, are registered as known, honest gaps rather than silently missing or fabricated (Epic 4). Same operator directive, same timestamp, establishes the scope-cross posture generally (`README.md §5`, "Why scope-cross").
- **Audit-grep impact:** None. `bestiary_5.json`'s field names and values, and this entry's prose, contain none of the dual-audit's forbidden code tokens (`STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack`) or the bundle-tag pattern. No exclusion needed.
- **Bundle-of-record:** SD-26, Epic 4 "Book Stub Manifest," criterion 4.7.
- **Remediation cycle:** `SD-27+ (unscheduled)` — matches the manifest's own `planned_resolution_bundle` field. Per `risks-and-open-questions.md §5` ("Deferrals"): concrete rule-system implementations and further corpus ingest land in SD-27+; no specific bundle number is committed yet, so neither the manifest nor this entry fabricates one.
- **Status:** Registered stub 2026-07-23.

### 0010 — `book_stub`: `bestiary_6` not yet ingested

- **Book / manifest path:** `bestiary_6` — `data/stubs/bestiary_6.json`
- **What's missing:** No `data/corpus/bestiary_6/` content exists. This PF1 sourcebook (Bestiary 6, Paizo, May 2017) has not been ingested into the corpus at all — zero class, spell, equipment, or other content-kind records. `content_kind_counts: null` in the manifest (not `0`) because no counting pass has run against this book; `null` means "not yet measured," `0` would falsely claim measurement happened and found nothing.
- **Justification (operator verbatim, 2026-07-21 17:39:26):** Per `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/README.md §3`, the operator directive establishes that the JSON cache split "honors the operator's 'in-scope books no stubs, future-state books knowingly stub' doctrine" — the 4 in-scope PF1 books (Core Rulebook, Advanced Player's Guide, Advanced Class Guide, Bestiary 1) get real JSON cache builds with no stubs (Epic 3), while the 21 future-state PF1 books, of which `bestiary_6` is one, are registered as known, honest gaps rather than silently missing or fabricated (Epic 4). Same operator directive, same timestamp, establishes the scope-cross posture generally (`README.md §5`, "Why scope-cross").
- **Audit-grep impact:** None. `bestiary_6.json`'s field names and values, and this entry's prose, contain none of the dual-audit's forbidden code tokens (`STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack`) or the bundle-tag pattern. No exclusion needed.
- **Bundle-of-record:** SD-26, Epic 4 "Book Stub Manifest," criterion 4.8.
- **Remediation cycle:** `SD-27+ (unscheduled)` — matches the manifest's own `planned_resolution_bundle` field. Per `risks-and-open-questions.md §5` ("Deferrals"): concrete rule-system implementations and further corpus ingest land in SD-27+; no specific bundle number is committed yet, so neither the manifest nor this entry fabricates one.
- **Status:** Registered stub 2026-07-23.

### 0011 — `book_stub`: `bonus_bestiary` not yet ingested

- **Book / manifest path:** `bonus_bestiary` — `data/stubs/bonus_bestiary.json`
- **What's missing:** No `data/corpus/bonus_bestiary/` content exists. This PF1 sourcebook (Bonus Bestiary, Paizo, June 2009) has not been ingested into the corpus at all — zero class, spell, equipment, or other content-kind records. `content_kind_counts: null` in the manifest (not `0`) because no counting pass has run against this book; `null` means "not yet measured," `0` would falsely claim measurement happened and found nothing.
- **Justification (operator verbatim, 2026-07-21 17:39:26):** Per `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/README.md §3`, the operator directive establishes that the JSON cache split "honors the operator's 'in-scope books no stubs, future-state books knowingly stub' doctrine" — the 4 in-scope PF1 books (Core Rulebook, Advanced Player's Guide, Advanced Class Guide, Bestiary 1) get real JSON cache builds with no stubs (Epic 3), while the 21 future-state PF1 books, of which `bonus_bestiary` is one, are registered as known, honest gaps rather than silently missing or fabricated (Epic 4). Same operator directive, same timestamp, establishes the scope-cross posture generally (`README.md §5`, "Why scope-cross").
- **Audit-grep impact:** None. `bonus_bestiary.json`'s field names and values, and this entry's prose, contain none of the dual-audit's forbidden code tokens (`STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack`) or the bundle-tag pattern. No exclusion needed.
- **Bundle-of-record:** SD-26, Epic 4 "Book Stub Manifest," criterion 4.9.
- **Remediation cycle:** `SD-27+ (unscheduled)` — matches the manifest's own `planned_resolution_bundle` field. Per `risks-and-open-questions.md §5` ("Deferrals"): concrete rule-system implementations and further corpus ingest land in SD-27+; no specific bundle number is committed yet, so neither the manifest nor this entry fabricates one.
- **Status:** Registered stub 2026-07-23.

(Entries 0012-000n reserved for the remaining 12 `book_stub` future-state books (criteria 4.10-4.22, minus `beginner_box` done by 4.2, `bestiary_2` done by 4.5, `bestiary_3` done by 4.4, `bestiary_4` done by 4.6, `bestiary_5` done by 4.7, `bestiary_6` done by 4.8, and `bonus_bestiary` done by 4.9) plus any other operator-directed exceptions. Any accidental stub found by the per-cycle audit goes into `risks-and-open-questions.md` as a Wired Integration Cleanup candidate, not here — the registry is operator-granted only.)
