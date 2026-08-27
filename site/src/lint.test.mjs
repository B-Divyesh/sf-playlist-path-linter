import test from "node:test";
import assert from "node:assert/strict";
import { lintPlaylist } from "./lint.mjs";

test("diagnoses normalization, case, and missing paths", () => {
  const report = lintPlaylist("Beyonce\u0301/Halo.flac\nTalk TALK/Spirit.mp3\nLost.mp3", "Beyoncé/Halo.flac\nTalk Talk/Spirit.mp3", true);
  assert.deepEqual(report.findings.map(({ code }) => code), ["unicode_normalization", "case_mismatch", "missing_path"]);
  assert.match(report.corrected, /Beyoncé\/Halo/);
});

test("does not guess ambiguous paths", () => {
  const report = lintPlaylist("Artist/song.mp3", "Artist/Song.mp3\nArtist/SONG.mp3", false);
  assert.equal(report.findings[0].code, "ambiguous_path");
  assert.equal(report.corrected, "Artist/song.mp3");
});

test("empty input has a useful next step", () => {
  const report = lintPlaylist("#EXTM3U", "", true);
  assert.equal(report.findings[0].code, "empty_playlist");
});
