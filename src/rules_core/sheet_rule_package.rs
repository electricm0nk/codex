//! Process-wide `data/sheet_rules/` package handle, reachable from pure
//! `rules_core` functions and from `src/bin/*.rs` targets such as
//! `class_census` -- SD-36 Epic F `epic-f-class-completion.md` §3.4a
//! (review finding 5).
//!
//! # Why this exists
//!
//! `compute_pilot_base_chassis` / `build_pilot_headless_receipt`
//! (`pilot_compute/class_shared_core.rs`) are the PURE functions every suite
//! in this crate and the `class_census` bin call, and neither has a
//! `SheetRulePackage` in hand -- `sheet_lines` on the returned struct is
//! documented "Empty from `compute_pilot_base_chassis` itself, which has no
//! `data/sheet_rules/` package in hand". `with_sheet_rules`, the only thing
//! that loads one, has exactly ONE production caller today:
//! `apps/desktop/src-tauri/src/character_hub.rs`'s private
//! `sheet_rule_package()` -- a process-wide `OnceLock` around
//! `corpus_loader::load_sheet_rules`, keyed off the desktop crate's own
//! `authoring_workbench::codex_repo_root()`. That pattern is desktop-crate
//! local and unreachable from `rules_core`'s own pure functions or from a
//! `rules_core`-only binary like `class_census`, which is not a desktop
//! binary and cannot depend on desktop-crate code.
//!
//! This module is the rules_core-level counterpart: same `OnceLock` shape,
//! same "named reason, never a silent empty package" contract, keyed off
//! [`crate::support::paths::repo_root`] (the same root
//! `pilot_compute::class_chassis_sheet_rules`'s own `static CACHE` already
//! uses) instead of a desktop-crate path helper.
//!
//! # Contract
//!
//! [`load_package_from`] never panics. It returns a named [`Err`] describing
//! exactly why the package could not be built -- an absent
//! `data/sheet_rules/` directory, or one present but carrying zero rules --
//! and never a silently empty [`SheetRulePackage`] standing in for "no data
//! here". [`package`] is the process-wide cached form callers reach for in
//! practice; it loads [`crate::support::paths::repo_root`] exactly once per
//! process and hands back the same `Ok`/`Err` on every later call.

use std::path::Path;
use std::sync::OnceLock;

use crate::rules_core::corpus_loader;
use crate::rules_core::sheet_rule::SheetRulePackage;
use crate::support::paths::repo_root;

/// Loads the `data/sheet_rules/` package rooted at `root`. Never panics.
/// Returns a named [`Err`] -- never a silently empty package -- when
/// `<root>/data/sheet_rules` is not a directory at all, or when it is a
/// directory that yields zero converted rules (an empty/degenerate
/// checkout, distinguished from "directory absent" in the message so a
/// reader knows which failure it hit).
pub fn load_package_from(root: &Path) -> Result<SheetRulePackage, String> {
    let dir = root.join("data/sheet_rules");
    if !dir.is_dir() {
        return Err(format!(
            "no data/sheet_rules directory at {} (regenerate with `cargo run --locked -p \
             codex-ingest --bin sheet_rule_convert`)",
            dir.display()
        ));
    }
    let load = corpus_loader::load_sheet_rules(&dir);
    if load.package.rules.is_empty() {
        return Err(format!(
            "data/sheet_rules directory at {} carries no rules ({} file diagnostics; \
             regenerate with `cargo run --locked -p codex-ingest --bin sheet_rule_convert`)",
            dir.display(),
            load.diagnostics.len()
        ));
    }
    Ok(load.package)
}

/// The `data/sheet_rules/` package, loaded once per process from
/// [`crate::support::paths::repo_root`] -- the rules_core-level counterpart
/// to the desktop crate's own `character_hub::sheet_rule_package` (§3.4a).
/// Every caller inside `rules_core` (and every `src/bin/*.rs` target, which
/// cannot reach the desktop crate at all) reaches the package through this
/// function, never by loading it again itself.
pub fn package() -> &'static Result<SheetRulePackage, String> {
    static PACKAGE: OnceLock<Result<SheetRulePackage, String>> = OnceLock::new();
    PACKAGE.get_or_init(|| load_package_from(&repo_root()))
}

