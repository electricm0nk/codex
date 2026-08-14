---
canonical: true
owner: god-emporer
status: planning-ready (2026-08-13)
date: 2026-08-13
companion_to: ./decisions.md, ./epic-breakdown.md
---

# SD-32 Scope Draft — Instrument Coverage and Consumer Wiring

## 0. What this bundle is for

Operator directive, 2026-08-13: *"improve our numbers, assuming the measuring
systems are accurate."* The second clause is a **constraint**, not a licence.
The instruments are to be trusted and **extended**; they are never to be tuned
to flatter the result. `decisions.md §1` carries the full rule.

This document does three things: it re-derives the real movable mass, it splits
that mass by reachability, and it ranks the split by units-moved-per-unit-of-work.

## 1. Method, and the one command

Every figure below is produced by one script, committed with this package:

```
python3 docs/release/SD-32-instrument-coverage-and-consumer-wiring/artifacts/derive-movable-mass.py
```

It reads two sources, both read-only:

1. `docs/work-inventory.json` — the generator's output
   (`cargo run --bin v06_work_inventory`), `generated_at 2026-08-13T09:33:16Z`.
2. The doneness verdict table, transcribed from `doneness_verdict()` and
   `_doneness_verdict_uncapped()` in the dashboard producer.

**The transcription is validated, not trusted.** The script recomputes
`by_doneness` from the inventory and asserts equality with the live payload's
`work_inventory.by_doneness` before printing anything:

```
transcription validated against live dashboard: True
  re-derived: {'done': 3426, 'held': 9475, 'in-progress': 734,
               'not-started': 21303, 'unmeasurable': 3547, 'deferred': 36}
  dashboard : {'done': 3426, 'held': 9475, 'in-progress': 734,
               'not-started': 21303, 'unmeasurable': 3547, 'deferred': 36}
```

Six of six buckets agree exactly. Every downstream cell in this document is
therefore a real split of the board's own numbers, not a parallel model of them.
If the producer's table ever changes, the script exits non-zero rather than
reporting a stale split.

## 1a. Figure stamp, and a live reclassification caught mid-scope

**Every figure in this document is stamped to inventory generation
`2026-08-13T09:33:16Z`**, which is the generation the dashboard's doneness
cache was built from, which is why the validation in §1 agrees six of six.

While this package was being authored, a **concurrent agent regenerated
`docs/work-inventory.json`** (stamp `2026-08-13T15:25:55Z`) and the guard in
`artifacts/derive-movable-mass.py` fired. It was not a verdict-table change —
the script now separates the two cases explicitly — it was a live
wiring-class reclassification, measured at **1,066 units changing
`(wiring_class, status)`**, overwhelmingly toward `derived`:

| movement | units |
|---|---:|
| `display` → `derived` (`not-ingested`) | 630 |
| `display` → `derived` (`grounded`) | 173 |
| `display` → `derived` (`not-started`) | 146 |
| `static` → `derived` (`unknown`) | 61 |
| `display` → `derived` (`text-complete`) | **26** |
| other | 30 |

Board effect: `done` 3,426 → 3,400, `held` 9,475 → 9,501. The 26 lost `done`
units are exactly the `display`+`text-complete` → `derived`+`text-complete`
moves — `display`+`text-complete` is a `done` cell, and `derived` has none.
**A reclassification that lowers `done` is the honest direction**, and nothing
here suggests otherwise; it is recorded because it moves this bundle's ground.

**The structural conclusions survive the regeneration, and were re-derived
against both generations:**

| | 09:33 | 15:25 |
|---|---:|---:|
| movable mass | 10,209 | 10,235 |
| bucket A (existing instrument) | **734** | **734** |
| bucket B1+B2 (classifier lever) | 1,776 | 1,601 |
| bucket B3 (static/derived, gated) | 6,418 | 6,619 |
| bucket C (spell, unreachable) | **1,281** | **1,281** |

A and C are identical. The movement is entirely **out of this bundle's rank-2
lever and into its blocked bucket** — which sharpens, rather than softens, the
conclusion that `decisions.md §2` is the dominant question on the board. If that
reclassification continues, E4's ceiling keeps shrinking and E5/E6's keeps
growing behind a gate nobody has opened.

