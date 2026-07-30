# Blocker Backlog — Comparative Sizing of the 6 Unassessed Tasks

> Operator directive: add TODO tasks for all blockers and prioritize
> addressing blocked work. The lead created 17 tasks, tagged 3 low from
> prior SWARM_TASKS.md findings (Bloodrager/Alchemist/Cavalier — big spell
> lists or chooser sets) and several more low from known architecture
> gaps, leaving **6 genuinely unassessed "needs-scoping" tasks**:
> #2 Hunter, #3 Inquisitor, #5 Brawler, #7 Skald, #8 Investigator,
> #16 feat-effects. This pass sizes those 6 the same way as the
> Investigator / Shaman-Summoner-Witch re-scopings — finding each one's
> actual remaining feature list and assessing genuine boundedness from the
> corpus and code, not from labels. Summoner (#17) is explicitly out of
> bounds pending the operator's own decision and is not sized here.

## Method

For each task, read the class's real remaining `KEY:<Class> ~ ...` records
in `{apg,acg}_abilities_class.lst`, extracted each candidate feature's own
`BONUS:VAR`/`BONUS:*` formula, and classified it: **flat standalone fact**
(cheap, the Trapfinding/Nimble idiom), **mechanism-reuse** (grounds by
reusing an already-built shape), **chooser-list** (defer or narrow to one
canonical), **feat-prereq no-op** (no numeric effect, honest defer), or
**needs-new-machinery** (a real architecture lift). Spellcasting reuse was
checked by grepping each class's `SPELLLIST:` token against the
already-built spell-list modules.

## #3 Inquisitor — the cheapest, highest-value slice of the six

**The lead's "do the 7 judgments reuse Justice's mechanism?" angle is
confirmed real.** Every judgment record carries the identical structural
anchor `BONUS:VAR|InqJudge<Type>LVL|InquisitorLVL` — the same shape Justice
(`InqJudgeJusticeLVL`) already grounds through its activation-gated,
uses-per-day-budget-enforced mechanism (`active_inquisitor_justice_judgment_bonus`).
The 8 non-Justice judgments (Destruction, Healing, Piercing, Protection,
Purity, Resiliency, Resistance, Smiting) are all flat, self-scoped bonuses
toggled by the *same* single judgment activation — they reuse Justice's
mechanism directly, differing only in bonus value and target pillar:
- Targets a **computed pillar** → integrates like Justice's attack bonus
  (e.g. any save-targeting judgment into the saves pillar).
- Targets an **uncomputed pillar** (AC, damage, DR, energy resistance,
  caster-level check, weapon properties) → grounds as a standalone flat
  fact, the Nimble/Destructive-Attacks idiom this engine already uses.
- **Build-time caveat (flag, don't assume):** each judgment's exact bonus
  *value* formula (e.g. Destruction's sacred-damage scaling, Protection's
  AC step) is not a literal `BONUS` on the ability record — it resolves
  through the shared judgment machinery, same as Justice's `1+LVL/5` did.
  Each must be re-derived from its own record/RAW at build time, not
  copied from Justice's `/5`. But the *mechanism* is provably reused.

**Plus four genuinely-flat standalone features**, all verified directly:
- **Monster Lore** — `BONUS:VAR|MonsterLoreBonus|WIS` (+WIS on Knowledge
  to identify monsters). Flat.
- **Stern Gaze** — `BONUS:VAR|SternGazeBonus|max(1,InquisitorLVL/2)` on
  Intimidate/Sense Motive. Flat, the exact `max(1,lvl/2)` idiom.
- **Cunning Initiative** — `BONUS:COMBAT|INITIATIVE|WIS` (+WIS to
  initiative). Flat.
- **Track** — `BONUS:VAR|TrackLVL|InquisitorLVL` (+level/2 on Survival,
  same as Slayer's own grounded Track). Flat.

**Domain** grants no power per RAW (free out-of-scope, as the task notes).
**Spellcasting** (WIS spontaneous, `MEMORIZE:NO`, **no `SPELLLIST` reuse
token** → own fresh list) stays deferred. **Verdict: the richest cheap
slice of the six** — 4 flat facts + 7 mechanism-reuse judgments, only
spellcasting deferred. Could lift `named_features_wired` substantially.

## #2 Hunter — real spellcasting reuse + one cheap flat feature

- **Spellcasting reuses ALREADY-BUILT lists**: `SPELLLIST:2|Druid|Ranger`
  — both `druid_spell_list.rs` and `ranger_spell_list.rs` exist. Hunter is
  WIS spontaneous (`MEMORIZE:NO`), so it needs a spontaneous known-spell
  table + per-day shape, but that machinery already exists (Skald/Sorcerer
  spontaneous-known). A real lift, but genuinely reuse-backed — closer to
  Skald's spellcasting closure than a from-scratch list.
- **Wild Empathy** — `BONUS:VAR|HunterWildEmpatyBonus|CHA+HunterLVL`. Flat
  (CHA + level), the Druid/Ranger Wild Empathy idiom. Cheap standalone.
- **Animal Focus** — `BONUS:ABILITYPOOL|Hunter Animal Focus|1`: a
  chooser-list (emulate an animal for a flat aspect bonus). Narrow to one
  canonical focus (Oracle-Mystery style) or defer.
- **Nature Training** — no numeric `BONUS` (counts as Druid/Ranger level
  for feat/option prereqs). Feat-prereq no-op; honest defer, no hook.
- **Verdict: a moderate bounded slice** — one cheap flat feature (Wild
  Empathy) plus a reuse-backed spellcasting lift; Animal Focus narrows,
  Nature Training defers cleanly.

## #7 Skald — two clean flat wins, the rest choosers

- **Bardic Knowledge** — `BONUS:VAR|BardicKnowledgeSkillBonus|max(1,SkaldLVL/2)`
  on all Knowledge skills. Flat, cheap.
- **Well-Versed** — `BONUS:VAR|SkaldWellVersedBonus|4` (+4 vs
  sonic/language-dependent effects). Flat, cheap.
- **Lore Master** — `SkaldLoreMasterUsesPerDay|min((SkaldLVL-1)/6,…)`: a
  uses-per-day pool (take 10/20 on Knowledge). Pool *size* is a flat fact
  (Panache-shaped); the take-10/20 effect isn't numeric — ground the pool
  size only.
- The other ~15 (Rage Powers, the Raging Song variants, Spell Kenning,
  Bonus Feats, etc.) are chooser-lists or complex mechanics — defer.
- **Verdict: a cheap 2–3-feature mini-slice** (Bardic Knowledge +
  Well-Versed + optional Lore Master pool), deferring the bulk. Bounded
  but modest in count — real, honest, small.

## #8 Investigator — a bounded-but-substantial shared-subsystem investment

Investigator's flat features (Trapfinding/Trap Sense/Inspiration pool) are
already grounded; **this task is specifically the spellcasting subsystem.**
It requires building the shared **Alchemist formula spell-list** — a new
`alchemist_spell_list.rs` `(&str, u8)` mapping (the `cleric_spell_list.rs`
shape) ingested from the **104 real `Alchemist=N` records** in
`apg_spells.lst` — then wiring Investigator's prepared-caster validation
(reuse Wizard/Arcanist's prepared shape; Investigator is `SPELLSTAT:INT
MEMORIZE:YES SPELLBOOK:YES`). **Not a cheap slice** — a real ingestion +
wiring effort, roughly Warpriest-sized (per the earlier Investigator
scoping doc). **But high leverage**: the list, once built, also unblocks
Alchemist's own deferred spellcasting (#4), so it is a shared investment,
not a single-class cost. **Verdict: bounded and substantial; the highest-
leverage of the non-cheap options because it unblocks two classes.**

## #5 Brawler — cheap features already spent; remainder needs machinery

Brawler's flat wins (AC Bonus, Cunning, Strike) are already wired
(`named_features_wired=3`). The remaining 9 are mostly not flat:
- **Maneuver Training** — `BONUS:ABILITYPOOL|Maneuver Training I Selection`:
  a chooser (pick a maneuver for a CMB bonus). Chooser + CMB pillar (this
  engine computes no CMB total).
- **Martial Training** — `FighterWeaponQualifyLVL`/`MonkFeatQualify` (counts
  as fighter/monk level for feat prereqs). Feat-prereq no-op; defer.
- **Knockout** — a DC-gated fall-unconscious effect (needs a target/save
  mechanic). Not a flat fact.
- **Close Weapon Mastery** — a close-weapon damage-die upgrade (Sacred-
  Weapon-shaped, flat-ish but low value, like Warpriest's near-zero die).
- **Martial Flexibility, Awesome Blow, Bonus Feats** — chooser/feat-grant/
  combat-maneuver machinery.
- **Verdict: the thinnest remaining flat value of the six.** The cheap
  features were already the right first pick; the rest need chooser or
  combat-maneuver/CMB machinery. Lower priority for a bounded slice.

## #16 Feat-effects — blocked or design-gated, not a bounded grounding

Per the existing `feat-effects-widening-scoping.md` (re-confirmed): 7 of
185 CRB feats are wired, and the two named next targets are both gated:
- **Option 1 — the other 8 two-skill General feats**: blocked on the
  **wiring-consumer constraint** — their target skills (Perception, Bluff,
  Fly, …) are not on the computed surface, so there is **no live consumer**
  to make grounding them meaningful. Requires skill-surface expansion
  first (not this task).
- **Option 2 — Skill/Weapon/Spell Focus chosen-target mechanism**: "the
  single biggest locked door," needs a real design conversation on how to
  record a player-chosen target in `selected_feats` — a design task, not a
  bounded widening.
- **Verdict: not a near-term bounded slice.** Either dependency-blocked
  (skills) or design-gated (chosen-target). Belongs behind a skill-surface
  or design decision, consistent with the lead's own framing.

## Comparative ranking

| Task | Cheap flat wins | Reuse / leverage | Boundedness | Recommended priority |
|---|---|---|---|---|
| **#3 Inquisitor** | 4 (Monster Lore, Stern Gaze, Cunning Init, Track) | 7 judgments reuse Justice's mechanism | High — biggest cheap grounding | **1st (best value)** |
| **#7 Skald** | 2–3 (Bardic Knowledge, Well-Versed, Lore Master pool) | — | High but modest count | 2nd (cheap, small) |
| **#2 Hunter** | 1 (Wild Empathy) | Spellcasting reuses built Druid+Ranger lists | Moderate | 3rd |
| **#8 Investigator** | 0 (already spent) | Alchemist list unblocks Alchemist too | Bounded but ~Warpriest-sized | 4th — deliberate high-leverage slice |
| **#5 Brawler** | ~0–1 (Close Weapon die) | — | Remainder needs chooser/CMB machinery | Low |
| **#16 Feat-effects** | — | — | Blocked (skills) or design-gated (Focus) | Hold — needs a prior decision |

## Recommendation

Build **Inquisitor (#3) first** — it is the clearest cheap, high-grounding
slice of the six: four verified flat standalone features plus seven
judgments that reuse Justice's already-built activation-gated mechanism,
with only fresh-list spellcasting deferred. Follow with **Skald (#7)** and
**Hunter (#2)** as smaller/moderate bounded slices (Skald's two flat wins;
Hunter's one flat win plus reuse-backed spellcasting). Treat **Investigator
(#8)** as a deliberate, high-leverage subsystem slice whenever a
Warpriest-sized effort fits — it unblocks Alchemist's spellcasting too.
**Deprioritize Brawler (#5)** (cheap features already spent; remainder
needs new machinery) and **hold feat-effects (#16)** pending either
skill-surface expansion or a chosen-target design decision.

## Open questions for the lead

1. **Inquisitor (#3)** greenlit as the first pick? And is grounding the 7
   judgments' *mechanism* (deferring only their exact per-judgment bonus
   values to build-time re-derivation) the right scope, or do you want each
   judgment's value fully verified before any land?
2. **Investigator (#8) / Alchemist (#4) sequencing** — build the shared
   Alchemist formula list under Investigator's task (unblocking both), or
   split the list-ingestion into its own task that both then depend on?
3. **Feat-effects (#16)** — hold pending a skill-surface decision, or do
   you want the Focus chosen-target *design* conversation scoped as its own
   task now (it's the biggest single locked door in feat-effects)?
4. **Skald (#7) vs Hunter (#2)** ordering — Skald is cheaper but smaller;
   Hunter is a bigger lift but reuse-backed and adds a full spellcasting
   pillar. Preference?
