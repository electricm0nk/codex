# SD-32 closure-readiness audit — cycle receipt

**Cycle:** `closure-readiness-audit` (read-only audit; no ingest/compute code written).
**Scope:** kanban rows 11 (`epic-2-cause-closure`) and 15 (`census-scope-closure`), plus the four
gates' live status and the branch's real test state. No writes to any sibling-lane territory
(`data/corpus/**/monster_ability/**`, `scripts/transcribe_monster_tables.py`,
`monster_chassis.rs`, `scripts/pi_scrub.py`, `declared_pi_shipping_audit`,
`cache_gen::{ultimate_equipment,acg,apg,beastiary1}.rs`, `gen_core_rulebook_cache.rs`,
`data/corpus/inner_sea_gods/equipment/**`, `scripts/census_untabled_base_class_feature_roster.py`,
`pilot_compute`, `rules_tables/**/*_features.rs`).
**Base/HEAD:** pinned base `1b2dbfcdbc18ed272536b242d9113630da1e7c1d`; two sibling commits fetched
and rebased in mid-cycle (`f0b6a8b96f` T9 PI last-leak, `cd60d08042` T12 census-widening
follow-up) — re-derivation below is against the rebased HEAD `cd60d08042`, corpus SHA
`7f818006e371188e5717fd18d74d18a420747fc6`.
**Worktree note:** this worktree's branch was NOT a descendant of the pinned SHA at cycle start
(stale `tranche/11` merge lineage, `1bb523773d`, `git status --porcelain` clean) — resolved per
`workflow-instruction.md` footgun-1 remediation: `git reset --hard "$PIN"` then
`git rebase origin/tranche/12` (no-op, PIN == origin/tranche/12 at that point).

## 1. Gate-by-gate live re-derivation (§17a — never trusted the brief's own figures without rerunning)

| Gate | Command | Live result |
|---|---|---|
| Gate 0 (census) | `python3 scripts/census_independent.py --pcgen-root "$PCGEN_CORPUS_ROOT" --inventory docs/work-inventory.json --output <path>` | `discovered=186 in_scope=38 excluded=148 unexplained=0` — PASS |
| Gate 0 (denominator) | `jq '.totals.units' docs/work-inventory.json` | `49438` (live; frozen launch figures 38,372/158-book/49,490 are all stale by construction, not requoted here) |
| Gate 1 (shape ledger) | `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output <path>` | `unclassified_count=0`; `join_status_counts` = `{matched: 11422, no_formula_tokens: 22919, no_record: 56}` over population 49438 — PASS, split reported per AT-32-G1-004 |
| Gate 2 (engines) | `cargo test --locked --lib pilot_compute::` | 898/898 pass, including `formula_interpreter_corpus_wide::tests::corpus_wide_scan_population_matches_the_closed_gate1_census` and `formula_interpreter::tests::corpus_shape_coverage` — PASS |
| Gate 3 (standing gate) | `scripts/verify.sh --only shape-coverage-standing-gate` | `PASS (population=34397 unclassified=0 no_record=56 corpus_sha=7f818006e371188e5717fd18d74d18a420747fc6)` |
| Gate 3 selftest | `scripts/verify.sh --only shape-coverage-standing-gate-selftest` | `PASS (20 cases passed)` |

No gate that "cannot fail" or an assertion that "had never run green" was found in this pass — all
four gates re-ran live and produced non-trivial, class-checked output (Gate 3's own red-proof test
suite exercises the real classification path per AT-32-G3-001, confirmed present in the 20-case
selftest).

**`no_record` breakdown by kind** (`python3` over `shape_ledger.py`'s own `rows` — never trusted
the "56, all monster_ability" claim without re-deriving):

```
Counter({'monster_ability': 56})
Counter({'bestiary': 21, 'bestiary_3': 10, 'horror_adventures': 9, 'bestiary_4': 7,
          'inner_sea_bestiary': 3, 'pathfinder_unchained': 3, 'bestiary_2': 2, 'bestiary_5': 1})
```

CONFIRMED: the dispatch brief's "56, all monster_ability" is correct at rebased HEAD.

**Row 17 census** (`python3 scripts/row17_census.py --check`):

