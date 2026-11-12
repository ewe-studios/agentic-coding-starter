---
name: "Local Research Hub"
description: "Serve a research repository from one Node server and present every demo and study through a left-rail portal with hash routing, an iframe stage, and a markdown reader."
approved: No
created: 2026-11-12
license: "MIT"
metadata:
  author: "Main Agent"
  version: "1.0"
  last_updated: "2026-11-12"
  tags: [node, static-server, portal, hash-routing, iframe, markdown, local-development]
tools: [Read, Write, Edit, Bash]
---

# Local Research Hub

## Overview

This skill turns a folder of demos, built docs and markdown studies into one
local site. A small Node server serves the repository root. A portal page keeps a
left rail mounted and swaps the right stage through an iframe. Hash routes make
every view linkable. A markdown reader gives raw studies a readable page.

The result is a springboard: one URL, one visual shell, every exploration one
click away.

## When to Use

Use this skill when:

- A repository holds several unrelated HTML pages and documents.
- The pages need one shared navigation shell.
- Local testing needs correct MIME types and video seeking.
- Markdown studies need to be read in the browser without a build step.

Do not use it when a single static page is enough, or when a real framework is
already present.

## Usage Type

**TEMPLATE.** Copy `serve.mjs` and adapt the route table and rail items.

## Core Concepts

### 1. The server is deliberately small

Node built-ins are enough: `http`, `fs`, `path`, `url`. The server must:

- Serve from one root and refuse paths that escape it.
- Resolve a directory to `index.html`.
- Set MIME types, including `text/markdown` and `video/mp4`.
- Support byte ranges so a video scrub bar works.
- Send `Cache-Control: no-store` during authoring.

```js
import http from 'node:http';
import { createReadStream } from 'node:fs';
import { stat } from 'node:fs/promises';
import path from 'node:path';

const root = process.cwd();
const MIME = {
  '.html': 'text/html; charset=utf-8',
  '.css': 'text/css; charset=utf-8',
  '.js': 'text/javascript; charset=utf-8',
  '.mjs': 'text/javascript; charset=utf-8',
  '.json': 'application/json; charset=utf-8',
  '.md': 'text/markdown; charset=utf-8',
  '.png': 'image/png',
  '.jpg': 'image/jpeg',
  '.svg': 'image/svg+xml',
  '.woff2': 'font/woff2',
  '.mp4': 'video/mp4',
  '.webm': 'video/webm',
};

http.createServer(async (req, res) => {
  const url = new URL(req.url, 'http://localhost');
  const rel = decodeURIComponent(url.pathname);
  let file = path.join(root, rel);
  if (!file.startsWith(root + path.sep) && file !== root) {
    res.writeHead(403); return res.end('Forbidden');
  }
  try {
    let info = await stat(file);
    if (info.isDirectory()) {
      file = path.join(file, 'index.html');
      info = await stat(file);
    }
    const type = MIME[path.extname(file).toLowerCase()] || 'application/octet-stream';
    const range = req.headers.range;
    if (range) {
      const [a, b] = range.replace('bytes=', '').split('-');
      const start = a ? Number(a) : 0;
      const end = b ? Number(b) : info.size - 1;
      res.writeHead(206, {
        'Content-Type': type,
        'Accept-Ranges': 'bytes',
        'Content-Range': `bytes ${start}-${end}/${info.size}`,
        'Content-Length': end - start + 1,
        'Cache-Control': 'no-store',
      });
      createReadStream(file, { start, end }).pipe(res);
      return;
    }
    res.writeHead(200, {
      'Content-Type': type,
      'Content-Length': info.size,
      'Accept-Ranges': 'bytes',
      'Cache-Control': 'no-store',
    });
    createReadStream(file).pipe(res);
  } catch {
    res.writeHead(404); res.end('Not found');
  }
}).listen(8300, '127.0.0.1');
```

### 2. One route table

The rail links, the router and the "open in tab" button read the same table.
The hash is the route.

