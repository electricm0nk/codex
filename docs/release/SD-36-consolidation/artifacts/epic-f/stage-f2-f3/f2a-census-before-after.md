# F2a — census before / after the generic class-family gate arm

Step F2a of SD-36 Epic F (`epic-f-class-completion.md` §4; acceptance rows F2.1 and F2.3 in
`epic-breakdown.md`). Branch `sd36/epic-f2-f3`, measured 2026-09-24.

**Command, both runs:** `cargo run --locked -j 8 --bin class_census -- --json <path>`.
BEFORE ran at the branch base (`cc21cac195`, no F2a code); its per-class and prestige
sections are byte-identical to the committed `artifacts/epic-f/census-f1c.json` (only
`generated_at` differs). AFTER is committed as `artifacts/epic-f/census-f2a.json`.

## Headline

| figure | BEFORE | AFTER | denominator |
|---|---|---|---|
| `ids` | 135 | **137** | merged census ids |
| `computed` (non-prestige, every level) | 61 of 61 | **63 of 63** | `non_prestige_swept` |
| `blocked` (non-prestige) | 0 | 0 | `non_prestige_swept` |
| `prestige_alone_blocked` | 74 of 74 | 74 of 74 | `prestige_swept` |
| `prestige_mix_computed` | 0 of 74 | 0 of 74 | `prestige_swept` |
| `prestige_mix_unknown` | 11 | 11 | `prestige_swept` |
| `mix_panel_computed` | 185 of 185 | 185 of 185 | `mix_panel_swept` (section byte-identical) |

## F2.3 did not hold as written — and the rise is not a double count

F2.3 expected `computed` to be the same number before and after, with any rise meaning an id
double-counted against an existing arm. The number rose by 2, from **a different mechanism**:

* **No existing id moved.** All 61 non-prestige ids from BEFORE have the same status, the
  same computed/blocked levels and the same blocking diagnostics AFTER (the table below).
  Nothing was counted twice: the census is keyed by class id, and 24 existing ids only
  gained `generic_class_chassis` in their `registries` list.
* **Two ids are new:** `class:ex_antipaladin` and `class:ex_inquisitor`. Appending
  `advanced_players_guide` to `CLASS_FAMILY_BOOKS` brought APG's two `Ex-*` variant classes
  into `generic_class_chassis`'s population. None of the eight canonical census sources
  claims an APG `Ex-*` class (CRB's Ex-Barbarian/Ex-Paladin are registered by
  `crb_untabled_class_chassis`; nothing registers the APG pair), so the census files them
  under `GenericOnly`. The new gate arm admits both (non-prestige, real chassis row), and
  both reach `Computed` at all 20 levels on the census fixture.
* **The two chassis are checked against PF1, not just present:**
  `generic_class_chassis::tests::the_two_apg_ex_classes_resolve_their_parent_class_chassis`
  (Ex-Antipaladin level 10 = +10 / +7 / +3 / +7, the Antipaladin table; Ex-Inquisitor level 8
  = +6 / +6 / +2 / +6, the Inquisitor table).
* **Why they were not excluded:** the converted records carry no field that separates them
  from the base classes (`tags` are `["Base", "PC"]`, like Ex-Paladin's; PCGen's `EXCLASS`
  link is not converted). Excluding them would take a per-book or per-slug rule, which the
  doctrine forbids ("ONE mechanical rule, never per-class special cases").

Pins moved, each with a `scripts/retro.py correction`: `class_census` tests
`every_registry_is_swept_once` (135 -> 137, `GenericOnly` = 2),
`census_id_set_matches_the_published_partition` (135 -> 137, 61 -> 63 of 63),
`sweep_covers_exactly_the_non_prestige_ids` (61 -> 63), and
`no_generic_only_stragglers_today`, renamed `generic_only_ids_are_exactly_the_two_apg_ex_classes`
(it pins the two ids by name). `scripts/verify-baselines.env` is a floor
(`check_class_census_baselines.py`: fails only when a figure falls below its baseline), so
`BASELINE_CENSUS_IDS=135` and `BASELINE_CENSUS_COMPUTED=61` still pass and are left for the
F2 closure to raise.

## Prestige alone: same verdict, uniform diagnostics

All 74 prestige ids stay `Blocked` alone. Their alone diagnostics are now the same for all 74:

| prestige ids | BEFORE | AFTER |
|---|---|---|
| 56 (original 14 books) | `combat.baseline_unsupported`, `defense.total_save.unsupported`, `skill.selected_modifier.unsupported` | + `class_chassis.prestige_entry_gate.unmet` |
| 18 (CRB 10 + APG 8) | `class_chassis.prestige_entry_gate.unmet`, `class_chassis.unsupported`, and the same three | `class_chassis.unsupported` gone (the class now dispatches its converted chassis row) |

