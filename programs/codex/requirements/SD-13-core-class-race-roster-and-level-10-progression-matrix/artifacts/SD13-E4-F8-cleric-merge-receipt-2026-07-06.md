# SD13-E4-F8 Cleric slice — merge receipt

Slice: SD-13 Class/Cleric — spellcasting-class progression + spell burden
branch: feat/sd13-class-cleric-bounded-burden
branch_base_sha: c78287cce76d3cce10fe814806558976fcfd70543 (origin/develop)
merge_target: develop
pr_draft: feat(sd13-e4-f8): cleric level-1 prepared divine spell-burden slice (to be opened)
generated_by: tech-priest (Magos Ferrix-9) on 2026-07-06
kanban_card: t_6577e5bf
parent_card: t_5d57e115

## Matrix row state change

row_id: class.cleric.progression_and_spell_burden
subject_type: Class
subject_id: class:cleric
before:
  support_state: Unverified
  evidence_tier: Observed
  evidence_freshness: AwaitingInitialEvidence
  grounding_ref: SD13_ROSTER_MATRIX_DOC
  blocker_or_lossiness_note: ""
after:
  support_state: Blocked
  evidence_tier: Computed
  evidence_freshness: RefreshableFromLiveProof
  grounding_ref: SD13_CLERIC_LEVEL1_TEST (= "tests/sd13_cleric_level1_spell_baseline.rs")
  blocker_or_lossiness_note: "SD13-E4-F-Cleric leaves direct computed evidence \
      that the deterministic Human Cleric level-1 prepared divine spell-bearing \
      identity is recognized on the compute seam, but the row stays blocked: the \
      deity/domain burden (deity selection, favored weapon, domain selection, \
      domain power, and any other deity- or domain-granted class features) is \
      not implemented, and the divine prepared spell posture burden (orisons, \
      spells prepared, spell slots per day, bonus spell slots from a high Wisdom \
      score, and spell save DCs) is not computed. No spell math is fabricated and \
      no Cleric level 2+ is proven"
  next_required_uplift: "SD13-E4 Cleric deity/domain mechanics slice, then \
      prepared spell posture and level-2+ progression"

This reclassification keeps the row honest. The slice leaves direct computed evidence
that the level-1 prepared divine spell-bearing class identity is recognized on the
compute seam — i.e. `class:cleric:1` is no longer dropped as an undocumented packet
placeholder — but explicitly distinguishes and blocks the two named burdens:

1. The deity/domain burden (deity selection, favored weapon, domain selection,
   domain power, deity-granted class features).
2. The divine prepared spell posture burden (orisons, spells prepared, spell slots
   per day, high-Wisdom bonus-spell slots, spell save DCs).

These are two distinct `ComputationDiagnostic` blockers, not a collapsed single
diagnostic: `class_feature.cleric.deity_domain.unsupported` and
`class_spell.cleric.prepared.unsupported`. The chassis stays recognized; the
spell-burden stays blocked. That is the project-accepted `Blocked / Computed`
shape — same posture sibling slices use for Sorcerer / Paladin / Ranger hybrid
rows.

## Chassis vs spell burden classification

Chassis: `class:cleric` level-1 identity recognized on the compute path
(`class_chassis.spell_baseline.cleric` direct evidence), Human race-choice seam
preserved (Human ability-bonus + Human bonus-feat), single-class level-1 only.
Status: computed but bounded.

Spell burden (separated, not collapsed):
  - Deity/domain burden: not implemented; named blocker
    `class_feature.cleric.deity_domain.unsupported`.
  - Divine prepared spell posture burden: not implemented; named blocker
    `class_spell.cleric.prepared.unsupported`.

Combined posture stays `Blocked` (i.e. "recognized as a real class but not yet
supported end-to-end"). No spell math fabricated. No level-2+ promoted.

## Files changed in this slice

Modified:
  - src/rules_core/pilot_compute.rs
      + explain_cleric_level1_spell_baseline(): direct computed recognition
        of the prepared divine spell-bearing identity (id
        "class_chassis.spell_baseline.cleric") with two distinct
        claim-blocking diagnostics (one for deity/domain, one for prepared
        spell posture). Wired into compute_pilot_base_chassis.
  - src/rules_core/support_state_matrix.rs
      + SD13_CLERIC_LEVEL1_TEST constant.
      + class.cleric.progression_and_spell_burden row reclassified from
        Unverified/Observed/AwaitingInitialEvidence to
        Blocked/Computed/RefreshableFromLiveProof with the explicit blocker
        note quoted above.
  - tests/sd13_support_state_matrix.rs
      + "class:cleric" added to proven_subjects.
      + "class.cleric.progression_and_spell_burden" added to
        expected_above_observed and to EXPECTED_REFRESHABLE_FROM_LIVE_PROOF
        (now 9 entries).
      + Count assertion updated from 6 to 5 remaining unproven core class rows.

Added:
  - tests/fixtures/rules_core/pf1_human_cleric_level1_sd13_deterministic_input.txt
      Bounded Human Cleric level-1 fixture with deity, alignment, two domains
      (PF1 Cleric selects two domains at level 1), Human bonus-feat, and Human
      ability-bonus selections; no spell math, no domain-power resolution, no
      level-2+ data.
  - tests/sd13_cleric_level1_spell_baseline.rs
      11 dedicated proof tests: direct recognition evidence, no fabricated
      spell math, two distinct claim-blocking diagnostics, integrated blocked
      posture, Human race seam preservation, negative controls (Fighter /
      Sorcerer / Rogue do not gain Cleric recognition, Cleric level-2 is not
      promoted), matrix Cleric row pinning, other spell-bearing rows stay
      truthful, and no row promoted to Supported/Lossy.

Forbidden write scope: the matrix file
  programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md
was NOT touched. The slice updates the matrix seed carrier in
  src/rules_core/support_state_matrix.rs
which is the single source of truth, consistent with the sibling Sorcerer / Paladin /
Ranger slices. This merge-receipt file is the only artifact written under
programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/
— it documents the matrix row state change without editing the read-only matrix file
itself, in line with the card body's "permitted ONLY via the same PR that lands the
slice, in the merge-receipt field, NOT by hand" doctrine.

## Verification

  - cargo test --test sd13_cleric_level1_spell_baseline: 11 passed; 0 failed
  - cargo test --test sd13_support_state_matrix: 26 passed; 0 failed
  - cargo test (full): 130 passed; 0 failed; 0 ignored across 24 test binaries
  - cargo build: clean
  - Forbidden matrix file unchanged

The slice is GREEN end-to-end. The matrix row reclassification is internally
consistent across the carrier code (`src/rules_core/support_state_matrix.rs`) and
the dedicated proof surface (`tests/sd13_cleric_level1_spell_baseline.rs`). The
five remaining core class rows (Barbarian, Bard, Druid, Monk, Wizard) stay
correctly `Unverified / Observed`. The accepted Sorcerer / Paladin / Ranger
truth is intact.