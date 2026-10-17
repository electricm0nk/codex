---
canonical: true
owner: sd38-e7.6
purpose: SD-38 (Cyberpunk RED) retrospective, grounded in the retro log and the package's own receipts, written at E7.6.
date: 2026-10-15
bundle: SD-38
branch: tranche-18
---

# SD-38 retrospective

Cyberpunk RED went from "no such system" to a system the real app creates, improves, prints and totals
from a transcribed corpus, with the Pathfinder render and the Starfinder seed lines unchanged. Every
number below carries the command that produced it. Numbers from `retro.py summary` come from

```
python3 scripts/retro.py summary --since 2026-10-13 --json
```

whose output at writing is kept in
`docs/release/SD-38-cyberpunk-red/artifacts/epic_7/E7.6_logs/retro-summary.json` (generated
`2026-10-15T20:17:56Z`; the log keeps growing, so a later re-run reads higher).

**State at writing.** 64 of 70 `kanban.md` rows `complete`; the six that are not are E7.6 (this card)
to E7.11, the closure chain
(`cd docs/release/SD-38-cyberpunk-red && awk -F'|' '$2 ~ /^ (C|E)[0-9]/ && $5 ~ /^ complete *$/' kanban.md | awk 'END{print NR}'`
run before this card's row moved; E7.5's scan found 63 of 63 non-exempt complete and 7 waiting).
The bundle ran 2026-10-13 to 2026-10-15 on `tranche-18`: 164 commits past the cut from develop
(`git log --oneline $(git merge-base origin/develop origin/tranche-18)..origin/tranche-18 | awk 'END{print NR}'`
-> 164; `git_join.commits` in the summary agrees).

## What shipped

Epics C, E0-E6 and E7.1-E7.5 (see `progress.md` Summary and its epic reports): the system seam and corpus
(E0, E1: 53 manifest kinds, 930 records, 0 null), the engine (E2), creation (E3), the sheet (E4),
improvement (E5), the resource/packaging lane (E6), three restorable starter seeds (E7.1), packaged
resources and a clean-checkout build (E7.2), fresh-install acceptance (E7.3, E7.3a), and one widest verify
(E7.4: 53 PASS, 0 FAIL, `scripts/verify.sh --full --show-actuals -j 8`). The PF three render hashes and SF
four seed files were identical to the E0.6 base at E7.4.

## The log

`events.total` 122 (`jq .events.total retro-summary.json`): 22 corrections, 10 incidents, 21 reworks,
1 deferral (resolved), 67 verification runs, 0 invalid lines, 49 shards. Verification fail rate 0.2388
(16 of 67 runs), every failure in one of two stages: `clippy` 13, `tauri-resources-tracked` 3
(`verification.by_failing_stage`).

### 1. The brief was the most-corrected artifact, again

Of the 22 corrections, 15 name our own planning documents or a discovery commit as the wrong party
(predicate: `by_subject` key contains `.md` or `commit`; Python over `retro-summary.json`; e.g.
`epic-breakdown.md` FILES YOU OWN rows for E3.4-E3.7 and E0.4, `decisions.md` A-11/section 2/D-6,
`acceptance-and-verification.md` G-1 SF command (two entries), `progress.md` Summary rows, the E7.3
discovery commit). The dominant shape: E3.3-E3.7 each listed three files and each needed a fourth
(`apps/desktop/src/boundary/cprCreation.ts`, whose `wire()` lists every request field) plus engine files,
taken as fence exits. This is the AGENTS.md rule "FILES YOU OWN must be closed under the change it
mandates", broken five times in a row in one epic; the correction after E3.3 did not stop E3.4-E3.7.
E4.4 was briefed as TS-only and needed engine rows (a fourth adapter fixture, cargo). The last
correction (E7.5: the closure-scan row constant said 69, actual 70, because E7.3a was discovered mid-run)
stopped the closure chain until the lead cleared it, which was the right outcome: the scan did its job.

### 2. One mechanism was missing and fired three times

`sf-seed-lines-out-dir-missing` fired 3 times as incidents (E4.5, E4.MC, E7.1; `incidents.recurring`), and two
reworks record the same cause (E4.1, and the copied E2.7 `run.sh`): the SF/CPR seed-line writer tests `fs::write`
into `SF_SEED_LINES_OUT` / `CPR_SHEET_LINES_OUT` and panic `NotFound` when the directory is absent. The
workflow prompts carried a warning; per AGENTS.md rule 8 a warning is not a control. The control (the
writer tests creating their out dir, or `verify.sh` creating it) was **not built in SD-38**; this bundle
chose not to touch `sf_sheet_print.rs` outside a card that owned it. It is listed under SD-39 changes.
`recurring` in the summary also lists `fixture-narrower-than-live` (2): the E4 sheet test asserted "Other
totals is empty" over 4 adapter fixtures while all 6 live sheets showed raw ids `cpr.cash`, `cpr.ammo.<i>`.

