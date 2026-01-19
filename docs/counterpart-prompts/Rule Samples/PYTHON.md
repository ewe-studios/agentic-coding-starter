# Python Standards

**Universal Python coding standards for the CA monorepo.**

These patterns apply to all Python code, regardless of framework (Django, FastAPI, etc.).

## Table of Contents

- [Type Hinting](#type-hinting)
- [Dataclasses](#dataclasses)
- [Imports and Modules](#imports-and-modules)
- [Functions](#functions)
- [Classes](#classes)
- [Exception Handling](#exception-handling)
- [Control Flow](#control-flow)
- [Generators and Memory Management](#generators-and-memory-management)
- [Code Organization](#code-organization)
- [Comments and Documentation](#comments-and-documentation)

## Type Hinting

### Modern Union Syntax

Use Python 3.10+ syntax for type hints:

```python
# Good: Modern syntax
def get_user(user_id: str) -> dict[str, Any] | None:
    ...

# Bad: Old typing module syntax
from typing import Optional, Dict, Any
def get_user(user_id: str) -> Optional[Dict[str, Any]]:
    ...
```

### StrEnum vs typing.Literal

**Use `typing.Literal`** for internal API signatures (static checking only):

```python
from typing import Literal

def align_text(mode: Literal["left", "right", "center"]):
    ...

align_text("left")  # IDE suggests options
```

**Use `StrEnum`** when values need runtime validation or iteration:

```python
from enum import StrEnum

class Status(StrEnum):
    ACTIVE = 'active'
    INACTIVE = 'inactive'

# Runtime validation!
if user_input not in Status:
    raise ValueError("Invalid status")
```

## Dataclasses

### Preferred Pattern: frozen=True and slots=True

**Always use `frozen=True` and `slots=True` for dataclasses:**

```python
from dataclasses import dataclass

# Good: Immutable, memory-efficient
@dataclass(frozen=True, slots=True)
class PageDimensions:
    width: int
    height: int
    title: str
    author: str
    created: datetime.datetime

# Benefits:
# - Immutability simplifies reasoning
# - Faster attribute access
# - Memory savings (small per class, adds up)
```

### When to Use Dataclasses

✅ **Use dataclasses for:**

- Structured data with 2+ attributes
- Data Transfer Objects (DTOs)
- Plain Old Data (POD) structures

❌ **Do NOT use dataclasses when:**

- FHIR protobuf, gRPC messages, or Django models already exist
- You'll serialize/deserialize to protos (use protos directly)

```python
# Bad: Duplicating what's in protobuf
@dataclass
class Visit:
    visit_id: str
    service_date: str
    mode_of_service: str

# Good: Use the proto directly
def process_visit(visit: visit_service_pb2.Visit):
    ...
```

## Imports and Modules

### Import Modules, Not Members

**CRITICAL**: Import entire modules, not individual members.

```python
# Bad:
from common.my_module import some_random_function
some_random_function()

# Good:
from common import my_module
my_module.some_random_function()

# Exceptions:
# 1. Classes with unique capitalized names
from common.my_module import SomeObviousClass
from ca_lib.grpc.exceptions import InvalidRequestArgumentsException

# 2. @patch(...) decorators in tests
from unittest.mock import patch
```

**Why?**

- Makes it explicit the function is from another file
- Reduces namespace collisions
- Enables proper mocking in tests

### No Aliasing

**Do NOT alias imports** - rename the module instead.

**Exception**: External libraries with naming conflicts (keep original name in alias).

```python
# Bad:
from visits import constants as visit_constants

# Good: Rename the file
from visits import visit_constants

# OK: External library conflict
from ca_lib import grpc
from google.protobuf import grpc as google_grpc
```

### Protobuf Imports

**Always import protobuf with `_pb2` suffix** to make it obvious the import is from generated code:

```python
# Bad:
from ca_lib.ca_lib_protocols.constants_pb2 import LINE_OF_BUSINESS_MA

# Good:
from ca_lib.ca_lib_protocols import constants_pb2

# OK: If name is too long, keep _pb2 suffix
from ca_user_service_protocols import user_management_service_pb2 as usr_mgmt_pb2
```

### No Wildcard Imports

```python
# Bad: Pollutes namespace
from foo import *

# Good:
from foo import bar
```

### All Imports at File Top

🚨 **NEVER IMPORT INSIDE FUNCTIONS OR METHODS** (ruff PLC0415)

```python
# Bad:
def some_function():
    from some_module import helper  # WRONG!
    ...

# Good:
from some_module import helper

def some_function():
    helper()
```

### Absolute Imports Only

```python
# Good:
from ca_user_service.auth import token_generation

# Bad:
from ..auth import token_generation  # Relative import
```

## Functions

### Internal Functions

Prepend internal functions with `_` and define them **BELOW** main functions:

```python
# Good:
def external_function():
    foo = _some_internal_function([1, 2, 3])
    ...

def _some_internal_function(foo_list):
    """Only used within this module."""
    return sum(foo_list)

# Bad:
def _some_internal_function(foo_list):  # Defined before main function
    return sum(foo_list)

def external_function():  # Main function should be first
    ...
```

### Return Statements

**When function usually returns something but not always:**

```python
def get_mother(person):
    if is_human(person):
        return person.mother
    else:
        return None  # Explicit
```

**When function needs early exit:**

```python
def submit_payment_for_visit(visit: models.Visit) -> None:
    if visit.is_ineligible_for_payment:
        return  # Early exit
    payment_amount = _determine_payment_amount(visit)
    _generate_payment(payment_amount)
```

**When function has side effects only (no return value):**

```python
def set_mother(person, mother):
    if is_human(person):
        person.mother = mother
    # No return statement - side effects only
```

### Functions with Side Effects

**Functions should either return a value OR have side effects - not both.**

```python
# Bad: Returns AND mutates
def append_foo(my_array: list) -> list:
    my_array.append("foo")
    return my_array

my_array = append_foo(my_array)  # Confusing

# Good: Side effect only
def append_foo(my_array: list) -> None:
    my_array.append("foo")

append_foo(my_array)  # Clear side effect
```

### Default Arguments

**Never use mutable objects or stateful calls as defaults:**

```python
# Bad: Mutable default
def foo(array: list = []):
    array.append(value)
    return array

# Bad: Stateful default
def foo(logdate=datetime.date.today()):
    return logdate.isoformat()

# Good:
def bar(array: list | None = None):
    array = array or []
    array.append(value)
    return array

# Good:
def foo(logdate: datetime.date | None = None):
    logdate = logdate or datetime.date.today()
    return logdate.isoformat()
```

## Classes

### When to Use Classes

**Our codebase is highly procedural.** Classes have a **higher bar** for use.

#### Guiding Principles

- Most operations use **standalone functions**
- Classes are **stylistically heavier** - use where structure is really needed
- Use **dataclasses** for structured data (not full classes)

#### Dataclass Rules

- Should only move sets of data around
- Shouldn't have many methods (2+ attributes, minimal methods)
- If Django model/FHIR proto exists, **reuse it** (don't create dataclass)

#### Abstract Base Classes (ABC) Rules

- Shouldn't be instantiated directly
- Should have at least 1 `@abstractmethod`
- Only use if subclassed in 2+ places
- Should have multiple methods

#### General Class Rules

Use classes when:

- Inheritance is needed and useful (but not too many layers)
- You instantiate with `__init__` and init variables or store state
- **Don't use** classes as storage of static/class methods only
- Should instantiate 3+ times (not just once)
- Should have 1+ public method and 2+ methods total

```python
# Bad: Only instantiated once
class ConfigHandler:
    def __init__(self, config_path):
        self.config = load_config(config_path)

config = ConfigHandler("config.yml")  # Only one instance

# Good: Use a function
def load_config(config_path):
    return _parse_yaml(config_path)
```

### Type Checking

Use `isinstance` and `issubclass`, not direct type comparison:

```python
# Bad:
(type(foo) == bar) or (type(foo) == baz)

# Good:
isinstance(foo, (bar, baz))
```

## Exception Handling

### Specific Exceptions

**Do NOT raise bare `Exception`** - use specific exceptions:

```python
# Bad:
raise Exception("Something went wrong")

# Good:
raise ValueError("User ID cannot be empty")
raise RuntimeError("Failed to connect to database")
```

### Exception Chaining

Use `raise ... from None` to avoid misleading exception chains:

```python
# Bad: Shows misleading ZeroDivisionError
def div(a, b):
    try:
        return a / b
    except ZeroDivisionError:
        raise ValueError('b cannot be 0.')

# Good: Clean error message
def div(a, b):
    try:
        return a / b
    except ZeroDivisionError:
        raise ValueError('b cannot be 0.') from None
```

**⚠️ Only hide exception chain when:**

- Inside a VERY specific exception handler
- Only one line could throw the exception
- You're translating the exception to something more meaningful

### Broad Exception Handling

Avoid catching bare `Exception` unless it's the last handler:

```python
try:
    main()
except (FooError, KeyboardInterrupt):
    pass
except Exception as exc:  # pylint: disable=broad-except
    logging.exception("Unknown error: %s", exc)
    sys.exit(1)
```

## Control Flow

### Match Statements

Use `match` for structural pattern matching (Python 3.10+):

```python
# Good: Match for structural dispatch
def process_command(command: str | tuple[str, ...]):
    match command:
        case "foo":
            {handle "foo" logic}
        case ("bar", filename):  # Automatically unpacks
            {handle "bar" logic}
        case ("baz", x, y):  # Automatically unpacks x and y
            {handle "baz" logic}
        case _:
            raise UnsupportedOperation

# Bad: Nested if/else for same logic
if command == "foo":
    ...
elif isinstance(command, tuple) and len(command) == 2 and command[0] == "bar":
    filename = command[1]  # Manual unpack
    ...
```

### Explicit Enum Handling

**CRITICAL**: Always handle all enum cases explicitly - never use default fallthrough:

```python
# Bad: Default case hides missing enum
match val:
    case enum_foo:
        {handle foo}
    case enum_bar:
        {handle bar}
    case _:  # or missing case
        {default logic}  # What if enum_new is added?

# Good: Explicit cases + error on unknown
match val:
    case enum_foo:
        {handle foo}
    case enum_bar:
        {handle bar}
    case enum_baz:
        {handle baz}
    case _:
        raise UnsupportedOperation  # Forces update when enum added

# Best: Type-safe with MyPy
import typing_extensions

match val:
    case enum_foo:
        ...
    case enum_bar:
        ...
    case enum_baz:
        ...
    case _:
        typing_extensions.assert_never(val)  # MyPy will fail if case missed
```

### Dictionary Access

Choose `.get()` vs `[]` based on error handling strategy:

```python
# Option 1: Graceful default
some_value = a_dict.get("some_key", "default_value")

# Option 2: Raise KeyError to alert someone
some_value = a_dict["some_key"]  # Will raise KeyError if missing

# Add context to KeyError for debugging
try:
    some_value = a_dict["critical_key"]
except KeyError:
    logging.error("Missing critical_key in config: %s", a_dict.keys())
    raise
```

### Avoid Nesting

"Flat is better than nested." - Zen of Python

```python
# Bad:
if foo:
    # do a bunch of stuff with foo
    ...
    ...

# Good: Early return
if not foo:
    raise UnsupportedError("Foo is required")
# do a bunch of stuff with foo
...
```

Use `itertools.product` instead of nested loops:

```python
# Bad:
for i in data_list:
    for j in data_list:
        if i + j == 2020:
            print(i * j)

# Good:
import itertools
for n in itertools.product(data_list, data_list):
    if sum(n) == 2020:
        print(np.product(n))
```

## Generators and Memory Management

### Use Generators to Avoid OOM

List comprehensions load everything into memory - use generators for large datasets:

```python
# Bad: Loads 100 billion items into memory
generator = range(1, 100_000_000_000)
formatted = [format(s) for s in generator]  # OOM!

# Good: Creates a generator
formatted = (format(s) for s in generator)  # Lazy evaluation
```

### When to Use Generators

✅ **Use generators when:**

- You don't need the entire set for reducer operations (sum, avg, count)
- Working with unknown size collections
- Processing large datasets

❌ **Don't use generators when:**

- You MUST reuse the list (generators can only iterate once)
- You need to iterate twice

### Batch Operations

For operations needing the full set (sum, avg, count, database queries), use batching:

```python
generator = range(1, 100_000_000_000)
CHUNK_SIZE = 1000

for chunk in moreitertools.ichunk(generator, CHUNK_SIZE):
    formatted = [format(s) for s in chunk]
    # Process 1000 records at a time
    more_resources = batch_get_resources(formatted)
```

### Generator with Decorators

**⚠️ Decorators don't work as expected on generators!**

The decorator only wraps the generator creation, not the execution:

```python
# Wrong: Decorator not applied to function body
@demo_context
def wrong_usage():
    for i in range(3):
        print(i)
        yield i

# Correct: Use context manager in function body
def correct_usage():
    with demo_context():
        for i in range(3):
            print(i)
            yield i
```

## Code Organization

### Naming Conventions

**Avoid `utils.py` and `helpers.py`** - be descriptive:

```python
# Bad:
utils.py

# Good:
string_formatting.py
date_helpers.py
validation_utils.py
```

**Unused variables**: Prefix with `_`

```python
# Loop with unused variable
for _ in range(20):
    do_something()

# Deprecated argument
def foo(bar, baz, _blah):
    # _blah is deprecated, will be removed in v2.0
    ...
```

**Never mask imports with variable names:**

```python
import socket

# Bad: Variable masks import
def blah(host, port, socket=None):
    if not socket:
        socket = socket.create_connection((host, port))  # Error!
    return socket
```

## Comments and Documentation

### When to Document

✅ **Document:**

- Non-trivial functions
- REST/gRPC entry points (public APIs)
- Complex algorithms

❌ **Do NOT document:**

- Trivial functions where docstring repeats code
- Obvious return types already in signature
- Method name with spaces instead of underscores

### Docstring Format (Google Style)

```python
def foo(bar, baz=None, **options):
    """One sentence about what this does.

    More detailed explanation if necessary, including todos, fixmes,
    caveats, etc.

    Arguments:
        bar (int): Something about bar.
        baz: More about baz. (default: None)
        **options: Additional arguments to pass to BlahClass constructor.

    Returns:
        A BlahClass instance, or None if the server returns an empty response.

    Raises:
        IOError: The server didn't respond.
    """
    ...
```

### Code Comments

```python
# Comments should be above the line they apply to
# with one space between # and text

# Exception: Inline comments for function arguments
foo(
    database='blah',
    user=user or 'aptible',  # TODO: Take username from environment
    password=None  # Force prompting user
)
```

### Never Comment Out Code

**Just delete it.** Git history preserves it.

```python
# Bad:
# def old_function():
#     ...

# Good:
# (delete it)
```

## Pylint Disabling

Always use full slug name, not number:

```python
# Bad:
except Exception as exc:  # pylint: disable=W0703

# Good:
except Exception as exc:  # pylint: disable=broad-except
```
