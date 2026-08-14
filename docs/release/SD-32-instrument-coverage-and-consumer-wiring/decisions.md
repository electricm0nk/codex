---
canonical: true
owner: god-emporer
status: planning-ready (2026-08-13)
date: 2026-08-13
companion_to: ./scope-draft.md
---

# SD-32 Decisions

## Decision 1 — The anti-gaming rule (LOAD-BEARING, VERBATIM)

**Status:** Binding on every cycle in this bundle, from launch, without
exception. Reproduced verbatim from the execution brief that authorised this
package (operator directive, 2026-08-13). This is the constraint the bundle
exists under; every other decision in this file is subordinate to it.

> **THE ONE RULE THAT OVERRIDES EVERYTHING ELSE: YOU MAY NOT MOVE A NUMBER BY LOWERING THE BAR.**
>
> The operator's directive is "improve our numbers, assuming the measuring systems are accurate." That
> second clause is a constraint, not a licence: the instruments are to be trusted and EXTENDED, never
> tuned to flatter the result. Every one of the following is forbidden, and doing any of them makes
> this work worse than not doing it:
>
> - Reclassifying a unit into an easier wiring_class so it clears a lower bar.
> - Loosening, skipping, #[ignore]-ing or special-casing a check so more units pass.
> - Marking a unit done on evidence weaker than its class actually requires.
> - Counting 'held' as done. SD-29 decisions.md §46.4 deliberately does NOT count it, and the
>   doneness_meaning text says so explicitly: "As done as the current instruments can prove, and
>   deliberately not counted as done."
> - Widening a bucket definition, or editing doneness_meaning, to make a bucket look better.
> - Ingesting fixture data, or hand-authoring rules data, to satisfy a check.
>
> This program has spent three days learning that a green instrument over an empty screen is worse
> than a red one. A number that moved because the bar moved is a lie told to the operator in the one
> artifact they use to judge progress. If a unit cannot legitimately reach its bar, LEAVE IT and say
> why. Reporting "fewer moved than hoped, honestly" is a success. If you ever find yourself editing a
> threshold, a classifier, or a definition to make a count rise, STOP and report it instead.

**Operative consequences for this bundle:**

1. Every acceptance criterion in `epic-breakdown.md` and
   `acceptance-and-verification.md` is phrased as **"units legitimately reach
   their existing bar"** — never "the count rises." A criterion that names a
   target count is malformed and must be rewritten before the cycle proceeds.
2. A cycle receipt reports units moved **and** units examined-and-left-alone,
   with the reason for each class of the latter. The second number is evidence
   of compliance, not of failure.
3. A cycle that ends with fewer units moved than its epic's ceiling, and a
   correct account of why, is **COMPLETE**, not `BLOCKED`.
4. `held` is never reported as `done`, never aggregated with `done`, and never
   described as "effectively done" in any receipt, progress entry or release note.

## Decision 2 — The verdict table has no `done` rung for `static`/`derived` (MEASUREMENT GATE)

**Status:** ANSWERED (2026-08-14). Gated epics E5 and E6; both now unblocked as
SD-30 scope (SD-32 folded into SD-30 per SD-30 `decisions.md §43`). See the
answer block below.

**Finding, re-derived.** The producer's `doneness_verdict()` table maps
`static` and `derived` to `held` for **every** status it accepts
(`ingested-magnitude`, `grounded`, `text-complete`). Only two cells in the whole
table produce `done`: `display`+`text-complete` and `computed`+`grounded`.
7,479 held units are `static` or `derived`
(`artifacts/derive-movable-mass.py`, "static/derived held by kind").

**Consequence.** The corpus-literal byte-equality sweep and the
evaluator-vs-fixture check — the two instruments the dashboard's own
`doneness_meaning` names as the missing checks for `static` and `derived` — can
be built exactly to specification, run green over the whole corpus, and move
**zero** units on the board. There is no rung for their result to land on.

