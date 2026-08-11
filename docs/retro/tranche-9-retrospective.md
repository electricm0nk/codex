# Tranche/9 Retrospective — SD-29 (Corpus-Wide Catch-Up Lanes)

**Branch:** `tranche/9`, cut from `a1295856` (the PR #359 merge of `tranche/8` into `develop`),
closed at `ac217788`, promoted as **PR #360** (`tranche/9` → `develop`, OPEN at the time of writing).
**Window:** `2026-08-10T22:53:00Z` … `2026-08-11T14:29:51Z` — **15 h 36 m 51 s** between the first
and last event in the log.
**Written on:** `tranche/9`, 2026-08-11, from `docs/retro/events/*.jsonl` via `scripts/retro.py`,
plus `docs/release/SD-29-corpus-wide-catch-up-lanes/{progress,kanban,decisions,release-notes}.md`,
git, and the orchestrating session's own workflow record.

This is the second retrospective written from the event log. Tranche/7's loudest finding was that
**figures restated from one brief into the next without re-measurement** were its largest source of
wrong claims. Tranche/9's is different and, if anything, worse: the bundle's dominant cost was not
wrong numbers at all — those were caught early and cheaply, roughly on schedule — it was **the
dispatch layer's physical arrangement of the run**. Six concurrent worktree agents on a two-core box
filled a disk, and a preflight gate correctly refusing to run on that disk silently cost SD-29 an
entire kind lane: **1,696 companion units, 0 grounded, no cycle ever started.**

---

## 0. How to reproduce every number in this document

```bash
python3 scripts/retro.py validate      # 600 event(s), all valid   (as of 2026-08-11T14:30Z)
python3 scripts/retro.py summary
python3 scripts/retro.py query --type correction --json
```

### 0.1 The selector, and why the obvious one is wrong

The log now holds 600 events across every bundle this repo has run. Three candidate selectors for
"tranche/9" were tested and two of them are wrong:

| candidate selector | result | verdict |
|---|---|---|
| `repo.branch == "tranche/9"` | 73 events | **WRONG — loses 76.** Eight lane agents ran in dispatch worktrees and stamped `worktree-wf_3516060a-756-{6..13}` as their branch. |
| `glob docs/retro/events/sd29*.jsonl` + `operator-prelaunch.jsonl` | 166 events | **WRONG in both directions.** Adds 21 tranche/8-era events (`operator-prelaunch` ×15 and `sd29-scope-and-debt` ×6, all dated 2026-08-01/02 — SD-29 *scoping*, done nine days before the branch existed) and misses 4 (`codex.jsonl`, `verify.sh`'s and `reclaim.sh`'s fallback-actor emissions during the run). |
| **`ts >= 2026-08-10T22:37:02Z`, all shards** | **149 events** | **CORRECT.** The cut point is the committer date of `a1295856`, the commit `tranche/9` was cut from. |

**The selector used throughout this document:**

```bash
python3 - <<'PY'
import json,glob,collections
CUT='2026-08-10T22:37:02Z'   # git log -1 --format=%cI a1295856 -> 2026-08-10T18:37:02-04:00
ev=[json.loads(l) for f in glob.glob('docs/retro/events/*.jsonl') for l in open(f) if l.strip()]
t9=[e for e in ev if e['ts']>=CUT]
print(len(t9), len({e['actor'] for e in t9}), collections.Counter(e['type'] for e in t9))
PY
# -> 149 19 Counter({'verification': 48, 'incident': 44, 'correction': 34,
#                    'deferral': 18, 'near_miss': 3, 'rework': 2})
```

**149 events across 19 actor shards.** Nothing in the log distinguishes a tranche/9 event by field;
only time does. That is itself a finding — see §6, rule **A9**.

Every count below is derived by a command shown at the point of use. Where a figure could not be
reproduced, the correction is stated inline and logged as a `correction` event — including
corrections to this retrospective's own inputs and to the orchestrating session's account of its own
run (§7).

---

## 1. What tranche/9 delivered

Derived from git across the tranche boundary (`a1295856` → `ac217788`):

| measure | value | command |
|---|---:|---|
| commits | 61 | `git rev-list --count a1295856..ac217788` |
| files changed | 555 | `git diff --shortstat a1295856 ac217788` |
| lines | +15 332 / −3 332 | same |
| new corpus record files | 32 (all `bonus_bestiary`) | `git diff --name-status … -- data/corpus \| awk '$1=="A"'` |
| new test files | 3 | `git diff --name-status … -- tests/ \| awk '$1=="A"'` |

Set against tranche/7's 78 commits / 5 775 files / 965 new corpus records, **this is a small bundle
by volume.** That is the correct reading. SD-29 re-cut itself twice mid-flight — from per-book epics
to kind lanes (`decisions.md §37`), then from seven books to all 37 in-scope books (`§38`) — and what
it actually shipped is mostly *instrument*, not content:

- **A corpus-wide picture that did not exist before.** `corpus-shape-37-books.md`: 37 books, 38 517
  units, 2 253 proven, derived in one pass with every figure carrying its command, then
  independently re-derived by a second pass against a freshly regenerated inventory (§8 of that
  file, 12 spot-checks, zero disagreements).
- **Three real classifier defects fixed in the measuring instrument itself**, each of which had been
  silently corrupting every figure downstream: the spell two-list divergence (192 units reported
  `not-ingested` while already on screen), the race-trait name-coincidence defect (grounded
  44 → **21**), and the companion mis-classification (`race_trait` 3 456 → 3 447, `companion`
  1 683 → 1 696).
- **The monster / monster-ability chassis, pilot-proven end to end** on Bonus Bestiary: 14 monster +
  17 monster_ability units grounded, `RuleSetId::BonusBestiary` plus its rules-table module,
  generator arm, wire DTO, `CORPUS_KIND_NAMES` entry, two reach claims, and frontend path. The cost
  is once-per-*kind*; every remaining monster-bearing book inherits it.
- **The identifier-discipline gate, made real.** Epic 1 found the audit implemented 3 of the 4
  patterns its own epic-breakdown named; Epic 1b found three further whole escape classes. The gate
  went from 0 self-test cases to **28**, wired into `verify.sh` as the new `audit-selftest` stage.
- **83 corpus feat rows** closing the feat not-ingested gap corpus-wide, and **769** equipment rows.

The gate ended green: `verify.sh` full, exit 0, **all 12 stages PASS**, `root-full` 6 170 passed
across 543 suites with **all 524 `tests/*.rs` suites executed**, `reach` 17 matched. That is the
outcome. The rest of this document is the process, and the process is where this tranche's story is.

---

## 2. Where the wrong claims came from

### 2.1 The count and the normalisation

34 corrections, 34 distinct subject strings, 14 distinct correcting actors (of 19 total). No subject
string repeats. As in tranche/7 the raw tally is a long tail by construction, so it has to be
normalised. Here is the normalisation, stated so it can be re-run and disputed — it is a **hand
classification of all 34**, single-label, primary-cause, performed by reading each event:

| who was wrong | n | share |
|---|---:|---:|
| **ARTIFACT** — shipped code, doc comment, generated data file, architecture doc, a governance sweep | **13** | **38.2 %** |
| **PACKAGE INSTRUCTION** — `kanban.md` card scope figures, `loop-instruction.md`, `epic-breakdown.md`, a dispatch brief's standing note, the operator's pre-launch brief | **10** | **29.4 %** |
| **SELF** — an actor correcting a claim it made in the same cycle | **6** | **17.6 %** |
| **THE LOG ITSELF** — three corrections whose subject is a prior `correction` event | 3 | 8.8 % |
| **AGENT** — another agent's receipt or committed baseline | 2 | 5.9 % |

The PACKAGE-INSTRUCTION share is **29.4 %**, against tranche/7's BRIEF share of **35.7 %**. The
direction is right but the movement is small and should not be over-read: the denominators differ
by 3.4× (34 vs 115) and the classification is a judgement.

**What did move, unambiguously, is self-correction.** Tranche/7: 14 of 115 = 12.2 %, and *"the lead
self-corrected zero times."* Tranche/9: **6 of 34 = 17.6 %**, and three of the six come from one
agent (`sd29-e5-monster-pilot`) correcting its own first draft three separate times in one cycle —
its natural-attack denominator, its wire adapter, and its cache generator's `wiring_class`.

```bash
# subject names the correcting actor, or says "this cycle"/"its own first"
# -> 9 raw, minus 3 substring false positives (an audit script's "its own header";
#    license-matrix.md's "its own five-minute manual sweep"; a dispatch brief that
#    happens to carry the actor's name) -> 6
```

The mechanism is visible in the events: every one of the six was caught by a test or a generator
diff the actor wrote **in the same pass**, before committing. `sd29-e4-frc` states it against
itself twice in ninety seconds — once for a generator total (83 vs 84) and once for a hand-summed
census map, the second annotated *"the exact failure that test's own comment (decisions.md 43) warns
about, repeated one line below the warning."*

### 2.2 The corrections were caught earlier than tranche/7's

```bash
python3 - <<'PY'
import json,glob,collections
CUT='2026-08-10T22:37:02Z'
ev=[json.loads(l) for f in glob.glob('docs/retro/events/*.jsonl') for l in open(f) if l.strip()]
cor=[e for e in ev if e['ts']>=CUT and e['type']=='correction']
print(collections.Counter(e.get('caught_before') for e in cor))
print('blast_radius stated:', sum(1 for e in cor if e.get('blast_radius')), 'of', len(cor))
PY
# -> Counter({None: 14, 'implementation': 9, 'merge': 7,
#             'nothing (already shipped)': 3, 'release': 1})
# -> blast_radius stated: 17 of 34
```

**9 of 20 corrections that state the field were caught before implementation began**, against
tranche/7's 6 of 41 for brief errors. `blast_radius` is stated on **17 of 34 (50 %)** — against
tranche/7's 17 of 41 (41 %) for brief errors and 49 of 74 (66 %) for everything else. Agents are
still measurably more reluctant to write down how far an *instruction's* wrong number travelled than
an artifact's, but the gap narrowed.

**Only 3 corrections carry `caught_before: nothing (already shipped)`** — and all three are the same
event: `sd29-prelaunch-fourfixes` correcting three *prior correction events* (tranche/8's
`epic-6-ui`, `epic-7-ucam`, `epic-8-uw` shards) which had each wrongly "corrected" a brief note that
was right all along. `*.pcc` matches `_ultimate_intrigue.pcc`; the shell glob does not exclude a
leading underscore. **Three agents in a prior bundle independently made the same wrong correction,
and it took a dedicated pre-launch pass to unwind it.** A correction is an assertion like any other,
and this log now contains its own worked example of one being wrong three times over.

### 2.3 The propagation chains, and what each teaches

**Chain 1 — the stale ref. Three instances, one root cause, and the third was the expensive one.**

| when | who corrected what | claim → correction |
|---|---|---|
| 08-11 12:45 | `sd29-e10-review` → **itself, first pass** | *"The bundle is missing all of Epics 4-7: 30 commits of lane work sit unmerged on eight worktree branches… Epic 11 would promote an empty bundle"* → **"The lane work was on `origin/tranche/9` all along. The LOCAL `tranche/9` ref was 33 commits stale."** |
| 08-11 12:46 | `sd29-e10-review` → three cycle receipts | *"DoD item 8 is currently unsatisfiable for every player-visible lane in this bundle"* → **satisfiable, and satisfied**: `epic-4-proven-spell` had committed two 1920×1200 PNGs of the live Spell Catalog in `1ddeb2f7`, an *ancestor* of the commit carrying the impossibility claim |
| 08-11 13:59 | `sd29-e10-review` → `epic-9-version` | root clippy measures **54**, so tighten the ceiling 75 → 54 → **55 on the real branch tip.** The 54 was measured 33 commits behind; the tightened ceiling failed the clippy stage on the merged tree by exactly one |

The review cycle names it itself: *"Third instance this bundle of a figure derived on an unfetched
ref."* And the fix on the third was the right one — *"Fixed by paying the warning down, not by
raising the ceiling back."*

The shape is `wrong tree → confident claim → claim restated by later cycles as established fact`.
The middle row is the sharpest thing in the log: **an impossibility claim propagated through two
further receipts while the evidence disproving it sat in their own shared git ancestry.** Nobody
re-tested it, because "the tooling can't do that" reads as an environmental fact rather than a
measurement. It would have entered the closure receipt as a bundle-wide tooling blocker and funded a
`driver.sh` repair card that is not needed.

**Chain 2 — the count pin nobody owned. 3 cycles blocked, ~2 hours of gate time, one commit to fix.**
`dde9dfc4` ("close the feat not-ingested gap corpus-wide, 83 rows") moved a record count without
sweeping for other files' hardcoded assertions. `tests/v06_apg_acg_feat_catalog.rs` went red on two
assertions. Then:

- `sd29-e6-racetrait-pilot` (08:34) hit it, **proved by content that its own diff could not have
  caused it** (*"this card's only code change is `src/bin/v06_work_inventory.rs`, a BINARY, which no
  integration test links"*), and deliberately did not fix it — the owning card was live under another
  actor.
- `sd29-e5-monster-extend` (11:10) hit it, proved attribution the same way, and again refused:
  *"Editing them from this card would clobber another session's live work, which
  `loop-instruction.md` names as a STOP."*
- `sd29-e10-review` (12:46) finally re-pinned both, one commit (`b4cff429`).

Both intermediate agents behaved **exactly right**, and the cost was still three blocked cycles. This
is tranche/7's normalized-red defect recurring — but recurring *visibly*: it was named as an incident
with a `recurrence_key` at first occurrence, attribution was proven rather than asserted, and it was
resolved inside the bundle rather than surviving it. AGENTS.md's rule A5, shipped out of the tranche/7
retrospective, is the reason. **The rule worked. The gap it leaves is that nothing re-queues a
blocker onto its owner** — three agents each independently rediscovered it.

**Chain 3 — the pilot book that had no pilot content.** `epic-6-race-trait-lane-pilot` was pinned to
`inner_sea_intrigue` on the strength of its 9 `race_trait` units. The unit *count* was right; the
*kind* was wrong. All 9 come from `isi_abilities_race_companion.lst` and are Clockwork Familiar /
Clockwork Spy construct-companion abilities. **The book carries zero genuine race traits.** Root
cause: `file_kind()` types a file by its basename, and `_abilities_race` was tested before the
`companion`/`familiar` markers. The pilot half of the card was `decision-blocked` on the spot; the
classifier fix shipped.

This is the *"not-ingested figures are classifier noise"* class, and this program has now hit it in
three consecutive bundles. The lesson has sharpened: it is not enough to sanity-check a count — a
count and a kind are two different claims, and **a scoping figure that selects a pilot must be
verified at source, one record deep, not at the inventory.**

### 2.4 One correction is more serious than the rest and belongs here

`sd29-e5-monster-pilot`, 09:13: the first monster_ability wire adapter served the corpus `DESC:`
token verbatim, so the catalog printed Babble as *"must succeed on a DC **%1** Will save."* Caught by
`driver.sh screenshot` under `RUN_DESKTOP_AGENT`, on 16 of the book's 17 ability descriptions. The
event's own summary: **"DoD item 8 earned its place again — a passing reach gate rendered internal
syntax to the player."** No test caught it. `render_pcgen_desc` now runs at the display boundary.

---

## 3. What caught things: the detection mechanisms, ranked

This is the load-bearing section. Tranche/7 ranked ad-hoc commands over source data at ~46 % and
on-screen driving at 14 %, and the program's practice was built on that result. It is re-run here
over tranche/9's own 34 corrections, with **tranche-7 §3's regexes unmodified**, so the two are
comparable:

```bash
# ad-hoc command : \b(grep|rg|awk|sed|wc -l|python3|find |sort -u|uniq|Counter|jq)\b
# repo test      : cargo test|npm (run )?test|vitest|#\[test\]|test fail|assertion
# on-screen      : driver\.sh|screenshot|Xvfb|DISPLAY|on-screen|desktop app
# probe          : throwaway|temporary (bin|harness|test)|probe\b|one-off (bin|harness)
# clean tree     : git worktree|detached|git stash|pristine|clean checkout
# verify.sh      : verify\.sh          git archaeology: git log|git show|git ls-files|git blame|git diff
```

| rank | mechanism | mentions | % of 34 | SOLE |
|---:|---|---:|---:|---:|
| 1 | **ad-hoc command over source data** (`grep`/`awk`/`python3`/`find`/`wc`) | **16** | **47.1 %** | **15** |
| 2 | a repo binary (`cargo run --locked --bin`) | 3 | 8.8 % | 2 |
| 3= | git archaeology (`git log`/`git show`/`git diff`) | 2 | 5.9 % | 0 |
| 3= | an automated test in the repo | 2 | 5.9 % | 2 |
| 3= | `scripts/verify.sh` | 2 | 5.9 % | 0 |
| 3= | `cargo clippy` measured directly | 2 | 5.9 % | 0 |
| 7 | **driving the shipped app on screen** | **1** | **2.9 %** | **1** |
| — | throwaway probe / clean-worktree re-measurement | 0 | 0 % | 0 |

Distribution: 20 corrections name one mechanism, 4 name two, and **10 name none this classifier
recognises.**

**Rank 1 reproduces almost exactly: 47.1 % against tranche/7's 46 %, sole-sufficient 15 of 16
times.** Two independent tranches, different bundles, different agents, different content, same
answer to within one point. That is the most robust number this program has. **A one-line `grep` /
`awk` / `python3` over the source corpus is the strongest guard available, and it is also the
cheapest.**

### 3.1 The classifier under-reports, and here is by how much

10 of 34 unmatched is a 29 % miss rate — tranche/7's was 10 %. Reading the ten shows the regexes,
not the tranche, are at fault. Two defects, both of the exact class this log exists to expose:

1. **`find ` with a trailing `\b`** cannot match `find ~/workspace/...` — `~` is not a word
   character. Four corrections verified by a `find` over the live PCGen tree scored zero.
2. **`ls ` is not in the classifier at all**, nor is a shell self-test harness
   (`bash scripts/tests/test_identifier_discipline_audit.sh`), nor "read the rendered surface" of a
   committed screenshot.

Re-run with those three holes closed and nothing else changed:

| rank | mechanism | mentions | % of 34 | SOLE |
|---:|---|---:|---:|---:|
| 1 | **ad-hoc command over source data** (adds `ls`, fixes `find`) | **22** | **64.7 %** | **20** |
| 2 | an automated test in the repo (adds the shell self-test) | 4 | 11.8 % | 3 |
| 3= | a repo binary | 3 | 8.8 % | 2 |
| 3= | git archaeology (adds `git merge-base`, `git ls-tree`, `git status`) | 3 | 8.8 % | 0 |
| 5= | **driving the shipped app on screen** (adds "read the rendered surface" of a committed PNG) | **2** | **5.9 %** | **1** |
| 5= | reading the cited source file directly | 2 | 5.9 % | 0 |
| 5= | `verify.sh` / `clippy` measured directly | 2 / 2 | 5.9 % | 0 |

Unmatched falls to 2 of 34. **Under either classifier, rank 1 is ad-hoc counting and its lead is
enormous — 47 % or 65 %, sole-sufficient in 15 or 20 cases.** Report the amended figure as the
better estimate and the verbatim figure as the tranche-7-comparable one; the conclusion does not
depend on the choice.

### 3.2 On-screen driving: the number fell, the finding did not

**Corrections-only, on-screen driving scores 1/34 (2.9 %) verbatim, 2/34 (5.9 %) amended, against
tranche/7's 16/115 (14 %).** Taken alone that reads like a collapse. It is not, and the corrections
denominator is the wrong instrument for this mechanism. Widen it to **corrections + near-misses (37
events)** and on-screen driving accounts for **3 of 37 = 8.1 %, and is the SOLE named mechanism in
all three.** Reading the three:

1. **The `%1` raw-token leak** (§2.4) — correction, `sd29-e5-monster-pilot`. Every Rust test green.
2. **The Spell Catalog chip row** — `near_miss`, `escaped: true`,
   `would_have_reached: "already shipped — UI spells joined the served chain a full bundle before
   this cycle, so this reached real users."` The screen served **1 286** spells under filter chips
   summing to **1 185**: Ultimate Intrigue's 101 spells present in the list, no chip to filter to
   them, no player-facing text naming their book. Caught by *"reading the rendered chip row and
   adding it up against the stated total."*
   The `gap` field is the single most quotable line in this tranche's log:
   > *"The frontend test written to prevent exactly this passed the whole time. Its oracle
   > `CHAINED_BOOK_CODES` was a **COPY of the `BOOK_ORDER` constant under test** rather than an
   > independent statement of what `spell_catalog.rs` chains, so it drifted in lockstep with the
   > defect it existed to catch. Every Rust test, every frontend test and the reach gate were green
   > with the defect live."*
3. **The wire-contract rename** — `epic-1b-naming-sweep` renamed a Tauri command
   (`load_ge08_authoring_workbench_snapshot` → `load_authoring_workbench_snapshot`) on both sides in
   one commit and then *drove the app* to the Developer panel: `WM_NAME(STRING) = "Codex"`,
   *"Connected to the app backend"*, *"Live backend data"*, `BACKEND v0.8.0 · 8b6dd7511f2f`. A
   one-sided rename fails at runtime, not compile time; every test would have stayed green.

**The mechanism's share fell; its unique competence did not move at all.** All three defects were
invisible to every test in the repo, and one of them had already shipped to real users. Tranche/7's
ruling stands verbatim and this tranche supplies fresh, independent evidence for it:

> **A claim that a value reaches the player is not verified until it has been read off the running
> app.**

The share fell for a structural reason worth naming: **most SD-29 cycles surfaced no new
player-visible record family at all.** Counting DoD-item-8 dispositions in `progress.md`: **3
PERFORMED**, **2 NOT PERFORMED (recorded as shortfalls)**, and **at least 6 declared a real N/A** —
classifier fixes, denominator derivations, provenance gating, version stamping and closure, where a
screenshot would prove nothing. The base rate changed, not the guard.

```bash
grep -n 'item 8' docs/release/SD-29-corpus-wide-catch-up-lanes/progress.md \
  | grep -iE 'N/A|NOT PERFORMED|MANDATORY'
```

### 3.3 What did *not* appear at all

`probe` and `clean tree` score **zero mentions**, against 11 and 7 in tranche/7. Two readings, and
the log cannot decide between them: either the instruments the corpus lanes needed already existed
this time (`v06_work_inventory`, `v06_corpus_trap_report --audit`, `reach_gate` — all shipped, all
used, and `cargo run --locked --bin` scores 3), or **nobody reached for a throwaway probe and so
nobody found what a probe would have found.** Tranche/7 recorded probes as sole-sufficient 9 times.
The log is silent on which of these is true, and that silence is itself a finding.

### 3.4 Cross-tabulated by who was wrong

- **Package-instruction errors** were caught almost exclusively by **counting** — `python3` over
  `docs/work-inventory.json` appears against them 7 times. Not one was caught by a test.
- **Artifact errors** split between counting and **reading the cited source**, and three of the
  four architecture-doc corrections were caught by a single `sed -n '/pub enum RuleSetId/,/^}/p'`.
- **Self-corrections** were caught by **a test or a generator diff written in the same pass**, 6 of
  6 — reproducing tranche/7's finding #2 exactly.
- **Agent errors** were caught by **re-measuring on the real ref** (`git fetch`, then the same
  command).

---

## 4. Process and tooling failures: orchestration vs agents

44 incidents. First, three properties of the incident set that must be stated before any of it can be
read:

**(a) 23 of the 44 are machine-emitted, not authored.** 14 are `reclaim.sh --apply` receipts and 9
are `verify.sh`'s end-of-run `df` disk-pressure check. Only **21 incidents were written by an agent
deciding something was worth recording.** Any per-incident percentage over the raw 44 is diluted by
the automation, and this document reports both denominators wherever it matters.

**(b) The recurrence keys are cleaner than tranche/7's but still fragment the loudest signal.**
`retro.py summary` clusters on exact key string and has **not** been given the fuzzy clustering
tranche-7 §5 asked for (`grep -n recurrence_key scripts/retro.py` → still `recurrence[key] += 1`).
Tranche/7 fragmented one class across eight spellings. Tranche/9 fragments the same physical
condition — the box ran out of disk — across **three**: `disk-full` (16), `disk-pressure` (9),
`preflight-disk-normalized-red` (1). 26 of 44 incidents, **59 %**, are one condition wearing three
names. The improvement is real (8 spellings → 3) and it came from automation emitting stable keys,
not from anyone fixing the reader.

**(c) The orchestrating session emitted zero events.** All 19 actors are dispatched agents or
automation. **Every orchestration failure below is visible only because a dispatched agent happened
to be in a position to observe it and chose to write it down.** The dispatch layer is the one actor
in this run with no instrument pointed at it, which is precisely why §7 exists.

### 4.1 Caused by the orchestration design

| n | mode | what actually happened |
|---:|---|---|
| **26** (59 % of all incidents; **9 of the 21 authored**) | **Disk exhaustion from bundle-wide concurrency** | Six worktree agents concurrent on one filesystem, plus a 60 G `target/` in the primary checkout. Recorded `used_percent`, 12 values: **90, 90, 91, 91, 91, 91, 92, 93, 93, 93, 94, 96.** `preflight-disk` is the single largest failing stage in the whole log (**9 failures**, more than every other stage combined). `reclaim.sh --apply` ran 14 times and, at the peak, **reclaimed 0.0 B** — *"every candidate is a live target dir, a worktree with unpushed commits, or a checked-out branch."* The condition was correctly diagnosed as structural by `sd29-e6-racetrait-pilot`: **"Disk pressure is now structural from bundle-wide concurrency, not from reclaimable garbage."** |
| **6** (receipt-recorded; **only 2 in the event log**) | **Wrong-base dispatch worktrees** | Dispatch worktrees were created at `7d9f1c4f` — **`origin/main`'s tip**, a PR-#23 merge from 2026-06-28 with no `docs/` tree at all, so `docs/release/SD-29-corpus-wide-catch-up-lanes/` did not exist and **none of the card's required reads were present.** Every affected agent detected it as its first action (`git log --oneline -3` + `ls` of the package dir) and recovered with `git fetch && git reset --hard origin/tranche/9`. |
| **3** | **Shared-checkout collisions between the two non-worktree agents** | Epic 8 and Epic 9 ran concurrently in the *primary* checkout. Consequences: a gate whose result *"certifies the mixture, not either card"*; a `pkill -f 'scripts/verify.sh'` that could have killed a sibling's 45-minute gate (near-miss, caught by `pgrep -af` immediately after); and **five green Epic 9 commits that could not be pushed at all**, because `git pull --rebase` and `git merge` both abort on a sibling's uncommitted retro shard and `git stash` is banned in this tree. |
| **2** | **`verify.sh` outlasting the turn budget** | Epic 2 run 1 ran out of turn mid-`root-full`, leaving its derivation untracked and its card `IN-FLIGHT` with no receipt (45 min). `epic-4-proven-feat-race-class` did the same and **never obtained an exit code at all** — its receipt states it exactly: *"a gate that has not returned is not a gate that passed."* |
| 1 | **Harness timeout misread as gate failure** | Epic 8 launched the gate through a backgrounded call with a 10-minute timeout; the harness SIGTERMed it. `verify.sh` exit **143** = 128+15. *"A non-zero exit code with four PASS stages already printed reads exactly like a failing gate."* Symmetrically, the harness's own task notification reported "completed (exit code 0)" — the **wrapper's** status. **Two available summaries pointed in opposite directions and neither was the gate's verdict.** |
| 1 | **Concurrent full sweeps in one checkout** | Two `verify.sh` full runs in the same tree, separate target dirs, both starved at `-j 2` on a 2-core box. Symptom: a sweep that *looks hung* for 15 minutes. Flagged `silent: true`. AGENTS.md's one-writer rule covers uncommitted work and says nothing about concurrent sweeps. |

**Quantifying the concurrency question the orchestrator raised.** Of the 21 *authored* incidents,
**9 (43 %)** name disk, load, contention, or a gate that did not return. Of all 44, **32 (73 %)**
touch that class. Either way it is the dominant mode, and unlike tranche/7 — where the dominant mode
was shared-tree writes, 10 of 34 = 29 % — **this one has a body count.**

**The bill, stated plainly:**

- **`epic-7-companion-lane-pilot` never started.** `verify.sh --only preflight-disk` exit 1 at 91 %
  used / 47 G free, twice, with `reclaim.sh --apply` (which freed ~1 MB) in between. The agent
  refused correctly: it did not weaken the gate, did not set `PREFLIGHT_DISK_MAX_PERCENT`, did not
  fabricate a pass, and deliberately left the card `READY` rather than parking it `IN-FLIGHT`. Its
  dependent `epic-7-companion-lane-extend` was therefore never eligible. **Cost: the entire companion
  kind — 1,696 units, 0 grounded.** By closure the disk was at 80 % / 97 G free. `sd29-e11-closure`
  names the real defect: *"An environmental refusal at a preflight gate silently cost SD-29 a whole
  kind lane"* — **because nothing re-queued the card once the condition cleared.**
- **`epic-4-proven-feat-race-class` never obtained a gate result**, left 83 landed rows behind two
  red count pins, and blocked two further cycles until Epic 10 fixed them.
- **`epic-5-monster-lane-pilot` was marked COMPLETE while its entire chassis sat unmerged** on
  `origin/worktree-wf_3516060a-756-9`. The extend cycle discovered it by
  `grep -rn 'monster_ability' --include=*.rs -l .` returning one file, merged the branch (3 conflicts,
  all in generated/append-only docs, zero code conflicts), and **spent its whole budget on the
  integration.** The incident names the mechanism precisely: **"Dispatch marks a card COMPLETE from
  its receipt, but nothing merges the worktree branch."**

**Time lost is stated on 3 of 44 incidents: 45 + 70 + 25 = 140 minutes.** That is a floor and a
badly incomplete one — it excludes the companion lane entirely, excludes the three cycles blocked by
the count pins, and excludes the extend cycle's whole budget. The honest statement is that **the
recorded figure is not usable** and the real cost is better read off the kanban: 3 cards `PARTIAL`,
1 `DECISION-BLOCKED`, 2 never started.

### 4.2 Genuinely agent-caused — 5

- **A repo-wide `sed` rewrote the gate that polices the rename** (`epic-1b-naming-sweep`), converting
  every self-test detection case into an already-clean string. *"The gate would have kept reporting
  26/26 while testing nothing."* Caught by reading `git diff` of `scripts/` before running the
  self-test; both files restored and excluded from every later pass. **Self-detected, self-reported.**
- **`gen_equipment_gap_tables` shipped a baked-in `/home/ubuntu/...` corpus path** plus a hand-rolled
  tilde expander. Caught by `verify.sh` `root-full` — `tests/no_foreign_home_paths.rs`, the guard
  tranche/7 built for exactly this.
- **A staged matcher was widened by appending** 769 rows "at the end so first-match is unchanged" —
  false, because the resolver's precedence is stage-major. `equipment_cost_gp_headless_resolve('Cold
  Iron')` went `Some(0.0)` → `None`. Caught by a `root-lib` test probing all 2 977 CRB rows × 3
  identity forms.
- **`gen_equipment_gap_tables`' already-held filter** emitted 741 rows instead of 769 (rework). Its
  `avoidable_by` is the durable lesson: *"Differencing the generator's own total against
  `docs/work-inventory.json`'s not-ingested count BEFORE wiring anything — 741 vs 769 named the
  defect in one line; **no test in the repo would have caught it, since both numbers are
  self-consistent.**"*
- **A record-count change without a sweep** (`dde9dfc4`, §2.3 chain 2) — rework, 1 commit, 2 cycles
  blocked.

Every one of the five was found by a mechanism the agents themselves ran, and every one was disclosed
rather than smoothed over. **Agent behaviour in this tranche was not the problem.** The one
substantive process criticism available against the agents — that three cycles each rediscovered the
same red pin — is fairer against the dispatch layer, which had no mechanism to route a discovered
blocker back to its owner.

### 4.3 The honest summary

**Tranche/7's finding was that 53 % of recorded process failures were emitted by the dispatch layer
and absorbed by the agents. Tranche/9's is the same finding, sharper.** The dispatch layer:

- placed **six concurrent worktree agents plus two shared-checkout agents** on a two-core box with a
  60 G `target/` already resident, and provided no disk budget, no concurrency cap, and no
  admission control;
- cut dispatch worktrees from **`origin/main`** — not from the branch the brief named — at least six
  times, so that six agents' first act was to discover their required reads did not exist;
- marked a card **COMPLETE from its receipt without merging its branch**, stranding a whole chassis;
- ran two agents in the **primary checkout** concurrently, blocking five green commits from being
  pushed at all;
- treated **"ran out of turn" as identical to "blocked"**, halting the entire run at Epic 2;
- and **emitted no retrospective events of its own**, so all of the above is recorded only where an
  agent noticed.

Against that, the agents refused to weaken a gate (twice), refused to edit a sibling's live files
(twice), refused to substitute a passing test for a missing screenshot (twice), refused to claim a
gate that had not returned, killed by PID after catching a pattern-kill near-miss, and recorded a
mixed-attribution gate result as mixed. **The discipline held on every occasion where it was tested.**

---

## 5. Did `verify.sh` earn its keep? And did `audit-selftest`?

### 5.1 `verify.sh`: yes, and more decisively than in tranche/7

```bash
python3 scripts/retro.py query --type verification --json   # filter ts >= CUT
# -> 48 runs: 26 full, 22 --only; 34 PASS, 14 FAIL (10 of the 26 full runs failed)
# -> stages_failed: preflight-disk 9, root-full 6, frontend-test 2, clippy 2, frontend-typecheck 1
```

48 machine-emitted runs, denominator honest because nobody chooses to record them. It grew from 9
stages to **12** (`pi-sweep` and `audit-selftest` added this bundle). What it actually caught:

- **The baked-in foreign home path** in a brand-new codegen binary — `root-full`,
  `tests/no_foreign_home_paths.rs`, both assertions. That guard exists *because of tranche/7*, and it
  fired on its first opportunity in a new bundle.
- **The staged-matcher repricing regression** — `root-lib`, a test probing 2 977 × 3 identity forms.
- **The prerequisite-completeness near-miss** — `root-full`,
  `every_pre_kind_in_the_catalog_is_either_modelled_or_declared_unmodelled`. Widening the feat catalog
  introduced `PREHANDSGTEQ` and `PRESIZEGTEQ`, which the evaluator had never seen; without the guard,
  3 corpus feats would have been offered to characters who do not qualify, silently.
- **The two unowned count pins** (§2.3 chain 2) — `root-full`, three times.
- **The disk condition**, 9 times, including the refusal that stopped Epic 7. That refusal cost a
  lane and it was **correct**: the gate did its job; the run had no business being arranged that way.
- **Decision 40's did-not-execute check now ships** (`verify.sh:378`, `comm -23` between the derived
  expected-suite list and the log's `Running` lines). The closure gate reports **all 524 `tests/*.rs`
  suites executed.** This is tranche-7's missing-guard-#1 partially built, and it is the direct
  answer to the tranche/7 defect where two proof-carrying parity gates never ran for 36 hours.

**Tranche/7's verdict was "a floor, not a detector" — 8 % of corrections, sole in 5, and it *"never
caught a wrong claim."*** That is now out of date. `verify.sh` still scores low on the corrections
classifier (2 of 34) but it caught **four defects and one escaped-class near-miss** in this bundle,
including two that no ad-hoc command would have found, and it is the sole reason the disk condition
was ever measured.

**Where it failed.** The red-streak assertion tranche/7 asked for still does not exist
(`grep -n 'streak\|consecutive\|repeat' scripts/verify.sh` → nothing). `root-full` was red on 6 runs
across 3 cycles for the same two assertions, and each cycle rediscovered the attribution from
scratch. The mitigation that made this survivable was *cultural*, not mechanical: AGENTS.md rule A5
made each agent attribute the failure by content rather than excuse it. **The rule shipped; the guard
did not.**

Second failure, and it is the one worth acting on: **`verify.sh` has no notion of who else is running
it.** Two full sweeps in one checkout starve each other for 15 minutes and look hung. Six sweeps
across six worktrees fill the disk. Nothing in the script or the repo detects either.

### 5.2 `audit-selftest`: it earned its keep at build time and has caught nothing since

```
stages_passed across 48 runs: audit-selftest 26, pi-sweep 19   (both never FAIL)
```

**As a standing gate, `audit-selftest` has a perfect record and zero catches.** 26 runs, 26 passes.
That is the honest answer and it should not be dressed up.

**As a construction discipline it is the highest-value thing in this bundle**, because building it
is what exposed the defect:

- **Epic 1** wrote the first self-test and found `OK_NO_BUNDLE_TAGS` implemented **3 of the 4 patterns
  its own acceptance criteria named**. The hyphenated form (`sd29-monster-row` as a CSS class or
  `data-testid`) passed clean. `blast_radius`: *"every SD-29 lane epic: Epic 1's acceptance criterion
  is this audit returning 0 findings, and Epic 10 re-runs it at bundle scope — a hyphen-form tag would
  have shipped past both."* 13 cases, RED first.
- **Epic 1b** then found **three further whole escape classes** in Epic 1's *hardened* version:
  neither GE-NN form, no infix form (`_` is a word character, so a leading `\b` can never match
  `kind_is_sd17_b3`), and no file-or-directory path tag at all — the regex was identifier-shaped and
  scanned content only. `bash scripts/tests/test_identifier_discipline_audit.sh` → **16 passed / 10
  failed** before the fix, 26/0 after. Later extended to 28.
- **Epic 1b also found the gate punished the fix**: it scanned the *whole* unified diff including
  `-` lines, so removing a tag counted as introducing one. `BASE_BRANCH=origin/develop bash
  scripts/identifier-discipline-audit.sh` → exit 1, offending line `-  grounding_ref:
  GE06_INPUT_CONTRACT_TEST`, **a deletion**. The audit's own header had claimed for months that it
  flags identifiers *"newly introduced by the cycle."*

**A gate with two recorded live escapes was itself unverified until this bundle, and all four defects
were found by hand — by writing a failing case, not by any existing test.** That is the argument for
`audit-selftest` in one sentence: it converts "the gate is correct" from an assumption into an
assertion. Its zero catches since are exactly what a correct regression guard looks like on a bundle
that did not regress.

---

## 6. Rules

Each is tied to the evidence that produced it. Nothing here is a preference.

### 6.1 For `AGENTS.md` — durable

Verified present today: `grep -ci` on `AGENTS.md` → `worktree` 4, `CARGO_TARGET_DIR` 3, `stash` 1,
`concurrent` 1, *"red for more than one run"* 1, *"widest build scope"* 1. **`RUN_DESKTOP_AGENT` is
still 0** — but the SD-29 `loop-instruction.md` carried it, and four cycles used a unique value with
zero display collisions this tranche. Tranche/7's rule B4 was obeyed by carrying it in the *dispatch*
rather than the durable file; it should now be promoted.

| # | rule | prevents |
|---|---|---|
| **A8** | **Concurrency has a disk budget, and it is checked before the fan-out, not by each agent afterwards.** A full sweep's target dir measures ~60 G here. *N* concurrent full-gate agents need *N* × that plus headroom above the 90 % `preflight-disk` floor. If the budget does not fit, dispatch fewer. | 26 of 44 incidents (59 %); one whole kind lane, 1,696 units, never started; `reclaim.sh --apply` reclaiming 0.0 B because every candidate was correctly refused. |
| **A9** | **A blocker recorded by an agent that cannot fix it must be routed to the card that owns it before the next card is dispatched.** *"Leave it alone, it's a sibling's live work"* is the correct agent behaviour and it is not a resolution. | 3 cycles independently rediscovering the same two red count pins; the companion card never re-queued after its disk condition cleared. |
| **A10** | **A gate that has not returned is not a gate that passed, and a non-zero exit code is not automatically a gate failure.** Read the number: 143 = 128+15 = SIGTERM. Corroborate against the log's own SUMMARY block. The harness's task status reports the *wrapper*, never the gate. | `epic-4-proven-feat-race-class` (`verify_exit_code: -1`, correctly refused to claim); `epic-8-toolkit`'s exit-143 misread, where the two available summaries pointed in opposite directions. |
| **A11** | **Never `pkill -f` on a pattern naming a shared tool in a shared checkout.** Resolve to a PID first: `pgrep -af`, read the listing, `kill` by number. On a shared checkout every agent's gate has the same command line by construction. | 1 near-miss; a sibling's 45-minute gate would have died with no cause in its own log — *"indistinguishable from the harness-timeout SIGTERM this same cycle had already been fooled by once."* |
| **A12** | **Before concluding a build has stalled, check for a sibling `verify.sh`.** `pgrep -fa 'verify.sh\|cargo test'`. Frozen log timestamps and a frozen `deps/*.d` count under live `rustc` mean **starved, not hung**. | 1 incident, `silent: true`, ~15 min. AGENTS.md's one-writer rule covers uncommitted work and is silent on concurrent sweeps. |
| **A13** | **Promote `RUN_DESKTOP_AGENT` into `AGENTS.md`,** with the driver's two known defects named: `driver.sh` greps for a window titled `Codex` while the real `WM_NAME` is `codex-desktop`, and `import -window <id>` returns a blank PNG under this container's WebKit compositing path while `import -window root` on the same display at the same moment captures the painted UI. | Tranche/7 rule B4, still unshipped. Two cycles recorded DoD item 8 as unperformable on the strength of a diagnosis (*"the window never appears"*) that a third cycle disproved — the window appears; **capture** fails. |

### 6.2 For the dispatching layer — per-run

| # | rule | evidence |
|---|---|---|
| **B10** | **Cut every dispatch worktree from the branch the brief names, and have the agent assert it as step 0.** `git rev-parse HEAD` and `ls` the package directory; if the required reads do not exist, `git fetch && git reset --hard <named-branch>` on a clean tree, then proceed. | 6 cycles recorded it; worktrees were cut at `7d9f1c4f`, which is `origin/main`'s tip from 2026-06-28. |
| **B11** | **A card is COMPLETE when its work is on the named branch, not when its receipt says so.** The dispatch layer must merge or fast-forward each worktree branch before marking the card, and the successor card must verify its parent's artifacts exist by content (`grep -rn <new-symbol> --include=*.rs -l .`) rather than trusting the status. | `epic-5-monster-lane-pilot` marked COMPLETE with its whole chassis on `origin/worktree-wf_3516060a-756-9`; the extend cycle spent its entire budget discovering and integrating it. |
| **B12** | **"Ran out of turn" is not "blocked."** A halt condition must distinguish an agent that hit a real blocker from one whose budget expired mid-gate. The second is resumable and the resumption is cheap — the build cache is warm. | Run 1 halted the entire workflow at Epic 2 because `if (!ok(...)) return` saw `outcome: "blocked"` from an agent that had finished its derivation and was waiting on `root-full`. The resumed cycle re-derived rather than restarted, and every figure reproduced. |
| **B13** | **One writer per checkout, including the primary one.** Worktree isolation was granted to six lane agents and denied to Epic 8 and Epic 9, which then collided on gate attribution, on a near-fatal `pkill`, and on a push that could not be made at all. | 3 incidents; 5 green commits left unpushed. |
| **B14** | **A scoping figure that selects a pilot is verified at source, one record deep.** A unit count and a unit *kind* are two different claims and the inventory only asserts the first. | `inner_sea_intrigue` pinned as the Race-Trait pilot on 9 `race_trait` units; all 9 are construct-companion abilities. The book has zero race traits. Third bundle running that `file_kind()`'s filename typing has produced a wrong figure. |
| **B15** | **The dispatching session must emit its own retro events.** It is the only actor in the run with no instrument pointed at it, and every finding in §4.1 exists only because a dispatched agent happened to be positioned to see it. | 19 actors, 0 orchestrator events; §7's corrections to the orchestrator's own account. |
| **B16** | **Fix the two classifier regexes in the retrospective toolchain** (`find ` cannot be followed by `\b`; `ls` is missing) and give `retro.py summary` fuzzy `recurrence_key` clustering. | 10 of 34 corrections unclassified by a regex defect of the exact class the log exists to expose; one physical condition reported under three keys. |

### 6.3 Still live at closure

| status | risk | evidence |
|---|---|---|
| 🔴 | `retro.py summary` still clusters on the exact `recurrence_key` string. One disk condition reads as `disk-full` ×16, `disk-pressure` ×9, `preflight-disk-normalized-red` ×1. | `grep -n recurrence_key scripts/retro.py:667` |
| 🔴 | No red-streak assertion in `verify.sh`. A stage red on N consecutive runs is still reported identically to one red once. | `grep -n 'streak\|consecutive' scripts/verify.sh` → nothing |
| 🔴 | `driver.sh:43` `WINDOW_TITLE="Codex"`; the real `WM_NAME` is `codex-desktop`. `driver.sh screenshot` returns a blank PNG under this container's compositing path. | `sed -n '43p;138,148p' apps/desktop/.claude/skills/run-desktop/driver.sh`; `epic-4-proven-feat-race-class` incident 07:38 |
| 🔴 | The vite port (1420) is **not** partitioned per agent the way `DISPLAY` is; a second concurrent app run dies with "Port 1420 is already in use", including against one's own orphaned vite. | same incident |
| 🔴 | `RUN_DESKTOP_AGENT` still absent from `AGENTS.md` (0 mentions). It survives only as long as each bundle's `loop-instruction.md` remembers it. | `grep -c RUN_DESKTOP_AGENT AGENTS.md` → 0 |
| 🟡 | `BASELINE_ROOT_LIB_TESTS` is 1604; the tree measures **1615**. Baselines are floors so the gate passes — deliberate headroom, flagged for a successor's own DoD-item-7 commit rather than taken as a closure drive-by. | `release-notes.md` §Known issues 8 |
| 🟡 | Three real Product-Identity leaks remain in committed Pipeline B tables (Sarenrae, Jarn, Asmodeus). Epic 3 gates **new** leaks; it does not fix other bundles' records. `pi-sweep` baselines them at 10 hits / 10 rows. | `sd29-e3-provenance` deferral; `docs/governance/pi-sweep-baseline.tsv` |
| ✅ | `tests/no_foreign_home_paths.rs` caught a fresh baked-in home path on its first opportunity. Decision 40's did-not-execute check ships and reports 524/524. AGENTS.md's A5 changed agent behaviour on a normalized red, verifiably. | §5.1 |

---

## 7. Corrections to this retrospective's own inputs

Logged under `RETRO_ACTOR=tranche-9-retrospective`
(`docs/retro/events/tranche-9-retrospective.jsonl`).

### 7.1 Corrections to the commissioning brief's own figures

| input | claimed | actual |
|---|---|---|
| tranche/9 retro brief, §THE DATA | "166 events sit in `docs/retro/events/sd29*.jsonl` plus `operator-prelaunch.jsonl`" | The glob yields 166, and 166 is **not** the tranche/9 event set. It **includes 21 tranche/8-era events** (`operator-prelaunch` ×15 and `sd29-scope-and-debt` ×6, dated 2026-08-01/02, nine days before `tranche/9` existed) and **excludes 4** tranche/9 events in `codex.jsonl`. The correct set under `ts >= 2026-08-10T22:37:02Z` is **149**. The brief was right that the glob might miss shards; it missed 4, and it also over-collected 21. |
| tranche/9 retro brief, §THE DATA | 50 correction / 49 verification / 43 incident / 19 deferral / 3 near_miss / 2 rework | Those figures reproduce **exactly** for the brief's own glob, and are wrong for the tranche. Correct: **34 / 48 / 44 / 18 / 3 / 2.** The brief's caution about the glob was warranted; its caution about `repo.branch` was also warranted (73 of 149). |

### 7.2 The orchestrator's eight observations, verified

| # | claim | verdict |
|---:|---|---|
| **1** | One Workflow, 17 agents, 50 402 277 ms, 2 884 281 tokens, 2 543 tool calls; phases Preflight → Foundation → Lanes → Version → Review → Closure | **CONFIRMED exactly**, all five figures and all six phase titles, from the workflow record (`runId wf_3516060a-756`). **One refinement:** 19 agent transcripts exist for 17 labels — the two extra are the discarded Epic 2 attempts (§2). And the Lanes phase ran **8** worktree agents across the run (`wf_3516060a-756-6` … `-13`), of which six were concurrent at peak. |
| **2** | Stopped and resumed twice; (a) run 1 halted at epic-2 on a "blocked" that was a turn-budget expiry, (b) run 2 stopped ~25 min into epic-2 to insert epic-1b | **CONFIRMED.** Three epic-2 transcripts: 23:53:37→00:04:37 (11 min), 00:05:58→00:30:55 (**25 min**), 00:32:05→01:05:08. The incident `verify-full-outlasts-turn-budget` (45 min) was emitted at 00:12:07 by the second. `epic-1b-naming-sweep` is Order **2.5** on the kanban, inserted mid-run. |
| **3** | Concurrency too aggressive; suspected consequences: epic-4-frc never got an exit code, epic-7 refused at preflight, epic-5 deleted its own 27 G target dir | **CONFIRMED in outcome, with two attributions corrected.** Epic 4-frc's missing exit code is attributed by its own receipt to **turn-budget expiry mid-`root-full`**, not directly to contention (contention lengthened the build; the receipt does not claim causation). Epic 5's target-dir deletion is recorded but the **27 G figure appears nowhere in the log** — the recorded figures are a 60 G `target/` in the primary checkout and a 13 G warm cache; the log cannot support 27 G. Epic 7's refusal is confirmed verbatim. **Quantified as asked: 9 of 21 authored incidents (43 %) and 32 of 44 total (73 %) name disk, load, contention, or a gate that did not return.** |
| **4** | Harness defect recurred "at least 3 times" | **CORRECTED UPWARD — 6.** The event log carries **2** `wrong-base-worktree` incidents; the cycle receipts record the reset for **six** cycles / six worktrees (`-7` spell, `-8` feat-race-class, `-9` monster pilot, `-10` race-trait pilot, `-12` race-trait extend, `-13` monster extend). The event log **under-records its own second-largest orchestration defect by 3×.** `7d9f1c4f` is confirmed as `origin/main`'s tip (`Merge pull request #23`, 2026-06-28). The downstream `unmerged-parent-card` effect is confirmed exactly as described. |
| **5** | DoD item 8 was **blind for the entire bundle**; on-screen driving caught nothing | **CONTRADICTED BY THE LOG. This is the largest correction in this document.** On-screen driving was performed and was load-bearing in **three** cycles and it caught **three defects, being the sole mechanism in all three**: the `%1` raw-token leak (`sd29-e5-monster-pilot`, correction, `driver.sh screenshot`), the Spell Catalog chip row (`sd29-e4-spell`, `near_miss`, `escaped: true`, **already reached real users**), and the Tauri wire-contract rename (`epic-1b-naming-sweep`, live Developer panel read). `epic-4-proven-spell` committed **two 1920×1200 PNGs** to `docs/release/.../artifacts/` (`git ls-tree -l origin/tranche/9 …` → 243 444 and 242 196 bytes). Item 8 was recorded as an unperformable shortfall in exactly **two** cycles (equip, feat-race-class), and `sd29-e10-review` corrected their generalisation *"unsatisfiable for every player-visible lane in this bundle"* to **"It is satisfiable and was satisfied."** The refusal to substitute the passing desktop test stage is confirmed and is to the agents' credit. **What the bundle therefore cannot claim** is narrower than the observation states but is still real: the *equipment* and *feat* catalog widenings — 769 and 83 player-visible rows — have **no on-screen evidence**, and by this program's own §3.2 ruling they are not verified as reaching the player. |
| **5b** | The diagnosis: "the binary exits before a window appears", reproduced 3× with three env overrides | **PARTLY CORRECTED by the log's own later work.** `sd29-e4-frc` (07:38) refined it: the GTK toplevel **is** created and mapped (`xdotool search --name Codex` finds it, `xwininfo` reports `Map State: IsViewable`, 1600×1000). Two real defects: `driver.sh` searches for `WM_NAME` `Codex` while the actual name is `codex-desktop`; and `import -window <id>` returns `Resource temporarily unavailable` / a 377-byte blank PNG while a root capture at the same moment succeeds. *"So the remedy is not 'wait longer' or 'unload the box' as the prior cycle guessed."* |
| **6** | epic-6 found its own pilot book wrong; `inner_sea_intrigue` carries zero genuine race traits, all 9 units are construct-companion abilities from `isi_abilities_race_companion.lst` | **CONFIRMED verbatim**, including the file name and the mechanism (`file_kind()` types by basename; `_abilities_race` was tested before the companion/familiar markers). One addition the observation omits: the same cycle found a *second*, larger figure defect — grounded race traits **44 → 21**, because §9.3's "name-coincidence false positives" were 23, not 4, with 19 of them **intra-`core_essentials` cross-race** collisions the doc never enumerated. |
| **7** | Harness reported a SECURITY WARNING on epic-2-prelaunch with no reason given | **CONFIRMED as reported; inspection found nothing improper.** See §7.3. |
| **8** | Epic 1 found the audit implemented 3 of 4 named patterns; Epic 1b found it caught no path tags and neither the PascalCase nor infix form; both caught by hand, not by a test | **CONFIRMED**, with the count raised: Epic 1b found **three** escape classes, not two — the **GE-NN family was absent entirely**, in addition to the infix and path-tag classes. `bash scripts/tests/test_identifier_discipline_audit.sh` → 16 passed / 10 failed before the fix. A **fourth** defect followed in the same cycle: the audit scanned the whole unified diff including deletions, so it failed the one cycle whose purpose was removing tags. |

### 7.3 The epic-2-prelaunch security warning — what inspection found

The warning is real and is recorded verbatim in the workflow record's `logs` array:

> `[epic-2-prelaunch] SECURITY WARNING: This subagent performed actions that may violate security
> policy. Reason: No reason provided. Review the subagent's actions carefully before acting on its
> output.`

**What was inspected.** All three epic-2 transcripts (`agent-af22c2bfdc9e05772`,
`agent-aba4a626591cd7423`, `agent-a5037b0df196960c4` — the last being the one the workflow record
attributes to the label), read exhaustively: **every** `Bash` command, `Write` and `Edit` target
across the three (40, 24 and 24 tool calls respectively). Plus the one commit attributed to
`sd29-e2-prelaunch` (`77997dc7`), by name-only file list.

**Finding: nothing improper.** The commit touches six paths, all documentation or generated
artifacts: four SD-29 package docs, the agent's own retro shard, and a one-line `generated_at` change
in `docs/work-inventory.json`. No source, no config, no credential store, no `settings.json`, no
network egress beyond `git push origin tranche/9` and `git ls-remote`. No `sudo`, no `chmod`, no
`curl`/`wget`, no base64, no `.ssh`, no secret material. A keyword sweep of all three transcripts for
`security|violat|credential|secret|password|.ssh|curl|wget|base64|chmod|sudo|settings.json|.env`
returns only (a) the standing skill catalogue in the system prompt, (b) `AGENTS.md`'s own
hard-stop list quoted back, and (c) the pre-launch checklist item *"OAuth credentials valid for
`git push`"*. **There is no trace of the warning inside the agent's own transcript.**

**Four behaviours are unusual enough to name as candidate triggers, none of them improper:**

1. All three attempts `cd`'d **outside the granted repo** into
   `/home/ubuntu/workspace/repos/pcgen/data/...` to read the PCGen corpus. Reads only, and the corpus
   is this project's source of truth — but it is outside the write scope the brief names.
2. Attempt 2 created a `CARGO_TARGET_DIR` at **`/home/ubuntu/cargo-targets/sd29-e2-prelaunch`** —
   `mkdir -p` in the user's home directory, outside both the repo and the scratchpad. (The directory
   still exists.)
3. Attempt 2 used `nohup ./scripts/verify.sh … &` to **detach a process that outlives the agent turn**,
   then polled with `pgrep -f "scripts/verify.sh"` — process inspection across other agents.
4. Attempt 3 ran `scripts/reclaim.sh --apply` (which deletes cargo target dirs, verify logs,
   worktrees and merged branches box-wide) and `rm -rf /tmp/codex-target-sd29-e2-prelaunch`.

Items 1 and 4 were done by nearly every cycle in this run and drew no warning, which weakens them as
explanations. Items 2 and 3 are, as far as this inspection can tell, **unique to this agent**.

**The honest limit of this finding:** the harness supplied no reason, the classifier's input is not
recoverable, and no correlate in the transcript distinguishes this agent from its siblings with
confidence. **I could not clear it and I found nothing to substantiate it.** The agent's output —
`corpus-shape-37-books.md` — was independently re-derived by its own closing pass across 12
spot-checks with zero disagreements, so the *work product* is verified on its own merits regardless
of the warning. That is stated as what it is: verification of the output, not exoneration of the
process.

**Recommendation:** a harness warning with `Reason: No reason provided` is not actionable and should
not be treated as either signal or noise. If it recurs, capture the classifier's input at emission —
otherwise it will keep costing an inspection like this one and keep ending here.

### 7.4 Corrections to this document's own method

| input | claimed | actual |
|---|---|---|
| tranche-7 §3's detection classifier, applied here | classifies each correction's `verified_by` | **Misses 10 of 34.** `\bfind \b` cannot match `find ~/...` (`~` is not a word character), `ls ` is absent entirely, and a shell self-test harness matches nothing. §3.1 reports both the verbatim and the amended run. The classifier that measures how errors are caught has an error of exactly the class it measures. |
| this document's §2.1 self-correction regex | 9 self-corrections | **6.** Three are substring false positives (an audit script's "its own header"; a governance doc's "its own five-minute manual sweep"; a dispatch brief carrying the actor's name). Stated so the 17.6 % figure can be disputed. |
| `retro.py summary` | dominant incident class `RECURRING x22 disk-full` | Correct as a key tally and wrong as a class size. The disk condition spans **three** keys totalling **26 of 44** within the tranche/9 window. Same defect tranche/7 §4 recorded, at reduced magnitude. |

