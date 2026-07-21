# SD-24 — Missing skills (tracked gap, non-blocking)

`loop-instruction.md` §1 item 6 names two skills as loop preconditions: `wired-integration-discipline` and `identifier-discipline`. Neither exists as an installed, invocable skill as of 2026-07-21. Checked:

- `hermes skills` registry — no matching install.
- `~/.claude/skills/` (global) — contains `codebase-design`, `diagnosing-bugs`, `grill-me`, `grilling`, `improve-codebase-architecture`, `tdd`. Neither doctrine skill present.
- `docs/release/SD-24-beta-readiness-and-multiclass/` repo-local `.claude/skills/` — not present.

`governance/identifier-discipline.md`, cross-referenced by `loop-instruction.md §6` as the "REPO-LOCAL CANONICAL identifier-discipline sibling," also does not exist. Only `governance/no-stub-mvp-doctrine.md` and `governance/wired-integration-stubs-registry.md` are present.

**Why this doesn't block SD-24's launch:** the actual audit content both skills would provide is already embedded as literal, runnable grep commands in `loop-instruction.md §2.3` step 4 (the dual-audit gate) and is exercised every cycle regardless of whether either skill is installed. The doctrine itself (`governance/no-stub-mvp-doctrine.md`) is present and governs the audit's forbidden-token list.

**What's actually missing:** a packaged, reusable `identifier-discipline` skill (equivalent doctrine doc `governance/identifier-discipline.md` doesn't exist either — only its cross-reference does) and a packaged `wired-integration-discipline` skill wrapping `governance/no-stub-mvp-doctrine.md`'s four-check audit for reuse outside this one embedded script.

**Follow-on action (operator, outside this bundle's run):** author `governance/identifier-discipline.md` as the missing doctrine doc, then package both as installable skills so future bundles can reference `hermes skills` instead of re-embedding the grep commands per bundle.
