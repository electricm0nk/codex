---
canonical: true
owner: god-emporer
status: dispatch — Epic 7 (Closure Epilogue) is now eligible
date: 2026-07-20
canonical_branch: tranche/5
kanban_board: codex-tranche-5
companion_to: /home/ubuntu/workspace/repos/codex/docs/release/SD-22/{progress.md,epic-breakdown.md,decisions.md,corpus-source-inventory.md,risks-and-open-questions.md}
---

# SD-22 — Closure Readiness Report (Epic 9, criterion-31)

This is Epic 9's first and only cycle. Per `epic-breakdown.md`'s Epic 9 section and
`decisions.md §4`, this cycle surveys every criterion 1-30's claimed status in
`progress.md`'s status matrix against real artifact evidence in
`docs/release/SD-22/artifacts/`, self-heals mechanically-resolvable shortfalls inline,
defers judgment calls to `risks-and-open-questions.md`, and — because every
mechanically-resolvable shortfall found was self-healed in this single cycle — dispatches
Epic 7.

## 1. Methodology

1. Read `loop-instruction.md` in full (canonical procedure), `progress.md`'s status
   matrix + cycle log, `epic-breakdown.md`'s Epic 9 section (criterion 31) and every
   other epic's exact criterion wording, `decisions.md §4` (Epic 9's own decision
   record), `corpus-source-inventory.md §6` (the cycle-artifact reader's contract:
   every landed cycle must mint an artifact with `## Red-phase evidence`,
   `## Green-phase evidence`, `## Files touched`, `## Cycle metadata`), and
   `risks-and-open-questions.md` (self-healable vs. non-self-healable conditions, the
   existing "Open judgments deferred to next SD" section).
2. Enumerated the full `docs/release/SD-22/artifacts/` tree (`find ... -type f`) and
   cross-referenced every file against the criteria that claim it in `progress.md`'s
   status matrix.
3. For every artifact, mechanically checked for the four required section headers
   (`## Red-phase evidence`, `## Green-phase evidence`, `## Files touched`,
   `## Cycle metadata`) via `grep`, then spot-read full file contents for anything the
   grep-only check would miss (e.g. a section present but a companion `## kanban`
   section absent).
4. Cross-checked `receipts.md`'s per-cycle YAML blocks against `progress.md`'s cycle
   log and the artifact files, to backfill any missing metadata (kanban card IDs,
   cycle IDs, timing) from a real, already-recorded source — never invented.
5. Re-ran the underlying verification commands live wherever a criterion's evidence
   was thin (Epic 1's identifier-audit grep, Epic 2's board/branch/process checks),
   rather than trusting 2026-07-19 prose at face value.
6. Re-ran the full verification suite (`cargo test --locked`, `cargo clippy --locked
   --tests -- -D warnings`) at the end to confirm the self-heals introduced no
   regression (all self-heals were docs/CI-yaml only — no `src/` production code was
   touched).
7. For anything found suspicious-but-not-a-clean-shortfall, logged it to
   `risks-and-open-questions.md` §"Open judgments deferred to next SD" instead of
   self-healing it, per the operator-judgment-call rule.

## 2. Criteria surveyed and their evidence state

