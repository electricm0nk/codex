# GitHub Bug Report Intake Contract

## Purpose
Define the structured bug-report issue contract for SD-11 so later implementation can create actionable GitHub issues instead of vague complaints.

## Required issue shape
- destination: GitHub Issues in the authoritative Codex repo
- issue type: `bug`
- minimum labels:
  - `bug`
  - channel/build tier label when available
  - platform label when available
  - affected-surface label when determinable

## Required fields
| Field | Source | Required | Notes |
|---|---|---:|---|
| issue title | user-supplied with app assistance | yes | must summarize the observable failure, not the guessed cause |
| build label/version | auto-captured | yes | visible in the app and attached to the issue |
| tester-facing channel/support label | auto-captured | yes | may map to operator branch lineage internally |
| platform | auto-captured | yes | include OS and, later, package/install context when available |
| current bounded workflow | auto-captured | yes | e.g. pilot character path, GE-08 authoring workbench, later SD-11 bounded character workflow |
| observed behavior | user-supplied | yes | what happened |
| expected behavior | user-supplied | yes | what the tester believed should happen |
| reproduction steps | user-supplied | yes | step-ordered when reproducible |
| diagnostics/explanation context | auto-captured when present | yes if available | include blocked claims, severity, explanation refs, or visible diagnostic text |
| attachment summary | mixed | no | screenshots/log bundles/save refs if supported |
| redaction declaration | user-supplied/auto | yes when attachments exist | state whether anything was omitted or scrubbed |

## Submission rules
- the app must distinguish a bug report from an enhancement request before composing the payload
- the app must never claim the bug report was filed unless the transport confirms it
- if submission fails, the app must preserve a copyable or locally saved structured payload instead of discarding evidence
- the payload must preserve structured sections even when sent as markdown text

## Minimum markdown body structure
1. Summary
2. Current build/channel/platform/workflow
3. Observed behavior
4. Expected behavior
5. Reproduction steps
6. Diagnostics / explanation context
7. Attachments / redactions

## Explicit refusals
- no free-form-only bug submission path
- no silent best-effort transport that loses evidence
- no issue body that omits build/channel/platform metadata when it was available