---

## 8. The five sentences worth carrying forward

1. **Cheap counting is still, by a wide margin, the best guard you have** — 47 % of corrections under
   tranche/7's own classifier, 65 % under a fixed one, sole-sufficient in 15 to 20 cases. Two tranches
   now agree to within one point. The bottleneck has never once been tooling.
2. **"It reaches the player" is verified on screen or not at all.** The share fell from 14 % to under
   6 %; the unique competence did not move. Three defects this tranche were invisible to every test in
   the repo, and one of them had already shipped — because the frontend test's oracle was a **copy of
   the constant under test.**
3. **Concurrency is a resource decision and it has a body count.** Six worktree agents on a two-core
   box filled a disk; a preflight gate correctly refusing to run on that disk cost SD-29 an entire kind
   lane — 1,696 units, 0 grounded — because **nothing re-queued the card once the condition cleared.**
4. **A card is done when its work is on the named branch, not when its receipt says so.** A chassis
   marked COMPLETE sat unmerged on a worktree branch and consumed its successor's entire budget.
5. **Every figure derived on an unfetched ref is fiction, and it happened three times in one cycle** —
   including an "impossibility" that propagated through two further receipts while the evidence
   disproving it sat in their shared git ancestry.

---

## 9. The forward work

Four things SD-29 did not land. All four are `decision-blocked` or unstarted **by ruling, not by
oversight**, each with a receipt and a `deferral` event, and each is stated in
`release-notes.md §Known issues`.

