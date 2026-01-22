# Test Documentation Examples

This document provides language-specific examples of the WHY/WHAT/IMPORTANCE test documentation format required in all tests.

## Purpose

Every test MUST include documentation explaining WHY it exists, WHAT it tests, and optionally why it's IMPORTANT. This prevents "mystery tests" and helps future developers understand test intent.

## Format Requirements

All test documentation must include:
1. **WHY**: Why this test exists (what problem/bug/requirement does it validate)
2. **WHAT**: What specific behavior is being tested
3. **IMPORTANCE**: Why this test matters (optional but recommended for critical tests)

---

## Rust Examples

### Example 1: Edge Case Test

```rust
/// WHY: Validates token expiration at exactly midnight (edge case from bug #234)
/// WHAT: Token with midnight expiry should be treated as expired
/// IMPORTANCE: Without this, users could access system for extra day after expiry
#[test]
fn test_token_expiry_at_midnight() {
    let token = create_token_with_expiry("2024-01-15T00:00:00Z");
    assert!(is_expired(&token));
}
```

### Example 2: Security Requirement Test

```rust
/// WHY: Password validation must enforce minimum complexity (security policy #12)
/// WHAT: Password with only lowercase letters should be rejected
/// IMPORTANCE: Prevents weak passwords that are easily compromised
#[test]
fn test_password_requires_complexity() {
    let weak_password = "password";
    let result = validate_password(weak_password);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ValidationError::InsufficientComplexity);
}
```

### Example 3: Production Incident Test

```rust
/// WHY: Fix for production incident #567 where empty cart caused crash
/// WHAT: Checkout with empty cart should return clear error, not panic
/// IMPORTANCE: Critical - was causing service downtime
#[test]
fn test_checkout_empty_cart_returns_error() {
    let cart = Cart::new();
    let result = checkout(&cart);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), CheckoutError::EmptyCart);
}
```

---

## TypeScript/JavaScript Examples

### Example 1: Rate Limiting Test

```typescript
/**
 * WHY: Rate limiter must track per-IP, not per-user (security requirement)
 * WHAT: Same IP with different users should hit rate limit
 * IMPORTANCE: Prevents distributed brute-force attacks
 */
test('rate limiter tracks by IP address', async () => {
  const ip = '192.168.1.1';
  for (let i = 0; i < 100; i++) {
    await makeRequest({ ip, user: `user_${i}` });
  }
  await expect(makeRequest({ ip, user: 'another_user' }))
    .rejects.toThrow('Rate limit exceeded');
});
```

### Example 2: API Contract Test

```typescript
/**
 * WHY: API v2 must maintain backward compatibility with v1 clients
 * WHAT: v1 request format should still work and return v1 response format
 * IMPORTANCE: Breaking this would affect 50k+ active v1 clients
 */
test('API v2 handles v1 request format', async () => {
  const v1Request = { version: 1, data: { userId: 123 } };
  const response = await apiHandler(v1Request);

  expect(response.version).toBe(1);
  expect(response.data).toHaveProperty('user_id'); // snake_case for v1
});
```

### Example 3: Edge Case Test

```typescript
/**
 * WHY: Date formatting broke for Feb 29 in leap years (bug #890)
 * WHAT: Leap year date should format correctly
 * IMPORTANCE: Caused user confusion and support tickets every 4 years
 */
test('formats leap year date correctly', () => {
  const leapDate = new Date('2024-02-29');
  expect(formatDate(leapDate)).toBe('February 29, 2024');
});
```

---

## Python Examples

### Example 1: Database Transaction Test

```python
def test_rollback_on_error():
    """
    WHY: Database transactions must rollback on any error (data integrity requirement)
    WHAT: Failed operation should not partially commit data
    IMPORTANCE: Prevents corrupted state in production database
    """
    with pytest.raises(IntegrityError):
        with transaction():
            create_user("test@example.com")
            create_user("test@example.com")  # Duplicate - should rollback both

    assert User.count() == 0  # No users should exist
```

### Example 2: Performance Requirement Test

```python
def test_query_performance():
    """
    WHY: Dashboard queries must complete under 100ms (SLA requirement)
    WHAT: Complex aggregation query should return in < 100ms
    IMPORTANCE: Customer-facing dashboard - slow queries affect user experience
    """
    start = time.time()
    result = get_dashboard_stats(user_id=12345)
    elapsed = (time.time() - start) * 1000

    assert elapsed < 100, f"Query took {elapsed}ms, expected < 100ms"
    assert result is not None
```

### Example 3: Concurrency Test

```python
@pytest.mark.asyncio
async def test_concurrent_updates_use_lock():
    """
    WHY: Concurrent balance updates caused race condition (prod bug #445)
    WHAT: Simultaneous updates should use lock and produce correct final balance
    IMPORTANCE: Without lock, users were losing money due to race condition
    """
    initial_balance = 100
    account = Account(balance=initial_balance)

    # Simulate 10 concurrent deposits of $10 each
    await asyncio.gather(*[
        account.deposit(10) for _ in range(10)
    ])

    assert account.balance == 200  # Should be 100 + (10 * 10)
```

---

## Go Examples

### Example 1: Goroutine Safety Test

