//! v0.6 corpus trap report — behavioural tests.
//!
//! **Why this exists.** Twenty of twenty-three PCGen books are still
//! un-ingested. Every ingestion cycle so far has rediscovered the *same*
//! corpus traps by hand, and nearly every count reported from those
//! rediscoveries was wrong on the first pass (396-vs-301 missing feats,
//! 207-vs-166 bonus-bearing feats, 180-vs-86 `BONUS:VAR` records). The
//! module under test turns that rediscovery into a mechanical scan an
//! agent can run against a book *before* writing any ingest code.
//!
//! The tests below are split into two halves that mean different things:
//!
//! * **Scanner behaviour** — hand-built LST text exercising each trap in
//!   isolation. These are the red-green-refactor tests for the parser and
//!   the trap classifiers. They never touch the real corpus, so they run
//!   everywhere.
//!
//! * **Corpus invariants** — assertions over the *already ingested*
//!   caches in `data/corpus/`, cross-checked against the real PCGen
//!   clone. These are the ratchets: they exist so a future ingest cannot
//!   silently reintroduce a trap that a previous ingest paid to learn
//!   about. They are gated on the corpus clone being present and skip
//!   (loudly) when it is not, because the clone is a developer-machine
//!   artifact, not a repo file.
//!
//! A note on severity, which the whole design turns on: a `.MOD` record
//! is *legitimate data*, not a defect. The defect is counting it as a
//! declaration. So the scanner reports `Severity::Trap` for corpus shapes
//! and reserves `Severity::Defect` for contradictions inside content we
//! already ingested.

use std::path::{Path, PathBuf};

use codex::pcgen_import::corpus_trap_baseline::{load_baseline, reconcile_trap_count};
use codex::pcgen_import::corpus_traps::{
    RecordShape, Severity, Trap, audit_ingested_cache, concept_census, scan_lst,
};

// ---------------------------------------------------------------------------
// Corpus location
// ---------------------------------------------------------------------------

/// Same default the cache-generator binaries (`gen_cache_acg` and
/// siblings) already use, with the same `PCGEN_CORPUS_ROOT` override.
fn corpus_root() -> Option<PathBuf> {
    let path = match std::env::var_os("PCGEN_CORPUS_ROOT") {
        Some(configured) => PathBuf::from(configured),
        // HOME-relative default: the operator keeps `workspace/` in the home
        // directory and syncs it between machines. Rust does not expand `~`.
        None => PathBuf::from(std::env::var_os("HOME").filter(|home| !home.is_empty())?)
            .join("workspace/repos/pcgen/data"),
    };
    if path.is_dir() { Some(path) } else { None }
}

fn books_dir(root: &Path) -> PathBuf {
    root.join("pathfinder/paizo/roleplaying_game")
}

fn cache_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/corpus")
}

/// `docs/governance/corpus-trap-baseline.tsv` — the registered, SD-33
/// inherited-debt baseline for four of this suite's corpus-invariant tests
/// (`decisions.md §13`). Loaded fresh every call, same pattern
/// `pi_sweep_rules_tables` uses for `pi-sweep-baseline.tsv`.
fn load_trap_baseline() -> Vec<codex::pcgen_import::corpus_trap_baseline::TrapBaselineEntry> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docs/governance/corpus-trap-baseline.tsv");
    load_baseline(&path).expect("docs/governance/corpus-trap-baseline.tsv parses")
}

// ===========================================================================
// Scanner behaviour — hand-built LST text
// ===========================================================================

/// Trap 1. `.MOD` modifies an existing record; it does not declare one.
/// Counting the two `.MOD` lines below as declarations would report three
/// feats where the file declares one.
#[test]
fn mod_lines_are_modifications_not_declarations() {
    let text = "Arcane Strike\tCATEGORY:FEAT\tTYPE:General\tDEFINE:ArcaneStrikeLVL|0\n\
                Arcane Strike.MOD\tBONUS:VAR|ArcaneStrikeLVL|1\n\
                CATEGORY=FEAT|Arcane Strike.MOD\tBONUS:VAR|ArcaneStrikeLVL|2\n";
    let scan = scan_lst("t.lst", text);

    assert_eq!(scan.declarations, 1, "one declaration, not three");
    assert_eq!(scan.modifications, 2);

    let mods: Vec<_> = scan.findings_for(Trap::ModRecord).collect();
    assert_eq!(mods.len(), 2);
    assert_eq!(mods[0].line, 2);
    assert_eq!(mods[0].severity, Severity::Trap, ".MOD is data, not a defect");
    assert!(
        mods[1].detail.contains("Arcane Strike"),
        "the finding must name the base record it modifies, got: {}",
        mods[1].detail
    );
    assert!(
        mods[0].detail.contains("declared in this file"),
        "should say whether the base declaration is present, got: {}",
        mods[0].detail
    );
}

/// A `.MOD` whose base record is *not* declared in the same file is a
/// different situation from one that is: the real declaration lives in
/// another book, so a book-scoped inventory that resolves it locally will
/// come up empty.
#[test]
fn mod_without_a_local_base_declaration_says_so() {
    let text = "Power Attack.MOD\tBONUS:COMBAT|DAMAGE|1\n";
    let scan = scan_lst("t.lst", text);
    let mods: Vec<_> = scan.findings_for(Trap::ModRecord).collect();
    assert_eq!(mods.len(), 1);
    assert!(
        mods[0].detail.contains("not declared in this file"),
        "got: {}",
        mods[0].detail
    );
}

