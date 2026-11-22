# Retrospective + Release-Package Toolkit

A portable set of tools and conventions for running **agent-driven engineering work** that stays
honest about what actually got done.

Two problems it solves, both of which get worse the more agents you run in parallel:

1. **Git records what landed. It records nothing about what nearly landed wrong.** Not the four
   times in one session a stated number was wrong and a teammate caught it, not what was
   consciously skipped, not what had to be redone. That pattern is far more useful for improving
   how you work than any individual fix — and it is gone the moment the session ends. The
   **retrospective event log** captures it as it happens.

2. **A dispatched agent's own report is not evidence.** An agent that says `complete` may have
   done a tenth of its population. A closure gate written as *"every criterion complete **or**
   filed with a named owner"* will let a project close over open work while every component
   behaves exactly as written. The **release-package chassis** is a set of documents and gates
   that make completion claims mechanically checkable.

Nothing here is tied to a domain. It works the same whether you are building a bicycle-parts
inventory API, a lawn-ornament storefront, or a compiler.

---

## What is in here

```
retro-skill/
├── README.md                    ← you are here
├── ADOPTION.md                  ← how to roll this out, in stages, and what to skip
├── tools/
│   ├── retro.py                 ← the event-log CLI. The only executable in the toolkit.
│   └── retro-schema.json        ← the event contract. retro.py builds its CLI from this,
│                                   so the tool and the contract cannot drift apart.
├── conduct/
│   ├── AGENTS-retro-section.md  ← drop-in snippet for your agent-conduct file
│   ├── blocker-doctrine.md      ← a blocker is cleared or escalated. There is no third option.
│   └── deferral-doctrine.md     ← the sibling rule: a *planned* deferral needs a checked condition
├── templates/
│   ├── release-package-template.md      ← the document chassis for one unit of delivery
│   └── workflow-instruction-template.md ← the per-cycle dispatch procedure
├── skills/
│   └── release-package-authoring/
│       └── SKILL.md             ← a Claude Code skill that authors and audits packages
└── examples/                    ← a small worked example
```

---

## What each piece is for

### `tools/retro.py` — the event log

A CLI that appends one-line events to an append-only log as things happen.

```bash
export RETRO_ACTOR="catalogue-import"          # who is emitting; set per agent or role

python3 retro.py correction \
  --subject "the parts-count figure in the import plan" \
  --claimed "4,200 SKUs" \
  --actual  "4,187 SKUs" \
  --verified-by "psql -c 'select count(*) from parts' → 4187"

python3 retro.py incident  ...
python3 retro.py deferral  ...
python3 retro.py summary --since 2026-01-01 --json
```

Eight event types: `correction`, `incident`, `near_miss`, `deferral`, `rework`, `verification`,
`note`, `resolution`.

**Five design choices that matter more than the code:**

| Choice | Why |
|---|---|
| One log file **per actor**, not one shared file | Many agents appending to one file conflict on nearly every merge, and "take both sides" is not a resolution a human should be asked to perform forty times. |
| `--verified-by` is **required** on a correction | Without the command that proves it, a correction is a competing assertion, not a finding. This is the single highest-value rule in the toolkit. |
| `RETRO_ACTOR` per agent, with a warning when it looks like a directory name | Otherwise the by-actor breakdown at the end resolves to opaque paths and tells you nothing. |
| Append-only; a correction to an event is a **new** event carrying `corrects: <id>` | Lines are never edited or deleted. The log is evidence, not a working document. |
| The schema names what **not** to log | No commits (git has them), no current status (that becomes a second, stale dashboard), no opinions without a check. |

**Emit at the moment it happens.** "I'll batch these at the end of the cycle" is how events
silently never get written.

### `conduct/` — the rules agents follow

`AGENTS-retro-section.md` is a snippet you paste into whatever file your agents already read
(`AGENTS.md`, `CLAUDE.md`, `CONTRIBUTING.md`). It is the *source* of the logging discipline;
everything else just echoes it.

`blocker-doctrine.md` and `deferral-doctrine.md` are the two rules that keep a board honest:

- A **blocker** on your Definition of Done is **cleared** (decompose it and run the cycles — a
  large blocker is a sequencing problem, not an exemption) or **escalated** to the human who owns
  the work. Never deferred, never handed to a successor project on a cycle's own authority.
- A **planned deferral** is legitimate, but it must name a **revisit condition that is checked,
  not remembered**, the checker that detects it, and the accepted cost.
- The test between them: *was this scope in the Definition of Done when the work launched?*
  If yes, it is a blocker.

### `templates/` — the document chassis

`release-package-template.md` lists the documents that make up one unit of delivery and what each
is for. `workflow-instruction-template.md` is the per-cycle procedure: how a cycle is dispatched,
what gates run on its diff, what its receipt must contain, and the order closure steps happen in.

The parts worth stealing even if you adopt nothing else:

- **The orchestrator never executes.** The session that plans and writes planning documents does
  not edit shipping code — that happens inside a dispatched agent. Discovering the real scope
  mid-investigation is a reason to re-dispatch, not to fix it inline.
- **Every dispatched agent sets its model explicitly.** An omitted model silently inherits the
  orchestrator's, which can be dramatically more expensive.
- **A dispatched agent gets exactly one turn.** Nothing wakes it. Wait for slow work inside the
  turn; commit before ending it.
- **A cycle's status is a row count on its own artifact**, never a self-assessment.
- **Three statuses, not two:** `complete`, `partial` (closed part, and named every remaining item
  by sub-cause with populations that sum exactly — the dispatch continues), and
  `blocked-escalated` (needs a human ruling — this pauses everything). *Needing more cycles is
  `partial`, never `blocked-escalated`.* A vocabulary with only two values forces honest partial
  work to pick the word that halts the project — and tempts a cycle to claim `complete` instead.
- **Every figure states its denominator in the same construct.** "8,463 units" is a number;
  "8,463 of 49,438 units" is a fact. The most expensive errors are the ones where the figure is
  correct and the population is wrong.

### `skills/release-package-authoring/SKILL.md`

A Claude Code skill that generates a new package's chassis from the templates and audits an
existing one. Copy the folder into `.claude/skills/` in your repo. Useful without Claude Code too
— it is a readable checklist.

---

## Getting started

The fastest useful thing is the event log alone. It takes about ten minutes and depends on
nothing else here:

```bash
mkdir -p tools docs/retro/events
cp path/to/retro-skill/tools/retro.py tools/
cp path/to/retro-skill/tools/retro-schema.json tools/
# edit the DEFAULT_EVENTS_DIR constant at the top of retro.py if you want a different location
python3 tools/retro.py --help
```

Then paste `conduct/AGENTS-retro-section.md` into your agent-conduct file and set `RETRO_ACTOR`
in each agent's environment. That is the whole minimum installation.

`ADOPTION.md` covers the staged rollout, what to adopt in what order, what you can skip, and the
mistakes that cost the most.

---

## The one thing to take away

Every rule here exists because something specific went wrong and a better-worded warning did not
prevent it happening again. **When the same failure recurs more than a handful of times, the fix
is a command with a non-zero exit code, not another sentence of prose.**

That is the whole philosophy. The tools are just the parts where it has already been done.
