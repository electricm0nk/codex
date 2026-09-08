---
canonical: true
owner: god-emporer
bundle_id: SD-35
date: 2026-09-07
---

# SD-35 Forward-Scope Register

Successor work depending on this package's output. **No unowned tidiness entries.**

**This register is not a parking lot.** Scope in SD-35's Definition of Done at launch cannot
appear here (`../../governance/blocker-closure-doctrine.md`). A row here is either (a) work that
only becomes possible *because* SD-35 shipped, or (b) a question SD-35's posture deliberately
does not answer.

**The test:** was this scope in the Definition of Done at launch? If yes, it is a blocker, not a
register row.

## C1.x — Owned by an SD-35 successor

| ID | Item | Owner |
|---|---|---|
| C1.1 | **The `Words` share.** Under `decisions.md §1` a term the character does not settle prints as words. At closure AT-35-E7-002 reports the `Words` share per kind. A successor may decide to audit a sample of `Words` lines for terms that *could* have resolved (`risks-and-open-questions.md §2`). Not SD-35 scope: SD-35's bar is that the line renders in one of three forms; which form is a per-record fact. | A successor SD-N, or a research spike. |
| C1.2 | **Subsystems the sheet rule made unnecessary, named:** a character-size subsystem (SD-34 wave 51's Monk blocker), a downtime/retraining subsystem (SD-34 wave 50's Ultimate Campaign blocker), a multi-companion progression system (SD-34 wave 44's Summoner/Broodmaster), a temporary-effect (`TEMPBONUS`) subsystem. Each renders as words in SD-35. A product bundle that wants them live gets this list with the unit counts from `token-coverage.json`. | A future product SD-N. **Named because SD-34 named them**, not because SD-35 deferred them. |
| C1.4 | **Our expression vocabulary (`Expr`) as a public schema.** SD-35 defines it for PF1e's needs. A second system (C2.1, C2.2) will need variants (`Expr::Skill`, resource pools). Not SD-35 scope: the vocabulary is closed for this bundle so the converter can be proven complete against it. | The first non-PF1e SD-N. |
| C1.3 | **SD-34's capability register rows proven unnecessary under the sheet rule** (AT-35-E5-005's `unnecessary-under-sheet-rule` values). Each carries the reason; a successor that reinstates a simulation-shaped bar for some kind re-opens the row. | A future SD-N, only if the bar changes. |

## C2.x — Future SD-N ownership

| ID | Item | Owner |
|---|---|---|
| C2.1 | **The second PCGen-format reader** (Starfinder, `data/starfinder` in the pinned checkout). Inherited unchanged through SD-32, SD-33, SD-34. **Explicitly out of SD-35's scope** (`scope-draft.md §9`). **Operator, 2026-09-07: Starfinder starts right after PF1e.** SD-35 hands it the converter, the parser, the generators, the oracle harness, and the pinned checkout **intact** (`decisions.md §11`, what is kept); the work is a second `.pcc` include structure and a second `Expr` vocabulary pass (C1.4), not a rebuild. | **SD-36.** |
| C2.2 | **Traveller, Cyberpunk Red, World of Darkness, Solarus Arcanum.** Inherited from SD-34 C2.2. Each needs its own answer to "what is the oracle" and now also "what is the sheet line" for a non-PCGen token vocabulary. | A future SD-N per system. |
| C2.3 | **The form-interpreter PMMG build** ("Edge of the Sea" tranche). Inherited from SD-34 C2.3; `scripts/verify.sh` still carries the warning every cycle. | A future SD-N. |
| C2.4 | **Fable review report-only proposals not taken by SD-35**: `pilot_compute/mod.rs` split (PC1–PC10, 78.5k lines with named seams), `src/bin` family consolidation beyond the bloat batches already landed, `v06_work_inventory.rs`'s `classify()` extraction (R9-03, a single ~2,555-line function), cache_gen dedup (R14-04/05), desktop dead surface (R11-03/04), un-run `scripts/tests` (R13-1). SD-35 takes only R10 (test families, AT-35-E1-003) because it is the one that moves build time. | A future refactor SD-N; `../SD-34-book-completion/fable-review.md §4` is the ranked list. |
| C2.5 | **The eight confirmed fable-review P1s** (`../SD-34-book-completion/fable-review.md §2`): `character_id` path-join validation, non-atomic saves in three stores, the C1.9 provenance basename bug in `v06_work_inventory.rs`, per-weapon size modifier omission, explanation-id paren slugging, chooser `recognized` contradiction, the companion-table emitter's non-atomic write, reach_gate "Surfaced" on a command with no frontend caller. **Not SD-35 scope** — none blocks a sheet line — but PC8-2 (per-weapon size modifier) is a wrong sheet number today and should be the first row of whichever bundle owns them. | A dedicated correctness SD-N, or the operator's call to fold into SD-35 by ruling. |

