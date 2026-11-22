# F6c receipt: Level Up names the engine's refusals, requirements print labels, offer lines match their record, the chassis cites the newest printing

Stage F6, step F6c (`polish.json` items F6-4, F6-5, F6-6). Branch `sd36/epic-f6-desktop-polish`,
on F6b `15927a35c5`. This step includes a converter step (`data/sheet_rules/**` changed under the
structural-diff protocol, §5). Out of scope here: P1-P4, #28.

## 1. (a) Level Up shows the engine's refusal on the option

**Rule.** An option's blocker is the multiclass gate's own answer, asked before the level is taken:
`pilot_compute::level_up_mix_blocker(leveled, class_id)`. `leveled` is the character after
`apply_level_up`, including its seeds, which is exactly what Accept builds. The function then runs
`multiclass_member`, the same per-class gate the sheet's mix chassis runs. The blocker carries:

- the gate's diagnostic id;
- one plain reading per diagnostic id (`mix_blocker_summary`; there is no per-class text);
- the fold's full message.

`list_level_up_class_options` puts it on every option in all three groups (`blocker` on
`LevelUpClassOptionDto` and `PrestigeClassOptionDto`). In the dialog:

- a blocked option is shown and disabled, with its reason after the label;
- a "Cannot be taken (N)" block lists each blocked option with its reason;
- the default selection skips blocked options;
- Accept is disabled unless the selected option can be taken (`canAcceptLevelUp`).

An unmet prestige requirement is still a printed note and never a block (§9.2).

**Population.** Human Fighter 6, the `f4c-level-up-fighter6-wire.json` build. There are **133**
options: advance 1, add a base class 58, add a prestige class 74. Each one was taken for real by
running `apply_level_up` and then `resolve_unified_pilot_snapshot`, the Accept path. Results:

| Outcome | Options |
|---|---|
| Computed | **123** of 133 |
| Refused, with the same blocker shown on the option | **10** of 133 |
| Refused by anything other than the mix gate | **0** of 133 |
| Blocker shown, but the engine computes it | **0** of 133 |

The 10 refused options:

- **6 FS-15 prestige classes**, `multiclass.save_shape.unrecognized`, shown as "save progression in
  the source data matches no PF1 form": Evangelist, Exalted, Mammoth Rider, Pure Legion Enforcer,
  Sentinel, Ulfen Guard. These are the same six named in FS-15.
- **4 Pathfinder Unchained base classes**, `multiclass.save_shape.unknown`, shown as "no source
  states this class's save progressions": Unchained Barbarian, Monk, Rogue and Summoner. This is
  new: F4c offered them as base classes to add, and Accept was refused by the engine. The mix gate
  reads a class's saves from the CRB class table or from a converted chassis record. These four
  have neither (`a_class_with_no_table_and_no_record_is_named_unknown`). **Remainder, by
  mechanism:** their saves are in the bespoke Pathfinder Unchained module
  (`rules_tables::pathfinder_unchained::class_chassis`). The mix gate does not read that module
  yet, the way F6b's hit-die rule reads the bespoke tier first. Until it does, these four can be
  taken alone but not added into a mix, and Level Up now says so.

Command: `cargo test --locked -j 8 --manifest-path apps/desktop/src-tauri/Cargo.toml level_up_options_name_the_engines_mix_refusal_before_accept -- --test-threads=8 --nocapture`.

**Cost.** Building the options warm, which is what the dialog pays, takes **1.1 s**. Cold it takes
80.7 s, because the census sweep that warms the roster is included.

## 2. (b) Entry requirements print labels, never ids

**Rule** (`level_up_option_filter::describe_gate`, the `Chosen` arm). The choice prints as the
label of the record that owns it, and the option prints as its label if it is a rule id, else as
its words: `label_or_words`, then `label_of`. Before, both went through `pretty`, which only
replaces `_` and `-`, so a choice id printed as `core rulebook:feat:weapon focus`.

