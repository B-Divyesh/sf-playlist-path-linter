# Handoff — Playlist Path Linter v0.1.0

## What shipped

- A publishable Rust single-binary CLI with a small public library API.
- M3U/M3U8 decoding for UTF-8 (with or without BOM), UTF-16 LE/BE BOM, and Windows-1252 M3U; corrected output retains the source encoding and line endings.
- Exact, NFC/NFD, case-sensitive, and case-insensitive path resolution against a recursively indexed library root.
- Precise findings for missing, ambiguous, outside-root, remote, Unicode-normalization, case, empty-playlist, and invalid date/year conditions.
- Date inspection for common tagged audio containers through Lofty, including year, recording date, and original release date. Valid ranges are `1000..=current year + 1`; calendar dates are checked.
- Read-only defaults, source-overwrite protection, stable exit codes, human output, versioned `--json`, and optional `--fix` copy.
- A responsive Vite documentation site with a fully local browser path-checking worksheet, folder/playlist pickers, corrected download, offline shell, privacy and terms pages, and explicit empty/loading/error/offline states.
- A product-specific blueprint drafting-sheet system, an 82 KB generated WebP hero, and a silent user-controlled recording of the actual worksheet.

## Run and verify

```sh
npm ci
npm test
npm run build
```

The deploy artifact is `dist/site/` and contains `index.html`. The release CLI is `target/release/playlist-path-linter`.

Additional checks run on 2026-08-27:

- `npm test`: 7 Rust unit tests, 5 Rust integration tests, 3 browser-core tests — all pass.
- `npm run build`: release binary and Vite site complete successfully.
- `cargo package --manifest-path crates/playlist-path-linter/Cargo.toml --allow-dirty`: package builds and verifies; 52.9 KiB unpacked / 15.3 KiB compressed at the time of verification.
- `/opt/fleet/lib/verify-url.sh http://127.0.0.1:4173 .factory/evidence/final-verify`: title, `lang`, single `h1`, main landmark, image alt, button labels, and browser console pass; no console errors.
- Playwright + axe WCAG A/AA audit at 390 × 844: zero violations, no horizontal overflow, local sample produces the expected three findings, corrected-download control enables.
- Lighthouse mobile production build: Performance 100, Accessibility 100, Best Practices 100, SEO 100; FCP 0.9 s, LCP 1.6 s, TBT 20 ms, CLS 0.
- Initial assets: 5.76 KB JS, 10.28 KB CSS, no webfonts, 82 KB WebP hero. The below-fold demo video uses `preload="none"`.
- Manual CLI smoke test confirmed exit code 1 for findings, Unicode correction in the copied M3U8, unchanged missing lines, and valid versioned JSON.

Ready-to-publish command (factory supplies credentials):

```sh
cargo publish --manifest-path crates/playlist-path-linter/Cargo.toml --dry-run
```

## Known gaps and next steps

- The browser worksheet intentionally checks paths only; browser sandboxing prevents it from parsing every native media tag format. The CLI is the authoritative date-tag checker and this distinction is stated in the UI.
- The CLI reads date tags but never repairs them. This is deliberate for v1’s read-only safety contract.
- No Windows/macOS CI matrix is committed yet. Both destination case behaviors are deterministic CLI modes and are covered in Linux fixture tests; release automation should add native runners before publishing platform binaries.
- Pilot-user validation from the opportunity brief remains a post-deployment activity.

No deployment, DNS, registry publishing, billing, secrets, analytics, or external music-service calls were performed.
