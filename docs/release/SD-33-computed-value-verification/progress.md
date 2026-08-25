---
canonical: true
owner: god-emporer
bundle_id: SD-33
status: in progress — Epics 1-4 complete (rows 1-15); Epic 5 started, AT-33-E5-001 in-progress (11 of 1,741 fixture-verified units re-examined) and AT-33-E5-002 in-progress (21 of 6,589 literal-verified units re-examined), rows 16-17 not yet complete; AT-33-E5-003 (row 18) complete — 0 disagreements among the 32 units examined so far
date: 2026-08-24
---

# SD-33 Progress

Live cycle-by-cycle record. Cycles **prepend** their entry (newest first) and update `kanban.md` in the same commit, via `workflow-instruction.md §5`'s retry protocol.

## Status

**Launch gates passed 2026-08-25** (`technical-requirements.md §1`, `workflow-instruction.md §1`):

1. SD-32's closure PR merged to `develop` — PR #376 MERGED, `origin/develop` = `f53b8e32da`
2. SD-32's instrument debt closed **inside SD-32** — 29 total / 0 open deferrals, `EXCLUDED_BOOKS = frozenset()`
3. `tranche/13` cut from `develop` and pushed — `origin/tranche/13` = `f652db7ac7`

Epic 1 complete; cycles 1-4 (`AT-33-E1-001` row 1, `AT-33-E1-002` row 2, `AT-33-E1-003` row 3,
`AT-33-E1-004` row 4) all landed. Epic 1 gates every other epic
(`workflow-instruction.md §3`) — Epics 2/3/4 (`parallel: yes`, worktree-isolated) are next.

**Epic 2 complete; rows 5-8 (`AT-33-E2-001..004`) all landed.
RULING: Path A** — the pinned PCGen builds headless on this box, and a
hand-authored `.pcg` round-trips through `BatchExporter` via a
hand-authored template producing real, independently-cross-checked
computed values (13 of 13 hand-derived RAW fields match the real oracle
export exactly). `scripts/oracle_harness/` answers
`(ours, oracle, agree|disagree|unverifiable)` per unit, proven both by
16/16 unit tests (all three verdicts, including a known-disagreeing case)
and by a live end-to-end run whose `disagree` record feeds the real
`scripts/box_ledger.py --check` fail-closed gate to exit 1. **No Path B
fallback was needed; no throughput-reduction escalation is raised** — Epic
5 (gated on Epic 2) can run the live-PCGen path at full mechanism
availability. See `artifacts/epic-2-oracle-harness/AT-33-E2-004_cycle_receipt.md`
and `artifacts/epic-2-oracle-harness/oracle-comparison-fixtures.md` for the
full ruling.

