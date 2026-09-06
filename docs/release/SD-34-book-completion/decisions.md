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

## §20 — Wave 33 lane A: `§2a`'s zero-magnitude ruling extends to set-shaped `class_feature` records with genuinely no upstream prose

**Question.** Bucket D's 27-unit `class_feature_*_held_by_*_table` shape (four rungs: `weapon-proficiency` 3, `weapon-and-armor-proficiency` 5, `class-skill-list` 10, `wizard-school-spell-list` 9) all carry `description: null` in `docs/work-inventory.json`. Wave 32 lane C recommended escalating the disposition to the operator. This cycle was directed **not** to escalate, and instead to determine from the data whether real `DESC:` prose exists upstream and was never ingested (a content fix), or genuinely does not exist anywhere (a different-shape ruling).

**Method — every one of the 27 checked individually against two independent sources**, not assumed from the shape:
1. The PCGen source tree (`~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilities_class.lst`) — the raw `.lst` row for every one of the 27 keys, read directly, checked for a `DESC:`/`SPROP:`/`BENEFIT:`/`ASPECT:` token.
2. The already-ingested corpus JSON (`data/corpus/core_rulebook/class_feature/**/*.json`) — `data.description`, checked directly, not inferred from `work-inventory.json` (which does not export a `description` field at all).

**Finding — the 27 split into two genuinely different shapes, not one:**

| Rung | Units | Real `DESC:` prose upstream? | Disposition |
|---|---:|---|---|
| `weapon-proficiency` (Bard/Druid/Rogue, `CATEGORY:Internal`) | 3 of 27 | **No** — confirmed absent both sources | Closed this cycle: `!has_real_description` → `grounded`/`class_feature_set_shaped_grant_carries_no_upstream_description_by_design` |
| `class-skill-list` (9 base classes + Jack of All Trades, `CATEGORY:Internal`) | 10 of 27 | **No** — confirmed absent both sources | Closed this cycle, same evidence string |
| `wizard-school-spell-list` (9 schools, `CATEGORY:Internal`) | 9 of 27 | **No** — confirmed absent both sources | Closed this cycle, same evidence string |
| `weapon-and-armor-proficiency` (Bard/Fighter/Paladin/Ranger/Rogue) | 5 of 27 | **Yes** — real multi-sentence `DESC:` prose exists in BOTH the PCGen source (`cr_abilities_class.lst` lines 2814/2816/2818/2819/2820) AND is already ingested into the corpus JSON (`data.description` populated, `VISIBLE:DISPLAY`, `wiring_class: "display"`) | **Not closed this cycle** — see below |

**22 of 27 (the first three rungs): the "different shape" branch.** These are genuinely set/list-shaped records (`CSKILL:`/`SPELLKNOWN:`/`AUTO:WEAPONPROF` tokens only) with no player-facing prose anywhere upstream — not an ingestion gap, a real absence. `§2a`'s standing ruling ("a zero-magnitude feature whose description is shown to the player is COMPLETE") is extended here to its honest sibling: **a zero-magnitude, set-shaped record that has no description to show is also complete** — there is no display gap to close because there is nothing to display. A new, distinct evidence string, `class_feature_set_shaped_grant_carries_no_upstream_description_by_design`, marks this — deliberately NOT `text-complete` (that status would assert real rendered prose that does not exist) and carrying no substring of `completion_atlas.py`'s `_DONE_VIOLATION_MARKERS`. It uses the SAME `status: "grounded"` the file's own existing "bounded grant-only identity record" idiom already uses (Sorcerer/Wizard's own Weapon and Armor Proficiency grounding, `pilot_compute/mod.rs`), so `completion_atlas.py`'s existing `DONE` citation (`status in {grounded, text-complete}`, `v06_work_inventory.rs:10195`) already covers it — no `BUCKET_DEFINITIONS` schema change needed, only the new marker itself, verified absent from `_DONE_VIOLATION_MARKERS`.

