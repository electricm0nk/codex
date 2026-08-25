# Cycle AT-33-E5-last67-weapon — Epic 5 Re-verification / AT-33-E5-002, AT-33-E5-003

- **Commit SHA:** `9b54e79366` (progress.md/kanban.md update landed separately, see `progress.md` entry `AT-33-E5-last67-weapon`)
- **Files touched:**
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/last67-weapon.oracle-results.json` (new — this lane's committed deliverable, 14 rows)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-last67-weapon_cycle_receipt.md` (this file)
  - `src/bin/e5_last67_weapon_ours.rs` (new — real "ours" probe, extends the `e5_*_ours.rs` family, calls `compute_equipment_effects` live, never hand-typed)
  - `docs/release/SD-33-computed-value-verification/progress.md` / `kanban.md` (updated in place, next commit)
  - `docs/retro/events/sd33-r5-weapon.jsonl` (new — 3 incidents, 1 correction, 2 deferrals)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (0 matches, `git diff --unified=0 f53b8e32da2ae1939b9ddb1b8375ba1baefd00ba...HEAD -- src/bin/e5_last67_weapon_ours.rs docs/release/.../last67-weapon.oracle-results.json scripts/oracle_harness/`)
- **Wired-integration audit result:** OK_NO_TOKENS (0 matches, same diff)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E5-002 — the 6,589 `literal-verified` units are re-examined
  >
  > **Evidence:** per-unit `(ours, oracle, verdict)` rows committed; agreement and disagreement
  > counts both stated, with the denominator.
  >
  > ### AT-33-E5-003 — every disagreement is a named defect, fixed or escalated
  >
  > A disagreement is never closed by adjusting the expectation to match our output. Each is
  > root-caused: either our computation is wrong (fix it) or the oracle comparison is wrong (fix
  > the harness, and re-run everything it already judged).

## Population re-derivation (first action, per the brief)

```
$ python3 -c "import json
wi=json.load(open('docs/work-inventory.json'))['units']
pop={u['id'] for u in wi if u.get('status') in ('literal-verified','fixture-verified')}
d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json'))['results']
miss=sorted(pop-{r['unit_id'] for r in d})
print(len(miss)); [print(m) for m in miss]"
67
<67 ids listed>
```

Matches the brief's stated 67. This lane's population (37 of the 67) was derived by reading
every one of the 67 units' full `raw_bonus_chains` (not a filtered view) and classifying by
mechanism. Full classification script output and per-unit chains are in this cycle's working
notes; the resulting 37-id list matches wave 4's own per-shape counts (24 + 6 + 4 + 3 = 37)
exactly:

```
$ python3 -c "
mine = [...37 ids...]
allids = [...67 ids from above...]
print('mine:', len(mine), 'unique:', len(set(mine)))
notmine = [x for x in allids if x not in set(mine)]
print('not mine:', len(notmine))
print('mine-not-in-all:', [x for x in mine if x not in allids])"
mine: 37 unique: 37
not mine: 30
mine-not-in-all: []
```
37 + 30 = 67 — every one of the 67 remaining accounted for between this lane and its three
siblings, none double-counted, none dropped.

## Shape table (all 37, by mechanism)

| Shape | Population | Examined this cycle | Verdicts this cycle |
|---|---:|---:|---|
| `WEAPONPROF=<x>` / `WEAPON` enhancement family, `compute_equipmods_effect`-covered | 24 | 11 | agree: 9, disagree: 2 |
| bare `WEAPON\|TOHIT,DAMAGE,ATTACKS`, no `TYPE=` qualifier | 6 | 2 | unverifiable: 2 |
| `WEAPON\|DAMAGEMULT` fractional crit-multiplier | 4 | 1 | unverifiable: 1 |
| wield-size `WIELDCATEGORY` + bare `WEAPON\|TOHIT` (no-penalty variants) | 3 | 0 | — blocked, see Finding 5 |
| **Total** | **37** | **14** | **agree 9, disagree 2, unverifiable 3** |

