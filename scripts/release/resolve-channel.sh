#!/usr/bin/env bash
# Map the publishing branch to its update channel. Prints `channel=` and `prerelease=` lines for
# $GITHUB_OUTPUT. Any branch other than the three governed ones fails: never default to alpha.
#   usage: resolve-channel.sh <ref_name>
set -euo pipefail
case "${1:-}" in
  main)    printf 'channel=stable\nprerelease=false\n' ;;
  test)    printf 'channel=beta\nprerelease=true\n' ;;
  develop) printf 'channel=alpha\nprerelease=true\n' ;;
  *)
    echo "resolve-channel: no publish channel for ref '${1:-}' (only main, test, develop publish)" >&2
    exit 1
    ;;
esac
