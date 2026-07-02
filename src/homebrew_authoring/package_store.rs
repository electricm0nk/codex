//! Deterministic file persistence for the first bounded GE-08 authored package.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use super::{
    EffectRecord, FeatRecord, PackageDiagnostic, PackageDiagnosticSeverity, PrerequisiteRecord,
    ProvenanceEntry, SourcePackage,
};
use crate::homebrew_authoring::package_manifest::{
    PackageManifest, PackageValidationState, ProofBinding,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageDiff {
    pub changed_files: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageStoreError {
    pub message: String,
}

impl std::fmt::Display for PackageStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for PackageStoreError {}

pub struct PackageStore;

impl PackageStore {
    pub fn save(package: &SourcePackage, root: &Path) -> Result<(), PackageStoreError> {
        let normalized = package.normalized_for_persistence();
        validate_persistable(&normalized)?;
        fs::create_dir_all(root).map_err(|err| io_error(root, err))?;
        ensure_layout(root)?;
        clear_known_record_files(root)?;

        for (relative_path, content) in file_map(&normalized) {
            let target = root.join(&relative_path);
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent).map_err(|err| io_error(parent, err))?;
            }
            fs::write(&target, content).map_err(|err| io_error(&target, err))?;
        }

        Ok(())
    }

    pub fn load(root: &Path) -> Result<SourcePackage, PackageStoreError> {
        let manifest = parse_manifest(&read_required(root.join("manifest.yaml"))?)?;
        let feat = read_optional_single_record(root.join("objects/feats"), parse_feat_record)?;
        let effect = read_optional_single_record(root.join("rules/effects"), parse_effect_record)?;
        let prerequisite =
            read_optional_single_record(root.join("rules/prerequisites"), parse_prerequisite_record)?;
        let provenance = parse_provenance(&read_required(root.join("metadata/provenance.yaml"))?)?;
        let diagnostics = parse_diagnostics(&read_required(root.join("metadata/diagnostics.yaml"))?)?;

        Ok(SourcePackage {
            manifest,
            feat,
            effect,
            prerequisite,
            provenance,
            diagnostics,
        }
        .normalized_for_persistence())
    }

    pub fn diff(left: &SourcePackage, right: &SourcePackage) -> PackageDiff {
        let left_map = file_map(&left.normalized_for_persistence());
        let right_map = file_map(&right.normalized_for_persistence());

        let mut changed_files = Vec::new();
        let mut keys = left_map.keys().cloned().collect::<Vec<_>>();
        for key in right_map.keys() {
            if !keys.iter().any(|existing| existing == key) {
                keys.push(key.clone());
            }
        }
        keys.sort();
        keys.dedup();

        for key in keys {
            if left_map.get(&key) != right_map.get(&key) {
                changed_files.push(key);
            }
        }

        PackageDiff { changed_files }
    }

    pub fn diff_roots(left_root: &Path, right_root: &Path) -> Result<PackageDiff, PackageStoreError> {
        let left = Self::load(left_root)?;
        let right = Self::load(right_root)?;
        Ok(Self::diff(&left, &right))
    }

    pub fn export(package: &SourcePackage, destination: &Path) -> Result<(), PackageStoreError> {
        let normalized = package.normalized_for_persistence();
        if normalized.manifest.validation_state != PackageValidationState::Valid {
            return Err(PackageStoreError {
                message: format!(
                    "package export refused: validation_state is {}",
                    normalized.manifest.validation_state.as_str()
                ),
            });
        }
        Self::save(&normalized, destination)
    }
}

fn ensure_layout(root: &Path) -> Result<(), PackageStoreError> {
    for relative in [
        "objects/feats",
        "rules/effects",
        "rules/prerequisites",
        "metadata",
    ] {
        let path = root.join(relative);
        fs::create_dir_all(&path).map_err(|err| io_error(&path, err))?;
    }
    Ok(())
}

