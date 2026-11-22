# Adoption notes

How to roll this out without swallowing all of it at once, and which parts are worth the trouble.

Read `README.md` first for what each file is. This document is about **order, cost, and the
mistakes that hurt most.**

---

## Adopt in four stages

Each stage is independently useful. Stop at any of them.

### Stage 1 — the event log (about 10 minutes)

The highest value-per-effort item in the toolkit, and it depends on nothing else.

```bash
mkdir -p tools docs/retro/events
cp retro-skill/tools/retro.py retro-skill/tools/retro-schema.json tools/
python3 tools/retro.py --help
```

Edit the `DEFAULT_EVENTS_DIR` constant at the top of `retro.py` if you want the log somewhere
else. Paste `conduct/AGENTS-retro-section.md` into whatever file your agents already read, and
export `RETRO_ACTOR` per agent or role.

Add `docs/retro/events/*.jsonl` to your repository — the log is evidence, so it belongs in
version control, not in a scratch directory.

**You will know it is working** when you run `summary` after a week and see a pattern you did not
know about. Typically: one category of mistake accounts for most corrections.

### Stage 2 — the two doctrines (about an hour of discussion)

`conduct/blocker-doctrine.md` and `conduct/deferral-doctrine.md`. These are decisions about how
your team behaves, not code, so the hour is spent agreeing rather than installing.

The concrete action: **grep your existing definitions of done for the phrase shape "complete
**or** …"**. Any acceptance criterion that accepts a filed blocker as satisfying it will
eventually let work close over an open item, with every component behaving exactly as written.

### Stage 3 — the receipt and the gates (a day)

From `templates/workflow-instruction-template.md`, take §6 (per-cycle procedure) and §7 (receipt
schema) only. Ignore the rest for now.

The receipt is the load-bearing piece. Require every cycle to report:

- every figure **with the command that produced it and its denominator**
- the literal output of a row count run on the cycle's own artifact
- what build/test scope was verified, and **the commit it ran at**
- movement stated in separate buckets — closure vs reclassification vs measurement-correction

That last one prevents the most common self-deception: a count that dropped because you changed
how you measured is not progress.

### Stage 4 — the full package chassis (a week, and only if you need it)

`templates/release-package-template.md` plus the skill. This is worth it when you have multiple
agents working a scoped body of work over days, with a real closure event at the end. It is
overkill for a single-session task.

---

## What to change for your project

| Thing | Where | Note |
|---|---|---|
| Event log location | `DEFAULT_EVENTS_DIR` in `retro.py` | The one constant most adopters edit. |
| Bundle naming | your own choice | Pick a short prefix and a number. Whatever you choose ends up in identifiers, so keep it short and never let it leak into shipping code. |
| The unit you count | your inventory document | Parts, SKUs, endpoints, tables, screens. The chassis does not care, but **pick one and stick to it** — a document that silently switches denominators is worse than one with no numbers. |
| Build/test commands | the workflow-instruction §6 and §1 | Replace with your own widest-scope build. |
| The dual-audit greps | workflow-instruction §6 | Keep the shape; change the token list to your own stub markers. |

---

## What you can skip

- **The parallel/sequential map (§3)** if you run one agent at a time. It exists to stop two
  agents writing the same file.
- **The worktree/branch sweep** if you are not creating per-agent worktrees.
- **The `partial` status** if your cycles are small enough to always finish. Add it the first time
  an honest cycle has to lie to keep the pipeline moving.
- **Everything in stage 4** unless you have a real closure event. A chassis with no closure is
  paperwork.

Do **not** skip `--verified-by` on corrections. It is the rule that makes the log trustworthy, and
it is the one people most want to relax when they are in a hurry.

---

## Mistakes that cost the most

These are ordered by how much they cost, not how likely they are.

**1. Treating an agent's self-report as evidence.**
An agent reported `complete` having written rows for 103 of its own 494-item population. Nothing
in its prose was false; the status field was a self-assessment. Only counting rows in its output
file caught it. **Derive status mechanically from the artifact, and have the verifying step derive
the missing SET, not just the count** — a count can match while membership does not.

**2. A closure gate that accepts a filed blocker.**
Covered above. This one closes a project over open work while every part behaves correctly, which
is why it survives review.

**3. Carrying a figure forward instead of re-deriving it.**
A number inherited from an earlier document is a recollection, not a measurement. It is
particularly dangerous when a later step regenerates the data the number came from — the figure
stays plausible and becomes wrong. **Re-derive at the commit you are actually working at.**

**4. Running the verification before the last write that can move it.**
A cycle ran its test suite, reported green truthfully, and a later commit in the same cycle
regenerated the data an assertion depended on. The green report was accurate and the tree was red
nine minutes later. **Run the widest verification after the last change that can move a figure,
and name the commit it ran at in the receipt.**

**5. A gate whose examined population does not grow when you add data.**
A sweep reporting "0 findings" over a population it never walked is indistinguishable from a
sweep that passed honestly. **Require every gate to print the population it examined**, and when a
step adds records, require the examined count to move by exactly the number added.

**6. Omitting the model on a dispatched agent call.**
It silently inherits the orchestrator's model. If your orchestrator runs on an expensive tier,
every worker does too. Set it explicitly on every call.

**7. Writing a better-worded warning instead of a control.**
A warning about a recurring mistake was added to every dispatch prompt and the mistake still
happened twenty-seven more times. **After the same failure recurs a handful of times, the fix is a
command with a non-zero exit code.** Build the check or accept the failure; a third paragraph of
prose is neither.

**8. Letting a status vocabulary force a lie.**
If the only statuses are `complete` and `blocked`, a cycle that honestly did half its work and
named the rest must pick one. Picking `blocked` halts everything for no reason; picking `complete`
is a false claim. Both are bad, and the vocabulary caused it.

---

## Two things that will feel wrong at first

**The log will feel like overhead for about a week.** Emitting an event when you catch your own
mistake is mildly unpleasant, and the value is invisible until the first summary. It becomes
worthwhile at the point where you can answer "what keeps going wrong?" with a count instead of an
impression.

**Recording a correction against yourself is the point, not a failure.** The most useful events in
any log are the ones where someone caught their own error before it shipped. A log with no
self-corrections is not a log of a team that makes no mistakes — it is a log nobody is honest in.

---

## A note on what this does not do

It does not track status. Deliberately.

Point-in-time state belongs in something you regenerate wholesale from source — an inventory, a
dashboard, a query. An event log that also carries status becomes a second dashboard, and a stale
dashboard is worse than none. The log answers *what happened*; something else answers *where are
we now*.
