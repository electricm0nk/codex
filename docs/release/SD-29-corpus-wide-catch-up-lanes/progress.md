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