**Scan** (`class_census::tests::no_prestige_requirement_line_prints_a_raw_id`). It covers every
top-level printed term of all **74** census prestige gates, **286** lines in total. A raw id is a
`:` with no whitespace on either side. Before: **12 of 286** lines carried one (Aldori Swordlord,
Arcane Archer, Argent Dramaturge, Death Slayer, Enchanting Courtesan, Loremaster, Phrenic Slayer,
Red Mantis Assassin, Sanguine Angel, Sighted Seeker, Student of War, Thrallherd). After:
**0 of 286**. Arcane Archer now prints `requires Longbow chosen for Weapon Focus`.

The census JSON's only change is the same words inside the prestige `alone_blocking_diagnostics`
messages of those 12 classes. Every count is equal to `census-f5.json`: ids 137, computed 63,
prestige swept 74, mix panel 185/185, roster 59.

## 3. (c) The shaman's "(domains) +1" line

**Oracle and book.**

- `acg_classes.lst:221`: `CLASS:Shaman ... SOURCEPAGE:p.35 ... BONUS:DOMAIN|NUMBER|1|PREABILITY:1,CATEGORY=Special Ability,TYPE.ShamanSpirit`.
- `acg_classes.lst:259-268`: ten rows `0 DOMAIN:<X> (Spirit)|PREABILITY:1,CATEGORY=Special Ability,Shaman Spirit ~ <X>`.
- `acg_domains.lst:5-14`: `<X> (Spirit)` records that carry only a `SPELLLEVEL:DOMAIN` spell list.

So the count is PCGen's model of **spirit magic**. PCGen stores each spirit's spirit-magic list as
a DOMAIN, which the class grants automatically when its spirit is held. The Advanced Class Guide
shaman (p.35, the class line's own `SOURCEPAGE`) bonds with a spirit and casts spirit-magic spells
from it. It takes no domain. The sheet printed `Shaman (domains) +1` and, in the desktop's Rules
and features section, a `Domains` heading over `Life (Spirit)`. The first live smoke run
(2026-09-27) found that second line. Neither line is in the book.

**Rule** (converter, `pool_link::withhold_class_granted_domain_counts`; one rule, no class named):

- A `BONUS:DOMAIN|NUMBER` count on a **class** record is withheld when the class grants its domains
  itself, meaning some domain record carries that class's automatic `Granter::Class` edge from its
  own `DOMAIN:` rows. The count is the oracle's slot for those grants, not a pick the player makes.
  It keeps no `offers` and does not print.
- A domain record that **only** such a class grants is the filler for that slot. It is still held,
  but it prints no line of its own. The class feature whose holding gates the grant prints the rule
  once: the `Life (Spirit)` spirit prints its spirit-magic text. The DOMAIN record has no prose.

**Scan** (`class_census::tests::no_printed_domain_count_is_filled_by_its_own_class_grants`,
`--nocapture`):

| | Before | After |
|---|---|---|
| Domain-count lines in the package (offering a domain) | 47 (47) | 47 (46) |
| Domain-count lines on a class record | 4 (cleric, paladin, shaman, daughter_of_urgathoa) | 4 |
| Classes whose own `DOMAIN:` rows grant domains | 4 (druid, ex_antipaladin, magus, shaman) | 4 |
| Printed counts filled by their own class's grants | **1** (shaman) | **0** |
| Domain records only a withheld-count class grants, printing | **10 of 10** (the shaman's spirit-magic lists) | **0 of 10** |

What stays and why:

- The cleric's and paladin's counts stay. Their classes grant no domain by class row, so the count
  is the player's pick.
- Daughter of Urgathoa's count stays for the same reason.
- The druid's count stays. It sits on the `Druid Domain` option record (Nature's Bond, CRB p.51,
  where the druid chooses a cleric domain), not on the class, and its domains (`Air`, and so on)
  still print.
- Magus and ex_antipaladin grant domains by class row but have no domain count, so nothing changes.

So no printed domain-count label disagrees with what fills it. The live sheet check is row
`sheet-f6c-shaman-no-domain-line` (§6). `class_census --sheet-dump class:shaman:5 --with-sheet-rules`
before and after is in `f6c-shaman5-lines.txt`.

## 4. (d) The chassis cites the newest printing

**Export (converter).** `reprint::stamp_class_printings` writes `provenance.printing` on every class
principal whose slug another book's class principal also states:

- `source_date`: its book's `.pcc` `SOURCEDATE:`;
- `printings`: the sorted list of those printings;
- `newest`: F3b2b's reprint resolver verdict (`newest_printing`, already computed once per
  ambiguous pair), when an ambiguous pair holds every printing and names one of them.

