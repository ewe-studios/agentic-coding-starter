# Agentic Coding Framework

> **A structured approach to AI-assisted software development that ensures consistency, quality, and predictability.**

This directory contains the complete framework for guiding AI coding agents (like Claude Code, Cursor, Windsurf, etc.) through your software development process. It defines HOW agents work, HOW code should be written, and WHAT should be built.

## 🎯 What is This?

The Agentic Coding Framework is a set of guidelines, rules, and standards that:

- **Standardizes** how AI agents interact with your codebase
- **Enforces** coding standards and best practices automatically
- **Orchestrates** complex workflows between specialized agents
- **Verifies** that all changes meet quality standards before commits
- **Tracks** features, tasks, and requirements systematically
- **Learns** from mistakes through documented learning logs

Think of it as a "Constitution" for AI agents working on your project - a comprehensive rulebook that ensures every agent follows the same processes, standards, and workflows.

## 🏗️ Framework Structure

```
.agents/
├── README.md              # This file - framework overview
├── AGENTS.md              # Entry point - agents MUST read this first
│
├── rules/                 # HOW agents must work
│   ├── 01-rule-naming-and-structure.md
│   ├── 02-rules-directory-policy.md
│   ├── 03-dangerous-operations-safety.md
│   ├── 04-work-commit-and-push-rules.md
│   ├── 05-coding-practice-agent-orchestration.md
│   ├── 06-specifications-and-requirements.md
│   ├── 07-language-conventions-and-standards.md
│   ├── 08-verification-workflow-complete-guide.md
│   └── 09-skills-identification-and-creation.md
│
├── stacks/                # HOW to write code (language-specific)
│   ├── javascript.md      # JavaScript/TypeScript standards
│   ├── python.md          # Python standards
│   ├── rust.md            # Rust standards
│   └── ...
│
├── skills/                # Documented expertise for specific tasks
│   ├── README.md
│   └── [skill-directories]/
│
└── specifications/        # WHAT to build (feature requirements)
    ├── Spec.md            # Master index
    └── NN-spec-name/      # Individual specifications
        ├── requirements.md
        ├── tasks.md
        └── verification.md (transient)
```

### Directory Purposes

| Directory | Purpose | Agent Usage |
|-----------|---------|-------------|
| **`rules/`** | Workflow, orchestration, verification processes | Read ALL files at session start |
| **`stacks/`** | Language-specific coding standards | Read ONLY relevant language files |
| **`skills/`** | Domain expertise for complex tasks | Read when needed, create when gaps found |
| **`specifications/`** | Feature requirements and task tracking | Read when working on features |

## 🚀 Getting Started

### For New Projects

You can bootstrap a new project with this framework in minutes:

1. **Clone the starter repository as your `.agents` directory:**
   ```bash
   cd your-project-root
   git clone https://github.com/ewe-studios/agentic-coding-starter .agents
   ```

2. **Remove the `.agents/.git` directory to make it part of your repository:**
   ```bash
   rm -rf .agents/.git
   ```

3. **Add it to your repository:**
   ```bash
   git add .agents
   git commit -m "Add agentic coding framework

   Initialized .agents directory with framework for AI-assisted development.

   Co-Authored-By: Claude <noreply@anthropic.com>"
   ```

4. **Create a `CLAUDE.md` redirect file in your project root:**
   ```bash
   cat > CLAUDE.md << 'EOF'
   # Claude AI Agent Configuration

   ## Primary Rule

   **MANDATORY:** Before performing any tasks, you **MUST**:

   1. **Load `.agents/AGENTS.md`** - Central configuration entry point
   2. **Follow all instructions** in `.agents/AGENTS.md`

   ---

   👉 **Go to [`.agents/AGENTS.md`](./.agents/AGENTS.md)** to get started.
   EOF
   ```

5. **Customize for your project:**
   - Edit `.agents/stacks/*.md` to match your language preferences
   - Add project-specific rules to `.agents/rules/` (following naming conventions)
   - Create your first specification in `.agents/specifications/`

### For Existing Projects

If you already have an `.agents` directory, you can update it:

```bash
# Backup your current .agents directory
mv .agents .agents.backup

# Clone the latest framework
git clone https://github.com/ewe-studios/agentic-coding-starter .agents
rm -rf .agents/.git

# Merge your custom rules, stacks, and specifications
cp -r .agents.backup/specifications/* .agents/specifications/ 2>/dev/null
# (Review and merge other customizations manually)

# Clean up
rm -rf .agents.backup
```

## 📋 How It Works

### 1. Agent Loading Sequence

When an AI agent starts working on your project, it MUST:

1. **Read `AGENTS.md`** - The entry point
2. **Load ALL files in `rules/`** - Workflow and orchestration rules
3. **Load ONLY relevant `stacks/[language].md`** - Language standards
4. **Read specification files** - When working on features

This ensures every agent has the same context and follows the same processes.

### 2. Agent Orchestration Model

The framework uses a **hierarchical agent model**:

- **Main Agent**: Orchestrator only - delegates all work, never codes directly
- **Implementation Agents**: Specialized workers (Rust Agent, Python Agent, etc.)
- **Verification Agent**: Runs tests and checks before commits
- **Specification Agent**: Updates requirements and task tracking

