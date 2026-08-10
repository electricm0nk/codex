# Tranche/7 Retrospective — SD-27 (Advanced Race Guide + Pathfinder Unchained)

**Branch:** `tranche/7` → `tranche/7-1`, merged as PR #351 (`431354a4`).
**Window:** 2026-07-30T22:20:09Z … 2026-08-01T17:35:01Z (first and last event in the log).
**Written on:** `tranche/8`, 2026-08-01, from `docs/retro/events/*.jsonl` via `scripts/retro.py`.

This is the first retrospective in this repo written from an event log rather than from memory. That
matters, because the log's single loudest finding is that memory — specifically, figures restated from
one brief into the next without re-measurement — was this tranche's largest source of wrong claims.

---

## 0. How to reproduce every number in this document

```bash
python3 scripts/retro.py validate      # 312 event(s), all valid   (as of 2026-08-01T19:26Z)
python3 scripts/retro.py summary
python3 scripts/retro.py query --type correction --json
```

**Scope of "tranche/7":** the log now contains events from the tranche/8 retrospective passes
themselves. Every tranche/7 figure below excludes the four analysis shards
(`retro-corrections-analyst`, `deferral-mining`, `tranche8-incident-retro`,
`tranche7-retro-synthesis`), which is the filter used throughout:

```bash
python3 - <<'PY'
import json,glob,collections
NEW={'retro-corrections-analyst','deferral-mining','tranche8-incident-retro','tranche7-retro-synthesis'}
ev=[json.loads(l) for f in glob.glob('docs/retro/events/*.jsonl') for l in open(f) if l.strip()]
t7=[e for e in ev if e['actor'] not in NEW]
print(len(t7), len({e['actor'] for e in t7}), collections.Counter(e['type'] for e in t7))
PY
# -> 293 63 Counter({'correction': 115, 'deferral': 83, 'verification': 50,
#                    'incident': 34, 'rework': 4, 'near_miss': 4, 'note': 3})
```

**293 events across 63 actor shards.** Every count below is derived by a command shown at the point of
use. Where a figure could not be reproduced, the correction is stated inline and logged as a
`correction` event — including corrections to this retrospective's own analysis inputs (§7).

---

## 1. What tranche/7 delivered

