# SD-24 — Missing skills (tracked gap, non-blocking)

`loop-instruction.md` §1 item 6 names two skills as loop preconditions: `wired-integration-discipline` and `identifier-discipline`. Neither exists as an installed, invocable skill as of 2026-07-21. Checked:

- `hermes skills` registry — no matching install.
- `~/.claude/skills/` (global) — contains `codebase-design`, `diagnosing-bugs`, `grill-me`, `grilling`, `improve-codebase-architecture`, `tdd`. Neither doctrine skill present.
- `docs/release/SD-24-beta-readiness-and-multiclass/` repo-local `.claude/skills/` — not present.

**Correction (2026-07-21):** an earlier version of this file claimed `governance/identifier-discipline.md` doesn't exist at all. That was wrong — it exists at the **workspace level**, `~/workspace/governance/identifier-discipline.md` (a real, substantive doctrine doc, not a stub), just not under this repo's local `governance/` directory where SD-24's cross-reference pointed. `governance/no-stub-mvp-doctrine.md` (repo-local) is the wired-integration-discipline counterpart. What's still accurate: neither doctrine is packaged as an installed, invocable Claude Code skill anywhere (`hermes skills` registry, global `~/.claude/skills/`, repo-local `.claude/skills/` all checked — no match).

**Why this doesn't block SD-24's launch:** the actual audit content both doctrines specify is already embedded as literal, runnable grep commands in `loop-instruction.md §2.3` step 4 (the dual-audit gate) and is exercised every cycle regardless of whether either doctrine is packaged as an installable skill.

**What's actually missing:** a packaged, reusable `identifier-discipline` skill wrapping `~/workspace/governance/identifier-discipline.md`'s naming rules, and a packaged `wired-integration-discipline` skill wrapping `governance/no-stub-mvp-doctrine.md`'s four-check audit — both for reuse across bundles instead of re-embedding the same grep commands per bundle.

**Follow-on action (operator, outside this bundle's run):** package both existing doctrine docs as installable skills so future bundles can reference `hermes skills` instead of re-embedding the grep commands. See `governance/loop-instruction-template.md` (authored 2026-07-21) for the broader durability fixes this bundle's retrofit surfaced.
