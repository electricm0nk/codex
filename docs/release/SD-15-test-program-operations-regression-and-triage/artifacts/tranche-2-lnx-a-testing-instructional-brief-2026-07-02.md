# Tranche-2 LNX-A Testing Instructional Brief

## Purpose
Issue the actual tester package for the grounded tranche-2 Linux alpha row so a tester can execute the install/use and bounded workflow path without waiting on tester identity theater, machine-selection theater, or a separately invented launch ritual.

## Doctrinal correction
- This brief explicitly corrects the earlier blockage that treated named tester selection as a prerequisite for issuing instructions.
- For this lane, **tester identity and exact machine identity are evidence fields, not gating conditions for brief issuance**.
- The real prerequisites are: a governed build handle, a Linux graphical session, the required runtime packages, and an evidence-capture path.
- A tester may be Todd Hintzmann or anyone else operating under Todd's authority. That difference matters for attribution, not launchability.
- Historical 2026-07-01 non-launch records remain truthful for that bounded attempt. This brief is the superseding forward-use packet.

## Packet identity
- packet type: `tester instructional brief`
- packet date: `2026-07-02`
- governing row: `LNX-A`
- target platform/support tier: `Linux / first-class`
- governed channel/support label: `alpha / first-class tester track`
- governed build label/version: `codex-desktop-shell-scaffold@0.0.0`
- governed publication handle: `github-release://electricm0nk/codex/alpha-v0.0.0-c2cea5c6`
- governed publication release URL: `https://github.com/electricm0nk/codex/releases/tag/alpha-v0.0.0-c2cea5c6`
- governed publication run URL: `https://github.com/electricm0nk/codex/actions/runs/28463728483`
- governed source revision: `c2cea5c6baeb3ca34077b85331214c4b42a4809c`
- required artifact name: `Codex.Desktop.Shell.Scaffold_0.0.0_amd64.deb`
- authoritative issue destination: `https://github.com/electricm0nk/codex/issues`

## Authority anchors
- `programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/tranche-2-install-and-use-matrix.md`
- `programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/tranche-2-clean-machine-validation-report.md`
- `programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/tranche-2-external-test-cycle-plan.md`
- `programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/regression-receipt-schema.md`
- `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/github-bug-report-intake-contract.md`
- `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/github-enhancement-request-intake-contract.md`
- `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/tester-feedback-evidence-capture-matrix.md`
- `/home/ubuntu/workspace/repos/codex/README.md`

## Who may execute this packet
Any tester operating a **graphical Linux desktop session** may execute this brief if they can:
1. acquire the governed alpha artifact named above
2. install the required Linux runtime packages
3. launch the app window
4. capture screenshots, visible text, and short notes when something goes wrong

This packet does **not** authorize macOS, Windows, `beta`, or ad hoc feature-branch artifacts as official tranche-2 evidence.

## Required environment
- graphical Linux desktop session
- Ubuntu 24.04-class environment strongly preferred because that path is the one already grounded
- outbound network access sufficient to obtain the governed artifact and apt packages
- `apt`, `sha256sum`, screenshot capability, and a writable local evidence folder

## Official-scope boundary
Use the governed Linux alpha publication for official evidence. Do **not** substitute:
- a local repo build
- a feature-branch artifact
- a hand-copied binary of uncertain origin
- a `beta` candidate track that does not exist yet

Those may be useful for developer proof, but they do not count as official tranche-2 tester evidence for this row.

## Evidence bundle setup
Before launching, create a local evidence folder and capture environment identity.

Suggested local folder:

```bash
mkdir -p "$HOME/codex-evidence/lnx-a-$(date +%F)"
cd "$HOME/codex-evidence/lnx-a-$(date +%F)"
cat /etc/os-release > os-release.txt
uname -a > uname.txt
```

If the tester is not on Ubuntu 24.04-class Linux, record the exact distro/version clearly. That does not forbid the run, but it must remain visible in the evidence.

