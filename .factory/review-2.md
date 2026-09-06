# Review 2 — Check playlist paths before import

**Verdict: PASS**

- Findings: **0**
- Untested public claims: **0**
- Implementation reviewed: `1605c2c70dc7de2501155c36d339e66b9a452e5e`
- Documentation/report HEAD reviewed: `a8e7b84ac690dd63db93f2f87b2b51fc55de6e21`
- Live URL: <https://playlist-path-linter.sociobot.in/>
- Reviewed: 2026-09-06 UTC

The live product matches the implementation candidate. The commits after `1605c2c` change only `.factory/handoff.md` and `.factory/verification-3.md`, so they do not require another product image.

## Before scrolling

Fresh 1440 × 960 desktop and 390 × 844 phone browsers showed:

- **Job:** “Check playlist paths before import.”
- **Audience:** musicians moving local playlists between tools.
- **First action:** **Try it with sample data**.
- **What happens next:** “Opens three known problems right away.”

The action was fully visible without scrolling at both sizes. Body text was 16 px, the page did not overflow horizontally, and every visible action measured at least 44 × 44 CSS px. Screenshots are in `/work/.evidence/review-2/desktop-first-screen.png` and `/work/.evidence/review-2/phone-first-screen.png`.

## Sample and real-data isolation

The first action opened `/demo/` in one click and immediately showed three realistic findings:

1. `unicode_normalization` for an NFD playlist path.
2. `case_mismatch` for a differently cased library path.
3. `missing_path` for an absent track.

The page kept the label “Demo — sample data, nothing is saved to your real worksheet” visible. Demo edits persisted only under `demo:playlist-path-linter:worksheet`. **Reset demo** restored all three findings. **Start for real** removed that key, returned home, and left both normal worksheet fields blank. No normal-workspace storage was read or written.

The populated desktop and phone captures are in `/work/.evidence/review-2/desktop-populated.png` and `/work/.evidence/review-2/phone-populated.png`. Machine-readable results are in `/work/.evidence/review-2/live-browser-review.json`.

## Browser paths

| Path | Observed result |
| --- | --- |
| Exact path | `Clean: 1 of 1 entries resolved.` |
| Empty M3U | `empty_playlist` |
| Parent traversal | `outside_root` |
| Missing path | `missing_path` |
| Add the missing library path | Recovered to clean |
| Remote URL | `remote_entry`; no outbound request |
| Demo edit and reload | Stayed inside the demo namespace |
| Reset and exit | Sample restored, then demo storage removed |

The normal worksheet started empty, explained its next step, and cleared on reload. The browser worksheet intentionally checks path text only; native audio date checks remain in the CLI.

## Clean checkout and installed artifact

A clean checkout of `a8e7b84` used Node 22.23.2, npm 10.9.8, Rust 1.98.0, and Cargo 1.98.0. After `npm ci`, all commands passed:

```sh
npm test
npm run build
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
npx tsc --noEmit --target ES2022 --module ESNext --moduleResolution bundler \
  --lib ES2022,DOM,DOM.Iterable --allowJs site/src/main.ts
cargo package --manifest-path crates/playlist-path-linter/Cargo.toml --allow-dirty
npm run audit:browser -- https://playlist-path-linter.sociobot.in/
```

`npm test` passed 7 Rust unit tests, 7 CLI integration tests, 3 browser-core tests, 24 tagged claim tests, and the designed-404 regression test. `npm run build` produced the release binary and `dist/site/`. The browser audit reported three findings, an enabled corrected-copy download, no load errors, no overflow, and no Axe violations.

The packaged crate contained 12 files and was installed into an empty Cargo root from its unpacked archive. The installed binary showed useful top-level help. `playlist-path-linter demo` then:

- created a unique temporary workspace;
- reported the Unicode, case, and missing-path problems;
- printed the workspace and corrected-copy paths;
- left the bundled source unchanged;
- produced the corrected copy; and
- exited `1`, as documented for a report with findings.

Installed-artifact output is in `/work/.evidence/review-2/packed-cli-demo.txt`.

## Public claims

`.factory/claims.json` declares 24 claims. Source inspection found exactly one matching `@claim:<id>` test for every entry and no undeclared tags. Every declared command was run individually from the clean checkout and passed.

| Claim IDs | Result |
| --- | --- |
| `m3u-m3u8`, `root-resolution`, `mixed-separators`, `unicode-normalization` | Pass |
| `case-mismatch`, `missing-ambiguous`, `outside-root`, `remote-no-fetch` | Pass |
| `invalid-dates`, `comments-extinf`, `unambiguous-corrections`, `encoding-preservation` | Pass |
| `json-report`, `exit-codes`, `case-modes`, `read-only` | Pass |
| `cli-no-network`, `browser-local`, `worksheet-refresh`, `folder-names` | Pass |
| `offline-reload`, `demo-known-problems`, `clean-setup`, `free-mit` | Pass |