Derived from git across the tranche boundary (`ae845260`, the tranche/6 merge → `431354a4`, PR #351):

| measure | value | command |
|---|---:|---|
| commits | 78 | `git rev-list --count ae845260..431354a4` |
| files changed | 5775 | `git diff --shortstat ae845260 431354a4` |
| lines | +292 587 / −32 903 | same |
| new corpus record files | 965 | `git diff --name-status … -- data/corpus \| awk '$1=="A"'` |
| new test files | 32 (30 named `sd27_*`) | `git diff --name-status … -- tests/ \| awk '$1=="A"'` |

New corpus records by book (`awk '$1=="A"{split($2,a,"/");print a[3]}' | sort | uniq -c`):
`advanced_race_guide` 638 · `pathfinder_unchained` 130 · `beastiary` 120 · `core_rulebook` 75 ·
`advanced_players_guide` 1 · `advanced_class_guide` 1.

The two in-scope books, on disk today (`find data/corpus/<book>/<kind> -name '*.json' | wc -l`):

| Advanced Race Guide | | Pathfinder Unchained | |
|---|---:|---|---:|
| equipment | 200 | class_feature | 64 |
| feat | 187 | equipment | 42 |
| race_trait | 156 | feat | 17 |
| spell | 92 | class | 4 |

`LICENSE.json` `records_processed`: ARG 635, PU 127, CRB 3400, beastiary 164 — and **absent** for
`advanced_players_guide` (641 records on disk) and `advanced_class_guide` (423). That gap is live and
routed (SD-29 §2.1).

Two other things landed that are worth naming as deliverables because they are what makes the rest of
this document possible:

- **The retrospective log itself.** `scripts/retro.py` plus 63 shards. `verify.sh` emits its own
  `verification` events, so the denominator in §5 is honest — nobody chose to record the runs that
  failed.
- **`decisions.md §29–§30`**, five architectural rulings derived at closure rather than asserted:
  the two-compute-twins trap, the third (TypeScript) twin, the reach gate's two blind spots, the
  `p.xx` placeholder-provenance rule, and the two path conventions.

The gate ended 9/9. That is the outcome. The rest of this document is the process.

---

## 2. What went wrong: the briefs were the largest single source of corrected claims

### 2.1 The count

115 corrections carry **100 distinct subject strings** over **53 distinct correcting actors**. Only 9
strings repeat, so the raw tally is a long tail by construction — agents wrote the subject as free
prose. It has to be normalised to say anything, so here is the normalisation, stated so it can be
re-run and disputed:

> **BRIEF** = the subject matches `/\bbriefs?\b|\bhandoff\b/i`, minus one false positive
> (`arg-race-traits-agent (integrity check in its handoff report)` — an agent report, not a brief).

```bash
python3 - <<'PY'
import json,glob,re
NEW={'retro-corrections-analyst','deferral-mining','tranche8-incident-retro','tranche7-retro-synthesis'}
ev=[json.loads(l) for f in glob.glob('docs/retro/events/*.jsonl') for l in open(f) if l.strip()]
cor=[e for e in ev if e['type']=='correction' and e['actor'] not in NEW]
b=[e for e in cor if re.search(r'\bbriefs?\b|\bhandoff\b',e['subject'],re.I)
                 and 'handoff report' not in e['subject'].lower()]
print(len(b), 'of', len(cor))
PY
# -> 41 of 115
```

| who was wrong | n | share |
|---|---:|---:|
| **BRIEF — the session lead's or orchestrator's own instruction artifact** | **41** | **35.7 %** |
| ARTIFACT — shipped code, doc comment, test, generated data file, baseline env | 40 | 34.8 % |
| AGENT — an agent's report or claim | 32 | 27.8 % |
| OTHER — a method or a phrase, not an actor | 2 | 1.7 % |

**The briefs were the single largest source, and by definition they were upstream of everything else.**
The 41 include `task brief` variants ×17, bare `brief` ×7, `task-brief` ×2, plus one-offs:
`tranche/7-1 closure brief`, `dispatching brief (PART A)`, `orchestrator handoff brief (tranche/7-1
verification task)`, `brief (HONESTY RULES section)`, `the four-surface-truth brief's file scope for
defect 1`, `task brief (KNOWN PRE-EXISTING list)`, and so on.

### 2.2 The brief that commissioned this retrospective got that number wrong, low, by 2.6×

The tranche/8 task brief enumerated the brief-corrections as `brief ×7 + task brief ×3 + task-brief ×2`
plus four named variants — **16**. It reached 16 by transcribing the `<-- repeated` markers out of
`retro.py summary` instead of counting the log. The real figure is 41. A second, smaller error in the
same sentence: the exact string `task brief` occurs **once**; the `×3` belongs to
`task brief (spell-level gate)`.

Both are logged (`docs/retro/events/retro-corrections-analyst.jsonl`). They are recorded here rather
than quietly fixed, because a brief that set out to measure the lead's error rate under-reported the
lead's error rate, in the lead's favour, by reading a summary instead of counting — which is the
tranche's dominant failure mode reproducing itself inside its own retrospective.

### 2.3 Brief errors were caught latest and declared least

```bash
# caught_before, BRIEF vs everything else
# BRIEF   : implementation 6 | merge 1 | release 2 | field unset 32
# non-BRIEF: implementation 7 | merge 14 | release 8 | brief 1 | already shipped 1 | unset 43
```

Only **6 of 41** brief errors were caught before implementation started. The other ~35 were acted on
before their premise was tested. And `blast_radius` — the field that records how far a wrong figure had
already travelled — was stated on **17 of 41** brief corrections versus **49 of 74** everything else.
Agents were measurably less willing to write down how far the lead's wrong number had spread.

### 2.4 Nobody re-read a brief against the repo before dispatching on it

**14 of 115 corrections are self-corrections** — an actor correcting its own earlier claim:

```bash
# [e for e in cor if re.search(r'\(self\)|\(me\)|\bmy own\b|this run|this session', e['subject'], re.I)
#                 or e['actor'].lower() in e['subject'].lower()]  -> 15, minus 1 substring FP -> 14
```

`feats (self)`, `equipmod-reach-agent (self)`, `removal-agent (self)`, `featprereq`, `desc-sweep`,
`arg-picker-agent` ×2, `pu-rogue-summoner-features` ×2, `alternate-racial-trait-agent`,
`my own first BAB survey`, `this run (verify72), on its own first pass`, `sd29-scope-and-debt (me)`,
`claude (this session, earlier report)`.

**The lead self-corrected zero times.** Every one of the 41 brief errors was found downstream, by an
agent that had already been dispatched on it. That is the process finding, and it is not about
carelessness: nothing in the tranche's shape created an occasion to re-read a brief against the repo
between writing it and dispatching on it. Agents had that occasion built in — they were required to
derive counts by command — and 14 of them used it on themselves.

### 2.5 The three propagation chains, and what each one actually teaches

**Chain 1 — "the failures are environmental." The most expensive single defect of the tranche.**

| when | who corrected what | claim → correction |
|---|---|---|
| 07-31 15:54 | `verify-agent` → **orchestrator handoff brief** | "exactly **2** pre-existing `/home/ubuntu` fixture failures; anything beyond those 2 is a real regression" → **5**, plus 1 genuine new regression "the brief's rule would have mislabelled as one of them" |
| 07-31 15:55 | `verify-agent` → resolver agent report | "all 6 are pre-existing, verified by git-stash" → *"a git-stash baseline inside a multi-commit tranche is not a pre-tranche baseline"* |
| 07-31 19:11 | `verify-agent` → **brief (HONESTY RULES)** | "gate is 8/9 green" → first full run scored **7/9**; the clippy stage was "a real, unreported regression from this tranche's own uncommitted work" |
| 07-31 19:28 | `size-modifier-agent` → **task brief (KNOWN PRE-EXISTING list)** | the corrected **5** had been copied into the next brief → **6** |
| 07-31 19:55 | `pu-class-wiring-agent` → **task brief (PU class wiring)** | the same "exactly 5" text again → **6**; "any agent trusting it would have attributed this pre-existing red to their own change" |
| 08-01 17:23 | `home-paths` → `scripts/verify-baselines.env` **and every root-full revision note since 07-31** | "environmental: this box lacks the fixtures" → *"not environmental — **49 functional defaults under tests/src/scripts hardcoded another machine's home directory**… with those fixed, root-full passes 5930/533 with NO PCGen env vars set and zero FAILED lines"* |
| 08-01 17:35 | `gate-9of9` | *"A hardcoded const named another machine's home directory; **the absent directory was the defect, not the missing environment.**"* Gate 9/9, exit 0. |

Span: **25 h 40 m 23 s** (`2026-07-31T15:54:59Z` → `2026-08-01T17:35:22Z`), at least four briefs, and
eight dated `RAISED`/`LOWERED` revisions in `scripts/verify-baselines.env`. Blast radius as recorded:
*"8 recorded baseline revisions, every SD-27 verification sweep in the tranche, and the SD-27 ARG+PU
parity gates which never executed once as a result."*

The shape is `2 → 5 → 5 copied forward → 6 → the frame itself was false and the true number is 0`. Both
intermediate corrections were *right*, and both made things worse: each one made the count more precise
on an axis that did not exist. **A plausible category absorbs corrections without being challenged, and
precision on a wrong axis reads as verification.** Nobody asked why a repo would legitimately require
`/home/ubuntu` to exist until an agent was scoped to ask exactly that.

**Chain 2 — the clippy ceiling: 5 corrections, 4 agents, to establish that the brief was right.**
`verify-agent` (77/76 root, 8/7 desktop, a real regression) → `alternate-racial-trait-agent` corrects
the brief's "desktop 7" to **17** → `sd27-reach-verify-agent` corrects that back to **exactly 7** and
separately corrects `four-surface-truth-agent`'s root **92** to **exactly 75** → `spellgate-agent`
corrects the brief to **18** → `featwire-agent` resolves it: *"7, exactly the recorded ceiling, when
counted the way `scripts/verify.sh` counts it… The 18 comes from `cargo clippy --all-targets | grep -c
^warning`, which counts cargo's per-target 'generated N warnings' summary lines and extra targets;
verify.sh excludes them by design and its own comment says so."*
`retro.py`'s own module docstring already names this exact defect — "a clippy baseline that was counting
summary lines" — and it recurred anyway. **Cost driver: the brief published a ceiling without publishing
the command that produces it.**

**Chain 3 — the PU class-feature magnitude: six values in 3 h 20 m, every step correctly verified.**
23 → 32 → 35/34 → 46 → 49 → 51 → 52. `pu-class-features` states it exactly:
*"**Both numbers are right under their own definition; only the definition was missing.**"*
The final correction is against the predicate itself, not a number: *"'Unchained Barbarian ~ Uncanny
Dodge Tracker' carries one and the predicate cannot see it."* No measurement in that chain was
dishonest. None was comparable.

### 2.6 The error classes that recurred, in the data's own words

**Provenance of this table, stated because the rest of this document insists on it:** unlike every
other figure here, this is a **hand classification** performed by the `retro-corrections-analyst`
mining pass, not a command I re-ran. Single-label, primary-cause assignment of all 115; cluster names
are drawn from wording the events use. I verified the two largest rows by reading the underlying
events (the wrong-twin cluster and the brief-dominated rows below) and the B/A/R columns are
consistent with the §2.1 totals, but **the per-row assignment is a judgement and should be treated as
one.** It is included because the *shape* is the finding; do not quote individual cell values as
measurements.



| n | class | B / A / R |
|---:|---|---|
| 15 | **wrong twin** — true of the engine, false of the shipped screen | 5 / 7 / 3 |
| 13 | stale-or-untrue doc comment / generated artifact | 1 / 1 / 11 |
| 12 | counted the wrong population | 4 / 5 / 3 |
| 11 | the measuring *command* counted the wrong lines | 3 / 7 / 1 |
| 10 | inherited a baseline or diagnosis without re-testing it | **7** / 1 / 2 |
| 10 | asserted, not derived | 5 / 4 / 1 |
| 9 | undeclared or mismatched predicate | **6** / 1 / 0 |
| 7 | misread PCGen's own token or field semantics | 1 / 2 / 4 |
| 6 | the guard or probe could not see the defect it claimed to cover | 0 / 0 / 6 |
| 5 | a deferral recorded as a decision, never re-tested | 2 / 0 / 3 |
| 5 | wrong citation (line / file / book) | 2 / 3 / 0 |
| 4 | domain-rule misread (published PF1 rules) | 3 / 0 / 1 |
| 3 | shipped path covers a subset; a silent default hides the rest | 0 / 0 / 3 |
| 2 | generalised from one sample | 0 / 0 / 2 |
| 2 | scope bound to the book in hand; the defect was general | 2 / 0 / 0 |
| 1 | process (TDD order) | 0 / 1 / 0 |

The two classes where the brief dominates are **"inherited a baseline without re-testing it"** (7 of 10)
and **"undeclared predicate"** (6 of 9). Those are the same defect at two altitudes: a figure was passed
down without the method that produced it.

The largest class overall is the **wrong twin**, and it is structural rather than clerical:

> `verify-agent-tranche7` overturning `featwire-agent`'s *"Sure and Fleet lands on computed Climb
> total"*: **"Climb stayed +5 and Acrobatics stayed +3 on a live Tiefling Fighter 1 before and after
> adding Sure and Fleet. `arg_computed_climb_bonus_from_feats` appears only in `pilot_compute.rs`, which
> the sheet's skills never reference… Same root cause as the Armor of the Pit correction: wired into the
> hardcoded twin, not the corpus twin the sheet reads."**

Three compute paths existed: `pilot_compute.rs`, `pilot_compute_corpus.rs`, and a third in TypeScript —
*"flat-footed AC exists only in TypeScript, in neither `pilot_compute.rs` nor `pilot_compute_corpus.rs`,
so the seam this tranche built cannot reach it."* Every "it's wired" claim was true of *some* path. This
is now ruled on at `decisions.md §29.1` and `§29.2`.

---

## 3. What caught things: the detection mechanisms, ranked

Classified from each correction's `verified_by` field. Multi-label; `SOLE` = the only mechanism named,
i.e. that guard alone would have caught it. The classifier is a regex over `verified_by` and is stated
here so it can be re-run:

```bash
# ad-hoc command : \b(grep|rg|awk|sed|wc -l|python3|find |sort -u|uniq|Counter|jq)\b
# repo test      : cargo test|npm (run )?test|vitest|#\[test\]|test fail|assertion
# on-screen      : driver\.sh|screenshot|Xvfb|DISPLAY|on-screen|desktop app
# probe          : throwaway|temporary (bin|harness|test)|probe\b|one-off (bin|harness)
# clean tree     : git worktree|detached|git stash|pristine|clean checkout
# verify.sh      : verify\.sh          git archaeology: git log|git show|git ls-files|git blame|git diff
```

| rank | mechanism | mentions | % of 115 | SOLE |
|---:|---|---:|---:|---:|
| 1 | **ad-hoc command over source data** (`grep`/`awk`/`python3`/`find`/`wc`) | **53** | **46 %** | **27** |
| 2 | an automated test in the repo (cargo/npm/a failing assertion) | 26 | 23 % | 12 |
| 3 | **driving the shipped app on screen** (`driver.sh` / screenshot / Xvfb) | 16 | 14 % | 9 |
| 4 | reading the cited file or corpus row directly | 12 | 10 % | 4 |
| 5 | **throwaway probe / temporary harness against the live pipeline** | 11 | 10 % | 9 |
| 6 | `scripts/verify.sh` — the 9-stage gate | 9 | 8 % | 5 |
| 7 | clean-worktree / detached-HEAD / stash re-measurement | 7 | 6 % | 1 |
| 8 | git archaeology (`git log -S`, `git show`) | 5 | 4 % | 0 |
| 9 | `cargo clippy` measured directly | 3 | 3 % | 1 |
| 10 | a repo binary (`cargo run --bin`) | 1 | 1 % | 1 |

Distribution: 69 corrections name one mechanism, 29 name two, 5 name three or more, and **12 name none
this classifier recognises** — those are the ones whose `verified_by` is prose ("read the record",
"the operator ruled"). *Two independent classifications of this field — this regex and the hand-pass in
`retro-corrections-analyst` — agree within ±2 on rank 1 and disagree on the ordering of ranks 3–5. That
disagreement is itself the tranche's own lesson (§2.5, chain 3): a different predicate, not a different
tree. Only the rank-1 result is robust enough to build a rule on.*

**What this says, in order of confidence:**

1. **The cheapest guard is by far the strongest.** A one-line `grep`/`awk`/`python3` over the source
   corpus caught 46 % of all errors and was sole-sufficient 27 times. Nothing in the toolchain competes
   on cost. Almost every wrong number in this tranche was a number nobody had counted. **The bottleneck
   was never tooling.**
2. **Tests are the guard that catches *your own* errors.** Every explicit self-correction was caught by
   an assertion the author wrote in the same pass. `feats (self)`: *"wrote 'two different feats' into
   three doc comments before a test I wrote in the same pass disproved it."* `equipmod-reach-agent
   (self)`: *"the wrong numbers were written into a test that failed immediately. Logged because they
   were **INVENTED rather than derived**."* Failing-first testing worked here as a retrospective
   instrument, not only a correctness one.
3. **Driving the app on screen is small but it is the only thing that catches the biggest class.**
   On-screen driving is the *sole* named mechanism in 9 corrections overall, and the mining pass's
   hand-classification attributes 6 of those to the wrong-twin cluster (§2.6 — a judgement, not a
   command). Whatever the exact split, the direction is unambiguous. Tests and greps
   *cannot* catch "wired into the twin the sheet doesn't read" — by construction, the test passes.
   **If one guard is adopted from this retrospective, make it this: a claim that a value reaches the
   player is not verified until it has been read off the running app.**
4. **Throwaway probes punch above their 10 %** — sole-sufficient 9 times. They exist precisely because
   no shipped test measured the thing in question.
5. **`verify.sh` is a floor, not a detector.** 8 %, sole in 5. It never caught a wrong *claim*; it
   caught regressions. And it has a failure mode it cannot self-detect — §5.

Cross-tabulated by who was wrong: brief errors were caught by **counting** and by **baseline
isolation** (clean-worktree appears against the brief 7 times, against agents twice, against artifacts
once). Agent errors were caught by **driving the app**. Artifact errors were caught by **probes and
direct file reads**.

---

## 4. Process and tooling failures: what the orchestration caused, and what agents caused

34 incidents. **First, a defect in the reader:** `retro.py summary` reports the tranche's dominant
incident mode as `RECURRING x2 shared-working-tree-concurrent-edit`. The true class size is **10 of 34
(29 %)**, fragmented across eight `recurrence_key` spellings plus one unkeyed event:

```
shared-working-tree-concurrent-edit x2 · shared-working-tree-concurrent-writers · shared-worktree-concurrent-writer
shared-worktree-concurrent-agents · shared-tree-concurrent-writes · shared-worktree-git-stash
shared-working-tree-stash · stash-in-shared-worktree   (+ 1 event with no recurrence_key)
```

`summary` clusters on the exact key string, so **the log's own reader under-reports its loudest signal
by 5×**. That is the same failure class the log exists to expose, occurring inside the log's reader.

### 4.1 Caused by the orchestration design — 18 of 34 (53 %)

| n | mode | what actually happened |
|---:|---|---|
| **10** | **Shared working tree, concurrent writers** | 6 × a sibling's in-flight edit broke my build; 4 × `git stash` swallowed a sibling's uncommitted work. `decisions.md §28` spent §8's file-touch partition on the written premise *"this branch is the only writer."* `size-modifier-agent` quotes it back: *"that premise is false in practice."* No agent chose to share a tree. |
| **5** | **Disk exhaustion / pressure** | `/tmp` tmpfs at 91 % → `ld terminated with signal 7 [Bus error]` (20 min); `/` at 91 %, 98 %, 98 %; `/home/todd` at **100 %, 0 avail** — *"30+ per-agent `CARGO_TARGET_DIR`s under ~/.cache totalling >600G, many 18-35G each"* (25 min). The rule shipped in the brief; the matching `rm -rf` did not. The original default pointed at `/tmp`, a 40 G tmpfs. |
| **4** | **`FILES YOU OWN` narrower than the change it mandated** | "fix both `.unwrap_or(Medium)` call sites" while listing one file's home; a Tauri command unreachable unless registered in `main.rs`; a DTO widening whose other two producers were unlisted. **Agents behaved correctly in all four** — they disclosed the overrun rather than shipping a half-fix. |
| **4** | **Stub-marker audit tripped by ordinary prose** | The word `placeholder` in an `assert!` message, in a `.tsx` comment, and 143 hits in `wired-integration-audit.sh` Check 1 — none a stub. |
| **2** | **Shared X display** | The sharpest case. `driver.sh` has namespaced DISPLAY/state/log per `RUN_DESKTOP_AGENT` since commit `f6fe0df2` (2026-07-24, a week before the tranche). It collided twice anyway, because the default is `default` and `grep -c RUN_DESKTOP_AGENT apps/desktop/.claude/skills/run-desktop/SKILL.md` → **0** (still 0 today). *The guard was built, was correct, and was never told to anyone.* A dispatch failure, not a tooling failure. |
| 1 | Duplicated work | `pu-class-features` and `arg-display-values` independently implemented SD-27 PU display-value resolution in the same file. |
| 1 | **The normalized red** | Below. |

**The most expensive mode, and it is not in any prior incident list:** `verify.sh`'s `root-full` stage
was RED on **29 of 33 full runs**, continuously from 07-31T05:35 to 08-01T15:58, always excused as
"environmental." That standing excuse concealed `tests/sd27_advanced_race_guide_parity.rs` and
`tests/sd27_pathfinder_unchained_parity.rs` — **the two gates that prove SD-27's own headline claim,
which never executed once for the entire tranche**. Corpus, fixtures and PCGen were all present the
whole time. `decisions.md §30.3` now records this in the bundle's own words.

```bash
python3 scripts/retro.py query --type verification --json   # 50 runs, 33 full, 29 of the 33 FAIL,
                                                            # failing_stages: root-full 29
```

### 4.2 Genuinely agent-caused — 6

- Phase 1 declaring COMPLETE on `cargo build --lib` alone; two bins were non-exhaustive, so `cargo test`
  ran **0 of 502 suites**.
- A cross-module `use crate::` inside a `#[path]`-included mod (rework).
- A prefix pin counting the wrong row family (rework).
- Reaching for `git stash` as a baselining tool (×4 — *joint*: the hazard is the orchestration's, the
  method choice is the agent's). All four self-detected, self-reported; nothing was lost.

### 4.3 Neither party — 6

The `ugrep` shim (`-o` deterministically dropped 1 of 527 matches on a large file); `driver.sh`'s
`pgrep` probe; two generator-drift incidents; `wired-integration-audit.sh` Check 1; the stub audit's
word-grep.

### 4.4 The honest summary

**53 % of this tranche's recorded process failures were emitted by the dispatch layer and then absorbed,
diagnosed and reported by the agents.** The dispatch layer concurrently wrote one tree, published an
ownership list narrower than the work it mandated, propagated a wrong diagnosis through eight baseline
revisions, and carried one environment rule (`CARGO_TARGET_DIR`) while omitting two others
(`RUN_DESKTOP_AGENT`, and the obligation to delete the target dir). The rule that was carried was
obeyed. The rules that were omitted collided twice and filled a disk.

**Recorded time lost: 161 minutes across the 12 of 34 incidents that state a figure.** 22 state none, so
161 is a floor. Three incidents are flagged `silent=true` — all three produced *plausible wrong answers*
rather than errors, and the sharpest is worth quoting because nothing would have caught it:

> *"I ran the measurement harness in a second source tree while pointing `CARGO_TARGET_DIR` at the same
> dir I had used for the working tree. Cargo served the worktree's artifacts back … and the harness
> reported 30 grounded PU records instead of 47 — **a plausible number, not an error**. Caught only
> because it contradicted a measurement taken minutes earlier."*

---

## 5. Did `verify.sh` earn its keep?

**Yes, and it also failed in a way it cannot self-detect.**

*Earned it.* 50 machine-emitted runs, 31 with a failing stage, denominator honest because nobody chose
to record them. It caught the 0-of-502 run ("cargo exit 101; 0 passed across 0 suites" — defect class #2
in its own header, working exactly as designed), caught the clippy regression (root 76→77, desktop 7→8,
**fixed at source, ceiling not raised**), and its own `awk`-based counter was *right* while the
hand-derivation the docs prescribed was wrong. Two of the five disk events exist only because
`verify.sh` emits them; nobody records disk afterwards.

*Failed.* It has no defence against a stage that is *always* red. Its failing-stage list read
`['root-full']` 29 times. The information needed to notice was in its own output; the shape of the
output made it invisible. It also cannot detect a false green in a gate that cannot see the content it
gates — two corrections record exactly that: *"the gate judges the catalog RESPONSE and never exercises
the mutation path."* And one near-miss records the reach gate's own pin failing silently: a family with
no `reach_of` arm let the whole desktop suite go green (295/295) with a false finding still listed.

**Missing guard #1 — a red-streak assertion.** When a stage has failed N consecutive runs, `verify.sh`
must refuse to report anything but `FAIL-WITH-STANDING-EXCUSE`, and must print the **named failing suite
list** for every failing stage rather than the stage name. Had it printed suite names once,
`sd27_*_parity` would have been noticed on day one instead of hour 36. Highest expected value of
anything in this document.

**Missing guard #2 — fuzzy clustering in `retro.py`.** `summary` should cluster near-duplicate
`recurrence_key`s, or `validate` should warn on them. Otherwise the loudest signal in the incident set
is the one the reader is least likely to see (§4).

**Missing guard #3 — a concurrent-writer preflight.** Three lines: if `git status --porcelain` lists a
file outside my declared ownership, stop and report. Would have prevented 10 incidents. Nothing in the
repo does this today.

---

## 6. Rules

Specific enough to follow. Each is tied to the evidence that produced it.

### 6.1 For `AGENTS.md` — durable, and **none of these are in it today**

Verified: `grep -ci '<term>' AGENTS.md` → `worktree` 0, `concurrent` 0, `CARGO_TARGET_DIR` 0, `stash` 0,
`RUN_DESKTOP_AGENT` 0.

| # | rule | prevents |
|---|---|---|
| **A1** | **One writer per tree.** Two agents must never hold uncommitted work in the same working tree. Before any write, run `git status --porcelain`; if it lists a file you did not modify, stop and report. Concurrent agents get `git worktree add` **and their own** `CARGO_TARGET_DIR`. | 10 incidents (29 %). Nothing caught these prospectively — every one was caught by an agent noticing a build error in a file it had never touched. |
| **A2** | **Never `git stash` in a shared tree.** To measure a HEAD baseline use `git show HEAD:<file>` into a temp path, or a separate worktree. `git stash` is tree-wide and takes everyone's work. | 4 incidents. Two agents independently derived this exact wording as their own resolution. |
| **A3** | **`CARGO_TARGET_DIR` is one dir per agent *per source tree*, never per agent.** A second source tree needs a second target dir. | The one silent measurement corruption (47 → 30). Caught by luck. |
| **A4** | **Delete your `CARGO_TARGET_DIR` when you finish; check disk before a full sweep.** A full sweep needs ~24 G. Under 15 % free, reclaim first. Never under `/tmp` — it is a 40 G tmpfs, 1/45th of `/`. `ld terminated with signal 7 [Bus error]` and "couldn't create a temp dir" are disk exhaustion masquerading as compiler bugs. | 5 incidents. `verify.sh` records disk pressure *after* the sweep; nothing prevents it. |
| **A5** | **A verification stage red for more than one run is a blocker, not a background condition.** Before excusing a failure as environmental, attribute **every** `test result: FAILED` line back to its `Running` line and name each suite. "The 5 known environmental failures" is a bucket, not an attribution. | The normalized red that hid both parity gates for 36 hours. |
| **A6** | **Derive counts with `awk`, not `grep -o`.** Some harnesses shim `grep` to ugrep, whose `-o` silently drops matches on large files while `-c` and `-n` stay correct. Any number that moves a baseline needs two independent implementations agreeing. | Currently documented only in `scripts/verify-baselines.env`, which is read only when re-measuring a baseline. |
| **A7** | **A magnitude is not wired until it moves on the twin the player reads** (`decisions.md §29.1`), and any surface that re-derives a rules number instead of rendering an engine `explanations` row is a candidate twin (`§29.2`). | 15 wrong-twin corrections, 6 caught only by a screenshot. |

### 6.2 For the dispatching brief — per-run, the lead's job

| # | rule | evidence |
|---|---|---|
| **B1** | **A number in a brief ships with the command that produced it, or it does not ship.** Not the value — the invocation. | 41 brief corrections. All three propagation chains are "the figure was published without its method." The clippy chain cost 5 corrections and 4 agents to prove the brief had been right all along, because the *re-measurement method* was undefined. |
| **B2** | **A ratio ships with its predicate.** "N of M carry a computed magnitude" is meaningless without the definition of *carries*. | 23 → 32 → 35 → 46 → 49 → 51 → 52 for one property on one tree, in 3 h 20 m, every step correctly verified. |
| **B3** | **`FILES YOU OWN` must be closed under the change it mandates.** Before dispatch, ask of each named fix: what else must change for this to *reach a user*? Command registration, DTO producers, second call sites. | 4 incidents; all four were agents correctly refusing to ship a half-fix. Plus one deferral in which an agent had to write outside its declared scope because the brief mandated "follow the established add-path pattern exactly" and the pattern requires a `boundary/` shim the brief did not grant. |
| **B4** | **Carry *every* environment rule, or none.** A guard that exists but is not named in the dispatch is a guard that does not exist. | `CARGO_TARGET_DIR` was carried and obeyed; `RUN_DESKTOP_AGENT` was omitted and collided twice, against a namespacing mechanism that had shipped a week earlier. |
| **B5** | **Partition on observed concurrency, not on a stated premise.** If a brief asserts a concurrency property, verify it (`git status`, `git worktree list`) before ruling on it. | `decisions.md §28` lifted the file partition citing "this branch is the only writer." 10 incidents followed. |
| **B6** | **One agent per file, and name the file in both briefs.** | Two agents wrote SD-27 PU display-value resolution into `pilot_compute.rs` simultaneously. |
| **B7** | **Verify at the widest build scope the repo has.** `cargo build --lib` green is not a completed phase. | `cargo test` builds bin targets; a broken bin meant **0 of 502 suites ran** while the phase reported COMPLETE. |
| **B8** | **Challenge the category, not just the count.** When a correction makes a number more precise without changing the frame, that is the moment to test the frame. | Chain 1: two correct corrections (2→5, 5→6) each reinforced a false category for 25 more hours. |
| **B9** | **Re-read the brief against the repo before dispatching on it.** One pass, deriving each stated figure by the command that would produce it. | 41 brief errors, 0 lead self-corrections, only 6 of 41 caught before implementation began. This is the single missing occasion in the tranche's shape. |

### 6.3 Still live on `tranche/8` today

Verified in the working tree at the time of writing:

| status | risk | evidence |
|---|---|---|
| 🔴 | `driver.sh:52` and `:111` probe `pgrep -f "target/debug/codex"`. Any brief mandating `export CARGO_TARGET_DIR=$HOME/.cache/...` removes the `target/` path component, so `launch` reports "Timed out" on a running app. | `command grep -n 'target/debug/codex' apps/desktop/.claude/skills/run-desktop/driver.sh` |
| 🔴 (new) | The readiness probe at `driver.sh:111` is **not** DISPLAY-filtered, though the kill loop at `:52`–`:53` **is**. Agent A launching can see agent B's process and declare ready against a window it does not own. | same file, lines 52–53 vs 111 |
| 🔴 | `RUN_DESKTOP_AGENT` still defaults to `default`; `SKILL.md` still has 0 mentions. Two agents that both follow the skill doc collide again. | `grep -c RUN_DESKTOP_AGENT …/run-desktop/SKILL.md` → 0 |
| 🔴 | No durable rule anywhere against concurrent writers in one tree. | `grep -ci` on `AGENTS.md` → 0 for all of `worktree`, `concurrent`, `CARGO_TARGET_DIR`, `stash` |
| 🔴 | No idempotency guard on any generator. Running `sd27_gen_book_cache` or `gen_cache_apg` is still destructive — one rewrote 467 unrelated files and dropped `records_processed` 635 → 479. | 2 incidents; `src/bin/sd27_gen_book_cache.rs` writes each `LICENSE.json` from what that one binary emitted |
| 🔴 | `scripts/wired-integration-audit.sh` Check 1 still greps the diff for `\b(STUB\|MOCK\|placeholder\|not yet implemented\|todo\|fixme\|hack)\b` with a path-exclusion list but **no context filter** — it cannot tell a comment explaining that no stand-in is rendered from an actual stub marker. Its Rust twin `tests/sd24_wired_integration_audit.rs` has a three-bucket allowlist, so writing "placeholder" in any new comment still breaks `root-full`. | 3 incidents + 1 rework; verified at `scripts/wired-integration-audit.sh:70-73` |
| 🟡 | `tests/no_foreign_home_paths.rs` `SCANNED_DIRS = ["tests","src","scripts"]`; `apps/desktop/`, `.claude/` and `docs/` are unguarded. **`docs/architecture/testing.md:186` does not merely mention a foreign path — it *prescribes* one**, directly contradicting `decisions.md §30.1`. Widening `SCANNED_DIRS` without fixing that doc yields a green guard beside a manual teaching the violation. | `command grep -n SCANNED_DIRS tests/no_foreign_home_paths.rs` |
| 🟡 | The `ugrep -o` finding lives only in `scripts/verify-baselines.env`, read only when re-measuring a baseline. | A6 above |
| ✅ | Foreign home path (`083521ed` + guard); two-compute-twins (`tests/sd27_feat_effects_reach_both_compute_paths.rs`); feat-removal affordance wired end to end; both parity suites now execute (`decisions.md §30.4`). | |

---

## 7. Corrections to this retrospective's own inputs

Logged under `RETRO_ACTOR=tranche7-retro-synthesis`
(`docs/retro/events/tranche7-retro-synthesis.jsonl`), plus 11 logged by the three mining passes.

| input | claimed | actual |
|---|---|---|
| tranche/8 retro brief | brief-authored corrections = 16 | **41** (§2.2). Logged by `retro-corrections-analyst`. |
| tranche/8 retro brief | the exact string `task brief` ×3 | ×1; the ×3 is `task brief (spell-level gate)`. |
| tranche/8 retro brief (incident pass) | disk class "recurred at 99 % on `/`" | **no event records 99 %.** Recorded `used_percent`: 91, 91, 98, 98, 100. The class recurred **5** times. |
| tranche/8 retro brief (incident pass) | "two agents shared one desktop driver display" (one event) | **twice**, 17 h apart, different pairs — `size-modifiers-agent` 07-31T21:23 and `arg-display-consumer` 08-01T14:17 (silent, 45 min). |
| tranche/8 retro brief (deferral pass) | many deferrals name SD-28/29/30 or "later bundle"/"unscheduled" | **0** name SD-28, **0** SD-30, **0** "later bundle", **0** "unscheduled"; **2** name SD-29. 63 of 83 carry a `--revisit`, and **not one names a bundle** — they name a file-owning cycle. **All routing to SD-28/29/30 is authorship, not transcription.** |
| tranche/8 retro brief (deferral pass) | the Summoner list, ARG feat magnitudes and Duergar's SLA each recur | Duergar appears in **exactly one** deferral. ARG feat magnitudes recur twice, the Summoner list three times. The largest duplicate cluster is one nothing named: **`LICENSE.json` / `records_processed`, 8 deferrals across 5 actors**. |
| `retro-corrections-analyst` | 7 self-corrections | **14** (§2.4). Its conclusion strengthens. |
| `retro-corrections-analyst` | chain 1 survived ~19 h | **25 h 40 m 23 s**. Wrong low. |
| `deferral-mining` | LICENSE.json cluster spans 6 actors | **5**. Event count (8) and conclusion both reproduce. |
| `SD-29/forward-scope-register.md` §5 | routes "all 74 deferrals" | the log holds **83**; nine postdate the register and are unrouted. |
| `SD-29/forward-scope-register.md` §4.3 | ledger row 03 CLOSED | **open.** Row 03 is the Halfling Adaptable Luck *magnitude*, collapsed with row 64 (the `%%` escape leak) and closed on that row's evidence. The shipped record still reads *"they only gain a bonus"* with no number. |
| `SD-29/forward-scope-register.md` §5 | rows 39, 47, 48, 51, 65 open | **all five CLOSED** at `tranche/8` HEAD, each verified by command. The register **understates** delivered progress. |

**Every one of those is the same defect this retrospective is about**, occurring in the retrospective's
own supply chain: a figure was read off a summary, a ledger, or a prior document instead of being
counted, and it propagated. It is included in full because a retrospective that exempts itself from its
own finding is worth nothing.

---

## 8. The five sentences worth carrying forward

1. **A number in a brief ships with the command that produced it, and a ratio ships with its predicate.**
   41 brief corrections; all three propagation chains are this one defect.
2. **"It reaches the player" is verified on screen or not at all.** Three compute twins existed and the
   tests passed against all of them.
3. **Challenge the category, not just the count.** Two correct corrections kept a false frame alive for
   25 more hours by making it more precise.
4. **Cheap counting is the best guard you have** — 46 % of everything, sole-sufficient 27 times. The
   bottleneck was never tooling.
5. **A stage that is always red tells you nothing, and a bucket is not an attribution.** 29 of 33 full
   runs, and the two gates proving the bundle's headline claim never ran once.

---

## 9. Where the forward work went

The 83 deferrals are routed in `docs/release/SD-28-ultimate-book-content-ingestion/forward-scope-register.md`,
`docs/release/SD-29-corpus-wide-catch-up-lanes/forward-scope-register.md` §7 (directory renamed
2026-08-10 from `SD-29-bestiary-line-book-ingestion`), and
`docs/release/SD-30-occult-and-companion-content-ingestion/forward-scope-register.md`. The headline of that
routing, stated here because it is a retrospective finding rather than a scoping one:

**39 of the 51 deduplicated survivors belong to no book bundle at all.** They are engine and UI debt.
The most useful thing a successor bundle can do with them is refuse to absorb them — sprinkling
UI conventions across three book bundles is precisely how `SpellCatalogScreen` and
`EquipmentCatalogScreen` diverged in the first place.
