# Object-definition rules — SD-32 Gate 0 census (AT-32-G0-002)

Written by `scripts/census_independent.py` (docstring is the canonical source; this file is the
human-readable summary + the live-run figures). Re-derive with:

```bash
export PCGEN_REPO_DIR="$(git rev-parse --show-toplevel)/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen"
export PCGEN_CORPUS_ROOT="$PCGEN_REPO_DIR/data"
python3 scripts/census_independent.py --pcgen-root "$PCGEN_CORPUS_ROOT" \
  --inventory docs/work-inventory.json \
  --output docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/diff.json
```

Corpus SHA: `grep PCGEN_ORACLE_SHA scripts/pcgen-oracle-pin.env`.

## Row identity rules (apply to every `.lst` row, regardless of kind)

1. **`.MOD` suffix on the identity field** — a continuation of an object defined elsewhere. Not a
   new unit. 23,625 such rows in the live run (`mod_continuation`).
2. **`.COPY=<name>` in the identity field** — a genuinely new named object, cloned from an existing
   one. Counted as a unit of its file's kind and tallied separately (`copy_derivation`, 2,338 in
   the live run) so the derivation stays visible rather than hidden inside a plain count.
3. **`.FORGET` suffix** — a removal directive, not a unit (`forget_directive`, 0 in the live run).
4. **Comment lines (`#...`), blank lines, and pure-directive lines** (the identity field itself
   contains `:` before any tab, e.g. `SOURCELONG:...`) are not rows at all and are skipped.

## Per-kind filename classification (the ten kinds AT-32-G0-002 names)

| Kind | Filename rule | Live count |
|---|---|---|
| `feat` | filename contains `feat`, OR a bare `*_abilities.lst` row whose `CATEGORY:` tag is `FEAT` | 2,548 |
| `class` | filename contains `class` (and not `companion`/`abilit`) | 2,377 |
| `spell` | filename contains `spell` | 2,777 |
| `monster` | filename contains `race` (not `companion`, not `_pc`), book is in the monster-book set (`bestiary*`, `monster_codex`, `bonus_bestiary`) — PCGen encodes creature stat blocks as `RACE`-kind LST rows | 1,218 |
| `monster_ability` | filename contains `abilit` + `_race`, book in the monster-book set | 3,784 |
| `equipment` | filename contains `equip` (not `equipmod`) | 6,197 |
| `equipment_modifier` | filename contains `equipmod` | 1,582 |
| `companion` | filename contains `companion` (races_companion, classes_companion, kits_companion, abilities_companion, companionmods) | 1,676 |
| `race` | filename contains `race` (not `companion`), book NOT in the monster-book set (or is `_pc` suffixed even inside one) | 241 |
| `race_trait` | filename contains `abilit` + `_race`, book NOT in the monster-book set | 5,637 |

**Ten-kind total (AT-32-G0-002's original list): 28,037 units** (figures above are as first
measured; re-derive live with the command at the top of this file — they have since moved as card
15 cycles land).

## Kinds added after AT-32-G0-002 (`decisions.md §12b`, card 15)

`census_independent.py`'s `ADDED_KINDS` tuple (kept separate from `TEN_KINDS` so AT-32-G0-002's own
"ten kinds" text stays a truthful description of what it originally named). Each addition needs a
matching `Kind::*` variant in `src/bin/v06_work_inventory.rs` so the walker and the inventory agree
(§12b's own acceptance bar).

**`decisions.md §17` (2026-08-23) replaced "one kind per cycle" with a generic mechanism.** A kind
whose only rule is "this filename substring means this kind" — every kind below — is a data row in
`file_kind`'s `SIMPLE_FILENAME_KINDS` table (`v06_work_inventory.rs`) plus a matching entry in
`_classify_kind_by_filename` here, not a tour of `enumerate_file`/`refine_kind`/duplicate-identity
handling. `skill` proved the mechanism (card 15); `template`/`deity`/`power`/`domain`/`language`
landed together through the same path, in one cycle, once the mechanism existed. See
`SIMPLE_FILENAME_KINDS`'s own doc comment for exactly what a kind of this shape still requires (one
`Kind::` enum variant, one table row, one `classify()` arm — all three are *data or a one-line
exhaustive-match arm*, never a new code path).

