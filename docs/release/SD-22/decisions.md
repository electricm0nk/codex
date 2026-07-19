---
canonical: true
owner: god-emporer
status: active (operator review 2026-07-15 scope, operator directives 2026-07-17 expanded scope to APG + ACG + Bestiary 1, operator directives 2026-07-18 confirmed ACG + APG are "the two advanced guides"; tranche lane pinned 2026-07-18 to tranche/5 with codex-tranche-5 board repurposed; bundle marked planning-ready)
date: 2026-07-15
canonical_branch: tranche/5 (operator directive 2026-07-18; replaces TBD placeholder; re-uses the dead-state codex-tranche-5 kanban board from the prior 2026-07-16 SD-21 launch that was repurposed to tranche/4-1)
kanban_board: codex-tranche-5 (operator directive 2026-07-18; re-uses the dead-state board from the prior 2026-07-16 SD-21 directive; the loop's Step 10 mint uses `--board codex-tranche-5` explicitly so it works regardless of operator's default-board setting)
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/README.md
mirror_of: /home/ubuntu/workspace/SD-22-content-source-ingest-and-dm-toolkit-scope-draft.md
---

# SD-22 — Decision Record ("Why we did that")

## 1. SD-22 scope is content-source ingest (APG + ACG + Bestiary 1) + DM toolkit (operator directives 2026-07-15, 2026-07-17, 2026-07-18)

**Decision (original 2026-07-15):** per operator directive 2026-07-15 ("With SD-22, we should include at least beastiary 1 rules"), SD-22's original scope was the DM toolkit + encounter builder + Bestiary 1 ingestion work. That bundle is downstream of SD-20 (per-character rules engine) and SD-21 (campaign manager with party-CR math). SD-21's Epic 2 (Campaign Manager + Drive) consumes the party-CR math that SD-22 will provide; SD-21's acceptance criteria reference it without yet grounding it.

**Decision (scope expansion 2026-07-17 — first part):** per operator directive 2026-07-17 ("those need to move to SD-22" — referring to "the two advanced guides"), SD-22's scope expanded to include the two "advanced guide" books. Per the operator's 2026-07-18 clarification ("ACG, APG are the two advanced guides"), the two advanced guides are **the Advanced Player's Guide (APG) and the Advanced Class Guide (ACG)** — not the "Ultimate Combat / Ultimate Magic" pair (those Ultimate-line books are NOT in SD-22's scope; a prior-turn doctrine-of-record flag that read otherwise was operator-corrected and is hereby superseded). The 2026-07-17 #1 directive therefore moves **APG + ACG** from SD-21 to SD-22 (per the lifecycle-routing rule).

**Decision (scope expansion 2026-07-17 — second part):** per operator directive 2026-07-17 ("APG and ACG have moved to SD-22"), SD-22's scope is **explicitly extended to own APG + ACG + the two advanced guides (which are APG + ACG themselves, named twice for emphasis on the ACG side)** + Bestiary 1 + DM toolkit. The two-advanced-guides-vs-APG-and-ACG apparent duplication is **operator-emphasized**, not redundant: both statements point to the same two books (APG + ACG) and the operator wanted both routings recorded for the audit trail.

