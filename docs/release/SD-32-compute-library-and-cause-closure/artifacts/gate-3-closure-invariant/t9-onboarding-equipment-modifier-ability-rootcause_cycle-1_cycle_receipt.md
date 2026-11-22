# Cycle t9-onboarding-equipment-modifier-ability-rootcause — Gate 3 Closure Invariant / `no_record` root-cause tracing

- **Card ID:** epic-2-cause-closure (card 11), `no_record == 0` closure line (`decisions.md §20`/`§27b`)
- **Commit SHA:** (this receipt only — no production code changed this cycle; see "Why no fix landed" below)
- **Files touched:** this receipt; `docs/retro/events/t9-onboarding.jsonl` (one correction)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (no code diff to audit)
- **Wired-integration audit result:** OK_NO_TOKENS (no code diff to audit)
- **Acceptance criterion:** `decisions.md §20`/`§27b` — `no_record` reaches zero; this cycle's scope
  was `equipment_modifier` (19), `equipment` (10), `ability` (1), assigned as **root-cause tracing,
  not yet a fix** (per dispatch brief: "root cause is NOT yet isolated... your first deliverable is
  the actual root cause, per group, not a fix").
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (oracle pin, `scripts/pcgen-oracle-pin.env`)
- **Status:** complete (as a root-cause deliverable) / returned-to-backlog (as a `no_record` closure —
  the population is still 30, unchanged this cycle)
- **Notes:** see full findings below.
- **Discovery forwards:** none opened (existing card 11 already tracks this scope)
- **Next-cycle plan:** implement the four fixes named below, in the order given (cheapest/most
  independent first); re-run `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json`
  after each and confirm the affected `no_record` rows clear.

## Re-derivation of the target population (`§17a`)

```
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger.json
```
`no_record` = **130** total (matches the dispatch brief). Restricted to my three kinds:

| kind | count | books |
|---|---:|---|
| `equipment_modifier` | 19 | advanced_class_guide 14, pathfinder_unchained 4, adventurers_guide 1 |
| `equipment` | 10 | ultimate_magic 6, advanced_class_guide 2, bestiary_3 1, ultimate_equipment 1 |
| `ability` | 1 | ultimate_campaign 1 |

Confirmed against `/tmp/ledger.json`'s `rows` where `join_status == "no_record"`. This matches the
brief's stated 19/10/1 split exactly — no correction needed to the population count itself.

## Root cause, per group (evidence, not a fix, per this cycle's assignment)

### Group A — ACG `equipment_modifier` (14 of 19): duplicate walker unit vs. `.COPY=`-alias-cited corpus record

`docs/work-inventory.json` mints **two separate units** for one underlying PCGen equipmod object:
- a long-form unit (`special_ability_<name>_<type>`) citing the object's **primary declaration
  line** (e.g. `acg_equipmods.lst:13` for "Burdenless")
- a short-form unit (bare display name, e.g. `burdenless`) citing a **`.COPY=<name>` "Old KEYs"
  alias row** later in the same file (`acg_equipmods.lst:86`)

`gen_equipment_gap_tables.rs`'s local `find_citation`/`try_files` (mirroring
`cache_gen::equipment_gap::try_files`, see its doc comment at `equipment_gap.rs:356-386`) is called
with `entry.key = entry.name = "Burdenless"` (the SHORT form, from `equipment_gap_tables.rs`'s
static table, e.g. line 451). Its search order is `KEY:` field, then `.COPY=<id>`, then
first-column exact match — so `.COPY=Burdenless` (line 86) wins BEFORE the first-column match at
line 13 is ever tried. The corpus record (`data/corpus/advanced_class_guide/equipment/equipmods/
burdenless.json`) therefore carries `source.line: 86`, matching the SHORT-form walker unit but not
the LONG-form one.

Verified for all 10 ACG `special_ability_*_armor`/`*_weapon` `ingested-magnitude` units (burdenless,
exclusionary x2, phantom_ammunition, prehensile, restful, sneaky x2, spiteful, trackless) and the 4
`unknown`-status Blood-Hunting/Spirit-Hunting pairs — same shape, same file, corpus records exist
at the `.COPY=` line in every case (`python3 /tmp/check_sourcelines.py`, all 48 files in
`equipment/equipmods/` cite lines 85-132, i.e. the "Old KEYs" block, never the 10-84 primary block).

