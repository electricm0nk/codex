# Card 15 — `class_feature` bucket reconciliation (`decisions.md §12b`)

**Lane:** census-scope-closure / measurement lane — `class_feature`, 18,231 units in
`kind_unenumerable` (`artifacts/gate-0-census-closure/diff.json`), disagreeing with
`docs/work-inventory.json`'s `totals.by_kind.class_feature` of 15,439 by 2,792 units, direction
previously unknown.

**Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`;
`scripts/verify.sh --only preflight-oracle` → PASS at the start of this cycle).

**Populations named, per `decisions.md §12c`:** every count below is the `class_feature`
sub-population of one of the bundle's three named totals — the census's `kind_unenumerable`
bucket (18,231, from the full corpus walk, not the not-done ledger), and the inventory's
`totals.units` all-origins population (15,439, out of 38,391). Neither is the ledger's 24,914
not-done population; §2 below shows the ledger only ever saw the 15,439 side of this split.

## 0. Answer, up front

**Both walkers were partly right and partly wrong, and the gap decomposes exactly (no residual
error term):**

```
18,231 (census kind_unenumerable["class_feature"])
  = 15,438   already correctly tracked in docs/work-inventory.json (by physical corpus location)
  +  2,614   CATEGORY:Internal rows — real PCGen bookkeeping records, NOT independent objects
  +    179   real class_feature objects the inventory currently drops — should be added
  --------
  = 18,231
```

The leading hypothesis in `decisions.md §12b` — that `mod_continuation`/`copy_derivation` inflate
the census's count — is **refuted, with proof** (§1). The counter-hypothesis — that the inventory
skips content the walker reaches — is **confirmed for 179 units** (§3), and a third, previously
unnamed cause explains the other 2,614 (§2): the walker under-applies its own existing
`CATEGORY:Internal` exclusion rule.

**Disposition:**

- **(A) IS an object — 15,438 + 179 = 15,617** real `class_feature` records. Already tracked, under
  the exact same kind name (`class_feature`), in `docs/work-inventory.json` for 15,438 of them; the
  other 179 are not currently tracked anywhere and should be added to that same kind (§3, §5). No
  new kind is needed — this is not a "different name for a tracked thing," it is a coverage gap in
  an existing tracked kind.
- **(B) NOT an object — 2,614** `CATEGORY:Internal` rows. Proven by class in §2: every one of the
  2,614 carries the literal field `CATEGORY:Internal`, PCGen's own marker for a helper/bookkeeping
  record rather than a player-facing feature. `scripts/census_independent.py` already applies this
  exact exclusion to *other* `_abilities_*.lst` files (its `ability_category:Internal` bucket, 839
  units elsewhere in `kind_unenumerable`) — it simply never applies it to `_abilities_class.lst`
  files, where the whole-file bucket match short-circuits before the row-level category check.

Separately, and **not part of the 18,231 population above** (§4): one `.MOD`-orphan-rescue unit
(`ultimate_wilderness:class_feature:exotic_heritage`) is tracked by the inventory but is correctly
absent from the 18,231, because its own identity literally ends `.MOD` and census correctly tallies
it under `mod_continuation` instead. Its existence does not change the 18,231/15,439 reconciliation;
it is noted for "sum the piles" completeness.

## 1. The leading hypothesis (`.MOD`/`.COPY=` inflation) — tested, refuted

`decisions.md §12b` names `mod_continuation: 23,625` / `copy_derivation: 2,338`
(diff.json-wide) as the leading hypothesis and requires it be tested, not assumed.

**Test:** `scripts/census_independent.py`'s own `count_objects()` (scripts/census_independent.py
lines 400-433) increments `mod_continuation` and then `continue`s — i.e. it **already excludes**
every `.MOD`-suffixed row from `kind_unenumerable` before the bucket count is incremented. This is
directly checkable by re-deriving the `class_feature`-scoped `.MOD` count and confirming the
arithmetic:

```bash
python3 - <<'PY'
import sys, os, json
sys.path.insert(0, "scripts")
import census_independent as ci

