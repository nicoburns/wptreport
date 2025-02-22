use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WptReport {
    pub time_start: u64,
    pub time_end: u64,
    pub run_info: RunInfo,
    pub results: Vec<TestResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WptScores {
    pub run_info: RunInfo,
    pub test_scores: BTreeMap<String, TestScore>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestScore {
    pub score: u64,
    pub subtests: BTreeMap<String, SubtestScore>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubtestScore {
    pub score: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunInfo {
    /// The browser engine tested (e.g. "servo")
    product: String,
    /// The code revision that the test was run against
    revision: String,

    // Flags
    automation: bool,
    debug: bool,
    display: Option<String>,
    has_sandbox: bool,
    headless: bool,
    verify: bool,
    wasm: bool,

    /// The OS that the tests were run on (e.g. "macos")
    os: String,
    /// OS version number
    os_version: String,
    /// OS version String
    version: String,
    /// The processor architecture the tests were run on (e.g. "arm")
    processor: String,
    /// The number of bits that the processor has (e.g. 64 for x86_64)
    bits: i64,
    /// The Python version used to run the tests
    python_version: i64,

    // OS Flags
    #[serde(default)]
    apple_catalina: bool,
    #[serde(default)]
    apple_silicon: bool,
    #[serde(default)]
    win10_2004: bool,
    #[serde(default)]
    win10_2009: bool,
    #[serde(default)]
    win11_2009: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestResult {
    pub test: String,
    pub status: TestStatus,
    pub duration: i64,
    pub known_intermittent: Vec<String>,
    pub message: Option<String>,
    pub subsuite: String,
    pub subtests: Vec<SubtestResult>,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TestStatus {
    Pass,
    Fail,
    Ok,
    Error,
    Timeout,
    Crash,
    Assert,
    PreconditionFailed,
    Skip,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubtestResult {
    pub name: String,
    pub status: SubtestStatus,
    pub known_intermittent: Vec<String>,
    pub message: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubtestStatus {
    Pass,
    Fail,
    Error,
    Timeout,
    Assert,
    PreconditionFailed,
    Notrun,
    Skip,
}
