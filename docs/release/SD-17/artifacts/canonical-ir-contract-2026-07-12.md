SD-17 Slice C — Canonical-IR Contract
====================================

slice_id:        SD17-C
slice_role:      lst-parser (canonical-ir-conversion)
assignee:        tech-priest
parent_gate:     t_dd3dacbd
parent_tranche:  tranche-2-7
date:            2026-07-12
authoritative:   programs/codex/requirements/SD-17-pcgen-corpus-include-graph-resolution/artifacts/canonical-ir-contract-2026-07-12.md
status:          FIRST DELIVERABLE of Slice C (this artifact is authored by Slice C
                 per the 2026-07-12 doctrine repair; GE-02/GE-04 own the wider
                 canonical-model surface but have not landed IRShape/IRNode types
                 in src/, so Slice C defines the bounded structural types this
                 slice ships)

Purpose
-------

Slice C consumes the parsed-LST records emitted by the six Slice B parsers
(B-1 through B-6) and converts them into the canonical internal representation
that the rules-core compute path consumes. This contract is the authoritative
specification for:

  1. the IRSchema field surface (what fields an IR schema advertises);
  2. the IRNode enum (one variant per B-family record kind);
  3. the IRDiagnostic struct (provenance + severity + code);
  4. the LST-to-IR mapping rules (how each B-family record kind maps to an
     IRNode variant);
  5. the lossy/partial-field inventory (what does NOT survive the conversion
     and what the downstream consumer can expect to find or not find).

This contract is the source of truth for the public API in
`src/pcgen_import/ir_converter.rs`; the implementation conforms to this
document, not the other way around. If the implementation and this contract
disagree, this document wins; the implementation is repaired.

Authoring boundary
------------------

This contract defines the **bounded structural types** that Slice C ships. It
does not define the canonical model that GE-02 (rules_core types) or GE-04
(rules_core schema) own. When those types land in `src/`, this slice's
converters can be re-targeted to consume the canonical model surface without
changing the IRNode/IRDiagnostic/IRSchema shape — those are the slice's
contractual output, not a transient implementation detail.

Provenance doctrine
-------------------

The canonical IR is a strict-provenance surface. Every IRNode carries:

  - source_path: String  — identity of the originating LST file
  - line_number: usize   — one-based source line number where the parsed
                            record originated
  - raw_line:    String  — verbatim copy of the source line as evidence

This is non-negotiable. The downstream rules-core compute path uses
source_path + line_number for every human-facing diagnostic it raises;
raw_line is the audit trail. Removing any of these from IRNode would
break the diagnostic contract that the rules-core consumer depends on.

IRSchema
--------

The `IRSchema` struct is a thin descriptor that the consumer hands to
`convert_to_ir` to declare which fields it expects. Slice C does NOT
filter records against IRSchema — it projects every record through a
lossless mapping whose only interpretation is the field-tag taxonomy the
schema names. The schema is therefore descriptive (what fields are
advertised), not prescriptive (which records are rejected).

Fields:

  - schema_id:     &'static str — canonical schema identifier, e.g.
                              "codex.pcgen.canonical-ir.v1"
  - schema_version: u32         — schema revision, bumped on breaking changes
  - recognized_kinds: &'static [&'static str]
                              — directive-token prefixes the schema
                                recognizes (e.g. "CLASS", "SPELL",
                                "RACE", "ABILITY", "EQUIP", "EQUIPMOD",
                                "DEITY", "DOMAIN", "KITS", "LANGUAGE",
                                "TEMPLATE", "COMPANIONMOD")

The default schema (constructed by `IRSchema::canonical_v1()`) recognizes
all six B-family kinds and is what the rules-core consumer expects.

IRNode
------

`IRNode` is a single enum whose variants cover every B-family record kind.
This is the bounded structural type the card body names: it is shaped by
the B-family parser outputs that already live in `src/pcgen_import/lst_parser/`,
not by a fabricated canonical model surface.

Variants:

  - Class(ClassEntryProjection)
  - SpellcastingClass(SpellcastingClassEntryProjection)
  - Race(RaceDeclarationProjection)
  - Ability(AbilityDeclarationProjection)
  - Spell(LstSpellRecordProjection)
  - Equipment(EquipmentRecordProjection)
  - Metadata(LstRecordProjection)        — the six metadata kinds

Every IRNode embeds provenance (source_path, line_number, raw_line) and
the B-family parser's structured record fields, projected 1:1 with no
interpretation. The projection types are thin wrappers: a `ClassEntryProjection`
holds a `ClassEntry` plus the three provenance fields (the ClassEntry already
carries `header_line_number` and `header_raw_line`, so the wrapper just
relabels to the canonical `line_number` / `raw_line` names the consumer
expects).

IRDiagnostic
------------

`IRDiagnostic` is the canonical error/warning surface for the converter.
It carries provenance (same triple as IRNode), a severity, a code, and a
human-readable message.

