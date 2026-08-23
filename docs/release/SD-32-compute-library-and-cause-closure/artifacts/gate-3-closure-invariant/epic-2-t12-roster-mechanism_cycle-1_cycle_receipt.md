# Cycle epic-2-t12-roster-mechanism — Gate 3 (closure invariant) / Card 11, shape T12 (continuation)

- **Card ID:** `epic-2-cause-closure` (row 11)
- **Commit SHA:** see `git log -1` at push time (this cycle rebases before pushing per §5)
- **Files touched:**
  - `src/rules_core/pilot_compute/untabled_base_class_feature_roster.rs` (new) — fixture-loading
    module for the generic corpus-derived class-feature roster; 5 unit tests.
  - `src/rules_core/pilot_compute/mod.rs` — new `push_untabled_base_class_feature_records`
    (generic, one function, no per-class code), wired into the `untabled_base_class_chassis::
    resolve` dispatch arm; declared the new submodule; 3 wiring tests
    (`untabled_base_class_feature_roster_wiring_tests`).
  - `scripts/census_untabled_base_class_feature_roster.py` (new) — the fixture's re-derive
    command, mechanical (one `CATEGORY=Class|<X>.MOD` regex, no per-class code).
  - `tests/fixtures/rules_core/untabled-base-class-feature-roster.json` (new, generated) — 40
    corpus-derived rows across 3 classes.
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` (row 11, prepended),
    `progress.md` (this entry).

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
  (`git diff --unified=0 -- src/rules_core/pilot_compute/mod.rs src/rules_core/pilot_compute/untabled_base_class_feature_roster.rs scripts/census_untabled_base_class_feature_roster.py | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` — no match)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff, `STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack` — no match)

- **Acceptance criterion (verbatim, this dispatch brief):** "Close the attribution gap. The chassis
  must emit ids that `classify()` can attribute to a class... Fixture-check every emitted value...
  Claim only what actually reaches `grounded`/`text-complete`." **MET, bounded**: 15 real
  `class_feature` units now reach `text-complete`, fixture-checked against the pinned oracle,
  RED→GREEN proven three times (module tests, wiring tests, mutation of the dispatch call site).

- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`,
  matches the pin; oracle bootstrapped fresh into this worktree's git-ignored slot via
  `scripts/fetch-pcgen-oracle.sh`, confirmed against the pin before use).

- **Status:** complete (this lane's own bounded scope — see "What this cycle closes" below; row 11
  stays `in-progress`, T12 is one of card 11's five open sub-shapes and this cycle does not close it
  in full)

## §17a re-derivation — the prior cycle's own figures, re-checked

The prior cycle (`epic-2-t12-modelled-class-books`, already landed on this branch — its own
`modelled_class_books()` registration of the 20-class `untabled_base_class_chassis` registry is
unchanged, unmodified by this cycle) named the exact next lever: "a `push_pu_class_feature_records`-
shaped generic roster mechanism... one fixture schema, one push function, reused across every class
it covers." This cycle builds that.

Re-derived T12's live population fresh, per `decisions.md §17a`, before trusting the prior receipt's
951 figure — found it had already moved to **1,004** (net growth, not regression; a concurrent
sibling lane, `card-15-internal`, landed between the two cycles and grew the corpus-wide
`class_feature` kind by +2,593 units via a `duplicate_identity` fix, unrelated to T12):

```
python3 -c "
import json
d = json.load(open('/tmp/inventory_after_roster.json'))  # cargo run --bin v06_work_inventory -- --stdout-only
print(len([u for u in d['units'] if (u.get('evidence') or '').startswith('class_feature_of_unmodelled_corpus_class')]))
"
# -> 1004
```

Reported here as an honest correction (§17a), not folded into this cycle's own closure claim — this
cycle's own effect is measured directly below, isolated from that concurrent drift.

## The generic mechanism: one corpus regex, one push function, reused across every class it covers