fn clear_known_record_files(root: &Path) -> Result<(), PackageStoreError> {
    for relative in ["objects/feats", "rules/effects", "rules/prerequisites"] {
        let directory = root.join(relative);
        if !directory.exists() {
            continue;
        }
        for entry in fs::read_dir(&directory).map_err(|err| io_error(&directory, err))? {
            let entry = entry.map_err(|err| io_error(&directory, err))?;
            let path = entry.path();
            if path.is_file() {
                fs::remove_file(&path).map_err(|err| io_error(&path, err))?;
            }
        }
    }
    Ok(())
}

fn file_map(package: &SourcePackage) -> BTreeMap<String, String> {
    let mut files = BTreeMap::new();
    files.insert("manifest.yaml".to_owned(), render_manifest(&package.manifest));
    files.insert(
        "metadata/provenance.yaml".to_owned(),
        render_provenance(&package.provenance),
    );
    files.insert(
        "metadata/diagnostics.yaml".to_owned(),
        render_diagnostics(&package.diagnostics),
    );

    if let Some(feat) = &package.feat {
        files.insert(
            format!("objects/feats/{}.yaml", feat.stable_id),
            render_feat_record(feat),
        );
    }
    if let Some(effect) = &package.effect {
        files.insert(
            format!("rules/effects/{}.yaml", effect.stable_id),
            render_effect_record(effect),
        );
    }
    if let Some(prerequisite) = &package.prerequisite {
        files.insert(
            format!("rules/prerequisites/{}.yaml", prerequisite.stable_id),
            render_prerequisite_record(prerequisite),
        );
    }

    files
}

/// Rejects a package whose rendered `key: value` lines the loader could not
/// read back. `save()` deliberately persists Draft/Invalid packages, so an
/// empty or multi-line string value would leave an unloadable bundle on disk
/// with no warning at write time.
fn validate_persistable(package: &SourcePackage) -> Result<(), PackageStoreError> {
    let field = |name: &str, value: &str| -> Result<(), PackageStoreError> {
        if value.contains('\n') || value.contains('\r') {
            return Err(parse_error(format!(
                "{name} contains a newline/carriage return and cannot be persisted"
            )));
        }
        if value.trim().is_empty() {
            return Err(parse_error(format!(
                "{name} is empty and would not survive a save/load round trip"
            )));
        }
        Ok(())
    };

    let manifest = &package.manifest;
    field("manifest package_id", &manifest.package_id)?;
    field("manifest package_title", &manifest.package_title)?;
    field("manifest game_system_id", &manifest.game_system_id)?;
    field("manifest package_version", &manifest.package_version)?;
    for dependency in &manifest.depends_on {
        field("manifest depends_on entry", dependency)?;
    }
    for kind in &manifest.supported_object_kinds {
        field("manifest supported_object_kinds entry", kind)?;
    }
    field("manifest provenance_policy", &manifest.provenance_policy)?;
    field("manifest proof_binding case_id", &manifest.proof_binding.case_id)?;
    field("manifest proof_binding slot", &manifest.proof_binding.slot)?;
    field("manifest proof_binding remove", &manifest.proof_binding.remove)?;
    field("manifest proof_binding add", &manifest.proof_binding.add)?;

    if let Some(feat) = &package.feat {
        field("feat stable_id", &feat.stable_id)?;
        field("feat package_id", &feat.package_id)?;
        field("feat display_name", &feat.display_name)?;
        field("feat object_kind", &feat.object_kind)?;
        field("feat substitutes_for", &feat.substitutes_for)?;
        for effect_id in &feat.effect_ids {
            field("feat effect_ids entry", effect_id)?;
        }
        for prerequisite_id in &feat.prerequisite_ids {
            field("feat prerequisite_ids entry", prerequisite_id)?;
        }
        field("feat semantic_intent", &feat.semantic_intent)?;
    }

    if let Some(effect) = &package.effect {
        field("effect stable_id", &effect.stable_id)?;
        field("effect owning_feat_id", &effect.owning_feat_id)?;
        field("effect target_family", &effect.target_family)?;
        field("effect modifier_type", &effect.modifier_type)?;
        field("effect stacking_posture", &effect.stacking_posture)?;
    }

    if let Some(prerequisite) = &package.prerequisite {
        field("prerequisite stable_id", &prerequisite.stable_id)?;
        field("prerequisite owning_feat_id", &prerequisite.owning_feat_id)?;
        field("prerequisite predicate", &prerequisite.predicate)?;
    }

    for entry in &package.provenance {
        field("provenance stable_id", &entry.stable_id)?;
        field("provenance source_package_id", &entry.source_package_id)?;
        field("provenance canonical_target_id", &entry.canonical_target_id)?;
        field("provenance canonical_target_field", &entry.canonical_target_field)?;
        field("provenance authored_path", &entry.authored_path)?;
        field("provenance source_system", &entry.source_system)?;
        field("provenance support", &entry.support)?;
    }

    for diagnostic in &package.diagnostics {
        field("diagnostic class", &diagnostic.class)?;
        field("diagnostic message", &diagnostic.message)?;
        field("diagnostic subject_ref", &diagnostic.subject_ref)?;
    }

    Ok(())
}

