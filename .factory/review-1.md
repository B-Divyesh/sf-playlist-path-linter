# Review 1 — Check playlist paths before import

**Verdict: FAIL**

- Findings: **5** (2 high, 2 medium, 1 low)
- Untested public claim families: **24**
- Live URL: <https://playlist-path-linter.sociobot.in/>
- Review window: 2026-09-05 to 2026-09-06 UTC
- Implementation reviewed: `217682747a4807fb98eb66ab4c8921587e056940`
- Documentation HEAD: `e46f1cb68a28216f2a52cc95c5ef001f960eb178`

The live HTML, JavaScript, CSS, service worker, manifest, image, privacy page, and terms page byte-match the local build. Commits after `2176827` change only `.factory` reports, so `2176827` is the implementation candidate and `e46f1cb` is the documentation commit.

## What a visitor sees before scrolling

- Job stated: catch path, Unicode, and date problems before a music server drops tracks.
- Audience stated: none. A visitor can infer that the page is for people importing M3U playlists, but the first screen does not name them.
- First action: **Inspect a playlist**. It scrolls to an empty worksheet. The sample action is below the first viewport on both 1440 × 960 desktop and 390 × 844 phone.

Screenshots: `/work/.evidence/review-1/desktop-first-screen.png` and `/work/.evidence/review-1/phone-first-screen.png`.

## Findings

### 1. High — The required demo sandbox does not exist

The visible sample is not the required one-click, isolated demo.

- `/demo` returns the ordinary landing page with HTTP 200. It does not enter a sample state or set the route title to `Demo — Playlist Path Linter`.
- The first-screen action is **Inspect a playlist**, not **Try it with sample data**. The sample button is below the fold.
- One sample click only fills the two text areas. It produces zero findings and says `Sample loaded. Run the inspection.` A second click is required to see output.
- After the second click, the sample is realistic and produces three precise findings: Unicode normalization, case mismatch, and a missing path.
- There is no persistent `Demo — sample data, nothing is saved` label, **Reset demo**, or **Start for real** control.
- The sample shares the ordinary worksheet state. Loading it after entering `My/Real/Playlist.flac` and `My/Real/Library.flac` replaced both values. There is no separate demo namespace or recovery control.
- `.factory/demo.md` and `examples/` are absent. The packed crate contains no sample data.
- The installed CLI rejects both `--demo` and `demo` with exit 2. It does not create a temporary sample workspace or say where output went.
- The landing recording shows the browser worksheet, not the real CLI running its main job.

This fails the demo-sandbox contract for both the site and the CLI. The empty browser worksheet itself does not save data: storage stayed empty and reload cleared both fields. That does not replace demo isolation.

### 2. High — Public claims have no required claim registry or tagged tests

`.factory/claims.json` is absent. No test contains an `@claim:<id>` tag, so there are no declared claim commands to run. Ordinary unit, integration, and browser tests pass, but they do not meet the required one-public-claim-to-one-sandbox-test contract.

The following 24 distinct, non-duplicate public claim families appear on the site, legal pages, CLI help, or README. Each lacks its required claim entry and tagged command, so the untested claim count is 24.

