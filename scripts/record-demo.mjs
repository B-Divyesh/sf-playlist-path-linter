import { chromium } from "playwright";

const url = process.argv[2] || "http://127.0.0.1:4173";
const executablePath = process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH || "/opt/pw-browsers/chromium-1208/chrome-linux64/chrome";
const browser = await chromium.launch({ executablePath, headless: true });
const context = await browser.newContext({
  viewport: { width: 960, height: 720 },
  recordVideo: { dir: ".factory/evidence/video", size: { width: 960, height: 720 } }
});
const page = await context.newPage();
await page.goto(url, { waitUntil: "networkidle" });
await page.locator("#worksheet").scrollIntoViewIfNeeded();
await page.waitForTimeout(900);
await page.getByRole("button", { name: "Load the sample" }).click();
await page.waitForTimeout(2400);
const video = page.video();
await context.close();
await video.saveAs("site/public/path-linter-demo.webm");
await browser.close();
