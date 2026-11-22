# iPhone design guide

These local notes apply Apple’s Human Interface Guidelines (HIG) to the Foundation UI renderer.
Sources checked on 2026-11-21.

## Screen structure

| User intent | iPhone presentation |
| --- | --- |
| Open a screen or return to the previous one | Use the system navigation stack, title, Back button, and back gesture. Place controls inside safe areas. |
| Act on this screen | Put frequent actions in the toolbar. Give an Add symbol a spoken name. Keep system bar materials and scrolling behavior. |
| Filter one list | Use a segmented picker for a few short, mutually exclusive choices. Offer a menu when labels need more room. |
| Read or complete a task | Align titles on a common leading edge. Allow multiline rows. Use a visible checkmark and expose the completion state to VoiceOver. |
| Scan related groups | Use list sections and their native insets. Put supporting counts in the section footer when they describe that section. |
| Add a task | Present a sheet with the field, Cancel, and Add. Use cancellation and confirmation toolbar placements. Keep the field visible above the keyboard. |
| Delete a row | Mark the action as destructive. Pair the swipe action with an accessible action. Ask for confirmation when the loss warrants it. |

The toolbar, segmented picker, inset list, and sheet form one suitable design for the todo example.
Choose roles from the screen’s task when designing another application.
[Designing for iOS](https://developer.apple.com/design/human-interface-guidelines/designing-for-ios) · [Toolbars](https://developer.apple.com/design/human-interface-guidelines/toolbars) · [Segmented controls](https://developer.apple.com/design/human-interface-guidelines/segmented-controls) · [Lists and tables](https://developer.apple.com/design/human-interface-guidelines/lists-and-tables) · [Sheets](https://developer.apple.com/design/human-interface-guidelines/sheets)

## Type and color

Use a system text style for each role. Use body for task names and footnote for supporting counts.
Dynamic Type follows the person’s preferred text size. Scale branded fonts relative to their system text style.
Let rows grow when text wraps. At large sizes, move secondary details below the title.
[Typography](https://developer.apple.com/design/human-interface-guidelines/typography)

Start with system backgrounds and label colors. Use grouped backgrounds for grouped lists.
Reserve accent color for actions that need emphasis. Give brand colors light, dark, and increased-contrast variants.
Check the foreground together with its actual background. Pair color with a symbol or text when it conveys state.
[Color](https://developer.apple.com/design/human-interface-guidelines/color)

## Touch and accessibility

Aim for a 44 by 44 point touch area for iPhone buttons. A small visible glyph can sit inside a larger touch area.
Give adjacent actions distinct targets. Retain native press feedback and respect Reduce Motion for custom transitions.
VoiceOver reads the interface aloud. Give it the action name and current value.
Keep focus in a presented sheet and check where focus returns after dismissal.
[Buttons](https://developer.apple.com/design/human-interface-guidelines/buttons) · [Accessibility](https://developer.apple.com/design/human-interface-guidelines/accessibility)
