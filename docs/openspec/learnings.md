# Learnings from OpenSpec

So reading the code, I find it interesting in how it implements the following:

- Large array of code logic to perform validation to reduce the underlying inference the agent must do to validate schemas for:
  - Requirements and Specifications
  - Skills documentation
  - Output expectations
- Interesting YAML based specifications for the different things it will have the LLM create, see below section [Prompts][#prompts].

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
