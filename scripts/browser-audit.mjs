import { mkdir, writeFile } from "node:fs/promises";
import { chromium } from "playwright";
import AxeBuilder from "@axe-core/playwright";

const url = process.argv[2] || "http://127.0.0.1:5173";
const executablePath = process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH || "/opt/pw-browsers/chromium-1208/chrome-linux64/chrome";
const browser = await chromium.launch({ executablePath, headless: true });
const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
const page = await context.newPage();
const consoleErrors = [];
page.on("pageerror", (error) => consoleErrors.push(String(error)));
page.on("console", (message) => { if (message.type() === "error") consoleErrors.push(message.text()); });
await page.goto(url, { waitUntil: "networkidle" });
await page.getByRole("button", { name: "Load the sample" }).click();
const findingCount = await page.locator(".finding").count();
const downloadEnabled = await page.getByRole("button", { name: "Download corrected copy" }).isEnabled();
const noHorizontalOverflow = await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth);
const axe = await new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa"]).analyze();
const report = {
  url,
  consoleErrors,
  findingCount,
  downloadEnabled,
  noHorizontalOverflow,
  violations: axe.violations.map(({ id, impact, help, nodes }) => ({ id, impact, help, nodes: nodes.length }))
};
await mkdir(".factory/evidence", { recursive: true });
await writeFile(".factory/evidence/browser-audit.json", JSON.stringify(report, null, 2));
console.log(JSON.stringify(report, null, 2));
await browser.close();
if (consoleErrors.length || findingCount !== 3 || !downloadEnabled || !noHorizontalOverflow || axe.violations.some((item) => item.impact === "serious" || item.impact === "critical")) process.exit(1);
