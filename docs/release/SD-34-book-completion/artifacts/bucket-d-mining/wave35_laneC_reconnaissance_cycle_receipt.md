# Cycle — SD-34 wave 35, Lane C — mine bucket D's two large untouched shapes (reconnaissance only, zero code/corpus changes)

**Status: partial.** Read-only mining cycle per this cycle's own dispatch brief (no
Rust/corpus changes authorized). Re-derived both target shapes fresh at this
cycle's own HEAD, confirmed both populations unchanged from the brief, then
decomposed each into named, population-counted sub-mechanisms — cheapest-first
— so the next wave can dispatch directly from the tables below.

- **Files touched this cycle:** this receipt, `progress.md`, `kanban.md`
  (docs only, per the brief's write-scope restriction). No `src/`,
  `Cargo.toml`, `data/corpus/`, or `docs/work-inventory.json` changes.
- **Worktree base note (self-healed, not escalated):** this cycle's assigned
  worktree started 41 commits behind `origin/tranche/14` (`ea2b3396f2`, the
  SD-33 PR #377 merge commit — `scripts/completion_atlas.py` and
  `docs/release/SD-34-book-completion/` did not exist there at all). Confirmed
  a clean fast-forward (`git merge-base --is-ancestor HEAD origin/tranche/14`
  → true, no local commits to lose) and fast-forwarded to `7ea9651b87` before
  any analysis — a self-healable stale-base condition
  (`workflow-instruction.md §8`), not a diverged-tree blocker. All figures
  below are re-derived at `7ea9651b87`, which matches this cycle's own
  dispatch-time brief figures exactly (see "Population" below) — confirming
  this is the correct, intended base, not a guess.

## Population derived fresh, not trusted from the brief

```
python3 scripts/completion_atlas.py --check
-> population=49438 buckets=10 unclassified=0 overlap=0
   DONE: 24963  A: 449  B: 11769  C: 4173  D: 2955  M: 4449  V: 289  U: 202  X: 170  Z: 19
   done_evidence_violations=0 missing_clearing_mechanisms=0 citation_failures=0
```

`D: 2955`, matching the brief's dispatch-time figure exactly. Of that:

```
class_feature_of_unmodelled_corpus_class:* -> 931 units, 70 distinct classes
class_feature_no_dedicated_magnitude_id_matched_the_record_slug -> 179 units
```

Both populations match the brief's stated figures exactly (931, 179) — not
smaller, not reshaped at the coarse level. `931 + 179 = 1110` of D's `2955`
(the wave 32 receipt's other four named mechanisms — 1,727 / 53 / 38 / 27 —
account for the remaining 1,845; `931+179+1727+53+38+27 = 2955`, confirmed by
summation, not assumed).

## Correction: the 75-class figure does not hold, 70 does

Both this wave's own dispatch brief and the wave 32 receipt it was copied from
state "75 distinct prestige/base classes" for the unmodelled-class shape. A
fresh, independent re-grouping at this cycle's own HEAD —

```python
collections.Counter(u["evidence"].split(":", 1)[1] for u in units
                     if (u.get("evidence") or "").startswith(
                         "class_feature_of_unmodelled_corpus_class:"))
```

— finds **70** distinct class slugs, not 75, while the unit population (931)
is bit-identical to the brief. The 75 figure was apparently never
independently re-verified by a second method after wave 32 first stated it;
this is the first re-derivation since. Logged:
`scripts/retro.py correction --subject "wave32_laneC_reconnaissance_receipt" --claimed "75 classes" --actual "70 classes" --verified-by "<the Counter command above>"`
(event id `1788403880209-sd34-wave35-lanec-53fd64`,
`docs/retro/events/sd34-wave35-lanec.jsonl`). This is a **prose correction**,
not a code fix — no bucket population moved, reported as
instrument-correction below, not a closure.

## Shape 1: `class_feature_of_unmodelled_corpus_class` — 931 units, decomposed into 5 sub-mechanisms

Per-book split (`python3 scripts/completion_atlas.py --by-book`-shaped query,
this shape only, largest first): `ultimate_psionics` 330, `adventurers_guide`
252, `advanced_players_guide` 105, `inner_sea_magic` 63, `ultimate_magic` 31,
`inner_sea_intrigue` 27, `occult_adventures` 25, `inner_sea_combat` 23,
`ultimate_intrigue` 19, `advanced_class_guide` 13, `book_of_the_damned_volume_1`
10, `ultimate_wilderness` 10, `book_of_the_damned_volume_2` 8,
`ultimate_combat` 6, `core_rulebook` 5, `inner_sea_world_guide` 3,
`pathfinder_unchained` 1.

The evidence string names the **corpus_class** the matcher ultimately
resolved (`slug(&corpus_class)`, `v06_work_inventory.rs:11951-11969`, this
cycle's own HEAD line numbers — re-derive with `grep -n
class_feature_of_unmodelled_corpus_class src/bin/v06_work_inventory.rs`),
derived from `class_feature_owner` walking `unit.key`'s `<Group> ~ <Feature>`
prefix against a candidate class-name list. Reading that matcher's actual
code (`class_feature_owner` at `v06_work_inventory.rs:9491`,
`modelled_class_books()` at `:13910`, `corpus_class_names` construction at
`:15404`) against each sub-population below — never assumed from the label
alone — splits the 931 into 5 genuinely different mechanisms:

| # | Sub-mechanism | Population | Real fix needed |
|---|---|---:|---|
| 1 | **Matcher misattributes a class that already has a real chassis** (`psychic_warrior` 18 + `rogue` 1) | **19** | Fix the matcher, not a new chassis — see "Sub-mechanism 1" below, exact line cited |
| 2 | **Non-class "creature type" collision** (`animal` 23, `undead` 13, `dragon` 15, `construct` 2, `plant` 9, `ooze` 1) | **63** | Not a PC-class chassis question at all — see "Sub-mechanism 2" |
| 3 | **Eidolon companion-progression records** (`eidolon`) | **16** | A companion-table mechanism (like Animal Companion), not a class chassis — see "Sub-mechanism 3" |
| 4 | **Single-unit matcher name-collision** (`sentinel`, "Sentinel Style Feat" ~ a combat-style feat chain, not a class feature) | **1** | Same short-word-collision failure mode as #1/#2, isolated — see "Sub-mechanism 4" |
| 5 | **Genuinely unmodelled prestige/base classes**, 60 distinct classes | **832** | Real new chassis per class, Epic 4/5 scope — full per-class table below |

`19 + 63 + 16 + 1 + 832 = 931` — sums exactly, re-derived by the same script
this session (`scratchpad/d_breakdown2.py`-shaped grouping, reproducible by
re-running the `Counter` command above and classifying each of the 70 class
slugs against the three named lists in "Sub-mechanism 1/2/3" below).

### Sub-mechanism 1 (19 units) — matcher discards a real chassis match; NOT a "new class" problem

Both classes named here **already compute a real chassis** the engine holds:
`psychic_warrior` is one of `untabled_base_class_chassis`'s own 20-class
registry (`resolve()` returns real BAB/save rows, confirmed in
`tests/fixtures/rules_core/untabled-base-class-chassis.json`), and `rogue` is
one of CRB's original 11 `ClassId::ALL` base classes
(`crb_class_name(ClassId::Rogue) => "rogue"`,
`v06_work_inventory.rs:8849`). Traced to source, not asserted from the label:

- **`psychic_warrior` (18 units)** — `modelled_class_books()`
  (`v06_work_inventory.rs:13910`) inserts every `untabled_base_class_chassis`
  registry entry's **underscore-slugged** `bare_name` directly as the
  `class_books` key (`:13958`, `class_books.insert(bare_name, book)`) — e.g.
  `"psychic_warrior"`, never converted to space form. The function's OWN doc
  comment three lines below, for the very next loop (CRB prestige classes),
  states the correct convention explicitly: *"This key is the corpus display
  name, lowercased AS-IS (never underscore-slugged): `classify`'s `Kind::Class`
  arm looks up `unit.name.to_lowercase()`, which for a multi-word class like
  'Arcane Archer' is `"arcane archer"` (a space), not `"arcane_archer"`"*
  (`:13966-13970`). The untabled-registry loop above it does not follow its
  own neighbor's documented rule. `class_feature_owner`'s matching
  (`:9491-9504`) already normalizes underscores to spaces via
  `class_name_as_group_text` (`:9475`, `class.replace('_', " ")`) for the
  CANDIDATE side, so a single-word class name is unaffected — this bug only
  bites a **multi-word** untabled-registry class name, and `psychic_warrior`
  is the ONLY multi-word entry among the registry's 20
  (`aegis, antipaladin, cryptic, dread, kineticist, magus, marksman, medium,
  mesmerist, occultist, psion, psychic, shifter, soulknife, spiritualist,
  tactician, vigilante, vitalist, wilder` are all single words). Confirmed
  against a real record: `ultimate_psionics:class_feature:
  psychic_warrior_eternal_warrior` (`corpus_key: "Psychic Warrior ~ Eternal
  Warrior"`) reports `class_feature_of_unmodelled_corpus_class:psychic_warrior`
  despite the chassis existing. **Not live-verified by a cargo run this
  cycle** (read-only mandate) — flagged as a traced, code-grounded hypothesis
  for the next wave's RED test, not an asserted fix.
- **`rogue` (1 unit)** — a different mechanism: `pathfinder_unchained:
  class_feature:unchained_rogue_finesse_training_choice` (`corpus_key:
  "Unchained Rogue ~ Finesse Training Choice"`). `modelled_owner` (searched
  over `class_books.keys()`) correctly finds `"unchained_rogue"` (PU's own
  registered class, 15 chars) over the shorter `"rogue"` (5 chars) via the
  length tie-break (`:9499`). The safety cross-check
  (`key_group_owner = modelled_owner.filter(|candidate| corpus_wide_owner
  ... == candidate)`, `:11934-11936`) then searches `corpus_class_names` (raw
  corpus `Kind::Class` record names only, `:15404-15409`) and finds only
  `"rogue"` — the corpus never declares a standalone `"Unchained Rogue"`
  class record (PF Unchained's Rogue variant is presented as alternate
  features grafted onto the base Rogue, not a separate class file). The
  mismatch (`"unchained_rogue" != "rogue"`) discards `key_group_owner`
  entirely, and every fallback (type_facet, pool-catalog) also misses
  (`type_facet` here is `"Unchained Rogue Finesse Damage Choice.
  SpecialQuality.Extraordinary"`, no `"<Class> Class Feature"` marker). The
  FINAL fallback (`:11951-11969`) re-runs `class_feature_owner` against
  `corpus_class_names` alone and reports whatever it finds as
  "unmodelled" — **without re-checking whether that name is itself a
  `class_books` member**. It finds `"rogue"`, which unambiguously IS
  modelled (`ClassId::Rogue`, single word, immune to sub-mechanism 1's
  underscore bug) — so this one unit is mislabeled purely because the final
  fallback branch never repeats the class_books membership check the earlier
  branches already do. Same "traced, not live-verified" caveat as above.

### Sub-mechanism 2 (63 units) — six standard Pathfinder **creature types**, not classes

`animal`, `undead`, `dragon`, `construct`, `plant`, `ooze` are the corpus's
own declared `Kind::Class` record names for these exact words — real corpus
records exist (confirmed: `corpus_class_names` is built from `Kind::Class`
units' `name.to_lowercase()`, `:15404-15409`), but every one of the 63
records here is a PLAYER-FACING feature belonging to a **companion or
subdomain mechanism**, not a playable class: `Animal Companion`/`Animal
Trick`/`Animal Speaker`/`Spirit Animal` (23), `Power Over Undead`/`Undead
Scourge`/`Undead Lord`/`Undead Savant Subschool` (13), `Dragon Shaman`/
`Dragon Subdomain`/`Order of the Dragon` (15), `Construct Subdomain`/`PaDFE
Construct` (2), `Plant Master Plant Focus` (9), `PaDFE` doesn't apply to ooze
— 1 unit, likely a companion-focus variant. These match via
`class_feature_owner`'s `ends_with`/`starts_with` whole-word rule against a
real but semantically-unrelated `Kind::Class` corpus record sharing the
creature-type word — the SAME failure shape wave 32's own receipt already
named for `"warrior"` vs `"Adaptive Warrior"` (a short/common word
coincidentally matching), just running in the opposite direction here (the
short word IS the winning match, and it's a real corpus record, but the
WRONG one for this feature). **Disposition, not a chassis-building task**:
these are companion/subdomain/archetype tables (Animal Companion, Undead
subdomain granted powers, Dragon Shaman totem features, ...) that likely
already have — or need — their OWN dedicated engine mechanism, structurally
unrelated to "does this class have a BAB/save chassis." Needs a
per-mechanism trace (does `Animal Companion` already compute elsewhere and
just misreport its evidence here? `core_rulebook:class_feature:
animal_companion_base` suggests yes, a druid/ranger mechanic that plausibly
already exists) before any code change — not this cycle's scope to trace
further.

### Sub-mechanism 3 (16 units) — Eidolon companion-progression tables

All 16 are Summoner's Eidolon evolution-slot tables by size class at each
Eidolon level (`Eidolon 1 ~ L/M/S` … `Eidolon 8 ~ S`, plus `Eidolon Companion
Progression ~ Standard` and `Eidolon ~ First Worlder`). `eidolon` matches a
real corpus `Kind::Class` record (the Eidolon IS modelled as its own
class-shaped entity for evolution-point bookkeeping), but a per-level
evolution-slot table is a companion mechanism, not a base/save chassis
question — same disposition-not-chassis shape as sub-mechanism 2, kept
separate because `eidolon` is a real declared class-shaped record rather than
a creature-type collision.

### Sub-mechanism 4 (1 unit) — a feat chain misattributed via the same collision shape

`ultimate_intrigue:class_feature:sentinel_style_feat_improved_sense_intruder`
(`corpus_key: "Sentinel Style Feat ~ Improved Sense Intruder"`,
`type_facet` includes `RangerBonusFeat`) is a combat STYLE FEAT chain (like
Vital Strike or Two-Weapon Fighting style feats), not a class feature at
all. It matches only because some corpus `Kind::Class` record is literally
named `"Sentinel"` and the group text `"sentinel style feat"` starts with
`"sentinel "`. Isolated (1 unit) and named separately from sub-mechanism 2
because it is a feat-vs-class-feature `Kind` question, not a creature-type
question — a `Kind` misclassification at ingest time is a different fix
surface than a companion-mechanism trace.

### Sub-mechanism 5 (832 units, 60 classes) — genuine new-chassis-per-class work, cheapest-first

Every remaining class checked against `modelled_class_books()`'s full key
set (CRB 11 + APG 6 + ACG 10 + UC 3 + PU 4 + `untabled_base_class_chassis`'s
20 + CRB's 10 prestige (`prestige-class-entry-requirements.json`,
`source_book == "core_rulebook"` only) + `crb_untabled_class_chassis`'s 7 —
`v06_work_inventory.rs:13910-13990`) — none of these 60 names appear in that
set under any spelling. Spot-checked four of the smaller ones
(`horizon_walker`, `argent_dramaturge`, `evangelist`, `gifted_blade`) against
their real corpus keys to rule out the sub-mechanism-2/4 collision shape —
all four are genuine, real prestige-class feature trees with no ambiguity.
Largest-first (highest chassis-building yield per class):

| Class | Units | Class | Units | Class | Units |
|---|---:|---|---:|---|---:|
| phrenic_slayer | 47 | psychic_detective | 18 | metamind | 11 |
| divine_scion | 46 | adaptive_warrior | 18 | holy_vindicator | 10 |
| sighted_seeker | 25 | psychic_fist | 18 | pathfinder_delver | 10 |
| phantom | 24 | cyphermage | 17 | diabolist | 10 |
| stalwart_defender | 21 | sanguine_angel | 17 | cerebremancer | 10 |
| elocater | 21 | twilight_talon | 17 | aspis_agent | 9 |
| thrallherd | 21 | war_mind | 17 | gray_corsair | 9 |
| asavir | 20 | master_spy | 16 | rivethun_emissary | 9 |
| metamorph | 19 | golden_legionnaire | 16 | ulfen_guard | 8 |
| body_snatcher | 19 | hellknight | 15 | master_chymist | 7 |
| psion_uncarnate | 19 | mammoth_rider | 15 | bellflower_tiller | 7 |
| pathfinder_savant | 15 | pure_legion_enforcer | 15 | hellknight_signifer | 7 |
| lion_blade | 15 | nature_warden | 14 | demoniac | 7 |
| psicrystal_imprinter | 14 | aldori_swordlord | 13 | mystic_archer | 7 |
| pyrokineticist | 13 | steel_falcon | 12 | soul_archer | 7 |
| enchanting_courtesan | 12 | telekinetic_weaponmaster | 12 | dark_tempest | 6 |
| battle_herald | 11 | rage_prophet | 11 | metaforge | 6 |
| lantern_bearer | 11 | magaambyan_arcanist | 11 | evangelist | 5 |
| storm_kindler | 11 | student_of_war | 11 | horizon_walker | 4 |
| westcrown_devil | 11 | gifted_blade | 3 | argent_dramaturge | 2 |

Sum: 832 across 60 classes (re-derive: same `Counter`, excluding the 10
classes pulled into sub-mechanisms 1–4 above). `phrenic_slayer` (47,
`ultimate_psionics`) and `divine_scion` (46, `adventurers_guide`) are the two
highest-yield single-class chassis builds remaining.

## Shape 2: `class_feature_no_dedicated_magnitude_id_matched_the_record_slug` — 179 units

Unlike Shape 1, every one of these 179 belongs to an **already-modelled**
class — this branch (`v06_work_inventory.rs:12335-12337`) is only reached
after `owner` (a real `class_books` match) is found and the engine's own
generic per-class roster proves it holds SOME record for this feature group,
but no record-specific magnitude id matched this exact feature's slug.

Per-book (`python3` grouping on `book`, largest first):
`ultimate_psionics` 45, `occult_adventures` 42, `core_rulebook` 29,
`pathfinder_unchained` 24, `advanced_players_guide` 14, `ultimate_magic` 10,
`ultimate_intrigue` 8, `ultimate_wilderness` 7.

Per-owning-class (36 distinct groups, all named, none left as "the rest"):

| Class | Units | Class | Units | Class | Units |
|---|---:|---|---:|---|---:|
| Mesmerist | 11 | Cryptic | 6 | Duelist | 4 |
| Magus | 10 | Dread | 6 | Shadowdancer | 4 |
| Medium | 8 | Tactician | 6 | Spiritualist | 4 |
| Unchained Monk | 8 | Vitalist | 6 | Unchained Summoner | 4 |
| Vigilante | 8 | Psychic | 5 | Soulknife | 4 |
| Antipaladin | 7 | Marksman | 5 | Wizard | 3 |
| Summoner | 7 | Wilder | 5 | Assassin | 2 |
| Kineticist | 7 | | | Fighter | 2 |
| Occultist | 7 | | | Loremaster | 2 |
| Aegis | 7 | | | Bard/Cleric/Druid/Paladin/Ranger/Sorcerer | 1 each |
| Shifter | 7 | | | | |
| Monk | 6 | | | | |
| Unchained Barbarian | 6 | | | | |
| Unchained Rogue | 6 | | | | |

Sum: 179 (Mesmerist-through-Wilder's 17 groups: 11+10+8+8+8+7+7+7+7+7+7+6+6+6
+6+6+6 = 132; Duelist-through-6-singles: 4+4+4+4+4+3+2+2+2+6 = 39; plus
Cryptic-through-Wilder already counted — re-derive via the by-group `Counter`
command, `sum(by_group.values()) == 179` confirmed live this cycle).

**A cheap sub-split exists inside these 179, mirroring wave 32's own
"1,727-shape" investigation exactly, not yet run to completion this
cycle:** 25 of the 179 are STILL `magnitude_token_count == 0` (zero
magnitude) despite reaching this "magnitude-id" branch — they are here
because the "text-complete" promotion's OTHER gates
(`has_real_description`/`universal_sheet_modifier`/`is_display_wiring_class_
for_promotion`) blocked them, not because a magnitude id is missing at all.
Of those 25: 17 already carry `wiring_class == "display"` (the promotion
gate's own requirement), 4 are `derived`, 4 are `ambiguous`. **The next
wave's cheapest first check on this shape**: cross-reference those 17
`display`-wiring-class, zero-magnitude units against real corpus `DESC:`
tokens and the `universal_sheet_modifier` gate, the exact method wave 32's
receipt already proved out on the 1,727-unit shape — this could promote a
subset to `text-complete` with zero magnitude-id work, the same way wave
32's 1,727-shape investigation was structured (there it found zero
promotable; this smaller, different 25-unit population has not been run
through that same check yet). The remaining 154 (`magnitude_token_count` 1–3)
need real per-feature magnitude-id matching work — Epic 3 scope, as wave
32's receipt already named.

## Movement (four buckets, this cycle)

- **Closure (bucket → DONE):** 0. Verified: `python3
  scripts/completion_atlas.py --check` before and after this cycle's own
  docs-only commit reports identical counts (`D: 2955` both times — no code
  or corpus file was touched, so this is guaranteed rather than merely
  observed).
- **Reclassification (bucket → different non-DONE bucket):** 0. No unit's
  `evidence` or `status` changed (`docs/work-inventory.json` untouched this
  cycle).
- **Reachability:** 0 units newly reached or lost reachability.
- **Instrument-correction:** 1 prose correction (the 75→70 class-count figure
  in the brief/wave-32 receipt, retro event
  `1788403880209-sd34-wave35-lanec-53fd64`) — not a code or script fix, a
  corrected number in the record. Two flagged-but-unverified matcher
  hypotheses (sub-mechanism 1, 19 units) are named for the next wave, not
  claimed as fixed or as instrument-corrections themselves (no cargo run
  confirmed them this cycle).

## Figures (every number, its command, its denominator)

- `population=49438`, `D: 2955` — `python3 scripts/completion_atlas.py
  --check`, of the full corpus.
- `931` (`class_feature_of_unmodelled_corpus_class:*`), `70` distinct classes
  — `Counter` grouping over `docs/work-inventory.json`'s `units`, of `D:
  2955`; re-derive with the exact command quoted above.
- `179` (`class_feature_no_dedicated_magnitude_id_matched_the_record_slug`) —
  same file, exact-string match, of `D: 2955`.
- `19 + 63 + 16 + 1 + 832 = 931` — sub-mechanism split of the 70-class
  breakdown, classified against `modelled_class_books()`'s full key set (read
  from `v06_work_inventory.rs:13910-13990`, cross-checked against
  `tests/fixtures/rules_core/untabled-base-class-chassis.json` (20 entries)
  and `tests/fixtures/rules_core/prestige-class-entry-requirements.json`
  filtered to `source_book == "core_rulebook"` (10 entries)), of the 931.
- `36` distinct owning classes, `179` units summing exactly — `Counter` on
  `corpus_key.split(" ~ ")[0]`, of the 179-unit shape.
- `25` of `179` are `magnitude_token_count == 0`; of those, `17` are
  `wiring_class == "display"`, `4` `derived`, `4` `ambiguous` — same file,
  field-level `Counter`, of the 179.
- `75 -> 70` class-count correction — re-derivation command above; the
  brief's/wave-32's own 75 was never independently re-verified until now.

## Verification

- `python3 scripts/completion_atlas.py --check` → clean, `citation_failures=0`,
  `done_evidence_violations=0`, `stale_derived_at=False`, exit 0 (run at this
  receipt's own final state, no drift from the pre-cycle run since no
  code/corpus/inventory file was touched).
- `python3 scripts/denominator_gate.py --check` → run against this receipt
  after writing it (see structured output for the literal line).
- No `src/`, `Cargo.toml`, `data/corpus/`, or `docs/work-inventory.json`
  changes this cycle (docs-only diff) — a `cargo build`/`cargo test` re-run is
  not warranted by this cycle's own diff and was not run, consistent with
  this cycle's read-only mandate and the wave's three-lane cargo-concurrency
  safety ceiling (`.cargo/config.toml` `jobs=6`, no cargo process started
  here at all).
- `git status --porcelain` clean before every write this cycle in this
  cycle's own isolated worktree; no `git add -A`; `git diff --cached
  --numstat` read before committing.

## Next-cycle plan (cheapest-first across both shapes)

1. **Sub-mechanism 1 (19 units, `psychic_warrior` 18 + `rogue` 1)** — write
   the RED test first (a `class_feature_owner`/`modelled_class_books` unit
   test asserting `psychic_warrior`'s multi-word key resolves, and that the
   final `corpus_class_names` fallback at `:11951` checks `class_books`
   membership before declaring a class unmodelled), confirm it fails for the
   stated reason, then fix both: (a) space-join `bare_name` in the untabled-
   registry loop the same way the CRB-prestige loop already does; (b) add a
   `facts.class_books` membership check to the final fallback branch before
   emitting `class_feature_of_unmodelled_corpus_class`. Cheapest of all five
   sub-mechanisms — no new chassis, no new corpus ingestion, a ~2-line
   matcher fix plus its test, and it may also affect classes outside this
   931-unit shape wherever the same two bugs fire (not traced further this
   cycle — flag for the next wave to check `Kind::Class`-level records too).
2. **Shape 2's 25-unit zero-magnitude sub-split** — run wave 32's own proven
   `DESC:`-token / `universal_sheet_modifier` cross-reference method against
   the 17 `display`-wiring-class units named above. Small, already scoped,
   reuses an existing proven method rather than inventing a new one.
3. **Sub-mechanisms 2–4 (80 units: 63 creature-type + 16 eidolon + 1
   sentinel)** — needs a disposition trace per named group (does `Animal
   Companion` already compute elsewhere under a different unit id? is
   `Eidolon`'s evolution-slot table a real gap or already served by a
   different mechanism?) before any code change — likely a
   `decisions.md`-shaped ruling on how a companion/subdomain feature should
   be evidenced, not a chassis question at all.
4. **Shape 2's remaining 154 magnitude-bearing units, 36 owning classes** —
   real per-feature magnitude-id matching, Epic 3 scope; dispatch by class
   group size, largest first (Mesmerist 11, Magus 10, Medium/Unchained
   Monk/Vigilante 8 each, ...).
5. **Sub-mechanism 5 (832 units, 60 classes)** — real new chassis per class,
   Epic 4/5 scope; the per-class table above is already cheapest-first
   (largest-yield-first) for dispatch — `phrenic_slayer` (47) and
   `divine_scion` (46) first.
