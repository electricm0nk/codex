# Feat-Effects Widening — Scoping (next slice past the 4 landed feats)

> Directed by the lead: feat **prerequisites** are fully wired (all 185
> CRB feat records across 4 categories, `feat_prereqs.rs` + its 4 submodules),
> but feat **effects** — the actual mechanical bonus a feat grants — are wired
> for only **4 of 185**: Toughness (+3 HP) and Great Fortitude / Iron Will /
> Lightning Reflexes (+2 to one named save each), all in
> `src/rules_core/feat_effects.rs`. This scopes the next groundable slice with
> the same "cheap, flat, self-scoped, not fabricated" discipline the class
> closures use.
>
> Lane: my writes are confined to `src/rules_core/feat_effects.rs`. I do **not**
> touch `pilot_compute.rs` or `rules_tables/{acg,apg}/` (backend's lane). That
> boundary turns out to be the single most important fact in this doc — see
> "The wiring-consumer constraint" below.

## The wiring-consumer constraint (read this first)

A feat effect only matters if some **computed value consumes it**. A pure
function in `feat_effects.rs` with no consumer is not a widening — it is dead
code that leaves the exact item-17 gap the engine was created to close (QA's
concrete finding: "a Fighter with Toughness stayed at HP 12 instead of 15").
So the real question is not "which feats are flat?" but "which feats are flat
**and** land on a value the engine actually computes today, **and** whose
consumer I am allowed to wire."

The two landed effects prove the shape — and the lane split:

| Effect | `feat_effects.rs` fn (my lane) | Consumer (who wires it) |
|---|---|---|
| Toughness HP | `hp_bonus_from_feats` | `apps/desktop/src-tauri/src/character_hub.rs:1866` (desktop app, **not** backend's lane) |
| Save feats | `save_bonuses_from_feats` | `pilot_compute.rs:25624` in `compute_total_saves` (**backend's lane**) |

The computed surface for the only path that reaches `Computed` today
(single-class Fighter, levels 1–3, per `status.md`) is small:

- **HP total** — consumer in `character_hub.rs`. Only CRB HP feat is Toughness → **done**, nothing left to widen here.
- **Base saves** — consumer in `pilot_compute.rs`. Great Fortitude / Iron Will / Lightning Reflexes → **done**.
- **Baseline Armor Class** (`compute_combat_baseline`, `pilot_compute.rs:26198`) — already includes a hardcoded `DODGE_AC_BONUS` (`:5323`, flat +1). Dodge is baked into the deterministic GE-08 posture, **not** read generically from `selected_feats`, so it is not a widening target.
- **Baseline melee attack bonus** — Weapon Focus is baked into the same fixed loadout (chosen-target, `feat:weapon_focus:weapon:longsword` compound id); no general chosen-target mechanism exists (`feat_effects.rs:26-34` already documents this gap). Not groundable generically.
- **Selected skill modifiers** — exactly three skills computed: **Climb, Intimidate, Swim** (`SelectedSkillModifiers`, `pilot_compute.rs:181-185`; computed in `compute_selected_skill_modifiers`, `:25847`). **This is the only unexhausted live consumer.** It is in **backend's lane**.
- **Initiative** — no Initiative total is computed anywhere (`pilot_compute.rs:14465`, `:1070`: "never wired into any actual Initiative total"). Dead consumer.
- **CMB / CMD** — no combat-maneuver total is computed. Dead consumer.

**Net:** every remaining groundable CRB feat effect lands on the selected-skill
modifiers, whose consumer (`compute_selected_skill_modifiers`) is in
`pilot_compute.rs` — backend's exclusive lane, the same split as
`save_bonuses_from_feats`. This is the central open question for the lead (below).

## Corpus findings (verified against `feat_data/general.rs` + real PF1 CRB text)

Only two CRB feats grant a flat, self-scoped bonus to one of the three
computed skills (Climb / Intimidate / Swim), with no chosen target:

- **Athletic** (`general.rs:17`): effect tokens
  `["SKILL","Climb","if(skillinfo(\"TOTALRANK\",\"Climb\")>=10,4,2)"]` and
  `["SKILL","Swim",...]`. Real PF1 CRB text: "You get a +2 bonus on Climb and
  Swim checks. If you have 10 or more ranks in one of these skills, the bonus
  increases to +4 for that skill." → **+2 Climb, +2 Swim** at every level the
  engine reaches `Computed`. Both skills are computed.
- **Persuasive** (`general.rs:50`): effect tokens
  `["SKILL","Diplomacy",...]` and `["SKILL","Intimidate",...]`. Real PF1 CRB
  text: "You get a +2 bonus on Diplomacy and Intimidate checks. If you have 10
  or more ranks... +4 for that skill." → **+2 Intimidate** (Diplomacy is not a
  computed skill, so its half of the effect has no live consumer and stays out).

**The `+2` is provable, not a fabricated simplification.** The corpus encodes
the value as `if(TOTALRANK >= 10, 4, 2)`. The deterministic selected-skill
posture pins `SELECTED_SKILL_RANK = 1` (`pilot_compute.rs:5335`), and skill
ranks never exceed character level, so `TOTALRANK >= 10` is provably false for
every character the engine computes → the conditional evaluates to `2`. This is
the identical reasoning the landed Toughness effect used for its own
`max(3,TL)` → `3` (`feat_effects.rs:52-62`, `TL` never exceeds 1 at supported
levels). The `+4` tier is deferred until the engine computes characters with
10+ ranks (level 10+), far beyond current coverage.

## Proposed scope

1. New `feat_effects.rs` function `skill_bonuses_from_feats(selected_feats:
   &[String]) -> SkillBonusesFromFeats`, mirroring `save_bonuses_from_feats`
   exactly (dependency-free leaf, no `pilot_compute` import).
2. `SkillBonusesFromFeats { climb: i16, intimidate: i16, swim: i16 }` —
   deliberately the exact three-field shape of `SelectedSkillModifiers`, so the
   consumer adds one field per skill, the same way `SaveBonusesFromFeats`
   mirrors the three saves. Recognizing only the three computed skills (not a
   general per-skill map) keeps every field of the returned struct a live,
   consumed value — no unwired half-effects.
3. Effect logic:
   - `climb += 2` if `selected_feats` contains `"Athletic"`
   - `swim += 2` if `selected_feats` contains `"Athletic"`
   - `intimidate += 2` if `selected_feats` contains `"Persuasive"`
   Keyed on the exact catalog `key` string (`"Athletic"` / `"Persuasive"`),
   verified the same way `TOUGHNESS_FEAT_KEY` was: `FeatTableEntry.key` passes
   through the selection pipeline unmodified.
4. Full RED-then-GREEN test module in `feat_effects.rs` mirroring the existing
   `save_bonuses_from_feats_tests` (absent-feat → 0, empty list → 0, each feat
   → its exact skills, both stacking, no substring/prefix false-match against
   e.g. a hypothetical "Athletic Steps"-style key, no double-count).
5. **Consumer wiring** (`compute_selected_skill_modifiers`, `pilot_compute.rs`,
   ~3 lines: `+ feat_skill_bonuses.climb` on the `climb` total, etc., with the
   explanation strings extended) — **this is in backend's lane, not mine.** See
   open question 1. I will author the `feat_effects.rs` half and its tests; the
   consumer edit must be done by backend or under an explicit narrow grant.

## What stays explicitly deferred, named honestly

- **The other 8 two-skill General feats** (Acrobatic, Alertness, Animal
  Affinity, Deceitful, Deft Hands, Magical Aptitude, Self-Sufficient, Stealthy)
  — identical real +2/+4 shape, but every one targets skills the engine does
  **not** compute (Perception, Fly, Bluff, Disguise, Spellcraft, Heal, Stealth,
  etc.). Wiring them would be unwired no-ops. They become groundable the moment
  those skills join the computed surface — not before.
- **Skill Focus** (`general.rs:55`, `["SKILL","%LIST","3",...]`) — `%LIST` is a
  player-chosen skill target with no slot in `selected_feats`; the same gap
  `feat_effects.rs:26-34` already documents. Deferred.
- **Spell Focus / Greater Spell Focus / Weapon Focus** — same chosen-target
  (`SCHOOL.%LIST` / weapon) problem, and Spell DC / attack totals are not
  generically consumed from `selected_feats` anyway.
- **Dodge** — already hardcoded as `DODGE_AC_BONUS` in the deterministic AC
  posture. Not a widening target (would be a refactor, out of scope).
- **Improved Initiative** and every initiative feat — no Initiative total is
  computed anywhere in the crate. Dead consumer.
- **CMB/CMD maneuver feats** (Improved/Greater Bull Rush, Disarm, Grapple,
  Overrun, Sunder, Trip, etc.) — no combat-maneuver total is computed. Dead
  consumer.
- **Equipment-gated AC feats** (Shield Focus, Greater Shield Focus, Two-Weapon
  Defense) — `PREEQUIP` shield / dual-wield; the deterministic posture carries
  no shield, so the effect is vacuously +0. Deferred.
- **Point-Blank Shot** — range-conditional (`SHORTRANGE`), no consumer.
- **Intimidating Prowess** (`combat.rs`, `["SKILL","Intimidate","STR"]`) — a
  genuine near-miss: Intimidate **is** computed, but the effect adds the STR
  **modifier**, not a flat constant, so `feat_effects.rs` would need to take
  `AbilityModifiers` as input (a new shape past the pure `selected_feats`
  lookup). Flagged as a possible fast-follow — see open question 3.
- **All Item Creation (8) and Metamagic (17) feats** — crafting-rule paragraphs
  and spell-slot mechanics; every ItemCreation record already carries
  `effect: None`. No flat numeric bonus to ground.

## Open questions for the lead

1. **Cross-lane wiring (blocker).** The only live consumer for skill-feat
   bonuses is `compute_selected_skill_modifiers` in `pilot_compute.rs` —
   backend's exclusive lane — exactly the split `save_bonuses_from_feats`
   already used (`feat_effects.rs` fn + `pilot_compute.rs` consumer). Shipping
   the `feat_effects.rs` function without the ~3-line consumer edit reproduces
   the item-17 gap (pick Athletic, Climb unchanged) and would be a no-stub
   violation. **How do you want the consumer edit handled — backend folds it
   into their next `pilot_compute` pass, or a narrow scoped grant to me for
   those exact lines in `compute_selected_skill_modifiers`?** I will not touch
   `pilot_compute.rs` without explicit direction.
2. **Is a 2-feat batch (Athletic, Persuasive) worth a cycle?** It is small only
   because the `Computed` surface is three skills — not because the feats are
   hard. If you'd rather I hold until more skills/values are computed (so a
   larger feat batch becomes live-consumable in one pass), that's a legitimate
   call; I'd rather surface it than ship a thin slice you didn't want.
3. **Intimidating Prowess?** It targets the computed Intimidate skill but adds
   the STR modifier (needs `AbilityModifiers` threaded into the feat_effects
   fn). Include it in this batch (accepting the slightly richer fn signature),
   or defer it with the rest?
