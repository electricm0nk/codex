// Byte-stable Markdown round trip (decisions.md §18): the editor works on \n-joined text;
// a save re-applies the file's own line ending, BOM and trailing newline.

export type Eol = '\n' | '\r\n';

export interface DecodedFile {
  text: string;
  eol: Eol;
  trailingNewline: boolean;
  bom: boolean;
  /** The text as first decoded; an unedited save writes `original` back exactly (mixed line endings included). */
  sourceText: string;
  original: Uint8Array;
}

const BOM = '﻿';

/** Throws on bytes that are not UTF-8: such a file is opened read-only, never rewritten. */
export function decode(bytes: Uint8Array): DecodedFile {
  let raw = new TextDecoder('utf-8', { fatal: true, ignoreBOM: true }).decode(bytes);
  const bom = raw.startsWith(BOM);
  if (bom) raw = raw.slice(1);
  const firstBreak = raw.search(/\r\n|\n/);
  const eol: Eol = firstBreak >= 0 && raw[firstBreak] === '\r' ? '\r\n' : '\n';
  let text = raw.replace(/\r\n?/g, '\n');
  const trailingNewline = text.endsWith('\n');
  if (trailingNewline) text = text.slice(0, -1);
  return { text, eol, trailingNewline, bom, sourceText: text, original: bytes };
}

export function encode(text: string, meta: DecodedFile): Uint8Array {
  if (text === meta.sourceText) return meta.original;
  const body = (meta.trailingNewline ? `${text}\n` : text).replace(/\n/g, meta.eol);
  return new TextEncoder().encode(meta.bom ? BOM + body : body);
}