/// The `.MOD` substring also appears *inside* token values — PCGen's
/// `var("STAT.3.MOD.NOEQUIP.NOTEMP")` is the real example from
/// `cr_feats.lst`'s `Spell Mastery`. A naive `grep -c '\.MOD'` counts
/// that line; only the first tab-separated field decides the shape.
#[test]
fn mod_inside_a_token_value_is_not_a_mod_record() {
    let text = "Spell Mastery\tCATEGORY:FEAT\tSELECT:var(\"STAT.3.MOD.NOEQUIP.NOTEMP\")\n";
    let scan = scan_lst("t.lst", text);
    assert_eq!(scan.modifications, 0);
    assert_eq!(scan.declarations, 1);
}

/// Trap 11 (not on the original list). `.COPY=` *does* declare a new
/// record — it is the mirror image of `.MOD`. Excluding it from a count
/// undercounts; the APG spell file carries 17 of them.
#[test]
fn copy_lines_declare_a_new_record() {
    let text = "Planar Binding\tSCHOOL:Conjuration\n\
                Planar Binding.COPY=Planar Binding (Demons Only)\tCLASSES:.CLEARALL\n";
    let scan = scan_lst("t.lst", text);

    assert_eq!(scan.copies, 1);
    assert_eq!(
        scan.declaring_lines(), 2,
        "a .COPY= line declares a record, so it counts toward the declaring total"
    );

    let copies: Vec<_> = scan.findings_for(Trap::CopyRecord).collect();
    assert_eq!(copies.len(), 1);
    assert_eq!(copies[0].record, "Planar Binding (Demons Only)");
    assert!(copies[0].detail.contains("Planar Binding"));
}

/// Trap 2. A `#`-prefixed line is a disabled duplicate that looks
/// entirely real — the APG's `#Elemental Fist` carries a live-looking
/// `TYPE:Combat`, full `BONUS:VAR` tokens and a `BENEFIT:`.
#[test]
fn hash_prefixed_records_are_disabled_not_declarations() {
    let text = "Elemental Fist\tCATEGORY:FEAT\tTYPE:Combat\tBONUS:VAR|ElementalFistDamage|1\n\
                #Elemental Fist\tCATEGORY:FEAT\tTYPE:Combat\tBONUS:VAR|ElementalFistDamage|1\n";
    let scan = scan_lst("t.lst", text);

    assert_eq!(scan.declarations, 1);
    assert_eq!(scan.disabled_records, 1);

    let disabled: Vec<_> = scan.findings_for(Trap::DisabledLine).collect();
    assert_eq!(disabled.len(), 1);
    assert_eq!(disabled[0].line, 2);
    assert_eq!(disabled[0].severity, Severity::Trap);
}

/// A prose comment is not a disabled record. `cr_spells.lst` opens with
/// `# CVS $Revision`, `###Block: Acid` and a `# Spell Name\tType\t...`
/// header row; classifying those as suppressed content would put noise
/// in front of every real finding.
#[test]
fn prose_comments_are_not_disabled_records() {
    let text = "# CVS $Revision: 1 $\n\
                ###Block: Acid\n\
                SOURCELONG:Core Rulebook\tSOURCESHORT:CR\n\
                Acid Splash\tSCHOOL:Conjuration\n";
    let scan = scan_lst("t.lst", text);

    assert_eq!(scan.disabled_records, 0);
    assert_eq!(scan.comments, 2);
    assert_eq!(scan.directives, 1, "SOURCELONG: row is a directive, not a record");
    assert_eq!(scan.declarations, 1);
}

/// The sharpest form of trap 2: the disabled twin carries *different*
/// rules than the live record. The APG's `Corruption Resistance` pair is
/// the real case — the disabled row lists `Inquisitor=2`, the live one
/// does not. Reading the wrong row ships a spell to the wrong class.
#[test]
fn disabled_twin_with_diverging_content_is_called_out() {
    let text = "Corruption Resistance\tCLASSES:Antipaladin,Paladin=2\tSCHOOL:Abjuration\n\
                #Corruption Resistance\tCLASSES:Antipaladin=2|Inquisitor=2|Paladin=2\tSCHOOL:Abjuration\n";
    let scan = scan_lst("t.lst", text);

    let f: Vec<_> = scan.findings_for(Trap::DisabledLine).collect();
    assert_eq!(f.len(), 1);
    assert!(
        f[0].detail.contains("live record of the same name"),
        "got: {}",
        f[0].detail
    );
    assert!(
        f[0].detail.contains("CLASSES"),
        "the diverging token must be named so the reader knows which row to trust, got: {}",
        f[0].detail
    );
}