```
row 17's actual population (placeholder / not genuinely derived):
  F0 by fallthrough                   0
  §27 provisional default            23   (corpus-wide total incl. done units: 24)
  ROW 17 HONEST SIZE                 23
excluded from row 17 (sequencing):
  not_ingested (no_record)           56
```

CONFIRMED: the brief's "23 provisional defaults, all stamped" is correct at rebased HEAD (re-run
twice, before and after the mid-cycle rebase — identical both times).

## 2. Row 11 (`epic-2-cause-closure`) — what is genuinely left

AT-32-E2-001's eight measured shapes: T2a, T2b, T9, T4, T12, T5, T1, T3 (T7/T8 close
opportunistically; T5 credited via Epic 4 card 4, T3 via Epic 5 card 1).

| Shape | Disposition | CONFIRMED-OPEN / ALREADY-CLOSED | Evidence |
|---|---|---|---|
| T1, T2a, T2b, T4, T7, T8 | Closed via the `epic-2-cause-closure/4` consolidation cycle (`bdb27d63f`) plus the `decisions.md §20` generic-ingest campaign. `## Open blockers` entries that once named these (filed 2026-08-22) are marked **RESOLVED, removed 2026-08-23** in `progress.md`. | **ALREADY-CLOSED** | `no_record` breakdown above shows zero units of any kind except `monster_ability` — none of T1/T2a/T2b/T4/T7/T8's populations (race_trait, class, equipment/equipment_modifier/ability, dashboard classifier) remain `no_record`. |
| T9 | `monster_ability` residual, 56 units, the multi-`DESC:` parse-refusal group (round 6/7/8's own named shape). Actively being closed by the sibling T9 lane per this dispatch's own brief. | **CONFIRMED-OPEN, sibling territory** | `no_record` Counter above; not my territory (`data/corpus/**/monster_ability/**`, `scripts/transcribe_monster_tables.py`). |
| T12 (the 108-magnitude-bearing `untabled_base_class_feature_roster` closure) | The "108/108 closed, 19 classes" claim (cycle4, `progress.md`) was **stale the moment it was made** — the very next cycle (`epic-2-t12-psion-shape3-closure`, prepended newest in `kanban.md` row 11) widened the census's Shape-3 rule and found **10 more magnitude-bearing records** (7 sibling classes' `"<ClassName> Manifesting"` + 3 new records on the already-"complete" antipaladin) plus psion's own 32-record discipline-choice pool and 2 escalated `BONUS:VAR` terms. **As of the sibling commit fetched and rebased in during this audit (`cd60d08042`, landed after this cycle's PIN):** the 10 new records are closed (212/212 targeted tests green per its own message) and the `BONUS:VAR` sum-vs-replace ambiguity is **resolved**, cited against real PCGen source (`pcgen/core/PlayerCharacter.java:2136`, `BonusManager.sumActiveBonusMap` — multiple `BONUS:VAR` entries on one target SUM). | **RESOLVED as of `cd60d08042`** (was the brief's named candidate operator-ruling item; no longer needs one) | `git log --oneline -3 origin/tranche/12` shows `cd60d08042` as HEAD at time of this audit; commit message re-derives row 17 (23) and `no_record` (56) matching this receipt's own independent re-derivation above. |
| T12's newly-sized pool population | `cd60d08042`'s own commit message sizes (does **not** close) `class_feature_pool_catalog.rs`'s whole pool-shaped exclusion class: **~1,913 group-qualified names, ~16,350 records, ~6,131 magnitude-bearing corpus-wide, of which only ~71 records (2 of 27 registered pools: Rogue Talent, Rage Power) are currently modeled.** This is a pre-existing SD-31 exclusion class (`src/rules_core/class_feature_pool_catalog.rs`'s own doc comment, "left named, not built, per the dispatch's own 'report what it would cost to extend' ask") now squarely in scope under `decisions.md §27b` ("EVERYTHING... no carve-outs survive"; cost/novelty/"needs a new mechanism" are explicitly NOT grounds for exclusion). | **CONFIRMED-OPEN, large, sibling territory (`pilot_compute`, `rules_tables/**/*_features.rs`)** | `src/rules_core/class_feature_pool_catalog.rs` header comment (read live); `cd60d08042`'s own commit message. No hard-impossibility claim made anywhere for this population (no absent source data, no licensing bar) — it fails `§27b`'s test for a valid exclusion. |
| T5, T3 | Credited via Epic 4 card 4 / Epic 5 card 1 respectively per AT-32-E2-001's own text; both epics show `complete` in kanban. | **ALREADY-CLOSED** (by cross-reference, not re-verified independently this cycle — out of my scope to re-audit Epic 4/5's own closure). | `kanban.md` cards 12/16 `complete`; not independently re-derived here. |

