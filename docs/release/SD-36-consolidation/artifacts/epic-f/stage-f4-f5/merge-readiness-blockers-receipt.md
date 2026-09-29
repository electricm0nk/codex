# SD-36 Epic F4 — merge-readiness blockers receipt

Blockers: `merge-readiness-blockers.json` (1 blocker, written verbatim before any fix).

## Blocker 1 — `link_path_a_picks` linked a pick to a foreign chooser

**Defect (as found).** The F4pre fallback (`src/rules_core/sheet_rule.rs`, `link_path_a_picks`,
`if found.is_empty()`, commit `4248733c1d`) looked a pick's bare `<member>` up in every kind and
linked the pick to any held Domains/Rules chooser offering that rule. It read neither the pick's
namespace nor the chooser's count. The shared fixture's Human ability-bonus pick
(`choice:human_ability_bonus -> ability:strength`, the GE-06 fixture; the desktop records the same
shape in `pf1_adapter.rs` `compose_character_input`) became `core_rulebook:domain:strength` under
the cleric's, the paladin's (count +0) and the shaman's domain count.

**Fix: one rule, no per-class case.**

- `CharacterFacts` now keeps the namespace of each legacy pick (`pick_namespaces`, filled by the one
  recorder `CharacterFacts::record_pick`, which `from_character` and
  `sheet_rule_package::linked_picks` both call). `domain:air` records the namespace `domain`. A
  selection with no namespace records none.
- In the fallback, the bare `<member>` candidate answers only when the pick's namespace names the
  option's kind (`split_rule_id(option).1 == namespace`). `ability:strength` names kind `ability`,
  so it never reaches `domain:strength`. A pick with no recorded namespace gets no bare candidate.
  The `<pool>_<member>` candidate is unchanged (`spirit:battle` -> `shaman_spirit_battle`).
- `offer_open` (new): a chooser whose `offers.count` evaluates to 0 or less for this character
  offers nothing. It is read in two places, and both use this one rule: the link fallback's
  offered-chooser filter, and the held set's option-under-a-held-choice pass. The paladin's
  `Paladin (domains)` count prints +0, so the paladin takes no domain.
- `sheet_rule_package::linked_picks` keys its cache on the whole selection id, because the
  namespace now changes the answer.
- Test helpers now pass the whole selection id (`domain:air`, `spirit:battle`) through
  `record_pick`: `held_for` in `tests/sd36_f4pre_pool_choices.rs` and the facts line in the
  temporary scan script `scripts/f4pre_pool_pass_scan.rs`.

The pick itself is still recorded. Only its link to a converted chooser is withheld.

## RED -> GREEN

New file `tests/sd36_f4_link_pick_namespace.rs`, with 7 tests.

- RED (`merge-readiness-red.log`, branch HEAD `d8cf00c76a` plus the tests only): 5 of 6 FAIL. The
  premise test passes. The population scan found `10 links; foreign 3`:
  - `cleric 5: choice:human_ability_bonus -> core_rulebook:domain:strength via core_rulebook:class:cleric#bonus1`
  - `paladin 5: ... via core_rulebook:class:paladin#bonus1`
  - `shaman 5: ... via advanced_class_guide:class:shaman#bonus1`
- GREEN (`merge-readiness-green.log`): 6 of 6, plus `sd36_f4pre_pool_choices` 10 of 10 unchanged.
  The seventh test, `a_domain_count_of_zero_offers_no_domain`, was added after the first GREEN
  run so the count gate has its own proof (below).

What the tests pin:

- `a_human_cleric_with_the_bonus_on_strength_holds_no_strength_domain`: at cleric 1 and 5, there is
  no HELD `core_rulebook:domain:strength` and no LINE for the domain or for `domain_power_strength_surge`.
  This uses `class_census::sheet_dump_with_rules_text`, the same construction as `--sheet-dump --with-sheet-rules`.
- `a_human_paladin_holds_no_domain`: paladin 5 holds no rule of kind `domain`.
- `a_human_shaman_holds_no_strength_domain`.
- `the_ability_bonus_pick_links_to_nothing`: cleric, paladin and shaman at level 5.
- `a_domain_count_of_zero_offers_no_domain`: a paladin 5 who records `choice:cleric_domain -> domain:air`
  gets no link to `paladin#bonus1`.
