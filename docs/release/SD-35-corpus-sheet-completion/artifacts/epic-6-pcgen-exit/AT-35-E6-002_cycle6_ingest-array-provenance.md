# `raw_bonus_chains` under `src/rules_core/` — the 12 citations, before and after

**Criterion:** `AT-35-E6-002`, cycle 6. **Purpose:** cycle 5 refused to reword these 12
citations because rewording "would move the grep without moving the fact — the euphemism the
residue gate exists to catch". This file is the control that makes the rewrite honest rather
than a euphemism: **every original wording is preserved verbatim here**, so the on-disk array
name a reader would grep for stays one `grep` away, on the tool side of the boundary
(`decisions.md §11`), and nothing was laundered.

**Why the rewrite happened at all, and why no ruling was owed.** Cycle 5 deferred these pending
an `AT-35-E6-004` `--closure` ruling on whether a provenance citation counts against
`live_hits`. The ruling already existed, in the instrument's own module docstring
(`scripts/pcgen_residue_gate.py`, lines 36–40):

> A hit is one regex match; a file counts once however many hits it carries.
> A mention inside a comment or a doc string counts -- the ruling is "nothing
> left of pcgen", and a comment explaining a PCGen token on the live side is
> a sign the code next to it still needs one.

and `--closure` requires `live_files=0 live_hits=0`. Recorded as
`correction 1789086732925-at-35-e6-002-987fc7`.