/// Trap 3. The display name in field 0 and the `KEY:` can differ. The ACG
/// declares nine `Summon Nature's Ally N` rows whose keys are
/// `Naturalist Summon Nature's Ally N` — genuinely different records from
/// the Core spells of the same display name, with their own duration
/// formula.
#[test]
fn key_differing_from_display_name_is_reported() {
    let text = "Summon Nature's Ally I\tKEY:Naturalist Summon Nature's Ally I\tSCHOOL:Conjuration\n\
                Acid Splash\tKEY:Acid Splash\tSCHOOL:Conjuration\n\
                Magic Missile\tSCHOOL:Evocation\n";
    let scan = scan_lst("t.lst", text);

    let f: Vec<_> = scan.findings_for(Trap::KeyDiffersFromName).collect();
    assert_eq!(f.len(), 1, "only the row whose KEY actually differs");
    assert_eq!(f[0].line, 1);
    assert!(f[0].detail.contains("Naturalist Summon Nature's Ally I"));
    assert!(f[0].detail.contains("Summon Nature's Ally I"));
}

/// Trap 7. `KEY:Rage Power ~ Animal Fury` is namespaced: a bare
/// `KEY:Animal Fury` grep returns zero and looks like the record does not
/// exist. 108 `Rage Power ~` keys exist; `Warpriest Bonus Feat ~` has 484.
#[test]
fn namespaced_keys_are_reported_with_their_namespace() {
    let text = "Animal Fury\tKEY:Rage Power ~ Animal Fury\tCATEGORY:Special Ability\n";
    let scan = scan_lst("t.lst", text);

    let f: Vec<_> = scan.findings_for(Trap::NamespacedKey).collect();
    assert_eq!(f.len(), 1);
    assert_eq!(f[0].detail, "namespace `Rage Power`; leaf `Animal Fury`");
    assert_eq!(scan.namespaces().get("Rage Power"), Some(&1usize));
}

/// Trap 4. An archetype record can look exactly like base-class content.
/// The ACG's only Bloodrager `RagePowersLVL` definition sits on
/// `KEY:Bloodrager Archetype ~ Primalist` (`CATEGORY:Archetype`,
/// `PRECLASS:1,Bloodrager=1`), so crediting base Bloodrager with rage
/// powers because the variable "is in the Bloodrager area" is wrong.
#[test]
fn archetype_scoped_records_are_flagged() {
    let text = "Primalist\tKEY:Bloodrager Archetype ~ Primalist\tCATEGORY:Archetype\tTYPE:Archetype.BloodragerArchetype\tPRECLASS:1,Bloodrager=1\tDEFINE:RagePowersLVL|0\n\
                Rage Powers\tKEY:Skald ~ Rage Powers\tCATEGORY:Special Ability\tTYPE:Skald Class Feature\tDEFINE:RagePowersLVL|0\n";
    let scan = scan_lst("t.lst", text);

    let f: Vec<_> = scan.findings_for(Trap::ArchetypeScoped).collect();
    assert_eq!(f.len(), 1, "only the CATEGORY:Archetype row");
    assert_eq!(f[0].record, "Primalist");
    assert!(
        f[0].detail.contains("Bloodrager"),
        "name the gated class so the reader knows what not to credit, got: {}",
        f[0].detail
    );
}

/// Trap 5. A shared display name is never evidence of a shared thing.
/// Bard's `KEY:Bard ~ Lore Master` and Skald's `KEY:Skald ~ Lore Master`
/// are distinct records; a bare name grep makes the Skald feature look
/// already covered.
#[test]
fn same_display_name_with_distinct_keys_is_reported() {
    let text = "Lore Master\tKEY:Bard ~ Lore Master\tCATEGORY:Special Ability\n\
                Lore Master\tKEY:Skald ~ Lore Master\tCATEGORY:Special Ability\n";
    let scan = scan_lst("t.lst", text);

    let f: Vec<_> = scan.findings_for(Trap::SharedNameDistinctRecords).collect();
    assert_eq!(f.len(), 2, "both sides of the collision are reported");
    assert!(f[0].detail.contains("Skald ~ Lore Master"));
    assert!(f[1].detail.contains("Bard ~ Lore Master"));
}

/// ...but two rows that share a name *and* a key are the same thing
/// restated, not a collision.
#[test]
fn same_display_name_with_the_same_key_is_not_a_collision() {
    let text = "Cackle\tKEY:Witch Hex ~ Cackle\tCATEGORY:Special Ability\n\
                Cackle\tKEY:Witch Hex ~ Cackle\tCATEGORY:Ability Focus\n";
    let scan = scan_lst("t.lst", text);
    assert_eq!(scan.findings_for(Trap::SharedNameDistinctRecords).count(), 0);
}

