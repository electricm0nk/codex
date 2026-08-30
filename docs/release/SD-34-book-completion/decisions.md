---
canonical: true
owner: god-emporer
bundle_id: SD-34
date: 2026-08-26
---

# SD-34 Decisions

Bundle-specific ADRs. Each states the decision, the reasoning, and — where it defers
something — the **revisit condition that must be checked, not remembered**
(`../../governance/deferral-revisit-doctrine.md`).

---

## §1 — SD-34's deliverable is the map, not the book count

**Decision.** SD-34's primary deliverable is an **exhaustive, mechanically-derived statement
of every step that remains for every unit in the corpus**. Completing the Core Rulebook is the
*proof* that the map is real, not the goal.

**Reasoning.** The operator named the problem directly:

> I need to know what is left. everything I think we are done, you surface 3 more things.
> that stops with sd-34.

That is a closure-completeness problem, and it has happened often enough to be a pattern
rather than bad luck. The cure is not more diligence — it is a fail-closed partition where
every unit lands in exactly one named bucket and `unclassified` is a hard error.

SD-33 delivered full ingestion and the shape engines. SD-34 uses them to establish what
*remains*, and prices it.

**Consequence.** The scoreboard is `scripts/completion_atlas.py --check`. A book count is a
by-product. **The 25%-of-37-books target is withdrawn as a success criterion** — the operator
named it negotiable, and a book count measures the wrong thing. It is replaced by a priced,
per-book, per-bucket forward plan (`scope-draft.md §7` S3).

---

## §2 — The atlas buckets, and the rule that an unpredicted step is a defect

**Decision.** Every unit lands in exactly one of ten buckets:

| Bucket | Meaning | Cleared by |
|---|---|---|
| `DONE` | nothing remains | — |
| `A` | engine has no table for this kind | building the table |
| `B` | table exists, record not in it | placing the record |
| `C` | held and computed, never surfaced | wiring the display/explanation path |
| `D` | other engine gap | per named sub-cause |
| `M` | magnitude ingested, never computed or applied | running the compute path |
| `V` | verified by proxy, never by the oracle | the SD-33 oracle harness |
| `U` | instrument cannot express a verdict | instrument correction |
| `X` | deferred with a stated reason | revisiting the stated condition |
| `Z` | not started | ordinary work |

**Any remaining step discovered that the atlas did not predict is a DEFECT IN THE ATLAS.** It
is logged as a `correction` retro event, the atlas is re-derived, and the discovery is
reported in `artifacts/epic-3-core-rulebook/atlas-defects.md`.

**Reasoning.** Discovering *work* is expected and normal. Discovering a *category* the map
missed is a failure of the deliverable itself. Separating those two is what makes the map
trustworthy rather than merely present. `D: other engine gap` is admitted only with its
sub-causes enumerated — a holding pen with a census, never a shrug.

**Enforced by:** AT-34-E1-002's fail-closed conditions; AT-34-E3-006's defect file (an empty
file is an excellent result, an absent file is a failure); AT-34-E6-001's re-derivation of the
atlas at HEAD.

**Relation to SD-33's `THE-BOX.md`.** The atlas is SD-34's partition of the inventory, in the
role `THE-BOX.md` played for SD-33. SD-34 has no `THE-BOX.md`. `scripts/box_ledger.py --check`
is inherited **read-only** as a second, independent partition of the same 49,438 units; it must
keep passing, and nothing in SD-34 writes to it.

---

## §2a — A shape engine computes a number; it does not complete a record

**Decision.** Recorded as a bundle fact, so no successor re-learns it.

**Reasoning.** `formula_interpreter` turns a formula string into a number — 10,626 of 11,652
recognised, 240 refused rather than guessed. That is its whole job.

It does not place the record in a table, attach it to a character, or show it to a player.
**26,396** units carry magnitude tokens; **13,119 of those 26,396** are still not held by the
engine. Half the shape engines' own feedstock is stuck downstream of them.

The engine's promotion ladder states the real requirement in code
(`src/bin/v06_work_inventory.rs:9595`) — four conditions, none of which is "a value was
computed":

```rust
if has_real_description
    && is_display_wiring_class_for_promotion(wc_class)
    && !universal_sheet_modifier
    && facts.class_feature_pool_catalog_holds(&unit.source_book, &unit.key)
```

**Enforced by:** AT-34-E1-004, which commits this as a proven statement with its counts
re-verified at HEAD.

---

## §2b — `not-ingested` is a misnomer and is renamed

**Decision.** The status field is renamed to state what it means. Ingestion is complete.

**Reasoning.** 26,002 of 26,002 of the field's units (100%) carry a real `source_file` and
`source_line`; 51,505 JSON files exist under `data/corpus/` (`content-unit-inventory.md §1`). Every evidence string is engine-side —
`ability_content_has_no_engine_table`, `class_feature_owner_matched_by_name_but_record_not_held_by_engine`,
and most explicitly `race_trait_record_loaded_but_never_applies`: **loaded**, then not applied.

The name asserts the opposite of the fact, and it has already misled — this package's own
first draft reported "52.7% not ingested" (26,047 of 49,438 at the time) to the operator, who had been told repeatedly and
correctly that ingestion was complete. A field name that is load-bearing in every report the
program produces, and wrong, is a correctness defect.

**Enforced by:** AT-34-E1-005, with a count sweep for the old string across `tests/`, `src/`,
`apps/`, `scripts/`.

---


## §3 — Every figure states its denominator in the same construct

**Decision.** Carried forward from SD-33 unchanged, and already mechanically enforced.

