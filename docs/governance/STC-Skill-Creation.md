---
title: STC Skill Creation — Harness-Side Recommendations for Skill Authors
stc_id: GOV-STC-SKILL-CREATION
canonical: true
owner: Todd Hintzmann
scope: universal
status: active
review_state: accepted
last_reviewed_at: 2026-08-22
canonical_source: ~/workspace/repos/codex/docs/governance/STC-Skill-Creation.md (this file)
related_artifacts:
  - ./workflow-instruction-template.md (every bundle's workflow-instruction.md is authored from this)
  - ./no-stub-mvp-doctrine.md (referenced by §6's dual-audit gate)
  - ./wired-integration-stubs-registry.md
  - ../release/template/template.md (every bundle's README.md is authored from this)
  - ../doctrine-external/identifier-discipline.md (forbidden source-identifier patterns)
upstream_targets:
  - any contributor authoring a skill in their own harness profile to interact with this project's STC chassis
date: 2026-08-22
---

# STC Skill Creation — Harness-Side Recommendations for Skill Authors

This document is **for a person writing SKILL.md files in their own harness profile** to interact
with the Codex STC chassis. It does *not* duplicate the project-local doctrine, the canonical
templates, or the live skills. It recommends the *interface contract* a skill should honor so
that bundles authored against this project's chassis behave correctly when the skill loads.

If you have Hermes profile access, the live skills at
`~/.hermes/profiles/<your-profile>/skills/{standard-template-constructs,spec-domain-bundle-authoring,workflow-instruction-doctrine,governed-workflow-doctrine}/`
are the *implementation* of this contract. This document is the *interface*.

## 0. Why this document exists

A contributor's harness can author a bundle correctly against `docs/release/template/template.md`
without ever knowing what an STC *is* in the abstract. The chassis template carries the bundle
shape; the workflow-instruction template carries the dispatch shape; the governance docs carry the
discipline. Skills on the contributor's profile are the *executor* — they read the templates,
enforce the discipline, and emit the receipts.

A contributor's skill fails in three predictable ways:

1. It **invents its own template shape** instead of mirroring `docs/release/template/template.md` —
   bundles ship inconsistent and the next cycle's verifier trips the dual-audit gate.
2. It **disables the dual-audit gate** because the bundle's first cycles look noisy — the
   `identifier-discipline` and `wired-integration-discipline` checks fire on every cycle, including
   clean ones, and a contributor who doesn't know the gate is supposed to fail-and-self-heal will
   silence it.
3. It **loads at the wrong phase** — the skill is invoked before the bundle is
   `planning-ready`, or invoked *inside* a per-cycle procedure where it should be invoked at
   bundle-launch, or invoked by the orchestrating session where it should be invoked inside a
   dispatched `agent()` call.

This document recommends the interface contract that prevents those three failure modes.

## 1. The interface contract — seven rules

### Rule 1 — Mirror the chassis template, don't fork it

The canonical bundle shape lives at `docs/release/template/template.md`. Every bundle's
`README.md` is authored from that template; every bundle's `workflow-instruction.md` is authored
from `docs/governance/workflow-instruction-template.md`.

**Recommendation.** When your skill needs to *generate* a new bundle's chassis, it must read
both templates and emit them verbatim, replacing only the bundle-specific fields (slug, branch,
build version, owner). Any field marked with `<...>`-style placeholder syntax in either
template is **for the contributor to fill in**, not for the skill to invent content for.

**Worked example (SD-32, 2026-08-22).** The chassis was filled out by hand from four source
documents (`README.md` + `scope-draft.md` + `epic-breakdown.md` + `HANDOFF.md`) that the
predecessor session had produced. The fill-out did *not* require a skill — it required reading the
template and harvesting from existing prose. A skill that tries to *generate* SD-32's
`scope-draft.md` from a single sentence will produce a shallow document. The interface contract
is: the skill produces the chassis shape; the human fills in the content from the predecessor's
docs.

### Rule 2 — Respect the readiness gate

Bundles carry a `status: planning-ready | ...` field in their README frontmatter. A bundle is
*not* dispatchable until it is `planning-ready`. The pre-launch checklist in
`docs/governance/workflow-instruction-template.md §1` enumerates the eight checks that gate launch.

**Recommendation.** Your skill should refuse to launch a bundle whose status is not
`planning-ready`, and should refuse to skip a checklist item. The checklist's item 7 ("Build
counter captured, not left as a template placeholder") is the most-violated: contributors leave
`0.X.<build_at_launch>` in shipped files, ship them, and the next cycle's receipt quotes a literal
template marker. The fix is mechanical — read
`apps/desktop/package.json` + `apps/desktop/src-tauri/tauri.conf.json` (NOT root `Cargo.toml`,
which stays pinned at `0.1.0`), capture the literal next value, and write it into all three call
sites in one commit.

### Rule 3 — Dispatch from the orchestrating session; execute inside dispatched agents

The orchestrating session is the *planner and launcher*. Per-cycle work — the RED→GREEN TDD
loop, the file mutations, the commit and push, the receipt writing — happens inside a
dispatched `agent()` / `Workflow` call. The orchestrating session's own `Edit` / `Write` /
`Bash` calls are reserved for read-only investigation and authoring planning docs.

**Recommendation.** Your skill should emit *one* dispatch primitive per cycle, not zero
(orchestrator does the work itself) and not many (orchestrator fragments the work). The
primitive is whatever the harness exposes — `Workflow` tool, `agent()` call, or a properly-shaped
sub-process invocation. The skill must NOT bypass this and `Edit` a shipping-code file from the
orchestrating session's own tool calls, even when "the fix is one line." That bypass is the
load-bearing anti-pattern; one bypassed cycle erodes the orchestrator/executor boundary and the
next cycles follow.

### Rule 4 — Honor the dual-audit gate on every cycle

Every cycle's diff against the bundle branch is audited for two patterns:

1. **Identifier audit.** No `sd[0-9]+_` / `SD[0-9]+_` / `Sd[0-9]+` patterns, no `t_<hex>` kanban
   tokens, no `SD-N-Ex...` audit IDs. Doctrine-of-record:
   `docs/doctrine-external/identifier-discipline.md`.
2. **Wired-integration audit.** No `STUB` / `MOCK` / `placeholder` / `not yet implemented` /
   `todo` / `fixme` / `hack` tokens in shipping code. Doctrine-of-record:
   `docs/governance/no-stub-mvp-doctrine.md` + the stubs registry at
   `docs/governance/wired-integration-stubs-registry.md`.

**Recommendation.** Your skill must run *both* greps on the cycle's diff and refuse to commit
when either fires. A single-token violation is self-healable inline (re-audit and continue); a
violation that names a real stub in shipping code is **non-self-healable** and the cycle exits
FAIL with a `## Open blockers` entry. The skill must NOT silence the gate, must NOT run it only
on certain cycles, and must NOT treat "all-green last 5 cycles" as permission to skip it on the
sixth. **The trailing `\b` in the identifier grep is deliberately omitted** — `\b` does not
match between `_` and a following word character, so a `\b` would silently fail to catch real
identifiers like `sd19_class_catalog`. Do not add it back.

### Rule 5 — Pin and quote the corpus SHA in every re-derive receipt

Many cycles quote a figure that is re-derived from a corpus — units done, coverage percentage,
denominator count. The hash of the corpus those figures were re-derived against is part of the
receipt, not an aside.

**Recommendation.** Your skill must read `scripts/pcgen-oracle-pin.env`'s `PCGEN_ORACLE_SHA`
field at cycle start and write it into every cycle receipt that quotes a corpus-derived
figure. A figure re-derived against an unstated corpus commit is not re-derived; it is a number
that may have drifted between sessions. The cycle receipt schema lives at
`docs/governance/workflow-instruction-template.md §7`.

### Rule 6 — Wire fixtures from bytes the engine does not read

When an engine emits values that are checked against a fixture, the fixture's expected values
must be transcribed from bytes the engine's read path does *not* touch. A fixture transcribed
from the same file the engine reads is a mirror, not a check — it will validate a fabrication
of the same wrong value the engine produced.

**Recommendation.** Your skill, when verifying an engine cycle, must point `--expected-from` at a
corpus-side file that is *outside* the engine's read graph. For the formula interpreter, this is
the PCGen corpus's expected-value corpora; for a generator, this is the corpus the generator was
supposed to reproduce. The discipline-of-record is `SD-31-corpus-closure-grind/artifacts/THE-BOX.md`'s
fixture check. A cycle without this is not a closed cycle even if `cargo test` is green.

### Rule 7 — Log retro events throughout; write and cite the retrospective at closure

Git records what landed; it records nothing about what nearly landed wrong or who caught it.
`AGENTS.md §Retrospective Logging` and `scripts/retro.py` are this project's mechanism for
capturing that, and SD-31's own retrospective (`docs/retro/sd31-retrospective.md`) is grounded in
its 1,940-event log rather than recollection *because* the logging happened throughout the run,
not as a single write-up at the end.

**Recommendation.** Your skill must (a) emit a retro event — `correction` / `incident` /
`deferral` / `rework` — the moment it catches one, not batched at cycle end; (b) after every epic,
run `scripts/retro.py summary --since <epic-start>` and fold the result into that epic's closing
receipt; and (c) at bundle closure, run the full-bundle summary, write it up as
`docs/retro/<bundle-slug>-retrospective.md` in the shape `sd31-retrospective.md` uses, and **cite
it from the bundle's own `references/README.md`** — a retrospective nobody links from the package
it's about is a completeness gap, not a formality (an SD-32 chassis review found and fixed exactly
this). The full procedure lives in `docs/governance/workflow-instruction-template.md §§2.3, 10–11`.

## 2. Failure modes your skill must NOT introduce

These are the recurring ways contributor skills break the chassis. Each one has a concrete
incidence in this project's history.

**2.1 Counterfeit completion.** The skill reports the bundle as closed when the diff is empty
or trivially green. The receipt reads "complete" but the acceptance criterion was met by
running a stub. **Recommendation.** The skill must re-derive the cycle's claimed outcome against
the live corpus (Rule 5) and the audit greps (Rule 4) on every cycle. An empty diff is itself a
signal — log it, do not celebrate it.

**2.2 Bypassing the orchestrator/executor boundary.** Per Rule 3. Single most-violated
discipline. **Recommendation.** The skill should refuse `Edit` / `Write` calls from the
orchestrating session that target paths inside the bundle's implementation trees
(`apps/desktop/`, `apps/desktop/src-tauri/`, `src/`, `scripts/`). The orchestrating session's
own tool calls are reserved for read-only investigation and planning-doc authoring.

**2.3 `git stash` from a subdirectory.** `git stash` stashes the whole shared checkout even when
called from a subdirectory. The repo's standing rule — *never `git stash` in this repo* — exists
because the bare form has bitten this program three times. **Recommendation.** Your skill should
not invoke `git stash` for any reason. To read a HEAD baseline, use `git show HEAD:<file> > /tmp/<file>`;
to read a sibling worktree's state, use a separate worktree.

**2.4 `find -newermt` for file freshness.** On this box, agent-file mtimes run slightly ahead of
system time, so `find -newermt` reports zero for a file written seconds ago. **Recommendation.**
Use a Python mtime comparison when freshness matters, or use `git status --porcelain` (which
reads the index, not the filesystem).

**2.5 Omitting `model` on a sub-agent call.** When the orchestrating session is on Opus and a
contributor's skill dispatches a sub-agent without explicitly setting `model`, the sub-agent
inherits Opus — and burns weekly quota fast. **Recommendation.** The skill's dispatch primitive
must set `model` explicitly. The standing tiering: Sonnet for build and integration, Opus only
for adversarial verification, Haiku for housekeeping.

**2.6 Shipment of `<...>` placeholders.** Per Rule 2's item 7 and the placeholder-resolution
checklist in `workflow-instruction-template.md §9`. **Recommendation.** The skill should run the
placeholder grep (`grep -rn '<[a-z_-]*>' docs/release/<bundle>/*.md`) as a pre-publish gate.
Any match must be a documented schema/receipt placeholder (e.g. `<cycle-id>` in the receipt
template), a documented deferred value (e.g. `0.12.<build_at_launch>` captured at first cycle),
or a resolution failure.

**2.7 Trusting a dispatched cycle's "complete" status without re-deriving.** The integration
cycle that catches the load-bearing defect exists *because* the lane that produced the work was
wrong in a way only an independent verifier catches. **Recommendation.** Even if the dispatch
primitive returns success, the skill must re-derive against the live corpus (Rule 5) and the
audit gates (Rule 4) before accepting the cycle.

**2.8 Treating a written summary as the deliverable.** SD-31 lost four full stalls this way — a
wave finished, a summary got written, and the turn ended without dispatching the next phase. Work
stopped until the operator noticed. **Recommendation.** Your skill's dispatch loop must not treat
"I wrote a summary" as a terminal state while ready, undispatched work remains. Dispatch first;
the summary then describes something that already happened.

**2.9 Writing a better-worded warning for a recurring incident instead of a control.** SD-31
wrote a wrong-base-worktree warning into every dispatch prompt from wave 15 onward; it fired 27
times anyway, because a sentence in a prompt is not a control. **Recommendation.** When your
skill's own incident log (Rule 7) shows the same failure recurring more than a handful of times,
the fix is a command with a nonzero exit code that stops the cycle, not an additional paragraph of
prose asking an agent to be careful.

## 3. Worked example — SD-32 chassis completion (2026-08-22)

The SD-32 chassis fill-out is the cleanest recent example of a contributor skill interacting
correctly with the chassis. The four source documents from the predecessor session were:

- `README.md` (status `draft`, four files total: scope-draft, epic-breakdown, HANDOFF, UNMERGED-BRANCHES)
- `scope-draft.md` (four-gate definition of done)
- `epic-breakdown.md` (5 epics with measured ceilings)
- `artifacts/HANDOFF.md` (5 operator-pattern footguns captured from SD-31's session)

A contributor's skill that read the template and the workflow-instruction template, and then followed
the seven rules above, would have:

- **Rule 1** — read the template, NOT forked a new shape. Emitted the 15 canonical files with the
  bundle-specific fields filled in.
- **Rule 2** — flagged the `0.12.<build_at_launch>` placeholder as a deferred-to-first-cycle
  value with the resolution procedure in `decisions.md §9` and `workflow-instruction.md §11`.
- **Rule 3** — used the orchestrating session for the chassis authoring (planning-doc work) and
  did not attempt any source-code edits in the same session.
- **Rule 4** — ran the dual-audit grep on the chassis-only diff and recorded
  `OK_NO_BUNDLE_TAGS / OK_NO_TOKENS` in the cycle-0 receipt.
- **Rule 5** — captured `PCGEN_ORACLE_SHA` from `scripts/pcgen-oracle-pin.env` at cycle-0 and
  recorded it in the pre-launch state block.
- **Rule 6** — N/A for this cycle (no engine fixture was emitted at chassis time).
- **Rule 7** — **missed, and caught only by a later content-completeness review.** SD-31's
  retrospective (`docs/retro/sd31-retrospective.md`) existed and was real, but the chassis
  fill-out never linked it from SD-32's own `references/README.md`. This is the concrete, real
  instance Rule 7 exists to prevent — a skill following Rule 7 would have added the citation in
  the same chassis-completion cycle instead of needing a follow-up fix.

The result was a planning-ready chassis with 15/15 canonical files, all 21 references resolving,
all placeholders explicitly justified at call site. The contributor's skill produced a chassis
the next cycle's `agent()` dispatch could read cold.

## 4. Worked example — SD-31 cycle 18 (live failure mode)

The most instructive *failed* example from this project's history is a wave-18 cycle that
dispatched six Opus build lanes because the skill omitted `model` on the agent call
(failure mode 2.5). The week's Opus quota burned 97% in three hours. The receiver-side
remediation was: every subsequent cycle's dispatch primitive sets `model` explicitly per the
tiering rule.

A contributor's skill that read this document before dispatching would not have made the
mistake. The lesson: **the recommendation is not just "set model explicitly" — it is "set
model explicitly because failure mode 2.5 is a recorded incident, and the next contributor
who omits it will burn the same quota."** Document the failure mode alongside the
recommendation.

## 5. Cross-references

- `../release/template/template.md` — the chassis template every bundle's `README.md` is
  authored from. Read this first when authoring a new bundle.
- `./workflow-instruction-template.md` — the per-cycle dispatch procedure every bundle's
  `workflow-instruction.md` is authored from. Read this first when writing or auditing a cycle.
- `./no-stub-mvp-doctrine.md` — the wired-integration doctrine-of-record. The dual-audit gate
  enforces this on every cycle.
- `./wired-integration-stubs-registry.md` — the operator-granted stub exceptions. The
  default-and-flag rule (refuse the empty case) lives here.
- `../doctrine-external/identifier-discipline.md` — the forbidden source-identifier patterns.
  The dual-audit gate's first grep enforces this.
- `../doctrine-external/spec-domain-lifecycle.md` — how spec-domain routing works. If your
  skill is routing work into spec domains, this is the rule set.
- `../../AGENTS.md §Retrospective Logging`, `scripts/retro.py`, `docs/retro/schema.json` — the
  event-logging discipline Rule 7 enforces.
- `docs/retro/sd31-retrospective.md` — the worked example Rule 7's write-up format follows.

## 6. What this document is NOT

- **Not a copy of the live skills.** The actual implementation lives in the operator's Hermes
  profile at `~/.hermes/profiles/<profile>/skills/{standard-template-constructs,
  spec-domain-bundle-authoring, workflow-instruction-doctrine, governed-workflow-doctrine}/`. Those
  carry the version_history metadata, the curator-backup snapshots, and the frontmatter
  relationships to other skills. A contributor without profile access should read this
  document; a contributor with profile access should read the live skills directly because they
  are the canonical implementation, not this interface description.
- **A Claude-Code-native rendering of this contract also lives in-repo** at
  `.claude/skills/stc-authoring/SKILL.md` — a project-scoped skill (not a personal one) so any
  Claude Code session working in this repo picks it up automatically. It condenses this
  document's seven rules into an actionable form for generating a new bundle's chassis or auditing
  an existing one for content completeness. Update it alongside this document when the interface
  contract changes; the two should never drift.
- **Not a substitute for reading the templates.** A contributor's skill that reads only this
  document and not `docs/release/template/template.md` will produce bundles that violate the
  chassis shape. The templates are the contract; this document is the meta-commentary.
- **Not a static doc.** It evolves when the interface contract evolves. The `last_reviewed_at`
  field carries the freshness marker; a contributor's skill should treat a doc with a stale
  `last_reviewed_at` as suspect and ask the operator for confirmation before relying on it.
