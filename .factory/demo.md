# Demo sandbox

## Browser demo

- URL: `https://playlist-path-linter.sociobot.in/demo/`
- First-screen action: **Try it with sample data** on `/` opens the URL in one click.
- Sample: one M3U8 playlist with an NFC/NFD mismatch, a case mismatch, and a missing path. It renders all three findings on load.
- Isolation: demo edits use only the `demo:playlist-path-linter:worksheet` localStorage key. The normal worksheet uses no storage. Demo code does not read or write any non-`demo:` storage key.
- Reset: **Reset demo** clears the demo key, restores the bundled sample, and reruns the inspection.
- Exit: **Start for real** removes the demo key and returns to `/`.

## CLI demo

- Command: `playlist-path-linter demo`
- Sample: the bundled `crates/playlist-path-linter/examples/demo-playlist.m3u8` has the same Unicode, case, and missing-path defects as the browser demo.
- Isolation: the command creates a unique temporary workspace with its own sample library and playlist. It never accepts or reads a user path.
- Output: it prints the workspace path and corrected-copy path. The process exits `1` because the known sample has findings.

The tagged `@claim:demo-known-problems` browser/CLI test runs both demo entries from a fresh state.
