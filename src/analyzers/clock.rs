use crate::prelude::*;

const IMPL_DISPLAY: &str = "Nice work using `impl Display` to implement to_string.";

const JUST_STORE_MINUTES: &str =
    "(Some people don't bother storing the hours in the struct which simplifies things a bit.)";

const DERIVE_PARTIAL_EQ: &str = "Equality can be derived automatically (`#[derive(PartialEq)]`).";
const DERIVE_DEBUG: &str = "Debug can be derived automatically (`#[derive(Debug)]`).";

const REM_EUCLID: &str =
    "Alternatively to `%` and `/` you can use `rem_euclid` and `div_euclid` which \
    [differ](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=617965fa5096580c67b809e0bc786917) \
    when negatives are involved.";

const SUGGEST_IMPL_DISPLAY: &str = "Rather than `impl ToString` \
    it's better to `impl Display` \
    and then we will get `to_string` for free.";

const CELEBRATE_REM_EUCLID: &str = "Nice work using rem_euclid!";

const IMPROVE_FORMAT_STRING: &str = "Have a look at \
    [std::fmt](https://doc.rust-lang.org/std/fmt/#width) \
    for a more succinct way to format the number.";

pub static LINTS: &[fn(&str) -> Option<(i32, String)>] = &[
    good!("Display for Clock" => IMPL_DISPLAY),
    good!("rem_euclid" => CELEBRATE_REM_EUCLID),
    bad_if_missing!("2}:{:0" => IMPROVE_FORMAT_STRING),
    bad!("Debug for Clock" => DERIVE_DEBUG),
    bad!("PartialEq for Clock" => DERIVE_PARTIAL_EQ),
    bad!("ToString for Clock" => SUGGEST_IMPL_DISPLAY),
    note_if_missing!("rem_euclid" => REM_EUCLID),
    note!("hours: i32," => JUST_STORE_MINUTES),
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
                    JUST_STORE_MINUTES.into(),
                    REM_EUCLID.to_string(),
                    DERIVE_PARTIAL_EQ.into(),
                ],
            ),
        );
    }
}
