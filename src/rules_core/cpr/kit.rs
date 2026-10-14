//! Cyberpunk RED starting kit and starting cash (Cyberpunk RED core rulebook, corpus pages
//! p099-p106; A-9).
//!
//! - **Streetrat and Edgerunner** (p099, p104): the Role's fixed kit, its `starting_kit` record:
//!   every plain line, one option of every "A or B" line ("pick one and only one"; the first
//!   printed option until the player picks another), every fashion line's pieces, and 500eb (the
//!   record's `cash_eb`) "to spend on other items or keep".
//! - **Complete Package** (p105, p106): no fixed kit; 2,550eb to buy weapons, armour, gear and
//!   the rest from the Night Market lists ("You keep anything you don't spend"), plus 800eb "to
//!   spend only on Fashion or Fashionware. Anything you don't spend goes away."
//!
//! Every method buys from the same shop: every priced record of the weapon, attachment,
//! ammunition, armour, gear, fashion, street drug, cyberdeck, cyberdeck hardware and program
//! catalogs, at its printed price (ammunition in tens, grenades and rockets one by one, p345).
//! Cyberware is bought in the cyberware step. A purchase the catalogs do not price, a quantity of
//! 0, a variant the record does not have and a spend above the budget are refused.
//!
//! The rows: `cpr.kit.budget`, `cpr.kit.spent` and `cpr.cash` (A-9: the budget minus the
//! purchases; after creation the saved cash is the sheet's write-in); for a Complete Package also
//! `cpr.kit.fashion_budget`, `cpr.kit.fashion_spent` and `cpr.kit.fashion_left`, which is 0 once
//! the character is saved (the unspent fashion budget is lost, never cash). Fashion is paid from
//! the fashion budget first and the rest from the cash.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::rules_core::cpr::armor::{CprArmorLocation, CprWornArmor};
use crate::rules_core::cpr::input::CprDeckChoice;
use crate::rules_core::cpr::roles::CprRole;
use crate::rules_core::pilot_compute::ComputationExplanation;

/// p105: "you get 2,550 Eurobucks (eb) to buy any weapons, armor, gear, and cyberware you want".
pub const COMPLETE_PACKAGE_CASH_EB: u32 = 2550;
/// p106: "an additional 800eb to spend only on Fashion or Fashionware. Anything you don't spend
/// goes away."
pub const COMPLETE_PACKAGE_FASHION_EB: u32 = 800;
/// p345: "Grenades and Rockets are purchased individually. All other ammunition is purchased in
/// increments of 10."
const AMMO_INCREMENT: u32 = 10;
/// The shotgun's other load (the kits' "Basic Shotgun Shell Ammunition", p344); every other
/// caliber is a weapon record's `ammo_type`.
const SHOTGUN_SHELL: &str = "Shotgun Shell";
/// The fixed kits print a Cyberdeck by its slots (`Cyberdeck (7 Slots)`).
const DECK_PREFIX: &str = "Cyberdeck (";
/// The fixed kits print a program as `Program: <name>`.
const PROGRAM_PREFIX: &str = "Program: ";

/// How the kit is made.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CprKitMethod {
    /// Streetrat and Edgerunner: the Role's fixed kit and 500eb.
    Fixed,
    /// Complete Package: 2,550eb and 800eb for fashion, everything bought.
    CompletePackage,
}

/// The player's kit choices: the option picked on an "or" line (by its index in
/// [`CprKitCatalog::fixed_lines`]) and the purchases.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprKitChoices {
    #[serde(default)]
    pub picks: Vec<CprKitPick>,
    #[serde(default)]
    pub purchases: Vec<CprKitPurchase>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprKitPick {
    pub line: usize,
    pub option: String,
}

/// One purchase: a shop kind (`weapon`, `ammo`, `armor`, `fashion`, ...), the record's name, the
/// variant where the record has them (armour's `Body`/`Head`, ammunition's caliber, a fashion
/// piece) and how many (ammunition: rounds, grenades or rockets).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprKitPurchase {
    pub kind: String,
    pub name: String,
    #[serde(default)]
    pub variant: Option<String>,
    pub quantity: u32,
}

/// One line of a fixed kit, as printed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprKitLine {
    /// `weapons_armor` or `outfit`.
    pub section: String,
    pub text: String,
    pub source_page: String,
    /// "A or B": exactly one option is taken.
    pub pick_one: bool,
    /// A fashion line (`Generic Chic: Jacket, Jewelry x3`): its options are the pieces.
    pub fashion: bool,
    pub options: Vec<CprKitOption>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprKitOption {
    pub name: String,
    pub quantity: u32,
}

/// One buyable record and its printed price.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CprShopItem {
    pub kind: String,
    pub name: String,
    /// The price of one (ammunition: of ten rounds, or of one grenade or rocket). A fashion
    /// style prices each piece in `variants` and has 0 here.
    pub cost_eb: u32,
    pub cost_text: String,
    pub source_page: String,
    /// What the purchase names as its variant (`Location`, `Caliber`, `Piece`), if anything.
    pub variant_label: Option<String>,
    /// The printed rule the variants come from (the ammunition's "Ammo Types Available").
    pub variant_note: String,
    pub variants: Vec<CprShopVariant>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CprShopVariant {
    pub name: String,
    /// A fashion piece's own price; `None` where the item's price holds.
    pub cost_eb: Option<u32>,
}

/// One item the character starts with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprKitItem {
    /// `weapons_armor` or `outfit` (fixed kit), or `bought`.
    pub section: String,
    /// The shop kind of a bought item; empty for a fixed-kit item.
    pub kind: String,
    /// As printed (fixed kit), or as the kits print the same item (`Light Armorjack Body Armor
    /// (SP11)`, `Basic Rifle Ammunition`, `Flashbang Grenade`, `Nomad Leathers Jacket`).
    pub name: String,
    pub quantity: u32,
    /// What it cost (0 for a fixed-kit item).
    pub cost_eb: u32,
    pub fashion: bool,
}

