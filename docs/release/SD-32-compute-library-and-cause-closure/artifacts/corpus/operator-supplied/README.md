# operator-supplied/ — the PCGen oracle slot (git-ignored)

This directory holds the PCGen oracle checkout that every SD-32 census, shape, and engine cycle
reads. **Everything under it except this README is git-ignored** by the repo rule
`docs/release/SD-*/artifacts/corpus/operator-supplied/**` — the oracle is a public repository
pinned by SHA (`scripts/pcgen-oracle-pin.env`), never vendored into this repo.

## Populate

```bash
cd "$(git rev-parse --show-toplevel)"
export PCGEN_REPO_DIR="$PWD/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen"
export PCGEN_CORPUS_ROOT="$PCGEN_REPO_DIR/data"
scripts/fetch-pcgen-oracle.sh --dest "$PCGEN_REPO_DIR"      # sparse cone at PCGEN_ORACLE_SHA, ~86 MB
scripts/verify.sh --only preflight-oracle                    # must PASS before any cycle
```

## Check

```bash
scripts/fetch-pcgen-oracle.sh --dest "$PCGEN_REPO_DIR" --check   # "pcgen-oracle: OK <sha> <path>"
grep PCGEN_ORACLE_SHA scripts/pcgen-oracle-pin.env              # the SHA every receipt cites
```

## Rules

- Never point `PCGEN_REPO_DIR` / `PCGEN_CORPUS_ROOT` at `~/workspace/repos/pcgen` or any path
  outside the repo while working this bundle.
- Never commit anything from `pcgen/` (the ignore rule enforces this; `git status --porcelain |
  grep operator-supplied` must print nothing).
- Never edit files inside `pcgen/`; a dirty tracked file makes `--check` fail by design.
- Bundle-produced files (fixtures, expected values, ledgers) go to the per-gate directories, not here.
