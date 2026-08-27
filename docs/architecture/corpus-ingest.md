# Corpus Ingest

> Scope: how real PCGen corpus files (`.pcc` entry files + `.lst` data files) are parsed and projected into the canonical source-IR the rules engine consumes.
> Last verified: **2026-08-25 against `tranche/13`** (SD-33 closure epilogue) for the new §"`raw_tokens` enrichment and the corpus-literal sweep's own closure builder" section; the 2026-08-18 `tranche/11` pass for §"Provenance is per-FIELD, not per-record" (SD-31 wave 14, `SD31-W14-INTEGRATE-001`) still stands; prior pass 2026-08-07 against tranche/8 (wiring_class/PI-screening convergence cycle) — parsing-pipeline sections (Stage 1-6) re-verified structurally only; the cache-layer additions are documented in [rules-data-tables.md](./rules-data-tables.md)
> Maintenance: updated at SD closure — see [README.md](./README.md) §Maintenance contract

## Purpose

`src/pcgen_import/` turns real PCGen corpus text — `.pcc` campaign entry
files and the `.lst` object-data files they include — into the canonical
source-IR envelope (`src/rules_core/source_content.rs`) that the rules
engine consumes. The corpus itself is never vendored into this repo: it
is an external checkout of PCGen data, located by the `PCGEN_CORPUS_ROOT`
environment variable at test time. Every corpus-gated test skips gracefully
(or hard-skips via `#[ignore]`) rather than failing when the corpus isn't
present — see [testing.md](./testing.md) §"Corpus-gated tests" for the full
catalog of patterns and which one to copy for a new test.

Parsing and semantic conversion are deliberately separate stages, per
`src/pcgen_import/mod.rs`'s module doc comment. Nothing in this module
interprets PF1 rule semantics (BONUS trees, pipe-delimited qualifiers,
spell-slot math); it only recognizes directive shapes and carries their
tokens forward with source provenance.

## Pipeline stages

The pipeline runs in one direction, each stage consuming the previous
stage's output type:

```
pcc.rs                  parse_pcc_entry            -> PccEntryFile
include_resolver.rs      resolve_pcc_includes_from  -> IncludeResolution
lst_parser/<kind>.rs     parse_<kind>_entries        -> per-kind parse result
ir_converter.rs          convert_to_ir / convert_*   -> SourceContentRecord<'a>
source_content_payload.rs SourceContentPayload<'a>   (payload enum, referenced above)
rules_core::source_content SourcePackageContent<'a>  (corpus-rooted aggregate)
```

### Stage 1 — `pcc.rs`: structural include edges

`src/pcgen_import/pcc.rs`'s `parse_pcc_entry(source_path, input_text)`
walks a `.pcc` file line by line and recognizes exactly one construct:
`PCC:` include directives. Every other line (`CLASS:`, `RACE:`,
`SKILL:`, ...) is ignored at this stage — no LST semantics are
interpreted here. The result is a `PccEntryFile` carrying:

- `includes: Vec<PccIncludeEdge>` — one entry per `PCC:` directive, with
  `source_path`, one-based `line_number`, verbatim `raw_directive`, and
  the parsed `target` text.
- `diagnostics: Vec<PccDiagnostic>` — a `PccDiagnosticKind::MalformedInclude`
  record for a `PCC:` line with no target, rather than a silently
  dropped line.

### Stage 2 — `include_resolver.rs`: deterministic include graph

`src/pcgen_import/include_resolver.rs` composes `pcc::parse_pcc_entry`
(it does not shadow it) and resolves the raw include-directive text into
an actual filesystem graph. `resolve_pcc_includes_from(corpus_root,
source_pcc_path)` performs a deterministic DFS over `PCC:` edges,
resolving PCGen's `@/` and `*/` path conventions against `corpus_root`
(see `resolve_pcgen_path`), and returns an `IncludeResolution` with:

- `pcc_files: Vec<ResolvedPccFile>` — every PCC file visited, in DFS
  preorder.
- `pcc_edges: Vec<ResolvedPccEdge>` — directed include edges with
  resolved absolute `target_path`.
