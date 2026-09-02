# Cycle — SD-34 wave 34, Lane C — wave 33 lane C's named 19-unit remainder

**Status: complete for 1 of 19; 18 named as still open, not escalated.** Wave 33 lane C's own
Next-cycle plan item 1 named 19 gate-eligible classes (10 untabled base classes + 7 CRB
NPC/`Ex-*` classes + Ninja/Samurai) that all already pass `has_supported_class_chassis` but
carried no `CLASS_WEAPON_PROFICIENCIES` row, so `combat.baseline_weapon_proficiency_unknown`
still claim-blocked every one of them. This cycle searched each of the 19 individually against
its own `data/corpus/**/class_feature/<class>/` directory (the same per-key discipline wave 33
lane A and lane C both used, not batch-assumed) and found:

- **17 of 19 genuinely absent, re-confirmed, no row added.** The 10 untabled base classes
  (Aegis, Antipaladin, Cryptic, Dread, Magus, Marksman, Soulknife, Tactician
  [`ultimate_psionics`'s base class, not Ultimate Combat's Tactician fighter archetype — a
  corpus identifier scope collision checked and avoided], Vitalist, Wilder) and the 7 CRB
  NPC/`Ex-*` classes (Adept, Aristocrat, Commoner, Expert, Warrior, Ex-Barbarian, Ex-Paladin)
  carry no `AUTO:WEAPONPROF` token anywhere this cycle's own search found. Magus carries a real
  `armor_proficiency.json` but no weapon-proficiency record at all; Ex-Barbarian and Ex-Paladin
  carry no `class_feature/` directory of their own whatsoever.
- **Samurai: a real record exists but carries nothing this table's schema can represent.**
  `ultimate_combat/class_feature/samurai_proficiencies/samurai_proficiencies.json` carries
  exactly `AUTO:WEAPONPROF|TYPE=Samurai` — a weapon TYPE selector, not a tier/named-weapon/
  weapon-group this table models — plus a virtual `Exotic Weapon Proficiency (Katana)` feat
  grant this engine does not model anywhere. An all-empty row would be indistinguishable from a
  real "proficient with nothing" claim, so none is added — the same reasoning wave 33 lane C's
  own comment already gave for leaving this one open.
- **Ninja: closed.** `ultimate_combat/class_feature/ninja/ninja_weapon_proficiencies.json`
  carries a real `AUTO:WEAPONPROF|Shortbow|Sword (Short)|Kama|Kusarigama (Sickle and Chain)|
  Nunchaku|Sai|Shuriken|Siangham|Wakizashi` token, transcribed in full. Its DESC additionally
  claims blanket Simple-weapon proficiency, but no matching token exists on this record, so
  Simple is deliberately NOT added — the identical boundary this file already applies to
  Occultist/Vigilante's own "simple and martial" DESC vs. Martial-only token (wave 33 lane C's
  own comment explicitly named this as the exact boundary a future Ninja row would need). This
  is a real, honest, partial transcription — but it resolves correctly for this table's one
  live consumer (`character_is_proficient_with`, always checked against the Longsword):
  Longsword is Martial-tier and not on Ninja's named list either way, so the Longsword
  nonproficiency verdict is correct regardless of the missing Simple tier.

The chassis gate was already open for all 19 (wave 33 lane C's own fix); a found row alone
closes a unit, verified directly for Ninja (`gunslinger_and_ninja_reach_computed_status_
samurai_does_not`) rather than assumed from the roster.

- **Commit SHA:** reported in the structured output — cannot cite its own future hash.
- **Files touched this cycle:**
  - `src/rules_core/rules_tables/crb/weapon_tables.rs` — one new `CLASS_WEAPON_PROFICIENCIES`
    row (`class:ninja`), transcribed from its own corpus token. `OUTSIDE_THE_CRB_WEAPON_TABLE`
    (in `every_named_class_proficiency_matches_a_real_weapon`) widened by two real Ultimate
    Combat weapons this CRB-only table has no stat block for (Kusarigama (Sickle and Chain),
    Wakizashi), the same scope boundary already carried for Mesmerist's Sword Cane.
    `CLASS_WEAPON_PROFICIENCIES.len()` assertion updated 41 → 42; the Longsword-proficiency
    test's denominator comment updated 41 → 42 (proficient count unchanged at 16 — Ninja is
    genuinely non-proficient). One new test,
    `ninja_has_its_real_named_weapon_list_and_no_blanket_simple_tier`, pins the transcription
    and the correct (non-proficient) Longsword verdict.
  - `src/rules_core/pilot_compute/mod.rs` — no gate/dispatch code changed (the chassis gate was
    already open for Ninja via the pre-existing `is_supported_uc_single_class` arm). Only test
    and doc-comment updates: `gunslinger_alone_reaches_computed_status` renamed to
    `gunslinger_and_ninja_reach_computed_status_samurai_does_not` and its assertions widened
    (Gunslinger AND Ninja now assert `Computed`; only Samurai asserts NOT `Computed`) — the
    test's own prior doc comment explicitly anticipated this exact failure shape ("if this now
    fails ... need revisiting, not just this assertion"), so the rename is the anticipated
    response, not a surprise. The `ultimate_combat_chassis_gate_tests` module doc comment and
    `the_four_chassis_integration_blockers_are_gone_for_all_three_uc_classes`'s doc comment
    updated to name Ninja as closed and Samurai as the sole remaining gap, with Samurai's real
    reason (weapon-TYPE-selector, no representation) stated instead of the stale "deliberately
    left open" framing.
  - `docs/work-inventory.json` — regenerated through the guarded path (below) at this cycle's
    own final commit. Population unchanged (49438); Ninja's own unit
    (`ultimate_combat:class:ninja`) now reads `status: "grounded"`, `evidence:
    "class_probe_observed_computed_delta_on_the_rendered_snapshot"`.
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` —
    regenerated via `python3 scripts/completion_atlas.py --check` at this cycle's own final
    commit.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — `git diff --unified=0 -- \
  src/rules_core/rules_tables/crb/weapon_tables.rs src/rules_core/pilot_compute/mod.rs | \
  grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` finds nothing.
- **Wired-integration audit result:** `OK_NO_TOKENS` — same diff range, `grep -nE
  '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` finds nothing.
