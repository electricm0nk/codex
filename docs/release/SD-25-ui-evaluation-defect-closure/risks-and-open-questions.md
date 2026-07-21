# SD-25 — Risks and Open Questions

> **Operating method:** see `./scope-draft.md`. Self-heal runs against this document's split.

## 1. Self-healable conditions

| Condition | Self-heal |
|---|---|
| Working tree dirty from prior failed cycle | `git checkout -- <file>` or `git reset --hard HEAD~1` |
| Single identifier-audit leak | rename inline; re-audit; commit |
| Single wired-integration four-check failure | remove the token; re-audit; commit |
| Cycle's tests fail for unrelated reason | fix the test setup |
| Cycle finds undesigned stub | record in Stubs Registry |
| Build counter out of sync | re-read `apps/desktop/package.json` + `apps/desktop/src-tauri/tauri.conf.json`; update `decisions.md §4` |
| `## DISCOVERED` duplicates | merge duplicates; mark de-dup |

## 2. Non-self-healable conditions (write to `## Open blockers`, exit FAIL)

| Condition | Action |
|---|---|
| Working tree diverged from `tranche/5-3` needs manual rebase | `## Open blockers`; exit FAIL |
| Two live orchestrators on conflicting files | First wins; second writes `CLAIM-EXISTS`; exit FAIL |
| SD-24 closure PR not merged to develop | Loop refuses to start (Tier-1 launch gate) |
| `## DISCOVERED` queue > 10 entries | Operator override required; pause |
| RED → GREEN not preserved in artifact | Cycle re-run with RED → GREEN captured |
| Cycle finds `success: true` from fake operation | Cycle rejected; falls back to wired-integration audit |
| Cycle finds inline mocks in shipping modules | Cycle rejected; cannot mark `complete` |
| Cycle finds "Would …" return strings in shipping code | Cycle rejected; cannot mark `complete` |
| Concurrent-write protocol fails 5 times (CLAIM-EXISTS) | Stop the orchestrator; operator intervention |

## 3. Override flags (operator-pinned)

| Flag | Default | Set behavior |
|---|---|---|
| FLAG-A: STRICT-STOP-AT-DEADLINE | unset (grace-tail) | strict stop at operator's deadline |
| FLAG-B: DEFERRAL-TO-SD-26 | unset | defer all UI-discovered defects to SD-26 |
| FLAG-C: STUB-BOOKS-OFF | unset | forbid stub bookings; future-system stubs require explicit operator sign-off per cycle |

## 4. Open questions (deferred to operator)

| Q | Question | Default |
|---|---|---|
| Q1 | SD-25 runtime scope vs. SD-26 — what if cycle count is too high for operator-pinned deadline? | Defer excess to SD-26 with FLAG-B |
| Q2 | UI-eval session findings count — how many defects? | Unknown; bundle is discovery-dominant |
| Q3 | Hub-of-Hubs extractor (3.2) — does it require migrating all existing character_hub tests or just keeping them passing? | Keep them passing (don't migrate test infrastructure to trait-based; SD-25 ships the adapter, SD-26 fans out the test refactor) |
| Q4 | Hard-stop at operator-set deadline — grace-tail or strict? | Grace-tail (FLAG-A is the override) |

## 5. Deferrals (operator-pinned non-self-healable items deferred to follow-on bundles)

- **PCGen library build** (SD-26's job). SD-25 ships the runner scaffolding only.
- **JSON cache build for 26 books** (SD-26).
- **Book-stub-manifest entries** (SD-26; Stubs Registry `book_stub` kind).
- **Equipment corpus extension** beyond PF1 core rules + APG + ACG + Bestiary 1 (deferred). Unaffected by Epic 7's new equipment/spell corpus intake (added 2026-07-21) — that intake stays inside the existing 4-book scope; it backfills SD-24's within-scope gaps (CRB/APG description ceilings, APG spell-text ceiling, Bestiary 1 never dispatched), not new books.
- **Storage-tier structural convergence** (deferred per SD-24's storage-tier deferral).
- **Hub-of-Hubs multi-system implementations** beyond the trait + StubAdapter (SD-26 / later).
- **Inline mocks / "Would …" strings outside bundle's file-touch** (next Wired Integration Cleanup epic).

## 6. Latent risks (monitored but not-blocking)

- **SD-24 closure PR lag.** Tier-1 launch gate unsatisfied if SD-24 is mid-cycle at deadline.
- **PCGen Gradle interaction.** `gradlew` requires Java; first invocation may need a JVM warm-up cycle.
- **Operator-tempo ceiling.** Discovery-dominant bundles require operator attention to manage `## DISCOVERED` priority-bump tags. If operator is unavailable for >10 entries, the loop pauses.
- **Oracle-parity assertion gap.** SD-25's PCGen runner scaffolds the script + normalize pipeline but does not yet assert parity (that's the comparator in SD-26). Until then, SD-25's runner is "produce one oracle output" not "claim parity."
- **Web-sourced content risk (Epic 7's equipment/spell corpus intake, added 2026-07-21).** d20pfsrd.com / aonprd.com content is not machine-verified the way the ingested PCGen LST corpus is (checksum-free, page structure can drift, a same-named cross-book/cross-edition item is a real false-match risk). Mitigated by the identity-match-before-write rule and per-record source-URL citation already specified in `epic-breakdown.md`'s Epic 7 criterion — not a reason to skip the pass, but cycles should not treat a web fetch as ground truth the way an LST token is.

## 7. Cross-reference

- `./scope-draft.md §5 Hard-stop conditions` — bundle-level hard stops
- `./decisions.md §3` — per-epic concurrency + tiering
- `./decisions.md §6` — tier-1 launch gate
- `./loop-instruction.md §5` — concurrent-write protocol
- `./loop-instruction.md §8` — self-heal posture
