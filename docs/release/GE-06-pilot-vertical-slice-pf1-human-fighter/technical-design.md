---
        title: GE-06 Technical Design
        stc_id: STC-CODEX-GE-06
        artifact_type: technical-design
        status: draft
        scope: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter
        source_stc: ./README.md
        source_artifacts:
          - ./technical-requirements.md
          - ./references/upstream-dependency-contract.md
          - ../GE-02-canonical-rules-model-and-content-packages/artifacts/canonical-model-specification.md
          - ../GE-04-rules-engine-and-explainability-core/artifacts/pilot-golden-computation-fixture-requirements.md
          - ../GE-05-oracle-validation-and-parity-harness/references/upstream-dependency-contract.md
        ---

        # GE-06 Technical Design

        ## Objective
        Describe the design response to GE-06: how the first integrated pilot proof path should be structured, what each upstream epic still owns, which payload boundaries need to exist, and how later execution should stay narrow enough to produce evidence instead of theater.

        ## Design posture
        GE-06 is an **integration contract**, not a subsystem owner.

        It should answer one question only:

        > Can the current Codex architecture carry one real PF1 Human Fighter level 1 path from grounded source content to computed behavior to selected oracle comparison to product-visible UI without hiding failures?

        That means GE-06 must orchestrate evidence across layers without stealing ownership from the layers.

        ## Integrated proof path
        ```mermaid
        flowchart LR
          A[GE-01 Legacy Corpus
Token families + matrix] --> B[GE-03 Import Bridge
Canonical content + provenance + diagnostics]
          C[GE-02 Canonical Model
Model homes + runtime boundary] --> B
          B --> D[GE-04 Rules Core
Compute + explain + diagnose]
          D --> E[GE-05 Oracle Comparison
Selected old vs new evidence]
          D --> F[GE-07 UI Consumer Boundary
Minimal product-visible slice]
          E --> G[GE-06 Viability Review]
          F --> G
          B --> G
        ```

        The design rule is simple:
        - **GE-01/GE-02/GE-03/GE-04/GE-05/GE-07 own the layer truths.**
        - **GE-06 owns only the integrated claim about whether those truths compose into a viable pilot.**

        ## Ownership split
        | Surface | Owns | GE-06 may consume | GE-06 must not do |
        |---|---|---|---|
        | GE-01 | grounded pilot corpus, token families, conversion posture | token-family gate list and grounded source boundaries | invent new token taxonomy or hide unsupported-token risk |
        | GE-02 | canonical model homes and source/runtime boundary | required canonical-object list and relationship expectations | create new canonical homes because integration is inconvenient |
        | GE-03 | importer, provenance, conversion-report posture | import evidence boundary and diagnostic obligations | claim import success without the importer surface |
        | GE-04 | compute, explanations, diagnostics, headless rules truth | fixture, derived-output, explanation, and diagnostic obligations | claim rules correctness or explanations without GE-04 evidence |
        | GE-05 | oracle comparison, normalization, known-gap posture | selected parity-dimension contract and claim-tier boundary | redefine parity or call outputs oracle-checked early |
        | GE-07 | desktop shell and broader UX architecture | minimum UI truth contract | replace GE-07 with a one-off UI story inside GE-06 |

        ## Required payload surfaces
        The integrated slice needs explicit payload boundaries so later implementation can stay auditable.

        ### 1. Import payload
        Minimum fields:
        - source package identity
        - imported object references or counts relevant to the pilot
        - provenance/source-map references
        - importer diagnostics and unsupported/lossy posture

        ### 2. Character-fixture payload
        Minimum fields:
        - case ID
        - grounded selections from the charter
        - closed deterministic selections from `artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md`
        - chosen output categories
        - expected diagnostic and known-gap posture

        ### 3. Computed payload
        Minimum fields:
        - character input echo
        - selected derived outputs
        - explanation references
        - failed-prerequisite / unavailable-choice outputs
        - engine diagnostics

        ### 4. Parity payload
        Minimum fields:
        - selected comparison dimensions
        - old-system source reference or blocker
        - new-system source reference
        - normalization rule or known-gap status
        - comparison result and claim tier

        ### 5. UI projection payload
        Minimum fields:
        - product-visible character summary fields
        - explanation affordance references
        - diagnostics visible to the UI
        - blocked/unavailable state visibility

        ### 6. Viability payload
        Minimum fields:
        - per-layer pass/block status
        - failure category and primary owner
        - claim-tier status for each selected output dimension
        - narrow/expand/rework recommendation

        ## Narrow-first implementation guidance
        GE-06 should not begin with the desktop UI.

        The first credible implementation path is:
        1. finalize the integrated pilot fixture boundary
        2. prove the headless import/compute/explain path
        3. layer selected oracle comparison on top
        4. only then surface the same real data in a minimal UI view
        5. write the viability review against evidence, not intuition

        This sequence preserves the program doctrine that UI truth is downstream of headless domain truth.

        ## Failure-classification design
        GE-06 should classify failure by the **first broken contract**, not the last visible symptom.

        Examples:
        - if the pilot cannot be represented canonically without semantic collapse, the primary owner is **model flaw** even if the UI later looks empty
        - if canonical content exists but cannot be loaded with grounded provenance, the primary owner is **importer flaw**
        - if imported content exists but derived values or explanations are wrong or absent, the primary owner is **engine flaw**
        - if computed outputs exist but selected old-vs-new comparison evidence is absent or non-comparable, the primary owner is **oracle gap**
        - if headless outputs exist but the UI hides diagnostics or depends on mock state, the primary owner is **UI gap**

        ## Viability-review posture
        The final GE-06 review should not ask “does the demo feel promising?”

        It should ask:
        - which selected outputs reached `Converted`, `Computed`, `Oracle-checked`, and `Product-visible`
        - which outputs are blocked and why
        - whether the remaining gaps are local and bounded or structural and disqualifying
        - whether the correct next move is narrow, expand upstream requirements, or stop

        ## What this design does not decide
        This design deliberately does **not** decide:
        - final code module boundaries in `/home/ubuntu/workspace/repos/codex`
        - final old-system command route for PCGen output capture
        - final UI framework choices or packaging constraints owned by GE-07
        - final fixture values for parity
        - final branch/worktree and write scope for a later code-authorizing handoff

        Those truths belong to later bounded readiness closure and handoff work.
