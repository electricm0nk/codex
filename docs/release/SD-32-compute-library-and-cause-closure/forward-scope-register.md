---
canonical: true
owner: god-emporer
status: planning-ready (chassis completed 2026-08-22 from SD-31 session)
date: 2026-08-22
---

# SD-32 Forward-Scope Register

Successor work depending on this package's output. Format follows
`SD-31-corpus-closure-grind/forward-scope-register.md`'s convention (one row per item, named
owner, no unowned tidiness entries). Three classes: **C1** = this package's successor epics own
the work, **C2** = a future SD-N owns it, **C3** = research-grade forward scope (not engineering
work in SD-32's posture).

## C1.x — Owned by a SD-32 successor epic

| ID | Item | Owner |
|---|---|---|
| C1.1 | Any class the Epic 3 class reachability work finds cannot be chassis-built within that epic's bound (e.g. a genuinely bespoke subsystem needing prose interpretation) is a **named finding**, not silently dropped. After `decisions.md §2`'s anti-gaming rule, not a unilateral deferral either: it is a proposal to the Structural Exclusion Register requiring operator sign-off. | Unassigned pending Epic 3's actual finding — do not pre-assign a successor for a population not yet named. |
| C1.2 | `docs/work-inventory.json`'s `doneness_verdict()` table for any new status word or computed-not-ingested category that survives Gate 0's census closure. The generator and producer must change in the same commit (`SD-30-.../state-goals-and-lessons.md §1.3` hazard 4). | The cycle that introduces the new status word. |
| C1.3 | The four unbuilt books (Epic 4 scope) that survive Gate 0's closed census: each is a C1 candidate for the cycle that proves its compiled rule set, with the per-cycle receipt naming the count of units it now covers. | Epic 4 (per-book cycles). |

## C2.x — Future SD-N ownership

| ID | Item | Owner |
|---|---|---|
| C2.1 | The second PCGen-format reader, when it exists. The first reader (LST, for PF1e) lives in SD-32's Gate 0 census walker. The second (e.g. for Starfinder, which is in the PCGen checkout at `data/starfinder` and shares `.lst` format but a different `.pcc` include structure) is the test the abstraction must survive before it is generalised (`decisions.md §5`). | A future SD-N. Starfinder is the obvious first candidate because the corpus already includes it. |
| C2.2 | The Traveller, Cyberpunk Red, World of Darkness, and Solarus Arcanum systems the operator has surveyed (`docs/governance/` licensing survey). All four are out of scope for SD-32; SD-32's tooling builds the **seam** they would consume (the `reader / analyser / reporter` shape of the census tool, the analyser-as-method-not-vocabulary posture), but does not build their readers. | A future SD-N per system. World of Darkness and Solarus Arcanum are not in PCGen; they need a different source strategy entirely (see C3.1). |
| C2.3 | The form-interpreter PMMG build ("Edge of the Sea" tranche), referenced in `SD-30-.../state-goals-and-lessons.md §1.3` hazard 4 (`scripts/observer/PF1e-dashboard.html` requires the PMMG build to render real shapes; `verify.sh` flags its absence every cycle). | A future SD-N. Tracked because every SD-N's `verify.sh` run carries the warning. |
| C2.4 | The four books with no compiled rule set on SD-32 close: those that survive Epic 4 but still have non-zero `not-done` units after Gate 3 are filed here as future SD-N work. The Epic 4 work is the on-ramp; the closure invariant surfaces what Epic 4 didn't reach. | A future SD-N, owned by whatever bundle follows SD-32. |
| C2.5 | **RESOLVED, removed 2026-08-23 — this entry was stale.** Originally filed Epic 2 (card 11)'s T2a/T2b/T9/T4/T12/T7/T8 blocker shapes here as `returned-to-backlog`, deferred to a future SD-N. `decisions.md §10`/`§13` (2026-08-22) overturned that disposition — "filed under Open blockers" is not a closure path, and the operator ruled all five (T2a, T2b, T9, T12, T4-L9, per `§13`'s table) closed by doing the work, not deferred. That work landed inside SD-32 itself: T1/T2a/T2b/T4/T7/T8 closed via the `epic-2-cause-closure/4` consolidation cycle (`bdb27d63f`) plus the `§20` generic-ingest campaign (`progress.md` "## Open blockers", five entries, all marked `RESOLVED, removed 2026-08-23`); T8 closed via `decisions.md §11`'s granted write scope to `scripts/observer/pf1e_dashboard_producer.py`; T9's residual `monster_ability` shape closed to zero (`no_record` 56→0, commit `be100ceea6`, "T9 round 9" — live-reconfirmed 2026-08-23 via `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json`: `join_status_counts={'matched': 11578, 'no_formula_tokens': 22819}`, no `no_record` key present); T12's magnitude-bearing population closed (`452c70d035`, `2382bed37b`, `a0ee0db4f4`, `d10da0a7ea`, `cd60d08042`). **One genuinely new, currently-unowned population surfaced along the way and is NOT covered by this removal**: `src/rules_core/class_feature_pool_catalog.rs`'s pool-shaped exclusion class (~1,913 group-qualified names, ~16,350 records, ~6,131 magnitude-bearing, only 2 of 27 registered pools modeled) — found by the `cd60d08042` cycle, confirmed CONFIRMED-OPEN by the closure-readiness audit (`artifacts/gate-3-closure-invariant/closure-readiness-audit_cycle-1_cycle_receipt.md` §2), and now owned by `kanban.md` row 18 (`decisions.md §27b` — no carve-outs survive), not by this register. This C2.5 slot is retired rather than reused, per this register's own "no unowned tidiness entries" convention (line 12) — a closed shape does not get a forward-scope row. | N/A — closed within SD-32; see `kanban.md` rows 11, 15, 18. |

