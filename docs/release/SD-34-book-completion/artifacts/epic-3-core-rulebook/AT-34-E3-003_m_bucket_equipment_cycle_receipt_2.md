# Cycle — Epic 3 (Core Rulebook to zero) / AT-34-E3-003 (bucket `M` — EQUIPMENT sub-causes, cycle 2)

- **Commit SHA:** `0519220786` (checkpoint of the code+tests; this receipt's own commit follows
  on top of it). Rebase base at cycle start: `origin/tranche/14`'s tip, `b38a8a399c` (commit
  `4d27d70551`'s successor chain through
  `fix(sd34): AT-34-E3-002 -- ground Cleric Domain via the generic pool-group pass`).

- **Continuation of, not a duplicate of,**
  `AT-34-E3-003_m_bucket_equipment_cycle_receipt.md` (this repo's own prior cycle, landed
  commit `7147fd86ab`, already merged into `tranche/14` before this cycle started). That cycle
  widened `equipment_key_is_wired` to consult `damage_total::resolve_base_damage_dice` for the
  same-line shape (`equipment_table_entry_with_corpus_magnitude`, 276→262, **14 real closures**)
  and confirmed, by reading the real corpus, that **0 of the 147 closure-only alias units**
  (`equipment_own_line_has_no_magnitude_but_closure_wiring_class_does`) resolved to a
  `DAMAGE:`-bearing base record under the resolver's THEN-current behavior. This cycle's brief
  explicitly warned not to trust an inherited reason without re-deriving it, so every claim in
  that prior receipt was independently re-checked against live corpus records before this cycle
  wrote a line of code (below).

