//! The `cpr.` explanation rows a Cyberpunk RED sheet renders. Every number on the sheet is
//! one row here (id `cpr.<area>.<field>`, value, derivation text); no frontend code
//! re-derives a number. E0.4 emits the ten STAT rows; the engine cards (E2) add the derived
//! values and E2.10 composes them.

use crate::rules_core::cpr::input::{CprCharacterInput, CprStat};
use crate::rules_core::pilot_compute::ComputationExplanation;

/// One `cpr.stat.<id>` row per STAT, in book order, each equal to the entered value.
pub fn stat_rows(input: &CprCharacterInput) -> Vec<ComputationExplanation> {
    CprStat::ALL
        .into_iter()
        .map(|stat| {
            let value = input.stats.get(stat);
            ComputationExplanation {
                id: format!("cpr.stat.{}", stat.id()),
                value: i16::from(value),
                detail: format!(
                    "{} {value}: the character's {} STAT as entered (Cyberpunk RED core rulebook p. 72-73)",
                    stat.label(),
                    stat.name()
                ),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::cpr::input::{CprCharacterInput, CprStat, CprStats};

    #[test]
    fn the_sheet_carries_the_ten_stats_as_cpr_stat_rows_equal_to_the_input() {
        let mut stats = CprStats::default();
        for (i, stat) in CprStat::ALL.into_iter().enumerate() {
            stats.set(stat, 1 + i as u8);
        }
        let rows = stat_rows(&CprCharacterInput { stats });

        let ids: Vec<&str> = rows.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(
            ids,
            [
                "cpr.stat.int", "cpr.stat.ref", "cpr.stat.dex", "cpr.stat.tech", "cpr.stat.cool",
                "cpr.stat.will", "cpr.stat.luck", "cpr.stat.move", "cpr.stat.body", "cpr.stat.emp",
            ]
        );
        let values: Vec<i16> = rows.iter().map(|r| r.value).collect();
        assert_eq!(values, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(rows[9].detail, "EMP 10: the character's Empathy STAT as entered (Cyberpunk RED core rulebook p. 72-73)");
    }
}
