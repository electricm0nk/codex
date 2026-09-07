# TOKEN-MODEL — token-vocabulary cost model for the 22,369-unit remainder

Author: Fable read-only review lane. Date: 2026-08-31. Repo HEAD at analysis: `920cb53307`;
`docs/work-inventory.json` as regenerated 2026-09-01T00:01:15Z (committed 2026-08-31 20:15 -0400,
ancestor of HEAD). Machine-readable companion: `TOKEN-MODEL.json` (same directory).

Provenance tags on every figure: **MEASURED** (derived live from a repo artifact by this lane's
own scripts), **EXTRAPOLATED** (a measured rate applied to an unmeasured population),
**ASSUMED** (no measurement backs it). Every figure states its denominator.

**Method.** Non-DONE units were partitioned with the live `scripts/completion_atlas.py::_bucket_of`
(imported, not re-implemented) over the live `docs/work-inventory.json`, then joined to their
`data/corpus/<book>/<kind>/*.json` records by `(book, source-file basename, source line)` with a
`record_key` fallback (corpus dir `beastiary` aliased to inventory book `bestiary` — the dir is
misspelled, the inventory is not). Token types are the `key` field of each record's
`data.raw_tokens` entry (the first field/prefix of each raw PCGen token). Join scripts are in this
session's scratchpad; every number below is re-derivable from the two inputs named above.

**Populations (all MEASURED):**

| Population | Count | Denominator |
|---|---:|---|
| Non-DONE units, 35 non-vehicle books (excl. `core_rulebook`, `ultimate_campaign`) | **22,369** | of 42,472 units in those books |
| — joined to a corpus record with `raw_tokens` | 22,366 (99.99%) | of 22,369 |
| — unjoined (no corpus record exists: 3 spells) | 3 | of 22,369 |
| — joined but `raw_tokens` empty | 921 (4.1%) | of 22,366 |
| Non-DONE, all 37 books (context only) | 24,475 | of 49,438 |
| Bucket split of the 22,366 joined | B 11,296 · C 3,981 · M 3,635 · D 2,587 · A 449 · V 175 · U 171 · X 53 · Z 19 | of 22,366 |

