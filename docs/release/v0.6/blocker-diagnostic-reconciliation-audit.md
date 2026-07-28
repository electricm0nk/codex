# Blocker-diagnostic reconciliation audit (task #76)

Roster-wide reconciliation of every claim-blocking class diagnostic against
what is actually shipped. Produced on the red branch; **no code changed** —
this is the input for whoever applies the edits once #74 is green.

## Method and its limits

For each of the 16 Blocked classes I extracted the diagnostic's own message
text, then checked each item it names against three independent signals:

1. **A real function exists** in `pilot_compute.rs` (production section, above
   the first `#[cfg(test)]` at char 1,932,746 of 2,613,606).
2. **It has a live call site** — a non-definition invocation in the production
   section. Every function listed below has at least one; none are dead code.
3. **A test references it** — by name in the inline test module or under
   `tests/`.

**Limit on signal 3, stated up front:** a `tests=0` reading means "no test
calls this helper *by name*", not "untested". Most of these are exercised
through the compute entry point rather than directly, so I use signal 3 only as
corroboration where it exists, never as evidence of absence. Where a feature is
grounded inline rather than in its own named function (Cavalier's Challenge
damage, Inquisitor's Smiting judgment, Brawler's Martial Training), the test
name is the only by-name signal and I say so explicitly.

Verdicts are about **whether the diagnostic text is true**, not about whether
the class should be unblocked. A class can have fully accurate text and still
be correctly blocked.

## Summary

| Verdict | Classes |
|---|---|
| **Stale** — claims ungrounded what is shipped | Summoner, Brawler, Swashbuckler, Bloodrager, Slayer, Investigator, Witch, Cavalier, Monk (9) |
| **Internally contradictory** — two diagnostics disagree | Shaman (1) |
| **Accurate** — no correction needed | Alchemist, Warpriest, Skald, Hunter, Oracle, Inquisitor (6) |

**Correction to my own earlier report:** I listed Oracle in the drift table I
sent team-lead. That was an overclaim. Oracle's "the remaining Mystery
revelations" means the ones beyond its five grounded mysteries, which is true.
Oracle is accurate. I also under-reported Summoner, which turns out to be one of
the worst cases and which I had not flagged at all.

---

## Stale

### Summoner — 4 misstated, 1 omitted (not previously flagged)

Claims ungrounded: "Life Link, **Bond Senses**, Shield Ally and Greater Shield
Ally, **Maker's Call**, Transposition, **Twin Eidolon**, the **Summon Monster**
spell-like ability".

**Four misstatements.** Shipped with live call sites:
`summoner_bond_senses_rounds_per_day`, `summoner_makers_call_uses_per_day`,
`summoner_twin_eidolon_minutes_per_day`, `summoner_summon_monster_uses_per_day`
/ `_duration_minutes` / `_spell_level`. These are task #35's Slice A, and task
#37 already bumped `named_features_wired` to 6 to credit them — so the crate
credits them in one file and denies them in another.

**One omission, a different defect shape.**
`summoner_merge_forms_rounds_per_day` is shipped and called, and the diagnostic
does not mention Merge Forms **at all** — neither as grounded nor as remaining.
Team-lead asked whether this was a naming mismatch with what the diagnostic
calls "Transposition". It is not: `apg_abilities_class.lst` carries
`KEY:Summoner ~ Merge Forms` and `KEY:Summoner ~ Transposition` as two distinct
base-class records (PF1 levels 16 and 10 respectively). So the Transposition
claim is *true* — nothing grounds it — and Merge Forms is simply absent from a
message that purports to enumerate the class's state.

Genuinely still ungrounded, all verified as having no implementation: Biped and
Serpentine base forms, Aspect and Greater Aspect, Life Link, Shield Ally /
Greater Shield Ally, Transposition, Life Bond, Gate, Cantrips, spontaneous
Charisma spellcasting, and the 104-record evolution point-buy economy.

**Corrected clause:** move Bond Senses, Maker's Call, Twin Eidolon and Summon
Monster into the grounded clause, and add Merge Forms there too — it belongs in
the message and currently appears nowhere in it.

### Brawler — 6 of 10 claimed items are shipped

Claims ungrounded: Brawler's Flurry, Knockout, Martial Flexibility, Awesome
Blow, Improved Awesome Blow, Martial Training, Bonus Feats, Close Weapon
Mastery, Maneuver Training, Alignment Selection.

Shipped: `brawler_flurry_extra_attacks`, `brawler_knockout_dc` /
`_stat_bonus` / `_uses_per_day`, `brawler_martial_flexibility_uses`,
`brawler_bonus_feat_count`, `brawler_maneuver_training_count`, plus Martial
Training grounded inline (test `martial_training_grounds_its_three_level_equivalence_facts`).

