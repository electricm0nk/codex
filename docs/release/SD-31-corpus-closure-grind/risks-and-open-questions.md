# SD-31 Risks and Open Questions

## Primary risks

1. **Cross-SD gate drift.** This package's Epic 3/4/5 depend on `SD-30-class-feature-archetype-bundle`'s
   Epic 3 (PI-screening) staying current. If SD-30 re-opens a book's PI screen (a regression found by
   its Epic 3-F4 regression gate), a cycle here that cited an earlier `COMPLETE` receipt without
   re-checking is now claiming against a stale gate. Mitigation: AT-31-003 requires citing the specific
   receipt, not just "PI gate is generally clean."
2. **Race/race_trait chassis dependency on SD-32.** Epic 4-F3/F4's ceiling (513/3,447 `race_trait`,
   0/103 `race`) is real and low until SD-32's race-chassis epic lands. A cycle that assumes the chassis
   exists without checking `SD-32-engine-capability-builds/kanban.md` first will misreport its own
   ceiling.
3. **Concurrency collision with SD-30's Epic 0 and SD-32.** All three packages can run concurrently
   (file-disjoint by design), but a cycle here touching `src/rules_core/rules_tables/<book>/` for a
   book SD-30's Epic 0 or SD-32's classifier work is also reading (not writing) could still race on a
   shared regeneration step (`cargo run --locked --bin v06_work_inventory`). Mitigation: the standing
   shared-checkout discipline (`git status` before every git write, no `git add -A`) applies unchanged.

## Open questions

1. **Does Epic 4-F3/F4's ceiling re-derivation happen automatically, or does a cycle have to notice
   SD-32 landed?** Not yet decided — no automated cross-SD trigger exists in this repo's tooling.
   Current answer: manual check, per `loop-instruction.md` override 5, until/unless the operator funds
   a cross-SD notification mechanism (out of scope for either package unless explicitly requested).
2. **Does this package run its own Bundle Code Review, or defer to SD-30's Epic 8?** Currently deferred
   to SD-30's Epic 8 (see `README.md` "Out of scope") on the theory that one whole-program review is
   cheaper than three duplicate ones. If the three packages' closures end up badly desynchronized in
   time, this may need revisiting — flagged, not decided here.
