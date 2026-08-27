/** Browser-only preview of path checks. Audio bytes and tags are intentionally not read. */
export function lintPlaylist(playlistText, libraryText, caseSensitive = true) {
  const library = libraryText.split(/\r?\n/).map(cleanPath).filter(Boolean);
  const exact = new Map(library.map((path) => [path, path]));
  const normalized = indexBy(library, (path) => normalize(path, !caseSensitive));
  const folded = indexBy(library, (path) => normalize(path, true));
  const lines = playlistText.split(/\r?\n/);
  const findings = [];
  const corrected = [...lines];
  let entries = 0;
  let resolved = 0;

  lines.forEach((raw, index) => {
    const path = raw.trim();
    if (!path || path.startsWith("#")) return;
    entries += 1;
    if (/^[a-z][a-z\d+.-]*:\/\//i.test(path)) {
      findings.push(item(index + 1, "remote_entry", "notice", path, "Remote entry skipped. The CLI never contacts music services."));
      return;
    }
    const candidate = cleanPath(path);
    if (candidate.split("/").includes("..")) {
      findings.push(item(index + 1, "outside_root", "error", path, "Path traverses outside the selected library root."));
      return;
    }
    if (exact.has(candidate)) { resolved += 1; return; }
    const bucket = normalized.get(normalize(candidate, !caseSensitive)) || (!caseSensitive ? undefined : folded.get(normalize(candidate, true)));
    if (!bucket?.length) {
      findings.push(item(index + 1, "missing_path", "error", path, "No known library path matches, including Unicode and configured case variants."));
    } else if (bucket.length > 1) {
      findings.push({ ...item(index + 1, "ambiguous_path", "error", path, "More than one library path is equivalent; no correction was guessed."), candidates: bucket });
    } else {
      resolved += 1;
      const actual = bucket[0];
      const sameNfc = candidate.normalize("NFC") === actual.normalize("NFC");
      findings.push(item(index + 1, sameNfc ? "unicode_normalization" : "case_mismatch", "warning", path, sameNfc ? "Unicode normalization differs from the library." : "Letter case differs from the library."));
      corrected[index] = path.includes("\\") && !path.includes("/") ? actual.replaceAll("/", "\\") : actual;
    }
  });
  if (!entries) findings.push(item(0, "empty_playlist", "warning", "", "No track entries found. Paste an M3U path or load the sample."));
  return { schema_version: 1, clean: findings.length === 0, summary: { entries, resolved, findings: findings.length }, findings, corrected: corrected.join(detectEnding(playlistText)) };
}

function indexBy(values, keyFn) {
  const map = new Map();
  for (const value of values) {
    const key = keyFn(value);
    map.set(key, [...(map.get(key) || []), value]);
  }
  return map;
}

function cleanPath(path) {
  return path.trim().replaceAll("\\", "/").replace(/^\.\//, "").replace(/\/{2,}/g, "/");
}

function normalize(path, fold) {
  const nfc = path.normalize("NFC");
  return fold ? nfc.toLocaleLowerCase("und") : nfc;
}

function item(line, code, severity, path, message) {
  return { line, code, severity, path, message, candidates: [] };
}

function detectEnding(text) { return text.includes("\r\n") ? "\r\n" : "\n"; }
