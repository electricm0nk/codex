#!/usr/bin/env python3
"""SD-37 E7.6: the registry of every figure in release-notes.md.

Each figure has implementation A (a shell command, quoted verbatim in the notes) and
implementation B (Python, independent of A). `derive()` runs both and returns the value only
when they agree. Run from anywhere; the repo root is the tree this file lives in.
Usage: python3 E7.6_figures.py            # print every figure, A and B
"""
import json, os, re, subprocess, sys, collections

ROOT = subprocess.check_output(["git", "-C", os.path.dirname(os.path.abspath(__file__)), "rev-parse", "--show-toplevel"], text=True).strip()
PKG = "docs/release/SD-37-starfinder-1e"
BASE = "1335bb7248"   # the tranche/17 commit this card started from (kanban/commit figures are pinned to it)
CUT = "20bf84a3b2"    # origin/develop at the branch cut (decisions.md section 2)
LOGS = PKG + "/artifacts/epic_7/E7.2_logs"


def sh(cmd):
    return subprocess.check_output(["bash", "-c", cmd], cwd=ROOT, text=True).strip()


def rd(path):
    return open(os.path.join(ROOT, path), encoding="utf-8").read()


def rdb(path):
    return open(os.path.join(ROOT, path), "rb").read()


def git_show(rev, path):
    return subprocess.check_output(["git", "-C", ROOT, "show", f"{rev}:{path}"], text=True)


def walk(rel):
    for r, _d, f in os.walk(os.path.join(ROOT, rel)):
        for x in f:
            yield os.path.join(r, x)


def passed_sum(log):
    return sum(int(m) for m in re.findall(r"^test result: \w+\. (\d+) passed;", rd(log), re.M))


def failed_lines(log):
    return len(re.findall(r"^test result: FAILED", rd(log), re.M))


def b_version():
    return ",".join(json.load(open(os.path.join(ROOT, p)))["version"] for p in ("apps/desktop/package.json", "apps/desktop/src-tauri/tauri.conf.json"))


def b_commits():
    return subprocess.check_output(["git", "-C", ROOT, "rev-list", "--count", f"{CUT}..{BASE}"], text=True).strip()


def b_kanban():
    rows = [l for l in git_show(BASE, PKG + "/kanban.md").splitlines() if re.match(r"^\| (C|E)[0-9]", l)]
    comp = [l for l in rows if l.split("|")[4].strip() == "complete"]
    return f"{len(rows)} {len(comp)}"


def b_units():
    j = json.load(open(os.path.join(ROOT, "docs/work-inventory.starfinder-1e.json")))
    return f"{len(j['units'])} {j['totals']['rows_in_scope']} {j['totals']['rows_excluded']}"


def b_report():
    j = json.load(open(os.path.join(ROOT, "data/starfinder-1e/sheet_rules/_report.json")))
    return f"{j['records']} {j['converted']} {j['refused']} {j['degraded_records']}"


def b_pf_units():
    return str(len(json.load(open(os.path.join(ROOT, "docs/work-inventory.json")))["units"]))


def b_pf_hash():
    t = rd(LOGS + "/pf_hash.log")
    a = re.search(r"pf-seed-render aldric sha256=([0-9a-f]{8})", t).group(1)
    e = re.search(r"pf-seed-render elowen sha256=([0-9a-f]{8})", t).group(1)
    return f"aldric {a} elowen {e}"


def b_stages():
    t = rd(LOGS + "/verify-full.out")
    return str(len(re.findall(r"^    PASS ", t, re.M)))


def b_passes():
    return " ".join(str(passed_sum(f"{LOGS}/{n}.log")) for n in ("root-lib", "root-full", "ingest-full", "desktop"))


def b_failed():
    return " ".join(str(failed_lines(f"{LOGS}/{n}.log")) for n in ("root-lib", "root-full", "ingest-full", "desktop"))


def b_frontend():
    return re.search(r"PASS\s+frontend-test\s+\((\d+/\d+) files\)", rd(LOGS + "/verify-full.out")).group(1)


