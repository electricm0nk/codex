# Worked example — a bicycle-parts inventory API

A short, complete walkthrough of the toolkit on a small piece of real-shaped work. Nothing here is
hypothetical about the tooling — every command runs.

**The job:** the parts catalogue has ~4,200 SKUs imported from three supplier feeds. Search returns
nothing for about a fifth of them. Fix it.

---

## 1. Before writing code: state the population

The first move is not a fix, it is a denominator.

```bash
$ psql -qtc "select count(*) from parts"
 4187

$ psql -qtc "select count(*) from parts where search_vector is null"
 812
```

**812 of 4,187 parts (19.4%)** have no search vector. Note the shape: *"812 of 4,187"*, not
*"812"*. A bare count invites the reader to supply their own denominator, and they will supply the
wrong one.

The plan document said 4,200. The database says 4,187. That gap is the first event.

```bash
export RETRO_ACTOR="search-backfill"

python3 tools/retro.py correction \
  --subject  "the catalogue size in the import plan" \
  --claimed  "4,200 SKUs" \
  --actual   "4,187 SKUs" \
  --verified-by "psql -qtc 'select count(*) from parts' → 4187"
```

`--verified-by` is required, and this is why: without the command, "actually it's 4,187" is one
person's assertion against another's. With it, it is settled.

---

## 2. During the work: emit as things happen

Partway in, the cause turns out not to be one thing.

```bash
$ psql -qtc "select supplier, count(*) from parts where search_vector is null group by supplier"
 velobits    | 604
 crankworks  | 191
 (none)      |  17
```

Three sub-causes, summing to 812 exactly. That enumeration is what makes the remainder
dispatchable — "the rest" would not be.

One of them is a genuine surprise:

```bash
python3 tools/retro.py incident \
  --summary     "the velobits feed sends part descriptions in a 'notes' field the importer never read" \
  --impact      "604 of 812 unindexed rows indexed empty text; search returned nothing rather than erroring" \
  --detected-by "psql group-by on supplier over parts where search_vector is null" \
  --resolution  "importer now reads notes as a description fallback"
```

Run `python3 tools/retro.py help incident` for the full field list — `--detected-by` is required,
because an incident nobody can say how they found is not reproducible. Every type has its own
required fields; the CLI is generated from the schema, so `help <type>` is always accurate.

And one is deliberately not done now:

```bash
python3 tools/retro.py deferral \
  --what    "re-index the 17 supplier-less legacy rows" \
  --reason  "no supplier feed to re-import from; each needs a human to identify the part" \
  --revisit "the legacy-import backfill lands, which produces the missing supplier column" \
  --scope   "17 of 4,187 parts"
```

Note the deferral names a **condition that gets checked**, not "later". A deferral without a
checker is a wish.

---

## 3. The status question

The fix handles velobits (604) and crankworks (191) — **795 of 812**. The 17 legacy rows cannot be
fixed without data that does not exist yet.

This cycle is **not** `complete`: 17 rows remain. It is **not** `blocked-escalated` either —
nothing here needs the operator to rule on anything; the path forward is known and named.

It is `partial`, with the remainder named:

> `partial` — 795 of 812 backfilled. Remaining: 17 of 812, all supplier-less legacy rows,
> deferred with a checked revisit condition (see the deferral event above). 604 + 191 + 17 = 812.

If the only available statuses had been `complete` and `blocked`, this cycle would have had to
either halt the project for no reason or claim work it did not do. That is why there is a third
one.

---

## 4. Closing: what the log tells you

```bash
$ python3 tools/retro.py summary --since 2026-01-01
```

Over a few weeks this is where the value shows up. A typical first summary produces something
nobody predicted — for example, that most corrections share one shape:

| Corrections by subject | Count |
|---|---:|
| a figure quoted from a planning document rather than re-queried | 9 of 14 |
| a count taken over the wrong table | 3 of 14 |
| other | 2 of 14 |

Nine of fourteen corrections came from trusting a written number instead of re-deriving it. That
is not a fact anyone would have volunteered in a meeting, and git shows none of it.

**The response to that is a control, not a reminder.** A check that fails when a figure appears in
a planning document without a runnable command next to it will stop those nine. A paragraph asking
people to be careful will not.

---

## 5. What a package would add

For a job this size, the event log alone is the right amount of process.

The full release-package chassis starts paying for itself when the work is large enough that
someone has to ask *"is it actually done?"* and the honest answer requires checking rather than
remembering — several agents, several days, a real closure event. See
`../templates/release-package-template.md`.

The rule of thumb: **if no one will ever need to verify the completion claim, you do not need a
chassis to make it verifiable.**
