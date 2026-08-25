//! SD-33 remediation wave 6 (`eqm-modifier-final` lane, `AT-33-E5-002`
//! remainder) -- repo-local "ours" batch probe for this lane's 6
//! `equipment_modifier` units (the 7th, `ultimate_combat:equipment:
//! arrow_iron_tipped_distance_20`, is `unverifiable`/
//! `no_comparable_export_token` -- see this cycle's receipt -- and has no
//! "ours" row here since no comparable oracle value exists to compare
//! against).
//!
//! Real live calls into `codex::rules_core::equipment_effects::general::
//! compute_var_effect` (the `EQMARMOR`-material shape's own chain, read
//! directly off the modifier record -- no host needed, mirroring the
//! live oracle's clean isolated fixture where the ONLY source of
//! `VAR.ArmorCheckPenalty` on the whole character is this modifier),
//! `codex::rules_core::damage_total::resolve_eqmweapon_damagesize_effect`
//! (new this cycle -- see that module's own doc comment), and
//! `codex::rules_core::equipment_effects::resolve_eqm_weightdiv_effect`
//! (new this cycle) -- against a synthetic host record for each of the
//! two shapes that need one, built to the SAME literal stats as this
//! cycle's live-oracle `.pcg`/`.lst` fixtures
//! (`scripts/oracle_harness/eqm-fixtures/sd33r6_eqm_items.lst`), fed
//! through the real `parse_equipment_entries`/`convert_equipment_record`
//! pipeline every other probe in this bundle uses -- no stubs, no
//! fixture-only values, no hand-derived duplicate of the corpus's own
//! literal tokens.
//!
//! Usage:
//!   e5_eqm_final_ours <repo_root> <output.json>
//!
//! Output: `{"unit_id": {"ours": <value>, "kind": <comparable-magnitude
//! label>}, ...}`.

use codex::pcgen_import::ir_converter::convert_equipment_record;
use codex::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
use codex::rules_core::corpus_loader::{load_equipment_corpus, BookCorpusRoot};
use codex::rules_core::damage_total::resolve_eqmweapon_damagesize_effect;
use codex::rules_core::equipment_effects::general::compute_var_effect;
use codex::rules_core::equipment_effects::resolve_eqm_weightdiv_effect;
use codex::rules_core::equipment_resolver::equipment_id_resolve;
use codex::rules_core::rules_tables::RuleSetId;
use serde_json::json;
use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: e5_eqm_final_ours <repo_root> <output.json>");
        return ExitCode::from(2);
    }
    let repo_root = Path::new(&args[1]);
    let output_path = &args[2];

    let books = ["core_rulebook", "advanced_race_guide"];
    let book_dirs: Vec<std::path::PathBuf> =
        books.iter().map(|b| repo_root.join("data/corpus").join(b)).collect();
    let roots: Vec<BookCorpusRoot> = books
        .iter()
        .zip(book_dirs.iter())
        .map(|(book_id, dir)| BookCorpusRoot { book_id, dir })
        .collect();
    let mut corpus = load_equipment_corpus(&roots);

    // Synthetic hosts, literally the same tokens as
    // `scripts/oracle_harness/eqm-fixtures/sd33r6_eqm_items.lst` (the
    // live-oracle fixture this cycle authored and ran), so "ours" and the
    // oracle measure the identical host.
    let host_text = "\
SD33R6 Shield Spiked\tKEY:SD33R6 ~ Shield Spiked\tTYPE:Shield.Heavy.Weapon.Resizable.Melee.ShieldBash.Close.Weapon Group Close.Nonmetal\tCOST:7\tWT:10\tACCHECK:-2\tCRITMULT:x2\tCRITRANGE:1\tDAMAGE:1d4\tWIELD:OneHanded\tSIZE:M\tSPELLFAILURE:15\tEQMOD:Special Quality ~ Spikes ~ Shieldbash\n\
SD33R6 Outfit Darkleaf\tKEY:SD33R6 ~ Outfit Darkleaf\tTYPE:Goods.Clothing.Resizable.Starting\tCOST:10\tWT:8\tEQMOD:Material ~ Darkleaf Cloth ~ Clothing\n";
    let parsed = parse_equipment_entries("sd33r6_eqm_items.lst", host_text);
    assert!(parsed.diagnostics.is_empty(), "synthetic host fixture must parse cleanly: {:?}", parsed.diagnostics);
    for record in parsed.entries {
        let record: &'static EquipmentRecord = Box::leak(Box::new(record));
        corpus.push(convert_equipment_record(record));
    }

    let mut out = serde_json::Map::new();

    // EQMARMOR-material shape: draco / dragonhide / material_dragonhide
    // all alias to the SAME real modifier (`KEY:Material ~ Dragonhide`),
    // confirmed by direct corpus-record read this cycle. Each unit_id's
    // own corpus_key is resolved independently (proves the OUTPUTNAME-
    // divergent-identity fix from wave 5 holds for all three aliases),
    // and each reads the modifier's own `BONUS:VAR|ArmorCheckPenalty`
    // chain directly -- no host needed, since this bonus lives on the
    // modifier record itself, not on a base item.
    let var_units: &[(&str, &str)] = &[
        ("core_rulebook:equipment_modifier:draco", "DRACO"),
        ("core_rulebook:equipment_modifier:dragonhide", "Dragonhide"),
        ("core_rulebook:equipment_modifier:material_dragonhide", "Material ~ Dragonhide"),
    ];
    for (unit_id, key) in var_units {
        if let Some((record, _)) = equipment_id_resolve(key, RuleSetId::Crb, &corpus) {
            let effect = compute_var_effect(record);
            let acp = effect.iter().find(|vb| vb.name == "ArmorCheckPenalty").map(|vb| vb.bonus);
            out.insert(
                (*unit_id).to_string(),
                json!({"ours": acp, "kind": "var_armor_check_penalty"}),
            );
        } else {
            out.insert((*unit_id).to_string(), json!({"ours": null, "kind": "unresolved"}));
        }
    }

    // EQMWEAPON|DAMAGESIZE shape: spike_sb / special_quality_spikes_
    // shieldbash both alias to `KEY:Special Quality ~ Spikes ~
    // Shieldbash`. "ours" is the stepped die on the synthetic shield
    // host above.
    for unit_id in [
        "core_rulebook:equipment_modifier:spike_sb",
        "core_rulebook:equipment_modifier:special_quality_spikes_shieldbash",
    ] {
        let stepped = resolve_eqmweapon_damagesize_effect("SD33R6 ~ Shield Spiked", &corpus);
        let die_str = stepped.map(|d| format!("{}d{}", d.count, d.die_size));
        out.insert(unit_id.to_string(), json!({"ours": die_str, "kind": "damage_die"}));
    }

    // EQM|WEIGHTDIV shape: material_darkleaf_cloth_clothing, on the
    // synthetic outfit host above.
    let weight = resolve_eqm_weightdiv_effect("SD33R6 ~ Outfit Darkleaf", &corpus);
    out.insert(
        "advanced_race_guide:equipment_modifier:material_darkleaf_cloth_clothing".to_string(),
        json!({"ours": weight, "kind": "weight_lbs"}),
    );

    let serialized = serde_json::to_string_pretty(&out).unwrap();
    if let Err(e) = fs::write(output_path, serialized) {
        eprintln!("failed to write {output_path}: {e}");
        return ExitCode::from(1);
    }

    println!("e5_eqm_final_ours: {} units resolved -> {}", out.len(), output_path);
    ExitCode::SUCCESS
}