**Reasoning.** SD-33 built `scripts/denominator_gate.py` and wired it into
`scripts/verify.sh` as a real stage; its scan scope was later widened to cover a package's
markdown documents, not only its cycle receipts. It caught six separate agents' own
receipts during SD-33's run, including the closure scanner's.

**Enforced by:** `scripts/verify.sh --only denominator-gate` — live and inherited, **but its
default scan scope is SD-33's folder** (`scripts/denominator_gate.py` `BUNDLE_DIR` /
`DEFAULT_GLOBS`). A green default run examines zero SD-34 files. Until AT-34-E1-006 widens the
default to this package, every cycle and the launch checklist (`workflow-instruction.md §1`
item 12) run it against SD-34 explicitly:
`python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` →
`violations=0`. Do not narrow its scope to make a cycle pass.

---

## §4 — A lane's status is a mechanical function of its artifact

**Decision.** Every dispatched lane runs the count command on its own output artifact and
sets its `status` from that number. The literal command output goes in the lane's return
value. The verifying scan derives the **set**, not the size.

**Reasoning.** SD-33 remediation wave 2's equipment lane returned `"status":"complete"`
having written rows for 103 of its own 494-unit population. Nothing in its prose was false;
the status field was a self-assessment. Only row-counting caught it. Separately, a count can
match while membership does not, which is why the scan subtracts id-sets rather than
comparing lengths.

**Enforced by:** AT-34-E6-001's scan; the required `row_count_command_output` field in every
lane's return contract (`workflow-instruction.md §6` step 8).

---

## §5 — A method carried past its limit is corrected, and everything it judged is re-run

**Decision.** When a measurement method changes, the affected set is **derived by
execution** and every row in it is re-run. The scan verifies coverage as rows-re-run of
rows-in-affected-set, both with denominators.

**Reasoning.** SD-33's recurring failure shape, hit three times. One PCGen character per
unit at n=8,330. A whole-character `AC.TOTAL` diff that could not isolate an armor value and
silently conflated a MAXDEX cap. An equipmod-attachment mechanism that never produced a
trustworthy value. Each was corrected — but a corrected method leaves **stale agreements
that look exactly like real ones**, and one such correction shipped with its re-run
unverified until the closure scan caught it.

**Consequence for SD-34:** AT-34-E1-005's rename of the `not-ingested` status field is itself
a method change — the atlas's A/B/C/D arms key on that string. The atlas is re-derived in the
same cycle, and AT-34-E5-001 re-runs it at HEAD.

**Enforced by:** AT-34-E3-004; AT-34-E4-003's second limb; AT-34-E6-001's scan check.

---

## §6 — A blocker is cleared or escalated, never deferred

**Decision.** Carried forward. A `## Open blockers` entry is a request for an operator
ruling; filing one **pauses the bundle**. Two dispositions: clear it (decompose and run the
cycles — a large blocker is a sequencing problem, not an exemption), or raise a hand and
wait.

**Reasoning.** SD-33 filed three blockers and cleared all three by decomposing them,
including one whose fix lived in an entirely different subsystem (the corpus-extraction
pipeline dropping `.MOD`-attached EQMOD references). The lane that filed it reasoned that
the repair belonged elsewhere and therefore was not its work. **A fix that lives in another
subsystem is still a fix** — more work, same authority.

**Test that separates a blocker from a planned capability deferral:** was this scope in the
Definition of Done at launch? If yes, it is a blocker.

**Enforced by:** `../../governance/blocker-closure-doctrine.md`; AT-34-E6-001's scan reads
the `## Open blockers` section, bounding it at the next `## ` heading and ignoring archived
`<details>` copies.

---

## §7 — Eight tables are built, one is costed — and a miscount that changed the plan

**Decision.** Epic 2 builds **8 of the 9** missing engine tables: the seven the Core Rulebook
exercises (`ability`, `template`, `deity`, `domain`, `skill`, `language`, `companion`) plus
`trait`, which the second vehicle book supplies. **`power`** (421 units) is costed in Epic 5
from the measured build rate, not built.

**A correction that reshaped this bundle.** This package's first draft said the Core Rulebook
exercised **six of nine** tables. It exercises **seven** — the draft missed its 21 `deity`
units. Re-deriving the per-kind, per-book coverage found two things at once: the real number is
seven, and `ultimate_campaign` is an almost-single-bucket book whose 154 `trait` units make the
eighth table nearly free.

That is why AT-34-E1-003 now requires the **book-coverage map**, not just the table list. A
table's unit count says how big it is; only the book map says what it unblocks.

**Why `power` is not built.** All 421 of its units are inside `ultimate_psionics`, a 3,498-unit
book with all eight non-DONE buckets occupied (A=852, B=769, C=304, D=356, M=168, V=322, U=10).
Building the table would not close that book, so the work would land with no banked book to
prove it. Costed with a measured rate, it becomes the successor bundle's cleanest opening move.

**This is a scope ruling with a stated reason, not a deferral.** The test
(`../../governance/deferral-revisit-doctrine.md`): was it in the Definition of Done at launch?
No — SD-34's DoD is two completed books plus the priced plan. `power` is a *deliverable of the
plan*, named with its population, its cost, and what its book would still need afterwards
(AT-34-E5-003).

---

## §8 — Every launch figure is re-verified at the branch cut, not remembered

**Decision.** Every population figure in `scope-draft.md §3/§5/§6/§6a` and `epic-breakdown.md`
was derived by execution at `ea2b3396f2` and is **not provisional** (`§2b`, `README.md §8`). It
is still a figure measured at one commit. AT-34-E1-001's first run re-derives all of them at the
`tranche/14` cut SHA, and no closure work starts against a number that run has not printed.

