---
canonical: true
owner: operator
bundle_id: SD-36
date: 2026-09-20
---

# SD-36 Release Notes

Measured baselines (before/after, each row re-derived with its own command) and bundle summary.

**Status at time of writing (2026-09-28, `tranche/16` HEAD `b021509e23`):** Epics B, A, E, C1, C2
and D1 are closed, and Epic F — class completion, scoped in by the operator's 2026-09-21 ruling — is
**closed in full (F0–F7; F7 sheet-visible remainders landed 2026-09-28, merge `0325a99ab1`, fix
`b021509e23`)**: the class census measures **137** class ids, **63 of 63**
non-prestige `Computed` at every level, prestige **68 of 74** `Computed` in a carrier mix, mix panel **185
of 185**, and the desktop Create roster offers **59** classes from the engine. F7 fixed the sheet-visible
prints these figures never captured: ability scores, the Weapons tab's proficiency list, spell/ability
prose formulas, prestige/class-skill labels and Elowen's skill allocation — none of them moved a headline
figure. Still to run, in order: graphify against the final tree, the PR merge (operator), the worktree
sweep. The "Measured baselines at Epic F closure" table below is the current
one; the table after it is kept as the C2-closure record it was written as.
---

## Summary

SD-36 consolidates the tree (Epics B, A, E, C1, C2, D1) and completes class coverage (Epic F, F0–F7):
census 137 class ids, 63 of 63 non-prestige `Computed` at every level, prestige 68 of 74 in a carrier mix,
mix panel 185 of 185, desktop roster 59 classes served by the engine (figures and commands below).

## User-Visible Changes

The Create picker offers 59 classes from the engine; Level Up adds any base or prestige class and names
each refused option's reason; the sheet's weapon proficiency, caster level, class skills and hit points
come from the engine; a second starter character (Elowen Ashgrave, Human Wizard 5) is seeded. The
Abilities panel, Weapons tab and Skills panel print the engine's own numbers and labels, and spell/ability
text prints the rule's words instead of a source formula (Epic F7). See "Desktop change set" and "Epic F"
below.

## Defects Fixed

22 SD-35 code-review findings (Epic E); desktop HP `Unknown` on 5 of 59 classes, wrong Martial tier and
caster level from hand tables, missing class skills, a shaman domain line the book does not grant, and raw
choice ids in prestige requirements (Epic F6). Odd ability scores printed one low, PCGen pseudo-weapons
listed as weapons, a seeded character loading with unallocated skill points, a non-skill id in a
class-skill list, prestige requirement lines printing lowercased slugs, and spell/ability text printing raw
PCGen formula syntax or an unshown gated sentence (Epic F7). See the Epic E and Epic F sections.

## Operational Notes

ui-smoke runs against an isolated app-data root and never writes the real character store (F6d). The
public status site stays frozen (Epic B).

## Verification Evidence

`scripts/verify.sh` full PASS 51/0 at `b021509e23`
(`artifacts/epic-f/stage-f7/verify-f7-2.log`); ui-smoke F7 rows in `artifacts/ui-smoke/f7/`; per-epic
receipts in `receipts.md`.

## Known Issues

See "Known follow-ups" and `forward-scope-register.md` (FS-15: 6 of 74 prestige carrier mixes; FS-23
package row; 9 of 59 roster classes with class skills Unknown; unresolved references 6,252 by mechanism;
FS-27 (new): 4 Unchained base classes refused in a mix; FS-28 (new): 33 catalog field summaries print a
choice id).

## Update Eligibility

Set at publish by the release manifest (`tools/release/check_release_manifest.py`, repo keeps the version placeholder until then); not decided in this document.

---

## Build 0.16.0

