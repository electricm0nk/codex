---
canonical: true
owner: god-emporer
bundle_id: SD-33
date: 2026-08-24
---

# SD-33 Technical Design

## 1. The shape of the problem

SD-32's instruments answer a **presence** question and are built accordingly: read the corpus, join a unit to a record, report whether the record is there. The join is the hard part; the verdict is trivial.

SD-33's instruments answer a **correctness** question, which inverts that. The join is already solved (`no_record == 0`). The hard part is the verdict, and it cannot be computed from our own data at all — it requires an external authority.

```
SD-32:   unit ──join──> corpus record          verdict = does it exist?
SD-33:   unit ──engine──> our value  ─┐
                                      ├──compare──> agree | disagree | unverifiable
         unit ──oracle──> their value ┘
```

**The `unverifiable` arm is not an error path.** It is the honest answer whenever the oracle cannot be reached for that unit, and it must be as visible in the output as the other two (`decisions.md §7`).

## 2. `THE-BOX.md` and `scripts/box_ledger.py`

### 2.1 Document

A living partition of **all 49,438 inventory units**. Frontmatter carries `derived_at: <sha>`; the body carries one section per group with a count, a disposition, and a re-derive command.

Groups are keyed by **verification disposition**, not by content kind — that is what makes it SD-33's box rather than SD-31's:

| Group | Meaning |
|---|---|
| `agrees` | computed and matches the oracle |
| `disagrees` | computed and does not match — **each is a named open defect** |
| `unverifiable_no_probe` | the kind has no probe; naming the kind is the disposition |
| `unverifiable_no_oracle_coverage` | probe exists, oracle has no answer for this unit |
| `nothing_to_compute` | F0 — measured empty; done under `SD-32-.../decisions.md §7` |
| `not_yet_run` | in scope, not yet through an engine |

### 2.2 Tool

`scripts/box_ledger.py --check` exits non-zero on any of the five conditions in `decisions.md §1`. The staleness gate (condition 5) is the one that keeps the document alive:

```python
# condition 5, in shape
derived = frontmatter["derived_at"]
if subprocess.run(["git", "merge-base", "--is-ancestor", derived, "HEAD"]).returncode != 0:
    fail(f"THE-BOX.md derived_at {derived} is not an ancestor of HEAD — re-derive before trusting it")
```

**Why this specific gate:** SD-31's THE-BOX was amended every wave because waves were told to. SD-33's is amended every wave because **the build fails if it is not**. That is the entire difference between the two bundles' relationship to their own goal (`decisions.md §4`).

The tool follows `scripts/coverage_ledger.py`'s **fail-closed-on-empty** posture: an empty or unreadable input is the "no coverage" case, never an implicit zero-unit pass.

## 3. The oracle harness

### 3.1 Path A — headless PCGen

```
authored .pcg  ──> PCGen (pinned 7f818006e371) ──> export template ──> machine-readable values
```

Entry point verified present in the pinned tree: `code/src/java/pcgen/system/BatchExporter.java`,
`boolean exportCharacter(String characterFilename, String outputFile)`. Supporting: `CommandLineArguments.java`, `Main.java`, `build.gradle`, `gradlew`.

The export template is the design's leverage point: rather than a character sheet, it emits **one row per computed variable**, so a single character exercises many units.

**Sparse-cone note:** the oracle slot's checkout cone is `data/pathfinder` + `system/gameModes/Pathfinder`, but **all 4,503 `.java` files are readable via `git show HEAD:<path>`** without widening it. A build requires widening; reading for Path B does not.

### 3.2 Path B — source-derived semantics

Read the operator implementation in PCGen's Java source and derive the expected semantics per shape. Proven: SD-32 read `plugin/jepcommands/MaxCommand.java` and found `numberOfParameters = -1` with a `first || param > result` fold, disproving a pinned test that asserted single-argument `max()` refuses. **Three real corpus records hit that divergence.**

Path B is per-shape rather than per-unit, so its throughput profile is completely different — which is why AT-33-E2-004 escalates the choice rather than absorbing it.

### 3.3 Comparison contract

```
compare(unit) -> { ours, oracle, verdict: agree | disagree | unverifiable, reason }
```

`reason` is required whenever `verdict != agree`. A harness exception maps to `unverifiable` with the exception as its reason — **never to `agree`**, and never to a silent skip.

## 4. Engine-coverage closure (Epic 3)

The corpus-wide runner already exists: `src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs`, emitting `formula_interpreter.corpus-wide.json` with per-family `population` / `recognised_units` / `refused_units` / `unjoined_units`.

**The 41% is a symptom with an unknown cause**, and the cause is uneven by family (F1 28%, F8 21%, F2 64%). AT-33-E3-001 requires the diagnosis first. The likely shapes — none confirmed — are a population filter in the runner, a join that silently drops units, or a family-assignment difference between the runner and `shape_ledger.py`. **Whichever it is, it is a generic mechanism, not 6,854 individual problems**, and the pass is written accordingly.

## 5. Interaction with existing instruments

| Instrument | SD-33's relationship |
|---|---|
| `scripts/shape_ledger.py` | consumed unchanged; supplies family + join status |
| `scripts/coverage_ledger.py` | **not** the box. Its `not_done_population()` drops 15,041 units by design; `box_ledger.py` covers the full inventory (`technical-requirements.md` R1) |
| `scripts/verify.sh` | gains the `denominator-gate` stage (AT-33-E1-004) |
| `src/bin/v06_work_inventory.rs` | Epic 4's territory — `doneness_verdict()`'s handling of `unknown` |
| `formula_interpreter_corpus_wide.rs` | Epic 3's territory |

**`coverage_ledger.py` is deliberately left alone.** Its population is correct *for the question it answers*. SD-33 adds an instrument rather than widening a proven one, so that SD-32's closure figures remain re-derivable exactly as they were reported.
