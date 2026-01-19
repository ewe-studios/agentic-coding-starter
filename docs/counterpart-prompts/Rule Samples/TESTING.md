# Testing Standards

**Universal testing standards for the CA monorepo.**

All Python services follow these testing patterns to ensure consistency, maintainability, and 100% coverage.

## Table of Contents

- [Test Requirements](#test-requirements)
- [Test Naming Convention](#test-naming-convention)
- [Test Organization](#test-organization)
- [Fixtures - Preferred Patterns](#fixtures---preferred-patterns)
- [Factory-Boy for Test Data](#factory-boy-for-test-data)
- [Parametrize Usage](#parametrize-usage)
- [Given/When/Then Pattern](#givenwhenthennpattern)
- [Django Testing Patterns](#django-testing-patterns)
- [Deterministic Test Data](#deterministic-test-data)

## Test Requirements

- **100% test coverage required** for all new code
- **Always use function-based tests** - never use test classes
- Separate `unit/` and `integration/` test directories where applicable
- Run tests with `pytest` (never use direct `python3` invocation)

## Test Naming Convention

**Pattern**: `test_<function>__<scenario>__<assertion>`

The scenario is optional for simple tests.

```python
# Examples
def test_divide__zero_denominator__raises_divide_by_zero_exception():
    ...

def test_update_task_status__under_some_scenario():
    ...
```

**Service/Class naming**: Convert PascalCase to snake_case

```python
# task_consumer_service.py
class TaskConsumerService:
    def UpdateTaskStatus(self):
        ...

# test_task_consumer_service.py
def test_update_task_status__under_some_scenario():
    ...
```

## Test Organization

### Directory Structure

```
ca-example-service/
├── example_app/
│   ├── tests/
│   │   ├── unit/
│   │   │   └── test_services.py
│   │   └── integration/
│   │       └── test_database_operations.py
│   ├── services.py
│   └── models.py
```

### File Organization

- One class per file for easier maintenance
- Django `models.py` files should be tested
- Test files mirror the structure of source files

## Fixtures - Preferred Patterns

**IMPORTANT**: We are moving TOWARD "fixtures in file" and AWAY FROM bloated conftest.py

### ✅ Preferred: Define Fixtures in Test Files

```python
# test_visits.py
import pytest

@pytest.fixture
def test_visit():
    """Create a test visit with standard data."""
    return {
        "visit_id": "test-123",  # nophi
        "service_date": datetime.date(2022, 1, 1),
        "mode_of_service": "in_person"
    }

@pytest.fixture
def authenticated_user():
    """Create authenticated user context."""
    return UserFactory(email="test@example.com")  # nophi

def test_create_visit__valid_data__succeeds(test_visit, authenticated_user):
    # given:
    visit_data = test_visit

    # when:
    result = create_visit(visit_data, authenticated_user)

    # then:
    assert result.id == test_visit["visit_id"]
```

### ⚠️ Use conftest.py ONLY For

1. **pytest_configure** and pytest hooks
2. **Truly shared fixtures** used across ALL tests (auth context, global mocks)
3. **Global protections** (network call blockers, etc.)

```python
# conftest.py - ONLY for truly shared patterns
import pytest

@pytest.fixture
def mock_network_calls(mocker):
    """Block all network calls in tests (global protection)."""
    mocker.patch("requests.get")
    mocker.patch("requests.post")

# DO NOT put service-specific fixtures here
# Those belong in the test file itself
```

### Using Fixtures That Aren't Referenced

Use `@pytest.mark.usefixtures` for fixtures that have side effects but aren't used in the test function:

```python
# Bad:
def test_handle_eob_message__with_missing_icd10s(
    test_visit,
    mock_retrieve_patient_by_patient_id,
    mock_retrieve_patient_by_mbi,
    mock_user_service_search_practitioners,
    mock_user_service_get_user,
):
    # Notice how we only need to test with the test_visit fixture
    assert test_visit == "foo"

# Good:
@pytest.mark.usefixtures(
    "mock_retrieve_patient_by_patient_id",
    "mock_retrieve_patient_by_mbi",
    "mock_user_service_search_practitioners",
    "mock_user_service_get_user",
)
def test_handle_eob_message__with_missing_icd10s(test_visit):
    assert test_visit == "foo"
```

### ❌ Avoid autouse=True

**Do NOT** set `autouse=True` on fixtures in conftest.py - it makes dependencies unclear.

**Exception**: Global protections like blocking network calls.

```python
# --> BAD:
# conftest.py
@pytest.fixture(autouse=True)
def authenticated():
    ...

# test_something.py
def test_foo():
    # all tests are authenticated, but what if we need to test that someone is not?

# --> GOOD:
# conftest.py
@pytest.fixture()
def authenticated():
    ...

# test_something.py
@pytest.mark.usefixtures("authenticated")
def test_foo():
    # explicitly marked as needing authentication
```

## Factory-Boy for Test Data

Use factory-boy for generating Django model instances in tests.

### Basic Factory Pattern

```python
# user_management/factories.py
from factory import Faker, SubFactory, SelfAttribute
from factory.django import DjangoModelFactory
from user_management import models

class OrganizationFactory(DjangoModelFactory):
    """Factory for generating Organization."""

    class Meta:
        model = models.Organization

    name = Faker("company")

class UserFactory(DjangoModelFactory):
    class Meta:
        model = models.User

    organization = SubFactory(OrganizationFactory)
    individual = SubFactory(IndividualFactory)
    user_status = usr_mgmt_pb2.USER_STATUS_ACTIVE
    registration_date = datetime(2003, 3, 7).date()  # Deterministic!
```

### Using Factories in Tests

```python
# test_user_service.py
from user_management.factories import UserFactory, OrganizationFactory

def test_create_user__with_organization__succeeds():
    # given:
    org = OrganizationFactory(name="Test Org")  # nophi

    # when:
    user = UserFactory(organization=org, email="test@example.com")  # nophi

    # then:
    assert user.organization == org
    assert user.email == "test@example.com"  # nophi
```

### Custom Faker Providers

```python
# factories.py
from faker.providers import BaseProvider

class CustomProvider(BaseProvider):
    def npi(self):
        npi = self.bothify(text="1########")
        return f"{npi}{validators.get_npi_checksum(npi)}"

Faker.add_provider(CustomProvider)

class PractitionerFactory(DjangoModelFactory):
    class Meta:
        model = models.Practitioner

    name = Faker("name")
    npi = Faker("npi")  # Uses custom provider
```

## Parametrize Usage

**Parametrize is DISCOURAGED in unit tests.** Use it sparingly and follow these rules:

### When to Use Parametrize

✅ **Good use cases:**

- Testing the same function with simple variations
- 2-4 similar test cases with simple data types

❌ **Bad use cases:**

- Complex logic requiring control flow (if/else, loops)
- Nested dicts or complex data structures
- Cases that aren't extremely similar in nature

### Rules for Parametrize

1. **Should NOT** have control flow statements (if/else, for loops) in the test
2. Cases should be **extremely similar** in nature
3. Use **simple datatypes** (no nested dicts)
4. **Always use pytest.param with id**

### Example

```python
@pytest.mark.parametrize(
    "input, expected_output",
    [
        pytest.param(
            "foo",
            True,
            id="Input matches",
        ),
        pytest.param(
            "bar",
            False,
            id="Input does not match",
        ),
    ],
)
def test_match_input_to_foo__all_scenarios(input, expected_output):
    """Test whether we should trigger for a certain resource."""
    assert match_input_to_foo(input) == expected_output
```

## Given/When/Then Pattern

For complex tests, use Given/When/Then pattern for clarity:

```python
def test_some_complicated_function__happy_path(mocker):
    # given:
    mock_service = mocker.patch("some_external_service")
    request = {"some": "data"}  # nophi
    expected_id = "test-123"  # nophi

    # when:
    result = some_complicated_function(request)

    # then:
    assert result.id == expected_id
    assert mock_service.called
```

## Django Testing Patterns

### Settings Override

```python
from django.test import override_settings

@override_settings(FEATURE_FLAG_ENABLED=True)
def test_feature__when_enabled__works():
    ...
```

### Database Queries

Test for N+1 queries using `assertNumQueries`:

```python
from django.test.utils import override_settings
from django.db import connection
from django.test.utils import CaptureQueriesContext

def test_get_visits__batch_query__no_n_plus_one():
    # given:
    visits = [VisitFactory() for _ in range(10)]

    # when:
    with CaptureQueriesContext(connection) as queries:
        result = get_visits_with_patients(visits)

    # then:
    assert len(queries) <= 2  # Should use select_related, not N+1
```

## Deterministic Test Data

**CRITICAL**: Tests must be deterministic. Never use `datetime.now()` in test setup.

```python
# Bad:
RESOURCE = {"date": timezone.now().date()}

def test_foo():
    assert foo(RESOURCE)

# Good:
RESOURCE = {"date": datetime.date(2022, 1, 1)}

@freeze_time("2022-01-03 02:30:00+00:00")
def test_foo():
    assert foo(RESOURCE)
```

### Using freezegun

```python
from freezegun import freeze_time

@freeze_time("2022-01-03 02:30:00+00:00")
def test_visit_date__frozen_time__uses_fixed_date():
    # given:
    visit = VisitFactory()  # Will use frozen time

    # when:
    result = visit.created_at

    # then:
    assert result == datetime(2022, 1, 3, 2, 30, 0, tzinfo=timezone.utc)
```

## Test Data Reusability

### Build Reusable Fixtures

When you find yourself repeating setup, move it to a fixture:

```python
# Bad: Repeated setup in every test
def test_authenticated_request__scenario_1():
    user = UserFactory(email="test@example.com")  # nophi
    token = generate_token(user)
    context = create_auth_context(token)
    ...

def test_authenticated_request__scenario_2():
    user = UserFactory(email="test@example.com")  # nophi
    token = generate_token(user)
    context = create_auth_context(token)
    ...

# Good: Reusable fixture
@pytest.fixture
def authenticated_context():
    user = UserFactory(email="test@example.com")  # nophi
    token = generate_token(user)
    return create_auth_context(token)

def test_authenticated_request__scenario_1(authenticated_context):
    ...

def test_authenticated_request__scenario_2(authenticated_context):
    ...
```

### Factories vs Fixtures

- **Factories**: For Django models and complex objects
- **Fixtures**: For simple test data, mocks, and contexts

## Running Tests

```bash
# All tests with coverage
pytest

# Specific file
pytest tests/unit/test_services.py

# Specific test
pytest tests/unit/test_services.py::test_create_user__valid_data__succeeds

# With markers
pytest -m "not slow"

# Parallel execution (faster)
pytest -n auto
```

## Coverage Requirements

Every PR must maintain 100% coverage:

```bash
# Run with coverage report
pytest --cov=. --cov-report=term-missing

# Fail if coverage drops below 100%
pytest --cov=. --cov-fail-under=100
```