### 9.1 The companion lane — 1,696 units, 0 grounded, never started

**State: READY, unclaimed, unblocked.** `epic-7-companion-lane-pilot` refused at Cycle-mechanics step
1c: `verify.sh --only preflight-disk` exit 1, **twice** (91 % used / 47 G free), with
`reclaim.sh --apply` in between which correctly reclaimed ~1 MB. The agent left the card `READY`
rather than parking it `IN-FLIGHT` under an agent that did no bounded work. Its dependent
`epic-7-companion-lane-extend` (Order 12) was consequently never eligible; no cycle claimed it and no
work was attempted.

- Corpus-wide: **1,696** `companion` units, all `not-ingested`/`not-started`, **0 grounded**
  (re-derived at closure from `docs/work-inventory.json`).
- The pilot scope as pinned: `inner_sea_combat`, 10 units, plus the mechanism build.
- **The blocker has cleared** — 80 % used / 97 G available at closure.
- **This is a ready re-dispatch, not a corpus finding.** Successor needs only to run it when the box
  is not carrying six concurrent worktrees.

### 9.2 Monster / monster-ability extend — chassis proven, ingest per book

**State: PARTIAL. `epic-5-monster-lane-extend`, ingest `decision-blocked`, re-dispatch per book.**
What landed: the pilot chassis is merged onto `tranche/9` and the denominators are re-derived. What
did not: **1,210 monster + 3,090 monster_ability units across 23 books** (bestiary 284/523,
bestiary_2 316/466, bestiary_3 261/40, bestiary_4 220/768, inner_sea_bestiary 40/190, …).

