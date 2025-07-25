//! A score summary file as used
use serde::{Deserialize, Serialize};

use crate::AreaScores;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreSummaryReport {
    pub focus_areas: Vec<String>,
    pub runs: Vec<RunSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusArea {
    pub name: String,
    pub areas: Vec<String>,
}
impl From<&str> for FocusArea {
    fn from(value: &str) -> Self {
        Self {
            name: value.to_string(),
            areas: vec![value.to_string()],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunSummary {
    /// The date the run occured in YYYY-MM-DD format
    pub date: String,
    /// The version of the WPT test suite that was run
    pub wpt_revision: String,
    /// The version of the browser that was tested
    pub product_revision: String,
    /// Scores are a percentage expressed a number between 0 and 1000
    pub scores: Vec<RunScores>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct RunScores {
    // pub interop_score: u16,
    pub total_tests: u32,
    #[serde(serialize_with = "as_int_if_int")]
    pub total_score: f64, // Servo score
    // pub total_tests_passed: u32,
    pub total_subtests: u32,
    pub total_subtests_passed: u32,
}

impl From<AreaScores> for RunScores {
    fn from(scores: AreaScores) -> Self {
        Self {
            // interop_score: scores.interop_score(),
            total_tests: scores.tests.total,
            total_score: scores.servo_score(),
            // total_tests_passed: scores.tests.pass,
            total_subtests: scores.subtests.total,
            total_subtests_passed: scores.subtests.pass,
        }
    }
}

/// Remove redundant decimal places
fn as_int_if_int<S>(x: &f64, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if x.fract() == 0.0 {
        s.serialize_u64(*x as u64)
    } else {
        s.serialize_f64(*x)
    }
}