def b_seed160():
    n = 0
    for f in ("artifacts/epic_0/seed-hand-values.md", "artifacts/epic_4/E4.2-initiative-hand-values.md",
              "artifacts/epic_4/E4.4-spell-dc-hand-values.md", "artifacts/epic_4/E4.5-loadout-hand-values.md"):
        for l in rd(f"{PKG}/{f}").splitlines():
            c = l.split("|")
            if l.startswith("|") and len(c) > 5 and c[1].startswith(" SF-") and "https:" in c[4]:
                n += 1
    return str(n)


def b_parity():
    def rows(p):
        return sum(1 for l in rd(p).splitlines() if l.strip() and not l.startswith("#"))
    obs = len([f for f in os.listdir(os.path.join(ROOT, "scripts/oracle_harness/sf_parity")) if f.endswith(".oracle.txt")])
    return f"{obs} {rows('scripts/oracle_harness/sf_parity/explained.tsv')} {rows('scripts/oracle_harness/sf_parity/not_in_oracle.tsv')}"


def b_smoke():
    rows = json.load(open(os.path.join(ROOT, "apps/desktop/scripts/ui-smoke/spec.json")))["rows"]
    sf = [r for r in rows if "starfinder" in r["id"]]
    rs = json.load(open(os.path.join(ROOT, PKG + "/artifacts/epic_6/E6.6_logs/smoke_final/real_store.json")))
    return f"{len(rows)} {len(sf)} {rs['before']['entries']} {rs['after']['entries']} {str(rs['before']['sha256'] == rs['after']['sha256']).lower()}"


def b_rules_tables():
    n = sum(1 for p in walk("data/rules_tables") if p.endswith(".json"))
    lst = sum(l.count(b".lst") > 0 for p in walk("data/rules_tables") for l in open(p, "rb"))
    return f"{n} {lst} {'absent' if not os.path.isdir(os.path.join(ROOT, 'src/rules_core/rules_tables')) else 'present'}"


def b_licence():
    rows = [l for l in rd("docs/governance/license-matrix.md").splitlines() if l.startswith("| `starfinder/")]
    inc = sum("**include**" in l.split("|")[-3] for l in rows)
    exc = sum("**exclude**" in l.split("|")[-3] for l in rows)
    off = sum(l.split("|")[-2].strip() == "false" for l in rows)
    return f"{len(rows)} {inc} {exc} {off}"


def b_pi_terms():
    t = rd("src/rules_core/pi_screening.rs")
    body = t.split("pub const SF_PI_TERMS: &[&str] = &[", 1)[1].split("];", 1)[0]
    body = "\n".join(l for l in body.splitlines() if not l.strip().startswith("//"))
    return str(len(re.findall(r'"([^"]+)"', body)))


def b_corpus():
    c = collections.Counter()
    for p in walk("data/starfinder-1e/corpus"):
        if p.endswith(".json"):
            j = json.loads(rdb(p))
            if isinstance(j, dict) and "license" in j:
                c[j["license"]] += 1
    return f"{sum(c.values())} {c['PI-REDACTED']}"


def b_sf_files():
    out = subprocess.check_output(["git", "-C", ROOT, "ls-files", "data/starfinder-1e"], text=True)
    return str(len([l for l in out.splitlines() if l]))


def b_retro():
    j = json.load(open(os.path.join(ROOT, PKG + "/artifacts/epic_7/E7.4_logs/retro-summary.json")))
    bt = j["events"]["by_type"]
    return f"{sum(bt.values())} {bt['correction']} {bt['incident']}"