Mechanism: the CRB/APG prestige classes now reach `compute_class_chassis`'s generic arm, and
that arm now also runs the prestige entry gate for a `Prestige`-tagged record. Before F2a
the gate ran only in the fallthrough arm, which only the 18 reached. F2.2's
prestige-alone diagnostic is still to come.

## Other dispatch change: arm order

`crb_untabled_class_chassis`'s arm now sits above the generic arm in `compute_class_chassis`.
The generic population carries CRB's 5 NPC + 2 `Ex-*` records too, so this keeps them on the
arm that owns them (same numbers, since both read the one converted record). `resolve` and
`is_prestige` also now require the `class:` prefix: a bare `"wizard"` would otherwise resolve
through the appended CRB record (`tests/sd20_contract_pilot_receipt.rs` and
`tests/sd20_contract_cell_map.rs` rely on bare `"wizard"` being unsupported).

## F2.1 — generic population pin

`generic_class_chassis::tests::every_conventional_class_in_class_family_books_resolves` (was
`all_seventy_eight_conventional_classes_resolve`): **78 -> 122** (distinct slugs, first book
in list order wins).

* `core_rulebook` +27 = 10 base PC (every CRB base class except Monk, whose converted record
  has no `BaseAttack` row) + 5 NPC + 2 `Ex-*` + 10 prestige.
* `advanced_players_guide` +17 = the 6 `ApgClassId` classes + Antipaladin + 2 `Ex-*` + 8
  prestige (Eidolon is tagged `Monster`, so it is not a conventional class).
* Shadowed slugs: none from CRB or APG. The three pre-existing collisions are pinned by name in
  `class_family_books_end_with_crb_then_apg_and_every_shadowed_slug_is_named`:
  `cyphermage` (adventurers_guide over inner_sea_magic), `hellknight` and
  `red_mantis_assassin` (adventurers_guide over inner_sea_world_guide).
* Against the spec ceiling "96, less any slug a bespoke arm already owns": 96 = 78 + the 18
  CRB/APG prestige classes. The other 26 appended slugs are base classes: 24 are owned by a
  bespoke arm that dispatches first (CRB table 10, APG table 6, untabled Antipaladin, CRB
  NPC/Ex 7), and 2 are the APG `Ex-*` pair above. 78 + 18 + 24 + 2 = 122.
* Desktop mirror (`class_catalog_generic.rs`): same 16 books, same order, own pin
  (`class_family_books_end_with_crb_then_apg_and_every_shadowed_slug_is_named`, 122 distinct
  slugs over 125 (book, slug) records). The catalog does not re-list the 10 CRB base classes
  `class_tables()` already prints (112 distinct generic names; catalog 1318 -> 1818 rows).

## Verification (2026-09-24, `CARGO_TARGET_DIR` scratch, `-j 8`, `--test-threads=8`)

| command | result |
|---|---|
| RED: `cargo test --locked -j 8 --lib generic_class -- --test-threads=8` before the new fn/books existed | did not compile (`is_prestige`, `is_supported_generic_class_family_single_class` missing); then with the fns but only 14 books, 3 of 15 FAILED (`class_family_books_end_with_crb_then_apg_and_every_shadowed_slug_is_named`, `crb_and_apg_prestige_classes_resolve_a_real_chassis`, `is_prestige_reads_the_record_s_own_prestige_tag`) |
| `cargo test --locked -j 8 --lib generic_class_chassis` | 11 passed, 0 failed |
| `cargo test --locked -j 8 --lib class_shared_core` | 3 passed (`a_generic_family_base_class_passes_the_shared_gate`, `a_prestige_class_never_passes_the_generic_family_arm` = 74 of 74 prestige refused, `the_generic_family_arm_refuses_past_the_ceiling_and_for_a_mix`) |
| `cargo test --locked -j 8 --lib class_census` | 30 passed, 1 ignored |
| `cargo test --locked -j 8 --no-fail-fast` (root, every target) | 287 result lines, 6,318 passed, 0 failed, 28 ignored |
| `cargo clippy --locked --tests -j 8 -- -D warnings` (root) | clean |
| desktop `cargo clippy --locked --tests -j 8 -- -D warnings` | clean |
| desktop `cargo test --locked -j 8` | 615 passed, 0 failed |
| `python3 scripts/gen_class_status_table.py --check` (AFTER json) | OK (table regenerated: ids=137 computed=63) |
| `python3 scripts/check_class_census_baselines.py` (AFTER json, env baselines) | OK |
| `python3 scripts/pcgen_residue_gate.py --check --closure` | PASS |

## Per-class statuses (non-prestige; status (computed levels / max level))