Fields:

  - source_path: String
  - line_number: usize
  - raw_line:    String
  - severity:    IRDiagnosticSeverity  — Error | Warning | Info
  - code:        &'static str          — canonical diagnostic code
                                            (e.g. "IR_MALFORMED_C17",
                                             "IR_UNKNOWN_KIND_C17",
                                             "IR_LOSSY_PROJECTION_C17")
  - source_kind: &'static str          — the B-family slice tag
                                            ("SD17-B-1" through "SD17-B-6")
  - message:     String

All Slice-C-emitted diagnostics use the slice tag `SD17-C` in
source_kind when the diagnostic is converter-originated. Diagnostics
forwarded from a B-family parser carry the originating slice's tag.

LST-to-IR mapping rules
-----------------------

The conversion is mechanically defined. For every variant of IRNode, the
mapping is:

  IRNode::Class(entry)        <- ClassEntry
  IRNode::SpellcastingClass(e)<- SpellcastingClassEntry
  IRNode::Race(race)          <- RaceDeclaration
  IRNode::Ability(ability)    <- AbilityDeclaration
  IRNode::Spell(spell)        <- LstSpellRecord
  IRNode::Equipment(equip)    <- EquipmentRecord
  IRNode::Metadata(record)    <- LstRecord          (the B-6 metadata record)

Where the wrapper projection types are:

  ClassEntryProjection        { source_path, line_number, raw_line, entry }
  SpellcastingClassEntryProjection { source_path, line_number, raw_line, entry }
  RaceDeclarationProjection   { source_path, line_number, raw_line, declaration }
  AbilityDeclarationProjection{ source_path, line_number, raw_line, declaration }
  LstSpellRecordProjection    { source_path, line_number, raw_line, record }
  EquipmentRecordProjection   { source_path, line_number, raw_line, record }
  LstRecordProjection         { source_path, line_number, raw_line, record }

The mapping rules are:

  R1. Provenance projection is exact — source_path, line_number, raw_line
      on every variant are the LST parser's own field, never synthesized.
      Where the B-family parser uses different field names (e.g.
      `header_line_number` on ClassEntry), the projection relabels to
      the canonical name; the value is identical.

  R2. Structured payload is by-reference. The IRNode variant holds a
      borrowed reference to the source record; conversion does not
      clone or re-parse. This keeps the conversion O(1) per record.

  R3. Forwarded diagnostics are preserved. Every LstDiagnostic carried
      by the B-family parser's container is wrapped as an IRDiagnostic
      with severity=Warning and source_kind set to the originating
      slice's tag (e.g. SD17-B-1 for ClassEntry).

  R4. Conversion is total — every B-family record produces an IRNode.
      There is no rejection. Records the schema does not recognize
      still convert to their respective variant; the schema's
      `recognized_kinds` is descriptive, not gating.

  R5. Conversion is deterministic — given the same B-family record, the
      converter always produces the same IRNode. No HashMap iteration
      order leaks into the output. No randomness is used.

  R6. Conversion is lossless at the structured-payload level — every
      field the B-family parser carries appears in the IRNode's
      embedded projection. The only "lossy" fields are documented in
      the next section.

Lossy / partial fields
----------------------

The conversion is intentionally lossless at the structured-payload level
(R6). The fields below are the converter's known partial surfaces — they
do NOT mean fields are dropped; they describe cases where the IRNode
inherits a partial state from the upstream parser:

  L1. Malformed diagnostics — a B-family record that the parser flagged
      with an LstDiagnostic (e.g. `MalformedSD17B1`) still converts to
      its IRNode variant. The forwarded IRDiagnostic carries the original
      message verbatim. The rules-core consumer is responsible for
      treating the IRNode as partial when an Error-severity IRDiagnostic
      is attached.

  L2. Class entry feature blocks — a ClassEntry with no
      `###Block:` sections still converts; the projection carries the
      empty `feature_blocks: Vec<ClassFeatureBlock>` field. No loss.

  L3. Spellcasting sub-shapes — a SpellcastingClassEntry whose LST
      source lacks a SPELLSTAT line projects `spell_stat: None` and
      `casting_posture: None`. The fields are present and explicit; the
      consumer can pattern-match. No loss.

  L4. Ability declaration variants — an AbilityDeclaration whose raw line
      is a bare pointer (no pipe-delimited declaration) projects
      `parsed: None`. The raw_directive is preserved verbatim.

  L5. Equipment record kinds — an EquipmentRecord whose `kind` enum
      distinguishes between Equipment and EquipmentMod projects that
      tag explicitly; no information is dropped.

  L6. Metadata record subset — an LstRecord whose `kind` is one of the
      six metadata kinds (Deity, Domain, Kits, Language, Template,
      CompanionMod) projects that tag. Other metadata kinds the
      parser does not recognize do NOT convert; they are out of scope
      for Slice B-6 and are not produced by the parser.