```js
const routes = {
  '/': 'explorations/home.html',
  '/sampler': 'explorations/dearwind-sampler.html',
  '/boards': 'explorations/dearwind-sampler.html#modes',
  '/strawberry': 'explorations/strawberry-corner.html',
  '/docs/mathalley': 'explorations/read.html?doc=mathalley/GAME-BY-GAME.md',
};

function route() {
  const key = location.hash.replace(/^#/, '') || '/';
  const target = routes[key] || routes['/'];
  stage.src = target;
  for (const a of document.querySelectorAll('[data-route]')) {
    a.classList.toggle('on', a.dataset.route === key);
  }
}
addEventListener('hashchange', route);
route();
```

### 3. The iframe stage

Why an iframe:

- Each exploration keeps its own CSS and scripts.
- The rail does not unmount, so the transition is instant.
- A same-origin page can be inspected and linked directly.

```html
<div class="shell">
  <aside class="rail">…</aside>
  <main>
    <header class="bar">…</header>
    <iframe id="stage" title="Exploration stage"></iframe>
  </main>
</div>
```

Set `border: 0`, `width: 100%`, `height: 100%`, and give the stage a background
so a slow page does not flash white.

### 4. Deep links and the open-in-tab action

The hash is on the portal, not the iframe. The "open in tab" button uses the
route target directly.

```js
openTab.onclick = () => window.open(stage.src, '_blank', 'noopener');
```

If the target contains its own hash, such as `#modes`, keep it. It addresses the
inner page, not the portal.

### 5. The markdown reader

Fetch the document, escape HTML, then render a small subset: headings,
paragraphs, lists, tables, fenced code, links, bold and inline code. Validate the
path: it must end in `.md` and stay inside the repository root.

```js
const doc = new URLSearchParams(location.search).get('doc') || '';
if (!/^[\w./-]+\.md$/.test(doc) || doc.includes('..')) {
  document.body.textContent = 'Invalid document';
} else {
  const text = await (await fetch('/' + doc)).text();
  document.querySelector('#doc').innerHTML = renderMarkdown(escapeHtml(text));
}
```

`escapeHtml` first is not optional. A captured file may contain angle brackets.

### 6. Mobile rail

Below 900 px, the rail becomes a drawer. A button in the top bar toggles a class
on the shell. The stage stays full width. Close the drawer after a route change.

## Step-by-Step Guide

1. Put `serve.mjs` at the repository root.
2. Run `node serve.mjs` and confirm `index.html` loads.
3. Create the portal shell: rail, top bar, stage.
4. Write the route table with every page.
5. Add the active state and the hash listener.
6. Add the markdown reader and one study route.
7. Add the open-in-tab action.
8. Add the mobile drawer.
9. Verify a video route seeks. If it does not, ranges are broken.

## Code Examples

### GOOD: relative paths inside the stage

```html
<a href="dearwind-sampler.html#modes">Mathalley boards</a>
```

### BAD: absolute local paths

```html
<a href="/Users/me/repo/explorations/dearwind-sampler.html">…</a>
```

### GOOD: one route map

```js
const target = routes[location.hash.slice(1)] || routes['/'];
```

### BAD: duplicate route strings in the rail and the router

The rail and the router drift. Keep one map.

## Common Pitfalls

| Problem | Cause | Fix |
| --- | --- | --- |
| Video cannot seek | No byte-range support | Return 206 with `Content-Range` |
| Markdown loads as a download | Wrong MIME type | `text/markdown; charset=utf-8` |
| Page escapes the root | `..` in the URL | Resolve and compare the prefix |
| Hash route ignored | `hashchange` not wired, or the hash is inside the iframe | Listen on the portal window |
| White flash between pages | No stage background | Set a paper background on the iframe |
| Fonts render as boxes | The page is served, but the font path is wrong | Check network requests in devtools |

## References

- Node HTTP: <https://nodejs.org/api/http.html>
- HTTP range requests: <https://developer.mozilla.org/en-US/docs/Web/HTTP/Range_requests>
- Example portal: `index.html` at the repository root
