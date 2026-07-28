# Blocked-classes ranked punch-list (task #74)

Scope: all 16 still-Blocked classes. One consolidated picture of what is
actually left per class, ranked by leverage (features unblocked per unit of
build effort) rather than by book or alphabet.

Every number below was re-derived this session from the raw PCGen corpus and
the shipped crate. No count is inherited from a prior scoping doc, including
my own.

## Method

- **Corpus side.** Base-class records only, screened by KEY prefix
  (`KEY:<ClassName> ~ …`), never by `CATEGORY:`/`TYPE:` — the Sacred Servant
  false negative makes type-screening unsafe. Sources: `cr_abilities_class.lst`,
  `cr_abilities.lst`, `apg_abilities_class.lst`, `apg_abilities.lst`,
  `acg_abilities_class.lst`, `acg_abilities_other.lst`, plus the three
  `*_classes.lst` files. `#`-prefixed records excluded.
- **Code side.** Grounded ids enumerated fresh via
  `grep -oh '"[a-z_]*\.[a-z0-9_.]*"' src/rules_core/pilot_compute.rs`, not from
  a previously "corrected" prefix list. That sweep turned up two id prefixes
  absent from the last enumeration, which is exactly why the list is re-derived
  rather than recalled.
- **Blocker side.** ~~The authoritative statement of what is left for a class is
  its own claim-blocking diagnostic in `pilot_compute.rs`, not any status doc.~~
  **This was wrong and it invalidated the first version of this document's
  ranking. See "Correction" below.** Blocker diagnostic prose has drifted
  badly from shipped reality across most of the roster. What is left for a
  class must be derived from the *code* — the grounding functions that actually
  exist and are actually called — and cross-checked against the corpus. The
  diagnostic text is now itself a defect surface, not a source of truth.

## Correction (2026-07-28)

The first version of this doc ranked the roster using each class's own
claim-blocking diagnostic message as the statement of remaining scope. That was
a methodological error, and team-lead caught the largest consequence of it.

I asserted (item 3, my headline recommendation) that Slayer's Studied Target and
Investigator's Studied Combat/Studied Strike were still deferred pending an
opponent-tracking pillar and could be grounded today. **They were already
grounded, with tests, in commit `3f44acdd` earlier the same day** — along with
Investigator's Studied Defense and Cavalier's Challenge damage. That commit's own
reasoning matches the re-derivation I did almost word for word. There was no work
to do.

Two things went wrong, and the second is worse than the first:

1. I read the deferral claim out of the blocker message text — immediately after
   proving, in item 2, that blocker message text can be stale. I applied that
   skepticism to Bloodrager and not to Slayer/Investigator. Same failure shape as
   the Cleric Domains archetype error: I verified the finding that looked
   suspicious and skipped the one that looked good.
2. **My own census already contained the contradiction.** The code-based census
   in this document does not list Studied Target, Studied Combat or Studied
   Strike as remaining for either class — because it matched them in the shipped
   code. I had the disconfirming evidence in my own output and let the prose
   override it.

Re-auditing on the corrected basis turned up that the drift is systemic, not
confined to those three features. See "Cross-cutting item 2" below, which now
supersedes both the original items 2 and 3.

### The counting basis, stated plainly

`named_features_expected` in `rules_tables/{apg,acg}/mod.rs` counts
**exact-segment base-class records only** — `KEY:Skald ~ …`, not
`KEY:Skald Versatile Performance ~ …`. The chooser families (Hexes, Domains,
Mysteries, Talents, Deeds, Discoveries, Blessings, Orders, Bloodlines, Rage
Powers) sit **outside** that denominator entirely. This matters for reading the
table below: a class at 9/19 is not "half done", because the 19 excludes the
chooser layer that is usually the larger remaining cost.

## The single most important structural finding

**Every one of the 16 classes is gated by exactly one unconditional
claim-blocking diagnostic** — a catch-all `other_features_deferred`
(or `evolutions_deferred` / `spellcasting_deferred` / `bonus_feat`). Inquisitor
alone has two (`domain_powers` plus `other_features_deferred`).

The consequence is the thing to internalize when answering "why hasn't more
reached Computed": **partial progress on a class produces zero Computed
classes.** Grounding 13 of a class's 14 magnitude-bearing records leaves the
catch-all firing exactly as hard as grounding none of them. The roster has been
accumulating real, verified work that cannot show up as a Computed class until
some class's catch-all is retired in full.

That is a sequencing fact, not a criticism of the work done. But it means
leverage ranking should favour **classes whose catch-all can be fully retired**,
not classes with the most remaining magnitudes.