/// What the kit puts on the sheet.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CprKitSheet {
    pub weapons: Vec<String>,
    pub armor: Vec<CprWornArmor>,
    pub shield: Option<String>,
    pub ammunition: Vec<(String, u32)>,
    pub deck: Option<CprDeckChoice>,
}

/// The names the sheet knows, for reading kit items onto it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct SheetNames {
    weapons: BTreeSet<String>,
    /// Worn armour by name, with its SP.
    armor: BTreeMap<String, u32>,
    shields: BTreeSet<String>,
    /// Cyberdecks by name, with their slots.
    decks: Vec<(String, u32)>,
    programs: BTreeSet<String>,
    hardware: BTreeSet<String>,
}

/// A built kit and its money.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CprKitOutcome {
    pub method: CprKitMethod,
    pub items: Vec<CprKitItem>,
    pub budget: u32,
    pub spent: u32,
    pub fashion_budget: u32,
    pub fashion_spent: u32,
    budget_source: String,
    names: SheetNames,
}

#[derive(Debug, Clone)]
struct FixedKit {
    role: String,
    cash_eb: u32,
    source_page: String,
    lines: Vec<CprKitLine>,
}

/// The fixed kits and the shop.
#[derive(Debug, Clone)]
pub struct CprKitCatalog {
    kits: Vec<FixedKit>,
    shop: Vec<CprShopItem>,
    names: SheetNames,
}

fn read_records(corpus_dir: &Path, kind: &str) -> Result<Vec<Value>, String> {
    let path = corpus_dir.join(format!("{kind}.json"));
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
}

fn text(record: &Value, field: &str) -> String {
    record[field].as_str().unwrap_or_default().to_owned()
}

fn eb(value: &Value) -> Option<u32> {
    value.as_u64().and_then(|v| u32::try_from(v).ok())
}

fn count(value: &Value, what: &str) -> Result<u32, String> {
    eb(value).filter(|q| *q > 0).ok_or_else(|| format!("starting_kit: '{what}' has no quantity"))
}

fn options_of(line: &Value, role: &str, line_text: &str) -> Result<Vec<CprKitOption>, String> {
    let missing = |what: &str| format!("starting_kit: {role} '{line_text}' has no {what}");
    match line["fashion_style"].as_str() {
        Some(style) => line["pieces"]
            .as_array()
            .ok_or_else(|| missing("pieces"))?
            .iter()
            .map(|p| Ok(CprKitOption { name: format!("{style} {}", text(p, "name")), quantity: count(&p["quantity"], line_text)? }))
            .collect(),
        None => line["options"]
            .as_array()
            .ok_or_else(|| missing("options"))?
            .iter()
            .map(|o| Ok(CprKitOption { name: text(o, "name"), quantity: count(&o["quantity"], line_text)? }))
            .collect(),
    }
}

/// The bullet calibers (p344: pistols and rifles fire bullets).
fn is_bullet(caliber: &str) -> bool {
    caliber.ends_with("Pistol") || caliber == "Rifle"
}

/// One ammunition-family word of the p345 "Ammo Types Available" column, as calibers.
fn family(word: &str, calibers: &[String]) -> Result<Vec<String>, String> {
    let keep = |f: fn(&str) -> bool| calibers.iter().filter(|c| f(c)).cloned().collect::<Vec<_>>();
    Ok(match word {
        "Bullets" => keep(is_bullet),
        "Arrows" => keep(|c| c == "Arrow"),
        "Slugs" => keep(|c| c == "Slug"),
        "Shotgun Shells" => keep(|c| c == SHOTGUN_SHELL),
        "Grenades" => keep(|c| c == "Grenade"),
        "Rockets" => keep(|c| c == "Rocket"),
        other => return Err(format!("ammo: '{other}' is not an ammunition family (p345)")),
    })
}

/// The calibers a printed availability allows: "All except X and Y.", "X, Y, and Z.", "X and Y
/// only.".
fn availability(printed: &str, calibers: &[String]) -> Result<Vec<String>, String> {
    let printed = printed.trim().trim_end_matches('.');
    let (except, list) = match printed.strip_prefix("All except ") {
        Some(rest) => (true, rest),
        None => (false, printed.strip_suffix(" only").unwrap_or(printed)),
    };
    let mut named = BTreeSet::new();
    for word in list.split(',').flat_map(|p| p.split(" and ")) {
        let word = word.trim();
        let word = word.strip_prefix("and ").unwrap_or(word).trim();
        if !word.is_empty() {
            named.extend(family(word, calibers)?);
        }
    }
    Ok(calibers.iter().filter(|c| named.contains(*c) != except).cloned().collect())
}

/// `<armour> <Body|Head> Armor (SP<n>)`, as the kits print worn armour.
fn worn_armor(name: &str) -> Option<(&str, CprArmorLocation)> {
    let (rest, _sp) = name.strip_suffix(')')?.rsplit_once(" Armor (SP")?;
    let (armor, location) = rest.rsplit_once(' ')?;
    let location = match location {
        "Body" => CprArmorLocation::Body,
        "Head" => CprArmorLocation::Head,
        _ => return None,
    };
    Some((armor, location))
}

fn value(v: u32) -> i16 {
    i16::try_from(v).unwrap_or(i16::MAX)
}

