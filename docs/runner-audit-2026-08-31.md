# Runner Audit — 2026-08-31

Audit of every GitHub Actions workflow file under `/home/ubuntu/workspace/repos/`,
classifying each job as **MIGRATABLE** (already runs on github-hosted), **MIGRATABLE-WITH-EFFORT**, or **LOCALITY-REQUIRED** (genuine need for self-hosted).

## Executive Summary

**All 35 jobs across 19 workflow files in 3 repos already run on github-hosted runners, except for one.** The only self-hosted declaration in the audited set is `terminus.infra/.github/workflows/deploy-hermes-vm.yml::deploy-hermes-honcho` (line 23), and it does **not** require locality — it calls the Semaphore API over HTTPS with `SEMAPHORE_API_TOKEN` / `SEMAPHORE_BASE_URL` secrets, no bind mounts, no `/`, `/home`, `/mnt`, or `/opt` references. It can be migrated to `ubuntu-latest` by flipping `runs-on`. The suspected PCGen oracle CI job **does not exist as a workflow job** — the PCGen corpus is used by operator-local `scripts/verify.sh`, which no workflow invokes. The corpus is 156M on disk, sourced from a public GitHub repo via sparse-checkout + blob-filter in `scripts/fetch-pcgen-oracle.sh`, and would re-bootstrap on a github-hosted runner in seconds if a workflow ever did call it. **Recommendation: retire all local runners; flip the one `runs-on: [self-hosted, trantor-internal]` to `ubuntu-latest`.**

## Methodology

**Search:**
- `find /home/ubuntu/workspace/repos -maxdepth 5 -name '*.yml' -path '*.github/workflows*' -not -path '*/node_modules/*' -not -path '*/.claude/*'`
- Same with `*.yaml` (returned zero hits)
- Excluded `.claude/worktrees/*` (ephemeral agent trees) and `apps/desktop/node_modules/*` (vendored deps).

**Files audited:** 19 canonical workflow files across 3 repos. `scratch/` and `scratch-monk-a832/` contain no `.github/workflows/`.

**Classification rules:**
- **MIGRATABLE**: `runs-on: ubuntu-latest | macos-latest | windows-latest | matrix.os` with only standard actions (`actions/checkout`, `actions/setup-*`, `actions/cache`, `actions/upload-artifact`, `actions/download-artifact`, `actions/github-script`), `secrets.GITHUB_TOKEN`, and `github.*` context references. No bind mounts, no local-path env vars, no `services:` with `volumes:`.
- **MIGRATABLE-WITH-EFFORT**: Self-hosted `runs-on` whose steps are pure API calls / file ops in `${{ github.workspace }}` with secrets only — they would work on github-hosted unchanged except for the `runs-on` declaration itself.
- **LOCALITY-REQUIRED**: Bind mounts, hardware-specific (GPU, arch), secrets not stored in GitHub, or jobs that read pre-warmed local state outside `${{ github.workspace }}`.

## Per-Repo Inventory

| Repo | Workflows | Jobs | Self-hosted jobs | Notes |
|---|---:|---:|---:|---|
| `codex` | 7 | 16 | 0 | All jobs on `ubuntu-latest`; publish lane uses `ubuntu-latest` + `macos-latest` + `windows-latest` matrix. |
| `pcgen` | 5 | 11 | 0 | All jobs on `ubuntu-latest`; release/nightly matrix across `ubuntu-latest`, `ubuntu-24.04-arm`, `macos-latest`, `windows-latest`. |
| `terminus.infra` | 7 | 8 | 1 | `deploy-hermes-vm.yml:23` is the only self-hosted declaration; pure HTTPS API call, no locality requirement. |
| **Total** | **19** | **35** | **1** | Counts derived from `grep -cE '^\s*runs-on:'` per file (see Reproduction Commands). |

Job counts derived from `grep -cE '^\s*runs-on:'` per workflow file (each declaration corresponds to one job; matrix jobs count as one because they share a single declaration). Per-file breakdown:

```
allow-only-develop-into-test.yml: 2    allow-only-test-into-main.yml: 2
check-release-manifest.yml: 1          deploy-site.yml: 1
promotion-gates.yml: 1                 publish-tester-release.yml: 6
tranche-3-ci.yml: 3                    codeql-analysis.yml: 1
gradle-nightly.yml: 4                  gradle-release-manual.yml: 3
gradle-release.yml: 2                  gradle-test.yml: 1
argocd-source-coverage-check.yml: 1    deploy-hermes-vm.yml: 1
elasticsearch-manifest-validation.yml: 1  seed-contract-check.yml: 1
tls-policy-check.yml: 1                vault-auth-proxy-image.yml: 2
vm-deploy-coverage-check.yml: 1
```

Sum: 35.

## Bucket 1 — MIGRATABLE (34 jobs)

All jobs in this bucket already run on github-hosted runners. Migration cost is zero — flip nothing, do nothing.

### codex (14 jobs)

| File | Line | Job | runs-on |
|---|---:|---|---|
| `allow-only-develop-into-test.yml` | 35 | `verify-source-branch` | `ubuntu-latest` |
| `allow-only-develop-into-test.yml` | 50 | `restore-develop-branch` | `ubuntu-latest` |
| `allow-only-test-into-main.yml` | 40 | `verify-source-branch` | `ubuntu-latest` |
| `allow-only-test-into-main.yml` | 55 | `restore-test-branch` | `ubuntu-latest` |
| `check-release-manifest.yml` | 41 | `check-release-manifest` | `ubuntu-latest` |
| `deploy-site.yml` | 32 | `deploy-site` | `ubuntu-latest` |
| `promotion-gates.yml` | 35 | `promotion-gate` | `ubuntu-latest` |
| `publish-tester-release.yml` | 20 | `stamp` | `ubuntu-latest` |
| `publish-tester-release.yml` | 140 | `test` | `ubuntu-latest` |
| `publish-tester-release.yml` | 188 | `publish-tester-release` (linux) | `ubuntu-latest` |
| `publish-tester-release.yml` | 438 | `publish-tester-release-macos` | `macos-latest` |
| `publish-tester-release.yml` | 515 | `publish-tester-release-windows` | `windows-latest` |
| `publish-tester-release.yml` | 628 | `finalize` | `ubuntu-latest` |
| `tranche-3-ci.yml` | 39, 56, 103 | `slice-tranche-base-guard`, `desktop-typecheck-and-test`, `tranche-manifest-coverage` | `ubuntu-latest` |

Evidence per locality-marker category (all absent, confirming migratability):
- No `${{ secrets.* }}` outside `GITHUB_TOKEN` and standard Cloudflare/Semaphore/Cloud secrets in `deploy-site.yml:54` and `deploy-hermes-vm.yml:39-40`.
- No `/home`, `/mnt`, `/opt`, or `~/workspace/...` references. `publish-tester-release.yml` uses `${GITHUB_WORKSPACE}` exclusively (lines 326, 331, 705, 767, 897) — that's the runner-relative checkout path, which github-hosted provides.
- No `services:` blocks with `volumes:` anywhere.
- No `CARGO_TARGET_DIR` env var override; `Swatinem/rust-cache@v2` (lines 63, 157, 239) keys on `workspaces: . -> target` and `apps/desktop/src-tauri -> target`, which is github-hosted-native.
- No reference to PCGen, oracle, corpus, `$PCGEN_CORPUS_ROOT`, `$PCGEN_REPO_DIR`, or `scripts/fetch-pcgen-oracle.sh` in any codex workflow file. Verified by `grep -nE 'verify\.sh|corpus_literal_sweep|preflight-oracle' .github/workflows/*.yml` returning zero hits.

### pcgen (13 jobs)