## C3.x — Research-grade forward scope

| ID | Item | Owner |
|---|---|---|
| C3.1 | **How to verify a system whose rules exist only as prose.** Inherited from SD-34 C3.1. | A research spike. |
| C3.2 | **Whether oracle agreement is the right definition of correct.** Inherited from SD-34 C3.2. SD-35 sharpens it further: the oracle checks totals; the sheet line is a new surface the oracle never sees. | A research spike. |
| C3.3 | **Errata.** Inherited from SD-34 C3.3. A corpus at 49,438 of 49,438 has no mechanism to notice its source changed upstream. | A future SD-N. |

## E1.x — Branches ruled OUT of SD-33's 2026-08-26 fold — do not re-litigate

Mirrored from `../SD-34-book-completion/forward-scope-register.md §E1`, unchanged. SD-34's
AT-34-E6-003 sweep deletes them; if SD-34's closure did not, AT-35-E7-003's sweep does, on the
same ruling, without re-diagnosis.

| ID | Branch | Ruling |
|---|---|---|
| E1.1 | `worktree-wf_a45ece26-3fc-1` | Superseded (1,612 grant files with the wrong `class` field semantics) |
| E1.2 | `worktree-wf_13156488-c9b-1` | Superseded (wave 20, replaced by SD-32's roster) |
| E1.3 | `worktree-wf_c1156061-e3f-5` | Superseded (27 lines of notes) |

## Carried forward from SD-34

| ID | Item | Owner |
|---|---|---|
| SD-33 open deferral 1 (`1787633115006-sd33-e4-unknown-136912`) — widen `REGISTERED_POOL_GROUPS`, 1,128 unmatched group prefixes | **AT-35-E3-001.** In SD-35's Definition of Done — not a carry. The mechanism is the corpus's own `ABILITYCATEGORY`/`CLASS` declarations, not per-group research. | AT-35-E3-001 |
| SD-33 open deferral 2 (`1787633121875-sd33-e4-unknown-58d073`) — `unmeasurable` in the dashboard producer's `_doneness_verdict_uncapped()` | **AT-35-E2-003** touches every status consumer; this closes there. | AT-35-E2-003 |
| SD-33 open deferral 3 (`1787667636036-sd33-r6-skillcombat-3dee2d`) — COMBAT non-AC subtoken aggregation (6 units), cross-record class-feature variable resolution (2), Special-Quality eqmod live-oracle attachment (2) | The 6 and 2 are converter mapping rows (AT-35-E4-001); the eqmod attachment is an oracle-harness question for AT-35-E4-002's run. In SD-35's DoD. | AT-35-E4-001, AT-35-E4-002 |
| SD-33 inherited test debt — 29 of 599 suites / 46 of 8,034 tests, proven pre-existing at `tranche/13` | Re-derived at the `tranche/15` cut as SD-35's baseline (`technical-requirements.md §3`). AT-35-E1-003's consolidation touches many of those suites; any that become green are reported, any still red are carried with their SHAs. SD-35 does not own fixing the rest. | A future SD-N, or a dedicated cleanup cycle. |
| `site-dashboard-check` hang — `publish-site-dashboard.sh --check` invokes `v06_work_inventory --summary` with no timeout | AT-35-E1-002 touches `verify.sh`'s stage list; the timeout wrapper lands there. In SD-35's DoD. | AT-35-E1-002 |
| SD-31 Decision 20's run-time interpreter permission | **Revoked by `decisions.md §11`; the exit is AT-35-E6-001..004.** In SD-35's DoD — not a carry. | AT-35-E6-001..004 |
| SD-34's own open cards at the `tranche/15` cut | **17 of 37 rows open** (13, 14, 15, 17, 20, 26, 27, 28–37) — SD-34 merged without its epilogue. **Folded into AT-35-E1-006** by operator ruling (`decisions.md §12`): retrospective written and cited, rows mapped to the SD-35 criteria that own their units. In SD-35's DoD — not a carry. | AT-35-E1-006 |