impl CprKitCatalog {
    pub fn load(corpus_dir: &Path) -> Result<Self, String> {
        let mut kits = Vec::new();
        for record in read_records(corpus_dir, "starting_kit")? {
            let role = text(&record, "role");
            let mut lines = Vec::new();
            for section in ["weapons_armor", "outfit"] {
                let page = record["pages"][section].as_str().map_or_else(|| text(&record, "source_page"), str::to_owned);
                for line in record[section].as_array().ok_or_else(|| format!("starting_kit: {role} has no {section}"))? {
                    let line_text = text(line, "text");
                    let options = options_of(line, &role, &line_text)?;
                    let pick_one = line["pick_one"].as_bool() == Some(true);
                    if options.is_empty() || (pick_one && options.len() < 2) {
                        return Err(format!("starting_kit: {role} '{line_text}' has too few options"));
                    }
                    let fashion = line["fashion_style"].is_string();
                    lines.push(CprKitLine { section: section.to_owned(), text: line_text, source_page: page.clone(), pick_one, fashion, options });
                }
            }
            kits.push(FixedKit {
                cash_eb: eb(&record["cash_eb"]).ok_or_else(|| format!("starting_kit: {role} has no cash_eb"))?,
                source_page: text(&record, "source_page"),
                role,
                lines,
            });
        }

        let names_of = |kind: &str| -> Result<BTreeSet<String>, String> {
            Ok(read_records(corpus_dir, kind)?.iter().map(|r| text(r, "name")).collect())
        };
        let mut names = SheetNames {
            weapons: names_of("weapon")?,
            programs: names_of("program")?,
            hardware: names_of("net_hardware")?,
            ..SheetNames::default()
        };
        for record in read_records(corpus_dir, "armor")? {
            match eb(&record["sp"]) {
                Some(sp) => names.armor.insert(text(&record, "name"), sp),
                None => names.shields.insert(text(&record, "name")).then_some(0),
            };
        }
        for record in read_records(corpus_dir, "cyberdeck")? {
            let slots = eb(&record["slots"]).ok_or_else(|| format!("cyberdeck: {} has no slots", text(&record, "name")))?;
            names.decks.push((text(&record, "name"), slots));
        }
        let mut calibers: Vec<String> = Vec::new();
        for record in read_records(corpus_dir, "weapon")? {
            if let Some(c) = record["ammo_type"].as_str() {
                if !calibers.iter().any(|k| k == c) {
                    calibers.push(c.to_owned());
                }
            }
        }
        calibers.push(SHOTGUN_SHELL.to_owned());

        let mut shop = Vec::new();
        for (kind, field) in [
            ("weapon", "cost_eb"),
            ("attachment", "cost_eb"),
            ("ammo", "cost_eb"),
            ("armor", "cost_eb"),
            ("gear", "cost_eb"),
            ("fashion", ""),
            ("drug", "cost_per_dose_eb"),
            ("cyberdeck", "cost_eb"),
            ("net_hardware", "cost_eb"),
            ("program", "cost_eb"),
        ] {
            for record in read_records(corpus_dir, kind)? {
                let name = text(&record, "name");
                let source_page = text(&record, "source_page");
                if kind == "fashion" {
                    let prices = record["prices"].as_object().ok_or_else(|| format!("fashion: {name} has no prices"))?;
                    let variants = prices
                        .iter()
                        .map(|(piece, p)| {
                            let cost = eb(&p["cost_eb"]).ok_or_else(|| format!("fashion: {name} {piece} has no price"))?;
                            Ok(CprShopVariant { name: piece.clone(), cost_eb: Some(cost) })
                        })
                        .collect::<Result<Vec<_>, String>>()?;
                    shop.push(CprShopItem {
                        kind: kind.to_owned(),
                        name,
                        cost_eb: 0,
                        cost_text: "priced by the piece".to_owned(),
                        source_page,
                        variant_label: Some("Piece".to_owned()),
                        variant_note: String::new(),
                        variants,
                    });
                    continue;
                }
                let Some(cost) = eb(&record[field]) else { continue };
                let cost_text = record["cost_text"].as_str().map_or_else(|| format!("{cost}eb"), str::to_owned);
                let fixed = |label: &str, list: &[String]| {
                    (Some(label.to_owned()), list.iter().map(|v| CprShopVariant { name: v.clone(), cost_eb: None }).collect())
                };
                let (variant_label, variants, variant_note) = match kind {
                    "ammo" => {
                        let printed = text(&record, "ammo_types_available");
                        let allowed = availability(&printed, &calibers)?;
                        if allowed.is_empty() {
                            return Err(format!("ammo: {name} allows no caliber"));
                        }
                        let (label, variants) = fixed("Caliber", &allowed);
                        (label, variants, format!("{} {}", printed.trim(), text(&record, "purchase_rule")))
                    }
                    "armor" if names.armor.contains_key(&name) => {
                        let (label, variants) = fixed("Location", &["Body".to_owned(), "Head".to_owned()]);
                        (label, variants, String::new())
                    }
                    _ => (None, Vec::new(), String::new()),
                };
                shop.push(CprShopItem {
                    kind: kind.to_owned(),
                    name,
                    cost_eb: cost,
                    cost_text,
                    source_page,
                    variant_label,
                    variant_note,
                    variants,
                });
            }
        }
        Ok(Self { kits, shop, names })
    }

    /// Every buyable record: weapons, attachments, ammunition, armour, gear, fashion, street
    /// drugs, cyberdecks, cyberdeck hardware, programs, each kind in corpus order.
    pub fn shop(&self) -> &[CprShopItem] {
        &self.shop
    }

    /// The Role's fixed kit lines (weapons and armour, then outfit); `None` for a Role with no
    /// kit record.
    pub fn fixed_lines(&self, role: CprRole) -> Option<&[CprKitLine]> {
        self.kits.iter().find(|k| k.role == role.name()).map(|k| k.lines.as_slice())
    }

