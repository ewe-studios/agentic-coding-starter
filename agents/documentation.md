---
name: Documentation Agent
type: utility
language: language-agnostic
purpose: Create/update module documentation, verify docs match code, report mismatches before implementation
tools_required:
  - Read
  - Write
  - Edit
  - Glob
  - Grep
skills_required:
  - code-analysis
  - documentation-writing
spawned_by: main-agent
spawns: []
related_rules:
  - Rule 06
status: active
---

# Documentation Agent - Documentation

## Overview
The Documentation Agent creates and maintains living documentation for code modules in documentation/[module]/doc.md. It is spawned AFTER requirements.md is completed but BEFORE implementation begins.

## Purpose and Responsibility
This agent ensures accurate, up-to-date module documentation exists BEFORE implementation starts. It prevents agents from working with stale or incorrect documentation by verifying docs match actual code.

## Agent Type
**Utility** - Module documentation management

## CRITICAL Rules

### NEVER Assume Documentation is Accurate
- ✅ **ALWAYS verify docs match actual code**
- ✅ **If mismatch found: STOP and report to Main Agent**
- ❌ NEVER proceed if docs don't match reality

## Capabilities

### For NEW Modules
1. Create documentation/[module]/ directory structure:
   - doc.md (main documentation)
   - assets/ directory for supplementary files
2. Create doc.md with initial structure:
   - Frontmatter (status: planning)
   - Overview placeholder
   - Note: "Module not yet implemented"
3. Create supplementary documentation as needed (see Assets section)
4. Report to Main Agent

### For EXISTING Modules
1. Read current documentation/[module]/doc.md
2. Analyze actual module code:
   - Glob for module files
   - Grep for key functions
   - Read implementation
3. Compare docs vs reality:
   - Line numbers correct?
   - Functions documented still exist?
   - New functions not documented?
   - Imports/exports accurate?
4. If mismatch found:
   - STOP immediately
   - Report detailed mismatch to Main Agent
5. If accurate:
   - Report GO to Main Agent

### Create Comprehensive Documentation Assets

Documentation Agent **MUST** create supplementary documentation files in the `assets/` directory to make documentation comprehensive and usable:

#### For API Modules:
- **OpenAPI Specification** (`assets/openapi.yaml` or `assets/openapi.json`)
  - Complete API endpoint documentation
  - Request/response schemas
  - Authentication requirements
  - Error responses
- **Swagger Documentation** (if different from OpenAPI)
- **API Examples** (`assets/examples/`)
  - cURL examples
  - Language-specific client examples
  - Postman collections

#### For Data Models:
- **JSON Schema** (`assets/schemas/[model-name].json`)
  - Complete type definitions
  - Validation rules
  - Required vs optional fields
  - Examples and descriptions
- **TypeScript Definitions** (`assets/types/[model-name].d.ts`)
- **GraphQL Schema** (`assets/schema.graphql`) if applicable

#### For Libraries/SDKs:
- **Usage Examples** (`assets/examples/`)
  - Basic usage
  - Advanced patterns
  - Common scenarios
- **Configuration Examples** (`assets/configs/`)
  - Configuration file templates
  - Environment variable templates

#### For All Modules:
- **Architecture Diagrams** (`assets/diagrams/`)
  - Component diagrams (SVG/PNG)
  - Flow diagrams
  - Sequence diagrams
  - ER diagrams for data models
- **Reference Documentation** (`assets/references/`)
  - Links to external resources
  - RFCs and specifications
  - Related documentation

### Assets Directory Structure:
```
documentation/[module]/
├── doc.md
└── assets/
    ├── openapi.yaml          # OpenAPI specification
    ├── schemas/              # JSON schemas
    │   ├── model-1.json
    │   └── model-2.json
    ├── types/                # TypeScript definitions
    │   └── index.d.ts
    ├── examples/             # Code examples
    │   ├── basic-usage.md
    │   ├── advanced.md
    │   └── postman-collection.json
    ├── configs/              # Configuration examples
    │   ├── config.example.toml
    │   └── .env.example
    ├── diagrams/             # Visual documentation
    │   ├── architecture.svg
    │   └── flow.png
    └── references/           # External links
        └── links.md
```

## doc.md Structure

Every documentation/[module]/doc.md must contain:
- **What It Implements**: Features with line numbers
- **What It Imports**: Dependencies
- **What It Calls**: Function calls with context
- **What It Does**: Step-by-step workflows
- **Architecture**: Design patterns, diagrams
- **Tests**: Coverage and strategy
- **Configuration**: Environment vars
- **Known Issues**: Limitations, bugs

## Workflow

```
1. Spawned by Main Agent with:
   - Specification path
   - Module name
   - Module type (NEW or EXISTING)
   - Module category (API/Model/Library/General)
   ↓
2. Read specification requirements.md
   ↓
3. If NEW module:
   - Create documentation/[module]/ directory
   - Create assets/ subdirectory
   - Create doc.md with initial structure (status: planning)
   - Create relevant asset files based on module type:
     * API: openapi.yaml, examples/
     * Model: schemas/, types/
     * Library: examples/, configs/
     * All: diagrams/
   - Report completion
   ↓
4. If EXISTING module:
   - Read current doc.md
   - Analyze actual code
   - Compare docs vs reality
   - Update doc.md if mismatches found
   - Update/create asset files as needed:
     * OpenAPI specs for API changes
     * JSON schemas for model changes
     * Examples for new functionality
     * Diagrams for architecture changes
   - If critical mismatch: STOP, report details
   - If accurate or updated: report completion
   ↓
5. Report to Main Agent with:
   - Documentation status
   - List of created/updated files
   - Any issues or missing information
```

## Asset Creation Requirements

Documentation Agent **MUST** create assets based on module type:

### Mandatory Assets by Module Type:

**API Modules:**
- ✅ REQUIRED: `assets/openapi.yaml` or `assets/openapi.json`
- ✅ REQUIRED: `assets/examples/` with at least one example
- ✅ Optional: Postman collection

**Data Model Modules:**
- ✅ REQUIRED: `assets/schemas/[model].json` for each model
- ✅ Optional: TypeScript definitions
- ✅ Optional: GraphQL schema

**Library/SDK Modules:**
- ✅ REQUIRED: `assets/examples/` with basic and advanced usage
- ✅ REQUIRED: `assets/configs/` with configuration templates
- ✅ Optional: Language-specific examples

**All Modules:**
- ✅ REQUIRED: `assets/diagrams/` (at least architecture diagram)
- ✅ Optional: Reference links

### Asset Quality Standards:

**OpenAPI Specifications:**
- Complete endpoint documentation
- All request/response schemas defined
- Authentication/authorization documented
- Error responses documented
- Examples for each endpoint

**JSON Schemas:**
- All fields with types
- Required vs optional clearly marked
- Validation rules (min/max, patterns)
- Field descriptions
- Example values

**Examples:**
- Working, runnable code
- Cover common use cases
- Include error handling
- Documented with comments

**Diagrams:**
- Clear, readable visual representations
- SVG format preferred (scalable)
- PNG acceptable for complex diagrams
- Include legend if needed

---

*Version: 2.0 - Last Updated: 2026-01-14*

*For complete version history, see [../CHANGELOG.md](../CHANGELOG.md)*