## Per-class census

Magnitude-bearing = base-class record carrying a `BONUS:`/`DR:`/`SPELLKNOWN:`/
`TEMPBONUS:` token. `wired` is the shipped `named_features_wired`.

| Class | wired/expected | mag-bearing base records | sole unconditional blocker |
|---|---|---|---|
| Monk | (CRB) | 14 | `monk.bounded_progression.bonus_feat` |
| Brawler | 9/14 | 8 | `acg.brawler.other_features_deferred` |
| Skald | 8/20 | 9 | `acg.skald.other_features_deferred` |
| Inquisitor | 5/19 | 5 | `inquisitor.domain_powers` **and** `apg.inquisitor.other_features_deferred` |
| Swashbuckler | 12/29 | 15 | `acg.swashbuckler.other_features_deferred` |
| Investigator | 9/95 | 16 | `acg.investigator.other_features_deferred` |
| Bloodrager | 9/19 | 9 | `acg.bloodrager.other_features_deferred` |
| Slayer | 6/15 | 8 | `acg.slayer.other_features_deferred` |
| Warpriest | 5/18 | 9 | `acg.warpriest.other_features_deferred` |
| Oracle | 5/19 | 18 | `apg.oracle.other_features_deferred` |
| Cavalier | 6/16 | 8 | `apg.cavalier.other_features_deferred` |
| Summoner | 6/17 | 9 | `apg.summoner.eidolon.evolutions_deferred` |
| Alchemist | 3/24 | 18 | `apg.alchemist.other_features_deferred` |
| Hunter | 3/21 | 8 | `acg.hunter.spellcasting_deferred` |
| Shaman | 2/10 | 5 | `acg.shaman.other_features_deferred` |
| Witch | 2/7 | 4 | `apg.witch.other_features_deferred` |

Investigator's 95 expected is a corpus artifact, not 95 features: the
`KEY:Investigator ~ …` namespace carries its extract/formula plumbing. Only 16
of the 95 carry a magnitude. Read the mag-bearing column, not the denominator.

## Ranked punch-list

**The tier narrative below is superseded in part.** It was written from blocker
prose and is preserved for its per-class detail, but where a tier placement
rested on a class being "unbuilt", cross-check the drift table above first —
Brawler and Swashbuckler in particular were placed in Tier C on claims that turn
out to be false. The corrected ranking basis is the code census immediately
below.

### Corrected basis: base-class magnitudes actually remaining

Derived from code, not prose. "Remaining" = base-class magnitude-bearing corpus
records with no corresponding grounding in `pilot_compute.rs`. The chooser
families sit outside this column.

| Class | mag-bearing | grounded | remaining |
|---|---|---|---|
| Inquisitor | 5 | 5 | **0** |
| Brawler | 8 | 8 | **0** |
| Skald | 9 | 9 | **0** |
| Monk | 14 | 13 | **1** (a plumbing record, not a feature) |
| Witch | 4 | 3 | 1 |
| Cavalier | 8 | 6 | 2 |
| Summoner | 9 | 7 | 2 |
| Swashbuckler | 15 | 13 | 2 |
| Shaman | 5 | 3 | 2 |
| Warpriest | 9 | 7 | 2 |
| Hunter | 8 | 5 | 3 |
| Slayer | 8 | 4 | 4 |
| Bloodrager | 9 | 5 | 4 |
| Investigator | 16 | 10 | 6 |
| Oracle | 18 | 11 | 7 |
| Alchemist | 18 | 7 | 11 (9 of them Mutagen tiers = one mechanism) |

The base-class magnitude layer is **essentially exhausted** for Inquisitor,
Brawler, Skald and Monk, and nearly so for most of the rest. What genuinely
remains across the roster is (a) the chooser families, (b) execution
architecture, (c) the class-skill gap in cross-cutting item 1 — not more
base-feature grounding.

This is the honest answer to "why hasn't more reached Computed": it is not that
the features aren't built. For several classes they largely are. The catch-all
diagnostics have not moved to match.

I am explicitly *not* concluding that any catch-all is retirable today —
the chooser and execution items behind them are real. But the blocker prose can
no longer be used to tell which is which, so per-class reconciliation
(cross-cutting item 2) is now the prerequisite for any further ranking.

Original ranking rationale follows, with the caveat above.

### Tier A — retirable catch-all, no architecture blocker

