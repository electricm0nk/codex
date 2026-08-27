---
canonical: true
owner: god-emporer
bundle_id: SD-34
date: 2026-08-26
---

# SD-34 Epic Breakdown — 6 epics, 27 criteria

Criterion IDs follow the program convention `AT-34-E<epic>-<nnn>`. Every criterion states its
**evidence obligation** — the command or artifact that proves it.

**Standing rule** (`decisions.md §3`): any percentage states its denominator in the same
construct. Enforced live by `scripts/verify.sh --only denominator-gate`.

**Second standing rule** (`decisions.md §4`): a lane's `status` is a mechanical function of its
own artifact, never a judgment about effort.

**Third standing rule, specific to this bundle** (`decisions.md §2`): any remaining step
discovered that the atlas did not predict is a **defect in the atlas**. It is logged as a
`correction`, the atlas is re-derived, and the discovery is reported. Discovering work is
expected; discovering a *category* the atlas missed is a failure of the deliverable.

---

## Epic 1 — The Completion Atlas

**Gated on:** launch gates. **Gates:** every other epic.
**This epic is the bundle's primary deliverable.** Epics 2–5 exist to prove it and price it.

### AT-34-E1-001 — every unit carries exactly one named remaining-step

`scripts/completion_atlas.py` partitions all **49,438** units into the buckets fixed by
`decisions.md §2`: `DONE`, `A`, `B`, `C`, `D`, `M`, `V`, `U`, `X`, `Z`. Each bucket carries a
count, a **named mechanism that clears it**, and a re-derive command.

**Evidence:** `python3 scripts/completion_atlas.py --check` exits 0 and prints
`population=49438 buckets=10 unclassified=0 overlap=0`, with the bucket counts summing to the
population. A committed `artifacts/epic-1-atlas/completion-atlas.json`.

**A bucket named for an absence of knowledge is a defect**, not a bucket. `D: other engine
gap` (1,230 units) is admitted only with its sub-causes enumerated — a holding pen with a
census, never a shrug. **The same rule binds `U: instrument cannot express a verdict`** (321
units: 270 `text_only_but_corpus_record_carries_no_description_to_show_a_player`, 51
`feat_served_description_is_a_placeholder_marker_not_prose`; by kind 140 `equipment_modifier`,
119 `equipment`, 62 `feat`). SD-33's register C1.1 required that kinds with **no probe at all** be
named, counted and owned, never absorbed into `unverifiable` — `U` is where that population
lives in SD-34, and the atlas enumerates its sub-causes and the probe (or absence of one) behind
each.

### AT-34-E1-002 — the atlas fails closed on six conditions

Exits non-zero on:

1. `unclassified != 0`
2. `overlap != 0`
3. a unit in `DONE` whose evidence does not support it
4. a bucket with no named clearing mechanism
5. a `derived_at` SHA that is not an ancestor of `HEAD` (**staleness gate**)
6. **a bucket whose definition does not cite the `file:line` that emits the evidence strings
   it keys on — or whose citation no longer resolves at `HEAD`**

**Evidence:** six RED→GREEN mutation proofs, one per condition, in the receipt. **A tool never
observed to fail is not a gate.**

**Condition 6 exists because of a real, expensive error** (`decisions.md §12` L1). The atlas's
A/B/C/D arms key on substrings of the `evidence` field. Reading a *field name* instead of the
code that writes it produced a wrong headline reported to the operator during this package's
authoring — the `not-ingested` status means "the engine does not hold this record", and
26,002 of 26,002 of its units (100%) carry a real `source_file` and `source_line`. Condition 6 makes the atlas unable to
define a bucket without pointing at the code that produces it, and unable to stay green when
that code moves.

### AT-34-E1-003 — the missing engine tables are enumerated and their book coverage mapped

Bucket A is **8,463** units across **9** kinds with no engine table. Each is named with its
population, **which books contain it**, and which books it would unblock.

