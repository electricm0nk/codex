# SD-29 — Corpus shape, all 37 in-scope books (Epic 2, cycle-0)

**Derived 2026-08-10 by `sd29-e2-prelaunch` (card `epic-2-prelaunch`, cycle `SD29-E2-F1-001`).**
One pass over the whole in-scope corpus, not per book. Every figure below is re-derived by the
command printed beside it; nothing here is transcribed from `scope-draft.md`, `decisions.md`,
`corpus-work-channels.md`, or a prior cycle's `progress.md`.

Regenerate the whole of this document's inputs with:

```bash
cargo run --locked --bin v06_work_inventory                 # writes docs/work-inventory.json
cargo run --locked --bin v06_corpus_trap_report -- <book>   # once per book dir, 37 times
```

`docs/work-inventory.json` used here: `generated_at: 2026-08-10T23:59:04Z` (regenerated
`2026-08-11T00:18:38Z` on the closing pass — see §8), `schema_version: 1`,
`corpus_root: /home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game`.

## 1. Denominator

```bash
python3 - <<'PY'
import json
U = json.load(open('docs/work-inventory.json'))['units']
inc = [u for u in U if u['book'] != 'beginner_box']
P = {'grounded', 'text-complete'}
print(len({u['book'] for u in inc}), 'books', len(inc), 'units,',
      sum(1 for u in inc if u.get('status') in P), 'proven')
PY
# -> 37 books 38517 units, 2253 proven
```

This reproduces `corpus-work-channels.md §10.2` exactly (37 / 38,517 / 2,253). The in-scope set is
**the whole corpus minus `beginner_box`** (19 units, all `equipment`, all `not-started`).
`core_essentials` is in the 37 and contributes 1,610 units / 46 proven.

**`proven` is a predicate, not a rank:** `status in {grounded, text-complete}`. Under the wider
predicate `{grounded, ingested-magnitude, text-complete, deferred-with-reason}` the same 37 books
read **8,450**. A lane citing "proven" must say which predicate it means.

## 2. Per-book shape

Columns: `scope` is the *inventory generator's* label, not SD-29 scope (see §4.1). `proven` uses the
§1 predicate. `files enum` / `files NOT enum` are `files_enumerated` / `len(files_not_enumerated)`.
`recon rows` is `len(reconciliation)`.

