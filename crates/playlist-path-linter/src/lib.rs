//! Library core for Playlist Path Linter.
//!
//! The public surface is deliberately small: configure [`LintOptions`] and call
//! [`lint_playlist`]. The function never modifies the source playlist or media.

mod encoding;
mod lint;
mod model;

pub use lint::lint_playlist;
pub use model::{CaseMode, Finding, LintError, LintOptions, Report, Severity, Summary};
