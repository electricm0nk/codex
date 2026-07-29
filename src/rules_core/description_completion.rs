//! Whether a rules feature that has **no number to compute** is nonetheless
//! complete, because its real rulebook description reaches the player.
//!
//! ## Why this module exists
//!
//! A large fraction of Pathfinder features carry zero numeric tokens in the
//! corpus. Deflect Arrows is the canonical example: its entire benefit is a
//! resolution ("once per round, deflect one ranged attack that would have hit
//! you"), and the corpus record carries no `BONUS:` token at all. There is no
//! magnitude to ground, no formula to evaluate, and no arithmetic a compute
//! seam could ever produce for it — not because the engine is unfinished, but
//! because the feature genuinely has none.
//!
//! Treating such a feature as "blocked, not yet built" is a category error: it
//! describes a gap that can never close, because there is nothing on the other
//! side of it. The player's actual need is served the moment the rulebook text
//! is in front of them, at which point they invoke the ability at the table
//! themselves.
//!
//! ## The integrity rule this module enforces
//!
//! "Zero magnitude" alone is **not** sufficient to call a feature complete.
//! That would let this codebase mark something done that a player cannot see
//! anywhere — exactly the failure the repo's Wired Integration doctrine
//! (`docs/governance/no-stub-mvp-doctrine.md`) forbids: a `success: true`
//! returned by an operation that did not actually do the work.
//!
//! So completion here requires **two** independently checkable facts, and this
//! module refuses to conclude `TextComplete` unless both hold:
//!
//! 1. **The text exists.** The feature's catalog record carries real,
//!    non-empty description text sourced from the corpus `DESC:` token — not
//!    fabricated here, not a placeholder.
//! 2. **The text reaches this character's player.** The feature is recorded on
//!    the exact input field the shipped app renders from, so the description
//!    genuinely appears on the character sheet for *this* character.
//!
//! Fact 2 is character-specific on purpose. The same feature can be
//! text-complete for one character and not for another, because the question
//! is never "does this codebase know the text" but "will this player see it".
//! When fact 2 fails, the caller keeps its claim-blocking diagnostic — the gap
//! is real, and the honest report is that nothing surfaces the text.
//!
//! ## Scope boundary
//!
//! This module answers the completion question *only* for features with no
//! numeric magnitude. A feature that has a real number the engine has not yet
//! computed is not this module's business and must never be routed through it:
//! showing a player the description of a feature whose number is missing does
//! not supply the number. Callers own that judgement, and each caller's own
//! doc comment records why its feature is zero-magnitude.

use crate::rules_core::character_input::CharacterInput;
use crate::rules_core::rules_tables::crb::feats::feat_tables;

/// Where in the shipped desktop app a feature's description is rendered.
///
/// Each variant names a real, existing render path — not an aspiration. Adding
/// a variant here is a claim that the app genuinely displays that text today,
/// and every variant below is traced to its render site in this file's
/// resolver doc comments.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DescriptionSurface {
    /// The character sheet's Feats tab, which resolves each entry of
    /// `selected_feats` against the CRB feat catalog and renders that entry's
    /// name plus its `detail` line (category + corpus description).
    ///
    /// Render path, traced end to end:
    /// `rules_tables::crb::feats::feat_tables()` (`description` field, corpus
    /// `DESC:` verbatim) -> `apps/desktop/src-tauri/src/feat_catalog.rs`
    /// `map_catalog_entry` -> the `list_feats` Tauri command ->
    /// `apps/desktop/src/boundary/listFeats.ts` (`FeatCatalogEntryDto.description`)
    /// -> `apps/desktop/src/characterHub/itemPickerFilter.ts`
    /// `mapFeatCatalogEntries` (folds description into `detail`) ->
    /// `apps/desktop/src/characterHub/featsTabModel.ts`
    /// `resolveSelectedFeatEntries` -> `CharacterSheet.tsx`'s `FeatsTab`,
    /// which renders `row.entry.detail` in a paragraph under the feat name.
    FeatsTab,
}

/// Whether a zero-magnitude feature is complete for a given character.
///
/// Deliberately not a `bool`. The two outcomes are not "true and false" — they
/// are "complete, and here is the exact text the player sees" versus "still a
/// genuine gap, and here is why nothing surfaces it". Collapsing them to a
/// bool is how a codebase loses the ability to tell those apart, which is the
/// specific failure this module exists to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZeroMagnitudeResolution {
    /// Nothing further is owed: the feature has no number to compute, and its
    /// real description reaches this character's player in the shipped app.
    TextComplete {
        /// The exact description text the player sees, carried so the caller
        /// can quote it in its own explanation record rather than restating
        /// it — an explanation that quotes the real surfaced string cannot
        /// silently drift away from what the app actually shows.
        description: &'static str,
        /// Where the player sees it.
        surface: DescriptionSurface,
    },
    /// Still a genuine gap. The caller must keep its claim-blocking
    /// diagnostic.
    NotSurfaced {
        /// Why nothing surfaces the text, in terms a diagnostic message can
        /// state directly to a reader.
        reason: NotSurfacedReason,
    },
}

