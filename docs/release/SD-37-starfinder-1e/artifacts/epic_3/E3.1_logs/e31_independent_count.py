"""Independent count of the in-scope SF .lst (E3.1): a Python re-implementation of the PCC
include walk over the 8 registered books, compared with the Rust test's written list."""
import os, sys
D = os.environ['PCGEN_CORPUS_ROOT']  # the pinned oracle's data/ (scripts/fetch-pcgen-oracle.sh)
BOOKS = ['paizo/core/_starfinder_core_rulebook.pcc', 'paizo/armory/_starfinder_armory.pcc',
         'paizo/character_operations_manual/_character_operations_manual.pcc',
         'paizo/pact_worlds/_starfinder_pact_worlds.pcc', 'paizo/near_space/_near_space.pcc',
         'paizo/alien_archive/_starfinder_alien_archive.pcc',
         'paizo/alien_archive_2/_starfinder_alien_archive_2.pcc',
         'paizo/alien_archive_3/_starfinder_alien_archive_3.pcc']
def res(src, t):
    t = t.split('|')[0].strip().replace('\\', '/')
    t = t[1:] if t.startswith('@') else t
    if t.startswith('*/'): return os.path.normpath(os.path.join(D, t[2:]))
    if t.startswith('/'): return os.path.normpath(os.path.join(D, t.lstrip('/')))
    return os.path.normpath(os.path.join(os.path.dirname(src), t))
out = set()
for b in BOOKS:
    pcc = os.path.join(D, 'starfinder', b)
    for line in open(pcc, encoding='utf-8', errors='replace'):
        c = line.strip()
        if not c or c.startswith('#') or ':' not in c: continue
        k, r = c.split(':', 1)
        if k == 'PCC' or '.lst' not in r.split('|')[0].lower(): continue
        p = res(pcc, r)
        assert os.path.isfile(p), p
        rel = os.path.relpath(p, D)
        if rel.startswith('starfinder/'): out.add(rel)
print('python in-scope SF .lst:', len(out))
rust = set(open(sys.argv[1]).read().split())
print('rust list:', len(rust), 'symmetric difference:', sorted(out ^ rust))
