//! Tool-side predicates over a class-feature pool candidate's ingested `.lst`
//! row.
//!
//! SD-35 `AT-35-E6-002` cycle 3, under `decisions.md` §11 and
//! `technical-design.md` §0 (the boundary is by path). These four readings
//! decide whether a corpus record is a standing, prose-only member of a class
//! feature pool. Every one of them is a question about the **ingest format** —
//! which token keys the row carries, how many `DESC:` fields it has, whether a
//! `PREABILITY` names an archetype category — so they belong on the converter
//! side, and `src/rules_core/class_feature_pool_catalog.rs` calls them across
//! the boundary rather than walking the token array itself.
//!
//! Behaviour-identical to the versions that lived in that module: the doc
//! comments recording *why* each refusal exists moved here verbatim with the
//! code they justify. `raw_tokens_carry_more_than_one_desc_segment` is the one
//! rename — it named the ingest field in its own identifier — and is
//! [`carries_more_than_one_desc_segment`] here.
//!
//! KEPT for Starfinder, like the rest of `src/pcgen_import/`.

use serde_json::Value;

use crate::pcgen_import::ingest_record;

/// Ingest token keys that carry a real, player-facing engine effect --
/// wave-22 adversarial review CONFIRMED (finding, severity high) that 9 of
/// the lane's 88 originally-banked records carry one of these alongside a
/// clean-rendering description (e.g. `Finesse Rogue`'s own `ABILITY:FEAT|
/// VIRTUAL|Weapon Finesse`, `Skill Mastery`'s `SELECT:3+INT`). Decision 7
/// condition 1 ("prose only, not a mechanic") and condition 2 ("nothing to
/// compute") both fail for a record carrying any of these -- the render-
/// and-refuse gate above only catches an UNRESOLVED `%N` inside the prose
/// itself, never a wholly separate mechanical token the description text
/// never mentions at all. Refused here, at the corpus-row level, per
/// Decision 7's own binding PROXY WARNING (hand-verify the WHOLE row, not
/// a magnitude-token proxy, before banking a zero-magnitude unit).
pub const ENGINE_EFFECT_TOKEN_KEYS: &[&str] =
    &["ABILITY", "CSKILL", "SELECT", "AUTO", "SAB", "BONUS", "DEFINE", "ADD", "SPELLS", "DR", "SR"];

/// SD31-W29-INTEGRATE (Ruling §18, `OPERATOR-RULINGS-2026-08-21.md`):
/// *"we need to show only valid choices."*
///
/// **Corrected mid-cycle, by this same integration pass, after a blanket
/// "any `PRE*` token" version of this guard broke three pre-existing,
/// correctly-served real records** (`core_rulebook: Rage Power ~ Clear
/// Mind` — `PREVARGTEQ:RagePowersPrereqLVL,8`; `advanced_class_guide:
/// Rage Power ~ Elemental Blood (Greater)` and `~ Linnorm Death Curse
/// (Crag)` — `PRELEVEL:MIN=4`/`MIN=8`). A blanket refusal conflates two
/// UNRELATED PF1e shapes:
///
/// * **A level/chain/skill gate within the pool's OWN class** (`PRELEVEL`,
///   `PREVARGTEQ` against a class-internal counter, most `PREABILITY
///   CATEGORY=Special Ability` chain prerequisites like "Greater" requiring
///   the character already hold "Lesser") — every character who stays in
///   this class and levels up CAN eventually take this option. It is a
///   real, valid, standing member of an OPEN pool (exactly what Ruling
///   §18's own worked answer already calls Rage Power/Rogue Talent: "any
///   [class] can eventually take any [option]") — the catalog is not
///   lying by listing it, the same way a feat reference list is not lying
///   by listing a feat the character does not qualify for YET.
/// * **A permanent, structural exclusion from the base class itself** — a
///   PCGen `PREABILITY` token whose value carries `CATEGORY=Archetype`,
///   meaning the option belongs to a specific ARCHETYPE swap
///   (`Barbarian Archetype ~ Giant Stalker`, etc.), not to the base class
///   the pool's `REGISTERED_POOL_GROUPS` entry is keyed against. A
///   character who never takes that archetype can NEVER take this option
///   at any level — this is the genuinely EXCLUSIVE-shaped case Ruling §18
///   forbids serving wholesale (confirmed: `adventurers_guide`'s
///   `giant_stalker_defense`/`topple_giant`/`underfoot`, all three
///   `PREABILITY = 1,CATEGORY=Archetype,Barbarian Archetype ~ Giant
///   Stalker`).
///
/// So the refusal is scoped to exactly the second shape: a `PREABILITY`
/// token whose value contains `CATEGORY=Archetype`. This catalog has no
/// character to check a level/skill prerequisite against, but it does not
/// need one to know an archetype-locked option is not a standing member of
/// the base class's own pool. A future cycle that wants real per-character
/// LEVEL/skill gating (so the picker can grey out, not just list,
/// not-yet-qualified options) needs that in the picker itself
/// (`class_feature_pool_picker.rs`), not a wider refusal here.
pub fn is_archetype_locked(data: &Value) -> bool {
    ingest_record::token_values(data, "PREABILITY").iter().any(|v| v.contains("CATEGORY=Archetype"))
}

