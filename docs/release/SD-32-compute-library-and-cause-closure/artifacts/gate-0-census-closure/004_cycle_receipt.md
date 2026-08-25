# Cycle 004 — Epic 3 (class reachability), second half / Criterion AT-32-E3-001

- **Card ID:** `epic-3-class-reachability`
- **Commit SHA:** `3362acb00` (rebased onto `origin/tranche/12` at push time; implementation +
  receipt landed in one commit).
- **Predecessor cycle:** `003_cycle_receipt.md` closed the mechanism-and-proof (prestige-entry-gate)
  half of this same criterion and explicitly deferred this half — cited, not re-closed, per
  `decisions.md §10`.
- **Files touched:**
  - `scripts/census_untabled_base_classes.py` (new) — the committed, deterministic re-derive
    command for the population this cycle wires: walks `$PCGEN_CORPUS_ROOT`, finds every
    `CLASS:<name>` line whose `TYPE:` field contains `Base` and `PC` but not `NPC`/`Prestige` and
    whose name doesn't start with `Ex-` (operator ruling B5, still open), keeps only names anchored
    in a book this repo has ingested, drops any class already recognized by one of the five existing
    `compute_class_chassis` dispatch families (CRB/APG/ACG/PU/UC, transcribed once from their own
    source as the exclusion allowlist), and extracts hit die / MAXLEVEL / BAB progression / good-poor
    save classification straight from each class's real `BONUS:COMBAT|BASEAB|...` and
    `BONUS:SAVE|BASE.<X>|...` fields.
  - `tests/fixtures/rules_core/untabled-base-class-chassis.json` (new, generated) — the 20-entry
    registry `include_str!`-embedded by `untabled_base_class_chassis.rs`.
  - `src/rules_core/pilot_compute/untabled_base_class_chassis.rs` (new) — loads the fixture,
    reuses `rules_tables::crb::class_tables`'s own `base_attack_bonus`/`save_bonus` formula
    functions (widened from private to `pub(crate)`, not re-derived) to compute a real chassis row
    per `(class_id, level)`. 8 unit tests (registry population, exclusion-set non-overlap,
    unknown-class / level-0 / beyond-ceiling `None`, two oracle-fixture-checked classes at
    opposite ends of the BAB-progression and save-classification space).
  - `src/rules_core/rules_tables/crb/class_tables.rs` — `BabProgression`, `base_attack_bonus`, and
    `save_bonus` widened from private to `pub(crate)` so the new module can reuse them instead of
    re-declaring a second, independently-maintained copy of either formula. No behavior change to
    any existing caller.
  - `src/rules_core/pilot_compute/mod.rs` — one new `else if` arm inside `compute_class_chassis`'s
    single-class dispatch, between the Ultimate-Combat arm and the prestige-entry-gate arm added by
    cycle `003`. Unlike the prestige arm, this one produces a real chassis magnitude (base attack
    bonus + all three base saves), pushing the same four `class_chassis.*` explanation ids
    `compute_generic_table_chassis` already uses. New `untabled_base_class_chassis_wiring_tests`
    module (3 tests) proving the arm runs through the real `compute_pilot_base_chassis` →
    `compute_class_chassis` call site for `class:kineticist`, not a direct unit call.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
  (`git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <scoped paths> | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` → no match)
- **Wired-integration audit result:** `OK_NO_TOKENS`
  (`git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <scoped paths> | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` → no match)
- **Acceptance criterion (verbatim, `acceptance-and-verification.md`):**
  - **AT-32-E3-001 — Class reachability.** "(Epic 3.) The 77 prestige classes have
    entry-requirement gating that exists nowhere in the codebase today; the cycle that builds it
    cites the `compute_class_chassis` call site and proves the gating runs (fixture-checked, of
    course). The 18 real base classes without tables and the 28 books-without-ruleset both feed
    this epic from Epic 4." — **MET in full now**: the prestige half (cycle `003`) plus this
    cycle's real-base-class-table half together close both named populations this criterion
    describes. Fixture-checked (of course): every emitted BAB/save/hit-die value is corpus-derived
    and two classes (`class:psion`, `class:kineticist`, plus `class:antipaladin` in the module's own
    unit tests) are checked against bytes hand-transcribed from the oracle's `up_classes.lst` /
    `oa_classes.lst` / `apg_classes.lst`, not from this cycle's own fixture file.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`), used
  by `scripts/census_untabled_base_classes.py`'s live run.
