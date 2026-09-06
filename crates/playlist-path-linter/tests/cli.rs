use playlist_path_linter::{lint_playlist, CaseMode, LintOptions};
use std::fs;
use std::process::Command;
use tempfile::tempdir;

fn riff_with_icrd(value: &str) -> Vec<u8> {
    let value_size = value.len() + 1;
    let padding = value_size % 2;
    let mut bytes = b"RIFF".to_vec();
    bytes.extend_from_slice(&64_u32.to_le_bytes());
    bytes.extend_from_slice(b"WAVEfmt ");
    bytes.extend_from_slice(&16_u32.to_le_bytes());
    bytes.extend_from_slice(&1_u16.to_le_bytes());
    bytes.extend_from_slice(&1_u16.to_le_bytes());
    bytes.extend_from_slice(&8000_u32.to_le_bytes());
    bytes.extend_from_slice(&16000_u32.to_le_bytes());
    bytes.extend_from_slice(&2_u16.to_le_bytes());
    bytes.extend_from_slice(&16_u16.to_le_bytes());
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&2_u32.to_le_bytes());
    bytes.extend_from_slice(&[0, 0]);
    bytes.extend_from_slice(b"LIST");
    bytes.extend_from_slice(&((12 + value_size + padding) as u32).to_le_bytes());
    bytes.extend_from_slice(b"INFOICRD");
    bytes.extend_from_slice(&(value_size as u32).to_le_bytes());
    bytes.extend_from_slice(value.as_bytes());
    bytes.push(0);
    if padding != 0 {
        bytes.push(0);
    }
    bytes
}

#[test]
fn diagnoses_unicode_missing_and_writes_a_corrected_copy() {
    let temp = tempdir().unwrap();
    let library = temp.path().join("music");
    fs::create_dir_all(library.join("Beyoncé")).unwrap();
    fs::write(
        library.join("Beyoncé/Halo.mp3"),
        b"not audio but path-valid",
    )
    .unwrap();
    let playlist = temp.path().join("set.m3u8");
    fs::write(
        &playlist,
        "#EXTM3U\nBeyonce\u{301}/Halo.mp3\nLost/Song.mp3\n",
    )
    .unwrap();
    let fixed = temp.path().join("set.fixed.m3u8");

    let report = lint_playlist(&LintOptions {
        playlist: playlist.clone(),
        library_root: library,
        fixed_output: Some(fixed.clone()),
        case_mode: CaseMode::Sensitive,
        check_dates: true,
    })
    .unwrap();

    assert_eq!(report.summary.entries, 2);
    assert_eq!(report.summary.resolved, 1);
    assert_eq!(report.summary.missing, 1);
    assert!(report
        .findings
        .iter()
        .any(|item| item.code == "unicode_normalization"));
    assert!(report
        .findings
        .iter()
        .any(|item| item.code == "missing_path"));
    assert!(fs::read_to_string(fixed)
        .unwrap()
        .contains("Beyoncé/Halo.mp3"));
    assert!(fs::read_to_string(playlist)
        .unwrap()
        .contains("Beyonce\u{301}"));
}

#[test]
fn reports_ambiguous_case_variants_without_guessing() {
    let temp = tempdir().unwrap();
    let library = temp.path().join("music");
    fs::create_dir_all(library.join("Artist")).unwrap();
    fs::write(library.join("Artist/SONG.mp3"), b"").unwrap();
    fs::write(library.join("Artist/Song.mp3"), b"").unwrap();
    let playlist = temp.path().join("set.m3u");
    fs::write(&playlist, "Artist/song.mp3\n").unwrap();

    let report = lint_playlist(&LintOptions {
        playlist,
        library_root: library,
        fixed_output: None,
        case_mode: CaseMode::Insensitive,
        check_dates: false,
    })
    .unwrap();
    assert_eq!(report.summary.ambiguous, 1);
    assert_eq!(report.findings[0].candidates.len(), 2);
}

#[test]
fn empty_playlist_is_not_reported_clean() {
    let temp = tempdir().unwrap();
    let library = temp.path().join("music");
    fs::create_dir(&library).unwrap();
    let playlist = temp.path().join("empty.m3u");
    fs::write(&playlist, "#EXTM3U\n").unwrap();
    let report = lint_playlist(&LintOptions {
        playlist,
        library_root: library,
        fixed_output: None,
        case_mode: CaseMode::Auto,
        check_dates: false,
    })
    .unwrap();
    assert!(!report.clean);
    assert_eq!(report.findings[0].code, "empty_playlist");
}

#[test]
fn refuses_to_overwrite_source() {
    let temp = tempdir().unwrap();
    let library = temp.path().join("music");
    fs::create_dir(&library).unwrap();
    let playlist = temp.path().join("set.m3u");
    fs::write(&playlist, "missing.mp3\n").unwrap();
    let error = lint_playlist(&LintOptions {
        playlist: playlist.clone(),
        library_root: library,
        fixed_output: Some(playlist),
        case_mode: CaseMode::Auto,
        check_dates: false,
    })
    .unwrap_err();
    assert!(error.to_string().contains("never overwritten"));
}

#[test]
fn catches_year_zero_in_a_riff_date_tag() {
    let temp = tempdir().unwrap();
    let library = temp.path().join("music");
    fs::create_dir(&library).unwrap();
    let track = library.join("dated.wav");
    fs::write(&track, riff_with_icrd("0000")).unwrap();
    let playlist = temp.path().join("dated.m3u8");
    fs::write(&playlist, "dated.wav\n").unwrap();

    let report = lint_playlist(&LintOptions {
        playlist,
        library_root: library,
        fixed_output: None,
        case_mode: CaseMode::Sensitive,
        check_dates: true,
    })
    .unwrap();
    assert!(report
        .findings
        .iter()
        .any(|item| item.code == "invalid_date_tag"));
    assert_eq!(report.summary.date_issues, 1);
}

#[test]
fn cli_reports_malformed_riff_timestamp_in_json() {
    let temp = tempdir().unwrap();
    let library = temp.path().join("library");
    fs::create_dir(&library).unwrap();
    fs::write(
        library.join("dated.wav"),
        riff_with_icrd("2024-01-01Tbogus"),
    )
    .unwrap();
    let playlist = temp.path().join("dated.m3u8");
    fs::write(&playlist, "dated.wav\n").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_playlist-path-linter"))
        .args([
            "lint",
            playlist.to_str().unwrap(),
            "--root",
            library.to_str().unwrap(),
            "--case",
            "sensitive",
            "--json",
        ])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(1));
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["clean"], false);
    assert_eq!(report["summary"]["date_issues"], 1);
    assert!(report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .any(|finding| {
            finding["code"] == "invalid_date_tag" && finding["path"] == "dated.wav"
        }));
}

#[test]
fn bundled_demo_reports_known_faults_and_keeps_its_source() {
    let output = Command::new(env!("CARGO_BIN_EXE_playlist-path-linter"))
        .arg("demo")
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("[unicode_normalization]"));
    assert!(stdout.contains("[case_mismatch]"));
    assert!(stdout.contains("[missing_path]"));
    let location = stdout
        .lines()
        .find_map(|line| line.strip_prefix("Demo files kept at: "))
        .expect("demo should name its retained workspace");
    let workspace = std::path::PathBuf::from(location);
    assert!(workspace.join("sample.fixed.m3u8").is_file());
    let source = fs::read_to_string(workspace.join("sample.m3u8")).unwrap();
    assert!(source.contains("Beyonce\u{301}"));
    fs::remove_dir_all(workspace).unwrap();
}
