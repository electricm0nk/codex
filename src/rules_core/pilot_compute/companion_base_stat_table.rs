//! Generic companion base-ability-score table (SD-32 T12
//! `epic-10-reference-library-residual-reach` row 20, cycles 5-13).
//!
//! # Cycle 13 addendum: all 54 remaining untagged records grounded --
//! # the 196-record `RACETYPE:Companion` population is now FULLY closed
//!
//! Continuing cycle 12's own next-cycle order (largest book first), cycle
//! 13 re-derived the full residual population directly from `data/corpus/
//! <book>/companion/*.json` across all nine books cycle 12 named (`§17a`),
//! this time globbing every file in each book's `companion/` directory
//! (not just `companion_*.json`) after a first pass under the narrower
//! glob undercounted `advanced_race_guide` by 2 (`brute_steed_camel.json`/
//! `brute_steed_horse.json` -- both real `MONSTERCLASS:Companion:2`
//! records, filed without the usual `companion_` filename prefix). The
//! corrected re-derivation found exactly **54**, matching cycle 12's own
//! figure book-for-book once the two `brute_steed_*` records are counted
//! (`advanced_race_guide` 6, not 4): `beastiary` (14), `ultimate_magic`
//! (9), `advanced_race_guide` (6), `bestiary_6` (6), `core_rulebook` (6),
//! `monster_codex` (5), `bestiary_5` (4), `inner_sea_combat` (3),
//! `horror_adventures` (1).
//!
//! Source: the same `aonprd.com/DruidCompanions.aspx?ItemName=<species>`
//! per-species fetch cycles 6-12 used, for every entry with its own
//! printed "Starting Statistics" line (48 of 54). The remaining 6 have no
//! independent stat block and are derived instead, each from an already-
//! grounded table entry plus the source's own explicit textual rule,
//! cross-checked against the corpus's own `natural_armor` token exactly
//! like every other entry: `carnivorous_flower`/`crawling_vine`/
//! `puffball`/`sapling_treant` (Advanced Race Guide p.26, the Treesinger
//! elf druid archetype's plant companions -- an aonprd.com "Plant"
//! category page plus a d20pfsrd/Paizo-blog corroboration, since AoN's own
//! Plant Companions index only tracks Ultimate Wilderness's 8), `brute_
//! steed_camel`/`brute_steed_horse` (Advanced Race Guide p.56, the Fell
//! Rider hobgoblin cavalier archetype's own class feature text: "+2
//! Strength, -2 Dexterity ... otherwise works like the cavalier's mount
//! ability" -- the already-grounded `camel`/`horse` entries plus that
//! delta), and `devolved_humanoid` (Horror Adventures p.50, the
//! Devolutionist druid archetype's own nature-bond text: "Use the stats
//! for an ape animal companion" -- the already-grounded `ape` entry
//! reused directly). All 6 derivations are independently confirmed by the
//! corpus's own `natural_armor` token agreeing with the source entry they
//! derive from, not merely asserted.
//!
//! Natural-armor cross-check: **53 of 54 agree** with the corpus's own
//! token. The lone exception, `dolphin_orca` (corpus 6, two independent
//! sources -- aonprd.com and a corroborating search -- agree on 1): traced
//! to the PCGen oracle itself (`b1_races_companion.lst` line 16,
//! `BONUS:VAR|AC_Natural_Armor|6|TYPE=Base`), which shares its exact
//! natural-armor value with the UNRELATED standalone Huge Orca monster
//! race entry in the same file (`b1_races.lst`) -- strong evidence of a
//! copy-paste error in PCGen's own third-party data, not a corpus
//! ingestion defect. Per `decisions.md §1a` (grounded by measurement, not
//! by whichever source is more convenient) the printed total (1) is what
//! is stored; the corpus/oracle's disagreeing token is documented here
//! rather than silently overridden or silently trusted. No delta was
//! backed out anywhere else in this cycle's diff -- every other Str/Con
//! value stored is the source's own printed (or once-removed, for the 6
//! derived entries) total directly, matching cycle 9's corrected
//! methodology throughout.
//!
//! Table: **142 -> 196** (142 + 54). The entire 196-record `RACETYPE:
//! Companion` base-race population this table targets is now grounded --
//! **zero** untagged records remain in any of the nine books cycle 12's
//! own next-cycle plan named. Row 20's own acceptance criterion
//! (`decisions.md §27b`/`§1a`: close or precisely size every
//! `RACETYPE:Companion` corpus record) is met in full for this table; see
//! this cycle's own receipt for the card-level `complete` determination.
//!
//! # Cycle 12 addendum: all 38 `ultimate_wilderness` untagged records
//! # grounded -- the single largest remaining bucket (104 -> 142)
//!
//! Continuing cycle 11's own next-cycle order, cycle 12 re-derived the
//! `ultimate_wilderness` base-race `RACETYPE:Companion` population directly
//! from `data/corpus/ultimate_wilderness/companion/*.json` (`§17a`),
//! filtering to `monster_class` starting with `"Companion"` and excluding
//! `companion_advancement_*`/`companion_body_type_*` records (a different
//! record type, not a base-race entry): **52 total**, of which `gulper_
//! plant` (cycle 5) and `ornithomimosaur` (cycle 11) were already grounded,
//! leaving exactly **38** -- matching cycle 11's own next-cycle figure
//! exactly.
//!
//! Source: `aonprd.com/DruidCompanions.aspx?ItemName=All&Category=Animal`
//! -- a full alphabetical index of every animal-companion species this
//! site tracks across all books, discovered this cycle -- confirmed all 38
//! `ultimate_wilderness` slugs are listed, then fetched each species' own
//! `DruidCompanions.aspx?ItemName=<species>` page directly for its printed
//! "Starting Statistics" line (Ultimate Wilderness pp.178-185 per page,
//! confirmed in each response), the same aonprd.com domain and per-species
//! fetch method cycles 6-11 already used. Every one of the 38 fetched pages
//! independently confirmed its own source book and page number. Cross-
//! checked against the corpus's own `natural_armor` field for all 38:
//! **100% agreement**, the same rate every prior cycle found. No delta was
//! backed out anywhere -- every Str/Con value stored is the page's own
//! printed total directly, matching cycle 9's corrected methodology.
//!
//! Table: **104 -> 142** (104 + 38). `ultimate_wilderness` is now fully
//! closed (0 of 52 base-race records remain ungrounded). 54 of 196 remain,
//! all untagged: `beastiary` (14), `ultimate_magic` (9), `advanced_race_
//! guide` (6), `bestiary_6` (6), `core_rulebook` (6), `monster_codex` (5
//! residual), `bestiary_5` (4 residual), `inner_sea_combat` (3, including
//! `griffon`, still a live pinned refusal example), `horror_adventures` (1).
//!
//! # Cycle 11 addendum: both named dinosaur refusals resolved; 28 of the
//! # 120 untagged-outside-`core_rulebook` records grounded (74 -> 104)
//!
//! Continuing cycle 10's own next-cycle order, cycle 11 first resolved the
//! two dinosaur-bucket refusals cycle 7 named and no cycle since had
//! cleared -- `pachycephalosaurus` and `ornithomimosaur` -- via
//! `legacy.aonprd.com/bestiary/animalCompanions.html` (a single
//! consolidated page carrying many companions' own printed "Starting
//! Statistics" blocks directly) for the first, and a targeted
//! `aonprd.com/DruidCompanions.aspx?ItemName=Ornithomimosaur` fetch for
//! the second, both cross-checked against a second independent fetch/
//! search and against the corpus's own `natural_armor` token. `28 of 28`
//! `AnimalCompanionDinosaur` records are now grounded.
//!
//! It then re-derived the population directly from `data/corpus/*/
//! companion/*.json` fresh (`§17a`): still 196 total, 144 untagged,
//! unchanged from cycles 8-10. Of the 144 untagged, 24 were already
//! grounded (`wolf`, `horse`, cycle 10's 22 `core_rulebook` species),
//! leaving 120 -- the same figure cycle 10's own receipt named. This cycle
//! added 28 of those 120, spanning `bestiary_3` (all 12), `bestiary_4`
//! (all 7), `bestiary_5` (8 of 12), and `monster_codex` (1, `Giant
//! Vulture`'s own `bestiary_3` reprint) -- the same two-source-plus-
//! corpus-tiebreaker method throughout, with `legacy.aonprd.com`'s own
//! consolidated page as source 1 for every entry (natural armor agreed
//! with the corpus's own token for all 28, a 100% agreement rate matching
//! every prior cycle).
//!
//! **74 (cycle 10's exit) + 2 (refusals resolved) + 28 (new untagged) =
//! 104.** 92 remain ungrounded, all untagged: `ultimate_wilderness` (38,
//! the single largest remaining bucket), `beastiary` (14), `ultimate_magic`
//! (9), `advanced_race_guide` (6), `bestiary_6` (6), `core_rulebook` (6 --
//! `Snake, Viper` and `Gar` among them), `monster_codex` (5 residual:
//! `cave_salamander`, `gorthek`, `python_riding`, `rat_riding`, `yzobu`),
//! `bestiary_5` (4 residual: `frog_father`, `frog_goliath`, `polar_bear`,
//! `polar_bear_dire`), `inner_sea_combat` (3), and `horror_adventures`
//! (1). `bestiary_3` and `bestiary_4` are now fully closed. Next cycle's
//! own concrete first step: `ultimate_wilderness`, at 38.
//!
//! # Cycle 9 addendum: a correctness defect in all 44 nonzero-delta
//! # entries cycles 4-8 added, found and fixed before grinding further
//!
//! Before adding any of the 142 untagged species this cycle's brief named
//! as its own scope, re-deriving each figure handed down (`§17a`) surfaced
//! that [`companion_base_stat_table`]'s own `strength`/`constitution`
//! fields for every one of the 44 species with a nonzero corpus
//! `BONUS:STAT` delta were **silently wrong** -- understated by exactly
//! that delta, in every one of the 23 aquatic/plant/primate species cycle
//! 8 added, all but six of the 26 dinosaurs cycles 6-7 added, and
//! `gulper_plant` itself (cycle 5).
//!
//! **The defect, and how it was found.** [`ground_companion_stat_block`]
//! (below) computes `strength_score = stats.strength + strength_bonus`,
//! where `strength_bonus` comes from `super::animal_companion_stat_bonus`
//! -- a UNIVERSAL, species-agnostic formula (`floor(MasterLevel/3)`, the
//! companion CLASS's own `BONUS:STAT|STR,DEX|floor(MasterLevel/3)`,
//! `core_rulebook/cr_abilities_companion.lst:60`), applied identically to
//! every species. It never reads the per-species corpus RACE record's own
//! `BONUS:STAT` delta at all. `stats.strength`/`stats.constitution`
//! therefore must hold each species' PRINTED 1st-level "Starting
//! Statistics" total directly -- exactly what `WOLF_COMPANION_STRENGTH_
//! SCORE`/`HORSE_COMPANION_STRENGTH_SCORE` (this module's parent, the
//! session's own original two hand-verified species) already do: Wolf's
//! constant is `13`, the printed AoN/d20pfsrd/corpus-page-citation total,
//! confirmed directly against `ground_wolf_companion_stat_block`'s own doc
//! comment ("Base ability scores ... Str `{WOLF_COMPANION_STRENGTH_
//! SCORE}`") and its own inline comment ("The companion class's own
//! level-scaling Strength bonus stacks on **the race's base score**").
//!
//! Cycles 5-8 instead treated the corpus's own per-species `BONUS:STAT`
//! delta as something to subtract OUT of the printed total before storing
//! it -- e.g. Gulper Plant's own doc comment (superseded, corrected
//! below): "AoN: ... Str 12 ... both agree on a base of Str 10 ... once
//! the delta is backed out". That delta is PCGen's own internal
//! delta-from-template bookkeeping (how the RACE file reconstructs the
//! printed total from PCGen's own default ability array), unrelated to
//! and never read by this engine's own companion-advancement math. Re-
//! fetching Gulper Plant's own AoN page directly this cycle
//! (`aonprd.com/DruidCompanions.aspx?ItemName=Gulper%20Plant`) confirmed
//! the printed total is Str 12 / Con 13, not the table's stored Str 10 /
//! Con 11 -- and the same mismatch was then confirmed, entry by entry,
//! against every other species' own doc comment (which already recorded
//! the correct printed AoN total in its own "AoN: Str X ... Con Y" text
//! -- only the struct literal itself, and the six-species-later derived
//! test assertions mirroring it, were wrong).
//!
//! **The fix.** All 44 affected struct literals (every species below with
//! a nonzero `STR`/`CON` corpus delta; the six dinosaurs with a genuine
//! zero delta on both -- `triceratops`, `stegosaurus`, `diplodocus`,
//! `styracosaurus`, `kentrosaurus`, `tylosaurus` -- were never wrong, by
//! construction) now hold the printed AoN total each entry's own doc
//! comment already recorded, matching Wolf/Horse's own precedent exactly.
//! Natural armor was NEVER affected: the corpus's own `AC_Natural_Armor`
//! token is the base value directly (not a delta), confirmed against
//! AoN's own printed "+n natural armor" line for every entry across
//! cycles 6-8, and stays exactly as grounded. The per-entry "Base Str
//! X-Y=Z" arithmetic sentences inline below are now superseded prose from
//! the pre-fix derivation, left in place rather than hand-edited line by
//! line at this scale -- the struct literals and the tests below them are
//! the operative, now-corrected source of truth; this addendum is the
//! authoritative record of the correction. Every test asserting an exact
//! Str/Con value was updated to match (`the_nine_dinosaur_companions_...`,
//! `the_seventeen_cycle_seven_dinosaur_companions_...`,
//! `the_twenty_three_cycle_eight_aquatic_plant_and_primate_companions_
//! ...`, and both `gulper_plant_...` tests, whose base-attack-bonus and
//! hit-points expectations shift with the corrected Str/Con modifiers).
//!
//! This is a correctness fix to 44 already-"closed" species, not new
//! population coverage -- the 52/196 grounded count and the 144-species
//! residual this cycle's brief named are unchanged by it. It took
//! priority over grinding the 142 untagged species this cycle because
//! shipping 20-30 more entries with the same wrong subtraction would have
//! compounded a real, character-creation-reachable defect at scale
//! (`decisions.md §1a`: a gate that cannot fail is worse than none; an
//! uncaught systematic error is the same failure by another name).
//!
//! # Cycle 8 addendum: the three remaining tagged buckets, and a population
//! # correction (`§17a`)
//!
//! Cycle 8 re-derived the true base-race `RACETYPE:Companion` population
//! directly from this repo's own ingested `data/corpus/*/companion/*.json`
//! (not trusted from a prior cycle's brief): filtering to records that
//! actually carry a `monster_class` starting with `"Companion"` (the real
//! per-record signal a companion mechanic uses -- some records, e.g.
//! Inner Sea Combat's own Hippocampus, carry `RACETYPE:Magical Beast`
//! rather than `RACETYPE:Companion` in the ingested JSON despite being a
//! genuine companion race) and excluding the separate "Companion
//! Advancement (...)" ability records (`monster_class: None`, a different
//! record type entirely, not a base-race entry) finds **196**, not the
//! 213 a prior cycle's raw-oracle-`.lst`-line count assumed. Of those 196:
//! 144 carry no `RACESUBTYPE:` tag at all, 28 are `AnimalCompanionDinosaur`
//! (cycles 6-7's own bucket), 12 are `Aquatic` (not 13 -- the prior
//! figure double-counted `Familiar`-racetype "Tiny Named Animal" records
//! that merely share the `Aquatic` `RACESUBTYPE:` tag with genuine
//! `MONSTERCLASS:Companion:2` records, e.g. `ultimate_wilderness/
//! companion/lamprey.json`'s own `MONSTERCLASS:Animal:1`/`RACETYPE:
//! Animal`), 8 are `PlantCompanion` (7 not already grounded by
//! `gulper_plant`), and 4 are `AnimalCompanionPrimate` -- summing to
//! 196 exactly. **`ConstructCompanion` does not exist in this corpus at
//! all**: the 3 raw `RACESUBTYPE:ConstructCompanion` records a prior
//! cycle's brief cited live in `data/pathfinder/ascension_games/
//! path_of_iron/poi_races_companion.lst` on the pinned oracle -- a
//! third-party (Ascension Games) supplement this repo's `data/corpus/`
//! has never ingested at all (`ls data/corpus/ | grep -i iron` finds
//! nothing) -- so that bucket is out of the 196-record population this
//! table targets, not merely unverified within it; ingesting a wholly new
//! book is a separate, much larger undertaking than this row's own
//! per-species table-filling scope.
//!
//! Cycle 8 grounds all three remaining tagged buckets in full: 12
//! `Aquatic`, all 7 remaining `PlantCompanion` (including `hunting_
//! cactus`, whose base scores this module's own cycle 4/5 correction
//! already externally verified as a worked example but never added to
//! the table), and all 4 `AnimalCompanionPrimate` -- 23 new entries, same
//! two-independent-source-plus-corpus-tiebreaker method cycles 6-7 set
//! (aonprd.com's own "Starting Statistics" block, cross-checked against
//! d20pfsrd for Octopus as an exact-match spot-check, plus the corpus's
//! own `BONUS:STAT` delta as the numeric tiebreaker). Natural armor,
//! read directly from the corpus's own `AC_Natural_Armor` token, matched
//! AoN's own printed "+n natural armor" line for all 23 -- the same 100%
//! agreement cycles 6-7 found, now confirmed across 46 dinosaur-plus-
//! aquatic-plus-plant-plus-primate species combined.
//!
//! **Before grinding the table further, cycle 8 re-asked cycle 6's own
//! `§17` question against PCGen's own Java** (not just the raw `.lst`
//! source cycle 6 already checked): is any part of the base ability-score
//! vector *computed* rather than hand-authored, now that a real character-
//! creation consumer exists to test against? `git grep -il
//! "AnimalCompanion" -- '*.java'` against the pinned oracle's git objects
//! (`git -C $PCGEN_REPO_DIR ls-tree -r --name-only HEAD`) finds no
//! ability-score-computing class at all -- every `*Companion*.java` file
//! (`CompanionModFacet`, `CompanionMod`, `CompanionListLst`,
//! `CompanionmodToken`, and their siblings) handles the companion-MOD
//! **linking** mechanic (which class's companion follows which master),
//! never ability scores. Re-reading `cr_classes_companion.lst` (the
//! `CLASS:Companion` definition itself, the shared progression math every
//! species reads) directly from the oracle's git objects confirms cycle
//! 4's own finding by an independent method: the class definition carries
//! `BONUS:COMBAT`/`BONUS:SAVE`/`BONUS:VAR` tokens for attack, saves, and
//! the `AnimalCompanionSkill`/`BaseClassSkillPts` derived values, but **no
//! ability-score token of any kind** -- confirming, a third independent
//! way now (raw `.lst` grep, ingested-JSON delta backing-out, and the
//! class definition's own token list), that the base ability-score block
//! is genuinely fixed, per-species, printed prose with no PCGen-side
//! derivation shortcut. It must be hand-authored; this cycle did so
//! honestly for the 23 species it had room to verify.
//!
//! # Cycle 7 addendum: the dispatch point cycle 6 named now exists
//!
//! Cycle 6 named, not hidden, that [`ground_companion_stat_block`] had zero
//! live callers: no class offered a character-creation-time CHOICE among
//! companion species, so a verified row here could never reach a real
//! character. Cycle 7 closes that gap. `super::ground_selected_companion_
//! or_default` (this module's parent, `mod.rs`) is the new dispatch point,
//! called from all three of this engine's companion-bearing class sites
//! (Druid's, Hunter's, and Cavalier's own animal-companion/mount grounding
//! functions), reading `super::COMPANION_SPECIES_CHOICE_ID` (`"choice:
//! companion_species"`) off the real `CharacterInput.chosen.selected_
//! choices` the same generic mechanism `choice:druid_nature_bond`/
//! `choice:cavalier_order` already use. `apps/desktop/src-tauri`'s
//! `CreateCharacterRequest` carries a new `companion_species: Option<
//! String>` field (`character_hub.rs`), threaded into that same choice by
//! `pf1_adapter.rs`'s `compose_character_input` -- the real character-
//! creation request path, not a test-only shortcut. An omitted field, or a
//! species this table has no verified row for, falls back to the class's
//! own prior fixed default (Wolf for Druid/Hunter, Horse for Cavalier)
//! rather than fabricating or blocking -- so this closes the wiring gap
//! with zero regression risk to any of the 61 classes' existing `Computed`
//! status. Proven at the real character-creation altitude (not an isolated
//! unit test) by `character_hub.rs`'s own
//! `a_druid_who_selects_gulper_plant_grounds_gulper_plant_not_wolf_at_
//! character_creation_altitude`.
//!
//! # Cycle 6 addendum: the dinosaur batch, and the wiring gap it surfaced
//!
//! Cycle 6 re-derived `§17`'s own question ("can the vectors be derived
//! from corpus data alone -- `BONUS:STAT` deltas plus a per-RACETYPE/size
//! baseline -- rather than typed in?") against the raw pinned PCGen oracle
//! directly, not just this repo's own ingested JSON: the "Companion (Gulper
//! Plant)" RACE line in `ultimate_wilderness/uw_races_companion.lst` itself
//! carries no absolute `STR:`/`DEX:`/etc token at all, only the `BONUS:STAT`
//! deltas already read below -- confirming cycle 5's own finding by an
//! independent method (the raw source line, not just the ingested-JSON
//! shape) rather than merely trusting it. The base is genuinely
//! Java/table-computed upstream and absent from every corpus form this
//! engine reads; no derivation shortcut exists. Cycle 6 therefore continued
//! hand-authoring, per this module's own §4 below, adding the nine
//! `AnimalCompanionDinosaur` species named in [`companion_base_stat_table`]'s
//! own doc comments (`allosaurus`, `ankylosaurus`, `pteranodon`,
//! `deinonychus`, `velociraptor`, `triceratops`, `tyrannosaurus`,
//! `amargasaurus`, `brachiosaurus`), each verified against AoN's own
//! "Starting Statistics" block plus the corpus's own `BONUS:STAT` delta as
//! the numeric tiebreaker, the same method Gulper Plant set.
//!
//! **Cycle 6's own finding here (`ground_companion_stat_block` had no live
//! caller anywhere in this crate, confirmed by `cargo build`'s dead-code
//! warning) is CLOSED as of cycle 7** -- see the cycle 7 addendum above.
//! Every existing companion-bearing class (Druid/Hunter's Wolf, Cavalier's
//! Horse) now dispatches through `ground_selected_companion_or_default`
//! first, falling back to the prior FIXED, single-species hand-authored
//! function (`ground_wolf_companion_stat_block`/`ground_horse_companion_
//! stat_block`) only when no real player selection is present.
//!
//! # What this module answers, and why it exists
//!
//! Cycles 3 and 4 asked and settled a question the wolf/horse hand-authored
//! constants in this module's parent (`ground_wolf_companion_stat_block`,
//! `ground_horse_companion_stat_block`) left open: where does a companion's
//! BASE (pre-advancement) ability-score block come from, for every species
//! this engine does not already hand-ground? Cycle 4 read the pinned oracle
//! directly and confirmed the block is **not `.lst` data anywhere in
//! PCGen's own source** -- every companion race record carries only
//! `BONUS:STAT` DELTAS (e.g. Wolf's own `STR|2 DEX|4 CON|4 INT|-8 WIS|2
//! CHA|-4`, `core_rulebook/cr_races_companion.lst:32`), never an absolute
//! score. The base score is fixed, per-species, printed prose the engine
//! must hand-author -- exactly the precedent `WOLF_COMPANION_STRENGTH_
//! SCORE`/`HORSE_COMPANION_STRENGTH_SCORE` (this module's parent, above)
//! already set, verified against two independent primary sources
//! (aonprd.com and d20pfsrd) plus the corpus as a tiebreaker.
//!
//! # This cycle's own correction to cycle 4's sizing (`decisions.md §17a`)
//!
//! Cycle 4 sized the follow-on as "a category table... a handful of
//! `RACESUBTYPE:` rows," inferring from the `RACESUBTYPE:PlantCompanion`-
//! style tags shared by several Ultimate Wilderness companions that a
//! single row could serve every member of one category. Re-derived, not
//! trusted: `grep -rh "RACETYPE:Companion" $PCGEN_REPO_DIR/data | grep -oE
//! "RACESUBTYPE:[A-Za-z]+" | sort | uniq -c` finds only 59 of the corpus's
//! 213 total `RACETYPE:Companion` records carry a `RACESUBTYPE:` tag at
//! all (31 `AnimalCompanionDinosaur`, 13 `Aquatic`, 8 `PlantCompanion`, 4
//! `AnimalCompanionPrimate`, 3 `ConstructCompanion`); the other 154 carry
//! none. Cross-checking two SAME-category Ultimate Wilderness members
//! (both `RACESUBTYPE:PlantCompanion`) against their own published base
//! scores (verified via aonprd.com's Druid Companions pages) REFUTES the
//! shared-category-base hypothesis directly: Gulper Plant's own delta
//! (`STR|2 CON|2 INT|-10 CHA|-8`, no DEX/WIS delta) against its printed
//! Str 12/Dex 11/Con 13/Int 1/Wis 10/Cha 3 backs out a base of STR 10 / DEX
//! 11 / CON 11 / INT 11 / WIS 10 / CHA 11; Hunting Cactus's own delta
//! (`STR|4 DEX|2 CON|6 INT|-8 WIS|2 CHA|-4`) against its printed Str
//! 14/Dex 13/Con 17/Int 2/Wis 13/Cha 6 backs out STR 10 / DEX 11 / CON 11 /
//! INT 10 / WIS 11 / CHA 10 -- the SAME category, two DIFFERENT base
//! vectors. **This is genuinely per-species data, not a per-category
//! table**, and the real population needing it is closer to 213 (or, if
//! scoped to only the "unusual" bucket cycle 4's own finding named, 59)
//! than "a handful."
//!
//! # What this cycle builds, and what it names rather than fabricates
//!
//! Building all 213 (or 59) entries to the same two-independent-source
//! verification bar `WOLF_COMPANION_STRENGTH_SCORE`'s own precedent sets
//! is real, sized, per-species sourcing work this one cycle does not have
//! the room to complete without lowering that bar -- and shipping an
//! under-verified number here is a worse outcome than shipping none: a
//! silently-wrong ability score corrupts a real character's combat math,
//! exactly the failure `decisions.md §1a` and this codebase's own
//! anti-fabrication test suite (`class_feature_grant_consumer.rs`'s
//! thirteen-test gate) exist to refuse. This module therefore:
//!
//! 1. Builds the GENERIC mechanism -- [`CompanionBaseStats`], the lookup
//!    table, and [`ground_companion_stat_block`] -- generalizing
//!    `ground_wolf_companion_stat_block`/`ground_horse_companion_stat_
//!    block`'s own proven math (same universal `MONSTERCLASS:Companion:2`
//!    Hit Dice progression -- confirmed present verbatim on every one of
//!    the 213 records, including Gulper Plant's own -- and the same
//!    universal `floor(MasterLevel/3)` Strength/Dexterity and
//!    `2*floor(MasterLevel/3)` natural-armor advancement, both from the
//!    SHARED `cr_classes_companion.lst`/`cr_abilities_companion.lst` files
//!    every companion species reads, species-specific "Companion
//!    Advancement ~ <Species>" abilities aside -- unmodeled here exactly
//!    as Wolf's own equivalent ability is unmodeled by its existing
//!    grounding function, so this is not a new scope gap, the same one).
//! 2. Populates it with Wolf and Horse, RE-DERIVED (not copied) from this
//!    module's parent's own already-verified constants, as the proof the
//!    generic function reproduces the existing, shipped, tested output
//!    byte-for-byte (`generic_wolf_reproduces_the_existing_hand_authored_
//!    wolf_function` below).
//! 3. Adds ONE new, externally re-verified species -- Gulper Plant
//!    (`RACESUBTYPE:PlantCompanion`, Ultimate Wilderness p.183) -- as
//!    concrete, non-hypothetical proof this generalizes past the two
//!    species it was built to reproduce, verified against aonprd.com's own
//!    Druid Companions page (Str 12, Dex 11, Con 13, Int 1, Wis 10, Cha 3;
//!    natural armor +1, `BONUS:VAR|AC_Natural_Armor|1|TYPE=Base`,
//!    `uw_races_companion.lst`, corpus-confirmed) with the corpus's own
//!    `BONUS:STAT` deltas as the tiebreaker check (both agree).
//! 4. Names the exact residual precisely rather than rounding it away.
//!
//! # Cycle 7 addendum: the dinosaur bucket closes (26 of 28), two named
//!
//! Continuing cycle 6's own next-cycle order, cycle 7 hand-authored the
//! remaining 17 of the 19 outstanding `AnimalCompanionDinosaur` records --
//! `elasmosaurus`, `stegosaurus`, `dimetrodon`, `iguanodon`, `spinosaurus`,
//! `dimorphodon`, `diplodocus`, `styracosaurus`, `ceratosaurus`,
//! `plesiosaurus`, `therizinosaurus`, `troodon`, `giganotosaurus`,
//! `kentrosaurus`, `quetzalcoatlus`, `parasaurolophus`, `tylosaurus` -- each
//! verified against a real "Starting Statistics" source (aonprd.com and/or
//! d20pfsrd, cross-checked by a second independent search query per
//! species) plus the corpus's own `BONUS:STAT` delta as the numeric
//! tiebreaker, reusing cycle 6's own "natural armor is direct, only Str/Con
//! need external verification" simplification (confirmed to hold again for
//! all 17: every `BONUS:VAR|AC_Natural_Armor|n|TYPE=Base` token matched its
//! source's printed "+n natural armor" line exactly, including Troodon's
//! own printed "no natural armor bonus at the starting level," which
//! matches the corpus record's own null `AC_Natural_Armor` token -- the
//! first species this table grounds with a genuine `natural_armor: 0`, not
//! an absent row). **Two of the 19 are named, not silently skipped**: for
//! `pachycephalosaurus` (Bestiary 3) no source this cycle could reach
//! separated its animal-companion "Starting Statistics" block from its
//! full-grown monster stat block (every search and fetch returned only the
//! CR-4 monster's own Str 22/Con 17, which is not the companion's base);
//! for `ornithomimosaur` (Ultimate Wilderness) the one source found gave a
//! number ambiguous between the companion's own base stats and the shared
//! "Companion Body Type ~ Avian" template baseline several Ultimate
//! Wilderness companions read from, which this cycle could not resolve
//! against a second independent source. Both refuse (return `false`,
//! ground nothing) rather than risk a silently-wrong score, per `§1a`.
//!
//! Combined with cycle 6's own nine, this closes 26 of 28 ingested
//! `AnimalCompanionDinosaur` records -- the two named above are the entire
//! remaining residual in this bucket. As of cycle 7, `companion_base_stat_
//! table`'s 29 entries (Wolf, Horse, Gulper Plant, the 26 dinosaurs) leave
//! 184 of 213 total `RACETYPE:Companion` corpus records without a
//! base-ability-score entry, and [`ground_companion_stat_block`] correctly
//! REFUSES for every one of them -- refuse rather than guess, the same
//! posture `class_feature_grant_consumer.rs`'s own module doc names
//! throughout. The next cycle's own concrete first steps: (a)
//! `pachycephalosaurus`/`ornithomimosaur`, if a second independent source
//! resolves either; (b) the `Aquatic` (13), `PlantCompanion` (7 remaining),
//! `AnimalCompanionPrimate` (4), and `ConstructCompanion` (3) tagged
//! buckets, largest first, repeating this cycle's own verification method.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use super::ComputationExplanation;