## C3.x — Research-grade forward scope

| ID | Item | Owner |
|---|---|---|
| C3.1 | **How to ingest a system whose rules exist only as prose.** This is a research question, not an engineering one, and assuming it into SD-32's tool design would be the same error as the PDF reader (`decisions.md §5`). Two things are known and worth carrying forward: (1) The pipeline splits in two — *getting text out of a source is source-specific and does not transfer; turning prose into structured objects and shapes is shared regardless of source, and is the harder, riskier half.* (2) **The whole anti-gaming apparatus rests on a checkable source** — every "re-derive it yourself" instruction, every mutation proof, every GAMED verdict bottoms out in *the corpus says X, verifiably against a pinned SHA.* A prose source removes that foundation: the extraction **is** the corpus, and the extraction is the thing most likely to be wrong. The discipline does not port unchanged, and a replacement for pinned ground truth must be designed before a prose-sourced system is attempted. | A research spike. Not a bundle item until the discipline question is answered. |
| C3.2 | **The shape analyser as a portable procedure, validated.** SD-32 ships the procedure (`extract / normalise / cluster / count / report`) and the PF1e binding. A future cycle that runs the procedure against Traveller's `.dat` files or WoD's dice-pool rules would validate the portability claim — and likely surface a new vocabulary that the procedure still describes but the current binding does not capture. | A research spike, gated on at least one non-PF1e system being scoped. |
| C3.3 | **The d20pfsrd.com ground-truth comparison.** If it is ever used: it is Open Game Content (legally cleaner than a commercial PDF) but is a **subset of the books by construction** — Product Identity is excluded by design. For PF1e it is largely redundant: PCGen is better structured, pinned, and already the ground truth every gate depends on. Adding a second PF1e source creates a conflict-resolution problem that does not currently exist. **If tested at all, test it on one of the four uncompiled books first**, where the discrepancy between PCGen-as-missing and d20pfsrd-as-present would actually be informative. | A research spike. Not a bundle item until a real second-source need appears. |

## Carried forward from SD-31's forward-scope register

SD-31's `forward-scope-register.md` carries C1.8 (wire `v06_corpus_trap_report -- --audit` into
`scripts/verify.sh`) and C1.9 (`v06_work_inventory.rs`'s `enumerate_file` bare-basename
nested-citation bug). Neither was closed at SD-31 launch (2026-08-15); both inherit as SD-32's
outstanding carry-forwards, because SD-31's launch-readiness remediation did not own them.

| ID | Item | Owner |
|---|---|---|
| C1.8 (carry) | Wire `v06_corpus_trap_report -- --audit` into `scripts/verify.sh` as a real stage. SD-31 made the audit a real, non-vacuous gate (proved it can both fail — 3 real `wiring-class-mismatch` defects, fixed — and pass) but did not wire it into CI. **SD-32 must** decide and land the `scripts/verify.sh` stage itself: which books/kinds it runs the audit against corpus-wide, and how a future legitimate `wiring_class: "ambiguous"` / `no_corpus_line` record is told apart from a future real regression. | SD-32 (natural home: Gate 0 cycle, or a new protective epic — unassigned pending a cycle picking it up). |
| C1.9 (carry) | `v06_work_inventory.rs`'s `enumerate_file` shares the bare-basename nested-subdirectory citation bug two sibling fixes (`corpus_traps.rs`'s audit self-check, `gen_book_cache.rs`'s generator) already closed. **Consequence, unconfirmed:** any book with a real citation nested under a subdirectory may have its `wiring_class` / `wiring_class_reason` / `wiring_class_signals` silently misclassified in `docs/work-inventory.json` itself — the measurement board this package's own figures are re-derived from every cycle. **SD-32 must:** (1) fix `enumerate_file`'s `rel` derivation the same way the other two call sites were fixed, or extract one shared helper; (2) re-measure via the guarded regen procedure; (3) report the board's movement via `doneness_verdict()` replayed at both ends. | SD-32 (unassigned pending a cycle picking it up — natural home: whichever gate next needs a fresh, trusted `docs/work-inventory.json` regen; the Gate 0 census walk is the latest point this must be resolved by). |

No item above is left without a stated home, per this program's standing "unowned deferral is
not a valid disposition" discipline (`SD-30-.../decisions.md §27`).
