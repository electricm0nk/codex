/**
 * Client-side portrait prep: validate, then center-crop + resize to a fixed
 * square PNG before anything is sent over the Tauri boundary. Keeps saved
 * portraits small and uniform without needing an image-processing crate on
 * the Rust side — the canvas API already does cover-fit cropping for free.
 */

export const PORTRAIT_TARGET_SIZE = 256;
export const MAX_PORTRAIT_UPLOAD_BYTES = 8 * 1024 * 1024;
export const ACCEPTED_PORTRAIT_TYPES = ['image/png', 'image/jpeg', 'image/gif', 'image/webp'];

export class PortraitUploadError extends Error {}

/**
 * Reads `file`, then draws it onto a `PORTRAIT_TARGET_SIZE`-square canvas
 * with a center crop (cover-fit: scale so the shorter side fills the
 * square, crop the overflow from the centered long side) and returns a PNG
 * data URL. Animated GIFs are flattened to their first frame — canvas has
 * no concept of animation, which is fine for a static portrait.
 */
export async function resizeAndCropPortrait(file: File): Promise<string> {
  if (!ACCEPTED_PORTRAIT_TYPES.includes(file.type)) {
    throw new PortraitUploadError(`Unsupported image type "${file.type || 'unknown'}". Use PNG, JPEG, GIF, or WebP.`);
  }
  if (file.size > MAX_PORTRAIT_UPLOAD_BYTES) {
    const maxMb = (MAX_PORTRAIT_UPLOAD_BYTES / (1024 * 1024)).toFixed(0);
    throw new PortraitUploadError(`Image is too large — the limit is ${maxMb} MB.`);
  }

  const objectUrl = URL.createObjectURL(file);
  try {
    const image = await loadImage(objectUrl);

    const canvas = document.createElement('canvas');
    canvas.width = PORTRAIT_TARGET_SIZE;
    canvas.height = PORTRAIT_TARGET_SIZE;
    const ctx = canvas.getContext('2d');
    if (!ctx) {
      throw new PortraitUploadError('Could not prepare the image for upload.');
    }

    const scale = Math.max(PORTRAIT_TARGET_SIZE / image.width, PORTRAIT_TARGET_SIZE / image.height);
    const drawWidth = image.width * scale;
    const drawHeight = image.height * scale;
    const offsetX = (PORTRAIT_TARGET_SIZE - drawWidth) / 2;
    const offsetY = (PORTRAIT_TARGET_SIZE - drawHeight) / 2;
    ctx.drawImage(image, offsetX, offsetY, drawWidth, drawHeight);

    return canvas.toDataURL('image/png');
  } finally {
    URL.revokeObjectURL(objectUrl);
  }
}

function loadImage(src: string): Promise<HTMLImageElement> {
  return new Promise((resolve, reject) => {
    const image = new Image();
    image.onload = () => resolve(image);
    image.onerror = () => reject(new PortraitUploadError('Could not read that image file.'));
    image.src = src;
  });
}
