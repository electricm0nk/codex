SD-17 Slice E — Canonical Source-IR Contract
==============================================

slice_id:        SD17-E
slice_role:      rules-core (canonical-source-ir-shape)
assignee:        tech-priest
parent_gate:     t_dd3dacbd
parent_tranche:  tranche-2-7
date:            2026-07-12
authoritative:   programs/codex/requirements/SD-17-pcgen-corpus-include-graph-resolution/artifacts/canonical-source-ir-contract-2026-07-12.md
status:          DELIVERABLE of Slice E (this artifact names every field
                 of the canonical source-IR envelope, the per-kind
                 projection rules, the lossy/partial mappings, the
                 intentional non-translations, and the schema version)
supersedes:      (none — Slice C's canonical-ir-contract-2026-07-12.md
                 remains authoritative for the IR-conversion surface;
                 this artifact is authoritative for the source-content
                 shape the canonical IR projects INTO.)

Purpose
-------

Slice E closes the GE-04 shortfall Todd identified. The rules engine
needs a *source-side* canonical IR — a record shape that represents
**what the PCGen corpus says is available**, with full provenance back
to the LST file and line. Slice C defined the converter's IRNode
output shape; Slice E defines the canonical envelope the converter
projects into. The two are the same projection but at different
layers:

  - Slice C surface (this slice's authoring boundary, NOT a Slice E
    change): `src/pcgen_import/ir_converter.rs::IRNode<'a>` —
    per-record enum the converter dispatches on.
  - Slice E surface (this artifact's authoritative subject):
    `src/rules_core/source_content.rs` — the corpus-rooted
    `SourcePackageContent` aggregate, the `SourceContentRecord<'a>`
    envelope, the `SourceContentKind` / `SourceContentPayload` tag
    hierarchy, and the `SourceContentDiagnostic` surface.

The Slice C contract remains authoritative for the converter's
input/output contract; this artifact is authoritative for the
envelope the converter fills and the rules engine eventually
consumes.

Authoritative types
-------------------

This contract covers every public type in
`src/rules_core::source_content`:

  - `SourceRef`
  - `SourceContentKind` (+ inner `MetadataKindInner`)
  - `SourceContentPayload<'a>`
  - `SourceContentRecord<'a>`
  - `SourceContentDiagnostic` (+ `SourceContentSeverity`,
    `SourceContentDiagnosticKind`)
  - `SourcePackageContent<'a>`
  - `SourceContentLoadResult<'a>`
  - The `SOURCE_IR_VERSION` constant (currently `1`)

Field-by-field specification
----------------------------

`SourceRef { lst_file: String, line: u32 }`

  - `lst_file`: identity of the LST file the record originated from.
    Stringified via `Path::to_string_lossy()` for path-typed parsers,
    verbatim for string-typed parsers (e.g. `LstSpellFile.source_path`).
  - `line`: one-based line number of the record's first directive in
    `lst_file`. Bounded to `u32` because no realistic PCGen corpus
    exceeds `u32::MAX` lines per file. `0` is reserved for the
    diagnostic path (forwarded-from-container) — every projected
    record's `SourceRef.line` is the parser's one-based line.

`SourceContentKind` enum:

  - `Class`               — martial class (B-1)
  - `SpellcastingClass`   — spellcasting class (B-2)
  - `Race`                — race pointer (B-3)
  - `Ability`             — ability declaration (B-3)
  - `Spell`               — spell row (B-4)
  - `Equipment`           — equipment / equipment-modifier (B-5)
  - `Metadata(inner)`     — metadata-kind (B-6) with inner tag

  The inner tag is `MetadataKindInner` enum:

  - `Deity`
  - `Domain`
  - `Kits`
  - `Language`
  - `Template`
  - `CompanionMod`

  Mapping from `crate::pcgen_import::lst_parser::metadata::MetadataKind`
  to `MetadataKindInner` is total and mechanical (every variant maps
  to exactly one inner tag and back). The split between the outer
  `SourceContentKind::Metadata` and the inner `MetadataKindInner` is
  intentional: the six metadata kinds share a single B-6 parser and a
  single `SourceContentPayload::Metadata` variant, while the six
  B-family kinds each have their own variant and a top-level tag here.

`SourceContentPayload<'a>` enum:

  - `Class(&'a ClassEntry)`
  - `SpellcastingClass(&'a SpellcastingClassEntry)`
  - `Race(&'a RaceDeclaration)`
  - `Ability(&'a AbilityDeclaration)`
  - `Spell(&'a LstSpellRecord)`
  - `Equipment(&'a EquipmentRecord)`
  - `Metadata(&'a LstRecord)`

  Every variant holds a borrowed reference to the parser's entry type
  (no copies; zero-cost projection). Variants are typed against the
  parser's exact entry shape so the rules engine can pattern-match
  without re-parsing. The variants are defined in
  `src/pcgen_import/source_content_payload.rs` and re-exported from
  `rules_core::source_content` to keep the import graph acyclic (see
  the source_content module doc-comment for the layering rationale).

`SourceContentRecord<'a> { source_ref: SourceRef, kind: SourceContentKind, payload: SourceContentPayload<'a> }`

  - `source_ref`: every record carries exactly one.
  - `kind`: drives per-kind routing in the rules engine; mirrors the
    payload variant 1:1.
  - `payload`: the borrowed entry, zero-copy.

`SourceContentDiagnostic { severity: SourceContentSeverity, kind: SourceContentDiagnosticKind, message: String, source_ref: SourceRef }`

  - `severity`: `Error` / `Warning` / `Info`. The rules engine uses
    this to decide whether the diagnostic blocks evaluation.
  - `kind`: `MalformedRecord` (forwarded from a B-family parser),
    `LossyMapping` (a corpus token was preserved as a raw string),
    `UnsupportedToken` (a corpus token the source-IR does not yet
    recognize), `PartialTranslation` (a record projected partially;
    known fields populated, unknown fields preserved on the borrowed
    entry).
  - `message`: human-readable explanation.
  - `source_ref`: the diagnostic's provenance anchor.

`SourcePackageContent<'a> { package_id: String, source_ref: SourceRef, records: Vec<SourceContentRecord<'a>>, diagnostics: Vec<SourceContentDiagnostic> }`

  - `package_id`: corpus identity (e.g. `pathfinder_pf1`,
    `dnd35e_system`).
  - `source_ref`: PCC entry file the include graph was resolved
    against.
  - `records`: flat list of every projected record, in source order.
  - `diagnostics`: projection-side diagnostics (deduplicated; lossy
    mappings, malformed-record forwards, unsupported tokens).

`SourceContentLoadResult<'a> { content: Option<SourcePackageContent<'a>>, diagnostics: Vec<SourceContentDiagnostic> }`

  - `content`: `None` when projection produced a blocking error.
  - `diagnostics`: every projection-side diagnostic (errors,
    warnings, infos), even when `content` is `None`.

  Convenience constructor: `SourceContentLoadResult::empty()` for
  no-content / no-diagnostic initial state.

B-family to source-IR mapping rules
------------------------------------

The converter (`src/pcgen_import/ir_converter.rs`) projects each
B-family entry type into a `SourceContentRecord`. The mapping is
total — every B-family entry has exactly one canonical envelope
variant.

### ClassEntry (B-1) → SourceContentRecord::class

  Direct projections (no loss):

  - `source_ref.lst_file` ← `record_source_path()` (may be empty for
    parsers without per-record paths; document-level converter fills
    in via forwarded diagnostics).
  - `source_ref.line` ← `header_line_number` (one-based).
  - `kind` ← `SourceContentKind::Class`.
  - `payload` ← `SourceContentPayload::Class(&entry)`.

  The borrowed `entry` carries every `tokens` and `feature_blocks`
  entry verbatim — the source-IR does not interpret them, so
  no information is lost at projection time.

### SpellcastingClassEntry (B-2) → SourceContentRecord::spellcasting_class

  Direct projections (no loss):

  - `source_ref` ← `(record_source_path(), header_line_number)`.
  - `kind` ← `SourceContentKind::SpellcastingClass`.
  - `payload` ← `SourceContentPayload::SpellcastingClass(&entry)`.

  The borrowed `entry` carries the full spellcasting sub-shape
  (`casting_posture`, `automatically_known_levels`,
  `spell_progression`, `domain_selections`,
  `school_specializations`) verbatim.

### RaceDeclaration (B-3) → SourceContentRecord::race

  Direct projections (no loss):

  - `source_ref` ← `(source_path, line_number)`.
  - `kind` ← `SourceContentKind::Race`.
  - `payload` ← `SourceContentPayload::Race(&decl)`.

  `RaceDeclaration` already carries `source_path` and `line_number`
  as per-record fields, so the source-ref is fully populated.

### AbilityDeclaration (B-3) → SourceContentRecord::ability

  Direct projections (no loss):

  - `source_ref` ← `(source_path, line_number)`.
  - `kind` ← `SourceContentKind::Ability`.
  - `payload` ← `SourceContentPayload::Ability(&decl)`.

  The borrowed `decl` carries `parsed: Option<AbilityParsedFields>`
  verbatim — pipe-delimited declaration structure is preserved
  untouched.

### LstSpellRecord (B-4) → SourceContentRecord::spell

  Direct projections (no loss):

  - `source_ref` ← `(source_path, line_number)`.
  - `kind` ← `SourceContentKind::Spell`.
  - `payload` ← `SourceContentPayload::Spell(&record)`.

  The borrowed `record` carries every extracted column (`school`,
  `subschool`, `descriptor`, `components`, `casting_time`, `range`,
  `target_area`, `duration`, `save_info`, `spell_resistance`,
  `description`, etc.) verbatim. Spell continuation rows are not
  modeled in the source-IR — the parser records them as raw
  evidence on the borrowed record; future semantic conversion is
  owned by a later slice.

### EquipmentRecord (B-5) → SourceContentRecord::equipment

  Direct projections (no loss):

  - `source_ref` ← `(record_source_path(), header_line_number)`.
  - `kind` ← `SourceContentKind::Equipment` (a single tag covers both
    `EQUIP:` and `EQUIPMOD:` records; the
    `EquipmentRecordKind::Equip` vs `EquipmentRecordKind::EquipMod`
    distinction lives on the borrowed record's `kind` field).
  - `payload` ← `SourceContentPayload::Equipment(&record)`.

  The borrowed `record` carries every extracted token
  (`KEY:`, `TYPE:`, `COST:`, `WT:`, `BONUS:`, `DAMAGE:`, `CRITRANGE:`,
  `CRITMULT:`, `EQMOD:`, etc.) verbatim. Pipe-delimited `BONUS:`
  chains are preserved as a linear list of `BonusToken`s (one per
  pipe-delimited qualifier), not a recursive tree — the parser
  flattens; the source-IR mirrors that.

### LstRecord (B-6 metadata) → SourceContentRecord::metadata

  Direct projections (no loss):

  - `source_ref` ← `(record_source_path(), line_number)`.
  - `kind` ← `SourceContentKind::Metadata(metadata_kind_inner(record.kind))`.
    The inner tag is one of the six `MetadataKindInner` variants.
  - `payload` ← `SourceContentPayload::Metadata(&record)`.

  The borrowed `record` carries `name: String` (the leading token
  of the directive value) and `raw_line: String` (the verbatim
  source line) verbatim.

Lossy and partial mappings
--------------------------

This contract deliberately defines the lossy and partial mappings
the Slice E projection introduces. These are the canonical
authoritative answers to "what does the source-IR NOT preserve?"
— anything not listed here is preserved.

### Lossy mappings

  NONE today. The Slice E projection is strictly zero-copy — every
  field on every B-family entry is reachable via the borrowed
  payload. The `LossyMapping` diagnostic variant exists for future
  extensions (e.g. when the rules engine requests a structured form
  for a corpus token the source-IR cannot yet consume). At that
  point, the lossy mapping is documented here and a
  `SourceContentDiagnostic { severity: Warning, kind: LossyMapping,
  ... }` is emitted with the token's `SourceRef`.

### Partial translations

  NONE today. Every record's projected shape is total — no field is
  silently dropped, no value is summarized. The `PartialTranslation`
  diagnostic variant exists for future extensions (e.g. when a
  corpus token is preserved as raw evidence on the borrowed entry
  but not surfaced through the canonical envelope because the
  envelope has no field for it yet). When that happens, the partial
  translation is documented here.

### Forwarded malformed-record diagnostics

  When a B-family parser emits a `MalformedSD17B1`,
  `MalformedBlockMarker`, `UnleveledFeatureLine`,
  `MalformedDirective`, or similar diagnostic on a record, the
  Slice E projection forwards it as a
  `SourceContentDiagnostic { severity: Error, kind: MalformedRecord,
  ... }`. The forwarding rule is:

  - The diagnostic's `source_ref.lst_file` ← the B-family
    container's `source_path`.
  - The diagnostic's `source_ref.line` ← the parser's recorded
    line number on the offending record (one-based, or `0` for
    container-wide diagnostics).
  - The diagnostic's `message` ← the parser's diagnostic message
    verbatim.
  - The diagnostic's `severity` ← `Error`.
  - The diagnostic's `kind` ← `MalformedRecord`.

  The Slice C contract documents the converter-level forwarding
  rules (`IR_FORWARDED_B1`, `IR_FORWARDED_B2`, etc.); the Slice E
  canonicalization maps those to the source-IR `MalformedRecord`
  kind via `IRDiagnostic::to_canonical`.

Intentional non-translations
----------------------------

This section names corpus content the source-IR does NOT represent.
Every entry below stays in the PCGen corpus verbatim but does not
enter `SourcePackageContent` in any form. Future tranches may
extend the source-IR to cover some of these; until then they are
explicit non-translations.

  - **UI presentation tags.** `OUTPUTNAME:`, `OUTPUTTEXT:`, `SORTKEY:`,
    `VISIBLE:`, `FORMATCAT:`, `NAMEOPT:` and other UI-shape
    directives stay in the corpus. They are reachable via the
    borrowed parser entry's token vector but the canonical envelope
    does not surface them. (Tranche-3's UI composition may consume
    them via the borrowed entry directly; no canonical field for
    them exists in this slice.)

  - **OS-specific tokens.** `BONUS:VARMAX=...|OSTYPE=Windows` and
    similar OS-conditional qualifiers stay in the corpus. The
    parser records them as raw evidence on the borrowed entry's
    `BonusToken::qualifiers` (B-5) but the canonical envelope does
    not surface OS specifics. A `LossyMapping` diagnostic MAY be
    emitted when the rules engine requests OS-conditional
    evaluation and the corpus record carries OS specifics — until
    then this is a non-translation.

  - **Plugin-load directives.** `SOURCELONG:`, `SOURCESHORT:`,
    `SOURCEWEB:`, `SOURCEDATE:`, `SOURCEPAGE:`, `SOURCELINK:` are
    corpus bookkeeping (the corpus author's metadata about their
    own source, not gameplay-relevant). They are reachable on the
    borrowed parser entry but the canonical envelope does not
    surface them.

  - **Corpus-specific magic tags.** `BONUS:COMPANIONMOD|...`,
    `TEMPLATE:...` (when used as a self-reference), and any
    corpus-specific custom tag the B-family parser recognizes but
    does not project into a structured field on the borrowed entry
    stays as raw token evidence on the borrowed entry.

  - **Versioning meta.** `!PRERULE:` annotations on description
    fields, `KEY:!VERSION=` suffixes, and other PCGen version-gate
    markers stay on the borrowed entry. They are reachable via the
    parser's per-row token vectors but the canonical envelope does
    not surface them.

  - **Unrecognized directive prefixes.** PCGen accepts directive
    prefixes outside the B-family scope (e.g. `FEAT:`, `SKILL:`,
    `DEITY:` is in scope, `FEAT:` is not yet). When the B-family
    parser encounters an unrecognized directive, it does not
    project a `SourceContentRecord` for it. The directive stays in
    the corpus; this is a non-translation.

Versioning
----------

This contract declares:

  - `source_ir_version: u32 = 1`
  - In Rust: `pub const SOURCE_IR_VERSION: u32 = 1;` in
    `src/rules_core/source_content.rs`.

Future evolution rules:

  - Any breaking change to a public type in `rules_core::source_content`
    bumps `source_ir_version`. The change is documented in this
    artifact with a dated "version N → N+1" section.
  - Any additive change (new variant on `SourceContentKind`,
    new variant on `SourceContentPayload`, new field on a
    diagnostic struct) MAY be made without a version bump IF
    existing consumers continue to compile. The conservative
    doctrine is to bump on any change — additive changes are
    silent, but consumers cannot enumerate them without reading
    the contract.
  - Converters from older parser outputs declare compatibility
    with a specific `source_ir_version`. The converter API does
    not currently surface a version field on `IRSchema`; when a
    second version ships, the schema's `schema_version: u32` field
    will reflect the version the converter emits.

Authoring boundary
------------------

This contract does NOT define:

  - The chosen-state shape (race_id, class_levels, ability_scores,
    selected_feats, skill_allocations, equipment_selections,
    selected_choices, selection_provenance). That lives in
    `src/rules_core/character_input.rs` and is preserved
    untouched by Slice E.
  - The compute path (pilot_compute.rs, pilot_failure.rs,
    pilot_view_model.rs, support_state_matrix.rs). Those are
    read-only in this slice.
  - The rules engine's consumption of the source-IR. Tranche-3
    composes the chosen-state shape with `SourcePackageContent` to
    evaluate; that integration is explicitly NOT a tranche-2-7
    deliverable.

Verification
------------

The implementation in `src/rules_core::source_content` and
`src/pcgen_import/ir_converter.rs` conforms to this contract. The
contract is verified by:

  - `cargo test --test sd17_e_source_ir_shape` — every contract
    clause has at least one test.
  - `cargo test --test sd17_c_ir_convert` — the converter still
    works against the canonical types (45 tests, all green).
  - `cargo test --test sd17_b1_martial_class`,
    `cargo test --test sd17_b_spellcasting_class`,
    `cargo test --test sd17_b_races_and_abilities`,
    `cargo test --test sd17_b_spells`,
    `cargo test --test sd17_b5_equipment`,
    `cargo test --test sd17_b_metadata_kinds` — the B-family
    parsers are untouched and pass unchanged.

If the implementation and this contract disagree, this document
wins; the implementation is repaired.