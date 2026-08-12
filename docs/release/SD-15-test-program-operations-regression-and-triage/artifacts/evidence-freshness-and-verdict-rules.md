# SD-15 Evidence Freshness and Verdict Rules

## Purpose
Define how SD-15 receipts are judged for freshness, sufficiency, and proof value so operators cannot mistake old evidence, thin evidence, or a failed rerun for proof that a defect is fixed or tranche closure is safe.

## Governing principle
The decisive rule is simple: absence of reproduction is not proof of resolution.

`not-reproduced` means only that a named rerun under named conditions did not reproduce the claim. It does not mean `fixed`, `closed`, `unsupported`, or `safe to update status surfaces` unless later evidence establishes those stronger conclusions explicitly.

## Evaluation model
Judge every receipt on three independent axes:
1. freshness — is the evidence still current for the claim being made now?
2. sufficiency — is enough of the receipt populated to support a real conclusion?
3. verdict — what does the evidence actually prove about the bounded path?

A receipt is honest only when all three axes are stated explicitly.

## 1. Freshness vocabulary
| State | Meaning | Typical trigger | What it allows | What it forbids |
|---|---|---|---|---|
| `current` | the receipt is aligned with the specific build/channel/support/workflow/environment claim now under review, and no newer contradictory evidence has displaced it | same bounded path, same or explicitly targeted build identity, same adjacent-authority baseline, no known superseding contradiction | bounded triage, bounded rerun planning, later reconciliation input | claiming broader closure than the receipt actually covers |
| `stale` | the receipt still preserves history, but it no longer matches the current truth target closely enough to support present-tense claims | build changed, channel/support meaning changed, platform/install path changed, adjacent SD-13 or SD-14 truth changed, or a newer receipt supersedes it | historical context, comparison against newer evidence | calling the current build fixed, broken, aligned, or closure-ready from this receipt alone |

## Freshness degradation triggers
Mark a receipt `stale` when any of the following is true:
- the build label/version, commit/build identity, or operator provenance handle no longer matches the claim target
- the tester-facing channel/support label has changed or was later shown to be wrong
- the platform, package/install path, or bounded workflow under test changed materially
- SD-13 support-state truth changed and the old receipt no longer represents the current supported/partial/unsupported boundary
- SD-14 persistence or migration truth changed and the old continuity claim is no longer the current contract
- a later receipt for the same path explicitly supersedes the older one
- the only evidence comes from authoring-machine behavior, but the downstream claim now requires clean-machine or external-tester proof

## 2. Sufficiency vocabulary
| State | Meaning | Minimum threshold | What it allows | What it forbids |
|---|---|---|---|---|
| `complete` | all required receipt fields are populated, or explicit absence markers are present where the schema permits them | build/channel/platform/workflow identity, observed and expected behavior, reproduction status, and enough provenance to reconstruct the claim later | strong bounded conclusions about the named path | broader claims outside the named path |
| `partial` | enough evidence exists to classify or route the report, but one or more required contextual fields are still missing for stronger downstream use | at least the bounded path and observed problem are understandable, but provenance, diagnostics, attachments, or adjacent context remain incomplete | triage, escalation, and targeted follow-up requests | closure, status-surface updates, or claims of fix completeness |
| `insufficient` | foundational fields are missing badly enough that the record cannot support a trustworthy verdict | the receipt cannot reliably say what build/path/expectation was under test, or what actually happened | hold as anecdotal or blocked evidence only | defect counts, closure use, fix claims, unsupported claims, or status reconciliation |

## Sufficiency downgrade rules
A receipt is at least `partial` only if all of the following are explicit:
- some bounded workflow or mission under test
- some concrete observed behavior
- some concrete expected behavior or expected bound
- some concrete build/channel/platform identity

A receipt is `insufficient` if any of those foundations are missing or only guessed.

A receipt may remain `partial` rather than `insufficient` when the core failure is visible but one or more of the following are still missing:
- operator provenance handle or immutable build identity
- clean-machine or external-environment identity
- attachments or diagnostics that would strengthen but are not strictly required to understand the claim
- SD-13 or SD-14 contextual anchors needed for stronger downstream judgment

