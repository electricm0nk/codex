# GitHub Enhancement Request Intake Contract

## Purpose
Define the structured enhancement-request issue contract for SD-11 so later implementation captures blocked workflows and requested capabilities cleanly.

## Required issue shape
- destination: GitHub Issues in the authoritative Codex repo
- issue type: `enhancement`
- minimum labels:
  - `enhancement`
  - channel/build tier label when available
  - platform label when available
  - affected-surface label when determinable

## Required fields
| Field | Source | Required | Notes |
|---|---|---:|---|
| issue title | user-supplied with app assistance | yes | must summarize the missing capability or blocked workflow |
| build label/version | auto-captured | yes | include the current app build context |
| tester-facing channel/support label | auto-captured | yes | include visible channel/support posture |
| platform | auto-captured | yes | include OS and, later, package/install context when available |
| current bounded workflow | auto-captured | yes | identifies the workflow where the request arose |
| tester goal | user-supplied | yes | what the tester was trying to accomplish |
| current friction or limitation | user-supplied | yes | what prevented or slowed that goal |
| requested capability or improvement | user-supplied | yes | what should exist or work differently |
| affected surface | mixed | yes | workbench area, workflow slice, or update/support surface |
| supporting evidence/examples | mixed | no | screenshots, sample data, or diagnostic context if helpful |

## Submission rules
- enhancement flow must remain separate from the bug flow even if they share transport and screen components
- enhancement requests must remain tied to real blocked workflows or missing capabilities, not generic brainstorming
- if submission fails, the app must preserve a copyable or locally saved structured payload instead of discarding the request

## Minimum markdown body structure
1. Summary
2. Current build/channel/platform/workflow
3. Tester goal
4. Current friction or limitation
5. Requested capability or improvement
6. Affected surface
7. Supporting evidence / examples

## Explicit refusals
- no vague “idea box” without workflow context
- no enhancement issue that omits build/channel/platform/workflow metadata when it was available