/// Trap 6. A variable can `DEFINE` to 0 on the record that reads it while
/// its real value arrives from an unconditional `BONUS:VAR` on a
/// different record. `WeaponFocusToHit` is the real case: the
/// `Weapon Focus` feat carries only `DEFINE:WeaponFocusToHit|0`, and the
/// only `BONUS:VAR|WeaponFocusToHit|1|TYPE=Base` in the corpus lives on
/// `CATEGORY=Internal|Default.MOD` in `cr_abilities.lst`.
#[test]
fn define_zero_whose_value_arrives_elsewhere_is_reported() {
    let text = "Weapon Focus\tCATEGORY:FEAT\tTYPE:Combat\tDEFINE:WeaponFocusToHit|0\n\
                CATEGORY=Internal|Default.MOD\tBONUS:VAR|WeaponFocusToHit|1|TYPE=Base\n\
                Self Contained\tDEFINE:LocalVar|0\tBONUS:VAR|LocalVar|3\n";
    let scan = scan_lst("t.lst", text);

    let f: Vec<_> = scan.findings_for(Trap::DefineZeroValueElsewhere).collect();
    assert_eq!(f.len(), 1, "the self-contained record is not a trap");
    assert_eq!(f[0].record, "Weapon Focus");
    assert!(f[0].detail.contains("WeaponFocusToHit"));
    assert!(
        f[0].detail.contains("t.lst:2"),
        "point at the line the value actually comes from, got: {}",
        f[0].detail
    );
}

/// A `DEFINE`-to-0 with no `BONUS:VAR` anywhere in scope is a *different*
/// report: the value comes from outside the scanned file entirely, and
/// the reader has to widen the search rather than read line N.
#[test]
fn define_zero_with_no_bonus_in_scope_says_the_source_is_out_of_scope() {
    let text = "Weapon Focus\tDEFINE:WeaponFocusToHit|0\n";
    let scan = scan_lst("t.lst", text);
    let f: Vec<_> = scan.findings_for(Trap::DefineZeroValueElsewhere).collect();
    assert_eq!(f.len(), 1);
    assert!(
        f[0].detail.contains("no `BONUS:VAR` in this file"),
        "got: {}",
        f[0].detail
    );
}

/// Trap 8. One record can carry many tokens. The corpus maximum is
/// `CATEGORY=Internal|Druid Domain ~ Base.MOD` in `apg_abilities_class.lst`
/// with 69 `BONUS:VAR` tokens on a single line — which is how a "180
/// records" estimate turned out to be 86.
#[test]
fn token_dense_records_report_tokens_and_records_separately() {
    let mut dense = String::from("Druid Domain ~ Base.MOD");
    for i in 0..12 {
        dense.push_str(&format!("\tBONUS:VAR|V{i}|1"));
    }
    let text = format!("{dense}\nSmall Feat\tBONUS:VAR|X|1\tBONUS:VAR|Y|1\n");
    let scan = scan_lst("t.lst", &text);

    assert_eq!(scan.bonus_var_tokens, 14, "14 tokens...");
    assert_eq!(scan.bonus_var_records, 2, "...carried by only 2 records");

    let f: Vec<_> = scan.findings_for(Trap::TokenDenseRecord).collect();
    assert_eq!(f.len(), 1, "only the record above the density threshold");
    assert!(f[0].detail.contains("12"));
}

/// Trap 10. A grep narrowed to `BONUS:`/`PRE:` hides the tokens that
/// govern how the bonus applies. `Spell Mastery` carries `MULT:YES` and a
/// `CHOOSE:`; `acg_feats.lst` has 30 `MULT:YES` and 20 `STACK:YES`.
#[test]
fn governing_tokens_outside_the_common_filter_are_reported() {
    let text = "Spell Mastery\tCATEGORY:FEAT\tPREVARGTEQ:SpellMasteryQualify,1\tMULT:YES\tCHOOSE:SPELLS|CLASSLIST=Wizard\n\
                Toughness\tCATEGORY:FEAT\tBONUS:HP|CURRENTMAX|3\n";
    let scan = scan_lst("t.lst", text);

    let f: Vec<_> = scan.findings_for(Trap::GoverningTokenHiddenByFilter).collect();
    assert_eq!(f.len(), 1, "Toughness carries no governing token");
    assert_eq!(f[0].record, "Spell Mastery");
    assert!(f[0].detail.contains("MULT"));
    assert!(f[0].detail.contains("CHOOSE"));
}

/// Line classification is the foundation everything else rests on, so it
/// gets asserted directly rather than only through the trap counts.
#[test]
fn record_shapes_cover_every_line_kind() {
    let text = "# comment\n\
                \n\
                SOURCELONG:Core\tSOURCESHORT:CR\n\
                Acid Splash\tSCHOOL:Conjuration\n\
                Acid Splash.MOD\tDESC:x\n\
                Acid Splash.COPY=Acid Spray\tDESC:y\n\
                #Acid Splash\tSCHOOL:Conjuration\n\
                CLASSES:Paladin=4\tCLASSES:Paladin=4\n";
    let scan = scan_lst("t.lst", text);
    let shapes: Vec<RecordShape> = scan.lines.iter().map(|l| l.shape).collect();
    assert_eq!(
        shapes,
        vec![
            RecordShape::Comment,
            RecordShape::Blank,
            RecordShape::Directive,
            RecordShape::Declaration,
            RecordShape::Modification,
            RecordShape::Copy,
            RecordShape::Declaration, // shape of the *disabled* row's content
            RecordShape::Continuation,
        ]
    );
    assert!(scan.lines[6].disabled);
    assert!(!scan.lines[3].disabled);
}

