import { importPcg, PCG_FILE_FILTER, pcgReportCounts, type PcgImportReport } from './importPcg';
import { assertEqual } from '../testSupport/asserts';

const report = (outcomes: string[]): PcgImportReport => ({
  sourceFile: 'a.pcg',
  gameMode: 'Pathfinder_RPG',
  unknownKeys: [],
  unmappedClasses: [],
  unmappedRace: null,
  warnings: [],
  lines: outcomes.map((outcome, index) => ({ key: `K${index}`, from: '', outcome, to: '' })),
});

async function verifiesWithNoRuntimeItThrowsADescriptiveError() {
  let thrown: unknown;
  try {
    await importPcg({ filePath: '/tmp/a.pcg', savedAt: '2026-11-04T00:00:00Z' });
  } catch (cause) {
    thrown = cause;
  }
  assertEqual(thrown instanceof Error ? thrown.message : String(thrown), 'Tauri runtime not available for importing a PCGen file', 'no runtime');
}

function verifiesTheFilePickerOffersPcgFiles() {
  assertEqual(PCG_FILE_FILTER.extensions.join(), 'pcg', 'the extension');
}

function verifiesEachOutcomeIsCountedAndAKindWithNoLinesIsLeftOut() {
  assertEqual(
    JSON.stringify(pcgReportCounts(report(['mapped', 'custom', 'mapped', 'added']))),
    JSON.stringify([
      { outcome: 'mapped', count: 2 },
      { outcome: 'custom', count: 1 },
      { outcome: 'added', count: 1 },
    ]),
    'counts in the report order'
  );
}

async function main() {
  await verifiesWithNoRuntimeItThrowsADescriptiveError();
  verifiesTheFilePickerOffersPcgFiles();
  verifiesEachOutcomeIsCountedAndAKindWithNoLinesIsLeftOut();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
