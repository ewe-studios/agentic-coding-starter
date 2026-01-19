# Monorepo Structure and Utilities

**CA monorepo organization and shared utilities reference.**

This document covers the monorepo structure, workspace configuration, and the shared ca-lib utilities available to all services.

## Table of Contents

- [Monorepo Structure](#monorepo-structure)
- [Workspace Configuration](#workspace-configuration)
- [ca-lib Utilities](#ca-lib-utilities)
- [Service Categories](#service-categories)
- [Cross-Service Dependencies](#cross-service-dependencies)

## Monorepo Structure

### Workspace Root

```
/Users/jacobyoung/code/ca/
├── pyproject.toml          # Workspace root configuration
├── uv.lock                 # Unified dependency lock file
├── ca-lib/                 # Shared utilities library
├── ca-messaging/           # Pub/sub messaging infrastructure
├── ca-analytics-platform/  # Analytics and metrics
├── ca-user-service/        # User management and authentication
├── ca-*-service/           # Individual microservices
└── docs/                   # Shared documentation
    ├── TESTING.md
    ├── PYTHON.md
    ├── DJANGO.md
    ├── GRPC.md
    ├── MAKEFILE.md
    └── MONOREPO.md (this file)
```

### Virtual Workspace Root

The root `pyproject.toml` is a **virtual workspace** - it doesn't build/install a package itself:

```toml
[project]
name = "ca-workspace"
version = "0.0.0"

[tool.uv]
package = false  # Virtual root - don't install this
```

**Purpose**:

- Provides unified `uv.lock` for all workspace members
- Manages monorepo-wide dev tools (ruff, pytest, prek)
- Enables cross-service dependency resolution

## Workspace Configuration

### Active Workspace Members

Services currently managed by the monorepo workspace (in `pyproject.toml`):

**Core Libraries**:

- `ca-lib` - Shared utilities
- `clover-fhir` - FHIR protocol definitions
- `clover-services-authentication` - Authentication framework
- `ca-messaging` - Pub/sub messaging
- `ca-analytics-platform` - Analytics infrastructure

**Microservices**:

- `ca-user-service` - User management, authentication, cohorts
- `ca-patient-data-service` - Patient data management
- `ca-clinical-data-service` - Clinical data processing
- `ca-chart-service` - Medical chart management
- `ca-task-service` - Task workflow orchestration
- `ca-recommender-framework` - Healthcare recommendation engine
- `ca-diagnosis-suspecting-service` - Diagnosis prediction
- `ca-care-quality-service` - Quality measure tracking

### Excluded Services (Legacy/Migration)

Services not yet migrated to uv workspace:

**Excluded**:

- `ca-dev` - Development tools
- `ca-pipeline-lib` - Data pipeline utilities
- `ca-ehr-data-collect-service` - EHR data collection
- `ca-ehr-service` - EHR integration
- `ca-ehr-shared-service` - Shared EHR utilities
- `ca-rtf-service` - RTF document processing
- `ca-scribe-service` - Medical scribing
- `ca-fhir-ingest-service` - FHIR data ingestion
- `ca-api-gateway-service` - API gateway
- `ca-visit-proxy-service` - Visit proxy
- `ca-visit-service` - Visit management
- `ca-visit-app` - Visit application
- `ca-end-to-end-testing` - E2E test suite

**Why excluded?** These services are in the process of migration to the workspace or have special build requirements.

## ca-lib Utilities

**ca-lib is the foundation library** - all services depend on it for shared utilities, patterns, and infrastructure.

### gRPC and Authentication

**Location**: `ca_lib.grpc/`

```python
from ca_lib.grpc.decorators import ca_user_service_authenticated
from ca_lib.grpc.decorators import DANGEROUS_unauthorized
from ca_lib.grpc.exceptions import InvalidRequestArgumentsException
from ca_lib.grpc.exceptions import ResourceNotFoundException
from ca_lib.grpc.exceptions import PermissionDeniedException
```

**Provides**:

- Authentication decorators for gRPC methods
- Standard exception types for error handling
- JWT token parsing and validation
- Request context management
- gRPC interceptors

**See also**: [GRPC.md](GRPC.md) for detailed patterns.

### Django Integration

**Location**: `ca_lib.djangolib/`

```python
from ca_lib.djangolib.models import base
from ca_lib.djangolib.fields import ProtobufEnumField
from ca_lib.djangolib import settings_utils
```

**Provides**:

- Base model classes (`UUIDPrimaryKeyModelBase`, `TimestampedModelBase`, `DeletionTimestampModelBase`)
- Protobuf enum integration
- Settings management utilities
- Database retry wrapper
- Sentry integration

**See also**: [DJANGO.md](DJANGO.md) for detailed patterns.

### Datetime Handling

**Location**: `ca_lib.clover_datetime`

```python
from ca_lib import clover_datetime

now = clover_datetime.CloverDatetime.now()
today = clover_datetime.CloverDatetime.today()
dt = clover_datetime.CloverDatetime.at(2022, 1, 15, 14, 30)
```

**Provides**:

- Timezone-aware datetime wrapper (all UTC)
- Date arithmetic utilities
- ISO string parsing

**See also**: [DJANGO.md - Datetime Handling](DJANGO.md#datetime-handling)

### Healthcare Domain Logic

#### Line of Business (LOB) Utilities

**Location**: `ca_lib.line_of_business_utils/`

```python
from ca_lib.line_of_business_utils import lob_constants

lob = lob_constants.LineOfBusiness.MA
lob_names = lob_constants.LineOfBusiness.values()
```

**Provides**:

- Multi-tenant LOB configuration
- LOB-specific business rules
- Short name format (MA, DCE, DUKE__AETNA)

#### FHIR Utilities

**Location**: `ca_lib.fhir_utils/`

**Provides**:

- FHIR resource parsing
- Healthcare data format conversions
- FHIR validation utilities

#### HEDIS Hero

**Location**: `ca_lib.hedis_hero/`

**Provides**:

- Healthcare quality measure tracking
- HEDIS compliance scoring
- Quality metrics calculation

#### Gaps in Care

**Location**: `ca_lib.gaps_in_care/`

**Provides**:

- Care gap identification
- Clinical quality measure tracking
- Patient cohort analysis

### Data and Testing Utilities

#### Fake Data Generation

**Location**: `ca_lib.fake_data/`

```python
from ca_lib.fake_data import factories
from ca_lib.fake_data import generators
```

**Provides**:

- Comprehensive test data generation
- Healthcare-specific fake data (diagnoses, medications, visits)
- Deterministic test fixtures

**See also**: [TESTING.md - Factory-Boy](TESTING.md#factory-boy-for-test-data)

#### Synthetic Data

**Location**: `ca_lib.synthetic_data/`

**Provides**:

- FHIR-compliant synthetic patient data
- Realistic patient cohorts for testing
- Privacy-safe demonstration data

#### Demo Utilities

**Location**: `ca_lib.demo_utils/`

**Provides**:

- Demo environment alignment
- Test data generation for demos
- Environment-specific data seeding

### Platform Infrastructure

#### Redis Caching

**Location**: `ca_lib.redis/`

```python
from ca_lib.redis import client
from ca_lib.redis import lock_manager
```

**Provides**:

- Distributed caching
- Redis lock management
- Cache warming utilities

#### Performance Tracking

**Location**: `ca_lib.performance/`

```python
from ca_lib.performance import metrics
from ca_lib.performance import timing
```

**Provides**:

- Prometheus metrics integration
- Performance timing decorators
- Request latency tracking

#### Sentry Integration

**Location**: `ca_lib.sentry/`

```python
from ca_lib.sentry import bootstrap
```

**Provides**:

- Sentry SDK initialization
- Error tracking integration
- Request tracing

#### Logging

**Location**: `ca_lib.logs/`

```python
from ca_lib.logs.formatter import StackdriverFormatter
```

**Provides**:

- Structured logging (JSON/text)
- Stackdriver/Cloud Logging integration
- Log correlation

### Utility Modules

#### Format Utilities

**Location**: `ca_lib.format_utils`

```python
from ca_lib import format_utils
```

**Provides**:

- Phone number formatting
- Address formatting
- Name normalization

#### Identifier Utilities

**Location**: `ca_lib.identifier_utils/`

**Provides**:

- NPI validation and checksum
- MBI (Medicare Beneficiary Identifier) handling
- SSN validation

#### Storage Utilities

**Location**: `ca_lib.storage_utils/`

**Provides**:

- Google Cloud Storage utilities
- S3-compatible storage
- File upload/download helpers

#### Contract Utilities

**Location**: `ca_lib.contract_utils/`

**Provides**:

- Provider contract management
- Contract validation logic
- Effective date calculations

#### Roles and Permissions

**Location**: `ca_lib.roles/`

```python
from ca_lib.ca_lib_protocols import roles_pb2
```

**Provides**:

- Role-based access control (RBAC)
- Permission validation
- User access profiles

### Protocol Buffers

**Location**: `ca_lib.ca_lib_protocols/`

```python
from ca_lib.ca_lib_protocols import constants_pb2
from ca_lib.ca_lib_protocols import roles_pb2
```

**Provides**:

- Shared protobuf message definitions
- Common enums (EHR systems, visit types, etc.)
- Cross-service contracts

**See also**: [GRPC.md - Protocol Buffer Imports](GRPC.md#protocol-buffer-imports)

### Static Class Pattern

**Location**: `ca_lib.staticclass`

```python
from ca_lib import staticclass

@staticclass.staticclass
class MyStaticClass:
    @staticmethod
    def method():
        return "no instance needed"
```

**Provides**:

- Decorator to prevent class instantiation
- Forces all methods to be static
- Used by `CloverDatetime` and utility classes

## Service Categories

### Core Infrastructure

- `ca-lib` - Shared utilities
- `ca-messaging` - Pub/sub infrastructure
- `ca-analytics-platform` - Metrics and analytics

### User and Authentication

- `ca-user-service` - User management, authentication, cohorts, Okta integration
- `clover-services-authentication` - Authentication framework

### Clinical Data

- `ca-patient-data-service` - Patient demographics and data
- `ca-clinical-data-service` - Clinical observations and records
- `ca-diagnosis-suspecting-service` - Diagnosis prediction
- `ca-care-quality-service` - Quality measures and HEDIS tracking

### Workflow and Operations

- `ca-task-service` - Task assignment and workflow
- `ca-chart-service` - Medical chart management
- `ca-recommender-framework` - Healthcare recommendations

### Data Standards

- `clover-fhir` - FHIR protocol definitions and utilities

## Cross-Service Dependencies

### Dependency Graph

```
ca-lib (foundation)
├── clover-fhir
├── ca-messaging
├── ca-analytics-platform
└── All microservices

ca-user-service
├── Provides: JWT tokens, user data, cohorts
└── Used by: All services requiring authentication

ca-patient-data-service
├── Provides: Patient demographics, enrollment
└── Used by: Clinical services, chart service

ca-clinical-data-service
├── Provides: Labs, vitals, observations
└── Used by: Care quality, chart service

ca-messaging
├── Provides: Pub/sub event bus
└── Used by: All services for async communication
```

### Import Pattern

**Services import from ca-lib**:

```python
# Good: Import shared utilities from ca-lib
from ca_lib.grpc.decorators import ca_user_service_authenticated
from ca_lib.djangolib.models import base
from ca_lib import clover_datetime

# Bad: Don't import directly from other services
# from ca_user_service import auth  # Cross-service import
```

**Services communicate via gRPC**:

```python
# Good: Use gRPC clients for cross-service calls
from ca_user_service_protocols import user_management_service_pb2_grpc

client = user_management_service_pb2_grpc.UserManagementServiceStub(channel)
response = client.GetUser(request)

# Bad: Direct Python imports across services
# from ca_user_service.user_management import models  # WRONG
```

### Shared Protocols

All services reference protobuf definitions from:

- `ca_lib/ca_lib_protocols/` - Common enums, constants
- `ca_user_service_protocols/` - User service contracts
- `clover-fhir/` - FHIR resource definitions
- Individual service `_protocols/` directories

## Dependency Management

### uv Workspace Mode

All workspace members share a single `uv.lock` file:

```bash
# Add dependency to a specific service
cd ca-user-service
uv add django

# Updates root uv.lock with resolved versions
# Other services get compatible versions automatically
```

### Installing Dependencies

```bash
# Install all workspace dependencies
cd /Users/jacobyoung/code/ca
uv sync

# Install specific service
uv sync --package ca-user-service

# Install with all extras
uv sync --all-extras
```

### Cross-Service Version Constraints

When services depend on each other:

```toml
# In ca-task-service/pyproject.toml
[project]
dependencies = [
    "ca-lib",  # Workspace dependency
    "ca-user-service-protocols",  # Cross-service protocol
]

[tool.uv.sources]
ca-lib = { workspace = true }
ca-user-service-protocols = { workspace = true }
```

## Protobuf Cross-References

Services reference proto files from other workspace members:

```bash
# In ca-task-service/Makefile
PROTO_INCLUDES=-I../ca-messaging -I../ca-lib -I../ca-user-service

protoc:
 python -m grpc_tools.protoc \
   -I=. $(PROTO_INCLUDES) \
   --python_out=. \
   ca_task_service_protocols/*.proto
```

**References**:

- `ca-lib/ca_lib/ca_lib_protocols/` - Common enums
- `ca-messaging/ca_messaging/messages/` - Messaging schemas
- Service-specific `_protocols/` directories

## Development Workflows

### Working Across Services

```bash
# Make changes to ca-lib
cd ca-lib
# Edit files
make protoc  # Regenerate if proto changed

# Update dependent service
cd ../ca-user-service
uv sync  # Gets updated ca-lib
make test  # Verify compatibility
```

### Publishing Development Builds

When developing cross-service features:

```bash
# Publish ca-lib dev version
cd ca-lib
make development_build
# Version: 1.2.3.dev42+abc123 (copied to clipboard)

# Use in other service
cd ../ca-task-service
# Update pyproject.toml with dev version
uv sync
```

## Monorepo Commands

### Root-Level Commands

```bash
# From /Users/jacobyoung/code/ca/

# Sync all workspace dependencies
uv sync

# Run tests for all services (not recommended - use per-service)
# pytest

# Lint all workspace members
ruff check .

# Format all workspace members
ruff format .
```

### Per-Service Commands

**Always use service-specific Makefiles:**

```bash
cd ca-user-service
make setup
make test
make run
```

**See also**: [MAKEFILE.md](MAKEFILE.md) for standard targets.
