use crate::encoding::PlaylistText;
use crate::{Finding, LintError, LintOptions, Report, Severity, Summary};
use lofty::file::TaggedFileExt;
use lofty::probe::Probe;
use lofty::tag::ItemKey;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Component, Path, PathBuf};
use unicode_normalization::UnicodeNormalization;
use walkdir::WalkDir;

#[derive(Clone)]
struct FileRecord {
    absolute: PathBuf,
    relative: String,
}

struct LibraryIndex {
    files: Vec<FileRecord>,
    exact: HashMap<String, usize>,
    nfc: HashMap<String, Vec<usize>>,
    folded: HashMap<String, Vec<usize>>,
}

impl LibraryIndex {
    fn build(root: &Path) -> Result<Self, LintError> {
        let mut files = Vec::new();
        for entry in WalkDir::new(root).follow_links(false) {
            let entry =
                entry.map_err(|error| LintError(format!("could not scan library: {error}")))?;
            if !entry.file_type().is_file() {
                continue;
            }
            let relative = entry
                .path()
                .strip_prefix(root)
                .map_err(|_| LintError("library scan escaped its root".into()))?;
            let relative = slash_path(relative);
            files.push(FileRecord {
                absolute: entry.path().to_path_buf(),
                relative,
            });
        }
        files.sort_by(|a, b| a.relative.cmp(&b.relative));
        let mut exact = HashMap::new();
        let mut nfc: HashMap<String, Vec<usize>> = HashMap::new();
        let mut folded: HashMap<String, Vec<usize>> = HashMap::new();
        for (index, file) in files.iter().enumerate() {
            exact.insert(file.relative.clone(), index);
            nfc.entry(normalize(&file.relative, false))
                .or_default()
                .push(index);
            folded
                .entry(normalize(&file.relative, true))
                .or_default()
                .push(index);
        }
        Ok(Self {
            files,
            exact,
            nfc,
            folded,
        })
    }

    fn match_path(&self, relative: &str, insensitive: bool) -> Match {
        if insensitive {
            if let Some(indexes) = self.folded.get(&normalize(relative, true)) {
                if indexes.len() > 1 {
                    return Match::Ambiguous(indexes.clone());
                }
            }
        }
        if let Some(index) = self.exact.get(relative) {
            return Match::Exact(*index);
        }
        let key = normalize(relative, insensitive);
        let indexes = if insensitive {
            self.folded.get(&key)
        } else {
            self.nfc.get(&key)
        };
        if let Some(indexes) = indexes {
            return match indexes.as_slice() {
                [index] => Match::Equivalent(*index),
                indexes => Match::Ambiguous(indexes.to_vec()),
            };
        }
        if !insensitive {
            if let Some(indexes) = self.folded.get(&normalize(relative, true)) {
                return match indexes.as_slice() {
                    [index] => Match::Equivalent(*index),
                    indexes => Match::Ambiguous(indexes.to_vec()),
                };
            }
        }
        Match::Missing
    }
}

enum Match {
    Exact(usize),
    Equivalent(usize),
    Ambiguous(Vec<usize>),
    Missing,
}

struct ParsedLine<'a> {
    body: &'a str,
    ending: &'a str,
}