**Reasoning.** This package has already been wrong twice by carrying a number forward (`§12`
L2), and once more by measuring one commit too early: its first measurement ran at the parent of
SD-33's final fold, and the fold moved DONE, B, M, D and `not-ingested` (all of 49,438) before
the package was even committed. The figures were re-run at the merged tip and the deltas recorded
in `content-unit-inventory.md §0`. The cut SHA may differ again.

**Revisit condition (checked, not remembered):** `artifacts/epic-1-atlas/completion-atlas.json`
landing with `derived_at` equal to the `tranche/14` cut SHA (AT-34-E1-001; staleness gate
AT-34-E1-002 condition 5). AT-34-E6-001's scan verifies that every figure carried into a later
epic was re-derived after that artifact existed, not inherited from this package's launch text.

**What "provisional" would have meant, and why it no longer applies.** An earlier draft of this
section called the `not-ingested` figure provisional and guessed that much of it was
`file_kind()` classifier noise. That guess was wrong in both directions: the figure is real
(26,002 of 26,002 units carry a real source line), and it means "the engine does not hold this
record", not "not ingested" (`§2b`). The noise hazard is still real for **per-kind** counts —
`file_kind()` types `.lst` files by filename — and AT-34-E1-003's per-book, per-kind map is where
it is checked.

---

## §9 — A measurement wave that banks zero units is a legitimate deliverable

**Decision.** Epic 1 is expected to bank zero closed units — it produces the map, not movement. It is judged on whether the
denominator it produces is true, not on movement.

**Reasoning.** Standing lesson, and SD-33's clearest single instance: correcting an
instrument is not closure, and closure counted against a wrong denominator is not closure
either. SD-31's three highest-value waves banked almost nothing and redirected the program
twice.

**Consequence.** A count that drops because measurement changed is reported in the
**instrument-correction** bucket, never folded into closure. All four buckets — closure /
reclassification / reachability / instrument-correction — appear in every cycle receipt.

---

## §10 — Verification happens at the widest build scope the repo has

**Decision.** Every cycle that changes shipping code, and the closure scan, run
`cargo test --locked --no-run` (does everything **compile**?), the full workspace run with
targets-executed counted, and `apps/desktop/src-tauri` **explicitly**.

**Reasoning.** SD-33 shipped a struct rename that broke one integration test file. The lib
suite stayed green at 2,836 passing while **0 of 543 integration targets executed** behind
that single compile error — for multiple waves, while the lanes that broke it reported
`complete`. `apps/desktop/src-tauri` is a separate cargo workspace a root sweep never
covers.

Second half of the lesson: **a lane's attribution of a failure is a claim, not evidence.**
One SD-33 lane blamed a different commit and called the breakage a pre-existing gap,
declining to fix it on that basis; `git show <sha>:<file>` across the candidate commits
disproved both claims in minutes. "Pre-existing at the branch cut" must be proven against
the cut SHA.

**Enforced by:** `workflow-instruction.md §6` step 3; AT-34-E6-001's scan.

---

## §11 — Build version

**Decision.** SD-34's first concrete build value is `0.14.0`, stamped in
`apps/desktop/package.json` and `apps/desktop/src-tauri/tauri.conf.json` at the
`tranche/14` cut.

**Reasoning.** The tranche digit moves on a **new `tranche/N` branch cut**, never on a
bundle's own closure. `tranche/13` carried `0.13.0` for SD-33; `tranche/14` carries `0.14.0`
for SD-34. Root `Cargo.toml` stays pinned at `0.1.0` and is not the version source of truth.

**Resolution point:** the `tranche/14` cut. SD-33's closure PR #377 merged to `develop` on
2026-08-27 (`ea2b3396f2`), so the cut is unblocked and happens as `workflow-instruction.md §1`
item 8. Until it lands, `README.md §1` records the branch as not yet cut — a documented deferral
with a named resolution point, not an unresolved placeholder.

---

## §19 — Operator ruling, 2026-08-29: `§17`'s disposition principle extends to bucket `V`

**Operator ruling:** *"assume the correctness of previous ruling and continue."* `§17`'s reasoning
governs bucket `V` as it governs `U`: **a unit carrying a real, named reason why no verdict can be
reached is dispositioned, not outstanding.** It is not a gap waiting on work; it is a checked
answer that happens to be "this cannot be checked, and here is why."

**The population, re-derived by the bucket-V lane against SD-33's own committed oracle results**
(`artifacts/epic-3-core-rulebook/bucket-v/`), with **zero new oracle runs**:

