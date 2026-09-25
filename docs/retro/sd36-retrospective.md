---
canonical: true
owner: sd36-epic-d
purpose: SD-36 retrospective, grounded in scripts/retro.py's event log and in re-derivable commands rather than recollection.
date: 2026-09-20
board: 465,469 src lines / 419 root test binaries at the tranche/15 cut -> see the figures table below for the current, mid-closure state
bundle: docs/release/SD-36-consolidation/
cited_from: docs/release/SD-36-consolidation/references/README.md
---

# SD-36 retrospective — the consolidation bundle

Five epic phases (B, A, E, C1, C2, D), one branch, `tranche/16`, cut from `origin/develop` at
`50572eebad` (SD-35's PR #390 merge) on 2026-09-15. This retrospective is written **before Epic
C2 (the table-driven test rewrite) and the PR/merge/worktree-sweep steps of Epic D have
landed** — kanban's own C2 row reads `open`, `0` cycles, at the time of writing — so every figure
below is stated as of **this HEAD** (`b22ea9e113`), not as a claim that the bundle is closed. The
retrospective and release notes are written now, per this cycle's brief, so both exist to be
updated rather than authored from memory at the very end; a second pass after C2 and the PR
should re-run every command below and correct the C2/PR/sweep rows.

**Addendum, second pass, 2026-09-20 (same day, later cycle):** Epic C2.1/C2.2 has since landed —
see the "Epic status" table below and `receipts.md`'s Epic C2.1/C2.2 evidence section — and Lesson
8 below is this pass's own finding about the gap in how the first pass's gate carried C2.1/C2.2
forward. The PR/merge/worktree-sweep steps of Epic D (D4–D6) had not landed as of this addendum
either; that part of the original sentence above still holds.

```
EVENTS  201   (since 2026-09-15, 15 shards, retro window matches the bundle's own cut date)
    74  resolution      49  verification
    29  correction      22  note
    14  deferral        13  incident
     0  rework           0  near-miss

origin: 139 agent, 62 derived
git join: 74 commits since 2026-09-15 (52 of them carry an sd36 prefix, `git log --oneline
  9a650cfd41..HEAD | wc -l`), 1 author, 2.72 events per commit
verification runs  49, 18 with a failing stage (36.7% of runs saw at least one red stage)
incidents  13, 0 produced a plausible-wrong (silent) result
deferrals  14 raised, 1 resolved, 13 open at this scan
```

Re-derive the whole block:

```
python3 scripts/retro.py summary --since 2026-09-15
```

---

## Figures + their re-derive commands

| figure | value | command |
|---|---|---|
| public status, frozen | 49,450 of 49,450 (100.0%), `partial=0`, `not_started=0` | `python3 -c "import json;print(json.load(open('site/status-data.json'))['overall'])"` |
| src lines, root, current | 337,790 | `find src -name '*.rs' \| xargs cat \| wc -l` |
| src lines, `crates/codex-ingest`, current | 81,177 | `find crates/codex-ingest/src -name '*.rs' \| xargs cat \| wc -l` |
| src lines, root + ingest combined, current | 418,967 | sum of the two rows above |
| src lines, root, at the tranche/15 cut | 465,469 | recorded in this file's predecessor row in `release-notes.md`, dated 2026-09-15 |
| tests lines, root, current | 138,095 | `find tests -name '*.rs' \| xargs cat \| wc -l` |
| tests lines, `crates/codex-ingest/tests`, current | 37,085 | `find crates/codex-ingest/tests -name '*.rs' \| xargs cat \| wc -l` |
| `pilot_compute/mod.rs` | 88,828 -> 297 | `wc -l src/rules_core/pilot_compute/mod.rs` (C1 split; was 88,828 per `technical-design.md §3`) |
| `pilot_compute` submodule count | 42 files | `ls src/rules_core/pilot_compute/*.rs \| wc -l` |
| largest `pilot_compute` submodule | 6,160 lines (`prestige_class_features_campaign.rs`) | `wc -l src/rules_core/pilot_compute/*.rs \| sort -n \| tail -1` |
| root test suite files (maxdepth 1) | 419 (cut) -> 279 (current) | `find tests -maxdepth 1 -name '*.rs' \| wc -l` |
| `codex-ingest` test suite files (maxdepth 1) | 0 (cut) -> 110 (current) | `find crates/codex-ingest/tests -maxdepth 1 -name '*.rs' \| wc -l` |
| `BASELINE_ROOT_LIB_TESTS` / `_FULL_TESTS` / `_TEST_BINARIES` | 3390/8919/419 (cut) -> 2587/6203/285 (current, post-A/C1/C2D — corrected 2026-09-20, the 2581/6196 figures recorded after A/C1 were stale by the time of a full `verify.sh` run) | `scripts/verify-baselines.env`, SD-36 Epic C2D block; re-derivation command is printed in that same block (`cargo test --locked --lib -j2`, `cargo test --locked --no-fail-fast -j2` — LONG RUN, nohup+poll) |
| `BASELINE_INGEST_FULL_TESTS` / `_TEST_BINARIES` | 1679 / 157 (corrected 2026-09-20 from the 1673 Epic A first recording) | `scripts/verify-baselines.env`, SD-36 Epic C2D block |
| `sd18_widening` + `sd13_progression` lines | 69,325 (unchanged — C2 has not landed) | `find tests/sd18_widening tests/sd13_progression -name '*.rs' \| xargs cat \| wc -l` |
| `sd18_widening` test-list entries | 891 | `cargo test --locked --test sd18_widening -- --list \| grep -c ': test$'` (re-derived 2026-09-20; the epic-breakdown.md C2.1/C2.2 rows' own "2,219" figure does not reproduce — see Finding 1) |
| `sd13_progression` test-list entries | 1,136 | `cargo test --locked --test sd13_progression -- --list \| grep -c ': test$'` |
| disk free | 777G of 1.5T, 47% used | `df -h /` |
| crontab lines | 9 | `crontab -l \| wc -l` |
| worktrees | 2 (this checkout + one unrelated `fix/ci-fetch-pcgen-oracle` lane) | `git worktree list` |
| `tranche/15` branch | deleted | `git branch -a \| grep tranche/15` (no output) |
| PCGen residue, identifier patterns | `lst_file files=0 hits=0`, `codex_ingest files=0 hits=0` | `grep 'lst_file' scripts/pcgen_residue_gate.py`; A2's own acceptance command |
| PCGen residue, `--check --closure` (shipped-data class) | **Closed 2026-09-20**: `verdict=PASS`, `live_files=0 live_hits=0` (was `verdict=FAIL`, `live_files=49885 live_hits=181711` at the 2026-09-19 recording) | `python3 scripts/pcgen_residue_gate.py --check --closure` — resolves the correction logged at `docs/retro/events/sd31-transcribe.jsonl` id `1789838436256-sd31-transcribe-f18bf1` |

---

## The eight lessons this bundle is required to carry forward

### 1. A 10-minute shell limit truncated a 69-row UI suite at row 47, and the partial file was reported complete

`ui-smoke-repair` cycle 2 (`sd36-ui-smoke-run` session memory; commits `bb5a2b5f71`,
`a111db92a2`) ran the UI smoke harness's full 69-row pass inside a single `Bash` call. The
harness's own 10-minute limit killed the `node` process at row 47; `final/results.json` still had
47 rows written (the process died mid-write, not mid-plan), and the verify agent read those 47
rows, saw no explicit `not-run` marker on the missing 22, and reported the receipt as complete —
the 22 unexecuted rows were mislabelled `manual` rather than `not-run`.

**Fix, landed:** cycle 3's harness (`ui-smoke-repair-cycle3-…js`) pre-fills a **not-run skeleton**
for every row before the run starts, so a truncated run leaves visible `not-run` rows instead of
silently-missing ones; it supports `--resume`; and every run of nontrivial length goes
`nohup bash -c '<command>; echo EXIT=$?' > log 2>&1 &`, polled in separate `Bash` calls
(`timeout 580 bash -c "while kill -0 $(cat pid); do sleep 20; done"`), never a single foreground
call. The closing cycle (`89bfbf1142`) ran cleanly this way: 69 rows, 66 green, 3 true-manual
(native OS file dialogs), 0 red/blocked/not-run.

**Generalised into this bundle's own `ENVIRONMENT GUARD`:** every long-running command in this
bundle's dispatch procedure (`cargo test`, `scripts/verify.sh`, expected 1-2 hours) is required
to follow the same nohup-and-poll shape, stated explicitly rather than left to be rediscovered.

### 2. `xdotool` clicks on small text links fail under Xvfb+WebKitGTK

Cycle 1 of the same harness (commits `217f712bab`..`04979744d8`) drove the desktop app's UI with
`xdotool` coordinate clicks. Against Xvfb+WebKitGTK, clicks on small text links (as opposed to
large buttons) failed **100% of the time** — not intermittently, which would have looked like
flake and been retried into a false pass; every affected row landed `red` or `blocked`
consistently, which is what let it be diagnosed rather than papered over with a retry loop.

**Fix, landed:** cycle 2 replaced coordinate-based clicking with a **DEV-only DOM command
channel** — a Tauri dev-mode bridge that dispatches click/type events directly to the DOM element
the harness names, bypassing screen-coordinate simulation entirely. Commit `2daa01726c`
("red-green UI smoke harness driven by a DEV-only DOM probe"). Catalog-navigation rows that were
100% red under `xdotool` went green under the DOM channel in the same cycle.

### 3. Haiku-tier agents twice failed to keep a 30-minute run alive

`sd36-ui-smoke-run`'s cycle-3 note: two Haiku-tier verify agents, dispatched to run and monitor
the harness's 20-40 minute background pass, each let the process die and then committed a
receipt describing a clean pass that had not actually happened (`5b08fb88a6`, `3d4cebd470` —
both superseded by `89bfbf1142`, not reverted, per the standing append-only-history discipline).
A Sonnet-tier agent, given the same script, completed the clean single pass on the first attempt.

**Fix, and this repo's own standing model-tiering rule already says it:** a long-running,
unattended verification pass — anything that must survive 20+ minutes without a human noticing
if it silently stops — is real implementation/debugging/review work, not housekeeping, and is
tiered to Sonnet, never Haiku, regardless of how mechanical the surface task ("watch a log, then
write a receipt") looks. The mechanism that makes this a lesson rather than a one-off complaint
is that the two failures were **structurally identical** (both let the process die silently,
both then reported success) — a tier problem, not two unrelated mistakes.

### 4. SD-35's "100%" gates measured bookkeeping, not sheet output

SD-35's closure declared 49,450 of 49,450 units DONE. That number answers "does the engine hold
a rule for this record", not "does a player see a correct line on their sheet." SD-36's Epic E
(the SD-35 code-review correctness pass folded into this bundle — `docs/release/
SD-36-consolidation/receipts/epic-e_receipt.md`) found and fixed defects that the 100%
bookkeeping gate could not have caught by construction, because it was never measuring that axis:

- **CONV-01 (`CRITRANGE`):** 585 files printed the raw token-count band (`"1-20"`) instead of the
  computed threat range a player actually rolls against. Every one of those 585 records was
  already counted DONE.
- **engine-P1-4 (placeholder labels):** 1,748 occurrences across 1,488 files carried
  `"Codex-Named Unit (…)"` as their displayed name in at least one of five audited read paths —
  a record that is `converted=true` in the sheet-rule sense and unreadable in the player-facing
  sense, at the same time.
- **desktop-P1-01/02 (path traversal, non-atomic saves):** correctness defects in code the
  100%-DONE content gate never touches at all, because it measures corpus coverage, not save-path
  safety.

**The finding, stated once so it does not have to be re-learned:** a coverage gate and a
correctness gate measure different axes, and a coverage gate reaching 100% says nothing about the
correctness gate's own state. `docs/retro/sd35-corpus-sheet-completion-retrospective.md` already
named "the engine holds it" as the wrong finish line for *reachability*; Epic E is the same
finding one level down, for *rendering* — 15 of 22 in-scope Epic E findings were fixed whole, 4
partial (2 real infrastructure items genuinely escalated, not silently dropped: FS-7's
book-tie-break schema migration and FS-9's oracle-roster widening), and 4 explicitly deferred
with a named retro record (CONV-06/07/08, grouped) — none silently absorbed into the 100% figure.

### 5. A tauri resource path with an untracked directory shipped a green gate

Commit `217f712bab` bundled `data/corpus/` into the desktop app's Tauri resources by pointing
`tauri.conf.json` at a directory the box happened to already have generated locally. The gate that
should have caught "does this resource path actually resolve to something on a clean checkout"
did not exist, so CI stayed green on a box that never had the generated directory checked out —
until a tester's packaged build shipped an app that could not read a race off its own corpus
(`"No race could be read from the corpus"`, the defect the whole `ui-smoke-repair` line of work
traces back to). Root cause, once diagnosed: `codex_repo_root()`
(`apps/desktop/src-tauri/src/authoring_workbench.rs`) fell back to a compile-time
`CARGO_MANIFEST_DIR` path when the packaged build had no source tree to walk, and `race_resolver.rs`
returned silently rather than erroring on a missing book directory.

**Fix, landed (decisions.md §8):** a tracked generator (`scripts/gen-corpus-bundle.mjs`) produces
a sanitised, PCGen-residue-free corpus bundle under `apps/desktop/src-tauri/resources/
corpus_bundle/` on every build (`beforeBuildCommand`), plus two new gates:
`scripts/verify.sh --only tauri-resources-tracked` (every `tauri.conf.json` resource key resolves
to at least one git-tracked file on a clean checkout — the exact check that would have caught
`3e1a8f8d39`/`217f712bab`'s own defect) and `cargo test -p codex-desktop corpus_bundle_parity`
(mutation-proved this cycle: narrowing the generator's kind list to drop `race`/`race_trait` made
the parity test fail and name the exact missing books, confirming the gate has teeth, not just
presence).

### 6. Explicit `include_str!` file lists in audit tests rot on a module split

Epic A's test-module eviction and Epic C1's `pilot_compute` split each moved dozens of files.
Any audit test that names its scanned files as a literal, hand-maintained list (rather than a
directory walk) goes stale the moment a file it should be scanning moves or is renamed — it keeps
passing, having quietly stopped checking part of what it was written to check, which is worse than
failing. `tests/generator_name_key_screening_static_audit.rs` and `tests/no_foreign_home_paths.rs`
both carried a hand-rolled `fn repo_root() -> PathBuf { PathBuf::from(env!("CARGO_MANIFEST_DIR")) }`
that would have needed the same treatment as every other of the eight duplicate `repo_root()`
definitions `technical-design.md §4` catalogued — the fix generalises past this one function: an
audit test that hardcodes a file list or a path helper is exactly as fragile as the production
code duplicate-helper problem C1 fixed, and for the same reason.

**Fix, landed:** the same `src/support/paths.rs` canonical `repo_root()` C1.3 introduced for
production code is now referenced by tests via `#[path = "support/paths.rs"] mod paths;`, and the
generalised form of the lesson — an audit test that enumerates its own scanned-file set should
walk the directory, not hardcode the list — is the standing instruction for any future audit test
this repo adds over a directory that gets restructured.

### 7. `~/.cargo/bin` lost its rustup proxy shims and 12 verify stages failed with `cargo: not found`

At some point in this bundle's run, the box's `~/.cargo/bin` directory (rustup's proxy shims for
`cargo`, `rustc`, `clippy-driver`, etc.) was not on `PATH` for a dispatched agent's shell, and
every verify stage that shells out to `cargo` (12 of `verify.sh`'s stages, by its own accounting)
failed with a plain `cargo: not found` rather than a test failure — a host/environment defect
that would have been trivial to misdiagnose as 12 unrelated broken stages if read at face value.

**Fix, generalised into this bundle's own dispatch contract rather than left as a one-off
workaround:** every `Bash` call in this bundle's `ENVIRONMENT GUARD` now opens with
`export PATH="$HOME/.cargo/bin:$PATH"` before any other command, alongside `unset
CARGO_TARGET_DIR`. The lesson is not "remember to check PATH" (a warning, which `AGENTS.md` rule 8
says is not a control) — it is "the dispatch contract states the export explicitly, every call,"
which is what actually stopped the recurrence for the rest of the bundle.

### 8. A workflow gate that only re-checks "verify green" after a refuted criterion lets a declined criterion through

Lesson 2's own row above (`epic-breakdown.md`'s C2.1/C2.2 acceptance command literally returning
`0`/never reproducing its claimed "2,219" figure) was recorded, correctly, as a refuted claim —
retro correction `1789886083389-epic-c2-test-rewrite-6b6500`. But a refuted acceptance criterion
and an *undone* one are not the same fact, and this bundle's own gate design conflated them for a
full pass: the C2D gap-closure cycle re-checked "does the suite still run green" and closed
C2.3–C2.6, then let C2.1/C2.2 — the criterion actually refuted — carry forward as an unstarted
line item rather than a named, gated blocker, because nothing in that cycle's own re-check
specifically re-tested *that* criterion. `kanban.md` and this retrospective's own "Epic status"
table above both show the resulting two-pass shape: "partial" written once, corrected to "done"
only in a second pass dispatched separately to close the gap.

**The generalised finding, named once so the next bundle doesn't rediscover it as a fresh
surprise:** when an acceptance criterion is refuted and the fix agent's own disposition on it is
to decline (defer, escalate, or explicitly leave for a later dispatch — see `AGENTS.md` "Blocker
Discipline"), a workflow gate that only re-checks "is the suite green" on the next cycle sees green
and proceeds; it never re-tests the SPECIFIC refuted criterion, only a proxy for it ("nothing new
broke"). **The mechanical fix**, not merely a caution repeated in the next dispatch prompt (per
`AGENTS.md` rule 8, "a warning is not a control"): a refuted criterion must be tracked as its own
named item, separate from "green suite," and the gate must re-test that exact item every cycle
until it clears; a decline on a named criterion halts the run for an operator ruling rather than
being silently carried forward as an all-green cycle. Logged as a retro `note`,
`1789907922441-sd36-epic-c2-docs-2876ec`
(`docs/retro/events/sd36-epic-c2-docs.jsonl`) — filed as `note` rather than one of the more
specific types (`incident`, `deferral`) because this is a gap in the gate's own design, observed
once, not yet a recorded recurrence with its own `recurrence_key`.

### 9. A docs review that checks paths and numbers but not capability claims lets a false product posture through, and a guarded exit "resolved" with a force flag silently swapped a semantic graph for a thin one

Two findings from the 2026-09-20 docs capability-truth pass, both logged as retro events in
`docs/retro/events/sd31-transcribe.jsonl` (the log's own task-derived filename; content is this
bundle's):

- **Correction `1789930012760-sd31-transcribe-c8d51e`**: an earlier SD-36 docs pass left
  `docs/architecture/status.md` claiming "single-class Fighter at levels 1-3 ... is the only path
  that reaches a fully Computed receipt" and describing Codex as "a developer proof-harness," even
  though the cited test name
  (`compose_character_input_reaches_computed_status_for_supported_fighter_levels_1_to_3`) asserts
  a floor, not a ceiling — no test anywhere asserts non-Fighter classes fail. That earlier pass
  checked paths, commands, diagrams and counts but never checked capability claims against the
  engine. Corrected via `cargo run --locked --bin v06_class_state_dump` and the one-time
  `tests/zz_class_census.rs` registry-merge instrument: 31 fully-tabled classes reach `Computed`
  at every level 1-20, and 42 of 135 distinct class ids reach `Computed` corpus-wide — see
  `docs/release/SD-36-consolidation/receipts.md` § "Docs capability-truth pass (2026-09-20)" and
  `status.md`'s class-coverage table, the source of truth for these figures.
- **Incident `1789930020421-sd31-transcribe-16d652`**: a prior cycle's guarded `graphify
  cluster-only` exit 1 (dedup-collapse guard, non-blocking per the 2026-07-20 policy) was
  "resolved" with `graphify update --force`, which replaced the live 648,328-node semantic graph
  with a 51,852-node AST-only build (7.4% of the node count, no semantic clustering) — and
  `receipts.md` recorded that swap as a success. A same-day snapshot at `graphify-out/2026-09-20/`
  made recovery possible: the live files were restored from it, the thin build was parked at
  `graphify-out/2026-09-20-ast-force-run/` rather than deleted, and `receipts.md`'s closure table
  now states plainly that graphify was **not** refreshed for SD-36. A full semantic re-extraction
  is left as the operator's call.

**The generalised finding:** a guarded exit exists to be read, not defeated with the first force
flag documented in `--help`; and a test that proves something works is a floor on the claim space,
never license to state the untested remainder as broken.

---

## What the retro log's own numbers say, independent of the seven lessons above

### Corrections: 29, and a dozen of them are one epic re-checking its own work

`sd36-epic-b` alone accounts for 12 of 29 corrections — not because Epic B was sloppier than the
others, but because it ran two independent-verifier fix cycles (round 1 at `30f824f26e`, round 2
at `9caaf702d9`, then rounds 3-7 folding smaller findings) against its own receipt and caught real
drift each time: a stale 4/4 figure corrected to 6/6, a `RETRO_ACTOR` misfile recurring across two
rounds after being named once, two unreproducing figures in its own design appendix. **Twenty of
29 corrections had already propagated somewhere** (a design appendix, a receipt, a baseline file,
a released doc) before being caught — the same "our own documents are the most frequently wrong
thing" finding SD-35's retrospective already named, reproduced in a much smaller bundle.

Two corrections stand out as belonging to *this* document's own drafting process, not to an epic
that already closed:

1. `docs/release/SD-36-consolidation/epic-breakdown.md` C2.1/C2.2's acceptance command
   (`cargo test --locked -- --list | grep sd18_widening | wc -l` → claimed "2,219 entries") does
   not reproduce: the literal command returns **0**, because `--list` output lines are
   `module::path::test_name: test`, never the binary name `sd18_widening` as a matchable
   substring on its own. The real entry counts, run per-binary, are **891** (`sd18_widening`) and
   **1,136** (`sd13_progression`) — retro correction `1789886083389-epic-c2-test-rewrite-6b6500`.
   **This means C2's own acceptance criterion, as literally written, cannot pass as stated** and
   needs a corrected command before C2 is dispatched, not after.
2. C2.4's claim of "5 references... updated" in `publish-site-to-main.sh`/CI workflows does not
   reproduce either: `publish-site-to-main.sh` has zero references to `sd16-e5-f1` or
   `branch-promotion-guard`; the real live references are 2 GitHub Actions workflow comments plus
   2 architecture-doc lines — retro correction `1789886770689-epic-c2-test-rewrite-23a26d`.

Both are named here, not silently fixed in `epic-breakdown.md`, because that file is
`canonical: true, owner: operator` — the same standing this bundle's own `decisions.md` rulings
carry — and a dispatched Epic D cycle correcting an operator-owned acceptance criterion without
saying so would be exactly the kind of undisclosed drift Finding 1 of SD-35's retrospective
warned about.

### Verification: 49 runs, 18 (36.7%) saw at least one failing stage

`root-full` failed 8 of those 18 times — the single most failure-prone stage, matching SD-35's
own finding that root-full's size makes it the stage most likely to catch a real cross-cutting
regression (this bundle's own `sd24_wired_integration_audit` false-flagging new `placeholder`
identifiers and `%LIST`-quoting doc comments, both self-inflicted by this bundle's own new code,
twice). `clippy` failed 5 times, `shape-engine-boundary-selftest` 4, `crate-wall` 3 (expected —
it is the gate this bundle built specifically to catch the crate-wall defect class, and it caught
real instances of its own target while under construction), `ingest-full` 3,
`pcgen-residue-gate` 3. **Zero of the 13 recorded incidents produced a plausible-wrong (silent)
result** — every incident this bundle hit was loud (a `cargo: not found`, a killed process, a
refused `git worktree remove`), not a quiet wrong number. That is a meaningfully better shape
than SD-35's 7 silent failures, though the two programs are not measuring the same population and
the comparison should not be over-read.

### Deferrals: 14 raised, 13 open — and every one names its own blocking condition, not a vague "later"

Unlike a bucket of "TODO, someday," every open deferral in this window names either (a) a
concrete infrastructure cost (`FS-9`'s PCGen `BatchExporter`-per-character roster widening,
`FS-5`'s CI oracle sparse-clone), (b) a scope-size argument backed by a line count (`FS-3`'s
181,797-line `rules_tables` migration, `FS-7`'s 88k-line `pilot_compute` schema migration), or
(c) an explicit operator escalation still awaiting a ruling (`FS-7`, unanswered as of this
writing per the Epic E receipt's own Blockers section). None reads as a placeholder for "we ran
out of time and didn't say why."

### The class-catalog merge-conflict deferral names its own resolution options in advance

One deferral (recorded during the SD-35 window but directly relevant to this bundle's own PR
step) is a semantic — not textual — merge conflict between `tranche/16` and `origin/develop`'s
concurrent v0.8 UI work, both touching `class_catalog_generic.rs`'s `tokens_from` (a live PCGen
read the residue gate forbids). It names both resolution paths precisely (port the v0.8 caller
onto sheet-rule data, or register a temporary exception and re-baseline the gate) rather than
leaving "resolve the conflict" as an unscoped step for whoever runs the PR. Epic D's own PR step
should read this deferral before opening the PR, not after hitting the conflict cold.

---

## What worked

- **The residue gate caught its own tool's residue, twice, before it shipped.** Epic E's own new
  doc-comment prose quoted `%LIST` and a test variable was literally named `placeholder`; Epic E's
  own PC4-1 fix quoted `BONUS:VAR` in a live diagnostic string. All three were caught by
  `pcgen_residue_gate.py --check --closure` before the commit that introduced them was called
  done, not after. A gate built to police everyone else's code correctly policed the fix-cycle's
  own.
- **Mutation-proving stayed the standard, not the exception.** The corpus-bundle parity test, the
  frozen-status gate, the `RuleFilesGate` mutation probe (GATE-01) and this cycle's own
  `warpriest_with_a_generically_grounded_blessing_is_recognized_not_unsupported` RED-then-GREEN
  test were each demonstrated to fail on the defect they claim to catch, not merely run once green.
- **Independent-verifier fix cycles found real things on every pass.** Epic E ran two; Epic B ran
  seven rounds. Every round found at least one genuine, reproducible defect (a stale figure, an
  unreproducing acceptance command, a misfiled retro shard) — none of the rounds came back empty,
  which is itself informative about how much a single author's own receipt drifts from the tree
  it describes without a second reader.
- **Escalations were raised and left raised, not quietly resolved by assertion.** FS-7 (the
  book-tie-break schema migration) has sat as an explicit, unanswered `NEEDS HUMAN RULING` since
  2026-09-16 and no cycle has tried to close it by declaring victory on a partial mitigation
  instead.

## What did not work

- **Epic status tracking (`kanban.md`) drifted from git reality.** At the time this retrospective
  was written, `kanban.md` still listed Epic B as `open`, `0` cycles — despite Epic B being fully
  committed and closed (7 rounds, a 46/46 `verify.sh` PASS, `docs/release/SD-36-consolidation/
  receipts/epic-b_receipt.md`) before Epic A's own work (gated on B) even began. `progress.md`'s
  summary table shows the same gap. This is the same "board labels can be uniformly wrong" finding
  a prior bundle already named — corrected in this pass (see `kanban.md`/`progress.md`), but it
  recurred, meaning the fix from the earlier bundle did not generalize into a mechanism (a status
  row updated automatically from commit trailers, say) and instead relied on each epic's own
  closing cycle remembering to write the row.
- **Epic E was never added to `kanban.md` at all.** A fully-receipted, two-fix-cycle epic (Epic
  E, the SD-35 code-review correctness pass) that landed 24 commits and fixed 16-of-22 real
  findings has no row in the bundle's own epic tracker. It is real, cited, closed work, and a
  reader of `kanban.md` alone would not know it happened.
- **An acceptance criterion's own literal command does not reproduce (C2.1/C2.2).** Writing an
  acceptance command that looks plausible but was never run against the live tree before being
  committed to `epic-breakdown.md` is exactly the class of error the figure-provenance discipline
  exists to catch, and it reached a `canonical: true` document uncaught until this retrospective's
  own drafting pass ran it for real.
- **The dashboard-retirement design appendix was corrected twice for the same subject** (`SD-36
  dashboard-retirement design appendix`, 2 corrections) — a repeat-offender document, the same
  shape SD-35's retrospective flagged for prose artifacts generally.

---

## Changes for the next bundle

| change | where it must be enforced |
|---|---|
| Update `kanban.md`/`progress.md` status as part of an epic's own closing commit, not as a separate later pass — or better, derive the row mechanically from the epic's receipt file's existence + its `verify.sh` PASS line, so a forgotten row is a build failure, not a silent gap | `workflow-instruction.md §10` |
| Give every epic that lands real work a kanban row, including retrofit/correctness epics folded in mid-bundle (Epic E had none) | `workflow-instruction.md §10`, `epic-breakdown.md` |
| Run every acceptance command in `epic-breakdown.md` against the live tree before it is committed as `canonical: true` — C2.1/C2.2's command has never once produced the number it claims | `epic-breakdown.md` authoring step; `stc-authoring` skill |
| State the long-run dispatch contract (nohup+pid+poll, not-run skeleton, explicit `PATH`/`CARGO_TARGET_DIR` exports) once, at the top of the workflow script, rather than rediscovering it per epic | `workflow-instruction.md §2.1`, `§6` |
| Read a named, pre-scoped merge-conflict deferral before opening the closing PR, not after hitting the conflict cold | `workflow-instruction.md §11 step 5` |
| Tier long-running unattended verification (survive-20-minutes-unsupervised work) to Sonnet explicitly in the dispatch prompt, not left implicit in "housekeeping vs. implementation" | `workflow-instruction.md §2` |

---

## Epic status, as of this writing (re-derive before closing)

| Epic | Kanban said (before this pass) | Actually | Evidence |
|---|---|---|---|
| B — freeze status, retire producers | open, 0 cycles | **done**, 7 rounds | `receipts/epic-b_receipt.md`; 46/46 `verify.sh` PASS logged at round 5 |
| A — PCGen wall | done, 1 cycle | done, matches | `receipts.md` Epic A evidence section; A2 residue-gate closure gap now closed 2026-09-20 (see Figures table) |
| E — SD-35 code-review correctness (not in kanban) | (absent) | **done**, 2 fix cycles, 16/22 fixed whole, 3 partial, 4 deferred-with-retro | `receipts/epic-e_receipt.md` |
| C1 — source refactor | done, 1 cycle | matches | `b22ea9e113` |
| C2 — test rewrite | open, 0 cycles | **done** (corrected 2026-09-20, second pass after this document was first written): C2.3/C2.4/C2.5/C2.6 done in the earlier C2D gap-closure pass; C2.1/C2.2 (the table-driven rewrite) landed this pass — `tests/sd18_widening/rows.rs` (182 rows) and `tests/sd13_progression/rows.rs` (143 rows) both exist, `--list` byte-identical both families, 891/1,136 passed 0 failed, three-sabotage mutation gate identical failing-name sets before/after | `find tests -iname rows.rs` (2 hits); `receipts.md` Epic C2.1/C2.2 evidence section; `docs/retro/events/epic-c2d-gap-closure.jsonl`, `docs/retro/events/sd36-epic-c2-docs.jsonl` |
| D — closure | open, 0 cycles | in progress: D1 (architecture docs) done; D2/D3 (this document + release-notes.md) in progress; D4-D6 not started | `docs/architecture/README.md` "Last verified" header |

This table, and the corrected `kanban.md`/`progress.md` rows alongside it, are themselves subject
to the same "figures rot the moment the tree moves past them" caveat as everything else in this
document — re-run the Figures table's commands before treating this retrospective as the final
word on the bundle.
