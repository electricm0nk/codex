//! Cyberware: the sheet numbers installed cyberware changes (Cyberpunk RED core rulebook
//! p. 111-118, full listing p. 359-368, therapy p. 230-231).
//!
//! Computed here (every value a `cpr.cyberware.` row): the preset Humanity Loss sum (p. 112:
//! "At Character Generation, Humanity Loss is preset"), the option slots used in each
//! container with an over-slot install refused (D-11), the cyberware SP for body and head
//! (Skin Weave 7, Subdermal Armor 11, highest wins), BODY after Grafted Muscle and Bone Lace
//! (+2, never to 11 or higher) and the Linear Frames (Sigma 12, Beta 14, with their
//! prerequisites), the always-on Kerenzikov Initiative bonus, and the therapy cap on maximum
//! Humanity (D-5). Every rule value comes from the corpus records; post-creation dice Humanity
//! Loss (A-3) is entered elsewhere.

use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;

use serde::Deserialize;

use crate::rules_core::pilot_compute::ComputationExplanation;

/// The foundation items the option pools hang from (p. 112: "Most classifications of
/// cyberware have a foundational piece of cyberware").
const NEURAL_LINK: &str = "Neural Link";
const CHIPWARE_SOCKET: &str = "Chipware Socket";
const CYBEREYE: &str = "Cybereye";
const CYBERAUDIO_SUITE: &str = "Cyberaudio Suite";
const CYBERARM: &str = "Cyberarm";
const CYBERLEG: &str = "Cyberleg";
/// A body's own eyes, arms and legs: two each. MultiOptic Mount ("additional Cybereyes") and
/// Artificial Shoulder Mount (two more Cyberarms) raise the count (p. 368).
const NATURAL_PAIR: usize = 2;
const BORGWARE: &str = "Borgware";

#[derive(Debug, Clone, Default, Deserialize)]
struct Effects {
    body_add: Option<u8>,
    body_set: Option<u8>,
    body_cap_exclusive: Option<u8>,
    sp_body: Option<u8>,
    sp_head: Option<u8>,
    initiative_bonus: Option<i16>,
}

#[derive(Debug, Clone, Deserialize)]
struct Requirement {
    name: String,
    count: usize,
}

#[derive(Debug, Clone, Deserialize)]
struct Prerequisites {
    body_min: Option<u8>,
    #[serde(default)]
    requires: Vec<Requirement>,
}

/// One `cyberware` corpus record, the fields the engine reads.
#[derive(Debug, Clone, Deserialize)]
struct CyberwareItem {
    name: String,
    category: String,
    #[serde(default)]
    subcategory: String,
    humanity_loss_preset: i16,
    slot_pool: Option<String>,
    pool_capacity: Option<u8>,
    slots_used: u8,
    slots_provided: Option<u8>,
    requires_count: Option<usize>,
    only_one_installable: bool,
    meat_limb_ok: bool,
    limb_group: Option<String>,
    #[serde(default)]
    speedware: bool,
    free_with: Option<String>,
    #[serde(default)]
    effects: Effects,
    prerequisites: Option<Prerequisites>,
    #[serde(default)]
    adds_foundations: BTreeMap<String, usize>,
    #[serde(default)]
    adds_foundations_up_to: BTreeMap<String, usize>,
    #[serde(default)]
    adds_option_slots: BTreeMap<String, u8>,
}

#[derive(Debug, Clone, Deserialize)]
struct TherapyRecord {
    name: String,
    max_humanity_penalty: Option<PenaltyRecord>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
struct PenaltyRecord {
    per_cyberware: i16,
    per_borgware: i16,
    zero_humanity_loss_excluded: bool,
}

/// p. 231: "Each piece of cyberware will decrease your maximum Humanity by 2. Each piece of
/// borgware cyberware lowers maximum Humanity by 4 instead."
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaxHumanityPenalty {
    pub per_cyberware: i16,
    pub per_borgware: i16,
}

/// Where an option sits: on which Cybereye, Cyberarm or Cyberleg (0-based, in install order of
/// that foundation), or alone in a meat arm or leg (0 or 1) for the options the book allows
/// there ("may be the only piece of cyberware in a meat arm").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CprMount {
    Eye(u8),
    Arm(u8),
    Leg(u8),
    MeatArm(u8),
    MeatLeg(u8),
}

/// One installed piece. Each purchase is one install: a paired option bought for two eyes is
/// two installs, one per eye (p. 361).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CprCyberwareInstall {
    pub name: String,
    pub mount: Option<CprMount>,
}

/// An option-slot container (D-11).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CprContainer {
    Fashionware,
    Internal,
    External,
    NeuralLink,
    Cybereye(u8),
    CyberaudioSuite,
    Cyberarm(u8),
    Cyberleg(u8),
}

impl CprContainer {
    /// The `cpr.cyberware.slots.<id>.` row segment.
    pub fn id(self) -> String {
        match self {
            CprContainer::Fashionware => "fashionware".to_string(),
            CprContainer::Internal => "internal".to_string(),
            CprContainer::External => "external".to_string(),
            CprContainer::NeuralLink => "neural_link".to_string(),
            CprContainer::Cybereye(n) => format!("cybereye_{}", u16::from(n) + 1),
            CprContainer::CyberaudioSuite => "cyberaudio_suite".to_string(),
            CprContainer::Cyberarm(n) => format!("cyberarm_{}", u16::from(n) + 1),
            CprContainer::Cyberleg(n) => format!("cyberleg_{}", u16::from(n) + 1),
        }
    }

    pub fn label(self) -> String {
        match self {
            CprContainer::Fashionware => "Fashionware".to_string(),
            CprContainer::Internal => "Internal Body".to_string(),
            CprContainer::External => "External Body".to_string(),
            CprContainer::NeuralLink => NEURAL_LINK.to_string(),
            CprContainer::Cybereye(n) => format!("{CYBEREYE} {}", u16::from(n) + 1),
            CprContainer::CyberaudioSuite => CYBERAUDIO_SUITE.to_string(),
            CprContainer::Cyberarm(n) => format!("{CYBERARM} {}", u16::from(n) + 1),
            CprContainer::Cyberleg(n) => format!("{CYBERLEG} {}", u16::from(n) + 1),
        }
    }
}

/// Why an install list is refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CprCyberwareRefusal {
    /// D-11: the install needs more option slots than the container has.
    SlotsFull { item: String, container: CprContainer, capacity: u8, requested: u8 },
    /// Any other printed install rule (foundation, pairing, only one, prerequisites, mount).
    Rule { item: String, reason: String },
}

impl fmt::Display for CprCyberwareRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CprCyberwareRefusal::SlotsFull { item, container, capacity, requested } => write!(
                f,
                "{item} does not fit: {} would use {requested} of {capacity} option slots",
                container.label()
            ),
            CprCyberwareRefusal::Rule { item, reason } => write!(f, "{item}: {reason}"),
        }
    }
}

