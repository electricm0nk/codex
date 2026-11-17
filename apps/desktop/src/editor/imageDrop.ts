// Drag-in images (decisions.md §18, §21 Q-F): an image file dropped on the editor is copied into
// the campaign's `Attachments/` by the caller's `saveAttachment` and linked as `![](Attachments/<name>)`.
import type { Extension } from '@codemirror/state';
import { EditorView } from '@codemirror/view';

/** Saves the bytes under `Attachments/`; resolves to the path it used (a collision adds `-2`). */
export type SaveAttachment = (name: string, bytes: Uint8Array) => Promise<string>;

const IMAGE_EXTENSION = /\.(png|jpe?g|gif|webp|svg|bmp|avif)$/i;

export function isImageFile(file: File): boolean {
  return file.type.startsWith('image/') || IMAGE_EXTENSION.test(file.name);
}

/** Saves every image among `files`, in order; resolves to their links, one per line. */
export async function imageLinks(files: File[], save: SaveAttachment): Promise<string> {
  const links: string[] = [];
  for (const file of files.filter(isImageFile)) {
    const path = await save(file.name, new Uint8Array(await file.arrayBuffer()));
    links.push(`![](${path})`);
  }
  return links.join('\n');
}

const hasFiles = (event: DragEvent) => event.dataTransfer?.types.includes('Files') ?? false;

export function imageDrop(save: SaveAttachment, onError: (message: string) => void): Extension {
  return EditorView.domEventHandlers({
    dragover(event, view) {
      if (!hasFiles(event) || view.state.readOnly) return false;
      event.preventDefault();
      return true;
    },
    drop(event, view) {
      const files = [...(event.dataTransfer?.files ?? [])].filter(isImageFile);
      if (files.length === 0 || view.state.readOnly) return false;
      event.preventDefault();
      const pos = view.posAtCoords({ x: event.clientX, y: event.clientY }) ?? view.state.selection.main.head;
      imageLinks(files, save)
        .then((text) => {
          // The file may have been closed while the copy ran.
          if (!view.dom.isConnected) return;
          const at = Math.min(pos, view.state.doc.length);
          view.dispatch({ changes: { from: at, insert: text }, selection: { anchor: at + text.length } });
        })
        .catch((err: unknown) => onError(`Could not add the image: ${err instanceof Error ? err.message : String(err)}`));
      return true;
    },
  });
}