**Closure path for row 11:** the eight measured shapes are all closed or (T9) actively closing in a
named sibling lane. The one item that is **not** closed and is **not** anyone's stated territory
right now is the `class_feature_pool_catalog.rs` pool population (~6,060 magnitude-bearing records
outside the 2 already-modeled pools). Under `§27b` this is in scope and has no valid exclusion —
it needs a kanban line item of its own (it currently exists only inside a commit message, which is
exactly the "named but unowned" shape this audit was dispatched to catch) and a dedicated cycle,
sized larger than any single closure cycle this bundle has run so far. **Row 11 is not closeable
today**; per this dispatch's own instruction it is left `in-progress` (not reclosed).

## 3. Row 15 (`census-scope-closure`) — what is genuinely left

Scope: "Close the 27,847 kind-unenumerable objects: enumerate + classify, or prove not-an-object by
class" (Gate 0 + Gate 1). Row's own status note: "3 of 3 lanes landed — integration cycle next."

Live re-derivation (§17a, not trusted from the note):

- Gate 0 `census_independent.py`: `unexplained=0` (table above).
- Gate 1 `shape_ledger.py`: `unclassified_count=0`, and the only `no_record` units left in the
  whole corpus are `monster_ability`'s 56 (T9's territory, not a card-15-shaped kind-unenumerable
  gap) — every other kind that card 15's own note history (source.path repair, template/language
  closure, `Kind::Trait`, `Kind::Ability`) reports as closed to zero is independently confirmed
  zero in this cycle's own `no_record` breakdown.