fn render_manifest(manifest: &PackageManifest) -> String {
    let mut output = String::new();
    let _ = writeln!(output, "schema_version: {}", manifest.schema_version);
    let _ = writeln!(output, "package_id: {}", manifest.package_id);
    let _ = writeln!(output, "package_title: {}", manifest.package_title);
    let _ = writeln!(output, "game_system_id: {}", manifest.game_system_id);
    let _ = writeln!(output, "package_version: {}", manifest.package_version);
    output.push_str("depends_on:\n");
    for dependency in &manifest.depends_on {
        let _ = writeln!(output, "  - {dependency}");
    }
    output.push_str("supported_object_kinds:\n");
    for kind in &manifest.supported_object_kinds {
        let _ = writeln!(output, "  - {kind}");
    }
    let _ = writeln!(
        output,
        "validation_state: {}",
        manifest.validation_state.as_str()
    );
    let _ = writeln!(output, "provenance_policy: {}", manifest.provenance_policy);
    output.push_str("proof_binding:\n");
    let _ = writeln!(output, "  case_id: {}", manifest.proof_binding.case_id);
    let _ = writeln!(output, "  slot: {}", manifest.proof_binding.slot);
    let _ = writeln!(output, "  remove: {}", manifest.proof_binding.remove);
    let _ = writeln!(output, "  add: {}", manifest.proof_binding.add);
    output
}

fn render_feat_record(feat: &FeatRecord) -> String {
    let mut output = String::new();
    let _ = writeln!(output, "stable_id: {}", feat.stable_id);
    let _ = writeln!(output, "package_id: {}", feat.package_id);
    let _ = writeln!(output, "display_name: {}", feat.display_name);
    let _ = writeln!(output, "object_kind: {}", feat.object_kind);
    let _ = writeln!(output, "substitutes_for: {}", feat.substitutes_for);
    output.push_str("effect_ids:\n");
    for effect_id in &feat.effect_ids {
        let _ = writeln!(output, "  - {effect_id}");
    }
    output.push_str("prerequisite_ids:\n");
    for prerequisite_id in &feat.prerequisite_ids {
        let _ = writeln!(output, "  - {prerequisite_id}");
    }
    let _ = writeln!(output, "semantic_intent: {}", feat.semantic_intent);
    output
}

fn render_effect_record(effect: &EffectRecord) -> String {
    let mut output = String::new();
    let _ = writeln!(output, "stable_id: {}", effect.stable_id);
    let _ = writeln!(output, "owning_feat_id: {}", effect.owning_feat_id);
    let _ = writeln!(output, "target_family: {}", effect.target_family);
    let _ = writeln!(output, "modifier_type: {}", effect.modifier_type);
    let _ = writeln!(output, "modifier_value: {}", effect.modifier_value);
    let _ = writeln!(output, "stacking_posture: {}", effect.stacking_posture);
    output
}

fn render_prerequisite_record(prerequisite: &PrerequisiteRecord) -> String {
    let mut output = String::new();
    let _ = writeln!(output, "stable_id: {}", prerequisite.stable_id);
    let _ = writeln!(output, "owning_feat_id: {}", prerequisite.owning_feat_id);
    let _ = writeln!(output, "predicate: {}", prerequisite.predicate);
    output
}

