---
canonical: true
owner: prose mapping lane (SD-35 Epic 2 token-mapping)
bundle_id: SD-35
date: 2026-09-07
head: tranche/15 (measured against the live docs/work-inventory.json and data/corpus on 2026-09-07)
---

# `prose` family — display and metadata tokens → `SheetRule.prose` / `label` / `provenance`

Companion table: `prose.json` (21 rows, one per token type or per ASPECT sub-key class). Every
count below states its denominator. Two denominators recur:

- **ND** = 21,722 non-DONE units joined to a corpus record (live inventory 2026-09-07; 35 books,
  `core_rulebook`/`ultimate_campaign`/vehicle excluded; 21,725 non-DONE, 3 unjoined spells). This
  lane's own join, same method as TOKEN-MODEL (2026-08-31: 22,369 / 22,366). The gap is DONE
  movement since the measurement, not a method change.
- **CW** = 51,474 `data/corpus` records (39 book dirs, recursive walk, `_parity` excluded);
  48,673 have a non-empty `raw_tokens` list, 2,768 have no field, 33 have an empty list.

## 1. The family's shape in one page

**The family is 58% of every token instance (an estimate inherited from the fable review's TOKEN-MODEL, 2026-08-31 — this lane restated no instance denominator for it; the lane's own denominator is ND 21,722 units, or CW 51,474 records, above) and has exactly one rule that
matters: how a `DESC:` segment becomes words.** TYPE/KEY/SOURCEPAGE/VISIBLE/SORTKEY/SOURCE\* are provenance or
flags; ASPECT/BENEFIT/SPROP/SAB/TEMPDESC are DESC with a different label. The converter's prose
pass is therefore one function applied to five token kinds, plus a per-sub-key table for ASPECT.

**The rule that turns a closure into `prose` (stated once, application order):**

1. **Rows, not `raw_tokens`.** Read the closure as rows via `wiring_class::token_closure_rows`
   and apply PCGen order: `.COPY=` base row → the record's own row → every `.MOD` row in file
   order. `data.raw_tokens` stores base, copy-base, mods, with no row boundaries — `.CLEAR`
   cannot be applied to it. Two closure defects the converter must not inherit (§3, case 1).
2. **Accumulate per family** (DESC, BENEFIT, SPROP, SAB, TEMPDESC, and ASPECT per sub-key):
   `.CLEAR` empties the family's list so far; `.CLEARALL` likewise; a redacted value
   (`[redacted PI]`) contributes nothing and stamps provenance.
3. **Per token**: split on `|`; pop trailing `!?PRE[A-Z0-9]+:` gate segments; N = highest `%N`
   referenced in the first segment (`%%N` counts as a candidate); the last N segments are the
   arguments, taken from the right (a ` | ` table separator inside prose survives); with N = 0
   strip one trailing tight `|tag`.
4. **Arguments**: integer literal → `Const`; bare name with a same-row `DEFINE:X|<int>` →
   `Const` (3,936 of 5,220 DESC bare-var args CW); `Name±int` on a same-row literal → `Const`;
   anything else (1,284 external names + 1,538 formulas CW, e.g. `max(MesmeristLVL/2,1)`) → a
   **slot** holding the formula lane's `Expr`; `%CHOICE`/`%LIST` → a Choice slot (§3, case 3).
   **Nothing is dropped.** Dropping is what `render_pcgen_desc` does today and is exactly what
   `decisions.md §1` rule 1 forbids ("DC 15", not a hole).
5. **Escapes**: `%%` → `%` except `%%N` where argument N exists (4 upstream rows); bare `%` kept
   only after a digit or as `d%`; otherwise it is an unresolvable placeholder → the record is
   `Refused { token_type: "DESC:bare-percent" }` (31 CW rows). Entities `&nl; &lbracket;
   &rbracket; &pipe; &comma;` decode; **`&colon;` must be added** (TEMPDESC uses it).
