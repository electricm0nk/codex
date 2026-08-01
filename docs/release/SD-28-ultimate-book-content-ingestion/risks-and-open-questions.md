# SD-28 Risks and Open Questions

## Primary risks

| ID | Risk | Consequence | Mitigation |
|----|------|-------------|------------|
| R-28-001 | Dreamscarred Press license audit surfaces records not matching open-content tier | Dropped records leave gaps in UPsi ingest | Cycle 0 precheck records each gap; downstream cycles proceed; dropped records logged in `artifacts/upsi-license-drops.md` |
| R-28-002 | Ultimate Equipment equipment catalog still CRB-only at epic 5 closure | Reach gate fails for UE records; cycle blocks | Forward-scope C3.1 records the catalog widening as operator-on-request retrofit; cycles pause on `decision-blocked` if widening absent |
| R-28-003 | Cross-bundle class overlap (UI ∩ SD-30) breaks if SD-30's canonical id changes mid-cycle | SD-28 references stale class id | Stable SD-30 reference card maintained; cycles read SD-30 progress before and after dispatch |
| R-28-004 | Build version `0.8.<build>` counter reads stale at closure | Wrong version published | The supervisor reads the current counter at cycle close, not at cycle open; closure Epic 10 reads the post-cycle value |
| R-28-005 | Hermes board retirement mid-flight leaves orphaned card references | Prior doctrine references Hermes cards that no longer exist | `kanban.md` card-id mapping maintained in this directory; references resolve at cycle dispatch |
| R-28-006 | Identifier-discipline audit fails on a cycle PR | Cycle blocked; PR cannot merge | Identifier audit runs in pre-cycle pre-flight (Epic 1 fires FIRST); failed audits gate the cycle |
| R-28-007 | Trap-report finds new trap patterns in newer Paizo U-line books | Cycle blocked; trap catalog needs extension | Trap catalog extension is per-cycle operator-pinned; cycle records new pattern + pauses |

## Open questions

### OQ-28-001 — UE equipment catalog widening: in-scope or retrofit?

**Question:** Is the equipment-catalog widening (`equipment_catalog.rs` from
CRB-only to all-books) in SD-28's epic structure, or a named prerequisite
outside it?

**Recommendation:** Retrofit per `forward-scope-register.md C3.1`. SD-28's
epic structure does not add a surface-building epic; cycles pause on
`decision-blocked` if widening is absent at Epic 5's closure. The operator
picks.

**Owner:** operator (per-cycle at Epic 5 closure).

### OQ-28-002 — Operator action on Hermes-board references in pre-SD-28 doctrine

**Question:** Prior doctrine records `codex-tranche-<N>` board names that
no longer resolve to Hermes-board instances. Should prior doctrine be
retrofitted, or local-file references left in place?

**Recommendation:** Local-file reference per `kanban.md` resolution rule
(card id → `kanban.md` row at dispatch). Prior doctrine is left in
place; references resolve at cycle time.

**Owner:** no in-scope decision; future bundle may retrofit.

### OQ-28-003 — Dreamscarred Press coverage for `psionics_unleashed` and `psionics_expanded`

**Question:** Operator named `ultimate_psionics` in SD-28's book list.
The corpus also has `psionics_unleashed` and `psionics_expanded`.
Are these in-scope for SD-28 or a separate bundle?

**Recommendation:** Out of scope for SD-28 per `forward-scope-register.md
C2.1`. The three Dreamscarred Press books are recorded as
"future-acquired, deferred." A future bundle may lock them in.

**Owner:** future bundle (operator-on-file).

## Intentionally deferred

- Bulk-modification retrofit (per `decisions.md §17a`).
- Catalog widening (per OQ-28-001 above).
- Post-tranche consumer (per the bundle's exiting relationship with
  whatever tranche follows `tranche/8`).

## Review trigger

Reopen SD-28's risk register when:

- A per-book epic finds a record that doesn't reach the gate after Epic 5's
  known equipment-surface gap is closed.
- The cross-bundle class id (UI ∩ SD-30) breaks.
- The trap-report catalog gains a new pattern that affects more than one book.
- Operator authorizes a bulk-modification retrofit.
- A successor bundle is named.
