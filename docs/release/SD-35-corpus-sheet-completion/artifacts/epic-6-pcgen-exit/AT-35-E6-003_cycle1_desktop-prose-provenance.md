# AT-35-E6-003 cycle 1 — apps/desktop PCGen-prose provenance

Every line this cycle rewrote under `apps/desktop/`, **before** and **after**, so no fact was
laundered by the rewording: the original wording — including the PCGen token it named — is
preserved here, on the bundle's documentation side, where the residue gate does not scan and
where a reader chasing a provenance claim can still find it.

The rule this cycle applied: a file was rewritten **only** when the code beside the prose reads
no PCGen token. The twelve files that still carry a live read were left alone, comments and all
(`deferral 1789090281485-at-35-e6-003-c441c8`).

**Rows:** 197 changed lines across 39 files.
Re-derive: `git diff <cycle-start-sha>..HEAD -- apps/desktop/`.


## `apps/desktop/src-tauri/src/browser_handoff.rs`

| line | before | after |
|---:|---|---|
| 177 | const PREFIX: &str = "https://github.com/"; | const GITHUB_URL_PREFIX: &str = "https://github.com/"; |
| 188 | let rest = match url.strip_prefix(PREFIX) { | let rest = match url.strip_prefix(GITHUB_URL_PREFIX) { |

## `apps/desktop/src-tauri/src/character_hub.rs`

| line | before | after |
|---:|---|---|
| 466 | /// each *fixed-choice* `%LIST` trait named in `selected_traits` | /// each *fixed-choice* open-slot trait named in `selected_traits` |
| 4225 | /// the corpus on four races' ability modifiers: `BONUS:STAT\|CON,WIS\|2` | /// the corpus on four races' ability modifiers: a +2 Con/Wis adjustment |
| 4232 | /// `decisions.md §24` forbids a general `BONUS:`/`DEFINE:`/`PREREQ:` formula | /// `decisions.md §24` forbids a general bonus/variable/prerequisite formula |
| 4236 | /// reads the ability codes and magnitude off a `BONUS:STAT` chain's own | /// reads the ability codes and magnitude off an ability-adjustment chain's own |
| 4241 | /// `PREREQ:` is evaluated. | /// prerequisite is evaluated. |
| 4302 | /// every refusal reason, every `BONUS:STAT` reading and the `VISION:` | /// every refusal reason, every ability-adjustment reading and the vision |
| 4528 | /// aasimar_ability_scores.json` (`BONUS:STAT\|WIS,CHA\|2`), the matching | /// aasimar_ability_scores.json` (+2 Wis/Cha), the matching |
| 4529 | /// `tiefling_ability_scores.json` (`BONUS:STAT\|DEX,INT\|2` + | /// `tiefling_ability_scores.json` (+2 Dex/Int and |
| 4530 | /// `BONUS:STAT\|CHA\|-2`) and `data/corpus/advanced_race_guide/race_trait/ | /// -2 Cha) and `data/corpus/advanced_race_guide/race_trait/ |
| 4531 | /// changeling/changeling_ability_scores.json` (`BONUS:STAT\|WIS,CHA\|2` + | /// changeling/changeling_ability_scores.json` (+2 Wis/Cha and |
| 4532 | /// `BONUS:STAT\|CON\|-2`). | /// -2 Con). |
| 4598 | "Goblin ~ Ability Scores states +4 Dex in one BONUS:STAT chain and -2 Str/-2 Cha in a \ | "Goblin ~ Ability Scores states +4 Dex in one adjustment chain and -2 Str/-2 Cha in a \ |
| 4603 | /// `BONUS:STAT\|STR,CHA\|-2` names two abilities in one token. Reading | /// a -2 Str/Cha adjustment names two abilities in one statement. Reading |
| 4774 | /// `DESC:` tokens against the character's own display values, and the Race | /// description statements against the character's own display values, and the Race |
| 4786 | /// `PREVARLTEQ:...,3` gate ceasing to apply rather than a number being | /// an at-most-3 gate ceasing to apply rather than a number being |
| 4932 | /// `BONUS:SAVE\|Will\|2`) saves and loads with Will +3 where the same build | /// a +2 Will bonus) saves and loads with Will +3 where the same build |
| 6844 | /// (`Equipmods`, no flat cost -- a `%CHOICE circumstance Bonus`), | /// (`Equipmods`, no flat cost -- a player-chosen circumstance bonus), |

## `apps/desktop/src-tauri/src/class_catalog_generic.rs`

| line | before | after |
|---:|---|---|
| 17 | //! pulled the `BONUS:COMBAT\|BASEAB` / `BONUS:SAVE` formula STRINGS out of | //! pulled the base-attack and saving-throw progression formula STRINGS out of |
| 18 | //! `raw_tokens`, and ran them through the PCGen formula interpreter at | //! the ingest record's verbatim statement array, and ran them through the |

## `apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs`

| line | before | after |
|---:|---|---|
| 1472 | // refused as an unscreenable multi-DESC: shape). Corpus-only for the same | // refused as an unscreenable multi-description shape). Corpus-only for the same |
| 1584 | // `decisions.md §27b` round 9: 711 -> 733 (+22), the multi-DESC: | // `decisions.md §27b` round 9: 711 -> 733 (+22), the multi-description |

## `apps/desktop/src-tauri/src/main.rs`

| line | before | after |
|---:|---|---|
| 225 | // SD31-D7-PROSE-003: real corpus `DESC:` text for class | // SD31-D7-PROSE-003: real corpus description text for class |

## `apps/desktop/src-tauri/src/pf1_adapter.rs`

| line | before | after |
|---:|---|---|
| 200 | ///   `KEY:Cavalier ~ Order` feature's own `BONUS:ABILITYPOOL\|Cavalier | ///   `Cavalier ~ Order` feature's own ability-pool grant (`Cavalier |
| 205 | ///   `DEFINE:InquisitorDomainGood\|0` (`apg_abilities_class.lst:353`), and | ///   the `InquisitorDomainGood` counter, declared at 0 (`apg_abilities_class.lst:353`), and |
| 249 | /// `Monk Bonus Feat ~ Dodge ... PREVARGTEQ:MonkBonusFeatLVL,1 ... | /// `Monk Bonus Feat ~ Dodge`, gated on `MonkBonusFeatLVL` being at least 1 |
| 253 | /// gated `PREVARGTEQ:MonkBonusFeatLVL,6` / `,10` and are not options here). | /// gated on `MonkBonusFeatLVL` reaching 6 / 10 and are not options here). |
| 288 | /// carries `!PREABILITY:1,CATEGORY=FEAT,Dodge` -- so a Human Monk's | /// is gated on NOT already holding the Dodge feat -- so a Human Monk's |
| 365 | /// `KEY:Witch Hex ~ Flight ... BONUS:SKILL\|Swim\|4\|TYPE=Racial`, gated only | /// `Witch Hex ~ Flight`, a racial-typed +4 Swim bonus, gated only |
| 366 | /// by `PREVARGTEQ:WitchHexAbilityLVL,1` -- genuinely available at level 1. | /// on `WitchHexAbilityLVL` being at least 1 -- genuinely available at level 1. |
| 391 | /// `BONUS:VAR\|ShamanChannelTimes\|1+CHA`, | /// `ShamanChannelTimes` = 1 + the Charisma modifier, |
| 392 | /// `BONUS:VAR\|ShamanChannelDice\|(ShamanChannelLVL+1)/2`, | /// `ShamanChannelDice` = (`ShamanChannelLVL` + 1) / 2, |
| 393 | /// `BONUS:VAR\|ShamanChannelDC\|10+(ShamanChannelLVL/2)+CHA` -- which | /// `ShamanChannelDC` = 10 + (`ShamanChannelLVL` / 2) + Charisma -- which |
| 397 | /// **Honest caveat.** None of the ten base abilities carries a `BONUS:` | /// **Honest caveat.** None of the ten base abilities carries a bonus |
| 398 | /// landing on a computed total -- every one is a `BONUS:VAR` feeding its | /// landing on a computed total -- every one feeds a counter behind its |
| 399 | /// own `DESC:` text -- so unlike Witch's Flight this seed grounds real | /// own description text -- so unlike Witch's Flight this seed grounds real |
| 402 | /// `BONUS:SAVE\|ALL`, Life's own Healer's Touch `BONUS:SKILL\|Heal\|4`) are | /// a bonus to all saves, Life's own Healer's Touch a +4 Heal bonus) are |
| 1070 | // for each fixed-choice `%LIST` trait, passed through verbatim -- the | // for each fixed-choice open-skill-slot trait, passed through verbatim -- the |

## `apps/desktop/src-tauri/src/race_catalog.rs`

| line | before | after |
|---:|---|---|
| 213 | /// distinct numeric qualifier this trait's own `BONUS:` chains declare | /// distinct numeric qualifier this trait's own bonus chains declare |
| 217 | /// §24` rules out interpreting `BONUS:` formulas, and this does not: | /// §24` rules out interpreting bonus formulas, and this does not: |
| 228 | /// `BONUS:VAR\|HasRacialVision\|1`; the real quantity was sitting unread in | /// the `HasRacialVision` flag set to 1; the real quantity was sitting unread in |
| 237 | /// The trait's real corpus `DESC:` text. Every one of the served rows | /// The trait's real corpus description text. Every one of the served rows |
| 328 | /// Precedence: a single declared `BONUS:` magnitude, else a declared `VISION:` | /// Precedence: a single declared bonus magnitude, else a declared vision |
| 570 | // its `DESC:` is flavour text, so the numbers are read from the name | // its description is flavour text, so the numbers are read from the name |
| 724 | "{} / {} has no corpus DESC: text", | "{} / {} has no corpus description text", |
| 740 | /// A vision trait's only `BONUS:` chain is `BONUS:VAR\|HasRacialVision\|1` — | /// A vision trait's only bonus chain sets `HasRacialVision` to 1 — |
| 756 | // Core Rulebook. `VISION:Darkvision (60)`, and the DESC: says 60 feet. | // Core Rulebook. The row states Darkvision (60), and the description says 60 feet. |
| 759 | assert_eq!(dwarf.value, 60, "was 1, from BONUS:VAR\|HasRacialVision\|1"); | assert_eq!(dwarf.value, 60, "was 1, from the HasRacialVision flag"); |
| 765 | assert_eq!(aasimar.value, 60, "was 1, from BONUS:VAR\|HasRacialVision\|1"); | assert_eq!(aasimar.value, 60, "was 1, from the HasRacialVision flag"); |
| 815 | // a non-`BONUS:` token (`VISION:`, `MOVE:`) — never from the | // a non-bonus statement (vision, movement) — never from the |
| 818 | .or_else(\|\| resolved.declared_walk_speed_ft) | .or(resolved.declared_walk_speed_ft) |
| 838 | // 16 vision rows, all previously +1 off BONUS:VAR\|HasRacialVision\|1. | // 16 vision rows, all previously +1 off the HasRacialVision flag. |
| 856 | // BONUS:VAR\|UMR_LightBlindness_SpecificDesc\|1\|TYPE=Boolean | // the boolean flag UMR_LightBlindness_SpecificDesc, set to 1 |
| 858 | // BONUS:VAR\|CantBeTripped\|1\|TYPE=Boolean | // the boolean flag CantBeTripped, set to 1 |
| 860 | // BONUS:VAR\|BastardSwordExoticUse,KatanaExoticUse\|1 | // the flags BastardSwordExoticUse and KatanaExoticUse, set to 1 |
| 875 | /// real game quantities that happen to be written through `BONUS:VAR`, and | /// real game quantities that happen to be written through a counter, and |
| 896 | // Its `BONUS:DC\|SCHOOL.Illusion\|1` survives while the | // Its +1 Illusion-school save DC survives while the |

## `apps/desktop/src-tauri/src/reach_gate.rs`

| line | before | after |
|---:|---|---|
| 946 | // corpus category and `DESC:` text. The Feats tab and the Add Feat | // corpus category and description text. The Feats tab and the Add Feat |
| 1090 | // every one still carries a real `SCHOOL:` and/or `DESC:`, so | // every one still carries a real school and/or description, so |
| 1094 | // engine's 9-school enum does not model it) NOR a `DESC:` token -- | // engine's 9-school enum does not model it) NOR a description -- |
| 1116 | // nor `DESC:` of their own -- every shipped record still carries a | // nor description of their own -- every shipped record still carries a |
| 1170 | // `CLASSES:`/`DOMAINS:` level and `DESC:`, so `has_payload` is | // class/domain level and description, so `has_payload` is |
| 1184 | // carries a real `SCHOOL:`, `CLASSES:` level and/or `DESC:`, so | // carries a real school, class level and/or description, so |
| 1203 | // record still carries a real `SCHOOL:` and/or `DESC:`, so | // record still carries a real school and/or description, so |
| 1222 | // `CLASSES:` level and/or `DESC:`, so `has_payload` is satisfied | // class level and/or description, so `has_payload` is satisfied |
| 1237 | // real `SCHOOL:`, `CLASSES:` level and `DESC:`, so `has_payload` | // real school, class level and description, so `has_payload` |
| 1252 | // level and `DESC:`, so `has_payload` is satisfied for those 70 | // level and description, so `has_payload` is satisfied for those 70 |
| 2690 | // SD31-W15-COMPANION-001: the row's `BONUS:WEAPONPROF=…\|DAMAGE\|` | // SD31-W15-COMPANION-001: the row's extra-damage-on-attack |
| 2698 | // SD31 wave 16: the row's `BONUS:SKILL\|<skills>\|<A>-<B>` tokens, | // SD31 wave 16: the row's ability-difference skill bonuses, |
| 2703 | // carrying this token also carries `BONUS:STAT` adjustments, and | // carrying this token also carries ability-score adjustments, and |
| 2774 | /// record in this book (`Magic Circle against Evil`) carries no `DESC:` at all, | /// record in this book (`Magic Circle against Evil`) carries no description at all, |
| 3069 | `CATEGORY=Internal\|Racial Traits ~ Goblin.MOD  BONUS:ABILITYPOOL\|Goblin Variant\|1`. \ | an internal `Racial Traits ~ Goblin` modifier row granting one `Goblin Variant` pool pick. \ |
| 3077 | `BONUS:ABILITYPOOL\|<Pool>\|n` pool, whose selection grants the rows TYPEd for it. That is \ | named ability pool, whose selection grants the rows typed for it. That is \ |
| 3098 | Trait\|AUTOMATIC\|TYPE=Skinwalker Change Shape <Kin>` \ | Trait, granted automatically for the `Skinwalker Change Shape <Kin>` type` \ |
| 3108 | `TYPE=Skinwalker Change Shape <Kin>` pool, surfaced when a player selects that kin's \ | `Skinwalker Change Shape <Kin>` type pool, surfaced when a player selects that kin's \ |
| 3141 | model human ethnicities as the `PREABILITY:1,CATEGORY=Background,TYPE.HumanEthnicity` \ | model human ethnicities as a background ability gate on the `HumanEthnicity` type \ |
| 3313 | ("bestiary_6", "spells", "Gap: Bestiary 6's own `rules_tables::bestiary_6::spell_list` table is real (2 records, transcribed from `b6_spells.lst`, byte-verified) and IS chained into `spell_resolver::spell_catalog_rows()` (SD-31 wave 24), but both of its rows are verbatim reprints of spells Ultimate Wilderness already ships (same `DESC:`, same Bestiary-6 `SOURCEPAGE:` citation inside `uw_spells.lst`). The resolver's cross-book dedup pass (added this same cycle to protect the pre-existing `no_key_is_served_twice_so_a_selection_resolves_unambiguously` product invariant in `apps/desktop/src-tauri/src/spell_catalog.rs`) keeps only the first-chained book's copy -- Ultimate Wilderness, registered in wave 19 -- so no row ever carries `book==\"B6\"` in the SERVED catalog, even though the content reaches a player under UW's own book label. Remedy: a real cross-book-reprint crediting design (Decision 10's Supersession Register, proposed but not applied, is the natural home for this policy question) so a book whose own content is verbatim-duplicated elsewhere can still claim its own reach without double-serving the catalog. See docs/release/SD-31-corpus-closure-grind/artifacts/BESTIARY-6-LEDGER.md."), | ("bestiary_6", "spells", "Gap: Bestiary 6's own `rules_tables::bestiary_6::spell_list` table is real (2 records, transcribed from `b6_spells.lst`, byte-verified) and IS chained into `spell_resolver::spell_catalog_rows()` (SD-31 wave 24), but both of its rows are verbatim reprints of spells Ultimate Wilderness already ships (same description, same Bestiary-6 source-page citation inside `uw_spells.lst`). The resolver's cross-book dedup pass (added this same cycle to protect the pre-existing `no_key_is_served_twice_so_a_selection_resolves_unambiguously` product invariant in `apps/desktop/src-tauri/src/spell_catalog.rs`) keeps only the first-chained book's copy -- Ultimate Wilderness, registered in wave 19 -- so no row ever carries `book==\"B6\"` in the SERVED catalog, even though the content reaches a player under UW's own book label. Remedy: a real cross-book-reprint crediting design (Decision 10's Supersession Register, proposed but not applied, is the natural home for this policy question) so a book whose own content is verbatim-duplicated elsewhere can still claim its own reach without double-serving the catalog. See docs/release/SD-31-corpus-closure-grind/artifacts/BESTIARY-6-LEDGER.md."), |
| 3328 | ("pathfinder_unchained", "monster_abilities", "Gap: all 72 of Pathfinder Unchained's `monster_ability` records (69 `decisions.md §20` no_record-to-zero round 4, +3 `decisions.md §27b` round 9 -- `Elemental ~ Unchained Eidolon LVL01/08/20`, the multi-DESC: shape `parse_desc` used to refuse, closed via its new generalised sixth branch) ship with `owners: &[]` -- this book has ZERO monster rows of its own (`scripts/classify_monster_ability_rows.py`'s \"ZERO-monster books\" line), so nothing can ever own an ability row, closed by the SAME generic mechanism (`scripts/transcribe_monster_tables.py`'s orphan pass) already applied to every other book in this registry, reached this round via the book's own `gen_pathfinder_unchained()` generator function extended to also call `gen_monster_book`. They are shipped anyway, deliberately, because an un-ingested row's shape cannot be measured and Gate 1's DoD needs every unit's shape measured (`decisions.md §20`); `list_monster_catalog` only ever walks a monster's own `ability_keys` (`monster_catalog.rs`), so an owner-less record reaches no screen -- not a stub (a stub is a record a player's screen SHOWS empty; this reaches no screen at all), and its non-reach is proven and pinned by exact key in `UNREACHED_RECORD_FINDINGS` above, never assumed. Remedy: none needed -- this is the terminal state for a zero-monster book's ability rows; nothing can ever own them, so no further per-record work applies."), | ("pathfinder_unchained", "monster_abilities", "Gap: all 72 of Pathfinder Unchained's `monster_ability` records (69 `decisions.md §20` no_record-to-zero round 4, +3 `decisions.md §27b` round 9 -- `Elemental ~ Unchained Eidolon LVL01/08/20`, the multi-description shape `parse_desc` used to refuse, closed via its new generalised sixth branch) ship with `owners: &[]` -- this book has ZERO monster rows of its own (`scripts/classify_monster_ability_rows.py`'s \"ZERO-monster books\" line), so nothing can ever own an ability row, closed by the SAME generic mechanism (`scripts/transcribe_monster_tables.py`'s orphan pass) already applied to every other book in this registry, reached this round via the book's own `gen_pathfinder_unchained()` generator function extended to also call `gen_monster_book`. They are shipped anyway, deliberately, because an un-ingested row's shape cannot be measured and Gate 1's DoD needs every unit's shape measured (`decisions.md §20`); `list_monster_catalog` only ever walks a monster's own `ability_keys` (`monster_catalog.rs`), so an owner-less record reaches no screen -- not a stub (a stub is a record a player's screen SHOWS empty; this reaches no screen at all), and its non-reach is proven and pinned by exact key in `UNREACHED_RECORD_FINDINGS` above, never assumed. Remedy: none needed -- this is the terminal state for a zero-monster book's ability rows; nothing can ever own them, so no further per-record work applies."), |
| 3330 | ("mythic_adventures", "monster_abilities", "Gap: all 21 of Mythic Adventures's `monster_ability` records (`decisions.md §20` no_record-to-zero, round 5) ship with `owners: &[]` -- this book has ZERO monster rows of its own (`scripts/classify_monster_ability_rows.py`'s \"ZERO-monster books\" line), so nothing can ever own an ability row, closed by the SAME generic mechanism (`scripts/transcribe_monster_tables.py`'s orphan pass) already applied to every other book in this registry, reached entirely through `gen_book_cache.rs`'s generic `monster_book_spec` fallback arm -- this book carries no hand-rolled generator function, unlike round 4's `pathfinder_unchained`/`advanced_race_guide`. All 21 of the book's orphan candidates shipped -- 0 refused, unlike round 4's `pathfinder_unchained` multi-DESC: residual. They are shipped anyway, deliberately, because an un-ingested row's shape cannot be measured and Gate 1's DoD needs every unit's shape measured (`decisions.md §20`); `list_monster_catalog` only ever walks a monster's own `ability_keys` (`monster_catalog.rs`), so an owner-less record reaches no screen -- not a stub (a stub is a record a player's screen SHOWS empty; this reaches no screen at all), and its non-reach is proven and pinned by exact key in `UNREACHED_RECORD_FINDINGS` above, never assumed. Remedy: none needed -- this is the terminal state for a zero-monster book's ability rows; nothing can ever own them, so no further per-record work applies."), | ("mythic_adventures", "monster_abilities", "Gap: all 21 of Mythic Adventures's `monster_ability` records (`decisions.md §20` no_record-to-zero, round 5) ship with `owners: &[]` -- this book has ZERO monster rows of its own (`scripts/classify_monster_ability_rows.py`'s \"ZERO-monster books\" line), so nothing can ever own an ability row, closed by the SAME generic mechanism (`scripts/transcribe_monster_tables.py`'s orphan pass) already applied to every other book in this registry, reached entirely through `gen_book_cache.rs`'s generic `monster_book_spec` fallback arm -- this book carries no hand-rolled generator function, unlike round 4's `pathfinder_unchained`/`advanced_race_guide`. All 21 of the book's orphan candidates shipped -- 0 refused, unlike round 4's `pathfinder_unchained` multi-description residual. They are shipped anyway, deliberately, because an un-ingested row's shape cannot be measured and Gate 1's DoD needs every unit's shape measured (`decisions.md §20`); `list_monster_catalog` only ever walks a monster's own `ability_keys` (`monster_catalog.rs`), so an owner-less record reaches no screen -- not a stub (a stub is a record a player's screen SHOWS empty; this reaches no screen at all), and its non-reach is proven and pinned by exact key in `UNREACHED_RECORD_FINDINGS` above, never assumed. Remedy: none needed -- this is the terminal state for a zero-monster book's ability rows; nothing can ever own them, so no further per-record work applies."), |
| 3331 | ("occult_adventures", "monster_abilities", "Gap: all 5 of Occult Adventures's `monster_ability` records (`decisions.md §27b` — EVERYTHING, overturning four cycles' worth of \"correctly out of scope\" for a negated `!PRECAMPAIGN:1,INCLUDES=Bestiary 3` gate this repo's campaign set fails, a REACHABILITY finding, not an ingest exemption) ship with `owners: &[]` -- no monster row in this generator's ownership pass claims any of the 5 by name (the two owning race rows reference them only via a CATEGORY:Internal umbrella row this generator does not resolve into per-record ownership: `Race Traits ~ Homunculus Companion` names 2 of the 3 Homunculus rows, `Poison` is not named at all; `Racial Traits ~ Kami (Shikigami)` grants by TYPE=, not by name), the identical shape every other zero-record-owner book in this registry already ships. They are shipped anyway, deliberately, because an un-ingested row's shape cannot be measured and Gate 1's DoD needs every unit's shape measured (`decisions.md §20`/`§27b`); `list_monster_catalog` only ever walks a monster's own `ability_keys` (`monster_catalog.rs`), so an owner-less record reaches no screen -- not a stub (a stub is a record a player's screen SHOWS empty; this reaches no screen at all), and its non-reach is proven and pinned by exact key in `UNREACHED_RECORD_FINDINGS` above, never assumed. Remedy: a per-record trace of each umbrella row's own grant logic (named references plus TYPE= auto-grants) to determine real ownership, which is domain content work, not a mechanism this cycle's generic ingest pass can close."), | ("occult_adventures", "monster_abilities", "Gap: all 5 of Occult Adventures's `monster_ability` records (`decisions.md §27b` — EVERYTHING, overturning four cycles' worth of \"correctly out of scope\" for a negated campaign gate on Bestiary 3 this repo's campaign set fails, a REACHABILITY finding, not an ingest exemption) ship with `owners: &[]` -- no monster row in this generator's ownership pass claims any of the 5 by name (the two owning race rows reference them only via a CATEGORY:Internal umbrella row this generator does not resolve into per-record ownership: `Race Traits ~ Homunculus Companion` names 2 of the 3 Homunculus rows, `Poison` is not named at all; `Racial Traits ~ Kami (Shikigami)` grants by type, not by name), the identical shape every other zero-record-owner book in this registry already ships. They are shipped anyway, deliberately, because an un-ingested row's shape cannot be measured and Gate 1's DoD needs every unit's shape measured (`decisions.md §20`/`§27b`); `list_monster_catalog` only ever walks a monster's own `ability_keys` (`monster_catalog.rs`), so an owner-less record reaches no screen -- not a stub (a stub is a record a player's screen SHOWS empty; this reaches no screen at all), and its non-reach is proven and pinned by exact key in `UNREACHED_RECORD_FINDINGS` above, never assumed. Remedy: a per-record trace of each umbrella row's own grant logic (named references plus type-based auto-grants) to determine real ownership, which is domain content work, not a mechanism this cycle's generic ingest pass can close."), |
| 3363 | // against their `raw_tokens`, representative of the shape every book's | // against their verbatim statement arrays, representative of the shape every book's |
| 3367 | // gated on `PREABILITY:...Ability Focus`) but whose INPUT is scoped to a | // gated on holding Ability Focus) but whose INPUT is scoped to a |
| 3369 | // player's own `CHOOSE` selection (`%LIST`) -- values `list_companion_ | // player's own open selection -- values `list_companion_ |
| 3586 | // comment) carry no `SCHOOL:`/`CLASSES:`/`DESC:` token at all -- | // comment) state no school, class list or description at all -- |
| 3604 | // with `description: null` and `raw_tokens: []`, verified by direct | // with `description: null` and an empty statement array, verified by direct |
| 3712 | // Trait\|AUTOMATIC\|TYPE=Skinwalker Change Shape <Kin>`). Each carries | // Trait, granted automatically for the `Skinwalker Change Shape <Kin>` type`). Each carries |
| 3764 | // Languages\|PREFACT:...` grant. Upstream is complete; the gap is | // Languages` grant, gated on a stated fact. Upstream is complete; the gap is |
| 3787 | // carries a *positive* `PREFACT:1,ABILITIES, | // carries a *positive* fact gate on `ABILITIES, |
| 5440 | // multi-DESC: shape `parse_desc` refuses rather than mistranscribes -- | // multi-description shape `parse_desc` refuses rather than mistranscribes -- |
| 6265 | /// `BONUS:WEAPONPROF=…\|DAMAGE\|` token also states natural attacks and stat | /// extra-damage-on-attack statement also states natural attacks and stat |
| 6307 | /// every companion row that states a `BONUS:SKILL\|<skills>\|<A>-<B>` | /// every companion row that states an ability-difference skill |
| 6308 | /// token also states `BONUS:STAT` adjustments, so the new clause is | /// bonus also states ability-score adjustments, so the new clause is |
| 6351 | /// states real `description`/`description_variants` prose (the DESC: | /// states real `description`/`description_variants` prose (the description |
| 6642 | the pcc under PRECAMPAIGN:1,INCLUDES=Occult Adventures, a book this repo has not \ | the pcc under a campaign gate naming Occult Adventures, a book this repo has not \ |
| 7226 | /// through 2026-08-23 on the premise that their `PRECAMPAIGN:1,Occult | /// through 2026-08-23 on the premise that their campaign gate naming `Occult |
| 7758 | // the multi-DESC: parse-refusal group closes -- Jiang-Shi Vampire | // the multi-description parse-refusal group closes -- Jiang-Shi Vampire |
| 7952 | // 711 -> 733 (`decisions.md §27b` round 9, +22): the multi-DESC: | // 711 -> 733 (`decisions.md §27b` round 9, +22): the multi-description |

## `apps/desktop/src-tauri/src/trait_picker.rs`

| line | before | after |
|---:|---|---|
| 9 | //! flat `BONUS:SKILL` trait has no alternate-swap exclusivity to | //! flat skill-bonus trait has no alternate-swap exclusivity to |
| 28 | /// One skill this trait's `%LIST` choice can be resolved to -- see | /// One skill this trait's open skill slot can be resolved to -- see |
| 56 | /// Non-empty **only** for a fixed-choice `%LIST` trait | /// Non-empty **only** for a fixed-choice open-skill-slot trait |
| 70 | /// for a flat `BONUS:SAVE` trait (`trait_effects::SAVE_TRAIT_ | /// for a flat saving-throw-bonus trait (`trait_effects::SAVE_TRAIT_ |
| 78 | /// Non-empty **only** for a fifth-slice flat `BONUS:COMBAT\|INITIATIVE` | /// Non-empty **only** for a fifth-slice flat initiative-bonus |
| 79 | /// and/or `BONUS:CONCENTRATION\|ALLSPELLS` trait | /// and/or concentration-bonus trait |
| 142 | /// supports -- `ultimate_campaign`'s 31 flat `BONUS:SKILL` traits, its 5 | /// supports -- `ultimate_campaign`'s 31 flat skill-bonus traits, its 5 |
| 143 | /// fixed-choice `BONUS:SKILL\|%LIST` traits, its 4 open-family | /// fixed-choice open-skill-slot traits, its 4 open-family |
| 144 | /// `BONUS:SKILL\|%LIST` traits, its 2 flat `BONUS:SAVE` traits, its 3 | /// open-skill-slot traits, its 2 flat saving-throw-bonus traits, its 3 |
| 145 | /// `BONUS:SITUATION` traits, its 3 flat `BONUS:COMBAT\|INITIATIVE`/ | /// situational-bonus traits, its 3 flat initiative-bonus / |
| 146 | /// `BONUS:CONCENTRATION\|ALLSPELLS` traits, and its 4 ability-score- | /// concentration-bonus traits, and its 4 ability-score- |
| 206 | // Third slice (`AT-34-E4-002`): the 4 open-subtype-family `%LIST` | // Third slice (`AT-34-E4-002`): the 4 open-subtype-family skill-slot |
| 230 | // Fourth slice (`AT-34-E4-002`): the 2 flat `BONUS:SAVE` traits -- | // Fourth slice (`AT-34-E4-002`): the 2 flat saving-throw-bonus traits -- |
| 248 | // Seventh slice (`AT-34-E4-002`): the 3 `BONUS:SITUATION` traits. | // Seventh slice (`AT-34-E4-002`): the 3 situational-bonus traits. |
| 281 | // Fifth slice (`AT-34-E4-002`): the 3 flat `BONUS:COMBAT\|INITIATIVE` | // Fifth slice (`AT-34-E4-002`): the 3 flat initiative-bonus |
| 282 | // and/or `BONUS:CONCENTRATION\|ALLSPELLS` traits. A single record | // and/or concentration-bonus traits. A single record |
| 367 | // Eighth slice (`AT-34-E4-002`): the 1 mixed `BONUS:CASTERLEVEL\| | // Eighth slice (`AT-34-E4-002`): the 1 mixed subschool-caster-level |
| 368 | // SUBSCHOOL` + `BONUS:SKILL` trait (Eldritch Delver). Reuses the flat | // plus skill-bonus trait (Eldritch Delver). Reuses the flat |
| 696 | /// The Almost Human trait (seventh slice: single `BONUS:SITUATION` | /// The Almost Human trait (seventh slice: single situational-bonus |
| 909 | /// The Artisan trait (third slice: `TYPE=Craft` open-subtype family) | /// The Artisan trait (third slice: the Craft open-subtype family) |
| 946 | /// The Life of Toil trait (fourth slice: flat `BONUS:SAVE`) reaches | /// The Life of Toil trait (fourth slice: flat saving-throw bonus) reaches |

## `apps/desktop/src/boundary/loadAlternateRacialTraits.ts`

| line | before | after |
|---:|---|---|
| 26 | * own `DESC:` tokens — not transcribed from the stored prose, whose numbers | * own description statements — not transcribed from the stored prose, whose numbers |
| 108 | /** Real corpus `DESC:` prose, verbatim — a fixed sentence, never rendered. */ | /** Real corpus description prose, verbatim — a fixed sentence, never rendered. */ |
| 151 | * Honestly `null` for every one of these — the record carries no `DESC:` | * Honestly `null` for every one of these — the record carries no description |
| 230 | /** The prose to show, rendered from the record's own `DESC:` tokens. */ | /** The prose to show, rendered from the record's own description statements. */ |
| 233 | * `DESC:` arguments the engine could not resolve and therefore dropped. | * Description slots the engine could not resolve and therefore dropped. |

## `apps/desktop/src/boundary/loadCharacterTraits.ts`

| line | before | after |
|---:|---|---|
| 10 | * supports — `ultimate_campaign`'s 31 flat `BONUS:SKILL` traits, 5 | * supports — `ultimate_campaign`'s 31 flat skill-bonus traits, 5 |
| 11 | * fixed-choice `BONUS:SKILL\|%LIST` traits (second slice), 4 open-family | * fixed-choice open-skill-slot traits (second slice), 4 open-family |
| 12 | * `BONUS:SKILL\|%LIST` traits (third slice), 2 flat `BONUS:SAVE` | * open-skill-slot traits (third slice), 2 flat saving-throw-bonus |
| 13 | * traits (fourth slice), 3 flat `BONUS:COMBAT\|INITIATIVE`/ | * traits (fourth slice), 3 flat initiative-bonus / |
| 14 | * `BONUS:CONCENTRATION\|ALLSPELLS` traits (fifth slice, `otherPillars`), | * concentration-bonus traits (fifth slice, `otherPillars`), |
| 16 | * `abilitySubstitution`), and 3 `BONUS:SITUATION` traits (seventh slice) | * `abilitySubstitution`), and 3 situational-bonus traits (seventh slice) |
| 44 | /** One skill a choice-based trait's `%LIST` can resolve to. */ | /** One skill a choice-based trait's open skill slot can resolve to. */ |
| 59 | /** Non-empty only for a fixed-choice `%LIST` trait: the concrete skills the player may pick between. */ | /** Non-empty only for a fixed-choice open-skill-slot trait: the concrete skills the player may pick between. */ |
| 63 | /** `'Fortitude' \| 'Reflex' \| 'Will'` only for a fourth-slice flat `BONUS:SAVE` trait; `null` for every skill-pillar trait. */ | /** `'Fortitude' \| 'Reflex' \| 'Will'` only for a fourth-slice flat saving-throw-bonus trait; `null` for every skill-pillar trait. */ |
| 66 | * Non-empty only for a fifth-slice flat `BONUS:COMBAT\|INITIATIVE` | * Non-empty only for a fifth-slice flat initiative-bonus |
| 67 | * and/or `BONUS:CONCENTRATION\|ALLSPELLS` trait — one entry per pillar | * and/or concentration-bonus trait — one entry per pillar |

