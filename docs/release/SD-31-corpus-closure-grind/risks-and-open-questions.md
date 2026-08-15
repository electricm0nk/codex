# SD-31 Risks and Open Questions

Rewritten 2026-08-15 after `decisions.md §2` absorbed SD-32 and re-sequenced the epics. The two
largest risks this file previously carried — the cross-SD race-chassis dependency and cross-package
concurrency — were **structural consequences of the split, and the merge removed them**. They are kept
below as resolved entries, because a risk that was retired by a decision is more useful than a risk
that silently disappears.

## Primary risks

1. **PI-gate citation drift.** Epics 5/6/7 depend on `SD-30-class-feature-archetype-bundle`'s Epic 3
   (PI-screening) staying current. SD-30 closed with that gate `COMPLETE`, but if its Epic 3-F4
   regression gate ever goes red for a book, a cycle here that cited an earlier `COMPLETE` receipt
   without re-checking is claiming against a stale gate. **Mitigation:** `AT-31-003` requires citing
   the specific receipt for the specific book, not "the PI gate is generally clean," and requires the
   production path to actually call the readers (`G1.4`/`G1.5` contracts).
2. **A capability gate opened too early.** Epic 6-F3/F4 open **per race batch** as Epic 1 delivers, and
   Epic 3-F4/Epic 5-F3 open when Epic 2 completes. A cycle that reads "Epic 1 is in flight" as "the
   gate is open" will ingest against a chassis that does not cover its book's races and produce records
   that cannot ground. **Mitigation:** the gate state is a named race-batch list in `kanban.md`, written
   by Epic 1-F3 as each batch lands — not an epic-level status word.
3. **The reachability audit becoming a report nobody acts on.** Epic 0's output is only useful if a
   dead-end it names gets an owner. **Mitigation:** `decisions.md §4` requires every dead-end to be
   assigned to an epic or proposed to the Structural Exclusion Register, and Epic 9 cannot close over
   an unowned one.
4. **Exclusion-register creep.** The register exists so genuinely-unreachable units can be excluded
   honestly. The failure mode is using it as the deferral hatch it replaced. **Mitigation:**
   `decisions.md §3` requires the proving command, the named missing capability, an Epic 0 audit run,
   and **operator sign-off** — and states explicitly that cost is never an exclusion reason.
5. **Shared-checkout collision with SD-30's still-open promotion PR.** `tranche/10` carries SD-30's
   closure and PR #363, unmerged. A cycle here commits to the same branch. **Mitigation:** the standing
   shared-checkout discipline (`git status` before every git write, never `git add -A`, never
   `git stash`) applies unchanged; the operator holds the merge.

## Resolved by `decisions.md §2` (kept as record, not live risks)

- ~~**Race/race_trait chassis dependency on SD-32.**~~ Resolved: the race chassis is Epic 1 of this
  package and runs before the lanes consuming it. There is no sibling package to check.
- ~~**Concurrency collision with SD-32.**~~ Resolved: no SD-32 exists. Concurrency inside this package
  is governed by the file-disjointness stated in `epic-breakdown.md` (capability track vs.
  `class_feature` track) and the standing per-checkout writer rule.

## Open questions

1. ~~**Do `equipment`/`equipment_modifier`/`companion`/`feat`/`monster_ability` get cards?** They are
   in the 100 % denominator but no card claims them (`forward-scope-register.md G1.3`). Epic 0's audit
   will surface their reachability, and Epic 9 cannot close over them silently — but whether they open
   as Epic 6 cards or as their own epic is not yet decided. **This is the most likely source of a
   late scope surprise in this package**, and it is named here rather than discovered at closure.~~
   **RESOLVED 2026-08-15 (launch-readiness remediation Step 2, blocker B2, operator ruling "open
   cards for the six unowned kinds").** They open as Epic 6 cards, not a new epic: `equipment` (F5),
   `equipment_modifier` (F6), `companion` (F7), `feat` (F8, routes SD-30 E0-F3's 217-unit
   probe-fixture residue), `monster_ability` (F9). A **sixth kind, `class`** (158 not-done units,
   `computed|not-ingested` dominant), found by the readiness review and named nowhere in this package
   before this correction, also opens as Epic 6-F10. Epic 6-F11 (held static/derived residual, 5,273
   units) and Epic 5-F4 (the 36 `deferred-with-reason` units) close the two remaining un-carded
   populations the same review found. See `epic-breakdown.md` Epic 6 F5-F11 and Epic 5-F4;
   `forward-scope-register.md G1.3` updated in step.
2. ~~**Does this package run its own Bundle Code Review, or rely on SD-30's Epic 8?** SD-30's Epic 8
   reviewed SD-30's diff, which does not include this package's work. Currently unresolved; the
   conservative reading is that Epic 9 needs a review step of its own before the exit gate. Flagged,
   not decided.~~ **RESOLVED 2026-08-15 (launch-readiness remediation Step 5, drift D12).** Yes: this
   package runs its own review, **Epic 9-F3** (`epic-breakdown.md`), scoped to this package's own diff
   against its branch point (the SD-30 split commit), not SD-30's already-reviewed diff. Shape follows
   `SD-30-.../epic-8-code-review`'s three parallel read-only dimensions
   (correctness/no-stub/reach, test quality, doc-fact accuracy), then an adversarial verify pass that
   attempts to refute each finding before it is accepted, then per-finding disposition
   (`fixed-in-bundle` or `deferred` to `forward-scope-register.md`). Epic 9-F1's exit-gate acceptance
   now requires F3 `COMPLETE` before the closure receipt is valid.
