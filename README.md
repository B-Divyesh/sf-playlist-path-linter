# Playlist Path Linter

Playlist Path Linter checks an M3U/M3U8 before a music server or tagger silently drops tracks. It resolves relative and absolute entries against a local library, detects Unicode normalization and case mismatches, reports ambiguous or missing files, inspects resolved audio tags for suspicious dates, and can write a corrected playlist plus a JSON report.

It is for musicians, DJs, archivists, and serious local-library listeners moving playlists between taggers, servers, NASes, and devices. Everything runs locally: no telemetry, metadata lookup, or network access.

Live docs and browser specimen: <https://playlist-path-linter.sociobot.in>

## Install

Download a release binary, or build with Rust 1.85+:

```sh
cargo install --path crates/playlist-path-linter
```

## Usage

Lint a playlist against a library root:

```sh
playlist-path-linter lint road-trip.m3u8 --root /music
```

Write a corrected copy without changing the source. The output retains the input BOM/encoding and line endings:

```sh
playlist-path-linter lint road-trip.m3u8 --root /music --fix road-trip.fixed.m3u8
```

Produce a machine-readable report:

```sh
playlist-path-linter lint road-trip.m3u8 --root /music --json > report.json
```

Use `--case sensitive` or `--case insensitive` to model the destination filesystem. `auto` is the default and follows the current platform. Run `playlist-path-linter --help` or `playlist-path-linter lint --help` for all options.

Exit codes:

| Code | Meaning |
| --- | --- |
| `0` | Playlist is clean |
| `1` | One or more findings were reported |
| `2` | Usage, encoding, filesystem, or write error |

The JSON schema is versioned by `schema_version`. Each finding includes the playlist line, stable code, severity, original path, candidates when relevant, and a precise explanation.

## What it checks

- Missing paths, including mixed `/` and `\\` separators.
- NFC/NFD Unicode mismatches and case-only mismatches.
- Ambiguous normalized paths (never guessed or auto-corrected).
- Relative traversal outside the selected library root.
- Remote URLs, reported as skipped rather than fetched.
- Resolved media tags containing `0000`, malformed dates, or years outside `1000..=current year + 1`.

Comments and `#EXTINF` records are retained. By default the CLI is read-only. A fixed copy changes only unambiguous path lines.

## Develop, test, and deploy

Requirements: Rust 1.85+, Node 20+, and npm 10+.

```sh
npm install
npm test
npm run build
```

`npm test` runs Rust unit/integration tests and browser-demo tests. `npm run build` creates the CLI release binary and the deployable static site at `dist/site/` (with `index.html` at that root). Run the site locally with `npm run dev`.

Ready-to-publish check:

```sh
cargo package --manifest-path crates/playlist-path-linter/Cargo.toml --allow-dirty
```

The factory owns registry credentials; this repository does not publish or deploy itself.

## Scope and privacy

Playlist Path Linter does not download audio, query metadata services, modify tags, run a media server, or alter a source playlist. The website’s specimen runs entirely in the browser and stores nothing.

## License

MIT. See [LICENSE](LICENSE).