| Kind | Filename rule | Census raw count | Landed in inventory | Cycle |
|---|---|---:|---:|---|
| `skill` | filename contains `_skills` | 170 | 149 (21 `core_essentials/ce_skills.lst` rows correctly deleted by the pre-existing `decisions.md §16` core_essentials-residual guard — real population growth, not a predicate widening; see `CORE_ESSENTIALS_RESIDUAL_DELETION_CEILING`'s doc comment, raised 117→138) | `card-15-enumerate` |
| `template` | filename contains `_templates` | 2,343 | see re-derive command below (core_essentials residual guard raised again, 138→171, for real `_templates`/`_languages` content — see next section) | `generic-enumeration` |
| `deity` | filename contains `_deities` | 460 | 460 (no core_essentials residual — no `core_essentials/races/<slug>/*_deities.lst` file exists) | `generic-enumeration` |
| `power` | filename contains `_powers` (Ultimate Psionics only) | 421 | 421 | `generic-enumeration` |
| `domain` | filename contains `_domains` | 183 | 183 | `generic-enumeration` |
| `language` | filename contains `_languages` | 143 | see re-derive command below | `generic-enumeration` |

**Consequence of landing `skill`, remediated by the `gate3-budget-repair` cycle:** landing
`skill`'s 149 real units raised Gate 3's (`scripts/shape_coverage_standing_gate.py`) `no_record`
population share from 10,419/24,914 to **10,530/25,055** (re-derive: `python3 scripts/shape_ledger.py
--inventory docs/work-inventory.json --corpus-root data/corpus --output /tmp/l.json && python3 -c
"import json,collections; r=json.load(open('/tmp/l.json'))['rows']; print(collections.Counter(x
['join_status'] for x in r))"`) — the `card-15-enumerate` cycle receipt's own quoted figure of
10,560 does not reproduce and is corrected by `scripts/retro.py correction`
(`docs/retro/events/gate3-budget-repair.jsonl`). None of `skill`'s rows have ever been ingested
into `data/corpus` under any kind, so this growth is the predicted, structural shape of every
remaining new-kind landing (`decisions.md §12b`), not a regression. The budget mechanism was
widened from a pure shrink-only ratchet to an **evidence-gated repin**: the constants moved to
10530/25055 together with a matching, committed, git-verifiable entry in
`no_record_budget_provenance.jsonl` (evidence commit `d904eceb6bda813f5a6d48a815a2b4df80d604bd`,
that cycle) — see `scripts/shape_coverage_standing_gate.py`'s module docstring and
`BudgetProvenanceTest` for the full mechanism and its anti-gaming tests.

`kit` (1 unit, `core_essentials/races/kitsune/kitsune_races.lst`) was **not** given a `Kind::Kit`
variant — investigation found it was never real content at all. The census's own `"kit" in b`
filename check false-positived on `kitsune_races.lst` (the race NAME "Kitsune" contains the
substring "kit"), diverting one real `race`-kind row into `kind_unenumerable["kit"]`.
`v06_work_inventory.rs`'s `file_kind` never had a "kit" branch and always resolved this file to
`Kind::Race` — the reconciliation this section exists to keep (§12b's "the two must agree" bar)
required *narrowing* the census's check to `"_kits" in b` (the real filename convention every
genuine `*_kits.lst` file uses), not adding a kind. Every genuine `*_kits.lst`/`*_kits_race.lst`
file uses PCGen's `STARTPACK:`-block format, whose rows all carry a `:` in their own first field and
are therefore already skipped as directive lines by the row parser regardless of bucket — verified
live, 48 real `_kits*.lst` files, 0 rows, under either the old or the new rule. Re-derive:
`git log --oneline -1 -- scripts/census_independent.py` and the `_kits` narrowing's own commit.

**Re-derive the current populations:**

```bash
export PCGEN_CORPUS_ROOT="$(git rev-parse --show-toplevel)/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data"
python3 scripts/census_independent.py --pcgen-root "$PCGEN_CORPUS_ROOT" \
  --inventory docs/work-inventory.json \
  --output /dev/stdout | python3 -c "import json,sys; print(json.load(sys.stdin)['counts_by_kind'])"
jq '.totals.by_kind' docs/work-inventory.json
```

**Known consequence, not remediated this cycle:** landing `template`/`deity`/`power`/`domain`/
`language`'s real units raises Gate 3's (`scripts/shape_coverage_standing_gate.py`) `no_record`
population share further, past the `gate3-budget-repair` cycle's own just-repinned 10,530/25,055 —
none of these rows have ever been ingested into `data/corpus` under any kind, so `scripts/verify.sh
--only shape-coverage-standing-gate` FAILs again (`no_record_budget_exceeded=True`,
13,975/28,490 — re-derive: `python3 scripts/shape_coverage_standing_gate.py --inventory
docs/work-inventory.json`). This is the gate correctly reporting real, previously-invisible
uncovered content, not a defect in the enumeration, and not a regression of the repair cycle's own
fix (the evidence-gated repin mechanism itself is untouched by this cycle) — it is the same
structural shape `decisions.md §14`/the `gate3-budget-repair` cycle both already named: every
new-kind landing that outpaces ingestion moves this number, by design, until a kind gets an engine
table or the budget is repinned again with fresh evidence. Not this cycle's to fix or paper over.

## Kind-unenumerable — named and counted, not pretended to be zero (AT-32-G0-002)

Real, named PCGen objects that do not map onto the ten kinds above. **27,847 units total**
(`total_kind_unenumerable_units`), broken out in `diff.json .kind_unenumerable`:

* **`class_feature` — 18,231 units.** `*_abilities_class*.lst` files. **DISCOVERY:** AT-32-G0-002's
  ten-kind list does not include `class_feature`, even though it is the single largest kind in
  `docs/work-inventory.json` (`totals.by_kind.class_feature` = 15,439 — the live inventory's
  largest kind by a wide margin). This walker does not force class-feature rows into `feat` or any
  other kind; it names the bucket and reports the count. Filed as a `## DISCOVERED` forward in
  `progress.md` and a deferral in the retro log (`docs/retro/events/gate-0-census.jsonl`) rather
  than resolved unilaterally — extending the ten-kind list is an operator-scoping question, not an
  implementation one.
* **`ability_category:<CATEGORY value>` — 5,891 units across 25 named sub-buckets** (`Special
  Ability` 3,436, `Internal` 839, `Words of Power` 369, `Ability Focus` 272, `Path Dabbling` 128,
  `Class Skill` 102, `Intelligent Item` 100, `Background` 72, `Afflictions` 70, `Aligned Class` 52,
  and 15 smaller buckets down to 1). Bare `*_abilities.lst` files hold multiple PCGen ability
  categories in one file, distinguished only by each row's own `CATEGORY:` tag — filename alone
  cannot classify these, so the walker classifies per-row and reports every category value it saw,
  including one truly unclassifiable row (`ability_category:UNKNOWN`, 15 units, rows with no
  `CATEGORY:` tag at all).
* **`template_row` — 2,343 units.** `*_templates.lst` files (racial/creature templates like
  "Advanced" or "Giant" that get applied to a base unit of some other kind — they are not
  themselves one of the ten kinds).
* **`deity` — 460**, **`power` — 421**, **`domain` — 183**, **`language` — 143**, **`kit` — 1.**
  Real named content objects with their own `.lst` files, outside the ten-kind roster.
* **`unclassified:<filename>` — 174 units across 10 skills files** (`cr_skills.lst` 110,
  `pu_skills.lst` 24, `ce_skills.lst` 21, and 7 smaller). Skill definitions are a real content kind
  this walker's filename table does not recognize by name; rather than silently guess a kind, every
  such file is named individually so a future cycle can decide whether `skill` becomes an eleventh
  kind or these are out of SD-32's scope entirely.

## Non-object files — skipped, not silently absorbed

253 files carrying pure engine/system wiring (datacontrols, datatables, variables,
globalmodifiers, stat/align/save/dynamic tables, biosettings, ability-category definitions,
weapon/armor/shield proficiency tables) hold no discrete named narrative objects and are excluded
entirely from both the kind and kind-unenumerable counts. The full file list is in `diff.json
.non_object_files` so the exclusion is auditable rather than silent.