pcgen_root = os.environ["PCGEN_CORPUS_ROOT"]
with open("docs/work-inventory.json") as f:
    inventory = json.load(f)
book_dirs = ci.discover_book_dirs(pcgen_root)
scope = ci.classify_scope(book_dirs, inventory)
pathfinder_root = os.path.join(pcgen_root, "pathfinder")

total = mod = forget = 0
for bd in scope.in_scope:
    for dirpath, _, filenames in os.walk(os.path.join(pathfinder_root, bd.rel_path)):
        for fn in filenames:
            if not fn.lower().endswith(".lst"):
                continue
            bucket, key = ci._classify_kind_by_filename(fn, bd.book_id)
            if not (bucket == "kind_unenumerable" and key == "class_feature"):
                continue
            for identity, raw in ci._parse_lst_rows(os.path.join(dirpath, fn)):
                total += 1
                iu = identity.upper()
                if iu.endswith(".FORGET"):
                    forget += 1
                elif iu.endswith(".MOD"):
                    mod += 1
print("total rows", total, "mod", mod, "forget", forget, "-> unit count", total - mod - forget)
PY
```

**Result:** `total rows 23435 mod 5204 forget 0 -> unit count 18231` — exactly the
`diff.json["kind_unenumerable"]["class_feature"]` figure
(`python3 -c "import json; print(json.load(open('docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/diff.json'))['kind_unenumerable']['class_feature'])"`
→ `18231`). **The 18,231 already has every `.MOD` continuation row removed.** `copy_derivation`
rows (`.COPY=` variants) *are* counted — correctly, per `decisions.md §"object-definition rules"` —
`.COPY=` declares a new named record, not a continuation; `census_independent.py` records
`copy_derivation` as a diagnostic counter alongside the count, never as a double-count. Zero
`.COPY=` rows exist in the `class_feature` file set specifically (same script, `is_copy` count = 0
over these 259 files). **Hypothesis refuted:** the gap is not `.MOD`/`.COPY=` shaped at all.

## 2. `CATEGORY:Internal` — the walker's own rule, inconsistently applied (2,614 units)

`src/bin/v06_work_inventory.rs`'s `enumerate_file` (lines 1944-1949) drops **every** row in
**every** file whose fields contain the literal `CATEGORY:Internal` (or a `CATEGORY=Internal|`
directive prefix) — unconditionally, before any kind-specific handling, and documented back to
SD28-E15. `scripts/census_independent.py` applies the *identical* rule, but only for bare
`*_abilities*.lst` files that are neither `_class` nor `_race` files (its `row_dependent` branch,
`_classify_kind_by_filename` lines 320-321) — the ten `ability_category:*` buckets in `diff.json`
(839 for `Internal` alone, elsewhere in `kind_unenumerable`) are exactly this rule's output on
*those* files. For `_abilities_class.lst` files, `_classify_kind_by_filename` resolves the whole
file to `class_feature` at line 319 and never re-checks `CATEGORY:` per row — so `Internal` rows in
these 259 files fall straight into the `class_feature` count instead of a separate
`ability_category:Internal` bucket.

**Proof by class**, not by sample — every one of the 2,614 unmatched rows in §0's split carries the
literal field, verified with the same re-derive script as §3 (the `is_internal` predicate applied
before the join; see the script in §5, which reports both counts from one run). Two representative
records, both from `core_rulebook`, `cr_abilities_class.lst`:

```
Damage Reduction ~ All      CATEGORY:Internal        DR:ClassFeatureDR_ALL/-
Damage Reduction ~ Silver   CATEGORY:Internal        DR:ClassFeatureDR_Silver/-
```

Neither carries a `DEFINE:`/`BONUS:` formula token a player-facing feature would (both classify as
`F0`, §6) — they exist purely so PCGen's own DR-tracking machinery has a named variable to hang a
`DR:` directive on. **Disposition: (B) NOT an object**, by the exact same rule the walker already
applies elsewhere — this is a consistency fix to `scripts/census_independent.py`'s bucketing (its
own row-category check should run for `_abilities_class.lst` files too, filing these under
`ability_category:Internal` instead of `class_feature`), not a discovery of new content.

## 3. The 179 real units the inventory does not currently track

After removing `.MOD`/`.FORGET`/directive rows (§1) and `CATEGORY:Internal` rows (§2) from the
18,231, **15,617** real corpus rows remain in the census's `class_feature` walk. Joining each one
to `docs/work-inventory.json`'s `units` array by physical location (`book`, `source_file`,
`source_line` — immune to any KEY-vs-display-name ambiguity, since it never looks at either)
finds **15,438** with a matching inventory unit and **179** with none.

None of the 179 are `CATEGORY:Internal`, `.MOD`, `.FORGET`, or directive rows — they were excluded
from that possibility by construction before the join. Each is a real, uniquely-located corpus row
carrying ordinary `class_feature`-shaped content (`CATEGORY:Special Ability` / `CATEGORY:Ability
Focus` / `CATEGORY:Class` fields, `DEFINE:`/`BONUS:` tokens in most of them — §6 gives the family
breakdown). Two representative examples, chosen because their sibling rows in the *same* file *are*
tracked (ruling out "wrong file kind" as an explanation):

- `advanced_players_guide/apg_abilities_class.lst:917` — `Witch Hex ~ Blight` (a basic Witch hex).
  Its immediate neighbours (lines 918-930, the other 13 basic hexes) are also missing; the file's
  *later* rows for the same hexes' Major/Grand-hex "Ability Focus" alias forms (lines 931-943,
  `Witch Hex ~ Agony` etc.) **are** tracked, field-for-field identical in shape. Root cause not
  fully pinned within this cycle's budget — circumstantial evidence (the `class_feature_pool_catalog`
  machinery `src/bin/v06_work_inventory.rs` lines 2650-2749 tracks a `choice:witch_hex` pool group,
  `wiring_class::` line ~8259) points at a pool-membership de-duplication step, but this is named
  as a hypothesis, not a proven mechanism — reported honestly rather than rounded to a guess.
- `ultimate_wilderness/uw_abilities_class_fap.lst:9` — `Vulture` (a favoured-animal-pool entry, same
  shape).

**Disposition: (A) IS an object.** All 179 belong to the existing `class_feature` kind (same name,
same file-kind rule that already classifies their siblings); none needs a new kind. Recommendation
for the integration cycle: extend `v06_work_inventory.rs`'s enumeration so these 179 physical rows
mint inventory units the same way their siblings in the same files already do — the exact list is
reproducible from the script in §5 (`residual` list, 179 entries, one line each with
book/file/line/identity).

**Per-book distribution** (not concentrated in one or two books — spread across 11 of the 38
in-scope books, roughly proportional to each book's overall `class_feature` volume):

| Book | Residual (should-be-added) |
|---|---:|
| advanced_class_guide | 52 |
| core_rulebook | 31 |
| ultimate_magic | 25 |
| advanced_players_guide | 24 |
| occult_adventures | 16 |
| advanced_race_guide | 13 |
| ultimate_combat | 6 |
| monster_codex | 5 |
| ultimate_wilderness | 4 |
| ultimate_intrigue | 2 |
| adventurers_guide | 1 |
| **Total** | **179** |

The **full 2,792-unit gap** (before §2/§3's split), broken down the same way, confirms it is not
concentrated either — it tracks each book's overall `class_feature` volume (largest gaps in the
largest spellcaster-heavy books: `ultimate_magic` 738, `core_rulebook` 606, `advanced_class_guide`
351, `advanced_players_guide` 295 — all consistent with `CATEGORY:Internal` rows scaling with a
book's total class-feature count, not a per-book anomaly).

## 4. Outside the 18,231 population: one `.MOD`-orphan rescue

`docs/work-inventory.json` carries `ultimate_wilderness:class_feature:exotic_heritage`, `origin:
"mod_only"`, at `uw_abilities_class.lst:1401`. That row's raw identity is literally
`CATEGORY=FEAT|Exotic Heritage.MOD` — it ends `.MOD`, so `census_independent.py` correctly tallies
it under `mod_continuation`, never under `kind_unenumerable["class_feature"]`. The inventory's own
`mod_only_rescue` path (documented at `src/bin/v06_work_inventory.rs` line 314, "a `.MOD` record
whose base name appears nowhere else in the corpus") promotes it to a real unit because no base
`Exotic Heritage` declaration exists anywhere in the corpus to modify. **This is correct behaviour
on both sides** — census's `mod_continuation` bucket is doing its job (this row genuinely is a
`.MOD` row), and the inventory's rescue is documented, deliberate, and outside this bucket's scope.
Named here only so the reconciliation in §0 accounts for every unit either walker counts anywhere
near `class_feature` — `15,439 (inventory) = 15,438 (matched to the 18,231) + 1 (this rescue,
outside the 18,231 entirely)`.

## 5. Full re-derive script (committed here; copy-paste runnable against the pinned corpus)

```bash
export PCGEN_CORPUS_ROOT="$(git rev-parse --show-toplevel)/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data"
python3 - <<'PY'
import sys, os, json
from collections import Counter
sys.path.insert(0, "scripts")
import census_independent as ci
import shape_ledger as sl

