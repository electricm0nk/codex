# AT-35-E6-003 cycle 5 — what actually blocks the compiled-table catalogs, measured

Cycle 2 named the nine compiled-table-rostered `apps/desktop` files' blocker as **converted-row
coverage** and sequenced all nine behind it. Cycles 3 and 4 closed the two *corpus*-rostered
files and left that diagnosis standing. This cycle built the join, pointed it at two of the nine,
and measured the result against the live package.

**Row coverage is no longer the binding blocker.** It is fixed, and the join this cycle built
closes the part of it that remained. The binding blocker is one level down: **the converter
writes no descriptive prose at all for a record whose description carries an argument the
formula side refuses.** `correction 1789106039024-at-35-e6-003-df19bf`.

One file (`monster_catalog.rs`) cleared the bar and shipped this cycle. The two that did not are
measured here, row for row, so the next cycle scopes the converter work rather than
rediscovering it.

## 1. The join — built, shipped, and used by one catalog

`apps/desktop/src-tauri/src/converted_prose.rs`. A catalog row names a book and a record key; the
converted package addresses a rule by `"<book>:<kind>:<slug>"`. The module is that join and
nothing else — no token, no formula string, no argument tail, no ingest-format vocabulary.

| step | what it matches | why it exists |
|---|---|---|
| 1. book-exact | `<book dir>:<kind>:<slug of key>` | the honest join: same book, same record |
| 1b. **source row** | the compiled key's `<file>_lst_<line>` tail against the converted rule's `provenance.closure_rows` | a corpus row with no name reaches the tables as `Codex-Named Unit (spell_..._lst_9)` and the converter resolves its real name (`Gorum's Armor`) — these can never join by name in either direction |
| 2. by name, package-wide | `SheetRulePackage::find` | a compiled table's book is sometimes a *printing*: `ultimate_magic_wordsofpower` has no converted directory at all |
| 3. printed qualifier dropped | one trailing parenthesised group, retried through 1 and 2 | the tables carry one row per printed variant (`Nondetection (self only)`), the converter one record per record |

Step 1b is new knowledge and is the reason the spell measurement below is 13 rows better than
cycle 2's. A suffix two rules share resolves to **neither**, by construction.

## 2. `monster_catalog.rs` — swapped, green, shipped

`serve_ability_description` now reads the converted `monster_ability` record instead of parsing
the chassis's stored description string at run time.

| | served abilities | with a description |
|---|---:|---:|
| before (run-time render) | 3,509 | 3,455 |
| after (converted package) | 3,509 | **3,456** |

Net **+1**, and the 19 on-screen tests, the cross-catalog syntax sweeps and the reach gate all
stayed green — `576 passed; 0 failed`. The floor is now a standing ratchet in the crate
(`monster_catalog::converted_ability_prose_population`), corpus-wide rather than a fixture
(`decisions.md §4`).

Re-derive:

```bash
cd apps/desktop/src-tauri
cargo test --locked -j 6 converted_ability_prose_population -- --nocapture
# -> monster abilities served=3509 with a description=3456
```

## 3. `spell_catalog.rs` — swapped, measured, reverted

2,481 catalog rows; 2,436 carried a description on the old path.

| after the swap | rows | what they are |
|---|---:|---|
| still described | 2,410 | |
| **gained** a description | 25 | rows the old path served nothing for |
| lost, and correctly so | 48 | the compiled table's stored string was the literal `[redacted PI]`, reaching the Spell Catalog screen |
| lost, and correctly so | 1 | `ACG :: Discern Next of Kin` — the compiled table ships text the **corpus record declares product identity** (`pi_field: description`) |
| **lost, real** | 2 | `APG :: Wall of Thorms`, `MYTHIC :: Elemental Body IIIMOD` |

The 48 redaction placeholders are the headline correction of this cycle: the first measurement
read "96 descriptions lost" and 48 of them were a redaction marker on a player's screen
(`correction 1789106039171-at-35-e6-003-ec568c`). **A raw loss count is not a loss count until
every lost row's old text is read.**

The 2 real losses are corpus records the converted package holds under no id and no name:

```bash
find data/sheet_rules -name 'wall_of_thorms.json' -o -name 'elemental_body_iiimod.json'   # -> (nothing)
find data/corpus     -name 'wall_of_thorms.json' -o -name 'elemental_body_iiimod.json'
# -> data/corpus/advanced_players_guide/spell/wall_of_thorms.json      (in_scope, full)
# -> data/corpus/mythic_adventures/spell/elemental_body_iiimod.json    (in_scope, full)
```

