# Cycle 001 — Gate 0 census closure / Criterion AT-32-G0-001, AT-32-G0-002

- **Card ID:** `gate-0-census-closure`
- **Commit SHA:** `58726ddfcc19d438d78af4f92ef978aff0f367e4` (rebased at push time; the implementation
  commit. `fd67946831f4a915df80f4ecb0050e61d68d97bc` is the follow-up commit recording this SHA.)
- **Files touched:**
  - `scripts/census_independent.py` (new — the independent walker, reader/analyser/reporter)
  - `scripts/tests/test_census_independent.py` (new — 11 unit tests)
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/diff.json` (new — generated)
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/excluded-directories.md` (new — generated)
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/object-definition-rules.md` (new — hand-authored summary of the script's own rules + live figures)
  - `docs/retro/events/gate-0-census.jsonl` (new — retro log for this actor)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion (verbatim, `acceptance-and-verification.md`):**
  - **AT-32-G0-001.** "Given the 158-book PCGen oracle directory tree at the pinned SHA... When the
    new independent walker (`scripts/census_independent.py`, Gate 0 deliverable) runs against it.
    Then the per-book diff against the inventory's 37-book roster reaches zero-unexplained: every
    excluded directory is named and justified as scope (not oversight) in
    `artifacts/gate-0-census-closure/excluded-directories.md`." — **MET.** `unexplained=0`.
  - **AT-32-G0-002.** "Given the inventory's current `docs/work-inventory.json` denominator of
    38,372 units (re-derived at Gate 0's first cycle, never transcribed from a prior wave). When an
    honest object-definition rule is written for each kind... covering `.MOD` continuations,
    `.COPY=` derivations, and template rows. Then a 'kind-unenumerable' category, if any exists, is
    named and counted — not pretended to be zero." — **MET.** Object-definition rules written
    (`object-definition-rules.md`); `.MOD`/`.COPY=`/template rows all handled with a stated rule;
    kind-unenumerable is real and large (27,847 units, 11 named buckets) and reported, not hidden.
- **Corpus SHA:** `grep PCGEN_ORACLE_SHA scripts/pcgen-oracle-pin.env` → see that file for the
  literal value; the live run above used the repo-local oracle slot at that pin
  (`scripts/verify.sh --only preflight-oracle` → PASS, run at cycle start).
- **Status:** complete
- **Notes / judgment calls:**
  1. **158 vs 186 book directories.** `acceptance-and-verification.md`, `technical-design.md`, and
     `scope-draft.md` all state "158-book PCGen oracle directory tree" with no command anywhere in
     the bundle that reproduces 158. This walker's own reproducible definition (immediate children
     of the five `paizo/` product categories, or of any non-`paizo` publisher directory, filtered
     to those recursively containing >=1 `.pcc` file) yields **186**, verified against the live
     pinned oracle. Logged as a correction
     (`scripts/retro.py correction --subject scope-draft.md --claimed 158 --actual 186
     --verified-by "python3 scripts/census_independent.py ..."`) rather than silently reconciled.
     I did not force-fit a narrower directory definition to hit 158 — no combination I tried that
     stayed principled and reproducible landed on that number, and AGENTS.md rule 9 says an
     unreproducible figure does not get to win over a reproducible one.
  2. **38,372 vs 38,391 units.** `docs/work-inventory.json` (`totals.units`), regenerated
     2026-08-21T23:23:45Z (one day before this cycle, by `cargo run --bin v06_work_inventory`), is
     **38,391**, not the 38,372 AT-32-G0-002 quotes. I did **not** regenerate
     `docs/work-inventory.json` in this cycle: the binary refuses to overwrite it without
     `CORPUS_LITERAL_SWEEP_REPORT` / `DERIVED_FIXTURE_CHECK_REPORT` set (it fail-closed-refused a
     bare `cargo run --bin v06_work_inventory` here — "this run would drop 8246 of the 8246
     verification stamp(s)") — those reports belong to a different pipeline (the literal-sweep +
     fixture-check cycle), not to a census card, and passing `--allow-stamp-loss` to force past a
     fail-closed guard protecting 8,246 verification stamps is exactly the kind of shortcut
     AGENTS.md rule 5/8 forbids taking on a prompt's authority. The existing file is one day old,
     already regenerated with `PCGEN_CORPUS_ROOT` pointed at a real oracle checkout, and its
     `total_units` field name from AT-32-G0-002's own verification command (`jq '.total_units'`)
     does not exist in the current schema (`schema_version: 1` has `.totals.units` instead) — a
     second stale-command finding, also logged.
  3. **AT-32-G0-002's ten-kind list omits `class_feature`**, the single largest kind in the live
     inventory (15,439 units). See `object-definition-rules.md` "DISCOVERY" section and the
     deferral logged in `docs/retro/events/gate-0-census.jsonl`. I did not force class-feature rows
     into one of the ten kinds; they are named (`class_feature`, 18,231 units in the census's own
     count) and left for an operator ruling or Gate 1's shape work to resolve.
  4. **AT-32-G0-003 (book onboarding) is out of this card's scope** — it is kanban card 4
     (`gate-0-book-onboarding-precondition`), sequenced behind this card per
     `workflow-instruction.md §2.4`/§3. Not attempted here.
- **Discovery forwards:**
  - `## DISCOVERED` — `class_feature` (18,231 units) and 26 other named kind-unenumerable buckets
    (skills files, deity, domain, kit, language, power, template_row, 25 `ability_category:*`
    values) exist outside AT-32-G0-002's ten-kind list. Needs an operator ruling: extend the
    vocabulary, or state explicitly that these are out of Gate 0's per-kind counting scope. Full
    breakdown: `object-definition-rules.md`.
  - `## DISCOVERED` — the "158-book" and "38,372-unit" figures cited across
    `acceptance-and-verification.md`, `technical-design.md`, `scope-draft.md` have no reproducible
    derivation command anywhere in the bundle and do not match this cycle's re-derivation (186 book
    dirs; 38,391 units). Both logged as `scripts/retro.py correction` events.
- **Next-cycle plan:** Card 4 (`gate-0-book-onboarding-precondition`) picks up AT-32-G0-003 —
  onboarding the 4 `future_state`-scoped books
  (`docs/work-inventory.json` books with `scope: "future_state"`) — sequenced behind this cycle.
  Gate 0 is not fully closed until card 4 also lands (AT-32-G0-003 is part of Gate 0's own
  definition of done, `decisions.md` Decision 2).