- **Status:** complete
- **Notes:**
  - **Population is 20, not 18 — corrected, not silently substituted.** `epic-breakdown.md` Epic 3
    and `AT-32-E3-001` both quote 18. `scripts/census_untabled_base_classes.py`'s own run (stderr
    `population=20`) finds 20: Aegis, Antipaladin, Cryptic, Dread, Kineticist, Magus, Marksman,
    Medium, Mesmerist, Occultist, Psion, Psychic, Psychic Warrior, Shifter, Soulknife, Spiritualist,
    Tactician, Vigilante, Vitalist, Wilder — spanning `advanced_players_guide` (Antipaladin),
    `occult_adventures` (6), `ultimate_magic` (Magus), `ultimate_wilderness` (Shifter),
    `ultimate_intrigue` (Vigilante), and `ultimate_psionics` (10). Every one of those six books
    already carries a `RuleSetId` variant (`Apg`, `Oa`, `Um`, `Uw`, `Ui`, `Upsi` —
    `src/rules_core/rules_tables/mod.rs`), so none of these 20 is part of the "28
    books-without-ruleset" Epic 4 population; this is genuinely Epic 3 scope, not a book-onboarding
    dependency. Logged as `scripts/retro.py correction`
    (`epic-breakdown.md Epic 3 / AT-32-E3-001`, id `1787446598014-epic-3-untabled-base-classes-f7789f`),
    following the identical pattern cycle `003`'s own 77→62 correction used.
  - **Two extraction bugs caught and fixed during this cycle's own build, not after.** (1) A
    multi-column `BONUS:SAVE|BASE.Fortitude,BASE.Reflex|...` field's second-and-later columns
    still carried their `BASE.` prefix after a naive single strip, silently dropping every class
    whose saves formula named two columns on one line (population 8, not 20, on the first run) —
    fixed by stripping the prefix per-column after the split, not once before it. (2) Four classes
    (Psion, Psychic Warrior, Soulknife, Wilder) also exist, under the identical name, in
    `psionics_unleashed`/`psionics_expanded` -- Dreamscarred Press books this repo has **not**
    ingested -- and `os.walk`'s non-deterministic visit order sometimes reached that uningested
    occurrence first, silently dropping the class even though its real `ultimate_psionics`
    occurrence (which this repo HAS ingested) existed (population 16, not 20, on the second run) --
    fixed by collecting every candidate path per class name and preferring the one anchored in an
    ingested book, rather than trusting the first match `os.walk` happens to visit. Both bugs were
    caught by comparing the script's own output against a hand-verified probe of the same corpus
    text before this cycle trusted the fixture — the fixture-discipline habit applied to the
    generator script itself, not only to its output.
  - **Formulas reused, not re-derived.** `base_attack_bonus`/`save_bonus` in
    `rules_tables::crb::class_tables` are the same two functions SD-18's test suite already
    verified; this cycle widened their visibility (`pub(crate)`) rather than writing a second copy,
    the same "extend the proven machinery" instruction the dispatch brief asked for.
  - **Vigilante's dual BAB line, resolved mechanically.** Vigilante's corpus record carries two
    `BONUS:COMBAT|BASEAB` lines gated by a `VigilanteFullBAB` talent toggle
    (`PREVAREQ:VigilanteFullBAB,0` / `,1`). The census script keeps the `,0` (toggle-off, default)
    line — a character has not taken the alternate-BAB talent unless they chose it — which
    classifies Vigilante ThreeQuarter BAB, matching its corpus default.
  - **Antipaladin's chassis numbers happen to equal Paladin's** (full BAB, Fortitude/Will good,
    Reflex poor, d10) — a real corpus fact, not a shortcut: this cycle still built Antipaladin a
    fully independent registry row and dispatch path rather than aliasing it to
    `ClassId::Paladin`, and a dedicated unit test (`antipaladin_level_1_matches_the_oracle_s...`)
    pins the value against hand-transcribed oracle bytes specifically so a future edit to
    `CLASS_META`'s Paladin row could never silently move Antipaladin's numbers too.
  - **RED→GREEN preserved for the wiring proof.** The two `class:kineticist`
    `untabled_base_class_chassis_wiring_tests` value-assertion tests were run RED first by
    temporarily replacing the new dispatch arm's condition with
    `None::<untabled_base_class_chassis::UntabledBaseClassRow>` (2 of 3 wiring tests failed —
    `base_attack_bonus`/saves stayed at their pre-wiring `0` instead of the real corpus-derived
    values; the third, `unregistered_class_id_still_falls_through_to_unsupported`, correctly stayed
    green throughout since it exercises the untouched `else` fallthrough), then restored and
    re-run GREEN — both runs captured live in this cycle, not asserted from memory.
  - **Full suite still green.** `cargo test --lib` (2375 passed, 13 ignored, 0 failed) after this
    cycle's changes — no pre-existing test asserted `class_chassis.unsupported` for any of the 20
    newly-dispatched class ids, so nothing regressed by making them reachable.
- **Discovery forwards:** none — this closes the deferral cycle `003` logged
  (id `1787441736902-epic-3-class-reachability-5a53e0`), and the 28-books-without-ruleset population
  this criterion also names remains Epic 4's own scope, unchanged by this cycle.
- **Gate-wrap-up retro-summary note (workflow-instruction.md §12 step 1):**
  `scripts/retro.py summary --since 2026-08-22 --json`, read (not just run). For this cycle's own
  actor (`epic-3-untabled-base-classes`): 1 verification-fail event (`preflight-oracle` on the
  fresh worktree before the oracle was fetched — self-healed via `scripts/fetch-pcgen-oracle.sh`,
  the documented remediation, not an incident), 1 correction (18-vs-20 population figure). No
  recurrence key fired more than once within this actor's own window.
- **Next-cycle plan:** none open for this criterion — AT-32-E3-001 is now met in full (both named
  populations closed). Card 12 moves to `complete`. The registry's own doc comment notes a natural
  follow-on (widening `pre_tokens.rs` for the currently-`Unmodelled` PRE-token kinds cycle `003`
  surfaced), but that is prestige-gate scope (card 11 territory), not this card's.
