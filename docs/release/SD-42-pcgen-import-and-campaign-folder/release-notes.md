---
canonical: true
bundle_id: SD-42
slug: pcgen-import-and-campaign-folder
tranche: 22
date: 2026-11-16
status: interim — replaced at E5.3
---

# Campaign Codex 0.22 — PCGen Import Accuracy and the Campaign Folder

SD-42 INTERIM NOTES. This body is replaced at E5.3 with the final notes; it exists so the release-notes
resolver passes from C1 on (`decisions.md §2`). Nothing publishes from `tranche-22`.

## Planned for 0.22

- PCGen import: one stack per item with its count; custom names and every custom property; nothing dropped
  (unmapped items become Create items and are listed in Notes); Pathfinder and Starfinder; checked whole-character
  against PCGen's own engine.
- Campaign Manager: a campaign is a folder of Markdown files you sync however you like; the creator is member
  #1; Party Resources, Adventure Log, Maps with pins, Wiki with backlinks and search; a shared `Characters/`
  backup that never overwrites another device's save; a live-preview Markdown editor.
- Linux: the `.deb` and AppImage start on Ubuntu 24.04 again (0.21.x was built against glibc 2.43; the release
  build now targets glibc 2.35 and a release gate fails on anything above it). Fixes #409.
