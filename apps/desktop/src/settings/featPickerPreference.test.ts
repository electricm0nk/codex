import { getHideUnmetFeats, setHideUnmetFeats, type FeatPickerPreferenceStorage } from './featPickerPreference';
import { assertEqual } from '../testSupport/asserts';

/**
 * The feat picker's "Hide feats I don't qualify for" app setting: ticked until
 * the player unticks it, and the choice survives. Storage is injected because
 * the Node test runtime has no localStorage; a store that throws stands in for
 * the private-mode failure the other settings modules guard against.
 */
function memoryStorage(): FeatPickerPreferenceStorage {
  const values = new Map<string, string>();
  return {
    getItem: (key) => values.get(key) ?? null,
    setItem: (key, value) => void values.set(key, value),
  };
}

const storage = memoryStorage();
assertEqual(getHideUnmetFeats(storage), true, 'never set: ticked');

setHideUnmetFeats(false, storage);
assertEqual(getHideUnmetFeats(storage), false, 'unticked is remembered');

setHideUnmetFeats(true, storage);
assertEqual(getHideUnmetFeats(storage), true, 'ticked again is remembered');

const garbled = memoryStorage();
garbled.setItem('codex.featPicker.hideUnmet', 'maybe');
assertEqual(getHideUnmetFeats(garbled), true, 'an unreadable value falls back to ticked');

const broken: FeatPickerPreferenceStorage = {
  getItem: () => {
    throw new Error('storage unavailable');
  },
  setItem: () => {
    throw new Error('storage unavailable');
  },
};
assertEqual(getHideUnmetFeats(broken), true, 'a store that throws reads as ticked');
setHideUnmetFeats(false, broken);

// The Node runtime has no localStorage, so this is the no-argument path meeting a missing store.
assertEqual(getHideUnmetFeats(), true, 'no store at all reads as ticked');

console.log('featPickerPreference.test.ts: all assertions passed');
