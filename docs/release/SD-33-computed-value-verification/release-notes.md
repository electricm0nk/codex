---
canonical: true
owner: god-emporer
bundle_id: SD-33
status: generated
date: 2026-08-26
---

# SD-33 Release Notes

Generated at bundle closure (`AT-33-E6-003` part 2), figures re-derived directly from the repo at
closure rather than copied from a mid-bundle receipt.

**Version:** `0.13.0` (`apps/desktop/package.json` and `apps/desktop/src-tauri/tauri.conf.json`,
both confirmed unchanged at closure — stamped at the `tranche/13` cut, `f652db7ac7`; the tranche
digit moves only on a new `tranche/N` branch cut, never on a bundle's own closure).

## Generation rule

Every figure below carries the command that produces it and its denominator (`decisions.md §2`).
Every headline figure was re-run in this closure cycle, in a clean `git worktree add --detach`
off `origin/tranche/13`, not copied from `AT-33-E6-001-attempt10_cycle_receipt.md` — the exact
hazard SD-32's own release notes hit (its Gate-1 figures shipped stale and its closure cycle had
to catch and re-derive them).

**Re-derived a second time, 2026-08-26, after the operator's fold ruling.** This document was
first generated against `1bfb80d7b7`. The fold added real content after that (the "Recovered
work" section below) and, separately, briefly broke a live lib-test assertion that a later fold
commit left stale — closed before this re-derivation, not papered over. Every figure in this
version was re-run against the fold's final tree rather than edited from the pre-fold numbers; see
`docs/retro/sd33-computed-value-verification-retrospective.md`'s fold section for the full account
of what moved and why.

## What shipped

**Bundle:** SD-33 — Computed-value verification
**PR:** [#377](https://github.com/electricm0nk/codex/pull/377) — `tranche/13` → `develop`, open
**Retrospective:** `docs/retro/sd33-computed-value-verification-retrospective.md`

SD-33 built a live PCGen-oracle comparison harness and used it to examine every computed value the
program had already marked `done`. Twelve computed-value and instrument defects were caught this
way — values that compiled clean, passed every prior test, and were wrong.

## Defects found and fixed

Each defect below is verified against `git log` in this closure worktree, not asserted from the
retrospective's own prose.

1. **Spell-DC fixture defect — 103 units, correct by accident.**
   `fixture-generate-spell-batch.py`'s `.pcg` template pinned `STAT:WIS|SCORE:10` for every
   casting class — silently right for Intelligence/Charisma casters (their save DC never reads the
   WIS score) and silently wrong for every Wisdom caster. Fixed to `18` to match the probe's own
   pinned ability array. Commit `dded72f0b4`.
2. **Armor compute defect — 22 units.** `compute_arms_armor_effect`/`compute_var_effect` never
   resolved a base item's `EQMOD:`-referenced modifier record's own separate `BONUS:` chain. New
   `equipment_effects::eqmod_referenced_records` resolver plus a `TYPE=Circumstance` exclusion fix.
   Commit `abc72f75ec`.
3. **AC measurement method — 4 units, a wrong ruler, not a wrong engine.** The harness's
   whole-character `AC.TOTAL` diff conflated an item's own AC bonus with a second-order `MAXDEX`
   cap loss or a co-located Dex-enhancement chain. Replaced with an absolute per-type isolator
   (`BONUS.COMBAT.AC.TOTAL.!BASE.!Ability.!Size`). Commit `a68fbeea3d`.
4. **`compute_equipmods_effect` multi-chain summing — 1 unit (`heavy_hammer`).** A record with two
   separately-scoped `BONUS:` chains only had the first summed. `WeaponEnhancementBonus` split into
   independent `tohit_bonus`/`damage_bonus` fields; corpus-wide scan confirmed exactly one affected
   record. Commit `2f1d52f22d`.
5. **`equipment_id_resolve` OUTPUTNAME-divergent identity fix.** A templated-variant record's
   engine identity resolution matched the wrong axis when `OUTPUTNAME` diverged from the base key.
   Commit `9df1c0b514`.
6. **Campaign-KEY-vs-display-name harness defect.** The `ultimate_psionics` oracle-comparison
   harness loaded campaigns by display name where the pinned oracle indexes by KEY, silently
   comparing against the wrong campaign context. Commit `9df1c0b514`.
7. **Corpus-extraction gap dropping `.MOD`-attached EQMOD references — 139 records across 9
   books, plus `rending_claw_blades`'s own two-fold engine gap.** The extraction pipeline never
   folded a record's second `.MOD`-attached `EQMOD:` token into `raw_tokens` at all (commit
   `fbc945f198`). Regenerating those 139 records then exposed two further `src/rules_core/`
   defects on `rending_claw_blades` — `eqmod_referenced_records` reading only the first `EQMOD:`
   token via `.find()`, and the weapon compute path never folding `EQMOD:`-referenced
   weapon-enhancement chains — closed via a new
   `equipmods::apply_eqmod_weapon_enhancement_bonus`. Commit `7d439876b7`.
8. **Two previously-unwired resolvers landed RED→GREEN.** `EQMWEAPON|DAMAGESIZE` and
   `EQM|WEIGHTDIV` had never been handled anywhere in `src/rules_core/` — a genuinely new
   modifier-application mechanism (EQMOD baked into a homebrew LST item at load time). Commit
   `a488e0abaf`.
9. **Epic 4's reclassification turned the lib suite red on an unmapped doneness pair.**
   `AT-33-E4-002`'s widened doneness classification left 3 of 2,836 lib tests red on an unmapped
   `(ambiguous, unmeasurable)` verdict pair (11 of 49,438 units) — closed inside this bundle rather
   than handed to a successor. Commit `00ca087775`.
10. **A struct rename in Epic 5's own commit hid 543 of 543 integration targets behind a compile
    error.** Splitting `WeaponEnhancementBonus`'s `affects`/`bonus` fields into `tohit_bonus`/
    `damage_bonus` (commit `2f1d52f22d`) left two test files outside the commit's own diff on the
    old field names; `0 of 543` integration targets executed until fixed. Commit `a0e1c017dd`.
11. **`corpus_literal_sweep`'s own two `.COPY=`/`DESC` defects — 10 records, 105 stale
    findings.** The sweep's own `.MOD`-identity closure builder, not the extraction enricher, was
    wrong on two shapes: a whole-book unsorted `read_dir` walk resolved a `.COPY=` base to the
    wrong same-named record (`hellscourge` → an unrelated decoy record), and a PI-redaction
    re-screen exclusion suppressed a legitimate redaction the corpus record had correctly applied.
    Both confirmed by hand-derivation against the pinned oracle `.lst` bytes; `data/corpus/**`
    untouched (0 of 6 files in the fix commit under that path). Commit `1bfb80d7b7`.
12. **The gate's own scan scope was itself a defect, found by the scanner and closed by an
    instrument lane.** The final-acceptance scan's attempt-9 shortfall was `corpus_literal_sweep`
    the tool being wrong about 10 of its own 137 newly-regenerated records, not a corpus bug —
    verified against all four illegitimate routes to a false zero (population narrowed, `raw_tokens`
    reverted, an exclusion added, `data/corpus/**` hand-edited — none of the four happened). Same
    fix as item 11, commit `1bfb80d7b7`.

**One legacy disagreement was escalated, not fixed, in an earlier wave** (the corpus-extraction gap
in item 7, before it was root-caused) and is now closed by item 7's fix — 0 of 8,330 examined units
remain `disagree` (below).

## Recovered work — the operator's 2026-08-26 fold

This is **recovered work, not newly built work**, and that provenance is the interesting part: a
sweep of stale local branches after PR #377 opened found two lanes that had been generated during
SD-31 and never merged, holding content genuinely absent from HEAD. Both branches' own commit
messages say what happened to them — *"PRESERVE 48 generated Skinwalker race_trait records from the
same lost lane"* and *"PRESERVE uncommitted race-chassis work from the wave-11 lane lost to an API
error"*. The operator ruled: fold the genuinely-unique content into SD-33 before the PR merges,
rather than let a real gap ride into SD-34 or force a from-scratch re-derivation of work that
already existed. Neither branch was merged on trust — each was regenerated through its guarded
generator path and independently hand-traced against the pinned oracle before landing. Three other
stale branches were swept at the same time and ruled **out** as superseded, not folded — recorded in
`forward-scope-register.md`.

1. **65 recovered Skinwalker `race_trait` corpus records — bestiary_5, 10 → 75.** The rescue
   branch (`sd31/racetrait4-SD31-E6-F4-005`) held 45 hand-copied Skinwalker `*_kin` records; folding
   them through the guarded generator path (`cargo run --locked --bin ingest_race_traits
   bestiary_5`) — rather than `git show`-ing the branch's own copies — produced **65** real records,
   20 more than the branch itself carried. **75 of 75** on-disk records hand-traced token-for-token
   against the pinned oracle `.lst`: 67 byte-identical, the other 8 differing only in the declared
   `DESC` PI redaction (matching `grep -c 'DESCISPI:YES'` on the oracle → 8 exactly). License/PI
   intact on all 65 new files (58 `OGL` / 8 `PI-REDACTED`); `data/corpus/bestiary_5/LICENSE.json`
   moves `records_processed` 279 → 344, `records_redacted` 9 → 17. Nine real wiring/reachability
   defects were fixed alongside the fold (two new `pilot_compute::ALTERNATE_TRAIT_SELECTED_SKILL_BONUSES`
   rows a player's skill total actually reads, a missing per-kin exclusion guard that let two
   incompatible replacement rows be selected together, and others) — full list in
   `artifacts/epic-6-closure/fold-skinwalker_cycle_receipt.md`. Commits `6e2f2f076b`/`56bbebe3d4`.
   Re-derive: `ls data/corpus/bestiary_5/race_trait/skinwalker/*.json | wc -l` → `75`.
2. **3 recovered Undine `race_trait_formula` fixture entries, plus the compute seam that makes
   them real.** The dispatch's own "103" framing was corrected in the same cycle it landed — 103 is
   raw string occurrences of the word "Undine" across the fixture file (~34 mentions × 3 records),
   not an entries count. The real population is **3** entries (`undine_acid_breath`,
   `undine_nereid_fascination`, `undine_ooze_breath`), each with 3 formula fields and 10
   `expected_at_sample_points` — **30 sample points, 90 scalar assertions total**, all 3 clearing in
   `run_race_trait_formula_bar_check`. Folded from `worktree-wf_be4660f2-72a-3`:
   `scripts/derive_race_trait_formula_fixtures.py` (reads only the pinned oracle `.lst` bytes via
   `pcgen_data_root()`, no engine module, no `data/corpus/` read) plus a new
   `UNDINE_RACE_TRAIT_FORMULAS`/`explain_undine_formula_race_trait` compute path wired unconditionally
   into `compute_pilot_base_chassis`. **Deliberately did not** add `"undine"` to
   `race_ids_with_a_magnitude_consumer()` — that would have silently credited 20 Undine `race_trait`
   records as `grounded` when only 3 have any real consumer, the exact gaming vector
   `OPEN-ISSUES.md` row 365 named; the union stays at 18 races, 0 board-credit units banked by this
   fold. Commit `948976aacb`.
   Re-derive: `python3 -c "import json; d=json.load(open('tests/fixtures/rules_core/derived-evaluator-fixtures.json')); e=d['race_trait_formula_entries']; print(len(e), sum(len(x['expected_at_sample_points']) for x in e))"` → `3 30`.
3. **The fold fixed two previously-failing suites outright.** `src/bin/ingest_races.rs` (43 of 44
   tests failing → green) and `tests/sd27_alternate_racial_trait_reachability.rs` (13 of 15 failing
   → green) both carried a pre-existing gap that the Skinwalker fold's own new records made the
   ingest walk reach deeply enough to trip and then close in the same cycle. The bundle's registered
   inherited-debt figure moves accordingly, a **shrink, not a reclassification**: **31 of 599 suites
   / 49 of 8,026 executed tests → 29 of 599 / 46 of 8,034 executed tests** (the executed denominator
   also grew, by the fold's own 8 new test cases). Re-derive:
   `cargo test --locked --no-fail-fast`, attributed back to each `Running` line
   (`artifacts/epic-6-closure/AT-33-E6-001-attempt12_cycle_receipt.md`).

**A regression the fold itself introduced was caught and closed inside this same reopen, not carried
forward.** `docs/work-inventory.json`'s regeneration (folding the Skinwalker records' status)
correctly moved F1's formula-bearing population 6,278 → 6,260, but a *later* commit
(`cef0ca1b39`, regenerating the same inventory file for an unrelated reason) moved it again to
6,257 without the lib suite being re-run afterward — leaving a stale `assert_eq!` red at HEAD.
Attempt 11 of the final-acceptance scan caught it; `fold-fix-repin` (`c0f5e9091e`) re-pinned the
assertion to the live value with its mechanism, and attempt 12 re-derived the whole closure clean.
Full account: `artifacts/epic-6-closure/AT-33-E6-001-attempt11_cycle_receipt.md`.

## Figures, re-derived at closure

**Oracle examination — 8,330 of 8,330 blessed units examined, 0 disagree:**
- Agree: 811 of 8,330 examined units
- Unverifiable (no comparable oracle export token, each with a stated structural reason): 7,519 of
  8,330 examined units, 0 of 7,519 without a reason
- Disagree: 0 of 8,330 examined units

Re-derive: `python3 scripts/box_ledger.py --check --oracle-results
docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json`
→ `uncovered=0 overlap=0 population=49438 oracle_disagreement=0`

**Work-inventory classification — 0 of 49,438 units at `unknown`.**
Re-derive: `jq '[.units[]|select(.status=="unknown")]|length' docs/work-inventory.json` → `0`
(inventory total: `jq '.units|length' docs/work-inventory.json` → `49438`)

**Formula interpreter coverage — 11,652 of 11,652, corpus-wide.**
Re-derive: `jq -r '[.families[].population]|add'
docs/release/SD-33-computed-value-verification/artifacts/epic-3-engine-coverage/formula_interpreter.corpus-wide.json`
→ `11652` (F1 6,308 / F2 2,337 / F3 671 / F4 1,086 / F5 589 / F6 391 / F7 12 / F8 196 / F9 62)

**Test suites, re-derived after the fold:**
- Rust lib: 2,845 of 2,845 passed (`cargo test --locked --lib`) — up from 2,837 at first closure;
  the fold's own new tests, plus the F1 re-pin (`fold-fix-repin`, `c0f5e9091e`) closing the one
  assertion the fold's inventory regen left stale
- Desktop Tauri app: 548 of 548 passed (`cargo test --locked` in `apps/desktop/src-tauri`),
  unchanged
- Integration targets: 543 of 543 build (`cargo test --locked --no-run`, `grep -c 'Executable
  tests/'` on the output)

**Corpus sweep — 0 findings of 48,699 records examined.** Grown from 48,634 by exactly **+65**,
the fold's own new Skinwalker records — not a vacuous pass; the population that grew is the one
the new records actually landed in.
Re-derive (run live for this doc, 2026-08-26): `cargo run --locked --bin corpus_literal_sweep` →
`corpus-literal-sweep: 48699 records examined of 51473 read, 413288 tokens compared (9
synthesized), 51460 digests checked, 0 findings`

**Denominator gate — 0 violations, run against this closure diff.**
Re-derive (run live for this doc): `python3 scripts/denominator_gate.py --check` →
`files_checked=69 violations=0`

## Inherited debt — registered, not vanished

**29 of 599 workspace test suites carry 46 of 8,034 executed tests that fail.** All 29 are
pre-existing at the `tranche/13` cut (`f652db7ac7`): 0 of 29 carry a single commit since the cut
(`for f in <each failing target>; do git log --oneline f652db7ac7..HEAD -- "$f" | wc -l; done |
awk '{s+=$1} END {print s}'` → `0`), and the failing set and its per-target pass/fail counts are
byte-identical between a clean run at the cut and this closure's HEAD. Registered at
`forward-scope-register.md §D1.1`, independently re-derived across five separate final-acceptance
scans (attempts 8, 9, 10, 11 and 12). A reviewer running the workspace suite should not be
surprised by this — it was true before SD-33 opened and SD-33 added 0 of these 29 failures.

**31/49/8,026 → 29/46/8,034 by the operator's 2026-08-26 fold ruling**, re-derived by
`AT-33-E6-001` attempt 12 (`cargo test --locked --no-fail-fast`, attributed back to each
`Running` line): the Skinwalker fold **fixed** two of the inherited 31 outright
(`src/bin/ingest_races.rs`, `tests/sd27_alternate_racial_trait_reachability.rs`), and the executed
denominator grew 8,026 → 8,034 with the fold's own new cases. This is a shrink of inherited debt,
not a reclassification of it.

`scripts/verify.sh`'s `site-dashboard-check` stage also carries a registered, reproduced-thrice
defect (no timeout wrapper around its own `v06_work_inventory --summary` call) —
`forward-scope-register.md §D1.2`. Neither item is this bundle's to fix; both are named rather than
silently carried.

## Architecture documentation updated

Five docs re-verified for current state and edited with real content this closure epilogue:
`docs/architecture/homebrew-and-oracle.md`, `docs/architecture/rules-engine.md`,
`docs/architecture/corpus-ingest.md`, `docs/architecture/testing.md`,
`docs/architecture/status.md`. Graphify was run against the updated tree (`AT-33-E6-003` part 1
receipt).
