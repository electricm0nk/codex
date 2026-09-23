//! SD-36 Epic F1b, R2 (`docs/release/SD-36-consolidation/epic-f-class-completion.md` §3b.2,
//! review finding 2 -- CONFIRMED, original spec verified broken): the mechanical join from a
//! bespoke `pilot_compute` class-feature FACET id (`class_feature.acg.brawler.knockout_dc`) to
//! the converted `data/sheet_rules/**/class_feature/*.json` RULE the facet is really talking
//! about.
//!
//! This replaces `HeldSeed::from_character`'s old exact-tail walk (`sheet_rule.rs`, formerly
//! "longest leading run of the remaining segments"), which the review proved collapses nearly
//! every facet that has no exact-slug rule onto its class's own PRINCIPAL rule
//! (`class_feature.acg.brawler.knockout_dc` -> `brawler`, not `brawler_knockout`) -- a
//! systematic mis-join, not an edge case, since a principal `class_feature` rule with the bare
//! class slug exists for nearly every class.
//!
//! **The corrected rule.** Strip the `class_feature.` prefix, drop every `corpus_record` family
//! marker segment (wherever it falls), and find the segment that names THIS class (the caller's
//! own `class_slug`, already known -- no enumerated namespace list is needed: whatever precedes
//! the class segment, `acg`/`pu`/`untabled`/a book slug, is a namespace and is dropped
//! structurally, by position, not by name). An explanation id that never names this class at
//! all is refused outright (`JoinResult::None`) rather than guessed at -- the one invariant that
//! keeps the join from ever crossing classes.
//!
//! Everything after the class segment is the FEATURE tail. It is matched against every real
//! `class_feature` rule slug by LONGEST COMMON PREFIX, counted in whole underscore-joined
//! words -- but two things the spec's original one-shot "join the whole remaining string"
//! description undersells, both found by running this join over the real, live population
//! (`class_census --duplicates`, `docs/release/.../stage4/dedup-receipt.md`), are load-bearing:
//!
//! 1. **Two tiers, not one.** A SCOPED candidate must carry `class_slug`'s own word-prefix
//!    (`{class_slug}_...`) -- this is what keeps the join from ever crossing classes, and a
//!    candidate must share at least `class_slug`'s own words PLUS one feature word; `class_slug`
//!    alone is never a valid match length, and the bare class slug (the class's own principal
//!    rule) is refused explicitly IN BOTH TIERS, so no accounting slip can ever return it (a
//!    nested sliding-window tail can equal the bare class slug even in the tier with no class
//!    prefix requirement, so the guard is not redundant there). But PF1 also writes plenty of
//!    rule text ONCE and grants it from several classes with no owning-class prefix at all
//!    (`uncanny_dodge`, shared by Rogue's base rule and Ninja's borrowed one) -- for these, a
//!    BARE tier matches the tail alone (no class prefix) against every `class_feature` slug, but
//!    only when the candidate's words start with the WHOLE remaining tail, never a partial run
//!    (this is what keeps a bare match from ever drifting into an unrelated rule by a one-word
//!    coincidence). Both tiers are scored by the same yardstick -- how many of the facet's own
//!    tail words the match actually explains ("coverage") -- so a fuller, more specific match
//!    always outranks a shorter partial one regardless of which tier found it; a class-scoped
//!    match wins a true tie against a bare one, since it carries the extra class-identity check.
//!    A SCOPED candidate that explains strictly less than the tail AND still has its own
//!    unexplained leftover words is refused outright, not merely outscored: sharing one feature
//!    word with a candidate that then diverges is a coincidence, not a stem match, and is worth
//!    `None`/`Ambiguous`, never a confident wrong `Matched` (`magus_arcana.pool` must never
//!    resolve to the unrelated `magus_arcana_pool_strike`, an arcana named "Pool Strike"; a
//!    `raging_*` facet must never resolve to `skald_raging_song` on the shared word "raging"
//!    alone). A candidate that fully explains the tail, or that itself has no leftover words (a
//!    real, shorter stem the facet merely asks more detail of, e.g. `knockout_dc` ->
//!    `brawler_knockout`), is unaffected by this refusal.
//! 2. **A sliding window over dot-segments, not just the full tail.** Some facet ids nest a
//!    named sub-feature ahead of its own attribute
//!    (`class_feature.untabled.dread.dread_manifesting.power_points` -- `dread_manifesting` is
//!    itself a feature name, `power_points` is the attribute the tail names), and some
//!    (`class_feature.<class>.corpus_record.<feature_slug>`, the class-feature-grant consumer's
//!    own id shape) even re-embed the class name as the tail's own leading word
//!    (`tactician_manifesting`) despite the id already carrying the class as its own segment --
//!    collapsed here rather than left to poison the word count. The tail is tried from its
//!    fullest form down to progressively shorter dot-segment-bounded suffixes (dot segments are
//!    the id author's own structural boundaries; underscore words inside one segment are not),
//!    and every qualifying match at every window position competes on the same coverage/excess
//!    scale -- so a shorter, more literal match at a later window position can still beat a
//!    weaker partial match the fuller window found, but never a stronger one.
//!
//! Two candidates still tied on the best (coverage, excess, tier) triple are a real,
//! distinguishable failure mode (`JoinResult::Ambiguous`), never silently resolved to either one.

use crate::rules_core::sheet_rule::{RuleId, SheetRulePackage};

/// The result of [`rule_for_explanation`]. Not a plain `Option`: "two rule slugs share this
/// facet's longest matching prefix" is a real, distinguishable failure mode from "no rule
/// shares even the minimum prefix" -- both are surfaced, never silently resolved to either
/// candidate. Both `Ambiguous` and `None` mean the same thing to a caller deciding whether to
/// hold a rule: no converted rule for this facet, the bespoke value keeps printing alone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JoinResult {
    /// Exactly one `class_feature` rule matches, unambiguously.
    Matched(RuleId),
    /// More than one distinct `class_feature` rule ties for the best (coverage, excess, tier).
    Ambiguous(Vec<RuleId>),
    /// No `class_feature` rule shares even the minimum prefix (`class_slug` + one feature word),
    /// or the explanation id never names this class at all.
    None,
}

