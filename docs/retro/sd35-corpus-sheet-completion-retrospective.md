---
canonical: true
owner: AT-35-E7-002
purpose: SD-35 retrospective, grounded in the 731-event retro log and in re-derivable commands rather than recollection.
date: 2026-09-15
board: 26,123 of 49,438 DONE at the tranche/15 cut -> 49,450 of 49,450 DONE at closure
bundle: docs/release/SD-35-corpus-sheet-completion/
cited_from: docs/release/SD-35-corpus-sheet-completion/references/README.md
---

# SD-35 retrospective — the corpus sheet-completion bundle

Seven epics, 27 criteria, 127 cycle receipts, eight days. The board moved **52.84% → 100.00%**
of a population that itself grew by 12 units mid-run. Every number below comes from
`docs/retro/events/` (731 events in the window), from `scripts/retro.py summary`, or from a
command printed beside it. Nothing here is recalled.

```
EVENTS  731   (since 2026-09-08, 77 shards, 0 invalid lines)
   225  correction      201  verification
    93  deferral         90  resolution
    75  incident         41  note            6  rework

origin: 501 agent, 230 derived
git join: 346 commits, 1 author, 2.11 events per commit
verification fail rate  13.93%  (28 failed runs of 201)
incidents with a recurrence key  13 distinct keys recur; 75 incidents total
SILENT failures (plausible wrong output)   7
recorded time lost   620 minutes = 10.3 hours
deferrals  93 raised, 75 resolved, 18 open at the final scan
```

Re-derive the whole block:

```
python3 scripts/retro.py summary --since 2026-09-08 --json
```

---

## Figures + their re-derive commands

| figure | value | command |
|---|---|---|
| board at the cut | 26,123 DONE of 49,438 | `git show 4c6c57eb9f:docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` |
| board at closure | 49,450 DONE of 49,450 | `python3 scripts/completion_atlas.py --check` |
| corpus-side headline | 48,864 of 48,864 real `data/corpus` rules records | cited from `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-7-closure/population-census-final.json` |
| rules written | 70,317 rules over 49,450 unit files | `python3 -c "import json;print(json.load(open('data/sheet_rules/_report.json'))['rules_written'])"` |
| refused tokens | 0 refused of 49,450 records | `python3 scripts/token_coverage.py --check` |
| live PCGen residue | 0 live files, 0 live hits | `python3 scripts/pcgen_residue_gate.py --check --closure` |
| build time before / after | 188.97 s -> 145.89 s, cold, paired | `python3 -c "import json;d=json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-1-tax-cut/build-time.json'))['paired_rerun'];print(d)"` |
| full gate | 49 of 49 stages PASS | `bash scripts/verify.sh` |

---

## 1. The batch floor worked, and the distribution proves it was the only thing that could have

`decisions.md §2` set a floor of 500 units per cycle and required the closure to be a script, not
a per-unit pass. Measured against the scope gate's own `closed=` row on all 127 receipts:

```
python3 - <<'PY'
import glob,re,statistics
v=[]
for f in glob.glob('docs/release/SD-35-corpus-sheet-completion/artifacts/*/*receipt*.md'):
    m=re.findall(r'closed=(\d+)',open(f,errors='replace').read())
    if m: v.append(max(int(x) for x in m))
v.sort(); print(len(v), 'receipts; min',v[0],'median',statistics.median(v),'max',v[-1])
PY
```

**units per cycle: min 0, median 0, max 21,911**, over 127 receipts.

That median is not an embarrassment, it is the shape the sheet rule produced. Five cycles of 127
closed anything at all:

| cycle | units closed |
|---|---|
| `AT-35-E2-005` (the converter run over the whole remainder) | 21,911 |
| `AT-35-E2-005-DISPOSITION` (the same work, re-stated by its disposition receipt) | 21,911 |
| `AT-35-E3-002` | 786 |
| `AT-35-E3-004` | 786 |
| `AT-35-E3-001` | 618 |

**One cycle closed 21,911 of the 23,315 units that were not DONE at the cut.** The other 122
cycles built the mechanism, proved it, audited it, or cleaned up after it. SD-34's waves 43–48
closed 2–45 units per build; SD-35's Epic 2 closed 21,911 in one. The floor did not make cycles
bigger — it made the *mechanism* corpus-wide, and the unit count then fell out of it for free.

