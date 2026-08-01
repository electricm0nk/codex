//! SD-27 — APG's 12 field-less spell rows resolve against their base spell.
//!
//! # The defect
//!
//! `apg::spell_list::SPELL_LIST` carries 297 records. Twelve arrived at
//! `list_spell_catalog` with a key and three nulls — no school, no level, no
//! description — and rendered in SpellCatalogScreen.tsx as a row of empty
//! columns. `reach_gate`'s `BARE_RECORD_FINDINGS` pinned all twelve by name.
//!
//! Eleven are PCGen `.COPY=` delta rows (`apg_spells.lst:1037`-`1052`):
//!
//! ```text
//! Planar Binding.COPY=Planar Binding (Demons Only)
//! Beast Shape I.COPY=Beast Shape I (Animals Only)
//! ```
//!
//! A `.COPY=` row defines a record as a copy of another, carrying no `SCHOOL:`,
//! `CLASSES:` or `DESC:` token of its own. The APG ingest already resolved the
//! ones whose base is an APG spell (`Elemental Aura (Cold Only)` inherits
//! Evocation / 3 from `Elemental Aura`) and deliberately stopped at the book
//! boundary: eleven bases — `Planar Binding`, `Planar Ally`, `Beast Shape I`,
//! `Blindness/Deafness`, `Meteor Swarm` — live in CRB's `cr_spells.lst`.
//! `spell_list.rs`'s own doc called that "a deliberate scope boundary for this
//! per-book module … cross-book spell variant resolution is future work".
//! This is that work; the boundary cost a player twelve blank rows.
//!
//! The twelfth is not a delta row at all — see
//! [`wall_of_thorms_keeps_its_upstream_typo_and_resolves_against_wall_of_thorns`].
//!
//! # Why this test re-derives instead of transcribing
//!
//! The inherited school/level/description are *literals* in `apg::spell_list`,
//! because that table is a flat generated `const` and cannot call a lookup at
//! compile time. A literal copy can drift from what it copied. So every
//! assertion below reads the CRB record at runtime and requires equality —
//! if `crb::spell_list` is ever corrected, the stale APG copy fails here
//! rather than quietly disagreeing with its own base spell.

use std::collections::BTreeSet;

use codex::rules_core::rules_tables::{apg, crb};

/// Every `.COPY=` variant in `apg_spells.lst` whose base record lives in CRB,
/// paired with that base. Read off `apg_spells.lst:1037`-`1052` — the file's
/// `###Block: Other Misc & Domains` — where each row is literally
/// `<base>.COPY=<variant>`.
///
/// The `.COPY=` rows absent from this list are the four whose base is an APG
/// spell (`Elemental Aura (Cold Only)`, `Corruption Resistance (Evil)`) or
/// which were already whole; the eleven at the top were the blank rows, and
/// the four at the bottom were partly resolved — school and text but no level
/// for the `Summon Monster` trio, nothing at all for Starsoul.
const CROSS_BOOK_COPY_ROWS: &[(&str, &str)] = &[
    ("Beast Shape I (Animals Only)", "Beast Shape I"),
    ("Blindness/Deafness (Only Cause Blindness)", "Blindness/Deafness"),
    ("Meteor Swarm (Dealing Cold Damage)", "Meteor Swarm"),
    ("Planar Ally (Agathions Only)", "Planar Ally"),
    ("Planar Ally (Archon Only)", "Planar Ally"),
    ("Planar Ally (Azata Only)", "Planar Ally"),
    ("Planar Binding (Daemons Only)", "Planar Binding"),
    ("Planar Binding (Demons Only)", "Planar Binding"),
    ("Planar Binding (Devils Only)", "Planar Binding"),
    ("Planar Binding (Inevitables Only)", "Planar Binding"),
    ("Planar Binding (Proteans Only)", "Planar Binding"),
    ("Summon Monster III (Reptiles Only)", "Summon Monster III"),
    ("Summon Monster V (Summons 1d3 Shadows)", "Summon Monster V"),
    ("Summon Monster VII (Reptiles Only)", "Summon Monster VII"),
    ("Call Lightning Storm (Starsoul)", "Call Lightning Storm"),
];

/// The one cross-book variant whose `description` is deliberately NOT its
/// base's: `apg_spells.lst:1075` is
/// `Call Lightning Storm (Starsoul).MOD ... DESC:.CLEAR  DESC:This spell
/// functions like call lightning, except that each bolt deals 5d6 points of
/// Fire damage ...`. PCGen layers `.COPY=` then `.MOD`, so the base supplies
/// school and level and the `.MOD` replaces the text.
const DESCRIPTION_OVERRIDDEN_BY_ITS_OWN_MOD: &str = "Call Lightning Storm (Starsoul)";

fn apg_entry(key: &str) -> &'static apg::spell_list::SpellListEntry {
    apg::spell_list::SPELL_LIST
        .iter()
        .find(|entry| entry.key == key)
        .unwrap_or_else(|| panic!("APG record {key:?} must exist"))
}

