# Verification 1 — FAIL

**Candidate:** `f2c9353f2eea7f8833c016cad5f356b87e10bb6f` (`main`)  
**Verified:** 2026-08-28 UTC  
**Live URL:** https://playlist-path-linter.sociobot.in/  
**Verdict:** **FAIL**

The deployed static site byte-matches the candidate, and the CLI, browser worksheet, package, privacy controls, accessibility, and performance checks were otherwise healthy. The core CLI nevertheless has a release-blocking false negative for malformed audio date tags, one of the brief's smallest-useful-product requirements.

## Release-blocking defect

### High — malformed date/time tag is silently accepted

`valid_date` accepts any suffix after an otherwise valid `YYYY-MM-DD` when its first character is `T` or a space. This contradicts both the documented accepted values (`YYYY`, `YYYY-MM`, `YYYY-MM-DD`) and the brief's requirement to report invalid date tags.

Fresh reproduction with the release binary:

```sh
# dated.wav has RIFF INFO/ICRD = 2024-01-01Tbogus
playlist-path-linter lint dated.m3u8 --root library --case sensitive --json
```

Observed: exit `0`, `clean: true`, `date_issues: 0`, no findings.  
Control: a similarly constructed `2023-02-29` tag correctly returned exit `1` with `invalid_date_tag`.

Impact: a malformed date tag can reach an importer without diagnosis, defeating the date-corruption protection advertised for the exact Beets-style failure in the brief. This is sufficient to reject the candidate.

## Checks that passed

### Clean checkout, build, quality, and package

- `npm ci`: completed; 0 npm vulnerabilities reported.
- `npm test`: passed — 7 Rust unit tests, 5 Rust integration tests, 3 browser-core tests.
- `npm run build`: passed; release binary and `dist/site/` produced.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo fmt --all -- --check`: passed.
- TypeScript check with Vite-compatible DOM libraries: `npx tsc --noEmit --target ES2022 --module ESNext --moduleResolution bundler --lib ES2022,DOM,DOM.Iterable --allowJs site/src/main.ts`: passed. (No typecheck/lint script is defined in `package.json`.)
- `cargo package --manifest-path crates/playlist-path-linter/Cargo.toml --allow-dirty`: passed; produced `target/package/playlist-path-linter-0.1.0.crate` (15,680 bytes).
- Clean-consumer install: `cargo install --path crates/playlist-path-linter --root /tmp/ppl-consumer... --force` passed; installed `playlist-path-linter 0.1.0` and public CLI ran successfully.

### CLI end-to-end

Using the release binary against a temporary library and CRLF M3U8, one run correctly produced exit `1`, schema version `1`, and line-addressed `unicode_normalization`, `ambiguous_path`, `missing_path`, `remote_entry`, and `outside_root` findings. It wrote only the unambiguous NFC correction; the input remained unchanged and the fixed copy retained all six CRLF endings.

- Case-insensitive mode returned an ambiguous-path error for `Song.mp3`/`SONG.mp3`; no correction was guessed.
- Invalid UTF-8 in an M3U8 returned exit `2` with a useful error.
- `--fix` targeting the source returned exit `2`; source overwrite was refused.
- `--help`, subcommand help, JSON output, and documented exit codes were exercised.
- The malformed-date false negative above is the exception.

### Deployment, privacy, response policy, and PWA

- Live root, JS, CSS, service worker, WebP, manifest, privacy, and terms responses all SHA-256 matched `dist/site/` from the candidate. Deployment is current, not a deployment-only failure.
- Live root returned HTTPS, HSTS, CSP (`default-src 'self'`), `Referrer-Policy: no-referrer`, `X-Content-Type-Options: nosniff`, and a restrictive camera/microphone/geolocation Permissions Policy. Hashed JS was `Cache-Control: public, max-age=31536000, immutable`.
- Source review and Chromium resource capture found no telemetry, storage APIs, uploads, third-party scripts, or outbound runtime requests. The only external links are explicit GitHub source/license links.
- On the live HTTPS origin, service worker control was active with cache `ppl-shell-v1`; after switching the browser offline, reload rendered the document title, H1, and offline notice without errors. Its cache name is versioned and activation removes old named caches. No pending service-worker update was available to exercise.

### Browser product QA

- Local production preview at desktop 1440×960 and mobile 390×844: no console/page errors, one H1, 16px body text, and no horizontal overflow.
- `npm run audit:browser -- http://127.0.0.1:4173/`: passed after creating its ignored evidence directory; 3 expected sample findings, corrected-download enabled, no console errors, zero axe violations.
- Independent Axe WCAG A/AA smoke audits at desktop and 390px had zero serious/critical violations.
- Keyboard-only flow reached the skip link, loaded the faulty sample, submitted the inspection with Enter, and rendered `3 findings across 3 entries.` Reduced-motion mode reduced result animation duration to `0.01ms`.
- Lighthouse mobile production preview: Performance 99, Accessibility 100, Best Practices 100, SEO 100; FCP 1.0s, LCP 1.6s, TBT 80ms, CLS 0.
- Initial JS is 5,755 bytes and CSS is 10,279 bytes; no shipped webfonts. The hero WebP is 83,652 bytes. All are within the stated budgets.

## Minor verification tooling observation

`npm run audit:browser` exits with `ENOENT` from a pristine checkout because it writes `.factory/evidence/browser-audit.json` without first creating `.factory/evidence/`. Creating that ignored directory allows the audit to pass. This did not affect the product runtime and is not the release-blocking defect.

## Required next step

Make date parsing exact: reject trailing content unless fully supporting and validating a documented timestamp format, add regression tests for malformed `T`/space suffixes, then rerun this verification suite. Do not release this candidate before that fix.
