//! SD-27 (`decisions.md` §28), feat-seam defect: **a feat wired once must
//! affect both compute paths, and it must be impossible to wire one without
//! the other.**
//!
//! # The defect this file exists to prevent returning
//!
//! This engine has two compute twins for the same pillars:
//!
//! * `rules_core::pilot_compute`'s `compute_combat_baseline` /
//!   `compute_selected_skill_modifiers` -- hardcoded Chain-Shirt arithmetic,
//!   exercised by most of the test suite; and
//! * `rules_core::pilot_compute_corpus`'s `compute_combat_baseline_from_corpus`
//!   / `compute_selected_skill_modifiers_from_corpus` -- real corpus-resolved
//!   equipment, and the pair `pf1_adapter::resolve_unified_pilot_snapshot`
//!   actually gates on, so **the pair whose numbers reach a player's sheet**.
//!
//! Measured on 2026-07-31, before the fix:
//!
//! ```text
//! grep -o 'feat_effects::[a-z_]*' src/rules_core/pilot_compute.rs        | sort -u | wc -l  -> 34
//! grep -o 'feat_effects::[a-z_]*' src/rules_core/pilot_compute_corpus.rs | sort -u | wc -l  ->  0
//! ```
//!
//! The corpus twin consumed **zero** feat effects and hand-inlined Dodge as its
//! only feat awareness. A feat wired into `pilot_compute.rs` therefore went
//! green in tests and changed nothing on screen -- a false-green generator.
//! Five feats were live in exactly that state, verified by running the
//! behavioural sibling of this file's guard before the fix:
//!
//! | feat | book | cell | hardcoded twin | sheet |
//! |------|------|------|----------------|-------|
//! | Athletic | CRB | Climb / Swim | 7 / 7 | 5 / 5 |
//! | Persuasive | CRB | Intimidate | 5 | 3 |
//! | Intimidating Prowess | CRB | Intimidate | 6 | 3 |
//! | Armor of the Pit | ARG | Armor Class | 19 | 17 |
//! | Sure and Fleet | ARG | Climb | 7 | 5 |
//!
//! # The guard
//!
//! `pilot_compute::feat_derived_pillar_contributions` is now the sole
//! `feat_effects` reader for every pillar the two twins derive independently,
//! and both twins consume it. The test below enforces that **structurally**, by
//! reading the two source files: neither twin's pillar functions may name
//! `feat_effects::` at all. Re-introducing a direct per-path read fails here.
//!
//! Its behavioural counterpart lives beside the code it guards, in
//! `pilot_compute_corpus.rs`:
//! `every_catalog_feat_moves_both_compute_paths_identically` sweeps the live
//! 690-record feat catalog (CRB + APG + ACG + ARG + PU) and pins all nine
//! shared cells equal across the two paths, feat by feat. The two guards are
//! deliberately different in kind: the behavioural one catches a divergence in
//! any feat that exists today, the structural one catches the *shape* that
//! produces divergences tomorrow, including for a producer no catalog feat
//! reaches yet.

const PILOT_COMPUTE: &str = include_str!("../src/rules_core/pilot_compute.rs");
const PILOT_COMPUTE_CORPUS: &str = include_str!("../src/rules_core/pilot_compute_corpus.rs");

/// The one function allowed to read `feat_effects` on behalf of both twins.
const SHARED_SEAM: &str = "feat_derived_pillar_contributions";

/// The four functions that each derive a shared pillar independently, and so
/// are the ones that can diverge. Named with the file they live in.
const TWIN_PILLAR_FUNCTIONS: &[(&str, &str)] = &[
    ("pilot_compute.rs", "compute_combat_baseline"),
    ("pilot_compute.rs", "compute_selected_skill_modifiers"),
    ("pilot_compute_corpus.rs", "compute_combat_baseline_from_corpus"),
    ("pilot_compute_corpus.rs", "compute_selected_skill_modifiers_from_corpus"),
];

/// The body of a top-level `fn <name>(` -- from its signature line to the first
/// column-0 `}`, which is where `rustfmt` puts every top-level item's closing
/// brace in this crate.
///
/// Matched on `fn <name>(` specifically, not on the bare name, so a doc comment
/// or a call site mentioning the function does not masquerade as its
/// definition. `compute_combat_baseline` is a prefix of
/// `compute_combat_baseline_from_corpus`, so the trailing `(` is load-bearing.
fn function_body<'a>(source: &'a str, name: &str) -> &'a str {
    let needle = format!("fn {name}(");
    let mut lines = source.lines();
    let mut start = None;
    let mut offset = 0usize;
    for line in lines.by_ref() {
        if line.starts_with(&needle) || line.starts_with(&format!("pub {needle}")) {
            start = Some(offset);
            break;
        }
        offset += line.len() + 1;
    }
    let start = start.unwrap_or_else(|| panic!("no top-level definition of `fn {name}(` found"));

    let mut end = start;
    let mut seen_signature = false;
    for line in source[start..].lines() {
        if seen_signature && line == "}" {
            return &source[start..end + 1];
        }
        seen_signature = true;
        end += line.len() + 1;
    }
    panic!("`fn {name}(` has no column-0 closing brace");
}

