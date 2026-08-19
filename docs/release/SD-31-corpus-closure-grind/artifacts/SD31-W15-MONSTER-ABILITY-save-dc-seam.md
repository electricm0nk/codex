---
canonical: true
owner: sd31-w15-monster-ability-lane
cycle: SD31-W15-MONSTER-ABILITY-001
date: 2026-08-19
oracle_pin: 7f818006e371188e5717fd18d74d18a420747fc6
---

# SD31-W15 — the `monster_ability` save-DC evaluator seam

The wave-15 seam lane for `kind=monster_ability`. Target population at dispatch: the 264
`wiring_class=derived` + `status=grounded` `monster_ability` units `doneness_verdict()` caps at
`held`, inside `monster_ability`'s wider 351-unit held mass.

## What the seam is

PF1, `Bestiary` Appendix 1 (Universal Monster Rules), "Format":

> "The save DC against a monster's special ability is equal to 10 + 1/2 the monster's racial HD +
> the monster's relevant ability modifier."

PCGen states the already-summed `10 + 1/2 racial HD` term **on the ability row**, as the `DESC:`
token's argument for the `%N` its prose introduces with the word `DC`:

```
DESC:...These creatures must succeed at a DC %1 Will save...|15+WIS
```

and it states the racial HD itself **on a different row, in a different file**, as the trailing
segment of `MONSTERCLASS:<type>:<HD>`:

```
Cephalophore	KEY:Cephalophore	...	MONSTERCLASS:Construct:12
```

The ability-modifier term stays symbolic on purpose. It depends on the creature's live ability
SCORE, and this ingest carries only `BONUS:STAT` ADJUSTMENTS, never scores (`SD31-E6-F1-002`,
`OPEN-ISSUES.md` row 44) — resolving it would be exactly the fabrication that finding refused.

## Why the fixture is not circular