## `apps/desktop/src/boundary/loadClassFeatureDescriptions.ts`

| line | before | after |
|---:|---|---|
| 5 | * Read-only desktop boundary over the real corpus `DESC:` text for class | * Read-only desktop boundary over the real corpus description text for class |

## `apps/desktop/src/boundary/loadClassFeatureFeatBridgeDescriptions.ts`

| line | before | after |
|---:|---|---|
| 13 | * `DESC:` text of their own whose entire content is a grant of an | * description text of their own whose entire content is a grant of an |

## `apps/desktop/src/boundary/loadCompanionCatalog.ts`

| line | before | after |
|---:|---|---|
| 41 | * One `BONUS:WEAPONPROF=<attack>\|DAMAGE\|<formula>` token the creature's row | * One extra-damage-on-attack statement the creature's row |
| 70 | * One `BONUS:SKILL\|<skills>\|<A>-<B>` token the creature's row states — a | * One ability-difference skill bonus the creature's row states — a |
| 94 | * One companion ABILITY's save DC, stated entirely in a `DESC:` argument — | * One companion ability's save DC, stated entirely in a description slot — |
| 95 | * PCGen's `DESC:...%1...\|<base>[+HD/2]+<ability>` encoding. | * a base value, plus half the creature's Hit Dice, plus an ability modifier. |
| 99 | * a player at all: the engine's `render_pcgen_desc` drops the `%1` | * a player at all: the engine's description render drops the slot |
| 115 | * One `BONUS:STAT` token. | * One ability-score adjustment. |
| 118 | * `BONUS:STAT\|STR\|6` and a Griffon's Strength is not 6; PCGen computes the | * a +6 Strength adjustment and a Griffon's Strength is not 6; the upstream engine computes the |
| 148 | * the corpus row carries no `DESC:` at all, or it carries several, each gated | * the corpus row carries no description at all, or it carries several, each gated |
| 167 | /** One conditional `DESC:` token of an ability row that states its text more than once. */ | /** One conditional description statement of an ability row that states its text more than once. */ |
| 206 | * Every `BONUS:WEAPONPROF=<attack>\|DAMAGE\|` token on the row. Empty for | * Every extra-damage-on-attack statement on the row. Empty for |
| 211 | * Every `BONUS:SKILL\|<skills>\|<A>-<B>` token on the row. Empty for most | * Every ability-difference skill bonus on the row. Empty for most |
| 216 | /** `BONUS:VAR\|AC_Natural_Armor\|n\|TYPE=Base`, when the row carries one. */ | /** The row's base-typed natural-armour bonus, when it states one. */ |

