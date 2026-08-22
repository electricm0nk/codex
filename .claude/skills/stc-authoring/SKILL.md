---
name: stc-authoring
description: Author, fill out, or audit an SD-N release package (an "STC" chassis) under docs/release/. Use when creating a new SD-N bundle, filling in its chassis files from predecessor docs, reviewing an existing package for content completeness, evaluating launch readiness, or checking a workflow-instruction.md / kanban.md dispatch cycle for protocol violations. Also use when the user asks to "evaluate loop-instruction.md" or "workflow-instruction.md for launch readiness," or to review a release package for gaps.
---

# STC Authoring

This project's release packages (`docs/release/SD-NN-<slug>/`) follow a fixed chassis. This
skill is the harness-side executor of the interface contract at
`docs/governance/STC-Skill-Creation.md` — read that file once for the full reasoning; this skill
is the condensed, actionable form.

**Two canonical templates.** Every bundle is authored from these, never forked:

- `docs/release/template/template.md` — the chassis shape: which files exist, what each is for,
  the bundle-snapshot table, the closure obligation.
- `docs/governance/workflow-instruction-template.md` — the per-cycle dispatch procedure every
  bundle's own `workflow-instruction.md` is authored from. (Bundles launched before 2026-08-22
  name this file `loop-instruction.md` — do not rename an existing bundle's file; this is a
  naming convention for new bundles only.)

## The 13 required canonical files

`docs/release/template/template.md §7` lists these; a bundle is not complete without all of
them:

```
README.md, scope-draft.md, workflow-instruction.md, progress.md,
epic-breakdown.md, decisions.md, risks-and-open-questions.md,
acceptance-and-verification.md, content-unit-inventory.md,
artifacts/, artifacts/README.md, references/, references/README.md
```

`technical-requirements.md` and `technical-design.md` are also standard in practice even though
the template's §7 list omits them — check the sibling packages under `docs/release/SD-*/` for
the current convention before treating a package missing them as complete.

## When generating a new bundle's chassis

1. Read both templates in full before writing anything.
2. Read every predecessor document the operator hands you (README/scope-draft/epic-breakdown/
   HANDOFF-style artifacts from the prior bundle's closing session). The chassis fill-out is a
   **transcription-and-organization task, not a generation task** — the content comes from what
   the predecessor session already produced, not from a one-sentence prompt.
3. Emit the files verbatim from the templates, replacing only bundle-specific fields (slug,
   branch, build version, owner). A `<...>`-style placeholder is for a human to resolve, never
   for you to invent content for.
4. Flag every remaining placeholder explicitly, and name where it resolves (e.g.
   `0.12.<build_at_launch>` — resolved at first cycle per `decisions.md §9` and
   `workflow-instruction.md §11`). Do not ship an unflagged placeholder.
5. Run the placeholder gate before calling it done:
   ```bash
   grep -rn '<[a-z_-]*>' docs/release/SD-NN-<slug>/*.md
   ```
   Every match must be a documented schema/receipt placeholder, a documented deferred value with
   a named resolution point, or a bug to fix.

## When auditing an existing package for content completeness

This is what "review for content completeness" or "evaluate for launch readiness" means in
practice:

1. `find docs/release/SD-NN-<slug> -type f | sort` — diff against the 13-file list above.
2. Read every file. A file that exists but is a bare template skeleton with no real content is
   the same defect as a missing file — check status frontmatter (`status: planning-ready` vs
   `draft`) and look for template markers the placeholder gate would catch.
3. Cross-check content against the package's own source documents (README, scope-draft,
   epic-breakdown, and any HANDOFF/artifacts files) — a fact stated in one file and never
   propagated to `kanban.md`, `references/README.md`, or `forward-scope-register.md` is a gap
   even when every file individually looks complete. Concretely: does every named piece of
   follow-up work (an "orphaned branch," a named lesson, a retrospective) have an actual queue
   entry or citation, not just a mention in prose?
4. Check whether a sibling package's retrospective (`docs/retro/*.md`) or predecessor-package
   lessons doc exists and is cited from this package's `references/README.md`. It is easy for a
   chassis fill-out to carry the *content* of a predecessor's lessons without ever linking the
   *source document* — grep for it:
   ```bash
   grep -rn "retro" docs/release/SD-NN-<slug>/references/README.md
   ```
5. Sanity-check any verification command that reads a file as structured data (`json.load`,
   `jq`, etc.) — confirm the target path is actually the right format. A command that tries to
   parse a markdown file as JSON is a real, verifiable defect, not a style nit.