/// Lint one playlist. Media and the source playlist are always read-only.
pub fn lint_playlist(options: &LintOptions) -> Result<Report, LintError> {
    let root = fs::canonicalize(&options.library_root).map_err(|error| {
        LintError(format!(
            "could not open library root {}: {error}",
            options.library_root.display()
        ))
    })?;
    if !root.is_dir() {
        return Err(LintError(format!(
            "library root is not a directory: {}",
            root.display()
        )));
    }
    let bytes = fs::read(&options.playlist).map_err(|error| {
        LintError(format!(
            "could not read playlist {}: {error}",
            options.playlist.display()
        ))
    })?;
    let strict_utf8 = options
        .playlist
        .extension()
        .and_then(|part| part.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("m3u8"));
    let decoded = PlaylistText::decode(&bytes, strict_utf8)?;
    let index = LibraryIndex::build(&root)?;
    let insensitive = options.case_mode.is_insensitive();
    let playlist_parent =
        fs::canonicalize(options.playlist.parent().unwrap_or(Path::new("."))).ok();
    let parent_relative = playlist_parent
        .as_deref()
        .and_then(|parent| parent.strip_prefix(&root).ok())
        .map(slash_path);

    let lines = split_lines(&decoded.text);
    let mut output = String::with_capacity(decoded.text.len());
    let mut findings = Vec::new();
    let mut summary = Summary::default();
    let mut checked_dates = HashSet::new();

    for (offset, line) in lines.iter().enumerate() {
        let line_number = offset + 1;
        let trimmed = line.body.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            output.push_str(line.body);
            output.push_str(line.ending);
            continue;
        }
        summary.entries += 1;
        if is_remote(trimmed) {
            summary.skipped += 1;
            findings.push(finding(
                line_number,
                "remote_entry",
                Severity::Notice,
                trimmed,
                "Remote entry was skipped; this tool never contacts music services.",
            ));
            output.push_str(line.body);
            output.push_str(line.ending);
            continue;
        }

        let relative_candidates = match entry_candidates(trimmed, &root, parent_relative.as_deref())
        {
            Ok(candidates) => candidates,
            Err(message) => {
                summary.missing += 1;
                findings.push(finding(
                    line_number,
                    "outside_root",
                    Severity::Error,
                    trimmed,
                    &message,
                ));
                output.push_str(line.body);
                output.push_str(line.ending);
                continue;
            }
        };
        let mut matches = Vec::new();
        for (candidate, from_parent) in &relative_candidates {
            match index.match_path(candidate, insensitive) {
                Match::Exact(file) => matches.push((file, true, candidate.clone(), *from_parent)),
                Match::Equivalent(file) => {
                    matches.push((file, false, candidate.clone(), *from_parent))
                }
                Match::Ambiguous(files) => {
                    for file in files {
                        matches.push((file, false, candidate.clone(), *from_parent));
                    }
                }
                Match::Missing => {}
            }
        }
        matches.sort_by_key(|(file, exact, _, from_parent)| (!*exact, !*from_parent, *file));
        matches.dedup_by_key(|(file, _, _, _)| *file);

        if matches.is_empty() {
            summary.missing += 1;
            findings.push(finding(line_number, "missing_path", Severity::Error, trimmed, "No file in the library matches this path, including NFC/NFD and configured case variants."));
            output.push_str(line.body);
            output.push_str(line.ending);
            continue;
        }
        if matches.len() > 1 {
            summary.ambiguous += 1;
            let mut item = finding(
                line_number,
                "ambiguous_path",
                Severity::Error,
                trimmed,
                "More than one library file is equivalent; no correction was guessed.",
            );
            item.candidates = matches
                .iter()
                .map(|(file, _, _, _)| index.files[*file].relative.clone())
                .collect();
            findings.push(item);
            output.push_str(line.body);
            output.push_str(line.ending);
            continue;
        }

        let (file_index, exact, matched_input, from_parent) = &matches[0];
        let file = &index.files[*file_index];
        summary.resolved += 1;
        let mut replacement = None;
        if !exact || matched_input != &file.relative {
            let code = mismatch_code(trimmed, &file.relative);
            let message = match code {
                "unicode_normalization" => "Path text is canonically equivalent but uses different Unicode normalization; use the library spelling.",
                "case_mismatch" => "Path differs from the library only by letter case; this can fail on case-sensitive destinations.",
                _ => "Path resolves through a non-exact spelling; use the library path for portable imports.",
            };
            findings.push(finding(
                line_number,
                code,
                Severity::Warning,
                trimmed,
                message,
            ));
            summary.corrected += 1;
            replacement = Some(corrected_path(
                trimmed,
                &root,
                &file.relative,
                parent_relative.as_deref(),
                *from_parent,
            ));
        }

        if options.check_dates && checked_dates.insert(file.absolute.clone()) {
            if let Some(issue) = inspect_date(&file.absolute) {
                summary.date_issues += 1;
                findings.push(finding(
                    line_number,
                    "invalid_date_tag",
                    Severity::Warning,
                    trimmed,
                    &issue,
                ));
            }
        }
        output.push_str(replacement.as_deref().unwrap_or(line.body));
        output.push_str(line.ending);
    }

    if summary.entries == 0 {
        findings.push(finding(
            0,
            "empty_playlist",
            Severity::Warning,
            "",
            "The playlist contains no track entries; add at least one path before import.",
        ));
    }

    let fixed_output = if let Some(path) = &options.fixed_output {
        if same_file_target(&options.playlist, path) {
            return Err(LintError(
                "--fix must name a new file; the source playlist is never overwritten".into(),
            ));
        }
        let encoded = decoded.encode(&output)?;
        fs::write(path, encoded).map_err(|error| {
            LintError(format!(
                "could not write corrected playlist {}: {error}",
                path.display()
            ))
        })?;
        Some(path.display().to_string())
    } else {
        None
    };

    Ok(Report {
        schema_version: 1,
        playlist: options.playlist.display().to_string(),
        library_root: root.display().to_string(),
        case_sensitive: !insensitive,
        clean: findings.is_empty(),
        fixed_output,
        summary,
        findings,
    })
}