/// The snippet with `//` line comments removed.
///
/// Load-bearing, not cosmetic: this codebase documents heavily, and both twins
/// carry doc comments and inline notes that legitimately *name*
/// `feat_effects::skill_bonuses_from_feats` and friends while calling nothing.
/// Scanning raw text would flag that prose as a violation and push the next
/// author to delete the explanation instead of fixing the wiring -- exactly
/// backwards.
///
/// Quote state is tracked per line so a `//` inside a string literal (a URL,
/// say) is not mistaken for a comment. Escaped quotes are honoured.
fn strip_line_comments(snippet: &str) -> String {
    let mut out = String::with_capacity(snippet.len());
    for line in snippet.lines() {
        let bytes: Vec<char> = line.chars().collect();
        let mut in_string = false;
        let mut escaped = false;
        let mut cut = bytes.len();
        for index in 0..bytes.len() {
            let current = bytes[index];
            if escaped {
                escaped = false;
                continue;
            }
            match current {
                '\\' if in_string => escaped = true,
                '"' => in_string = !in_string,
                '/' if !in_string && index + 1 < bytes.len() && bytes[index + 1] == '/' => {
                    cut = index;
                    break;
                }
                _ => {}
            }
        }
        out.extend(bytes[..cut].iter());
        out.push('\n');
    }
    out
}

/// Every distinct `feat_effects::<producer>` name a snippet's **code** (not its
/// comments) references.
fn feat_effects_producers(snippet: &str) -> Vec<String> {
    let code = strip_line_comments(snippet);
    let mut found: Vec<String> = Vec::new();
    for (index, _) in code.match_indices("feat_effects::") {
        let rest = &code[index + "feat_effects::".len()..];
        let name: String = rest
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect();
        if name.is_empty() {
            continue;
        }
        if !found.contains(&name) {
            found.push(name);
        }
    }
    found.sort();
    found
}

/// The structural invariant. Neither twin may read `feat_effects` for a pillar
/// it derives independently -- both must go through the shared seam, which is
/// what makes "wired into one path only" unrepresentable rather than merely
/// discouraged.
#[test]
fn the_two_compute_twins_read_feat_effects_only_through_the_shared_seam() {
    let mut offenders: Vec<String> = Vec::new();

    for (file, function) in TWIN_PILLAR_FUNCTIONS {
        let source = match *file {
            "pilot_compute.rs" => PILOT_COMPUTE,
            "pilot_compute_corpus.rs" => PILOT_COMPUTE_CORPUS,
            other => panic!("unknown source file {other}"),
        };
        let body = function_body(source, function);

        for producer in feat_effects_producers(body) {
            offenders.push(format!(
                "{file}::{function} calls feat_effects::{producer} directly -- move it into \
                 pilot_compute::{SHARED_SEAM} so BOTH compute paths pick it up"
            ));
        }

        assert!(
            body.contains(SHARED_SEAM),
            "{file}::{function} must consume pilot_compute::{SHARED_SEAM}; without it, a feat \
             wired into the other twin is invisible in this one"
        );
    }

    assert!(
        offenders.is_empty(),
        "{} direct feat_effects read(s) inside a twin pillar function. This is the exact shape \
         that made ARG's Armor of the Pit and four other feats change a test and change nothing \
         a player could see:\n{}",
        offenders.len(),
        offenders.join("\n")
    );
}

/// The seam must actually be a `feat_effects` reader -- otherwise the test
/// above could pass vacuously against a seam that had quietly stopped
/// consuming anything, with every feat effect silently dropped from both twins
/// instead of one.
#[test]
fn the_shared_seam_is_where_the_pillar_feat_producers_actually_live() {
    let seam = function_body(PILOT_COMPUTE, SHARED_SEAM);
    let producers = feat_effects_producers(seam);

    assert_eq!(
        producers,
        vec![
            "arg_computed_climb_bonus_from_feats".to_string(),
            "armor_of_the_pit_natural_armor_bonus_from_feats".to_string(),
            "skill_bonuses_from_feats".to_string(),
        ],
        "the shared seam's feat_effects producer set changed. That is fine -- but update this \
         pin deliberately, and confirm the new producer reaches BOTH twins through an accessor \
         on FeatDerivedPillarContributions rather than being read in one twin only"
    );
}

/// Both twins must read the shared seam, and the corpus twin -- the one whose
/// numbers reach the sheet -- must not name `feat_effects` anywhere in the
/// whole file.
///
/// Stated as a whole-file property, not just a per-function one: a helper added
/// further down that file, called from a pillar function, would slip past the
/// per-function scan above.
#[test]
fn the_corpus_twin_names_no_feat_effects_producer_anywhere() {
    let producers = feat_effects_producers(PILOT_COMPUTE_CORPUS);
    assert!(
        producers.is_empty(),
        "pilot_compute_corpus.rs must reach every feat effect through \
         pilot_compute::{SHARED_SEAM}, never directly; found: {producers:?}"
    );
    assert!(
        PILOT_COMPUTE_CORPUS.contains(SHARED_SEAM),
        "pilot_compute_corpus.rs must consume pilot_compute::{SHARED_SEAM}"
    );
}
