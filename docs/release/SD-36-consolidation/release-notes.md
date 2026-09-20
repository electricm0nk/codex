---
canonical: true
owner: operator
bundle_id: SD-36
date: 2026-09-20
---

# SD-36 Release Notes

Measured baselines (before/after, each row re-derived with its own command) and bundle summary.

**Status at time of writing:** Epics B, A, E and C1 are committed and closed. Epic C2's core
deliverable — the table-driven test rewrite (C2.1/C2.2) — has not started
(`tests/sd18_widening/rows.rs` does not exist); C2.3 (path-helper consolidation), C2.4 (branch-
promotion test move) and C2.5 (oracle tests kept and re-run against the real corpus) are done, and
C2.6's baseline drift is corrected (`scripts/verify-baselines.env`'s new C2D block). Epic D's own
D1 (architecture docs) is done; D2 (`docs/retro/sd36-retrospective.md`) is written alongside this
document; D4 (graphify), D5 (PR/merge) and D6 (worktree sweep) have not run. The "After (SD-36)"
column below is therefore **current-HEAD (`b22ea9e113`) plus this cycle's uncommitted C2D fixes,
not final-closure** — several rows (test-entry counts, `sd18_widening`/`sd13_progression` line
counts) will move again once C2.1/C2.2 land, and this table should be re-derived a second time at
that point rather than trusted as-is past this date.

---

## Build 0.16.0

