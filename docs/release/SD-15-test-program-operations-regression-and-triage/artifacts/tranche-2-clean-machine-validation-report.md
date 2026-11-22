# Tranche-2 Clean-Machine Validation Report

## Purpose
Define the run-ready receipt/report surface for proving that a named tranche-2 build was acquired, installed, launched, and exercised on a named clean environment outside the authoring machine.

## Boundary of this surface
- This file is a receipt boundary and execution template, not evidence that any clean-machine run already happened.
- No step may be marked `pass` or `pass-with-known-bounds` without real evidence attached to that exact step.
- Authoring-machine success, local developer builds, or headless build success do not count as clean-machine proof.
- If the environment selection, governed build handle, or acquisition path is still unresolved, record that gap explicitly as `blocked`, `unknown`, `pending-selection`, or `pending-publication-handle` rather than guessing.
- This surface must stay coupled to SD-12 distribution truth, the SD-15 install/use matrix, the SD-15 triage taxonomy, the SD-15 regression receipt schema, and SD-15 evidence-freshness rules.

## Allowed status vocabulary
### Per-step status
- `not-run` — the step has not been executed yet
- `pass` — the step completed and the required evidence is attached
- `pass-with-known-bounds` — the step completed with an explicit bounded limitation that does not counterfeit broader support
- `blocked` — a missing prerequisite, artifact handle, environment fact, or proof surface prevented truthful continuation
- `failed` — the step contradicted the tranche claim materially on the named clean environment

### Final verdict
- `pass` — the named clean-machine path completed with evidence for every required step and no hidden unsupported assumptions
- `pass-with-known-bounds` — the named path completed, but the report preserves an explicit bound grounded by adjacent authority and evidence
- `blocked` — the run could not reach a truthful end-state because prerequisite truth, governed artifact access, or environment readiness was missing
- `failed` — the named clean-machine path materially failed on the governed build under test

## Report status header
Populate every field. When a field cannot yet be grounded, preserve the absence explicitly.

- report state: `completed`
- run handle: `SD15-CM-2026-07-01-001`
- validation date: `2026-07-01`
- validator: `God-Emperor`
- validator role or profile: `god-emporer`
- environment selection state: `selected`
- build selection state: `selected`
- adjacent authority references used:
  - `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md`
  - `programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/tranche-2-install-and-use-matrix.md`
  - `programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/triage-class-dictionary.md`
  - `programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/regression-receipt-schema.md`
  - `programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/evidence-freshness-and-verdict-rules.md`
  - `/home/ubuntu/workspace/repos/codex/README.md`
  - `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md`

## Clean environment identity
These fields define what counts as the named clean machine for this specific receipt.

- environment class: `fresh VM equivalent`
- environment handle: `docker://ubuntu@sha256:786a8b558f7be160c6c8c4a54f9a57274f3b4fb1491cf65146521ae77ff1dc54/sd15-clean-run-3`
- virtualization or host substrate: `disposable Docker container from ubuntu:24.04 on the workspace host`
- operating system: `Ubuntu`
- OS version / build: `24.04.4 LTS (Noble Numbat)`
- architecture: `x86_64`
- desktop session type: `Xvfb X11 session under dbus-run-session`
- package baseline or image provenance: `ubuntu@sha256:786a8b558f7be160c6c8c4a54f9a57274f3b4fb1491cf65146521ae77ff1dc54`
- preinstalled runtime/tooling allowed before acquisition: `none inside the container beyond the base ubuntu:24.04 image; GTK/webkit/Xvfb/dbus packages were installed as part of the validation path`
- network posture or access assumptions relevant to acquisition/install: `outbound network available for apt packages and governed artifact download via GitHub-backed release assets`
- local storage/install context relevant to the run: `deb copied into /tmp and installed with dpkg inside the disposable container`
- reset proof or freshness evidence handle: `artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/os-release.txt`; `artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/container-env.txt`
- notes on any deviation from an ideal blank environment: `This is a containerized GUI path rather than a separate physical workstation or VM, but SD-15 explicitly allows a containerized GUI equivalent when the environment is named and fresh. The run stayed isolated from the authoring checkout and started from a fresh Ubuntu image each time.`