| Verdict | Units | of 2,793 |
|---|---:|---:|
| `agree` — real oracle round-trip, matched | 385 | 13.8% |
| `unverifiable`, SD-33's named reasons (`no_bonus_chain`, `oracle_export_no_spellname_line`, …) | 2,197 | 78.7% |
| `unverifiable` — `no_probe_surface` (AT-33-E1-003's census, 11 kinds) | 130 | 4.7% |
| **Dispositioned under this ruling** | **2,712** | **97.1%** |
| Not dispositioned — needs a probe built, out of that lane's scope | 81 | 2.9% |
| **Disagreements** | **0** | — |

**What made this reusable rather than a shortcut.** SD-33's harness genuinely produced these
verdicts and they were never fed back into SD-34's atlas — a reconciliation gap, not a missing
measurement. The lane freshness-checked a 20-row sample against the live corpus and found no
drift. **Zero disagreements**: nothing was found wrong, only unreachable.

**Two limits this ruling does NOT relax.**

1. **The 81 remainder is not covered.** Those need a probe built, and building it is work, not a
   disposition. They stay outstanding and named.
2. **`no_probe_surface` is a weaker claim than the rest** — it says *we never built the
   instrument*, not *the oracle cannot express it*. The 130 are dispositioned under this ruling,
   but they are the first thing to revisit if a future bundle builds those probes, and
   AT-34-E5-002's capability register carries them as a named capability rather than a closed
   question.

**A disagreement is never dispositioned.** If any later oracle run returns `disagree`, that unit
re-opens. There are none today, and that fact is what makes 2,712 reusable rather than assumed.

**Enforced by:** the consolidated results file, which cites the verdict and reason per unit;
AT-34-E6-001 re-deriving the atlas at HEAD; `PCGEN_ORACLE_SHA` named in every figure drawn from
the pinned corpus.

---

## §18 — Operator ruling, 2026-08-28: widen the anti-fabrication gates **by construction**

**Operator ruling.** Presented with three options on
`ANTI_FABRICATION_GATE_EXCLUDED_CLASSES` — the seven-class exclusion
(`wizard, bard, paladin, cleric, sorcerer, druid, monk`) blocking **218 of 242** remaining
`class_feature_owner_matched_by_name_but_record_not_held_by_engine` units in `core_rulebook`
(Sorcerer 137, Cleric 39, Monk 25, Wizard 7, Paladin 5, Bard 4, Druid 1) — the operator chose
**option A: widen by construction.**

**The rule.** An anti-fabrication gate accepts an explanation **when that explanation cites a real
corpus record**, instead of when its id appears on a hand-maintained allowlist. The allowlists
become a *property* — *"every explanation must trace to a corpus record"* — rather than a list of
known-good ids.

**Why this is a strengthening, not a weakening.** The gates exist to stop the engine inventing
rules that are not in the books, and that is the correct thing to guard. But their current
**shape** does not test it. Five `bard_level4..8` tests allowlist the whole `class_feature.bard.`
namespace, so **any new bard id fails regardless of correctness**, and a fabricated id already on
the list would pass. *"Cites a real corpus record"* is the property the guard was always reaching
for; the allowlist was a proxy for it. Replacing a proxy with the thing it approximates makes the
guard stronger, and it is the only form that scales to the remaining books.

**The bar this ruling must clear, and it is high.** `OPEN-ISSUES.md` row 338 records that wave
22's reconciliation attempt was **REJECTED as GAMED** for falsely claiming these gates needed no
widening. A cycle implementing §18 therefore may not:

- weaken, delete, or `#[ignore]` any of the nine acceptance tests;
- make a test pass by narrowing what it examines;
- claim a gate needs no change without a live run proving it.

It must instead show, per gate, that the **new** property is enforced and that a **fabricated
explanation still fails it** — a RED→GREEN mutation proof per gate, planting an explanation citing
no corpus record and confirming the catch. **A gate never observed to fail is not a gate.**

**Druid and Monk are a separate, third mechanism** and are not covered by the corpus-citation
property alone: `is_druid_pillar_id` / `is_monk_pillar_id` (`src/rules_core/level_up/`) are closed
id-prefix allowlists on `LevelUpPlan`'s explanation filter. They need the same by-construction
treatment applied to that filter, and a cycle that widens only the fabrication gates must say
plainly that Druid (1) and Monk (25) remain.

**Two of the seven were never documented.** Cleric and Sorcerer were found live by a lane running
the full suite, not from `OPEN-ISSUES.md`. Any cycle here runs the **full** suite against its own
draft rather than the scoped subset, because this exclusion list has already grown twice from
gates nobody knew about.

**Enforced by:** the nine acceptance tests, unweakened and re-run; a per-gate RED→GREEN mutation
proof; AT-34-E3-001's own bar; AT-34-E6-001 re-deriving at HEAD.

---

## §17 — Operator ruling, 2026-08-28: bucket `U` is DONE; bucket `X` needs the choice filter

**Operator ruling.** Presented with three options on whether records the engine deliberately does
not model can be `DONE`, the operator chose **option C — split the two buckets** — and supplied the
requirement that decides `X`:

> *"in x, when a character levels up the ui will query the back end to pull valid choices for the
> player to select from. the back end needs to be able to conduct the filter"*

### `U` — DONE

**Correction to this section's own examples, 2026-08-28.** As first written this ruling named
`BANE`, `FLM_BRST` and `FRT_HVY` as examples of the no-description shape. **They are not.** All
three carry a real corpus description and are among the 18 units that did **not** close. I took
three of four examples from the unit list without reading their corpus records — the same
field-name-is-not-field-meaning error this bundle already recorded twice. The ruling's *substance*
held; its illustrations did not, and a reader checking it against `BANE` would reasonably have
concluded the reasoning did not apply.

**What actually closed:** 110 of 321 units corpus-wide, `core_rulebook` **58 → 18**, all
`kind=equipment_modifier` with zero magnitude tokens and no description anywhere in the token
closure, verified by a whole-corpus before/after diff by unit id showing exactly 110 changed and
the id-set unchanged at 49,438. Writing the predicate generically rather than per-book moved 110
units where a `core_rulebook`-only one would have moved 40.

**The 30 corpus-wide remainders are a different shape, and split cleanly:**
- **21** carry an unresolved PCGen substitution (`%CHOICE`, `%d<N>`) — a real unmodelled choice or
  value. That is nearer bucket `X`'s "deliberately not modelled" than blanket DONE, and it needs
  its own ruling rather than an extension of this one.
- **9** trip a confirmed defect in `render_pcgen_desc`, which drops a bare `%` even when preceded
  by a digit, so a phrase like *"75 percent chance"* loses its sign. That is a **real shipping bug
  found incidentally**,
  filed as its own scoped fix, not absorbed into this ruling.


The 58 `core_rulebook` `unmeasurable` units are internal equipment-modifier codes (`BANE`,
`FLM_BRST`, `FRT_HVY`, `Magical Enhancments (+1..+10)`). **0 of 58 carry a magnitude token**, and
**37 of 58 are `visible: false`**. The player reads *"+1 Flaming Burst Longsword"* on the weapon
record; the code itself is plumbing that attaches an effect, never a thing anyone reads.

The precedent is already inside this book: **186 of its 1,380 `DONE` units are `visible: false`.**
Invisibility has never blocked `DONE` here, and these carry no content a player is owed. They are
finished.

### `X` — NOT done, and now with a named clearing mechanism

**This corrects `§16`.** *"Only the count grounds; which option is chosen is not modelled"* is
sound **engineering** for the magnitude, but it is **not a terminal product state**. The operator's
requirement makes the option rows real work: at level-up the UI asks the backend for the valid
choices, so **the backend must be able to filter the eligible set against a specific character**.
A static list of eight feats is not an answer to *"what can THIS sorcerer take right now?"*

That was the flaw in `§16` and in the ratified Fighter/Cavalier/Brawler precedent it rested on:
both stop at the count. The product needs the filter, so the precedent is **insufficient**, not
merely inconsistent.

**The capability is half-built, and the missing half is nameable:**

| Piece | State |
|---|---|
| `list_class_feature_pool_options()` (`apps/desktop/src-tauri/src/class_feature_pool_picker.rs`) | **exists** — returns `pool_options().clone()`, the whole static list, unfiltered |
| `evaluate_feat_prerequisites`, `evaluate_catalog_feat_prerequisites`, `character_prereq_facts` (`src/rules_core/feat_prereqs.rs`) | **exists** — can judge a prerequisite against character facts |
| a query joining them: *given this character, which options are valid* | **MISSING** — this is the whole gap |

**Consequence.** Bucket `X`'s clearing mechanism is no longer "revisit the stated condition". It is
**build the per-character choice filter and expose it to the UI**. That is a named, buildable
capability, and it belongs in AT-34-E5-002's capability register whether or not SD-34 builds it.
No cycle may move a unit into `X` to park it (`§16` amendment), and no cycle may call an option row
`DONE` on the strength of the count alone.

**Enforced by:** AT-34-E3-001's bar; AT-34-E5-002's capability register; AT-34-E6-001 re-deriving
both at HEAD.

---

## §16 — "Only the count grounds" is ratified precedent, not an open question

**Decision.** A class-feature record whose content is *"pick N from this eligible set"* is
**held** when the engine grounds the **count** and names the **eligible set**, without modelling
which option a given character picked. This is not a new ruling and no operator ruling is needed
to apply it: it is the treatment already shipped and described in the engine's own source as
**"the ratified Fighter/Cavalier/Brawler treatment"**
(`src/rules_core/pilot_compute/mod.rs`, `ARCANE_BLOODLINE_ELIGIBLE_BONUS_FEATS` and
`ground_sorcerer_arcane_bloodline_progression`): *"Only the COUNT of slots is grounded as a
magnitude; which feat fills a slot is a player choice this seam deliberately does not model."*

**Reasoning.** Cycles 2, 3 and 4 of `class_feature_owner_matched_by_name_but_record_not_held_by_engine`
each named this as an *"operator-scoped classification ruling"* and each declined to act, so
**103 of 344** units in the bundle's largest remaining mechanism (Sorcerer Bloodline Feat 87,
Ranger Combat Style Feat 16) sat still across three cycles waiting for a decision the codebase had
already made for four other classes. A question that precedent answers is not an escalation; it is
research a cycle owes before it stops.