**What each rewrite does.** It names the **tool-side accessor** that reads the array —
`pcgen_import::ingest_record::bonus_chain_qualifiers` (`src/pcgen_import/ingest_record.rs:68`) —
in place of the array's on-disk field name. That is the re-derive path `AGENTS.md` rule 9 asks
for, one hop more precise than the field name was: a reader runs the function, not a grep. The
live side's own name for the concept is already `DeclaredBonuses` / `declared_bonuses`
(`src/rules_core/race_resolver.rs:418`, renamed by cycle 5's mechanism 2), so the prose now
matches the type beside it.

## The table

| # | file | line (pre-edit) | original wording (verbatim) | now reads |
|---|---|---:|---|---|
| 1 | `src/rules_core/equipment_effects.rs` | 1074 | ``record's own `raw_bonus_chains` (`TOHIT` only, per its own real chain)`` | ``record's own declared bonus chains (`pcgen_import::ingest_record::bonus_chain_qualifiers`; `TOHIT` only, per its own real chain)`` |
| 2 | `src/rules_core/rules_tables/pathfinder_unchained/monk_features.rs` | 1202 | ``(`raw_tokens_excluding_bonus`/`raw_bonus_chains` now read the full `.MOD` closure`` | ``(the generator's token reader and its declared-bonus-chain reader -- `pcgen_import::ingest_record::{token_pairs, bonus_chain_qualifiers}` -- now read the full `.MOD` closure`` |
| 3 | `src/rules_core/equipment_effects/general.rs` | 64 | ``(confirmed against the live corpus: `raw_bonus_chains` is empty on every one)`` | ``(confirmed against the live corpus: every one declares no bonus chains at all -- `pcgen_import::ingest_record::bonus_chain_qualifiers` returns empty)`` |
| 4 | `src/rules_core/equipment_effects/general.rs` | 265 | ``(`raw_bonus_chains` is empty on this record — confirmed against the live corpus)`` | ``(this record declares no bonus chains at all — `pcgen_import::ingest_record::bonus_chain_qualifiers` returns empty, confirmed against the live corpus)`` |
| 5 | `src/rules_core/equipment_effects/equipmods.rs` | 578 | ``heavy_hammer.json`'s `raw_bonus_chains` — a TOHIT-only …`` | ``heavy_hammer.json`'s declared bonus chains (`pcgen_import::ingest_record::bonus_chain_qualifiers`) — a TOHIT-only …`` |
| 6 | `src/rules_core/equipment_effects/arms_armor.rs` | 134 | ``record's own `raw_bonus_chains` for `COMBAT|AC|*|TYPE=Circumstance``` | ``record's own declared bonus chains (`pcgen_import::ingest_record::bonus_chain_qualifiers`) for `COMBAT|AC|*|TYPE=Circumstance``` |
| 7 | `src/rules_core/equipment_effects/arms_armor.rs` | 492 | ``(`raw_bonus_chains` is empty) — its`` | ``(it declares no bonus chains at all — `pcgen_import::ingest_record::bonus_chain_qualifiers` returns empty) — its`` |
| 8 | `src/rules_core/pilot_compute/mod.rs` | 10646 | ``Derived by scanning every ARG alternate's `raw_bonus_chains` against the engine's computed-total surface`` | ``Derived by scanning every ARG alternate's declared bonus chains (`pcgen_import::ingest_record::bonus_chain_qualifiers`) against the engine's computed-total surface`` |
| 9 | `src/rules_core/pilot_compute/mod.rs` | 10667 | ``rescans every alternate's `raw_bonus_chains` against the engine's computed-total surface`` | ``rescans every alternate's declared bonus chains (`pcgen_import::ingest_record::bonus_chain_qualifiers`) against the engine's computed-total surface`` |
| 10 | `src/rules_core/pilot_compute/mod.rs` | 12196 | ``ooze_breath}.json`'s `raw_bonus_chains`) at fold time`` | ``ooze_breath}.json`'s declared bonus chains, read by `pcgen_import::ingest_record::bonus_chain_qualifiers`) at fold time`` |
| 11 | `src/rules_core/pilot_compute/mod.rs` | 13117 | ``scanning all 153 alternates' `raw_bonus_chains` against the engine's own computed-total surface`` | ``scanning all 153 alternates' declared bonus chains (`pcgen_import::ingest_record::bonus_chain_qualifiers`) against the engine's own computed-total surface`` |
| **12** | `src/rules_core/pilot_compute/mod.rs` | 11694 | **not a comment — see below** | **see below** |

## Row 12 is not a comment, and cycle 5's table was wrong about it

Cycle 5 wrote that the remainder was "12 hits, 6 files, and every one of them a comment".
**Eleven were.** The twelfth sits inside a `format!` that builds
`ComputationExplanation { id: "race.rougarou.trait_bundle.natural_weapon", … }.detail` — the
**rendered sheet line a player reads**. It read:

```
(rougarou_abilities_race.lst:20 ABILITY:Internal|AUTOMATIC|Bite,
 raw_bonus_chains WEAPONPROF=Bite/DAMAGESIZE -1), a secondary attack if the …
```

so our own ingest array name *and* a PCGen qualifier chain were being printed onto the sheet.
`decisions.md §1`: the sheet prints one final number or the rule's words. `DAMAGESIZE|-1` on a
bite means the damage die steps down one size, which is exactly how the record's `1d4` is
reached from a Medium bite's `1d6` — so the words were available and the token was never
needed. It now reads:

```
(rougarou_abilities_race.lst:20 ABILITY:Internal|AUTOMATIC|Bite,
 with the bite's damage die stepped down one size), a secondary attack if the …
```

The value is unchanged (`value: ROUGAROU_BITE_DAMAGE_DIE` = 4) and
`rougarou_gets_speed_senses_and_natural_weapon_explanations` still asserts
`detail.contains("1d4")`. Recorded as `correction 1789086743344-at-35-e6-002-becdad`.

**What this rewrite does not cover** (`AGENTS.md` rule 7). The `.lst` source citation and the
`ABILITY:Internal|AUTOMATIC|Bite` token beside it are still PCGen source syntax in a shipped
detail string, and so are the sibling records' `MOVE:Walk,30` and
`ABILITY:Special Ability|AUTOMATIC|…`. That population is **PCGen source-file syntax in live
prose**, a different and much larger mechanism than this criterion's ingest-array clause — the
residue gate counts it under its `BONUS:` / `DESC:` / `TYPE=` token-syntax patterns
(`src/rules_core` `hits=11414` at this tree), and it is `AT-35-E6-004`'s closure surface, not
this criterion's. Naming it here so it is not mistaken for closed.

## Re-derive

```
grep -rho '\braw_bonus_chains\b' --include=*.rs src/rules_core/ | wc -l     # 0  (was 12)
grep -rl  '\braw_bonus_chains\b' --include=*.rs src/rules_core/ | wc -l     # 0  (was 6)
grep -rho '\braw_tokens\b'       --include=*.rs src/rules_core/ | wc -l     # 0  (unchanged, cycle 4)
python3 scripts/pcgen_residue_gate.py --check                               # identifier_hits 162 -> 150
```

Denominator for every figure above: the **196** live `src/rules_core` source files the residue
gate scans (`python3 scripts/pcgen_residue_gate.py --check`, `root src/rules_core files=`).