- **Acceptance criterion:** *(no `AT-34-E#` card names this specific shape — this is a
  bucket-D mining cycle, continuing wave 33 lane C's own Next-cycle plan item 1. That plan's own
  instruction: search each of the 19 individually, add a real row for every one a record
  actually exists for, name the rest by mechanism.)*

## Figures + their re-derive commands

- **Population, unchanged:** 49438. Command: `python3 scripts/completion_atlas.py --check` →
  `population=49438 buckets=10 unclassified=0 overlap=0`.
- **`class_modelled_but_no_observed_delta_on_the_rendered_snapshot`:** 29 (pre-cycle, wave 33
  lane C's own final commit `ba2876a54a`, unchanged since) → **28 measured** (this cycle's own
  final `--check` run, `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/
  completion-atlas.json`'s `sub_causes.D.class_modelled_but_no_observed_delta_on_the_rendered_
  snapshot`). Delta **-1**, exactly Ninja.
- **DONE:** 24994 (pre-cycle) → **24995 measured**. Delta **+1**.
- **D:** 2924 (pre-cycle) → **2923 measured**. Delta **-1**, the same unit leaving bucket D for
  DONE.
- **overlap=0, unclassified=0, done_evidence_violations=0, citation_failures=0,
  missing_clearing_mechanisms=0, stale_derived_at=False** — all from the same `--check` run
  above, at this cycle's own final commit.
- **`denominator_gate.py --check`:** `files_checked=156 violations=0`. **`--check-provenance`:**
  `files_checked=86 figures_examined=128 violations=0`. Both run at this cycle's own final
  commit.
- **Scoped test count:** 24 passed, 0 failed. Command: `cargo test --locked --lib -j 6 --
  class_weapon_proficiency_tests untabled_class_chassis_gate_tests
  ultimate_combat_chassis_gate_tests`, `CARGO_TARGET_DIR=/tmp/cargo-sd34-wave34-lanec`, 13.23s.

## Row-count command output

Not a row-owning artifact in the `AT-34-E#` card sense (no card names this cycle) — the atlas
`--check` output above is this cycle's own row-count instrument, per the receipt schema's
allowance for cycles outside the epic/card structure.

## Build scope verified

Scoped, not full-workspace (full `scripts/verify.sh` runs once at wave-end by a different agent
after all three wave-34 lanes land, per this wave's own dispatch instruction):

`cargo test --locked --lib -j 6 -- class_weapon_proficiency_tests untabled_class_chassis_gate_
tests ultimate_combat_chassis_gate_tests` at this cycle's own final commit: **24 passed, 0
failed, 0 ignored, 3023 filtered out**, finished in 13.23s.

`cargo test --locked --lib -j 6 -- weapon_tables` (the whole module, both nested test mods): 28
passed, 0 failed.

`python3 scripts/completion_atlas.py --check` at the same commit: **exit 0**, all six
fail-closed conditions clear (see Figures above).

`cargo test --locked --no-run` (full workspace) — **not run this cycle**, per this wave's own
dispatch instruction ("run ONLY the tests scoped to what you touched ... a separate agent runs
the full `scripts/verify.sh` once at wave-end").

## Sweep population

`corpus_literal_sweep --json-out`: **corpus-literal-sweep: CLEAN** (guard input for the
`docs/work-inventory.json` regeneration below). This cycle adds no new corpus records and
touches no corpus JSON files under `data/corpus/` (it changes engine code and one static Rust
table only), so the examined population is expected — and confirmed — unchanged from wave 33
lane C's own clean sweep. `derived_evaluator_fixture_check --json-out`: no findings.

## Oracle pin

`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` — unchanged, carried forward from
`scripts/pcgen-oracle-pin.env`. This cycle's own proficiency transcription reads the pinned
PCGen checkout's already-ingested corpus JSON
(`data/corpus/ultimate_combat/class_feature/ninja/ninja_weapon_proficiencies.json`), itself
derived from the pinned checkout, not a separate unpinned clone.

## Guarded regeneration

`docs/work-inventory.json` regenerated through the guarded path, no `--allow-stamp-loss`
needed:

```
cargo run --locked --bin corpus_literal_sweep -- --json-out <scratch>/corpus_literal_sweep_report.json --quiet
-> corpus-literal-sweep: CLEAN
cargo run --locked --bin derived_evaluator_fixture_check -- --json-out <scratch>/derived_evaluator_fixture_check_report.json --quiet
-> (no findings)
CORPUS_LITERAL_SWEEP_REPORT=<scratch>/corpus_literal_sweep_report.json \
DERIVED_FIXTURE_CHECK_REPORT=<scratch>/derived_evaluator_fixture_check_report.json \
cargo run --locked --bin v06_work_inventory -j 6
-> docs/work-inventory.json regenerated, 49438 units, generated_at 2026-09-02T20:35:22Z
```

## Status: complete (1/19), remainder named by mechanism (18/19), no escalation

## Movement, four buckets

- **Closure:** 1 unit — Ninja (`ultimate_combat:class:ninja`) — moved bucket D → DONE (real
  `Computed`/`grounded` receipt, a real, honestly-partial `CLASS_WEAPON_PROFICIENCIES`
  transcription landed).
- **Reclassification:** 0 — Ninja closed, it did not move sideways between non-DONE buckets.
- **Reachability:** 18 — the remaining 18 of the named 19, checked individually against
  `data/corpus/` and (for Samurai) confirmed against `docs/work-inventory.json`, not assumed
  from the registry list: **17** with genuinely no `AUTO:WEAPONPROF` token found anywhere (10
  untabled base classes + 7 CRB NPC/`Ex-*` classes) + **1** (Samurai) with a real record that
  carries nothing this table's schema can represent (a weapon-TYPE selector, not a
  tier/named-weapon/weapon-group).
- **Instrument-correction:** 0.

## Notes

- **The chassis-gate-alone-closes-a-unit claim from wave 33 lane C's own Next-cycle plan was
  verified directly, not assumed.** `gunslinger_and_ninja_reach_computed_status_samurai_does_
  not` builds a real headless receipt for Ninja at level 5 and asserts `HeadlessReceiptStatus::
  Computed` — the actual deliverable, not just the absence of the four chassis-gate blockers.
- **Proficiency transcription discipline held identically to wave 33 lane C's own precedent.**
  Ninja's DESC claims blanket Simple-weapon proficiency ("proficient with all simple weapons")
  that the ingested token does not carry as a matching facet; the added row transcribes only
  what the token carries (the 9 named weapons), the same boundary already shipped for
  Occultist/Vigilante's Martial-only rows. This is a real, honest partial transcription, stated
  as such in the table's own comment — not a claim that the row is complete. It is correct for
  the table's one live consumer today (`character_is_proficient_with`, always checked against
  the Longsword, which is Martial-tier and outside Ninja's named list either way), and the risk
  of a future wider consumer trusting a `Some(row)` as complete is named explicitly in the code
  comment rather than left implicit.
- **Samurai's own record was re-verified, not re-quoted from wave 33 lane C's receipt.** Its
  file lives under a differently-named sibling directory
  (`class_feature/samurai_proficiencies/samurai_proficiencies.json`, not
  `class_feature/samurai/...`) — read directly this cycle to confirm the `AUTO:WEAPONPROF|
  TYPE=Samurai` content wave 33 lane C's own comment described, per this cycle's own
  "do not batch-assume" instruction.
- **`Kusarigama (Sickle and Chain)` and `Wakizashi` are real weapons this CRB-only table has no
  stat block for**, added to `OUTSIDE_THE_CRB_WEAPON_TABLE` rather than silently dropped from
  Ninja's named list or fabricated as a stat block that does not exist in this table's own
  106-record CRB scope.

## Next-cycle plan

1. **Samurai** — closing it needs `weapon_tables.rs`'s schema to grow a weapon-TYPE-selector
   representation (`TYPE=Samurai`) first, a larger change than a transcription; out of this
   cycle's scope.
2. **17 genuinely-absent classes** (10 untabled base + 7 CRB NPC/`Ex-*`) — no corpus proficiency
   record exists for any of them under this cycle's own search; closing any of them would need
   either a corpus re-check against a newer oracle pin, or an operator ruling on how to handle a
   real chassis with no proficiency data at all (e.g., a documented "unknown, not non-proficient"
   posture change, which is a policy question, not a transcription).
3. Wave 33 lane C's other two open threads (the 10 out-of-scope prestige classes;
   Psychic Warrior's `psychic_warrior`/`psychic warrior` key-mismatch bug) are unchanged by this
   cycle and remain open as that receipt named them.
