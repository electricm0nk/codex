# Class/Multiclass Breadth (Risks Item 8) — Scoping Plan

> Operator-directed (2026-07-24): scope, don't implement, a working chassis
> for the 8 of 11 CRB classes that currently never reach `Computed`
> (Barbarian, Bard, Cleric, Druid, Monk, Paladin, Ranger, Sorcerer). Same
> shape as `item-1-architecture-wall-design.md`/`items-1-and-27-scoping.md`:
> design/scoping only, no chassis code in this pass. Read those two first
> for the established "verify before assuming, size before dispatching"
> discipline this plan follows.

## The central finding: the chassis is not missing — it exists, and is already running, on every call

The backlog framing ("8 of 11 CRB classes have no working chassis at all")
is correct about the observable behavior (none of the 8 ever reaches
`HeadlessReceiptStatus::Computed`), but the natural reading — "build a
`compute_barbarian_chassis`-style function per class from scratch, the same
order of magnitude as Fighter/Wizard/Rogue" — is wrong. Verified directly,
not assumed:

- Every one of the 8 classes already has its own `explain_<class>_level1_...`
  function in `pilot_compute.rs` (e.g. `explain_barbarian_level1_chassis`,
  `explain_ranger_level1_chassis_and_class_feature_separation`,
  `explain_sorcerer_level1_spell_baseline`), and **`compute_pilot_base_chassis`
  already calls every one of them, on every single invocation** (lines
  ~4638-4710). They are not dead code — they run today, computing real BAB,
  saves, HP-relevant hit-die data, and a long list of named class features
  (Rage's flat constants, Bardic Performance's rounds/day and Inspire
  Courage magnitude, Smite Evil/Lay on Hands/Divine Grace, Favored
  Enemy/Terrain, Track, combat style, Channel Energy, domain choice,
  bloodline/Eschew Materials, Wild Empathy, Ki pool/Slow Fall, etc.) —
  they are just architected as **parallel, standalone explanation
  surfaces**, deliberately not wired into the three integrated pillars
  (`base_attack_bonus`, `compute_total_saves`, `compute_combat_baseline`).
- `rules_tables::crb::class_tables()` already carries real, data-verified
  BAB/save/hit-die/level-ceiling rows for **all 11** CRB classes, not just
  the 3 currently supported. The generic table-driven chassis path
  (`compute_generic_table_chassis`, already used for Rogue) is fully
  data-ready for the other 8 today.
- The single chokepoint gating all of this is `table_class_id`
  (`pilot_compute.rs:6747`) — a 3-entry allowlist (Fighter/Wizard/Rogue)
  that every other dispatch function (`compute_class_chassis`,
  `is_supported_multiclass_mix` → `multiclass_class_level_supported`,
  and `durability.rs::compute_max_hp`) routes through. Widening it is a
  small, mechanical change in isolation.

**Why it isn't just a one-line fix**: `table_class_id`'s own doc comment is
first-hand evidence this was already tried and deliberately scoped back.
Widening it to all 11 classes in one step broke ~60 pre-existing
negative-control assertions across ~15 `tests/**` files in this codebase's
own history (confirmed via `cargo test --test '*' --no-fail-fast` at the
time) — those files assert, verbatim, that each class's own standalone
explanation "must not be wired into the integrated total" (e.g.
`tests/sd13_ranger_base_attack_and_saves.rs` checks the explanation detail
text literally contains the word `"standalone"`). Widening the dispatch
for a class flips that class's own negative-control tests from correct to
failing — not a bug in the new code, a genuine, expected assertion flip
that needs coordinating with QA the same way sub-task 1's 42-site handoff
did, just per-class instead of all at once.

**Second, independent layer**: even after `table_class_id` recognizes a
class, **7 of the 8** (all but Ranger) push their own unconditional
claim-blocking diagnostic inside their `explain_*` function — a second,
deliberate gate, separate from the dispatch allowlist:

| Class | Self-blocking diagnostic(s) | What's missing |
|---|---|---|
| Barbarian | `class_feature.barbarian.bounded_progression.rage_execution.unsupported` | Rage *execution* (activation, round consumption, fatigue, applying its flat constants to a total) — the constants and rounds/day are already grounded |
| Monk | `class_feature.monk.bounded_progression.bonus_feat.unsupported` | The level-1 bonus feat's real mechanics (needs a feat-effect, same engine as risks item 17) |
| Cleric | domain-power burden + `class_spell.cleric.prepared_divine.unsupported` | Domain *powers* execution; full prepared-divine posture (slots/day, spontaneous cure/inflict, save DCs, bonus slots) |
| Druid | nature-bond/wild-empathy (ungrounded) + `class_spell.druid.prepared_divine.unsupported` | Nature bond entirely; same prepared-divine posture as Cleric. Smallest, least-built of the 8 (~524 lines); still capped at level 15, not 20 |
| Paladin | `class_spell.paladin.partial_caster.unsupported` (explicitly "unchanged... still claim-blocking at every level") | Only the partial-caster spell *posture* is declared out of scope — everything else (Smite Evil, Lay on Hands, Divine Grace, Mercy, Channel Positive Energy, late auras, Holy Champion) is fully grounded through level 20 |
| Ranger | **none** | Its `explain_ranger_...` function's signature has no `diagnostics` parameter at all — it structurally cannot self-block. BAB/saves, Track, combat style + bonus feats, Favored Enemy/Terrain, Hunter's Bond, and a full partial-caster spell ladder are grounded through level 20 |
| Sorcerer | bloodline-progression burden + `class_spell.sorcerer.spontaneous.unsupported` | Bloodline power progression; spontaneous known-spell/slot posture |
| Bard | performance-execution burden + `class_spell.bard.spontaneous_known_and_per_day.unsupported` | Bardic Performance *execution* (action economy, round tracking, countersong/fascinate); spontaneous known-spell/slot posture |

