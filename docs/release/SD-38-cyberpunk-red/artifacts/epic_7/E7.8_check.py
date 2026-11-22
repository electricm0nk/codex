#!/usr/bin/env python3
"""SD-38 E7.8: every figure in release-notes.md carries its command, and the value written equals the
value re-derived now by two independent implementations (A = shell, B = Python).

  python3 -I docs/release/SD-38-cyberpunk-red/artifacts/epic_7/E7.8_check.py --emit   # print the figure lines
  python3 -I docs/release/SD-38-cyberpunk-red/artifacts/epic_7/E7.8_check.py --plant  # check + prove it can fail

Run from the repository root. Exit 0 only if every figure matches the notes (and, with --plant, each of
three in-memory corruptions is rejected).
"""
import json, os, re, subprocess, sys

ROOT = subprocess.check_output(["git", "rev-parse", "--show-toplevel"], text=True).strip()
os.chdir(ROOT)
PKG = "docs/release/SD-38-cyberpunk-red"
NOTES = f"{PKG}/release-notes.md"
E4 = f"{PKG}/artifacts/epic_7/E7.4/logs"
HEAD = "cc9d7d9a3c"   # tranche-18 at the start of E7.8
CUT = "bb6107aee0"     # tranche-17-ui head the branch was cut from (C1's base)


def sh(cmd):
    return subprocess.check_output(["bash", "-c", cmd], text=True).strip()


def jl(p):
    return json.load(open(p, encoding="utf-8"))


def gsh(rev, path):
    return subprocess.check_output(["git", "show", f"{rev}:{path}"], text=True)


def stage_pass_tests(name):
    s = 0
    for l in open(f"{E4}/stage-{name}.log", encoding="utf-8", errors="replace"):
        m = re.match(r"test result: \w+\. (\d+) passed;", l)
        if m:
            s += int(m.group(1))
    return s


def stage_failed(name):
    return sum(1 for l in open(f"{E4}/stage-{name}.log", encoding="utf-8", errors="replace") if l.startswith("test result: FAILED"))


def kanban_counts():
    n = c = 0
    for l in gsh(HEAD, f"{PKG}/kanban.md").splitlines():
        p = l.split("|")
        if len(p) > 5 and re.match(r"^ (C|E)\d", p[1]):
            n += 1
            c += p[4].strip() == "complete"
    return f"{n} {c}"


