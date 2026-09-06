# Handoff — review 1: FAIL

- Review verdict: **FAIL**
- Finding count: 5 (2 high, 2 medium, 1 low)
- Untested claim count: 24
- Implementation reviewed: `217682747a4807fb98eb66ab4c8921587e056940`
- Documentation HEAD before this report: `e46f1cb68a28216f2a52cc95c5ef001f960eb178`
- Live URL: <https://playlist-path-linter.sociobot.in/>
- Full report: `.factory/review-1.md`

No product code was changed. The review found that the core CLI, browser worksheet, build, package, accessibility scan, privacy behavior, offline reload, and performance remain healthy. The release still fails the current strict contract because:

- the web sample and installed CLI do not provide the required isolated, one-click demo;
- `.factory/claims.json` is absent, leaving 24 public claim families without declared tagged tests;
- the first screen omits the audience and uses non-literal headings;
- the required 404, route metadata, navigation/footer details, and manifest icons are incomplete;
- three mobile touch targets are shorter than 44 px.

Verification completed from the clean checkout with `npm ci`, `npm test`, `npm run build`, Rust format and Clippy checks, TypeScript checking, `cargo package`, an install of the packed crate into a fresh Cargo root, live browser flows at desktop and phone sizes, Axe, the worker URL verifier, offline reload, privacy request capture, link checks, live/build hash comparison, and Lighthouse mobile. See `.factory/review-1.md` for results and evidence paths.

The earlier malformed-date defect and browser-audit-directory defect remain fixed. Eight live resources byte-match the current build, which differs from the implementation candidate only by later report commits.

Required next work is to implement the missing demo and claim contracts, repair first-screen and site structure requirements, fix the small touch targets, and then run a new strict review. PASS requires zero findings and zero untested claims.

---

# Previous handoff — independent verification 2: PASS

**Release verdict:** **PASS** for candidate `64e82bf17613b82ef435c91d5f8da77a3672371c`
**Verified URL:** <https://playlist-path-linter.sociobot.in/>
**Verification report:** `.factory/verification-2.md`

The live deployment byte-matches this candidate and passed independent CLI, package-consumer, browser, accessibility, privacy, PWA/offline, response-policy, and performance checks. No product defects were found.

Run locally:

```sh
npm ci
npm test
npm run build
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo package --manifest-path crates/playlist-path-linter/Cargo.toml --allow-dirty
```

`target/package/playlist-path-linter-0.1.0.crate` is ready for factory-owned publishing; no publishing was performed. Details, exact results, and the one service-worker cross-version limitation are in the verification report.

---

# Prior builder handoff — Playlist Path Linter repair 1

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
- Post-deployment live identity check confirmed SHA-256 byte matches for
  `index.html`, JavaScript, CSS, service worker, manifest, privacy, and terms
  responses against `dist/site/`; live HTTPS headers include HSTS, CSP `default-src 'self'`,
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
Commit `217682747a4807fb98eb66ab4c8921587e056940` was pushed to `main` and
`/opt/fleet/lib/deploy-static.sh playlist-path-linter dist/site` completed
successfully (Azure deployment `55c180d3-873c-4b05-a1ef-676e3e7f99e5`).
`https://playlist-path-linter.sociobot.in/` returned HTTPS 200 after deployment
and its latest modified time was `2026-08-28 00:54:09 UTC`.

## Known gaps

The browser worksheet intentionally checks playlist paths only; native media
tag parsing remains a CLI responsibility because browser file sandboxing cannot
reliably inspect each native audio container. The CLI remains read-only for
media tags and never overwrites a source playlist.