**Operational consequence.** A cycle in this bundle re-derives against the
stamp the dashboard is actually showing, and treats a stamp mismatch as
"re-derive", not as "the table broke."

## 2. The two cells that produce `done`

This is the finding everything else in the document hangs off, and it is read
directly off the producer's verdict table:

| wiring_class | status | verdict |
|---|---|---|
| `display` | `text-complete` | **done** |
| `computed` | `grounded` | **done** |
| `display` | `grounded` | held |
| `display` | `ingested-magnitude` | in-progress |
| `static` / `derived` | `grounded`, `text-complete`, `ingested-magnitude` | **held — every status** |
| `ambiguous` | any evidence-bearing status | held |
| `computed` | `text-complete`, `ingested-magnitude` | in-progress |
| any | `unknown` | unmeasurable |

**Exactly two cells produce `done`.** For a `held`/`in-progress` unit to
legitimately become `done`, it must end up as `computed`+`grounded` or
`display`+`text-complete`. There is no third path.

The corollary is severe and is the largest single fact in this scope:
**`static` and `derived` have no `done` cell at all.** 7,479 held units are
`static` or `derived`. The corpus-literal byte-equality sweep and the
evaluator-vs-fixture check — the two instruments the dashboard's own
`doneness_meaning` names as missing — could be built exactly as specified, run
green, and move **zero** units, because the table has no rung above `held` for
those classes to land on. `decisions.md §2` handles this as an operator
decision request, not as a change a cycle makes.

## 3. Re-derived movable mass

`held` + `in-progress` = **10,209** units. By kind and doneness:

| kind | held | in-progress |
|---|---:|---:|
| equipment | 4,675 | 296 |
| monster_ability | 1,295 | — |
| spell | 1,281 | — |
| monster | 1,235 | — |
| companion | 506 | — |
| race_trait | 249 | — |
| feat | 102 | 1 |
| class_feature | 91 | — |
| equipment_modifier | 34 | 437 |
| race | 7 | — |
| **total** | **9,475** | **734** |

These match the standing figures in the launch brief. The rest of this section
is the part the standing figures do not carry.

## 4. The reachability split

### Bucket A — reachable with an instrument that EXISTS (734 units, 7.2%)

Bucket A is exactly the `in-progress` bucket, and `in-progress` is **100%
`computed`**: every one of the 734 is `computed` + (`ingested-magnitude` |
`text-complete`), needing only an observed consumer delta to become
`computed`+`grounded` = `done`. That agrees with the producer's own definition
of `in-progress` ("the bar is reachable with an instrument that exists"), which
is a second, independent confirmation of the split.

It divides on a fact about the probe, not about the content:

| sub-bucket | units | cells | what actually blocks it |
|---|---:|---|---|
| **A1** — probe examines it, no mechanical effect observed | **375** | 233 `equipment_modifier`/computed/ingested-magnitude, 142 `equipment`/computed/ingested-magnitude | the ENGINE does not apply the item's effect. `equipment_key_is_wired()` equips the item alone and asks `compute_equipment_effects` for a non-`None` AC / max-dex / spell-failure / ACP / skill / ability / weapon-enhancement result. These items return nothing. |
| **A2** — the probe's key universe omits the book | **358** | 204 `equipment_modifier`, 154 `equipment`, all computed | `probe_equipment_effect_wiring()` builds its key set from **four** compiled tables (`crb`, `apg`, `acg`, `beastiary1`) and loads corpus from six `OBSERVABLE_BOOK_DIRS`. Eleven books have a compiled `equipment_tables.rs`. The probe never looks at these units at all. |
| **A4** — one feat | **1** | `feat`/computed/text-complete, `ultimate_wilderness` | single unit; not worth its own card. |

A2's book split (`ultimate_equipment` 190, `ultimate_psionics` 82,
`pathfinder_unchained` 39, `ultimate_combat` 22, `advanced_race_guide` 24,
`ultimate_intrigue` 1) is entirely inside books that already have a compiled
`src/rules_core/rules_tables/<book>/equipment_tables.rs`.

**The probe demonstrably reaches these kinds.** `computed`+`grounded` counts
corpus-wide: `equipment` 37, `equipment_modifier` 40. Both non-zero, both from
`core_rulebook`/`advanced_class_guide`. So A1 and A2 are not aspirational — the
instrument produces `done` for these kinds today.