**1. Hunter.** Its blocker names spellcasting (task #44, in flight), the
class-skill list (see cross-cutting item 1 — cheap), Nature Training (a real
record with *zero* numeric tokens; grounds as a named no-op on the Oracle-Haunted
precedent, or stays named-and-deferred), Precise Companion, and 12 further
Animal Focus options (canonical narrowing, already precedented — Bull is done).
**Nothing on Hunter's list requires new architecture.** It is the only class of
which that is true outright.

**2. Monk.** 13 of 14 magnitude-bearing records already grounded — the most
complete class on the blocked list. Its sole blocker is the level-1 bonus feat
drawn from a 7-option restricted list, and 3 of those 7 (Dodge, Improved
Grapple, Scorpion Style) are *already grounded elsewhere in the crate*. Deflect
Arrows is genuinely blocked (zero corpus tokens anywhere — confirmed in the
opponent-interaction design doc) but it is one *option*, not a required feature,
so option-level narrowing applies. Plus the class-skill gap below.

**3. Summoner.** Single bounded blocker (`evolutions_deferred`), already fully
scoped in `summoner-eidolon-mvp-scoping.md`: Quadruped is canonical, every
base-form evolution is `AUTOMATIC`. One narrowing task, no open questions.

**4. Witch.** Spellcasting already landed (#23/#33). What remains is the
Familiar — whose machinery (`ground_familiar_master_benefit`) is **already
shipped and already class-agnostic**, reused by Shaman and Arcanist — plus one
canonical hex. `hexes-shared-verification-scoping.md` recommends Flight (the
only Witch hex landing on a computed total). Smallest remaining surface of any
class on the board.

### Tier B — chooser-narrowing, precedented, medium cost

**5. Slayer** and **6. Investigator.** Both are gated on features their own
diagnostics still describe as "deferred pending an opponent-tracking pillar" —
Studied Target, Studied Combat, Studied Strike. **That deferral is stale.** See
cross-cutting item 3: all three carry real magnitudes derived purely from the
character's own level and INT, with no opponent property anywhere in the
formula. Under the standing scope-condition ruling they ground today, with no
new architecture. Three features across two classes for one bounded task.

**7. Bloodrager.** Blocker text is stale (cross-cutting item 2). Real remaining
scope is the Bloodline slot; the Draconic ladder is already scoped for Sorcerer
(#60/#61) but Bloodlines are **parallel, not shared**, so that work does not
carry over — Bloodrager needs its own.

**8. Cavalier.** Banner/Greater Banner look flat and cheap; the 5 non-Sword
Orders are standard narrowing. The charge family and every challenge rider stay
blocked on real missing engine state — leave them named.

**9. Warpriest.** 18 of 20 Blessings remain (2 done, narrowing precedented).
**Check task #72 before scheduling this** — see cross-cutting item 4.

**10. Oracle.** 11 of 18 magnitudes grounded already. The remainder is dominated
by the Mystery revelation chooser (10 mysteries × ~11 records), the single
largest chooser attached to any one class. Cure/Inflict conversion mirrors an
unmodelled Cleric gap and should stay deferred.

**11. Shaman.** Spellcasting landed. Remaining is Spirit Magic (spell layering),
Manifestation, 9 primary spirits, and the 62-record Hex/Spirit-Hex family
(scoped in #71; recommended for deferral there and I still recommend that).

**12. Alchemist.** Largest raw magnitude gap on the board (18 records, 7
grounded) — but **that gap is misleading**: 9 of the 11 remaining are the
Mutagen matrix (Str/Dex/Con × First/Second/Third), which are tiers of one
already-grounded mechanism and count once under the standing methodology. Real
remaining: the 48-record Discovery chooser, Swift Alchemy, Swift Poisoning,
Grand Discovery.

### Tier C — dominated by genuinely-missing architecture

**13. Swashbuckler.** Deeds narrow cleanly, but Swashbuckler Finesse needs a
feat-prerequisite-substitution hook that does not exist. Note the
`SwashbucklerDeedQualifyLVL` corpus defect recorded in its scoping doc.

**14. Skald.** 9 of 9 magnitude-bearing records already grounded. Almost
everything named in its blocker is *execution* — DR ally-extension, Spell
Kenning's cross-class borrowing, Lore Master's take-10/20, Versatile
Performance's skill substitution. Cheap class-skill fix aside, this class is
architecture-bound.

**15. Brawler.** 8 of 8 magnitudes grounded. Remaining (Flurry, Knockout,
Martial Flexibility, Awesome Blow, Maneuver Training) is execution-shaped.

**16. Inquisitor.** Only class with **two** unconditional blockers, so it needs
two full closures rather than one. Task #47 in flight addresses one side; the
domain-powers side is separate even though #64 already generalized Touch of Good
to Inquisitor.

## Cross-cutting items

These are not per-class work; each touches several classes at once and each is
cheaper than any class closure above.

### 1. Four classes are missing real class-skill bonuses (verified defect)

Only three skills are modelled as computed totals (Climb, Intimidate, Swim), via
`selected_skill_{climb,intimidate,swim}_is_class_skill` in `pilot_compute.rs`.
I diffed each blocked class's real corpus `CSKILL:` list against those three
predicates. Twelve of the sixteen match exactly. Four do not:

| Class | Climb | Intimidate | Swim |
|---|---|---|---|
| Monk | corpus Y / shipped **—** | corpus Y / shipped **—** | corpus Y / shipped **—** |
| Inquisitor | corpus Y / shipped **—** | corpus Y / shipped **—** | corpus Y / shipped **—** |
| Hunter | corpus Y / shipped **—** | corpus Y / shipped **—** | corpus Y / shipped **—** |
| Skald | corpus Y / shipped **—** | corpus Y / shipped **—** | corpus Y / shipped **—** |

All four carry all three skills in their real corpus lists, verified against raw
`CSKILL:` text, not inferred. That is 12 missing +3 class-skill bonuses. The fix
is one match arm per class per predicate.

Honest framing: because all four classes are claim-blocked anyway, this is a
**latent** defect rather than a wrong number a user can currently save. But three
of the four are Hunter, Monk and Inquisitor — the classes nearest to unblocking
— so it becomes user-visible precisely when they close. Fix it before, not after.

#### Implementation scope for this fix (added 2026-07-28)

The three predicates are at `pilot_compute.rs:33024`
(`selected_skill_climb_is_class_skill`), `:33048` (`_intimidate_`) and `:33071`
(`_swim_`). Each needs one added match arm per class — 12 arms total, Monk /
Inquisitor / Hunter / Skald across all three.

**The hazard is not breakage, it is inertness.** I checked for tests asserting
the current (wrong) behaviour and found none: no test suite for any of the four
classes allocates skill ranks at all, and no test references `skill:climb`,
`skill:swim` or `skill:intimidate` for them. So the change breaks nothing —
and equally, **nothing in the existing suite proves it works.** Shipping it
against a green run would be an uninformative pass of exactly the shape flagged
in the standing "a perturbation that fails to fire proves nothing" rule.

Whoever takes this must add fixtures that allocate ranks in Climb/Intimidate/Swim
for each of the four classes and assert the +3 lands, then perturb (remove the
class arm) and confirm the assertion fails. Without that, the fix is
unverifiable rather than verified.

Also worth recording: Bloodrager's class-skill grounding rests on **DESC prose,
not a `CSKILL:` token** — ACG carries no `CSKILL:` on any class line, and
Bloodrager's ability record encodes its 11 skills only in `DESC:`. That is the
weaker Panache-shaped evidentiary path. It is already shipped and I am not
challenging it; it should just be known to be that, not a token.

### 2. Blocker diagnostic prose has drifted from shipped reality, roster-wide

This supersedes the original items 2 and 3. It is the single largest obstacle to
classes reaching Computed, and it is not a build problem — it is a
truth-in-diagnostics problem.

Enumerating the grounding functions that actually exist and are actually called
in `pilot_compute.rs`, then comparing against what each class's blocker claims
"remains ungrounded":

| Class | Blocker claims ungrounded | Actually shipped and called |
|---|---|---|
| Bloodrager | Fast Movement, Uncanny Dodge, Blood Sanctuary, DR, Greater/Mighty tiers | `ground_bloodrager_remaining_features`, `ground_bloodrager_damage_reduction` (task #42) |
| Slayer | Studied Target | `slayer_studied_target_bonus`, `slayer_studied_target_count` (`3f44acdd`) |
| Investigator | Studied Combat, Studied Strike | `investigator_studied_combat_bonus`/`_duration`, `investigator_studied_strike_dice` (`3f44acdd`) |
| Cavalier | Challenge's own +level damage against its target | grounded in `3f44acdd` |
| **Brawler** | Flurry, Knockout, Martial Flexibility, Martial Training, Bonus Feats, Maneuver Training | `brawler_flurry_extra_attacks`, `brawler_knockout_dc`/`_stat_bonus`/`_uses_per_day`, `brawler_martial_flexibility_uses`, `brawler_maneuver_training_count`, `brawler_bonus_feat_count` |
| **Swashbuckler** | "Deeds … named but not built", Weapon Training/Mastery | `ground_swashbuckler_deeds` (called at 14186), `swashbuckler_derring_do_uses`, `_dodging_panache_bonus`, `_precise_strike_damage`, `_bleeding_wound_damage`, `_stab_save_dc`, `_weapon_training_bonus` |
| Monk | bonus-feat options need "a feat engine that does not exist" | 4 of the 7 options grounded: Dodge, Improved Grapple, Scorpion Style, and `monk_combat_reflexes_additional_attacks_of_opportunity` |
| Shaman | "the other 9 primary spirits … remain ungrounded" | `shaman_battle_spirit_bonus`, `_monstrous_insight_bonus`, `_stardust_duration_rounds`/`_penalty`, `_storm_burst_duration_rounds`, `_spirit_touch_bonus_damage` |
| Oracle | "the remaining Mystery revelations" | `ground_oracle_tier_one_revelations`, `active_oracle_natures_whispers_ac_bonus`, `active_oracle_sidestep_secret_reflex_bonus` |
| Witch | "the other ~18 base hexes … remain ungrounded" | `witch_flight_hex_swim_bonus` (called at 33242) |

Brawler and Swashbuckler are the worst two: both are described by their own
diagnostics as essentially unbuilt, and both have most of the named features
shipped and wired. All call sites above were checked to be live, not dead code.

**This also corrects one of my own recommendations.** The Hexes scoping doc (#71)
recommended Flight as the canonical Witch hex to build, and that recommendation
was accepted and deferred on my advice. Flight is already built. The deferral
decision was made on a premise that was false when I wrote it.

The fix is per-class reconciliation of diagnostic text against shipped code —
mechanical, text-only, no new grounding. It should precede any ranking decision,
because until it is done nobody (including me) can read a class's blocker and
know what is actually left.

### 4. Task #72 may already unblock Warpriest's Sacred Weapon

Warpriest's blocker attributes Sacred Weapon's active enhancement, and roughly
15 Blessing powers, to "a weapon-enhancement activation surface and a summon
subsystem" that do not exist. Task #72 is currently building a weapon-ingestion
pillar and per-weapon attack total. Whether that pillar supplies the
weapon-enhancement surface Warpriest names is a question I could not settle from
the outside, and it should be asked of #72's owner **before** Warpriest is
scheduled — if it does, Warpriest moves up several places.

## Recommended order

1. Cross-cutting 3 (Studied Target / Studied Combat / Studied Strike) — three
   features, two classes, no new architecture, ruling already made.
2. Cross-cutting 1 (class-skill predicates for Monk/Inquisitor/Hunter/Skald) —
   verified defect, one-line-per-arm fix.
3. Cross-cutting 2 (Bloodrager stale text) — free, and an honesty bug.
4. Hunter to closure, once #44 lands.
5. Monk to closure.
6. Summoner, then Witch.

**Revised order (2026-07-28), superseding the six items above:**

1. **Cross-cutting 2 — reconcile blocker text against shipped code, all 16
   classes.** Text-only, no new grounding, mechanical. This has to go first:
   until it lands, no ranking of remaining work can be trusted, including this
   document's. Start with Brawler and Swashbuckler — both are described by their
   own diagnostics as unbuilt and are largely built.
2. **Cross-cutting 1 — class-skill predicates for Monk/Inquisitor/Hunter/Skald.**
   Verified corpus defect, one match arm per class per predicate.
3. Re-rank once (1) lands and the diagnostics say something true, then take
   whichever class's catch-all is genuinely closest to retirable.

The former first item — ground Studied Target/Combat/Strike — is **withdrawn**;
that work was already done in `3f44acdd`. Hunter/Monk/Summoner/Witch remain
plausible closure candidates but their placement rested on blocker prose and
should be re-derived after step 1.

Neither remaining item should start against a red branch (a Barbarian/Skald
regression from #54 is in flight).

## Caveats

- I did not verify that retiring a class's catch-all is *sufficient* for
  Computed — only that it is the one unconditional class-level blocker. Other
  pillars (combat, defense, skills) have their own diagnostics that I did not
  trace per class.
- The mag-bearing counts are magnitude-token counts, not
  `named_features_wired`-equivalents; facets of one mechanism count separately
  here and would fold down under the wired methodology. Alchemist's Mutagen
  matrix is the clearest case and is called out inline.
- Effort ordering within Tier B is a judgment call on comparable-looking
  candidates, not a measured difference.
