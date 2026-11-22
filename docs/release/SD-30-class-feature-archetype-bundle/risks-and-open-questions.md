# SD-30 Risks and Open Questions

**Re-scoped 2026-08-10 (`decisions.md §33-38`).** R-30-001/002/003 and OQ-30-001/002 (Mythic
reach-surface, Occult psychic-discipline surface, Inner Sea campaign-tool surface, Inner Sea epic
granularity) are retired or narrowed below — they were book-scoped risks for a bundle that is now
kind-scoped. New risks (R-30-010 through R-30-012) and a new open question (OQ-30-004) cover the
per-class measurement-gating shape and the closed SD-29 collision.

## Primary risks

| ID | Risk | Consequence | Mitigation |
|----|------|-------------|------------|
| R-30-001 | Mythic Adventures reach-surface prerequisite — **narrowed to `class_feature` tier features only** (RETIRED for monster/non-class content, now SD-29's) | Reach gate fails for mythic-path tier `class_feature` records; Epic 6 cycle blocks | `forward-scope-register.md C3.1` records the consumer surface as a retrofit candidate; cycles pause on `decision-blocked` |
| R-30-002 | Occult Adventures psychic-discipline consumer surface — **narrowed to `class_feature`-shaped discipline chooser content** | Reach gate fails for psychic-discipline `class_feature` records; Epic 6 cycle blocks | `C3.2` records the consumer surface; per-cycle gap filing per `decisions.md §18` |
| R-30-003 | RETIRED 2026-08-10 — Inner Sea campaign-tool consumer integration | n/a — no non-`class_feature` Inner Sea content is SD-30's | `forward-scope-register.md C3.3`, fully retired |
| R-30-004 | Cross-bundle class-grant overlap with SD-28 (Occultist, Spiritualist, Medium, Mesmerist) | SD-28 references stale class id if SD-30 redefines | SD-30 owns canonical class definitions; SD-28 references from `progress.md`. Per `decisions.md §5`, unchanged by the re-scope |
| R-30-005 | Recently-published precedence rule conflicts with operator-confirmed later directives | SD-30's records override SD-28/SD-29's when they should defer | `decisions.md §16` records the precedence verbatim; per-record resolution at cycle dispatch |
| R-30-006 | Build version `0.10.<build>` counter reads stale at closure | Wrong version published | Supervisor reads the current counter at cycle close, not at cycle open; Epic 9 (Closure) reads the post-cycle value |
| R-30-007 | Identifier-discipline audit fails on a cycle PR | Cycle blocked; PR cannot merge | Identifier audit runs in pre-cycle pre-flight (Epic 1 fires FIRST) |
| R-30-008 | Trap-report finds new trap patterns in a `class_feature`-bearing book | Cycle blocks; trap catalog needs extension | Trap catalog extension is per-cycle operator-pinned; cycle records new pattern + pauses |
| R-30-009 | Reach gate running zero tests (return code 0, matched-tests = 0) | False positive; bundle closes with no surface claim | AT-30-002 hard-checks matched-tests > 0; cycle retries or reports blocker |
| R-30-010 | An Epic 6 chassis-sweep cycle claims a class Epic 4 has not yet hand-verified, producing records with no measured wireable path | Ingested records that cannot pass reach-gate via archetype supersession; wasted cycle time; a repeat of `§63`'s core finding (sizing without measurement is unreliable) | Per-class gate, `decisions.md §37`: `epic-6-chassis-sweep` cannot claim a class without citing that class's `epic-4-measurement` receipt; `kanban.md`'s per-class dispatch note enforces this at claim time |
| R-30-011 | The `unknown` bucket's option-pool content gets bulk-reclassified or bulk-ingested without per-class characterization, repeating SD-28 Epic 15's own near-miss (an initial pass nearly extrapolated a program-wide compression ratio before per-class checking caught it, `§64`) | Fabricated or over-broad `grounded` claims on option-pool content the engine deliberately does not compute per-option | `decisions.md §38` + Epic 4-F4: characterization is per-class, cites SD-28's proven method, and explicitly does not change any unit's status by itself |
| R-30-012 | A future SD-29 lane cycle and an SD-30 Epic 6 cycle both target the same book, and a dispatcher assumes the old sixteen-book collision rule still needs manual avoidance | Wasted vigilance on a risk that Decision §35 already closed structurally (no kind is claimed by both packages in any book) | `decisions.md §35` is the authoritative closure; cite it directly rather than re-deriving the collision check per cycle |
| R-30-013 | An Epic 6 chassis-sweep cycle claims a class/book before SD30-E3-F2 (declared-PI reader) lands, shipping a `class_feature` record whose source declares `NAMEISPI:YES`/`DESCISPI:YES` past the 55-term blacklist — a real licensing exposure, not a code-quality nit (`decisions.md §39`) | Published Product Identity; the same defect SD-29 found and fixed for `race_trait` (`SD-29-corpus-wide-catch-up-lanes/decisions.md §53`) reproduces at `class_feature` scale — 464 source rows re-derived across 6 of 23 in-scope books (`adventurers_guide` 276, `inner_sea_magic` 67, `inner_sea_world_guide` 49, `inner_sea_intrigue` 45, `book_of_the_damned_volume_2` 18, `inner_sea_combat` 9) | `kanban.md`'s `epic-3-pi-gate` card states F2 as a hard block on `epic-6-chassis-sweep`; no per-class/per-book Epic 6 cycle may claim before F2's `progress.md` completion receipt exists |

## Open questions

### OQ-30-001 — RETIRED, 2026-08-10 — Per-book epic granularity for Inner Sea series

**Original question:** whether the nine Inner Sea modules run as one shared epic or nine separate
epics. **Retired, not merely answered:** the re-scope (`decisions.md §33-38`) removed the premise —
there is no per-book epic to split. Inner Sea's `class_feature` units are distributed across whichever
classes Epic 4 measures next, the same as any other book's `class_feature` content. Preserved here as
a historical record of a question the old structure needed and the new one does not.

### OQ-30-002 — Mythic Adventures consumer-surface retrofit (narrowed, still open)

**Question:** Does the Mythic Adventures `class_feature` (tier-feature) consumer surface land in
SD-30's epic structure, or surface as a separate bundle?

**Recommendation:** Retrofit per `forward-scope-register.md C3.1` unless operator overrides. SD-30's
epic structure does not force the surface build; cycles pause on `decision-blocked` if widening is
absent at Epic 6 dispatch for a mythic-path class.

**Owner:** operator (per-cycle at Epic 6 dispatch for Mythic Adventures classes).

### OQ-30-003 — Operator action on Hermes-board references in pre-SD-30 doctrine

**Question:** Prior doctrine records `codex-tranche-<N>` board names that
no longer resolve to Hermes-board instances. Should prior doctrine be
retrofitted, or local-file references left in place?

**Recommendation:** Local-file reference per `kanban.md` resolution rule.
Prior doctrine is left in place; references resolve at cycle dispatch.

**Owner:** no in-scope decision; future bundle may retrofit.

### OQ-30-004 — CLOSED 2026-08-10 — Does SD-30 need to be re-scoped or retired?

**Original question (SD-29's OQ-29-004, mirrored here):** SD-29's corpus-wide re-scope made SD-30's
sixteen-book list a subset of SD-29's scope. What should SD-30 become?

**Answer: re-scoped, not retired.** SD-30 becomes the `class_feature` bundle (`decisions.md §33-38`)
— the one kind lane SD-29's own `§38.4` explicitly leaves out. This closes both SD-29's OQ-29-004 and
R-29-009 from SD-30's side; the mirrored resolution is recorded in SD-29's own
`risks-and-open-questions.md` and `decisions.md §38.5` per this decision's cross-bundle write
authorization (reference-and-resolution only, SD-29's scope/epics untouched).

**Owner:** resolved by operator directive 2026-08-10. No further action.

## Intentionally deferred

- Bulk-modification retrofit (per `decisions.md §17`).
- Cross-bundle cycles that consume SD-28/SD-29's class id (the four shared classes cycle is the only
  such case; SD-28 references SD-30's canonical id per `decisions.md §5`/`§16`).
- The chooser-interaction primitive's design for Oracle/Arcanist/Sorcerer (Epic 4-F3) — real work,
  not yet started, explicitly not forced into this pass.
- The 303-unit genuinely-unreachable `unknown` subset and the 1,772-unit unclustered remainder
  (`decisions.md §38`) — characterized, not yet resolved; net-new engine work pending an operator
  ruling.

## Review trigger

Reopen SD-30's risk register when:

- An Epic 6 cycle finds a record that doesn't reach the gate after its class's consumer-surface
  prerequisite closes.
- The cross-bundle class id (Occultist family) breaks.
- The trap-report catalog gains a new pattern that affects more than one book.
- Operator authorizes a bulk-modification retrofit.
- Epic 4 completes its class inventory (SD30-E4-F1) and the true remaining-class count is known.
- A successor bundle is named.