/// Trap 9. A count is meaningless without its scope. `concept_census`
/// exists so a per-book subtotal can never be quoted as a corpus total:
/// it always answers with the per-book breakdown, never a bare number.
#[test]
fn concept_census_reports_per_book_never_a_bare_total() {
    let Some(root) = corpus_root() else {
        eprintln!("SKIP concept_census_reports_per_book_never_a_bare_total: no PCGEN_CORPUS_ROOT");
        return;
    };
    let census = concept_census(&books_dir(&root), "WitchHex").expect("census runs");

    assert!(
        census.per_book.len() > 1,
        "witch hexes are spread across books; a single-book answer is the trap"
    );
    let apg = census
        .per_book
        .get("advanced_players_guide")
        .copied()
        .unwrap_or(0);
    assert!(apg > 0);
    assert!(
        census.total > apg,
        "APG subtotal {apg} must not equal the corpus total {}",
        census.total
    );
    assert_eq!(
        census.total,
        census.per_book.values().sum::<usize>(),
        "the total is the sum of the parts it shows its work for"
    );
}

// ===========================================================================
// Corpus invariants — ratchets over already-ingested content
// ===========================================================================

/// **Baselined ratchet, `decisions.md §13`.** SD-33's already-verified,
/// already-out-of-DoD inherited debt (`forward-scope-register.md` D1.1) —
/// "registered, not absorbed": the bar is not "zero", it is "matches the
/// count `docs/governance/corpus-trap-baseline.tsv` registers", checked in
/// both directions exactly the way `pi_sweep_rules_tables` reconciles
/// `pi-sweep-baseline.tsv` (`corpus_trap_baseline::reconcile_trap_count`,
/// mutation-proved below in
/// `the_trap_baseline_reconciler_actually_fires_on_a_planted_regression_and_a_stale_row`).
/// A live count ABOVE the row is a real regression; a live count BELOW it is
/// a stale row that must be updated deliberately, not left to rot. Driving
/// this population to zero is `AT-34-E1-008`'s scope, not this test's.
#[test]
fn no_ingested_record_is_sourced_from_a_disabled_line() {
    let Some(root) = corpus_root() else {
        eprintln!("SKIP no_ingested_record_is_sourced_from_a_disabled_line: no PCGEN_CORPUS_ROOT");
        return;
    };
    let findings = audit_ingested_cache(&cache_dir(), &root).expect("cache audit runs");
    let violations: Vec<_> = findings
        .iter()
        .filter(|f| f.trap == Trap::DisabledLine)
        .collect();
    let baseline = load_trap_baseline();
    let verdict = reconcile_trap_count(Trap::DisabledLine.id(), violations.len(), &baseline);
    assert!(
        verdict.is_matched(),
        "{}\nfindings ({}): {violations:#?}",
        verdict.explain(),
        violations.len()
    );
}

/// **Invariant.** Every ingested record's cited line must exist and must
/// resolve. A citation that points past the end of its file, or at a
/// blank line, is a broken provenance chain.
#[test]
fn every_ingested_citation_resolves_to_a_real_line() {
    let Some(root) = corpus_root() else {
        eprintln!("SKIP every_ingested_citation_resolves_to_a_real_line: no PCGEN_CORPUS_ROOT");
        return;
    };
    let findings = audit_ingested_cache(&cache_dir(), &root).expect("cache audit runs");
    let violations: Vec<_> = findings
        .iter()
        .filter(|f| f.trap == Trap::UnresolvableCitation)
        .collect();
    assert!(
        violations.is_empty(),
        "ingested records cite lines that do not resolve: {violations:#?}"
    );
}

/// **Baselined ratchet, `decisions.md §13`.** No two ingested records within
/// the same book and kind may share a `record_key` *beyond the registered
/// baseline* — distinct records must be distinguishable by key, which is
/// exactly what trap 3 and trap 5 attack, but this population is SD-33's
/// already-verified, already-out-of-DoD inherited debt
/// (`forward-scope-register.md` D1.1), "registered, not absorbed." Checked
/// both directions against `docs/governance/corpus-trap-baseline.tsv`, the
/// same way `pi_sweep_rules_tables` reconciles `pi-sweep-baseline.tsv`. A
/// live count ABOVE the row is a real regression; BELOW it is a stale row.
#[test]
fn no_two_ingested_records_share_a_record_key() {
    let Some(root) = corpus_root() else {
        eprintln!("SKIP no_two_ingested_records_share_a_record_key: no PCGEN_CORPUS_ROOT");
        return;
    };
    let findings = audit_ingested_cache(&cache_dir(), &root).expect("cache audit runs");
    let violations: Vec<_> = findings
        .iter()
        .filter(|f| f.trap == Trap::SharedNameDistinctRecords)
        .collect();
    let baseline = load_trap_baseline();
    let verdict = reconcile_trap_count(Trap::SharedNameDistinctRecords.id(), violations.len(), &baseline);
    assert!(
        verdict.is_matched(),
        "{}\nfindings ({}): {violations:#?}",
        verdict.explain(),
        violations.len()
    );
}