## Build under test identity
These fields must identify the governed artifact being validated.

- tester-visible build label or version: `codex-desktop-shell-scaffold@0.0.0`
- tester-facing channel / support label: `alpha / Linux first-class`
- operator provenance handle: `github-release://electricm0nk/codex/alpha-v0.0.0-c2cea5c6`
- commit or build identity: `c2cea5c6baeb3ca34077b85331214c4b42a4809c`
- publication or acquisition handle: `https://github.com/electricm0nk/codex/actions/runs/28463728483`
- artifact name / package format: `Codex Desktop Shell Scaffold_0.0.0_amd64.deb`
- artifact checksum or immutable integrity handle when available: `sha256 c8f2f1b48a5f7fcdc2bc7e5db6ce0e2e1568d8e23a62c2d8b97ce8438b2e4031`
- rollback / withdrawal context if relevant: `not-applicable`
- package/install context expected by this run: `official Linux alpha deb installed with dpkg on Ubuntu 24.04.4`

## Run entry conditions
A truthful clean-machine run is launchable only when each condition below is either satisfied or explicitly marked blocked.

| Entry condition | Required state before execution | Evidence handle or note | If unsatisfied, status | Failure route / primary SD-15 class |
|---|---|---|---|---|
| Clean environment selected | Named clean environment exists and its identity fields are populated | `docker://ubuntu@sha256:786a8b558f7be160c6c8c4a54f9a57274f3b4fb1491cf65146521ae77ff1dc54/sd15-clean-run-3`; `artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/os-release.txt` | `blocked` | `install/use defect` |
| Freshness proven | Reset / reimage / fresh-VM evidence exists for the environment under test | Fresh ubuntu:24.04 container launch plus captured base environment files in `artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/` | `blocked` | `install/use defect` |
| Governed build identified | Build/channel/provenance/acquisition fields are populated from SD-12 truth | `artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/provenance.json`; `artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/checksums.sha256`; `artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/artifact.sha256.txt` | `blocked` | `packaging or distribution defect` |
| Bounded workflow target selected | The exact workflow under test is named from the install/use matrix | Linux alpha row in `artifacts/tranche-2-install-and-use-matrix.md` | `blocked` | `status or documentation drift` |
| Evidence capture path available | Screenshot/log/attachment destination or equivalent evidence path is known | `artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/` and linked receipt `artifacts/sd15-rr-2026-07-01-linux-alpha-clean-machine-ge08-fallback.md` | `blocked` | `status or documentation drift` |

## Ordered execution ledger
Mark each row only from real execution evidence. If a row fails or blocks, either embed the needed evidence here or reference the linked SD-15 regression receipt handle.

