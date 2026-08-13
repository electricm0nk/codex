---
canonical: true
owner: god-emporer
status: planning-ready (2026-08-13)
date: 2026-08-13
---

# SD-32 Forward Scope Register

Work this bundle deliberately does not take, with the reason and the successor
condition. An entry here is a decision, not an oversight.

---

## F1 — The 3,547 `unmeasurable` units

**Mass:** 3,218 `class_feature` + 329 `feat`. `status` came back `unknown`: the
record gives no evidence at all, so there is nothing to place on the ladder.

**Command:**
`python3 docs/release/SD-32-instrument-coverage-and-consumer-wiring/artifacts/derive-movable-mass.py`
→ final section.

**Why not here.** They are not `held` and not `in-progress`, so they are outside
this bundle's movable mass by definition. The dashboard's own `doneness_meaning`
calls them "a work item against the instruments, not a statement about the
content."

**Why it may be the better bundle.** 3,547 units is larger than everything E2,
E3 and E4 can move combined (ceiling 2,509). Every one of them is a unit the
instruments **failed to read**, not a unit the content failed to satisfy — which
makes it the purest instrument-coverage work on the board and the least exposed
to the gaming risk that dominates this bundle. Recorded as `risks-and-open-questions.md Q4`.

**Successor condition:** its own bundle, characterising the `unknown` reasons by
frequency before any fix is attempted.

---

## F2 — Spell units the consumer-delta probe cannot reach