**What would be needed.** Two coordinated changes, neither of them inside this
bundle's write scope:

1. The generator (`src/bin/v06_work_inventory.rs`) emits a **new, strictly
   stronger status word** for a unit whose corpus literal was byte-compared, or
   whose evaluator was checked against a fixture, and passed — e.g.
   `literal-verified` / `fixture-verified`. It is a new word precisely so it
   cannot be confused with `grounded`, which means something else.
2. The producer's verdict table gains a rule mapping `static`/`derived` + that
   word to `done`, and `doneness_meaning` gains the sentence describing it.

**Why this is not a §1 violation, and why a cycle still may not do it.** Adding
a rung **above** `held`, reachable only by evidence that does not exist today,
is the opposite of lowering a bar: it raises the ceiling for units that clear a
*new and stricter* check. Nothing already `held` becomes `done` without that new
evidence. But it is still a change to the artifact the operator uses to judge
progress, made by the party being judged — so it is requested, in writing, with
this reasoning, and the operator or dashboard owner makes it. **A cycle that
edits the producer to do this has violated `§1` regardless of the reasoning
above.**

**Until this is answered:** E5 and E6 stay `BLOCKED` on the kanban. Their work
is real and their instruments are worth building on the merits — but this
package does not pretend they move the number.

**Answer (operator, 2026-08-14, launch session).** The operator ratified the
`literal-verified`/`fixture-verified` rung proposed above — and its verdict-table
mapping (`static`/`derived` + that word → `done`) — retroactively covering the
rung's actual landing during the SD-29 → SD-30 handoff, and brought SD-32's
remaining scope formally into SD-30 (this decision is recorded here, in the
SD-32 package, because it answers an SD-32-numbered decision; the operative
consequences are also recorded in SD-30 `decisions.md §49`). E5
(`e5-static-sweep`) and E6 (`e6-derived-check`) are unblocked.

The operator's words, verbatim:

> I would rather bring SD-32 into the SD-30 scope. I think the gist of what I was
> saying with "done rung for static and derived" was basically that some things do
> not require computation. If a fireball is 1d6 per spellcaster level - you don't
> need to compute 6d6 for a 6th level caster - you need to display that the
> fireball spell is 6d6 because the character in question is 6th level. That's
> just printed in the character sheet. The actual rolling of 6d6 happens on a
> table, with dice, and the additions are added by the player's brain. Our goal
> here is to print a character sheet that the user can use at the table - we are
> not making a video game. So in many cases we just need to expose the end rule -
> once we can do that it's done. If a spell says 1d6 per character level, you just
> need to be able to determine the character level and say the true value when the
> character sheet is created.

**Reasoning, as ratified.** The product bar is a table-usable character sheet,
not a simulation engine. `done` for a `static` or `derived` unit means the sheet
exposes the end rule with a true resolved value for *this* character — the
parameters (e.g. caster level) are resolved and the rule is displayed correctly
(e.g. "6d6"); the rolling of the dice and the arithmetic of applying the result
happen at the table, in the player's hands, and are explicitly out of scope for
the engine to simulate. This is the same reasoning already ratified for
zero-magnitude `display` text features (`v06-...` standing ruling) extended to
`static`/`derived` numeric rules: a correctly resolved and displayed formula is
as complete as a computed one, for this product.

**The constraint that stands, unrelaxed.** Only units that genuinely pass the
byte-equality (`static`) or fixture-verification (`derived`) check earn the new
status word and the `done` verdict; a unit that fails either check stays `held`
and is reported, not stamped. Decision §1's anti-gaming rule — no reclassifying,
no loosening a check, no counting `held` as done, no editing a bar to move a
number — is untouched by this answer; the rung was added because it was earned
on the merits (per the "why this is not a §1 violation" reasoning above), not to
relax anything.

## Decision 3 — The wiring-class classifier is accepted on accuracy, not on movement

**Status:** Binding on epic E4.

