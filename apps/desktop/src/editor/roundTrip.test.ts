import { createHash } from 'node:crypto';
import { readdirSync, readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { EditorState } from '@codemirror/state';
import { decode, encode } from './roundTrip';
import { assertEqual } from '../testSupport/asserts';

const fixtureDir = join(dirname(fileURLToPath(import.meta.url)), 'fixtures');
const sha256 = (bytes: Uint8Array) => createHash('sha256').update(bytes).digest('hex');

function main() {
  const names = readdirSync(fixtureDir).filter((name) => name.endsWith('.md')).sort();
  if (names.length < 6) throw new Error(`expected >= 6 fixtures, found ${names.length}`);

  for (const name of names) {
    const bytes = new Uint8Array(readFileSync(join(fixtureDir, name)));
    const decoded = decode(bytes);
    assertEqual(sha256(encode(decoded.text, decoded)), sha256(bytes), `${name}: decode -> encode`);

    // The editor normalises line breaks to \n; a save without an edit must still be byte-identical.
    const viaState = EditorState.create({ doc: decoded.text }).doc.toString();
    assertEqual(sha256(encode(viaState, decoded)), sha256(bytes), `${name}: EditorState round trip`);
    // Rebuild from the metadata alone (the path an edited save takes): every single-ending file is exact.
    if (name !== 'mixed-eol.md') {
      const rebuilt = encode(viaState, { ...decoded, sourceText: '\u0000not-the-source' });
      assertEqual(sha256(rebuilt), sha256(bytes), `${name}: rebuilt from eol/bom/trailing-newline`);
    }
    console.log(`  ok ${name} sha256 ${sha256(bytes).slice(0, 12)}`);
  }

  const crlf = decode(new Uint8Array(readFileSync(join(fixtureDir, 'crlf.md'))));
  assertEqual(crlf.eol, '\r\n', 'crlf.md detects CRLF');
  assertEqual(crlf.text.includes('\r'), false, 'decoded text holds no \\r');
  assertEqual(crlf.trailingNewline, true, 'crlf.md ends with a newline');

  const bom = decode(new Uint8Array(readFileSync(join(fixtureDir, 'bom.md'))));
  assertEqual(bom.bom, true, 'bom.md detects the BOM');
  assertEqual(bom.text.startsWith('# BOM'), true, 'the BOM is not part of the text');

  const bare = decode(new Uint8Array(readFileSync(join(fixtureDir, 'no-trailing-newline.md'))));
  assertEqual(bare.trailingNewline, false, 'no-trailing-newline.md has none');

  // An edit keeps the file's line ending, BOM and trailing-newline shape.
  const edited = new TextDecoder().decode(encode(crlf.text.replace('A **CRLF** file.', 'Edited.\nTwo lines.'), crlf));
  assertEqual(edited.endsWith('Edited.\r\nTwo lines.\r\n'), true, 'an edit is written with CRLF');
  const bomEdited = encode('# New', bom);
  assertEqual([...bomEdited.slice(0, 3)].join(','), '239,187,191', 'an edit keeps the BOM');

  let threw = false;
  try {
    decode(new Uint8Array([0x23, 0x20, 0xff, 0xfe]));
  } catch {
    threw = true;
  }
  assertEqual(threw, true, 'bytes that are not UTF-8 are refused, never rewritten');
  console.log(`roundTrip: ${names.length} fixtures byte-stable`);
}

main();
