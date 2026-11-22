import { localIsoDate, nextSessionTitle, sessionLabel, statusMark } from './adventureLogModel';
import type { AdventureLogEntry } from '../boundary/campaignFolder/adventureLog';
import { assertEqual } from '../testSupport/asserts';

/**
 * SD-42 E2.7 (decisions.md §15.2): the Adventure Log list shows each entry's session (a day or a
 * range), its in-game date and an SD-o mark when the frontmatter was not read; "+" proposes the
 * next session title and stamps the local date.
 */

const entry = (over: Partial<AdventureLogEntry>): AdventureLogEntry => ({
  path: 'Adventure Log/x.md',
  title: 'x',
  author: null,
  posted: null,
  session: null,
  inGameDate: null,
  frontmatterOk: true,
  ...over,
});

function testSessionLabel() {
  assertEqual(sessionLabel({ start: '2026-11-17', end: null }), '2026-11-17', 'one day');
  assertEqual(sessionLabel({ start: '2026-11-14', end: '2026-11-16' }), '2026-11-14 – 2026-11-16', 'a range');
  assertEqual(sessionLabel(null), '—', 'no readable session');
}

function testNextSessionTitle() {
  assertEqual(nextSessionTitle([]), 'Session 1', 'empty log');
  assertEqual(nextSessionTitle([entry({}), entry({})]), 'Session 3', 'one past the count');
}

function testLocalIsoDate() {
  assertEqual(localIsoDate(new Date(2026, 0, 5, 23, 59)), '2026-01-05', 'local date, zero-padded, late evening stays that day');
}

function testStatusMark() {
  assertEqual(statusMark(entry({ frontmatterOk: false })), 'frontmatter not read', 'SD-o mark');
  assertEqual(statusMark(entry({})), null, 'no mark when read');
}

testSessionLabel();
testNextSessionTitle();
testLocalIsoDate();
testStatusMark();
console.log('adventureLogModel: 4 cases passed');