- **Files touched:**
  - `src/rules_core/damage_total.rs` — `resolve_base_damage_dice` widened with a one-hop
    `BASEITEM:` fallback (`base_item_damage_dice_token`, new private fn): when a record's own
    `tokens` carry no `DAMAGE:` token, the function now reads the record's `BASEITEM:` token (a
    real PCGen alias convention: "this record's stats inherit from BASEITEM's record") and
    re-resolves THAT key through the SAME `equipment_id_resolve` call the function already makes
    for its primary lookup — not a second resolution mechanism, a second call to the existing
    one. Four new tests: `baseitem_alias_chases_to_its_base_records_damage_dice` (fixture, pins
    the mechanism), `baseitem_naming_an_unresolvable_record_stays_none` (negative control — no
    panic, no fabrication), `a_records_own_damage_token_wins_over_its_baseitem` (a record's own
    token always wins; the fallback only fires when the record's own row has nothing).
  - `src/bin/v06_work_inventory.rs` — one new test,
    `equipment_probe_promotes_a_baseitem_alias_via_its_bases_damage_token`, proving the probe
    (`equipment_key_is_wired`, unchanged this cycle — it already calls
    `resolve_base_damage_dice`) now promotes the real, on-disk `Crossbow (Light)` record end to
    end against the live `core_rulebook` corpus.
  - This receipt, `docs/release/SD-34-book-completion/progress.md`,
    `docs/release/SD-34-book-completion/kanban.md`.
  - **Deliberately NOT touched/committed:** `docs/work-inventory.json` and
    `artifacts/epic-1-atlas/completion-atlas.json` — this wave's dispatch brief reserves their
    regeneration to a single shared end-of-wave cycle, same rule the prior equipment cycle
    followed. Figures below come from a local three-pass regen
    (`corpus_literal_sweep` → `derived_evaluator_fixture_check` → `v06_work_inventory`, release,
    no `--allow-stamp-loss`), read, then `git restore`-d before the final commit.

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — `git diff --unified=0 <rebase-base>...HEAD
  -- src/rules_core/ src/bin/ scripts/oracle_harness/ artifacts/epic-3-core-rulebook/
  ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  → zero matches on this cycle's own diff (`src/rules_core/damage_total.rs` +
  `src/bin/v06_work_inventory.rs` only).

- **Wired-integration audit result:** `OK_NO_TOKENS` — same command, second pattern
  (`\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b`) → zero matches on this
  cycle's own diff.

- **Acceptance criterion (verbatim, `epic-breakdown.md`):** "AT-34-E3-003 — buckets M, V, D, U, X
  close" (per-bucket, atlas reporting zero for `core_rulebook`, movement in four buckets). This
  cycle is a further slice of the same territory the prior equipment cycle worked
  (`equipment_table_entry_with_corpus_magnitude` +
  `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does`) — the criterion as a
  whole stays open (M's other eight sub-causes and buckets V/D/U/X are untouched by design, per
  this wave's dispatch brief's no-collision territory boundary: sibling lanes own
  `ability_content`/`race_trait_generic`/`template_content`/choice-wiring/explanation-id work).

- **Re-derived at cycle start (never trusting the inherited figure):** `python3 scripts/completion_atlas.py --book core_rulebook --check`
  against rebased HEAD (`b38a8a399c`) → `core_rulebook` M = **958** (matches the prior equipment
  cycle's own post-fix figure exactly — its 14 closures are already reflected in the committed
  `docs/work-inventory.json`, confirmed live, not assumed). Split (re-derived via the same
  `json.load`/`Counter` one-liner against `docs/work-inventory.json`, filtered
  `book=='core_rulebook'`, `status=='ingested-magnitude'`):
  `ability_content_table_holds_record_magnitude_not_yet_computed` 217,
  `equipment_table_entry_with_corpus_magnitude` **262** (down from 276 — the prior cycle's 14
  closures, confirmed),
  `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does` **147** (unchanged — the
  prior cycle's own claimed 0 closures on this shape, confirmed),
  `race_trait_generic_table_holds_record_magnitude_not_yet_computed` 119,
  `template_content_table_holds_record_magnitude_not_yet_computed` 96,
  `in_catalog_with_corpus_magnitude_but_no_observed_consumer` 47,
  `domain_content_table_holds_record_magnitude_not_yet_computed` 34,
  `skill_content_table_holds_record_magnitude_not_yet_computed` 19,
  `spell_list_entry_with_resolved_level` 15,
  `race_trait_states_a_universal_sheet_modifier_pending_compute` 2. **This cycle's territory
  (the same two EQUIPMENT sub-causes) is 262 + 147 = 409 at cycle start** — the prior cycle's own
  stated remainder, confirmed exactly, not carried forward unchecked.

- **Independent re-investigation of the 409 (real corpus reads, not assumption) — this is the
  cycle's main deliverable.** Every one of the 409 units' real, on-disk corpus JSON record was
  loaded (key→file map built by walking `data/corpus/core_rulebook/**` and matching
  `data.key`), and classified by its actual `raw_bonus_chains`/`raw_tokens`/`description` shape.
  Script: a one-off classifier (not committed — a diagnostic read, not a compute-path change),
  reproducible by: for each unit, load its corpus JSON by `corpus_key`; if it (or, new this
  cycle, its `BASEITEM:`-chased base record) carries a `CHOOSE:` token or a `%CHOICE` qualifier
  → `CHOICE_GATED`; else if it carries no `raw_bonus_chains` at all → prose-magnitude or chassis
  plumbing (split by whether `description` carries digits); else grouped by the bonus chain's
  first qualifier (`VAR`, `ITEMCOST`, `EQM`, `EQMWEAPON`, `WEAPON`, …).

  | # | Sub-shape (mechanism, not a corpus field) | Population | Real disposition, checked against actual records |
  |---:|---|---:|---|
  | 1 | `CHOICE_GATED` — `%CHOICE`/`CHOOSE:` present (e.g. `BNS_AC_DEFL`, `BNS_AC_INSI`, `BNS_AC_LUCK`, `BNS_SAV_*`, `BNS_ENHC_AB`, `MAX_GNL`) | 99 | Confirmed **sibling lane's territory** (choice-selection/explanation-id wiring, explicitly off limits this cycle) — the magnitude genuinely depends on a player choice this probe has no mechanism to observe. |
  | 2 | `VAR` bonus chain, no choice (e.g. `Amulet of Mighty Fists` → `DEFINE:MightyFistValue`, `Book of Infinite Spells` → `DEFINE:NegLevels`, `Phylactery of \[Negative\|Positive\] Channeling` → `DEFINE:ClericChannelNegativeEnergyDice`) | 121 | Real, player-facing mechanical effects (confirmed: Phylactery of Negative Channeling genuinely widens a Cleric's Channel Energy dice pool — `src/rules_core/level_up/cleric.rs` already computes that pool, but from character level, never from an equipped item) — but each requires wiring a SPECIFIC cross-subsystem interaction (equipment ↔ class-feature compute) that does not exist as a generic, already-callable function today. **New engineering, bucket-B-shaped**, not a probe widening — refines the prior receipt's "not player-facing" framing (checked against real records: several of these ARE player-facing; what they lack is cross-subsystem wiring, not mechanical relevance). |
  | 3 | No bonus chain, no real prose magnitude — internal PCGen chassis/plumbing (e.g. `CHARGED_ITEM_*`, `A_1USEM[A\|E\|I]`, `Crossbow (Light)`-shaped `BASEITEM:` aliases whose base carries no checked field) | 100 | Genuinely empty of mechanical content this probe's promotion ladder could ever ground, OR (the `BASEITEM:` alias shape) resolved and found to carry no `DAMAGE`/8-field match on the base either. **1 of these 100 (`Crossbow (Light)`) is this cycle's own real closure** — see below. |
  | 4 | No bonus chain, real prose magnitude only (potions, elixirs, named artifacts — e.g. `Potion of Bull's Strength`, `Elixir of Swimming`, `Holy Avenger`, `Nine Lives Stealer`, special ammunition like `Arrow (Slaying)`) | 71 | The item's real mechanical value (a caster-level-scaled potion effect, a named artifact's stated bonus) lives only in `description`/`DESC:` prose, never a structured token this or any existing probe reads. A new description-linked probe is real new engineering (matches the prior receipt's "named artifacts" note, now with an exact, checked population). |
  | 5 | `ITEMCOST` only (material/masterwork/quality cost formulas, e.g. `MWORKT`, `Material ~ Cold Iron`) | 9 | Cost-plumbing, not a mechanical effect a character sheet renders. No compute-path gap; there is nothing to compute. |
  | 6 | `EQM` (utility fields with no `ResolvedEquipmentEffect` slot — `WEIGHTADD`, `HANDS`, e.g. `Holy Symbol (Silver)`, `Special Ability ~ Animated ~ Shield`, `Special Quality ~ Locked Gauntlet`) | 3 | Real, fixed, non-choice magnitudes — but weight/hands-free are subsystems this engine's `ResolvedEquipmentEffect` does not model at all today. New field + new subsystem, not a probe widening. |
  | 7 | `EQMWEAPON` range/crit modifiers with no compute-path field (`RANGEMULT`, `RANGEADD`, `CRITRANGEDOUBLE` — `Special Ability ~ Distance ~ Ranged`, `~ Throwing ~ Melee`, `~ Keen ~ Weapon`) | 3 | Checked against `damage_total.rs`: `resolve_critical_threat_range`/`resolve_critical_multiplier` read a WEAPON's own `CRITRANGE`/`CRITMULT`, never an EQUIPMOD's amplifier token layered onto whatever weapon it is attached to (equipment-modifier composition) — no already-wired consumer exists for that composition. Confirmed new engineering, not a probe widening. |
  | 8 | `ITEMCOST,WEAPON` material/quality plumbing (`Material ~ Alchemical Silver`, `Special Quality ~ Broken ~ Weapon`) | 2 | Same as row 5. |
  | 9 | `WEAPON` fixed effect with no field (`Special Ability ~ Speed ~ Weapon`) | 1 | Same shape as row 7 — a weapon special ability the engine has no slot for. |

  **Sum: 99+121+100+71+9+3+3+2+1 = 409**, matching the cycle-start population exactly (checked
  by script, not by hand addition).

- **This cycle's real closure, found by disproving (narrowly) the prior cycle's stated
  reason — logged as a `correction` retro event.** The prior receipt stated "none of the 147
  closure-only alias rows in this population happen to resolve to a base record that itself
  carries a `DAMAGE:` token" — TRUE under the resolver's behavior at the time (`equipment_id_resolve`
  never chased a `BASEITEM:` token), but an instrument limitation, not a genuine absence. Reading
  the real corpus record for `Crossbow (Light)` (`core_rulebook/cr_equip_arms_armor.lst` line
  321): its own row carries `BASEITEM:Light Crossbow (Base)` and no `DAMAGE:` token; the real
  base record `Light Crossbow (Base)` (`data/corpus/core_rulebook/equipment/arms_armor/light_crossbow_base.json`)
  carries `DAMAGE:1d8`. Widening `resolve_base_damage_dice` with a one-hop `BASEITEM:` chase
  through the SAME `equipment_id_resolve` call (not a new resolution mechanism) closes this real
  gap — and is a genuine **player-facing bug fix independent of the M-bucket accounting**:
  before this cycle, a player who selected `Crossbow (Light)` on their character sheet got
  `resolve_weapon_damage_breakdown` → `None` (no `WeaponDamageBreakdown` at all) for a real,
  ordinary CRB weapon.

  **Population check, corpus-wide, before writing code — done in two passes, the first of
  which underclaimed and was corrected by the second (named here, not hidden).** First pass
  scoped to the 409-unit `ingested-magnitude` (bucket-`M`) territory only: exactly **1** unit
  (`Crossbow (Light)`) carries a literal `BASEITEM:` token resolving to a `DAMAGE:`-bearing
  base, corpus-wide. **Second pass, run against BOTH evidence strings at ANY status** (not only
  `ingested-magnitude`) after the fix was already written and the real closure observed: finds
  **17** matches, all `core_rulebook` — the same 1 `ingested-magnitude` unit, plus **16** already
  `literal-verified` (bucket `V`) weapons whose evidence string was stale
  (`equipment_own_line_has_no_magnitude_but_closure_wiring_class_does`, an `M`-shaped reason)
  even though a separate, unrelated mechanism had already verified them by proxy:
  `Axe (Throwing)`, `Battleaxe`, `Club`, `Dart`, `Falchion`, `Greataxe`, `Greatclub`,
  `Hammer (Light)`, `Handaxe`, `Pick (Heavy)`, `Pick (Light)`, `Sap`, `Scimitar`, `Scythe`,
  `Sickle`, `Warhammer` — each a real `BASEITEM:` alias resolving to a real `DAMAGE:`-bearing
  base (e.g. `Club` → `BASEITEM:Club (Base)`, confirmed live against
  `data/corpus/core_rulebook/equipment/arms_armor/club.json`). **These 16 move no bucket** —
  bucket `V` before, bucket `V` after (`decisions.md`'s bucket definitions: `literal-verified`
  is `V`, not `DONE`) — but their evidence string is now accurate instead of stale, a real
  correctness fix under `decisions.md §12` L1 ("a field's name is not its meaning" — here, an
  evidence string was not its own reason) reported as a **reclassification**, not a closure.
  **Bucket-`M` closure remains exactly 1** (only `Crossbow (Light)` was ever in bucket `M`); no
  other book carries this exact `BASEITEM:`+`DAMAGE:` pattern in either evidence string at any
  status (checked corpus-wide, both passes). (The base records also carry `CRITRANGE`/`CRITMULT`
  tokens `resolve_critical_threat_range`/`resolve_critical_multiplier` could equally chase via
  the identical one-hop pattern — **discovered, not fixed this cycle**: those two functions are
  not called by `equipment_key_is_wired` at all, so widening them moves no bucket count, and is
  out of this cycle's declared scope; named here so a future cycle inherits the finding rather
  than rediscovering it.)

- **Figures + their re-derive commands (post-fix, from this cycle's own local three-pass
  regen — `corpus_literal_sweep --json-out` → `derived_evaluator_fixture_check --json-out` →
  `CORPUS_LITERAL_SWEEP_REPORT=... DERIVED_FIXTURE_CHECK_REPORT=... cargo run --release --bin
  v06_work_inventory`; the plain regen refused to write with "this run would drop 9564 of 9564
  verification stamp(s)" until both reports were supplied — a real fail-closed guard, not this
  cycle's defect, cleared per its own printed instruction):**
  - `core_rulebook` bucket `M`: **958 → 957** (**−1, this cycle's own closure**). Re-derive:
    `python3 scripts/completion_atlas.py --book core_rulebook --check` (against the local regen;
    the committed `docs/work-inventory.json` at HEAD still reads 958 until the wave's shared
    regen cycle lands this cycle's source change).
  - `core_rulebook` DONE: **4,344 → 4,383** (+39 total in this regen — **+1 is this cycle's own
    closure; +38 is the ALREADY-COMMITTED, ALREADY-RECEIPTED `AT-34-E3-002` Cleric Domain fix**,
    commits `b38a8a399c`/`50c10d5cc3`, which this shared local regen incidentally also captured
    since it had not yet been regenerated into `docs/work-inventory.json` either — confirmed by
    id: all 55 `class_feature` status changes in the full before/after diff are Cleric-Domain
    keys (`*_domain_*`), 38 landing on `grounded` (DONE) and 17 on `literal-verified` (bucket
    `V`, not DONE — `core_rulebook` V: 87 → 104, +17, entirely this fix, 0 from this cycle's
    own change). Same command.
  - `core_rulebook` `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does`:
    **147 → 146** (−1, `Crossbow (Light)`). Re-derive: the same `json.load`/`Counter` one-liner
    against the local regen output.
  - `core_rulebook` `equipment_table_entry_with_corpus_magnitude`: **262 → 262** (0 — this
    cycle's fix only reaches the closure-only shape; no same-line record in this territory
    carries a `BASEITEM:` token, confirmed above).
  - Corpus-wide bucket `M`: **4,966 → 4,965** (**−1**, this cycle's own closure, isolated by
    re-running `completion_atlas.py --check` against the PRE-regen inventory directly — same
    command, different input file — to separate this cycle's delta from the co-mingled Cleric
    Domain fix's own, out of population 49,438 (unchanged — 0 added, 0 removed). Re-derive:
    `python3 scripts/completion_atlas.py --check`.
  - Corpus-wide DONE: **24,314 → 24,353** (+39; same +1/+38 split as `core_rulebook` above — no
    other book's DONE count moved from either fix). Corpus-wide V: **262 → 279** (+17, entirely
    Cleric Domain's own new bucket-`V` members, 0 from this cycle's change — this cycle's 16
    reclassified weapons were already counted in the 262 V-before figure).
  - **Whole-corpus before/after diff by unit id:** 49,438 units before, 49,438 after, **0
    added, 0 removed, 72 changed** — decomposed by kind: 55 `class_feature` (Cleric Domain,
    not this cycle — see above) + 17 `equipment` (this cycle's own `BASEITEM:` widening: **1
    real closure**, `core_rulebook:equipment:crossbow_light`
    `ingested-magnitude → grounded`, evidence `equipment_effect_probe_observed_computed_delta`;
    **16 reclassifications**, all `literal-verified → literal-verified` — same evidence
    transition — see the Population-check section above for the full 16-name list). 55+17=72,
    matching exactly.
  - `corpus_literal_sweep`, this cycle's own baseline run (release, this cycle's own commit's
    corpus state, no `data/corpus/**` file touched): **48,708 examined of 51,482 read**, CLEAN,
    0 findings — unchanged from the pre-cycle figure (`decisions.md §12` L8 — 0 delta expected
    because 0 corpus records were added or changed; only `src/rules_core/damage_total.rs` and
    `src/bin/v06_work_inventory.rs` changed).
  - `derived_evaluator_fixture_check` (release, same corpus state): **1,839 units cleared over
    2,580 fixture rows, 0 failed, 0 not ingested**.

- **Row-count command output (this cycle's own local regen, the artifact this cycle's status is
  set from):**
  ```
  $ python3 -c "
  import json, collections
  d = json.load(open('docs/work-inventory.json'))
  units = d['units']
  m = [u for u in units if u.get('book')=='core_rulebook' and u.get('status')=='ingested-magnitude']
  print('core_rulebook M count:', len(m))
  print(collections.Counter(u.get('evidence') for u in m))
  "
  core_rulebook M count: 957
  Counter({'equipment_table_entry_with_corpus_magnitude': 262,
  'ability_content_table_holds_record_magnitude_not_yet_computed': 217,
  'equipment_own_line_has_no_magnitude_but_closure_wiring_class_does': 146,
  'race_trait_generic_table_holds_record_magnitude_not_yet_computed': 119,
  'template_content_table_holds_record_magnitude_not_yet_computed': 96,
  'in_catalog_with_corpus_magnitude_but_no_observed_consumer': 47,
  'domain_content_table_holds_record_magnitude_not_yet_computed': 34,
  'skill_content_table_holds_record_magnitude_not_yet_computed': 19,
  'spell_list_entry_with_resolved_level': 15,
  'race_trait_states_a_universal_sheet_modifier_pending_compute': 2})
  ```
  Sum 262+217+146+119+96+47+34+19+15+2 = **957**, matching
  `python3 scripts/completion_atlas.py --book core_rulebook --check`'s own live `M: 957`
  exactly. This cycle's own sub-cause of the two:
  `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does` 147→**146**, −1 exactly,
  this cycle's own real closure.

- **Build scope verified (at the final commit SHA):**
  - `cargo test --locked --lib rules_core::damage_total::` — **33/33 pass** (30 pre-existing + 3
    new: `baseitem_alias_chases_to_its_base_records_damage_dice`,
    `baseitem_naming_an_unresolvable_record_stays_none`,
    `a_records_own_damage_token_wins_over_its_baseitem`). Confirmed RED first
    (`baseitem_alias_chases_to_its_base_records_damage_dice` failed with
    "Crossbow (Light) must chase BASEITEM to its base record's DAMAGE token" before the fix, for
    the intended reason — the fallback did not exist yet), then GREEN after.
  - `cargo test --locked --bin v06_work_inventory` — **454/454 pass** (453 pre-existing + 1 new:
    `equipment_probe_promotes_a_baseitem_alias_via_its_bases_damage_token`, proven against the
    real on-disk `core_rulebook` corpus).
  - `cargo test --locked --no-run` (full workspace, `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-003`)
    — **exit 0**.
  - `apps/desktop/src-tauri` (separate cargo workspace,
    `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-003-desktop`): `cargo test --locked --no-run
    --manifest-path apps/desktop/src-tauri/Cargo.toml` — tested explicitly because
    `damage_total::resolve_base_damage_dice` is transitively reachable from the desktop crate's
    `character_hub.rs` (`codex = { path = "../../.." }`); the change widens an existing
    function's return value (strictly more `Some` cases, same type), no signature change.
    **Exit 0**.
  - `python3 scripts/completion_atlas.py --check` (local regen) — confirmed run, exit 0
    (`unclassified=0 overlap=0`), `citation_failures=0`, `missing_clearing_mechanisms=0`,
    `stale_derived_at=False` (no line-shifting insertion this cycle — the new code was added
    entirely inside `resolve_base_damage_dice`'s existing block plus one small new private fn in
    the SAME file region, not before any cited line in `v06_work_inventory.rs` or
    `completion_atlas.py`'s own citations — confirmed: this cycle added 0 lines to
    `v06_work_inventory.rs` above any `BUCKET_DEFINITIONS` citation line; the one new test is
    appended after `equipment_probe_promotes_a_real_weapon_with_a_real_damage_token`, inside the
    `#[cfg(test)]` module, below every cited production-code line).

- **Sweep population:** no `data/corpus/**` records added or regenerated — only
  `src/rules_core/damage_total.rs` and `src/bin/v06_work_inventory.rs` (test-only addition)
  changed. `corpus_literal_sweep`: **48,708 examined of 51,482 read**, CLEAN, 0 findings,
  unchanged before/after (`decisions.md §12` L8 — 0 delta expected, 0 delta confirmed).

- **Denominator gate against this package:**
  `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` →
  `files_checked=15 violations=8`, all 8 pre-existing verbatim-quoted corpus prose in
  `progress.md` (`FRT_HVY`'s "75% chance..."), already flagged by prior cycles; this cycle's own
  new prose (this receipt, the `progress.md` entry, the `kanban.md` row addendum) contains no
  bare percentage.

- **Oracle pin:** `PCGEN_ORACLE_SHA` per `scripts/pcgen-oracle-pin.env`
  (`7f818006e371188e5717fd18d74d18a420747fc6`) — not consulted this cycle (no oracle-pinned
  corpus read; the `DAMAGE:`/`BASEITEM:` tokens this cycle reads come from the repo's own
  `data/corpus/core_rulebook/`, already ingested).

- **Status:** partial

- **Movement, four buckets:** closure (**1** unit, `core_rulebook:equipment:crossbow_light`,
  `ingested-magnitude → grounded`, genuinely newly promoted — a real, already-wired compute path
  widened by one hop, not a reclassification of the same evidence — this cycle's own, isolated
  from the co-mingled Cleric Domain regen noise above); reclassification (**16** units, all
  `core_rulebook` weapons already `literal-verified` both before and after — bucket `V`
  unchanged — evidence string corrected from the stale `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does`
  to the accurate `equipment_effect_probe_observed_computed_delta`, named individually above;
  **separately, 55 more units reclassified/closed this same regen are the ALREADY-COMMITTED
  `AT-34-E3-002` Cleric Domain fix, NOT this cycle's work** — named here only so the whole-corpus
  72-unit diff is fully accounted for, not claimed); reachability (0); instrument-correction (0
  — no citation line shifted; the retro `correction` event above documents the prior cycle's
  stated-reason being narrowly superseded by an instrument widening, not a citation re-pin).

- **Notes:**
  - **The exhaustive 9-shape classification above is this cycle's main deliverable, not the
    1-unit closure.** The prior cycle's own next-step #2 named exactly this gap: "a finer real-
    corpus categorization of the 409 ... would let a future cycle pick the cheapest remaining
    shape first — this cycle categorized at a coarse level ... but did not exhaustively classify
    every one of the 409." This cycle did exactly that, against real records, summing to exactly
    409 with a stated real disposition and mechanism for every row — no "the rest" bucket
    remains inside this cycle's territory.
  - **Why the closure is 1, not more, and why that is reported honestly rather than reframed.**
    The dispatch brief's own guidance ("Take the largest sub-cause you can finish end-to-end")
    was followed by measuring every candidate mechanism's real population BEFORE writing code
    (the `BASEITEM:`-chase population check, run corpus-wide, found exactly 1). No other
    already-wired-compute-path opportunity survived the same check (rows 1–2 and 5–9 above all
    require either a player choice this probe cannot observe, or a compute path/subsystem field
    that genuinely does not exist yet). A 1-unit real, tested, player-facing fix was still worth
    landing — it is free once found, correct, and the classification work that found it (and
    ruled out everything larger) is the real value delivered this cycle.
  - **Independent verification, not inheritance, of the prior cycle's own claims.** Every
    starting figure (972 pre-, 958 post-prior-cycle `core_rulebook` M; 262/147 this cycle's
    territory split) was re-derived from the live `docs/work-inventory.json` at this cycle's own
    rebase base before any code was read or written, per `decisions.md §12` L2. The prior
    cycle's `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does` = 0-closures
    claim was independently checked against real records (not merely re-quoted) and found
    accurate under the resolver behavior at the time — the retro `correction` event above
    reflects the instrument widening, not an error in that receipt.
  - **The dispatch's `origin/salvage/wave13-lane3` rescue branch was consulted per the brief's
    instruction, then correctly superseded.** Diffing `1ea93e99ce..origin/salvage/wave13-lane3`
    shows the identical `DAMAGE:`-token widening the ALREADY-MERGED `AT-34-E3-003_m_bucket_equipment_cycle_receipt.md`
    cycle landed (confirmed: `git merge-base --is-ancestor 10b003443d HEAD` → true, i.e. that
    exact fix is already on `tranche/14`). Nothing from the salvage branch was carried forward
    unverified this cycle — this cycle's own new work (the `BASEITEM:` chase and the 9-shape
    classification) does not appear on that branch at all.

- **Remainder — every unit in this cycle's two EQUIPMENT sub-causes, named by real mechanism, at
  HEAD (post this cycle's own local regen):**

  | Sub-cause (evidence string) | Population before this cycle | Closed this cycle | Population after | Real sub-shapes composing it (from the 9-row table above) |
  |---|---:|---:|---:|---|
  | `equipment_table_entry_with_corpus_magnitude` | 262 | 0 | **262** | choice-gated (sibling scope), VAR cross-subsystem, chassis/plumbing, prose-only, ITEMCOST/EQM/EQMWEAPON/WEAPON no-field shapes — exact per-shape split available in the 9-row table (it does not separately break out same-line vs. closure-only membership; both evidence strings are covered together there) |
  | `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does` | 147 | 1 | **146** | same shapes, minus the 1 `BASEITEM:`+`DAMAGE:` closure this cycle found and closed |

  Sum: 262 + 146 = **408**, this cycle's own closure (1) plus this remainder (408) = 409,
  matching this cycle's own start-of-cycle territory exactly.

  **Every other `M` sub-cause, and buckets V/D/U/X, are untouched by this cycle** (out of
  territory by the dispatch brief's own no-collision rule):

  | Bucket / sub-cause | `core_rulebook` population (pre-regen HEAD figure, unaffected by this cycle) |
  |---|---:|
  | `M` `ability_content_table_holds_record_magnitude_not_yet_computed` | 217 |
  | `M` `race_trait_generic_table_holds_record_magnitude_not_yet_computed` | 119 |
  | `M` `template_content_table_holds_record_magnitude_not_yet_computed` | 96 |
  | `M` `in_catalog_with_corpus_magnitude_but_no_observed_consumer` | 47 |
  | `M` `domain_content_table_holds_record_magnitude_not_yet_computed` | 34 |
  | `M` `skill_content_table_holds_record_magnitude_not_yet_computed` | 19 |
  | `M` `spell_list_entry_with_resolved_level` | 15 |
  | `M` `race_trait_states_a_universal_sheet_modifier_pending_compute` | 2 |
  | `V` (whole bucket) | 87 |
  | `D` (whole bucket) | 366 |
  | `U` (whole bucket) | 10 |
  | `X` (whole bucket) | 115 |

- **Next-cycle plan:**
  1. **This cycle's own remainder (408) needs real new engineering, not more probe-widening** —
     ranked by the 9-row table's population: rows 1 (99, choice wiring — sibling scope, will
     unblock once that lane lands), 2 (121, cross-subsystem equipment↔class-feature wiring —
     genuinely the next-largest, but each of its members likely needs its OWN specific bridge,
     not one generic function — worth costing per-VAR-target before committing), 4 (71,
     description-linked magnitude probe — a genuinely generic new probe shape, one build could
     plausibly close many of the 71 at once, the best next ROI candidate among the remainder).
  2. **Rows 6/7/9 (7 units total) are genuinely too small to justify new
     `ResolvedEquipmentEffect` fields on their own** — worth folding into whatever cycle builds
     the AC-bonus-equipmod compute path (bucket-B-shaped, a sibling epic's likely territory) if
     one is ever built, rather than a dedicated cycle.
  3. **The `CRITRANGE`/`CRITMULT` `BASEITEM:`-chase discovery** (this receipt's Figures section)
     is a real, cheap, correctness fix independent of any bucket count — worth a small follow-up
     cycle purely for the desktop-visible weapon crit display, not gated on this criterion.
  4. **The shared regen cycle** must pick up this cycle's source change (`resolve_base_damage_dice`'s
     `BASEITEM:` widening) the next time it commits `docs/work-inventory.json`'s three-pass
     pipeline — this cycle already ran that regen locally and confirmed the exact effect (not
     merely predicted): `core_rulebook` M 958→957,
     `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does` 147→146, DONE +1,
     corpus-wide M −1, DONE +1, plus 16 `core_rulebook` bucket-`V` evidence-string corrections
     (no bucket movement) — no other book carries this exact pattern (confirmed corpus-wide,
     both the `ingested-magnitude`-only and any-status passes above). The shared cycle should
     treat any mismatch from these exact figures as a real regression, not noise — same
     discipline the prior equipment cycle asked for and got confirmed on
     (`AT-34-E3-001_wave9_regen_receipt.md`).
  5. **`ability_content` (217, a sibling lane's territory) remains the largest overall `M`
     sub-cause.**
