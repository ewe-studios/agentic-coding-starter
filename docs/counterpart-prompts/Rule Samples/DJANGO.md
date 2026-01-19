# Django Standards

**Django patterns for the CA monorepo.**

All Django services in the monorepo follow these patterns for models, queries, configuration, and data handling.

## Table of Contents

- [Base Model Classes](#base-model-classes)
- [Environment Configuration](#environment-configuration)
- [Database Queries and Performance](#database-queries-and-performance)
- [Datetime Handling](#datetime-handling)
- [Enum Integration](#enum-integration)
- [Settings Override in Tests](#settings-override-in-tests)

## Base Model Classes

**Location**: `ca_lib.djangolib.models.base`

Use these abstract base classes for consistent model patterns across services.

### UUIDPrimaryKeyModelBase

**Use for**: All new models (standard surrogate primary key)

```python
from ca_lib.djangolib.models import base

class MyModel(base.UUIDPrimaryKeyModelBase):
    name = models.CharField(max_length=255)
    # id field automatically provided (UUIDField)
```

**Benefits**:

- UUID primary keys avoid sequential ID enumeration attacks
- Enables distributed ID generation without coordination
- Safer for external API exposure

### TimestampedModelBase

**Use for**: Models needing automatic creation/modification tracking

```python
from ca_lib.djangolib.models import base

class MyModel(base.TimestampedModelBase):
    name = models.CharField(max_length=255)
    # date_created and date_modified automatically managed
```

**Fields provided**:

- `date_created` - Set once on creation
- `date_modified` - Updated on every save

### DeletionTimestampModelBase

**Use for**: Models requiring soft delete functionality

```python
from ca_lib.djangolib.models import base

class MyModel(base.DeletionTimestampModelBase):
    name = models.CharField(max_length=255)
    # Inherits date_created, date_modified, date_deleted

# Usage
my_instance.soft_delete()  # Sets date_deleted to now()

# Default manager excludes deleted records
MyModel.objects.all()  # Only non-deleted records
```

**Features**:

- Custom manager (`ExcludeDeletedModelsManager`) filters deleted records by default
- `soft_delete()` method sets `date_deleted` timestamp
- Audit trail of deletions

### EffectiveTimeRangeModelBase

**Use for**: Time-bounded records (contracts, agreements, configurations)

```python
from ca_lib.djangolib.models import base

class Contract(base.EffectiveTimeRangeModelBase):
    terms = models.TextField()
    # effective_datetime and expiration_datetime provided

# Usage
active_contracts = Contract.get_active_as_of(datetime.now())
```

**Fields provided**:

- `effective_datetime` - When record becomes active
- `expiration_datetime` - When record expires (nullable)

**Methods**:

- `get_active_as_of(timestamp)` - Query for records active at a specific time

### Combining Base Classes

You can inherit from multiple base classes:

```python
class MyModel(base.UUIDPrimaryKeyModelBase, base.DeletionTimestampModelBase):
    """UUID primary key + soft delete + timestamps."""
    name = models.CharField(max_length=255)
```

## Environment Configuration

**Use django-configurations** for environment-specific settings.

### Pattern

```python
# settings.py
from configurations import Configuration, values

class Common(Configuration):
    """Shared settings across all environments."""
    DEBUG = values.BooleanValue(False)
    SECRET_KEY = values.SecretValue()

    # Feature flags with defaults
    ENABLE_NEW_FEATURE = values.BooleanValue(False, environ_prefix=None)

    @classmethod
    def post_setup(cls):
        """Hook for post-setup initialization."""
        super().post_setup()

class Development(Common):
    """Local development settings."""
    DEBUG = True
    ENABLE_NEW_FEATURE = True

class Staging(Common):
    """QA/Staging environment settings."""
    LOG_JSON = True

    @classmethod
    def pre_setup(cls):
        """Enforce required environment variables."""
        super().pre_setup()
        # Validate required env vars

class Production(Staging):
    """Production environment settings."""
    # Inherits from Staging, overrides as needed
```

### Environment Variable Management

Categorize environment variables by requirement level:

```python
# Required - application fails to start if missing
ENVIRON_REQUIRED_KEYS = {
    "DATABASE_URL",
    "SECRET_KEY",
}

# Optional - no default allowed, but not mandatory
ENVIRON_ONLY_KEYS = {
    "SENDGRID_API_KEY",
    "SLACK_WEBHOOK",
}

# Optional - default value allowed, overridden by env var
DEFAULT_ALLOWED_KEYS = {
    "LOG_LEVEL",
    "FEATURE_FLAG_ENABLED",
}
```

### Multi-Tenancy Pattern

Use `BRAND` setting for multi-tenant configuration:

```python
BRAND = values.Value(os.getenv("BRAND") or "clover", environ_prefix=None)

_BRAND_DEFAULT_VALUES = {
    "clover": {
        "BASE_URL": "https://cloverassistant.com",
        "EMAIL_SENDER": "support@cloverhealth.com",
    },
    "counterpart": {
        "BASE_URL": "https://counterparthealth.com",
        "EMAIL_SENDER": "support@counterparthealth.com",
    },
}

BASE_URL = values.Value(_BRAND_DEFAULT_VALUES[BRAND.value]["BASE_URL"], environ_prefix=None)
```

## Database Queries and Performance

### Avoid N+1 Queries

**Bad: N+1 queries**

```python
# 1 query to get visits, then N queries for related patients
visits = Visit.objects.all()
for visit in visits:
    print(visit.patient.name)  # Query per visit!
```

**Good: Use select_related for foreign keys**

```python
# 1 query with JOIN
visits = Visit.objects.select_related("patient").all()
for visit in visits:
    print(visit.patient.name)  # No additional query
```

**Good: Use prefetch_related for reverse foreign keys / many-to-many**

```python
# 2 queries total (1 for patients, 1 for all visits)
patients = Patient.objects.prefetch_related("visits").all()
for patient in patients:
    for visit in patient.visits.all():  # No additional queries
        print(visit.service_date)
```

### Batch Database Operations

**Bad: Individual queries in loop**

```python
for user_id in user_ids:
    user = User.objects.get(id=user_id)  # N queries
    process_user(user)
```

**Good: Bulk fetch**

```python
users = User.objects.filter(id__in=user_ids)  # 1 query
user_map = {user.id: user for user in users}
for user_id in user_ids:
    process_user(user_map[user_id])
```

**Good: Bulk create**

```python
# Bad: N queries
for data in records:
    MyModel.objects.create(**data)

# Good: 1 query
MyModel.objects.bulk_create([MyModel(**data) for data in records])
```

### Transaction Performance

**Use `savepoint=False` for performance in nested transactions:**

```python
from django.db import transaction

# Bad: Creates savepoint for each nested transaction (slow)
@transaction.atomic()
def outer_function():
    for item in items:
        nested_function(item)

@transaction.atomic()
def nested_function(item):
    # Savepoint created here
    item.save()

# Good: Disable savepoints in inner transactions
@transaction.atomic()
def outer_function():
    for item in items:
        nested_function(item)

@transaction.atomic(savepoint=False)  # No savepoint overhead
def nested_function(item):
    item.save()
```

**When to use `savepoint=False`**:

- Inner transactions within a loop
- Performance-critical paths
- When you don't need partial rollback of nested transactions

**When NOT to use `savepoint=False`**:

- When you need to catch and recover from exceptions in nested transactions
- Top-level transactions (always use savepoint=True or default)

### Query Optimization Checklist

Before deploying:

```python
from django.db import connection
from django.test.utils import CaptureQueriesContext

def test_get_visits__no_n_plus_one():
    # given:
    visits = [VisitFactory() for _ in range(10)]

    # when:
    with CaptureQueriesContext(connection) as queries:
        result = get_visits_with_patients(visits)

    # then:
    assert len(queries) <= 2  # Should use select_related, not N+1
```

## Datetime Handling

**CRITICAL**: Always use `CloverDatetime` for timezone-aware datetimes.

**Location**: `ca_lib.clover_datetime.CloverDatetime`

### Why CloverDatetime?

- **All methods return UTC** - no timezone ambiguity
- **Prevents naive datetime bugs** - always timezone-aware
- **Healthcare compliance** - consistent timestamp handling for audit trails

### Common Patterns

```python
from ca_lib import clover_datetime

# Current time (UTC)
now = clover_datetime.CloverDatetime.now()  # datetime.datetime with tzinfo=UTC

# Today at specific time
today_midnight = clover_datetime.CloverDatetime.today()  # 00:00:00 UTC
today_noon = clover_datetime.CloverDatetime.today(hour=12)  # 12:00:00 UTC

# Specific datetime
dt = clover_datetime.CloverDatetime.at(2022, 1, 15, 14, 30)  # UTC

# From ISO string (handles timezone conversion)
dt = clover_datetime.CloverDatetime.from_iso_string("2022-01-15T14:30:00Z")

# Relative times
week_ago = clover_datetime.CloverDatetime.days_ago(7)
next_week = clover_datetime.CloverDatetime.days_from_now(7)

# Date boundaries
start_of_year = clover_datetime.CloverDatetime.start_of_year()
start_of_day = clover_datetime.CloverDatetime.start_of_day(some_datetime)

# Date arithmetic
age_years = clover_datetime.CloverDatetime.years_old("1990-05-15")
days_diff = clover_datetime.CloverDatetime.days_difference(end_date, start_date)
```

### In Django Models

```python
from django.db import models
from ca_lib import clover_datetime

class Visit(models.Model):
    service_date = models.DateField()
    created_at = models.DateTimeField(default=clover_datetime.CloverDatetime.now)
    # NOT: default=datetime.now (naive datetime, wrong timezone)
```

### In Tests

**Bad: Non-deterministic tests**

```python
# Test fails at different times of day
RESOURCE = {"date": timezone.now().date()}

def test_foo():
    assert process_resource(RESOURCE)
```

**Good: Use freezegun**

```python
from freezegun import freeze_time
from ca_lib import clover_datetime

RESOURCE = {"date": datetime.date(2022, 1, 1)}

@freeze_time("2022-01-03 02:30:00+00:00")
def test_foo():
    now = clover_datetime.CloverDatetime.now()
    # now is frozen to 2022-01-03 02:30:00 UTC
    assert process_resource(RESOURCE)
```

## Enum Integration

**Use ProtobufEnumField** to integrate protobuf enums with Django models.

**Location**: `ca_lib.djangolib.fields.ProtobufEnumField`

### Pattern

```python
from django.db import models
from ca_lib.djangolib.fields import ProtobufEnumField
from ca_user_service_protocols import user_management_service_pb2 as usr_mgmt_pb2

class User(models.Model):
    user_status = ProtobufEnumField(
        enum_class=usr_mgmt_pb2.UserStatus,
        default=usr_mgmt_pb2.USER_STATUS_ACTIVE,
    )

# Usage
user.user_status = usr_mgmt_pb2.USER_STATUS_SUSPENDED
user.save()

# Queries work with enum values
active_users = User.objects.filter(user_status=usr_mgmt_pb2.USER_STATUS_ACTIVE)
```

**Benefits**:

- Type-safe enum storage
- Protobuf serialization compatibility
- Database stores integer value, application uses enum

## Settings Override in Tests

Use `@override_settings` to test feature flags and configuration:

```python
from django.test import override_settings

@override_settings(FEATURE_FLAG_ENABLED=True)
def test_feature__when_enabled__works():
    # Test with feature enabled
    result = do_something_with_feature()
    assert result.feature_used

@override_settings(FEATURE_FLAG_ENABLED=False)
def test_feature__when_disabled__skips():
    # Test with feature disabled
    result = do_something_with_feature()
    assert not result.feature_used
```

**Applies to**:

- Feature flags
- Multi-tenancy settings (BRAND)
- External service URLs
- Timeout values
- Any Django setting

## Database Cache Pattern

Django's `DatabaseCache` for distributed caching:

```python
# settings.py
CACHES = {
    "default": {
        "BACKEND": "django.core.cache.backends.db.DatabaseCache",
        "LOCATION": "django_cache",
    },
    "user_permissions": {
        "BACKEND": "django.core.cache.backends.db.DatabaseCache",
        "LOCATION": "django_cache_user_permissions",
    },
}

# Usage
from django.core.cache import caches

user_cache = caches["user_permissions"]
user_cache.set("user:123:permissions", permissions_data, timeout=3600)
cached_perms = user_cache.get("user:123:permissions")
```

**When to use DatabaseCache**:

- Multi-server deployments (shared state)
- Persistent cache across restarts
- When Redis overhead isn't justified

**Performance consideration**:

- `cache.get()` internally deletes expired entries (can cause deadlocks under high concurrency)
- Catch `OperationalError` for deadlocks and treat as cache miss

```python
from django.core.cache import cache
from django.db import OperationalError

def get_cached_data(key):
    try:
        return cache.get(key)
    except OperationalError:
        # Deadlock during cache cleanup - treat as miss
        return None
```
