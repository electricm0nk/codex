# Cycle — SD-34 wave 37, Lane B — Sentinel Kind-retag hypothesis refuted (no code change); sub-mechanism 5's stale 832-unit figure re-derived to 634

- **Commit SHA:** (this receipt's own commit, see report)
- **Files touched:** this receipt, `progress.md`, `kanban.md`,
  `docs/retro/events/sd34-wave37-laneb.jsonl` (new),
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
  (`derived_at` pointer refresh only, from running `completion_atlas.py --check`
  at this cycle's own HEAD — no bucket data changed). **No `src/`, `scripts/`,
  or `data/corpus/**` file touched.**
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — no code diff this cycle,
  N/A by construction.
- **Wired-integration audit result:** `OK_NO_TOKENS` — no code diff this
  cycle, N/A by construction.
- **Acceptance criterion (verbatim from this cycle's dispatch brief):** two
  independent items from wave 36 lane C's own "Next-cycle plan" — (1) item 4,
  Sentinel (1 unit): "the corpus record ... needs an ingest-time `Kind::feat`
  re-tag ... find the exact record, and fix the ingest-time classification
  ... confirm you are editing the right one before touching anything"; (2)
  item 2: re-derive sub-mechanism 5's corrected 832-unit/60-class population
  post-regen, "same Counter method wave 35 lane C used originally", read-only.

## Item 1 (Sentinel, 1 unit) — the requested fix is factually wrong; NOT made

**Investigated first, per the brief's own instruction to confirm before
touching anything. The investigation refutes the premise.**

The unit (`ultimate_intrigue:class_feature:sentinel_style_feat_improved_sense_intruder`,
`corpus_key: "Sentinel Style Feat ~ Improved Sense Intruder"`) is currently
`status: "engine-does-not-hold"`, `evidence:
"class_feature_of_unmodelled_corpus_class:sentinel"`
(`docs/work-inventory.json`, this cycle's own HEAD).

**Both wave 35 lane C's original finding and wave 36 lane C's carried-forward
restatement are wrong about what this record is.** Direct reads:

1. `data/corpus/ultimate_intrigue/class_feature/sentinel_style_feat/improved_sense_intruder.json`
   — `CATEGORY:Special Ability`, `TYPE:RangerBonusFeat`, `class: "Sentinel"`,
   `PREVARGTEQ:Sentinel_Style_Feat_Improved_Sense_Intruder,1`,
   `PREVARGTEQ:CombatStyleLVL,10`, `BONUS:VAR|SentinelSenseIntruderRange|10`.
2. `data/corpus/ultimate_intrigue/class_feature/sentinel/sense_intruder.json`
   (`"Sentinel ~ Sense Intruder"`, `type_facet: "Ranger Class
   Features.SentinelClassFeatures..."`) — the BASE ability granted by the
   **Ranger's "Sentinel" archetype** (`ranger_archetype/sentinel.json`
   confirms `class: "Ranger"`, `TYPE:Archetype.RangerArchetype`). Its own
   `DESC:` states verbatim: *"Starting at 10th level, whenever the sentinel
   gains a combat style feat, she can choose to trade it for a 10-foot
   increase in the radius of this ability."* — i.e. "Improved Sense Intruder"
   IS the Ranger's own 10th-level combat-style-feat SLOT, spent on this
   option instead of a normal bonus feat. It is not a Vigilante talent (wave
   36's own receipt calls it "the base Vigilante talent" — also wrong; there
   is no Vigilante involved anywhere in this chain).
3. `data/corpus/advanced_players_guide/class_feature/ranger_combat_style_feat/vital_strike.json`
   — **wave 35's own cited analogy** ("a combat STYLE FEAT chain, like Vital
   Strike") — is itself filed as `Kind::ClassFeature`
   (`class_feature/ranger_combat_style_feat/vital_strike.json`), not
   `Kind::Feat`, with the byte-identical shape (`CATEGORY:Special Ability`,
   `TYPE:RangerBonusFeat`, `class: "Ranger"`, a `PREVARGTEQ` gate). ~180
   sibling records share this exact pattern across 6 books (`grep -rl
   RangerBonusFeat data/corpus/*/class_feature/`, all under `Kind::ClassFeature`,
   zero under `Kind::Feat` — `grep -rl RangerBonusFeat data/corpus/*/feat/
   data/corpus/*/feat_generic/` returns nothing). **The corpus's own
   established, consistent convention is that a class's grant of a named
   feat via a bonus-feat-chain slot is `Kind::ClassFeature`, never
   `Kind::Feat`** — wave 35/36's "not a class feature at all" claim
   contradicts the very analogy it was built on.

**Real cause of the D-bucket stall, traced to the actual matcher code**
(`class_feature_owner`, `v06_work_inventory.rs:9491`; the "unmodelled
corpus class" fallback, `:11951-11969`): the group text `"sentinel style
feat"` collides with a real, genuinely-unmodelled corpus `Kind::Class`
record literally named `"Sentinel"` (`inner_sea_gods:class:sentinel`, `TYPE:
PC.Prestige`, itself `status: "engine-does-not-hold"`) — the **same
short-word-collision shape as "Order of the Dragon"** (wave 36 lane C's own
case-(a) fix), not a `Kind` question at all.

**Even a correct matcher fix would not close this unit.** Two independent
reasons, both checked, not assumed:
- `magnitude_token_count: 1` (the `BONUS:VAR|SentinelSenseIntruderRange|10`
  token) — **not** zero-magnitude, so the "text-only = complete" promotion
  path wave 36 lane C used for Order of the Dragon does not apply here.
- The underlying mechanism (Ranger's own combat-style bonus-feat chain) is
  itself unbuilt engine-wide: `Counter` over `docs/work-inventory.json`'s
  `units` where `kind == "class_feature"` and `"ranger_combat_style_feat" in
  id` → `{'engine-does-not-hold': 151, 'deferred-with-reason': 29}`, **0
  `DONE`**, evidence `class_feature_owner_matched_by_name_but_record_not_held_by_engine`
  on all 151 (the matcher already correctly resolves "Ranger" as owner for
  every one of these 151 siblings — proof the owner-resolution path itself
  works fine when it isn't collision-blocked — and the engine still doesn't
  hold any of them). Re-attributing Sentinel's owner from the collision to
  "Ranger" would land it in the exact same D-bucket boat as its 151
  siblings, evidence string only, zero bucket movement.

**Disposition: genuinely unbuilt (case b: real mechanism gap — Ranger's
10th-level combat-style-feat-chain choice, currently misattributed to a
name collision rather than to Ranger, but unbuildable either way this
cycle since the chassis itself is Epic 4/5 scope), not a `Kind`
misclassification.** No ingest-time change made — `data/corpus/**` is
guarded-generator-path-only (`workflow-instruction.md §6`), and the
requested change would have written a wrong classification onto a
correctly-classified record. **Units closed this item: 0** — reported
honestly, not stubbed.

**Retro-logged correction** (event id `1788442730755-sd34-wave37-laneb-7cdc91`,
`docs/retro/events/sd34-wave37-laneb.jsonl`): subject `wave35_laneC_reconnaissance_receipt
/ wave36_laneC_creature_type_collision_disposition_cycle_receipt`, claimed
"Sentinel Style Feat ~ Improved Sense Intruder is a feat chain, not a class
feature at all; needs an ingest-time Kind::feat re-tag", actual "it IS a
genuine class_feature, same short-word-collision shape as Order of the
Dragon, verified against 4 corpus records + the 151-sibling status Counter",
`caught-before: implementation` (caught before the wrong ingest-time change
was made, per the brief's own "confirm before touching anything"
instruction).

## Item 2 (figure re-derivation) — sub-mechanism 5's 832/60 population is now 634/60

Read-only. Re-ran wave 35 lane C's own exact `Counter` method (grepped from
`wave35_laneC_reconnaissance_cycle_receipt.md`) against the **current
committed** `docs/work-inventory.json`, after rebasing this cycle's worktree
onto the real live `tranche/14` tip (see "Worktree base note" below):

```python
collections.Counter(u["evidence"].split(":", 1)[1] for u in units
                     if (u.get("evidence") or "").startswith(
                         "class_feature_of_unmodelled_corpus_class:"))
```

`701` units across `68` distinct classes total (down from wave 35's `931`/`70`).
Excluding the 10 classes already pulled into sub-mechanisms 1–4
(`psychic_warrior`, `rogue`, `animal`, `undead`, `dragon`, `construct`,
`plant`, `ooze`, `eidolon`, `sentinel` — same exclusion set wave 35 used):

**Sub-mechanism 5, corrected: `634` units across the SAME `60` classes**
(no class dropped out, no new class appeared — set-equal to wave 35's own
60-class roster, confirmed by direct set difference). `634` vs. wave 35's
`832` = **`198` units closed** since the original census, entirely
attributable to wave 36 lane A + lane C's matcher fixes composed on the
merged tree (`D: 2891→2662` overall this cycle's own fresh
`completion_atlas.py --check`, matching the wave-36-wave-end-gate's own
reported final state exactly).

**Honesty note on a small reconciliation gap:** wave 36 lane C's own receipt
claimed "202 of the 215 [closures] are outside this cycle's own 80-unit
scope, from sub-mechanism 5's own 60-class population." This cycle's fresh
re-derivation finds `198`, not `202` — a 4-unit gap against that receipt's
own figure. Not investigated further (out of this read-only item's scope);
flagged for whoever dispatches sub-mechanism 5 next in case it matters at
the per-class level. The `634`/`60` figure itself is directly reproducible
by the command above at this cycle's own committed HEAD and is what this
receipt certifies as current.

### Corrected per-class breakdown (634 units, 60 classes, largest-yield-first)

| Class | Units | Class | Units | Class | Units |
|---|---:|---|---:|---|---:|
| divine_scion | 45 | steel_falcon | 12 | stalwart_defender | 6 |
| phrenic_slayer | 43 | psicrystal_imprinter | 12 | mammoth_rider | 6 |
| sighted_seeker | 21 | lantern_bearer | 11 | pathfinder_delver | 6 |
| thrallherd | 19 | magaambyan_arcanist | 11 | demoniac | 6 |
| psychic_detective | 18 | storm_kindler | 11 | master_chymist | 5 |
| cyphermage | 17 | westcrown_devil | 11 | enchanting_courtesan | 5 |
| twilight_talon | 17 | pyrokineticist | 11 | dark_tempest | 5 |
| golden_legionnaire | 16 | metamind | 10 | battle_herald | 4 |
| phantom | 16 | aspis_agent | 9 | master_spy | 4 |
| psychic_fist | 16 | gray_corsair | 9 | pure_legion_enforcer | 4 |
| asavir | 15 | pathfinder_savant | 9 | evangelist | 4 |
| metamorph | 15 | rivethun_emissary | 9 | metaforge | 4 |
| war_mind | 15 | cerebremancer | 9 | soul_archer | 4 |
| hellknight | 14 | telekinetic_weaponmaster | 9 | horizon_walker | 3 |
| adaptive_warrior | 14 | student_of_war | 8 | nature_warden | 3 |
| aldori_swordlord | 13 | diabolist | 8 | rage_prophet | 3 |
| sanguine_angel | 13 | lion_blade | 8 | gifted_blade | 3 |
| body_snatcher | 13 | bellflower_tiller | 7 | holy_vindicator | 2 |
| elocater | 13 | hellknight_signifer | 7 | argent_dramaturge | 2 |
| psion_uncarnate | 13 | mystic_archer | 7 | ulfen_guard | 1 |

Sum check: `634` — re-derive with the exact `Counter` command above, sum the
dict's values excluding the 10-class exclusion set.

Highest-yield remaining single-class chassis builds: `divine_scion` (45,
`adventurers_guide`) and `phrenic_slayer` (43, `ultimate_psionics`) — same
two classes wave 35 flagged as highest-yield, now at a smaller, corrected
count each. **`ulfen_guard`, `argent_dramaturge`, `holy_vindicator` are now
the smallest (1–2 units)** if a next wave wants a minimal proof-of-chassis
target instead of the highest-yield one.

## Worktree base note (self-healed, not escalated)

This cycle's assigned worktree started at `ea2b3396f2` (the SD-33 PR #377
merge commit) — far behind the LOCAL `tranche/14` branch's real tip
(`c1580ac9ba`, wave 36's own wave-end gate). `origin/tranche/14` is itself
stale (`7ea9651b87`, wave 33 lane D) — the same "local branch is the real
live tip, origin lags" condition wave 36 lane C's own receipt documented.
Confirmed a clean fast-forward (`git merge-base --is-ancestor HEAD
tranche/14` → true, no local commits to lose) and rebased
(`git rebase tranche/14`, zero conflicts, zero commits of this cycle's own
to carry — this cycle had none yet). Re-ran `completion_atlas.py --check`
post-rebase and confirmed it matches wave 36's own final reported state
exactly (`population=49438`, `D: 2662`, `DONE: 25242`, `citation_failures=0`)
before doing any of this cycle's own analysis — confirming the correct base,
not a guess.

## Figures (every number, its command, its denominator)

- `population=49438 buckets=10 unclassified=0 overlap=0`, `D: 2662`,
  `DONE: 25242` — `python3 scripts/completion_atlas.py --check`, this
  cycle's own HEAD (post-rebase, pre-any-of-this-cycle's-own-writes; no
  bucket-moving change made this cycle, so this IS the final state too).
- `701` units / `68` classes total under `class_feature_of_unmodelled_corpus_class:*`,
  `634` units / `60` classes for sub-mechanism 5 specifically — the `Counter`
  command quoted above, of `docs/work-inventory.json`'s `49438`-unit
  population, this cycle's own committed HEAD.
- `151` engine-does-not-hold / `29` deferred-with-reason / `0` DONE — `Counter`
  over `units` where `kind=="class_feature"` and `"ranger_combat_style_feat"
  in id`, grouped by `status`, of that 180-unit sub-population, this cycle's
  own committed HEAD.
- `magnitude_token_count: 1` for the Sentinel unit — direct field read,
  `docs/work-inventory.json`, id
  `ultimate_intrigue:class_feature:sentinel_style_feat_improved_sense_intruder`.
- `1` correction retro-logged — `docs/retro/events/sd34-wave37-laneb.jsonl`,
  event id `1788442730755-sd34-wave37-laneb-7cdc91`.

## Row-count command output

```
$ grep -n "^| [0-9]* |" docs/release/SD-34-book-completion/kanban.md | tail -1
| 37 | `mine-bucket-d` | 3 | wave 32, lane C (no AT-34-E# card yet) | partial | ...
```
Same accumulating row 37 (`mine-bucket-d`) wave 32/35/36 all appended into —
this cycle appends its own sentence, per house style. Status stays
`partial`: this cycle closed 0 units.

## Build scope verified

No `src/`, `Cargo.toml`, or `scripts/` file touched this cycle — no build or
test run required or performed, per `workflow-instruction.md §6` step 3
("if touched"). `apps/desktop/src-tauri` — not touched, not run, same
reasoning.

## Sweep population

`corpus_literal_sweep` — not run: no `data/corpus/**` file added, changed,
or removed this cycle (N/A, 0 delta by construction, nothing to sweep).

## Oracle pin

Not applicable — no figure this cycle came from the pinned PCGen oracle
corpus; both items are derived from `docs/work-inventory.json` and the
committed `data/corpus/**` records directly.

## Status

**complete** — both assigned items are fully dispositioned: item 1's
requested fix is verified factually wrong and NOT made (0 units closed,
honestly reported, correction retro-logged); item 2's read-only
re-derivation is done and the corrected `634`-unit/`60`-class figure with
full per-class breakdown is in this receipt for the next wave to dispatch
directly, no further reconnaissance needed.

## Movement, four buckets

- **Closure:** 0.
- **Reclassification:** 0.
- **Reachability:** 0.
- **Instrument-correction:** 1 (the Sentinel Kind-retag hypothesis, wave
  35/36 lane C, refuted this cycle) + implicit re-derivation of sub-mechanism
  5's stale figure (832→634, not logged as a separate `correction` event
  since it is an expected, already-flagged consequence of wave 36's own
  matcher fix, not a newly-discovered wrong prior claim).

## Notes (judgment calls)

- **Why the brief's own hypothesis was trusted for two prior cycles before
  being checked:** wave 35 lane C's own reconnaissance cycle (read-only,
  no code-change authority) asserted the Kind-misclassification claim from
  the record's shape alone (`TYPE:RangerBonusFeat`, "like Vital Strike")
  without checking how Vital Strike's OWN record is actually filed. Wave 36
  lane C's disposition-trace cycle carried the claim forward unchanged
  (correctly out of its own narrower matcher-precedence scope, so it never
  re-checked the premise either). This cycle's own dispatch brief
  explicitly required confirming before touching anything — that
  instruction is what caught it, not luck.
- **Why no matcher fix was attempted either, even though the real cause
  (a short-word collision) IS a matcher-shaped bug**: proven above that even
  a correct fix would not move this unit out of D (its 151 real siblings are
  proof the underlying mechanism is unbuilt regardless of owner
  attribution) — a matcher fix here would only relabel the evidence string,
  not close anything, and was not the scope this cycle was dispatched for.
  Named for whoever eventually builds Ranger's combat-style-feat-chain
  mechanism (Epic 4/5 scope): fix the collision at the same time, since
  `inner_sea_gods:class:sentinel` (the real unmodelled prestige class) will
  otherwise keep absorbing this record's owner attribution.

## Next-cycle plan

1. **Sub-mechanism 5 is now directly dispatchable at `634` units / `60`
   classes** (this receipt's own table) — no reconnaissance cycle needed.
   `divine_scion` (45) and `phrenic_slayer` (43) remain the highest-yield
   single-class chassis builds; `ulfen_guard` (1), `argent_dramaturge` (2),
   `holy_vindicator` (2) are the cheapest if a minimal proof-of-shape is
   preferred first.
2. **Sentinel (1 unit) stays in D, correctly** — real disposition is
   "genuinely unbuilt, Epic 4/5 scope (Ranger's own 10th-level
   combat-style-feat-chain slot)", not a `Kind` bug. No further action
   needed unless/until Ranger's combat-style bonus-feat-chain mechanism
   (151 siblings, same evidence shape) is built — fold Sentinel's 1 unit
   into that dispatch and fix its owner-attribution collision
   (`class_feature_owner` vs. `inner_sea_gods:class:sentinel`) at the same
   time, same code surface as Order of the Dragon's own fix.
3. **The 4-unit gap between this cycle's own `198` and wave 36 lane C's
   claimed `202`** (both measuring sub-mechanism 5's own closure from the
   same matcher fix) is unreconciled — flagged, not chased, since it does
   not change sub-mechanism 5's own corrected remaining population (`634`,
   independently re-derived and directly reproducible).
