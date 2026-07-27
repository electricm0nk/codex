# v0.6 Alpha Release Swarm — Report

Status: DRAFT (living document, updated as waves complete). Not an attestation yet.

Owner of this document: lead (orchestrator) collates; QA owns the attestation
content per §4.4 and §7.1 of `docs/release/v0.6/release-swarm.md`.

## Agent Status (operator directive, 2026-07-24: updated on every write to
this report, not just SWARM_STATUS.md)

| Agent | Status | Detail |
|---|---|---|
| backend | in_progress | Closed #10, #19, #1 (Bloodrager, both halves), and #20 (`effective_feats` wiring, honestly self-corrected to a no-op-today finding, real gap filed as #22) — all lead-verified. **Cavalier (#6) closed (`b5284379`)** — Challenge (uses/day + self `-2` AC penalty integrated into the real armor-class total), Expert Trainer, both feat counts, Order of the Sword's Sense Motive bonus gated on an explicit `choice:cavalier_order` pick, 8th class-skill widening instance. The decoy-variable hazard from the brief was real and is now guarded by its own dedicated test (`no_grounded_value_uses_the_never_consumed_decoy_formula`) — `CavalierOrderChallengeBonus` confirmed referenced nowhere but its own definition. Three of five magnitudes correctly named as DESC-sourced rather than token-verified. `named_features_wired` 1→6. Lead independently verified: 714/714 lib, 11/11 Cavalier tests, APG audit 3/3, clippy unchanged. Self-reported a third TDD-ordering slip (pattern: long verification phases pull implementation ahead of tests) — proven red/green retroactively each time, but named as a real pattern to watch. **Swashbuckler (#14) partial landed (`a2ac2b95`)** — Finesse's prereq substitution (generalized `brawler_cunning_effective_intelligence_score` to take a second operand, with a dedicated test pinning Brawler still floors at 13), Weapon Training, Bonus Feats' level-equivalence fact. Lead independently verified: 720/720 lib, 14/14 tests. Deeds held for two rulings, both now given: substitute `SwashbucklerLVL` for the broken gate variable (confirmed, `risks-and-open-questions.md` item 50), and **each deed counts as its own `named_features_wired` slot rather than folding into one** — lead checked the actual KEY prefixes and DESC text (each deed sits as a sibling of the parent `Deeds` record, not nested under a shared sub-mechanism, and the 6 real deeds are independent simultaneously-available abilities with genuinely different effects, not mutually-exclusive variants like Alchemist's Mutagen or Judgment's sub-types) — landing at 3→12, the biggest single-class jump this segment, because the underlying feature count genuinely is bigger, not because the rule bent. 8th class-skill widening instance confirmed (Cavalier was 7th, corrected from Bloodrager). Building deeds now, then Summoner (#17) |
| frontend | idle | Found the real Arcanist colon-convention bug via live-testing before shipping the `CLASS_OPTIONS` entry (correctly reverted rather than list a broken class) — now clear to re-attempt now that backend's fix has landed and been independently verified. Standing by |
| qa | idle | Bard's 16-file known-spell wave landed and lead-verified 100% (cb372cb3); entire workspace green; Bloodrager's closure doesn't appear to need a dedicated wave (no shared diagnostic retired, only its own new one added) — no new wave queued |
| scout | in_progress | Five scoping docs landed this cycle (Bloodrager, Cavalier, Summoner, Brawler — all greenlit, queued for backend), plus a meta-sweep: caught that 4 of 5 recently-checked task descriptions (#1/#10/#6/#5) inherited stale "too expensive" framing from before the standalone-grounding bar correction or a reuse pattern existed. Directed to audit **#11-15 (`tasks-11-15-stale-framing-sweep.md`)** — 3 of 5 confirmed stale: **Witch/Shaman spellcasting** are both near-total-corpus-reachable reuse jobs (Alchemist-module-shaped), not from-scratch ingestion (Shaman's 281-record count lead-verified exact; Witch's claimed 249 flagged for re-derivation, lead's own rough count came to 252); **Swashbuckler Finesse** genuinely reuses Brawler's already-built Cunning idiom (`max(CHASCORE,INTSCORE)` vs `max(13,INTSCORE)`, same `CombatFeatIntRequirement` variable, lead-verified exact) rather than needing new design; **Slayer** had a real sequencing error (Slayer Talents doesn't need to wait on the Studied Target design question — it's a narrowable chooser today). **Monk's Deflect Arrows confirmed correctly framed** (zero `BONUS` tokens, lead-verified) — the one row left unchanged. All five task descriptions corrected. **Swashbuckler (#14) scoped and greenlit** (`swashbuckler-finesse-and-deeds-scoping.md`) — self-caught its own sweep error (Deeds are automatic, not a chooser: no `BONUS:ABILITYPOOL` token anywhere, good news for scope) and found a genuine PCGen corpus authoring bug: `SwashbucklerDeedQualifyLVL` gates all six deed tiers but is only ever set from `MagusLVL` in the entire tree, never `SwashbucklerLVL` — a pure Swashbuckler literally qualifies for zero deeds taken literally. Lead independently re-verified (including checking Swashbuckler's own class chassis lines directly) and ruled the `SwashbucklerLVL` substitution a transcription fix, not fabrication (risks item 50). Idle, holding for next assignment |
| featmate | idle | Producer side of #16 now complete: Category A (33 CRB feats, `19ca4ae6`) plus all three Focus families — Skill (task #21, `144bf89f`), Spell (`5d90dc91`), **Weapon (`d322cb61`, 13/13 tests + 699/699 clean lib lead-verified)**. Weapon Focus's headline finding: it encodes stacking the OPPOSITE way from Spell Focus — `WeaponFocusToHit`/`GreaterWeaponFocusToHit` are genuinely separate variables (lead confirmed both `BONUS:VAR|...|1|TYPE=Base` tokens on the same `Default.MOD` record), so they truly add to +2, unlike Spell Focus's shared `TYPE=SpellFocus` take-highest. Correctly applied the ambiguity ruling in the opposite direction too: Greater Weapon Focus's own +1 is unambiguous so it grounds without base Weapon Focus, unlike Greater Spell Focus. Task #20's leaf half (`effective_feats`, `bb7d0c95`) also landed, broader in scope than asked (Ranger's Endurance, not just Monk's Stunning Fist). Everything outstanding in this lane is now consumer wiring — backend's queue. Standing down on new producer work, idle |

**Progress: Fighter, Wizard, Rogue, Ranger, Paladin, Barbarian, Sorcerer, Cleric, Druid, Bard, and now Arcanist genuinely reach Computed via the compute engine (11 of 27) — the original 10 are reachable through the real product UI (product-reachability gap fully closed for those 10).** **MILESTONE: Arcanist (ACG) is the first non-CRB class ever to reach genuine `HeadlessReceiptStatus::Computed` this segment** — after its full build (`ce73a598`) landed real prepared spellcasting, Arcane Reservoir, and class-skill wiring but stayed Blocked on `exploits_deferred` alone, backend found that one of Arcanist's 46 Exploits (Metamagic Knowledge) is genuinely a one-time bonus-feat grant, not an activation-gated Reservoir-consuming ability, and reused an already-built SD-20 module (`feat_prereqs::metamagic`) to validate the chosen feat for real — recognizing it narrows `exploits_deferred` enough to clear the last claim-blocking diagnostic (`54d8048b`). Confirmed via the actual milestone test asserting `HeadlessReceiptStatus::Computed` with zero claim-blocking diagnostics, not assumed. Backend then closed the BACKEND half of the product-reachability gap the same day (`772298bc`, Path A): `compose_character_input`/`apply_level_up` now seed both the starter spell and the Metamagic Knowledge choice (Arcanist needed both sites, like Wizard, unlike Sorcerer/Cleric/Druid's single-site fix) — a new milestone test proves a freshly composed Arcanist reaches `Computed` with zero caller-added anything, confirmed via the actual desktop suite (214/214). **But the lead independently found a second, still-open piece of the same gap**: unlike Sorcerer/Cleric/Druid (already listed in `characterHubModel.ts`'s `CLASS_OPTIONS` as `headless-only` before their own Path A), Arcanist has ZERO entry in `CLASS_OPTIONS` at all — the real Create Character form's class `<select>` is populated only from that array, no free-text option, so a real user still cannot select Arcanist in the shipped UI today. Arcanist's own `Computed` status is genuinely reachable through the backend command layer now, but not yet through the actual app — a distinct, still-open follow-on, named honestly rather than folded into the milestone claim. Monk has 6 of 7 restricted-list feats closed (partial, only Deflect Arrows remains). Skald, Bloodrager, Brawler, and Hunter (all ACG) plus Cavalier, Alchemist, and Inquisitor (all APG) each have a real named ability genuinely grounded but stay Blocked on other deferred features (partial). Warpriest (ACG, `03bb35d3`) is a genuine FULL build outside CRB — real prepared spellcasting with an independently-verified own table, plus Blessings/Sacred Weapon, all real; stays Blocked on its own other-features diagnostic alone. Warpriest's closure also caught and fixed a real class-skill-bonus widening bug (Climb/Intimidate/Swim bonus was silently not applying for a class whose real corpus list includes all three). Slayer (ACG, `77910256`) is a third full build and the first with zero spellcasting scope — a non-caster whose four remaining sub-features (Sneak Attack dice, Trap Sense, Trapfinding, Track) are all real, independently-verified flat formulas, honestly counted as 4 distinct wired features and grounded as standalone records with no total-integration, the same already-precedented idiom Barbarian's and Rogue's own Trap Sense already use; also caught a third instance of the class-skill-bonus widening bug. Stays Blocked on its own other-features diagnostic alone (Studied Target, opponent-dependent; Slayer Talents, a chooser-list). **Oracle (APG, `25c5bcae`) is the 4th full build and the first genuine full build in APG** — real spontaneous known-spell casting mirroring Sorcerer's own shape, plus Mystery (Life/Healing Hands) and Curse (Clouded Vision); unlike Arcanist, stays permanently Blocked (proven via a dedicated test). **Also in `25c5bcae`: a real bug found by frontend's live-testing was fixed** — Arcanist's Path A seed used a bare feat name with zero colons, failing the save layer's colon-segment requirement; fixed with a genuine translation layer (`arcanist_metamagic_knowledge_feat_name`), verified with a real end-to-end save-path regression test. Swashbuckler (ACG, `d267fe79`) is the 5th full build (Panache/Charmed Life/Nimble, granted starting 2nd level for Charmed Life). Investigator (ACG, `46eb6c3d`) is the 6th full build and the first to force a genuine architectural change — its 2-of-3 partial class-skill match (Climb+Intimidate, no Swim) required splitting the shared class-skill-bonus function into three independent per-skill functions, behavior-preserving for all 6 prior classes. Witch (APG, `d70f70d7`) is the 7th full build — Ward hex (flat `+2/+2` deflection/resistance, scaling to `+4`), plus a fifth class-skill-bonus instance and a third distinct partial-match shape (Intimidate only). A real pre-build correction also landed: Shaman's originally-scoped Healer's Touch MVP turned out to be level-8-gated, swapped to Life Spirit's Channel (immediately available, Cleric-Channel-Energy-shaped) before backend built it. Also fixed: a real shipped bug scout's formula audit found in Warpriest's Sacred Weapon dice count (`ab07e8e5`), a sibling-branch transcription error active at levels 15-19. **Shaman (ACG, `8574c80c`) is the 8th full build and completes a milestone: all ten real ACG classes now have at least one genuinely wired named feature** — Life Spirit's Channel (Cleric-Channel-Energy-shaped), swapped in after a real pre-build correction found the originally-scoped Healer's Touch was level-8-gated. 14 classes total across both books have real partial engine progress (not counting Arcanist, now fully Computed). Only Summoner remains outside real progress, held as genuinely subsystem-blocked and now logged as an explicit open question (risks-and-open-questions.md item 38) rather than an implicit backlog item. **Per operator directive (2026-07-26): pausing new class work here until this closure is fully reconciled and reviewed** — no new scoping or building starts without an explicit go-ahead. See the detailed table below for the full done/in-progress/queued breakdown by class.**

---

## Full class/race chassis breadth — detailed status (operator request, 2026-07-25)

Authorized 2026-07-24 (operator go-ahead alongside item 7's wealth work,
in response to a dashboard status question); running continuously since.
27 total classes across the corpus: 11 CRB + 6 APG + 10 ACG (Bestiary has
no PC classes). Race dimension: 7 playable races (Human, Dwarf, Elf,
Gnome, Half-Elf, Half-Orc, Halfling). Once a class reaches `Computed`, it
works identically across all 7 races at the levels offered — the only
race-specific wrinkle found anywhere in this epic is a historical,
pre-existing gate (`explain_hybrid_level1_chassis`) that blocks a
single-class Human specifically at level 1 for Ranger/Paladin only; every
other race/level/class combination is race-independent once its own
engine work lands.

### CRB (11 classes) — the only book with real per-class engine work started

| Class | Status | Race/level support | Landed in |
|---|---|---|---|
| Fighter | **Computed** | `full` — every race, levels 1-3 | pre-existing (SD-13/SD-18 era) |
| Wizard | **Computed** | `full` — every race, level 1 | pre-existing |
| Rogue | **Computed** | `full` — every race, level 1 | pre-existing |
| Ranger | **Computed** | `full-except-human-level-1` — every race/level 1-5 except single-class Human at level 1 | `b7642d97` (spell posture), UI fix `3fd04f25` |
| Paladin | **Computed** | `full-except-human-level-1` — same shape as Ranger | `ee3c50ce` (spell posture), UI fix `3fd04f25` |
| Barbarian | **Computed** | `full` -- unconditional, no race/level carve-out at all (never appears in the shared hybrid gate); real Rage execution engine (Strength/Constitution/Will/AC bonuses, over-budget blocking) | Rage engine + UI fix (`d020a5e8`), both lead-verified |
| Sorcerer | **Computed** (levels 1-2 only; level 3+ still blocked on bonus spells/feats, a separate future slice) — **UI: `full`, `levelOptions: [1,2]`, live-verified** | known-spell posture, Arcane bloodline recognition, and Arcane Bond identity recognition all real; the bloodline-power diagnostic's remaining pieces are provably vacuous (metamagic-DC and cast-a-spell preconditions can never arise in this codebase) | `d6067603` (engine), `9bafe303` (UI reachability), `adf57cfb` (UI label), lead-verified 378/378 lib + 78/78 desktop against real `HeadlessReceiptStatus::Computed` |
| Cleric | **Computed** (Good domain only; Good+Healing stays Blocked on Rebuke Death; no level cap) — **UI: `full`, `levelOptions: [1,2,3]`, live-verified** | prepared-spell posture + Good domain's Touch of Good (self-application only, ally-targeting unmodeled) both real; domain spell-list contents named non-blocking; Healing's Rebuke Death stays a genuine separate blocker | `fca4e64e`, `b98a20d7` (engine), `9bafe303` (UI reachability), `adf57cfb` (UI label), lead-verified 384/384 lib + 78/78 desktop against real `HeadlessReceiptStatus::Computed` |
| Druid | **Computed** (level 1 only, Wolf companion; advancement past level 1 named non-blocking) — **UI: `full`, `levelOptions: [1]`, live-verified** | prepared-spell posture + animal-companion Wolf stat block (standalone, non-integrating) both real; Link/Share Spells vacuous corrections; domain-type nature bond still falls through to the catch-all | `dda46d4a`, `9aeec493` (engine), `9bafe303` (UI reachability), `adf57cfb` (UI label), lead-verified 388/388 lib + 78/78 desktop against real `HeadlessReceiptStatus::Computed` |
| Bard | **Computed** — UI: `full`, live-verified reachable | Inspire Courage attack-bonus engine + known-spell posture both real (gate-ordering bug caught and fixed proactively along the way); other 6 performance types honestly named as unmodeled; spell tables verified through level 10 only | `0374b96a` (Inspire Courage), `86d26f88` (known-spell closure), test wave `cb372cb3`, `833ea89c` (UI label), lead-verified 393/393 lib + 78/78 desktop against real `HeadlessReceiptStatus::Computed` |
| Monk | **Blocked** — 6 of 7 feats closed | `table_class_id` still not widened (base-attack/save/fast-movement stay standalone). Dodge/Catch Off-Guard/Throw Anything (provably vacuous preconditions) plus Combat Reflexes (extra-AoO capacity, `max(Dex mod,0)`), Scorpion Style (DC + duration, both real Monk-derived numbers), and Improved Grapple (+2/+2 CMB/CMD magnitude, reusing the Dwarf Stability idiom — no new pillar needed) are all closed for real, each explicitly not claiming the trigger event itself is resolved. Only Deflect Arrows remains — the one feat with zero standalone numeric value, genuinely needs a full opponent-interaction/incoming-attack engine this codebase has no framework for at all | `18920c3d` + `b1a453a1` (first 3), `e45b622f` (remaining 3, adversarial-reviewed: dropped an originally-proposed new CMB/CMD pillar per the review's Dwarf Stability finding), lead re-verified against committed HEAD: 414/414 lib, full workspace sweep zero failures |

**CRB tally: 10 of 11 genuinely Computed, 1 of 11 (Monk) with real partial engine progress (6 of 7 restricted-list feats closed — only Deflect Arrows remains, needing a genuinely new opponent-interaction engine). All 11 CRB classes now have at least some real engine work landed.**

### APG (6 classes) — 5 of 6 have real partial engine progress, 1 still dispatch-only

Chassis (BAB/save/HP) landed together in one commit (`c511c132`): real
BAB/save progression and real hit-die-derived HP via each class's own
`HD:` token, deliberately kept OUTSIDE `table_class_id`/
`multiclass_class_level_supported` to avoid a false-Computed multiclass
loophole. Since then, Cavalier, Alchemist, Inquisitor, and Oracle have all
gone beyond chassis-only: Cavalier's Mount (reusing the Hunter/Druid
animal-companion pattern) was the first-ever APG class-specific closure;
Alchemist's Mutagen combined choice-recognition with activation-gating
for the first time; Inquisitor's Judgment (narrowed to Justice as the
canonical MVP judgment type) is the third, and caught a real pre-commit
bug (a missing over-budget check in a second, separate helper function)
before it shipped; **Oracle is the first genuine FULL build in APG** —
real spontaneous known-spell casting (mirroring Sorcerer's own shape,
since Oracle is spontaneous unlike Cleric/Wizard/Arcanist/Warpriest) plus
Mystery (Life/Healing Hands) and Curse (Clouded Vision) choices, all real.

