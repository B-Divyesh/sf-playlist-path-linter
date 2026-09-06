# Verification 3 — PASS

**Verdict: PASS**

- Findings: **0**
- Untested public claims: **0**
- Implementation reviewed: `1605c2c70dc7de2501155c36d339e66b9a452e5e`
- Documentation handoff reviewed: `ac04e36b4e7588a8b245b0d78b25f2f3f796a7a7`
- Live URL: <https://playlist-path-linter.sociobot.in/>
- Verified: 2026-09-06 UTC

This independent verification passed. The deployed site matches the implementation candidate for the checked static resources. The later documentation commit changes reports only.

## First screen

On fresh 1440 × 960 desktop and 390 × 844 phone contexts, before scrolling:

- **Job:** “Check playlist paths before import.”
- **Audience:** musicians moving local playlists between tools.
- **First action:** **Try it with sample data**, with “Opens three known problems right away.”

The action was visible without scrolling at both sizes. The three facts were “Runs locally,” “No account,” and “Free and MIT licensed.” Screenshots are at `/work/.evidence/verification-3/desktop-first-screen.png` and `/work/.evidence/verification-3/phone-first-screen.png`.

## Clean-checkout and packaged artifact

The clean checkout used Node 22.23.2, npm 10.9.8, Rust/Cargo 1.98.0. All commands passed:

```sh
npm ci
npm test
npm run build
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
npx tsc --noEmit --target ES2022 --module ESNext --moduleResolution bundler \
  --lib ES2022,DOM,DOM.Iterable --allowJs site/src/main.ts
cargo package --manifest-path crates/playlist-path-linter/Cargo.toml --allow-dirty
npm run audit:browser -- https://playlist-path-linter.sociobot.in/
```

`npm test` passed 7 Rust unit tests, 7 Rust CLI integration tests, 3 browser-core tests, and the 24-claim suite. Build produced `target/release/playlist-path-linter` and `dist/site/`. The browser audit found three expected sample findings, an enabled corrected-copy download, no console errors, no horizontal overflow, and zero Axe violations.

The packed crate was unpacked into a fresh temporary consumer and installed with `cargo install --path <unpacked crate> --root <empty root> --force`. The installed public binary showed useful top-level help. `playlist-path-linter demo` created a unique temporary workspace, reported NFC/NFD, case, and missing-path findings, printed the corrected-copy and workspace paths, kept its source unchanged, and exited 1 as documented for a demo with findings.

## Claims

All 24 entries in `.factory/claims.json` have one tagged command and every declared command was run individually from this checkout. Each passed:

| Claim IDs | Result |
| --- | --- |
| `m3u-m3u8`, `root-resolution`, `mixed-separators`, `unicode-normalization`, `case-mismatch`, `missing-ambiguous`, `outside-root`, `remote-no-fetch` | Pass |
| `invalid-dates`, `comments-extinf`, `unambiguous-corrections`, `encoding-preservation`, `json-report`, `exit-codes`, `case-modes`, `read-only` | Pass |
| `cli-no-network`, `browser-local`, `worksheet-refresh`, `folder-names`, `offline-reload`, `demo-known-problems`, `clean-setup`, `free-mit` | Pass |

The commands exercised fresh temporary libraries/browser contexts as declared. There are no missing, duplicate, false, incomplete, or untested public claims. Page and README claim wording matches the registry.

## Live browser and product paths

Fresh desktop and phone browser contexts passed with no console or page errors and no horizontal overflow.

- `/demo/` immediately rendered exactly three realistic findings. Its persistent label read “Demo — sample data, nothing is saved to your real worksheet.” **Reset demo** restored three findings. **Start for real** removed `demo:playlist-path-linter:worksheet`, returned home, and left the normal worksheet blank. No non-demo localStorage keys were present.
- Normal worksheet: an exact path returned `Clean: 1 of 1 entries resolved.` Empty input returned `empty_playlist`; parent traversal returned `outside_root`; a missing path returned `missing_path`; adding that known library path recovered to clean.
- Keyboard: the skip link focused and moved to main; the normal controls operated with keyboard. At phone width, visible controls met the 44 px target baseline; no tested button/select/primary action was smaller.
- Accessibility: live Axe WCAG A/AA/2.1 A/AA found zero violations at desktop and phone sizes. The orange focus treatment was visible. With reduced motion, animation/transition durations were `0.01ms`.
- Privacy: the live demo had no third-party requests; the browser-local claim test also recorded only same-origin requests. No account, cookies, analytics, ads, third-party scripts, or uploads were observed.
- Offline: after a normal online reload, service worker control was active. A fresh offline reload of `/demo/` retained its title/H1 and all three findings without errors.
- Routes: `/`, `/demo/`, `/privacy/`, and `/terms/` returned 200 with their own correct titles, one H1, and main landmark. An unknown route returned deliberate HTTP 404 with `Page not found — Playlist Path Linter`; this is expected behavior, not a defect. All 49 crawled internal/external links returned success.

## Deployment identity and policy

SHA-256 byte matches were confirmed between `dist/site/` and live `/`, `/demo/`, `/privacy/`, `/terms/`, `/404.html`, `/sw.js`, manifest, robots, sitemap, hero image, and social card. Live HTTPS headers included HSTS, self-only CSP with `frame-ancestors 'none'`, `Referrer-Policy: no-referrer`, `X-Content-Type-Options: nosniff`, and restrictive camera/microphone/geolocation policy.

`scripts/verify-url.sh` is not present in this repository, so it could not be run. Its requested title/lang/main/alt/console coverage was independently performed by the live browser audit and manual route checks above; all passed.

## Earlier findings disposition

| Earlier finding | Current disposition |
| --- | --- |
| Malformed date suffix accepted | Fixed; invalid-date claim and CLI regression coverage pass. |
| Browser audit lacked its evidence directory | Fixed; current browser audit creates it and passes. |
| Browser and CLI demo sandbox absent | Fixed; direct populated demo, isolation, reset/exit, bundled CLI sample, and packed-artifact demo pass. |
| Claims registry/tagged tests absent | Fixed; 24 declared commands were individually executed and passed. |
| First-screen/plain-word audit incomplete | Fixed; job, audience, action, literal copy, and copy audit are present. |
| Routes, metadata, navigation, icons, and designed 404 incomplete | Fixed; route/title/metadata checks and intentional HTTP 404 pass. |
| Mobile controls below 44 px | Fixed; phone measurements pass. |

## Finding summary

No findings of any severity. No public claims remain untested. **PASS.**
