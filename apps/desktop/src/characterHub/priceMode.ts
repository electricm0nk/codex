/**
 * How equipment moves money, chosen on the equipment screen. Per session only: it is never saved on
 * the character, and the backend's `PriceMode` (camelCase) is the source of truth for the rule.
 */
export type PriceMode = 'cashless' | 'standard' | 'characterBuild';

export const DEFAULT_PRICE_MODE: PriceMode = 'standard';

export const PRICE_MODE_OPTIONS: ReadonlyArray<{ value: PriceMode; label: string; hint: string }> = [
  {
    value: 'cashless',
    label: 'Cashless',
    hint: 'Add and remove items freely. No money is spent or returned.',
  },
  {
    value: 'standard',
    label: 'Buy 100% / Sell 50%',
    hint: 'Pay the full catalog price; selling an item back returns half of it.',
  },
  {
    value: 'characterBuild',
    label: 'Character build (100% / 100%)',
    hint: 'Pay the full catalog price; selling an item back returns all of it, so trying items out costs nothing.',
  },
];

/** What selling back an item priced `costCopper` returns; mirrors `refund_copper` in the backend. */
export function refundPreviewCopper(costCopper: number, mode: PriceMode): number {
  switch (mode) {
    case 'cashless':
      return 0;
    case 'standard':
      return Math.floor(costCopper / 2);
    case 'characterBuild':
      return costCopper;
  }
}
