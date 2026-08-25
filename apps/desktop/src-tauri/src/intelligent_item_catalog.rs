//! Player-facing reference surface for PF1's intelligent/legendary item
//! build system (SD-31 wave-18, `intelligent_items:desktop` lane).
//!
//! # Why this module exists
//!
//! An intelligent item's own ability scores, Ego and alignment are real,
//! fully ingested corpus content -- 169 `equipmods` records across
//! `core_rulebook` (the classic Intelligent Item system, CRB p.172-174) and
//! `mythic_adventures` (the parallel Legendary Item system, Mythic
//! Adventures p.172) -- but before this module, nothing in `apps/desktop`
//! rendered them: `grep -rln "Intelligent Item" apps/desktop/src` found only
//! `equipment_catalog.rs`'s own doc comment naming the key family as a
//! within-book duplicate-key example, never a screen a player reads. This
//! module is that missing render path.
//!
//! # Source of truth: the live corpus, not a fixture
//!
//! Reads every `data/corpus/<book>/equipment/equipmods/*.json` record whose
//! `data.key` contains the literal substring `"Intelligent Item"` --
//! `core_rulebook`'s own `Intelligent Item ~ *` / `Intelligent Item
//! Alignment (*)` / `Intelligent Item Purpose (*)` keys and
//! `mythic_adventures`'s `Legendary Item ~ Intelligent Item...` keys all
//! carry it. Matching on the corpus's own semantic identity (the `KEY:`
//! token) rather than a filename convention means a future book that adds
//! more such records is picked up automatically, the same posture
//! `class_feature_descriptions.rs` takes reading `data.class`.
//!
//! # Hidden trigger rows are read but never served as options
//!
//! 17 of the 169 records (9 CRB alignments + 8 CRB purposes) are a `VISIBLE:
//! NO` shadow pair of their spelled-out, purchasable sibling (e.g.
//! `Intelligent Item Alignment (LG)`, hidden, sits beside `Intelligent Item ~
//! Alignment / Lawful Good`, the real EQBUILDER choice) -- confirmed by
//! reading both records' own `raw_tokens`, not assumed from naming. A player
//! never sees the hidden trigger as a choice in PCGen's own item builder
//! either, so serving it here as if it were a fourth "alignment option" would
//! misrepresent the corpus, not merely omit detail. Filtered out entirely by
//! [`load_intelligent_item_components`]; see
//! `hidden_trigger_rows_never_reach_the_served_catalog` (this module's own
//! test) for the exact 17-row list this was checked against.
//!
//! # No fabricated Ego score
//!
//! An item's total Ego score is `sum of every chosen component's Ego
//! contribution`, and which components are chosen is a build-time choice
//! this corpus does not fix for any specific item -- exactly the runtime-
//! context-the-corpus-does-not-fix shape `docs/release/
//! SD-31-corpus-closure-grind` names for monster spell-like abilities. This
//! module ships the FORMULA every component states (a literal integer
//! contribution for most rows; for the shared Base row, the literal
//! price-bracket formula transcribed mechanically from
//! `raw_bonus_chains`, never hand-copied prose -- see
//! [`format_base_ego_price_bands`]) and never a resolved total. Pinned by
//! `no_component_ever_emits_a_fabricated_resolved_total_ego_score` and
//! mutation-proved by `mutation_removing_the_ego_delta_none_guard_would_be_
//! caught_by_the_pin` (both below).
//!
//! # PI screening
//!
//! Every one of the 169 records carries `pi_field: null` / `pi_marker: null`
//! in the live corpus (checked, not assumed --
//! `every_served_record_carries_no_declared_pi_marker`, below) and every
//! rendered `name`/`description` is checked against
//! `codex::rules_core::pi_screening::PI_BLACKLIST_TERMS`, the same live term
//! list `reach_gate.rs` checks served Inner Sea World Guide content against
//! -- `every_served_name_and_description_clears_the_pi_blacklist`, below.
//!
//! # The leak guard
//!
//! Same posture as `class_feature_descriptions.rs`: `render_pcgen_desc`'s
//! output is checked with `leaked_pcgen_syntax` and a leaking row is
//! refused (its description omitted, not the whole record dropped), never
//! shipped or panicked on. This is not hypothetical for this population --
//! `OPEN-ISSUES.md` row 138 already found `core_rulebook:equipment_modifier:
//! IntItemBase`'s ability-score-summary `SPROP` (`"Intelligence %, Wisdom
//! %, Charisma %, Ego Score %|..."`) leaks under this exact check; this
//! module re-derives that refusal independently rather than trusting the
//! prior fix's file (`refuses_the_known_leaking_base_ability_score_sprop`,
//! below).

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde::Serialize;
use serde_json::Value;

use crate::authoring_workbench::codex_repo_root;

