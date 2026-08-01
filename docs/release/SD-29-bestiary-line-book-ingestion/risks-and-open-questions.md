# SD-29 Risks and Open Questions

## Primary risks

| ID | Risk | Consequence | Mitigation |
|----|------|-------------|------------|
| R-29-001 | Bestiary 5 cycle shape differs from Bestiary 2-4 (player-options vs monster-block) | Per-book cycle shape variability, dispatch timing differences | Cycle-0 trap-report + work-inventory output gates Epic 6; per-cycle receipt records shape-finding |
| R-29-002 | Bestiary 1 monster-surface prerequisite blocks Epic 7 | DM Toolkit extension records gap; cycles pause on `decision-blocked` | `successor-forward-scope-register.md C3.1` records the toolkit extension as operator-on-request retrofit; cycles land even if Epic 7 surfaces |
| R-29-003 | Cross-bundle monster reprint (e.g., a famous monster in B2 reappearing in B5 with wording changes) | Cycle records the conflict; trap-report re-runs | `decisions.md §16` cross-book conflict rule applies; newer book is doctrine, older is errata |
| R-29-004 | Build version `0.9.<build>` counter reads stale at closure | Wrong version published | Supervisor reads the current counter at cycle close, not at cycle open; closure Epic 8 reads the post-cycle value |
| R-29-005 | Identifier-discipline audit fails on a cycle PR | Cycle blocked; PR cannot merge | Identifier audit runs in pre-cycle pre-flight (Epic 1 fires FIRST); failed audits gate the cycle |
| R-29-006 | Trap-report finds new trap patterns (e.g., a monster with overlay metadata only present in B5 not B2) | Cycle records the trap; trap catalog needs extension | Trap catalog extension is per-cycle operator-pinned; cycle records new pattern + pauses |
| R-29-007 | Bestiary 5 records overlap with prior books (Adventurer's Guide races, etc.) | Cycle records the overlap; operator decides which bundle owns the canonical record | Trap-report surfaces the overlap in cycle-0; operator-pinned per-record which bundle owns the canonical definition |

## Open questions

### OQ-29-001 — DM Toolkit extension (Epic 7): in-scope or retrofit?

**Question:** Does the DM Toolkit extension (consume Bestiary 2-5) land in
SD-29's Epic 7, or surface as a separate bundle?

**Recommendation:** Retrofit per `successor-forward-scope-register.md C3.1` unless
operator overrides. SD-29's epic structure does not force the surface
build; cycles pause on `decision-blocked` if widening is absent at
Epic 7's closure.

**Owner:** operator (per-cycle at Epic 5/6 closure; Epic 7's gating).

### OQ-29-002 — Bestiary 5 ingest subtype at cycle-0

**Question:** Cycle-0 trap-report + work-inventory output is the gate.
If Bestiary 5 surfaces `monster` units (zero, per the 07-30 shape
finding), do we proceed with per-race / per-feat / per-companion-mod
cycles, or drop Bestiary 5 in favor of Bestiary 6 + Bonus Bestiary?

**Recommendation:** Proceed with player-options cycles if zero `monster`
units (per `decisions.md §18`); operator-pinned per-cycle at Epic 6
dispatch.

**Owner:** operator (per-cycle at Epic 2 pre-flight).

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

- A per-book epic finds a record that doesn't reach the gate after Epic 7's toolkit extension closes.
- The cross-bundle monster-record join (SD-22 Bestiary 1 ∪ SD-29 Bestiary 2-5) breaks.
- The trap-report catalog gains a new pattern that affects more than one bestiary.
- Operator authorizes a bulk-modification retrofit.
- A successor bundle is named.
