<!--
  Drop-in snippet for an agent-conduct file (AGENTS.md, CLAUDE.md, CONTRIBUTING.md, or
  equivalent). Paste the section below as-is, then edit the two marked spots:
    1. the path to your logging script
    2. the path to your event schema / field reference
-->

## Retrospective Logging

When you catch an error, hit an incident, defer work, or redo something, emit a one-line event to
the retrospective log via `<!-- EDIT: path to your logging script, e.g. scripts/retro.py -->`. The
log is append-only and survives the run even when the working tree does not — a git diff shows what
changed, not what went wrong along the way or what someone believed before it was corrected.

- **Correction:** `retro correction --subject <who-was-wrong> --claimed <claimed-value> --actual <real-value> --verified-by <command-or-check>`.
  The `--verified-by` field is **required**. An unverified correction is not a finding — it is just
  a second, competing assertion, and the log should not carry those.
- **Incident, deferral, rework:** use the matching event type (`incident`, `deferral`, `rework`).
  Run the tool's own help for that type to see its required fields; don't guess at a shape.
- **Reference:** `<!-- EDIT: path to your event schema / field reference -->`.

**Emit at the moment it happens.** Never batch events for the end of a session, and never
reconstruct them afterward from a summary or a chat transcript. A correction written from memory
after the fact has already lost the exact claimed value, the exact actual value, or the check that
proved it — the three things that make it useful to whoever reads the log later.

**Set an actor identity per agent or role** (an environment variable such as `RETRO_ACTOR`, or
whatever your tool uses) before it runs, so every event in a multi-agent or multi-role run is
attributable to the specific agent or role that produced it, not to a shared, anonymous default.

**Why this exists:** a log kept honest in the moment is the only record that survives disagreement
about what happened. If corrections, incidents, and deferrals are only ever described in prose after
the fact, the story tends to drift toward whoever tells it last. A structured, append-only,
verified-at-write-time log is what settles that argument — and it only works if every agent writes
to it as events happen, not as a closing summary.
