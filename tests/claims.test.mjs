import { after, before, test } from "node:test";
import assert from "node:assert/strict";
import { createServer } from "node:http";
import { mkdtemp, readFile, rm, stat, writeFile, mkdir } from "node:fs/promises";
import { existsSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, normalize, extname } from "node:path";
import { execFileSync, spawnSync } from "node:child_process";
import { chromium } from "playwright";

const repository = process.cwd();
const dist = join(repository, "dist/site");
const binary = join(repository, "target/release/playlist-path-linter");
let server;
let origin;
let browser;

before(async () => {
  execFileSync("cargo", ["build", "--workspace", "--release"], { cwd: repository, stdio: "inherit" });
  execFileSync("npm", ["run", "build:site"], { cwd: repository, stdio: "inherit" });
  server = createServer(async (request, response) => {
    const url = new URL(request.url, "http://127.0.0.1");
    let route = decodeURIComponent(url.pathname);
    if (route === "/") route = "/index.html";
    if (route === "/demo" || route === "/demo/") route = "/demo/index.html";
    if (route.endsWith("/")) route += "index.html";
    const candidate = normalize(join(dist, route));
    if (!candidate.startsWith(dist)) {
      response.writeHead(403).end();
      return;
    }
    try {
      const info = await stat(candidate);
      if (!info.isFile()) throw new Error("not a file");
      response.writeHead(200, { "Content-Type": contentType(candidate) });
      response.end(await readFile(candidate));
    } catch {
      response.writeHead(404, { "Content-Type": "text/html; charset=utf-8" });
      response.end(await readFile(join(dist, "404.html")));
    }
  });
  await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
  origin = `http://127.0.0.1:${server.address().port}`;
  const executablePath = process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH || "/opt/pw-browsers/chromium-1208/chrome-linux64/chrome";
  browser = await chromium.launch(existsSync(executablePath) ? { executablePath, headless: true } : { headless: true });
});

after(async () => {
  await browser?.close();
  await new Promise((resolve) => server?.close(resolve));
});

function contentType(path) {
  return ({ ".html": "text/html; charset=utf-8", ".js": "text/javascript", ".css": "text/css", ".webp": "image/webp", ".jpg": "image/jpeg", ".png": "image/png", ".svg": "image/svg+xml", ".webmanifest": "application/manifest+json" })[extname(path)] || "application/octet-stream";
}

async function fixture() {
  const directory = await mkdtemp(join(tmpdir(), "ppl-claim-"));
  const root = join(directory, "library");
  await mkdir(join(root, "Artist"), { recursive: true });
  await writeFile(join(root, "Artist/Song.mp3"), "fixture audio");
  return { directory, root, playlist: join(directory, "set.m3u8") };
}

function run(args) {
  return spawnSync(binary, args, { cwd: repository, encoding: "utf8" });
}

async function runJson(args) {
  const result = run([...args, "--json"]);
  assert.notEqual(result.status, null, result.stderr);
  return { result, report: JSON.parse(result.stdout) };
}

async function withPage(callback) {
  const context = await browser.newContext({ viewport: { width: 1280, height: 900 } });
  const page = await context.newPage();
  try { return await callback(page, context); } finally { await context.close(); }
}

function riffWithIcrd(value) {
  const text = Buffer.from(`${value}\0`);
  const padding = text.length % 2;
  const chunks = [Buffer.from("WAVEfmt "), u32(16), Buffer.from([1, 0, 1, 0]), u32(8000), u32(16000), Buffer.from([2, 0, 16, 0]), Buffer.from("data"), u32(2), Buffer.from([0, 0]), Buffer.from("LIST"), u32(12 + text.length + padding), Buffer.from("INFOICRD"), u32(text.length), text, Buffer.alloc(padding)];
  const body = Buffer.concat(chunks);
  return Buffer.concat([Buffer.from("RIFF"), u32(body.length), body]);
}

function u32(value) { const buffer = Buffer.alloc(4); buffer.writeUInt32LE(value); return buffer; }

test("@claim:m3u-m3u8 reads both M3U and M3U8 playlists", async () => {
  const setup = await fixture();
  try {
    for (const extension of ["m3u", "m3u8"]) {
      const playlist = join(setup.directory, `set.${extension}`);
      await writeFile(playlist, "Artist/Song.mp3\n");
      const { result, report } = await runJson(["lint", playlist, "--root", setup.root, "--no-date-check"]);
      assert.equal(result.status, 0);
      assert.equal(report.summary.resolved, 1);
    }
  } finally { await rm(setup.directory, { recursive: true, force: true }); }
});