Public API
----------

  pub struct IRSchema { ... }
  pub enum   IRNode   { ... }
  pub struct IRDiagnostic { ... }
  pub enum   IRDiagnosticSeverity { Error, Warning, Info }

  impl IRSchema {
      pub fn canonical_v1() -> Self
  }

  // Family-specific converters (one per B-family parser):
  pub fn convert_class_entry(entry: &ClassEntry) -> IRNode
  pub fn convert_spellcasting_class_entry(entry: &SpellcastingClassEntry) -> IRNode
  pub fn convert_race_declaration(r: &RaceDeclaration) -> IRNode
  pub fn convert_ability_declaration(a: &AbilityDeclaration) -> IRNode
  pub fn convert_spell_record(s: &LstSpellRecord) -> IRNode
  pub fn convert_equipment_record(e: &EquipmentRecord) -> IRNode
  pub fn convert_metadata_record(r: &LstRecord) -> IRNode

  // Document-level converters (consume the B-family parse-result containers):
  pub fn convert_class_parse_result(r: &ClassParseResult, schema: &IRSchema)
      -> Vec<(IRNode, Vec<IRDiagnostic>)>
  pub fn convert_spellcasting_class_parse_result(
      r: &SpellcastingClassParseResult, schema: &IRSchema)
      -> Vec<(IRNode, Vec<IRDiagnostic>)>
  pub fn convert_lst_entry_file(r: &LstEntryFile, schema: &IRSchema)
      -> Vec<(IRNode, Vec<IRDiagnostic>)>
  pub fn convert_lst_metadata_document(r: &LstMetadataDocument, schema: &IRSchema)
      -> Vec<(IRNode, Vec<IRDiagnostic>)>
  pub fn convert_spell_record_list(
      records: &[LstSpellRecord], source_path: &str, schema: &IRSchema)
      -> Vec<(IRNode, Vec<IRDiagnostic>)>
  pub fn convert_equipment_parse_result(
      r: &EquipmentParseResult, schema: &IRSchema)
      -> Vec<(IRNode, Vec<IRDiagnostic>)>

  // The signature the card body names — accepts any B-family record by
  // dispatch. Implemented as an enum-discriminated trampoline; the family-
  // specific converters above are the public entry points for typed callers.
  pub fn convert_to_ir(parsed_record: &ParsedLstRecord, schema: &IRSchema)
      -> Result<IRNode, IRDiagnostic>

  // The canonical input enum that `convert_to_ir` accepts:
  pub enum ParsedLstRecord<'a> {
      Class(&'a ClassEntry),
      SpellcastingClass(&'a SpellcastingClassEntry),
      Race(&'a RaceDeclaration),
      Ability(&'a AbilityDeclaration),
      Spell(&'a LstSpellRecord),
      Equipment(&'a EquipmentRecord),
      Metadata(&'a LstRecord),
  }

Performance contract
--------------------

The conversion is O(n) in the number of records and O(1) per record.
Specifically:

  - Each variant of `convert_to_ir` is O(1) — a structural relabel, no
    heap allocation, no iteration over the record's inner fields.
  - Document-level converters iterate the container's record list
    exactly once and call the O(1) variant for each entry. O(n) total.
  - The conversion never clones the B-family record; it projects by
    reference. This keeps the per-record cost dominated by the IRNode
    enum-discriminator dispatch, not by data copying.

Acceptance verification
-----------------------

The acceptance tests in `tests/sd17_c_ir_convert.rs` exercise:

  V1. round-trip per kind — for every variant of IRNode, a hand-built
      B-family record converts to its expected IRNode and the projection
      fields are byte-for-byte equal to the source.

  V2. forwarded-diagnostic preservation — every LstDiagnostic carried by
      a B-family parse-result container surfaces as an IRDiagnostic with
      the correct severity, code, source_kind, and message.

  V3. malformed diagnostic — a hand-built LST with a malformed entry
      surfaces as IRDiagnostic with severity=Error and code=
      "IR_MALFORMED_C17" (or the originating parser's slice-specific code
      if forwarded).

  V4. O(n) performance — a document of 5,000 records converts in under
      250ms on the standard CI runner.

  V5. line-number provenance — every IRNode carries the source line
      number from the originating B-family record. Test asserts each
      variant's `line_number` matches.

  V6. schema idempotence — converting with `IRSchema::canonical_v1()`
      produces the same IRNodes as converting with any other IRSchema;
      the schema is descriptive, not gating.

Scope non-goals
---------------

  N1. The IRSchema is descriptive, not prescriptive. The converter does
      not reject records based on `recognized_kinds`. That gating is the
      rules-core consumer's job, not the converter's.

  N2. The converter does not interpret the value grammar of any token
      beyond what the B-family parser already captured. No BONUS tree
      construction, no pipe-delimited qualifier parsing, no rule-system
      semantics. Those are owned by rules_core.

  N3. The converter does not own canonical-model types. The
      IRNode/IRSchema/IRDiagnostic types this slice ships are the
      bounded structural types named in the slice card body. When
      GE-02 / GE-04 land canonical-model types in `src/`, the
      converter can be retargeted; this contract documents the
      converter's surface, not the canonical model's surface.

  N4. The converter does not touch the UI, the rules_core, or the
      SD-13 matrix file.

  N5. The converter does not implement I/O. It accepts in-memory
      B-family parse-results; it does not read LST files.

Let it be recorded.