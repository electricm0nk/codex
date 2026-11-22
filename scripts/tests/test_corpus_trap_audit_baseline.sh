#!/usr/bin/env bash
# Detection self-test for scripts/corpus_trap_audit_baseline.py — the
# comparator behind verify.sh's `corpus-trap-audit` stage.
#
# Same lesson as corpus-sweep-selftest: an instrument whose ability to say NO
# is untested emits its PASS with identical confidence whether it is working
# or dead. This one guards a gate that must stay green over registered debt
# while still failing instantly on `wiring-class-mismatch` — the check
# AT-34-E1-008 drove to zero and that regressed silently once already.
#
# Never reads the real corpus: every case is a synthetic findings payload.
set -uo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CMP="$REPO_ROOT/scripts/corpus_trap_audit_baseline.py"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

passed=0
failed=0

# Emit a findings log with the given "kind:severity:count" triples, preceded
# by a progress line the parser must skip.
mk_log() {
    local path="$1"; shift
    printf 'building...\n' >"$path"
    python3 - "$path" "$@" <<'PYEOF' >>"$path"
import json, sys
out = []
for spec in sys.argv[2:]:
    kind, sev, n = spec.split(":")
    for i in range(int(n)):
        out.append({"trap": kind, "severity": sev,
                    "file": f"/repo/data/corpus/book_{i % 3}/ability/r{i}.json"})
print(json.dumps({"findings": out}))
PYEOF
}

BASELINE=(mod-record:DEFECT:2117 key-differs-from-name:DEFECT:650
          shared-name-distinct-records:DEFECT:249 disabled-line:DEFECT:165
          mod-record:TRAP:407)

check() {
    local name="$1" want_exit="$2" want_grep="$3" log="$4"
    local out status
    out="$(python3 "$CMP" "$log" 2>&1)"; status=$?
    if [[ "$status" == "$want_exit" ]] && grep -qE "$want_grep" <<<"$out"; then
        printf 'ok   %s\n' "$name"; passed=$((passed + 1))
    else
        printf 'FAIL %s (exit %s, wanted %s; output: %s)\n' "$name" "$status" "$want_exit" "$out"
        failed=$((failed + 1))
    fi
}

# 1. The registered baseline exactly -> PASS, and every kind is named.
mk_log "$TMP/base.log" "${BASELINE[@]}"
check 'registered baseline passes' 0 'verdict=PASS' "$TMP/base.log"
check 'reports wiring-class-mismatch=0 by name' 0 'wiring-class-mismatch=0' "$TMP/base.log"
check 'reports mod-record at its own count' 0 'mod-record=2117' "$TMP/base.log"
check 'reports key-differs-from-name at its own count' 0 'key-differs-from-name=650' "$TMP/base.log"
check 'reports shared-name-distinct-records at its own count' 0 'shared-name-distinct-records=249' "$TMP/base.log"
check 'reports disabled-line at its own count' 0 'disabled-line=165' "$TMP/base.log"
check 'reports the TRAP total separately' 0 'traps=407' "$TMP/base.log"

# 2. One wiring-class-mismatch defect -> FAIL. The regression that decayed
#    silently for 13 days is now caught on the first run after it happens.
mk_log "$TMP/wcm.log" "${BASELINE[@]}" wiring-class-mismatch:DEFECT:1
check 'one wiring-class-mismatch fails' 2 'wiring-class-mismatch=1 is NOT registered' "$TMP/wcm.log"

# 3. A registered kind drifting UP -> FAIL, naming that kind.
mk_log "$TMP/up.log" mod-record:DEFECT:2118 key-differs-from-name:DEFECT:650 \
    shared-name-distinct-records:DEFECT:249 disabled-line:DEFECT:165
check 'registered kind above its pin fails' 2 'mod-record=2118 REGRESSED' "$TMP/up.log"

# 4. A registered kind drifting DOWN -> FAIL as a stale pin, not a silent
#    green. Absorption is impossible in either direction.
mk_log "$TMP/down.log" mod-record:DEFECT:2117 key-differs-from-name:DEFECT:650 \
    shared-name-distinct-records:DEFECT:249 disabled-line:DEFECT:164
check 'registered kind below its pin fails as stale' 2 'disabled-line=164 is BELOW' "$TMP/down.log"

# 5. An unregistered kind that is not wiring-class-mismatch -> FAIL too.
mk_log "$TMP/new.log" "${BASELINE[@]}" unresolvable-citation:DEFECT:3
check 'any unregistered defect kind fails' 2 'unresolvable-citation=3 is NOT registered' "$TMP/new.log"

# 6. A TRAP-severity finding of an unregistered kind is NOT a failure: the
#    register pins DEFECTs. Traps are reported, never gated.
mk_log "$TMP/trap.log" "${BASELINE[@]}" copy-record:TRAP:12
check 'unregistered TRAP severity does not fail' 0 'traps=419' "$TMP/trap.log"

# 7. A log with no findings line -> parse error, never a vacuous PASS.
printf 'building...\nsome noise\n' >"$TMP/empty.log"
check 'missing findings line is a parse error' 1 'PARSE_ERROR=' "$TMP/empty.log"

# 8. An EMPTY findings array must NOT pass: it means the registered debt
#    vanished, i.e. the pin is stale (or the audit examined nothing).
mk_log "$TMP/zero.log"
check 'empty findings does not pass vacuously' 2 'is BELOW its pinned' "$TMP/zero.log"

printf 'passed: %d  failed: %d\n' "$passed" "$failed"
(( failed == 0 ))