V4 = ["root-lib", "root-full", "ingest-full", "desktop"]
Z = lambda cmd: cmd  # readability
FIG = [
 ("R-1", "build version in the two desktop manifests",
  "grep -h -m1 '\"version\"' apps/desktop/package.json apps/desktop/src-tauri/tauri.conf.json | awk -F'\"' '{print $4}' | paste -sd,",
  lambda: ",".join(jl(p)["version"] for p in ["apps/desktop/package.json", "apps/desktop/src-tauri/tauri.conf.json"]),
  "`json.load` of both files, key `version`"),
 ("R-2", f"commits on tranche-18 past the cut {CUT}, to {HEAD}",
  f"git log --oneline {CUT}..{HEAD} | awk 'END{{print NR}}'",
  lambda: sh(f"git rev-list --count {CUT}..{HEAD}"),
  "`git rev-list --count` over the same range"),
 ("R-3", f"kanban rows, then rows `complete`, at {HEAD} (before this card's own row moved)",
  f"git show {HEAD}:{PKG}/kanban.md | awk -F'|' '$2 ~ /^ (C|E)[0-9]/ {{ n++; if ($5 ~ /^ complete *$/) c++ }} END {{ print n, c }}'",
  kanban_counts,
  "split on `|`, id in column 1 and status in column 4 of the same file"),
 ("R-4", "Cyberpunk RED record kinds in MANIFEST.json, records they expect, kinds shipped print-only",
  "jq -r '[(.kinds|length), ([.kinds[].expected]|add), ([.kinds[]|select(.print_only)]|length)]|map(tostring)|join(\" \")' data/cyberpunk-red/corpus/MANIFEST.json",
  lambda: (lambda k: f"{len(k)} {sum(x['expected'] for x in k)} {sum(1 for x in k if x['print_only'])}")(jl("data/cyberpunk-red/corpus/MANIFEST.json")["kinds"]),
  "`json.load`, `len`, `sum` and a filter over `kinds`"),
 ("R-5", "tracked files under `data/cyberpunk-red`",
  "git ls-files data/cyberpunk-red | awk 'END{print NR}'",
  lambda: str(len([l for l in sh("git ls-files data/cyberpunk-red").splitlines() if l])),
  "`git ls-files` read in Python, non-empty lines"),
 ("R-6", "licence file: `open_game_content`, `operator_sign_off.signed_off`",
  "jq -r '[.license_declaration.open_game_content, .operator_sign_off.signed_off]|map(tostring)|join(\" \")' data/cyberpunk-red/corpus/LICENSE.json",
  lambda: " ".join(str(v).lower() for v in (jl("data/cyberpunk-red/corpus/LICENSE.json")["license_declaration"]["open_game_content"], jl("data/cyberpunk-red/corpus/LICENSE.json")["operator_sign_off"]["signed_off"])),
  "`json.load` of the same file"),
 ("R-7", "ui-smoke rows in `spec.json`, of them Cyberpunk rows",
  "jq -r '[(.rows|length), ([.rows[]|select(.id|test(\"cyberpunk\"))]|length)]|map(tostring)|join(\" \")' apps/desktop/scripts/ui-smoke/spec.json",
  lambda: (lambda r: f"{len(r)} {sum(1 for x in r if 'cyberpunk' in x['id'])}")(jl("apps/desktop/scripts/ui-smoke/spec.json")["rows"]),
  "`json.load`, `len(rows)` and a substring test on `id`"),
 ("R-8", "`verify.sh` stages passed, stages failed, in the E7.4 full run",
  f"awk '/PASS  /{{p++}} /FAIL  /{{f++}} END{{print p, f+0}}' {E4}/verify-sh.out",
  lambda: (lambda t: f"{len(re.findall(r'^    PASS  ', t, re.M))} {len(re.findall(r'^    FAIL  ', t, re.M))}")(open(f"{E4}/verify-sh.out", encoding="utf-8", errors="replace").read()),
  "regex `^    PASS  ` and `^    FAIL  ` over the same log"),
 ("R-9", "tests passed: root lib, root full, ingest full, desktop",
  f"for n in root-lib root-full ingest-full desktop; do awk '/^test result:/{{for(i=1;i<=NF;i++) if($i==\"passed;\") s+=$(i-1)}} END{{printf \"%s \", s+0}}' {E4}/stage-$n.log; done | awk '{{$1=$1; print}}'",
  lambda: " ".join(str(stage_pass_tests(n)) for n in V4),
  "regex `test result: \\w+\\. (\\d+) passed;` summed per log"),
 ("R-10", "`test result: FAILED` lines in the same four logs",
  f"for n in root-lib root-full ingest-full desktop; do awk '/^test result: FAILED/{{c++}} END{{printf \"%s \", c+0}}' {E4}/stage-$n.log; done | awk '{{$1=$1; print}}'",
  lambda: " ".join(str(stage_failed(n)) for n in V4),
  "`str.startswith('test result: FAILED')` per log"),
 ("R-11", "frontend test files passed",
  f"awk '/PASS +frontend-test/{{gsub(/[()]/,\"\",$3); print $3}}' {E4}/verify-sh.out",
  lambda: re.search(r"PASS +frontend-test +\((\d+/\d+) files\)", open(f"{E4}/verify-sh.out", encoding="utf-8", errors="replace").read()).group(1),
  "regex `PASS +frontend-test +\\((\\d+/\\d+) files\\)` over the same log"),
 ("R-12", "Pathfinder seed render hashes (first 8 hex), E7.4 head",
  f"awk '{{n=$2; sub(/\\.json/,\"\",n); printf \"%s%s %s\", (k++?\" \":\"\"), n, substr($1,1,8)}} END{{print \"\"}}' {PKG}/artifacts/epic_7/E7.4/pf-render/head/sha256.txt",
  lambda: " ".join(f"{l.split()[1][:-5]} {l.split()[0][:8]}" for l in open(f"{PKG}/artifacts/epic_7/E7.4/pf-render/head/sha256.txt") if l.strip()),
  "`str.split` per line of the same file"),
 ("R-13", "Pathfinder render hash pairs equal between E0.6 base and E7.4 head (diff exit status), then Starfinder seed-line hash files equal (diff exit status)",
  f"diff -q {PKG}/artifacts/epic_0/E0.6/pf-render/base/sha256.txt {PKG}/artifacts/epic_7/E7.4/pf-render/head/sha256.txt >/dev/null; a=$?; diff -q {PKG}/artifacts/epic_0/E0.6/sf-lines/base/sha256.txt {PKG}/artifacts/epic_7/E7.4/sf-lines/head/sha256.txt >/dev/null; echo $a $?",
  lambda: " ".join("0" if open(a).read() == open(b).read() else "1" for a, b in [
      (f"{PKG}/artifacts/epic_0/E0.6/pf-render/base/sha256.txt", f"{PKG}/artifacts/epic_7/E7.4/pf-render/head/sha256.txt"),
      (f"{PKG}/artifacts/epic_0/E0.6/sf-lines/base/sha256.txt", f"{PKG}/artifacts/epic_7/E7.4/sf-lines/head/sha256.txt")]),
  "byte comparison of the same file pairs in Python"),
 ("R-14", "retro events since 2026-10-13 (recorded summary), corrections, incidents",
  f"jq -r '[.events.total, .events.by_type.correction, .events.by_type.incident]|map(tostring)|join(\" \")' {PKG}/artifacts/epic_7/E7.6_logs/retro-summary.json",
  lambda: (lambda e: f"{sum(e['by_type'].values())} {e['by_type']['correction']} {e['by_type']['incident']}")(jl(f"{PKG}/artifacts/epic_7/E7.6_logs/retro-summary.json")["events"]),
  "`json.load`, `sum(by_type.values())`"),
]