6. Report gaps concretely (file + line + what's missing), then fix them as small, targeted edits
   — this is planning-doc authoring, not shipping-code work, so it's fine for the orchestrating
   session to make these edits directly (see the boundary rule below).

## The orchestrator/executor boundary — the rule most often violated

The session that plans, scopes, or authors/audits a bundle's **planning docs**
(`workflow-instruction.md`, `epic-breakdown.md`, `decisions.md`, `kanban.md`, `progress.md`,
`README.md`, etc.) may edit those files directly. It must **never** edit shipping code
(`apps/desktop/`, `apps/desktop/src-tauri/`, `src/`, `scripts/`) directly, even for a one-line
fix, even mid-investigation, even when a Plan-Mode approval already authorized the underlying
change. That work happens inside a dispatched `agent()` / `Workflow` call — see
`workflow-instruction.md §2.2` for the full self-check. Discovering the real scope of a fix while
investigating is a reason to dispatch (or re-dispatch) an agent call with the corrected scope,
never a license to make the fix inline because the context is already loaded.

## The dual-audit gate (every dispatched cycle)

Two greps run on every cycle's diff before it's allowed to commit:

```bash
BASE_BRANCH=$(git merge-base HEAD origin/develop)

# Identifier audit — no trailing \b (it doesn't match _ -> word-char boundaries;
# adding it back silently stops catching real identifiers like sd19_class_catalog)
git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <scoped paths> ':!**/__tests__/**' ':!**/*.test.*' \
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'

# Wired-integration audit
git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <scoped paths> ':!**/__tests__/**' ':!**/*.test.*' \
  | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
```

A single-token violation is self-healable inline (fix, re-audit, continue). A real stub in
shipping code is **non-self-healable** — exit FAIL with a `## Open blockers` entry in
`progress.md`, don't silence the gate, don't skip it on a cycle that "looks clean," don't treat
five green cycles as license to skip the sixth.

## Failure modes to actively guard against

Each of these has a real, recorded incident in this project's history (full detail in
`docs/governance/STC-Skill-Creation.md §2`):

- **Counterfeit completion.** A closed cycle whose diff is empty or trivially green. Re-derive
  the claimed outcome against the live corpus and the audit gates before accepting it — don't
  transcribe a dispatched agent's self-report.
- **Boundary bypass.** See above. Single most-violated rule in this project's history.
- **`git stash` from a subdirectory.** Never — the bare form stashes the whole shared checkout
  even when called from inside `docs/release/SD-NN/`. Use `git show HEAD:<file> > /tmp/<file>` to
  read a baseline instead.
- **`find -newermt` for freshness.** Lies on this box (agent-file mtimes run ahead of system
  time). Use a Python mtime comparison or `git status --porcelain`.
- **Omitting `model` on a dispatched agent call.** Inherits the orchestrator's model, not Sonnet.
  A real incident burned 97% of weekly Opus quota this way. Set it explicitly every time: Sonnet
  for build/integration, Opus only for adversarial verification, Haiku for housekeeping.
- **Shipping `<...>` placeholders.** Run the placeholder gate (above) before calling any bundle
  planning-ready.
- **Trusting a "complete" cycle status without re-deriving.** The whole point of an integration
  cycle is to catch what the lane that produced the work got wrong.

## Fixture and corpus-SHA discipline (when a cycle emits engine values)

- Every re-derived figure quoted in a receipt names the command that produced it and, if it came
  from the pinned PCGen corpus, the `PCGEN_ORACLE_SHA` from `scripts/pcgen-oracle-pin.env`. A
  figure without a stated corpus commit is not re-derived — it's a number that may have drifted.
- A fixture's expected value must be transcribed from bytes the engine's own read path does
  **not** touch. A fixture built from the same file the engine reads is a mirror, not a check —
  it will happily validate a fabricated value.

## Cross-references

- `docs/governance/STC-Skill-Creation.md` — the full interface contract this skill condenses.
- `docs/release/template/template.md` — the chassis template.
- `docs/governance/workflow-instruction-template.md` — the per-cycle dispatch procedure template.
- `docs/governance/no-stub-mvp-doctrine.md`, `docs/doctrine-external/identifier-discipline.md` —
  doctrine-of-record for the dual-audit gate.
- `docs/retro/` — retrospectives. Check whether the package under audit cites the relevant one.
