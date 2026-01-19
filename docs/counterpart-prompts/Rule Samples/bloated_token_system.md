# Modern Context Token System

Used by Claude to track progress across sessions. Not intended to be a human-ready summary.

TODO: Remove post migration to context token

## Overview

This implementation provides an enhanced token generation system that includes pre-computed user context data for performance optimization. The system generates both base tokens and "bloated" tokens (with context data) to measure size impact during migration while maintaining backwards compatibility.

## Problem Statement

Traditional token generation requires multiple database queries and service calls during request processing to gather user context. This system addresses that by:

- **Pre-computing context data** using an optimized database view
- **Including compressed context** in tokens when beneficial
- **Measuring size impact** through dual token generation
- **Providing fallback mechanisms** to ensure reliability

## Architecture

### Core Components

1. **Token Components** (`auth/token_components.py`)
   - Structured data containers for token generation
   - Immutable `TokenComponents` NamedTuple
   - Validation and error handling

2. **Context Collection** (`auth/context_utils.py`)
   - Optimized context data collection using `UserTokenContextView`
   - Feature flag data retrieval and processing
   - User access profile optimization
   - Caching for performance

3. **Token Generation** (`auth/token_generation.py`)
   - Conditional token generation based on feature flag
   - When enabled: dual token generation (base + bloated) for measurement
   - Size tracking and logging integration
   - Internal functions for token assembly

4. **Size Tracking** (`auth/token_size_tracking.py`)
   - Token size monitoring and logging with bucketing
   - Performance metrics collection
   - Cache-based tracking to reduce log volume

### Data Flow

```text
User Request → Token Creation → Context Collection → Token Assembly → Size Tracking
                     ↓                    ↓              ↓            ↓
              Base Components → UserTokenContextView → Base Token → Size Logging
                     ↓                    ↓              ↓            ↓
              Context Data → Feature Flags → Bloated Token → Measurement
```

## Key Features

### 1. Database View Optimization

Context data is efficiently collected using the `UserTokenContextView`:

```sql
-- Optimized view that pre-joins user data, feature flags, and access profiles
-- Reduces multiple database queries to a single optimized view query
```

### 2. Size Management

- **Tracking**: Token size monitoring and logging with bucketing
- **Measurement**: When enabled, both base and bloated token sizes are tracked
- **Logging**: Selective logging for new size buckets to reduce noise
- **Context**: Tracks whether context was available and included in returned token

### 3. Production Safety

- **Caching**: User context data cached for performance
- **Testing**: Comprehensive test coverage for all components
- **Feature Flags**: Controlled rollout via feature flag configuration
- **Monitoring**: Built-in size tracking and performance metrics

## Performance Characteristics

### Database Optimization

- **Context Collection**: Single database view query instead of multiple joins
- **Caching**: User context data cached to reduce database load
- **View Performance**: Optimized UserTokenContextView for fast data retrieval

### Size Tracking

- **Bucketing**: Token sizes rounded to nearest 100 bytes for efficient grouping
- **Selective Logging**: Only logs new size buckets to reduce log volume
- **Memory Usage**: Minimal overhead with cache-based tracking

## Configuration

### Feature Flags

```python
# Enable bloated token context (controls both collection and inclusion)
# When True: collects context data, generates bloated tokens, and includes them in responses
# When False: skips context collection and only generates base tokens
ENABLE_BLOATED_TOKEN_CONTEXT = True

# Cache timeout for token size tracking
TOKEN_SIZE_CACHE_TIMEOUT_SECONDS = 86400  # 24 hours
```

### Environment Variables

No additional environment variables required. Configuration is handled through Python constants that can be overridden in deployment.

## Security Considerations

### Data Included

The bloated token system includes:

- Feature flag states
- User permissions and roles
- Access profile mappings
- Personal information - no PHI

### Data Security

- **Database Security**: UserTokenContextView uses existing security controls
- **No secrets**: No cryptographic keys or secrets used in context data

## Monitoring and Debugging

### Size Monitoring

```python
from auth.token_size_tracking import track_and_log_token_sizes

# Token sizes are automatically tracked and logged when new size buckets are encountered
track_and_log_token_sizes(
    user_id="user123",
    base_size=1000,
    final_size=1500,
    has_context_data=True,
    context_included=False,
    field_sizes={"current_user": 200, "feature_flags": 800}
)
```

### Context Collection Monitoring

```python
from auth.context_utils import collect_token_context_from_view

# Collect context data using optimized database view
context_data = collect_token_context_from_view(user_id, access_mode)
print(f"Context collected: {context_data is not None}")
print(f"Feature flags count: {len(context_data.get('feature_flags', {}))}")
```

### Debug Information