const CLASS_FEATURE_KIND: &str = "class_feature";
const CORPUS_RECORD_MARKER: &str = "corpus_record";
/// SCOPED beats BARE on an exact (coverage, excess) tie -- a class-prefixed match carries an
/// extra identity check a bare one does not.
const TIER_SCOPED: u8 = 0;
const TIER_BARE: u8 = 1;

/// Split an underscore-joined string into its non-empty words, then merge a lone `"s"`
/// word-token into the token immediately before it.
///
/// This is the mechanical undoing of the corpus's OWN apostrophe-to-underscore convention:
/// `nature's` converts to the slug fragment `nature_s`, `efreeti's` to `efreeti_s` -- a bare,
/// single-letter `"s"` segment is never a real English word on its own anywhere in this
/// population, it is always the tail of a possessive the converter's own slugifier split at
/// the underscore. Evidenced by real, committed slugs that only join once this merge runs
/// (SD-36 Epic F1b stage-4 fix pass, the dedup receipt's own §7.1 "11 dropped matches"
/// population diff): `core_rulebook:class_feature:druid_resist_nature_s_lure` (the facet's own
/// tail spells it `resist_natures_lure`, one word, no underscore) and
/// `adventurers_guide:class_feature:asavir_efreeti_s_blessing`. A structural fact about how the
/// corpus encodes possessives, applied identically to every slug this module ever splits into
/// words -- never a per-facet list.
fn words(s: &str) -> Vec<String> {
    let raw: Vec<&str> = s.split('_').filter(|w| !w.is_empty()).collect();
    let mut out: Vec<String> = Vec::with_capacity(raw.len());
    for w in raw {
        if w == "s" && !out.is_empty() {
            out.last_mut().expect("checked non-empty above").push('s');
        } else {
            out.push(w.to_owned());
        }
    }
    out
}

/// Two words are the SAME word for join purposes when they are equal, or differ only by a
/// trailing grammatical `"s"` (`"feat"` <-> `"feats"`, `"strike"` <-> `"strikes"`).
///
/// A second, independent normalisation class from [`words`]'s own apostrophe-merge: this one is
/// plain English pluralisation, not the corpus's possessive-slug convention, and the two never
/// overlap in practice (a possessive splits into a bare one-letter `"s"` TOKEN merged away
/// before this function ever runs; a plural is a real word that already ends in `s`). Evidenced
/// by real, committed slugs where a facet's own tail names the concept in one number and the
/// converted rule's slug names it in the other (SD-36 Epic F1b stage-4 fix pass, dedup receipt
/// §7.1): `class_feature.acg.brawler.bonus_feat_count` (facet says "feat") must reach
/// `advanced_class_guide:class_feature:brawler_bonus_feats` (rule says "feats"), and
/// `class_feature.pu.unchained_monk.style_strikes_known` (facet says "strikes") must reach
/// `pathfinder_unchained:class_feature:unchained_monk_style_strike` (rule says "strike"). A
/// class-agnostic grammatical fact, never a per-facet list.
fn words_eq(a: &str, b: &str) -> bool {
    a == b || format!("{a}s") == b || format!("{b}s") == a
}

/// The longest common prefix of `query` and `candidate`, word by word, where two words match
/// when [`words_eq`] says so. Returns `(len, exact)`: `len` is the matched word count (the
/// join's own coverage/excess arithmetic, unchanged by normalisation); `exact` is true only
/// when EVERY matched word pair was a literal, un-normalised match.
///
/// `exact` exists so a normalisation-dependent match (a plural substitution, or `words`'s own
/// apostrophe merge) can never unseat an already-correct LITERAL match into a manufactured tie
/// -- `consider`'s own ranking treats it as the lowest-priority tiebreak, after (coverage,
/// excess, tier), so it only ever matters between two candidates already tied on all three.
/// Evidenced by a real population regression the stage-4 fix pass's own re-run population scan
/// surfaced (dedup receipt §7.1): once `words_eq` lets a facet's singular "deed"/"feat" match a
/// rule's plural "deeds"/"feats", TWO real, distinct GENERIC pool-container records
/// (`advanced_class_guide:class_feature:swashbuckler_deeds`, the umbrella "Swashbucklers spend
/// panache points to accomplish deeds" record; `advanced_class_guide:class_feature:
/// warpriest_bonus_feats`, the umbrella bonus-feat-progression record) start tying, at the SAME
/// (coverage, excess, tier), against an ALREADY-correct, exact, more specific sibling match
/// (`swashbuckler_evasive`, `warpriest_focus_weapon`) the join held cleanly before this
/// normalisation existed -- turning a `Matched` into a manufactured `Ambiguous`, a real
/// `some -> ambiguous` regression this tiebreak closes back to zero.
fn longest_common_prefix(query: &[String], candidate: &[String]) -> (usize, bool) {
    let mut len = 0;
    let mut exact = true;
    for (a, b) in query.iter().zip(candidate.iter()) {
        if a == b {
            len += 1;
        } else if words_eq(a, b) {
            len += 1;
            exact = false;
        } else {
            break;
        }
    }
    (len, exact)
}

/// One qualifying candidate, ranked by (`coverage` descending, `excess` ascending, `tier`
/// ascending, `exact` descending) -- the higher this tuple, the better the match. `coverage` is
/// how many of the facet's own tail words (never counting `class_slug`'s own words) the match
/// explains; `excess` is how many of the CANDIDATE's own words are left over past that match (0
/// = an exact stem); `exact` is whether every matched word was literal, never a tokenisation
/// normalisation (see [`longest_common_prefix`]) -- the lowest-priority key, since it only ever
/// decides an otherwise-genuine tie.
struct Candidate<'a> {
    coverage: usize,
    excess: usize,
    tier: u8,
    exact: bool,
    slug: &'a str,
}