- `lst_files: Vec<ResolvedLstFile>` — flat LST references discovered on
  non-`PCC:` lines (any line of the form `<KIND>:<path>.lst`), each
  tagged with its directive `kind` (e.g. `CLASS`, `RACE`, `SPELL`,
  `DATACONTROL`) and the line that emitted it.
- `diagnostics: Vec<IncludeDiagnostic>` — `IncludeDiagnosticKind`
  variants `MalformedInclude` (propagated from the B-family PCC parser),
  `MissingTarget` (an include or LST reference resolves to a
  non-existent file), `CycleDetected` (a `PCC:` include points back to a
  file already on the active DFS stack — the diagnostic message includes
  the full cycle path), and `ReadFailed` (a PCC file could not be read).

Downstream parsers (Stage 3) discover which `.lst` files to parse from
`IncludeResolution::lst_files`; this module does not parse LST content
itself.

### Stage 3 — `lst_parser/`: per-kind LST parsers

`src/pcgen_import/lst_parser/mod.rs` partitions LST parsing by object
kind, one module per kind:

- `class.rs` — `parse_class_entries` recognizes `CLASS:<name>` lines for
  a fixed allowlist, `MARTIAL_CLASS_NAMES` (Fighter, Barbarian, Monk,
  Rogue, Ranger, Paladin, Cavalier, Brawler, Slayer, Swashbuckler, plus
  each name's `Ex-<name>` mirror). A class name outside the allowlist is
  skipped silently (no diagnostic) — it belongs to a different parser or
  a future widening. Output is `ClassEntry` (tokens plus `###Block:`
  `ClassFeatureBlock`/`ClassLevelLine` feature data), aggregated into a
  `ClassParseResult`.
- `spellcasting_class.rs` — the same allowlist pattern via
  `SPELLCASTING_CLASS_NAMES` (Cleric, Druid, Wizard, Sorcerer, Bard,
  Alchemist, Inquisitor, Oracle, Summoner, Witch, Arcanist, Bloodrager,
  Hunter, Investigator, Shaman, Skald, Warpriest). `parse_spellcasting_class_entries`
  additionally derives a `CastingPosture` (Prepared / Spontaneous /
  Spellbook) from `SPELLSTAT:`/`MEMORIZE:`/`SPELLBOOK:` tokens and
  harvests progression-curve and domain-selection `###Block:` rows into
  `SpellcastingClassEntry`. Both allowlists widen one class at a time as
  SD-22 ingest cycles verify each class's real `CLASS:` line shape
  against the corpus — putting a class on the wrong allowlist (martial
  vs. spellcasting) is a correctness bug the module doc comments call
  out per class.
- `race_ability.rs` — `parse_lst_entry` recognizes `RACE:`/`RACES:`
  pointer lines and `ABILITY:` declarations (pointer or full
  pipe-delimited form), producing `LstEntryFile` with `race_pointers:
  Vec<RaceDeclaration>` and `ability_declarations: Vec<AbilityDeclaration>`.
- `spell.rs` — row-shaped `SPELL:` parsing (`LstSpellRecord`), tolerant
  of both "tight TSV" and "aligned TSV" corpus layouts via a known-tag
  scan (`KNOWN_TAGS`) rather than fixed column indices.
- `equipment.rs` — `EQUIP:`/`EQUIPMOD:` row parsing (`EquipmentRecord`),
  including flattened `BONUS:` chains (`BonusToken`) so a chain with
  many pipe-delimited qualifiers still parses in O(n) without recursion.
- `metadata.rs` — the six flat metadata kinds (`MetadataKind::{Deity,
  Domain, Kits, Language, Template, CompanionMod}`), each occurrence
  becoming one `LstRecord`.
- `monster_stat_block.rs` — a bare tab-delimited row parser
  (`parse_monster_stat_block_entries`), written for SD-22 Epic 5's
  Bestiary 1 ingest because `race_ability.rs`'s `RACE:`/`ABILITY:`-only
  recognizer extracts zero records from `b1_races.lst` (monster rows
  there have no directive prefix — the name is the unprefixed first tab
  field). A row qualifies only if it carries a `CR:` token (rows
  without one, like the bare `Skeleton`/`Zombie` template-shim rows, are
  skipped without a diagnostic) and is not a `.MOD`/`.COPY=` override
  row. **This parser is not wired into `ir_converter.rs` or
  `SourceContentPayload`** — there is no `MonsterStatBlockRecord`
  variant on either enum, and its only caller in the repo is the
  parser's own test suite (`tests/sd17_b_monster_stat_block.rs`). Its
  output is read and hand-transcribed into `rules_tables` book modules
  rather than flowing through the canonical-IR projection path
  automatically (see [rules-data-tables.md](./rules-data-tables.md)'s
  hand-transcription convention).

Every per-kind parser's outputs are reachable through one kind-tagged
union: `ParsedLstRecord<'a>` (`src/pcgen_import/lst_parser/mod.rs`,
canonical home; re-exported from `src/pcgen_import/mod.rs` and from
`ir_converter.rs` for backward compatibility). Its seven variants —
`Class`, `SpellcastingClass`, `Race`, `Ability`, `Spell`, `Equipment`,
`Metadata` — each borrow (`&'a ...`) the corresponding B-family entry
type. `monster_stat_block.rs`'s `MonsterStatBlockRecord` has no
`ParsedLstRecord` variant, consistent with it sitting outside the
canonical-IR pipeline.

### Stage 4 — `ir_converter.rs`: canonical projection

`src/pcgen_import/ir_converter.rs` is the canonical projection path. Its
public entry point, `convert_to_ir(parsed_record: &ParsedLstRecord<'a>,
_schema: &IRSchema) -> SourceContentRecord<'a>`, is a total,
enum-discriminated trampoline over seven per-family converters
(`convert_class_entry`, `convert_spellcasting_class_entry`,
`convert_race_declaration`, `convert_ability_declaration`,
`convert_spell_record`, `convert_equipment_record`,
`convert_metadata_record`) — every `ParsedLstRecord` variant has exactly
one canonical envelope shape; there is no rejection path at this stage.
Per-document converters (`convert_class_parse_result`,
`convert_lst_entry_file`, `convert_spell_file`, ...) and corpus-rooted
`convert_package_from_*` builders wrap the per-record converters to
consume a whole B-family parse-result container in one O(n) pass,
accumulating a `SourcePackageContent` plus a forwarded-diagnostics
vector.

`IRSchema::canonical_v1()` describes (not enforces) the directive-token
vocabulary the schema recognizes; `IRSchema::recognizes` is advisory,
not a filter the converter itself applies.

`IRDiagnostic` (converter-side; distinct from the canonical
`SourceContentDiagnostic`) is reshaped by `IRDiagnostic::to_canonical`:
codes prefixed `IR_FORWARDED_*` (a diagnostic forwarded verbatim from a
B-family parser) map to `SourceContentSeverity::Error` +
`SourceContentDiagnosticKind::MalformedRecord`; every other
converter-originated code maps to `SourceContentSeverity::Info` +
`SourceContentDiagnosticKind::PartialTranslation`.

### Stage 5 — `source_content_payload.rs`: the payload enum

`SourceContentPayload<'a>` (`src/pcgen_import/source_content_payload.rs`)
is the typed, kind-tagged union of borrowed B-family entries
(`Class(&'a ClassEntry)`, `SpellcastingClass(&'a SpellcastingClassEntry)`,
`Race(&'a RaceDeclaration)`, `Ability(&'a AbilityDeclaration)`,
`Spell(&'a LstSpellRecord)`, `Equipment(&'a EquipmentRecord)`,
`Metadata(&'a LstRecord)`) that lives behind every
`SourceContentRecord`.

Its own doc comment explains why it lives in `pcgen_import` rather than
in `rules_core::source_content`, where the rest of the canonical
envelope lives: the variants reference parser entry types from
`pcgen_import::lst_parser::*`. If the enum lived in
`rules_core::source_content` instead, the import graph would cycle —
`pcgen_import::ir_converter` already constructs `SourceContentRecord`
and would need to import from `rules_core::source_content`, which would
in turn need parser types from `pcgen_import`. Keeping the payload enum
beside the parser surface keeps the dependency one-directional:
`rules_core::source_content` re-exports the enum
(`pub use crate::pcgen_import::source_content_payload::SourceContentPayload;`),
and the only thing crossing the boundary is the finished envelope
`pcgen_import::ir_converter` builds. The module also carries the total,
mechanical `MetadataKind` <-> `MetadataKindInner` mapping
(`b6_metadata_kind_to_canonical` and its inverse) for the same reason.

### Stage 6 — `rules_core::source_content`: the canonical envelope

`src/rules_core/source_content.rs` defines the rest of the envelope that
the rules engine eventually consumes:

- `SourceRef { lst_file: String, line: u32 }` — the provenance anchor
  every record and diagnostic carries.
- `SourceContentKind` — a tag mirroring the seven payload variants, with
  `Metadata(MetadataKindInner)` distinguishing the six metadata kinds
  under one shared payload variant.
- `SourceContentRecord<'a> { source_ref, kind, payload }` — one record
  per LST directive.
- `SourceContentDiagnostic { severity, kind, message, source_ref }` —
  the projection-side diagnostic surface (distinct from converter-side
  `IRDiagnostic`).
- `SourcePackageContent<'a> { package_id, source_ref, records, diagnostics }`
  — the corpus-rooted aggregate; `records_by_kind` returns a
  deterministically ordered (sorted by `(lst_file, line)`, ties broken by
  insertion order via a stable sort) filtered `Vec`.
- `SourceContentLoadResult<'a> { content: Option<SourcePackageContent<'a>>, diagnostics }`
  — the top-level load result; `content` is `None` only when projection
  hit a blocking error.

## Zero-copy / borrowed design

Every `SourceContentPayload` variant is a borrow, never an owned clone
of the underlying parser entry — `SourceContentRecord<'a>` and
`SourcePackageContent<'a>` are lifetime-parameterized over the B-family
parse-result container that produced them. Per-record conversion
(Stage 4) is O(1); per-document conversion is O(n) in record count; the
whole pipeline never clones a parsed entry on the hot path. Consumers
that need to own a projected record clone the underlying entry
explicitly — the canonical-IR surface itself never does.

For contributors, this means: the B-family parse-result container (e.g.
`ClassParseResult`, `LstEntryFile`) must outlive every
`SourceContentRecord`/`SourcePackageContent` built from it. Code that
tries to return a `SourcePackageContent<'a>` from a function that owns
the parse result locally will not compile — the parse result has to be
kept alive by the caller for as long as the projected records are used.

## Diagnostics posture during ingest

Diagnostics accumulate at every stage rather than aborting the parse.
The canonical `SourceContentDiagnosticKind` (`src/rules_core/source_content.rs`)
has four variants, each with a fixed severity via its constructor:
`MalformedRecord` (`SourceContentDiagnostic::malformed`, `Error` — a
malformed-record diagnostic forwarded from a B-family parser; the
consumer must treat the record as absent), `LossyMapping`
(`::lossy_mapping`, `Warning` — part of the content was preserved as a
raw token string rather than a structured form), `UnsupportedToken`
(`::unsupported_token`, `Warning` — a directive/value token the corpus
supports but the source-IR does not currently recognize), and
`PartialTranslation` (`::partial_translation`, `Info` — known fields
are populated; unknown fields remain on the underlying entry but are
not surfaced in the canonical shape).

Every diagnostic carries a `SourceRef`, so a diagnostic can always be
traced back to the exact LST file and line that produced it — including
container-level diagnostics with no specific line, which anchor to
`line == 0` as the canonical placeholder (see
`IRDiagnostic::to_canonical`'s doc comment).

## Adding support for a new record kind

To add a seventh (or eighth) B-family record kind end to end, touch, in
order:

1. `src/pcgen_import/lst_parser/<new_kind>.rs` — new parser module,
   producing a parse-result struct and an entry struct with source
   provenance (`source_path`/`line_number` or a container-level
   equivalent), following the existing per-kind modules' shape.
2. `src/pcgen_import/lst_parser/mod.rs` — register `pub mod <new_kind>;`,
   re-export the new entry type, add a `ParsedLstRecord::<NewKind>(&'a NewKindEntry)`
   variant and a `from_<new_kind>` convenience constructor.
3. `src/pcgen_import/source_content_payload.rs` — add a matching
   `SourceContentPayload::<NewKind>(&'a NewKindEntry)` variant, and wire
   it into `kind_token()` and `source_slice()`.
4. `src/rules_core/source_content.rs` — add the matching
   `SourceContentKind::<NewKind>` variant, and wire it into `token()`
   and `source_slice()`.
5. `src/pcgen_import/ir_converter.rs` — add a `convert_<new_kind>_entry`
   per-family converter, wire it into `convert_to_ir`'s match, and add a
   `forward_<new_kind>_diagnostics` helper plus a per-document/
   corpus-rooted converter if the new kind's parser groups records into
   a document container.
6. If the new kind needs its own include-graph discovery convention
   (a new PCC directive prefix), extend
   `src/pcgen_import/include_resolver.rs`'s LST-reference recognizer —
   otherwise the existing generic `<KIND>:<path>.lst` scan already
   covers it.

## Provenance is per-FIELD, not per-record (new 2026-08-18, SD-31 wave 14)

`shape_b_v1::CorpusRecordV1` carries **two** provenance slots, and they answer
different questions:

* `source: CorpusSource` — where the RECORD came from. For a repo-resident
  cache record this is normally `lst_token` (a pinned `path`/`sha256`/`line`
  into the oracle), or one of the two honest variants
  `lst_corrected_ingest` / `lst_inherited_copy`.
* `description_source: Option<CorpusSource>` — where the record's DESCRIPTION
  came from, when that differs. Populated only where it genuinely differs.

The split exists because SD-26 `decisions.md §11.2` made `source` a
discriminated union to record the provenance of the FIELD each intake cycle
was closing, and for 412 already-shipped equipment records that field was the
description alone: their identity, `cost_gp` and `weight` were generated from
real `KEY:`/`COST:`/`WT:` tokens, while the prose came from a web second
source because APG's three equipment `.lst` files carry **zero** `DESC:`
tokens. Those records were stamped `web_second_source` outright, which put
them **outside `corpus_literal_sweep`'s population entirely** (it walks
`lst_token` + `raw_tokens`), so nothing had ever byte-compared them against
the oracle.

`rules_core::cache_gen::lst_provenance_repair` (driven by
`bin/repair_lst_provenance`, `--check` for a dry run) narrows such a record:
it resolves the real row with `equipment_gap::find_citation`, verifies it
against the closure `corpus_literal_sweep::token_closure` itself builds,
**refuses** unless every claimed `cost_gp`/`weight` is numerically stated by a
`COST:`/`WT:` token in that closure, moves the web citation intact to
`description_source`, and refreshes the record's `wiring_class` from the row
it has just cited (a record that had no citation legitimately read
`ambiguous`/`no_corpus_line`; keeping that stamp after narrowing would be a
self-contradiction). Refusals are reported by name and the record is left
alone — two records currently refuse (`hammer_ricochet`, whose cited row is a
`.COPY=` declaration, and `rag_armor_dark_creeper`, whose identity matches no
row).

**Known gap, and the operating rule that follows from it.** Neither
`cache_gen::apg::generate_equipment` nor
`gen_core_rulebook_cache::equipment_source` emits the narrowed shape yet —
both still stamp `web_second_source` and neither knows the
`description_source` key — and no `verify.sh` stage runs either generator.
Re-running one therefore REVERTS the narrowing. `tests/sd31_lst_provenance_
repair_is_durable.rs` makes that a red gate rather than a silent regression,
and the standing rule until the generators are taught the shape is: **after
running either equipment cache generator, re-run
`cargo run --locked --bin repair_lst_provenance` before committing.**

## `raw_tokens` enrichment and the corpus-literal sweep's own closure builder (SD-33)

Every book's equipment codegen pipeline evolved independently (CRB reads a
hand-curated static table; APG/ACG/Bestiary use their own pre-compiled
tables with a `weight` field name instead of CRB's `weight_lbs`; ARG/PU
parse raw LST directly) — but every Shape B v1 equipment record, regardless
of pipeline, already carries an exact citation back to its real PCGen LST
source line (`source.path` + `source.line`, a `lst_token`-kind source).
`src/bin/enrich_equipment_raw_tokens.rs` uses that citation directly: it
re-parses the cited raw LST file, finds the record whose header line matches
`source.line`, and adds `raw_tokens`/`raw_bonus_chains` keys onto the
on-disk JSON's `data` object — without touching any other field. It
deliberately operates on raw `serde_json::Value`, never a typed Rust struct:
an earlier version deserialized into a typed cache struct and re-serialized
the whole record, silently dropping every field that struct didn't know
about (APG/ACG/Bestiary's `weight`, PU's `equip_type`/`plus` — a real,
caught-before-commit data loss). Records whose `source.kind` is not
`lst_token` (a `web_second_source` or `same_book_fallback` record — no raw
LST line to enrich from) are left untouched and counted separately, not
treated as an error.

