---
name: "Cozy Canvas 2D Illustration"
description: "Redraw three.js cozy scenes as flat Canvas 2D illustrations: shared palette records, path primitives, contact shadows, fixed design space, crisp DPR, and hit maps shared with the 3D panel."
approved: No
created: 2026-11-12
license: "MIT"
metadata:
  author: "Main Agent"
  version: "1.0"
  last_updated: "2026-11-12"
  tags: [canvas2d, illustration, 2d, palette, hit-testing, responsive]
tools: [Read, Write, Edit, Bash]
---

# Cozy Canvas 2D Illustration

## Overview

This skill draws the same scene as a three.js diorama with Canvas 2D paths. The
goal is a sibling, not a copy: the flat drawing keeps the silhouette, the palette
roles, the answer chips and the composition, and drops the lighting and depth.

Canvas 2D is a good second renderer because it has no context limit, no shader
compile, and no GPU requirement. It runs on the same data as the 3D scene, so the
two panels can show one shared question.

## When to Use

Use this skill when:

- A 3D scene needs a flat counterpart for comparison or as a fallback.
- A page needs many small illustrations without many WebGL contexts.
- The style is paper, ink and flat fills: characters, machines, props, diagrams.

Do not use it for photorealistic output or heavy per-pixel effects.

## Usage Type

**EDUCATIONAL.** Read the recipes, then rebuild the shapes for the new subject.

## Core Concepts

### 1. Draw in a fixed design space

Pick a design size for each panel, for example 1200 × 704. Draw everything in
that space. Scale once at the start. The same code then renders a 600 px panel and
a 2400 px retina panel without changing coordinates.

```js
function drawScene(ctx, cssW, cssH) {
  const S = 1200;                       // design width
  const scale = cssW / S;
  ctx.save();
  ctx.scale(scale, scale);
  // draw at S × (S * cssH / cssW)
  ctx.restore();
}
```

Set the backing store to `cssW * dpr`. Cap the device pixel ratio at 2.

```js
const dpr = Math.min(devicePixelRatio || 1, 2);
canvas.width = cssW * dpr;
canvas.height = cssH * dpr;
canvas.style.width = cssW + 'px';
canvas.style.height = cssH + 'px';
const ctx = canvas.getContext('2d');
ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
```

### 2. Four helpers cover most drawings

- Rounded rectangle: cabinets, cards, chips, parcels.
- Ellipse: heads, pools, contact shadows.
- Polygon: ears, awnings, arrows, sails.
- Quadratic curve: cables, handles, waves, hooks.

```js
function rr(g, x, y, w, h, r, fill, stroke, lw) {
  g.beginPath();
  g.moveTo(x + r, y);
  g.arcTo(x + w, y, x + w, y + h, r);
  g.arcTo(x + w, y + h, x, y + h, r);
  g.arcTo(x, y + h, x, y, r);
  g.arcTo(x, y, x + w, y, r);
  g.closePath();
  if (fill) { g.fillStyle = fill; g.fill(); }
  if (stroke) { g.strokeStyle = stroke; g.lineWidth = lw || 2; g.stroke(); }
}

function ell(g, x, y, rx, ry, fill, stroke, lw) {
  g.beginPath();
  g.ellipse(x, y, rx, ry, 0, 0, Math.PI * 2);
  if (fill) { g.fillStyle = fill; g.fill(); }
  if (stroke) { g.strokeStyle = stroke; g.lineWidth = lw || 2; g.stroke(); }
}
```

### 3. Palette roles match the 3D records

Read the same record the 3D builder reads. Use `body` for the main shape, `light`
for bands and muzzles, `accent` for hats and trim, `ink` for outlines.

```js
const PRIZE = { body: '#f2b8a5', light: '#fff8ea', ink: '#3b3345' };
```

Outline colour is ink at 16 to 25% alpha. Solid black outlines read as a cartoon
from another family.

### 4. Contact shadows, not gradients