**How to apply it.** Ground the slot count as a magnitude, name the real eligible set so the
explanation cites something rather than gesturing at an unnamed pool, and emit a
**non-claim-blocking diagnostic** stating what is deliberately not claimed — exactly the three-part
shape the Arcane Bloodline seam already uses. Do not seed a default choice, and do not report a
number the supersession rules would contradict.

**What this does NOT settle.** The sibling question — whether a record the corpus gives **no
content to at all** can ever be `held` — is still open and is genuinely a definitional question
about the deliverable (`artifacts/epic-3-core-rulebook/atlas-defects.md` entries 1-3). §16 covers
only records that *do* have content whose shape is a choice.

**Amendment, same day, after the first cycle applied it.** §16 as first written said the record is
*held* but never named the destination **status**. The cycle applied it and moved 93 units from
bucket `B` to bucket `X` (`deferred-with-reason`), reporting that as *"Closure: 93"*. It is not
closure: `B -> X` is **reclassification** under `§9`'s own four-bucket rule, and `X` must itself
reach zero for AT-34-E3-005. Core Rulebook `DONE` rose only **1,369 -> 1,380 (+11)** while `X` rose
**21 -> 116 (+95)**. The book advanced by 11 of 6,701, not 93.

The ratified precedent does not settle the destination either — its own feat-pool units carry
`text-complete`, `ingested-magnitude` and `engine-does-not-hold` across four records, so there is
no consistent prior answer to copy.

**What is settled and what is not.** Settled: grounding the count and naming the eligible set is
the correct *engineering*, and no operator ruling is needed to do it. **Not settled:** whether a
per-option row, whose option the engine deliberately does not model, is `DONE` or is a permanent
resident of `X`. That is the same definitional question as the no-content shapes in
`artifacts/epic-3-core-rulebook/atlas-defects.md`, and it decides whether *"every bucket to zero"*
is reachable at all. **A cycle must not resolve it by choosing a destination status on its own
authority** — report the movement honestly in the bucket it truly landed in, and leave the
question open.

**Enforced by:** AT-34-E3-001's own bar; AT-34-E6-001 re-deriving it at HEAD, and specifically
re-checking that no cycle reported a `B -> X` move as closure.

---

