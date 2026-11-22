#!/usr/bin/env python3
"""SD-37 E6.MC planted mutations (adversarial, new shapes; the E6.2-E6.5a card plants are re-run
separately). Each plant must turn its gate red for the reason named; every file is restored
byte-for-byte (sha256 checked). One cargo process at a time.

MC1 the sheet adjusts an engine number (KAC + 1 in the layout model)    -> starfinderSheet.test.ts (E6.3)
MC2 the adapter serves KAC as EAC (engine row moved, desktop unchanged) -> Rust fixture-drift test (E6.3)
                                                                          + 160-of-160 hand values (E4.6)
MC3 the level-up dialog keeps its own increase count (4 > engine's)     -> starfinderLevelUpModel.test.ts
    (planted as: the dialog ignores the preview's count; E6.MC fix)
MC4 the creation form ignores the engine's point-buy budget              -> starfinderCreationModel.test.ts
"""
import hashlib, os, subprocess, sys

T = '/home/ubuntu/workspace/worktrees/codex-sd37'
D = f'{T}/apps/desktop'
ENV = dict(os.environ, PATH=os.path.expanduser('~/.cargo/bin') + ':' + os.environ['PATH'],
           CARGO_TARGET_DIR='/home/ubuntu/workspace/worktrees/cargo-target/sd37-codex-sd37')
TSX = f'{D}/node_modules/.bin/tsx'
SHEET_TEST = ([TSX, 'src/characterHub/starfinderSheet.test.ts'], D)
DRIFT = (['cargo', 'test', '--locked', '-j', '8', '--bin', 'codex-desktop', '--',
          '--test-threads=8', 'the_frontend_starfinder_sheet_fixtures_are_the_adapters_load_responses',
          'sf_seed_every_hand_value'], f'{D}/src-tauri')
LEVEL_TEST = ([TSX, 'src/characterHub/starfinderLevelUpModel.test.ts'], D)
CREATE_TEST = ([TSX, 'src/characterHub/starfinderCreationModel.test.ts'], D)
MODEL = f'{D}/src/characterHub/starfinderSheetModel.ts'
ADAPTER = f'{D}/src-tauri/src/sf_adapter.rs'
LEVEL = f'{D}/src/characterHub/starfinderLevelUpModel.ts'
CREATE = f'{D}/src/characterHub/starfinderCreationModel.ts'

PLANTS = [
    ('MC1 the sheet adds 1 to KAC', MODEL,
     'return { rowId: id, value: formatValue(row.value, signed), detail: row.detail };',
     "return { rowId: id, value: formatValue(row.value + (id === 'sf.kac' ? 1 : 0), signed), detail: row.detail };",
     SHEET_TEST),
    ('MC2 the adapter serves KAC as EAC', ADAPTER,
     '("sf.eac", &self.defense.eac),', '("sf.eac", &self.defense.kac),', DRIFT),
    ('MC3 the increase ignores the engine count', LEVEL,
     'if (draft.abilityIncreases.length >= preview.increaseScores) {',
     'if (draft.abilityIncreases.length >= 4) {', LEVEL_TEST),
    ('MC4 point buy ignores the engine budget', CREATE,
     'if (next < 0 || spent > rules.budget) {', 'if (next < 0 || spent > 10) {', CREATE_TEST),
]


def sha(p):
    return hashlib.sha256(open(p, 'rb').read()).hexdigest()


red = 0
for name, path, old, new, (cmd, cwd) in PLANTS:
    before = sha(path)
    text = open(path).read()
    assert text.count(old) == 1, f'{name}: anchor not unique/absent'
    open(path, 'w').write(text.replace(old, new))
    try:
        r = subprocess.run(cmd, cwd=cwd, env=ENV, capture_output=True, text=True)
    finally:
        open(path, 'w').write(text)
    restored = sha(path) == before
    out = r.stdout + r.stderr
    hits = [l for l in out.splitlines() if ' FAILED' in l or 'panicked' in l or l.startswith('Error') or 'assertion' in l][:4]
    ok = r.returncode != 0 and restored
    red += ok
    print(f'{name}: exit={r.returncode} {"RED" if r.returncode else "GREEN(!)"} restored={restored}')
    for l in hits:
        print('    ' + l[:260])
    sys.stdout.flush()
print(f'{red} of {len(PLANTS)} plants red, every file restored' if red == len(PLANTS) else f'verdict=FAIL {red} of {len(PLANTS)}')
sys.exit(0 if red == len(PLANTS) else 1)
