# Verification 2 — PASS

**Candidate:** `64e82bf17613b82ef435c91d5f8da77a3672371c` (`main`)
**Verified:** 2026-08-28 UTC
**Live URL:** <https://playlist-path-linter.sociobot.in/>
**Verdict:** **PASS**

This is an independent clean-checkout verification against the researched brief. The release is a usable local-first CLI for M3U/M3U8 path handoff, with a complementary browser path worksheet. No release-blocking or non-blocking product defects were found.

## Build, tests, static checks, and package

The worktree was clean and already checked out at the candidate before installation. The following all passed:

```sh
npm ci
npm test
npm run build
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
npx tsc --noEmit --target ES2022 --module ESNext --moduleResolution bundler \
  --lib ES2022,DOM,DOM.Iterable --allowJs site/src/main.ts
cargo package --manifest-path crates/playlist-path-linter/Cargo.toml --allow-dirty
```

- `npm test`: 7 Rust unit tests, 6 Rust CLI integration tests, and 3 browser-core tests passed.
- `npm run build`: generated `target/release/playlist-path-linter` and `dist/site/`.
- No repository lint/typecheck script is defined; the explicit Rust format/Clippy and TypeScript checks above pass.
- The ready-to-publish crate was produced at `target/package/playlist-path-linter-0.1.0.crate` (16,109 bytes).
- I unpacked that crate into a new temporary consumer, installed it using `cargo install --path <unpacked-crate> --root <empty-root> --force`, and ran both top-level and `lint` help. The installed public binary worked.

## Independent CLI end-to-end matrix

Using the release binary against a fresh temporary library and CRLF M3U8:

| Case | Observed result |
| --- | --- |
| Exact existing track | exit 0, JSON `clean: true`, one resolved entry |
| NFC/NFD mismatch | exit 1, line-addressed `unicode_normalization`; corrected copy uses the library spelling |
| Case mismatch | exit 1, `case_mismatch` under the selected destination model |
| Two case-equivalent files | exit 1, `ambiguous_path`; no guessed correction |
| Missing path | exit 1, `missing_path` |
| `../` escape | exit 1, `outside_root` |
| Remote URL | exit 1 with non-fetching `remote_entry` notice |
| Mixed defect playlist | six entries: 2 resolved, 2 corrected, 2 missing, 1 ambiguous, 1 skipped; expected codes were all present |
| `--fix` source target | exit 2: source overwrite refused |
| Invalid M3U8 bytes | exit 2 with decoding error |
| Malformed WAV `INFO/ICRD=2024-01-01Tbogus` | packed-consumer binary exited 1; `invalid_date_tag`, `date_issues: 1` |

The corrected mixed playlist retained all seven CRLF line endings. `--json` provided schema version `1` and line-specific diagnostics. The command is non-interactive and `--help` documents the subcommand, options, and exits as expected.

## Browser, accessibility, visual, and performance QA

The production build was exercised in Chromium at 1440x960 and 390x844. Visual inspection confirms the deployed blueprint-drafting design remains legible and correctly stacks at 390px.

- Exactly one `h1`, one `main`, a valid title and `lang="en"`; no console or page errors.
- No horizontal overflow at either viewport.
- Keyboard-only navigation reached the skip link, both text/file controls, case selector, sample button, and Run inspection button. Space loaded the sample; Enter ran it; the report announced `3 findings across 3 entries.` and received focus.
- Focus ring is visible on keyboard focus: 3px orange (`rgb(166, 63, 18)`) outline.
- Axe WCAG A/AA audits at both viewports found zero serious/critical violations. `npm run audit:browser -- http://127.0.0.1:4173/` also passed: 3 expected findings, enabled corrected download, no console errors, no overflow, zero Axe violations.
- With reduced motion enabled, the finding animation duration is `0.01ms`.
- Lighthouse mobile production-preview report: Performance **99**, Accessibility **100**, Best Practices **100**, SEO **100**; FCP 1.7s, LCP 1.7s, TBT 30ms, CLS 0. (Chrome emitted a post-audit tab-crash warning after writing the complete JSON report; the scored report and measurements were present.)
- Initial JS is 5,755 B, CSS 10,279 B, hero WebP 83,652 B, and no webfonts ship: all are within the stated budgets.

## Privacy, policies, PWA, and deployment identity

- Source review and a Chromium request capture found no telemetry, analytics, third-party scripts/fonts, uploads, or runtime outbound requests. Explicit GitHub links are user-initiated navigation only.
- On the live origin, `localStorage`, `sessionStorage`, and IndexedDB each contained zero entries after page use. The worksheet operates locally.
- HTTPS live headers include HSTS, `Referrer-Policy: no-referrer`, `X-Content-Type-Options: nosniff`, restrictive camera/microphone/geolocation Permissions Policy, and CSP `default-src 'self'` with self-only script/style/image sources. Hashed JS is `public, max-age=31536000, immutable`; the WebP is cached for one week.
- The live root, JS, CSS, service worker, manifest, hero image, privacy page, and terms page SHA-256 byte-match `dist/site/` from this candidate. This rules out the earlier deployment-only concern for this commit.
- The live service worker controls the page with cache `ppl-shell-v1`. Offline reload retained the title and H1 and displayed the offline notice without errors. `registration.update()` completed with the current worker active and no pending waiting/installing worker; the worker's versioned-cache activation removes old cache names.

## Defects by severity

None found.

## Verification limitations

The browser worksheet deliberately validates path text only; native audio tag inspection is correctly performed by the packaged CLI. No alternate deployed service-worker version was available, so update behavior was checked through `registration.update()` and the current worker/cache lifecycle rather than a real cross-version rollout.