## §15 — A cycle that closes part of its population and names the rest reports `partial`, not `blocked-escalated`

**Decision.** A dispatched cycle has **three** terminal states, not two:

| Status | Meaning | Effect |
|---|---|---|
| `complete` | the cycle's whole assigned population reached the bar | its kanban row goes `complete` |
| `partial` | it closed part, and **named every remaining unit by sub-cause with populations that sum exactly** | its row stays `in-progress`; **the dispatch continues** and a later cycle takes the remainder |
| `blocked-escalated` | it needs an **operator ruling** — a policy or scope question no cycle may decide | **pauses the bundle** |

**Needing more cycles is never `blocked-escalated`. It is `partial`.**

**Reasoning — this was a defect in the dispatch contract, not in the lane.** AT-34-E3-001's
`class_feature_option_pool_record_not_held_by_engine` cycle did everything right: it closed 6 of
63, named all **57 of 57** remaining across seven sub-causes summing exactly
(28+10+9+3+3+2+2), explicitly **declined** to file a `## Open blockers` entry, and wrote in its own
receipt *"Not an operator-ruling request … this is a sequencing report."* Its `status` field still
had to read `blocked-escalated`, because the dispatch schema offered no other non-`complete` value
— and that value halts the wave. **An honest cycle was forced to choose the word that stops the
bundle.** A vocabulary that cannot express "I did my share, here is the named remainder" will keep
producing false pauses, and — worse in the other direction — tempt a cycle to report `complete`
over a partial result, which is the counterfeit-completion failure this program has hit repeatedly.

**This is not a new kanban state and not a deferral route.** `in-progress` already exists in
`kanban.md`'s vocabulary and is exactly what a partially-closed criterion is. Nothing is forwarded
to a successor bundle, nothing leaves the Definition of Done, and `§6`'s rule stands unchanged: a
blocker on the DoD is cleared or escalated, never deferred. The **named remainder is the price** —
a `partial` whose sub-causes do not sum to its stated total is a `complete` claim in disguise, and
fails the same way (`§4`).

**Enforced by:** the dispatch script's `CYCLE_SCHEMA` (`artifacts/sd-34-dispatch.workflow.js`),
where only `blocked-escalated` halts and `partial` requires the `remainder` field; AT-34-E6-001's
scan, which re-derives every `complete` from the repo and fails on any row still `in-progress`.

---

## §14 — Decomposing a criterion into more cycles is a sequencing decision, not an operator ruling

**Decision.** AT-34-E3-001's escalation is **cleared without an operator ruling**. Bucket B for
`core_rulebook` is **nine** mechanisms totalling **1,006 of 1,006** remaining units — re-derived
from `docs/work-inventory.json` at HEAD, not transcribed (the filing cycle said "ten"; the
enumeration returns nine, and their populations sum to 1,006 exactly). Each becomes its own
dispatched cycle, cheapest-first. The criterion's bar — bucket B at zero for `core_rulebook` — is
unchanged.

**Reasoning.** "This does not fit in one cycle, may I run more cycles?" is the orchestrator's
sequencing call, and `../../governance/blocker-closure-doctrine.md` answers it directly: **a large
blocker is a sequencing problem, not an exemption.** Filing it as `## Open blockers` pauses the
bundle to ask permission to keep working, which is the one thing an escalation must never be used
for. The filing cycle did the valuable half correctly — it **named the remainder by mechanism with
a population each**, which is what makes the next wave dispatchable at all. A remainder called
"the rest" would have been the defect.

| Mechanism | Units of 1,006 |
|---|---:|
| `domain_content_absent_from_domain_table_in_core_rulebook` | 1 |
| `race_trait_absent_from_race_traits` | 9 |
| `class_absent_from_ClassId_ALL_and_book_class_id_enums` | 17 |
| `deity_content_absent_from_deity_table_in_core_rulebook` | 21 |
| `class_feature_option_pool_record_not_held_by_engine` | 63 |
| `companion_absent_from_core_rulebook_companion_tables` | 100 |
| `race_trait_race_not_modelled` | 132 |
| `class_feature_owner_matched_by_name_but_record_not_held_by_engine` | 330 |
| `class_feature_option_pool_record_with_magnitude_not_held_by_engine` | 333 |

**The two sub-questions the filing cycle raised are answered, not forwarded:**

**`domain` (1 unit).** `Death (Pharasma)` at `cr_domains.lst:46` has no corpus JSON anywhere under
`data/corpus/core_rulebook/`. That is ordinary ingestion work through the guarded `gen_book_cache`
path — never hand-authored (`§N5`). No ruling needed.

**`deity` (21 units) — PI constraint, stated rather than escalated.** Every one of these records is
already redacted (`codex_generated_name: true`, key rewritten to `Codex-Named Unit (...)`). Two
precedents settle it. SD-32 `§28`'s **standing consequence**: a term is not Product Identity unless
it is on the `§19` 60-term list, and re-raising requires new evidence, not a fresh scan. And this
bundle's own AT-34-E2-001 already shipped the deity table **keying on and returning the masked
keys, because that is what the corpus holds** — explicitly recorded there as not a defect.

Placing these 21 records therefore proceeds **under a named constraint**: the fix matches on the
record's already-stored `source_file`/`source_line` coordinates and keeps the masked key. It must
**not read, log, emit, or reconstruct the redacted real name** in any new code path, receipt, test
name, or commit message. `scripts/verify.sh --only site-public-status-pi-gate` and
`--only site-dashboard-pi-gate` must stay green. **If a lane finds the work cannot be done inside
that constraint, that is a genuine escalation** — un-redaction is not a decision any cycle makes on
its own authority.