/// One companion species' verified base (pre-advancement) statistics --
/// the exact set `WOLF_COMPANION_STRENGTH_SCORE`/`HORSE_COMPANION_
/// STRENGTH_SCORE` and their siblings already hand-author per species in
/// this module's parent, gathered into one row so [`ground_companion_
/// stat_block`] can be table-driven rather than one hand-typed function
/// per species.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CompanionBaseStats {
    pub(crate) strength: i16,
    pub(crate) constitution: i16,
    pub(crate) natural_armor: i16,
    pub(crate) hit_die_size: u8,
}

/// The verified companion base-stat table, keyed by the same lower-case,
/// underscore-joined species slug [`super::pu_feature_slug`]-style
/// convention this module's sibling `class_feature_grant_consumer.rs`
/// already uses for its own id suffixes -- `"wolf"`, `"horse"`,
/// `"gulper_plant"`.
///
/// Hit Die size is 8 for every entry, not per-species: the PF1 Core
/// Rulebook's own "Animal Companion Base Statistics" table fixes the
/// companion's Hit Die at d8 regardless of the companion's own creature
/// type (confirmed by both existing entries already sharing it despite
/// being two different real creature types, Animal in both cases, and
/// unchanged for Gulper Plant's own Plant type below -- the companion
/// mechanic overrides the normal per-type Hit Die a standalone monster of
/// the same species would otherwise use).
fn companion_base_stat_table() -> &'static BTreeMap<&'static str, CompanionBaseStats> {
    static TABLE: OnceLock<BTreeMap<&'static str, CompanionBaseStats>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut out = BTreeMap::new();
        out.insert(
            "wolf",
            CompanionBaseStats {
                strength: super::WOLF_COMPANION_STRENGTH_SCORE,
                constitution: super::WOLF_COMPANION_CONSTITUTION_SCORE,
                natural_armor: super::WOLF_COMPANION_NATURAL_ARMOR,
                hit_die_size: super::WOLF_COMPANION_HIT_DIE_SIZE,
            },
        );
        out.insert(
            "horse",
            CompanionBaseStats {
                strength: super::HORSE_COMPANION_STRENGTH_SCORE,
                constitution: super::HORSE_COMPANION_CONSTITUTION_SCORE,
                natural_armor: super::HORSE_COMPANION_NATURAL_ARMOR,
                hit_die_size: super::HORSE_COMPANION_HIT_DIE_SIZE,
            },
        );
        // Ultimate Wilderness p.183, RACESUBTYPE:PlantCompanion. AoN's own
        // "Starting Statistics" (Druid Companions page, Gulper Plant): Str
        // 12, Con 13, +1 natural armor. Cycle 9 correction (`§17a`): this
        // is the value grounded here DIRECTLY, matching Wolf/Horse's own
        // established precedent (`WOLF_COMPANION_STRENGTH_SCORE`/
        // `HORSE_COMPANION_STRENGTH_SCORE`, this module's parent) -- the
        // corpus's own `BONUS:STAT|STR|2 BONUS:STAT|CON|2` per-species
        // deltas (`uw_races_companion.lst`) are PCGen's own internal
        // delta-from-template mechanic and are NOT subtracted from the
        // printed total: `ground_companion_stat_block` (below) adds only
        // the companion CLASS's own universal `animal_companion_stat_
        // bonus`/`animal_companion_natural_armor_bonus` level advance on
        // top of this field, never a species-specific one. Cycles 5-8
        // backed the per-species delta out anyway (Str 10/Con 11 here,
        // and the same error in all 44 other nonzero-delta entries below)
        // -- this cycle's own addendum documents the discovery and the
        // fix. `BONUS:VAR|AC_Natural_Armor|1|TYPE=Base` is unaffected: the
        // corpus's own natural-armor token was always the base value
        // directly, never a delta, so every prior natural-armor figure in
        // this table stays correct. Dex/Int/Wis/Cha are not grounded by
        // this module (this table's own consumer, like Wolf's, grounds
        // only the fields with a live downstream reader -- attack bonus,
        // saves, AC, HP).
        out.insert(
            "gulper_plant",
            CompanionBaseStats { strength: 12, constitution: 13, natural_armor: 1, hit_die_size: 8 },
        );
        // Row 20 cycle 6: the `AnimalCompanionDinosaur` bucket -- the largest
        // untagged `RACESUBTYPE:` category cycle 5's own next-cycle plan
        // named (31 of 213 by the oracle's own raw-`.lst` count; 28 in this
        // repo's own ingested `data/corpus/*/companion/*.json`). Each entry
        // below is verified the same two-independent-source-plus-corpus-
        // tiebreaker way `gulper_plant` was: AoN's own "Starting Statistics"
        // block (`aonprd.com/DruidCompanions.aspx?ItemName=...`), a SECOND
        // independent fetch/search confirming the same printed total where
        // one was available, and the corpus's own `BONUS:STAT` deltas (this
        // module's parent's own `data/corpus/<book>/companion/companion_
        // dinosaur_<species>.json`) as the numeric tiebreaker -- printed
        // total minus the corpus's own delta backs out the base, and the
        // corpus's own `BONUS:VAR|AC_Natural_Armor|<n>|TYPE=Base` token is
        // the base natural armor DIRECTLY (not a delta to back out), cross-
        // checked against AoN's own printed "+n natural armor" line, which
        // agreed for all nine species below. Dex/Int/Wis/Cha are not
        // grounded here either, matching gulper_plant's own scope (this
        // table's consumer grounds only the fields with a live downstream
        // reader).
        out.insert(
            // AoN: Str 14, Dex 16, Con 10, Int 2, Wis 15, Cha 10, +4 natural
            // armor. Corpus delta (`core_rulebook/companion_dinosaur_
            // allosaurus.json`): STR|4 DEX|6 INT|-8 WIS|4 (no CON delta),
            // AC_Natural_Armor|4|TYPE=Base. Base Str 14-4=10, Con 10-0=10.
            "allosaurus",
            CompanionBaseStats { strength: 14, constitution: 10, natural_armor: 4, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 10, Dex 14, Con 9, Int 2, Wis 12, Cha 8, +9 natural
            // armor. Corpus delta (`beastiary/companion_dinosaur_
            // ankylosaurus.json`): CON,CHA|-2 DEX|4 INT|-8 WIS|2 (no STR
            // delta), AC_Natural_Armor|9|TYPE=Base. Base Str 10-0=10,
            // Con 9-(-2)=11.
            "ankylosaurus",
            CompanionBaseStats { strength: 10, constitution: 9, natural_armor: 9, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 8, Dex 21, Con 10, Int 2, Wis 14, Cha 12, +0 natural
            // armor. Corpus delta (`beastiary/companion_dinosaur_
            // pteranodon.json`): CHA|2 DEX|10 INT|-8 STR|-2 WIS|4 (no CON
            // delta), no `AC_Natural_Armor` token (base 0, matches AoN's
            // "+0"). Base Str 8-(-2)=10, Con 10-0=10.
            "pteranodon",
            CompanionBaseStats { strength: 8, constitution: 10, natural_armor: 0, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Size Small, Str 11, Dex 17, Con 17, Int 2, Wis 12, Cha
            // 14, +1 natural armor. Corpus delta (`core_rulebook/companion_
            // dinosaur_deinonychus.json`): CHA|4 CON|6 DEX|6 INT|-8 WIS|2
            // (no STR delta), AC_Natural_Armor|1|TYPE=Base. Base Str
            // 11-0=11, Con 17-6=11.
            "deinonychus",
            CompanionBaseStats { strength: 11, constitution: 17, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN: identical printed block to Deinonychus (Str 11, Dex 17,
            // Con 17, Int 2, Wis 12, Cha 14, +1 natural armor) -- confirmed
            // by a second, independent AoN fetch, not assumed from the
            // shared name. Corpus delta (`core_rulebook/companion_
            // dinosaur_velociraptor.json`) is byte-identical to
            // Deinonychus's own (CHA|4 CON|6 DEX|6 INT|-8 WIS|2,
            // AC_Natural_Armor|1|TYPE=Base) -- same base, Str 11, Con 11.
            "velociraptor",
            CompanionBaseStats { strength: 11, constitution: 17, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 10, Dex 13, Con 11, Int 2, Wis 12, Cha 7, +6 natural
            // armor. Corpus delta (`beastiary/companion_dinosaur_
            // triceratops.json`): CHA|-4 DEX|2 INT|-8 WIS|2 (no STR/CON
            // delta), AC_Natural_Armor|6|TYPE=Base. Base Str 10-0=10, Con
            // 11-0=11.
            "triceratops",
            CompanionBaseStats { strength: 10, constitution: 11, natural_armor: 6, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 14, Dex 16, Con 10, Int 2, Wis 15, Cha 10, +4 natural
            // armor -- same printed block as Allosaurus (both share the PF1
            // "large theropod" starting statistics). Corpus delta
            // (`beastiary/companion_dinosaur_tyrannosaurus.json`): DEX|6
            // INT|-8 STR|4 WIS|4 (no CON delta), AC_Natural_Armor|4|
            // TYPE=Base. Base Str 14-4=10, Con 10-0=10.
            "tyrannosaurus",
            CompanionBaseStats { strength: 14, constitution: 10, natural_armor: 4, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 11, Dex 18, Con 9, Int 2, Wis 13, Cha 10, +3 natural
            // armor (confirmed by two independent fetches). Corpus delta
            // (`bestiary_6/companion_amargasaurus.json`): CON|-2 DEX|8
            // INT|-8 WIS|2 (no STR delta), AC_Natural_Armor|3|TYPE=Base.
            // Base Str 11-0=11, Con 9-(-2)=11.
            "amargasaurus",
            CompanionBaseStats { strength: 11, constitution: 9, natural_armor: 3, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 13, Dex 14, Con 11, Int 2, Wis 13, Cha 10, +3 natural
            // armor (confirmed by two independent fetches). Corpus delta
            // (`beastiary/companion_dinosaur_brachiosaurus.json`): DEX|4
            // INT|-8 STR|2 WIS|2 (no CON delta), AC_Natural_Armor|3|
            // TYPE=Base. Base Str 13-2=11, Con 11-0=11.
            "brachiosaurus",
            CompanionBaseStats { strength: 13, constitution: 11, natural_armor: 3, hit_die_size: 8 },
        );
        // Row 20 cycle 7: the remaining 17 of the 19 `AnimalCompanionDinosaur`
        // records cycle 6 did not reach (`pachycephalosaurus` and
        // `ornithomimosaur` are the two named residuals -- see this module's
        // own cycle 7 doc addendum above for why each refuses). Same
        // verification method: a real "Starting Statistics" source
        // (aonprd.com and/or d20pfsrd, cross-checked by a second independent
        // search per species) plus the corpus's own `BONUS:STAT` delta as
        // the numeric tiebreaker; natural armor read directly from the
        // corpus's own `AC_Natural_Armor` token (cycle 6's own
        // simplification, reconfirmed for all 17 below).
        out.insert(
            // AoN/d20pfsrd: Str 10, Dex 18, Con 12, Int 2, Wis 13, Cha 9, +2
            // natural armor. Corpus delta (`beastiary/companion_dinosaur_
            // elasmosaurus.json`): CHA|-2 CON|2 DEX|8 INT|-8 WIS|2 (no STR
            // delta), AC_Natural_Armor|2|TYPE=Base. Base Str 10-0=10, Con
            // 12-2=10.
            "elasmosaurus",
            CompanionBaseStats { strength: 10, constitution: 12, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN/d20pfsrd: Str 10, Dex 18, Con 10, Int 2, Wis 12, Cha 10,
            // +6 natural armor. Corpus delta (`beastiary/companion_
            // dinosaur_stegosaurus.json`): DEX|8 INT|-8 WIS|2 (no STR/CON
            // delta), AC_Natural_Armor|6|TYPE=Base. Base Str 10-0=10, Con
            // 10-0=10.
            "stegosaurus",
            CompanionBaseStats { strength: 10, constitution: 10, natural_armor: 6, hit_die_size: 8 },
        );
        out.insert(
            // AoN/d20pfsrd: Str 12, Dex 16, Con 14, Int 1, Wis 12, Cha 3,
            // +2 natural armor. Corpus delta (`bestiary_3/companion_
            // dimetrodon.json`): CHA|-8 CON|4 DEX|6 STR|2 WIS|2,
            // AC_Natural_Armor|2|TYPE=Base. Base Str 12-2=10, Con 14-4=10.
            "dimetrodon",
            CompanionBaseStats { strength: 12, constitution: 14, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN/d20pfsrd: Str 17, Dex 15, Con 15, Int 2, Wis 12, Cha 7,
            // +3 natural armor. Corpus delta (`bestiary_3/companion_
            // iguanodon.json`): CHA|-4 CON|4 DEX|4 STR|6 WIS|2,
            // AC_Natural_Armor|3|TYPE=Base. Base Str 17-6=11, Con 15-4=11.
            "iguanodon",
            CompanionBaseStats { strength: 17, constitution: 15, natural_armor: 3, hit_die_size: 8 },
        );
        out.insert(
            // AoN/d20pfsrd: Str 18, Dex 15, Con 15, Int 2, Wis 13, Cha 3,
            // +3 natural armor. Corpus delta (`bestiary_3/companion_
            // spinosaurus.json`): CHA|-8 CON|4 DEX|4 STR|8 WIS|2,
            // AC_Natural_Armor|3|TYPE=Base. Base Str 18-8=10, Con 15-4=11.
            "spinosaurus",
            CompanionBaseStats { strength: 18, constitution: 15, natural_armor: 3, hit_die_size: 8 },
        );
        out.insert(
            // AoN/d20pfsrd (2 independent searches agree): Str 10, Dex 19,
            // Con 10, Int 2, Wis 13, Cha 12, +1 natural armor. Corpus delta
            // (`bestiary_4/companion_dinosaur_dimorphodon.json`): CHA|-4
            // CON|2 DEX|4 INT|-8 STR|-2 WIS|4, AC_Natural_Armor|1|TYPE=Base.
            // Base Str 10-(-2)=12, Con 10-2=8 -- a NEGATIVE species delta on
            // Strength (unlike every other dinosaur this table grounds),
            // consistent with Dimorphodon's own Small size and light,
            // flight-built frame.
            "dimorphodon",
            CompanionBaseStats { strength: 10, constitution: 10, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN/d20pfsrd: Str 10, Dex 14, Con 10, Int 2, Wis 12, Cha 10,
            // +6 natural armor. Corpus delta (`bestiary_4/companion_
            // dinosaur_diplodocus.json`): DEX|4 INT|-8 WIS|2 (no STR/CON
            // delta), AC_Natural_Armor|6|TYPE=Base. Base Str 10-0=10, Con
            // 10-0=10.
            "diplodocus",
            CompanionBaseStats { strength: 10, constitution: 10, natural_armor: 6, hit_die_size: 8 },
        );
        out.insert(
            // AoN/d20pfsrd: Str 10, Dex 13, Con 11, Int 2, Wis 12, Cha 7,
            // +6 natural armor. Corpus delta (`bestiary_4/companion_
            // dinosaur_styracosaurus.json`): CHA|-4 DEX|2 INT|-8 WIS|2 (no
            // STR/CON delta), AC_Natural_Armor|6|TYPE=Base. Base Str
            // 10-0=10, Con 11-0=11.
            "styracosaurus",
            CompanionBaseStats { strength: 10, constitution: 11, natural_armor: 6, hit_die_size: 8 },
        );
        out.insert(
            // AoN/d20pfsrd: Str 14, Dex 17, Con 11, Int 2, Wis 11, Cha 10,
            // +4 natural armor. Corpus delta (`bestiary_5/companion_
            // ceratosaurus.json`): DEX|6 INT|-8 STR|4 (no CON delta),
            // AC_Natural_Armor|4|TYPE=Base. Base Str 14-4=10, Con 11-0=11.
            "ceratosaurus",
            CompanionBaseStats { strength: 14, constitution: 11, natural_armor: 4, hit_die_size: 8 },
        );
        out.insert(
            // AoN/d20pfsrd: Str 12, Dex 15, Con 12, Int 2, Wis 15, Cha 9,
            // +1 natural armor. Corpus delta (`bestiary_5/companion_
            // plesiosaurus.json`): CHA|-2 CON|2 DEX|4 INT|-8 STR|2 WIS|4,
            // AC_Natural_Armor|1|TYPE=Base. Base Str 12-2=10, Con 12-2=10.
            "plesiosaurus",
            CompanionBaseStats { strength: 12, constitution: 12, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN/d20pfsrd: Str 12, Dex 18, Con 10, Int 2, Wis 15, Cha 11,
            // +4 natural armor. Corpus delta (`bestiary_5/companion_
            // therizinosaurus.json`): DEX|8 INT|-8 STR|2 WIS|4 (no CON
            // delta), AC_Natural_Armor|4|TYPE=Base. Base Str 12-2=10, Con
            // 10-0=10.
            "therizinosaurus",
            CompanionBaseStats { strength: 12, constitution: 10, natural_armor: 4, hit_die_size: 8 },
        );
        out.insert(
            // AoN/d20pfsrd: Str 7, Dex 17, Con 10, Int 2, Wis 14, Cha 13,
            // NO natural armor bonus at the starting level (the corpus's
            // own `companion_troodon.json` carries no `AC_Natural_Armor`
            // token at all, agreeing exactly). Corpus delta: CHA|2 DEX|6
            // INT|-8 STR|-4 WIS|4 (no CON delta). Base Str 7-(-4)=11, Con
            // 10-0=10. The first entry in this table with a genuine
            // natural_armor of 0, not an absent row.
            "troodon",
            CompanionBaseStats { strength: 7, constitution: 10, natural_armor: 0, hit_die_size: 8 },
        );
        out.insert(
            // AoN/d20pfsrd: Str 14, Dex 16, Con 10, Int 2, Wis 15, Cha 10,
            // +4 natural armor. Corpus delta (`bestiary_6/companion_
            // giganotosaurus.json`): DEX|6 INT|-8 STR|4 WIS|4 (no CON
            // delta), AC_Natural_Armor|4|TYPE=Base. Base Str 14-4=10, Con
            // 10-0=10.
            "giganotosaurus",
            CompanionBaseStats { strength: 14, constitution: 10, natural_armor: 4, hit_die_size: 8 },
        );
        out.insert(
            // AoN/d20pfsrd: Str 10, Dex 16, Con 10, Int 2, Wis 13, Cha 10,
            // +2 natural armor. Corpus delta (`bestiary_6/companion_
            // kentrosaurus.json`): DEX|6 INT|-8 WIS|2 (no STR/CON delta),
            // AC_Natural_Armor|2|TYPE=Base. Base Str 10-0=10, Con 10-0=10.
            "kentrosaurus",
            CompanionBaseStats { strength: 10, constitution: 10, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN/d20pfsrd: Str 9, Dex 21, Con 10, Int 2, Wis 14, Cha 12,
            // +2 natural armor. Corpus delta (`bestiary_6/companion_
            // quetzalcoatlus.json`): CHA|2 DEX|10 INT|-8 STR|-2 WIS|4 (no
            // CON delta), AC_Natural_Armor|2|TYPE=Base. Base Str
            // 9-(-2)=11, Con 10-0=10.
            "quetzalcoatlus",
            CompanionBaseStats { strength: 9, constitution: 10, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN/d20pfsrd: Str 11, Dex 18, Con 9, Int 2, Wis 13, Cha 10,
            // +2 natural armor. Corpus delta (`core_rulebook/companion_
            // dinosaur_parasaurolophus.json`): CON|-2 DEX|8 INT|-8 WIS|2
            // (no STR delta), AC_Natural_Armor|2|TYPE=Base. Base Str
            // 11-0=11, Con 9-(-2)=11.
            "parasaurolophus",
            CompanionBaseStats { strength: 11, constitution: 9, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN/d20pfsrd: Str 10, Dex 17, Con 10, Int 2, Wis 13, Cha 9,
            // +3 natural armor. Corpus delta (`core_rulebook/companion_
            // dinosaur_tylosaurus.json`): CHA|-2 DEX|6 INT|-8 WIS|2 (no
            // STR/CON delta), AC_Natural_Armor|3|TYPE=Base. Base Str
            // 10-0=10, Con 10-0=10.
            "tylosaurus",
            CompanionBaseStats { strength: 10, constitution: 10, natural_armor: 3, hit_die_size: 8 },
        );
        // Row 20 cycle 11: the two dinosaur-bucket refusals cycle 7 first
        // named and every cycle since (5-10) could not clear. Both resolve
        // this cycle via `legacy.aonprd.com/bestiary/animalCompanions.html`
        // (a single consolidated page listing many companions' own
        // "Starting Statistics" blocks directly, source 1) plus a second,
        // independent per-species `aonprd.com/DruidCompanions.aspx?
        // ItemName=...` fetch for each (source 2) -- both agreeing exactly,
        // and both cross-checked against the corpus's own `AC_Natural_
        // Armor` token as the numeric tiebreaker.
        out.insert(
            // AoN (consolidated page) & AoN (DruidCompanions.aspx?
            // ItemName=Dinosaur+(Pachycephalosaurus), independent second
            // fetch): Str 15, Dex 16, Con 13, Int 2, Wis 12, Cha 5, +3
            // natural armor -- both agree exactly. Corpus (`bestiary_3/
            // companion/companion_pachycephalosaurus.json`) natural_armor:
            // 3 -- agrees with both.
            "pachycephalosaurus",
            CompanionBaseStats { strength: 15, constitution: 13, natural_armor: 3, hit_die_size: 8 },
        );
        out.insert(
            // AoN (`DruidCompanions.aspx?ItemName=Ornithomimosaur`, a
            // targeted single-item fetch): Str 11, Dex 15, Con 12, Int 2,
            // Wis 13, Cha 8, +1 natural armor. Corpus (`ultimate_wilderness/
            // companion/companion_ornithomimosaur.json`) natural_armor: 1,
            // and the corpus's own `BONUS:STAT|DEX|4 CON|2 INT|-8 WIS|2
            // CHA|-2` delta (`uw_races_companion.lst`, no STR delta) is
            // internally consistent with the printed total -- agrees.
            "ornithomimosaur",
            CompanionBaseStats { strength: 11, constitution: 12, natural_armor: 1, hit_die_size: 8 },
        );
        // Row 20 cycle 8: the three remaining tagged `RACESUBTYPE:` buckets
        // this module's own cycle 7 next-cycle plan named -- `Aquatic` (12,
        // re-derived from 13; see this module's own cycle 8 doc addendum),
        // `PlantCompanion` (the 7 not already grounded by `gulper_plant`),
        // and `AnimalCompanionPrimate` (4). Same two-independent-source-
        // plus-corpus-tiebreaker method throughout: aonprd.com's own
        // "Starting Statistics" block (cross-checked against d20pfsrd for
        // Octopus, confirming an exact match, and against this module's own
        // cycle 6 doc comment for `hunting_cactus`, already externally
        // verified there), the corpus's own `BONUS:STAT` delta as the
        // numeric tiebreaker, and natural armor read directly from the
        // corpus's own `AC_Natural_Armor` token -- which matched every one
        // of AoN's own printed "+n natural armor" lines below exactly, 23
        // of 23, the same 100% agreement rate cycles 6 and 7 found.
        out.insert(
            // AoN: Str 14, Dex 16, Con 12, Int 1, Wis 12, Cha 8, +5 natural
            // armor. Corpus delta (`beastiary/companion_eel_giant_moray.
            // json`): CON|2 DEX|4 STR|4 (no INT/WIS/CHA delta),
            // AC_Natural_Armor|5|TYPE=Base. Base Str 14-4=10, Con 12-2=10.
            "eel_giant_moray",
            CompanionBaseStats { strength: 14, constitution: 12, natural_armor: 5, hit_die_size: 8 },
        );
        out.insert(
            // AoN (confirmed by an independent d20pfsrd fetch, exact
            // match): Str 12, Dex 17, Con 14, Int 2, Wis 12, Cha 3, +1
            // natural armor. Corpus delta (`beastiary/companion_octopus.
            // json`): STAT|STR|2 STAT|CON|4, AC_Natural_Armor|1|
            // TYPE=Base. Base Str 12-2=10, Con 14-4=10.
            "octopus",
            CompanionBaseStats { strength: 12, constitution: 14, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN: (regular) Squid -- Str 14, Dex 15, Con 11, Int 2, Wis
            // 12, Cha 2, +1 natural armor (distinct from the separate
            // `squid_giant` record below). Corpus delta (`beastiary/
            // companion_squid.json`): STAT|STR,DEX|4 (STR delta 4),
            // AC_Natural_Armor|1|TYPE=Base (no CON delta token). Base Str
            // 14-4=10, Con 11-0=11.
            "squid",
            CompanionBaseStats { strength: 14, constitution: 11, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 14, Dex 15, Con 11, Int 2, Wis 12, Cha 2, +1 natural
            // armor. Corpus delta (`bestiary_5/companion_cameroceras.
            // json`): STR|4 (no CON delta), AC_Natural_Armor|1|TYPE=Base.
            // Base Str 14-4=10, Con 11-0=11.
            "cameroceras",
            CompanionBaseStats { strength: 14, constitution: 11, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 14, Dex 18, Con 10, Int 1, Wis 13, Cha 6, +4 natural
            // armor. Corpus delta (`bestiary_6/companion_dunkleosteus.
            // json`): STR|4 (no CON delta), AC_Natural_Armor|4|TYPE=Base.
            // Base Str 14-4=10, Con 10-0=10.
            "dunkleosteus",
            CompanionBaseStats { strength: 14, constitution: 10, natural_armor: 4, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 13, Dex 15, Con 15, Int 1, Wis 12, Cha 2, +4 natural
            // armor. Corpus delta (`core_rulebook/companion_shark.json`):
            // STR|2 CON|4, AC_Natural_Armor|4|TYPE=Base. Base Str
            // 13-2=11, Con 15-4=11.
            "shark",
            CompanionBaseStats { strength: 13, constitution: 15, natural_armor: 4, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 16, Dex 9, Con 15, Int 2, Wis 12, Cha 11, +4 natural
            // armor. Corpus delta (`inner_sea_combat/companion_hippocampus.
            // json`): STR|6 CON|4, AC_Natural_Armor|4|TYPE=Base. Base Str
            // 16-6=10, Con 15-4=11.
            "hippocampus",
            CompanionBaseStats { strength: 16, constitution: 15, natural_armor: 4, hit_die_size: 8 },
        );
        out.insert(
            // AoN (Giant Crab): Str 13, Dex 14, Con 13, Int --, Wis 11, Cha
            // 4, +5 natural armor. Corpus delta (`ultimate_magic/
            // companion_crab_giant.json`): STR|2 CON|2, AC_Natural_Armor|5|
            // TYPE=Base. Base Str 13-2=11, Con 13-2=11.
            "crab_giant",
            CompanionBaseStats { strength: 13, constitution: 13, natural_armor: 5, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 13, Dex 15, Con 12, Int 1, Wis 12, Cha 2, +1 natural
            // armor. Corpus delta (`ultimate_wilderness/companion_
            // anglerfish.json`): STR|2 CON|2, AC_Natural_Armor|1|TYPE=Base.
            // Base Str 13-2=11, Con 12-2=10.
            "anglerfish",
            CompanionBaseStats { strength: 13, constitution: 12, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 13, Dex 13, Con 15, Int 1, Wis 8, Cha 2, +6 natural
            // armor. Corpus delta (`ultimate_wilderness/companion_
            // armorfish.json`): STR|2 CON|4, AC_Natural_Armor|6|TYPE=Base.
            // Base Str 13-2=11, Con 15-4=11.
            "armorfish",
            CompanionBaseStats { strength: 13, constitution: 15, natural_armor: 6, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 13, Dex 14, Con 12, Int 1, Wis 15, Cha 6, +3 natural
            // armor. Corpus delta (`ultimate_wilderness/companion_
            // hammerhead_shark.json`): STR|2 CON|2, AC_Natural_Armor|3|
            // TYPE=Base. Base Str 13-2=11, Con 12-2=10.
            "hammerhead_shark",
            CompanionBaseStats { strength: 13, constitution: 12, natural_armor: 3, hit_die_size: 8 },
        );
        out.insert(
            // AoN (Giant Squid, distinct from the regular `squid` record
            // above): Str 12, Dex 15, Con 13, Int 2, Wis 12, Cha 3, +1
            // natural armor. Corpus delta (`ultimate_wilderness/companion_
            // squid_giant.json`): STR|2 CON|2, AC_Natural_Armor|1|
            // TYPE=Base. Base Str 12-2=10, Con 13-2=11.
            "squid_giant",
            CompanionBaseStats { strength: 12, constitution: 13, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 14, Dex 13, Con 12, Int 1, Wis 11, Cha 4, +2 natural
            // armor. Corpus delta (`ultimate_wilderness/companion_corpse_
            // eater_fungus.json`): STR|4 CON|2, AC_Natural_Armor|2|
            // TYPE=Base. Base Str 14-4=10, Con 12-2=10.
            "corpse_eater_fungus",
            CompanionBaseStats { strength: 14, constitution: 12, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 12, Dex 15, Con 14, Int 1, Wis 12, Cha 9, +1 natural
            // armor. Corpus delta (`ultimate_wilderness/companion_creeping_
            // puffball.json`): STR|2 CON|4, AC_Natural_Armor|1|TYPE=Base.
            // Base Str 12-2=10, Con 14-4=10.
            "creeping_puffball",
            CompanionBaseStats { strength: 12, constitution: 14, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN, Ultimate Wilderness p.183 -- this module's own cycle 6
            // doc comment (`§17a`'s own correction worked example) already
            // externally verified: Str 14, Dex 13, Con 17, Int 2, Wis 13,
            // Cha 6, +3 natural armor (confirmed here from the corpus's own
            // `AC_Natural_Armor|3|TYPE=Base` token, not stated in cycle 6's
            // own excerpt). Corpus delta (`ultimate_wilderness/companion_
            // hunting_cactus.json`): STR|4 CON|6, matching cycle 6's own
            // figures exactly. Base Str 14-4=10, Con 17-6=11.
            "hunting_cactus",
            CompanionBaseStats { strength: 14, constitution: 17, natural_armor: 3, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 10, Dex 15, Con 13, Int 1, Wis 11, Cha 2, +1 natural
            // armor. Corpus delta (`ultimate_wilderness/companion_rash_
            // creeper.json`): CON|2 (no STR delta), AC_Natural_Armor|1|
            // TYPE=Base. Base Str 10-0=10, Con 13-2=11.
            "rash_creeper",
            CompanionBaseStats { strength: 10, constitution: 13, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 14, Dex 17, Con 13, Int 1, Wis 12, Cha 6, +1 natural
            // armor. Corpus delta (`ultimate_wilderness/companion_
            // slithering_sundew.json`): STR|4 CON|2, AC_Natural_Armor|1|
            // TYPE=Base. Base Str 14-4=10, Con 13-2=11.
            "slithering_sundew",
            CompanionBaseStats { strength: 14, constitution: 13, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 12, Dex 15, Con 14, Int 1, Wis 12, Cha 5, +2 natural
            // armor. Corpus delta (`ultimate_wilderness/companion_snapping_
            // flytrap.json`): STR|2 CON|4, AC_Natural_Armor|2|TYPE=Base.
            // Base Str 12-2=10, Con 14-4=10.
            "snapping_flytrap",
            CompanionBaseStats { strength: 12, constitution: 14, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 10, Dex 13, Con 14, Int 1, Wis 13, Cha 6, +2 natural
            // armor. Corpus delta (`ultimate_wilderness/companion_sniper_
            // cactus.json`): CON|4 (no STR delta), AC_Natural_Armor|2|
            // TYPE=Base. Base Str 10-0=10, Con 14-4=10.
            "sniper_cactus",
            CompanionBaseStats { strength: 10, constitution: 14, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 13, Dex 17, Con 10, Int 2, Wis 12, Cha 7, +1 natural
            // armor. Corpus delta (`core_rulebook/companion_ape.json`):
            // STR|2 (no CON delta), AC_Natural_Armor|1|TYPE=Base. Base Str
            // 13-2=11, Con 10-0=10.
            "ape",
            CompanionBaseStats { strength: 13, constitution: 10, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 13, Dex 17, Con 12, Int 2, Wis 12, Cha 7, +1 natural
            // armor. Corpus delta (`ultimate_wilderness/companion_
            // chimpanzee.json`): STR|2 CON|2, AC_Natural_Armor|1|
            // TYPE=Base. Base Str 13-2=11, Con 12-2=10.
            "chimpanzee",
            CompanionBaseStats { strength: 13, constitution: 12, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 15, Dex 19, Con 8, Int 2, Wis 15, Cha 10, +3 natural
            // armor. Corpus delta (`bestiary_6/companion_devil_monkey.
            // json`): STR|4 CON|-2, AC_Natural_Armor|3|TYPE=Base. Base Str
            // 15-4=11, Con 8-(-2)=10.
            "devil_monkey",
            CompanionBaseStats { strength: 15, constitution: 8, natural_armor: 3, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 13, Dex 17, Con 10, Int 2, Wis 12, Cha 7, +1 natural
            // armor -- identical printed block to `ape` (Bestiary 5's own
            // Megaprimatus shares the Ape's base). Corpus delta (`bestiary_
            // 5/companion_megaprimatus.json`): STR|2 (no CON delta),
            // AC_Natural_Armor|1|TYPE=Base. Base Str 13-2=11, Con 10-0=10.
            "megaprimatus",
            CompanionBaseStats { strength: 13, constitution: 10, natural_armor: 1, hit_die_size: 8 },
        );
        // Row 20 cycle 10: the first 22 of the 142 untagged (`RACESUBTYPE`-
        // less) `core_rulebook` companion records, using the CORRECTED
        // methodology cycle 9 established (this table's own cycle-9
        // addendum, above): the printed AoN "Starting Statistics" total is
        // grounded DIRECTLY -- never backed out by the corpus's own per-
        // species `BONUS:STAT` delta, which `ground_companion_stat_block`'s
        // own consumer (below) never reads. Every AoN total was gathered by
        // cycle 9 (`row20-cycle9-receipt.md`, "Data gathered but not yet
        // committed") via a direct `aonprd.com/DruidCompanions.aspx`
        // fetch, cross-checked against d20pfsrd for Bear/Boar (exact
        // match), and independently confirmed here by comparing the
        // corpus's own `natural_armor` field (`data/corpus/core_rulebook/
        // companion/companion_<slug>.json`) against AoN's printed "+n
        // natural armor" line -- agreement on all 22 records, cross-
        // checked fresh this cycle (`scratch_check.py`, not committed).
        // Where AoN's own printed stat block covers more than one corpus
        // record (Bird: Eagle/Hawk/Owl; Cat Big: Lion/Tiger; Cat Small:
        // Cheetah/Leopard), each corpus record still gets its own table
        // entry -- the 196-record population counts records, not species.
        out.insert(
            // AoN (Badger/Wolverine): Str 10, Dex 17, Con 15, Int 2, Wis 12,
            // Cha 10, +2 natural armor. Corpus (`companion_badger_
            // wolverine.json`) natural_armor: 2 -- agrees.
            "badger_wolverine",
            CompanionBaseStats { strength: 10, constitution: 15, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN (Bear): Str 15, Dex 15, Con 13, Int 2, Wis 12, Cha 6, +2
            // natural armor -- cross-checked against d20pfsrd, exact match.
            // Corpus (`companion_bear.json`) natural_armor: 2 -- agrees.
            "bear",
            CompanionBaseStats { strength: 15, constitution: 13, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN (Bird: Eagle/Hawk/Owl, one shared stat block): Str 10,
            // Dex 17, Con 12, Int 2, Wis 14, Cha 6, +1 natural armor.
            // Corpus (`companion_bird_eagle.json`) natural_armor: 1 --
            // agrees.
            "bird_eagle",
            CompanionBaseStats { strength: 10, constitution: 12, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN (Bird: Eagle/Hawk/Owl, one shared stat block): same as
            // `bird_eagle` above. Corpus (`companion_bird_hawk.json`)
            // natural_armor: 1 -- agrees.
            "bird_hawk",
            CompanionBaseStats { strength: 10, constitution: 12, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN (Bird: Eagle/Hawk/Owl, one shared stat block): same as
            // `bird_eagle` above. Corpus (`companion_bird_owl.json`)
            // natural_armor: 1 -- agrees.
            "bird_owl",
            CompanionBaseStats { strength: 10, constitution: 12, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN (Boar): Str 13, Dex 12, Con 15, Int 2, Wis 13, Cha 4, +6
            // natural armor -- cross-checked against d20pfsrd, exact match.
            // Corpus (`companion_boar.json`) natural_armor: 6 -- agrees.
            "boar",
            CompanionBaseStats { strength: 13, constitution: 15, natural_armor: 6, hit_die_size: 8 },
        );
        out.insert(
            // AoN (Camel): Str 18, Dex 15, Con 14, Int 2, Wis 11, Cha 4, +1
            // natural armor. Corpus (`companion_camel.json`) natural_armor:
            // 1 -- agrees.
            "camel",
            CompanionBaseStats { strength: 18, constitution: 14, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN (Cat, Big: Lion/Tiger, one shared stat block): Str 13,
            // Dex 17, Con 13, Int 2, Wis 12, Cha 6, +1 natural armor.
            // Corpus (`companion_cat_big_lion.json`) natural_armor: 1 --
            // agrees.
            "cat_big_lion",
            CompanionBaseStats { strength: 13, constitution: 13, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN (Cat, Big: Lion/Tiger, one shared stat block): same as
            // `cat_big_lion` above. Corpus (`companion_cat_big_tiger.json`)
            // natural_armor: 1 -- agrees.
            "cat_big_tiger",
            CompanionBaseStats { strength: 13, constitution: 13, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN (Cat, Small: Cheetah/Leopard, one shared stat block): Str
            // 12, Dex 21, Con 13, Int 2, Wis 12, Cha 6, +1 natural armor.
            // Corpus (`companion_cat_small_cheetah.json`) natural_armor: 1
            // -- agrees.
            "cat_small_cheetah",
            CompanionBaseStats { strength: 12, constitution: 13, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN (Cat, Small: Cheetah/Leopard, one shared stat block):
            // same as `cat_small_cheetah` above. Corpus (`companion_cat_
            // small_leopard.json`) natural_armor: 1 -- agrees.
            "cat_small_leopard",
            CompanionBaseStats { strength: 12, constitution: 13, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN (Crocodile/Alligator): Str 15, Dex 15, Con 15, Int 1, Wis
            // 12, Cha 2, +4 natural armor. Corpus (`companion_crocodile_
            // alligator.json`) natural_armor: 4 -- agrees.
            "crocodile_alligator",
            CompanionBaseStats { strength: 15, constitution: 15, natural_armor: 4, hit_die_size: 8 },
        );
        out.insert(
            // AoN (Dog): Str 13, Dex 17, Con 15, Int 2, Wis 12, Cha 6, +2
            // natural armor. Corpus (`companion_dog.json`) natural_armor:
            // 2 -- agrees.
            "dog",
            CompanionBaseStats { strength: 13, constitution: 15, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN (Pony): Str 13, Dex 13, Con 12, Int 2, Wis 13, Cha 4, +2
            // natural armor. Corpus (`companion_pony.json`) natural_armor:
            // 2 -- agrees.
            "pony",
            CompanionBaseStats { strength: 13, constitution: 12, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN (Dire Rat): Str 10, Dex 17, Con 12, Int 2, Wis 13, Cha 4,
            // +0 natural armor. Corpus (`companion_dire_rat.json`) carries
            // no `AC_Natural_Armor` token (base 0) -- agrees.
            "dire_rat",
            CompanionBaseStats { strength: 10, constitution: 12, natural_armor: 0, hit_die_size: 8 },
        );
        out.insert(
            // AoN (Herd Animal/Ram): Str 10, Dex 15, Con 11, Int 2, Wis 14,
            // Cha 4, +1 natural armor. Corpus (`companion_herd_animal_
            // ram.json`) natural_armor: 1 -- agrees.
            "herd_animal_ram",
            CompanionBaseStats { strength: 10, constitution: 11, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN (Hippopotamus): Str 11, Dex 8, Con 12, Int 2, Wis 13, Cha
            // 4, +6 natural armor. Corpus (`companion_hippopotamus.json`)
            // natural_armor: 6 -- agrees.
            "hippopotamus",
            CompanionBaseStats { strength: 11, constitution: 12, natural_armor: 6, hit_die_size: 8 },
        );
        out.insert(
            // AoN (Primate/Baboon): Str 12, Dex 15, Con 12, Int 2, Wis 12,
            // Cha 4, +0 natural armor. Corpus (`companion_primate_
            // baboon.json`) carries no `AC_Natural_Armor` token (base 0) --
            // agrees.
            "primate_baboon",
            CompanionBaseStats { strength: 12, constitution: 12, natural_armor: 0, hit_die_size: 8 },
        );
        out.insert(
            // AoN (Ray/Manta): Str 8, Dex 15, Con 11, Int 2, Wis 12, Cha 4,
            // +1 natural armor. Corpus (`companion_ray_manta.json`)
            // natural_armor: 1 -- agrees.
            "ray_manta",
            CompanionBaseStats { strength: 8, constitution: 11, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN (Ray/Stingray): Str 6, Dex 13, Con 13, Int 2, Wis 12, Cha
            // 4, +0 natural armor. Corpus (`companion_ray_stingray.json`)
            // carries no `AC_Natural_Armor` token (base 0) -- agrees.
            "ray_stingray",
            CompanionBaseStats { strength: 6, constitution: 13, natural_armor: 0, hit_die_size: 8 },
        );
        out.insert(
            // AoN (Turtle/Giant Snapping): Str 8, Dex 10, Con 9, Int 1, Wis
            // 10, Cha 2, +10 natural armor. Corpus (`companion_turtle_
            // giant_snapping.json`) natural_armor: 10 -- agrees.
            "turtle_giant_snapping",
            CompanionBaseStats { strength: 8, constitution: 9, natural_armor: 10, hit_die_size: 8 },
        );
        out.insert(
            // AoN (Snake, Constrictor): Str 15, Dex 17, Con 13, Int 1, Wis
            // 12, Cha 2, +2 natural armor. Corpus (`companion_snake_
            // constrictor.json`) natural_armor: 2 -- agrees.
            "snake_constrictor",
            CompanionBaseStats { strength: 15, constitution: 13, natural_armor: 2, hit_die_size: 8 },
        );
        // Row 20 cycle 11: 28 of the 120 untagged records that remain
        // outside `core_rulebook` after cycle 10's own 22 -- the two named
        // dinosaur-bucket refusals resolved above, plus the largest
        // single-fetch win of this row: `legacy.aonprd.com/bestiary/
        // animalCompanions.html`, ONE consolidated page carrying dozens of
        // companions' own printed "Starting Statistics" blocks from
        // Bestiary 3/4/5, Advanced Race Guide, and Monster Codex at once
        // (source 1), cross-checked per entry against the corpus's own
        // `natural_armor` field as the numeric tiebreaker -- agreement on
        // every one of the 28 below. `Giant Vulture` is reprinted verbatim
        // in both `bestiary_3` and `monster_codex` (identical stat
        // adjustments and natural-armor tokens, confirmed record by
        // record); the 196-record population counts records, not species
        // (cycle 9's own finding), so it gets two distinct table entries
        // here, matching the `bird_eagle`/`bird_hawk`/`bird_owl` and
        // `cat_big_lion`/`cat_big_tiger` precedent above.
        out.insert(
            // AoN (consolidated page): Str 10, Dex 17, Con 14, Int 2, Wis
            // 13, Cha 5, +1 natural armor. Corpus (`bestiary_3/companion/
            // companion_antelope.json`) natural_armor: 1 -- agrees.
            "antelope",
            CompanionBaseStats { strength: 10, constitution: 14, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN (consolidated page): Str 8, Dex 10, Con 9, Int 2, Wis 13,
            // Cha 6, +10 natural armor. Corpus (`bestiary_3/companion/
            // companion_archelon.json`) natural_armor: 10 -- agrees.
            "archelon",
            CompanionBaseStats { strength: 8, constitution: 9, natural_armor: 10, hit_die_size: 8 },
        );
        out.insert(
            // AoN (consolidated page): Str 10, Dex 17, Con 12, Int 2, Wis
            // 11, Cha 10, no natural-armor line (base 0). Corpus
            // (`bestiary_3/companion/companion_axe_beak.json`) carries no
            // `AC_Natural_Armor` token (base 0) -- agrees.
            "axe_beak",
            CompanionBaseStats { strength: 10, constitution: 12, natural_armor: 0, hit_die_size: 8 },
        );
        out.insert(
            // AoN (consolidated page): Str 14, Dex 14, Con 15, Int 2, Wis
            // 13, Cha 6, +4 natural armor. Corpus (`bestiary_3/companion/
            // companion_baluchitherium.json`) natural_armor: 4 -- agrees.
            "baluchitherium",
            CompanionBaseStats { strength: 14, constitution: 15, natural_armor: 4, hit_die_size: 8 },
        );
        out.insert(
            // AoN (consolidated page): Str 11, Dex 14, Con 12, Int 2, Wis
            // 14, Cha 6, +1 natural armor. Corpus (`bestiary_3/companion/
            // companion_basilosaurus.json`) natural_armor: 1 -- agrees.
            "basilosaurus",
            CompanionBaseStats { strength: 11, constitution: 12, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN (consolidated page): Str 12, Dex 17, Con 14, Int 2, Wis
            // 15, Cha 5, +1 natural armor. Corpus (`bestiary_3/companion/
            // companion_elk.json`) natural_armor: 1 -- agrees.
            "elk",
            CompanionBaseStats { strength: 12, constitution: 14, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN (consolidated page): Str 12, Dex 14, Con 14, Int 2, Wis
            // 11, Cha 7, no natural-armor line (base 0). Corpus (`bestiary_
            // 3/companion/companion_giant_chameleon.json`) carries no
            // `AC_Natural_Armor` token (base 0) -- agrees.
            "giant_chameleon",
            CompanionBaseStats { strength: 12, constitution: 14, natural_armor: 0, hit_die_size: 8 },
        );
        out.insert(
            // AoN (consolidated page): Str 11, Dex 15, Con 12, Int 2, Wis
            // 14, Cha 7, no natural-armor line (base 0). Corpus (`bestiary_
            // 3/companion/companion_giant_gecko.json`) carries no `AC_
            // Natural_Armor` token (base 0) -- agrees.
            "giant_gecko",
            CompanionBaseStats { strength: 11, constitution: 12, natural_armor: 0, hit_die_size: 8 },
        );
        out.insert(
            // AoN (consolidated page): Str 12, Dex 15, Con 14, Int 2, Wis
            // 15, Cha 7, +2 natural armor. Corpus (`bestiary_3/companion/
            // companion_giant_vulture.json`) natural_armor: 2 -- agrees.
            "giant_vulture",
            CompanionBaseStats { strength: 12, constitution: 14, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // Same printed species, reprinted verbatim in `monster_codex`
            // (`companion_giant_vulture.json`, identical stat-adjustment
            // and natural-armor tokens to the `bestiary_3` record above) --
            // a distinct corpus record, so it gets its own table entry per
            // this module's own duplicate-record precedent (see doc note
            // above). AoN (consolidated page): Str 12, Dex 15, Con 14, Int
            // 2, Wis 15, Cha 7, +2 natural armor. Corpus (`monster_codex/
            // companion/companion_giant_vulture.json`) natural_armor: 2 --
            // agrees.
            "giant_vulture_monster_codex",
            CompanionBaseStats { strength: 12, constitution: 14, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN (consolidated page): Str 12, Dex 14, Con 13, Int 2, Wis
            // 11, Cha 7, no natural-armor line (base 0). Corpus (`bestiary_
            // 3/companion/companion_kangaroo.json`) carries no `AC_Natural_
            // Armor` token (base 0) -- agrees.
            "kangaroo",
            CompanionBaseStats { strength: 12, constitution: 13, natural_armor: 0, hit_die_size: 8 },
        );
        out.insert(
            // AoN (consolidated page): Str 13, Dex 17, Con 12, Int 2, Wis
            // 12, Cha 6, +1 natural armor. Corpus (`bestiary_3/companion/
            // companion_megalania.json`) natural_armor: 1 -- agrees.
            "megalania",
            CompanionBaseStats { strength: 13, constitution: 12, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN (consolidated page): Str 12, Dex 15, Con 16, Int 2, Wis
            // 13, Cha 7, no natural-armor line (base 0). Corpus (`bestiary_
            // 3/companion/companion_thylacine.json`) carries no `AC_
            // Natural_Armor` token (base 0) -- agrees.
            "thylacine",
            CompanionBaseStats { strength: 12, constitution: 16, natural_armor: 0, hit_die_size: 8 },
        );
        out.insert(
            // AoN (consolidated page, "Giant Weasel"): Str 10, Dex 19, Con
            // 13, Int 2, Wis 12, Cha 10, +1 natural armor. Corpus
            // (`bestiary_4/companion/companion_weasel_giant.json`) natural_
            // armor: 1 -- agrees.
            "weasel_giant",
            CompanionBaseStats { strength: 10, constitution: 13, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN (consolidated page): Str 16, Dex 17, Con 12, Int 2, Wis
            // 13, Cha 7, +2 natural armor. Corpus (`bestiary_4/companion/
            // companion_giraffe.json`) natural_armor: 2 -- agrees.
            "giraffe",
            CompanionBaseStats { strength: 16, constitution: 12, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN (consolidated page): Str 16, Dex 13, Con 15, Int 1, Wis
            // 12, Cha 6, +4 natural armor. Corpus (`bestiary_4/companion/
            // companion_seahorse.json`) natural_armor: 4 -- agrees.
            "seahorse",
            CompanionBaseStats { strength: 16, constitution: 15, natural_armor: 4, hit_die_size: 8 },
        );
        out.insert(
            // AoN (consolidated page): Str 10, Dex 19, Con 14, Int 2, Wis
            // 15, Cha 8, no natural-armor line (base 0). Corpus (`bestiary_
            // 4/companion/companion_stag.json`) carries no `AC_Natural_
            // Armor` token (base 0) -- agrees.
            "stag",
            CompanionBaseStats { strength: 10, constitution: 14, natural_armor: 0, hit_die_size: 8 },
        );
        out.insert(
            // AoN (consolidated page): Str 14, Dex 8, Con 16, Int 2, Wis 13,
            // Cha 9, +6 natural armor. Corpus (`bestiary_4/companion/
            // companion_tortoise.json`) natural_armor: 6 -- agrees.
            "tortoise",
            CompanionBaseStats { strength: 14, constitution: 16, natural_armor: 6, hit_die_size: 8 },
        );
        out.insert(
            // AoN (consolidated page): Str 10, Dex 15, Con 12, Int 2, Wis
            // 12, Cha 5, no natural-armor line (base 0). Corpus (`bestiary_
            // 4/companion/companion_trumpeter_swan.json`) carries no `AC_
            // Natural_Armor` token (base 0) -- agrees.
            "trumpeter_swan",
            CompanionBaseStats { strength: 10, constitution: 12, natural_armor: 0, hit_die_size: 8 },
        );
        out.insert(
            // AoN (consolidated page): Str 12, Dex 13, Con 14, Int 2, Wis
            // 13, Cha 6, +4 natural armor. Corpus (`bestiary_4/companion/
            // companion_walrus.json`) natural_armor: 4 -- agrees.
            "walrus",
            CompanionBaseStats { strength: 12, constitution: 14, natural_armor: 4, hit_die_size: 8 },
        );
        out.insert(
            // AoN (consolidated page, "Blue Whale"): Str 11, Dex 19, Con 10,
            // Int 2, Wis 14, Cha 6, +1 natural armor. Corpus (`bestiary_5/
            // companion/companion_whale_blue.json`) natural_armor: 1 --
            // agrees.
            "whale_blue",
            CompanionBaseStats { strength: 11, constitution: 10, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN (consolidated page): Str 12, Dex 14, Con 13, Int 2, Wis
            // 13, Cha 3, +4 natural armor. Corpus (`bestiary_5/companion/
            // companion_chalicotherium.json`) natural_armor: 4 -- agrees.
            "chalicotherium",
            CompanionBaseStats { strength: 12, constitution: 13, natural_armor: 4, hit_die_size: 8 },
        );
        out.insert(
            // AoN (consolidated page): Str 10, Dex 21, Con 13, Int 2, Wis
            // 12, Cha 6, +1 natural armor. Corpus (`bestiary_5/companion/
            // companion_digmaul.json`) natural_armor: 1 -- agrees.
            "digmaul",
            CompanionBaseStats { strength: 10, constitution: 13, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN (consolidated page): Str 15, Dex 14, Con 15, Int 1, Wis
            // 14, Cha 2, +3 natural armor. Corpus (`bestiary_5/companion/
            // companion_kaprosuchus.json`) natural_armor: 3 -- agrees.
            "kaprosuchus",
            CompanionBaseStats { strength: 15, constitution: 15, natural_armor: 3, hit_die_size: 8 },
        );
        out.insert(
            // AoN (consolidated page): Str 13, Dex 16, Con 12, Int 2, Wis
            // 11, Cha 9, +1 natural armor. Corpus (`bestiary_5/companion/
            // companion_moa.json`) natural_armor: 1 -- agrees.
            "moa",
            CompanionBaseStats { strength: 13, constitution: 12, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN (consolidated page): Str 11, Dex 19, Con 10, Int 2, Wis
            // 14, Cha 6, +1 natural armor. Corpus (`bestiary_5/companion/
            // companion_narwhal.json`) natural_armor: 1 -- agrees.
            "narwhal",
            CompanionBaseStats { strength: 11, constitution: 10, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN (consolidated page): Str 14, Dex 12, Con 17, Int 1, Wis
            // 13, Cha 3, +4 natural armor. Corpus (`bestiary_5/companion/
            // companion_uintatherium.json`) natural_armor: 4 -- agrees.
            "uintatherium",
            CompanionBaseStats { strength: 14, constitution: 17, natural_armor: 4, hit_die_size: 8 },
        );
        out.insert(
            // AoN (consolidated page): Str 14, Dex 16, Con 12, Int 2, Wis
            // 11, Cha 4, +1 natural armor. Corpus (`bestiary_5/companion/
            // companion_wolliped.json`) natural_armor: 1 -- agrees.
            "wolliped",
            CompanionBaseStats { strength: 14, constitution: 12, natural_armor: 1, hit_die_size: 8 },
        );
        // Cycle 12: the single largest remaining bucket, `ultimate_wilderness`'s
        // own 38 untagged `RACETYPE:Companion` records not already grounded by
        // `gulper_plant`/`ornithomimosaur`. Source: `aonprd.com/DruidCompanions.
        // aspx?ItemName=<species>`'s own printed "Starting Statistics" line per
        // species (an index page, `DruidCompanions.aspx?ItemName=All&Category=
        // Animal`, enumerated the full species list first), cross-checked
        // against the corpus's own `natural_armor` token for all 38 -- 100%
        // agreement, same as every prior cycle.
        out.insert(
            // AoN: Str 8, Dex 15, Con 12, Int 2, Wis 13, Cha 7, +1 natural armor
            // (Ultimate Wilderness p.178). Corpus (`ultimate_wilderness/companion/
            // companion_archaeopteryx.json`) natural_armor: 1 -- agrees.
            "archaeopteryx",
            CompanionBaseStats { strength: 8, constitution: 12, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 13, Dex 15, Con 13, Int --, Wis 10, Cha 2, +2 natural
            // armor (Ultimate Wilderness p.184). Corpus (`companion_assassin_
            // bug_giant.json`) natural_armor: 2 -- agrees.
            "assassin_bug_giant",
            CompanionBaseStats { strength: 13, constitution: 13, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 13, Dex 14, Con 13, Int 2, Wis 13, Cha 5, +2 natural armor
            // (Ultimate Wilderness p.178). Corpus (`companion_bustard.json`)
            // natural_armor: 2 -- agrees.
            "bustard",
            CompanionBaseStats { strength: 13, constitution: 13, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 12, Dex 15, Con 12, Int 2, Wis 13, Cha 5, +1 natural armor
            // (Ultimate Wilderness p.179). Corpus (`companion_capybara.json`)
            // natural_armor: 1 -- agrees.
            "capybara",
            CompanionBaseStats { strength: 12, constitution: 12, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 14, Dex 13, Con 13, Int --, Wis 11, Cha 2, +2 natural
            // armor (Ultimate Wilderness p.184). Corpus (`companion_caterpillar_
            // giant.json`) natural_armor: 2 -- agrees.
            "caterpillar_giant",
            CompanionBaseStats { strength: 14, constitution: 13, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 14, Dex 12, Con 12, Int 2, Wis 11, Cha 4, +2 natural armor
            // (Ultimate Wilderness p.179). Corpus (`companion_cattle.json`)
            // natural_armor: 2 -- agrees.
            "cattle",
            CompanionBaseStats { strength: 14, constitution: 12, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 9, Dex 10, Con 17, Int --, Wis 11, Cha 2, +1 natural armor
            // (Ultimate Wilderness p.184). Corpus (`companion_cockroach_giant.
            // json`) natural_armor: 1 -- agrees.
            "cockroach_giant",
            CompanionBaseStats { strength: 9, constitution: 17, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 13, Dex 17, Con 12, Int --, Wis 12, Cha 9, +2 natural
            // armor (Ultimate Wilderness p.184). Corpus (`companion_dragonfly_
            // giant.json`) natural_armor: 2 -- agrees.
            "dragonfly_giant",
            CompanionBaseStats { strength: 13, constitution: 12, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 11, Dex 15, Con 12, Int 2, Wis 11, Cha 4, +2 natural armor
            // (Ultimate Wilderness p.179). Corpus (`companion_eohippus.json`)
            // natural_armor: 2 -- agrees.
            "eohippus",
            CompanionBaseStats { strength: 11, constitution: 12, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 10, Dex 11, Con 12, Int --, Wis 13, Cha 2, +2 natural
            // armor (Ultimate Wilderness p.184). Corpus (`companion_eurypterid.
            // json`) natural_armor: 2 -- agrees.
            "eurypterid",
            CompanionBaseStats { strength: 10, constitution: 12, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 10, Dex 15, Con 11, Int 2, Wis 15, Cha 6, +1 natural armor
            // (Ultimate Wilderness p.179). Corpus (`companion_falcon.json`)
            // natural_armor: 1 -- agrees.
            "falcon",
            CompanionBaseStats { strength: 10, constitution: 11, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 13, Dex 15, Con 13, Int 2, Wis 14, Cha 10, +4 natural
            // armor (Ultimate Wilderness p.179). Corpus (`companion_frilled_
            // lizard_giant.json`) natural_armor: 4 -- agrees.
            "frilled_lizard_giant",
            CompanionBaseStats { strength: 13, constitution: 13, natural_armor: 4, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 17, Dex 13, Con 13, Int 2, Wis 13, Cha 6, +1 natural armor
            // (Ultimate Wilderness p.179). Corpus (`companion_grizzly_bear.json`)
            // natural_armor: 1 -- agrees.
            "grizzly_bear",
            CompanionBaseStats { strength: 17, constitution: 13, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 11, Dex 16, Con 12, Int 2, Wis 13, Cha 9, +1 natural armor
            // (Ultimate Wilderness p.180). Corpus (`companion_llama.json`)
            // natural_armor: 1 -- agrees.
            "llama",
            CompanionBaseStats { strength: 11, constitution: 12, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 12, Dex 17, Con 11, Int --, Wis 10, Cha 7, +3 natural
            // armor (Ultimate Wilderness p.185). Corpus (`companion_locust_
            // giant.json`) natural_armor: 3 -- agrees.
            "locust_giant",
            CompanionBaseStats { strength: 12, constitution: 11, natural_armor: 3, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 12, Dex 17, Con 14, Int --, Wis 13, Cha 6, +3 natural
            // armor (Ultimate Wilderness p.185). Corpus (`companion_mantis_
            // shrimp_giant.json`) natural_armor: 3 -- agrees.
            "mantis_shrimp_giant",
            CompanionBaseStats { strength: 12, constitution: 14, natural_armor: 3, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 15, Dex 13, Con 12, Int 2, Wis 10, Cha 7, +1 natural armor
            // (Ultimate Wilderness p.180). Corpus (`companion_marsupial_devil.
            // json`) natural_armor: 1 -- agrees.
            "marsupial_devil",
            CompanionBaseStats { strength: 15, constitution: 12, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 12, Dex 16, Con 13, Int 2, Wis 13, Cha 7, +1 natural armor
            // (Ultimate Wilderness p.180). Corpus (`companion_marsupial_lion.
            // json`) natural_armor: 1 -- agrees.
            "marsupial_lion",
            CompanionBaseStats { strength: 12, constitution: 13, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 12, Dex 15, Con 13, Int 2, Wis 10, Cha 5, +1 natural armor
            // (Ultimate Wilderness p.180). Corpus (`companion_mole_giant.json`)
            // natural_armor: 1 -- agrees.
            "mole_giant",
            CompanionBaseStats { strength: 12, constitution: 13, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 12, Dex 15, Con 14, Int 2, Wis 13, Cha 7, +2 natural armor
            // (Ultimate Wilderness p.180). Corpus (`companion_moose.json`)
            // natural_armor: 2 -- agrees.
            "moose",
            CompanionBaseStats { strength: 12, constitution: 14, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 14, Dex 21, Con 15, Int --, Wis 13, Cha 6, +1 natural
            // armor (Ultimate Wilderness p.185). Corpus (`companion_mosquito_
            // giant.json`) natural_armor: 1 -- agrees.
            "mosquito_giant",
            CompanionBaseStats { strength: 14, constitution: 15, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 10, Dex 17, Con 13, Int 2, Wis 13, Cha 6, +2 natural armor
            // (Ultimate Wilderness p.181). Corpus (`companion_owl_giant.json`)
            // natural_armor: 2 -- agrees.
            "owl_giant",
            CompanionBaseStats { strength: 10, constitution: 13, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 13, Dex 12, Con 14, Int 2, Wis 13, Cha 9, +2 natural armor
            // (Ultimate Wilderness p.181). Corpus (`companion_panda.json`)
            // natural_armor: 2 -- agrees.
            "panda",
            CompanionBaseStats { strength: 13, constitution: 14, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 14, Dex 12, Con 15, Int 2, Wis 13, Cha 5, +1 natural armor
            // (Ultimate Wilderness p.181). Corpus (`companion_porcupine_giant.
            // json`) natural_armor: 1 -- agrees.
            "porcupine_giant",
            CompanionBaseStats { strength: 14, constitution: 15, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 13, Dex 14, Con 15, Int 1, Wis 12, Cha 4, +2 natural armor
            // (Ultimate Wilderness p.181). Corpus (`companion_prionosuchus.
            // json`) natural_armor: 2 -- agrees.
            "prionosuchus",
            CompanionBaseStats { strength: 13, constitution: 15, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 10, Dex 15, Con 12, Int 1, Wis 13, Cha 2, +2 natural armor
            // (Ultimate Wilderness p.181). Corpus (`companion_reef_snake.json`)
            // natural_armor: 2 -- agrees.
            "reef_snake",
            CompanionBaseStats { strength: 10, constitution: 12, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 13, Dex 14, Con 13, Int 2, Wis 12, Cha 5, +2 natural armor
            // (Ultimate Wilderness p.181). Corpus (`companion_reindeer.json`)
            // natural_armor: 2 -- agrees.
            "reindeer",
            CompanionBaseStats { strength: 13, constitution: 13, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 15, Dex 15, Con 13, Int 2, Wis 13, Cha 8, +1 natural armor
            // (Ultimate Wilderness p.181). Corpus (`companion_saber_toothed_
            // cat.json`) natural_armor: 1 -- agrees.
            "saber_toothed_cat",
            CompanionBaseStats { strength: 15, constitution: 13, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 13, Dex 14, Con 15, Int 1, Wis 12, Cha 4, +3 natural armor
            // (Ultimate Wilderness p.181). Corpus (`companion_salamander_giant.
            // json`) natural_armor: 3 -- agrees.
            "salamander_giant",
            CompanionBaseStats { strength: 13, constitution: 15, natural_armor: 3, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 9, Dex 14, Con 13, Int 2, Wis 12, Cha 9, +1 natural armor
            // (Ultimate Wilderness p.181). Corpus (`companion_skunk_giant.json`)
            // natural_armor: 1 -- agrees.
            "skunk_giant",
            CompanionBaseStats { strength: 9, constitution: 13, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 12, Dex 15, Con 15, Int --, Wis 11, Cha 2, +1 natural
            // armor (Ultimate Wilderness p.185). Corpus (`companion_solifugid_
            // giant.json`) natural_armor: 1 -- agrees.
            "solifugid_giant",
            CompanionBaseStats { strength: 12, constitution: 15, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 10, Dex 17, Con 10, Int --, Wis 10, Cha 2, +1 natural
            // armor (Ultimate Wilderness p.185). Corpus (`companion_spider_web_
            // tyrant.json`) natural_armor: 1 -- agrees.
            "spider_web_tyrant",
            CompanionBaseStats { strength: 10, constitution: 10, natural_armor: 1, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 8, Dex 15, Con 11, Int 1, Wis 12, Cha 2, +2 natural armor
            // (Ultimate Wilderness p.182). Corpus (`companion_spitting_cobra.
            // json`) natural_armor: 2 -- agrees.
            "spitting_cobra",
            CompanionBaseStats { strength: 8, constitution: 11, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 12, Dex 11, Con 12, Int --, Wis 12, Cha 7, +2 natural
            // armor (Ultimate Wilderness p.185). Corpus (`companion_termite_
            // giant.json`) natural_armor: 2 -- agrees.
            "termite_giant",
            CompanionBaseStats { strength: 12, constitution: 12, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 9, Dex 13, Con 13, Int --, Wis 10, Cha 2, +3 natural
            // armor (Ultimate Wilderness p.185). Corpus (`companion_whiptail_
            // centipede_giant.json`) natural_armor: 3 -- agrees.
            "whiptail_centipede_giant",
            CompanionBaseStats { strength: 9, constitution: 13, natural_armor: 3, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 15, Dex 15, Con 13, Int 2, Wis 12, Cha 6, +2 natural armor
            // (Ultimate Wilderness p.182). Corpus (`companion_wolfdog.json`)
            // natural_armor: 2 -- agrees.
            "wolfdog",
            CompanionBaseStats { strength: 15, constitution: 13, natural_armor: 2, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 14, Dex 10, Con 12, Int 2, Wis 10, Cha 5, +4 natural armor
            // (Ultimate Wilderness p.182). Corpus (`companion_yak.json`)
            // natural_armor: 4 -- agrees.
            "yak",
            CompanionBaseStats { strength: 14, constitution: 12, natural_armor: 4, hit_die_size: 8 },
        );
        out.insert(
            // AoN: Str 14, Dex 15, Con 13, Int 2, Wis 14, Cha 6, +2 natural armor
            // (Ultimate Wilderness p.182). Corpus (`companion_zebra.json`)
            // natural_armor: 2 -- agrees.
            "zebra",
            CompanionBaseStats { strength: 14, constitution: 13, natural_armor: 2, hit_die_size: 8 },
        );
        // AoN: Str 9, Con 9, +0 natural armor (Pathfinder RPG Bestiary p.30). Corpus
        // (`bat_dire.json`) natural_armor: 0 -- agrees.
        out.insert(
            "bat_dire",
            CompanionBaseStats { strength: 9, constitution: 9, natural_armor: 0, hit_die_size: 8 },
        );
        // AoN: Str 12, Con 13, +1 natural armor (Pathfinder RPG Bestiary p.88). Corpus
        // (`dolphin.json`) natural_armor: 1 -- agrees.
        out.insert(
            "dolphin",
            CompanionBaseStats { strength: 12, constitution: 13, natural_armor: 1, hit_die_size: 8 },
        );
        // AoN (`aonprd.com/DruidCompanions.aspx?ItemName=Orca`, corroborated
        // by an independent search-engine cross-check): Str 11, Con 10, +1
        // natural armor (Pathfinder RPG Bestiary p.88). Corpus (`beastiary/companion_dolphin_
        // orca.json`) carries `BONUS:VAR|AC_Natural_Armor|6|TYPE=Base` --
        // PCGen's own oracle (`b1_races_companion.lst` line 16) really does say
        // 6, not a repo ingestion error, but it is provably wrong: the wild-
        // monster `Dolphin (Orca)` race entry in the SAME oracle file
        // (`b1_races.lst`) shares the identical `AC_Natural_Armor|6` token --
        // strong evidence PCGen's companion entry was copy-pasted from the
        // standalone Huge Orca monster stat block rather than authored from
        // the companion-specific Medium starting statistics. Two independent
        // sources agree on 1; the printed total is grounded here directly,
        // matching this table's own doctrine of measurement over fabrication
        // (`decisions.md §1a`) -- the corpus's own natural-armor token is the
        // outlier this cycle, not the ground truth, for this one record only.
        out.insert(
            "dolphin_orca",
            CompanionBaseStats { strength: 11, constitution: 10, natural_armor: 1, hit_die_size: 8 },
        );
        // AoN: Str 12, Con 18, +0 natural armor (Pathfinder RPG Bestiary p.119). Corpus
        // (`eel_electric.json`) natural_armor: 0 -- agrees.
        out.insert(
            "eel_electric",
            CompanionBaseStats { strength: 12, constitution: 18, natural_armor: 0, hit_die_size: 8 },
        );
        // AoN: Str 14, Con 13, +4 natural armor (Pathfinder RPG Bestiary p.128). Corpus
        // (`elephant.json`) natural_armor: 4 -- agrees.
        out.insert(
            "elephant",
            CompanionBaseStats { strength: 14, constitution: 13, natural_armor: 4, hit_die_size: 8 },
        );
        // AoN: Str 14, Con 13, +4 natural armor (Pathfinder RPG Bestiary p.128). Corpus
        // (`elephant_mastodon.json`) natural_armor: 4 -- agrees.
        out.insert(
            "elephant_mastodon",
            CompanionBaseStats { strength: 14, constitution: 13, natural_armor: 4, hit_die_size: 8 },
        );
        // AoN: Str 15, Con 16, +1 natural armor (Pathfinder RPG Bestiary p.135). Corpus
        // (`frog_giant.json`) natural_armor: 1 -- agrees.
        out.insert(
            "frog_giant",
            CompanionBaseStats { strength: 15, constitution: 16, natural_armor: 1, hit_die_size: 8 },
        );
        // AoN: Str 11, Con 11, +0 natural armor (Pathfinder RPG Bestiary p.157). Corpus
        // (`goblin_dog.json`) natural_armor: 0 -- agrees.
        out.insert(
            "goblin_dog",
            CompanionBaseStats { strength: 11, constitution: 11, natural_armor: 0, hit_die_size: 8 },
        );
        // AoN: Str 14, Con 12, +1 natural armor (Pathfinder RPG Bestiary p.174). Corpus
        // (`herd_animal_aurochs.json`) natural_armor: 1 -- agrees.
        out.insert(
            "herd_animal_aurochs",
            CompanionBaseStats { strength: 14, constitution: 12, natural_armor: 1, hit_die_size: 8 },
        );
        // AoN: Str 14, Con 12, +1 natural armor (Pathfinder RPG Bestiary p.174). Corpus
        // (`herd_animal_bison.json`) natural_armor: 1 -- agrees.
        out.insert(
            "herd_animal_bison",
            CompanionBaseStats { strength: 14, constitution: 12, natural_armor: 1, hit_die_size: 8 },
        );
        // AoN: Str 10, Con 13, +2 natural armor (Pathfinder RPG Bestiary p.179). Corpus
        // (`hyena.json`) natural_armor: 2 -- agrees.
        out.insert(
            "hyena",
            CompanionBaseStats { strength: 10, constitution: 13, natural_armor: 2, hit_die_size: 8 },
        );
        // AoN: Str 13, Con 12, +1 natural armor (Pathfinder RPG Bestiary p.194). Corpus
        // (`lizard_monitor.json`) natural_armor: 1 -- agrees.
        out.insert(
            "lizard_monitor",
            CompanionBaseStats { strength: 13, constitution: 12, natural_armor: 1, hit_die_size: 8 },
        );
        // AoN: Str 14, Con 15, +4 natural armor (Pathfinder RPG Bestiary p.235). Corpus
        // (`rhinoceros.json`) natural_armor: 4 -- agrees.
        out.insert(
            "rhinoceros",
            CompanionBaseStats { strength: 14, constitution: 15, natural_armor: 4, hit_die_size: 8 },
        );
        // AoN: Str 12, Con 9, +5 natural armor (Pathfinder RPG Bestiary p.236). Corpus
        // (`roc.json`) natural_armor: 5 -- agrees.
        out.insert(
            "roc",
            CompanionBaseStats { strength: 12, constitution: 9, natural_armor: 5, hit_die_size: 8 },
        );
        // AoN: Str 10, Con 15, +2 natural armor (Ultimate Magic p.36). Corpus
        // (`ant_giant.json`) natural_armor: 2 -- agrees.
        out.insert(
            "ant_giant",
            CompanionBaseStats { strength: 10, constitution: 15, natural_armor: 2, hit_die_size: 8 },
        );
        // AoN: Str 13, Con 13, +6 natural armor (Ultimate Magic p.36). Corpus
        // (`beetle_giant.json`) natural_armor: 6 -- agrees.
        out.insert(
            "beetle_giant",
            CompanionBaseStats { strength: 13, constitution: 13, natural_armor: 6, hit_die_size: 8 },
        );
        // AoN: Str 8, Con 11, +2 natural armor (Ultimate Magic p.36). Corpus
        // (`centipede_giant.json`) natural_armor: 2 -- agrees.
        out.insert(
            "centipede_giant",
            CompanionBaseStats { strength: 8, constitution: 11, natural_armor: 2, hit_die_size: 8 },
        );
        // AoN: Str 9, Con 12, +0 natural armor (Ultimate Magic p.36). Corpus
        // (`leech_giant.json`) natural_armor: 0 -- agrees.
        out.insert(
            "leech_giant",
            CompanionBaseStats { strength: 9, constitution: 12, natural_armor: 0, hit_die_size: 8 },
        );
        // AoN: Str 10, Con 10, +3 natural armor (Ultimate Magic p.37). Corpus
        // (`mantis_giant.json`) natural_armor: 3 -- agrees.
        out.insert(
            "mantis_giant",
            CompanionBaseStats { strength: 10, constitution: 10, natural_armor: 3, hit_die_size: 8 },
        );
        // AoN: Str 11, Con 12, +1 natural armor (Ultimate Magic p.37). Corpus
        // (`scorpion_giant.json`) natural_armor: 1 -- agrees.
        out.insert(
            "scorpion_giant",
            CompanionBaseStats { strength: 11, constitution: 12, natural_armor: 1, hit_die_size: 8 },
        );
        // AoN: Str 13, Con 13, +4 natural armor (Ultimate Magic p.37). Corpus
        // (`slug_giant.json`) natural_armor: 4 -- agrees.
        out.insert(
            "slug_giant",
            CompanionBaseStats { strength: 13, constitution: 13, natural_armor: 4, hit_die_size: 8 },
        );
        // AoN: Str 6, Con 10, +0 natural armor (Ultimate Magic p.37). Corpus
        // (`spider_giant.json`) natural_armor: 0 -- agrees.
        out.insert(
            "spider_giant",
            CompanionBaseStats { strength: 6, constitution: 10, natural_armor: 0, hit_die_size: 8 },
        );
        // AoN: Str 10, Con 11, +2 natural armor (Ultimate Magic p.37). Corpus
        // (`wasp_giant.json`) natural_armor: 2 -- agrees.
        out.insert(
            "wasp_giant",
            CompanionBaseStats { strength: 10, constitution: 11, natural_armor: 2, hit_die_size: 8 },
        );
        // AoN: Str 10, Con 15, +2 natural armor (Advanced Race Guide p.26). Corpus
        // (`carnivorous_flower.json`) natural_armor: 2 -- agrees.
        out.insert(
            "carnivorous_flower",
            CompanionBaseStats { strength: 10, constitution: 15, natural_armor: 2, hit_die_size: 8 },
        );
        // AoN: Str 13, Con 13, +2 natural armor (Advanced Race Guide p.26). Corpus
        // (`crawling_vine.json`) natural_armor: 2 -- agrees.
        out.insert(
            "crawling_vine",
            CompanionBaseStats { strength: 13, constitution: 13, natural_armor: 2, hit_die_size: 8 },
        );
        // AoN: Str 10, Con 12, +1 natural armor (Advanced Race Guide p.26). Corpus
        // (`puffball.json`) natural_armor: 1 -- agrees.
        out.insert(
            "puffball",
            CompanionBaseStats { strength: 10, constitution: 12, natural_armor: 1, hit_die_size: 8 },
        );
        // AoN: Str 15, Con 12, +1 natural armor (Advanced Race Guide p.26). Corpus
        // (`sapling_treant.json`) natural_armor: 1 -- agrees.
        out.insert(
            "sapling_treant",
            CompanionBaseStats { strength: 15, constitution: 12, natural_armor: 1, hit_die_size: 8 },
        );
        // Advanced Race Guide p.56, the Fell Rider (Hobgoblin Cavalier) archetype's own
        // "Brute Steed" class feature: "gains a +2 bonus to Strength, but
        // takes a -2 penalty to Dexterity. This ability otherwise works like
        // the cavalier's mount ability" -- not an independent stat block, a
        // modifier on the ordinary Camel companion (already grounded above,
        // Str 18/Con 14/+1 natural armor). Str 18+2=20, Con
        // unaffected (14), natural armor unaffected (1) -- Dex is not a
        // field this table tracks. Corpus (`advanced_race_guide/companion_
        // brute_steed_camel.json`) `natural_armor`: 1 -- agrees with the unmodified Camel
        // value, independently confirming this derivation.
        out.insert(
            "brute_steed_camel",
            CompanionBaseStats { strength: 20, constitution: 14, natural_armor: 1, hit_die_size: 8 },
        );
        // Advanced Race Guide p.56, the Fell Rider (Hobgoblin Cavalier) archetype's own
        // "Brute Steed" class feature: "gains a +2 bonus to Strength, but
        // takes a -2 penalty to Dexterity. This ability otherwise works like
        // the cavalier's mount ability" -- not an independent stat block, a
        // modifier on the ordinary Horse companion (already grounded above,
        // Str 16/Con 15/+4 natural armor). Str 16+2=18, Con
        // unaffected (15), natural armor unaffected (4) -- Dex is not a
        // field this table tracks. Corpus (`advanced_race_guide/companion_
        // brute_steed_horse.json`) `natural_armor`: 4 -- agrees with the unmodified Horse
        // value, independently confirming this derivation.
        out.insert(
            "brute_steed_horse",
            CompanionBaseStats { strength: 18, constitution: 15, natural_armor: 4, hit_die_size: 8 },
        );
        // AoN: Str 12, Con 12, +8 natural armor (Bestiary 6 p.311). Corpus
        // (`brontotherium.json`) natural_armor: 8 -- agrees.
        out.insert(
            "brontotherium",
            CompanionBaseStats { strength: 12, constitution: 12, natural_armor: 8, hit_die_size: 8 },
        );
        // AoN: Str 14, Con 15, +4 natural armor (Bestiary 6 p.311). Corpus
        // (`deinotherium.json`) natural_armor: 4 -- agrees.
        out.insert(
            "deinotherium",
            CompanionBaseStats { strength: 14, constitution: 15, natural_armor: 4, hit_die_size: 8 },
        );
        // AoN: Str 14, Con 15, +4 natural armor (Bestiary 6 p.312). Corpus
        // (`elasmotherium.json`) natural_armor: 4 -- agrees.
        out.insert(
            "elasmotherium",
            CompanionBaseStats { strength: 14, constitution: 15, natural_armor: 4, hit_die_size: 8 },
        );
        // AoN: Str 16, Con 10, +5 natural armor (Bestiary 6 p.312). Corpus
        // (`mokele_mbembe.json`) natural_armor: 5 -- agrees.
        out.insert(
            "mokele_mbembe",
            CompanionBaseStats { strength: 16, constitution: 10, natural_armor: 5, hit_die_size: 8 },
        );
        // AoN: Str 8, Con 10, +0 natural armor (Bestiary 6 p.312). Corpus
        // (`raven_giant.json`) natural_armor: 0 -- agrees.
        out.insert(
            "raven_giant",
            CompanionBaseStats { strength: 8, constitution: 10, natural_armor: 0, hit_die_size: 8 },
        );
        // AoN: Str 15, Con 11, +4 natural armor (Bestiary 6 p.312). Corpus
        // (`titanoboa.json`) natural_armor: 4 -- agrees.
        out.insert(
            "titanoboa",
            CompanionBaseStats { strength: 15, constitution: 11, natural_armor: 4, hit_die_size: 8 },
        );
        // AoN: Str 14, Con 15, +1 natural armor (Bestiary 2 p.128). Corpus
        // (`gar.json`) natural_armor: 1 -- agrees.
        out.insert(
            "gar",
            CompanionBaseStats { strength: 14, constitution: 15, natural_armor: 1, hit_die_size: 8 },
        );
        // AoN: Str 14, Con 15, +4 natural armor (Bestiary 2 p.186). Corpus
        // (`megafauna_arsinoitherium.json`) natural_armor: 4 -- agrees.
        out.insert(
            "megafauna_arsinoitherium",
            CompanionBaseStats { strength: 14, constitution: 15, natural_armor: 4, hit_die_size: 8 },
        );
        // AoN: Str 13, Con 13, +5 natural armor (Bestiary 2 p.186). Corpus
        // (`megafauna_gylptodon.json`) natural_armor: 5 -- agrees.
        out.insert(
            "megafauna_gylptodon",
            CompanionBaseStats { strength: 13, constitution: 13, natural_armor: 5, hit_die_size: 8 },
        );
        // AoN: Str 12, Con 14, +3 natural armor (Bestiary 2 p.187). Corpus
        // (`megafauna_megaloceros.json`) natural_armor: 3 -- agrees.
        out.insert(
            "megafauna_megaloceros",
            CompanionBaseStats { strength: 12, constitution: 14, natural_armor: 3, hit_die_size: 8 },
        );
        // AoN: Str 9, Con 11, +5 natural armor (Bestiary 2 p.187). Corpus
        // (`megafauna_megatherium.json`) natural_armor: 5 -- agrees.
        out.insert(
            "megafauna_megatherium",
            CompanionBaseStats { strength: 9, constitution: 11, natural_armor: 5, hit_die_size: 8 },
        );
        // AoN: Str 8, Con 11, +2 natural armor (Pathfinder RPG Core Rulebook p.54). Corpus
        // (`snake_viper.json`) natural_armor: 2 -- agrees.
        out.insert(
            "snake_viper",
            CompanionBaseStats { strength: 8, constitution: 11, natural_armor: 2, hit_die_size: 8 },
        );
        // AoN: Str 13, Con 13, +2 natural armor (Monster Codex p.128). Corpus
        // (`cave_salamander.json`) natural_armor: 2 -- agrees.
        out.insert(
            "cave_salamander",
            CompanionBaseStats { strength: 13, constitution: 13, natural_armor: 2, hit_die_size: 8 },
        );
        // AoN: Str 16, Con 13, +4 natural armor (Monster Codex p.172). Corpus
        // (`gorthek.json`) natural_armor: 4 -- agrees.
        out.insert(
            "gorthek",
            CompanionBaseStats { strength: 16, constitution: 13, natural_armor: 4, hit_die_size: 8 },
        );
        // AoN: Str 11, Con 13, +1 natural armor (Monster Codex p.128). Corpus
        // (`python_riding.json`) natural_armor: 1 -- agrees.
        out.insert(
            "python_riding",
            CompanionBaseStats { strength: 11, constitution: 13, natural_armor: 1, hit_die_size: 8 },
        );
        // AoN: Str 14, Con 17, +1 natural armor (Monster Codex p.177). Corpus
        // (`rat_riding.json`) natural_armor: 1 -- agrees.
        out.insert(
            "rat_riding",
            CompanionBaseStats { strength: 14, constitution: 17, natural_armor: 1, hit_die_size: 8 },
        );
        // AoN: Str 14, Con 12, +1 natural armor (Monster Codex p.124). Corpus
        // (`yzobu.json`) natural_armor: 1 -- agrees.
        out.insert(
            "yzobu",
            CompanionBaseStats { strength: 14, constitution: 12, natural_armor: 1, hit_die_size: 8 },
        );
        // AoN: Str 15, Con 16, +1 natural armor (Bestiary 5 p.313). Corpus
        // (`frog_father.json`) natural_armor: 1 -- agrees.
        out.insert(
            "frog_father",
            CompanionBaseStats { strength: 15, constitution: 16, natural_armor: 1, hit_die_size: 8 },
        );
        // AoN: Str 15, Con 16, +1 natural armor (Bestiary 5 p.313). Corpus
        // (`frog_goliath.json`) natural_armor: 1 -- agrees.
        out.insert(
            "frog_goliath",
            CompanionBaseStats { strength: 15, constitution: 16, natural_armor: 1, hit_die_size: 8 },
        );
        // AoN: Str 15, Con 13, +2 natural armor (Bestiary 5 p.313). Corpus
        // (`polar_bear.json`) natural_armor: 2 -- agrees.
        out.insert(
            "polar_bear",
            CompanionBaseStats { strength: 15, constitution: 13, natural_armor: 2, hit_die_size: 8 },
        );
        // AoN: Str 15, Con 13, +2 natural armor (Bestiary 5 p.313). Corpus
        // (`polar_bear_dire.json`) natural_armor: 2 -- agrees.
        out.insert(
            "polar_bear_dire",
            CompanionBaseStats { strength: 15, constitution: 13, natural_armor: 2, hit_die_size: 8 },
        );
        // AoN: Str 16, Con 16, +4 natural armor (Inner Sea Combat p.14). Corpus
        // (`griffon.json`) natural_armor: 4 -- agrees.
        out.insert(
            "griffon",
            CompanionBaseStats { strength: 16, constitution: 16, natural_armor: 4, hit_die_size: 8 },
        );
        // AoN: Str 15, Con 14, +2 natural armor (Inner Sea Combat p.14). Corpus
        // (`hippogriff.json`) natural_armor: 2 -- agrees.
        out.insert(
            "hippogriff",
            CompanionBaseStats { strength: 15, constitution: 14, natural_armor: 2, hit_die_size: 8 },
        );
        // AoN: Str 17, Con 13, +2 natural armor (Inner Sea Combat p.14). Corpus
        // (`worg.json`) natural_armor: 2 -- agrees.
        out.insert(
            "worg",
            CompanionBaseStats { strength: 17, constitution: 13, natural_armor: 2, hit_die_size: 8 },
        );
        // A `devolutionist` druid's own nature-bond text (Horror Adventures
        // p.50): "Use the stats for an ape animal companion" -- not an
        // independent stat block, the already-grounded `ape` entry's own Str
        // 13/Con 10/+1 natural armor (above), reused directly. Corpus
        // (`horror_adventures/companion_devolved_humanoid.json`)
        // `natural_armor`: 1 -- agrees with `ape`'s own value, independently
        // confirming this derivation.
        out.insert(
            "devolved_humanoid",
            CompanionBaseStats { strength: 13, constitution: 10, natural_armor: 1, hit_die_size: 8 },
        );
        out
    })
}

/// Formats a table slug (`"gulper_plant"`) as the printed species name
/// (`"Gulper Plant"`) [`ground_companion_stat_block`]'s callers need for
/// `species_display_name`, by title-casing each underscore-joined word --
/// deterministic text formatting of an already-verified slug, not a new
/// fact: every slug in [`companion_base_stat_table`] already carries its
/// own verified display name in its doc comment above, and this function's
/// own output matches each one exactly (confirmed by
/// `companion_display_name_matches_every_table_entrys_documented_name`
/// below).
pub(crate) fn companion_display_name(slug: &str) -> String {
    slug.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Grounds `species_slug`'s standalone companion stat block, exactly the
/// way [`super::ground_wolf_companion_stat_block`]/[`super::ground_horse_
/// companion_stat_block`] already do for their own two species, but
/// table-driven via [`companion_base_stat_table`] rather than one
/// hand-typed function per species. Returns `true` when `species_slug`
/// was found and grounded, `false` when it refuses -- never guesses --
/// because no verified base-stat entry exists yet for that species.
///
/// Reuses this module's parent's own universal companion-advancement math
/// (`super::animal_companion_table_index`/`_natural_armor_bonus`/
/// `_stat_bonus`/`_hit_points`), confirmed species-agnostic by every
/// candidate record this cycle checked sharing the identical
/// `MONSTERCLASS:Companion:2` progression tag, so a new species needs
/// only its own base-ability-score row here, never a second copy of the
/// advancement math.
pub(crate) fn ground_companion_stat_block(
    species_slug: &str,
    id_prefix: &str,
    owner_class_label: &str,
    species_display_name: &str,
    companion_level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) -> bool {
    let Some(stats) = companion_base_stat_table().get(species_slug) else {
        return false;
    };
    let companion_hd =
        super::ANIMAL_COMPANION_HIT_DICE_BY_MASTER_LEVEL[super::animal_companion_table_index(companion_level)];
    let companion_hd_value = i16::from(companion_hd);
    let companion_base_attack_bonus = companion_hd_value * 3 / 4;
    let companion_fort_ref_save = companion_hd_value / 2 + 2;
    let companion_will_save = companion_hd_value / 3;
    let strength_bonus = super::animal_companion_stat_bonus(companion_level);
    let strength_score = stats.strength + strength_bonus;
    let natural_armor = stats.natural_armor + super::animal_companion_natural_armor_bonus(companion_level);
    let companion_armor_class = 10 + natural_armor;
    let strength_modifier = super::ability_modifier(strength_score);
    let constitution_modifier = super::ability_modifier(stats.constitution);
    let companion_attack_bonus = companion_base_attack_bonus + strength_modifier;
    let companion_hp =
        super::animal_companion_hit_points(companion_hd, stats.hit_die_size, constitution_modifier);

    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.{species_slug}_stat_block"),
        value: 0,
        detail: format!(
            "{owner_class_label} level {companion_level} animal companion, {species_display_name}: \
             a wholly separate creature with its own combat statistics -- none of the values below \
             are ever applied to the {owner_class_label}'s own integrated totals. Base ability \
             scores: Str {}, Con {}. This is a bounded recognition record only (+0); the \
             companion's own flat stat values are grounded separately as standalone explanation \
             records below",
            stats.strength, stats.constitution
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.base_attack_bonus"),
        value: companion_attack_bonus,
        detail: format!(
            "{species_display_name} companion base attack bonus at {companion_hd} HD (PF1 Core \
             Rulebook Animal Companion Base Statistics: HD*3/4 = {companion_base_attack_bonus}) + \
             Strength modifier ({strength_modifier:+}, Str {strength_score} = base {} + \
             {strength_bonus} from the companion class's own floor(master level/3) Strength/\
             Dexterity advance) = {companion_attack_bonus}. Standalone record; the companion is a \
             separate creature, not integrated into the {owner_class_label}'s own combat totals",
            stats.strength
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.base_save.fortitude"),
        value: companion_fort_ref_save,
        detail: format!(
            "{species_display_name} companion base Fortitude save at {companion_hd} HD (PF1 Core \
             Rulebook Animal Companion Base Statistics: classlevel/2+2 = {companion_fort_ref_save}). \
             Standalone record; not the {owner_class_label}'s own save"
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.base_save.reflex"),
        value: companion_fort_ref_save,
        detail: format!(
            "{species_display_name} companion base Reflex save at {companion_hd} HD (PF1 Core \
             Rulebook Animal Companion Base Statistics: classlevel/2+2 = {companion_fort_ref_save}). \
             Standalone record; not the {owner_class_label}'s own save"
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.base_save.will"),
        value: companion_will_save,
        detail: format!(
            "{species_display_name} companion base Will save at {companion_hd} HD (PF1 Core \
             Rulebook Animal Companion Base Statistics: classlevel/3 = {companion_will_save}). \
             Standalone record; not the {owner_class_label}'s own save"
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.armor_class"),
        value: companion_armor_class,
        detail: format!(
            "{species_display_name} companion armor class: base 10 + natural armor (+{natural_armor} \
             = the species' own base +{} plus +{} from the companion class's own \
             2*floor(master level/3) natural-armor advance) = {companion_armor_class}. Standalone \
             record; Dexterity's own contribution to the companion's AC is not grounded",
            stats.natural_armor,
            super::animal_companion_natural_armor_bonus(companion_level)
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.hit_points"),
        value: companion_hp,
        detail: format!(
            "{species_display_name} companion hit points at {companion_hd} HD (d{}): maximized \
             first Hit Die plus average for each of the remaining {} (this codebase's own \
             established HP idiom, durability.rs's compute_max_hp), each plus the companion's \
             Constitution modifier ({constitution_modifier:+}, Con {}) = {companion_hp}",
            stats.hit_die_size,
            companion_hd.saturating_sub(1),
            stats.constitution
        ),
    });
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_wolf_reproduces_the_existing_hand_authored_wolf_function() {
        // Same inputs `animal_companion_stat_block_tests` (this module's
        // parent) already exercises for `ground_wolf_companion_stat_
        // block`, at master level 1 -- proving this table-driven function
        // is not a second, independently-drifting implementation.
        let mut generic = Vec::new();
        let grounded =
            ground_companion_stat_block("wolf", "companion", "Druid", "Wolf", 1, &mut generic);
        assert!(grounded, "wolf must be found in the table");

        let mut hand_authored = Vec::new();
        super::super::ground_wolf_companion_stat_block("companion", "Druid", 1, &mut hand_authored);

        let generic_values: BTreeMap<&str, i16> =
            generic.iter().map(|e| (e.id.rsplit('.').next().unwrap(), e.value)).collect();
        let hand_values: BTreeMap<&str, i16> =
            hand_authored.iter().map(|e| (e.id.rsplit('.').next().unwrap(), e.value)).collect();
        for (suffix, hand_value) in &hand_values {
            // `wolf_stat_block`: the recognition-only +0 record, different id
            // shape between the two (only its value, +0 both sides, would
            // match anyway). `bite_attack`: the species' own primary
            // natural-attack damage bonus -- deliberately NOT generalized
            // here (this module's own doc: "grounds only the fields with a
            // live downstream reader -- attack bonus, saves, AC, HP"), since
            // a natural attack's own shape (bite/hoof/vine/claw, and which
            // multiplier applies) is genuinely per-species and would need
            // its own verified data, not a formula this generic function may
            // assume.
            if *suffix == "wolf_stat_block" || *suffix == "bite_attack" {
                continue;
            }
            assert_eq!(
                generic_values.get(suffix),
                Some(hand_value),
                "generic ground_companion_stat_block must reproduce {suffix} byte-for-byte"
            );
        }
    }

    #[test]
    fn generic_horse_reproduces_the_existing_hand_authored_horse_function() {
        let mut generic = Vec::new();
        let grounded =
            ground_companion_stat_block("horse", "mount", "Cavalier", "Horse", 1, &mut generic);
        assert!(grounded, "horse must be found in the table");

        let mut hand_authored = Vec::new();
        super::super::ground_horse_companion_stat_block("mount", "Cavalier", 1, &mut hand_authored);

        let generic_values: BTreeMap<&str, i16> =
            generic.iter().map(|e| (e.id.rsplit('.').next().unwrap(), e.value)).collect();
        let hand_values: BTreeMap<&str, i16> =
            hand_authored.iter().map(|e| (e.id.rsplit('.').next().unwrap(), e.value)).collect();
        for (suffix, hand_value) in &hand_values {
            // See the wolf test's own comment for why the natural-attack
            // damage bonus (here, the Horse's hoof attack) is excluded.
            if *suffix == "horse_stat_block" || *suffix == "hoof_attack" {
                continue;
            }
            assert_eq!(
                generic_values.get(suffix),
                Some(hand_value),
                "generic ground_companion_stat_block must reproduce {suffix} byte-for-byte"
            );
        }
    }

    #[test]
    fn gulper_plant_grounds_a_real_new_species_at_master_level_1() {
        // Cycle 9 correction (`§17a`): the table's own `strength`/
        // `constitution` fields are the species' PRINTED 1st-level total
        // (Str 12, Con 13 for Gulper Plant per aonprd.com's own "Starting
        // Statistics" line), not that total with the corpus's own
        // `BONUS:STAT` delta backed out -- `ground_companion_stat_block`
        // adds only the universal `animal_companion_stat_bonus` level
        // advance on top, never the corpus's per-species delta (see this
        // module's own cycle 9 addendum above for the full derivation and
        // why this was wrong for all 44 previously-added species with a
        // nonzero delta). Str 12 base + 0 advancement (floor(1/3)=0) =
        // Str 12, modifier +1; base attack bonus at 2 HD (master level 1's
        // own HD per the universal table) = 2*3/4 = 1; +1 Str modifier = 2.
        let mut explanations = Vec::new();
        let grounded = ground_companion_stat_block(
            "gulper_plant",
            "companion",
            "Druid",
            "Gulper Plant",
            1,
            &mut explanations,
        );
        assert!(grounded, "gulper_plant must be found in the table");
        let by_id: BTreeMap<&str, i16> = explanations.iter().map(|e| (e.id.as_str(), e.value)).collect();
        assert_eq!(by_id.get("companion.base_attack_bonus"), Some(&2));
    }

    #[test]
    fn gulper_plant_base_saves_and_armor_class_at_master_level_1() {
        let mut explanations = Vec::new();
        assert!(ground_companion_stat_block(
            "gulper_plant",
            "companion",
            "Druid",
            "Gulper Plant",
            1,
            &mut explanations,
        ));
        let by_id: BTreeMap<&str, i16> = explanations.iter().map(|e| (e.id.as_str(), e.value)).collect();
        assert_eq!(by_id.get("companion.base_save.fortitude"), Some(&3));
        assert_eq!(by_id.get("companion.base_save.reflex"), Some(&3));
        assert_eq!(by_id.get("companion.base_save.will"), Some(&0));
        // AC = 10 + natural armor (1 + 0 advancement) = 11.
        assert_eq!(by_id.get("companion.armor_class"), Some(&11));
        // HP at 2 HD, d8, Con modifier +1 (cycle 9 correction: Con 13, not
        // the delta-backed-out 11): maximized first (8+1=9) + average
        // second (durability::average_hit_die_value(8) = 5, +1 = 6, the
        // PF1 round-up convention) = 15.
        assert_eq!(by_id.get("companion.hit_points"), Some(&15));
    }

    #[test]
    fn an_unknown_species_slug_refuses_rather_than_guesses() {
        // `decisions.md §1a`/the same posture `class_feature_grant_
        // consumer.rs`'s own module doc names throughout: no verified
        // base-stat entry, no grounded record -- ever. Cycle 13 grounded
        // `griffon` (this test's own slug through cycle 12) as part of
        // closing the full 196-record population, so the refusal example
        // moves to a slug that is not, and never has been, a real PF1
        // companion species -- proving the refusal path still exists and
        // still works now that the table has no real gaps left to exercise
        // it with.
        let mut explanations = Vec::new();
        let grounded = ground_companion_stat_block(
            "not_a_real_companion_species",
            "companion",
            "Druid",
            "Not A Real Companion Species",
            1,
            &mut explanations,
        );
        assert!(!grounded, "an ungrounded species must refuse, not fabricate a stat block");
        assert!(explanations.is_empty());
    }

    #[test]
    fn companion_display_name_matches_every_table_entrys_documented_name() {
        // Confirms the deterministic title-case formatter used by row 20
        // cycle 7's own dispatch wiring (`ground_selected_companion_or_
        // default`, this module's parent) never drifts from the printed
        // species names this module's own doc comments already verified.
        for (slug, expected) in [
            ("wolf", "Wolf"),
            ("horse", "Horse"),
            ("gulper_plant", "Gulper Plant"),
            ("allosaurus", "Allosaurus"),
            ("ankylosaurus", "Ankylosaurus"),
            ("pteranodon", "Pteranodon"),
            ("deinonychus", "Deinonychus"),
            ("velociraptor", "Velociraptor"),
            ("triceratops", "Triceratops"),
            ("tyrannosaurus", "Tyrannosaurus"),
            ("amargasaurus", "Amargasaurus"),
            ("brachiosaurus", "Brachiosaurus"),
            ("elasmosaurus", "Elasmosaurus"),
            ("stegosaurus", "Stegosaurus"),
            ("dimetrodon", "Dimetrodon"),
            ("iguanodon", "Iguanodon"),
            ("spinosaurus", "Spinosaurus"),
            ("dimorphodon", "Dimorphodon"),
            ("diplodocus", "Diplodocus"),
            ("styracosaurus", "Styracosaurus"),
            ("ceratosaurus", "Ceratosaurus"),
            ("plesiosaurus", "Plesiosaurus"),
            ("therizinosaurus", "Therizinosaurus"),
            ("troodon", "Troodon"),
            ("giganotosaurus", "Giganotosaurus"),
            ("kentrosaurus", "Kentrosaurus"),
            ("quetzalcoatlus", "Quetzalcoatlus"),
            ("parasaurolophus", "Parasaurolophus"),
            ("tylosaurus", "Tylosaurus"),
            ("eel_giant_moray", "Eel Giant Moray"),
            ("octopus", "Octopus"),
            ("squid", "Squid"),
            ("cameroceras", "Cameroceras"),
            ("dunkleosteus", "Dunkleosteus"),
            ("shark", "Shark"),
            ("hippocampus", "Hippocampus"),
            ("crab_giant", "Crab Giant"),
            ("anglerfish", "Anglerfish"),
            ("armorfish", "Armorfish"),
            ("hammerhead_shark", "Hammerhead Shark"),
            ("squid_giant", "Squid Giant"),
            ("corpse_eater_fungus", "Corpse Eater Fungus"),
            ("creeping_puffball", "Creeping Puffball"),
            ("hunting_cactus", "Hunting Cactus"),
            ("rash_creeper", "Rash Creeper"),
            ("slithering_sundew", "Slithering Sundew"),
            ("snapping_flytrap", "Snapping Flytrap"),
            ("sniper_cactus", "Sniper Cactus"),
            ("ape", "Ape"),
            ("chimpanzee", "Chimpanzee"),
            ("devil_monkey", "Devil Monkey"),
            ("megaprimatus", "Megaprimatus"),
            ("badger_wolverine", "Badger Wolverine"),
            ("bear", "Bear"),
            ("bird_eagle", "Bird Eagle"),
            ("bird_hawk", "Bird Hawk"),
            ("bird_owl", "Bird Owl"),
            ("boar", "Boar"),
            ("camel", "Camel"),
            ("cat_big_lion", "Cat Big Lion"),
            ("cat_big_tiger", "Cat Big Tiger"),
            ("cat_small_cheetah", "Cat Small Cheetah"),
            ("cat_small_leopard", "Cat Small Leopard"),
            ("crocodile_alligator", "Crocodile Alligator"),
            ("dog", "Dog"),
            ("pony", "Pony"),
            ("dire_rat", "Dire Rat"),
            ("herd_animal_ram", "Herd Animal Ram"),
            ("hippopotamus", "Hippopotamus"),
            ("primate_baboon", "Primate Baboon"),
            ("ray_manta", "Ray Manta"),
            ("ray_stingray", "Ray Stingray"),
            ("turtle_giant_snapping", "Turtle Giant Snapping"),
            ("snake_constrictor", "Snake Constrictor"),
            ("pachycephalosaurus", "Pachycephalosaurus"),
            ("ornithomimosaur", "Ornithomimosaur"),
            ("antelope", "Antelope"),
            ("archelon", "Archelon"),
            ("axe_beak", "Axe Beak"),
            ("baluchitherium", "Baluchitherium"),
            ("basilosaurus", "Basilosaurus"),
            ("elk", "Elk"),
            ("giant_chameleon", "Giant Chameleon"),
            ("giant_gecko", "Giant Gecko"),
            ("giant_vulture", "Giant Vulture"),
            ("giant_vulture_monster_codex", "Giant Vulture Monster Codex"),
            ("kangaroo", "Kangaroo"),
            ("megalania", "Megalania"),
            ("thylacine", "Thylacine"),
            ("weasel_giant", "Weasel Giant"),
            ("giraffe", "Giraffe"),
            ("seahorse", "Seahorse"),
            ("stag", "Stag"),
            ("tortoise", "Tortoise"),
            ("trumpeter_swan", "Trumpeter Swan"),
            ("walrus", "Walrus"),
            ("whale_blue", "Whale Blue"),
            ("chalicotherium", "Chalicotherium"),
            ("digmaul", "Digmaul"),
            ("kaprosuchus", "Kaprosuchus"),
            ("moa", "Moa"),
            ("narwhal", "Narwhal"),
            ("uintatherium", "Uintatherium"),
            ("wolliped", "Wolliped"),
            ("archaeopteryx", "Archaeopteryx"),
            ("assassin_bug_giant", "Assassin Bug Giant"),
            ("bustard", "Bustard"),
            ("capybara", "Capybara"),
            ("caterpillar_giant", "Caterpillar Giant"),
            ("cattle", "Cattle"),
            ("cockroach_giant", "Cockroach Giant"),
            ("dragonfly_giant", "Dragonfly Giant"),
            ("eohippus", "Eohippus"),
            ("eurypterid", "Eurypterid"),
            ("falcon", "Falcon"),
            ("frilled_lizard_giant", "Frilled Lizard Giant"),
            ("grizzly_bear", "Grizzly Bear"),
            ("llama", "Llama"),
            ("locust_giant", "Locust Giant"),
            ("mantis_shrimp_giant", "Mantis Shrimp Giant"),
            ("marsupial_devil", "Marsupial Devil"),
            ("marsupial_lion", "Marsupial Lion"),
            ("mole_giant", "Mole Giant"),
            ("moose", "Moose"),
            ("mosquito_giant", "Mosquito Giant"),
            ("owl_giant", "Owl Giant"),
            ("panda", "Panda"),
            ("porcupine_giant", "Porcupine Giant"),
            ("prionosuchus", "Prionosuchus"),
            ("reef_snake", "Reef Snake"),
            ("reindeer", "Reindeer"),
            ("saber_toothed_cat", "Saber Toothed Cat"),
            ("salamander_giant", "Salamander Giant"),
            ("skunk_giant", "Skunk Giant"),
            ("solifugid_giant", "Solifugid Giant"),
            ("spider_web_tyrant", "Spider Web Tyrant"),
            ("spitting_cobra", "Spitting Cobra"),
            ("termite_giant", "Termite Giant"),
            ("whiptail_centipede_giant", "Whiptail Centipede Giant"),
            ("wolfdog", "Wolfdog"),
            ("yak", "Yak"),
            ("zebra", "Zebra"),
            // Cycle 13's own 54 new entries, closing the full population.
            ("bat_dire", "Bat Dire"),
            ("dolphin", "Dolphin"),
            ("dolphin_orca", "Dolphin Orca"),
            ("eel_electric", "Eel Electric"),
            ("elephant", "Elephant"),
            ("elephant_mastodon", "Elephant Mastodon"),
            ("frog_giant", "Frog Giant"),
            ("goblin_dog", "Goblin Dog"),
            ("herd_animal_aurochs", "Herd Animal Aurochs"),
            ("herd_animal_bison", "Herd Animal Bison"),
            ("hyena", "Hyena"),
            ("lizard_monitor", "Lizard Monitor"),
            ("rhinoceros", "Rhinoceros"),
            ("roc", "Roc"),
            ("ant_giant", "Ant Giant"),
            ("beetle_giant", "Beetle Giant"),
            ("centipede_giant", "Centipede Giant"),
            ("leech_giant", "Leech Giant"),
            ("mantis_giant", "Mantis Giant"),
            ("scorpion_giant", "Scorpion Giant"),
            ("slug_giant", "Slug Giant"),
            ("spider_giant", "Spider Giant"),
            ("wasp_giant", "Wasp Giant"),
            ("carnivorous_flower", "Carnivorous Flower"),
            ("crawling_vine", "Crawling Vine"),
            ("puffball", "Puffball"),
            ("sapling_treant", "Sapling Treant"),
            ("brute_steed_camel", "Brute Steed Camel"),
            ("brute_steed_horse", "Brute Steed Horse"),
            ("brontotherium", "Brontotherium"),
            ("deinotherium", "Deinotherium"),
            ("elasmotherium", "Elasmotherium"),
            ("mokele_mbembe", "Mokele Mbembe"),
            ("raven_giant", "Raven Giant"),
            ("titanoboa", "Titanoboa"),
            ("gar", "Gar"),
            ("megafauna_arsinoitherium", "Megafauna Arsinoitherium"),
            ("megafauna_gylptodon", "Megafauna Gylptodon"),
            ("megafauna_megaloceros", "Megafauna Megaloceros"),
            ("megafauna_megatherium", "Megafauna Megatherium"),
            ("snake_viper", "Snake Viper"),
            ("cave_salamander", "Cave Salamander"),
            ("gorthek", "Gorthek"),
            ("python_riding", "Python Riding"),
            ("rat_riding", "Rat Riding"),
            ("yzobu", "Yzobu"),
            ("frog_father", "Frog Father"),
            ("frog_goliath", "Frog Goliath"),
            ("polar_bear", "Polar Bear"),
            ("polar_bear_dire", "Polar Bear Dire"),
            ("griffon", "Griffon"),
            ("hippogriff", "Hippogriff"),
            ("worg", "Worg"),
            ("devolved_humanoid", "Devolved Humanoid"),
        ] {
            assert_eq!(companion_display_name(slug), expected);
        }
    }

    #[test]
    fn griffon_base_attack_bonus_and_armor_class_at_master_level_1() {
        // Cycle 13: `griffon` was this module's own long-standing negative-
        // test example (cycle 5 through 12) precisely because it had no
        // verified entry. Now grounded (Inner Sea Combat p.14: Str 16, Con
        // 16, +4 natural armor), this proves it the same positive way
        // `gulper_plant`'s own tests above do: base attack bonus at 2 HD =
        // 2*3/4 = 1, +3 Str modifier (Str 16) = 4. AC = 10 + natural armor
        // (4 + 0 advancement) = 14.
        let mut explanations = Vec::new();
        let grounded = ground_companion_stat_block(
            "griffon",
            "companion",
            "Druid",
            "Griffon",
            1,
            &mut explanations,
        );
        assert!(grounded, "griffon must be found in the table now that cycle 13 grounded it");
        let by_id: BTreeMap<&str, i16> = explanations.iter().map(|e| (e.id.as_str(), e.value)).collect();
        assert_eq!(by_id.get("companion.base_attack_bonus"), Some(&4));
        assert_eq!(by_id.get("companion.armor_class"), Some(&14));
    }

    #[test]
    fn only_one_hundred_and_ninety_six_of_the_corpus_s_196_racetype_companion_records_have_a_base_stat_entry(
    ) {
        // Named exactly, not rounded away (§16/§17a): the honest residual
        // this cycle leaves for the next one. Row 20 cycle 8 re-derived the
        // true base-race `RACETYPE:Companion` population directly from
        // `data/corpus/*/companion/*.json` (filtering to records that
        // actually carry `MONSTERCLASS:Companion:*` and excluding the
        // separate "Companion Advancement (...)" ability records, which
        // are not base-race entries at all) and found 196, not the 213 a
        // prior cycle's raw-`.lst`-line count assumed -- that count did
        // not exclude the "Companion Advancement" records, which share the
        // `companion_*` filename prefix but carry no `monster_class`.
        // Cycle 8 added 12 `Aquatic`, 7 `PlantCompanion` (the ones not
        // already grounded by `gulper_plant`), and 4 `AnimalCompanion
        // Primate` species -- 23 in total -- on top of cycle 5's wolf/
        // horse/gulper_plant and cycles 6-7's 26 of 28 `AnimalCompanion
        // Dinosaur` species (52 = 29 + 23). The `ConstructCompanion`
        // bucket a prior cycle's brief named as size 3 does not exist in
        // this corpus at all: those 3 raw `RACESUBTYPE:ConstructCompanion`
        // records live in `path_of_iron/poi_races_companion.lst`, a
        // third-party (Ascension Games) book this repo's corpus has never
        // ingested (`data/corpus/` has no `path_of_iron` entry at all) --
        // out of the 196-record population this table targets, not merely
        // unverified within it. Cycle 9 found and fixed a correctness
        // defect in 44 of those 52 (see the module's cycle-9 addendum) but
        // added no new species. Cycle 10 added the first 22 of the 142
        // untagged (`RACESUBTYPE`-less) `core_rulebook` records (52 + 22 =
        // 74). Cycle 11 resolved both named dinosaur-bucket refusals
        // (`pachycephalosaurus`, `ornithomimosaur` -- now 28 of 28
        // `AnimalCompanionDinosaur` species grounded) and added 28 of the
        // 120 untagged records outside `core_rulebook` (`bestiary_3`,
        // `bestiary_4`, `bestiary_5`, `monster_codex`; `giant_vulture` is a
        // verbatim reprint across `bestiary_3` and `monster_codex`, two
        // corpus records, two table entries) -- 74 + 2 + 28 = 104. 92
        // remain ungrounded, all untagged, spanning `ultimate_wilderness`
        // (38), `beastiary` (14), `bestiary_3`/`bestiary_5` (6 each
        // residual), `ultimate_magic` (9), `bestiary_4`/`bestiary_6`/
        // `advanced_race_guide`/`core_rulebook`/`monster_codex` (residuals),
        // `inner_sea_combat` (3), and `horror_adventures` (1). Cycle 12
        // added all 38 `ultimate_wilderness` records (104 -> 142), leaving
        // 54 across the remaining nine books. Cycle 13 re-derived that
        // residual directly (`§17a`, this module's own cycle-13 addendum
        // above) -- 54 exactly, confirmed once the narrower `companion_*`
        // filename glob a first pass used was replaced with a full
        // directory glob catching `advanced_race_guide`'s two `brute_
        // steed_*` files -- and grounded every one of them: `beastiary`
        // (14), `ultimate_magic` (9), `advanced_race_guide` (6),
        // `bestiary_6` (6), `core_rulebook` (6), `monster_codex` (5),
        // `bestiary_5` (4), `inner_sea_combat` (3), `horror_adventures`
        // (1) -- 142 + 54 = 196. The full `RACETYPE:Companion` base-race
        // population is now closed: zero untagged records remain in any
        // book (`decisions.md §27b`/`§1a`).
        assert_eq!(
            companion_base_stat_table().len(),
            196,
            "wolf, horse, gulper_plant, all 28 AnimalCompanionDinosaur species, the full \
             Aquatic/PlantCompanion/AnimalCompanionPrimate buckets (23 more), 22 of the 142 \
             untagged core_rulebook records, 28 more untagged records outside core_rulebook, \
             cycle 12's 38 `ultimate_wilderness` untagged records, and cycle 13's final 54 \
             untagged records spanning beastiary/ultimate_magic/advanced_race_guide/bestiary_6/ \
             core_rulebook/monster_codex/bestiary_5/inner_sea_combat/horror_adventures -- every \
             real base-race RACETYPE:Companion corpus record now has a verified \
             base-ability-score entry"
        );
    }

    /// Row 20 cycle 12's own positive counterpart: all 38 `ultimate_wilderness`
    /// untagged records, the single largest remaining bucket cycle 11 named,
    /// pinning the exact base ability scores this cycle's doc comments derive.
    #[test]
    fn the_thirty_eight_cycle_twelve_ultimate_wilderness_companions_ground_their_own_verified_base_scores(
    ) {
        for (slug, display, expected_str, expected_con, expected_natural_armor) in [
            ("archaeopteryx", "Archaeopteryx", 8i16, 12i16, 1i16),
            ("assassin_bug_giant", "Assassin Bug Giant", 13, 13, 2),
            ("bustard", "Bustard", 13, 13, 2),
            ("capybara", "Capybara", 12, 12, 1),
            ("caterpillar_giant", "Caterpillar Giant", 14, 13, 2),
            ("cattle", "Cattle", 14, 12, 2),
            ("cockroach_giant", "Cockroach Giant", 9, 17, 1),
            ("dragonfly_giant", "Dragonfly Giant", 13, 12, 2),
            ("eohippus", "Eohippus", 11, 12, 2),
            ("eurypterid", "Eurypterid", 10, 12, 2),
            ("falcon", "Falcon", 10, 11, 1),
            ("frilled_lizard_giant", "Frilled Lizard Giant", 13, 13, 4),
            ("grizzly_bear", "Grizzly Bear", 17, 13, 1),
            ("llama", "Llama", 11, 12, 1),
            ("locust_giant", "Locust Giant", 12, 11, 3),
            ("mantis_shrimp_giant", "Mantis Shrimp Giant", 12, 14, 3),
            ("marsupial_devil", "Marsupial Devil", 15, 12, 1),
            ("marsupial_lion", "Marsupial Lion", 12, 13, 1),
            ("mole_giant", "Mole Giant", 12, 13, 1),
            ("moose", "Moose", 12, 14, 2),
            ("mosquito_giant", "Mosquito Giant", 14, 15, 1),
            ("owl_giant", "Owl Giant", 10, 13, 2),
            ("panda", "Panda", 13, 14, 2),
            ("porcupine_giant", "Porcupine Giant", 14, 15, 1),
            ("prionosuchus", "Prionosuchus", 13, 15, 2),
            ("reef_snake", "Reef Snake", 10, 12, 2),
            ("reindeer", "Reindeer", 13, 13, 2),
            ("saber_toothed_cat", "Saber Toothed Cat", 15, 13, 1),
            ("salamander_giant", "Salamander Giant", 13, 15, 3),
            ("skunk_giant", "Skunk Giant", 9, 13, 1),
            ("solifugid_giant", "Solifugid Giant", 12, 15, 1),
            ("spider_web_tyrant", "Spider Web Tyrant", 10, 10, 1),
            ("spitting_cobra", "Spitting Cobra", 8, 11, 2),
            ("termite_giant", "Termite Giant", 12, 12, 2),
            ("whiptail_centipede_giant", "Whiptail Centipede Giant", 9, 13, 3),
            ("wolfdog", "Wolfdog", 15, 13, 2),
            ("yak", "Yak", 14, 12, 4),
            ("zebra", "Zebra", 14, 13, 2),
        ] {
            let mut explanations = Vec::new();
            let grounded = ground_companion_stat_block(
                slug,
                "companion",
                "Druid",
                display,
                1,
                &mut explanations,
            );
            assert!(grounded, "{slug} must ground a real stat block");
            let detail = &explanations
                .iter()
                .find(|e| e.id == format!("companion.{slug}_stat_block"))
                .unwrap_or_else(|| panic!("expected a companion.{slug}_stat_block record"))
                .detail;
            assert!(
                detail.contains(&format!("Str {expected_str}")),
                "{slug} expected base Str {expected_str} in detail: {detail}"
            );
            assert!(
                detail.contains(&format!("Con {expected_con}")),
                "{slug} expected base Con {expected_con} in detail: {detail}"
            );
            let table = companion_base_stat_table();
            let stats = table.get(slug).expect("entry must exist");
            assert_eq!(stats.natural_armor, expected_natural_armor, "{slug} natural armor");
            assert_eq!(stats.hit_die_size, 8, "{slug} hit die size is always d8 per the companion mechanic");
        }
    }

    /// Row 20 cycle 8's own positive counterpart, pinning the exact base
    /// ability scores this cycle's doc comments derive for all 23 new
    /// entries across the three tagged buckets `Aquatic`,
    /// `PlantCompanion`, and `AnimalCompanionPrimate`.
    #[test]
    fn the_twenty_three_cycle_eight_aquatic_plant_and_primate_companions_ground_their_own_verified_base_scores(
    ) {
        for (slug, display, expected_str, expected_con, expected_natural_armor) in [
            ("eel_giant_moray", "Eel Giant Moray", 14i16, 12i16, 5i16),
            ("octopus", "Octopus", 12, 14, 1),
            ("squid", "Squid", 14, 11, 1),
            ("cameroceras", "Cameroceras", 14, 11, 1),
            ("dunkleosteus", "Dunkleosteus", 14, 10, 4),
            ("shark", "Shark", 13, 15, 4),
            ("hippocampus", "Hippocampus", 16, 15, 4),
            ("crab_giant", "Crab Giant", 13, 13, 5),
            ("anglerfish", "Anglerfish", 13, 12, 1),
            ("armorfish", "Armorfish", 13, 15, 6),
            ("hammerhead_shark", "Hammerhead Shark", 13, 12, 3),
            ("squid_giant", "Squid Giant", 12, 13, 1),
            ("corpse_eater_fungus", "Corpse Eater Fungus", 14, 12, 2),
            ("creeping_puffball", "Creeping Puffball", 12, 14, 1),
            ("hunting_cactus", "Hunting Cactus", 14, 17, 3),
            ("rash_creeper", "Rash Creeper", 10, 13, 1),
            ("slithering_sundew", "Slithering Sundew", 14, 13, 1),
            ("snapping_flytrap", "Snapping Flytrap", 12, 14, 2),
            ("sniper_cactus", "Sniper Cactus", 10, 14, 2),
            ("ape", "Ape", 13, 10, 1),
            ("chimpanzee", "Chimpanzee", 13, 12, 1),
            ("devil_monkey", "Devil Monkey", 15, 8, 3),
            ("megaprimatus", "Megaprimatus", 13, 10, 1),
        ] {
            let mut explanations = Vec::new();
            let grounded = ground_companion_stat_block(
                slug,
                "companion",
                "Druid",
                display,
                1,
                &mut explanations,
            );
            assert!(grounded, "{slug} must ground a real stat block");
            let detail = &explanations
                .iter()
                .find(|e| e.id == format!("companion.{slug}_stat_block"))
                .unwrap_or_else(|| panic!("expected a companion.{slug}_stat_block record"))
                .detail;
            assert!(
                detail.contains(&format!("Str {expected_str}")),
                "{slug} expected base Str {expected_str} in detail: {detail}"
            );
            assert!(
                detail.contains(&format!("Con {expected_con}")),
                "{slug} expected base Con {expected_con} in detail: {detail}"
            );
            let table = companion_base_stat_table();
            let stats = table.get(slug).expect("entry must exist");
            assert_eq!(stats.natural_armor, expected_natural_armor, "{slug} natural armor");
            assert_eq!(stats.hit_die_size, 8, "{slug} hit die size is always d8 per the companion mechanic");
        }
    }

    /// Row 20 cycle 10's own positive counterpart, pinning the printed AoN
    /// base ability scores this cycle grounds DIRECTLY for the first 22 of
    /// the 142 untagged `core_rulebook` companion records -- the corrected
    /// methodology cycle 9's own addendum established (no per-species
    /// `BONUS:STAT` delta backed out).
    #[test]
    fn the_twenty_two_cycle_ten_untagged_core_rulebook_companions_ground_their_own_verified_base_scores(
    ) {
        for (slug, display, expected_str, expected_con, expected_natural_armor) in [
            ("badger_wolverine", "Badger Wolverine", 10i16, 15i16, 2i16),
            ("bear", "Bear", 15, 13, 2),
            ("bird_eagle", "Bird Eagle", 10, 12, 1),
            ("bird_hawk", "Bird Hawk", 10, 12, 1),
            ("bird_owl", "Bird Owl", 10, 12, 1),
            ("boar", "Boar", 13, 15, 6),
            ("camel", "Camel", 18, 14, 1),
            ("cat_big_lion", "Cat Big Lion", 13, 13, 1),
            ("cat_big_tiger", "Cat Big Tiger", 13, 13, 1),
            ("cat_small_cheetah", "Cat Small Cheetah", 12, 13, 1),
            ("cat_small_leopard", "Cat Small Leopard", 12, 13, 1),
            ("crocodile_alligator", "Crocodile Alligator", 15, 15, 4),
            ("dog", "Dog", 13, 15, 2),
            ("pony", "Pony", 13, 12, 2),
            ("dire_rat", "Dire Rat", 10, 12, 0),
            ("herd_animal_ram", "Herd Animal Ram", 10, 11, 1),
            ("hippopotamus", "Hippopotamus", 11, 12, 6),
            ("primate_baboon", "Primate Baboon", 12, 12, 0),
            ("ray_manta", "Ray Manta", 8, 11, 1),
            ("ray_stingray", "Ray Stingray", 6, 13, 0),
            ("turtle_giant_snapping", "Turtle Giant Snapping", 8, 9, 10),
            ("snake_constrictor", "Snake Constrictor", 15, 13, 2),
        ] {
            let mut explanations = Vec::new();
            let grounded = ground_companion_stat_block(
                slug,
                "companion",
                "Druid",
                display,
                1,
                &mut explanations,
            );
            assert!(grounded, "{slug} must ground a real stat block");
            let detail = &explanations
                .iter()
                .find(|e| e.id == format!("companion.{slug}_stat_block"))
                .unwrap_or_else(|| panic!("expected a companion.{slug}_stat_block record"))
                .detail;
            assert!(
                detail.contains(&format!("Str {expected_str}")),
                "{slug} expected base Str {expected_str} in detail: {detail}"
            );
            assert!(
                detail.contains(&format!("Con {expected_con}")),
                "{slug} expected base Con {expected_con} in detail: {detail}"
            );
            let table = companion_base_stat_table();
            let stats = table.get(slug).expect("entry must exist");
            assert_eq!(stats.natural_armor, expected_natural_armor, "{slug} natural armor");
            assert_eq!(stats.hit_die_size, 8, "{slug} hit die size is always d8 per the companion mechanic");
        }
    }

    /// Row 20 cycle 11's own positive counterpart: the two resolved
    /// dinosaur-bucket refusals plus the 28 untagged records added outside
    /// `core_rulebook`, pinning the exact base ability scores this cycle's
    /// doc comments derive.
    #[test]
    fn the_thirty_cycle_eleven_companions_ground_their_own_verified_base_scores() {
        for (slug, display, expected_str, expected_con, expected_natural_armor) in [
            ("pachycephalosaurus", "Pachycephalosaurus", 15i16, 13i16, 3i16),
            ("ornithomimosaur", "Ornithomimosaur", 11, 12, 1),
            ("antelope", "Antelope", 10, 14, 1),
            ("archelon", "Archelon", 8, 9, 10),
            ("axe_beak", "Axe Beak", 10, 12, 0),
            ("baluchitherium", "Baluchitherium", 14, 15, 4),
            ("basilosaurus", "Basilosaurus", 11, 12, 1),
            ("elk", "Elk", 12, 14, 1),
            ("giant_chameleon", "Giant Chameleon", 12, 14, 0),
            ("giant_gecko", "Giant Gecko", 11, 12, 0),
            ("giant_vulture", "Giant Vulture", 12, 14, 2),
            ("giant_vulture_monster_codex", "Giant Vulture Monster Codex", 12, 14, 2),
            ("kangaroo", "Kangaroo", 12, 13, 0),
            ("megalania", "Megalania", 13, 12, 1),
            ("thylacine", "Thylacine", 12, 16, 0),
            ("weasel_giant", "Weasel Giant", 10, 13, 1),
            ("giraffe", "Giraffe", 16, 12, 2),
            ("seahorse", "Seahorse", 16, 15, 4),
            ("stag", "Stag", 10, 14, 0),
            ("tortoise", "Tortoise", 14, 16, 6),
            ("trumpeter_swan", "Trumpeter Swan", 10, 12, 0),
            ("walrus", "Walrus", 12, 14, 4),
            ("whale_blue", "Whale Blue", 11, 10, 1),
            ("chalicotherium", "Chalicotherium", 12, 13, 4),
            ("digmaul", "Digmaul", 10, 13, 1),
            ("kaprosuchus", "Kaprosuchus", 15, 15, 3),
            ("moa", "Moa", 13, 12, 1),
            ("narwhal", "Narwhal", 11, 10, 1),
            ("uintatherium", "Uintatherium", 14, 17, 4),
            ("wolliped", "Wolliped", 14, 12, 1),
        ] {
            let mut explanations = Vec::new();
            let grounded = ground_companion_stat_block(
                slug,
                "companion",
                "Druid",
                display,
                1,
                &mut explanations,
            );
            assert!(grounded, "{slug} must ground a real stat block");
            let detail = &explanations
                .iter()
                .find(|e| e.id == format!("companion.{slug}_stat_block"))
                .unwrap_or_else(|| panic!("expected a companion.{slug}_stat_block record"))
                .detail;
            assert!(
                detail.contains(&format!("Str {expected_str}")),
                "{slug} expected base Str {expected_str} in detail: {detail}"
            );
            assert!(
                detail.contains(&format!("Con {expected_con}")),
                "{slug} expected base Con {expected_con} in detail: {detail}"
            );
            let table = companion_base_stat_table();
            let stats = table.get(slug).expect("entry must exist");
            assert_eq!(stats.natural_armor, expected_natural_armor, "{slug} natural armor");
            assert_eq!(stats.hit_die_size, 8, "{slug} hit die size is always d8 per the companion mechanic");
        }
    }

    /// Proves each new dinosaur entry is real and reachable, not merely
    /// inserted into the table -- same shape as `an_unknown_species_slug_
    /// refuses_rather_than_guesses`'s own positive counterpart, pinning the
    /// exact base ability scores this cycle's doc comments derive.
    #[test]
    fn the_nine_dinosaur_companions_ground_their_own_verified_base_scores() {
        for (slug, display, expected_str, expected_con, expected_natural_armor) in [
            ("allosaurus", "Allosaurus", 14i16, 10i16, 4i16),
            ("ankylosaurus", "Ankylosaurus", 10, 9, 9),
            ("pteranodon", "Pteranodon", 8, 10, 0),
            ("deinonychus", "Deinonychus", 11, 17, 1),
            ("velociraptor", "Velociraptor", 11, 17, 1),
            ("triceratops", "Triceratops", 10, 11, 6),
            ("tyrannosaurus", "Tyrannosaurus", 14, 10, 4),
            ("amargasaurus", "Amargasaurus", 11, 9, 3),
            ("brachiosaurus", "Brachiosaurus", 13, 11, 3),
        ] {
            let mut explanations = Vec::new();
            let grounded = ground_companion_stat_block(
                slug,
                "companion",
                "Druid",
                display,
                1,
                &mut explanations,
            );
            assert!(grounded, "{slug} must ground a real stat block");
            let detail = &explanations
                .iter()
                .find(|e| e.id == format!("companion.{slug}_stat_block"))
                .unwrap_or_else(|| panic!("expected a companion.{slug}_stat_block record"))
                .detail;
            assert!(
                detail.contains(&format!("Str {expected_str}")),
                "{slug} expected base Str {expected_str} in detail: {detail}"
            );
            assert!(
                detail.contains(&format!("Con {expected_con}")),
                "{slug} expected base Con {expected_con} in detail: {detail}"
            );
            let table = companion_base_stat_table();
            let stats = table.get(slug).expect("entry must exist");
            assert_eq!(stats.natural_armor, expected_natural_armor, "{slug} natural armor");
            assert_eq!(stats.hit_die_size, 8, "{slug} hit die size is always d8 per the companion mechanic");
        }
    }

    /// Row 20 cycle 7's own positive counterpart to
    /// `the_nine_dinosaur_companions_ground_their_own_verified_base_scores`:
    /// proves each of the 17 species cycle 7 added is real and reachable,
    /// pinning the exact base ability scores this cycle's doc comments
    /// derive. Troodon's own `expected_natural_armor` of 0 is a genuine
    /// verified value (no natural armor bonus at the starting level per its
    /// own source, matching the corpus's own absent `AC_Natural_Armor`
    /// token) -- not a stand-in for "not yet grounded".
    #[test]
    fn the_seventeen_cycle_seven_dinosaur_companions_ground_their_own_verified_base_scores() {
        for (slug, display, expected_str, expected_con, expected_natural_armor) in [
            ("elasmosaurus", "Elasmosaurus", 10i16, 12i16, 2i16),
            ("stegosaurus", "Stegosaurus", 10, 10, 6),
            ("dimetrodon", "Dimetrodon", 12, 14, 2),
            ("iguanodon", "Iguanodon", 17, 15, 3),
            ("spinosaurus", "Spinosaurus", 18, 15, 3),
            ("dimorphodon", "Dimorphodon", 10, 10, 1),
            ("diplodocus", "Diplodocus", 10, 10, 6),
            ("styracosaurus", "Styracosaurus", 10, 11, 6),
            ("ceratosaurus", "Ceratosaurus", 14, 11, 4),
            ("plesiosaurus", "Plesiosaurus", 12, 12, 1),
            ("therizinosaurus", "Therizinosaurus", 12, 10, 4),
            ("troodon", "Troodon", 7, 10, 0),
            ("giganotosaurus", "Giganotosaurus", 14, 10, 4),
            ("kentrosaurus", "Kentrosaurus", 10, 10, 2),
            ("quetzalcoatlus", "Quetzalcoatlus", 9, 10, 2),
            ("parasaurolophus", "Parasaurolophus", 11, 9, 2),
            ("tylosaurus", "Tylosaurus", 10, 10, 3),
        ] {
            let mut explanations = Vec::new();
            let grounded = ground_companion_stat_block(
                slug,
                "companion",
                "Druid",
                display,
                1,
                &mut explanations,
            );
            assert!(grounded, "{slug} must ground a real stat block");
            let detail = &explanations
                .iter()
                .find(|e| e.id == format!("companion.{slug}_stat_block"))
                .unwrap_or_else(|| panic!("expected a companion.{slug}_stat_block record"))
                .detail;
            assert!(
                detail.contains(&format!("Str {expected_str}")),
                "{slug} expected base Str {expected_str} in detail: {detail}"
            );
            assert!(
                detail.contains(&format!("Con {expected_con}")),
                "{slug} expected base Con {expected_con} in detail: {detail}"
            );
            let table = companion_base_stat_table();
            let stats = table.get(slug).expect("entry must exist");
            assert_eq!(stats.natural_armor, expected_natural_armor, "{slug} natural armor");
            assert_eq!(stats.hit_die_size, 8, "{slug} hit die size is always d8 per the companion mechanic");
        }
    }

    /// Row 20 cycle 7 named `pachycephalosaurus`/`ornithomimosaur` as two
    /// records that must keep refusing until a second independent source
    /// resolved each. Cycle 11 found that source for both (this module's
    /// own cycle 11 doc addendum); `the_thirty_cycle_eleven_companions_
    /// ground_their_own_verified_base_scores` above now proves both ground
    /// real, verified stat blocks. This test's own prior assertion (both
    /// refuse) is retired here rather than left pinning a now-false
    /// refusal -- the underlying fact it tracked changed, not the
    /// verification bar itself.
    #[test]
    fn pachycephalosaurus_and_ornithomimosaur_no_longer_refuse() {
        for (slug, display) in [
            ("pachycephalosaurus", "Pachycephalosaurus"),
            ("ornithomimosaur", "Ornithomimosaur"),
        ] {
            let mut explanations = Vec::new();
            let grounded =
                ground_companion_stat_block(slug, "companion", "Druid", display, 1, &mut explanations);
            assert!(
                grounded,
                "{slug} was resolved in cycle 11 and must ground a real, verified stat block"
            );
            assert!(!explanations.is_empty());
        }
    }
}
