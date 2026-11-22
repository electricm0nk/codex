---
canonical: true
wave: 31
purpose: >
  The operator asked two direct questions and is waiting on the answers to decide how the program
  proceeds. This document answers them, with numbers, at the top. Everything below is the evidence
  and the reconciliation. Board FROZEN this wave: 13,458/38,372 (35.07%), docs/work-inventory.json
  byte-identical throughout (confirmed below). Nothing banked, nothing reclassified, no regen run.
provenance: >
  Six measurement lanes plus two independent adversarial reviewers. Every number below is the
  REVIEWED, CORRECTED figure where a reviewer corrected one — not the lane's original claim. Where
  a reviewer returned GAMED on a specific claim, that claim is not carried forward, and this
  document says so explicitly rather than silently dropping it.
---

# MEASURE TWICE — the wave-31 root-cause census and compute-library costing

## Answers, first

### (a) What are the X, Y and Z? What is the plumbing-vs-rules split?

**Twelve recurring blocker shapes, each with a corpus-wide count, cluster into two families: our own
plumbing (a dispatch that never existed, a matcher that only accepts an exact string, a gate reading
the wrong field, a generator that silently erases its own output) and genuine rules complexity (an
engine mechanism that was never built at all). The split favors plumbing, but by a smaller margin
than first measured.**

Corrected ratio: **~3.3:1 to 4.4:1 plumbing-favoring**, not the originally-filed 4:1–5:1. The
correction matters more than the headline: the largest "genuine complexity" bucket (2,453 units,
classes outside the modelled roster) overlaps the largest "plumbing" bucket (8,243 units, a
class-name field read from the wrong place) by a **measured** 1,354–2,124 units, not the ~532 the
original number assumed. Those overlap units are exactly the classes — Psychic, Vigilante, Medium,
Magus, Shifter, Kineticist, Spiritualist, Occultist — that genuinely need to be modelled, not
relabeled. **The operator's suspicion survives the correction. The precision of "4:1 to 5:1" does
not.** See §1 for the full twelve-shape table and §2 for the overlap arithmetic.

**A thirteenth, previously-undiscovered instance of the Monk shape was found this wave**, live, in
equipment: Advanced Player's Guide's equipment-category table omits the `Equipmods` variant that
seven other book table directories carry, while APG's own corpus holds 35 equipmods records — the
identical "table exists, one row/variant is missing" pattern the Monk case named, just in a kind
nobody had checked. It is mostly mitigated already by a separate gap-filling table, so no units are
silently lost today, but it disproves this wave's own claim (later retracted, see §4) that
equipment/spell/feat cannot host this shape at all.

### (b) Is a compute library worth building?

**Confirmed in shape, overstated in reach.** Ten real semantic families exist (not forty, not
one) — every corpus BONUS/DEFINE formula reduces to one of ten shapes, and that reduction survives
independent re-derivation exactly. The grammar to evaluate nine of the ten already exists
(`formula_interpreter.rs`, authorized by Ruling §20) and a wave-26 accumulator module
(`bonus_stack_reader.rs`, 329 lines) already proves the "read the producers of a named variable and
sum them" binding pattern the tenth family needs, generalizable to 77.2% of the corpus's distinct
custom identifiers — a materially better answer than either lane's own framing of the binding layer
(one lane overclaimed a narrow mechanism's reach at 46.8%, undercounting how much a broader,
already-proven mechanism actually covers at 77.2%; the correction runs in both directions, not one).

But the ceiling on what a compute library can close is **3,201 of 24,914 not-done units (12.8%)**,
not the 4,948 (19.9%) first filed — the single largest family (flat literal constants, 1,747 units)
gets **zero** benefit from any shared function, by the finding lane's own admission, because each
consumer still needs individual wiring regardless of how the constant is read. The other 87.2% of
the not-done population (ingestion, chassis-building, consumer-wiring) is untouched by a compute
library no matter how good it is.

**The operator's flagship example — "1d6 per level" — has zero instances in the population any lane
measured.** Dice notation does not appear anywhere in the 2,671 distinct BONUS/DEFINE formulas. It
lives in a different corpus surface (DESC/NATURALATTACKS/DAMAGE/SPROP/ASPECT/ALTDAMAGE tokens, 4,911
raw hits), sized this wave for the first time but never clustered or joined to units. That is real,
uncosted, separate work — filed as `sweeps.md` S17, not silently folded into the formula census.