**Enforced by:** AT-34-E3-001's own bar (bucket B at zero for `core_rulebook`), re-derived by
AT-34-E6-001 at HEAD; the two PI gates above.

---

## §13 — AT-34-E1-007's blocker is CLEARED by decomposition, not by narrowing the criterion

**Decision.** The `corpus-trap-audit` stage AT-34-E1-007 wired reports **10,196 defects of 10,603
findings** against the live corpus, of which **7,015 of 10,196** are `wiring-class-mismatch`
across **34 of 37** books. The criterion is **not** re-scoped to "the stage exists and is wired".
The defects are driven to zero, as **AT-34-E1-008**, and AT-34-E1-007 closes when the stage it
wires genuinely exits 0.

**Reasoning.** The lane offered two dispositions: run the remediation, or rule the criterion
satisfied independent of whether the corpus is clean. The second is a carve-out — a gate that
passes because its bar moved, which `../../governance/blocker-closure-doctrine.md` and this
program's own history both reject. **A large blocker is a sequencing problem, not an exemption.**

The remaining 3,181 of 10,196 (`mod-record` 2,117, `key-differs-from-name` 650,
`shared-name-distinct-records` 249, `disabled-line` 165) are SD-33's already-verified,
already-out-of-DoD inherited debt (`forward-scope-register.md` D1.1's `v06_corpus_trap_report`
target). They stay registered, not absorbed: AT-34-E1-008's bar is `wiring-class-mismatch = 0`,
with the other four trap kinds reported at their unchanged counts.

**Verified independently before ruling** (not transcribed from the lane): the audit was re-run
from the orchestrating session — `findings=10603 DEFECT=10196 TRAP=407`,
`wiring-class-mismatch=7015 of 10196` across 34 books — and the regression history confirmed by
`git log -1 b32926f2af`, the `SD30-CARRY-001` commit that drove this same check `177 -> 0` on
2026-08-14. Nothing has run `--audit` between then and now. **That is the cost of an unwired
gate, and it is the exact thing AT-34-E1-007 exists to end.**

**Consequence.** Epic 1 gains `data/corpus/**` write scope for AT-34-E1-008 only, via the guarded
generator path (`gen_book_cache`), never hand-edits, never `--allow-stamp-loss` (`§N5`,
`risks-and-open-questions.md §6`). Precedent and mechanism: `SD30-CARRY-001` (`b32926f2af`) did
this for 10 books / 177 defects. The PI-and-`raw_tokens` survival check is per record, and
`corpus_literal_sweep`'s examined-population must move by exactly the record delta (`§12` L8).

**Enforced by:** AT-34-E1-008's per-book zero; AT-34-E1-007's own `exits 0` bar, unchanged;
AT-34-E6-001's re-run of both at HEAD.

---

## §12 — Lessons carried in from SD-33's run, each with its enforcing command

**A lesson without a mechanism is a quote.** SD-31's lessons were captured in SD-32's package
and ignored, because they were prose. L1–L5 come from the session that closed SD-33 and
authored this package; L6–L8 are SD-33's own retrospective §6 fold lessons
(`../../retro/sd33-computed-value-verification-retrospective.md`), written for this bundle and
carried here so they arrive as mechanisms. Each names what makes it fail, and **an entry here
without an enforcer is itself a defect** tracked in `risks-and-open-questions.md`.

The first five share one root: **a derived artifact was trusted instead of the source it derives
from.** A field's name instead of the code that writes it. An author's own earlier number
instead of the data. A workflow's status instead of the repo. A lane's account of a failure
instead of `git`.

### L1 — A field's name is not its meaning

Read the code that writes a status or verdict field before quoting it.

**Cost:** the `not-ingested` status means *"the engine does not hold this record"*.
26,002 of 26,002 of its units (100%) carry a real `source_file` and `source_line`. Reported to
the operator as "52.7% of the corpus (26,047 of 49,438) is not ingested", against a question they had asked repeatedly and been answered
correctly. The hazard was already in this package's own draft as a written warning, and the
number was quoted as fact anyway.

**Enforced by:** AT-34-E1-002 condition 6 — a bucket definition must cite the `file:line`
emitting the evidence strings it keys on, and the atlas fails closed when that citation stops
resolving. Plus AT-34-E1-005, which renames the field so it cannot mislead a third time.

### L2 — Never carry your own number forward; re-derive it

A figure inherited from an earlier document is a recollection, not a measurement.

**Cost:** two counting errors in this package, both from inherited numbers — the ingestion
figure, and "the Core Rulebook exercises six of nine tables" when it exercises seven (the draft
missed 21 `deity` units). The second error hid the second vehicle book until the count was
re-derived from the corpus.

**Enforced by:** AT-34-E1-006 — a `verify.sh` stage that fails on a figure with no reachable
re-derive command. Also closes `workflow-instruction.md §12` row 15's UNENFORCED marking.

### L3 — A dispatch script's return value is not a closure claim

Verify completion against the repo before relaying it.

**Cost:** an SD-33 workflow returned `closed: true` having never written its release notes; the
file still read `status: not generated` and the board row still read `not-started`. Separately,
a lane returned `"status":"complete"` over 103 of its own 494 units — caught only by counting
rows in its artifact.

**Enforced by:** AT-34-E6-001 — every `complete` in `kanban.md` is re-derived from the repo by
the scan, and no closure claim may rest on a script's own return value. Reinforced by
`§4`'s rule that a lane's status is a mechanical function of its artifact.

### L4 — Match structured fields, not substrings

**Cost:** an SD-33 remediation wave halted spuriously after a **passing** scan, because its
failure check searched for the words `blocked-escalated` and found them inside a sentence
stating that rows were *not* blocked-escalated. A full wave of wall time, on a green result.

