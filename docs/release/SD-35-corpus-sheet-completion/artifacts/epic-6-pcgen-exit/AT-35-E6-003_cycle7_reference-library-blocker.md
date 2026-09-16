# AT-35-E6-003 cycle 7 — what blocks `reference_library_catalog.rs`, measured

Cycle 6's next-cycle scope listed `reference_library_catalog.rs` (15 residue hits) among "the five
structural readers, which need `SheetRule.applies`/`grants` rather than prose". This cycle built
the join, pointed it at the whole population, and measured the result before writing any swap.

**It does not clear the bar, and the reason is a number, not a judgement.** Swapping it to the
converted package today would take **1,150 of the 9,679** descriptions it serves off a player's
screen. The measurement is below so the next cycle scopes the mechanism rather than rediscovering
it, and so nobody ships the swap on the assumption that it is a fair trade.

## 1. The population

`reference_library_catalog.rs` serves every record under `data/corpus/<book>/<kind_dir>/` for the
twelve generic content-kind directories its own `REFERENCE_LIBRARY_KIND_DIRS` names: `ability`,
`class_generic`, `deity`, `domain`, `feat_generic`, `language`, `monster_generic`, `power`,
`race_generic`, `skill`, `template`, `trait_generic`.

**9,697 records.** Of those, **9,679** carry something today and **18** are the genuinely-empty
residual the module's own doc comment names.

It resolves content in three tiers: the record's `description` field, a `DESC` row inside
`raw_tokens`, and — for the many records the source ships as a bare mechanical row with no prose
at all — a rendered summary of the record's own non-administrative `raw_tokens`. Tier 3 is where
**4,523** of the 9,679 land, and tier 3 is an ingest-format reader by construction.

## 2. What the converted package holds for the same 9,697

Resolved through the four-step join `converted_prose.rs` already implements (book-exact, then the
source row both sides record, then package-wide by name, then one trailing printed qualifier
dropped), with the corpus directory's `_generic` suffix stripped to reach the converter's own kind
name:

| the converted rule states | records |
|---|---:|
| descriptive prose (`Desc`/`Benefit`/`Special`) | 4,566 |
| a stat block only (`Aspect`/`StatBlock`/`WhenActive`) | 480 |
| no prose, but a value / target / grant / choice / gate / tag | 3,488 |
| **nothing at all, or no rule under any join step** | **1,163** |

So a three-tier swap — description, then stat block, then the record's own stated facts in English
— would serve **8,534** of 9,697 and gain 5 records that carry nothing today.

## 3. The 1,150, by mechanism

Of the 1,163 that resolve to nothing, 1,150 carry something today and would lose it:

| mechanism | records |
|---|---:|
| **no converted rule under any join step** — every one of them under `data/corpus/<book>/ability/` | 489 |
| resolves **book-exact** to a rule that states nothing: `Text` value, no target, no grant, no choice, an `Always` gate, no tags | 563 |
| resolves by **source row** to the same | 81 |
| resolves by **name** to the same | 17 |

The 489 are the same shape cycle 6 recorded for `core_essentials` and for
`advanced_players_guide:spell:wall_of_thorms`: real corpus records that are **not units of
`docs/work-inventory.json`**, and `sheet_rule::load_population` walks the inventory's units, so the
converter's own population never sees them. Admitting them moves the bundle-wide denominator 49,438
and is inventory scope, not this criterion's — **reported, not excused**.

## 4. What the old text for those 1,150 actually is

Cycle 5's standing correction — *a raw loss count is not a loss count until every lost row's old
text is read* — applies, and it does **not** discharge this one. Their tier-3 summaries are built
from these token keys (count of rows carrying each):

```
CATEGORY 597 · PREMULT 509 · DESC 492 · TYPE 489 · VISIBLE 393 · BONUS 350 · SPELLS 233
HD 204 · ABILITY 160 · ASPECT 150 · CSKILL 115 · RACESUBTYPE 101 · SUBRACE 96 · DEFINE 73
```

Much of that is administrative (`CATEGORY`, `VISIBLE`, `TYPE`) and losing it is a correction rather
than a regression. `PREMULT` (509), `BONUS` (350) and `SPELLS` (233) are not: they are the record's
real prerequisites, bonuses and spell grants, and a swap that drops them drops rules content.

Counted separately: of the records that join **book-exact** and carry a `DESC` token whose words the
converted rule does not hold, there are **37**, and every one of their `DESC` values is the literal
`[redacted PI]` — correctly refused at ingest, not a converter gap.

## 5. The gate agrees

`reach_gate.rs::BARE_RECORD_FINDINGS` pins every record that reaches a surface carrying only its
key, **by exact key, in both directions**. Shipping this swap would require adding roughly 1,145
keys to that list. That is the gate doing exactly what it was built to do, and it is the reason
this artifact exists instead of a swap.

## 6. What the next cycle needs, named as mechanisms

1. **The 489 `ability` records that are in no inventory unit.** Inventory scope. Until they are
   units, the converter cannot hold them and no live reader can serve them from our schema.
2. **A renderer for a converted rule's stated facts.** Built and measured this cycle (`value` +
   `target` + `bonus_type` → "+2 racial bonus to Strength"; `grants` → "Grants proficiency with
   …"; `offers` → "Choose 1 from …"; `applies` → "Available when: …"; `tags`), and **not shipped**,
   because with nothing consuming it it would be dead code. It closes 3,488 of the 4,523 tier-3
   rows. Land it in the cycle that swaps the file, not before.
3. **The 563 book-exact rules that state nothing.** Diagnose one group before assuming a cause: a
   record whose whole content is `PREMULT`/`BONUS`/`SPELLS` should be carrying an `applies`, a
   `target` and a `grants`, and the fact that it carries none is a converter finding, not a
   property of the record.

## 7. Figures + their re-derive commands

Every figure in §2 and §3 comes from one committed, re-runnable script — it reproduces
`converted_prose.rs`'s four-step join and `reference_library_catalog.rs`'s three content tiers
rather than approximating them:

```
python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003_cycle7_reference_library_census.py
records=9697 served_today=9679
converted: description=4566 stat_block=480 facts=3488 none=1163 served_after=8534
lost=1150 by_join_step={'book_exact': 563, 'no_rule': 489, 'by_name': 17, 'source_row': 81}
lost by corpus kind dir={'ability': 672, 'domain': 10, 'template': 450, 'feat_generic': 1, 'race_generic': 7, 'deity': 9, 'skill': 1}
```

| figure | denominator | command |
|---|---|---|
| 9,697 records; 9,679 served today; 4,566 / 480 / 3,488 / 1,163; 1,150 lost split 489 / 563 / 81 / 17 | every record under the twelve kind dirs | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003_cycle7_reference_library_census.py` |
| 4,523 records at tier 3 today | the same 9,697 | the same script's `served_today` tiering — tier 3 is the branch after the `description` field and the `DESC` raw token both miss (`served_today()` in that file) |
| the token-key histogram in §4, and the 37 `[redacted PI]` rules in §4 | the 1,150, and rules under `data/sheet_rules/<book>/<kind>/` | derived in this cycle's working tree with the same walk; the script above prints the partition those rows come from (`by_join_step`) |
| `apps/desktop` residue after this cycle: 6 files / 179 hits | the live roots' source files | `python3 scripts/pcgen_residue_gate.py --check` |