test("@claim:root-resolution resolves relative and absolute entries under the library root", async () => {
  const setup = await fixture();
  try {
    await writeFile(setup.playlist, `Artist/Song.mp3\n${join(setup.root, "Artist/Song.mp3")}\n`);
    const { result, report } = await runJson(["lint", setup.playlist, "--root", setup.root, "--no-date-check"]);
    assert.equal(result.status, 0);
    assert.equal(report.summary.resolved, 2);
  } finally { await rm(setup.directory, { recursive: true, force: true }); }
});

test("@claim:mixed-separators resolves Windows and Unix path separators", async () => {
  const setup = await fixture();
  try {
    await writeFile(setup.playlist, "Artist\\Song.mp3\n");
    const { result, report } = await runJson(["lint", setup.playlist, "--root", setup.root, "--no-date-check"]);
    assert.equal(result.status, 0);
    assert.equal(report.summary.resolved, 1);
  } finally { await rm(setup.directory, { recursive: true, force: true }); }
});

test("@claim:unicode-normalization diagnoses NFC and NFD spelling differences", async () => {
  const setup = await fixture();
  try {
    await mkdir(join(setup.root, "Beyoncé"), { recursive: true });
    await writeFile(join(setup.root, "Beyoncé/Halo.mp3"), "fixture");
    await writeFile(setup.playlist, "Beyonce\u0301/Halo.mp3\n");
    const { result, report } = await runJson(["lint", setup.playlist, "--root", setup.root, "--no-date-check"]);
    assert.equal(result.status, 1);
    assert.equal(report.findings[0].code, "unicode_normalization");
  } finally { await rm(setup.directory, { recursive: true, force: true }); }
});

test("@claim:case-mismatch diagnoses case-only differences", async () => {
  const setup = await fixture();
  try {
    await writeFile(setup.playlist, "artist/song.mp3\n");
    const { result, report } = await runJson(["lint", setup.playlist, "--root", setup.root, "--case", "sensitive", "--no-date-check"]);
    assert.equal(result.status, 1);
    assert.equal(report.findings[0].code, "case_mismatch");
  } finally { await rm(setup.directory, { recursive: true, force: true }); }
});

test("@claim:missing-ambiguous reports missing paths and equivalent duplicates", async () => {
  const setup = await fixture();
  try {
    await writeFile(join(setup.root, "Artist/SONG.mp3"), "fixture");
    await writeFile(setup.playlist, "Artist/song.mp3\nMissing/Encore.mp3\n");
    const { result, report } = await runJson(["lint", setup.playlist, "--root", setup.root, "--case", "insensitive", "--no-date-check"]);
    assert.equal(result.status, 1);
    assert.deepEqual(report.findings.map((finding) => finding.code), ["ambiguous_path", "missing_path"]);
    assert.equal(report.findings[0].candidates.length, 2);
  } finally { await rm(setup.directory, { recursive: true, force: true }); }
});

test("@claim:outside-root rejects parent traversal", async () => {
  const setup = await fixture();
  try {
    await writeFile(setup.playlist, "../outside.mp3\n");
    const { result, report } = await runJson(["lint", setup.playlist, "--root", setup.root, "--no-date-check"]);
    assert.equal(result.status, 1);
    assert.equal(report.findings[0].code, "outside_root");
  } finally { await rm(setup.directory, { recursive: true, force: true }); }
});

test("@claim:remote-no-fetch reports remote URLs without requesting them", async () => {
  const setup = await fixture();
  let requests = 0;
  const remote = createServer(() => { requests += 1; });
  await new Promise((resolve) => remote.listen(0, "127.0.0.1", resolve));
  try {
    const address = remote.address();
    await writeFile(setup.playlist, `http://127.0.0.1:${address.port}/track.mp3\n`);
    const { result, report } = await runJson(["lint", setup.playlist, "--root", setup.root, "--no-date-check"]);
    assert.equal(result.status, 1);
    assert.equal(report.findings[0].code, "remote_entry");
    assert.equal(requests, 0);
  } finally {
    await new Promise((resolve) => remote.close(resolve));
    await rm(setup.directory, { recursive: true, force: true });
  }
});

