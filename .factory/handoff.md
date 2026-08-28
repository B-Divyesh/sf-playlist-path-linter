# Handoff — Verification 1

## FAIL — do not release `f2c9353f2eea7f8833c016cad5f356b87e10bb6f`

Verified on 2026-08-28 UTC against commit `f2c9353f2eea7f8833c016cad5f356b87e10bb6f` and https://playlist-path-linter.sociobot.in/.

The live deployment byte-matches the candidate and passed build, package, CLI path-resolution, browser, privacy, PWA offline, accessibility, response-header, and performance checks. It fails the researched acceptance contract because the CLI reports a WAV recording-date tag of `2024-01-01Tbogus` as clean (exit 0, no JSON finding). The product promises to diagnose invalid date tags and its own documented accepted forms stop at `YYYY-MM-DD`; malformed timestamps can therefore be silently handed to an importer.

Detailed independent evidence and exact commands are in `.factory/verification-1.md`.

## How to reproduce

```sh
npm ci
npm test
npm run build
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
cargo package --manifest-path crates/playlist-path-linter/Cargo.toml --allow-dirty
```

Run the release CLI against a fixture whose resolved WAV has RIFF `INFO/ICRD` text `2024-01-01Tbogus`; it incorrectly exits 0 and emits `clean: true`. A `2023-02-29` control fixture returns the expected `invalid_date_tag` finding.

## Next step

Tighten date validation, add regression coverage for malformed timestamp suffixes, rebuild, and request re-verification. No product code, deployment, publishing, billing, or external service state was changed by this verification.