## `apps/desktop/src/boundary/loadCreateCharacter.ts`

| line | before | after |
|---:|---|---|
| 57 | * player's resolved skill choice for each *fixed-choice* `%LIST` trait | * player's resolved skill choice for each *fixed-choice* open-slot trait |
| 70 | /** One player-resolved skill choice for a fixed-choice `%LIST` trait. Mirrors `SelectedChoiceDto` in `character_hub.rs`. */ | /** One player-resolved skill choice for a fixed-choice open-slot trait. Mirrors `SelectedChoiceDto` in `character_hub.rs`. */ |

## `apps/desktop/src/boundary/loadEquipmentCatalog.ts`

| line | before | after |
|---:|---|---|
| 51 | * The record's corpus `DESC:` prose, already rendered on the Rust side by | * The record's corpus description prose, already rendered on the Rust side by |
| 53 | * `render_pcgen_desc` treatment the spell catalog uses, which is what | * description treatment the spell catalog uses, which is what |

## `apps/desktop/src/boundary/loadMonsterCatalog.ts`

| line | before | after |
|---:|---|---|
| 118 | * Dice), or `null` when this monster has no `BONUS:VAR\|SLA_CL\|` token on | * Dice), or `null` when this monster states no spell-like-ability caster level on |

## `apps/desktop/src/characterHub/CharacterSheet.tsx`