| Step | Operator action on the named clean environment | Expected result | Required evidence to mark `pass` or `pass-with-known-bounds` | Failure route / primary SD-15 class | Status | Evidence handle(s) | Linked regression receipt / blocker handle | Notes |
|---|---|---|---|---|---|---|---|---|
| 1. Confirm clean baseline | Started a disposable Ubuntu 24.04 container, recorded OS/runtime identity, and preserved the containerized GUI path as the named clean environment | The run starts from the named clean environment, not the authoring machine | Environment screenshot or system-info capture, reset proof handle, and populated clean-environment fields | `install/use defect` if the environment cannot be established cleanly; `status or documentation drift` if durable surfaces disagree on what environment was used | `pass` | `artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/os-release.txt`; `artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/container-env.txt` |  | Fresh-image container evidence exists and the environment stayed isolated from the authoring checkout. |
| 2. Acquire governed build | Downloaded the governed Linux alpha deb plus release checksum and provenance files from the GitHub-backed alpha publication surface | The exact build under test is retrieved with named provenance | Artifact handle, download/publication evidence, build label, channel/support label, and checksum/integrity handle when available | `packaging or distribution defect` if the governed artifact cannot be obtained or proven; `unsupported semantics or known unsupported paths` only if adjacent authority explicitly says this path is unavailable | `pass` | `artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/provenance.json`; `artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/checksums.sha256` |  | Governed publication handle and release metadata were available. |
| 3. Record build identity before use | Verified the deb checksum against the published checksums file and preserved the published source revision/channel metadata before install | The report preserves the exact build being exercised | Visible build/version evidence, provenance handle, package format, and any immutable build identifier available before launch | `packaging or distribution defect` if identity is ambiguous or contradictory; `status or documentation drift` if durable surfaces overclaim a build/channel state that the artifact does not support | `pass` | `artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/artifact.sha256.txt`; `artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/checksums.sha256`; `artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/provenance.json` |  | Deb checksum matched the published checksum for the governed alpha asset. |
| 4. Install or unpack | Installed the deb inside the named Ubuntu container with `dpkg -i`, allowing dependency repair via apt if needed | The build becomes install-ready on the named environment | Install transcript, screenshot, or equivalent proof plus package/install context | `install/use defect` if install fails; `packaging or distribution defect` if the package itself is malformed or incomplete | `pass` | `artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/dpkg-status.txt` |  | Package status shows `install ok installed` with version `0.0.0`. |
| 5. First launch | Launched `/usr/bin/codex_desktop_shell_scaffold` under `dbus-run-session xvfb-run` and captured X11 evidence | The app launches far enough to judge workbench entry truthfully | Launch screenshot, logs/diagnostics when present, and observed versus expected result | `install/use defect` when launch fails; `status or documentation drift` if launch/status expectations conflict with durable truth surfaces | `pass` | `artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/window.png`; `artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/xwininfo.txt`; `artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/app.stderr.log`; `artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/app-process.txt` |  | The app produced a visible window titled `Codex Desktop Shell Scaffold`; only non-fatal EGL warnings appeared in stderr. |
| 6. Reach bounded workflow entry | Waited for the workbench surface to render and captured the visible tester workbench state | The named bounded workflow becomes reachable on the clean environment | Screenshot or equivalent visible proof of workbench entry plus the workflow name under test | `install/use defect` if the workbench cannot be reached; deeper failures should not be flattened into install/use once entry succeeds | `pass-with-known-bounds` | `artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/window.png`; `artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/xwininfo.txt` | `artifacts/sd15-rr-2026-07-01-linux-alpha-clean-machine-ge08-fallback.md` | The clean run reached an explicitly labeled `SD-11 TESTER WORKBENCH` fallback surface, not the preferred GE08 authoring workbench. Entry exists, but it is bounded and visibly degraded. |
| 7. Exercise the bounded mission | Observed whether the preferred GE08 workbench loaded the Guard Stance proof package and displayed package state, preview state, and structured snapshot | The named workflow completes or reaches a truthful bounded stopping point | Mission-specific screenshots/logs, observed versus expected behavior, and any adjacent SD-13 or SD-14 context required by the path | Route by evidence: `ui or presentation defect`, `rules-engine defect`, `content or data defect`, `unsupported semantics or known unsupported paths`, or `persistence, migration, or saved-state continuity defect` as appropriate | `failed` | `artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/window.png`; `artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/app.stderr.log` | `artifacts/sd15-rr-2026-07-01-linux-alpha-clean-machine-ge08-fallback.md` | The UI reported: `GE08 authoring workbench unavailable: Failed to load GE08 authoring workbench: package root does not exist: tests/fixtures/ge08/guard-stance-package (resolved to /home/runner/work/codex/codex/tests/fixtures/ge08/guard-stance-package).` This materially contradicts the repo README's expected current behavior for the bounded GE08 workbench. |
| 8. Capture issue-ready evidence on any non-pass result | Preserved screenshot, X11 window evidence, package/provenance/checksum files, environment identity, and a durable SD-15 receipt | Every non-pass state has durable evidence and a classifiable route | Attachment handles, redaction posture, observed/expected statement, reproduction or impossibility note, and linked receipt fields per the SD-15 regression receipt schema | `status or documentation drift` if the report tries to summarize failure without the needed evidence; otherwise use the class established by the underlying failing step | `pass` | `artifacts/sd15-rr-2026-07-01-linux-alpha-clean-machine-ge08-fallback.md`; `artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/` | `artifacts/sd15-rr-2026-07-01-linux-alpha-clean-machine-ge08-fallback.md` | Evidence is current, complete, and routeable; the blocker is now cleared into a concrete defect receipt. |