**Evidence:** `artifacts/epic-1-atlas/missing-engine-tables.json` — per kind: unit count, the
per-book breakdown, the engine surface a table would attach to, and the books that reach zero
bucket-A once it exists. **This is the direct answer to "do we need to build something after
the shape engine runs".**

**The book-coverage half is load-bearing**, not decoration: it is what identified
`ultimate_campaign` as an almost-single-bucket book and corrected an earlier miscount of how
many tables the Core Rulebook exercises (`decisions.md §7`).

### AT-34-E1-004 — the shape-engine boundary is stated as a fact, not an assumption

A committed statement, proven by execution, of what a shape engine does and where its output
stops — so no future bundle re-learns it.

**Evidence:** `artifacts/epic-1-atlas/shape-engine-boundary.md`, carrying the count of
magnitude-bearing units (**26,396**), how many of those the engine still does not hold
(**13,119 of 26,396**), and the four-condition promotion ladder quoted from
`src/bin/v06_work_inventory.rs` with its line number re-verified at HEAD.

### AT-34-E1-005 — the `not-ingested` status field is renamed

The field asserts the opposite of its meaning: 26,002 of 26,002 of its units (100%) carry a real
`source_file` and `source_line`, and every evidence string is engine-side. It has already
misled once, during this package's own authoring.

**Evidence:** the field renamed to state what it means (e.g. `engine-does-not-hold`) across
`src/bin/v06_work_inventory.rs`, `docs/work-inventory.json`, and every consumer, with
RED→GREEN. A count sweep across `tests/`, `src/`, `apps/`, `scripts/` for the old string,
reported. The atlas's A/B/C/D arms key on this string and are updated in the same cycle.

### AT-34-E1-006 — every figure in this package carries its re-derive command, enforced

A `scripts/verify.sh` stage that **fails** on a figure stated in an SD-34 package document or
cycle receipt without a re-derive command reachable from it.

**Evidence:** RED→GREEN mutation proof — a deliberately-unsourced figure fails the stage; the
sourced form passes. Wired into `verify.sh`'s stage list, alongside `denominator-gate`, not as
a standalone script.

**This closes two open defects at once:**

- **`workflow-instruction.md §12` row 15, marked UNENFORCED at launch** — *a vacuous pass is not
  a pass; state every gate's population.* The same stage requires a gate's PASS line to name the
  population it examined.
- **`decisions.md §12` L2** — a figure carried forward from an author's own earlier document
  rather than re-derived. Both of this package's counting errors (the ingestion figure, and
  "six of nine tables" when it was seven) were inherited numbers, not fresh ones.

**The rule the stage encodes:** a number is either accompanied by the command that produces it,
or it is not a figure — it is a recollection.

**Second obligation of the same cycle:** widen `scripts/denominator_gate.py`'s default scope
(`BUNDLE_DIR` / `DEFAULT_GLOBS`) from SD-33's folder to this package, so that
`scripts/verify.sh --only denominator-gate` examines SD-34 without an explicit path
(`decisions.md §3`). RED→GREEN: the default run's `files_checked` must include every SD-34 `.md`.

### AT-34-E1-007 — `v06_corpus_trap_report --audit` is a real `verify.sh` stage

Inherited unclosed through SD-31, SD-32 and SD-33 (each register carried it as C1.8). It is not
carried a fourth time: Epic 1 already opens the `verify.sh` stage list (AT-34-E1-006), and the
wiring pattern is established.