| line | before | after |
|---:|---|---|
| 1722 | * shows `race_trait_picker::render_trait_description`'s output — corpus `DESC:` | * shows `race_trait_picker::render_trait_description`'s output — corpus description |
| 2134 | // SD31-D7-PROSE-003: the real corpus `DESC:` text, fetched once and joined | // SD31-D7-PROSE-003: the real corpus description text, fetched once and joined |
| 2259 | SD31-D7-PROSE-003: the real rulebook `DESC:` text, when the | SD31-D7-PROSE-003: the real rulebook description text, when the |

## `apps/desktop/src/characterHub/CreateCharacterForm.tsx`

| line | before | after |
|---:|---|---|
| 302 | // each selected fixed-choice `%LIST` trait, keyed by trait id. A trait | // each selected fixed-choice open-slot trait, keyed by trait id. A trait |
| 922 | only the 53 `ultimate_campaign` traits whose `BONUS:SKILL`, | only the 53 `ultimate_campaign` traits whose skill, |
| 923 | `BONUS:SAVE`, `BONUS:SITUATION`, `BONUS:COMBAT\|INITIATIVE`/ | saving-throw, situational, initiative / |
| 924 | `BONUS:CONCENTRATION\|ALLSPELLS`, ability-score-difference | concentration bonus, ability-score-difference |

