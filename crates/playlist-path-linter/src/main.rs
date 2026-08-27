use clap::{Parser, Subcommand, ValueEnum};
use playlist_path_linter::{lint_playlist, CaseMode, LintOptions, Severity};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "playlist-path-linter", version, about = "Catch path, Unicode, and date corruption before importing M3U playlists", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Check a playlist against a local music library
    Lint {
        /// M3U or M3U8 playlist to inspect
        playlist: PathBuf,
        /// Root directory containing the destination music library
        #[arg(long, short = 'r', value_name = "DIRECTORY")]
        root: PathBuf,
        /// Write an unambiguously corrected copy (the source is never changed)
        #[arg(long, value_name = "PLAYLIST")]
        fix: Option<PathBuf>,
        /// Print the versioned report as JSON
        #[arg(long)]
        json: bool,
        /// Destination filesystem case behavior
        #[arg(long, value_enum, default_value_t = CaseArg::Auto)]
        case: CaseArg,
        /// Skip media date-tag inspection
        #[arg(long)]
        no_date_check: bool,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum CaseArg {
    Auto,
    Sensitive,
    Insensitive,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Command::Lint {
            playlist,
            root,
            fix,
            json,
            case,
            no_date_check,
        } => {
            let options = LintOptions {
                playlist,
                library_root: root,
                fixed_output: fix,
                case_mode: match case {
                    CaseArg::Auto => CaseMode::Auto,
                    CaseArg::Sensitive => CaseMode::Sensitive,
                    CaseArg::Insensitive => CaseMode::Insensitive,
                },
                check_dates: !no_date_check,
            };
            match lint_playlist(&options) {
                Ok(report) => {
                    let has_findings = !report.clean;
                    if json {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&report)
                                .expect("report serialization is infallible")
                        );
                    } else {
                        print_human(&report);
                    }
                    if has_findings {
                        ExitCode::from(1)
                    } else {
                        ExitCode::SUCCESS
                    }
                }
                Err(error) => {
                    eprintln!("playlist-path-linter: {error}");
                    ExitCode::from(2)
                }
            }
        }
    }
}

fn print_human(report: &playlist_path_linter::Report) {
    println!("Playlist Path Linter");
    println!("  playlist  {}", report.playlist);
    println!("  library   {}", report.library_root);
    println!(
        "  model     case-{}",
        if report.case_sensitive {
            "sensitive"
        } else {
            "insensitive"
        }
    );
    println!();
    if report.findings.is_empty() {
        println!("✓ Clean — {} entries resolved.", report.summary.resolved);
    } else {
        for item in &report.findings {
            let mark = match item.severity {
                Severity::Error => "error",
                Severity::Warning => "warn ",
                Severity::Notice => "note ",
            };
            println!("{mark} L{} [{}] {}", item.line, item.code, item.path);
            println!("      {}", item.message);
            for candidate in &item.candidates {
                println!("      candidate: {candidate}");
            }
        }
        println!();
        println!(
            "{} entries · {} resolved · {} missing · {} ambiguous · {} date issues",
            report.summary.entries,
            report.summary.resolved,
            report.summary.missing,
            report.summary.ambiguous,
            report.summary.date_issues
        );
    }
    if let Some(path) = &report.fixed_output {
        println!("Corrected copy: {path}");
    }
}
