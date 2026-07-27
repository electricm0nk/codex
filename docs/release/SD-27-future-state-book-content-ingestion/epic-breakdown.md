# SD-27 — Epic Breakdown

> **Rewritten 2026-07-27 to the committed 2-book scope.** This file is what the orchestrator reads
> (`loop-instruction.md §3`). It previously carried the pre-directive 19-book / 45-cycle plan while every
> other document in the bundle described a 2-book scope — a workflow driven off it would have dispatched
> ~43 out-of-scope cycles. The 19-book plan is preserved as the SD-28+ forward plan in §6; it is **not**
> SD-27's dispatch list.
>
> **In-scope books:** Advanced Race Guide (ARG) + Pathfinder Unchained (PU), matching the operator's
> `SD-27 (ARG + PU)` dashboard workchannel. Adventurer's Guide is routed to SD-30.
>
> **Cycle IDs are the receipt filenames.** The `pre_build` / `verify` / `parity` names below are the same
> ones `loop-instruction.md`, `progress.md` and `scripts/sd27-workflow.py` use, so
> `artifacts/<epic>/<book>_<stage>-cycle_receipt.md` resolves deterministically from any of them.

## 1. Epic 1 — Code-Side Identifier Cleanup (governance base; fires FIRST)

Single cycle. Defensive scope; the runner audits the existing identifier shape, no renames expected.

| Criterion | Cycle | Owner | Tier | Notes |
|-----------|-------|-------|------|-------|
| 1.1 | `identifier-audit` | Backend | Sonnet | Standard audit; produces `artifacts/epic_1/identifier-audit-cycle_receipt.md` |

## 2. Epic 2 — Label Resolution, License Pre-flight, and Book Pre-Build

### 2.1 Bundle-level cycles (serial; each gates the next)

| Criterion | Cycle | Owner | Tier | Notes |
|-----------|-------|-------|------|-------|
| 2.0 | `label-resolution` | Backend | Sonnet | Per-cycle blocking decision; resolves `SD-27` vs. `SD-27+ (unscheduled)`. Operator pulls the lever. Gates 2.0.5 |
| 2.0.5 | `shape-b-license-stripping-preflight` | Backend | Sonnet | Shape B v1 schema + PI-blacklist + per-book `LICENSE.json` template. Gates 2.0.6 |
| 2.0.10 | `all-23-books-license-conformance-verify` | Backend | Sonnet | Terminal verify across all 23 books (4 in-scope + 19 future-state). Gates 2.1 |

### 2.2 In-scope book license retro-fit (4 cycles, parallel — file-disjoint)

| Criterion | Cycle | Owner | Tier | Notes |
|-----------|-------|-------|------|-------|
| 2.0.6 | `crb-license-retrofit` | Backend | Sonnet | `data/corpus/core_rulebook/` |
| 2.0.7 | `apg-license-retrofit` | Backend | Sonnet | `data/corpus/advanced_players_guide/` |
| 2.0.8 | `acg-license-retrofit` | Backend | Sonnet | `data/corpus/advanced_class_guide/` |
| 2.0.9 | `beastiary-license-retrofit` | Backend | Sonnet | `data/corpus/beastiary/` — note the corpus dir is `beastiary`, not `beastiary1`; only `rules_tables/beastiary1/` carries the digit |

### 2.3 Future-state book pre-build + verify (4 cycles, 2 books × 2 stages)

The two books are file-disjoint and therefore parallel-capable; stages within a book are serial. In
practice the operator's "tune, then go wide" model gates the books serially too (`scope-draft.md §1.2`)
— ARG closes or fails before PU dispatches. Stage ordering is enforced mechanically by the `depends_on`
chain in the reporting manifest, not by convention — see §5.

| Criterion | Cycle | Owner | Tier | Notes |
|-----------|-------|-------|------|-------|
| 2.1 | `advanced_race_guide_pre_build` | Backend | Sonnet (or free/discounted) | ARG; 23 `.lst` files at source. Operator-gated on failure |
| 2.1' | `advanced_race_guide_verify` | Backend | Sonnet (or free/discounted) | ARG; dual-audit + schema conformance over the cache |
| 2.2 | `pathfinder_unchained_pre_build` | Backend | Sonnet (or free/discounted) | PU; 11 `.lst` files at source. Operator-gated on failure |
| 2.2' | `pathfinder_unchained_verify` | Backend | Sonnet (or free/discounted) | PU; same shape as 2.1' |

### 2.4 Per-book cycle story (per `loop-instruction.md §3.3`)

```
Per-book pre-build cycle (E2.1 / E2.2):
1.  Claim the reporting item:
    python3 scripts/sd27-workflow.py claim sd27.<book>.pre_build --agent backend
2.  Read source LST corpus at $PCGEN_DATA_ROOT/<book>/
3.  Inventory content kinds (classes, spells, equipment, feats, races, bestiary, etc.)
4.  Generate src/rules_core/rules_tables/<book>/ Rust module (Shape B v1 enumeration)
5.  Run the sd27_gen_book_cache codegen tool against the rules_tables module
6.  Write data/corpus/<book>/{content_kind}/{content_id}.json per Shape B v1
    (license field populated, PI redaction complete)
7.  Write data/corpus/<book>/LICENSE.json
8.  Write tests/sd27_<book>_cache_shape.rs (key-set + key-order conformance)
9.  Update docs/governance/wired-integration-stubs-registry.md: the book_stub entry's
    Status -> "Resolved" + resolved_at + resolved_by + bundle_of_record + cycle_receipt
10. Update data/stubs/<book>.json: content_kind_counts -> real number map
11. Run the dual-audit gate:
      bash scripts/identifier-discipline-audit.sh   -> OK_NO_BUNDLE_TAGS
      bash scripts/wired-integration-audit.sh       -> AUDIT PASSED
12. Commit + push + write the receipt
13. Report completion:
    python3 scripts/sd27-workflow.py complete sd27.<book>.pre_build --receipt <path>
```