| Criteria | Epic | Artifact evidence found | Outcome |
|---|---|---|---|
| 1-2 | 1 — Identifier Cleanup | None existed as a discrete file (evidence lived only in `progress.md`'s cycle log prose) | **Self-healed**: backfilled `artifacts/epic_1_2/prelaunch_and_identifier_audit_cycle_receipt.md`, re-running the live verification commands rather than only reformatting old prose. See §3.1. |
| 3-5 | 2 — Operator Pre-Launch | Same as above | Same backfill artifact covers all three (E2.3/E2.4/E2.5); all three re-verified live. |
| 6-9 | 3 — APG ingest | `artifacts/apg/class_{alchemist,cavalier,inquisitor,oracle,summoner,witch}_cycle_receipt.md` + `spell_list_cycle_receipt.md` + `equipment_tables_cycle_receipt.md` (8 files) | Clean — all 8 files carry all four required sections. No self-heal needed. |
| 10-13 | 4 — ACG ingest | `artifacts/acg/class_{arcanist,bloodrager,brawler,hunter,investigator,shaman,skald,slayer,swashbuckler,warpriest}_cycle_receipt.md` + `spell_list_cycle_receipt.md` + `equipment_tables_cycle_receipt.md` (12 files) | **Self-healed one file**: `class_warpriest_cycle_receipt.md` had the other 3 required sections but was missing `## kanban`. See §3.2. |
| 14-17 | 5 — Bestiary 1 ingest | `artifacts/beastiary1/subset_0{1..8}_cycle_receipt.md` (8 files) | Clean — all 8 files carry all four required sections. One judgment call logged (§4.2), not a shortfall. |
| 18-21 | 6 — DM Toolkit | `artifacts/dm_toolkit/{encounters,party_cr,deterministic_tests,happy_path_integration}_cycle_receipt.md` (4 files) | Clean — all four files carry all four required sections. |
| 22-26 | 7 — Closure Epilogue | None (Epic 7 has not started) | **Not a shortfall.** `progress.md`'s own row (`E7.22-26`) correctly states "Not started (fires last)" and status `open`. Epic 7 is structurally gated behind this very criterion (31) dispatching it — there is nothing to evidence yet, and claiming otherwise would itself be the fake-completion problem `AGENTS.md` rules out. |
| 27-29 | 8 — Build Version | `artifacts/epic_8/{three_version_fields,build_label_format,release_closure_checklist}_cycle_receipt.md` (3 files) | **Self-healed all three**: each had `## Red-phase evidence`, `## Green-phase evidence`, `## Files touched` but used a top-of-file bullet list instead of a `## Cycle metadata` section, and none had a `## kanban` section. See §3.3. Also self-healed a real, previously-flagged CI drift these cycles left behind. See §3.4. |
| 30 | 8 — standing gate | Re-verified live this cycle | **Closed out this cycle**: `cargo test --locked` (154+ tests across every suite, 0 failed) and `cargo clippy --locked --tests -- -D warnings` (clean) re-run fresh at HEAD `d5db4fd` plus this cycle's own self-heal commit. Marked `complete` in `progress.md`. |

**Result: 25 of 30 criteria are `complete` with clean, verified artifact evidence
(after self-healing). 5 criteria (22-26, Epic 7) are correctly `open` and not yet
eligible to run — that is the bundle's intended structure, not a shortfall.** No
criterion is `complete` in `progress.md` without corresponding evidence after this
cycle's self-heals landed. No criterion's claimed status disagrees with its actual
evidence.

## 3. Shortfalls found and self-healed

### 3.1 Epic 1 + Epic 2 (criteria 1-5) had no discrete cycle-artifact file

The underlying verification work was real (documented in `progress.md`'s
`cycle-2026-07-19T00:00:00Z` and `cycle-2026-07-19T03:50:00Z` cycle-log entries), but
no file existed under `docs/release/SD-22/artifacts/` to evidence it — `progress.md`
prose alone is not the artifact-evidence surface `corpus-source-inventory.md §6`
describes. Self-healed by writing
`artifacts/epic_1_2/prelaunch_and_identifier_audit_cycle_receipt.md`, which **re-runs
every verification command live** (2026-07-20) rather than merely restating
2026-07-19 claims:

- E1.1: `grep -rE "sd22_|SD22_|Sd22|SD-22-[A-Z][0-9]" apps/desktop/ apps/desktop/src-tauri/ src/rules_core/` — 24 hits, all of them doc-comment citations of the bundle's own `tests/sd22_*.rs` test-file names (see Judgment-1, §4.1) — zero hits in every actual identifier-discipline leak category (Tauri commands, TS `Sd22`/`SD22_`, `data-testid`, `SD-22-Ex` comments, `t_<hex>`/`AV-PAY-N` tokens).
- E1.2: vacuous, re-confirmed (no renames were ever needed).
- E2.3: `hermes kanban boards current` → `codex-tranche-5`, 24/24 tasks done.
- E2.4: `git ls-remote origin tranche/5` matches local `HEAD` exactly (`d5db4fd`).
- E2.5: exactly one `claude` process in `ps -eo pid,etime,stat,cmd`.

### 3.2 ACG Warpriest artifact missing `## kanban` section

`artifacts/acg/class_warpriest_cycle_receipt.md` had Red-phase, Green-phase, Files
touched, and Cycle metadata sections, but no `## kanban` section (every sibling ACG
class artifact has one). Cross-referenced `receipts.md`'s matching YAML block
(`criterion: acg_warpriest`, notes field) and found the card was in fact already
minted: `t_71902daa` on `codex-tranche-5`, `status=done`, per that block's own notes
("the existing kanban card (`t_71902daa`, already minted and marked done by the
original cycle)"). Self-healed by adding the `## kanban` section with that
already-recorded card ID — not a fabricated value.

### 3.3 Epic 8's three artifacts used a bullet-list header instead of `## Cycle metadata` / `## kanban` sections

`artifacts/epic_8/{three_version_fields,build_label_format,release_closure_checklist}_cycle_receipt.md`
each opened with a `- cycle_id: ... / - criterion_section: ... / - row_or_kind: ... /
- branch_tip_before: ... / - rule_set_used: ...` bullet block directly under the H1
title, instead of a `## Cycle metadata` section per `corpus-source-inventory.md §6`'s
contract shape, and none had a `## kanban` section at all. Self-healed by adding a
proper `## Cycle metadata` section to each (reusing the already-known cycle_id,
criterion number, and `rule_set_used: n/a` values — Epic 8 is version-metadata, not
content-source ingest, so `corpus_input_path`/`RuleSetId`/`ingest_pipeline_version`
are legitimately `n/a`) and a `## kanban` section noting `no card: hermes unavailable
from cloud sandbox`, cross-referenced against `receipts.md`'s matching blocks
(`kanban_card: "no card: hermes unavailable from cloud sandbox"` for all three,
confirmed by direct grep).

### 3.4 CI workflow's build-version stamp was one tranche behind (`0.4.` instead of `0.5.`)

`artifacts/epic_8/release_closure_checklist_cycle_receipt.md`'s own
"Note: the workflow stamp line is stale, and this cycle does not fix it" section
already flagged this explicitly as "a candidate self-heal item for Epic 9's
closure-readiness eval, since it's a real, mechanically-verifiable drift (not a
judgment call)". Re-confirmed live: `.github/workflows/publish-tester-release.yml`
line 62 still read `VERSION="0.4.${GITHUB_RUN_NUMBER}"`, while the three repo version
files (`apps/desktop/package.json`, `apps/desktop/src-tauri/tauri.conf.json`,
`apps/desktop/src-tauri/Cargo.toml`) have carried `0.5.95` since criterion 27 landed.
This workflow triggers on push to `develop`/`main` only — i.e. it will fire for real
once SD-22's closure PR (Epic 7) merges — so an unfixed `0.4.` prefix would have
silently published a tranche-regressed version number on SD-22's very first tester
release. Self-healed: bumped the literal to `VERSION="0.5.${GITHUB_RUN_NUMBER}"` and
added a short `SD22-E9:` doc-comment note explaining the bump and its provenance.
Confirmed no other `.github/workflows/*.yml` file carries the stale `"0.4.` literal.
This is not a `src/` production-code change (Epic 9's own "does not change
source-code behavior" boundary in `epic-breakdown.md` is about the rules-engine
source under `src/`; this is CI/release-process configuration, the same category
Epic 8's own criteria already own) — it is exactly the worked example
`loop-instruction.md` line 94 gives for what an Epic 9 self-heal cycle is allowed to
touch: *"if Epic 9 finds Epic 8's `apps/desktop/package.json` has the wrong version,
Epic 9's self-heal cycle bumps it."*

## 4. Judgment calls deferred (not self-healed)

Logged to `risks-and-open-questions.md` §"Open judgments deferred to next SD":

### 4.1 Judgment-1 — Epic 1's grep-audit pattern flags the bundle's own approved `tests/sd22_*.rs` naming convention

24 grep hits, all of them doc-comment citations of test file names that
`corpus-source-inventory.md` itself mandates (`tests/sd22_apg_class_alchemist_resolves.rs`
etc.) — not leaked scratch/audit identifiers. Whether the criterion's exception
clause should be widened to exempt this shape, or whether the doc comments should be
reworded, is a scope-of-record call for the next SD, not a mechanical fix this cycle
should improvise. E1.1 remains `complete` — the substantive identifier-discipline
concern is genuinely zero hits.

### 4.2 Judgment-2 — Epic 5 landed 8 of a stated "default 8-12" subset range

`acceptance-and-verification.md` line 101 states the range as "default 8-12"; 8
subsets (41 monsters) meets the floor but not the ceiling. All 8 landed subsets have
clean, complete artifact evidence and criterion 17 (DM-toolkit consumption) has been
satisfied since subset 01 — this is not an evidence shortfall. Whether 8 is
sufficient for Epic 5's own closure, or whether the operator wants 1-2 more subset
cycles toward the 12-subset ceiling before Epic 7's closure PR, is an explicit
operator/orchestrator call this cycle does not make unilaterally (mirroring how the
subset-08 cycle itself declined to unilaterally declare Epic 5 "fully closed").

Neither judgment call blocks criterion-31's dispatch decision: both are logged,
neither represents a `complete`-claim-without-evidence shortfall, and both are
explicitly the kind of "technically fine, worth a human look" case
`decisions.md §4`'s self-healing boundary describes as *not* Epic 9's job to resolve
in-bundle.

## 5. Final dispatch decision

**All mechanically-resolvable shortfalls found in this cycle were self-healed in this
same cycle.** After the self-heals:

- Criteria 1-21 and 27-30 (25 of 30): `complete`, with clean, verified artifact
  evidence, cross-referenced from `progress.md`'s status matrix.
- Criteria 22-26 (5 of 30, Epic 7): correctly `open` — not yet eligible to run,
  gated behind this very dispatch. Not a shortfall.
- 2 judgment calls logged to `risks-and-open-questions.md`, neither blocking.
- Full verification suite re-run clean: `cargo test --locked` 154+ tests / 0 failed
  across every suite; `cargo clippy --locked --tests -- -D warnings` clean.

**Criterion 31 → `complete`.** Epic 9 dispatches Epic 7: `progress.md`'s status
matrix and cycle log are updated to record Epic 7 (criteria 22-26, Closure Epilogue)
as now unblocked and next-eligible. The loop's normal cycle-pickup path handles Epic
7 from its next firing.
