---
canonical: true
owner: operator
bundle_id: SD-36
date: 2026-09-15
---

# SD-36 Technical Design

Architectural rationale for the three major moves: dashboard freeze, crate wall, and bloat cuts.

---

## §1 — Dashboard freeze and producer retirement

### Problem

The PF1e status dashboard and all supporting machinery (work inventory bins, support state matrix, reach gate, desktop bridge) exist only to provide one feedback surface: "How complete is the content for each book?" That question was answered at **100% complete** (49,450 of 49,450 units) by the end of SD-35.

The supporting machinery has grown:
- `v06_work_inventory.rs` — 33,091 lines
- `support_state_matrix.rs` — 7,475 lines
- `reach_gate.rs` (desktop) — 8,846 lines
- Desktop bridge code — ~1,500 lines
- Cron jobs and dashboard renderer — ~2,000 lines
- Test overhead — ~2,500 lines
- **Total: 55,827 lines to maintain for a static answer.**

### Solution

Freeze the status page at 100% complete. The page (`site/status.html`, `site/status-data.json`) is regenerated once per bundle closure and never updated in live work. This:
1. Deletes the 55,827 lines of supporting code.
2. Removes the per-cycle "denominator" question from the engine.
3. Simplifies the public interface.
4. Keeps the frozen status for historical reference and user communication.

### Enforcement

New gate `scripts/site/check_frozen_status.py` asserts:
- Denominator is exactly 49,450 units (every unit in the DONE statuses from `docs/work-inventory.json`).
- `overall.pct == 100.0`.
- All partial and not_started counts are 0.
- `generated_at` timestamp is constant.

The gate runs per cycle and after closure before publication.

---

## §2 — PCGen crate wall

### Problem

PCGen machinery (converter, oracle, tool binaries, tool tests) exists only for content ingest. It is never used by the desktop app or the rule engine at runtime. However:
1. It is scattered across `src/pcgen_import/`, `src/oracle_validation/`, tool bins in `src/bin/`, and tool tests in `tests/`.
2. The desktop app accidentally links it via shared `src/` structure (no explicit boundary).
3. Gate blind spots make it hard to enforce "nothing of PCGen in live code."

### Solution

Move all PCGen machinery into dedicated `crates/codex-ingest`. Structure:

```
crates/codex-ingest/
├── Cargo.toml           (no dep on desktop app; depends on codex)
├── src/lib.rs           (exports pcgen_import, oracle_validation, bar_check)
├── src/pcgen_import/    (moved from src/)
├── src/oracle_validation/
└── tests/               (moved from tests/; ~82 suites)

Workspace root:
├── Cargo.toml           (adds members = ["crates/*"])
└── apps/desktop/src-tauri/Cargo.toml  (codex-ingest ONLY in [dev-dependencies])
```

### Enforcement

**Build gate (crate-wall stage):**
1. `cargo tree --locked -e normal,build` in desktop shows 0 lines starting `codex-ingest ` (build-deps only).
2. `cargo metadata --no-deps` shows `codex-ingest` depends on `codex`; never the reverse.
3. Desktop manifest awk: `codex-ingest` appears only under `[dev-dependencies]`.
4. `python3 scripts/pcgen_residue_gate.py --check` exits 0.

If any check fails, the build stops. The gate cannot be bypassed.

**Type-identity trap mitigation:** If `codex` dev-depends on `codex-ingest` (forbidden), Rust compiles `codex` twice: once for the lib and once for the bin/test, with different `CARGO_MANIFEST_DIR` values. The ingest module sees inconsistent symbols. The `cargo metadata` check detects this.

---

## §3 — Bloat cuts: source refactor and test rewrite

### Problem

Two files have grown to maintenance hazard size:
- `src/rules_core/pilot_compute/mod.rs` — 88,828 lines (one file)
- Tests with table-driven logic — ~75,000 lines (2 families, 2,219 entries)

Both are structurally sound but hinder readability and IDE performance.

### Solution

#### Pass C1: Split `pilot_compute/mod.rs`

Move logic into ~36 submodules by class/system:
- `class_*.rs` — ~12 class-specific modules, ~3–6k lines each
- `race_seams.rs`, `combat.rs`, `feat_pillars.rs`, `skills_and_saves.rs`, `spellcasting.rs`, `pool_groups.rs`, `companion.rs`, `untabled_base_class_features.rs`, `prestige_class_features.rs`, `class_dispatch.rs`, `class_unchained.rs`
- `mod.rs` → ~4,000 lines (re-exports via `pub use x::*;` to keep all 62 call sites working unchanged)