**This is a genuine duplicate-unit defect in the walker enumeration**, not a missing-ingest defect —
the content is already correctly ingested (readable at the `.COPY=`-cited coordinate); the
long-form `special_ability_*` unit is simply an orphan id with no matching citation. Fix belongs in
`v06_work_inventory.rs`'s equipmods enumeration: either (a) mint only ONE unit per object, citing
the SAME coordinate the ingest pipeline resolves to, or (b) have `shape_ledger.py`'s join treat a
`.COPY=`-linked pair as one identity. (a) is preferred — one real object should be one unit.

### Group B — `pathfinder_unchained` `equipment_modifier` (4 of 19): correct content, wrong directory (kind misclassification)

`data/corpus/pathfinder_unchained/equipment/0_abp_enhancement_to_{weapon,ammunition,armor,shield}.json`
exist, `category: "equipmods"`, and cite the EXACT (book, source_file, source_line) coordinates the
`abp_0_*` walker units expect (`pu_equipmods.lst:4` etc. — verified byte-for-byte). But they are
written FLAT under `equipment/`, not `equipment/equipmods/`. `shape_ledger.py::build_corpus_index`
derives `kind` from the directory one level under the book (`kind_from_path_parts`), so these four
records index as `kind="equipment"`, not `equipment_modifier` — the exact kind-blind-join collision
class `decisions.md §25`'s discovery-forward already names (`build_corpus_index`'s own docstring:
"a unit's join could be answered by the WRONG kind's record" — here it's answered by NO kind, since
`equipment` already has its own real units and this one collides with nothing, it just sits
unindexed under the wrong key).

