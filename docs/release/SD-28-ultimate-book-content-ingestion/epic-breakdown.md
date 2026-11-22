---
canonical: true
owner: god-emporer
status: planning-ready (operator directives 2026-08-01)
date: 2026-08-01
canonical_branch: tranche/8 (operator directive 2026-08-01)
build_version_target: 0.8.<build>
companion_to: ./scope-draft.md, ./decisions.md
---

# SD-28 Epic Breakdown

12 epics × ~3 acceptance criteria = ~36 criteria. Mirrors SD-22's epic
shape with the seven-book expansion (UC, UM, UE, UI, UCam, UW, UPsi) →
seven per-book content-source-ingest epics, plus Epic 12's end-of-run
code review (operator directive 2026-08-01, added post-launch).

Epic 1 fires FIRST. Epic 10 (Closure Epilogue) fires LAST — corrected
here from a prior "Epic 11 fires LAST" statement that was never true
against this file's own "Recommended sequencing" diagram below, which
has always ordered `E11 → E10`. Epics 3-9 (per-book) may run in any
order post-Epic 2, but each book is one cycle-batch (no Epic 3 cycle
interleaves with Epic 4 cycle). Epic 12 (Bundle Code Review) fires
after Epic 11 and all content epics, before Epic 10.

## Epic 1 (SD28-E1) — Code-Side Identifier Cleanup

**Objective:** Establish identifier discipline across all code this bundle introduces.

**Derived from:** `decisions.md §6` (Identifier discipline).

### Feature seeds

#### SD28-E1-F1 — Identifier-disclosure audit pass

Acceptance:

- No `sd28_*`, `SD28_*`, `Sd28*`, `sd28-*` patterns in the seven books' surface code (`src/rules_core/rules_tables/ultimate_*/`, `src/rules_core/rules_tables/dreamscarred_press/`).
- No `t_<hex>` kanban tokens in source files.
- Identifier-discipline audit script returns 0 findings.

#### SD28-E1-F2 — Schema-side grep

Acceptance:

- The four-grep dual-audit (identifier-discipline + wired-integration) runs cleanly post-Epic-1 commit.
- The CLI flags any future-leaked `SD28-*` patterns with a one-line exit-code-1 message.

## Epic 2 (SD28-E2) — Operator Pre-Launch

**Objective:** Pre-launch checklist verification before any per-book cycle fires.

**Derived from:** `loop-instruction.md §"Pre-launch checklist"` + operator directives 2026-08-01.

### Feature seeds

#### SD28-E2-F1 — Local-file dispatch readiness

Acceptance:

- `kanban.md` lists at least one ready card.
- `progress.md` exists with first-cycle placeholder.
- Working tree clean (`git status` returns no uncommitted changes).

#### SD28-E2-F2 — Branch-pushed + licensing pre-cycle verification

Acceptance:

- Branch `tranche/8` is pushed to origin (`git push -u origin tranche/8` succeeds).
- The Dreamscarred Press licensing pre-cycle verification ran against `dreamscarred_press/ultimate_psionics/` and recorded its output in `artifacts/dreamscarred-license-precheck.md`.

## Epic 3 (SD28-E3) — Ultimate Combat content-source ingest

**Objective:** Per-class / per-chooser cycles for Ultimate Combat (Gunslinger, Ninja, Samurai, new martial rules).

**Derived from:** `scope-draft.md §"Book list" slot 1` + `decisions.md §11`.

### Feature seeds

#### SD28-E3-F1 — Class records (Gunslinger, Ninja, Samurai, others)

Acceptance:

- One canonical record per class in `src/rules_core/rules_tables/ultimate_combat/` (per `decisions.md §6` PascalCase / camelCase discipline).
- Reach-gate claim executes the real IPC builder for each class.
- Trap-report output recorded in `artifacts/uc-trap-report.md`.

#### SD28-E3-F2 — Chooser-shaped cycles (panache, grit, martial flexibility)

Acceptance:

- Each chooser mechanism has a record slice that the rules-engine can read.
- Per-cycle tier-2 model swap authorized per `decisions.md §11` (free/discounted) if dispatch is templated.

## Epic 4 (SD28-E4) — Ultimate Magic content-source ingest

**Objective:** Per-class + per-spell-subsystem cycles (new casting variants, words of power, truename).

**Derived from:** `scope-draft.md §"Book list" slot 2`.

### Feature seeds

#### SD28-E4-F1 — Class records (Magus, others)

#### SD28-E4-F2 — Spell subsystems (Words of Power, Truename)

## Epic 5 (SD28-E5) — Ultimate Equipment content-source ingest

**Objective:** Per-equipment-entry cycles.

**Derived from:** `scope-draft.md §"Book list" slot 3` + `decisions.md §10 (legacy §18-resolution direction)`.

### Feature seeds

#### SD28-E5-F1 — Equipment records

Acceptance:

- One canonical record per equipment entry in `src/rules_core/rules_tables/ultimate_equipment/`.
- Reach-gate coverage for equipment. **§10 (legacy) / §18 forces the catalog widening**: if `equipment_catalog.rs` is still CRB-only when this epic closes, the cycle records the gap as `decision-blocked` and surfaces the widening as C3.1 in the forward-scope register.

## Epic 6 (SD28-E6) — Ultimate Intrigue content-source ingest

**Objective:** Per-class / per-social-rule cycles (Vigilante, and other Ultimate Intrigue classes).

**Derived from:** `scope-draft.md §"Book list" slot 4` + `decisions.md §5` cross-bundle class overlap.

### Feature seeds

#### SD28-E6-F1 — Class records (Vigilante, others)

Acceptance:

- Canonical class records in `src/rules_core/rules_tables/ultimate_intrigue/`.
- For the four classes shared with SD-30 (Occultist, Spiritualist, Medium, Mesmerist), SD-28 references the SD-30 canonical id; SD-28 does not redefine.

## Epic 7 (SD28-E7) — Ultimate Campaign content-source ingest

**Objective:** Per-system-subsystem cycles (downtime, kingdom-building, traits, retraining).

**Derived from:** `scope-draft.md §"Book list" slot 5`.

### Feature seeds

