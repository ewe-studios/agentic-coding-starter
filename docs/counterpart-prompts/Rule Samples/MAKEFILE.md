# Makefile Standards

**Standard Makefile targets for the CA monorepo.**

All Python services follow these standard Makefile patterns for consistent development workflows.

## Table of Contents

- [Setup and Installation](#setup-and-installation)
- [Testing and Validation](#testing-and-validation)
- [Development Server](#development-server)
- [Code Generation](#code-generation)
- [Database Operations](#database-operations)
- [Data Management](#data-management)
- [Build and Deploy](#build-and-deploy)
- [Docker Operations](#docker-operations)

## Setup and Installation

### `make setup`

**Full development environment setup** - runs all steps needed to start development.

```bash
make setup
```

**What it does**:

1. Checks for Mac dependencies (buf, protoc)
2. Runs `uv sync` to install Python dependencies
3. Sets up database with required extensions
4. Creates database (if needed)
5. Runs pre-deploy hooks (migrations, etc.)

**Use when**: First time setting up the repository or after major changes.

### `make dependencies`

**Install dependencies only** - no database setup.

```bash
make dependencies
```

**What it does**:

1. Checks Mac dependencies
2. Runs `uv sync` for Python packages
3. Installs publishing tools (twine, keyring)

**Use when**: Updating dependencies without touching database.

### `make pre_run`

**Minimal setup to run locally** - faster than full setup.

```bash
make pre_run
```

**What it does**:

1. Runs `uv sync`
2. Generates protobuf code
3. Runs pre-deploy hooks

**Use when**: Quick restart without database reset.

### `make pyenv`

**Setup Python virtual environment only.**

```bash
make pyenv
```

**What it does**: Creates `.venv` directory using `uv venv`.

## Testing and Validation

### `make test`

**Run full test suite with coverage.**

```bash
make test
```

**What it does**:

1. Generates protobuf code
2. Runs `pytest` with parallel execution (`-n auto`)
3. Generates coverage report
4. **Requires 100% coverage to pass**

**Flags**:

```bash
# Run specific test file
pytest ca_lib/tests/test_utils.py

# Run specific test function
pytest ca_lib/tests/test_utils.py::test_function_name

# Run with coverage
make test
```

### `make validate`

**Run all code validation checks.**

```bash
make validate
```

**What it does**:

1. `buf lint` - Lint protocol buffer files
2. `buf breaking` - Check for breaking protobuf changes
3. `buf format` - Check protobuf formatting
4. `ruff check` - Lint Python code
5. `ruff format --check` - Check Python formatting
6. `mypy` - Type checking

**Use when**: Before committing code or in CI/CD.

**Exit codes**:

- 0: All checks passed
- Non-zero: At least one check failed

### `make test_migrations`

**Verify Django migrations are consistent.**

```bash
make test_migrations
```

**What it does**: Runs `python manage.py makemigrations --check --no-input`.

**Use when**: Before committing Django model changes.

## Development Server

### `make run`

**Run gRPC development server.**

```bash
make run

# With Vault integration
OKTA_USERNAME=your.email@example.com make run
```

**What it does**:

- Starts gRPC server with Django
- Optionally integrates with Vault for secrets
- Loads environment variables

**Use when**: Running service locally for development.

### `make watch`

**Run server with auto-restart on file changes.**

```bash
make watch

# With Vault
OKTA_USERNAME=your.email@example.com make watch
```

**What it does**:

- Starts gRPC server with file watching
- Automatically restarts on `.py` file changes
- Uses `watchmedo` from watchdog

**Use when**: Active development with frequent code changes.

### `make consume_watch`

**Run pub/sub consumer with auto-restart.**

```bash
make consume_watch
```

**What it does**: Starts pub/sub message consumer with file watching.

**Use when**: Developing services that consume pub/sub messages.

## Code Generation

### `make protoc`

**Generate protobuf/gRPC code from `.proto` files.**

```bash
make protoc
```

**What it does**:

1. Runs `grpc_tools.protoc` compiler
2. Generates `*_pb2.py` (message definitions)
3. Generates `*_pb2_grpc.py` (service stubs)
4. Generates `*_pb2.pyi` (mypy type stubs)

**Use when**: After modifying `.proto` files.

**Example**:

```bash
# Edit auth_service.proto
vim ca_user_service_protocols/auth_service.proto

# Regenerate code
make protoc

# Commit generated files
git add ca_user_service_protocols/*_pb2*.py*
git commit -m "EI-123 update auth service protobuf"
```

### `make clean`

**Clean auto-generated files.**

```bash
make clean
```

**What it does**:

- Removes `build/`, `dist/`, `*.egg*` directories
- Deletes `__pycache__` directories
- Deletes generated protobuf files (`*_pb2*.py`, `*_pb2*.pyi`)
- Removes `.coverage` file

**Use when**: Before regenerating protobuf code or troubleshooting build issues.

## Database Operations

### `make db_env_setup`

**Setup database and run pre-deploy hooks.**

```bash
make db_env_setup
```

**What it does**:

1. Creates PostgreSQL extensions (`btree_gist`, `uuid-ossp`)
2. Creates database (if doesn't exist)
3. Runs pre-deploy hooks (migrations)

**Use when**: Initial database setup or after dropping database.

### `make drop_db`

**Drop the database.**

```bash
make drop_db
```

**What it does**: Runs `DROP DATABASE IF EXISTS` on PostgreSQL.

**⚠️ Warning**: Deletes all data. Use only in development.

**Typical workflow**:

```bash
make drop_db
make db_env_setup
make fake_data
```

## Data Management

### `make fake_data`

**Setup fake data for development.**

```bash
make fake_data
```

**What it does**:

1. Runs `python manage.py setup_fake_data`
2. Runs `python manage.py setup_v2_fake_data` (if applicable)

**Use when**: Populating development database with test data.

**Example**:

```bash
# Fresh database with test data
make drop_db
make db_env_setup
make fake_data
```

### `make fake_data_qa_automated_test`

**Setup users for automated testing.**

```bash
make fake_data_qa_automated_test
```

**What it does**: Creates deterministic test users for QA automation.

**Use when**: Setting up QA or staging environments.

### `make release_flags`

**Release all feature flags locally.**

```bash
make release_flags
```

**What it does**: Runs `python manage.py release_all_feature_flags_in_dev`.

**Use when**: Testing features behind feature flags in development.

## Build and Deploy

### `make development_build`

**Build and push development package to Google Artifact Registry.**

```bash
make development_build
```

**What it does**:

1. Cleans generated files
2. Regenerates protobuf code
3. Builds wheel with dev version: `VERSION.devN+GITSHA`
4. Uploads to GAR
5. (macOS only) Copies version to clipboard

**Use when**: Publishing development version for staging or cross-repo development.

**Version format**: `1.2.3.dev42+abc123`

### `make e2e_development_build`

**Build and push end-to-end test version.**

```bash
make e2e_development_build
```

**What it does**: Same as `development_build` but with e2e version format.

**Version format**: `1.2.3+abc123.e2e`

### `make pre_deploy`

**Update dependencies and protoc before deployment.**

```bash
make pre_deploy
```

**What it does**:

1. Runs `uv sync`
2. Generates protobuf code
3. Runs pre-deploy hook script

**Use when**: In deployment scripts before running service.

### `make post_deploy`

**Post-deployment tasks.**

```bash
make post_deploy
```

**What it does**: Runs post-deploy hook script (data migrations, cache warming, etc.).

**Use when**: After service deployment to initialize runtime state.

## Docker Operations

### `make docker`

**Build Docker image.**

```bash
make docker
```

**What it does**: Builds final Docker image tagged as `PACKAGE_NAME:latest`.

### `make trivy`

**Scan Docker image for vulnerabilities.**

```bash
make trivy
```

**What it does**:

1. Builds Docker image
2. Runs Trivy security scanner
3. Shows vulnerability age statistics
4. Displays image size

**Use when**: Before pushing Docker images to production.

### `make docker_test`

**Build and run test Docker image.**

```bash
make docker_test
```

**What it does**: Builds test image and runs tests inside container.

**Use when**: Testing Docker build in CI/CD.

### `make docker_serve`

**Run Docker container locally.**

```bash
make docker_serve
```

**What it does**: Starts Docker container for local testing.

## Performance Testing

### `make perf_test`

**Run performance tests with baseline comparison.**

```bash
make perf_test
```

**What it does**:

1. Runs tests marked with `@pytest.mark.basic_perf_test`
2. Runs tests marked with `@pytest.mark.overall_perf_test`
3. Compares against saved baseline
4. Fails if performance regresses > 30% (basic) or > 80% (overall)

**Use when**: Validating performance hasn't regressed.

### `make save_perf_test_baseline`

**Save new performance baseline.**

```bash
make save_perf_test_baseline
```

**What it does**: Runs performance tests and saves results as new baseline.

**Use when**: After intentional performance improvements or changes.

## LLM Evaluation

### `make llm_eval`

**Run LLM evaluation tests (if present).**

```bash
make llm_eval
```

**What it does**: Discovers and runs `eval_*.py` files for LLM model evaluation.

**Use when**: Testing LLM-powered features.

## Common Workflows

### First-Time Setup

```bash
make setup
make fake_data
make release_flags
make run
```

### Daily Development

```bash
# Update dependencies
make dependencies

# Start dev server with auto-reload
make watch
```

### Before Committing

```bash
# Run validation
make validate

# Run tests
make test

# Check migrations (Django services)
make test_migrations
```

### After Modifying Protos

```bash
make protoc
git add *_protocols/*_pb2*.py*
git commit -m "EI-123 update protobuf definitions"
```

### Fresh Database

```bash
make drop_db
make db_env_setup
make fake_data
make release_flags
```

### Publishing Development Version

```bash
make development_build
# Version copied to clipboard (macOS)
# Use in other services' pyproject.toml
```

## Version Management

### `make version`

**Display next version number.**

```bash
make version
```

**What it does**: Calculates version from git tags (increments patch).

### `make dev_version`

**Display development version.**

```bash
make dev_version
```

**Format**: `VERSION.devN+GITSHA`

### `make e2e_version`

**Display e2e version.**

```bash
make e2e_version
```

**Format**: `VERSION+GITSHA.e2e`

## Environment Variables

Common environment variables used by Makefile targets:

```bash
# Database
DATABASE_URL=postgresql://localhost:5432/ca_user_service

# Vault integration
OKTA_USERNAME=your.email@example.com

# Google Artifact Registry
GAR_URL=https://us-python.pkg.dev/clover-sre-001/clover-production/

# Python version
PYTHON_VERSION=3.12.8
```

## Target Dependencies

Understanding target dependencies helps optimize workflows:

```
setup
├── mac_dependencies
├── sync (uv sync)
└── db_env_setup
    └── protoc

test
└── protoc

validate
└── protoc

run
└── (no dependencies - uses existing environment)

watch
└── (no dependencies - uses existing environment)

development_build
├── clean
└── protoc
```
