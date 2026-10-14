//! Cyberpunk RED Derived STATs (core rulebook p. 79-80): Hit Points, the Seriously Wounded
//! threshold, Death Save, Humanity, current EMP, plus the MOVE and LUCK values the sheet
//! carries. Each value is one `cpr.derived.<field>` explanation row.
//!
//! Only the numbers that land on the sheet are computed here. Wound penalties, the Death
//! State and Luck spending are play-time rules the sheet prints, not values this module
//! simulates.

use crate::rules_core::cpr::input::{CprCharacterInput, CprStat};
use crate::rules_core::pilot_compute::ComputationExplanation;

const SOURCE_HP: &str = "Cyberpunk RED core rulebook p. 79";
const SOURCE_HUMANITY: &str = "Cyberpunk RED core rulebook p. 80";

/// p. 79: "Hit Points equal to 10 + (5[BODY and WILL averaged, rounding up])".
pub fn hit_points(body: u8, will: u8) -> i16 {
    let average_rounded_up = (u16::from(body) + u16::from(will)).div_ceil(2);
    10 + 5 * average_rounded_up as i16
}

/// p. 79: "Seriously Wounded Wound Threshold is half of their total HP (rounded up)".
pub fn seriously_wounded(hit_points: i16) -> i16 {
    hit_points.max(0).unsigned_abs().div_ceil(2) as i16
}

/// p. 80: "For every point of Empathy the Character has, they gain 10 points of Humanity".
pub fn humanity_max(emp: u8) -> i16 {
    i16::from(emp) * 10
}

/// p. 80: EMP lowers "every time the tens place of your Humanity value is lowered" (44 -> 4,
/// 39 -> 3). A-4: a negative Humanity gives EMP 0 (p. 232, "Equal to 0 and You Have
/// Negative Humanity"), so the value never goes below 0.
pub fn current_emp(humanity: i16) -> i16 {
    humanity.div_euclid(10).max(0)
}