`src/rules_core/corpus_literal_sweep.rs` (see above, "the closure, not the
base row alone, is the correct comparand") is the independent verifier that
byte-compares those populated `raw_tokens` against its own `.MOD`-chain
closure derived from the pinned oracle. Two real defects in the sweep's own
closure builder, not in the enriched data, were found and fixed once
`enrich_equipment_raw_tokens.rs` populated `raw_tokens` corpus-wide and gave
the sweep something non-vacuous to check:

1. **`copy_base_row` resolved a `.COPY=` base by walking the whole book in
   `std::fs::read_dir`'s own unsorted, filesystem-order-dependent order**
   (affected 9 of 10 mismatching records). A same-named-but-structurally-
   different row (e.g. a weapon-proficiency-list definition carrying only
   `TYPE:`, no `COST:`/`WT:`/`DAMAGE:`) living in a *separate* file in the
   same book could win the old book-wide "first match" race ahead of the
   real base row that lives in the *same* file as the citing `.COPY=` row.
   Fixed: `copy_base_row` now checks the citing record's own file first,
   always, falling back to the rest of the book (sorted, for determinism —
   matching `wiring_class::build_mod_index`'s existing precedent) only when
   no same-file base exists.
2. **`compare_tokens`'s blacklist-rescreen exemption unconditionally
   excluded `DESC`** (1 of 10 mismatching records). PI screening on `DESC`
   applies independently of whether a record's own `license`/`pi_field`
   declare a redaction — so an undeclared-but-correctly-redacted `DESC:`
   token (protecting real PI the same mechanism already protects elsewhere)
   was reported as a false mismatch. Fixed: the exemption now covers `DESC`
   too, checked after the `codex_generated_name` branch rather than folded
   into it.

Neither fix touches `data/corpus/**` or `enrich_equipment_raw_tokens.rs` —
both hand-checked records' `raw_tokens` were already byte-correct; the
defect was entirely in the sweep's own reconstruction of the comparand.
`cargo run --locked --bin corpus_literal_sweep` is a `scripts/verify.sh`
stage (`corpus-sweep`); see [testing.md](./testing.md).

See [rules-data-tables.md](./rules-data-tables.md) for what happens
downstream once a corpus record is projected: transcribing its values
into the hand-authored `rules_tables` book modules, and — new as of the
wiring_class/PI-screening convergence cycle — the GE-01 `wiring_class`
taxonomy every corpus record now carries (`src/rules_core/wiring_class.rs`,
determined from a unit's full token closure, not the base row alone),
`Trap::WiringClassMismatch` (`src/pcgen_import/corpus_traps.rs`) which
guards that stamp against drift, and the shared PI-screening pass
(`src/rules_core/pi_screening.rs`) every JSON-cache writer now runs
through. See
[rules-engine.md](./rules-engine.md) for how the rules engine consumes
`SourcePackageContent` once corpus content is wired into compute, and
[testing.md](./testing.md) for the corpus-gated test conventions beyond
the graceful-skip pattern shown above.