fn crb_entry(key: &str) -> &'static crb::spell_list::SpellListEntry {
    crb::spell_list::SPELL_LIST
        .iter()
        .find(|entry| entry.key == key)
        .unwrap_or_else(|| panic!("CRB record {key:?} must exist"))
}

#[test]
fn every_cross_book_copy_row_carries_its_base_spells_fields() {
    for (variant, base) in CROSS_BOOK_COPY_ROWS {
        let copy = apg_entry(variant);
        let source = crb_entry(base);
        // `apg::spell_list` and `crb::spell_list` each declare their own
        // `Pf1SchoolId`, so the comparison is on the rendered name — which is
        // also what a player sees in the catalog column.
        assert_eq!(
            copy.school.map(|s| format!("{s:?}")),
            Some(format!("{:?}", source.school)),
            "{variant} must inherit {base}'s school"
        );
        assert_eq!(
            copy.level,
            Some(source.level),
            "{variant} must inherit {base}'s level"
        );
        if *variant == DESCRIPTION_OVERRIDDEN_BY_ITS_OWN_MOD {
            assert_ne!(
                copy.description,
                Some(source.description),
                "{variant}'s own `.MOD DESC:` overrides the base text; if these ever match, \
                 the override was lost"
            );
        } else {
            assert_eq!(
                copy.description,
                Some(source.description),
                "{variant} must inherit {base}'s description verbatim"
            );
        }
        assert!(copy.description.is_some_and(|d| !d.trim().is_empty()));
        assert!(copy.full_text, "an inherited CRB description is full SRD text");
    }
    assert_eq!(CROSS_BOOK_COPY_ROWS.len(), 15);
}

/// `apg_spells.lst:1555` reads, verbatim:
///
/// ```text
/// Wall of Thorms    DOMAINS:Blood Subdomain=5    SOURCELINK:http://paizo.com/pathfinderRPG/prd/spells/wallOfThorns.html#_wall-of-thorns
/// ```
///
/// That is a PCGen source defect, and the evidence is in the same file:
///
/// * `apg_spells.lst:1431` is `Wall of Thorns.MOD    DOMAINS:Decay Subdomain=5`
///   — the identical construct, correctly spelled, correctly suffixed;
/// * every neighbouring row in the Blood/Tactics Subdomain block (`:1550`-`:1560`)
///   is `<CRB spell>.MOD    DOMAINS:...`;
/// * the row's own `SOURCELINK` names `wallOfThorns.html#_wall-of-thorns`;
/// * `Thorms` occurs exactly once in the entire PCGen data checkout.
///
/// So the row is `Wall of Thorns.MOD` with a one-letter typo and a dropped
/// `.MOD`, which makes PCGen create a junk spell object instead of modifying
/// the real one — and the APG ingest faithfully copied the junk.
///
/// **The key is preserved.** Renaming it here would silently repair upstream
/// data and make this corpus disagree with the file it cites, in a repo whose
/// records carry a `source.line` a reader is expected to be able to open. What
/// is repaired is the *content*: the record resolves against `Wall of Thorns`
/// the same way the eleven `.COPY=` rows resolve against their bases, so a
/// player sees the real spell (with a visibly odd name) instead of a blank row.
#[test]
fn wall_of_thorms_keeps_its_upstream_typo_and_resolves_against_wall_of_thorns() {
    assert!(
        apg::spell_list::SPELL_LIST.iter().any(|e| e.key == "Wall of Thorms"),
        "the upstream key is preserved verbatim, typo and all"
    );
    assert!(
        !apg::spell_list::SPELL_LIST.iter().any(|e| e.key == "Wall of Thorns"),
        "and it is NOT silently renamed to the correct spelling"
    );

    let row = apg_entry("Wall of Thorms");
    let base = crb_entry("Wall of Thorns");
    assert_eq!(row.school.map(|s| format!("{s:?}")), Some(format!("{:?}", base.school)));
    assert_eq!(row.description, Some(base.description));
    // `DOMAINS:Blood Subdomain=5` on the row itself and CRB's own level agree.
    assert_eq!(row.level, Some(5));
    assert_eq!(row.level, Some(base.level));
}

/// The count that made this a finding, asserted directly: no APG record
/// reaches `list_spell_catalog` carrying nothing but its key.
///
/// Before this cycle: 12. This is the regression pin — a 13th field-less
/// record, or one of these twelve losing its fields again, fails here as well
/// as in `reach_gate`.
#[test]
fn no_apg_spell_record_is_left_with_no_school_no_level_and_no_description() {
    let bare: BTreeSet<&str> = apg::spell_list::SPELL_LIST
        .iter()
        .filter(|entry| {
            entry.school.is_none()
                && entry.level.is_none()
                && entry.description.is_none_or(|d| d.trim().is_empty())
        })
        .map(|entry| entry.key)
        .collect();
    assert!(bare.is_empty(), "APG records with no payload at all: {bare:?}");
    assert_eq!(apg::spell_list::SPELL_LIST.len(), 297);
}