| book | inventory `scope` | units | proven | files enum | files NOT enum | trap-report files | trap-report hits | recon rows | monster | monster_ability |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `advanced_class_guide` | in_scope | 3136 | 182 | 11 | 18 | 25 | 10664 | 8 | 0 | 106 |
| `advanced_players_guide` | in_scope | 3234 | 180 | 18 | 15 | 25 | 10949 | 8 | 0 | 0 |
| `advanced_race_guide` | in_scope | 1546 | 161 | 21 | 11 | 25 | 6550 | 9 | 0 | 1 |
| `adventurers_guide` | future_state | 974 | 0 | 10 | 9 | 19 | 2066 | 0 | 0 | 0 |
| `bestiary` | in_scope | 951 | 46 | 16 | 17 | 26 | 1785 | 8 | 330 | 523 |
| `bestiary_2` | future_state | 974 | 0 | 12 | 11 | 19 | 1597 | 0 | 316 | 466 |
| `bestiary_3` | future_state | 1194 | 0 | 12 | 10 | 20 | 2334 | 0 | 261 | 40 |
| `bestiary_4` | future_state | 1218 | 0 | 17 | 13 | 30 | 2306 | 0 | 220 | 768 |
| `bestiary_5` | future_state | 165 | 0 | 10 | 7 | 17 | 426 | 0 | 0 | 39 |
| `bestiary_6` | future_state | 59 | 0 | 8 | 4 | 12 | 158 | 0 | 0 | 13 |
| `bonus_bestiary` | future_state | 34 | 0 | 3 | 1 | 4 | 13 | 0 | 14 | 17 |
| `book_of_the_damned_volume_1` | future_state | 90 | 0 | 9 | 4 | 12 | 195 | 0 | 5 | 36 |
| `book_of_the_damned_volume_2` | future_state | 254 | 0 | 8 | 6 | 14 | 614 | 0 | 4 | 17 |
| `core_essentials` | shared_library | 1610 | 46 | 119 | 373 | 40 | 2769 | 0 | 0 | 380 |
| `core_rulebook` | in_scope | 5131 | 628 | 16 | 23 | 35 | 12822 | 9 | 0 | 0 |
| `horror_adventures` | future_state | 856 | 0 | 16 | 10 | 26 | 2755 | 0 | 3 | 71 |
| `inner_sea_bestiary` | future_state | 234 | 0 | 2 | 5 | 7 | 473 | 0 | 40 | 190 |
| `inner_sea_combat` | future_state | 398 | 0 | 9 | 5 | 10 | 1029 | 0 | 0 | 0 |
| `inner_sea_faiths` | future_state | 3 | 0 | 1 | 3 | 4 | 497 | 0 | 0 | 0 |
| `inner_sea_gods` | future_state | 477 | 0 | 7 | 11 | 18 | 2582 | 0 | 39 | 161 |
| `inner_sea_intrigue` | future_state | 256 | 0 | 11 | 5 | 16 | 469 | 0 | 0 | 0 |
| `inner_sea_magic` | future_state | 335 | 0 | 7 | 5 | 12 | 956 | 0 | 0 | 0 |
| `inner_sea_races` | future_state | 315 | 0 | 6 | 4 | 10 | 1269 | 0 | 0 | 0 |
| `inner_sea_taverns` | future_state | 20 | 0 | 2 | 0 | 2 | 26 | 0 | 0 | 0 |
| `inner_sea_temples` | future_state | 64 | 0 | 4 | 1 | 5 | 16 | 0 | 0 | 0 |
| `inner_sea_world_guide` | future_state | 376 | 0 | 23 | 21 | 23 | 601 | 0 | 14 | 30 |
| `monster_codex` | future_state | 207 | 0 | 13 | 8 | 21 | 271 | 0 | 2 | 3 |
| `mythic_adventures` | future_state | 969 | 0 | 5 | 6 | 11 | 3145 | 0 | 0 | 21 |
| `occult_adventures` | future_state | 1729 | 0 | 18 | 7 | 25 | 5466 | 0 | 1 | 3 |
| `pathfinder_unchained` | in_scope | 826 | 29 | 6 | 10 | 16 | 3799 | 5 | 0 | 72 |
| `ultimate_campaign` | in_scope | 23 | 21 | 1 | 10 | 9 | 780 | 1 | 0 | 0 |
| `ultimate_combat` | in_scope | 2056 | 236 | 34 | 12 | 42 | 6349 | 8 | 0 | 0 |
| `ultimate_equipment` | in_scope | 1615 | 185 | 9 | 7 | 12 | 2213 | 3 | 0 | 0 |
| `ultimate_intrigue` | in_scope | 1102 | 104 | 14 | 7 | 21 | 2778 | 8 | 0 | 6 |
| `ultimate_magic` | in_scope | 1729 | 117 | 16 | 14 | 25 | 6318 | 8 | 0 | 13 |
| `ultimate_psionics` | in_scope | 2495 | 192 | 11 | 21 | 32 | 5140 | 9 | 21 | 79 |
| `ultimate_wilderness` | in_scope | 1862 | 126 | 23 | 12 | 35 | 4493 | 10 | 0 | 52 |
| **37 books** | — | **38517** | **2253** | 528 | 706 | 705 | 106673 | 94 | 1270 | 3107 |

## 3. Per-kind remaining, corpus-wide (the lane denominators)

`remaining = not-started + not-ingested + unknown`, summed over the 37 books:

```bash
python3 - <<'PY'
import json
from collections import Counter
d = json.load(open('docs/work-inventory.json'))
byst = {}
for b in d['books']:
    if b['id'] == 'beginner_box': continue
    for k, v in b['kinds'].items():
        for s, n in v['by_status'].items():
            byst.setdefault(k, Counter())[s] += n
for k in sorted(byst):
    c = byst[k]
    print(f"{k:20}{c['not-started'] + c['not-ingested'] + c['unknown']:7}   {dict(c)}")
PY
```

| kind | units | remaining | kanban card figure | verdict |
|---|---:|---:|---:|---|
| `class` | 185 | 158 | 158 | ✅ |
| `class_feature` | 15,472 | 15,329 | — (SD-30 territory) | n/a |
| `companion` | 1,683 | 1,683 | 1,683 | ✅ |
| `equipment` | 6,208 | **1,144** | 1,163 | ❌ **off by 19** |
| `equipment_modifier` | 1,580 | 812 | 812 | ✅ |
| `feat` | 2,610 | **1,348** | 1,350 | ⚠️ predicate |
| `monster` | 1,270 | 1,224 | 1,224 | ✅ |
| `monster_ability` | 3,107 | 3,107 | 3,107 | ✅ |
| `race` | 103 | 96 | 96 | ✅ |
| `race_trait` | 3,456 | 3,412 | 3,412 | ✅ |
| `spell` | 2,843 | ~~1,754~~ **1,561** | 1,754 | ❌ **inflated by 193** |