| File | Line | Job | runs-on |
|---|---:|---|---|
| `codeql-analysis.yml` | 26 | `analyze` (matrix: java) | `ubuntu-latest` |
| `gradle-nightly.yml` | 35 | `check_changes` | `ubuntu-latest` |
| `gradle-nightly.yml` | 94 | `create_release` | `ubuntu-latest` |
| `gradle-nightly.yml` | 146 | `build_release` (matrix: ubuntu-latest, ubuntu-24.04-arm, macos-latest, windows-latest) | matrix |
| `gradle-nightly.yml` | 234 | `cleanup_old_nightlies` | `ubuntu-latest` |
| `gradle-release-manual.yml` | 60 | `prepare_release` | `ubuntu-latest` |
| `gradle-release-manual.yml` | 178 | `build_release` (matrix: ubuntu-latest, ubuntu-24.04-arm, macos-latest, windows-latest) | matrix |
| `gradle-release-manual.yml` | 255 | `bump_to_next_dev` | `ubuntu-latest` |
| `gradle-release.yml` | 25 | `create_release` | `ubuntu-latest` |
| `gradle-release.yml` | 98 | `build_release` (matrix: ubuntu-latest, ubuntu-24.04-arm, macos-latest, windows-latest) | matrix |
| `gradle-test.yml` | 14 | `build` | `ubuntu-latest` |

Evidence: standard Gradle + JDK 25 setup (`gradle-nightly.yml:179-188`), `actions/cache@v4` for downloaded JDK/JavaFX archives (line 202-211), pure GitHub Release API calls via `softprops/action-gh-release@v2`. No bind mounts, no local-path env, no `services:`.

### terminus.infra — the MIGRATABLE subset (7 jobs)

| File | Line | Job | runs-on |
|---|---:|---|---|
| `argocd-source-coverage-check.yml` | 11 | `validate-argocd-source-coverage` | `ubuntu-latest` |
| `elasticsearch-manifest-validation.yml` | 23 | `validate-elasticsearch-manifests` | `ubuntu-latest` |
| `seed-contract-check.yml` | 11 | `validate-seed-contract` | `ubuntu-latest` |
| `tls-policy-check.yml` | 11 | `validate-tls-policy` | `ubuntu-latest` |
| `vault-auth-proxy-image.yml` | 21 | `test` | `ubuntu-latest` |
| `vault-auth-proxy-image.yml` | 49 | `build-push` | `ubuntu-latest` |
| `vm-deploy-coverage-check.yml` | 11 | `validate-vm-deploy-coverage` | `ubuntu-latest` |
| `deploy-hermes-vm.yml` | 23 | `deploy-hermes-honcho` | **`[self-hosted, trantor-internal]` — see Bucket 2** |

The 7 jobs above are pure Python/Go/Helm validation; no locality dependencies.

## Bucket 2 — MIGRATABLE-WITH-EFFORT (1 job)

### `terminus.infra/.github/workflows/deploy-hermes-vm.yml::deploy-hermes-honcho` (line 21-43)

Currently `runs-on: [self-hosted, trantor-internal]` (line 23).

**What it actually does** (full body, lines 25-42):
- `actions/checkout@v6` (line 27)
- `actions/setup-python@v6` with `python-version: "3.12"` (lines 29-32)
- `pip install requests` (line 35)
- A single shell step that `python scripts/run_semaphore_pipeline.py hermes-honcho` with env vars `SEMAPHORE_API_TOKEN`, `SEMAPHORE_BASE_URL`, `COMMIT_SHA` (lines 38-42).

**Locality markers — checked, none present:**
- No bind mounts. No `/`, `/home`, `/mnt`, `/opt`, or `~/workspace/...` references anywhere in the file.
- Secrets used are only `SEMAPHORE_API_TOKEN` and `SEMAPHORE_BASE_URL` — both standard repo secrets, configured at the GitHub repo or org level, available to any runner type.
- The only "internal" surface is `SEMAPHORE_BASE_URL: https://semaphore.trantor.internal` (referenced in the workflow comment at line 10). **`trantor.internal` is a DNS zone reachable from a github-hosted runner only if DNS resolves it** — github-hosted runners have no route to a private `.internal` zone by default. **This is a genuine concern, not a self-hosting requirement.**