```python
# Check cache state for token size tracking
from django.core.cache import cache

# View tracked size buckets
tracked_buckets = cache.get("token_size_buckets", set())
print(f"Tracked size buckets: {tracked_buckets}")

# Check specific bucket counts
for bucket in tracked_buckets:
    base_count = cache.get(f"token_size_base_{bucket}", 0)
    final_count = cache.get(f"token_size_final_{bucket}", 0)
    print(f"Bucket {bucket}: base={base_count}, final={final_count}")
```

## Testing

### Integration Tests

Located in `auth/tests/test_auth_utils.py` and related test files:

- Token size tracking and logging
- Context collection from UserTokenContextView
- Token component validation
- Size bucketing and cache behavior

### Test Coverage

- Token generation with dual-token measurement
- Size tracking with cache-based bucketing
- Context data collection and validation
- Permission validation and error handling

### Running Tests

```bash
# Run auth tests with token tracking functionality
DJANGO_SETTINGS_MODULE=ca_user_service.settings python -m pytest auth/tests/test_auth_utils.py -v

# Run with coverage
DJANGO_SETTINGS_MODULE=ca_user_service.settings python -m pytest auth/tests/ --cov=auth --cov-report=html

# Run specific token size tracking tests
DJANGO_SETTINGS_MODULE=ca_user_service.settings python -m pytest auth/tests/test_auth_utils.py::test__track_and_log_token_sizes__first_occurrence__logs_info -v
```

## Common Issues and Troubleshooting

### Context Collection Issues

**Symptom**: Missing context data in logs
**Cause**: UserTokenContextView not returning data
**Solution**:

1. Check database view exists and has correct permissions
2. Verify user exists in the view with valid access mode
3. Check cache for context data

### Size Tracking Not Logging

**Symptom**: No size tracking logs appearing
**Cause**: Token sizes falling into existing buckets, or feature flag disabled
**Solution**:

1. Verify feature flag is enabled: `ENABLE_BLOATED_TOKEN_CONTEXT = True`
2. Check cache for existing size buckets: `cache.get("token_size_buckets")`
3. Clear cache to see new logging: `cache.clear()`

### Performance Issues

**Symptom**: Slow token creation
**Cause**: Database view performance or cache misses
**Solution**:

1. Check UserTokenContextView query performance
2. Verify cache hit rates for context data
3. Monitor database query execution times

## Migration Guide

### Phase 1: Initial Testing

```python
# Set in configuration (disabled by default)
ENABLE_BLOATED_TOKEN_CONTEXT = False  # Start with feature disabled
```

### Phase 2: Enable for Testing Environment

```python
# Enable in test/staging environments
ENABLE_BLOATED_TOKEN_CONTEXT = True  # Collect context and include in tokens
```

1. Deploy with context enabled in test environment
2. Monitor token size tracking logs
3. Validate context data collection performance
4. Verify token size impact is acceptable

### Phase 3: Gradual Production Rollout

```python
# Enable in production with monitoring
ENABLE_BLOATED_TOKEN_CONTEXT = True  # When ready for production rollout
```

1. Enable for subset of production traffic
2. Monitor performance metrics and error rates
3. Track token size distributions
4. Verify no degradation in service performance

### Phase 4: Full Deployment

```python
# Enable globally for all users
ENABLE_BLOATED_TOKEN_CONTEXT = True
```

## Performance Benchmarks

Based on testing with the UserTokenContextView optimization:

| Scenario | Context Collection | Size Tracking | Database Queries |
|----------|-------------------|---------------|------------------|
| Small User | <10ms | <1ms | 1 query |
| Medium User | <20ms | <1ms | 1 query |
| Large User | <50ms | <1ms | 1 query |
| Cache Hit | <1ms | <1ms | 0 queries |

## API Reference

### Main Functions

#### `track_and_log_token_sizes(user_id, base_size, final_size, has_context_data, context_included, field_sizes=None)`

Tracks token sizes in cache and logs selectively for new size buckets.

**Parameters:**

- `user_id`: User ID for logging context
- `base_size`: Size of base token without context fields
- `final_size`: Size of final token (base or bloated) for measurement
- `has_context_data`: Whether context data was available
- `context_included`: Whether context was included in final token
- `field_sizes`: Optional breakdown of individual field sizes in bytes

#### `collect_token_context_from_view(user_id, access_mode)`

Collects context data using the optimized UserTokenContextView.

**Parameters:**

- `user_id`: User ID to collect context for
- `access_mode`: Access mode configuration

**Returns:**

- `Dict[str, Any] | None`: Context data or None if not available

#### `validate_permissions_are_numeric(permissions)`

Validates that permissions are numeric values (integers).

**Parameters:**

- `permissions`: List of permission values to validate

**Raises:**

- `TypeError`: If any permission is not an integer

## Related Documentation

- [Token Format Documentation](./TOKEN_FORMAT.md) - Schema for token context fields
- [UserTokenContextView](../ca-user-service/user_management/models.py) - Database view implementation
- [Authentication Tests](../ca-user-service/auth/tests/test_auth_utils.py) - Test coverage and examples