Two corrections were fixed in `kanban.md` by the Epic 2 cycle that authored this table; a third
(`spell`) was found later, by the lane that consumed the figure, and is listed first because it is
the one this table itself got wrong rather than the card:

- **`equipment` 1,163 → 1,144.** The 1,163 figure counted `beginner_box`'s 19 `equipment` units,
  which are the *excluded* book. Verified: `beginner_box` carries exactly `{'equipment': (19,
  {'not-started': 19})}` and nothing else, and 1,163 − 1,144 = 19.
- **`spell` 1,754 → 1,561** (added 2026-08-11 by cycle `SD29-E4-F1-001`, the `epic-4-proven-spell`
  lane). Not an arithmetic error and not a predicate difference — the inventory itself was wrong.
  `v06_work_inventory::gather_engine_facts` built its `spell_levels` map from three hand-written
  `.insert()` calls (`core_rulebook`/`advanced_players_guide`/`advanced_class_guide`) while the
  shipped desktop `spell_catalog::build_spell_catalog` chained **five** books, adding ARG and UI.
  So 192 spells that were already ingested *and already served on screen* were counted as remaining
  work. This is Decision 36's two-lists-one-fact pattern, at the same place SD-28-E15 had already
  fixed it for `equipment` — the equipment map sits four lines below the spell map in the same
  function, carrying a doc comment describing this exact defect. Both consumers now read one
  registry (`spell_resolver::spell_catalog_rows()`). Re-derived by regenerating
  `docs/work-inventory.json` and diffing per book/kind against `git show HEAD:docs/work-inventory.json`:
  `advanced_race_guide` spell `{'not-ingested': 93}` → `{'ingested-magnitude': 92, 'not-ingested': 1}`,
  `ultimate_intrigue` spell `{'not-ingested': 101}` → `{'ingested-magnitude': 101}`, and **no other
  book/kind pair moved**. The one surviving ARG unit is `Fins to Feet (self only)`
  (`arg_spells.lst:230`), a `.COPY=` delta row whose `CLASSES:.CLEARALL` leaves the corpus stating
  no level for it — recorded, not invented.
- **`feat` 1,350 → 1,348 + 2 `deferred-with-reason`.** Not an error in the total, a predicate
  difference: `feat` is the only kind in the 37 with `deferred-with-reason` units outside
  `class_feature` (2 of them). Stated explicitly rather than silently reconciled.

## 4. Findings that change how the lanes must be dispatched

### 4.1 `reconciliation` is empty for 24 of the 37 books

The inventory computes `reconciliation` (corpus-units vs. engine-records delta, per kind) **only for
books its own `scope` field labels `in_scope`** — 13 books, 94 reconciliation rows total. The other
24 (`future_state` ×23, `shared_library` ×1) carry `"reconciliation": []`.

That label is the *generator's* scope, not SD-29's. Every one of the 24 is in SD-29's 37. So for
`bestiary_2..6`, `bonus_bestiary`, `monster_codex`, all eleven `inner_sea_*`, `occult_adventures`,
`mythic_adventures`, `horror_adventures`, `adventurers_guide`, both `book_of_the_damned_*`, and
`core_essentials`, **a lane cannot read a corpus-vs-engine delta out of the inventory** — it has to
derive its own. Lanes touching those books must not treat a missing `reconciliation` as "no delta".

### 4.2 `inner_sea_bestiary` is in scope and is not a stub

`loop-instruction.md`'s "Corpus shape notes" (re-derived 2026-08-02, i.e. *before* the corpus-wide
re-scope of 2026-08-10) lists `inner_sea_bestiary/` as an out-of-scope adjacent and describes it as
a "pcc+jpg stub". Both halves are now wrong:

- **Scope:** `corpus-work-channels.md §10.2` (2026-08-10) — the basis of `decisions.md §38`'s
  corpus-wide re-scope — excludes exactly one book, `beginner_box`. `inner_sea_bestiary` is one of
  the 37.