fn render_provenance(entries: &[ProvenanceEntry]) -> String {
    let mut output = String::from("entries:\n");
    for entry in entries {
        let _ = writeln!(output, "  - stable_id: {}", entry.stable_id);
        let _ = writeln!(output, "    source_package_id: {}", entry.source_package_id);
        let _ = writeln!(output, "    canonical_target_id: {}", entry.canonical_target_id);
        let _ = writeln!(
            output,
            "    canonical_target_field: {}",
            entry.canonical_target_field
        );
        let _ = writeln!(output, "    authored_path: {}", entry.authored_path);
        let _ = writeln!(output, "    source_system: {}", entry.source_system);
        let _ = writeln!(output, "    support: {}", entry.support);
    }
    output
}

fn render_diagnostics(diagnostics: &[PackageDiagnostic]) -> String {
    if diagnostics.is_empty() {
        return "diagnostics: []\n".to_owned();
    }

    let mut output = String::from("diagnostics:\n");
    for diagnostic in diagnostics {
        let _ = writeln!(output, "  - class: {}", diagnostic.class);
        let _ = writeln!(output, "    severity: {}", diagnostic.severity.as_str());
        let _ = writeln!(output, "    message: {}", diagnostic.message);
        let _ = writeln!(output, "    subject_ref: {}", diagnostic.subject_ref);
        let _ = writeln!(output, "    claim_blocking: {}", diagnostic.claim_blocking);
    }
    output
}

fn parse_manifest(text: &str) -> Result<PackageManifest, PackageStoreError> {
    let mut schema_version = None;
    let mut package_id = None;
    let mut package_title = None;
    let mut game_system_id = None;
    let mut package_version = None;
    let mut depends_on = Vec::new();
    let mut supported_object_kinds = Vec::new();
    let mut validation_state = None;
    let mut provenance_policy = None;
    let mut proof_binding = ParsedProofBinding::default();
    let mut section = ManifestSection::TopLevel;

    for raw_line in text.lines() {
        let line = raw_line.trim_end();
        if line.is_empty() {
            continue;
        }
        if let Some(value) = line.strip_prefix("schema_version: ") {
            schema_version = Some(value.parse::<u16>().map_err(|_| parse_error("manifest schema_version is invalid"))?);
            section = ManifestSection::TopLevel;
        } else if let Some(value) = line.strip_prefix("package_id: ") {
            package_id = Some(value.to_owned());
            section = ManifestSection::TopLevel;
        } else if let Some(value) = line.strip_prefix("package_title: ") {
            package_title = Some(value.to_owned());
            section = ManifestSection::TopLevel;
        } else if let Some(value) = line.strip_prefix("game_system_id: ") {
            game_system_id = Some(value.to_owned());
            section = ManifestSection::TopLevel;
        } else if let Some(value) = line.strip_prefix("package_version: ") {
            package_version = Some(value.to_owned());
            section = ManifestSection::TopLevel;
        } else if line == "depends_on:" {
            section = ManifestSection::DependsOn;
        } else if line == "supported_object_kinds:" {
            section = ManifestSection::SupportedKinds;
        } else if let Some(value) = line.strip_prefix("validation_state: ") {
            validation_state = Some(PackageValidationState::parse(value).ok_or_else(|| {
                parse_error(format!("manifest validation_state '{value}' is unsupported"))
            })?);
            section = ManifestSection::TopLevel;
        } else if let Some(value) = line.strip_prefix("provenance_policy: ") {
            provenance_policy = Some(value.to_owned());
            section = ManifestSection::TopLevel;
        } else if line == "proof_binding:" {
            section = ManifestSection::ProofBinding;
        } else if let Some(value) = line.strip_prefix("  - ") {
            match section {
                ManifestSection::DependsOn => depends_on.push(value.to_owned()),
                ManifestSection::SupportedKinds => supported_object_kinds.push(value.to_owned()),
                _ => return Err(parse_error(format!("unexpected list item in manifest: {line}"))),
            }
        } else if let Some(rest) = line.strip_prefix("  ") {
            match section {
                ManifestSection::ProofBinding => {
                    let (key, value) = split_field(rest)?;
                    proof_binding.apply(key, value)?;
                }
                _ => return Err(parse_error(format!("unexpected indented manifest field: {line}"))),
            }
        } else {
            return Err(parse_error(format!("unsupported manifest line: {line}")));
        }
    }

    Ok(PackageManifest {
        schema_version: schema_version.ok_or_else(|| parse_error("manifest missing schema_version"))?,
        package_id: package_id.ok_or_else(|| parse_error("manifest missing package_id"))?,
        package_title: package_title.ok_or_else(|| parse_error("manifest missing package_title"))?,
        game_system_id: game_system_id.ok_or_else(|| parse_error("manifest missing game_system_id"))?,
        package_version: package_version.ok_or_else(|| parse_error("manifest missing package_version"))?,
        depends_on,
        supported_object_kinds,
        validation_state: validation_state.ok_or_else(|| parse_error("manifest missing validation_state"))?,
        provenance_policy: provenance_policy.ok_or_else(|| parse_error("manifest missing provenance_policy"))?,
        proof_binding: proof_binding.finish()?,
    })
}

