# Playlist Path Linter

Check playlist paths before import. It is for musicians and local-library listeners moving M3U playlists between taggers, servers, and devices.

The CLI reads M3U and M3U8 playlists. It resolves relative and absolute entries under a library root. It handles mixed slash separators. It diagnoses NFC/NFD and case-only path differences. It reports missing and ambiguous paths without guessing. It rejects parent traversal outside the root. It reports remote URLs without fetching them.

Resolved audio files are checked for year zero, malformed dates, and implausible future dates. Comments and EXTINF records stay in corrected playlists. Safe corrections preserve the source playlist, UTF-8 BOM, and line endings. JSON reports are versioned and include line-level findings.

Everything runs locally. The CLI does not contact music or metadata hosts. The browser worksheet makes no uploads or third-party requests. The normal worksheet clears after refresh. The folder picker reads selected file names without reading audio contents.

Live site: <https://playlist-path-linter.sociobot.in/>. One-click browser demo: <https://playlist-path-linter.sociobot.in/demo/>.

## Try the sample

Run the bundled sample without setting up a music library:

```sh
playlist-path-linter demo
```

The command creates a temporary workspace. It reports Unicode, case, and missing-path problems. It prints where the source and corrected copy were kept. The command exits `1` because the sample has findings. The input also ships at `examples/demo-playlist.m3u8`.

The browser demo opens the same three known problems in one click. It uses `demo:playlist-path-linter:*` browser storage only. Reset demo restores the sample. Start for real discards the demo storage.

## Install

Use Rust 1.85 or later:

```sh
cargo install --path crates/playlist-path-linter
```

## Use the CLI

Check a playlist against a library root:

```sh
playlist-path-linter lint road-trip.m3u8 --root /music
```

Write a corrected copy without changing the source:

```sh
playlist-path-linter lint road-trip.m3u8 --root /music --fix road-trip.fixed.m3u8
```

Write a JSON report for scripts:

```sh
playlist-path-linter lint road-trip.m3u8 --root /music --json > report.json
```

Use `--case sensitive`, `--case insensitive`, or `--case auto`. `auto` follows the current platform. The report states the selected case model.

| Exit code | Result |
| --- | --- |
| `0` | Every playlist entry resolved cleanly. |
| `1` | The report contains one or more findings. |
| `2` | The command could not run because of usage, encoding, filesystem, or write errors. |

The tool is read-only by default. It does not change a source playlist or media file. The browser worksheet checks path text only. The CLI checks date tags in resolved audio files.

## Develop, test, and build

Requirements: Rust 1.85 or later, Node 20 or later, and npm 10 or later.

```sh
npm ci
npm test
npm run build
```

The documented setup builds the release binary and the static site. The site output is `dist/site/`. The release binary is `target/release/playlist-path-linter`.

Run the site locally:

```sh
npm run dev
```

Create the factory-ready crate without publishing it:

```sh
cargo package --manifest-path crates/playlist-path-linter/Cargo.toml --allow-dirty
```

The factory owns registry credentials. This repository does not publish or deploy itself.

## Privacy and license

No account is needed. The tool is free and MIT licensed. There are no cookies, ads, analytics, or third-party scripts. Read the [privacy policy](https://playlist-path-linter.sociobot.in/privacy/) and [terms](https://playlist-path-linter.sociobot.in/terms/).

## Scope

Playlist Path Linter does not download audio. It does not query metadata services. It does not edit tags.

## License

MIT. See [LICENSE](LICENSE).