    /// One purchase priced: the item it gives, or why it is refused.
    fn price(&self, purchase: &CprKitPurchase) -> Result<CprKitItem, String> {
        let CprKitPurchase { kind, name, variant, quantity } = purchase;
        if *quantity == 0 {
            return Err(format!("{name}: buy a quantity of 1 or more"));
        }
        let item = self.shop.iter().find(|i| &i.kind == kind && &i.name == name).ok_or_else(|| {
            if self.shop.iter().any(|i| &i.kind == kind) {
                format!("'{name}' is not a priced {kind} of the Night Market lists (p340-p379)")
            } else {
                format!(
                    "a {kind} is not bought with the starting cash (the shop sells weapons, attachments, ammunition, armor, gear, fashion, street drugs, cyberdecks, cyberdeck hardware and programs)"
                )
            }
        })?;
        let listed = || item.variants.iter().map(|v| v.name.as_str()).collect::<Vec<_>>().join(", ");
        let picked = match (&item.variant_label, variant) {
            (None, None) => None,
            (None, Some(v)) if self.names.shields.contains(name) => {
                return Err(format!("{name} is held in one arm, not worn at a location (p350); it takes no '{v}'"))
            }
            (None, Some(v)) => return Err(format!("{name} takes no variant ('{v}')")),
            (Some(label), None) => {
                return Err(format!("{name}: name the {} ({})", label.to_lowercase(), listed().replace(", ", " or ")))
            }
            (Some(label), Some(v)) => Some(item.variants.iter().find(|x| &x.name == v).ok_or_else(|| {
                let note = if item.variant_note.is_empty() { String::new() } else { format!("; {}", item.variant_note) };
                format!("{name}: '{v}' is not a {} it comes in ({}{note})", label.to_lowercase(), listed())
            })?),
        };
        let (label, cost) = match (kind.as_str(), picked) {
            ("fashion", Some(piece)) => (format!("{name} {}", piece.name), piece.cost_eb.unwrap_or(0) * quantity),
            ("armor", Some(location)) => {
                let sp = self.names.armor.get(name).copied().unwrap_or(0);
                (format!("{name} {} Armor (SP{sp})", location.name), item.cost_eb * quantity)
            }
            ("ammo", Some(caliber)) => {
                let caliber = caliber.name.as_str();
                let prefix = name.strip_suffix(" Ammunition").unwrap_or(name);
                if caliber == "Grenade" || caliber == "Rocket" {
                    (format!("{prefix} {caliber}"), item.cost_eb * quantity)
                } else if quantity % AMMO_INCREMENT != 0 {
                    return Err(format!(
                        "{name} ({caliber}): {quantity} rounds; ammunition other than grenades and rockets is bought in increments of 10 (p345)"
                    ));
                } else {
                    (format!("{prefix} {caliber} Ammunition"), item.cost_eb * (quantity / AMMO_INCREMENT))
                }
            }
            _ => (name.clone(), item.cost_eb * quantity),
        };
        Ok(CprKitItem { section: "bought".to_owned(), kind: kind.clone(), name: label, quantity: *quantity, cost_eb: cost, fashion: kind == "fashion" })
    }

    /// The kit the method gives with the player's picks and purchases, or every reason it is
    /// refused.
    pub fn build(&self, role: CprRole, method: CprKitMethod, choices: &CprKitChoices) -> Result<CprKitOutcome, Vec<String>> {
        let mut reasons = Vec::new();
        let mut items = Vec::new();
        let (budget, fashion_budget, budget_source) = match method {
            CprKitMethod::Fixed => {
                let Some(kit) = self.kits.iter().find(|k| k.role == role.name()) else {
                    return Err(vec![format!("the {} has no starting kit record", role.name())]);
                };
                let mut picked: BTreeMap<usize, &str> = BTreeMap::new();
                for pick in &choices.picks {
                    let Some(line) = kit.lines.get(pick.line) else {
                        reasons.push(format!("the {} kit has no line {}", role.name(), pick.line));
                        continue;
                    };
                    if !line.pick_one {
                        reasons.push(format!("line {} ('{}') has no choice to pick", pick.line, line.text));
                    } else if !line.options.iter().any(|o| o.name == pick.option) {
                        reasons.push(format!("'{}' is not an option of '{}'", pick.option, line.text));
                    } else if picked.insert(pick.line, pick.option.as_str()).is_some() {
                        reasons.push(format!("'{}' is picked twice; pick one and only one (p099)", line.text));
                    }
                }
                for (index, line) in kit.lines.iter().enumerate() {
                    let take: Vec<&CprKitOption> = if line.pick_one {
                        let name = picked.get(&index).copied().unwrap_or(line.options[0].name.as_str());
                        line.options.iter().filter(|o| o.name == name).take(1).collect()
                    } else {
                        line.options.iter().collect()
                    };
                    for option in take {
                        items.push(CprKitItem {
                            section: line.section.clone(),
                            kind: String::new(),
                            name: option.name.clone(),
                            quantity: option.quantity,
                            cost_eb: 0,
                            fashion: line.fashion,
                        });
                    }
                }
                (kit.cash_eb, 0, format!("the Streetrat and Edgerunner {}eb ({})", kit.cash_eb, kit.source_page))
            }
            CprKitMethod::CompletePackage => {
                if !choices.picks.is_empty() {
                    reasons.push("a Complete Package has no fixed kit to pick from: everything is bought (p105)".to_owned());
                }
                (COMPLETE_PACKAGE_CASH_EB, COMPLETE_PACKAGE_FASHION_EB, format!("the Complete Package {COMPLETE_PACKAGE_CASH_EB}eb (p105)"))
            }
        };
        for purchase in &choices.purchases {
            match self.price(purchase) {
                Ok(item) => items.push(item),
                Err(reason) => reasons.push(reason),
            }
        }
        let bought = items.iter().filter(|i| i.section == "bought");
        let fashion_spent: u32 = bought.clone().filter(|i| i.fashion).map(|i| i.cost_eb).sum();
        let other_spent: u32 = bought.filter(|i| !i.fashion).map(|i| i.cost_eb).sum();
        let spent = other_spent + fashion_spent.saturating_sub(fashion_budget);
        if spent > budget {
            reasons.push(format!(
                "the purchases cost {spent}eb{}; the cash to spend is {budget}eb ({budget_source})",
                if fashion_budget > 0 { format!(" beyond the {fashion_budget}eb fashion budget") } else { String::new() }
            ));
        }
        if !reasons.is_empty() {
            return Err(reasons);
        }
        Ok(CprKitOutcome { method, items, budget, spent, fashion_budget, fashion_spent, budget_source, names: self.names.clone() })
    }
}

impl CprKitOutcome {
    /// The cash left: the budget minus the purchases (A-9).
    pub fn cash(&self) -> u32 {
        self.budget - self.spent
    }