No logic changes, only moves. Largest submodule ≈ 6,600 lines.

#### Pass C2: Table-driven test rewrite

Consolidate test families (`tests/sd18_widening/`, `tests/sd13_progression/`) into:
- Roster file + per-row module files (structure unchanged for `--list` identity).
- New `rows.rs` with `const ROWS: &[Row]` per family.
- Macro (`paste!`) emitting one `#[test]` per row.
- Bespoke tests (~770) remain verbatim.
- Result: ~75,000 lines → ~26,000 lines (entry count unchanged: 2,219 / 8,723 total).

### Enforcement

1. **C1 proof:** `cargo test --locked -- --list` byte-identical (modules split but re-exported; all paths work).
2. **C2 proof:** `cargo test --locked -- --list` diff artifact showing same entries (table-driven rewrite proves test structure preserved).
3. Both passes: Full `bash scripts/verify.sh` green.

---

## §4 — Path helper consolidation

### Problem

Eight locations each define `repo_root()`, `corpus_root()`, and related path fns independently:
- `src/bin/` (~3 copies)
- `src/rules_core/` (~2 copies)
- `crates/codex-ingest/` (added in A; if defined locally, would be 9 copies)
- `apps/desktop/` (~3 copies)

Each has subtly different error handling or assumptions.

### Solution

New `src/support/paths.rs` with canonical definitions + unit tests:
```rust
pub fn repo_root() -> PathBuf { /* single canonical impl */ }
pub fn corpus_root() -> PathBuf { /* single canonical impl */ }
pub fn corpus_root_if_set() -> Option<PathBuf>
pub fn pcgen_corpus_root() -> PathBuf
pub fn find_json_files(dir: &Path) -> Vec<PathBuf>
```

All eight locations updated to `use codex::support::paths::*;` instead of local definitions.

### Enforcement

Gate: `git grep -w 'fn repo_root' | wc -l` exits with count = 1 (only in `src/support/paths.rs`). Same for `corpus_root` and `find_json_files`.

---

## §5 — Clippy enforcement

### Problem

The build does not enforce `deny(warnings)` globally. Lint warnings accumulate; no systematic pressure to fix them.

### Solution

Add `-- -D warnings` to `clippy_one_crate()` function in `scripts/verify.sh`. This makes clippy failures (warnings) exit non-zero, so CI catches them. Run at Epic C1 end; baseline set to 0 warnings.

### Enforcement

Gate: `bash scripts/verify.sh` runs clippy stages (one per crate after C1); any warning = non-zero exit. Baseline `BASELINE_CLIPPY_WARNINGS_*` set to 0.

---

## §6 — Epic F: class completion architecture

### Problem

A permanent census instrument measured the engine's true class coverage corpus-wide (61 base-type
ids + 74 prestige-type ids = 135 distinct class ids, across every registry
`compute_class_chassis`'s dispatch chain reads, not only the 31-id desktop Create picker's own
registry): **42 of 135** reach `HeadlessReceiptStatus::Computed` at every swept level. The real
remainder is not "Core Rulebook only" — it spans untabled exotic classes blocked solely on a
missing weapon-proficiency answer, 56 chassis-bearing prestige classes with no gate arm checking
their chassis at all, and a corpus-wide link defect that drops class-feature grants (including
proficiency, class skills, languages) before the converted path ever sees them. Full measurement:
`docs/release/SD-36-consolidation/artifacts/epic-f/docs-truth/class-census.md`; full plan:
`epic-f-class-completion.md`.

### Solution — six architectural pieces, each read-not-invent (paper-sheet doctrine)

**1. Census instrument** (`src/rules_core/class_census.rs`, `src/bin/class_census.rs`). Merges
every class registry the dispatch chain reads into one `BTreeMap<slug, Row>`; sweeps base classes
alone (level 1..=max) and prestige classes only in a deterministic carrier mix (never alone, for
the Computed column — a second `alone_status` column asserts all 74 of 74 prestige ids are
`Blocked` with a named game-rule diagnostic, a negative control). The carrier is `wizard`/`cleric`
by the gate's caster term, `fighter` otherwise, with the `carrier + max_level <= 20` cap taking
precedence over an unmet numeric requirement when the two conflict; the two prestige ids needing
both an Arcane and a Divine caster term (`mystic_theurge`, `evangelist`) get a second, independent
carrier rather than picking one arbitrarily. A row's Computed status is therefore a property of
the class in a legal build, not an artefact of which carrier the census happened to pick — a new
`carrier` column names it, and a row whose chassis expression references caster level without a
named caster carrier reports `Unknown`, never a confidently-wrong 0. Gains `--sheet-dump <dir>`
and `--only <class>` for F1b's headless whole-character render. Baselines can only rise
(`BASELINE_CENSUS_IDS=135`, `BASELINE_CENSUS_COMPUTED=42`, `BASELINE_CENSUS_MIX_COMPUTED=<measured,
never guessed>`, `BASELINE_CENSUS_PRESTIGE_ALONE_BLOCKED=74`), and the merged id set is itself
pinned to `status.md`'s own published partition (31+3+20+7+74) before the instrument may move.