**Net scope-of-record (post-2026-07-18 clarification):** SD-22 owns **content-source ingest for APG (Advanced Player's Guide) + ACG (Advanced Class Guide) + Bestiary 1** plus the **DM toolkit** (encounter builder, party-CR math, diagnostic surfaces). SD-22 does NOT own Ultimate Combat / Ultimate Magic / any other "Ultimate"-line book — those Ultimate books were never in scope; prior-turn doc references to "the two advanced guides (Ultimate Combat + Ultimate Magic)" were operator-corrected and are superseded by this clarification.

**Why three content-source book lines + DM toolkit + closure machinery in one bundle:**
1. **Content-source ingest is a coherent lane.** APG + ACG + Bestiary 1 are all "load book content into the rules-core corpus" work; SD-22's bundle can grow that as its primary surface. Each book gets its own epic and its own per-cycle work unit (per-class for APG and ACG; per-monster-block for Bestiary 1), but the same source-book sibling-directory pattern under `src/rules_core/rules_tables/<book>/` applies across all three (per SD-19 §9 source-book subdirectories pattern).
2. **DM toolkit belongs with content-source ingest.** The encounter builder and party-CR math consume monster data (Bestiary 1 outputs), class data (APG + ACG outputs), and rule-set-id-tagged spell data (APG + ACG outputs) — they can't be built until content-source ingest has at least populated one book. Bundling them keeps the data dependency inside one bundle.
3. **Per-character rules-engine work is out of scope.** That's SD-20 (Tranche-4) and SD-21 (Tranche-4-1). SD-22 doesn't compete with that lane; SD-22 *feeds* it by populating content.

**Operational consequence for SD-21's epic decomposition.** SD-21's Epic 2 (APG ingestion) and Epic 3 (ACG ingestion) **no longer exist** in SD-21; they're re-homed to SD-22 as Epic 1 (APG) and Epic 2 (ACG). SD-21's `epic-breakdown.md` was rewritten accordingly (2026-07-17); SD-21's Epic 1 is now Code-Side Identifier Cleanup (the governance base requirement), Epic 2 is Campaign Manager + Drive, Epic 3 is Update UI bug, Epic 4 is Closure Epilogue, Epic 5 is Build Version Numbering (with the `<major>.<tranche-base>.<build>` scheme), Epic 6 is Wizard single-class completion, Epic 7 is Multiclass stacking. SD-21 reads from SD-19's `rules_tables/crb/` only.

**Operational consequence for SD-22's epic decomposition (this turn's author).** SD-22's `epic-breakdown.md` (created 2026-07-18 by operator review) lays out **8 epics + 2 promotion gates** (per `epic-breakdown.md` §"Execution lane split"). The 8 epics are: (1) APG content-source ingest; (2) ACG content-source ingest; (3) Bestiary 1 content-source ingest; (4) DM toolkit (encounter builder + party-CR math); (5) Closure Epilogue; (6) Build Version Numbering; (7) Code-Side Identifier Cleanup (the governance base requirement; fires FIRST on shared files per the identifier-discipline doctrine); (8) Operator-pre-launch (board-exists, branch-pushed, OAuth-credentials). The per-class Epic 1+2 sub-stories and per-monster-block Epic 3 sub-stories are operator-pinned at SD-22 cycle launch.

**Operator-recorded open calls (deferred from first issuance):**
- *Per-class Epic 1+2 ordering.* APG's 6-class ingestion order (Alchemist → Cavalier → Inquisitor → Oracle → Summoner → Witch; corrected 2026-07-19 — Gunslinger/Magus are Ultimate Combat/Ultimate Magic content, not APG, per `corpus-source-inventory.md §1`) is operator-pinned at SD-22 cycle launch. ACG's class order (Alchemist → Arcanist → Bloodrager → Brawler → Hunter → Investigator → Shaman → Skald → Swashbuckler → Warpriest + any ACG printing additions) is operator-pinned at SD-22 cycle launch.
- *Per-monster-block Epic 3 ordering.* Bestiary 1's monster block order (e.g. alphabetical-by-CR, alphabetical-by-name, by-environment-then-CR, etc.) is operator-pinned at SD-22 cycle launch.
- *DM toolkit GUI scope.* The DM-toolkit GUI is outside this planning bundle. If a separate GUI-bundle is needed for the DM-toolkit surface (encounter-builder screen, party-CR dashboard), that's a future `SD-23` or similar — *not* SD-22's lane.
- *Book-expansion scope.* If a future operator-pinned directive adds "Ultimate Combat / Ultimate Magic" or other Ultimate-line books to SD-22, that's an addendum to this §1 decision, not a re-authoring. Default-and-flag: Ultimate books remain *out of scope* until operator-pinned.
- *ACMG-and-other-advanced-books.* Beyond APG + ACG (the two advanced guides) and Bestiary 1, no other PF1 books are in SD-22's scope. "Advanced Race Guide" (ARG) and "Ultimate" series are NOT in scope; operator-call only.