pcgen_root = os.environ["PCGEN_CORPUS_ROOT"]
with open("docs/work-inventory.json") as f:
    inventory_json = json.load(f)
book_dirs = ci.discover_book_dirs(pcgen_root)
scope = ci.classify_scope(book_dirs, inventory_json)
pathfinder_root = os.path.join(pcgen_root, "pathfinder")

inv_lookup = {(u["book"], u["source_file"], u["source_line"])
              for u in inventory_json["units"] if u.get("kind") == "class_feature"}

census_rows, internal_rows = [], 0
for bd in scope.in_scope:
    for dirpath, _, filenames in os.walk(os.path.join(pathfinder_root, bd.rel_path)):
        for fn in sorted(filenames):
            if not fn.lower().endswith(".lst"):
                continue
            bucket, key = ci._classify_kind_by_filename(fn, bd.book_id)
            if not (bucket == "kind_unenumerable" and key == "class_feature"):
                continue
            with open(os.path.join(dirpath, fn), encoding="utf-8", errors="replace") as fh:
                for lineno, raw in enumerate(fh, 1):
                    line = raw.rstrip("\n")
                    if not line.strip() or line.lstrip().startswith("#") or "\t" not in line:
                        continue
                    identity = line.split("\t", 1)[0]
                    if ":" in identity:
                        continue  # directive line
                    iu = identity.upper()
                    if iu.endswith(".FORGET") or iu.endswith(".MOD"):
                        continue  # §1 / §4
                    fields = line.split("\t")
                    is_internal = (any(f.strip() == "CATEGORY:Internal" for f in fields)
                                   or identity.startswith("CATEGORY=Internal|"))
                    if is_internal:
                        internal_rows += 1  # §2
                        continue
                    census_rows.append((bd.book_id, fn, lineno, identity, line))