/// **Ratchet, debt paid to zero.** An ingested record's `record_key`
/// must equal the `KEY:` on the corpus line it cites.
///
/// Nine ACG spells used to violate this: the ACG ingest stored the
/// display name `Summon Nature's Ally N` for lines whose `KEY:` is
/// `Naturalist Summon Nature's Ally N` — trap 3 realised in shipped
/// cache data, with the Naturalist archetype's variant spell filed
/// under the Core spell's identity. That has been fixed: the ACG spell
/// cache now resolves those nine rows via their own `KEY:` field
/// (`src/rules_core/cache_gen/acg.rs`) and is re-keyed to
/// `Naturalist Summon Nature's Ally N` on disk, so the known-debt list
/// below is empty.
///
/// The assertion stays a ratchet, not a one-time pass: the allowlist is
/// enumerated (currently empty) so *any* key mismatch anywhere in any
/// book — including a regression of the nine rows just fixed — fails.
/// The debt cannot silently grow back, and it cannot hide.
///
/// **Beyond that narrow, already-paid-to-zero allowlist**, the live corpus
/// carries a much larger, separately-registered population of this same
/// trap kind: SD-33's already-verified, already-out-of-DoD inherited debt
/// (`decisions.md §13`, `forward-scope-register.md` D1.1), "registered, not
/// absorbed." That population is checked against
/// `docs/governance/corpus-trap-baseline.tsv` in both directions, the same
/// way `pi_sweep_rules_tables` reconciles `pi-sweep-baseline.tsv` — a live
/// count above the row is a real regression, below it is a stale row.
#[test]
fn ingested_record_keys_match_their_cited_line() {
    let Some(root) = corpus_root() else {
        eprintln!("SKIP ingested_record_keys_match_their_cited_line...: no PCGEN_CORPUS_ROOT");
        return;
    };

    // Known outstanding debt, enumerated so it cannot silently grow.
    // Each entry is (book, cached record_key). The ACG Naturalist debt (9
    // rows) was paid off by re-keying to the archetype-qualified KEY: —
    // leave this mechanism in place rather than deleting it, since the
    // next mis-keyed ingest, in ACG or any other book, must be added here
    // deliberately, not silently absorbed.
    //
    // SD31-W5-INTEGRATE-001 (2026-08-16) landed 10 rows here (the
    // `OPEN-ISSUES.md` row 90/92 class-ability-cited subset of a 50-record
    // `find_citation` mis-citation finding — re-derived to 39 records at
    // the time of the fix below, not 50; the other 40 was itself an
    // over-count carried in the original finding's own prose). `find_citation`
    // tried a `KEY:`-field match across EVERY `.lst` in the book before a
    // first-column match in ANY file, so a proficiency/class-ability row's
    // `KEY:` could beat the real equipment row.
    //
    // `SD31-E6-F5-004` fixed the root cause (`equipment_gap.rs`'s
    // `find_citation` now tries every strategy against equipment-shaped
    // files — basename containing `"equip"` — before ever falling back to
    // a non-equipment-shaped file) and re-cited all 39 affected records
    // (`gen_cache_hand_authored_equipment` re-run after deleting the 39
    // wrong-citation files so the no-clobber write guard could not block
    // the correction). All 10 of these rows now cite their real equipment
    // row and no longer trip this trap — debt paid to zero, matching the
    // ACG Naturalist precedent. Left empty, not deleted, so the next
    // mis-keyed ingest is still caught and must be enumerated deliberately.
    const KNOWN_KEY_MISMATCH_DEBT: &[(&str, &str)] = &[];

    let findings = audit_ingested_cache(&cache_dir(), &root).expect("cache audit runs");
    let mismatches: Vec<_> = findings
        .iter()
        .filter(|f| f.trap == Trap::KeyDiffersFromName)
        .collect();

    let unexpected: Vec<_> = mismatches
        .iter()
        .filter(|f| {
            !KNOWN_KEY_MISMATCH_DEBT
                .iter()
                .any(|(book, key)| f.file.contains(book) && f.record == *key)
        })
        .collect();

    // Beyond the ACG-Naturalist/equipment-citation debt this file already paid
    // to zero (KNOWN_KEY_MISMATCH_DEBT above — still enforced: any regression
    // of those specific rows shows up as `unexpected` growing beyond the
    // baseline, since it is not in that allowlist), the remaining population
    // is SD-33's separately-registered inherited debt (`decisions.md §13`).
    // Baselined, not zero-toleranced, checked both directions.
    let baseline = load_trap_baseline();
    let verdict = reconcile_trap_count(Trap::KeyDiffersFromName.id(), unexpected.len(), &baseline);
    assert!(
        verdict.is_matched(),
        "{}\nunexpected findings ({} of {} total, {} already paid to zero and enumerated \
         above): {unexpected:#?}",
        verdict.explain(),
        unexpected.len(),
        mismatches.len(),
        KNOWN_KEY_MISMATCH_DEBT.len(),
    );
    for f in &mismatches {
        assert_eq!(f.severity, Severity::Defect, "a mis-keyed ingest is a defect");
    }
}

