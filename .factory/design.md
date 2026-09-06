# Visual thesis — Blueprint drafting sheet

Playlist Path Linter lives at a boundary where invisible differences matter: two paths can look identical while their bytes disagree. The site therefore feels like a working audio-library blueprint rather than a marketing template. A precise drawing grid, registration marks, measured rules, and an exploded path diagram make resolution logic visible. Orange pencil marks identify faults; green inspection stamps confirm a clean handoff.

## Palette

This is intentionally a single light mode, like a physical drafting sheet under neutral studio light. Painting the page explicitly keeps the metaphor coherent and avoids treating dark mode as a cosmetic inversion.

| Token | Value | Use |
| --- | --- | --- |
| Paper | `#f4f0e6` | canvas |
| Sheet | `#fffdf7` | raised work areas |
| Blueprint ink | `#123a59` | primary text and rules |
| Muted ink | `#526a78` | secondary text |
| Cyan line | `#8cb9c6` | grid and construction lines |
| Inspection orange | `#a63f12` | actions, warnings, focus |
| Pass green | `#176044` | verified state |
| Fault red | `#922f35` | destructive/error state |

All body pairings exceed 4.5:1. State always has an icon or label in addition to color.

## Type and spacing

- Headings: `Arial Narrow`, `Aptos Narrow`, system sans; condensed drawing-title character without a font download.
- Body and labels: `ui-monospace`, `SFMono-Regular`, `Cascadia Code`, monospace; paths stay visually inspectable.
- Scale: 14 / 16 / 20 / 28 / clamp(42–72) px. Body never drops below 16 px.
- Spacing follows a 4 px drafting unit, mostly 8 / 12 / 16 / 24 / 32 / 48 / 72.
- Content is capped at 1180 px; prose measures 60–72 characters. At 390 px, the hero drawing follows the intro and comparison columns become a single vertical run.

## Interaction grammar

Controls resemble labeled drafting tools: square corners clipped by one chamfer, 2 px ink outlines, 44 px minimum targets, and a visible orange offset focus ring. The live specimen is a worksheet, not a faux terminal: paste or load an M3U, optionally add known library paths, then inspect a line-addressed report. Results announce through a polite live region. Download only becomes available after a lint pass.

## Motion

One 260 ms reveal moves inspection marks upward by 8 px when a result is produced; button presses move 1 px like a physical tool. There are no loops. With `prefers-reduced-motion: reduce`, transforms and smooth scrolling are removed and state changes are immediate.

## Original asset plan and provenance

- `site/public/blueprint-path-study.webp`: generated specifically for this product using the factory image generator on 2026-08-27. Prompt: “Editorial technical blueprint illustration on warm ivory drafting paper, top-down exploded diagram of an audio playlist path passing through Unicode normalization and file matching checkpoints, navy cyanographic ink, cyan construction grid, rust-orange inspection pencil circles, small abstract waveform and folder silhouettes, tactile screen-print grain, lots of clean negative space, no legible words, no letters, no logos, no watermark, landscape.” Generator: factory `gen-image.sh` / deployment `factory-image`. License: project-owned generated asset. It is explanatory: the same path is shown surviving normalization checkpoints.
- Grid, path connector, status glyphs, and registration marks are hand-authored CSS/SVG primitives in the repository; no stock icons or external assets.
- `site/public/path-linter-demo.webm`: silent screen capture of the production browser worksheet using Playwright/Chromium; project-authored on 2026-08-27. It records the actual product, uses native video controls, and does not autoplay.
- `site/public/social-card.jpg`: a 1200 × 630 crop of the project-owned blueprint study, composed locally on 2026-09-06 for social previews. It preserves the same path-resolution illustration and adds no third-party material.
- `site/public/apple-touch-icon.png`, `site/public/app-icon-192.png`, and `site/public/app-icon-512.png`: hand-composed raster versions of the project’s navy path waveform and orange inspection point on the paper palette, made locally on 2026-09-06. They contain no stock assets.

The generated hero art is WebP and remains below 300 KB. The 1200 × 630 social crop is 148 KB. All runtime assets are local; no third-party scripts, fonts, trackers, or network calls.