Steps 9 and 10 touch files shared with the other book's cycle — serialize on them (one cycle at a
time), per `decisions.md §8`.

## 3. Epic 3 — PCGen Parity Baseline (2 fixtures)

2 cycles, one per in-scope book. Each is serial-after that book's `verify` stage.

| Criterion | Cycle | Owner | Tier | Notes |
|-----------|-------|-------|------|-------|
| 3.1 | `advanced_race_guide_parity` | Backend | Sonnet (or free/discounted) | Serial-after 2.1' |
| 3.2 | `pathfinder_unchained_parity` | Backend | Sonnet (or free/discounted) | Serial-after 2.2' |

### 3.1 Per-book parity cycle story

```
Per-book parity cycle (E3.x):
1. Claim: python3 scripts/sd27-workflow.py claim sd27.<book>.parity --agent backend
2. Read data/corpus/<book>/ cache (this bundle's own E2.x output)
3. Author the pf_<book>_human_<class>_level1_golden.pcg fixture, mirroring SD-26's
   pilot Fighter pattern at
   tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt
4. Run the existing PCGen Gradle pipeline against the fixture
   (scripts/pcgen-run-character.sh, scripts/pcgen-normalize-output.py)
5. Sanitize output; write data/corpus/<book>/_parity/pf_<book>_human_<class>_level1.json
6. Run src/oracle_validation/comparator::compare against the per-book receipt
7. Record the per-dimension match/mismatch table in the receipt
8. Document the inherited CG-03 baseline (7-of-9 ceiling) in the receipt —
   the assertion is "match rate at cycle close", not 9-of-9
9. Commit + push + receipt
10. Report: python3 scripts/sd27-workflow.py complete sd27.<book>.parity --receipt <path>
```

## 4. Epic 4 — Closure Epilogue (fires LAST)

| Criterion | Cycle | Owner | Tier | Notes |
|-----------|-------|-------|------|-------|
| 4.1 | `final-criterion-scan` | Backend | Sonnet | Per-criterion terminal-state table cross-checked against 3 independent sources |
| 4.2 | `architecture-closure` | Backend | Opus | `bash scripts/architecture-truth-up.sh` + `bash scripts/graphify-update.sh` + PR; template §2's adversarial-verify |
| 4.3 | `release-notes` | Backend | Haiku | Populates the 7-section `release-notes.md` template |
| 4.4 | `version-bump` | Backend | Haiku | 0.6.0 → 0.6.1 per `major.tranche-base.build` |
| 4.5 | `pr-merge` | Backend | Sonnet | Operator merges per standing convention |

## 5. Total cycle count

| Epic | Cycles |
|------|--------|
| E1 | 1 |
| E2 | 3 bundle-level + 4 in-scope retro-fit + 4 per-book (2 books × pre-build/verify) = 11 |
| E3 | 2 |
| E4 | 5 |
| **Total** | **19 cycles** |

The 6 per-book cycles (2 books × pre_build, verify, parity — license prep is folded into pre_build,
per scope-draft.md §1.2.1) are also the 6 items in the
reporting manifest `sd27_book_pre_build` — see `loop-instruction.md §8`. The manifest's `depends_on`
chain is what actually enforces stage ordering: `python3 scripts/sd27-workflow.py claim` refuses an
out-of-order claim, so a mis-sequenced dispatch fails loudly instead of silently producing a cache
before its license stage ran.

## 6. SD-28+ forward plan (NOT SD-27's dispatch list)

The 17 deferred future-state books. The per-book cycle pattern above is templated and reusable; the
operator picks the next batch after SD-27's E4.5 closes. The dashboard already routes these across
three workchannels:

| Workchannel | Books |
|---|---|
| SD-28 (Ultimate) | ultimate_campaign, ultimate_combat, ultimate_equipment, ultimate_intrigue, ultimate_magic, ultimate_wilderness (6) |
| SD-29 (Bestiary) | bestiary_2, bestiary_3, bestiary_4, bestiary_5, bestiary_6, bonus_bestiary, monster_codex (7) |
| SD-30 (Adventure+) | adventurers_guide, mythic_adventures, occult_adventures, horror_adventures (4) |

Beginner Box and Core Essentials are **not** in this table — they were removed from scope per operator
directive 2026-07-27 (redundant to other tomes; will not be brought in). Their stub manifests and
registry entries `#0005` / `#0012` remain on disk, out-of-scope rather than deleted.

## 7. Cross-reference

- `./scope-draft.md` — the committed scope (§3 is the per-book cycle map).
- `./decisions.md` — decision record (§8 file-touch partition; §19 dispatch; §20 reporting).
- `./technical-design.md` — architectural surface.
- `./technical-requirements.md` — pre-loop prerequisites + normative requirements.
- `./loop-instruction.md` — per-cycle procedure (§8 is the reporting contract).
- `./progress.md` — live cycle log.
- `./artifacts/README.md` — per-cycle receipt structure.
- `scripts/sd27-workflow.py` — the runnable dispatch-state driver.
