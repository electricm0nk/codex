#!/usr/bin/env bash
#
# pcgen-run-character.sh — Criterion 4.1 (SD-25 Epic 4: PCGen Runner Scaffolding)
#
# Drives the real PCGen Gradle wrapper (headless batch-export mode) to run a
# PCGen-native character file (.pcg) through the actual PCGen engine end to
# end, producing raw XML output for scripts/pcgen-normalize-output.py
# (criterion 4.2) to normalize into the golden-fixture comparison shape used
# by src/oracle_validation/{golden_fixture,selected_parity_dimensions}.rs.
#
# This invokes PCGen's real `pcgen.system.Main` batch-export path via
# `./gradlew run --args=...` — the same route documented as
# `legacy_route=headless Gradle run batch export via code/testsuite/base-xml.ftl`
# in tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt
# and demonstrated by PCGen's own
# code/src/slowtest/pcgen/inttest/PcgenFtlTestCase.java (see PCGen's own
# AGENTS.md "Local Run Examples" section for the equivalent documented
# invocation). No PCGen output is mocked, stubbed, or fabricated here: every
# invocation launches the real PCGen jar (via Gradle) against a real .pcg
# character file and captures its real stdout-reported success/failure and
# real exported XML.
#
# Usage:
#   pcgen-run-character.sh -c <character.pcg> [-o <output.xml>] [-e <exportsheet>] [-w <pcgen-repo-dir>]
#
# See -h for full option/environment-variable documentation.

set -euo pipefail

PROG_NAME="$(basename "$0")"

# HOME-relative on purpose: the operator keeps `workspace/` in the home
# directory and syncs it between machines, so this default is correct on every
# box. `$HOME` rather than `~` because `~` does not expand inside quotes.
DEFAULT_PCGEN_DIR="${HOME}/workspace/repos/pcgen"
DEFAULT_EXPORT_SHEET_REL="code/testsuite/base-xml.ftl"

usage() {
    cat <<EOF
Usage: ${PROG_NAME} -c <character.pcg> [-o <output.xml>] [-e <exportsheet>] [-w <pcgen-repo-dir>]

Drives the PCGen Gradle wrapper in headless batch-export mode to run a
PCGen-native character (.pcg) file through the real PCGen engine end to end,
producing raw XML output for scripts/pcgen-normalize-output.py.

Options:
  -c <file>   Path to a PCGen-native character file (.pcg). Required.
  -o <file>   Output XML path. Default: a file under a fresh scratch
              directory (see PCGEN_RUN_OUTPUT_DIR), named after the
              character file's basename.
  -e <file>   Export sheet template. Relative paths are resolved against
              the PCGen repo root. Default: code/testsuite/base-xml.ftl
              (the FreeMarker template PCGen's own character integration
              tests use, and this bundle's documented legacy_route for
              oracle-evidence capture).
  -w <dir>    PCGen repo root (must contain an executable ./gradlew).
              Default: \$PCGEN_REPO_DIR or ${DEFAULT_PCGEN_DIR}.
  -h          Show this help and exit.

Environment:
  PCGEN_REPO_DIR       Same as -w.
  PCGEN_RUN_OUTPUT_DIR Directory for generated XML output when -o is not
                        given. Default: a fresh mktemp -d directory.

Exit codes:
  0  PCGen ran successfully and produced the output file.
  1  Usage error (missing/invalid arguments, or referenced file/dir absent).
  2  The PCGen Gradle run itself failed (non-zero exit from ./gradlew) or
     reported success without producing output.

Notes:
  PCGen's batch-export CLI (pcgen.system.Main / CommandLineArguments) takes
  a PCGen-native .pcg character file via -c/--character, not an arbitrary
  text format. This script drives that real CLI faithfully; it does not
  translate any other character-input format into .pcg.
EOF
}

character_file=""
output_file=""
export_sheet=""
pcgen_dir="${PCGEN_REPO_DIR:-${DEFAULT_PCGEN_DIR}}"

while getopts ":c:o:e:w:h" opt; do
    case "${opt}" in
        c) character_file="${OPTARG}" ;;
        o) output_file="${OPTARG}" ;;
        e) export_sheet="${OPTARG}" ;;
        w) pcgen_dir="${OPTARG}" ;;
        h)
            usage
            exit 0
            ;;
        \?)
            echo "${PROG_NAME}: unknown option -${OPTARG}" >&2
            usage >&2
            exit 1
            ;;
        :)
            echo "${PROG_NAME}: option -${OPTARG} requires an argument" >&2
            usage >&2
            exit 1
            ;;
    esac
done

if [[ -z "${character_file}" ]]; then
    echo "${PROG_NAME}: -c <character.pcg> is required" >&2
    usage >&2
    exit 1
fi

if [[ ! -f "${character_file}" ]]; then
    echo "${PROG_NAME}: character file not found: ${character_file}" >&2
    exit 1
fi
character_file="$(cd "$(dirname "${character_file}")" && pwd)/$(basename "${character_file}")"

gradlew_path="${pcgen_dir}/gradlew"
if [[ ! -x "${gradlew_path}" ]]; then
    echo "${PROG_NAME}: PCGen Gradle wrapper not found or not executable: ${gradlew_path}" >&2
    echo "${PROG_NAME}: set -w or \$PCGEN_REPO_DIR to the PCGen repo checkout root" >&2
    exit 1
fi

if [[ -z "${export_sheet}" ]]; then
    export_sheet="${pcgen_dir}/${DEFAULT_EXPORT_SHEET_REL}"
elif [[ "${export_sheet}" != /* ]]; then
    export_sheet="${pcgen_dir}/${export_sheet}"
fi
if [[ ! -f "${export_sheet}" ]]; then
    echo "${PROG_NAME}: export sheet template not found: ${export_sheet}" >&2
    exit 1
fi

if [[ -z "${output_file}" ]]; then
    output_dir="${PCGEN_RUN_OUTPUT_DIR:-$(mktemp -d -t pcgen-run-character.XXXXXX)}"
    mkdir -p "${output_dir}"
    output_file="${output_dir}/$(basename "${character_file}" .pcg).xml"
else
    mkdir -p "$(dirname "${output_file}")"
fi
# PCGen's batch exporter can refuse to overwrite an existing output file
# (CommandLineArguments requires -o to be either non-existent-and-creatable
# or an existing writable file); a stale file from a prior run should not
# make a fresh run's success ambiguous.
rm -f "${output_file}"

settings_dir="$(mktemp -d -t pcgen-run-character-settings.XXXXXX)"
cleanup() {
    rm -rf "${settings_dir}"
}
trap cleanup EXIT

echo "${PROG_NAME}: running PCGen (gradlew run) against ${character_file}" >&2
echo "${PROG_NAME}: export sheet: ${export_sheet}" >&2
echo "${PROG_NAME}: output file:  ${output_file}" >&2

gradle_args="--character \"${character_file}\" --exportsheet \"${export_sheet}\" --outputfile \"${output_file}\" --settingsdir \"${settings_dir}\""

if ! ( cd "${pcgen_dir}" && ./gradlew run --console=plain --args="${gradle_args}" ); then
    echo "${PROG_NAME}: PCGen Gradle run failed" >&2
    exit 2
fi

if [[ ! -s "${output_file}" ]]; then
    echo "${PROG_NAME}: PCGen reported success but produced no output at ${output_file}" >&2
    exit 2
fi

echo "${output_file}"
