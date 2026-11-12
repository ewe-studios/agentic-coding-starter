---
name: "Cozy 3D Scene Building"
description: "Build seeded three.js dioramas in the Dearwind cozy style: palette roles, primitive recipes, warm lighting, camera framing, and one shared WebGL context for many canvases."
approved: No
created: 2026-11-12
license: "MIT"
metadata:
  author: "Main Agent"
  version: "1.0"
  last_updated: "2026-11-12"
  tags: [threejs, procedural, 3d, cozy, palette, webgl, seed]
tools: [Read, Write, Edit, Bash]
---

# Cozy 3D Scene Building

## Overview

This skill builds small three.js dioramas that look like they belong in Dearwind.
The scene is generated from data, not loaded from a model. A seed picks the parts,
the palette roles colour them, and a fixed lighting recipe keeps every scene warm
and low-contrast. The same pattern produces characters, props, and the fourteen
Mathalley mode boards.

The important idea is the **recipe**. A recipe is a small data record plus a build
function. The record says which parts exist. The function turns parts into
primitives. A seed chooses between recipes, so every reload can produce a new
scene from the same code.

## When to Use

Use this skill when:

- A page needs several small 3D scenes in one visual family.
- Geometry should come from code, not a GLB.
- The scenes are illustrative: characters, props, carnival machines, dioramas.
- The page must stay light: no shadow maps, no post-processing, few draw calls.

Do not use it for realistic scenes, large worlds, or anything that needs physics.

## Usage Type

**EDUCATIONAL.** Study the examples, then rewrite the recipes for the new scene.
Do not import files from this skill into project code.

## Core Concepts

### 1. Palette roles, not colours

Every recipe names roles, not hues. The palette supplies the hue.

| Role | Meaning |
| --- | --- |
| `body` | Main surface: fur, paint, cloth |
| `light` | Muzzle, belly, highlight face |
| `inner` | Ear, nose, cheek blush |
| `accent` | Hat, coat, trim |
| `ink` | Outlines, dark parts, eyes |

Keep saturation low. One saturated accent per scene is enough. Never use pure
black; use the ink colour.

```js
const SPECIES = {
  otter: {
    body: '#a87a52', light: '#f2dcc0', inner: '#7a553a',
    accent: '#4f7fa8', ink: '#3a2b2e',
  },
};
```

### 2. Primitive grammar

The Dearwind look comes from a small set of rounded primitives:

- Head: sphere scaled to `(1, 0.92, 0.95)`.
- Muzzle: sphere scaled flat, pushed forward.
- Ears: cones, spheres, or flattened spheres, mirrored.
- Body: capsule or scaled sphere.
- Limbs: capsules with small radius.
- Props: rounded boxes and cylinders.

Build a `Group` per character or prop. Parent the parts. Then scale the whole
group once. Keep geometry creation in one place so parts can be reused.

```js
function capsuleProp({ r = 0.4, h = 0.85, body, light }) {
  const g = new THREE.Group();
  const lower = new THREE.Mesh(
    new THREE.SphereGeometry(r, 20, 14),
    new THREE.MeshLambertMaterial({ color: body })
  );
  lower.position.y = r;
  const band = new THREE.Mesh(
    new THREE.CylinderGeometry(r * 1.001, r * 1.001, r * 0.18, 20),
    new THREE.MeshLambertMaterial({ color: light })
  );
  band.position.y = r * 1.08;
  const cap = new THREE.Mesh(
    new THREE.SphereGeometry(r * 1.002, 20, 10, 0, Math.PI * 2, 0, Math.PI / 2),
    new THREE.MeshLambertMaterial({ color: shade(body, -18) })
  );
  cap.position.y = r * 1.08;
  g.add(lower, band, cap);
  return g;
}
```

### 3. Warm, three-light setup

```js
scene.add(new THREE.HemisphereLight(0xfff4e2, 0x9aa7b0, 1.15));
const key = new THREE.DirectionalLight(0xffffff, 1.5);
key.position.set(5, 8, 6);
scene.add(key);
const fill = new THREE.DirectionalLight(0xffd9b0, 0.5);
fill.position.set(-5, 3, 4);
scene.add(fill);
```

- Hemisphere light does most of the work.
- The key light shapes the top faces.
- The fill is warm and weak.
- Do not enable shadow maps. Use a flat contact-shadow disc or ellipse instead.
- `MeshLambertMaterial` is enough. Use `MeshToonMaterial` only when the banding
  is wanted.

