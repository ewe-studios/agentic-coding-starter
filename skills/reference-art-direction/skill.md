---
name: "Reference Art Direction"
description: "Turn a reference screenshot or video into usable design tokens: extract a palette with ImageMagick, read type and layout, map roles, and document provenance honestly."
approved: No
created: 2026-11-12
license: "MIT"
metadata:
  author: "Main Agent"
  version: "1.0"
  last_updated: "2026-11-12"
  tags: [art-direction, palette, design-tokens, imagemagick, ffmpeg, research]
tools: [Read, Write, Bash, Edit]
---

# Reference Art Direction

## Overview

This skill reads a reference image or video and produces the material a
reimplementation needs: a palette with named roles, a type hierarchy, a layout
sketch, a motion list, and a clear note about what the source actually is.

The output is a small set of design tokens and a written teardown. The tokens go
into CSS variables. The teardown goes into the research page. The provenance note
protects the original designer and keeps the analysis honest.

## When to Use

Use this skill when:

- A user supplies a screenshot, a walkthrough, or a design they like.
- A new visual language must be matched without copying assets.
- A page needs a palette that feels native to a reference.

Do not use it to reproduce copyrighted artwork or to claim access to source code
that is not public.

## Usage Type

**EDUCATIONAL.** Follow the extraction steps, then write the tokens for the new
project.

## Core Concepts

### 1. Get stills from a video

Use ffmpeg. Extract one frame every few seconds for a contact sheet, then grab
full frames at the chapter points.

```bash
ffmpeg -i walkthrough.webm -vf "fps=1/6,scale=480:-1,tile=4x3" -frames:v 1 sheet.jpg
ffmpeg -i walkthrough.webm -vf "fps=1/6,scale=1200:-1" frames/f%02d.jpg
```

The contact sheet shows the screens. The full frames are the evidence for the
teardown.

### 2. Extract the palette

ImageMagick reduces the image to 16 colours and prints a histogram. Sort by area.

```bash
magick hero.png -resize 600x -colors 16 -format %c histogram:info:- | sort -rn
```

Sample specific regions by point or crop when the histogram is dominated by the
background.

```bash
magick hero.png -format '%[pixel:p{760,540}]\n' info:
magick hero.png -crop 300x200+600+500 +repage -resize 200x -colors 8 \
  -format %c histogram:info:-
```

### 3. Name roles, not just hex codes

A palette is useful only when each colour has a job.

| Role | Test |
| --- | --- |
| Ground | The largest low-saturation area |
| Card | The raised surface |
| Ink | The darkest text or outline, never pure black |
| Action | The most saturated colour, used sparingly |
| Secondary | A supporting accent, one per section |
| Soft | A tint for selected or free states |

```json
{
  "paper": "#FBF6E7",
  "sand": "#EADAC8",
  "ink": "#554B37",
  "action": "#C15443",
  "action-strong": "#B42F2B",
  "mint": "#7CBB9C",
  "mustard": "#CFBD6A",
  "blush": "#F9B7A9",
  "sage": "#B4D09F",
  "card": "#FFFDF7"
}
```

### 4. Read the type hierarchy

Record three levels, not every font size.

1. **Feeling:** script, handwritten, or display. Used for a few words.
2. **Product:** serif or high-contrast face. Used for names and prices.
3. **Function:** caps with wide tracking, or a clean sans. Used for labels and
   navigation.

Note the case and tracking. Wide-tracked caps are a cheap way to make a small
label feel considered.

### 5. Sketch the layout in regions

Write the page as boxes with proportions. Do not trace pixels.

```
Home:    [top bar] [hero text 35%] [anchor art 40%] [promo stack 25%]
Menu:    [anchor art 30%] [product sheet 70%]
Events:  [calendar 60%] [event card 30%] [anchor art 10%]
```

This is enough to build a responsive version.

### 6. List motion by trigger

Describe motion as a short list:

- On load: panels fade up, 300 to 500 ms.
- On navigate: the anchor illustration slides, panels cross-fade.
- On select: the product sheet slides in from the right.
- On hold: the illustration turns with the pointer.

Avoid copying timing exactly. Match the feel.

### 7. State what the source is

A concept video is not a codebase. Write one line:

> This is a design concept. There is no public source. The analysis describes
> what the walkthrough shows, not how the site was built. Concept and art by
> <designer>.

Credit the designer with their preferred handle and keep the original file
outside the repository when it is large.

## Step-by-Step Guide

1. Watch the video once without pausing.
2. Cut stills at the chapter points.
3. Extract the 16-colour histogram.
4. Sample regions for roles the histogram hides.
5. Name six to twelve tokens.
6. Read the type hierarchy and the layout regions.
7. List the motion triggers.
8. Write the provenance line.
9. Build the research page from the tokens.
10. Check the page at phone width and in dark conditions.

## Code Examples

### GOOD: role tokens in CSS

```css
:root {
  --paper: #FBF6E7;
  --card: #FFFDF7;
  --ink: #554B37;
  --action: #C15443;
  --mint: #7CBB9C;
}
```

### BAD: one token per sampled colour

```css
:root { --c1:#FBF6E7; --c2:#F5ECD6; --c3:#EADAC8; --c4:#D5CEAF; }
```

### BAD: claiming the original stack

> Built with React and GSAP.

The video does not show that. Describe what is visible.

## Common Pitfalls

| Problem | Cause | Fix |
| --- | --- | --- |
| Palette too beige | Histogram dominated by the ground | Sample crops for accents |
| Colours look muddy in CSS | sRGB values used as linear | Keep CSS values, do not convert |
| Layout does not fit content | Traced pixel positions | Rebuild as proportion regions |
| Wrong credit | Guessed from a watermark | Read the footer or the source post |
| Research page feels like a copy | Reused assets and type | Rebuild the layout, keep only tokens |

## References

- ImageMagick colour docs: <https://imagemagick.org/script/color.php>
- ffmpeg filters: <https://ffmpeg.org/ffmpeg-filters.html>
- Example study: `strawberry-corner/analysis/STRAWBERRY-CORNER.md`
