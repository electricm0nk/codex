# AT-35-E6-003-RULED cycle 1 — the run-time `pcgen_import` census (operator ruling B16)

Every shipping-code line under a live root that names `pcgen_import`, with the mechanism that
holds it there. Re-derive the whole of this file:

```
python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle1_runtime_import_census.py
```

Denominator for every figure below: every source file (`.rs .ts .tsx .js .jsx .mjs .cjs`) under
the five live roots of `technical-design.md` §0, comment lines excluded (ruling B14) and
`#[cfg(test)]` regions excluded (ruling B15). The script asserts its total against
`scripts/pcgen_residue_gate.py`’s own `hits_by_pattern["pcgen_import"]`, so the census and the gate
cannot disagree.

## The two movements, never netted

- **Instrument correction (B15), reported as zero progress:** 300 hits across 45 files leave the
  count. Every one was inside a `#[cfg(test)]` region compiled out of the shipping binary. Not one
  line of shipping code changed because of it.
- **Defect that was always there (B16):** 58 hits across 25 files enter the count, 12 of them under
  `apps/desktop/`, where the gate printed `files=0 hits=0`. They were in the shipping binary the
  whole time. This is not a regression this cycle caused.

## The census

```
pcgen_import_hits=58 files=25
by_root=apps/desktop=12, src/rules_core=46

=== renderer: hits=8 files=2 ===
  why it is still here: The PCGen description renderer called at run time on a corpus record's stored `description`. Needs the converted rule's own prose (`resolved_prose::render_description`) seeded from the values the caller already computes, gated by a corpus-wide parity test against the tool-side renderer. The mechanism is proved; `AT-35-E6-003-FINISH` cycle 3 did it for racial traits.
  apps/desktop/src-tauri/src/feat_catalog.rs:213  let clean = |text: &str| codex::pcgen_import::pcgen_desc::leaked_pcgen_syntax(text).is_none();
  src/rules_core/pilot_compute/class_feature_grant_consumer.rs:963  let mut values = crate::pcgen_import::pcgen_desc::PcgenDisplayValues::new();
  src/rules_core/pilot_compute/class_feature_grant_consumer.rs:968  crate::pcgen_import::pcgen_desc::render_pcgen_desc_with_values(&record.raw_description, &values);
  src/rules_core/pilot_compute/class_feature_grant_consumer.rs:972  if crate::pcgen_import::pcgen_desc::leaked_pcgen_syntax(&rendered.text).is_some() {
  src/rules_core/pilot_compute/class_feature_grant_consumer.rs:1036  let args = crate::pcgen_import::pcgen_desc::desc_token_arguments(&record.raw_description);
  src/rules_core/pilot_compute/class_feature_grant_consumer.rs:1068  let mut values = crate::pcgen_import::pcgen_desc::PcgenDisplayValues::new();
  src/rules_core/pilot_compute/class_feature_grant_consumer.rs:1081  let rendered = crate::pcgen_import::pcgen_desc::render_pcgen_desc_with_values(
  src/rules_core/pilot_compute/class_feature_grant_consumer.rs:1088  if crate::pcgen_import::pcgen_desc::leaked_pcgen_syntax(&rendered.text).is_some() {

=== lst_parser_types: hits=22 files=13 ===
  why it is still here: The ingest-format record STRUCTS (`EquipmentRecord`, `LstSpellRecord`, the class/metadata/race-ability/spellcasting rows) are declared on the converter side and used as the live side's own data type. Not a token read -- a type ownership fact. Needs the live side to own a converted equipment/spell record shape the converter emits into, which does not exist yet.
  apps/desktop/src-tauri/src/corpus_fixtures.rs:24  use codex::pcgen_import::lst_parser::equipment::parse_equipment_entries;
  apps/desktop/src-tauri/src/corpus_fixtures.rs:25  use codex::pcgen_import::lst_parser::spell::parse_lst_spell_row;
  apps/desktop/src-tauri/src/corpus_fixtures.rs:68  let record: &'static codex::pcgen_import::lst_parser::spell::LstSpellRecord =
  apps/desktop/src-tauri/src/corpus_fixtures.rs:79  let entry: &'static codex::pcgen_import::lst_parser::equipment::EquipmentRecord =
  src/rules_core/composed_input.rs:55  use crate::pcgen_import::lst_parser::class::{
  src/rules_core/composed_input.rs:58  use crate::pcgen_import::lst_parser::equipment::{
  src/rules_core/composed_input.rs:61  use crate::pcgen_import::lst_parser::metadata::{
  src/rules_core/composed_input.rs:64  use crate::pcgen_import::lst_parser::race_ability::{
  src/rules_core/composed_input.rs:67  use crate::pcgen_import::lst_parser::spell::{
  src/rules_core/composed_input.rs:70  use crate::pcgen_import::lst_parser::spellcasting_class::{
  src/rules_core/corpus_loader.rs:39  use crate::pcgen_import::lst_parser::equipment::{
  src/rules_core/corpus_loader.rs:42  use crate::pcgen_import::lst_parser::spell::{LstSpellRecord, LstSpellRecordPayload};
  src/rules_core/damage_total.rs:106  use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;
  src/rules_core/encumbrance.rs:60  use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;
  src/rules_core/equipment_effects.rs:51  use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;
  src/rules_core/equipment_effects/arms_armor.rs:22  use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;
  src/rules_core/equipment_effects/equipmods.rs:87  use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;
  src/rules_core/equipment_effects/general.rs:19  use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;
  src/rules_core/equipment_effects/intelligent_item.rs:66  use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;
  src/rules_core/equipment_effects/magic_items.rs:22  use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;
  src/rules_core/equipment_resolver.rs:18  use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;
  src/rules_core/spell_resolver.rs:15  use crate::pcgen_import::lst_parser::spell::LstSpellRecord;

=== ingest_record_tokens: hits=7 files=5 ===
  why it is still here: Run-time token-string reads: `token_pairs`, `bonus_chain_qualifiers`, `rebuild_bonus_token`, `token_count`, and the `RaceCacheData` / `RaceTraitCacheData` payload shapes. These are the literal ingest tokens the sheet rule forbids, reached through a function call so no token-syntax pattern fires. Needs the converted package to carry the same facts keyed by `VarId` rather than by source token name.
  src/rules_core/corpus_loader.rs:380  for (k, v) in crate::pcgen_import::ingest_record::token_pairs(data) {
  src/rules_core/corpus_loader.rs:384  for qualifiers in crate::pcgen_import::ingest_record::bonus_chain_qualifiers(data) {
  src/rules_core/corpus_loader.rs:390  let raw_bonus = crate::pcgen_import::ingest_record::rebuild_bonus_token(&qualifiers);
  src/rules_core/derived_evaluator_fixture_check.rs:29  use crate::pcgen_import::ingest_record;
  src/rules_core/race_resolver.rs:95  use crate::pcgen_import::ingest_payload::{RaceCacheData, RaceTraitCacheData};
  src/rules_core/rules_tables/simple_kind_tables.rs:168  let raw_token_count = crate::pcgen_import::ingest_record::token_count(data);
  src/rules_core/trait_pool.rs:51  use crate::pcgen_import::ingest_record;

=== trait_and_pool_tokens: hits=5 files=4 ===
  why it is still here: Per-row token readers for racial traits, feature pools and bonus chains. The racial-trait half of this group is the one `AT-35-E6-003-FINISH` cycle 3 already proved removable; the remaining call sites are the pool/catalog half, which has no converted pool-member table yet.
  apps/desktop/src-tauri/src/race_trait_picker.rs:114  use codex::pcgen_import::race_trait_tokens;
  src/rules_core/class_feature_pool_catalog.rs:114  use crate::pcgen_import::pool_member_tokens;
  src/rules_core/race_resolver.rs:86  use crate::pcgen_import::bonus_chain_reader::{self, DeclaredBonuses};
  src/rules_core/race_resolver.rs:87  use crate::pcgen_import::race_trait_tokens;
  src/rules_core/skinwalker_change_shape.rs:73  use crate::pcgen_import::race_trait_tokens;

=== ir_converter: hits=6 files=4 ===
  why it is still here: The converter's own conversion entry points invoked from live code at run time -- `convert_equipment_record`, `convert_spell_record`, the include resolver, and one `cache_gen::equipment_gap::RenameInfo` field type. The live side is running the conversion instead of reading its output. Needs the converted artefact to be produced at build time and read as data.
  apps/desktop/src-tauri/src/corpus_fixtures.rs:23  use codex::pcgen_import::ir_converter::{convert_equipment_record, convert_spell_record};
  src/rules_core/composed_input.rs:47  use crate::pcgen_import::include_resolver::{
  src/rules_core/composed_input.rs:50  use crate::pcgen_import::ir_converter::{
  src/rules_core/corpus_loader.rs:88  package.push(crate::pcgen_import::ir_converter::convert_equipment_record(record));
  src/rules_core/corpus_loader.rs:137  package.push(crate::pcgen_import::ir_converter::convert_spell_record(record));
  src/rules_core/rules_tables/crb/json_cache.rs:151  pub rename: Option<crate::pcgen_import::cache_gen::equipment_gap::RenameInfo>,

=== source_content_payload: hits=5 files=5 ===
  why it is still here: Payload and bonus-reader types re-exported or imported by the live side. Shape-only dependencies on converter-side declarations; they move when the converted equipment/source-content shape above exists.
  src/rules_core/equipment_effects/arms_armor.rs:21  use crate::pcgen_import::equipment_bonus_reader;
  src/rules_core/equipment_effects/equipmods.rs:86  use crate::pcgen_import::equipment_bonus_reader;
  src/rules_core/equipment_resolver.rs:19  use crate::pcgen_import::source_content_payload::SourceContentPayload;
  src/rules_core/source_content.rs:50  pub use crate::pcgen_import::source_content_payload::SourceContentPayload;
  src/rules_core/spell_resolver.rs:16  use crate::pcgen_import::source_content_payload::SourceContentPayload;

=== provenance_prose: hits=5 files=1 ===
  why it is still here: NOT a run-time read: the converter's module PATH written inside a string literal, as the provenance sentence a reach-gate row prints for where its words come from. B14 excused the same sentence in a `//` comment; in a string literal the gate counts it, correctly and deliberately -- widening the pattern to exempt string literals would be exactly the forbidden weakening. Clearing it is a prose edit, and it is left alone here on purpose: trimming it would move the total without moving any dependency.
  apps/desktop/src-tauri/src/reach_gate.rs:3417  these rows' words from their own source rows, `src/pcgen_import/sheet_rule/` \
  apps/desktop/src-tauri/src/reach_gate.rs:3433  converter deriving these rows' words from their own source rows, `src/pcgen_import/\
  apps/desktop/src-tauri/src/reach_gate.rs:3445  converter, `src/pcgen_import/sheet_rule/`.",
  apps/desktop/src-tauri/src/reach_gate.rs:3454  words from their own source rows, `src/pcgen_import/sheet_rule/`.",
  apps/desktop/src-tauri/src/reach_gate.rs:3475  content exists; otherwise the converter, `src/pcgen_import/sheet_rule/`.",

census written to docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle1_runtime_import_census.json
```