### 3. Clippy is not in the targeted loop

13 of 16 failed verify runs failed at `clippy`. 7 of the 21 reworks name clippy (`by_cause` keys containing `clippy`, summed in Python), a lint found only after
the local commit (`collapsible_if`, `enum_variant_names`, a 6-field tuple type, a 3-tuple return, a
`Vec::contains` rewrite, a doc comment in a shipping file hitting the audit), each with the same cause:
targeted `cargo test` does not run clippy. Workflow step 6 puts clippy after the commit; the lint is
cheaper before it. Also: the clippy stage was already red on the base (`sd38-e0.1`, 67c589c5f5).

### 4. Shared-tree collision, one

`shared-tree-two-writers` (1): E0.4 and E0.2 were dispatched concurrently into the one serial tree
(both depended only on E0.1). Caught by the status-before-write rule; no work was lost. `time_lost_minutes`
48 in the summary (`incidents.time_lost_minutes`, only incidents that recorded it).

### 5. Premise errors caught by the evidence, not the brief

E4.5's own test premise (Foreign Object raises the Base Death Save Penalty) was wrong against p. 222;
E4.6 assumed `alt_fire_modes` is null when absent but 4 of 29 weapons hold the string "None" (18 null);
E5.4 cited p. 231 for a table printed on p. 230 (pNNN = printed page + 1); E1 text-layer transcription had
paraphrased rather than copied page text (rework: attempt 1 paraphrased, `pdftotext -bbox-layout` made
verbatim extraction possible, no operator ruling needed). Each was found by running the real data or the
page, which is what the TDD-with-quoted-reason rule exists for. One RED was not observed directly
(E5.3: tests written before the implementation but first run after it; `near_misses`/`caught_before`
lists 1 implementation catch).

## What worked

- **The closure scan stopped the chain.** E7.5 refused to pass on a stale constant (70 rows, constant 69).
- **A second implementation for every baseline figure** (awk + Python) found the 37-vs-39 progress
  count before it propagated (correction by sd38-e3.3).
- **PF and SF stayed byte-identical** through every card that touched a render path (E7.4 compares the
  three PF hashes and four SF seed files to E0.6).
- **Discovered cards instead of carve-outs**: E4.8 (raw ids on the live sheet, unnamed checkboxes),
  E7.3a (`unknown` placeholders in the update panel) were added and finished, not flagged.

## What did not work

- **Full `npm run ui-smoke` is not in any gate.** E4.7 finding (a): 9 of the first 40 rows were red on
  `tranche-18` from before the cut (PF create form Levels column, 13c387b7ef; equipment chips,
  afff7a621f), ≈50 PF rows stall behind it (`artifacts/epic_4/E4.7/full-suite-attribution.txt`). E7.4 did
  not run it (not in G-9) and recorded that. It is still unowned: a stale-row rewrite in `spec.json`.
- **FILES YOU OWN stayed a hand-written list** (section 1).
- **One deferral**: E1.8 verbatim prose fields on encounter (55) and beat_chart (64) records were first
  paraphrased; resolved by re-transcription (`deferrals.open` 0, `resolved` 1).

## Changes for SD-39

1. Writer tests for `SF_SEED_LINES_OUT` / `CPR_SHEET_LINES_OUT` create their out dir (or `verify.sh`
   does): a mechanism, not a prompt line.
2. Generate FILES YOU OWN by closing it: grep the wire/producer for every field the criterion adds
   (`boundary/*.ts` `wire()` for any new request field) before the brief ships.
3. Run `cargo clippy --locked --tests` on touched crates before the first local commit of any `.rs` card.
4. Give the full `npm run ui-smoke` an owner and a place in G-9, or state in the package that it is not a
   gate.
5. Re-count the board constant in the closure scan at every discovered card (`sed` it in the card that
   adds the row), so E7.5 cannot be the one to find it.
6. Never dispatch two cards into one serial tree on "depends only on" reasoning; check
   `git status --porcelain` and `git worktree list` first.

## Not covered by this retrospective

The summary reads events logged under `--since 2026-10-13`; work before that date (the SD-38 authoring
cycle) is outside it. Event counts are events someone chose to log, not a count of defects: the
`silent` incident count is 2 and no instrument measures defects nobody logged. Quality of the
transcribed corpus beyond the E1.V 112-record image sample is not re-proven here.
