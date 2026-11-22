"""SD-37 E0.2: per-PCC licence evidence for the SF corpus at the pinned oracle.
Usage: python3 sf_licence_evidence.py  (reads $PCGEN_CORPUS_ROOT or ~/workspace/repos/pcgen/data)"""
import os, re
root = os.environ.get("PCGEN_CORPUS_ROOT", os.path.expanduser("~/workspace/repos/pcgen/data"))
S = os.path.join(root, "starfinder")
PRED = re.compile(r"^[^#\s]")
SRC = ("SOURCELONG", "SOURCESHORT", "SOURCEWEB", "SOURCEDATE")
DIRS = ["paizo/core", "paizo/core/_society", "paizo/armory", "paizo/character_operations_manual",
        "paizo/pact_worlds", "paizo/near_space", "paizo/alien_archive", "paizo/alien_archive_2",
        "paizo/alien_archive_3", "paizo/starfinder_society_rules", "lpj_design/infinite_space"]
def rows(d):
    n = 0
    for r, ds, fs in os.walk(d):
        if d.endswith("paizo/core") and "_society" in ds: ds.remove("_society")
        for f in fs:
            if f.endswith(".lst"):
                for line in open(os.path.join(r, f), encoding="latin-1"):
                    if PRED.match(line) and not line.startswith(SRC): n += 1
    return n
print("dir\tpcc\tPUBNAMELONG\tactive_COPYRIGHT_lines\tOGL1.0a_in_COPYRIGHT\tISOGL\tEXTRAFILE\tOGL.txt\tOGL.txt_lines\tAll_Rights_Reserved\tCommunity_Use_INFOTEXT\tPI_declaration_in_OGL.txt\tdata_rows")
total = 0
for d in DIRS:
    full = os.path.join(S, d)
    pccs = []
    for r, ds, fs in os.walk(full):
        if d == "paizo/core" and "_society" in ds: ds.remove("_society")
        pccs += [os.path.join(r, f) for f in fs if f.endswith(".pcc")]
    assert len(pccs) == 1, (d, pccs)
    p = pccs[0]
    lines = open(p, encoding="latin-1").read().splitlines()
    pub = next((l.split(":", 1)[1] for l in lines if l.startswith("PUBNAMELONG:")), "-")
    cr = [l for l in lines if l.startswith("COPYRIGHT:")]
    ogl_cr = any("Open Game License" in l for l in cr)
    isogl = next((l for l in lines if l.lstrip("#").startswith("ISOGL")), "-")
    extra = ",".join(l for l in lines if "EXTRAFILE" in l) or "-"
    ogl = os.path.join(os.path.dirname(p), "OGL.txt")
    has = os.path.isfile(ogl)
    ogl_txt = open(ogl, encoding="latin-1").read() if has else ""
    arr = any("All Rights Reserved" in l for l in cr)
    cu = any(l.startswith("INFOTEXT:") and "Community Use" in l for l in lines)
    pid = "hereby identified as Product Identity" in ogl_txt.replace("\n", " ")
    n = rows(full); total += n
    print("\t".join(map(str, [d, os.path.relpath(p, S), pub, len(cr), ogl_cr, isogl, extra, has,
                              ogl_txt.count("\n") if has else 0, arr, cu, pid, n])))
print("TOTAL_ROWS", total)