/// **Invariant, stated as the thing it is not.** 272 ingested records
/// cite a `.MOD` line, and that is *correct*: PCGen splits a spell across
/// a declaring row and one or more `.MOD` rows carrying `DESC:`/`ITEM:`,
/// and the ingest cites the row the field it took actually lives on.
///
/// The defect trap 1 describes is counting a `.MOD` as a *declaration*.
/// The checkable form of that is: every `.MOD`-cited ingested record must
/// have a live base declaration of the same name in the same file. If it
/// does not, the ingest manufactured a record out of a modification.
///
/// **Baselined ratchet, `decisions.md §13`.** The population of such orphaned
/// records is SD-33's already-verified, already-out-of-DoD inherited debt
/// (`forward-scope-register.md` D1.1), "registered, not absorbed" — checked
/// both directions against `docs/governance/corpus-trap-baseline.tsv`, the
/// same way `pi_sweep_rules_tables` reconciles `pi-sweep-baseline.tsv`. A
/// live count ABOVE the row is a real regression; BELOW it is a stale row.
/// Driving this population to zero is `AT-34-E1-008`'s scope, not this
/// test's — its own bar is `wiring-class-mismatch = 0`, a different trap.
#[test]
fn every_mod_sourced_ingest_has_a_live_base_declaration() {
    let Some(root) = corpus_root() else {
        eprintln!("SKIP every_mod_sourced_ingest_has_a_live_base_declaration: no PCGEN_CORPUS_ROOT");
        return;
    };
    let findings = audit_ingested_cache(&cache_dir(), &root).expect("cache audit runs");
    let orphans: Vec<_> = findings
        .iter()
        .filter(|f| f.trap == Trap::ModRecord && f.severity == Severity::Defect)
        .collect();
    let baseline = load_trap_baseline();
    let verdict = reconcile_trap_count(Trap::ModRecord.id(), orphans.len(), &baseline);
    assert!(
        verdict.is_matched(),
        "{}\nfindings ({}): {orphans:#?}",
        verdict.explain(),
        orphans.len()
    );
}

/// **Mutation proof, modelled on
/// `sd30_declared_product_identity_in_shipped_class_features::the_leak_
/// detectors_actually_fire_on_a_planted_leak_and_clear_on_a_redacted_row`.**
/// The four baselined tests above only prove the reconciliation mechanism
/// PASSES against today's live population — that alone is exactly the
/// "cannot fail, so it proves nothing when it passes" trap this repo has
/// been burned by before. This proves the mechanism can also FAIL, in both
/// directions `decisions.md §13` and `pi_table_sweep::reconcile` require it
/// to catch, independent of whatever the live corpus currently contains: a
/// synthetic baseline, never touching `data/corpus/**` or the real
/// `docs/governance/corpus-trap-baseline.tsv`, is checked against a planted
/// regression, a planted stale row, and an exact match. Runs unconditionally
/// — no `PCGEN_CORPUS_ROOT` needed, exactly like `sd30`'s own proof.
#[test]
fn the_trap_baseline_reconciler_actually_fires_on_a_planted_regression_and_a_stale_row() {
    let synthetic_baseline_text = "mod-record\t2117\tsynthetic baseline, this proof only\n";
    let baseline = codex::pcgen_import::corpus_trap_baseline::parse_baseline(synthetic_baseline_text)
        .expect("synthetic baseline parses");

    // 1. A planted regression — one MORE live finding than the baseline
    //    records — must be flagged, not silently accepted.
    let regressed = reconcile_trap_count("mod-record", 2118, &baseline);
    assert!(
        !regressed.is_matched(),
        "the baseline reconciler failed to flag a planted regression (2118 vs baseline 2117) \
         — this instrument cannot fail, which means it proves nothing when it passes"
    );

    // 2. A planted stale row — one FEWER live finding than the baseline
    //    records (debt paid down without the row being updated) — must also
    //    be flagged, not treated as a free pass.
    let stale = reconcile_trap_count("mod-record", 2116, &baseline);
    assert!(
        !stale.is_matched(),
        "the baseline reconciler failed to flag a stale row (2116 vs baseline 2117) — a \
         baseline that only catches increases would let debt paid down silently rot the file"
    );

    // 3. The same trap, exactly at the baseline — must clear (the gate goes
    //    green once the count genuinely matches, not stay permanently red).
    let matched = reconcile_trap_count("mod-record", 2117, &baseline);
    assert!(matched.is_matched(), "an exact match must clear the gate");

    // 4. A trap id with no row at all — must be flagged, never a silent
    //    zero-tolerance default.
    let missing = reconcile_trap_count("disabled-line", 0, &baseline);
    assert!(!missing.is_matched(), "a trap id absent from the baseline must not silently pass");
}

// ===========================================================================
// Cold run — the scanner must work on a book nobody has ingested
// ===========================================================================