/// SD-36 F3c4: the character's legacy Path-A picks (`choice:<pool> -> <ns>:<member>`) linked to
/// the converted options its held choosers offer for them
/// ([`crate::rules_core::sheet_rule::link_path_a_picks`], one rule), over the process-wide
/// package. The held set it links against is the character's classes and race alone -- the
/// chooser a class offers is held through the class. Each link says whether the option is held
/// once the pick is recorded (`option_held`). Empty when the package does not load or no pick
/// links. Cached per (race, class levels, picks).
pub fn linked_picks(
    input: &crate::rules_core::character_input::CharacterInput,
) -> Vec<crate::rules_core::sheet_rule::LinkedPick> {
    use crate::rules_core::sheet_rule::{held_set, id_slug, link_path_a_picks, CharacterFacts, HeldSeed};
    use std::collections::BTreeMap;
    use std::sync::Mutex;
    type Key = (String, Vec<(String, u8)>, Vec<(String, String)>);
    static CACHE: OnceLock<Mutex<BTreeMap<Key, Vec<crate::rules_core::sheet_rule::LinkedPick>>>> = OnceLock::new();
    let Ok(package) = package() else { return Vec::new() };
    let chosen = &input.chosen;
    let picks: Vec<(String, String)> = chosen
        .selected_choices
        .iter()
        .filter(|c| c.choice_set_id.starts_with("choice:"))
        .map(|c| (c.choice_set_id.clone(), id_slug(&c.selection_id)))
        .collect();
    // Cheap pre-check: a pick can link only when a rule named `<pool>_<member>` or `<member>`
    // exists (`link_path_a_picks`).
    if !picks.iter().any(|(set, member)| {
        let pool = set.strip_prefix("choice:").unwrap_or(set);
        !package.find_in_every_kind(&format!("{pool}_{member}")).is_empty() || !package.find_in_every_kind(member).is_empty()
    }) {
        return Vec::new();
    }
    let classes: Vec<(String, u8)> = chosen.class_levels.iter().map(|c| (id_slug(&c.class_id), c.level)).collect();
    let key: Key = (chosen.race_id.clone(), classes.clone(), picks.clone());
    let cache = CACHE.get_or_init(|| Mutex::new(BTreeMap::new()));
    if let Ok(guard) = cache.lock()
        && let Some(hit) = guard.get(&key)
    {
        return hit.clone();
    }
    let class_levels: Vec<(String, i64)> = classes.iter().map(|(c, l)| (c.clone(), i64::from(*l))).collect();
    let race = Some(id_slug(&chosen.race_id));
    let seed = HeldSeed { race: race.clone(), classes: class_levels.clone(), ..HeldSeed::default() };
    let mut facts = CharacterFacts {
        level: class_levels.iter().map(|(_, l)| *l).sum(),
        class_levels,
        race,
        ..CharacterFacts::default()
    };
    for (set, member) in &picks {
        facts.choices.entry(set.clone()).or_default().push((member.clone(), member.clone()));
    }
    let held = held_set(package, &seed, &facts);
    let mut links = link_path_a_picks(package, &held, &facts);
    if !links.is_empty() {
        for link in &links {
            facts.choices.entry(link.chooser.clone()).or_default().push((link.option.clone(), link.option.clone()));
        }
        let with_picks = held_set(package, &seed, &facts);
        for link in &mut links {
            link.option_held = with_picks.holds_itself(&link.option);
        }
    }
    if let Ok(mut guard) = cache.lock() {
        guard.insert(key, links.clone());
    }
    links
}

#[cfg(test)]
mod tests {
    use super::*;

    /// F1.9: the real repo checkout's `data/sheet_rules/` loads cleanly and
    /// carries the full converted population.
    #[test]
    fn the_real_repo_root_loads_a_nonempty_package() {
        let result = load_package_from(&repo_root());
        let package = result.as_ref().expect("real data/sheet_rules must load");
        assert!(!package.rules.is_empty(), "a real checkout must carry converted rules");
    }

