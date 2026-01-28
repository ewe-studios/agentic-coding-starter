// Basic Rust Implementation Template
//
// Copy this file as starting point for new implementations.
// Follow all standards from .agents/stacks/rust.md

/// Example demonstrating clean, synchronous Rust implementation patterns.

fn main() {
    // TODO: Implement your logic here following rust.md standards:
    //
    // 1. Use Result<T, E> with ? operator
    // 2. Naming conventions (snake_case)
    // 3. Add doc comments for public items
    // 4. Type safety - newtypes and enums when appropriate

    let result = example_function();
}

/// Example function demonstrating proper error handling.
///
/// Returns Result<T, E> to propagate errors explicitly rather than using unwrap().
fn example_function() -> Result<String, String> {
    Ok("success".to_string())
}
