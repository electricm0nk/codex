# Cycle — Epic 3 (Core Rulebook to zero) / AT-34-E3-003 (bucket `M` — EQUIPMENT sub-causes)

- **Commit SHA:** this receipt is committed in the same commit as the code/docs it describes;
  `git log -1` at the time of reading resolves it. Started from `1ea93e99ce`
  (`origin/tranche/14` at rebase time).

- **Files touched:**
  - `src/bin/v06_work_inventory.rs` — `equipment_key_is_wired` (the shared function
    `probe_equipment_effect_wiring` calls per `(engine_book, key)`, whose result feeds
    `classify()`'s `Kind::Equipment`/`Kind::EquipmentModifier` "observed" rung) widened with a
    second, independent check: after the existing eight `compute_equipment_effects` fields
    (armor-class/max-dex/spell-failure/ACP/skill/ability/weapon-enhancement/spell-resistance) all
    come back `None`, the function now also consults
    `damage_total::resolve_base_damage_dice(key, corpus)`. This is **not a new compute path** —
    `resolve_base_damage_dice` already reads a weapon's real `DAMAGE:` corpus token into a
    structured `DiceExpression`, and is already the entry gate
    `damage_total::resolve_weapon_damage_breakdown` uses to build the `WeaponDamageBreakdown` the
    desktop app's `character_hub.rs` renders on the real character sheet (confirmed:
    `grep -n 'resolve_base_damage_dice' src/rules_core/damage_total.rs` shows it gating
    `resolve_weapon_damage_breakdown` at line 798; `grep -n 'WeaponDamageBreakdown'
    apps/desktop/src-tauri/src/character_hub.rs` shows the wire-form consumer). Consulting it here
    widens what the probe **observes**, exactly the shape the prior `skill_content` cycle's
    `skill_bonus_is_grounded_for_display_name` widened for `Kind::Skill` — no new subsystem, no
    new evidence rung, the SAME `equipment_effect_probe_observed_computed_delta` evidence string
    the probe's other observed-delta promotions already use. Four new tests:
    `equipment_probe_promotes_a_real_weapon_with_a_real_damage_token` (real on-disk CRB corpus,
    `Bastard Sword (Base)`), `equipment_probe_promotes_a_hand_built_weapon_with_only_a_damage_token`
    (fixture, pins the mechanism to the `DAMAGE:` token alone),
    `equipment_probe_does_not_promote_a_record_with_no_damage_token_and_no_other_effect` (negative
    control), `a_real_damage_token_promotes_the_equipment_unit_to_grounded_end_to_end`
    (`classify()`-level, same shape as the pre-existing
    `an_observed_computed_delta_outranks_the_text_complete_rung`).
  - `scripts/completion_atlas.py` — 10 `BUCKET_DEFINITIONS` `file:line` citations re-derived and
    corrected. This cycle's insertion into `equipment_key_is_wired` (line 6607, +20 lines) shifted
    every citation below it by exactly +20 — measured via `sed -n '<old+20>p'
    src/bin/v06_work_inventory.rs` against each citation's own `must_contain` literal before
    writing the new line number (never guessed by offset alone); `citation_failures=0` confirmed
    by `python3 scripts/completion_atlas.py --check` after.
  - `docs/work-inventory.json` (regenerated at HEAD, guarded three-pass pipeline —
    `corpus_literal_sweep --json-out` → `derived_evaluator_fixture_check --json-out` →
    `v06_work_inventory` with both report env vars set, no `--allow-stamp-loss` needed or used).
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (regenerated
    by its own `--check` run after the citation fixes — counts and citations only, no hand edits).
  - This receipt, `docs/release/SD-34-book-completion/progress.md`,
    `docs/release/SD-34-book-completion/kanban.md`.

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` on this cycle's own diff —
  `git diff --unified=0 1ea93e99ce -- src/rules_core/ src/bin/ scripts/oracle_harness/
  ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  → zero matches. The wider epic-file-touch-set diff since the tranche cut
  (`git diff --unified=0 ea2b3396f2fde9223dde93522bd2288b463a21ee...HEAD -- src/rules_core/
  src/bin/ scripts/oracle_harness/ artifacts/epic-3-core-rulebook/`) carries 20 matches — all
  pre-existing, all in `src/rules_core/pilot_compute/mod.rs`'s already-landed sibling-cycle
  citations of real `sd13_*`/`sd25_*` test file names (confirmed by the immediately-prior
  `skill_content` cycle's own receipt), none introduced by this cycle, none inside either file
  this cycle touched.

- **Wired-integration audit result:** `OK_NO_TOKENS` on this cycle's own diff (same command,
  second pattern) → zero matches. The wider epic-diff carries 17 matches, all the word
  "placeholder" in its real PF1/PCGen domain sense (already-landed, already reported by the prior
  cycle's receipt), never a code stub; verified none are in this cycle's own touched files.

- **Acceptance criterion (verbatim, `epic-breakdown.md`):** "AT-34-E3-003 — buckets M, V, D, U, X
  close" (per-bucket, atlas reporting zero for `core_rulebook`, movement in four buckets). This
  cycle closes part of bucket `M`'s two EQUIPMENT sub-causes only
  (`equipment_table_entry_with_corpus_magnitude`, `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does`)
  — the criterion as a whole stays open (`M`'s other four sub-causes, and buckets V/D/U/X, are
  untouched by design; territory boundary stated in this cycle's own dispatch brief).

- **Figures + their re-derive commands:**
  - `core_rulebook` bucket `M`: **972 → 958** (−14). Re-derive:
    `python3 scripts/completion_atlas.py --book core_rulebook --check`.
  - `core_rulebook` DONE: **4330 → 4344** (+14). Same command.
  - `core_rulebook` `equipment_table_entry_with_corpus_magnitude` sub-cause: **276 → 262** (−14).
    Re-derive:
    `python3 -c "import json,collections; d=json.load(open('docs/work-inventory.json')); u=[x for x in d['units'] if x['book']=='core_rulebook' and x['status']=='ingested-magnitude']; print(collections.Counter(x['evidence'] for x in u))"`
    — denominator: 972 `core_rulebook` `M`-bucket units at cycle start (this cycle's own dispatch
    figure, itself re-derived fresh at cycle start against `docs/work-inventory.json` and matched
    exactly before any edit).
  - `core_rulebook` `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does` sub-cause:
    **147 → 147** (0 closed) — same command. **The two equipment shapes are genuinely different, as
    the dispatch brief warned**: every one of this cycle's 14 real closures resolved a `DAMAGE:`
    token sitting on the unit's OWN corpus row (`text_only == false` already, i.e. the
    `equipment_table_entry_with_corpus_magnitude` shape); zero came from the closure-only shape
    (an alias row whose own line carries no magnitude token at all — `resolve_base_damage_dice`
    reads a `DAMAGE:` token off the record `equipment_id_resolve` returns, and none of the 147
    closure-only alias rows in this population happen to resolve to a base record carrying one).
    **One fix did not cover both shapes.**
  - Corpus-wide bucket `M`: **5038 → 5007** (−31), denominator 49,438 total units. Re-derive:
    `python3 scripts/completion_atlas.py --check`. **The fix is generic by construction — the
    same `equipment_key_is_wired` function every book's probe pass calls — and the movement
    proves it: 31 units closed across 9 books**, not just `core_rulebook`:
    `core_rulebook` 14, `ultimate_equipment` 5, `bestiary_3` 4, `inner_sea_races` 2,
    `ultimate_psionics` 2, `advanced_class_guide` 1, `advanced_players_guide` 1, `bestiary_2` 1,
    `ultimate_combat` 1 (sum 14+5+4+2+2+1+1+1+1 = 31). Every one of the 8 non-`core_rulebook`
    closures is a real weapon record in that book's own on-disk equipment corpus carrying its own
    `DAMAGE:` token — the same mechanism, applied by construction, not by a book filter.
    (`damage_total::resolve_base_damage_dice`'s internal `RuleSetId::Crb` parameter only affects
    the returned `TableCellRef`'s citation label, not which corpus is searched — see the function's
    own signature and `equipment_id_resolve`'s implementation, which matches purely against the
    `corpus` argument passed in; the probe already scopes that argument to one book's own corpus at
    a time. Named as a real, if cosmetic, imprecision below.)
  - Corpus-wide DONE: **24242 → 24273** (+31). Same command as the corpus-wide M figure.
  - **6 additional corpus-wide units reclassified, not closed** (movement bucket
    "reclassification", not "closure" — see below): all 6 in `ultimate_equipment`, all already at
    status `literal-verified` (bucket `V`, not `M`) both before and after this cycle — their
    `evidence` string changed from `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does`
    to `equipment_effect_probe_observed_computed_delta` because the SAME probe widening now
    observes a real `DAMAGE:`-token weapon that a separate, already-passing oracle-verification
    pass had already stamped `literal-verified` by proxy. This does not move bucket `M` at all —
    named here only because it appeared in the same before/after diff and per `decisions.md §12`
    L1 a field's name (evidence) is reported honestly even when it does not move a bucket count.
  - Whole-corpus before/after diff by unit id (script:
    `/tmp/claude-1000/.../scratchpad/diff_inventory2.py`, comparing the pre-cycle
    `docs/work-inventory.json` snapshot against the regenerated file): **49,438 units before,
    49,438 after, 0 added, 0 removed, 37 changed** — 31 real status transitions
    (`ingested-magnitude → grounded`, all 31) plus the 6 evidence-only reclassifications above.
    No other status transition occurred anywhere in the corpus.

- **Row-count command output:**
  ```
  $ python3 -c "
  import json
  d = json.load(open('docs/work-inventory.json'))
  units = d['units']
  m = [u for u in units if u.get('book')=='core_rulebook' and u.get('status')=='ingested-magnitude']
  print('core_rulebook M count:', len(m))
  import collections
  print(collections.Counter(u.get('evidence') for u in m))
  "
  core_rulebook M count: 958
  Counter({'ability_content_table_holds_record_magnitude_not_yet_computed': 217,
  'equipment_table_entry_with_corpus_magnitude': 262,
  'equipment_own_line_has_no_magnitude_but_closure_wiring_class_does': 147,
  'race_trait_generic_table_holds_record_magnitude_not_yet_computed': 119,
  'template_content_table_holds_record_magnitude_not_yet_computed': 96,
  'in_catalog_with_corpus_magnitude_but_no_observed_consumer': 47,
  'domain_content_table_holds_record_magnitude_not_yet_computed': 34,
  'skill_content_table_holds_record_magnitude_not_yet_computed': 19,
  'spell_list_entry_with_resolved_level': 15,
  'race_trait_states_a_universal_sheet_modifier_pending_compute': 2})
  ```
  Sum 217+262+147+119+96+47+34+19+15+2 = **958**, matching
  `python3 scripts/completion_atlas.py --book core_rulebook --check`'s own live `M: 958` exactly.
  This cycle's own two sub-causes: 262+147 = **409** (down from 423 — this cycle's assigned
  territory), −14 exactly.