#### SD28-E7-F1 — Player-options subsystems

Acceptance:

- Trait, downtime, kingdom-building, and retraining rules have representation slices.
- Pre-computed effects (no execution engines); see `decisions.md §18` rules-as-data doctrine.

## Epic 8 (SD28-E8) — Ultimate Wilderness content-source ingest

**Objective:** Per-class + per-Companion-rules cycles.

**Derived from:** `scope-draft.md §"Book list" slot 6`.

### Feature seeds

#### SD28-E8-F1 — Class records

#### SD28-E8-F2 — Companion-rules records

## Epic 9 (SD28-E9) — Ultimate Psionics content-source ingest (Dreamscarred Press tier)

**Objective:** Per-class + per-power cycles, gated on licensing pre-cycle verification per `decisions.md §17`.

**Derived from:** `scope-draft.md §"Book list" slot 7` + `decisions.md §17`.

### Feature seeds

#### SD28-E9-F1 — Pre-cycle licensing verification

Acceptance:

- Trap-report output against `dreamscarred_press/ultimate_psionics/` records license-conformance findings.
- Any record not matching open-content tier is dropped from per-cycle scope (recorded as cycle finding).

#### SD28-E9-F2 — Class + power records

Acceptance:

- One canonical record per class and power in `src/rules_core/rules_tables/ultimate_psionics/`.
- Reach-gate coverage for each class/power.

## Epic 10 (SD28-E10) — Closure Epilogue

**Objective:** Standard part-of-handoff; tranche promotion PR fires after all per-book epics closed.

**Derived from:** `decisions.md §22` (operating form) + the build-version amendment (2026-07-17).

### Feature seeds

#### SD28-E10-F1 — Closure cycle

Acceptance:

- All Epic 3-9 per-book cycles `complete` in `progress.md`.
- `release-notes.md` populated with the seven books' per-record rollup.
- Tranche promotion PR fires: `tranche/8 → develop`; `0.8.<last_build>` remains the post-closure value.

#### SD28-E10-F2 — Workspace-tree removal (move-not-copy)

Acceptance:

- The source-of-record directory (`programs/codex/requirements/SD-28-ultimate-book-content-ingestion/`) is removed on the publish commit per `decisions.md §22`.
- The canonical repo-resident home is `docs/release/SD-28-ultimate-book-content-ingestion/`.

## Epic 11 (SD28-E11) — Build Version Numbering

**Objective:** Establish the first concrete value of the build version scheme.

**Derived from:** `decisions.md §15`.

### Feature seeds

#### SD28-E11-F1 — Version patch

Acceptance:

- First concrete value: `0.8.<build>` (read from current build counter at cycle close).
- Closing-PR iteration on Epic 10 increments per the 2026-07-17 build-version amendment.
- Major remains `0` until first main-publish.

## Epic 12 (SD28-E12) — Bundle Code Review

**Objective:** A full code review of the bundle's entire diff against its
branch point, run after every content-ingest epic (3-9) and Epic 11 (Build
Version Numbering) are closed — not in parallel with them, and not scoped to
only the final cycle. `./scripts/verify.sh` passing is a **precondition** to
this epic firing, not the review itself: a green gate says the tests that
exist pass, it says nothing about whether the code is right.

**Derived from:** operator directive 2026-08-01 (the v0.6 CRB run closed
without an end-of-run code review) + `decisions.md §26`.

### Feature seeds

#### SD28-E12-F1 — Whole-bundle diff review

Acceptance:

- The reviewed diff scope is the bundle's full change set against its branch
  point (`git diff origin/develop...HEAD`, the same merge-base triple-dot
  comparison `scripts/identifier-discipline-audit.sh` and
  `scripts/wired-integration-audit.sh` already default to via
  `BASE_BRANCH`), not the closing cycle's slice alone.
- `./scripts/verify.sh` has a recorded green run for that diff, cited as a
  precondition in the epic's receipt.