| # | Public claim family | Direct review evidence | Required claim test |
|---:|---|---|---|
| 1 | Reads M3U and M3U8 playlists | Passed in CLI and browser checks | Missing |
| 2 | Resolves relative and absolute entries against a library root | Relative paths passed; source inspected for absolute paths | Missing |
| 3 | Handles mixed slash separators | Source inspected | Missing |
| 4 | Detects NFC/NFD differences | Passed in packaged CLI and browser | Missing |
| 5 | Detects case-only differences | Passed in tests and browser sample | Missing |
| 6 | Reports missing and ambiguous paths | Passed in packaged CLI | Missing |
| 7 | Rejects traversal outside the library root | Passed in packaged CLI and browser | Missing |
| 8 | Skips remote URLs without fetching them | Passed in packaged CLI and browser request capture | Missing |
| 9 | Finds year zero, malformed dates, and implausible future dates | Clean test suite passed, including malformed timestamp regression | Missing |
| 10 | Retains comments and EXTINF records | Source inspected and ordinary tests cover related output | Missing |
| 11 | Writes only unambiguous corrections | Passed in packaged CLI and browser download | Missing |
| 12 | Preserves input encoding, BOM, and line endings | Ordinary unit and prior verification evidence only | Missing |
| 13 | Emits versioned JSON with line details and stable fields | Passed in packaged CLI | Missing |
| 14 | Uses documented exit codes 0, 1, and 2 | Passed in packaged CLI | Missing |
| 15 | Models sensitive, insensitive, and automatic case behavior | Sensitive and insensitive paths passed | Missing |
| 16 | Does not change the source playlist, media, or tags | Source-overwrite refusal and unchanged browser source passed | Missing |
| 17 | Makes no CLI network, telemetry, or metadata requests | Source and runtime request inspection passed | Missing |
| 18 | Runs the worksheet locally without uploads | Browser request capture passed | Missing |
| 19 | Stores nothing and clears worksheet data on refresh | Storage and reload checks passed | Missing |
| 20 | Reads selected folder names but not audio contents | Source inspection only | Missing |
| 21 | Keeps the worksheet working after an offline reload | Passed in a fresh browser context | Missing |
| 22 | Loads the documented sample and reports three known problems | Passed only after two clicks | Missing |
| 23 | Clean setup commands test, build, and produce package artifacts | Passed in this review | Missing |
| 24 | Is free/MIT and uses no accounts, cookies, ads, or third-party scripts | License, source, storage, and request checks passed | Missing |

Some behavior is directly proven above, but none is continuously declared and tested in the required claim sandbox. A strict PASS requires all 24 to be registered or the public wording removed.

### 3. Medium — First-screen and section copy do not meet the plain-words contract

- The H1 is `Make every track findable.` It does not name the concrete job of checking playlist paths before import.
- The first-screen sentence explains the risk but does not name the audience.
- The primary action does not say what happens after activation.
- Section headings and labels use non-literal copy: `The boundary check`, `Looks equal isn’t equal.`, `See the fault lines.`, and `Handoff with proof`.
- `.factory/copy-audit.md` is absent, so the required sentence-length, banned-word, and terminology check was not completed.

The browser title is good. `Playlist Path Linter — inspect M3U paths before import` is route-appropriate for the landing page and is under 60 characters.

### 4. Medium — Required routes, metadata, and standard navigation are incomplete

- `/definitely-not-a-real-route` and `/404.html` both return HTTP 200 and render the landing page. This is not a deliberate 404 response; the required designed 404 route is absent.
- The landing page has no canonical URL, Open Graph tags, Twitter card tags, 1200 × 630 social image, or apple-touch icon.
- The privacy and terms pages have route titles but no description, canonical, Open Graph, Twitter, theme-color, or manifest metadata.
- The web manifest has an empty `icons` array.
- The landing header has no Demo or Privacy link. At 390 px it hides Worksheet and Install, leaving only the external Source link.
- The footer omits `Built by Param Factory` and a version/build identifier.
- External Source and MIT license links do not tell the visitor that they leave the site.
- `sitemap.xml` lists the existing home, privacy, and terms pages, but cannot list the missing demo route.

All links that do exist returned HTTP 200. Privacy and terms content is present and readable.

### 5. Low — Three mobile touch targets are shorter than 44 CSS pixels

At 390 px wide, the home wordmark is 37.19 px high, `choose a playlist` is 16 px high, and `choose a library folder` is 37.69 px high. The hidden file inputs themselves are 1 × 1 px. The visible labels receive a focus outline through `:has()`, but their clickable height still misses the 44 px baseline.

Other controls meet the target size. Keyboard order, activation, and focus visibility passed.

## Product paths checked

### Browser worksheet

Fresh desktop and phone contexts produced the same results:

| Path | Result |
|---|---|
| Empty page | Clear empty state and next step |
| Exact path | `Clean: 1 of 1 entries resolved.` |
| Empty M3U | `empty_playlist` finding |
| Parent traversal | `outside_root` finding |
| Remote URL | `remote_entry` notice; no external request |
| Missing path | `missing_path` finding |
| Add the missing path and rerun | Clean recovery |
| Unicode mismatch | `unicode_normalization`; corrected download uses NFC spelling |
| Download | `playlist.fixed.m3u8`; pasted source remained unchanged |
| Reload | Worksheet returned to its empty state |

The sample output is shown in `/work/.evidence/review-1/desktop-populated.png` and `/work/.evidence/review-1/phone-populated.png`. Machine-readable browser evidence is in `/work/.evidence/review-1/live-browser-audit.json` and `live-path-matrix.json`.

