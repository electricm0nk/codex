# SD-32 Risks and Open Questions

## Primary risks

1. **Chassis-design scope creep.** "Build the missing race chassis" could balloon into a full
   re-architecture of `RaceCorpus` if not bounded. Epic 1-F1's design decision must state explicitly
   what is and is not being rebuilt, and cite the 18 already-modeled races' resolution path as the
   contract not to break (AT-32-002).
2. **Classifier gaming risk, restated.** This is explicitly "ranked #2 by ceiling and #1 by gaming
   risk" (Decision 1(b)'s own rationale) — the highest-risk work in the entire split three-package
   program for accidentally moving a number by lowering a bar. The labelling-gate-first discipline
   (Epic 2-F1 before F2) is the primary defense; a cycle under schedule pressure is exactly where this
   risk is highest.
3. **Handoff verification gap.** If this package reports a chassis/classifier "done" but
   `SD-31-corpus-closure-grind` never actually consumes it (e.g., its ingest cards stay stuck on a
   different blocker), the capability sits unused and the 100% mandate does not move despite both
   packages reporting progress. AT-32-004's two-sided citation requirement exists specifically to catch
   this.

## Open questions

1. **Does the race chassis need a full `RaceCorpus` entry per race, or is a narrower shim sufficient
   for `race_trait` grounding?** Not yet decided — Epic 1-F1's design decision answers this; flagged
   here so a cycle does not silently pick the expensive answer without recording why.
2. **What is "genuinely unreachable" for the verdict-path epic, post-classifier?** The classifier may
   confirm some units have no possible verdict path even after this epic's build (e.g., content this
   repo's engine model fundamentally cannot represent). Epic 2-F3 requires these be named findings, not
   silently left in `unknown` — but the disposition of a genuinely-unreachable finding (does it get its
   own successor SD, or does it stay a permanent named exception to the 100% mandate) is an operator
   question this package does not answer unilaterally.
