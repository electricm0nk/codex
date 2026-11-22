# Criterion 31 — `decisions.md`/`risks-and-open-questions.md` final review (cycle 16)

## `risks-and-open-questions.md`

Added a "Closure — final review (Criterion 31, 2026-07-21)" section confirming status for every original item:
- **R1-R5:** R1 and R2 remain latent/deferred as designed (out of SD-23's Option-A scope); R3, R4, R5 resolved favorably (SD-22 closed before SD-23 launched, zero accidental stubs surfaced, no file-touch collision).
- **OQ1-OQ2:** both resolved exactly as anticipated during execution — `CampaignMember.members: []` shipped as a valid empty campaign (Epic 4); `level_up_character` shipped without multiclass-gating validation (Epic 5).
- **D1-D4:** all four confirmed still deferred/unchanged, as designed.
- **New item recorded:** the graphify-bootstrap gap discovered in Criterion 27 (not part of the original R/OQ/D set, since it's a tooling gap discovered during closure, not a bundle-scope risk) — flagged for a future bundle or manual operator action.

## `decisions.md`

Added §16, the closure final entry: bundle status (CLOSED, 33/33 criteria, 16 cycles), both promotion PR URLs and merge commits, the corrected build counter (`0.5.97`, tranche-base unchanged), a summary of the assignee/lifecycle correction holding for the rest of the bundle after cycle 5, the standing audit false positive, and the two real gaps found and fixed during closure (the architecture-truth-up checker fix, the export/import round-trip fix).

Commit SHA: recorded in this cycle's combined closure-docs commit.