/// One purchasable (or, for the single shared Base row, foundational)
/// component of the intelligent/legendary item build system.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IntelligentItemComponentDto {
    /// Corpus book directory this record was read from (`"core_rulebook"`,
    /// `"mythic_adventures"`).
    pub book: String,
    /// Grouping label derived from the `KEY:` token's own structure -- see
    /// [`family_for_key`]. Not a fixed enum: an unanticipated future shape
    /// falls back to its own key text rather than being miscategorized.
    pub family: String,
    /// The corpus `KEY:` token verbatim.
    pub key: String,
    pub name: String,
    pub cost_gp: Option<f64>,
    /// Rendered from the record's `SPROP` token(s) (these records never
    /// carry a `DESC:`), leak-checked, joined when a record states more
    /// than one. `None` when the record has no real prose at all, or when
    /// every candidate leaks unresolved PCGen syntax and is refused.
    pub description: Option<String>,
    /// Every `raw_bonus_chains` `VAR` effect this record states, literally
    /// transcribed -- never evaluated against a hypothetical build.
    pub mechanics: Vec<IntelligentItemMechanicDto>,
    /// Convenience read of `mechanics` for the common case: `Some(n)` only
    /// when this record carries exactly one `IntelligentItemEgo` mechanic
    /// AND that mechanic's formula is a bare integer literal (never the
    /// Base row's price-bracket formula, which has no single number to
    /// report -- see the module doc's "No fabricated Ego score" section).
    pub ego_delta: Option<i32>,
}

/// One literal `VAR` effect from a component's `raw_bonus_chains`.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IntelligentItemMechanicDto {
    /// The corpus `VAR` name verbatim (`"IntelligentItemEgo"`,
    /// `"IntItemStatINT"`, ...) -- kept alongside `effect` so a reader who
    /// knows the token can cross-check it, the same transparency
    /// `formatDamageBonus`/`formatSkillBonus` (`CompanionCatalogScreen.tsx`)
    /// already give an unparsed formula.
    pub variable: String,
    /// Human label for `variable` (`"Ego"`, `"Intelligence"`, ...); falls
    /// back to `variable` itself for a name this module does not recognize,
    /// never a guess.
    pub effect: String,
    /// The formula/value, rendered readably. A bare integer literal renders
    /// signed (`"+2"`, `"-1"`); the Base row's price-bracket formula renders
    /// through [`format_base_ego_price_bands`]; anything else is passed
    /// through [`simplify_formula`] (mechanical `var("X")` -> `X`
    /// unwrapping only, never a hand interpretation).
    pub formula: String,
    /// The gating condition, translated from the token's own `PRE*`
    /// qualifier through [`translate_condition`] when recognized; an
    /// unrecognized `PRE*` shape is still surfaced, prefixed to mark it as
    /// untranslated, rather than silently dropped.
    pub condition: Option<String>,
    /// The bonus-stacking type tag (`TYPE=Purpose`, `TYPE=Boolean`), when
    /// the record states one. Not a condition -- kept separate so a reader
    /// never mistakes a stacking-group tag for a wielder requirement.
    pub bonus_type: Option<String>,
}

/// PF1's 9 short alignment codes -> the words a player reads. A closed,
/// universal ruleset convention (identical to every other alignment display
/// this engine already prints elsewhere), not corpus-derived prose, so it is
/// hardcoded here rather than transcribed per-record.
fn alignment_name(code: &str) -> String {
    match code {
        "LG" => "Lawful Good",
        "NG" => "Neutral Good",
        "CG" => "Chaotic Good",
        "LN" => "Lawful Neutral",
        "TN" | "N" => "True Neutral",
        "NE" => "Neutral Evil",
        "LE" => "Lawful Evil",
        "CE" => "Chaotic Evil",
        "CN" => "Chaotic Neutral",
        other => return other.to_string(),
    }
    .to_string()
}

/// `var("IntItemNegativeLevel")` -> `IntItemNegativeLevel`. Purely
/// mechanical textual unwrapping of PCGen's `var("X")` reference syntax --
/// never an evaluation, never a guess at what `X` resolves to.
fn simplify_formula(formula: &str) -> String {
    let mut out = String::with_capacity(formula.len());
    let mut rest = formula;
    while let Some(start) = rest.find("var(\"") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 5..];
        if let Some(end) = after.find("\")") {
            out.push_str(&after[..end]);
            rest = &after[end + 2..];
        } else {
            out.push_str(&rest[start..]);
            rest = "";
            break;
        }
    }
    out.push_str(rest);
    out
}

