---
name: ios-human-interface-design
description: Design and review Foundation UI screens for iPhone. Use for SwiftUI controls, presentation roles, style tokens, accessibility, and native example screenshots.
---

# Design an iPhone screen

Start with the interaction. Identify the screen title, primary action, related choices, and each row’s purpose.
Use the [local iPhone design guide](references/iphone.md) to choose the native control and its system appearance.

## Apply Foundation UI styles

A presentation role gives a node a purpose, such as `toolbar-action`.
A style token is a number that the host maps to a color or font.
The Rust view supplies roles, content, and action bindings. The Swift package owns native controls and their event bridge.

- Inspect `mobile/ios/Sources/EweUI/Presentation.swift` for specialized roles. Inspect `Screen.swift` for generic nodes and style modifiers.
- Follow each role through rendering. A toolbar, list section, or sheet can consume a child directly.
- Apply an explicit style to the component part it describes. Preserve native pressed, disabled, selected, and destructive states.
- Use the host’s adaptive color or scalable font for a recognized token. Preserve the native role’s default for absent overrides.
- Follow the public entry point used by each example. Check theme delivery through both `LocalScreen` and `ServerScreen`.
- Keep the examples responsible for task data and actions. Put reusable presentation and token lookup in the package.

For example, a task title can use a host color and body font. The adjacent Delete button keeps its destructive role.
Use the same action binding and node identity when a host changes the theme.

## Review the result

1. Run the screen on an iPhone simulator. Inspect a screenshot with a long task title and a completed item.
2. Switch to dark appearance and an accessibility text size. Check wrapping, sheet scrolling, and keyboard clearance.
3. Apply a host theme. Compare the same screen with the default theme. Inspect button and text contrast in both appearances.
4. Inspect VoiceOver labels, values, and action order. Complete a task and access Delete without swiping the row.
5. Run the task flow through both the server and on-device entry points after a shared renderer change.