The population is 6 records over 3 slugs:

| Slug | Printings (SOURCEDATE) | Resolver verdict | Cited |
|---|---|---|---|
| hellknight | adventurers_guide 2017-06, inner_sea_world_guide 2011-03 | **one object**, newest `adventurers_guide:class:hellknight` (identical rows) | adventurers_guide |
| cyphermage | adventurers_guide 2017-06, inner_sea_magic 2011-07 | not proved (rows differ in non-`SOURCE` tokens, no `DESC:`) | adventurers_guide |
| red_mantis_assassin | adventurers_guide 2017-06, inner_sea_world_guide 2011-03 | not proved (same reason) | adventurers_guide |

**Runtime** (`multiclass_fold::resolve_printings`):

1. The resolver's verdict answers.
2. If there is no verdict and every printing states the same chassis, the newest by `SOURCEDATE:`
   is cited. No number depends on it.
3. Otherwise nothing answers, never a book picked by directory order.

`generic_class_chassis`'s population now resolves through the same function. Before, its
`CLASS_FAMILY_BOOKS` order meant the first listed book won. Pinned by
`the_chassis_cites_the_newest_printing_in_any_order`: all three slugs cite Adventurer's Guide in
either input order. Before F6c the reversed Cyphermage group cited `inner_sea_magic` (RED). This
closes F3p's remainder ("the converter would have to export its verdict").

## 5. Converter step (structural-diff protocol)

The baseline is a read-only `git archive` of `/home/ubuntu/workspace/repos/codex` `tranche/16` HEAD
`0a234a035a`, taken from `data/sheet_rules`.

**Delta classes** (`structural_diff_f6c_deltas.json`, from `f6c_delta_pins.py`): **17** deltas, 0
unexplained.

| Class | Pinned | What |
|---|---|---|
| `class_printing` | 6 (2 proved one object, 4 not) | `provenance.printing` added; provenance otherwise equal |
| `class_granted_domain_count` | 1 (`advanced_class_guide:class:shaman#bonus1`) | `offers` removed (it was the F4pre domain choice) and `print` true -> false |
| `class_granted_domain_record` | 10 (the `<X> (Spirit)` records) | `print` true -> false only |

**Diff tool change.** `tranche/16` HEAD already carries F3p and F4pre, so their one-sided undo
stopped working. Diffing that package against itself failed with 2,106 unexpected field deltas.
`structural_diff.py` now applies the same undo to a baseline that already carries a stage. Each
undo's own owner guard leaves a baseline that predates the stage untouched, so earlier runs read
the same as before. The self-diff of the `tranche/16` package now passes. `f6c_apply` runs first
and restores what F6c withheld, so the F4pre pin still checks the shaman's offer.