**Decision.** The classifier that resolves `ambiguous` (360 units) and
re-examines `display`+`grounded` (1,416 units) is accepted or rejected on
**agreement with a hand-labelled sample**, and on nothing else.

1. **E4-F1 runs first and is a gate.** A sample of at least 100 units, stratified
   across the five wiring classes and across at least four kinds, is
   hand-labelled from the corpus record — the whole record, not a field-filtered
   grep — **before** the classifier is written. The labels are committed. The
   labeller records the token evidence for each label.
2. The classifier's acceptance criterion is its **agreement rate against that
   held-out sample**, reported per class and per kind, plus its full confusion
   matrix. There is no target count of units moved anywhere in E4's acceptance.
3. **Movement is reported in both directions.** A classifier that reclassifies
   180 units into `computed` and 400 units out of `computed` into `static`
   reports both, and its net effect on `done` may be **negative**. That is a
   **passing** outcome. A classifier that only ever moves units toward the two
   `done`-producing cells is presumptively wrong and must be re-examined before
   its output is accepted.
4. If E4-F1's sample shows the current classifier is substantially correct and
   the `display`+`grounded` contradiction is real but rare, E4-F2 is **not
   dispatched**, E4 closes at F1, and the 1,776 units are reported as
   "examined, correctly classified, left alone." That is `COMPLETE`.

**Rationale.** This lever is ranked #2 by ceiling and #1 by gaming risk. Under
§1's first forbidden item — "reclassifying a unit into an easier wiring_class so
it clears a lower bar" — a classifier is exactly the instrument that could do
that at scale while looking principled. The defence is that the classifier is
judged against ground truth established *before* anyone knows which way it moves
the count.

## Decision 4 — Probe coverage extension is a coverage change, not a bar change

**Status:** Binding on epic E2.

**Decision.** `probe_equipment_effect_wiring()` currently builds its key set
from four compiled equipment tables (`crb`, `apg`, `acg`, `beastiary1`) and
loads corpus from six `OBSERVABLE_BOOK_DIRS`; eleven books have a compiled
`equipment_tables.rs`. E2 widens both to the full compiled set.

**`equipment_key_is_wired()` is not touched.** The predicate — equip this item
alone, against the real corpus, and observe at least one non-`None` mechanical
stat effect from `compute_equipment_effects` — stays byte-identical. Items the
widened probe examines and finds inert stay `ingested-magnitude`, correctly, and
E2's receipt reports how many did.

**Why this is §1-compliant.** The bar is unchanged; it is applied to units it
never previously examined. Widening the population a fixed test runs over is
coverage. Weakening the test so more of the existing population passes is
gaming. E2 does the first and its diff must show `equipment_key_is_wired`'s body
unmodified.

## Decision 5 — `spell` is reported as structurally blocked and is not worked for the numbers

**Status:** **[SUPERSEDED 2026-08-13]** — both of its evidential legs were
falsified, the second one by measurement. Retained verbatim below rather than
edited, because the shape of the error is the lesson. See
`progress.md` -> `Cycle — ground-spell-units` for the derivation and
`docs/retro/events/probe-spell-ground.jsonl` for the correction event.

**Leg 1 — "`classify()`'s `Kind::Spell` arm cannot return `grounded`".** True
when written, stale by 2026-08-07: `epic-31-spell-wiring` wired
`spellbook::compute_spellbook_coverage` into
`pf1_adapter::resolve_unified_pilot_snapshot`, so a spell's own level now
reaches `CharacterSheet.tsx`'s `DC {entry.dc}` cell. Corrected by the
`spell-consumer-delta-probe` cycle (`aafd492c`).