**Example workflow:**
```
User: "Add user authentication"
  ↓
Main Agent: Analyzes requirement, creates specification
  ↓
Main Agent: Spawns Rust Implementation Agent
  ↓
Rust Agent: Writes authentication code, reports back
  ↓
Main Agent: Spawns Verification Agent
  ↓
Verification Agent: Runs tests, linting, builds
  ↓
Main Agent: Spawns Specification Agent
  ↓
Specification Agent: Updates tasks.md with completion
  ↓
Main Agent: Creates commit and pushes
```

### 3. Zero-Commit Without Verification

**NO CODE IS COMMITTED WITHOUT VERIFICATION.**

The framework enforces:
- All tests must pass
- All linters must pass
- All builds must succeed
- All checks must complete

If verification fails:
- A `verification.md` report is created
- Issues are documented
- Code is NOT committed until fixed

### 4. Specification-Driven Development

Every feature starts with a specification:

```markdown
specifications/
└── 01-user-authentication/
    ├── requirements.md      # What to build and why
    ├── tasks.md            # Task checklist with progress
    └── verification.md     # Failure reports (if any)
```

This ensures:
- Clear requirements before coding starts
- Trackable progress throughout implementation
- Documentation of what was built and why

## 🎓 Core Principles

1. **Orchestration Always** - Main Agent delegates, never implements
2. **Verification Required** - No commits without passing checks
3. **Zero Deviation** - All standards must be followed exactly
4. **Context Efficiency** - Only load what you need (especially stack files)
5. **Learning Logs** - Document mistakes and patterns for improvement
6. **Specification-Driven** - Every feature has clear requirements
7. **Safety First** - Dangerous operations require explicit approval

## 🔧 Customization

### Adding Project-Specific Rules

Create new rule files following the naming convention:

```bash
.agents/rules/10-my-project-rule.md
```

Rules are loaded numerically, so numbering controls precedence.

### Adding Language Stacks

Create new stack files for your languages:

```bash
.agents/stacks/golang.md
.agents/stacks/elixir.md
```

Follow the structure in existing stack files (frontmatter, standards, verification workflow, learning logs).

### Creating Skills

When agents encounter knowledge gaps, they can create skills:

```bash
.agents/skills/01-kubernetes-deployment/
├── skill.md           # Documented knowledge
└── deploy-script.sh   # Optional helper scripts
```

Skills must be approved before use.

## 📚 Key Documents

| Document | Purpose | Read When |
|----------|---------|-----------|
| **AGENTS.md** | Entry point and loading instructions | Every session start |
| **rules/04-work-commit-and-push-rules.md** | Commit requirements | Before any commit |
| **rules/05-coding-practice-agent-orchestration.md** | Agent coordination | Every session start |
| **rules/06-specifications-and-requirements.md** | Feature specification format | Creating features |
| **rules/08-verification-workflow-complete-guide.md** | Complete verification guide | Before verification |
| **stacks/[language].md** | Language-specific standards | Before writing code |

## 🤝 Contributing to the Framework

The framework is open source and welcomes contributions:

1. **Report Issues**: Found a problem? Open an issue on GitHub
2. **Suggest Improvements**: Have ideas? Create a discussion
3. **Submit Pull Requests**: Improvements welcome!
4. **Share Learning Logs**: Help others avoid common mistakes

Repository: https://github.com/ewe-studios/agentic-coding-starter

## 🆘 Troubleshooting

### Agent Not Following Rules

**Problem**: Agent ignores standards or skips verification

**Solution**:
1. Check that `CLAUDE.md` or similar redirect exists in project root
2. Verify agent loaded `AGENTS.md` at session start
3. Ensure all rules are numbered correctly in `rules/` directory
4. Explicitly remind agent: "Please load .agents/AGENTS.md and follow all rules"

### Verification Failures

**Problem**: Tests fail, builds break, linting errors

**Solution**:
1. Check `verification.md` in specification directory for details
2. Review stack file for correct verification commands
3. Ensure all dependencies are installed
4. Check that verification workflow is defined correctly

### Missing Language Stack

**Problem**: No stack file for your language

**Solution**:
1. Copy an existing stack file as template
2. Customize for your language's conventions
3. Define verification workflow (test/lint/build commands)
4. Add learning logs section for future improvement

## 📖 Further Reading

- **[AGENTS.md](./AGENTS.md)** - Start here! Entry point for all agents
- **[Specification Format Guide](./rules/06-specifications-and-requirements.md)** - How to structure requirements
- **[Verification Workflow](./rules/08-verification-workflow-complete-guide.md)** - Complete verification guide
- **[Skills System](./rules/09-skills-identification-and-creation.md)** - How skills work
- **[Skills Directory](./skills/README.md)** - Available skills and usage

## 📜 License

This framework is released under the MIT License. See the starter repository for details.

## 🙏 Credits

Developed by [Ewe Studios](https://github.com/ewe-studios) for consistent, high-quality AI-assisted software development.

---

**Version**: 3.0.0
**Last Updated**: 2026-01-14
**Repository**: https://github.com/ewe-studios/agentic-coding-starter

---

> **Remember**: This framework is a living document. As you learn better practices, update your rules, stacks, and skills. The AI agents will benefit from every improvement you make.
