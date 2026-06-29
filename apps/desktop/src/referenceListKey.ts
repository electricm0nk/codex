export function createReferenceListKey(
  label: string,
  discriminator: string | number
): string {
  return `${label}-${discriminator}`;
}
