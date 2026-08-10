# SD-29 Risks and Open Questions

**Re-cut 2026-08-10 (`decisions.md §37`).** Risks and open questions below are updated for the
kind-lane structure. OQ-29-002 (Bestiary 5 ingest-subtype gate) is retired outright — the lane
structure has no per-book fallback-cycle-type decision to make; see its entry below for why. A new
risk (R-29-008) covers the Epic 3 provenance gate.

**Re-scoped corpus-wide 2026-08-10 (`decisions.md §38`).** Every lane now runs across all 37
in-scope books, not the retired seven. A new risk (R-29-009) and open question (OQ-29-004) record
the SD-30 collision this re-scope surfaces: SD-30's sixteen-book list is now a subset of SD-29's
corpus-wide lane scope, and this package's write scope does not extend to resolving it.

## Primary risks

| ID | Risk | Consequence | Mitigation |
|----|------|-------------|------------|
| R-29-001 | A book with zero `monster` units (Bestiary 5, 6) is mistaken for a gap rather than an ordinary lane-routing fact | Wasted cycle time investigating a non-issue | Epic 2's corpus-wide pre-flight records every book's per-kind counts once; a zero count for one lane's kind in one book is not an exception — see AT-29-008 |
| R-29-002 | Bestiary 1 monster-surface prerequisite blocks Epic 8 | DM Toolkit extension records gap; cycle records `decision-blocked` and moves to next ready card | `successor-forward-scope-register.md C3.1` records the toolkit extension as operator-on-request retrofit; cycles land even if Epic 8 surfaces |
| R-29-003 | Cross-bundle monster reprint (e.g., a famous monster in B2 reappearing in B5 with wording changes) | Cycle records the conflict; trap-report re-runs | `decisions.md §16` cross-book conflict rule applies; newer book is doctrine, older is errata |
| R-29-004 | Build version `0.9.<build>` counter reads stale at closure | Wrong version published | Supervisor reads the current counter at cycle close, not at cycle open; closure Epic 11 reads the post-cycle value |
| R-29-005 | Identifier-discipline audit fails on a cycle PR | Cycle blocked; PR cannot merge | Identifier audit runs in pre-cycle pre-flight (Epic 1 fires FIRST); failed audits gate the cycle |
| R-29-006 | Trap-report finds new trap patterns (e.g., a monster with overlay metadata only present in B5 not B2) | Cycle records the trap; trap catalog needs extension | Trap catalog extension is per-cycle operator-pinned; cycle records new pattern and moves to next ready card |
| R-29-007 | Bestiary 5 records overlap with prior books (Adventurer's Guide races, etc.) | Cycle records the overlap; operator decides which bundle owns the canonical record | Trap-report surfaces the overlap in cycle-0; operator-pinned per-record which bundle owns the canonical definition |
| R-29-008 | A lane's Pipeline-B content (`rules_tables/*.rs`) ships a Product-Identity leak, the same shape as the three real leaks `docs/governance/license-matrix.md` already found in other bundles' tables | Unredacted PI content in committed source | Epic 3's per-lane PI-blacklist sweep is a hard gate on every lane's first content commit per book (AT-29-003a); a hit is a stop, not a routed-around finding |
| R-29-009 | **RESOLVED 2026-08-10** — SD-30 (`docs/release/SD-30-occult-and-companion-content-ingestion/decisions.md §1`) owned a planning-ready sixteen-book list, dispatched per-book, not re-cut by this package's corpus-wide re-scope (`decisions.md §38.5`). Every one of those sixteen books carried units of SD-29's now-corpus-wide lanes — the same (kind, book) cells. Two writers (an SD-29 lane cycle and an SD-30 per-book cycle) could land on the same table file for the same book | Duplicate or conflicting canonical records for the same content; wasted cycle time; possible silent overwrite | **Resolved structurally, not by dispatch discipline.** Operator directive 2026-08-10 re-scoped SD-30 to the `class_feature` bundle (the one kind lane this package's `§38.4` leaves out); SD-30's sixteen-book list dissolved outright. No (kind, book) cell is now claimed by both packages — SD-29 claims every kind except `class_feature` corpus-wide, SD-30 claims only `class_feature` corpus-wide. See `docs/release/SD-30-class-feature-archetype-bundle/decisions.md §33-35` (renamed from `SD-30-occult-and-companion-content-ingestion` via `git mv`) and this package's own `decisions.md §38.5` resolution note. |

## Open questions

### OQ-29-001 — DM Toolkit extension (Epic 8): in-scope or retrofit?

**Question:** Does the DM Toolkit extension (consume SD-29's monster
records) land in SD-29's Epic 8, or surface as a separate bundle?

**Recommendation:** Retrofit per `successor-forward-scope-register.md C3.1` unless
operator overrides. SD-29's epic structure does not force the surface
build; cycle records `decision-blocked` in progress.md and moves to next ready card if widening is absent at
Epic 8's closure gate — now Epic 5's pilot cycle-batch (Bonus Bestiary), not "Epics 5 and 6" under
the retired per-book numbering, and not Epic 4 under the retired seven-book kind-lane numbering
(Epic 4 is now the corpus-wide proven-path lane, `decisions.md §38`).

**Owner:** operator (per-cycle at Epic 5's pilot cycle-batch closure; Epic 8's gating).

### OQ-29-002 — RETIRED, 2026-08-10 — Bestiary 5 ingest subtype at cycle-0

**Original question:** whether Bestiary 5 (zero `monster` units) proceeds with player-options
cycles or gets swapped for Bestiary 6 + Bonus Bestiary. **Retired, not merely answered:** the
kind-lane re-cut (`decisions.md §37`) removed the premise this question needed — there is no
per-book epic to gate or swap. Bestiary 5's units are simply distributed across Epic 5
(`monster_ability`, 39), Epic 6 (`race_trait`, 63), and Epic 7 (`companion`, 57) under the current
(`decisions.md §38`) numbering; Bestiary 6 was never a candidate swap-in, it is one of the (now 37,
originally seven) books each lane already covers. Preserved here as a historical record of a
question the old structure needed and the new one does not.

### OQ-29-004 — CLOSED 2026-08-10 — SD-30 collision: does SD-30 need to be re-scoped or retired?

**Question:** SD-29's re-scope to corpus-wide lanes (`decisions.md §38`) makes SD-30's sixteen-book
list a subset of SD-29's scope. What should SD-30 become — retired outright, folded into SD-29's
lanes, or narrowed to the content SD-29's lanes genuinely do not reach (if any remains after the
overlap is mapped)?

**Answer: re-scoped, not retired or folded in.** Operator directive 2026-08-10: SD-30 becomes the
`class_feature` bundle — the one kind lane this package's own `§38.4` names as staying out of SD-29's
scope, funded through `corpus-work-channels.md §9.1` and previously unassigned to any SD number
(`successor-forward-scope-register.md C1.3`). SD-30's sixteen-book list dissolves outright, not
narrowed to a subset of those sixteen books — see `docs/release/SD-30-class-feature-archetype-bundle/decisions.md
§33-35` for the full resolution (renamed from `SD-30-occult-and-companion-content-ingestion` via
`git mv`, this package's own `decisions.md §38.5` cross-references it). R-29-009 above is resolved by
the same directive.

**Owner:** resolved by operator directive 2026-08-10. No further action from this package.

### OQ-29-003 — Operator action on Hermes-board references in pre-SD-29 doctrine

**Question:** Prior doctrine records `codex-tranche-<N>` board names that
no longer resolve to Hermes-board instances. Should prior doctrine be
retrofitted, or local-file references left in place?

**Recommendation:** Local-file reference per `kanban.md` resolution rule.
Prior doctrine is left in place; references resolve at cycle dispatch.

**Owner:** no in-scope decision; future bundle may retrofit.

## Intentionally deferred

- Bulk-modification retrofit (per `decisions.md §17`).
- Monster catalog command + browser (per the 07-30 scope-draft proposal; epic deferred to a separate bundle).
- Cross-bundle SD-29 ∩ SD-30 dependencies (Occultist family reprints; SD-30 owns canonical class definitions per Honcho duracon).

## Review trigger

Reopen SD-29's risk register when:

- A lane cycle-batch finds a record that doesn't reach the gate after Epic 8's toolkit extension closes.
- The cross-bundle monster-record join (SD-22 Bestiary 1 ∪ SD-29's corpus-wide lane scope) breaks.
- The trap-report catalog gains a new pattern that affects more than one book.
- Operator authorizes a bulk-modification retrofit.
- A successor bundle is named (`class_feature`'s successor is now SD-30, resolved 2026-08-10 —
  `decisions.md §38.4`/OQ-29-004 no longer open).
- **CLOSED 2026-08-10:** the operator resolved the SD-30 collision (OQ-29-004) — SD-30 became the
  `class_feature` bundle; R-29-009's mitigation is updated in place above, not left pending.
