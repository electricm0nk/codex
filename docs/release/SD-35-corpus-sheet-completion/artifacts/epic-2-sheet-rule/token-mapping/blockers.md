---
canonical: true
owner: synthesis lane (SD-35 Epic 2 token-mapping)
bundle_id: SD-35
date: 2026-09-07
head: tranche/15
---

# Epic 2 — the real remaining blockers after the token-mapping synthesis

Everything a lane or judge called a blocker was re-tested against the corpus and the pinned
PCGen source this session (`SYNTHESIS.md` header lists what was opened). Items the synthesis
CLOSED by design and that are no longer blockers: cross-record variables (schema v2 `Var` +
contribution table), division semantics (exact `Div`, one truncation), prose slots, the
three-valued `Applies`, the `_pfs/` leak (a fix, not a ruling), class LEVEL grants for Paizo
base classes (refuted: already in the closure), `If`/`Cmp`, `ChoiceCount`, `Toggle`,
`FeatureNotReplaced`, `PcLevelBonus`, QUALIFY's G9, NATURALATTACKS (mapped), the spell
stat-block tokens (mapped), and every unit-count disagreement (re-derived, `§C12`).

Denominator for every unit figure: 23,315 non-DONE of 49,438 units unless stated (CW =
corpus-wide records of 51,474). Owners: AT-35-E2-001 converter, AT-35-E2-005 first pass +
oracle, AT-35-E4-001 mapping rows, AT-35-E4-002 oracle run, AT-35-E1-002 gate plumbing.

## Blockers that are work, not rulings

