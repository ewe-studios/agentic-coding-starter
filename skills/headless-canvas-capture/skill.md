---
name: "Headless Canvas Capture"
description: "Capture and verify WebGL and Canvas 2D pages with Playwright: SwiftShader flags, deterministic waits, element screenshots, offline file:// checks, and console reporting."
approved: No
created: 2026-11-12
license: "MIT"
metadata:
  author: "Main Agent"
  version: "1.0"
  last_updated: "2026-11-12"
  tags: [playwright, chromium, screenshot, webgl, canvas2d, verification, offline]
tools: [Read, Write, Edit, Bash]
---

# Headless Canvas Capture

## Overview

This skill drives Chromium with Playwright to screenshot canvas-heavy pages and
to prove that they work offline. It covers the waits that make a WebGL frame
stable, the flags that enable software rendering, and the checks that catch a
blank panel before a screenshot ships.

The same pattern produces documentation images, board thumbnails, and the
`verify-sampler-offline.mjs` report.

## When to Use

Use this skill when:

- A demo must be screenshotted at one or more widths.
- A page uses three.js or Canvas 2D and may fail silently.
- A deliverable claims "works offline"; the claim needs a test.
- A design mockup must be rendered to PNG for review.

## Usage Type

**EXECUTABLE.** Copy the patterns into a `tooling/*.mjs` script. The scripts are
standalone and depend only on `playwright`.

## Core Concepts

### 1. Launch with software WebGL

```js
const browser = await chromium.launch({
  headless: true,
  args: [
    '--enable-webgl',
    '--ignore-gpu-blocklist',
    '--enable-unsafe-swiftshader',
  ],
});
```

SwiftShader is slow but deterministic. Expect 15 to 20 seconds for a large
three.js scene to settle.

### 2. Wait for the page, the fonts, and the scene

`networkidle` alone is not enough. Fonts change metrics, and a scene may build
after the module loads.

```js
await page.goto(url, { waitUntil: 'networkidle', timeout: 60000 });
await page.evaluate(() => document.fonts.ready);
await page.waitForTimeout(1800); // scene settle
```

Better: expose a readiness flag from the page.

```js
// in the page
window.__ready = true;
// in the script
await page.waitForFunction(() => window.__ready === true, { timeout: 60000 });
```

### 3. Screenshot the element, not the viewport

An element shot crops the card and removes surrounding page noise. Playwright
handles elements taller than the viewport.

```js
await page.locator('#board-claw').screenshot({
  path: 'captures/sampler/board-claw.png',
});
```

Set `deviceScaleFactor: 2` for retina output. Keep it at 1 for large boards to
avoid 30 MB PNGs.

### 4. Prove offline with the network blocked

Open the page from a `file://` URL and abort every HTTP request. Record any
request that still starts.

```js
const ctx = await browser.newContext({ offline: true });
const page = await ctx.newPage();
const leaked = [];
await page.route('http://**', route => { leaked.push(route.request().url()); route.abort(); });
await page.route('https://**', route => { leaked.push(route.request().url()); route.abort(); });
await page.goto('file:///abs/path/dearwind-sampler.html');
if (leaked.length) throw new Error('network used: ' + leaked.join(', '));
```

A passing run has zero requests and zero page errors. A local ES module import
under `file://` fails with a CORS/null-origin error. That is the reason the
sampler is one inline classic script.

### 5. Detect blank canvases

A screenshot can be a cream rectangle and still pass. Sample the canvas pixels.

```js
const nonBlank = await page.evaluate(() => {
  const c = document.querySelector('#board-claw canvas.right');
  const g = c.getContext('2d');
  const d = g.getImageData(0, 0, c.width, c.height).data;
  let min = 255, max = 0;
  for (let i = 0; i < d.length; i += 4 * 97) {
    const v = (d[i] + d[i + 1] + d[i + 2]) / 3;
    if (v < min) min = v;
    if (v > max) max = v;
  }
  return max - min;
});
if (nonBlank < 20) throw new Error('canvas looks blank');
```

For WebGL canvases, read the pixels immediately after a render, or copy the
frame into a 2D canvas first. `preserveDrawingBuffer: true` is the alternative,
but it costs performance.

### 6. Collect errors as data

```js
const errors = [];
page.on('pageerror', e => errors.push(String(e).slice(0, 600)));
page.on('console', m => {
  if (m.type() === 'error') errors.push(m.text().slice(0, 400));
});
```

Write a JSON report next to the screenshots. The report is the evidence behind a
documentation claim.

### 7. Capture inside an iframe

The springboard hosts explorations in an iframe. Use `frameLocator`.

```js
const frame = page.frameLocator('#stage');
await frame.locator('#modes').screenshot({ path: 'modes.png' });
```

Wait for the iframe to load and for its fonts to settle.

## Step-by-Step Guide

1. Start the local server.
2. Write the script with the launch flags and the error collectors.
3. Navigate and wait for fonts and the readiness flag.
4. Take one full-page screenshot to check the layout.
5. Take the element screenshots.
6. Run the offline pass with all HTTP aborted.
7. Sample two canvases: one 3D-fed, one pure 2D.
8. Write the JSON report and fail the script on errors.

## Code Examples

### GOOD: explicit readiness

```js
await page.waitForFunction(() => window.__boardsReady === 14);
```

### BAD: a fixed sleep only

```js
await page.waitForTimeout(500);
await page.screenshot(); // may catch the scene mid-build
```

### GOOD: offline test from disk

```js
await page.goto('file:///Users/me/research/explorations/dearwind-sampler.html');
```

### BAD: checking `file://` in Firefox and calling it done

Firefox allows some local module forms that Chrome blocks. Test the browsers the
reader will use, and state which ones passed.

## Common Pitfalls

| Problem | Cause | Fix |
| --- | --- | --- |
| Blank canvas shot | Screenshot before the first frame | Wait for the readiness flag, then one frame |
| Fonts shift layout | Shot before webfonts load | `document.fonts.ready` |
| Offline test passes wrongly | HTTP not blocked | `context.setOffline(true)` plus route aborts |
| Local module fails | Chrome null origin on `file://` | Inline the code or run a server |
| Huge PNGs | `deviceScaleFactor: 2` and large boards | Use 1 for board captures |
| Element shot misses content | Element clipped by `overflow:hidden` | Screenshot the page or the visible child |

## References

- Playwright screenshots: <https://playwright.dev/docs/screenshots>
- Chrome SwiftShader flag: `--enable-unsafe-swiftshader`
- Existing scripts: `src.wavedash/tooling/shot-sampler.mjs`,
  `src.wavedash/tooling/capture-island.mjs`