## 3. Verdict vocabulary
| Verdict | Meaning | What it proves | What it does not prove |
|---|---|---|---|
| `reproduced` | the named bounded path failed again under named conditions | the defect or contradiction is still observable on the named path | root cause, fix plan, or broader product state |
| `reproduced-with-bounds` | the rerun confirmed a failure or unsupported boundary that is already bounded by adjacent authority | the named path remains partial or unsupported in the way the receipt records | that the whole adjacent domain is unsupported |
| `not-reproduced` | the rerun under named conditions did not recreate the original behavior | only that this rerun did not reproduce the claim under those exact conditions | fixed, resolved, closed, safe to promote, or safe to update README/ledger truth |
| `blocked-reproduction` | the operator could not rerun truthfully because a prerequisite, artifact, environment, or evidence handle was missing | the reproduction attempt is blocked and the blocker is part of the evidence | anything about defect absence or resolution |
| `unsupported-confirmed` | adjacent authority plus evidence proves the tester hit a bounded unsupported path | the report should remain visibly unsupported or partial rather than misfiled as a normal bug | that the underlying product behavior is defect-free outside that unsupported bound |
| `status-drift-confirmed` | durable truth surfaces materially contradict one another or a named receipt | the contradiction itself is real and operator-visible | that the runtime behavior is otherwise correct |
| `partial-proof` | the record supports a real suspicion or route, but not a closure-grade conclusion | follow-up work is justified and should be targeted | fix, closure, or status alignment claims |
| `insufficient-proof` | the record is too weak to support a bounded conclusion | more evidence is required before honest routing or counting | any material conclusion |

## Allowed conclusion matrix
| Freshness | Sufficiency | Verdict | Truthful conclusion | Forbidden shortcut |
|---|---|---|---|---|
| `current` | `complete` | `reproduced` | a current bounded defect or contradiction exists on the named path | claiming root cause, overall tranche failure, or universal impact |
| `current` | `complete` | `reproduced-with-bounds` | the named bounded path remains partial or unsupported as documented | relabeling the outcome as an ordinary defect for convenience |
| `current` | `complete` | `not-reproduced` | the named rerun failed to reproduce the claim under the named conditions | calling the issue fixed, closed, or status-safe |
| `current` | `complete` | `unsupported-confirmed` | the report hit a real unsupported boundary governed by adjacent authority | treating the report as proof that the supported path is healthy |
| `current` | `complete` | `status-drift-confirmed` | the contradiction between durable surfaces or receipts is real | downgrading the problem to wording polish if the contradiction changes operator decisions |
| `current` | `partial` | any non-insufficient verdict | the report can be routed or followed up, but stronger claims must wait | closure, README/ledger updates, or fix claims |
| `current` | `insufficient` | `insufficient-proof` | the evidence is too weak for trustworthy adjudication | classifying as fixed, unsupported, or cleanly reproduced |
| `stale` | any | any | the receipt is historical context only until refreshed | using it as present-tense closure or status proof |

## Explicit rule for `not-reproduced`
To be precise:
- `not-reproduced` is a verdict about one rerun attempt, not a health state of the product
- `not-reproduced` may coexist with `current` freshness and `complete` sufficiency, but it still does not authorize `fixed`
- `not-reproduced` becomes operationally useful only when compared against earlier receipts and the exact changed conditions are visible
- if the original claim lacked strong evidence, `not-reproduced` may only downgrade certainty to `partial-proof`; it does not erase the original report

## When a stronger conclusion than `not-reproduced` is allowed
A later lane may claim something stronger only when it has additional evidence beyond the rerun failure itself. Examples:
- a superseding receipt on a changed build or commit with explicit provenance and clean-machine/install/use proof
- an adjacent-authority decision that the original path was actually unsupported, with the supporting SD-13 or SD-14 context made explicit
- a reconciliation decision showing that the relevant durable status surfaces have been updated from current evidence rather than optimistic silence

Until then, keep the verdict as `not-reproduced`, `partial-proof`, or `blocked-reproduction`.

## Downstream-use rules
| Use case | Minimum evidence posture |
|---|---|
| bounded triage routing | `current` + at least `partial` |
| install/use or clean-machine follow-up | `current` + enough provenance to name the exact build/platform/workflow target |
| external-test planning input | `current` + at least `partial`, plus visible unsupported/bound warnings |
| repo/program/ledger status reconciliation input | `current` + `complete`; `stale`, `partial`, or `insufficient` receipts cannot stand alone |
| fix or closure claim | not authorized by this document alone; requires later evidence surfaces and reconciliation |

## Refresh rules
When a receipt becomes `stale`, do not silently edit history into the old record.
Use one of these patterns:
- create a new receipt that references `supersedes_receipt_id`
- update the existing receipt only if the earlier evidence remains clearly preserved and the new freshness/sufficiency/verdict state is explicit
- if the new run targets a different build, platform, workflow, or adjacent-authority baseline, prefer a new superseding receipt rather than mutating the old one into ambiguity

## Explicit refusals
- do not let `not-reproduced` masquerade as `fixed`
- do not let `stale` evidence masquerade as `current`
- do not let `partial` evidence masquerade as closure-grade proof
- do not let `insufficient` anecdote become a defect count, unsupported ruling, or status-surface update
- do not treat clean-machine, external-test, or reconciliation artifacts as optional if the downstream claim depends on them
