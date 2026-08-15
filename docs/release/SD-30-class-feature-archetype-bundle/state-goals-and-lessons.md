---
canonical: true
status: living — update at each SD-30 cycle closure
created: 2026-08-14
created_by: operator directive ("a file that covers the state of where we left things, our goals, and our retrospective lessons learned")
supersedes: nothing — companion to README.md and loop-instruction.md
---

# SD-30 — State, Goals, and Lessons Learned

Written at the close of the SD-29 → SD-32 → SD-30 handoff session (2026-08-13/14), immediately
before the `tranche/9` promotion PR (#360) merged. It exists so the next session does not re-derive
what this one paid for.

Everything below carries the command or commit behind it. Where a figure has no derivation, it says
so. **Re-derive before relying on any number here** — figures decay, and this program's single
largest recorded defect class is transcribed-instead-of-derived counts.

---

## 1. State: where things were left

### 1.1 The board (`done`, the product bar)

**Re-derived 2026-08-14, SD30-E0-F4-001 (epic-0 closure cycle), superseding the table below.**
Imported the dashboard producer's own `_doneness_verdict_uncapped()` (module import, not
transcription) and replayed it over `git show <ref>:docs/work-inventory.json`, corpus-wide with
`beginner_box` excluded (matching the live dashboard's own exclusion, `pf1e_dashboard_producer.py`'s
`_exclude_books_from_kind_doneness`) — cross-checked byte-for-byte against the live
`/home/ubuntu/swarm-observer/PF1e-dashboard.json` `work_inventory.by_doneness`/`by_doneness_kind`
(`generated_at` 2026-08-14T21:26:18Z), which agrees exactly:

```
python3 -c "
import json, importlib.util, collections
spec = importlib.util.spec_from_file_location('m', 'scripts/observer/pf1e_dashboard_producer.py')
mod = importlib.util.module_from_spec(spec); spec.loader.exec_module(mod)
d = json.load(open('docs/work-inventory.json'))['units']
c = collections.Counter()
for u in d:
    if u.get('book') == 'beginner_box': continue
    c[mod.doneness_verdict(u.get('wiring_class'), u.get('status'), u.get('kind'))] += 1
print(c)
"
```