/// One `(qualifiers[0] == "VAR" && qualifiers[1] == "IntelligentItemEgo")`
/// formula carries the Base row's price-bracket Ego table as a single
/// arithmetic expression: `(BaseCostTracker>=1001)+(BaseCostTracker>=5001)+
/// ...`. Rather than hand-transcribe PF1's own printed table (a second,
/// independent, error-prone source of truth), this walks the literal
/// `>=N` clauses in the expression, tallies identical thresholds into their
/// coefficient, and renders the result -- so the served text is a direct,
/// mechanical function of the corpus bytes, re-checked against those same
/// bytes by `format_base_ego_price_bands_matches_a_dumb_independent_count_
/// of_the_raw_formula_bytes` below.
fn format_base_ego_price_bands(formula: &str) -> String {
    let mut thresholds: Vec<i64> = Vec::new();
    let mut rest = formula;
    while let Some(start) = rest.find(">=") {
        let after = &rest[start + 2..];
        let digits: String = after.chars().take_while(|c| c.is_ascii_digit()).collect();
        if let Ok(n) = digits.parse::<i64>() {
            thresholds.push(n);
        }
        rest = &after[digits.len()..];
    }
    if thresholds.is_empty() {
        return format!("(unrecognized Ego formula shape) {formula}");
    }
    let mut bands: Vec<(i64, i32)> = Vec::new();
    for t in thresholds {
        if let Some(last) = bands.last_mut() {
            if last.0 == t {
                last.1 += 1;
                continue;
            }
        }
        bands.push((t, 1));
    }
    let parts: Vec<String> = bands
        .iter()
        .map(|(threshold, coefficient)| format!("price \u{2265} {threshold} gp: +{coefficient} Ego"))
        .collect();
    format!("Base Ego from item price (cumulative): {}", parts.join("; "))
}

/// Translates one PCGen `PRE*` qualifier token into plain prose. Covers
/// every condition shape this 169-record population actually uses
/// (`PREALIGN`, `PREVARGTEQ`, `PREVARLTEQ`) plus the generic `PREVARLT` a
/// future record could add; anything else is surfaced verbatim, marked
/// untranslated, rather than dropped -- the same "refuse silently losing
/// information" posture `render_pcgen_desc`'s `dropped_args` takes.
fn translate_condition(token: &str) -> String {
    let (negate, body) = match token.strip_prefix('!') {
        Some(rest) => (true, rest),
        None => (false, token),
    };
    if let Some(rest) = body.strip_prefix("PREALIGN:") {
        let readable = rest.split(',').map(alignment_name).collect::<Vec<_>>().join(" or ");
        return if negate {
            format!("wielder's alignment is not {readable}")
        } else {
            format!("wielder's alignment is {readable}")
        };
    }
    for (prefix, symbol) in [
        ("PREVARGTEQ:", ">="),
        ("PREVARLTEQ:", "<="),
        ("PREVARGT:", ">"),
        ("PREVARLT:", "<"),
    ] {
        if let Some(rest) = body.strip_prefix(prefix) {
            let mut parts = rest.splitn(2, ',');
            let var = parts.next().unwrap_or(rest);
            let n = parts.next().unwrap_or("");
            let clause = format!("{var} {symbol} {n}");
            return if negate { format!("NOT ({clause})") } else { clause };
        }
    }
    format!("(untranslated condition token) {token}")
}

/// Human label for a `raw_bonus_chains` `VAR` name. Falls back to the
/// variable itself for anything unrecognized, never a guess.
fn friendly_var_label(var: &str) -> String {
    match var {
        "IntelligentItemEgo" => "Ego",
        "IntItemStatINT" => "Intelligence",
        "IntItemStatWIS" => "Wisdom",
        "IntItemStatCHA" => "Charisma",
        "SpeechBonusLang" => "Bonus languages known",
        "IntItemSenseRange" | "INTITEMSENSERANGE" => "Sense range (feet)",
        "NegLevels" | "NegativeLevel" | "IntItemNegativeLevel" => "Negative levels while attuned",
        "IntItemAlignment" => "Alignment marker (internal)",
        "IntItemCost" | "BaseCostTracker" => "Item price tracker (internal)",
        "IntItemSpeech" => "Speech flag (internal)",
        "IntItemBlindsense" => "Blindsense flag (internal)",
        "IntItemDarkvision" => "Darkvision flag (internal)",
        "IntItemPowers" => "Powers flag (internal)",
        other => return other.to_string(),
    }
    .to_string()
}