| Gate | Command | Result |
|---|---|---|
| write | `cargo run --locked --quiet -j 8 -p codex-ingest --bin sheet_rule_convert -- --write` | exit 0; records 49,450, converted 49,450, refused 0; rules 73,363; var tables 6,211 (unmoved) |
| pins | `python3 .../scripts/f6c_delta_pins.py <tranche/16 pkg> data/sheet_rules .../structural_diff_f6c_deltas.json` | 6 + 1 + 10 pinned; 0 unexplained |
| structural diff | `python3 .../scripts/structural_diff.py data/sheet_rules --baseline <tranche/16 pkg>` | **`verdict=PASS`** (`f6c-structural-diff.txt`): F6c 6/6 + 1/1 + 10/10 undone; unexpected field deltas 0; removed edges 0; removed grants 0; records 49,450 -> 49,450 |
| planted mutations | `python3 .../scripts/f6c_planted_mutations.py <scratch> . <tranche/16 pkg>` | **10 of 10 FAIL, both controls PASS** (`f6c-planted-mutations.txt`) |
| diff self-test | `python3 .../scripts/structural_diff_test.py` | 38 of 38 (+1: `test_f6c_apply_undoes_exactly_its_pins`) |
| freshness | `cargo run --locked --quiet -j 8 -p codex-ingest --bin sheet_rule_convert -- --check` | exit 0, `verdict=PASS` (final tree) |
| residue | `python3 scripts/pcgen_residue_gate.py --check --closure` | `verdict=PASS`, live hits 0 |
| frozen | `python3 scripts/site/check_frozen_status.py --check` | OK, frozen at 100% (49,450 units) |
| bundle | `node scripts/gen-corpus-bundle.mjs` | `files_copied=14029`; tree unchanged |

The planted mutations:

- M1: the Hellknight verdict names the older printing.
- M2: a verdict the resolver never proved (Cyphermage).
- M3: a SOURCEDATE moves.
- M4: a pinned printing is withdrawn.
- M5: an unpinned printing is planted on Fighter.
- M6: the shaman's count prints again.
- M7: the shaman's count offers domains again.
- M8: the cleric's count is withheld.
- M9: a spirit-magic domain record prints again.
- M10: the druid's Air domain stops printing.

`data/corpus/**` and `site/**` were not touched.

## 6. Live app (ui-smoke)

`RUN_DESKTOP_AGENT=f6c-smoke node scripts/ui-smoke/run.mjs --only <12 rows> --out docs/release/SD-36-consolidation/artifacts/ui-smoke/f6`.
One app, with the DEV probe on, stopped at the end. The rows are:

- the 8 `create-character-*` rows;
- the 2 existing `level-up-*` rows;
- 2 new rows. `level-up-fighter6-blockers-and-labels` checks the "Cannot be taken (10)" block with
  the Ulfen Guard, Evangelist and Unchained Rogue reasons, plus `requires Longbow chosen for Weapon Focus`.
  `sheet-f6c-shaman-no-domain-line` forbids `(domains)` and `Domains\nLife (Spirit)` on a Shaman 1's
  Rules and features, and expects `Spirit Magic`.

`spec.json` now has 81 rows (79 + 2). The inventory doc was regenerated.

**Result: 11 of 12 green** (`artifacts/ui-smoke/f6/results.json`, `run.log`, screenshots). All 3
`level-up-*` rows are green, both new rows included, and 7 of 8 `create-character-*` rows are green.

- **The red row is `create-character-render`, red on a cold app only.** It is the first row after
  launch. Its snapshot still read "Loading" when the row's wait budget ran out, because the
  debug-build census roster warm-up takes about 80 s (the cold option build measured 80.7 s, §1).
  On the same build, after ten landing and settings rows had warmed the app, it is **green**
  (`f6/warm-rerun/`: 11 of 11). F6c does not touch the creation path: `f4c-class-roster-wire.json`
  was regenerated and did not change.
- **What the first live runs found, now fixed** (`f6/first-run/`):
  - The app's own dev build (non-test) did not compile, because `level_up_blocker` called an import
    that is `#[cfg(test)]`-only. The call is now fully qualified, and the non-test build is part of
    §7.
  - The "Cannot be taken" list sat in a collapsed `<details>` the probe could not open. It now
    always prints.
  - The Shaman sheet printed `Domains / Life (Spirit)`. This is closed by the second half of the
    §3 rule.
  - The Shaman row hit the global `failure` forbid because the Chain Shirt's rule text reads
    "Arcane spell failure: 20%". The row now has `allowGlobalForbid: ["failure"]`, with that reason
    in its notes.

