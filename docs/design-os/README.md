# Design OS Distllations

As described by the author, [DesignOS](https://github.com/buildermethods/design-os/blob/main/docs/index.md):

It's a product planning and design tool that helps you define your product vision, structure your data model, design your UI, and export production-ready components for implementation. Rather than jumping straight into code, you work through a guided process that captures what you're building and why—then hands off everything your coding agent needs to build it right.

It splits the underlying process of generating the design specification into 4 parts:

- Product Planning
- Design System
- Section Design
- Export - most important step which should produce a hand-off package which the AI can use

#### Tips

- Follow the sequence — Each step builds on the previous. Don't skip ahead.
- Be specific — The more detail you provide, the better the output.
- Iterate — Each command is a conversation. Refine until you're happy.
- Restart the dev server — After creating new components, restart to see changes.

## Product Planning

The first phase, focused on foundation questions that help shape the overal vision, target screens, audience and critical things that are important to understand clarity on the product vision.

## Step 1: Vision

Here we want to get clarity on the following to create the products core identity, by asking about:

- Product Name
- Description - 1-3 sentence capturing the essence of the product
- Problems & solutions — What pain points you're addressing and how
- Key features — The main capabilities that make this possible

Its to create a: product-overview.md describe the product.

## Prompt

You are helping the user define their product vision for Design OS. This is a conversational, multi-step process.

### Step 1: Gather Initial Input

First, ask the user to share their raw notes, ideas, or thoughts about the product they want to build. Be warm and open-ended:

"I'd love to help you define your product vision. Tell me about the product you're building - share any notes, ideas, or rough thoughts you have. What problem are you trying to solve? Who is it for? Don't worry about structure yet, just share what's on your mind."

Wait for their response before proceeding.

### Step 2: Ask Clarifying Questions

After receiving their input, use the AskUserQuestion tool to ask 3-5 targeted questions to help shape:

- **The product name** - A clear, concise name for the product
- **The core product description** (1-3 sentences that capture the essence)
- **The key problems** the product solves (1-5 specific pain points)
- **How the product solves each problem** (concrete solutions)
- **The main features** that make this possible

**Important:** If the user hasn't already provided a product name, ask them:

- "What would you like to call this product? (A short, memorable name)"

Other example clarifying questions (adapt based on their input):

- "Who is the primary user of this product? Can you describe them?"
- "What's the single biggest pain point you're addressing?"
- "How do people currently solve this problem without your product?"
- "What makes your approach different or better?"
- "What are the 3-5 most essential features?"

Ask questions one or two at a time, and engage conversationally.

### Step 3: Present Draft and Refine

Once you have enough information, present a draft summary:

"Based on our discussion, here's what I'm capturing for **[Product Name]**:

**Description:**
[Draft 1-3 sentence description]

**Problems & Solutions:**

1. [Problem] → [Solution]
2. [Problem] → [Solution]

**Key Features:**

- Feature 1
- Feature 2
- Feature 3

Does this capture your vision? Would you like to adjust anything?"

Iterate until the user is satisfied.

### Step 4: Create the File

Once the user approves, create the file at `/product/product-overview.md` with this exact format:

```markdown
# [Product Name]

## Description

[The finalized 1-3 sentence description]

## Problems & Solutions

### Problem 1: [Problem Title]

[How the product solves it in 1-2 sentences]

### Problem 2: [Problem Title]

[How the product solves it in 1-2 sentences]

[Add more as needed, up to 5]

## Key Features

- [Feature 1]
- [Feature 2]
- [Feature 3]
  [Add more as needed]
```

**Important:** The `# [Product Name]` heading at the top is required - this is what displays as the card title in the app.

## Step 5: Confirm Completion

Let the user know:

"I've created your product overview at `/product/product-overview.md`. The homepage will now display **[Product Name]** with your product vision. You can run `/product-roadmap` next to break this down into development sections."

## Important Notes

- Be conversational and helpful, not robotic
- Ask follow-up questions when answers are vague
- Help the user think through their product, don't just transcribe
- Keep the final output concise and clear
- The format must match exactly for the app to parse it correctly
- **Always ensure the product has a name** - if user didn't provide one, ask for it

## Step 2: RoadMap

We want to break the prodcut down into 3 - 5 sections, with each having:

- A navigation item in the apps UI
- A self contained feature areea that can be designed and built independently
- A logical phase of the development roadmap.

This will be built from the generated product-overview.md in Vision, the Ai should propose the sections and ordering to be discussed on, sections should be sequenctially developed within provided priority, building from the core sections down to the connected ones.

## Prompt

You are helping the user create or update their product roadmap for the product. You will either create or update existing roadmpa output file:

1. **Create** an initial roadmap if one doesn't exist
2. **Sync** changes if the user has manually edited the markdown files

### Step 1: Check Current State

First, check if `/product/product-roadmap.md` exists and read `/product/product-overview.md` if it exists.

---

### If No Roadmap Exists (Creating New)

#### Analyze the Product Overview

Read the product overview and analyze:

- The core description
- The problems being solved
- The key features listed

#### Propose Sections

Based on your analysis, propose 3-5 sections that represent:

- **Navigation items** - main areas of the product UI
- **Roadmap phases** - logical order for building
- **Self-contained feature areas** - each can be designed and built independently

Present your proposal:

"Based on your product overview, I'd suggest breaking this into these sections:

1. **[Section Title]** - [One sentence description]
2. **[Section Title]** - [One sentence description]
3. **[Section Title]** - [One sentence description]

These are ordered by importance and logical development sequence. The first section would be the core functionality, with each subsequent section building on it."

Then use the AskUserQuestion tool to ask the user: "Does this breakdown make sense? Would you like to adjust any sections or their order?"

#### Refine with User

Iterate on the sections based on user feedback. Ask clarifying questions:

- "Should [feature X] be its own section or part of [Section Y]?"
- "What would you consider the most critical section to build first?"
- "Are there any major areas I'm missing?"

#### Create the File

Once approved, create `/product/product-roadmap.md` with this exact format:

```markdown
# Product Roadmap

## Sections

### 1. [Section Title]

[One sentence description]

### 2. [Section Title]

[One sentence description]

### 3. [Section Title]

[One sentence description]
```

### Confirm

"I've created your product roadmap at `/product/product-roadmap.md`. The homepage now shows your [N] sections:

1. **[Section 1]** — [Description]
2. **[Section 2]** — [Description]
3. **[Section 3]** — [Description]

**Next step:** Run `/data-model` to define the core entities and relationships in your product. This establishes a shared vocabulary that keeps your sections consistent."

---

### If Roadmap Already Exists (Syncing)

#### Read Current Files

Read both:

- `/product/product-overview.md`
- `/product/product-roadmap.md`

#### Report Current State

"I see you already have a product roadmap defined with [N] sections:

1. [Section 1 Title]
2. [Section 2 Title]
   ...

What would you like to do?

- **Update sections** - Add, remove, or reorder sections
- **Sync from files** - I'll re-read the markdown files and confirm everything is in sync
- **Start fresh** - Regenerate the roadmap based on the current product overview"

#### Handle User Choice

**If updating sections:**
Ask what changes they want to make, then update the file accordingly.

**If syncing:**
Confirm the current state matches what's in the files. If the user has manually edited the `.md` files, let them know the app will pick up those changes on next build/refresh.

**If starting fresh:**
Follow the "Creating New" flow above, but note you're replacing the existing roadmap.

---

### Important Notes

- Sections should be ordered by development priority
- Each section should be self-contained enough to design and build independently
- Section titles become navigation items in the app
- The numbered format (`### 1. Title`) is required for parsing
- Keep descriptions to one sentence - concise and clear
- Don't create too many sections (3-5 is ideal)

## Step 3: Data Models

Despite the naming, we are here to focus and define the core entities using the product which helps us define the user stories for these entities and the actions they do, what views they see and manage.
This are non-technical but conceptual, helping us identify our core users concretely:

- Entity names - User, Project, Invoice, Task, etc.
- Plain Language Description - what does each entity represents
- Relationships - whats the connection between entities

The goal is not database schemas, etc but conceptual description of these avatars and how they interact with the product.

### Prompt

You are helping the user define the core data model for their product. This establishes the "nouns" of the system — the entities and their relationships.

## Step 1: Check Prerequisites

First, verify that the product overview and roadmap exist:

1. Read `/product/product-overview.md` to understand what the product does
2. Read `/product/product-roadmap.md` to understand the planned sections

If either file is missing, let the user know:

"Before defining your data model, you'll need to establish your product vision. Please run `/product-vision` first, then `/product-roadmap` to define your sections."

Stop here if prerequisites are missing.

## Step 2: Gather Initial Input

Review the product overview and roadmap, then present your initial analysis:

"Based on your product vision and roadmap, I can see you're building **[Product Name]** with sections for [list sections].

Let me help you define the core data model — the main "things" your app will work with.

Looking at your product, here are some entities I'm seeing:

- **[Entity 1]** — [Brief description based on product overview]
- **[Entity 2]** — [Brief description based on sections]
- **[Entity 3]** — [Brief description]

Does this capture the main things your app works with? What would you add, remove, or change?"

Wait for their response before proceeding.

## Step 3: Refine Entities

Use the AskUserQuestion tool to clarify:

- "Are there any other core entities in your system that users will create, view, or manage?"
- "For [Entity], what are the most important pieces of information it contains? (Don't need every field, just the key ones)"
- "How do these entities relate to each other?"

Keep the conversation focused on:

- **Entity names** — What are the main nouns?
- **Plain-language descriptions** — What does each entity represent?
- **Relationships** — How do entities connect to each other?

**Important:** Do NOT define every field or database schema details. Keep it minimal and conceptual.

## Step 4: Present Draft and Refine

Once you have enough information, present a draft:

"Here's your data model:

**Entities:**

- **[Entity1]** — [Description]
- **[Entity2]** — [Description]

**Relationships:**

- [Entity1] has many [Entity2]
- [Entity2] belongs to [Entity1]
- [Entity3] links [Entity1] and [Entity4]

Does this look right? Any adjustments?"

Iterate until the user is satisfied.

## Step 5: Create the File

Once approved, create the file at `/product/data-model/data-model.md` with this format:

```markdown
# Data Model

## Entities

### [EntityName]

[Plain-language description of what this entity represents and its purpose in the system.]

### [AnotherEntity]

[Plain-language description.]

[Add more entities as needed]

## Relationships

- [Entity1] has many [Entity2]
- [Entity2] belongs to [Entity1]
- [Entity3] belongs to both [Entity1] and [Entity2]
  [Add more relationships as needed]
```

**Important:** Keep descriptions minimal — focus on what each entity represents, not every field it contains. Leave room for the implementation agent to extend the model.

## Step 6: Confirm Completion

Let the user know:

"I've created your data model at `/product/data-model/data-model.md`.

**Entities defined:**

- [List entities]

**Relationships:**

- [List key relationships]

This provides a shared vocabulary that will be used when generating sample data for your sections. When you run `/sample-data`, it will reference this model to ensure consistency.

Next step: Run `/design-tokens` to choose your color palette and typography."

## Important Notes

- Keep the data model **minimal** — entity names, descriptions, and relationships
- Do NOT define detailed schemas, field types, or validation rules
- Use plain language that a non-technical person could understand
- Relationships should describe how entities connect conceptually
- The implementation agent will extend this with additional fields as needed
- Entity names should be singular (User, Invoice, Project — not Users, Invoices)

## Step 4: Design Tokens

This is where we define the visual identity of the product, such as:

- Colors - Primary (main accent for buttons, links, key actions), Secondary (Complementary acents for tags, highlights), Neutrals -Backgrounds, texts, borders
- Typography - The heading, body, mono (for code and technical contents)

The AI is to generate a colors.json that defines our overall color pallete and typography.json containg our clear defined typoraphy elements.

### Prompt

You are helping the user choose colors and typography for their product. These design tokens will be used consistently across all screen designs and the application shell.

## Step 1: Check Prerequisites

First, verify that the product overview exists:

Read `/product/product-overview.md` to understand what the product is.

If it doesn't exist:

"Before defining your design system, you'll need to establish your product vision. Please run `/product-vision` first."

Stop here if the prerequisite is missing.

## Step 2: Explain the Process

"Let's define the visual identity for **[Product Name]**.

I'll help you choose:

1. **Colors** — A primary accent, secondary accent, and neutral palette
2. **Typography** — Fonts for headings, body text, and code

These will be applied consistently across all your screen designs and the application shell.

Do you have any existing brand colors or fonts in mind, or would you like suggestions?"

Wait for their response.

## Step 3: Choose Colors

Help the user select from Tailwind's built-in color palette. Present options based on their product type:

"For colors, we'll pick from Tailwind's palette so they work seamlessly with your screen designs.

**Primary color** (main accent, buttons, links):
Common choices: `blue`, `indigo`, `violet`, `emerald`, `teal`, `amber`, `rose`, `lime`

**Secondary color** (complementary accent, tags, highlights):
Should complement your primary — often a different hue or a neutral variation

**Neutral color** (backgrounds, text, borders):
Options: `slate` (cool gray), `gray` (pure gray), `zinc` (slightly warm), `neutral`, `stone` (warm gray)

Based on [Product Name], I'd suggest:

- **Primary:** [suggestion] — [why it fits]
- **Secondary:** [suggestion] — [why it complements]
- **Neutral:** [suggestion] — [why it works]

What feels right for your product?"

Use AskUserQuestion to gather their preferences if they're unsure:

- "What vibe are you going for? Professional, playful, modern, minimal?"
- "Any colors you definitely want to avoid?"
- "Light mode, dark mode, or both?"

## Step 4: Choose Typography

Help the user select Google Fonts:

"For typography, we'll use Google Fonts for easy web integration.

**Heading font** (titles, section headers):
Popular choices: `DM Sans`, `Inter`, `Poppins`, `Manrope`, `Space Grotesk`, `Outfit`

**Body font** (paragraphs, UI text):
Often the same as heading, or: `Inter`, `Source Sans 3`, `Nunito Sans`, `Open Sans`

**Mono font** (code, technical content):
Options: `IBM Plex Mono`, `JetBrains Mono`, `Fira Code`, `Source Code Pro`

My suggestions for [Product Name]:

- **Heading:** [suggestion] — [why]
- **Body:** [suggestion] — [why]
- **Mono:** [suggestion] — [why]

What do you prefer?"

## Step 5: Present Final Choices

Once they've made decisions:

"Here's your design system:

**Colors:**

- Primary: `[color]`
- Secondary: `[color]`
- Neutral: `[color]`

**Typography:**

- Heading: [Font Name]
- Body: [Font Name]
- Mono: [Font Name]

Does this look good? Ready to save it?"

## Step 6: Create the Files

Once approved, create two files:

**File 1:** `/product/design-system/colors.json`

```json
{
  "primary": "[color]",
  "secondary": "[color]",
  "neutral": "[color]"
}
```

**File 2:** `/product/design-system/typography.json`

```json
{
  "heading": "[Font Name]",
  "body": "[Font Name]",
  "mono": "[Font Name]"
}
```

## Step 7: Confirm Completion

Let the user know:

"I've saved your design tokens:

- `/product/design-system/colors.json`
- `/product/design-system/typography.json`

**Your palette:**

- Primary: `[color]` — for buttons, links, key actions
- Secondary: `[color]` — for tags, highlights, secondary elements
- Neutral: `[color]` — for backgrounds, text, borders

**Your fonts:**

- [Heading Font] for headings
- [Body Font] for body text
- [Mono Font] for code

These will be used when creating screen designs for your sections.

Next step: Run `/design-shell` to design your application's navigation and layout."

## Reference: Tailwind Color Palette

Available colors (each has shades 50-950):

- **Warm:** `red`, `orange`, `amber`, `yellow`, `lime`
- **Cool:** `green`, `emerald`, `teal`, `cyan`, `sky`, `blue`
- **Purple:** `indigo`, `violet`, `purple`, `fuchsia`, `pink`, `rose`
- **Neutral:** `slate`, `gray`, `zinc`, `neutral`, `stone`

## Reference: Popular Google Font Pairings

- **Modern & Clean:** DM Sans + DM Sans + IBM Plex Mono
- **Professional:** Inter + Inter + JetBrains Mono
- **Friendly:** Nunito Sans + Nunito Sans + Fira Code
- **Bold & Modern:** Space Grotesk + Inter + Source Code Pro
- **Editorial:** Playfair Display + Source Sans 3 + IBM Plex Mono
- **Tech-forward:** JetBrains Mono + Inter + JetBrains Mono

## Important Notes

- Colors should be Tailwind palette names (not hex codes)
- Fonts should be exact Google Fonts names
- Keep suggestions contextual to the product type
- The mono font is optional but recommended for any product with code/technical content
- Design tokens apply to screen designs only — the Design OS app keeps its own aesthetic

## Step 5: Application Shell

Here we focus on getting clarity with possible visual samples or examples of the type of:

- Application Shell - The persistent ui elements like navigation - do we have a hambuger menu or a top menu, minimal header (logo, user menus, ..etc)
- User menus placement and contents
- Responsive behaviours and how it adapts in mobile
- Additional information like navtiations for (Settings, Help, etc)

We have the AI generate these shell, user nativation and menu for quick preview and also provide them as clear input for when we start the implementation.

Creates:

product/shell/spec.md — Shell specification
src/shell/components/AppShell.tsx — Main shell wrapper
src/shell/components/MainNav.tsx — Navigation component
src/shell/components/UserMenu.tsx — User menu component
src/shell/ShellPreview.tsx — Preview wrapper for Design OS

### Prompt

You are helping the user design the application shell — the persistent navigation and layout that wraps all sections. This is a screen design, not implementation code.

## Step 1: Check Prerequisites

First, verify prerequisites exist:

1. Read `/product/product-overview.md` — Product name and description
2. Read `/product/product-roadmap.md` — Sections for navigation
3. Check if `/product/design-system/colors.json` and `/product/design-system/typography.json` exist

If overview or roadmap are missing:

"Before designing the shell, you need to define your product and sections. Please run:

1. `/product-vision` — Define your product
2. `/product-roadmap` — Define your sections"

Stop here if overview or roadmap are missing.

If design tokens are missing, show a warning but continue:

"Note: Design tokens haven't been defined yet. I'll proceed with default styling, but you may want to run `/design-tokens` first for consistent colors and typography."

## Step 2: Analyze Product Structure

Review the roadmap sections and present navigation options:

"I'm designing the shell for **[Product Name]**. Based on your roadmap, you have [N] sections:

1. **[Section 1]** — [Description]
2. **[Section 2]** — [Description]
3. **[Section 3]** — [Description]

Let's decide on the shell layout. Common patterns:

**A. Sidebar Navigation** — Vertical nav on the left, content on the right
Best for: Apps with many sections, dashboard-style tools, admin panels

**B. Top Navigation** — Horizontal nav at top, content below
Best for: Simpler apps, marketing-style products, fewer sections

**C. Minimal Header** — Just logo + user menu, sections accessed differently
Best for: Single-purpose tools, wizard-style flows

Which pattern fits **[Product Name]** best?"

Wait for their response.

## Step 3: Gather Design Details

Use AskUserQuestion to clarify:

- "Where should the user menu (avatar, logout) appear?"
- "Do you want the sidebar collapsible on mobile, or should it become a hamburger menu?"
- "Any additional items in the navigation? (Settings, Help, etc.)"
- "What should the 'home' or default view be when the app loads?"

## Step 4: Present Shell Specification

Once you understand their preferences:

"Here's the shell design for **[Product Name]**:

**Layout Pattern:** [Sidebar/Top Nav/Minimal]

**Navigation Structure:**

- [Nav Item 1] → [Section]
- [Nav Item 2] → [Section]
- [Nav Item 3] → [Section]
- [Additional items like Settings, Help]

**User Menu:**

- Location: [Top right / Bottom of sidebar]
- Contents: Avatar, user name, logout

**Responsive Behavior:**

- Desktop: [How it looks]
- Mobile: [How it adapts]

Does this match what you had in mind?"

Iterate until approved.

## Step 5: Create the Shell Specification

Create `/product/shell/spec.md`:

```markdown
# Application Shell Specification

## Overview

[Description of the shell design and its purpose]

## Navigation Structure

- [Nav Item 1] → [Section 1]
- [Nav Item 2] → [Section 2]
- [Nav Item 3] → [Section 3]
- [Any additional nav items]

## User Menu

[Description of user menu location and contents]

## Layout Pattern

[Description of the layout — sidebar, top nav, etc.]

## Responsive Behavior

- **Desktop:** [Behavior]
- **Tablet:** [Behavior]
- **Mobile:** [Behavior]

## Design Notes

[Any additional design decisions or notes]
```

## Step 6: Create Shell Components

Create the shell components at `src/shell/components/`:

### AppShell.tsx

The main wrapper component that accepts children and provides the layout structure.

```tsx
interface AppShellProps {
  children: React.ReactNode;
  navigationItems: Array<{ label: string; href: string; isActive?: boolean }>;
  user?: { name: string; avatarUrl?: string };
  onNavigate?: (href: string) => void;
  onLogout?: () => void;
}
```

### MainNav.tsx

The navigation component (sidebar or top nav based on the chosen pattern).

### UserMenu.tsx

The user menu with avatar and dropdown.

### index.ts

Export all components.

**Component Requirements:**

- Use props for all data and callbacks (portable)
- Apply design tokens if they exist (colors, fonts)
- Support light and dark mode with `dark:` variants
- Be mobile responsive
- Use Tailwind CSS for styling
- Use lucide-react for icons

## Step 7: Create Shell Preview

Create `src/shell/ShellPreview.tsx` — a preview wrapper for viewing the shell in Design OS:

```tsx
import data from "@/../product/sections/[first-section]/data.json"; // if exists
import { AppShell } from "./components/AppShell";

export default function ShellPreview() {
  const navigationItems = [
    { label: "[Section 1]", href: "/section-1", isActive: true },
    { label: "[Section 2]", href: "/section-2" },
    { label: "[Section 3]", href: "/section-3" },
  ];

  const user = {
    name: "Alex Morgan",
    avatarUrl: undefined,
  };

  return (
    <AppShell
      navigationItems={navigationItems}
      user={user}
      onNavigate={(href) => console.log("Navigate to:", href)}
      onLogout={() => console.log("Logout")}
    >
      <div className="p-8">
        <h1 className="text-2xl font-bold mb-4">Content Area</h1>
        <p className="text-stone-600 dark:text-stone-400">
          Section content will render here.
        </p>
      </div>
    </AppShell>
  );
}
```

## Step 8: Apply Design Tokens

If design tokens exist, apply them to the shell components:

**Colors:**

- Read `/product/design-system/colors.json`
- Use primary color for active nav items, key accents
- Use secondary color for hover states, subtle highlights
- Use neutral color for backgrounds, borders, text

**Typography:**

- Read `/product/design-system/typography.json`
- Apply heading font to nav items and titles
- Apply body font to other text
- Include Google Fonts import in the preview

## Step 9: Confirm Completion

Let the user know:

"I've designed the application shell for **[Product Name]**:

**Created files:**

- `/product/shell/spec.md` — Shell specification
- `src/shell/components/AppShell.tsx` — Main shell wrapper
- `src/shell/components/MainNav.tsx` — Navigation component
- `src/shell/components/UserMenu.tsx` — User menu component
- `src/shell/components/index.ts` — Component exports
- `src/shell/ShellPreview.tsx` — Preview wrapper

**Shell features:**

- [Layout pattern] layout
- Navigation for all [N] sections
- User menu with avatar and logout
- Mobile responsive design
- Light/dark mode support

**Important:** Restart your dev server to see the changes.

When you design section screens with `/design-screen`, they will render inside this shell, showing the full app experience.

Next: Run `/shape-section` to start designing your first section."

## Important Notes

- The shell is a screen design — it demonstrates the navigation and layout design
- Components are props-based and portable to the user's codebase
- The preview wrapper is for Design OS only — not exported
- Apply design tokens when available for consistent styling
- Keep the shell focused on navigation chrome — no authentication UI
- Section screen designs will render inside the shell's content area

## Step 7: Designing Sections - Shaping

Here the goal is to use the AI or any already ready viusal elements or UI design you have to describe each of the sections extracted from the planning phase, here we want to:

Define what does each section do, define the overall priority for each and through conversatio establish the following:

- Overview - 2-3 sentences defining said section
- User flows - the main actions and step by step interactions
- UI requirements - specific layouts, patterns and components required
- Scope boundaries - whats intentionally excluded.

### Prompt

You are helping the user define the specification for a section of their product. This is a conversational process to establish the scope of functionality, user flows, and UI requirements.

## Step 1: Check Prerequisites

First, verify that `/product/product-roadmap.md` exists. If it doesn't:

"I don't see a product roadmap defined yet. Please run `/product-roadmap` first to define your product sections, then come back to shape individual sections."

Stop here if the roadmap doesn't exist.

## Step 2: Identify the Target Section

Read `/product/product-roadmap.md` to get the list of available sections.

If there's only one section, auto-select it. If there are multiple sections, use the AskUserQuestion tool to ask which section the user wants to work on:

"Which section would you like to define the specification for?"

Present the available sections as options.

## Step 3: Gather Initial Input

Once the section is identified, invite the user to share any initial thoughts:

"Let's define the scope and requirements for **[Section Title]**.

Do you have any notes or ideas about what this section should include? Share any thoughts about the features, user flows, or UI patterns you're envisioning. If you're not sure yet, we can start with questions."

Wait for their response. The user may provide raw notes or ask to proceed with questions.

## Step 4: Ask Clarifying Questions

Use the AskUserQuestion tool to ask 4-6 targeted questions to define:

- **Main user actions/tasks** - What can users do in this section?
- **Information to display** - What data and content needs to be shown?
- **Key user flows** - What are the step-by-step interactions?
- **UI patterns** - Any specific interactions, layouts, or components needed?
- **Scope boundaries** - What should be explicitly excluded?

Example questions (adapt based on their input and the section):

- "What are the main actions a user can take in this section?"
- "What information needs to be displayed on the primary view?"
- "Walk me through the main user flow - what happens step by step?"
- "Are there any specific UI patterns you want to use (e.g., tables, cards, modals)?"
- "What's intentionally out of scope for this section?"
- "Are there multiple views needed (e.g., list view and detail view)?"

Ask questions one or two at a time, conversationally. Focus on user experience and interface requirements - no backend or database details.

## Step 5: Ask About Shell Configuration

If a shell design has been created for this project (check if `/src/shell/components/AppShell.tsx` exists), ask the user about shell usage:

"Should this section's screen designs be displayed **inside the app shell** (with navigation header), or should they be **standalone pages** (without the shell)?

Most sections use the app shell, but some pages like public-facing views, landing pages, or embedded widgets should be standalone."

Use AskUserQuestion with options:

- "Inside app shell" - The default for most in-app sections
- "Standalone (no shell)" - For public pages, landing pages, or embeds

If no shell design exists yet, skip this question and default to using the shell.

## Step 6: Present Draft and Refine

Once you have enough information, present a draft specification:

"Based on our discussion, here's the specification for **[Section Title]**:

**Overview:**
[2-3 sentence summary of what this section does]

**User Flows:**

- [Flow 1]
- [Flow 2]
- [Flow 3]

**UI Requirements:**

- [Requirement 1]
- [Requirement 2]
- [Requirement 3]

**Display:** [Inside app shell / Standalone]

Does this capture everything? Would you like to adjust anything?"

Iterate until the user is satisfied. Don't add features that weren't discussed. Don't leave out features that were discussed.

## Step 7: Create the Spec File

Once the user approves, create the file at `product/sections/[section-id]/spec.md` with this exact format:

```markdown
# [Section Title] Specification

## Overview

[The finalized 2-3 sentence description]

## User Flows

- [Flow 1]
- [Flow 2]
- [Flow 3]
  [Add all flows discussed]

## UI Requirements

- [Requirement 1]
- [Requirement 2]
- [Requirement 3]
  [Add all requirements discussed]

## Configuration

- shell: [true/false]
```

**Important:**

- Set `shell: true` if the section should display inside the app shell (this is the default)
- Set `shell: false` if the section should display as a standalone page without the shell

The section-id is the slug version of the section title (lowercase, hyphens instead of spaces).

## Step 8: Confirm and Next Steps

Let the user know:

"I've created the specification at `product/sections/[section-id]/spec.md`.

You can review the spec on the section page. When you're ready, run `/sample-data` to create sample data for this section."

## Important Notes

- Be conversational and helpful, not robotic
- Ask follow-up questions when answers are vague
- Focus on UX and UI - don't discuss backend, database, or API details
- Keep the spec concise - only include what was discussed, no bloat
- The format must match exactly for the app to parse it correctly

## Step 8: Create Sample Data

Here, we focus on creating sample data to represent the different entities, their relationships we have it create 5 -10 realistic entires for these entities and the relationships
as simple JSON that would feed the relevant UI elements and components that makes these entities usable in a frontend context.

We can also have it generate type definitions and prop interfaces that makes it easy to supply these to the related components and their different actiosn e.g onEdit, onDelete, etc.

We use the benefit of AI to creatge realistic:

- Names (instead of Lorum Epsum)
- Dates
- Varied content length an status
- Edge cases - empty arrays, long tests

### Prompt

You are helping the user create realistic sample data for a section of their product. This data will be used to populate screen designs. You will also generate TypeScript types based on the data structure.

## Step 1: Check Prerequisites

First, identify the target section and verify that `spec.md` exists for it.

Read `/product/product-roadmap.md` to get the list of available sections.

If there's only one section, auto-select it. If there are multiple sections, use the AskUserQuestion tool to ask which section the user wants to generate data for.

Then check if `product/sections/[section-id]/spec.md` exists. If it doesn't:

"I don't see a specification for **[Section Title]** yet. Please run `/shape-section` first to define the section's requirements, then come back to generate sample data."

Stop here if the spec doesn't exist.

## Step 2: Check for Global Data Model

Check if `/product/data-model/data-model.md` exists.

**If it exists:**

- Read the file to understand the global entity definitions
- Entity names in your sample data should match the global data model
- Use the descriptions and relationships as a guide

**If it doesn't exist:**
Show a warning but continue:

"Note: A global data model hasn't been defined yet. I'll create entity structures based on the section spec, but for consistency across sections, consider running `/data-model` first."

## Step 3: Analyze the Specification

Read and analyze `product/sections/[section-id]/spec.md` to understand:

- What data entities are implied by the user flows?
- What fields/properties would each entity need?
- What sample values would be realistic and helpful for design?
- What actions can be taken on each entity? (These become callback props)

**If a global data model exists:** Cross-reference the spec with the data model. Use the same entity names and ensure consistency.

## Step 4: Present Data Structure

Present your proposed data structure to the user in human-friendly language. Non-technical users should understand how their data is being organized.

**If using global data model:**

"Based on the specification for **[Section Title]** and your global data model, here's how I'm organizing the data:

**Entities (from your data model):**

- **[Entity1]** — [Description from data model]
- **[Entity2]** — [Description from data model]

**Section-specific data:**

[Any additional data specific to this section's UI needs]

**What You Can Do:**

- View, edit, and delete [entities]
- [Other key actions from the spec]

**Sample Data:**

I'll create [X] realistic [Entity1] records with varied content to make your screen designs feel real.

Does this structure make sense? Any adjustments?"

**If no global data model:**

"Based on the specification for **[Section Title]**, here's how I'm proposing to organize your data:

**Data Models:**

- **[Entity1]** — [One sentence explaining what this represents]
- **[Entity2]** — [One sentence explanation]

**How They Connect:**

[Explain relationships in simple terms]

**What You Can Do:**

- View, edit, and delete [entities]
- [Other key actions from the spec]

**Sample Data:**

I'll create [X] realistic [Entity1] records with varied content to make your screen designs feel real.

Does this structure make sense for your product? Any adjustments?"

Use the AskUserQuestion tool if there are ambiguities about what data is needed.

## Step 5: Generate the Data File

Once the user approves the structure, create `product/sections/[section-id]/data.json` with:

- **A `_meta` section** - Human-readable descriptions of each data model and their relationships (displayed in the UI)
- **Realistic sample data** - Use believable names, dates, descriptions, etc.
- **Varied content** - Mix short and long text, different statuses, etc.
- **Edge cases** - Include at least one empty array, one long description, etc.
- **TypeScript-friendly structure** - Use consistent field names and types

### Required `_meta` Structure

Every data.json MUST include a `_meta` object at the top level with:

1. **`models`** - An object where each key is a model name and value is a plain-language description
2. **`relationships`** - An array of strings explaining how models connect to each other

Example structure:

```json
{
  "_meta": {
    "models": {
      "invoices": "Each invoice represents a bill you send to a client for work completed.",
      "lineItems": "Line items are the individual services or products listed on each invoice."
    },
    "relationships": [
      "Each Invoice contains one or more Line Items (the breakdown of charges)",
      "Invoices track which Client they belong to via the clientName field"
    ]
  },
  "invoices": [
    {
      "id": "inv-001",
      "invoiceNumber": "INV-2024-001",
      "clientName": "Acme Corp",
      "clientEmail": "billing@acme.com",
      "total": 1500.0,
      "status": "sent",
      "dueDate": "2024-02-15",
      "lineItems": [
        { "description": "Web Design", "quantity": 1, "rate": 1500.0 }
      ]
    }
  ]
}
```

The `_meta` descriptions should:

- Use plain, non-technical language
- Explain what each model represents in the context of the user's product
- Describe relationships in terms of "contains", "belongs to", "links to", etc.
- **Match the global data model descriptions if one exists**

The data should directly support the user flows and UI requirements in the spec.

## Step 6: Generate TypeScript Types

After creating data.json, generate `product/sections/[section-id]/types.ts` based on the data structure.

### Type Generation Rules

1. **Infer types from the sample data values:**
   - Strings → `string`
   - Numbers → `number`
   - Booleans → `boolean`
   - Arrays → `TypeName[]`
   - Objects → Create a named interface

2. **Use union types for status/enum fields:**
   - If a field like `status` has known values, use a union: `'draft' | 'sent' | 'paid' | 'overdue'`

   - Base this on the spec and the variety in sample data

3. **Create a Props interface for the main component:**
   - Include the data as a prop (e.g., `invoices: Invoice[]`)
   - Include optional callback props for each action (e.g., `onDelete?: (id: string) => void`)

4. **Use consistent entity names:**
   - If a global data model exists, use the same entity names
   - This ensures consistency across sections

Example types.ts:

```typescript
// =============================================================================
// Data Types
// =============================================================================

export interface LineItem {
  description: string;
  quantity: number;
  rate: number;
}

export interface Invoice {
  id: string;
  invoiceNumber: string;
  clientName: string;
  clientEmail: string;
  total: number;
  status: "draft" | "sent" | "paid" | "overdue";
  dueDate: string;
  lineItems: LineItem[];
}

// =============================================================================
// Component Props
// =============================================================================

export interface InvoiceListProps {
  /** The list of invoices to display */
  invoices: Invoice[];
  /** Called when user wants to view an invoice's details */
  onView?: (id: string) => void;
  /** Called when user wants to edit an invoice */
  onEdit?: (id: string) => void;
  /** Called when user wants to delete an invoice */
  onDelete?: (id: string) => void;
  /** Called when user wants to archive an invoice */
  onArchive?: (id: string) => void;
  /** Called when user wants to create a new invoice */
  onCreate?: () => void;
}
```

### Naming Conventions

- Use PascalCase for interface names: `Invoice`, `LineItem`, `InvoiceListProps`

- Use camelCase for property names: `clientName`, `dueDate`, `lineItems`

- Props interface should be named `[SectionName]Props` (e.g., `InvoiceListProps`)

- Add JSDoc comments for callback props to explain when they're called

- **Match entity names from the global data model if one exists**

## Step 7: Confirm and Next Steps

Let the user know:

"I've created two files for **[Section Title]**:

1. `product/sections/[section-id]/data.json` - Sample data with [X] records

2. `product/sections/[section-id]/types.ts` - TypeScript interfaces for type safety

The types include:

- `[Entity]` - The main data type
- `[SectionName]Props` - Props interface for the component (includes callbacks for [list actions])

When you're ready, run `/design-screen` to create the screen design for this section."

## Important Notes

- Generate realistic, believable sample data - not "Lorem ipsum" or "Test 123"
- Include 5-10 sample records for main entities (enough to show a realistic list)
- Include edge cases: empty arrays, long text, different statuses
- Keep field names clear and TypeScript-friendly (camelCase)
- The data structure should directly map to the spec's user flows
- Always generate types.ts alongside data.json
- Callback props should cover all actions mentioned in the spec
- **Use entity names from the global data model for consistency across sections**

## Steps 9: Design the Screens

Here is where we build actual components (react, ..etc), where we utilize the sample data and specification to create realistic, real life components for
the screens of each section.

Here we actually have AI generate actual real life components that can be directly used in the actual implementation of the full sections, here we get to
visualize in actual working code the different components and their different states with the entities, their states and the relationships.

More so, we you can create preview wrappers that help you get clarity and even use the pywright mcp to screenshot these so your AI gets a visual view of these
components and their state, the requriements we lay out is that. Screenshots are useful for:

- Visual reference during implementation
- Documentation and handoff materials
- Comparing designs across sections

All screen designs include:

- Mobile responsive — Tailwind responsive prefixes (sm:, md:, lg:)
- Light & dark mode — Using dark: variants
- Design tokens applied — Your color palette and typography choices
- All spec requirements — Every user flow and UI requirement implemented
- Multiple Views

If the spec implies multiple views (list view, detail view, form, etc.), you'll be asked which to build first. Run /design-screen again for additional views.

Creates:

- src/sections/[section-id]/components/[ViewName].tsx — Main component
- src/sections/[section-id]/components/[SubComponent].tsx — Sub-components as needed
- src/sections/[section-id]/components/index.ts — Component exports
- src/sections/[section-id]/[ViewName].tsx — Preview wrapper

### Prompt: Screenshoting

You are helping the user capture a screenshot of a screen design they've created. The screenshot will be saved to the product folder for documentation purposes.

## Prerequisites: Check for Playwright MCP

Before proceeding, verify that you have access to the Playwright MCP tool. Look for a tool named `browser_take_screenshot` or `mcp__playwright__browser_take_screenshot`.

If the Playwright MCP tool is not available, output this EXACT message to the user (copy it verbatim, do not modify or "correct" it):

---

To capture screenshots, I need the Playwright MCP server installed. Please run:

```
claude mcp add playwright npx @playwright/mcp@latest
```

## Then restart this Claude Code session and run `/screenshot-design` again.

Do not substitute different package names or modify the command. Output it exactly as written above.

Do not proceed with the rest of this command if Playwright MCP is not available.

## Step 1: Identify the Screen Design

First, determine which screen design to screenshot.

Read `/product/product-roadmap.md` to get the list of available sections, then check `src/sections/` to see what screen designs exist.

If only one screen design exists across all sections, auto-select it.

If multiple screen designs exist, use the AskUserQuestion tool to ask which one to screenshot:

"Which screen design would you like to screenshot?"

Present the available screen designs as options, grouped by section:

- [Section Name] / [ScreenDesignName]
- [Section Name] / [ScreenDesignName]

## Step 2: Start the Dev Server

Start the dev server yourself using Bash. Run `npm run dev` in the background so you can continue with the screenshot capture.

Do NOT ask the user if the server is running or tell them to start it. You must start it yourself.

After starting the server, wait a few seconds for it to be ready before navigating to the screen design URL.

## Step 3: Capture the Screenshot

Use the Playwright MCP tool to navigate to the screen design and capture a screenshot.

The screen design URL pattern is: `http://localhost:3000/sections/[section-id]/screen-designs/[screen-design-name]`

1. First, use `browser_navigate` to go to the screen design URL
2. Wait for the page to fully load
3. **Click the "Hide" link** in the navigation bar to hide it before taking the screenshot. The Hide button has the attribute `data-hide-header` which you can use to locate it.
4. Use `browser_take_screenshot` to capture the page (without the navigation bar)

**Screenshot specifications:**

- Capture at desktop viewport width (1280px recommended)
- Use **full page screenshot** to capture the entire scrollable content (not just the viewport)
- PNG format for best quality

When using `browser_take_screenshot`, set `fullPage: true` to capture the entire page including content below the fold.

## Step 4: Save the Screenshot

The Playwright MCP tool can only save screenshots to its default output directory (`.playwright-mcp/`). You must save the screenshot there first, then copy it to the product folder.

1. **First**, use `browser_take_screenshot` with just a filename (no path):
   - Use a simple filename like `dashboard.png` or `invoice-list.png`
   - The file will be saved to `.playwright-mcp/[filename].png`

2. **Then**, copy the file to the product folder using Bash:
   ```bash
   cp .playwright-mcp/[filename].png product/sections/[section-id]/[filename].png
   ```

**Naming convention:** `[screen-design-name]-[variant].png`

Examples:

- `invoice-list.png` (main view)
- `invoice-list-dark.png` (dark mode variant)
- `invoice-detail.png`
- `invoice-form-empty.png` (empty state)

If the user wants both light and dark mode screenshots, capture both.

## Step 5: Confirm Completion

Let the user know:

"I've saved the screenshot to `product/sections/[section-id]/[filename].png`.

The screenshot captures the **[ScreenDesignName]** screen design for the **[Section Title]** section."

If they want additional screenshots (e.g., dark mode, different states):

"Would you like me to capture any additional screenshots? For example:

- Dark mode version
- Mobile viewport
- Different states (empty, loading, etc.)"

## Important Notes

- Start the dev server yourself - do not ask the user to do it
- Screenshots are saved to `product/sections/[section-id]/` alongside spec.md and data.json
- Use descriptive filenames that indicate the screen design and any variant (dark mode, mobile, etc.)
- Capture at a consistent viewport width for documentation consistency
- Always capture full page screenshots to include all scrollable content
- After you're done, you may kill the dev server if you started it

### Prompt: Design

You are helping the user create a screen design for a section of their product. The screen design will be a props-based React component that can be exported and integrated into any React codebase.

## Step 1: Check Prerequisites

First, identify the target section and verify that `spec.md`, `data.json`, and `types.ts` all exist.

Read `/product/product-roadmap.md` to get the list of available sections.

If there's only one section, auto-select it. If there are multiple sections, use the AskUserQuestion tool to ask which section the user wants to create a screen design for.

Then verify all required files exist:

- `product/sections/[section-id]/spec.md`
- `product/sections/[section-id]/data.json`
- `product/sections/[section-id]/types.ts`

If spec.md doesn't exist:

"I don't see a specification for **[Section Title]** yet. Please run `/shape-section` first to define the section's requirements."

If data.json or types.ts don't exist:

"I don't see sample data for **[Section Title]** yet. Please run `/sample-data` first to create sample data and types for the screen designs."

Stop here if any file is missing.

## Step 2: Check for Design System and Shell

Check for optional enhancements:

**Design Tokens:**

- Check if `/product/design-system/colors.json` exists
- Check if `/product/design-system/typography.json` exists

If design tokens exist, read them and use them for styling. If they don't exist, show a warning:

"Note: Design tokens haven't been defined yet. I'll use default styling, but for consistent branding, consider running `/design-tokens` first."

**Shell:**

- Check if `src/shell/components/AppShell.tsx` exists

If shell exists, the screen design will render inside the shell in Design OS. If not, show a warning:

"Note: An application shell hasn't been designed yet. The screen design will render standalone. Consider running `/design-shell` first to see section screen designs in the full app context."

## Step 3: Analyze Requirements

Read and analyze all three files:

1. **spec.md** - Understand the user flows and UI requirements
2. **data.json** - Understand the data structure and sample content
3. **types.ts** - Understand the TypeScript interfaces and available callbacks

Identify what views are needed based on the spec. Common patterns:

- List/dashboard view (showing multiple items)
- Detail view (showing a single item)
- Form/create view (for adding/editing)

## Step 4: Clarify the Screen Design Scope

If the spec implies multiple views, use the AskUserQuestion tool to confirm which view to build first:

"The specification suggests a few different views for **[Section Title]**:

1. **[View 1]** - [Brief description]
2. **[View 2]** - [Brief description]

Which view should I create first?"

If there's only one obvious view, proceed directly.

## Step 5: Invoke the Frontend Design Skill

Before creating the screen design, read the `frontend-design` skill to ensure high-quality design output.

Read the file at `.claude/skills/frontend-design/SKILL.md` and follow its guidance for creating distinctive, production-grade interfaces.

## Step 6: Create the Props-Based Component

Create the main component file at `src/sections/[section-id]/components/[ViewName].tsx`.

### Component Structure

The component MUST:

- Import types from the types.ts file
- Accept all data via props (never import data.json directly)
- Accept callback props for all actions
- Be fully self-contained and portable

Example:

```tsx
import type { InvoiceListProps } from "@/../product/sections/[section-id]/types";

export function InvoiceList({
  invoices,
  onView,
  onEdit,
  onDelete,
  onCreate,
}: InvoiceListProps) {
  return (
    <div className="max-w-4xl mx-auto">
      {/* Component content here */}

      {/* Example: Using a callback */}
      <button onClick={onCreate}>Create Invoice</button>

      {/* Example: Mapping data with callbacks */}
      {invoices.map((invoice) => (
        <div key={invoice.id}>
          <span>{invoice.clientName}</span>
          <button onClick={() => onView?.(invoice.id)}>View</button>
          <button onClick={() => onEdit?.(invoice.id)}>Edit</button>
          <button onClick={() => onDelete?.(invoice.id)}>Delete</button>
        </div>
      ))}
    </div>
  );
}
```

### Design Requirements

- **Mobile responsive:** Use Tailwind responsive prefixes (`sm:`, `md:`, `lg:`) and ensure the design layout works gracefully on mobile, tablet and desktop screen sizes.
- **Light & dark mode:** Use `dark:` variants for all colors
- **Use design tokens:** If defined, apply the product's color palette and typography
- **Follow the frontend-design skill:** Create distinctive, memorable interfaces

### Applying Design Tokens

**If `/product/design-system/colors.json` exists:**

- Use the primary color for buttons, links, and key accents
- Use the secondary color for tags, highlights, secondary elements
- Use the neutral color for backgrounds, text, and borders
- Example: If primary is `lime`, use `lime-500`, `lime-600`, etc. for primary actions

**If `/product/design-system/typography.json` exists:**

- Note the font choices for reference in comments
- The fonts will be applied at the app level, but use appropriate font weights

**If design tokens don't exist:**

- Fall back to `stone` for neutrals and `lime` for accents (Design OS defaults)

### What to Include

- Implement ALL user flows and UI requirements from the spec
- Use the prop data (not hardcoded values)
- Include realistic UI states (hover, active, etc.)
- Use the callback props for all interactive elements
- Handle optional callbacks with optional chaining: `onClick={() => onDelete?.(id)}`

### What NOT to Include

- No `import data from` statements - data comes via props
- No features not specified in the spec
- No routing logic - callbacks handle navigation intent
- No navigation elements (shell handles navigation)

## Step 7: Create Sub-Components (If Needed)

For complex views, break down into sub-components. Each sub-component should also be props-based.

Create sub-components at `src/sections/[section-id]/components/[SubComponent].tsx`.

Example:

```tsx
import type { Invoice } from "@/../product/sections/[section-id]/types";

interface InvoiceRowProps {
  invoice: Invoice;
  onView?: () => void;
  onEdit?: () => void;
  onDelete?: () => void;
}

export function InvoiceRow({
  invoice,
  onView,
  onEdit,
  onDelete,
}: InvoiceRowProps) {
  return (
    <div className="flex items-center justify-between p-4 border-b">
      <div>
        <p className="font-medium">{invoice.clientName}</p>
        <p className="text-sm text-stone-500">{invoice.invoiceNumber}</p>
      </div>
      <div className="flex gap-2">
        <button onClick={onView}>View</button>
        <button onClick={onEdit}>Edit</button>
        <button onClick={onDelete}>Delete</button>
      </div>
    </div>
  );
}
```

Then import and use in the main component:

```tsx
import { InvoiceRow } from "./InvoiceRow";

export function InvoiceList({
  invoices,
  onView,
  onEdit,
  onDelete,
}: InvoiceListProps) {
  return (
    <div>
      {invoices.map((invoice) => (
        <InvoiceRow
          key={invoice.id}
          invoice={invoice}
          onView={() => onView?.(invoice.id)}
          onEdit={() => onEdit?.(invoice.id)}
          onDelete={() => onDelete?.(invoice.id)}
        />
      ))}
    </div>
  );
}
```

## Step 8: Create the Preview Wrapper

Create a preview wrapper at `src/sections/[section-id]/[ViewName].tsx` (note: this is in the section root, not in components/).

This wrapper is what Design OS renders. It imports the sample data and feeds it to the props-based component.

Example:

```tsx
import data from "@/../product/sections/[section-id]/data.json";
import { InvoiceList } from "./components/InvoiceList";

export default function InvoiceListPreview() {
  return (
    <InvoiceList
      invoices={data.invoices}
      onView={(id) => console.log("View invoice:", id)}
      onEdit={(id) => console.log("Edit invoice:", id)}
      onDelete={(id) => console.log("Delete invoice:", id)}
      onCreate={() => console.log("Create new invoice")}
    />
  );
}
```

The preview wrapper:

- Has a `default` export (required for Design OS routing)
- Imports sample data from data.json
- Passes data to the component via props
- Provides console.log handlers for callbacks (for testing interactions)
- Is NOT exported to the user's codebase - it's only for Design OS
- **Will render inside the shell** if one has been designed

## Step 9: Create Component Index

Create an index file at `src/sections/[section-id]/components/index.ts` to cleanly export all components.

Example:

```tsx
export { InvoiceList } from "./InvoiceList";
export { InvoiceRow } from "./InvoiceRow";
// Add other sub-components as needed
```

## Step 10: Confirm and Next Steps

Let the user know:

"I've created the screen design for **[Section Title]**:

**Exportable components** (props-based, portable):

- `src/sections/[section-id]/components/[ViewName].tsx`
- `src/sections/[section-id]/components/[SubComponent].tsx` (if created)
- `src/sections/[section-id]/components/index.ts`

**Preview wrapper** (for Design OS only):

- `src/sections/[section-id]/[ViewName].tsx`

**Important:** Restart your dev server to see the changes.

[If shell exists]: The screen design will render inside your application shell, showing the full app experience.

[If design tokens exist]: I've applied your color palette ([primary], [secondary], [neutral]) and typography choices.

**Next steps:**

- Run `/screenshot-design` to capture a screenshot of this screen design for documentation
- If the spec calls for additional views, run `/design-screen` again to create them
- When all sections are complete, run `/export-product` to generate the complete export package"

If the spec indicates additional views are needed:

"The specification also calls for [other view(s)]. Run `/design-screen` again to create those, then `/screenshot-design` to capture each one."

## Important Notes

- ALWAYS read the `frontend-design` skill before creating screen designs
- Components MUST be props-based - never import data.json in exportable components
- The preview wrapper is the ONLY file that imports data.json
- Use TypeScript interfaces from types.ts for all props
- Callbacks should be optional (use `?`) and called with optional chaining (`?.`)
- Always remind the user to restart the dev server after creating files
- Sub-components should also be props-based for maximum portability
- Apply design tokens when available for consistent branding
- Screen designs render inside the shell when viewed in Design OS (if shell exists).

## Step 10: Export

With completed design, we will have the AI edxport everything for the implementation agent to build on and with, generally you do this step when:

- Product vision and roadmap are defined
- Atleast one section has screen designs
- You are satisfied with the design direction.

The export process will:

- Checks prerequisites — Verifies required files exist
- Gathers all design assets — Components, types, data, tokens
- Generates implementation instructions — Including ready-to-use prompts
- Generates test instructions — TDD specs for each section
- Creates the export package — A complete product-plan/ directory
- Creates a zip file — product-plan.zip for easy download

It also includes prompt examples that allow you to:

- One shot the full implementation
- Do a section by section implementation (more adviced)

### Prompt

You are helping the user export their complete product design as a handoff package for implementation. This generates all files needed to build the product in a real codebase.

## Step 1: Check Prerequisites

Verify the minimum requirements exist:

**Required:**

- `/product/product-overview.md` — Product overview
- `/product/product-roadmap.md` — Sections defined
- At least one section with screen designs in `src/sections/[section-id]/`

**Recommended (show warning if missing):**

- `/product/data-model/data-model.md` — Global data model
- `/product/design-system/colors.json` — Color tokens
- `/product/design-system/typography.json` — Typography tokens
- `src/shell/components/AppShell.tsx` — Application shell

If required files are missing:

"To export your product, you need at minimum:

- A product overview (`/product-vision`)
- A roadmap with sections (`/product-roadmap`)
- At least one section with screen designs

Please complete these first."

Stop here if required files are missing.

If recommended files are missing, show warnings but continue:

"Note: Some recommended items are missing:

- [ ] Data model — Run `/data-model` for consistent entity definitions
- [ ] Design tokens — Run `/design-tokens` for consistent styling
- [ ] Application shell — Run `/design-shell` for navigation structure

You can proceed without these, but they help ensure a complete handoff."

## Step 2: Gather Export Information

Read all relevant files:

1. `/product/product-overview.md` — Product name, description, features
2. `/product/product-roadmap.md` — List of sections in order
3. `/product/data-model/data-model.md` (if exists)
4. `/product/design-system/colors.json` (if exists)
5. `/product/design-system/typography.json` (if exists)
6. `/product/shell/spec.md` (if exists)
7. For each section: `spec.md`, `data.json`, `types.ts`
8. List screen design components in `src/sections/` and `src/shell/`

## Step 3: Create Export Directory Structure

Create the `product-plan/` directory with this structure:

```
product-plan/
├── README.md                    # Quick start guide
├── product-overview.md          # Product summary (always provide)
│
├── prompts/                     # Ready-to-use prompts for coding agents
│   ├── one-shot-prompt.md       # Prompt for full implementation
│   └── section-prompt.md        # Prompt template for section-by-section
│
├── instructions/                # Implementation instructions
│   ├── one-shot-instructions.md # All milestones combined
│   └── incremental/             # For milestone-by-milestone implementation
│       ├── 01-foundation.md
│       ├── 02-[first-section].md
│       ├── 03-[second-section].md
│       └── ...
│
├── design-system/               # Design tokens
│   ├── tokens.css
│   ├── tailwind-colors.md
│   └── fonts.md
│
├── data-model/                  # Data model
│   ├── README.md
│   ├── types.ts
│   └── sample-data.json
│
├── shell/                       # Shell components
│   ├── README.md
│   ├── components/
│   │   ├── AppShell.tsx
│   │   ├── MainNav.tsx
│   │   ├── UserMenu.tsx
│   │   └── index.ts
│   └── screenshot.png (if exists)
│
└── sections/                    # Section components
    └── [section-id]/
        ├── README.md
        ├── tests.md               # Test-writing instructions for TDD
        ├── components/
        │   ├── [Component].tsx
        │   └── index.ts
        ├── types.ts
        ├── sample-data.json
        └── screenshot.png (if exists)
```

## Step 4: Generate product-overview.md

Create `product-plan/product-overview.md`:

```markdown
# [Product Name] — Product Overview

## Summary

[Product description from product-overview.md]

## Planned Sections

[Ordered list of sections from roadmap with descriptions]

1. **[Section 1]** — [Description]
2. **[Section 2]** — [Description]
   ...

## Data Model

[If data model exists: list entity names]
[If not: "Data model to be defined during implementation"]

## Design System

**Colors:**

- Primary: [color or "Not defined"]
- Secondary: [color or "Not defined"]
- Neutral: [color or "Not defined"]

**Typography:**

- Heading: [font or "Not defined"]
- Body: [font or "Not defined"]
- Mono: [font or "Not defined"]

## Implementation Sequence

Build this product in milestones:

1. **Foundation** — Set up design tokens, data model types, and application shell
2. **[Section 1]** — [Brief description]
3. **[Section 2]** — [Brief description]
   ...

Each milestone has a dedicated instruction document in `product-plan/instructions/`.
```

## Step 5: Generate Milestone Instructions

Each milestone instruction file should begin with the following preamble (adapt the milestone-specific details):

```markdown
---

## About These Instructions

**What you're receiving:**
- Finished UI designs (React components with full styling)
- Data model definitions (TypeScript types and sample data)
- UI/UX specifications (user flows, requirements, screenshots)
- Design system tokens (colors, typography, spacing)
- Test-writing instructions for each section (for TDD approach)

**What you need to build:**
- Backend API endpoints and database schema
- Authentication and authorization
- Data fetching and state management
- Business logic and validation
- Integration of the provided UI components with real data

**Important guidelines:**
- **DO NOT** redesign or restyle the provided components — use them as-is
- **DO** wire up the callback props to your routing and API calls
- **DO** replace sample data with real data from your backend
- **DO** implement proper error handling and loading states
- **DO** implement empty states when no records exist (first-time users, after deletions)
- **DO** use test-driven development — write tests first using `tests.md` instructions
- The components are props-based and ready to integrate — focus on the backend and data layer

---
```

### 01-foundation.md

Place in `product-plan/instructions/incremental/01-foundation.md`:

```markdown
# Milestone 1: Foundation

> **Provide alongside:** `product-overview.md`
> **Prerequisites:** None

[Include the preamble above]

## Goal

Set up the foundational elements: design tokens, data model types, routing structure, and application shell.

## What to Implement

### 1. Design Tokens

[If design tokens exist:]
Configure your styling system with these tokens:

- See `product-plan/design-system/tokens.css` for CSS custom properties
- See `product-plan/design-system/tailwind-colors.md` for Tailwind configuration
- See `product-plan/design-system/fonts.md` for Google Fonts setup

[If not:]
Define your own design tokens based on your brand guidelines.

### 2. Data Model Types

[If data model exists:]
Create TypeScript interfaces for your core entities:

- See `product-plan/data-model/types.ts` for interface definitions
- See `product-plan/data-model/README.md` for entity relationships

[If not:]
Define data types as you implement each section.

### 3. Routing Structure

Create placeholder routes for each section:

[List routes based on roadmap sections]

### 4. Application Shell

[If shell exists:]

Copy the shell components from `product-plan/shell/components/` to your project:

- `AppShell.tsx` — Main layout wrapper
- `MainNav.tsx` — Navigation component
- `UserMenu.tsx` — User menu with avatar

**Wire Up Navigation:**

Connect navigation to your routing:

[List nav items from shell spec]

**User Menu:**

The user menu expects:

- User name
- Avatar URL (optional)
- Logout callback

[If shell doesn't exist:]

Design and implement your own application shell with:

- Navigation for all sections
- User menu
- Responsive layout

## Files to Reference

- `product-plan/design-system/` — Design tokens
- `product-plan/data-model/` — Type definitions
- `product-plan/shell/README.md` — Shell design intent
- `product-plan/shell/components/` — Shell React components
- `product-plan/shell/screenshot.png` — Shell visual reference

## Done When

- [ ] Design tokens are configured
- [ ] Data model types are defined
- [ ] Routes exist for all sections (can be placeholder pages)
- [ ] Shell renders with navigation
- [ ] Navigation links to correct routes
- [ ] User menu shows user info
- [ ] Responsive on mobile
```

### [NN]-[section-id].md (for each section)

Place in `product-plan/instructions/incremental/[NN]-[section-id].md` (starting at 02 for the first section):

```markdown
# Milestone [N]: [Section Title]

> **Provide alongside:** `product-overview.md`
> **Prerequisites:** Milestone 1 (Foundation) complete, plus any prior section milestones

## Goal

Implement the [Section Title] feature — [brief description from roadmap].

## Overview

[One paragraph describing what this section enables users to do. Focus on the user's perspective and the value they get from this feature. Extract from spec.md overview.]

**Key Functionality:**

- [Bullet point 1 — e.g., "View a list of all projects with status indicators"]
- [Bullet point 2 — e.g., "Create new projects with name, description, and due date"]
- [Bullet point 3 — e.g., "Edit existing project details inline"]
- [Bullet point 4 — e.g., "Delete projects with confirmation"]
- [Bullet point 5 — e.g., "Filter projects by status or search by name"]

[List 3-6 key capabilities that the UI components support and need backend wiring]

## Recommended Approach: Test-Driven Development

Before implementing this section, **write tests first** based on the test specifications provided.

See `product-plan/sections/[section-id]/tests.md` for detailed test-writing instructions including:

- Key user flows to test (success and failure paths)
- Specific UI elements, button labels, and interactions to verify
- Expected behaviors and assertions

The test instructions are framework-agnostic — adapt them to your testing setup (Jest, Vitest, Playwright, Cypress, RSpec, Minitest, PHPUnit, etc.).

**TDD Workflow:**

1. Read `tests.md` and write failing tests for the key user flows
2. Implement the feature to make tests pass
3. Refactor while keeping tests green

## What to Implement

### Components

Copy the section components from `product-plan/sections/[section-id]/components/`:

[List components]

### Data Layer

The components expect these data shapes:

[Key types from types.ts]

You'll need to:

- Create API endpoints or data fetching logic
- Connect real data to the components

### Callbacks

Wire up these user actions:

[List callbacks from Props interface with descriptions]

### Empty States

Implement empty state UI for when no records exist yet:

- **No data yet:** Show a helpful message and call-to-action when the primary list/collection is empty
- **No related records:** Handle cases where associated records don't exist (e.g., a project with no tasks)
- **First-time user experience:** Guide users to create their first item with clear CTAs

The provided components include empty state designs — make sure to render them when data is empty rather than showing blank screens.

## Files to Reference

- `product-plan/sections/[section-id]/README.md` — Feature overview and design intent
- `product-plan/sections/[section-id]/tests.md` — Test-writing instructions (use for TDD)
- `product-plan/sections/[section-id]/components/` — React components
- `product-plan/sections/[section-id]/types.ts` — TypeScript interfaces
- `product-plan/sections/[section-id]/sample-data.json` — Test data
- `product-plan/sections/[section-id]/screenshot.png` — Visual reference

## Expected User Flows

When fully implemented, users should be able to complete these flows:

### Flow 1: [Primary Flow Name — e.g., "Create a New Project"]

1. User [starting action — e.g., "clicks 'New Project' button"]
2. User [next step — e.g., "fills in project name and description"]
3. User [next step — e.g., "clicks 'Create' to save"]
4. **Outcome:** [Expected result — e.g., "New project appears in the list, success message shown"]

### Flow 2: [Secondary Flow Name — e.g., "Edit an Existing Project"]

1. User [starting action — e.g., "clicks on a project row"]
2. User [next step — e.g., "modifies the project details"]
3. User [next step — e.g., "clicks 'Save' to confirm changes"]
4. **Outcome:** [Expected result — e.g., "Project updates in place, changes persisted"]

### Flow 3: [Additional Flow — e.g., "Delete a Project"]

1. User [starting action — e.g., "clicks delete icon on a project"]
2. User [next step — e.g., "confirms deletion in the modal"]
3. **Outcome:** [Expected result — e.g., "Project removed from list, empty state shown if last item"]

[Include 2-4 flows covering the main user journeys in this section. Reference the specific UI elements and button labels from the components.]

## Done When

- [ ] Tests written for key user flows (success and failure paths)
- [ ] All tests pass
- [ ] Components render with real data
- [ ] Empty states display properly when no records exist
- [ ] All user actions work
- [ ] User can complete all expected flows end-to-end
- [ ] Matches the visual design
- [ ] Responsive on mobile
```

## Step 6: Generate one-shot-instructions.md

Create `product-plan/instructions/one-shot-instructions.md` by combining all milestone content into a single document. Include the preamble at the very top:

```markdown
# [Product Name] — Complete Implementation Instructions

---

## About These Instructions

**What you're receiving:**

- Finished UI designs (React components with full styling)
- Data model definitions (TypeScript types and sample data)
- UI/UX specifications (user flows, requirements, screenshots)
- Design system tokens (colors, typography, spacing)
- Test-writing instructions for each section (for TDD approach)

**What you need to build:**

- Backend API endpoints and database schema
- Authentication and authorization
- Data fetching and state management
- Business logic and validation
- Integration of the provided UI components with real data

**Important guidelines:**

- **DO NOT** redesign or restyle the provided components — use them as-is
- **DO** wire up the callback props to your routing and API calls
- **DO** replace sample data with real data from your backend
- **DO** implement proper error handling and loading states
- **DO** implement empty states when no records exist (first-time users, after deletions)
- **DO** use test-driven development — write tests first using `tests.md` instructions
- The components are props-based and ready to integrate — focus on the backend and data layer

---

## Test-Driven Development

Each section includes a `tests.md` file with detailed test-writing instructions. These are **framework-agnostic** — adapt them to your testing setup (Jest, Vitest, Playwright, Cypress, RSpec, Minitest, PHPUnit, etc.).

**For each section:**

1. Read `product-plan/sections/[section-id]/tests.md`
2. Write failing tests for key user flows (success and failure paths)
3. Implement the feature to make tests pass
4. Refactor while keeping tests green

The test instructions include:

- Specific UI elements, button labels, and interactions to verify
- Expected success and failure behaviors
- Empty state handling (when no records exist yet)
- Data assertions and state validations

---

[Include product-overview.md content]

---

# Milestone 1: Foundation

[Include 01-foundation.md content WITHOUT the preamble — it's already at the top. This includes design tokens, data model, routing, AND application shell.]

---

# Milestone 2: [First Section Name]

[Include first section handoff content WITHOUT the preamble]

---

# Milestone 3: [Second Section Name]

[Include second section handoff content WITHOUT the preamble]

[Repeat for all sections, incrementing milestone numbers]
```

## Step 7: Copy and Transform Components

### Shell Components

Copy from `src/shell/components/` to `product-plan/shell/components/`:

- Transform import paths from `@/...` to relative paths
- Remove any Design OS-specific imports
- Ensure components are self-contained

### Section Components

For each section, copy from `src/sections/[section-id]/components/` to `product-plan/sections/[section-id]/components/`:

- Transform import paths:
  - `@/../product/sections/[section-id]/types` → `../types`
- Remove Design OS-specific imports
- Keep only the exportable components (not preview wrappers)

### Types Files

Copy `product/sections/[section-id]/types.ts` to `product-plan/sections/[section-id]/types.ts`

### Sample Data

Copy `product/sections/[section-id]/data.json` to `product-plan/sections/[section-id]/sample-data.json`

## Step 8: Generate Section READMEs

For each section, create `product-plan/sections/[section-id]/README.md`:

```markdown
# [Section Title]

## Overview

[From spec.md overview]

## User Flows

[From spec.md user flows]

## Design Decisions

[Notable design choices from the screen design]

## Data Used

**Entities:** [List entities from types.ts]

**From global model:** [Which entities from data model are used]

## Visual Reference

See `screenshot.png` for the target UI design.

## Components Provided

- `[Component]` — [Brief description]
- `[SubComponent]` — [Brief description]

## Callback Props

| Callback   | Description                             |
| ---------- | --------------------------------------- |
| `onView`   | Called when user clicks to view details |
| `onEdit`   | Called when user clicks to edit         |
| `onDelete` | Called when user clicks to delete       |
| `onCreate` | Called when user clicks to create new   |

[Adjust based on actual Props interface]
```

## Step 9: Generate Section Test Instructions

For each section, create `product-plan/sections/[section-id]/tests.md` with detailed test-writing instructions based on the section's spec, user flows, and UI design.

````markdown
# Test Instructions: [Section Title]

These test-writing instructions are **framework-agnostic**. Adapt them to your testing setup (Jest, Vitest, Playwright, Cypress, React Testing Library, RSpec, Minitest, PHPUnit, etc.).

## Overview

[Brief description of what this section does and the key functionality to test]

---

## User Flow Tests

### Flow 1: [Primary User Flow Name]

**Scenario:** [Describe what the user is trying to accomplish]

#### Success Path

**Setup:**

- [Preconditions - what state the app should be in]
- [Sample data to use - reference types from types.ts]

**Steps:**

1. User navigates to [page/route]
2. User sees [specific UI element - be specific about labels, text]
3. User clicks [specific button/link with exact label]
4. User enters [specific data in specific field]
5. User clicks [submit button with exact label]

**Expected Results:**

- [ ] [Specific UI change - e.g., "Success toast appears with message 'Item created'"]
- [ ] [Data assertion - e.g., "New item appears in the list"]
- [ ] [State change - e.g., "Form is cleared and reset to initial state"]
- [ ] [Navigation - e.g., "User is redirected to /items/:id"]

#### Failure Path: [Specific Failure Scenario]

**Setup:**

- [Conditions that will cause failure - e.g., "Server returns 500 error"]

**Steps:**

1. [Same steps as success path, or modified steps]

**Expected Results:**

- [ ] [Error handling - e.g., "Error message appears: 'Unable to save. Please try again.'"]
- [ ] [UI state - e.g., "Form data is preserved, not cleared"]
- [ ] [User can retry - e.g., "Submit button remains enabled"]

#### Failure Path: [Validation Error]

**Setup:**

- [Conditions - e.g., "User submits empty required field"]

**Steps:**

1. User leaves [specific field] empty
2. User clicks [submit button]

**Expected Results:**

- [ ] [Validation message - e.g., "Field shows error: 'Name is required'"]
- [ ] [Form state - e.g., "Form is not submitted"]
- [ ] [Focus - e.g., "Focus moves to first invalid field"]

---

### Flow 2: [Secondary User Flow Name]

[Repeat the same structure for additional flows]

---

## Empty State Tests

Empty states are critical for first-time users and when records are deleted. Test these thoroughly:

### Primary Empty State

**Scenario:** User has no [primary records] yet (first-time or all deleted)

**Setup:**

- [Primary data collection] is empty (`[]`)

**Expected Results:**

- [ ] [Empty state message is visible - e.g., "Shows heading 'No projects yet'"]
- [ ] [Helpful description - e.g., "Shows text 'Create your first project to get started'"]
- [ ] [Primary CTA is visible - e.g., "Shows button 'Create Project'"]
- [ ] [CTA is functional - e.g., "Clicking 'Create Project' opens the create form/modal"]
- [ ] [No blank screen - The UI is helpful, not empty or broken]

### Related Records Empty State

**Scenario:** A [parent record] exists but has no [child records] yet

**Setup:**

- [Parent record] exists with valid data
- [Child records collection] is empty (`[]`)

**Expected Results:**

- [ ] [Parent renders correctly with its data]
- [ ] [Child section shows empty state - e.g., "Shows 'No tasks yet' in the tasks panel"]
- [ ] [CTA to add child record - e.g., "Shows 'Add Task' button"]
- [ ] [No broken layouts or missing sections]

### Filtered/Search Empty State

**Scenario:** User applies filters or search that returns no results

**Setup:**

- Data exists but filter/search matches nothing

**Expected Results:**

- [ ] [Clear message - e.g., "Shows 'No results found'"]
- [ ] [Guidance - e.g., "Shows 'Try adjusting your filters' or similar"]
- [ ] [Reset option - e.g., "Shows 'Clear filters' link"]

---

## Component Interaction Tests

### [Component Name]

**Renders correctly:**

- [ ] [Specific element is visible - e.g., "Displays item title 'Sample Item'"]
- [ ] [Data display - e.g., "Shows formatted date 'Dec 12, 2025'"]

**User interactions:**

- [ ] [Click behavior - e.g., "Clicking 'Edit' button calls onEdit with item id"]
- [ ] [Hover behavior - e.g., "Hovering row shows action buttons"]
- [ ] [Keyboard - e.g., "Pressing Escape closes the modal"]

**Loading and error states:**

- [ ] [Loading - e.g., "Shows skeleton loader while data is fetching"]
- [ ] [Error - e.g., "Shows error message when data fails to load"]

---

## Edge Cases

- [ ] [Edge case 1 - e.g., "Handles very long item names with text truncation"]
- [ ] [Edge case 2 - e.g., "Works correctly with 1 item and 100+ items"]
- [ ] [Edge case 3 - e.g., "Preserves data when navigating away and back"]
- [ ] [Transition from empty to populated - e.g., "After creating first item, list renders correctly"]
- [ ] [Transition from populated to empty - e.g., "After deleting last item, empty state appears"]

---

## Accessibility Checks

- [ ] [All interactive elements are keyboard accessible]
- [ ] [Form fields have associated labels]
- [ ] [Error messages are announced to screen readers]
- [ ] [Focus is managed appropriately after actions]

---

## Sample Test Data

Use the data from `sample-data.json` or create variations:

[Include 2-3 example data objects based on types.ts that tests can use]

```typescript
// Example test data - populated state
const mockItem = {
  id: "test-1",
  name: "Test Item",
  // ... other fields from types.ts
};

const mockItems = [mockItem /* ... more items */];

// Example test data - empty states
const mockEmptyList = [];

const mockItemWithNoChildren = {
  id: "test-1",
  name: "Test Item",
  children: [], // No related records
};

// Example test data - error states
const mockErrorResponse = {
  status: 500,
  message: "Internal server error",
};
```
````

---

## Notes for Test Implementation

- Mock API calls to test both success and failure scenarios
- Test each callback prop is called with correct arguments
- Verify UI updates optimistically where appropriate
- Test that loading states appear during async operations
- Ensure error boundaries catch and display errors gracefully
- **Always test empty states** — Pass empty arrays to verify helpful empty state UI appears (not blank screens)
- Test transitions: empty → first item created, last item deleted → empty state returns

````

### Guidelines for Writing tests.md

When generating tests.md for each section:

1. **Read the spec.md thoroughly** — Extract all user flows and requirements
2. **Study the screen design components** — Note exact button labels, field names, UI text
3. **Review types.ts** — Understand the data shapes for assertions
4. **Include specific UI text** — Tests should verify exact labels, messages, placeholders
5. **Cover success and failure paths** — Every action should have both tested
6. **Always test empty states** — Primary lists with no items, parent records with no children, filtered results with no matches
7. **Be specific about assertions** — "Shows error" is too vague; "Shows red border and message 'Email is required' below the field" is specific
8. **Include edge cases** — Boundary conditions, transitions between empty and populated states
9. **Stay framework-agnostic** — Describe WHAT to test, not HOW to write the test code

## Step 10: Generate Design System Files

### tokens.css

```css
/* Design Tokens for [Product Name] */

:root {
  /* Colors */
  --color-primary: [Tailwind color];
  --color-secondary: [Tailwind color];
  --color-neutral: [Tailwind color];

  /* Typography */
  --font-heading: '[Heading Font]', sans-serif;
  --font-body: '[Body Font]', sans-serif;
  --font-mono: '[Mono Font]', monospace;
}
````

### tailwind-colors.md

```markdown
# Tailwind Color Configuration

## Color Choices

- **Primary:** `[color]` — Used for buttons, links, key accents
- **Secondary:** `[color]` — Used for tags, highlights, secondary elements
- **Neutral:** `[color]` — Used for backgrounds, text, borders

## Usage Examples

Primary button: `bg-[primary]-600 hover:bg-[primary]-700 text-white`
Secondary badge: `bg-[secondary]-100 text-[secondary]-800`
Neutral text: `text-[neutral]-600 dark:text-[neutral]-400`
```

### fonts.md

````markdown
# Typography Configuration

## Google Fonts Import

Add to your HTML `<head>` or CSS:

```html
<link rel="preconnect" href="https://fonts.googleapis.com" />
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
<link
  href="https://fonts.googleapis.com/css2?family=[Heading+Font]&family=[Body+Font]&family=[Mono+Font]&display=swap"
  rel="stylesheet"
/>
```
````

## Font Usage

- **Headings:** [Heading Font]
- **Body text:** [Body Font]
- **Code/technical:** [Mono Font]

````

## Step 11: Generate Prompt Files

Create the `product-plan/prompts/` directory with two ready-to-use prompt files.

### one-shot-prompt.md

Create `product-plan/prompts/one-shot-prompt.md`:

```markdown
# One-Shot Implementation Prompt

I need you to implement a complete web application based on detailed design specifications and UI components I'm providing.

## Instructions

Please carefully read and analyze the following files:

1. **@product-plan/product-overview.md** — Product summary with sections and data model overview
2. **@product-plan/instructions/one-shot-instructions.md** — Complete implementation instructions for all milestones

After reading these, also review:
- **@product-plan/design-system/** — Color and typography tokens
- **@product-plan/data-model/** — Entity types and relationships
- **@product-plan/shell/** — Application shell components
- **@product-plan/sections/** — All section components, types, sample data, and test instructions

## Before You Begin

Please ask me clarifying questions about:

1. **Authentication & Authorization**
   - How should users sign up and log in? (email/password, OAuth providers, magic links?)
   - Are there different user roles with different permissions?
   - Should there be an admin interface?

2. **User & Account Modeling**
   - Is this a single-user app or multi-user?
   - Do users belong to organizations/teams/workspaces?
   - How should user profiles be structured?

3. **Tech Stack Preferences**
   - What backend framework/language should I use?
   - What database do you prefer?
   - Any specific hosting/deployment requirements?

4. **Backend Business Logic**
   - Any server-side logic, validations or processes needed beyond what's shown in the UI?
   - Background processes, notifications, or other processes to trigger?

5. **Any Other Clarifications**
   - Questions about specific features or user flows
   - Edge cases that need clarification
   - Integration requirements

Lastly, be sure to ask me if I have any other notes to add for this implementation.

Once I answer your questions, create a comprehensive implementation plan before coding.

````

### section-prompt.md

Create `product-plan/prompts/section-prompt.md`:

```markdown
# Section Implementation Prompt

## Define Section Variables

- **SECTION_NAME** = [Human-readable name, e.g., "Invoices" or "Project Dashboard"]
- **SECTION_ID** = [Folder name in sections/, e.g., "invoices" or "project-dashboard"]
- **NN** = [Milestone number, e.g., "02" or "03" — sections start at 02 since 01 is Foundation]

---

I need you to implement the **SECTION_NAME** section of my application.

## Instructions

Please carefully read and analyze the following files:

1. **@product-plan/product-overview.md** — Product summary for overall context
2. **@product-plan/instructions/incremental/NN-SECTION_ID.md** — Specific instructions for this section

Also review the section assets:

- **@product-plan/sections/SECTION_ID/README.md** — Feature overview and design intent
- **@product-plan/sections/SECTION_ID/tests.md** — Test-writing instructions (use TDD approach)
- **@product-plan/sections/SECTION_ID/components/** — React components to integrate
- **@product-plan/sections/SECTION_ID/types.ts** — TypeScript interfaces
- **@product-plan/sections/SECTION_ID/sample-data.json** — Test data

## Before You Begin

Please ask me clarifying questions about:

1. **Authentication & Authorization** (if not yet established)
   - How should users authenticate?
   - What permissions are needed for this section?

2. **Data Relationships**
   - How does this section's data relate to other entities?
   - Are there any cross-section dependencies?

3. **Integration Points**
   - How should this section connect to existing features?
   - Any API endpoints already built that this should use?

4. **Backend Business Logic**
   - Any server-side logic, validations or processes needed beyond what's shown in the UI?
   - Background processes, notifications, or other processes to trigger?

5. **Any Other Clarifications**
   - Questions about specific user flows in this section
   - Edge cases that need clarification

## Implementation Approach

Use test-driven development:

1. Read the `tests.md` file and write failing tests first
2. Implement the feature to make tests pass
3. Refactor while keeping tests green

Lastly, be sure to ask me if I have any other notes to add for this implementation.

Once I answer your questions, proceed with implementation.
```

## Step 12: Generate README.md

Create `product-plan/README.md`:

```markdown
# [Product Name] — Design Handoff

This folder contains everything needed to implement [Product Name].

## What's Included

**Ready-to-Use Prompts:**

- `prompts/one-shot-prompt.md` — Prompt template for full implementation
- `prompts/section-prompt.md` — Prompt template for section-by-section implementation

**Instructions:**

- `product-overview.md` — Product summary (provide with every implementation)
- `instructions/one-shot-instructions.md` — All milestones combined for full implementation
- `instructions/incremental/` — Milestone-by-milestone instructions (foundation, then sections)

**Design Assets:**

- `design-system/` — Colors, fonts, design tokens
- `data-model/` — Core entities and TypeScript types
- `shell/` — Application shell components
- `sections/` — All section components, types, sample data, and test instructions

## How to Use This

### Option A: Incremental (Recommended)

Build your app milestone by milestone for better control:

1. Copy the `product-plan/` folder to your codebase
2. Start with Foundation (`instructions/incremental/01-foundation.md`) — includes design tokens, data model, routing, and application shell
3. For each section:
   - Open `prompts/section-prompt.md`
   - Fill in the section variables at the top (SECTION_NAME, SECTION_ID, NN)
   - Copy/paste into your coding agent
   - Answer questions and implement
4. Review and test after each milestone

### Option B: One-Shot

Build the entire app in one session:

1. Copy the `product-plan/` folder to your codebase
2. Open `prompts/one-shot-prompt.md`
3. Add any additional notes to the prompt
4. Copy/paste the prompt into your coding agent
5. Answer the agent's clarifying questions
6. Let the agent plan and implement everything

## Test-Driven Development

Each section includes a `tests.md` file with test-writing instructions. For best results:

1. Read `sections/[section-id]/tests.md` before implementing
2. Write failing tests based on the instructions
3. Implement the feature to make tests pass
4. Refactor while keeping tests green

The test instructions are **framework-agnostic** — they describe WHAT to test, not HOW. Adapt to your testing setup (Jest, Vitest, Playwright, Cypress, RSpec, Minitest, PHPUnit, etc.).

## Tips

- **Use the pre-written prompts** — They include important clarifying questions about auth and data modeling.
- **Add your own notes** — Customize prompts with project-specific context when needed.
- **Build on your designs** — Use completed sections as the starting point for future feature development.
- **Review thoroughly** — Check plans and implementations carefully to catch details and inconsistencies.
- **Fill in the gaps** — Backend business logic may need manual additions. Incremental implementation helps you identify these along the way.

---

_Generated by Design OS_
```

## Step 13: Copy Screenshots

Copy any `.png` files from:

- `product/shell/` → `product-plan/shell/`
- `product/sections/[section-id]/` → `product-plan/sections/[section-id]/`

## Step 14: Create Zip File

After generating all the export files, create a zip archive of the product-plan folder:

```bash
# Remove any existing zip file
rm -f product-plan.zip

# Create the zip file
cd . && zip -r product-plan.zip product-plan/
```

This creates `product-plan.zip` in the project root, which will be available for download on the Export page.

## Step 15: Confirm Completion

Let the user know:

"I've created the complete export package at `product-plan/` and `product-plan.zip`.

**What's Included:**

**Ready-to-Use Prompts:**

- `prompts/one-shot-prompt.md` — Prompt for full implementation
- `prompts/section-prompt.md` — Prompt template for section-by-section

**Instructions:**

- `product-overview.md` — Product summary (always provide with instructions)
- `instructions/one-shot-instructions.md` — All milestones combined
- `instructions/incremental/` — [N] milestone instructions (foundation, then sections)

**Design Assets:**

- `design-system/` — Colors, fonts, tokens
- `data-model/` — Entity types and sample data
- `shell/` — Application shell components
- `sections/` — [N] section component packages with test instructions

**Download:**

Restart your dev server and visit the Export page to download `product-plan.zip`.

**How to Use:**

1. Copy `product-plan/` to your implementation codebase
2. Open `prompts/one-shot-prompt.md` or `prompts/section-prompt.md`
3. Add any additional notes, then copy/paste into your coding agent
4. Answer the agent's clarifying questions about auth, data modeling, etc.
5. Let the agent implement based on the instructions

The components are props-based and portable — they accept data and callbacks, letting your implementation agent handle routing, data fetching, and state management however fits your stack."

## Important Notes

- Always transform import paths when copying components
- Include `product-overview.md` context with every implementation session
- Use the pre-written prompts — they prompt for important clarifying questions
- Screenshots provide visual reference for fidelity checking
- Sample data files are for testing before real APIs are built
- The export is self-contained — no dependencies on Design OS
- Components are portable — they work with any React setup

## Step 11: Implementation

With the handoff generated, we can then use this with the ai to perform implementation as alluded to either incrementally or via one shot attempts.

## Implementation Approaches

### Option A: Incremental Implementation (Recommended)

For larger products or when you want to review progress incrementally.

**How it works:**

Work through the instructions in order:

1. **Foundation** (`instructions/incremental/01-foundation.md`) — Design tokens, data model types, routing
2. **Shell** (`instructions/incremental/02-shell.md`) — Application shell and navigation
3. **Sections** (`instructions/incremental/03-*.md`, `04-*.md`, etc.) — Each feature section, one at a time

For each milestone:

1. Open `product-plan/prompts/section-prompt.md`
2. Fill in the section variables at the top (SECTION_NAME, SECTION_ID, NN)
3. Add any section-specific notes
4. Copy/paste the prompt into your coding agent
5. Answer clarifying questions and let the agent implement
6. Review and test before moving to the next milestone

**The section prompt:**

- References the section's instruction file and assets
- Points to `tests.md` for test-driven development
- Asks about auth, data relationships, and integration points

### Option B: One-Shot Implementation

For simpler products or when you want to build everything in one session.

**How it works:**

1. Open `product-plan/prompts/one-shot-prompt.md`
2. Add any additional notes (tech stack preferences, constraints)
3. Copy/paste the prompt into your coding agent
4. Answer the agent's clarifying questions about auth, user modeling, etc.
5. Let the agent plan and implement everything

The prompt references `product-overview.md` and `instructions/one-shot-instructions.md`, and guides your agent to ask important questions before starting.

**The prompt includes questions about:**

- Authentication & authorization (login methods, user roles)
- User & account modeling (single-user vs multi-user, teams/workspaces)
- Tech stack preferences (backend framework, database)
- Any other clarifications needed

## Test-Driven Development

Each section includes a `tests.md` file with detailed test-writing instructions. We recommend a TDD approach:

1. **Read the test instructions** — Review `sections/[section-id]/tests.md`
2. **Write failing tests** — Based on the user flows and assertions described
3. **Implement the feature** — Make the tests pass
4. **Refactor** — Clean up while keeping tests green

The test instructions include:

- **User flow tests** — Success and failure paths for key interactions
- **Empty state tests** — Verifying behavior when no records exist
- **Component interaction tests** — Specific UI elements and behaviors
- **Edge cases** — Boundary conditions and transitions

Test instructions are **framework-agnostic**—they describe WHAT to test, not HOW. Adapt them to your testing setup (Jest, Vitest, Playwright, Cypress, RSpec, Minitest, PHPUnit, etc.).

## Spec-Driven Development

We also recommend a spec-driven approach:

1. **Review the design** — Understand what's been designed and why
2. **Ask clarifying questions** — Resolve any ambiguities before coding
3. **Write the technical spec** — Define the backend architecture, API contracts, database schema
4. **Write tests first** — Based on the provided test instructions
5. **Implement** — Build to make tests pass
6. **Verify** — Ensure the implementation matches the design

This approach prevents wasted work from misunderstandings and ensures the backend properly supports the frontend designs.

## Clarifying Questions

Before finalizing any implementation plan, encourage your agent to ask questions like:

**Architecture:**

- What backend framework are we using?
- How should authentication work?
- Are there existing patterns in this codebase to follow?

**Data:**

- How should the data model extend what's defined?
- Are there validation rules beyond what the UI shows?
- How should relationships be handled (eager loading, lazy loading)?

**Integration:**

- How should the callbacks be implemented (API calls, local state)?
- What error handling patterns should we use?
- Are there existing UI components to reuse alongside the new ones?

**Scope:**

- Should we implement all features in this milestone or prioritize?
- Are there any features to skip for now?
- What's the testing strategy?

## What Your Agent Needs to Build

The Design OS export provides finished UI designs. Your implementation agent still needs to create:

**Backend:**

- Database schema and migrations
- API endpoints (REST or GraphQL)
- Business logic and validation
- Authentication and authorization

**Data Layer:**

- State management setup
- Data fetching and caching
- Real-time updates (if needed)

**Integration:**

- Routing configuration
- Callback implementations
- Error handling and loading states
- Empty state handling (when no records exist)
- Form validation and submission

**Tests:**

- Unit and integration tests based on `tests.md` instructions
- User flow tests (success and failure paths)
- Empty state verification

**The UI components are complete and production-ready.** Focus implementation effort on the data layer, backend, and tests—don't redesign or restyle the provided components.

## Tips

- **Use the pre-written prompts** — They include important clarifying questions about auth and data modeling
- **Always include product-overview.md** — It gives essential context about the full product
- **Write tests first** — Use the `tests.md` instructions for TDD
- **Review incrementally** — Section-by-section implementation lets you catch issues early
- **Test with sample data first** — Use the provided sample-data.json before building real APIs
- **Handle empty states** — Ensure good UX when no records exist (first-time users)
- **Trust the components** — They're designed and styled already; wire them up, don't rebuild them