- **Nothing technical blocks it.** The deferral says so explicitly: *"the chassis is now on the branch
  and generic. The blocker was integration, not capability."*
- **Do not extrapolate a per-unit rate from the pilot.** Essentially all of the pilot's cost was the
  once-per-*kind* chassis; the next book inherits it. Re-dispatch **one card per book**, highest
  density first: `bestiary`, `bestiary_2`, `bestiary_4`, `bestiary_3`, `inner_sea_bestiary`.
- **One decision is owed before the dice can ground:** 13 of the pilot book's 14 named natural
  attacks carry no damage dice anywhere in the corpus. Bestiary 1 closed the same gap by grounding
  dice from published text (`beastiary1::natural_attack_provenance`); a successor must be explicitly
  funded to do that, or accept `damage_dice: None`. **No dice were invented.**
- `monster_codex` (207/207 units, entirely not-started) is the gating book for retiring the
  `beastiary1/race_traits` `OPEN_FINDINGS` entry — its closure mechanism
  (`tests/sd27_duergar_invisibility_sla_is_upstream_blocked.rs` going red) is intact and untouched.

### 9.3 Race-trait ingest — ceiling-blocked, and the ceiling is named

**State: PARTIAL ×2. Both classifier halves COMPLETE and shipped; both ingest halves
`decision-blocked`.** This is the most important distinction in the forward work: **it is not
effort-blocked, it is ceiling-blocked**, and the ceiling is outside SD-29's epic structure entirely.

