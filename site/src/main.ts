import "./style.css";
import { lintPlaylist } from "./lint.mjs";

const playlist = document.querySelector<HTMLTextAreaElement>("#playlist")!;
const library = document.querySelector<HTMLTextAreaElement>("#library")!;
const caseMode = document.querySelector<HTMLSelectElement>("#case-mode")!;
const form = document.querySelector<HTMLFormElement>("#specimen-form")!;
const results = document.querySelector<HTMLElement>("#results")!;
const status = document.querySelector<HTMLElement>("#status")!;
const download = document.querySelector<HTMLButtonElement>("#download")!;
const sample = document.querySelector<HTMLButtonElement>("#sample")!;
const playlistFile = document.querySelector<HTMLInputElement>("#playlist-file")!;
const libraryFiles = document.querySelector<HTMLInputElement>("#library-files")!;
const offline = document.querySelector<HTMLElement>("#offline")!;
const resetDemo = document.querySelector<HTMLButtonElement>("#reset-demo");
const startReal = document.querySelector<HTMLButtonElement>("#start-real");
const demo = document.body.dataset.demo === "true";
const demoKey = "demo:playlist-path-linter:worksheet";
const demoSample = {
  playlist: "#EXTM3U\n#EXTINF:261,Beyoncé — Halo\nBeyonce\u0301/I Am... Sasha Fierce/Halo.flac\nTalk TALK/Spirit of Eden/Desire.mp3\nMissing/Encore.mp3",
  library: "Beyoncé/I Am... Sasha Fierce/Halo.flac\nTalk Talk/Spirit of Eden/Desire.mp3",
  caseMode: "sensitive"
};
let lastReport: ReturnType<typeof lintPlaylist> | null = null;

sample.addEventListener("click", () => loadSample(true, true));

playlistFile.addEventListener("change", async () => {
  const file = playlistFile.files?.[0];
  if (!file) return;
  setBusy("Reading the playlist locally…");
  try {
    playlist.value = await file.text();
    persistDemo();
    status.textContent = `Loaded ${file.name}. Nothing was uploaded.`;
  } catch {
    status.textContent = "Could not read that playlist. Choose a plain-text M3U or M3U8 file.";
  } finally {
    setBusy();
  }
});

libraryFiles.addEventListener("change", () => {
  setBusy("Indexing file names locally…");
  requestAnimationFrame(() => {
    const paths = [...(libraryFiles.files || [])].map((file) => (file as File & { webkitRelativePath?: string }).webkitRelativePath || file.name);
    library.value = paths.join("\n");
    persistDemo();
    status.textContent = `Indexed ${paths.length} local file path${paths.length === 1 ? "" : "s"}. File contents were not read.`;
    setBusy();
  });
});

playlist.addEventListener("input", persistDemo);
library.addEventListener("input", persistDemo);
caseMode.addEventListener("change", persistDemo);

form.addEventListener("submit", (event) => {
  event.preventDefault();
  runInspection();
});

download.addEventListener("click", () => {
  if (!lastReport) return;
  const blob = new Blob([lastReport.corrected], { type: "audio/x-mpegurl;charset=utf-8" });
  const link = document.createElement("a");
  link.href = URL.createObjectURL(blob);
  link.download = "playlist.fixed.m3u8";
  link.click();
  URL.revokeObjectURL(link.href);
  status.textContent = "Corrected copy downloaded. Your pasted source is unchanged.";
});

resetDemo?.addEventListener("click", () => {
  localStorage.removeItem(demoKey);
  loadSample(true, true);
  status.textContent = "Demo reset. Three known problems are shown again.";
});

startReal?.addEventListener("click", () => {
  localStorage.removeItem(demoKey);
  location.assign("/");
});

function loadSample(announce: boolean, reset = false) {
  const saved = demo && !reset ? readDemo() : null;
  const values = saved || demoSample;
  playlist.value = values.playlist;
  library.value = values.library;
  caseMode.value = values.caseMode;
  persistDemo();
  runInspection(announce ? "Sample inspected" : undefined);
}

function readDemo() {
  try {
    const parsed = JSON.parse(localStorage.getItem(demoKey) || "null");
    if (parsed && typeof parsed.playlist === "string" && typeof parsed.library === "string" && (parsed.caseMode === "sensitive" || parsed.caseMode === "insensitive")) return parsed;
  } catch {
    // An invalid demo value is discarded by the next write.
  }
  return null;
}

function persistDemo() {
  if (!demo) return;
  localStorage.setItem(demoKey, JSON.stringify({ playlist: playlist.value, library: library.value, caseMode: caseMode.value }));
}

function runInspection(prefix?: string) {
  try {
    lastReport = lintPlaylist(playlist.value, library.value, caseMode.value === "sensitive");
    render(lastReport, prefix);
    download.disabled = !lastReport.summary.entries;
  } catch {
    status.textContent = "The inspection could not finish. Check the pasted text and try again.";
  }
}

function render(report: ReturnType<typeof lintPlaylist>, prefix?: string) {
  const summary = report.summary;
  const outcome = report.clean
    ? `Clean: ${summary.resolved} of ${summary.entries} entries resolved.`
    : `${summary.findings} finding${summary.findings === 1 ? "" : "s"} across ${summary.entries} entries.`;
  status.textContent = prefix ? `${prefix}: ${outcome}` : outcome;
  results.className = `results ${report.clean ? "is-clean" : "has-findings"}`;
  if (report.clean) {
    results.innerHTML = '<div class="empty-result"><span aria-hidden="true">✓</span><div><strong>Ready for import</strong><p>Every entry has an exact library path.</p></div></div>';
  } else {
    results.innerHTML = report.findings.map((finding) => `<article class="finding finding--${finding.severity}"><p class="finding-meta"><span>${escapeHtml(finding.severity)}</span> ${finding.line ? `LINE ${finding.line}` : "PLAYLIST"} · ${escapeHtml(finding.code)}</p><code>${escapeHtml(finding.path || "No track entries")}</code><p>${escapeHtml(finding.message)}</p>${finding.candidates.length ? `<ul>${finding.candidates.map((candidate) => `<li><code>${escapeHtml(candidate)}</code></li>`).join("")}</ul>` : ""}</article>`).join("");
  }
  results.setAttribute("tabindex", "-1");
  results.focus({ preventScroll: true });
}

function setBusy(message?: string) {
  form.toggleAttribute("aria-busy", Boolean(message));
  if (message) status.textContent = message;
}

function escapeHtml(value: string) {
  return value.replace(/[&<>"]/g, (character) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" })[character]!);
}

function updateOffline() { offline.hidden = navigator.onLine; }
window.addEventListener("online", updateOffline);
window.addEventListener("offline", updateOffline);
updateOffline();

if (demo) loadSample(true);
const secureLocalPreview = location.hostname === "localhost" || location.hostname === "127.0.0.1";
if ("serviceWorker" in navigator && (location.protocol === "https:" || secureLocalPreview)) navigator.serviceWorker.register("/sw.js").catch(() => undefined);