**Finding:** the underlying numeric claim — every kind-unenumerable object is now enumerated and
classified, or accounted for as `no_record` in T9's own named residual — holds up under
re-derivation. What has **not** happened is the row's own stated "integration cycle next": a
consolidation pass across its three landed lanes analogous to what row 11's `epic-2-cause-closure/4`
cycle did (re-deriving all sub-shapes' disposition directly from `git log`/receipts, not from the
lanes' own prose, and setting the row's status). No such consolidation receipt exists yet for row
15. **This audit does not manufacture that consolidation** (out of scope — no ingest/compute
territory was touched, and the dispatch's own hard rule says leave row 15 `in-progress`), but
reports: the content-closure numbers are live-confirmed at zero-gap; the row's own required
bookkeeping step (the integration cycle) is the only thing standing between it and `complete`.

## 4. Named-but-unowned sweep (§2 of the dispatch)

Grepped `kanban.md`/`progress.md`/`decisions.md` for "named not attempted", "next-cycle plan",
"discovery forward", "deferred", "flagged", "out of scope", "logged not fixed", "escalated" and
read every match in context (not pattern-matched):

- **`## Open blockers` in `progress.md`:** five entries present, **all five marked `RESOLVED,
  removed 2026-08-23`** with their own resolution evidence and re-derivation commands. No live
  (unresolved) blocker entry exists in this file.
- **`forward-scope-register.md` C2.5** (T2a/T2b/T9/T4/T12/T7/T8 "filed as `returned-to-backlog`"):
  **STALE.** This register entry describes the *first* dispatch run's (2026-08-22) disposition,
  before `decisions.md §10`/`§13` reopened all five named sub-populations for real closure and
  before the consolidation cycle closed them. It was not updated when the Open Blockers entries it
  cites were resolved. Not a functional blocker — the underlying work is closed (§2 above) — but a
  documentation-drift finding: a future reader of `forward-scope-register.md` alone would believe
  work is deferred that has since landed. Flagged here, not corrected (outside this audit's granted
  write scope — `forward-scope-register.md` is not in `kanban.md`/`progress.md`/receipt).
  - **Note:** the SAME register entry is where `T12` was informally named. The pool population
    finding in §2 above should get its own explicit row here or in `kanban.md` once a cycle is
    dispatched to own it — right now it exists only in one commit message.
- **`BONUS:VAR` combination-semantics escalation** (the brief's named candidate for an operator
  ruling): **RESOLVED**, not merely re-escalated — `cd60d08042` cites real PCGen source code
  confirming the "sum" reading, landed after this cycle's PIN and picked up by the mid-cycle
  rebase. No live escalation remains on this item.
- Several older "escalated, not fixed" / "named not attempted" matches (e.g. `spell_mod_access`
  generator conflict, `deity` 459-unit PI exclusion, an earlier `T2a` next-cycle-plan note) were
  read in full and found to describe states from earlier in the bundle's history, chronologically
  superseded by later prepended kanban entries and/or the `no_record`-by-kind re-derivation above
  (none of those kinds carry any live `no_record` population today). No corroborating live evidence
  of an unresolved gap was found for any of them.

**No genuinely-open, unowned item was found other than the `class_feature_pool_catalog.rs` pool
population named in §2.**

## 5. Test suite state

Scoped runs (full unscoped `cargo test --locked --no-fail-fast` not run, per dispatch instruction —
this codebase's targeted suites below cover the areas this audit's rows and gates touch):

| Suite | Command | Result |
|---|---|---|
| `pilot_compute::` | `cargo test --locked --lib pilot_compute::` | 898/898 pass |
| `rules_tables::` | `cargo test --locked --lib rules_tables::` | 623/623 pass, 3 ignored |
| `cache_gen::` | `cargo test --locked --lib cache_gen::` | 186/186 pass |
| Generator static audit | `cargo test --locked --test generator_name_key_screening_static_audit` | 4/4 pass |
| Gate 3 standing gate + selftest | `scripts/verify.sh --only shape-coverage-standing-gate[-selftest]` | both PASS |
| `apps/desktop/src-tauri` `equipment_catalog::` (separate cargo workspace) | `cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml equipment_catalog::` | **14/17 pass, 3 FAILED**: `catalog_spans_every_ingested_book_with_their_real_counts` (72 vs 71), `description_coverage_is_pinned_per_book` (97 vs 72), `filter_equipment_catalog_matches_category_exactly_across_every_book` (1095 vs 1077) |
| `declared_pi_shipping_audit` (Rust bin, corpus-wide PI scan) | `cargo run --locked --bin declared_pi_shipping_audit` | Started; did not complete within this cycle's turn (long-running full-corpus scan, still executing at report time — output at `artifacts/pi_audit_run.out`, not committed, scratch only). Corroborating evidence from the sibling T9 lane's own most-recent cycle: `sd32_t9_corpus_wide_pi_rescan.py` reports 0 field-level hits over 51,360 records (`progress.md`, `t9-onboarding-pi-last-leak-and-generators` entry), and this audit's own live run of `generator_name_key_screening_static_audit` (4/4 pass) confirms the structural "every identity-bearing generator screens" invariant independently. |

**`apps/desktop` equipment-catalog pin drift: CONFIRMED STILL RED.** This matches the pre-existing,
named `scripts/retro.py deferral` (`progress.md`: "One larger pre-existing drift deferred, not
fixed": `apps/desktop/src-tauri/src/equipment_catalog.rs`'s test module carries several stale
pinned counts — per-book description coverage, category-filter total, overall catalog length —
"unrelated to anything this cycle touched", logged as a deferral, out of scope for that cycle and
this one). No other new red suite was found in the areas this audit ran; nothing was silently fixed
or silently inherited beyond reporting it here.

## 6. Disk

`df -h /`: reported at end of cycle in the dispatch response.

## Verdict

Gates 0-3 all re-derive live and PASS against their own acceptance criteria; no gate-that-cannot-
fail or never-green assertion was found in this pass. Row 11 and row 15 both stay `in-progress`
per dispatch instruction. Row 11's real remaining content is the `class_feature_pool_catalog.rs`
pool population (~6,060 unmodeled magnitude-bearing records) — large, genuinely open, currently
unowned by any named lane or kanban line, and in scope under `§27b`. Row 15's real remaining work
is its own stated integration/consolidation cycle, not further content closure. The
`apps/desktop` equipment-catalog pin drift is confirmed still red and still correctly out of scope
as a pre-existing, named deferral.