A soft ellipse under the object sells weight. Paint it with a radial gradient
that fades to transparent. Do not add drop shadows to every part.

```js
function contact(g, x, y, rx, ry) {
  const grd = g.createRadialGradient(x, y, 0, x, y, rx);
  grd.addColorStop(0, 'rgba(59,51,69,.20)');
  grd.addColorStop(1, 'rgba(59,51,69,0)');
  g.save();
  g.translate(x, y); g.scale(1, ry / rx); g.translate(-x, -y);
  g.fillStyle = grd;
  g.beginPath(); g.arc(x, y, rx, 0, Math.PI * 2); g.fill();
  g.restore();
}
```

### 5. Labels use the same chip helper

Draw answer chips with `rr` plus centred text. Keep one chip function so the 3D
sprite and the 2D panel show the same shape. The chip is the most important
element on the board; it must never sit under a claw or a cable.

```js
function chip(g, x, y, w, h, label, fill = '#fffdf7') {
  rr(g, x - w / 2, y - h / 2, w, h, h / 2, fill, 'rgba(59,51,69,.22)', 2);
  g.fillStyle = '#3b3345';
  g.font = '800 ' + Math.min(h * 0.55, 22) + 'px "Baloo 2", sans-serif';
  g.textAlign = 'center';
  g.textBaseline = 'middle';
  g.fillText(String(label), x, y + 1);
}
```

### 6. Translate the 3D recipe

| three.js | Canvas 2D |
| --- | --- |
| Scaled sphere | Ellipse |
| Rounded box | Rounded rect |
| Cone | Triangle or arc |
| Cylinder | Rounded rect with an ellipse cap |
| Directional light | One lighter top face |
| Ambient occlusion | Contact shadow ellipse |
| Sprite label | Chip drawn last |
| Camera | Cropped composition, no perspective |

Decide the crop first. The 2D panel should show the same subject at the same
scale, not an isometric projection of the 3D camera.

### 7. Shared hit map

Gameplay lives in one state object. Both panels declare the same normalised
answer positions. A pointer handler maps the click to a board coordinate, then to
the nearest hotspot.

```js
const hotspots = [
  { id: 0, x: 0.30, y: 0.70 },
  { id: 1, x: 0.50, y: 0.70 },
  { id: 2, x: 0.70, y: 0.70 },
];
```

## Step-by-Step Guide

1. Screenshot the 3D frame.
2. Choose the 2D crop. Write down the design size.
3. Block in the largest shapes first: cabinet, floor, machine body.
4. Add the mid shapes: prizes, carriages, pipes, cards.
5. Add labels and chips.
6. Add contact shadows and two or three paper details.
7. Test at phone width and at 2× DPR.
8. Check that every chip is legible and unobstructed.

## Code Examples

### GOOD: one palette record, two renderers

```js
const PALETTE = { body: '#c8604a', light: '#fff8ea' };
build3D(PALETTE);     // material colours
draw2D(ctx, PALETTE); // fill and band
```

### BAD: a traced screenshot

```js
ctx.drawImage(renderer.domElement, 0, 0); // a copy, not a sibling
```

### BAD: fixed pixel coordinates in CSS space

```js
ctx.fillRect(600, 350, 80, 80); // breaks when the panel resizes
```

## Common Pitfalls

| Problem | Cause | Fix |
| --- | --- | --- |
| Blurry output | Canvas width equals CSS width | Scale the backing store by DPR |
| Text changes size | Font size in device pixels | Size text in design units |
| Shapes move on resize | Coordinates from the CSS box | Draw in a fixed design space |
| Labels collide with parts | No layout check | Move chips or parts before adding detail |
| Flat and lifeless | No contact shadows | Add one soft ellipse per object |
| Inconsistent style | Colours mixed inline | Read one palette record |

## References

- MDN Canvas API: <https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API>
- Dearwind palette: `src.wavedash/analysis/game-data.json`
- Sampler implementation: `explorations/dearwind-sampler.html`