## 2. SD-22 launch branch and kanban board are operator-pinned: `tranche/5` + `codex-tranche-5` (operator directive 2026-07-18)

**Decision (operator-pinned 2026-07-18):** SD-22's launch branch is **`tranche/5`** and its kanban board is **`codex-tranche-5`** (the dead-state board from the prior 2026-07-16 SD-21 launch that was repurposed to `tranche/4-1` / `codex-tranche-4-1`).

**Reason:** the dead-state `codex-tranche-5` board was already in the boards list as a 0-card entry (operator-typed earlier in this session before the SD-21 reorg moved SD-21 to `tranche/4-1`). Repurposing the existing dead-state board avoids creating yet another slug and keeps the SD-22 lane visibly adjacent to SD-21's `tranche/4-1` on the merge graph without inheriting SD-21's rules-engine lane.

**Cross-reference discipline.** This is **NOT implicit inheritance**. SD-22's launch handoff documents `tranche/5` / `codex-tranche-5` as its own choice; the bundle's content (APG + ACG + Bestiary 1 + DM toolkit) is genuinely different from SD-21's content (rules-engine + identifier cleanup + build version + multiclass support). The shared name `codex-tranche-5` is reused only for the kanban-board slug — the *branch* `tranche/5` is fresh, the *board* `codex-tranche-5` is reused.

**Operational consequence.** SD-22's `epic-breakdown.md`, `loop-instruction.md`, and any cycle-mint command use `--board codex-tranche-5` explicitly (hard-coded). The promotion PR at SD-22 closure is `tranche/5 → develop`. The closure-flow doctrine (per `../../doctrine-external/spec-domain-lifecycle.md`) opens a documentation PR against `programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/README.md` flipping closure-state frontmatter.

**What SD-22's launch-branch decision is NOT.** SD-22 is **not** a `tranche/4-2` dash-release (that option is reserved for future per-character-rules-engine dash releases if needed); SD-22 is **not** a brand-new trunk branch (`tranche/5` is the canonical Tranche-5 lane, the natural successor to Tranche-4 that closed at SD-20's promotion to develop); SD-22 is **not** inheriting SD-21's `tranche/4-1` (different content, different lane).