impl std::error::Error for CprCyberwareRefusal {}

fn rule(item: &str, reason: impl Into<String>) -> CprCyberwareRefusal {
    CprCyberwareRefusal::Rule { item: item.to_string(), reason: reason.into() }
}

/// The cyberware catalog (`cyberware.json`) and the therapy max-Humanity penalty
/// (`therapy.json`), read from a CPR corpus directory.
#[derive(Debug, Clone)]
pub struct CprCyberwareCatalog {
    items: BTreeMap<String, CyberwareItem>,
    penalty: MaxHumanityPenalty,
    zero_humanity_loss_excluded: bool,
    /// Capacity of the foundation-less pools (`fashion`, `internal`, `external`).
    pool_capacity: BTreeMap<String, u8>,
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
}

impl CprCyberwareCatalog {
    /// Load `cyberware.json` and `therapy.json` from `corpus_dir`
    /// (`GameSystem::CyberpunkRed.package_roots(..).corpus`).
    pub fn load(corpus_dir: &Path) -> Result<Self, String> {
        let records: Vec<CyberwareItem> = read_json(&corpus_dir.join("cyberware.json"))?;
        let mut items = BTreeMap::new();
        let mut pool_capacity = BTreeMap::new();
        for item in records {
            if let (Some(pool), Some(capacity)) = (&item.slot_pool, item.pool_capacity)
                && *pool_capacity.entry(pool.clone()).or_insert(capacity) != capacity
            {
                return Err(format!("cyberware.json: pool {pool} has two capacities"));
            }
            let name = item.name.clone();
            if items.insert(name.clone(), item).is_some() {
                return Err(format!("cyberware.json: {name} is listed twice"));
            }
        }
        for foundation in [NEURAL_LINK, CYBEREYE, CYBERAUDIO_SUITE, CYBERARM, CYBERLEG] {
            if items.get(foundation).and_then(|f| f.slots_provided).is_none() {
                return Err(format!("cyberware.json: foundation {foundation} has no slots_provided"));
            }
        }
        if !items.contains_key(CHIPWARE_SOCKET) {
            return Err(format!("cyberware.json: no {CHIPWARE_SOCKET} record"));
        }
        for pool in ["fashion", "internal", "external"] {
            if !pool_capacity.contains_key(pool) {
                return Err(format!("cyberware.json: pool {pool} has no pool_capacity"));
            }
        }

        let therapies: Vec<TherapyRecord> = read_json(&corpus_dir.join("therapy.json"))?;
        let mut penalty: Option<PenaltyRecord> = None;
        for therapy in &therapies {
            if let Some(p) = therapy.max_humanity_penalty {
                match penalty {
                    Some(seen) if seen != p => {
                        return Err(format!("therapy.json: {} prints a different max-Humanity penalty", therapy.name))
                    }
                    _ => penalty = Some(p),
                }
            }
        }
        let penalty = penalty.ok_or("therapy.json: no record carries max_humanity_penalty")?;
        Ok(CprCyberwareCatalog {
            items,
            penalty: MaxHumanityPenalty { per_cyberware: penalty.per_cyberware, per_borgware: penalty.per_borgware },
            zero_humanity_loss_excluded: penalty.zero_humanity_loss_excluded,
            pool_capacity,
        })
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn max_humanity_penalty(&self) -> MaxHumanityPenalty {
        self.penalty
    }

    fn slots_provided(&self, foundation: &str) -> u8 {
        self.items.get(foundation).and_then(|f| f.slots_provided).unwrap_or(0)
    }

    /// The record a printed name is (the p117 Role packs and the p105 example print names the
    /// p359-368 listing spells differently): the record's own name; the name without spaces or
    /// case ("Tech Hair" is Techhair); or the record's name followed by the word its subcategory
    /// brackets ("Sandevistan Speedware" is Sandevistan, "Neuralware (Speedware)").
    pub fn record_name(&self, printed: &str) -> Option<&str> {
        if let Some((name, _)) = self.items.get_key_value(printed) {
            return Some(name.as_str());
        }
        let squash = |s: &str| s.chars().filter(|c| !c.is_whitespace()).flat_map(char::to_lowercase).collect::<String>();
        let wanted = squash(printed);
        let mut found = self.items.values().filter(|it| squash(&it.name) == wanted);
        if let (Some(item), None) = (found.next(), found.next()) {
            return Some(item.name.as_str());
        }
        let mut found = self.items.values().filter(|it| {
            printed.strip_prefix(it.name.as_str()).and_then(|rest| rest.strip_prefix(' ')).is_some_and(|word| {
                it.subcategory
                    .split_once('(')
                    .and_then(|(_, bracket)| bracket.strip_suffix(')'))
                    .is_some_and(|bracketed| bracketed.eq_ignore_ascii_case(word))
            })
        });
        match (found.next(), found.next()) {
            (Some(item), None) => Some(item.name.as_str()),
            _ => None,
        }
    }

    /// Creation-time placement: each printed name resolved to its record ([`Self::record_name`]),
    /// and each option that sits on a Cybereye, Cyberarm or Cyberleg given its mount: the first
    /// such foundation of the list with option slots left (a paired option, one per eye, p. 361,
    /// prefers a foundation that does not hold it yet), else, for an option the book allows
    /// alone in a meat limb, the first meat arm or leg not yet holding a piece (p. 365, p. 367).
    /// An option with no room is left on the first foundation, so [`Self::check`] refuses it as
    /// over the slots (D-11).
    pub fn place(&self, printed: &[String]) -> Result<Vec<CprCyberwareInstall>, CprCyberwareRefusal> {
        let names: Vec<&str> = printed
            .iter()
            .map(|p| self.record_name(p).ok_or_else(|| rule(p, "is not in the cyberware catalog (p. 112-117)")))
            .collect::<Result<_, _>>()?;
        let count = |foundation: &str| names.iter().filter(|n| **n == foundation).count();
        let (eyes, arms, legs) = (count(CYBEREYE), count(CYBERARM), count(CYBERLEG));
        let mut used: BTreeMap<CprMount, u8> = BTreeMap::new();
        let mut held: BTreeMap<CprMount, Vec<&str>> = BTreeMap::new();
        let mut meat_taken: Vec<CprMount> = Vec::new();
        let mut out = Vec::with_capacity(names.len());
        for name in names {
            let item = &self.items[name];
            let on_foundation = |make: fn(u8) -> CprMount, n: usize, foundation: &str, used: &BTreeMap<CprMount, u8>, held: &BTreeMap<CprMount, Vec<&str>>| {
                let cap = self.slots_provided(foundation);
                let room: Vec<CprMount> = (0..n)
                    .map(|i| make(i as u8))
                    .filter(|m| used.get(m).copied().unwrap_or(0) + item.slots_used <= cap)
                    .collect();
                let fresh = room.iter().copied().find(|m| !held.get(m).is_some_and(|h| h.contains(&name)));
                fresh.or(room.first().copied())
            };
            let meat = |make: fn(u8) -> CprMount, taken: &[CprMount]| {
                (0..NATURAL_PAIR as u8).map(make).find(|m| !taken.contains(m)).filter(|_| item.meat_limb_ok)
            };
            let arm = || on_foundation(CprMount::Arm, arms, CYBERARM, &used, &held);
            let leg = || on_foundation(CprMount::Leg, legs, CYBERLEG, &used, &held);
            let mount = match item.slot_pool.as_deref() {
                Some("cybereye") => on_foundation(CprMount::Eye, eyes, CYBEREYE, &used, &held).or((eyes > 0).then_some(CprMount::Eye(0))),
                Some("cyberarm") => arm()
                    .or_else(|| meat(CprMount::MeatArm, &meat_taken))
                    .or((arms > 0).then_some(CprMount::Arm(0)))
                    .or(item.meat_limb_ok.then_some(CprMount::MeatArm(0))),
                Some("cyberleg") => leg()
                    .or_else(|| meat(CprMount::MeatLeg, &meat_taken))
                    .or((legs > 0).then_some(CprMount::Leg(0)))
                    .or(item.meat_limb_ok.then_some(CprMount::MeatLeg(0))),
                Some("cyberlimb") => arm().or_else(leg).or((arms > 0).then_some(CprMount::Arm(0))).or((legs > 0).then_some(CprMount::Leg(0))),
                _ => None,
            };
            match mount {
                Some(m @ (CprMount::MeatArm(_) | CprMount::MeatLeg(_))) => meat_taken.push(m),
                Some(m) => {
                    *used.entry(m).or_insert(0) += item.slots_used;
                    held.entry(m).or_default().push(name);
                }
                None => {}
            }
            out.push(CprCyberwareInstall { name: name.to_owned(), mount });
        }
        Ok(out)
    }

    /// Check `installs` against every printed install rule and compute the loadout.
    /// `base_body` is the BODY STAT before cyberware (the Linear Frame prerequisites read it).
    pub fn check(&self, base_body: u8, installs: &[CprCyberwareInstall]) -> Result<CprCyberwareLoadout, CprCyberwareRefusal> {
        let items: Vec<&CyberwareItem> = installs
            .iter()
            .map(|install| {
                self.items
                    .get(&install.name)
                    .ok_or_else(|| rule(&install.name, "is not in the cyberware catalog (p. 112-117)"))
            })
            .collect::<Result<_, _>>()?;
        let count = |name: &str| items.iter().filter(|it| it.name == name).count();

        // Only one of an item, and one Speedware at a time (p. 360).
        let mut speedware = 0;
        for item in &items {
            if item.only_one_installable && count(&item.name) > 1 {
                return Err(rule(&item.name, "only one can be installed"));
            }
            if item.speedware {
                speedware += 1;
                if speedware > 1 {
                    return Err(rule(&item.name, "only one Speedware can be installed at a time (p. 360)"));
                }
            }
        }

        // How many eyes, arms and legs the body has room for.
        let extra = |foundation: &str| -> usize {
            items
                .iter()
                .map(|it| {
                    it.adds_foundations.get(foundation).or_else(|| it.adds_foundations_up_to.get(foundation)).copied().unwrap_or(0)
                })
                .sum()
        };
        let mut foundations = BTreeMap::new();
        for foundation in [CYBEREYE, CYBERARM, CYBERLEG] {
            let installed = count(foundation);
            let room = NATURAL_PAIR + extra(foundation);
            if installed > room {
                return Err(rule(foundation, format!("{installed} installed, the body has room for {room}")));
            }
            foundations.insert(foundation, installed);
        }
        let eyes = foundations[CYBEREYE];
        let arms = foundations[CYBERARM];
        let legs = foundations[CYBERLEG];
        let neural_links = count(NEURAL_LINK);
        let sockets = count(CHIPWARE_SOCKET);
        let suites = count(CYBERAUDIO_SUITE);

        // Printed prerequisites (the Linear Frames, p. 368).
        for item in &items {
            if let Some(prerequisites) = &item.prerequisites {
                if let Some(min) = prerequisites.body_min
                    && base_body < min
                {
                    return Err(rule(&item.name, format!("installation requires BODY {min}; BODY is {base_body}")));
                }
                for required in &prerequisites.requires {
                    if count(&required.name) < required.count {
                        return Err(rule(&item.name, format!("installation requires {} {}", required.count, required.name)));
                    }
                }
            }
            for pool in item.adds_option_slots.keys() {
                if pool == "cyberaudio_suite" && suites == 0 {
                    return Err(rule(&item.name, format!("requires a {CYBERAUDIO_SUITE}")));
                }
            }
        }

        // Option slots per container (D-11), in install order: the install that overflows is refused.
        let mut used: BTreeMap<CprContainer, u8> = BTreeMap::new();
        let mut capacity: BTreeMap<CprContainer, u8> = BTreeMap::new();
        for pool in ["fashion", "internal", "external"] {
            let container = match pool {
                "fashion" => CprContainer::Fashionware,
                "internal" => CprContainer::Internal,
                _ => CprContainer::External,
            };
            capacity.insert(container, self.pool_capacity[pool]);
        }
        if neural_links > 0 {
            capacity.insert(CprContainer::NeuralLink, self.slots_provided(NEURAL_LINK).saturating_mul(neural_links as u8));
        }
        if suites > 0 {
            let added: u8 = items.iter().filter_map(|it| it.adds_option_slots.get("cyberaudio_suite")).sum();
            capacity.insert(CprContainer::CyberaudioSuite, self.slots_provided(CYBERAUDIO_SUITE) + added);
        }
        for n in 0..eyes {
            capacity.insert(CprContainer::Cybereye(n as u8), self.slots_provided(CYBEREYE));
        }
        for n in 0..arms {
            capacity.insert(CprContainer::Cyberarm(n as u8), self.slots_provided(CYBERARM));
        }
        for n in 0..legs {
            capacity.insert(CprContainer::Cyberleg(n as u8), self.slots_provided(CYBERLEG));
        }
        for container in capacity.keys() {
            used.insert(*container, 0);
        }

        let mut meat_limbs: BTreeMap<CprMount, &str> = BTreeMap::new();
        let mut chips = 0;
        for (install, item) in installs.iter().zip(&items) {
            let name = item.name.as_str();
            let no_mount = |container: Option<CprContainer>| -> Result<Option<CprContainer>, CprCyberwareRefusal> {
                match install.mount {
                    None => Ok(container),
                    Some(mount) => Err(rule(name, format!("takes no mount, got {mount:?}"))),
                }
            };
            let mut meat_piece = |mount: CprMount| -> Result<(), CprCyberwareRefusal> {
                let index = match mount {
                    CprMount::MeatArm(n) | CprMount::MeatLeg(n) => n,
                    _ => return Ok(()),
                };
                if usize::from(index) >= NATURAL_PAIR {
                    return Err(rule(name, format!("there is no meat limb {mount:?}")));
                }
                if item.free_with.is_some() {
                    // A Standard Hand / Foot in a meat limb is not counted as a piece there (p. 365, p. 367).
                    return Ok(());
                }
                if let Some(other) = meat_limbs.insert(mount, name) {
                    return Err(rule(name, format!("must be the only cyberware in a meat limb; {other} is already in it")));
                }
                Ok(())
            };
            let limb = |mount: Option<CprMount>, arm_ok: bool, leg_ok: bool| -> Result<Option<CprContainer>, CprCyberwareRefusal> {
                match mount {
                    Some(CprMount::Arm(n)) if arm_ok && usize::from(n) < arms => Ok(Some(CprContainer::Cyberarm(n))),
                    Some(CprMount::Leg(n)) if leg_ok && usize::from(n) < legs => Ok(Some(CprContainer::Cyberleg(n))),
                    Some(CprMount::MeatArm(_)) if arm_ok && item.meat_limb_ok => Ok(None),
                    Some(CprMount::MeatLeg(_)) if leg_ok && item.meat_limb_ok => Ok(None),
                    Some(mount) => Err(rule(name, format!("cannot be mounted at {mount:?}"))),
                    None => Err(rule(name, "needs a mount: which Cyberarm, Cyberleg or meat limb holds it")),
                }
            };
            let container = match item.slot_pool.as_deref() {
                None => match (install.mount, item.free_with.as_deref()) {
                    (Some(mount @ CprMount::MeatArm(_)), Some(CYBERARM)) | (Some(mount @ CprMount::MeatLeg(_)), Some(CYBERLEG)) => {
                        meat_piece(mount)?;
                        None
                    }
                    _ => no_mount(None)?,
                },
                Some("fashion") => no_mount(Some(CprContainer::Fashionware))?,
                Some("internal") => no_mount(Some(CprContainer::Internal))?,
                Some("external") => no_mount(Some(CprContainer::External))?,
                Some("neural_link") => {
                    if neural_links == 0 {
                        return Err(rule(name, format!("requires a {NEURAL_LINK}")));
                    }
                    // Subdermal Grip also sits in an arm without using its slots (p. 367).
                    match install.mount {
                        None => {}
                        Some(mount @ CprMount::MeatArm(_)) if item.meat_limb_ok => meat_piece(mount)?,
                        Some(CprMount::Arm(n)) if item.limb_group.as_deref() == Some("arm") && usize::from(n) < arms => {}
                        Some(mount) => return Err(rule(name, format!("cannot be mounted at {mount:?}"))),
                    }
                    Some(CprContainer::NeuralLink)
                }
                Some("chipware_socket") => {
                    chips += 1;
                    if chips > sockets {
                        return Err(rule(name, format!("Chipware needs its own {CHIPWARE_SOCKET}: {chips} chips, {sockets} sockets")));
                    }
                    no_mount(None)?
                }
                Some("cyberaudio_suite") => {
                    if suites == 0 {
                        return Err(rule(name, format!("requires a {CYBERAUDIO_SUITE}")));
                    }
                    no_mount(Some(CprContainer::CyberaudioSuite))?
                }
                Some("cybereye") => {
                    if let Some(pair) = item.requires_count
                        && eyes < pair
                    {
                        return Err(rule(name, format!("is paired: needs {pair} Cybereyes, {eyes} installed")));
                    }
                    match install.mount {
                        Some(CprMount::Eye(n)) if usize::from(n) < eyes => Some(CprContainer::Cybereye(n)),
                        Some(mount) => return Err(rule(name, format!("cannot be mounted at {mount:?}"))),
                        None => return Err(rule(name, "needs a mount: which Cybereye holds it")),
                    }
                }
                Some("cyberarm") => {
                    let container = limb(install.mount, true, false)?;
                    if container.is_none() {
                        meat_piece(install.mount.expect("limb() refuses a missing mount"))?;
                    }
                    container
                }
                Some("cyberleg") => {
                    if let Some(pair) = item.requires_count
                        && legs < pair
                    {
                        return Err(rule(name, format!("is paired: needs {pair} Cyberlegs, {legs} installed")));
                    }
                    let container = limb(install.mount, false, true)?;
                    if container.is_none() {
                        meat_piece(install.mount.expect("limb() refuses a missing mount"))?;
                    }
                    container
                }
                Some("cyberlimb") => match install.mount {
                    Some(CprMount::MeatArm(_) | CprMount::MeatLeg(_)) | None => {
                        return Err(rule(name, "needs a Cyberarm or Cyberleg mount"))
                    }
                    mount => limb(mount, true, true)?,
                },
                Some(other) => return Err(rule(name, format!("unknown option pool {other}"))),
            };
            if let Some(container) = container {
                let slot = used.get_mut(&container).expect("every reachable container has a capacity");
                let requested = *slot + item.slots_used;
                let cap = capacity[&container];
                if requested > cap {
                    return Err(CprCyberwareRefusal::SlotsFull {
                        item: name.to_string(),
                        container,
                        capacity: cap,
                        requested,
                    });
                }
                *slot = requested;
            }
        }

        // Effects that land on the sheet.
        let humanity_loss = items.iter().map(|it| it.humanity_loss_preset).sum();
        let counts_against_cap = |it: &&&CyberwareItem| !self.zero_humanity_loss_excluded || it.humanity_loss_preset > 0;
        let borgware = items.iter().filter(counts_against_cap).filter(|it| it.category == BORGWARE).count() as i16;
        let pieces = items.iter().filter(counts_against_cap).filter(|it| it.category != BORGWARE).count() as i16;

        let mut body = base_body;
        for effects in items.iter().map(|it| &it.effects) {
            if let Some(add) = effects.body_add {
                let raised = body + add;
                body = match effects.body_cap_exclusive {
                    Some(cap) if raised >= cap => body.max(cap - 1),
                    _ => raised,
                };
            }
        }
        for effects in items.iter().map(|it| &it.effects) {
            if let Some(set) = effects.body_set {
                body = body.max(set);
            }
        }
        let sp_body = items.iter().filter_map(|it| it.effects.sp_body).max().unwrap_or(0);
        let sp_head = items.iter().filter_map(|it| it.effects.sp_head).max().unwrap_or(0);
        let initiative_bonus = items.iter().filter_map(|it| it.effects.initiative_bonus).sum();

        let slots = capacity.iter().map(|(container, cap)| (*container, used[container], *cap)).collect();
        Ok(CprCyberwareLoadout {
            pieces_installed: items.len(),
            installed: items.iter().map(|it| (it.name.clone(), it.humanity_loss_preset)).collect(),
            humanity_loss,
            pieces,
            borgware,
            penalty: self.penalty,
            base_body,
            body,
            sp_body,
            sp_head,
            initiative_bonus,
            slots,
        })
    }
}

/// An install list that passed every rule, and the numbers it puts on the sheet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CprCyberwareLoadout {
    pieces_installed: usize,
    /// Each installed piece in install order: its name and its preset Humanity Loss (p. 112).
    installed: Vec<(String, i16)>,
    humanity_loss: i16,
    /// Installed non-borgware pieces that lower maximum Humanity (HL > 0, p. 231).
    pieces: i16,
    borgware: i16,
    penalty: MaxHumanityPenalty,
    base_body: u8,
    body: u8,
    sp_body: u8,
    sp_head: u8,
    initiative_bonus: i16,
    slots: Vec<(CprContainer, u8, u8)>,
}

