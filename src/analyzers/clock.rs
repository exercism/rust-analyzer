use crate::prelude::*;

const IMPL_DISPLAY: &str = "rust.clock.impl_display_used";
const ONLY_STORE_MINUTES: &str = "rust.clock.only_store_minutes";
const DERIVE_PARTIAL_EQ: &str = "rust.clock.derive_partial_eq";
const DERIVE_DEBUG: &str = "rust.clock.derive_debug";
const USE_REM_EUCLID: &str = "rust.clock.use_rem_euclid";
const SUGGEST_IMPL_DISPLAY: &str = "rust.clock.use_impl_display";
const REM_EUCLID_USED: &str = "rust.clock.rem_euclid_used";
const IMPROVE_FORMAT_STRING: &str = "rust.clock.improve_format_string";

// Order here will be order displayed in to user.
pub static LINTS: &[Lint] = &[
    good!("Display for Clock" => IMPL_DISPLAY),
    good!("rem_euclid" => REM_EUCLID_USED),
    bad_if_missing!(":02" | ":0>2" | ":>02" => IMPROVE_FORMAT_STRING),
    bad!("Debug for Clock" => DERIVE_DEBUG),
    bad!("PartialEq for Clock" => DERIVE_PARTIAL_EQ),
    bad!("ToString for Clock" => SUGGEST_IMPL_DISPLAY),
    note_if_missing!("rem_euclid" => USE_REM_EUCLID),
    note!("hours: i32," => ONLY_STORE_MINUTES),
];

pub struct ClockAnalyzer;

impl Analyze for ClockAnalyzer {
    fn analyze(&self, _solution_ast: &File, solution_raw: &str) -> Result<AnalysisOutput> {
        // pass threshold set high as we've not got enough checks to auto-approve yet.
        self.run_lints("add_minutes(", solution_raw, LINTS, 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzers::output::AnalysisStatus;

    fn test_analyzer_output(solution_ast: &File, solution_raw: &str, expected: AnalysisOutput) {
        assert_eq!(
            ClockAnalyzer.analyze(solution_ast, solution_raw).ok(),
            Some(expected)
        )
    }

    #[test]
    fn approve_and_well_done_for_using_underscores_in_literal() {
        let solution = r#"
use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

const HOURS_IN_A_DAY: i32 = 24 * 60;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // stop overflows of hours and minutes, but capping it
        let mut total_minutes = (hours * 60 + minutes) % HOURS_IN_A_DAY;
        // if too low, bring up to positives
        if total_minutes < 0 {
            total_minutes += HOURS_IN_A_DAY;
        }

        Clock {
            hours: total_minutes / 60,
            minutes: total_minutes % 60
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}
            "#;
        test_analyzer_output(
            &syn::parse_str::<File>(solution).unwrap(),
            solution,
            AnalysisOutput::new(
                AnalysisStatus::ReferToMentor,
                vec![
                    IMPL_DISPLAY.into(),
                    DERIVE_PARTIAL_EQ.into(),
                    USE_REM_EUCLID.to_string(),
                    ONLY_STORE_MINUTES.into(),
                ],
            ),
        );
    }
}