Four classes (Cleric, Druid, Sorcerer, Bard) name the identical shape of
remaining gap — a "prepared/spontaneous spell posture" burden — worth
treating as one shared piece of future scope, not four separate ones, once
it's tackled.

**A documentation-accuracy bug worth fixing alongside this** (trivial,
separate from the scoping question): `multiclass_class_level_supported`'s
own doc comment claims it was "widened... to every core class" the table
carries data for — the function body still calls `table_class_id`, which
only ever recognizes 3. The comment is stale relative to the code it
describes. Flagging so it doesn't mislead a future reader into believing
multiclass already supports all 11.

**A naming trap, unrelated to the above, worth flagging so it isn't
misread**: `support_state_matrix.rs` independently tracks these same 8
classes and shows several as `Computed` on its own axis — but that
`Computed` is a value of an entirely different enum, `EvidenceTier`
(`Observed, Parsed, Converted, Computed, OracleChecked, ProductVisible`,
meaning "this dimension has been exercised in code"), not
`pilot_compute::HeadlessReceiptStatus::Computed` (meaning "no claim-blocking
diagnostic present," the thing this backlog item is actually about). Same
variant name, unrelated types. The backlog framing is correct; the matrix
is also correct; they answer different questions.

## Relative size (evidence: file/line counts, not estimates)

| Class | doc+gate+explain fn (lines) | test files | test LOC | level ceiling | self-block? |
|---|---|---|---|---|---|
| Ranger | ~2,425 | 33 | 13,069 | 20 | none |
| Bard | ~1,091 | 31 | 11,724 | 20 | yes (x2) |
| Monk | ~1,128 | 17 | 7,051 | **12** | yes |
| Paladin | ~959 | 30 | 11,638 | 20 | yes |
| Sorcerer | ~694 | 30 | 11,170 | 20 | yes (x2) |
| Cleric | ~625 | 24 | 8,953 | 20 | yes (x2) |
| Barbarian | ~762 | 23 | 9,433 | 20 | yes |
| Druid | ~524 | 18 | 6,856 | **15** | yes (x2) |

For comparison, the 3 already-`Computed` classes: Fighter 26 files/8,153
lines, Wizard 32/11,126, Rogue 27/8,943 test lines. In aggregate, the 8
"unsupported" classes already have about as much test investment as the 3
supported ones — this is a wiring/integration gap, not a research or
test-authoring gap.

## Recommendation: Ranger first, one class at a time after that

Ranger is the only one of the 8 with no self-imposed blocker and the
largest existing investment (2,425 lines, 33 test files, full level-20
chassis + partial-caster spell ladder already grounded). It needs:

1. Add `"class:ranger" => Some(ClassId::Ranger)` to `table_class_id`.
2. Coordinate with QA on `tests/sd13_ranger_base_attack_and_saves.rs` (and
   any sibling Ranger test asserting the standalone/"not wired in" shape) —
   compute the exact site list first, hand off, don't touch `tests/**`
   unilaterally, same protocol as sub-task 1.
3. Verify end-to-end: a Human Ranger built with the same fixed
   Longsword/Chain-Shirt/Dodge/Weapon-Focus loadout `compose_character_input`
   already gives every class reaches `Computed` for real, at multiple
   levels (1, 5, 11, 20 breakpoints, mirroring sub-task 4's own
   re-verification discipline) — not assumed from the dispatch change
   alone.
4. Decide, explicitly, whether `durability.rs::compute_max_hp` should be
   widened for Ranger in the same cycle (it shares the identical
   `table_class_id` chokepoint) or deliberately deferred — don't let it
   silently ride along unverified.

This mirrors this whole session's own established discipline (Toughness
before the other 3 feats; the single-weapon slice before per-weapon
resolution; armor/shield before weapon-loadout widening in sub-task 4) —
prove the cheapest real case fully, verified, before generalizing.

**After Ranger, each of the other 7 is its own bounded cycle**, in
whatever priority order the operator/lead wants, each needing: (a) the
`table_class_id` entry, (b) resolving that class's own named execution-engine
gap (not a guess — the table above names exactly what's missing per
class), (c) the same QA test-coordination step, (d) the same
multi-level re-verification. None of the 7 should be batched together —
each is independently sized and independently risky (a Cleric mistake
doesn't need to block a Barbarian cycle), matching the "one class, one
coordinated test update" framing `table_class_id`'s own doc comment
already anticipated.

## What this plan is not

This is not a decision to build all 8. It's the sizing and sequencing pass
the operator asked for before any chassis code starts. The real remaining
work per class (rage execution, domain powers, bloodline progression,
performance execution, the shared prepared/spontaneous spell posture) is
real engineering, not administrative wiring — this doc sizes it, it does
not shrink it. Total remaining scope across all 8 is still substantial;
Ranger being cheap does not imply the other 7 are.

## Open question for the operator/lead

Confirm Ranger as the first slice, and confirm the per-class sequencing
approach (one bounded cycle at a time, not a combined push) before any
code starts.