**Leg 2 — "would move 178 units from `held` to `in-progress` ... and none to
`done`".** Wrong, and it modelled a different mechanism. **178** is exactly the
count of `spell` units the producer's `NO_GROUNDING_PROBE` cap holds down —
uncapped verdict `in-progress`, capped verdict `held` (162 `computed` + 16
`display`, all **non-`grounded`**). Only removing `spell` from that tuple — a
*producer* edit this bundle forbids itself (`§1`, `§6`) — produces the
`held` -> `in-progress` transition. **Grounding is the opposite operation and
never passes through that cell**, because the cap fires only on units that are
not `grounded`, and `computed` + `grounded` is a `done` cell outright.

Measured over the real before/after inventories with the verdict table
transcribed in `artifacts/derive-movable-mass.py`:
`done` 3,418 -> 3,464 (**+46**), `held` 9,501 -> 9,455 (**−46**),
`in-progress` 716 -> 716 (**+0**), units moved into a worse bucket: **0**.

**What replaces it.** `spell` is not bucket C. 623 CRB spell units are
legitimately `grounded`; 46 of them reach `done`. The honest ceiling for
further `done` movement on `spell` is **113 units** (the `computed`-class
remainder), not 637 and not 1,281 — and reaching them needs engine work
(`spellbook::casting_ability_for_class` mapping more casting classes), not a
looser probe. `forward-scope-register.md F2` is updated accordingly.

---

**Original text, retained verbatim:**

**Status:** Binding on the whole bundle.

**Decision.** All 1,281 held `spell` units are reported as bucket C,
structurally unreachable, with the reason on the record. No epic in this bundle
attempts to move them.

**Evidence.** `spell` `grounded` corpus-wide is **0**, by construction:
`classify()`'s `Kind::Spell` arm cannot return `grounded` — "no currently-wired
consumer reads a spell's magnitude, so every resolved-level spell stays
`ingested-magnitude`." A spell's only `done` cell is `display`+`text-complete`,
occupied by exactly one spell.

Building a spell consumer-delta probe would move 178 units from `held` to
`in-progress` — a *worse*-looking bucket — and none to `done`, because reaching
`done` needs a real consumer that reads a spell magnitude, i.e. spellcasting as
product work. That is a product decision, recorded at
`forward-scope-register.md F2`, not a numbers lever.

## Decision 6 — `companion`'s `NO_GROUNDING_PROBE` listing is stale; report it, do not act on it

**Status:** Reported to the dashboard owner. No code change in this bundle.

**Finding.** The producer lists `companion` in `NO_GROUNDING_PROBE` on the
stated grounds that "`companion` and `spell` alone read `grounded: 0`." The
current payload carries **922 grounded companion units**, 416 of them
`computed`+`grounded` and already counted `done`. The justification is stale.

**But the cap moves zero companion units.** The cap only fires on units that
would otherwise be `in-progress` (`computed`/`display` + non-`grounded`), and
the corpus has none for `companion`: all 506 held companions are `derived` (270),
`display` (215), `static` (19) or `ambiguous` (2). Removing `companion` from the
tuple would change the board by 0.

**Decision.** Report it (`forward-scope-register.md F5`); change nothing.
Correcting a producer constant that moves 0 units is still a producer edit by
the party being measured, and this bundle does not make those. Recorded here so
that a later reader does not re-derive it as a new finding.

## Decision 7 — Scope is instrument coverage and consumer wiring, not content ingestion

**Status:** Binding.

`not-started` (21,303 units) is content that is not in the engine. Moving it is
book-ingestion work and belongs to the SD-29/SD-30 lanes, not here. This bundle
touches only units the engine already holds.

## Decision 8 — Every figure ships with the command that produced it

**Status:** Binding on every cycle receipt in this bundle.

