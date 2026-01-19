# gRPC and Protocol Buffers Standards

**gRPC service patterns for the CA monorepo.**

All gRPC services follow these patterns for protocol buffer usage, service registration, and implementation.

## Table of Contents

- [Protocol Buffer Imports](#protocol-buffer-imports)
- [Code Generation Workflow](#code-generation-workflow)
- [Service Registration](#service-registration)
- [Service Implementation](#service-implementation)
- [Authentication Decorators](#authentication-decorators)
- [Error Handling](#error-handling)

## Protocol Buffer Imports

**CRITICAL**: Always import protobuf modules with the `_pb2` suffix.

This makes it obvious the import is from generated code and prevents confusion with application modules.

### Import Pattern

```python
# Good: Clear that these are generated protobuf files
from ca_lib.ca_lib_protocols import constants_pb2
from ca_user_service_protocols import user_management_service_pb2 as usr_mgmt_pb2

# Usage
status = constants_pb2.STATUS_ACTIVE
user_type = usr_mgmt_pb2.USER_STATUS_ACTIVE

# Bad: Unclear if this is generated or application code
from ca_lib.ca_lib_protocols.constants_pb2 import STATUS_ACTIVE
```

### Import Naming

**Default**: Use full module name with `_pb2` suffix

```python
from ca_user_service_protocols import user_management_service_pb2

user = user_management_service_pb2.User()
```

**When name is too long**: Keep `_pb2` suffix in alias

```python
# OK: Long name - alias but keep _pb2 suffix
from ca_user_service_protocols import user_management_service_pb2 as usr_mgmt_pb2

user = usr_mgmt_pb2.User()

# Bad: Lost _pb2 suffix
from ca_user_service_protocols import user_management_service_pb2 as usr_mgmt
```

### gRPC Service Imports

Import both the message definitions (`_pb2`) and the gRPC service stubs (`_pb2_grpc`):

```python
from ca_user_service_protocols import auth_service_pb2
from ca_user_service_protocols import auth_service_pb2_grpc

# auth_service_pb2 contains message definitions
request = auth_service_pb2.CreateTokenRequest()

# auth_service_pb2_grpc contains service stubs
# Used for service registration and client stubs
```

## Code Generation Workflow

### Generating Protobuf Code

**Always run after modifying `.proto` files:**

```bash
make protoc
```

This generates:

- `*_pb2.py` - Message definitions and serialization
- `*_pb2_grpc.py` - Service stubs and client/server interfaces
- `*_pb2.pyi` - Type stubs for mypy

### Protoc Command (from Makefile)

```makefile
protoc:
 uv run python -m grpc_tools.protoc \
   -I=. -I../ca-messaging -I../ca-lib \
   --python_out=. \
   --grpc_python_out=. \
   --mypy_out=relax_strict_optional_primitives:. \
   --mypy_grpc_out=. \
   ca_user_service_protocols/*.proto
```

**Flags explained**:

- `-I=.` - Include current directory for imports
- `-I../ca-messaging` - Include monorepo messaging protos
- `-I../ca-lib` - Include shared ca-lib protos
- `--python_out=.` - Generate Python message code
- `--grpc_python_out=.` - Generate gRPC service code
- `--mypy_out=...` - Generate mypy type stubs
- `--mypy_grpc_out=.` - Generate mypy stubs for gRPC

### Generated Files Policy

**Generated files ARE committed to version control** in this monorepo.

```bash
# After running make protoc, commit generated files
git add ca_user_service_protocols/*_pb2.py
git add ca_user_service_protocols/*_pb2_grpc.py
git add ca_user_service_protocols/*_pb2.pyi
git commit -m "EI-123 regenerate protobuf code after service changes"
```

**Why commit generated files?**

- Ensures all developers use identical generated code
- CI/CD doesn't need protoc installation
- Easier to review proto changes via diffs

### Clean Generated Files

```bash
make clean

# Deletes:
# - *_pb2.py
# - *_pb2_grpc.py
# - *_pb2.pyi
```

Use before regenerating to ensure clean state.

## Service Registration

All gRPC services must be registered in the service's `services.py` file.

### Registration Pattern

**Location**: `ca_user_service/ca_user_service/services.py`

```python
from ca_user_service_protocols import auth_service_pb2_grpc
from ca_user_service_protocols import user_management_service_pb2_grpc
from auth import services as auth_services
from user_management import services as user_management_services

SERVICES_TO_REGISTER = [
    (
        auth_service_pb2_grpc.add_AuthServiceServicer_to_server,
        auth_services.AuthService,
    ),
    (
        user_management_service_pb2_grpc.add_UserManagementServiceServicer_to_server,
        user_management_services.UserManagementService,
    ),
]
```

**Pattern**:

1. Import the `add_*Servicer_to_server` function from `*_pb2_grpc`
2. Import your service implementation class
3. Add tuple to `SERVICES_TO_REGISTER` list: `(registration_function, service_class)`

### Server Startup

The gRPC server automatically registers all services in `SERVICES_TO_REGISTER`:

```python
# In ca_user_service/management/commands/run_grpc_server.py
from ca_user_service.services import SERVICES_TO_REGISTER

for add_servicer_to_server, service_class in SERVICES_TO_REGISTER:
    add_servicer_to_server(service_class(), server)
```

## Service Implementation

### Basic Service Class

Inherit from the generated `*Servicer` base class:

```python
from grpc import ServicerContext
from ca_lib.grpc.decorators import DANGEROUS_unauthorized

from ca_user_service_protocols import health_check_service_pb2 as hc_pb2
from ca_user_service_protocols import health_check_service_pb2_grpc as hc_pb2_grpc


class HealthCheckService(hc_pb2_grpc.HealthCheckServiceServicer):
    @DANGEROUS_unauthorized
    def RichHealthCheck(
        self,
        request: hc_pb2.RichHealthCheckRequest,
        context: ServicerContext,
    ) -> hc_pb2.RichHealthCheckResponse:
        """
        Perform a rich health check, checking downstream services and database connections.
        """
        # Implementation here
        return hc_pb2.RichHealthCheckResponse(
            healthy=True,
            version="abc123",
        )
```

### Method Signature Pattern

**Every gRPC method has the same signature:**

```python
def MethodName(
    self,
    request: RequestProto,
    context: ServicerContext,
) -> ResponseProto:
    """Docstring describing the RPC."""
    # Implementation
    return ResponseProto()
```

- `request` - Protobuf message from client
- `context` - gRPC context (for metadata, authentication, cancellation)
- Return type - Protobuf message response

### Type Hints Required

```python
# Good: Full type hints
def CreateUser(
    self,
    request: usr_mgmt_pb2.CreateUserRequest,
    context: ServicerContext,
) -> usr_mgmt_pb2.CreateUserResponse:
    ...

# Bad: Missing type hints
def CreateUser(self, request, context):
    ...
```

## Authentication Decorators

**Location**: `ca_lib.grpc.decorators`

All service methods require authentication unless explicitly marked otherwise.

### Standard Authentication

```python
from ca_lib.grpc.decorators import ca_user_service_authenticated

@ca_user_service_authenticated
def CreateUser(
    self,
    request: usr_mgmt_pb2.CreateUserRequest,
    context: ServicerContext,
) -> usr_mgmt_pb2.CreateUserResponse:
    # User authentication is verified
    # Access user from context.auth_user_id
    ...
```

### Unauthenticated Endpoints

**Use sparingly** - only for health checks, public endpoints:

```python
from ca_lib.grpc.decorators import DANGEROUS_unauthorized

@DANGEROUS_unauthorized
def RichHealthCheck(
    self,
    request: hc_pb2.RichHealthCheckRequest,
    context: ServicerContext,
) -> hc_pb2.RichHealthCheckResponse:
    # No authentication required
    ...
```

**The `DANGEROUS_` prefix is intentional** - it forces you to think about whether an endpoint should be public.

### Common Decorators

```python
# Standard user authentication
@ca_user_service_authenticated
def method(...): ...

# No authentication (health checks, public APIs)
@DANGEROUS_unauthorized
def method(...): ...

# Require specific permissions
@ca_user_service_authenticated
@require_permission(Permission.ADMIN)
def method(...): ...
```

## Error Handling

Use `ca_lib.grpc.exceptions` for standard gRPC errors.

### Common Exception Types

```python
from ca_lib.grpc.exceptions import (
    InvalidRequestArgumentsException,
    PermissionDeniedException,
    ResourceNotFoundException,
)

def GetUser(
    self,
    request: usr_mgmt_pb2.GetUserRequest,
    context: ServicerContext,
) -> usr_mgmt_pb2.GetUserResponse:
    # Validation
    if not request.user_id:
        raise InvalidRequestArgumentsException("user_id is required")

    # Not found
    user = User.objects.filter(id=request.user_id).first()
    if not user:
        raise ResourceNotFoundException(f"User {request.user_id} not found")

    # Permission check
    if not has_permission(context.auth_user_id, user):
        raise PermissionDeniedException("Cannot access this user")

    return usr_mgmt_pb2.GetUserResponse(user=user_to_proto(user))
```

### Exception Hierarchy

- `InvalidRequestArgumentsException` - Invalid input (400)
- `ResourceNotFoundException` - Resource not found (404)
- `PermissionDeniedException` - Insufficient permissions (403)
- `UnauthenticatedException` - Authentication failed (401)
- `InternalServerException` - Unexpected server error (500)

### Exception Best Practices

```python
# Good: Specific exception with helpful message
if not user.is_active:
    raise InvalidRequestArgumentsException(
        f"User {user.id} is not active and cannot be modified"
    )

# Bad: Generic exception without context
if not user.is_active:
    raise Exception("User not active")

# Good: Catch specific exceptions
try:
    dangerous_operation()
except ValueError as exc:
    raise InvalidRequestArgumentsException(f"Invalid value: {exc}") from None

# Bad: Catch all exceptions
try:
    dangerous_operation()
except Exception as exc:  # Too broad
    raise InternalServerException(str(exc))
```

## Proto File Organization

### Directory Structure

```
ca-user-service/
├── ca_user_service_protocols/
│   ├── auth_service.proto
│   ├── user_management_service.proto
│   ├── auth_service_pb2.py (generated)
│   ├── auth_service_pb2_grpc.py (generated)
│   └── auth_service_pb2.pyi (generated)
```

### Proto File Naming

- Service definitions: `*_service.proto`
- Message definitions: `*_messages.proto`
- Generated files: `*_pb2.py`, `*_pb2_grpc.py`, `*_pb2.pyi`

## Testing gRPC Services

### Unit Testing Pattern

```python
from ca_user_service_protocols import user_management_service_pb2 as usr_mgmt_pb2
from user_management.services import UserManagementService

def test_create_user__valid_request__succeeds():
    # given:
    service = UserManagementService()
    request = usr_mgmt_pb2.CreateUserRequest(
        email="test@example.com",  # nophi
        name="Test User",
    )
    context = create_mock_context()  # Test utility

    # when:
    response = service.CreateUser(request, context)

    # then:
    assert response.user.email == "test@example.com"  # nophi
    assert response.user.name == "Test User"
```

### Mock gRPC Context

```python
from unittest.mock import Mock
from grpc import ServicerContext

def create_mock_context(user_id: str = "test-user-id") -> ServicerContext:
    context = Mock(spec=ServicerContext)
    context.auth_user_id = user_id
    return context
```