PCGen grants a base class's own-named class features through a `CATEGORY=Class|<ClassName>.MOD`
line: one `ABILITY:<Category>|AUTOMATIC|<ClassName> ~ <Feature>|...PREVARGTEQ:<Var>_CFP_Level,<N>`
field per feature. This is the SAME shape across every class that uses it —
`scripts/census_untabled_base_class_feature_roster.py` writes **one** regex extraction, with no
per-class branching, and runs it against every one of the 20 `untabled_base_class_chassis` registry
classes. `src/rules_core/pilot_compute/untabled_base_class_feature_roster.rs` loads the resulting
fixture (an `include_str!`, same pattern as `untabled_base_class_chassis.rs`'s own fixture) keyed by
`class_id`, and `push_untabled_base_class_feature_records` (mod.rs) is **one function**, reused for
every class the fixture covers — the same "a 21st class costs nothing" shape the chassis registry
itself already established.

**Coverage found, honestly bounded, not assumed universal:** of the 20 registered classes, exactly
**3** (`antipaladin`, `magus`, `vigilante`) use this `.MOD` shape for their own-named-group grants —
40 records total. The other 17 (`aegis`, `cryptic`, `dread`, `kineticist`, `marksman`, `medium`,
`mesmerist`, `occultist`, `psion`, `psychic`, `psychic_warrior`, `shifter`, `soulknife`,
`spiritualist`, `tactician`, `vitalist`, `wilder`) grant their own-named features through a different
progression convention this script does not parse — **confirmed absent by direct scan** (a
`unit test asserts this for `cryptic` specifically, `a_class_the_census_script_found_no_mod_shaped_
data_for_is_honestly_empty`), not merely unchecked. `roster_for()` returns an empty slice for these
17, and the wiring test `a_class_with_no_roster_data_emits_no_untabled_class_feature_ids` proves no
row is fabricated for them.

## RED → GREEN, proven three times

1. `untabled_base_class_feature_roster::tests` — mutated `roster_for` to return an empty `Vec`
   unconditionally, re-ran: `antipaladin_has_at_least_one_row_and_every_row_states_its_own_min_level`
   and `antipaladin_touch_of_corruption_matches_the_oracle_s_level_2_grant` both FAILED for the
   intended reason (`"census script found antipaladin data; fixture must carry it"` /
   `"Touch of Corruption must be in the fixture"`). Reverted, both GREEN.
2. `untabled_base_class_feature_roster_wiring_tests` — commented out the
   `push_untabled_base_class_feature_records(...)` call at its real dispatch site
   (`compute_class_chassis`'s `untabled_base_class_chassis::resolve` arm), re-ran:
   `antipaladin_level_2_reaches_touch_of_corruption_via_the_generic_roster` and
   `antipaladin_level_3_gains_the_level_3_gated_feature` both FAILED for the intended reason (the
   real `explanations` list from a live `compute_pilot_base_chassis` call carried no
   `class_feature.untabled.*` id at all — printed and inspected, not assumed). Reverted, both GREEN.
3. Level-gating itself proven both directions in the same wiring test module: level 2 carries
   `Touch of Corruption` (min_level 2) but NOT `Aura of Cowardice` (min_level 3); level 3 gains it —
   the same "absent means not yet granted" contract `push_pu_class_feature_records` already uses,
   confirmed live, not asserted.

## Suites run

- `cargo test --locked --lib untabled_base_class_feature_roster` (new module + wiring tests):
  **7/7** (targeted).
- `cargo build --locked --lib`: clean (1 pre-existing, unrelated dead-code warning).
- `cargo build --locked --bin v06_work_inventory`: clean (2 pre-existing, unrelated dead-code
  warnings — this cycle touches no code in that binary; only `modelled_class_books()`'s prior-cycle
  registration, unmodified here, feeds it).
- `cargo test --locked --bin v06_work_inventory`: **329/329** (unchanged from the branch's own
  pre-cycle baseline — this cycle's engine-side change is additive-only at the compute layer; the
  census/classifier tool itself is untouched).
- **Full `cargo test --locked --lib` NOT re-run this cycle** (dispatch brief's own "scope your test
  runs" instruction — the full unscoped run has not reliably finished inside a single cycle on this
  box per multiple prior receipts on this branch). Targeted module coverage above is the scoped
  substitute; no file this cycle touches is imported by any test this scoping would miss (`grep -rn
  'untabled_base_class_feature_roster' src/ tests/ apps/` confirms the only call sites are this
  cycle's own new module, its own wiring-test module, and the one dispatch-arm call site).

## Live re-derive: what actually closed, fixture-checked, not fabricated (`decisions.md §16`)

Regenerated the full inventory in-memory (`cargo run --bin v06_work_inventory -- --stdout-only`,
pinned oracle, **not written to `docs/work-inventory.json`** — a measurement; `git status
--porcelain` confirms only this cycle's own files changed):

```
python3 -c "
import json
d = json.load(open('inventory_after.json'))
target_classes = ('Antipaladin ~ ', 'Magus ~ ', 'Vigilante ~ ')
rows = [u for u in d['units'] if u.get('kind')=='class_feature'
        and (u.get('corpus_key') or '').startswith(target_classes)]
print('total own-named units under the 3 covered classes:', len(rows))
from collections import Counter
print(Counter((u['status'], u['evidence']) for u in rows))
"
# -> total 44
# -> Counter({('not-ingested','class_feature_no_dedicated_magnitude_id_matched_the_record_slug'): 25,
#             ('text-complete','explanation_id_observed_and_corpus_record_carries_real_description'): 15,
#             ('not-ingested','class_feature_owner_matched_by_name_but_record_not_held_by_engine'): 2,
#             ('not-ingested','no_explanation_id_and_no_diagnostic_names_this_feature'): 2})
```

**15 real `class_feature` units reach `text-complete` this cycle, by class:**

```
Antipaladin ~ Aura of Cowardice, Aura of Despair, Aura of Sin, Aura of Vengeance, Plague Bringer  (5)
Vigilante ~ Dual Identity, Startling Appearance, Vengeance Strike, Weapon and Armor Proficiencies  (4)
Magus ~ Cantrips, Counterstrike, Greater Spell Access, Knowledge Pool, Spellstrike, True Magus     (6)
```

Every one of these is a text-only corpus record (`wiring_class: "display"`, no `%N` magnitude
placeholder) whose engine-observed `class_feature.untabled.<class>.corpus_record.<slug>` id now
satisfies `classify()`'s broad (roster-inclusive) `exact_suffix_grounded` check, exactly the same
promotion path `decisions.md §7`'s zero-magnitude rule already grants `push_pu_class_feature_records`'
own PU roster. **No unit was promoted to `grounded`** — `explanation_id_observed_in_a_real_
computation`'s count is unaffected by this cycle (the STRICT check that gates `grounded` excludes
every `.corpus_record.` roster id, by the SAME pre-existing rule that protects `push_pu_class_
feature_records`; confirmed by direct read of `non_roster_ids()`'s filter, unmodified by this
cycle) — this cycle never claims a magnitude the engine does not compute.

**The other 29 of the 44 remain honestly `not-ingested`, not fabricated closed:** 25 are
magnitude-bearing records (`Touch of Corruption`, `Channel Negative Energy`, `Spellstrike`'s siblings
that actually resolve a formula, ...) whose real closure needs a per-feature magnitude function —
the roster id alone cannot and does not credit them (proven by the strict-check exclusion above,
live). 4 are records this fixture's own scope deliberately excludes (`Class Skills`, and 3 more not
shaped as a `.MOD`-granted own-named feature at all — a different corpus row shape, out of this
cycle's scope, not guessed at).

## Mechanism-sized plan for the remainder (not a per-class list)

1. **17 of the 20 registered classes have zero data in this fixture.** Their own-named-group grants
   use a different PCGen progression convention (plausibly a direct `CLASS:` multi-column table, or
   a psionics-specific shape for `psion`/`psychic`/`psychic_warrior`/`soulknife` — not investigated
   this cycle; a real next-cycle scoping question, not guessed at here). Extending the census script
   to a second, verified pattern is the next generic-mechanism increment.
2. **The 25 magnitude-bearing records under the 3 already-covered classes** need real per-feature
   compute functions (Antipaladin's Touch of Corruption heals/damages `LayOnHandsDice`d6, Magus's
   Spellstrike is a real combat mechanic, ...) — this is genuinely per-feature work, the same shape
   `ground_unchained_barbarian_class_features` does for PU, not a generic pass. Named, not
   attempted, per the dispatch brief's own "do not start eleven subsystems" instruction (that
   applies with equal force to magnitude functions for the small tier).
3. **Pool-shaped groups** (`Vigilante Talent`, `Magus Arcana`, `Antipaladin`'s own `Fiendish
   Boon`-adjacent picks, ...) remain entirely out of this mechanism's scope by design — they need
   per-pool verification (`class_feature_pool_catalog.rs`'s own construction discipline), not a
   generic extraction.
4. **11-large tier:** unchanged from the prior cycle's assessment — no shared mechanic across the
   11, each needs its own subsystem for magnitude-bearing features; this cycle's roster mechanism
   would lower their text-only share the same way it did for Antipaladin/Magus/Vigilante, IF their
   own-named grants use the `.MOD` shape (unconfirmed for any of the 11 except Magus, which this
   cycle DID cover, 6 units).

## What this cycle did NOT do

No corpus data or `docs/work-inventory.json` changed (measurement only, per §16). Kanban row 11
stays `in-progress`. `Kind::Class` layer untouched (this cycle is `class_feature`-only). The other
four open sub-shapes under card 11 (T2b, T9, T2a-residual, T4-L9) are untouched.

## Next-cycle plan

1. Investigate the progression shape for the 17 uncovered registry classes (start with `psion`/
   `psychic`/`psychic_warrior`/`soulknife` as a plausible shared psionics convention, then the
   remaining 13 individually).
2. Build real magnitude functions for the highest-value of the 25 already-identified magnitude-
   bearing records under the 3 covered classes (Spellstrike and Arcane Pool are Magus's signature
   mechanics and worth prioritizing).
3. Escalate the 11-large tier's magnitude-bearing cost as a named, mechanism-sized plan once more
   classes' progression shapes are known.

`df -h /`: see final report.