| bucket | units (was, F0-close snapshot) | units (now, F4-close, re-derived) |
|---|---:|---:|
| **done** | **5,837** | **5,837** (unchanged) |
| held | 6,954 | 6,916 (−38) |
| in-progress | 848 | 848 |
| not-started | 21,319 | 20,895 (−424) |
| unmeasurable | 3,546 | 3,989 (+443) |
| deferred | 36 | 36 |
| **total** | **38,540** | **38,521** (−19: the original table did not exclude `beginner_box`'s 19 units, a bug the live producer's `_exclude_books_from_kind_doneness` already guards against for the corpus-wide total — see that function's own docstring) |

The −38/+443/−424 movement (not `done`-affecting) traces entirely to `feat`: F3's `unknown`-residue
characterization cycle re-ran the guarded `v06_work_inventory` regen as part of confirming F1's
static/derived rung, and the regenerated corpus read moved 38 `feat` units from `text-complete`/`held`
to `unknown`/`unmeasurable` (329→367 `unknown`, matching F3's own re-derived figure in `kanban.md`) —
a genuine corpus-read correction, not a classifier defect; `done` itself is untouched by it.
`retro.py correction` event `1786743412894-sd30-e0-f4-report-0f3bbc` (`docs/retro/events/sd30-e0-f4-report.jsonl`).

Per kind, `done` / total (re-derived same command, grouped by `kind`; matches the live dashboard's
`by_doneness_kind` exactly):

| kind | done | total | % |
|---|---|---|---|
| equipment_modifier | 911 | 1,580 | 57.7% |
| feat | 1,178 | 2,610 | 45.1% |
| equipment | 2,626 | 6,208 | 42.3% |
| companion | 416 | 1,696 | 24.5% |
| class | 27 | 185 | 14.6% |
| monster_ability | 334 | 3,107 | 10.7% |
| race_trait | 266 | 3,447 | 7.7% |
| spell | 47 | 2,843 | 1.7% |
| monster | 7 | 1,270 | 0.6% |
| class_feature | 25 | 15,472 | 0.2% |
| **race** | **0** | **103** | **0.0%** |

Per-kind `done`/total is unchanged from the F0-close snapshot except `equipment`'s total (6,227 →
6,208, the same `beginner_box`-exclusion correction as the corpus-wide total above — its `done`/`held`
split is unaffected). `race` at 0% is a **structural closure blocker**, now owned by
`SD-31-corpus-closure-grind` (`acceptance-and-verification.md AT-31-005`, moved from this package's
retired `AT-30-015` per `decisions.md §51`).

### 1.2 What landed in the handoff session

- **SD-29 CLOSED** (`decisions.md §70`, closure run 3). Every lane at a *measured* ceiling with the
  remainder classified, not hand-waved. Its first closure attempt was premature and the operator
  reopened it; the second refused to close with 63 workable units outstanding; the third closed
  honestly.
- **Two doneness rungs added** under operator directive "add the done rung for static and derived":
  `literal-verified` (2,322 units) and `fixture-verified` (49). Board 3,464 → 5,837.
- **SD-32 folded into SD-30.** SD-32 was created by an orchestrator without the operator asking for
  it; its content (the corpus-literal sweep, the evaluator-vs-fixture check, the spell
  consumer-delta probe, the `wiring_class` `%N` fix, the inventory determinism fix) is merged and
  stands. Its package is retired to a pointer.
- **SD-30 widened** to drive ALL kinds to closure, with SD-29's per-book ingest lanes folded in
  (`decisions.md §43`, `§44`; Epic 0 = apply-existing-instruments, Epic 10 = ingest lanes).
- **Green gate**: `./scripts/verify.sh --full` `VERIFY_EXIT=0`, 16/16 stages, at tip `94e9ed9c`.

### 1.3 Live hazards a successor inherits

1. **The regenerator silently drops 2,371 verification stamps.** A plain
   `v06_work_inventory` run does NOT reapply the corpus-literal-sweep and derived-fixture-check
   passes, so it overwrites `literal-verified`/`fixture-verified` with nothing and the diff looks
   like an ordinary refresh. **This has no guard in code.** Anyone regenerating the inventory must
   run the stamping passes or they erase the entire board gain. FIRST CANDIDATE FOR A GUARD.
2. **The anti-correlation is not fully dead.** `display`+`text-complete` reads `done` while
   `derived`/`static`+`text-complete` reads `held`, so a *correct* classifier improvement can still
   subtract from the board. Fixed for the feat/equipment path (`2ce72913`); the spell-kind path
   (`text-complete` from an unresolved corpus LEVEL) is a live residual instance.
3. **`done` is unreachable for `ambiguous`** at every status — same shape as the static/derived gap
   that the rungs fixed. Not yet addressed.
4. **The dashboard producer is not under version control** and runs from cron every 5 minutes under
   flock. Its `static`/`derived` branch RAISES on an unrecognised status: emitting a new status word
   from the generator without landing the producer rule in the SAME change crashes the dashboard
   rather than degrading it. Back it up to `/home/ubuntu/swarm-observer/.backups/` before editing.
5. **`compute_wiring_class_summary()` silently serves a stale wiring-class cache** when its mtime
   beats the source doc — it produced a false zero during measurement. Pass `cache_path` explicitly.

---

## 2. Goals

### 2.1 The charter

SD-30 owns the **full path to closure for every kind**: instrument application AND per-book ingest
(`decisions.md §43`, `§44`). It is no longer the `class_feature` bundle.

### 2.2 Ordering, by units-moved-to-`done` per unit of effort

1. **Epic 0 — apply existing instruments** to `held`. The data is already in the engine; only the
   confirming check is missing. Cheapest movement available.
2. **Epic 3 — the PI gate** (`§39`, cards `SD30-E3-F2/F3/F4`) **hard-blocks all ingest**. This is a
   LICENSING constraint, not a quality preference, and closure pressure does not relax it. It
   matters more under the widened charter, not less, because SD-30 now writes far more records.
3. **Epic 10 — ingest lanes** (F1 monster, F2 spell, F3 race, F4 race_trait), each gated behind the
   PI screen per book.

### 2.2.1 Pointer (2026-08-14, `decisions.md §49`) — table-sheet doneness doctrine

> **Correction/addition:** the operator ratified SD-32 `decisions.md §2`'s open `static`/`derived`
> "no `done` rung" measurement gate on 2026-08-14, unblocking E5/E6, and stated the underlying
> doctrine: `done` means the character sheet exposes the end rule with a true resolved value for
> this character (parameters resolved, rule displayed) — not that the engine simulates the
> mechanism (dice-rolling, arithmetic happen at the table). See `decisions.md §49` for the verbatim
> ruling and full consequences; this section's Epic 0/ordering material is otherwise unchanged.

### 2.3 The honest ceiling

> **Correction (2026-08-14, `decisions.md §45`, operator directive, launch session):** this
> section's ~81%/100%-not-promised framing is **superseded**. The SD-30 exit bar is now 100% across
> the board on the PF1e dashboard, via capability-building (race chassis, real verdict paths,
> book onboarding), not descoping. The measurement below stands as an accurate snapshot of the
> engine as it stood on the date it was written; it is no longer accepted as the target ceiling.
> Original text preserved below per this package's standing convention.

Instruments alone cannot close this. Ingest is required for the kinds the operator cares most about
(`monster` 0.6%, `spell` 1.7%, `race` 0%, `class_feature` 0.2%). The combined bounded estimate is
**~81% (~31,328 of 38,521)**, with a floor of ~7,193 units genuinely unreachable without new engine
capability (2,894 race_trait chassis-blocked + ~3,547 unmeasurable). **100% is not promised and
should not be.** Epic 10's first cycles must re-derive the real per-kind split before planning.

---

## 3. Retrospective: lessons learned

Each of these cost real time or nearly shipped a defect. They are written as rules because that is
how they get followed.

### 3.1 Measurement

- **`grounded` is not `done`.** The program reported `grounded` (5,349) while the product board read
  `done` (3,464). Races had 7 grounded and 0 done. Report the bar the operator actually reads.
- **Raw remainder is not workload.** Split every raw count into workable vs structurally-unreachable
  BEFORE planning, with the command recorded. SD-29's race-trait lane found only ~553 of 3,447 units
  had a modeled race chassis. Its premature closure came from getting this wrong.
- **Validate a proxy where it makes its confident claim.** A classifier shipped this week "made two
  false confident claims before it made a true one," caught only by running it against four cases
  whose answers three other documents already recorded.
- **Re-derive at the point of use.** Ad-hoc commands over source data catch ~50% of all logged
  corrections — more than `verify.sh`, on-screen driving, and every repo test combined
  (`docs/retro/tranche-7-retrospective.md` §3).
- **A gate that cannot fail proves nothing.** This repo has shipped three: a bundle-tag audit
  implementing 3 of its 4 patterns; an open-handle check silently dead behind a SIGPIPE; and
  `reclaim.sh` returning silently on a missing `CACHEDIR.TAG`. Each was caught by someone running it
  against a known-answer case. Prove new instruments fail by corrupting input, before trusting a pass.

### 3.2 Honesty under pressure

- **Never move a number by lowering a bar.** The best work of the week was a cycle DECLINING an easy
  +24 because those rows' bonus type stacks on base armor and equipment modifiers have no delta
  without a host item: *"reading the chain anyway would report a number no player can see."*
- **A smaller honest number is the success condition.** The board went DOWN 26 units when the
  classifier improved, and down another 39 when the prose-magnitude contradiction was fixed. Both
  were correct.
- **Report the board's movement, not the status movement.** Three separate runs reported status
  counts as board movement and were wrong every time: +623 when the board moved +46; +18 when the tip
  was −8; +623 again. Import the producer's own `doneness_verdict()` and replay it over
  `git show <ref>:docs/work-inventory.json` at both ends.
- **Fix the root cause, not the symptom.** The tempting fix for the anti-correlation was mapping
  `static`/`derived`+`text-complete` → `done`. That would have marked thousands of never-checked
  units as done. The real defect was two divergent magnitude signals.

### 3.3 Orchestration (these are the orchestrator's own errors)

- **Concurrency is bounded by cores, not ambition.** 4 cores. Six concurrent worktree agents produced
  load 9–12, disk at 94%, one lane that never obtained a gate exit code and one that never started.
  **Two build-capable agents is the cap.**
- **An agent that backgrounds work and yields is NOT woken when it finishes.** This stalled four
  agents across two days. Dispatch briefs must say: foreground it, or poll inline in the same turn.
- **"Idle" does not mean "dead".** Twice the orchestrator read an idle notification as a stall and
  dispatched a duplicate agent onto live work. Check for a `.reclaim-claim`, a running PID, and an
  uncommitted diff before concluding an agent has stopped.
- **Never run two scope authors on the same package.** Two agents converged on `epic-breakdown.md`
  and one had to discard its draft. Serialize doc work.
- **Commit and push as you go.** Work was stranded on unmerged worktree branches or left uncommitted
  **three times**, including a real 72-record ingest that the board read as "nothing landed".
  Merged-ness is verified by CONTENT, never by a prior agent's say-so.
- **A mid-flight scope change reaches an agent that has already delegated too late to matter.** Three
  consecutive rulings missed their target this way. Dispatch fresh with a consolidated brief instead.
- **Inter-agent waits deadlock.** Two agents each waited on the other while both were idle. Replace
  message dependencies with filesystem ones: a commit is a fact that can be polled; a ping is an
  event that may never fire.

### 3.4 Closure-attempt state, 2026-08-14 (`SD30-E9-F1-001`, `epic-9-closure`)

Written at the point `epic-9-closure` first attempted to check the board for a tranche-promotion PR
and found it not ready. Recorded here because this file is the living state/hazards record, and both
findings below are new relative to §1.3.

**Card state at this cycle, verified by content (not by `kanban.md`'s own status word):**

- Epics 0, 1, 2, 3 are genuinely `COMPLETE` — each has a `progress.md` receipt and the landed symbol
  greps this package's own doctrine requires (`epic-0`: `literal-verified`/`fixture-verified` in
  `pf1e_dashboard_producer.py`; `epic-1`: identifier audit pass; `epic-3`: `pi_table_sweep.rs`,
  `NAMEISPI`/`DESCISPI` handling in `ingest_pu_classes.rs` and both transcriber scripts, the
  regression test file — all re-confirmed present at this cycle's `HEAD`).
- **Epic 7 (`epic-7-version`) is marked `COMPLETE` in `kanban.md` but is not actually gate-confirmed.**
  The version bump to `0.10.0` landed, but the first `verify.sh` run failed at the `desktop` stage on
  a stale `Cargo.lock` (fixed, `cca272e8`), and the **retry** gate — the only run since the
  `Cargo.lock` fix — was found by this cycle **still running** (PID `663386`, launched ~21:40) and,
  once it reached `frontend-test`, **failed**: `src/release/buildVersionTriple.test.ts` and
  `src/releaseChecks/buildLabelFixtureFreshness.test.ts` both FAIL, the latter with the message
  `src/testerWorkbench/loadTesterWorkbenchSurface.test.ts must carry the current tranche's build-label
  fixture "Codex 0.10.0-test"` — a fixture the version-bump cycle did not update alongside the version
  triple. **This is a real, live finding, not inferred**: read directly off
  `/tmp/codex-verify-GyoD6r/frontend-test.log` while the gate was still in flight. `epic-9-closure`
  did not fix it — the fixture lives inside `epic-7-version`'s own change surface, a different card,
  and a live gate process from that cycle was still running in the same shared checkout at the time
  (touching the same tree an in-flight process is reading is the exact hazard `AGENTS.md`'s
  "one writer per tree" rule exists to prevent). **Whoever next claims `epic-7-version` must update
  the stale build-label fixture(s) to `"Codex 0.10.0-test"`, re-run the full gate, and only then flip
  the card `COMPLETE` for real** — `kanban.md`'s current `COMPLETE` mark on that row is premature and
  should be read as `IN-FLIGHT` in substance until a green gate exists at the version-bump tip.
- **Epic 8 (`epic-8-code-review`) has not been started at all.** No `progress.md` receipt, `kanban.md`
  row still `READY`. It hard-blocks `epic-9-closure` (`kanban.md`: "gated on every other card").
- **Net: the tranche-promotion PR was NOT opened this cycle.** Two real blockers, not one: Epic 8
  unstarted, and Epic 7's own gate unconfirmed with a genuine new failure discovered while checking.
  Recorded as `decision-blocked` in `progress.md`, cycle `SD30-E9-F1-001`.

**New hazard for a successor to inherit (add to §1.3's numbered list in substance, not renumbering the
existing five to avoid breaking prior cross-references):**

6. **A version bump's fixture surface is wider than the three build-config files.** `epic-7-version`
   updated `apps/desktop/package.json`, `tauri.conf.json`, `src-tauri/Cargo.toml`, and the two
   `buildVersionTriple.test.ts` files, but missed a build-label string fixture consumed by
   `src/testerWorkbench/loadTesterWorkbenchSurface.test.ts` and asserted fresh by
   `src/releaseChecks/buildLabelFixtureFreshness.test.ts`. **A version-bump cycle's own DoD item 1
   ("`verify.sh` exits 0") is the only mechanism that actually catches this** — grepping the three
   config files for the new version string, as that cycle's own receipt did, is not sufficient
   evidence of a complete bump. Do not mark a version-bump card `COMPLETE` before the full gate — not
   just `root-full`/`root-lib` — has returned a captured exit code. **Resolved**: the re-dispatch
   cycle (`SD30-E7-F1-001`, 2026-08-14/15) fixed the fixture, polled a full gate inline to
   `VERIFY_EXIT=0` at `4630fec2`, and the card is genuinely `COMPLETE`.
7. **A citation-resolution bug can hide behind its own audit's fixture.** `epic-8-code-review`'s DoD-3
   trap-audit self-check (`corpus_traps.rs`) had a bare-basename `file_basename` bug: a fixture nested
   one level under a book subdirectory passed for the wrong reason (the citation format the fixture
   used never actually engaged the `.parent()` fallback the bug lived in). Rebuilding the fixture with
   a real, directory-prefixed citation shape (matching every real corpus citation) went RED and
   surfaced that the **identical** bug was independently live in `gen_book_cache.rs`'s own generator
   — already shipped 3 wrong `wiring_class` stamps into production `inner_sea_gods` monster data. The
   general lesson: a reviewer's own proposed fixture for a fix is not sufficient evidence the fix
   engages the real code path — rebuild it to match production shape before trusting it, and when a
   citation-resolution bug is found in one call site, grep every other call site of the same
   underlying join/basename pattern before declaring the class of bug closed (`v06_work_inventory.rs`'s
   `enumerate_file` was found to share it and is **not yet fixed** — `forward-scope-register.md`
   C1.9). Also: capture the audit's exit code directly, never through `| tail`'s own exit status — this
   cycle's own first read of `TRAP_AUDIT_EXIT` was silently wrong for exactly that reason.

### 3.5 Verification

- **Capture the exit code directly, never through a pipe, and never infer a pass.** A gate that
  reached its last stage with no visible failures had in fact returned `VERIFY_EXIT=1` on `clippy`.
  The orchestrator inferred green from artifacts and was wrong.
- **Baseline movements are their own reviewable commit** carrying `--show-actuals` output (DoD
  item 7). Folding them into a fix commit hides them.
- **DoD item 8 is not waivable.** On-screen verification is the only mechanism that reaches the
  "wired into a twin the sheet doesn't read" defect class, which has bitten this program three
  times. The harness exists: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`, unique
  `RUN_DESKTOP_AGENT` per cycle, never concurrent with `verify.sh`.

### 3.6 Closure, 2026-08-15 (`SD30-E9-F2-001`, `epic-9-closure`, second attempt)

`§3.4`'s two blockers are both resolved and re-confirmed by content at this cycle's own `HEAD`
(`44497b67e`, pre-this-cycle's-own-doc-commits): `epic-7-version`'s gate green
(`VERIFY_EXIT=0`, `4630fec2`, `progress.md` `SD30-E7-F1-001` re-dispatch) and `epic-8-code-review`
complete (`VERIFY_EXIT=0`, `fc461781a`, `progress.md` `SD30-E8-F3-001`, three real defects fixed in
bundle — see release-notes.md). `v06_corpus_trap_report -- --audit` (DoD item 3) re-derived by this
cycle independently: `TRAP_AUDIT_EXIT=0`, `259 0 mod-record` — the 177-defect finding `SD30-CARRY-001`
was dispatched against is genuinely closed, not merely claimed closed. Full accounting is in
`progress.md`'s `SD30-E9-F2-001` receipt; this section exists only to record that the two things
`§3.4` said a successor must check were in fact checked, by content, not by trusting the kanban row
or a prior receipt's own say-so — the standing discipline this whole file exists to enforce.

**No new hazard this cycle.** The five hazards in §1.3 and the two added under §3.4 (now seven total)
remain the live inheritance for `SD-31-corpus-closure-grind` and `SD-32-engine-capability-builds`, the
two packages that now carry SD-30's remaining scope forward. In particular: hazard 1 (the
inventory-regen stamp-loss guard) and hazard 4 (the dashboard producer's unrecognised-status RAISE)
apply directly to SD-31's ingest-lane cycles; hazard 7 (bare-basename citation-resolution bugs,
`v06_work_inventory.rs`'s `enumerate_file` still carrying the unconfirmed instance,
`forward-scope-register.md` C1.9) applies to whichever bundle next touches measurement.