fn parse_feat_record(text: &str) -> Result<FeatRecord, PackageStoreError> {
    let mut stable_id = None;
    let mut package_id = None;
    let mut display_name = None;
    let mut object_kind = None;
    let mut substitutes_for = None;
    let mut effect_ids = Vec::new();
    let mut prerequisite_ids = Vec::new();
    let mut semantic_intent = None;
    let mut section = RecordListSection::TopLevel;

    for raw_line in text.lines() {
        let line = raw_line.trim_end();
        if line.is_empty() {
            continue;
        }
        if let Some(value) = line.strip_prefix("stable_id: ") {
            stable_id = Some(value.to_owned());
            section = RecordListSection::TopLevel;
        } else if let Some(value) = line.strip_prefix("package_id: ") {
            package_id = Some(value.to_owned());
            section = RecordListSection::TopLevel;
        } else if let Some(value) = line.strip_prefix("display_name: ") {
            display_name = Some(value.to_owned());
            section = RecordListSection::TopLevel;
        } else if let Some(value) = line.strip_prefix("object_kind: ") {
            object_kind = Some(value.to_owned());
            section = RecordListSection::TopLevel;
        } else if let Some(value) = line.strip_prefix("substitutes_for: ") {
            substitutes_for = Some(value.to_owned());
            section = RecordListSection::TopLevel;
        } else if line == "effect_ids:" {
            section = RecordListSection::EffectIds;
        } else if line == "prerequisite_ids:" {
            section = RecordListSection::PrerequisiteIds;
        } else if let Some(value) = line.strip_prefix("semantic_intent: ") {
            semantic_intent = Some(value.to_owned());
            section = RecordListSection::TopLevel;
        } else if let Some(value) = line.strip_prefix("  - ") {
            match section {
                RecordListSection::EffectIds => effect_ids.push(value.to_owned()),
                RecordListSection::PrerequisiteIds => prerequisite_ids.push(value.to_owned()),
                RecordListSection::TopLevel => {
                    return Err(parse_error(format!("unexpected feat record list item: {line}")))
                }
            }
        } else {
            return Err(parse_error(format!("unsupported feat record line: {line}")));
        }
    }

    Ok(FeatRecord {
        stable_id: stable_id.ok_or_else(|| parse_error("feat record missing stable_id"))?,
        package_id: package_id.ok_or_else(|| parse_error("feat record missing package_id"))?,
        display_name: display_name.ok_or_else(|| parse_error("feat record missing display_name"))?,
        object_kind: object_kind.ok_or_else(|| parse_error("feat record missing object_kind"))?,
        substitutes_for: substitutes_for
            .ok_or_else(|| parse_error("feat record missing substitutes_for"))?,
        effect_ids,
        prerequisite_ids,
        semantic_intent: semantic_intent
            .ok_or_else(|| parse_error("feat record missing semantic_intent"))?,
    })
}

