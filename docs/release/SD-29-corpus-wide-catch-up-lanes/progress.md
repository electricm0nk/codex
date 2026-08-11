# SD-29 — Per-cycle Receipts

This file carries the per-cycle receipt for SD-29. Each cycle appends a
new section with the cycle-id and the operator-readable per-cycle facts.

The supervisor reads this file to verify completion before the next cycle
claim (per `decisions.md §14a` local-file dispatch + `loop-instruction.md`
§"Step 6").

## Cycle 0.0 — Chassis Land (planning-ready)

**Date:** 2026-08-01
**Cycle ID:** `SD29-LAND-1`
**Operator:** Todd Hintzmann (directive 2026-08-01)
**Surface:** this directory (`docs/release/SD-29-bestiary-line-book-ingestion/`)

### What landed

- 15-file canonical chassis per the spec-domain-bundle-authoring skill (matching SD-22 through SD-28's published shape).
- Per-doctrine amendments per operator directive 2026-08-01:
  - **Decision §13** — `tranche/9` branch, no Hermes board (parallel to SD-28's `tranche/8`).
  - **Decision §14** — `0.9.<build>` build version.
  - **Decision §14a** — Hermes board retired, local-file dispatch.
  - **Decision §15** — four-bestiary list confirmed (B2-B5) with cycle-0 shape gating.
  - **Decision §16** — cross-book conflict rule (newer = doctrine).
  - **Decision §17** — bulk modifications deferred.
  - **Decision §18** — Bestiary 5 shape-resolution (player-options cycles, not monster-block).
  - **Decision §19** — reach gate is the definition of done; engines only when strictly necessary; rules-as-data with pre-computed values (supersedes §12).
  - **Decision §20** — operator ack-chain recorded.

### Pre-launch state

| Check | Status |
|-------|--------|
| `kanban.md` exists | DONE 2026-08-02 — kanban.md present, 13 dispatch-ordered cards (epics 1–13) |
| Branch `tranche/9` pushed to origin | PENDING (operator action at cycle launch) |
| OAuth credentials valid | PENDING (operator action at cycle launch) |
| Working tree clean | ASSUMED (pre-launch verification) |
| Cycle-0 trap-report + work-inventory for B2-B5 | PENDING (Epic 2 cycle) |

### Next cycle

The next cycle is Epic 2's pre-flight: claim from the existing 13-card `kanban.md` board (epics 1-13 as dispatch-ordered cards); verify branch + OAuth + tree state; run cycle-0 trap-report + work-inventory for all four bestiaries (B5's shape finding determines Epic 6's cycle shape).

## Cycle 0.0+1 — Unattended-mode acknowledgment (operator directive 2026-08-01)

**Date:** 2026-08-01
**Cycle ID:** `SD29-LAND-2` (unattended-mode directive landing)
**Operator:** Todd Hintzmann (out of town per directive)
**Surface:** this directory (`docs/release/SD-29-bestiary-line-book-ingestion/`)

### What landed

The operator is out of town and may not see the harness's output for days. Per
operator directive 2026-08-01, this bundle operates in **unattended mode**.

Cycles MUST NOT pause to ask the operator questions. The operator's verbatim:

> "include instructions to all 3 that indicate they will be running in unnattended
> mode since i will be out of town while this runs. They may not stop to ask
> questions - it might be days before i notice."

The doctrine is mirrored across three files:

- `loop-instruction.md` §"OPERATING METHOD" sub-callout (cycle supervisor reads it first).
- `decisions.md` Decision §22 (load-bearing doctrine entry).
- `progress.md` Cycle 0.0+1 (this entry — per-cycle receipt confirms the operator-on-record).

The receipt chain is the operator's after-return review surface. When the
operator returns, the cycle receipts in this file carry the per-cycle decisions
that the harness made on its behalf.

### Operating protocol summary (mirror of `decisions.md §22`)

1. Default-and-flag, not ask.
2. No `clarify` tool calls.
3. Blockers are recorded, not raised.
4. `decision-blocked` IS allowed.
5. Closure is a goal, not a stop signal.

### Bundle-specific unattended-mode notes

Epic 7 (DM Toolkit extension, consumer of Bestiary 2-5 records) is the most
likely place where the cycle will want an operator decision. The unattended-mode
protocol routes Epic 7's resolution through `successor-forward-scope-register.md C3.1`
retrofit — record `decision-blocked` in this file with the recorded reason and
proceed on the safe default (retrofit; Epic 7 lands inside SD-29's epic
structure only if Epic 3-6 per-book cycles complete with reach-gate claims and
the toolkit-extension scope is operator-pinned in scope before Epic 7 fires).

## Cycle 0.0+2 — Kind-lane re-cut (operator directive 2026-08-10)

**Date:** 2026-08-10
**Cycle ID:** `SD29-LAND-3`
**Operator:** Todd Hintzmann (directive: "The SDs are our bodies of work. If our plan for SD-29
needs to be completely rescoped to address something else, then that is where we need to make
those updates. After the PR is merged, we will start SD-29 in tranche/9. That needs to be defined
and recorded.")
**Surface:** this directory (`docs/release/SD-29-bestiary-line-book-ingestion/`) and
`docs/release/corpus-work-channels.md`.

### What landed

`decisions.md §36` (2026-08-10, same day) had already ruled SD-29 partitions by kind, not by book,
following the `file_kind()` correction that moved Bestiary 1 from 620 `race_trait` to 21
`race_trait` + 523 `monster_ability`. The ruling had not been executed — `epic-breakdown.md` still
carried Epics 3-6/11-13 as seven per-book epics. `decisions.md §37` (this cycle) executes it:

- `epic-breakdown.md` rewritten in full: 11 epics — Epic 3 (new, Provenance Gate: PI-screening)
  plus Epics 4-7 (kind lanes: Monster/Monster-Ability chassis [2,159 units, pilot-then-extend on
  Bonus Bestiary], Race-Trait [1,124 units, defect-fix-alongside], Companion [275 units], Residual
  proven-path content [203 units, excludes `class_feature`'s 90 units]) replace the retired
  per-book Epics 3-6/11-13. Epic 7 (DM Toolkit) renumbered Epic 8; Epic 8 (Closure) renumbered
  Epic 11; Epic 9 (Build Version) and Epic 10 (Code Review) keep their numbers.
- `kanban.md` cards rewritten to match: lane cards (with the monster lane split into pilot +
  extend), renumbered dependency chain.
- `acceptance-and-verification.md`: new AT-29-003a (provenance gate); AT-29-008 repurposed
  (retired the Bestiary-5-specific fallback-cycle-type question, which the lane structure no
  longer needs); every Epic cross-reference renumbered.
- `scope-draft.md`, `technical-requirements.md`, `technical-design.md`, `loop-instruction.md`,
  `risks-and-open-questions.md`, `successor-forward-scope-register.md`, `release-notes.md`,
  `README.md`: every per-book epic cross-reference updated to the lane structure; the
  "SUPERSEDED IN PART" banner on `epic-breakdown.md` (which had pointed at this exact gap since
  2026-08-01) is removed, since the re-cut it called for has landed.
- Provenance (`../corpus-work-channels.md §6`, "Blocking before the first channel runs"):
  resolved for OGL/attribution — `docs/governance/license-matrix.md` (commit `314a7ad9`) already
  establishes it for all seven SD-29 books. **Not resolved** for PI-screening — the matrix found
  zero PI-screening anywhere in `rules_tables/*.rs` (the pipeline every SD-29 lane writes into)
  and three real leaks in other bundles' tables of the same pipeline. Epic 3 gates every lane on a
  per-lane PI-blacklist sweep before that lane's first content commit.
- `../corpus-work-channels.md` committed (was untracked) so the channel analysis this re-cut
  executes is not lost.
- Correction recorded: `corpus-work-channels.md §4`'s own "SD-29's 7 books" kind totals
  (`monster_ability` 1,869 / `monster` 1,143 / `race_trait` 1,145 / `companion` 334) are the
  **eight**-book sum including Bestiary 1, not SD-29's actual seven-book sum (1,346 / 813 / 1,124
  / 275 respectively). See `decisions.md §37.0.1`.

### Verification

- Every figure in `decisions.md §37` re-derived via `python3` against `docs/work-inventory.json`;
  commands included inline.
- `git status --porcelain` checked before staging; only the explicit paths this cycle touched were
  staged (docs-only, no `.rs`/`.tsx`/scripts).

### Next cycle

Epic 1 (Identifier Cleanup) is still the first cycle to fire once SD-29 launches on `tranche/9`
(cut from the post-SD-28 tip, per `decisions.md §34`). This re-cut is a docs-only change; no
tranche/9 branch exists yet and no lane cycle has run.

## Cycle 0.0+3 — Corpus-wide re-scope (operator directive 2026-08-10)

**Date:** 2026-08-10
**Cycle ID:** `SD29-LAND-4`
**Operator:** Todd Hintzmann (directive: "What I'm really after is establishing lanes that we can
use to rapidly catch up all the books in parallel — both those we have touched and those we have
not touched.")
**Surface:** this directory (renamed `docs/release/SD-29-bestiary-line-book-ingestion/` →
`docs/release/SD-29-corpus-wide-catch-up-lanes/`, via `git mv`) and
`docs/release/corpus-work-channels.md`.

### What landed

`decisions.md §37` (same day, prior cycle) re-cut SD-29 into kind lanes but kept the lane scope
pinned to the retired seven-book set. This cycle supersedes that boundary — `decisions.md §38` is
the corpus-wide re-scope:

- Directory renamed via `git mv` (history preserved); every internal cross-reference and every
  outside-package reference to the old path updated.
- `decisions.md §38` added: every corpus figure re-derived independently from
  `docs/work-inventory.json` (commands included), not transcribed from the driving brief. One
  correction found: the brief's `feat` remaining figure (1,348) undercounted by 2 — it silently
  excluded 2 `deferred-with-reason` units; the correct figure is 1,350. All other brief figures
  (corpus totals, per-kind held/remaining, Bestiary 1's 4.8%) checked out exactly.
- `epic-breakdown.md` rewritten: Epic 4 is now the corpus-wide Proven-Path Content Lanes tier
  (equipment, feat, spell, equipment_modifier, race, class — day-one parallel, no mechanism);
  Monster/Monster-Ability moved Epic 4→5, Race-Trait Epic 5→6, Companion Epic 6→7, each now
  corpus-wide with a named pilot book. Epics 1-3, 8-11 renumbering-adjacent text updated
  (Epic 8's gate is now Epic 5's pilot, not Epic 4's).
- `kanban.md` cards rewritten to the new numbering; Epic 4 split into three proven-path cards
  (equipment+equipment_modifier, spell, feat+race+class); Epics 5-7 each split into pilot/extend
  cards.
- `README.md`, `scope-draft.md`, `risks-and-open-questions.md`,
  `successor-forward-scope-register.md`, `forward-scope-register.md`, `technical-design.md`,
  `technical-requirements.md`, `acceptance-and-verification.md`, `loop-instruction.md`: banners
  and cross-references added/updated pointing at `decisions.md §38`'s corpus-wide scope and the
  Epic 4→5/5→6/6→7 renumbering; seven-book history preserved, not deleted.
- `risks-and-open-questions.md`: new R-29-009 and OQ-29-004 record the SD-30 collision — SD-30's
  sixteen-book list is now a subset of SD-29's corpus-wide lane scope. **Flagged, not resolved** —
  SD-30's own package was not touched, per explicit brief instruction.
- `corpus-work-channels.md §9.4`'s deferral marked superseded a second time (once by `§37` for
  partitioning, once by `§38` here for book-list scope).
- `class_feature` (15,472 units corpus-wide, up from the 90-unit seven-book figure) stays out of
  every lane; successor owner named as whichever bundle executes
  `corpus-work-channels.md §9.1`'s funded per-class archetype measurement (not yet assigned an SD
  number).
- Bestiary 1 (951 units, 4.8% proven, 901 `not-ingested`) confirmed simply in scope under the
  corpus-wide lanes — no separate epic, no separate receipt track, no operator decision required.

### Verification

- Every figure re-derived independently via `python3` against `docs/work-inventory.json`; commands
  included inline in `decisions.md §38`.
- `git status --porcelain` checked before staging; only explicit SD-29-package and
  `corpus-work-channels.md` paths touched (docs-only). The ~15 uncommitted files from the
  concurrent SD-28 session on this branch were left untouched — none staged, none read as part of
  this cycle's edits beyond the initial `git status` scan.
- No `git add -A`/`git add .` used; no `git stash` used.

### Next cycle

Same as Cycle 0.0+2's: Epic 1 (Identifier Cleanup) is still the first cycle to fire once SD-29
launches on `tranche/9`. This re-scope is docs-only; no `tranche/9` branch exists yet and no lane
cycle has run. The SD-30 collision (OQ-29-004) is an open item for the operator to resolve before
or during launch — this package does not block on it, but dispatch discipline must avoid
double-claiming the same (kind, book) cell until it is. *[Resolved later the same day, 2026-08-10:
SD-30 re-scoped to the class_feature/archetype bundle (`bc8f5fac`); OQ-29-004 CLOSED, R-29-009
RESOLVED — see `decisions.md §38.5`'s resolution note.]*

## Pre-launch readiness audit (2026-08-02)

**2026-08-02 pre-launch readiness pass (operator-side):** branch tip at audit: b63cda4e on tranche/8 (tranche/9 cut deferred to SD-29 launch per decisions.md §34). Scope operator-pinned to the 7-book cut (adds bestiary_6, bonus_bestiary, monster_codex — Epics 11–13). Launch-readiness fixes applied across the package; decisions §21–§34 landed. Sequential launch after SD-28 closure.

## Pre-launch readiness audit (2026-08-10, post-re-scope)

**Branch tip at audit: 462c40bc on tranche/8.** First full-package audit since the 2026-08-10
re-cut (`a8cac700`, `decisions.md §37`) and corpus-wide re-scope (`472acb4f`, `§38`). Every path
cited by the package's 14 docs was re-verified on disk; remediation landed in this commit.

**Verdict: NOT GO yet — sequencing-blocked on operator actions; the package itself is
launch-capable after this remediation.** Blockers outside this package's write scope:

1. **SD-28's promotion PR #359 (`tranche/8` → `develop`) is OPEN.** `decisions.md §34` requires
   `tranche/9` be cut from `develop` *after* that PR merges; `develop` is 137 commits behind
   `tranche/8`. (Note: `loop-instruction.md` item 5 uses the weaker "closure receipt exists"
   test, which IS satisfied — SD-28's closure receipt is dated 2026-08-10. The §34 branch-cut
   rule remains the binding gate.)
2. **`tranche/9` does not exist** — correct per §34 (the SD-29 launch session cuts+pushes it
   post-merge). `tranche/10` (SD-30 prep, cut 2026-08-01 from an early tranche/8 tip) already
   exists, so tranche numbering will land out of chronological order; recorded, not fixed.
3. **SD-28 closed by operator decision at state `d0402a19`**, with its `epic-30-integrity` and
   `epic-10-closure` cards never claimed and 8 cards in-flight — its closure receipt says so
   explicitly. SD-29 inherits SD-28's standing caveats: 8 `OPEN_FINDINGS` entries in
   `reach_gate.rs` (`beastiary1/race_traits` + seven `<book>/archetypes`, the latter SD-30's).
4. **`docs/retro/kind-lane-refactor-proposal.md`** (2026-08-09, "proposal, not a decision")
   names a prerequisite refactor (§4) it says per-kind parallelism depends on — review before
   the first parallel lane dispatch.

**Package remediation applied in this commit (docs-only):**

- `artifacts/` directory created with README — the trap-report sink TR-29-003/AT-29-003 cite
  did not exist.
- Stale `reach_gate.rs:840` line pins re-pointed to `:986` (with dated notes) in
  `scope-draft.md`, `loop-instruction.md` (×2), `decisions.md` (§10 supersession note, §19),
  `successor-forward-scope-register.md`; `monsters_reach()` span re-pinned to `:1300`.
- "Sole surviving `OPEN_FINDINGS` entry" corrected to eight entries (dated corrections) in
  `scope-draft.md`, `successor-forward-scope-register.md`, `decisions.md §10`;
  `loop-instruction.md` DoD item 6 now tells cycles to leave the seven archetype entries
  standing (SD-30's).
- Seven-book residue in live text rewritten corpus-wide: `epic-breakdown.md` Epic 1 audit
  scope; `technical-design.md` "Cycle paths" + monster-lane sections (pre-§38 epic mapping);
  `technical-requirements.md` TR-29-001 example, produced-artifacts list, success definition;
  `acceptance-and-verification.md` AT-29-001/-002/-008 and exit-checklist items (incl. the
  `class_feature` figure 90 → 15,472 per §38.4).
- §38.0 denominator note added (tables run over 38 books/38,536 units incl. `beginner_box`;
  product scope is 37/38,517).
- SD-30-collision resolution propagated to the three spots still saying "unresolved"
  (`README.md`, `decisions.md §38.5` heading, the SD29-LAND-4 receipt above).
- `tests/sd27_pu_class_features.rs` ledger citation corrected to
  `tests/sd27_pu_class_features_reach_by_corpus_key.rs`.
- Outside the package: SD-28 `forward-scope-register.md` C1.1 owner path (two renames stale)
  re-pointed to `SD-29-corpus-wide-catch-up-lanes/`.
- Checked and left alone: `decisions.md §14`'s `scripts/workflow-dispatch.sh` mention is a
  deliberate negative reference (names what SD-29 does *not* use); README §"Two SD-29
  packages" seven-book text is preserved-as-authored consolidation history.

Verify.sh not re-run: diff is markdown-only (plus one new README), no Rust/test surface
touched; working tree was clean before and after, no concurrent writers observed.

---

(c) Per-cycle receipts append below this line as cycles fire.

---

## SD29-E1-F1-001 — Epic 1, Code-Side Identifier Cleanup — COMPLETE

**Card:** `epic-1-identifier` (kanban.md Order 1). **Actor:** `sd29-e1-identifier`.
**Branch:** `tranche/9`. **Branch tip at claim:** `a8bb6716`. **Cycle date:** 2026-08-10.
**Mode:** unattended (no operator questions asked; no `clarify` call).

### Outcome

Epic 1's three SD29-E1-F1 acceptance criteria and SD29-E1-F2 are met, and the audit that
*asserts* them was found to be under-powered and repaired. The cycle's substance is that
repair — not a clean grep, which was already true on arrival.

### Re-derived figures (every number below carries the command that produced it)

- **`sd29_`/`SD29_`/`Sd29` identifiers in shipping source: 0.** Three `sd29`-shaped hits exist
  and all three are prose (`SD-29` in a comment), not identifiers:
  `grep -rniE 'sd[-_]?29' --include='*.rs' --include='*.ts' --include='*.tsx' src apps/desktop/src apps/desktop/src-tauri/src | wc -l`
  → **3** (`apps/desktop/src/sd21/buildVersionTriple.test.ts:71`,
  `apps/desktop/src/releaseChecks/buildVersionTriple.test.ts:43`,
  `apps/desktop/src-tauri/src/reach_gate.rs:1565`).
- **`t_<hex8+>` kanban tokens in shipping source: 0.**
  `grep -rnE '\bt_[0-9a-f]{8,}\b' --include='*.rs' --include='*.ts' --include='*.tsx' src apps/desktop/src apps/desktop/src-tauri/src | wc -l`
  → **0**. Nine hits exist repo-wide, all in `tests/sd13_*.rs` doc comments naming the matrix
  slice each test covers — which the audit excludes by design and doctrine permits.
- **`src/rules_core/rules_tables/` book directories that exist today: 14** (plus 3 loose `.rs`
  files and `mod.rs`): `ls src/rules_core/rules_tables/`. The acceptance criterion's "every
  `<book>/` directory a lane writes (all 37 in-scope books)" is forward-looking — 23 of the 37
  have no directory yet because no lane has run. Audited what exists; the audit is diff-scoped,
  so each lane's own cycle re-audits what it adds.
- **Epic-label citations in shipping source: 777.**
  `grep -rnE '\b[Ss][Dd][0-9]+-[A-Za-z0-9][A-Za-z0-9-]*\b' --include='*.rs' --include='*.ts' --include='*.tsx' src apps/desktop/src apps/desktop/src-tauri/src | wc -l`
  → **777**. This figure is load-bearing: it is why the new hyphen pattern requires a lowercase
  letter after the hyphen (see below).

### The defect this cycle actually fixed

`epic-breakdown.md` SD29-E1-F1 names **four** patterns: `sd29_*`, `SD29_*`, `Sd29*`, `sd29-*`.
`scripts/identifier-discipline-audit.sh` implemented **three**. A hyphenated bundle tag — the
form a CSS class, `data-testid`, or string key naturally takes, e.g. `"sd29-monster-row"` —
passed the gate clean. Epic 1's own acceptance criterion is "identifier-discipline audit script
returns 0 findings", and Epic 10 re-runs the same script at bundle scope, so the gap sat under
both of this bundle's identifier checkpoints.

TDD, per `AGENTS.md`: the failing case was written first and observed RED for the intended
reason (`hyphen bundle tag (sd29-)`: expected exit 1, got 0), then the regex gained an
`[Ss][Dd][0-9]+-[a-z][A-Za-z0-9-]*` branch and it went GREEN.

**Judgment call, recorded per "Stop vs. press on":** the first fix used
`[Ss][Dd][0-9]+-[A-Za-z0-9]...`, which matches all 777 epic-label citations above and would have
turned the gate into noise the moment any lane wrote `// SD29-E5-F1: ...` in shipping source.
Narrowed to require a **lowercase** letter after the hyphen — citations are `SD28-E14-F1`
(uppercase), identifiers are `sd29-monster-row` (lowercase). Two non-detection cases now pin
this. The doc slug `SD-29-...` cannot match either form: its hyphen precedes the digits.

### Files changed

- `scripts/identifier-discipline-audit.sh` — fourth pattern added, with the reasoning and the
  777 figure recorded inline so the next editor does not re-widen it.
- `scripts/tests/test_identifier_discipline_audit.sh` — **new.** 13 cases: 7 that must be caught
  (all four named patterns, the `t_<hex>` token, plus the two escapes the script's own header
  records as having happened live — a tag in top-level `src/lib.rs`, and one in the separate
  tauri crate) and 6 that must stay clean (doc slugs, epic citations, tests, ordinary code).
  Each case builds a throwaway git repo under `mktemp`; nothing touches this checkout.
- `scripts/verify.sh` — new `audit-selftest` stage in **both** `ALL_STAGES` and `QUICK_STAGES`,
  placed second (after `preflight-disk`, before any build). It fails on a non-zero self-test
  **and** on zero cases discovered — the same "a gate running zero tests asserts nothing" guard
  the `reach` stage already carries. No build, no baseline, seconds to run.
- `docs/release/SD-29-corpus-wide-catch-up-lanes/kanban.md` — card claim, then COMPLETE.

Rationale for wiring the stage rather than leaving a script: the audit script's header records
two occasions when this gate passed clean over a real planted tag. Both were caught by hand,
neither by a test. A self-test nobody runs would have been a third instance of the same pattern.

### Verification

- **`./scripts/verify.sh` (FULL, not `--quick`) → exit `0`.** Captured directly
  (`echo "VERIFY_EXIT=$?"`, never through a pipe): `VERIFY_EXIT=0`. Log:
  `/tmp/codex-verify-PgWBo6`. All **11** stages passed —
  `preflight-disk audit-selftest root-lib root-full desktop reach frontend-install frontend-test
  frontend-typecheck clippy class-dump`.
  - `audit-selftest` **13 passed, 0 failed** (the new stage, green on its first gated run).
  - `root-full` **6128 passed across 537 suites, all 521 `tests/*.rs` suites executed** — the
    `comm -23` never-ran check (Decision 40) reports zero missing suites.
  - `reach` **16 passed** — non-zero, so DoD item 2's "0 matched tests is a hard failure" is
    satisfied. This card ingests nothing and adds no record family, so it makes no new reach
    claim; the gate is cited here as unbroken, not as evidence of new coverage.
  - `desktop` 413, `frontend-test` 98/98 files, `frontend-typecheck` clean,
    `clippy` root:54 desktop:7 warnings / 0 errors, `class-dump` 31/31 computing.
- **Dual-audit (SD29-E1-F2), both green post-fix:**
  `bash scripts/identifier-discipline-audit.sh` → `OK_NO_BUNDLE_TAGS`, exit **0**;
  `bash scripts/wired-integration-audit.sh` → all four checks clean
  (`OK_NO_TOKENS`, `OK_NO_NOOP_HANDLERS`, `OK_NO_MOCK_LEAKS`, `OK_NO_WOULD_STRINGS`), exit **0**.
- **DoD item 3:** `cargo run --locked --bin v06_corpus_trap_report -- --audit` → exit **0**,
  "No defects: every ingested record's citation agrees with the line it names" (259 mod-record
  traps, 0 defects).
- **DoD item 5** (four-check wired-integration audit): clean, as above.

### DoD items recorded N/A, with reason

The Definition of done is written "**per book-ingest cycle**". This card ingests no corpus
records and adds no record family.

- **Item 4 (`v06_work_inventory` regeneration, units leave `not-started`):** N/A — no book's
  units move. Deliberately did not regenerate `docs/work-inventory.json`, which would have
  produced a diff of `generated_at` alone and put a shared artifact into a commit that has no
  claim on it. Epic 2 (`epic-2-prelaunch`) owns the corpus-wide shape pass.
- **Item 6 (`OPEN_FINDINGS`):** unchanged. The eight standing entries (`beastiary1/race_traits`
  + seven `<book>/archetypes`) are left exactly as-is per `loop-instruction.md` DoD item 6.
- **Item 7 (baseline movements):** none made. `verify.sh` reported four baselines drifting
  (`BASELINE_ROOT_LIB_TESTS` 1488→1600, `BASELINE_ROOT_FULL_TESTS` 5996→6128,
  `BASELINE_ROOT_TEST_BINARIES` 536→537, `BASELINE_CLIPPY_WARNINGS_ROOT` ceiling 75 vs 54
  measured). These are **notes, not failures**, they pre-date this cycle (SD-28's landing), and
  item 7 requires a *separate reviewable commit carrying `--show-actuals` output*. Safer default
  under unattended mode: left alone and handed forward rather than folded into an identifier
  commit. **Followup for Epic 9 or Epic 10.**
- **Item 8 (on-screen desktop verification):** N/A — this cycle surfaces no player-visible record
  family. Nothing it changed is reachable from the character sheet; the entire diff is two shell
  scripts and one new shell test. `RUN_DESKTOP_AGENT` was therefore never needed and
  `driver.sh` was not invoked. Driving the app here would have produced a screenshot proving
  nothing, which is the ceremony `decisions.md`'s twin-trap guidance exists to prevent, not an
  instance of it. **Every content lane (Epics 4-7) still owes this item in full.**

### Findings handed forward (not fixed here — out of this bundle's diff scope)

1. **Two bundle-tagged directories live in shipping source:** `apps/desktop/src/sd16/`
   (9 non-test modules under `feedback/` and `update/`, imported by `App.tsx` and
   `boundary/loadUpdateAction.ts`) and `apps/desktop/src/sd21/` (2 test files). Both pre-date
   SD-29. SD-28 established the naming precedent by adding `apps/desktop/src/releaseChecks/`.
   Not renamed here: the audit is diff-scoped by design, and `AGENTS.md` rule 3 forbids
   unrelated renames. Emitted as a `deferral` event. Note this is a *path* tag — the audit's
   regex is identifier-shaped and does not flag directory names, so no gate catches this class.
2. **`scripts/verify-baselines.env` declares three keys twice** (`BASELINE_ROOT_FULL_TESTS`,
   `BASELINE_ROOT_TEST_BINARIES`, `BASELINE_DESKTOP_TESTS`); the file is sourced, so the last
   assignment silently wins. Harmless today, a trap the moment someone edits the first
   occurrence and sees nothing change. Followup for Epic 10.

### Concurrency (recorded, per shared-checkout discipline)

Up to **four** `./scripts/verify.sh` runs were live in this same checkout during this cycle.
Two used the default `CARGO_TARGET_DIR` and two their own. No artifact cross-feed (target dirs
differ where it matters, and `verify.sh` mutates no tracked source), but the ~490-binary build
was CPU-starved for ~40 minutes and *looked* hung — `target/debug/deps/*.d` frozen at 2167 with
the log unchanged. Diagnosed live, not assumed:
`pgrep -fa "verify.sh|cargo test"` showed the sibling runs and `ps` showed rustc still burning
CPU. Emitted as an `incident` (`--recurrence-key concurrent-verify-same-checkout`).
`git status --porcelain` was re-run before every git write. Two files in the working tree at
commit time belong to other sessions and were **deliberately left uncommitted**:
`docs/retro/events/codex.jsonl` (a sibling's full-sweep event — different `log_dir`
`/tmp/codex-verify-XrQvq0`, and no `audit-selftest` stage, so provably not this cycle's) and
`docs/retro/events/sd29-preflight.jsonl` (untracked, not this actor's shard).

### Retro events emitted (`docs/retro/events/sd29-e1-identifier.jsonl`)

`correction` (the audit's missing fourth pattern, `--verified-by` the RED→GREEN self-test case),
`deferral` (the two bundle-tagged directories), `incident` (concurrent verify sweeps), plus
three auto-emitted `verification` events from `verify.sh` itself (two `--only`, one full).

### Gate for the rest of the bundle

Epic 1 is **COMPLETE**, which unblocks `epic-2-prelaunch` (Order 2) and `epic-9-version`
(Order 13). No other epic ran or was touched by this cycle.

---

## Cycle `SD29-E2-F1-001` (closing pass) — card `epic-2-prelaunch` → COMPLETE

**Actor:** `sd29-e2-prelaunch`. **Branch:** `tranche/9`. **Date:** 2026-08-11.
**Epic 2 — Operator Pre-Launch, corpus-wide (all 37 in-scope books, one pass).**

This is the **closing pass** of a cycle whose opening pass did the whole derivation and then ran
out of turn with `verify.sh` still in `root-full`. Nothing was restarted: the claim was kept, the
opening pass's untracked derivation (`corpus-shape-37-books.md`) was **re-derived rather than
trusted**, and this pass owed and paid the gate.

### Gate — DoD item 1

`./scripts/verify.sh` (FULL, not `--quick`), exit code captured directly from the process, never
through a pipe (`echo "VERIFY_EXIT=$?"` appended to the log by the same shell that ran it):

```
VERIFY_EXIT=0        RESULT: PASS
preflight-disk PASS (78% used, 107G avail)   audit-selftest PASS (13 passed, 0 failed)
root-lib       PASS (1600 passed)            root-full      PASS (6128 passed across 537 suites,
                                                            all 521 tests/*.rs suites executed)
desktop        PASS (413 passed)             reach          PASS (16 passed)
frontend-install PASS  frontend-test PASS (98/98 files)  frontend-typecheck PASS (tsc clean)
clippy         PASS (root:54 desktop:7 warnings, 0 errors)   class-dump PASS (31/31 computing)
```

Run under its own `CARGO_TARGET_DIR=/tmp/codex-target-sd29-e2-prelaunch` per the Epic 1
build-contention incident, launched early in the background so the derivation work ran alongside
it. It was CPU-shared but never hung — diagnosed live with `pgrep -c rustc` (3-4 live) and a
rising `ls /tmp/codex-target-sd29-e2-prelaunch/debug/deps/*.d | wc -l` (97 → 108 → 573), per the
"frozen timestamps mean starved, not hung" rule. **No stage failed even once**, so no
same-attribution repeat-failure incident arose.

### DoD item 2 — reach

`reach` **PASS with 16 matched tests**, not zero. A gate matching zero tests would be a hard
failure; it matched 16. This card is a derivation/pre-launch card and **ingests no records**, so
it introduces no new families of its own — item 2 is satisfied by the gate running non-vacuously
against the existing inventory, and every content lane (Epics 4-7) still owes its own family
claims in full.

### DoD item 3 — trap-report audit

`cargo run --locked --bin v06_corpus_trap_report -- --audit` → exit **0**:
`259 mod-record traps, 0 defects` — "No defects: every ingested record's citation agrees with the
line it names."

### DoD item 4 — work-inventory regeneration and idempotency

**Proven directly against git, not asserted.** The opening pass wrote the inventory at
`generated_at: 2026-08-10T23:59:04Z`; this pass regenerated it at `2026-08-11T00:18:38Z`.

```bash
git diff --stat docs/work-inventory.json
# -> docs/work-inventory.json | 2 +-   (1 insertion, 1 deletion)
grep -n generated_at docs/work-inventory.json | head -2
# -> 2:  "generated_at": "2026-08-11T00:18:38Z",
```

One changed line, and it is `generated_at`. That is the item-4 second-run property demonstrated by
the cycle itself. No book's units move (this card ingests nothing), which is the expected shape for
a pre-launch card.

### Step 1b — every figure re-derived, not transcribed

The opening pass's `corpus-shape-37-books.md` was treated as a prior cycle's output, i.e. as
something to check. Twelve spot-checks were re-run against a freshly regenerated inventory and the
live `pcgen` tree; the commands and both passes' results are tabulated in that file's new **§8**.

| spot-check | result |
|---|---|
| denominator (37 books / 38,517 units / 2,253 proven) | reproduced ✅ |
| per-kind remaining, **all ten** lane kinds | reproduced, all ten ✅ |
| `feat` `deferred-with-reason` = 2 | reproduced ✅ |
| monster-bearing books = 14, total 1,270 | reproduced ✅ |
| books with empty `reconciliation` = 24 | reproduced ✅ |
| space-in-filename pcc (`bestiary_6/_bestiary_6 _for_players.pcc`) | reproduced ✅ |
| `SOURCESHORT:B1` × 3, third in `bestiary/_pfs/` | reproduced ✅ |
| `_pfs/` subtrees = 12 | reproduced ✅ |
| B2 races 322 = 314 + 8 `.COPY=` | reproduced ✅ |
| zero-byte `.lst` in B1-B4 = 7 | reproduced ✅ |
| `PRECAMPAIGN` inside the gated `.lst` files = 0 (gate is on the pcc load line) | reproduced ✅ |
| gated support files | **sharpened — see below** |

**Zero disagreements.** No Hard-stop "a figure derived this cycle disagrees with a figure recorded
in this package" case arose.

**One sharpening (correction event emitted).** The shape note's command for the conditional
cross-book support files was `grep -rn '_ma.lst\|_oa.lst' --include='*.pcc' bestiary_4 bestiary_5`,
which returns **10 lines** for **6 distinct files** (2 `_ma` + 4 `_oa`) — the pcc load line and a
later reference both match the same file. A lane transcribing "10" as a file count would be wrong.
Fixed in both `loop-instruction.md` and `corpus-shape-37-books.md` §5/§8 with the distinct-file
form: `grep -rho '[a-z0-9_/]*_\(ma\|oa\)\.lst' --include='*.pcc' bestiary_4 bestiary_5 | sort -u`
→ **6**.

### Findings carried from the opening pass (verified, now committed)

All three stand after re-derivation and are now on the branch rather than living untracked:

1. **`equipment` remaining 1,163 → 1,144.** The old figure counted the *excluded* `beginner_box`'s
   19 units. Fixed in `kanban.md` card `epic-4-proven-equip-mod`.
2. **`feat` 1,350 → 1,348 + 2 `deferred-with-reason`.** A predicate difference, not an arithmetic
   error — stated explicitly rather than silently reconciled.
3. **`inner_sea_bestiary` / `inner_sea_world_guide` are in scope and are not stubs.**
   `loop-instruction.md` called them out-of-scope adjacents and the former a "pcc+jpg stub"; both
   halves were wrong after `decisions.md §38`'s corpus-wide re-scope. `inner_sea_bestiary` holds 7
   `.lst` + a `_pfs/` subtree, 234 units of which **40 are `monster`**, 473 trap hits.

**Dispatch-relevant finding for every downstream lane:** `reconciliation` is empty for **24 of the
37** books, because the inventory computes it only for books its *own* `scope` field labels
`in_scope` (13 books, 94 rows). That label is the generator's scope, not SD-29's. Lanes touching
those 24 must derive their own corpus-vs-engine delta and **must not read a missing
`reconciliation` as "no delta"**.

### DoD items 5-8

- **Item 5 (four-check wired-integration audit):** clean, vacuously — this cycle adds no code path,
  no handler, and no production data path. Its entire diff is documentation plus a regenerated
  generated artifact.
- **Item 6 (`OPEN_FINDINGS`):** unchanged, deliberately. The eight standing entries
  (`beastiary1/race_traits` + seven `<book>/archetypes`) are left exactly as-is per DoD item 6;
  `beastiary1/race_traits` is expected to retire in Epic 5's Monster Codex batch, and the seven
  archetype entries belong to SD-30.
- **Item 7 (baseline movements):** **none made, deliberately.** `verify.sh` again reported the four
  known drifts (`ROOT_LIB_TESTS` 1488→1600, `ROOT_FULL_TESTS` 5996→6128, `ROOT_TEST_BINARIES`
  536→537, `CLIPPY_WARNINGS_ROOT` ceiling 75 vs 54 measured). These are **notes, not failures**;
  they pre-date this cycle (SD-28's landing); and item 7 requires a *separate reviewable commit
  carrying `--show-actuals`*. Per the standing instruction, Epic 9 or Epic 10 owns that commit and
  every other card leaves them alone. **Left alone. Followup stands.**
- **Item 8 (on-screen desktop verification):** **N/A, and this is a real N/A rather than a skip.**
  The item is conditional on a record family *this cycle newly surfaced* that is player-visible.
  This cycle surfaces none — it ingests nothing and changes no player-reachable path. `driver.sh`
  was therefore not invoked and `RUN_DESKTOP_AGENT` was not needed. A screenshot here would prove
  nothing, which is the ceremony the twin-trap guidance exists to prevent, not an instance of it.
  **Every content lane (Epics 4-7) still owes item 8 in full.**

### Judgment calls taken under unattended mode (default-and-flag, no operator asked)

1. **Kept the opening pass's claim rather than re-claiming.** Resuming, not restarting; the card
   moved `IN-FLIGHT` → `COMPLETE` by this pass.
2. **Did not commit two other actors' retro shards** left dirty in the shared checkout:
   `docs/retro/events/codex.jsonl` and `docs/retro/events/sd29-e1-identifier.jsonl` (both modified
   by sibling sessions), and untracked `docs/retro/events/sd29-preflight.jsonl` (actor
   `sd29-preflight`, not this actor). Same call Epic 1 made. Only this actor's own shard is
   committed. `git status --porcelain` was re-run before every git write; no `git add -A`, no
   `git stash`.
3. **Committed the opening pass's derivation as-is plus a §8 verification appendix**, rather than
   rewriting it — the derivation reproduced exactly, so rewriting would have destroyed the audit
   trail of two independent passes agreeing.

### Retro events (`docs/retro/events/sd29-e2-prelaunch.jsonl`)

Opening pass: 3 × `correction` (the two shape-note errors + the `equipment` 1,163→1,144 kanban
figure), 1 × `deferral` (derive reconciliation for the 24 books the inventory does not cover),
1 × `incident` (ran out of turn mid-`root-full`), 2 × auto-emitted `verification`.
Closing pass: 1 × `correction` (the 10-lines-vs-6-files sharpening, `--verified-by` both commands
and both counts), plus auto-emitted `verification` events from the `--only preflight-disk` run and
the full green gate.

### Gate for the rest of the bundle

**Epic 2 is COMPLETE.** The pre-launch checklist is green and the corpus-wide 37-book shape is
derived, committed, and independently re-verified. This unblocks `epic-3-provenance` (Order 3),
which is the sole gate in front of every content lane (Epics 4-7). `epic-9-version` (Order 13) was
already unblocked by Epic 1.

---

## Cycle SD29-E1B-F1-001 — `epic-1b-naming-sweep` (Order 2.5) — COMPLETE

**Actor:** `sd29-e1b-naming`. **Branch:** `tranche/9`. **Date:** 2026-08-11.
**Card:** added to `kanban.md` by this cycle (Order 2.5, Depends-on `epic-1-identifier`), per the
operator directive it implements. **Decision:** `decisions.md` Decision 41.

**Operator directive, verbatim:** "references to SD and ge were to be replaced a few tranches ago
with function based naming. clean that up while you are at it."

### 1. Re-derived inventory (step 1b — the scouted list was NOT trusted)

| Figure | Command | Result |
|---|---|---|
| tracked path tags outside `docs/` and `tests/` | `git ls-files \| grep -Ei '(^\|/)[a-z_]*(sd\|ge)[-_]?[0-9]{2}' \| grep -vE '^(docs\|tests)/'` | 66 paths (11 rename targets after excluding the `data/corpus/.../*_range_120.json` false positive, which matches `ge_12` inside `range_120` and is not a tag) |
| tagged files under `tests/` | `git ls-files 'tests/*' \| grep -Ec '(^\|/)[a-z_]*(sd\|ge)[-_]?[0-9]{2}'` | **531** — deliberately NOT renamed (see §5) |
| tag-shaped hits per shipping source file | `grep -rnE '...' --include='*.rs' --include='*.ts' src apps scripts \| awk -F: '{print $1}' \| sort \| uniq -c \| sort -rn` | `support_state_matrix.rs` 319, `ge08_workbench.rs` 52, `loadGe08AuthoringWorkbench.ts` 29, `preview_bridge.rs` 13, then a long tail |
| of those 319, how many are real identifiers | manual read of the distinct-token histogram (`grep -roE ... \| sort \| uniq -c \| sort -rn`) | **2** (`seeded_sd13_e1_f1_current_truth`, `GE06_INPUT_CONTRACT_TEST`); the rest are `tests/...` citations — the doctrine's documented exclusion class, SD-25 1.1 |

The scouted brief listed `src/bin/sd27_gen_advanced_race_guide_cache.rs` transitively via a doc
comment; re-derivation shows **that file does not exist** — the citation in
`rules_tables/advanced_race_guide/json_cache.rs` is stale. Left as-is: this card renames files and
identifiers, it does not repair unrelated stale prose. Also found: `src/bin/gen_cache_beastiary.rs`
already follows the correct convention — the precedent to copy was already in the tree.

### 2. What landed (4 commits, each with the gate re-run)

1. `66fac552` **gate hardening, TDD.** 10 new self-test cases, RED (16 passed / 10 failed) before
   the change, GREEN after. Three escape classes, each verified live against this repo:
   (a) the GE-NN family was absent from Epic 1's regex entirely; (b) `_` is a word character, so a
   leading `\b` cannot match an infix tag — `kind_is_sd17_b3`, `build_ge08_workbench_snapshot` and
   `seeded_sd13_e1_f1_current_truth` all passed Epic 1's hardened gate clean; (c) the regex is
   identifier-shaped and scanned file *content* only, so no file/directory path tag was ever
   detectable. Added a path-tag check over `--diff-filter=AR` names, and made the SD-25 1.1
   exclusion class explicit (`tests/...` citations stripped before matching).
2. `8b6dd751` **the sweep itself** — 443 changed paths, mechanical, no behavior/signature/module
   changes.
3. `06d926e9` **import re-depth fix.** `src/sd16/{feedback,update}/` moved up one level, so every
   import that escaped the old `sd16/` segment needed one fewer `../`. Caught by
   `frontend-typecheck` (27 errors) and `frontend-test` (23 files) in this cycle's FIRST full
   `verify.sh` run, which is recorded below as a genuine RED, not smoothed over.
4. `fd02648d` **gate defect the sweep itself exposed** — see §4.

### 3. Renames

**Paths.** `apps/desktop/src/sd16/feedback/`→`feedback/`; `sd16/update/`→`update/`;
`sd21/`→`release/`; `src-tauri/src/ge08_workbench.rs`→`authoring_workbench.rs`;
`boundary/loadGe08AuthoringWorkbench.ts`→`loadAuthoringWorkbench.ts`;
`src-tauri/resources/ge08/`→`resources/authoring_workbench/`;
`tests/fixtures/ge08/`→`tests/fixtures/authoring_workbench/`;
`src/bin/sd26_gen_core_rulebook_cache.rs`→`gen_core_rulebook_cache.rs`;
`src/bin/sd27_gen_book_cache.rs`→`gen_book_cache.rs`;
`scripts/sd27-workflow.py`→`book-ingest-workflow.py`;
`scripts/sd27_apg_license_retrofit.py`→`apg_license_retrofit.py`.

**Identifiers.** The whole `Ge08*` type family loses its prefix across Rust and TS (including the
infix forms `buildGe08Diagnostics`, `mapGe08Snapshot`, `build_ge08_workbench_snapshot`, which the
first `\bGe08` pass missed); `GE08_E1_*`→`HOMEBREW_PROOF_*`;
`GE06_BASE_ARMOR_CLASS_WITHOUT_BONUS_FEAT_SLOT`→`BASE_ARMOR_CLASS_WITHOUT_BONUS_FEAT_SLOT`;
`GE06_INPUT_CONTRACT_TEST`→`INPUT_CONTRACT_TEST`;
`seeded_sd13_e1_f1_current_truth`→`seeded_current_truth`;
`mod sd27_prerequisite_tests`→`mod prerequisite_tests`;
`kind_is_sd17_b3`→`kind_is_malformed_race_or_ability`;
`diagnostic_carries_sd17_b3_slice_tag`→`diagnostic_carries_malformed_race_or_ability_kind`;
env var `SD27_INGESTED_AT`→`CODEX_INGESTED_AT`; label value
`surface:ge08-authoring-workbench`→`surface:authoring-workbench`; plus four in-file `#[cfg(test)]`
fn names. The `slice: "SD17-B-3"` **string literal** is a data value carried in shipped
diagnostics, not an identifier — it is deliberately left alone, because changing it would change
behavior and this is a rename, not a refactor.

**WIRE CONTRACT.** The Tauri command `load_ge08_authoring_workbench_snapshot` →
`load_authoring_workbench_snapshot`, changed on both sides in the same commit: the
`#[tauri::command]` fn and the `invoke_handler` registration in `src-tauri/src/main.rs`, and the
`invoke<...>('load_authoring_workbench_snapshot', ...)` call site in
`boundary/loadAuthoringWorkbench.ts`. A mismatch here fails at runtime, not compile time — which is
exactly why DoD item 8 below is load-bearing for this card and not a formality.

### 4. The gate defect this card's own diff exposed

Running the hardened gate over the finished sweep FAILED (exit 1), citing
`-                grounding_ref: GE06_INPUT_CONTRACT_TEST` — a **deletion**. The script scanned the
whole unified diff, so every tag this card removed reappeared as a violation on its own `-` lines.
Its own header had claimed for months that it "flags bundle identifiers newly introduced by the
cycle"; the implementation never restricted itself to added lines. A gate that punishes the fix is
worse than no gate. Fixed in `fd02648d` with two new self-test cases (28 total, RED first).

Post-fix: `BASE_BRANCH=origin/develop bash scripts/identifier-discipline-audit.sh` → `OK_NO_BUNDLE_TAGS`, **exit 0**.

### 5. Judgment calls taken as safe defaults (unattended mode — recorded, not asked)

1. **`tests/*.rs` file names keep their tags.** 531 tracked files, and they are the citation targets
   of the doctrine's documented exclusion class; renaming them obliges rewriting cited prose across
   the whole shipping tree. Scoped out by the brief; recorded as a `deferral` retro event, not a
   silent omission.
2. **Closed-bundle historical receipts under `docs/release/SD-27-.../` keep their references to
   `scripts/sd27-workflow.py`.** They record what was run at the time. Live source citations of
   every renamed file WERE updated (25 `sd27_gen_book_cache` + 9 `sd26_gen_core_rulebook_cache`
   citations across `src/`, plus `docs/architecture/`).
3. **`scripts/verify-baselines.env` was touched, but only in prose.** Two doc-comment lines cite
   `src/bin/sd27_gen_book_cache.rs` by name; they follow the rename. `git diff origin/develop...HEAD
   --stat -- scripts/verify-baselines.env` → `2 insertions, 2 deletions`, **no baseline value
   changed**. The four drifted baselines named in the standing note (ROOT_LIB_TESTS 1488→1600,
   ROOT_FULL_TESTS 5996→6128, ROOT_TEST_BINARIES 536→537, CLIPPY_WARNINGS_ROOT ceiling 75 vs 54
   measured) are LEFT STANDING for Epic 9/10 per DoD item 7. This run reproduced all four exactly.
4. **`data/corpus/.../intelligent_item_sense_range_120.json` is not renamed.** It matched the
   scouting regex only because `range_120` contains `ge_12`. Not a tag.

### 6. Incident: the sweep nearly disarmed its own gate

A repo-wide `sed` over the tracked file list rewrote `scripts/identifier-discipline-audit.sh` AND
`scripts/tests/test_identifier_discipline_audit.sh`, silently converting every detection case's
planted tag into an already-clean string. The self-test would have kept printing `26 passed, 0
failed` while testing nothing. Caught by reading `git diff` of `scripts/` before re-running the
suite; both files restored from HEAD and excluded by an explicit `grep -v` from every later `sed`
pass. Emitted as an `incident` with recurrence key `rename-sweep-rewrites-its-own-gate`.

### 7. Definition of done

1. **`./scripts/verify.sh` (FULL) exit code captured directly** — written to a file by a wrapper
   script, never through a pipe. **First run: exit 1** (`frontend-test`, `frontend-typecheck` — the
   import-depth breakage, real, fixed in `06d926e9`). **Second run: exit 0**, all 11 stages PASS:
   `preflight-disk`, `audit-selftest` (26→28 cases), `root-lib` (1600 passed), `root-full` (6128
   passed across 537 suites, all 521 `tests/*.rs` suites executed), `desktop` (413 passed), `reach`
   (16 passed), `frontend-install`, `frontend-test` (98/98 files), `frontend-typecheck` (clean),
   `clippy` (root:54 desktop:7 warnings, 0 errors), `class-dump` (31/31 computing). Third run after
   the `fd02648d` gate fix recorded below.
2. **Reach claims:** `reach` stage reports **16 matched claims**, not zero. This card ingests no
   content and adds no record family, so it introduces no new claim and removes none — the 16 are
   the same 16 Epic 2 left standing, still passing after every rename.
3. `cargo run --locked --bin v06_corpus_trap_report -- --audit` → **exit 0** ("No defects: every
   ingested record's citation agrees with the line it names").
4. **`docs/work-inventory.json` untouched** — no content ingested, no units moved. This card changes
   no corpus data; the inventory's inputs are unchanged by construction.
5. **Wired-integration four-check audit: clean.** No new code paths, no stubs, no fixture-only data:
   every change is a rename of an existing wired path, and the wire contract's live behavior is
   proven on screen in item 8.
6. **`OPEN_FINDINGS` unchanged.** No family became unsurfaceable; nothing was added or retired.
7. **No baseline movement.** See §5.3.
8. **On-screen verification — MANDATORY for this card, and it earned its place.**
   `RUN_DESKTOP_AGENT=sd29-e1b-naming-cycle` (unique to this cycle), driven via
   `apps/desktop/.claude/skills/run-desktop/driver.sh`: `launch` → `title` (`WM_NAME(STRING) =
   "Codex"`) → gear → **Developer** tab. The captured screen reads
   **"Connected to the app backend"**, `FEATURE CHECK: Character-preview authoring check —
   valid / success`, `DATA SOURCE: Live backend data`, `BACKEND: v0.8.0 · 8b6dd7511f2f`. That panel
   is rendered from the snapshot returned by the renamed Tauri command
   `load_authoring_workbench_snapshot`; had either side of the rename been missed, it would read
   "Failed to load diagnostics" or fall back — and every test in the repo would still have been
   green. Screenshot: `e1b-developer.png`.
   **Driver note for successors:** `driver.sh screenshot` (`import -window <id>`) returned an
   all-black 417-byte PNG on every attempt in this container across three runs, including at 55 s
   of settle; `import -window root` on the same display at the same moment captured the painted UI
   correctly. Use the root capture when the windowed one comes back black — it is not the app
   failing to paint.

### 8. Convention for every successor lane (the reason this card lands before Epic 3)

**Any new codegen binary is named for its function.** A lane adding a book-cache generator writes
`src/bin/gen_book_cache.rs`-shaped names — NOT `sd29_gen_*.rs`. The `sd27` precedent this card
deleted is not available to copy, and `src/bin/gen_cache_beastiary.rs` shows the correct form.
The same rule binds modules, structs, consts, test module names, env vars, label/string keys, and
directories. Both tag families are banned: SD-NN and GE-NN.
`scripts/identifier-discipline-audit.sh` now catches path tags and PascalCase/infix forms, not just
prefix forms, and no longer fires on tags a diff *removes*. The documented exclusion class stands: a
doc comment or string literal citing a real `tests/...` file by name is not a violation.

### Retro events (`docs/retro/events/sd29-e1b-naming.jsonl`)

2 × `correction` (Epic 1's gate had three escape classes; the gate flagged its own cure),
1 × `incident` (`rename-sweep-rewrites-its-own-gate`), 1 × `deferral` (531 `tests/` file names),
plus auto-emitted `verification` events from every `verify.sh` run — including the RED one.

### Git discipline

`git status --porcelain` run before every git write. No `git add -A`, no `git stash`. Three other
actors' retro shards (`codex.jsonl`, `sd29-e1-identifier.jsonl`, untracked `sd29-preflight.jsonl`)
were left dirty and uncommitted — same call Epic 1 and Epic 2 made. Only this actor's own shard is
committed.

---

## Cycle SD29-E3-F1-001 — `epic-3-provenance` (Provenance Gate: PI-screening wired into Pipeline B)

**Actor:** `sd29-e3-provenance` · **Branch:** `tranche/9` · **Branch tip at claim:** `a0e1733e`
**Card:** `epic-3-provenance` (kanban Order 3) · **PR-id:** none (direct commit to `tranche/9`, pre-authorized)
**Cycle-type:** provenance gate — gates Epics 4-7 corpus-wide

### 1. What this card had to produce, and what it actually produced

`decisions.md §37.3` / `epic-breakdown.md` SD29-E3-F1 / `acceptance-and-verification.md` AT-29-003a
require the 55-term Product-Identity blacklist to run against a lane's own newly-generated content
**before it lands in `rules_tables/`**, with a hit treated as a hard stop. Before this cycle,
`docs/governance/license-matrix.md`'s central finding stood: **zero** files under `rules_tables/`
called `pi_screening`, `PI_BLACKLIST_TERMS`, or `classify_field`.

Landed — real wiring in the production path, no fixture, no stub:

- **`src/rules_core/pi_table_sweep.rs`** — the screen. Uses the shared
  `pi_screening::PI_BLACKLIST_TERMS` (it does **not** fork the list; forking is the exact failure
  `pi_screening.rs`'s own header documents). Two entry points:
  `screen_generated_table(file, generated) -> Vec<PiSweepHit>` is the lane-facing call an
  extraction/table-generation step makes on the text it is about to write — a non-empty return is
  the hard stop; `sweep_dir` + `reconcile` are the standing tree-wide check.
- **`src/bin/pi_sweep_rules_tables.rs`** — the CLI a lane runs and pastes into its receipt.
  Exit `0` clean, `1` unbaselined hit **or stale baseline row**, `2` I/O or parse failure.
- **`docs/governance/pi-sweep-baseline.tsv`** — the 10 pre-existing hits with an explicit
  `real-leak` / `false-positive` disposition per row. Any other disposition string is a parse
  **error**, not a silent suppression.
- **`scripts/verify.sh`** — new `pi-sweep` stage, in **both** `ALL_STAGES` and `QUICK_STAGES`
  (a lane must not be able to land a leak on a fast loop and find it only on a full sweep). The
  stage fails on a non-zero exit **and** on an exit-0 run that did not print `CLEAN` — the same
  0-matched guard `reach` and `audit-selftest` each carry.
- **`tests/pi_table_sweep.rs`** — 6 integration tests incl. the live-tree gate; 4 further unit
  tests in the module.

### 2. Re-derived figures (command first, value second — nothing transcribed)

- **Hits in Pipeline B:** `cargo run --locked --bin pi_sweep_rules_tables`
  → `pi-sweep: 10 hits over src/rules_core/rules_tables, 10 baseline rows` / `CLEAN`.
  Independently reproduced before writing any Rust, with a throwaway extractor over
  `pi_screening.rs`'s literal list against `glob('src/rules_core/rules_tables/**/*.rs')`:
  **137 files, 55 terms, 10 hits** — same 10.
- **Term count:** 55, not 54 or 56. A first extraction pass read **56** because the list's own
  trailing comment quotes `"Jarn"` a second time; stripping `//` lines gives 55 (20 deities + 34
  places + Jarn). Recorded because the surrounding docs quote 54 and 55 in different places.
- **Negative control (the gate is not vacuous):** a scratch tree
  (`--repo-root <scratch>` holding one faux table reading `"blessed by Iomedae in Absalom"`)
  → `2 unbaselined hit(s), 10 stale row(s)`, **exit 1**. The gate fails when it should.
- **Disk preflight:** `./scripts/verify.sh` stage `preflight-disk` → PASS, 79% used, 105G available.

### 3. The sweep's output, recorded (AT-29-003a's evidence requirement)

10 hits, all accounted for in the baseline: **3 real leaks** —
`acg/archetype_tables.rs` `Sarenrae`, `acg/spell_list.rs` `Jarn`,
`advanced_race_guide/archetype_tables.rs` `Asmodeus` — and **7 false positives**:
`ultimate_magic/equipment_tables.rs` `Geb` inside "Gebr", plus **six** `Nex` matches inside the
spell name **"Discern Next of Kin"** (`acg/shaman_spell_list.rs`, `acg/spell_list.rs`,
`apg/witch_spell_list.rs`, `crb/{bard,sorcerer,wizard}_spell_list.rs`).

**Correction against this package's cited authority.** `license-matrix.md` (and, through it,
`decisions.md §37.3` and `epic-breakdown.md` Epic 3) reports its manual sweep as **3 real + 1 false
positive = 4 hits**. The real count is **10**; the three real leaks are exactly right, the
false-positive class was undercounted 1 → 7. Corrected in place via an addendum to
`license-matrix.md` (the "Not fixed here" section is left standing as authored) and emitted as a
`correction` retro event with the command as `--verified-by`. No new *real* leak was found.

**The three real leaks are NOT fixed here — deferred, not overlooked.** They live in ACG's and
ARG's tables, owned by the bundles that landed them; `decisions.md §37.3` explicitly puts them
outside SD-29's write scope. Each is a `real-leak` baseline row naming its owner, and a `deferral`
retro event records the revisit condition. This is the safer default taken under UNATTENDED MODE:
redacting another bundle's committed records unasked would have been the scope expansion.

### 4. OGL / attribution — cited, not re-derived (SD29-E3-F2)

All 37 in-scope books' OGL/attribution status is `docs/governance/license-matrix.md`'s per-book
table: real `OGL.txt`, active `.pcc` `COPYRIGHT:` block, `ISOGL:YES`, publisher, section-15
attribution. Two rows a lane must read rather than re-derive: **`ultimate_combat`** declares
`#EXTRAFILE:OGL.txt` for a file that does not exist — attribution is recoverable from the `.pcc`
`COPYRIGHT` block only; **`core_essentials`** is the one *unestablished* row, its own declarations
commented out, practically recoverable only through `core_rulebook.pcc`'s unconditional inclusion.
No row was found stale this cycle. `beginner_box` is out of scope and needs no citation.

### 5. Definition of done

1. **`./scripts/verify.sh` (FULL) — exit code captured directly, never through a pipe: 0.**
   Written by `echo $? > verify-e3-run2.exit` on the statement immediately after the command. (Run 1
   of the same gate on the same tree also reported `RESULT: PASS`, 12 of 12 stages, but its status
   was never written to a file, so this receipt cites run 2.)
   Stage results: all **12** PASS — `preflight-disk`, **`pi-sweep` (10 hits / 10 baseline rows, CLEAN — the new stage)**, `audit-selftest` (28 cases), `root-lib` (1604 passed), `root-full` (6138 passed across 539 suites, all 522 `tests/*.rs` suites executed), `desktop` (413), `reach` (16), `frontend-install`, `frontend-test` (98/98 files), `frontend-typecheck`, `clippy` (root:54 desktop:7 warnings, 0 errors), `class-dump` (31/31 computing).
2. **Reach claims:** the `reach` stage reports **16 matched claims**, not zero. This card ingests no content and adds no record family, so
   it introduces no new claim and retires none.
3. `cargo run --locked --bin v06_corpus_trap_report -- --audit` → **exit 0**: "No defects: every ingested record's citation agrees with the line it names" (259 trap rows, 0 defects, `mod-record`).
4. **`docs/work-inventory.json` regenerated twice and left unchanged.** `cargo run --locked --bin v06_work_inventory` ×2, both exit 0; a `json.load` diff with `generated_at` popped is **byte-identical** to the committed file, so the second run changed only `generated_at` and the `generated_at`-only churn was reverted (`git checkout -- docs/work-inventory.json`). No units moved out of `not-started`, correctly — this card ingests nothing. The
   standing hazard (regenerating `data/corpus` destroys `license`/`pi_field`/`raw_tokens`) is not
   touched: **no generator was run and no file under `data/corpus/` was written this cycle.**
5. **Wired-integration four-check audit over this cycle's files: clean** —
   `OK_NO_TOKENS`, `OK_NO_NOOP_HANDLERS`, `OK_NO_MOCK_LEAKS`, `OK_NO_WOULD_STRINGS`.
   `scripts/wired-integration-audit.sh` (which audits the whole `develop...HEAD` bundle diff, not
   this cycle's) reports one Check-1 hit —
   `placeholder="e.g. GE08 authoring workbench"` → `placeholder="e.g. authoring workbench"` — an
   HTML `placeholder` **attribute** in `epic-1b-naming-sweep`'s own diff, not a stub token and not
   from this card. Recorded, not folded in.
6. **`OPEN_FINDINGS` unchanged.** No family became unsurfaceable.
7. **No baseline movement committed.** `scripts/verify-baselines.env` deliberately untouched per the
   standing Epic-1 followup: the four SD-28 drifts plus this cycle's own additions
   (`root-lib` measured **1604**, +4 from `pi_table_sweep`'s unit tests; `root-full` moves by this
   cycle's 6 new integration tests) are notes, not failures. Epic 9/10 owns the separate reviewable
   `--show-actuals` commit.
8. **On-screen desktop verification: N/A, and not a skip.** DoD item 8 binds "any record family
   whose reach claim is player-visible." This card surfaces **no record family** — it adds a
   build/CI-time provenance gate whose only outputs are a CLI report and a verify.sh stage. There is
   no sheet value, no catalog row, and no compute twin to read; a screenshot would photograph an
   unrelated screen and assert nothing. `RUN_DESKTOP_AGENT` was therefore not consumed, and the
   `desktop` stage's own result is carried in item 1.

### 6. Git discipline

`git status --porcelain` run before every git write. No `git add -A` (explicit paths only), no
`git stash`. Other actors' retro shards (`codex.jsonl`, `sd29-e1-identifier.jsonl`, untracked
`sd29-preflight.jsonl`) left dirty and uncommitted — same call Epics 1, 1b and 2 made. Only this
actor's own shard is committed. `CARGO_TARGET_DIR=/home/ubuntu/workspace/.codex-targets/sd29-e3-provenance`
(own dir, never under `/tmp`), removed at cycle end; `scripts/reclaim.sh --apply` run after.

### 7. What every successor lane (Epics 4-7) must now do

Two lines, non-optional, per book cycle-batch:

1. Call `pi_table_sweep::screen_generated_table(<target path>, <generated text>)` in the extraction
   step **before** writing the table. Non-empty ⇒ hard stop for that record; do not write, do not
   route around, record the hits in the receipt.
2. Run `cargo run --locked --bin pi_sweep_rules_tables` after the write and paste its output into
   the cycle receipt, alongside the `license-matrix.md` row for the book touched.

If a hit is a genuine false positive (the `Nex`/"Next" class), add a `false-positive` baseline row
with the collision explained — never widen the gate and never edit the blacklist to make it quiet.

### Retro events (`docs/retro/events/sd29-e3-provenance.jsonl`)

1 × `correction` (license-matrix.md's own sweep: 4 hits claimed → 10 actual), 1 × `deferral` (the
three pre-existing real leaks, owned by other bundles), plus `verify.sh`'s auto-emitted
`verification` event.

## Cycle SD29-E4-F1-001 — `epic-4-proven-equip-mod` (Proven-Path Lane: equipment + equipment_modifier)

**Actor:** `sd29-e4-equip` · **Branch:** `tranche/9` · **Branch tip at claim:** `579d5941`
**Card:** `epic-4-proven-equip-mod` (kanban Order 4) · **PR-id:** none (direct commit to `tranche/9`, pre-authorized)
**Cycle-type:** proven-path content lane, corpus-wide

### 1. Re-derived figures (command first, value second — nothing transcribed)

The brief said the package records **1,163 + 812** and told me to verify, not transcribe. Re-derived:

```bash
python3 -c "
import json
from collections import Counter
U=json.load(open('docs/work-inventory.json'))['units']
inc=[u for u in U if u['book']!='beginner_box']
for k in ('equipment','equipment_modifier'):
    ks=[u for u in inc if u['kind']==k]
    print(k, len(ks), Counter(u['status'] for u in ks))"
# equipment          6208  {ingested-magnitude 4638, not-started 959, text-complete 293, not-ingested 185, grounded 133}
# equipment_modifier 1580  {not-ingested 584, ingested-magnitude 456, text-complete 272, not-started 228, grounded 40}
```

- **equipment remaining = 959 + 185 = 1,144** — `kanban.md`'s corrected figure, not `scope-draft.md`'s 1,163.
- **equipment_modifier remaining = 228 + 584 = 812** — matches.
- **The predicate behind "remaining" is stated nowhere in this package and is load-bearing:** it is
  `status in {not-started, not-ingested}`. Recorded here so the next lane does not have to re-discover it.

**The 1,956 splits into two populations that need completely different work, and only one of them is
this card's "proven path":**

| population | units | what it needs |
|---|---:|---|
| `not-ingested` — book already has a compiled `RuleSetId`, its equipment table just lacks the record | **769** | table rows. No new mechanism. |
| `not-started` — book has no compiled rule set at all (`inner_sea_gods`, `occult_adventures`, `mythic_adventures`, …, 13 books) | **1,187** | a new `RuleSetId` variant per book, a compiled rule-set module, a corpus cache. Not a table row. |

```bash
python3 -c "
import json
from collections import Counter
U=json.load(open('docs/work-inventory.json'))['units']
rem=[u for u in U if u['book']!='beginner_box' and u['kind'] in ('equipment','equipment_modifier')
     and u['status'] in ('not-started','not-ingested')]
print(Counter(u['status'] for u in rem))"
# Counter({'not-started': 1187, 'not-ingested': 769})
```

**This cycle closed all 769 of the `not-ingested` population and none of the 1,187 `not-started`
one.** Read `src/bin/v06_work_inventory.rs`'s `classify()`: a book with no compiled rule set returns
`not-started` *before* the equipment table is ever consulted, so adding catalog rows for those 13
books would change no unit's status and would ship a row the engine has no rule set behind. That is
the boundary of "no new mechanism is needed," and it is 769 units wide, not 1,956.

### 2. What landed

- **`src/bin/gen_equipment_gap_tables.rs`** — the codegen. Re-parses the 19 `.lst` files those 769
  units come from (paths taken from the inventory's own `source_file` field, not a directory glob)
  under `v06_work_inventory::enumerate_file`'s **exact** record predicate, and emits only the records
  the hand-authored tables do not hold. PI-screens the generated text through
  `pi_table_sweep::screen_generated_table` **before** writing — Epic 3's mandated line 1.
- **`src/rules_core/rules_tables/equipment_gap_tables.rs`** — generated, 769 rows.
- **`src/rules_core/equipment_resolver.rs`** — `equipment_catalog_rows()` is now
  `hand_authored_equipment_rows()` (the eleven per-book tables, split out as its own public function
  so the generator's filter is provably their complement rather than a hand-maintained exclusion
  list) **chained with the gap rows, last**, so first-match key lookup is unchanged. New
  `EQUIPMENT_BOOK_UW`.
- **`apps/desktop/src-tauri/src/equipment_catalog.rs`** — one `map_gap_entry` mapper into
  `build_equipment_catalog()`, so the rows reach the catalog screen, the sheet's Add Equipment picker
  and `list_equipment`. Nine pinned count tests re-derived (see §4).
- **`apps/desktop/src-tauri/src/reach_gate.rs`** — `equipment_reach` unions the book's gap keys into
  the **claim**, not merely into the surface; new `("ultimate_wilderness", "equipment")` arm.
- **`src/bin/v06_work_inventory.rs`** — `equipment_book_slug_for` learns `"UW"`.
- **`tests/equipment_gap_tables.rs`** — 7 integration tests.

### 3. Honest reporting of "proven" (the brief's explicit ask)

`work-inventory`'s `proven` predicate is `{grounded, text-complete}` and **equipment has no probe** —
`probe_equipment_effect_wiring` observes a computed delta for only a handful of keys, so almost no
equipment record can ever reach `grounded`. The honest split for what this cycle moved:

- **engine-holds (`ingested-magnitude` or `text-complete`): all 769.** The engine holds the record
  with its real `COST:`/`WT:` fields, and the desktop catalog serves it.
- **strictly-proven (`grounded`): 0 of the 769**, and that is a property of the instrument, not of
  the work — no equipment probe exists for these keys.

### 4. Per-book, and the counts that had to move

Generator output, re-derived by `tests/equipment_gap_tables.rs` against the table rather than copied
from stdout — each figure equals that book's own `not-ingested` unit count:

| book | gap rows | source |
|---|---:|---|
| `core_rulebook` | 335 | `cr_equipmods.lst` 332 + `core_essentials` 3 |
| `ultimate_wilderness` | 127 | 78 general + 45 magic + 1 arms/armor + 3 equipmods |
| `ultimate_psionics` | 113 | `up_equipmods.lst` |
| `ultimate_equipment` | 65 | 39 magic + 12 arms/armor + 4 general + 10 equipmods |
| `advanced_class_guide` | 50 | 48 equipmods + 2 `_pfs/pfs_acg_equip.lst` |
| `advanced_players_guide` | 37 | `apg_equipmods.lst` |
| `ultimate_combat` | 20 | `uc_equipmods.lst` |
| `advanced_race_guide` | 15 | 14 equipmods + 1 arms/armor |
| `ultimate_intrigue` | 7 | `ui_equipmods.lst` |
| **total** | **769** | |

**`core_essentials`' three rows are filed under CRB deliberately.** Their inventory evidence is
`shared_library_record_held_by_no_ingested_host`; `core_rulebook.pcc` includes that shared library
unconditionally, so CRB is the observed host, and putting the keys in CRB's set is what lets
`classify()` attribute them rather than leave them unattributed.

**`ultimate_wilderness` had ZERO equipment rows before this cycle** despite being a compiled rule set
whose feats and archetypes already reach the player. All 127 of its catalog rows are gap rows; it
needed a new `EQUIPMENT_BOOK_UW` code, which `equipment_catalog_books()` picks up automatically
(it derives from the resolver) but which `equipment_book_slug_for` panics on until told — by design.

**Nine pinned desktop counts moved, each re-derived from the live catalog, none guessed.** Catalog
total **6,146 → 6,915** (+769, exactly the lane). Descriptions **3,755 → 3,844**. `ArmsArmor` filter
**921 → 937**. CRB `Equipmods` **658 → 990**. ACG `Equipmods` **48 → 96**. APG gains a real
`Equipmods` category (0 → 37) — the old test comment asserted "no `apg_equipmods.lst` in the corpus",
which is **false**: the file exists and carries 37 records.

**Cross-book key collisions 136 → 203, and this is the correct answer, not a defect.** A record
Ultimate Equipment reprints out of the Core Rulebook is a record in *both* books; this lane's
predicate is "a record this book's own table does not hold". The collision test was rewritten to
keep the original UC/UE review intact — it now asserts the pinned 136-key set against the
**hand-authored rows alone** (gap `(book, key)` pairs removed) and separately requires every *new*
collision to involve a gap row. A new collision between two hand tables still fails.

### 5. Definition of done

**A real shipped-behaviour regression this cycle found and fixed, not routed around.**
`equipment_cost_gp_headless_resolve`'s precedence is **stage-major, not row-major**: stage 1 matches
any CRB row by `KEY:` before stage 2 matches any CRB row by display name. Chaining the gap rows last
is therefore *not* sufficient on its own. CRB's hand table holds a row whose display **name** is
`Cold Iron` (0 gp); `cr_equipmods.lst` holds a distinct record whose **`KEY:`** is `Cold Iron` and
which carries no `COST:` token — so the new row won stage 1 and repriced a shipped CRB identity
`Some(0.0)` → `None`. Caught by `widening_leaves_every_crb_identity_resolving_to_its_original_cost`
(exhaustive over all 2,977 CRB rows × 3 probes — the reason it is exhaustive rather than sampled).
Fixed by running the five-stage match over `hand_authored_equipment_rows()` to exhaustion first and
only then over the full catalog, with the helper returning the matched **row** so "matched a row
whose price is honestly `None`" stays distinguishable from "matched nothing".

**A second, pre-existing ambiguity class grew from 1 to 36, pinned by name.**
`the_two_lookups_agree_on_every_catalog_key_but_the_one_pinned_collision` guards keys where
`equipment_catalog_row_by_key` and the free-form resolver disagree. Every one of the 35 additions is
the `Cold Iron` shape — a gap row whose `KEY:` equals some hand row's display *name*. Both records
are real and both belong in the catalog; only the free-form *string* is ambiguous, and the remedy
that test's own doc comment already prescribes (a caller holding a picker key prices via
`equipment_catalog_row_by_key`, never the free-form resolver) already covers it. Pinned by name, not
by count, so a 37th still fails.

### 5. Work-inventory movement (DoD item 4)

`cargo run --locked --bin v06_work_inventory` → exit 0, `docs/work-inventory.json` regenerated:

```bash
python3 -c "
import json
from collections import Counter
U=json.load(open('docs/work-inventory.json'))['units']
inc=[u for u in U if u['book']!='beginner_box']
for k in ('equipment','equipment_modifier'):
    print(k, Counter(u['status'] for u in inc if u['kind']==k))"
# equipment          {ingested-magnitude 4817, not-started 959, text-complete 299, grounded 133}
# equipment_modifier {text-complete 841, ingested-magnitude 471, not-started 228, grounded 40}
```

**`not-ingested` is now 0 for both kinds** (was 185 + 584 = 769). `ingested-magnitude` +179 for
equipment and +15 for equipment_modifier; `text-complete` +6 and +569 — the equipmod files are
mostly magnitude-free `SPROP:`-only records, which is why the bulk landed `text-complete` rather
than `ingested-magnitude`, and that is the honest classification, not a shortfall. The 959 + 228
`not-started` units are untouched and belong to the 13 uncompiled books (see §1).

**The standing hazard was not touched:** no generator wrote anything under `data/corpus/`, so no
`license`/`pi_field`/`raw_tokens` were destroyed.

### 6. Definition of done — status at receipt time

1. **`./scripts/verify.sh` (FULL), exit code captured directly via `echo $? > verify.exit`, never
   through a pipe.** Two runs this cycle. **Run 1 was RED and correctly so** — `root-lib` failed on
   the `Cold Iron` repricing regression described above; that is a gate catching a real defect, and
   it was fixed at source, not routed around. **Run 2, on the fixed tree**, is the one this receipt
   cites; stage results are recorded in §7.
2. **Reach claims — this card's families are `<book>/equipment` for nine books.** `equipment_reach`
   now unions each book's 769 gap keys into the **claim** itself, so the gate asserts on the new
   rows rather than merely tolerating them, and `("ultimate_wilderness", "equipment")` is a new arm.
   No family is claimed by absence.
3. `cargo run --locked --bin v06_corpus_trap_report -- --audit` — see §7.
4. **`docs/work-inventory.json` regenerated, units moved: `not-ingested` 769 → 0** (§5).
5. **Wired-integration four-check audit over this cycle's files: clean.** No TODO/FIXME tokens, no
   no-op handlers, no mock leaks, no "would have" strings. The rows are served by the same
   `build_equipment_catalog()` the catalog screen, the Add Equipment picker and `list_equipment`
   already read — a real path, not a parallel one.
6. **`OPEN_FINDINGS` unchanged.** No family became unsurfaceable; UW's equipment went from
   unsurfaced-and-unclaimed to surfaced-and-claimed, which retires nothing and adds nothing.
7. **No baseline movement committed.** `scripts/verify-baselines.env` deliberately untouched per the
   standing Epic-1 followup; `root-lib` measures **1604** against a 1488 baseline and the other three
   SD-28 drifts still stand. Epic 9/10 owns that separate `--show-actuals` commit.
8. **On-screen desktop verification: NOT PERFORMED — recorded as a shortfall in §10, not
   claimed and not substituted.** This is the one DoD item this cycle does not satisfy.

### 7. Verification results

**`./scripts/verify.sh` (FULL) — RESULT: PASS, exit code `0`.** Captured directly with
`echo $? > verify.exit` on the statement immediately after the command, never through a pipe.
All **12** stages PASS:

| stage | result |
|---|---|
| `preflight-disk` | PASS (disk budget OK) |
| `pi-sweep` | PASS — 10 hits over `src/rules_core/rules_tables`, 10 baseline rows, **CLEAN**. The 769 new rows added **zero** PI hits; the generator's own pre-write screen also reported `CLEAN (0 hits)`. |
| `audit-selftest` | PASS (28 cases) |
| `root-lib` | PASS (**1604** passed) |
| `root-full` | PASS (**6145** passed across **541** suites; all **523** `tests/*.rs` suites executed) |
| `desktop` | PASS (413 passed) |
| `reach` | PASS (**16** matched claims — not zero) |
| `frontend-install` / `frontend-test` / `frontend-typecheck` | PASS (98/98 files, `tsc` clean) |
| `clippy` | PASS (root:54 desktop:7 warnings, 0 errors) |
| `class-dump` | PASS (31/31 computing) |

**Three runs, and the two red ones were the gate doing its job.** Run 1: `root-lib` red on the
`Cold Iron` repricing regression (§4). Run 2: `root-full` red on `tests/no_foreign_home_paths.rs`
(the generator baked `/home/ubuntu/...` in as `PCGEN_CORPUS_ROOT`'s default and carried a tilde
expander — one machine's truth shipped as everyone's; the variable is now required, matching
`pathfinder_unchained::monk_features`' existing convention) **and** `desktop` red on two
`character_hub` picker pins. Run 3, on the fixed tree: green. `preflight-disk` was also red on run 2
at 90% used / 20G free — two concurrent agents each holding a 27 GB `CARGO_TARGET_DIR` — and passed
on run 3 at 84%. No stage was weakened, skipped or `#[ignore]`d to reach green.

**The desktop failure was the most valuable one, and its fix is not a number.** The Attach Modifier
picker now offers **1,082 → 1,666** rows, +584 corpus gap-lane Equipmods. Both tests' *load-bearing*
assertions hold unchanged: **zero** offered rows are refused by the attach gate, and no new
display-vs-charge divergence appears (still exactly the three pinned same-book same-key rows). So
none of the 584 newly offered rows is a dead affordance — the exact defect
`docs/governance/no-stub-mvp-doctrine.md` names and that this picker has shipped before. 13 of the
584 carry a real non-zero `COST:` (`priced_non_crb` 116 → 129); the rest are honestly `None` or
`Some(0.0)`, which is the ordinary shape of an equipment *modifier*, not a gap.

**DoD item 3 — `cargo run --locked --bin v06_corpus_trap_report -- --audit` → exit `0`:**
"No defects: every ingested record's citation agrees with the line it names" (259 trap rows,
0 defects, `mod-record`).

**DoD item 4 — idempotence proven, not asserted:** `v06_work_inventory` run twice (both exit 0); a
`json.load` comparison with `generated_at` popped from both reports **True** (identical), and the
`generated_at`-only churn was reverted with `git checkout -- docs/work-inventory.json`.

### 8. Git discipline

`git status --porcelain` run before every git write. No `git add -A` — explicit paths only. No
`git stash`. Two commits, both pushed to `tranche/9`:
`2a35b60f` (the lane) and `919703e3` (the run-2 fixes). Another actor's worktree
(`wf_3516060a-756-7`, the `epic-4-proven-spell` card) was live on the same box throughout; nothing
of theirs was touched, and `origin/tranche/9` was re-fetched before each push.
`CARGO_TARGET_DIR=/home/ubuntu/workspace/codex-target-sd29-e4-equip` — this agent's own directory,
never under `/tmp` — removed at cycle end, `scripts/reclaim.sh --apply` run after.

### 9. What the next lane inherits

1. **The `not-started` half is not this lane's shape.** 1,187 equipment/equipment_modifier units sit
   in 13 books with no compiled `RuleSetId`. Each needs a rule-set variant and a compiled module
   before a single catalog row can move a unit. Sizing that as "proven path" would be wrong.
2. **`gen_equipment_gap_tables` is re-runnable and self-checking.** Adding a book to `BOOK_INPUTS`
   is the whole change; the already-held filter derives from `hand_authored_equipment_rows()`, so it
   cannot drift.
3. **Appending rows to a staged matcher is not order-safe** (§4). Any lane widening a resolver whose
   precedence is stage-major must run the old row set to exhaustion first.

### 10. DoD item 8 — on-screen desktop verification: **NOT PERFORMED. Recorded as a shortfall, not claimed.**

The equipment catalog is player-visible, so item 8 binds here — this is not an N/A the way Epic 3's
build-time gate was. It could not be performed:

```bash
cd apps/desktop
RUN_DESKTOP_AGENT=sd29-e4-equip-cycle1 ./.claude/skills/run-desktop/driver.sh launch
# -> "Timed out waiting for launch"
RUN_DESKTOP_AGENT=sd29-e4-equip-cycle1 ./.claude/skills/run-desktop/driver.sh title
# -> "No running app — run 'driver.sh launch' first"
pgrep -af 'codex-desktop$'   # -> nothing
```

`RUN_DESKTOP_AGENT` was set to a value unique to this cycle, per the skill's concurrency rule (it
resolved to `DISPLAY=:78`, disjoint from the sibling agent's). The build succeeds and the log reads
`Finished dev profile ... Running <target>/debug/codex-desktop`; the binary then exits before any
window appears. The only preceding message is
`libEGL warning: DRI3 error: Could not get DRI3 device`. Reproduced **three** times, once with
`WEBKIT_DISABLE_COMPOSITING_MODE=1 WEBKIT_DISABLE_DMABUF_RENDERER=1 LIBGL_ALWAYS_SOFTWARE=1`. The
box was under load average ~10 from a concurrent sibling agent's full verify throughout. Emitted as
a `retro.py incident` with recurrence-key `run-desktop-driver-window-never-appears`.

**What is deliberately NOT done here:** the passing `desktop` stage is not offered as a substitute.
DoD item 8 exists precisely because a passing test cannot prove a player sees a value, and three
compute twins have each passed a gate while showing nothing on the sheet. The strongest player-path
evidence this cycle actually has is stated as exactly what it is, and no more: `character_hub`'s
`every_equipmods_row_the_picker_offers_is_recognized_by_the_attach_gate` proves that **all 584**
newly-offered Attach Modifier rows are recognized by the attach command and priced at the figure the
picker displays — zero refusals, zero new display-vs-charge divergences. That is a stronger-than-
usual code-path claim, and it is still not a screenshot.

**The remedy, for whoever picks this up:** re-run the driver on an unloaded box and screenshot the
Equipment Catalog filtered to book `UW` (127 rows, a book that served zero rows before this cycle —
the cleanest possible on-screen proof) plus one CRB `Equipmods` row from `cr_equipmods.lst`. If the
driver still cannot bring up a window, that is a tooling blocker worth its own card: DoD item 8 is
unsatisfiable for every player-visible lane in this bundle until it is fixed.
---

## Cycle SD29-E4-F1-001 — `epic-4-proven-spell` (Proven-Path Lane: spell, corpus-wide)

**Actor:** `sd29-e4-spell` · **Branch:** `tranche/9` · **Branch tip at claim:** `579d5941`
**Card:** `epic-4-proven-spell` (kanban Order 5) · **PR-id:** none (direct commit, pre-authorized)
**Commit:** `5f85d64e` · **Cycle-type:** proven-path kind lane, no new mechanism

### 1. The card's own figure was wrong, and correcting it WAS the bounded work

The card reads "corpus-wide, 1,754 remaining units". Re-deriving that number before accepting it
(Cycle mechanics step 1b) is what found the defect.

`v06_work_inventory::gather_engine_facts` built its `spell_levels` map from **three**
hand-written `.insert()` calls — `core_rulebook`, `advanced_players_guide`,
`advanced_class_guide` — while the shipped desktop `spell_catalog::build_spell_catalog` chained
**five**, adding `advanced_race_guide` and `ultimate_intrigue`. Every ARG and UI spell was
therefore reported `not-ingested` **while already being served to the player on screen**.

This is Decision 36's two-lists-one-fact pattern. It is not a new discovery in this repo: the
`equipment_keys` map **four lines below** the spell map in the same function was rebuilt by
SD-28-E15 for exactly this reason, and carries a doc comment describing the failure mode. The
spell family was left as the last hand-maintained copy.

**Landed:** `spell_resolver::spell_catalog_rows()` — one registry normalizing all five books'
mutually-incompatible `SpellListEntry`/`Pf1SchoolId` types (each book declares its own). Both
consumers now read it. `spell_book_slug_for` panics on an unmapped book code rather than
silently dropping a book, mirroring `equipment_book_slug_for`.

### 2. Re-derived figures — command first, value second, nothing transcribed

- **Spell remaining, before:** `1754`; **after:** `1561`. Command:
  `cargo run --locked --bin v06_work_inventory`, then a per-book/kind diff of the regenerated
  `docs/work-inventory.json` against `git show HEAD:docs/work-inventory.json` under the predicate
  `corpus-shape-37-books.md` §3 uses (`not-started + not-ingested + unknown`, `beginner_box`
  excluded). Exactly two rows moved:
  `advanced_race_guide` spell `{'not-ingested': 93}` → `{'ingested-magnitude': 92, 'not-ingested': 1}`;
  `ultimate_intrigue` spell `{'not-ingested': 101}` → `{'ingested-magnitude': 101}`.
  **No other book/kind pair in the file changed** — the same diff was run across every kind, not
  just `spell`, precisely so "surgical" is a measurement rather than a claim.
- **1,754 was itself correct as a starting total** (26 books carry `spell`, 2,843 units); the card
  and `corpus-shape-37-books.md` agreed with the file. The *file* was wrong, not the transcription.
  Both are corrected in place per "PRESS ON — this package's own stated figure turns out wrong".
- **Per-book base/`.COPY=` record counts** for the books that have a rule set but no spell table,
  straight from the PCGen oracle
  (`awk -F'\t' '!/^#/ && !/^SOURCE/ && NF>0 && $1 !~ /\.MOD/ && $1 !~ /\.COPY=/ {print $1}' <book>/*spells*.lst | sort -u | wc -l`):
  `ultimate_magic` 269+19, `ultimate_combat` 147+0, `core_essentials` 110+1,
  `ultimate_wilderness` 61+9, `ultimate_equipment` 1+0, `core_rulebook` 662+12.
  Handed forward so the next cycle-batch does not re-derive from scratch.
- **Disk preflight:** `./scripts/verify.sh --only preflight-disk` → PASS, 78% used, 106G available.

### 3. The defect the tests could not catch, and the screen did

DoD item 8 is the reason this cycle is not a false pass.

With the registry landed and every Rust and frontend test green, the running Spell Catalog read:

> "**1286** spells across the Core Rulebook, Advanced Player's Guide, Advanced Class Guide and
> Advanced Race Guide"

above filter chips `CRB (652) APG (297) ACG (144) ARG (92)` — **summing to 1185**. UI's 101 spells
were in the served payload and in the list, with no chip to filter to them and no player-facing
text naming their book. A **third** copy of the same list — the frontend's `BOOK_ORDER` /
`BOOK_LABELS` — had been stuck at four books since UI's spells were ingested.

**The frontend test written to prevent exactly this passed the whole time.** Its oracle,
`CHAINED_BOOK_CODES`, was a *copy of the constant under test* rather than an independent statement
of what `build_spell_catalog` chains, so it drifted in lockstep with the defect. Fixed RED-first
(`Error: BOOK_ORDER matches the Rust adapter chain order: expected CRB,APG,ACG,ARG,UI, got
CRB,APG,ACG,ARG`), then green: **98/98 frontend test files pass**. The test's header now instructs
successors to derive that constant from `spell_catalog.rs`'s chain, not from `BOOK_ORDER`.

This is the "validate proxies against known truth" lesson, on a proxy that had been wrong for a
full bundle while reporting itself healthy.

### 4. Tests added (TDD — RED confirmed for the intended reason in every case)

Rust (`src/bin/v06_work_inventory.rs`):
`spell_book_slug_for_covers_every_catalog_book`,
`arg_and_ui_spell_keys_are_reachable_through_the_derived_map`,
`registry_preserves_every_key_the_hand_maintained_map_carried` (the widening must be pure for the
three already-mapped books — this is what makes the change safe to land without moving any
already-`ingested-magnitude` unit).
Desktop (`spell_catalog.rs`): `mapping_helpers_agree_with_the_registry` — the five per-book
`map_*_entry` helpers are retained as the typed proof of which fields each book genuinely supplies,
and this test asserts the registry reproduces all five exactly, so they are proof rather than
a second implementation free to drift.

### 5. Definition of done

1. **`./scripts/verify.sh` (FULL) — exit code captured directly, never through a pipe**, by
   `echo "$status" > verify-<run>.exit` on the statement immediately after the command
   (`run-verify.sh`).

   **Run 2 (on the committed tree, `5f85d64e`): exit 1 — 11 of 12 stages PASS, and the single
   red stage is `preflight-disk`, not a content stage.**
   `pi-sweep` (10 hits / 10 baseline rows, CLEAN), `audit-selftest` (28), `root-lib` (1604),
   **`root-full` (6141 passed across 539 suites, all 522 `tests/*.rs` suites executed)**,
   `desktop` (414), `reach` (**16**), `frontend-install`, `frontend-test` (98/98 files),
   `frontend-typecheck` (clean), `clippy` (root:54 desktop:7 warnings, **0 errors**),
   `class-dump` (31/31 computing). The `all 522 tests/*.rs suites executed` clause is the
   `comm -23` derived-vs-`Running`-lines check Decision 40 requires — no suite silently skipped.
   `preflight-disk` failed on the box-wide floor (`max 90% used`): `/` at **90% used, 49G free**,
   against a 20G-free floor it cleared and a 90%-used ceiling it did not.
   **Remedy applied exactly as Cycle mechanics step 1c prescribes**, not routed around:
   `scripts/reclaim.sh` (dry run) → `scripts/reclaim.sh --apply` → re-check. `--apply` reclaimed
   what it safely could (one abandoned target dir, 1020.7KB) and **correctly declined the rest** —
   live target dirs, verify-log dirs under 6h, two worktrees with uncommitted changes, two
   branches checked out elsewhere. Still 90%. The pressure is **other concurrent agents'**
   `CARGO_TARGET_DIR`s, which this cycle must not delete. Releasing this cycle's own 27G build
   cache took `/` to **84% used, 79G available** and `preflight-disk` to **PASS** — confirmed by
   direct re-run.

   **Run 3 (same committed tree, cold build cache): `verify-run3.exit` = `0`. `RESULT: PASS`,
   all 12 of 12 stages PASS** — `preflight-disk`, `pi-sweep` (10 hits / 10 baseline rows),
   `audit-selftest` (28), `root-lib` (1604), `root-full` (**6141 passed across 539 suites, all
   522 `tests/*.rs` suites executed**), `desktop` (414), `reach` (**16**), `frontend-install`,
   `frontend-test` (98/98 files), `frontend-typecheck`, `clippy` (root:54 desktop:7 warnings,
   0 errors), `class-dump` (31/31 computing). **This is the run this cycle's DoD item 1 cites.**
   `root-full` passed here on a cold rebuild too, which is the third independent execution of
   `parse_runs_in_linear_time_on_a_synthetic_large_file` since run 1's red — see the `root-full`
   bullet below.

   **Run 4 — the run this receipt's exit code finally cites, because the tree changed after run 3.**
   `origin/tranche/9` advanced to `919703e3` (the equipment lane's two commits) while this cycle
   was verifying, so this lane **rebased** onto it rather than force-pushing over it (see §6.5).
   The rebased tree is a different tree, so the gate was re-run on it:
   `verify-final.exit` = **`0`**, `RESULT: PASS`, **12 of 12 stages** —
   `preflight-disk`, `pi-sweep`, `audit-selftest` (28), `root-lib` (1604),
   **`root-full` (6148 passed across 541 suites, all 523 `tests/*.rs` suites executed)**,
   `desktop` (414), `reach` (**16**), `frontend-install`, `frontend-test` (98/98),
   `frontend-typecheck`, `clippy` (0 errors), `class-dump` (31/31). The counts rise from run 3's
   6141/539/522 by the equipment lane's own new suite, as expected on a merged tree.
   An immediately preceding full run on the same rebased tree was 11/12 with only
   `preflight-disk` red (the box crossed the 90% ceiling again under other agents' builds); a
   `scripts/reclaim.sh --apply` and the box settling to 88% cleared it, and every content stage
   was green in **both** of those runs.

   All three **run-1** failures are separately accounted for, none accepted as "environmental" on
   assertion — and note that run 2 independently cleared all three:
   - **`frontend-test`** — REAL, and mine: the RED test of §3, caught mid-cycle. Fixed; 98/98 pass.
   - **`root-full`** — `parse_runs_in_linear_time_on_a_synthetic_large_file`
     (`tests/sd17_b5_equipment.rs:463`), a **wall-clock** assertion: "5k equipment records should
     parse in well under 2s, took 2.778812837s". Attribution **proven by re-execution, not
     asserted** (Cycle mechanics step 4): re-run 3× → `ok ... finished in 1.10s / 1.10s / 1.13s`,
     all comfortably inside the 2s bound. The red run overlapped a concurrent 496-crate
     `npx tauri dev` build for this cycle's own DoD-item-8 driving, on a 4-core box at load
     **10.59**. Nothing in this cycle touches equipment LST parsing. Annotated as flaky-under-
     contention per "Self-heal", and **not** re-fired blind — the re-run is the evidence.
     It then passed inside the full `root-full` stage in **both** run 2 and run 3 (the latter on
     a cold rebuild), so the assertion has now gone green 5 times against 1 red, and the one red
     is the only execution that overlapped a concurrent 496-crate build.
   - **`clippy`** — `desktop: could not count clippy output (errors= warnings=)`. The desktop
     clippy invocation contended with the same `tauri dev` build for the desktop target dir.
     Root clippy was clean and countable throughout (`root:54` warnings, **0 errors**).
   Per Decision 39 this is the **first** occurrence of each attribution; none recurred, so no
   `--recurrence-key <stage>-normalized-red` incident was warranted. A disk-pressure `incident`
   WAS auto-emitted by `verify.sh` itself (`/` at 90% used) and is recorded in §7.
2. **Reach claims — not zero, and this card's own families are among them.** The `reach` stage
   passed in all three runs (**16 matched claims**, never 0) and the two families this card moved,
   `advanced_race_guide/spells` and `ultimate_intrigue/spells`, are live claims backed by
   `spells_reach` over `build_spell_catalog` (`reach_gate.rs:758`, `:767`) — they were already
   claimed, which is precisely why reporting their units as `not-ingested` was incoherent. This
   card adds no record family and retires none.
3. `cargo run --locked --bin v06_corpus_trap_report -- --audit` → **exit 0** (written to
   `trap-audit.exit`): "No defects: every ingested record's citation agrees with the line it
   names" — 259 trap rows, 0 defects, `mod-record`.
4. **`docs/work-inventory.json` regenerated, and its units moved in the intended direction** —
   192 spell units left `not-ingested` for `ingested-magnitude`, nothing else moved. Committed.
   The standing hazard (regenerating `data/corpus` destroys `license`/`pi_field`/`raw_tokens`) is
   untouched: **no generator ran and no file under `data/corpus/` was written this cycle.**
5. **Wired-integration four-check over this cycle's files: clean.** No forbidden tokens, no
   no-op handlers, no mock leaks, no "Would …" strings in the added lines of
   `spell_resolver.rs`, `v06_work_inventory.rs`, `spell_catalog.rs`,
   `SpellCatalogScreen.tsx`/`.test.ts`. `scripts/wired-integration-audit.sh` (which audits the
   whole `develop...HEAD` bundle diff, not this cycle's) still reports the **one** pre-existing
   Check-1 hit from `epic-1b-naming-sweep`'s own diff —
   `placeholder="e.g. GE08 authoring workbench"` → `placeholder="e.g. authoring workbench"`, an
   HTML attribute, not a stub token. Recorded, not folded in — identical call to Epic 3's.
6. **`OPEN_FINDINGS` unchanged.** No family became unsurfaceable. The `beastiary1/race_traits`
   entry and the seven `<book>/archetypes` entries are untouched, as their owners require.
7. **No baseline movement committed.** `scripts/verify-baselines.env` deliberately untouched per
   the standing Epic-1 followup. Run 1 reported the drift as BASELINE NOTES (not failures):
   `ROOT_LIB_TESTS` 1488 recorded / **1604** measured, `DESKTOP_TESTS` 413 / **414** (+1 is this
   cycle's `mapping_helpers_agree_with_the_registry`), `CLIPPY_WARNINGS_ROOT` ceiling 75 / **54**
   measured. Epic 9/10 owns the separate reviewable `--show-actuals` commit.
8. **On-screen desktop verification — done, and it is what caught §3.**
   `RUN_DESKTOP_AGENT=sd29-e4-spell-cycle` (unique to this cycle), driven via
   `apps/desktop/.claude/skills/run-desktop/driver.sh`: `launch` → `title`
   (`WM_NAME(STRING) = "Codex"`, confirming our own window per the skill's known readiness gap)
   → `Browse Spell Catalog`. Both captures are committed beside this receipt.
   Before (`artifacts/e4-spell-catalog-before-ui-chip.png`) — 1286 served, four chips summing
   to 1185, four books named. After (`artifacts/e4-spell-catalog-after-ui-chip.png`) — chip row reads
   `CRB (652) APG (297) ACG (144) ARG (92)` **`UI (101)`**, summing to 1286, and the prose names
   Ultimate Intrigue. Rendered by the live Vite/Tauri app reading `list_spell_catalog` through
   the new registry; had the registry dropped a book, the totals would not reconcile.
   **Driver limitation recorded, not glossed:** `driver.sh click` did not reach the webview at
   this cycle's 1920×1200 geometry — the "Browse Spell Catalog" link needed a `focus`-then-`click`
   pair to register, and later chip clicks did not register at all (one produced a white paint
   artifact over an unrelated chip). The chip-filter interaction is therefore **not** claimed as
   driven; what is claimed is what the captured images show. A successor driving this screen
   should re-derive coordinates from a fresh screenshot rather than reuse the skill's documented
   1280×800 values.

### 6. Judgment calls taken as safe defaults (unattended mode — recorded, not asked)

1. **Scope: the divergence fix, not a six-book table generation sweep.** Extending the widened
   path to `ultimate_magic`/`ultimate_combat`/`ultimate_wilderness` additionally needs, per book,
   a `RuleSetId`-bearing table module, a `spell_catalog_rows` arm, a `spell_book_slug_for` arm, a
   `reach_gate` claim, a frontend label, and its own on-screen verification. Epic 1's budgeting
   rule ("budget your turn so the gate's exit code, the commit, and the receipt all land") was
   applied: landing that half-finished beside an unverified gate would have been worse than
   landing the correction proven. Recorded as a `deferral` with the re-derived per-book counts in
   §2, not as an omission. **Size was not the reason** — the divergence had to be fixed first
   regardless, because every subsequent book's ingest would have been silently under-reported by
   the same map.
2. **The worktree was reset onto `tranche/9`.** This agent was dispatched into
   `.claude/worktrees/wf_3516060a-756-7`, which was checked out at `7d9f1c4f` (a GE-08-era commit,
   PR #23) with **no `docs/` tree at all** — the SD-29 package did not exist in it.
   `git reset --hard 579d5941` onto `origin/tranche/9` (verified a descendant of the `a1295856`
   the dispatch brief named, via `git merge-base --is-ancestor`) after confirming the worktree was
   clean. The safer default: reset a clean scratch branch rather than refuse the card.
3. **`apps/desktop` `npm ci` was run in this worktree.** It had no `node_modules`, so
   `npx tauri dev` failed with `sh: 1: vite: not found`. Not a code change.
4. **The three real PI leaks stay standing.** `pi-sweep` passed in every run (10 hits / 10 baseline
   rows, CLEAN) — unchanged by this cycle, which generated **no** table under `rules_tables/` and
   so had no `screen_generated_table` call to make. Epic 3's two-line obligation is satisfied
   vacuously and stated here rather than silently skipped.
5. **Rebased onto the equipment lane, never force-pushed over it.** `origin/tranche/9` moved from
   `579d5941` to `919703e3` mid-cycle — `epic-4-proven-equip-mod` running concurrently in its own
   worktree. Per "STOP — the work would revert or clobber another session's live work on the
   shared branch", this lane rebased. Two conflicts, both resolved by keeping **both** lanes'
   content: `kanban.md` (their card's `IN-FLIGHT` claim + this card's row) and
   `docs/work-inventory.json`. The inventory was **regenerated**, not side-picked — both lanes
   change engine facts the generator reads, so either snapshot would have disagreed with the code
   beside it. The regen moves 14 equipment/equipment_modifier rows (remaining 1,144 → 959 and
   812 → 228, the equipment lane's own landed result, whose committed snapshot predated its
   tables) and **zero `spell` rows**: spell remaining stays 1,561, verified by diffing every
   book/kind pair rather than only the ones expected to move. Commit `33010d8d`.
6. **Cycle-id collision recorded, not papered over.** Both Epic 4 cards independently minted
   `SD29-E4-F1-001` — neither worktree could see the other's claim. Rewriting either receipt would
   desync it from its own commit messages, so the ids stand and `kanban.md` now carries a note
   saying to disambiguate by card id + `Claimed-by` (both unique), with a lane-suffix convention
   for the next concurrent split.

### 7. Disk

`verify.sh` auto-emitted a disk-pressure `incident` at `/` **90% used** (51G free) after run 1 —
this box carries several concurrent agents' `CARGO_TARGET_DIR`s. `scripts/reclaim.sh` (dry run)
reports **0 items, 0.0B**: every candidate is correctly declined by its own guards — target dirs
in live use, verify-log dirs younger than 6h, two worktrees with uncommitted changes, two branches
checked out elsewhere. The guards behaved correctly; there is simply nothing safely reclaimable
from this cycle's vantage point. `scripts/reclaim.sh --apply` run at cycle end regardless, per
Cycle mechanics step 8, and this cycle's own two aux target dirs
(`.codex-targets/sd29-e4-spell`, `.codex-targets/sd29-e4-spell-aux`) removed by hand.

### 8. What the next `epic-4-proven-spell` cycle-batch starts from

**1,561 remaining, not 1,754**, and the residual splits on a line that changes the work:

- **622 units in books that HAVE a compiled rule set** — `ultimate_magic` 291, `ultimate_combat`
  147, `core_essentials` 109, `ultimate_wilderness` 61, `core_rulebook` 12, `ultimate_equipment` 1,
  `advanced_race_guide` 1. These are reachable through the proven path: generate the book's
  `spell_list` table, add one arm to `spell_catalog_rows()`, one to `spell_book_slug_for`, one
  `BOOK_LABELS`/`BOOK_ORDER` entry, one `reach_gate` claim. The registry means the work inventory
  and the catalog both widen from that single arm.
- **939 units in books with NO compiled rule set** (`occult_adventures` 473, `inner_sea_gods` 96,
  `horror_adventures` 72, `bestiary_4` 56, `adventurers_guide` 49, and 10 more). These classify as
  `not-started`/`no_compiled_rule_set_for_book`. **A spell table cannot move them** — they need a
  rule set first. Do not scope them into a spell cycle-batch.
- `core_rulebook`'s 12 and ARG's 1 are `.COPY=` delta rows; the remedy is the existing
  `tests/sd27_apg_delta_spell_rows_resolve_against_their_base.rs` precedent, not new table rows.

### Retro events (`docs/retro/events/sd29-e4-spell.jsonl`)

1 × `correction` (the lane's own 1,754 → 1,561, `--verified-by` the regenerate-and-diff command),
1 × `incident` (`two-lists-one-fact-divergence`, `--silent`), 2 × `deferral` (the 1,561 residual
split by rule-set presence; the `Fins to Feet (self only)` `.COPY=` row), plus `verify.sh`'s
auto-emitted `verification` events from every run — including the red one — and its own
disk-pressure incident.

### Git discipline

`git status` run before every git write. No `git add -A` (nine explicit paths), no `git stash`.
Other actors' retro shards left dirty and uncommitted; only this actor's own shard is committed.
`CARGO_TARGET_DIR=/home/ubuntu/workspace/.codex-targets/sd29-e4-spell` (own dir, plus `-aux` for
concurrent non-gate cargo runs so they did not thrash the dir `verify.sh` was using) — Epic 1's
build-contention rule.

## Cycle SD29-E4-F2-001 — `epic-4-proven-feat-race-class` (Proven-Path Lane: feat + race + class)

**Actor:** `sd29-e4-frc` · **Branch:** `tranche/9` · **Branch tip at claim:** `cabf9089`
**Card:** `epic-4-proven-feat-race-class` (kanban Order 6) · **PR-id:** none (direct commit to `tranche/9`, pre-authorized)
**Cycle-type:** proven-path content lane, corpus-wide

### 0. The headline, stated before the detail: this card's premise was wrong for two of its three kinds

The card says "feat + race + class, corpus-wide … **No new mechanism needed**". That is true for
**feat** and false for **race** and **class**, and the classifier's own source says so:

```bash
# src/bin/v06_work_inventory.rs, gather_engine_facts():
#   feat_keys   <- all_feat_tables()                                   ... a CATALOG
#   race_names  <- RaceId::ALL.map(race_name)                          ... an ENUM of modelled races
#   class_books <- ClassId::ALL + ApgClassId::ALL + AcgClassId::ALL    ... an ENUM of modelled classes
# classify():
#   Kind::Race  -> not_ingested("race_absent_from_RaceId_ALL")
#   Kind::Class -> not_ingested("class_absent_from_ClassId_ALL_and_book_class_id_enums")
```

A `feat` unit becomes ingested when a **table row** exists — the equipment lane's exact shape. A
`race` or `class` unit becomes ingested only when a new **modelled entity** is added to an enum and
swept through the real compute pipeline (`RaceId` carries 7 variants today; `ClassId` +
`ApgClassId` + `AcgClassId` together carry the classes the sweep drives). Closing race's 62 and
class's 103 `not-ingested` units means **165 new modelled entities**, each with racial
modifiers/size/speed or a full class chassis. That is a mechanism per record, not a table row, and
it belongs with the Tier-2 mechanism-gated epics, not in a proven-path lane.

**Per "Stop vs. press on", a wrong premise in this package is a PRESS ON, not a stop.** This cycle
therefore closed the feat lane in full and did **not** attempt race or class. Both are reported
below as untouched, with their counts, so the next scoping pass has the real number rather than a
silent shortfall. Emitted as a `retro.py correction`.

### 1. Re-derived figures (command first, value second — nothing transcribed)

The card said **1,348 + 96 + 158**. Re-derived:

```bash
python3 -c "
import json
from collections import Counter
U=json.load(open('docs/work-inventory.json'))['units']
inc=[u for u in U if u['book']!='beginner_box']
for k in ('feat','race','class'):
    print(k, dict(Counter(u['status'] for u in inc if u['kind']==k)))"
# feat  {text-complete 1183, unknown 307, grounded 77, not-ingested 84, not-started 957, deferred-with-reason 2}
# race  {not-ingested 62, not-started 34, grounded 7}
# class {grounded 27, not-ingested 103, not-started 55}
```

| kind | card says | re-derived under `status in {not-started, not-ingested}` |
|---|---:|---:|
| feat | 1,348 | **1,041** (957 + 84) |
| race | 96 | **96** ✓ (34 + 62) |
| class | 158 | **158** ✓ (55 + 103) |

**The feat difference is a predicate difference, not an arithmetic error, and it is exactly the
kind's 307 `unknown` units:** 1,041 + 307 = 1,348. Every prior SD-29 lane (see the equipment lane's
receipt §1) used `{not-started, not-ingested}`; the card's figure additionally counted `unknown`.
Emitted as a `retro.py correction` rather than silently folded in.

**And, as in the equipment lane, "remaining" splits into two populations needing different work:**

```bash
python3 -c "
import json
from collections import Counter
U=json.load(open('docs/work-inventory.json'))['units']
rem=[u for u in U if u['book']!='beginner_box' and u['kind']=='feat'
     and u['status'] in ('not-started','not-ingested')]
print(Counter(u['status'] for u in rem))"
# Counter({'not-started': 957, 'not-ingested': 84})
```

The 957 `not-started` feat units live in books with **no compiled `RuleSetId` at all** —
`classify()` returns `not-started` before the feat catalog is ever consulted, so a catalog row for
them would move no unit and ship a record the engine has no rule set behind. **The proven path is
84 units wide, not 1,041.**

### 2. What landed

- **`src/bin/gen_feat_gap_tables.rs`** — the codegen, modelled directly on
  `gen_equipment_gap_tables`. Re-parses the 9 `.lst` files those 84 units come from (paths taken
  from the inventory's own `source_file` field, never a directory glob) under
  `v06_work_inventory::enumerate_file`'s **exact** `Kind::Feat` predicate — including its
  `has_classifying_token` requirement that a feat row carry a `TYPE:` token — and emits only the
  records the hand-authored tables do not hold. PI-screens the generated text through
  `pi_table_sweep::screen_generated_table` **before** writing (Epic 3's mandated line 1);
  `PCGEN_CORPUS_ROOT` is required from the environment with no default and no tilde expansion.
- **`src/rules_core/rules_tables/feat_gap_tables.rs`** — generated, **83 rows**.
- **`src/rules_core/rules_tables/feats_all.rs`** — `all_feat_tables()` is now
  `hand_authored_feat_tables()` (the eleven per-book projections, split out as its own public
  function so the generator's filter is provably their complement rather than a hand-maintained
  exclusion list) **with each book's gap rows appended last**, so a first-match key lookup over a
  book's slice resolves exactly as before.
- **`src/rules_core/feat_prereqs/pre_tokens.rs`** — `PRESIZEGTEQ:` is now a **modelled** kind
  (§4), and `PREHANDSGTEQ:` is declared unmodelled with its reason.
- **`apps/desktop/src-tauri/src/feat_catalog.rs`** — new
  `catalog_serves_every_corpus_gap_row` test; eleven pinned per-source counts and the category
  census re-derived (§3).
- **`tests/feat_gap_tables.rs`** — 6 integration tests.
- No change was needed in `apps/desktop/src-tauri/src/reach_gate.rs`: `feats_reach` derives its
  **claim** from `all_feat_tables()`, so the 83 rows entered the claim automatically and the gate
  asserts on them (DoD item 2, §6).

### 3. Per-book, and the counts that had to move

Generator output, re-derived by `tests/feat_gap_tables.rs` and
`feat_catalog::catalog_serves_every_corpus_gap_row` against the served catalog rather than copied
from stdout:

| book | gap rows | source |
|---|---:|---|
| `advanced_race_guide` | 48 | `arg_feats.lst` |
| `core_rulebook` | 16 | `cr_feats.lst` 1 + `core_essentials/ce_feats.lst` 15 |
| `ultimate_magic` | 12 | `um_feats.lst` 7 + `um_feats_wordsofpower.lst` 5 |
| `ultimate_intrigue` | 3 | `support/ui_feats_oa.lst` |
| `ultimate_combat` | 2 | `uc_feats.lst` |
| `ultimate_psionics` | 1 | `up_feats.lst` |
| `ultimate_wilderness` | 1 | `uw_feats.lst` (of 2 units — see §5) |
| **total** | **83** | |

**`core_essentials`' 15 rows are filed under CRB deliberately**, on the equipment lane's own
precedent: that shared library has no rule set, `core_rulebook.pcc` includes it unconditionally,
and the inventory reports these units `shared_library_record_held_by_no_ingested_host` precisely
because no ingested host's table holds them. Filing them under CRB is what lets `classify()`
attribute them.

**Counts that moved, each re-derived from the live catalog, none guessed:**

| pin | before | after | note |
|---|---:|---:|---|
| joined feat catalog total | 1,578 | **1,661** | +83, exactly the lane |
| desktop `build_feat_catalog()` | 1,578 | **1,661** | the picker's own total |
| `list_feats_for_character` | 1,578 | **1,661** | the character sheet's own list |
| served descriptions | 1,565 | **1,630** | +65; 18 gap rows carry neither `DESC:` nor `BENEFIT:` and are served with **no** description rather than a fabricated one |
| records carrying any prerequisite | 1,429 | **1,492** | +63 of 83 |
| prerequisite clauses / kinds | 3,805 / 35 | **3,914 / 37** | §4 |
| a starting Fighter's eligible feats | 509 | **552** | §4 |

**Four per-book assertions were pointed at `hand_authored_feat_tables()` rather than renumbered**
(`spans_every_ingested_book_with_their_real_counts`, `each_books_slice_is_exactly_its_own_table`,
`the_per_book_category_split_is_the_real_one`,
`the_per_book_prerequisite_coverage_is_the_real_one`). Each states a fact about what that book's
own module authored; pointing them at the joined catalog would have converted a per-book ingest pin
into a pin on this lane's size. The joined catalog is pinned separately and per book by the new
`the_joined_catalog_is_the_hand_authored_one_plus_the_corpus_gap_rows`.

### 4. Two real findings the gates produced, neither routed around

**(a) The lane introduced two prerequisite token kinds the evaluator had never seen.**
`tests/sd27_feat_prerequisite_enforcement.rs::every_pre_kind_in_the_catalog_is_either_modelled_or_declared_unmodelled`
— the completeness guard — failed with `["PREHANDSGTEQ (1 occurrences)", "PRESIZEGTEQ (2 occurrences)"]`.
Without that guard those three records' prerequisites would have been **silently ignored** and the
feats offered to characters who do not qualify. Resolved in the two different, honest ways:

* **`PRESIZEGTEQ:` is now MODELLED**, not declared unmodelled. `PRESIZELTEQ:` was already modelled
  against `facts.size`, so the fact it needs was already in hand; declaring it unmodelled would
  have been a fabricated gap. `evaluate_size_lteq` became `evaluate_size_bound(.., at_least)`,
  test-first (`size_at_least_is_evaluated_against_the_characters_real_size`, confirmed RED with
  `Unmodelled { token: "PRESIZEGTEQ:L", note: "this prerequisite kind has no landed evaluation
  path" }` before the change).
* **`PREHANDSGTEQ:` is declared unmodelled with its reason** — a creature's limb count is a fact
  this engine holds nowhere; no ingested race records it and no chassis computes it. Guessing "2"
  would deny or allow on an assumption, which is the exact failure `ClauseOutcome`'s third variant
  exists to prevent.

**Modelling `PRESIZEGTEQ` then moved a number DOWN, which is the point:** a starting Fighter's
eligible-feat count is **552, not 553** — `Awesome Blow` (`PRESIZEGTEQ:L`) is now correctly denied
to a Medium character with a stated reason rather than offered under an unverifiable prerequisite.

**(b) Two new cross-book key collisions, both correct, each checked against the owning record.**
`cross_book_key_collisions_are_exactly_the_known_set` went from 1 to 3. Per this card's own
warning that a shared name never implies a shared thing, neither was accepted on the name:

* `Feral Combat Training` (Uc / Upsi) — `up_feats.lst` carries the comment *"Feral Combat Training
  copied from Ultimate Combat - consider INCLUDEing (and .MODding) it"* immediately above the
  record. **The corpus states the reprint itself.**
* `Extended Animal Focus` (Acg / Uw) — one record in `uw_feats.lst`, the same Hunter animal-focus
  feat ACG prints; UW is the book that expands animal focus.

Following the equipment lane's precedent, the original review was kept intact rather than
renumbered: the test now asserts `Endurance` is still the **only** collision across the
hand-authored tables (so a new clash between two books' own ingests fails exactly as before) and
pins the joined catalog's three exactly, so a **third** collision — which might well be two
different feats sharing a name — still fails.

### 5. The one unit this lane deliberately does not close

`uw_feats.lst:164` is `CATEGORY=Special Ability|Samurai ~ Mount.MOD	TYPE:Mount`. A `.MOD` row is an
**overlay onto a record defined elsewhere**, not a new record: `enumerate_file` skips it on the
normal path and stashes it in `mod_targets` for corpus-wide resolution afterwards, which is the only
reason it surfaces as a unit at all. Emitting a `RuleSetId::Uw` catalog row for it would ship a UW
feat the corpus never declared. So the generator emits **83, not 84**, and the unit stays
`not-ingested` with a named test
(`the_one_not_ingested_feat_unit_this_lane_deliberately_does_not_close`) carrying the reason.

**Caught by differencing the generator's 83 against the inventory's 84** — the equipment lane's own
receipt recommended exactly that check, and it is what found this. Emitted as a `retro.py
correction`.

### 6. Work-inventory movement (DoD item 4)

`cargo run --locked --bin v06_work_inventory` → exit 0:

```bash
python3 -c "
import json
from collections import Counter
U=json.load(open('docs/work-inventory.json'))['units']
inc=[u for u in U if u['book']!='beginner_box']
for k in ('feat','race','class'):
    print(k, dict(Counter(u['status'] for u in inc if u['kind']==k)))"
# feat  {text-complete 1240, unknown 333, grounded 77, not-started 957, deferred-with-reason 2, not-ingested 1}
# race  {not-ingested 62, not-started 34, grounded 7}
# class {grounded 27, not-ingested 103, not-started 55}
```

**feat `not-ingested`: 84 → 1** (the documented `.MOD` residual, §5). `text-complete` +57 and
`unknown` +26 — the split reflects each record's own magnitude tokens, not a shortfall. The 957
`not-started` units are untouched and belong to books with no compiled rule set (§1).

**race and class are unchanged, and that is reported, not hidden:** 96 and 158 remaining
respectively, for the reason in §0.

**The standing hazard was not touched:** no generator wrote anything under `data/corpus/`, so no
`license`/`pi_field`/`raw_tokens` were destroyed.

### 7. Honest reporting of "proven"

`work-inventory`'s `proven` predicate is `{grounded, text-complete}`, and a feat reaches `grounded`
only when `probe_feat_effect_wiring` observes a computed delta for its key. For the 83 rows:

- **engine-holds (`text-complete` or `unknown`): all 83.** The engine holds each record with its
  real corpus `DESC:`/`BENEFIT:`/`TYPE:`/`PRE`-family tokens, the desktop catalog serves it, and
  the prerequisite gate evaluates it.
- **strictly-proven (`grounded`): 0 of the 83.** No feat-effect producer was written for these
  keys — this lane adds catalog records, not effect producers, and claiming otherwise would be the
  fabrication this program's doctrine exists to prevent.

### 8. Git discipline

`git status --porcelain` run before every git write. No `git add -A` — explicit paths only. No
`git stash`. `CARGO_TARGET_DIR=/home/ubuntu/workspace/codex-target-sd29-e4-frc` (plus a separate
`-desktop` dir for the Tauri crate) — this agent's own directories, never under `/tmp`.

**One environment note worth recording:** this agent's assigned worktree was provisioned at commit
`7d9f1c4f` (an 80-file tree from PR #23, months of history behind `tranche/9`) with no `docs/`,
`scripts/` or `data/` at all. It was clean, so it was reset onto `origin/tranche/9` (`cabf9089`)
before any work began. Nothing of another actor's was touched.

### 9. Definition of done — status at receipt time

1. **`./scripts/verify.sh` (FULL) — NOT SATISFIED AT RECEIPT TIME.** Run 2 was still inside
   `root-full`'s ~490-binary build when this cycle's turn budget ran out. **No exit code is
   claimed**, and the card is therefore left `IN-FLIGHT`, not `COMPLETE`. What IS proven is in §10.
   The command was launched with its exit code captured directly
   (`echo "VERIFY_EXIT=$?" > verify2.exit` on the statement immediately after it, never through a
   pipe); the file is empty because the run had not returned.
2. **Reach claims — this card's family is `<book>/feats` for seven books, and no code change was
   needed to make the claim cover the new rows.** `reach_gate.rs`'s `feats_reach` builds its
   `ingested` set from `all_feat_tables()` itself, so appending the gap rows put them inside the
   **claim**, not merely inside the surface: the gate now asserts that all 83 reach
   `build_feat_catalog()`. Nothing is claimed by absence. Backed independently by the new
   `feat_catalog::catalog_serves_every_corpus_gap_row`, which checks each row **by key, under its
   own book's wire `source`** — a total moving by 83 proves 83 rows arrived somewhere, not that
   *these* 83 arrived attributed to the right book.
3. `cargo run --locked --bin v06_corpus_trap_report -- --audit` — §10.
4. **`docs/work-inventory.json` regenerated, units moved: feat `not-ingested` 84 → 1** (§6).
5. **Wired-integration four-check audit over this cycle's files: clean.** No TODO/FIXME tokens, no
   no-op handlers, no mock leaks, no "would have" strings. The rows are served by the same
   `build_feat_catalog()` the Feat picker, `list_feat_catalog` and `list_feats_for_character`
   already read — a real path, not a parallel one — and they are *gated* on that path, not merely
   listed: 39 of the 83 are correctly ineligible for a level-1 Fighter, each with a stated reason.
6. **`OPEN_FINDINGS` unchanged.** No family became unsurfaceable. The one unclosed feat unit (§5)
   is not an unsurfaced family — it is a `.MOD` overlay whose base record is another book's, pinned
   by a named test rather than by an `OPEN_FINDINGS` entry.
7. **No baseline movement committed.** `scripts/verify-baselines.env` deliberately untouched per
   the standing Epic-1 followup; the four SD-28 drifts still stand and Epic 9/10 owns that separate
   `--show-actuals` commit.
8. **On-screen desktop verification: NOT PERFORMED — recorded as a shortfall in §11, not claimed
   and not substituted.** This is the one DoD item this cycle does not satisfy.

### 10. Verification results

**`./scripts/verify.sh` (FULL) — run 2, on the final rebased tree. RESULT: INCOMPLETE, no exit
code. Four stages PASS, `root-full` still building when the cycle ended.** Stated as exactly that
and no more — a gate that has not returned is not a gate that passed.

Stage results recorded in `verify2.log` at the point the cycle ended:

| stage | result |
|---|---|
| `preflight-disk` | PASS (disk budget OK) |
| `pi-sweep` | PASS — 10 hits over `src/rules_core/rules_tables`, 10 baseline rows, **CLEAN**. The 83 new rows added **zero** PI hits; the generator's own pre-write screen also reported `CLEAN (0 hits)`. |
| `audit-selftest` | PASS (28 cases) |
| `root-lib` | PASS (**1606** passed) |
| `root-full` | **IN FLIGHT** — still building ~490 test binaries (`/tmp/codex-verify-KH26C1/root-full.log`, 0 suites reported) |
| `desktop` / `reach` / `frontend-*` / `clippy` / `class-dump` | **NOT REACHED** |

**What stands behind the unfinished stages, stated as the weaker claim it is:** every suite these
stages run was run directly by this cycle and was green — `cargo test --locked --lib` (1605),
`cargo test --locked --test feat_gap_tables` (6), `--test sd27_feat_prerequisite_enforcement` (9),
and `cargo test --locked` in `apps/desktop/src-tauri` (414). That is not a substitute for the gate
and is not offered as one; `root-full` reaches test binaries those commands do not, which is
precisely why `decisions.md` Decision 8 forbids composing a substitute command set.

**A proxy-validation error this cycle made and caught, worth recording:** while waiting, `root-full`
progress was read out of `/tmp/codex-verify-wXu5Pr/root-full.log` — 540 suites, 0 failures — and
briefly taken as this run's result. It was a **sibling agent's** log directory; run 2's is
`KH26C1`, which held 0 results. Caught by re-reading `verify2.log`'s own `logs:` line rather than
pattern-matching a directory name. Exactly the "validate the proxy where it makes the confident
claim" failure, on a shared box where several agents write `/tmp/codex-verify-*`.

**Run 1 was RED, correctly, and its failure was the gate doing its job**: `root-lib` failed on
`a_starting_fighter_keeps_a_real_catalog_and_every_denial_states_why` (552 vs 553) — the direct
consequence of modelling `PRESIZEGTEQ:` mid-cycle, i.e. one feat that had been offered under an
unverifiable prerequisite is now correctly denied. Fixed at source, not routed around. No stage was
weakened, skipped or `#[ignore]`d.

**Every failure this cycle hit was a pin that had to move or a real finding; none was worked
around.** The full list, in the order they surfaced: 7 `root-lib` pins, 4 root integration pins in
`tests/sd27_feat_prerequisite_enforcement.rs` (one of them the completeness guard of §4a, a real
finding), 5 desktop pins, and the run-1 `root-lib` regression above.

### 11. DoD item 8 — on-screen desktop verification: **NOT PERFORMED. Recorded as a shortfall, not claimed.**

The feat catalog is player-visible, so item 8 binds here. `RUN_DESKTOP_AGENT` was set to a value
unique to this cycle (`sd29-e4-frc-cycle1`) per the skill's concurrency rule. It could not be
performed, and this cycle located the failure considerably more precisely than the equipment lane's
cycle did — enough that the remedy it proposed ("re-run on an unloaded box") is now known to be the
wrong remedy.

**What was established, with commands:**

```bash
DISPLAY=:93 xdotool getdisplaygeometry      # -> 1920 1200   (Xvfb is healthy)
DISPLAY=:93 xdotool search --name Codex     # -> 2097153     (the window DOES exist)
DISPLAY=:93 xdotool getwindowname 2097153   # -> codex-desktop
DISPLAY=:93 xwininfo -id 2097153            # -> Map State: IsViewable, 1600x1000 at +10+10
DISPLAY=:93 import -window 2097153 shot.png # -> "Resource temporarily unavailable"
DISPLAY=:93 import -window root -crop 1600x1000+10+10 shot2.png
identify shot2.png                          # -> PNG 1600x1000 ... 2c 377B   (blank: 2 colours)
```

So the window is created, mapped and viewable, and the webview really is running the frontend (its
`console.warn` lines stream into the tauri log). **What fails is capture**: WebKit renders through a
compositing path X11 cannot read back, consistent with the
`libEGL warning: DRI3 error: Could not get DRI3 device` logged on every launch. Waiting longer or
unloading the box does not address that.

**Three concrete defects in the tooling this cycle found, each worth a card:**

1. **`driver.sh` searches for a window titled `Codex`; the real `WM_NAME` is `codex-desktop`.** That
   alone makes `launch` time out with "Timed out waiting for launch" on a run where the app came up
   perfectly.
2. **`driver.sh` derives `DISPLAY_NUM` by `cksum` into 60..89 and does not clear stale locks.** Ten
   `/tmp/.X*-lock` files from killed sibling agents were on the box; landing on one gives
   `Failed to initialize GTK` with no hint of the cause.
3. **The vite port 1420 is not partitioned per agent the way `DISPLAY` is.** A second app run on the
   same box — including against one's own orphaned vite from a torn-down attempt — dies with
   `Port 1420 is already in use`.

**What is deliberately NOT done here:** the passing `desktop` stage is not offered as a substitute.
The strongest player-path evidence this cycle actually has is stated as exactly what it is and no
more: `catalog_serves_every_corpus_gap_row` proves all 83 rows are served by `build_feat_catalog()`
under the correct book, and `a_starting_fighter_keeps_a_real_catalog_and_every_denial_states_why`
proves the picker's eligibility verdict is computed for every one of them, with 39 correctly
ineligible and each carrying a reason string. That is a strong code-path claim. It is still not a
screenshot.

### 12. What the next lane inherits

1. **`gen_feat_gap_tables` is re-runnable and self-checking.** Adding a book to `BOOK_INPUTS` is the
   whole change; the already-held filter derives from `hand_authored_feat_tables()`, so it cannot
   drift.
2. **Splitting `hand_authored_*` from the joined catalog is now the lane pattern, and it does more
   than feed the generator.** It is also what let four per-book assertions keep their original
   numbers and their original meaning instead of being renumbered into pins on this lane's size.
   Any lane widening a shared aggregate should do the same before touching a single count.
3. **Widening a catalog can introduce prerequisite/formula token kinds the evaluator has never
   seen** (§4a). Expect the completeness guard to fire, and answer it per token: model it when the
   fact is already in hand, declare it unmodelled with a reason when it is not. Do not reach for the
   unmodelled list by default.
4. **`race` and `class` are not proven-path work** (§0). 96 + 158 units, requiring 165 new modelled
   entities in `RaceId`/`ClassId`/`ApgClassId`/`AcgClassId` plus compute-pipeline sweeps. They need
   a mechanism-gated epic alongside Epics 5-7, not a Tier-1 card.
5. **DoD item 8 is currently unsatisfiable for every player-visible lane in this bundle** until the
   three driver defects in §11 are fixed. That is now a diagnosed tooling blocker with named
   remedies, not a mystery.

---

## Cycle SD29-E6-F1-001 — `epic-6-race-trait-lane-pilot` (Race-Trait Lane: pilot)

**Actor:** `sd29-e6-racetrait-pilot` · **Branch:** `tranche/9` · **Base:** `e16c0a02` ·
**Commit:** `bf08d524` · **Date:** 2026-08-11

**Outcome: SPLIT.** The card carried two deliverables. The **classifier defect fix is COMPLETE**.
The **`inner_sea_intrigue` pilot ingest is `decision-blocked`** — the pinned pilot book carries zero
race traits. Neither half was fudged into the other.

### 0. Cycle-start correction: wrong-base worktree

The dispatched worktree was cut from `origin/main` (`7d9f1c4f`, a GE-08-era commit), not `tranche/9`.
`docs/release/SD-29-corpus-wide-catch-up-lanes/` did not exist in it. Detected by the first
`ls` of the package directory; resolved with `git fetch origin tranche/9 && git reset --hard
origin/tranche/9` (clean tree, no work to lose). Recorded here because
`docs/retro/events/` already carries `wrong-base-worktree` as a named recurring incident class.

### 1. The defect, re-derived before any code changed

`corpus-work-channels.md` §9.3 / SD-28 §56. `v06_work_inventory`'s `Kind::RaceTrait` arm built a
candidate id for **every** race the engine models and grounded on any hit:

```rust
let candidates: Vec<String> = facts.race_names.iter()
    .map(|r| format!("{r}.{}", slug(&unit.name))).collect();
if candidates.iter().any(|c| facts.race_trait_ids.contains(c)) { /* grounded */ }
```

`race_trait_ids` is built solely from CRB's hardcoded `race_traits()` table
(`src/rules_core/rules_tables/crb/race_tables.rs`, 49 rows over 7 races), so **any** book's trait
reached `grounded` on a trait-name collision alone.

Re-derived pre-fix, command recorded in full:

```
python3 -c "import json,collections; d=json.load(open('docs/work-inventory.json')); \
  g=[x for x in d['units'] if x['kind']=='race_trait' and x['status']=='grounded']; \
  print(len(g), collections.Counter(x['book'] for x in g))"
```

→ `44 Counter({'core_essentials': 39, 'ultimate_psionics': 4, 'advanced_race_guide': 1})` — matching
§9.3's stated 44/39/4/1 exactly.

### 2. §9.3 undercounted the defect — corrected in place, not folded silently

§9.3 says 4 false positives. **There were 23.** The 19 it missed are *intra-`core_essentials`
cross-race* coincidences, not the cross-book form it looked for:

| Record | grounded off | why it is wrong |
|---|---|---|
| `Aquatic Elf ~ Elven Magic` | `elf.elven_magic` | "Aquatic Elf" is not "Elf" |
| `Drow ~ Keen Senses` | `elf.keen_senses` | Drow is not modelled |
| `Svirfneblin ~ Stonecunning` | `dwarf.stonecunning` | Svirfneblin is not modelled |
| `Blue ~ Keen Senses` (UPsi) | `elf.keen_senses` | §9.3's own example |

Corrected in `docs/release/corpus-work-channels.md` §9.3 with a dated `FIXED 2026-08-11` block, and
emitted as a `correction` retro event with `--verified-by` (per "Retrospective log": a correction
without it is a competing assertion).

§9.3's **headline claim survives intact**: exactly one legitimate grounded race trait outside
`core_essentials` — ARG's `Saltbeard ~ Dwarf ~ Greed`.

### 3. The fix

New `modelled_race_of_race_trait(key, race_names)` reads the record's own race from its corpus key's
`~`-qualifiers. Two design points, both load-bearing:

1. **The trailing segment is excluded.** It is the trait name, never the race; without the exclusion
   a trait named after a race would nominate itself and re-open the same coincidence class.
2. **All leading qualifiers are searched, not just the first.** ARG's heritage form
   `Saltbeard ~ Dwarf ~ Greed` carries its base race in the *middle*. A first-segment-only rule
   would have discarded the one legitimate non-`core_essentials` record.

Applied to **both** sites — the `Kind::RaceTrait` verdict arm and its twin in
`EngineFacts::holds_key` (which was book-gated but still race-blind). A one-site fix would have left
the shared-library host-attribution path defective.

TDD: 3 tests written first in a new `race_trait_grounding_tests` module, named for function per the
2026-08-11 naming directive (no `sd29_`/`SD-NN` tag). They pin the four §9.3 false positives by name,
the two legitimate forms, and the self-nomination guard.

### 4. Result, re-derived after the fix

Same command as §1 against the regenerated inventory:

→ `21 Counter({'core_essentials': 20, 'advanced_race_guide': 1})`

**44 → 21.** 23 false positives removed; the 21 survivors are 20 genuine CRB traits plus the ARG
heritage record.

**One true finding surfaced.** `Dwarf ~ Hatred` was grounding off `gnome.hatred`. CRB's
`race_traits()` table carries `Hatred` for **Gnome only** (`grep -n "Hatred" -B3
src/rules_core/rules_tables/crb/race_tables.rs` → a single hit, `race_id: RaceId::Gnome`), although
PF1e gives dwarves Hatred too. The Dwarf record now correctly reports `not-ingested`. This is a real
gap in the hardcoded table exposed by the fix, **not** a regression — flagged here rather than
patched, because widening `race_traits()` is CRB chassis work outside this card.

### 5. Why the pilot ingest is `decision-blocked`

The card pinned `inner_sea_intrigue` (9 units). **The count is right; the kind is not.**

```
python3 -c "import json; d=json.load(open('docs/work-inventory.json')); \
  [print(x['source_file'],'|',x['corpus_key']) for x in d['units'] \
   if x['book']=='inner_sea_intrigue' and x['kind']=='race_trait']"
```

→ all 9 from `isi_abilities_race_companion.lst`: `Clockwork Familiar ~ Electricity`,
`~ Item Installation`, `~ Potion/Scroll/Wand Installation`, `~ Tinkering`,
`Clockwork Spy ~ Record Audio`, `~ Self-Destruct`, `~ Tinkering`.

Confirmed **at source**, not from the inventory's own classification:
`grep -rn "Clockwork Familiar" ~/workspace/repos/pcgen/data/pathfinder/paizo/campaign_setting/inner_sea_intrigue/*.lst`
→ `CATEGORY:Special Ability  TYPE:ClockworkFamiliarRacialAbility.SpecialQuality.Extraordinary`.

These are **construct-companion abilities** — Epic 7 Companion-Lane shape. They are typed
`race_trait` only because `file_kind()` types `*_abilities_race*.lst` by **filename**, and this file
is `..._abilities_race_companion.lst`. `inner_sea_intrigue` has **zero** genuine race traits.

Building the lane mechanism against them would validate it against content of the wrong kind —
precisely the untrustworthy success criterion §9.3 warns of. This is loop-instruction.md's named hard
stop: *"a book's derived shape contradicts its recorded ingest subtype — the cycle reports; the
operator re-pins the book list."* No substitute pilot book was chosen unilaterally.

**Blast radius is small:** corpus-wide only **11** of 3,456 `race_trait` units come from
`*companion*.lst` (9 here + 2 in `b4_abilities_race_ce_companion.lst`). The **lane** is sound; only
the **pilot selection** is wrong. Epic 6's 3,412 extend figure is not materially affected.

**Re-pin candidates** (re-derived, `*companion*.lst` excluded): `ultimate_intrigue` (3),
`ultimate_magic` (3), `inner_sea_bestiary` (4), `ultimate_combat` (4), `monster_codex` (14),
`bestiary` (21). **Recommended: `monster_codex`** — DoD item 6 already expects it to retire the
standing `beastiary1/race_traits` `OPEN_FINDINGS` entry, so the pilot and that retirement land
together.

### 6. Verification

`./scripts/verify.sh` (FULL, exit code captured directly, never through a pipe):
**`VERIFY_EXIT=1`** — 10 stages passed, 2 failed. **Neither failure is attributable to this card**,
and neither was routed around.

| Stage | Result |
|---|---|
| pi-sweep, audit-selftest, desktop (415), reach (16), frontend-{install,test,typecheck}, clippy (root 55 / desktop 7, 0 errors), class-dump (31/31) | PASS |
| root-lib | PASS (1606) |
| **root-full** | **FAIL** — cargo exit 101; 6157 passed / 543 suites |
| **preflight-disk** | **FAIL** — 94% used |

**`root-full` — pre-existing, proven not mine.** Two failures, both in
`tests/v06_apg_acg_feat_catalog.rs`: `the_aggregate_catalog_spans_every_ingested_book` (expected
185, got 201) and `cross_book_feat_key_repeats_are_exactly_the_known_set` (gained
`Extended Animal Focus (Acg,Uw)`, `Feral Combat Training (Uc,Upsi)`). Both are **feat** assertions
left stale by `dde9dfc4 feat(sd29): close the feat not-ingested gap corpus-wide (83 rows)`.

Attribution is proven, not asserted: `git diff --name-only` shows this card's only code change is
`src/bin/v06_work_inventory.rs` — a **binary**. Integration tests link the `codex` **lib** crate and
cannot observe a bin. `git diff --name-only | grep -c "^src/rules_core\|^src/lib.rs"` → **0**.

**Not fixed here.** `tests/v06_apg_acg_feat_catalog.rs` belongs to `epic-4-proven-feat-race-class`,
still `IN-FLIGHT` under `sd29-e4-frc`. Editing it would clobber a live agent's work on the shared
branch — loop-instruction.md's explicit STOP. Emitted as an `incident` with recurrence-key
`stale-count-assertion-after-record-change`. **Every card dispatched from `tranche/9` after
`dde9dfc4` inherits this red root-full.**

**`preflight-disk` — environmental, deliberately NOT overridden.** 94% used / 34G free on a box
running 5 concurrent worktree agents. The floor that tripped is the **percentage** check (max 90%),
not the 20G free-space check, which passed comfortably. `scripts/reclaim.sh` (dry run) could free
only **985.7KB** — every other candidate is a live agent's cargo target dir or a worktree with
uncommitted/unpushed work, all correctly refused by the script's own guards.

`verify.sh` offers `PREFLIGHT_DISK_MAX_PERCENT` as a documented override. **This cycle did not use
it.** Weakening a gate to obtain green is banned outright by loop-instruction.md, and an
override would have converted a true capacity signal into a fabricated pass. Emitted as an
`incident` with recurrence-key `disk-full`.

Build contention was handled per Epic 1's recorded incident: own
`CARGO_TARGET_DIR=/tmp/codex-target-sd29-e6-racetrait-pilot`, gate launched early in the background
while the remaining bounded work proceeded.

### 7. Definition of done

| # | Item | Status |
|---|---|---|
| 1 | `verify.sh` exits 0 | **NO** — exit 1; both failing stages proven not attributable (§6), neither weakened nor skipped |
| 2 | `reach` passes with a claim for this card's families | **YES** — `reach` PASS, **16 matched tests, not zero**. `race_traits` is a live family with real claims (`reach_gate.rs:548`, `:924-937` for crb/beastiary1/arg/apg); it does not pass by absence |
| 3 | `v06_corpus_trap_report -- --audit` exits 0 | **YES** — `TRAP_AUDIT_EXIT=0`, 259 traps / **0 defects** |
| 4 | `v06_work_inventory` regenerates; second run changes only `generated_at` | **YES** — both runs exit 0; `diff` of both outputs with `generated_at` filtered → **exit 0** |
| 5 | Four-check wired-integration audit clean | **YES** — no new production path added; the change removes false positives from a reporting instrument. No stub, no fixture-only data, no dead affordance introduced |
| 6 | Unsurfaceable families carry an `OPEN_FINDINGS` entry | **N/A** — no family was newly surfaced. The standing `beastiary1/race_traits` entry is **left standing**, correctly: it is expected to retire under Epic 5's Monster Codex batch, which this card did not run |
| 7 | Baseline movements in a separate commit with `--show-actuals` | **N/A — deliberately untouched.** The run reported 3 stale baselines (`ROOT_LIB_TESTS` 1488→1606, `DESKTOP_TESTS` 413→415, `CLIPPY_WARNINGS_ROOT` ceiling 75 vs 55 measured). Per the standing Epic 1 followup these belong to Epic 9/10; **not** folded in |
| 8 | On-screen verification for player-visible families | **N/A** — this card surfaced **zero** new player-visible families; it corrected a reporting instrument (`docs/work-inventory.json`). Independently, `epic-4-proven-feat-race-class`'s receipt §11-12.5 records DoD item 8 as currently unsatisfiable bundle-wide pending three named `driver.sh` defects |

### 8. What the next lane inherits

1. **The race-trait grounding instrument is now trustworthy.** Any Epic 6 cycle can use
   `status == "grounded"` as a real success criterion. Before this cycle it could not — which is
   exactly why §9.3 ruled the fix must land *alongside* the lane rather than after it.
2. **Pilot-book selection must check unit KIND, not just unit COUNT.** `inner_sea_intrigue` was
   picked because it had 9 units. All 9 were the wrong kind. `file_kind()` types by filename, so a
   `*_abilities_race_companion.lst` lands in the `race_trait` bucket — check `source_file` before
   pinning a pilot.
3. **`Dwarf ~ Hatred` is a real hole in `crb::race_tables`** (§4), now visible instead of masked.
4. **`root-full` is red on `tranche/9` for reasons no current card owns** (§6). The next card to run
   the full gate should expect it and must not attribute it to itself.
---

## Cycle SD29-E7-F1-001 — `epic-7-companion-lane-pilot` — **BLOCKED (not started)**

- **Actor:** `sd29-e7-companion-pilot`
- **Date:** 2026-08-11
- **Branch tip at read:** `579d5941` (tranche/9)
- **Card status left at:** `READY` (deliberately **not** claimed — the cycle never reached
  Cycle-mechanics step 2, so claiming would have parked the card `IN-FLIGHT` under an agent that
  did no bounded work)
- **PR-id:** none (no commit produced)

### 1. Why this cycle did not start

`loop-instruction.md` Cycle mechanics **step 1c** is a refusal gate: *"Refuse to start the bounded
work below if it fails."* It failed, twice, with the reclaim step in between:

```
./scripts/verify.sh --only preflight-disk        -> EXIT=1
    repo filesystem (…/.claude/worktrees/wf_3516060a-756-11, mounted at /): 91% used, 47G available
    FAIL: disk budget below floor (max 90% used, min 20G free).
scripts/reclaim.sh                               -> dry run; only stale verify-log dirs eligible
scripts/reclaim.sh --apply                       -> exit 0, ~1MB reclaimed; df unchanged at 91%
./scripts/verify.sh --only preflight-disk        -> EXIT=1  (same figures)
```

`reclaim.sh` behaved correctly and reclaimed nothing meaningful: every `cargo-target`, `worktrees`
and `branches` candidate was `SKIPPED` as live / unpushed / uncommitted — five sibling SD-29
worktree agents (`wf_3516060a-756-6…-10`) hold them.

### 2. The gate is right here, not a percentage artifact — re-derived

The tempting judgment call was to press on with the gate's own documented override
(`PREFLIGHT_DISK_MAX_PERCENT`), on the theory that 47G free clears the 20G free floor by 2.4x and
only the *percentage* criterion trips because the disk is large. That theory was tested against the
real cost of a target dir rather than assumed, and it is wrong:

```
timeout 110 du -s --block-size=1G -x /home/ubuntu/workspace/repos/codex/target   ->  60   (GB)
df -h /                                                                          ->  48G available
```

**One cargo target dir for this repo is ~60G; 48G is available.** The BUILD-CONTENTION RULE requires
this cycle to export its *own* `CARGO_TARGET_DIR`, and a fresh one for a FULL ~490-binary sweep does
not fit. Starting it would have driven the shared filesystem to 100% under five concurrent agents —
the exact recorded failure mode (`tranche-7-retrospective.md` §4.1: `/home` at 100%, 0 bytes
available, "ld terminated with signal 7 [Bus error]"), and it would have taken the siblings down
with it. Overriding the floor was therefore rejected as *weakening a gate to get green*, which
"Stop vs. press on" bans outright.

### 3. Decision recorded under UNATTENDED MODE

Per UNATTENDED MODE item 3, the blocker is **recorded, not raised**; no `clarify` call was made and
nothing was fabricated. This is a resource blocker external to the card, not `decision-blocked` on a
scope question. **Remedy for the supervisor:** dispatch this card when fewer sibling worktrees are
in flight, or after their target dirs are reclaimable, or pin it to a checkout on a filesystem with
≥60G free. Nothing about the card itself is in doubt.

### 4. Re-derived figures the successor cycle can start from (step 1b, done before the gate blocked)

All three re-derived directly, commands recorded verbatim; none transcribed from
`epic-breakdown.md` or `decisions.md §38.1`.

- **Corpus-wide `companion` remaining = 1,683 across 17 books** — matches the card and
  `epic-breakdown.md`. Command:
  `python3 -c "import json;d=json.load(open('docs/work-inventory.json'));print(sum(b['kinds']['companion']['units'] for b in d['books'] if 'companion' in b['kinds']))"` → `1683`
- **Pilot book `inner_sea_combat` `companion` = 10 units, all `not-started`** — matches the card's
  "10 units". Command:
  `python3 -c "import json;d=json.load(open('docs/work-inventory.json'));print([b['kinds']['companion'] for b in d['books'] if b['id']=='inner_sea_combat'])"`
  → `[{'units': 10, 'by_status': {'not-started': 10}}]`
- **The 10 units are NOT one flat set — they are 4 chassis + 6 attached abilities, from two
  different `.lst` files.** This is the finding that answers SD29-E7-F1's acceptance criterion
  ("chassis or attribute-set, whichever the corpus's companion `.lst` shape actually supports —
  determined by this epic's own trap-report, not assumed from `race`/`race_trait`'s shape"). Command:
  `python3 -c "import json,collections;d=json.load(open('docs/work-inventory.json'));print(collections.Counter(u['source_file'] for u in d['units'] if u['kind']=='companion' and u['book']=='inner_sea_combat'))"`
  → `Counter({'isc_abilities_companion.lst': 6, 'isc_races_companion.lst': 4})`
  - **4 chassis** (`isc_races_companion.lst`, lines 5-8, `wiring_class: computed`): Companion
    (Griffon), (Hippocampus), (Hippogriff), (Worg). These are **RACE-shaped rows** —
    `SIZE:`/`MOVE:`/`BONUS:STAT|…`/`BONUS:VAR|AC_Natural_Armor|…`/`MONSTERCLASS:Companion:2`/
    `RACETYPE:Magical Beast` — verified by reading
    `~/workspace/repos/pcgen/data/pathfinder/paizo/campaign_setting/inner_sea_combat/isc_races_companion.lst`.
  - **6 abilities** (`isc_abilities_companion.lst`): 4 × `Companion Advancement ~ <beast>`
    (`computed`) + `Unable to carry a rider while flying` and `Worg ~ Mastery` (both `display`).
  - **Conclusion for the mechanism build: chassis-plus-attached-features, structurally the same
    shape as `race`/`race_trait` after all** — but that is now an *observed* conclusion with the
    corpus rows behind it, not the assumption the acceptance criterion warned against.

### 5. Architecture survey completed (no code written)

Recorded so the successor cycle does not repeat it. The companion mechanism needs, at minimum:
`src/rules_core/shape_b_v1.rs` (a `CompanionCacheData` alongside `RaceCacheData`/`RaceTraitCacheData`);
a `CompanionCorpus` loader modelled on `src/rules_core/race_resolver.rs`'s `load_race_corpus`
(chassis dir + feature dir, `link_automatic_grants` post-pass); a desktop surface modelled on
`apps/desktop/src-tauri/src/race_trait_picker.rs`; and **three** `reach_gate.rs` edits —
`CORPUS_BOOK_IDS` (add `inner_sea_combat`), `CORPUS_KIND_NAMES` (add `("companion", "companions")`),
and a `companions_reach()` arm in `reach_of`. Both `CORPUS_BOOK_IDS` and `CORPUS_KIND_NAMES` **fail
closed** — writing `data/corpus/inner_sea_combat/companion/*.json` without both edits breaks the
`reach` stage rather than silently exempting the family, so the corpus write and the gate edits must
land in the same commit.

**Standing hazard flagged for the successor, not folded in:** DoD item 4 ("the book's units leave
`not-started`") cannot be satisfied for `inner_sea_combat` by a companion ingest alone.
`v06_work_inventory.rs`'s `classify()` returns `not-started` / `no_compiled_rule_set_for_book` for
any book absent from `COMPILED_RULE_SETS` (`src/bin/v06_work_inventory.rs:899` `rule_set_for`,
`:1658`), and `inner_sea_combat` is absent. Adding a `RuleSetId` for it is the ~7-file book-onboarding
tax and would also move its 314 `class_feature` units from `not-started` to `not-ingested` — a kind
`decisions.md §38.4` excludes from every lane. That is a real scope question for the pilot to rule
on in its receipt; it is stated here as a finding, not resolved by an agent that wrote no code.

### 6. Definition of done

**Not met — and not claimed.** No item of the 8 is satisfied; `./scripts/verify.sh` (FULL) was never
run, so there is no exit code to cite and none is invented. `RUN_DESKTOP_AGENT` was never set and no
`driver.sh` call was made.

### 7. Git discipline

`git status` run before every git write. No `git add -A`, no `git stash`. **No production file,
`data/corpus/` file, generator output or baseline was touched.** The only writes are this receipt and
this actor's own retro shard. No `CARGO_TARGET_DIR` was created — that is the point of the blocker —
so there is none to clean up; `scripts/reclaim.sh --apply` was nonetheless run (exit 0).

### Retro events (`docs/retro/events/sd29-e7-companion-pilot.jsonl`)

1 × `incident`, `--recurrence-key disk-full`, `--used-percent 91`, `--actors-affected 6`. No
`verification` event: `verify.sh` FULL never ran.

## Cycle SD29-E6-F2-001 — `epic-6-race-trait-lane-extend` (Race-Trait Lane: extend) — **PARTIAL**

- **Card:** `epic-6-race-trait-lane-extend` (kanban order 10)
- **Claimed-by:** `sd29-e6-racetrait-extend` · **Claimed-at:** 2026-08-11T09:30:00Z
- **Branch:** `tranche/9` · **Branch-point:** `24462c4a` · **Worktree:** `wf_3516060a-756-12`
- **Outcome:** SPLIT. The **companion mis-classification fix is COMPLETE and committed**; the
  **corpus-wide race-trait ingest is `decision-blocked`** on a race chassis outside this bundle.
- **PR-id:** none — committed directly to `tranche/9` per the bundle's pre-authorised push rule.

### 0. Cycle-start correction: wrong worktree base

The dispatched worktree was cut from `origin/main` (`7d9f1c4f`, GE-08 era) and contained no
`docs/release/SD-29-corpus-wide-catch-up-lanes/` at all. Reset to `origin/tranche/9` on a clean tree
before any work — the same recurring dispatch-harness defect `sd29-e6-racetrait-pilot` recorded.

### 1b. Re-derived figures (every number below carries its command)

The card's scope figure — "27 books / 3,412 remaining units minus the pilot's 9" — is **wrong in
both terms**, corrected in place here and in `kanban.md`.

```
python3 -c "import json,collections; d=json.load(open('docs/work-inventory.json')); u=[x for x in d['units'] if x['kind']=='race_trait']; print(len(u), len(set(x['book'] for x in u)), collections.Counter(x['status'] for x in u))"
```

| | units | books | statuses |
|---|---|---|---|
| at `24462c4a` (pre-fix) | **3,456** | **27** | not-ingested 1,813 · not-started 1,622 · grounded 21 |
| after this cycle | **3,447** | **26** | not-ingested 1,813 · not-started 1,613 · grounded 21 |

Evidence breakdown of the 3,447 (same command, keyed on `(status, evidence)`):

```
python3 -c "import json,collections; d=json.load(open('docs/work-inventory.json')); print(collections.Counter((x['status'],x['evidence']) for x in d['units'] if x['kind']=='race_trait'))"
```

- 1,613 `not-started` / `no_compiled_rule_set_for_book`
- 864 `not-ingested` / `shared_library_record_held_by_no_ingested_host`
- **805 `not-ingested` / `race_trait_race_not_modelled`**
- **144 `not-ingested` / `race_trait_absent_from_race_traits`**
- 21 `grounded` / `race_trait_record_grounded_by_race_traits`

### 3. The bounded work that COMPLETED — companion-ability files were typed `race_trait`

`file_kind()` (`src/bin/v06_work_inventory.rs`) tests the `_abilities_race` substring **before** its
`_abilities_companion` / `_abilities_familiar` markers, so any basename containing both fell to
`Kind::RaceTrait`. Two such files exist corpus-wide:

```
python3 -c "import json,collections; d=json.load(open('docs/work-inventory.json')); c=collections.Counter(x.get('source_file') for x in d['units'] if x['kind']=='race_trait'); print([(f,n) for f,n in c.items() if 'companion' in (f or '') or 'familiar' in (f or '')])"
```
→ pre-fix `[('b4_abilities_race_ce_companion.lst', 2), ('isi_abilities_race_companion.lst', 9)]`,
post-fix `[]`.

Confirmed **at source**, not from the inventory:

```
grep -rn "CATEGORY" --include="isi_abilities_race_companion.lst" --include="b4_abilities_race_ce_companion.lst" ~/workspace/repos/pcgen/data/pathfinder
```
→ `CATEGORY:Special Ability TYPE:ClockworkSpyRacialAbility.SpecialQuality.Supernatural`,
`TYPE:ClockworkFamiliarRacialAbility…` — Clockwork Spy / Clockwork Familiar **construct-companion**
abilities, plus Bestiary 4's core-essentials companion abilities (`Comprehend Languages ~ Constant`,
`Grab ~ Medium`, …). Neither file holds a racial trait of any player race.

This is the defect `sd29-e6-racetrait-pilot` *found* and flagged (it correctly refused to build a
pilot on those 9 units) but did not fix. Fixing it is squarely inside this lane: it is the
race-trait kind's own classifier.

**TDD.** Two tests written first, in a new function-named module
`companion_ability_file_classification_tests` (no SD-NN / GE-NN tag, per the 2026-08-11 naming
directive). RED was the recorded corpus state itself — the same 11 rows appear as `race_trait` in
`docs/work-inventory.json` at `24462c4a`. Fix: inside the existing `_abilities_race` arm, a basename
also carrying `companion` or `familiar` returns `Kind::Companion`. The narrowing is provably
exhaustive — `grep -rl "" --include="*companion*.lst" --include="*familiar*.lst"` over the pcgen
tree, basenames only, shows exactly two basenames matching both patterns.

**Corpus-wide effect**, measured as an id-level set diff of `docs/work-inventory.json` at `HEAD` vs.
regenerated (not as a count difference, which would have hidden the `bestiary_4` asymmetry):

```
python3 -c "import json; old=json.load(open('/tmp/.../old-inv.json')); new=json.load(open('docs/work-inventory.json')); o={x['id'] for x in old['units'] if x['kind']=='race_trait'}; n={x['id'] for x in new['units'] if x['kind']=='race_trait'}; print(len(o-n), len(n-o))"
```
→ **9 removed, 0 added** (all nine `inner_sea_intrigue:race_trait:clockwork_*`), and on the
companion side **13 added, 0 removed**: the same 9 plus 4 `bestiary_4` rows. The asymmetry is real
and expected — `is_excluded_race_trait_row` had been dropping 4 of the b4 file's 6 rows on the
race-trait path; the companion path keeps all 6, of which 4 are new ids. Total units 38,536 →
38,540.

`inner_sea_intrigue` therefore no longer appears as a race-trait book at all (27 → 26 books), which
is why the pilot's re-pin is now a narrower question than the kanban note stated: it is needed as a
per-book ingest exemplar, not as a classifier probe.

### 3b. The half that is `decision-blocked` — no race chassis

The extend lane cannot ground a single one of the remaining 3,426 ungrounded units. Grounding a
race trait requires the record's own race to be modelled (`modelled_race_of_race_trait`, landed by
the pilot), and `race_trait_ids` comes solely from CRB's hardcoded `race_traits()` — **7 races, 49
rows**. That is what the 805 `race_trait_race_not_modelled` and 144
`race_trait_absent_from_race_traits` units are saying directly, and the remaining 2,477 sit behind
`no_compiled_rule_set_for_book` / `shared_library_record_held_by_no_ingested_host`.

Ingesting them anyway would produce records with no reach claim — precisely what DoD items 2 and 6
forbid. This is `loop-instruction.md`'s named stop: *"a record family cannot be surfaced without
work outside this bundle's epic structure … the cycle reports the gap; it does not add an epic and
it does not ingest without a reach claim."* Recorded as `decision-blocked`, not idled; a race
chassis card is the remedy.

### 6. `beastiary1/race_traits` OPEN_FINDINGS — LEFT STANDING, with the reason

DoD item 6 expects this lane's Monster Codex batch to retire it. It cannot, and the reason is
factual rather than a judgement call:

```
grep -rln "Duergar_ReplaceSLAEnlargePerson" ~/workspace/repos/pcgen/data/pathfinder --include=*.lst
```
→ `monster_codex/mc_abilities_race.lst` plus three `core_essentials/races/duergar/` files. The
gating var's only non-`core_essentials` setter is Monster Codex.

```
python3 -c "import json,collections; d=json.load(open('docs/work-inventory.json')); mc=[x for x in d['units'] if x['book']=='monster_codex']; print(len(mc), collections.Counter(x['status'] for x in mc))"
```
→ `207 Counter({'not-started': 207})`. **Monster Codex is not ingested at all.** Its ingest belongs
to Epic 5's monster lane, whose pilot card (`epic-5-monster-lane-pilot`) is still `READY` and
unclaimed. Retiring the entry from this card would have meant ingesting Monster Codex under a
race-trait card — a scope substitution, not a closure. Entry stands; deferral event emitted.

The seven `<book>/archetypes` entries were not touched (SD-30's).

### 4. Verification

`./scripts/verify.sh` FULL (never `--quick`), own `CARGO_TARGET_DIR=/tmp/codex-target-sd29-e6-racetrait-extend`
per the Epic 1 build-contention rule, exit code captured directly and never through a pipe.

**VERIFY_EXIT = 1.** The exit is determinate independent of the sweep's remaining stages:
`preflight-disk` is stage 1 and it FAILED, so `verify.sh` cannot exit `0` on this run. DoD item 1 is
therefore **NO**.

Stages reached before this cycle's turn budget expired:

| stage | result |
|---|---|
| `preflight-disk` | **FAIL** — `/` 91% used (max 90%), 45G free (min 20G, passes). Percentage floor only. |
| `pi-sweep` | PASS — 10 hits over `src/rules_core/rules_tables`, 10 baseline rows |
| `audit-selftest` | PASS — 28 passed, 0 failed (this is the stage that would catch an SD-NN/GE-NN tag in the new test module's name; it passed with the module in the tree) |
| `root-lib` | PASS — 1606 passed (includes this cycle's 2 new tests) |
| `root-full` … onwards | **did not complete** — CPU/lock-starved, not hung; still building ~490 test binaries when this cycle's turn budget expired. Independently expected RED for a reason owned by another card: `grep -n "185" tests/v06_apg_acg_feat_catalog.rs` → `:263 assert_eq!(entries_for(RuleSetId::Crb), 185);` still stands, the stale assertion `sd29-e6-racetrait-pilot` reported (measured 201) and attributed to `dde9dfc4`'s feat landing. It belongs to `epic-4-proven-feat-race-class`, which is still `IN-FLIGHT` under `sd29-e4-frc`; not touched, to avoid clobbering live work. |

The starvation was diagnosed rather than assumed, per the build-contention rule:
`ps -eo pid,etime,args | grep cargo` showed a sibling agent's `cargo test --locked --no-fail-fast
-j 2` running 19m38s while this cycle's `cargo test --locked --lib` sat at 2m22s with
`pgrep -c rustc` → **0** — blocked on the shared cargo build lock, six worktree agents deep, load
average 11.7. No stage was weakened, skipped, `#[ignore]`d, or excluded, and
`PREFLIGHT_DISK_MAX_PERCENT` was not set.

**Narrow verification that DID execute** for the change this cycle actually landed:

```
cargo test --locked --bin v06_work_inventory companion_ability_file_classification
```
→ `2 passed; 0 failed; 20 filtered out`. The change is confined to `src/bin/v06_work_inventory.rs`,
a **binary** no integration test links, plus the regenerated `docs/work-inventory.json`.

**DoD roll-call.** item 1 **NO** (exit 1, preflight-disk). item 2 **NOT REACHED** — the `reach`
stage did not run; no new reach claim was declared either, since no family was newly surfaced.
item 3 **NOT REACHED** — `v06_corpus_trap_report -- --audit` could not run under the same cargo
lock. item 4 **PARTIAL** — the generator ran and exited `0` and its output is committed, but the
second confirming run (only `generated_at` may differ) could not execute. item 5 N/A (no production
path touched — a generator's classifier, not a user-facing affordance). item 6 addressed in §6
above. item 7 N/A (§7). item 8 N/A (§8).

### 7. Baselines

Untouched. DoD item 7 N/A deliberately — the standing `verify-baselines.env` drift is Epic 9/10's
separate `--show-actuals` commit, per the standing note every other card leaves alone.

### 8. On-screen desktop verification

N/A: this cycle surfaced **no new player-visible family**. It moved 11 records between two kinds in
a generator's inventory; no new reach claim was declared, no new value reaches the sheet. `RUN_DESKTOP_AGENT`
was therefore never needed. (Had the ingest half proceeded, item 8 would have been mandatory — it is
skipped because the ingest half is blocked, not because the check was waived.)

### Retro events

DECISION: 4 emitted by this cycle (2 `correction`, 2 `deferral`), plus verify.sh's own derived
`verification` and disk-pressure `incident` events, all in
`docs/retro/events/sd29-e6-racetrait-extend.jsonl`.

### Blockers carried forward

1. **Race chassis.** 3,426 race-trait units across 26 books cannot ground until the engine models
   more than 7 races. Needs a `src/rules_core` race-chassis card; no SD-29 epic owns it.
2. **`preflight-disk` fails structurally**, third consecutive cycle (`sd29-e6-racetrait-pilot`,
   `sd29-e7-companion-pilot`, this one). `/` at 91% used, 45G free — the **percentage** floor trips,
   the 20G free-space floor passes. `scripts/reclaim.sh` (dry run) would free **0.0B**: every
   candidate is a live target dir, a worktree with unpushed commits, or a checked-out branch. Six
   concurrent worktree agents plus a 60G `target/` in the primary checkout. `PREFLIGHT_DISK_MAX_PERCENT`
   exists and was **not** used — weakening a gate to get green is banned. This is bundle-wide
   concurrency, not garbage, and it is now a recurring incident rather than an environment quirk.
3. **Epic 6 pilot re-pin** still outstanding, narrowed as described in §3.
4. **`beastiary1/race_traits`** stays in `OPEN_FINDINGS` pending Epic 5's Monster Codex ingest.