6. **Gates**: a gate whose operands are same-row literals is decided at convert time (segment
   kept or discarded); any other gate becomes that **segment's** Applies (gap G2). `!PRERULE:1,
   DisplayFullSpell` marks the spell one-line summary (gap G4).
7. **Join**: surviving segments per family joined with one space, families in the order DESC,
   BENEFIT, SPROP (`Special:` lines), SAB, ASPECT display sub-keys (`<Label>: <text>` lines),
   TEMPDESC (`When active:`). DESCISPI:YES removes the DESC family only.
8. **PI screen last**: `classify_field` over the final `prose` and `label`; a hit refuses the
   record (the marker is never written into a sheet string). Provenance is derived from tokens,
   not from the record's `pi_field` stamp (50 CW records carry a redacted token with
   `pi_field: null`).

**Label**: OUTPUTNAME with `[NAME]` → parenthetical of the name, else the record name, else the
codex-neutral name when NAMEISPI:YES; `(Ex)/(Su)/(Sp)` suffix from TYPE. **Value**: the family
contributes none, except `ASPECT:CheckCount` (uses/period) and `TEMPVALUE` (bounded choice).

## 2. Rows

| token_type | units (ND) | units (CW) | maps_to | confidence |
|---|---:|---:|---|---|
| DESC | 11,317 | 28,065 | Text | high |
| DESC:.CLEAR | — (30 CW instances) | 30 | Text (control) | high |
| BENEFIT | 457 | 638 | Text | high |
| ASPECT: display sub-keys (Ability Benefit, SaveBonus, CombatBonus, SkillBonus, …) | 1,801 any-ASPECT | 4,419 any-ASPECT | Text | medium |
| ASPECT:CheckCount / CheckType | — | ~690 | Number(Expr) | medium |
| ASPECT:NAME | — | 103 | Text (label) | medium |
| ASPECT: structural (ChildAbility, MasterAbility, Archetype Base Class, Bloodline, StatBlockName, SourceBook) | — | ~1,500 | Metadata | high |
| SAB | 0 | 2 | Text | high |
| SPROP | 167 | 3,662 | Text | high |
| TEMPDESC | 31 | 184 | Text | medium |
| TEMPVALUE | 105 | 164 | Number(Expr) via bounded Choice | medium |
| `%CHOICE` / `%LIST` in prose | 54 | ~200 | Text (Choice slot) | medium |
| KEY | 14,295 | 31,497 | Metadata (id) | high |
| OUTPUTNAME | 586 | 2,123 | Text (label) | medium |
| NAMEISPI / DESCISPI | 934 / 529 | 1,177 / 685 | Metadata (provenance.pi) | high |
| VISIBLE | 4,279 | 7,005 | Metadata (print flag) | medium |
| TYPE (display facet) | 16,943 | 41,139 | Metadata (+ `(Su)` suffix) | high |
| SORTKEY | 419 | 3,250 | Metadata | high |
| QUALIFY | 4 | 5 | Applies (waiver on target) | medium |
| SOURCEPAGE / SOURCELONG / SOURCESHORT / SOURCEWEB / SOURCEDATE / SOURCELINK | 11,772 (SOURCEPAGE) | 32,115 | Metadata (provenance) | high |
| (no raw_tokens / empty) | 921 | 2,801 | Text (read the `.lst` row) | high |

**REFUSE rows: none.** Two refusals are named inside rows and are per *token shape*, never per
record: a bare `%` that is neither `NN%` nor `d%` (31 CW rows, `DESC:bare-percent`), and a
blacklist hit on the final `prose`/`label` (count known only at convert time; today's stamps say
2,330 CW records are PI-redacted somewhere, 925 in `description`).

## 3. The three hardest cases, with the records that show them

**Case 1 — `.CLEAR` arrives from the wrong row.** `core_rulebook:class_feature:Sorcerer
Bloodline Feat ~ Scribe Scroll` (`cr_abilities_class.lst:2071`) ships `DESC:You can create magic
scrolls. \t BENEFIT:… \t DESC:.CLEAR \t DESC:Neither the craft feats nor the item creation
section … \t BENEFIT:.CLEAR`. The `.CLEAR` rows are not on line 2071. They are
`CATEGORY=FEAT|Scribe Scroll.MOD … !PRECHARACTERTYPE:1,PC …` in
`core_rulebook/_pfs/pfs_cr_feats.lst:20` — a Pathfinder Society overlay that (a) never applies to a
PC and (b) targets the FEAT, not the Special Ability whose display *name* happens to be "Scribe
Scroll". `wiring_class::build_mod_index` walks `_pfs/` and keys by bare name with `CATEGORY=`
stripped, so both leak (215 CW records carry PFS-overlay tokens; 20 of the tree's 133
`DESC:.CLEAR` fields and 12 of 14 `BENEFIT:.CLEAR` are `_pfs/`). Rule: exclude `_pfs/` by path
and match `.MOD` on (category, name) or KEY. Same shape: `advanced_class_guide:class_feature:
Shaman Hex ~ Fetish` (`acg_abilities_class.lst:1431` + `_pfs/pfs_acg_abilities_class.lst:7`).
The genuine `.CLEAR` is the `.COPY=` shape — `horror_adventures:equipment:Ursine Rageskin`
(`ha_equip_arms_armor.lst:9`, `Hide Armor (Base).COPY=Ursine Rageskin … DESC:.CLEAR \t DESC:This +1
hide armor …`) — which only works with row order copy-base → own row.

**Case 2 — one sentence, two mutually exclusive segments, a number inside.**
`occult_adventures:class_feature:Psychic Bloodline ~ Psychic Strike` (`oa_abilities_class.lst:1881`)
carries `ASPECT:Ability Benefit|%1/day (1d6, DC %2)|…|PREVARLT:…DamageBonus,1` **and**
`ASPECT:Ability Benefit|%1/day (1d6+%2, DC %3)|…|PREVARGTEQ:…DamageBonus,1`, plus
`ASPECT:CheckCount|%1|Sorcerer_Psychic_BloodlinePower1Times`. The sheet must print exactly one of
the two (`6/day (1d6+2, DC 16)` for an 11th-level Cha-16 sorcerer), with the DC folded
(`Sum([Const(10), SpellLevel, AbilityMod(Cha)])`-shaped) and the `1d6+%2` kept as dice. This needs
per-segment Applies (G2), prose slots (G1), and a uses value beside the prose (G5).
`render_pcgen_desc_tokens` gets the gate right (PREVAR\* only) and then **drops** every argument.
The same shape at smaller scale: `bestiary_2:monster_ability:telepathy_miles`
(`ce_abilities_race.lst:1955`, `%1 miles` vs `%1 mile`).

**Case 3 — the words are a choice.** `pathfinder_unchained:class_feature:Unchained Rogue Talent
~ Minor Magic` (`pu_abilities_class.lst:647`): `DESC:You can cast %1. … The caster level for this
ability is %2. The save DC for this spell is %3.|%CHOICE|RogueCasterLevel|10+INT`. `%1` is a NAME
(the chosen 0-level spell), not a number; `Expr::Choice` resolves to an integer in the schema.
Unmade it prints "a chosen 0-level spell"; made it prints "detect magic". `%LIST` is the plural
(`adventurers_guide:class_feature:Pathfinder Savant ~ Esoteric Spell`, `ag_abilities_class.lst:413`:
`… on your spell list: %1.|%LIST`). Today both are dropped unconditionally
(`SD31-W6-INTEGRATE-001`), which is why 21 units sat in SD-34 as "unresolved substitution".

## 4. `render_pcgen_desc` today, and the byte-for-byte parity plan for Epic 6

**What it does** (`src/rules_core/pcgen_desc.rs`; 17 live callers: `class_feature_pool_catalog`,
`race_resolver` (`render_pcgen_desc_tokens`), `pilot_compute/mod.rs` (`PU_RESOLVABLE_DESCRIPTIONS`,
a hand table), `class_feature_grant_consumer` (`_with_values`), `derived_evaluator_fixture_check`,
`spell_resolver` (doc only), the desktop catalogs `equipment/spell/monster/companion_pool/
class_feature_descriptions`, and `gen_feat_gap_tables`/`v06_work_inventory` on the tool side):

1. `strip_trailing_qualifiers` — pop `!?PRE[A-Z0-9]+:` segments from the right (returned as gates).
2. `max_arg_reference` — highest `%N`, counting `%%N`.
3. `split_prose_and_args` — N = 0: strip one trailing *tight* `|tag`; else args = last N
   segments, prose = the rest rejoined with `|`; fewer segments than N: segment 0 is prose.
4. Left-to-right scan of the prose: `%%N` with arg N present → resolve; `%%` → `%` only after a
   digit or `d`/`D` dice, else **drop**; `%N` → `resolve_desc_argument` (int literal, `values`
   lookup, `Name±int`) else **drop and pop a trailing `+`/`-`**; `%UPPERCASE` → **drop**; bare `%`
   → keep after a digit or as `d%`, else **drop**. Every drop is reported in `dropped_args`.
5. If anything dropped: `collapse_whitespace` (all runs → one space, trim). Otherwise
   byte-identical.
6. `decode_pcgen_entities` (five entities).
7. `render_pcgen_desc_tokens`: skip a token whose PREVAR\* gate is *decided* false
   (`eval_desc_gate`; PREABILITY etc. are `Undecided` and keep their prose); join non-empty
   renders with one space. No `.CLEAR` handling anywhere — `DESC:.CLEAR` renders as the literal
   `.CLEAR` (no live record hits this today only because the 30 records are served by other paths).

**Why the converter must not be byte-identical in production:** steps 4's drops are the defect
the operator's rule 1 names. **Parity is proved with a drop-mode flag instead**: the converter
renders every slot as the empty string and pops a trailing sign, then collapses whitespace, and
that string must equal `render_pcgen_desc_tokens(desc_tokens, PcgenDisplayValues::new()).text` for
**every DESC-bearing record CW (28,065)** and, for the fixture roster, equal
`render_pcgen_desc_tokens(desc_tokens, values)` with the roster's `values` (the `race_resolver`
and `pilot_compute` paths). The drop-mode renderer is a test-only function in the converter crate;
the live evaluator never has it. Second parity sample: the 752 zero-token records whose
`data.description` is already rendered must reproduce that field byte-for-byte. Third: the
`class_feature` ingester's `desc_value` (first segment only when a later segment is
PREVAREQ/PREVARGTEQ-gated, else all joined) is a *different* reading from
`render_pcgen_desc_tokens` — the converter follows the token rule above, and the 4,019 CW
multi-DESC records are the diff set to review once (a mechanical list, not per-record judgment).

## 5. Expr / schema gaps (consolidated)

- **G1 prose slots.** `prose: String` cannot hold a character-dependent term. Proposal:
  `prose: Vec<ProseSegment>` with `ProseSegment::{Text(String), Slot(Expr), Choice(ChoiceId),
  Dice{dice, modifier}}`, or `prose: String` with `{0}` anchors + `slots: Vec<Expr>`. Needed by
  DESC (1,284 external + 1,538 formula args CW), BENEFIT, ASPECT (338 + 213), SPROP.
- **G2 per-segment Applies.** One DESC/ASPECT/SPROP segment can carry its own PRE\* gate
  (749 PREVARGTEQ, 438 !PREABILITY, 428 PRECLASS, 354 PREVAREQ, 263 PREVARLT … CW on prose
  tokens). `SheetRule.applies` is whole-rule. Proposal: `ProseSegment { applies: Option<Applies> }`.
- **G3 choice-as-name.** `Choice(ChoiceId)` yields an integer; `%CHOICE`/`%LIST` yield a name.
  Proposal: `Expr::ChoiceName(ChoiceId)` (or `ProseSegment::Choice`) + a per-CHOOSE-type table of
  unmade words, owned by the choice lane.
- **G4 spell summary.** `!PRERULE:1,DisplayFullSpell` segments (1,872 CW) are the one-line
  spell summary. Proposal: `summary: Option<String>` on SheetRule, or discard (open question).
- **G5 more than one value.** `ASPECT:CheckCount` (uses/period) sits beside a magnitude on the
  same record. Proposal: `uses: Option<(Expr, Period)>` or `values: Vec<(label, SheetValue)>`.
- **G6 print flag.** `VISIBLE:NO` rules are held, not printed. Proposal: `print: bool` on
  SheetRule (or in provenance).
- **G7 label slot.** `ASPECT:NAME|Str +%1|<var>` (103 CW) makes the label character-dependent.
  Proposal: `label: Vec<ProseSegment>` or fall back to the record name with the template in prose.
- **G8 bounded choice.** `TEMPVALUE:MIN=..|MAX=..` needs a Choice with an integer range.
- **G9 waiver in Applies.** `QUALIFY` adds "or holds rule X" to another rule's Applies.
- **Entity table.** `&colon;` is a sixth entity not in `PCGEN_ENTITIES`.

## 6. `provenance` proposal

```rust
pub struct Provenance {
    pub book: String,                 // corpus book dir, inventory spelling ("bestiary")
    pub kind: String,
    pub rows: Vec<SourceRow>,         // every closure row that contributed, in application order
    pub oracle_pin: String,           // PCGEN_ORACLE_SHA from scripts/pcgen-oracle-pin.env
    pub page: Option<String>,         // SOURCEPAGE ("p.242")
    pub source_book: SourceBook,      // SOURCELONG/SHORT/WEB/DATE row overrides, else the book header
    pub link: Option<String>,         // SOURCELINK
    pub license: License,             // OGL | PiRedacted (from tokens, not the record stamp)
    pub pi: PiFlags,                  // { name_declared, description_declared, redacted: Vec<Field> }
    pub label_source: LabelSource,    // Name | OutputName | CodexNeutral
    pub type_tags: Vec<String>,       // TYPE segments (never a redacted one)
    pub sort_key: Option<String>,
    pub converter: String,            // "sheet_rule_convert <crate version>+<git sha>"
    pub converted_at: String,         // ISO-8601
    pub refused: Vec<String>,         // token_type names; empty on a complete rule
}
pub struct SourceRow { pub file: String, pub line: usize, pub sha256: String, pub role: RowRole } // Base | CopyBase | Mod
```

Rules: no string that came from a PI-declared or redacted field enters provenance (no term hit
text, no redacted KEY, no redacted TYPE segment); `rows` cites file:line so the fine print and the
oracle harness can reach the source; `license`/`pi` are re-derived every conversion from the
tokens, which also retires the `reconcile_description_pi_stamps` post-pass for `data/sheet_rules/`.

## 7. Handoffs to other lanes

- **formula**: every DESC/BENEFIT/ASPECT/SPROP argument that is not a same-row literal (1,284 +
  1,538 DESC, 338 + 213 ASPECT, 66 BENEFIT CW); `ASPECT:CheckCount` variables; `TEMPVALUE` MAX
  variables. SAB has no embedded formulas (0 of 2).
- **prereq**: per-segment gates on prose tokens; `QUALIFY` waivers; `!PRECHARACTERTYPE:1,PC` on
  `_pfs/` rows (or drop by path).
- **choice**: `%CHOICE`/`%LIST` slots and the unmade-words table; `TEMPVALUE` bounded choice.
- **bonus**: nothing; this family contributes no magnitude.
