# Cycle — SD-34 wave 33, Lane C — `class_modelled_but_no_observed_delta_on_the_rendered_snapshot`'s 38-unit class-level snapshot-delta shape

**Status: complete for 9 of 38; 29 named as next-cycle scope, not escalated.** Bucket D's
`class_modelled_but_no_observed_delta_on_the_rendered_snapshot` shape covers classes the
engine's own class registries (`untabled_base_class_chassis`, `crb_untabled_class_chassis`)
already model with a real BAB/base-save chassis, but whose `Kind::Class` receipt never showed a
computed delta because a shared gate function, `has_supported_class_chassis` — checked
independently by `compute_total_saves`, `compute_combat_baseline`, and
`compute_selected_skill_modifiers` — had no matching arm for either registry. This cycle widens
that gate to both registries (27 classes total) and adds real `CLASS_WEAPON_PROFICIENCIES` rows,
transcribed from each class's own corpus weapon-and-armor-proficiency token, for the 9 of those
27 with a real proficiency record found this cycle: Kineticist, Medium, Mesmerist, Occultist,
Vigilante, Psychic, Spiritualist, Psion, Shifter. Each of the 9 now reaches `Computed` for real.
The gate widening alone (all 27) does not close a unit by itself — `combat.baseline_weapon_
proficiency_unknown` still claim-blocks the 17 without a matching proficiency row, honestly
(plus 2 more, Ninja and Samurai, already gate-eligible before this cycle via a pre-existing UC
arm and in the identical no-row boat — see Notes), so the movement is exactly the 9 with both
fixes landed together.

**Recovered from a server crash** (2026-09-02 kernel soft-lockup under heavy parallel
`rust-lld` link jobs, confirmed via `journalctl -b -1`, unrelated to this cycle's own
work — the crash hit mid-flight before this lane could commit) via its preserved worktree diff,
rebased onto `tranche/14` (which had moved to include wave 33 lane A, `e8fc4f8ff9`, and lane B,
`8d4646e2a8`, in the meantime), then landed. The only real conflicts were in the two shared
generated artifacts (`docs/work-inventory.json`, `docs/release/SD-34-book-completion/artifacts/
epic-1-atlas/completion-atlas.json`) — resolved by discarding both sides' numbers and
regenerating each fresh, through its own real generator, at this cycle's own final commit,
never hand-merged. The three source files this cycle touches (`src/rules_core/pilot_compute/
mod.rs`, `src/rules_core/rules_tables/crb/weapon_tables.rs`, `src/bin/v06_work_inventory.rs`)
rebased clean with zero conflicts — neither lane A nor lane B touched any of the three.

- **Commit SHA:** `73345ff6cc1a46e7003aeebafbbf70bd9e9eae30` (code); the final commit including
  this receipt + `progress.md` (reported in the structured output — cannot cite its own future
  hash).
- **Files touched this cycle:**
  - `src/rules_core/pilot_compute/mod.rs` — `has_supported_class_chassis` gained two new arms,
    `is_supported_untabled_base_class_single_class` (the 20-class `untabled_base_class_chassis`
    registry) and `is_supported_crb_untabled_class_single_class` (CRB's 7 NPC/`Ex-*` classes,
    `crb_untabled_class_chassis`), plus the gate's description string updated to name both new
    families. Deliberately NOT extended to `prestige_class_entry_gate` (no BAB/save chassis
    exists there by design — that registry's own doc comment says so). Four new tests
    (`untabled_class_chassis_gate_tests`): all 27 classes pass the gate at every real level, a
    prestige class still fails it, the 9 classes with a real proficiency row reach `Computed`,
    and a class without one still honestly reports `combat.baseline_weapon_proficiency_unknown`
    with every OTHER chassis-gate blocker gone.
  - `src/rules_core/rules_tables/crb/weapon_tables.rs` — 9 new `CLASS_WEAPON_PROFICIENCIES`
    rows (Kineticist, Medium, Mesmerist, Occultist, Vigilante, Psychic, Spiritualist, Psion,
    Shifter), each transcribed from that class's own corpus `weapon_and_armor_proficiency*.json`
    token (`AUTO:WEAPONPROF`/`TYPE=WeaponProf*`), never from DESC prose alone — the table's own
    existing discipline (Occultist/Vigilante's DESC says "simple and martial" but their real
    token carries only `TYPE=WeaponProfMartial`, so Simple is deliberately NOT added for
    either — the identical boundary this file's own Ninja/Samurai rows would need if either
    gained one). 18 real classes across this cycle's own two registries (10 untabled base + 7
    CRB NPC/`Ex-*` + Antipaladin, i.e. the 27-class roster minus the 9 closed) carry no matching
    corpus proficiency record found this cycle and are NOT added (real, open, honestly-reported
    gap, not assumed Simple-only) — Ninja and Samurai are a separate, pre-existing Ultimate
    Combat gate (not part of these two registries) in the identical no-row situation; see the
    Movement/Reachability section's 19-unit figure, which combines both.
    `CLASS_WEAPON_PROFICIENCIES.len()` assertion updated 32 → 41; Longsword-proficiency count
    updated 14 of 32 → 16 of 41 (Occultist and Vigilante both added to the Longsword-proficient
    roster, Longsword being Martial-tier either way).
  - `src/bin/v06_work_inventory.rs` — `--class-probe`'s CLI path reused
    `modelled_class_books()` (71 classes) instead of reconstructing a stale 27-class subset
    (`ClassId`+`ApgClassId`+`AcgClassId` only) that predated every later UC/PU/untabled-base-
    class/prestige/NPC widening and silently never probed any of them — the CLI's own ceiling
    report was blind to the population `main()`'s real classification runs against, the "reports
    success without executing anything" failure shape this instrument must not repeat. Two new
    tests: the 9 classes are now probe-observed `Wired` against the full modelled set; the 10
    prestige classes correctly stay unwired (no chassis magnitude by design).
  - `docs/work-inventory.json` — regenerated through the guarded path (below) at this cycle's
    own final commit, all three lanes' changes present. Population unchanged (49438); `grounded`
    4331 → 4340 (+9, matching the 9 closures exactly — the other by-status deltas are within
    normal noise from lane A/B's own already-landed changes being re-measured at this SHA, not
    new movement this cycle).
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` —
    regenerated via `python3 scripts/completion_atlas.py --check` (the command itself writes
    this artifact) at this cycle's own final commit.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — `git diff --unified=0 8d4646e2a8..HEAD --
  src/rules_core/pilot_compute/mod.rs src/rules_core/rules_tables/crb/weapon_tables.rs
  src/bin/v06_work_inventory.rs | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  finds nothing.
