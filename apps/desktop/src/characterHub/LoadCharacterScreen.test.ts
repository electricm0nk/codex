import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { assert } from '../testSupport/asserts';
import type { PcgImportReport } from '../boundary/importPcg';
import { ImportFileChoices, PcgImportReportPanel } from './LoadCharacterScreen';

const report = (over: Partial<PcgImportReport> = {}): PcgImportReport => ({
  sourceFile: 'pf_Rogue.pcg',
  gameMode: 'Pathfinder_RPG',
  unknownKeys: [],
  unmappedClasses: [],
  unmappedRace: null,
  warnings: [],
  lines: [
    { key: 'RACE', from: 'Human', outcome: 'mapped', to: 'race:human' },
    { key: 'EQUIPNAME', from: 'Odd Hat', outcome: 'custom', to: 'custom equipment' },
    { key: 'item:longsword', from: '', outcome: 'added', to: 'a worn Longsword' },
  ],
  ...over,
});

const panel = (over: Partial<PcgImportReport> = {}) =>
  renderToStaticMarkup(createElement(PcgImportReportPanel, { report: report(over), onClose: () => {} }));

function verifiesTheImportButtonOffersBothKindsOfFile() {
  const html = renderToStaticMarkup(createElement(ImportFileChoices, { onCodexFile: () => {}, onPcgenFile: () => {} }));
  assert(html.includes('>Codex file<'), 'a Codex file');
  assert(html.includes('>PCGen file<'), 'a PCGen file');
}

function verifiesTheReportNamesTheFileAndEveryLine() {
  const html = panel();
  assert(html.includes('pf_Rogue.pcg'), 'the file');
  assert(html.includes('Pathfinder_RPG'), 'its game mode');
  assert(html.includes('1 mapped') && html.includes('1 custom') && html.includes('1 added'), 'the counts');
  for (const text of ['RACE', 'Human', 'race:human', 'Odd Hat', 'custom equipment', 'item:longsword', 'a worn Longsword']) {
    assert(html.includes(text), `the line text ${text}`);
  }
  assert(html.includes('>Close report<'), 'it can be closed');
}

function verifiesTheReportShowsWarningsAndWhatCodexDoesNotHold() {
  const html = panel({ warnings: ['Eqmod not applied'], unknownKeys: ['ZZKEY'], unmappedClasses: ['Soldier 3'], unmappedRace: 'Android' });
  assert(html.includes('Eqmod not applied'), 'a warning');
  assert(html.includes('ZZKEY'), 'an unknown key');
  assert(html.includes('Soldier 3'), 'a class Codex does not hold');
  assert(html.includes('Android'), 'a race Codex does not hold');
}

function verifiesAReportWithNothingToFlagHasNoEmptySections() {
  const html = panel();
  for (const heading of ['Warnings', 'Unknown keys', 'Classes Codex does not hold', 'Race Codex does not hold']) {
    assert(!html.includes(heading), `no ${heading} section`);
  }
}

verifiesTheImportButtonOffersBothKindsOfFile();
verifiesTheReportNamesTheFileAndEveryLine();
verifiesTheReportShowsWarningsAndWhatCodexDoesNotHold();
verifiesAReportWithNothingToFlagHasNoEmptySections();