## Evidence minimums for any non-pass step
A blocked, failed, or bounded step is not truthful unless it records or links at least:
- the named clean environment handle
- tester-visible build label/version
- tester-facing channel/support label
- operator provenance handle and immutable build identity when available
- publication/acquisition handle when relevant
- platform/OS and package/install context
- the exact step and bounded workflow under test
- observed behavior and expected behavior
- diagnostics, screenshots, logs, or explicit absence markers
- primary SD-15 class and linked adjacent-authority references
- attachment handles plus redaction posture
- reproduction steps or an explicit impossibility note when the path could not proceed

## Final verdict gate
Do not populate the verdict until the ordered execution ledger is complete enough to support it.

| Final verdict | When it is allowed | Required proof burden | Forbidden shortcut |
|---|---|---|---|
| `pass` | Every required step reached `pass`, the clean environment is named, the build identity is named, and evidence exists for the bounded workflow on that clean environment | Named environment identity, governed build identity, per-step evidence handles, and no hidden unsupported assumptions | Treating authoring-machine success or a filled template as proof |
| `pass-with-known-bounds` | The run succeeded, but an explicit bound remains visible and grounded by adjacent authority without counterfeiting broader support | Same as `pass`, plus the exact bound, adjacent-authority citation, and why the bound does not invalidate the bounded mission claim | Hiding a material limitation inside notes while still claiming ordinary `pass` |
| `blocked` | A prerequisite, governed artifact handle, environment fact, or proof surface prevented truthful completion | Evidence of the blocker, the step where it occurred, and the SD-15 class used to route it | Softening unresolved acquisition or environment gaps into optimism |
| `failed` | The named clean-machine path materially contradicted the tranche claim under test | Evidence of the failed step, observed versus expected behavior, and triage route strong enough for later reconstruction | Treating missing evidence as proof that the run failed |

## Final report verdict
- freshness: `current`
- sufficiency: `complete`
- verdict: `failed`
- primary SD-15 class: `content or data defect`
- linked receipt: `artifacts/sd15-rr-2026-07-01-linux-alpha-clean-machine-ge08-fallback.md`
- decisive conclusion: The clean-machine blocker is cleared. The governed Linux alpha deb can be acquired, installed, launched, and observed on a named clean environment, but the preferred GE08 workbench still fails on that governed artifact because the Guard Stance package root resolves to a nonexistent path. The next truthful move is a fix plus a superseding alpha rerun, not continued claims that clean-machine proof is merely pending.

## Explicit refusals
- do not treat this document's existence as evidence that a clean-machine run occurred
- do not mark any step `pass` without the evidence handle required for that row
- do not treat authoring-machine proof, local feature-branch proof, or headless build proof as clean-machine validation
- do not omit the clean environment identity, build identity, or acquisition path and then still claim a verdict
- do not collapse bounded workflow failures into generic install/use noise when the evidence shows a deeper UI, rules, content, unsupported, or persistence class
- do not let `not-run` or missing fields be interpreted as silent success