**Verdict: build the binding-layer generalization (a data-driven version of the already-proven
producer-accumulator), and the two cheapest concrete lifts (a TYPE-facet triage tool, a
self-erasure lint) — but do not expect a compute library to move the board by more than roughly an
eighth of the remaining work.** See §3 for the full family table and §5 for the ranked build list.

**One number from the wave's own dispatch brief is withdrawn.** "33,830 formula tokens reduce to
1,049 shapes, the top 15 covering 80%" — the first two figures (33,830; 14,752 distinct raw values)
reproduce exactly. The third does not reproduce under any normalisation two independent lanes tried.
Treat "1,049 / top-15 / 80%" as **retracted**, not as a planning input for any future wave — and note
it was already written into the SD-32 package's own scope README as settled fact before this wave's
measurement returned (§6).

---

## 1. The twelve shapes, corpus-wide, corrected

| # | Shape | Corpus-wide count | Plumbing or rules? | Todo |
|---|---|---:|---|---|
| T1 | **Dispatch gap** (Monk shape) — complete table, missing string→id link | Classes: **0 of 157**, CLOSED/exhausted (`sweeps.md` S1). Race/monster: **partial only** — 0 orphaned variants among the 46-of-326 hand-modelled monster roster and the 7-of-38 CRB race roster; the other 280 chassis-served monster entries and 31 non-CRB races were never checked for this shape. Equipment: **1 live instance found this wave** (APG `Equipmods` variant, ~35 records, mostly mitigated by a separate gap table). | Plumbing | `sweeps.md` S1 (CLOSED), S2 (PARTIAL — corrected, see §4) |
| T2 | **Matcher too narrow / wrong field trusted for identity** (largest plumbing cluster) | S3 race-trait compound key: **2,472** (exact, reproduced). S8 `data.class` misattribution: **8,243 of 11,502 (71.7%)**, corrected from 8,210/71.4% — anchor and numerator both independently re-derived this wave. **New this wave: only 2,360 of that 8,243 are cleanly prefix-remappable (true plumbing); the remaining 5,883 are a MIX of category-label plumbing and genuine unmodelled-class content** (Medium 92, Psychic 85, Magus 70, Vigilante 67, Shifter 50, Kineticist 45, Spiritualist 44 among them) — assigning the whole 8,243 to "plumbing" is not supported. S12 evidence-string dishonesty: **1,321 of 12,114** (not the earlier 1,377 — a 5-code table sums to 1,321). D10 reversed-key false positive: **13**. | Plumbing (mostly) — but 5,883 of the 8,243 need re-examination before crediting either bucket | `sweeps.md` S3, S8, S12; `defects.md` D10 |
| T3 | **Self-erasing regeneration** — generators rebuild-from-scratch with no idempotency guard | Python: **2,110 fixture rows, 8 families, CLOSED** wave 29. Rust: **3 of 29 binaries confirmed-or-high-confidence vulnerable** (`gen_advanced_race_guide` live-reproduced — 93 spell + 15 equipment records wiped; `gen_companion_book` 927 companion records and `gen_core_rulebook_cache` 664 spell records confirmed by code-read, not yet live-tested; `gen_pathfinder_unchained` 42 equipment records), **17 of 29 never reached**. | Plumbing | `sweeps.md` S6 (PARTIAL); `defects.md` D9 (OPEN) |
| T4 | **Built-but-unreachable render surface** — content exists, PI-screened, but doneness never treats it as proof of holds | L8: up to **2,763** (class_feature descriptions). L9: **471 identified, 0 of 471 TRUE reachable** — a prior lane's own reachability claim was false and caught on review. | Plumbing | `levers.md` L8, L9 |
| T5 | **Book-level gate** — one missing `RuleSetId` variant short-circuits a whole book | **422** — `inner_sea_magic` 335, `inner_sea_temples` 64, `inner_sea_taverns` 20, `inner_sea_faiths` 3. Exact, per-book split independently reproduced. | Plumbing | `levers.md` L10 |
| T6 | **Allowlist standing in for a general mechanism** | **8 files** contain the literal word "allowlist" by grep — but 3 of the 8 are ingest/inventory binaries, not production rules paths, and only 2 of 8 have any *measured* excluded population (Druid/Monk LevelUpPlan gates; Cleric domain grounding, 15 of Bestiary 6's 18 class_feature units). The "8" is a word-search proxy, not a mechanism census. | Plumbing (confirmed, scope narrower than filed) | `defects.md` D4, D5; `levers.md` L2; `sweeps.md` S18 (renumbered, was S20 in lane draft) |
| T7 | **Shallow/single-hop traversal in a multi-hop relationship** | **4 units today**; 3 of 4 protected from silent fabrication only by an incidental level mismatch, not by structure. Confirmed end-to-end by independent re-derivation. | Plumbing | `defects.md` D12 (OPEN) |
| T8 | **Status stamp never re-examined once written** | **12** CRB class_feature units. Not independently re-derived this wave; carried forward unchallenged. | Plumbing | `defects.md` D13 (OPEN) |
| T9 | **Per-record onboarding backlog** ("key absent from X table") in already-registered books | **3,098 across 10 evidence-code families**, corrected from the previously-filed 2,651/6 families — the 447-unit gap (race_trait 238, class 152, race 57) was silently excluded from the earlier figure without being stated. Restated in full, not silently re-narrowed. | Plumbing (onboarding, not rules complexity) | `levers.md` L20 |
| T10 | **Unverified proxy measurement in the census process itself** | S11: 431 filed vs. 471 real (proxy wrong in both directions). B6/B9/B10/B11: original predicates unreproducible. S11 remains the single highest-leverage **unstarted** sweep in the ledger, because every other census number's reliability inherits from the same class of proxy. | Measurement-methodology, not unit-shaped | `sweeps.md` S11 (NOT STARTED beyond one instance) |
| T11 | **Policy-frozen capability** — the formula-interpreter ban, overturned this cycle | Directly gates **2,287 of G3's 3,320** formula-interpretable-but-unconsumed class_feature units; indirect share of every hand-written per-class function in the program unquantified. Sat unexamined ~18 waves after its own stated precondition (a fixture-check mechanism) was satisfied. | Neither — a standing policy choice, now resolved | Ruling §20 (resolution is the ruling itself) |
| T12 | **Genuine missing engine mechanism** (contrast bucket) | L0 prestige gating: 0 exists, gates 77 classes (downstream population uncounted). L1 chassis: **2,453**, confirmed exact by independent re-derivation — **but overlaps T2/S8 by a measured 1,354 units (floor) to ~2,124 (scaled), not the ~532 originally assumed** (see §2). L7 template mechanism ≥479. L13 three new sub-engines, 38 units. L3 monster/companion bridge, 28 units. | Rules complexity | `levers.md` L0, L1, L3, L7, L13 |

**Sum check, informally**: plumbing-shaped populations (T1–T10, excluding T11's policy-shape and
counting T2's 8,243 whole for now) run roughly **11,900–14,700** units by the same additive method
the original lane used; genuine-missing-mechanism (T12) runs roughly **3,000** units before
correction. The correction in §2 does not change either bucket's raw membership — it changes how
much of T12's 2,453 should also be counted inside T2's 8,243, which is a double-counting fix, not a
reclassification of any single unit's shape.

---

## 2. The overlap correction, in full

The original lane declared the T2/T12 overlap **"could not determine"** and assumed it was small
(~532, filed as a new sweep, not measured). An adversarial reviewer measured it directly:

- Join L1's population (2,453 units, `evidence` prefix `class_feature_of_unmodelled_corpus_class`)
  to the corpus on `data.key`: **1,564 of 2,453 join (63.7%)**.
- Of those 1,564, **1,354 carry a non-dispatched `data.class`** — meaning they sit simultaneously
  inside S8's 8,243-unit "plumbing" numerator AND inside T12's "genuine missing mechanism" bucket.
- Scaling the 86.6% (1,354/1,564) join-rate-adjusted overlap to the full 2,453 gives **~2,124**.
- The overlap's top `data.class` values are unambiguously missing-mechanism, not plumbing: Psychic
  85, Vigilante Talent 78, Medium 75, Magus Arcana 57, Vigilante 55, Magus 54, Shifter 50,
  Kineticist 45, Shifter Aspect 45, Spiritualist 44, Occultist 41, Antipaladin 30 — and the S8
  ledger's own row already says of this bucket "needs the class itself modelled, not a field remap."

**Corrected ratio arithmetic**: at the measured floor, plumbing ≈ 11,900–14,700 minus nothing (the
overlap units still count as plumbing-labeled by evidence code, they are just ALSO genuine-complexity
by content) — the honest statement is not "subtract from plumbing" but **"T12's 2,453 rules-complexity
figure is not cleanly additive against T2's plumbing figure; up to 2,124 of it is the same content
counted under a plumbing-shaped evidence code."** Read conservatively (treating the overlap as
rules-complexity, not plumbing, since modelling the class is what actually closes it): plumbing
≈ 11,900–14,700, rules-complexity effectively ≈ 3,000 (T12's own count, since the overlap units were
never double-counted in T12's own total, only in the *ratio*'s plumbing side) — **ratio ≈ 3.5:1–4.4:1**
using the measured floor, **≈ 3.3:1–4.2:1** using the scaled estimate. Both still clearly favor
plumbing. Neither is 4:1–5:1 with the precision first filed.

---

## 3. The compute-shape families, corrected

Non-overlapping primary-family partition of the 24,914 not-done units (a unit's PRIMARY family is
assigned by priority order when it carries more than one shape; the full overlapping "touches ≥1
token" view exists too but is not the operationally useful number). Independently reproduced exactly
by an adversarial reviewer, script-for-script:

| Family | Not-done units (primary) | Grammar already in `formula_interpreter.rs`? | What closes it |
|---|---:|---|---|
| Flat-constant magnitude (bare literal, e.g. +2 damage) | **1,747** | Yes — but irrelevant, see below | **Nothing from a compute library** — each consumer needs individual wiring regardless (per the finding lane's own §3: "removes zero units of this work by itself") |
| Per-level scaling (`<Class>LVL` bare/arithmetic) | **1,140** | Yes | The binding-layer accumulator (§3.1 below) |
| Ability-modifier-derived (STR/DEX/CON/INT/WIS/CHA in formula) | **804** | Yes, fully | Consumer wiring only — grammar is not the blocker |
| Named-counter/pool variable (e.g. a class's per-day-uses tracker) | **563** | Yes, via the accumulator pattern | The binding-layer accumulator |
| Clamped/capped per-level scaling (min/max/floor wrapping a level expr) | **368** | Yes, fully (n-ary min/max, real floor/ceil) | Depends on the per-level lever above |
| `classlevel(...)`-derived | **211** | Grammar yes, but **blocked on a real, disclosed interpreter bug** (`classlevel()` does not verify its class-name argument — `defects.md` D2) | D2's fix, then the accumulator |
| Conditional-step (if/boolean toggle) | **54** | Yes since wave 26 | The accumulator's boolean-flag sub-pattern |
| Other named-variable expression (residual) | **37** | Mostly — 4 tokens hit a real extraction bug (see §3.2) | Case-by-case |
| Skill-rank-derived (`skillinfo` TOTALRANK) | **17** | Yes, for this one keyword | Depends on `levers.md` L13 (bardic performance engine) |
| Level-threshold step-count (sum of level≥N indicator terms) | **7** | Yes | Consumer wiring only |
| **Total (primary partition)** | **4,948** | — | — |
| **Minus the flat-constant family (zero library benefit)** | **3,201 (12.8% of 24,914)** | — | **This is the real ceiling on what a compute library buys** |

**Multiplicity**: a unit that carries any formula shape at all averages 1.79 families (histogram:
1 family 3,834 units, 2 families 2,501, 3 families 539, 4 families 341, 5 families 21, 6 families 4,
7 families 1) — reproduced exactly.

### 3.1 The binding layer, corrected in both directions

1,156 distinct custom identifiers exist across the corpus's formulas. The originally-filed claim —
"541 of 1,156 (46.8%) reduce to ONE generic mechanism, a per-class current-level lookup keyed by
class name" — **does not survive**: only **101–103 of the 541** have a stem that is exactly a
modelled class name. The other **438 of 541** appear as a `BONUS:VAR`/`DEFINE` **target** somewhere
in the corpus — they are accumulated variables another record writes into, not bare class levels an
engine could hand over natively. (Worked example: `AlchemistBombLVL` is `DEFINE:AlchemistBombLVL|0`
plus two separate `BONUS:VAR|AlchemistBombLVL|<producer>` writes — an accumulator target, not a
lookup.)

The counterpart claim — "530 (45.9%) are genuinely one-off, the honest ceiling on what clustering
buys" — is wrong in the OPPOSITE direction: **391 of those 530** also have an in-corpus producer
token and are covered by the same accumulator mechanism.

**Net, corrected**: one generic producer-reading accumulator — the pattern `bonus_stack_reader.rs`
(329 lines, shipped wave 26) already implements for one narrow use — covers **893 of 1,156 (77.2%)**
distinct custom identifiers. The genuinely producerless, one-off residual is **263 of 1,156 (22.8%)**,
of which roughly 101 are bare class levels an engine can supply natively without any accumulator at
all. This is materially better news than either the over- or under-claim: the binding layer is not
"half solved, half bespoke" — it is "three-quarters covered by one already-proven mechanism, a
quarter genuinely bespoke."

### 3.2 A real, disclosed interpreter bug found in the process

`extract_formula_field` mis-extracts the 3-field `<SUBTAG>|<formula>|TYPE=<X>` token shape (e.g.
`BONUS:COMBAT|1|TYPE=NaturalArmor` extracts the literal text `TYPE=NaturalArmor` instead of the
formula `1`). Confirmed exactly **4 tokens corpus-wide** — safe today only because the mis-extracted
text happens to fail to parse as a formula, so it refuses rather than silently computing garbage.
Filed as `defects.md` D14.

### 3.3 Dice notation is a different, uncosted surface

Zero instances of `\d+[dD]\d+` exist inside the corpus's 2,671 distinct BONUS/DEFINE formulas.
**4,911 raw dice-notation tokens exist corpus-wide**, but in DESC (2,303), NATURALATTACKS (1,310),
DAMAGE (514), SPROP (457), ASPECT (218), and ALTDAMAGE (42) — a different corpus surface entirely,
sized for the first time this wave but not clustered into families or joined to units. Filed as
`sweeps.md` S17.

### 3.4 Frequency is not leverage

The single highest-frequency formula shape corpus-wide — `BONUS:STAT|<ABIL>|N`, an ability-score
bonus, 21.4% of every formula token in the corpus — gates almost none of the remaining work.
Corrected measurement, using a complete `(book, source_file, source_line)` join rather than the
53.6%-coverage join the first pass used: **326 of 24,914 not-done units (1.3%)** carry this token at
all, or **266** restricted to the exact single-ability flat-literal shape named. The DIRECTIONAL
finding survives the correction even though the number moved 4–5x: corpus-wide token frequency and
not-done-population leverage are different axes, and a shape-frequency-first automation strategy
would misallocate effort.

### 3.5 The real cross-cutting binding gap, corrected

`defects.md` D2 (`classlevel()`/`<X>LVL` binding) was measured this wave for the first time.
**Corrected reach, using the complete formula-field-only join** (the first pass conflated a variable
being WRITTEN with one being READ, and used a join that only found 53.6% of not-done units' corpus
records): **1,957 not-done units, 523 distinct variable names** — not the 2,340/616 first filed. This
is still the single largest identified cross-cutting binding gap once the accumulator mechanism
itself (§3.1) is separated from per-unit consumer wiring, but it is a smaller number than first
claimed, and that wrong number had already been written into `defects.md` as measured fact — corrected
in §7 below.

### 3.6 What already exists, provably

- **`formula_interpreter.rs`** (Ruling §20): a real recursive-descent evaluator, oracle-derived line
  by line, mutation-proven, gated on `derived_evaluator_fixture_check`. Recognises **2,553–2,671** of
  the corpus's 2,671 distinct BONUS/DEFINE formula segments (95.6–100%, two independent probes
  1 unit apart, both real parser runs not proxies).
- **`bonus_stack_reader.rs`** (329 lines, wave 26): a generic BONUS:VAR producer-accumulator with
  PREVARGTEQ gating, oracle-derived from `PreVariableTester.java`/`BonusManager.java`. Already proven,
  already shipped, currently used narrowly — the highest-leverage asset to generalize, not build from
  scratch. (One lane's PCGen-comparison claimed "no generic stacking mechanism exists anywhere in
  this codebase" — that claim does not survive; the file exists and does exactly this. The narrower,
  TRUE claim is that no TYPE-driven stacking-*suppression* rule exists — see §5's blocked item.)
- **`domain_power.rs`** (920 lines, wave 25/26): in-repo proof the "collapse N hand-written allowlisted
  functions into 1 evaluator + a transcribed-formula catalog" pattern works when tried — replaced 2
  hand-written Cleric/Inquisitor domain functions with 1 evaluator + a 5-domain, 42-token catalog.
- **~191 doc-cited, non-test compute functions** already exist across `pilot_compute/`+`rules_tables/`
  (widened from a prior lane's own 4-file, 172-hit citation scan, which itself contained 34
  test-fixture false positives). Most sampled bodies are literal, unmodified instances of the
  interpreter's own arithmetic grammar (`(level+2)/3`, `10+int_mod+level/2`) — proof the shape
  recognition, not the arithmetic, was always the missing piece.
- **A near-zero-cost dedup, corrected in scope**: `save_bonus` is duplicated in 16 files (not 17 — one
  `grep` hit is an unrelated function, `save_bonuses_from_feats`), 15 of which are genuine duplicates,
  14 byte-identical + 2 trivial variants — a real, cheap win once its visibility is changed from
  private `fn` to `pub fn` (it is currently NOT exported, so "never extended" understates the actual
  blocker). `base_attack_bonus`'s duplication is **NOT as cheap as first filed**: 21 files define it,
  20 with a DIFFERENT signature than the canonical (`fn(level)` vs. the canonical `fn(bab_progression,
  level)`) and 6 distinct one-line bodies — the real fix is threading a `BabProgression` argument
  through 20 call sites, not deleting 20 byte-identical copies.
- **A genuinely new, small gap**: level-banded table lookups (`min(5, level/4)`-style banding into a
  lookup row) are NOT covered by the interpreter today — it evaluates one formula per call, not an
  indexed table. 11 existing hand-written instances confirm the shape recurs (e.g. one class's own
  doc comment says the identical banding ladder is reused across 9 sibling columns). Small, real,
  uncosted beyond those 11 instances.

---

## 4. What was retracted this wave

**One lane attempted to close `sweeps.md` S2 (generalizing the Monk shape beyond classes) on the
claim "0 of 3 kinds (equipment/spell/feat) have an IdEnum+table+dispatch construction at all,"
against an explicit written prohibition retained in the same table row** ("Do NOT close S2 on the
enum-architecture argument... broaden it to 'any source-of-truth table vs. consumer drift'"). Both
the closure and its supporting architectural claim are **false**: `EquipmentCategory` (11 book
modules, `pub const ALL`, a backing table, a `corpus_file_name()` match dispatch), `FeatCategory`,
and `Pf1SchoolId` (20 enum sites total) all have exactly the construction the claim said did not
exist. The null result came from grepping three guessed type names (`SpellId`, `FeatId`,
`EquipmentId`) that are simply not the names in use — a name-shaped proxy, the same instrument-failure
mode this program has recorded repeatedly (see `MEMORY.md`'s "validate proxies against known truth").

**`sweeps.md` S2 stays PARTIAL, not CLOSED.** The real finding — a live counterexample in equipment
(§0, the APG `Equipmods` gap) — is preserved and strengthens the sweep rather than closing it: the
Monk shape CAN and DOES recur outside the three enum-mediated kinds a prior wave checked.

Separately, that same lane's enum-variant counts (race 0 of 7, monster 0 of 46) are real — both cited
tests genuinely assert what was claimed — but were presented as kind-level closure when they cover
only a fraction of each kind's board population (7 of 38 catalog races against 95 race + 3,504
race_trait board units; 46 of 326 catalog monster entries against 1,270 board monster units). Neither
test probes the corpus→enum direction, which is where the Monk shape actually lives. Restated
narrowly in §1's T1 row.

---

## 5. Automation candidates, ranked

**A candidate whose output cannot be independently verified is not viable, regardless of unit count —
the same condition Ruling §20 attached to the interpreter itself.** Every candidate below is scored
on that basis explicitly, not just cost and reach.

| Rank | Candidate | What it automates | Units closed | Build cost | How its output is checked |
|---|---|---|---:|---|---|
| 1 | **TYPE-facet / group-name triage tool** (new lever `levers.md` L22) | Reads a corpus record's raw `TYPE:`/group-name token into a structured facet, replacing at least 4 different ad-hoc substring/regex proxies used by prior census lanes | 0 directly (a measurement tool) — but corrects up to 1,321 currently-mis-evidenced S12 units and would materially improve at least 3 other named proxy failures (S11, the G1 444-name pool-axis census) | ~half day (unchanged estimate, cited three waves running) | **Trivial and strong**: output is the raw corpus token itself, byte-for-byte — anyone can re-run the same scan and get the same string back. No interpretation, no fixture needed. |
| 2 | **Generic formula-binding accumulator** (new lever `levers.md` L23, generalizing `bonus_stack_reader.rs`) | Resolves a named custom identifier by finding and summing its in-corpus BONUS:VAR/DEFINE producer chain, with PREVARGTEQ gating — the pattern already proven for one narrow use, generalized to be data-driven | Reach: up to 893 of 1,156 distinct identifiers (77.2%) have a resolvable producer chain; translated to not-done units this underlies most of the named-counter family (563) and a large share of per-level-scaling (1,140) — order-of-magnitude ~1,500–1,900 units EVENTUALLY addressable, not a guaranteed close (each still needs a wired consumer and its own fixture) | Medium — the accumulator PATTERN is proven, but generalizing identifier resolution from one hand-wired case to data-driven needs real engineering; sequence `defects.md` D2's `classlevel()` fix first (211 units in that sub-family are unsafe to bank through it until D2 lands) | **Strong, already-proven gate**: every value must clear `derived_evaluator_fixture_check` against a fixture transcribed from bytes the evaluator never reads — Ruling §20's own condition, already in force and mutation-proven able to fail |
| 3 | **Generator self-erasure lint** (cross-ref `defects.md` D9, `sweeps.md` S6; new lever `levers.md` L25) | Statically flags any `gen_*`/`enrich_*` binary that wipes-and-rebuilds a directory without a per-file exists-guard or key-preserving merge — the shape that has bitten this program at least 3 confirmed times, with 17 of 29 binaries still unassessed | 0 directly (regression-prevention, not doneness) — protects up to ~1,600+ records (927 companion + 664 core_rulebook spell + 42 pathfinder_unchained equipment, code-read-confirmed vulnerable) from a future silent regression | Low — mechanical check (does the write path guard-or-merge, yes/no), and the fix pattern is already shipped once (`gen_monster_book`'s own guard) as a template | **Deterministic**: pass/fail against a known-vulnerable fixture (a directory holding a file the generator does not own) — reproducible, no interpretation |
| 4 | **Race-trait compound-key matcher fix** (`sweeps.md` S3) | Fixes `modelled_race_of_race_trait()`'s exact-match-only key comparison to accept compound keys | Up to 2,472 units RECLASSIFIED off `race_trait_race_not_modelled` — explicitly not a guaranteed doneness gain, only a reclassification; each unit still needs its own doneness path afterward | **Unscoped** — 7 waves of naming this sweep without anyone pricing the actual matcher fix; ranked lowest of the four viable candidates for exactly this reason | A real regen diff against the pinned oracle (the standard verification this program already runs) — real, but not yet run; today's number is a population size, not a proven yield |

**Rejected — do not build, with reasons:**

- **Naive literal formula-shape cache.** 6,231 distinct raw shapes among only 2,289 formula-bearing
  G3 units — nothing to cache, one-off by construction.
- **OPEN/EXCLUSIVE pool-axis auto-classifier.** No oracle byte backs an automatic answer; hand
  classification already demonstrated a ~15% first-pass error rate (3 of 20 named groups got no real
  axis on first attempt).
- **Bulk pool/race_trait-group registration without per-group verification.** The identical failure
  mode was demonstrated twice already, waves 28 and 30.
- **DESC-tail formula extractor as currently specified.** Has a known, unguarded false-positive shape
  (a `PRE*` token inside a `DESC` tail misread as a formula) — not buildable until the guard exists.
- **`equipment_key_is_wired()` two-clause widen.** Closes 0, risks fabricating a computed zero.
- **Twice-run-diff fixture regression test.** Provably blind to the exact self-erasure bug class it
  was proposed for (see S6/D7).

**Named but not ranked — needs an operator ruling, not a build decision:**

- **A generic bonus-stacking mechanism restricted to TYPE-driven stacking-suppression** (new lever
  `levers.md` L26). One lane proposed a full "target-resolution + stacking" mechanism on the premise
  that no accumulation mechanism exists at all; that premise is wrong (`bonus_stack_reader.rs`
  already accumulates). The narrower, real gap — deciding whether two bonuses of the same TYPE stack
  or the higher wins — is real and unsized against the not-done population (the corpus carries 5,182
  BONUS-token TYPE= occurrences across 90 distinct values, corrected from a first-filed 7,897/277 that
  swept in non-BONUS TYPE-shaped fields like `EQMODTYPE=`/`RACETYPE=`). This needs the same kind of
  ruling §20 gave the interpreter before it is worth costing further — filed as `blocked.md` B16.
- **Level-banded table-lookup helper** (new lever `levers.md` L24, §3.6). Real, cheap-looking, but reach beyond the 11 named
  instances is unmeasured — flagged for a future costing pass, not enough evidence to rank yet.

---

## 6. What was already written down before this wave's measurement returned

**Reported loudly, per the wave's own instruction, because the operator has been burned by exactly
this shape before.** During this wave, before its lanes returned, a separate session opened the
program's next package (`docs/release/SD-32-compute-library-and-cause-closure/`) and committed a
scope document to `tranche/11` (commit `c93b87ddb`, docs-only, does not touch this package's files).
Its own scope section states plainly: *"33,830 formula tokens reduce to 1,049 shapes with the top 15
covering 80 percent."* That is the exact dispatch figure §0 above retracts — it did not reproduce
under any normalisation this wave tried, and it was already load-bearing in a downstream package's
committed scope before the retraction existed. The SD-32 document itself says its epics are
"deliberately provisional... wave 31 is measuring the compute-shape families and the root-cause
taxonomy now, and those numbers decide three of the five" — so it explicitly anticipated being
corrected by this document. **It has not yet been corrected; that is next-package housekeeping, out
of this wave's write scope (`docs/release/SD-31-corpus-closure-grind/` only), and is named here so it
is not silently inherited as fact.**

---

## 7. Reproduction

Every number above traces to one of these commands, run against the live, unmodified
`docs/work-inventory.json` and `data/corpus` on `tranche/11`:

```
# Board unchanged
md5sum docs/work-inventory.json
# -> d64ddfc677fd1683f5b7638889a25c54

# Twelve-shape corpus-wide counts: python3 filters over docs/work-inventory.json's `evidence`
# field, same method as scripts/coverage_ledger.py's own 46-group partition:
python3 scripts/coverage_ledger.py --groups docs/release/SD-31-corpus-closure-grind/artifacts/w30-coverage-table.json --strict

# Formula token/formula/shape census (33,830 tokens; 14,752 distinct raw values; 2,671 distinct
# formula segments; 10-family primary partition; 1,156 distinct custom identifiers):
# a walk of data/corpus/**/*.json summing raw_tokens with key=='DEFINE' or key.startswith('BONUS'),
# joined to docs/work-inventory.json's not-done population on (book, source_file, source_line).
# Full script reconstructed and independently re-run by two adversarial reviewers this wave;
# not re-committed as a script this cycle (measurement-only wave, no code/tooling changes banked).

# Interpreter grammar coverage:
cargo test --lib rules_core::pilot_compute::formula_interpreter::tests::corpus_shape_coverage -- --nocapture
# (run in an isolated CARGO_TARGET_DIR, deleted after)

# Overlap between T2 (data.class misattribution) and T12 (unmodelled-class chassis population):
# join docs/work-inventory.json units with evidence prefix class_feature_of_unmodelled_corpus_class
# to data/corpus on data.key; of the 1,564 that join, count those with a non-dispatched data.class.

# S9 class-keyed lookup table scope check:
grep -rn 'class_id: "class:' --include=*.rs src | wc -l
# -> 49 (not 42 as first filed — corrected this wave)
```