**Per-position build-version responsibility** (per the operator's 2026-07-17 `<major>.<tranche-base>.<build>` amendment, applied symmetrically to SD-22): SD-22's first concrete release value lands as `0.5.<current_build>` (major `0` until first main-publish, tranche `5` because `tranche/5` is the base, build is the next monotonic counter value after the last committed build on `tranche/5`).

## 3. Operator-deferred shape decisions for SD-22

**Decision:** multiple operator-deferred shape decisions for SD-22 are recorded as scope-of-record open calls, not blocked on the bundle's first cycle:

- **Closure-state frontmatter field vocabulary.** Per `../../doctrine-external/spec-domain-lifecycle.md` open call #1; SD-21's closure flow is the first worked example and will set the field shape. SD-22's closure flow uses the same shape (mirrors SD-21 Epic 5).
- **Audit-trail size for closure PRs.** Resolved via SD-21 Epic 5 criterion 21's closure-test-suite run as the audit trail. SD-22's closure flow mirrors.
- **Bundle size budget.** Per SD-21's `decisions.md §20`, no bundle-size budget has been pinned. SD-22's eventual size will be whatever the operator pins (8 epics / 30 criteria across this bundle's `epic-breakdown.md`, but the budget is operator-pinned).
- **Per-class Epic 1+2 sub-stories.** The APG 9-class ordering and ACG class ordering (which class lands first) are operator-pinned at SD-22 cycle launch — *not* in this bundle's doctrine.
- **Per-monster-block Epic 3 ordering.** Bestiary 1's monster block ordering is operator-pinned at SD-22 cycle launch.
- **DM-toolkit GUI.** Out of scope for SD-22's source STC; if a GUI-bundle is needed, that's a future `SD-23`.
- **Book-expansion.** Future operator-pinned directives to add Ultimate-line books or other PF1 sources to SD-22 are addendum decisions to §1, not re-authorings.
- **First-main-publish.** Out of scope for SD-22; SD-22's `major` stays `0` until the first publish to `main` (which is also out of scope for SD-22 — that's a release-process decision).

## Cross-reference

- `~/workspace/programs/codex/requirements/SD-21-campaign-manager-and-persistence/decisions.md` §9 — documents the prior SD-22 placeholder scope ("DM toolkit + encounter builder + Bestiary 1") and the 2026-07-17 scope expansion. **Note**: the §9 entry in `SD-21/decisions.md` now correctly encodes both 2026-07-17 directives (advanced guides = APG + ACG; plus the follow-up that explicitly moves APG + ACG to SD-22); the 2026-07-18 operator clarification closed the loop on which two books are meant.
- `~/workspace/governance/spec-domain-lifecycle.md` — sibling doctrine; governs SD-22's lifecycle posture (tranche → develop → closed). The closure-flow doctrine mirrors SD-21's pattern.
- `~/workspace/governance/identifier-discipline.md` — sibling doctrine; governs any code-identifier work in SD-22's per-book epic cycles (Epic 7 fires first on shared files).
- `~/workspace/programs/codex/requirements/SD-19-corpus-aware-compute-seam/` — sibling bundle; the Tranche-3 corpus-source ingest pattern SD-22 inherits from (`rules_tables/<book>/` sibling directories).
- `~/workspace/programs/codex/requirements/SD-20-rules-engine-completeness/` — sibling bundle; per-character rules-engine surface that SD-22's content-source ingest feeds into.
- `~/workspace/programs/codex/requirements/SD-21-campaign-manager-and-persistence/` — sibling bundle; SD-21's Epic 2 (Campaign Manager + Drive) consumes the party-CR math that SD-22 will provide. SD-21 reads `rules_tables/crb/` only; SD-22 owns the other books.
- `~/workspace/SD-22-content-source-ingest-and-dm-toolkit-scope-draft.md` (created 2026-07-18 by operator review) — canonical handoff; carries the prominent-early `/loop /goal` OPERATING METHOD callout mirroring SD-21's new pattern (`/batch` deferred per §5).
- `~/workspace/SD-22-content-source-ingest-and-dm-toolkit-loop-instruction.md` (created 2026-07-18 by operator review) — loop body; the per-cycle procedure, file-touch partition, post-mortem card mint, and progress-doc update.

## 4. Epic 9 — Closure Readiness (eval + self-heal + dispatch); operator directive 2026-07-19

**Decision (operator directive 2026-07-19):** SD-22 grows by one epic — **Epic 9, "Closure Readiness"** — that fires between Epic 8 (Build Version Numbering) and Epic 7 (Closure Epilogue). Epic 9's job is the *final-acceptance gate* the operator previously ran by hand: every criterion 1-30 evaluated against artifact evidence, shortfall diagnosed, self-healing cycles fired if needed, and only after a clean pass does Epic 9 dispatch Epic 7 to do the actual closure work (open the `tranche/5 → develop` PR, generate release notes, etc.).

**Why Epic 9 exists.** Closure PRs are expensive to revert: a regression or missing-artifact that surfaces after promotion puts the operator into a tranche-only-revert. The previous SD-21 closure pattern (Epic 4 = Closure Epilogue) bundled the eval into Epic 4's first-cycle scan, which conflates "I checked the criteria" with "I dispatched the closure PR." Decoupling is the doctrine: an Epic-9-style evaluator epic, with bounded self-healing, gets the eval where it can be re-run cheaply and produces a clean dispatch decision rather than a hand-waved "good enough."

**Cycle ordering update.** Previously 1 → 2 → 3 → 4 → 5 → 6 → 8 → 7. After the directive: 1 → 2 → 3 → 4 → 5 → 6 → 8 → 9 → 7. Epic 9 *must* run before Epic 7 starts; the loop's cycle picker refuses Epic 7's eligibility until Epic 9's criterion-31 is `complete`.

**Criterion-31 (Epic 9's single acceptance criterion).** A composite criterion covering: (a) every criterion 1-30 has artifact evidence in `docs/release/SD-22/artifacts/` cross-referenced from `progress.md`; (b) `progress.md` status-matrix claims for criteria 1-30 match the artifact-evidence survey (a `complete` claim with no artifact is a *shortfall*, not met); (c) if any shortfall exists, the self-healing cycle(s) have landed and a fresh pass shows 30/30 clean; (d) when (a)-(c) hold, Epic 9 dispatches Epic 7 (opens the Epic 7 cycle on the kanban board, marking Epic 7 as eligible).

**Self-healing boundary.** Self-healing is **open-ended** until the goal is met per operator directive. Judgment calls during self-heal (e.g., "this rule-table entry looks suspicious but is technically correct") are *not* remediated by Epic 9 itself — they are logged to `risks-and-open-questions.md` §"Open judgments deferred to next SD" for the *next* bundle's audit. Closure PR reverts are not on Epic 9's option tree.

**What Epic 9 explicitly does NOT do.**

- Does not run the closure test suite (`cargo test --workspace`). That's Epic 7's run-the-test-suite pre-PR gate.
- Does not open the `tranche/5 → develop` PR. That's Epic 7's `gh pr create`.
- Does not generate release notes. That's Epic 7.
- Does not increment the version on tranche promotion (`0.5.<last_build>` → `0.6.0`). That's Epic 7 (mirrors SD-21 Epic 4).

**Numeric consequences.** Total criteria count moves from 30 to 31. Epic 9 carries 1 criterion (criterion-31). Epic 7's five criteria (22-26) are unchanged; Epic 8's four criteria (27-30) are unchanged. The only new number is criterion-31 in Epic 9.

**First-runner doctrine.** Epic 9 is a *first-here* addition; SD-23+ bundles should adopt the same Epic-N+1 closure-readiness shape rather than continuing to bake eval into the closure epilogue. SD-21's Epic 4 closure pattern retroactively has this gap recorded for future SD-21-clone audit (`risks-and-open-questions.md` §"SD-21 retroactive note: closure eval should be its own epic").

**Recorded.** Added 2026-07-19 mid-conversation after operator surfaced the doctrine during a release-package sweep. Mirrored to all `docs/release/SD-22/*` copies (operator workspace + repo-local).

## 5. Corpus source is real PCGen LST data via `src/pcgen_import/` (corrected 2026-07-19); `/batch` deferred (operator directives 2026-07-18)

**Superseded framing (recorded for the audit trail, in force 2026-07-19T00:00Z–2026-07-19T04:00Z only):** an earlier version of this §5 held that no corpus source existed anywhere and that Epic 3/4/5 cycles should generate `corpus/<book>_<unit>.json` from the model's own OGL/SRD memory. A cloud loop cycle (Epic 3, Alchemist, cycle 1) correctly refused to act on that framing — it tried `aonprd.com` and `d20pfsrd.com` (both HTTP 403 from the sandbox), found no in-repo source, and declined to transcribe class content from training-data recall, citing `AGENTS.md`'s no-fabrication rule and the exact precedent `rules_tables/crb/class_tables.rs` already documents. It logged a real `## Open blockers` entry instead of forcing a commit. That refusal was correct and surfaced this decision's error.

**Decision (corrected 2026-07-19, operator directive):** the real corpus source is **PCGen's published `.lst` data** — the same source SD-19 used for the CRB (`technical-requirements.md` §2, `technical-design.md` — CRB's class tables were "sourced verbatim from real PCGen corpus records"). SD-22 mirrors that pipeline exactly, via the existing, tested ingest engine at `src/pcgen_import/` (parsers in `src/pcgen_import/lst_parser/*`, `ir_converter.rs`). No new parsing code is needed; APG/ACG/Bestiary-1 are new *inputs* to an engine that already exists.

**Where the LST data lives:**
- Locally: the sibling repo `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/{advanced_players_guide,advanced_class_guide,bestiary}/` (already on this machine; this is the same tree SD-19's `CORPUS_ROOT`/`PCGEN_CORPUS_ROOT`-gated tests point at for CRB).
- In a cloud/remote sandbox that only clones `codex`: add `https://github.com/PCGen/pcgen` (the public upstream PCGen repo, which the local sibling repo's `origin` remote confirms is the canonical source) as a second git source alongside `codex`, then use the same `data/pathfinder/paizo/roleplaying_game/<book>/` path inside that checkout.
- APG: `apg_classes.lst` (class chassis: HD/BAB/saves per `CLASS:` records), `apg_abilities_class.lst` (class features), `apg_equip_*.lst`, `apg_feats.lst`, spells embedded per-class. ACG: `acg_classes.lst`, `acg_abilities_class.lst` (or equivalent — confirm exact filename per cycle), `acg_equip.lst`, `acg_equipmods.lst`, `acg_feats.lst`. Bestiary 1: `b1_races.lst` (monster stat blocks are PCGen "races"), `b1_abilities_race.lst`, `b1_equip_*.lst`, `b1_templates.lst`.

**Ingest shape per cycle.** A cycle parses the relevant `.lst` record(s) with the existing `src/pcgen_import/lst_parser/*` functions (`parse_class_entries`/`parse_class_file` for classes, `parse_lst_entry` for race/monster records, `parse_lst_spell_row`/`_file` for spells, `parse_equipment_entries`/`_file` for equipment), runs them through `ir_converter::convert_to_ir` (or the specific `convert_*` helper), and hand-populates `rules_tables/<book>/<file>.rs` from the resulting `SourceContentRecord`s — same shape as `rules_tables/crb/class_tables.rs`. The Rust module's doc comment cites the source `.lst` file and record key (e.g. `CLASS:Alchemist` in `apg_classes.lst`) as provenance, mirroring the CRB precedent. Per `rules_tables/crb/class_tables.rs`'s own documented scope boundary, named per-level feature *text* beyond what a formula or a directly-transcribed LST field provides remains out of scope unless the LST record carries it verbatim — this is not a license to expand scope, only to use the same verified-source discipline CRB already established.

**Consequence for `corpus-source-inventory.md`.** That file's "Content shape" prose columns (authored 2026-07-19 before this correction, from memory, not from the LST source) are **not authoritative** — see the corrective banner added at the top of that file. The `rust_module_path`/`test_fixture_path`/`cycle_artifact_path`/`RuleSetId` columns remain valid routing information; the prose describing named class features per class does not and must be re-derived from the real `.lst` record before a cycle ships it.

**Operational consequence.** The loop-instruction's SD-22-specific hard stop is restored to its original shape (a cycle blocks only when the LST tree is genuinely unreachable in that session's environment, or the specific record isn't found in it — not "operator hasn't supplied a file," since the file already exists). The pre-launch checklist item is satisfied: real structured-data input exists today at the paths above.

**Decision (`/batch` deferral — operator directive 2026-07-18, unchanged):** SD-22 launches as **`/loop 60m /goal ./loop-instruction.md`** — *without* `/batch`. `/batch` may be re-added later only when ≥2 book lanes are genuinely parallel-eligible under the file-touch partition.

**Decision (`/batch` deferral — operator directive 2026-07-18):** SD-22 launches as **`/loop 60m /goal ./loop-instruction.md`** — *without* `/batch`. `/batch` may be re-added later **only** when ≥2 book corpora exist on disk and the book lanes (Epic 3 APG / Epic 4 ACG / Epic 5 Bestiary 1) are genuinely parallel-eligible under the file-touch partition. At launch, zero corpora exist and Epic 1/2 are single-stream by the dependency graph, so `/batch` would buy nothing and add multi-stream collision surface.

**Recorded.** Added 2026-07-18 at SD-22 launch planning (this session's pre-launch verification + operator directives). Mirrored to all `docs/release/SD-22/*` copies (operator workspace + repo-local). The decision record is now 5 items.