## `apps/desktop/src/characterHub/alternateTraitSelection.test.ts`

| line | before | after |
|---:|---|---|
| 262 | * every alternate's `DESC:` tokens and ships the result on both the menu and | * every alternate's description statements and ships the result on both the menu and |

## `apps/desktop/src/characterHub/alternateTraitSelection.ts`

| line | before | after |
|---:|---|---|
| 94 | * row's own `DESC:` tokens — never the stored `data.description`, whose | * row's own description statements — never the stored `data.description`, whose |
| 112 | * `DESC:` arguments the engine could not resolve to a literal and therefore | * Description slots the engine could not resolve to a literal and therefore |

## `apps/desktop/src/characterHub/classFeaturesModel.ts`

| line | before | after |
|---:|---|---|
| 83 | * The real rulebook `DESC:` text for this feature, or `null`. | * The real rulebook description text for this feature, or `null`. |

## `apps/desktop/src/characterHub/composeCreateCharacterRequest.test.ts`

| line | before | after |
|---:|---|---|
| 28 | * `Racial Ability Scores` rows carry `BONUS:ABILITYPOOL\|Ability Bonus\|1`. | * `Racial Ability Scores` rows carry one `Ability Bonus` pool grant. |

## `apps/desktop/src/characterHub/composeCreateCharacterRequest.ts`