## The 14 examined rows

Full rows (ours, oracle, verdict, reason/note) are in
`last67-weapon.oracle-results.json`. Summary:

| unit_id | ours | oracle | verdict |
|---|---:|---:|---|
| `core_rulebook:equipment:rod_flailing` | 3 | 3 | agree |
| `core_rulebook:equipment:rod_python` | 1 | 1 | agree |
| `core_rulebook:equipment:rod_thunder_and_lightning` | 1 | 1 | agree |
| `core_rulebook:equipment:rod_viper` | 2 | 2 | agree |
| `core_rulebook:equipment:mattock_of_the_titans` | 3 | 3 | agree |
| `advanced_race_guide:equipment:claw_blades_catfolk` | 1 | 1 | agree |
| `advanced_race_guide:equipment:rending_claw_blades` | 0 | 1 | **disagree** |
| `ultimate_equipment:equipment:berserking_sword` | 2 | 2 | agree |
| `ultimate_equipment:equipment:cursed_backbiter_spear` | 2 | 2 | agree |
| `ultimate_equipment:equipment:heavy_hammer` | 0 | 4 | **disagree** |
| `ultimate_equipment:equipment:ornery_pistol` | 2 | 2 | agree |
| `advanced_players_guide:equipment:crossbow_double` | null | -4 | unverifiable |
| `core_rulebook:equipment:rod_withering` | null | 1 | unverifiable |
| `advanced_players_guide:equipment:sword_cane` | null | null | unverifiable |

Re-derive:
```
$ python3 -c "import json,collections
d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/last67-weapon.oracle-results.json'))['results']
print('rows', len(d)); print(collections.Counter(x['verdict'] for x in d))
print('reasonless', len([x for x in d if x['verdict']=='unverifiable' and not (x.get('reason') or '').strip()]))
ids=[x['unit_id'] for x in d]; print('dupes', len(ids)-len(set(ids)))"
rows 14
Counter({'agree': 9, 'unverifiable': 3, 'disagree': 2})
reasonless 0
dupes 0

$ python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/last67-weapon.oracle-results.json
uncovered=0 overlap=0 population=49438 oracle_disagreement=2 unverifiable_done=0 stale=False
ORACLE_DISAGREEMENT: advanced_race_guide:equipment:rending_claw_blades, ultimate_equipment:equipment:heavy_hammer
```

## Method — real "ours", real oracle, no hand-rolling

**"ours"**: new `src/bin/e5_last67_weapon_ours.rs`, one process, one corpus load, real live
calls into `codex::rules_core::equipment_effects::compute_equipment_effects` for every one of
the 37 units (all 37, not just the 24 shape-A ones — the probe's own output confirms `null` for
every shape-B/C/D unit, proving `compute_equipmods_effect`'s deliberate exclusion of
`TYPE=Enhancement`-less bare `WEAPON` chains and `WIELDCATEGORY`/`DAMAGEMULT` chains empirically,
not from re-reading the module doc alone):
```
$ cargo run --locked --bin e5_last67_weapon_ours -- <repo_root> weapon-manifest.json weapon-ours-output.json
e5_last67_weapon_ours: 37 units in manifest, 37 resolved, 0 unresolved -> weapon-ours-output.json
```