- `scripts/identifier-discipline-audit.sh` and `scripts/wired-integration-audit.sh`
  (this bundle's standing per-cycle dual-audit) are re-run once more at
  bundle scope.

#### SD28-E12-F2 — Correctness, no-stub, reach, test-quality, no-hand-authored-frontend-data sweep

Acceptance:

- A sample of this bundle's rules logic is checked against the source corpus
  for the seven books; disagreements are recorded as findings, not assumed
  away.
- No stub, fixture-only, or mock data in a production path per
  `docs/governance/no-stub-mvp-doctrine.md`; any operator-approved exception
  is entered in `docs/governance/wired-integration-stubs-registry.md`, not
  left unregistered.
- A sample of records this bundle claims reach a player surface is spot-checked
  against `reach_gate.rs`'s `OPEN_FINDINGS` mechanism and the live IPC/UI
  path — reach-gate green is necessary, not sufficient, on its own.
- Test quality, not just count: per
  `docs/governance/book-ingestion-playbook.md §7.4`, a sample of this
  bundle's new gates/tests is checked for a case that actually fails when the
  thing it protects is broken, not only a case that passes.
- No hand-authored rules data under `apps/desktop/src/` — rules content is
  sourced from `src/rules_core/rules_tables/`, never hand-typed into a
  frontend file.

#### SD28-E12-F3 — Findings triage

Acceptance:

- Every finding records a severity and a disposition: `fixed-in-bundle` or
  `deferred`. No finding is silently dropped.
- A `deferred` finding names an owner (a person or a specific successor
  bundle) and is entered in `forward-scope-register.md` — not left
  unrecorded.
- Real defects found are fixed in-bundle before Epic 10 (Closure Epilogue)
  fires; the review does not become a rubber stamp that defers everything to
  avoid scope growth.
- A `scripts/retro.py` event is emitted per finding, carrying `--verified-by`.

**Note:** the operator can separately trigger `/code-review ultra` (a
multi-agent cloud review of the branch) at any time. That path is
operator-triggered and billed — a cycle running under §21's unattended-mode
protocol cannot launch it itself — so Epic 12 must stand on its own as the
bundle's actual gate; `/code-review ultra` is a supplement, not a dependency.

## Recommended sequencing (dependency order, not exclusive scope)

```
E1 → E2 → E3, E4, E5, E6, E7, E8, E9 (any order, file-disjoint) → E11 → E12 → E10
```

The per-book epics are **file-disjoint** by source path (each writes to its own
`src/rules_core/rules_tables/<book>/`), so they can run in parallel under
operator-pinned concurrency. The classic repo-level wiring epics (E1, E2,
E11, E12, E10) are sequential. E12 (Bundle Code Review) runs after every
other epic but E10 — any review finding is fixed before the tranche-promotion
PR (part of E10) opens.

## Completion gate

SD-28 closes when:

- All Epic 3-9 per-book cycles `complete` with reach-gate claims and trap-report outputs.
- Epic 12 (Bundle Code Review) closed, all findings triaged with named owners for deferrals.
- `progress.md` carries the closure receipt.
- `release-notes.md` is populated.
- The tranche-promotion PR `tranche/8 → develop` is opened and merged.
- `docs/release/SD-28-ultimate-book-content-ingestion/` carries the canonical
  12-file chassis (post-move-not-copy publish).

---

# Completion epics (E13–E30) — 100% proven across 13 books

**Added:** 2026-08-02, operator directive ("get the 6 previously-started books
AND the 7 Ultimate books to 100% proven"). Recorded as `decisions.md §32`.

Epics 1–12 above are unchanged. One clarification applies to them: **Epics 3–9's
definition of done is now "100% proven for this book", not merely "a reach claim
exists."** A reach-gate claim plus a trap report closes a *cycle*; it does not
close the book. Each of Epics 3–9 is superseded, book-for-book, by its
corresponding completion epic below (E24–E29 for the six remaining Ultimate
books; E13 for Ultimate Campaign), which carries the measurable unit target.

## The measured starting state

All figures below are re-derived, not transcribed. Authoritative source is the
`work_inventory` section of the observer dashboard, generated by
`cargo run --bin v06_work_inventory`.

**Re-derivation command (`$WI`), used for every per-book figure in this section:**

```bash
python3 -c "
import json
d=json.load(open('/home/ubuntu/swarm-observer/PF1e-dashboard.json'))['work_inventory']
print('generated_at', d['generated_at'], 'total', d['total_units'], 'proven', d['proven_units'])
for b in sorted(d['books'], key=lambda b: b['units']-b['proven']):
    if b['proven'] < b['units']:
        print(b['id'], b['units'], b['proven'], b['units']-b['proven'], b['by_status'])
"
```

Snapshot used to author this section: `generated_at 2026-08-02T11:50:31Z`,
`total_units 44191`, `proven_units 2900`.

| Book | Units | Proven | Gap | ingested-magnitude | not-ingested | unknown | not-started |
|---|---|---|---|---|---|---|---|
| `core_rulebook` | 5716 | 912 | **4804** | 3062 | 973 | 762 | 0 |
| `advanced_players_guide` | 3605 | 657 | **2948** | 608 | 1139 | 1197 | 0 |
| `advanced_class_guide` | 3508 | 985 | **2523** | 376 | 981 | 1148 | 0 |
| `advanced_race_guide` | 2269 | 206 | **2063** | 0 | 1528 | 533 | 0 |
| `core_essentials` | 2639 | 46 | **2593** | 0 | 2593 | 0 | 0 |
| `bestiary` | 1027 | 42 | **985** | 4 | 981 | 0 | 0 |
| `pathfinder_unchained` | 882 | 52 | **830** | 0 | 297 | 532 | 0 |
| `ultimate_psionics` | 2854 | 0 | **2854** | 0 | 0 | 0 | 2854 |
| `ultimate_magic` | 2446 | 0 | **2446** | 0 | 0 | 0 | 2446 |
| `ultimate_combat` | 2182 | 0 | **2182** | 0 | 0 | 0 | 2182 |
| `ultimate_wilderness` | 2030 | 0 | **2030** | 0 | 0 | 0 | 2030 |
| `ultimate_equipment` | 1615 | 0 | **1615** | 0 | 0 | 0 | 1615 |
| `ultimate_intrigue` | 1265 | 0 | **1265** | 0 | 0 | 0 | 1265 |
| `ultimate_campaign` | 23 | 0 | **23** | 0 | 0 | 0 | 23 |
| **Total (13 books)** | **32061** | **2900** | **29161** | 4050 | 8492 | 4172 | 12415 |

Note the 14 rows: the directive says "6 previously-started books" but the
inventory reports **seven** books with a compiled rule set and non-zero proven
units. `core_essentials` is the seventh (2639 units, 46 proven). It is included;
excluding it would leave the target unreachable by definition. **Assumption
flagged for the operator:** if `core_essentials` was meant to be out of scope,
drop E21 and the total gap falls to 26,568.

**Two facts derived from the same snapshot that shape every epic below:**

1. **The 13 books contain the entire non-`not-started` corpus.** Corpus-wide
   `grounded` 301, `text-complete` 2599, `ingested-magnitude` 4050,
   `not-ingested` 8492, `unknown` 4172, `deferred-with-reason` 32 — every one of
   those figures is matched exactly by the 13-book aggregate. Only `not-started`
   differs (12415 of the corpus's 24545). Verified by:

   ```bash
   python3 -c "
   import json
   d=json.load(open('/home/ubuntu/swarm-observer/PF1e-dashboard.json'))['work_inventory']
   th=set('core_rulebook advanced_players_guide advanced_class_guide advanced_race_guide core_essentials bestiary pathfinder_unchained ultimate_combat ultimate_psionics ultimate_magic ultimate_wilderness ultimate_equipment ultimate_intrigue ultimate_campaign'.split())
   agg={}
   for b in d['books']:
       if b['id'] in th:
           for k,v in b['by_status'].items(): agg[k]=agg.get(k,0)+v
   print('13-book', agg); print('corpus  ', d['by_status'])
   "
   ```

   Consequence: no epic below can be satisfied by work in another book, and no
   other book's progress can contribute to this target.

2. **`proven` = `grounded` + `text-complete` only.** It excludes
   `ingested-magnitude`, whose own `status_vocabulary` entry in the dashboard
   reads verbatim: *"The engine holds the record WITH its real numeric fields,
   but this generator observes no consumer delta for this kind (spells,
   equipment). Strictly weaker than `grounded` and deliberately a separate word:
   calling it grounded would be the same over-claim this inventory exists to
   prevent."*

   `core_rulebook` is the extreme case: 3062 of its 5716 units (54%) are
   `ingested-magnitude` — the engine already holds them with real numbers — yet
   its proven figure is 912 (16%). **Ingesting more content cannot by itself
   reach 100% proven.** The observation harness must be widened first. E14 is
   therefore a prerequisite, not an afterthought.

## The anti-gaming rule (binding on every epic in this section)

> No epic may reach its target by reclassifying units, relaxing the classifier,
> broadening what counts as text-complete, weakening or skipping a gate, or
> editing the work-inventory generator to report more favourably. The only
> legitimate paths to proven are (a) the engine genuinely holds the record and a
> real consumer observes its magnitude, or (b) the corpus record genuinely
> carries no magnitude token (text-complete, per the operator's standing rule).
> Any unit that cannot reach proven honestly gets `deferred-with-reason`
> carrying the engine's own verbatim diagnostic, or an `OPEN_FINDINGS` entry —
> never a silent reclassification.

E14 is the one epic that changes `v06_work_inventory.rs`, and it is the sharpest
gaming risk in the whole set: widening the harness is exactly what an
unscrupulous run would do to make numbers move without doing work. E14's own
acceptance criteria bind it (see E14-F3).

## Realism statement — read this before estimating anything

2,900 proven units is **the entire proven output of this program to date**,
across every bundle that has ever run. The directive asks for 32,061 — a further
**29,161 units, roughly ten times everything achieved so far**. Of that gap:

- 12,415 units are in books with no compiled rule set at all (the seven Ultimate books);
- 8,492 are real gaps inside books already called "finished";
- 4,172 cannot even be costed yet because the generator could not classify them;
- 4,050 are already held by the engine and are blocked solely on observation.

This scope dwarfs everything the program has done to date. **No duration
estimate appears anywhere in this section.** E13 exists to produce the first
honest cost-per-unit measurement; until it reports, every other epic's duration
is explicitly **unestimated**, and any schedule asserted before then is a guess
wearing a number's clothing.

## Epic 13 (SD28-E13) — Cost calibration: one small book, end to end

**Objective:** Take `ultimate_campaign` (23 units, the smallest book in the
corpus by an order of magnitude) from 0 proven to 23 proven, and record the
actual measured cost per unit so that every later epic's estimate rests on a
measured number rather than a guess.

**Runs FIRST among the completion epics.** Every other completion epic
depends on it.

**Derived from:** operator directive 2026-08-02; `decisions.md §32`.

### Feature seeds

#### SD28-E13-F1 — `ultimate_campaign` to 100% proven

Definition of done, in units and status buckets:

- `ultimate_campaign`: `proven` 0 → **23 of 23**; `not-started` 23 → 0.
- Every unit is `grounded` or `text-complete`, or `deferred-with-reason`
  carrying the engine's verbatim diagnostic. Zero `unknown`, zero
  `not-ingested`, zero `not-started`.

Progress command:

```bash
python3 -c "
import json
b=[x for x in json.load(open('/home/ubuntu/swarm-observer/PF1e-dashboard.json'))['work_inventory']['books'] if x['id']=='ultimate_campaign'][0]
print(b['id'], b['proven'], '/', b['units'], b['by_status'])
"
```

#### SD28-E13-F2 — The calibration receipt

Acceptance:

- A receipt at `artifacts/e13-cost-calibration.md` records, per status bucket
  reached: units, wall-clock elapsed, and cycles consumed — measured, not
  estimated.
- It reports separate cost-per-unit figures for at minimum: a unit that reached
  `text-complete`, a unit that reached `grounded`, and any unit that reached
  `deferred-with-reason`. These are different kinds of work and one blended
  average would mislead every epic that consumes it.
- It states plainly which parts of the cost are per-book fixed cost (new rule
  set, module wiring, gate scaffolding) and which scale per unit. A 23-unit book
  has an unusually bad fixed-to-variable ratio; extrapolating its blended
  average to a 2,854-unit book would understate the large books.
- **The receipt is an input, not a verdict.** If it shows the directive's scope
  is not reachable within the program's means, that is recorded as a finding in
  `risks-and-open-questions.md` and surfaced to the operator, not smoothed over.

**Depends-on:** `epic-2-prelaunch`.

## Epic 14 (SD28-E14) — Observation-harness widening (spell + equipment consumers)

**Objective:** Make a spell's and an equipment item's magnitude **observable
reaching a real consumer**, the way feats and class features already are, so
that the 4,050 units currently parked in `ingested-magnitude` can reach
`grounded` on their merits.

**This is the gating prerequisite for the largest single block of already-done
work in the program.**

**Derived from:** `work_inventory.status_vocabulary["ingested-magnitude"]`;
operator directive 2026-08-02.

### Why the harness, and not the content

`src/bin/v06_work_inventory.rs:1328 fn classify()` resolves each unit by kind.
For `Kind::Feat` it consults `facts.feat_effect_wired`, populated by
`probe_feat_effect_wiring(fixture)` (`v06_work_inventory.rs:1227`) — an actual
probe that observes a computed delta, which is why feats can reach `grounded`.
For `Kind::Spell` and `Kind::Equipment`/`Kind::EquipmentModifier` **there is no
probe at all**: the classifier assigns `ingested-magnitude` structurally, from
the mere presence of a spell-list entry with a resolved level
(`evidence: "spell_list_entry_with_resolved_level"`) or an equipment-table entry
carrying a corpus magnitude
(`evidence: "equipment_table_entry_with_corpus_magnitude"`). The status is
correct and honest; the absence it reports is the harness's, not the content's.

### Affected volume (re-derived)

By-kind composition of `ingested-magnitude`, from the repo-resident inventory
shard `docs/work-inventory.json`:

```bash
python3 -c "
import json,collections
u=json.load(open('/home/ubuntu/workspace/repos/codex/docs/work-inventory.json'))['units']
c=collections.Counter((x['kind']) for x in u if x['status']=='ingested-magnitude')
print(sorted(c.items()), sum(c.values()))
"
```

→ `equipment` 2700, `spell` 1067, `equipment_modifier` 283; total **4050**,
which matches the dashboard's corpus-wide `ingested-magnitude` count exactly.
(Caveat recorded honestly: that shard's own `generated_at` is
`2026-08-02T04:02:12Z` and its `proven_units` reads 2642 against the dashboard's
11:50:31Z / 2900, so it is ~8 h stale for *totals*; its `ingested-magnitude`
by-kind split nonetheless reconciles to the current total. Re-run the command
against a fresh shard before treating the split as current.)

### Feature seeds

#### SD28-E14-F1 — Spell-effect consumer probe

Acceptance:

- A probe analogous to `probe_feat_effect_wiring` observes a **real computed
  delta** for a spell — a magnitude the spell contributes that a consumer
  actually reads — and `classify()`'s `Kind::Spell` arm consults it before
  falling through to `ingested-magnitude`.
- The consumer observed is the twin the player reads
  (`pilot_compute_corpus.rs`), per `decisions.md §29.1` / `AGENTS.md`: a
  magnitude is not wired until it moves on the twin the player reads.
- A spell the engine does **not** wire still classifies as
  `ingested-magnitude`. Proven by a test that fails when the probe is made
  permissive.

#### SD28-E14-F2 — Equipment / equipment-modifier consumer probe

Acceptance:

- Same shape, for `Kind::Equipment` and `Kind::EquipmentModifier`.
- `decisions.md §10`'s equipment-catalog widening is a dependency of this
  feature, not a parallel concern: a probe over a CRB-only
  `equipment_catalog.rs` can observe nothing for six other books.

#### SD28-E14-F3 — Anti-gaming binding on this epic specifically

Acceptance:

- The `proven_units` delta attributable to E14 alone is recorded in
  `artifacts/e14-harness-widening.md`, before/after, with the generator
  invocation.
- Each new probe ships with a **negative** test: a unit the engine genuinely
  does not wire must NOT be promoted by it. A probe that promotes everything it
  sees is the gaming failure mode this criterion exists to catch.
- No change to the `text-complete` predicate (`unit.magnitude_token_count == 0`,
  `v06_work_inventory.rs`), to the `proven` formula, or to any status
  definition. E14 adds observation; it does not redefine what counts.
- Any unit that the widened harness still cannot observe stays
  `ingested-magnitude` and is named in `OPEN_FINDINGS` — it is not moved.

Definition of done, in units and status buckets:

- Of the 4,050 `ingested-magnitude` units, every one is either promoted to
  `grounded` by an observed consumer delta, or remains `ingested-magnitude` with
  a named `OPEN_FINDINGS` entry saying what the engine does not yet do with it.
- Zero `ingested-magnitude` units without one of those two dispositions.

Progress command:

```bash
python3 -c "
import json
d=json.load(open('/home/ubuntu/swarm-observer/PF1e-dashboard.json'))['work_inventory']
print('ingested-magnitude', d['by_status'].get('ingested-magnitude'), 'grounded', d['by_status'].get('grounded'), 'proven', d['proven_units'])
"
```

**Depends-on:** `epic-13-calibration`.

## Epic 15 (SD28-E15) — `unknown` classification sweep

**Objective:** Classify the 4,172 units the generator could not classify. Until
these are classified, **nobody knows what they cost** — they are absent from
every estimate in this document by construction. Cheap relative to its value;
runs early.

**Derived from:** `work_inventory.by_status["unknown"] == 4172`, re-derived by
`$WI` above.

### What these actually are

`classify()`'s feat arm emits `unknown` with this reason, verbatim from
`v06_work_inventory.rs`: *"corpus record carries N magnitude token(s) and the
feat IS in the engine's catalog, but the feat-effect probe observed no computed
delta across the swept postures. That is the probe's documented lower-bound
behaviour: the effect may need a posture, an opponent or a combat action this
engine does not model. Reported as unknown rather than deferred because no
engine diagnostic is scoped to a feat, so there is no engine text to quote."*

So `unknown` is largely the *same* observation limit as `ingested-magnitude`,
wearing a different word because the probe exists but saw nothing. By book
(`$WI`): `advanced_players_guide` 1197, `advanced_class_guide` 1148,
`core_rulebook` 762, `advanced_race_guide` 533, `pathfinder_unchained` 532 —
these five hold all 4,172. By kind, from the 04:02Z shard: `class_feature` 3013,
`feat` 94 (this split is measurably staler than the totals; re-derive).

### Feature seeds

#### SD28-E15-F1 — Every `unknown` gets a real disposition

Definition of done, in units and status buckets:

- `by_status["unknown"]` → **0**, with every one of the 4,172 units moved to
  exactly one of: `grounded` (a consumer delta was observed once the posture the
  reason names is supplied), `text-complete` (the corpus record genuinely
  carries no magnitude token), `not-ingested` (the engine genuinely holds no
  matching record — a real gap, correctly reported), or
  `deferred-with-reason` carrying an engine diagnostic verbatim.
- No unit reaches `text-complete` by any route other than
  `magnitude_token_count == 0` on its own corpus record.

#### SD28-E15-F2 — The sweep produces a cost model, not just a reclassification

Acceptance:

- The sweep's output at `artifacts/e15-unknown-sweep.md` reports, per resulting
  bucket, how many units landed there and what each bucket now costs to finish,
  using E13's measured per-unit figures.
- Any epic below whose book contains `unknown` units restates its gap after this
  epic reports. Their targets are stated in units of *gap*, which does not move;
  their *composition* does.

Progress command:

```bash
python3 -c "
import json
d=json.load(open('/home/ubuntu/swarm-observer/PF1e-dashboard.json'))['work_inventory']
print('unknown remaining', d['by_status'].get('unknown', 0))
for b in d['books']:
    n=b['by_status'].get('unknown',0)
    if n: print(' ', b['id'], n)
"
```

**Depends-on:** `epic-13-calibration`. Runs in parallel with `epic-14-harness`
(disjoint: E14 touches the spell/equipment arms of `classify()`, E15 the
feat/class_feature arms — but both edit `v06_work_inventory.rs`, so they must
not hold uncommitted work in the same tree; dispatch with
`isolation: 'worktree'` per `decisions.md §29`).

## Epic 16 (SD28-E16) — `not-ingested` backfill inside started books

**Objective:** Close the `not-ingested` units where the book **is** ingested
but the engine holds no matching record. These are real gaps inside books
already described as finished.

**Derived from:** `work_inventory.by_status["not-ingested"]`.

> **CORRECTED 2026-08-07 by `epic-16-backfill`.** This section previously
> cited 8,492 as the target and named `core_essentials` (2,593 units) as "the
> sharpest instance." Per operator directive 2026-08-02, `core_essentials`
> and `beginner_box` are **excluded from SD-28 scope** — "redundant to other
> tomes, never coming into scope" — and the live dashboard
> (`/home/ubuntu/swarm-observer/PF1e-dashboard.json`,
> `work_inventory.excluded_books`) already applies that exclusion. The epic
> text had not absorbed the directive. Re-derived 2026-08-07 against the same
> dashboard: `work_inventory.by_status["not-ingested"]` → **5,899**, and
> `8492 - 2593 = 5899` reconciles exactly, confirming `core_essentials`'s
> 2,593 units are the entire delta and are NOT in this epic's scope. See
> `docs/retro/events/epic-16-backfill.jsonl` for the correction event.

### Composition (re-derived 2026-08-07, excluded books removed)

By book (`$WI`, dashboard `work_inventory.books[].by_status["not-ingested"]`):
`advanced_race_guide` 1528, `advanced_players_guide` 1139,
`advanced_class_guide` 981, `bestiary` 981, `core_rulebook` 973,
`pathfinder_unchained` 297 — **total 5,899**. By-kind composition should be
re-derived fresh from a current shard before use; the previously published
by-kind figures (`race_trait` 3276, `class_feature` 1665, `companion` 754,
`equipment_modifier` 417, `monster` 289, `spell` 121, `class` 58, `race` 53,
`feat` 29, `equipment` 5) summed to 8,492 against the pre-correction total and
have not been re-split against the 5,899 figure.

> **SECOND CORRECTION, 2026-08-07 by `epic-16-backfill`, same cycle.** The
> `race_trait` 3,276 figure above (and by extension a large share of the
> 5,899 total) is not a reliable measure of real remaining work. Row-by-row
> tracing of all six books' `_abilities_race.lst` files (`decisions.md §37`,
> full table and commands there) found the `Kind::RaceTrait` classifier
> overcounts `race_trait` `not-ingested` in every one of the six books, via
> three distinct mechanisms (Favored Class Bonus / `CATEGORY:Choice`
> mixing in ARG and APG; whole-file misclassification of monster/creature
> special-ability content in ACG, CRB, Bestiary, and PU). **Only APG has a
> real, non-zero, closable `race_trait` gap, and it is at most 50 units** —
> every other book's `race_trait` `not-ingested` count is either already
> fully ingested (ARG, 156/156), or contains zero genuine racial-trait
> content at all. This does not by itself produce a corrected 5,899
> replacement (that needs the same per-kind, per-book trace for
> `class_feature`/`companion`/`equipment_modifier`/`monster`/`spell`/etc.,
> not done this cycle) — it establishes that the `race_trait` component of
> that total, the largest single kind in the original by-kind split, is
> now known to be mostly noise, and that no other kind has yet been given
> the same scrutiny. **Do not launch a per-book `race_trait` ingest epic
> against any of ACG/CRB/Bestiary/PU's `not-ingested` count** without first
> running `decisions.md §37`'s row-by-row trace for that book — the
> ARG/APG mechanism (fixed) does not generalize to the other four.

### Feature seeds

#### SD28-E16-F1 — Backfill to a real record

Definition of done, in units and status buckets:

- `by_status["not-ingested"]` → **0** across the 13 books; every unit either
  reaches `grounded`/`text-complete`/`ingested-magnitude` on a genuinely
  ingested record, or `deferred-with-reason` with a verbatim engine diagnostic.
- No unit leaves `not-ingested` except by the engine actually acquiring a
  matching record. Reclassification without ingestion is the prohibited path.

#### SD28-E16-F2 — Discovery is not single-sourced

Acceptance:

- Per `decisions.md §29.3`, no content family rests on a single discovery
  source. The backfill's coverage is checked against the corpus directory
  listing, not only against `pub const NAME: &[Type]` slices, which
  `scanned_inventory()` can see but §24-shaped hand-modelled pure functions
  never emit.

Progress command:

```bash
python3 -c "
import json
d=json.load(open('/home/ubuntu/swarm-observer/PF1e-dashboard.json'))['work_inventory']
print('not-ingested remaining', d['by_status'].get('not-ingested', 0))
for b in d['books']:
    n=b['by_status'].get('not-ingested',0)
    if n: print(' ', b['id'], n)
"
```

**Depends-on:** `epic-13-calibration`, `epic-15-unknown-sweep` (the sweep can
move units *into* `not-ingested`, which is a correct outcome; running the
backfill first would leave a second pass to do).

## Epics 17–23 — Per-book completion, the seven started books

Ordered **cheapest gap first**, so that real per-book cost is learned on a small
book before a 4,804-unit one is committed to. Each is a book-scoped roll-up of
E14/E15/E16 plus whatever residue remains; each closes only when its book reads
100% proven.

**Common definition of done** (substitute the book's own id and totals):
`proven == units` for that book, with every unit in `grounded` or
`text-complete`, save units in `deferred-with-reason` carrying the engine's
verbatim diagnostic. Zero `unknown`, zero `not-ingested`, zero
`ingested-magnitude`, zero `not-started`.

**Common progress command:**

```bash
BOOK=<book_id> python3 -c "
import json,os
b=[x for x in json.load(open('/home/ubuntu/swarm-observer/PF1e-dashboard.json'))['work_inventory']['books'] if x['id']==os.environ['BOOK']][0]
print(b['id'], b['proven'], '/', b['units'], 'gap', b['units']-b['proven'], b['by_status'])
"
```

| Epic | Book | Units | Proven at start | Gap | Depends-on |
|---|---|---|---|---|---|
| **SD28-E17** | `pathfinder_unchained` | 882 | 52 | **830** | E13, E14, E15, E16 |
| **SD28-E18** | `bestiary` | 1027 | 42 | **985** | E13, E14, E15, E16, E17 |
| **SD28-E19** | `advanced_race_guide` | 2269 | 206 | **2063** | E13, E14, E15, E16, E17 |
| **SD28-E20** | `advanced_class_guide` | 3508 | 985 | **2523** | E13, E14, E15, E16, E17 |
| **SD28-E21** | `core_essentials` | 2639 | 46 | **2593** | E13, E14, E15, E16, E17 |
| **SD28-E22** | `advanced_players_guide` | 3605 | 657 | **2948** | E13, E14, E15, E16, E17 |
| **SD28-E23** | `core_rulebook` | 5716 | 912 | **4804** | E13, E14, E15, E16, E17 |

Sub-total gap, seven started books: **16,746**.

Per-book notes that change the work, not just the number:

- **E17 `pathfinder_unchained`** goes first: smallest gap, and 532 of its 830
  are `unknown` — so it is the book where E15's sweep is most directly tested
  against a completion target. Highest signal per unit of cost.
- **E18 `bestiary`**: 981 of 985 are `not-ingested` monsters. `Kind::Monster`
  reaches `grounded` through `monster_resolve` returning a real stat block, so
  this book is nearly pure ingestion with an existing observation path — no E14
  dependency in practice, but it is listed for gate uniformity.
- **E19 `advanced_race_guide`**: 1528 `not-ingested`, dominated by race traits.
  `decisions.md §29.3` names ARG explicitly as the book whose headline content
  a reach gate passed 11 tests without ever asking about. Treat gate-green here
  as necessary, not sufficient.
- **E21 `core_essentials`**: 98% `not-ingested`. This is an ingestion epic
  wearing a completion epic's name.
- **E23 `core_rulebook`** goes last and is the largest single gap in the
  program: 4,804 units, of which 3,062 (64% of its gap) are `ingested-magnitude`
  and therefore **blocked entirely on E14**, not on content work. If E14
  succeeds, this epic is far cheaper than its headline number; if E14 fails,
  this epic cannot be completed at all by any amount of ingestion. That
  conditional is the single largest uncertainty in this plan.

## Epics 24–29 — Per-book 100% proven, the six remaining Ultimate books

These **supersede** Epics 3–9 book-for-book, extending each from "a reach claim
exists" to "100% proven for this book." Epic 7 (`ultimate_campaign`) is
superseded by E13 instead, since that book is the calibration subject.

All six are entirely `not-started` — no compiled rule set — so each carries its
full unit count as its gap. Ordered smallest gap first.

**Common definition of done** and **common progress command:** as for E17–E23.

| Epic | Book | Units | Proven at start | Gap | Supersedes | Depends-on |
|---|---|---|---|---|---|---|
| **SD28-E24** | `ultimate_intrigue` | 1265 | 0 | **1265** | E6 | E13, E14, E15, E16 |
| **SD28-E25** | `ultimate_equipment` | 1615 | 0 | **1615** | E5 | E13, E14 (hard), E24 |
| **SD28-E26** | `ultimate_wilderness` | 2030 | 0 | **2030** | E8 | E13, E14, E24 |
| **SD28-E27** | `ultimate_combat` | 2182 | 0 | **2182** | E3 | E13, E14, E24 |
| **SD28-E28** | `ultimate_magic` | 2446 | 0 | **2446** | E4 | E13, E14 (hard), E24 |
| **SD28-E29** | `ultimate_psionics` | 2854 | 0 | **2854** | E9 | E13, E14, E24, licensing precheck (E2-F2) |

Sub-total gap, six Ultimate books: **12,392**. Plus `ultimate_campaign`'s 23
(E13) → **12,415**, matching the 13-book `not-started` aggregate exactly.

Notes:

- **E25 `ultimate_equipment`** and **E28 `ultimate_magic`** carry a *hard* E14
  dependency. Equipment- and spell-heavy books ingest straight into
  `ingested-magnitude`, which is not proven. Without the widened harness these
  two epics can do their entire content job correctly and still report a proven
  count near zero. Do not start them before E14 closes.
- **E29 `ultimate_psionics`** additionally requires the Dreamscarred Press
  licensing precheck recorded under `epic-2-prelaunch` (SD28-E2-F2). Largest
  Ultimate-book gap in the set.
- **E24 `ultimate_intrigue`** goes first among the Ultimate books and is the
  second calibration point: it is the first *full-size* book taken to 100%
  proven, and its measured cost is what E25–E29's estimates rest on. E13
  calibrates the per-unit variable cost on 23 units; E24 calibrates the
  per-book fixed cost at realistic scale. Both figures are needed before any
  Ultimate-book schedule is credible.
- Overlap with SD-30 stands (`decisions.md §5`): for Occultist, Spiritualist,
  Medium and Mesmerist, SD-28 references the SD-30 canonical id and does not
  redefine. Units so referenced still count toward this bundle's target when the
  observation is real.

## Epic 30 (SD28-E30) — Completion integrity gate

**Objective:** Prove that the 100%-proven claim was reached honestly, before it
is reported as reached.

**Derived from:** the anti-gaming rule above; `decisions.md §32`.

### Feature seeds

#### SD28-E30-F1 — Generator-integrity diff

Acceptance:

- The full diff of `src/bin/v06_work_inventory.rs` across this whole effort is
  reviewed against the anti-gaming rule, line by line. Every change is
  classified as *added observation* or *changed definition*. **Any changed
  definition is a finding**, not a footnote, and must carry explicit operator
  approval recorded in `decisions.md`.
- The `proven` formula (`grounded + text-complete`), the `text-complete`
  predicate (`magnitude_token_count == 0`), and every `status_vocabulary` string
  are compared verbatim against their state at this section's authoring
  snapshot (`2026-08-02T11:50:31Z`). Unchanged, or approved.

#### SD28-E30-F2 — Disposition audit

Acceptance:

- Every `deferred-with-reason` unit is checked to carry an engine diagnostic
  **verbatim**, with its `reason_id`, per the status vocabulary — not a
  re-narration. A deferral without a quotable engine diagnostic is a finding.
- Every `OPEN_FINDINGS` entry raised by E14/E15/E16 is still open, closed with
  evidence, or explicitly deferred with a named owner in
  `forward-scope-register.md`. None silently dropped.
- A random sample of units that moved into `text-complete` is re-checked
  against its own corpus record for magnitude tokens. Any that carries one is a
  gaming finding and the whole bucket is re-audited.

#### SD28-E30-F3 — The final number, with its command

Acceptance:

- `proven_units == 32061` across the 13 books, reported with the exact command
  that produced it and the generator's `generated_at`.
- Two independent implementations agree on the count, per `AGENTS.md`
  ("any number that moves a baseline needs two independent implementations
  agreeing").
- Any shortfall is reported as a shortfall with its unit count and cause. **A
  partial result honestly reported closes this epic; a complete result
  dishonestly reached does not.**

**Depends-on:** E13–E29 (all completion epics), and `epic-12-code-review`.

## Epic 31 (SD28-E31) — Spell magnitude → player surface

**Objective:** Wire the real, magnitude-bearing spellbook computation into a
surface the player actually sees, closing the "third, disconnected twin"
finding `epic-14-harness` recorded rather than papered over.

**Derived from:** `epic-14-harness`'s "F1 -- what actually happened" section
(`artifacts/e14-harness-widening.md`). `spellbook::compute_spellbook_coverage`
reads each resolved spell's real `level` and computes `spell_save_dc`/
`slots_total`/`slots_used` -- genuine magnitude-bearing output, wired into
`contract::PilotReceipt.spellbook` -- but `contract::build_pilot_receipt` was
called by nothing the desktop app runs (`grep -rn build_pilot_receipt
apps/desktop/src-tauri/src` returned 0 hits at that epic's close).
`pf1_adapter::resolve_unified_pilot_snapshot` -- the function the desktop app
actually gates its sheet on -- never called it either. Exactly the shape
`decisions.md §29.1`/`§29.2` already names: a real computation that never
reaches the surface the player's sheet is built from.

### Feature seeds

#### SD28-E31-F1 — Wire the magnitude to a player-visible surface

Acceptance:

- `PilotSnapshot` (`src/rules_core/pilot_view_model.rs`) gains a
  `spellbook: Option<PilotSpellbookViewModel>` field, projected via
  `PilotSpellbookViewModel::from_coverage(&SpellbookCoverage)` --
  `None` (not zeroed) for a non-caster or a build with no spell yet
  resolved against the corpus, matching the `damage_reduction`/`companion`
  "absent, not zeroed" convention already on that struct.
- `pf1_adapter::resolve_unified_pilot_snapshot` calls
  `compute_spellbook_coverage(character_input, corpus)` and populates the
  new field -- the desktop app's own gate function, not a parallel path.
- `character_hub.rs`'s `PilotSnapshotDto`/`map_snapshot_dto` carry the new
  field to the wire (`PilotSpellbookDto`, `skip_serializing_if` absent
  discipline).
- The desktop Spells tab (`CharacterSheet.tsx`'s `SpellsTab`) renders real
  spell save DC and slot total/used numbers from `snapshot.spellbook`,
  replacing the tab's prior "DCs ... are not computed" caption.
- **On-screen verification is the acceptance test**, per
  `no-stub-mvp-doctrine.md`: a value computed but not rendered is a stub.
  Screenshot a Wizard 1 holding a real resolved spell (Alarm, Abjuration,
  level 1) and confirm the save DC reads `10 + spell level + ability
  modifier` for that build.

#### SD28-E31-F2 — Real spell probe (earned only if F1 makes one possible)

Acceptance, only attempted once F1 lands and a spell's own level provably
moves a number on the player-visible snapshot:

- A `v06_work_inventory.rs` probe promotes `Kind::Spell` from
  `ingested-magnitude` to `grounded` only when the observed
  `slots_total`/`spell_save_dc` magnitude varies with that spell's own
  `level` -- not merely that the spell resolves (the exact defect that got
  `epic-14-harness`'s own spell probe reverted: it promoted 1,067 of 1,067
  by testing resolution, not magnitude).
- A negative test pins a present-but-non-mechanical spell as NOT promoted,
  with recorded evidence that the negative test fails when the probe is
  made permissive (`decisions.md §32`'s anti-gaming rule).
- If no discriminating probe is possible, this feature seed is not built;
  the 1,067 units stay `ingested-magnitude` with their existing
  `OPEN_FINDINGS` entry, and the epic closes on F1 alone.

**Depends-on:** `epic-14-harness`.

## Recommended sequencing — completion epics

```
E13 (calibration, 23 units)
  → E14 (harness widening)  ┐
  → E15 (unknown sweep)     ├ parallel, separate worktrees
                            ┘
  → E16 (not-ingested backfill; after E15)
  → E17 (pathfinder_unchained — first per-book completion, calibrates book cost)
  → E18, E19, E20, E21, E22, E23 (started books, file-disjoint, any order)
  → E24 (ultimate_intrigue — full-size-book calibration)
  → E25, E26, E27, E28, E29 (Ultimate books; E25/E28 hard-gated on E14)
  → E30 (completion integrity gate)
  → E10 (closure epilogue)
```

E14 and E15 both edit `v06_work_inventory.rs`. They may run concurrently only in
separate worktrees with separate `CARGO_TARGET_DIR`s (`decisions.md §29`,
`AGENTS.md` "Concurrency and Measurement"). Per-book epics are file-disjoint by
`src/rules_core/rules_tables/<book>/` and may fan out.

## Completion gate — 100% proven

This target is met when, and only when:

- All 13 books report `proven == units`: **32,061 of 32,061**.
- Epic 30 closed with zero unresolved gaming findings.
- Every `deferred-with-reason` unit carries a verbatim engine diagnostic.
- The number is reported with the command that produced it and the generator's
  `generated_at`, per `AGENTS.md`.