/// `true` when the record's row carries no [`ENGINE_EFFECT_TOKEN_KEYS`] entry
/// -- i.e. it is genuinely prose-only, not merely prose-renders-clean.
pub fn has_no_engine_effect_token(data: &Value) -> bool {
    !ingest_record::token_keys(data).iter().any(|k| ENGINE_EFFECT_TOKEN_KEYS.contains(k))
}

/// A silent-truncation defect found by on-screen DoD-8 inspection while
/// widening this catalog to Rage Power (`SD31-W23-POOLMEMBER-002`), present
/// in neither the render-and-refuse gate nor the engine-effect-token gate:
/// PCGen ships a handful of records with MULTIPLE `DESC:` tab fields on the
/// same row -- a lead-in clause plus several `PREVAREQ:`-gated continuation
/// clauses, one per "which element/condition did the character pick" branch
/// (e.g. `Rage Power ~ Elemental Blood (Greater)`'s real oracle row: `DESC:
/// While raging, the barbarian gains` followed by four separate `DESC:
/// ...a burrow speed of 30 feet.|PREVAREQ:BloodRage Acid,1` / `...a swim
/// speed of 60 feet.|PREVAREQ:BloodRage Cold,1` / ... segments).
///
/// Refused structurally here: any record whose row carries more than
/// one `DESC:` field is, by construction, showing only a fragment of what
/// the oracle actually states, regardless of whether that fragment happens
/// to read as a complete sentence -- UNLESS [`shipped_description_is_the_
/// already_regenerated_safe_multi_desc_join`] proves this specific
/// record's shipped `data.description` has already been caught up (see
/// that function's own doc comment for why the proof, not just the shape,
/// gates the exception).
pub fn carries_more_than_one_desc_segment(data: &Value) -> bool {
    ingest_record::token_values(data, "DESC").len() > 1
}

