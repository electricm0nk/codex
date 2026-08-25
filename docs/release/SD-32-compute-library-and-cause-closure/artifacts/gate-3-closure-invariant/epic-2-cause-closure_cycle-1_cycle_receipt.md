# Cycle 1 — Epic 2 (cause closure) / Card 11 `epic-2-cause-closure`

- **Card ID:** `epic-2-cause-closure`
- **Commit SHA:** `185027717`
- **Files touched:** `apps/desktop/src-tauri/src/reach_gate.rs` (one new test,
  `dispatch_gap_race_and_monster_families_all_have_book_level_reach_arms`),
  `docs/retro/events/epic-2-cause-closure.jsonl` (new — 1 correction, 3 deferrals),
  `kanban.md` (card 11 → complete), `progress.md` (this entry).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
  (`git diff --unified=0 1bb523773d32705d1b7387fd4c494861523f55ba...HEAD -- apps/desktop/src-tauri/src/reach_gate.rs`
  — no `sd[0-9]+_`/`SD[0-9]+_`/`Sd[0-9]+`/`t_[0-9a-f]{8,}` matches).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff — no
  `STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack` tokens).
- **Acceptance criterion:** `acceptance-and-verification.md` AT-32-E2-001 — "Cause closure closes
  by class, not by instance. The eight measured blocker shapes (T2a, T2b, T9, T4, T12, T5, T1, T3)
  are each closed corpus-wide rather than instance-by-instance. T5 is credited via Epic 4's card 4
  and T3 via Epic 5's card 1... T8/T7 close opportunistically... T10 has no unit count and is a
  census-process item." This cycle's own scope, per `kanban.md` card 11's note.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`,
  `PCGEN_ORACLE_SHA`) — read from the repo-local slot, re-derived via
  `scripts/verify.sh --only preflight-oracle` (fresh worktree, empty slot, self-healed per §8 by
  `scripts/fetch-pcgen-oracle.sh --dest <repo-local pcgen slot>`; PASS after fetch).
- **Status:** complete (T1 closed this cycle; T5/T3 cited, not re-closed; T2a/T2b/T9/T4/T12/T7/T8
  scoped and deferred with named next-cycle plans — see Notes)