- **Stub:** `ls .../campaign_setting/inner_sea_bestiary/` → 7 `.lst` files plus `_pfs/`; the
  inventory reports **234 units, 40 of them `monster`**, and the trap report reports 7 files /
  473 hits. Not a stub.

Same for `inner_sea_world_guide` (376 units, 14 monsters), also listed as an out-of-scope adjacent.
Corrected in `loop-instruction.md` by this cycle; `correction` retro event emitted.

### 4.3 Monster-bearing books: 14, not "the bestiaries"

```bash
python3 -c "import json;d=json.load(open('docs/work-inventory.json'));[print(f\"{b['id']:30}{b['kinds'].get('monster',{}).get('units',0):6}\") for b in d['books'] if b['kinds'].get('monster',{}).get('units',0)]"
```

`bestiary` 330, `bestiary_2` 316, `bestiary_3` 261, `bestiary_4` 220, `inner_sea_bestiary` 40,
`inner_sea_gods` 39, `ultimate_psionics` 21, `bonus_bestiary` 14, `inner_sea_world_guide` 14,
`book_of_the_damned_volume_1` 5, `book_of_the_damned_volume_2` 4, `horror_adventures` 3,
`monster_codex` 2, `occult_adventures` 1 — total **1,270**.

`bestiary_5` (165 units) and `bestiary_6` (59) carry **zero** `monster` units, confirming the
shape note. Epic 5's extend batch must be driven off this list, not off directory names: seven of
the fourteen monster-bearing books are not bestiaries, and two books named "bestiary" hold no
monsters.

`monster_ability` is distributed differently again: `core_essentials` alone holds 380 and
`inner_sea_bestiary` 190, while `bestiary_3` holds only 40 against its 261 monsters.

## 5. Corpus-shape hazards, re-derived (not transcribed)

Every hazard in `loop-instruction.md` §"Corpus shape notes" was re-run against the tree on
2026-08-10. Results, with the command:

| hazard | command | result |
|---|---|---|
| space in a pcc filename | `find . -name '*.pcc' \| grep ' '` | **holds** — exactly one: `bestiary_6/_bestiary_6 _for_players.pcc` |
| `.pcc` leading-underscore split | `ls bestiary*/*.pcc` | **holds** — B1/B2/B3 main pccs bare, B4/B5/B6 underscored; glob `*.pcc` |
| `SOURCESHORT` not unique | `grep -rl 'SOURCESHORT:B1' --include='*.pcc' .` | **holds, sharpened** — 3 files, but the third is `bestiary/_pfs/_.pcc`, a *subdirectory* pcc; a flat `<book>/*.pcc` glob finds only 2. 12 books carry a `_pfs/` subtree. |
| `*_races_pc.lst` are `.MOD` overlays | `awk '!/^#/&&NF>0&&!/^SOURCELONG/' bestiary_2/b2_races_pc.lst \| wc -l` and `\| grep -c '\.MOD'` | **holds** — 7 lines, 7 of them `.MOD` |
| B2 = 322 = 314 + 8 `.COPY=` | `awk '!/^#/ && !/^SOURCELONG/ && NF>0' bestiary_2/b2_races.lst \| wc -l` → 322; `\| grep -c '\.COPY='` → 8; `grep -c '\.MOD'` → 0 | **holds** |
| conditional cross-book support files | `grep -rn '_ma.lst\|_oa.lst' --include='*.pcc' bestiary_4 bestiary_5` | **holds, sharpened** — the `PRECAMPAIGN:1,Mythic Adventures` / `Occult Adventures` gate is on the **pcc load line**, not inside the `.lst`. Grepping the `.lst` files for `PRECAMPAIGN` returns **0**; a lane that checks the file for its own gate will conclude, wrongly, that it is ungated. 2 `_ma` + 4 `_oa` files. |
| seven zero-byte `.lst` in B1–B4 | `find bestiary bestiary_2 bestiary_3 bestiary_4 -name '*.lst' -size 0 \| wc -l` | **holds** — 7 |

## 6. Trap-report, corpus-wide (cycle-0 record)

37 book directories, `v06_corpus_trap_report -- <book> --json`, all exit **0**. 705 files,
**106,673** trap hits:

| trap | hits |
|---|---:|
| `key-differs-from-name` | 30,162 |
| `namespaced-key` | 28,207 |
| `mod-record` | 21,669 |
| `shared-name-distinct-records` | 8,248 |
| `define-zero-value-elsewhere` | 5,922 |
| `governing-token-hidden-by-filter` | 5,844 |
| `copy-record` | 2,832 |
| `archetype-scoped` | 2,171 |
| `disabled-line` | 1,378 |
| `token-dense-record` | 240 |