**Honest ceiling, stated once.** "The probe reaches this kind" is not "the probe
will fire for this unit." A2's 358 units become `grounded` only insofar as their
items really do produce a mechanical effect once the probe looks; the ones that
do not stay `ingested-magnitude`, correctly. 734 is a **ceiling**, not a forecast.

### Bucket B — reachable once a NAMED missing instrument is built (8,194 units, 80.3%)

| sub-bucket | units | the named missing instrument |
|---|---:|---|
| **B1** — `ambiguous` + evidence | **360** | the wiring-class classifier over the full token closure GE-01 defines |
| **B2** — `display` + `grounded` | **1,416** | the same classifier — a real consumer delta contradicts a no-magnitude classification |
| **B3** — `static`/`derived` + evidence (non-spell) | **6,418** | the corpus-literal byte-equality sweep (`static`) and the evaluator-vs-fixture check (`derived`) — **AND** a `done` rung the verdict table does not have (`§2` above, `decisions.md §2`) |

B1+B2 = **1,776** units whose bar is not merely unconfirmed but **not yet
known**. Dominant cells: `monster_ability`/display/grounded **1,121**,
`race_trait`/ambiguous/grounded **223**, `companion`/display/grounded **215**,
`feat`/ambiguous/text-complete **71**, `class_feature`/display/grounded **54**.
(A further 42 `spell` units also carry an unresolved class; they are counted in
bucket C because the spell block binds first.)

**B1/B2 is where the gaming risk lives, and it is the sharpest in the bundle.**
A "classifier fix" that moves units from `display` to `computed` moves them
straight into the `done` cell. `decisions.md §3` governs it: the classifier's
acceptance criterion is **agreement with a hand-labelled sample**, movement is
recorded in **both** directions, and a net-negative movement is a **passing**
outcome.

B3 is the mass: `equipment`/static/ingested-magnitude alone is **4,428**,
`monster`/derived/grounded **1,229**. It is also the bucket that moves nothing
until `decisions.md §2` is answered.

### Bucket C — structurally unreachable (1,281 units, 12.5%)

**Every unit in bucket C is a `spell`.** `spell` `grounded` count corpus-wide is
**0**, and it is 0 by construction, not by omission: `classify()`'s `Kind::Spell`
arm never returns `grounded` — "no currently-wired consumer reads a spell's
magnitude, so every resolved-level spell stays `ingested-magnitude`." A spell's
only `done` cell is `display`+`text-complete`, which exactly **one** spell in the
corpus occupies.

- C1 — 178 units the `NO_GROUNDING_PROBE` cap moves from `in-progress` to `held`
  (159 computed/ingested-magnitude, 16 display/ingested-magnitude, 3
  computed/text-complete).
- C2 — 1,103 units blocked by the `static`/`derived`/`ambiguous` situation *and*
  by the spell block, whichever is lifted first.

**Two corrections to the standing brief, both re-derived:**

1. **`companion` is not blocked behind `NO_GROUNDING_PROBE`.** The launch brief
   marks `companion` 506 with `(* = blocked behind NO_GROUNDING_PROBE)`. The
   corpus carries **922 `grounded` companion units**, 416 of them
   `computed`+`grounded` and already counted `done`. The `NO_GROUNDING_PROBE`
   cap moves **0** companion units — every companion in `held` is
   `derived` (270), `display` (215), `static` (19) or `ambiguous` (2), all
   blocked by the verdict table or the classifier, not by a missing probe. The
   producer's own justification for listing `companion` ("`companion` and
   `spell` alone read `grounded: 0`") is **stale against the current payload**.
   This is a report to the dashboard owner (`forward-scope-register.md F5`), not
   a change a cycle makes, and acting on it would move 0 units either way.
2. **`spell` is the only genuinely probeless kind**, and only 178 of its 1,281
   held units are held *by that cap*; the other 1,103 would still be held after a
   spell probe existed.

## 5. Ranking by units-moved-per-unit-of-work

**Method.** `R = ceiling_units / estimated_agent_epics`, where an *agent-epic*
is one bounded RED→GREEN dispatch with its own verification, estimated from the
number of distinct code surfaces the lever touches. Three rules govern the table:

- A lever whose ceiling depends on an unsanctioned measurement change is
  reported at **R = 0 (blocked)**. It is never averaged in at its unblocked
  value, because a blocked lever's real yield today is zero.
- `ceiling_units` is an **upper bound**, never a forecast. Where the yield
  fraction is unknown, the table says so in the confidence column rather than
  inventing a multiplier.
- Ranking is advisory for dispatch order only. It never justifies a shortcut:
  a high-R lever that cannot clear its bar honestly is left alone.

| rank | lever | epic | ceiling | epics | R | confidence |
|---:|---|---|---:|---:|---:|---|
| 1 | Extend the equipment-effect probe's key/corpus universe from 4 tables to 11 | E2 | 358 | 1 | **358** | **high** — same bar, wider coverage; the instrument already produces `done` for these kinds |
| 2 | Wiring-class classifier over the full token closure | E4 | 1,776 | 4 | **444** | **low on yield, high on risk** — ceiling-ranked only; E4-F1 calibrates before E4-F2 moves anything |
| 3 | Wire the equipment effect shapes `compute_equipment_effects` does not apply | E3 | 375 | 3 | **125** | medium — 233 of 375 are CRB `equipment_modifier`, likely a small number of shared effect shapes |
| — | Static corpus-literal byte-equality sweep | E5 | 4,805 | 3 | **0 (blocked)** | would be 1,602 — the highest in the bundle — the moment `decisions.md §2` is answered |
| — | Derived evaluator-vs-fixture check | E6 | 2,674 | 5 | **0 (blocked)** | would be 535; it is also the **only** path that ever unblocks 1,061 spell units |
| last | Spell consumer-delta probe | — | ~0 | 4+ | **~0** | explicitly **not** recommended for the numbers: it moves 178 units from `held` to the worse-looking `in-progress`, and reaching `done` still needs a real consumer that does not exist |

E1 (the `decisions.md §2` decision request) is not in the ranking because it
writes no code and moves no unit by itself. It is card #1 anyway: it costs
nothing and it is the sole gate on 7,479 units — 73% of the entire movable mass.

**Recommended dispatch order:** E1 (free, unblocks the top of the table) → E2
(highest confidence) → E4-F1 (calibrate, cheap, decides whether E4 is worth 4
epics) → E3 → E4 rest → E5/E6 only if E1 is answered yes.

## 6. What this bundle can honestly be expected to move

With no change to the measurement pipeline, the ceiling on legitimate `done`
movement is:

```
734 (bucket A, ceiling) + yield × 1,776 (bucket B1/B2, yield uncalibrated)
```

`done` is 3,426 today. The **absolute ceiling** with the board as wired is
5,936, and the realistic figure is materially lower on both terms — A's 734
assumes every examined item turns out to produce an effect, and B1/B2's yield is
unmeasured until E4-F1 runs. **A plausible honest outcome for this bundle is
several hundred units, not several thousand.** Saying so now is the point:
`decisions.md §1` makes "fewer moved than hoped, honestly" a success condition,
and this section is the figure that condition will be judged against.

If `decisions.md §2` is answered yes, 7,479 further units become addressable —
but they become addressable *by building two real instruments*, which is the
bulk of E5 and E6, not by the answer itself.

## 7. Non-goals

- **Any write under `/home/ubuntu/swarm-observer/` or the producer skill directory.**
  Read-only, without exception. `decisions.md §2` is a request to the operator.
- Editing `doneness_meaning`, `DONENESS_VALUES`, `NO_GROUNDING_PROBE`,
  `EXCLUDED_BOOKS` or any bucket definition.
- Reclassifying a unit's `wiring_class` other than as the output of a classifier
  whose accuracy is separately demonstrated (`decisions.md §3`).
- Ingesting fixture data or hand-authoring rules data to satisfy a check
  (`decisions.md §1`, and `AGENTS.md` §6 / the no-stub doctrine).
- The 3,547 `unmeasurable` units (3,218 `class_feature`, 329 `feat`). They are
  the largest single instrument defect on the board and they are **not** in this
  bundle's movable mass. `forward-scope-register.md F1`.
- `not-started` (21,303). That is content ingestion, which is SD-29/SD-30 work.