**Evidence:** the stage listed in `verify.sh`'s `ALL_STAGES`; `scripts/verify.sh --only
corpus-trap-audit` exits 0 and prints the population it examined; RED→GREEN by planting one trap
the audit must catch, confirming the catch, removing the probe, and confirming the baseline
returns to zero. The stage's own timeout wrapper is part of the deliverable — SD-33's register
D1.2 records a sibling stage that could not bound its own runtime.

---

## Epic 2 — Build eight of the nine tables

**Gated on:** Epic 1. **Population:** **8,042 of 8,463** bucket-A units, across 8 kinds.

| Kind | Units | Why built here |
|---|---:|---|
| ability | 4,337 | Core Rulebook (471) **and** Ultimate Campaign (88) |
| template | 2,248 | Core Rulebook (262) |
| trait | 487 | Ultimate Campaign (154) — and 4 other books |
| deity | 459 | Core Rulebook (21) |
| domain | 183 | Core Rulebook (34) |
| skill | 149 | Core Rulebook (110) |
| language | 136 | Core Rulebook (22) |
| companion | 43 | Core Rulebook (14) |

**`power` (421 units) is the one table not built here.** Every one of its units is inside
`ultimate_psionics`, a 3,498-unit book with all eight non-DONE buckets occupied — building the
table would not come close to closing that book, so there would be no banked book to prove the
work. It is **costed** in Epic 5 from the measured build rate of the eight
(`decisions.md §7`).

### AT-34-E2-001 — each of the eight tables is built, or proven unnecessary

For each kind, either an engine table exists and holds records, or a proof by execution that
the kind needs none — e.g. every unit of that kind is `display`-class and its terminal state is
a rendered description.

**Evidence:** per kind, either the table's location and a transcript of it holding a named
record, or the counts showing no magnitude is involved. **"No table needed" is a finding that
must be proven, never assumed to save work.**

### AT-34-E2-002 — each new table is fail-closed

A table returns a real record or a named refusal. It never returns a fabricated or defaulted
entry.

**Evidence:** per table, a RED→GREEN pair — observed refusing an absent key, and returning a
real record for a present one.

### AT-34-E2-003 — the measured build rate is recorded

The cost of building an engine table has never been measured in this program, and Epic 5's
pricing of `power` depends on it.

**Evidence:** `artifacts/epic-2-tables/table-build-rate.json` — per table: wall time, lines
changed, what dominated, and whether the kind's shape made it cheaper or dearer than the
others. **A single blended average across eight tables is not the deliverable** — the spread
is what makes a projection for `power` honest.

### AT-34-E2-004 — bucket A reaches zero for both vehicle books

**Evidence:** `python3 scripts/completion_atlas.py --book core_rulebook --check` and
`--book ultimate_campaign --check` each report bucket A at zero, with movement stated in four
buckets (closure / reclassification / reachability / instrument-correction).

---

## Epic 3 — Core Rulebook to zero

**Gated on:** Epic 2. **Population:** all **6,701** Core Rulebook units.
The first and deeper of the two proofs that the atlas is real.

**Every bucket except `Z` is present in this book**, which is what makes it measure the real
cost of every step type in the atlas.

### AT-34-E3-001 — bucket B closes: records reach their tables

**970** Core Rulebook units whose table exists but which are not in it.

**Evidence:** the atlas reporting bucket B at zero for `core_rulebook`, and the mechanism that
placed them named — **by mechanism, not per record.**

### AT-34-E3-002 — bucket C closes: held records reach the player

**370** units the engine holds and computes but never surfaces.

**Evidence:** per unit, the explanation or display path that now carries it. A unit the player
still cannot see is not cleared, whatever the engine holds.

### AT-34-E3-003 — buckets M, V, D, U, X close

**512** M, **2,582** V, **119** D, **58** U, **6** X.

**V is the largest and is inherited from SD-33's own population** — rowed by proxy, never
oracle-checked. Clearing it means running those units through the harness that already exists.
It is the bucket most likely to dominate this epic's wall time.

**Evidence:** per bucket, the atlas reporting zero for `core_rulebook`, with movement in four
buckets. **A count that drops because measurement changed is instrument-correction, not
closure.**

### AT-34-E3-004 — the cost of every step type is measured, not estimated

For each bucket cleared: units cleared, wall time, and what dominated.

**Evidence:** `artifacts/epic-3-core-rulebook/step-cost-ledger.json`. **This is Epic 5's input
and the reason this epic exists at all.** A bucket cleared without its cost recorded has
delivered half its value.

### AT-34-E3-005 — the Core Rulebook reaches zero remaining steps

**Evidence:** `python3 scripts/completion_atlas.py --book core_rulebook --check` exits 0 with
`DONE=6701 of 6701`, every other bucket zero. Plus
`artifacts/epic-3-core-rulebook/core-rulebook-completion-manifest.json` — one row per unit, its
final state, and the evidence pointer establishing it. **The closure scan re-derives a random
sample independently.**

### AT-34-E3-006 — anything the atlas failed to predict is recorded as an atlas defect

**Evidence:** `artifacts/epic-3-core-rulebook/atlas-defects.md` — per discovery: what it was,
why the atlas missed it, the `correction` retro event, and the atlas re-derivation that
followed. **An empty file is a valid and excellent result; an absent file is a failure.**

This criterion is the operator's "three more things" guard made mechanical.

---

## Epic 4 — Ultimate Campaign to zero

**Gated on:** Epic 2. **Population:** all **265** Ultimate Campaign units.
The second proof — and deliberately a **differently shaped** book.

Its entire profile is four rows:

| Kind | Bucket | Units |
|---|---|---:|
| trait | A — no engine table | 154 |
| ability | A — no engine table | 88 |
| feat | U — unmeasurable | 21 |
| feat | X — deferred with reason | 2 |

**No B, C, D, M or V at all.** 242 of 265 units clear on the two tables Epic 2 already builds.
It is the cleanest book in the corpus and the best table-to-book ratio available anywhere.

### AT-34-E4-001 — the 23-unit non-A tail is resolved

**21** `feat` units at `U: instrument cannot express a verdict` and **2** at `X: deferred with
reason`.

**Evidence:** for each `U` unit, the instrument correction that lets a verdict be expressed —
or a proven statement that no verdict is possible, with the reason. For each `X`, its stated
deferral condition checked and resolved. **`U` is an instrument failure, not a unit property;
clearing it is an instrument-correction and is reported in that bucket.**

### AT-34-E4-002 — Ultimate Campaign reaches zero remaining steps

**Evidence:** `python3 scripts/completion_atlas.py --book ultimate_campaign --check` exits 0
with `DONE=265 of 265`, every other bucket zero, plus
`artifacts/epic-4-ultimate-campaign/ultimate-campaign-completion-manifest.json`.

### AT-34-E4-003 — a second, independent cost measurement is recorded

The Core Rulebook is deep and many-bucketed; Ultimate Campaign is shallow and single-bucketed.
Two books of opposite shape give Epic 5 a **range** rather than one blended number.

**Evidence:** `artifacts/epic-4-ultimate-campaign/step-cost-ledger.json`, and a stated
comparison against Epic 3's rates: where they agree, where they diverge, and **which
divergences are explained by book shape rather than by noise**. A projection built on a single
book's rate says so.

---

## Epic 5 — Price the remaining 35 books

**Gated on:** Epics 3 and 4. **Population:** the non-DONE units outside the two vehicle books,
across **35** books.

### AT-34-E5-001 — a per-book, per-bucket forward plan exists for every remaining book

**Evidence:** `artifacts/epic-5-forward-plan/forward-plan.json` — per book, per bucket: unit
count, the mechanism that clears it, and the projected cost using the **measured** rates from
Epics 2, 3 and 4. Every projection names the rate it used and the sample size behind it.

**A projection built on a thin sample says so in its own row.** A confident number from a thin
sample is a failure this program has hit repeatedly.

### AT-34-E5-002 — every capability that must still be built is named

Beyond the `power` table: anything Epics 3 or 4 proved is required and does not exist.

**Evidence:** `artifacts/epic-5-forward-plan/capability-register.json` — per capability: what
it is, which buckets and books it unblocks, its population, and whether SD-34 built it. **This
is the operator's second explicit question answered in machine-readable form.**

### AT-34-E5-003 — the `power` table is costed

421 units, all inside `ultimate_psionics` — not built here, costed here, using the measured
build rate from Epic 2's eight tables and the spread across them.

**Evidence:** the projected cost, the rate it derives from, and the reason it was not built
(`decisions.md §7`). Plus what `ultimate_psionics` would still need after it exists — that book
has all eight non-DONE buckets occupied, so the table alone does not close it, and the plan
must say so.

### AT-34-E5-004 — the plan is ordered by real cost, cheapest-first, and single-bucket books are flagged

**Evidence:** the forward plan sorted by projected cost per book, with the ordering's basis
stated. **Books whose remaining work is a single bucket are identified by name** — those are
the genuine low-hanging fruit, and finding `ultimate_campaign` this way is what earned it
Epic 4.

---

## Epic 6 — Closure epilogue

**Gated on:** Epics 1–5 all `complete`. Fires **once**.

### AT-34-E6-001 — final-acceptance scan

Every criterion `AT-34-E1-001` … `AT-34-E5-004` is `complete`, and every `kanban.md` card is
`complete`. **A card at `in-progress`, `blocked-escalated`, or `complete`-with-a-deferred-half
blocks closure.** There is no "complete *or* filed under `## Open blockers`".