| class id | BEFORE | AFTER | change |
|---|---|---|---|
| class:adept | Computed (20/20) | Computed (20/20) | + generic_class_chassis registry |
| class:aegis | Computed (20/20) | Computed (20/20) |  |
| class:alchemist | Computed (20/20) | Computed (20/20) | + generic_class_chassis registry |
| class:antipaladin | Computed (20/20) | Computed (20/20) | + generic_class_chassis registry |
| class:arcanist | Computed (20/20) | Computed (20/20) |  |
| class:aristocrat | Computed (20/20) | Computed (20/20) | + generic_class_chassis registry |
| class:barbarian | Computed (20/20) | Computed (20/20) | + generic_class_chassis registry |
| class:bard | Computed (20/20) | Computed (20/20) | + generic_class_chassis registry |
| class:bloodrager | Computed (20/20) | Computed (20/20) |  |
| class:brawler | Computed (20/20) | Computed (20/20) |  |
| class:cavalier | Computed (20/20) | Computed (20/20) | + generic_class_chassis registry |
| class:cleric | Computed (20/20) | Computed (20/20) | + generic_class_chassis registry |
| class:commoner | Computed (20/20) | Computed (20/20) | + generic_class_chassis registry |
| class:cryptic | Computed (20/20) | Computed (20/20) |  |
| class:dread | Computed (20/20) | Computed (20/20) |  |
| class:druid | Computed (20/20) | Computed (20/20) | + generic_class_chassis registry |
| class:ex_antipaladin | — | Computed (20/20) | NEW id (GenericOnly) |
| class:ex_barbarian | Computed (20/20) | Computed (20/20) | + generic_class_chassis registry |
| class:ex_inquisitor | — | Computed (20/20) | NEW id (GenericOnly) |
| class:ex_paladin | Computed (20/20) | Computed (20/20) | + generic_class_chassis registry |
| class:expert | Computed (20/20) | Computed (20/20) | + generic_class_chassis registry |
| class:fighter | Computed (20/20) | Computed (20/20) | + generic_class_chassis registry |
| class:gunslinger | Computed (20/20) | Computed (20/20) |  |
| class:hunter | Computed (20/20) | Computed (20/20) |  |
| class:inquisitor | Computed (20/20) | Computed (20/20) | + generic_class_chassis registry |
| class:investigator | Computed (20/20) | Computed (20/20) |  |
| class:kineticist | Computed (20/20) | Computed (20/20) |  |
| class:magus | Computed (20/20) | Computed (20/20) |  |
| class:marksman | Computed (20/20) | Computed (20/20) |  |
| class:medium | Computed (20/20) | Computed (20/20) |  |
| class:mesmerist | Computed (20/20) | Computed (20/20) |  |
| class:monk | Computed (20/20) | Computed (20/20) |  |
| class:ninja | Computed (20/20) | Computed (20/20) |  |
| class:occultist | Computed (20/20) | Computed (20/20) |  |
| class:oracle | Computed (20/20) | Computed (20/20) | + generic_class_chassis registry |
| class:paladin | Computed (20/20) | Computed (20/20) | + generic_class_chassis registry |
| class:psion | Computed (20/20) | Computed (20/20) |  |
| class:psychic | Computed (20/20) | Computed (20/20) |  |
| class:psychic_warrior | Computed (20/20) | Computed (20/20) |  |
| class:ranger | Computed (20/20) | Computed (20/20) | + generic_class_chassis registry |
| class:rogue | Computed (20/20) | Computed (20/20) | + generic_class_chassis registry |
| class:samurai | Computed (20/20) | Computed (20/20) |  |
| class:shaman | Computed (20/20) | Computed (20/20) |  |
| class:shifter | Computed (20/20) | Computed (20/20) |  |
| class:skald | Computed (20/20) | Computed (20/20) |  |
| class:slayer | Computed (20/20) | Computed (20/20) |  |
| class:sorcerer | Computed (20/20) | Computed (20/20) | + generic_class_chassis registry |
| class:soulknife | Computed (20/20) | Computed (20/20) |  |
| class:spiritualist | Computed (20/20) | Computed (20/20) |  |
| class:summoner | Computed (20/20) | Computed (20/20) | + generic_class_chassis registry |
| class:swashbuckler | Computed (20/20) | Computed (20/20) |  |
| class:tactician | Computed (20/20) | Computed (20/20) |  |
| class:unchained_barbarian | Computed (20/20) | Computed (20/20) |  |
| class:unchained_monk | Computed (20/20) | Computed (20/20) |  |
| class:unchained_rogue | Computed (20/20) | Computed (20/20) |  |
| class:unchained_summoner | Computed (20/20) | Computed (20/20) |  |
| class:vigilante | Computed (20/20) | Computed (20/20) |  |
| class:vitalist | Computed (20/20) | Computed (20/20) |  |
| class:warpriest | Computed (20/20) | Computed (20/20) |  |
| class:warrior | Computed (20/20) | Computed (20/20) | + generic_class_chassis registry |
| class:wilder | Computed (20/20) | Computed (20/20) |  |
| class:witch | Computed (20/20) | Computed (20/20) | + generic_class_chassis registry |
| class:wizard | Computed (20/20) | Computed (20/20) | + generic_class_chassis registry |