---
canonical: true
owner: operator
bundle_id: SD-37
date: 2026-10-02
---

# SD-37 Technical Requirements

## 1. Pre-loop prerequisites

Each prerequisite below has a check and an owning card. None of them is assumed.

| # | Prerequisite | Check | State on 2026-10-02 | Closes at |
|---|---|---|---|---|
| P-1 | `tranche/17` exists from `origin/develop` | `git merge-base --is-ancestor 20bf84a3b2 tranche/17 && echo OK` | local only; not pushed | C1 (push) |
| P-2 | Oracle on pin | `git -C "$PCGEN_REPO_DIR" rev-parse HEAD` = `PCGEN_ORACLE_SHA` | on pin (default checkout; `$PCGEN_REPO_DIR` unset) | — |
| P-3 | SF data in a fresh sparse fetch | E0.1's fresh-clone command | **absent**: the sparse list is `data/pathfinder system/gameModes/Pathfinder data/darwins_world_2/…` (`scripts/pcgen-oracle-pin.env`); `scripts/fetch-pcgen-oracle.sh:94` defaults to `data/pathfinder` | E0.1 |
| P-4 | Version `0.17.0` stamped | CUI F-17 commands | `0.16.0` | C1 |
| P-5 | Package reviewed by Opus/Fable | C0.2 receipt | **done** 2026-10-02 (`artifacts/cycle_0/C0.2_cycle_receipt.md`) | C0.2 |
| P-6 | Disk headroom for ≥ 1 full sweep (~24 G) plus E4a's separate target dir | `df -h /` | 468 G free of 1.5 T (68% used) | re-checked before every sweep |
| P-7 | Memory for 3 cargo lanes | `free -g` | 134 G total, 52 G free (`free -g \| sed -n 2p`) | re-checked before long runs |
| P-8 | SRD reachable for E0.4 | E0.4's first fetch | reachable on 2026-10-02: `curl -s -o /dev/null -m 15 -w '%{http_code}' https://www.aonsrd.com/Classes.aspx` → 200 (C0.2 probe; a probe, not proof the needed pages hold the values) | E0.4 (SD-c if not) |
| P-9 | Clean tree | `git status --porcelain` empty | empty before authoring | every wave commit |

## 2. Normative requirements

- **NR-1 (no PF behaviour change).** Every PF render of the seeds is byte-identical across E1, E2,
  E4 (which generalises PF `money.rs`/`encumbrance.rs`), E6 and E4a, checked with E1.4's harness. A PF diff is a defect (SD-i). It is never re-baselined.
- **NR-2 (generic SF compute).** No SF per-class compute module. `ls
  src/rules_core/pilot_compute | awk 'tolower($0) ~ /soldier|mystic|envoy|technomancer|operative|mechanic|solarian|biohacker|vanguard|witchwarper/' | awk 'END{print NR}'`
  must print 0.
- **NR-3 (paper sheet).** Compute only sheet-total values (`decisions.md §5`).
- **NR-4 (overloaded fields).** An SF value is read only through the SF mapping table. A PCGen
  field with no SF oracle row is refused (`decisions.md §8`).
- **NR-5 (licence).** Only books listed in the SF registry are read. SSRGG, LPJ and
  `paizo/core/_society/` are absent from it and named in the exclusion record (`decisions.md §6`;
  test written by E3.1).
- **NR-6 (PCGen-free live side).** `pcgen_residue_gate.py --check --closure` is PASS on PF and SF
  packages. "Live residue = 0" means no PCGen token is spelled in a live file. It does not mean the
  live side never reads PCGen-derived data (SD-35 progress note), and receipts say so in those
  words.
- **NR-7 (crate wall).** `codex` never depends on `codex-ingest`; the desktop takes `codex-ingest`
  as a dev-dependency only (`verify.sh` `crate-wall` stage).
- **NR-8 (fixtures).** Every SF fixture value cites an SRD URL. No fixture is generated from
  converter output (`decisions.md §18`).
- **NR-9 (engine single source on desktop).** No hand-kept SF tables in `apps/desktop/src/**`
  (R2).
- **NR-10 (isolated app root).** Any app-launching harness runs under a per-run `XDG_DATA_HOME`
  (R5).
- **NR-11 (tiering).** Every `agent()` sets `model` (`decisions.md §11`).
- **NR-12 (wired integration).** The four-check audit passes on every code-bearing card.
  `StubAdapter` stops serving `starfinder-1e` (registry 0002).

## 3. Deferred from SD-36 / predecessors (adopted or registered)

| Item | Source | SD-37 disposition |
|---|---|---|
| Second PCGen-format reader (C2.1) | SD-35 FSR C2.1 (owner "SD-36", stale) | **Adopted** → E3 |
| `rules_tables` → data package (FS-3) | SD-36 FSR FS-3, ruling 7 | **Adopted** → E4a |
| `.lst` citation burn-down (D6) | SD-36 scope-draft D6 | **Adopted** → E4a.3; D6's `lst_file` half is already present (`scripts/pcgen_residue_gate.py:223,229`) |
| CI oracle fetch (FS-5) | SD-36 FSR FS-5 | Partly covered by E0.1 (the sparse list); the CI step already calls `fetch-pcgen-oracle.sh` (`.github/workflows/{pr-tests,tranche-3-ci,publish-tester-release}.yml`) |
| Oracle-parity roster widening (FS-9) | SD-36 FSR FS-9 | SF roster at E7.1; PF widening stays a candidate (FSR-C5) |
| Source book through `HeldSeed` (FS-7) | SD-36 FSR FS-7 | Cross-system collisions handled by system namespacing (E1, SD-h); PF FS-7 itself stays a candidate (FSR-C4) |
| SD-34 P1s (FS-2), FS-15, FS-27, FS-28, Epic F next-week list | SD-36 | Candidates (FSR-C1–C3, C6–C7) |