The lesson is not "batch more". It is: **when a cycle's cost is dominated by the build and the
audit rather than by the units, the only number worth optimising is the number of cycles.**

## 2. Lines per unit: the sheet is thin, and that is the finding

A "unit" is one corpus record. A "line" is one rendered sheet rule. Over the whole shipped
package:

```
python3 - <<'PY'
import json,os,collections,statistics
n=[]
for b in os.listdir('data/sheet_rules'):
    p='data/sheet_rules/'+b
    if not os.path.isdir(p) or b.startswith('_'): continue
    for k in os.listdir(p):
        q=p+'/'+k
        if not os.path.isdir(q): continue
        for fn in os.listdir(q):
            if fn.endswith('.json'):
                r=json.load(open(q+'/'+fn)); n.append(len(r) if isinstance(r,list) else 1)
n.sort(); h=collections.Counter(n)
print('units',len(n),'rules',sum(n),'mean %.2f'%statistics.mean(n),'median',statistics.median(n),'max',n[-1])
for k in range(1,11): print(k, h[k], '%.1f%%'%(100*h[k]/len(n)))
print('>10', sum(c for k,c in h.items() if k>10))
PY
```

| lines | units | share of 49,450 |
|---|---|---|
| 1 | 44,840 | 90.7% of 49,450 |
| 2 | 1,302 | 2.6% of 49,450 |
| 3 | 701 | 1.4% of 49,450 |
| 4 | 472 | 1.0% of 49,450 |
| 5 | 407 | 0.8% of 49,450 |
| 6 | 344 | 0.7% of 49,450 |
| 7 | 376 | 0.8% of 49,450 |
| 8 | 297 | 0.6% of 49,450 |
| 9 | 229 | 0.5% of 49,450 |
| 10 | 132 | 0.3% of 49,450 |
| >10 | 350 | 0.7% of 49,450 |

**mean 1.42, median 1, p95 4, p99 9, max 201.** Nine records in ten print exactly one line.

This is the number that should have been measured at scoping and was not. The SD-34 fable review
sized the work by *token vocabulary* (189 types, 28 covering 90%) and was right about the cost
driver. But the **output** side — how much sheet a record actually produces — was never measured
until the package was built. Had it been, the single-line dominance would have made the
"render it, do not simulate it" ruling obvious a bundle earlier: you do not build a simulation
engine to print one line.

The 201-line tail is real and is not a defect: a handful of class-progression records legitimately
carry one rule per level.

## 3. `Words` is the majority form — the sheet rule's central claim, measured

`decisions.md §1`: a term that cannot be resolved to a final number stays as the rule's words.
Critics of that ruling expected `Words` to be a small escape hatch. It is the **majority**.

```
python3 - <<'PY'
import json,os,collections
pk=collections.defaultdict(collections.Counter)
for b in os.listdir('data/sheet_rules'):
    p='data/sheet_rules/'+b
    if not os.path.isdir(p) or b.startswith('_'): continue
    for k in os.listdir(p):
        q=p+'/'+k
        if not os.path.isdir(q): continue
        for fn in os.listdir(q):
            if not fn.endswith('.json'): continue
            r=json.load(open(q+'/'+fn))
            for x in (r if isinstance(r,list) else [r]):
                v=x.get('value')
                pk[k]['Words' if v in ('Text',None) else list(v)[0]] += 1
for k in sorted(pk,key=lambda k:-sum(pk[k].values())):
    c=pk[k]; t=sum(c.values())
    print('%-20s %6d %6d %5.1f%%'%(k,t,c['Words'],100*c['Words']/t))
PY
```