    fn money_rows(&self, saved: bool) -> Vec<ComputationExplanation> {
        let row = |id: &str, v: u32, detail: String| ComputationExplanation { id: id.to_owned(), value: value(v), detail };
        let mut rows = vec![
            row("cpr.kit.budget", self.budget, format!("Starting cash {}eb: {}", self.budget, self.budget_source)),
            row(
                "cpr.kit.spent",
                self.spent,
                format!("Spent {}eb on {} purchase(s) at the printed prices", self.spent, self.items.iter().filter(|i| i.section == "bought").count()),
            ),
            row(
                "cpr.cash",
                self.cash(),
                format!(
                    "Cash {}eb = {}eb - {}eb spent at creation (A-9); after creation the cash is written in",
                    self.cash(),
                    self.budget,
                    self.spent
                ),
            ),
        ];
        if self.method == CprKitMethod::CompletePackage {
            let left = self.fashion_budget.saturating_sub(self.fashion_spent);
            rows.push(row("cpr.kit.fashion_budget", self.fashion_budget, format!("Fashion budget {}eb: only for Fashion or Fashionware (p106)", self.fashion_budget)));
            rows.push(row("cpr.kit.fashion_spent", self.fashion_spent, format!("Fashion bought: {}eb", self.fashion_spent)));
            rows.push(if saved {
                row("cpr.kit.fashion_left", 0, format!("Fashion budget left 0eb: the {left}eb unspent went away at creation (p106: \"Anything you don't spend goes away.\")"))
            } else {
                row("cpr.kit.fashion_left", left, format!("Fashion budget left {left}eb = {}eb - {}eb; lost if unspent (p106)", self.fashion_budget, self.fashion_spent.min(self.fashion_budget)))
            });
        }
        rows
    }

    /// The money rows while the character is being made.
    pub fn rows(&self) -> Vec<ComputationExplanation> {
        self.money_rows(false)
    }

    /// The money rows of the saved character: the fashion budget left is 0 (lost).
    pub fn rows_after_save(&self) -> Vec<ComputationExplanation> {
        self.money_rows(true)
    }

    /// What lands on the sheet: weapons by name, worn armour at its location, the first shield,
    /// ammunition, grenades and rockets by count, the first Cyberdeck with the programs and
    /// hardware of the kit (none without a deck).
    pub fn sheet(&self) -> CprKitSheet {
        let names = &self.names;
        let mut sheet = CprKitSheet::default();
        let mut programs = Vec::new();
        let mut hardware = Vec::new();
        for item in &self.items {
            let name = item.name.as_str();
            let deck_by_slots = name
                .strip_prefix(DECK_PREFIX)
                .and_then(|r| r.strip_suffix(" Slots)"))
                .and_then(|n| n.parse::<u32>().ok())
                .and_then(|slots| names.decks.iter().find(|(_, s)| *s == slots));
            if names.weapons.contains(name) {
                sheet.weapons.extend(std::iter::repeat_n(name.to_owned(), item.quantity as usize));
            } else if names.shields.contains(name) {
                sheet.shield.get_or_insert_with(|| name.to_owned());
            } else if let Some((armor, location)) = worn_armor(name).filter(|(a, _)| names.armor.contains_key(*a)) {
                sheet.armor.push(CprWornArmor { name: armor.to_owned(), location });
            } else if name.ends_with(" Ammunition") || name.ends_with(" Grenade") || name.ends_with(" Rocket") {
                sheet.ammunition.push((name.to_owned(), item.quantity));
            } else if let Some((deck, _)) = names.decks.iter().find(|(d, _)| d == name).or(deck_by_slots) {
                if sheet.deck.is_none() {
                    sheet.deck = Some(CprDeckChoice { deck: deck.clone(), programs: Vec::new(), hardware: Vec::new() });
                }
            } else if let Some(program) = name.strip_prefix(PROGRAM_PREFIX).or((item.kind == "program").then_some(name)) {
                if names.programs.contains(program) {
                    programs.extend(std::iter::repeat_n(program.to_owned(), item.quantity as usize));
                }
            } else if item.kind == "net_hardware" && names.hardware.contains(name) {
                hardware.extend(std::iter::repeat_n(name.to_owned(), item.quantity as usize));
            }
        }
        if let Some(deck) = sheet.deck.as_mut() {
            deck.programs = programs;
            deck.hardware = hardware;
        }
        sheet
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    use serde_json::Value;

    fn corpus_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/cyberpunk-red/corpus")
    }

    fn catalog() -> CprKitCatalog {
        CprKitCatalog::load(&corpus_dir()).expect("the kit catalog loads")
    }

    fn records(kind: &str) -> Vec<Value> {
        serde_json::from_str(&std::fs::read_to_string(corpus_dir().join(format!("{kind}.json"))).unwrap()).unwrap()
    }

    fn buy(kind: &str, name: &str, variant: Option<&str>, quantity: u32) -> CprKitPurchase {
        CprKitPurchase { kind: kind.to_owned(), name: name.to_owned(), variant: variant.map(str::to_owned), quantity }
    }

    fn row(rows: &[ComputationExplanation], id: &str) -> i16 {
        rows.iter().find(|r| r.id == id).unwrap_or_else(|| panic!("no row {id} in {rows:?}")).value
    }

    fn refused(outcome: Result<CprKitOutcome, Vec<String>>, words: &[&str]) {
        let reasons = outcome.expect_err("refused");
        assert!(reasons.iter().any(|r| words.iter().all(|w| r.contains(w))), "no reason with {words:?} in {reasons:?}");
    }

