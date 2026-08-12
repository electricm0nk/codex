# Tester Feedback Evidence Capture Matrix

## Purpose
Record the exact evidence fields SD-11 expects for tester bug and enhancement submissions, including how each field is captured and what redaction rules apply.

## Matrix
| Field | Capture mode | Required | Applies to | Notes |
|---|---|---:|---|---|
| build label/version | auto-captured | yes | bug + enhancement | must come from the running app/build surface |
| tester-facing channel/support label | auto-captured | yes | bug + enhancement | operator branch lineage may be stored internally, but the user-facing label is what the tester sees |
| platform / OS | auto-captured | yes | bug + enhancement | include package/install context later when available |
| current bounded workflow | auto-captured | yes | bug + enhancement | identifies the active tester flow |
| current data-source identity | auto-captured | yes | bug + enhancement | distinguish real command data from placeholder/fallback state |
| observed behavior | user-supplied | yes | bug | what happened |
| expected behavior | user-supplied | yes | bug | what should have happened |
| reproduction steps | user-supplied | yes | bug | explicit steps when reproducible |
| tester goal | user-supplied | yes | enhancement | what the tester was trying to accomplish |
| current friction / limitation | user-supplied | yes | enhancement | what blocked or slowed the goal |
| requested capability / improvement | user-supplied | yes | enhancement | desired change |
| diagnostics / blocked claims | auto-captured when present | required when present | bug; optional for enhancement | preserve severity/class/reason fields when the UI has them |
| explanation / provenance refs | auto-captured when present | optional | bug + enhancement | useful when available; do not fabricate |
| screenshot / attachment refs | mixed | optional | bug + enhancement | allowed only if the app later supports explicit attachment handling |
| redaction declaration | user-supplied or auto | required when attachments/logs are included | bug + enhancement | must state that sensitive content was scrubbed or omitted |

## Redaction rules
- do not attach raw logs, save files, or screenshots silently
- if an attachment can contain user-sensitive data, the flow must either redact it or require explicit user confirmation before inclusion
- when redaction occurs, the payload must say that it occurred so triage understands missing context intentionally

## Failure-handling rule
If the app cannot submit or attach everything immediately, it must preserve the structured matrix values in a local draft or copyable payload rather than dropping evidence.