The landing page, legal pages, README, CLI help, and install/demo copy were cross-checked against the registry. No missing, duplicate, incomplete, false, or untested public claim was found. Individual command results and full output are in `/work/.evidence/review-2/claim-commands.json` and `/work/.evidence/review-2/claim-commands.log`.

## Accessibility, keyboard, mobile, and motion

- `/opt/fleet/lib/verify-url.sh` passed HTTPS, title, language, one H1, main landmark, image alt text, button labels, and console checks.
- Playwright Axe WCAG 2.0/2.1 A/AA checks found zero violations on home, demo, privacy, terms, and the designed 404 at phone size.
- The skip link was first in keyboard order. Enter moved to `#main`.
- Focus used a visible 3 px orange outline.
- Forms had bound labels and the result status used a polite live region.
- Phone and desktop layouts had no horizontal overflow.
- With reduced motion enabled, animation and transition durations were `0.01ms`.
- No flashing, autoplay, keyboard trap, or missing image alternative was found.

The expected browser console resource message while directly opening the deliberate HTTP 404 was classified as 404 evidence, not a page defect. Normal 200 routes loaded without console or page errors.

## Offline, privacy, routes, and links

- After an online visit, the demo was service-worker controlled under cache `ppl-shell-v2`.
- A fresh offline reload retained `Demo — Playlist Path Linter`, its H1, offline notice, and all three findings.
- No waiting service worker existed. The worker deletes older named caches on activation. There is no public cross-version update claim.
- All worksheet and demo requests were same-origin. No account, cookie, analytics, ad, upload, third-party script, or music-service request was observed.
- `/`, `/demo/`, `/privacy/`, and `/terms/` returned 200 with their own title, one H1, correct heading order, and main landmark.
- An unknown address returned deliberate HTTP 404 with `Page not found — Playlist Path Linter` and a working return action.
- All 12 unique link targets across the checked pages returned success, including the explicit external source link.
- Live headers included HSTS, a self-only CSP with `frame-ancestors 'none'`, `Referrer-Policy: no-referrer`, `X-Content-Type-Options: nosniff`, and restrictive device permissions.

This is a static site and local CLI, so backend tenant isolation, restart persistence, health endpoints, server request allowances, and 429 handling do not apply.

## Deployment and performance

Thirteen built resources byte-matched the live deployment, including home, demo, privacy, terms, the worker, manifest, sitemap, hashed JS/CSS, product images, and the designed 404 body. The unknown route also returned the expected HTTP 404 status. Checksums are in `/work/.evidence/review-2/deployment-match.json`.

The initial JavaScript is 6,781 bytes, CSS is 11,443 bytes, and the hero WebP is 83,652 bytes. No webfont ships. A fresh live mobile Lighthouse run scored Performance 100, Accessibility 100, Best Practices 100, and SEO 100. FCP was 0.89 s, LCP 1.20 s, TBT 63 ms, and CLS 0. The full result is `/work/.evidence/review-2/lighthouse.json`.

## Earlier findings

| Earlier item | Current disposition |
| --- | --- |
| Malformed date suffix passed silently | Fixed. The clean release binary and `invalid-dates` claim reject malformed suffixes; regression tests pass. |
| Browser audit failed when its evidence directory was absent | Fixed. The clean-checkout browser audit created its directory and passed. |
| Browser and CLI demo sandbox was absent | Fixed. The one-click populated browser demo, separate storage, reset/exit, bundled CLI demo, and retained workspace all passed. |
| Claims registry and tagged tests were absent | Fixed. All 24 declarations have one tagged test and every declared command passed individually. |
| First-screen and section copy was not literal | Fixed. The job, audience, first action, outcome note, facts, and copy audit are present and plain. |
| Required route metadata, navigation, icons, and 404 were incomplete | Fixed. Metadata, route titles, navigation, icons, sitemap, legal routes, and designed HTTP 404 passed. |
| Three phone actions were shorter than 44 px | Fixed. No visible phone action measured below 44 × 44 px. |
| Verification 2 had no alternate worker version to exercise | No product claim depends on a cross-version update. Current control, offline reload, cache version, no waiting worker, and old-cache cleanup were verified. |
| Earlier deployment identity check | Confirmed again with 13 live byte matches. |

The deterministic path checker does not need an AI step. It already provides the brief's useful next action: an unambiguous corrected copy and JSON report. No missed AI, import/export, or sync feature was found within v1 scope.

## Finding summary

No finding of any severity was found. No public claim remains untested. **PASS.**
