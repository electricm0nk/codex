#!/usr/bin/env python3
"""E6.5 planted mutations: each one must turn its test red (for the reason named), then the file
is restored byte-for-byte (sha256 checked). One cargo process at a time."""
import hashlib, os, subprocess, sys

T = '/home/ubuntu/workspace/worktrees/codex-sd37'
D = f'{T}/apps/desktop'
ENV = dict(os.environ, PATH=os.path.expanduser('~/.cargo/bin') + ':' + os.environ['PATH'],
           CARGO_TARGET_DIR='/home/ubuntu/workspace/worktrees/cargo-target/sd37-codex-sd37')
RUST = (['cargo', 'test', '--locked', '-j', '8', '--bin', 'codex-desktop', 'sf_level_up', '--', '--test-threads=8'], f'{D}/src-tauri')
FRONT = ([f'{D}/node_modules/.bin/tsx', 'src/characterHub/starfinderLevelUpModel.test.ts'], D)
LU = f'{D}/src-tauri/src/sf_level_up.rs'
AB = f'{T}/src/rules_core/pilot_compute/sf_abilities.rs'
MODEL = f'{D}/src/characterHub/starfinderLevelUpModel.ts'

PLANTS = [
    ('P1 increase step: +1 only from 19 (engine increase_term)', AB, 'if score >= 17 {', 'if score >= 19 {', RUST),
    ('P2 a held class does not advance', LU, 'Some(class) => class.level += 1,', 'Some(class) => class.level += 0,', RUST),
    ('P3 added skill ranks are dropped', LU, 'Some(a) => a.ranks = a.ranks.saturating_add(ranks),', 'Some(a) => a.ranks = a.ranks.saturating_add(0 * ranks),', RUST),
    ('P4 the increase is never due', LU, 'increase_due: INCREASE_LEVELS.contains(&new_level),', 'increase_due: INCREASE_LEVELS.contains(&0),', RUST),
    ('P5 the skill cap is not checked', LU, 'if skill.added != 0 && skill.ranks + skill.added > skill.max_ranks {', 'if skill.added != 0 && skill.ranks + skill.added > skill.max_ranks + 100 {', RUST),
    ('P6 the preview lists no change', LU, 'Ok(after_sheet) => out.changes = changes(&before_sheet, &after_sheet),', 'Ok(after_sheet) => out.changes = changes(&after_sheet, &after_sheet),', RUST),
    ('P7 the level is not saved', LU, '    SavedCharacterStore::save(&envelope, root).map_err(|err| err.message)?;\n    Ok(SfCreateResponse::Saved {', '    let _ = &envelope;\n    Ok(SfCreateResponse::Saved {', RUST),
    ('P8 rank buttons ignore the engine cap', MODEL, 'if (added < 0 || listed.ranks + added > listed.maxRanks) {', 'if (added < 0) {', FRONT),
    ('P9 change lines swap before and after', MODEL, '${formatChange(change.before, signed)} \u2192 ${formatChange(change.after, signed)}', '${formatChange(change.after, signed)} \u2192 ${formatChange(change.before, signed)}', FRONT),
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
    fails = [l for l in out.splitlines() if ' FAILED' in l or 'panicked' in l or l.startswith('Error')][:3]
    ok = r.returncode != 0 and restored
    red += ok
    print(f'{name}: exit={r.returncode} {"RED" if r.returncode else "GREEN(!)"} restored={restored}')
    for l in fails:
        print('    ' + l[:220])
print(f'{red} of {len(PLANTS)} plants red, every file restored' if red == len(PLANTS) else f'verdict=FAIL {red} of {len(PLANTS)}')
sys.exit(0 if red == len(PLANTS) else 1)