**The scan additionally verifies the deliverable's own integrity**, because that is what this
bundle is for:

- `completion_atlas.py --check` exits 0 with `unclassified=0`, **re-run at HEAD**
- every bucket's `file:line` citation still resolves at `HEAD` (condition 6)
- both completion manifests' evidence pointers resolve, on independently drawn samples
- `atlas-defects.md` exists, and each defect it names produced an atlas re-derivation
- every forward-plan projection names its measured rate and sample size
- all nine tables are accounted for — eight built, `power` costed with its rate
- `scripts/verify.sh --only figure-provenance` exits 0 across the package (AT-34-E1-006)
- **no closure claim rests on a dispatch script's own return value.** Every `complete` in
  `kanban.md` is re-derived from the repo by this scan (`decisions.md §12` L3)
- **no `incident` recurrence key fired 3+ times without producing a mechanical control**, or
  an escalation naming why one is not possible (`decisions.md §12` L5):
  `python3 scripts/retro.py summary --since <launch> --json` — read the recurrence keys
- **every corpus change moved the sweep's examined-population by exactly the records added**
  (`decisions.md §12` L8) — read each cycle receipt's sweep-population row
- **every build-scope row names the SHA it ran at, and that SHA is the cycle's last
  figure-moving commit** (`decisions.md §12` L7)