(The 3 unjoined units are bucket-B spells; B's true 35-book count is 11,299, matching B-SYNTH.)

---

## 1. The token vocabulary of the remainder

**189 distinct top-level token types** carry the entire 22,366-unit remainder, over **181,274
token instances** (~8.1 tokens/unit). MEASURED, denominator = every raw token on every joined
non-DONE unit.

This is the operator's central claim, and it measures out: the data surface of 22,369 units of
"work" is 189 strings, and it is savagely head-heavy.

### 1.1 Coverage curve (MEASURED)

"Unit coverage at k" = units whose *every* token type is within the top-k types (zero-token units
count as covered from k=0) — the strictest reading, i.e. the number of token types you must
handle before whole units are fully readable.

| Coverage target | Token types needed (of 189), by **instances** | By **whole units** |
|---:|---:|---:|
| 50% | 6 | 21 |
| 80% | 15 | 49 |
| 90% | 28 | 68 |
| 95% | 44 | 87 |
| 99% | 84 | 128 |

Per bucket (unit-coverage thresholds, MEASURED; denominators B 11,296 / C 3,981 / D 2,587 / M 3,635):

| Bucket | distinct types | 50% | 80% | 90% | 95% | 99% |
|---|---:|---:|---:|---:|---:|---:|
| B | 134 | 18 | 35 | 44 | 59 | 88 |
| C | 79 | 13 | 25 | 36 | 44 | 59 |
| D | 99 | 17 | 31 | 39 | 53 | 77 |
| M | 130 | 16 | 48 | 59 | 73 | 105 |

### 1.2 Not all tokens are compute — the vocabulary is smaller than it looks

Classing every type by what handling it requires (MEASURED instance shares, denominator 181,274):

| Class | Instances | Share | Types | What it needs |
|---|---:|---:|---:|---|
| metadata/identity (KEY, TYPE, CATEGORY, SOURCEPAGE, FACT, VISIBLE, …) | 78,987 | 43.6% | 22 | Nothing — already ingested; the kind-table loader consumes these |
| bonus/var (BONUS, DEFINE, TEMPBONUS, MULT, STACK, …) | 34,780 | 19.2% | 6 | The interpreter (envelope + formula + stacking) |
| grant/structural (ABILITY, AUTO, CSKILL, CLASSES, NATURALATTACKS, …) | 18,641 | 10.3% | 24 | Per-token grant consumers |
| display (DESC, BENEFIT, ASPECT, SAB, …) | 18,602 | 10.3% | 5 | Text serve — the standing "text-only features are complete" ruling |
| prereq (PRE*/!PRE*) | 14,734 | 8.1% | 69 | The PRE evaluator |
| spell-data (CASTTIME, RANGE, COMPS, SAVEINFO, …) | 6,016 | 3.3% | 12 | Static display fields |
| spell-grant (SPELLKNOWN, SPELLS, SPELLLEVEL, …) | 3,543 | 2.0% | 4 | Spell-list/known wiring |
| equip-data (COST, WT, DAMAGE, CRITRANGE, …) | 2,157 | 1.2% | 11 | Static item fields (equipment chassis exists) |
| choice (CHOOSE, ADD) | 1,643 | 0.9% | 3 | Choice machinery (§4) |
| other (long tail) | 2,171 | 1.2% | 33 | Case-by-case |

**Only ~139 types (75,512 instances, 41.7%) are compute-bearing at all**, and that sub-vocabulary
is even steeper: **top 3 compute types = 50% of compute instances, top 21 = 90%, top 34 = 95%,
top 65 = 99%** (MEASURED). 58.3% of all token instances are metadata/display/static fields the
ingest and kind tables already consume.

**3,899 units (17.4% of 22,366) carry no compute-bearing token at all** — their entire record is
metadata + description + static fields. Their remaining work is table membership plus text serve,
the exact shape the Epic 2 loader cleared at 8,014 units for 0.59h. MEASURED.

Top of the vocabulary (full 189-row table in `TOKEN-MODEL.json`):

| Token type | Instances | Units carrying | Class | rules_core files mentioning |
|---|---:|---:|---|---:|
| BONUS | 21,810 | 8,660 | bonus/var | 131 |
| TYPE | 17,910 | 17,515 | metadata | 147 |
| CATEGORY | 16,777 | 16,777 | metadata | 80 |
| DESC | 14,985 | 11,815 | display | 138 |
| KEY | 14,854 | 14,854 | metadata | 139 |
| ABILITY | 12,484 | 5,728 | grant | 111 |
| SOURCEPAGE | 12,225 | 12,192 | metadata | 31 |
| DEFINE | 9,459 | 4,236 | bonus/var | 35 |
| FACT | 8,635 | 1,630 | metadata | 32 |
| VISIBLE | 4,312 | 4,290 | metadata | 31 |
| PREMULT | 3,025 | 2,398 | prereq | 38 |
| ASPECT | 2,996 | 1,872 | display | 10 |
| PREVARGTEQ | 2,823 | 2,593 | prereq | 52 |
| PREABILITY | 2,467 | 2,364 | prereq | 59 |
| COST | 1,995 | 1,981 | equip-data | 40 |
| SPELLKNOWN | 1,956 | 1,001 | spell-grant | 7 |
| CLASSES | 1,621 | 1,238 | grant | 58 |
| MULT | 1,582 | 1,581 | bonus/var | 50 |
| CHOOSE | 1,533 | 1,532 | choice | 11 |
| !PREABILITY | 1,507 | 1,470 | prereq | 26 |
| STACK | 1,313 | 1,313 | bonus/var | 10 |
| PRECLASS | 1,204 | 1,186 | prereq | 36 |
| SPELLS | 1,107 | 609 | spell-grant | 36 |
| FACTSET | 1,055 | 463 | metadata | 2 |
| NATURALATTACKS | 1,033 | 978 | grant | 18 |

BONUS subdivides by its first value segment (application target). Collapsing `WEAPONPROF=<name>`
shapes, the remainder holds **35 distinct BONUS sub-targets; 9 cover 95% of 21,810 BONUS
instances** (MEASURED): VAR 15,396 · ABILITYPOOL 1,480 · SKILL 965 · STAT 721 · SPELLCAST 598 ·
WEAPONPROF 439 · SAVE 395 · COMBAT 367 · SPELLKNOWN 360, then SITUATION 254, CASTERLEVEL 181,
a PI-redacted target 97, and a tail of 23 more under 100 each.

---

## 2. What the engine already interprets

Two coverage readings, one generous and one strict — both stated so neither can masquerade as
the other.

**Generous (string-mention grep, MEASURED):** 151 of 189 token types appear somewhere in
`src/rules_core/**` — 98.1% of instances, and 94.2% of units have every one of their token types
mentioned somewhere. This is an upper bound only (a doc-comment mention counts); do not price
off it.

**Strict (named engines, verified by reading the modules):**

| Engine | File(s) | What it genuinely interprets | Remainder exposure |
|---|---|---|---|
| Formula interpreter (F1–F9) | `src/rules_core/pilot_compute/formula_interpreter.rs` (1,615 lines) + `formula_interpreter_corpus_wide.rs` | The arithmetic formula grammar inside `BONUS:VAR`/`DEFINE` values, semantics traced to the pinned PCGen oracle (JEP chain) | `BONUS:VAR` = 15,396 instances on **6,271 units (28.0%)**; DEFINE on 4,236 units (18.9%); either: **6,708 units (30.0%)** MEASURED |
| Bonus stack reader | `pilot_compute/bonus_stack_reader.rs` (794 lines) | Multi-`BONUS:VAR` summing + trailing `PREVARGTEQ` gates on one record | same population |
| PRE evaluator | `feat_prereqs/pre_tokens.rs` (1,334 lines) + siblings | **42 PRE families** (48 of the remainder's 69 prereq types counting `!`-negations) | **97.3% of the remainder's 14,734 prereq instances** MEASURED; the 21 unhandled types total 403 instances (2.7%) |
| Generic kind-table loader | `rules_tables/simple_kind_tables.rs` (208 shared lines) | Record placement/membership + description serve for 7 kinds — i.e. the entire metadata+display class (53.9% of instances) | consumed already; marginal cost per kind measured at ~7 lines |
| Equipment effects | `equipment_effects/*` + `equipment_effects.rs` | EQMOD, item stat tokens, weapon/armor fields | equip-data class (1.2%) + EQMOD (189 instances) |
| Spell chassis | `spell_resolver.rs`, `spellbook/`, per-book spell-list tables | SPELLLEVEL/CLASSES list membership (partially); spell static fields ingested | spell-grant class partially |

**What is genuinely NOT interpreted anywhere (MEASURED, strict):**

- **Non-VAR BONUS application semantics as a generic mechanism.** `BONUS:SKILL/STAT/COMBAT/SAVE/…`
  appear in many kind-specific consumers (trait_effects, feat_effects, race_creation, equipment)
  but there is no generic envelope interpreter that applies an arbitrary record's non-VAR BONUS.
  ~6,414 non-VAR BONUS instances; `BONUS:SPELLCAST` (598 instances, 72 units) has **zero mentions**
  in `src/rules_core`.
- **Spell static display fields**: CASTTIME/COMPS/SAVEINFO/SPELLRES — 676/648/631/631 instances,
  zero `rules_core` mentions (ingested on the record, never surfaced by a table).
- 21 tail PRE families (403 instances), and a ~30-type long tail (SUBRACE 146, LANGBONUS 97,
  ITYPE 97, DEFINESTAT 70, UMULT 59, … each under 150 instances).

**Unit-weighted bottom line:** the single biggest already-built engine (the F1–F9 interpreter +
stack reader) reaches 30.0% of remainder units; the placement/metadata machinery reaches 100% of
the 17.4% no-compute-token units; the PRE evaluator covers 97.3% of all prerequisite gating. The
uncovered compute surface is ~10–15 BONUS sub-targets, ~20 grant/structural consumers, and a
sub-3% tail — a **vocabulary gap measured in dozens of token types, not thousands of units**.

---

## 3. What the slow hours actually bought (bucket-B receipts, epic 3)

Source: `artifacts/epic-3-core-rulebook/step-cost-ledger.json` (29 B cycles, 2,432.3 wall-min,
235 units to DONE = 5.8/h blended, MEASURED) and three receipts read in full:

- **`AT-34-E3-001_class_feature_owner_matched_cycle_receipt.md`** — one cycle, **0 units closed**.
  The entire cycle bought: re-deriving the 346-unit population, a hand census of 7 sub-causes,
  one regression test pinning the census, and the receipt. 100% analysis/process, 0% closure.
- **`AT-34-E3-001_class_feature_owner_matched_cycle_receipt_6.md`** — 253.6 min, **3 units closed**
  (0.71/h): one hand-written Rust grounding function + 2 class-ID consts + **9 hand-written tests**
  + guarded regen + receipts.
- **`AT-34-E3-001_class_feature_option_pool_with_magnitude_cycle_receipt_8.md`** — 617.4 min,
  **2 units closed** (0.19/h). Itemized from its own text:

| Line item | Evidence in receipt | Interpreter eliminates? |
|---|---|---|
| Read predecessor receipt in full + re-derive population | required by decisions.md §12 | Amortizes (per pass, not per 2 units) |
| Read corpus raw_tokens by hand to learn the formula | "read the corpus record's raw tokens BEFORE writing the gate" | **YES — this is literally the interpreter's job** |
| Hand-write Rust: 1 const + 1 gate fn + 1 computation block + 2 `ComputationExplanation` pushes for 2 formulas | `pilot_compute/mod.rs` diff | **YES — the formulas (`3+INT`, `(LVL-8)/2+1`) are F-family shapes the interpreter already parses** |
| Hand-extend the probe (`probe_wizard_arcane_school_wiring`, 5th variant) | `v06_work_inventory.rs` diff | **YES — replaced by a generic compute pass** |
| 3 hand-written tests per feature pair | RED→GREEN section | **YES — replaced by oracle-generated fixture rows** (Decision 20's gate still satisfied) |
| Citation-shift maintenance in `completion_atlas.py` + `missing_engine_tables.py` (12 file:line re-derivations) | Files-touched list | Remains, but per pass not per cycle |
| Guarded regen + sweep/fixture-check env-var dance | "2–3 minutes wall" + stamp-loss guard discovery | Remains — **minutes**, and once per batch |
| Full test suites (lib 2,910 + bin 408 + workspace no-run) | Figures table | Remains per commit; amortizes with batch size |
| Receipt + progress.md + kanban.md + retro events | Files-touched list | Remains per cycle; amortizes over units/cycle |

**Answer:** the variable cost that made these cycles 0.19–0.71 units/h is per-feature
hand-modeling — reading tokens by eye, transcribing their arithmetic into bespoke Rust, and
hand-writing per-feature probes and tests. Every one of those line items is exactly what a token
interpreter + generic compute pass + oracle-fixture generation replaces. What survives is the
fixed per-cycle process (regen, sweeps, suites, receipts — tens of minutes) and the judgment
calls (choice shapes, display-safety gates), which amortize over batch size instead of being paid
per 2–3 units. The receipts themselves say so: cycle 1's own conclusion is that the remaining 346
units need "real engine wiring… comparable in size to a single Epic 2 simple-kind table," i.e.
mechanism-shaped work, not 346 hand units.

---

## 4. The CHOOSE problem, quantified

Denominator 22,366 joined non-DONE units, 35 books. MEASURED.

| Measure | Units | Share |
|---|---:|---:|
| Any `CHOOSE:` token at all | 1,532 | 6.8% |
| — of which `CHOOSE:NOCHOICE` (a no-op marker, **not** a player choice) | ~1,055 tokens | — |
| **Genuinely choice-bearing** (`CHOOSE:` ≠ NOCHOICE, or `ADD:`, or `BONUS:ABILITYPOOL`) | **1,669** | **7.5%** |
| — by bucket | B 565 · C 447 · M 433 · D 195 · X 19 · V 3 · U 6 · A 1 | |
| Token *values* referencing `%CHOICE`/`%LIST` (choice-dependent magnitudes) | 384 | 1.7% |

Choice sub-shapes: BONUS:ABILITYPOOL dominates (1,091 units); real `CHOOSE:` selectors are a long
tail (NUMCHOICES 83+31+16+12+12, ABILITYSELECTION 79, WEAPONPROFICIENCY 68, SKILL 67, SPELLS 64,
CLASS 58, …).

**The historical "CHOOSE forces hand-modeling" problem is a 7.5% problem, not a remainder-wide
problem.** 92.5% of the remainder carries no player choice at all. And the repo already holds a
ruled precedent for the 7.5%: v0.6's Path-A canonical-default seeding (Sorcerer/Cleric/Druid
reached Computed via canonical defaults, not a real picker) — a policy decision, not new
research.

---

## 5. Revised cost model

**Calibrators (all MEASURED):**

| Calibrator | Figure | Source |
|---|---|---|
| Formula interpreter core build | ~8.9h (6.5h initial, 2 cycles + 2.4h generalization, 4 cycles) for the whole BONUS:VAR/DEFINE arithmetic grammar; then ~2 days of widening cycles for family coverage | `docs/retro/sd32-…-retrospective.md`, B-SYNTH §2.1 |
| Interpreter recognition | 4,696 of 4,798 units run = 97.9% recognized (grammar-reach, no fabricated vars); the 4,798 was 41% of the then-11,652 formula-bearing population | SD-32 retro Findings 2 |
| Kind-table loader | 0.59h (2,138s commit-to-commit) for 7 kinds on 208 shared lines, ~7 marginal lines/kind; swept bucket A 8,463 → 449 corpus-wide | `artifacts/epic-2-tables/table-build-rate.json` |
| Oracle harness | Already built (sunk); 8,330 units rowed, 0 disagree; ~10 dispatch waves ≈ 2 days to run+triage at that scale | SD-33 kanban rows 17–19 |
| Guarded inventory regen | 2–3 min wall per full corpus pass | receipt `…option_pool_with_magnitude_cycle_receipt_8.md` |
| Hand-modeling rates | 0.19–14.64 units/h; blended 5.8 to DONE (B, core_rulebook, 29 cycles) | `step-cost-ledger.json` |

**Cost components, interpreter route** (vocabulary-priced — the population never appears as a
multiplier except in verification):

| # | Component | Size driver | Bracket | Provenance |
|---|---|---|---:|---|
| 1 | Non-VAR BONUS application semantics: ~9 sub-targets to 95% of BONUS instances, ~15 to 99% | 9–15 sub-targets × 1–4h (bracketed between the 0.08h/kind loader marginal and the 8.9h full-grammar build; each sub-target is an application rule, not a grammar) | **15–60h** | EXTRAPOLATED |
| 2 | PRE tail: 21 unhandled families, 403 instances | 21 × 0.25–1h (pre_tokens.rs pattern is established) | **5–20h** | EXTRAPOLATED |
| 3 | Grant/structural + spell-grant consumers to 99% of compute instances (~20 types: ABILITY, AUTO, CSKILL, CLASSES, SPELLS, SPELLKNOWN, SPELLLEVEL, DOMAINS, TEMPBONUS, NATURALATTACKS, MOVE, VISION, DR, SR, …; several partially exist) | ~20 types × 1–4h | **25–80h** | EXTRAPOLATED / partly ASSUMED (which partials count as done is a judgment) |
| 4 | Choice machinery: generalize Path-A canonical-default seeding to the 1,669 choice units | one mechanism | **20–60h** | ASSUMED (bounded by the v0.6 precedent; skippable — defers 7.5% of units) |
| 5 | One corpus-wide compute pass (run interpreter over all records, emit generic explanations, regen) | regen is minutes; iterations + breakage triage dominate | **10–30h** | EXTRAPOLATED |
| 6 | Oracle verification + fixture generation over the interpreter-reachable set (~14.8k units; SD-33 did 8,330 in ~2 days of waves) | scale ×1.8 on a built harness + disagreement triage | **30–100h** | EXTRAPOLATED |
| | **Engine-route total (1+2+3+5+6)** | | **85–290h** | EXTRAPOLATED |
| | **With choice machinery (＋4)** | | **105–350h** | EXTRAPOLATED/ASSUMED |

**What this reaches (MEASURED shape, denominator 22,366):** the clean interpreter-reachable set —
no choice token, no PI-redacted value, raw_tokens present, buckets A/B/D/M — is **14,817 units
(66.2%)**: B 9,799 · M 2,343 · D 2,227 · A 448. Of these, 2,310 carry no compute token at all
(pure placement/display). Components 4 and a generic-explanation surface would extend reach into
most of the rest.

**What does NOT collapse** (union of the named sets = **7,549 units, 33.8% of 22,366**, MEASURED;
per-cause counts overlap):

| Non-collapsing set | Units | Why | Honest treatment |
|---|---:|---|---|
| Bucket C (`no_explanation_id_and_no_diagnostic_names_this_feature`) | 3,981 (17.8%) | Surfacing, not computing: 39 hand-written `explanation_id` call sites today. An interpreter-emitted generic explanation record could attack this, but that is UNPROVEN — and its only measured rate (41.3 units/h) is **to V, 0.0 to DONE** | Own line item: 100–400h EXTRAPOLATED-ANALOG, or a generic-explanation spike first |
| Choice-bearing | 1,669 (7.5%) | Needs canonical-default policy or a real picker | Component 4, or deferred |
| PI-redacted token values | 1,333 (6.0%) | The corpus copy carries `[redacted PI]` in the very values an interpreter would read; per-record PI handling constrains automation | Partial: non-redacted tokens on the same record still interpret; residue is per-record work |
| Zero-token / unjoined | 924 (4.1%) | Nothing to interpret — needs ingest or reclassification | Bucket-mechanism work (mostly bestiary templates/monster-class synthesized units) |
| V/U/X/Z residue | 418 (1.9%) | Probes, instrument fixes, capability builds, bootstrap | B-SYNTH card-1 spike rows |
| Also not collapsing (not unit-priced): per-cycle process (regen/sweeps/suites/receipts), Decision-7 display-safety judgment, dashboard/UI surfacing, operator rulings on reclassification questions | — | fixed cost per pass | folded into components 5–6 brackets |

**The comparison that decides the question:**

- Hand-modeling the remainder at the measured blended 5.8 units/h → **3,857h**; at the measured
  slow-class 0.71–1.5 units/h that dominates the remainder's composition (B-SYNTH §1.2: 67% of
  bucket B is the slow class) → **~15,000–31,000h**. EXTRAPOLATED from MEASURED rates.
- Interpreter route: **~105–350h of engine work** reaching ~66% of units directly, plus the
  non-collapsing 7,549-unit residue, of which the dominant block (bucket C) needs either its own
  ~100–400h analog-priced lane or one unproven generic-explanation mechanism.

Even taking the worst end of every engine bracket and the best end of the hand bracket, the
interpreter route is **~5–10× cheaper**; against the realistic slow-class composition it is
**~30–80× cheaper**. No plausible error in components 1–6 closes a gap of that size.

**Honest bracket for the remainder under the interpreter model:
~250–800h total** — 105–350h engine build+verify (EXTRAPOLATED), plus 100–400h for bucket C
(EXTRAPOLATED-ANALOG, the weakest number here), plus a residual hand/judgment lane for the
~3,000-unit true residue (zero-token, PI, X/U/Z/V, choice-if-deferred) at mechanism-level rates
(ASSUMED 50–150h). Against the current method's 3,900–31,000h. The single biggest unknown is not
a token: it is whether bucket C's surfacing can be generated rather than hand-wired, and that is
a ~1-cycle spike to answer.

---

## 6. Why hasn't this been done? The rulings, quoted

The constraint was real, operator-pinned, and **is already overturned** — the current
hand-modeling method is running on a dead ruling's inertia, which the repo's own retrospective
already named.

1. **The ban** — `docs/release/SD-27-future-state-book-content-ingestion/decisions.md` §24.1
   (operator ruling, 2026-07-31):
   > "**No formula interpreter. Each feature is a hand-written, corpus-verified pure function.** …
   > an interpreter is the highest-risk option for *silently* wrong answers… A hand-modelled
   > formula that is wrong is a failing test; a misinterpreted token is a plausible number nobody
   > checks. The cost — linear growth with content — is accepted deliberately in exchange for
   > that property."

2. **The overturn** — `docs/release/SD-31-corpus-closure-grind/decisions.md` Decision 20
   (operator ruling, 2026-08-21, folded from `OPERATOR-RULINGS-2026-08-21.md` §20), verbatim:
   > "I choose thousands. if we need to revisit this, we can do it in the future. for now we need
   > to get something in front of the user community."
   With the recorded condition: "**every interpreted value banked by a consumer must clear
   `derived_evaluator_fixture_check`. An interpreted value with no fixture is not done.** §24.1's
   real concern is answered by that gate, not abandoned." The correctness bar (reproduce ≥166
   hand-modelled functions via `formula_reproduction_harness.rs`) was set and the module built
   against it.

