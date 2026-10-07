/**
 * Print layout for the character sheet: US letter pages, black on white whatever the theme.
 *
 * Page 1 is the sheet's own three-column layout (progression, stats and combat, details and skills)
 * at print size; the tabs then follow as sections (`.print-tab`), the first on a fresh page. The CSS is
 * emitted only for print media, so nothing here changes the on-screen sheet. Class names it relies
 * on (`sheet-cols`, `sheet-left`, `sheet-mid`, `sheet-right`, `print-tab`, `no-print`) are applied in
 * `CharacterSheet.tsx`.
 */

export const PRINT_PAGE_IN = { width: 8.5, height: 11 } as const;
export const PRINT_MARGIN_IN = 0.4;

/** Fixed column widths for page 1; the middle column takes whatever is left. */
export const PRINT_COLUMNS_IN = { left: 1.7, right: 2.25, gap: 0.12 } as const;

/** Tabs in printed order. Must list exactly the sheet's tabs (checked by `printLayout.test.ts`). */
export const PRINT_TAB_ORDER = ['Weapons', 'Defense', 'Gear', 'Spells', 'Feats', 'Pets', 'Actions'] as const;

export function printContentWidthIn(): number {
  return PRINT_PAGE_IN.width - 2 * PRINT_MARGIN_IN;
}

/** The tabs to print: all of them, except an empty Pets page. */
export function printableTabs(options: { hasPets: boolean }): Array<(typeof PRINT_TAB_ORDER)[number]> {
  return PRINT_TAB_ORDER.filter((tab) => tab !== 'Pets' || options.hasPets);
}

export function buildPrintCss(): string {
  const { left, right, gap } = PRINT_COLUMNS_IN;
  return `
@page { size: letter; margin: ${PRINT_MARGIN_IN}in; }
@media print {
  :root, body, body.theme-dark, body.theme-light {
    --color-page: #fff !important;
    --color-surface: #fff !important;
    --color-surface-2: #f2f2f2 !important;
    --color-text: #000 !important;
    --color-text-secondary: #1a1a1a !important;
    --color-text-muted: #404040 !important;
    --color-text-faint: #606060 !important;
    --color-border: #8c8c8c !important;
    --color-border-strong: #1a1a1a !important;
    --color-accent: #000 !important;
    --color-accent-hover: #000 !important;
    --color-on-accent: #fff !important;
    --color-on-accent-muted: #ddd !important;
    --color-link: #000 !important;
    --color-warn: #5a3b00 !important;
    --color-error: #8b0000 !important;
  }
  html { color-scheme: light !important; background: #fff !important; }
  html, body { background: #fff !important; color: #000 !important; font-size: 8.5pt !important; }
  body * { box-shadow: none !important; text-shadow: none !important; -webkit-print-color-adjust: exact; print-color-adjust: exact; }
  main { padding: 0 !important; margin: 0 !important; max-width: none !important; }

  /* Controls and dialogs have no place on paper. */
  button, [role="dialog"], [role="presentation"], .no-print { display: none !important; }
  /* Form fields keep their values but lose their chrome. */
  select, input, textarea { appearance: none !important; -webkit-appearance: none !important; background: transparent !important; border: none !important; border-bottom: 0.5pt solid #8c8c8c !important; border-radius: 0 !important; color: #000 !important; padding-left: 0 !important; }

  /*
   * Page 1: progression down the left, stats and combat on top of the middle with skills below them
   * in two columns, character details down the right. A grid (not the on-screen flex columns) lets
   * the right column's two panels land in different places.
   */
  .sheet-cols { display: grid !important; grid-template-columns: ${left}in minmax(0, 1fr) ${right}in; grid-template-rows: auto 1fr; column-gap: ${gap}in; row-gap: 0.1in; padding: 0 !important; }
  .sheet-left { grid-column: 1; grid-row: 1 / span 2; min-width: 0 !important; width: auto !important; padding-right: 0.08in !important; }
  .sheet-mid { grid-column: 2; grid-row: 1; min-width: 0 !important; }
  .sheet-right { display: contents !important; }
  .print-details { grid-column: 3; grid-row: 1 / span 2; min-width: 0; }
  .print-skills { grid-column: 2; grid-row: 2; min-width: 0; }
  .skill-rows { display: block !important; column-count: 2; column-gap: 0.15in; }
  .skill-rows > div { break-inside: avoid; }
  .sheet-cols, .sheet-cols * { overflow: visible !important; }

  /* After page 1 the tabs flow one after another; a tab that fits stays whole, a long one breaks. */
  .print-tab { break-inside: avoid; margin-top: 0.2in; }
  .print-tab-first { break-before: page; margin-top: 0; }
  .print-tab h2 { break-after: avoid; border-bottom: 1.5pt solid #000; font-size: 12pt; margin: 0 0 0.1in; padding-bottom: 0.03in; }
  .print-tab > div { box-shadow: none !important; }
  tr, li, .print-keep { break-inside: avoid; }
}
`;
}
