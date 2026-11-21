---
name: android-material-design
description: Design and review native Android screens, Material 3 component choices, Compose themes, accessibility and Ewe renderer style mappings.
---

# Android Material Design

Use this local guide for Android user interface (UI) work. Material 3 is Google's design system. Jetpack Compose supplies the Android controls. Sources checked on 2026-11-21.

## Choose the screen structure

Start with the main user task. Assign a visual hierarchy before choosing colors. These are project choices for a task list.

| Intent | Android presentation | Check |
|---|---|---|
| Name the screen | Use a top app bar. Put the list below it. | The title and first row remain distinct at large font sizes. |
| Add a task | Give the main action one prominent control. Place Add in the top app bar for the Ewe todo screen. | The final row scrolls clear of the button. |
| Filter the list | Use a single-choice segmented row for All, Open and Done. | Selection has a visible indicator and an announced state. |
| Complete a task | Use a list item with a checkbox and a wrapping title. | A tap on the completion target sends one action. |
| Delete a task | Give a separate delete control a label that identifies the task. | Its touch area stays separate from completion. |
| Edit or confirm | Use a modal bottom sheet for entry. Use an alert dialog for a decision. | Back and Cancel dismiss through the same action. |

A segmented row groups related choices and gives the end segments their correct shape. [Google's segmented button guidance](https://developer.android.com/develop/ui/compose/components/segmented-button)

Use the available window width. Let the list scroll independently of its top bar. Keep controls clear of system bars, cutouts and the keyboard.
An inset is the space that a system component occupies. Apply each inset once. [Google's window inset guidance](https://developer.android.com/develop/ui/compose/system/insets)

## Resolve the theme

A style token names a visual role. A host maps that role to a value for the current theme.

1. Read the active `MaterialTheme` inside the renderer. Allow the application to supply its color scheme, typography and shapes.
2. Use each Material component's defaults when the author omits a style. They provide pressed, focused, selected and disabled states.
3. Resolve an explicit Ewe style token at the affected node. Preserve the paired content color when changing a container.
4. Use a matching pair such as `primaryContainer` and `onPrimaryContainer`. Check contrast again after a brand override.
5. Give text a role such as `titleLarge` or `bodyLarge`. Allow wrapping and system font scaling.
6. Offer wallpaper colors when the host chooses dynamic color on Android 12 or later. Supply light and dark schemes for other devices.

Material components consume these roles through the surrounding theme. [Google's Material 3 theming guide](https://developer.android.com/develop/ui/compose/designsystems/material3)

## Apply this in Ewe

Start in `mobile/android/ui/src/main/kotlin/ewe/ui/compose/`. Inspect `EweScreen.kt` for node mappings and `EweHost.kt` for both public hosts.

- Render the same node tree through the server host and the on-device host. Keep task logic in Rust.
- Choose controls from node kind, presentation and properties. Handling a tap must preserve the action's node identifier, payload and dispatch count.
- Inspect `PropId.COLOR`, `BACKGROUND` and `FONT` when adding a style. Preserve the distinction between a missing property and an explicit token.
- Check the path from each public host to `EweScreen`. A resolver accepted only by the inner screen cannot serve users of a public host.
- Apply package-wide control changes in the renderer. Let example applications select their theme at the host.

## Verify on Android

- Allow each interactive target at least 48 by 48 density-independent pixels (dp). A smaller visible icon can occupy this space. [Google's accessibility defaults](https://developer.android.com/develop/ui/compose/accessibility/api-defaults)
- Use TalkBack, Android's screen reader. Check each action label, focus order, selected state and disabled state. Hide decorative icons from speech.
- Capture the displayed screen in light and dark mode. Repeat with large text, a long task title and the keyboard open.
- Inspect the pixels. Check for clipped text, hidden actions, overlapping targets and duplicate insets.
- Apply a distinct application theme. Compare the same list, selection and modal in both hosts.
- Complete, filter and delete one task. Verify each gesture triggers its original Rust action once.

## Glossary

| Term | Meaning |
|---|---|
| dp | Density-independent pixel. A layout unit that accounts for screen density. |
| UI | User interface. The controls and content that a person uses. |