- **the denominator gate examined this package** — `files_checked` in the default run includes
  every SD-34 `.md` (AT-34-E1-006's second obligation), `violations=0`

**If anything is short, the cycle stops here** — no retrospective, no sweep, **no PR**.

### AT-34-E6-002 — retrospective written and cited

`docs/retro/sd34-book-completion-retrospective.md`, grounded in
`python3 scripts/retro.py summary --since <bundle-launch-date> --json`. **Cited from
`references/README.md` in the same cycle.** Every lesson names its enforcing command or is
marked `UNENFORCED`, and an `UNENFORCED` marking is a tracked defect.

### AT-34-E6-003 — sweep, architecture docs, graphify, PR, release notes

Full worktree/branch sweep with counts found vs removed; architecture-docs refresh and graphify
per `../template/template.md §6`; PR; release notes and version bump.

**The sweep reads `forward-scope-register.md §E1` first.** SD-33 ruled three branches OUT of its
fold by name and wrote *"SD-34 must not re-litigate them"*. They are deleted, not re-diagnosed.
Any **other** branch the sweep finds carrying unmerged records is diagnosed schema-against-HEAD
(`decisions.md §12` L6) before it is folded or removed, and the diagnosis goes in the receipt.

**Order is load-bearing:** retrospective and sweep happen **before** the PR opens.
