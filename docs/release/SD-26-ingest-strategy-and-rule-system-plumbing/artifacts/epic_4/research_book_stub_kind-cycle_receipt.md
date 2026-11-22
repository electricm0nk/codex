# Cycle research_book_stub_kind — Epic 4 / Criterion 4.1

- **Card ID:** (see kanban step, below)
- **Commit SHA:** (filled in after push — see `progress.md` for the landed SHA)
- **Files touched:**
  - `docs/governance/wired-integration-stubs-registry.md` (added "The `book_stub` kind" subsection defining the adapted field set + entry #0003; bumped `last_reviewed_at` to 2026-07-22; updated the reserved-entries footer note)
  - `data/stubs/advanced_race_guide.json` (new — pilot per-book stub manifest)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** Epic 4, Criterion 4.1 — Research epic: define the `book_stub` kind in the Stubs Registry. Defines the metadata fields; validates against an existing stub (`epic-breakdown.md` §Criterion 4.1).
- **Status:** complete
- **Notes:**
  - **Field-set design.** `book_stub` reuses #0001/#0002's seven-field entry shape (it is a
    *kind* of the same registry, not a new registry), swapping the two file-specific fields for
    data-specific equivalents: `File / line` → `Book / manifest path`; `Stub pattern` → `What's
    missing`. The other five fields (`Justification`, `Audit-grep impact`, `Bundle-of-record`,
    `Remediation cycle`, `Status`) carry over unchanged in label, with `book_stub`-specific
    guidance on what each should say (see the new "The `book_stub` kind" subsection). `Status`
    for `book_stub` reads "Registered stub `<date>`" rather than "Accepted `<date>`" — a
    deliberate wording distinction from #0001/#0002 (there is no code to "accept"; the entry
    just records a data gap the operator has already scoped out of this bundle). This is a
    judgment call, not dictated by any source doc; flagging it explicitly for the next 21 cycles
    to follow consistently.
  - **JSON manifest shape.** Used `content-unit-inventory.md §2.1`'s shape exactly, no deviation:
    `{book_id, book_name, planned_resolution_bundle, content_kind_counts: null,
    registered_at: <ISO-8601>}`. Confirmed by direct JSON-schema assertion (see Verification
    below).
  - **`planned_resolution_bundle` value — discrepancy found and resolved per explicit brief
    instruction.** `decisions.md §10` ("Operator-deferred shape decisions") pins this field to
    the literal string `"SD-27"` as the operator-pinned default. This cycle's brief explicitly
    instructed `"SD-27+ (unscheduled)"` instead, citing `risks-and-open-questions.md §5`'s
    documented deferral posture ("concrete rule-system implementations land in SD-27+" — an
    open-ended, not-yet-scheduled deferral, not a commitment to a specific numbered bundle).
    Followed the brief's explicit instruction (more specific and more recent than `decisions.md
    §10`'s general default, and better matches the no-fabrication doctrine — `decisions.md §10`
    itself says "operator may override"). **Flagging this discrepancy for the operator/next
    cycles:** `decisions.md §10` should probably be corrected to `"SD-27+ (unscheduled)"` to stay
    consistent with what's now actually landed in the registry + all 21 forthcoming manifests,
    or the operator should confirm `"SD-27"` (a real, scheduled bundle) is in fact intended, in
    which case this pilot entry + `decisions.md §10`'s literal text should be reconciled the
    other way. Not self-healable inline (needs an operator call); did not block this cycle since
    the brief's instruction is unambiguous and directly on-point.
  - **Operator-verbatim citation.** Used `README.md §3`'s quoted phrase — "honors the operator's
    'in-scope books no stubs, future-state books knowingly stub' doctrine" (operator directive
    2026-07-21 17:39:26) — as the entry's justification citation, per the brief's explicit
    instruction to cite it rather than paraphrase it away. This is README's own quoting of the
    operator directive (the raw operator utterance itself is not reproduced verbatim anywhere in
    the bundle's docs beyond this quoted fragment and the adjacent "ready to go, durable
    artifact..." quote); cited as the closest available operator-verbatim text per the registry's
    "operator's verbatim directive is required" rule.
  - **Pilot book choice.** Used `advanced_race_guide` — the brief's explicit pilot choice, and
    also the first book alphabetically in `content-unit-inventory.md §2.2`'s 21-book list, so
    criteria 4.2-4.22 have an unambiguous "which book did 4.1 already do" answer (4.2 should pick
    the next book in the list, `adventurers_guide`, not re-do `advanced_race_guide`).
  - **Validated the pattern is mechanically repeatable (per brief step 4):** the #0003 entry's
    seven fields are all book-agnostic in structure — every field's content follows directly
    from `{book_id, book_name}` (known in advance from `content-unit-inventory.md §2.2`'s list),
    the shared operator-verbatim citation (same directive covers all 21 books), and the shared
    `planned_resolution_bundle` value (`"SD-27+ (unscheduled)"`, same for all 21 per
    `decisions.md §10`'s "operator-pinned default" framing, now corrected per the discrepancy
    note above). No per-book design decision remains open; criteria 4.2-4.22 can copy this
    entry's template field-by-field, substituting only `book_id`/`book_name`/entry number/
    criterion number/date. No gap found requiring resolution.
- **Discovery forwards:** the `decisions.md §10` / brief `planned_resolution_bundle` value
  discrepancy noted above (`"SD-27"` vs. `"SD-27+ (unscheduled)"`) — forwarded to `progress.md`
  `## DISCOVERED`.
- **Next-cycle plan:** Criteria 4.2-4.22 each pick the next book from
  `content-unit-inventory.md §2.2`'s 21-book list (alphabetical order, `advanced_race_guide`
  already done here), write `data/stubs/<book_id>.json` following this cycle's exact shape, and
  add the next-numbered registry entry (#0004, #0005, ...) following this cycle's exact
  `book_stub` field template. All 21 can dispatch in parallel per `epic-breakdown.md`
  ("Concurrency: `parallel: yes` after 4.1"), since each touches a disjoint `data/stubs/<book>.json`
  file and appends (not edits) to the shared registry doc — the only shared-write contention is
  the registry doc's append point and `progress.md`, both covered by the standard concurrent-write
  retry protocol.