    /// Each Role's fixed kit (weapons and armour, then outfit) is its `starting_kit` record's
    /// lines, read here as plain JSON: a plain line gives every option at its quantity, an "or"
    /// line gives the option picked (the first when none is), a fashion line gives each piece.
    #[test]
    fn each_roles_fixed_kit_is_its_starting_kit_record() {
        let catalog = catalog();
        let kits = records("starting_kit");
        assert_eq!(kits.len(), 10);
        for kit in &kits {
            let role = CprRole::from_name(kit["role"].as_str().unwrap()).unwrap();
            let mut expected = Vec::new();
            for section in ["weapons_armor", "outfit"] {
                for line in kit[section].as_array().unwrap() {
                    if let Some(style) = line["fashion_style"].as_str() {
                        for piece in line["pieces"].as_array().unwrap() {
                            expected.push((
                                section.to_owned(),
                                format!("{style} {}", piece["name"].as_str().unwrap()),
                                piece["quantity"].as_u64().unwrap() as u32,
                            ));
                        }
                        continue;
                    }
                    let options = line["options"].as_array().unwrap();
                    let take: Vec<&Value> =
                        if line["pick_one"].as_bool() == Some(true) { vec![&options[0]] } else { options.iter().collect() };
                    for option in take {
                        expected.push((
                            section.to_owned(),
                            option["name"].as_str().unwrap().to_owned(),
                            option["quantity"].as_u64().unwrap() as u32,
                        ));
                    }
                }
            }
            let outcome = catalog.build(role, CprKitMethod::Fixed, &CprKitChoices::default()).expect("the default kit builds");
            let got: Vec<(String, String, u32)> =
                outcome.items.iter().map(|i| (i.section.clone(), i.name.clone(), i.quantity)).collect();
            assert_eq!(got, expected, "{}", role.name());
            assert!(outcome.items.iter().all(|i| i.cost_eb == 0), "a fixed kit costs nothing");
            assert_eq!(row(&outcome.rows(), "cpr.kit.budget"), kit["cash_eb"].as_i64().unwrap() as i16);
            assert_eq!(row(&outcome.rows(), "cpr.cash"), 500, "{}: 500eb kept", role.name());
        }
    }

    #[test]
    fn an_or_pick_takes_that_option_and_a_bad_pick_is_refused() {
        let catalog = catalog();
        let menu = catalog.fixed_lines(CprRole::Lawman).expect("Lawman kit");
        let (index, line) = menu.iter().enumerate().find(|(_, l)| l.text.starts_with("Basic Rifle Ammunition x100 or")).unwrap();
        assert!(line.pick_one);
        assert_eq!(line.options.len(), 3);
        let pick = |option: &str| CprKitChoices {
            picks: vec![CprKitPick { line: index, option: option.to_owned() }],
            purchases: Vec::new(),
        };
        let outcome = catalog.build(CprRole::Lawman, CprKitMethod::Fixed, &pick("Basic Slug Ammunition")).unwrap();
        assert!(outcome.items.iter().any(|i| i.name == "Basic Slug Ammunition" && i.quantity == 100));
        assert!(!outcome.items.iter().any(|i| i.name == "Basic Rifle Ammunition"));
        refused(catalog.build(CprRole::Lawman, CprKitMethod::Fixed, &pick("Basic Arrow Ammunition")), &["not an option"]);
        let plain = menu.iter().position(|l| !l.pick_one).unwrap();
        refused(
            catalog.build(
                CprRole::Lawman,
                CprKitMethod::Fixed,
                &CprKitChoices { picks: vec![CprKitPick { line: plain, option: "Assault Rifle".to_owned() }], purchases: Vec::new() },
            ),
            &["no choice"],
        );
        refused(
            catalog.build(
                CprRole::Lawman,
                CprKitMethod::Fixed,
                &CprKitChoices { picks: vec![CprKitPick { line: 99, option: "Shotgun".to_owned() }], purchases: Vec::new() },
            ),
            &["no line 99"],
        );
        refused(
            catalog.build(CprRole::Lawman, CprKitMethod::CompletePackage, &pick("Basic Slug Ammunition")),
            &["Complete Package", "no fixed kit"],
        );
    }

    /// Streetrat and Edgerunner spend their 500eb at the catalogs' prices (p099: "to spend on
    /// other items or keep"); the cash row is what is left.
    #[test]
    fn fixed_kit_purchases_spend_the_500eb_and_an_overspend_is_refused() {
        let catalog = catalog();
        let choices = CprKitChoices {
            picks: Vec::new(),
            purchases: vec![buy("weapon", "Heavy Pistol", None, 1), buy("gear", "Flashlight", None, 2)],
        };
        let outcome = catalog.build(CprRole::Solo, CprKitMethod::Fixed, &choices).unwrap();
        // Heavy Pistol 100eb (p. 341) + Flashlight 20eb x2.
        assert_eq!(row(&outcome.rows(), "cpr.kit.spent"), 140);
        assert_eq!(row(&outcome.rows(), "cpr.cash"), 360);
        let bought: Vec<(&str, u32, u32)> =
            outcome.items.iter().filter(|i| i.section == "bought").map(|i| (i.name.as_str(), i.quantity, i.cost_eb)).collect();
        assert_eq!(bought, vec![("Heavy Pistol", 1, 100), ("Flashlight", 2, 40)]);
        refused(
            catalog.build(
                CprRole::Solo,
                CprKitMethod::Fixed,
                &CprKitChoices { picks: Vec::new(), purchases: vec![buy("weapon", "Assault Rifle", None, 2)] },
            ),
            &["1000eb", "500eb"],
        );
    }

