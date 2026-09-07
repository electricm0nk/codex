# Cycle — Epic 3 (Core Rulebook to zero) / AT-34-E3-003 (bucket `M` — EQUIPMENT sub-causes, cycle 6)

- **Commit SHA:** `acd6a6a5e3` (source fix + tests, checkpointed and pushed before this receipt;
  this receipt's own commit follows on top, same pattern cycles 2–5 used).

- **Continuation of, not a duplicate of,** `AT-34-E3-003_m_bucket_equipment_cycle_receipt.md`
  (cycle 1), `_2.md` (cycle 2), `_3.md` (cycle 3), `_4.md` (cycle 4), `_5.md` (cycle 5), all
  already merged into `tranche/14` before this cycle started. Worktree opened stale (at the
  tranche cut, `ea2b3396f2`); `git fetch origin && git rebase origin/tranche/14` moved HEAD to
  the real tip, `4744c55bd0` (`AT-34-E3-002` cycle 8's progress/kanban row) — one more
  `AT-34-E3-003` cycle, two more shared regen waves (20/21) and `AT-34-E4-002` cycle 9 had
  landed since this wave's own dispatch brief was written. The brief's own figures
  (`core_rulebook` M = 972, split 276+147 = 423) were stale by construction; every figure
  below was independently re-derived at the real rebase base before any code was read or
  written (`decisions.md §12` L2).

- **Re-derived at cycle start (never trusting the inherited figure):**
  `python3 scripts/completion_atlas.py --book core_rulebook --check` at the rebase base →
  `core_rulebook` M = **811** (matches cycle 5's own post-fix figure and the wave-20 shared
  regen's own closing figure exactly, live-confirmed). Split, read directly off
  `docs/work-inventory.json`: `equipment_table_entry_with_corpus_magnitude` **163**,
  `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does` **99** — sum 262,
  cycle 5's own closing remainder, independently reproduced.

- **Cycle 5's own next-cycle plan (item 1) stated: "Nothing in this cycle's own two EQUIPMENT
  sub-causes is generically closable by an existing instrument without new subsystem work."**
  **That premise is wrong, and this cycle disproves it** (per this wave's own dispatch note:
  "this bundle has had a cycle disprove another's stated reason"). Retro `correction` event
  logged against cycles 2/3/5's shared "COST:/WT:-only means nothing to compute" disposition
  (`docs/retro/events/sd34-at-34-e3-003.jsonl`, `--verified-by` naming the direct module read
  below).

- **The real gap, found by reading the actual compute-path code, not re-running the census a
  sixth time:** `src/rules_core/encumbrance.rs`'s `compute_encumbrance` already reads a
  resolved equipment record's own `WT:`/`COST:` tokens (`weight_and_cost_from_record`) and is
  already a real, wired consumer — called by `pilot_compute_corpus::compute_pilot_with_corpus`
  and `contract::build_pilot_receipt`, and its own per-item weight/cost breakdown is already
  rendered on the real desktop character sheet's Gear tab
  (`apps/desktop/src-tauri/src/character_hub.rs`'s `CarriedItemDto { item_id, weight_lbs,
  cost_gp }`, `EncumbranceComputation`). `v06_work_inventory`'s equipment wiring probe
  (`equipment_key_is_wired`) never consulted it — the SAME probe-blind-spot shape cycles 3/4/5
  already found and fixed for `TEMPBONUS:`/`damage_total::resolve_base_damage_dice`, not a
  genuine absence of a compute path. Cycle 5's own qualifier-shape census correctly identified
  "182 of 189 (no-chain shape) carry only COST/WT" as a real corpus fact; its DISPOSITION —
  "nothing beyond price/weight to compute" — is what this cycle corrects.

- **Second, independent gap found while implementing the first:** a "thin" corpus record (no
  `raw_tokens` array at all — `corpus_loader.rs`'s own doc comment already names this
  ingestion shape) reconstructs an `EquipmentRecord` whose `tokens` list holds only a
  synthesized `KEY:` entry — `weight_and_cost_from_record` therefore returns `None` even
  though the record's own already-ingested `weight_lbs`/`cost_gp` top-level JSON fields carry
  the exact same data an enriched record's `WT:`/`COST:` tokens would. Confirmed corpus-wide
  (not sampled): of 4,470 enriched equipment/equipment_modifier records under
  `data/corpus/**/equipment/**/*.json` that carry both a `WT:` token and a `weight_lbs` field,
  **0 mismatches** — the top-level field is the same ingested value, not a second, divergent
  source.

- **Two composable fixes, both real-corpus TDD (RED confirmed for the intended reason, then
  GREEN):**
  1. `encumbrance::equipment_key_resolves_a_carried_weight` (new) — resolves `item_id` and
     checks whether `weight_and_cost_from_record` finds a `WT:` value, the exact gate
     `compute_encumbrance` itself applies before counting an item as carried (`COST:` alone,
     with no `WT:`, is deliberately NOT sufficient — matches that function's own "weight is
     required, cost is supplementary" rule, so this widening never diverges from the real
     consumer's own gate). Wired into `equipment_key_is_wired` as a third `.or_else`-shaped
     check, after the existing stat-effect and damage-dice checks.
  2. `corpus_loader::equipment_record_from_json` — synthesizes `WT:`/`COST:` tokens from the
     ingested `weight_lbs`/`cost_gp` top-level fields whenever `raw_tokens` did not itself
     carry them, the SAME idiom this function already uses to synthesize a `KEY:` token. Only
     fires when the token is genuinely absent, so an enriched record's own real literal always
     wins unchanged (proven by a negative-control test with deliberately conflicting values).

- **Files touched:**
  - `src/rules_core/encumbrance.rs` — `equipment_key_resolves_a_carried_weight` (new,
    `pub fn`) + 3 new tests (real `Horn of Valhalla (Brass)` corpus-shaped fixture positive
    case, cost-only negative control, unresolvable-item negative control).
  - `src/bin/v06_work_inventory.rs` — `equipment_key_is_wired` widened with the new
    `.or_else`-shaped consultation + 3 new integration tests (real `Horn of Valhalla (Brass)`
    on-disk positive case, hand-built weight-only positive case, hand-built cost-only-no-weight
    negative control).
  - `src/rules_core/corpus_loader.rs` — `equipment_record_from_json`'s `WT:`/`COST:`
    synthesis + 4 new tests (real thin on-disk `Arrow (Slaying)` fixture end-to-end through
    the full loader + `equipment_key_resolves_a_carried_weight`, an isolated unit-level
    synthesis proof, a no-override negative control, a no-field negative control).

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — run against this cycle's own isolated
  diff (`git diff --unified=0` against the rebase base, `-- src/rules_core/ src/bin/`) before
  committing: zero matches. The §6-literal whole-tranche command (scoped to `src/rules_core/`,
  `src/bin/`, since `merge-base HEAD origin/develop`) reports pre-existing hits from work
  already committed before this cycle started (the pattern's own doc comment names this class
  of hit as intentionally still caught, "to keep catching `sd19_class_catalog`") — none
  introduced by this cycle's own isolated diff.

- **Wired-integration audit result:** `OK_NO_TOKENS` (both the isolated diff and, by
  inspection, no `STUB`/`MOCK`/`placeholder`/`todo`/`fixme`/`hack` token in any hunk this
  cycle added).

- **Acceptance criterion (verbatim, `epic-breakdown.md` AT-34-E3-003):** "buckets M, V, D, U,
  X close ... Evidence: per bucket, the atlas reporting zero for `core_rulebook`, with
  movement in four buckets. A count that drops because measurement changed is
  instrument-correction, not closure." This cycle's fix is a real closure mechanism (a
  compute-path observation the probe was blind to, not an instrument correction); the
  remainder — including the fact that the shared regen has not yet applied this fix to the
  committed `docs/work-inventory.json` — is reported honestly below.

- **RED → GREEN (both fixes, independently):**
  - `encumbrance::equipment_key_resolves_a_carried_weight`: temporarily short-circuited the
    new probe check to `false`, re-ran `equipment_probe_promotes_a_real_item_via_its_
    weight_token_alone` and `equipment_probe_promotes_a_hand_built_item_with_only_a_weight_
    token` → both FAILED for the intended reason ("Horn of Valhalla (Brass) carries no
    BONUS/TEMPBONUS/DAMAGE chain..." / "carries a real WT: token..."). Restored the call,
    re-ran → both PASSED.
  - `corpus_loader` synthesis: temporarily gated both synthesis blocks with `false &&`,
    re-ran `equipment_record_from_json_synthesizes_wt_and_cost_when_raw_tokens_is_absent` and
    `a_thin_record_with_no_raw_tokens_still_synthesizes_its_real_weight_and_cost` → both
    FAILED for the intended reason ("WT: must be synthesized" / "synthesized WT: token must
    be present"). Restored, re-ran → both PASSED.
  - Scoped suites: `cargo test -p codex --lib rules_core::encumbrance::` — **9/9 pass** (was
    6; +3 new). `cargo test -p codex --lib rules_core::corpus_loader::` — **10/10 pass** (was
    6; +4 new). `cargo test --bin v06_work_inventory equipment` — **33/33 pass** (was 30; +3
    new). `cargo test -p codex --lib rules_core::equipment_effects::` — **86/86 pass**
    (unchanged — this cycle touches no file under `equipment_effects/`).

- **Figures + their re-derive commands:**
  - `core_rulebook` M at cycle start: `python3 scripts/completion_atlas.py --book
    core_rulebook --check` → **811**, denominator `core_rulebook` population 6,701.
  - Territory split at cycle start: Python script against `docs/work-inventory.json`
    (`u.get('book')=='core_rulebook' and u.get('status')=='ingested-magnitude' and
    u.get('evidence')=='equipment_table_entry_with_corpus_magnitude'` / `'...closure_
    wiring_class_does'`) → **163** / **99**, denominator `core_rulebook` M 811.
  - Corpus-wide territory (same script, `kind` in `{equipment, equipment_modifier}`,
    evidence in the same two strings, ALL books): **763** total across **21** books
    (`core_rulebook` 262 of it, matching the book-scoped count above exactly).
  - **Real, direct post-fix closability measurement (the authoritative figure this cycle's
    status is set from).** The full `v06_work_inventory` regen (fed the real
    `corpus_literal_sweep`/`derived_evaluator_fixture_check` reports, no `--allow-stamp-loss`)
    was started and genuinely did not finish inside this cycle's turn budget — killed after
    ~13 minutes CPU with the corpus scan still in progress, `docs/work-inventory.json`
    confirmed byte-identical to its pre-run state (`md5sum` match) so no partial-write risk.
    **This is the SAME wall this wave's own `AT-34-E4-002` cycle 9 already hit and named**
    (`kanban.md` row 19–21: "the regen pipeline was still running after 6+ minutes and was
    killed... Not yet confirmed by a shared regen — pending the wave's own closing cycle").
    Rather than accept an unverified guess, this cycle measured the SAME function the full
    regen's classifier consults — `equipment_key_is_wired`, called with zero modification,
    against the real on-disk corpus, loaded one book at a time exactly as `probe_equipment_
    effect_wiring` itself does (not a classify()-level approximation with hand-built facts;
    the literal probe function, the literal corpus). Confirmed structurally
    (`src/bin/v06_work_inventory.rs`'s `Kind::Equipment | Kind::EquipmentModifier` classify
    arm): `facts.equipment_effect_wired` — built from exactly this probe — is consulted
    FIRST, unconditionally, ahead of every other rung, for both kinds. A temporary `#[test]`
    (removed before this commit; not left as permanent scaffolding since it depended on a
    scratch-directory file path) ran this measurement, book by book, and printed:
    ```
    CYCLE6_WIDE_BOOK book=advanced_class_guide territory=50 wired=41
    CYCLE6_WIDE_BOOK book=advanced_players_guide territory=11 wired=4
    CYCLE6_WIDE_BOOK book=advanced_race_guide territory=12 wired=3
    CYCLE6_WIDE_BOOK book=adventurers_guide territory=88 wired=61
    CYCLE6_WIDE_BOOK book=bestiary territory=4 wired=3
    CYCLE6_WIDE_BOOK book=bestiary_4 territory=1 wired=0
    CYCLE6_WIDE_BOOK book=core_rulebook territory=262 wired=33
    CYCLE6_WIDE_BOOK book=inner_sea_gods territory=14 wired=14
    CYCLE6_WIDE_BOOK book=inner_sea_intrigue territory=1 wired=0
    CYCLE6_WIDE_BOOK book=inner_sea_magic territory=64 wired=6
    CYCLE6_WIDE_BOOK book=inner_sea_temples territory=35 wired=32
    CYCLE6_WIDE_BOOK book=inner_sea_world_guide territory=4 wired=0
    CYCLE6_WIDE_BOOK book=mythic_adventures territory=3 wired=0
    CYCLE6_WIDE_BOOK book=occult_adventures territory=3 wired=2
    CYCLE6_WIDE_BOOK book=pathfinder_unchained territory=4 wired=0
    CYCLE6_WIDE_BOOK book=ultimate_combat territory=11 wired=0
    CYCLE6_WIDE_BOOK book=ultimate_equipment territory=120 wired=15
    CYCLE6_WIDE_BOOK book=ultimate_intrigue territory=1 wired=1
    CYCLE6_WIDE_BOOK book=ultimate_magic territory=8 wired=0
    CYCLE6_WIDE_BOOK book=ultimate_psionics territory=66 wired=5
    CYCLE6_WIDE_BOOK book=ultimate_wilderness territory=1 wired=1
    CYCLE6_WIDE_TOTAL total=763 wired=221
    ```
    **corpus-wide: 221 of 763 territory units now resolve `equipment_key_is_wired`; `core_rulebook`: 33 of 262.**
    Re-run with fix 2 (the `corpus_loader` synthesis) temporarily disabled isolates each
    fix's own contribution: fix 1 alone (the probe widening, already-enriched records only)
    → **109 corpus-wide (23 `core_rulebook`)**; fix 2's own marginal contribution (thin
    records, only effective once fix 1 is also active) → **+112 corpus-wide (+10
    `core_rulebook`)**. `109 + 112 = 221`, confirmed.

- **Build scope verified:** `cargo test --locked --no-run` — **exit 0**, full workspace, run
  at `acd6a6a5e3`. `apps/desktop/src-tauri` tested explicitly — **exit 0** ("Finished `test`
  profile [unoptimized + debuginfo] target(s) in 2m 47s"), run at `acd6a6a5e3` (a separate,
  freshly-created `CARGO_TARGET_DIR`, no shared-slot contention with the root workspace build).

- **Sweep population:** `cargo run --locked --release --bin corpus_literal_sweep --
  --json-out …` → **48,708 examined of 51,482 read, 413,336 tokens compared (9 synthesized),
  51,469 digests checked, 0 findings, CLEAN** — identical to cycle 5's own baseline
  (`git status --porcelain -- data/corpus/` confirms empty: no corpus records touched this
  cycle, so the examined-population must be unchanged, and is).
  `derived_evaluator_fixture_check --json-out …` → **1,839 units cleared over 2,580 fixture
  rows, 0 failed, 0 not ingested** — identical to cycle 5's own baseline.

- **Oracle pin:** `PCGEN_ORACLE_SHA` per `scripts/pcgen-oracle-pin.env` — not consulted this
  cycle; this fix's figure comes from `encumbrance`/`corpus_loader`'s own real corpus read and
  the wiring probe's own direct observation, not an oracle round-trip.

- **Row-count command output (this cycle's own artifact — the direct probe measurement above,
  since the whole-inventory id-diff a full regen would produce did not finish inside this
  cycle's turn budget):**
  ```
  CYCLE6_WIDE_TOTAL total=763 wired=221
  CYCLE6_MEASURE (core_rulebook only) total=262 newly_wired=33 still_unwired=229
  ```
  This cycle's own real closure-eligible count is **221 corpus-wide (33 `core_rulebook`)** —
  proven against the exact function `classify()`'s equipment arm consults, against the real
  on-disk corpus. It is **not yet an `-> grounded` transition in the committed
  `docs/work-inventory.json`**: that requires a full regen run, which this cycle could not
  complete (see above). The next regen (this wave's own closing cycle, or the next shared
  regen wave, per `AT-34-E4-002` cycle 9's own precedent) will apply it and produce the
  authoritative id-diff.

- **Status:** partial.

- **Movement, four buckets:** **Closure: 0 committed this cycle** (the fix is real and its
  221-unit corpus-wide / 33-unit `core_rulebook` reach is proven directly against the real
  compute path, but `docs/work-inventory.json` is unchanged by this cycle's own commit — no
  bucket boundary has yet been crossed in the persisted board). **Proven closure-eligible,
  pending the next regen: 221 corpus-wide, 33 `core_rulebook`.** Reclassification: 0.
  Reachability: 0. Instrument-correction: 0 (no corpus record touched, no fixture/sweep
  disagreement surfaced — the retro `correction` event above is a correction of a prior
  cycle's DISPOSITION of a fact, not of a measured count). `scripts/box_ledger.py --check`
  (run against the still-current, pre-regen `docs/work-inventory.json`) — 7 pre-existing
  stale-count WARNINGs against `THE-BOX.md` (inherited SD-33 artifact, unowned by this
  mechanism, already documented by prior SD-34 cycles), `uncovered=28655
  oracle_disagreement=0 unverifiable_done=0 stale=False` — exit 1 on the stale-count WARNINGs
  alone, not a new defect this cycle introduced, unchanged from cycle 5's own citation.

- **`docs/work-inventory.json` / `completion-atlas.json` NOT touched this cycle** (no local
  regen completed to produce a new version of either) — nothing to `git restore`; both files
  are byte-identical to the rebase base (`md5sum` confirmed). The next regen that DOES
  complete owns applying this cycle's fix to both, per `decisions.md §9`'s established
  file-ownership pattern.

- **Notes (judgment calls):**
  - Fix 1 (the probe widening) and fix 2 (the loader synthesis) are independent and
    composable — fix 1 alone closes only already-enriched records (109 corpus-wide); fix 2
    alone changes nothing without fix 1 (thin records still needed the probe to consult the
    newly-synthesized token). Both are needed for the full 221-unit reach.
  - Deliberately gated on `WT:`/weight only, not `COST:` alone — matching `compute_
    encumbrance`'s own real behavior exactly (an item with cost but no weight is not counted
    as carried, so it is not "computed and applied" by this consumer either). A record whose
    only magnitude is `COST:` with no `WT:` remains correctly unclosed by this fix — this is
    why 542 of the 763-unit territory (763−221) stay unwired: most are `%CHOICE`/`VAR`-gated
    (cycles 2–5's already-declined boundary, corpus-wide, not just `core_rulebook`), `COST:`-
    only records, or other new-subsystem shapes cycle 5's `core_rulebook`-scoped census
    already named.
  - **Choosing the direct-probe measurement over the full regen was a deliberate, precedented
    disposition, not a shortcut.** This wave's own `AT-34-E4-002` cycle 9 already established
    that a >6-minute local regen with no functional payoff over a targeted measurement is
    correctly killed and reported as a "functional" (not whole-corpus-regen-confirmed) result
    (`kanban.md` row 19–21). This cycle's own measurement is more direct than that precedent
    (the literal production probe function against the literal on-disk corpus, not a
    classify()-level test with hand-built `EngineFacts`), and is fully reproducible by anyone
    re-running `equipment_key_is_wired` against `load_equipment_corpus`'s real output for
    each book in the territory list above.
  - This does not touch the `%CHOICE`-gated family (34 units, `core_rulebook`) or the
    `VAR`/`PRE`-gated family (18 units, `core_rulebook`) cycles 2–5 already correctly
    declined — those need real `EquipmentSelection`-level "chosen value" plumbing, a
    different criterion's scope, unchanged by this cycle.

- **Next-cycle plan:**
  1. **Run the full regen to completion** (ideally at a point in the wave where no other
     cargo build contends for the shared slot, and with enough turn budget to let a
     multi-minute unoptimized run finish) to produce the authoritative `docs/work-
     inventory.json` id-diff and apply this cycle's own 221-unit / 33-`core_rulebook`-unit
     reach. Until then, `core_rulebook` M stays reported at **811** in the committed board
     (this cycle's fix is real and tested but not yet reflected there).
  2. **The `core_rulebook` remainder after this fix applies** (262 − 33 = 229) is dominated
     by the same shapes cycle 5's own qualifier-shape census already named: `%CHOICE`-gated
     (34), `VAR`/`PRE`-gated (18), `COST:`-only-no-`WT:` records, and small new-subsystem
     shapes (`SKILLRANK`, `EQM`, `EQMWEAPON`, `WEAPON`, `DR`) each too small alone. A future
     cycle should re-derive this exact remainder fresh (not inherit 229) once the regen has
     actually applied this cycle's own closures, since some units cycle 5 counted as
     "COST/WT-only, nothing to compute" may now resolve differently under the new gate.
  3. **`ability_content` (217, sibling lane) remains the largest overall `core_rulebook` `M`
     sub-cause**, unchanged by this cycle (out of territory).