| kind | rules | `Words` | `Words` share |
|---|---|---|---|
| deity | 459 | 459 | 100.0% of 459 |
| language | 136 | 136 | 100.0% of 136 |
| domain | 185 | 180 | 97.3% of 185 |
| spell | 3,104 | 2,797 | 90.1% of 3,104 |
| power | 447 | 397 | 88.8% of 447 |
| feat | 2,904 | 2,473 | 85.2% of 2,904 |
| monster_ability | 4,601 | 3,621 | 78.7% of 4,601 |
| equipment_modifier | 1,660 | 1,282 | 77.2% of 1,660 |
| class_feature | 20,943 | 15,503 | 74.0% of 20,943 |
| equipment | 7,322 | 4,754 | 64.9% of 7,322 |
| ability | 5,285 | 3,243 | 61.4% of 5,285 |
| race_trait | 3,000 | 1,831 | 61.0% of 3,000 |
| race | 136 | 81 | 59.6% of 136 |
| trait | 546 | 316 | 57.9% of 546 |
| template | 3,068 | 1,066 | 34.7% of 3,068 |
| companion | 4,751 | 946 | 19.9% of 4,751 |
| skill | 380 | 55 | 14.5% of 380 |
| class | 837 | 82 | 9.8% of 837 |
| monster | 10,553 | 719 | 6.8% of 10,553 |
| **all kinds** | **70,317** | **39,941** | **56.8% of 70,317** |

`Number` 27,198, `Dice` 3,173, `DiceBySize` 5 make up the rest.

Two things follow.

**The split is by kind, not by book.** `monster` is 6.8% words because a stat block is numbers;
`deity` is 100% words because a deity's entry *is* prose. The kind-lane refactor proposal
(`docs/retro/kind-lane-refactor-proposal.md`) said book was never a real partition boundary. This
table is the strongest evidence yet: the variance across kinds is 93 points, across books it is
noise. SD-35 dispatched mechanism-wide for exactly this reason and should have been the last
bundle that had to argue the point.

**423 records converted with a degraded term** (`data/sheet_rules/_report.json`
`degraded_records`) — a `Number` whose expression could not resolve, printed as words instead.
That is 0.86% of 49,450 and it is the sheet rule *working*, not failing. The converter does not
refuse; it degrades and still renders.

## 4. Build time: the tax cut paid, and one of its four measurements was a lie

Epic 1 folded 184 templated integration-test files into two binaries.

| run | tree | wall | conditions |
|---|---|---|---|
| 1 | before | 164.49 s | quiet at start |
| 2 | after | 295.95 s | **another lane's full build running, load 40 on 24 cores** |
| 3 | before (paired) | **188.97 s** | quiet, back-to-back with 4 |
| 4 | after (paired) | **145.89 s** | quiet, back-to-back with 3 |

**Before 188.97 s -> after 145.89 s, −43.08 s, −22.8%**, cold, `CARGO_INCREMENTAL=0`, both halves
on the same quiet box. Test binaries 544 -> 362 (−182 links); the test *entries* are byte-identical
at 8,723 with `0` only-in-before and `0` only-in-after.

```
python3 -c "import json;print(json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-1-tax-cut/build-time.json'))['paired_rerun'])"
```

The instructive part is run 2. Taken alone it says the consolidation made the build **80% slower**,
and it was measured honestly, on the right trees, with the right command. It was wrong because the
box was contended. The lane kept it in the artifact, labelled it `NOT comparable`, and paired a
re-measurement instead of deleting it.

**Lesson, and it generalises past this repo: a wall-clock measurement on a shared machine is not a
measurement until it is paired.** Four runs were needed to get one honest number, and the
three-run-shorter version of this story would have shipped a false regression.

A second figure worth carrying: the operator's original complaint was a **70-minute** full suite.
The compile half is 2.5 minutes. The rest is execution (37.6 min post-fold, single observation).
The tax cut was aimed at the wrong half of the number, and still helped. Nobody measured which
half before choosing the fix.

## 5. PCGen residue per epic — 260 live files to zero, and all of it in one epic

`decisions.md §11`: PCGen is a converter input and a test oracle; nothing on the live side
(`src/rules_core` minus `cache_gen`, `src/saved_character`, `src/campaign`, `src/homebrew_authoring`,
`apps/desktop`) may read a PCGen token, formula string, or `raw_tokens`.

```
python3 scripts/pcgen_residue_gate.py --check --closure
```

| epic | `live_files` at first receipt | at last receipt | moved |
|---|---|---|---|
| 1 — tax cut | 260 | 260 | 0 (the gate itself was built here) |
| 2 — sheet rule | 260 | 253 | −7 |
| 3 — place and surface | 260 | 253 | −7 |
| 4 — resolve and verify | 260 | 253 | −7 |
| 5 — residues | 260 | 253 | −7 |
| 6 — PCGen exit | 254 | **0** | **−254** |
| 7 — closure | 0 | 0 | 0 |