| line | before | after |
|---:|---|---|
| 98 | * The player's resolved skill choice for each *fixed-choice* `%LIST` | * The player's resolved skill choice for each *fixed-choice* open-slot |

## `apps/desktop/src/characterHub/featPickerEligibility.test.ts`

| line | before | after |
|---:|---|---|
| 69 | 'not verified: one of its alternatives could not be evaluated (references a PCGen runtime variable this engine does not model) (PREMULT:1,[PREVARGTEQ:PreStatScore_INT,13],[PREVARGTEQ:CombatFeatIntRequirement,13])', | 'not verified: the referenced rules variable has no converted table (Intelligence 13 or higher, or the combat-feat Intelligence requirement is at least 13)', |

## `apps/desktop/src/characterHub/itemPickerFilter.test.ts`

| line | before | after |
|---:|---|---|
| 16 | * Real catalog rows. `description` is each record's verbatim corpus `DESC:` | * Real catalog rows. `description` is each record's verbatim corpus description |
| 32 | // carries no SCHOOL:/CLASSES:/DESC: token. | // states no school, class list or description. |
| 41 | * `DESC:` token. | * description. |
| 238 | 'a record whose corpus row has no DESC: shows only book and category, never fabricated text' | 'a record whose corpus row states no description shows only book and category, never fabricated text' |