`pathfinder_unchained` is **absent** from `gen_equipment_gap_tables.rs`'s `BOOK_INPUTS` table
(confirmed: `grep -n pathfinder_unchained src/bin/gen_equipment_gap_tables.rs` returns only a doc-
comment reference, no `BookInput` entry), so these 4 files were NOT written by that generator.
`ingested_at: "2026-08-03T19:41:44Z"` predates the equipmods-nesting convention
(`equipment_gap.rs`'s `write_dir = if is_modifier { book_out.join("equipmods") } else { book_out
}`, introduced later) — these are leftovers from an earlier, unidentified write path that never
adopted the subdirectory convention. Next cycle: locate that write path (not
`gen_equipment_gap_tables.rs`, not `cache_gen::hand_authored_equipment` — neither names
`pathfinder_unchained`) or, more simply, move the 4 existing files into
`equipment/equipmods/` and re-verify their citations/fixtures survive the move (a data move, not a
regeneration, so no re-verification-stamp loss).

### Group C — `adventurers_guide` `equipment_modifier` (1 of 19): file simply missing from the generator's input list

`gen_equipment_gap_tables.rs:496-500`'s `adventurers_guide` `BookInput` lists exactly 3 files
(`ag_equip_arms_armor.lst`, `ag_equip_general.lst`, `ag_equip_magic_items.lst`) — `ag_equipmods.lst`
(confirmed present in the oracle checkout, containing "Medium Grey Maiden Plate	KEY:Special Ability
~ Agile Maiden ~ Armor...") is never read. Trivial, single-line fix: add
`"pathfinder/paizo/roleplaying_game/adventurers_guide/ag_equipmods.lst"` to that book's `files`
list and regenerate `equipment_gap_tables.rs` (`cargo run --locked --bin gen_equipment_gap_tables`).

### Group D — the `_pfs/` PFS-overlay-vs-base-file citation mismatch (11 of 30, spanning `equipment`×9 and `ability`×1, across 4 books)

**One root cause, confirmed independently in 4 places:**

| Unit | Walker cites | Real corpus record cites |
|---|---|---|
| `ultimate_magic:equipment:lab_journal_of_constance_inflix` | `pfs_um_equip_general.lst:9` | `um_equip_general.lst:17` (`codex_named_unit_..._17.json`) |
| `ultimate_magic:equipment:journeyman_book_of_rul_thaven` | `pfs_um_equip_general.lst:13` | `um_equip_general.lst:21` (`..._21.json`) |
| `ultimate_magic:equipment:insights_of_far_seeing_taernis` | `pfs_um_equip_general.lst:16` | `um_equip_general.lst:25` (`..._25.json`) |
| `ultimate_magic:equipment:master_books_of_rul_thaven` | `pfs_um_equip_general.lst:20` | `um_equip_general.lst:31` (`..._31.json`) |
| `ultimate_magic:equipment:library_of_the_dancer_of_skins` | `pfs_um_equip_general.lst:22` | `um_equip_general.lst:33` (`..._33.json`) |
| `ultimate_magic:equipment:the_formulae_of_master_gebr` | `pfs_um_equip_general.lst:23` | `um_equip_general.lst:34` (`..._34.json`) |
| `advanced_class_guide:equipment:dust_knuckles_forget` | `pfs_acg_equip.lst:6` | `acg_equip.lst:231` (`dust_knuckles.json`) |
| `advanced_class_guide:equipment:false_face_forget` | `pfs_acg_equip.lst:7` | `acg_equip.lst` (`false_face.json`) |
| `bestiary_3:equipment:ranged_cannon` | `pfs_b3_equip_arms_armor.lst:10` | `b3_equip_arms_armor.lst:15` (`ranged_cannon_clockwork_goliath.json`) |
| `ultimate_campaign:ability:trait_corpse_cannibal` | `pfs_uca_abilities_traits.lst:7` | `uca_abilities_traits.lst:281` or `:282` (both carry "Corpse Cannibal" text; not yet disambiguated — see below) |

Every PFS overlay file (`_pfs/pfs_*.lst`) declares a **legality annotation** (a `.FORGET` removal
flag, a `TYPE:PFSNotLegal` restriction) about an item that is DECLARED and CITED elsewhere, in the
book's base file. `docs/work-inventory.json`'s walker enumerates the overlay row as if it were the
item's own citable declaration; every real ingest pipeline (`equipment_gap.rs`'s explicit `.FORGET`
guard; the base-file-only scan every book's `BOOK_INPUTS`/`hand_authored_equipment` entry performs)
correctly resolves to — and cites — the base file instead, so a real corpus record already exists
for every one of these 10 units, just under a different `(source_file, source_line)` than the
walker's unit points to. **None of these 10 need new ingest — they need the walker's citation fixed
to resolve through to the base declaration**, the same resolution `find_citation`'s `.COPY=`/
first-column search already performs for ordinary content.

**Generic fix, one mechanism for ~⅓ of this cycle's whole population:** teach
`v06_work_inventory.rs`'s equipment/ability enumeration to detect a `_pfs/`-directory or
`pfs_`-prefixed source file and resolve through to the base declaration's citation (by `KEY:`/name
match, same primitive `find_citation` already implements) rather than citing the overlay row
directly. Given `§17`'s standing instruction, this is exactly the "generic pass, not per-object
work" shape to prioritize — worth re-sweeping the WHOLE corpus for the `_pfs/`-citation shape once
built, not just these 10.

`ultimate_campaign`'s exact line (281 vs 282) needs one more read to disambiguate before a fix can
cite it precisely — flagged, not guessed.

### Group E — `ultimate_equipment` `equipment` (1 of 10, `otyugh_hide`): genuinely un-ingested, §24 rename never applied

"Otyugh Hide" (`ue_equip_arms_armor.lst:66`) carries `NAMEISPI:YES` in the source LST. No corpus
record exists anywhere under `data/corpus/ultimate_equipment/` for it (`find ... -iname
"*otyugh*"` returns nothing). `ultimate_equipment` is NOT in `gen_equipment_gap_tables.rs`'s
`BOOK_INPUTS` for the arms/armor file (only `ue_equipmods.lst` is listed there) — its `equipment`
kind runs through a separate, dedicated pipeline (`gen_cache_ultimate_equipment.rs` /
`cache_gen::ultimate_equipment`, referenced in `equipment_gap.rs`'s own doc comments as the
already-shipped, pre-`decisions.md §24` per-book table). That pipeline predates the `§24`
Codex-neutral-rename mechanism (`resolve_name_or_rename`, landed in `cache_gen::equipment_gap`) and
has evidently never had it ported — a `NAMEISPI:YES` row there is still being dropped outright
rather than ingested-and-renamed. **This is the one group in my scope that is a real ingest gap**,
not a citation/kind-routing defect. Fix: port `resolve_name_or_rename`'s call into
`gen_cache_ultimate_equipment.rs`'s equivalent per-record loop, the same way `hand_authored_
equipment.rs` already reuses it (see that file's own doc comment, "Reuse, not duplication").

## `collect_base_fields` cross-book blindness — re-derived correction, not confirmed

The dispatch brief's "Also fix the third instance" section states `gen_equipment_gap_tables.rs::
collect_base_fields`'s cross-book blindness is "confirmed and un-fixed." Re-derivation against
current HEAD does not support this:

- `collect_base_fields`'s own doc comment (`gen_equipment_gap_tables.rs:701-709`) already states it
  builds its lookup "from every PLAIN (non-`.COPY=`) row across **a book's own input files**".
- Its single call site in `main()` (`:918-920`) sits inside `for input in BOOK_INPUTS { ... }` and
  passes only `file_texts` built from `input.files` — that book's own files, nothing wider.
- No other cross-book-wide corpus/text scan was found in this binary (`held`, similarly, is a
  `BTreeMap<&'static str /* book */, _>`, correctly partitioned per book).