```go
// WHY: Validates that cache is safe for concurrent access (requirement from design doc)
// WHAT: Multiple goroutines reading/writing should not cause race conditions
// IMPORTANCE: Cache is used by all API handlers - must be thread-safe
func TestCacheConcurrentAccess(t *testing.T) {
    cache := NewCache()
    var wg sync.WaitGroup

    // Spawn 100 goroutines writing to cache
    for i := 0; i < 100; i++ {
        wg.Add(1)
        go func(id int) {
            defer wg.Done()
            cache.Set(fmt.Sprintf("key_%d", id), id)
        }(i)
    }

    wg.Wait()

    // Verify all writes succeeded
    for i := 0; i < 100; i++ {
        val, ok := cache.Get(fmt.Sprintf("key_%d", i))
        require.True(t, ok)
        require.Equal(t, i, val)
    }
}
```

### Example 2: Error Handling Test

```go
// WHY: Network errors must be retried with exponential backoff (resilience requirement)
// WHAT: Failed HTTP request should retry 3 times before giving up
// IMPORTANCE: Prevents transient network issues from causing user-visible errors
func TestHTTPClientRetries(t *testing.T) {
    attempts := 0
    server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        attempts++
        if attempts < 3 {
            w.WriteHeader(http.StatusInternalServerError)
        } else {
            w.WriteHeader(http.StatusOK)
        }
    }))
    defer server.Close()

    client := NewClientWithRetry(3)
    resp, err := client.Get(server.URL)

    require.NoError(t, err)
    require.Equal(t, http.StatusOK, resp.StatusCode)
    require.Equal(t, 3, attempts) // Should have retried twice
}
```

---

## Documentation Guidelines

### ✅ DO

**Write concise comments**:
- 2-4 lines for WHY/WHAT
- 1 line for IMPORTANCE
- Keep it brief and scannable

**Reference context when relevant**:
- Bug numbers (`bug #234`)
- Ticket IDs (`JIRA-567`)
- Production incidents (`prod incident #445`)
- Design docs (`requirement from design doc`)
- Security policies (`security policy #12`)

**Explain business rules and edge cases**:
```rust
/// WHY: Leap seconds handling per IETF RFC 3339
/// WHAT: Time with 60th second should be treated as next minute
```

**Document non-obvious test requirements**:
```typescript
// WHY: Safari bug - Date() constructor behaves differently than Chrome
// WHAT: Date parsing should work consistently across browsers
```

**Use plain language**:
- Avoid jargon and acronyms
- Write for someone unfamiliar with the code
- Be specific about what's being tested

### ❌ DON'T

**Write obvious comments**:
```rust
// ❌ BAD
/// WHY: Tests addition
/// WHAT: Tests that 1 + 1 equals 2
#[test]
fn test_addition() {
    assert_eq!(1 + 1, 2);
}
```

**Repeat what the code already says**:
```typescript
// ❌ BAD
/**
 * WHY: Tests the login function
 * WHAT: Calls login() with username and password
 */
test('login test', () => {
  const result = login('user', 'pass');
  expect(result).toBeTruthy();
});
```

**Write essays**:
```python
# ❌ BAD - Too verbose
"""
WHY: This test exists because in the previous implementation we had
a very complex issue where the database connection would sometimes
fail intermittently, and after investigating for several weeks we
discovered that the connection pool was not being properly managed,
so we implemented a fix that ensures connections are always returned
to the pool, and this test validates that fix by...
"""
```

**Omit the WHY** (most important part):
```go
// ❌ BAD - Missing WHY
// WHAT: Tests that cache returns nil for missing key
func TestCacheMiss(t *testing.T) { ... }
```

**Document in learnings.md what belongs in test comments**:
- Test comments = Why THIS SPECIFIC test exists
- learnings.md = Overall implementation insights and patterns
- Don't duplicate between the two

---

## Quick Reference

### Minimum Required Format

**Simple format**:
```
WHY: [Why does this test exist?]
WHAT: [What behavior is being tested?]
```

**With importance** (for critical tests):
```
WHY: [Why does this test exist?]
WHAT: [What behavior is being tested?]
IMPORTANCE: [Why does this matter?]
```

### When to Add IMPORTANCE

Add IMPORTANCE when:
- ✅ Test validates critical security requirement
- ✅ Test prevents data corruption or loss
- ✅ Test fixes high-impact production bug
- ✅ Test validates SLA or performance requirement
- ✅ Test ensures backward compatibility
- ✅ Breaking this test would have serious consequences

Skip IMPORTANCE when:
- ❌ Test is self-explanatory
- ❌ Impact is obvious from WHY/WHAT
- ❌ Test is routine/standard validation

---

## Language-Specific Comment Styles

| Language | Comment Style |
|----------|---------------|
| Rust | `/// Comment` (doc comment above test) |
| TypeScript/JS | `/** Comment */` (JSDoc style) |
| Python | `"""Comment"""` (docstring inside function) |
| Go | `// Comment` (regular comment above function) |
| Java | `/** Comment */` (Javadoc style) |
| C# | `/// <summary>Comment</summary>` (XML doc comment) |

---

## Validation Checklist

Before committing tests, verify:
- [ ] Every test has WHY comment
- [ ] Every test has WHAT comment
- [ ] Critical tests have IMPORTANCE comment
- [ ] Comments are concise (not essays)
- [ ] Bug/ticket numbers referenced when relevant
- [ ] Plain language used (no unnecessary jargon)
- [ ] Comments add value (not repeating code)

---

*Created: 2026-01-22*
*Referenced in: Rule 05, section "Test Documentation Requirements"*
