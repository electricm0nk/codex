import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { PrintPaper } from '../settings/printPaper';

/**
 * Write desktop boundary for the two print commands (operator rulings R5 and R19).
 *
 * `save_sheet_pdf` renders the character's sheet with the Rust print pipeline, writes it to the
 * path the save-as dialog returned and opens it in the system PDF viewer. `print_sheet_pdf` renders
 * the same bytes to `<temp>/codex-print/<characterId>.pdf` and opens that; the player prints from
 * the viewer. Both take the paper and the sections to print flattened beside `characterId`, and the campaigns that
 * list the character, which live in the frontend's store (`campaignModel.ts`).
 */

export interface CampaignNoteDto {
  name: string;
  description: string;
}

/** The print dialog's eight section boxes, in box order (`print/mod.rs` `PrintSections`). */
export interface PrintSectionsDto {
  main: boolean;
  feats: boolean;
  inventory: boolean;
  spellbook: boolean;
  bio: boolean;
  progression: boolean;
  math: boolean;
  notes: boolean;
}

export interface PrintOptionsDto {
  paper: PrintPaper;
  /** The sections ticked. */
  sections: PrintSectionsDto;
  campaigns: CampaignNoteDto[];
}

export interface PrintSheetPdfRequest extends PrintOptionsDto {
  characterId: string;
}

export interface SaveSheetPdfRequest extends PrintSheetPdfRequest {
  filePath: string;
}

/** What the command wrote: where, how many bytes, and every warning Typst raised making it. */
export interface SavedSheetPdf {
  filePath: string;
  bytes: number;
  warnings: string[];
}

export async function saveSheetPdf(request: SaveSheetPdfRequest): Promise<SavedSheetPdf> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for saving a sheet PDF');
  }
  try {
    return await invoke<SavedSheetPdf>('save_sheet_pdf', { request });
  } catch (cause: unknown) {
    throw new Error(formatError(cause));
  }
}

export async function printSheetPdf(request: PrintSheetPdfRequest): Promise<SavedSheetPdf> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for printing a sheet');
  }
  try {
    return await invoke<SavedSheetPdf>('print_sheet_pdf', { request });
  } catch (cause: unknown) {
    throw new Error(formatError(cause));
  }
}