RS = PKG + "/artifacts/epic_7/E7.4_logs/retro-summary.json"
# (id, value label, A shell command, B description, B function)
FIGS = [
    ("R-1", "build version in the two desktop manifests",
     "grep -h -m1 '\"version\"' apps/desktop/package.json apps/desktop/src-tauri/tauri.conf.json | awk -F'\"' '{print $4}' | paste -sd,",
     "`json.load` of both files, key `version`", b_version),
    ("R-2", f"commits on tranche/17 past the cut {CUT}, to {BASE}",
     f"git log --oneline {CUT}..{BASE} | awk 'END{{print NR}}'",
     "`git rev-list --count` over the same range", b_commits),
    ("R-3", f"kanban rows, then rows `complete`, at {BASE} (before this card's own row moved)",
     f"git show {BASE}:{PKG}/kanban.md | awk -F'|' '$2 ~ /^ (C|E)[0-9]/ {{ n++; if ($5 ~ /^ complete *$/) c++ }} END {{ print n, c }}'",
     "regex over the same file, status = column 4 after the id", b_kanban),
    ("R-4", "Starfinder work units, in-scope LST rows, excluded rows (CRB + 7 books)",
     "jq -r '[(.units|length), .totals.rows_in_scope, .totals.rows_excluded]|map(tostring)|join(\" \")' docs/work-inventory.starfinder-1e.json",
     "`len(units)` and `totals` via `json.load`", b_units),
    ("R-5", "converter report: records, converted, refused, degraded records",
     "jq -r '[.records,.converted,.refused,.degraded_records]|map(tostring)|join(\" \")' data/starfinder-1e/sheet_rules/_report.json",
     "`json.load` of the same report", b_report),
    ("R-6", "Pathfinder work-inventory units (the PF side, unmoved)",
     "jq '.units|length' docs/work-inventory.json",
     "`len(json.load(...)['units'])`", b_pf_units),
    ("R-7", "Pathfinder seed render hashes (first 8 hex), last run in E7.2",
     f"awk '/^pf-seed-render (aldric|elowen) /{{printf \"%s%s %s\", (n++?\" \":\"\"), $2, substr($3,8,8)}} END{{print \"\"}}' {LOGS}/pf_hash.log",
     "regex over the same log", b_pf_hash),
    ("R-8", "`verify.sh` stages passed in the full run (E7.2)",
     f"awk '/^    PASS/{{n++}} END{{print n}}' {LOGS}/verify-full.out",
     "regex `^    PASS ` over the same log", b_stages),
    ("R-9", "tests passed: root lib, root full, ingest full, desktop",
     "for n in root-lib root-full ingest-full desktop; do awk '/^test result:/{for(i=1;i<=NF;i++) if($i==\"passed;\") s+=$(i-1)} END{printf \"%s \", s+0}' " + LOGS + "/$n.log; done | awk '{$1=$1; print}'",
     "regex `test result: \\w+\\. (\\d+) passed;` summed per log", b_passes),
    ("R-10", "`test result: FAILED` lines in the same four logs",
     "for n in root-lib root-full ingest-full desktop; do awk '/^test result: FAILED/{c++} END{printf \"%s \", c+0}' " + LOGS + "/$n.log; done | awk '{$1=$1; print}'",
     "regex `^test result: FAILED` per log", b_failed),
    ("R-11", "frontend test files passed",
     f"awk '/PASS +frontend-test/{{gsub(/[()]/,\"\",$3); print $3}}' {LOGS}/verify-full.out",
     "regex over the same log", b_frontend),
    ("R-12", "Starfinder seed hand values (4 seeds), each with an SRD URL",
     "cd " + PKG + "/artifacts && awk -F'|' '/^\\|/ && $2 ~ /^ SF-/ && $5 ~ /https:/' epic_0/seed-hand-values.md epic_4/E4.2-initiative-hand-values.md epic_4/E4.4-spell-dc-hand-values.md epic_4/E4.5-loadout-hand-values.md | awk 'END{print NR}'",
     "line split on `|`, same predicate", b_seed160),
    ("R-13", "oracle parity: PCGen observation files, explained differences, engine fields the oracle exports no value for",
     "cd scripts/oracle_harness/sf_parity && echo $(ls *.oracle.txt | awk 'END{print NR}') $(awk '!/^#/ && NF' explained.tsv | awk 'END{print NR}') $(awk '!/^#/ && NF' not_in_oracle.tsv | awk 'END{print NR}')",
     "`os.listdir` and non-comment, non-blank line counts", b_parity),
    ("R-14", "ui-smoke rows in `spec.json`, of them Starfinder rows, then the real store's entry count before and after, then whether its sha256 is equal",
     "jq -r --slurpfile s " + PKG + "/artifacts/epic_6/E6.6_logs/smoke_final/real_store.json '[(.rows|length), ([.rows[]|select(.id|test(\"starfinder\"))]|length), $s[0].before.entries, $s[0].after.entries, ($s[0].before.sha256==$s[0].after.sha256)]|map(tostring)|join(\" \")' apps/desktop/scripts/ui-smoke/spec.json",
     "`json.load` of both files", b_smoke),
    ("R-15", "`data/rules_tables` JSON files, files carrying the text `.lst`, and the Rust module `src/rules_core/rules_tables`",
     "echo $(find data/rules_tables -name '*.json' | awk 'END{print NR}') $(grep -rlF '.lst' data/rules_tables | awk 'END{print NR}') $(test -d src/rules_core/rules_tables && echo present || echo absent)",
     "`os.walk`; per-line byte search for `.lst`; `os.path.isdir`", b_rules_tables),
    ("R-16", "Starfinder licence-matrix rows, `include`, `exclude`, `operator_sign_off` false",
     "awk -F'|' '/^\\| `starfinder\\//{n++; if($(NF-2) ~ /include/) i++; if($(NF-2) ~ /exclude/) x++; if($(NF-1) ~ /^ *false *$/) f++} END{print n, i, x, f}' docs/governance/license-matrix.md",
     "split on `|`, same columns", b_licence),
    ("R-17", "Starfinder product-identity screen terms (`SF_PI_TERMS`)",
     "awk '/^pub const SF_PI_TERMS/{s=1;next} s && /^\\];/{exit} s && !/^ *\\/\\//{n+=gsub(/\"[^\"]+\"/,\"&\")} END{print n}' src/rules_core/pi_screening.rs",
     "regex `\"([^\"]+)\"` over the array body minus comment lines", b_pi_terms),
    ("R-18", "Starfinder corpus records (files carrying a `license` field), of them `PI-REDACTED`",
     "echo $(grep -rlE '\"license\": \"(OGL|PI-REDACTED)\"' data/starfinder-1e/corpus | awk 'END{print NR}') $(grep -rl '\"license\": \"PI-REDACTED\"' data/starfinder-1e/corpus | awk 'END{print NR}')",
     "`os.walk` + `json.loads`, top-level `license` key", b_corpus),
    ("R-19", "tracked files under `data/starfinder-1e`",
     "git ls-files data/starfinder-1e | awk 'END{print NR}'",
     "`git ls-files` read in Python, non-empty lines", b_sf_files),
    ("R-20", "retro events since 2026-10-02 (recorded summary), corrections, incidents",
     "jq -r '[(.events.by_type|add), .events.by_type.correction, .events.by_type.incident]|map(tostring)|join(\" \")' " + RS,
     "`json.load`, `sum(by_type.values())`", b_retro),
]


def derive():
    out = {}
    for fid, label, a, bdesc, bfn in FIGS:
        try:
            av = sh(a)
        except Exception as e:  # a failing command is a disagreement, not a crash
            av = f"A-ERROR {e}"
        try:
            bv = bfn()
        except Exception as e:
            bv = f"B-ERROR {e}"
        out[fid] = {"label": label, "A_cmd": a, "A": av, "B_desc": bdesc, "B": bv, "agree": av == bv}
    return out


if __name__ == "__main__":
    d = derive()
    bad = 0
    for k, v in d.items():
        print(f"{k}\t{v['A']}\t{v['B']}\t{'AGREE' if v['agree'] else 'DISAGREE'}\t{v['label']}")
        bad += not v["agree"]
    print(f"FIGURES {len(d)} DISAGREE {bad}")
    sys.exit(1 if bad else 0)