| Class | BAB | Hit die | Status |
|---|---|---|---|
| Alchemist | 3/4 | d8 | **Blocked** — real progress: full build, real prepared-caster spellcasting now genuinely grounded. Mutagen (choice-driven, verified two separate ways), Bomb (damage `1+(level-1)/2`d6+INT, DC `10+level/2+INT`, uses/day `level+INT`, all standalone weapon-damage facts, folded as one `named_features_wired` slot per the Cleric-Channel-Energy precedent), Poison Resistance (identical tiers to Investigator's own, own parallel-copy formula), and prepared-extract spellcasting (reuses the shared `alchemist_spell_list` module + Investigator's own validation shape) all genuinely grounded. Two flagged formula hazards on Bomb's own corpus record (a conditional dice-count override, a bonus-count variant) confirmed provably vacuous — gated behind Ultimate-Magic-archetype and Gnome-race content this engine never ingests/models, not hidden bugs. Discovery stays a deferred chooser-list; Swift Alchemy/Swift Poisoning correctly deferred (no numeric magnitude). Stays Blocked on `other_features_deferred`. `named_features_wired = 3`. Committed `101bf40d` (Mutagen) + `9a0022e0` (Bomb/Poison Resistance/spellcasting), lead re-verified against the real committed HEAD: 582/582 lib, `sd24_apg_class_coverage_audit` 3/3, all 19 Alchemist-specific tests |
| Cavalier | **full** | d10 | **Blocked** — real progress: Mount (Horse companion stat block, effective druid level = cavalier level) genuinely grounded; Share Spells correctly not fabricated. **Deepened 2026-07-27 (task #6)**: Challenge (uses/day + self `-2` AC penalty integrated into the real armor-class total, DESC-sourced not token-backed), Expert Trainer, bonus-combat-feat and teamwork-feat counts, and Order of the Sword's Sense Motive bonus (gated on an explicit `choice:cavalier_order` pick) all genuinely grounded; a real decoy-variable hazard (`CavalierOrderChallengeBonus`, referenced nowhere, sitting on the natural entry point next to the genuinely-used `OrderChallengeBonus`) was caught before code was written and is now guarded by its own dedicated test; 8th class-skill-bonus widening instance fixed. `named_features_wired` 1→6. Still Blocked on 10+ remaining deferred features (no spellcasting — pure martial class; the charge family, Banner, Demanding Challenge, the five non-Sword Orders, every order's challenge rider, By My Honor). Committed `d256bc9c` (Mount) + `b5284379` (deepening), lead re-verified against the real committed HEAD: 714/714 lib, `sd24_apg_class_coverage_audit` 3/3, 11 Cavalier-specific tests, clippy unchanged |
| Inquisitor | 3/4 | d8 | **Blocked** — real progress: Justice/Protection/Purity/Smiting judgments (all four reuse the same activation-gated, uses-per-day-budget-enforced mechanism; Protection `1+level/5` to Armor Class, Purity `1+level/5` to all saves, Smiting a standalone DR-bypass fact) plus Stern Gaze's Intimidate half (`max(1,level/2)`, unconditional) plus Monster Lore (+WIS on Knowledge), Cunning Initiative (+WIS on Initiative), and Track (`max(level/2,1)` on Survival, byte-identical to Slayer's own) all genuinely grounded as standalone facts. The last three were corrected in a follow-on (task #18) after a real methodology fix: this codebase's own established precedent (Bard's Bardic Knowledge, Slayer's Track, Barbarian's Damage Reduction) already tolerates a genuinely-verified standalone flat fact with zero live consumer — the original "needs a live consumer" bar this closure first applied was stricter than the codebase's own dominant practice. Domain grants no power per RAW (provably out of scope). Studied-target-style opponent-dependent judgments (Destruction/Healing/Piercing/Resiliency/Resistance) still need real engine state that doesn't exist and stay deferred. Spellcasting (WIS spontaneous, no reusable spell list) stays deferred. `named_features_wired` 1→2→5. Committed `22abe6e5` (Justice) + `14126998` (Protection/Purity/Smiting/Stern Gaze) + `6c8ca561` (Monster Lore/Cunning Initiative/Track), lead re-verified against the real committed HEAD: 540/540 lib, `sd24_apg_class_coverage_audit` 3/3, 13/13 Inquisitor tests |
| Oracle | 3/4 | d8 | **Blocked** — real progress: full build, not a single ability, first genuine FULL build in APG. Real spontaneous known-spell posture (own independently-verified per-level `CAST`/`KNOWN` table) + Mystery (Life Mystery's Healing Hands, flat `+4 Heal` bonus) + Curse, now genuinely deepened (task #10, `db64e788`+`c1ce97fe`): Lame (land-speed penalty computed off the character's real race speed, not a hardcoded 30 — 3 of 7 races are 20-ft), Wasting (CHA/Intimidate trade), Deaf (initiative penalty + opposed-Perception `-4`, all three magnitudes wired) join Clouded Vision as four independently-grounded top-level curse records; plus 6 Tier-1 Life Mystery revelations (Channel, Combat Healer, Delay Affliction, Energy Body, Enhanced Cures, Sidestep Secret, Nature's Whispers) gated behind a real `choice:oracle_revelation` mechanism proven load-bearing by a dedicated test (a Mystery pick alone grounds nothing). Sidestep Secret integrates into the Reflex total and Nature's Whispers into AC, both proven by differencing the same character with/without the revelation. Bone/Near Death correctly left un-integrated — its bonus applies only against disease/mind-affecting/poison, and this engine has no per-category save facet, so folding it into the flat save totals would silently promote a narrow bonus into an unconditional one. Cinder Dance + Lame confirmed as a corpus-illegal combo (`!PREABILITY`-gated), diagnosed rather than let the +10/-10 net to zero. Class-skill list confirmed clean (no Climb/Intimidate/Swim — no bug here, same shape as Wizard/Arcanist). **Unlike Arcanist, stays permanently Blocked** on `other_features_deferred` (Cure Wounds/Inflict Wounds spontaneous conversion, Tongues, the other 9 Mysteries' revelations) — proven via a dedicated test asserting the Blocked status even with everything else recognized, not assumed. `named_features_wired` 2→5 (curses count individually — each a top-level `Oracle ~ ...` corpus record, same shape as Inquisitor's Stern Gaze/Track; revelations fold into the single Mystery slot — each namespaced `<Mystery Name> ~ ...`, same shape as Judgment's sub-types — so the count stays 5 even once the remaining 9 mysteries land). Committed `25c5bcae` (original) + `db64e788` (curses) + `c1ce97fe` (revelations), lead re-verified in an isolated worktree at the committed SHA: 643/643 lib, `sd24_apg_class_coverage_audit` 3/3, 36 Oracle tests (22 new), full workspace sweep 478 blocks clean, clippy unchanged (23 pre-existing warnings) |
| Summoner | 3/4 | d8 | dispatch-only, Blocked — queued, untouched |
| Witch | 1/2 | d6 | **Blocked** — real progress: full build, not a single ability. Ward hex (flat `+2/+2` deflection/resistance, scaling to `+4` at levels 8/16) genuinely grounded, choice-recognized the same way as Oracle's Mystery/Warpriest's Blessing; confirmed genuinely available at level 1 (`PREVARGTEQ:WitchHexAbilityLVL,1`, no hidden gate). Also caught and fixed a real class-skill-bonus widening bug: fifth instance, and a third genuinely distinct partial-match shape — Witch's own list includes Intimidate but neither Climb nor Swim (unlike Investigator's Climb+Intimidate). `named_features_wired = 1` (Hex slot alone). Spellcasting and the Familiar both stay deferred (no reusable spell list exists for Witch). Stays permanently Blocked on `other_features_deferred`. Committed `d70f70d7`, lead re-verified against the real committed HEAD: 496/496 lib, `sd24_apg_class_coverage_audit` 3/3, all 7 Witch-specific tests individually, the 10-test skill-bonus regression suite, desktop 215/215 (unchanged) |

**APG tally: 0 of 6 reach Computed (a real named-feature or spellcasting gap keeps every APG class Blocked regardless of chassis work), 5 of 6 (Cavalier, Alchemist, Inquisitor, Oracle, Witch) have a real named ability or full build genuinely grounded, 1 of 6 (Summoner) untouched beyond chassis dispatch.**

### ACG (10 classes) — 1 of 10 genuinely Computed, 9 of 10 have real partial engine progress, 0 dispatch-only — MILESTONE: every real ACG class now has at least one genuinely wired feature

Chassis (BAB/save/HP) landed together in one commit (`71cd41b6`), same shape
and same multiclass-loophole avoidance as APG. QA's per-class survey found
**4 real full-BAB classes**, not the 1 originally assumed from APG's
Cavalier precedent — flagged below. Since then, 7 of the 10 have gone
beyond chassis-only: Skald and Bloodrager (Rage-shaped, activation-gated);
Brawler (always-on, pure function of level); Hunter (always-on, reuses
Druid's own reviewed Wolf companion stat block); **Arcanist, Warpriest, and
Slayer** — the first three of any of these to attempt genuine FULL builds
(chassis + skills + real spellcasting-or-equivalent + class-specific
features), not a single ability. **MILESTONE: Arcanist (`54d8048b`) went
further and reached genuine `Computed`** — the first non-CRB class this
segment to do so — via a real follow-on closure (Metamagic Knowledge)
narrowing its last claim-blocking diagnostic. Warpriest and Slayer each
stay Blocked on their own remaining-feature diagnostic; Slayer is the
first of the three with zero spellcasting scope at all.

| Class | BAB | Hit die | Status |
|---|---|---|---|
| Arcanist | 3/4 | d6 | **Computed** — backend command path only, **NOT yet in the real UI dropdown** (MILESTONE — first non-CRB class this segment to reach genuine `Computed`) — full build: real prepared spellbook validation (own independently-verified per-day table, genuinely NOT identical to Wizard's despite sharing its spell list — 4/2 vs Wizard's 3/1 at level 1, 2nd-level access at Arcanist level 4 vs Wizard's level 3) + Arcane Reservoir (flat pool, no choice, `3+level` max/`3+level/2` daily fill) + class-skill wiring (needed zero new code) all genuinely grounded, confirmed Blocked on `exploits_deferred` alone thereafter (a Wizard-style spell-seed follow-on doesn't clear it, since the diagnostic was unconditional — backend caught and reverted that attempt). **Follow-on closure found the real fix**: of Arcanist's 46 Exploits, Metamagic Knowledge is genuinely a one-time bonus-feat grant (no Reservoir cost, unlike the other 45), validated via an already-built, unrelated SD-20 module (`feat_prereqs::metamagic`) — recognizing it narrows `exploits_deferred` enough to clear the last claim-blocking diagnostic. Confirmed via the actual milestone test: `HeadlessReceiptStatus::Computed` with zero claim-blocking diagnostics. **Then closed product-reachability the same day (Path A)**: `compose_character_input`/`apply_level_up` now seed both the starter spell and the Metamagic Knowledge choice (Arcanist needed both sites, like Wizard, unlike Sorcerer/Cleric/Druid's single-site fix) — new milestone test `arcanist_level1_reaches_computed_from_compose_character_input_alone` proves a freshly composed Arcanist reaches `Computed` with zero caller-added anything. Committed `ce73a598` (full build) + `54d8048b` (Metamagic Knowledge) + `772298bc` (Path A), lead re-verified against the real HEAD at every stage: 463/463 lib, `sd24_acg_class_coverage_audit` 3/3 ("seven" closures, unchanged), all 14 Arcanist-specific engine tests, full desktop suite 214/214 |
| Bloodrager | **full** | d10 | **Blocked** — real progress: Bloodrage (Str/Con/Will/AC, self-only by RAW, mirrors Barbarian's Rage exactly) genuinely grounded. **Closed 2026-07-27 (task #1)**: fixed a real shipped honesty bug — the spellcasting diagnostic claim-blocked unconditionally at every level despite Bloodrager having no `CAST:`/`KNOWN:` rows below level 4 per RAW (`PRECLASS:1,Bloodrager=4`), now level-aware and non-blocking below 4; corrected a false "no class-skill list" diagnostic claim, grounding the real 11-skill list (7th widening instance); built the full levels 4-20 spells-per-day/known table straight off the corpus tokens; built the complete 183-entry spell list (110 corpus-reachable, 73 routed through the existing unresolved-selection idiom, stated plainly as "110 of 183"). `named_features_wired` 1→2 (Bloodrage + Spells; class skills excluded, `CATEGORY:Internal`, same exclusion as Warpriest's identical record). Still Blocked on `other_features_deferred` (Fast Movement, Uncanny Dodge, Blood Sanctuary, Damage Reduction, the Greater/Tireless/Mighty tiers, the whole Bloodline slot). Committed `15560e62` (Bloodrage) + `73c70055` (diagnostic+tables) + `f24c6f24` (spell list), lead re-verified against the real committed HEAD: 686/686 lib, `sd24_acg_class_coverage_audit` 3/3, 17 Bloodrager-specific tests, clippy unchanged |
| Brawler | **full** | d10 | **Blocked** — real progress: AC Bonus (level-scaled dodge bonus, `+0` until 4th level) genuinely grounded (`19c792e1`). **Deepened 2026-07-26** (per operator direction to complete in-progress work rather than start new classes): Brawler's Cunning (flat, unconditional `max(13,INT)` combat-feat-prereq floor) + Brawler's Strike (level-gated DR-bypass progression `(level>=5)+(level>=9)+(level>=12)+(level>=17)`, honestly inert below level 5) both genuinely grounded, plus a 6th class-skill-bonus instance fixed (Brawler's own list genuinely includes all three tracked skills). `named_features_wired` 1→3. Still Blocked on 9 remaining deferred named features (Flurry, Knockout, Martial Flexibility, Awesome Blow family, Martial Training, Bonus Feats, Close Weapon Mastery, Maneuver Training, Strike's own Alignment Selection chooser — all genuinely out of scope for a bounded slice). Committed `19c792e1` + `4555a862`, lead re-verified the deepening in true isolation from concurrent WIP: 506/506 lib, `sd24_acg_class_coverage_audit` 3/3, desktop 215/215, all 7 Brawler-specific tests, the 11-test skill-bonus regression suite |
| Hunter | 3/4 | d8 | **Blocked** — real progress: Animal Companion (Wolf, effective druid level = hunter level, reuses Druid's own reviewed stat-block math via 2 newly-shared functions) genuinely grounded and unconditional, plus Wild Empathy (`CHA+level`, flat unconditional check-modifier fact) and Animal Focus (Bull picked as the canonical choice, `+2/+4/+6` STR enhancement at levels 1/8/15, activation-gated with a genuinely enforced per-day minutes budget). Animal Focus was a real correction under the standalone-grounding methodology fix — a prior pass wrongly deferred it as "a chooser," when each of the 13 real options is actually a flat, self-scoped magnitude that narrows cleanly (the same Oracle-Mystery-style pick-one-canonical shape). Nature Training confirmed correctly deferred (zero numeric `BONUS` token at all, a feat-qualification flag only, not a missed win). Spellcasting deliberately deferred to its own follow-on (reuses already-built Druid+Ranger spell lists, a real reuse-backed lift, not from-scratch). `named_features_wired` 1→3. Committed `21c5b13a` (Animal Companion) + `f1c46e19` (Wild Empathy + Animal Focus), lead re-verified against the real committed HEAD: 556/556 lib, `sd24_acg_class_coverage_audit` 3/3, all 9 Hunter-specific tests individually |
| Investigator | 3/4 | d8 | **Blocked** — real progress: full build, real prepared-caster spellcasting now genuinely grounded (not deferred). Trapfinding/Trap Sense/Inspiration pool-size, Poison Resistance (`InvestigatorPoisonLVL`'s real `2/5/8/10`-level tiers: None/`+2`/`+4`/`+6`/immunity, a corrected secondary-source error), and Alchemy (flat `+level` Craft-alchemy bonus) all standalone. **Spellcasting subsystem now built**: the shared `alchemist_spell_list` module (104 real records — 13 new Alchemist spells + 91 `.MOD` grafts onto CRB spells, extracted directly from `apg_spells.lst`, 0 duplicates) plus Investigator's own prepared-extract validation (mirrors Arcanist's shape exactly — no arcane-school opposed-cost mechanic) plus the full 1-20 Extracts Prepared table (3-independent-source-verified, a real deliberate deviation from Wizard's/Arcanist's own level-3-bounded convention since Investigator's chassis already supports all 20 levels and the data is fully verified — lead-approved after confirming the level-3 bound elsewhere is a convention, not a ceiling: Barbarian/Monk are already widened to 20/12 respectively) plus extract save DC (`10 + extract level + INT`). Stays Blocked on `other_features_deferred` (Studied Combat/Strike, opponent-dependent, ruled deferred consistently with Slayer's own Studied Target; Investigator Talents, a chooser-list). `named_features_wired = 6` (adds the spellcasting slot). The shared spell-list module is now available for Alchemist's own task #4. Committed `46eb6c3d` (chassis) + `fb3c08b0` (Poison Resistance + Alchemy) + `80efdb44` (spellcasting subsystem), lead re-verified against the real committed HEAD: 569/569 lib, `sd24_acg_class_coverage_audit` 3/3, all 13 Investigator-specific tests |
| Shaman | 3/4 | d8 | **Blocked** — real progress: full build, not a single ability. Life Spirit's Channel (a real Channel Positive Energy variant, structurally identical to Cleric's own Channel Energy) genuinely grounded: uses/day `1+CHA`, dice `(level+1)/2` d6, save DC `10+(level/2)+CHA` — the DC Cleric's own grounding omits. Choice-recognized via `choice:shaman_spirit -> spirit:life`, mirroring Oracle's/Witch's three-branch shape. **Real pre-build correction**: the originally-scoped Healer's Touch MVP turned out to be level-8-gated (`PREVARGTEQ:ShamanSpiritGreater,1`), not immediately available like Oracle's own Healing Hands — caught before building, independently confirmed by the lead directly against the corpus, swapped to Channel. Class-skill list confirmed to exclude all three of Climb/Intimidate/Swim — no bug here. `named_features_wired = 1` (Spirit slot). Stays permanently Blocked on `other_features_deferred` (fresh spellcasting, Spirit Animal's unbuilt Familiar, the other 9 spirits). Committed `8574c80c`, lead re-verified against the real committed HEAD: 502/502 lib, `sd24_acg_class_coverage_audit` 3/3 (canary renamed to reflect the milestone), all 6 Shaman-specific tests, desktop 215/215 (unchanged) |
| Skald | 3/4 | d8 | **Blocked** — real progress: Inspired Rage (self-application inferred, mirrors Barbarian's Rage), real spontaneous spellcasting (known-spell posture + access ladder/per-day/save-DC, reusing Bard's spell list and pure lookup functions directly — byte-identical tables, 2-source verified), self-Damage Reduction (`0/1/2/3` at levels 9/14/19, mirroring Barbarian's own standalone DR-record shape exactly, ally-extension via Raging Song deferred since this engine models no allies), and Bardic Knowledge (`max(level/2,1)`, byte-identical to Bard's own formula) all genuinely grounded. Still Blocked on the remaining unbuilt named features (Rage Powers, Raging Song variants, Spell Kenning, Well-Versed, Lore Master, Versatile Performance, and the ally/opponent-targeting song family — Skald has none of Bard's ~20-feature prior investment). DR and Bardic Knowledge were corrected in via task #7 after the same standalone-grounding methodology fix Inquisitor's task #18 established — both were initially mis-scoped as blocked under an over-strict "needs a live consumer" bar. `named_features_wired` 1→2→3. Committed `d2eb0798` (Inspired Rage) + `7e7f6fbd` (spellcasting) + `6c8ca561` (Damage Reduction + Bardic Knowledge), lead re-verified against the real committed HEAD: 540/540 lib, `sd24_acg_class_coverage_audit` 3/3, 18/18 Skald tests |
| Slayer | **full** | d10 | **Blocked** — real progress: full build, not a single ability, first ACG/APG closure with zero spellcasting scope (confirmed non-caster — no `SPELLSTAT` token). Sneak Attack dice (`level/3`), Trap Sense (`max(1,level/3)`), Trapfinding (`level/2` on Perception/Disable Device), and Track (`max(level/2,1)` on Survival) all genuinely grounded as four independently-verified flat formulas, each its own standalone explanation record with no total-integration — mirroring Barbarian's and Rogue's own Trap Sense precedent exactly. Honestly counted as `named_features_wired = 4` (not folded down like Arcanist/Warpriest's 2, since none of the four share an underlying mechanism). Also caught and fixed a third instance of the class-skill-bonus widening bug (Climb/Intimidate/Swim). Stays Blocked on `other_features_deferred` alone (Studied Target, opponent-dependent; Slayer Talents, a chooser-list) — confirmed via the actual `HeadlessReceiptStatus::Blocked` test. Committed `77910256`, lead re-verified against the real committed HEAD: 457/457 lib, `sd24_acg_class_coverage_audit` 3/3 ("seven" closures), all 6 Slayer-specific tests individually confirmed |
| Swashbuckler | **full** | d10 | **Blocked** — real progress: full build, not a single ability, non-caster (zero `SPELLSTAT`), same cheap structural shape as Slayer. Panache (`max(1,CHA)` pool, DESC-text/archetype-cross-validated — the base class's own corpus record carries no literal `BONUS:VAR` token, unlike every other formula this segment has grounded; named honestly as a different, weaker evidentiary path rather than silently treated as equivalent) + Charmed Life (activation-gated save bonus, `((level-2)/4)+3` uses/day, granted starting 2nd level — confirmed via web search then independently re-confirmed by the lead directly in the corpus's own per-level grant table, resolving the level-1 negative-operand question by making it moot: the feature doesn't exist yet at level 1) + Nimble (flat AC dodge bonus, standalone since this engine computes no player AC total anywhere) all genuinely grounded. Also caught and fixed a real class-skill-bonus widening bug (fourth instance: Climb/Intimidate/Swim). **Deepened and closed 2026-07-27 (task #14)**: Finesse's prereq-substitution half grounded (generalized Brawler's own Cunning idiom to a second operand, `max(CHASCORE,INTSCORE)`, with a dedicated test pinning Brawler still floors at 13); Weapon Training and Bonus Feats' level-equivalence fact grounded; all six real-magnitude Deeds grounded (Derring-Do, Dodging Panache, Precise Strike, Bleeding Wound, Deadly Stab, Stunning Stab) after finding and fixing a genuine PCGen corpus authoring bug — `SwashbucklerDeedQualifyLVL` (the gate variable for all six deed tiers) is never set from `SwashbucklerLVL` anywhere in the ingested tree, only from `MagusLVL`, so implemented literally a pure Swashbuckler qualified for zero deeds; substituting `SwashbucklerLVL` directly was ruled a transcription fix, not fabrication (`risks-and-open-questions.md` item 50). `named_features_wired` 3→12 (Panache/Charmed Life/Nimble/Finesse/Weapon Training/Bonus Feats + the 6 individually-counted deeds — ruled as independent, simultaneously-available abilities with genuinely distinct effects, not mutually-exclusive variants like Alchemist's Mutagen). Still Blocked on `other_features_deferred` (Deeds' 15 remaining tokenless entries, genuine no-ops; Finesse's Weapon-Finesse-with-light/piercing-weapons benefit, which has no corpus token and needs real attack mechanics this engine doesn't have). Committed `d267fe79` (original) + `a2ac2b95` (Finesse/Training/Feats) + `fc29c297` (deeds) + `78ba4550` (count correction), lead re-verified against the real committed HEAD: 725/725 lib, `sd24_acg_class_coverage_audit` 3/3, 19 Swashbuckler-specific tests, clippy unchanged |
| Warpriest | 3/4 | d8 | **Blocked** — real progress: full build. Real prepared spellbook + Blessings + Sacred Weapon's base-damage-die formula + Destructive Attacks all genuinely grounded from the original closure. **Deepened (task #9, `c6dd2c6b`)**: Fervor (uses `level/2+WIS`, heal dice `1+max(0,min(20,level)-2)/3`), Channel Energy DC (`10+level/2+WIS` — the BONUS:VAR token, not the CHA-based DESC-arg copy-paste artifact on the separate Channel Positive/Negative Energy display records), Sacred Armor (enhancement `1+max(0,(min(20,level)-7)/3)`, uses `level`), and Strength Surge (`max(1,level/2)`, the self-targeted Blessing minor power, narrowed the Destructive-Attacks way) all landed as standalone facts. Also caught and fixed a real class-skill-bonus widening bug on the original closure. Correctly still Blocked on `other_features_deferred`: Sacred Weapon's active enhancement and ~15 touch-weapon/summon-monster Blessing powers, a genuine weapon-enhancement/summon-subsystem architecture gap, deliberately deferred as its own future design question (same category as Familiar/Eidolon). `named_features_wired` 2→5. Committed `03bb35d3` (original) + `c6dd2c6b` (deepening), lead re-verified against the real committed HEAD: 621/621 lib, both SD-24 coverage audits 3/3, all 18 Warpriest-specific tests individually |

**ACG tally: 1 of 10 (Arcanist) genuinely reaches `Computed` — MILESTONE, first non-CRB class this segment, via the Metamagic Knowledge follow-on closure (`54d8048b`) narrowing its last claim-blocking diagnostic; engine-Computed and product-reachable (Path A closed, `772298bc`+`25c5bcae`). 9 of 10 (Skald, Bloodrager, Brawler, Hunter, Warpriest, Slayer, Swashbuckler, Investigator, Shaman) have real engine progress genuinely grounded but stay Blocked on their own permanent remaining-feature gaps — 0 of 10 untouched beyond chassis dispatch. MILESTONE: every one of the 10 real ACG classes now has at least one genuinely wired named feature.**

### Product-reachability gap — FULLY CLOSED (backend `9bafe303` + frontend `adf57cfb`, 2026-07-25)

Sorcerer/Cleric/Druid's `Computed` posture each requires a specific choice
(Arcane bloodline+bond, Good domain, Wolf nature-bond respectively) that
**the real character-creation UI had no way to submit** — confirmed three
independent ways when first found (no picker in `CreateCharacterForm.tsx`;
no auto-seeded default in `compose_character_input`; no choices field on
the wire contract). Rather than building the full interactive picker this
implied (Path B — a new request field, new frontend UI, per-class option
content authored honestly for 3 different shapes), frontend's own scoping
found a cheaper option already shipped once: **Path A**, mirroring the
existing Wizard silent-canonical-default precedent (a fixed choice seeded
for every character of a class, no player-facing picker, the same
"documented, honest limitation" shape Wizard's own school-specialization
choice already uses). Backend implemented Path A in `compose_character_input`
(`9bafe303`) — Sorcerer gets Arcane bloodline + a familiar Arcane Bond,
Cleric gets the Good domain, Druid gets the animal-companion nature bond
(Wolf automatic, no species picker needed) — all three now reach `Computed`
with zero claim-blocking diagnostics through the real creation UI. **Lead
independently verified**: read all 3 conditional blocks directly, ran the
full desktop Rust suite (212/212) and frontend suite (78/78) against the
real committed HEAD, and traced `create_character_at_root`'s own Blocked/
Saved gating logic to confirm it's the exact same code path the updated
tests exercise — a genuine proof of the real command path, not a divorced
internal check. Path B (a real interactive picker, letting a player
actually choose their own bloodline/domain/companion) remains deliberately
deferred as separate future product-decision work, framed the same
"out-of-scope for now" way Wizard's own precedent already is. Bard was
always unaffected (empty known-spell selection is honestly valid). Skald/
Bloodrager/Brawler/Hunter/Cavalier are also unaffected (activation-gated
or always-on, never choice-gated). Full writeup in
`risks-and-open-questions.md` item 8's chain. **Frontend's own follow-on
landed and is verified** (`adf57cfb`): re-checked per class rather than
assuming a uniform bucket — none of the three need `full-except-human-
level-1` (confirmed directly none appear in `hybrid_level1_class`'s match
arms, each choice check is race-independent). Real per-class level ranges
found and live-verified through the actual dev build, including boundary
crossings: Sorcerer `full`, `levelOptions: [1,2]` (level 3 correctly stays
blocked with the real diagnostic shown, not silently advanced); Cleric
`full`, `levelOptions: [1,2,3]` (no cap found); Druid `full`,
`levelOptions: [1]` (level 2 correctly stays blocked). **Lead independently
re-verified against the real committed HEAD**: read the committed
`CLASS_OPTIONS` entries directly, ran the full frontend suite (78/78) and
typecheck — both clean. **This gap is now fully closed** — all 10 classes
that reach `Computed` via the compute engine are genuinely reachable
through the real product, each with an honest, live-verified support
level.

### What's actually queued next (in order, as currently planned)

1. **Arcanist's real remaining blocker: Exploits (or an equivalent deferral change)** — backend attempted the Wizard-style `compose_character_input` spell-seed follow-on and found it has zero product-visible effect: `exploits_deferred` is unconditionally `claim_blocking` (unlike Wizard's own genuinely-conditional remaining diagnostic), so `create_character_at_root` stays `Blocked` regardless of whether a spell is seeded. Correctly reverted rather than shipped speculatively. Reaching product-visible `Computed` needs Exploits itself addressed (built, or given its own honest deferral treatment that doesn't unconditionally block), not just a spell seed. Not yet scoped.
2. **Deflect Arrows** — Monk's one remaining restricted-list feat, correctly re-confirmed as needing a genuinely new opponent-interaction/incoming-attack engine this codebase has no framework for at all. Not scheduled.
3. Monk's/Brawler's/Hunter's/Cavalier's/Alchemist's/Inquisitor's/Arcanist's own test-cleanup waves, once QA scopes whether any are needed (same "no globally-retired diagnostic" reasoning that made Bloodrager's wave unnecessary may apply — their diagnostics are new, not replacements).
4. Summoner (APG) is the only remaining fully-untouched class on the whole 27-class roster — held as genuinely subsystem-blocked (needs the unbuilt Eidolon), logged as an explicit open question in risks-and-open-questions.md item 38, not decided reflexively.
5. Backend's own next target, not yet chosen as of the last check-in.

### Honest scale note

This item was flagged from the start as **not a bounded task** — each class needs its own real BAB/save progression, HP, class-skill list, spellcasting (for casters), and class-feature implementation, roughly the same order of magnitude as the original Fighter/Wizard/Rogue chassis work multiplied by up to 26 more classes. 10 CRB classes are genuinely done (7 of those reachable through the real product UI today), Monk has 3 of 7 feats closed, 3 ACG classes have one real ability each grounded, and the largest bucket (skill lists/features/spellcasting for all 27, plus all remaining class-feature work for 13 of 16 APG/ACG classes) hasn't been started yet. This is expected to keep running for a while longer — the operator's standing "no job too big" directive is what's keeping it moving rather than treating the size as a reason to stop.

---

## CHECKPOINT (lead, 2026-07-24, ~15:25 ET) — bounded swarm-actionable backlog is exhausted

Both backend's and frontend's closure-readiness passes came back clean (frontend
found and fixed one real pre-existing gap — a dead Print button in `WeaponsTab`,
`c014ffec`; backend found genuinely nothing across a full workspace test+build
and a formal 4-check no-stub-mvp doctrine audit against the *entire*
`develop...tranche/6` diff, not just today's commits). Lead independently
re-verified both: the doctrine grep checks (0 hits), the full root-crate
workspace test suite, and the desktop suite/typecheck.

Cross-checked against `SWARM_TASKS.md`: every remaining open row is now one of:
- **Blocked on an equipment-attachment schema decision** — item 1's multi-weapon
  attack-bonus math (the single-weapon case already shipped).
- **Blocked on an explicit operator decision** — item 27, whether `Computed`
  should ever accept a non-hardcoded equipment posture (this is the decision
  that would unlock real value from bridging the headless/corpus-aware wall).
- **Deferred, not assigned this wave** — item 18, Wizard non-Human spell-math
  completeness (a documented gap, not a wrong value; backend's scoping
  read-through suggests the eventual fix may be smaller than first estimated,
  see item 18's note — informational only, not a reopened decision).
- **Multi-cycle future-epic scope** — class/multiclass breadth for 8 of 11 CRB
  classes, and starting wealth for the 12 non-CRB-recognized classes in the
  operator's table (no class id exists for them anywhere in the crate yet).

This is a real checkpoint, not a stall or a dead swarm: nothing is broken, all
three teammates are idle by choice with clean queues, not blocked on each
other or on infrastructure. It reflects the edge of what this wave's scope can
close without operator input on items 1/18/27, or a decision to open a
substantially larger new wave (class-breadth work is its own multi-cycle
epic, not a bounded task). Surfaced here plainly per standing guidance rather
than manufacturing further busywork to avoid reporting it. The swarm remains
live and will pick up immediately on any operator direction or newly
discovered bounded work; the standing ~20-minute autonomous check-in
continues regardless.

**CORRECTION (operator, 2026-07-24, ~21:20 ET) — "future-epic scope" and "backlog" language above was read, correctly, as shoving real work off to a later cycle. It was not intended as deferral, but it reads as one, and that's what matters.** Operator directive, verbatim in substance: build every class from all four primary books (CRB, Bestiary 1, APG, ACG) fully wired, no shortcuts. This is not the 8-CRB-class framing used above — of the 27 classes across CRB+APG+ACG (Bestiary has no PC classes at all), only 3 (Fighter/Wizard/Rogue) have a working chassis today; the other 24, CRB and non-CRB alike, all need real BAB/save/HP/skill/spellcasting/feature computation, not a data-only stub. QA's follow-up survey found CRB's 8 need only a narrow dispatch-table widening, but APG's 6 and ACG's 10 need genuinely new infrastructure (no `ClassId`/`CLASS_META` equivalent exists for those books) — a real architectural split, not a uniform 24-class task.

**UPDATE (2026-07-24, ~22:45 ET) — Ranger's BAB/save dispatch-widening landed and independently verified.** Given the scale of this decision, one adversarial review was run on the scoping plan before greenlighting continuous implementation — it found a real bug: the plan's "Ranger has zero self-block" claim was wrong in a way that risked a false-positive `Computed` status while Ranger's spellcasting was genuinely uncomputed. Backend fixed all 4 findings (real spell-posture diagnostic added, Paladin confirmed unaffected, multiclass combos tested, plan/comment corrected). Lead independently verified every claim rather than trusting the report — 4296/4296 lib, 212/212 desktop, all 4 review requirements confirmed by directly running the relevant tests.

**CORRECTION (~23:45 ET) — "class 1 of 24 done" overstated this; caught by frontend.** The dispatch-widening is real, valuable, and proven safe, but Ranger still cannot reach `Computed` — its spell posture is genuinely uncomputed and the new diagnostic blocks it unconditionally. Frontend investigated an instruction of the lead's before implementing it, found the premise false, live-verified via the real dev build, and correctly made no change. Clarifying with backend whether Ranger's spell posture is next or intentionally deferred, and confirming "done" means "genuinely reaches Computed" for all 24 classes going forward. See risks-and-open-questions.md item 8 for the full record.

**UPDATE (~00:00) — backend confirmed terminology, APG's 6-class dispatch landed correctly labeled.** Backend will state exactly which pillar advanced vs. whether the class actually reaches `Computed` in every future report. APG's BAB/save/HP dispatch (`c511c132`) landed for all 6 classes, explicitly stated as NOT making them reach `Computed`, and proactively avoided the exact multiclass loophole Ranger's review found. Lead-verified 283/283 lib + 212/212 desktop. Next: Ranger's real spellcasting.

**UPDATE (2026-07-24, ~15:35 ET) — operator decided all three gated items.**
Item 27: widen the posture gate to accept any equipment (real engineering
scope; backend to produce a scoping/sequencing plan first, same shape as the
earlier `item-1-architecture-wall-design.md`, before implementation starts).
Item 18: widen Wizard non-Human spell-math now — dispatched to backend, who
must empirically re-verify the formulas for a non-Human Wizard end to end,
not just rely on the earlier read-through scoping note. Item 1's multi-weapon
case: add a real equipment-attachment schema field, sequenced together with
item 27 since both need the same underlying attachment data. New wave of
backend work dispatched; docs updated in `risks-and-open-questions.md` and
`SWARM_TASKS.md`.

---

## (a) Red-green test catalogue completeness — alpha bar §1 item 4

Per-calculation status against `tests/` as of 2026-07-23 (QA baseline survey,
`tranche/6` @ commit `43f8d46`, before wave-1 landed):

| Calculation | Status | Evidence |
| :--- | :--- | :--- |
| Ability scores | Covered | Base scores threaded through `character_input.rs`; asserted across `tests/sd13_*_level1_*baseline.rs` |
| Attack rolls | Covered | `base_attack_bonus` computed and asserted per class/level in `tests/sd13_*` (levels 1-10), `tests/sd18_*` (levels 11-20) |
| BAB/save progression (single-class) | Covered | `tests/sd13_*_base_attack_and_saves.rs` per class |
| BAB/save progression (multiclass stacking) | **Partial** | `tests/sd21_multiclass_fighter_wizard_chassis_computes.rs`, `tests/sd24_multiclass_fighter_wizard_split.rs`, `tests/sd24_multiclass_deterministic.rs` all assert real numeric BAB/save-fraction stacking — but **only for the Fighter/Wizard pair** (good-BAB+good-Fort vs poor-BAB+poor-Fort/Reflex+good-Will). No coverage for 3/4-BAB classes (Rogue, Cleric, Bard, etc.), two-good-save stacking, or 3-class multiclass. |
| Skill allocation | Covered | `tests/sd20_skill_allocation_{class_skill,cross_class,max_rank_cap,parity,untrained}.rs` |
| Spell slot allocation | Covered | `tests/sd13_*_spells_per_day_counts.rs`, `tests/sd20_spellbook_*.rs` (per school) |
| AC | Covered, with a caveat | `defense.baseline_armor_class` asserted with real values (e.g. `tests/ge08_preview_bridge.rs`, `tests/sd21_wizard_chassis_computes.rs` — value 17); equipment AC delta asserted (`tests/sd20_tabletop_readiness_integration.rs:1473` — Chain Shirt +4). Caveat: `src/rules_core/pilot_compute.rs` comments indicate AC stays `claim_blocking`-gated outside certain class-chassis paths — scope of "every reachable AC calc" needs re-verification once frontend widens which chassis are reachable. |
| Durability | **Gap — no production code (definition resolved)** | Lead ruling (recorded in `risks-and-open-questions.md` item 4, 2026-07-23): durability = character survivability display (max/current/temp HP, nonlethal damage, dying/unconscious/death thresholds), not item hardness. Follow-up survey against that definition: `src/rules_core` has only a single isolated `class_chassis.fighter.level_1_hit_points` explanation value (`pilot_compute.rs:7408-7425`); there is no aggregate `max_hit_points` rolled up across a full level-up chain (unlike AC, which has a `sheet.armor_class` cell), no `current_hp`/`temp_hp` fields anywhere in `contract.rs` or `character_input.rs`, and no nonlethal/dying/unconscious/death state machine. Same shape of gap as carry capacity/encumbrance/money — needs backend build, not QA test-authoring, until an aggregate HP field and state machine exist. See appendix below for sourced threshold rules. |
| Carry capacity | **Gap — no production code** | No `carry_capacity`/`carrying_capacity` computation found anywhere in `src/rules_core`. Not a test gap — the calculation itself doesn't exist. |
| Encumbrance | **Gap — no production code** | Same as carry capacity — no encumbrance/load computation found in `src/rules_core`. |
| Money conversion | **Gap — no production code** | No currency-conversion, starting-gold, or wealth-by-level logic found in `src/rules_core`. Only per-item `cost_gp` pricing exists on equipment records. Corroborates frontend's independent finding (see `SWARM_STATUS.md` "Happened" log) that money/currency has no schema field anywhere in the engine. |
| Level-up hit points | Covered | `tests/sd13_fighter_level1_hit_point_baseline.rs`, `tests/sd20_levelup_*.rs` per class |
| Multiclass stacking (general) | Partial | Base chassis (BAB/save) stacking covered for Fighter/Wizard (see above). Skill points / feats / spell-slot stacking under multiclass not independently verified in this survey — needs a follow-up pass once BAB/save gap is closed. |

**Bottom line (as of the original 2026-07-23 wave-1 survey, before wave-1 landed):**
3 of the 12 alpha-bar calculations (carry capacity, encumbrance, money
conversion) had **zero production implementation**, not just zero tests —
these were backend build items, not QA test-authoring items. Multiclass
BAB/save stacking had real tests but only for one class pair.

### Refresh (2026-07-23, after backend's wave-1 close)

Requested by the lead after skill/level-up/bio/feat/money persistence,
BAB/save widening to Fighter/Wizard/Rogue, and the carry-capacity/encumbrance
calc all landed. Updated rows only; unlisted rows are unchanged from above.

| Calculation | Status | Evidence |
| :--- | :--- | :--- |
| BAB/save progression (multiclass stacking) | **Covered, still partial breadth** | Backend's `d20a5b9` widened `compute_multiclass_base_chassis` to Fighter/Wizard/Rogue via the table-driven `compute_generic_table_chassis` path; QA's `8d814e8` adopted 40 downstream tests into the catalogue (verified against real computation output, not transcribed) across 12 files. Still only 3 of 11 core classes are in the multiclass allowlist (Barbarian/Bard/Cleric/Druid/Monk/Paladin/Ranger/Sorcerer remain out, per backend's own doc comment in `pilot_compute.rs` — each has its own pre-existing standalone-only chassis and would need the same coordinated catalogue-adoption pass this one got). |
| AC (equipment bonus) | Covered, caveat resolved for this angle | Backend audited equipment AC bonus wiring during the encumbrance task (`d475097`'s commit message: "equipment AC bonus itself... was already real and wired via equipment_effects.rs prior to this change — audited first, nothing to fix there"). The `claim_blocking`-gating-breadth caveat from the original survey (which classes/postures can reach a `Computed` AC at all) is unchanged and is really the same issue as risks-and-open-questions.md item 1 (the single hardcoded deterministic posture), not an AC-specific gap. |
| Carry capacity | **Covered** | `src/rules_core/encumbrance.rs` (commit `d475097`) implements `carrying_capacity_thresholds`, transcribed and cited from Archives of Nethys (`aonprd.com/Rules.aspx?ID=118`, fetched 2026-07-23) with the source's own >29 extrapolation rule. QA independently cross-checked the table against the real PCGen `load.lst` data twice — once during the original spec pass, once again as part of catalogue adoption. Catalogue entry: `tests/v06_encumbrance.rs` (commit `a7e8971`, 6 tests, independently authored — different fixture items and assertions than the module's own inline tests), plus 2 inline `#[cfg(test)]` tests remaining in the module itself. |
| Encumbrance | **Covered** | Same file/commit as carry capacity. `compute_encumbrance` sums real per-item corpus weight (`WT:` token) across every `EquippedActive`/`SelectedInactive` selection, flags unresolvable items rather than fabricating a zero, and is wired unconditionally into `PilotReceipt.encumbrance` in `contract.rs` (not gated behind the narrow deterministic-posture check that blocks combat/skill totals). Catalogue entry: `tests/v06_encumbrance.rs` (same file/commit as carry capacity — covers the Medium encumbrance tier and a true-zero-loadout case the inline tests didn't). |
| Money conversion | **Covered (conversion only)** | `src/rules_core/money.rs` (commit `67490ac`) implements `copper_to_denominations`/`denominations_to_copper`/`gp_to_copper` using QA's own formula-spec ratios, explicitly flagged (matching QA's own appendix) as not independently PCGen-source-verified. Deliberately scoped to conversion/spend-tracking only — starting-wealth-by-class (PCGen's `GOLD:` token) stays unresolved, per QA's original finding, not guessed. Catalogue entry: `tests/v06_money_conversion.rs` (commit `a7e8971`, 6 tests, including a real CRB corpus item's exact rounding — Torch, `cost_gp: 0.01`), plus 6 inline tests remaining in the module itself. |
| Durability | **Still a total gap** | Re-confirmed via fresh grep across `src/rules_core/` and `tests/`: zero hits for `max_hit_points`/`current_hp`/`temp_hp`/`dying`/`unconscious` beyond the same single isolated level-1 fighter value found in the original survey. Not touched by wave-1. Still the correct wave-2 target per the lead's ruling (risks-and-open-questions.md item 4) and QA's sourced spec appendix below. |

**Refreshed bottom line:** carry capacity, encumbrance, and money conversion
are no longer production gaps and are now fully in the official `tests/**`
catalogue (`tests/v06_money_conversion.rs`, `tests/v06_encumbrance.rs`,
commit `a7e8971`), independently authored and cross-checked, not adopted
by transcription. Durability remains the one calculation with zero
production surface. Multiclass BAB/save stacking breadth (8 of 11 classes
still outside the allowlist) is the other open item. The narrow
deterministic-`Computed`-posture gate (risks-and-open-questions.md item 1)
is a separate, larger issue: backend's fuller scoping found 4 independent
exactness gates. **AC-gate widening was dropped, not landed** — backend
found the headless compute layer (`compute_pilot_base_chassis`) structurally
has no corpus parameter, so real per-item AC math can't be spliced into the
gate cheaply; bridging that would mean either threading a corpus parameter
through ~347 call sites or moving the `Computed`/`Blocked` decision to the
corpus-aware layer entirely — a real architecture decision, flagged to the
operator as a possible future epic rather than a wave-2 item, not something
to assume is coming. Attack-bonus and general-skill posture widening remain
explicitly deferred too, and may share the same headless/corpus-aware split
once scoped. Backend has moved on to durability, money-conversion PCGen
verification, and comparator field-extraction instead, none of which share
this architectural constraint.

### Full resurvey (2026-07-23, wave-2 close / autonomous operation start)

Requested by the lead at the start of fully-autonomous operation (see
`risks-and-open-questions.md`'s operator directive). A great deal has
landed since the last refresh: durability grounded, the Wizard bootstrap/
first-spell/slot-budget three-layer investigation fully closed, and a
QA-found Wizard spell-save-DC gap fixed. Full table below supersedes both
tables above as the current state of truth; rows unchanged since the last
refresh are repeated for completeness rather than left to cross-reference.

| # | Calculation | Status | Evidence |
| :-- | :--- | :--- | :--- |
| 1 | Ability scores | Covered | Unchanged — `tests/sd13_*_level1_*baseline.rs` |
| 2 | Attack rolls | Covered | Unchanged — `tests/sd13_*`/`tests/sd18_*` per class/level |
| 3 | BAB/save progression (single-class) | Covered | Unchanged — `tests/sd13_*_base_attack_and_saves.rs` |
| 4 | BAB/save progression (multiclass) | **Covered, breadth still 3/11** | Fighter/Wizard/Rogue only (`table_class_id` in `pilot_compute.rs`, confirmed unchanged by direct grep this pass). 40 catalogue tests adopted (`8d814e8`). Barbarian/Bard/Cleric/Druid/Monk/Paladin/Ranger/Sorcerer remain outside the allowlist — each has its own pre-existing standalone-only chassis; widening any of them needs the same coordinated catalogue-adoption pass Rogue got, not a code-only change. |
| 5 | Skill allocation | Covered | Unchanged — `tests/sd20_skill_allocation_*.rs`. Note: real-world reachability is separately constrained by the narrow deterministic-posture gate (item 1 in the risks doc), not a skill-calculation defect. |
| 6 | Spell slot allocation | **Covered, and a real enforcement bug found + fixed this wave** | Base per-day counts unchanged (`tests/sd13_*_spells_per_day_counts.rs`, `tests/sd20_spellbook_*.rs`). New this wave: Wizard's slot-budget *enforcement* was silently broken for every real corpus spell (`parse_wizard_spellbook_spell_id` only recognized the one synthetic seed spell's id shape, `<school>.<level>.<name>`, so real spells like "Grease" were dropped from the consumed-slots sum before the over-budget check ever ran — a Wizard could add unlimited real spells with zero enforcement). Found by frontend live-testing, root-caused precisely, fixed by backend (`365b3a1a`, genuine RED→GREEN: fix disabled, reproduced the bug, re-enabled, confirmed green), live re-verified end-to-end through the real Add Spell UI (risks-and-open-questions.md item 13). Tested at the Tauri command layer (`apps/desktop/src-tauri`'s own suite, 188/188) and via `cargo test --lib` (197/197) — not `tests/**`, since the bug was in command-layer spell-id resolution, not a `rules_core` pure-function gap; no adoption action needed on my side, this is the correct home for that coverage. |
| 7 | AC | Covered (baseline + equipment delta), gate-widening dropped | `defense.baseline_armor_class` and equipment AC delta unchanged and covered. Equipment AC bonus itself independently audited as already real and wired (`d475097`'s commit message). Splicing that into the `Computed`/`Blocked` gate for arbitrary loadouts was scoped, greenlit, then **dropped** after backend found the headless compute layer has no corpus parameter (see the "AC-gate widening" note above) — flagged to the operator as a possible future epic, not attempted. |
| 8 | Durability | **Covered** | `src/rules_core/durability.rs` (commit `0aeed25a`) grounds `compute_max_hp` (maximized level-1 die + average-rounded-up every level after, PF1's named non-rolling default, floored at 1 HP/level) and `classify_durability` (Normal/Staggered/Disabled/Unconscious/Dying/Dead per standard PF1/d20 SRD thresholds). Scoped to single-class Fighter/Wizard/Rogue, same reason as the multiclass BAB/save dispatch (which single level was character-level-1, for the maximized die, is genuinely ambiguous from multiclass `CharacterClassLevel`'s cumulative-level shape). Wired into `SelectedParityDimensions::from_pilot_receipt` (not a `receipt.durability` field the way encumbrance got one) and independently PCGen-verified end-to-end (`tests/sd26_pilot_case_verification.rs`, max_hp=12 matched exactly against a real PCGen export). Catalogue entry: `tests/v06_durability.rs` (commit `ec48b501`, 12 tests, independently authored — different classes/levels than the module's own 13 inline tests, plus a direct second-path cross-check of the PCGen-verified max_hp=12 value), plus the 13 inline tests remaining in the module. |
| 9 | Carry capacity | Covered | Unchanged — `tests/v06_encumbrance.rs` |
| 10 | Encumbrance | Covered | Unchanged — `tests/v06_encumbrance.rs` |
| 11 | Money conversion | Covered (conversion only) | Unchanged — `tests/v06_money_conversion.rs`. Starting-wealth-by-class remains unresolved (risks item 7), correctly not guessed. |
| 12 | Level-up hit points | Covered | Unchanged — `tests/sd13_fighter_level1_hit_point_baseline.rs`, `tests/sd20_levelup_*.rs` |
| — | Wizard spell save DC (not a named bar-4 item, but the same DC family as Paladin/Ranger/Sorcerer/Bard) | **Covered, new this wave** | QA found zero `wizard_spell_save_dc` computation while doing item 11's LST cross-check (risks item 12) — the other 4 caster classes all had it, Wizard didn't; not a bug, an explicitly-disclaimed pre-existing gap. Fixed by backend (`3b397315`): `10 + spell_level + intelligence_modifier`, confirmed against real PCGen data (`cr_classes.lst` `SPELLSTAT:INT`). QA adopted (`e95112a1`): `tests/sd13_wizard_spell_save_dcs.rs` (7 tests, independently authored — caught and corrected one wrong assumption about multiclass behavior by running the real computation rather than guessing), plus fixed the one downstream negative-control test the new records made stale. |

**Resurvey bottom line (updated after durability's catalogue adoption,
`ec48b501`):** all 12 alpha-bar calculations are now either fully covered
or, for multiclass BAB/save breadth specifically, covered-but-narrow (see
below) — every production gap identified across every prior survey (carry
capacity, encumbrance, money conversion, durability) is closed and
catalogued, none left as backend-only inline tests. Multiclass
BAB/save breadth (3/11 classes) remains the largest *known* calculation gap
against the bar's "any class" framing, though it is a well-understood,
bounded, repeatable pattern (Rogue's own widening + 40-test adoption is the
template) rather than new design work. Beyond raw calculation coverage, the
bigger alpha-bar story this wave is Wizard becoming a **second genuinely
UI-reachable class** (class creation, first-spell bootstrap, and slot-budget
enforcement all live-verified end-to-end) — this is bar items 2/3 progress,
not item 4 calculation coverage, but it's the more consequential wave-2
result. The narrow deterministic-`Computed`-posture gate (risks item 1) and
the still-unwidened AC/attack-bonus/skill-posture architecture split remain
the single largest structural distance between current state and "any
class... reaches Computed for the choices a tester actually makes" — this
is a UI-reachability problem layered on top of calculation correctness, not
fixed by adding more calculations.

### Comprehensive consolidation (2026-07-23/24, fully-autonomous-session checkpoint)

Requested by the lead as a checkpoint after a long autonomous run covering
several real defects found and fixed beyond the original calculation-gap
surveys above. This section is the current, authoritative picture; it does
not repeat every historical detail already recorded above, only what has
materially changed or newly landed.

**Calculation coverage** (§a's 12-item table): unchanged from the "Full
resurvey" table above in outcome — all 12 are covered, multiclass BAB/save
breadth is still 3 of 11 classes (Fighter/Wizard/Rogue), same well-understood
bounded pattern, not attempted further this session (no new class was
greenlit for widening). What *did* change is the accuracy of several
already-"Covered" rows, captured as defects below — coverage existing is not
the same as coverage being *correct*, and this session found several real
gaps between the two.

**Alpha-bar items 1-3/7** (§d below is largely stale as of the original
wave-1 draft; the current truth):
- **Item 2/3** (create + advance a character of any class/race): materially
  further along than believed. Fighter, Wizard, and Rogue are all now
  confirmed UI-reachable end-to-end (creation, level-up, multiclass dip) —
  Wizard needed a real three-layer bootstrap fix (class acquisition, first
  spell, slot-budget enforcement, all live-verified), Rogue needed none
  (confirmed reachable with zero gap). **Race support for these three
  classes is not Human-gated** — this was a stale, never-verified assumption
  the swarm inherited and has now disproven: Elf Wizard and Elf Rogue both
  live-verified reaching `Computed`/`Saved` through the real creation UI.
  The other 8 core classes (Barbarian/Bard/Cleric/Druid/Monk/Paladin/
  Ranger/Sorcerer) still have zero chassis computation for any race — this
  is unchanged and is not a quick fix (each needs its own multi-epic
  calculation engine, not a UI/wiring fix).
- **Item 4** (every reachable calc matches PCGen): the 12 named calculations
  are covered per the table above, but "every reachable calculation" is
  narrower in practice than the bar's framing suggests — see the
  bar-distance assessment below.

**PCGen-delta defects found this session** (§b was empty this whole swarm
until now — populated for real below):

| # | Defect | Status |
| :-- | :--- | :--- |
| 1 | Wizard spell-save-DC: no computation existed at all (Paladin/Ranger/Sorcerer/Bard all had it) | **Fixed** (`3b397315`), catalogue-adopted (`e95112a1`) |
| 2 | Wizard spell-slot-budget enforcement: real corpus spells silently bypassed the over-budget check (only the one synthetic seed spell's id shape was recognized) | **Fixed** (`365b3a1a`), live re-verified through the real Add Spell UI |
| 3 | Class-skill-modifier bug: `compute_selected_skill_modifiers` applied the Climb/Intimidate/Swim class-skill `+3` unconditionally — silently wrong for Wizard (whose real class-skill list includes none of the three), coincidentally right for Rogue | **Fixed** (`93a0636d`), catalogue-adopted (`3b843add`), independently re-verified against the real PCGen corpus citations before adoption |
| 4 | Racial ability-modifier gap: Elf/Dwarf/Gnome/Halfling each silently missing one real `+2` mental-ability racial component (Elf: INT, Dwarf: WIS, Gnome: CHA, Halfling: CHA) — the code's own comment mischaracterized Elf's as an "out of scope alternate variant" when it's the CRB-standard default | **Fixed for all 4 races** (Elf `9ec0e036`, Dwarf/Gnome/Halfling `2f05dee4`), **catalogue-adopted for all 4** (Elf `e9d02c25`, Dwarf/Gnome/Halfling `fb01768d`) — each real PCGen citation independently re-verified before adoption, not trusted from the commit message |
| 5 | Racial Small-size effect miscategorization: Gnome's and Halfling's size explanations claim "no numeric effect to attack rolls, AC..." despite correctly citing `SIZE:SMALL` — real PF1 Small size grants +1 AC/+1 attack/-1 CMB-CMD/+4 Stealth | **Fixed** (`2f05dee4`, bundled with defect 4), **catalogue-adopted** (`fb01768d`) — text-only correction; `compute_combat_baseline` has no size-modifier term for *any* race today, so this doesn't change a computed value, only stops an incorrect claim |
| 6 | Feat-effects engine: verified concretely (built a real fixture, added Toughness to `selected_feats`, ran the real `build_pilot_headless_receipt` entry point) that **no feat outside the 3 hardcoded into the deterministic posture gate (Power Attack, Dodge, Weapon Focus) has any mechanical effect anywhere** — confirmed by grep this isn't Toughness-specific, there is no general feat-effects computation in `pilot_compute.rs` at all | **Not a quick fix** — logged as its own architecture gap, linked to the existing AC/attack-bonus/skill-posture widening item (risks-and-open-questions.md item 1) rather than assigned to backend as routine work. Not attempted this swarm. |
| 7 | **MAJOR — CreateCharacterForm never actually submitted racial ability adjustments, for any of the 4 fixed-adjustment races, since the form was first built.** `calculatedScore()` (raw + racial adjustment) was computed for the on-screen preview only; the submitted `abilityScores` used raw, unadjusted `rawScore()` instead — every non-Human Elf/Dwarf/Gnome/Halfling character ever created had silently wrong ability scores, independent of and predating this session's engine-side explanation-text fixes (defect 4 above only corrected what the text *described*, not what got *submitted*) | **Fixed** (`f2c616ed`), live-verified end-to-end for Elf (disk-confirmed correct DEX/CON/INT cascade) and Dwarf (disk-confirmed `constitution:16/wisdom:14/charisma:6` on a fresh character created through the real UI). Gnome/Halfling verified via real production-code execution (actual function + actual race-catalog data, not a reimplementation) after a session-scoped GUI environment blocker (since fixed, `f6fe0df2`) prevented completing their live-disk leg — accepted as sufficient given the mechanism is unconditional/race-agnostic and already twice disk-proven |
| 8 | Fighter multiclass/race level-lookup gap, 3 instances: `validate_fighter_feat_choice_legality` and two sibling checks in `unmet_combat_posture_conditions` used single-class-only or Human-only level lookups instead of the multiclass-aware `fighter_level_in_mix`, silently skipping validation for non-Human and/or multiclass Fighters — one instance empirically confirmed exploitable (a Human Fighter1/Rogue3 with a wrong bonus-feat choice produced zero diagnostics before the fix) | **Fixed, systematic sweep complete** (`0eb9ea65`, `32289cb4` follow-up, `68721ca0`) — all 4 `_legality`/`_conditions`/`validate_` gate functions in `pilot_compute.rs` checked, no further instances. Currently no live UI attack surface (the create/level-up flow hardcodes canonical choices for the slots these checks protect) — real defense-in-depth for the command/API layer, not an active user-visible bug today |
| 9 | `skill_allocation.rs`'s class-skill recognition was Fighter-only, so neither Wizard nor Rogue had ANY grounded class-skill posture — silently left the PF1 cross-class rank cap completely unenforced for both (confirmed empirically: a level-1 Wizard could dump 5 ranks into a cross-class skill with a real cap of 1, zero diagnostic) | **Fixed** (`21f815c1`), grounded against the real PCGen corpus (Rogue: all 5 bounded skills, `cr_abilities_class.lst:2838`; Wizard: genuinely empty, `cr_abilities_class.lst:2565`, checked not assumed), **catalogue-adopted** (`d35521ec`, `2ab19bc7`) — real fixture-driven tests through the actual parser, both citations independently re-verified, and a fresh-eyes re-check found and closed a real gap in an *existing* test that used the bare string `"wizard"` instead of the real `"class:wizard"` id and so never actually exercised Wizard recognition |
| 10 | Wire-serialization bug: `CreateCharacterResponse::Saved`'s `corpus_derived` field serialized literally as snake_case on the wire (the enum's `kind` tag deliberately keeps no `rename_all`, so a bare fix would have broken every `outcome.kind === 'Saved'` check), silently `undefined` on the TS side — the Spells/Gear tabs looked stale right after a real, successful mutation | **Fixed** (`498679d1`, per-field `#[serde(rename = "corpusDerived")]`, an identical latent bug in `PurchaseEquipmentResponse` caught and fixed proactively in the same commit) — independently re-verified by QA with a real RED reproduction (temporarily reverted the fix, watched the exact symptom reappear, restored, confirmed GREEN) and an independent re-sweep of every `#[serde(tag = ...)]` enum in the crate, not just the ones already named |

**Bar-distance assessment (honest current picture):** the alpha bar is
**not** met yet, and the remaining distance is now well-characterized rather
than vague:
1. **Multiclass breadth** — 3 of 11 classes in the BAB/save-stacking
   allowlist. Bounded, repeatable, not attempted for the other 8 this
   session (no explicit greenlight to widen further).
2. **Class-chassis breadth** — 8 of 11 classes have zero base-chassis
   computation for *any* race, not a UI-reachability problem, a genuine
   missing-engine problem per class.
3. **Posture narrowness** — even for the 3 working classes, only one exact
   equipment/skill/feat combination ever reaches `Computed`. AC-gate,
   attack-bonus, and general-skill-posture widening were all scoped and then
   **dropped** this swarm after backend found the real blocker is
   architectural (the headless compute layer has no corpus parameter) —
   flagged to the operator as a possible future epic, not a wave-2 item.
4. **Feat effects** — confirmed nonexistent beyond the 3 feats hardcoded
   into the posture gate itself. Same shape of problem as item 3 (a
   structural gap, not a missing calculation), newly discovered this
   session.
5. **What *is* solid**: the 12 named calculations for the 3 working classes
   are genuinely correct and PCGen-cross-verified once you're inside the one
   supported posture — the defects found this session were about *breadth*
   (which classes/races reach a correct answer) and *honesty* (comments
   claiming something is out of scope when it's real), not about the core
   arithmetic being wrong once a build is actually `Computed`. Money
   purchasing is now a real atomic transaction (`purchase_equipment`), and
   the render-staleness/corpus-derived wire bugs found along the way are
   both closed.

### Second checkpoint (2026-07-24, post-race-bundle and post-submission-bug)

A second round of real defects landed after the checkpoint above was
written, closing out the remaining open threads from it rather than
introducing new scope:

- **The 4-race ability-modifier gap (defect 4) is now fully closed, not
  partial.** Dwarf/Gnome/Halfling's engine-side fix landed (`2f05dee4`) and
  was catalogue-adopted the same session (`fb01768d`), each citation
  independently re-verified against the real PCGen corpus rather than
  trusted from the commit message. All 4 races now correctly ground their
  real 3-stat racial adjustment.
- **A materially bigger, independent bug was found and fixed underneath
  it**: the create-character form was never actually *submitting* any
  race's adjusted ability scores — only displaying them — since the form
  was first built (defect 7). This predates and is unrelated to the
  engine-text fixes; it means every non-Human character created through
  the shipped UI, this entire swarm and before, had silently wrong ability
  scores baked into the saved file. Fixed (`f2c616ed`) and verified: Elf
  and Dwarf both disk-confirmed correct through a real create-character
  UI walkthrough; Gnome/Halfling confirmed via direct execution of the
  real production function against the real race-catalog data (a session-
  scoped GUI environment collision between concurrent agents, since fixed,
  prevented completing their disk leg — accepted as sufficient given the
  fix is unconditional across races and twice disk-proven already).
- **A third, unrelated defect class was found and closed by a systematic
  sweep**: Fighter's feat-choice-legality gate had the same "single-class
  or Human-only level lookup" blind spot in 3 separate places, one of them
  empirically confirmed exploitable before the fix (defect 8). All 4
  candidate gate functions in the file were checked; sweep is complete.
- **Housekeeping resolved, not newly found**: DR exposure through the DTO,
  the money-panel/equipment-purchase atomic-transaction gap, and
  `load_saved_character` exposing `spells_selected` (risks-and-open-
  questions.md items 6/9/9a) all landed as real backend work this session
  — closing three previously-logged "backlog, non-blocking" items outright
  rather than leaving them to accumulate.
- **Deferred, not fixed, and correctly so**: a non-Human Wizard's
  spell-specific grounding (spell-save-DC, spellbook-slot ceiling) never
  runs at all — the one function that both grounds it and enforces its
  level-3 ceiling is itself Human-gated. BAB/saves/HP for non-Human Wizards
  remain correct (a separate, already-widened path). Ruled a completeness
  gap (nothing computes *wrong*, a subsystem simply doesn't run), not a
  correctness bug — filed alongside the feat-effects and AC/attack-bonus
  architecture items rather than fixed or blocked this wave.

None of this changes the bar-distance shape below — multiclass/class-chassis
breadth and the posture-narrowness/feat-effects architecture gaps are
untouched — but it meaningfully strengthens confidence in the *correctness*
of what the 3 working classes already claim, and closes out several
previously-open threads cleanly rather than leaving them to drift.

### Third checkpoint (2026-07-24, post-skill-allocation-fix, sweep, and scoping synthesis)

A short round focused on closing out the last silent-correctness bug this
swarm's sweep pattern found, plus a synthesis pass over the architecture
gaps that remain:

- **`skill_allocation.rs`'s Fighter-only class-skill recognition (defect 9)
  is fixed and catalogue-adopted.** Same failure shape as the earlier
  class-skill-modifier bug (defect 3) — a silently wrong number with no
  claim-blocking diagnostic — but on rank enforcement rather than a
  modifier value: neither Wizard nor Rogue had any grounded class-skill
  posture in this module, so the PF1 cross-class rank cap never engaged
  for either. Both PCGen citations independently re-verified. Catalogue
  coverage went through a real fresh-eyes re-check (requested by the lead
  after a quota-outage stewardship landing) that found and closed a real
  gap in an *existing* test — it used the bare string `"wizard"` rather
  than the real `"class:wizard"` id, so it never actually exercised Wizard
  recognition in a multiclass union despite its name — and caught its own
  mid-draft assertion error (asserted the cross-class cap value where the
  real class-skill cap value applied) by running before trusting it.
- **A systematic sweep for the same failure shape elsewhere came back
  clean** (risks item 21): backend checked `pilot_compute.rs`,
  `skill_allocation.rs`, `durability.rs`, and `money.rs` for any remaining
  Fighter-only-grounded computation with a silent downstream consequence.
  One candidate (`explain_rogue_level1_chassis`'s single-class-only gate)
  was traced and ruled out — its output has no downstream consumer, so a
  missing record there is cosmetic, not a silent wrong number, the same
  shape already established for non-Human Rogue elsewhere. **This is the
  signal that closes out the "keep watching for silent bugs" thread**: the
  three working classes' shared computation paths are now confirmed clean,
  not just unexamined.
- **`docs/release/v0.6/future-epic-scoping.md`** consolidates the three
  remaining gaps (risks items 1/17/18) side by side for the operator's
  eventual review: the headless/corpus-aware architecture wall (attack-
  bonus enhancement math, skill armor-check-penalty), the feat-effects
  engine absence, and the non-Human Wizard spell-math gap. Confirms they
  are independent — fixing the architecture wall would unlock the first
  gap's two sub-problems but buys nothing for feat effects or Wizard's
  spell math, so none of the three blocks starting on either of the others.
  No new facts, a cross-item synthesis of what's already independently
  established in `risks-and-open-questions.md`.
- **Two DTO-exposure fixes (defects from risks items 6/9/9a) got real
  fixture-driven catalogue coverage**: the DR-exposure DTO field
  (`PilotSnapshot.damage_reduction`) now has a test driving a real
  Barbarian fixture through the real compute pipeline (stronger than the
  synthetic-receipt shape backend's own inline tests use, since Barbarian
  can't reach `Computed` today and a real end-to-end proof needs one
  synthesized field rather than a fully fabricated receipt). The other two
  (`purchase_equipment` atomicity, `spells_selected` exposure) are
  confirmed structurally unreachable from `tests/**` — entirely in the
  separate `codex-desktop` crate with no `rules_core` equivalent to
  complement — already correctly covered by backend's own inline
  Tauri-layer tests, same crate boundary established for Rogue's UI
  reachability earlier this swarm.

Net effect: the bar-distance picture is unchanged in shape from the second
checkpoint, but is now backed by an explicit clean-sweep result rather than
an implicit absence of further findings, and the three remaining
architecture gaps have a single reference document instead of being spread
across several risks-doc entries.

### Fourth checkpoint (2026-07-24, independent-verification sweep)

Distinct from the checkpoints above: those document what *landed*. This one
documents what's been *independently re-checked*, by whom, and how deep —
the coverage of the verification itself, which is what a future close-out
pass needs to know it can rely on rather than re-derive from scratch.

**Method.** Per the lead's ask (following the pattern already established
for the 4-race ability-adjustment work), QA scanned the full commit history
(`git log --oneline origin/develop..origin/tranche/6`, 160+ commits) for
every real `feat`/`fix`/`frontend` commit — as opposed to `docs`/status
commits — and worked down the list picking whichever unverified item looked
highest-value or highest-risk, giving each the same standard: read the
actual code/diff directly rather than re-asserting the commit message, run
the real tests personally rather than trusting a reported pass count, and
reproduce RED before trusting a claimed fix wherever that was cheap to do.

**17 areas independently verified clean this session, each with its own
concrete method (not just "looks fine"):**

| Area | Commit(s) | What was independently confirmed |
| :--- | :--- | :--- |
| Defense-tab DR wiring | `26ac0704` | TS type genuinely added to the shared `PilotSnapshotDto` (not ad-hoc); absent-case JSX renders `null`, never a fabricated zero; Barbarian's unreachability traced through `compute_class_chassis` → `table_class_id` → `is_supported_multiclass_mix` — no code path lets it reach `Computed`, not assumed from the claim |
| Durability status thresholds | `durability.rs` | All 6 `classify_durability` states independently re-derived against real PF1/d20 SRD rules from first principles, not copied from the module's own doc comment; matched backend's own parallel check exactly |
| Class-support labeling (all 11 CRB classes) | `34635157` | Every single row checked individually against `pilot_compute.rs` source (not spot-checked) — `supported_wizard_level`/`supported_rogue_level` genuinely never check `race_id`; all 8 `human-diagnostics-only` classes independently confirmed to share the identical gate pattern and all fall outside `table_class_id`'s 3-class allowlist |
| Wire-serialization fix | `498679d1` | Ran the 2 new tests personally; **reproduced RED myself** (temporarily reverted the `#[serde(rename)]`, watched the exact snake-case symptom reappear, restored, confirmed GREEN); independently re-swept the crate for other `#[serde(tag = ...)]` enums rather than trusting the "swept the rest" claim — found the same 4 backend already named, confirmed the 2 unaffected ones genuinely have no underscored fields |
| Feat catalog exposure | `89c3710a` | Per-category counts (50/110/8/17=185) re-derived by grepping the raw data files directly, not trusted from the doc comment; confirmed the Tauri wrapper does a true 1:1 map with no filtering; confirmed the "safe to append" claim in `unmet_combat_posture_conditions`'s own source (a presence check, not an exact-set match) |
| Level-up choice/skill persistence | `7694b227` | The "exactly one colon" grammar constraint traced to its real origin (`git log -S`, predates the swarm by 3 days — not invented to justify this fix); atomicity proven by reading the actual round-trip test, which reloads from disk and checks all three mutated fields landed together |
| Bio field persistence | `0ab784df` | The "already-saved" check confirmed to use a real `SavedCharacterStore::load`, not the naive `root.exists()` the commit says it deliberately avoided; overwrite-not-append proven via the actual two-save-then-reload test |
| `set_skill_allocations` | `e0a0bda4` | Wholesale-replace confirmed in source (`= skill_allocations`, no merge); the "reordered set proves replacement" test's premise re-derived (traced the seed fixture's real default order first, then confirmed the reversed submission round-trips exactly) |
| Money balance persistence | `67490acb` | Negative-balance rejection confirmed in source before any write; confirmed the DTO derivation reuses the exact `money::copper_to_denominations` function QA's own `tests/v06_money_conversion.rs` already covers, not a parallel reimplementation that could drift |
| `skill_allocation.rs` cross-class fix | `21f815c1` | Both PCGen citations re-verified against the local corpus checkout directly; **fresh-eyes re-check** (requested after a quota-outage stewardship landing) found and closed a real gap in an *existing* test that used the bare string `"wizard"` instead of the real `"class:wizard"` id |
| `combat.base_attack_bonus` dimension | `cda3bf1c` + `b8eff433` | All 4 mechanically-specified test files updated and run personally, including both real PCGen engine invocations (not just the fast synthetic ones), before backend's commit was allowed to land per the cross-surface protocol |
| LevelUpDialog wiring | `e8e45976` | Mechanics (hit-die choice, skill-allocation omission) confirmed clean; traced a comment's staleness by comparing commit timestamps directly, then confirmed the gap it describes is live-reachable today (Fighter's `levelOptions` includes a real bonus-feat level), not theoretical |
| SkillAllocationDialog wiring | `75200fcb` | `skillIdFor`'s "5 confirmed ids" claim checked against `skill_key_ability_modifier` directly; the "unrecognized ids are inert" claim confirmed via the actual `continue`-on-`None` branch, not assumed from the absence of an error |
| Bio editor wiring | `94a38657` | Character-switch reload's `cancelled`-guard against a stale in-flight load read directly in the `useEffect`, not just claimed by the commit |
| Feat picker + Feats tab | `febf4d80` + `aa611ce1` | Every one of the 6 `toCharacterMutationRefresh` call sites individually traced to confirm each threads the correct feat list (unchanged vs. plus-the-new-feat) for its specific mutation |
| Wizard spell-pick routing | `d55a919a` | The Wizard-preference-over-`heldClasses[0]` logic and the atomic-vs-plain routing both confirmed directly in the diff |
| Actions tab + dead-tab removal | `743c358b` | Confirmed `ActionsTab` is a pure display component with no new computation or backend call; version-bump fixture fix confirmed genuine (`0.6.0-test` now reads correctly) |

**Money panel (`59d5bc0a`) — clean on the static half, live-UI leg
inconclusive.** The `gpToCopper`/`gp_to_copper` formula match and the
boundary wiring were confirmed directly. The live-UI leg hit a real but
unrelated environment quirk (this session's window reports 1920×1200 via
`xdotool` but genuinely renders/screenshots at 1280×900, and the Load
Character dialog's action-button row wasn't reachable at any coordinate
tried) — not a code bug, correctly not force-fit into a false pass. Confirms
the `RUN_DESKTOP_AGENT` fix (`f6fe0df2`) holds under real concurrent use
though: this session's `:98` display and frontend's simultaneous `:96`
session never interfered with each other.

**2 real findings surfaced by this sweep** (both filed to the correct
owner, not fixed by QA — see `risks-and-open-questions.md` items 22/23/25
for full detail):
- Item 22 (now RESOLVED): `characterProgression.ts` — the module behind the
  classSummary comma-separator fix (`d03bc89d`) — had zero dedicated test
  coverage at all; frontend closed it same-day with a real
  `characterProgression.test.ts` (12 functions covered, one genuine RED
  caught along the way in a title-case regex assumption).
- Item 23 (now RESOLVED, landed minutes after this checkpoint was written):
  LevelUpDialog's own comment about why feat picks aren't collected at
  level-up was stale (the blocker it names was closed hours after that
  commit landed), and the underlying gap — real, currently-reachable at
  Fighter's level 2 — has since been fixed (`ddfc66bb`): a new
  `levelGrantsFeat` predicate detects a feat-granting level and routes
  through the same real feat picker the Feats tab uses, live-verified both
  branches (a Dwarf Fighter 1→2 picked Cleave; a Wizard 1→2 with no feat at
  that level leveled up uninterrupted).
- Item 25 (open, backlog, systemic): a recurring pattern across 4 frontend
  persistence-wiring modules (`characterProgression.ts` — since resolved —,
  `skillsModel.ts`/`setSkillAllocations`, the LevelUpDialog module, and
  `characterBio.ts`) of sound logic shipped with zero dedicated test file.
  Confirmed as a real, systemic gap rather than four coincidences; not
  re-flagged a fifth/sixth time once the pattern was established.

**What this checkpoint changes**: nothing about the bar-distance shape —
still not signing. What it adds is a documented, itemized answer to "how
much of what landed has actually been independently re-checked, and how,"
for whenever the operator or a future close-out pass wants to know the
verification depth wasn't assumed.

**Not signing the attestation.** Per §4.4's "Done" criteria, this requires
every shipped calculation having red-green coverage (true) *and* the
operator's alpha bar in §1 holding (not true — items 2 and 4 above are real,
acknowledged gaps, not stub surfaces, but still gaps against "any class...
matches PCGen"). This checkpoint is for visibility, not closure.

## Current-state summary (2026-07-24, full closure of the bounded backlog)

The four checkpoints above are an incremental log of *how* the picture got
here; this section is the *destination* — one coherent current-state read,
so anyone (operator, future close-out pass, a teammate picking this back up
cold) can get the whole picture without walking the history. Nothing below
contradicts the checkpoints above; it supersedes them only in the sense of
being the up-to-date summary, not a new finding.

**Where things stand.** As of this pass, all 26 numbered items in
`risks-and-open-questions.md`'s "Open questions" section are resolved or
correctly deferred. Two consecutive backend self-directed scans (the
Fighter-only-grounding correctness sweep, the parity-comparator field
sweep) each came back clean on their own second pass — a genuine signal,
not an absence of looking. QA's completeness sweep independently
re-verified essentially every real `feat`/`fix`/`frontend` commit the swarm
produced (17 areas verified clean, 2 real gaps found and since closed).
The bounded, same-session backlog — bugs, wiring gaps, missing tests,
missing UI surfaces for already-computed data — is genuinely exhausted
right now. That is a narrower claim than "the alpha bar is met," addressed
directly below.

**Full defects table, brought current.** The 10-row table above (under
"Comprehensive consolidation") is the complete, current list of every real
PCGen-delta/correctness defect found across the whole swarm — nothing has
been found since defect 10 (the wire-serialization bug) that isn't already
in it. All 10 are fixed and catalogue-adopted where `tests/**` coverage
applies, or explicitly logged as architecture-level and not attempted,
never left ambiguous.

**Beyond correctness defects, the other real work this swarm closed**
(UI-reachability and wiring gaps, not PCGen-delta correctness bugs, so
tracked in `risks-and-open-questions.md` rather than the defects table
above): the full Wizard three-layer UI-bootstrap chain (class acquisition,
first-spell, slot-budget enforcement); Rogue's UI reachability (zero gap);
race-agnostic reachability for all three working classes, disproving a
stale "Human only" assumption; the feat catalog + picker + persisted feat
list; bio, money, skill-allocation, level-up, and durability persistence
end to end; a new PCGen parity dimension (`combat.base_attack_bonus`); a
feat-pick affordance at feat-gaining level-ups; a Load-list staleness fix;
and a full pass closing 5 frontend modules' test-coverage gaps that QA's
sweep surfaced (items 22/25 in the risks doc).

**Bar-distance assessment, restated plainly against what's verified now:**
1. **Multiclass breadth** — still 3 of 11 classes (Fighter/Wizard/Rogue) in
   the BAB/save-stacking allowlist. Unchanged all session; bounded and
   repeatable (Rogue's own widening is the template) but not attempted
   further — no greenlight to widen beyond these three this wave.
2. **Class-chassis breadth** — still 8 of 11 classes with zero base-chassis
   computation for *any* race. Confirmed multiple times this session, not
   assumed. Each needs its own multi-cycle calculation engine — not a
   wiring fix, a genuine missing-engine problem per class.
3. **Posture narrowness** — even for the 3 working classes, the
   `Computed`/`Blocked` gate still only accepts one exact combination.
   AC/attack-bonus/skill-ACP widening was scoped, then correctly dropped
   after backend found the real blocker is the headless/corpus-aware
   architecture split — see `future-epic-scoping.md`, not reattempted this
   wave, flagged for the operator as a real future epic.
4. **Feat effects** — confirmed nonexistent beyond the 3 feats hardcoded
   into the posture gate (Power Attack, Dodge, Weapon Focus). Same
   architecture-gap shape as item 3, not a missing calculation.
5. **Non-Human Wizard spell-math completeness** — spell-save-DC and the
   spellbook-slot ceiling never run for any non-Human Wizard (the one
   function grounding both is Human-gated). BAB/saves/HP remain correct
   for non-Human Wizards via a separate, already-widened path — this is a
   completeness gap (a subsystem that doesn't run), not a correctness bug
   (a value that's wrong).
6. **What *is* solid**: the 12 named alpha-bar calculations are genuinely
   correct and PCGen-cross-verified once a build reaches `Computed`, for
   all three working classes — every defect found this session was about
   *breadth* (which classes/races reach a correct answer) and *honesty*
   (claims that overstated or understated real scope), not the core
   arithmetic being wrong once inside the one supported posture.

**What's genuinely left** (matching the lead's own bounded-backlog
assessment, risks doc item 3):
- **Architecture-level, not bounded work** (full detail in
  `future-epic-scoping.md`): the headless/corpus-aware wall (blocks
  attack-bonus and skill-ACP widening), the feat-effects engine's total
  absence, and Wizard non-Human spell-math completeness. None of these are
  a same-session task; each is confirmed independent of the other two —
  fixing one buys nothing toward the others.
- **Class/multiclass breadth**: the other 8 CRB classes, each its own
  multi-cycle engine effort.
- **Operator-only, not an engineering call**: starting-wealth-by-class
  (risks item 7) — a content-provenance/licensing question, exhaustively
  searched and confirmed absent from every real corpus source available
  here, not an open lookup.
- **Outside this swarm's control**: the observer-lane status (risks-doc
  Risks §5) — operator-side infrastructure.

**Not claiming the alpha bar is met.** It isn't — on class/race breadth (2
of 4 books' worth of classes genuinely reachable) and on the three
architecture gaps above. What this summary says is narrower and, we
believe, fully substantiated: the bounded backlog reachable without an
architecture decision or an operator content call is genuinely exhausted,
not abandoned early or padded with busywork to look active.

**Not signing the attestation** — same reasoning as every checkpoint above,
restated once more for anyone reading only this section: §4.4's "Done"
criteria needs both red-green coverage on every shipped calculation (true)
and the operator's alpha bar genuinely holding (not true, for the breadth
and architecture reasons above). This summary is for visibility and
closure-readiness, not a substitute for that sign-off.

## (b) PCGen-delta defects found and fix/ticket status

See the consolidated table above (Comprehensive consolidation section) for
the authoritative, current list — kept there rather than duplicated here to
avoid two sources of truth drifting apart. Historical note: this section sat
empty through wave-1 close and the original wave-2 resurvey, since no
calculation-changing defect had landed yet at either checkpoint; the first
real defects (Wizard spell-save-DC, slot-budget enforcement) landed after
the wave-2 resurvey was already written, which is why they don't appear in
the tables above this one.

## (c) Four-check wired-integration audit results

### Interim audit checkpoint (2026-07-24)

Run early at the lead's request, since the swarm's remaining distance from
the alpha bar now looks architecture-bounded rather than "more bugs to
find" — a good point to surface any wired-integration violation while
backend/frontend can still fix it live, rather than as a surprise at actual
closure. Per the operator's ceremony waiver (this doc, top), the receipt
ceremony is waived but the audit itself is not — this is an *interim*
checkpoint, not the final one; the closure-time audit (§7.1 of
`release-swarm.md`) still runs separately against the final combined diff
before the closure PR opens.

**Method**: extracted every added line (`+` lines only, diff metadata
excluded) from `git diff origin/develop...origin/tranche/6` (116 files,
10,576 insertions across the full swarm to date), tagged by source file,
and ran all four greps by hand against that extraction.

**Result: clean. Zero real violations found.**

1. **Forbidden tokens** (`\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b`, case-insensitive, outside tests/docs): 2 files matched, 5 lines total, all false positives on inspection — `apps/desktop/src-tauri/src/pf1_adapter.rs` has two doc-comment lines explicitly describing a placeholder that was *removed* ("not a synthetic placeholder", "no reason to keep seeding a placeholder now that the real path [exists]"); `apps/desktop/src/characterHub/CharacterSheet.tsx` has a real `placeholder="gp amount"` HTML input attribute (the doctrine's target is stubbed logic, not input hint text), a comment explaining why a duplicate "coming soon" tab was deliberately *removed*, and a comment explicitly documenting a case where fabricating feat options *would* violate the doctrine — i.e. recording that they correctly did **not** stub it, not that they did.
2. **No-op handlers** (`onClick={()=>{}}` / `onClick={undefined`): 0 real hits. The only match was this document's own checklist line describing the check.
3. **Mock-library leaks outside tests** (`mockResolvedValue|mockReturnValue(|vi.mock(|__mocks__`): 0 real hits, same self-reference-only result.
4. **`"Would ..."` strings**: 0 real hits outside this doc's and `risks-and-open-questions.md`'s own descriptions of the check; broadened the search for near-miss phrasing (`Would compute/return/apply/resolve/handle/implement/support/do/be/have`, unquoted) to sanity-check the exact-match regex wasn't too narrow — still zero hits.

Raw grep commands (reproducible): each check run against a Python-extracted
`file\t<added-line>` table built from the diff, e.g. for check 1:
`grep -iE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' <extracted> | grep -vE '^(tests/|docs/|.*\.test\.(ts|tsx)|.*_test\.rs)'`.

**Re-verified independently (lead, 2026-07-24)**, per the doctrine's
"executed by QA, re-verified by the lead" requirement: extracted added
lines fresh from `git diff origin/develop...origin/tranche/6` myself
(without reading QA's extraction) and ran all four greps. Same 5 lines on
check 1, same zero-real-hits result on checks 2-4. Confirms QA's audit —
clean, no wired-integration violations in the swarm's diff to date.

## (d) Alpha-bar items 1-3 and 7 confirmation

**Superseded by the Comprehensive consolidation section above** — this
section's original wave-1-era text is left below for provenance only, since
it's now materially stale (items 2/3 in particular have real, live-verified
progress the text below doesn't reflect).

- **Item 1** (installer without intervention, past SmartScreen): Not
  re-verified this session — no installer-affecting change landed in this
  swarm's scope. CI already builds unsigned MSI/NSIS per
  `publish-tester-release.yml`; still expected to hold, not re-confirmed.
- **Item 2** (create a character from any of CRB/B1/APG/ACG, load from disk):
  Materially advanced — Fighter/Wizard/Rogue all confirmed creatable/
  loadable for any race (not just Human), live-verified for Elf
  specifically. The other 8 classes remain fully blocked (zero chassis
  computation), not a stub-surface problem — a missing-engine one.
- **Item 3** (advance 6 levels, multiclass, spells/feats/equipment/bio/money):
  Materially advanced — Wizard's full bootstrap chain (class acquisition,
  first spell, slot-budget enforcement) and Rogue's UI reachability are both
  closed and live-verified. Money purchasing is now a real atomic
  transaction. Feat *selection* works (recorded, persisted); feat *effect*
  does not (see defect 6 in the consolidation table) — a real, newly-found
  gap against this item's spirit, even though nothing about feat selection
  itself is stubbed.
- **Item 7** (PR lands green on CI, four-check audit re-run, SWARM_REPORT.md
  recorded): Pending — this document is that artifact, still in draft; the
  four-check audit itself has not been run yet (see §c above), correctly
  held until the closure PR is genuinely being opened.

---

## Appendix: formula spec for durability / carry capacity / encumbrance / money conversion (for backend wave 2)

QA prep work for the four calculations flagged above as having zero production
implementation. Sourced from the real PCGen engine checkout at
`/home/ubuntu/workspace/repos/pcgen` (the same repo the swarm's PCGen parity
tooling already shells out to — `scripts/pcgen-run-character.sh`), not from
memory, wherever an authoritative source file exists. Confidence level is
called out per item; anything not directly sourced from a PCGen file should be
treated as "needs verification against a real PCGen run" before being
hardcoded into a parity test.

### Durability (character survivability)

Per the lead's ruling, scope is: max HP, current HP, temporary HP, nonlethal
damage tracking, dying/unconscious/death thresholds. Standard PF1 rules
(open game content, not PCGen-sourced — high confidence, but not yet
cross-checked against a PCGen run):

- **Max HP** = sum, per class level in level order, of that level's Hit Die
  contribution + Constitution modifier, with a floor of **1 HP per level**
  regardless of Con penalty. Level 1 uses the **maximum** value of the class's
  Hit Die (already implemented for Fighter: `FIGHTER_LEVEL_1_MAX_HIT_DIE_HIT_POINTS
  + constitution_modifier` in `pilot_compute.rs:7418`) — every level after
  that uses either a rolled or (more commonly, and what PCGen/most digital
  tools default to) an **average/fixed** value per the class's Hit Die
  (already computed per-level and tested in `sd13_*_level*_progression.rs` /
  `sd20_levelup_*.rs` — those tests cover the per-level *increment*; there is
  no test or field for the *running total*). In a multiclass build, each
  class level contributes using its own class's Hit Die.
  - Favored Class Bonus: a level where the player chose +1 HP (instead of a
    skill point) adds 1 more HP at that level — check whether
    `sd13_fighter_favored_class_bonus_choice.rs` threads this into an HP
    total anywhere, since today it looks like it's tracked but not summed.
- **Current HP**: starts equal to max HP; decremented by damage taken during
  play. This is a live-tracking field, not a build-time derived calculation —
  needs a data field with `default = max_hp`, not a "formula."
- **Temporary HP**: granted by specific spells/effects (e.g. *false life*),
  not derived from chassis math. Likely out of v0.6 scope unless a specific
  spell/item that grants it is already selectable; flag to backend to confirm
  scope before building a general temp-HP resource system.
- **Nonlethal damage**: tracked as a separate running total against current
  HP, not a subtraction from it.
- **Thresholds** (standard PF1/d20 SRD rule, high confidence):
  - `current_hp == 0` → **disabled** (can take a single move or standard
    action per round; a standard action causes 1 more point of nonlethal
    damage and leaves the character at 0, not negative).
  - `current_hp < 0` and `current_hp > -constitution_score` → **dying**
    (unconscious, loses 1 HP/round unless stabilized).
  - `current_hp <= -constitution_score` → **dead**.
  - `nonlethal_damage == current_hp` (current HP still `> 0`) → **staggered**.
  - `nonlethal_damage > current_hp` → **unconscious** (stable, not dying,
    since the excess is nonlethal).

  **Correction (QA, 2026-07-24):** this appendix originally wrote the
  staggered threshold as `>=`, which overlaps with unconscious below —
  imprecise pre-implementation spec text, not what shipped. The actual
  `durability.rs::classify_durability` (and this session's independent
  re-derivation of all 6 states against real PF1/d20 SRD rules) uses exact
  equality for staggered; corrected above rather than left to mislead a
  future reader.

### Carry capacity / encumbrance

**Sourced directly from PCGen's own Pathfinder game-mode data file** —
`/home/ubuntu/workspace/repos/pcgen/system/gameModes/Pathfinder/load.lst`.
This is the exact table PCGen itself uses, so a parity test built from these
numbers should match PCGen output by construction (still worth a spot-check
run). Engine logic (extrapolation beyond the table) lives in
`pcgen/core/system/LoadInfo.java` in that same checkout.

**Trap for anyone hand-writing fixture corpus text** (found while writing
`tests/v06_encumbrance.rs`): this crate's real corpus `KEY:` tokens only
carry a `(Base)` suffix for items with real magical/enhancement variants
(armor, shields, weapons — e.g. `Chain Shirt (Base)`, `Longsword (Base)`).
Plain General-category items do not (e.g. `Backpack`, not
`Backpack (Base)`). A fixture item whose `KEY:` guesses wrong on this
silently resolves to `unresolved_item_ids` rather than erroring — the
equipment resolver is strict/exact-match, not fuzzy, so a wrong suffix
looks like "this item weighs 0" rather than a loud failure. Check the real
entry in `src/rules_core/rules_tables/crb/equipment_data/*.rs` before
hand-transcribing a `KEY:` token into fixture text.

- **Base table** (`LOAD:<Strength>|<max load in lbs, at 1x "Heavy" multiplier>`),
  Strength 0-29:
  `0|0, 1|10, 2|20, 3|30, 4|40, 5|50, 6|60, 7|70, 8|80, 9|90, 10|100, 11|115,
  12|130, 13|150, 14|175, 15|200, 16|230, 17|260, 18|300, 19|350, 20|400,
  21|460, 22|520, 23|600, 24|700, 25|800, 26|920, 27|1040, 28|1200, 29|1400`.
- **Beyond Strength 29**: multiply the value at `(score - 10)` by `LOADMULT:4`
  — i.e. every +10 Strength beyond the table quadruples the Str-29 baseline
  chain (`LoadInfo.getLoadScoreValue`, the `loadScoreMultiplier` /
  `loadMultStep=10` fields).
- **Encumbrance tiers**, each expressed as a multiplier of the base table
  value plus a skill-check-penalty-style modifier
  (`ENCUMBRANCE:<name>|<multiplier>||<penalty>`):
  - Light: `1/3` of table value, penalty `0`.
  - Medium: `2/3` of table value, penalty `-3`.
  - Heavy: `1x` of table value (this is literally the table value itself —
    "heavy load" *is* the tabulated max), penalty `-6`.
  - OverHead (max lift over head): `1x`, penalty `-6`.
  - OffGround (max lift/budge off the ground): `2x`, penalty `-6`.
  - PushDrag (max push or drag): `5x`, penalty `-6`.
- **Size adjustment** (`SIZEMULT:<size code>|<multiplier>`, relative to
  Medium = 1x): Fine `0.125`, Diminutive `0.25`, Tiny `0.5`, Small `0.75`,
  Large `2`, Huge `4`, Gargantuan `8`, Colossal `16`. Effective Strength for
  the load table lookup is the character's actual Strength score — the size
  multiplier is applied to the resulting load value, not to the Strength
  score used for table lookup.

### Money conversion

- **Denomination ratios** (standard d20/PF1 currency, open content — **not**
  independently confirmed against a PCGen source file in this pass; I found
  no explicit conversion-table data file in the PCGen checkout, which is
  consistent with these being simple linear arithmetic rather than tabulated
  data, but flagging as not-yet-source-verified): 1 platinum piece (pp) = 10
  gold pieces (gp) = 100 silver pieces (sp) = 1000 copper pieces (cp).
  Equipment `cost_gp` fields already price everything in gp; conversion is
  just `value_in_gp * {pp: 0.1, gp: 1, sp: 10, cp: 100}` and back.
- **Starting wealth by class**: searched `data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst`
  for a `GOLD:` token (PCGen's per-class starting-gold-roll field, e.g.
  `GOLD:5d6`) and found none in that file. **Unresolved at the time** —
  either starting wealth lives in a different PCGen data file not yet
  checked, or PCGen leaves it as a manual/optional step; don't guess a value
  here. **Follow-up completed (backend, risks-and-open-questions.md item
  7):** the deeper search this note asked for was done — the whole data
  tree, PCGen's gameMode-level `miscinfo.lst`, and the wider corpus — and
  found nothing real anywhere, plus caught a real trap (a stub-labeled
  `starting_gold` column in an unrelated closure artifact that looks
  citable but explicitly isn't licensed data). This genuinely doesn't exist
  in any real, licensed corpus source available in this environment — a
  content-provenance/licensing question for the operator now, not an open
  engineering lookup.

---

## QA attestation

**Not yet signed.** This section is filled in only when the alpha bar in §1
of `release-swarm.md` genuinely holds, per §4.4's "Done" criteria. Until
then, this document is a living gap-tracker, not a sign-off.