/// `Intelligent Item ~ Ability Score / Charisma 11` -> `"Ability Score"`;
/// `Legendary Item ~ Intelligent Item ~ Sense / Darkvision` -> `"Sense"`;
/// `Intelligent Item Alignment (LG)` -> `"Alignment"`; the shared root
/// records (`Intelligent Item ~ Base`, `Legendary Item ~ Intelligent Item`)
/// -> `"Base"`. Derived from the `KEY:` token's own `~`/`/`/`(` structure,
/// re-checked corpus-wide by `family_for_key_partitions_every_visible_
/// record_into_a_non_empty_family` below rather than assumed exhaustive.
fn family_for_key(key: &str) -> String {
    if key == "Intelligent Item ~ Base" || key == "Legendary Item ~ Intelligent Item" {
        return "Base".to_string();
    }
    if key.contains('~') {
        let after_tilde = key.split_once('~').map(|x| x.1).unwrap_or("").trim();
        // Mythic keys carry a second `Intelligent Item ~` segment
        // (`Legendary Item ~ Intelligent Item ~ Ability Score / ...`); strip
        // it so both books share one family taxonomy.
        let after_tilde = after_tilde.strip_prefix("Intelligent Item ~").map(str::trim).unwrap_or(after_tilde);
        let family = after_tilde.split('/').next().unwrap_or(after_tilde).trim();
        if !family.is_empty() {
            return family.to_string();
        }
    }
    if key.contains("Alignment (") {
        return "Alignment".to_string();
    }
    if key.contains("Purpose (") {
        return "Purpose".to_string();
    }
    key.to_string()
}

fn is_real_prose(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return false;
    }
    let lower = trimmed.to_ascii_lowercase();
    !matches!(lower.as_str(), ".clear" | ".clearall" | "[redacted pi]")
}

/// Every `SPROP` token's value, rendered and leak-checked exactly as
/// `class_feature_descriptions.rs` treats a `DESC:` value, joined with a
/// space when a record states more than one (the Base row states two: its
/// ability-score summary and its Empathy note). A leaking candidate is
/// refused individually -- the record's other, well-formed `SPROP` values
/// still ship, matching this module's skip-the-row-not-the-book posture.
fn safe_description(tokens: &[Value], key: &str, book: &str) -> Option<String> {
    let mut parts = Vec::new();
    for token in tokens {
        if token["key"].as_str() != Some("SPROP") {
            continue;
        }
        let Some(raw) = token["value"].as_str() else { continue };
        if !is_real_prose(raw) {
            continue;
        }
        let rendered = codex::rules_core::pcgen_desc::render_pcgen_desc(raw);
        if let Some(leak) = codex::rules_core::pcgen_desc::leaked_pcgen_syntax(&rendered.text) {
            eprintln!(
                "intelligent_item_catalog: refusing one SPROP value on {key:?} ({book}) -- \
                 rendered text still carries {leak}. Raw token: {raw:?}"
            );
            continue;
        }
        if is_real_prose(&rendered.text) {
            parts.push(rendered.text);
        }
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" "))
    }
}

fn has_visible_no(tokens: &[Value]) -> bool {
    tokens
        .iter()
        .any(|t| t["key"].as_str() == Some("VISIBLE") && t["value"].as_str() == Some("NO"))
}

fn build_mechanics(chains: &[Value]) -> Vec<IntelligentItemMechanicDto> {
    let mut out = Vec::new();
    for chain in chains {
        let Some(qualifiers) = chain["qualifiers"].as_array() else { continue };
        let strs: Vec<&str> = qualifiers.iter().filter_map(Value::as_str).collect();
        if strs.len() < 3 || strs[0] != "VAR" {
            continue;
        }
        let variable = strs[1].to_string();
        let raw_formula = strs[2];
        let formula = if variable == "IntelligentItemEgo" && raw_formula.contains("BaseCostTracker") {
            format_base_ego_price_bands(raw_formula)
        } else if let Ok(n) = raw_formula.parse::<i64>() {
            format!("{n:+}")
        } else {
            simplify_formula(raw_formula)
        };
        let mut condition = None;
        let mut bonus_type = None;
        for extra in &strs[3..] {
            if let Some(rest) = extra.strip_prefix("TYPE=") {
                bonus_type = Some(rest.to_string());
            } else {
                condition = Some(translate_condition(extra));
            }
        }
        out.push(IntelligentItemMechanicDto {
            effect: friendly_var_label(&variable),
            variable,
            formula,
            condition,
            bonus_type,
        });
    }
    out
}

fn ego_delta_from(mechanics: &[IntelligentItemMechanicDto]) -> Option<i32> {
    let ego: Vec<&IntelligentItemMechanicDto> =
        mechanics.iter().filter(|m| m.variable == "IntelligentItemEgo").collect();
    if ego.len() != 1 {
        return None;
    }
    // Only a bare signed-integer formula (never the Base row's price-band
    // sentence) counts as a convenience delta -- see the module doc's "No
    // fabricated Ego score" section.
    ego[0].formula.parse::<i32>().ok()
}

fn walk_json_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            walk_json_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "json") {
            out.push(path);
        }
    }
}