Genuinely still ungrounded — zero evidence anywhere, matching team-lead's
independent spot-check exactly: **Awesome Blow, Improved Awesome Blow, Close
Weapon Mastery, Brawler's Strike Alignment Selection.**

**Corrected clause:** the remaining list is those four items only.

### Swashbuckler — Deeds are built

Claims: "Deeds (…Derring-Do, Dodging Panache, Menacing Swordplay, Precise
Strike, and the rest — **named but not built**)", plus Bonus Feats, Weapon
Training/Mastery, Grace/Edge.

Shipped: `ground_swashbuckler_deeds` (called at 14186),
`swashbuckler_derring_do_uses`, `_dodging_panache_bonus`,
`_precise_strike_damage`, `_bleeding_wound_damage`, `_stab_save_dc`,
`_deed_tier_reached` (4 calls, 3 tests), `_weapon_training_bonus`.

Genuinely still ungrounded: Swashbuckler Finesse (the feat-prerequisite
substitution hook — a real architectural gap), Bonus Feats, Weapon Mastery,
Swashbuckler's Grace/Edge, Menacing Swordplay.

**Corrected clause:** Deeds move to the grounded clause with the honest caveat
that a subset of deeds is grounded, not the whole chooser.

### Bloodrager — task #42's whole closure is denied

Claims ungrounded: Fast Movement, Uncanny Dodge, Blood Sanctuary, Damage
Reduction, the Greater/Tireless/Mighty Bloodrage tiers, and the Bloodline slot.

Shipped: `ground_bloodrager_remaining_features`,
`ground_bloodrager_damage_reduction` (11 test references on
`bloodrager_damage_reduction_amount`). The `named_features_wired = 9` comment in
`rules_tables/acg/mod.rs` enumerates these by name.

Genuinely still ungrounded: the Bloodline slot, and (per the separate
`spellcasting_deferred` diagnostic, which is accurate) the spell list itself,
Blood Casting, Eschew Materials, and bloodline bonus spells.

**Corrected clause:** the remaining list is the Bloodline slot only.

### Slayer — both named items are at least partly shipped

Claims ungrounded: Studied Target ("no target-creature representation exists
anywhere in this codebase") and Slayer Talents.

Shipped: `slayer_studied_target_bonus`, `slayer_studied_target_count`,
`slayer_talent_count` (each 1 call, 1 test) — landed in `3f44acdd`.

Nuance worth keeping in the corrected text: `slayer_talent_count` grounds the
**pool size only**. The individual talents in the 41-record chooser are still
unbuilt, so "Slayer Talents" is half-true rather than simply wrong — the same
pool-size-not-execution idiom Panache and Warpriest's Blessings already use.

**Corrected clause:** Studied Target moves to grounded; Slayer Talents is
restated as "pool size grounded, individual talents unbuilt". The
"no target-creature representation exists" sentence should go — it is the
premise that the standing scope-condition ruling retired.

### Investigator — Studied Combat/Strike shipped

Claims: "Studied Combat, Studied Strike (both opponent-dependent, **deferred
pending an opponent-tracking pillar**, ruled consistently with Slayer's own
Studied Target)".

Shipped: `investigator_studied_combat_bonus`, `_duration`,
`investigator_studied_strike_dice` (all tested by name). Studied Defense also
landed in `3f44acdd`.

Genuinely still ungrounded: Inspiration's actual spend, the remainder of
Investigator Talents, Keen Recollection, Poison Lore, Swift Alchemy, True
Inspiration.

**Corrected clause:** delete the opponent-tracking deferral sentence; move all
three Studied features to grounded.

### Witch — three separate stale claims

Claims ungrounded: (a) "fresh own-list spellcasting (Cantrips, Patron Spells…)",
(b) "the Familiar and Familiar Touch Spells (**an unbuilt subsystem**)",
(c) "the other ~18 base hexes plus the Major Hex/Grand Hex tiers".

Shipped: (a) `ground_witch_prepared_spells`, `witch_total_spells_per_day`,
`witch_base_spells_per_day_table`, `witch_spell_level_access` — plus the
326-record spell list from tasks #23/#33. (b) `ground_or_block_witch_class_features`
calls the shared `ground_familiar_master_benefit` at line 12435 — the same
class-agnostic machinery Arcanist (12832) and Shaman (15328) use. (c)
`witch_flight_hex_swim_bonus` grounds Flight.

Genuinely still ungrounded: ~17 remaining base hexes, the Major/Grand tiers,
Familiar Touch Spells specifically (distinct from the master benefit).

**This one has a decision attached.** My #71 doc recommended Flight as the
canonical Witch hex to build, and team-lead accepted my advice to defer it.
Flight was already built when I wrote that. The deferral rested on a false
premise and should be revisited on its merits.

### Cavalier — Challenge damage shipped