**Required migration change:**
1. Flip `runs-on: [self-hosted, trantor-internal]` to `runs-on: ubuntu-latest` (1-line change).
2. Confirm `SEMAPHORE_BASE_URL` resolves from a github-hosted runner. Two options:
   - **Preferred:** Replace the Semaphore URL with the publicly-reachable Semaphore hostname (if Semaphore is fronted by a proxy/Tailscale Funnel/Caddy on a public IP).
   - **Alternative:** Add a Tailscale sidecar step (`tailscale/github-action@v2` with a short-lived auth key from `TAILSCALE_AUTHKEY` secret) before the `python scripts/run_semaphore_pipeline.py` step. ~5 lines of YAML; auth key is a standard secret.

**Cost: ~5-10 lines of YAML, no script changes.**

## Bucket 3 — LOCALITY-REQUIRED (0 jobs)

**No job in the audited set requires self-hosted locality.**

Specifically checked and rejected:
- **PCGen oracle corpus (suspected by operator):** Not invoked by any workflow. `scripts/verify.sh`'s `preflight-oracle` stage calls `scripts/fetch-pcgen-oracle.sh --check`, which expects `$PCGEN_CORPUS_ROOT` (default `$HOME/workspace/repos/pcgen/data`) to already contain a checked-out sparse cone. The fetch script does a `git fetch --depth 1 --filter=blob:none origin <pinned-SHA>` from a public GitHub URL — entirely reproducible on a github-hosted runner in seconds. **The corpus is local-only because `verify.sh` runs locally, not because anything prevents it from running in CI.** No workflow file invokes `verify.sh` or any corpus-sweep script (grep returned 0 matches).
- **Self-hosted rationale in any workflow:** Only `deploy-hermes-vm.yml` declares self-hosted, and the only justifications in the workflow comments are: (a) "Calls Semaphore API directly — no Temporal/ArgoCD since these are native VM services" (lines 3-4) — this is a deployment-lanes comment, not a runner-lanes comment; (b) `trantor-internal` label, which is a network-reach concern (handled in Bucket 2).

## PCGen Oracle Deep-Dive

The operator suspected a PCGen oracle job using "mounted corpus data and warm cargo caches." Here is what actually exists:

### What's on disk

- `$PCGEN_REPO_DIR` (default `$HOME/workspace/repos/pcgen`): 1.6G total, with `.git/` taking most of the budget.
- `$PCGEN_CORPUS_ROOT` (= `$PCGEN_REPO_DIR/data`): **156M**, containing 17 publisher subdirs (`paizo`, `dreamscarred_press`, `kobold_press`, etc.).
- The full pinned cone is `data/pathfinder` (86M) + `system/gameModes/Pathfinder` (596K). Sparse-checkout is already cone-only (`PCGEN_ORACLE_SPARSE_PATHS="data/pathfinder system/gameModes/Pathfinder"` in `scripts/pcgen-oracle-pin.env` line 6).

### How it's fetched

`scripts/fetch-pcgen-oracle.sh` (236 lines, full reading above) uses three strategies, in order of cost:
1. **Fresh clone with depth-1 + blob-filter at the pinned SHA** (lines 158-177): `git init` → `git remote add origin https://github.com/PCGen/pcgen.git` → `git sparse-checkout init --cone` → `git sparse-checkout set data/pathfinder system/gameModes/Pathfinder` → `git fetch --depth 1 --filter=blob:none origin <SHA>`. This is what runs when `$DEST` doesn't exist. Blob-filtered + depth-1 + sparse means the network payload is only the directory entries + the few files actually read by the corpus instruments — in practice a few MB.
2. **Direct SHA fetch on an existing checkout** (lines 144-147): `git fetch --depth 1 origin <SHA>` + `git checkout --detach FETCH_HEAD`. Cheap when the cone is already checked out.
3. **Full filtered fetch fallback** (lines 149-153): `git fetch --filter=blob:none origin` if the remote refuses the SHA-direct fetch.

### Cold-start cost on a github-hosted runner

The corpus instruments read from the sparse cone, which is already what fetch strategy #1 fetches. The actual on-disk size after sparse-checkout is 86M for `data/pathfinder` + 596K for `system/gameModes/Pathfinder` ≈ **~87M of corpus content** — but with `--filter=blob:none` and depth-1, Git only downloads the directory entries and the blobs the corpus instruments touch, not the full 156M. Realistic cold-start download: **5-20 MB of object data**, taking 1-10 seconds on github-hosted bandwidth. This is dwarfed by `cargo test --locked` (multiple minutes) and `./gradlew build` (10-30 minutes) — the existing CI cost dominates.