## `apps/desktop/src/characterHub/itemPickerFilter.ts`

| line | before | after |
|---:|---|---|
| 106 | // The corpus `DESC:` prose the Rust adapter already renders. Until this | // The corpus description prose the Rust adapter already renders. Until this |
| 187 | // record has no `DESC:` token (a real gap: CRB's "Heighten Spell +N" | // record states no description (a real gap: CRB's "Heighten Spell +N" |

## `apps/desktop/src/characterHub/previewData.ts`

| line | before | after |
|---:|---|---|
| 46 | // the engine from the corpus row's own `DESC:` tokens against this | // the engine from the corpus row's own description statements against this |

## `apps/desktop/src/characterHub/raceRoster.ts`

| line | before | after |
|---:|---|---|
| 18 | * corpus on four races' ability modifiers, because `BONUS:STAT\|CON,WIS\|2` | * corpus on four races' ability modifiers, because a +2 Con/Wis adjustment |

## `apps/desktop/src/characterHub/racialTraitsModel.ts`

| line | before | after |
|---:|---|---|
| 31 | * 3. **Absence is rendered as absence.** A dropped `DESC:` argument means the | * 3. **Absence is rendered as absence.** A dropped description slot means the |
| 54 | /** `DESC:` arguments the engine could not resolve, so a gap is visible. */ | /** Description slots the engine could not resolve, so a gap is visible. */ |