matched = sum(1 for r in census_rows if (r[0], r[1], r[2]) in inv_lookup)
residual = [r for r in census_rows if (r[0], r[1], r[2]) not in inv_lookup]

print("census kind_unenumerable[class_feature] (diff.json)       :", 18231)
print("  minus CATEGORY:Internal (§2, NOT an object)              :", internal_rows)
print("  minus matched-to-inventory (already tracked)              :", matched)
print("  = residual, real object, not yet tracked (§3)             :", len(residual))
assert internal_rows + matched + len(residual) == 18231

def family(line):
    formulas = []
    for f in line.split("\t"):
        f = f.strip()
        if f.startswith("DEFINE:"):
            seg = sl.extract_formula_segment("DEFINE", f[7:])
        elif f.startswith("BONUS"):
            k, _, v = f.partition(":")
            seg = sl.extract_formula_segment("BONUS", v)
        else:
            continue
        if seg is not None:
            formulas.append(seg)
    if not formulas:
        return sl.FAMILY_F0_NO_FORMULA
    pri = {fid: i for i, (fid, *_r) in enumerate(sl.FAMILIES)}
    best, best_rank = None, None
    for seg in formulas:
        fam = sl.classify_formula(seg)
        rank = pri.get(fam, len(sl.FAMILIES) + 1)
        if best_rank is None or rank < best_rank:
            best, best_rank = fam, rank
    return best