- **Notes:**

  **What this cycle closes: T1 (dispatch gap / "Monk shape"), corpus-wide, all three kinds.**

  `epic-breakdown.md` Epic 2's T1 row states: "classes **0** (exhausted); ... Race/monster **never
  fully checked** — 280 monster entries and 31 non-CRB races unexamined," citing
  `SD-31-corpus-closure-grind/todo/sweeps.md` S1 (CLOSED for classes)/S2 (PARTIAL for races/
  monsters) and `artifacts/MEASURE-TWICE.md`'s own T1 row.

  Investigation (re-derived, not transcribed):
  - Equipment leg: the SD-31-cited live instance (APG's `EquipmentCategory` enum omitting the
    `Equipmods` variant against 35 real records) is fully mitigated already —
    `grep -c 'book: "APG"' src/rules_core/rules_tables/equipment_gap_tables.rs` → `37`, all
    `category: "Equipmods"`, served through `equipment_resolver.rs`'s
    `equipment_gap_tables::equipment_gap_rows()` chain. Zero units silently lost. No code change
    needed; this leg was already closed by existing infrastructure.
  - Race/monster leg (the genuinely unexamined 280+31): traced the architecture rather than
    hand-checking 311 records one at a time. `RaceId::ALL`/`beastiary1::MonsterId::ALL` are
    **hand-authored enums that exist only for the CRB race roster (7) and the Bestiary-1
    hand-modelled monster roster (46)**. Every other race/monster (the 31 non-CRB races, the 280
    Bestiary-1 chassis-served monsters, and all monsters in the other 12 monster books) is served
    through a **corpus-derived** path with no separate hand-authored id table:
    `race_catalog::ingested_race_ids_for_book` reads `corpus.race_keys()` and filters by
    `book_id` (`apps/desktop/src-tauri/src/race_catalog.rs:416-429` — no enum, no hand list);
    `rules_core::rules_tables::bestiary::mod.rs`'s doc comment states its 280-monster complement
    is "not maintained by hand... derived from the other table's own shipped records." The
    Monk-shape defect requires a *complete hand-authored table* with a *separate, missing*
    string→id dispatch link. A corpus-derived path has no second table to be missing an entry
    from — it is structurally immune to this specific defect shape (a different failure mode,
    "the corpus-derivation itself silently drops rows," is possible in principle, which is why the
    new test below checks the derived sets are non-empty rather than only checking book presence).
  - Confirmed `reach_gate.rs` already runs a book-level version of exactly this check, generically,
    for every kind (`every_ingested_family_is_accounted_for`, corpus-inventory-driven, not
    hand-listed) and a record-level reachability claim per book
    (`every_declared_claim_actually_carries_the_records`), and that both already pass at HEAD
    (`scripts/verify.sh --only reach` reported `PASS 30/30` as of Cycle 3's own receipt, unchanged
    by this cycle). Verified book coverage is complete, not assumed: `data/corpus/*/race` has
    exactly 6 book directories (`core_rulebook, beastiary, bestiary_2, bestiary_5, bestiary_6,
    advanced_race_guide`) and `data/corpus/*/monster` has exactly 13
    (`beastiary, bestiary_2, bestiary_3, bestiary_4, bonus_bestiary, book_of_the_damned_volume_1,
    book_of_the_damned_volume_2, horror_adventures, inner_sea_bestiary, inner_sea_gods,
    inner_sea_world_guide, monster_codex, ultimate_psionics`) — both counts match `reach_gate.rs`'s
    own `("<book>", "races")`/`("<book>", "monsters")` match arms exactly (verified by reading the
    match block, `apps/desktop/src-tauri/src/reach_gate.rs:1280-1606`).

  **New test, RED→GREEN.** Added
  `reach_gate::tests::dispatch_gap_race_and_monster_families_all_have_book_level_reach_arms`
  (`apps/desktop/src-tauri/src/reach_gate.rs`) so this closure is a standing, named assertion
  rather than a one-time argument in a receipt — a future regression that narrows either kind back
  toward its hand-modelled subset fails here, by name, instead of silently. It asserts, freshly
  derived from `corpus_inventory()` (never a literal book list): (1) at least 6 race families and
  13 monster families exist in the real corpus; (2) every one has a `reach_of` dispatch arm (the
  book-level form of the Monk shape); (3) for the four furthest-from-hand-modelled race books
  (`bestiary_2`, `bestiary_5`, `bestiary_6`, `advanced_race_guide`), `ingested_race_ids_for_book`
  returns a non-empty, corpus-derived set (catching the "derivation itself silently drops every
  row" failure mode a bare presence check would miss).

  RED→GREEN: temporarily raised the race-book-count assertion to `>= 999` and re-ran; failed for
  the intended reason (`"found 6: [...]"`, the real 6 named exactly) — command:
  `CARGO_TARGET_DIR=... cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml
  --bin codex-desktop dispatch_gap_race_and_monster_families_all_have_book_level_reach_arms`.
  Reverted to the real threshold, re-ran green (1/1). No regression: the full `reach_gate::` module
  (31 tests, up from 30) and the full desktop `--bins` suite (516 tests, up from 515) both pass,
  0 failed.

  **What this cycle cites rather than re-closes (per card 11's own note, AT-32-E2-001):**
  - **T5** (422 units, book-level missing-`RuleSetId` gate) — credited via card 4's own receipt
    (`artifacts/gate-0-census-closure/002_cycle_receipt.md`, `progress.md` Cycle 3): all four books
    landed their first compiled `RuleSetId`, arithmetic re-derived and matching (3+335+20+64=422).
  - **T3** (3 of 29 Rust generators vulnerable to self-erasure, 17 never reached) — credited via
    card 1's own receipt (`artifacts/epic-5-protective-sweep/cycle-1_cycle_receipt.md`,
    `progress.md` Cycle 1): population re-verified 29, 7 vulnerable generators fixed (the 2 SD-31
    D9 binaries plus 5 more found in the "17 never checked" bucket).

  **What this cycle scopes and defers, with a named next-cycle plan (not attempted, not claimed
  closed):**
  - **T2a** (8,243 units, `data.class` read from wrong place) — `MEASURE-TWICE.md` itself states
    only 2,360 of the 8,243 are cleanly prefix-remappable; the other 5,883 are "a MIX of
    category-label plumbing and genuine unmodelled-class content" needing per-value
    re-examination before either bucket is credited, and 1,354–2,124 of those overlap **T12**
    directly (Psychic, Vigilante, Medium, Magus, Shifter, Kineticist, Spiritualist, Occultist need
    real modelling, not relabelling). Closing T2a and T12 independently would double-count or
    under-count the overlap; they need one combined cycle, not two separate half-measures.
  - **T2b** (2,472 units, race-trait compound-key matcher), **T9** (2,651 units, per-record book
    onboarding backlog across spell/companion/feat/monster_ability/equipment/monster), **T4** (up
    to 2,763 units, built-but-unreachable render surface — `epic-breakdown.md` flags a prior
    wave's own 471-unit claim as false, "true reachable count of zero," so this needs the real
    driver re-run before any number is trusted) — none investigated this cycle; each is a
    multi-thousand-unit population on its own.
  - **T7** (D12, 4 units, shallow single-hop archetype-grant traversal) — investigated; the fix
    site is identified (`load_raw_grant_facts`'s `granted_via_archetype` CATEGORY-text derivation,
    `src/rules_core/pilot_compute/class_feature_grant_consumer.rs:374`, distinct from the
    already-multi-hop `resolve_pcgen_var_chain` in the same file) but not implemented — logged as
    a `scripts/retro.py deferral`.
  - **T8** (D13, 12 units, `wiring_class`-vs-`status` classifier blind spot) — **scope-boundary
    finding, not a difficulty deferral.** The fix site is
    `scripts/observer/pf1e_dashboard_producer.py`'s `doneness_verdict()`/`wiring_class`
    classifier, which `technical-design.md`'s own "What this bundle does not touch" section names
    as SD-30's Epic 0 surface, read-only from SD-32. `AT-32-E2-001` lists T8 as "opportunistic" but
    closing it as literally written would require editing a file this bundle is explicitly scoped
    not to own. Logged as a `scripts/retro.py deferral`, flagged for an operator ruling or a
    successor bundle that owns the producer.
  - **T10** (unverified proxy measurement, no unit count) — per card 11's own note and
    `AT-32-E2-001`'s text, this is a census-process item, not something this card closes; left
    untouched, as scoped.

  This is a first, closed cycle of what card 11 needs — not the whole card. The precedent for a
  multi-thousand-unit epic needing more than one cycle per card is Gate 2 itself (cards 6/7/8,
  three separate cycles for a narrower ten-family scope than Epic 2's eight blocker shapes).
  `kanban.md`'s `Status` column is left as this cycle's own honest state (see below), not marked
  fully `complete` for the whole card — see the kanban entry for the precise wording.

- **Discovery forwards:** none requiring a new card (T7/T8/T2a/T2b/T9/T4/T12 are named,
  scoped deferrals against the existing card 11, not new-scope discoveries).
- **Next-cycle plan:** pick one of T2a+T12 (combined, per the overlap above), T2b, T9, or T4 as the
  next cycle's sole target — each on its own likely needs the same shape Gate 2 used (a
  measurement/scoping pass, then a fixture-checked corpus-wide close), not a single-pass fix. T7
  (4 units) is the cheapest remaining item and a reasonable opportunistic pickup if a cycle has
  spare scope after its own primary target. T8 needs an operator ruling on write-scope before any
  cycle can touch it.
