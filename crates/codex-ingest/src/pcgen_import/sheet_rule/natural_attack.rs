//! SD-36 Epic F3c5: a PCGen `CATEGORY:Internal` natural-attack helper row, converted as a FACT on
//! the rule that grants it.
//!
//! # The oracle
//!
//! - `system/gameModes/Pathfinder/miscinfo.lst:303` (pinned oracle checkout):
//!   `ABILITYCATEGORY:Internal VISIBLE:NO EDITABLE:NO EDITPOOL:NO`. An `Internal` ability is a
//!   hidden, non-selectable helper: PCGen never shows it and the player never picks it.
//! - `Bite` (`core_essentials/ce_abilities_race.lst:249`), `CATEGORY:Internal`,
//!   `TYPE:NaturalAttack.NaturalAttackPrimary.Primary`: it `DEFINE`s and raises `BiteAttacks` and
//!   `NaturalAttacks`, and adjusts the Bite weapon by `BONUS:WEAPONPROF=Bite|TOHIT|DAMAGE`. Its
//!   `.MOD` rows (`:307`, `:327`) set `BiteSize` from `NaturalAttackSize` and grant the size
//!   helper `Bite 1 (<size>)` (`:354`, `PREBASESIZEEQ:M TEMPLATE:Bite 1 (Medium)`), whose
//!   template (`ce_templates.lst:26`) carries `NATURALATTACKS:Bite,Weapon.Natural...,*1,1d6`.
//! - `docs/listfilepages/globalfilestagpages/globalfilesother.html` `NATURALATTACKS`: the tag
//!   creates the natural weapon ("(Natural weapon name)", its types, count and damage) and "creates
//!   a proficiency for the attacks on the fly which allows the use of the `BONUS:WEAPONPROF` tag to
//!   effect it" -- the `BONUS:WEAPONPROF=<attack>` rows on the helper adjust that one weapon.
//!
//! So holding `Internal|Bite` gives the character exactly one thing the sheet can name: the Bite
//! natural attack.
//!
//! # The rule (one mechanism, no per-attack case)
//!
//! A **natural-attack helper** is an ability-family row outside `_pfs/`, plain (not `.MOD` /
//! `.COPY=`), whose `CATEGORY:` is `Internal` and whose `TYPE:` carries `NaturalAttack`, that NO
//! inventory unit stands for (no unit owns the row; no unit answers its `(Internal, KEY)` /
//! `(Internal, name)` pair). Its **object** is the row plus every `.MOD` row targeting its key.
//!
//! The object is **natural-attack-only** when every token on every row of it -- followed through
//! each `ABILITY:Internal|...|<t>` grant (itself a natural-attack helper, recursively) and each
//! `TEMPLATE:<t>` (a template row outside `_pfs/`) -- is one of:
//! - identity / display: `CATEGORY`, `KEY`, `TYPE`, `VISIBLE`, `OUTPUTNAME`, `SOURCE*`;
//! - bookkeeping: `DEFINE`, `BONUS:VAR`, a `PRE*` / `!PRE*` gate;
//! - the attack itself: `NATURALATTACKS`, and `BONUS:WEAPONPROF=<w>` where `<w>` is an attack the
//!   object's `NATURALATTACKS` name or the helper's own name.
//!
//! Its attacks are the names its `NATURALATTACKS` entries state (the first comma field). An
//! object that reaches no `NATURALATTACKS` names no attack and is not converted.
//!
//! Such a helper converts as `Fact::NaturalAttack(<attack>)`, one per attack, on the rule whose
//! `ABILITY:Internal|<nature>|<helper>` names it (`GatedFactGrant` when that row is gated), so
//! the reference resolves. No rule and no inventory unit is added: the 49,450-unit inventory does
//! not move.
//!
//! Named, never guessed (`_defects/<kind>.json`):
//! - `natural-attack-helper-carries-more`: a helper whose object carries a token outside the list
//!   (a real ability), with the tokens;
//! - `natural-attack-helper-names-no-attack`: a helper whose object reaches no `NATURALATTACKS`;
//! - `natural-attack-helper-pair-shared`: two helpers claiming one `(Internal, key-or-name)` pair
//!   with different attacks (neither is used).
//!
//! A reference to any of these stays in `unresolved-references`.

