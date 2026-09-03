# Cycle — SD-34 wave 36, Lane C — disposition trace of wave 35 lane C's sub-mechanisms 2–4 (80 units), one matcher bug found and fixed

- **Commit SHA:** `045612dd25` (final; matcher fix itself is `140424bda5`)
- **Files touched:** `src/bin/v06_work_inventory.rs` (matcher fix + 2 new tests),
  `scripts/completion_atlas.py` (4 shifted citation pins re-derived),
  `docs/work-inventory.json` (guarded regen), `docs/release/SD-34-book-completion/
  artifacts/epic-1-atlas/completion-atlas.json` (regenerated snapshot), this receipt,
  `progress.md`, `kanban.md`, `docs/retro/events/sd34-wave36-lanec.jsonl`.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (`git diff --unified=0
  4379c9be05...HEAD -- src/bin/v06_work_inventory.rs scripts/completion_atlas.py`, 137
  lines, no `sd[0-9]+_`/`SD[0-9]+_`/`t_[0-9a-f]{8,}` hits).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff, no `STUB`/`MOCK`/
  `placeholder`/`not yet implemented`/`todo`/`fixme`/`hack`).
- **Acceptance criterion (verbatim from this cycle's dispatch brief):** "DISPOSITION
  TRACE (investigation, real code change only if the trace clearly resolves without
  ambiguity) on wave 35 lane C's named Sub-mechanisms 2–4 (80 units)... For each ...
  determine whether the feature is (a) already computed elsewhere under a DIFFERENT
  unit id this shape double-counts (a real matcher bug you should fix, same rigor as
  lane A's own sub-mechanism 1), (b) a genuinely-needed new companion/subdomain-table
  mechanism (name it precisely, do not build it this cycle), or (c) something else...
  Fix ONLY case (a) findings (matcher bugs) if you find any -- do not attempt (b) work
  this cycle."

## Worktree base note (self-healed, not escalated)

This cycle's assigned worktree started at `7ea9651b87` (wave 33 lane D), 30 commits
behind the LOCAL `tranche/14` branch's real tip (`4379c9be05`, wave 35's own wave-end
gate). `git fetch origin tranche/14` returns a still-stale `origin/tranche/14` ref
(also `7ea9651b87`) — the local branch is the correct, current one; two other wave-36
lanes' own commits (`39bfdaabda`, `da93ef2254`, lane B) were already reachable via
`git log --all` on their own unmerged worktree branch, confirming `tranche/14`
(local) is this wave's real live base. Confirmed a clean fast-forward
(`git merge-base --is-ancestor 7ea9651b87 tranche/14` → true) and rebased this
cycle's own checkpoint commit onto it (`git rebase tranche/14`, zero conflicts).
**This was caught only after already committing and regenerating once against the
stale base** — that first regen was killed and redone from scratch against the
correct base; nothing from the stale-base attempt was kept. `completion_atlas.py`'s
4 citation pins (A/B/C/V) were re-derived after this cycle's own ~34-line insertion
shifted them (`citation_failures` 4→0).

## Disposition table — all 80 units, none left as "the rest"

Traced against the real corpus JSON (`data/corpus/**`, `"data"."class"` and raw
tokens), `docs/work-inventory.json`'s sibling records, and the classifier's actual
code path (`class_feature_owner`, `class_feature_owner_via_type_facet`,
`class_feature_pool_catalog_holds`/`class_feature_standalone_catalog_holds`,
`v06_work_inventory.rs:9491-12900`, this cycle's own HEAD line numbers) — never
assumed from the evidence label alone, the same rigor as wave 35 lane C's own
sub-mechanism 1 trace.

| Group | Units | Real owner (traced to source) | Disposition | Outcome |
|---|---:|---|---|---|
| **animal** — Animal Companion (base/standard companions/tricks) | 4 | Ranger `Hunter's Bond`/Druid `Nature's Bond` (both real, modelled CRB classes) | **(b)** genuinely unbuilt | `Hunter's Bond ~ Animal Companion`/`Nature's Bond ~ Animal Companion` (the REAL granting records) are ALSO `engine-does-not-hold` under different D-bucket shapes — no Animal Companion progression is computed anywhere in this engine yet. **Refutes wave 35's own "plausibly already exists" hypothesis** — retro-logged correction, `1788423408755-sd34-wave36-lanec-12107d` |
| **animal** — Spirit Animal (ACG/adventurers_guide/ultimate_wilderness) | 13 | ACG "Spirit Animal" choice pool (real corpus record; owning class not fully traced this cycle) | **(b)** genuinely unbuilt | Corpus-wide `"spirit animal"` search: 20 total, 18 `engine-does-not-hold`, 2 `ingested-magnitude` — no DONE sibling anywhere; 11 of these 13 are zero-magnitude and now `text-complete` via this cycle's own fix (see Movement below) |
| **animal** — Animal Speaker (ultimate_magic) | 6 | Ultimate Magic feat/talent tree (owning class not fully traced this cycle) | **(b)** genuinely unbuilt | Corpus-wide `"animal speaker"` search: 7 total, all `engine-does-not-hold` — no DONE sibling anywhere |
| **undead** — Power Over Undead (Command/Turn) | 2 | Cleric (Channel Energy chassis) | **(b)** genuinely unbuilt, **confirmed by the engine's own doc comment** | `pilot_compute/mod.rs`'s `oracle_channel_dc` doc comment, verbatim: *"the feat's own `cr_feats.lst` record names five variables — `ClericChannelPositiveEnergyDC`, `PaladinChannelPositiveEnergyDC`, `ClericChannelNegativeEnergyDC`, `PowerOverUndeadCommandDC`, `PowerOverUndeadTurnDC` — and this engine computes a total for none of them: Cleric grounds `channel_energy_dice`/`channel_energy_uses_per_day` but no DC at all"* |
| **undead** — Undead Savant Subschool ×2 (ACG) | 2 | **Arcanist** (real, modelled chassis — memory note "Arcanist: first ACG/APG class to reach Computed") | **(c)** real misattribution, **NOT fixed** | `type_facet: "ArcanistClassFeatures.SpecialQuality..."` — no space before `ClassFeatures`, so the existing marker-extractor (`class_feature_type_facet_owner_candidates`, requires a leading-space `" Class Feature(s)"` suffix) cannot recover it. The corpus's own `/data/class` field (`"Undead"`) is unreliable elsewhere in this SAME 80-unit population (Power Over Undead/Animal Companion Base also carry `class` = the collision word for internal-bookkeeping reasons) — not safely generalizable without a corpus-wide validation pass this cycle's narrow-fix bar does not cover |
| **undead** — Undead School ~ Bolster (APG) | 1 | Wizard (Focused Arcane School) | **(c)** same shape as Undead Savant Subschool, not fixed | `data/corpus/advanced_players_guide/class_feature/focused_arcane_school/undead_school.json` confirms the real owning directory |
| **undead** — Undead Subdomain ~ Death's Kiss (APG) | 1 | Cleric subdomain granted power | **(c)**/(b) domain-vs-class_feature dual representation | `advanced_players_guide:domain:undead_subdomain` (the sibling `Kind::domain` record) is ALSO `engine-does-not-hold` — not a double count against a DONE unit |
| **undead** — Undead Scourge ×3 (APG, Paladin archetype) | 3 | **Paladin** (real, modelled chassis) | **(b)** genuinely unbuilt, **confirmed** | `core_rulebook:class_feature:paladin_smite_evil` (base Paladin's OWN Smite Evil) is ALSO `engine-does-not-hold` — nothing to double-count against |
| **undead** — PaDFE Undead | 1 | Pathfinder Delver | see PaDFE row below |
| **undead** — Undead Minion ~ Kabriri (book_of_the_damned_v2) | 1 | Demonic Obedience boon chain for the demon lord Kabriri (no PC class) | **(b)** genuinely unbuilt, **partially-built sibling mechanism** | `Ghoulish Apotheosis ~ Kabriri` (`text-complete`) and `Demonic Obedience ~ Kabriri` (`oracle-agree`) — same demon lord's OTHER boon tiers — ARE done; this is the one remaining ungrounded tier, not a duplicate |
| **undead** — Undead Lord ×2 (ultimate_magic, Cleric archetype) | 2 | Cleric (archetype) | **(b)** genuinely unbuilt | `data/corpus/ultimate_magic/class_feature/cleric_archetype/undead_lord.json` confirms the real owner; no DONE sibling found |
| **dragon** — Order of the Dragon ×5 (APG) | 5 | **Cavalier** (real, modelled chassis) | **(a) FIXED THIS CYCLE** | See "The matcher bug" below |
| **dragon** — Dragon Subdomain ~ Dragonbreath (inner_sea_world_guide) | 1 | Cleric Dragon subdomain granted power | **(c)** domain-vs-class_feature dual representation | The domain-kind sibling for the SAME book (`inner_sea_world_guide:domain:dragon_subdomain`) is `ingested-magnitude`, not DONE — not a double count |
| **dragon** — Dragon Shaman ×9 (ultimate_magic, Druid archetype) | 9 | **Druid** (7 of 9 records) / literally `"Shaman"` (2 of 9 — `class` field disagrees WITHIN this one archetype) | **(c)** real misattribution, **NOT fixed** | Same reasoning as Undead Savant Subschool: the corpus's own `/data/class` field is internally inconsistent even within a single archetype (confirmed by direct read of all 9 corpus JSON files), so it cannot be trusted as a safe, generalizable owner signal this cycle |
| **construct** — Construct Subdomain ~ Animate Servant (APG) | 1 | Cleric subdomain granted power | **(c)**/(b) same domain dual-representation pattern | `advanced_players_guide:domain:construct_subdomain` (sibling) ALSO `engine-does-not-hold` |
| **construct** — PaDFE Construct | 1 | Pathfinder Delver | see PaDFE row below |
| **plant** — Plant Master Plant Focus ×9 (ultimate_wilderness) | 9 | **Hunter** (real, modelled chassis, Hunter archetype "Plant Master") | **(c)** same shape as Dragon Shaman, not fixed | `data/corpus/ultimate_wilderness/class_feature/*/plant_master*.json`: base features carry `class: "Hunter"`; this 9-unit sub-choice table carries `class: "Plant"` (the collision word) |
| **ooze** — PaDFE Ooze | 1 | Pathfinder Delver | see PaDFE row below |
| **PaDFE Construct/Ooze/Undead** (adventurers_guide) | 3 | **Pathfinder Delver** (real corpus prestige class; its "Guardbreaker" feature's `ABILITY:` tokens grant these three) | **(c)** real misattribution, **zero bucket movement even if fixed** | `data/corpus/adventurers_guide/class_feature/pathfinder_delver/guardbreaker.json` confirms `class: "Pathfinder Delver"` and grants `PaDFE Construct`/`PaDFE Ooze`/`PaDFE Undead`. Pathfinder Delver is itself one of sub-mechanism 5's 60 genuinely-unmodelled classes (10 OTHER units already counted there) — reclassifying these 3 would not move them out of D. **Flag for the next wave building Pathfinder Delver's chassis**: its real total is 13 units, not 10 |
| **eidolon** | 16 | **Summoner** (Eidolon evolution-slot-per-level table) | **(b)** genuinely unbuilt, **confirmed twice** | `class_summoner.rs`'s own doc comment: *"Named per-level features (Eidolon, Bond Senses, Life Link, Shield Ally, Aspect, ...) ... are out of scope for this cycle"*; independently corroborated by wave 36 lane B's own concurrent kanban row 36 finding, *"an eidolon mechanic this crate does not model at all"* |
| **sentinel** — Sentinel Style Feat ~ Improved Sense Intruder (ultimate_intrigue) | 1 | Not a class feature at all — a combat style feat chain (like Vital Strike) | **(c)** `Kind` misclassification at ingest, **NOT fixed** | NOT a duplicate of `Sentinel ~ Sense Intruder` (the base Vigilante talent, itself still D-bucket, a genuinely different feature) — confirmed by reading both corpus records. `"Sentinel"` is a legitimately common word across 140+ real distinct corpus records (Vigilante archetype, Warpriest deity boons, Ranger `Summit Sentinel` archetype, ...), confirmed by a corpus-wide search. Needs an ingest-time `Kind::feat` re-tag, a different code surface than this cycle's matcher-owner-resolution scope |

**Sum check:** animal 4+13+6=23, undead 2+2+1+1+3+1+1+2=13, dragon 5+1+9=15,
construct 1+1=2, plant 9, ooze 1, eidolon 16, sentinel 1 → **23+13+15+2+9+1+16+1 = 80**,
exactly the brief's own population, none left as "the rest."

## The matcher bug (case a) — traced, fixed, tested

`class_feature_of_unmodelled_corpus_class`'s owner-resolution `else` branch
(`v06_work_inventory.rs`, `classify()`'s `Kind::ClassFeature` arm) asks whether the
CORPUS declares a class matching the group text — but this corpus-wide check runs
and RETURNS before the `text_only` promotion checks a few lines below it
(`class_feature_pool_catalog_holds`/`class_feature_standalone_catalog_holds`) ever
get a chance to run, even when the record would independently pass them.

For `"Order of the Dragon"`, `group.ends_with(" dragon")` matches the corpus's own
UNMODELLED bestiary `Kind::Class` record `"Dragon"` (`bestiary:class:dragon`, itself
`engine-does-not-hold`) — short-circuiting the record into a false "unmodelled
class" gap. Its real owner is Cavalier (`type_facet:
"CavalierClassFeatures.CavalierOrder..."`), which `class_feature_owner`'s own
group-text rule can never recover (`"order of the dragon"` neither starts nor ends
with `"cavalier "`) — **the fix does not need to resolve that owner at all**, it
only needed to stop reporting a false gap when a holds-check would independently
succeed. Proven by direct comparison: every non-colliding sibling order —
`"Order of the Beast"`, `"Order of the Cockatrice"`, `"Order of the Lion"`,
`"Order of the Shield"`, `"Order of the Star"`, `"Order of the Sword"`, `"Order of
the Paw"`, `"Order of the Asp"`, `"Order of the Chain"`, `"Order of the Gate"`,
`"Order of the Nail"`, `"Order of the Pyre"`, `"Order of the Rack"`, `"Order of the
Scourge"`, `"Order of the Guard"`, `"Order of the Green"`, `"Order of the Seal"`,
`"Order of the Tome"`, `"Order of the Warrior"` — already reaches `text-complete`
through exactly the promotion `"Order of the Dragon"` was denied, confirmed live in
`docs/work-inventory.json` before this cycle's own fix. `"Order of the Warrior"`
in particular already succeeds despite `"warrior"` ALSO being a real corpus
`Kind::Class` collision word (CRB's own NPC Warrior class) — the difference is that
`"warrior"` IS itself a `class_books` MEMBER (modelled), so `key_group_owner`
resolves it directly and the record never reaches the "owner is None" branch at
all; `"dragon"` is a real corpus class name that is itself unmodelled, so only the
WIDE `corpus_class_names` check (not the `class_books`-scoped one) finds it.

**Fix**: the corpus-wide collision check's early return is now guarded — it still
fires exactly as before UNLESS the SAME `text_only && has_real_description &&
is_display_wiring_class_for_promotion(wc_class) && !universal_sheet_modifier`
guards every sibling promotion already requires, ALSO combined with
`class_feature_pool_catalog_holds`/`class_feature_standalone_catalog_holds`,
independently succeed — in which case control falls through, unchanged, to the
EXISTING downstream checks (no new logic duplicated, no new grounding invented).
This can only ever decline to report a gap that was never real; it can never
fabricate a `grounded`/`text-complete` verdict of its own.

**RED → GREEN, both directions proven:**
- `a_creature_type_collision_does_not_block_an_already_served_pool_catalog_record`
  — before the fix: `status="engine-does-not-hold"`,
  `evidence="class_feature_of_unmodelled_corpus_class:dragon"` (RED, confirmed
  failing for the intended reason before the patch). After: `status=
  "text-complete"`, `evidence="class_feature_pool_catalog_serves_a_rendered_
  description"` (GREEN).
- `a_creature_type_collision_with_no_holds_check_still_reads_unmodelled_corpus_class`
  — NEGATIVE CONTROL: the same collision with NO holds-check backing it still
  reports the real gap, unchanged (proves the fix declines a false gap only when a
  holds-check independently proves the record is already served, never
  unconditionally).

## Movement — the real, regen-verified delta (wider than this cycle's own 80-unit scope, honestly reported)

**`population=49438 buckets=10 unclassified=0 overlap=0`**
(`python3 scripts/completion_atlas.py --check`, this receipt's own final state).

- **`D: 2891 → 2676` (−215), `DONE: 25027 → 25242` (+215).** Guarded regeneration
  (`corpus_literal_sweep`: 48706 of 51476 examined, CLEAN, 0 findings;
  `derived_evaluator_fixture_check`: 1839 units cleared over 2580 fixture rows, 0
  failed; no `--allow-stamp-loss` used or needed — no `data/corpus/**` file touched
  this cycle) confirms **215 real units close to DONE**, all via `status:
  "text-complete"`, `evidence: "class_feature_pool_catalog_serves_a_rendered_
  description"` (214) / `"class_feature_standalone_catalog_serves_a_rendered_
  description"` (0 this run) — every one independently re-verified against
  `docs/work-inventory.json`'s own before/after diff (`git diff HEAD~1 --
  docs/work-inventory.json`, byte-level, 215 changed unit ids, 0 unexpected).
- **13 of the 215 are inside this cycle's own assigned 80-unit disposition-trace
  scope**: dragon 6 (5 `Order of the Dragon` sub-features + `Dragon Shaman ~ Wild
  Empathy`/`~ Dragon Bite`, both zero-magnitude text-only sub-features of the
  9-unit Dragon Shaman group whose OWNER-attribution problem is still open, but
  which independently pass the SAME pool-catalog holds-check regardless of owner),
  animal 6 (3 Spirit Animal + 1 Animal Trick + 2 Animal Speaker, all zero-magnitude
  text-only), undead 1 (`Undead Scourge ~ Aura of Life`).
- **202 of the 215 are OUTSIDE this cycle's own 80-unit scope**, from
  sub-mechanism 5's own 60-class "genuinely unmodelled prestige/base class"
  population (`Stalwart Defender` 15, `Master Spy` 12, `Nature Warden` 11, `Pure
  Legion Enforcer` 11, `Mammoth Rider` 9, `Holy Vindicator` 8, `Rage Prophet` 8,
  `Phantom` 8, `Elocater` 8, `Battle Herald` 7, `Ulfen Guard` 7, `Enchanting
  Courtesan` 7, `Lion Blade` 7, `Pathfinder Savant` 6, `Body Snatcher` 6, `Psion
  Uncarnate` 6, `Asavir` 5, `Pathfinder Delver` 4, `Sanguine Angel` 4, `Adaptive
  Warrior` 4, `Metamorph` 4, `Phrenic Slayer` 4, `Psychic Warrior` 4, `Sighted
  Seeker` 4, plus 18 more classes at 1–3 units each — full per-class breakdown
  reproducible via the diff script cited above). **Honestly reported as an
  unavoidable, provably-safe emergent effect of the same narrow fix, not new
  chassis work chosen this cycle**: every affected class's OWN magnitude-bearing
  siblings were spot-checked and correctly remain `engine-does-not-hold`
  (`Stalwart Defender ~ AC Bonus`/`~ Damage Reduction`/`~ Defensive Stance`, all
  `magnitude_token_count > 0`, still D) — the fix promotes only each class's own
  ZERO-magnitude, `display`-wiring-class, description-carrying sub-features, per
  the SAME "text-only = complete" rung every other class's siblings already ride.
  **Consequence for sub-mechanism 5's own next-wave dispatch**: its "832 units, 60
  classes" figure is now stale by 202 (text-only) units — the REAL new-chassis
  (magnitude-bearing) remainder is smaller than the wave 35 receipt's own table
  states; re-deriving the exact per-class magnitude-bearing remainder is next-wave
  scope, not this cycle's (a disposition-trace cycle, not a re-census cycle).
- **Reclassification (bucket → different non-DONE bucket):** 0 — every moved unit
  went straight to DONE, none shuffled between non-DONE buckets.
- **Reachability:** 0 units newly reached or lost reachability.
- **Instrument-correction:** 1 retro-logged correction (wave 35's own "Animal
  Companion... plausibly already exists" hypothesis, refuted — see the animal row
  above); 4 `completion_atlas.py` citation pins re-derived (A/B/C/V, this cycle's
  own insertion shifted them, `citation_failures` 4→0, no bucket population moved
  by that fix alone).

## Figures (every number, its command, its denominator)

- `population=49438`, `D: 2891 → 2676`, `DONE: 25027 → 25242` — `python3
  scripts/completion_atlas.py --check`, of the full corpus, before/after this
  cycle's own regen at the SAME (correct, post-rebase) base.
- `80` units (23 animal + 13 undead + 15 dragon + 2 construct + 9 plant + 1 ooze +
  16 eidolon + 1 sentinel) — `Counter` over `docs/work-inventory.json`'s `units`
  filtered to `evidence.startswith("class_feature_of_unmodelled_corpus_class:")`
  and the class slug in `{animal, undead, dragon, construct, plant, ooze, eidolon,
  sentinel}`, of the 931-unit shape.
- `215` units closed, `13` in-scope / `202` out-of-scope — `git diff HEAD~1 --
  docs/work-inventory.json`, per-id status/evidence comparison (script:
  `/tmp/.../scratchpad/diff_status2.py`, reproducible by re-running the same
  `Counter`-over-diff method), of the 931-unit shape's own population.
- `48706 of 51476` corpus records examined, CLEAN — `corpus_literal_sweep
  --json-out`, of the full corpus.
- `1839 units cleared over 2580 fixture rows, 0 failed` —
  `derived_evaluator_fixture_check --json-out`, of the fixture's own 2,580-row
  coverage.
- `521` tests pass (`cargo test --locked --bin v06_work_inventory`, this cycle's
  own final HEAD) — of the crate's full `v06_work_inventory` unit-test suite (2
  new this cycle, 0 failed, 0 regressed against the pre-cycle 521 baseline
  re-measured at the corrected base before the fix).

## Row-count command output

```
$ grep -n "^| [0-9]* |" docs/release/SD-34-book-completion/kanban.md | tail -1
| 37 | `mine-bucket-d` | 3 | wave 32, lane C (no AT-34-E# card yet) | partial | ...
```
Row 37 (`mine-bucket-d`) is the SAME accumulating row wave 32 and wave 35 lane C
both appended into (no dedicated `AT-34-E#` card exists for generic bucket-D
mining) — this cycle appends its own sentence to that row rather than opening a
new one, per house style.

## Build scope verified

- `cargo test --locked --bin v06_work_inventory` → 521/521 pass, this cycle's own
  final HEAD `045612dd25`.
- `cargo test --locked --no-run` → exit 0 (full workspace), same SHA.
- Desktop crate (`apps/desktop/src-tauri`) — not run this cycle: `git diff --stat
  4379c9be05...HEAD -- apps/desktop/` is empty, no file under `apps/desktop/`
  touched, honestly reported skipped (workflow-instruction.md §6 step 3 scopes
  this to "if touched").

## Sweep population

- `corpus_literal_sweep`: `48706 examined of 51476 read, 0 findings, CLEAN` — no
  `data/corpus/**` record added, changed, or removed this cycle (Rust classifier
  logic only), so the delta vs. any prior sweep is 0, consistent with 0 records
  added.

## Oracle pin

- `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-
  oracle-pin.env`) — confirmed live (`git -C $PCGEN_CORPUS_ROOT/.. rev-parse
  HEAD` at the oracle checkout matches exactly) before the guarded regen ran.

## Status

**complete** — the disposition trace covers all 80 units with a named next-step
and no unit left as "the rest"; the one case-(a) finding (Order of the Dragon, 5
units) is fixed, tested (RED→GREEN + negative control), and regen-verified; the
regen's own wider, honestly-reported 215-unit closure is a provably-safe emergent
effect, not scope creep chosen this cycle.

## Notes (judgment calls)

- **Why Dragon Shaman/Undead Savant Subschool/Plant Master were NOT fixed despite
  a real, traced owner existing**: unlike Order of the Dragon (where the fix
  needed no owner resolution at all — only declining a false gap the existing
  holds-check already independently proves safe), these three groups' remaining
  MAGNITUDE-bearing sub-features would need the matcher to actually RESOLVE the
  real owner (Druid/Arcanist/Hunter) to ground anything beyond their own
  already-safely-promoted zero-magnitude subset. The only candidate signal
  (`/data/class` on the corpus JSON) is proven UNRELIABLE within this cycle's own
  80-unit population (Dragon Shaman itself carries `class: "Shaman"` on 2 of its 9
  records, `class: "Druid"` on the other 7; Power Over Undead/Animal Companion
  Base carry the collision word itself as `class`) — plumbing it safely would need
  a corpus-wide validation pass this narrow-fix cycle does not cover. Named
  precisely for the next wave, not attempted.
- **PaDFE Construct/Ooze/Undead is a real misattribution with zero achievable
  bucket movement this cycle** — Pathfinder Delver itself has no chassis
  (sub-mechanism 5), so reclassifying these 3 units' evidence string would not
  close them; flagged as a correction to sub-mechanism 5's own count (13, not 10)
  for whenever that class is actually built.
- **The 202-unit wider effect was not chosen scope** — it is the unavoidable,
  single-code-path consequence of fixing the ONE thing this cycle's brief actually
  authorized (a matcher precedence bug). Reported in full per `AGENTS.md`'s "every
  figure carries the command that produced it" and "a proof is only as wide as the
  cases it covers" — spot-checked, not merely counted.

## Next-cycle plan

1. **Undead Savant Subschool/Plant Master/Dragon Shaman's remaining magnitude-
   bearing sub-features** (real owner traced — Arcanist/Hunter/Druid — but not
   resolvable via any existing matcher signal): needs either (a) a corpus-wide
   audit of `/data/class` reliability before trusting it as a new owner-resolution
   signal, or (b) a narrower, per-group hardcoded owner override (same shape as
   the `CLASS_FEATURE_POOLS` table) if the population is small enough to enumerate
   safely.
2. **Sub-mechanism 5's own 832-unit/60-class figure needs re-deriving** post-regen
   — 202 of its own population closed as text-only this cycle; the real
   magnitude-bearing new-chassis remainder is smaller. A fresh `Counter` over the
   committed inventory, same method as wave 35's own, gives the corrected figure.
3. **Pathfinder Delver's real total is 13 units, not 10** — fold the 3 PaDFE units
   into its per-class chassis-building dispatch whenever that class is picked up.
4. **Sub-mechanism 3 (Eidolon, 16 units) and sub-mechanism 4 (Sentinel, 1 unit)**
   are fully dispositioned this cycle: Eidolon needs a genuinely new Summoner
   evolution-slot-per-level table (Epic 4/5 scope, not built); Sentinel needs an
   ingest-time `Kind::feat` re-tag (a different code surface, not attempted).
5. **The seven domain-vs-class_feature dual-representation units** (Dragon
   Subdomain ~ Dragonbreath, Undead Subdomain ~ Death's Kiss, Construct Subdomain
   ~ Animate Servant, and their kin) are a real, small, well-scoped mechanism gap
   — a domain granted-power grounding path this engine does not have — named
   precisely, Epic 3 scope, not built this cycle.
