//! Netrunning sheet numbers: NET Actions per Turn from the Interface rank, and a
//! Cyberdeck's slots available and used. Everything else in the Netrunning chapter is
//! printed, not computed (NET-run play is not simulated); the brain-damage rule is one
//! printed line.
//!
//! Sources (Cyberpunk RED core rulebook): Interface rank to NET Actions, p. 197; deck slots
//! 5 / 7 / 9 and "Both Programs and Hardware take up the same limited slots", p. 196 and
//! p. 353; Black ICE takes 2 slots, p. 204; Hardware takes 1 slot unless noted, p. 208;
//! Bodyweight Suit "adds one Hardware only Option Slot" and hardware there "can only take
//! up 1 Option Slot", p. 350; Cyberdeck Cyberarm option "Adds 1 extra Program/Hardware
//! Slot", p. 364. Both extra slots apply to one deck, and a loadout over the deck's slots
//! is refused (author default A-7).

use crate::rules_core::pilot_compute::ComputationExplanation;

/// The printed brain-damage rule. The book says Zap and the brain-damage Programs and
/// Black ICE deal damage "directly to the Netrunner's brain" (p. 201); the sheet prints how
/// that damage is taken and computes nothing from it.
pub const BRAIN_DAMAGE_RULE: &str = "Brain damage (from Zap and from Programs and Black ICE that deal damage directly to the brain) is subtracted from your Hit Points; armor does not reduce it (Cyberpunk RED core rulebook p. 201).";

/// Why a netrunning input is refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CprNetrunningError {
    /// Interface is a Role Ability rank, 1 to 10.
    InterfaceRankOutOfRange(u8),
    /// Hardware takes 1 or 2 slots (p. 208).
    HardwareSlotsOutOfRange { name: String, slots: u8 },
    /// The loadout needs more slots than the deck has.
    OverCapacity { needed: u8, available: u8, detail: String },
}

impl std::fmt::Display for CprNetrunningError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InterfaceRankOutOfRange(rank) => {
                write!(f, "Interface rank {rank} is outside 1-10")
            }
            Self::HardwareSlotsOutOfRange { name, slots } => {
                write!(f, "Hardware {name} takes {slots} slots; Hardware takes 1 or 2 slots")
            }
            Self::OverCapacity { detail, .. } => f.write_str(detail),
        }
    }
}

impl std::error::Error for CprNetrunningError {}

/// NET Actions per Turn for an Interface rank (p. 197): 1-3 → 2, 4-6 → 3, 7-9 → 4, 10 → 5.
pub fn net_actions_for_interface_rank(rank: u8) -> Result<u8, CprNetrunningError> {
    match rank {
        1..=3 => Ok(2),
        4..=6 => Ok(3),
        7..=9 => Ok(4),
        10 => Ok(5),
        _ => Err(CprNetrunningError::InterfaceRankOutOfRange(rank)),
    }
}

/// The `cpr.netrunning.net_actions` row for an Interface rank.
pub fn net_actions_row(rank: u8) -> Result<ComputationExplanation, CprNetrunningError> {
    let actions = net_actions_for_interface_rank(rank)?;
    Ok(ComputationExplanation {
        id: "cpr.netrunning.net_actions".to_string(),
        value: i16::from(actions),
        detail: format!(
            "Interface rank {rank} → {actions} NET Actions per Turn (ranks 1-3: 2, 4-6: 3, 7-9: 4, 10: 5; Cyberpunk RED core rulebook p. 197)"
        ),
    })
}

/// The three Cyberdeck qualities (p. 196).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CprDeckQuality {
    Poor,
    Standard,
    Excellent,
}

impl CprDeckQuality {
    pub const ALL: [CprDeckQuality; 3] = [Self::Poor, Self::Standard, Self::Excellent];

    /// The deck's own slots: 5, 7 or 9.
    pub fn base_slots(self) -> u8 {
        match self {
            Self::Poor => 5,
            Self::Standard => 7,
            Self::Excellent => 9,
        }
    }

    /// The book's name for the deck, as the corpus records it.
    pub fn name(self) -> &'static str {
        match self {
            Self::Poor => "Poor Quality Cyberdeck",
            Self::Standard => "Cyberdeck (Standard Quality)",
            Self::Excellent => "Excellent Quality Cyberdeck",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|quality| quality.name() == name)
    }
}

/// The extra slots a deck can gain (A-7: both may apply to one deck).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CprDeckExtras {
    /// Wearing a Bodyweight Suit: +1 slot that only 1-slot Hardware may use.
    pub bodyweight_suit: bool,
    /// The Cyberdeck Cyberarm option: +1 slot for a Program or Hardware.
    pub cyberarm_deck: bool,
}

