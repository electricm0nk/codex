//! The mapping table, TRANSCRIBED from
//! `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/token-mapping/mapping-table.v1.json`
//! (249 rows, `decisions.md` §15). Every `token_type` string below is the JSON row's
//! `token_type` verbatim so `tests::table_is_a_transcription_of_the_json` can prove the
//! transcription is complete in both directions. A mapping that is not in this table is a table
//! defect to record (`_refused.json`, per token type), never a rule invented inside a cycle.
//!
//! `maps_to` is the row's coarse disposition; the per-head conversion logic lives in the sibling
//! modules (`formula`, `prereq`, `prose`, `convert`), each of which cites the row it transcribes
//! at the arm that implements it.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Family {
    Bonus,
    Formula,
    Prereq,
    Prose,
    Synthesis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapsTo {
    Number,
    Text,
    Dice,
    Applies,
    Metadata,
    Refuse,
    Choice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Row {
    pub token_type: &'static str,
    pub family: Family,
    pub maps_to: MapsTo,
}

pub const ROWS: &[Row] = &[
    Row { token_type: "BONUS:VAR", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:VAR (TYPE=Boolean flag)", family: Family::Bonus, maps_to: MapsTo::Text },
    Row { token_type: "BONUS:VAR ([redacted PI] value)", family: Family::Bonus, maps_to: MapsTo::Refuse },
    Row { token_type: "BONUS:STAT", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:ABILITYPOOL", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:SKILL", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:WEAPONPROF=<name>", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:SPELLCAST", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:COMBAT", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:SPELLKNOWN", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:SITUATION", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:SAVE", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:CASTERLEVEL", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:SIZEMOD", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:MOVEADD", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:POSTMOVEADD", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:MOVEMULT", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:[redacted PI]", family: Family::Bonus, maps_to: MapsTo::Refuse },
    Row { token_type: "BONUS:PCLEVEL", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:SKILLPOOL", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:SPECIALTYSPELLKNOWN", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:SKILLRANK", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:SLOTS", family: Family::Bonus, maps_to: MapsTo::Metadata },
    Row { token_type: "BONUS:MISC", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:HP", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:DOMAIN", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:VISION", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:DC", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:DR", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:CONCENTRATION", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:SKILLPOINTS", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:WIELDCATEGORY", family: Family::Bonus, maps_to: MapsTo::Text },
    Row { token_type: "BONUS:RANGEADD", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:RANGEMULT", family: Family::Bonus, maps_to: MapsTo::Text },
    Row { token_type: "BONUS:WEAPON", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:LANGUAGES", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:FOLLOWERS", family: Family::Bonus, maps_to: MapsTo::Metadata },
    Row { token_type: "BONUS:MONSKILLPTS", family: Family::Bonus, maps_to: MapsTo::Metadata },
    Row { token_type: "BONUS:FEAT", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:EQMARMOR", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "BONUS:UDAM", family: Family::Bonus, maps_to: MapsTo::Number },
    Row { token_type: "DEFINE", family: Family::Formula, maps_to: MapsTo::Metadata },
    Row { token_type: "DEFINE (PI-redacted token)", family: Family::Formula, maps_to: MapsTo::Refuse },
    Row { token_type: "DEFINESTAT", family: Family::Formula, maps_to: MapsTo::Metadata },
    Row { token_type: "FORMULA:ability modifier (STR DEX CON INT WIS CHA as an identifier in any formula position)", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "FORMULA:ability score (STRSCORE..CHASCORE)", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "FORMULA:TL", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "FORMULA:CL, bare classlevel(), classlevel(\"APPLIEDAS=NONEPIC\")", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "FORMULA:classlevel(\"X\"), cl(\"X\"), var(\"CL=X\")", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "FORMULA:HD", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "FORMULA:BAB", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "FORMULA:SIZE, SIZEMOD", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "FORMULA:CR", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "FORMULA:CASTERLEVEL, SPELLLEVEL (bare identifiers)", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "FORMULA:integer literal, + - * / and parentheses, unary minus", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "FORMULA:fractional literal (0.5, 1.5, 1/3, X*3/4)", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "FORMULA:max(...), min(...) (also MAX, MIN)", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "FORMULA:floor(x)", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "FORMULA:ceil(x)", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "FORMULA:if(cond, a, b) and comparison arithmetic ((X>=k)*2)", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "FORMULA:charbonusto(\"PCLEVEL\", \"X\")", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "FORMULA:var(\"<export token>\")", family: Family::Formula, maps_to: MapsTo::Refuse },
    Row { token_type: "FORMULA:skillinfo(\"RANK\"|\"TOTALRANK\"|\"TOTAL\", \"<skill>\")", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "FORMULA:count(\"ABILITIES\", \"CATEGORY=..\", \"KEY=..|TYPE=..\")", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "FORMULA:mastervar(\"X\") / MASTERVAR / MasterLevel", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "FORMULA:%CHOICE, %LIST", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "FORMULA:%SPELLLEVEL %CASTERLEVEL %CHARGES %SPELLCOST %SPELLXPCOST BASECOST (equipment-modifier pricing)", family: Family::Formula, maps_to: MapsTo::Metadata },
    Row { token_type: "FORMULA:corpus variable, SAME-record contributors only", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "FORMULA:corpus variable, CROSS-record contributors (the resolution rule)", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "FORMULA:variable with a contributor row the corpus does NOT hold (in an ingested file)", family: Family::Formula, maps_to: MapsTo::Refuse },
    Row { token_type: "FORMULA:identifier DEFINEd nowhere (corpus or pinned PCGen data)", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "FORMULA:malformed (parser refusals)", family: Family::Formula, maps_to: MapsTo::Refuse },
    Row { token_type: "DAMAGE / ALTDAMAGE", family: Family::Formula, maps_to: MapsTo::Dice },
    Row { token_type: "CRITRANGE / ALTCRITRANGE", family: Family::Formula, maps_to: MapsTo::Text },
    Row { token_type: "CRITMULT / ALTCRITMULT", family: Family::Formula, maps_to: MapsTo::Text },
    Row { token_type: "UDAM", family: Family::Formula, maps_to: MapsTo::Dice },
    Row { token_type: "UMULT", family: Family::Formula, maps_to: MapsTo::Text },
    Row { token_type: "RANGE (spell / power keyword)", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "WT", family: Family::Formula, maps_to: MapsTo::Metadata },
    Row { token_type: "COST (literal)", family: Family::Formula, maps_to: MapsTo::Metadata },
    Row { token_type: "COST (formula, equipment_modifier)", family: Family::Formula, maps_to: MapsTo::Metadata },
    Row { token_type: "ACCHECK", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "SPELLFAILURE", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "MAXDEX", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "MOVE", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "MOVECLONE", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "UNENCUMBEREDMOVE", family: Family::Formula, maps_to: MapsTo::Text },
    Row { token_type: "REACH", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "HITDIE (n, with optional class filter)", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "HITDIE (%-step: %+1, %/4)", family: Family::Formula, maps_to: MapsTo::Refuse },
    Row { token_type: "HD (class)", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "HD (template band: <lo>-<hi>:TEMPLATE:<name>)", family: Family::Formula, maps_to: MapsTo::Applies },
    Row { token_type: "DR", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "SR", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "CR", family: Family::Formula, maps_to: MapsTo::Text },
    Row { token_type: "CRMOD", family: Family::Formula, maps_to: MapsTo::Metadata },
    Row { token_type: "SPELLS", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "SPELLS (PI-redacted token)", family: Family::Formula, maps_to: MapsTo::Refuse },
    Row { token_type: "SPELLKNOWN", family: Family::Formula, maps_to: MapsTo::Metadata },
    Row { token_type: "SPELLLEVEL", family: Family::Formula, maps_to: MapsTo::Metadata },
    Row { token_type: "TEMPBONUS", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "TEMPVALUE", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "TEMPDESC", family: Family::Formula, maps_to: MapsTo::Text },
    Row { token_type: "DESC / BENEFIT %N argument (a formula inside a prose token)", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "SAB", family: Family::Formula, maps_to: MapsTo::Text },
    Row { token_type: "SPROP", family: Family::Formula, maps_to: MapsTo::Text },
    Row { token_type: "SIZE", family: Family::Formula, maps_to: MapsTo::Metadata },
    Row { token_type: "SIZE (formula)", family: Family::Formula, maps_to: MapsTo::Refuse },
    Row { token_type: "LEGS / HANDS", family: Family::Formula, maps_to: MapsTo::Metadata },
    Row { token_type: "VISION", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "PLUS", family: Family::Formula, maps_to: MapsTo::Metadata },
    Row { token_type: "ADDLEVEL", family: Family::Formula, maps_to: MapsTo::Metadata },
    Row { token_type: "LEVELSPERFEAT", family: Family::Formula, maps_to: MapsTo::Metadata },
    Row { token_type: "KEYSTAT", family: Family::Formula, maps_to: MapsTo::Metadata },
    Row { token_type: "ITEM", family: Family::Formula, maps_to: MapsTo::Metadata },
    Row { token_type: "SITUATION", family: Family::Formula, maps_to: MapsTo::Text },
    Row { token_type: "EQMOD / ALTEQMOD", family: Family::Formula, maps_to: MapsTo::Metadata },
    Row { token_type: "PROFICIENCY / WIELD", family: Family::Formula, maps_to: MapsTo::Metadata },
    Row { token_type: "CONTAINS / CHARGES / BASEITEM / BASEQTY / MODS / FUMBLERANGE", family: Family::Formula, maps_to: MapsTo::Metadata },
    Row { token_type: "STAT: / AC: / LEVELADJUSTMENT: (listed in v06_work_inventory's magnitude_tokens)", family: Family::Formula, maps_to: MapsTo::Metadata },
    Row { token_type: "PREABILITY", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREMULT", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREVARGTEQ", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREVARGT", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREVARLT", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREVARLTEQ", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREVAREQ", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREVARNEQ", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PRECLASS", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PRECLASSLEVELMAX", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PRESUBCLASS", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PRESTAT", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PRESKILL", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PRECSKILL", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PRETOTALAB", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PRECHECKBASE", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PRELEVEL", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREPCLEVEL", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PRELEVELMAX", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREHD", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PRERACE", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREFACT", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PRETEMPLATE", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PRESIZEEQ", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PRESIZEGTEQ", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PRESIZELTEQ", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PRESIZELT", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREBASESIZEGTEQ", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREBASESIZEEQ", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREBASESIZELT", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREALIGN", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREDEITY", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREDEITYDOMAIN", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREDEITYALIGN", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREDOMAIN", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREWEAPONPROF", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREPROFWITHSHIELD", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREARMORTYPE", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PRESPELL", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PRESPELLTYPE", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PRESPELLCAST", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PRESPELLBOOK", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PRESPELLDESCRIPTOR", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREMOVE", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREVISION", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PRELANG", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREGENDER", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREAGESET", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREEQUIP", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREDR", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREHANDSGTEQ", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PREREACHGTEQ", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PRETEXT", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PRETYPE", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PRERULE", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "PRECAMPAIGN", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "PRECHARACTERTYPE", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "PRE", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "!PRE* (all negated forms)", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "feat.prerequisites (token-less feat records)", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "KEY", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "CATEGORY", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "TYPE", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "VISIBLE", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "ABILITY", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "AUTO", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "TEMPLATE", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "KIT", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "CSKILL", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "CCSKILL", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "MONCSKILL", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "CLASSES", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "DOMAINS", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "RACETYPE", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "RACESUBTYPE", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "SUBRACE", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "FACT / FACTSET", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "SERVESAS", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "QUALIFY", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "REMOVE", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "ADD", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "CHOOSE", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "SELECT", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "MULT", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "STACK", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "LANGBONUS", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "STARTFEATS", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "LEVELSPERFEAT", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "class LEVEL grants (CLASS .lst level lines)", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: ".MOD / .COPY closure rows", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "trailing |PRE... on host tokens (per-addend gates)", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "DESC", family: Family::Prose, maps_to: MapsTo::Text },
    Row { token_type: "DESC:.CLEAR", family: Family::Prose, maps_to: MapsTo::Text },
    Row { token_type: "BENEFIT", family: Family::Prose, maps_to: MapsTo::Text },
    Row { token_type: "ASPECT:<display sub-key> (Ability Benefit, AbilityBenefit, SaveBonus, CombatBonus, SkillBonus, RacialSkillModifier, Immunity, Resistance, ResistanceOutput, UnarmedNotes, Vision, Maneuverability, ModifyAC, CMBCircumstance, NaturalAttack*, Typical Alignment, Ancestry, Alternate *, Skill Modifiers, Spell-Like Ability, *SaveBonus, Racial Points)", family: Family::Prose, maps_to: MapsTo::Text },
    Row { token_type: "ASPECT:CheckCount / ASPECT:CheckType", family: Family::Prose, maps_to: MapsTo::Number },
    Row { token_type: "ASPECT:NAME", family: Family::Prose, maps_to: MapsTo::Text },
    Row { token_type: "ASPECT:<structural sub-key> (ChildAbility, MasterAbility, Archetype Base Class, Bloodline, StatBlockName, SourceBook)", family: Family::Prose, maps_to: MapsTo::Metadata },
    Row { token_type: "SAB", family: Family::Prose, maps_to: MapsTo::Text },
    Row { token_type: "SPROP", family: Family::Prose, maps_to: MapsTo::Text },
    Row { token_type: "TEMPDESC", family: Family::Prose, maps_to: MapsTo::Text },
    Row { token_type: "OUTPUTNAME", family: Family::Prose, maps_to: MapsTo::Text },
    Row { token_type: "NAMEISPI / DESCISPI (and the corpus `license`/`pi_field`/`pi_marker` stamps)", family: Family::Prose, maps_to: MapsTo::Metadata },
    Row { token_type: "SORTKEY", family: Family::Prose, maps_to: MapsTo::Metadata },
    Row { token_type: "SOURCEPAGE / SOURCELONG / SOURCESHORT / SOURCEWEB / SOURCEDATE / SOURCELINK", family: Family::Prose, maps_to: MapsTo::Metadata },
    Row { token_type: "(record with no raw_tokens / empty raw_tokens)", family: Family::Prose, maps_to: MapsTo::Text },
    Row { token_type: "NATURALATTACKS", family: Family::Formula, maps_to: MapsTo::Dice },
    Row { token_type: "REACHMULT", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "MONSTERCLASS", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "MAXLEVEL", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "SCHOOL", family: Family::Prose, maps_to: MapsTo::Text },
    Row { token_type: "SUBSCHOOL", family: Family::Prose, maps_to: MapsTo::Text },
    Row { token_type: "DESCRIPTOR", family: Family::Prose, maps_to: MapsTo::Text },
    Row { token_type: "DURATION", family: Family::Prose, maps_to: MapsTo::Text },
    Row { token_type: "CASTTIME", family: Family::Prose, maps_to: MapsTo::Text },
    Row { token_type: "TARGETAREA", family: Family::Prose, maps_to: MapsTo::Text },
    Row { token_type: "COMPS", family: Family::Prose, maps_to: MapsTo::Text },
    Row { token_type: "SAVEINFO", family: Family::Prose, maps_to: MapsTo::Text },
    Row { token_type: "SPELLRES", family: Family::Prose, maps_to: MapsTo::Text },
    Row { token_type: "DEITYWEAP", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "ALIGN", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "COMPANIONLIST", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "FOLLOWERS", family: Family::Formula, maps_to: MapsTo::Number },
    Row { token_type: "USEUNTRAINED", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "QUALITY", family: Family::Prose, maps_to: MapsTo::Text },
    Row { token_type: "ROLE", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "NAMEOPT / ITYPE / REPLACES / FORMATCAT / ASSIGNTOALL", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "REGION / REMOVABLE / VARIANTS / INFO / EXCLUSIVE / ALLOWBASECLASS / EXCLASS / WEAPONBONUS / ACHECK / CHANGEPROF / ADDSPELLLEVEL", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "%CHOICE / %LIST (all positions)", family: Family::Synthesis, maps_to: MapsTo::Choice },
    // ---- SD-35 AT-35-E4-001: the 25 heads the census carried with no row -----------------
    // Every head below was in `token-coverage.json`'s `unmapped_token_types` at `07e29075b4`
    // and degraded 974 records for want of a row, not for want of a readable rule. None is a
    // term of a sheet total, so none is a `Number` and no oracle comparison is owed.
    Row { token_type: "BONUS:EQM", family: Family::Bonus, maps_to: MapsTo::Metadata },
    Row { token_type: "BONUS:EQMWEAPON", family: Family::Bonus, maps_to: MapsTo::Metadata },
    Row { token_type: "BONUS:ITEMCOST", family: Family::Bonus, maps_to: MapsTo::Metadata },
    Row { token_type: "[redacted PI] token", family: Family::Prose, maps_to: MapsTo::Metadata },
    Row { token_type: "ALTTYPE", family: Family::Formula, maps_to: MapsTo::Metadata },
    Row { token_type: "ARMORTYPE", family: Family::Formula, maps_to: MapsTo::Metadata },
    Row { token_type: "BONUSSPELLSTAT", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "DOMAIN", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "GROUP", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "ITEMCREATE", family: Family::Formula, maps_to: MapsTo::Metadata },
    Row { token_type: "KNOWNSPELLS", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "MEMORIZE", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "MODTOSKILLS", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "MONCCSKILL", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "NUMPAGES", family: Family::Formula, maps_to: MapsTo::Metadata },
    Row { token_type: "PAGEUSAGE", family: Family::Formula, maps_to: MapsTo::Metadata },
    Row { token_type: "PRESPELLSCHOOL", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PRESPELLSCHOOLSUB", family: Family::Prereq, maps_to: MapsTo::Applies },
    Row { token_type: "PROHIBITSPELL", family: Family::Prose, maps_to: MapsTo::Text },
    Row { token_type: "SLOTS", family: Family::Formula, maps_to: MapsTo::Metadata },
    Row { token_type: "SPELLBOOK", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "SPELLLIST", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "SPELLSTAT", family: Family::Prereq, maps_to: MapsTo::Metadata },
    Row { token_type: "STARTSKILLPTS", family: Family::Prereq, maps_to: MapsTo::Metadata },
];

pub fn row(token_type: &str) -> Option<&'static Row> {
    ROWS.iter().find(|r| r.token_type == token_type)
}

/// The table row a raw token head (with its value's first field, for `BONUS:`) is transcribed
/// under, or `None` when the head has no row -- the converter refuses such a token as
/// `unmapped:<HEAD>`.
pub fn row_for_head(head: &str, value: &str) -> Option<&'static Row> {
    let head = head.trim();
    if let Some(neg) = head.strip_prefix('!')
        && neg.starts_with("PRE")
    {
        return row("!PRE* (all negated forms)");
    }
    let key: &str = match head {
        "BONUS" => {
            let sub = value.split('|').next().unwrap_or("").trim();
            let sub_head = sub.split('=').next().unwrap_or("").trim();
            if sub_head.starts_with("[redacted") {
                return row("BONUS:[redacted PI]");
            }
            return match sub_head {
                "WEAPONPROF" => row("BONUS:WEAPONPROF=<name>"),
                other => {
                    let owned = format!("BONUS:{other}");
                    ROWS.iter().find(|r| r.token_type == owned)
                }
            };
        }
        "DEFINE" => {
            if value.contains("[redacted") { "DEFINE (PI-redacted token)" } else { "DEFINE" }
        }
        "SPELLS" => {
            if value.contains("[redacted") { "SPELLS (PI-redacted token)" } else { "SPELLS" }
        }
        "DESC" => {
            if value.trim() == ".CLEAR" { "DESC:.CLEAR" } else { "DESC" }
        }
        "ASPECT" => {
            let sub = value.split('|').next().unwrap_or("").trim();
            return Some(aspect_row(sub));
        }
        "DAMAGE" | "ALTDAMAGE" => "DAMAGE / ALTDAMAGE",
        "CRITRANGE" | "ALTCRITRANGE" => "CRITRANGE / ALTCRITRANGE",
        "CRITMULT" | "ALTCRITMULT" => "CRITMULT / ALTCRITMULT",
        "RANGE" => "RANGE (spell / power keyword)",
        "COST" => "COST (literal)",
        "HITDIE" => {
            if value.contains('%') { "HITDIE (%-step: %+1, %/4)" } else { "HITDIE (n, with optional class filter)" }
        }
        "HD" => {
            if value.contains("TEMPLATE") { "HD (template band: <lo>-<hi>:TEMPLATE:<name>)" } else { "HD (class)" }
        }
        "SIZE" => {
            let v = value.trim();
            if v.chars().all(|c| c.is_ascii_alphabetic()) { "SIZE" } else { "SIZE (formula)" }
        }
        "LEGS" | "HANDS" => "LEGS / HANDS",
        "EQMOD" | "ALTEQMOD" => "EQMOD / ALTEQMOD",
        "PROFICIENCY" | "WIELD" => "PROFICIENCY / WIELD",
        "CONTAINS" | "CHARGES" | "BASEITEM" | "BASEQTY" | "MODS" | "FUMBLERANGE" => {
            "CONTAINS / CHARGES / BASEITEM / BASEQTY / MODS / FUMBLERANGE"
        }
        "STAT" | "AC" | "LEVELADJUSTMENT" => {
            "STAT: / AC: / LEVELADJUSTMENT: (listed in v06_work_inventory's magnitude_tokens)"
        }
        "FACT" | "FACTSET" => "FACT / FACTSET",
        "NAMEISPI" | "DESCISPI" => {
            "NAMEISPI / DESCISPI (and the corpus `license`/`pi_field`/`pi_marker` stamps)"
        }
        "SOURCEPAGE" | "SOURCELONG" | "SOURCESHORT" | "SOURCEWEB" | "SOURCEDATE" | "SOURCELINK" => {
            "SOURCEPAGE / SOURCELONG / SOURCESHORT / SOURCEWEB / SOURCEDATE / SOURCELINK"
        }
        "NAMEOPT" | "ITYPE" | "REPLACES" | "FORMATCAT" | "ASSIGNTOALL" => {
            "NAMEOPT / ITYPE / REPLACES / FORMATCAT / ASSIGNTOALL"
        }
        "REGION" | "REMOVABLE" | "VARIANTS" | "INFO" | "EXCLUSIVE" | "ALLOWBASECLASS" | "EXCLASS"
        | "WEAPONBONUS" | "ACHECK" | "CHANGEPROF" | "ADDSPELLLEVEL" => {
            "REGION / REMOVABLE / VARIANTS / INFO / EXCLUSIVE / ALLOWBASECLASS / EXCLASS / WEAPONBONUS / ACHECK / CHANGEPROF / ADDSPELLLEVEL"
        }
        // Head alias: the table's PRERACE row carries the `RACETYPE=<t>` clause this head
        // spells as its own token (37 units; recorded as a table defect in the cycle receipt).
        "PRERACETYPE" => "PRERACE",
        // Head alias (SD-35 AT-35-E4-001): the ingest's own key for an ABILITY token a
        // global-variable contributor row added to this record. The value is an ABILITY token
        // body verbatim, so it reads under the ABILITY row.
        "GLOBALVAR:ABILITY" => "ABILITY",
        other => other,
    };
    row(key)
}

/// The `ASPECT:<sub-key>` row for one sub-key: the uses row, the name row, the structural
/// (bookkeeping) row, or the display row.
pub fn aspect_row(sub_key: &str) -> &'static Row {
    let s = sub_key.trim();
    if s.ends_with("CheckCount") || s.ends_with("CheckType") {
        return row("ASPECT:CheckCount / ASPECT:CheckType").unwrap();
    }
    if s == "NAME" {
        return row("ASPECT:NAME").unwrap();
    }
    if is_structural_aspect(s) {
        return row("ASPECT:<structural sub-key> (ChildAbility, MasterAbility, Archetype Base Class, Bloodline, StatBlockName, SourceBook)").unwrap();
    }
    ROWS.iter().find(|r| r.token_type.starts_with("ASPECT:<display sub-key>")).unwrap()
}

pub fn is_structural_aspect(sub_key: &str) -> bool {
    matches!(
        sub_key,
        "ChildAbility" | "MasterAbility" | "Archetype Base Class" | "Bloodline" | "StatBlockName" | "SourceBook"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn table_is_a_transcription_of_the_json() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/token-mapping/mapping-table.v1.json",
        );
        let text = std::fs::read_to_string(&path).expect("mapping-table.v1.json is readable");
        let json: serde_json::Value = serde_json::from_str(&text).unwrap();
        let json_types: BTreeSet<String> = json["rows"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["token_type"].as_str().unwrap().to_string())
            .collect();
        let rust_types: BTreeSet<String> = ROWS.iter().map(|r| r.token_type.to_string()).collect();
        assert_eq!(json["rows"].as_array().unwrap().len(), 273, "the table has 273 rows (decisions.md §15's 249 plus AT-35-E4-001's 24)");
        assert_eq!(ROWS.len(), 273, "every JSON row is transcribed, duplicates included");
        // 273 rows spell 269 distinct token types: four rows repeat a type string (two families
        // wrote a row for the same head); the transcription keeps all 273.
        assert_eq!(json_types.len(), 269);
        let missing: Vec<_> = json_types.difference(&rust_types).collect();
        let extra: Vec<_> = rust_types.difference(&json_types).collect();
        assert!(missing.is_empty() && extra.is_empty(), "missing={missing:?} extra={extra:?}");
        for r in json["rows"].as_array().unwrap() {
            let tt = r["token_type"].as_str().unwrap();
            let maps_to = r["maps_to"].as_str().unwrap();
            let rust = row(tt).unwrap();
            let expect = if maps_to.starts_with("REFUSE") {
                MapsTo::Refuse
            } else if maps_to.starts_with("Metadata") {
                MapsTo::Metadata
            } else if maps_to.starts_with("Number") {
                MapsTo::Number
            } else if maps_to.starts_with("Text") {
                MapsTo::Text
            } else if maps_to.starts_with("Dice") {
                MapsTo::Dice
            } else if maps_to.starts_with("Applies") {
                MapsTo::Applies
            } else {
                MapsTo::Choice
            };
            assert_eq!(rust.maps_to, expect, "{tt}: maps_to drifted from the JSON");
        }
    }

    #[test]
    fn heads_route_to_their_rows() {
        assert_eq!(row_for_head("BONUS", "VAR|X|1").unwrap().token_type, "BONUS:VAR");
        assert_eq!(row_for_head("BONUS", "WEAPONPROF=Bite|DAMAGE|1").unwrap().token_type, "BONUS:WEAPONPROF=<name>");
        assert_eq!(row_for_head("!PREABILITY", "1,x").unwrap().token_type, "!PRE* (all negated forms)");
        assert_eq!(row_for_head("ASPECT", "CheckCount|%1|X").unwrap().maps_to, MapsTo::Number);
        assert_eq!(row_for_head("ASPECT", "SourceBook|X").unwrap().maps_to, MapsTo::Metadata);
        assert_eq!(row_for_head("ASPECT", "Ability Benefit|+2").unwrap().maps_to, MapsTo::Text);
        assert_eq!(row_for_head("DESC", ".CLEAR").unwrap().token_type, "DESC:.CLEAR");
        assert_eq!(row_for_head("HD", "10").unwrap().token_type, "HD (class)");
        assert_eq!(row_for_head("SIZE", "M").unwrap().token_type, "SIZE");
        assert!(row_for_head("NOSUCHTOKEN", "").is_none());
    }
}