Heaviest books by trap hits: `core_rulebook` 12,822, `advanced_players_guide` 10,949,
`advanced_class_guide` 10,664, `advanced_race_guide` 6,550, `ultimate_combat` 6,349,
`ultimate_magic` 6,318. Lightest: `bonus_bestiary` 13 (4 files) — which is why it is the correct
Epic 5 pilot.

`cargo run --locked --bin v06_corpus_trap_report -- --audit` → exit **0**, "No defects: every
ingested record's citation agrees with the line it names" (259 mod-record traps, 0 defects).

## 7. `files_not_enumerated`

706 files across the 37 are enumerated by no kind, against 528 that are. `core_essentials` alone
accounts for **373** of the 706 — more than half — against 119 enumerated. This is expected for a
shared-library book (ability categories, datacontrols, globalvars, skills, languages, templates,
kits, profs), but it means any lane quoting a `core_essentials` coverage ratio must state whether
its denominator is enumerated files or all files.

## 8. Independent re-derivation on the closing pass (2026-08-11)

Sections 1-7 were derived on the opening pass, which was cut off before its gate finished. The
closing pass did **not** trust them wholesale (step 1b: re-derive at the point of use). It re-ran
the derivation against a freshly regenerated `docs/work-inventory.json`
(`generated_at: 2026-08-11T00:18:38Z`) and re-ran a sample of the §5 hazard commands against the
live `pcgen` tree. Results:

| spot-check | command | opening pass | closing pass |
|---|---|---|---|
| denominator | §1 python block | 37 / 38517 / 2253 | **37 / 38517 / 2253** ✅ |
| per-kind remaining, all 10 lane kinds | §3 python block | see §3 table | **identical, all 10** ✅ |
| `feat` `deferred-with-reason` | same block | 2 | **2** ✅ |
| monster-bearing books | §4.3 command | 14 books / 1270 | **14 / 1270** ✅ |
| books with empty `reconciliation` | `sum(1 for b in books if b['id']!='beginner_box' and not b['reconciliation'])` | 24 | **24** ✅ |
| space in a pcc filename | `find . -name '*.pcc' \| grep ' '` | 1 (`bestiary_6/_bestiary_6 _for_players.pcc`) | **1, same file** ✅ |
| `SOURCESHORT:B1` not unique | `grep -rl 'SOURCESHORT:B1' --include='*.pcc' .` | 3, third in `bestiary/_pfs/` | **3, same** ✅ |
| `_pfs/` subtrees | `find . -maxdepth 3 -type d -name '_pfs' \| wc -l` | 12 | **12** ✅ |
| B2 race count / `.COPY=` | `awk '!/^#/ && !/^SOURCELONG/ && NF>0' bestiary_2/b2_races.lst \| wc -l` / `grep -c '\.COPY='` | 322 / 8 | **322 / 8** ✅ |
| zero-byte `.lst` in B1-B4 | `find bestiary bestiary_2 bestiary_3 bestiary_4 -name '*.lst' -size 0 \| wc -l` | 7 | **7** ✅ |
| `PRECAMPAIGN` inside the gated `.lst` files | `grep -c PRECAMPAIGN bestiary_5/support/*_oa.lst` | 0 | **0 for all 4** ✅ |
| gated support files, **distinct** | `grep -rho '[a-z0-9_/]*_\(ma\|oa\)\.lst' --include='*.pcc' bestiary_4 bestiary_5 \| sort -u` | 2 `_ma` + 4 `_oa` | **6 distinct** ✅ (the raw `grep -rn` returns **10 lines** — the pcc load line and a later reference both match; count distinct files, not lines) |

**Zero disagreements.** No Hard-stop "figure derived this cycle disagrees with a figure recorded in
this package" case arose. The one sharpening added by the closing pass is the last row: the §5
command as written (`grep -rn`) yields 10 lines for 6 files, so a lane transcribing "10" as a file
count would be wrong.

**DoD item 4 (idempotency) is satisfied by this pass itself:** the opening pass wrote the inventory
at `23:59:04Z`, the closing pass regenerated it at `00:18:38Z`, and every figure in §1-§4 and §7
reproduced byte-identically — a second run changed only `generated_at`.