/// The `AT-34-E3-001 class_feature_option_pool` cycle's own narrow fix,
/// sub-cause 8: `Martial Weapon Proficiency Output` (standalone) and
/// `Octopus Wild Shape ~ Poison` (pool) each carry a genuine sequential
/// DESC continuation with no mechanical reason for the split -- unlike
/// `Rage Power ~ Elemental Blood (Greater)`'s PREVAREQ-gated alternative
/// branches, joining every segment IS this record's real, complete
/// description. `cache_gen::class_feature::generate`'s own `desc_value`
/// (a different file, this package's disjoint-file-touch convention) now
/// performs that join at ingest time for exactly this safe shape, so a
/// record whose `data.description` has been regenerated since carries the
/// FULL joined text already.
///
/// **Why this function re-derives the join instead of trusting the shape
/// alone.** Corpus-wide, many OTHER multi-DESC records share the same
/// "no PREVAREQ/PREVARGTEQ gate" shape but have NOT been regenerated --
/// their shipped `data.description` is still the stale, first-segment-only
/// value the old `desc_value` produced. Gating on shape alone (relaxing
/// [`carries_more_than_one_desc_segment`] to skip every
/// ungated multi-DESC row) was tried and reverted: it silently served
/// ~186 other records' stale, truncated `data.description` across
/// multiple books and mechanisms this cycle does not own -- exactly the
/// silent-truncation defect this module exists to prevent, reopened at
/// corpus scale. Re-deriving the expected join from `raw_tokens` directly
/// and requiring it to match the ALREADY-SHIPPED `data.description` proves
/// ingest has actually caught up for this one record; every other
/// not-yet-regenerated record fails the equality check and stays refused,
/// unchanged from before this cycle.
pub fn shipped_description_is_the_already_regenerated_safe_multi_desc_join(
    data: &Value,
    shipped_description: &str,
) -> bool {
    let segments: Vec<&str> = ingest_record::token_values(data, "DESC");
    if segments.len() <= 1 {
        return false;
    }
    if segments[1..].iter().any(|s| s.contains("PREVAREQ") || s.contains("PREVARGTEQ")) {
        return false;
    }
    let expected_join = segments.iter().map(|s| s.trim()).collect::<Vec<_>>().join(" ");
    expected_join == shipped_description
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// The `data` object shape these predicates are handed in production --
    /// the record's own token array under its real field name, read through
    /// [`ingest_record`].
    fn row(tokens: Value) -> Value {
        json!({"key": "Probe", "raw_tokens": tokens})
    }

    #[test]
    fn carries_more_than_one_desc_segment_counts_desc_keys_only() {
        let one = row(json!([{"key": "KEY", "value": "x"}, {"key": "DESC", "value": "x"}]));
        assert!(!carries_more_than_one_desc_segment(&one));
        let two = row(json!([
            {"key": "DESC", "value": "a"},
            {"key": "DESC", "value": "b"},
        ]));
        assert!(carries_more_than_one_desc_segment(&two));
        // A second occurrence of an unrelated key must never trip this check.
        let unrelated_repeat = row(json!([
            {"key": "DESC", "value": "a"},
            {"key": "SOURCEPAGE", "value": "p.1"},
            {"key": "SOURCEPAGE", "value": "p.2"},
        ]));
        assert!(!carries_more_than_one_desc_segment(&unrelated_repeat));
    }

    #[test]
    fn shipped_description_is_the_already_regenerated_safe_multi_desc_join_requires_an_exact_match() {
        let two_plain = row(json!([
            {"key": "DESC", "value": "a"},
            {"key": "DESC", "value": "b"},
        ]));
        // Not yet regenerated: shipped description is still just the first
        // segment -- stays refused.
        assert!(!shipped_description_is_the_already_regenerated_safe_multi_desc_join(&two_plain, "a"));
        // Regenerated: shipped description is the full safe join.
        assert!(shipped_description_is_the_already_regenerated_safe_multi_desc_join(&two_plain, "a b"));
        // A choice-branch-gated row never has a safe join, regardless of
        // what the shipped description says.
        let choice_gated = row(json!([
            {"key": "DESC", "value": "While raging, the barbarian gains"},
            {"key": "DESC", "value": " a burrow speed of 30 feet.|PREVAREQ:BloodRage Acid,1"},
        ]));
        assert!(!shipped_description_is_the_already_regenerated_safe_multi_desc_join(
            &choice_gated,
            "While raging, the barbarian gains a burrow speed of 30 feet.|PREVAREQ:BloodRage Acid,1"
        ));
        // A single-DESC row has nothing to join.
        let one = row(json!([{"key": "DESC", "value": "a"}]));
        assert!(!shipped_description_is_the_already_regenerated_safe_multi_desc_join(&one, "a"));
    }

    #[test]
    fn has_no_engine_effect_token_refuses_ability_and_select_but_allows_a_plain_desc_only_record() {
        let clean = row(json!([{"key": "KEY", "value": "x"}, {"key": "DESC", "value": "x"}]));
        assert!(has_no_engine_effect_token(&clean));
        let with_ability = row(json!([{"key": "ABILITY", "value": "FEAT|VIRTUAL|Weapon Finesse"}]));
        assert!(!has_no_engine_effect_token(&with_ability));
        let with_select = row(json!([{"key": "SELECT", "value": "3+INT"}]));
        assert!(!has_no_engine_effect_token(&with_select));
    }

    // SD31-W29-INTEGRATE (Ruling §18): only an ARCHETYPE-lock (a permanent,
    // structural exclusion from the base class) is refused -- an ordinary
    // level/chain/skill prerequisite within the pool's own class is not,
    // because every character of that class can eventually satisfy it.
    #[test]
    fn is_archetype_locked_refuses_only_a_preability_category_archetype_token() {
        let clean = row(json!([{"key": "KEY", "value": "x"}, {"key": "DESC", "value": "x"}]));
        assert!(!is_archetype_locked(&clean));

        // Ordinary within-class prerequisites -- must NOT be refused. Real
        // shapes: `core_rulebook: Rage Power ~ Clear Mind`
        // (`PREVARGTEQ:RagePowersPrereqLVL,8`), `advanced_class_guide:
        // Rage Power ~ Linnorm Death Curse (Crag)` (`PRELEVEL:MIN=4`), and
        // a `PREABILITY CATEGORY=Special Ability` chain prerequisite (e.g.
        // "Greater" requiring "Lesser" already held), all of which stay
        // served today.
        for (key, value) in [
            ("PREVARGTEQ", "RagePowersPrereqLVL,8"),
            ("PRELEVEL", "MIN=4"),
            ("PRESKILL", "1,Knowledge (Arcana)=5"),
            ("PREFACT", "Deity,Zon-Kuthon"),
            ("PREMULT", "1,[PRELEVEL:MIN=8],[PREABILITY:1,CATEGORY=Special Ability,X]"),
            ("PREABILITY", "1,CATEGORY=Special Ability,Rage Power ~ Elemental Blood (Lesser)"),
        ] {
            let ungated = row(json!([{"key": key, "value": value}]));
            assert!(
                !is_archetype_locked(&ungated),
                "{key}={value} is an ordinary within-class prerequisite, not an archetype lock"
            );
        }

        // The genuinely EXCLUSIVE shape -- must be refused.
        let archetype_gated = row(json!([
            {"key": "PREABILITY", "value": "1,CATEGORY=Archetype,Barbarian Archetype ~ Giant Stalker"}
        ]));
        assert!(is_archetype_locked(&archetype_gated));
    }

    /// A record with no token array at all is prose-only and unlocked, not a
    /// panic -- the "thin record" shape `ingest_record` reports as empty.
    #[test]
    fn a_thin_record_with_no_token_array_is_prose_only_and_unlocked() {
        let thin = json!({"key": "Thin"});
        assert!(has_no_engine_effect_token(&thin));
        assert!(!is_archetype_locked(&thin));
        assert!(!carries_more_than_one_desc_segment(&thin));
        assert!(!shipped_description_is_the_already_regenerated_safe_multi_desc_join(&thin, ""));
    }
}