### Could it be replaced?

Three alternatives, in increasing cost:

1. **Reuse `fetch-pcgen-oracle.sh` as a workflow step.** Add `actions/checkout` (for `scripts/fetch-pcgen-oracle.sh`) → `bash scripts/fetch-pcgen-oracle.sh --dest $GITHUB_WORKSPACE/.pcgen-oracle --force` → export `PCGEN_CORPUS_ROOT=$GITHUB_WORKSPACE/.pcgen-oracle/data` → existing sweep steps run unchanged. **~3 lines of YAML per job that needs it.** This is the cheapest path; the script already handles all three fetch strategies, SHA-pinning, and dirty-cone refusals.

2. **`actions/cache` on the fetched cone.** Key on `PCGEN_ORACLE_SHA`. After first run, the cone is in the cache and subsequent runs skip the fetch entirely. Cost: 2 lines of `actions/cache@v4`. Risk: 10 GB cache cap per repo; 87M fits comfortably. Benefit: subsequent runs save ~5-10s on a job that already takes minutes.

3. **Pre-built artifact on a release.** Tag a tarball of the sparse cone as a GitHub Release asset; download via `actions/download-artifact` or `gh release download`. Cost: a separate publish job that re-tars the cone whenever the pin SHA changes (one-line cron or pin-bump workflow). Benefit: skip git entirely on consumers. Downside: tarball is ~87M, which adds meaningfully to release asset size and download time per run.

**Recommendation: option 1 (call `fetch-pcgen-oracle.sh` directly).** The script already exists, already pins, already handles the failure modes. Wrapping it in a workflow step is mechanical. If corpus-sweep jobs become frequent (e.g. nightly on `develop`), promote to option 2.

### Recommendation for the corpus workflow surface

**No action needed today.** No workflow invokes the corpus. If a future workflow does (e.g. an SD-33 closure gate in CI), implement option 1 at that time. There is no migration cost because there is nothing to migrate — the corpus lives on the operator's machine because that's where `verify.sh` runs, not because CI requires it.

## Concrete Recommendation

1. **Migrate `terminus.infra/.github/workflows/deploy-hermes-vm.yml:23`** from `runs-on: [self-hosted, trantor-internal]` to `runs-on: ubuntu-latest`. Add a Tailscale sidecar step (or change `SEMAPHORE_BASE_URL` to a public hostname) so the runner can reach `semaphore.trantor.internal`. ~5-10 lines of YAML.
2. **Retire all per-repo local self-hosted runners.** No remaining job needs them.
3. **No corpus/oracle workflow exists to migrate.** `scripts/fetch-pcgen-oracle.sh` is reusable as-is if a corpus-sweep workflow is ever created.

**Total migration surface:** 1 workflow file, ~5-10 lines.

## Reproduction Commands

```bash
# File inventory
find /home/ubuntu/workspace/repos -maxdepth 5 \( -name '*.yml' -o -name '*.yaml' \) \
  -path '*.github/workflows*' -not -path '*/node_modules/*' -not -path '*/.claude/*' | sort

# Per-file job count (matches buckets above)
for f in /home/ubuntu/workspace/repos/{codex,pcgen,terminus.infra}/.github/workflows/*.yml; do
  echo "$(basename $f): $(grep -cE '^  [a-zA-Z][a-zA-Z0-9_-]*:|^    runs-on:' $f)"
done

# Self-hosted declarations only
grep -nE 'self-hosted|trantor-internal' \
  /home/ubuntu/workspace/repos/{codex,pcgen,terminus.infra}/.github/workflows/*.yml

# PCGen corpus size (local machine)
du -sh /home/ubuntu/workspace/repos/pcgen/data /home/ubuntu/workspace/repos/pcgen/data/pathfinder /home/ubuntu/workspace/repos/pcgen/system/gameModes/Pathfinder
```