## 7. Verification (one pass, after all changes): `f6c-verify.log`

| Check | Result |
|---|---|
| converter `--check` | `verdict=PASS`; records 49,450, converted 49,450, refused 0, rules 73,363, var tables 6,211 |
| `pcgen_residue_gate.py --check --closure` | live_hits=0, `verdict=PASS` |
| `check_frozen_status.py --check` | OK, frozen at 100% (49,450 units) |
| `gen-corpus-bundle.mjs` | files_copied=14029; tree unchanged |
| F6c scans (`--nocapture`, the three tests) | 3 passed; counts as in §2-§4 |
| root lib `cargo test --locked -j 8 --lib -- --test-threads=8` (covers multiclass, class_census, level_up_option_filter) | 2744 passed, 0 failed, 6 ignored |
| the 7 `tests/sd36_*.rs` files | 54 passed, 0 failed |
| codex-ingest `--lib sheet_rule` (pool_link, reprint) | 69 passed, 0 failed |
| root clippy `--tests --workspace` | exit 0, 0 warnings |
| desktop `cargo test` | 626 passed, 0 failed |
| desktop non-test build, `--no-default-features` (the app's dev command) | exit 0, 0 errors (its 67 dead-code warnings are in code F6c does not touch) |
| desktop clippy `--tests` | exit 0, 0 warnings |
| `class_census --json` vs `census-f5.json` | every count equal; the only difference is the 12 prestige `alone_blocking_diagnostics` messages whose requirement text now prints labels (§2) |
| `gen_class_status_table.py --check` | OK (ids 137, computed 63, prestige swept 74, mix panel 185 of 185) |
| frontend `npm run typecheck && npm test` | clean; 127/127 test files |

Pass 1 of this table was also all green. It was redone because the live run then found the
non-test build break and the `Domains` line (§6).

## 8. Remainder, by mechanism

- **The four Unchained base classes cannot join a mix** (§1). The mix gate has no save source for
  them; their saves live in the bespoke module. Level Up shows this as `multiclass.save_shape.unknown`.
- **The six FS-15 prestige classes** stay Blocked. The cause is the oracle's `BONUS:SAVE` formula
  defect, closable only by a book-cited override (FS-15). Level Up now shows it on the option.
- **Catalog field summaries word a `Chosen` choice by its id.** There are 33 grant references,
  rendered by `sheet_rule_catalog::fact_words` / `weapon_words` (for example "the weapon chosen for
  occult adventures:class feature:champion weapon choice"). This is the reference-library field
  summary, not a sheet or Level Up line. Those describers take no package, so they cannot read a
  label. Fixing it means passing the package in, the way `describe_gate` already does.
- The shaman sheet's "Class skills Unknown" is F6a's named remainder (9 ACG classes whose converted
  package leaves the `Class|<Class>` edge unresolved), not F6c.

## 9. Artifacts

- `f6c-red.log`, `f6c-green.log`, `f6c-verify.log`
- `f6c-structural-diff.txt`, `f6c-planted-mutations.txt`, `f6c-shaman5-lines.txt`
- `../scripts/f6c_delta_pins.py`, `../scripts/f6c_planted_mutations.py`,
  `../scripts/structural_diff_f6c_deltas.json`; `structural_diff.py` and its self-test were
  extended
- `../stage-f4-f5/f4c-level-up-fighter6-wire.json`, regenerated
  (`CODEX_WRITE_F4C_WIRE=1`): blockers on 10 options, and the 12 requirement lines now print labels
- `../../ui-smoke/f6/` (the run, `first-run/`, `warm-rerun/`)
