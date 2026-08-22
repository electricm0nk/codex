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

**Ten-kind total: 28,037 units** (`total_counted_units`).

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
