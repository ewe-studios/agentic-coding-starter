# Learnings from OpenSpec

So reading the code, I find it interesting in how it implements the following:

- Large array of code logic to perform validation to reduce the underlying inference the agent must do to validate schemas for:
  - Requirements and Specifications
  - Skills documentation
  - Output expectations
- Interesting YAML based specifications for the different things it will have the LLM create, see below section [Prompts][#prompts].

---

## HTTP Client Implementation Learnings (2026-03-03)

### 1. Proxy Support Implementation - Multiple Connection Paths

**Key Insight**: When implementing proxy support, always trace through ALL connection creation paths, not just the obvious ones.

**What Happened**:
- Initial implementation added proxy support to `request_stream.rs` (simple request path)
- User correctly identified missing support in `request_redirect.rs` (redirect path)
- This would have caused redirected requests to bypass proxy configuration

**Critical Learning**: HTTP client has multiple connection creation paths:
1. **Direct requests**: `GetHttpRequestStreamTask` → uses pool connection
2. **Redirect requests**: `GetHttpRequestRedirectTask` → creates new connections for redirects
3. Both paths must support the same features (proxy, timeouts, auth, etc.)

**Architecture Pattern Discovered**:
```rust
// Task state tuples must carry ClientConfig through entire chain:
type InitData = Box<(Request, Timeout, Pool, MaxRedirects, Config)>;
type TryingData = Box<(Request, Timeout, Pool, Descriptor, Redirects, Config)>;

// Both simple and redirect tasks need config parameter:
GetHttpRequestStreamTask::new(request, timeout, pool, config)
GetHttpRequestRedirectTask::new(request, timeout, pool, redirects, config)
```

**Best Practice**: When adding infrastructure features (proxy, auth, timeouts), always:
1. Grep for ALL connection creation points (`create_http_connection`, `create_https_connection`)
2. Check both simple request AND redirect task implementations
3. Verify state tuples carry necessary context through entire state machine
4. Run validation to confirm completeness across all code paths

### 2. Working with Existing Codebase Patterns - Don't Fight the Architecture

**Key Insight**: When the architecture uses specific patterns (Connection → RawStream → SharedByteBufferStream), work WITH those patterns, not against them.

**What Happened**:
- Initially tried complex workarounds to extract `Connection` from wrapped types
- User repeatedly said "don't be stupid" - signal to reconsider approach
- Solution: Use `Connection` directly from start, wrap only when needed

**Wrong Approach**:
```rust
// Trying to extract Connection from HttpClientConnection
let connection = http_conn.stream.into_inner().into_connection()  // Doesn't exist!
```

**Right Approach**:
```rust
// Work with Connection directly
let connection = Connection::with_timeout(addr, timeout)?;
// Wrap temporarily for I/O operations
let stream = SharedByteBufferStream::rwrite(RawStream::from_connection(connection.try_clone()?)?);
// Return the original Connection
Ok(connection)
```

**Lesson**: When you're fighting the type system or architecture:
1. Step back and look at existing patterns
2. Check what other similar code does (e.g., `upgrade_to_tls` takes Connection, not HttpClientConnection)
3. Ask "what does the caller actually need?" (Connection for TLS upgrade, not a wrapper)

### 3. Proxy Connection Return Types - Different Protocols Need Different Approaches

**Key Insight**: HTTP proxy and HTTPS proxy have fundamentally different return types based on their security models.

**Architecture Decision**:
```rust
// HTTP proxy: Returns plain Connection (TCP tunnel)
fn connect_via_http_proxy() -> Result<Connection, HttpClientError>

// HTTPS proxy: Returns HttpClientConnection (TLS to proxy)
fn connect_via_https_proxy() -> Result<HttpClientConnection, HttpClientError>
```

**Rationale**:
- **HTTP proxy**: Plain TCP tunnel → can be upgraded to TLS for HTTPS targets → return `Connection`
- **HTTPS proxy**: TLS to proxy already established → can't extract `Connection` → return wrapped `HttpClientConnection`
- **HTTPS target through HTTPS proxy**: Would need double-TLS (not yet supported)

**Pattern for Handling**:
```rust
match proxy_config.protocol {
    ProxyProtocol::Http => {
        let conn = pool.connect_via_http_proxy(...)?;
        if target_is_https {
            HttpClientConnection::upgrade_to_tls(conn, host, port)  // Upgrade tunnel
        } else {
            wrap_connection(conn)  // Wrap plain tunnel
        }
    }
    ProxyProtocol::Https => {
        let http_conn = pool.connect_via_https_proxy(...)?;
        if target_is_https {
            return Err(NotSupported);  // Double-TLS not supported
        }
        Ok(http_conn)  // Already wrapped with TLS to proxy
    }
}
```

### 4. Feature Completeness Validation - Always Run Comprehensive Checks

**Key Insight**: Features may LOOK complete but have subtle gaps. Validation agent provides comprehensive verification.

**Validation Results** (65 tests across 3 features):
- ✅ Cookie-Jar: 12 tests - RFC 6265 compliance verified
- ✅ Middleware: 18 tests - Thread safety (Send + Sync) verified
- ✅ Proxy-Support: 35 tests - Environment detection, NO_PROXY bypass verified

**What Validation Caught**:
- All implementations complete with no TODOs/stubs
- Documentation complete (WHY/WHAT/HOW patterns throughout)
- Code quality verified (fmt, clippy passing)
- Integration verified (all exports present in mod.rs)
- Feature specifications needed status updates to "completed"

**Best Practice**: After implementing a feature:
1. Run comprehensive test suite (not just unit tests)
2. Check for TODOs, FIXMEs, `unimplemented!()`, `todo!()`
3. Verify documentation completeness
4. Run `cargo fmt --check` and `cargo clippy`
5. Update feature specifications to reflect completion status
6. Use validation agent for final sign-off

### 5. TLS Upgrade Pattern - Reuse Existing Infrastructure

**Key Insight**: Don't reinvent TLS upgrade logic - reuse the existing `upgrade_to_tls` method.

**What We Had**:
```rust
// Existing method that does TLS handshake
impl HttpClientConnection {
    fn upgrade_to_tls(
        connection: Connection,  // Takes plain Connection!
        host: &str,
        port: u16,
    ) -> Result<Self, HttpClientError>
}
```

**What We Did**:
```rust
// HTTP proxy returns Connection → perfect for upgrade_to_tls!
let tunnel_connection = pool.connect_via_http_proxy(...)?;
if url.is_https() {
    HttpClientConnection::upgrade_to_tls(tunnel_connection, host, port)
} else {
    wrap_plain_connection(tunnel_connection)
}
```

**Lesson**: Before implementing complex logic:
1. Grep for similar functionality (`upgrade_to_tls`, `create_https_connection`)
2. Check what parameters existing methods take (guides your return types)
3. Reuse existing infrastructure rather than duplicating logic

### 6. Environment Variable Patterns - Case-Insensitive with Fallback

**Key Insight**: Follow Unix conventions for environment variables (uppercase preferred, lowercase fallback).

**Implementation Pattern**:
```rust
pub fn from_env(scheme: &Scheme) -> Option<Self> {
    if scheme.is_http() {
        Self::from_env_var("HTTP_PROXY")
            .or_else(|| Self::from_env_var("http_proxy"))  // Fallback
    } else {
        Self::from_env_var("HTTPS_PROXY")
            .or_else(|| Self::from_env_var("https_proxy"))
    }
}
```

**NO_PROXY Bypass Logic**:
```rust
// Support multiple patterns:
- Wildcard: "*" matches everything
- Exact match: "localhost" matches "localhost"
- Suffix with dot: ".example.com" matches "api.example.com"
- Suffix without dot: "example.com" matches "api.example.com" AND "example.com"
```

**Tests**: Comprehensive environment variable tests with `#[serial]` attribute to prevent race conditions.

### 7. State Machine Configuration - Carry Context Through All States

**Key Insight**: State machines need configuration available in ALL states, not just init.

**Problem**: Initially only had config in Init state:
```rust
enum State {
    Init(Request, Config),
    Trying(Request),  // Config lost!
}
```

**Solution**: Add config to ALL state tuples:
```rust
type InitData = Box<(Request, Timeout, Pool, Redirects, Config)>;
type TryingData = Box<(Request, Timeout, Pool, Descriptor, Redirects, Config)>;

// Pass config through transitions:
State::Init((req, timeout, pool, redirects, config)) => {
    State::Trying((req, timeout, pool, descriptor, redirects, config))
}
```

**Rationale**: Features like proxy need config when creating connections in ANY state (Init, Trying, Retrying, etc.).

---

## Prompts

### TDD Development Workflow

#### Feature specification defining requirements

instruction: |
template: |

    ## Feature: <!-- feature name -->

    <!-- Feature description -->

    ## Requirements

    <!-- List of requirements -->

    ## Acceptance Criteria

    <!-- List of acceptance criteria -->

Create the feature specification that defines WHAT to build.

    Sections:
    - **Feature**: Name and high-level description of the feature's purpose and user value
    - **Requirements**: List of specific requirements. Use SHALL/MUST for normative language.
    - **Acceptance Criteria**: Testable criteria in WHEN/THEN format

    Format requirements:
    - Each requirement should be specific and testable
    - Use `#### Scenario: <name>` with WHEN/THEN format for acceptance criteria
    - Define edge cases and error scenarios explicitly
    - Every requirement MUST have at least one scenario

    Example:
    ```
    ## Feature: User Authentication

    Users can securely log into the application.

    ## Requirements

    ### Requirement: Password validation
    The system SHALL validate passwords meet minimum security requirements.

    #### Scenario: Valid password accepted
    - **WHEN** password has 8+ chars, uppercase, lowercase, and number
    - **THEN** password is accepted

    #### Scenario: Weak password rejected
    - **WHEN** password is less than 8 characters
    - **THEN** system displays "Password too short" error
    ```

    This spec drives test creation - each scenario becomes a test case.

#### Test files written before implementation

template: | ## Test Plan

    <!-- Describe the testing strategy -->

    ## Test Cases

    ### <!-- Test case name -->

    - **Given:** <!-- preconditions -->
    - **When:** <!-- action -->
    - **Then:** <!-- expected result -->

instruction: |
Write tests BEFORE implementation (TDD red phase).

    File naming:
    - Create test files as `tests/<feature>.test.ts`
    - One test file per feature/capability
    - Use descriptive names matching the spec

    Test structure:
    - Use Given/When/Then format matching spec scenarios
    - Group related tests with `describe()` blocks
    - Each scenario from spec becomes at least one `it()` test

    Coverage requirements:
    - Cover each requirement from the spec
    - Include happy path (success cases)
    - Include edge cases (boundary conditions)
    - Include error scenarios (invalid input, failures)
    - Tests should fail initially (no implementation yet)

    Example:
    ```typescript
    describe('Password validation', () => {
    it('accepts valid password with all requirements', () => {
        // GIVEN a password meeting all requirements
        const password = 'SecurePass1';
        // WHEN validating
        const result = validatePassword(password);
        // THEN it should be accepted
        expect(result.valid).toBe(true);
    });

    it('rejects password shorter than 8 characters', () => {
        // GIVEN a short password
        const password = 'Short1';
        // WHEN validating
        const result = validatePassword(password);
        // THEN it should be rejected with message
        expect(result.valid).toBe(false);
        expect(result.error).toBe('Password too short');
    });
    });
    ```

    Follow the spec requirements exactly - tests verify the spec.

requires: - spec

#### Implementation code to pass the tests

template: | ## Implementation Notes

    <!-- Technical implementation details -->

    ## API

    <!-- Public API documentation -->

    ## Usage

    <!-- Usage examples -->

instruction: |
Implement the feature to make tests pass (TDD green phase).

    TDD workflow:
    1. Run tests - confirm they fail (red)
    2. Write minimal code to pass ONE test
    3. Run tests - confirm that test passes (green)
    4. Refactor if needed while keeping tests green
    5. Repeat for next failing test

    Implementation guidelines:
    - Write minimal code to pass each test - no more, no less
    - Run tests frequently to verify progress
    - Keep functions small and focused
    - Use clear, descriptive names

    Code organization:
    - Create source files in `src/<feature>.ts`
    - Export public API clearly
    - Keep implementation details private
    - Add JSDoc comments for public functions

    Example structure:
    ```typescript
    /**
    * Validates a password meets security requirements.
    * @param password - The password to validate
    * @returns Validation result with valid flag and optional error
    */
    export function validatePassword(password: string): ValidationResult {
    if (password.length < 8) {
        return { valid: false, error: 'Password too short' };
    }
    // ... additional checks
    return { valid: true };
    }
    ```

    Don't over-engineer - implement only what tests require.

requires: - tests

#### Documentation for the implemented feature

template: |

    ## Overview

    <!-- Feature overview -->

    ## Getting Started

    <!-- Quick start guide -->

    ## Examples

    <!-- Code examples -->

    ## Reference

    <!-- API reference or additional details -->

instruction: |
Document the implemented feature.

    Sections:
    - **Overview**: What the feature does and why it exists (1-2 paragraphs)
    - **Getting Started**: Quick start guide to use the feature immediately
    - **Examples**: Code examples showing common use cases
    - **Reference**: Detailed API documentation, configuration options

    Guidelines:
    - Write for the user, not the developer
    - Start with the most common use case
    - Include copy-pasteable code examples
    - Document all configuration options with defaults
    - Note any limitations, edge cases, or gotchas
    - Link to related features or specs

    Example structure:
    ```markdown
    ## Overview

    Password validation ensures user passwords meet security requirements
    before account creation or password changes.

    ## Getting Started

    Import and use the validation function:

    ```typescript
    import { validatePassword } from './password';

    const result = validatePassword('MySecurePass1');
    if (!result.valid) {
    console.error(result.error);
    }
    ```

    ## Examples

    ### Basic validation
    ...

    ### Custom error handling
    ...

    ## Reference

    ### validatePassword(password)

    | Parameter | Type | Description |
    |-----------|------|-------------|
    | password | string | The password to validate |

    **Returns**: `{ valid: boolean, error?: string }`
    ```

Reference the spec for requirements, implementation for details.
