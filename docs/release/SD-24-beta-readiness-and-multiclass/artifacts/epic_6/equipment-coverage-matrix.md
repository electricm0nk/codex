# Equipment / Armor / Spell Coverage Matrix — Epic 6 Criterion 6.1

> Read-only audit. No production remediation in this cycle — this artifact is the
> gap list criteria 6.2–6.5 consume. Source of truth for every number below:
> `tests/sd24_equipment_coverage_audit.rs` (standing regression) plus direct
> inspection of `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/`
> (reachable from this environment).

## 1. Plan-vs-reality correction

The task brief named `src/rules_core/rules_tables/equipment/*.rs`, `armor/*.rs`,
`spells/*.rs` (per `loop-instruction.md §2.4`'s file-touch partition table) as
Epic 6's owned files. No such flat `equipment/`, `armor/`, or `spells/`
directories exist. The real shape (already corrected by SD-22's own per-book
ingest, and by this bundle's own cycle-launch note) is per-corpus:

- `src/rules_core/rules_tables/{crb,apg,acg}/equipment_tables.rs` (+ CRB's own
  `equipment_data/{arms_armor,general,magic_items,equipmods}.rs` subdirectory)
- `src/rules_core/rules_tables/{crb,apg,acg}/spell_list.rs`
- **`src/rules_core/rules_tables/beastiary1/` has no equipment or spell module
  at all** — the book-level module carries only monster stat blocks
  (`MonsterStatBlock`, `NaturalAttack`) per its own `mod.rs` doc comment. There
  is no file to audit for fields; the gap is total (see §4).

This audit is scored against the real modules above.

## 2. Record-coverage matrix (rows present vs. real corpus)

| Book | Category | Real corpus count (active, non-`.MOD`, deduped) | Repo record count | Coverage |
|---|---|---:|---:|---:|
| CRB | Arms/Armor | 310 | 310 | 100% |
| CRB | General | 453 | 453 | 100% |
| CRB | Magic Items | 1556 | 1556 | 100% |
| CRB | Equipmods | 658 | 658 | 100% |
| **CRB equipment total** | | **2977** | **2977** | **100%** |
| APG | General + Arms/Armor + Magic Items | 94 + 76 + 171 = 341 | 3 | 0.9% |
| ACG | General + Arms/Armor + Magic Items (`acg_equip.lst`, `TYPE:`-disambiguated) | 221 | 3 | 1.4% |
| Bestiary 1 | General + Arms/Armor + Magic Items (`b1_equip_*.lst`) | 2 + 3 + 2 = 7 | 0 (no module) | 0% |
| CRB | Spells (`cr_spells.lst`) | 675 | 652 | 96.6% |
| APG | Spells (`apg_spells.lst`) | 298 | 4 | 1.3% |
| ACG | Spells (`acg_spells.lst`) | 145 | 4 | 2.8% |

Real corpus counts derived by: `grep -vE '^\s*#|^\s*$' <file> | grep -vE '\.MOD' |
awk -F'\t' '{print $1}' | sort -u | wc -l` against the live PCGen LST files
(dedups any duplicate-name lines the same way the real game engine treats them
as one record). CRB equipment's exact figures were already independently
documented per-category in `equipment_data/{arms_armor,general,magic_items,equipmods}.rs`'s
own doc comments (SD-17's `KEY:`-merge-dedup fix) and matched this cycle's
independent recount exactly.

**Finding: CRB equipment is fully record-ingested.** The `equipment_tables.rs`
module doc comment's framing ("Bootstrap coverage: one representative item per
category... Exhaustive per-category coverage is the loop's job") is stale
prose left over from the original SD-22 bootstrap; the real data has been
100% since a later SD-17 pass. This is a documentation gap, not a data gap —
see `## DISCOVERED`.

**Finding: CRB spells are 96.6% record-complete** (652/675) — a real, modest
gap of 23 spells not yet ingested.

**Finding: APG and ACG equipment + spells are still at their original SD-22
bootstrap samples** (3 equipment rows, 4 spell rows per book) — large,
genuine record-coverage gaps.

**Finding: Bestiary 1 has zero equipment ingestion** — no module exists. The
real corpus is small (7 total records across three `b1_equip_*.lst` files),
so this is a bounded, cheap remediation target for a follow-on 6.2-family
cycle, but it does not exist today.

## 3. Field-coverage matrix (cost / weight / description / full text)

| Book | Has cost | Has weight | Has full description (equipment) | Audit status |
|---|---|---|---|---|
| CRB equipment | Real per-row (`cost_gp: Option<f64>`, populated per corpus `COST:` token; `None` is a genuine "no independent price" case for some sub-component rows, not a gap) | **0/2977 — no `weight` field exists on `EquipmentTableEntry`** | **0/2977 — no `description` field exists on `EquipmentTableEntry`** | Schema-level gap, not per-row |
| APG equipment | Real per-row (3/3 populated) | **0/3 — same schema absence** | **0/3 — same schema absence** | Schema-level gap |
| ACG equipment | Real per-row (3/3 populated) | **0/3 — same schema absence** | **0/3 — same schema absence** | Schema-level gap |
| Bestiary 1 equipment | n/a — no module | n/a | n/a | No module to audit |

**`EquipmentTableEntry` (CRB, APG, ACG — all three books share the same field
set) has exactly four fields: `key`, `category`, `name`, `cost_gp`.** There is
no `weight` field and no `description` field in the type at all, in any book.
This is the headline finding for criterion 6.1: criteria 6.3 (weight) and 6.4
(description) are not "fill in the blanks in existing rows" work — they
require a schema change (add the fields to `EquipmentTableEntry`) before any
per-row population can start, in every one of the three books that have an
equipment table at all.

| Book | Has description (spell) | Full SRD/PRD text present |
|---|---|---|
| CRB spells | 652/652 (100% — `description: &'static str` is non-optional) | **0/652 — every entry is the corpus's short `DESC:` summary line only** |
| APG spells | 4/4 (100%) | **0/4 — same** |
| ACG spells | 4/4 (100%) | **0/4 — same** |

**Finding: no spell in any book carries full SRD/PRD text.** Unlike equipment,
`SpellListEntry.description` *does* exist as a field and *is* populated for
every present record — but only with the corpus's short one-sentence `DESC:`
summary (e.g. Alarm: *"Alarm creates a subtle ward on an area you select."*,
25 chars), never the full spell text. The real corpus carries the full text on
a **separate `<Name>.MOD` record** with its own, much longer `DESC:` token
(Alarm's `.MOD` record's description is ~900 characters, covering the mental
vs. audible alarm mechanics, range-reduction-through-walls rule, and the
`permanency` interaction — none of which the base record's short summary
mentions). Criterion 6.5's "full spell text" therefore requires ingesting a
second corpus record type (`.MOD` full-text records) this repo's spell
ingester has never read, not just filling in existing rows.

## 4. Per-corpus gap list (steers criteria 6.2–6.5)

1. **[6.3 — weight] Add a `weight` field to `EquipmentTableEntry`** (shared
   shape across CRB/APG/ACG) and populate it from the corpus `WT:` token.
   Schema change first, then per-row population, in all three books.
2. **[6.4 — description] Add a `description` field to `EquipmentTableEntry`**
   and populate it from the corpus. Equipment records in `cr_equip_*.lst` /
   `apg_equip_*.lst` / `acg_equip.lst` generally do **not** carry a `DESC:`
   token the way spells do (PCGen equipment descriptions are usually prose in
   the printed book, not an LST token) — this needs its own sourcing
   methodology decision before criterion 6.4 can proceed (see `## DISCOVERED`).
3. **[6.2 — cost / record coverage] APG equipment: ingest the remaining
   338 of 341 real records** (94 general + 76 arms/armor + 171 magic items,
   minus the 3 already-bootstrapped).
4. **[6.2 — cost / record coverage] ACG equipment: ingest the remaining
   218 of 221 real records** (`acg_equip.lst`, `TYPE:`-disambiguated).
5. **[6.2 — new scope] Bestiary 1 equipment: ingest all 7 real records**
   (`b1_equip_general.lst` ×2, `b1_equip_arms_armor.lst` ×3,
   `b1_equip_magic_items.lst` ×2) — no module exists yet; this is new work,
   not remediation of an existing gap.
6. **[6.2 — record coverage] CRB spells: ingest the remaining 23 of 675 real
   records** in `cr_spells.lst` not currently in `SPELL_LIST`.
7. **[6.2/6.5 — record + full-text coverage] APG spells: ingest the remaining
   294 of 298 real records**, all with full `.MOD`-sourced text once criterion
   6.5's methodology lands.
8. **[6.2/6.5 — record + full-text coverage] ACG spells: ingest the remaining
   141 of 145 real records**, same full-text sourcing dependency.
9. **[6.5 — full text, all books] Every present spell record (652 CRB + 4
   APG + 4 ACG = 660 records) needs its `description` replaced with the
   corpus's `<Name>.MOD` full-text `DESC:` token**, not just newly-ingested
   records — this is a re-ingest of already-landed rows, not purely
   additive work.

## 5. Cross-references

- `tests/sd24_equipment_coverage_audit.rs` — standing regression test, the
  executable form of this matrix (RED → GREEN evidence in this cycle's
  receipt).
- `src/rules_core/rules_tables/{crb,apg,acg}/equipment_tables.rs` —
  `EquipmentFieldCoverage` / `field_coverage_report()`.
- `src/rules_core/rules_tables/{crb,apg,acg}/spell_list.rs` —
  `SpellFieldCoverage` / `spell_coverage_report()`.
- `./artifacts/epic_6/equipment-coverage-audit_cycle_receipt.md` — this
  cycle's full receipt.