Per `AGENTS.md` ("A number in a brief ships with the command that produced it,
or it does not ship — not the value, the invocation"), and because transcribed
figures are this program's rank-one recorded defect class: every count in a
receipt, a progress entry or a release note carries its invocation.
`artifacts/derive-movable-mass.py` is the canonical one for movable-mass
figures, and it self-validates against the live payload before printing.

## Decision 9 — The equipment probe asks its question of the whole engine catalog, book-scoped

**Status:** Landed. `src/bin/v06_work_inventory.rs`.

**The defect.** `probe_equipment_effect_wiring()` built its key universe from
four hand-maintained `.extend()` calls over `crb`/`apg`/`acg`/`beastiary1`'s
compiled equipment tables, while `classify()` decides `known` from
`facts.equipment_keys`, which SD-28-E15 had already rebuilt to derive from
`equipment_resolver::equipment_catalog_rows()`. Two lists of the same fact,
never reconciled — Decision 36's pattern, one function over from where it had
already been fixed once. **3,123 of the catalog's 6,395 keys had never been
asked the wiring question at all**, so their units could only ever report
`equipment_table_entry_with_corpus_magnitude`, not because the engine computes
nothing from them but because nobody asked.

Command:

```
CARGO_TARGET_DIR=... cargo test --bin v06_work_inventory \
  the_probe_key_universe -- --nocapture
# probe key universe: 6395 keys; four compiled tables alone: 3272 keys;
# previously unexamined: 3123
```

**The fix, and what it is NOT.** The universe is now derived from
`equipment_catalog_rows()` (`probe_equipment_key_universe`), pinned by
`the_probe_examines_every_key_the_engine_catalog_holds`. **The bar is
untouched**: `equipment_key_is_wired` — resolve against the real on-disk
corpus, produce at least one non-`None` mechanical stat effect — is
byte-for-byte what it was. This widens what is ASKED, never what counts as an
answer.

**And it raised the bar in one place.** Widening the universe immediately
surfaced a name-coincidence defect the flat key set had been hiding: six
Ultimate Equipment units grounded off ARG/CRB corpus rows, although
`data/corpus/ultimate_equipment` does not exist at all. `Celestial Shield` is
the proof that a shared key is not a shared item — ARG's
(`arg_equip_arms_armor.lst:22`) is a **heavy** shield, 13,170 gp, `ACCHECK:0`,
`SPELLFAILURE:0`; UE's (`ue_equip_arms_armor.lst:126`) is a **light** shield,
4,020 gp, `ACCHECK:-1`, `SPELLFAILURE:5`. This is `race_trait`'s SD-28 §56
defect (`modelled_race_of_race_trait`) reproduced one kind over. The probe is
now book-scoped: each book's catalog keys resolve against **that book's own
corpus, loaded alone**, and the result is keyed `(engine_book, key)`, which
`classify()` matches as a pair. A book with a catalog but no corpus directory
now gets no probe coverage at all — the honest result. Pinned by
`a_key_two_books_share_grounds_only_the_book_whose_corpus_was_read`.

**What moved.** Re-derived, not transcribed; both inventories generated by
`cargo run --release --bin v06_work_inventory`, verdict table applied by
`artifacts/derive-movable-mass.py`'s transcription (self-validated against the
live payload in the same session):

| | before (`2026-08-13T09:33:16Z`) | after (`2026-08-13T15:53:13Z`) | Δ |
|---|---|---|---|
| `done` | 3,426 | 3,444 | **+18** |
| `held` | 9,475 | 9,475 | 0 |
| `in-progress` | 734 | **716** | **−18** |
| `grounded` (all units) | 4,699 | 4,726 | +27 |
| `equipment` grounded | 133 | 145 | +12 |
| `equipment_modifier` grounded | 40 | 55 | +15 |

**Zero units moved down the ladder and zero units changed `wiring_class`.**
27 units reached `grounded`; 18 of them are `computed` and therefore reach
`done`, the other 9 are `static`/`derived` and stay `held`, exactly as the
verdict table requires. 15 of the 18 are Pathfinder Unchained's Automatic
Bonus Progression weapon/ammunition enhancement modifiers; 3 are ARG magic
items.

## Decision 10 — 716 of the 734 `in-progress` units are NOT reachable with an instrument that exists

**Status:** Binding finding. This corrects the scope card's premise.