**Bundle-level figure (`AT-33-E1-003`'s own evidence bar, not a footnote):** of the corpus's **19**
distinct `kind` values (`jq -r '.units[].kind' docs/work-inventory.json | sort -u | wc -l`), **8**
carry a probe capable of verifying a computed magnitude and **11** do not
(`python3 scripts/probe_surface_census.py --check` → `kinds_with_probe=8 kinds_without_probe=11`).
Of the 11: 8 have no engine table at all (`ability`, `template`, `deity`, `power`, `domain`,
`skill`, `language`, `trait`), and 3 have an engine table but only a presence/lookup check, never a
computed-delta observation (`monster`, `monster_ability`, `companion`) — see
`artifacts/epic-1-instruments/probe-surface-census.json` and its cycle receipt for the full
per-kind table and the source citations.

**Cards complete: 16 / 21** (`jq` re-derive: count `complete` rows in `kanban.md`'s table body) —
Epics 1-4 (rows 1-15) plus row 18 (`AT-33-E5-003`). Rows 16-17 (`AT-33-E5-001`/`AT-33-E5-002`,
gated on Epic 2) and Epic 6 (rows 19-21) remain.

**Epic 4 complete; rows 13-15 (`AT-33-E4-001..003`) all landed.** The 4,224 units at
`status: "unknown"` reach zero (`jq '[.units[]|select(.status=="unknown")]|length' docs/work-inventory.json`
→ `0`), root-caused before any count moved (`unknown-rootcause.md`) and reported in the four required
buckets: **closure 0 / reclassification 3,906 (854 → `ingested-magnitude`, 3,052 → `not-ingested`,
both because the instrument already had the evidence and lacked the code path to say so) /
reachability 0 / instrument-correction 318** (the genuinely-irreducible remainder, renamed
`unknown` → `unmeasurable`, disposition unchanged). `python3 scripts/box_ledger.py --check` →
`uncovered=0 overlap=0 population=49438 unverifiable_done=0 stale=False`, no warnings. See
`artifacts/epic-4-unknown-classification/AT-33-E4-001..003_cycle_receipt.md` (three files, one per
criterion) for the full per-unit verification.

**Discovery, disclosed (not this criterion's own movement): the committed `docs/work-inventory.json`
was stale by 3,985 units unrelated to `unknown`.** It had not been regenerated since 2026-08-23;
`AT-33-E4-002`'s regen (the only way to prove the classifier fix) necessarily also captured real
SD-32 engine work that had landed on `develop`/`tranche/13` in the interim but was never reflected on
the board — `grounded` +106, `text-complete` +3,739, `not-ingested` net −896 aside from `AT-33-E4`'s
own +3,052, `ingested-magnitude` +100 aside from `AT-33-E4`'s own +854. Verified by an `id`-keyed
join, not an aggregate count; full breakdown in `AT-33-E4-002`'s receipt.

**Cross-file follow-up, disclosed, not a blocker:** `scripts/observer/pf1e_dashboard_producer.py`'s
`_doneness_verdict_uncapped()` raises on any `(wiring_class, status)` pair it has no rule for, and its
table still names `status == "unknown"` rather than the renamed `"unmeasurable"`. Outside
`AT-33-E4`'s write scope (`src/bin/v06_work_inventory.rs`, `docs/work-inventory.json`,
`artifacts/epic-4-unknown-classification/`, `THE-BOX.md` append-only); a one-line fix for whichever
cycle next touches that file.

**Denominator gate is now live** (`AT-33-E1-004`): `scripts/verify.sh --only denominator-gate`
runs `scripts/denominator_gate.py --check` against this bundle's own `artifacts/**/*_cycle_receipt.md`
+ `progress.md` (4 files as of this commit, 0 violations) and fails closed on a bare percentage —
proven both ways through the real stage invocation, not just the underlying script:
`DENOMINATOR_GATE_PATHS=<malformed file> bash scripts/verify.sh --only denominator-gate` → exit 1;
corrected form → exit 0. See the cycle receipt for the full transcript.

**Epic 5 in progress; `AT-33-E5-001` (row 16) is `in-progress`, not `complete`.** The
`fixture-verified` population is 1,741 (spell 1,288 / companion 187 / monster 140 /
monster_ability 100 / class_feature 15 / equipment 11 —
`jq -r '[.units[]|select(.status=="fixture-verified")]|group_by(.kind)|map({kind:.[0].kind,count:length})' docs/work-inventory.json`).
This cycle extended `AT-33-E2-004`'s proven Path A mechanism from Epic 2's one hand-authored
fighter to a real batch: the entire `equipment` kind (**11 of 1,741**) was re-verified against a
live PCGen `BatchExporter` round-trip (11 separate `.pcg` characters, 11/11 `./gradlew run`
invocations exit 0) with `ours` sourced from a live call into the real
`codex::rules_core::equipment_effects::compute_equipment_effects` engine function — **11 of 11
agree, 0 disagree**, verified through `scripts/oracle_harness/run.py` and independently through
`scripts/box_ledger.py --check --oracle-results ...` (exit 0). The remaining **1,730 of 1,741**
are not yet examined and are **not** folded into a false 100%: 1,303 (`spell`+`class_feature`)
carry a real magnitude probe and are queued with a concrete next-cycle plan (real per-unit/batch
`.pcg`+template authoring cost — the sizing question `AT-33-E2-004`'s receipt named as Epic 5's
own scope); 427 (`companion`+`monster`+`monster_ability`) carry **no** magnitude probe at all
(`AT-33-E1-003`'s pre-existing finding, `probe_exists: false`) and cannot be oracle-compared by
this cycle's mechanism until Epic 1's probe surface widens. No `## Open blockers` entry is filed
— this is decomposition and first-slice execution of a population `AT-33-E2-004` already named as
too large for one cycle, not an escalation. Full detail:
`artifacts/epic-5-reverification/README.md` and `AT-33-E5-001_cycle_receipt.md`.

**`AT-33-E5-002` (row 17) is also `in-progress`, not `complete`.** The `literal-verified`
population is **6,589** — a separate, non-overlapping population from `AT-33-E5-001`'s 1,741
`fixture-verified` units (equipment 5,170 / monster 843 / monster_ability 148 / spell 217 /
companion 99 / equipment_modifier 46 / race 36 / class_feature 17 / race_trait 13 —
`jq -r '[.units[]|select(.status=="literal-verified")]|group_by(.kind)|map({kind:.[0].kind,count:length})' docs/work-inventory.json`).
This cycle re-used `AT-33-E5-001`'s already-proven mechanism (the built PCGen jar, the
`e5-equip-stats.txt.ftl` template, the `Belt`/`Headband` `.pcg` slot convention) against a new,
21-item slice of the `literal-verified` `equipment` kind carrying the same single-ability
`STAT|<ability>|<n>|TYPE=Enhancement` shape — **21 of 21 agree, 0 disagree**, verified through
`scripts/oracle_harness/run.py` and independently through
`scripts/box_ledger.py --check --oracle-results ...` (exit 0). The remaining **6,568 of 6,589**
are not yet examined and are **not** folded into a false 100%: 5,478 (`equipment` remainder +
`spell` + `equipment_modifier` + `race` + `class_feature` + `race_trait`) carry a real magnitude
probe and are queued with a concrete next-cycle plan; 1,090 (`companion`+`monster`+`monster_ability`)
carry **no** magnitude probe at all (`AT-33-E1-003`'s pre-existing finding, same structural gap
`AT-33-E5-001` already named). No `## Open blockers` entry is filed. Full detail:
`artifacts/epic-5-reverification/README.md` ("AT-33-E5-002" section) and
`AT-33-E5-002_cycle_receipt.md`.

**`AT-33-E5-003` (row 18) is `complete`.** Independently re-derived (not transcribed) from the two
committed oracle-results files rows 16/17 produced: **0 disagreements among the 32 units examined
to date** (`equipment.oracle-results.json` 11 records + `equipment-literal.oracle-results.json` 21
records, merged and re-checked through `scripts/box_ledger.py --check --oracle-results` — condition
3, `AT-33-E1-002` — `oracle_disagreement=0`, exit 0). Zero `progress.md` disagreement-ledger entries
are required because zero disagreements exist. **Not a claim that the full 8,330-unit population
has no disagreement** — 32 of 8,330 (0.38%) is examined; rows 16/17 remain `in-progress` and own
examining the rest. The reopening condition is mechanical, proven by mutation this cycle: a single
injected `"verdict": "disagree"` record makes `box_ledger.py --check` exit 1 and name the unit
(`oracle_disagreement=1`) — so a future rows-16/17 cycle that lands a real disagreement will be
caught by an existing gate, not by memory. Full detail: `artifacts/epic-5-reverification/README.md`
("AT-33-E5-003" section) and `AT-33-E5-003_cycle_receipt.md`.

## Disagreement ledger

Per `AT-33-E5-003`'s evidence line: one entry per disagreement, each resolved to a commit or an
operator escalation. **Current entries: none — 0 disagreements found in the 32 units
`AT-33-E5-001`/`AT-33-E5-002` have examined to date** (re-derive:
`python3 -c "import json,collections;a=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment.oracle-results.json'));b=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-literal.oracle-results.json'));print(collections.Counter(r['verdict'] for r in a['results']+b['results']))"`
→ `Counter({'agree': 32})`). This is a live section: the next `AT-33-E5-001`/`AT-33-E5-002` cycle
whose oracle-results file contains a `disagree` record must add an entry here, root-caused per the
criterion's Evidence line, before that entry's disagreement can be considered resolved.

| unit_id | ours | oracle | root cause | resolution | commit |
|---|---:|---:|---|---|---|
| _(none yet — 0 of 32 examined units disagree)_ | | | | | |

## Cycle entry schema

Each entry states, at minimum:

- criterion ID and card number
- commit SHA(s)
- **every figure with the command that produces it and its denominator** (`decisions.md §2`)
- **movement in four buckets** — closure / reclassification / reachability / instrument-correction
- receipt path

## Open blockers

None. **This section is not a parking lot.** An entry here is a request for an operator ruling and it **pauses the bundle** (`../../governance/blocker-closure-doctrine.md`). It is never a disposition, never a closure path, and no later cycle may proceed past a blocked card on its own authority.

## Cycles

### Cycle AT-33-E5-003 — disagreement-resolution (row 18, Epic 5) — complete

- **Criterion:** `AT-33-E5-003` — every disagreement is a named defect, fixed or escalated.
- **Files:** `artifacts/epic-5-reverification/README.md` (extended, new "AT-33-E5-003" section),
  `AT-33-E5-003_cycle_receipt.md` (new),
  `AT-33-E5-003.combined-oracle-results.json` (new — `AT-33-E5-001`'s 11 + `AT-33-E5-002`'s 21
  records merged), `progress.md` (this entry + `## Disagreement ledger` section), `kanban.md`
  (row 18).
- **What landed:** independently re-derived the current disagreement population directly from the
  two committed oracle-results JSON files (not transcribed from either prior receipt's prose) —
  **0 disagreements among the 32 units examined to date**, re-checked through
  `scripts/box_ledger.py --check --oracle-results` (`AT-33-E1-002`'s condition-3 gate) on the
  merged file, independently of the harness that produced the two source files.
- **Figures:** 32 of 8,330 (0.38%) of the `fixture-verified`+`literal-verified` population examined
  by `AT-33-E5-001`/`AT-33-E5-002` to date; 0 of 32 disagree; `box_ledger.py --check` on the merge
  → `oracle_disagreement=0`, exit 0. Full table with commands: `AT-33-E5-003_cycle_receipt.md`.
- **Movement (four buckets):** closure 0 / reclassification 0 / reachability 0 /
  instrument-correction 0 — no inventory unit's status changes; this criterion resolves
  disagreements and none exist to resolve.
- **RED→GREEN:** mutation proof against the detection mechanism (not new production code, since
  0 disagreements exist to fix): a scratch copy of the committed merge with one record's `verdict`
  set to `disagree` → `box_ledger.py --check` reports `oracle_disagreement=1`, names
  `ultimate_equipment:equipment:belt_of_mighty_hurling_greater`, exit 1 (RED). The real, unmutated,
  committed merge → `oracle_disagreement=0`, exit 0 (GREEN). Mutated file lived only in `/tmp`, never
  committed.
- **Status: complete** — the evidence line's obligation (one `progress.md` entry per disagreement,
  each resolved) is satisfied because the set of disagreements to resolve is empty, verified
  independently rather than assumed. **This is not a claim about the 8,298 not-yet-examined units**
  of the 8,330-unit population — that is `AT-33-E5-001`/`AT-33-E5-002`'s own scope (rows 16/17,
  correctly `in-progress`). This criterion's scope is reactive to what those two produce, and what
  they have produced so far is zero disagreements. The reopening condition is mechanical
  (`box_ledger.py` condition 3, proven live by the mutation proof above), not a promise to
  remember — the next disagreement either row surfaces will fail that gate by name.
- **Notes:** considered and rejected fabricating a synthetic production disagreement (e.g.
  temporarily reverting a real fix) to exercise the fix/escalate machinery end-to-end — rejected as
  the same shape of dishonesty the criterion explicitly forbids in reverse ("never closed by
  adjusting the expectation to match our output"); the mutation proof above tests the **detection**
  mechanism only, the same legitimate technique `AT-33-E1-002`'s own five mutation proofs used, and
  makes no claim about production correctness.
- **Test scoping:** ran `box_ledger.py --check` (three invocations: real merge before mutation, the
  mutated scratch copy, real merge after) and independent Python verdict tallies against both
  source oracle-results files. Did not re-run `test_box_ledger.py`/`test_oracle_harness.py` (neither
  file changed this cycle — confirmed via `git status --porcelain` before this cycle's first
  write). Did not run `cargo test`/`cargo build` (no `src/` file touched) or
  `apps/desktop/src-tauri` (separate cargo workspace, untouched).
- **Receipt:** `artifacts/epic-5-reverification/AT-33-E5-003_cycle_receipt.md`.

### Cycle AT-33-E5-002 — reverify-literal-verified (row 17, Epic 5) — in-progress

- **Criterion:** `AT-33-E5-002` — the 6,589 `literal-verified` units are re-examined against the oracle.
- **Files:** `artifacts/epic-5-reverification/README.md` (extended, new "AT-33-E5-002" section),
  `AT-33-E5-002_cycle_receipt.md` (new),
  `equipment-literal.oracle-export.txt` / `equipment-literal.ours.json` / `equipment-literal.oracle-results.json` (new),
  `fixtures/equipment-literal-pcg/*.pcg` (new, 21), `fixtures/equipment-literal-oracle-txt/*.txt` (new, 21),
  2 build-transcript files (new),
  `ours-derivation/equipment-literal-ours-probe.{rs,Cargo.toml,output.json}` (new — reference copy
  of a scratch program that reads the `codex` crate as a path dependency from outside this repo;
  it writes nothing into the repo).
- **What landed:** re-used `AT-33-E5-001`'s already-proven mechanism (built PCGen jar, template,
  `.pcg` slot convention) against 21 of the 6,589 `literal-verified` `equipment` units carrying the
  same single-ability `STAT|<ability>|<n>|TYPE=Enhancement`/`Belt`-or-`Headband` shape — 21 live
  `.pcg` characters, each real-`EQUIPSET`-equipped, exported through `./gradlew run` for real
  (21/21 exit 0) against a real `ours` value from a live call into
  `codex::rules_core::equipment_effects::compute_equipment_effects`. **21 of 21 agree, 0 disagree**,
  verified through `scripts/oracle_harness/run.py` and independently through
  `scripts/box_ledger.py --check --oracle-results ...` (exit 0).
- **Figures:** see `AT-33-E5-002_cycle_receipt.md`'s figures table — every value with its
  denominator and re-derive command. Headline: 21 of 6,589 (0.32%) examined this cycle; 5,478 of
  6,589 not-yet-examined but probe-bearing; 1,090 of 6,589 not-yet-examined and probe-less
  (pre-existing gap, not created this cycle).
- **Movement (four buckets):** closure 0 / reclassification 0 / reachability 0 /
  instrument-correction 0 — no unit's `status` field changed; this cycle re-uses existing
  instruments without finding a defect in any of them.
- **RED→GREEN:** before this cycle, `AT-33-E5-002`'s evidence obligation had zero real rows —
  `equipment-literal.oracle-results.json` did not exist and no `literal-verified`-population
  `.pcg`/template pair had been authored. After: 21 real per-unit rows, each backed by an
  independently-derived live PCGen export (21/21 `./gradlew run` exit 0) and a live engine call
  (`cargo run --release` exit 0), compared for real (`agree=21 disagree=0 unverifiable=0`),
  independently re-verified by `scripts/box_ledger.py --check` exiting 0 on the same file.
- **Status: in-progress, not complete** — same disposition and same reasoning `AT-33-E5-001`
  (the sibling criterion) used: the criterion's evidence bar asks for the full population's
  per-unit rows, and marking 21 of 6,589 `complete` would be the false-100% shape `decisions.md`
  §2 and `AGENTS.md` rule 2 forbid. No `## Open blockers` entry filed; not an escalation.
- **Notes:** the pinned PCGen checkout, already built and jarred by `AT-33-E5-001` earlier in this
  same dispatch session, was reused unmodified (no rebuild) via the shared per-session scratchpad
  directory. 8 same-shape `equipment` candidates (5 multi-ability, 2 different-slot, 1
  different-book) were identified but excluded from this slice, named explicitly in the receipt
  and `README.md` rather than silently dropped.
- **Test scoping:** ran `scripts/oracle_harness/run.py` and `scripts/box_ledger.py --check`
  (both unmodified this cycle) against this cycle's real output. Did not re-run
  `scripts/tests/test_oracle_harness.py` or `scripts/tests/test_box_ledger.py` (neither file
  changed). Did not run the Rust workspace's `cargo test`/`cargo build` (no `src/` file changed —
  the scratch `equip_probe` crate lives outside this repo). Did not run `apps/desktop/src-tauri`
  (separate cargo workspace, untouched).
- **Receipt:** `artifacts/epic-5-reverification/AT-33-E5-002_cycle_receipt.md`.

### Cycle AT-33-E5-001 — reverify-fixture-verified (row 16, Epic 5) — in-progress

- **Criterion:** `AT-33-E5-001` — the 1,741 `fixture-verified` units are re-examined against the oracle.
- **Files:** `artifacts/epic-5-reverification/README.md` (new), `AT-33-E5-001_cycle_receipt.md` (new),
  `equipment.oracle-export.txt` / `equipment.ours.json` / `equipment.oracle-results.json` (new),
  `fixtures/e5-equip-stats.txt.ftl` (new), `fixtures/equipment-pcg/*.pcg` (new, 11),
  `fixtures/equipment-oracle-txt/*.txt` (new, 11), 2 build-transcript files (new),
  `ours-derivation/equipment-ours-probe.{rs,Cargo.toml,output.json}` (new — reference copy of a
  scratch program that reads the `codex` crate as a path dependency from outside this repo; it
  writes nothing into the repo).
- **What landed:** extended `AT-33-E2-004`'s proven Path A mechanism (one hand-authored fighter)
  to a real re-verification batch covering the entire `equipment` kind (11 of 1,741
  `fixture-verified` units) — 11 live `.pcg` characters, each with one
  `Belt of Mighty Hurling`/`Shifter's Headband` item real-`EQUIPSET`-equipped into its correct
  PCGen slot, exported through `./gradlew run` for real (11/11 exit 0) against a real `ours` value
  from a live call into `codex::rules_core::equipment_effects::compute_equipment_effects` (not
  read from the corpus's `raw_bonus_chains` field directly — that would only check ingestion, not
  computation). Compared via `scripts/oracle_harness/run.py` (`AT-33-E2-003`'s CLI, unmodified).
- **Figures:**
  - `fixture-verified` population: 1,741 of 49,438
    (`jq '[.units[]|select(.status=="fixture-verified")]|length' docs/work-inventory.json`)
  - Examined against a live oracle round-trip this cycle: 11 of 1,741 (0.63%)
    (`AT-33-E5-001_cycle_receipt.md`'s per-unit table; source `equipment.oracle-results.json`)
  - Agreement: 11 of 11 examined; disagreement: 0 of 11 examined
    (`python3 scripts/oracle_harness/run.py --oracle-export artifacts/epic-5-reverification/equipment.oracle-export.txt --ours artifacts/epic-5-reverification/equipment.ours.json --output <out>.json`)
  - `box_ledger.py --check` against this cycle's real oracle-results: `uncovered=0 overlap=0
    population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False`, exit 0
    (`python3 scripts/box_ledger.py --check --oracle-results artifacts/epic-5-reverification/equipment.oracle-results.json`)
  - Not yet examined, real probe exists (`spell`+`class_feature`): 1,303 of 1,741
    (`jq -r '[.units[]|select(.status=="fixture-verified" and (.kind=="spell" or .kind=="class_feature"))]|length' docs/work-inventory.json`)
  - Not yet examined, no probe exists at all (`companion`+`monster`+`monster_ability`): 427 of 1,741
    (`jq -r '[.units[]|select(.status=="fixture-verified" and (.kind=="companion" or .kind=="monster" or .kind=="monster_ability"))]|length' docs/work-inventory.json`)
- **Movement (four buckets):** closure 0 / reclassification 0 / reachability 0 /
  instrument-correction 0 — no `docs/work-inventory.json` `status` field changed this cycle;
  the oracle-agreement result is recorded in `equipment.oracle-results.json`, not as a status
  transition.
- **RED→GREEN:** before this cycle, `AT-33-E5-001`'s evidence obligation had zero real per-unit
  rows against the 1,741 population (`equipment.oracle-results.json` did not exist, no
  `fixture-verified` unit had an authored `.pcg`/template pair). After: 11 real rows, each backed
  by an independently-executed live PCGen export (11/11 `./gradlew run` exit 0) and a live engine
  call (`cargo run --release` against the real `codex` crate, exit 0) — `agree=11 disagree=0
  unverifiable=0`, cross-checked by `box_ledger.py --check` exiting 0 on the same file.
- **Status: in-progress, not `complete`.** 11 of 1,741 is genuine progress, not the full
  population the criterion's Evidence line asks for. Per `workflow-instruction.md §8` /
  `AGENTS.md`'s blocker-closure doctrine, a blocker bigger than one cycle is decomposed and run
  across cycles, not exempted or marked done early — marking this row `complete` on 11 of 1,741
  would be the false-100% shape `decisions.md §2` and `AGENTS.md` rule 2 exist to prevent. No
  `## Open blockers` entry filed; the bundle is not paused. Kanban row 16 stays `in-progress`.
- **Notes:** full methodology, the honest 6-kind partition, and a concrete next-cycle plan (per
  sub-population) in `artifacts/epic-5-reverification/README.md`. `AT-33-E5-002`
  (6,589 `literal-verified` units) is a separate criterion, not started by this cycle.
- **Test scoping:** ran `scripts/oracle_harness/run.py` and `scripts/box_ledger.py --check`
  (both Epic 2/Epic 1 tools, unmodified, against this cycle's real output). Did not re-run
  `scripts/tests/test_oracle_harness.py`/`test_box_ledger.py` (neither file changed this cycle).
  Did not run the codex repo's own `cargo test`/`cargo build` (no `src/` file changed — the
  scratch `equip_probe` program lives outside the repo). Did not run `apps/desktop/src-tauri`
  (separate cargo workspace, untouched).
- **Receipt:** `artifacts/epic-5-reverification/AT-33-E5-001_cycle_receipt.md`.

### Cycle AT-33-E4-001..003 — unknown classification (rows 13-15, Epic 4)

- **Criteria:** `AT-33-E4-001` (root cause established before any count moves), `AT-33-E4-002`
  (the 4,224 reach zero, movement in four buckets), `AT-33-E4-003` (nothing lands in a bucket meaning
  "we did not look").
- **Commits:** `5bce7235d6` (E4-001, root-cause doc), `00ca087775` (E4-002, classifier fixes +
  regenerated inventory), `acdc10de3f` (E4-003, `THE-BOX.md` updated).
- **Files:** `artifacts/epic-4-unknown-classification/unknown-rootcause.md` (new),
  `src/bin/v06_work_inventory.rs` (5 `classify()` call sites + `STATUS_VOCABULARY` + 12 test
  assertions), `docs/work-inventory.json` (regenerated, sole-writer scope), `THE-BOX.md`
  (append-only: group counts, `unknown` → `unmeasurable` rename).
- **Root cause (`unknown-rootcause.md`):** 5 evidence shapes inside the 4,224. Three
  (519 + 309 + 26 = 854) were instrument asymmetry — every sibling `Kind` already reads
  `ingested-magnitude` for the identical "real magnitude, no observed consumer" shape, or
  `wiring_class::signals`' own guard conditions prove the closure carries a magnitude the record's own
  line does not show. One (3,052, `ClassFeature` owner-unresolved) is the same instrument asymmetry
  for the disposition (its `text_only` sibling already reaches `not-ingested` off the identical probe
  finding), plus a named, unattempted research opportunity (2 registered pools vs 1,128 distinct
  unmatched group prefixes). The remainder (270 + 48 = 318) is genuinely irreducible this cycle: no
  existing status honestly fits either shape (a truly-empty corpus record; a served description
  corrupted by an upstream PI/not-implemented marker, outside this file's write scope).
- **Figures:** before `4,224` at `status:"unknown"` (`jq '[.units[]|select(.status=="unknown")]|length'
  docs/work-inventory.json`, denominator `49,438` total units); after `0`. Per-unit verified movement
  (`id`-keyed join, not aggregate deltas): `3,052 → not-ingested`, `854 → ingested-magnitude`,
  `318 → unmeasurable` (renamed from `unknown`, disposition unchanged); `3052+854+318=4224` exact.
  `box_ledger.py --check` → `uncovered=0 overlap=0 population=49438 unverifiable_done=0 stale=False`,
  no warnings.
- **Movement, four buckets:** closure 0 / reclassification 3,906 / reachability 0 /
  instrument-correction 318.
- **Discovery, disclosed, not counted in the four buckets above:** the committed inventory was stale
  by 3,985 units unrelated to `unknown` (last regenerated 2026-08-23; real SD-32 engine work landed
  since was never captured). Full breakdown in `AT-33-E4-002`'s receipt.
- **RED → GREEN:** pre-fix pinned tests encoded `"unknown"` as correct for these shapes; confirmed RED
  for the intended reason, fixed `classify()`, updated the 12 assertions to the new honest values,
  `cargo test --bin v06_work_inventory` → 359/359.
- **Test scoping:** ran `cargo test --bin v06_work_inventory` (this criterion's only in-scope binary).
  Did not run a full workspace sweep or `apps/desktop/src-tauri` (separate cargo workspace, untouched
  by this cycle).
- **Cross-file follow-up, disclosed, not a blocker:** `scripts/observer/pf1e_dashboard_producer.py`
  still names `status == "unknown"` in its fail-closed `(wiring_class, status)` table; outside this
  criterion's write scope, a one-line fix for whichever cycle next touches that file.
- **Receipts:** `artifacts/epic-4-unknown-classification/AT-33-E4-001_cycle_receipt.md`,
  `AT-33-E4-002_cycle_receipt.md`, `AT-33-E4-003_cycle_receipt.md`.

### Cycle AT-33-E3-001..004 — engine coverage (rows 9-12, Epic 3)

- **Criteria:** `AT-33-E3-001` (root-cause), `AT-33-E3-002` (F1 gap closes), `AT-33-E3-003`
  (F2-F9 close), `AT-33-E3-004` (corpus-wide run reports 100%, 11,652 of 11,652, with its denominator).
- **Files:** `src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs` (modified),
  `artifacts/epic-3-engine-coverage/coverage-gap-rootcause.md` (new),
  `artifacts/epic-3-engine-coverage/formula_interpreter.corpus-wide.json` (new, SD-33's own path —
  SD-32's `artifacts/gate-2-engines/...` never touched), `THE-BOX.md` (append-only note, no
  group/count changed).
- **Root cause (execution-verified, not assumed):** two independent staleness layers, not a code
  defect. (1) SD-32's committed Gate-2 run artifact (`population=4,798`) predates 9 later commits
  (`25dbee17aa..80329736f4`) that grew its own Gate-1 census inside SD-32 itself
  (`ledger.json` F1..F9 grew to 11,338) and was never regenerated — 6,540 of the 6,854-unit gap
  (95.4% of 6,854). (2) that frozen census is itself stale against the CURRENT corpus/inventory —
  314 more units exist today (11,652 of 11,652 fresh `python3 scripts/shape_ledger.py` rollup) —
  4.6% of 6,854. Full trace with concrete sample unit coordinates and commit SHAs:
  `artifacts/epic-3-engine-coverage/coverage-gap-rootcause.md`.
- **Fix:** `formula_interpreter_corpus_wide.rs` no longer reads SD-32's frozen
  `docs/release/SD-32-.../artifacts/gate-1-shape-closure/ledger.json`. It regenerates the Gate 1
  census fresh, at scan time, by invoking `scripts/shape_ledger.py` (never re-implemented in Rust —
  `decisions.md §4` single-source-of-truth), caching the result process-wide (`OnceLock`) so
  `cargo test`'s several `#[test]` fns in this module share one ~28s regeneration.
- **Figures:**
  - True F1..F9 population (`README.md §4` row E) = **11,652** —
    `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json` (family rollup summed)
  - Prior committed run population = **4,798 of 11,652** —
    `jq .total_population docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-2-engines/formula_interpreter.corpus-wide.json`
  - Fresh SD-33 run population = **11,652 of 11,652** —
    `jq .total_population docs/release/SD-33-computed-value-verification/artifacts/epic-3-engine-coverage/formula_interpreter.corpus-wide.json`
  - Row G comparison (E − new F): `11,652 − 11,652 = 0`
  - F1 (largest family): true = run = **6,308 of 6,308** (up from the prior run's 1,790)
  - Per-family run-population == true-population for all of F1..F9: full table in the receipt
  - Recognition (separate from coverage, per the epic-breakdown NOTE): **10,626 of 11,652**
    recognised, **240 of 11,652** refused (named, e.g. unrecognised `var("CL=...")`), **786 of
    11,652** unjoined (this module's own join is narrower than `shape_ledger.py`'s three-way join —
    named forward scope, not silently folded into "recognised")
- **Movement (four buckets):** closure 6,854 (the full population gap — 6,854 previously-un-walked
  F1..F9 units now walked and either recognised or named-refused) / reclassification 0 /
  reachability 0 / instrument-correction 6,854 (the "41%"/"4,798 of 11,652" figures are corrected
  to their real cause, staleness — both bucket counts describe the same movement from two angles:
  the population figure closes, and the prior figure is corrected).
- **RED→GREEN:** new test
  `f1_population_matches_the_current_true_formula_bearing_count_not_the_stale_sd32_census`. RED
  (module still reading the frozen census): `assertion left == right failed ... left: 6032 right:
  6308`. GREEN (module regenerates fresh): `cargo test --locked --lib -p codex
  rules_core::pilot_compute::formula_interpreter_corpus_wide::tests::f1_population...` → `1 passed`.
  Existing `a_subset_run_trips_the_population_mismatch_check` re-pointed at the new fresh-census
  function and still green.
- **Test scoping:** ran `cargo test --locked --lib -p codex formula_interpreter` (42/42 passed —
  both `formula_interpreter` and `formula_interpreter_corpus_wide` modules, matched by substring)
  and `cargo build --locked --lib -p codex` (clean, pre-existing unrelated warnings only). Did not
  run the full `cargo test --locked --lib -p codex` workspace sweep (2,824+ tests, no other module
  touched) or `apps/desktop/src-tauri` (separate cargo workspace, untouched).
- **Receipt:** `artifacts/epic-3-engine-coverage/AT-33-E3-001..004_cycle_receipt.md`.

### Cycle AT-33-E2-004 — oracle-path-ruling (row 8, Epic 2)

- **Criterion:** `AT-33-E2-004` — the Path A / Path B ruling is recorded and escalated.
- **Commit SHA:** `84a5781c11`
- **Files:** `artifacts/epic-2-oracle-harness/oracle-comparison-fixtures.md` (new — carries the ruling), `artifacts/epic-2-oracle-harness/AT-33-E2-004_cycle_receipt.md` (new), `progress.md` (this entry).
- **Ruling: Path A.** All three named risks (`decisions.md §5`) resolved in Path A's favor by execution (`AT-33-E2-001`); a real round-trip export produced real values (`AT-33-E2-002`); the comparison harness is built and proven live (`AT-33-E2-003`).
- **Figures:**
  - Named risks resolved without forcing Path B: 3 of 3 named in `decisions.md §5` (`AT-33-E2-001_cycle_receipt.md`)
  - Path B fallback invocations this cycle: 0 of 1 (Epic 2's own spike) (no Java-source-reading fallback file exists under `artifacts/epic-2-oracle-harness/`)
- **Consequence for Epic 5:** none negative — Epic 5 can run the live-PCGen path this cycle proved, at full mechanism availability, rather than the slower per-shape Path B fallback. **No escalation filed** — `decisions.md §5`'s escalation clause is conditioned on Path A *failing*, and it did not.
- **Movement (four buckets):** closure 0 / reclassification 0 / reachability 0 / instrument-correction 0 — this is a ruling, not a unit-status change.
- **Receipt:** `artifacts/epic-2-oracle-harness/AT-33-E2-004_cycle_receipt.md`.

### Cycle AT-33-E2-003 — oracle-comparison-harness (row 7, Epic 2)

- **Criterion:** `AT-33-E2-003` — the comparison harness answers the per-unit question.
- **Commit SHA:** `84a5781c11`
- **Files:** `scripts/oracle_harness/__init__.py`, `scripts/oracle_harness/compare.py`, `scripts/oracle_harness/oracle_export.py`, `scripts/oracle_harness/run.py` (all new), `scripts/tests/test_oracle_harness.py` (new), four fixture JSONs under `artifacts/epic-2-oracle-harness/fixtures/` (new), `artifacts/epic-2-oracle-harness/oracle-comparison-fixtures.md` (new).
- **What landed:** `compare_unit(unit_id, ours, oracle)` returns `{"unit_id","ours","oracle","verdict"}`, `verdict ∈ {agree, disagree, unverifiable}`; `unverifiable` is a normal return value on a missing/blank oracle value, never an exception, never folded into `agree`. `run_comparison`/`run.py` produce the exact shape `scripts/box_ledger.py::load_oracle_results` reads.
- **Figures:**
  - Unit test suite (new): 16 passed, 0 failed, of `scripts/tests/test_oracle_harness.py`'s own 16 cases (`python3 -m unittest scripts.tests.test_oracle_harness -v`)
  - Combined with existing box_ledger suite: 41 passed, 0 failed, of both files' combined 41 cases (`python3 -m unittest scripts.tests.test_oracle_harness scripts.tests.test_box_ledger -v`)
  - Live CLI run, agree-only fixture: agree=4, disagree=0, unverifiable=1, of 5 units (`python3 scripts/oracle_harness/run.py --oracle-export .../pf1_fighter_l1.computed.txt --ours .../fixtures/pf1_fighter_l1.ours-sample.json --output .../fixtures/pf1_fighter_l1.oracle-results-demo.json`)
  - Live CLI run, known-disagreeing fixture: agree=3, disagree=1, unverifiable=1, of 5 units (same command with `ours-sample-with-bug.json`) — then fed to `python3 scripts/box_ledger.py --check --oracle-results .../fixtures/pf1_fighter_l1.oracle-results-demo-DISAGREE.json` → exit 1, `oracle_disagreement=1`
- **Fixture discipline:** every `oracle=...` literal in the unit tests was hand-transcribed from the real committed `pf1_fighter_l1.computed.txt` bytes (read by eye, typed as a Python literal); the test file never opens that file, and the one test class that *does* exercise the real parser (`OracleExportParsingTest`) uses only an inline string, never the committed file.
- **Movement (four buckets):** closure 0 / reclassification 0 / reachability 0 / instrument-correction 0 — this cycle builds and proves the instrument; the demo fixtures use synthetic unit ids scoped to this cycle's `.pcg`, not real `docs/work-inventory.json` units.
- **RED→GREEN:** `ImportError: cannot import name 'compare' from 'oracle_harness'` before the package existed (intended reason); 16/16 green after.
- **Receipt:** `artifacts/epic-2-oracle-harness/AT-33-E2-003_cycle_receipt.md`.

### Cycle AT-33-E2-002 — oracle-character-roundtrip (row 6, Epic 2)

- **Criterion:** `AT-33-E2-002` — a character round-trips through the oracle.
- **Commit SHA:** `84a5781c11`
- **Files:** `artifacts/epic-2-oracle-harness/fixtures/pf1_fighter_l1.pcg` (new), `artifacts/epic-2-oracle-harness/computed-values.txt.ftl` (new), `artifacts/epic-2-oracle-harness/pf1_fighter_l1.computed.txt` (new), `artifacts/epic-2-oracle-harness/build-transcript-05-batchexport-SUCCESS.log` (new).
- **What landed:** a hand-authored Level 1 Human Fighter `.pcg` (Core Rulebook only) exported through the pinned PCGen's `BatchExporter` via a hand-authored FreeMarker template emitting `pcstring(...)`-token computed variables (HP, AC, BAB, `VAR.CMB`/`VAR.CMD`, all three saves) as machine-readable `KEY=VALUE` lines.
- **Figures:**
  - Export command exit code: 0, of 1 (final, corrected) attempt (`build-transcript-05-batchexport-SUCCESS.log`, last line `BUILD SUCCESSFUL`)
  - `SEVERE`-level log lines: 0, of the full transcript (`grep -c SEVERE artifacts/epic-2-oracle-harness/build-transcript-05-batchexport-SUCCESS.log`)
  - Independently-derived RAW values matching the real oracle output: 13 of 13 fields checked (table in the cycle receipt; re-derive the oracle side with `cat artifacts/epic-2-oracle-harness/pf1_fighter_l1.computed.txt`)
- **Movement (four buckets):** closure 0 / reclassification 0 / reachability 0 / instrument-correction 0 — proves a round-trip mechanism, moves no inventory unit.
- **Notes:** first export attempt failed for the intended reason (`data/homebrew`/`data/_universal` outside the checkout's initial sparse scope) — see `AT-33-E2-001`'s entry below.
- **Receipt:** `artifacts/epic-2-oracle-harness/AT-33-E2-002_cycle_receipt.md`.

### Cycle AT-33-E2-001 — oracle-path-a-feasibility (row 5, Epic 2)

- **Criterion:** `AT-33-E2-001` — Path A feasibility is established by execution.
- **Commit SHA:** `84a5781c11`
- **Files:** `artifacts/epic-2-oracle-harness/README.md` (new), `artifacts/epic-2-oracle-harness/.gitignore` (new), `artifacts/epic-2-oracle-harness/build-transcript-{01..04}-*.log` (new).
- **What landed:** fetched the pinned PCGen (`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`) into a scratch, gitignored, cone-mode sparse checkout inside this cycle's own write scope (never `~/workspace/repos/pcgen`), and ran `./gradlew --version`/`compileJava`/`jar` for real on `OpenJDK 25 Temurin`.
- **All three named risks (`decisions.md §5`) resolved to facts:**
  1. Gradle vs Java 25 — not a conflict (`build.gradle` pins `javaVersion = 25`; Gradle `9.5.1` ran cleanly).
  2. `pcgen.gui2.UIPropertyContext` coupling — real (registered even in batch mode) but non-blocking (its properties are `javafx.scene.paint.Color` value objects, no display-server call; confirmed by a successful headless export in `AT-33-E2-002`).
  3. `.pcg` input authoring — solved by hand-authoring one, using the repo's own `code/testsuite/PCGfiles/*.pcg` samples only to confirm tag vocabulary.
- **Figures:**
  - Named risks resolved to a fact: 3 of 3 named in `decisions.md §5` (manual: read `Main.java`/`UIPropertyContext.java`, then the commands below)
  - `./gradlew compileJava` first attempt: exit 1, of 1 attempt, failed for the intended reason (missing `PCGen-Formula` subproject dir in the initial sparse cone) (`build-transcript-02-compileJava-first-attempt-FAILED.log`)
  - `./gradlew compileJava` corrected attempt: exit 0, of 1 attempt (`build-transcript-03-compileJava-SUCCESS.log`)
  - Plugin jars produced: 11 of 11 `createJarTask` calls in `code/gradle/plugins.gradle` (`ls pcgen-oracle-checkout/plugins/*.jar | wc -l`)
- **Movement (four buckets):** closure 0 / reclassification 0 / reachability 0 / instrument-correction 0 — proves a build-feasibility fact, moves no inventory unit.
- **Receipt:** `artifacts/epic-2-oracle-harness/AT-33-E2-001_cycle_receipt.md`.

### Cycle AT-33-E1-004 — denominator-gate (row 4, Epic 1)

- **Criterion:** `AT-33-E1-004` — the denominator gate is a real `scripts/verify.sh` stage.
- **Files:** `scripts/denominator_gate.py` (new), `scripts/tests/test_denominator_gate.py` (new),
  `scripts/verify.sh` (extended — new `denominator-gate` stage in both stage sets + dispatch case).
- **What landed:** a line-level check — a line carrying a bare percentage with no denominator
  marker (`of <N>` / `out of <N>` / `<N>/<M>` fraction / literal `denominator <N>`) anywhere on
  that same line is a violation. Wired into `scripts/verify.sh`'s stage list directly (not a
  standalone script — closes the `SD-31-.../forward-scope-register.md` C1.8 gap named for
  `v06_corpus_trap_report`), in both `ALL_STAGES` and `QUICK_STAGES`. Default scope is
  deliberately this bundle's own generated evidence (`artifacts/**/*_cycle_receipt.md` +
  `progress.md`) — not this bundle's planning prose (outside this criterion's write scope) and not
  every prior bundle's receipts (261 files repo-wide, unaudited, a separate task); overridable via
  `DENOMINATOR_GATE_PATHS`.
- **Figures:**
  - Unit test suite (new): 17 passed, 0 failed (`python3 -m unittest scripts.tests.test_denominator_gate -v`)
  - Regression: `test_box_ledger.py` 25/25, `test_probe_surface_census.py` 11/11 — 36/36
    (`python3 -m unittest scripts.tests.test_box_ledger scripts.tests.test_probe_surface_census -v`)
  - Live default-scope check: 4 files checked, 0 violations (`python3 scripts/denominator_gate.py --check`)
  - Stage present in both stage sets: `bash scripts/verify.sh --list | grep denominator-gate` →
    `denominator-gate     yes   yes`
- **Movement (four buckets):** closure 0 / reclassification 0 / reachability 0 /
  instrument-correction 0 — this cycle builds an instrument (a gate with an exit code); it moves
  no inventory unit.
- **RED→GREEN:** `ModuleNotFoundError: No module named 'denominator_gate'` before the module
  existed (intended reason); 17/17 green after. **The criterion's own evidence obligation** — a
  mutation proof through `scripts/verify.sh --only denominator-gate` itself, pointed via
  `DENOMINATOR_GATE_PATHS` at a synthetic fixture whose only figure is a bare, undenominated
  percentage (`decisions.md` §2's own motivating shape) → `FAIL`, exit 1; the identical fixture
  corrected to state 97.9% of 4,798 and 41% of 11,652, denominator in the same construct → `PASS`,
  exit 0; default invocation with no override, against the real committed 4-file scope → `PASS`,
  exit 0. Full transcripts in the receipt.
- **Notes:** scope is deliberately narrow — this bundle's own receipts + `progress.md`, not
  repo-wide and not this bundle's own planning prose (which narrates the same 41%-of-11,652 /
  97.9%-of-4,798 figures `decisions.md` §2 cites as the motivating defect, and is outside this
  criterion's write scope). See the receipt's Notes for the full reasoning and the 261-file
  repo-wide sweep that informed the scoping decision.
- **Test scoping:** ran `scripts/tests/test_denominator_gate.py` (17/17, new) and
  `scripts/tests/test_box_ledger.py` + `test_probe_surface_census.py` (36/36, regression).
  `bash -n scripts/verify.sh` (syntax check) and `bash scripts/verify.sh --only denominator-gate`
  (three invocations — default GREEN, override RED, override GREEN). Did not run `scripts/verify.sh`
  in full (other stages' preconditions unrelated to this cycle's files), the Rust workspace (no
  `.rs` file touched), or `apps/desktop/src-tauri` (separate cargo workspace, untouched).
- **Receipt:** `artifacts/epic-1-instruments/AT-33-E1-004_cycle_receipt.md`.

### Cycle AT-33-E1-003 — probe-surface-census (row 3, Epic 1)

- **Criterion:** `AT-33-E1-003` — the probe surface is enumerated for real.
- **Files:** `scripts/probe_surface_census.py` (new), `scripts/tests/test_probe_surface_census.py`
  (new), `artifacts/epic-1-instruments/probe-surface-census.json` (new, generated).
- **What landed:** every corpus `kind` (19, live) enumerated by reading `src/bin/v06_work_inventory.rs`'s
  exhaustive verdict match arm-by-arm, cross-checked against live evidence strings for every claim
  (not from memory or prior prose, per `decisions.md §7`). 8 kinds carry a probe function that
  changes an input and observes a delta on a rendered computed snapshot (`class`, `class_feature`,
  `feat`, `spell`, `equipment`, `equipment_modifier`, `race`, `race_trait`); 11 do not — 8 with no
  engine table at all, 3 with an engine table but only a presence/lookup check (`monster`,
  `monster_ability`, `companion`).
- **Figures:**
  - Distinct corpus `kind` count: 19 (`jq -r '.units[].kind' docs/work-inventory.json | sort -u | wc -l`)
  - Kinds with a magnitude probe: **8 of 19**; without: **11 of 19**
    (`python3 scripts/probe_surface_census.py --check` → `kinds_with_probe=8 kinds_without_probe=11`)
  - Units covered by a probe-bearing kind: 34,246 of 49,438
    (`jq '[.units[] | select(.kind | IN("class","class_feature","feat","spell","equipment","equipment_modifier","race","race_trait"))] | length' docs/work-inventory.json`)
  - Units in a no-probe kind: 15,192 of 49,438
    (`jq '[.units[] | select(.kind | IN("monster","monster_ability","companion","ability","template","deity","power","domain","skill","language","trait"))] | length' docs/work-inventory.json`)
  - Per-kind probe-fire confirmation (live, execution-derived): `class`=28, `class_feature`=26,
    `feat`=108, `spell`=966, `equipment`+`equipment_modifier`=605, `race`=39, `race_trait`=309 real
    units each — proving each claimed probe genuinely fired, not merely exists in source. All 11
    no-probe kinds confirmed at 0.
  - Full per-kind table with unit counts and re-derive commands: cycle receipt.
- **Movement (four buckets):** closure 0 / reclassification 0 / reachability 0 /
  instrument-correction 0 — this cycle builds and runs an enumeration instrument; it moves no
  unit's status.
- **RED→GREEN:** `ModuleNotFoundError: No module named 'probe_surface_census'` before the module
  existed (intended reason); 11/11 green after, including 3 live-corpus acceptance tests run
  against the real `docs/work-inventory.json`. 3 mutation proofs on `--check`'s fail-closed gate:
  an unmapped-kind unit, a `probe_exists:true` kind whose only unit never fires the probe, and a
  `probe_exists:false` kind carrying probe-shaped evidence — all three correctly detected and
  reported by name. Regression: `scripts/tests/test_box_ledger.py` re-run, 25/25 still green
  (untouched this cycle).
- **Notes:** presence-only lookups (`monster`/`monster_ability`/`companion`) are deliberately NOT
  counted as magnitude probes even though the inventory's own vocabulary calls their result
  `grounded` — the criterion's bar is "can verify a computed magnitude", and a `holds_key` table
  lookup answers a different, weaker question. See the receipt's Notes for the full reasoning and
  the recursive-find hazard check (confirmed no probe implementation exists outside
  `v06_work_inventory.rs`'s sibling `v06_content_state_dump.rs`, and confirmed `data/corpus/`'s
  extra `*_generic`/`_parity` directory names are storage-layout artifacts, not a 20th kind).
- **Test scoping:** ran `scripts/tests/test_probe_surface_census.py` (11/11) and
  `scripts/tests/test_box_ledger.py` (25/25, regression). Did not run `scripts/verify.sh` (any
  stage — `AT-33-E1-004` owns wiring this cycle's files into it), the Rust workspace, or
  `apps/desktop/src-tauri` (no `.rs` file touched).
- **Receipt:** `artifacts/epic-1-instruments/AT-33-E1-003_cycle_receipt.md`.

### Cycle AT-33-E1-002 — box-fail-closed (row 2, Epic 1)

- **Criterion:** `AT-33-E1-002` — `box_ledger.py` fails closed on all five conditions.
- **Files:** `scripts/box_ledger.py` (extended), `scripts/tests/test_box_ledger.py` (extended), `THE-BOX.md` (extended — `"unverifiable"` field on every ledger group).
- **What landed:** conditions 3-5 (oracle disagreement / an `unverifiable` unit dispositioned `done` / `derived_at` staleness gate) added to the same `box_ledger.py` `AT-33-E1-001` built; conditions 1-2 (uncovered/overlap) were already implemented and are re-verified here, not re-implemented.
- **Figures:**
  - `box_ledger.py --check` on the committed `THE-BOX.md` → `uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False` (`python3 scripts/box_ledger.py --check`)
  - Unit test suite: 25 passed, 0 failed (`python3 -m unittest scripts.tests.test_box_ledger -v`) — 9 carried from the prior cycle, 16 new
  - `unknown` (unverifiable) group population, used in two of the five mutation proofs: 4,224 of 49,438 (`jq '[.units[]|select(.status=="unknown")]|length' docs/work-inventory.json`)
- **Movement (four buckets):** closure 0 / reclassification 0 / reachability 0 / instrument-correction 0 — this cycle extends the instrument, moves no unit.
- **RED→GREEN:** 12 `AttributeError`s + 3 `AssertionError`s before implementation (intended reason: the three new mechanisms didn't exist); 25/25 green after. **Five live mutation proofs, one per condition, against the real committed `THE-BOX.md` (or a temp copy with exactly one mutation)** — full transcripts in the receipt: (1) `unknown` group deleted → `uncovered=4224`, exit 1; (2) a colliding group added → `overlap=5099`, exit 1; (3) a real `--oracle-results` fixture with one `verdict: disagree` record → `oracle_disagreement=1`, exit 1, corrected to `agree` → exit 0; (4) the real `unknown` group's disposition changed `unverifiable`→`done` (its own `unverifiable: true` flag left on — reproduces SD-32's exact over-claim) → `unverifiable_done=4224`, exit 1; (5) `derived_at` replaced with a fabricated SHA → `STALE:` + exit 1, committed file's real SHA → exit 0. Every RED case's corresponding GREEN is the untouched committed file, exit 0.
- **Notes:** oracle-disagreement check is wired now (reads `AT-33-E2-003`'s harness output shape), not deferred — it activates automatically once Epic 2 lands `oracle-results.json`, no second cycle needed. `"unverifiable"` is a ledger-group-level flag, not a per-unit field, because `uncovered==0 overlap==0` already makes a unit's disposition equal to its one group's disposition.
- **Test scoping:** ran `scripts/tests/test_box_ledger.py` only (25/25 green) — this criterion's whole file-touch set. Did not run `scripts/verify.sh` (`AT-33-E1-004`'s stage, not yet wired), the Rust workspace, or `apps/desktop/src-tauri` (no files in either changed).
- **Receipt:** `artifacts/epic-1-instruments/AT-33-E1-002_cycle_receipt.md`.

### Cycle AT-33-E1-001 — box-partition (row 1, Epic 1)

- **Criterion:** `AT-33-E1-001` — `THE-BOX.md` exists as a living partition of the full inventory.
- **Files:** `scripts/box_ledger.py` (new), `scripts/tests/test_box_ledger.py` (new), `THE-BOX.md` (new).
- **Figures:**
  - population = 49,438 (`jq '.units | length' docs/work-inventory.json`; cross-checked against `jq '.totals.units' docs/work-inventory.json`, both agree — no correction needed)
  - `box_ledger.py --check` → `uncovered=0 overlap=0 population=49438` (`python3 scripts/box_ledger.py --check`)
  - 9 groups partition the population by the inventory's `status` field (already exhaustive and non-overlapping — 9 distinct non-null values, 0 duplicate unit ids); group counts: `grounded` 3,234, `literal-verified` 6,589, `fixture-verified` 1,741, `ingested-magnitude` 1,543, `text-complete` 5,099, `deferred-with-reason` 46, `not-ingested` 26,943, `not-started` 19, `unknown` 4,224 — each command in `THE-BOX.md`'s table, sum = 49,438.
- **Movement (four buckets):** closure 0 / reclassification 0 / reachability 0 / instrument-correction 0 — this cycle builds the instrument, moves no unit.
- **Explicit `unverifiable` bucket** (`decisions.md §7`): the `unknown` group, 4,224 units — owned by Epic 4 to move.
- **RED→GREEN:** `python3 -m unittest scripts.tests.test_box_ledger` failed with `ModuleNotFoundError` before `box_ledger.py` existed (intended reason); 9/9 green after, including the live-corpus acceptance case. Mutation proof: `THE-BOX.md` copy with the `unknown` group deleted correctly failed closed (`uncovered=4224`, exit 1); committed file untouched.
- **Receipt:** `artifacts/epic-1-instruments/AT-33-E1-001_cycle_receipt.md`.
