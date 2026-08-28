# Handoff — Playlist Path Linter repair 1

## Release-blocking repair

Fixed the verifier's high-severity false negative from candidate
`f2c9353f2eea7f8833c016cad5f356b87e10bb6f`: the CLI previously accepted a
date with any text after a valid `YYYY-MM-DD` when that text began with `T` or
a space. The validator now accepts exactly the documented date-only forms:
`YYYY`, `YYYY-MM`, and `YYYY-MM-DD`.

The exact regression is covered at two levels:

- Unit coverage rejects both `2024-01-01Tbogus` and
  `2024-01-01 trailing text`.
- An integration test writes a valid RIFF/WAV `INFO/ICRD=2024-01-01Tbogus`,
  invokes the built `playlist-path-linter` CLI with `--json`, and asserts exit
  code `1`, `clean: false`, `summary.date_issues: 1`, and an
  `invalid_date_tag` finding.

Also repaired the verifier's minor tooling observation: `npm run audit:browser`
now creates its ignored `.factory/evidence/` directory, so it passes from a
pristine checkout. Playwright is pinned to `1.58.2`, matching the provided
browser installation.

## Verification evidence — 2026-08-28 UTC

All commands were run from a clean `npm ci` install after the repair:

```sh
npm ci
npm test
npm run build
npx tsc --noEmit --target ES2022 --module ESNext --moduleResolution bundler \
  --lib ES2022,DOM,DOM.Iterable --allowJs site/src/main.ts
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo package --manifest-path crates/playlist-path-linter/Cargo.toml --allow-dirty
```

- `npm test`: passed — 7 Rust unit tests, 6 Rust integration tests (including
  the CLI/WAV regression), and 3 browser-core tests.
- `npm run build`: passed — optimized Rust binary plus `dist/site/`.
- TypeScript check, Rust formatting, and Clippy with warnings denied: passed.
- `cargo package`: passed — package verified from `target/package/`.
- Clean-consumer smoke test passed:
  `cargo install --path crates/playlist-path-linter --root <temporary-root> --force`,
  followed by `playlist-path-linter --help`.
- `npm run audit:browser -- http://127.0.0.1:4173/`: passed at 390 × 844;
  three expected worksheet findings, corrected download enabled, no console
  errors, no horizontal overflow, and zero Axe violations.
- Production-preview Playwright checks passed at 1440 × 960 and 390 × 844:
  exactly one H1 and one main landmark, no horizontal overflow, no console or
  page errors, and no serious/critical WCAG A/AA violations. Keyboard Tab
  reached the skip link; Enter loaded the faulty sample and ran the inspection,
  rendering `3 findings across 3 entries.`
- `verify-url.sh` against the local production preview passed title, `lang`,
  one H1, main landmark, image alt text, button labels, and browser-console
  checks.
- Lighthouse mobile production preview: Performance 100, Accessibility 100,
  Best Practices 100, SEO 100; FCP 0.9 s, LCP 1.6 s, TBT 0 ms, CLS 0.
- Initial assets remain within budget: JavaScript 5,755 B, CSS 10,279 B, and
  project-owned hero WebP 83,652 B; no webfonts or third-party runtime assets.
- Live pre-deployment identity check confirmed `index.html` SHA-256 matches
  `dist/site/`; live HTTPS headers include HSTS, CSP `default-src 'self'`,
  `Referrer-Policy: no-referrer`, `X-Content-Type-Options: nosniff`, and the
  restrictive camera/microphone/geolocation Permissions Policy. Source and
  browser checks found no telemetry, storage, uploads, or runtime outbound
  requests except explicit GitHub links.
- Live PWA check passed: after registration and an online reload, Chromium was
  controlled by `ppl-shell-v1`; switching offline and reloading retained the
  title, one H1, and the offline notice without errors. The service worker's
  versioned cache activation removes older cache names.

## Publish and deploy

The deployable artifact remains the static site at `dist/site/` and the CLI
remains the Rust single binary. The ready-to-publish check is:

```sh
cargo package --manifest-path crates/playlist-path-linter/Cargo.toml --allow-dirty
```

Registry publishing was not performed; factory credentials own that action.
The repair is deployed with the factory static deployment configuration after
the repair commit is pushed.

## Known gaps

The browser worksheet intentionally checks playlist paths only; native media
tag parsing remains a CLI responsibility because browser file sandboxing cannot
reliably inspect each native audio container. The CLI remains read-only for
media tags and never overwrites a source playlist.