/// Folds a new candidate into the running best set: strictly better replaces it, tied joins it,
/// worse is dropped. `best` never mixes candidates of different (coverage, excess, tier, exact).
fn consider<'a>(best: &mut Vec<Candidate<'a>>, coverage: usize, excess: usize, tier: u8, exact: bool, slug: &'a str) {
    if let Some(top) = best.first() {
        let cmp = coverage
            .cmp(&top.coverage)
            .then(excess.cmp(&top.excess).reverse())
            .then(tier.cmp(&top.tier).reverse())
            .then(exact.cmp(&top.exact));
        match cmp {
            std::cmp::Ordering::Greater => {
                best.clear();
                best.push(Candidate { coverage, excess, tier, exact, slug });
            }
            std::cmp::Ordering::Equal => {
                best.push(Candidate { coverage, excess, tier, exact, slug });
            }
            std::cmp::Ordering::Less => {}
        }
    } else {
        best.push(Candidate { coverage, excess, tier, exact, slug });
    }
}

/// The mechanical facet-to-rule join. See the module doc comment for the corrected algorithm;
/// this function is the one Rust place it is implemented -- `HeldSeed::from_character`'s
/// held-set fixpoint calls it directly. The desktop frontend's `noticeHasSheetRule`
/// (`apps/desktop/src/characterHub/classFeaturesModel.ts::ruleForExplanation`, review finding
/// 5) cannot call this function across the wire boundary, so it is a byte-for-byte TypeScript
/// port instead, kept in lockstep by this shared spec (`epic-f-class-completion.md` §3b.2) and
/// by this module's own tests pinning the exact algorithm shape -- a deliberate, named mirror,
/// not an independent second algorithm.
pub fn rule_for_explanation(package: &SheetRulePackage, class_slug: &str, explanation_id: &str) -> JoinResult {
    let rest = explanation_id.strip_prefix("class_feature.").unwrap_or(explanation_id);
    let segs: Vec<&str> = rest.split('.').filter(|s| *s != CORPUS_RECORD_MARKER).collect();

    // Locate the class's own segment; everything before it is a namespace segment this join
    // does not need to name to drop. An id that never names this class at all cannot be safely
    // scoped to it -- refused, not guessed.
    let Some(class_at) = segs.iter().position(|s| *s == class_slug) else {
        return JoinResult::None;
    };
    let rest_segs = &segs[class_at + 1..];
    if rest_segs.is_empty() {
        return JoinResult::None; // the class's own segment alone -- never a valid match length
    }
    let class_words = words(class_slug);
    let scope_prefix = format!("{class_slug}_");
    let min_scoped_len = class_words.len() + 1;

    let mut best: Vec<Candidate> = Vec::new();

    // The sliding window: try the fullest tail first, then progressively shorter dot-segment
    // suffixes. Every window position's qualifying matches compete on the same scale below, so
    // trying a shorter window never displaces a genuinely stronger match a fuller window found.
    for i in 0..rest_segs.len() {
        let tail_joined = rest_segs[i..].join("_");
        let tail_words = words(&tail_joined); // RAW -- never mutated; the true tail this window names
        if tail_words.is_empty() {
            continue;
        }
        let original_tail_len = tail_words.len();

        // Scoped tier, against the RAW tail. Some facet ids (grant-consumer `corpus_record`
        // ids, some hand-authored parent-feature names) genuinely name a rule whose OWN slug
        // repeats the class word (`vigilante_vigilante_specialization`,
        // `gunslinger_gunslinger_initiative` are real, existing slugs, not typos) -- trying the
        // raw tail first is what lets an exact match to one of THESE win outright, before any
        // collapsed/looser interpretation below gets a vote.
        let raw_query: Vec<String> = class_words.iter().cloned().chain(tail_words.iter().cloned()).collect();
        for slug in package.slugs_of_kind(CLASS_FEATURE_KIND) {
            // Refuses the bare class slug (the class's own principal rule) explicitly, on top
            // of the length check below already excluding it -- belt and suspenders, per the
            // spec's own "under any circumstance" wording.
            if slug == class_slug || !slug.starts_with(&scope_prefix) {
                continue;
            }
            let candidate_words = words(slug);
            let (lcp, exact) = longest_common_prefix(&raw_query, &candidate_words);
            if lcp < min_scoped_len {
                continue;
            }
            let coverage = lcp - class_words.len();
            let excess = candidate_words.len() - lcp;
            // A partial match (the candidate does not explain the whole tail) AND a candidate
            // that itself has unexplained leftover words is a one-word coincidence, not a real
            // stem match (review finding 1, the magus/skald population cases) -- refused
            // outright (dropped from `best`, never scored), leaving `None`/`Ambiguous` as the
            // honest outcome instead of a confident, wrong `Matched`. A candidate that fully
            // consumes the tail, or that itself has no leftover words (a shorter, real stem the
            // facet merely asks more detail of), is unaffected.
            if coverage < tail_words.len() && excess > 0 {
                continue;
            }
            consider(&mut best, coverage, excess, TIER_SCOPED, exact, slug);
        }

        // Scoped tier, against a COLLAPSED tail: only tried when the raw tail itself starts
        // with a redundant repeat of the class's own words (a facet id re-embedding the class
        // name as the tail's OWN leading word despite the id already carrying the class as its
        // own segment). This is offered as an ADDITIONAL interpretation, never a replacement
        // for the raw one above -- a genuine doubled-name rule (previous paragraph) still wins
        // on excess (0 leftover words) when one exists; the collapsed interpretation only wins
        // when no such literal doubled-name rule does. A collapsed match earns full credit
        // (as if it covered the WHOLE original tail) only when it explains the ENTIRE collapsed
        // tail -- a partial collapsed match earns no bonus for the words it did not even
        // attempt to explain.
        if tail_words.len() > class_words.len() && tail_words[..class_words.len()] == class_words[..] {
            let collapsed_tail = &tail_words[class_words.len()..];
            let collapsed_query: Vec<String> = class_words.iter().cloned().chain(collapsed_tail.iter().cloned()).collect();
            for slug in package.slugs_of_kind(CLASS_FEATURE_KIND) {
                if slug == class_slug || !slug.starts_with(&scope_prefix) {
                    continue;
                }
                let candidate_words = words(slug);
                let (lcp, exact) = longest_common_prefix(&collapsed_query, &candidate_words);
                if lcp < min_scoped_len {
                    continue;
                }
                // Credited only for the collapsed words this match actually explains -- never
                // promoted to `original_tail_len` (review finding 1: that promotion is what let
                // a one-word coincidence in the collapsed tail outscore every honest competitor
                // and win outright, e.g. `magus_arcana.pool` -> `magus_arcana_pool_strike`).
                let coverage = lcp - class_words.len();
                let excess = candidate_words.len() - lcp;
                // Same partial-match-with-leftover refusal as the raw scoped tier, measured
                // against the true original tail length (never the collapsed one) -- a collapsed
                // match that does not explain the WHOLE original tail, and whose candidate still
                // has words of its own left over, is refused rather than scored.
                if coverage < original_tail_len && excess > 0 {
                    continue;
                }
                consider(&mut best, coverage, excess, TIER_SCOPED, exact, slug);
            }
        }

        // Bare tier: no class prefix required, but the candidate must be an EXACT match for the
        // whole remaining RAW tail -- the ENTIRE tail consumed AND no leftover words of the
        // candidate's own. Review finding 1's real population evidence (`magus_arcana.pool` ->
        // `magus_arcana_pool_strike`) turned out to be won by THIS tier, not (only) the scoped
        // one the finding's own mechanism description named: `pool_strike`'s first three words
        // literally are the raw tail `magus_arcana_pool`, so requiring only "the tail is a
        // matched PREFIX of the candidate" (candidate may have its own trailing words, here
        // `strike`) is exactly the same one-word/one-stem coincidence the scoped tier had, just
        // with no class-prefix check to even narrow it. Requiring `excess == 0` here -- a real
        // stem match, not a longer candidate that merely starts the same way -- is what keeps a
        // bare match from ever drifting into an unrelated rule by a coincidence at all. It still
        // refuses the class's own principal rule explicitly (review finding 2): a nested
        // sliding-window tail CAN equal the bare class slug (a later dot-segment named after the
        // class, e.g. a facet id shaped `class_feature.<class>.<class>`), so the comment this
        // replaced -- "a bare class-slug-only candidate could never fully consume a real
        // (non-empty) tail" -- was wrong, and the spec's "never under any circumstance" guard
        // belongs here too, not only in the scoped tier.
        for slug in package.slugs_of_kind(CLASS_FEATURE_KIND) {
            if slug == class_slug {
                continue;
            }
            let candidate_words = words(slug);
            let (lcp, exact) = longest_common_prefix(&tail_words, &candidate_words);
            if lcp != tail_words.len() || lcp != candidate_words.len() {
                continue;
            }
            consider(&mut best, lcp, 0, TIER_BARE, exact, slug);
        }
    }

    if best.is_empty() {
        return JoinResult::None;
    }
    let mut ids: Vec<RuleId> = best.iter().filter_map(|c| package.find(CLASS_FEATURE_KIND, c.slug).cloned()).collect();
    ids.sort();
    ids.dedup();
    match ids.len() {
        0 => JoinResult::None,
        1 => JoinResult::Matched(ids.into_iter().next().expect("len == 1")),
        _ => JoinResult::Ambiguous(ids),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::sheet_rule::{Applies, Provenance, SheetRule, SheetValue, Subject};

    fn rule(id: &str) -> SheetRule {
        SheetRule {
            id: id.to_owned(),
            label: id.to_owned(),
            value: SheetValue::Text,
            also: vec![],
            prose: vec![],
            applies: Applies::Always,
            target: None,
            bonus_type: None,
            print: true,
            pool: "special_ability".into(),
            tags: vec![],
            subject: Subject::Character,
            repeatable: false,
            granted_by: vec![],
            offers: None,
            grants: vec![],
            closure_complete: false,
            always_held: false,
            provenance: Provenance::default(),
        }
    }

    fn package(slugs: &[&str]) -> SheetRulePackage {
        let mut pkg = SheetRulePackage::new();
        for slug in slugs {
            pkg.insert_rule(rule(&format!("core_rulebook:class_feature:{slug}")));
        }
        pkg.finish();
        pkg
    }

    #[test]
    fn the_three_known_good_pairs_still_join() {
        let pkg = package(&["fighter", "fighter_bravery", "fighter_armor_training", "brawler", "brawler_ac_bonus"]);
        assert_eq!(
            rule_for_explanation(&pkg, "fighter", "class_feature.fighter.bravery"),
            JoinResult::Matched("core_rulebook:class_feature:fighter_bravery".into())
        );
        assert_eq!(
            rule_for_explanation(&pkg, "fighter", "class_feature.fighter.armor_training"),
            JoinResult::Matched("core_rulebook:class_feature:fighter_armor_training".into())
        );
        assert_eq!(
            rule_for_explanation(&pkg, "brawler", "class_feature.acg.brawler.ac_bonus"),
            JoinResult::Matched("core_rulebook:class_feature:brawler_ac_bonus".into())
        );
    }

    #[test]
    fn a_facet_id_never_joins_to_the_class_principal_rule() {
        let pkg = package(&["brawler", "brawler_knockout", "fighter"]);
        // review finding 2's own worked example: `knockout_dc` has no rule slug
        // `brawler_knockout_dc`; the corrected join must land on `brawler_knockout`
        // (a real stem match), never fall through to the bare `brawler` principal.
        assert_eq!(
            rule_for_explanation(&pkg, "brawler", "class_feature.acg.brawler.knockout_dc"),
            JoinResult::Matched("core_rulebook:class_feature:brawler_knockout".into())
        );
        // no rule named `fighter_weapon_and_armor_proficiency` exists in this package (the real
        // package's record for this concept is reverse-ordered,
        // `weapon_and_armor_proficiency_fighter`, and is therefore correctly out of scope too):
        // the join must refuse, never fall back to the bare `fighter` principal.
        assert_eq!(
            rule_for_explanation(&pkg, "fighter", "class_feature.fighter.weapon_and_armor_proficiency"),
            JoinResult::None
        );
    }

    #[test]
    fn class_slug_alone_is_never_a_valid_match_length() {
        let pkg = package(&["brawler", "brawler_knockout"]);
        assert_eq!(rule_for_explanation(&pkg, "brawler", "class_feature.brawler"), JoinResult::None);
    }

    #[test]
    fn an_exact_stem_wins_over_a_longer_sibling_that_merely_starts_the_same_way() {
        // The Barbarian Uncanny Dodge case (stage-3 receipt's own SUSPECTED-duplicate #1):
        // `barbarian_uncanny_dodge` and a genuinely distinct `barbarian_uncanny_dodge_tracker`
        // record share every word the facet id names; the shorter, fully-consumed candidate is
        // the correct join target, not an `Ambiguous` tie, because the tracker record has a
        // fourth word the facet never asked for.
        let pkg = package(&["barbarian", "barbarian_uncanny_dodge", "barbarian_uncanny_dodge_tracker"]);
        assert_eq!(
            rule_for_explanation(&pkg, "barbarian", "class_feature.barbarian.uncanny_dodge"),
            JoinResult::Matched("core_rulebook:class_feature:barbarian_uncanny_dodge".into())
        );
    }

    #[test]
    fn two_candidates_that_both_fully_explain_the_tail_and_only_then_diverge_are_ambiguous() {
        // The facet's tail is fully explained ("strike", 1 word, coverage == tail length) by
        // both candidates -- they only diverge on a THIRD word neither's own tail ever asked
        // about, so this is a real, distinguishable tie, not the one-word-coincidence shape
        // review finding 1 refuses (that shape has partial coverage; this one has full coverage
        // with each candidate independently more specific).
        let pkg = package(&["brawler", "brawler_strike_types", "brawler_strike_options"]);
        match rule_for_explanation(&pkg, "brawler", "class_feature.brawler.strike") {
            JoinResult::Ambiguous(mut ids) => {
                ids.sort();
                assert_eq!(
                    ids,
                    vec![
                        "core_rulebook:class_feature:brawler_strike_options".to_string(),
                        "core_rulebook:class_feature:brawler_strike_types".to_string(),
                    ]
                );
            }
            other => panic!("expected Ambiguous, got {other:?}"),
        }
    }

    /// Review finding 1: a SCOPED candidate that explains only PART of the facet's tail (here,
    /// "strike" of "strike_bonus" -- "bonus" is never addressed) AND itself has leftover words
    /// of its own ("types"/"options") is a one-word coincidence, not a real stem match. Refusing
    /// both means the tie collapses to `None`, not a confident (or even a merely surfaced-tied)
    /// `Matched`/`Ambiguous` -- `None` and `Ambiguous` are equally safe to a caller (see
    /// [`JoinResult`]'s own doc comment), but a coincidence this thin should not be dignified
    /// with a named tie either.
    #[test]
    fn a_partial_match_with_its_own_leftover_words_is_refused_not_tied() {
        let pkg = package(&["brawler", "brawler_strike_types", "brawler_strike_options"]);
        assert_eq!(rule_for_explanation(&pkg, "brawler", "class_feature.brawler.strike_bonus"), JoinResult::None);
    }

    #[test]
    fn the_join_never_crosses_classes() {
        let pkg = package(&["fighter", "fighter_bravery", "rogue", "rogue_bravery_of_the_bold"]);
        // Only `fighter`-scoped rules are ever candidates for a `fighter`-owned facet, even
        // when a same-tailed rule exists under a different class's prefix.
        assert_eq!(
            rule_for_explanation(&pkg, "fighter", "class_feature.fighter.bravery"),
            JoinResult::Matched("core_rulebook:class_feature:fighter_bravery".into())
        );
    }

    #[test]
    fn an_explanation_id_that_never_names_this_class_is_refused_not_guessed() {
        let pkg = package(&["brawler", "brawler_knockout"]);
        assert_eq!(rule_for_explanation(&pkg, "brawler", "class_feature.domain.abyssal_strength_10"), JoinResult::None);
    }

    #[test]
    fn corpus_record_ids_still_join_through_the_family_marker() {
        let pkg = package(&["rogue", "rogue_new_talents"]);
        assert_eq!(
            rule_for_explanation(&pkg, "rogue", "class_feature.rogue.corpus_record.new_talents"),
            JoinResult::Matched("core_rulebook:class_feature:rogue_new_talents".into())
        );
    }

    /// Population regression over the REAL converted package (SD-36 Epic F1b, the print-paths
    /// investigation, `desktop-print-paths.md` §2a/§2d): the three worked-example pairs the
    /// stage-3 blast-radius receipt's own SUSPECTED-duplicate list and review finding 2 both
    /// name, resolved against `data/sheet_rules/` as committed. Barbarian and Rogue Uncanny
    /// Dodge each carry a genuinely distinct `_tracker` sibling record (different
    /// `closure_rows`, not a `#`-suffixed fact of the base rule) -- the corrected join must
    /// still land on the BASE rule, not the tracker and not `Ambiguous`, because the facet id
    /// names no fourth word the tracker's extra word would answer (R3: the tracker is a
    /// separate oracle record that prints alongside the join's target, not a candidate the join
    /// itself picks between).
    #[test]
    fn real_package_barbarian_and_rogue_uncanny_dodge_join_the_base_rule_not_the_tracker() {
        let pkg = crate::rules_core::sheet_rule_package::package()
            .as_ref()
            .expect("the real data/sheet_rules/ package loads in this checkout");
        assert_eq!(
            rule_for_explanation(pkg, "barbarian", "class_feature.barbarian.uncanny_dodge"),
            JoinResult::Matched("core_rulebook:class_feature:barbarian_uncanny_dodge".into())
        );
        assert_eq!(
            rule_for_explanation(pkg, "rogue", "class_feature.rogue.uncanny_dodge"),
            JoinResult::Matched("core_rulebook:class_feature:rogue_uncanny_dodge".into())
        );
    }

    /// The real `weapon_and_armor_proficiency` records for Fighter and Wizard are named in
    /// REVERSED word order (`weapon_and_armor_proficiency_fighter`, not
    /// `fighter_weapon_and_armor_proficiency`) -- confirmed by `desktop-print-paths.md` §2d.
    /// §3b.2's own RED test names the acceptable outcomes for this exact pair: `None` OR
    /// `Ambiguous` -- either is a named, surfaced failure, never a silent guess -- but NEVER
    /// `Matched(fighter)`, the class principal rule, which also really exists in this package.
    /// The real answer here is `Ambiguous`: `fighter_weapon_mastery` and `fighter_weapon_training`
    /// both share the facet's leading two words ("fighter", "weapon") and neither is a false
    /// principal-rule fallback, so this is the mechanism doing its job, not a defect.
    #[test]
    fn real_package_reversed_word_order_is_reported_not_guessed() {
        let pkg = crate::rules_core::sheet_rule_package::package()
            .as_ref()
            .expect("the real data/sheet_rules/ package loads in this checkout");
        match rule_for_explanation(pkg, "fighter", "class_feature.fighter.weapon_and_armor_proficiency") {
            JoinResult::Matched(id) => panic!("must never silently match the principal rule, got {id}"),
            JoinResult::None | JoinResult::Ambiguous(_) => {}
        }
    }

    /// Ninja borrows Rogue's Uncanny Dodge text wholesale -- the real converted record carries
    /// NO class prefix at all (`core_rulebook:class_feature:uncanny_dodge`,
    /// `improved_uncanny_dodge`), because PF1 itself writes this rule once and several classes
    /// grant it. Found as a real `some->none` regression by the population scan
    /// (`class_census --duplicates`): the SCOPED tier alone (needing a `ninja_`-prefixed slug)
    /// finds nothing, so the BARE tier -- requiring the whole tail as a matched prefix -- is
    /// load-bearing here, not optional generosity.
    #[test]
    fn real_package_ninja_joins_the_bare_shared_uncanny_dodge_rules() {
        let pkg = crate::rules_core::sheet_rule_package::package()
            .as_ref()
            .expect("the real data/sheet_rules/ package loads in this checkout");
        assert_eq!(
            rule_for_explanation(pkg, "ninja", "class_feature.uc.ninja.uncanny_dodge"),
            JoinResult::Matched("core_rulebook:class_feature:uncanny_dodge".into())
        );
        assert_eq!(
            rule_for_explanation(pkg, "ninja", "class_feature.uc.ninja.improved_uncanny_dodge"),
            JoinResult::Matched("core_rulebook:class_feature:improved_uncanny_dodge".into())
        );
    }

    /// `class_feature.untabled.dread.dread_manifesting.power_points`: the tail nests a named
    /// sub-feature (`dread_manifesting`, itself starting with the class's own word) ahead of the
    /// actual attribute (`power_points`) the facet is about. Found as a real `some->none`
    /// regression: a single full-tail attempt scores the sub-feature name higher than the real,
    /// shorter, class-scoped target (`dread_power_points`) ever could -- only the sliding window,
    /// trying the shorter `power_points`-only tail after the fuller one fails to qualify at all,
    /// reaches the correct rule.
    #[test]
    fn real_package_a_nested_sub_feature_name_does_not_bury_the_real_attribute_rule() {
        let pkg = crate::rules_core::sheet_rule_package::package()
            .as_ref()
            .expect("the real data/sheet_rules/ package loads in this checkout");
        assert_eq!(
            rule_for_explanation(pkg, "dread", "class_feature.untabled.dread.dread_manifesting.power_points"),
            JoinResult::Matched("ultimate_psionics:class_feature:dread_power_points".into())
        );
    }

    /// `class_feature.untabled.tactician.corpus_record.tactician_manifesting`: the grant
    /// consumer's own `feature_slug` re-embeds the owning class's name (`tactician_manifesting`)
    /// even though the id already carries `tactician` as its own segment -- the doubled word
    /// must be collapsed, or it silently outscores the real target against an unrelated
    /// same-prefix rule that happens to share only the doubled class word.
    #[test]
    fn real_package_a_doubled_class_name_in_the_tail_is_collapsed_not_left_to_win() {
        let pkg = crate::rules_core::sheet_rule_package::package()
            .as_ref()
            .expect("the real data/sheet_rules/ package loads in this checkout");
        assert_eq!(
            rule_for_explanation(pkg, "tactician", "class_feature.untabled.tactician.corpus_record.tactician_manifesting"),
            JoinResult::Matched("ultimate_psionics:class_feature:tactician_manifesting".into())
        );
    }

    /// `class_feature.apg.oracle.life_mystery.healing_hands`: Oracle's mystery-power records are
    /// named `<mystery>_<power>` with NO class prefix at all
    /// (`life_mystery_healing_hands`), distinct from the class-scoped mystery-SELECTOR rule
    /// (`oracle_life_mystery`) that also exists and shares only the facet's first two tail words.
    /// The full-tail BARE match (coverage 4 of 4) must outrank the shorter SCOPED partial match
    /// (coverage 2 of 4) even though the scoped one carries the class prefix -- coverage, not
    /// tier, is the primary key.
    /// `class_feature.untabled.vigilante.corpus_record.vigilante_specialization`: the real
    /// converted package genuinely names its PARENT selector rule with the class word repeated
    /// (`vigilante_vigilante_specialization` -- not a typo, a real record, confirmed
    /// `ls data/sheet_rules/ultimate_intrigue/class_feature | grep vigilante_vigilante`), while
    /// `vigilante_specialization_avenger`/`_stalker` are its two SPECIFIC sub-options. Found as
    /// a real `some->ambiguous` regression: collapsing the doubled word unconditionally hid the
    /// real, more specific (zero-leftover) target behind a tie between the two sub-options.
    #[test]
    fn real_package_a_genuine_doubled_class_name_rule_wins_over_its_own_sub_options() {
        let pkg = crate::rules_core::sheet_rule_package::package()
            .as_ref()
            .expect("the real data/sheet_rules/ package loads in this checkout");
        assert_eq!(
            rule_for_explanation(pkg, "vigilante", "class_feature.untabled.vigilante.corpus_record.vigilante_specialization"),
            JoinResult::Matched("ultimate_intrigue:class_feature:vigilante_vigilante_specialization".into())
        );
    }

    /// `class_feature.uc.gunslinger.gunslinger_initiative`: same shape, no `corpus_record`
    /// marker this time -- the bespoke facet id itself already repeats the class word, and the
    /// real record's own slug matches it exactly. Found as a real `some->none` regression.
    #[test]
    fn real_package_a_doubled_class_name_facet_id_with_no_collapsed_alternative_still_joins() {
        let pkg = crate::rules_core::sheet_rule_package::package()
            .as_ref()
            .expect("the real data/sheet_rules/ package loads in this checkout");
        assert_eq!(
            rule_for_explanation(pkg, "gunslinger", "class_feature.uc.gunslinger.gunslinger_initiative"),
            JoinResult::Matched("ultimate_combat:class_feature:gunslinger_gunslinger_initiative".into())
        );
    }

    /// Review finding 1, confirmed-wrong join #1: the collapsed-tail arm used to credit
    /// `magus_arcana.pool` with the WHOLE original tail (3 words) merely for consuming the
    /// 2-word collapsed tail, letting it beat every honest competitor and land on an unrelated
    /// arcana literally named "Pool Strike". The real sheet held this rule with `granted_by:
    /// null` and it was the only `magus_arcana_*` record held on a magus:20 build -- this join
    /// was the sole cause. Neither `None` nor `Ambiguous` prints a rule line, so either is safe;
    /// `Matched(magus_arcana_pool_strike)` is the one forbidden outcome.
    #[test]
    fn real_package_magus_arcana_pool_never_joins_the_unrelated_pool_strike_arcana() {
        let pkg = crate::rules_core::sheet_rule_package::package()
            .as_ref()
            .expect("the real data/sheet_rules/ package loads in this checkout");
        match rule_for_explanation(pkg, "magus", "class_feature.untabled.magus.magus_arcana.pool") {
            JoinResult::Matched(id) if id == "ultimate_magic:class_feature:magus_arcana_pool_strike" => {
                panic!("must never join the collapsed-tail one-word coincidence to an unrelated arcana, got {id}")
            }
            _ => {}
        }
    }

    /// Review finding 1, confirmed-wrong joins #2/#3: a SCOPED match sharing only the class
    /// prefix plus the single word "raging" must never beat the real `rage_power_raging_climber`
    /// / `rage_power_raging_swimmer` records (which this package does carry) by landing on the
    /// unrelated `skald_raging_song`.
    #[test]
    fn real_package_skald_raging_climber_and_swimmer_never_join_raging_song_on_one_word() {
        let pkg = crate::rules_core::sheet_rule_package::package()
            .as_ref()
            .expect("the real data/sheet_rules/ package loads in this checkout");
        for facet in ["raging_climber", "raging_swimmer", "raging_leaper"] {
            let id = format!("class_feature.acg.skald.{facet}");
            match rule_for_explanation(pkg, "skald", &id) {
                JoinResult::Matched(rid) if rid == "advanced_class_guide:class_feature:skald_raging_song" => {
                    panic!("{facet}: must never join a one-word 'raging' coincidence to skald_raging_song, got {rid}")
                }
                _ => {}
            }
        }
    }

    /// Review finding 2: the bare tier had no principal-rule guard. A nested sliding-window tail
    /// can equal the bare class slug (`class_feature.fighter.fighter`, or any id whose LAST
    /// dot-segment is the class's own name), which the bare tier's "whole remaining tail is a
    /// matched prefix" rule alone does not exclude -- the class's own principal rule slug IS a
    /// one-word, zero-excess match for a one-word tail equal to the class slug.
    #[test]
    fn bare_tier_never_returns_the_class_principal_rule_either() {
        // Only the class's own principal rule exists in this package -- pre-fix, the bare tier
        // (no principal guard) would match it outright, since a one-word tail equal to the class
        // slug is trivially "the whole remaining tail, fully consumed" by the class's own slug.
        let pkg = package(&["fighter"]);
        assert_eq!(rule_for_explanation(&pkg, "fighter", "class_feature.fighter.fighter"), JoinResult::None);
        // A nested window landing on a class-named final segment, reached only via the sliding
        // window (the fuller windows fail to qualify at all): `class_feature.fighter.x.fighter`.
        assert_eq!(rule_for_explanation(&pkg, "fighter", "class_feature.fighter.x.fighter"), JoinResult::None);
    }

    /// SD-36 Epic F1b stage-4 fix pass (dedup receipt §7.1, "the 11 dropped matches"): the
    /// blocker-1 refusal rule that closed the magus/skald one-word coincidences also dropped 11
    /// OTHER facets from `Matched` to `None` as an unnamed side effect (every one `before: None`
    /// in the pre-R2 naive walk too, so none is a shipped regression). Of the 11, these six
    /// (plus the 2 skald `raging_*` cases already pinned refused above, and 1 magus retarget)
    /// are pure TOKENISATION variants -- the corpus's own possessive-apostrophe convention
    /// (`words`'s merge) or plain English singular/plural (`words_eq`) -- recovered here by that
    /// one mechanical normalisation, never a per-facet list. See `dedup-receipt.md` §7.1 for the
    /// full population diff and the other three, which stay refused (real content divergence,
    /// not a tokenisation variant).
    #[test]
    fn real_package_tokenisation_variants_recover_six_of_the_eleven_dropped_matches() {
        let pkg = crate::rules_core::sheet_rule_package::package()
            .as_ref()
            .expect("the real data/sheet_rules/ package loads in this checkout");
        // Possessive-apostrophe split: the facet's own tail spells the word `natures` (one
        // word, no underscore); the real converted rule's slug spells it `nature_s` (the
        // corpus's own apostrophe-to-underscore convention, `words`'s merge undoes it).
        assert_eq!(
            rule_for_explanation(pkg, "druid", "class_feature.druid.resist_natures_lure"),
            JoinResult::Matched("core_rulebook:class_feature:druid_resist_nature_s_lure".into())
        );
        // Singular/plural (`words_eq`): the facet asks "how many bonus feat(s)" (`_count`, a
        // qualifier over a shorter, real stem the candidate already fully explains -- zero
        // leftover words of its own); the real rule's slug names the concept in the plural.
        assert_eq!(
            rule_for_explanation(pkg, "brawler", "class_feature.acg.brawler.bonus_feat_count"),
            JoinResult::Matched("advanced_class_guide:class_feature:brawler_bonus_feats".into())
        );
        assert_eq!(
            rule_for_explanation(pkg, "swashbuckler", "class_feature.acg.swashbuckler.bonus_feat_count"),
            JoinResult::Matched("advanced_class_guide:class_feature:swashbuckler_bonus_feats".into())
        );
        assert_eq!(
            rule_for_explanation(pkg, "unchained_monk", "class_feature.pu.unchained_monk.bonus_feats_known"),
            JoinResult::Matched("pathfinder_unchained:class_feature:unchained_monk_bonus_feat".into())
        );
        assert_eq!(
            rule_for_explanation(pkg, "unchained_monk", "class_feature.pu.unchained_monk.style_strikes_known"),
            JoinResult::Matched("pathfinder_unchained:class_feature:unchained_monk_style_strike".into())
        );
        // The apostrophe merge also recovers a SECOND, unrelated facet through the sliding
        // window (`efreeti's` -> `efreeti_s`, at a nested dot-segment position, not the tail's
        // own leading word) -- onto the MORE SPECIFIC of two real sibling records that share
        // the merged stem (`asavir_efreeti_s_blessing_mount`, which also explains the tail's own
        // "mount" word, over the shorter `asavir_efreeti_s_blessing`): a real, mechanically
        // justified outcome of "the fullest full-consumption candidate wins" (`dedup-receipt.md`
        // §7.1 names the one content nuance this raises -- the sibling's own prose does not
        // carry a fire-resistance clause, a converted-package content question, not a join
        // defect: this join reports what the corpus supports, same as `desktop-print-paths.md`'s
        // `weapon_and_armor_proficiency` naming-inconsistency finding).
        assert_eq!(
            rule_for_explanation(
                pkg,
                "asavir",
                "class_feature.adventurers_guide.asavir.efreeti_blessing_mount.fire_resistance"
            ),
            JoinResult::Matched("adventurers_guide:class_feature:asavir_efreeti_s_blessing_mount".into())
        );
    }

    /// SD-36 Epic F1b stage-4 fix pass, dedup receipt §7.1: the `words_eq` plural/singular
    /// normalisation, run over the REAL population, creates a manufactured tie between an
    /// already-correct EXACT match and a real, distinct GENERIC pool-container record that only
    /// ties because of the normalisation (`swashbuckler_deeds`'s own tail describes the whole
    /// deed family, "Swashbucklers spend panache points to accomplish deeds"; `warpriest_
    /// bonus_feats`'s own tail describes the class's whole bonus-feat progression) -- a real
    /// `some -> ambiguous` regression the population re-run surfaced. `longest_common_prefix`'s
    /// own `exact` tiebreak (a literal match beats a normalisation-dependent one on an otherwise
    /// genuine tie) closes it back to the pre-normalisation `Matched` answer, without narrowing
    /// `words_eq` itself (which the six recoveries above still need).
    #[test]
    fn real_package_an_exact_match_beats_a_normalised_tie_against_a_generic_pool_container() {
        let pkg = crate::rules_core::sheet_rule_package::package()
            .as_ref()
            .expect("the real data/sheet_rules/ package loads in this checkout");
        assert_eq!(
            rule_for_explanation(pkg, "swashbuckler", "class_feature.acg.swashbuckler.deed.evasive_grant"),
            JoinResult::Matched("advanced_class_guide:class_feature:swashbuckler_evasive".into())
        );
        assert_eq!(
            rule_for_explanation(pkg, "warpriest", "class_feature.acg.warpriest.focus_weapon.bonus_feat_granted"),
            JoinResult::Matched("advanced_class_guide:class_feature:warpriest_focus_weapon".into())
        );
    }

    /// The other six of the same 11 (dedup receipt §7.1): real content divergence, not a
    /// tokenisation variant, so the refusal rule is correct to keep refusing them -- a
    /// tokenisation normalisation must never widen into "drop any word the candidate lacks".
    #[test]
    fn real_package_genuine_content_divergence_among_the_eleven_stays_refused() {
        let pkg = crate::rules_core::sheet_rule_package::package()
            .as_ref()
            .expect("the real data/sheet_rules/ package loads in this checkout");
        // The facet names an extra descriptive word ("combat") the real rule's own slug never
        // uses at all -- not a plural/apostrophe variant, a different word placed mid-tail.
        assert_eq!(
            rule_for_explanation(pkg, "cavalier", "class_feature.apg.cavalier.bonus_combat_feat_count"),
            JoinResult::None
        );
        // The facet's trailing words ("flanking_level") and the rule's own trailing word
        // ("tracker") name genuinely different concepts past the shared "uncanny_dodge" stem.
        assert_eq!(
            rule_for_explanation(pkg, "bloodrager", "class_feature.acg.bloodrager.uncanny_dodge_flanking_level"),
            JoinResult::None
        );
        // "attack" and "of_blows" are unrelated words, not a tokenisation of the same word.
        assert_eq!(
            rule_for_explanation(pkg, "unchained_monk", "class_feature.pu.unchained_monk.flurry_attack_count"),
            JoinResult::None
        );
    }

    #[test]
    fn real_package_a_full_bare_match_outranks_a_shorter_scoped_partial_match() {
        let pkg = crate::rules_core::sheet_rule_package::package()
            .as_ref()
            .expect("the real data/sheet_rules/ package loads in this checkout");
        assert_eq!(
            rule_for_explanation(pkg, "oracle", "class_feature.apg.oracle.life_mystery.healing_hands"),
            JoinResult::Matched("advanced_players_guide:class_feature:life_mystery_healing_hands".into())
        );
    }
}
