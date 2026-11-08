import { useState } from 'react';
import { SettingRow, selectStyle } from './AppearancePanel';
import { PRINT_PAPERS, getPrintPaper, isPrintPaper, setPrintPaper, type PrintPaper } from './printPaper';

/**
 * Settings, Print (operator ruling R3): the paper every printed sheet uses. The Print dialog starts
 * from it and can change it for one print without changing this.
 */
export function PrintPanel(props: { initialPaper?: PrintPaper }) {
  const [paper, setPaper] = useState<PrintPaper>(() => props.initialPaper ?? getPrintPaper());
  return (
    <div>
      <SettingRow
        name="Print paper"
        description="The paper every printed sheet uses. The Print dialog can change it for one print."
        control={
          <select
            aria-label="Print paper"
            value={paper}
            onChange={(event) => {
              const next = event.target.value;
              if (isPrintPaper(next)) {
                setPaper(next);
                setPrintPaper(next);
              }
            }}
            style={selectStyle}
          >
            {PRINT_PAPERS.map((option) => (
              <option key={option.value} value={option.value}>
                {option.label}
              </option>
            ))}
          </select>
        }
      />
    </div>
  );
}