Epics 2–5 ran against the same tree and share one −7: the seven files that came out in the Epic 2
wrap-up correction. **Epic 6 did 100% of the removal**, over 67 of the bundle's 127 cycle receipts,
in a long monotonic descent: 254 → 208 → 197 → 81 → 45 → 25 → 16 → 8 → 4 → 0. It never rose.

```
python3 - <<'PY'
import glob,re,os,collections
per=collections.defaultdict(list)
for f in sorted(glob.glob('docs/release/SD-35-corpus-sheet-completion/artifacts/*/*receipt*.md')):
    v=re.findall(r'pcgen_live_files=(\d+)',open(f,errors='replace').read())
    if v: per[f.split('/')[-2]].append(int(v[0]))
for e in sorted(per): print(e, 'receipts',len(per[e]), 'first',per[e][0], 'last',per[e][-1])
PY
```

```
grep -ho 'pcgen_live_files=[0-9]*' docs/release/SD-35-corpus-sheet-completion/artifacts/*/*receipt*.md | sort -u
```

SD-34's `content-unit-inventory.md §6` sized this at **78 files by coarse grep**. The gate,
built in Epic 1 with an *empty and pinned* `EXCLUDED_PREFIXES`, found **260**. The estimate was
low by 3.3x, and the estimate was the one everyone had been planning against.

**A grep written to find a problem and a gate written to refuse a merge are not the same
instrument.** The gate's pattern list was widened twice during the run (`raw_bonus_chains`,
`pcgen_import`) and narrowed **zero** times — which is the only property that makes its terminal
`0` mean anything.

And the tool side survived: `src/pcgen_import` went **262 → 1,486** function bodies,
`src/oracle_validation` 46 → 66, `scripts/oracle_harness` 15 → 76 python defs. **Zero net
deletions.** Starfinder starts on that asset.

---

## What the data says, before any interpretation

### 6. Our own documents are still the most frequently wrong thing in the program

**225 corrections.** SD-31 found the repeat offenders were written artifacts, not code and not
people. SD-35 did not fix that; it got better at catching them:

```
caught before  implementation 28   merge 16   release 6   brief 3
               nothing (already shipped) 1
```

**One of the 57 corrections that record where they were caught reached a shipped state.** (The
other 168 of 225 leave the field empty, so 57 is the honest denominator here, not 225.) But the
repeat subjects are, again, prose:
`dispatch prompt` (2), `sd35-dispatch-brief:AT-35-E6-003-SWEEP` (2), `SD-35 dispatch prompt for
AT-35-E6-003-SWEEP` (2), `epic-breakdown.md AT-35-E1-004 criterion text` (2),
`epic-breakdown.md AT-35-E2-001 evidence` (2).

**Three of the ten repeat-corrected subjects are the same dispatch prompt.** A prompt is written
once, read by one agent, and never tested. It is the highest-leverage untested artifact we have.

### 7. The gates that failed are the gates that watch prose

Of 28 failed verification runs in 201:

```
figure-provenance             14
site-dashboard-check           8
shape-engine-boundary-selftest 6
denominator-gate               2
reachability-audit-selftest    2
clippy / desktop / root-full / pcgen-residue-gate / stage 0   1 each
```

**Half of all gate failures were the figure-provenance gate** — a gate whose entire job is to
refuse a number that carries no command to re-derive it. It is doing real work, and the thing it
keeps catching is us.

The 4-firing recurrence key `figure-provenance-command-on-next-line` is the sharpest form of it:
the command *was* written, one line below the figure, where the gate cannot see it. Four separate
cycles made the same formatting mistake. That is a missing affordance (a receipt template), not
four careless agents.

**This retrospective's own cycle caught a fifteenth instance of it, in the final-acceptance scan's
own entry.** `AT-35-E7-001` cycle 4 reported `denominator-gate files_checked=360 violations=0` from
a `verify.sh` run at `7753c29915`, then wrote its `progress.md` entry at `17ea4c1595` — *after*
that run — wrapping `48,864 of 48,864 real data/corpus rules records` and `= 100%` onto two lines.
The denominator was present and correct; the line break hid it from the gate, and the next
`--check` read `violations=1`. Rewrapped here, no figure altered
(`correction 1789503578833-at-35-e7-002-23ccb2`). **The last commit of a closure cycle is written
after its own gate run, so the gate never sees it.** That is a hole in the cycle contract, not a
mistake by the scan.

