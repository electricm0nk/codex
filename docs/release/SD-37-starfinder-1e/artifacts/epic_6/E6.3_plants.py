#!/usr/bin/env python3
"""E6.3 planted mutations: each must turn the E6.3 proof RED, then every file is restored.

P1 a hand-kept number: the Strength score cell prints a constant instead of its row.
P2 a Pathfinder tile: the sheet writes 'Flat-Footed' itself. (A layout row naming an id the engine
   does not send, e.g. `sf.cmd`, renders nothing: a row with no engine value is never laid out.)
P3 a dropped section: the skills section is not laid out (its rows fall to 'Other totals', unsigned).
P4 a number outside the rows: a label carries a literal digit.
P5 the Starfinder route removed: isStarfinderCharacter answers false (a Starfinder save opens the Pathfinder sheet).
P6 a stale fixture: one fixture row's value moved; the Rust drift test must fail.
P7 the engine row removed: SfSheet::explanations drops the ability-score rows.

Usage: python3 E6.3_plants.py   (from anywhere; needs CARGO_TARGET_DIR exported for P6/P7)
"""
import hashlib
import os
import subprocess
import sys

T = '/home/ubuntu/workspace/worktrees/codex-sd37'
APP = f'{T}/apps/desktop'
MODEL = f'{APP}/src/characterHub/starfinderSheetModel.ts'
SHEET = f'{APP}/src/characterHub/StarfinderCharacterSheet.tsx'
FIXTURE = f'{APP}/src/characterHub/__tests__/starfinderSheetFixtures/SF-Envoy-3.json'  # moved by E6.5 (wired-integration audit test-path exclusion)
ADAPTER = f'{APP}/src-tauri/src/sf_adapter.rs'
TSX = f'{APP}/node_modules/.bin/tsx'
FRONTEND = [TSX, 'src/characterHub/starfinderSheet.test.ts']
RUST_FIXTURE = ['cargo', 'test', '--locked', '-j', '8', '--bin', 'codex-desktop',
                'the_frontend_starfinder_sheet_fixtures_are_the_adapters_load_responses', '--', '--test-threads=8']
RUST_ROWS = ['cargo', 'test', '--locked', '-j', '8', '--bin', 'codex-desktop',
             'every_seed_carries_its_ability_score_and_class_level_rows', '--', '--test-threads=8']


def sha(path):
    return hashlib.sha256(open(path, 'rb').read()).hexdigest()


def plant(name, path, old, new, cmd, cwd):
    before = open(path).read()
    digest = sha(path)
    assert before.count(old) == 1, f'{name}: anchor not unique in {path}'
    open(path, 'w').write(before.replace(old, new))
    try:
        r = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True)
    finally:
        open(path, 'w').write(before)
    restored = sha(path) == digest
    tail = (r.stdout + r.stderr).strip().splitlines()
    reason = next((l for l in reversed(tail) if 'Error' in l or 'panicked' in l or 'FAILED' in l or 'assertion' in l), tail[-1] if tail else '')
    status = 'FAIL' if r.returncode != 0 else 'PASS'
    print(f'{name}: {status} (exit {r.returncode}) restored={restored} :: {reason.strip()[:300]}')
    return status == 'FAIL' and restored


results = [
    plant('P1 hand-kept score', MODEL,
          "cells: [cell(`sf.ability_score.${ability.key}`, false), cell(`sf.ability_modifier.${ability.key}`, true)],",
          "cells: [{ ...(cell(`sf.ability_score.${ability.key}`, false) as StarfinderSheetCell), value: '16' }, cell(`sf.ability_modifier.${ability.key}`, true)],",
          FRONTEND, APP),
    plant('P2 Pathfinder Flat-Footed tile', SHEET,
          "      <RulesAndFeaturesSection lines={sheet.sheetLines}",
          "      <span>Flat-Footed</span>\n      <RulesAndFeaturesSection lines={sheet.sheetLines}",
          FRONTEND, APP),
    plant('P3 skills dropped', MODEL,
          "rows: idsWithPrefix('sf.skill.').map(",
          "rows: idsWithPrefix('sf.skill.').filter(() => false).map(",
          FRONTEND, APP),
    plant('P4 literal digit in a label', MODEL,
          "{ id: 'sf.resolve', label: 'Resolve Points', signed: false },",
          "{ id: 'sf.resolve', label: 'Resolve Points (max 20)', signed: false },",
          FRONTEND, APP),
    plant('P5 route removed', MODEL,
          "  return summary.gameSystem === STARFINDER_GAME_SYSTEM;",
          "  return summary.gameSystem === 'starfinder';",
          FRONTEND, APP),
    plant('P6 stale fixture', FIXTURE,
          '"id": "sf.eac",\n      "value": ',
          '"id": "sf.eac",\n      "value": 1',
          RUST_FIXTURE, f'{APP}/src-tauri'),
    plant('P7 engine score rows removed', ADAPTER,
          '                id: format!("sf.ability_score.{name}"),',
          '                id: format!("sf.ability_score_dropped.{name}"),',
          RUST_ROWS, f'{APP}/src-tauri'),
]
ok = all(results)
print(f'verdict={"PASS" if ok else "FAIL"} red={sum(results)} of {len(results)}')
sys.exit(0 if ok else 1)