/// Reads every `equipmods` record under `<repo_root>/data/corpus/*/equipment/
/// equipmods/*.json` whose `KEY:` token contains `"Intelligent Item"`,
/// dropping the 17 `VISIBLE: NO` hidden trigger rows (see the module doc).
fn load_intelligent_item_components(repo_root: &Path) -> Vec<IntelligentItemComponentDto> {
    let corpus_root = repo_root.join("data/corpus");
    let mut out = Vec::new();
    let Ok(books) = std::fs::read_dir(&corpus_root) else { return out };
    let mut book_dirs: Vec<_> = books.flatten().collect();
    book_dirs.sort_by_key(|e| e.file_name());
    for book_entry in book_dirs {
        let book_dir = book_entry.path();
        if !book_dir.is_dir() {
            continue;
        }
        let book = book_entry.file_name().to_string_lossy().to_string();
        let mods_dir = book_dir.join("equipment").join("equipmods");
        if !mods_dir.is_dir() {
            continue;
        }
        let mut files = Vec::new();
        walk_json_files(&mods_dir, &mut files);
        for file in files {
            let Ok(text) = std::fs::read_to_string(&file) else { continue };
            let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
            let data = &doc["data"];
            let (Some(key), Some(name)) = (data["key"].as_str(), data["name"].as_str()) else {
                continue;
            };
            if !key.contains("Intelligent Item") {
                continue;
            }
            let tokens = data["raw_tokens"].as_array().cloned().unwrap_or_default();
            if has_visible_no(&tokens) {
                continue;
            }
            let chains = data["raw_bonus_chains"].as_array().cloned().unwrap_or_default();
            let mechanics = build_mechanics(&chains);
            let ego_delta = ego_delta_from(&mechanics);
            out.push(IntelligentItemComponentDto {
                book: book.clone(),
                family: family_for_key(key),
                key: key.to_string(),
                name: name.to_string(),
                cost_gp: data["cost_gp"].as_f64(),
                description: safe_description(&tokens, key, &book),
                mechanics,
                ego_delta,
            });
        }
    }
    out
}

/// Built once, cached for the process lifetime -- mirrors
/// `class_feature_descriptions.rs`'s own caching shape.
fn intelligent_item_components() -> &'static Vec<IntelligentItemComponentDto> {
    static TABLE: OnceLock<Vec<IntelligentItemComponentDto>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let repo_root = codex_repo_root()
            .expect("codex repo root must resolve for intelligent item catalog loading");
        load_intelligent_item_components(&repo_root)
    })
}

