# Skill Usage Examples

Reference examples for how agents should use skills based on their usage type.

---

## Usage Type Declaration Examples

### TEMPLATE Declaration
```markdown
**Skill Usage Type**: TEMPLATE - Copy all files to project and customize

### Template: api-client.ts
**Usage**: COPY to your project and customize for your API

**Instructions**:
1. Copy: `cp api-client.ts src/clients/your-api-client.ts`
2. Copy helpers: `cp http-helpers.ts src/clients/http-helpers.ts`
3. Customize for your API
4. Import from project: `import { ApiClient } from './clients/your-api-client';`
5. NEVER import from `.agents/skills/` in project code
```

### EXECUTABLE Declaration
```markdown
**Skill Usage Type**: EXECUTABLE - Run scripts as external tools

### Script: scraper.js
**Usage**: EXECUTE as external command

```bash
node scraper.js --url <URL> --selector <CSS_SELECTOR>
```

**Output**: JSON data to stdout or file
```

### EDUCATIONAL Declaration
```markdown
**Skill Usage Type**: EDUCATIONAL - Learn pattern and implement fresh

**External Dependencies**:
- Install: `npm install jsonwebtoken`

### Example: jwt-example.ts
**Usage**: STUDY this example, then IMPLEMENT fresh code using `jsonwebtoken`

Study the pattern, install the library, write your own implementation.
NEVER import from `.agents/skills/` directory.
```

---

## Agent Usage Examples

### Using TEMPLATE Skills

**Scenario**: Create custom API client from template

```bash
# Step 1: Copy ALL files (templates + helpers)
cp .agents/skills/rest-api-client/api-client.ts ./src/clients/product-api.ts
cp .agents/skills/rest-api-client/http-helpers.ts ./src/clients/http-helpers.ts
cp .agents/skills/rest-api-client/retry-logic.ts ./src/clients/retry-logic.ts

# Step 2: Customize the COPIED files in project

# Step 3: Import from PROJECT location
```

```typescript
// In project code
import { ProductApi } from './clients/product-api';
import { handleError } from './clients/http-helpers';

// ❌ NEVER: import { ... } from '.agents/skills/...'
```

### Using EXECUTABLE Skills

**Scenario**: Scrape product data from website

```bash
# Execute script from .agents/skills/ location
node .agents/skills/web-scraper/scraper.js \
  --url "https://example.com/products" \
  --output ./data/products.json
```

```typescript
// Consume output in project code
import fs from 'fs';
const products = JSON.parse(fs.readFileSync('./data/products.json'));

// ❌ NEVER copy or modify executable scripts
// ❌ NEVER import from .agents/skills/
```

### Using EDUCATIONAL Skills

**Scenario**: Implement JWT authentication

```bash
# Step 1: Install external dependencies
npm install jsonwebtoken bcrypt

# Step 2: Study the skill examples (read, don't copy)
# Read: .agents/skills/auth/jwt-example.ts
```

```typescript
// Step 3: Implement FRESH code in project
import jwt from 'jsonwebtoken';  // From NPM, NOT .agents/skills/

export class AuthService {
  generateToken(userId: string): string {
    return jwt.sign({ userId }, this.secret, { expiresIn: '24h' });
  }
}

// ❌ NEVER: import { ... } from '.agents/skills/...'
```

---

## Frontmatter Scanning Example

```bash
# Efficient scan - frontmatter only
for skill in .agents/skills/*/skill.md; do
  head -n 20 "$skill"
done
```

---

## Seed Management Pattern

```rust
fn run_with_seed<F>(test_fn: F)
where
    F: FnOnce(u64) + std::panic::UnwindSafe
{
    let seed: u64 = std::env::var("TEST_SEED")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| rand::random());

    println!("TEST_SEED={}", seed);

    let result = std::panic::catch_unwind(|| test_fn(seed));

    if result.is_err() {
        eprintln!("Reproduce with: TEST_SEED={} cargo test", seed);
        std::panic::resume_unwind(result.unwrap_err());
    }
}
```

---

## Sub-Agent Reporting Templates

### Skill Creation Report (to Main Agent)
```
Created new skill: [skill-name]
Location: .agents/skills/[skill-name]/skill.md
Reason: [Why this skill was necessary]
Research sources: [Links to documentation/resources used]

Attached files:
- script1.js: Brief description
- examples/example1.js: Pattern example

Ready for review and user approval.
```

### Cannot Proceed - Unapproved Skill
```
Cannot proceed. Required skill not approved:
  Skill: .agents/skills/[skill-name]/skill.md
  Status: approved: No
  Reason needed: [Explanation]

Awaiting user approval to continue.
```

### Cannot Proceed - Unclear Skill
```
Cannot proceed with skill: [skill-name]

Clarity Issue: [Specific problem]
- What's unclear: [Detailed explanation]
- Why it's blocking: [Impact on implementation]
- What's needed: [What would make it clear]

Request: Please review and clarify before I proceed.
```

---

*Created: 2026-01-20*
*Purpose: Code examples and templates for Rule 09 (Skills Creation) and Rule 11 (Skills Usage)*