The `in-progress` bucket's own definition promises "the bar is reachable with
an instrument that exists". For 716 of the original 734 that promise is false,
and the reasons are structural rather than a matter of effort. Re-derived from
the regenerated inventory and the same on-disk `data/corpus/` records the probe
itself reads (`artifacts/why-in-progress-equipment-stalls.py`):

| units | why it cannot clear its bar |
|---|---|
| 295 | **No `data/corpus/<book>/equipment` directory exists at all.** UE (190), UPsi (82), UC (22), UI (1) have compiled catalogs but no ingested corpus, so there is no record for any probe to read. Blocked on ingestion, which Decision 7 puts out of this bundle's scope. |
| 239 | **The record resolves and carries neither a readable token (`MAXDEX`/`SPELLFAILURE`/`ACCHECK`) nor any bonus chain at all.** Its `magnitude_token_count` comes from `COST`/`WT`-shaped tokens. There is no magnitude for a consumer-delta probe to observe; a probe that promoted these would be measuring nothing. |
| 136 | **The record carries a bonus chain in a family `equipment_effects.rs` does not read** — `BONUS:VAR` (130 chains), `ITEMCOST` (47), `EQMARMOR` (17), `EQM` (8), `SAVE` (7), `WEAPONPROF` (20 across four weapon types), and a tail. These need **new consumer wiring in the engine**, not a new instrument. |
| 38 | **The record carries a shape the effect model *does* read, and still no delta was observed.** The nearest-miss population, and the most interesting one. 24 are `BONUS:COMBAT\|AC\|<n>\|TYPE=ArmorEnhancement` / `TYPE=ShieldEnhancement` rows (CRB `Special Ability ~ +1..+5 ~ Armor/Shield`, PU's `ABP ~ +N Attunement ~ Armor/Shield`), which `arms_armor::armor_class_bonus_from_bonus_chains` deliberately does not match — it accepts only `TYPE=Armor`/`TYPE=Shield`, the base item's own AC, never a modifier's enhancement on top. The rest are `%CHOICE`-parameterised chains (`BONUS:COMBAT\|AC\|%CHOICE\|TYPE=DEFLECTION`, `BONUS:STAT\|%CHOICE`) whose magnitude is not a literal at all, and weapon `TOHIT`/`DAMAGE` chains gated on `PREVARGT`/`PREEQUIPBOTH` conditions the standalone probe cannot satisfy. |
| 7 | Book has a corpus, but no record under this unit's key or name. |

Those four rows sum to 715; the 716th `in-progress` unit is a single
`feat`/`computed`/`text-complete` record, a different kind on a different
probe.

The honest total that this card could move with the instrument that exists was
**18, not 734**. The remaining 716 are recorded rather than forced.

**The named next lever, with its size.** The 38-unit row is the cheapest real
lever left and the 136-unit row the next: together 174 units that are neither
an ingestion gap nor an absence of magnitude. Both are
`forward-scope-register F8` work — teaching a consumer to read a magnitude the
record genuinely carries — and both are engine wiring against a live twin, not
instrument work.

**Why the 38 were not simply taken here, even though the shape is already
readable.** Two reasons, and both are the same rule.

1. `TYPE=ArmorEnhancement` is a *different bonus type* from `TYPE=Armor` in
   PF1e, and it stacks on top of the base armor bonus rather than replacing
   it. Making `armor_class_bonus_from_bonus_chains` accept it changes a
   character's computed AC. That is a product behaviour change with parity
   surface, owed a fixture and an oracle comparison, not a one-line widening
   slipped in under an instrument card.
2. These are equipment **modifiers**. The probe equips each key standalone with
   `applied_modifiers: []`, which is the right question for an item and the
   wrong question for a modifier — a modifier's delta only exists relative to a
   host item. Reading its chain while it is equipped alone would report a
   number no player can ever see.

Widening the probe to "observe" either of these without the consumer work is
precisely the green-instrument-over-an-empty-screen failure this bundle exists
to avoid, so neither was done.
