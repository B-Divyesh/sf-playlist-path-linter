use serde::Serialize;
use std::fmt;
use std::path::PathBuf;

/// Filesystem case behavior to model while resolving playlist entries.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CaseMode {
    Auto,
    Sensitive,
    Insensitive,
}

impl CaseMode {
    pub(crate) fn is_insensitive(self) -> bool {
        match self {
            Self::Auto => cfg!(windows) || cfg!(target_os = "macos"),
            Self::Sensitive => false,
            Self::Insensitive => true,
        }
    }
}

/// Options for one lint pass.
#[derive(Clone, Debug)]
pub struct LintOptions {
    pub playlist: PathBuf,
    pub library_root: PathBuf,
    pub fixed_output: Option<PathBuf>,
    pub case_mode: CaseMode,
    pub check_dates: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Notice,
}

/// One stable, line-addressed diagnosis.
#[derive(Clone, Debug, Serialize)]
pub struct Finding {
    pub line: usize,
    pub code: String,
    pub severity: Severity,
    pub path: String,
    pub message: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub candidates: Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Summary {
    pub entries: usize,
    pub resolved: usize,
    pub corrected: usize,
    pub missing: usize,
    pub ambiguous: usize,
    pub date_issues: usize,
    pub skipped: usize,
}

/// Versioned machine-readable result.
#[derive(Clone, Debug, Serialize)]
pub struct Report {
    pub schema_version: u8,
    pub playlist: String,
    pub library_root: String,
    pub case_sensitive: bool,
    pub clean: bool,
    pub fixed_output: Option<String>,
    pub summary: Summary,
    pub findings: Vec<Finding>,
}

#[derive(Debug)]
pub struct LintError(pub String);

impl fmt::Display for LintError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for LintError {}

impl From<std::io::Error> for LintError {
    fn from(value: std::io::Error) -> Self {
        Self(value.to_string())
    }
}
