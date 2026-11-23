import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { assert, assertEqual } from '../testSupport/asserts';
import { AppMenuList, withOptions } from './AppMenu';

/**
 * The ☰ menu on every screen (decisions.md §24.10): Options opens the Settings modal; the fixed ⚙
 * button is gone. A screen with its own ☰ menu (the character sheet, the campaign sheet) puts
 * Options in it and claims the top bar; every other screen gets App's ☰ menu.
 */

const here = dirname(fileURLToPath(import.meta.url));
const srcRoot = join(here, '..');
const read = (path: string) => readFileSync(join(srcRoot, path), 'utf8');

// Options is the last entry and choosing it opens the Settings modal.
let opened = 0;
const sheetItems = [
  { label: 'Open', onSelect: () => undefined },
  { label: 'Print', onSelect: () => undefined },
];
const items = withOptions(sheetItems, () => {
  opened += 1;
});
assertEqual(items.map((item) => item.label).join(), 'Open,Print,Options', 'a screen menu keeps its entries and gains Options last');
assertEqual(items[2]!.dividerBefore, true, 'Options sits under a divider after the screen entries');
items[2]!.onSelect();
assertEqual(opened, 1, 'choosing Options opens the Settings modal');
assertEqual(withOptions([], () => undefined).map((item) => item.label).join(), 'Options', 'the app menu on a plain screen holds Options alone');

const html = renderToStaticMarkup(createElement(AppMenuList, { items, onChosen: () => undefined }));
assert(html.includes('>Options</button>'), `the open ☰ menu shows an Options entry: ${html}`);

// "Main menu" first (operator UI-1, decisions.md §26.1): present iff the hub claims a goMain (every mode but 'landing').
let wentMain = 0;
const goMain = () => {
  wentMain += 1;
};
const onScreen = withOptions(sheetItems, () => undefined, goMain);
assertEqual(onScreen.map((item) => item.label).join(), 'Main menu,Open,Print,Options', 'main menu is first on a screen menu');
assertEqual(onScreen[1]!.dividerBefore, true, 'main menu is first on a screen menu: a divider follows it');
assertEqual(
  withOptions([], () => undefined, goMain).map((item) => item.label).join(),
  'Main menu,Options',
  'main menu is first on the app menu'
);
assertEqual(
  withOptions(sheetItems, () => undefined, null).map((item) => item.label).join(),
  'Open,Print,Options',
  'main menu is absent when nothing claims it (the landing screen)'
);
assertEqual(withOptions([], () => undefined).map((item) => item.label).join(), 'Options', 'main menu is absent when nothing claims it (the landing screen): the app menu');
onScreen[0]!.onSelect();
assertEqual(wentMain, 1, 'choosing main menu runs the claimed goMain');
const mainHtml = renderToStaticMarkup(createElement(AppMenuList, { items: onScreen, onChosen: () => undefined }));
assert(mainHtml.indexOf('>Main menu</button>') >= 0 && mainHtml.indexOf('>Main menu</button>') < mainHtml.indexOf('>Open</button>'), `the open ☰ menu draws Main menu first: ${mainHtml}`);

// App: no ⚙ button; the host's openOptions opens SettingsModal; the app ☰ menu shows unless a screen claims the bar.
const app = read('App.tsx');
assert(!app.includes('⚙') && !/aria-label="Open\s+settings"/.test(app), 'the fixed ⚙ button is removed from App.tsx');
assert(/openOptions:\s*\(\)\s*=>\s*setSettingsOpen\(true\)/.test(app), 'the menu host opens the existing SettingsModal');
assert(/screenMenus === 0 \? <AppMenu items=\{\[\]\} fixed \/>/.test(app), 'App shows its own ☰ menu on every screen that has none');

// Every screen kind the hub renders (CharacterHubPage's Mode union → its component) has a ☰ menu:
// its own (with Options) or App's.
const hub = read('characterHub/CharacterHubPage.tsx');
const modes = [...hub.match(/type Mode =([^;]+);/)![1]!.matchAll(/'([^']+)'/g)].map((m) => m[1]!);
assertEqual(modes.length, 18, 'the census: 18 hub screens');
const OWN_MENU: Record<string, string> = {
  sheet: 'characterHub/CharacterSheet.tsx',
  'campaign-sheet': 'campaign/CampaignSheet.tsx',
};
for (const mode of modes) {
  if (OWN_MENU[mode]) {
    const source = read(OWN_MENU[mode]!);
    assert(source.includes('<AppMenu '), `${mode}: its own ☰ menu is the shared AppMenu (gains Options, claims the bar)`);
  }
}
for (const other of [
  'characterHub/LandingScreen.tsx',
  'characterHub/LoadCharacterScreen.tsx',
  'characterHub/StarfinderCharacterSheet.tsx',
  'characterHub/CyberpunkCharacterSheet.tsx',
  'dmToolkit/DmToolkitScreen.tsx',
  'campaign/CampaignManagerScreen.tsx',
]) {
  assert(!read(other).includes('<AppMenu '), `${other}: no menu of its own, so App's ☰ menu shows there`);
}
assert(!read('characterHub/CharacterSheet.tsx').includes('setMenuOpen'), "the sheet's dropdown is the lifted AppMenu, not a copy");

// The hub claims Main menu whenever it is off the main screen; App's host holds the claim.
assert(
  /useLayoutEffect\(\s*\(\)\s*=>\s*\(offMain \? claimMain\(\(\) => setMode\('landing'\)\) : undefined\)/.test(hub) &&
    /const offMain = mode !== 'landing';/.test(hub),
  "CharacterHubPage.tsx claims claimMain under mode !== 'landing' with () => setMode('landing')"
);
assert(/claimMain:\s*\(next\b/.test(app) && /\{ \.\.\.menuCallbacks, goMain \}/.test(app) && /<CharacterHubPage claimMain=\{menuCallbacks\.claimMain\} \/>/.test(app), "App's menu host implements claimMain and hands AppMenu the claimed goMain");

console.log('AppMenu.test.ts: all assertions passed');