**CLOSED-AND-REOPENED 2026-08-13.** The original F2 ("a spell consumer-delta
probe") was **done inside this bundle**, not forwarded: cards
`spell-consumer-delta-probe` (`aafd492c`) and `ground-spell-units`
(`90bd9975`). Its rationale was falsified — see the `[SUPERSEDED]` banner on
`decisions.md §5`. What remains forward is the part the probe genuinely cannot
reach.

~~**Mass:** would move 178 `spell` units from `held` to `in-progress`, and 0 to
`done`.~~ **Retracted.** Measured outcome of actually building it: `done` +46,
`held` −46, `in-progress` +0, worse-bucket transitions 0. The 178 was the
`NO_GROUNDING_PROBE` cap population, which only a *producer* edit moves.

**Mass now forward: 113 `spell` units** — the `computed`-class remainder, the
only wiring class whose grounding can reach `done` under today's verdict table.
By book: ARG 92, ACG 13, APG 7, UI 1. (A further 524 non-`computed`
`ingested-magnitude` spells exist but would land on `held` even if grounded, so
they are behind `decisions.md §2`, not behind this item.)

**Why not here — three distinct blockers, none of them a probe-tuning knob:**

1. **ARG 92 + CRB 29 — `no_casting_class_has_it`.** The probe selects a spell
   only through a class whose own CRB list holds it, and only through the seven
   ids `spellbook::casting_ability_for_class` maps to a casting ability. A class
   it does not map yields no DC at all, so widening the probe's list alone
   observes nothing. Reaching these needs the **engine** to model more casting
   classes (Alchemist, Witch, Magus, ...). Probing them as a Wizard instead is
   explicitly declined: it reports a magnitude no player can see.
2. **APG 271 + ACG 144 — `no_table_effect`.** The key is in no per-school
   table, so no `SpellEffect` and no level is produced. This is ingestion into
   `crb::spell_list`'s per-school tables, i.e. content work.
3. **UI 101 — never asked.** There is no `data/corpus/ultimate_intrigue/`, so
   the book is not in `OBSERVABLE_BOOK_DIRS` and the probe has no corpus to
   load. Blocked on the UI corpus landing, not on the instrument.

**Successor condition:** (1) more casting classes reach
`casting_ability_for_class`; (2) APG/ACG spells reach a per-school table;
(3) the Ultimate Intrigue corpus lands on disk. Each independently unlocks its
own slice; none of them is unlocked by changing the probe.

---

## F3 — The `static` byte-equality sweep and the `derived` evaluator-vs-fixture check, if the gate is declined

**Mass:** 7,479 held units (4,805 `static`, 2,674 `derived`).

**Why conditionally not here.** E5 and E6 are in this bundle but `BLOCKED
(decision)` on `decisions.md §2`. If the operator declines the `done` rung, both
epics move to this register rather than being attempted: the instruments would
still be worth building on their engineering merits, but they would move zero
units on the board and this bundle must not pretend otherwise.

**Successor condition:** `decisions.md §2` answered yes, in writing, by the
operator or dashboard owner.

---

## F4 — Effect shapes E3 does not take

**Mass:** the remainder of the 375 bucket-A1 units after E3-F2 takes the largest
shapes.

**Why not here.** E3-F1 groups the inert items by effect shape; E3-F2 takes the
ones whose wiring is bounded. Shapes that need substantial new engine capability
are left, listed, with the reason. Leaving a shape is a **COMPLETE** outcome
(`epic-breakdown.md` E3-F2).

**Successor condition:** the shape becomes product scope in its own right.

---

## F5 — `companion`'s stale `NO_GROUNDING_PROBE` listing

**Mass:** 0 units. The cap fires on nothing for `companion`.

**Finding.** The producer lists `companion` in `NO_GROUNDING_PROBE` on the
grounds that it reads `grounded: 0`. The current payload carries **922 grounded
companion units**, 416 of them already `done`. The justification is stale.

**Why not here.** `decisions.md §6`. It is a producer edit, and this bundle does
not make producer edits — not even correct ones that move nothing. Reported to
the dashboard owner so a later reader does not re-derive it as new work.

**Successor condition:** the dashboard owner's call. If they act, expect the
board to be unchanged, which is itself the confirmation that the derivation was
right.

---

## F6 — `not-started` (21,303 units)

**Why not here.** `decisions.md §7`. That is content the engine does not hold —
book-ingestion work belonging to the SD-29/SD-30 lanes. This bundle touches only
units the engine already holds.

**Successor condition:** the existing ingestion lanes; no new successor needed.

---

## F7 — A second reviewer for E8

**Why recorded.** `risks-and-open-questions.md R1` residual: E8 reviews a diff
authored under the same pressure that E8 exists to catch. A reviewer that did
not implement E2/E3/E4 is materially stronger.

**Successor condition:** operator or orchestrator assigns one at E8 dispatch.

---

## F8 — Consumer wiring for the 174 units whose bonus chain no consumer reads

**Why recorded.** `decisions.md §10`. After E2 widened the probe, the only
`in-progress` population that is neither an ingestion gap nor an absence of
magnitude is 174 equipment/equipment_modifier units, and it splits in two:

- **38 near-misses.** The record carries a shape `equipment_effects.rs` already
  reads and still produced no delta. 24 of them are
  `BONUS:COMBAT|AC|<n>|TYPE=ArmorEnhancement`/`TYPE=ShieldEnhancement` rows —
  CRB `Special Ability ~ +1..+5 ~ Armor/Shield`, PU `ABP ~ +N Attunement ~
  Armor/Shield` — which `armor_class_bonus_from_bonus_chains` deliberately does
  not match, accepting only `TYPE=Armor`/`TYPE=Shield`. The rest are
  `%CHOICE`-parameterised chains and `PREVARGT`/`PREEQUIPBOTH`-gated weapon
  chains. **This is the cheapest lever left on the equipment kind.**
- **136 unread families.** `BONUS:VAR` (130 chains), `ITEMCOST` (47),
  `EQMARMOR` (17), `EQM` (8), `SAVE` (7), `WEAPONPROF` (20), and a tail.

**Why not here.** `ResolvedEquipmentEffect` has exactly seven fields and
`equipment_key_is_wired` reads all seven — the probe is not under-reading, the
*engine* does not compute these. Closing it means widening the effect model and
landing each new magnitude on a twin the player actually reads
(`AGENTS.md`: "a magnitude is not wired until it moves on the twin the player
reads"). That is engine work with parity surface, not instrument work, and
extending the probe to "observe" a chain no consumer reads would be precisely
the green-instrument-over-an-empty-screen failure this bundle exists to avoid.

**Two extra constraints the successor must carry.** (1) `TYPE=ArmorEnhancement`
stacks *on top of* the base armor bonus rather than replacing it, so accepting
it changes computed AC and owes a fixture plus an oracle comparison. (2) These
are equipment **modifiers**; the probe equips each key standalone with
`applied_modifiers: []`, which is the wrong question for a modifier. A
modifier-shaped probe applies the mod to a host item and reads the delta
against that host. Neither of those is a one-line widening.

**Successor condition:** an E3-shaped card whose ceiling is re-derived to 174,
not the scope card's 375 — 239 of that 375 carry no bonus chain at all and
cannot be wired by any effect model.

**Derivation:**
`python3 docs/release/SD-32-instrument-coverage-and-consumer-wiring/artifacts/why-in-progress-equipment-stalls.py`

---

## F9 — 295 in-progress equipment units have a compiled catalog and no corpus

**Why recorded.** `decisions.md §10`. Ultimate Equipment (190), Ultimate
Psionics (82), Ultimate Combat (22) and Ultimate Intrigue (1) have compiled
`equipment_tables` and catalog rows, but no `data/corpus/<book>/equipment`
directory at all. No consumer-delta probe can ever reach them, and E2's
book-scoping now says so out loud instead of letting them ground off another
book's reprint of a same-named, different item.

**Why not here.** Ingestion, which `decisions.md §7` puts out of scope.

**Successor condition:** the SD-29/SD-30 ingestion lanes producing
`data/corpus/<book>/equipment` for those four books. On the day that lands, E2's
probe covers them with no further instrument work — the keys are already in its
universe.

---

## F10 — 38 equipment names still resolve by scan order when NEITHER record owns the name

**Recorded by** the `inventory-determinism` cycle (`progress.md`,
2026-08-13, `probe-determinism`). **Named owner:** the next cycle that touches
`src/rules_core/equipment_resolver.rs`.

**What landed.** `equipment_id_resolve` now matches a record's corpus
**identity** first — its `KEY:` token when it has one, its name when it does
not. That is what a needle naming a KEY-less record needs, and it is what fixed
CRB's `Shoes` (the item, no `KEY:`) losing to
`Artisan's Tools (Shoes)` (an equipment modifier with the same display name)
whenever the corpus was scanned in a different order.

**What is left.** The identity pass only fires when *some* record's identity
equals the needle. Where several records share a display name and **none** of
them is identified by that name, the bare-name pass still answers
first-match-wins over corpus order — now path order rather than `read_dir`
order, so it is deterministic and checkout-stable, but it is still arbitrary.
CRB's `Cloth` is the shape: `Artisan's Tools (Cloth)` against
`Material ~ Cloth`, neither identified as `Cloth`.

**Why it is not urgent, stated no more strongly than it was proved.** What was
measured is that with the identity pass in place, the generator's whole output
is byte-identical whether the corpus is scanned in path order or in the
filesystem order that preceded it — so no unit's doneness moved under either
order. What was *not* proved is that nothing can reach the bare-name pass with
an ambiguous needle: `equipment_catalog_rows()` is built from compiled per-book
tables, and a compiled key that matches no corpus record's identity would fall
through to exactly that pass. The successor should establish which, rather than
inherit this paragraph's confidence.

**Successor condition.** Either prove no live caller can reach the bare-name
pass with an ambiguous needle and delete it, or make it refuse an ambiguous
match outright — returning `None` rather than a coin-flip is the honest answer
for a lookup that cannot tell two records apart.

**Derivation.** Two records also share an *identity* outright in CRB
(`Holy Symbol (Silver)`, `Holy Symbol (Wooden)`), which the identity pass cannot
disambiguate either; same successor.

```
python3 -c "
import json,os,collections
base='data/corpus'
tot=0
for book in sorted(os.listdir(base)):
    eq=os.path.join(base,book,'equipment')
    if not os.path.isdir(eq): continue
    byname=collections.defaultdict(set)
    for dp,_,fns in os.walk(eq):
        if '_parity' in dp: continue
        for fn in fns:
            if not fn.endswith('.json') or fn=='LICENSE.json': continue
            d=(json.load(open(os.path.join(dp,fn))).get('data') or {})
            k=next((t['value'] for t in d.get('raw_tokens') or [] if t.get('key')=='KEY'), None)
            byname[d.get('name','')].add(k if k is not None else d.get('name',''))
    n=sum(1 for v in byname.values() if len(v)>1)
    if n: print(book, n); tot+=n
print('total', tot)"
-> advanced_class_guide 18 / advanced_race_guide 1 / core_rulebook 19 / total 38
```

---

## F11 — 29 pairs of corpus keys differ only in punctuation and may be one record each

**Recorded by** the `inventory-determinism` cycle. **Named owner:** the
ingestion lane that owns the book each pair belongs to (Ultimate Psionics owns
20 of the 29), as a **content** ruling — not instrument work.

**What this is.** `units[].id` collisions were fixed by disambiguating the
*identifier*, deliberately and explicitly without merging the units: two
distinct corpus keys are two records, and merging them would have changed a
count this cycle had no authority to change. But the pairs themselves deserve a
look, because most of them are one feature spelled two ways:

| book | pairs | shape |
|---|---:|---|
| `ultimate_psionics` | 20 | `Path Skill Acrobatics` / `Path Skill ~ Acrobatics`, and 15 more skills; `Thrallherd Mind Control` / `Thrallherd ~ Mind Control` |
| `core_rulebook` | 3 | `MITHRAL_ITEM` / `Mithral (Item)`; `Intelligent Item Purpose (Slay All)` / `Intelligent Item ~ Purpose / Slay All` |
| `ultimate_combat` | 2 | `Master Of Many Styles ~ Perfect Style` / `Master of Many Styles ~ Perfect Style` — a capital `O` |
| `advanced_class_guide`, `advanced_players_guide`, `advanced_race_guide`, `adventurers_guide` | 4 | one each |

The Ultimate Psionics `Path Skill` block is the clearest: sixteen skills, each
appearing once at `up_abilities_class.lst:600-615` with the `~` namespace form
and once at `:619-634` without it, the two rows landing on different
`wiring_class` values (`computed` vs `display`) and different `status`
(`unknown` vs `not-ingested`).

**Why it matters to the board.** If these are duplicates, the board is counting
each of them twice, and one member of each pair is scoring a `wiring_class` the
other contradicts. If they are genuinely two records, nothing is wrong and the
disagreement is real content. Either answer is fine; **not knowing which** is
what should not persist.

**Successor condition.** A per-pair ruling from whoever owns the book, backed by
the two `.lst` rows read side by side — not a bulk merge, and explicitly not a
merge performed to make a count fall.

**Derivation.**

```
python3 -c "import json,collections; d=json.load(open('docs/work-inventory.json'));
b=collections.defaultdict(list)
for u in d['units']: b[u['id'].split('__')[0]].append(u)
for k,v in sorted(b.items()):
    if len(v)>1: print(k, [x['corpus_key'] for x in v], [x['wiring_class'] for x in v])"
```
