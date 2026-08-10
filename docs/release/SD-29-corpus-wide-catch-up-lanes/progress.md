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
