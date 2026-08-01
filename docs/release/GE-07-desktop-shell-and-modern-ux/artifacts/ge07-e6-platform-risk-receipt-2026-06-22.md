---
title: GE07-E6 Platform Risk Receipt
artifact_type: spike-receipt
stc_id: STC-CODEX-GE-07
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE07-E6 — Cross-platform packaging and ship-readiness spike
workflow_route: planning
readiness: planning-ready
status: active
review_date: 2026-06-22
owner: Todd Hintzmann
scope: program
code_authority: false
related_artifacts:
  - ./cross-platform-build-constraint-questions.md
  - ../../../plans/spec-domains/GE-09-expansion-packaging-and-release-governance.md
---

# GE07-E6 Platform Risk Receipt

## Verdict
GE07-E6 can now make one honest claim and must refuse a second.

Honest claim: the packaging and ship-readiness blocker set for Linux, Windows, and macOS is now grounded enough to guide later release-governance work.

Refused claim: GE-07 is not ship-ready, and it is not even packaging-ready for a real shell slice yet.

The decisive reason is simple: there is still no real desktop shell subtree in the Codex repo, no Tauri project to build, and no platform-signing/tooling lane grounded beyond documentary research. Packaging truth can be mapped now. Release readiness cannot be claimed now.

## What this pass proved
1. `origin/develop` still contains no `apps/desktop/`, `src-tauri/`, or frontend package files, so there is no real shell slice to package.
2. The current Linux host has Rust and Node, but it does not currently have Tauri CLI, Linux Tauri system packages, cross-Windows packaging tools, or Apple signing tooling.
3. Official Tauri documentation imposes real platform-specific obligations that must remain explicit rather than being hand-waved behind “Tauri is cross-platform”.
4. The first truthful packaging posture is documentary risk control plus later bounded proof receipts, not early ship-readiness theater.

## Live repo and host evidence
### Repo identity
Commands run in `/home/ubuntu/workspace/repos/codex` on 2026-06-22:
- `git -C /home/ubuntu/workspace/repos/codex branch --show-current`
- `git -C /home/ubuntu/workspace/repos/codex rev-parse HEAD`
- `git -C /home/ubuntu/workspace/repos/codex rev-parse origin/develop`
- `git -C /home/ubuntu/workspace/repos/codex ls-tree -r --name-only origin/develop | grep -E '^(apps/desktop/|src-tauri/|package.json|pnpm-lock.yaml|package-lock.json|yarn.lock|vite\.config|tsconfig|src/)' || true`

Observed results:
- checked-out branch: `ge06-e3-f2-classifier-impl`
- checked-out commit: `cc45f2c84b0c6bd3b3a7886f9f3068ece8b58e48`
- `origin/develop`: `7bc89e8c1edf8f1d1a6d490a0ad28ac72fc6f104`
- tree entries on `origin/develop`: Rust core/test files only; no `apps/desktop/`, `src-tauri/`, `package.json`, or other shell/package markers

This is the primary stop condition. Packaging research may continue. Stronger ship claims may not.

### Host/runtime truth
Commands run:
- toolchain presence/version check for `cargo`, `rustc`, `node`, `npm`, `tauri`, and `rustup`
- `rustup target list --installed`
- `dpkg-query` checks for `libwebkit2gtk-4.1-dev`, `libwebkit2gtk-4.1-0`, `libgtk-3-dev`, `libgtk-3-0`, `libayatana-appindicator3-dev`, `libxdo-dev`, `lld`, `llvm`, and `nsis`
- presence check for `cargo-xwin`, `xwin`, `signtool`, `xcode-select`, `security`, `codesign`, global `@tauri-apps/cli`, and `cargo tauri`
- `uname -a`
- `cat /etc/os-release`

Observed results:
- present: `cargo 1.96.0`, `rustc 1.96.0`, `node v22.22.3`, `npm 10.9.8`, `rustup`
- absent: `tauri`, `cargo tauri`, global `@tauri-apps/cli`
- installed Rust targets: `x86_64-unknown-linux-gnu` only
- absent Linux build packages on this host: `libwebkit2gtk-4.1-dev`, `libwebkit2gtk-4.1-0`, `libgtk-3-dev`, `libgtk-3-0`, `libayatana-appindicator3-dev`, `libxdo-dev`
- absent cross-Windows/build tools on this host: `lld`, `llvm`, `nsis`, `cargo-xwin`
- absent Apple/Windows signing tools on this host: `signtool`, `xcode-select`, `security`, `codesign`
- host OS: `Ubuntu 24.04.4 LTS`

That means the current machine is not yet a truthful packaging proof surface even for Linux, and it is far from a cross-platform release surface.

## Official Tauri constraints recovered this pass
### Linux
From Tauri prerequisites, Debian/AppImage, and RPM docs:
- Linux development requires packages including `libwebkit2gtk-4.1-dev`, `libxdo-dev`, `libssl-dev`, `libayatana-appindicator3-dev`, and related GTK dependencies.
- AppImage, Debian, and RPM packaging all inherit the same compatibility rule: build on the oldest base system you intend to support that still ships WebKitGTK 4.1.
- Tauri explicitly points to Ubuntu 22.04 or Debian 12 as suitable baseline examples.
- Building on a newer base system can raise the minimum glibc requirement and create runtime failure on older Linux systems.
- AppImage is easier to distribute but typically larger and still does not remove the old-enough-build-base requirement.