/// The whole point is that an agent starting book #5 gets the trap report
/// *before* writing ingest code. Ultimate Combat has never been ingested,
/// so this is the honest cold test.
#[test]
fn scanner_runs_cold_against_an_uningested_book() {
    let Some(root) = corpus_root() else {
        eprintln!("SKIP scanner_runs_cold_against_an_uningested_book: no PCGEN_CORPUS_ROOT");
        return;
    };
    let book = books_dir(&root).join("ultimate_combat");
    let scan = codex::pcgen_import::corpus_traps::scan_book(&book).expect("cold scan runs");

    assert!(scan.files.len() > 5, "ultimate_combat has many .lst files");
    assert!(scan.declaring_lines() > 100);
    assert!(scan.modifications() > 0, "UC carries .MOD records");

    // Every trap that is a *shape* rather than a coincidence must fire on
    // a book this size; if one silently reports zero the detector is
    // broken, not the book.
    for trap in [
        Trap::ModRecord,
        Trap::DisabledLine,
        Trap::NamespacedKey,
        Trap::KeyDiffersFromName,
        Trap::GoverningTokenHiddenByFilter,
    ] {
        assert!(
            scan.count_for(trap) > 0,
            "{trap:?} reported zero findings across ultimate_combat, which would mean \
             the detector is not wired, not that the book is clean"
        );
    }

    // Nothing in a cold scan of upstream data is a defect: it is all
    // legitimate corpus shape the ingest must handle.
    assert!(
        scan.findings().all(|f| f.severity == Severity::Trap),
        "a cold corpus scan must never claim upstream data is defective"
    );
}

/// Pinned counts for the four ingested books. These are regression
/// anchors: the corpus is a read-only upstream clone, so these numbers
/// only move if the scanner's classification changes.
#[test]
fn ingested_book_scans_match_pinned_counts() {
    let Some(root) = corpus_root() else {
        eprintln!("SKIP ingested_book_scans_match_pinned_counts: no PCGEN_CORPUS_ROOT");
        return;
    };
    let dir = books_dir(&root);

    // ACG spell file: 144 live declarations, 0 `.COPY=`. The ACG spell
    // cache holds exactly 144 records — the scanner's declaration rule
    // reproduces what the ingest actually did.
    let acg_spells = scan_lst_file(&dir.join("advanced_class_guide/acg_spells.lst"));
    assert_eq!(acg_spells.declarations, 144);
    assert_eq!(acg_spells.copies, 0);
    // 598 *live* `.MOD` rows. A `grep -c` on the first field returns 611,
    // because 13 of them are `#`-disabled — which is trap 1 and trap 2
    // compounding into one number.
    assert_eq!(acg_spells.modifications, 598);
    assert_eq!(acg_spells.disabled_records, 13);

    // Trap 3 at exactly the reported scale: 9 spell rows in the ACG and 9
    // in the APG carry a `KEY:` that differs from their display name, for
    // the 18 cross-book collisions the incident reported. All 18 are flat
    // renames — the dangerous shape, invisible to a name-based join.
    let apg_spells = scan_lst_file(&dir.join("advanced_players_guide/apg_spells.lst"));
    assert_eq!(acg_spells.count_for(Trap::KeyDiffersFromName), 9);
    assert_eq!(apg_spells.count_for(Trap::KeyDiffersFromName), 9);
    assert!(
        acg_spells
            .findings_for(Trap::KeyDiffersFromName)
            .all(|f| f.detail.contains("flat rename")),
        "all nine ACG mismatches are flat renames, not namespaced keys"
    );

    // Trap 11 at the scale that matters: `cr_equip_magic_items.lst`
    // declares 1150 records via `.COPY=` and *zero* any other way. An
    // equipment inventory that skips `.COPY=` finds no magic items at all.
    let crb_items = scan_lst_file(&dir.join("core_rulebook/cr_equip_magic_items.lst"));
    assert_eq!(crb_items.copies, 1150);
    assert_eq!(
        crb_items.declaring_lines(),
        crb_items.copies + crb_items.declarations
    );
    assert!(
        crb_items.copies > crb_items.declarations,
        "most CRB magic items exist only as `.COPY=` rows"
    );

    // APG feats + ACG feats: 76 live `.MOD` records between them — the
    // exact overcount that inflated a feat estimate by ~30%.
    let apg_feats = scan_lst_file(&dir.join("advanced_players_guide/apg_feats.lst"));
    let acg_feats = scan_lst_file(&dir.join("advanced_class_guide/acg_feats.lst"));
    assert_eq!(
        apg_feats.modifications + acg_feats.modifications,
        76,
        "the two feat files carry 76 live .MOD records"
    );

    // CRB feats: a naive `grep -c '\.MOD'` returns 4; only 3 lines are
    // `.MOD` records and only 2 of those are live. (The fourth hit is
    // `Spell Mastery`'s `SELECT:var("STAT.3.MOD.NOEQUIP.NOTEMP")`.)
    let crb_feats = scan_lst_file(&dir.join("core_rulebook/cr_feats.lst"));
    assert_eq!(crb_feats.modifications, 2);
    // `grep -c '^#'` returns 18 here, but 17 of those are prose: file
    // headers, `###Block:` separators, tab-aligned column legends, and
    // two disabled *fragments* whose first field is empty or a bare
    // token. Exactly one is a suppressed record (`#Arcane Strike.MOD`).
    assert_eq!(crb_feats.disabled_records, 1);
    assert_eq!(crb_feats.declarations, 195);

    // Bestiary 1 is ingested as monsters; it must scan without error and
    // report real structure.
    let b1 = codex::pcgen_import::corpus_traps::scan_book(&dir.join("bestiary"))
        .expect("bestiary 1 scans");
    assert!(b1.declaring_lines() > 100);
}

fn scan_lst_file(path: &Path) -> codex::pcgen_import::corpus_traps::FileScan {
    let text = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    scan_lst(&path.display().to_string(), &text)
}