| id | What | Units (denominator) | Owner | What closes it |
|---|---|---|---|---|
| B1 | **Oracle export set lacks sheet totals.** `computed-values.txt.ftl` exports STAT score/mod, HP, AC total/touch/flat, BAB, CMB, CMD, CHECK total/base; `charbuild-remainder.txt.ftl` exports STAT scores and `SA.n.DESC`; `weapon-family` exports `WEAPON.COUNT` only. Nothing exports skills, initiative, speed, vision, DR, spell DCs, caster levels, spells per day/known, or per-weapon attack lines. | BONUS:SKILL 706 + BONUS:VISION 19 + MOVEADD 87 + DR 24 + DC 27 + SPELLCAST 93 + SPELLKNOWN 338 + CASTERLEVEL 113 = 1,407 rows' worth of units with no PCGen total (the brief's "763+"); RANGE 665, SR 43, DR 108, MOVE 165, REACH 60, VISION 102, SPELLS 671 are sheet-line only | AT-35-E4-002 (harness tokens) before AT-35-E2-005 claims parity | add `SKILL.<n>.TOTAL`, `INITIATIVEMOD`, `MOVEMENT.<n>`, `VISION`, `DR`, `SPELLLISTCAST`/`SPELLLISTKNOWN`, `SPELLLISTDC`, `WEAPON.<n>.TOTALHIT/DAMAGE` to the three `.ftl` templates; until then those rows verify by sheet line and by `SA.n.DESC` where a DESC slot carries the number |
| B2 | **Un-ingested rows that carry DEFINEs / BONUS:VARs.** Names DEFINEd only on pinned rows the corpus never ingested (`up_classes.lst` VitalistLVL and 8 more psionic classes, `ag_classes.lst` prestige levels, `ce_abilities.lst` Caster_Level_Bonus, `sos_abilities.lst`, companion mods); plus contributor base rows in ingested files (`acg_abilities_globalvar.lst` 703, `isg_deities.lst` 594, `cr_abilities_class.lst` 593, `apg_abilities_globalvar.lst` 492 of 1,799 base rows). | 434 non-DONE units hang on a never-ingested DEFINE (130 names / 778 instances, bonus family); formula row 29's upper bound ≤2,657 non-DONE units depend on an un-ingested contributor (unverified, needs the closure) | AT-35-E2-001 (converter reads the closure from the pinned tree, so the CONTRIBUTIONS convert now) + AT-35-E4-001 (the never-ingested records themselves are an ingest remainder per FILE) | the converter's variable index reads the pinned tree (`§C1`, `§C2`); the record-level remainder is the four `_globalvar`/class files first (2,382 of 7,768 rows) |
| B3 | **ABILITYCATEGORY pool definitions are an input the converter does not have.** 208 `*_abilitycategories.lst` files / 2,831 rows in the pinned checkout; 0 in `data/corpus`; `src/pcgen_import` reads none (`corpus_traps.rs:196`). Needed for `BONUS:ABILITYPOOL` pools (`Choice.count` + `OptionSet::Rules{pool, tags, requires}`); `CHOOSE:ABILITYSELECTION|<parent>|TYPE=` is self-contained for 122 of 144 records. | 1,093 non-DONE units (1,013 pool names) | AT-35-E2-001 | a ~30-line tool-side reader for CATEGORY, TYPE, PRE, PLURAL, POOL |
| B4 | **Character facts the live record lacks**, each with the gates waiting on it. Under two-valued `Applies` a gate over an absent fact is Exclude (never silently held), so these are counts, not Words. | alignment: PREALIGN 117 + !PREALIGN 24 + DOMAINS-with-PREALIGN 421 (trailing); deity: PREDEITY 50 (41 PI-redacted) + PREDEITYDOMAIN 2 + PREDEITYALIGN 8; gender 2; age category 1; base size 41 (`Expr::BaseSize`); spells known / highest spell level 12 + 10 + 1; companion master level 209 gates (`Expr::MasterLevel`) | AT-35-E2-002 (the evaluator's character facts) | add each fact to `CharacterPrereqFacts` / the chassis output; alignment and deity first (they gate DOMAINS for every cleric) |
| B5 | **Prestige / hybrid / third-party class level lines are parsed and never persisted.** `ClassFeatureBlock.level_lines` (`src/pcgen_import/lst_parser/class.rs`): 2,016 `<lvl>\tABILITY:` lines in 72 `*classes*.lst` files (630 Paizo lines at level ≠ 1: adventurers_guide 181, advanced_class_guide 154, advanced_players_guide 114, ultimate_magic 53, core_rulebook 49) plus 682 BONUS, 381 ADD, 168 DOMAIN, 41 TEMPLATE numbered lines. NOT a blocker for Paizo base classes (their grants are `CATEGORY=Class|X.MOD` rows already in the closure, `§C10`). | the class_feature records with no grant edge after both sources are read — derive at convert time; the population ceiling is 12,109 non-DONE class_feature records with `data.class`, of which 1,918 already have a holder via `data/class_feature_grants/` | AT-35-E2-001 | read `level_lines` tool-side at convert time for every level-line head (ABILITY, BONUS, ADD, DOMAIN, TEMPLATE); `data/class_feature_grants/` is the cross-check; no ruling, no corpus regeneration |
| B6 | **PI residue, counted once.** `[redacted PI]` values: BONUS 99 units, DEFINE 40, SPELLS 66, PREMULT bodies 327 instances, PREDEITY 41, ABILITY 295, KIT 12, TEMPLATE names 67, CHOOSE 11, AUTO 4, PRECAMPAIGN 5; DESC/BENEFIT/ASPECT markers 1,232 + 24 + 118 instances; term-hit renames 456 CW; 2,330 CW records PI-redacted somewhere. The design rule (`§C20`): omit the field, stamp `provenance.pi`, codex-neutral label, `Situational{"requirement withheld"}` for a redacted gate, never refuse. The value-bearing subset (BONUS/DEFINE/SPELLS = 205 units) stays REFUSE by shape: there is nothing to compute. | 205 REFUSE units + ~900 CW term-hit records under the omit-and-stamp rule | AT-35-E2-001 (omit + stamp), AT-35-E1-002 (re-key `declared_pi_shipping_audit.rs` to `provenance.pi` + `prose`) | the omit-and-stamp rule; Decision 28 is the standing PI ruling — the label-only question below is the only part for the operator |
| B7 | **The 1,008 units with no `raw_tokens`** (888 without the field + 33 empty, 37-book join; 921 by the lane's 35-book join): 708 source rows carry a prose token, 204 none, 751 have a shipped `description`; 351 are feat records with a `prerequisites` list. The converter reads the `.lst` row, not `raw_tokens`, so these convert normally. 11 CW records (2 non-DONE: `advanced_players_guide/equipment/hammer_ricochet.json`, `spell/fester.json`) carry NO source block. | 1,008 (of which 2 refuse as `no_source_row`) + 3 unjoined spells (`no_corpus_record`: `botd2_spells_ndl.lst:6`, `uc_spells.lst:121`, `oa_spells.lst:464`) | AT-35-E2-001 (row read), AT-35-E4-001 (the 5 record defects) | the row-read path; repair the 5 records' source/join |
| B8 | **Small shape refusals with no v2 variant.** HITDIE `%+1` / `%/N` die-step (8 units; `HitDieStep` semantics not read this session), `SIZE:<formula>` (1), equipment/encumbrance state in a formula (`ENCUMBERANCE`, `COUNT[EQTYPE…]`, `ACCHECK`, `MOVEBASE`; ~60 units, formula row 21), `DAMAGESIZE` with no base die reachable through the grant closure (count derived at convert time, ≤ 585 instances), undefined names inside a function or under `* /` (123 uses), `PRESUBCLASS` until a SUBCLASS-line reader exists (17), `%LIST` as an SPROP variable (48 CW, unverified). | ≤ 260 units | AT-35-E4-001 | each is one reader or one Java citation; none is per-record |
| B9 | **Converter closure hygiene** — the `_pfs/` skip and KEY-based `.MOD` matching (`§C4`) must land in `wiring_class::build_mod_index` before the first conversion, or 826 CW records convert with overlay tokens and 4,508 display-name attachments. | 826 CW records (192 visible in `raw_tokens`); 874 mod_only units (867 non-DONE) depend on the closure being right | AT-35-E2-001 | the two index changes; the cross-book part is ruling 3 |
| B10 | **Gate plumbing.** `token_coverage.py` must count refusals per token SHAPE (the table's `maps_to == REFUSE` rows plus the shapes in B8), and `sheet_rule_convert --check` must fail on a `SheetRule` that carries a `%`-marker, a `TYPE=`, a `DEFINE`d name, or any `_vars/` id with no table. | gate, not units | AT-35-E1-002 / AT-35-E1-005 | the residue gate's literal list already covers `%CHOICE`, `%LIST`, `TYPE=`; add `%[0-9]` and the var-id-without-table check |

## Operator rulings (at most three; each with options and a recommendation)

**RULED 2026-09-08 — the operator accepted all three recommendations (`../../../decisions.md §15`):
R1 (a) hide, R2 (a) omit-and-stamp, R3 (a) corpus-wide. The options below are kept for the record.**

**Ruling 1 — VISIBLE:DISPLAY rows: PCGen's sheet or "print the rule"?** 2,319 instances (of
7,505 VISIBLE) are `DISPLAY`, which `Visibility.java:26` defines as "shows up in the GUI, but
not on the output sheet"; they are selector and helper records (Wizard ~ Scribe Scroll,
`cr_abilities_class.lst:2572`, whose feat row already prints). Units are `sheet-complete`
under every option (converted, held, renderable).
- (a) Follow PCGen: `print = false` for DISPLAY and NO; the feature prints once, where the
  book puts it. **Recommended.** A DISPLAY row is bookkeeping the player never writes on paper.
- (b) Print everything except NO: the operator's literal "print the rule"; Scribe Scroll
  appears twice on a wizard's sheet, and 2,319 helper rows print.
- (c) Print DISPLAY rows in a collapsed "also held" list under the section: honours both
  readings at the cost of one more sheet affordance.

**Ruling 2 — PI term-hit bucket (~900 CW records: 431 DESC term hits, 24 BENEFIT, 456
term-hit renames; plus 205 units whose VALUE is `[redacted PI]`): refuse forever, or print
label-only?** Decision 28 already forbids the PI text; the question is the record's terminal
state.
- (a) Omit the redacted field, stamp `provenance.pi`, print the codex-neutral label and
  whatever is licensed (a number, other segments); the record is `sheet-complete`. The 205
  value-redacted units stay REFUSE by shape (nothing to compute). **Recommended** — one outcome
  for one fact, same as a declared NAMEISPI/DESCISPI; the audit re-keys to `provenance.pi`.
- (b) Refuse every term-hit record forever: a permanent ~900-record bucket no mechanism can
  close; contradicts the no-carve-outs rule.
- (c) Print label-only for every PI-touched record (drop even the licensed number): simplest
  audit, loses licensed magnitudes on ~450 records.

**Ruling 3 — Converter closure scope: corpus-wide or per book?** 4,419 `.MOD` targets / 6,686
rows CW name a base record that exists only in another book (`oa_feats.lst:77`
`Improved Feint.MOD` onto the core_rulebook feat); PCGen applies them when both campaigns
load; today's per-book index attaches none. Also 56 CW records have their BASE row inside a
`_pfs/` file (inner_sea_world_guide 46, core_rulebook 3, advanced_race_guide 3, …).
- (a) Corpus-wide closure: a `.MOD` in a later book changes the earlier book's `SheetRule`;
  provenance cites the foreign row; the product ships one merged rule set. **Recommended** —
  it is what the corpus loader already does (every book loads), and it is PCGen's behaviour
  with all campaigns on; the 56 `_pfs/`-based records convert from their own row (path
  exclusion applies to the MOD INDEX only) and are tagged `provenance.overlay = pfs`.
- (b) Per-book closure: the 6,686 rows never attach; the CRB Improved Feint stays the CRB's;
  matches a "one book at a time" reading nothing in the product implements.
- (c) Corpus-wide with a per-rule `book_overrides` list so a future book toggle can drop them:
  correct and future-proof, one more field and one more evaluator branch now.