impl CprCyberwareLoadout {
    /// Each installed piece in install order: its name and its preset Humanity Loss (p. 112).
    pub fn installed(&self) -> &[(String, i16)] {
        &self.installed
    }

    /// The sum of the installed pieces' preset Humanity Loss.
    pub fn humanity_loss(&self) -> i16 {
        self.humanity_loss
    }

    /// BODY after cyberware.
    pub fn body(&self) -> u8 {
        self.body
    }

    /// D-5: maximum Humanity = 10 × base EMP − 2 per installed piece − 4 per borgware piece;
    /// 0-HL pieces excluded.
    pub fn humanity_cap(&self, base_emp: u8) -> i16 {
        10 * i16::from(base_emp) - self.pieces * self.penalty.per_cyberware - self.borgware * self.penalty.per_borgware
    }

    /// `(used, capacity)` of a container this loadout has.
    pub fn slots(&self, container: CprContainer) -> Option<(u8, u8)> {
        self.slots.iter().find(|(c, _, _)| *c == container).map(|(_, used, cap)| (*used, *cap))
    }

    /// The `cpr.cyberware.` rows.
    pub fn rows(&self, base_emp: u8) -> Vec<ComputationExplanation> {
        let mut rows = vec![
            ComputationExplanation {
                id: "cpr.cyberware.humanity_loss".to_string(),
                value: self.humanity_loss,
                detail: format!(
                    "Humanity Loss {}: the preset Humanity Loss of {} installed pieces (p. 112: at Character Generation Humanity Loss is preset)",
                    self.humanity_loss, self.pieces_installed
                ),
            },
            ComputationExplanation {
                id: "cpr.cyberware.humanity_cap".to_string(),
                value: self.humanity_cap(base_emp),
                detail: format!(
                    "Maximum Humanity {}: 10 x EMP {base_emp} - {} pieces of cyberware x {} - {} pieces of borgware x {}; 0 Humanity Loss cyberware excluded (p. 231)",
                    self.humanity_cap(base_emp),
                    self.pieces,
                    self.penalty.per_cyberware,
                    self.borgware,
                    self.penalty.per_borgware
                ),
            },
            ComputationExplanation {
                id: "cpr.cyberware.body".to_string(),
                value: i16::from(self.body),
                detail: format!(
                    "BODY {}: BODY {} after Grafted Muscle and Bone Lace (+2, not to 11 or higher, p. 364) and Linear Frames (Sigma 12, Beta 14, p. 368)",
                    self.body, self.base_body
                ),
            },
            ComputationExplanation {
                id: "cpr.cyberware.sp_body".to_string(),
                value: i16::from(self.sp_body),
                detail: format!("Body SP {} from cyberware: the highest of Skin Weave 7 and Subdermal Armor 11 installed (p. 365)", self.sp_body),
            },
            ComputationExplanation {
                id: "cpr.cyberware.sp_head".to_string(),
                value: i16::from(self.sp_head),
                detail: format!("Head SP {} from cyberware: the highest of Skin Weave 7 and Subdermal Armor 11 installed (p. 365)", self.sp_head),
            },
            ComputationExplanation {
                id: "cpr.cyberware.initiative_bonus".to_string(),
                value: self.initiative_bonus,
                detail: format!("Initiative +{}: Kerenzikov, always on (p. 360)", self.initiative_bonus),
            },
        ];
        for (container, used, cap) in &self.slots {
            let id = container.id();
            let label = container.label();
            rows.push(ComputationExplanation {
                id: format!("cpr.cyberware.slots.{id}.used"),
                value: i16::from(*used),
                detail: format!("{label}: {used} of {cap} option slots used (p. 112)"),
            });
            rows.push(ComputationExplanation {
                id: format!("cpr.cyberware.slots.{id}.capacity"),
                value: i16::from(*cap),
                detail: format!("{label}: {cap} option slots (p. 112-117, p. 359-368)"),
            });
        }
        rows
    }
}

