# SD-36 Epic F4pre: selection pools become converted choices (receipt)

Scope: FS-21, before F4 offers these classes in the Create picker. The SD-32 generic pool-group
pass (`push_generic_pool_group_selection_magnitude`) printed **5,460** member values with no level
gate on selections that had no converted pick option (a level-5 cleric printing an 8th-level domain
power is a wrong sheet). This is a **converter step**: `data/sheet_rules/**` was regenerated under
the structural-diff protocol.

- Branch `sd36/epic-f4-f5`, worktree `/home/ubuntu/workspace/worktrees/codex-epic-f4`, on top of F3p
  (`6db56623e8`).
- Structural-diff baseline: tranche/16 HEAD `070c253e93`, extracted read-only with
  `git -C /home/ubuntu/workspace/repos/codex archive tranche/16 data/sheet_rules`. Pins baseline:
  the package at `6db56623e8` (`git archive HEAD data/sheet_rules`), so the pins name only F4pre.
- Oracle pin `7f818006e371188e5717fd18d74d18a420747fc6`.
- Tests: `tests/sd36_f4pre_pool_choices.rs` (new, 10 tests);
  `pool_link::tests::ability_pool_picks_and_domain_counts_offer_their_members` (new).

## 1. Every pool the pass prints, and its converted choice

Denominator: the pass's 6 callers (the pools it prints).

| pool | the oracle's pick | converted choice before F4pre | after F4pre |
|---|---|---|---|
| Sorcerer Bloodline | `CHOOSE:ABILITYSELECTION\|Sorcerer Bloodline` + `ABILITY:...\|%LIST` (`cr_abilities_class.lst:2329`) | yes: F3c4b `pool_option` rules, `Granter::Choice` | unchanged |
| Bloodrager Bloodline | the same shape (ACG) | yes: F3c4b | unchanged |
| Cleric Domain | `BONUS:DOMAIN\|NUMBER\|ClericDomainCount` on `CLASS:Cleric` (`cr_classes.lst:55`), `BONUS:VAR\|ClericDomainCount\|2` | **no** (converted as a bare `Other("domains")` line) | `core_rulebook:class:cleric#bonus1` offers `Domains`, count `ClericDomainCount` |
| Shaman Spirit | `BONUS:ABILITYPOOL\|Shaman Spirit\|1` on `Shaman ~ Spirit` (`acg_abilities_class.lst:1386`); category `CATEGORY:Special Ability TYPE:ShamanSpirit` (`acg_abilitycategories.lst:100`) | **no** (a bare `Pool` line) | `advanced_class_guide:class_feature:shaman_spirit` offers `Rules{special_ability, [ShamanSpirit]}` |
| Warpriest Blessing | `BONUS:ABILITYPOOL\|Warpriest Blessing\|2` (`acg_abilities_class.lst:2146`) | no | offers `Rules{special_ability, [Blessings]}` |
| Cavalier Order | `BONUS:ABILITYPOOL\|Cavalier Order\|1` (`apg_abilities_class.lst:195`) | no | offers `Rules{special_ability, [CavalierOrder, Cavalier Order]}` |

