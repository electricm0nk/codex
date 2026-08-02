# SD-30 Risks and Open Questions

## Primary risks

| ID | Risk | Consequence | Mitigation |
|----|------|-------------|------------|
| R-30-001 | Mythic Adventures reach-surface prerequisite | Reach gate fails for Mythic records; cycle blocks | `forward-scope-register.md C3.1` records the Mythic consumer surface as a retrofit candidate; cycles pause on `decision-blocked` |
| R-30-002 | Occult Adventures psychic-discipline consumer surface | Reach gate fails for psychic records; cycle blocks | `C3.2` records the consumer surface; per-cycle gap filing per `decisions.md §18` |
| R-30-003 | Inner Sea campaign-tool consumer integration | Series-level reach failure; bundle-level gate | `C3.3` records the campaign-tool surface; per-cycle gap filing |
| R-30-004 | Cross-bundle class-grant overlap with SD-28 (Occultist, Spiritualist, Medium, Mesmerist) | SD-28 references stale class id if SD-30 redefines | SD-30 owns canonical class definitions; SD-28 references from `progress.md`. Per `decisions.md §5` |
| R-30-005 | Recently-published precedence rule conflicts with operator-confirmed later directives | SD-30's records override SD-28/SD-29's when they should defer | `decisions.md §16` records the precedence verbatim; per-record resolution at cycle dispatch |
| R-30-006 | Build version `0.10.<build>` counter reads stale at closure | Wrong version published | Supervisor reads the current counter at cycle close, not at cycle open; closure Epic reads the post-cycle value |
| R-30-007 | Identifier-discipline audit fails on a cycle PR | Cycle blocked; PR cannot merge | Identifier audit runs in pre-cycle pre-flight (Epic 1 fires FIRST) |
| R-30-008 | Trap-report finds new trap patterns in campaign_setting books | Cycle blocks; trap catalog needs extension | Trap catalog extension is per-cycle operator-pinned; cycle records new pattern + pauses |
| R-30-009 | Reach gate running zero tests (return code 0, matched-tests = 0) | False positive; bundle closes with no surface claim | AT-30-002 hard-checks matched-tests > 0; cycle retries or reports blocker |

## Open questions

### OQ-30-001 — Per-book epic granularity for Inner Sea series

**Question:** Are the nine Inner Sea modules one shared epic (Epic 7) or
nine separate epics (Epic 7-15)? The 13-file canonical chassis supports
either; the closure gate triggers per-epic.

**Recommendation:** Treat the nine Inner Sea modules as one shared epic
for the cycle-batch level. Inner-book cycles (per-trait / per-region /
etc.) remain per-module within the shared epic. Operator-pinned at
Cycle 2 dispatch.

**Owner:** operator (per-cycle at Cycle 2 dispatch).

### OQ-30-002 — Mythic Adventures consumer-surface retrofit

**Question:** Does the Mythic Adventures consumer surface (mythic-path
mechanics + tier features + monster stat blocks) land in SD-30's epic
structure, or surface as a separate bundle?

**Recommendation:** Retrofit per `forward-scope-register.md C3.1` unless
operator overrides. SD-30's epic structure does not force the surface
build; cycles pause on `decision-blocked` if widening is absent.

**Owner:** operator (per-cycle at Epic 5 dispatch).

### OQ-30-003 — Operator action on Hermes-board references in pre-SD-30 doctrine

**Question:** Prior doctrine records `codex-tranche-<N>` board names that
no longer resolve to Hermes-board instances. Should prior doctrine be
retrofitted, or local-file references left in place?

**Recommendation:** Local-file reference per `kanban.md` resolution rule.
Prior doctrine is left in place; references resolve at cycle dispatch.

**Owner:** no in-scope decision; future bundle may retrofit.

## Intentionally deferred

- Bulk-modification retrofit (per `decisions.md §17`).
- The four deferred books — recorded in `forward-scope-register.md C2.x`: NPC Codex and Planar Adventures (absent from the corpus, future-acquisition candidates); Occult Origins and Haunted Heroes Handbook (present in the corpus under `player_companion/`, deferred by explicit operator choice 2026-08-01).
- Cross-bundle cycles that consume SD-28/SD-29's class id (the four shared classes cycle is the only such case; SD-28 references SD-30's canonical id per `decisions.md §16`).

## Review trigger

Reopen SD-30's risk register when:

- A per-book epic finds a record that doesn't reach the gate after Epic's consumer-surface prerequisite closes.
- The cross-bundle class id (Occultist family) breaks.
- The trap-report catalog gains a new pattern that affects more than one book.
- Operator authorizes a bulk-modification retrofit.
- The four deferred books surface through a future acquisition.
- A successor bundle is named.