fn parse_effect_record(text: &str) -> Result<EffectRecord, PackageStoreError> {
    let mut stable_id = None;
    let mut owning_feat_id = None;
    let mut target_family = None;
    let mut modifier_type = None;
    let mut modifier_value = None;
    let mut stacking_posture = None;

    for raw_line in text.lines() {
        let line = raw_line.trim_end();
        if line.is_empty() {
            continue;
        }
        if let Some(value) = line.strip_prefix("stable_id: ") {
            stable_id = Some(value.to_owned());
        } else if let Some(value) = line.strip_prefix("owning_feat_id: ") {
            owning_feat_id = Some(value.to_owned());
        } else if let Some(value) = line.strip_prefix("target_family: ") {
            target_family = Some(value.to_owned());
        } else if let Some(value) = line.strip_prefix("modifier_type: ") {
            modifier_type = Some(value.to_owned());
        } else if let Some(value) = line.strip_prefix("modifier_value: ") {
            modifier_value = Some(value.parse::<i16>().map_err(|_| parse_error("effect modifier_value must be numeric"))?);
        } else if let Some(value) = line.strip_prefix("stacking_posture: ") {
            stacking_posture = Some(value.to_owned());
        } else {
            return Err(parse_error(format!("unsupported effect record line: {line}")));
        }
    }

    Ok(EffectRecord {
        stable_id: stable_id.ok_or_else(|| parse_error("effect record missing stable_id"))?,
        owning_feat_id: owning_feat_id.ok_or_else(|| parse_error("effect record missing owning_feat_id"))?,
        target_family: target_family.ok_or_else(|| parse_error("effect record missing target_family"))?,
        modifier_type: modifier_type.ok_or_else(|| parse_error("effect record missing modifier_type"))?,
        modifier_value: modifier_value.ok_or_else(|| parse_error("effect record missing modifier_value"))?,
        stacking_posture: stacking_posture.ok_or_else(|| parse_error("effect record missing stacking_posture"))?,
    })
}

fn parse_prerequisite_record(text: &str) -> Result<PrerequisiteRecord, PackageStoreError> {
    let mut stable_id = None;
    let mut owning_feat_id = None;
    let mut predicate = None;

    for raw_line in text.lines() {
        let line = raw_line.trim_end();
        if line.is_empty() {
            continue;
        }
        if let Some(value) = line.strip_prefix("stable_id: ") {
            stable_id = Some(value.to_owned());
        } else if let Some(value) = line.strip_prefix("owning_feat_id: ") {
            owning_feat_id = Some(value.to_owned());
        } else if let Some(value) = line.strip_prefix("predicate: ") {
            predicate = Some(value.to_owned());
        } else {
            return Err(parse_error(format!("unsupported prerequisite record line: {line}")));
        }
    }

    Ok(PrerequisiteRecord {
        stable_id: stable_id.ok_or_else(|| parse_error("prerequisite record missing stable_id"))?,
        owning_feat_id: owning_feat_id
            .ok_or_else(|| parse_error("prerequisite record missing owning_feat_id"))?,
        predicate: predicate.ok_or_else(|| parse_error("prerequisite record missing predicate"))?,
    })
}

fn parse_provenance(text: &str) -> Result<Vec<ProvenanceEntry>, PackageStoreError> {
    let mut entries = Vec::new();
    let mut current: Option<ParsedProvenanceEntry> = None;

    for raw_line in text.lines() {
        let line = raw_line.trim_end();
        if line.is_empty() || line == "entries:" {
            continue;
        }
        if let Some(value) = line.strip_prefix("  - stable_id: ") {
            if let Some(parsed) = current.take() {
                entries.push(parsed.finish()?);
            }
            let mut next = ParsedProvenanceEntry::default();
            next.stable_id = Some(value.to_owned());
            current = Some(next);
        } else if let Some(rest) = line.strip_prefix("    ") {
            let (key, value) = split_field(rest)?;
            let Some(current) = current.as_mut() else {
                return Err(parse_error("provenance field appeared before any entry"));
            };
            current.apply(key, value)?;
        } else {
            return Err(parse_error(format!("unsupported provenance line: {line}")));
        }
    }

    if let Some(parsed) = current.take() {
        entries.push(parsed.finish()?);
    }

    Ok(entries)
}