Every earlier seam in this family (`equipment` `BONUS:STAT`, `monster` `SLA_CL`, `spell`
`DURATION`/`RANGE`, `class_feature` `BONUS:VAR`) pins an expected value read off **the same row**
the evaluator parses; its independence rests entirely on the two readings arriving through
different artifacts (upstream `.lst` bytes vs. this repo's `data/corpus/` ingest).

This seam adds a second, stronger independence: the expected value is **also** fixed by a second
corpus row the evaluator never reads, tied to the first by the printed rule.

| | source | read by |
|---|---|---|
| the evaluator's output | the ability row's `DESC:` argument, via the compiled `monster_chassis` tables generated from `data/corpus/` | `rules_core::derived_evaluator_fixture_check::monster_ability_save_dc` |
| expectation, route A | the ability row's upstream `.lst` bytes | `scripts/derive_monster_ability_save_dc_fixtures.py`, re-checked by an independent reference parser in the test file |
| expectation, route B | the **owning monster's** upstream `MONSTERCLASS:` token, through the printed rule | the same script, re-checked live against the chassis by `universal_monster_rule_save_dc_base` |

A fixture entry is emitted **only when routes A and B agree.** Where they disagree neither side is
known to be right, so the unit is not fixtured and not credited — see the published disagreements
below. The predicate is stated before the run, not chosen after it.

## The linked-ability requirement

PCGen namespaces a monster's own ability rows `<Monster> ~ <Ability>`. The derivation resolves the
owner by splitting that key and finding a row whose `KEY:` is exactly `<Monster>` **in the ability
file's own book directory**. An ability that resolves to no monster row of its own book is an
ORPHAN — a template-namespaced row no monster applies — and there is no racial HD to apply the
printed rule to, so it is excluded. One row fell out here (`orphan_no_owner_monster_row_in_this_book`
= 1).

## The derivation, verbatim

```
export PCGEN_CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data   # pin 7f818006e3
python3 scripts/derive_monster_ability_save_dc_fixtures.py --report
```

```
monster_ability derived+grounded units considered: 264
  no_DC_slot_with_int_plus_stat_argument         146
  EMITTED                                         92
  two_derivations_disagree                        23
  row_states_no_DESC                               2
  orphan_no_owner_monster_row_in_this_book         1
```

## The 23 rows where the two derivations disagree — NOT credited

Every one is in `bestiary_4`. Every other book in the population agrees 100 %. The offsets are not
uniform (`-6` to `+2`), so this is not one systematic transcription rule; it is either printed stat
blocks that deviate from the Universal Monster Rule, or a racial-HD attribution defect confined to
Bestiary 4. **Recorded, not resolved** — no unit moves on it either way.

| unit | `DESC` argument | stated base | owner `MONSTERCLASS` | printed rule gives |
|---|---|---:|---|---:|
| `bestiary_4:monster_ability:boilborn_demon_fever` | `CON+13` | 13 | `Boilborn` `Ooze:2` | 11 |
| `bestiary_4:monster_ability:boilborn_devil_chills` | `CON+9` | 9 | `Boilborn` `Ooze:2` | 11 |
| `bestiary_4:monster_ability:boilborn_leprosy` | `CON+9` | 9 | `Boilborn` `Ooze:2` | 11 |
| `bestiary_4:monster_ability:cephalophore_dazing_gaze` | `15+WIS` | 15 | `Cephalophore` `Construct:12` | 16 |
| `bestiary_4:monster_ability:cephalophore_dazing_strike` | `15+WIS` | 15 | `Cephalophore` `Construct:12` | 16 |
| `bestiary_4:monster_ability:chaneque_steal_soul` | `12+WIS` | 12 | `Chaneque` `Fey:3` | 11 |
| `bestiary_4:monster_ability:fleshdreg_sinful_bite_greed` | `10+CHA` | 10 | `Fleshdreg` `Aberration:2` | 11 |
| `bestiary_4:monster_ability:fleshdreg_sinful_bite_lust` | `10+CHA` | 10 | `Fleshdreg` `Aberration:2` | 11 |
| `bestiary_4:monster_ability:fleshdreg_sinful_bite_pride` | `10+CHA` | 10 | `Fleshdreg` `Aberration:2` | 11 |
| `bestiary_4:monster_ability:fleshdreg_sinful_bite_sloth` | `10+CHA` | 10 | `Fleshdreg` `Aberration:2` | 11 |
| `bestiary_4:monster_ability:fleshwarp_ghonhatine_powerful_stench` | `15+CON` | 15 | `Fleshwarp (Ghonhatine)` `Aberration:12` | 16 |
| `bestiary_4:monster_ability:fleshwarp_ghonhatine_regurgitate` | `15+CON` | 15 | `Fleshwarp (Ghonhatine)` `Aberration:12` | 16 |
| `bestiary_4:monster_ability:fleshwarp_grothlut_digestive_spew` | `11+CON` | 11 | `Fleshwarp (Grothlut)` `Aberration:5` | 12 |
| `bestiary_4:monster_ability:fleshwarp_grothlut_disgusting_demise` | `10+CON` | 10 | `Fleshwarp (Grothlut)` `Aberration:5` | 12 |
| `bestiary_4:monster_ability:fleshwarp_grothlut_piteous_moan` | `11+CON` | 11 | `Fleshwarp (Grothlut)` `Aberration:5` | 12 |
| `bestiary_4:monster_ability:gaki_fear_cone` | `12+CHA` | 12 | `Gaki` `Undead:9` | 14 |
| `bestiary_4:monster_ability:jack_o_lantern_strangling_ensnare` | `CON+13` | 13 | `Jack-o'-Lantern` `Plant:2` | 11 |
| `bestiary_4:monster_ability:lorelei_murmur` | `CHA+15` | 15 | `Lorelei` `Aberration:12` | 16 |
| `bestiary_4:monster_ability:lorelei_vortex` | `CON+21` | 21 | `Lorelei` `Aberration:12` | 16 |
| `bestiary_4:monster_ability:owb_curse_of_darkness` | `13+CON` | 13 | `Owb` `Outsider (Fort/Will):8` | 14 |
| `bestiary_4:monster_ability:rat_king_disease` | `15+CON` | 15 | `Rat King` `Magical Beast:6` | 13 |
| `bestiary_4:monster_ability:saguaroi_needle_cone` | `CON+16` | 16 | `Saguaroi` `Plant:7` | 13 |
| `bestiary_4:monster_ability:shard_slag_excruciating_burn` | `CON+10` | 10 | `Shard Slag` `Ooze:12` | 16 |

(23 rows, one per unit, matching the `two_derivations_disagree` bucket exactly. Verbatim source:
`scripts/derive_monster_ability_save_dc_fixtures.py --report`.)

## Mutation proof — the gate genuinely fails

Two mutations were applied to the real source, the suite re-run, and the source restored
byte-identically (`diff -q` against a pre-mutation copy).

| mutation | applied to | result |
|---|---|---|
| `MonsterAbilitySaveDc { base: base + 1, .. }` | `monster_ability_save_dc` (the evaluator, half 1 of the bar) | **RED — 92 of 92 committed fixtures mismatch**; `the_engine_evaluator_reproduces_every_committed_monster_ability_fixture` and `a_wrong_expected_save_dc_makes_the_bar_check_fail` both FAILED |
| `Some(10 + hd / 3)` | `universal_monster_rule_save_dc_base` (the printed rule, half 2) | **RED — 88 of 92 mismatch**; `a_wrong_universal_monster_rule_base_makes_the_bar_check_fail` FAILED (the 4 survivors are owners with HD ≤ 2, where `hd/2 == hd/3`) |
| — (restored) | — | `test result: ok. 8 passed; 0 failed` |

## What this does NOT claim

**The DC number is verified, and it is still not on a player's screen.** `render_pcgen_desc` DROPS
a `%N` whose argument it cannot resolve to an integer, and `15+WIS` is not an integer, so
`apps/desktop/src-tauri/src/monster_catalog.rs::serve_ability_description` today renders these rows
as *"must succeed at a DC Will save"* — the number missing entirely. The evaluator this cycle adds
computes exactly the value that hole wants, but nothing serves it yet: `PcgenDisplayValues` is an
integer table and cannot express "15 + the creature's Wisdom modifier". Closing that needs a
`save_dc` field on `MonsterAbilityDto`, a frontend render, a reach-gate claim and a DoD-8 on-screen
pass — a bounded follow-on, deliberately not smuggled into this lane's write scope (the seam-monster
lane also holds `monster_catalog.rs` this wave). Logged as `OPEN-ISSUES` row 254.

The `done` credit these 92 units take rests on: they were already `grounded` (a real consumer-delta
observation), and the engine's evaluator now reproduces, over the compiled tables, a value fixed
independently by the printed rule and a second corpus row. It does **not** rest on the DC being
rendered.