    /// The p105 example: Jay spends exactly 2,550eb (cyberware aside, which E3.7's step buys).
    /// The p106 example: 380eb of fashion from the 800eb fashion budget, 420eb lost.
    #[test]
    fn complete_package_buys_from_the_catalogs_with_2550eb_and_800eb_for_fashion() {
        let catalog = catalog();
        let choices = CprKitChoices {
            picks: Vec::new(),
            purchases: vec![
                buy("weapon", "Assault Rifle", None, 1),
                buy("weapon", "Heavy SMG", None, 1),
                buy("weapon", "Heavy Melee Weapon", None, 1),
                buy("armor", "Light Armorjack", Some("Body"), 1),
                buy("armor", "Light Armorjack", Some("Head"), 1),
                buy("ammo", "Basic Ammunition", Some("H Pistol"), 100),
                buy("ammo", "Basic Ammunition", Some("Rifle"), 100),
                buy("ammo", "Flashbang Ammunition", Some("Grenade"), 1),
                buy("ammo", "Smoke Ammunition", Some("Grenade"), 2),
                buy("gear", "Binoculars", None, 1),
                buy("fashion", "Nomad Leathers", Some("Bottoms"), 1),
                buy("fashion", "Nomad Leathers", Some("Top"), 1),
                buy("fashion", "Nomad Leathers", Some("Jacket"), 1),
                buy("fashion", "Nomad Leathers", Some("Footwear"), 1),
                buy("fashion", "Generic Chic", Some("Mirrorshades"), 1),
            ],
        };
        let outcome = catalog.build(CprRole::Solo, CprKitMethod::CompletePackage, &choices).unwrap();
        let rows = outcome.rows();
        assert_eq!(row(&rows, "cpr.kit.budget"), 2550);
        // 500 + 100 + 100 + 100 + 100 + 100 + 100 + 100 + 100 (2 x Smoke 50) + 50 (Binoculars).
        assert_eq!(row(&rows, "cpr.kit.spent"), 1350);
        assert_eq!(row(&rows, "cpr.cash"), 1200);
        assert_eq!(row(&rows, "cpr.kit.fashion_budget"), 800);
        // Nomad Leathers Bottoms 100, Top 20, Jacket 100, Footwear 100 (p106), Generic Chic
        // Mirrorshades 20.
        assert_eq!(row(&rows, "cpr.kit.fashion_spent"), 340);
        assert_eq!(row(&rows, "cpr.kit.fashion_left"), 460);
        let saved = outcome.rows_after_save();
        assert_eq!(row(&saved, "cpr.kit.fashion_left"), 0, "unspent fashion budget is lost (p106)");
        assert_eq!(row(&saved, "cpr.cash"), 1200, "the lost fashion budget never becomes cash");
        let labels: Vec<&str> = outcome.items.iter().map(|i| i.name.as_str()).collect();
        assert!(labels.contains(&"Light Armorjack Body Armor (SP11)"), "{labels:?}");
        assert!(labels.contains(&"Basic H Pistol Ammunition"), "{labels:?}");
        assert!(labels.contains(&"Flashbang Grenade"), "{labels:?}");
        assert!(labels.contains(&"Nomad Leathers Jacket"), "{labels:?}");
    }

    #[test]
    fn complete_package_fashion_over_800_is_paid_from_the_cash_and_an_overspend_is_refused() {
        let catalog = catalog();
        let price = |style: &str, piece: &str| {
            records("fashion").iter().find(|r| r["name"] == style).unwrap()["prices"][piece]["cost_eb"].as_i64().unwrap()
        };
        let jacket = price("Asia Pop", "Jacket");
        let n = (800 / jacket + 1) as u32;
        let outcome = catalog
            .build(
                CprRole::Media,
                CprKitMethod::CompletePackage,
                &CprKitChoices { picks: Vec::new(), purchases: vec![buy("fashion", "Asia Pop", Some("Jacket"), n)] },
            )
            .unwrap();
        let rows = outcome.rows();
        let over = i64::from(n) * jacket - 800;
        assert_eq!(row(&rows, "cpr.kit.fashion_left"), 0);
        assert_eq!(i64::from(row(&rows, "cpr.kit.spent")), over);
        assert_eq!(i64::from(row(&rows, "cpr.cash")), 2550 - over);
        refused(
            catalog.build(
                CprRole::Media,
                CprKitMethod::CompletePackage,
                &CprKitChoices { picks: Vec::new(), purchases: vec![buy("weapon", "Malorian Arms 3516", None, 1)] },
            ),
            &["10000eb", "2550eb"],
        );
    }

    #[test]
    fn a_purchase_the_catalogs_do_not_price_is_refused() {
        let catalog = catalog();
        let one = |p: CprKitPurchase| CprKitChoices { picks: Vec::new(), purchases: vec![p] };
        let cp = CprKitMethod::CompletePackage;
        refused(catalog.build(CprRole::Solo, cp, &one(buy("weapon", "Lightsaber", None, 1))), &["Lightsaber", "not"]);
        refused(catalog.build(CprRole::Solo, cp, &one(buy("vehicle", "Roadbike", None, 1))), &["vehicle"]);
        refused(catalog.build(CprRole::Solo, cp, &one(buy("gear", "Flashlight", None, 0))), &["quantity"]);
        refused(catalog.build(CprRole::Solo, cp, &one(buy("armor", "Light Armorjack", None, 1))), &["Body or Head"]);
        refused(catalog.build(CprRole::Solo, cp, &one(buy("armor", "Bulletproof Shield", Some("Body"), 1))), &["held"]);
        refused(catalog.build(CprRole::Solo, cp, &one(buy("fashion", "Asia Pop", Some("Cape"), 1))), &["Cape"]);
        // p345: "All other ammunition is purchased in increments of 10."
        refused(catalog.build(CprRole::Solo, cp, &one(buy("ammo", "Basic Ammunition", Some("Rifle"), 15))), &["increments of 10"]);
        // Flashbang: "Grenades only".
        refused(catalog.build(CprRole::Solo, cp, &one(buy("ammo", "Flashbang Ammunition", Some("Rifle"), 10))), &["Grenades only"]);
        refused(catalog.build(CprRole::Solo, cp, &one(buy("ammo", "Basic Ammunition", Some("Laser"), 10))), &["Laser"]);
        refused(catalog.build(CprRole::Solo, cp, &one(buy("weapon", "Heavy Pistol", Some("Gold"), 1))), &["takes no"]);
    }