### 4. Camera framing

Board panels are wide. A 34° vertical field of view, a camera 8 to 10 units from
the subject, and a target around y = 1.7 frames a cabinet, a train, or a cast of
characters with headroom.

```js
const cam = new THREE.PerspectiveCamera(34, w / h, 0.1, 100);
cam.position.set(1.9, 3.6, 8.5);
cam.lookAt(0, 1.75, 0);
```

Check the extremes. The top rail and the front row must both fit. If a part clips,
move the camera back before you change the field of view.

### 5. Labels as sprites

Answer chips in 3D need to face the camera. Draw the chip on a 2D canvas, use it
as a `CanvasTexture`, and put it on a `Sprite`.

```js
const cv = document.createElement('canvas');
cv.width = 256; cv.height = 160;
const g = cv.getContext('2d');
// draw a rounded chip and the number here
const tex = new THREE.CanvasTexture(cv);
tex.colorSpace = THREE.SRGBColorSpace;
const sprite = new THREE.Sprite(new THREE.SpriteMaterial({
  map: tex, transparent: true, depthTest: false,
}));
sprite.scale.set(0.95, 0.6, 1);
```

`depthTest: false` keeps the label visible through a claw or a cable. Accept the
trade-off: the label can appear in front of parts it should sit behind.

### 6. One WebGL context for many boards

Browsers cap live WebGL contexts. Fourteen boards plus other stages will exceed
the cap, and old contexts are dropped. Use one scratch renderer and copy each
frame into a per-board 2D canvas.

```js
const gl = new THREE.WebGLRenderer({ antialias: true, alpha: true });
gl.setPixelRatio(Math.min(devicePixelRatio, 2));

function paint(board) {
  gl.setSize(board.w, board.h, false);
  gl.render(board.scene, board.camera);
  board.ctx2d.drawImage(gl.domElement, 0, 0, board.w, board.h);
}
```

The board's left `<canvas>` is a 2D canvas. `drawImage` runs in the same frame,
so no `preserveDrawingBuffer` is needed.

Render only visible boards. An `IntersectionObserver` toggles a `visible` flag;
the frame loop skips the rest.

### 7. Seeds and determinism

Use a small seeded generator, never `Math.random`, when a screenshot must be
repeatable.

```js
function mulberry32(a) {
  return function () {
    a |= 0; a = a + 0x6D2B79F5 | 0;
    let t = Math.imul(a ^ a >>> 15, 1 | a);
    t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t;
    return ((t ^ t >>> 14) >>> 0) / 4294967296;
  };
}
```

## Step-by-Step Guide

1. Write the recipe record. Name the roles and the parts.
2. Define the palette. Check the roles against the Dearwind tokens.
3. Build one part at a time. Parent parts to a group.
4. Add the three lights and a contact shadow.
5. Set the camera from the panel aspect. Check the top and bottom margins.
6. Add labels as sprites.
7. Render once and screenshot. Fix framing before adding detail.
8. Add gentle idle motion only after the still frame works.
9. Wire the scene to the shared renderer if the page has more than four canvases.

## Code Examples

### GOOD: a recipe shared by both renderers

```js
const prize = { kind: 'capsule', body: '#f2b8a5', light: '#fff8ea' };
// 3D reads prize.body and prize.light.
// Canvas 2D reads the same record and draws a rounded shape.
```

### BAD: hard-coded colours in the build function

```js
new THREE.MeshLambertMaterial({ color: '#ff0000' }); // breaks the family
```

### BAD: a new renderer per board

```js
for (const board of boards) new THREE.WebGLRenderer(); // context loss
```

## Common Pitfalls

| Problem | Cause | Fix |
| --- | --- | --- |
| Top of the scene is cut | Camera too low or too close | Raise the camera, move it back, then check the target |
| Labels sit behind parts | Default depth test | `depthTest: false` on the sprite material |
| Scene looks dirty | Too many hues and rim lights | One accent, one fill, low saturation |
| Context lost mid-page | More than ~16 WebGL contexts | One scratch renderer, copy to 2D canvases |
| Screenshot differs each run | `Math.random` layout | Seeded generator |
| Glass reads as a wall | Opaque material or wrong order | Low opacity, `transparent: true`, no depth write |

## References

- three.js r160 documentation: <https://threejs.org/docs/>
- Dearwind rendering notes: `src.wavedash/dearwind/markdown/05-rendering-and-materials.md`
- Sampler implementation: `explorations/dearwind-sampler.html`