    /// F1.9: an absent `data/sheet_rules` directory is a NAMED `Err`, never
    /// a panic and never a silently empty package -- proven with a real
    /// temporary root that genuinely carries no `data/sheet_rules` at all
    /// (mirrors the scratch-root idiom `derived_evaluator_fixture_check.rs`
    /// already uses for this crate's other filesystem-rooted tests).
    #[test]
    fn an_absent_sheet_rules_directory_is_a_named_err_not_a_panic_or_empty_ok() {
        let root = std::env::temp_dir()
            .join(format!("codex_sheet_rule_package_handle_absent_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("scratch root creates cleanly");

        let result = load_package_from(&root);

        let _ = std::fs::remove_dir_all(&root);

        let Err(reason) = result else {
            panic!("an absent data/sheet_rules directory must be a named Err, not Ok");
        };
        assert!(
            reason.contains("data/sheet_rules") && reason.contains(&root.display().to_string()),
            "the Err must name the exact missing path, not a generic message: {reason:?}"
        );
    }

    /// The same contract, over a root whose `data/sheet_rules` directory
    /// exists but is genuinely empty (zero rule files) -- distinguished
    /// from "directory absent" so an operator reads the right cause.
    #[test]
    fn an_empty_sheet_rules_directory_is_also_a_named_err_never_an_empty_ok() {
        let root = std::env::temp_dir()
            .join(format!("codex_sheet_rule_package_handle_empty_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("data/sheet_rules")).expect("scratch dirs create cleanly");

        let result = load_package_from(&root);

        let _ = std::fs::remove_dir_all(&root);

        let Err(reason) = result else {
            panic!("an empty data/sheet_rules directory must be a named Err, not Ok");
        };
        assert!(reason.contains("no rules"), "the Err must name the empty-population cause: {reason:?}");
    }

    /// The cached, process-wide handle agrees with a direct
    /// [`load_package_from`] call over the same real root -- the cache
    /// changes WHERE the load happens (once per process), never WHAT it
    /// returns.
    #[test]
    fn the_cached_handle_agrees_with_a_direct_load_over_the_real_root() {
        let direct = load_package_from(&repo_root());
        let cached = package();
        assert_eq!(direct.is_ok(), cached.is_ok());
        if let (Ok(direct), Ok(cached)) = (&direct, cached) {
            assert_eq!(direct.rules.len(), cached.rules.len());
        }
    }

    /// This process's own resident set size, in KiB, from `/proc/self/status`
    /// -- Linux only, `None` off-Linux. Used only by the one-off measurement
    /// below, never by production code.
    fn resident_memory_kb() -> Option<u64> {
        let status = std::fs::read_to_string("/proc/self/status").ok()?;
        status
            .lines()
            .find(|l| l.starts_with("VmRSS:"))
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|v| v.parse().ok())
    }

    /// §3.4a's required measurement: first-load wall time and the resident
    /// memory delta a loaded [`SheetRulePackage`] holds for its real
    /// population, from one real run -- not an estimate. `#[ignore]`d so the
    /// ordinary `cargo test` suite does not pay a second whole-package parse
    /// on every run (the two tests above already prove correctness); run
    /// deliberately with `--ignored --nocapture --test-threads=1` (isolated
    /// so no other test's own memory use pollutes the delta) to produce the
    /// numbers recorded in
    /// `docs/../scratchpad/sd36/f1/package-handle-measurements.md`.
    #[test]
    #[ignore = "one-off measurement (F1.9); run with --ignored --nocapture --test-threads=1"]
    fn measure_first_load_wall_time_and_resident_memory() {
        // First load in this fresh process. Its wall time is the closest
        // proxy this sandbox can measure for a "cold" load: dropping the OS
        // page cache (`/proc/sys/vm/drop_caches`) needs root and would also
        // hit every other agent sharing this box's filesystem cache, so it
        // is deliberately not attempted here -- see the receipt this test's
        // output is copied into for that caveat, stated plainly rather than
        // silently passed off as a true cold-cache number.
        let before_first_kb = resident_memory_kb();
        let start_first = std::time::Instant::now();
        let first = load_package_from(&repo_root()).expect("real data/sheet_rules loads");
        let elapsed_first = start_first.elapsed();
        let after_first_kb = resident_memory_kb();

        // Second load, same process, same files: the OS page cache is now
        // certainly warm (this process just read every one of those files),
        // and this call bypasses the `OnceLock` entirely (a fresh
        // `SheetRulePackage`, not the cached one) so it pays the full parse
        // again -- a genuine warm-cache timing, not a cache-hit no-op.
        let before_second_kb = resident_memory_kb();
        let start_second = std::time::Instant::now();
        let second = load_package_from(&repo_root()).expect("real data/sheet_rules loads");
        let elapsed_second = start_second.elapsed();
        let after_second_kb = resident_memory_kb();

        println!(
            "SHEET_RULE_PACKAGE_MEASUREMENT first_wall_ms={} second_wall_ms={} rules={} vars={} \
             rss_before_first_kb={:?} rss_after_first_kb={:?} rss_delta_first_kb={:?} \
             rss_before_second_kb={:?} rss_after_second_kb={:?} rss_delta_second_kb={:?}",
            elapsed_first.as_millis(),
            elapsed_second.as_millis(),
            first.rules.len(),
            first.vars.len(),
            before_first_kb,
            after_first_kb,
            before_first_kb.zip(after_first_kb).map(|(b, a)| a.saturating_sub(b)),
            before_second_kb,
            after_second_kb,
            before_second_kb.zip(after_second_kb).map(|(b, a)| a.saturating_sub(b)),
        );
        assert_eq!(first.rules.len(), second.rules.len(), "two loads of the same on-disk package must agree");
    }
}