**oracle**: `scripts/oracle_harness/charbuild_remainder_run_one.sh` (unmodified, reused) against
a hand-built multi-weapon `.pcg` fixture — one Level-1 Human Fighter wearing/wielding up to 17
of this lane's items simultaneously via PCGen's generic `Equipped` `EQUIPSET` location (proven
this cycle: `getExpandedWeapons`/`getEquipmentOfType("Weapon",3)` do NOT require a hand-specific
location, so N items batch into one export — a real amortization improvement over
`AT-33-E5-last75`'s one-fixture-per-item precedent), querying `WEAPON.n.MAGICHIT` /
`WEAPON.n.MAGICDAMAGE` / `WEAPON.n.MULT` per weapon in one `BatchExporter` run. All live, against
the pinned oracle (`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`, resolved via
`$PCGEN_REPO_DIR`, never a literal path in this doc). Per-unit cost after fixture setup: well
under a minute; the dominant cost was diagnosing three real campaign/name issues below, not the
JVM.

## Finding 1 — `AT-33-E5-last75`'s open MAGICHIT-sign question, resolved

Wave 4 recorded `mattock_of_the_titans`'s `WEAPON.0.MAGICHIT=-3` (expected `+3`) as unresolved.
This cycle ran a size-matched control in the SAME fixture, SAME run: `Rod (Flailing)`
(`SIZE:M`, `BONUS:WEAPON|DAMAGE,TOHIT|3|TYPE=Enhancement` — numerically identical bonus to
Mattock's) shows `MAGICHIT=+3` cleanly. Mattock (`SIZE:G`, wielded by a Medium fixture) shows
`MAGICHIT=-3`: PCGen's `WeaponToken.getMagicHitToken` sums `eq.getBonusToHit` (`bonusTo(pc,
"WEAPON","TOHIT",true)`, ALL `WEAPON|TOHIT` bonuses regardless of `TYPE=`) plus the
`WEAPONPROF=TYPE.*` bucket — an oversized weapon's own to-hit penalty is folded into that same
untyped `WEAPON|TOHIT` bucket PCGen computes internally (not a token our corpus record carries
at all), netting `3 - 6 = -3`. **Verdict: MAGICHIT is a valid, reliable comparable token for a
normally-sized weapon; it is confounded (not wrong, and not a defect in either side) for a
size-mismatched one.** Recorded via `scripts/retro.py correction`. `mattock_of_the_titans`'s row
in this cycle's results compares `DAMAGE` only (the clean dimension), with TOHIT's exclusion
stated in the row's own `note` field.

## Finding 2 — `heavy_hammer`: real `compute_equipmods_effect` defect, single-chain limitation

`heavy_hammer`'s own corpus record carries THREE separate `raw_bonus_chains`
(`BONUS:MOVEADD|TYPE.All|-10`, `BONUS:WEAPONPROF=Warhammer|TOHIT|-2`,
`BONUS:WEAPONPROF=Warhammer|DAMAGE|4`) — all present in `data/corpus/`. This cycle's real probe
(`e5_last67_weapon_ours`, a live call into `compute_equipment_effects`) returns `bonus=-2
affects=TOHIT` only: the second `WEAPONPROF=Warhammer|DAMAGE|4` chain never reaches
`WeaponEnhancementBonus` at all, matching `compute_equipmods_effect`'s own module doc comment
("reads the record's **first** ... chain, if any"). Live oracle confirms the real item DOES
grant both: `MAGICHIT=-2` (agrees), `MAGICDAMAGE=+4` (our engine has no value for this at all —
a genuine player-facing gap: this item's damage bonus would never render in the desktop app).
Recorded as a real `disagree`, and as `scripts/retro.py incident`
(`equipmods-single-chain-per-record`) — **not fixed this cycle**: a same-record multi-chain-
summing change to `compute_equipmods_effect` needs its own RED→GREEN and a regression check
against every other examined unit in this lane (several of which rely on the current
first-chain-wins behavior not changing their own single-chain answer).

## Finding 3 — `rending_claw_blades`: real corpus `.MOD`/`EQMOD`-merge gap, different root cause

Live oracle: `MAGICHIT=+1` (agrees with ours), `MAGICDAMAGE=+1` (disagrees — ours has no
DAMAGE-affecting chain at all; the record's own `raw_bonus_chains` is a single
`WEAPON|TOHIT|1|TYPE=Enhancement`). Root-caused against the pinned PCGen source directly
(`advanced_race_guide/arg_equip_arms_armor.lst`): the live record is defined via `Rending Claw
Blades.MOD ... EQMOD:Special Ability ~ Keen ~ Weapon.Special Ability ~ +1 ~ Weapon.Material ~
Steel` — a `.MOD` line attaching the canonical "+1 Weapon" enhancement equipmod (the same
`BONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement` quality examined directly in
`equipmods.rs`'s own unit tests) plus Keen. `data/corpus/advanced_race_guide/equipment/
rending_claw_blades.json`'s `raw_tokens` carries only `EQMOD: 'Material ~ Steel'` — the
`.MOD`-attached EQMOD entries were never captured by the corpus extraction pipeline for this
`.MOD`-defined record. **This is a corpus-generation gap, not a `compute_equipmods_effect`
defect** — a perfect resolver reading this exact JSON would still compute `DAMAGE=0`, because
the source JSON has no DAMAGE-affecting token to read. `data/corpus/**` is out of this lane's
write scope (guarded-generator-path-only, never hand-edited). Recorded as a real `disagree` and
as `scripts/retro.py incident` (`corpus-mod-eqmod-merge-gap`).

## Finding 4 — two real, reproducible oracle-harness campaign-load gaps

**Advanced Class Guide** fails to load standalone against the pinned checkout, even combined
with Core Rulebook/Advanced Race Guide/Advanced Player's Guide/Ultimate Equipment:
`java.lang.IllegalStateException: Cannot ask for resolution: Reference Prodigy (%LIST) has not
been resolved` (`SourceFileLoader:499`). Blocks all 5 ACG-sourced units in this lane's
population (4 rapiers, `brawler_s_flurry`).

**Ultimate Psionics** is authored for a DIFFERENT PCGen gamemode
(`GAMEMODE:Pathfinder` in `ultimate_psionics.pcc`, not `Pathfinder_RPG`) — `Could not find
campaign: Ultimate Psionics` aborts the whole character load when combined with a
`Pathfinder_RPG` fixture. Blocks `flurry_of_fists`/`flurry_of_strikes`. Consistent with, and
independently confirming, `AT-33-E5-last75`'s own separately-diagnosed `ultimate_psionics`
harness gap (its Finding 2 — a live-in-campaign SKILL-bonus defect once the campaign DOES load;
this cycle's gap is one level earlier, the campaign not loading at all in this gamemode
combination).

Both recorded via `scripts/retro.py incident` (`oracle-harness-campaign-load-failure`). Neither
fixed this cycle — real, structural, next-cycle scope.

## Finding 5 — wield-size equipmod attachment not proven live this cycle

The 3 `special_quality_wield_size_*_no_penalty` units are equipment **modifiers**
(`BONUS:WEAPON|WIELDCATEGORY|-n` plus a bare `WEAPON|TOHIT|2n`), attached to a base weapon via
`CUSTOMIZATION:[BASEITEM:<weapon>|DATA:EQMOD=<key>]` — not standalone weapons like this lane's
other 34 units. This cycle did not prove a live attachment round-trip for a Special-Quality-
category eqmod (as opposed to the simple `EQMOD=STEEL` material shorthand already proven in
prior waves) within its time budget. `WIELDCATEGORY` is already ruled `unverifiable`/non-scalar
per `AT-33-E5-last75`; the bare `TOHIT` half (the comparable magnitude for these 3, per this
lane's brief) needs that attachment syntax proven first. Deferred, not fixed.

## Verdict discipline

Every `agree`/`disagree` row's `ours` value came from a real, mechanical call into
`compute_equipment_effects` (`e5_last67_weapon_ours`), never hand-typed. `heavy_hammer` and
`rending_claw_blades` are recorded as `disagree` honestly (doctrine: "a real disagreement is a
find, not a failure") even though both trace to real, well-evidenced defects outside this
lane's own write scope to fix this cycle. `sword_cane`'s `DAMAGEMULT` fractional value is
recorded `unverifiable` rather than truncated to an integer (truncating -0.5 to 0 or -1 would
manufacture a false `agree`/`disagree` — doctrine explicitly forbids this). `crossbow_double`
and `rod_withering`'s bare-`WEAPON`-chain-no-resolver units are `unverifiable` (not a fabricated
literal "ours") with the real oracle value recorded for the next cycle that builds the resolver.

## Status: blocked-escalated

**Not `complete`.** 14 of this lane's 37-unit population are genuinely examined with real,
per-unit `(ours, oracle, verdict)` rows and populated reasons on every `unverifiable` row (2
real, well-evidenced `disagree`s among them). The remaining 23 are named per-shape above with
concrete structural reasons (two distinct campaign-load harness gaps, one ammunition-launcher
pairing gap, one unproven equipmod-attachment syntax, one natural-attack-fixture gap covering 13
units) and a concrete next-cycle plan — not "ran out of time" vaguely.

## Movement, four buckets

- **Closure:** 14 units of this lane's 37-unit population get a real, committed oracle
  disposition for the first time (9 agree, 2 disagree, 3 unverifiable, each reasoned).
- **Reclassification:** none — no unit's `docs/work-inventory.json` `status` field changed.
- **Reachability:** confirmed 24 of 37 units' shape (`WEAPONPROF`/`WEAPON` enhancement family)
  needs zero new `src/rules_core/` code for the units it CAN reach (11 of 24 examined this
  cycle); the multi-weapon-per-export batching technique (generic `Equipped` location, no hand
  assignment needed) is a real forward movement on reachability for every future weapon-shape
  lane, amortizing JVM startup across up to 17 items per run this cycle (vs. 1 per run in prior
  waves).
- **Instrument-correction:** 1 (`scripts/retro.py correction`) — `AT-33-E5-last75`'s open
  MAGICHIT-sign question resolved (Finding 1). Plus 3 new incidents recorded (Findings 2, 3, 4)
  naming real, reproducible defects/gaps for next-cycle remediation, none of which was silently
  smoothed over into a false `agree`.

## Notes

Judgment calls made explicit per the brief:
- **Comparable magnitude for the 24-unit `WEAPONPROF`/`WEAPON` family:** the `DAMAGE` half of
  the chain when the chain grants one (every examined unit's `TOHIT` and `DAMAGE` halves are
  numerically identical except `heavy_hammer` and `rending_claw_blades`, both `disagree`s where
  the DAMAGE half is exactly where the defect lives — DAMAGE is representative and avoids
  `mattock_of_the_titans`'s TOHIT size-confound); `TOHIT` alone when the chain grants no DAMAGE
  component (`claw_blades_catfolk`).
- **Bare `WEAPON` chains, no `TYPE=`:** `TOHIT` is the single comparable magnitude (the only
  roll these chains ever affect in this lane's population) — recorded `unverifiable: no_resolver`
  rather than fabricating an "ours" the engine does not compute, per doctrine.
- **`DAMAGEMULT`:** no PCGen export token isolates a fractional per-attack multiplier delta
  (`WEAPON.n.MULT` returns the weapon's base integer multiplier, confirmed unchanged live on
  `sword_cane`) — `unverifiable`, not truncated.
- **Wield-size no-penalty variants:** the bare `TOHIT` half IS the comparable magnitude per the
  brief (the `WIELDCATEGORY` half stays `unverifiable`/non-scalar per `AT-33-E5-last75`), but no
  live value was obtained this cycle (Finding 5) — deferred, not fabricated.

## RED→GREEN

`src/bin/e5_last67_weapon_ours.rs` is new production tooling (a batch probe, not a
`src/rules_core/` behavior change) — RED: before this cycle, no repo-local binary computed
`weapon_enhancement_bonus` for this population in batch (prior lanes hand-typed "ours" per
`AT-33-E5-last75`'s own precedent for its 8 rows). GREEN: `cargo run --locked --bin
e5_last67_weapon_ours` resolves all 37 units, 0 unresolved, backed by the SAME real
`compute_equipment_effects` function the shipped engine and `AT-33-E1-003`'s wiring probe both
call — confirmed by a clean `cargo build --locked --bin e5_last67_weapon_ours` and a real run
producing `weapon-ours-output.json` with the exact per-unit values this receipt's tables cite.
No `src/rules_core/` production behavior changed this cycle (Findings 2/3/4/5 are real defects
named for their own future RED→GREEN cycles, not fixed here — fixing `compute_equipmods_effect`'s
single-chain limitation needs a regression check this cycle's time budget did not allow).

## Test scoping

Ran `cargo build --locked --bin e5_last67_weapon_ours` (clean, only pre-existing unrelated
warnings elsewhere in the crate) and `cargo run --locked --bin e5_last67_weapon_ours -- ...`
(37/37 resolved). Ran `bash scripts/oracle_harness/charbuild_remainder_run_one.sh` (unmodified,
reused) against hand-built `.pcg`/`.ftl` fixtures in a scratch directory (not committed — no
new fixture files under `scripts/oracle_harness/` or `artifacts/` this cycle beyond the results
JSON and this receipt; the fixtures themselves are reproducible from this receipt's own Method
section and are not this lane's own deliverable per its write scope). Ran
`python3 scripts/box_ledger.py --check --oracle-results .../last67-weapon.oracle-results.json`
(shown above). **Did not** run the root `cargo test` sweep or `apps/desktop/src-tauri` — no
`src/rules_core/` file changed this cycle (new `src/bin/` probe only, no library behavior
change); the probe's own build+run is the real, live verification of its correctness (it calls
the existing, already-tested `compute_equipment_effects`/`compute_equipmods_effect` functions
directly, adding no new logic of its own beyond manifest I/O).

## Next-cycle plan

1. Fix `compute_equipmods_effect`'s single-chain-per-record limitation (Finding 2) with its own
   RED→GREEN and a regression check against every unit this lane and its siblings have already
   examined — unblocks `heavy_hammer` moving from `disagree` to `agree` (once the fix is proven
   correct, not merely non-regressing).
2. Root-cause the corpus generator's `.MOD`/`EQMOD`-merge gap (Finding 3) for
   `rending_claw_blades` and any sibling `.MOD`-defined equipment records across the corpus —
   likely affects more than this one unit.
3. Isolate the Advanced Class Guide standalone-load failure (likely a missing prerequisite
   splatbook) and the Ultimate Psionics gamemode split (Finding 4) — unblocks 7 units in this
   lane alone (5 ACG + 2 psionics) plus whatever `AT-33-E5-last75`'s own remaining
   `ultimate_psionics` population still needs.
4. Build a launcher+ammunition fixture pair for `heartstake_bolts_5`.
5. Prove one live `CUSTOMIZATION:[BASEITEM:<weapon>|DATA:EQMOD=<Special Quality key>]` attachment
   round-trip — unblocks the 3 wield-size-no-penalty units' comparable `TOHIT` half (Finding 5).
6. Build a natural-attack-bearing fixture (race/template granting Bite/Claw/Hoof, or confirm the
   exact campaign combination that gives CRB's `Unarmed Strike` prof `TYPE:Natural`) and resolve
   `cursed_sword_2`'s EQUIPSET-VALUE parenthesis lookup failure — unblocks the remaining 13 units
   (5 amulets, `belt_of_teeth`, 5 horseshoes, `talons_of_leng`, `cursed_sword_2`).
7. Re-run `AT-33-E6-001` as the next attempt once this lane's siblings land too — population will
   still be short of 8,330 by up to 23 units (this lane's own remainder) plus whatever the
   sibling lanes leave, unless a further cycle closes them.