**Cut date:** 2026-09-15 (`tranche/16` from `origin/develop` at `50572eebad`, SD-35's PR #390 merge).

**What changed so far:** dashboard freeze and producer retirement (Epic B), a PCGen crate wall
(Epic A), 22 SD-35 code-review correctness findings folded in as Epic E, and a source-side bloat
cut (`pilot_compute` split + path-helper consolidation, Epic C1). Not yet landed: the test-side
bloat cut (Epic C2) and bundle closure (rest of Epic D).

---

## Measured baselines

| Figure | Before (SD-35 cut, 2026-09-15) | After (current HEAD `b22ea9e113`) | Command | Change / why |
|---|---|---|---|---|
| src lines, root | 465,469 | **337,790** | `find src -name '*.rs' \| xargs cat \| wc -l` | down 127,679: Epic B deleted 55,827 lines outright; Epic A moved `src/pcgen_import`/`src/oracle_validation` (and their `#[cfg(test)]` modules) into `crates/codex-ingest`, which is why this row is NOT the whole story — see the next row |
| src lines, `crates/codex-ingest` | 0 (crate did not exist) | **81,177** | `find crates/codex-ingest/src -name '*.rs' \| xargs cat \| wc -l` | new crate; Epic A moves, not new code (`technical-design.md §2`) |
| src lines, root + ingest combined | 465,469 | **418,967** | sum of the two rows above | net reduction 46,502 lines across the whole workspace — this is the number that reflects Epic B's actual deletion, since the ingest move is a relocation, not a cut |
| `pilot_compute/mod.rs` | 88,828 | **297** | `wc -l src/rules_core/pilot_compute/mod.rs` | C1 split into 42 submodules (`ls src/rules_core/pilot_compute/*.rs \| wc -l`); largest is `prestige_class_features_campaign.rs` at 6,160 lines, under `technical-design.md §3`'s ≤6,600 target |
| tests lines, root | 182,070 | **138,095** | `find tests -name '*.rs' \| xargs cat \| wc -l` | down 43,975 so far, mostly Epic A's move of ~82 tool-test suites; Epic C2 (not yet run) is expected to cut a further ~49,000 from `sd18_widening`/`sd13_progression` alone |
| tests lines, `crates/codex-ingest/tests` | 0 | **37,085** | `find crates/codex-ingest/tests -name '*.rs' \| xargs cat \| wc -l` | new crate, Epic A moves |
| `sd18_widening` + `sd13_progression` lines | ~75,000 (design estimate) | **69,325**, unchanged | `find tests/sd18_widening tests/sd13_progression -name '*.rs' \| xargs cat \| wc -l` | Epic C2 has not landed; this row will not move until it does |
| `sd18_widening` test-list entries | claimed "2,219" in `epic-breakdown.md` C2.1/C2.2 — **does not reproduce** | **891** | `cargo test --locked --test sd18_widening -- --list \| grep -c ': test$'` | the acceptance command as literally written (`grep sd18_widening` over the combined `--list` output) returns 0, because `--list` lines never carry the binary name as a substring; corrected per-binary counts recorded in `docs/retro/sd36-retrospective.md` (retro correction `1789886083389-epic-c2-test-rewrite-6b6500`) |
| `sd13_progression` test-list entries | (same claim, same non-reproduction) | **1,136** | `cargo test --locked --test sd13_progression -- --list \| grep -c ': test$'` | see above |
| `BASELINE_ROOT_LIB_TESTS` | 3390 (cut) | **2587** | `scripts/verify-baselines.env`, SD-36 Epic C2D block (LONG RUN to re-derive: `cargo test --locked --lib -j2`) | −803 net vs. the cut; the Epic A A10 recording below (2581) undercounted by 6 — those 6 tests landed with the corpus-bundle/C1 commits after A10 was recorded and the baseline wasn't bumped until this pass's `verify.sh` run caught it (`receipts.md` C2.5/C2.6 note) |
| `BASELINE_ROOT_FULL_TESTS` | 8919/8926 (cut, two nearby recordings) | **6203** | `scripts/verify-baselines.env` (LONG RUN: `cargo test --locked --no-fail-fast -j2`) | same stale-baseline correction as the row above (+7 vs. the 6196 A10 recording); clean run at this recording (`verify2-C2D-1.log`: 50/50 stages PASS, exit 0 — the JAVA_HOME hazard the A10 note describes did not recur here) |
| `BASELINE_ROOT_TEST_BINARIES` | 419 (cut) | **285** | `scripts/verify-baselines.env`; cross-check `find tests -maxdepth 1 -name '*.rs' \| wc -l` = 279 | −134, the suite files Epic A moved out of root |
| `BASELINE_INGEST_FULL_TESTS` / `_TEST_BINARIES` | 0 / 0 (crate did not exist) | **1679 / 157** | `scripts/verify-baselines.env`, SD-36 Epic C2D block | new crate; +6 vs. the 1673 A10 first recording, same stale-baseline correction; cross-check `find crates/codex-ingest/tests -maxdepth 1 -name '*.rs' \| wc -l` = 110 |
| `BASELINE_DESKTOP_TESTS` | 570 (cut) | **612** | `scripts/verify-baselines.env`, SD-36 Epic C2D block | +42 vs. the cut; +15 vs. the 597 recorded before this pass caught the drift — Epic B (some churn), Epic E's atomic-save/path-traversal/placeholder-label regression tests, and desktop-side additions since |
| `BASELINE_FRONTEND_TEST_FILES` | 100 (cut) | **125** | `scripts/verify-baselines.env`, SD-36 Epic C2D block | +4 vs. the 121 recorded before this pass |
| public status | 95.0% of 37,892 (pre-freeze) | **100.0% of 49,450**, `partial=0`, `not_started=0` | `python3 -c "import json;print(json.load(open('site/status-data.json'))['overall'])"` | frozen per `decisions.md §4/§5`; regenerated once at closure, never updated live |
| PCGen residue, identifier patterns | (gate did not check for these) | `lst_file files=0 hits=0`, `codex_ingest files=0 hits=0` | `grep 'lst_file' scripts/pcgen_residue_gate.py`, A2's own acceptance command | Epic A code-wall class is clean |
| PCGen residue, `--check --closure` (shipped-data class) | 260 live files (SD-35 closure) | **`verdict=FAIL`, `live_files=49885 live_hits=181711`** at the last recorded run (2026-09-19) | `python3 scripts/pcgen_residue_gate.py --check --closure` | **regression, not yet closed** — `apps/desktop/src-tauri/tauri.conf.json` bundled `data/corpus/` whole in an earlier interim commit (`217f712bab`); the sanitised-bundle fix (`decisions.md §8`, commit `b1e3b2fa4a`) supersedes that bundling, but this gate has not been re-run against `b1e3b2fa4a`+ to confirm the fix closes it — see Known follow-ups |
| root-full test count immediately after Epic E's own fix cycle 2 (pre-Epic-A move) | 7855 (Epic B closure) | 7876 (Epic E fix cycle 2 closure, before Epic A's crate move) | `scripts/verify-baselines.env`, Epic E fix-cycle-2 block | +21 across Epic E's new regression tests; superseded by the Epic A move rows above, kept here for the audit trail |