Neither is in the converter's recorded refusal set — `data/sheet_rules/_refused.json` holds 142
entries and every one is `no_corpus_record`, so these two are converted-and-then-absent, which
nothing currently reports.

## 4. `feat_catalog.rs` — swapped, measured, reverted

2,227 catalog rows; 2,161 carried a description on the old path. After the swap: 2,124 described,
1 gained, 38 lost — 8 of them the same `[redacted PI]` marker, and **30 real**.

| book | rows | why the converted record states no prose |
|---|---:|---|
| `Ce` (Core Essentials) | 11 | `data/sheet_rules/` has **no `core_essentials/` directory** — the book is compiled but not converted |
| `Acg` | 10 | the converter wrote the record with `prose: []` |
| `Pu` | 9 | same |

The ACG group is the diagnosis. `advanced_class_guide:feat:befuddling_strike` exists, carries a
full `applies` gate, and carries **no prose key at all**, while its corpus row states a complete
paragraph:

```bash
python3 -c "import json;d=json.load(open('data/sheet_rules/advanced_class_guide/feat/befuddling_strike.json'));print('prose' in d[0])"
# -> False
sed -n '20p' "$PCGEN_CORPUS_ROOT/pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_feats.lst"
# -> ...DESC:Befuddling Strike forces a foe ... DC %1 Fortitude saving throw ...|CL/2+10+WIS|BefuddlingStrikeTimes
```

`convert_desc_like` converts each `|`-argument through `convert_formula`; `CL` on a record with
no owning class is `Err("FORMULA:CL-no-owner")` (`src/pcgen_import/sheet_rule/formula.rs:501-506`),
the `?` propagates, and `convert_token`'s caller refuses the **whole DESC row**. The description
is not degraded — it is dropped.

Under `decisions.md §1` form 3 and `§15` R2's ruled shape, a term nobody can settle is printed as
**words**, not refused. This is the same correction cycle 4 made for `PREVARLT`'s equipped-item
census, one level over: the gate side learned to print its words, the prose-argument side has
not.

## 5. Three converted records leak a bare `%` into player-facing prose

Surfaced by the catalogs' own cross-catalog syntax sweeps once they read the package. Invisible
to the standing `data/sheet_rules/` token grep, which reads `0` and correctly so — none of these
strings is in that pattern set.

| record | the leak |
|---|---|
| `core_rulebook:spell:teleport` | `d %` — a percentile-dice notation written with an inserted space (cycle 2's `correction 1789093674392`) |
| `inner_sea_world_guide:spell:ancestral_memory` | `(70+CASTERLEVEL)%` — an unconverted variable name (cycle 2's `correction 1789093674266`) |
| `feat Prophetic Visionary` | `a rules variable%` and `+a rules variable` — a slot rendered as words where the sentence needed a number |

Cycle 2 recorded the first two. Neither has been fixed in the three cycles since, and the third
is new.

## 6. What the next cycle needs, named as mechanisms

1. **`FORMULA:CL-no-owner` prints words rather than refusing the row.** 30 feat rows measured
   here; the population across all kinds is larger and unmeasured — 487 of 2,883 converted `feat`
   rules and 634 of 3,102 `spell` rules carry no prose at all
   (`python3 - <<'PY'` walk of `data/sheet_rules/`, counting rules with a falsy `prose`).
2. **The three `%` leaks in §5.**
3. **`core_essentials` is compiled but not converted** — 11 feat rows, and every other family
   that book carries.
4. **`advanced_players_guide:spell:wall_of_thorms` and `mythic_adventures:spell:elemental_body_iiimod`
   are converted-and-then-absent**, reported by nothing.
5. Only then the two catalogs, each with the before/after census in §3 and §4 re-run, so a loss
   cannot ship silently.
6. Still untouched and **not** blocked on the above: `race_trait_picker.rs` (33),
   `intelligent_item_catalog.rs` (28), `companion_catalog.rs` (52),
   `reference_library_catalog.rs` (15) and `equipment_catalog.rs` (25), which need
   `SheetRule.applies`/`grants` rather than prose, and
   `apps/desktop/src/characterHub/raceCreationCoverage.test.ts` (21), which cycle 2 measured as
   blocked on a `race_trait` `target` gap.