fn parse_diagnostics(text: &str) -> Result<Vec<PackageDiagnostic>, PackageStoreError> {
    if text.trim() == "diagnostics: []" {
        return Ok(Vec::new());
    }

    let mut diagnostics = Vec::new();
    let mut current: Option<ParsedDiagnostic> = None;

    for raw_line in text.lines() {
        let line = raw_line.trim_end();
        if line.is_empty() || line == "diagnostics:" {
            continue;
        }
        if let Some(value) = line.strip_prefix("  - class: ") {
            if let Some(parsed) = current.take() {
                diagnostics.push(parsed.finish()?);
            }
            let mut next = ParsedDiagnostic::default();
            next.class = Some(value.to_owned());
            current = Some(next);
        } else if let Some(rest) = line.strip_prefix("    ") {
            let (key, value) = split_field(rest)?;
            let Some(current) = current.as_mut() else {
                return Err(parse_error("diagnostic field appeared before any entry"));
            };
            current.apply(key, value)?;
        } else {
            return Err(parse_error(format!("unsupported diagnostic line: {line}")));
        }
    }

    if let Some(parsed) = current.take() {
        diagnostics.push(parsed.finish()?);
    }

    Ok(diagnostics)
}

fn read_optional_single_record<T, F>(directory: PathBuf, parser: F) -> Result<Option<T>, PackageStoreError>
where
    F: Fn(&str) -> Result<T, PackageStoreError>,
{
    if !directory.exists() {
        return Ok(None);
    }

    let mut files = fs::read_dir(&directory)
        .map_err(|err| io_error(&directory, err))?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();
    files.sort();

    match files.len() {
        0 => Ok(None),
        1 => Ok(Some(parser(&read_required(files.remove(0))?)?)),
        _ => Err(PackageStoreError {
            message: format!(
                "expected at most one authored record in {}, found {}",
                directory.display(),
                files.len()
            ),
        }),
    }
}

fn read_required(path: PathBuf) -> Result<String, PackageStoreError> {
    fs::read_to_string(&path).map_err(|err| io_error(&path, err))
}

fn split_field(line: &str) -> Result<(&str, &str), PackageStoreError> {
    let Some((key, value)) = line.split_once(':') else {
        return Err(parse_error(format!("expected field with ':' separator, got '{line}'")));
    };
    Ok((key.trim(), value.trim_start()))
}

fn io_error(path: &Path, err: std::io::Error) -> PackageStoreError {
    PackageStoreError {
        message: format!("{}: {err}", path.display()),
    }
}