/// What a loaded item is, for slot use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CprDeckItemKind {
    /// A Booster, Defender or Attacker: 1 slot.
    Program,
    /// Black ICE: 2 slots.
    BlackIce,
    /// Hardware: its printed slots, 1 or 2.
    Hardware { slots: u8 },
}

/// One Program or piece of Hardware loaded on the deck.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CprDeckItem {
    pub name: String,
    pub kind: CprDeckItemKind,
}

impl CprDeckItem {
    pub fn program(name: &str) -> Self {
        Self { name: name.to_string(), kind: CprDeckItemKind::Program }
    }

    pub fn black_ice(name: &str) -> Self {
        Self { name: name.to_string(), kind: CprDeckItemKind::BlackIce }
    }

    pub fn hardware(name: &str, slots: u8) -> Self {
        Self { name: name.to_string(), kind: CprDeckItemKind::Hardware { slots } }
    }

    /// Slots this item takes: Program 1, Black ICE 2, Hardware its printed 1 or 2.
    pub fn slots(&self) -> u8 {
        match self.kind {
            CprDeckItemKind::Program => 1,
            CprDeckItemKind::BlackIce => 2,
            CprDeckItemKind::Hardware { slots } => slots,
        }
    }
}

/// A deck's computed slot totals and its two sheet rows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CprDeckLoad {
    pub slots_available: u8,
    pub slots_used: u8,
    /// The 1-slot Hardware placed in the Bodyweight Suit's slot, if any.
    pub bodyweight_suit_hardware: Option<String>,
    pub rows: Vec<ComputationExplanation>,
}

/// Slots available and used for a deck and its loadout; a loadout over the deck's slots is
/// refused (A-7). The Bodyweight Suit slot holds one 1-slot Hardware only.
pub fn compute_deck(
    quality: CprDeckQuality,
    extras: CprDeckExtras,
    items: &[CprDeckItem],
) -> Result<CprDeckLoad, CprNetrunningError> {
    for item in items {
        if let CprDeckItemKind::Hardware { slots } = item.kind
            && !(1..=2).contains(&slots)
        {
            return Err(CprNetrunningError::HardwareSlotsOutOfRange { name: item.name.clone(), slots });
        }
    }

    let base = quality.base_slots();
    let cyberarm = u8::from(extras.cyberarm_deck);
    let suit = u8::from(extras.bodyweight_suit);
    // Slots any Program or Hardware may use, and the suit's Hardware-only slot.
    let general_capacity = base + cyberarm;
    let slots_available = general_capacity + suit;

    let needed: u32 = items.iter().map(|item| u32::from(item.slots())).sum();
    let bodyweight_suit_hardware = if extras.bodyweight_suit {
        items
            .iter()
            .find(|item| item.kind == CprDeckItemKind::Hardware { slots: 1 })
            .map(|item| item.name.clone())
    } else {
        None
    };
    let general_needed = needed - u32::from(bodyweight_suit_hardware.is_some());
    let needed_u8 = u8::try_from(needed).unwrap_or(u8::MAX);

    if general_needed > u32::from(general_capacity) {
        let detail = if extras.bodyweight_suit && bodyweight_suit_hardware.is_none() {
            format!(
                "The loadout needs {needed} slots; the {} has {general_capacity} slots for Programs and Hardware plus 1 Bodyweight Suit slot that only 1-slot Hardware can use, and none is loaded",
                quality.name()
            )
        } else {
            format!(
                "The loadout needs {needed} slots; the {} has {slots_available} (deck {base}{}{})",
                quality.name(),
                if extras.cyberarm_deck { " + 1 Cyberarm deck" } else { "" },
                if extras.bodyweight_suit { " + 1 Bodyweight Suit, Hardware only" } else { "" },
            )
        };
        return Err(CprNetrunningError::OverCapacity { needed: needed_u8, available: slots_available, detail });
    }

    let rows = vec![
        ComputationExplanation {
            id: "cpr.netrunning.deck_slots_available".to_string(),
            value: i16::from(slots_available),
            detail: format!(
                "{} {base} slots{}{} = {slots_available} (Cyberpunk RED core rulebook p. 196, p. 350, p. 364)",
                quality.name(),
                if extras.cyberarm_deck { " + 1 Cyberarm deck slot" } else { "" },
                if extras.bodyweight_suit { " + 1 Bodyweight Suit Hardware-only slot" } else { "" },
            ),
        },
        ComputationExplanation {
            id: "cpr.netrunning.deck_slots_used".to_string(),
            value: i16::from(needed_u8),
            detail: format!(
                "{} loaded item(s) use {needed} slots (Program 1, Black ICE 2, Hardware 1 or 2; Cyberpunk RED core rulebook p. 204, p. 208)",
                items.len()
            ),
        },
    ];

    Ok(CprDeckLoad { slots_available, slots_used: needed_u8, bodyweight_suit_hardware, rows })
}

