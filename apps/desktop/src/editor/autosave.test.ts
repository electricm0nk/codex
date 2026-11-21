import { createAutosave, type AutosaveIO, type AutosaveState } from './autosave';
import { assert, assertEqual } from '../testSupport/asserts';

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));
const DELAY = 30;

// An in-memory file with an mtime, and an editor document.
function harness(initial = 'hello') {
  const disk = { text: initial, mtime: 100, writes: 0 };
  const doc = { text: initial };
  const states: AutosaveState[] = [];
  const io: AutosaveIO = {
    getText: () => doc.text,
    write: async (text) => {
      disk.text = text;
      disk.mtime += 1;
      disk.writes += 1;
      return disk.mtime;
    },
    stat: async () => disk.mtime,
    read: async () => ({ text: disk.text, modifiedMs: disk.mtime }),
    replaceDoc: (text) => {
      doc.text = text;
      saver.changed(); // the editor's update listener fires on the replacement, as CodeMirror's does
    },
    onState: (state) => states.push(state),
  };
  const saver = createAutosave(io, disk.mtime, DELAY);
  const edit = (text: string) => {
    doc.text = text;
    saver.changed();
  };
  return { disk, doc, states, saver, edit, last: () => states[states.length - 1] };
}

const cases: Array<[string, () => Promise<void>]> = [
  ['debounced write', async () => {
    const h = harness();
    h.edit('h');
    h.edit('he');
    h.edit('hey');
    await sleep(DELAY / 3);
    assertEqual(h.disk.writes, 0, 'nothing is written before the delay');
    await sleep(DELAY * 3);
    assertEqual(h.disk.writes, 1, 'three edits make one write');
    assertEqual(h.disk.text, 'hey', 'the last text is written');
    assertEqual(h.last()?.kind, 'saved', 'the state reads saved');
    h.edit('hey!');
    await h.saver.flush(); // blur
    assertEqual(h.disk.writes, 2, 'blur writes at once');
    await h.saver.flush();
    assertEqual(h.disk.writes, 2, 'nothing to write when clean');
  }],
  ['mtime change blocks the write and raises the banner', async () => {
    const h = harness();
    h.disk.text = 'from another editor';
    h.disk.mtime = 500;
    h.edit('mine');
    await sleep(DELAY * 3);
    assertEqual(h.disk.writes, 0, 'no write over a file changed on disk');
    assertEqual(h.disk.text, 'from another editor', 'the other text is kept');
    assertEqual(h.last()?.kind, 'conflict', 'the banner is up');
    h.edit('mine, more');
    await sleep(DELAY * 3);
    await h.saver.flush();
    assertEqual(h.disk.writes, 0, 'no write while the banner is up');
    await h.saver.keepMine();
    assertEqual(h.disk.text, 'mine, more', 'Keep mine writes the editor text');
    assertEqual(h.last()?.kind, 'saved', 'the banner is down after Keep mine');
    h.edit('mine, again');
    await h.saver.flush();
    assertEqual(h.disk.text, 'mine, again', 'auto-save resumes against the new mtime');
  }],
  ['reload replaces the doc', async () => {
    const h = harness();
    h.disk.text = 'theirs';
    h.disk.mtime = 900;
    h.edit('mine');
    await h.saver.flush();
    assertEqual(h.last()?.kind, 'conflict', 'the banner is up');
    await h.saver.reload();
    assertEqual(h.doc.text, 'theirs', 'Reload puts the disk text in the editor');
    assertEqual(h.last()?.kind, 'idle', 'the banner is down');
    await sleep(DELAY * 3);
    assertEqual(h.disk.writes, 0, 'the reload itself is not written back');
    h.edit('theirs, edited');
    await h.saver.flush();
    assertEqual(h.disk.text, 'theirs, edited', 'auto-save resumes against the reloaded mtime');
  }],
  ['write failure keeps the edit dirty', async () => {
    let fail = true;
    const states: AutosaveState[] = [];
    const saver = createAutosave(
      {
        getText: () => 'text',
        write: async () => {
          if (fail) throw new Error('disk full');
          return 7;
        },
        stat: async () => 1,
        read: async () => ({ text: '', modifiedMs: 1 }),
        replaceDoc: () => undefined,
        onState: (s) => states.push(s),
      },
      1,
      DELAY,
    );
    saver.changed();
    await saver.flush();
    const failed = states[states.length - 1];
    assert(failed?.kind === 'error' && failed.message.includes('disk full'), 'the failure is shown');
    fail = false;
    await saver.flush();
    assertEqual(states[states.length - 1]?.kind, 'saved', 'the next flush retries the unsaved edit');
    saver.dispose();
  }],
];

async function main() {
  let passed = 0;
  for (const [name, run] of cases) {
    await run();
    passed += 1;
    console.log(`  ok ${name}`);
  }
  console.log(`autosave: ${passed} cases passed`);
}

main().catch((err: unknown) => {
  console.error(err);
  process.exit(1);
});
