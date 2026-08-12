# SD-27 — Progress

> **Live cycle log.** Per `spec-domain-bundle-authoring` — the canonical 10-file chassis includes `progress.md` for tracking cycle state. The orchestrator's `Workflow` writes `## TODO`, `## DONE`, `## DISCOVERED`, `## Status matrix`, `## Open blockers` sections as the bundle runs.

## Status matrix

| Criterion | Cycle | Epic | Status | Started | Completed | Notes |
|-----------|-------|------|--------|---------|-----------|-------|
| 1.1 | `identifier-audit` | E1 | complete | 2026-07-27T19:59Z | 2026-07-27T20:04Z | Full-tree scan (not diff-scoped — see receipt); 0 genuine bundle-tagged code identifiers. Matches SD-26's own E1.1 precedent |
| 2.0 | `label-resolution` | E2 | complete | 2026-07-27T20:10Z | 2026-07-27T20:16Z | Operator chose `"SD-27"`. All 21 stubs + registry + v0.6 risks doc resolved |
| 2.0.5 | `shape-b-license-stripping-preflight` | E2 | complete | 2026-07-27T20:20Z | 2026-07-27T20:28Z | src/rules_core/shape_b_v1.rs + docs/governance/ogl-pi-blacklist.md (draft) + dual-audit test, all independently re-verified. Gates 2.0.6+ |
| 2.0.6 | `crb-license-retrofit` | E2 | complete | 2026-07-27T20:36Z | 2026-07-27T20:44Z | 3,326 records classified (3,326 OGL, 0 redacted). Registry update N/A (no CRB entry exists) |
| 2.0.7 | `apg-license-retrofit` | E2 | complete | 2026-07-27T20:36Z | 2026-07-27T20:44Z | 641 records classified (641 OGL, 0 redacted) |
| 2.0.8 | `acg-license-retrofit` | E2 | complete | 2026-07-27T20:36Z | 2026-07-27T20:44Z | 423 records classified (422 OGL, 1 PI-REDACTED — flagged for operator review) |
| 2.0.9 | `beastiary-license-retrofit` | E2 | complete | 2026-07-27T20:36Z | 2026-07-27T20:44Z | 45 records classified (45 OGL, 0 redacted) |
| 2.0.10 | `all-23-books-license-conformance-verify` | E2 | complete | 2026-07-27T21:00Z | 2026-07-27T21:05Z | 23/23 books accounted for, 0 defects (4 corpus-conformant + 19 honest stub gaps). Gates 2.1+ cleared |
| 2.1 | `advanced_race_guide_pre_build` | E2 | complete | 2026-07-27T21:03Z | 2026-07-27T21:31Z | 479 records (92 spell + 200 equipment + 187 feat), all OGL, 0 redacted. Racial/ability-formula content out of scope (no precedent for any book) |
| 2.1' | `advanced_race_guide_verify` | E2 | complete | 2026-07-27T21:31Z | 2026-07-27T21:38Z | Independently re-verified 3x (subagent + orchestrator); sha256/line citations confirmed real |
| 3.1 | `advanced_race_guide_parity` | E3 | complete | 2026-07-28T11:30Z | 2026-07-28T11:47Z | Real PCGen Gradle pipeline, BUILD SUCCESSFUL. 13/15 dimensions match. 1 inherited CG-03 mismatch + 1 NEW genuine finding (encumbrance.rs CRB-only weight lookup) |
| 2.2 | `pathfinder_unchained_pre_build` | E2 | complete | 2026-07-27T21:03Z | 2026-07-27T21:26Z | 59 records (17 feat + 42 equipment), all OGL, 0 redacted. Book adds no new spells (honest absence) |
| 2.2' | `pathfinder_unchained_verify` | E2 | complete | 2026-07-27T21:26Z | 2026-07-27T21:32Z | Independently re-verified 3x; sha256/line citations confirmed real |
| 3.2 | `pathfinder_unchained_parity` | E3 | complete | 2026-07-28T11:30Z | 2026-07-28T11:47Z | Real PCGen Gradle pipeline, BUILD SUCCESSFUL. 14/15 dimensions match — only the inherited CG-03 mismatch (no ARG-style equipment-weight gap; PU's pilot exercised a feat, not a book-specific weighted item) |
| (deferred) | — | E3 | — | — | — | **17 deferred future-state books** (Adventurer's Guide, B2-B6, Bonus Bestiary, Horror Adventures, Monster Codex, Mythic Adventures, Occult Adventures, the 6 Tier-2 Ultimate books) are operator-gated on SD-27 closing cleanly. Beginner Box and Core Essentials removed from scope per operator directive 2026-07-27 (redundant to other tomes; will not be brought in). Deferred to SD-28+. The pre-build cycle pattern from E2.1-2.2 is templated and reusable. |
| 4.1 | `final-criterion-scan` | E4 | complete | 2026-07-28T11:58Z | 2026-07-28T12:05Z | 14/14 pre-closure criteria complete across all 3 independent sources, 0 discrepancies |
| 4.2 | `architecture-closure` | E4 | complete | 2026-07-28T12:00Z | 2026-07-28T12:03Z | Truth-up: clean, no architecture impact. Graphify: genuine environment gap (no CLI installed), honestly reported not fabricated |
| 4.3 | `release-notes` | E4 | complete | 2026-07-28T12:05Z | 2026-07-28T12:12Z | All 7 sections populated from receipt-backed facts, 0 placeholders |
| 4.4 | `version-bump` | E4 | complete | 2026-07-28T12:15Z | 2026-07-28T12:22Z | 0.6.0 → 0.6.1 across all 5 anchors (4 primary + `src-tauri/Cargo.lock`'s own-package field, a newly-discovered 5th anchor). `cargo check --offline` clean |
| 4.5 | `pr-merge` | E4 | PR opened | 2026-07-28T12:30Z | 2026-07-28T12:35Z | [PR #342](https://github.com/electricm0nk/codex/pull/342), `tranche/7` → `develop`. Operator go-ahead given; merge itself is the operator's own action |

## TODO

- (none yet)

## DONE

- **1.1 identifier-audit** (2026-07-27T20:04Z) — full-tree code-identifier scan clean, 0 hits. Receipt:
  `artifacts/epic_1/identifier-audit-cycle_receipt.md`.
- **2.0 label-resolution** (2026-07-27T20:16Z) — operator chose `"SD-27"`; all 21 stubs + registry + the
  v0.6 risks doc reconciled. Receipt: `artifacts/epic_2/label-resolution-cycle_receipt.md`.
- **2.0.5 shape-b-license-stripping-preflight** (2026-07-27T20:28Z) — Shape B v1 schema (additive over
  v0, proven against the real on-disk corpus) + PI-blacklist draft + dual-audit test, 10/10 new tests
  passing (independently re-verified by the orchestrator). Receipt:
  `artifacts/epic_2/2.0.5-shape-b-license-stripping-preflight-cycle_receipt.md`.

- **2.0.6-2.0.9 in-scope book license retrofit** (2026-07-27T20:44Z) — all 4 in-scope books
  retro-fitted in parallel (4,435 total records: 3,326 CRB + 641 APG + 423 ACG + 45 Bestiary; 1 real
  redaction, in ACG). Fixed a shared-test-file staleness (Audit 1's pre-retrofit `license==None`
  assumption) and a real regression in SD-26's own `tests/sd26_cache_core_rulebook.rs` (broke on the
  new `LICENSE.json`), both applied once by the orchestrator after collecting all 4 parallel agents'
  results. `cargo test --workspace --locked`: 4,802 passed / 3 pre-existing environment-dependent
  failures, zero regressions. Receipts: `artifacts/epic_2/2.0.{6,7,8,9}-*-license-retrofit-cycle_receipt.md`.
- **2.0.10 all-23-books-license-conformance-verify** (2026-07-27T21:05Z) — 23/23 books accounted
  for, 0 defects. Gates E2.1+ cleared. Receipt:
  `artifacts/epic_2/2.0.10-all-23-books-license-conformance-verify-cycle_receipt.md`.
- **2.1/2.1' + 2.2/2.2' per-book pre-build + verify** (2026-07-27T21:38Z) — real Rust codegen from raw
  LST source for both in-scope future-state books: ARG (479 records: 92 spell + 200 equipment + 187
  feat) + PU (59 records: 17 feat + 42 equipment, no new spells). Both books' agents ran concurrently
  in the same shared working directory (no worktree isolation) and hit a real, self-corrected
  file-touch-partition collision on `src/rules_core/rules_tables/mod.rs` — recorded as a real
  coordination risk for future concurrent per-book batches, not swept under the rug. Independently
  re-verified by the orchestrator a third time beyond each book's own subagent pre-build+verify pair:
  direct sha256/line-citation spot-checks against the real LST source, full `git status` scope audit,
  full workspace test suite (4,817 passed / 3 pre-existing failures, zero regressions). Both books'
  registry entries (#0003, #0017) updated to `"Resolved"`. Live reporting dashboard updated for real
  (`sd27_book_pre_build`: 4/6 complete at that point). Receipts:
  `artifacts/epic_2/{advanced_race_guide,pathfinder_unchained}_{pre_build,verify}-cycle_receipt.md`.
- **3.1/3.2 per-book PCGen parity baseline** (2026-07-28T11:47Z) — real PCGen Gradle pipeline run for
  both books (BUILD SUCCESSFUL both times), real `.pcg` fixtures hand-authored against the live PCGen
  data schema, real `comparator::compare` output. ARG: 13/15 dimensions match (2 mismatches: the
  inherited CG-03 baseline, plus a **new, genuine finding** — `src/rules_core/encumbrance.rs` resolves
  equipment weight via the CRB-only compiled `equipment_tables()` static table, so real equipment from
  other books resolves against the corpus correctly but its weight is silently dropped; root-caused,
  documented, not fixed — outside this cycle's file partition). PU: 14/15 dimensions match (only the
  inherited CG-03 mismatch; PU's pilot didn't trigger the encumbrance gap since it exercised a feat, not
  a book-specific weighted item — cross-validates the ARG finding as real and book-content-dependent,
  not a fluke). Independently re-verified by the orchestrator: re-ran both parity tests directly (exact
  match to reported tables), confirmed the `encumbrance.rs` root cause by reading the actual import
  statement, full workspace suite with `PCGEN_REPO_DIR` set (4,820 passed / 2 pre-existing failures —
  `sd26_pcgen_runner` now genuinely passes with the real PCGen checkout wired in). Live dashboard: 6/6
  items complete. Receipts: `artifacts/epic_3/{advanced_race_guide,pathfinder_unchained}_parity-cycle_receipt.md`.
- **4.1-4.3 closure scan, architecture truth-up, release notes** (2026-07-28T12:12Z) — 14/14
  pre-closure criteria confirmed across 3 independent sources; architecture truth-up clean (no
  architecture-doc impact), graphify honestly reported as a real environment gap (CLI not
  installed) rather than faked; all 7 release-notes sections populated from receipt-backed facts.
  Receipts: `artifacts/epic_4/{final-criterion-scan,architecture-closure,release-notes}-cycle_receipt.md`.
- **4.4 version-bump** (2026-07-28T12:22Z) — `0.6.0 → 0.6.1` across `apps/desktop/package.json`,
  `package-lock.json` (both embedded version fields), `src-tauri/tauri.conf.json`,
  `src-tauri/Cargo.toml`, and `src-tauri/Cargo.lock`'s own `codex-desktop` package entry — a 5th
  anchor not listed anywhere in the bundle's authoring docs, discovered and fixed this cycle via
  `cargo check --offline` (clean; only pre-existing, unrelated dead-code warnings).
  `buildVersionTriple.test.ts`'s `startsWith('0.6.')` assertion remains satisfied. Receipt:
  `artifacts/epic_4/version-bump-cycle_receipt.md`.

## DISCOVERED

- **`src/rules_core/encumbrance.rs` drops equipment weight for any non-CRB book's items.** Found by
  cycle 3.1 (ARG parity): `compute_encumbrance` resolves an equipped item's corpus record via the
  generic, book-agnostic `equipment_id_resolve` (works correctly across all books), but then looks up
  that item's **weight** via `crate::rules_core::rules_tables::crb::equipment_tables::equipment_tables()`
  — a compiled-in, CRB-only static table (confirmed: `encumbrance.rs:32`'s only import). A real ARG item
  (the Dogslicer) resolved correctly against the corpus but its weight silently fell into
  `unresolved_item_ids` instead of being counted (PCGen: 30 lbs, Codex: 29 lbs). Cross-validated as real
  and book-specific by cycle 3.2 (PU): PU's pilot, which didn't equip a non-CRB weighted item, shows no
  such gap (29=29). **Not fixed** — `src/rules_core/encumbrance.rs` is outside both parity cycles'
  file-touch partition. A future cycle needs authority to touch it; likely fix shape: resolve weight via
  the same corpus-generic path `equipment_id_resolve` already uses, not the CRB-only static table. See
  `artifacts/epic_3/advanced_race_guide_parity-cycle_receipt.md` for full root-cause detail.
- **`apps/desktop/src-tauri/Cargo.lock` is a 5th version anchor, not listed in any authoring doc.**
  Found by cycle 4.4: bumping `src-tauri/Cargo.toml`'s version does not update `Cargo.lock`'s own
  `codex-desktop` package entry — that requires a live `cargo` invocation (`cargo check --offline`
  used here) to resync. Future version-bump cycles should check this file explicitly. See
  `artifacts/epic_4/version-bump-cycle_receipt.md`.

## Open blockers

- ~~**Tier-1 launch-gate dependency:** SD-26's closure PR has not yet landed.~~ **CLEARED 2026-07-27** (corrected same day — an earlier note here said PR #339; that was backwards). SD-26 merged via **PR #338** — `62e7b617` is a confirmed ancestor of `origin/develop`, and the SD-26 package + `src/bin/sd26_gen_core_rulebook_cache.rs` are both present there. PR #339 is a separate, later CG-03 bugfix, unrelated to SD-26 closure. Caveat: SD-26's own `progress.md` on develop still shows its terminal `6.5` row as "awaiting operator merge" — stale paper-trail, not evidence the merge didn't happen. Per `decisions.md §7` + `loop-instruction.md §2` item 1.
- ~~**Bundle label discrepancy:** `SD-27` vs. `SD-27+ (unscheduled)` — operator's lever pull at cycle 2.0.~~ **CLEARED 2026-07-27.** Operator chose `"SD-27"`; resolved across all 21 stubs + registry + SD-26's `decisions.md:102` (already correct) + v0.6's risks doc. Per `decisions.md §2` + `artifacts/epic_2/label-resolution-cycle_receipt.md`.
- **CG-03 inherited baseline:** SD-27's per-book parity baseline asserts "match rate at cycle close," not 9-of-9. Per `forward-scope-register.md §"Class 0.3"` + `decisions.md §10`. **Exercised for real this run** — both books' `combat.baseline_melee_attack_bonus` mismatch (PCGen 5 / Codex 6) reproduces the exact same root cause SD-26 already documented; inherited, not chased, per cycle design.
- **v0.6 in-progress overlap:** v0.6 is actively working class/race breadth (Fighter/Wizard/Rogue + 8 remaining CRB classes). SD-27's partition restricts SD-27 cycles to the per-book content paths. Per `scope-draft.md §4`. Confirmed no collision occurred during this run (checked `origin/tranche/6` before dispatching the retrofit cycles).
- **NEW — concurrent per-book cycles share one working directory, not isolated worktrees.** `loop-instruction.md §8`'s "Worktree isolation... not needed" note (inherited from `decisions.md §8`) did not anticipate two per-book cycles both needing the same shared file (`src/rules_core/rules_tables/mod.rs`, not currently allow-listed by the partition at all). This run hit and self-corrected a real collision; it should not be relied on to self-correct every time. Recommend either `isolation: 'worktree'` for future concurrent per-book batches, or extending the partition to allow-list `rules_tables/mod.rs` under a serial-on-shared-file rule (matching how the registry file and `data/stubs/` are already handled).
- ~~**`src/rules_core/rules_tables/{advanced_race_guide,pathfinder_unchained}/` are not wired into `codex`'s public module tree.**~~ **CLEARED in Phase 2 (2026-07-31).** Both modules are registered in `rules_tables/mod.rs`, and the `pilot_compute.rs` integration this blocker warned about has since been done for both books under `decisions.md §28`. **One residue remains, deferred:** `src/bin/sd27_gen_book_cache.rs` still `#[path]`-includes the ARG module a *second* time into its own crate. That duplicate compilation is the sole reason several ARG items report as dead code; deleting the include should drop warnings off the 75-warning root clippy baseline. Logged at `docs/retro/events/spellgate.jsonl`.
- ~~**`decisions.md §8`'s file-touch partition forbids `src/rules_core/pilot_compute.rs` and `rules_tables/<book>/`.**~~ **RECORDED AS SPENT, 2026-07-31 (`decisions.md §28`).** §8 was a *concurrency* partition against v0.6's simultaneous class/race work — its own text says so. v0.6 closed and PR #342 merged, so there was nothing left to partition against, and two shipped defects (size modifiers on every Small character; PU's 4 classes grounded but unselectable) could not be closed without `pilot_compute.rs`. The remaining §8 prohibitions (`docs/release/v0.6/`, `src/oracle_validation/`) stand and were honoured throughout. **Standing guard, enforced:** every `pilot_compute.rs` change landed with a test pinning the before/after per affected race or class.

## Reporting manifest

The live cycle state is the reporting manifest `sd27_book_pre_build`, not this table — see
`loop-instruction.md §8`. Read it with:

```bash
python3 scripts/sd27-workflow.py status
```

6 items (2 books × pre_build/verify/parity — the license-prep stage is folded into pre_build, per
scope-draft.md §1.2.1; no cycle ever claims a standalone "license" item). This file and the manifest must agree; the
manifest is authoritative for status, this table for narrative.

## Cycle receipts

| Cycle | Receipt path |
|-------|--------------|
| 1.1 identifier-audit | `artifacts/epic_1/identifier-audit-cycle_receipt.md` |
| 2.0 label-resolution | `artifacts/epic_2/label-resolution-cycle_receipt.md` |
| 2.0.5 shape-b-license-stripping-preflight | `artifacts/epic_2/2.0.5-shape-b-license-stripping-preflight-cycle_receipt.md` |
| 2.0.6 crb-license-retrofit | `artifacts/epic_2/2.0.6-crb-license-retrofit-cycle_receipt.md` |
| 2.0.7 apg-license-retrofit | `artifacts/epic_2/2.0.7-apg-license-retrofit-cycle_receipt.md` |
| 2.0.8 acg-license-retrofit | `artifacts/epic_2/2.0.8-acg-license-retrofit-cycle_receipt.md` |
| 2.0.9 beastiary-license-retrofit | `artifacts/epic_2/2.0.9-beastiary-license-retrofit-cycle_receipt.md` |
| 2.0.10 all-23-books-license-conformance-verify | `artifacts/epic_2/2.0.10-all-23-books-license-conformance-verify-cycle_receipt.md` |
| 2.1 advanced_race_guide_pre_build | `artifacts/epic_2/advanced_race_guide_pre_build-cycle_receipt.md` |
| 2.1' advanced_race_guide_verify | `artifacts/epic_2/advanced_race_guide_verify-cycle_receipt.md` |
| 2.2 pathfinder_unchained_pre_build | `artifacts/epic_2/pathfinder_unchained_pre_build-cycle_receipt.md` |
| 2.2' pathfinder_unchained_verify | `artifacts/epic_2/pathfinder_unchained_verify-cycle_receipt.md` |
| 3.1 advanced_race_guide_parity | `artifacts/epic_3/advanced_race_guide_parity-cycle_receipt.md` |
| 3.2 pathfinder_unchained_parity | `artifacts/epic_3/pathfinder_unchained_parity-cycle_receipt.md` |
| 4.1 final-criterion-scan | `artifacts/epic_4/final-criterion-scan-cycle_receipt.md` |
| 4.2 architecture-closure | `artifacts/epic_4/architecture-closure-cycle_receipt.md` |
| 4.3 release-notes | `artifacts/epic_4/release-notes-cycle_receipt.md` |
| 4.4 version-bump | `artifacts/epic_4/version-bump-cycle_receipt.md` |
| 4.5 pr-merge | `artifacts/epic_4/pr-merge-cycle_receipt.md` |

---

# Phase 2 — Player Reachability: real end state

> Appended at closure, `tranche/7-1`, 2026-08-01. Phase 1's record above is left intact.
>
> **Every number below was derived by running a command at closure.** This bundle's documents have
> repeatedly carried figures nobody could reproduce; where a prior cycle report's number turned out
> wrong, the correction is stated rather than the number quietly replaced.

## Why there is a Phase 2

The operator's **2026-07-30 directive** redefined SD-27's definition of done: *"all data is ingested,
compute is available, and can reach the end user through the ui. there is not a single thing left to be
done for that thing to be utilized by a user."*

That is a different bar from Phase 1's, and it superseded two framings the bundle had shipped with:

* `README.md §1` / `technical-design.md:156` — "no new engine work, new class chassis, or new rule
  mechanics." **Superseded** (`decisions.md §24.2`).
* `decisions.md §8`'s file-touch partition. **Recorded as spent, not wrong** (`decisions.md §28`).

## Cycle record

Phase 2 ran as a swarm of bounded single-purpose cycles on `tranche/7` and `tranche/7-1` rather than
as numbered epic cycles, so there is no status-matrix row per cycle. The auditable record is
**21 commits** (`c79884f8..06641a54`) plus a 12-file working tree, and **51 actor shards** in
`docs/retro/events/`.

| Commit | What landed |
|---|---|
| `e5617855` | Shape B v1 `race` + `race_trait` payload types |
| `cdab4d10` | 18 in-scope race chassis + 175 standard racial traits ingested |
| `0a2c96ad` | ARG alternate racial traits ingested for the 18 in-scope races |
| `7be59136` | Book-agnostic race resolver + CRB 7-race regression pin |
| `a1c9f193` / `1c976a94` | 18 corpus races served through the desktop race catalog, surfaced to the player |
| `47045b70` | `decisions.md §27` — corrected two wrong published numbers |
| `7e593f59` / `5bb53bbb` | Raw PCGen `DESC:` syntax stopped reaching players, at ingest and at serve |
| `c82b7b47` | ARG alternate-racial-trait picker + honest catalog values |
| `3ff9b899` | Race sizes corrected, creation widened to 18, PU's 4 Unchained classes grounded |
| `ff8d9e02` / `9220a929` | `decisions.md §28` — §8's partition recorded as spent |
| `b4504c49` | PU's 4 Unchained classes reach players; size modifier reaches AC |
| `b86957fc` | 105 dead equipment rows closed; touch/CMB/CMD grounded in the engine; ART on a character |
| `f4dcb522` | Per-class spell levels for APG/ACG/ARG; Aasimar's 9 dead rows closed; reach gate sees ARG/PU |
| `c191ff3d` | **The two-compute-twins trap closed**; feat prerequisites enforced; spell picker filtered |
| `06641a54` | Flat-footed AC into the engine; monster catalog; **reach 22 → 26 of 27** |
| *(working tree)* | ARG feat magnitudes (8 more wired); cross-catalog `DESC:` guard; post-creation removal for feats/spells/equipment |

**Diff shape:** `git diff --stat c79884f8..HEAD` → **601 files changed, +70,739 / −958**.

## Content end state

**5,390 corpus content records across 6 books**, counted on disk
(`find data/corpus -name '*.json' -not -name 'LICENSE.json' -not -path '*/_parity/*' | wc -l`):
CRB 3,400 · APG 641 · **ARG 635** · ACG 423 · B1 164 · **PU 127**.

**Phase 2 added exactly 417 records**, every one a content kind no book had ever ingested:
156 `advanced_race_guide/race_trait` · 108 `beastiary/race_trait` · 67 `core_rulebook/race_trait` ·
64 `pathfinder_unchained/class_feature` · 11 `beastiary/race` · 7 `core_rulebook/race` ·
4 `pathfinder_unchained/class`.

Before this phase **no `data/corpus/*/race/` directory existed anywhere** (`decisions.md §25.5`) —
the CRB's 7 races lived only as a hardcoded 7-variant enum.

## Reach end state

**26 of 27 (book, content-kind) families reach a player.** The full per-book, per-content-kind table —
including the distinction between *catalog-reach* and *magnitude-reach*, which are not the same claim —
is in `release-notes.md §11`. It is not duplicated here, because two copies of a table is how this
repo's documents drifted in the first place.

**SD-27's own two books are 8 of 8 families catalog-reach-complete.**

**The one NO:** `beastiary1/race_traits` — `Duergar ~ Spell-Like Ability ~ Invisibility`, gated on
`Duergar_ReplaceSLAEnlargePerson`, which **no ingested book sets**. Its only setter anywhere is in
Monster Codex. **Upstream-blocked, not broken**, and proven so by an executable test that also proves
the mirror row *does* reach a player. Routed to **SD-29**; the test goes RED the day Monster Codex is
ingested, which is how the finding closes.

## Verification end state

`scripts/verify.sh --full --show-actuals -j 8`, 2026-08-01: **8 stages pass, 1 fails.**

```
  passed:  8  root-lib desktop reach frontend-install frontend-test
              frontend-typecheck clippy class-dump
  FAILED:  1  root-full
```

| Stage | Measured |
|---|---|
| root-lib | 1,426 passed, 0 failed |
| root-full | 5,848 passed / **5 failed** / 526 suites |
| desktop | 400 passed, 0 failed |
| reach | 16 passed |
| frontend-test | 97/97 files |
| frontend-typecheck | `tsc --noEmit` clean |
| clippy | root 75, desktop 7, **0 errors** — at ceiling, not raised |
| class-dump | 31/31 classes computing |

**The 5 root-full failures are the known environmental set**, all needing PCGen or pilot fixtures under
a hardcoded `/home/ubuntu/...` path absent on this box: `sd26_pcgen_runner`,
`sd26_pilot_case_verification`, `sd27_advanced_race_guide_parity`, `sd27_pathfinder_unchained_parity`,
`v06_wizard_pilot_case_verification`. **Pre-existence proven per file, two ways** —
`git diff --stat c79884f8..HEAD -- <file>` *and* `git diff --stat HEAD -- <file>` are **both empty
(0 bytes) for all five**. Cross-checked so a larger failure cannot hide behind the suite count: summed
`failed;` = 5 and `test result: FAILED` lines = 5.

`scripts/verify-baselines.env` was updated; **floors only rose**, and every delta was derived by
counting the tests themselves rather than by subtracting two measurements:

| Floor | Δ | Reconciliation |
|---|---:|---|
| `BASELINE_ROOT_LIB_TESTS` 1,412 → 1,426 | +14 | `feat_effects.rs` +9, `pcgen_desc.rs` +5, `pilot_compute.rs` +0 |
| `BASELINE_ROOT_FULL_TESTS` 5,825 → 5,848 | +23 | root-lib's +14 **plus** `tests/sd27_arg_and_pu_feat_effects.rs` +9 |
| `BASELINE_DESKTOP_TESTS` 385 → 400 | +15 | `character_hub.rs` +10, `equipment_catalog.rs` +4, `feat_catalog.rs` +1, `main.rs` +0 |
| `BASELINE_FRONTEND_TEST_FILES` 96 → 97 | +1 | `boundary/removeSelection.test.ts` |
| `BASELINE_ROOT_TEST_BINARIES` 526 | **0** | no new test file, no new bin target — measured, not assumed |
| clippy ceilings 75 / 7 | **0** | at ceiling, **not raised** |

## DISCOVERED (Phase 2)

Four findings that outlived the cycles that produced them, recorded in full at `decisions.md §29`
because each is a *shape* that will recur on the next book:

1. **§29.1 — The two-compute-twins trap.** `pilot_compute.rs` and `pilot_compute_corpus.rs` derive the
   same pillars independently. The corpus twin — **the one whose numbers reach the sheet** — read
   `feat_effects` **zero** times while the hardcoded twin read it 34. A feat wired into the tested twin
   passed its test and changed nothing on screen: a **false-green generator**. Five feats were live in
   that state, **three of them Core Rulebook**. Closed by a single shared seam plus a structural *and*
   a behavioural guard.
2. **§29.2 — There is a third twin, in TypeScript.** Closing the Rust seam does not close the class of
   defect. Three cells moved into the engine this phase; **one remains and is measured, not suspected**
   — the headline HIT POINTS panel is computed frontend-locally and ignores
   `feat_effects::hp_bonus_from_feats`, so a Toughness holder reads **two different max-HP values on
   the same sheet**. The generalisation is the finding: *any* surface that re-derives a rules number
   instead of rendering an engine explanation is a candidate twin.
3. **§29.3 — The reach gate has two blind spots.** (a) *Function-wrapped tables* — the source scan
   matched column-zero `pub const NAME: &[Type]` only, so PU (whose records sit inside
   `pub fn equipment_tables()` / `pub fn feat_tables()`) was invisible to it **and** absent from the
   ingest diagnostic simultaneously; the gate asserted nothing about the book in either direction while
   its records were already reaching live commands. Closed. (b) *Hand-modelled tables* — a
   `decisions.md §24` pure function is not a record slice and is invisible to the scan **by
   construction, permanently**. Within PU alone, `rogue_features.rs` and `summoner_features.rs` are
   unseen while `barbarian_features.rs` and `monk_features.rs` are seen. **§24 and the source scan are
   in permanent tension**, and `data/corpus/` is the load-bearing discovery source for §24-shaped
   content.
4. **§29.4 — `SOURCEPAGE:p.xx` is a placeholder.** Across the 18 in-scope races' 175 standard trait
   rows, **143 carry `p.xx` and only 32 carry a real page**. Transcribing verbatim would have
   manufactured **143 false citations**. Every ingest binary maps `p.xx` → `None` while preserving the
   raw token, and a test pins it. The generalised rule: *a page is trustworthy only when it is not
   `p.xx`, and that must be checked per row rather than assumed per content-kind.*

## Open blockers (Phase 2)

Nothing blocks closure. The material open work is the **deferral register at `release-notes.md §13`**,
grouped by *why* each item was left and routed to where it should land — **72 `deferral` events** in
`docs/retro/events/*.jsonl`, alongside 85 corrections, 26 incidents, 4 near-misses and 4 reworks.

The four that most constrain the next cycle:

1. **Missing engine dimensions gate the largest magnitude gaps.** 24 of ARG's 49
   unconditionally-bonused feats and 142 of its 153 alternate racial traits change no computed number —
   not because they are unwired, but because the total they target does not exist
   (`ABILITYPOOL` choosers, spell-like abilities, per-day resource pools, fly manoeuvrability,
   companion levels, natural-weapon damage dice). `tests/sd27_alternate_racial_trait_reachability.rs`
   re-derives the reachable set every run and **fails, naming them**, the day the surface widens.
2. **Two books block content that is otherwise ready.** Ultimate Magic / Ultimate Combat (**SD-28**)
   for 46 of the Unchained Summoner's 202 spells; Monster Codex (**SD-29**) for the one unreached
   Bestiary 1 record. B2/B3/B4 (**SD-28**) for 19 of ARG's 37 races.
3. **Two stale `LICENSE.json` record counts**, uncorrected because no cycle owned the file:
   `core_rulebook` reports 3,326 against 3,400 on disk, and `beastiary` reports 45 against 164. **Both
   gaps are exactly this phase's own additions** (74 = 7 race + 67 race_trait; 119 = 11 race +
   108 race_trait), which is what makes them provably stale rather than suspicious.
   `tests/sd27_book_license_record_counts.rs` covers PU and ARG only — both **correct** — and extending
   its `BOOKS` const to the other two is the one-line change that would have caught this. Left
   deliberately as the only edit needed.
4. **Shared-working-tree concurrency, again.** Phase 1 flagged a collision on `rules_tables/mod.rs` and
   recommended worktree isolation. Phase 2 hit the same class **three more times** as `git stash`
   incidents: agents measuring a "before" baseline by stashing repeatedly stashed *concurrent agents'*
   uncommitted work, and one stash was popped by another process mid-run, producing a junk
   13-failure measurement. No work was lost. **`isolation: 'worktree'` should be the default for any
   future multi-agent batch, not a recommendation.**
