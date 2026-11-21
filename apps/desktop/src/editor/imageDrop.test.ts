import { imageLinks, isImageFile } from './imageDrop';
import { assert, assertEqual } from '../testSupport/asserts';

const file = (name: string, type: string, text = 'bytes') => new File([text], name, { type });

const cases: Array<[string, () => Promise<void>]> = [
  ['link inserted', async () => {
    const saved: Array<[string, string]> = [];
    const links = await imageLinks([file('map.png', 'image/png', 'PNG')], async (name, bytes) => {
      saved.push([name, new TextDecoder().decode(bytes)]);
      return `Attachments/${name}`;
    });
    assertEqual(links, '![](Attachments/map.png)', 'the Q-F link to the saved file');
    assertEqual(saved.map(([n, b]) => `${n}=${b}`).join(), 'map.png=PNG', 'the dropped bytes are saved under their name');
  }],
  ['collision suffix', async () => {
    const onDisk = new Set(['map.png']);
    // The save command picks the free name (campaign_folder.rs save_attachment); the link uses what it returns.
    const save = async (name: string) => {
      let candidate = name;
      for (let n = 2; onDisk.has(candidate); n += 1) candidate = name.replace(/(\.[^.]*)?$/, `-${n}$1`);
      onDisk.add(candidate);
      return `Attachments/${candidate}`;
    };
    const links = await imageLinks([file('map.png', 'image/png'), file('map.png', 'image/png')], save);
    assertEqual(links, '![](Attachments/map-2.png)\n![](Attachments/map-3.png)', 'each drop links its own suffixed file');
  }],
  ['only images', async () => {
    assert(isImageFile(file('a.png', 'image/png')), 'png by type');
    assert(isImageFile(file('a.JPG', '')), 'jpg by extension when the type is empty');
    assert(!isImageFile(file('notes.md', 'text/markdown')), 'a Markdown file is not an image');
    assertEqual(await imageLinks([file('notes.md', 'text/markdown')], async () => 'never'), '', 'non-images are not saved');
  }],
];

async function main() {
  let passed = 0;
  for (const [name, run] of cases) {
    await run();
    passed += 1;
    console.log(`  ok ${name}`);
  }
  console.log(`imageDrop: ${passed} cases passed`);
}

main().catch((err: unknown) => {
  console.error(err);
  process.exit(1);
});