**5 of 27 (`weapon-and-armor-proficiency`): the "ingest it" branch, discovered to already be half-true and half-blocked — reported, not rushed.** The prose exists AND is already ingested; what's missing is that the ENGINE side never wired it. `v06_work_inventory.rs`'s `Kind::ClassFeature` arm promotes a text-only, real-description, `display`-wiring-class record to `text-complete` only once `pilot_compute`'s own `class_feature_effect_wired` probe observes a grounded explanation for that exact key — and `pilot_compute::explain_base_class_weapon_and_armor_proficiency`/`ground_class_weapon_and_armor_proficiency` (the precedented idiom that already grounds Sorcerer, Wizard, Cleric, Assassin and Shadowdancer's own version of this exact record shape) has never been extended to Bard, Fighter, Paladin, Ranger, or Rogue. Extending it is real, in-scope-shaped, precedented work — but doing it at the SAME rigor the precedent requires (Cleric's own cycle 6 read every one of that class's registered archetypes before trusting any replacement text) is not a same-cycle-safe move here: all five of these classes have real, registered archetypes across `rules_tables/*/archetype_tables.rs` that supersede one or more of their proficiency slots (many more matches than Cleric's single Ecclesitheurge case) — grounding them without reading every one risks shipping stale archetype text to a player who selected one, which is exactly the fabrication risk `AGENTS.md` rule 7/8 warns against. Named exactly as next-cycle scope in this wave's own receipt, not closed here.

**Enforced by:** `python3 scripts/completion_atlas.py --check` (population/DONE/D deltas, `done_evidence_violations=0`, `citation_failures=0`); the two source-file checks named above, individually, per key (this receipt's own `figures` section carries the literal grep commands); `cargo test` on the touched module.

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
| `agree` — real oracle round-trip, matched | 385 of 2,793 | 13.8% of 2,793 |
| `unverifiable`, SD-33's named reasons (`no_bonus_chain`, `oracle_export_no_spellname_line`, …) | 2,197 of 2,793 | 78.7% of 2,793 |
| `unverifiable` — `no_probe_surface` (AT-33-E1-003's census, 11 kinds) | 130 of 2,793 | 4.7% of 2,793 |
| **Dispositioned under this ruling** | **2,712 of 2,793** | **97.1% of 2,793** |
| Not dispositioned — needs a probe built, out of that lane's scope | 81 of 2,793 | 2.9% of 2,793 |
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

## §21 — Wave 34 lane A: `§20`'s deferred 5-unit `weapon-and-armor-proficiency` rung closes, and the wave-33 receipt's own "all five have real archetypes" premise was wrong for two of them

**Decision.** `pilot_compute::explain_base_class_weapon_and_armor_proficiency`/
`ground_class_weapon_and_armor_proficiency` — the idiom already grounding Sorcerer, Wizard,
Cleric, Assassin, and Shadowdancer's own version of this record shape — is extended to Bard,
Fighter, Paladin, Ranger, and Rogue, closing `§20`'s named 5-unit deferral
(`class_feature_weapon_and_armor_proficiency_grant_held_by_class_proficiency_tables`). All five
now ground to `text-complete` (bucket D → DONE).

**Method — every one of these five classes' own registered archetypes read individually**, the
same rigor `§20`'s own next-cycle plan demanded (Cleric's cycle 6 precedent), not grep-and-trust:

| Class | `proficiency_slot_ids` passed | Archetypes found claiming a slot | Own replacement text resolved? |
|---|---|---|---|
| Bard | `BardWeaponProficiencies`, `BardArmorProficiencies` | Geisha (UM, both), Dervish Dancer (UC, weapon only) | No — both archetypes' own catalog `grants` name no "~ Weapon and Armor Proficiency" sub-feature with text; "not resolved" branch |
| Fighter | `FighterArmorProficiencies`, `FighterTowerShieldProficiency` | Cad, Gladiator, Tactician, Unarmed Fighter (armor); Dragoon, Unbreakable (tower shield) — all UC | No — all six carry `description: None` on their own proficiency grant; "not resolved" branch |
| Paladin | `PaladinArmorProficiencies`, `PaladinWeaponProficiencies` | Holy Gun (UC, both) | No — `description: None`; "not resolved" branch |
| Ranger | `&[]` (empty) | **None** | N/A |
| Rogue | `&[]` (empty) | **None** | N/A |

**The wave-33 receipt's own premise — "all five of these classes have real archetypes doing
exactly that, unlike the zero-archetype Assassin/Shadowdancer precedent" — does not hold for
Ranger or Rogue.** Every registered archetype for both classes across every archetype-table
module that carries them was read; not one `replaces` list names any of the eight TYPE facets
the base Ranger/Rogue corpus record's own `!PREABILITY` negation gates reference
(`RangerArmorProficiencies`/`RangerWeaponProficiency`/`RangerLightArmorProficiency`/
`RangerMediumArmorProficiency`/`RangerShieldProficiency`/`RogueWeaponProficiencies`/
`RogueArmorProficiencies`/`RogueLightArmor` — confirmed by direct grep, zero matches for any).
PCGen's own upstream data anticipates such an archetype existing somewhere in the wider game
line; none of the books this engine has ingested happens to be it. Ranger and Rogue pass empty
`proficiency_slot_ids`, the identical shape to Assassin/Shadowdancer — a corrected finding, not a
shortcut (`scripts/retro.py correction --subject wave33-lane-a-receipt --claimed
"all_five_classes_have_superseding_archetypes" --actual
"ranger_and_rogue_have_zero_superseding_archetypes_in_this_engines_catalog" --verified-by "grep
-rn 'RangerArmorProficiencies\|RangerWeaponProficiency\|RangerLightArmorProficiency\|
RangerMediumArmorProficiency\|RangerShieldProficiency\|RogueWeaponProficiencies\|
RogueArmorProficiencies\|RogueLightArmor' src/rules_core/rules_tables/*/archetype_tables.rs"`).

**The fabrication hazard the dispatch named by name, confirmed real and closed by omission, not
by a runtime guard.** Paladin's own `Divine Hunter` archetype replaces
`PaladinArmorProficiencyHeavy` alone (heavy armor only — light/medium armor and every weapon
proficiency are untouched), and names no "~ Weapon and Armor Proficiency" sub-feature of its own
(its real grant is "Divine Hunter ~ Precise Shot", "This ability replaces her Heavy Armor
Proficiency"). `PaladinArmorProficiencyHeavy` is deliberately absent from Paladin's
`proficiency_slot_ids` list for exactly this reason: including it would make a Divine Hunter
selection wrongly claim "the base progression does not apply" for a class whose light/medium
armor and weapon proficiencies remain fully in force — shipping stale, over-broad archetype text
to a real player who selected a real archetype. `pilot_compute::base_class_weapon_and_armor_
proficiency_tests::paladin_weapon_and_armor_proficiency_divine_hunter_does_not_supersede_the_
base_grant` pins this directly: with Divine Hunter selected, the base grant's own full text
still renders, unchanged, un-superseded.

**A second, distinct blocker beyond `pilot_compute` wiring, found and closed in the same
cycle.** Wiring `pilot_compute` alone does not promote this key shape to `text-complete` through
`v06_work_inventory.rs`'s own GENERIC owner/group match (`class_feature_exact_suffix_grounded`'s
`group.eq_ignore_ascii_case(&class_name_as_group_text(owner))` guard) — this record's own key is
REVERSED (`"Weapon and Armor Proficiency ~ <Class>"`, `group` = the constant text `"Weapon and
Armor Proficiency"`, never a class name), which is exactly why `classify()`'s `Kind::ClassFeature`
arm treats it as an owner-resolution FAILURE and falls to `weapon_and_armor_proficiency_grant_
class_id`'s own named-list branch in the first place. `class_feature_owner_via_type_facet`'s own
doc comment proves this generic path can never widen what grounds a record through that owner/
group guard — so the wave-33 receipt's assumption that grounding these five would "promote [them]
to `text-complete` via that SAME earlier check" the way Cleric did was also incomplete. This
cycle's fix is in the SAME branch `§20`'s own comment already named
(`weapon_and_armor_proficiency_grant_class_id`'s consumer in `v06_work_inventory.rs`): it now
checks `facts.explanation_ids` directly for the new `class_feature.<slug>.weapon_and_armor_
proficiency` ids `ground_class_weapon_and_armor_proficiency` emits, gated by the same three
display-wiring guards (`has_real_description`, `is_display_wiring_class_for_promotion`,
`!universal_sheet_modifier`) every sibling `text-complete` promotion in this file already
requires.

**Figures.** `python3 scripts/completion_atlas.py --check`: population **49438** (unchanged),
`D: 2924 → 2919` (**-5**), `DONE: 24994 → 24999` (**+5**), `overlap=0 unclassified=0
done_evidence_violations=0 citation_failures=0`.

**Enforced by:** 10 new `pilot_compute::base_class_weapon_and_armor_proficiency_tests` cases (a
base-grounding + an archetype-supersession/non-supersession test per class, including the Divine
Hunter negative-control above) plus 2 new `v06_work_inventory.rs` classify() tests (the positive
promotion case and a per-class-slug control) and 1 renamed pre-existing negative control;
`cargo test --lib --bin v06_work_inventory -- weapon_and_armor_proficiency` (lib 24 passed, bin 3
passed, 0 failed); `python3 scripts/completion_atlas.py --check` (figures above).

---

## §22 — Operator ruling, 2026-09-04: bucket D's genuine new-chassis remainder stays in scope for SD-34

**Decision.** The growing "genuine new-chassis, not a classifier fix" remainder inside bucket D
— confirmed across waves 37-40's own investigations — **stays in scope for SD-34**. It is not
deferred to a future bundle (an "SD-35"-style split), and it is not to be silently left as "the
rest" once the cheap classifier-side fixes run out.

**What this remainder actually is, named precisely as of wave 40's own close:**
- Shape 2's own confirmed new-chassis units (no per-feature compute function exists anywhere):
  Duelist (4), Shadowdancer (4), Assassin (2), Loremaster (2), Cleric's Aura (1), Paladin's
  Detect Evil (1), Wizard's Arcane Bond (1) — 15 units, each independently traced against real
  engine source (wave 39 lane B's own receipt).
- Sub-mechanism 5 (`class_feature_of_unmodelled_corpus_class`'s own largest split): 634 units
  across 60 classes, independently investigated twice (wave 37 lane B, wave 38 lane B) and both
  times found genuinely too large for a single wave-sized cycle — real per-class chassis-
  building work, not a matcher widening.
- Three individually-named single-unit gaps needing a genuinely different fix each (Fighter's
  Weapon Training — a new engine-side explanation id; Psychic's Phrenic Pool — a probe-input
  widening; Monk's Stunning Fist — a classifier owner/namespace-recognition change), all
  confirmed and declined-not-attempted across waves 39-40.

**Why this needed a ruling rather than another "pick the cheapest item" wave:** the cheap,
classifier-side fixes in this area are now largely exhausted (waves 38-40 closed 144 units this
way in three waves; the confirmed remainder above is qualitatively different work). Continuing
to dispatch "cheapest next item" waves against this remainder without a scope ruling risked
either silently treating the 634-unit sub-mechanism-5 population as permanently out of reach, or
under-scoping a wave against work that genuinely needs real chassis-building effort.

**How to apply.** Future waves against this remainder should be scoped as real feature-building
work (new `ground_<class>_class_features`-style dispatch functions, following the same
SD-32-card-11 precedent this whole area already established), not classifier-matcher tweaks.
Expect materially lower units-closed-per-wave-effort than the classifier-fix waves that preceded
this ruling — that is the honest shape of the remaining work, not a regression. No further
operator ruling is needed to keep dispatching against this remainder; this entry is the standing
authorization.

**CORRECTION, 2026-09-04 (same day, operator-prompted re-audit): the three named single-unit
"genuinely different fix" gaps above were OVERSTATED — do not trust the "structurally larger"
framing for any of them without re-checking, and treat the 15/634-unit piles as UNVERIFIED
difficulty, not confirmed-hard, until someone actually reads the code:**
- **Monk's Stunning Fist**: the compute is already fully wired and tested
  (`feat_effects::stunning_fist_facts_from_feats`, emits
  `feat.standalone.stunning_fist.save_dc` / `.uses_per_day`). `class_feature_known_synonym_
  grounded` (the literal `CLASS_FEATURE_ID_KNOWN_SYNONYMS` table built in wave 39) does a pure
  `owner`/`group` + full-string-id lookup with NO requirement that the id contain `.owner.` as a
  substring — this record's own `group` ("Monk") already satisfies that guard. **A single table
  entry closes this**, no new mechanism needed. The "needs the classifier's own group==owner
  guard... a structurally larger change" framing was written against the OLDER
  `class_feature_exact_suffix_grounded` check only, without re-checking against the synonym
  table that already supersedes it for exactly this shape.
- **Fighter's Weapon Training**: the claim "no discrete id — folded directly into a combined
  total, never pushed as its own `ComputationExplanation`" is **factually wrong**, verified by
  direct read: `pilot_compute/mod.rs` pushes a real, live `class_feature.fighter.weapon_training`
  explanation (an exact-match id, no synonym table even needed). It simply never fires during
  classification because the classifier's own generic per-class sweep never supplies a
  `FIGHTER_WEAPON_TRAINING_GROUP_CHOICE_ID` selection — the SAME "needs one canonical default
  choice" gap `canonical_seeds_for()` already solves for wizard/arcanist/sorcerer/cleric/druid
  and others. Adding a `"fighter" => (...)` arm to that same existing match statement is the
  fix, not new engine code.
- **Psychic's Phrenic Pool**: same root cause as Fighter's — the compute and its explanation id
  (`class_feature.untabled.psychic.phrenic_pool.value`) already exist and are tested; the
  generic sweep just never selects a Psychic Discipline. Same `canonical_seeds_for()` fix shape
  as Fighter's, not a bespoke "probe-input widening."

**What held up on re-check:** Wizard's Arcane Bond (one of the 15), spot-checked in full —
genuinely no compute exists anywhere for it (confirmed by exhaustive grep, unlike the three
above). Paladin's Detect Evil and Cleric's Aura were spot-checked only partially (inconclusive)
— a real antipaladin analog (`detect_good`) already exists for Paladin's own opposite-alignment
case, which is at minimum suggestive that Detect Evil may follow the identical already-built
shape rather than needing new work; this was NOT confirmed either way and needs an actual check
before scoping a wave against it. The 634-unit sub-mechanism-5 population has not been
re-audited at all since this correction — its own "genuinely too large" framing from waves 37/38
predates this finding and should be treated as unverified, not confirmed, until it is.

**How to apply, corrected:** before scoping ANY future wave's difficulty against this
remainder's own prior write-ups, re-verify against the real compute functions first (grep the
real `pilot_compute/mod.rs`, don't trust a prior wave's "no compute exists" or "needs a
different kind of fix" claim at face value) — this bundle has now demonstrated a repeatable
failure mode where a cheap, already-precedented fix (a table entry, or an existing
`canonical_seeds_for()` match arm) got mischaracterized as bespoke new-feature work. Fighter's
Weapon Training and Psychic's Phrenic Pool are BOTH now believed to be `canonical_seeds_for()`
match-arm additions, not real feature-building — re-scope wave 41+ accordingly rather than
routing them into "Epic 4/5-shaped" effort.

**FURTHER UPDATE, 2026-09-04 (wave 41): all three of the above (Stunning Fist, Fighter's Weapon
Training, Psychic's Phrenic Pool) are now closed** — `core_rulebook:class_feature:monk_stunning_fist`
→ `literal-verified` (V), `core_rulebook:class_feature:fighter_weapon_training` and
`occult_adventures:class_feature:psychic_phrenic_pool` → `grounded` (DONE), independently
re-verified by the orchestrator against a fresh `docs/work-inventory.json` join, not just taken
on the fixing agent's word. `DONE: 25358→25360`, `D: 2523→2520`, `V: 321→322`. Full receipt:
`artifacts/bucket-d-mining/wave41_shape2_three_corrected_units_cycle_receipt.md`.

**Paladin's Detect Evil and Cleric's Aura, now properly checked (real verdict, not a guess):**
neither matches the cheap classifier-wiring shape that closed the three above — for both, an
exhaustive grep confirms **no compute function and no `ComputationExplanation` id exist at all**
anywhere in the engine (unlike Stunning Fist/Weapon Training/Phrenic Pool, where the compute
already existed and only classifier-visibility was missing). By the strict cheap-fix/genuinely-hard
binary this maps to genuinely-hard, same bucket as Wizard's Arcane Bond. **But with a real,
material caveat**: both are pure class-level pass-through magnitudes with an exact, already-built
structural precedent sitting in the same file family, for the antipaladin (Paladin's own mirror
class) — `detect_good_caster_level()` and `aura_of_evil_strength_level()` in
`src/rules_core/rules_tables/apg/antipaladin_features.rs` (built as an SD-32 follow-up
specifically noted as "mirroring the CRB Paladin's Aura of Good/Detect Evil," but the Paladin/Cleric
originals were never symmetrically added). Writing `detect_evil_caster_level()` / a Cleric
aura-strength function is realistically a ~10-line copy-adapt of those two existing functions,
plus wiring (Paladin currently has no `ground_paladin_class_features`-style push at all; Cleric's
would extend `explain_cleric_level1_spell_baseline`). **Scope these as "small, precedented new
compute, citing the antipaladin functions as literal templates" — not as classifier-reachability
work (like the three above), and not as open-ended Epic 4/5 feature-building (like Arcane Bond).**
This is a third difficulty tier this bundle has not previously named explicitly; worth watching
for elsewhere in the 15-unit new-chassis list and the 634-unit sub-mechanism-5 population before
assuming everything left is one of the two extremes.

**WAVE 42 UPDATE, 2026-09-04: Paladin's Detect Evil and Cleric's Aura are now CLOSED** — both
`core_rulebook:class_feature:paladin_detect_evil` and `core_rulebook:class_feature:cleric_aura`
→ `grounded` (DONE), via two new pure functions (`paladin_detect_evil_caster_level`,
`cleric_aura_strength_level`) copying the antipaladin precedent exactly, independently
re-verified by the orchestrator (fresh `completion_atlas.py --check` + a direct id→status join).
`DONE: 25360→25362`, `D: 2520→2518`. Full receipt:
`artifacts/bucket-d-mining/wave42_paladin_detect_evil_and_cleric_aura_cycle_receipt.md`.

**Also this wave: a real, rigorous re-audit of the remaining 12 units** from the original 15-unit
"genuinely new-chassis" list (Duelist ×4, Shadowdancer ×4, Assassin ×2, Loremaster ×2) —
NOT yet fixed, but the difficulty finding matters for scoping. **Headline: none of the 12 is
genuinely-hard. All 12 are small-precedented-new-compute**, several with an even stronger
precedent than Paladin/Cleric had (multiple existing byte-for-byte-comparable functions already
shipped for other classes, not just one antipaladin mirror). Specifics:
- All 4 classes (Duelist, Shadowdancer, Assassin, Loremaster) are prestige classes already
  registered in `prestige_class_entry_gate::is_registered`, and a generic mechanism
  (`class_feature_grant_consumer::push_generic_class_feature_grant_records`) already fires for
  all of them — but it only emits the feature's grant LEVEL, never its real magnitude, which is
  why these 12 units still show `engine-does-not-hold` despite that mechanism running. This is
  NOT a classifier-visibility gap (unlike wave 41's 3 units) — each genuinely needs one new
  formula function.
- Every one of the 12 has a directly comparable existing function already in
  `pilot_compute/mod.rs` to copy: e.g. Duelist's Precise Strike (`+level` weapon damage) mirrors
  `swashbuckler_precise_strike_damage` almost verbatim; Assassin's Save Against Poisons formula
  (`AssassinLVL/2`) is already a literal string in this repo's own test fixtures
  (`class_feature_grant_consumer.rs:2392`); Shadowdancer's Shadow Illusion/Shadow Call mirror the
  already-shipped `ground_summoner_slice_a_features` "ground the SLA triple, don't model the
  effect" split; Loremaster's Secret Lore pool mirrors the already-wired generic pool-choice
  mechanism (`push_generic_pool_choice_magnitude`).
- No `ClassId` enum entry or dedicated per-class file exists yet for any of the four prestige
  classes — the fix shape is 4 new `ground_or_block_<class>_class_features`-style dispatch
  functions (one per class, following `ground_swashbuckler_deeds`/`ground_summoner_slice_a_features`
  as the template), not 12 separate ad-hoc additions.
- **Recommendation for wave 43+: re-scope this 12-unit population OUT of "genuinely new-chassis"
  framing entirely** and treat it the same as the Paladin/Cleric precedented-add track — it is
  cheaper than Paladin/Cleric was, not harder, since most units have multiple precedents rather
  than one. Only Wizard's Arcane Bond (of the original 15) still holds up as genuinely open-ended
  new-chassis work requiring real subsystem modeling (weapon enhancement bonuses, no precedent
  anywhere in the engine).
- The 634-unit sub-mechanism-5 population is still completely unaudited under this scrutiny —
  four straight small-population checks (3 cheap-fix, 2 precedented-new-compute, 12
  precedented-new-compute) all landed away from "genuinely hard," so its own "too large to be
  worth auditing" framing from waves 37/38 should be treated as unverified, not confirmed, same
  as before.

**WAVE 43 UPDATE, 2026-09-05: all 12 of the 12-unit remainder named above are now CLOSED** —
Duelist's Canny Defense / Improved Reaction / Precise Strike / Elaborate Defense, Shadowdancer's
Shadow Illusion / Shadow Call / Shadow Jump / Summon Shadow, Assassin's Save against Poisons /
Death Attack, Loremaster's Lore / Secret Lore — via four new `ground_or_block_<class>_class_
features`-style dispatch functions (`ground_duelist_class_features`, `ground_shadowdancer_class_
features`, `ground_assassin_class_features`, `ground_loremaster_class_features`), each called
unconditionally from `compute_pilot_base_chassis` (no `ClassId`-family enum entry exists for any
of the four, confirmed directly), the same shape wave 42's `ground_paladin_detect_evil`
established. Two real corpus discrepancies between DESC prose and the computed token were found
and resolved by transcribing the literal token (Shadow Illusion's uses/day is a literal `1`, not
DESC's implied `floor(level/2)`; Shadow Jump's daily distance is a literal cumulative
`20/40/80/160`, not DESC's doubling `40/80/160/320`) — both follow the same authoritative-token-
over-DESC-prose ruling `warpriest_channel_energy_dc` already established for this bundle, not a
new precedent. Independently re-verified by the orchestrator (fresh `completion_atlas.py --check`
+ a direct id→status join over 49438 units, confirming exactly these 12 changed and nothing
else): `DONE: 25362→25369 (+7)`, `D: 2518→2506 (−12)`, `V: 322→327 (+5)` — not all 12 landed in
DONE; 5 landed in bucket V (`literal-verified`/`fixture-verified`, "verified by proxy, never by
the oracle"), the same D→V shape wave 41 hit for Monk's Stunning Fist, a legitimately-resolved
bucket rather than a lesser outcome. Both `cargo test --locked --lib` (3077 passed, up from 3068)
AND the full `cargo test --locked --no-fail-fast` integration suite were run this cycle — the
exact step wave 42's own cycle skipped, which is what let that wave's real regression through
undetected until its own wave-end gate. Full receipt:
`artifacts/bucket-d-mining/wave43_duelist_shadowdancer_assassin_loremaster_cycle_receipt.md`.

**Shape 2's new-chassis remainder after this cycle: 1 unit — Wizard's Arcane Bond** — the only
unit of the original 15-unit list that still holds up as genuinely open-ended new-chassis work
requiring real subsystem modeling (weapon enhancement bonuses, no precedent anywhere in the
engine). Every other unit named in this section across waves 39-43 is now closed.

**WAVE 44 UPDATE, 2026-09-05: a real script bug fixed (13 prestige classes, 191 units,
recovered into the entry-requirement registry) and 4 of the 5 wave-43-wave-end-gate-audit's
classifier-collision candidates closed** — `scripts/census_prestige_class_entry_requirements.py`'s
`extract()` keyed purely by display name across the full 158-book oracle
(`prestige_names.setdefault(name, path)`), so a filesystem-order race could let an older,
un-ingested predecessor book silently win over the real ingested book and drop the entry forever.
Fixed by ranking every candidate source by whether its own book is ingested BEFORE breaking ties
by relative path, proven with a new regression test that forces both walk orders against a
synthetic corpus reproducing the exact collision
(`scripts/tests/test_census_prestige_class_entry_requirements.py`, 4 tests). Re-ran against the
real pinned oracle: population `62 -> 74`. 12 of the 13 named classes recovered (Phrenic Slayer,
Thrallherd, Psychic Fist, War Mind, Elocater, Psion Uncarnate, Pyrokineticist, Metamind,
Cerebremancer, Pathfinder Savant, Student of War, Pathfinder Delver); **Gifted Blade was NOT
recovered** — it never actually carries a `TYPE:...Prestige` line anywhere in the oracle, the
audit's own 13-name list was one name too long. Diffed the regenerated fixture against its pre-fix
committed version: 144 insertions, 0 deletions — every pre-existing entry byte-identical. 3 of the
12 spot-checked directly against the real `.lst` source (Phrenic Slayer, Thrallherd, Cerebremancer)
— exact match.

**4 classifier collisions closed, 3 of the 4 audits corrected by direct corpus read before any code
was written:**

- **Item 1 (`power_over_undead_turn_undead`/`command_undead`) — audit's real-owner claim was
  WRONG.** Not Cleric: `cr_abilities_class.lst:2681`'s own `TYPE:WizardClassFeatures...` facet and
  `PowerOverUndeadLVL <- NecromancySchoolLVL <- WizardLVL` chain are Wizard-Necromancy-School-only,
  no `ClericLVL` anywhere. This is Wizard's Necromancy School arcane-school power (mimics Channel
  Energy mechanically, which misled the audit), not Cleric's own Channel Positive/Negative Energy —
  that remains a real, separate, still-open gap (`cleric_channel_positive_energy`/
  `cleric_channel_negative_energy`), explicitly untouched. Closed 5 facts (uses/day, Turn/Command
  DC, Command HD, Grave Touch, Life Sight) via `wizard_has_canonical_necromancy_selection`.
- **Item 2 (`order_of_the_dragon`) — audit's formula-shape caution was correct to raise, and
  checked out true.** `apg_abilities_class.lst:243`'s Survival bonus is the identical
  `max(1,CavalierLVL/2)` shape as Order of the Sword's own Sense Motive bonus, verified rather than
  assumed. Closed via `cavalier_order_of_the_dragon_survival_bonus`. **Also fixed, found during
  this wave's own review, not shipped as an oversight:** two diagnostic messages and the
  `cavalier_deferred_remainder_posture` helper unconditionally named only "Order of the Sword" and
  "the one canonical Order" — false prose for a character who recorded Order of the Dragon instead.
  Widened to name both Orders generically.
- **Item 3 (`padfe_construct`/`padfe_ooze`/`padfe_undead`) — audit's real-owner claim was WRONG.**
  Not Ranger: each PaDFE record's `%1` substitution is set ONLY by Pathfinder Delver's own
  Guardbreaker feature (`ag_abilities_class.lst:382`), gated to apply only when the character does
  NOT already have Ranger's real Favored Enemy of that type — no `RangerLVL` anywhere in the
  record's own token closure. Ranger's `choice:ranger_favored_enemy` recognizer is real and
  untouched; it was simply never the right attribution path for this record. Closed via
  `pathfinder_delver_padfe_bonus`/`ground_pathfinder_delver_class_features` (a fifth "no `ClassId`
  enum entry" prestige-class dispatch, same shape as wave 43's four). This item's own classify()-
  level reachability coverage was added this cycle (3 reachability + 1 negative-control test) to
  match the rigor the other three items already had.
- **Item 4 — split finding.** Spiritualist's Phantom Emotional Focus pool (7 records) closed via
  one new `push_generic_pool_choice_magnitude` call — the audit's "misrouted, not unmodelled"
  framing was half right (classifier routing was the bug), but no existing function named which
  focus was picked, so a small compute addition was genuinely needed too. **Summoner's Eidolon half
  NOT closed — audit's "already wired" claim was WRONG.**
  `eidolon_companion_progression_standard`'s real record is the First Worlder archetype's own
  master-linked progression trigger (`mastervar("FirstWorlderEidolon")`), not one of the base
  Eidolon facts `ground_summoner_eidolon` already grounds — confirmed by reading all 7 of that
  function's existing explanations directly. Widening the search found **15 sibling units**
  (`ultimate_magic`'s per-body-size/per-tier Eidolon evolution progressions) under the identical
  collision marker — a materially larger, genuinely harder population than the audit's single-unit
  framing suggested. Left named and unclosed for a future wave.

**Item 5 (also-check), Psychic Detective — NOT attempted, genuinely more involved than the
archetype-recognition shape alone.** Confirmed `VISIBLE:NO` (an Investigator archetype, matching
the audit), but the specific unit's own magnitude is a `STACK:YES`/`MULT:YES` Expanded Arcana
choice-pool slot gated at combined level >= 16 — needs Investigator's own archetype-substitution
handling checked first, not a simple owner-reroute. Left named and unclosed.

**Verification, independently re-derived by the orchestrator against a fresh `docs/work-
inventory.json` join, not just taken on the fixing agent's word:** exactly **16 units** changed
status, zero collateral movement (id-set unchanged at 49438) — `DONE: 25369→25375 (+6)`,
`B: 11769→11766 (−3)`, `D: 2506→2493 (−13)`, `V: 327→337 (+10)`. Not all 16 landed in DONE: 6
landed `grounded` (Order of the Dragon, all 5 Necromancy facts), 10 landed `literal-verified` (3
PaDFE + 7 Phantom Emotional Focus) — the same D/B→V shape waves 41/43 already hit. F1/`shape_
ledger.py` census re-derived: `5206 -> 5196`, verified per-id (Pathfinder Delver's 3 PaDFE records
plus all 7 Phantom Emotional Focus records are F1-shaped; Order of the Dragon and the 5 Necromancy
facts are not). Both `cargo test --locked --lib` (3090 passed, up from 3077) and the full `cargo
test --locked --no-fail-fast` integration suite were run this cycle.

Every citation this wave's own insertions shifted was re-derived, not just the new compute's own
tests — `scripts/completion_atlas.py`'s 10 bucket citations, `scripts/shape_engine_boundary.py`'s
promotion-ladder citation, and `scripts/missing_engine_tables.py`'s two engine-surface citations,
all stale purely from this wave's own line-number shift (the shape_engine_boundary/
missing_engine_tables ones were ALSO already stale at HEAD before this wave touched anything,
never caught because those two scripts' own tests are not wired into `verify.sh` — named as a
finding for a future wave, not fixed beyond re-deriving the pins this wave's own edits require).
Full receipt: `artifacts/bucket-d-mining/wave44_census_bug_and_classifier_collisions_cycle_
receipt.md`.

**What remains open after this wave:** Summoner Eidolon's 16-unit population (1 First Worlder
trigger + 15 Broodmaster multi-companion progressions, genuinely harder than the audit assumed);
Psychic Detective's Expanded Arcana choice-pool record (genuinely more involved than the
archetype-recognition shape alone); Cerebremancer's "Advance Manifesting" sub-cause (unexamined
this wave); sub-mechanism 5's remaining ~500 (of 699) units (un-re-audited since wave 43's own
wave-end-gate finding).

**WAVE 45 UPDATE, 2026-09-05: sub-mechanism 5's population re-derived fresh (686, not 634 or
699), cross-referenced against the now-74-entry prestige registry (598 registered / 88 not), and
Ultimate Psionics Phrenic Slayer's full 32-unit Favored Enemy remainder closed.**

**Fresh population re-derivation.** Neither wave 37/38's 634 nor wave 43/44's 699 held up:
querying `docs/work-inventory.json` directly for units whose evidence contains
`class_feature_of_unmodelled_corpus_class` at this cycle's own pre-edit HEAD (`4e96826b5e`)
returns **686** — both the registry fixture (wave 44's own census-script fix, 62→74 entries) and
the classifier itself have changed materially since either prior count was written, and this
population moves every time a classifier-collision item elsewhere in this section closes (it
shares evidence-string real estate with several of wave 44's own closures).

**Cross-referenced against `tests/fixtures/rules_core/prestige-class-entry-requirements.json`'s
74 entries** (extract each unit's evidence-suffix slug, check membership against the fixture's own
`class_id` set): **598 registered / 88 not registered**, summing exactly to 686. The 598
registered units are exactly the population `decisions.md`'s own standing ruling above already
names as "registered prestige class... only a per-feature magnitude formula is missing" — this
wave's own re-derivation confirms that framing still holds at the current, corrected population
size, not merely at wave 42's smaller 12-unit slice. The 88 not-registered units split cleanly:
`psychic_detective` (18) and `eidolon` (16) are the SAME two populations wave 44 already named
and left open (Expanded Arcana choice-pool, Summoner Eidolon/Broodmaster); `animal` (17),
`phantom` (9), `plant` (9), `undead` (8), `dragon` (8) are unrelated corpus name-collisions with
bestiary/pseudo-class records sharing a name (a Shaman spirit choice, a rogue-talent set, a
Ranger-archetype focus choice, a mixed Wizard/Cleric population, and a mixed Cavalier/Ultimate-
Magic population respectively — none of them prestige classes at all); `gifted_blade` (3) is
wave 44's own confirmed non-prestige exclusion. None of the 88 is a registered prestige class;
all are named, none attempted this wave.

**Closed this wave: Ultimate Psionics Phrenic Slayer's full Favored Enemy remainder, 32 of the
class's 43 sm5 units** (the base record plus all 31 creature-type sub-records) — the exact
"favored-enemy-style choice" shape this section's own WAVE 42 UPDATE named as precedented
(`ground_pathfinder_delver_class_features`'s PaDFE bonus, wave 44), needing only
`PhrenicSlayerLVL` (the class's own raw level, `up_classes.lst:932`) with no cross-class prime-
stat resolution. `SlayerFavoredEnemy = 2*floor((2+PhrenicSlayerLVL)/3)`
(`up_abilities_class.lst:1326`), shared identically by every one of the 31 creature-type display
sub-records (each carrying no own `DEFINE`/`BONUS` token, only an `ASPECT` referencing the base
record's own variable) — verified not just against the ingested corpus JSON but independently
cross-checked byte-for-byte against the real, non-ingested PCGen oracle
(`~/workspace/repos/pcgen/data/pathfinder/dreamscarred_press/ultimate_psionics/
up_abilities_class.lst:1326,1338-1368`), a stronger bar than prior waves in this section applied.
**A real miscount caught during this cycle's own investigation:** the creature-type population was
first (wrongly) counted as 30; a direct file-count and a second independent oracle cross-check
both confirm 31, caught by the new test module's own first assertion (RED for the right reason)
before any test was allowed to pass. `DONE: 25375→25407 (+32)`, `D: 2493→2461 (−32)`,
`V: 337 (unchanged — all 32 landed straight in DONE, none in V)` (independently re-derived,
before/after id-set unchanged at 49438, zero collateral movement — full accounting in
`artifacts/bucket-d-mining/wave45_registered_prestige_magnitude_formulas_cycle_receipt.md`). Both
`cargo test --locked --lib` (3095 passed, up from 3090) and the full `cargo test --locked
--no-fail-fast` integration suite were run this cycle. F1/`shape_ledger.py` census re-derived:
unchanged at 5196 (0 of the 32 closed units are F1-shaped — 31 are `F0`, 1 is `F5`, verified
per-id; a separate, older doneness instrument `shape_ledger.py` reuses,
`scripts/observer/pf1e_dashboard_producer.py`, treats `wiring_class` `display`/`derived` +
`grounded` as `HELD` not `DONE`, so this cycle's units correctly stay counted in that instrument's
own population even though `completion_atlas.py`'s SD-34 buckets correctly show them DONE — two
instruments drawing the line differently, not a defect).

**Phrenic Slayer's own remaining 11 units, named and not attempted:** Advance Astral Suit/Mind
Blade/Manifesting and their 4 two-and-three-way combinations key off cross-class variables
(`AegisCL`, `MndBladeLVL`, `ABILITYPOOL|Manifesting Level Advancement`) that depend on which
parent psionic class granted entry into the prestige class; Brain Nausea, Lucid Buffer, Power
Resistance, and Rebound Attack all key off `PhrenicSlayerPrimeStat` (a class-specific "prime
manifesting ability" fact, itself gated on the entry-class choice) — both are genuinely separate
subsystem-modeling questions from the raw-level-only Favored Enemy record this wave closes, left
named for a future wave rather than folded in speculatively.

**Sub-mechanism 5's remaining population after this wave: 654 (686 − 32)**, split **566
registered** (the highest-value remaining target — every one already benefits from the SAME
`chassis_supported(...) || prestige_class_entry_gate::is_registered(...)` generic grant-level
mechanism; only per-feature magnitude formulas remain, the identical shape this wave's own
Phrenic Slayer closure and waves 42-44's own precedent both demonstrate) and **88 not registered**
(named above by slug, out of scope). This population has now been fresh-re-derived and
cross-referenced for population make-up in three consecutive waves (43, 44, 45); the "genuinely
too large for a single wave" framing from waves 37/38 continues to not hold up under scrutiny —
every population checked so far resolves to precedented, closable small-compute work, not
open-ended new subsystem modeling, with the sole exception of Summoner Eidolon/Broodmaster and
Psychic Detective, both already separately named.

**WAVE 46 UPDATE, 2026-09-05: sub-mechanism-5 re-derived fresh a fourth consecutive time (654,
exactly matching wave 45's own post-cycle figure -- unlike every prior re-derivation in this
section, no drift found this time), and 20 units closed across seven registered prestige
classes.**

**Fresh re-derivation, unchanged from wave 45's close.** `654` total, split `566` registered / `88`
not registered, identical to wave 45's own post-cycle split — the registry fixture and classifier
have not moved since wave 45 closed. The 88 not-registered units are the same population wave 45
named by slug, re-confirmed unchanged.

**Population grouped by owning prestige class: 58 distinct classes** among the 566 registered
units, from 45 (Divine Scion) down to 1 (Sentinel). Phrenic Slayer's own remaining 11 units stay
explicitly out of scope (cross-class prime-stat/parent-entry dependency, unchanged from wave 45).
**A real, useful negative finding from this wave's own scan:** at least 9 Ultimate Psionics
prestige classes (Sighted Seeker, Thrallherd, Psion Uncarnate, Cerebremancer, Metamind, Elocater,
Psicrystal Imprinter, Soul Archer, Metaforge) carry the IDENTICAL `AS`/`MB`/`MBAS`/`Ma`/`MaAS`/
`MaMB`/`MaMBAS` cross-class-manifester-level shape Phrenic Slayer's own excluded 11 units already
carry — confirmed by direct read, not assumed from the naming pattern alone. This is the same
underlying subsystem-modeling question repeated across at least 10 classes now, not 10 separate
questions; a future wave scoping real work against it should treat it as one question.

**Closed this wave: 20 units across seven prestige classes**, every formula read directly from
its own corpus record (three of the seven needed the class's own level-table file too, not just
the class_feature record's own tokens — the same cross-file idiom wave 44 established for
Pathfinder Delver's Guardbreaker), independently cross-checked against the real, non-ingested
PCGen oracle:
- **Pathfinder Delver** (extends wave 44's dispatch, no new `ClassId`): Guardbreaker's own record,
  Master Explorer, Thrilling Escape, Vigilant Combatant, Fortunate Soul, True Seeing — 6 units, all
  self-contained on the class's own raw level.
- **Argent Dramaturge**: Argent Performance (rounds + save DC, the "10 + level + ability modifier"
  idiom) and Dramaturgical Flourish (pool size) — 2 units.
- **Horizon Walker**: Favored Terrain, Terrain Mastery, Terrain Dominance — 3 pool-size-only units.
- **Nature Warden**: Companion Bond, Survivalist — 2 raw-level-tracking units (Woodforging, the
  class's third open unit, carries no magnitude token at all and is left unattempted).
- **Rage Prophet**: Rage Prophet Mystery, Ragecaster — 2 raw-level-tracking units (Spirit Warrior,
  the class's third open unit, same no-magnitude-token shape as Woodforging, left unattempted).
- **Holy Vindicator**: Stigmata — 1 unit (Channel Smite, a bonus-feat grant with no magnitude
  token, left unattempted).
- **Stalwart Defender**: AC Bonus, Damage Reduction, Defensive Powers, Defensive Stance — 4 units
  (Increased Damage Reduction, a pool-member magnitude needing untracked selection state, and
  Renewed Defense, dice notation this engine does not parse, both left unattempted).

Every new formula was written against real pure-formula tests plus a real-pipeline reachability
test (26 new lib tests, 15 new bin-level tests), `DONE: 25407→25419 (+12)`,
`D: 2461→2441 (−20)`, `V: 337→345 (+8)` — not all 20 landed in DONE; 8 landed in bucket V
(`literal-verified`, "verified by proxy, never by the oracle"), the same D→V shape waves
41/43/44 already hit, a legitimately-resolved bucket rather than a lesser outcome — independently
re-derived by the orchestrator against a fresh `completion_atlas.py --check` + direct id→status
join over 49438 units, confirming exactly these 20 changed and nothing else. Both `cargo test
--locked --lib` (3121 passed, up from 3095) and the full `cargo test --locked --no-fail-fast`
integration suite were run this cycle (run to completion twice against the fully-settled tree,
identically 8545 passed / 0 failed / 67 ignored both times). F1/`shape_ledger.py` census
re-derived: `5196 → 5193`, verified per-id (3 of the 20 closed units are F1-shaped — Nature
Warden's Companion Bond and Pathfinder Delver's Thrilling Escape/Fortunate Soul; the other 17 are
F0/F2/F4/F5), `formula_interpreter_corpus_wide.rs`'s own pinned census test updated to match. Full
receipt: `artifacts/bucket-d-mining/wave46_registered_prestige_magnitude_formulas_cycle_receipt.md`.

**Sub-mechanism 5's remaining population after this wave: 634 (654 − 20)**, split **546
registered** (across 55 remaining prestige classes; at least 9 confirmed AS/MB/Ma-shaped classes
are genuinely harder, same bucket as Phrenic Slayer's own remaining 11; several large
heterogeneous classes like Divine Scion (45 units) remain real but slower closable work) and **88
not registered** (unchanged, named above).

**WAVE 47 UPDATE, 2026-09-06: a stalled prior build agent's Divine Scion work was recovered,
ONE real correctness bug found and fixed, and 43 of Divine Scion's 45 sub-mechanism-5 units
closed.**

**Recovery context.** This wave's own dispatched build agent (`sd34-wave47.workflow.js`) ran for
~48 minutes and wrote real, substantial code targeting Divine Scion (source book
`inner_sea_magic`, 45 sub-mechanism-5 units, the single largest remaining class wave 46 named) —
a `probe_divine_scion_wiring` function and `divine_scion_wired` `EngineFacts` field in
`v06_work_inventory.rs`, and a `ground_divine_scion_class_features` implementation plus a full
test module in `pilot_compute/mod.rs` — then stalled without committing, writing a receipt, or
running any verification. This cycle read the FULL uncommitted diff against the real corpus
records directly (never trusting the diff's own comments or the prior agent's characterization of
its own work, the same distrust this bundle applies to every prior wave's own self-report), found
one genuine correctness bug, fixed it, finished the wiring, verified/rewrote the tests, and
completed every verification step the stalled agent never ran.

**The bug, found by reading the real oracle rather than trusting the recovered draft's own doc
comments.** The recovered draft's `ground_divine_scion_class_features` ground all 35 per-domain
Domain Specialization sub-records AND all 4 Opposition Alignment DR records unconditionally, for
every Divine Scion character simultaneously, regardless of which domain or opposition alignment
the character actually has. Both are real `ABILITYPOOL`-gated one-of-N choices — confirmed
against the real, non-ingested PCGen oracle
(`~/workspace/repos/pcgen/data/pathfinder/paizo/campaign_setting/inner_sea_magic/
ism_classes.lst:103`/`:104`'s own `BONUS:ABILITYPOOL|Opposition Alignment|1` / `BONUS:ABILITYPOOL|
Domain Specialization|1`, pool SIZE 1, and `ism_abilities_class.lst`'s own section headers at
lines 35 and 47, reading literally `# Opposition Alignment choices` and `# Domain Specialization
choices`) — the exact shape this codebase already gates everywhere else via a real
`choice_selection(input, CHOICE_ID)` check (`SORCERER_BLOODLINE_CHOICE_ID`,
`CLERIC_DOMAIN_CHOICE_ID`, `BLOODRAGER_BLOODLINE_CHOICE_ID`, `RANGER_COMBAT_STYLE_CHOICE_ID`, and
roughly 30 other call sites). The recovered draft's OWN doc comment correctly excluded True Scion
Charisma/Wisdom citing this exact reasoning ("a genuine pool-selection-state question this engine
does not yet track") — but did not notice its own two siblings, Domain Specialization and
Opposition Alignment, carried the identical `ABILITYPOOL` shape, and ground them unconditionally
instead. This is exactly the class of defect this bundle's own standing doctrine (`§2a`'s "a shape
engine computes a number; it does not complete a record", and the ruling this file opened under
-- "a wrong computed number looks like a right one") exists to catch: a computed value that looks
right (a real formula, a real corpus citation, real passing tests) but is factually wrong for any
real character querying it.

**The fix.** Two new choice-set-id consts
(`DIVINE_SCION_DOMAIN_SPECIALIZATION_CHOICE_ID`/`DIVINE_SCION_OPPOSITION_ALIGNMENT_CHOICE_ID`),
following the `"domain:<slug>"`/`"alignment:<slug>"` selection-id convention already established
elsewhere. `ground_divine_scion_class_features` now gates the per-domain block and the
per-alignment DR block on `choice_selection(input, <CHOICE_ID>)` matching the recorded selection —
a real character's own receipt now surfaces exactly the ONE domain and ONE alignment it actually
recorded, never all 39 (35 + 4) simultaneously. The 4 genuinely unconditional single-owner facts
(Domain Specialization's own pool-SIZE, Divine Wrath, Deific Defense, Weapon and Armor
Proficiency) are untouched. `probe_divine_scion_wiring` (used both by the corpus-wide census
sweep and this wave's own reachability tests) was rewritten to sweep every one of the 35 domain
selections and 4 alignment selections in turn, collecting the union of corpus keys the real
pipeline resolves — the same `probe_cleric_domain_generic_member_wiring` idiom this file already
uses for Cleric Domain/Sorcerer Bloodline, adapted for a hand-rolled (not generic-pool-group)
grounding function. `canonical_seeds_for` gained a `"divine_scion"` arm (`domain:fire` /
`alignment:evil`), the same "give the sweep one canonical default choice" convention already used
for wizard/cleric/sorcerer/fighter/psychic and others.

**Net effect on this wave's own closure count: none** — all 43 corpus keys the recovered draft
targeted still resolve reachable, because reachability at the corpus-wide census level means "the
real pipeline resolves this key for SOME real character configuration," not "every character has
it," the same existence-based semantics Cleric Domain's own ~9-domain population already
established (confirmed directly: none of Cleric Domain's own per-domain records are grounded
unconditionally for every cleric either). What changed is CORRECTNESS: a real Divine Scion
character's own receipt now shows exactly the domain/alignment they recorded, not a fabricated
39-facts-at-once answer.

**Closed this wave: 43 of Divine Scion's 45 sub-mechanism-5 units** — Domain Specialization's own
pool-size record, Divine Wrath, Deific Defense, Weapon and Armor Proficiency, all four Opposition
Alignment DR records (Chaotic/Evil/Good/Lawful), and all 35 per-domain Domain Specialization
sub-records (Air through Weather) — every formula verified directly against the real corpus JSON
and independently cross-checked against the real, non-ingested PCGen oracle; all 35 domains' own
uses-per-day tokens were cross-checked EXHAUSTIVELY (not sampled) against the raw `.lst` file and
matched exactly. **True Scion Charisma/Wisdom (2 units) remain named, not attempted**, unchanged
from the recovered draft's own honest scoping: a real `ABILITYPOOL|True Scion|1` mutually-exclusive
choice between an ability-score bump to Charisma or Wisdom, each also re-stating the same
`DomainSpecBonus`/`DivineWrathBonus`/`DeificDefenseBonus` increments already excluded above — a
genuine pool-selection-state question this engine does not yet track for this specific choice.

Independently re-derived by the orchestrator against a fresh `docs/work-inventory.json` join, not
just taken on the fixing agent's word: exactly **43 units** changed status, zero collateral
movement (id-set unchanged at 49438) — `DONE: 25419→25458 (+39)`, `D: 2441→2398 (−43)`,
`V: 345→349 (+4)`. 39 landed `grounded` (all 35 per-domain records + all 4 Opposition Alignment
records, every one `wiring_class: computed`); 4 landed `literal-verified` (the 4 unconditional
records, every one `wiring_class: static` and swept-verified) — the same D→V shape waves
41/43/44/45/46 already hit. F1/`shape_ledger.py` census re-derived: `5193 → 5155`, verified
per-id (38 of the 43 closed units are F1-shaped; the other 5 are 4 `F0` + 1 `F8`). Both
`cargo test --locked --lib` (3134 passed, up from 3121) and the full `cargo test --locked
--no-fail-fast` integration suite were run this cycle (run to completion twice against the fully-
settled tree — the F1 pin update is a real `.rs` edit — identically 8562 passed / 0 failed / 67
ignored both times). Full receipt:
`artifacts/bucket-d-mining/wave47_registered_prestige_magnitude_formulas_cycle_receipt.md`.

**A generalizable finding for future waves against this same sub-mechanism-5 population:** before
grounding ANY corpus record shaped `# <X> choices` / granted via `BONUS:ABILITYPOOL|<X>|1`, check
whether the class's OTHER already-excluded units (like True Scion here) share the identical
`ABILITYPOOL` shape — a wave that correctly excludes one such choice but grounds a sibling choice
unconditionally is the exact defect this cycle's own correction fixed. Grep the raw oracle's own
section-header comments (`# ... choices`) as a cheap first signal before scoping a future wave.

**Sub-mechanism 5's remaining population after this wave: 591 (634 − 43)**, split across the
remaining 54 registered prestige classes (Divine Scion no longer among the large ones — only its
own 2-unit True Scion remainder is left) and the 88 not-registered units (unchanged, named
above). The ≥10-class AS/MB/Ma cross-class-manifester-level population is unchanged, untouched
this wave.

---
