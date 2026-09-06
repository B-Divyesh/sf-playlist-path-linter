# Handoff — repair 2

## Release

- **Implementation SHA:** `1605c2c70dc7de2501155c36d339e66b9a452e5e`
- **Implementation commits:** `b34044d` adds the isolated demos, claims suite, copy, metadata, and site structure; `1605c2c` corrects live 404 handling.
- **Live URL:** <https://playlist-path-linter.sociobot.in/>
- **Static deployment:** `2787a074-a9ec-4748-86ac-3601366cd750`, followed by the 404 configuration deployment for `1605c2c`.
- **Billing:** not applicable. The researched brief specifies a free MIT utility; no paid offer or billing metadata is required.

## What changed

- Added the required one-click demo at `/demo/`. It opens three realistic findings immediately, keeps a persistent demo label, supports **Reset demo** and **Start for real**, and uses only `demo:playlist-path-linter:worksheet` storage.
- Added `playlist-path-linter demo`. It creates a unique temporary sample library, reports Unicode, case, and missing-path defects, writes a corrected copy, prints the workspace path, and leaves user files untouched. The packed crate contains its bundled sample.
- Added shipped sample inputs under `examples/` and `crates/playlist-path-linter/examples/`, plus `.factory/demo.md`.
- Added `.factory/claims.json` with 24 public claims. Each has exactly one tagged, outcome-based CLI or browser test in `tests/claims.test.mjs`.
- Rewrote the landing and README in plain words. The first screen now states the job, audience, and first action. `.factory/copy-audit.md` records sentence counts and terminology.
- Added `/demo/`, `/privacy/`, `/terms/`, and designed `/404.html` structure; per-route titles and metadata; canonical, Open Graph, Twitter, social image, manifest icons, sitemap, complete navigation/footer, and live HTTP 404 behavior.
- Corrected all mobile controls to at least 44 × 44 CSS px. Added focusability to the horizontally scrollable CLI output.
- Added product-owned social and app-icon assets. Their provenance is recorded in `.factory/design.md`.
- Added `.factory/catalog-description.txt` and copied its exact verb-first description to `/work/.evidence/catalog-description.txt`.

## Verification

From a clean `npm ci` install:

```sh
npm test
npm run build
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
npx tsc --noEmit --target ES2022 --module ESNext --moduleResolution bundler \
  --lib ES2022,DOM,DOM.Iterable --allowJs site/src/main.ts
cargo package --manifest-path crates/playlist-path-linter/Cargo.toml --allow-dirty
npm run audit:browser -- http://127.0.0.1:4173/
```

All commands pass. `npm test` runs 7 Rust unit tests, 7 Rust CLI integration tests, 3 browser-core tests, 24 tagged claim tests, and one static 404 regression test. Every command in `.factory/claims.json` was also run individually and passed.

The packaged crate was unpacked into a clean temporary consumer, installed with `cargo install --path`, and verified with `--help` and `demo`. The installed binary reported all three known sample defects and named its retained workspace.

Local and live browser checks passed on 1440 × 960 and 390 × 844:

- first-screen action visible before scrolling;
- one-click sample has three findings, persistent demo label, reset, and safe exit;
- keyboard flow, focus rings, no horizontal overflow, and all visible interactive targets at least 44 × 44 px;
- `verify-url.sh` passed title, `lang`, landmark, image-alt, button, and console checks;
- Playwright Axe WCAG A/AA integration found zero violations locally and live;
- normal worksheet reload clears its fields; demo storage is separate; request capture found only same-origin requests;
- after service-worker control, a fresh live offline reload retained the demo title, H1, and three findings;
- `/definitely-not-a-real-route` now returns HTTP 404 with `Page not found — Playlist Path Linter`;
- root, demo, privacy, terms, 404, worker, manifest, images, and hashed assets byte-match the deployed build.

Lighthouse mobile production-preview result: Performance **100**, Accessibility **100**, Best Practices **100**, SEO **100**; FCP 0.95 s, LCP 1.65 s, TBT 0 ms, CLS 0. The standalone Axe CLI could not start its bundled Chrome in this container; the repository’s Playwright Axe integration passed with the installed Chromium instead.

## Earlier findings

| Earlier finding | Current disposition |
| --- | --- |
| Malformed date suffix accepted | Fixed earlier and retained: malformed RIFF date integration coverage passes. |
| Browser audit failed without evidence directory | Fixed earlier and retained: the audit creates the directory. |
| Missing isolated browser and CLI demo | Fixed with `/demo/`, `playlist-path-linter demo`, shipped fixtures, reset, and isolation checks. |
| Missing claims registry and tests | Fixed with 24 declared public claims and individually-run tagged checks. |
| Non-literal first-screen copy and no copy audit | Fixed with literal headings, named audience/action, and `.factory/copy-audit.md`. |
| Missing route metadata, nav/footer, icons, and 404 | Fixed. A live fallback issue found during this repair was corrected in `1605c2c`; unknown paths now return HTTP 404. |
| Mobile controls below 44 px | Fixed and measured at 390 px. |

## Known scope boundary

The browser worksheet inspects playlist path text only. Native media date-tag inspection remains in the CLI, where the local audio files are available. This is intentional and documented; the browser never uploads or reads audio contents.