`root-full` failed **once**, and that once was PCGen's own gradle build failing to reach
`api.adoptium.net`. It took **three** cycles of the final-acceptance scan to classify it, because
cycle 3 correctly refused to call it environmental on a green re-run alone and demanded the cause.
That refusal was right and cost about a day.

### 8. `wrong-base-worktree` fired ten more times, and the one-line fix is still unwritten

```
wrong-base-worktree                          10
epic-wrapup-gate-red                          7
disk-full                                     6
site-dashboard-json-stale-after-inventory-move 6
untracked-worktrees-dir-on-shared-checkout    5
figure-provenance-command-on-next-line        4
retro-actor-lost-between-bash-calls           4
unfolded-gate-artifacts-die-with-the-worktree 3
```

`AGENTS.md` rule 8 says a warning carried forward is a missing mechanism. SD-31 recorded
`wrong-base-worktree` at 27 firings and found the fix. It fired **10 more times** in SD-35.

`untracked-worktrees-dir-on-shared-checkout` reached its **8th consecutive carry** with the fix
written out in the incident text — *a `.worktrees/` line in `.gitignore`* — and never applied,
because `.gitignore` was outside every epic's granted file-touch set. Eight cycles each correctly
refused to exceed their write scope, and the aggregate effect was that a one-line fix went
unwritten for the length of a bundle. **Per-cycle scope discipline and program-level mechanism
debt are in direct tension, and nothing in the workflow resolves it.** That is an operator
decision, and this retrospective is the place it should be made.

`retro-actor-lost-between-bash-calls` (4) has a mechanical cause: `RETRO_ACTOR` is exported in one
`Bash` call and the next call starts a fresh shell. At least one derived event in this bundle is
filed under the wrong actor because of it, and the cycle that found it committed it as-is and
corrected it with an event rather than rewriting the log. That is the right handling and it is
also why `sd31-transcribe` shows 163 events in an SD-35 window.

### 9. Seven silent failures, and the ones that mattered were equality pins

`silent 7` — failures that produce plausible wrong output rather than an error. The two costliest
were both **hand-maintained equality pins on a growing population**:

- `test_shape_engine_boundary.py` pinned `magnitude_bearing == 26396`; the live value was 26,397.
  Fixed not by writing 26397 but by splitting the assertion into a **floor** (`>= 26396`,
  population only grows) plus a **second implementation** (complement recount, no number pinned),
  and mutation-proving both directions red.
- `reach_gate.rs::BARE_RECORD_FINDINGS` held 22 entries for records that had since gained real
  fields. The gate's own test said *delete them*. They were deleted; **no assertion was widened.**

**An equality pin on a population that grows is a guaranteed future silent failure.** It is the
same defect SD-34 carried for six waves at `9475`. Three bundles in a row have now paid for it.

### 10. Rework: six events, and half were a dispatcher re-running a finished cycle

Only **6 rework events** in 731 — the lowest ratio of any bundle so far. But two of the six share a
cause worth naming: a criterion returned `status: partial` on a bar its own rule made unreachable
(zero new mappings), and **the dispatcher's remainder loop re-dispatches on `partial`**. The
receipt and the dispatch prompt both cited the §8 rule that should have stopped it. Nothing
mechanical did.

The other named cause is the sharpest single-line lesson of the bundle:

> `git mv` stages a moved file at move time; path repairs made to that file **afterwards** stayed
> unstaged, and the cycle's explicit-path `git add` list (shared-checkout discipline forbids
> `git add -A`) named only paths it had not already staged.

A commit therefore shipped moved modules without their own intra-crate path fixes and **did not
build**. The shared-checkout rule that forbids `git add -A` is correct and it has this exact sharp
edge. The mechanism is `git status --porcelain` **after** staging, not only before.

---

## What worked

- **One mechanism, corpus-wide.** 21,911 units in one cycle. `decisions.md §2`'s floor is the
  single highest-value ruling in the package.
- **The sheet rule ended the argument about reachability.** Under "render the rule's words", there
  is no such thing as a record the engine cannot model — only a record that prints prose. The
  atlas's `X` (deferred-with-reason, 168) and `Z` (not-started, 19) buckets both went to **zero**,
  and no carve-out list exists anywhere in the closure instruments (`EXCLUDED_PREFIXES` is `()`).