- `crb::race_traits()` models exactly **7** races, hardcoded, 49 rows.
- Of **3,447** `race_trait` units, **805** carry `race_trait_race_not_modelled` and **144**
  `race_trait_absent_from_race_traits`. **21 are grounded** (down from a falsely-reported 44).
- **No book's race traits can ground until a real race chassis lands** — races plus their trait
  tables in `src/rules_core`. **No bundle owned that work at SD-29 closure.**
- The pilot card additionally needs an **operator re-pin**: `inner_sea_intrigue` has zero genuine race
  traits. The deferral names re-derived candidates — `ultimate_intrigue` (3), and the other smallest
  genuinely-race-trait-bearing books.

### 9.4 Epic 8 → the C3.1 retrofit

**State: DECISION-BLOCKED, ruled under UNATTENDED MODE item 4, tracked at
`successor-forward-scope-register.md C3.1 (ACTIVE)`.** Epic 8's own in-scope criterion — *"a lane
cycle needed the consumer surface to satisfy its reach claim"* — was **tested and unmet**: both reach
claims Epic 5's pilot landed assess the already-shipped `list_monster_catalog`, and **zero** assess a
toolkit surface. The DM Toolkit therefore does not land inside SD-29.

- **Nothing technical blocks it:** the engine half (`src/rules_core/encounters.rs`, `party_cr.rs`)
  shipped under SD-22 and is untouched.