### Installed CLI

The crate produced by `cargo package` was installed into a new temporary Cargo root. The installed executable passed:

- top-level and `lint` help;
- an exact path with exit 0 and clean JSON;
- a mixed fixture with Unicode, ambiguous, missing, outside-root, and remote entries with exit 1;
- corrected-copy output without changing the source;
- an invalid library root with exit 2;
- refusal to overwrite its source with exit 2.

The packed artifact contains 11 files and no bundled example directory. Both CLI demo forms fail as described in finding 1.

## Accessibility, privacy, offline, and performance

- Worker `verify-url.sh`: passed HTTPS, title, `lang`, one H1, main landmark, image alt, labeled buttons, and console checks.
- Axe WCAG A/AA at 1440 × 960 and 390 × 844: zero violations.
- Keyboard: skip link is first, controls work with Space and Enter, results receive focus, and the orange 3 px focus ring is visible.
- Reduced motion: result animation and transitions reduce to `0.01ms`.
- No horizontal overflow at normal desktop or phone size.
- Privacy flow: no external runtime requests; localStorage and sessionStorage remained empty; IndexedDB was empty in prior verification; reload cleared inputs.
- Offline: after an online visit, a fresh context was service-worker controlled by `ppl-shell-v1`; offline reload retained the title, H1, and offline notice. No waiting update existed. The worker deletes older named caches on activation.
- Security headers: HSTS, self-only CSP, `Referrer-Policy: no-referrer`, `X-Content-Type-Options: nosniff`, and restrictive camera/microphone/geolocation policy are live.
- Lighthouse mobile: Performance 100, Accessibility 100, Best Practices 100, SEO 100; FCP 1.1 s, LCP 1.4 s, TBT 0 ms, CLS 0.
- Initial JS is 5,755 bytes, CSS 10,279 bytes, and the hero WebP 83,652 bytes. No webfont ships.

The deterministic linter does not need an AI feature. No missed AI-assisted step was found.

## Clean-checkout commands

The worktree was clean at the start. Documented prerequisites were available: Node 22.23.2, npm 10.9.8, and Rust 1.98.0.

| Command | Result |
|---|---|
| `npm ci` | Pass; 0 vulnerabilities |
| `npm test` | Pass; 7 Rust unit, 6 CLI integration, 3 browser-core tests |
| `npm run build` | Pass; release binary and `dist/site/` produced |
| `cargo fmt --all -- --check` | Pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `npx tsc --noEmit --target ES2022 --module ESNext --moduleResolution bundler --lib ES2022,DOM,DOM.Iterable --allowJs site/src/main.ts` | Pass |
| `cargo package --manifest-path crates/playlist-path-linter/Cargo.toml --allow-dirty` | Pass; 15.7 KiB crate |
| `npm run audit:browser -- https://playlist-path-linter.sociobot.in/` | Pass; three findings, enabled download, no overflow/errors/Axe violations |

There were no declared claim commands because `.factory/claims.json` is missing. That absence is finding 2, not a skipped successful check.

## Earlier findings

| Earlier item | Current disposition |
|---|---|
| Verification 1 high: `2024-01-01Tbogus` passed silently | Fixed in `2176827`. Clean tests reject it and the CLI integration test reports `invalid_date_tag` with exit 1. |
| Verification 1 minor: browser audit failed when `.factory/evidence/` did not exist | Fixed in `2176827`. The script creates the directory and passed from this clean checkout. |
| Verification 2: deployment identity | Confirmed again. Eight live resources byte-match the build. |
| Verification 2 limitation: no alternate service-worker version to exercise | Still no alternate version was available. Current registration, offline reload, cache version, update state, and old-cache deletion logic were checked. No public cross-version update claim was found. |

## Required next work

1. Implement the CLI and web demo contract with bundled examples, isolation, direct `/demo`, one-click populated output, persistent label, reset, and safe exit.
2. Add `.factory/claims.json` and one tagged sandbox test for every retained public claim.
3. Rewrite the first screen and section headings in literal language and add the copy audit.
4. Add the missing 404, metadata, navigation, footer information, manifest icons, and mobile touch target sizing.
5. Repeat the strict review. PASS requires zero findings and zero untested claims.