Logged: `scripts/retro.py correction` id `1787515659162-t9-onboarding-60d66c`
(`docs/retro/events/t9-onboarding.jsonl`, subject "dispatch-brief (equipment_modifier-rootcause
cycle)"). **No fourth site of this defect class was found in this binary either** — the search was
scoped to `src/bin/gen_equipment_gap_tables.rs` only; a corpus-wide grep for the same shape (a
book-keyed lookup built from an un-scoped file list) was not run this cycle and is a reasonable
next step if the defect class recurs elsewhere.

## Why no fix landed this cycle

The dispatch brief explicitly scoped this cycle to root-cause tracing first: *"root cause is NOT
yet isolated... So your first deliverable is the actual root cause, per group, not a fix."* That
deliverable is above, for all five groups (A-E), covering all 30 units in scope (19+10+1). Four of
the five groups (A, B, D, E) name a specific, already-scoped fix location; group C's fix is a single
line already fully specified. None was implemented this cycle so this receipt could ship the
complete, re-derived root-cause picture across the whole population rather than a partial fix to
one group leaving the rest re-investigated by a successor cycle — implementing any one group's fix
would have consumed the remaining budget without covering the others. **Card 11 stays
in-progress**; the next cycle should implement groups C (trivial), D (highest leverage, ~⅓ of this
population, generic), B, A, then E in that order.

## `docs/work-inventory.json` regen dependency

Per the dispatch brief, `corpus_literal_sweep` is `clean:false` and a sibling lane owns the fix;
this cycle did not touch `corpus_literal_sweep.rs` or `class_feature` records, and did not attempt
a regen. Group D's finding (the walker's PFS-overlay-vs-base citation mismatch) and Group A's
finding (the duplicate `.COPY=`-alias unit) are BOTH walker-enumeration defects that a bare regen
would NOT fix on its own — the walker must be corrected first, then regenerated. Group E's fix is
independent of the walker/regen entirely (it is a corpus-record-generator gap). Group B and C are
also generator-side, independent of the walker.

## Environment / oracle

```
PCGEN_REPO_DIR=<worktree>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen
scripts/fetch-pcgen-oracle.sh --dest "$PCGEN_REPO_DIR"   # bootstrapped this cycle, fresh worktree
scripts/verify.sh --only preflight-oracle                # PASS, oracle at pin 7f818006e371188e5717fd18d74d18a420747fc6
```