## Acquisition instructions
1. Open the governed publication release URL: `https://github.com/electricm0nk/codex/releases/tag/alpha-v0.0.0-c2cea5c6`
2. If the release page is visible, acquire the governed Linux alpha publication for `alpha-v0.0.0-c2cea5c6` directly from the release assets list there.
3. Use the publication run URL only as provenance context if you need to inspect how the release was built: `https://github.com/electricm0nk/codex/actions/runs/28463728483`
4. Download these files into the evidence folder:
   - `Codex.Desktop.Shell.Scaffold_0.0.0_amd64.deb`
   - `checksums.sha256`
   - `provenance.json`
5. Do not continue if you cannot tie the download back to the governed build identity above.

## Integrity verification
Run:

```bash
sha256sum "Codex.Desktop.Shell.Scaffold_0.0.0_amd64.deb" | tee artifact.sha256.txt
grep -F "Codex Desktop Shell Scaffold_0.0.0_amd64.deb" checksums.sha256 | tee published-checksum-line.txt
```

Current release-unit quirk: the published `checksums.sha256` file still records the Linux asset with a spaced filename (`Codex Desktop Shell Scaffold_0.0.0_amd64.deb`) even though the actual release asset name is dot-separated (`Codex.Desktop.Shell.Scaffold_0.0.0_amd64.deb`). Treat the SHA-256 value as the authority for integrity verification and record the filename mismatch as a separate packaging/publication-truth defect if encountered.

If the checksum does not match the published checksum, stop immediately and classify the outcome as `packaging or distribution defect`.

## Runtime package prerequisites
Install the Linux runtime dependencies required by the official deb:

```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-0 libgtk-3-0
```

These are the grounded package prerequisites named in the tranche-2 install/use matrix and clean-machine receipt.

## Install instructions
Install the governed deb:

```bash
sudo dpkg -i "Codex.Desktop.Shell.Scaffold_0.0.0_amd64.deb"
dpkg -s codex-desktop-shell-scaffold | tee dpkg-status.txt
which codex_desktop_shell_scaffold | tee binary-path.txt
```

Expected package identity:
- package: `codex-desktop-shell-scaffold`
- version: `0.0.0`
- depends: `libwebkit2gtk-4.1-0`, `libgtk-3-0`

If installation fails, preserve the full terminal output and classify the outcome as `install/use defect` unless the artifact itself is malformed, in which case classify it as `packaging or distribution defect`.

## Launch instructions
From the graphical Linux desktop session, launch:

```bash
/usr/bin/codex_desktop_shell_scaffold
```

Capture at minimum:
- one screenshot of the first meaningful app window state
- any visible error text
- brief notes on what happened versus what you expected

## Bounded mission bundle
Execute the following missions in order. Unlike the older over-gated doctrine, only integrity failures, acquisition failures, or inability to launch should halt the entire packet immediately. Missing labels or bounded workflow failures should usually be captured and carried forward as evidence.

### M1 — Acquire governed build
- goal: obtain the exact governed Linux alpha artifact and companion files
- success signal: artifact, checksum file, and provenance file are present and traceable to the governed run
- failure class: `packaging or distribution defect`

### M2 — Install and first launch
- goal: install the deb and open the app window successfully
- success signal: app launches from `/usr/bin/codex_desktop_shell_scaffold`
- failure class: `install/use defect`

### M3 — Confirm visible identity if present
- goal: capture any visible build/channel/support wording the app exposes
- success signal: screenshot or note preserving visible identity
- important correction: if the app does not surface this clearly, **record that fact and continue** so long as the app remains usable enough to pursue the bounded mission
- likely route if identity is missing or contradictory: `status or documentation drift`

### M4 — Reach bounded tester workbench
- goal: reach the bounded tester/workbench surface truthfully
- success signal: visible workbench or bounded fallback state
- acceptable bounded result: explicitly labeled `SD-11 TESTER WORKBENCH` fallback
- failure class: `install/use defect` if the app cannot reach a usable workbench surface at all