Warpriest Blessing and Cavalier Order printed 0 values before and after (F3c4's finding).

## 2. The rule (one mechanism, no per-class case)

`crates/codex-ingest/src/pcgen_import/sheet_rule/pool_link.rs::link_pool_choices`. The oracle hands out
a pick among a category's objects with two tokens, and both now convert as a choice on the pick:

- `BONUS:ABILITYPOOL|<C>|<n>` (PCGen: `<n>` picks in `<C>`, whose members are every ability of its
  parent `CATEGORY:` carrying all of its `TYPE:` tags) -> `offers: {id: <pick>, count: <n>, from:
  Rules{pool: <parent slug>, tags: <TYPE tags>, requires: Always}}`, when `<C>` is a child category
  with a `TYPE:` view that selects at least one converted record. This is D6's shape
  (`link_weapon_choice_pools`) without D6's single-member-weapon restriction; D6 runs first and is
  unchanged.
- `BONUS:DOMAIN|NUMBER|<n>` -> `offers: {id: <line>, count: <n>, from: Domains}`.

Left out, as before: a zero or negative count (a record spending a pick it fills itself), a pick
that already offers a choice, a pool whose name is itself a parent category, and a category with no
`TYPE:` view or with a view that selects no converted record.

**The options carry their own grants.** Each option is the member record itself: `Shaman Spirit ~
Battle` carries the `ABILITY:` edges to `Battle Spirit ~ ...`, gated `PREVARGTEQ:ShamanSpiritGreater,1`
and so on, and `domain:air` carries `Core Domain ~ Air Domain` -> `Domain Power ~ Lightning Arc`,
gated `DomainAirAbilityTriggerLVL >= 1`. So the lines are held at the level the record states.

**No member edge (a deliberate deviation from D8's materialised `Granter::Choice` edges).** A
`TYPE`-filtered child category can be the size of the feat list. Measured on the pre-F4pre package
with a Python proxy: materialising D8's edge for every `BONUS:ABILITYPOOL` pick would add about
102,000 `granted_by` edges (feat 52,434, special ability 44,728). Every pick whose members reach a
weapon-proficiency grant would also flip into the proficiency reader's `unrecorded_member_pick`
Unknown, for example Fighter Bonus Feat. Instead the engine reads the option set (§3), and the
package carries one `offers` field per pick.

**Population** (`structural_diff_f4pre_deltas.json`, from `f4pre_delta_pins.py`): **1,733 offers
added** on 1,733 rules.

- 1,686 ability-pool picks, by parent: special ability 1,387, internal 163, feat 125, words of
  power 3, background 2, mythic spell 2, aligned class 1, class skill 1, and the two bloodrager
  subcategories 1 each.
- 47 domain counts: Cleric, Paladin, Shaman, Daughter of Urgathoa, Nature's Bond (Druid Domain),
  the two Tempest Druid Nature Bonds (AG, ISM), Divine Hunter, Emissary Domain Influence, the 33
  Forbidden Rites domains, and the 5 Varisian Pilgrim subdomains.
- 0 deltas of any other shape.
- Records 49,450 -> 49,450; rules 73,363 -> 73,363; var tables 6,211 -> 6,211.

## 3. The engine (`src/rules_core/sheet_rule.rs`)

- **`held_set`.** An option recorded under a choice the character holds is held when the choice's
  option set selects it (`offer_selects`: `Rules` means the principal rule is in the offered pool and
  carries every offered tag, case-insensitive as PCGen matches `TYPE`; `Domains` means a `domain`
  rule) and when the option's own gate (plus the set's `requires`) includes. It is held through the
  chooser's holder class. Choices whose members carry `Granter::Choice` edges (D8, F3c3, F3c4b) are
  held through those edges exactly as before.
- **`link_path_a_picks`.** When no pick-row option answers (the F3c4 path), a Path-A pick
  `choice:<pool> -> <ns>:<member>` names the rule called `<member>` or `<pool>_<member>` (the oracle's
  `<Category> ~ <Member>` key) that a held choice offers. One printing per kind is used
  (`SheetRulePackage::find_in_every_kind`, the package's `find` printing), so Scalykind, which three
  books print, is one option. A pick that several choices or options answer is still not linked.
- **`sheet_rule_package::linked_picks`.** The cheap pre-check also accepts `<member>` in any kind.

## 4. RED -> GREEN

RED (`f4pre-red.log`): `cargo test --locked -j 8 --test sd36_f4pre_pool_choices -- --test-threads=8`
on the unchanged engine, converter and package. **7 of 8 failed.** The control
`an_invented_domain_links_to_nothing` passed.

| test | RED | GREEN |
|---|---|---|
| `the_domain_count_and_the_spirit_pool_convert_as_choices` | cleric domain count offers `None` | `Domains`, count `Var(ClericDomainCount)`; shaman spirit `Rules{special_ability,[ShamanSpirit]}` |
| `a_cleric_domain_pick_links_to_the_domain_choice_and_holds_the_domain` | no link | `(cleric#bonus1, core_rulebook:domain:air)`, held |
| `an_air_cleric_holds_each_domain_power_at_the_level_the_book_grants_it` | domain not held | CRB p.41: Lightning Arc held at 5; Electricity Resistance absent at 5 and held at 6 |
| `the_pool_pass_prints_no_air_domain_value_for_a_linked_pick` | pass printed `class_feature.cleric.domain.generic.air_domain.*` | none at 1, 5, 20 |
| `a_shaman_spirit_pick_links_to_the_spirit_choice_and_holds_the_spirit` | no link | `(shaman_spirit, shaman_spirit_battle)`, held |
| `a_battle_shaman_holds_each_spirit_ability_at_the_level_the_book_grants_it` | not held | ACG p.35/37 and the oracle's `ShamanSpiritGreater` at 8 and `ShamanSpiritTrue` at 16: Battle Spirit only at 5; + Enemies' Bane at 8; + Paragon of Battle at 16 |
| `the_pool_pass_prints_no_battle_spirit_value_for_a_linked_pick` | pass printed | none at 1, 5, 20 |
| `an_invented_domain_links_to_nothing` | ok (control) | ok |
| `the_canonical_seeds_are_the_new_choices_defaults` | added after the first GREEN | see §6 |
| `a_domain_three_books_print_links_to_one_printing` | added after the first after-scan showed Scalykind unlinked (3 printings -> ambiguous) | links `bestiary_6:domain:scalykind`, held |

GREEN: `f4pre-green.log`, 10 of 10.

## 5. Ungated values: 5,460 -> 3,420 (`f4pre-pool-pass-scan.md`)

Script: `artifacts/epic-f/scripts/f4pre_pool_pass_scan.rs`. It is the F3c4 scan unchanged, plus a
`SEL` row per selection. It was appended temporarily to `pool_groups.rs` and run with `cargo test
--locked -j 8 --lib -- --test-threads=8 --nocapture f4pre_pool_pass_scan`, then removed.

Denominator, the same as F3c4's:
- 6 pools;
- 191 selections the pass's own `real_pool_group_for_selection_slug` resolves;
- levels 1..=20;
- the census fixture.

The before run was on `6db56623e8` and reproduces F3c4's 5,460 exactly.

| pool | ungated before | after |
|---|---:|---:|
| Sorcerer Bloodline | 1,040 | 1,040 |
| Bloodrager Bloodline | 220 | 220 |
| Cleric Domain | 2,960 | **1,920** (30 of 73 selections now linked and held) |
| Shaman Spirit | 1,240 | **240** (12 of 14) |
| Warpriest Blessing / Cavalier Order | 0 / 0 | 0 / 0 |
| **total** | **5,460** | **3,420** |

**The remainder, by mechanism** (every selection is classified in `f4pre-pool-pass-scan.md`; 0
unclassified):

- **(a) Not a pick the pool offers, 1,700.** The scan's candidate rule resolves some groups that are
  not selectable objects: `Forbidden Rites Domain`, a magus archetype group (1,380); `Core Domain`
  (40); and the pools' own names, `Shaman Spirit` (240) and `Bloodrager Bloodline` (40). No
  character records these as a pick.
- **(b) A wildblooded mutation, not a bloodline, 560 over 16 selections.** `Wildblooded ~ <X>`
  (UM p.70) is picked in `Wildblooded Bloodline Mutation` behind the Wildblooded archetype, so
  `bloodline:<x>` names no bloodline option.
- **(c) The option's own gate excludes this character, 920.**
  - 11 druid-only domains (`PRECLASS:1,Druid=1`, `um_domains.lst`), 320.
  - Chaos, Evil and Law, 120. They are alignment-gated, and the character record carries no
    alignment, so the held set never assumes one.
  - FS-19's Imperious and Kobold (race template), 480.
- **(d) An inquisition, not a domain, 60.** Heresy, Oblivion and Tactics are `Inquisition ~ <X>`
  (UM). There is no domain rule for them.
- **(e) No converted pick option, 180.** Verdant Bloodrager (UW): no bloodrager pick option or
  choice selects its record.

None of these values feeds a sheet total. FS-21 is updated with this outcome.

The cleric `LINE` rows read `NOT-HELD` because the pass reads the `<X> Domain ~ <power>` records
(`cr_abilities_class.lst:3179+`), which only the inquisitor's `Inquisitor Domain ~ <X>` records
grant. The held set holds the `Domain Power ~ <power>` records the domain itself grants. The yield
removes the pass's values, and the domain's own lines print at their levels.

## 6. class_seeds canonical defaults

No seed changed. The new choices' defaults are **the fixture's existing picks**
(`class_seeds::canonical_seeds_for`), which now link to the converted choices:

- **Shaman: `spirit:life`**, linked to `(shaman_spirit, shaman_spirit_life)` and held.
- **Cleric: `domain:good`**, linked to `(cleric#bonus1, core_rulebook:domain:good)` but **not
  held**. Good is `PREALIGN:LG,NG,CG` (`cr_domains.lst`), and the character record carries no
  alignment. This is pinned by `the_canonical_seeds_are_the_new_choices_defaults`. The cleric's
  second domain slot (`ClericDomainCount` = 2) stays unrecorded, as before. No default was invented.

## 7. Census (`census-f4pre.json`)

`cargo run --locked -j 8 --bin class_census -- --json <scratch>/census-f4pre.json`, final tree:

```
ids=137 computed=63 blocked=0
prestige_swept=74 prestige_alone_blocked=74 prestige_mix_computed=68 prestige_mix_unknown=0
mix_panel_swept=185 mix_panel_computed=185 mix_panel_blocked=0
```

- Per-key diff against `census-f3p.json`: 1 key, `generated_at`. **0 rows move.**
- Against `census-f3c5.json`: 33 keys, all of them `generated_at` plus F3p's 32 entry-requirement
  text keys.
- `scripts/check_class_census_baselines.py` (135/63/74/185/68): OK.
- `scripts/gen_class_status_table.py --check`: OK.

## 8. Converter gates

| gate | command | result |
|---|---|---|
| write | `cargo run --locked --quiet -j 8 -p codex-ingest --bin sheet_rule_convert -- --write` | exit 0; records 49,450, converted 49,450, refused 0; rules 73,363; var tables 6,211 |
| pins | `python3 .../scripts/f4pre_delta_pins.py <6db56623e8 pkg> data/sheet_rules .../structural_diff_f4pre_deltas.json` | 1,733 offers pinned; 0 unexplained |
| structural diff | `python3 .../scripts/structural_diff.py data/sheet_rules --baseline <tranche/16 pkg>` | **`verdict=PASS`**, exit 0 (`f4pre-structural-diff.txt`): F4pre 1,733 of 1,733 offers removed before the diff; F3p 465 + 414 of 465 + 414; unexpected field deltas 0; removed edges 0; removed grants 0 |
| planted mutations | `python3 .../scripts/f4pre_planted_mutations.py <scratch> . <tranche/16 pkg>` | **7 of 7 FAIL, both controls PASS** (`f4pre-planted-mutations.txt`) |
| diff self-test | `python3 .../scripts/structural_diff_test.py` | 37 of 37 (+1: `test_f4pre_offer_pins_gate_a_withdrawn_offer_and_an_unpinned_one`) |
| freshness | `cargo run --locked --quiet -j 8 -p codex-ingest --bin sheet_rule_convert -- --check` | exit 0, `verdict=PASS` (final tree) |
| residue | `python3 scripts/pcgen_residue_gate.py --check --closure` | `verdict=PASS`, shipped_scanned 70,045, hits 0 |
| frozen | `python3 scripts/site/check_frozen_status.py --check` | OK, frozen at 100% (49,450 units) |
| bundle | `node scripts/gen-corpus-bundle.mjs` | `files_copied=14029`; tree unchanged |

The planted mutations:
- M1: the cleric's domain choice withdrawn. This deactivates the owner check, so all 1,732 other
  offers surface as unexpected field deltas.
- M2: the domain count becomes 3.
- M3: the spirit choice offers another category's tag.
- M4: the spirit pick offers the domains.
- M5: the spirit choice is keyed under another rule.
- M6: an unpinned offer is planted on Power Attack.
- M7: the spirit option set gains a requirement.

`data/corpus/**` and `site/**` are untouched. Under `data/`, only `data/sheet_rules/**` moved:
1,382 rule files (the offers; some files carry several picks).

## 9. Moved pins (fixture protocol, `docs/retro/events/sd36-f4pre-executor.jsonl`)

The full root suite after the converter step (`f4pre-full-root.log`) had exactly 2 failures, both
generic-pass pins that assumed the pass prints for a cleric domain. Both moved the F3c4 way: count
0, and the held set prints the domain's own powers.

- **`pool_groups::generic_pool_group_selection_wiring_tests::cleric_generic_domain_pass_grounds_a_never_hand_modelled_domain`
  (Plant).** Wooden Fist held at cleric 5; Bramble Armor not yet (CRB p.46, 6th).
- **`...::cleric_generic_domain_pass_grounds_animal_via_the_class_record_merge` (Animal).** Speak
  with Animals and the Animal Companion power held at cleric 5 (CRB p.40, 1st and 4th).

## 10. Verify

| what | command | result |
|---|---|---|
| F4pre tests | `cargo test --locked -j 8 --test sd36_f4pre_pool_choices -- --test-threads=8` | 10 passed |
| moved pins | `cargo test --locked -j 8 --lib -- --test-threads=8 pool_groups` | 40 passed |
| full root | `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` | 293 result lines, 6,387 passed, **2 failed (the moved pins, §9)**, 27 ignored; re-run green after the move |
| ingest, all targets | `cargo test --locked -j 8 -p codex-ingest --no-fail-fast -- --test-threads=8` | exit 0; 167 result lines, 1,764 passed, 0 failed, 43 ignored |
| desktop, all targets | `cargo test --locked -j 8 --manifest-path apps/desktop/src-tauri/Cargo.toml --no-fail-fast -- --test-threads=8` | exit 0; 615 passed |
| clippy root / ingest / desktop | `cargo clippy --locked --tests -j 8 [-p codex-ingest \| --manifest-path apps/desktop/src-tauri/Cargo.toml] -- -D warnings` | exit 0 / 0 / 0 |
| census | §7 | unchanged against `census-f3p.json` |

The first ingest build failed to compile the new unit test (`Expr::Const` takes `i32`). It was
fixed and the suite re-run green (`f4pre-full-ingest.log`).

## 11. What stays open

- **FS-21 remainder, 3,420 values:** mechanisms (a)-(e) in §5.
- **Granter::Deity (not changed, noted).** A deity's `DOMAINS:` converts to `Granter::Deity`, which
  the held set reads as "held when the character worships the deity": every one of that deity's
  domains is held, not two picks. No character record carries a deity today
  (`CharacterFacts::deity` is `None`), so nothing prints from it. It becomes a wrong sheet when a
  deity is recorded, so it must be decided before a deity picker ships.
