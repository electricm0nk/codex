# Shipping-code doctrine

Two rules about what may reach your main branch, and the two greps that enforce them. Together
they are the **dual-audit gate** that `../templates/workflow-instruction-template.md` §6 runs on
every cycle's diff.

Both exist because the failure they prevent is invisible in review: the code compiles, the tests
pass, and the defect is a thing that *isn't there*.

---

## Rule 1 — no stubs in shipping code

**Code that ships does what it claims to do.**

Not allowed on a path a user can reach:

- a function that returns a canned value instead of computing one
- an inline mock, fixture, or hard-coded sample standing in for real data
- an event handler wired to a visible control that does nothing
- a string like `"Would send the email"` or `"TODO: actually charge the card"` where the real
  action belongs
- a branch that silently succeeds when it cannot do the work

**Why it needs a rule rather than judgement:** a stub is indistinguishable from a working feature
in every artifact a reviewer normally looks at. The board says done, the tests are green, the
demo works because the demo hits the stubbed path. It is discovered by a user, usually late.

**The honest alternatives**, in preference order:

1. Implement it.
2. Make it fail loudly — a raised error naming what is missing is safe; a silent success is not.
3. Do not ship the affordance. A button that is absent is better than a button that lies.

If you genuinely must ship a placeholder — a vendor integration that does not exist yet, say —
it goes in a written exceptions list with an owner and a removal condition, and the control it
sits behind is disabled and labelled. An exception you can enumerate is manageable. An exception
nobody wrote down is a bug with a good disguise.

### The gate

```bash
BASE=$(git merge-base HEAD origin/main)

git diff --unified=0 "${BASE}...HEAD" -- <your source paths> \
    ':!**/__tests__/**' ':!**/*.test.*' \
  | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' \
  || echo 'OK_NO_TOKENS'
```

Adjust the token list to your codebase's own markers. Exclude test paths — a mock in a test is
the point.

A single stray token in a comment is fixable inline: remove it, re-run, carry on. **A real stub on
a shipping path is not self-healable** — stop, and escalate it as a blocker
(`blocker-doctrine.md`). Do not silence the gate, and do not skip it on a cycle that "looks
clean". Five green runs are not a licence to skip the sixth.

---

## Rule 2 — no process identifiers in shipping code

**Your work-tracking vocabulary must not leak into the product.**

If your bundles are called `PROJ-14`, then `proj14_parts_table`, `// PROJ-14: added here`, and
`class Proj14Importer` are all defects. So are ticket numbers, sprint names, and internal
run identifiers in symbol names.

**Why it matters more than it looks:**

- The identifier outlives the process. Two reorganisations later, `proj14_` means nothing to
  anyone, and it is now load-bearing in a name nobody dares change.
- It encodes *when* something was built into *what it is*. A reader learns which sprint added the
  table and nothing about what the table holds.
- It spreads. One `proj14_` field becomes a `proj14_` accessor, then a `proj14_` migration, then
  a public API field.

Name things for what they are. Process context belongs in the commit message, the receipt, and
the event log — all of which are searchable and none of which ship.

### The gate

```bash
git diff --unified=0 "${BASE}...HEAD" -- <your source paths> \
    ':!**/__tests__/**' ':!**/*.test.*' \
  | grep -nE '\b(proj[0-9]+_|PROJ[0-9]+_|Proj[0-9]+)' \
  || echo 'OK_NO_BUNDLE_TAGS'
```

Replace `proj` with your own prefix.

**Do not add a trailing `\b` to that pattern.** `\b` matches a word boundary, and there is no
boundary between `_` and a word character — so `proj14_parts_table`, exactly the case you most
want to catch, would stop matching. This looks like a harmless tidy-up and silently disables the
check.

---

## Where these run

`../templates/workflow-instruction-template.md` §6 step 2 runs both on every cycle's diff, before
the cycle is allowed to commit. Running them once at the end of a project finds the same problems
at the point they are most expensive to fix.