---

## Epic E — SD-35 code-review correctness fixes (folded into this bundle)

Not in the original four-epic plan; the operator's 2026-09-15 ruling ("Yes, all confirmed P1s
plus the two gates") authorized folding SD-35's own code-review findings into this bundle rather
than opening a separate one. Two independent-verifier fix cycles ran against it
(`docs/release/SD-36-consolidation/receipts/epic-e_receipt.md`).

**Disposition: 15-of-22 findings fixed whole in the first pass; the two fix cycles brought the
running total to 16 fixed whole, 3 partial (a real, tested mitigation landed, with the larger
half of each genuinely a multi-cycle infrastructure item), 4 deferred with a named retro record
(CONV-06/07/08 as one group; PC4-1 — since resolved, see below).**

| Finding | Disposition | Note |
|---|---|---|
| CONV-01 `CRITRANGE` | fixed | 585 files printed the raw token-count band instead of the computed threat range; regenerated (`rules_written` 70,317 → 71,862) |
| CONV-02 multi-line label loss | fixed, 2 passes | second pass corrected 5,013 single-line records' label back to the clean assembled form |
| CONV-03 `AC_Natural_Armor` | fixed | e.g. `bestiary:monster:wolf` now carries its AC natural-armor line |
| CONV-04 `.COPY=` equipment name | fixed | bare type-word equipment labels 1,155 → 6 residual (scoped to `data/sheet_rules/*/equipment`) |
| CONV-05 per-record degradation | fixed | 423 records still carry a degradation (unchanged count — the fix changes which *lines* wipe, not how many records degrade) |
| CONV-06/07/08 (prose paraphrase, PI over-redaction, negated `PREABILITY`) | **deferred**, retro `1789638758050-sd36-epic-e-78bb64` | each changes semantics corpus-wide; needs its own RED/GREEN cycle with a full before/after diff |
| engine-P1-1/2 (Slot-over-Choice, MasterVar/MasterLevel) | fixed | render as words instead of a silent deterministic zero |
| engine-P1-3 (book tie-break) | **partial** — census-gate mitigation landed; full fix escalated | `FS-7`, unanswered `NEEDS HUMAN RULING` as of this writing (schema migration across all of `pilot_compute`, 88k+ lines) |
| engine-P1-4 (placeholder labels) | fixed, 4 call sites (2 found and fixed in the fix cycle, beyond the brief's original 2) | 1,748 occurrences / 1,488 files now resolve through `display_label` at every audited read path |
| engine-P2-1 (`SPROP` leak) | fixed | |
| desktop-P1-01 (path traversal) | fixed, 2 bypasses (the second found by the fix cycle) | permanent regression control added (source-grep for a re-typed local resolver) |
| desktop-P1-02 (non-atomic saves) | fixed, 3 stores | `saved_character`, `campaign`, `homebrew_authoring` local stores |
| desktop-P2-01 (4 orphan commands) | fixed | 3 confirmed wired with real frontend callers; 1 (`list_wizard_school_options`) dropped per the brief's own fallback |
| PC8-1/PC8-2 (id parens, size modifier) | fixed | |
| PC4-1 (Warpriest Blessing contradiction) | fixed in fix cycle 2 (was escalated COMPLEX/unresolved in the review itself) | per-choice success detection, not a group-membership check; `FS-8` marked RESOLVED |
| R12-01 (`transcribe_companion_tables.py` atomic write) | fixed | |
| GATE-01 (bookkeeping-only coverage gate) | fixed | `RULE_FILES` check + mutation probe |
| GATE-02 (oracle roster denominator) | **partial** — widened 29→31, re-run end to end | `FS-9`; remaining ~29-book stratified widening is infrastructure work (a real PCGen `BatchExporter` invocation per new character), not a doc/code change |
| GATE-03 (residue-gate token vocabulary, P3) | recorded, no change | `FS-6`, explicit P3, next code-quality pass |

---

## Desktop change set

- **Corpus root resolution fixed** (`authoring_workbench.rs`'s `codex_repo_root()`): no longer
  falls back to a compile-time `CARGO_MANIFEST_DIR` path on a packaged build with no source tree —
  the defect behind "No race could be read from the corpus" on a tester's machine.
- **Generated, PCGen-free corpus bundle** (`decisions.md §8`): `scripts/gen-corpus-bundle.mjs`
  mirrors the six `data/corpus/<book>/<kind>/` directories the live loaders actually read, trims
  each record to the fields the loader reads, and strips the residue gate's own pattern vocabulary
  from every surviving string. Ships alongside `data/sheet_rules/` (raw, already residue-free).
  **Net shipped size: 64 MB (sanitised corpus bundle) + 254 MB (`data/sheet_rules`) = 318 MB**,
  against the operator's original "~490 MB, bundle everything" figure — the 173 MB difference is
  PCGen ingest scaffolding (`raw_tokens`, `raw_bonus_chains`, unread fields) the sanitiser strips,
  not a reduction in the record population shipped. Parity is proved, not merely asserted:
  `cargo test -p codex-desktop corpus_bundle_parity` runs the same production loaders against the
  raw corpus and the bundle and asserts equal counts/rosters/diagnostics; mutation-proved this
  cycle by temporarily dropping `race`/`race_trait` from the generator's kind list and confirming
  the test names the exact missing books.
- **Two new build gates:** `scripts/verify.sh --only tauri-resources-tracked` (every
  `tauri.conf.json` resource key resolves to a git-tracked file on a clean checkout — the check
  that would have caught commit `3e1a8f8d39`'s untracked-resource defect) and
  `corpus_bundle_parity` above.
- **UI smoke harness** (`apps/desktop/scripts/ui-smoke/spec.json` + a DEV-only DOM command
  channel replacing `xdotool` coordinate clicks): 69 rows, closed at **66 green, 3 manual** (native
  OS file dialogs: import/export/portrait), **0 red/blocked/not-run**, commit `89bfbf1142`.
- **22 SD-35 code-review correctness fixes** — see the Epic E table above.

---

## Architecture-docs rewrite (D1)

`docs/architecture/README.md`'s own "Last verified" header now reads **2026-09-20 against
`tranche/16` (`b22ea9e113`, SD-36 Epic D)**. Nine existing docs updated for topics this bundle
touched (boundary, rules-engine, desktop-app, status, testing, conventions, corpus-ingest,
homebrew-and-oracle, overview, persistence, release-pipeline, rules-data-tables), one doc deleted
(`support-state-matrix.md`, retired with Epic B's own machinery), two new docs added
(`getting-started.md`, `glossary.md`). The rewrite itself is entirely uncommitted (HEAD is still
`b22ea9e113`), so the committed-range diffstat undercounts it by two full orders of magnitude — it
measures nothing of this pass. The real figure, working-tree diff against the tranche/15 cut:

```
$ git diff --stat 50572eebad -- docs/architecture/
 14 files changed, 3639 insertions(+), 2763 deletions(-)
```

That command's own file list still misses the two brand-new docs — `git diff --stat` only shows
tracked-file deltas, and `getting-started.md` / `glossary.md` are untracked (`git status
--porcelain docs/architecture/` shows both `??`), 343 and 275 lines respectively (`wc -l`). Adding
them: **16 files touched, 4,257 lines added, 2,763 removed**, net +1,494, none of it yet committed.

This is a completed, uncommitted-as-of-this-writing pass; do not edit `docs/architecture/**`
further from this bundle — link to it (as this document does) rather than duplicating its content.
Stage and commit the two new docs by name alongside the modified ones when this bundle lands
(`git add docs/architecture/getting-started.md docs/architecture/glossary.md`) — neither is
gitignored, both are simply pending their own commit like `tests/support/paths.rs` (Epic C2.3).

---

## Known follow-ups (not closed by this bundle)

1. ~~**PCGen residue gate's `--check --closure` shipped-data class is not confirmed clean.**~~
   **Closed.** Re-run 2026-09-20 against `b1e3b2fa4a`+`a68bb09eb2`+this cycle's C2D fixes:
   `python3 scripts/pcgen_residue_gate.py --check --closure` → `live_files=0 live_hits=0
   verdict=PASS` (matches `verify2-C2D-1.log`'s `pcgen-residue-gate` stage). The sanitised-bundle
   fix does close the regression; no further action needed here.
2. **Epic C2's table-driven rewrite (C2.1/C2.2) has still not started** — `tests/sd18_widening/`
   and `tests/sd13_progression/` remain 90 and 96 per-row files respectively, unchanged from the
   SD-35 cut (`git status --porcelain tests/sd18_widening tests/sd13_progression` empty; no
   `rows.rs` exists in either directory). What this cycle DID close: C2.3 (`tests/support/paths.rs`
   is now tracked, not left as a pending-commit hazard — see the D1 section's staging note), C2.4
   (branch-promotion test already moved, see `git status` `RM` line), and C2.5 (both oracle test
   sets — 21 in root `tests/`, 31 in `crates/codex-ingest/tests` — re-run once against the real
   PCGen corpus this pass, 52/52 green; see `receipts.md`'s new Epic C2 section). C2.1/C2.2's own
   acceptance command as literally written also does not reproduce (see the test-entry rows in the
   baseline table above) — correct that command before dispatching the rewrite, not after. The
   rewrite itself (~75,000 lines across 186 files into `rows.rs` + a `paste!`-driven macro,
   `technical-design.md` §3) is a multi-day, high-risk-of-silent-corruption effort on its own —
   each row's assertions encode genuinely distinct PF1e rule values per (class, level), not
   interchangeable boilerplate — and was deliberately not attempted as a rushed pass alongside the
   smaller C2.3/C2.5/C2.6/D3 fixes in this cycle. Scope it as its own dispatch.
3. **Settled-only loader** (named in the Epic D brief as a known follow-up; not otherwise detailed
   in this bundle's own package documents at time of writing — carry forward to the next STC
   scoping pass rather than left unstated here).
4. **Post-merge branch/worktree sweep** (Epic D step D6 / `workflow-instruction.md §11 step 2-3`):
   `git worktree list` currently shows only this checkout plus one unrelated
   `fix/ci-fetch-pcgen-oracle` lane, and `tranche/15` is already deleted — a smaller sweep than
   SD-35's, but still an explicit step to run and receipt at actual closure, not to assume clean
   from this snapshot.
5. **FS-7** (engine-P1-3 full book-tie-break fix) remains an open, unanswered `NEEDS HUMAN
   RULING` — see `forward-scope-register.md`.
6. **FS-9** (GATE-02's remaining ~29-book oracle-roster widening) and **FS-6** (residue-gate
   token-vocabulary widening, P3) are recorded infrastructure/code-quality deferrals for a future
   bundle.

---

## Deferred work

See `forward-scope-register.md` for the full deferral register (FS-1 through FS-9): semantic
dedups in `pilot_compute`, SD-34 correctness P1s, `rules_tables` → data-package migration,
`pf1e_dashboard_producer.py` extraction, CI oracle fetch, GATE-03 vocabulary widening, the
engine-P1-3 book-tie-break schema migration, and GATE-02's roster widening (FS-8, the Warpriest
Blessing contradiction, was resolved during this bundle and is marked so in that register, not
left listed as open).

---