- `no_linked_pick_over_the_non_prestige_population_comes_from_a_foreign_pool`: the population scan
  below.

**Sabotage** (`merge-readiness-sabotage.log`). Each gate was reverted by itself, then restored:

- Namespace filter reverted: 4 FAIL (the cleric, shaman and ability-pick tests, and the population
  scan). The paladin test still passes, because the count gate alone stops that domain from being
  held.
- Link-fallback count gate reverted: 1 FAIL (`a_domain_count_of_zero_offers_no_domain`).

## Population scan: every linked pick and every held rule it reaches

Command: `cargo test --locked -j 8 --test sd36_f4_link_pick_namespace no_linked -- --test-threads=8 --nocapture`
(`merge-readiness-link-scan.log`).

Denominator: the 63 non-prestige census classes (the test asserts 63), each at min(5, max level),
on the census fixture plus the canonical seeds (`class_seeds::input_for`).

- Linked picks: 10 before, 7 after. **Foreign links: 3 before, 0 after.** A link is foreign when its
  option is neither the kind the pick's namespace names nor the `<pool>_<member>` record (or that
  record's granting option).
- Held rules reached through a linked pick: 16. This is the held set with the links recorded, minus
  the held set without them. By class:
  - bloodrager 5: 4 (Arcane bloodline)
  - shaman 5: 3 (Life spirit)
  - sorcerer 5: 9 (Arcane bloodline)
  - cleric `domain:good`: 0 (linked, but gated on an alignment the record does not carry, as in F4pre)
  - inquisitor `domain:good`: 0
  - fighter `dodge` / `power_attack`: 0

  Every reached row is listed as a `REACHED|` line. None is a domain the character did not pick.

## Census (`census-merge-readiness.json`)

Command: `cargo run --locked -j 8 --bin class_census -- --json <scratch>/census-mrb.json`.

Result: ids=137, computed=63, prestige alone blocked 74 of 74, prestige mix computed 68 of 74,
mix panel 185 of 185.

Per-key diff against `census-f4pre.json`: 277 paths. All of them are:

- the F4a/F4c schema additions (`in_desktop_roster` and `roster_reason` on 63 + 74 entries,
  `roster_offered`);
- the F4a `input_posture` prose;
- `generated_at`.

**0 status rows move.** Census status does not read the sheet's domain lines, so this fix cannot
move it.

## Not re-run

The F4pre pool-pass scan (`f4pre-pool-pass-scan.md`, 5,460 -> 3,420) was not re-run, because it
takes 56 minutes. Its "linked and held" decision reads `sheet_rule_package::linked_picks` on inputs
whose selections carry their namespace (`domain:<x>`, `spirit:<x>`, `bloodline:<x>`). Under the new
rule, each of those names:

- its own kind (domain); or
- its `<pool>_<member>` record (spirit, bloodline).

Its cleric pool is `choice:cleric_domain` under the cleric's count of 2. That count is open. The
existing pins still hold at 1, 5 and 20: `the_pool_pass_prints_no_air_domain_value_for_a_linked_pick`
and `the_pool_pass_prints_no_battle_spirit_value_for_a_linked_pick`. The script's own held-check
line now records the namespace, so a re-run measures the same thing.

## Correction to `f4pre-receipt.md` §6

"No default was invented" was true of the seeds. It was false of the sheet: the fallback linked the
fixture's `ability:strength` into a domain. That sentence now points here.

## Verify (one pass, after all changes)

| gate | command | result |
|---|---|---|
| clippy | `cargo clippy --locked -j 8 --all-targets -- -D warnings` | exit 0 (`merge-readiness-clippy.log`) |
| root suite | `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` | 6,398 passed / 0 failed / 27 ignored over 294 test binaries. Baseline `f4-suite-root-test.log` was 6,391 / 0 / 27; +7 are the new tests (`merge-readiness-root-test.log`) |
| desktop suite | `cargo test --locked -j 8 --manifest-path apps/desktop/src-tauri/Cargo.toml --no-fail-fast -- --test-threads=8` | 621 passed / 0 failed / 0 ignored, the same as `f4-suite-desktop-test.log` (`merge-readiness-desktop-test.log`) |

The frontend did not change, so there is no ui-smoke re-run. The converter did not change, so
`data/sheet_rules/**` is untouched.