### Windows
From Tauri prerequisites, Windows installer, and Windows signing docs:
- Windows development requires Microsoft C++ Build Tools and WebView2.
- `.msi` packaging uses WiX and can only be created on Windows.
- Cross-building from Linux/macOS is only documented for NSIS, with caveats; Tauri recommends native Windows or CI runners where possible.
- Cross-building from Linux/macOS also requires NSIS, LLVM/LLD, a Windows MSVC Rust target, and usually `cargo-xwin`.
- Windows code signing is not required merely to execute the app, but it is required to avoid SmartScreen/browser trust problems and for Microsoft Store distribution.

### macOS
From Tauri prerequisites and macOS signing docs:
- Desktop development requires Xcode Command Line Tools at minimum.
- Shipping outside the App Store still requires Apple signing infrastructure, typically a Developer ID Application certificate.
- Signing requires an Apple device and an Apple Developer account; the docs call out the paid account requirement for distribution.
- CI signing requires exported certificate material and Apple credentials.

### Updater
From the Tauri updater plugin docs:
- updater signatures are mandatory; verification cannot be disabled
- losing the updater private key means future updates cannot be published for already-installed users
- updater key custody is therefore release-governance work, not an incidental shell-scaffold detail

## Platform risk table
| Platform | Grounded packaging path | Grounded blocker now | Earliest honest proof target |
|---|---|---|---|
| Linux | Tauri can emit `.deb`, `.rpm`, and `AppImage` bundles | no shell subtree exists; current host lacks Linux Tauri system dependencies; current host is Ubuntu 24.04 rather than the older baseline Tauri recommends for broad compatibility | after a real shell slice exists, stand up a bounded Linux build lane on Ubuntu 22.04 or Debian 12 and produce one build receipt |
| Windows | Tauri can emit NSIS setup executables and, on Windows, WiX `.msi` installers | no shell subtree exists; no Windows target/tooling installed here; no signing material or Windows runner grounded | after a real shell slice exists, decide whether the first proof is NSIS-only or native-Windows MSI/NSIS, then generate a Windows packaging receipt |
| macOS | Tauri can bundle/sign macOS apps with Apple signing infrastructure | no shell subtree exists; no Apple tooling or Apple signing credentials/hardware grounded in this environment | after a real shell slice exists, route packaging proof onto a Mac-backed runner with signing identity and notarization plan |

## Release-governance decision inputs produced by this pass
1. Linux compatibility baseline must be an explicit policy decision. If Codex wants older-distribution reach, do not treat the current Ubuntu 24.04 host as the canonical release builder.
2. Windows installer format must be chosen consciously. If the program wants the easiest early external distribution experiment, NSIS is the first likely candidate. If it wants MSI, a Windows-native lane is mandatory.
3. macOS distribution is a governance and credential problem, not just a build-command problem. Apple account, certificate custody, and Mac-backed CI/hardware must be budgeted explicitly.
4. Updater adoption should remain deferred until release-governance is ready to own signing-key custody and rotation risk.
5. Platform-specific config should be expected. Tauri supports `tauri.linux.conf.json`, `tauri.windows.conf.json`, and `tauri.macos.conf.json`, which argues for keeping future packaging policy explicit per OS instead of burying everything in one generic config blob.

## What must be answered before the first real shell slice
These are the packaging questions that affect even the first non-production implementation lane:
1. Which Linux baseline does Codex consider truthful for first-proof packaging receipts: Ubuntu 22.04, Debian 12, or something narrower?
2. Is the first shell slice allowed to stay completely non-bundled, or must it prove one Linux package output immediately once a scaffold exists?
3. Should Windows early proof be deferred entirely until a shell exists on a Windows runner, or is a Linux-hosted NSIS experiment acceptable as a documentary spike only?
4. Does the first real shell slice need any local database, bundled assets, tray support, media playback, or native dialogs that would enlarge packaging scope immediately?

## What can wait for later GE-09 release-governance work
1. store-distribution posture
2. updater rollout and release channels
3. certificate storage and rotation policy
4. notarization automation and release publishing pipelines
5. public download/distribution surfaces

## What this receipt authorizes
This receipt authorizes documentary truth only:
- GE-07 may claim that platform blocker discovery now has live repo evidence plus official Tauri source grounding
- later GE-07 or GE-09 governance work may reuse these blocker classes and decision inputs
- future shell work may refuse ship-readiness theater until a real shell slice and platform receipts exist

## What this receipt does not authorize
This receipt does not authorize:
- any claim that Codex currently builds as a Tauri desktop app
- any claim that Linux, Windows, or macOS packaging is already proven
- any updater/signing key generation or release-channel policy by implication
- any GE-07 code authority
- any claim that Tauri's cross-platform story removes platform-specific signing, runtime, or builder obligations

## Completion rule
This artifact is complete because it grounds the live repo stop condition, records the current host/tooling gaps, converts Tauri's official platform constraints into Codex-specific risk classes, and preserves the decisive truth: packaging questions are now visible, but ship readiness remains unearned.