- **Build scope verified:**
  - `cargo test --locked --bin v06_work_inventory` (targeted, scoped to this cycle's own touched
    binary) — **443/443 pass** (4 new tests listed above; 439 pre-existing, all still passing).
  - `cargo test --locked --lib rules_core::damage_total::` — pass (no source changes in
    `damage_total.rs` itself; this run confirms the module's own suite is undisturbed by being
    called from a new caller).
  - `python3 -m unittest scripts.tests.test_completion_atlas` — pass (citation re-pin verified
    against the live suite, not just `--check`'s own output).
  - `cargo test --locked --no-run` (full workspace) — exit BUILD_SCOPE_NO_RUN_EXIT, run at this
    cycle's own HEAD (after the last commit that could move a figure — `decisions.md §12` L7).
  - `apps/desktop/src-tauri` (separate cargo workspace,
    `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-003-desktop`):
    `cargo test --locked --no-run --manifest-path apps/desktop/src-tauri/Cargo.toml` — exit
    BUILD_SCOPE_DESKTOP_EXIT. Tested explicitly because `src/bin/v06_work_inventory.rs`'s changed
    function calls `damage_total::resolve_base_damage_dice`, a function the desktop crate's own
    `character_hub.rs` also calls transitively (`codex = { path = "../../.." }` in its
    `Cargo.toml`) — the change adds a read-only consultation, no signature changed on any
    desktop-visible type.

- **Sweep population:** no `data/corpus/**` records added or regenerated — only the engine
  classifier and the derived inventory changed. `corpus_literal_sweep`: **48,708 examined of
  51,482 read** before this cycle's own regen pass (used as its
  `CORPUS_LITERAL_SWEEP_REPORT` input) — 0 delta expected and to be confirmed after, correctly
  matching 0 corpus records added (`decisions.md §12` L8).

- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`) — not consulted this cycle (no oracle-pinned corpus read; the
  `DAMAGE:` token this cycle reads comes from the repo's own `data/corpus/core_rulebook/`, already
  ingested).

- **Status:** partial

- **Movement, four buckets:** closure (31 units corpus-wide, 14 `core_rulebook`, genuinely newly
  promoted `ingested-magnitude → grounded` — a real, already-wired compute path
  (`damage_total::resolve_base_damage_dice`, feeding the desktop app's own
  `WeaponDamageBreakdown`) now backs their magnitude, not a reclassification of the same
  evidence); reclassification (6 units, `ultimate_equipment`, evidence string corrected on
  already-`literal-verified` units, bucket `V` not `M`, described above); reachability 0;
  instrument-correction (10 `scripts/completion_atlas.py` citation line numbers re-pinned after
  this cycle's own +20-line insertion shifted them — a housekeeping correction to the atlas
  instrument itself, not a corpus movement).

- **Notes:**
  - **Both equipment shapes were investigated with real evidence, not assumed to share a fix.**
    Before writing any code, this cycle added a temporary diagnostic test (later replaced by the
    permanent tests above) that ran the real, on-disk `core_rulebook` corpus through every
    candidate `damage_total` function for all 423 units in both sub-causes. Result: only
    `resolve_base_damage_dice` produced new coverage, and only for the
    `equipment_table_entry_with_corpus_magnitude` shape (13–14 of 276, the small discrepancy
    between the pre-code JSON-based estimate and the real post-fix Rust-derived count of 14 is
    attributable to one record — `Special Quality ~ Spikes ~ Shieldbash`, an `equipment_modifier`
    whose own record carries a `DAMAGE:` token but was miscategorized by the exploratory JSON scan
    as `arms_armor`-only; the REAL, authoritative number is the post-regen inventory diff's 14,
    not the pre-code estimate). The `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does`
    shape (147, alias/closure-only rows) got **zero** closures from this mechanism — confirmed, not
    assumed, by the same real-corpus diagnostic. **One fix did not cover both shapes**, exactly as
    the dispatch brief warned to check for.
  - **Why the remainder (409) is not more of the 423, stated plainly.** `compute_equipment_effects`'s
    other categories (`general`, `magic_items`, `equipmods`) were checked and found to have NO
    existing compute path for the dominant remaining shapes:
    - Equipment **modifiers** that grant an AC-type bonus (`Special Ability ~ Bonus AC /
      {Deflection,Enhancement,Insight,Luck,Natural Armor,Other,Profane,Sacred}` — 7 real corpus
      records among the remainder) have no field on `ResolvedEquipmentEffect` at all;
      `arms_armor`'s AC fields are the base item's OWN armor/shield token, never an attached
      modifier's granted bonus. Building one is a new compute path (bucket-`B`-shaped engineering,
      explicitly out of this cycle's "instrument already exists" scope), not a wiring fix.
    - Equipment modifiers whose bonus chain is `%CHOICE`-gated (`Special Ability ~ Bonus Save /
      {Insight,Luck,Other,Profane,Resistance,Sacred}` and similar — a player-choice value, not a
      literal) need the choice-selection mechanism the dispatch brief explicitly reserves to a
      sibling lane (explanation-id/choice wiring) this cycle must not touch.
    - Many `equipment_modifier` records (`Special Ability ~ Charged Item / <N> Maximum`,
      `~ Spell Effect / ...`, `~ Enhancement Cost`, and similar — internal PCGen plumbing that
      scales an item's market price or charge count) carry a real magnitude token
      (`BONUS:VAR`/`BONUS:ITEMCOST`) that is not a player-facing mechanical effect at all; no
      compute path exists or should exist for these under the current promotion ladder (they are
      arguably a `decisions.md §17`-shaped internal-plumbing case for a FUTURE cycle to rule on,
      not this one).
    - Several `equipment` (base item) records are named artifacts whose magnitude is stated in
      corpus prose alongside a real `BONUS:` chain but resolve to record shapes
      (`equipment_id_resolve`) this cycle did not build a probe for (`Holy Avenger`,
      `Nine Lives Stealer`, `Luck Blade`, and similar — 28 of the 262 remaining
      `equipment_table_entry_with_corpus_magnitude` units are named artifacts with no direct
      `DAMAGE:`/`ACCHECK:`/etc. token on their own row, per a real-corpus categorization pass this
      cycle ran but did not act on).
    None of these four shapes has an existing, already-wired compute path this cycle could
    consult without building new engineering — the honest boundary the dispatch brief drew between
    bucket `M` and bucket `B`.
  - Denominator gate against this package:
    `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` →
    DENOMINATOR_GATE_RESULT.

- **Remainder — every unit in this cycle's two EQUIPMENT sub-causes, named, at HEAD:**

  | Sub-cause (evidence string) | Population before | Closed this cycle | Population after | Clearing mechanism for the rest |
  |---|---:|---:|---:|---|
  | `equipment_table_entry_with_corpus_magnitude` | 276 | 14 | **262** | per real sub-shape: ~7 AC-bonus equipmods need a new compute path (bucket-B-shaped, not this cycle's scope); an unmeasured number are `%CHOICE`-gated (sibling lane's scope); an unmeasured number are internal PCGen plumbing (VAR/ITEMCOST — a future `decisions.md §17`-shaped ruling); ~28 are named artifacts needing a description-linked probe not built this cycle |
  | `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does` | 147 | 0 | **147** | same four sub-shapes as above, applied to the closure-only (alias-row) population; zero of this shape happened to resolve to a `DAMAGE:`-bearing base record this cycle |

  Sum: 262 + 147 = **409**, matching this cycle's own live re-derive
  (`python3 scripts/completion_atlas.py --book core_rulebook --check` → `M: 958`, and the two
  sub-cause counts printed by the row-count command above) exactly. This cycle's assigned
  territory was 423 (276+147); 14 closed, 409 remain, 14+409 = 423 exactly.

  **Every other `M` sub-cause, and buckets V/D/U/X, are untouched by this cycle** (out of
  territory by the dispatch brief's own boundary — `ability_content` and sibling `race_trait`/
  `template`/`domain`/`skill_content` compute probes belong to a sibling lane per the dispatch's
  explicit no-collision rule):

  | Bucket / sub-cause | `core_rulebook` population (live, post-cycle) |
  |---|---:|
  | `M` `ability_content_table_holds_record_magnitude_not_yet_computed` | 217 |
  | `M` `race_trait_generic_table_holds_record_magnitude_not_yet_computed` | 119 |
  | `M` `template_content_table_holds_record_magnitude_not_yet_computed` | 96 |
  | `M` `in_catalog_with_corpus_magnitude_but_no_observed_consumer` | 47 |
  | `M` `domain_content_table_holds_record_magnitude_not_yet_computed` | 34 |
  | `M` `skill_content_table_holds_record_magnitude_not_yet_computed` | 19 |
  | `M` `spell_list_entry_with_resolved_level` | 15 |
  | `M` `race_trait_states_a_universal_sheet_modifier_pending_compute` | 2 |
  | `V` (whole bucket) | 81 |
  | `D` (whole bucket) | 366 |
  | `U` (whole bucket) | 10 |
  | `X` (whole bucket) | 115 |

- **Next-cycle plan:**
  1. **This cycle's own remainder (409) needs real new engineering, not more wiring** — an
     AC-bonus-from-equipmod compute path (bucket-B-shaped), which is a legitimate next equipment
     sub-cycle but is explicitly out of "instrument already exists" scope; scope and cost it before
     committing to the full population.
  2. **A finer real-corpus categorization of the 409** (how many are `%CHOICE`-gated vs. internal
     plumbing vs. named-artifact-prose vs. genuinely uncomputed AC/save bonuses) would let a future
     cycle pick the cheapest remaining shape first — this cycle ran that categorization at a
     coarse level (Notes, above) but did not exhaustively classify every one of the 409.
  3. **`ability_content` (217, a sibling lane's territory)** is the largest `M` sub-cause overall
     and already routes through `simple_kind_verdict`'s shared `grounded_magnitude` parameter per
     the prior `skill_content` cycle's own next-cycle plan — not this cycle's to pick up.
  4. **Buckets `V` (81), `D` (366), `U` (10), `X` (115) are entirely untouched by this cycle**,
     unchanged from the prior cycle's own receipt.