def derive():
    out = {}
    for fid, what, a_cmd, b_fn, b_desc in FIG:
        a, b = sh(a_cmd), b_fn()
        out[fid] = dict(what=what, A_cmd=a_cmd, A=a, B=b, B_desc=b_desc, agree=(a == b))
    return out


def line(fid, d):
    return (f"- **{fid}** {d['what']}: **{d['A']}**. A: `{d['A_cmd']}` gives `{d['A']}`. "
            f"B (Python, independent): {d['B_desc']} gives `{d['B']}`.")


def problems(text, derived):
    bad, lines = [], {}
    for l in text.splitlines():
        m = re.match(r"^- \*\*(R-\d+)\*\*", l)
        if m:
            lines[m.group(1)] = l
    for fid, d in derived.items():
        if not d["agree"]:
            bad.append(f"{fid}: implementations disagree A={d['A']!r} B={d['B']!r}")
        l = lines.get(fid)
        if l is None:
            bad.append(f"{fid}: no figure line in release-notes.md")
            continue
        if f"**{d['A']}**" not in l:
            bad.append(f"{fid}: the line does not carry the derived value **{d['A']}**")
        if d["A_cmd"] not in l:
            bad.append(f"{fid}: the line does not quote implementation A's command")
        if d["B_desc"] not in l:
            bad.append(f"{fid}: the line does not name implementation B")
    for fid in lines:
        if fid not in derived:
            bad.append(f"{fid}: a figure line with no registered command")
    if "SD-38 INTERIM NOTES" in text:
        bad.append("interim sentinel still present")
    return bad


def main():
    derived = derive()
    if "--emit" in sys.argv:
        for fid, d in derived.items():
            print(line(fid, d))
        return 0
    text = open(NOTES, encoding="utf-8").read()
    bad = problems(text, derived)
    for b in bad:
        print("PROBLEM", b)
    if bad:
        return 1
    if "--plant" in sys.argv:
        first = next(iter(derived))
        plants = {
            "wrong value": text.replace(f"**{derived[first]['A']}**", "**999**", 1),
            "dropped command": text.replace(derived[first]["A_cmd"], "true", 1),
            "unregistered figure": text + "\n- **R-99** an invented figure: **1**.\n",
        }
        for name, t in plants.items():
            if not problems(t, derived):
                print("PLANT_NOT_REJECTED", name)
                return 1
            print("PLANT_REJECTED", name)
    print("RELEASE_NOTES_CHECK_OK", len(derived), "figures")
    return 0


sys.exit(main())