/// The derived rows for one character. `current_humanity` is the character's Humanity after
/// Humanity Loss (the cyberware module computes the loss; with none it equals
/// [`humanity_max`] of the entered EMP).
pub fn derived_rows(input: &CprCharacterInput, current_humanity: i16) -> Vec<ComputationExplanation> {
    let body = input.stats.get(CprStat::Body);
    let will = input.stats.get(CprStat::Will);
    let emp = input.stats.get(CprStat::Emp);
    let movement = input.stats.get(CprStat::Move);
    let luck = input.stats.get(CprStat::Luck);

    let hp = hit_points(body, will);
    let average = (u16::from(body) + u16::from(will)).div_ceil(2);
    let wounded = seriously_wounded(hp);
    let hum_max = humanity_max(emp);
    let emp_now = current_emp(current_humanity);

    let row = |id: &str, value: i16, detail: String| ComputationExplanation {
        id: format!("cpr.derived.{id}"),
        value,
        detail,
    };
    vec![
        row(
            "hit_points",
            hp,
            format!("HP {hp} = 10 + 5 x ceil((BODY {body} + WILL {will}) / 2) = 10 + 5 x {average} ({SOURCE_HP})"),
        ),
        row(
            "seriously_wounded",
            wounded,
            format!("Seriously Wounded {wounded} = ceil(HP {hp} / 2) ({SOURCE_HP})"),
        ),
        row("death_save", i16::from(body), format!("Death Save {body} = BODY {body} ({SOURCE_HP})")),
        row(
            "humanity_max",
            hum_max,
            format!("Humanity {hum_max} = EMP {emp} x 10 ({SOURCE_HUMANITY})"),
        ),
        row(
            "emp_current",
            emp_now,
            format!(
                "EMP {emp_now} = max(0, floor(Humanity {current_humanity} / 10)) ({SOURCE_HUMANITY}; negative Humanity gives 0, p. 232)"
            ),
        ),
        row("move", i16::from(movement), format!("MOVE {movement}: the MOVE STAT as entered (Cyberpunk RED core rulebook p. 72)")),
        row("luck_pool", i16::from(luck), format!("LUCK pool {luck} = LUCK {luck} (Cyberpunk RED core rulebook p. 72)")),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::cpr::input::{CprCharacterInput, CprStat, CprStats};

    /// The p080 Hit Points table, transcribed from the page (not from the corpus the engine
    /// reads): rows WILL 2-10, columns BODY 2-15.
    const HP_TABLE_P080: &str = include_str!("../../../tests/fixtures/cpr/hp_table_p080.json");

    #[test]
    fn all_126_cells_of_the_p080_hp_table_equal_the_formula() {
        let table: serde_json::Value = serde_json::from_str(HP_TABLE_P080).unwrap();
        let bodies: Vec<u8> = table["body_columns"]
            .as_array()
            .unwrap()
            .iter()
            .map(|b| b.as_u64().unwrap() as u8)
            .collect();
        assert_eq!(bodies, (2..=15).collect::<Vec<u8>>());
        let rows = table["rows"].as_array().unwrap();
        assert_eq!(rows.len(), 9);

        let mut equal = 0;
        let mut mismatches = Vec::new();
        for row in rows {
            let will = row["will"].as_u64().unwrap() as u8;
            let cells = row["hit_points"].as_array().unwrap();
            assert_eq!(cells.len(), bodies.len());
            for (body, cell) in bodies.iter().zip(cells) {
                let printed = cell.as_i64().unwrap() as i16;
                let computed = hit_points(*body, will);
                if computed == printed {
                    equal += 1;
                } else {
                    mismatches.push((*body, will, printed, computed));
                }
            }
        }
        assert_eq!(mismatches, [], "(BODY, WILL, printed, computed)");
        assert_eq!(equal, 126);
    }

    #[test]
    fn seriously_wounded_is_half_hp_rounded_up() {
        assert_eq!(seriously_wounded(55), 28);
        assert_eq!(seriously_wounded(40), 20);
        assert_eq!(seriously_wounded(25), 13);
    }

    #[test]
    fn current_emp_lowers_with_the_tens_place_of_humanity_and_stops_at_zero() {
        // p081: "a Character with 44 Humanity has an EMP of 4 until their Humanity is lowered
        // to 39, at which point their EMP lowers to 3"; A-4: negative Humanity gives EMP 0.
        let got: Vec<i16> = [44, 39, 50, 46, -3].into_iter().map(current_emp).collect();
        assert_eq!(got, [4, 3, 5, 4, 0]);
        assert_eq!(current_emp(-15), 0);
        assert_eq!(current_emp(0), 0);
    }

    #[test]
    fn humanity_max_is_ten_per_emp_point_as_the_p081_table_prints() {
        let printed = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        let got: Vec<i16> = (1..=10).map(humanity_max).collect();
        assert_eq!(got, printed);
    }

    fn sample_input() -> CprCharacterInput {
        let mut stats = CprStats::default();
        for (stat, value) in [
            (CprStat::Int, 6),
            (CprStat::Ref, 8),
            (CprStat::Dex, 7),
            (CprStat::Tech, 5),
            (CprStat::Cool, 6),
            (CprStat::Will, 7),
            (CprStat::Luck, 5),
            (CprStat::Move, 6),
            (CprStat::Body, 8),
            (CprStat::Emp, 5),
        ] {
            stats.set(stat, value);
        }
        CprCharacterInput { stats }
    }

    #[test]
    fn derived_rows_are_cpr_rows_with_the_book_values() {
        // BODY 8 + WILL 7 = 15, averaged and rounded up = 8 -> HP 10 + 5 x 8 = 50 (p080 table
        // row WILL 7, column BODY 8 prints 50).
        let rows = derived_rows(&sample_input(), 46);
        let pairs: Vec<(&str, i16)> = rows.iter().map(|r| (r.id.as_str(), r.value)).collect();
        assert_eq!(
            pairs,
            [
                ("cpr.derived.hit_points", 50),
                ("cpr.derived.seriously_wounded", 25),
                ("cpr.derived.death_save", 8),
                ("cpr.derived.humanity_max", 50),
                ("cpr.derived.emp_current", 4),
                ("cpr.derived.move", 6),
                ("cpr.derived.luck_pool", 5),
            ]
        );
        assert!(rows.iter().all(|r| r.id.starts_with("cpr.")));
        assert_eq!(
            rows[0].detail,
            "HP 50 = 10 + 5 x ceil((BODY 8 + WILL 7) / 2) = 10 + 5 x 8 (Cyberpunk RED core rulebook p. 79)"
        );
    }
}