use std::collections::{BTreeMap, BTreeSet};

use super::closure::{row_identity, tokenize_row, FileFamily, PinnedTree, RowRef, RowShape};
use super::ctx::split_gates;
use super::prose::pi_hit;

const INTERNAL: &str = "INTERNAL";

/// One natural-attack helper, classified.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NaturalAttackHelper {
    /// The helper's own row.
    pub row: RowRef,
    /// `(INTERNAL, KEY)` upper.
    pub key: String,
    pub name: String,
    /// The attacks its object's `NATURALATTACKS` name (sorted, unique).
    pub attacks: Vec<String>,
}

/// Everything the scan found.
#[derive(Debug, Clone, Default)]
pub struct NaturalAttackScan {
    /// `(INTERNAL, KEY-or-name upper)` -> the helper that answers it.
    pub helpers: BTreeMap<(String, String), NaturalAttackHelper>,
    /// Every natural-attack helper row the tree declares that no unit stands for, classified or
    /// not (the denominator).
    pub declared: usize,
    pub defects: BTreeMap<String, Vec<String>>,
}

/// The verdict on one token (module doc).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenRead {
    /// Allowed, nothing to follow.
    Ok,
    /// `NATURALATTACKS`: the attack names.
    Attacks(Vec<String>),
    /// `BONUS:WEAPONPROF=<w>`: checked against the attacks once they are known.
    WeaponBonus(String),
    /// `ABILITY:Internal|...|<t>` targets (upper).
    Helpers(Vec<String>),
    /// `TEMPLATE:<t>` targets (upper).
    Templates(Vec<String>),
    /// A token outside the list: `HEAD` (and the sub-token where there is one).
    Other(String),
}

/// Read one `(head, value)` token of a helper or template row (module doc list).
pub fn read_token(head: &str, value: &str, template_row: bool) -> TokenRead {
    let h = head.trim();
    let upper = h.to_ascii_uppercase();
    let bare = upper.trim_start_matches('!');
    if bare.starts_with("PRE") {
        return TokenRead::Ok;
    }
    match upper.as_str() {
        "CATEGORY" | "KEY" | "TYPE" | "VISIBLE" | "OUTPUTNAME" | "DEFINE" => {
            if template_row && matches!(upper.as_str(), "CATEGORY" | "DEFINE") {
                // A template carrying a category or a variable declaration is not the plain
                // natural-attack template the helpers apply.
                return TokenRead::Other(upper);
            }
            TokenRead::Ok
        }
        s if s.starts_with("SOURCE") => TokenRead::Ok,
        "NATURALATTACKS" => {
            let names: Vec<String> = value.split('|').filter_map(|e| e.split(',').next()).map(|n| n.trim().to_string()).filter(|n| !n.is_empty()).collect();
            if names.is_empty() {
                return TokenRead::Other("NATURALATTACKS (no attack name)".into());
            }
            TokenRead::Attacks(names)
        }
        "BONUS" => {
            let (fields, _) = split_gates(value);
            let sub = fields.first().map(|s| s.trim().to_string()).unwrap_or_default();
            let sub_u = sub.to_ascii_uppercase();
            if sub_u == "VAR" {
                return TokenRead::Ok;
            }
            if let Some(w) = sub_u.strip_prefix("WEAPONPROF=")
                && !template_row
            {
                let _ = w;
                return TokenRead::WeaponBonus(sub["WEAPONPROF=".len()..].trim().to_string());
            }
            TokenRead::Other(format!("BONUS:{}", sub_u.split('=').next().unwrap_or("")))
        }
        "ABILITY" if !template_row => {
            let (fields, _) = split_gates(value);
            if fields.len() < 3 || !fields[0].trim().eq_ignore_ascii_case("Internal") {
                return TokenRead::Other(format!("ABILITY:{}", fields.first().map(|s| s.trim()).unwrap_or("")));
            }
            let targets: Vec<String> = fields.iter().skip(2).map(|t| t.trim().to_ascii_uppercase()).filter(|t| !t.is_empty()).collect();
            if targets.iter().any(|t| t.contains("%LIST") || t.starts_with("TYPE=") || t.starts_with("TYPE.")) {
                return TokenRead::Other("ABILITY:Internal (a choice or type selector)".into());
            }
            TokenRead::Helpers(targets)
        }
        "TEMPLATE" if !template_row => {
            let targets: Vec<String> = value.split('|').map(|t| t.trim().to_ascii_uppercase()).filter(|t| !t.is_empty()).collect();
            if targets.iter().any(|t| t.contains("%LIST") || t.starts_with("CHOOSE:") || t.starts_with("ADDCHOICE:")) {
                return TokenRead::Other("TEMPLATE (a choice)".into());
            }
            TokenRead::Templates(targets)
        }
        _ => TokenRead::Other(upper),
    }
}