**Enforced by:** `workflow-instruction.md §2.4`'s dispatch-script contract — the gate check
matches the scan's own `gate` and `status` fields, never a bare substring. The skeleton in that
section is the template every SD-34 dispatch is authored from.

### L5 — A repeated workaround means clear the obstacle

Three or more careful detours around the same problem is the signal to remove it.

**Cost:** an uncommitted staged revert of a landed fix sat in the shared checkout for five
consecutive waves. Four lanes politely worked around it in clean detached worktrees. The fifth
swept it into a commit titled *"release notes + version bump"* — 7 deletions, 142 modifications,
zero additions. It was never pushed, but a `git push` from that checkout would have reverted
verified work and deleted four pieces of closure evidence behind an open PR.

**Enforced by:** `workflow-instruction.md §10` step 1 and AT-34-E6-001 — any `incident`
recurrence key firing 3+ times must produce a **mechanical control**, or an escalation naming
why one is not possible. A better-worded warning does not satisfy it.

### L6 — A stale branch's file count is not its value

Read the branch's own record schema against HEAD's, and check whether the live consumer requires
a field the branch's records do not carry.

**Cost:** SD-33's closing sweep found a 1,612-file grant branch that looked like the largest
recovery available and was superseded (its `class` field held feature-group names, and it lacked
`granted_via_archetype`, which the consumer defaults to `true` when absent); the 45-record branch
that looked marginal was the real one. Folding by file count would have silently mis-marked
1,612 records.

**Enforced by:** `forward-scope-register.md §E1` — SD-33's three ruled-out branches are listed by
name and AT-34-E6-003's sweep treats that table as authoritative. Any branch the sweep finds
outside that table is diagnosed schema-against-HEAD before it is folded or deleted, and the
diagnosis goes in the sweep receipt.

### L7 — Run the suite after the last write that can move it

**Cost:** SD-33's `fold-skinwalker` re-pinned a population assertion correctly and reported the
lib suite green; the next commit regenerated `docs/work-inventory.json` for an unrelated reason
and moved the number again without re-running the suite. A true "0 failed" receipt produced a
red tree one commit later, and cost the bundle two more final-acceptance attempts.

**Enforced by:** `workflow-instruction.md §6` step 3 — the widest-scope build runs **after the
last commit in the cycle that can move a figure an assertion depends on**, and the receipt's
build-scope row names that commit's SHA. A cycle that regenerates the inventory after its test
run has not verified.

### L8 — A gate's examined-population must grow when records are added

**Cost:** the only proof that SD-33's fold records were genuinely inside `corpus_literal_sweep`'s
population was the examined-count moving 48,634 → 48,699, exactly the fold's +65 records. A
"0 findings" result with an unchanged count would have been indistinguishable from the sweep
silently skipping every new file.

**Enforced by:** `workflow-instruction.md §6` step 3 and AT-34-E6-001 — every cycle that adds or
regenerates corpus records reports the sweep's examined-count before and after, and the delta
must equal the record delta. A gate whose population did not move over a corpus change has not
examined it.

---

## §17 — A declared-but-deferred record type gets built when a second and third real consumer are named, not left declared forever

**Decision.** `companion_chassis::CompanionClassRecord` is built: `*_classes_companion.lst` rows
(a PCGen monster CLASS — a hit-dice progression, neither a creature nor an ability) are now a
third, real, held record type alongside `CompanionRecord` and `CompanionAbilityRecord`, verified
against all three of its real corpus-wide consumers in one cycle: `core_rulebook` (2 rows,
`AT-34-E3-001`'s own `companion_absent_from_core_rulebook_companion_tables` mechanism, closed
0-of-2), `ultimate_magic` (3 rows) and `book_of_the_damned_volume_1` (2 rows).

**Reasoning.** SD-29's own companion round 8 (`docs/release/SD-29-corpus-wide-catch-up-lanes/
decisions.md §65.1`) named this shape, screened it as DROP-AND-NAME rather than modelled, and
stated explicitly that modelling it is "a new record type... which a round taking one should
declare up front — this round does not take it." Three later `AT-34-E3-001` cycles (this
mechanism's own cycles 2-4) re-confirmed the same finding and, by cycle 4, had named all three
real consumers precisely (not merely "a monster-class shape exists somewhere") — the exact
condition SD-29 §65.1 set for taking the work. A record type declared against one consumer and
never checked against the other two named ones is unverified generalization; this cycle's own
`companion_class_record_generalizes_to_its_three_real_consumers` test proves the type against
all three, including the corpus's own second row shape (a bare-numbered `###Block: Level
Advancement` line — `um_classes_companion.lst:13`, `botd1_classes_companion.lst:8` — which
`v06_work_inventory::enumerate_file`'s directive screen treats as its own record because a first
field with no `:` is never a directive).

**What this does NOT settle.** `CompanionClassRecord` computes nothing — `hit_dice`/`max_level`
are carried verbatim, never fed into a BAB/save/hit-point formula, the same discipline
`CompanionRecord::monster_class`'s own doc states for the identical shape read from the creature
side. Reaching `grounded` (bucket B → D) settles nothing about bucket M (computed) or bucket V
(verified) — that is a different mechanism's job (`§2a`). `core_rulebook`'s own 84 `Animal
Companion ~ …` book-wide-grant ability rows were already attributed to creatures directly (Shape
7, pre-existing) and are unaffected by this decision; they were never blocked on this record type.

**Enforced by:** `companion_chassis.rs`'s `companion_absent_from_core_rulebook_companion_tables_
reaches_zero` and `companion_class_record_generalizes_to_its_three_real_consumers` tests, both
re-derived against the live corpus and `docs/work-inventory.json` rather than transcribed from a
prior cycle's receipt.

---