/// The distinct ways a zero-magnitude feature can fail to reach the player.
///
/// Separated because they call for different remedies and a diagnostic that
/// conflates them misdirects whoever reads it: one is a data gap in the
/// corpus-backed catalog, the other is a character whose build never recorded
/// the feature on the field the app renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotSurfacedReason {
    /// No catalog record resolves for this feature at all, so there is no
    /// text to show even in principle.
    NoCatalogRecord,
    /// A catalog record resolves but carries no usable description — the
    /// corpus record has no `DESC:` token, or it is empty. Rendering it would
    /// show the player a blank.
    NoDescriptionText,
    /// The text exists in the catalog, but this character has not recorded the
    /// feature on the field the app's render path reads, so the sheet shows
    /// the player nothing for it.
    NotRecordedOnRenderedField,
}

/// Folds a feat identifier down to a comparable identity: lowercase,
/// alphanumeric characters only, with any `feat:` prefix and any trailing
/// sub-choice segments dropped.
///
/// This is a deliberate, tested mirror of `normalizeFeatIdentity` in
/// `apps/desktop/src/characterHub/featsTabModel.ts`. It must stay in step with
/// it, because this module's whole claim is "the Feats tab will render this
/// feat's row" — a fold that matched here but not there would let this module
/// report `TextComplete` for a row the app silently drops.
/// `feat_identity_fold_matches_frontend_shapes` below pins the exact input
/// shapes that motivate each clause, taken from the frontend's own doc
/// comment.
///
/// The two real shapes in `selected_feats` are the catalog's human-readable
/// `key` (e.g. `"Deflect Arrows"`, what the in-sheet "Add Feat" picker pushes)
/// and the engine's `feat:snake_case` token (e.g. `"feat:deflect_arrows"`,
/// what character creation and level-up seed). Both must fold together.
fn fold_feat_identity(raw: &str) -> String {
    let without_prefix = raw.strip_prefix("feat:").unwrap_or(raw);
    let base_segment = without_prefix.split(':').next().unwrap_or("");
    base_segment
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

/// Resolve whether a **feat** with no numeric magnitude is complete for
/// `input`, on the strength of its description reaching the player.
///
/// `feat_key` may be given in either real shape (catalog key or engine
/// token) — it is folded the same way the frontend folds it.
///
/// Both required facts are checked against live data, never assumed:
///
/// 1. The feat resolves in the real 185-record CRB feat catalog
///    (`feat_tables()`) and that record's `description` is present and
///    non-blank. This is the same catalog, and the same field, that the
///    `list_feats` command projects to the frontend, so a description found
///    here is exactly the text the app receives.
/// 2. `input.chosen.selected_feats` carries the feat. That field is precisely
///    what `CharacterSheet.tsx`'s `FeatsTab` renders from, so its presence is
///    what makes the description appear on this character's sheet. A feat
///    named only by a choice-slot selection (`selected_choices`) is *not*
///    rendered anywhere — the Feats tab never reads that field — so this
///    check deliberately fails for it.
///
/// Both real in-app paths that grant a feat (`handleAddFeat` and
/// `handleLevelUpFeatPick` in `CharacterSheet.tsx`, including the bonus-feat
/// pick offered at level-up) route through `add_feat_selection`, which appends
/// to `selected_feats` — so a feat a player actually picked satisfies fact 2.
pub fn feat_description_completion(
    input: &CharacterInput,
    feat_key: &str,
) -> ZeroMagnitudeResolution {
    let identity = fold_feat_identity(feat_key);

    let Some(entry) = feat_tables()
        .iter()
        .find(|entry| fold_feat_identity(entry.key) == identity)
    else {
        return ZeroMagnitudeResolution::NotSurfaced {
            reason: NotSurfacedReason::NoCatalogRecord,
        };
    };

    let Some(description) = entry.description.filter(|text| !text.trim().is_empty()) else {
        return ZeroMagnitudeResolution::NotSurfaced {
            reason: NotSurfacedReason::NoDescriptionText,
        };
    };

    let recorded_on_rendered_field = input
        .chosen
        .selected_feats
        .iter()
        .any(|selected| fold_feat_identity(selected) == identity);

    if !recorded_on_rendered_field {
        return ZeroMagnitudeResolution::NotSurfaced {
            reason: NotSurfacedReason::NotRecordedOnRenderedField,
        };
    }

    ZeroMagnitudeResolution::TextComplete {
        description,
        surface: DescriptionSurface::FeatsTab,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::character_input::load_character_input_fixture;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn input_with_selected_feats(feats: &[&str]) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.selected_feats = feats.iter().map(|f| (*f).to_owned()).collect();
        input
    }

    /// The identity fold absorbs every shape difference the frontend's own
    /// `normalizeFeatIdentity` documents, so this module and the Feats tab
    /// always agree on which catalog row a raw selection resolves to.
    #[test]
    fn feat_identity_fold_matches_frontend_shapes() {
        // Engine token vs. catalog key, the two real `selected_feats` shapes.
        assert_eq!(fold_feat_identity("feat:deflect_arrows"), "deflectarrows");
        assert_eq!(fold_feat_identity("Deflect Arrows"), "deflectarrows");
        // A compound token carrying a sub-choice identifies the feat by its
        // first segment only.
        assert_eq!(
            fold_feat_identity("feat:weapon_focus:weapon:longsword"),
            "weaponfocus"
        );
        // Punctuation the display name keeps but the token drops.
        assert_eq!(fold_feat_identity("Gorgon's Fist"), "gorgonsfist");
        assert_eq!(fold_feat_identity("feat:gorgons_fist"), "gorgonsfist");
    }

    /// The proof case: Deflect Arrows carries real corpus description text and
    /// the character records it on the field the Feats tab renders, so the
    /// player genuinely sees it.
    #[test]
    fn feat_recorded_on_the_rendered_field_with_real_text_is_text_complete() {
        let input = input_with_selected_feats(&["feat:deflect_arrows"]);

        let resolution = feat_description_completion(&input, "Deflect Arrows");

        match resolution {
            ZeroMagnitudeResolution::TextComplete {
                description,
                surface,
            } => {
                assert_eq!(surface, DescriptionSurface::FeatsTab);
                assert!(
                    !description.trim().is_empty(),
                    "a text-complete verdict must carry the real surfaced text"
                );
                // Pinned against the live catalog record rather than a copy,
                // so this can never pass on text this module invented.
                let catalog_text = feat_tables()
                    .iter()
                    .find(|e| e.key == "Deflect Arrows")
                    .and_then(|e| e.description)
                    .expect("the CRB catalog carries Deflect Arrows with a description");
                assert_eq!(description, catalog_text);
            }
            other => panic!("expected TextComplete, got {other:?}"),
        }
    }

    /// The integrity guard, and the whole reason this returns a resolution
    /// rather than a bool: the catalog knows the text, but this character
    /// never recorded the feat on the rendered field, so the sheet shows the
    /// player nothing. That is still a real gap and must not read as
    /// complete.
    #[test]
    fn feat_absent_from_the_rendered_field_is_not_surfaced_even_though_text_exists() {
        let input = input_with_selected_feats(&["feat:power_attack"]);

        assert_eq!(
            feat_description_completion(&input, "Deflect Arrows"),
            ZeroMagnitudeResolution::NotSurfaced {
                reason: NotSurfacedReason::NotRecordedOnRenderedField,
            }
        );
    }

    /// Both real `selected_feats` shapes reach the same verdict — a character
    /// whose feat was added by the in-sheet picker (catalog key) is no less
    /// surfaced than one seeded with the engine token.
    #[test]
    fn catalog_key_shape_in_selected_feats_is_equally_surfaced() {
        let input = input_with_selected_feats(&["Deflect Arrows"]);

        assert!(matches!(
            feat_description_completion(&input, "feat:deflect_arrows"),
            ZeroMagnitudeResolution::TextComplete { .. }
        ));
    }

    /// A feat the catalog does not know cannot be surfaced, whatever the
    /// character recorded — the Feats tab falls back to rendering the raw
    /// string with no description at all.
    #[test]
    fn feat_with_no_catalog_record_is_not_surfaced() {
        let input = input_with_selected_feats(&["feat:not_a_real_feat"]);

        assert_eq!(
            feat_description_completion(&input, "feat:not_a_real_feat"),
            ZeroMagnitudeResolution::NotSurfaced {
                reason: NotSurfacedReason::NoCatalogRecord,
            }
        );
    }

    /// A catalog record with no corpus `DESC:` token would render a blank
    /// line to the player, which is not completion. Uses a real such record
    /// (the "Heighten Spell +N" bonus-tier family, per `FeatTableEntry`'s own
    /// doc comment) located from live data, so this test tracks the corpus
    /// rather than a hardcoded assumption.
    #[test]
    fn feat_with_no_description_text_is_not_surfaced() {
        let Some(descriptionless) = feat_tables()
            .iter()
            .find(|entry| entry.description.is_none_or(|d| d.trim().is_empty()))
        else {
            // Every catalog record has text today — then there is no way for
            // this branch to mislead a player, and nothing to assert.
            return;
        };
        let input = input_with_selected_feats(&[descriptionless.key]);

        assert_eq!(
            feat_description_completion(&input, descriptionless.key),
            ZeroMagnitudeResolution::NotSurfaced {
                reason: NotSurfacedReason::NoDescriptionText,
            },
            "record {:?} has no description and must not read as complete",
            descriptionless.key
        );
    }
}