Claims ungrounded: "Challenge's own +level damage against its target".
Shipped in `3f44acdd`, grounded inline (test
`cavalier_challenge_damage_grounds_only_while_challenging` at 49281) rather than
in its own named function.

Genuinely still ungrounded, and correctly attributed to real missing engine
state: Banner and Greater Banner, the charge family, Demanding Challenge, the
five non-Sword Orders and their challenge riders, By My Honor.

**Corrected clause:** move Challenge damage to grounded; the rest of the
paragraph, including the "blocked on real missing engine state" reasoning,
stands as written.

### Monk — the premise sentence is false

Claims its bonus feat is blocked because the choice "requires a general
feat-selection or feat-prerequisite/effect engine that does not exist in this
bounded martial chassis baseline".

Of the 7 restricted-list options, **4 are grounded**: Dodge (the message's own
other branch already concedes this), Improved Grapple, Scorpion Style
(`monk_scorpion_style_dc`), and Combat Reflexes
(`monk_combat_reflexes_additional_attacks_of_opportunity`, called at 25921).

Genuinely ungrounded: Catch Off-Guard, Throw Anything, and Deflect Arrows —
the last genuinely blocked, with zero corpus tokens anywhere (confirmed in the
opponent-interaction design doc), not merely unbuilt.

**Corrected clause:** the blocker should name the three unbuilt options rather
than assert that no feat engine exists. Same correction shape as #52.

---

## Internally contradictory

### Shaman — its two diagnostics disagree with each other

- `class_feature.acg.shaman.spirit_powers.unsupported`: "**All ten primary
  Spirits** (Battle, Bones, Flame, Heavens, Life, Lore, Nature, Stone, Waves,
  Wind) **are recognized** through their own immediately-available base ability".
- `class_feature.acg.shaman.other_features_deferred.unsupported`: "**the other 9
  primary spirits** … remain ungrounded anywhere in this codebase".

Both ship; they cannot both be true. The evidence favours the first:
`ground_shaman_spirit_base_ability` plus `shaman_battle_spirit_bonus` (6 tests),
`shaman_monstrous_insight_bonus`, `shaman_spirit_touch_bonus_damage`,
`shaman_stardust_duration_rounds` / `_penalty`, `shaman_storm_burst_duration_rounds`
— magnitudes spanning well beyond Life Spirit alone.

**Corrected clause:** `other_features_deferred` should stop claiming the other
nine spirits are ungrounded and instead name what actually remains — Spirit
Magic, Manifestation, and the Hex/Spirit-Hex chooser.

---

## Accurate — no correction needed

**Alchemist.** Discovery, Swift Alchemy and Swift Poisoning have no
implementation anywhere; no alchemist function contradicts the text.

**Warpriest.** Only `warpriest_destructive_attacks_bonus` and
`warpriest_strength_surge_bonus` exist, exactly as the text says. The
"18 of the 20 Blessing types" count is right.

**Skald.** Every feature the text lists as grounded has a matching called
function; every one it lists as remaining has none.

**Hunter.** `hunter_animal_focus_bull_bonus` is the only focus grounded, as
claimed. Spellcasting is genuinely unbuilt (task #44 in flight).

**Oracle.** "The remaining Mystery revelations" correctly means those beyond
the five grounded ones, which `mystery_powers` enumerates. One minor
*under*-claim: the grounded clause names only Life Mystery's Healing Hands while
`ground_oracle_tier_one_revelations`, `active_oracle_natures_whispers_ac_bonus`,
`active_oracle_sidestep_secret_reflex_bonus` and `oracle_near_death_save_bonus`
also ship. Optional polish, not a defect.

**Inquisitor.** All judgments it claims grounded are real, including Smiting
(grounded inline; test at 41930). Both its diagnostics are consistent.

---

## Cross-cutting note carried forward

Three of the six "accurate" classes — Inquisitor, Hunter, Skald — plus Monk
contain the line "this class has no class-skill list". That is **true of the
engine and false of the corpus**: all four carry Climb, Intimidate and Swim in
their real `CSKILL:` records and are absent from all three
`selected_skill_*_is_class_skill` predicates. That is the separate verified
defect from the punch-list, not stale text, and it should be fixed in code
rather than in prose.

## What this does and does not establish

- It establishes that 10 of 16 diagnostics misstate the state of the codebase,
  and gives the corrected content for each.
- It does **not** establish that any class is ready to unblock. Every class
  audited still has genuinely ungrounded features behind its catch-all; the
  corrections shrink the lists, they do not empty them. The closest are
  Bloodrager (Bloodline slot only) and Brawler (four items).
- Verdicts rest on function-existence plus a live call site. I did not execute
  the test suite — the branch is red — so "shipped" here means present and
  wired, not independently re-verified as passing.