    /// Every ammunition record's printed availability reads as a set of calibers; the pinned
    /// table is the p345 "Ammo Types Available" column.
    #[test]
    fn every_ammunition_type_reads_its_printed_availability() {
        let catalog = catalog();
        let menu = catalog.shop();
        let ammo: Vec<(&str, Vec<&str>)> = menu
            .iter()
            .filter(|i| i.kind == "ammo")
            .map(|i| (i.name.as_str(), i.variants.iter().map(|v| v.name.as_str()).collect()))
            .collect();
        let bullets = ["M Pistol", "H Pistol", "VH Pistol", "Rifle"];
        fn with<'a>(parts: &[&[&'a str]]) -> BTreeSet<&'a str> {
            parts.iter().flat_map(|p| p.iter().copied()).collect()
        }
        let expected: Vec<(&str, BTreeSet<&str>)> = vec![
            ("Basic Ammunition", with(&[&bullets, &["Slug", "Shotgun Shell", "Arrow"]])),
            ("Armor-Piercing Ammunition", with(&[&bullets, &["Slug", "Arrow", "Grenade", "Rocket"]])),
            ("Biotoxin Ammunition", with(&[&["Arrow", "Grenade"]])),
            ("EMP Ammunition", with(&[&["Grenade"]])),
            ("Expansive Ammunition", with(&[&bullets, &["Slug", "Arrow"]])),
            ("Flashbang Ammunition", with(&[&["Grenade"]])),
            ("Incendiary Ammunition", with(&[&bullets, &["Arrow", "Grenade", "Shotgun Shell"]])),
            ("Poison Ammunition", with(&[&["Arrow", "Grenade"]])),
            ("Rubber Ammunition", with(&[&bullets, &["Slug", "Arrow"]])),
            ("Sleep Ammunition", with(&[&["Arrow", "Grenade"]])),
            ("Smart Ammunition", with(&[&bullets, &["Arrow", "Rocket"]])),
            ("Smoke Ammunition", with(&[&["Grenade"]])),
            ("Teargas Ammunition", with(&[&["Grenade"]])),
        ];
        assert_eq!(ammo.len(), 13);
        for ((name, got), (want_name, want)) in ammo.iter().zip(&expected) {
            assert_eq!(name, want_name);
            assert_eq!(got.iter().copied().collect::<BTreeSet<&str>>(), *want, "{name}");
        }
    }

    /// The shop lists every priced record of the buyable kinds, at its printed price.
    #[test]
    fn the_shop_lists_every_priced_record_of_the_buyable_kinds() {
        let catalog = catalog();
        let menu = catalog.shop();
        for (kind, field) in [
            ("weapon", "cost_eb"),
            ("attachment", "cost_eb"),
            ("armor", "cost_eb"),
            ("gear", "cost_eb"),
            ("drug", "cost_per_dose_eb"),
            ("cyberdeck", "cost_eb"),
            ("net_hardware", "cost_eb"),
            ("program", "cost_eb"),
            ("ammo", "cost_eb"),
        ] {
            let priced: Vec<(String, i64)> = records(kind)
                .iter()
                .filter_map(|r| Some((r["name"].as_str()?.to_owned(), r[field].as_i64()?)))
                .collect();
            let listed: Vec<(String, i64)> =
                menu.iter().filter(|i| i.kind == kind).map(|i| (i.name.clone(), i64::from(i.cost_eb))).collect();
            assert_eq!(listed, priced, "{kind}");
        }
        let styles = records("fashion");
        let listed: Vec<&CprShopItem> = menu.iter().filter(|i| i.kind == "fashion").collect();
        assert_eq!(listed.len(), styles.len());
        for (item, style) in listed.iter().zip(&styles) {
            let pieces = style["prices"].as_object().unwrap();
            assert_eq!(item.variants.len(), pieces.len(), "{}", item.name);
            for v in &item.variants {
                assert_eq!(i64::from(v.cost_eb.unwrap()), pieces[&v.name]["cost_eb"].as_i64().unwrap(), "{} {}", item.name, v.name);
            }
        }
    }

    /// What lands on the sheet: weapons by name, worn armour at its location, the shield, the
    /// ammunition counts and the Netrunner's deck with its programs.
    #[test]
    fn the_kit_puts_weapons_armour_ammunition_and_the_deck_on_the_sheet() {
        let catalog = catalog();
        let solo = catalog.build(CprRole::Solo, CprKitMethod::Fixed, &CprKitChoices::default()).unwrap().sheet();
        assert!(solo.weapons.contains(&"Assault Rifle".to_owned()), "{:?}", solo.weapons);
        assert_eq!(
            solo.armor,
            vec![
                CprWornArmor { name: "Light Armorjack".to_owned(), location: CprArmorLocation::Body },
                CprWornArmor { name: "Light Armorjack".to_owned(), location: CprArmorLocation::Head },
            ]
        );
        assert!(solo.ammunition.iter().any(|(label, n)| label == "Basic Rifle Ammunition" && *n == 70), "{:?}", solo.ammunition);
        let menu = catalog.fixed_lines(CprRole::Solo).unwrap();
        let shield_line = menu.iter().position(|l| l.text == "Heavy Melee Weapon or Bulletproof Shield").unwrap();
        let shielded = catalog
            .build(
                CprRole::Solo,
                CprKitMethod::Fixed,
                &CprKitChoices {
                    picks: vec![CprKitPick { line: shield_line, option: "Bulletproof Shield".to_owned() }],
                    purchases: Vec::new(),
                },
            )
            .unwrap()
            .sheet();
        assert_eq!(shielded.shield.as_deref(), Some("Bulletproof Shield"));
        let runner = catalog.build(CprRole::Netrunner, CprKitMethod::Fixed, &CprKitChoices::default()).unwrap().sheet();
        let deck = runner.deck.expect("the Netrunner's Cyberdeck (7 Slots)");
        assert_eq!(deck.deck, "Cyberdeck (Standard Quality)");
        assert_eq!(deck.programs, vec!["Armor", "Sword", "See Ya", "Sword", "Worm"], "the plain lines, then each \"or\" line's first option");
        // Programs bought without a deck stay in the kit, off the deck.
        let no_deck = catalog
            .build(
                CprRole::Solo,
                CprKitMethod::CompletePackage,
                &CprKitChoices { picks: Vec::new(), purchases: vec![buy("program", "Sword", None, 1)] },
            )
            .unwrap()
            .sheet();
        assert!(no_deck.deck.is_none());
    }
}