/// Whether a row's own `TYPE:` carries `NaturalAttack`.
fn is_natural_attack_typed(tokens: &[(String, String)]) -> bool {
    tokens.iter().any(|(k, v)| k.eq_ignore_ascii_case("TYPE") && v.split('.').any(|t| t.trim().eq_ignore_ascii_case("NaturalAttack")))
}

struct Walk<'a> {
    tree: &'a PinnedTree,
    /// `(INTERNAL, KEY)` -> the plain natural-attack-typed row (first in tree order).
    helper_rows: &'a BTreeMap<String, RowRef>,
    /// Template KEY-or-name upper -> its plain row (first in tree order, outside `_pfs/`).
    templates: &'a BTreeMap<String, RowRef>,
}

#[derive(Default)]
struct Reach {
    attacks: BTreeSet<String>,
    weapon_bonuses: BTreeSet<String>,
    other: BTreeSet<String>,
}

impl Walk<'_> {
    /// Walk one helper's object (module doc), accumulating into `reach`.
    fn helper(&self, key: &str, seen: &mut BTreeSet<String>, reach: &mut Reach) {
        if !seen.insert(key.to_string()) {
            return;
        }
        let Some(&row) = self.helper_rows.get(key) else {
            reach.other.insert(format!("ABILITY:Internal|{key} (not a natural-attack helper row)"));
            return;
        };
        let mut rows = vec![row];
        rows.extend_from_slice(self.tree.mods_for(FileFamily::Ability, INTERNAL, key));
        for r in rows {
            let (_, tokens) = tokenize_row(self.tree.row_text(r));
            for (k, v) in &tokens {
                match read_token(k, v, false) {
                    TokenRead::Ok => {}
                    TokenRead::Attacks(a) => reach.attacks.extend(a),
                    TokenRead::WeaponBonus(w) => {
                        reach.weapon_bonuses.insert(w.to_ascii_uppercase());
                    }
                    TokenRead::Helpers(ts) => {
                        for t in ts {
                            self.helper(&t, seen, reach);
                        }
                    }
                    TokenRead::Templates(ts) => {
                        for t in ts {
                            self.template(&t, reach);
                        }
                    }
                    TokenRead::Other(o) => {
                        reach.other.insert(format!("{o} ({})", self.tree.cite(r)));
                    }
                }
            }
        }
    }

    fn template(&self, key: &str, reach: &mut Reach) {
        let Some(&row) = self.templates.get(key) else {
            reach.other.insert(format!("TEMPLATE:{key} (no template row)"));
            return;
        };
        let (_, tokens) = tokenize_row(self.tree.row_text(row));
        for (k, v) in &tokens {
            match read_token(k, v, true) {
                TokenRead::Ok => {}
                TokenRead::Attacks(a) => reach.attacks.extend(a),
                TokenRead::Other(o) => {
                    reach.other.insert(format!("{o} ({})", self.tree.cite(row)));
                }
                // Not produced for a template row.
                TokenRead::WeaponBonus(_) | TokenRead::Helpers(_) | TokenRead::Templates(_) => {
                    reach.other.insert(format!("{k} ({})", self.tree.cite(row)));
                }
            }
        }
    }
}