print("residual family distribution:", Counter(family(r[4]) for r in residual))
PY
```

Running this against the pinned oracle (`7f818006e371188e5717fd18d74d18a420747fc6`) prints:

```
census kind_unenumerable[class_feature] (diff.json)       : 18231
  minus CATEGORY:Internal (§2, NOT an object)              : 2614
  minus matched-to-inventory (already tracked)              : 15438
  = residual, real object, not yet tracked (§3)             : 179
residual family distribution: Counter({'F2': 134, 'F0': 36, 'F4': 7, 'F1': 1, 'F3': 1})
```

## 6. Shape-family sampling (canonical vocabulary — `decisions.md §12a`, card 14's
`artifacts/gate-1-shape-closure/family-vocabulary.md`)

Card 14 ruled `scripts/shape_ledger.py`'s F0-F10 table canonical. `scripts/shape_ledger.py --output
artifacts/gate-1-shape-closure/ledger.json` already classifies the 15,438 currently-tracked
`class_feature` units that are also in the not-done population (15,105 of the 15,438 — the
remainder are already `done`, outside the ledger's own not-done scope) with `unclassified_count`
contribution 0 — every one lands in F0-F10:

```bash
python3 -c "
import json
from collections import Counter
d = json.load(open('docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-1-shape-closure/ledger.json'))
cf = [r for r in d['rows'] if r['kind'] == 'class_feature']
print(len(cf), Counter(r['family'] for r in cf))
"
# 15105 Counter({'F0': 10560, 'F1': 1709, 'F2': 1489, 'F4': 569, 'F5': 360, 'F6': 210,
#                 'F3': 134, 'F8': 40, 'F9': 26, 'F7': 5, 'F10': 3})
```

For the 179 not-yet-tracked units (§3) and the 2,614 excluded ones (§2), the same
`classify_formula`/`extract_formula_segment` functions apply directly to each row's raw
`DEFINE:`/`BONUS:` tokens (§5's script does this for the full 179; ten individually-inspected
samples below, spanning both categories, to show the method by hand as well as by script):

| Record | Book | Line | Disposition | Family | Why |
|---|---|---:|---|---|---|
| `Barbarian` (FavoredClass alias) | core_rulebook | 68 | (A), already tracked | **F2** | `BONUS:ABILITYPOOL\|Favored Class Bonus\|BarbarianLVL` — bare `<Class>LVL` term |
| `Barbarian` (class-selection-pool declarator) | core_rulebook | 98 | (A), already tracked (id `barbarian`, this row's sibling collision) | **F2** | `BONUS:VAR\|Barbarian_CFP_Level\|BarbarianLVL\|TYPE=Base` |
| `Damage Reduction ~ All` | core_rulebook | 62 | (B), not an object | **F0** | `CATEGORY:Internal`; no DEFINE/BONUS formula token, only a `DR:` directive |
| `Damage Reduction ~ Silver` | core_rulebook | 63 | (B), not an object | **F0** | same shape |
| `Witch Hex ~ Blight` | advanced_players_guide | 917 | (A), residual (§3) | **F0** | `CATEGORY:Ability Focus`/`TYPE:Ability Focus` only, no formula token |
| `Aberrant Bloodline` (Bloodrager) | advanced_class_guide | 566 | (A), already tracked | **F2** | `BONUS:VAR\|…\|BloodragerBloodlineLVL` (highest-priority segment among 15 BONUS tokens) |
| `Exotic Heritage` (`.MOD` orphan rescue) | ultimate_wilderness | 1401 | (A), tracked, outside the 18,231 (§4) | **F0** | no DEFINE/BONUS token on the `.MOD` row itself |
| `Domain Power ~ Touch of Good` (instance 1) | core_rulebook | 713 | (A), already tracked | **F4** | `BONUS:VAR\|TouchofGoodTimes\|DomainGoodTimes` — bare identifier reference outranks the sibling `DEFINE:TouchofGoodTimes\|0` (F1) |
| `Domain Power ~ Touch of Good` (instance 2, byte-identical DEFINE/BONUS content, differs only in `TYPE:`) | core_rulebook | 3220 | ambiguous — see note | **F4** | same formula content as instance 1 |
| 179-row script sample (§5) | — | — | (A), residual | **F2**×134 / **F0**×36 / **F4**×7 / **F1**×1 / **F3**×1 | machine-classified, full population |

**Note on the two "Touch of Good" instances:** unlike `Barbarian` (two rows with *different*
DEFINE/BONUS content, genuinely different mechanics wearing the same display name), these two rows
have byte-identical `DEFINE:`/`BONUS:` content and differ only in a `TYPE:` facet
(`...DomainPower.Good Domain` vs `...DomainPower`) — this looks like the same domain power declared
twice in the corpus rather than two distinct objects. This is exactly the "some real, some not"
split `decisions.md §1a`/§12b warn against rounding away: **not all 179+2,614 excluded/residual
rows are the same shape**, and a handful of the currently-tracked 15,438 (not counted in this
memo's headline totals, since they are inside the 15,438 "already tracked" bucket, not part of the
179 or 2,614) may themselves be exact-content duplicates worth a follow-up pass. Flagged, not
silently absorbed into either bucket above.

## 7. Summary for `progress.md`

- Leading `.MOD`/`.COPY=` hypothesis (`decisions.md §12b`): **tested and refuted** —
  `census_independent.py` already excludes `.MOD` rows before counting; `.COPY=` rows in this
  bucket number zero.
- Real cause, in two parts: (1) **2,614** `CATEGORY:Internal` rows the walker counts as
  `class_feature` but should file under `ability_category:Internal` like it already does for other
  `_abilities_*.lst` files — **NOT an object**, proven by class. (2) **179** real `class_feature`
  records across 11 books that `docs/work-inventory.json` does not currently enumerate — **IS an
  object**, needs to be added to the existing `class_feature` kind.
- `18,231 = 15,438 (agree) + 2,614 (not-an-object) + 179 (should-be-added)`, verified exactly by the
  script in §5.
- All 179 residual units classify cleanly into the canonical F0-F10 vocabulary (F2 134 / F0 36 / F4
  7 / F1 1 / F3 1) — no new family, no unclassified residue.
- One `.MOD`-orphan-rescue unit (`ultimate_wilderness:class_feature:exotic_heritage`) sits outside
  the 18,231 population entirely and needs no action.
- Flagged for the integration cycle, not actioned here (write scope excludes
  `docs/work-inventory.json`, `scripts/census_independent.py`, `scripts/shape_ledger.py`): (1) add
  the 179-row list to the inventory's `class_feature` enumeration; (2) apply the existing
  `CATEGORY:Internal` row-check to `census_independent.py`'s `_abilities_class.lst` branch so the
  2,614 file under `ability_category:Internal` instead of `class_feature`; (3) spot-check whether
  any of the 15,438 "already agree" units are byte-identical corpus duplicates like `Touch of Good`
  (§6 note) — not quantified here, named as open follow-up.