#[tauri::command]
pub fn list_intelligent_item_catalog() -> Vec<IntelligentItemComponentDto> {
    intelligent_item_components().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_root() -> PathBuf {
        codex_repo_root().expect("repo root resolves under `cargo test`")
    }

    #[test]
    fn alignment_name_covers_all_nine_pf1_codes() {
        assert_eq!(alignment_name("LG"), "Lawful Good");
        assert_eq!(alignment_name("NG"), "Neutral Good");
        assert_eq!(alignment_name("CG"), "Chaotic Good");
        assert_eq!(alignment_name("LN"), "Lawful Neutral");
        assert_eq!(alignment_name("TN"), "True Neutral");
        assert_eq!(alignment_name("NE"), "Neutral Evil");
        assert_eq!(alignment_name("LE"), "Lawful Evil");
        assert_eq!(alignment_name("CE"), "Chaotic Evil");
        assert_eq!(alignment_name("CN"), "Chaotic Neutral");
    }

    #[test]
    fn simplify_formula_unwraps_var_reference_and_leaves_everything_else_untouched() {
        assert_eq!(simplify_formula("1+var(\"IntItemNegativeLevel\")"), "1+IntItemNegativeLevel");
        assert_eq!(simplify_formula("COST"), "COST");
        assert_eq!(simplify_formula("4"), "4");
    }

    #[test]
    fn translate_condition_covers_every_shape_this_population_uses() {
        assert_eq!(
            translate_condition("!PREALIGN:LG"),
            "wielder's alignment is not Lawful Good"
        );
        assert_eq!(translate_condition("PREALIGN:LG,NG"), "wielder's alignment is Lawful Good or Neutral Good");
        assert_eq!(translate_condition("PREVARGTEQ:IntelligentItemEgo,20"), "IntelligentItemEgo >= 20");
        assert_eq!(translate_condition("PREVARLTEQ:IntelligentItemEgo,19"), "IntelligentItemEgo <= 19");
        assert_eq!(translate_condition("PREWEIRDNEWTOKEN:X"), "(untranslated condition token) PREWEIRDNEWTOKEN:X");
    }

    /// Transcribed by hand from the real corpus formula string (read via
    /// `cat data/corpus/core_rulebook/equipment/equipmods/
    /// intelligent_item_base.json`), independent of
    /// [`format_base_ego_price_bands`]'s own implementation -- this is the
    /// expected VALUE, not a re-derivation using the function under test.
    #[test]
    fn format_base_ego_price_bands_matches_the_hand_transcribed_corpus_formula() {
        let formula = "(BaseCostTracker>=1001)+(BaseCostTracker>=5001)+(BaseCostTracker>=10001)+\
                        (BaseCostTracker>=20001)+(BaseCostTracker>=50001)+(BaseCostTracker>=50001)+\
                        (BaseCostTracker>=100001)+(BaseCostTracker>=100001)+(BaseCostTracker>=200001)+\
                        (BaseCostTracker>=200001)+(BaseCostTracker>=200001)+(BaseCostTracker>=200001)";
        let rendered = format_base_ego_price_bands(formula);
        assert_eq!(
            rendered,
            "Base Ego from item price (cumulative): price \u{2265} 1001 gp: +1 Ego; \
             price \u{2265} 5001 gp: +1 Ego; price \u{2265} 10001 gp: +1 Ego; \
             price \u{2265} 20001 gp: +1 Ego; price \u{2265} 50001 gp: +2 Ego; \
             price \u{2265} 100001 gp: +2 Ego; price \u{2265} 200001 gp: +4 Ego"
        );
    }

    /// A second, independent readback of the SAME real file on disk --
    /// counts `">=200001"` occurrences with a dumb substring count rather
    /// than calling [`format_base_ego_price_bands`], then checks that count
    /// against the live loader's own served output. Proves the parser
    /// against the file, not against itself (the anti-circularity rule this
    /// package's dispatch names).
    #[test]
    fn format_base_ego_price_bands_matches_a_dumb_independent_count_of_the_raw_formula_bytes() {
        let path = repo_root()
            .join("data/corpus/core_rulebook/equipment/equipmods/intelligent_item_base.json");
        let raw = std::fs::read_to_string(&path).expect("intelligent_item_base.json must exist");
        assert_eq!(raw.matches(">=200001").count(), 4, "the live file's own coefficient for the top band");
        assert_eq!(raw.matches(">=100001").count(), 2);
        assert_eq!(raw.matches(">=50001").count(), 2);
        assert_eq!(raw.matches(">=1001").count(), 1);

        let components = load_intelligent_item_components(&repo_root());
        let base = components
            .iter()
            .find(|c| c.key == "Intelligent Item ~ Base")
            .expect("the CRB Base row must be served");
        let ego_mechanic = base
            .mechanics
            .iter()
            .find(|m| m.variable == "IntelligentItemEgo")
            .expect("the Base row states an IntelligentItemEgo formula");
        assert!(ego_mechanic.formula.contains("200001 gp: +4"));
        assert!(ego_mechanic.formula.contains("100001 gp: +2"));
        assert!(ego_mechanic.formula.contains("50001 gp: +2"));
        assert!(ego_mechanic.formula.contains("1001 gp: +1"));
    }

    #[test]
    fn family_for_key_partitions_every_visible_record_into_a_non_empty_family() {
        let components = load_intelligent_item_components(&repo_root());
        assert!(components.len() > 100, "expected the ~152 visible components, got {}", components.len());
        for component in &components {
            assert!(!component.family.trim().is_empty(), "{:?} has no family", component.key);
        }
        let crb_base = components
            .iter()
            .find(|c| c.key == "Intelligent Item ~ Base")
            .expect("CRB Base row present");
        assert_eq!(crb_base.family, "Base");
        let mythic_base = components
            .iter()
            .find(|c| c.key == "Legendary Item ~ Intelligent Item")
            .expect("Mythic base row present");
        assert_eq!(mythic_base.family, "Base");
        let int_14 = components
            .iter()
            .find(|c| c.key == "Intelligent Item ~ Ability Score / Intelligence 14")
            .expect("a real ability score row present");
        assert_eq!(int_14.family, "Ability Score");
        let align = components
            .iter()
            .find(|c| c.key == "Intelligent Item ~ Alignment / Lawful Good")
            .expect("a real alignment row present");
        assert_eq!(align.family, "Alignment");
    }

    /// Transcribed by hand from `intelligent_item_ability_score_
    /// intelligence_14.json`'s own `raw_bonus_chains`: `VAR
    /// IntelligentItemEgo 2` and `VAR IntItemStatINT 4`.
    #[test]
    fn a_real_ability_score_row_carries_its_literal_ego_and_stat_deltas() {
        let components = load_intelligent_item_components(&repo_root());
        let int_14 = components
            .iter()
            .find(|c| c.key == "Intelligent Item ~ Ability Score / Intelligence 14")
            .expect("present");
        assert_eq!(int_14.ego_delta, Some(2));
        let stat = int_14
            .mechanics
            .iter()
            .find(|m| m.variable == "IntItemStatINT")
            .expect("states an Intelligence delta");
        assert_eq!(stat.formula, "+4");
        assert_eq!(stat.effect, "Intelligence");
    }

    /// Pins the module doc's central honesty claim: the Base row's Ego
    /// mechanic is real and present, but it never collapses to a single
    /// `ego_delta` integer, because summing its price bands into one number
    /// would require a specific item's price -- context this corpus does
    /// not fix.
    #[test]
    fn no_component_ever_emits_a_fabricated_resolved_total_ego_score() {
        let components = load_intelligent_item_components(&repo_root());
        let base = components.iter().find(|c| c.key == "Intelligent Item ~ Base").expect("present");
        assert_eq!(base.ego_delta, None, "the Base row's Ego is a price-dependent formula, never a number");
        assert!(base.mechanics.iter().any(|m| m.variable == "IntelligentItemEgo"));
        // No served component anywhere states an unconditional flat
        // "resolved total Ego" — every Ego-bearing mechanic is either a
        // per-component contribution (a small literal delta, honestly
        // partial) or the Base row's own explicit price-band formula.
        for component in &components {
            for mechanic in &component.mechanics {
                if mechanic.variable == "IntelligentItemEgo" {
                    assert!(
                        mechanic.formula.starts_with('+')
                            || mechanic.formula.starts_with('-')
                            || mechanic.formula.starts_with("Base Ego from item price"),
                        "{:?} states an Ego formula shape this test does not recognize: {}",
                        component.key,
                        mechanic.formula
                    );
                }
            }
        }
    }

    /// Mutation check: if `ego_delta_from` were changed to also resolve the
    /// Base row's price-band formula down to a placeholder integer (e.g.
    /// treating an unparsed formula as `0`), this test fails --
    /// demonstrating the guard above is load-bearing, not a tautology.
    #[test]
    fn mutation_removing_the_ego_delta_none_guard_would_be_caught_by_the_pin() {
        let formula_shaped = IntelligentItemMechanicDto {
            variable: "IntelligentItemEgo".to_string(),
            effect: "Ego".to_string(),
            formula: format_base_ego_price_bands(
                "(BaseCostTracker>=1001)+(BaseCostTracker>=5001)",
            ),
            condition: None,
            bonus_type: None,
        };
        assert_eq!(
            ego_delta_from(std::slice::from_ref(&formula_shaped)),
            None,
            "a price-band formula must never parse as a bare integer delta"
        );
        // `build_mechanics` always formats a literal integer with an
        // explicit sign (`format!("{n:+}")`), and Rust's own integer
        // `FromStr` accepts that leading `+` -- so this shape (exactly what
        // a real ability-score/purpose row's `formula` field holds) must
        // resolve to a real delta, matching
        // `a_real_ability_score_row_carries_its_literal_ego_and_stat_deltas`'s
        // own expectation for `Intelligent Item ~ Ability Score /
        // Intelligence 14`. A mutation that made `ego_delta_from` refuse
        // this real shape (over-tightening) would be caught here.
        let literal_shaped = IntelligentItemMechanicDto {
            variable: "IntelligentItemEgo".to_string(),
            effect: "Ego".to_string(),
            formula: "+2".to_string(),
            condition: None,
            bonus_type: None,
        };
        assert_eq!(ego_delta_from(std::slice::from_ref(&literal_shaped)), Some(2));
        // A mutation that made `ego_delta_from` treat two Ego mechanics on
        // one component as summable (rather than refusing, since this
        // population never states two) would be caught here.
        let two_ego_mechanics = [literal_shaped.clone(), literal_shaped];
        assert_eq!(
            ego_delta_from(&two_ego_mechanics),
            None,
            "two Ego mechanics on one component must refuse to guess a combined delta"
        );
    }

    /// The 17 hidden `VISIBLE: NO` trigger rows this module's doc comment
    /// names, checked by exact key, never appear in the served catalog.
    #[test]
    fn hidden_trigger_rows_never_reach_the_served_catalog() {
        let components = load_intelligent_item_components(&repo_root());
        let served_keys: std::collections::BTreeSet<&str> =
            components.iter().map(|c| c.key.as_str()).collect();
        let hidden = [
            "Intelligent Item Alignment (CE)",
            "Intelligent Item Alignment (CG)",
            "Intelligent Item Alignment (CN)",
            "Intelligent Item Alignment (LE)",
            "Intelligent Item Alignment (LG)",
            "Intelligent Item Alignment (LN)",
            "Intelligent Item Alignment (NE)",
            "Intelligent Item Alignment (NG)",
            "Intelligent Item Alignment (TN)",
            "Intelligent Item Purpose (Defend Deity Servant)",
            "Intelligent Item Purpose (Defend Race or Kind)",
            "Intelligent Item Purpose (Slay Align)",
            "Intelligent Item Purpose (Slay Arcane)",
            "Intelligent Item Purpose (Slay Deity Servant)",
            "Intelligent Item Purpose (Slay Divine)",
            "Intelligent Item Purpose (Slay NonCasters)",
            "Intelligent Item Purpose (Slay Race or Kind)",
        ];
        assert_eq!(hidden.len(), 17);
        for key in hidden {
            assert!(!served_keys.contains(key), "{key:?} is a hidden trigger row and must not be served");
        }
        // The hidden row's spelled-out, purchasable sibling IS served.
        assert!(served_keys.contains("Intelligent Item ~ Alignment / Lawful Good"));
        assert!(served_keys.contains("Intelligent Item ~ Purpose / Slay Arcane Spellcaster"));
    }

    #[test]
    fn every_served_record_carries_no_declared_pi_marker() {
        let repo_root = repo_root();
        let corpus_root = repo_root.join("data/corpus");
        let mut checked = 0;
        for book in ["core_rulebook", "mythic_adventures"] {
            let dir = corpus_root.join(book).join("equipment").join("equipmods");
            let mut files = Vec::new();
            walk_json_files(&dir, &mut files);
            for file in files {
                let text = std::fs::read_to_string(&file).expect("readable");
                let doc: Value = serde_json::from_str(&text).expect("valid json");
                if !doc["data"]["key"].as_str().unwrap_or_default().contains("Intelligent Item") {
                    continue;
                }
                assert!(doc["pi_field"].is_null(), "{file:?} carries a declared pi_field");
                assert!(doc["pi_marker"].is_null(), "{file:?} carries a declared pi_marker");
                checked += 1;
            }
        }
        // Row-19 desktop reach/catalog reds (SD-32, 2026-08-24): +2 from
        // the T12 census/class-feature lanes' corpus growth. Re-derived by
        // running this exact walk-and-assert loop, which panics per file if
        // either PI field is declared, so reaching `checked == 171` without
        // a panic is itself the proof all 171 are already clean -- not a
        // loosened check.
        assert_eq!(checked, 171, "expected all 171 intelligent/legendary item records to be checked");
    }

    /// Live PI-blacklist sweep over every emitted name/description, the
    /// same live term list `reach_gate.rs` checks served Inner Sea World
    /// Guide content against — a term added to the blacklist later fails
    /// here automatically, rather than this module needing its own copy.
    #[test]
    fn every_served_name_and_description_clears_the_pi_blacklist() {
        let components = load_intelligent_item_components(&repo_root());
        assert!(components.len() > 100);
        for component in &components {
            let mut fields = vec![component.name.to_ascii_lowercase(), component.key.to_ascii_lowercase()];
            if let Some(desc) = &component.description {
                fields.push(desc.to_ascii_lowercase());
            }
            for field in &fields {
                for term in codex::rules_core::pi_screening::PI_BLACKLIST_TERMS {
                    assert!(
                        !field.contains(&term.to_ascii_lowercase()),
                        "{:?} matches PI blacklist term {term:?}",
                        component.key
                    );
                }
            }
        }
    }

    /// `OPEN-ISSUES.md` row 138's already-diagnosed leak, re-derived
    /// independently here rather than trusted from that fix's own file:
    /// the Base row's ability-score-summary `SPROP` states 4 bare `%`
    /// placeholders with a 4-argument tail `render_pcgen_desc` cannot
    /// resolve without character context, so it must be refused, and the
    /// Base row's OTHER real `SPROP` (about Empathy) must still ship.
    #[test]
    fn refuses_the_known_leaking_base_ability_score_sprop() {
        let components = load_intelligent_item_components(&repo_root());
        let base = components.iter().find(|c| c.key == "Intelligent Item ~ Base").expect("present");
        let description = base.description.as_deref().unwrap_or_default();
        assert!(
            !description.contains('|'),
            "a leaked pipe-argument tail must never reach the served description: {description:?}"
        );
        assert!(
            description.to_ascii_lowercase().contains("empathy"),
            "the Base row's other real SPROP (Empathy) must still ship: {description:?}"
        );
    }

    #[test]
    fn every_served_description_renders_without_a_pcgen_syntax_leak() {
        let components = load_intelligent_item_components(&repo_root());
        let mut checked = 0;
        for component in &components {
            if let Some(description) = &component.description {
                if let Some(leak) = codex::rules_core::pcgen_desc::leaked_pcgen_syntax(description) {
                    panic!("{:?} ({}): leaked {leak}", component.key, component.book);
                }
                checked += 1;
            }
        }
        assert!(checked > 5, "no real descriptions were checked; the check proved nothing");
    }

    #[test]
    fn loads_both_the_crb_and_mythic_intelligent_item_families() {
        let components = load_intelligent_item_components(&repo_root());
        assert!(components.iter().any(|c| c.book == "core_rulebook"));
        assert!(components.iter().any(|c| c.book == "mythic_adventures"));
    }

    #[test]
    fn list_intelligent_item_catalog_returns_the_cached_table() {
        let a = list_intelligent_item_catalog();
        let b = list_intelligent_item_catalog();
        assert_eq!(a.len(), b.len());
        assert!(!a.is_empty());
    }
}