- **Wired-integration audit result:** `OK_NO_TOKENS` — same diff range, `grep -nE
  '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` finds nothing.
- **Acceptance criterion:** *(no `AT-34-E#` card names this specific shape — this is a
  bucket-D mining cycle, continuing wave 32 lane C's own reconnaissance, which named this
  38-unit shape as one of bucket D's six mechanisms. This wave's own dispatch instruction:
  close as many of the 38 as a real, honestly-reported chassis+proficiency fix supports; name
  the rest by mechanism, do not escalate.)*

## Figures + their re-derive commands

- **Population, unchanged:** 49438. Command: `python3 scripts/completion_atlas.py --check` →
  `population=49438 buckets=10 unclassified=0 overlap=0`.
- **`class_modelled_but_no_observed_delta_on_the_rendered_snapshot`:** 38 (pre-cycle, lane B's
  own final commit `8d4646e2a8`, unchanged by lane B — `git show 8d4646e2a8:docs/release/
  SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json | python3 -c "import json,
  sys; print(json.load(sys.stdin)['sub_causes']['D']['class_modelled_but_no_observed_delta_on_
  the_rendered_snapshot'])"` → 38) → **29 measured** (this cycle's own final `--check` run,
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`'s
  `sub_causes.D.class_modelled_but_no_observed_delta_on_the_rendered_snapshot`). Delta **-9**,
  exactly the 9 classes closed.
- **DONE:** 24985 (pre-cycle, same `8d4646e2a8` snapshot) → **24994 measured** (this cycle's own
  final `--check` run). Delta **+9**.
- **D:** 2933 (pre-cycle, same snapshot) → **2924 measured**. Delta **-9**, the same 9 units
  leaving bucket D for DONE.
- **overlap=0, unclassified=0, done_evidence_violations=0, citation_failures=0,
  missing_clearing_mechanisms=0, stale_derived_at=False** — all from the same `--check` run
  above, at this cycle's own final commit (not carried forward from the crashed run).
- **`denominator_gate.py --check`:** `files_checked=155 violations=0`. **`--check-provenance`:**
  `files_checked=85 figures_examined=128 violations=0`. Both run at this cycle's own final
  commit.
- **Scoped test count:** 31 passed, 0 failed. Commands: `cargo test --locked --lib -j 6 --
  untabled_class_chassis_gate_tests class_weapon_proficiency_tests` (**19 passed**, 0 failed,
  3027 filtered out, 17.41s) and `cargo test --locked --bin v06_work_inventory -j 6 --
  class_probe_tests` (**12 passed**, 0 failed, 501 filtered out, 26.43s), both run with
  `CARGO_TARGET_DIR=/tmp/cargo-sd34-fold-lanec`.

## Row-count command output

Not a row-owning artifact in the `AT-34-E#` card sense (no card names this cycle) — the atlas
`--check` output above is this cycle's own row-count instrument, per the receipt schema's
allowance for cycles outside the epic/card structure.

## Build scope verified

Scoped, not full-workspace (full `scripts/verify.sh` runs once at wave-end by a different agent
after all three lanes land, per this fold's own dispatch instruction):

`cargo test --locked --lib -j 6 -- untabled_class_chassis_gate_tests class_weapon_proficiency_tests`
at this cycle's own commit `73345ff6cc`: **19 passed, 0 failed, 0 ignored, 3027 filtered out**,
finished in 17.41s.

`cargo test --locked --bin v06_work_inventory -j 6 -- class_probe_tests` at the same commit:
**12 passed, 0 failed, 0 ignored, 501 filtered out**, finished in 26.43s.

`python3 scripts/completion_atlas.py --check` at the same commit: **exit 0**, all six
fail-closed conditions clear (see Figures above).

`cargo test --locked --no-run` (full workspace) — **not run this cycle**, per this fold's own
dispatch instruction ("run ONLY the tests scoped to files you touched... the full
`scripts/verify.sh` runs once at the very end, by a different agent, after all three lanes are
in"). The prior full green run (lane D, `7ea9651b87`, 40/40, `/tmp/codex-verify-9KJsiq`) is the
last one this repo has recorded; it predates this cycle's own commit and does not cover this
cycle's own changes.

## Sweep population

`corpus_literal_sweep --json-out`: **48706 records examined, 48673 verified, `clean: true`**
this cycle's own run (guard input for the `docs/work-inventory.json` regeneration below). This
cycle adds no new corpus records and touches no corpus JSON files under `data/corpus/` (it
changes engine code and one static Rust table only), so the examined population is expected —
and confirmed — unchanged from any prior clean sweep.

## Oracle pin

`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` — unchanged, carried forward from
`scripts/pcgen-oracle-pin.env`. This cycle's own proficiency transcriptions read the pinned
PCGen checkout's already-ingested corpus JSON (`data/corpus/**/class_feature/**/weapon_and_
armor_proficiency*.json`), itself derived from the pinned checkout, not a separate unpinned
clone.

## Guarded regeneration

`docs/work-inventory.json` regenerated through the guarded path, no `--allow-stamp-loss`
needed, after the rebase landed:

```
cargo run --locked --bin corpus_literal_sweep -- --json-out <scratch>/corpus_literal_sweep_report.json --quiet
-> corpus-literal-sweep: CLEAN
cargo run --locked --bin derived_evaluator_fixture_check -- --json-out <scratch>/derived_evaluator_fixture_check_report.json --quiet
-> (no findings)
CORPUS_LITERAL_SWEEP_REPORT=<scratch>/corpus_literal_sweep_report.json \
DERIVED_FIXTURE_CHECK_REPORT=<scratch>/derived_evaluator_fixture_check_report.json \
cargo run --locked --bin v06_work_inventory -j 6
-> docs/work-inventory.json regenerated, 49438 units, generated_at 2026-09-02T16:05:43Z
```

An earlier attempt at a plain (unguarded) regen was killed deliberately after ~12 minutes
without finishing, once `main()`'s own stamp-loss guard code (`v06_work_inventory.rs:16236-
16258`) was read and confirmed it would `std::process::exit(1)` without writing anything the
moment it reached the write step, for lack of the two report env vars — killing it and running
the guarded path immediately was cheaper than letting a doomed run finish.

## Status: complete (9/38), remainder named by mechanism (29/38), no escalation

## Movement, four buckets

- **Closure:** 9 units — Kineticist, Medium, Mesmerist, Occultist, Vigilante, Psychic,
  Spiritualist, Psion, Shifter — moved bucket D → DONE (real `Computed` receipt, both the
  chassis gate and a real proficiency row landed together).
- **Reclassification:** 0 — the 9 did not move sideways between non-DONE buckets, they closed.
- **Reachability:** 19 — every remaining non-prestige unit in the 29, checked individually
  against `docs/work-inventory.json` (not assumed from the registry list): **10** untabled base
  classes newly gate-eligible via this cycle's own fix (Aegis, Antipaladin, Cryptic, Dread,
  Magus, Marksman, Soulknife, Tactician [`ultimate_psionics`], Vitalist, Wilder) + **7** CRB
  NPC/`Ex-*` classes newly gate-eligible via this cycle's own fix (Adept, Aristocrat, Commoner,
  Expert, Warrior, Ex-Barbarian, Ex-Paladin) + **2** Ultimate Combat classes (Ninja, Samurai)
  that were ALREADY gate-eligible before this cycle (`is_supported_uc_single_class`, the
  pre-existing UC arm Gunslinger already closed under — unrelated to this cycle's own fix) and
  independently confirmed to have no `CLASS_WEAPON_PROFICIENCIES` row either. All 19 share the
  identical remaining blocker regardless of which gate arm covers them: no proficiency row was
  found this cycle, so `combat.baseline_weapon_proficiency_unknown` still claim-blocks every one
  of them, honestly — verified directly by
  `a_class_without_a_new_proficiency_row_still_reports_proficiency_unknown`, which confirms
  every OTHER chassis-gate blocker (`class_chassis.unsupported`, `combat.baseline_unsupported`,
  `defense.total_save.unsupported`, `skill.selected_modifier.unsupported`) is gone for the newly
  gated classes too — no unit closes without its own proficiency row.
- **Note on the registry's 20th class:** `untabled_base_class_chassis`'s own registry lists 20
  classes, but only 19 of them appeared in this 38-unit bucket pre-cycle — **Psychic Warrior**
  is absent from it entirely, carrying a DIFFERENT bucket-D evidence string,
  `class_absent_from_ClassId_ALL_and_book_class_id_enums` (`docs/work-inventory.json`, checked
  directly: `ultimate_psionics:class:psychic_warrior`). Read, not fixed, this cycle: the
  registry inserts it into `modelled_class_books()` under the underscored key `psychic_warrior`
  while `classify()`'s `Kind::Class` lookup uses the corpus display name lowercased, `psychic
  warrior` (a space) — a real naming-convention mismatch, a different mechanism than this
  cycle's own gate/proficiency gap, out of this cycle's scope, named here so it is not lost.
- **Instrument-correction:** 1 — `--class-probe`'s CLI population widened from a stale 27-class
  subset to the full 71-class `modelled_class_books()` set, so the ceiling report it prints is
  no longer blind to every UC/PU/untabled-base-class/prestige/NPC class `main()`'s own
  classification already covers.

## Notes

- **The remaining 29 of 38 split into two real sub-shapes, not one "the rest," verified against
  `docs/work-inventory.json` unit-by-unit, not assumed from a registry count.** 19 classes have
  a real chassis (a gate arm accepts them, whether this cycle's own two new arms for 17 of them,
  or the pre-existing UC arm for the other 2, Ninja/Samurai) but no proficiency record found
  this cycle. The other 10 are prestige classes, deliberately outside this cycle's scope:
  `prestige_class_entry_gate` returns no chassis magnitude by design (its own module doc comment
  states this), so extending `has_supported_class_chassis` to them would fold a fabricated
  BAB/save total into `compute_total_saves`/`compute_combat_baseline` that no real corpus row
  backs. Verified this stays true post-fix by `a_prestige_class_id_still_fails_the_gate`. 19 +
  10 = 29, matching the post-cycle population exactly.
  (`9 closed + 19 reachability + 10 out-of-scope = 38`, the pre-cycle population, also exact.)
- **Proficiency transcription discipline held.** Occultist and Vigilante's own `DESC:` prose
  both say "simple and martial," but their real corpus token carries only
  `TYPE=WeaponProfMartial` — the table transcribes the token, matching the precedent this file
  already sets for Ninja/Samurai (checked, not assumed: the module's existing header comment
  names this exact boundary rule).
- **`Tactician` corpus identifier scope collision, checked and avoided.** `ultimate_psionics`'s
  own base class Tactician (in the 20-class untabled registry) is a distinct corpus identifier
  from Ultimate Combat's Tactician fighter archetype — verified before naming it in a comment,
  per the standing hazard this repo has hit before (a shared name is not a shared thing).
- **10 of the 27 gate-eligible classes carry no matching corpus proficiency record found this
  cycle** — named as real, open, next-cycle scope in the roster comment
  (`weapon_tables.rs`), not assumed Simple-only.

## Next-cycle plan

1. **19** (10 untabled base classes + 7 CRB NPC/`Ex-*` classes + Ninja/Samurai) — the remaining
   gate-eligible classes with a real chassis but no proficiency row found this cycle: search
   each one's own `data/corpus/<book>/class_feature/<class>/weapon_and_armor_proficiency*.json`
   (or sibling naming) individually — the same per-key discipline this cycle and wave 33 lane A
   both used — and add a real `CLASS_WEAPON_PROFICIENCIES` row for every one a record actually
   exists for. Cheapest, highest-value: the chassis gate is already open for all 19 (17 via this
   cycle's own two new arms, 2 via the pre-existing UC arm), so a found row alone closes the
   unit.
2. **10 prestige classes** — out of this cycle's scope by design (no chassis magnitude exists
   in `prestige_class_entry_gate`); any future work here would need to build a real prestige
   BAB/save chassis first, a different and much larger mechanism than this cycle's gate
   widening.
3. **Psychic Warrior's `psychic_warrior`/`psychic warrior` key mismatch** (see Reachability
   note above) — a real, separate, one-line-shaped bug discovered but not fixed this cycle
   (out of this 38-unit shape's own scope; it lives under a different bucket-D evidence string,
   `class_absent_from_ClassId_ALL_and_book_class_id_enums`).
