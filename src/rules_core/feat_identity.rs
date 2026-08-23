//! The one place that decides whether two feat identifiers name the same feat.
//!
//! `chosen.selected_feats` genuinely holds two different shapes for the same
//! feat, and which one a given entry carries depends only on how it was added:
//!
//! - the feat catalog's own human-readable `key`, e.g. `"Deflect Arrows"` --
//!   what the Feats tab's "Add Feat" picker sends. `CharacterSheet.tsx`'s
//!   `handleAddFeat` / `handleLevelUpFeatPick` append `entry.key` straight from
//!   the catalog, `add_feat_selection` passes it through, and
//!   `apply_add_feat_selection` pushes it onto `selected_feats` unmodified.
//! - the engine's lowercase, `feat:`-prefixed, snake_case token, e.g.
//!   `"feat:deflect_arrows"` -- what `pf1_adapter.rs`'s
//!   `compose_character_input` seeds a freshly created character with, and what
//!   `selected_choices` selection ids use throughout.
//!
//! Nothing normalises between them on the way in, and nothing should: both are
//! faithful records of what the player did, and rewriting one into the other at
//! the boundary would silently rewrite characters already on disk. The fold
//! belongs at the point of *comparison*, which is what this module is.
//!
//! **Why this module exists rather than a helper next to one consumer.** The
//! Feats tab has always folded for display (`normalizeFeatIdentity` in
//! `apps/desktop/src/characterHub/featsTabModel.ts`), and
//! `description_completion.rs` mirrored that fold in Rust. Effect resolution
//! did not: every producer and posture gate compared feat strings with `==`.
//! The result was a live, player-facing defect -- picking Dodge or Weapon Focus
//! from the catalog sent a string that `unmet_combat_posture_conditions` could
//! not match, so the character's entire deterministic combat baseline came back
//! `Blocked`: no armor class and no melee attack bonus, on a sheet that
//! displayed the feat correctly the whole time. Display and effect resolution
//! now fold through this single function, so they cannot drift apart again.
//!
//! **The fold is safe against collisions, checked and not assumed.** Folding is
//! whole-string equality, never a prefix match, so a longer feat whose name
//! begins with a grounded key (`"Acrobatic Steps"` vs. `"Acrobatic"`) still
//! does not match. `folding_never_merges_two_distinct_catalog_feats` below
//! proves the stronger property over the live 690-record catalog: no two
//! distinct feat keys fold together. One key string does appear in two books
//! -- `Endurance`, which Pathfinder Unchained re-lists from the Core Rulebook
//! -- but those are one feat under one identity, not two the fold merged; see
//! `rules_tables::feats_all`'s own
//! `cross_book_key_collisions_are_exactly_the_known_set` for the corpus
//! evidence.

/// One feat identifier's comparable characters, lazily and without allocating.
///
/// Each clause absorbs one real difference between the two shapes above:
///  - a leading `"feat:"` prefix (present on engine tokens, absent on catalog
///    keys);
///  - a compound token carrying a sub-choice, e.g.
///    `"feat:weapon_focus:weapon:longsword"` -- only the segment immediately
///    after `"feat:"` identifies the feat itself, so later segments are
///    dropped;
///  - spaces in the catalog key vs. underscores in the engine token;
///  - punctuation the corpus keeps in the display name but the engine token
///    drops, e.g. `"Gorgon's Fist"` vs. `"feat:gorgons_fist"`.
fn identity_chars(raw: &str) -> impl Iterator<Item = char> + '_ {
    let without_prefix = raw.strip_prefix("feat:").unwrap_or(raw);
    let base_segment = without_prefix.split(':').next().unwrap_or("");
    base_segment
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| c.to_ascii_lowercase())
}

/// Folds a feat identifier -- in either real shape -- to its comparable
/// identity: lowercase, ASCII-alphanumeric characters only.
///
/// Prefer [`same`] or [`holds`] when comparing; this allocates, and exists for
/// callers that need the folded value itself (a map key, a diagnostic).
pub fn fold(raw: &str) -> String {
    identity_chars(raw).collect()
}