**2. Resolver fix** (`crates/codex-ingest/src/pcgen_import/sheet_rule/{prereq,ctx}.rs`). The
converter's `resolve_rule(category, name)` looks up `(category, key)` literally; when a category
is a CHILD `ABILITYCATEGORY` (e.g. "Wizard Class Feature", parent "Special Ability"), the record is
indexed under the parent and the literal lookup misses. Fix: on a miss, retry with the parent
(child->parent map built from the oracle's own `ABILITYCATEGORY` rows already in the closure tree),
keeping KEY-exact matching (never a name-similarity guess). Closes 4,456 of 11,925 unresolved
references (option A, `decisions.md §12`). Carries the AUTO grant's PRE-gate onto the emitted
effect (`Effect::GatedFactGrant { fact, when }`) rather than discarding it (`let _ = when;` today)
— the engine evaluates `when` at effect-application time (fixpoint step 3), never approximating an
undecidable gate as granted.

**3. WeaponSet-at-ingest** (`convert.rs` + `closure.rs`). Weapon-group membership (e.g. Samurai's
katana/naginata/wakizashi tier) lives on the oracle's weapon-PROFICIENCY rows, not on the 19
converted record kinds; adding a `weapon_proficiency` kind would move the frozen 49,450-record
count. Instead: resolve membership at ingest from a read-only index over the oracle's
`*_profs_weapon.lst` rows (never a record), store it on the GRANTING rule as
`ProfRef::WeaponSet { label, members }`. Record count stays 49,450; the delta lives inside
existing rules' `grants`.

**4. Proficiency reader** (`src/rules_core/pilot_compute/class_proficiency_sheet_rules.rs`, new,
sibling of `class_chassis_sheet_rules.rs`). Builds a `HeldSeed` for one `(class, level)`, runs the
existing `held_set` fixpoint, collects `Effect::FactGrant(Fact::Proficiency(..))` from the held
rules (the converted path does NOT fold `FactGrant` into `CharacterFacts` — the reader does its own
collection). `weapon_tables::class_weapon_proficiency`'s 42 hand-pinned rows keep first precedence
(ruling 7: no `rules_tables` move before Starfinder); the reader is the fallback for every other
class (`decisions.md §13`: read the converted record, never author ~93 new Rust rows). Returns
`Some(empty)` only when the class's `closure_complete` flag is true (no unresolved reference in its
grant closure carries a weapon grant, and no unfindable reference at all); otherwise `Unknown` —
never a fabricated "proficient with nothing". The converted vocabulary itself already carries junk
tags (`Auto`, `KoboldTailAttachment`) that match no tier, weapon group, or `WeaponSet` — every row
the reader returns must be re-derivable from a named oracle row, not only the 42 hand-pinned static
rows; an unrecognized tag makes that class's answer `Unknown` too. Requires a process-wide, lazily-loaded
`SheetRulePackage` handle at the `rules_core` layer (mirroring the desktop's own
`character_hub.rs` `OnceLock` precedent, one layer too high for `rules_core`'s pure functions to
reach today) with a named `Err` fallback, never a silent empty package or a panic.

**5. Print-path reconciliation rule** (F1b, `class_shared_core.rs:40-52`
`with_sheet_rules`/`reconcile_sheet_lines`). Option A's corpus-wide link repair surfaces
previously-unresolved class-feature/race-trait/ability rules onto the converted print path for the
first time — sibling-amplified well beyond the raw 4,456 edge count (measured offline before the
population run, not assumed). A `rule_for_explanation(package, class_slug, explanation_id) ->
Matched | Ambiguous | None` join (longest-common-prefix in whole underscore-joined words, minimum
`class_slug` + one feature word, explicit refusal of the bare class-principal rule id) resolves a
bespoke facet id to its converted rule id, so a duplicate line is caught by identity (one rule id
prints once, `held_set` already guarantees this) and a numeric disagreement is caught by a
TEST-ONLY population assertion (`tests/sd36_sheet_value_agreement.rs`, 0 disagreements required
before shipping) — never a runtime "who wins" branch, since the doctrine requires nothing left to
reconcile live on a shipped sheet.