/// Find and classify every natural-attack helper (module doc). `answered(cat, key_or_name)` says
/// whether an inventory unit (or an earlier-registered option) already answers the pair;
/// `owned(row)` whether a unit owns the row.
pub fn scan(tree: &PinnedTree, owned: &dyn Fn(RowRef) -> bool, answered: &dyn Fn(&str, &str) -> bool) -> NaturalAttackScan {
    let mut s = NaturalAttackScan::default();
    // Every plain natural-attack-typed Internal row (the walk follows grants into them even when
    // a unit stands for the granted one), and every template row, outside `_pfs/`.
    let mut helper_rows: BTreeMap<String, RowRef> = BTreeMap::new();
    let mut candidates: Vec<(RowRef, String, String)> = Vec::new();
    let mut templates: BTreeMap<String, RowRef> = BTreeMap::new();
    for (fi, file) in tree.files.iter().enumerate() {
        if file.is_pfs || !matches!(file.family, FileFamily::Ability | FileFamily::Template) {
            continue;
        }
        for (li, raw) in file.lines.iter().enumerate() {
            let row = RowRef { file: fi, line: li + 1 };
            let ident = row_identity(raw);
            if ident.shape != RowShape::Plain {
                continue;
            }
            if file.family == FileFamily::Template {
                templates.entry(ident.key.clone()).or_insert(row);
                continue;
            }
            if ident.category != INTERNAL {
                continue;
            }
            let (head, tokens) = tokenize_row(raw);
            if !is_natural_attack_typed(&tokens) {
                continue;
            }
            helper_rows.entry(ident.key.clone()).or_insert(row);
            let name = head.trim().to_string();
            if owned(row) || answered(INTERNAL, &ident.key) || answered(INTERNAL, &name.to_ascii_uppercase()) {
                continue;
            }
            candidates.push((row, ident.key.clone(), name));
        }
    }
    s.declared = candidates.len();
    let walk = Walk { tree, helper_rows: &helper_rows, templates: &templates };
    // Pair -> (helper, whether another helper claimed it with different attacks).
    let mut claims: BTreeMap<(String, String), (NaturalAttackHelper, bool)> = BTreeMap::new();
    for (row, key, name) in candidates {
        let cite = tree.cite(row);
        let mut reach = Reach::default();
        // The walk starts at this row, not at whichever row first declared the key.
        let mut seen = BTreeSet::new();
        seen.insert(key.clone());
        let mut rows = vec![row];
        rows.extend_from_slice(tree.mods_for(FileFamily::Ability, INTERNAL, &key));
        for r in rows {
            let (_, tokens) = tokenize_row(tree.row_text(r));
            for (k, v) in &tokens {
                match read_token(k, v, false) {
                    TokenRead::Ok => {}
                    TokenRead::Attacks(a) => reach.attacks.extend(a),
                    TokenRead::WeaponBonus(w) => {
                        reach.weapon_bonuses.insert(w.to_ascii_uppercase());
                    }
                    TokenRead::Helpers(ts) => {
                        for t in ts {
                            walk.helper(&t, &mut seen, &mut reach);
                        }
                    }
                    TokenRead::Templates(ts) => {
                        for t in ts {
                            walk.template(&t, &mut reach);
                        }
                    }
                    TokenRead::Other(o) => {
                        reach.other.insert(format!("{o} ({})", tree.cite(r)));
                    }
                }
            }
        }
        let attack_upper: BTreeSet<String> = reach.attacks.iter().map(|a| a.to_ascii_uppercase()).collect();
        for w in &reach.weapon_bonuses {
            if !attack_upper.contains(w) && *w != name.to_ascii_uppercase() {
                reach.other.insert(format!("BONUS:WEAPONPROF={w} (not this helper's attack)"));
            }
        }
        if !reach.other.is_empty() {
            let o: Vec<String> = reach.other.into_iter().collect();
            s.defects.entry("natural-attack-helper-carries-more".into()).or_default().push(format!("Internal|{name} ({cite}): {}", o.join("; ")));
            continue;
        }
        if reach.attacks.is_empty() {
            s.defects.entry("natural-attack-helper-names-no-attack".into()).or_default().push(format!("Internal|{name} ({cite})"));
            continue;
        }
        if reach.attacks.iter().any(|a| pi_hit(a).is_some()) {
            s.defects.entry("natural-attack-helper-carries-more".into()).or_default().push(format!("Internal|{name} ({cite}): attack name withheld (product identity)"));
            continue;
        }
        let helper = NaturalAttackHelper { row, key: key.clone(), name: name.clone(), attacks: reach.attacks.into_iter().collect() };
        let mut pairs = vec![(INTERNAL.to_string(), key.clone())];
        if name.to_ascii_uppercase() != key {
            pairs.push((INTERNAL.to_string(), name.to_ascii_uppercase()));
        }
        for pair in pairs {
            match claims.get_mut(&pair) {
                None => {
                    claims.insert(pair, (helper.clone(), false));
                }
                Some((first, shared)) => {
                    if first.attacks != helper.attacks {
                        *shared = true;
                        s.defects.entry("natural-attack-helper-pair-shared".into()).or_default().push(format!(
                            "Internal|{}: {} {:?} and {cite} {:?}",
                            pair.1,
                            tree.cite(first.row),
                            first.attacks,
                            helper.attacks
                        ));
                    }
                }
            }
        }
    }
    for (pair, (helper, shared)) in claims {
        if !shared {
            s.helpers.insert(pair, helper);
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_bite_helper_tokens_read_as_one_natural_attack() {
        // `ce_abilities_race.lst:249`.
        assert_eq!(read_token("CATEGORY", "Internal", false), TokenRead::Ok);
        assert_eq!(read_token("BONUS", "VAR|BiteAttacks|1", false), TokenRead::Ok);
        assert_eq!(read_token("BONUS", "WEAPONPROF=Bite|TOHIT|-5|PREVAREQ:UseWeaponsWithNaturalAttacks,1", false), TokenRead::WeaponBonus("Bite".into()));
        assert_eq!(read_token("!PREVAREQ", "UseWeaponsWithNaturalAttacks,1", false), TokenRead::Ok);
        // `:327`, `:354`, `ce_templates.lst:26`.
        assert_eq!(read_token("ABILITY", "Internal|AUTOMATIC|Bite 1 (Fine)|PREVAREQ:BiteSize,1", false), TokenRead::Helpers(vec!["BITE 1 (FINE)".into()]));
        assert_eq!(read_token("TEMPLATE", "Bite 1 (Medium)", false), TokenRead::Templates(vec!["BITE 1 (MEDIUM)".into()]));
        assert_eq!(
            read_token("NATURALATTACKS", "Bite,Weapon.Natural.NaturalPrimary.Melee.Bludgeoning,*1,1d6", true),
            TokenRead::Attacks(vec!["Bite".into()])
        );
    }

    #[test]
    fn a_token_beyond_the_attack_is_named() {
        assert_eq!(read_token("SAB", "Something", false), TokenRead::Other("SAB".into()));
        assert_eq!(read_token("BONUS", "SKILL|Perception|2", false), TokenRead::Other("BONUS:SKILL".into()));
        assert_eq!(read_token("ABILITY", "Special Ability|AUTOMATIC|Rend", false), TokenRead::Other("ABILITY:Special Ability".into()));
        assert_eq!(read_token("ABILITY", "Internal|AUTOMATIC|%LIST", false), TokenRead::Other("ABILITY:Internal (a choice or type selector)".into()));
        // A template row may not grant abilities or adjust a weapon.
        assert_eq!(read_token("BONUS", "WEAPONPROF=Bite|TOHIT|1", true), TokenRead::Other("BONUS:WEAPONPROF".into()));
        assert_eq!(read_token("DR", "5/magic", true), TokenRead::Other("DR".into()));
    }
}
