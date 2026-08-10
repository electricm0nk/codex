# SD-29 Risks and Open Questions

**Re-cut 2026-08-10 (`decisions.md §37`).** Risks and open questions below are updated for the
kind-lane structure. OQ-29-002 (Bestiary 5 ingest-subtype gate) is retired outright — the lane
structure has no per-book fallback-cycle-type decision to make; see its entry below for why. A new
risk (R-29-008) covers the Epic 3 provenance gate.

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

## Open questions

### OQ-29-001 — DM Toolkit extension (Epic 8): in-scope or retrofit?

**Question:** Does the DM Toolkit extension (consume SD-29's monster
records) land in SD-29's Epic 8, or surface as a separate bundle?

**Recommendation:** Retrofit per `successor-forward-scope-register.md C3.1` unless
operator overrides. SD-29's epic structure does not force the surface
build; cycle records `decision-blocked` in progress.md and moves to next ready card if widening is absent at
Epic 8's closure gate — now Epic 4's pilot cycle-batch (Bonus Bestiary), not "Epics 5 and 6" under
the retired per-book numbering.

**Owner:** operator (per-cycle at Epic 4's pilot cycle-batch closure; Epic 8's gating).

### OQ-29-002 — RETIRED, 2026-08-10 — Bestiary 5 ingest subtype at cycle-0

**Original question:** whether Bestiary 5 (zero `monster` units) proceeds with player-options
cycles or gets swapped for Bestiary 6 + Bonus Bestiary. **Retired, not merely answered:** the
kind-lane re-cut (`decisions.md §37`) removed the premise this question needed — there is no
per-book epic to gate or swap. Bestiary 5's units are simply distributed across Epic 4
(`monster_ability`, 39), Epic 5 (`race_trait`, 63), and Epic 6 (`companion`, 57); Bestiary 6 was
never a candidate swap-in, it is one of the seven books each lane already covers. Preserved here as
a historical record of a question the old structure needed and the new one does not.

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
- The cross-bundle monster-record join (SD-22 Bestiary 1 ∪ SD-29's seven books) breaks.
- The trap-report catalog gains a new pattern that affects more than one bestiary.
- Operator authorizes a bulk-modification retrofit.
- A successor bundle is named.
