import { characterRows, diffLines, IMPORT_CHOICES } from './charactersModel';
import type { CampaignCharacterBackup, CampaignCharacterFileDiff } from '../boundary/campaignFolder/characters';
import { assertEqual } from '../testSupport/asserts';

/**
 * SD-42 E2.10 (decisions.md §16, Q20–Q22): the campaign sheet's Characters list (the `Characters/`
 * backups), the two import choices with stand-in as the default, and the diff lines (shown, never merged).
 */

const same = (actual: unknown, expected: unknown, message: string) => assertEqual(JSON.stringify(actual), JSON.stringify(expected), message);

const backup = (over: Partial<CampaignCharacterBackup>): CampaignCharacterBackup => ({
  folder: 'aldric-char-1',
  characterId: 'char-1',
  label: 'Aldric',
  gameSystem: 'pathfinder_1e',
  classes: 'class:fighter:3',
  race: 'race:human',
  masterDevice: "Bea's laptop",
  counter: 4,
  savedAt: '2026-11-17T20:00:00Z',
  error: null,
  ...over,
});

function testRowsShowLabelMasterAndWhetherLocal() {
  const rows = characterRows([backup({}), backup({ folder: 'elowen-char-2', characterId: 'char-2', label: 'Elowen', masterDevice: null, counter: null })], ['char-2']);
  same(
    rows.map((row) => [row.folder, row.title, row.master, row.onThisDevice]),
    [
      ['aldric-char-1', 'Aldric', "Master: Bea's laptop · saved 2026-11-17", false],
      ['elowen-char-2', 'Elowen', 'No master recorded (an older copy)', true],
    ],
    'each backup shows its label, its master device and save date, and whether this device holds a copy'
  );
  assertEqual(rows[0].subtitle.includes('Human'), true, `race label in the subtitle: ${rows[0].subtitle}`);
}

function testUnreadableBackupIsListedWithWhy() {
  const rows = characterRows([backup({ folder: 'broken-x', characterId: '', label: '', error: 'envelope.txt missing' })], []);
  same([rows[0].title, rows[0].master], ['broken-x', 'Not readable: envelope.txt missing'], 'an unreadable backup is listed by folder with why, never hidden');
}

function testStandInIsTheDefaultChoice() {
  same(IMPORT_CHOICES.map((choice) => [choice.mode, choice.label, choice.isDefault]), [['StandIn', 'Play as stand-in', true], ['MoveMaster', 'Move the master here', false]], 'stand-in first and default (Q21)');
}

function testDiffLinesNameEachFileAndSide() {
  const diffs: CampaignCharacterFileDiff[] = [
    { path: 'bio.json', localBytes: 19, backupBytes: null, newer: 'Local', changedFields: [] },
    { path: 'envelope.txt', localBytes: 300, backupBytes: 305, newer: 'Backup', changedFields: ['display_label', 'saved_at'] },
    { path: 'notes.md', localBytes: null, backupBytes: 11, newer: 'Backup', changedFields: [] },
  ];
  same(
    diffLines(diffs),
    ['bio.json — only on this device', 'envelope.txt — the backup is newer; changed: display_label, saved_at', 'notes.md — only in the backup'],
    'one line per differing file; nothing is merged'
  );
  same(diffLines([]), ['No differences: this device’s copy matches the backup.'], 'an equal copy says so');
}

testRowsShowLabelMasterAndWhetherLocal();
testUnreadableBackupIsListedWithWhy();
testStandInIsTheDefaultChoice();
testDiffLinesNameEachFileAndSide();
console.log('charactersModel: 4 cases passed');