fn split_lines(text: &str) -> Vec<ParsedLine<'_>> {
    if text.is_empty() {
        return Vec::new();
    }
    text.split_inclusive('\n')
        .map(|line| {
            if let Some(body) = line.strip_suffix("\r\n") {
                ParsedLine {
                    body,
                    ending: "\r\n",
                }
            } else if let Some(body) = line.strip_suffix('\n') {
                ParsedLine { body, ending: "\n" }
            } else {
                ParsedLine {
                    body: line,
                    ending: "",
                }
            }
        })
        .collect()
}

fn entry_candidates(
    entry: &str,
    root: &Path,
    parent_relative: Option<&str>,
) -> Result<Vec<(String, bool)>, String> {
    let portable = entry.replace('\\', "/");
    let path = Path::new(&portable);
    if path.is_absolute() {
        let absolute = lexical_normalize(path)
            .ok_or_else(|| "Absolute path traverses above its filesystem root.".to_string())?;
        let relative = absolute
            .strip_prefix(root)
            .map_err(|_| "Absolute path is outside the selected library root.".to_string())?;
        return Ok(vec![(slash_path(relative), false)]);
    }
    if looks_like_windows_absolute(&portable) {
        return Err("Windows absolute path cannot be mapped to this library root; make it relative before import.".into());
    }
    let normalized = normalize_relative(&portable)
        .ok_or_else(|| "Relative path traverses outside the selected library root.".to_string())?;
    let mut candidates = vec![(normalized.clone(), false)];
    if let Some(parent) = parent_relative {
        if !parent.is_empty() {
            if let Some(joined) = normalize_relative(&format!("{parent}/{portable}")) {
                if joined != normalized {
                    candidates.insert(0, (joined, true));
                }
            }
        }
    }
    Ok(candidates)
}

fn normalize_relative(path: &str) -> Option<String> {
    let mut parts = Vec::new();
    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            value => parts.push(value),
        }
    }
    Some(parts.join("/"))
}

fn lexical_normalize(path: &Path) -> Option<PathBuf> {
    let mut result = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Prefix(prefix) => result.push(prefix.as_os_str()),
            Component::RootDir => result.push(Path::new("/")),
            Component::CurDir => {}
            Component::ParentDir => {
                if !result.pop() {
                    return None;
                }
            }
            Component::Normal(part) => result.push(part),
        }
    }
    Some(result)
}

fn corrected_path(
    original: &str,
    root: &Path,
    relative: &str,
    parent_relative: Option<&str>,
    from_parent: bool,
) -> String {
    let mut corrected = if Path::new(&original.replace('\\', "/")).is_absolute() {
        root.join(relative).display().to_string()
    } else if from_parent {
        relative_from(parent_relative.unwrap_or(""), relative)
    } else {
        relative.to_string()
    };
    if original.contains('\\') && !original.contains('/') {
        corrected = corrected.replace('/', "\\");
    }
    corrected
}

fn relative_from(directory: &str, target: &str) -> String {
    let from: Vec<_> = directory
        .split('/')
        .filter(|part| !part.is_empty())
        .collect();
    let to: Vec<_> = target.split('/').filter(|part| !part.is_empty()).collect();
    let common = from.iter().zip(&to).take_while(|(a, b)| a == b).count();
    let mut parts = vec![".."; from.len() - common];
    parts.extend_from_slice(&to[common..]);
    if parts.is_empty() {
        ".".into()
    } else {
        parts.join("/")
    }
}

fn mismatch_code(original: &str, actual: &str) -> &'static str {
    let original = original.replace('\\', "/");
    if original.nfc().collect::<String>() == actual.nfc().collect::<String>() {
        "unicode_normalization"
    } else if normalize(&original, true) == normalize(actual, true) {
        "case_mismatch"
    } else {
        "path_spelling"
    }
}

fn normalize(value: &str, fold_case: bool) -> String {
    let normalized: String = value.nfc().collect();
    if fold_case {
        normalized.to_lowercase()
    } else {
        normalized
    }
}