**Cut date:** 2026-09-15 (`tranche/16` from `origin/develop` at `50572eebad`, SD-35's PR #390 merge).

**What changed so far:** dashboard freeze and producer retirement (Epic B), a PCGen crate wall
(Epic A), 22 SD-35 code-review correctness findings folded in as Epic E, a source-side bloat cut
(`pilot_compute` split + path-helper consolidation, Epic C1), and now a test-side bloat cut
(table-driven `sd18_widening`/`sd13_progression` rewrite, Epic C2.1/C2.2), and class completion across
the whole corpus (Epic F: census instrument, converter link repair and 16 more converter mechanisms,
proficiency reader, generic gate arm, prestige rule, multiclass fold for every class with a chassis,
desktop class roster and prestige level-up). Not yet landed: F5.3 and the rest of bundle closure (D4–D6).

---

## Measured baselines at Epic F closure (2026-09-26)

Before = the Epic F baseline, `tranche/16` @ `424e93e93c` (2026-09-20), unless the row says otherwise.
After = HEAD `e70a8745ed`; the test rows are the last full `scripts/verify.sh` run at merge `c2b7e03dd5`
(`artifacts/epic-f/stage-f4-f5/verify-f4-1.log`, PASS 51 stages, 0 failed; HEAD after it adds docs and
config only). Fact sheet with every command: `artifacts/epic-f/stage-f4-f5/f5-facts.md`.

| Figure | Before | After | Command |
|---|---|---|---|
| class ids, corpus-wide | 135 | **137** | `cargo run --locked -j 8 --bin class_census -- --json <path>` → `ids` (the +2 are APG Ex-Antipaladin/Ex-Inquisitor, brought in by F2a) |
| class ids `Computed` at every swept level | 42 of 135 | **63 of 63** non-prestige (63 of 137 ids; prestige is never measured alone) | same → `computed`, `non_prestige_swept` |
| prestige Blocked alone (the game rule) | not measured | **74 of 74** | same → `prestige_alone_blocked` |
| prestige `Computed` in a carrier mix | 0 of 74 | **68 of 74** (6 Blocked on an oracle save formula, FS-15) | same → `prestige_mix_computed` |
| multiclass mix panel `Computed` | not measured (185 of 185 at F0) | **185 of 185** | same → `mix_panel_computed` |
| desktop Create picker | 31 (hardcoded `CLASS_OPTIONS`) | **59** (engine roster; 4 Ex-* states census-only by ruling) | same → `roster_offered`; `cargo test --locked -j 8 --manifest-path apps/desktop/src-tauri/Cargo.toml list_class_creation_roster` |
| unresolved references | 11,925 | **6,252** (A 0, B 63, D 2,646, E 2,764, F 779) | `python3 -c "import json;print(len(json.load(open('data/sheet_rules/_defects/unresolved-references.json'))))"`; `python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/unres2.py` |
| grant-by-type defects | 613 | **24** | `python3 -c "import json;print(len(json.load(open('data/sheet_rules/_defects/grant-by-type.json'))))"` |
| undefined-variable defects | 739 | **89** (+776 informational `undeclared-in-pinned-tree`) | same over `undefined-variables.json` |
| sheet-rule records | 49,450 | **49,450** (0 refused, unmoved) | `python3 -c "import json;d=json.load(open('data/sheet_rules/_report.json'));print(d['records'],d['refused'])"` |
| rules written | 71,862 (70,317 at the cut; the 71,869 quoted in F1c notes is the pre-F1c figure) | **73,363** | same → `rules_written` |
| variable contribution tables | 5,309 | **6,211** | same → `var_tables` |
| class principals with `closure_complete` | 0 (attestation did not exist) | **136 of 189** | `python3 -c "import json,glob;fs=glob.glob('data/sheet_rules/*/class/*.json');print(sum(bool((json.load(open(f))[0]).get('closure_complete')) for f in fs),len(fs))"` |
| root lib tests | 2,587 (baseline floor) | **2,727** | `verify.sh` `root-lib` |
| root full tests / suites | 6,203 / 285 | **6,398 / 293** | `verify.sh` `root-full` |
| codex-ingest tests / suites | 1,679 / 157 | **1,764 / 166** | `verify.sh` `ingest-full` |
| desktop Rust tests | 612 | **621** | `verify.sh` `desktop` |
| frontend test files | 125 | **126** | `verify.sh` `frontend-test` |
| clippy warnings root / desktop / ingest | 0 / 0 / 0 | **0 / 0 / 0** | `verify.sh` `clippy` |
| src lines, root | 337,790 | **349,508** | `find src -name '*.rs' \| xargs cat \| wc -l` |
| src lines, `crates/codex-ingest` | 81,185 | **85,906** | `find crates/codex-ingest/src -name '*.rs' \| xargs cat \| wc -l` |
| tests lines, root | 132,264 | **134,255** | `find tests -name '*.rs' \| xargs cat \| wc -l` |
| tests lines, `crates/codex-ingest/tests` | 37,163 | **39,323** | `find crates/codex-ingest/tests -name '*.rs' \| xargs cat \| wc -l` |
| Tauri commands registered | 75 | **77** | comment-stripped count over `generate_handler![...]` in `apps/desktop/src-tauri/src/main.rs` |
| ui-smoke rows | 69 (66 green, 3 manual) | **76** (69 as before + 7 F4 rows, 7 of 7 green; regression 4 of 4) | `python3 -c "import json;print(len(json.load(open('apps/desktop/scripts/ui-smoke/spec.json'))['rows']))"`; `artifacts/ui-smoke/{final,f4}/results.json` |
| public status | 100.0% of 49,450 | **100.0% of 49,450** (frozen) | `python3 -c "import json;print(json.load(open('site/status-data.json'))['overall'])"` |

The test floors in `scripts/verify-baselines.env` still read the "Before" values; re-recording them is
F5.3.

## Measured baselines (at Epic C2 closure, 2026-09-20 — kept as recorded)

| Figure | Before (SD-35 cut, 2026-09-15) | After (HEAD `5ee77f8d85` + the then-uncommitted C2.1/C2.2 rewrite, since landed in `909942637d`) | Command | Change / why |
|---|---|---|---|---|
| src lines, root | 465,469 | **337,790** | `find src -name '*.rs' \| xargs cat \| wc -l` | down 127,679: Epic B deleted 55,827 lines outright; Epic A moved `src/pcgen_import`/`src/oracle_validation` (and their `#[cfg(test)]` modules) into `crates/codex-ingest`, which is why this row is NOT the whole story — see the next row |
| src lines, `crates/codex-ingest` | 0 (crate did not exist) | **81,177** | `find crates/codex-ingest/src -name '*.rs' \| xargs cat \| wc -l` | new crate; Epic A moves, not new code (`technical-design.md §2`) |
| src lines, root + ingest combined | 465,469 | **418,967** | sum of the two rows above | net reduction 46,502 lines across the whole workspace — this is the number that reflects Epic B's actual deletion, since the ingest move is a relocation, not a cut |
| `pilot_compute/mod.rs` | 88,828 | **297** | `wc -l src/rules_core/pilot_compute/mod.rs` | C1 split into 42 submodules (`ls src/rules_core/pilot_compute/*.rs \| wc -l`); largest is `prestige_class_features_campaign.rs` at 6,160 lines, under `technical-design.md §3`'s ≤6,600 target |
| tests lines, root | 182,070 | **132,069** | `find tests -name '*.rs' \| xargs cat \| wc -l` | down 50,001 total, of which 6,026 is this pass's C2.1/C2.2 cut (see the row below) on top of Epic A's earlier move of ~82 tool-test suites |
| tests lines, `crates/codex-ingest/tests` | 0 | **37,085** | `find crates/codex-ingest/tests -name '*.rs' \| xargs cat \| wc -l` | new crate, Epic A moves |
| `sd18_widening` + `sd13_progression` lines | 69,325 (SD-36 cut, pre-C2) | **63,299** of 69,325 (-6,026 lines, -8.7%) | `find tests/sd18_widening tests/sd13_progression -name '*.rs' \| xargs cat \| wc -l` | Epic C2.1/C2.2 landed this pass: `sd18_widening` 29,041 of 33,621 (-13.6% of that family, 182 rows converted); `sd13_progression` 34,258 of 35,704 (-4.05% of that family, 143 rows converted) — full breakdown, sabotage-gate proof, and self-audit result in `receipts.md`'s Epic C2.1/C2.2 evidence section. Smaller than the design doc's ~75,000→~26,000 estimate: only the two near-universal negative-control shapes were mechanically convertible without risking the safety rule (assertions moved, never rewritten) — see receipts.md "What stayed bespoke" |
| `sd18_widening` test-list entries | claimed "2,219" in `epic-breakdown.md` C2.1/C2.2 — **does not reproduce** | **891**, unchanged before/after the rewrite | `cargo test --locked --test sd18_widening -- --list \| grep -c ': test$'` | the acceptance command as literally written (`grep sd18_widening` over the combined `--list` output) returns 0, because `--list` lines never carry the binary name as a substring; corrected per-binary counts recorded in `docs/retro/sd36-retrospective.md` (retro correction `1789886083389-epic-c2-test-rewrite-6b6500`); `--list` diff before vs. after the row-conversion is byte-identical (`receipts.md`) |
| `sd13_progression` test-list entries | (same claim, same non-reproduction) | **1,136**, unchanged before/after the rewrite | `cargo test --locked --test sd13_progression -- --list \| grep -c ': test$'` | see above; `--list` diff also byte-identical |
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

**Landed.** The D1 rewrite is committed in `4369b61a1d` (2026-09-20, `docs(sd36,epic-d): complete
architecture docs rewrite ...`): 15 files, 4,239 insertions, 2,576 deletions (`git show --stat
4369b61a1d`), including the two new docs `getting-started.md` (343 lines) and `glossary.md`
(275 lines). `tests/support/paths.rs` (Epic C2.3) landed in `45ef7e2327`. Later passes, including
Epic F5a (`a857be8959`), edited `docs/architecture/**` again; each file's own "Last verified"
header names its current pass.

The paragraph below is the record written at C2 time, before that commit, kept as written except
that its tense is past:

> `docs/architecture/README.md`'s "Last verified" header then read **2026-09-20 against
> `tranche/16` (`b22ea9e113`, SD-36 Epic D)**. The pass updated the existing docs for topics this
> bundle touched (boundary, rules-engine, desktop-app, status, testing, conventions,
> corpus-ingest, homebrew-and-oracle, overview, persistence, release-pipeline, rules-data-tables),
> deleted one (`support-state-matrix.md`, retired with Epic B's own machinery) and added two
> (`getting-started.md`, `glossary.md`). At that writing the rewrite was uncommitted (HEAD was
> `b22ea9e113`), so it was measured as a working-tree diff against the tranche/15 cut:
>
> ```
> $ git diff --stat 50572eebad -- docs/architecture/
>  14 files changed, 3639 insertions(+), 2763 deletions(-)
> ```
>
> That diff missed the two new docs, which were untracked then (343 and 275 lines, `wc -l`).
> Adding them gave **16 files touched, 4,257 lines added, 2,763 removed**, net +1,494.

---

## Known follow-ups (not closed by this bundle)

1. ~~**PCGen residue gate's `--check --closure` shipped-data class is not confirmed clean.**~~
   **Closed.** Re-run 2026-09-20 against `b1e3b2fa4a`+`a68bb09eb2`+this cycle's C2D fixes:
   `python3 scripts/pcgen_residue_gate.py --check --closure` → `live_files=0 live_hits=0
   verdict=PASS` (matches `verify2-C2D-1.log`'s `pcgen-residue-gate` stage). The sanitised-bundle
   fix does close the regression; no further action needed here.
2. ~~**Epic C2's table-driven rewrite (C2.1/C2.2) has still not started.**~~ **Closed 2026-09-20.**
   `tests/sd18_widening/rows.rs` and `tests/sd13_progression/rows.rs` both now exist; 182
   (`sd18_widening`) and 143 (`sd13_progression`) tests expand from row + macro data (the two
   near-universal negative-control shapes) instead of hand-written bodies; `--list` output is
   byte-identical before/after in both families; both families run green (891 / 1,136 passed, 0
   failed); the three-sabotage mutation gate confirms the same test NAMES fail before and after
   (`receipts.md`'s Epic C2.1/C2.2 evidence section has the full breakdown, sabotage results, and
   self-audit outcome). This is smaller than the design doc's ~75,000→~26,000, all-tests-converted
   estimate — only the two shapes mechanically verifiable as a clean 1:1 extraction (no silently
   dropped exception cases) were converted this pass; the remaining bespoke tests in each family
   (~591 in `sd18_widening`, ~993 in `sd13_progression`) keep their original bodies, per the safety
   rule (assertions moved, never rewritten, never guessed through) — see receipts.md "What stayed
   bespoke" for the full per-shape accounting. C2.3 (`tests/support/paths.rs` tracked), C2.4
   (branch-promotion test moved), C2.5 (both oracle test sets — 21 in root `tests/`, 31 in
   `crates/codex-ingest/tests` — re-run once against the real PCGen corpus, 52/52 green) and C2.6
   (`scripts/verify-baselines.env` C2D block re-synced) were already closed in an earlier pass. All
   of C2.1–C2.6 are now done; nothing of Epic C2 remains open. (Uncommitted when this entry was
   written; the rewrite landed in `909942637d`, and `tests/support/paths.rs` in `45ef7e2327`.)
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

See `forward-scope-register.md` for the full deferral register (FS-1 through FS-23, with a mechanism
remainder table re-derived at Epic F closure): semantic
dedups in `pilot_compute`, SD-34 correctness P1s, `rules_tables` → data-package migration,
`pf1e_dashboard_producer.py` extraction, CI oracle fetch, GATE-03 vocabulary widening, the
engine-P1-3 book-tie-break schema migration, GATE-02's roster widening (FS-8, the Warpriest
Blessing contradiction, was resolved during this bundle and is marked so in that register, not
left listed as open), and Epic F's four link-mechanism forward-scope rows (FS-10 through FS-13:
mechanisms B/D/E/F of the unresolved-reference table, with the small class-closure-blocking
subsets — 5 rows of mechanism D, 15 of mechanism F — that Epic F itself closes named separately
from the corpus-wide remainder that stays deferred).

---

## Epic F — Class completion (closed 2026-09-28, F0–F7)

Scoped into this bundle by the operator's 2026-09-21 ruling (`decisions.md §11`) to close the class gaps
before PR #393 merges. Plan: `epic-f-class-completion.md`; criteria with their measured figures:
`epic-breakdown.md` "Epic F"; receipts: `receipts.md` and `artifacts/epic-f/`.

**Outcome** (`cargo run --locked -j 8 --bin class_census -- --json <path>`,
`artifacts/epic-f/stage-f4-f5/census-f5.json`): the engine covers classes across the whole corpus — every
one of the 63 non-prestige class ids of the 137 the census merges (from the Core Rulebook to Ultimate
Psionics and Ultimate Wilderness, and the CRB NPC classes) computes a full sheet at every level; every
prestige class computes in its carrier mix except 6 of 74, named; every multiclass panel mix computes.

| Batch | What it did | Status | Headline (denominator, command in `epic-breakdown.md`) |
|---|---|---|---|
| F0 | permanent census instrument, `verify.sh` stage, status table generated from it | done 2026-09-21 | baseline 42 of 135 measured |
| F1 + F1b | converter link repair (Option A) + print-path reconciliation + proficiency reader | done 2026-09-22/24 | 4,456 links closed; +4,491 edges; reader 42 of 42 static rows |
| F1c | converter fixes the reader exposed (type grants, line-scoped conditions, Unchained records, closure attestation, choice-pool options, always-held globals, variable-pool picks) | done 2026-09-24 | non-prestige 42 -> 61 of 61 |
| F2 | generic gate arm; prestige-alone game rule | done 2026-09-25 | 63 of 63 (137 ids); prestige alone 74 of 74 Blocked |
| F3 | multiclass fold for every class with a chassis; skill ranks, sub-classes, bloodline picks, helper rows | done 2026-09-25 | prestige mixes 0 -> 68 of 74; mix panel 185 of 185; 187 negative controls at status parity |
| F4 | desktop class roster from the census; single-source seeds; prestige level-up | done 2026-09-26 | roster 31 -> 59; ui-smoke 7 of 7 |
| F5 | closure deltas | done 2026-09-26 | this document, status/architecture docs, decisions §11–§14, forward-scope register, retrospective |
| F6 | desktop polish: class facts from the engine; HP source rule; feat skill bonuses from the record; Level Up blockers and requirement labels; shaman domain line; newest-printing verdict; isolated ui-smoke app data; second seed character | done 2026-09-27 (merge `509244a2b6`) | hand tables 3 -> 0; HP Unknown 5 -> 0 of 59; class skills 12 -> 50 of 59; Level Up 10 of 133 options refused with the blocker shown, 0 without; raw-id requirement lines 12 -> 0 of 286; shaman domain lines 10 -> 0 of 10; `verify.sh` PASS 51/0 |
| F7 | sheet-visible remainders (operator "defects first, merge after"): ability scores from the engine; Weapons tab lists only weapon records; spell/ability prose formulas print as words (converter step); class-skill lists hold only skills; prestige/Level-Up requirement lines print labels; Expert/Summoner/Psion canonical picks marked `(default pick)`; Elowen's skill points allocated; Magus Knowledge (religion) checked against the oracle | done 2026-09-28 (merge `0325a99ab1`, fix `b021509e23`) | Elowen Con 12 -> 13, Aldric Str 18 -> 19 (48 of 48 scores match); Weapons tab pseudo-weapons 0 of 8 characters; formula-shape residue hits 2,361 -> 0 on 1,997 files; class-skill non-skill ids 0 of 148; prestige requirement slugs 0 of 286; default-pick markers on 3 of 59 roster classes; census unchanged (137/63/74/185/59); `verify.sh` PASS 51/0 |

**Named remainder** (`forward-scope-register.md`): 6 of 74 prestige carrier mixes on an oracle save
formula (FS-15, a book-cited override path); 3 prestige classes whose proficiency answer is Unknown by
mechanism G (diabolist and rivethun_emissary still compute in their fighter carrier mix, whose
weapon union the fighter decides; exalted is one of the FS-15 six); unresolved references
6,252 by mechanism (B 63, D 2,646, E 2,764, F 779; FS-10..FS-13); FS-14, FS-18, FS-19, FS-21, FS-22, FS-23.
Desktop remainders on the 59-class roster, after F6 and F7 (`artifacts/epic-f/stage-f6/`,
`artifacts/epic-f/stage-f7/`): HP `Unknown` 0 of 59
(was 5; FS-24 closed, FS-23 sheet side closed); class skills answered for 50 of 59 (was 12 from a hand
table; the 9 ACG classes print `Class skills Unknown` — an unresolved `Class|<Class>` edge, a converter
step; FS-25 desktop side closed, re-confirmed unchanged by F7-10); weapon proficiency and caster level 59
of 59 from the engine, hand-kept desktop class tables 0 (FS-26 closed). F7's own remainder, named by
mechanism: 4 Pathfinder Unchained base classes cannot be added into a mix, only taken alone (`FS-27`, new
— the mix gate has no save source for the bespoke Unchained module yet); 33 reference-library catalog
field summaries still print a `Chosen` grant's raw id, not its label (`FS-28`, new — those describers take
no package to resolve one).