3. **The retrospective's own verdict** — `docs/retro/sd31-retrospective.md`:
   > "the no-formula-interpreter ruling **sat unexamined for ~18 waves after its own stated
   > precondition for revisiting it had been satisfied.** The fixture mechanism it was waiting
   > for landed in wave 13. Nobody re-read the ruling. Eighteen waves of hand-writing arithmetic
   > that was sitting in the corpus the whole time."

**Does anything still bind?** Yes, two things — neither is a ban:
(a) Decision 20's fixture gate: interpreted values must clear `derived_evaluator_fixture_check`
(so component 6's fixture generation is mandatory, not optional); (b) Decision 20 was worded
"FOR THIS PACKAGE" (SD-31) but recorded in SD-31's decisions.md "because it governs SD-31
packages' construction choices going forward," and SD-32 built `formula_interpreter` +
`bonus_stack_reader` under it with operator-accepted closure. No later ruling re-narrows it. A
one-line SD-35 decision entry re-affirming the authorization at bundle scope would remove any
ambiguity; nothing in the record blocks it.

---

## 7. Verdict

**The operator's position holds, and it is the repo's own already-ruled position.** All of its
load-bearing claims measure out against the live corpus:

1. **The data is machine-readable**: 99.99% of the 22,369-unit remainder joins to a corpus record
   with preserved `raw_tokens` (MEASURED).
2. **Cost scales with vocabulary, not units**: 189 top-level token types carry everything; ~139
   are compute-bearing; ~34 compute types cover 95% of compute instances; the BONUS family needs
   ~9–15 application sub-targets; the PRE family is already 97.3% instance-covered (all MEASURED).
3. **The hard part is already built**: the arithmetic-formula grammar (the one genuinely
   grammar-shaped token family) exists, oracle-traced, at 97.9% recognition of everything it has
   been run over; the placement machinery cleared 8,014 units for 0.59h; the verification harness
   exists at 8,330 rows / 0 disagreements (all MEASURED).
4. **The slow hand cycles were paying interpreter-shaped costs by hand** — reading tokens by eye
   and transcribing F-family arithmetic into bespoke Rust at 0.19–0.71 units/h (MEASURED, §3).
5. **The historical blocker is overturned**, on the record, by the operator, with the safety
   concern converted into a fixture gate the pipeline already runs (§6).

**Honest new bracket for the 22,369-unit remainder: ~250–800h** (≈85–350h engine build + compute
+ oracle-verify reaching ~66% of units directly; ~100–400h for the bucket-C surfacing question,
the weakest-provenance number here; ~50–150h mechanism-level residue) — versus **3,900–31,000h**
under the current hand-modeling method. The brackets do not overlap under any reading of the
data. What the interpreter does NOT make free: bucket C's DONE-ness (17.8%), player-choice
policy (7.5%), PI-redacted records (6.0%), token-less units (4.1%), and the per-pass
process/verification overhead — 7,549 units (33.8%) in union, which is where a per-mechanism
plan (not per-unit, not per-book) still has to do real work.

**Recommended first probe (cheap, decisive):** run the existing `formula_interpreter` +
`bonus_stack_reader` over the remainder's 6,708 BONUS:VAR/DEFINE-bearing units (30.0%, MEASURED)
with oracle fixtures per Decision 20 — every engine in that sentence already exists; the only new
code is the pass itself. Its measured yield-per-hour settles the §5 brackets with data instead of
extrapolation, exactly the shape of B-SYNTH's card-1 spike.