test("@claim:invalid-dates finds year zero, malformed dates, and future dates", async () => {
  const setup = await fixture();
  try {
    for (const [name, value] of [["zero.wav", "0000"], ["malformed.wav", "2024-01-01Tbroken"], ["future.wav", "2999-01-01"]]) {
      await writeFile(join(setup.root, name), riffWithIcrd(value));
      await writeFile(setup.playlist, `${name}\n`);
      const { result, report } = await runJson(["lint", setup.playlist, "--root", setup.root]);
      assert.equal(result.status, 1);
      assert.equal(report.summary.date_issues, 1);
      assert.equal(report.findings[0].code, "invalid_date_tag");
    }
  } finally { await rm(setup.directory, { recursive: true, force: true }); }
});

test("@claim:comments-extinf retains comments and EXTINF records in a corrected copy", async () => {
  const setup = await fixture();
  try {
    await mkdir(join(setup.root, "Beyoncé"), { recursive: true });
    await writeFile(join(setup.root, "Beyoncé/Halo.mp3"), "fixture");
    await writeFile(setup.playlist, "#EXTM3U\r\n#EXTINF:261,Beyoncé — Halo\r\nBeyonce\u0301/Halo.mp3\r\n");
    const fixed = join(setup.directory, "fixed.m3u8");
    const { result } = await runJson(["lint", setup.playlist, "--root", setup.root, "--fix", fixed, "--no-date-check"]);
    assert.equal(result.status, 1);
    const output = await readFile(fixed, "utf8");
    assert.match(output, /^#EXTM3U\r\n#EXTINF:261,Beyoncé — Halo\r\nBeyoncé\/Halo.mp3\r\n$/);
  } finally { await rm(setup.directory, { recursive: true, force: true }); }
});

test("@claim:unambiguous-corrections writes only safe corrections and keeps the source", async () => {
  const setup = await fixture();
  try {
    const source = "Artist/song.mp3\n";
    await writeFile(setup.playlist, source);
    const fixed = join(setup.directory, "fixed.m3u8");
    const { result, report } = await runJson(["lint", setup.playlist, "--root", setup.root, "--fix", fixed, "--no-date-check"]);
    assert.equal(result.status, 1);
    assert.equal(report.summary.corrected, 1);
    assert.equal(await readFile(setup.playlist, "utf8"), source);
    assert.equal(await readFile(fixed, "utf8"), "Artist/Song.mp3\n");
  } finally { await rm(setup.directory, { recursive: true, force: true }); }
});

test("@claim:encoding-preservation retains a UTF-8 BOM and CRLF endings", async () => {
  const setup = await fixture();
  try {
    const input = Buffer.from("\uFEFFArtist/song.mp3\r\n", "utf8");
    await writeFile(setup.playlist, input);
    const fixed = join(setup.directory, "fixed.m3u8");
    const { result } = await runJson(["lint", setup.playlist, "--root", setup.root, "--fix", fixed, "--no-date-check"]);
    assert.equal(result.status, 1);
    const output = await readFile(fixed);
    assert.deepEqual(output.subarray(0, 3), Buffer.from([0xef, 0xbb, 0xbf]));
    assert.equal(output.subarray(3).toString("utf8"), "Artist/Song.mp3\r\n");
  } finally { await rm(setup.directory, { recursive: true, force: true }); }
});

test("@claim:json-report emits a versioned report with line-level findings", async () => {
  const setup = await fixture();
  try {
    await writeFile(setup.playlist, "Missing/Encore.mp3\n");
    const { result, report } = await runJson(["lint", setup.playlist, "--root", setup.root, "--no-date-check"]);
    assert.equal(result.status, 1);
    assert.equal(report.schema_version, 1);
    assert.equal(report.findings[0].line, 1);
    assert.equal(report.findings[0].code, "missing_path");
    assert.equal(typeof report.findings[0].severity, "string");
  } finally { await rm(setup.directory, { recursive: true, force: true }); }
});

test("@claim:exit-codes uses 0 for clean, 1 for findings, and 2 for errors", async () => {
  const setup = await fixture();
  try {
    await writeFile(setup.playlist, "Artist/Song.mp3\n");
    assert.equal(run(["lint", setup.playlist, "--root", setup.root, "--no-date-check"]).status, 0);
    await writeFile(setup.playlist, "Missing/Encore.mp3\n");
    assert.equal(run(["lint", setup.playlist, "--root", setup.root, "--no-date-check"]).status, 1);
    assert.equal(run(["lint", setup.playlist, "--root", join(setup.directory, "missing-root")]).status, 2);
  } finally { await rm(setup.directory, { recursive: true, force: true }); }
});

test("@claim:case-modes reports the selected sensitive, insensitive, and automatic model", async () => {
  const setup = await fixture();
  try {
    await writeFile(setup.playlist, "Artist/Song.mp3\n");
    const sensitive = await runJson(["lint", setup.playlist, "--root", setup.root, "--case", "sensitive", "--no-date-check"]);
    const insensitive = await runJson(["lint", setup.playlist, "--root", setup.root, "--case", "insensitive", "--no-date-check"]);
    const automatic = await runJson(["lint", setup.playlist, "--root", setup.root, "--case", "auto", "--no-date-check"]);
    assert.equal(sensitive.report.case_sensitive, true);
    assert.equal(insensitive.report.case_sensitive, false);
    assert.equal(typeof automatic.report.case_sensitive, "boolean");
  } finally { await rm(setup.directory, { recursive: true, force: true }); }
});

test("@claim:read-only leaves playlists and media files unchanged", async () => {
  const setup = await fixture();
  try {
    const media = join(setup.root, "Artist/Song.mp3");
    const beforeMedia = await readFile(media);
    const source = "Artist/Song.mp3\n";
    await writeFile(setup.playlist, source);
    const { result } = await runJson(["lint", setup.playlist, "--root", setup.root, "--no-date-check"]);
    assert.equal(result.status, 0);
    assert.equal(await readFile(setup.playlist, "utf8"), source);
    assert.deepEqual(await readFile(media), beforeMedia);
  } finally { await rm(setup.directory, { recursive: true, force: true }); }
});

test("@claim:cli-no-network completes remote-entry linting without contacting the remote host", async () => {
  const setup = await fixture();
  let hits = 0;
  const remote = createServer(() => { hits += 1; });
  await new Promise((resolve) => remote.listen(0, "127.0.0.1", resolve));
  try {
    const { port } = remote.address();
    await writeFile(setup.playlist, `https://127.0.0.1:${port}/metadata.mp3\n`);
    const outcome = run(["lint", setup.playlist, "--root", setup.root, "--no-date-check"]);
    assert.equal(outcome.status, 1);
    assert.equal(hits, 0);
  } finally { await new Promise((resolve) => remote.close(resolve)); await rm(setup.directory, { recursive: true, force: true }); }
});

test("@claim:browser-local runs the worksheet without uploads or third-party requests", async () => {
  await withPage(async (page) => {
    const requests = [];
    page.on("request", (request) => requests.push(request.url()));
    await page.goto(`${origin}/`, { waitUntil: "networkidle" });
    await page.locator("#playlist").fill("Artist/Song.mp3");
    await page.locator("#library").fill("Artist/Song.mp3");
    await page.getByRole("button", { name: "Run inspection" }).click();
    await assert.doesNotReject(() => page.getByText("Clean: 1 of 1 entries resolved.").waitFor());
    assert.ok(requests.every((url) => new URL(url).origin === origin));
  });
});

test("@claim:worksheet-refresh clears normal worksheet data after refresh", async () => {
  await withPage(async (page) => {
    await page.goto(`${origin}/`, { waitUntil: "networkidle" });
    await page.locator("#playlist").fill("My/Real/Playlist.flac");
    await page.locator("#library").fill("My/Real/Library.flac");
    await page.reload({ waitUntil: "networkidle" });
    assert.equal(await page.locator("#playlist").inputValue(), "");
    assert.equal(await page.locator("#library").inputValue(), "");
    assert.deepEqual(await page.evaluate(() => Object.keys(localStorage)), []);
  });
});

test("@claim:folder-names reads chosen folder names without reading audio contents", async () => {
  await withPage(async (page) => {
    await page.goto(`${origin}/`, { waitUntil: "networkidle" });
    await page.evaluate(() => {
      const input = document.querySelector("#library-files");
      const file = { name: "Song.flac", webkitRelativePath: "Artist/Album/Song.flac", text() { throw new Error("audio contents must not be read"); } };
      Object.defineProperty(input, "files", { configurable: true, value: [file] });
      input.dispatchEvent(new Event("change"));
    });
    await page.getByText("Indexed 1 local file path. File contents were not read.").waitFor();
    assert.equal(await page.locator("#library").inputValue(), "Artist/Album/Song.flac");
  });
});

test("@claim:offline-reload keeps the worksheet available after the first visit", async () => {
  const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
  const page = await context.newPage();
  try {
    await page.goto(`${origin}/demo/`, { waitUntil: "networkidle" });
    await page.evaluate(async () => { await navigator.serviceWorker.ready; });
    await page.reload({ waitUntil: "networkidle" });
    await context.setOffline(true);
    await page.reload({ waitUntil: "domcontentloaded" });
    assert.equal(await page.title(), "Demo — Playlist Path Linter");
    await assert.doesNotReject(() => page.getByRole("heading", { level: 1, name: "Check sample playlist paths" }).waitFor());
  } finally { await context.close(); }
});

test("@claim:demo-known-problems opens populated sample results in an isolated demo", async () => {
  const cliDemo = run(["demo"]);
  assert.equal(cliDemo.status, 1);
  assert.match(cliDemo.stdout, /\[unicode_normalization\]/);
  assert.match(cliDemo.stdout, /\[case_mismatch\]/);
  assert.match(cliDemo.stdout, /\[missing_path\]/);
  const cliWorkspace = cliDemo.stdout.match(/Demo files kept at: (.+)$/m)?.[1];
  assert.ok(cliWorkspace);
  assert.ok(existsSync(join(cliWorkspace, "sample.fixed.m3u8")));
  await rm(cliWorkspace, { recursive: true, force: true });
  await withPage(async (page) => {
    await page.goto(`${origin}/`, { waitUntil: "networkidle" });
    await page.evaluate(() => localStorage.setItem("real:worksheet", "My/Real/Playlist.flac"));
    await page.getByRole("link", { name: "Try it with sample data" }).click();
    await page.waitForURL(/\/demo\/?$/);
    assert.equal(await page.title(), "Demo — Playlist Path Linter");
    assert.equal(await page.locator(".finding").count(), 3);
    await assert.doesNotReject(() => page.getByText("Demo — sample data, nothing is saved to your real worksheet.").waitFor());
    assert.equal(await page.evaluate(() => localStorage.getItem("real:worksheet")), "My/Real/Playlist.flac");
    assert.ok((await page.evaluate(() => Object.keys(localStorage))).every((key) => key === "real:worksheet" || key.startsWith("demo:")));
    await page.getByRole("button", { name: "Reset demo" }).click();
    assert.equal(await page.locator(".finding").count(), 3);
    await page.getByRole("button", { name: "Start for real" }).click();
    await page.waitForURL(`${origin}/`);
    assert.equal(await page.evaluate(() => localStorage.getItem("real:worksheet")), "My/Real/Playlist.flac");
    assert.equal(await page.evaluate(() => localStorage.getItem("demo:playlist-path-linter:worksheet")), null);
  });
});

test("@claim:clean-setup builds the release binary and static site artifacts", () => {
  assert.ok(existsSync(binary));
  assert.ok(existsSync(join(dist, "index.html")));
  assert.ok(existsSync(join(dist, "demo/index.html")));
  assert.ok(existsSync(join(dist, "404.html")));
});

test("@claim:free-mit works without an account and is MIT licensed", async () => {
  const metadata = JSON.parse(execFileSync("cargo", ["metadata", "--no-deps", "--format-version", "1"], { cwd: repository, encoding: "utf8" }));
  assert.equal(metadata.packages.find((pkg) => pkg.name === "playlist-path-linter").license, "MIT");
  await withPage(async (page) => {
    const requests = [];
    page.on("request", (request) => requests.push(request.url()));
    await page.goto(`${origin}/demo/`, { waitUntil: "networkidle" });
    assert.equal(await page.locator(".finding").count(), 3);
    assert.equal(await page.evaluate(() => document.cookie), "");
    assert.ok(requests.every((url) => new URL(url).origin === origin));
  });
});

test("static site serves a designed 404 response for an unknown address", async () => {
  await withPage(async (page) => {
    const response = await page.goto(`${origin}/not-a-playlist-linter-route`, { waitUntil: "networkidle" });
    assert.equal(response.status(), 404);
    await page.getByRole("heading", { level: 1, name: "This page was not found" }).waitFor();
    assert.equal(await page.title(), "Page not found — Playlist Path Linter");
  });
});