- **The residue gate was built before the removal, not after.** It is why `260 -> 0` is a
  measurement instead of a claim, and why the widening-only property is checkable.
- **The final-acceptance scan failed three times and was right each time.** It refused a green
  tick on an environmental classification, refused a differently-named file as a named deliverable,
  and refused a manifest one population behind. Every one of those was closed at source.
- **Gates were mutation-proved, not just run.** Five instruments had probes planted, caught, and
  removed with the tree byte-identical afterwards.
- **`builds_recorded` is a measured count of compile sessions, not a self-report.** 73 receipts read
  `0` (doc-only cycles), 17 read `1`, 36 read `2`–`6`. The overruns are visible precisely because
  the figure cannot be gamed, and one lane wrote "this is a real overrun of §3's one-build target"
  into its own receipt.

## What did not work

- **The isolated wrap-up gate worker cannot perform the worktree sweep it is assigned.**
  `workflow-instruction.md §10 step 2` gives the sweep to a read-only worker that is structurally
  incapable of `git worktree remove`. It failed this way in Epics 2, 3, 4, 5 and 6 — **10 open
  deferrals, one per attempt** — and it failed again in this cycle (see the sweep section).
- **Gate artifacts died with their worktree, three times.** A worker's report and retro shard lived
  only inside `.claude/worktrees/`, and nothing mechanical checked the hand-off landed. The first
  correction cycle folded 6; a later one found **5 more still unfolded**; this cycle found **6 more
  still unfolded** after that. Three independent sweeps, each believing itself complete.
- **`epic-wrapup-gate-red` fired 7 times**, every time on a stale hand-maintained constant.
- **The population moved under the instruments mid-run.** Ruling B18 admitted 12 records; the
  completion manifest kept reporting 49,438 for two cycles and a stratified sample was drawn from
  the stale figure. `49,438` is superseded everywhere and is exactly the sort of number that
  propagates.
- **A `partial` status with no mechanical stop re-dispatched a finished cycle.**

---

## Changes for SD-36

| change | where it must be enforced |
|---|---|
| Move the worktree sweep off the isolated gate worker onto a committing agent on the shared checkout, **and** add a fold-first sub-step that runs before any removal is attempted | `workflow-instruction.md §10 step 2` |
| Make the harness permission rule for `git worktree remove` an explicit launch precondition, or accept that worktrees accumulate and budget the disk | launch checklist; `decisions.md` |
| Ban hand-maintained equality pins on growing populations: a count assertion is a **floor plus a second implementation**, both mutation-proved | `AGENTS.md`; the gate self-tests |
| Give the receipt a template whose figure rows put the command **on the same line** — the figure-provenance gate's 14 failures are a formatting affordance, not 14 mistakes | `workflow-instruction.md §7` |
| Export `RETRO_ACTOR` in every dispatch prompt's *first* command of each `Bash` call, or pass `--actor` explicitly on every `retro.py` invocation | dispatch prompt template |
| Carry a program-level mechanism-debt list that a cycle may write to even when the file is outside its epic scope (starting with `.gitignore`'s `.worktrees/` line, 8 carries) | `decisions.md`; `forward-scope-register.md` |
| Stage-then-verify: `git status --porcelain` **after** the explicit `git add`, because `git mv` pre-stages and later repairs to the same file do not follow | shared-checkout discipline, `AGENTS.md` |
| Make `partial` a terminal status the remainder loop does not re-dispatch when the criterion's own rule makes its population unreachable | dispatcher |
| Measure the **output** shape (lines per unit) at scoping, not at closure | `scope-draft.md` |
| Run the prose gates (`denominator-gate`, `figure-provenance`) **as the last act before commit**, not inside the cycle's `verify.sh` — a closure cycle's own `progress.md` and `kanban.md` entries are written after its gate run and are therefore never gated | `workflow-instruction.md §6`; the cycle contract |

---

## The closure sweep (`workflow-instruction.md §11` step 3) — counts found vs removed

Run by `AT-35-E7-002` on the shared checkout at `a57ae67cb5`.

```
git worktree list ; git branch -a ; df -h /
```

| item | found | removed | kept, and why |
|---|---|---|---|
| dispatcher worktrees under `.claude/worktrees/wf_291be5c8-5f3-*` | 16 | **0** | `git worktree remove` is refused by the Claude Code auto-mode permission classifier for a dispatched agent on this checkout — refused as a loop **and** as a single explicit path |
| `worktree-wf_*` local branches | 23 | **8** | the 15 still held checked out by a live worktree cannot be deleted while it exists |
| unfolded artifacts inside those worktrees | 6 | **6 folded into the main checkout** | — |
| `test` branch | 1 | **0** | persistent self-healing release gate — never delete |
| `update-index` branch | 1 | **0** | the updater feed — never delete |
| remote branches | 8 | 0 | none stale; `origin/HEAD`, `develop`, `main`, `test`, `tranche/15`, `update-index`, and two live fix/docs branches |
| disk | 61% used, 569G free of 1.5T | unchanged | the 13G under `.claude/worktrees` could not be reclaimed |

**Nothing was lost by the refusal, and that was established before the sweep rather than assumed.**
All 16 worktree `HEAD`s are ancestors of `tranche/15`. Of the 23 branches, 22 are ancestors of
`tranche/15`; the 23rd (`worktree-wf_291be5c8-5f3-96`) is 47 commits ahead of `tranche/15` and
**every one of those 47 is contained in `origin/develop`** (`git merge-base --is-ancestor`), so it
was safe to delete and was deleted. Of the 30 untracked files found across the worktrees, 24 were
already present in the main checkout **and main's copy was the later one in every case** (the
worktree copies were pre-denominator-gate drafts). The 6 that existed nowhere else were copied in
and are committed with this retrospective:

```
docs/release/.../epic-1-tax-cut/epic-1_wrapup-gate_receipt.md
docs/release/.../epic-3-place-and-surface/EPIC-3_wrapup_gate_report_regate.md
docs/release/.../epic-6-pcgen-exit/EPIC-6_wrapup_gate_report_regate6.md
docs/retro/events/epic-1-wrapup-gate.jsonl
docs/retro/events/at-35-e3-regate.jsonl
docs/retro/events/e6-wrapup-regate-1.jsonl
```

The ~14 modified `.rs` files per worktree were checked and discarded deliberately: they are a
superseded earlier pass at Epic 6's `pcgen_import` relocation (`use crate::pcgen_import::...`),
and HEAD has already moved those modules out of `src/rules_core/pilot_compute/` entirely.

**This is the eleventh recurrence of the same refusal.** Ten open deferrals from the Epic 2–6
wrap-ups each record it; `AT-35-E6-WRAPUP-FIX2`'s receipt already notes that removal was
"attempted — refused by the harness permission layer, not by git". The escalation is therefore not
that the sweep is hard, but that **`workflow-instruction.md §10 step 2` and `§11 step 3` both
assign a step no agent in this program has ever been permitted to run.** It needs either an
operator-run `git worktree remove --force` over the 16 paths, or a `Bash` permission rule, and it
belongs in SD-36's launch checklist rather than in a twelfth deferral. Recorded as
`deferral 1789503195144-at-35-e7-002-023beb`.

---

## The finding that reframed the package

Two numbers, next to each other:

**56.8% of 70,317 shipped sheet rules print the rule's words rather than a computed value.**
**90.7% of 49,450 units print exactly one line.**

Campaign Codex spent three bundles building magnitude engines, consumer-delta probes, and
fixture harnesses to *prove* a record was modelled. SD-35's measurement says the median record is
one line of prose, and the majority form across the whole corpus is prose. The engine work was not
wasted — 27,198 rules do carry a real `Number` and 3,173 a real `Dice`, and those are the lines a
player actually adds up. But the finish line had been set at "the engine holds it", and under that
bar 23,315 units were open at the cut with no path that did not involve modelling every one.

The operator's ruling — *it is a paper character-sheet generator, not a video game* — did not lower
the bar. It corrected the bar to the artifact. Once "DONE" meant "the line is on the sheet",
one converter run closed 21,911 units, `_refused.json` went empty, the deferred and not-started
buckets went to zero, and there was nothing left to carve out.

**The finish line shapes the work.** SD-34 wrote that down. SD-35 is the measurement that proves
it, and the proof is a distribution, not an argument.