/// A Netrunner's netrunning block on the sheet: the computed rows and the printed lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CprNetrunnerSheet {
    pub rows: Vec<ComputationExplanation>,
    pub printed: Vec<String>,
    pub deck: Option<CprDeckLoad>,
}

/// The netrunning block for an Interface rank and, when the character carries one, a deck.
pub fn netrunner_sheet(
    interface_rank: u8,
    deck: Option<(CprDeckQuality, CprDeckExtras, &[CprDeckItem])>,
) -> Result<CprNetrunnerSheet, CprNetrunningError> {
    let mut rows = vec![net_actions_row(interface_rank)?];
    let deck = match deck {
        Some((quality, extras, items)) => {
            let load = compute_deck(quality, extras, items)?;
            rows.extend(load.rows.iter().cloned());
            Some(load)
        }
        None => None,
    };
    Ok(CprNetrunnerSheet { rows, printed: vec![BRAIN_DAMAGE_RULE.to_string()], deck })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Transcribed from the page image of p. 197 (pages/p198) and the inventory
    // `artifacts/cycle_0/inventory/netrunning.md` §1, never from the corpus.
    const NET_ACTIONS_BY_RANK_P197: [u8; 10] = [2, 2, 2, 3, 3, 3, 4, 4, 4, 5];

    /// The p. 157 sample Netrunner's deck (pages/p158): "Cyberdeck (7 slots: Sword, Sword,
    /// Killer, Worm, Worm, Armor)". Killer is Black ICE (inventory §2, p. 207).
    fn sample_deck_p157() -> Vec<CprDeckItem> {
        vec![
            CprDeckItem::program("Sword"),
            CprDeckItem::program("Sword"),
            CprDeckItem::black_ice("Killer"),
            CprDeckItem::program("Worm"),
            CprDeckItem::program("Worm"),
            CprDeckItem::program("Armor"),
        ]
    }

    fn row<'a>(rows: &'a [ComputationExplanation], id: &str) -> &'a ComputationExplanation {
        rows.iter().find(|r| r.id == id).unwrap_or_else(|| panic!("no row {id} in {rows:?}"))
    }

    #[test]
    fn interface_ranks_one_to_ten_give_the_p197_net_actions() {
        let computed: Vec<u8> =
            (1..=10).map(|rank| net_actions_for_interface_rank(rank).unwrap()).collect();
        assert_eq!(computed, NET_ACTIONS_BY_RANK_P197);
        for rank in 1..=10u8 {
            let r = net_actions_row(rank).unwrap();
            assert_eq!(r.id, "cpr.netrunning.net_actions");
            assert_eq!(r.value, i16::from(NET_ACTIONS_BY_RANK_P197[usize::from(rank - 1)]));
        }
    }

    #[test]
    fn an_interface_rank_outside_one_to_ten_is_refused() {
        assert_eq!(
            net_actions_for_interface_rank(0),
            Err(CprNetrunningError::InterfaceRankOutOfRange(0))
        );
        assert_eq!(
            net_actions_for_interface_rank(11),
            Err(CprNetrunningError::InterfaceRankOutOfRange(11))
        );
    }

    #[test]
    fn deck_qualities_have_five_seven_nine_slots() {
        let slots: Vec<u8> = CprDeckQuality::ALL.into_iter().map(CprDeckQuality::base_slots).collect();
        assert_eq!(slots, [5, 7, 9]);
        assert_eq!(CprDeckQuality::from_name("Cyberdeck (Standard Quality)"), Some(CprDeckQuality::Standard));
        assert_eq!(CprDeckQuality::from_name("Cyberdeck"), None);
    }

    #[test]
    fn item_slot_use_is_program_one_black_ice_two_hardware_printed() {
        assert_eq!(CprDeckItem::program("Worm").slots(), 1);
        assert_eq!(CprDeckItem::black_ice("Killer").slots(), 2);
        assert_eq!(CprDeckItem::hardware("Range Upgrade", 1).slots(), 1);
        assert_eq!(CprDeckItem::hardware("KRASH Barrier", 2).slots(), 2);
    }

    #[test]
    fn the_p157_sample_deck_uses_seven_of_seven_slots() {
        let load =
            compute_deck(CprDeckQuality::Standard, CprDeckExtras::default(), &sample_deck_p157()).unwrap();
        assert_eq!((load.slots_used, load.slots_available), (7, 7));
        assert_eq!(row(&load.rows, "cpr.netrunning.deck_slots_used").value, 7);
        assert_eq!(row(&load.rows, "cpr.netrunning.deck_slots_available").value, 7);
    }

    #[test]
    fn an_eighth_slot_on_the_sample_deck_is_refused() {
        let mut items = sample_deck_p157();
        items.push(CprDeckItem::program("Eraser"));
        let err = compute_deck(CprDeckQuality::Standard, CprDeckExtras::default(), &items).unwrap_err();
        match err {
            CprNetrunningError::OverCapacity { needed, available, .. } => {
                assert_eq!((needed, available), (8, 7));
            }
            other => panic!("expected OverCapacity, got {other:?}"),
        }
    }

    #[test]
    fn the_cyberarm_deck_adds_one_slot_for_a_program() {
        let mut items = sample_deck_p157();
        items.push(CprDeckItem::program("Eraser"));
        let extras = CprDeckExtras { cyberarm_deck: true, ..CprDeckExtras::default() };
        let load = compute_deck(CprDeckQuality::Standard, extras, &items).unwrap();
        assert_eq!((load.slots_used, load.slots_available), (8, 8));
    }

    #[test]
    fn the_bodyweight_suit_slot_takes_one_slot_hardware_only() {
        let suit = CprDeckExtras { bodyweight_suit: true, ..CprDeckExtras::default() };

        let mut with_hardware = sample_deck_p157();
        with_hardware.push(CprDeckItem::hardware("Range Upgrade", 1));
        let load = compute_deck(CprDeckQuality::Standard, suit, &with_hardware).unwrap();
        assert_eq!((load.slots_used, load.slots_available), (8, 8));
        assert_eq!(load.bodyweight_suit_hardware.as_deref(), Some("Range Upgrade"));

        // A Program cannot use the suit's slot.
        let mut with_program = sample_deck_p157();
        with_program.push(CprDeckItem::program("Eraser"));
        assert!(matches!(
            compute_deck(CprDeckQuality::Standard, suit, &with_program),
            Err(CprNetrunningError::OverCapacity { .. })
        ));

        // 2-slot Hardware cannot use it either ("can only take up 1 Option Slot", p. 350).
        let mut with_two_slot = sample_deck_p157();
        with_two_slot.push(CprDeckItem::hardware("KRASH Barrier", 2));
        assert!(matches!(
            compute_deck(CprDeckQuality::Standard, suit, &with_two_slot),
            Err(CprNetrunningError::OverCapacity { .. })
        ));
    }

    #[test]
    fn both_extra_slots_apply_to_one_deck() {
        let both = CprDeckExtras { bodyweight_suit: true, cyberarm_deck: true };
        let mut items = sample_deck_p157();
        items.push(CprDeckItem::program("Eraser"));
        items.push(CprDeckItem::hardware("Insulated Wiring", 1));
        let load = compute_deck(CprDeckQuality::Standard, both, &items).unwrap();
        assert_eq!((load.slots_used, load.slots_available), (9, 9));

        items.push(CprDeckItem::program("See Ya"));
        assert!(compute_deck(CprDeckQuality::Standard, both, &items).is_err());
    }

    #[test]
    fn hardware_outside_one_or_two_slots_is_refused() {
        let items = [CprDeckItem::hardware("Odd", 3)];
        assert_eq!(
            compute_deck(CprDeckQuality::Excellent, CprDeckExtras::default(), &items),
            Err(CprNetrunningError::HardwareSlotsOutOfRange { name: "Odd".to_string(), slots: 3 })
        );
    }

    #[test]
    fn the_netrunner_block_carries_net_actions_deck_rows_and_the_printed_brain_damage_rule() {
        let items = sample_deck_p157();
        let sheet =
            netrunner_sheet(4, Some((CprDeckQuality::Standard, CprDeckExtras::default(), &items))).unwrap();
        assert_eq!(row(&sheet.rows, "cpr.netrunning.net_actions").value, 3);
        assert_eq!(row(&sheet.rows, "cpr.netrunning.deck_slots_used").value, 7);
        assert_eq!(row(&sheet.rows, "cpr.netrunning.deck_slots_available").value, 7);
        assert!(sheet.rows.iter().all(|r| r.id.starts_with("cpr.")));
        assert_eq!(sheet.printed, [BRAIN_DAMAGE_RULE.to_string()]);
        assert!(BRAIN_DAMAGE_RULE.contains("Hit Points") && BRAIN_DAMAGE_RULE.contains("armor does not reduce it"));

        let no_deck = netrunner_sheet(10, None).unwrap();
        assert_eq!(no_deck.rows.len(), 1);
        assert_eq!(no_deck.rows[0].value, 5);
        assert_eq!(no_deck.printed, [BRAIN_DAMAGE_RULE.to_string()]);
    }
}
