import { PRINT_PAPERS, getPrintPaper, setPrintPaper, type PrintPaperStorage } from './printPaper';
import { assertEqual } from '../testSupport/asserts';

/**
 * Settings, Print paper (operator ruling R3): A4 until the player picks US Letter, and the choice
 * survives. Storage is injected because the Node test runtime has no localStorage.
 */
function memoryStorage(): PrintPaperStorage {
  const values = new Map<string, string>();
  return {
    getItem: (key) => values.get(key) ?? null,
    setItem: (key, value) => void values.set(key, value),
  };
}

assertEqual(PRINT_PAPERS.map((paper) => paper.value).join('|'), 'a4|letter', 'two papers, A4 first');
assertEqual(PRINT_PAPERS.map((paper) => paper.label).join('|'), 'A4|US Letter', 'the papers read as the player knows them');

const storage = memoryStorage();
assertEqual(getPrintPaper(storage), 'a4', 'never set: A4');
setPrintPaper('letter', storage);
assertEqual(getPrintPaper(storage), 'letter', 'US Letter is remembered');
assertEqual(storage.getItem('codex.print.paper'), 'letter', 'under its own key');
setPrintPaper('a4', storage);
assertEqual(getPrintPaper(storage), 'a4', 'A4 again is remembered');

const garbled = memoryStorage();
garbled.setItem('codex.print.paper', 'legal');
assertEqual(getPrintPaper(garbled), 'a4', 'a paper the app does not print on reads as A4');

const broken: PrintPaperStorage = {
  getItem: () => {
    throw new Error('storage unavailable');
  },
  setItem: () => {
    throw new Error('storage unavailable');
  },
};
assertEqual(getPrintPaper(broken), 'a4', 'a store that throws reads as A4');
setPrintPaper('letter', broken);

console.log('printPaper.test.ts: all assertions passed');