## `apps/desktop/src/characterHub/spellsTabModel.test.ts`

| line | before | after |
|---:|---|---|
| 57 | // resolves, but whose corpus row carries no SCHOOL:/CLASSES:/DESC:. | // resolves, but whose corpus row states no school, class list or description. |

## `apps/desktop/src/companionCatalog/CompanionCatalogScreen.test.ts`

| line | before | after |
|---:|---|---|
| 36 | * 4. A `BONUS:STAT` value is an ADJUSTMENT. `STR +6` under a heading reading | * 4. A stated ability-score value is an ADJUSTMENT. `STR +6` under a heading reading |
| 300 | DAMAGE_BONUS_CAPTION.includes('BONUS:WEAPONPROF'), | DAMAGE_BONUS_CAPTION.includes('as the corpus states it'), |
| 325 | SKILL_BONUS_CAPTION.includes('BONUS:SKILL'), | SKILL_BONUS_CAPTION.includes('as the corpus states it'), |

## `apps/desktop/src/companionCatalog/CompanionCatalogScreen.tsx`

| line | before | after |
|---:|---|---|
| 159 | * Load-bearing, not decoration: the corpus states `BONUS:STAT\|STR\|6` and a | * Load-bearing, not decoration: the corpus states a +6 Strength adjustment and a |
| 164 | export const STAT_ADJUSTMENT_CAPTION = 'Ability score adjustments (corpus BONUS:STAT tokens)'; | export const STAT_ADJUSTMENT_CAPTION = 'Ability score adjustments (as the corpus states them)'; |
| 172 | export const DAMAGE_BONUS_CAPTION = 'Extra damage on attack (corpus BONUS:WEAPONPROF DAMAGE tokens)'; | export const DAMAGE_BONUS_CAPTION = 'Extra damage on attack (as the corpus states it)'; |
| 196 | export const SKILL_BONUS_CAPTION = 'Skill bonus from ability difference (corpus BONUS:SKILL tokens)'; | export const SKILL_BONUS_CAPTION = 'Skill bonus from ability difference (as the corpus states it)'; |
| 214 | * `DESC:` argument — the same fact `render_pcgen_desc` drops the `%1` | * description slot — the same fact the description render drops the |

## `apps/desktop/src/companionCatalog/companionCatalogRuntime.ts`

| line | before | after |
|---:|---|---|
| 41 | // The Griffon's own row states no `BONUS:SKILL` ability-difference | // The Griffon's own row states no ability-difference skill |
| 102 | // familiar_clockwork_spy.json`'s own `BONUS:SKILL\|Climb,Swim\|DEX-STR` | // familiar_clockwork_spy.json`'s own Climb/Swim Dex-minus-Str skill bonus |

## `apps/desktop/src/equipmentCatalog/EquipmentCatalogScreen.tsx`

| line | before | after |
|---:|---|---|
| 58 | * `null` is the corpus's honest "this row has no `DESC:` token" — 974 of the | * `null` is the corpus's honest "this row states no description" — 974 of the |
| 131 | * Records carrying real corpus `DESC:` prose. Derived from what loaded, | * Records carrying real corpus description prose. Derived from what loaded, |

## `apps/desktop/src/equipmentCatalog/equipmentCatalogRuntime.ts`

| line | before | after |
|---:|---|---|
| 15 | * Each row's `description` is that record's **real** corpus `DESC:` prose, | * Each row's `description` is that record's **real** corpus description prose, |

## `apps/desktop/src/monsterCatalog/MonsterCatalogScreen.tsx`

| line | before | after |
|---:|---|---|
| 441 | BONUS:VAR\|SLA_CL\| token at all -- never a bare number | stated spell-like-ability caster level at all -- never a bare number |

## `apps/desktop/src/monsterCatalog/monsterCatalogRuntime.ts`

| line | before | after |
|---:|---|---|
| 219 | // Allip's row carries no `BONUS:VAR\|SLA_CL\|` token -- its abilities | // Allip's row states no spell-like-ability caster level -- its abilities |
| 262 | // No `BONUS:VAR\|SLA_CL\|` token on this row: its grants state their own | // No spell-like-ability caster level on this row: its grants state their own |

## `apps/desktop/src/raceCatalog/AlternateTraitPicker.tsx`

| line | before | after |
|---:|---|---|
| 73 | * row's own `DESC:` tokens against a character's display values, never the | * row's own description statements against a character's display values, never the |
| 549 | * - `droppedArgs` names `DESC:` arguments the engine could not resolve. No | * - `droppedArgs` names description slots the engine could not resolve. No |