/// Humanity after cyberware: 10 × EMP (p. 081) less the Humanity Loss.
pub fn humanity_after_loss(base_emp: u8, humanity_loss: i16) -> i16 {
    10 * i16::from(base_emp) - humanity_loss
}

/// D-6: EMP is the tens place of Humanity (p. 081: 44 Humanity is EMP 4, 39 is EMP 3), never
/// below 0 (A-4). The p118 "(-2 EMP)" strings are not read.
pub fn emp_after_humanity_loss(base_emp: u8, humanity_loss: i16) -> i16 {
    humanity_after_loss(base_emp, humanity_loss).div_euclid(10).max(0)
}

/// D-5: therapy regains Humanity up to the cap, never past it (p. 231).
pub fn humanity_after_therapy(current: i16, regained: i16, cap: i16) -> i16 {
    if current >= cap {
        current
    } else {
        (current + regained).min(cap)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn corpus_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/cyberpunk-red/corpus")
    }

    fn catalog() -> CprCyberwareCatalog {
        CprCyberwareCatalog::load(&corpus_dir()).expect("the CPR cyberware corpus loads")
    }

    fn install(name: &str) -> CprCyberwareInstall {
        CprCyberwareInstall { name: name.to_string(), mount: None }
    }

    fn mounted(name: &str, mount: CprMount) -> CprCyberwareInstall {
        CprCyberwareInstall { name: name.to_string(), mount: Some(mount) }
    }

    fn row(rows: &[ComputationExplanation], id: &str) -> i16 {
        rows.iter().find(|r| r.id == id).unwrap_or_else(|| panic!("no row {id} in {rows:?}")).value
    }

    #[test]
    fn the_catalog_holds_96_items_and_the_p231_max_humanity_penalty() {
        let catalog = catalog();
        assert_eq!(catalog.len(), 96);
        assert_eq!(catalog.max_humanity_penalty(), MaxHumanityPenalty { per_cyberware: 2, per_borgware: 4 });
    }

    // ---- the 10 Role packages (p118) sum to CUI F-66 -----------------------------------

    #[derive(serde::Deserialize)]
    struct Fixture {
        packages: Vec<FixturePackage>,
    }
    #[derive(serde::Deserialize)]
    struct FixturePackage {
        role: String,
        printed_humanity_loss: i16,
        groups: Vec<Vec<FixtureItem>>,
    }
    #[derive(serde::Deserialize, Clone)]
    struct FixtureItem {
        name: String,
        inventory_hl: i16,
        mount: Option<String>,
    }

    fn fixture() -> Fixture {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/cpr/role_cyberware_p118.json");
        serde_json::from_str(&std::fs::read_to_string(&path).expect("fixture reads")).expect("fixture parses")
    }

    fn fixture_mount(text: &str) -> CprMount {
        let (kind, index) = text.split_once(':').expect("mount is kind:index");
        let index: u8 = index.parse().expect("mount index");
        match kind {
            "eye" => CprMount::Eye(index),
            "arm" => CprMount::Arm(index),
            "leg" => CprMount::Leg(index),
            "meat_arm" => CprMount::MeatArm(index),
            "meat_leg" => CprMount::MeatLeg(index),
            other => panic!("unknown mount kind {other}"),
        }
    }

    /// Every way to pick one option from each printed line.
    fn combinations(groups: &[Vec<FixtureItem>]) -> Vec<Vec<FixtureItem>> {
        groups.iter().fold(vec![Vec::new()], |acc, group| {
            acc.iter()
                .flat_map(|picked| {
                    group.iter().map(move |option| {
                        let mut next = picked.clone();
                        next.push(option.clone());
                        next
                    })
                })
                .collect()
        })
    }

    #[test]
    fn the_ten_role_packages_sum_to_their_printed_humanity_loss() {
        let catalog = catalog();
        let fixture = fixture();
        let roles: Vec<&str> = fixture.packages.iter().map(|p| p.role.as_str()).collect();
        assert_eq!(
            roles,
            ["Rockerboy", "Solo", "Netrunner", "Tech", "Medtech", "Media", "Lawman", "Exec", "Fixer", "Nomad"]
        );
        let printed: Vec<i16> = fixture.packages.iter().map(|p| p.printed_humanity_loss).collect();
        assert_eq!(printed, [9, 14, 14, 12, 12, 10, 10, 12, 16, 14], "CUI F-66");

        let mut checked = 0;
        for package in &fixture.packages {
            for picks in combinations(&package.groups) {
                let inventory_sum: i16 = picks.iter().map(|p| p.inventory_hl).sum();
                assert_eq!(inventory_sum, package.printed_humanity_loss, "{} inventory arithmetic", package.role);
                let installs: Vec<CprCyberwareInstall> = picks
                    .iter()
                    .map(|p| CprCyberwareInstall { name: p.name.clone(), mount: p.mount.as_deref().map(fixture_mount) })
                    .collect();
                let loadout = catalog
                    .check(5, &installs)
                    .unwrap_or_else(|e| panic!("{} package {:?} refused: {e}", package.role, picks.iter().map(|p| &p.name).collect::<Vec<_>>()));
                assert_eq!(loadout.humanity_loss(), package.printed_humanity_loss, "{} engine sum", package.role);
                checked += 1;
            }
        }
        // 10 packages; Solo, Medtech, Media, Fixer, Nomad have one 2-way choice, Exec two: 1+2+1+1+2+2+1+4+2+2.
        assert_eq!(checked, 18);
    }

    // ---- D-6: EMP is computed from Humanity, never read from the "(-2 EMP)" string --------

    #[test]
    fn media_and_lawman_packs_from_emp_5_leave_emp_4_not_3() {
        let catalog = catalog();
        let media = [install("Amplified Hearing"), install("Cyberaudio Suite"), install("Light Tattoo")];
        let lawman = [install("Hidden Holster"), install("Subdermal Pocket")];
        for pack in [&media[..], &lawman[..]] {
            let loadout = catalog.check(5, pack).expect("pack installs");
            assert_eq!(loadout.humanity_loss(), 10);
            assert_eq!(humanity_after_loss(5, loadout.humanity_loss()), 40);
            assert_eq!(emp_after_humanity_loss(5, loadout.humanity_loss()), 4);
        }
        // p081's own example: 44 Humanity is EMP 4, 39 is EMP 3; negative Humanity is EMP 0 (A-4).
        assert_eq!(emp_after_humanity_loss(5, 6), 4);
        assert_eq!(emp_after_humanity_loss(5, 11), 3);
        assert_eq!(emp_after_humanity_loss(1, 14), 0);
    }

    // ---- D-11: one refusal per container ----------------------------------------------

    fn fill(name: &str, n: usize) -> Vec<CprCyberwareInstall> {
        (0..n).map(|_| install(name)).collect()
    }

    fn assert_slots_full(result: Result<CprCyberwareLoadout, CprCyberwareRefusal>, container: CprContainer, capacity: u8) {
        match result {
            Err(CprCyberwareRefusal::SlotsFull { container: c, capacity: cap, .. }) => {
                assert_eq!((c, cap), (container, capacity));
            }
            other => panic!("expected {container:?} full at {capacity}, got {other:?}"),
        }
    }

    #[test]
    fn fashionware_internal_and_external_each_refuse_the_eighth_slot() {
        let catalog = catalog();
        for (item, container) in [
            ("Light Tattoo", CprContainer::Fashionware),
            ("Contraceptive Implant", CprContainer::Internal),
            ("Subdermal Pocket", CprContainer::External),
        ] {
            assert!(catalog.check(5, &fill(item, 7)).is_ok(), "7 {item} fit");
            assert_slots_full(catalog.check(5, &fill(item, 8)), container, 7);
        }
    }

    #[test]
    fn the_neural_link_refuses_the_sixth_option_slot_and_chipware_takes_none() {
        let catalog = catalog();
        let mut five = vec![install("Neural Link")];
        five.extend(fill("Interface Plugs", 4));
        five.push(install("Braindance Recorder"));
        // Chipware Socket and its chip use no Neural Link slot (p360).
        five.push(install("Chipware Socket"));
        five.push(install("Memory Chip"));
        let loadout = catalog.check(5, &five).expect("5 of 5 Neural Link slots");
        assert_eq!(loadout.slots(CprContainer::NeuralLink), Some((5, 5)));
        let mut six = five.clone();
        six.push(install("Interface Plugs"));
        assert_slots_full(catalog.check(5, &six), CprContainer::NeuralLink, 5);
    }

    #[test]
    fn each_cybereye_refuses_its_fourth_slot() {
        let catalog = catalog();
        let eye0 = |name: &str| mounted(name, CprMount::Eye(0));
        let base = vec![install("Cybereye"), eye0("Dartgun")];
        assert!(catalog.check(5, &base).is_ok(), "Dartgun uses 3 of 3");
        let over = [base.clone(), vec![eye0("Chyron")]].concat();
        assert_slots_full(catalog.check(5, &over), CprContainer::Cybereye(0), 3);
        // The second eye has its own 3 slots.
        let second = [base, vec![install("Cybereye"), mounted("Chyron", CprMount::Eye(1))]].concat();
        let loadout = catalog.check(5, &second).expect("second eye has room");
        assert_eq!(loadout.slots(CprContainer::Cybereye(1)), Some((1, 3)));
    }

    #[test]
    fn the_cyberaudio_suite_refuses_its_fourth_slot_unless_a_sensor_array_adds_five() {
        let catalog = catalog();
        let mut three = vec![install("Cyberaudio Suite")];
        three.extend(["Amplified Hearing", "Bug Detector", "Level Damper"].map(install));
        assert!(catalog.check(5, &three).is_ok());
        let four = [three.clone(), vec![install("Radio Communicator")]].concat();
        assert_slots_full(catalog.check(5, &four), CprContainer::CyberaudioSuite, 3);
        let with_array = [four, vec![install("Sensor Array")], fill("Audio Recorder", 4)].concat();
        let loadout = catalog.check(5, &with_array).expect("3 + 5 = 8 slots");
        assert_eq!(loadout.slots(CprContainer::CyberaudioSuite), Some((8, 8)));
        let nine = [with_array, vec![install("Homing Tracer")]].concat();
        assert_slots_full(catalog.check(5, &nine), CprContainer::CyberaudioSuite, 8);
    }

    #[test]
    fn a_cyberarm_refuses_its_fifth_slot_and_a_cyberleg_its_fourth() {
        let catalog = catalog();
        let arm = |name: &str| mounted(name, CprMount::Arm(0));
        let arm_ok = vec![install("Cyberarm"), arm("Popup Shield"), arm("Grapple Hand")];
        assert!(catalog.check(5, &arm_ok).is_ok(), "3 + 1 = 4 of 4");
        let arm_over = [arm_ok, vec![arm("Tool Hand")]].concat();
        assert_slots_full(catalog.check(5, &arm_over), CprContainer::Cyberarm(0), 4);

        let leg = |name: &str, i: u8| mounted(name, CprMount::Leg(i));
        let legs_ok = vec![
            install("Cyberleg"),
            install("Cyberleg"),
            leg("Jump Booster", 0),
            leg("Talon Foot", 0),
            leg("Jump Booster", 1),
        ];
        assert!(catalog.check(5, &legs_ok).is_ok(), "2 + 1 = 3 of 3");
        let leg_over = [legs_ok, vec![leg("Hardened Shielding", 0)]].concat();
        assert_slots_full(catalog.check(5, &leg_over), CprContainer::Cyberleg(0), 3);
    }

    #[test]
    fn options_need_their_foundation_pairs_and_mounts() {
        let catalog = catalog();
        let refused = |installs: &[CprCyberwareInstall]| match catalog.check(5, installs) {
            Err(CprCyberwareRefusal::Rule { .. }) => {}
            other => panic!("expected a rule refusal for {installs:?}, got {other:?}"),
        };
        refused(&[install("Kerenzikov")]); // no Neural Link
        refused(&[install("Neural Link"), install("Memory Chip")]); // no Chipware Socket
        refused(&[install("Amplified Hearing")]); // no Cyberaudio Suite
        refused(&[install("Cybereye"), install("Chyron")]); // no mount
        refused(&[install("Cybereye"), mounted("Image Enhance", CprMount::Eye(0))]); // paired, 1 eye
        refused(&[mounted("Popup Shield", CprMount::MeatArm(0))]); // not a meat-arm option
        refused(&[mounted("Wolvers", CprMount::MeatArm(0)), mounted("Rippers", CprMount::MeatArm(0))]); // one per meat arm
        refused(&[install("Neural Link"), install("Kerenzikov"), install("Sandevistan")]); // one Speedware
        refused(&[install("Cyberaudio Suite"), install("Cyberaudio Suite")]); // only 1
        refused(&[install("Cybereye"), install("Cybereye"), install("Cybereye")]); // 2 eyes without MultiOptic
        refused(&[install("Unknown Implant")]);
        // A-5: MultiOptic Mount adds eyes, each with its own 3 slots.
        let extra_eye = [
            install("MultiOptic Mount"),
            install("Cybereye"),
            install("Cybereye"),
            install("Cybereye"),
            mounted("Dartgun", CprMount::Eye(2)),
        ];
        let loadout = catalog.check(5, &extra_eye).expect("MultiOptic Mount allows a third eye");
        assert_eq!(loadout.slots(CprContainer::Cybereye(2)), Some((3, 3)));
        // A-5: Artificial Shoulder Mount adds two Cyberarms, each with 4 slots.
        let arms = [
            install("Artificial Shoulder Mount"),
            install("Cyberarm"),
            install("Cyberarm"),
            install("Cyberarm"),
            mounted("Popup Shield", CprMount::Arm(2)),
        ];
        assert_eq!(catalog.check(5, &arms).expect("third arm").slots(CprContainer::Cyberarm(2)), Some((3, 4)));
    }

    // ---- effects that land on the sheet ------------------------------------------------

    #[test]
    fn skin_weave_and_subdermal_armor_give_body_and_head_sp_highest_wins() {
        let catalog = catalog();
        let rows = |installs: &[CprCyberwareInstall]| catalog.check(5, installs).expect("installs").rows(5);
        let weave = rows(&[install("Skin Weave")]);
        assert_eq!((row(&weave, "cpr.cyberware.sp_body"), row(&weave, "cpr.cyberware.sp_head")), (7, 7));
        let armor = rows(&[install("Subdermal Armor")]);
        assert_eq!((row(&armor, "cpr.cyberware.sp_body"), row(&armor, "cpr.cyberware.sp_head")), (11, 11));
        let both = rows(&[install("Skin Weave"), install("Subdermal Armor")]);
        assert_eq!(row(&both, "cpr.cyberware.sp_body"), 11, "highest source, no stacking");
        let none = rows(&[install("Biomonitor")]);
        assert_eq!(row(&none, "cpr.cyberware.sp_body"), 0);
    }

    #[test]
    fn grafted_muscle_adds_two_body_capped_at_ten() {
        let catalog = catalog();
        let body = |base: u8, installs: &[CprCyberwareInstall]| catalog.check(base, installs).expect("installs").body();
        let grafted = [install("Grafted Muscle and Bone Lace")];
        assert_eq!(body(6, &grafted), 8);
        assert_eq!(body(9, &grafted), 10, "cannot increase BODY to 11 or higher");
        assert_eq!(body(10, &grafted), 10);
        assert_eq!(body(6, &[]), 6);
    }

    #[test]
    fn linear_frames_set_body_12_and_14_with_their_prerequisites() {
        let catalog = catalog();
        let grafted = || install("Grafted Muscle and Bone Lace");
        let sigma = || install("Implanted Linear Frame Sigma");
        let beta = || install("Implanted Linear Frame Beta");
        assert_eq!(catalog.check(6, &[grafted(), sigma()]).expect("Sigma").body(), 12);
        assert_eq!(catalog.check(8, &[grafted(), grafted(), beta()]).expect("Beta").body(), 14);
        let refused = |base: u8, installs: &[CprCyberwareInstall]| {
            assert!(matches!(catalog.check(base, installs), Err(CprCyberwareRefusal::Rule { .. })), "{base} {installs:?}");
        };
        refused(5, &[grafted(), sigma()]); // needs BODY 6
        refused(6, &[sigma()]); // needs Grafted Muscle and Bone Lace
        refused(7, &[grafted(), grafted(), beta()]); // needs BODY 8
        refused(8, &[grafted(), beta()]); // needs two Grafted Muscle and Bone Lace
    }

    #[test]
    fn kerenzikov_adds_two_initiative_and_sandevistan_none() {
        let catalog = catalog();
        let bonus = |name: &str| {
            let rows = catalog.check(5, &[install("Neural Link"), install(name)]).expect("installs").rows(5);
            row(&rows, "cpr.cyberware.initiative_bonus")
        };
        assert_eq!(bonus("Kerenzikov"), 2);
        assert_eq!(bonus("Sandevistan"), 0, "an Action, printed, not always on");
    }

    // ---- D-5: the therapy cap on maximum Humanity -------------------------------------

    #[test]
    fn the_therapy_cap_is_ten_times_emp_less_two_per_piece_and_four_per_borgware() {
        let catalog = catalog();
        // 2 pieces + 1 borgware from EMP 6 → 60 − 2 − 2 − 4 = 52.
        let fixture = [install("Neural Link"), install("Kerenzikov"), install("MultiOptic Mount")];
        let loadout = catalog.check(5, &fixture).expect("installs");
        assert_eq!(loadout.humanity_cap(6), 52);
        assert_eq!(row(&loadout.rows(6), "cpr.cyberware.humanity_cap"), 52);
        // 0-HL cyberware does not lower the cap.
        let zero = [install("Biomonitor"), install("Light Tattoo")];
        assert_eq!(catalog.check(5, &zero).expect("installs").humanity_cap(6), 60);
        // The free Standard Hand that comes with a Cyberarm is not a piece; the arm is.
        assert_eq!(catalog.check(5, &[install("Cyberarm")]).expect("installs").humanity_cap(6), 58);
        // Therapy regains Humanity up to the cap, never past it.
        assert_eq!(humanity_after_therapy(30, 7, 52), 37);
        assert_eq!(humanity_after_therapy(48, 12, 52), 52);
        assert_eq!(humanity_after_therapy(52, 5, 52), 52);
    }

    // ---- creation: the printed names of the kits, and where each option goes ------------

    #[test]
    fn printed_kit_names_resolve_to_their_records() {
        let catalog = catalog();
        // p117 prints "Tech Hair" and "Sandevistan Speedware"; the p360 listing names them
        // Techhair and Sandevistan (subcategory "Neuralware (Speedware)").
        assert_eq!(catalog.record_name("Tech Hair"), Some("Techhair"));
        assert_eq!(catalog.record_name("Sandevistan Speedware"), Some("Sandevistan"));
        assert_eq!(catalog.record_name("Neural Link"), Some("Neural Link"));
        assert_eq!(catalog.record_name("Sandevistan Cyberarm"), None, "only the record's own subcategory word");
        assert_eq!(catalog.record_name("Unknown Implant"), None);
    }

    fn placed(catalog: &CprCyberwareCatalog, names: &[&str]) -> Vec<CprCyberwareInstall> {
        let names: Vec<String> = names.iter().map(|n| (*n).to_owned()).collect();
        catalog.place(&names).unwrap_or_else(|e| panic!("{names:?}: {e}"))
    }

    #[test]
    fn placement_mounts_each_option_on_its_foundation_or_alone_in_a_meat_limb() {
        let catalog = catalog();
        // The Tech's pack: MicroOptics on the Cybereye, the Tool Hand alone in a meat arm.
        let tech = placed(&catalog, &["Cybereye", "MicroOptics", "Skinwatch", "Tool Hand"]);
        assert_eq!(
            tech,
            vec![
                install("Cybereye"),
                mounted("MicroOptics", CprMount::Eye(0)),
                install("Skinwatch"),
                mounted("Tool Hand", CprMount::MeatArm(0)),
            ]
        );
        assert!(catalog.check(5, &tech).is_ok());
        // Two meat-arm pieces take one meat arm each; the Sandevistan Speedware takes no mount.
        let solo = placed(&catalog, &["Neural Link", "Sandevistan Speedware", "Wolvers", "Rippers"]);
        assert_eq!(solo[1], install("Sandevistan"));
        assert_eq!((solo[2].mount, solo[3].mount), (Some(CprMount::MeatArm(0)), Some(CprMount::MeatArm(1))));
        // A Cyberarm takes an arm option before a meat arm does.
        let arm = placed(&catalog, &["Cyberarm", "Wolvers"]);
        assert_eq!(arm[1], mounted("Wolvers", CprMount::Arm(0)));
        // A paired option goes one per eye (p. 361).
        let pair = placed(&catalog, &["Cybereye", "Cybereye", "Image Enhance", "Image Enhance"]);
        assert_eq!((pair[2].mount, pair[3].mount), (Some(CprMount::Eye(0)), Some(CprMount::Eye(1))));
        assert!(catalog.check(5, &pair).is_ok());
        // An eye with no room left: the option stays on the first eye, and the check refuses it
        // as over the slots (D-11), not as unmounted.
        let full = placed(&catalog, &["Cybereye", "Dartgun", "Chyron"]);
        assert_eq!(full[2], mounted("Chyron", CprMount::Eye(0)));
        assert_slots_full(catalog.check(5, &full), CprContainer::Cybereye(0), 3);
        // A printed name the catalog does not have is refused.
        assert!(matches!(catalog.place(&["Unknown Implant".to_owned()]), Err(CprCyberwareRefusal::Rule { .. })));
    }

    #[test]
    fn the_rows_carry_humanity_loss_body_and_slot_totals() {
        let catalog = catalog();
        let installs = [install("Cyberaudio Suite"), install("Amplified Hearing"), install("Light Tattoo")];
        let rows = catalog.check(7, &installs).expect("installs").rows(5);
        assert_eq!(row(&rows, "cpr.cyberware.humanity_loss"), 10);
        assert_eq!(row(&rows, "cpr.cyberware.body"), 7);
        assert_eq!(row(&rows, "cpr.cyberware.slots.fashionware.used"), 1);
        assert_eq!(row(&rows, "cpr.cyberware.slots.fashionware.capacity"), 7);
        assert_eq!(row(&rows, "cpr.cyberware.slots.cyberaudio_suite.used"), 1);
        assert_eq!(row(&rows, "cpr.cyberware.slots.cyberaudio_suite.capacity"), 3);
        assert!(rows.iter().all(|r| r.id.starts_with("cpr.cyberware.")), "{rows:?}");
        assert!(rows.iter().all(|r| !r.detail.contains("EMP)")), "never prints the '(-2 EMP)' fragment");
    }
}