### M5 — Attempt the preferred GE-08 workbench path
- goal: attempt the Guard Stance proof-package path and observe whether package state, preview state, and structured snapshot behavior load truthfully
- historical likely outcome from the grounded clean-machine receipt: the governed artifact may fall back and report `package root does not exist` for `tests/fixtures/ge08/guard-stance-package`
- critical rule: if this happens, **treat it as a valid test outcome, not tester error**
- likely class for that known failure: `content or data defect`

### M6 — Produce issue-ready evidence
- goal: leave behind enough evidence that the result can be triaged without memory or folklore
- required output: screenshot(s), observed/expected notes, reproduction steps or impossibility note, environment identity, build identity, and classification guess if obvious
- if GitHub submission is not completed immediately, preserve a structured local draft instead of dropping the evidence

## Expected truthful outcomes
The tester should not measure success by whether the product looks finished. The packet is successful if it produces one of the following truthful end states:
1. the governed artifact launches and the GE-08 path works
2. the governed artifact launches and reproduces the known bounded GE-08 failure with evidence
3. the governed artifact cannot be acquired, installed, or launched, and the blocker is captured with evidence

Any of those outcomes is operationally useful. Silence and half-memory are not.

## Known current bound
The current clean-machine receipt already established this likely bounded failure on the governed Linux alpha artifact:

> `GE08 authoring workbench unavailable: Failed to load GE08 authoring workbench: package root does not exist: tests/fixtures/ge08/guard-stance-package ...`

If you reproduce that exact result, capture it cleanly and file it as a bug. Do not spend the session inventing local repo-path workarounds. The purpose of the packet is to test the governed artifact, not to rescue it by hand.

## Issue filing contract
### File a bug when
- the governed artifact cannot be acquired or verified
- the deb does not install cleanly on the grounded Linux path
- the app does not launch
- the app launches but the bounded GE-08 mission fails or falls back unexpectedly
- visible build/channel/support identity is contradictory or materially missing

### File an enhancement when
- the app works as described but a bounded workflow is still too limited to accomplish a legitimate tester goal

### Required bug fields
- issue title summarizing the observable failure
- build label/version
- tester-facing channel/support label
- platform / OS
- current bounded workflow
- observed behavior
- expected behavior
- reproduction steps
- diagnostics / visible explanation text when present
- attachment summary and redaction declaration when attachments exist

### Required enhancement fields
- issue title summarizing the blocked workflow or missing capability
- build label/version
- tester-facing channel/support label
- platform / OS
- current bounded workflow
- tester goal
- current friction or limitation
- requested capability or improvement
- affected surface
- supporting evidence/examples when helpful

## Minimal local draft template when immediate GitHub filing is inconvenient
```markdown
# [bug|enhancement] <short title>

## Build / channel / platform / workflow
- build: codex-desktop-shell-scaffold@0.0.0
- channel/support: alpha / first-class tester track
- platform: <linux distro/version>
- workflow: LNX-A / bounded GE-08 Guard Stance path

## Observed behavior or current friction
<what happened>

## Expected behavior or tester goal
<what should have happened / what you were trying to do>

## Reproduction steps
1. ...
2. ...
3. ...

## Diagnostics / explanation text
<copy visible error text if present>

## Attachments / redactions
- screenshots: <paths or handles>
- logs: <paths or handles>
- redaction posture: <none | scrubbed | omitted>
```

## Narrow stop conditions
Stop immediately only when one of these is true:
- the artifact identity cannot be tied to the governed publication
- the checksum mismatches
- the deb cannot be installed far enough to test
- the app cannot launch far enough to observe a meaningful UI state
- evidence capture is impossible and would force the result back into folklore

Do **not** stop merely because:
- the app shows the bounded fallback workbench
- the GE-08 package path fails
- visible identity text is incomplete
- the result is disappointing

Those are reportable outcomes, not reasons to abort the packet.

## Completion rule
This brief is complete when the tester has produced either:
- a successful bounded-workflow confirmation, or
- an issue-ready evidence bundle showing the exact failure or bounded fallback encountered

The lesser models would have turned this into another permission chain. The correct action is simpler: execute, capture, classify, and move.