fn parse_error(message: impl Into<String>) -> PackageStoreError {
    PackageStoreError {
        message: message.into(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ManifestSection {
    TopLevel,
    DependsOn,
    SupportedKinds,
    ProofBinding,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RecordListSection {
    TopLevel,
    EffectIds,
    PrerequisiteIds,
}

#[derive(Default)]
struct ParsedProofBinding {
    case_id: Option<String>,
    slot: Option<String>,
    remove: Option<String>,
    add: Option<String>,
}

impl ParsedProofBinding {
    fn apply(&mut self, key: &str, value: &str) -> Result<(), PackageStoreError> {
        match key {
            "case_id" => self.case_id = Some(value.to_owned()),
            "slot" => self.slot = Some(value.to_owned()),
            "remove" => self.remove = Some(value.to_owned()),
            "add" => self.add = Some(value.to_owned()),
            _ => return Err(parse_error(format!("unsupported proof_binding field '{key}'"))),
        }
        Ok(())
    }

    fn finish(self) -> Result<ProofBinding, PackageStoreError> {
        Ok(ProofBinding {
            case_id: self.case_id.ok_or_else(|| parse_error("proof_binding missing case_id"))?,
            slot: self.slot.ok_or_else(|| parse_error("proof_binding missing slot"))?,
            remove: self.remove.ok_or_else(|| parse_error("proof_binding missing remove"))?,
            add: self.add.ok_or_else(|| parse_error("proof_binding missing add"))?,
        })
    }
}

#[derive(Default)]
struct ParsedProvenanceEntry {
    stable_id: Option<String>,
    source_package_id: Option<String>,
    canonical_target_id: Option<String>,
    canonical_target_field: Option<String>,
    authored_path: Option<String>,
    source_system: Option<String>,
    support: Option<String>,
}

impl ParsedProvenanceEntry {
    fn apply(&mut self, key: &str, value: &str) -> Result<(), PackageStoreError> {
        match key {
            "source_package_id" => self.source_package_id = Some(value.to_owned()),
            "canonical_target_id" => self.canonical_target_id = Some(value.to_owned()),
            "canonical_target_field" => self.canonical_target_field = Some(value.to_owned()),
            "authored_path" => self.authored_path = Some(value.to_owned()),
            "source_system" => self.source_system = Some(value.to_owned()),
            "support" => self.support = Some(value.to_owned()),
            _ => return Err(parse_error(format!("unsupported provenance field '{key}'"))),
        }
        Ok(())
    }

    fn finish(self) -> Result<ProvenanceEntry, PackageStoreError> {
        Ok(ProvenanceEntry {
            stable_id: self.stable_id.ok_or_else(|| parse_error("provenance missing stable_id"))?,
            source_package_id: self
                .source_package_id
                .ok_or_else(|| parse_error("provenance missing source_package_id"))?,
            canonical_target_id: self
                .canonical_target_id
                .ok_or_else(|| parse_error("provenance missing canonical_target_id"))?,
            canonical_target_field: self
                .canonical_target_field
                .ok_or_else(|| parse_error("provenance missing canonical_target_field"))?,
            authored_path: self
                .authored_path
                .ok_or_else(|| parse_error("provenance missing authored_path"))?,
            source_system: self
                .source_system
                .ok_or_else(|| parse_error("provenance missing source_system"))?,
            support: self.support.ok_or_else(|| parse_error("provenance missing support"))?,
        })
    }
}

#[derive(Default)]
struct ParsedDiagnostic {
    class: Option<String>,
    severity: Option<PackageDiagnosticSeverity>,
    message: Option<String>,
    subject_ref: Option<String>,
    claim_blocking: Option<bool>,
}

impl ParsedDiagnostic {
    fn apply(&mut self, key: &str, value: &str) -> Result<(), PackageStoreError> {
        match key {
            "severity" => {
                self.severity = PackageDiagnosticSeverity::parse(value);
                if self.severity.is_none() {
                    return Err(parse_error(format!("unsupported diagnostic severity '{value}'")));
                }
            }
            "message" => self.message = Some(value.to_owned()),
            "subject_ref" => self.subject_ref = Some(value.to_owned()),
            "claim_blocking" => {
                self.claim_blocking = Some(match value {
                    "true" => true,
                    "false" => false,
                    _ => return Err(parse_error(format!("diagnostic claim_blocking must be true/false, got '{value}'"))),
                })
            }
            _ => return Err(parse_error(format!("unsupported diagnostic field '{key}'"))),
        }
        Ok(())
    }

    fn finish(self) -> Result<PackageDiagnostic, PackageStoreError> {
        Ok(PackageDiagnostic {
            class: self.class.ok_or_else(|| parse_error("diagnostic missing class"))?,
            severity: self.severity.ok_or_else(|| parse_error("diagnostic missing severity"))?,
            message: self.message.ok_or_else(|| parse_error("diagnostic missing message"))?,
            subject_ref: self.subject_ref.ok_or_else(|| parse_error("diagnostic missing subject_ref"))?,
            claim_blocking: self
                .claim_blocking
                .ok_or_else(|| parse_error("diagnostic missing claim_blocking"))?,
        })
    }
}