- What is owed: a Tauri command over those modules and a real DM Toolkit screen replacing
  `CharacterHubPage.tsx`'s `StubScreen`, consuming Epic 5's monster/monster_ability records.
- Revisit condition, as written: a successor bundle named for the C3.1 retrofit, **or** a later lane
  cycle producing a record family whose only viable player surface is GM-side.

### 9.5 Smaller items a successor should not rediscover

| item | state |
|---|---|
| **Equipment and feat catalog widenings have no on-screen evidence** (769 + 83 player-visible rows) | §3.2's ruling applies: not verified as reaching the player. Cheapest possible successor task, blocked only on `driver.sh`'s capture defect. |
| `class_feature` Tier-3 deferral, **15,472 units** | Deferred by `decisions.md §38.4` / `successor-forward-scope-register.md C1.3`; owned by SD-30's class_feature/archetype bundle. |
| Spell-list tables for 6 books with a compiled rule set but no `spell_list` table | `ultimate_magic` (269 + 19 `.COPY=`), `ultimate_combat` (147), `core_essentials` (110 + 1), … The lane now starts from a correct **1,561**, not an inflated 1,754. |
| `reconciliation` empty for **24 of 37** books | The inventory computes it only for the 13 its own `scope` field calls `in_scope`. **A missing `reconciliation` must not be read as "no delta."** Each lane derives its own. |
| Three real PI leaks in committed Pipeline B tables | Owned by the bundles that landed them (SD-27/SD-28), not by Epic 3. |
| 531 tagged `tests/*.rs` filenames; 174 tagged `docs/` paths | Deliberately out of scope — the documented exclusion class (SD-25 1.1). Renaming them obliges rewriting cited prose tree-wide. |
| `apps/desktop/src/sd16/`, `src/sd21/` directory names | Pre-existing from SD-16/SD-21; `releaseChecks/` is the naming precedent. Revisit: whichever bundle next owns `apps/desktop/src/` structure. |
| Duplicate `buildVersionTriple.test.ts` (`src/release/` 120 lines vs `src/releaseChecks/` 51) | Dedupe deferred past Epic 10; the `release/` copy is the fuller original. |
| `BASELINE_ROOT_LIB_TESTS` 1604 vs measured 1615 | Headroom, not drift. Flagged so a successor re-pins deliberately in its own `--show-actuals` commit. |