fn inspect_date(path: &Path) -> Option<String> {
    let tagged = Probe::open(path).ok()?.read().ok()?;
    let tag = tagged.primary_tag().or_else(|| tagged.first_tag())?;
    let max = current_year() + 1;
    for (label, key) in [
        ("year", ItemKey::Year),
        ("recording date", ItemKey::RecordingDate),
        ("original release date", ItemKey::OriginalReleaseDate),
    ] {
        for value in tag.get_strings(&key) {
            if !valid_date(value, max) {
                return Some(format!("{label} tag is {value:?}; expected YYYY, YYYY-MM, or YYYY-MM-DD with a year from 1000 through {max}."));
            }
        }
    }
    None
}

fn valid_date(value: &str, max_year: u32) -> bool {
    let text = value.trim();
    if text.len() < 4 || !text.as_bytes()[..4].iter().all(u8::is_ascii_digit) {
        return false;
    }
    let year = match text[..4].parse::<u32>() {
        Ok(year) => year,
        Err(_) => return false,
    };
    if !(1000..=max_year).contains(&year) {
        return false;
    }
    if text.len() == 4 {
        return true;
    }
    if text.as_bytes().get(4) != Some(&b'-') {
        return false;
    }
    let month = match text.get(5..7).and_then(|part| part.parse::<u8>().ok()) {
        Some(value @ 1..=12) => value,
        _ => return false,
    };
    if text.len() == 7 {
        return true;
    }
    if text.as_bytes().get(7) != Some(&b'-') {
        return false;
    }
    let day = match text.get(8..10).and_then(|part| part.parse::<u8>().ok()) {
        Some(value) => value,
        None => return false,
    };
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    if day == 0 || day > days[usize::from(month - 1)] {
        return false;
    }
    // The public contract deliberately accepts date-only values. Do not accept
    // a timestamp prefix without parsing its entire timestamp: silently
    // allowing trailing text would let malformed tags pass into an importer.
    text.len() == 10
}

fn current_year() -> u32 {
    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let z = (seconds / 86_400) as i64 + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let mut year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let month = mp + if mp < 10 { 3 } else { -9 };
    if month <= 2 {
        year += 1;
    }
    year as u32
}

fn slash_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
fn looks_like_windows_absolute(path: &str) -> bool {
    path.as_bytes().get(1) == Some(&b':') || path.starts_with("//")
}
fn is_remote(path: &str) -> bool {
    path.contains("://") && !looks_like_windows_absolute(path)
}
fn same_file_target(source: &Path, target: &Path) -> bool {
    if source == target {
        return true;
    }
    match (fs::canonicalize(source), fs::canonicalize(target)) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    }
}
fn finding(line: usize, code: &str, severity: Severity, path: &str, message: &str) -> Finding {
    Finding {
        line,
        code: code.into(),
        severity,
        path: path.into(),
        message: message.into(),
        candidates: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative_normalization_rejects_escape() {
        assert_eq!(
            normalize_relative("A/../B/song.mp3").as_deref(),
            Some("B/song.mp3")
        );
        assert_eq!(normalize_relative("../song.mp3"), None);
    }

    #[test]
    fn line_endings_are_identified() {
        let lines = split_lines("a\r\nb\nlast");
        assert_eq!((lines[0].body, lines[0].ending), ("a", "\r\n"));
        assert_eq!((lines[2].body, lines[2].ending), ("last", ""));
    }

    #[test]
    fn validates_real_calendar_dates() {
        assert!(valid_date("2024-02-29", 2027));
        assert!(valid_date("1999", 2027));
        assert!(!valid_date("0000", 2027));
        assert!(!valid_date("2023-02-29", 2027));
        assert!(!valid_date("2024-01-01Tbogus", 2027));
        assert!(!valid_date("2024-01-01 trailing text", 2027));
        assert!(!valid_date("twenty", 2027));
    }

    #[test]
    fn derives_current_year() {
        assert!((2026..=2100).contains(&current_year()));
    }

    #[test]
    fn keeps_playlist_relative_corrections_relative() {
        assert_eq!(
            relative_from("playlists/live", "Artist/Album/song.mp3"),
            "../../Artist/Album/song.mp3"
        );
        assert_eq!(
            relative_from("sets", "sets/Artist/song.mp3"),
            "Artist/song.mp3"
        );
    }
}