**6. Gate arm** (`class_shared_core.rs` `is_supported_generic_class_family_single_class`,
`has_supported_class_chassis`). A new arm covers the `generic_class_chassis` registry's 78 class
records (56 prestige-tagged, 22 already covered by an earlier arm), excluding `Prestige`-tagged
records from the Computed gate. Paired with a claim-blocking `prestige_class.requires_base_class_
levels` diagnostic for a prestige class taken alone (no base-class levels) — chassis numbers are
never emitted for that case. Falsifiable acceptance: census `computed == 42` of 135 BOTH before and
after this arm lands, since the 22 non-prestige ids already reach Computed through an existing arm
(a rise would mean a double-count bug, not progress).

**7. Multiclass fold** (`class_occult_and_psionic.rs` `multiclass_class_level_supported`,
`multiclass_good_saves` — corrected: this function lives in `class_occult_and_psionic.rs:3808`,
not `class_shared_core.rs`, which ends before the line originally cited). One generic fold, zero
per-class rewrites: BAB
(sum), saves (fractional), HP (per-class hit die x levels + Con), skill points (per-class ranks x
levels + Int), class-skill union and weapon-proficiency union computed once for the character;
class-feature text, spell slots/caster level and per-class pools taken verbatim from each class's
isolated single-class run, re-scoped `multiclass.<class>.<original id>`. HP and skill-point figures
require two new `ClassChassis` readers (`hit_die: Option<u8>`, `skill_ranks_per_level:
Option<u8>`, both parsed from the converted `StatBlock` prose — same family as the entry-
requirements gate already read in F0) built as an F0/F1 prerequisite, not F3 or F4: a class with
`None` for either field reports `Unknown`, never a silently-zeroed total. Good/poor save
derivation reads a new public `ClassChassis::save_shape(index) -> Option<SaveProgression>`
accessor (`class_chassis_sheet_rules.rs`, alongside the private `saves` field it exposes), with
explicit `Degraded`/`Unrecognized` arms so a class whose converted save formula did not survive
conversion cleanly (a known failure mode: it prints as words, not an `Expr`) stays `Blocked` in
a mix rather than being silently folded in as `poor`.

**8. Class creation roster** (`apps/desktop/src-tauri/src/character_hub.rs`
`list_class_creation_roster`, mirror of `list_race_creation_roster`). A class is offered iff the
census says Computed at every level AND its `hit_die` reader returns `Some` — engine-derived, so
the picker can never again offer an uncomputable class, or a Computed class with no HP figure to
print (7 of 185 class records carry no hit-die prose row; 4 of the 7 are generic-family classes
that can otherwise reach Computed). Every excluded class carries a NAMED reason
(`hit_die_absent | not_computed | prestige | ex_state`), never a bare boolean, so the census and
the roster can never silently diverge. Prestige classes appear only in level-up, with their
printed entry requirements and met/unmet note (`decisions.md §14`). Ex-* states (Ex-Barbarian,
Ex-Paladin, and `ex_antipaladin` if distinct) are census-only, never offered at creation — they
are reached only through the game's own fall-from-grace mechanic. Reads the desktop's existing
process-wide package handle, not a fresh 135x20-receipt census sweep at picker-open time.

### Enforcement

1. **Census gate:** `bash scripts/verify.sh --only class-census` green; baselines in
   `scripts/verify-baselines.env` can only rise.
2. **Converter structural diff:** same file set, same rule-id set, every rule's JSON minus
   `granted_by`/`grants`/`closure_complete` byte-identical; `records 49450 -> 49450`; added edges
   per kind matches the mechanism-A table exactly or the difference is explained row by row.
3. **Print-path population test:** `cargo test --locked --test sd36_sheet_value_agreement` — 0
   disagreements across every census class at levels 1/10/max — is a hard gate on the F1
   population commit landing at all.
4. **Residue and frozen-status gates stay 0 of 0 / green** through every Epic F batch:
   `python3 scripts/pcgen_residue_gate.py --check --closure`; `python3 scripts/site/
   check_frozen_status.py --check`; `git status --porcelain -- data/corpus site | wc -l` -> 0.
5. Full detail, every acceptance command, every RED-first test and the adversarial review log:
   `epic-f-class-completion.md`.

---