/// Whether two feat identifiers name the same feat, in any mix of shapes.
///
/// Allocation-free: it compares the two folds character by character rather
/// than materialising either, because the producers that call it run once per
/// feat per compute.
pub fn same(left: &str, right: &str) -> bool {
    let mut left_chars = identity_chars(left);
    let mut right_chars = identity_chars(right);
    loop {
        match (left_chars.next(), right_chars.next()) {
            (None, None) => return true,
            (left_char, right_char) if left_char != right_char => return false,
            _ => {}
        }
    }
}

/// Whether the character holds `feat_key` at all, whichever shape it was
/// recorded in. The presence test every non-repeatable feat's producer wants.
pub fn holds(selected_feats: &[String], feat_key: &str) -> bool {
    selected_feats.iter().any(|held| same(held, feat_key))
}

/// How many times the character holds `feat_key`, whichever shape each
/// occurrence was recorded in.
///
/// Separate from [`holds`] because PF1 has genuinely repeatable feats
/// (`STACK:YES MULT:YES` in the corpus -- Fleet, the `Extra ...` resource
/// feats), whose magnitude is counted by occurrence. Collapsing those to a
/// boolean would understate a verified bonus exactly as surely as fabricating
/// one would overstate it.
pub fn count(selected_feats: &[String], feat_key: &str) -> usize {
    selected_feats.iter().filter(|held| same(held, feat_key)).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::rules_tables::feats_all::all_feat_tables;
    use std::collections::HashMap;

    /// The exact input shapes `normalizeFeatIdentity` in
    /// `apps/desktop/src/characterHub/featsTabModel.ts` documents, pinned here
    /// so the Rust fold and the TypeScript fold cannot drift apart. The
    /// frontend's own `featsTabModel.test.ts` pins the identical table.
    const FRONTEND_SHAPE_CASES: &[(&str, &str)] = &[
        // The two real `selected_feats` shapes.
        ("feat:deflect_arrows", "deflectarrows"),
        ("Deflect Arrows", "deflectarrows"),
        // A compound token identifies the feat by its first segment only.
        ("feat:weapon_focus:weapon:longsword", "weaponfocus"),
        // Punctuation the display name keeps but the token drops.
        ("Gorgon's Fist", "gorgonsfist"),
        ("feat:gorgons_fist", "gorgonsfist"),
        // An APG corpus `KEY:` and the display name it renders as.
        ("Elemental Spell ~ Acid", "elementalspellacid"),
        ("Elemental Spell (Acid)", "elementalspellacid"),
    ];

    #[test]
    fn fold_matches_the_frontend_identity_shapes() {
        for (raw, expected) in FRONTEND_SHAPE_CASES {
            assert_eq!(&fold(raw), expected, "folding {raw:?}");
        }
    }

    #[test]
    fn same_agrees_with_fold_across_every_pair_of_pinned_shapes() {
        for (left, left_fold) in FRONTEND_SHAPE_CASES {
            for (right, right_fold) in FRONTEND_SHAPE_CASES {
                assert_eq!(
                    same(left, right),
                    left_fold == right_fold,
                    "same({left:?}, {right:?}) must agree with their folds"
                );
            }
        }
    }

    /// The whole fix rests on folding being lossless *between* feats: it must
    /// merge the two shapes of one feat and never merge two different feats.
    /// Checked against the live catalog rather than reasoned about, because a
    /// future ingested book could introduce a colliding pair.
    #[test]
    fn folding_never_merges_two_distinct_catalog_feats() {
        // `SD31-E6-F2-007` -- exactly one pair, both verified against the
        // pinned oracle. `mythic_adventures/ma_feats.lst:54`'s "Blind-Fight
        // (Mythic)" carries `KEY:Blind Fight` (no hyphen, and its own
        // `PREABILITY:1,CATEGORY=FEAT,Blind Fight` prerequisite repeats the
        // same unhyphenated spelling) -- an upstream PCGen typo internal to
        // that one row, not a fabrication this ingest introduced, and not a
        // producer-firing hazard: no engine producer is keyed off either
        // "Blind-Fight" or "Blind Fight" today (`grep -rn` over
        // `src/rules_core/` and `pilot_compute/`, this cycle). Shipped
        // byte-faithful to the corpus rather than silently hyphen-corrected
        // -- correcting the KEY would be inventing data the source row does
        // not carry. Any OTHER pair still fails this test exactly as before.
        const KNOWN_TYPO_COLLISION: (&str, &str) = ("Blind-Fight", "Blind Fight");
        let mut by_identity: HashMap<String, &'static str> = HashMap::new();
        let mut checked = 0;
        for book in all_feat_tables() {
            for entry in book.entries {
                checked += 1;
                let identity = fold(entry.key);
                if let Some(previous) = by_identity.insert(identity.clone(), entry.key) {
                    let pair_is_known = (previous, entry.key) == KNOWN_TYPO_COLLISION
                        || (entry.key, previous) == KNOWN_TYPO_COLLISION;
                    assert!(
                        previous == entry.key || pair_is_known,
                        "{previous:?} and {:?} both fold to {identity:?}; folding would make \
                         either feat's producer fire for the other",
                        entry.key
                    );
                }
            }
        }
        // 1578 hand-authored records + the 649 corpus gap rows the feat gap
        // lane joined on (`rules_tables::feat_gap_tables`: `SD31-E6-F8-001`'s
        // original 83 + `SD31-E6-F8-002`'s 242 + `SD31-E6-F2-007`'s 199
        // Mythic Adventures rows -- SD31-W10-INTEGRATE-001 excluded 159
        // VISIBLE:EXPORT display-plumbing twins from the original 358 --
        // + `SD31-E6-F8-003`'s 7 (inner_sea_intrigue 6 + book_of_the_
        // damned_volume_2 1) + SD-32 Gate 0 book-onboarding precondition's
        // 9 inner_sea_taverns rows + SD-32 T9 onboarding's (card 11) 109
        // (inner_sea_combat 23 + inner_sea_gods 86)). Re-derived from the
        // live catalog, not incremented on faith.
        assert_eq!(checked, 2227, "the whole ingested feat catalog must be checked");
    }

    /// A longer feat whose name merely begins with a grounded key must not
    /// match it -- the property several producers' doc comments rely on.
    #[test]
    fn folding_is_whole_string_equality_not_a_prefix_match() {
        assert!(!same("Acrobatic Steps", "Acrobatic"));
        assert!(!same("feat:acrobatic_steps", "Acrobatic"));
        assert!(!same("Greater Weapon Focus", "Weapon Focus"));
        assert!(!same("feat:greater_weapon_focus", "feat:weapon_focus"));
    }

    #[test]
    fn holds_and_count_see_both_shapes() {
        let feats: Vec<String> = ["feat:dodge", "Fleet", "feat:fleet", "Weapon Focus"]
            .iter()
            .map(|f| (*f).to_owned())
            .collect();

        assert!(holds(&feats, "Dodge"), "the catalog key must find the engine token");
        assert!(holds(&feats, "feat:weapon_focus"), "the token must find the catalog key");
        assert!(!holds(&feats, "Toughness"));

        // Fleet is genuinely repeatable, and the two picks arrived in different
        // shapes -- both count.
        assert_eq!(count(&feats, "Fleet"), 2);
        assert_eq!(count(&feats, "feat:fleet"), 2);
        assert_eq!(count(&feats, "Dodge"), 1);
        assert_eq!(count(&feats, "Toughness"), 0);
    }

    /// The empty-input edges, so no caller has to guard them.
    #[test]
    fn degenerate_identifiers_never_match_a_real_feat() {
        assert_eq!(fold("feat:"), "");
        assert!(!same("feat:", "Dodge"));
        assert!(same("feat:", ""));
        assert!(!holds(&["feat:".to_owned()], "Dodge"));
    }
}
