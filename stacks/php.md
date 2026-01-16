# PHP Coding Standards

## Overview
- **Language**: PHP 8.3+ (prefer latest stable 8.4+)
- **Use Cases**: Web applications, REST APIs, GraphQL APIs, CLI tools, backend services, CMS systems
- **Official Docs**: https://www.php.net/docs.php
- **PHP: The Right Way**: https://phptherightway.com/

## Setup and Tools

### Required Tools
- **PHP**: 8.3+ or 8.4+ (latest stable recommended)
- **Composer**: Dependency manager (2.0+)
- **PHPStan**: Static analysis tool (Level 9 recommended)
- **Psalm**: Alternative static analysis tool
- **PHP_CodeSniffer**: Code style checker (PSR-12)
- **PHP-CS-Fixer**: Automatic code formatter
- **PHPUnit**: Testing framework (10.0+)
- **Pest**: Modern testing alternative (optional)
- **Xdebug**: Debugger and coverage tool

### Installation
```bash
# Install PHP 8.3+ (Ubuntu/Debian)
sudo apt update
sudo apt install php8.3 php8.3-cli php8.3-fpm php8.3-mbstring php8.3-xml php8.3-curl php8.3-zip

# Install Composer
curl -sS https://getcomposer.org/installer | php
sudo mv composer.phar /usr/local/bin/composer

# Install development tools
composer require --dev phpstan/phpstan
composer require --dev vimeo/psalm
composer require --dev phpunit/phpunit
composer require --dev squizlabs/php_codesniffer
composer require --dev friendsofphp/php-cs-fixer
composer require --dev pestphp/pest

# Install Xdebug
sudo apt install php8.3-xdebug
```

### Configuration Files

#### **composer.json** - Project Configuration
```json
{
    "name": "vendor/project-name",
    "description": "Project description",
    "type": "project",
    "license": "MIT",
    "require": {
        "php": "^8.3",
        "ext-mbstring": "*",
        "ext-json": "*"
    },
    "require-dev": {
        "phpunit/phpunit": "^10.0",
        "phpstan/phpstan": "^1.10",
        "vimeo/psalm": "^5.0",
        "squizlabs/php_codesniffer": "^3.7",
        "friendsofphp/php-cs-fixer": "^3.0"
    },
    "autoload": {
        "psr-4": {
            "App\\": "app/"
        },
        "files": [
            "app/helpers.php"
        ]
    },
    "autoload-dev": {
        "psr-4": {
            "Tests\\": "tests/"
        }
    },
    "config": {
        "optimize-autoloader": true,
        "preferred-install": "dist",
        "sort-packages": true,
        "allow-plugins": {
            "pestphp/pest-plugin": true
        }
    },
    "scripts": {
        "test": "phpunit",
        "test:coverage": "phpunit --coverage-html coverage",
        "phpstan": "phpstan analyse",
        "psalm": "psalm",
        "cs:check": "php-cs-fixer fix --dry-run --diff",
        "cs:fix": "php-cs-fixer fix",
        "lint": "phpcs",
        "lint:fix": "phpcbf"
    },
    "minimum-stability": "stable",
    "prefer-stable": true
}
```

#### **phpstan.neon** - PHPStan Configuration
```neon
parameters:
    level: 9
    paths:
        - app
        - tests
    excludePaths:
        - vendor
    checkMissingIterableValueType: true
    checkGenericClassInNonGenericObjectType: true
    reportUnmatchedIgnoredErrors: true
    treatPhpDocTypesAsCertain: false
    ignoreErrors:
        # Add specific ignores here if absolutely necessary
```

#### **psalm.xml** - Psalm Configuration
```xml
<?xml version="1.0"?>
<psalm
    errorLevel="1"
    resolveFromConfigFile="true"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xmlns="https://getpsalm.org/schema/config"
    xsi:schemaLocation="https://getpsalm.org/schema/config vendor/vimeo/psalm/config.xsd"
>
    <projectFiles>
        <directory name="app" />
        <directory name="tests" />
        <ignoreFiles>
            <directory name="vendor" />
        </ignoreFiles>
    </projectFiles>
    <issueHandlers>
        <MissingReturnType errorLevel="error" />
        <MissingParamType errorLevel="error" />
        <MissingPropertyType errorLevel="error" />
    </issueHandlers>
</psalm>
```

#### **.php-cs-fixer.php** - Code Style Configuration
```php
<?php

declare(strict_types=1);

$finder = PhpCsFixer\Finder::create()
    ->in(__DIR__)
    ->exclude('vendor')
    ->exclude('storage')
    ->exclude('bootstrap/cache');

return (new PhpCsFixer\Config())
    ->setRules([
        '@PSR12' => true,
        'strict_param' => true,
        'array_syntax' => ['syntax' => 'short'],
        'ordered_imports' => ['sort_algorithm' => 'alpha'],
        'no_unused_imports' => true,
        'not_operator_with_successor_space' => true,
        'trailing_comma_in_multiline' => true,
        'phpdoc_scalar' => true,
        'unary_operator_spaces' => true,
        'binary_operator_spaces' => true,
        'blank_line_before_statement' => [
            'statements' => ['break', 'continue', 'declare', 'return', 'throw', 'try'],
        ],
        'phpdoc_single_line_var_spacing' => true,
        'phpdoc_var_without_name' => true,
        'single_trait_insert_per_statement' => true,
        'declare_strict_types' => true,
    ])
    ->setFinder($finder);
```

#### **phpunit.xml** - PHPUnit Configuration
```xml
<?xml version="1.0" encoding="UTF-8"?>
<phpunit xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:noNamespaceSchemaLocation="vendor/phpunit/phpunit/phpunit.xsd"
         bootstrap="vendor/autoload.php"
         colors="true"
         failOnRisky="true"
         failOnWarning="true"
         stopOnFailure="false">
    <testsuites>
        <testsuite name="Unit">
            <directory>tests/Unit</directory>
        </testsuite>
        <testsuite name="Feature">
            <directory>tests/Feature</directory>
        </testsuite>
    </testsuites>
    <source>
        <include>
            <directory suffix=".php">app</directory>
        </include>
    </source>
    <coverage>
        <report>
            <html outputDirectory="coverage"/>
            <text outputFile="php://stdout" showUncoveredFiles="false"/>
        </report>
    </coverage>
    <php>
        <env name="APP_ENV" value="testing"/>
    </php>
</phpunit>
```

## Coding Standards

### Naming Conventions (PSR-1, PSR-12)

#### Classes and Namespaces
- **Classes**: PascalCase (StudlyCase)
- **Interfaces**: PascalCase with `Interface` suffix (optional but recommended)
- **Traits**: PascalCase with `Trait` suffix (optional but recommended)
- **Enums**: PascalCase
- **Namespaces**: PascalCase following PSR-4
- **Abstract Classes**: PascalCase, optionally prefix with `Abstract`

```php
<?php

declare(strict_types=1);

namespace App\Services\User;

use App\Contracts\UserRepositoryInterface;
use App\Traits\LoggableTrait;

abstract class AbstractUserService
{
    // ...
}

final class UserService extends AbstractUserService
{
    use LoggableTrait;

    public function __construct(
        private readonly UserRepositoryInterface $repository
    ) {}
}

interface CacheableInterface
{
    public function getCacheKey(): string;
}

enum UserStatus: string
{
    case Active = 'active';
    case Inactive = 'inactive';
    case Suspended = 'suspended';
}
```

#### Methods and Properties
- **Methods**: camelCase
- **Properties**: camelCase
- **Constants**: UPPER_SNAKE_CASE
- **Variables**: camelCase or snake_case (be consistent)

```php
<?php

declare(strict_types=1);

namespace App\Models;

final class User
{
    private const MAX_LOGIN_ATTEMPTS = 5;
    private const DEFAULT_ROLE = 'user';

    private string $firstName;
    private string $lastName;
    private int $loginAttempts = 0;

    public function getFullName(): string
    {
        return "{$this->firstName} {$this->lastName}";
    }

    public function incrementLoginAttempts(): void
    {
        $this->loginAttempts++;
    }

    public function hasExceededMaxAttempts(): bool
    {
        return $this->loginAttempts >= self::MAX_LOGIN_ATTEMPTS;
    }
}
```

#### Files
- **Classes**: One class per file, match class name
- **File names**: Match class name exactly (case-sensitive)
- **Test files**: `*Test.php` suffix

```
src/
├── Models/
│   ├── User.php
│   └── Post.php
├── Services/
│   ├── UserService.php
│   └── AuthenticationService.php
└── Exceptions/
    └── UserNotFoundException.php

tests/
├── Unit/
│   ├── UserTest.php
│   └── UserServiceTest.php
└── Feature/
    └── AuthenticationTest.php
```

### Code Organization

#### Project Structure (MVC Pattern)
```
project/
├── .php-cs-fixer.php
├── composer.json
├── phpstan.neon
├── phpunit.xml
├── psalm.xml
├── app/
│   ├── Controllers/
│   │   ├── UserController.php
│   │   └── AuthController.php
│   ├── Models/
│   │   ├── User.php
│   │   ├── Post.php
│   │   ├── DTOs/                    # DTOs right beside Models
│   │   │   ├── CreateUserDTO.php
│   │   │   ├── UpdateUserDTO.php
│   │   │   └── UserDTO.php
│   │   └── Enums/                   # Enums with Models
│   │       ├── UserStatus.php
│   │       └── UserRole.php
│   ├── Services/
│   │   ├── UserService.php
│   │   ├── AuthService.php
│   │   └── EmailService.php
│   ├── Repositories/
│   │   ├── UserRepository.php
│   │   ├── PostRepository.php
│   │   └── Contracts/               # Repository interfaces
│   │       ├── UserRepositoryInterface.php
│   │       └── PostRepositoryInterface.php
│   ├── Middleware/
│   │   ├── AuthMiddleware.php
│   │   └── CorsMiddleware.php
│   ├── Exceptions/
│   │   ├── UserNotFoundException.php
│   │   ├── ValidationException.php
│   │   └── AuthenticationException.php
│   ├── Traits/
│   │   ├── Timestampable.php
│   │   └── SoftDeletes.php
│   ├── Validators/
│   │   ├── UserValidator.php
│   │   └── PostValidator.php
│   └── helpers.php
├── tests/
│   ├── Unit/
│   │   ├── Models/
│   │   │   ├── UserTest.php
│   │   │   └── PostTest.php
│   │   ├── Services/                # Tests mirror service directory structure
│   │   │   ├── UserServiceTest.php
│   │   │   ├── AuthServiceTest.php
│   │   │   └── EmailServiceTest.php
│   │   └── Repositories/
│   │       ├── UserRepositoryTest.php
│   │       └── PostRepositoryTest.php
│   └── Feature/
│       ├── UserRegistrationTest.php
│       ├── UserAuthenticationTest.php
│       └── PostCreationTest.php
├── config/
│   ├── app.php
│   ├── database.php
│   └── services.php
├── routes/
│   ├── web.php
│   └── api.php
├── views/
│   ├── layouts/
│   │   └── app.php
│   └── users/
│       ├── index.php
│       └── show.php
├── public/
│   ├── index.php
│   ├── css/
│   └── js/
└── storage/
    ├── logs/
    └── cache/
```

**Key Structure Principles**:
- DTOs live in `Models/DTOs/` - they represent model data shapes
- Enums live in `Models/Enums/` - they represent model state/type values
- Unit tests mirror the exact directory structure of what they test
  - `Services/UserService.php` → `tests/Unit/Services/UserServiceTest.php`
  - `Repositories/UserRepository.php` → `tests/Unit/Repositories/UserRepositoryTest.php`
- Test class names match the file they test with `Test` suffix

### Strict Types (MANDATORY)

**ALWAYS** declare strict types at the top of every PHP file:

```php
<?php

declare(strict_types=1);

namespace App\Services;

// Your code here
```

**Why?**
- Prevents type coercion bugs
- Enforces type safety
- Makes code more predictable
- Required for PHPStan level 9

### Type Hints (MANDATORY)

**ALWAYS** use type hints for:
- Function parameters
- Return types
- Properties (PHP 7.4+)

```php
<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\User;
use App\Exceptions\UserNotFoundException;

final class UserService
{
    public function __construct(
        private readonly UserRepository $repository,
        private readonly Logger $logger
    ) {}

    /**
     * Find a user by ID.
     *
     * @throws UserNotFoundException
     */
    public function findById(int $userId): User
    {
        $user = $this->repository->find($userId);

        if ($user === null) {
            throw new UserNotFoundException("User {$userId} not found");
        }

        return $user;
    }

    /**
     * Get all active users.
     *
     * @return array<int, User>
     */
    public function getActiveUsers(): array
    {
        return $this->repository->findByStatus(UserStatus::Active);
    }

    /**
     * Update user email.
     */
    public function updateEmail(int $userId, string $email): bool
    {
        $user = $this->findById($userId);
        $user->setEmail($email);

        return $this->repository->save($user);
    }
}
```

### Documentation Standards

#### PHPDoc Comments
- **MANDATORY** for all public methods, properties, and classes
- Use for complex logic explanation
- Include `@param`, `@return`, `@throws` tags
- Use type annotations for arrays and generics

```php
<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\User;
use App\DTOs\CreateUserDTO;
use App\Exceptions\ValidationException;
use App\Exceptions\DuplicateEmailException;

/**
 * Service for managing user operations.
 *
 * Handles user creation, retrieval, updates, and deletion
 * with proper validation and authorization checks.
 */
final class UserService
{
    /**
     * Create a new user from DTO.
     *
     * @param CreateUserDTO $dto User creation data
     * @return User The created user instance
     * @throws ValidationException When validation fails
     * @throws DuplicateEmailException When email already exists
     */
    public function create(CreateUserDTO $dto): User
    {
        $this->validateEmail($dto->email);

        if ($this->emailExists($dto->email)) {
            throw new DuplicateEmailException(
                "Email {$dto->email} already exists"
            );
        }

        return $this->repository->create([
            'name' => $dto->name,
            'email' => $dto->email,
            'password' => $this->hashPassword($dto->password),
        ]);
    }

    /**
     * Get users by status.
     *
     * @param UserStatus $status Status to filter by
     * @return list<User> List of users with given status
     */
    public function getUsersByStatus(UserStatus $status): array
    {
        return $this->repository->findByStatus($status);
    }

    /**
     * Get user metadata.
     *
     * @param User $user User instance
     * @return array<string, mixed> Metadata key-value pairs
     */
    private function getUserMetadata(User $user): array
    {
        return [
            'last_login' => $user->getLastLogin(),
            'login_count' => $user->getLoginCount(),
            'preferences' => $user->getPreferences(),
        ];
    }
}
```

## Best Practices

### Modern PHP Features (PHP 8.3+)

#### Constructor Property Promotion
```php
<?php

declare(strict_types=1);

namespace App\Services;

// OLD WAY (verbose)
class OldUserService
{
    private UserRepository $repository;
    private Logger $logger;

    public function __construct(UserRepository $repository, Logger $logger)
    {
        $this->repository = $repository;
        $this->logger = $logger;
    }
}

// NEW WAY (PHP 8.0+) - PREFER THIS
final class UserService
{
    public function __construct(
        private readonly UserRepository $repository,
        private readonly Logger $logger
    ) {}
}
```

#### Readonly Properties (PHP 8.1+)
```php
<?php

declare(strict_types=1);

namespace App\DTOs;

// Immutable DTO with readonly properties
final class CreateUserDTO
{
    public function __construct(
        public readonly string $name,
        public readonly string $email,
        public readonly string $password
    ) {}
}

// Can also use on entire class (PHP 8.2+)
readonly final class UserDTO
{
    public function __construct(
        public int $id,
        public string $name,
        public string $email
    ) {}
}
```

#### Enums (PHP 8.1+)
```php
<?php

declare(strict_types=1);

namespace App\Models\Enums;

// Backed enum with string values - located in Models/Enums
enum UserStatus: string
{
    case Active = 'active';
    case Inactive = 'inactive';
    case Suspended = 'suspended';
    case Deleted = 'deleted';

    /**
     * Get display name for status.
     */
    public function label(): string
    {
        return match ($this) {
            self::Active => 'Active User',
            self::Inactive => 'Inactive',
            self::Suspended => 'Suspended',
            self::Deleted => 'Deleted',
        };
    }

    /**
     * Check if user can login.
     */
    public function canLogin(): bool
    {
        return $this === self::Active;
    }

    /**
     * Get all statuses that allow login.
     *
     * @return array<UserStatus>
     */
    public static function loginableStatuses(): array
    {
        return [self::Active];
    }
}

// Usage
$status = UserStatus::Active;
echo $status->value; // 'active'
echo $status->label(); // 'Active User'

if ($status->canLogin()) {
    // Allow login
}
```

#### Match Expressions (PHP 8.0+)
```php
<?php

declare(strict_types=1);

// OLD: switch statement
function getStatusColorOld(string $status): string
{
    switch ($status) {
        case 'active':
            return 'green';
        case 'inactive':
            return 'gray';
        case 'suspended':
            return 'orange';
        case 'deleted':
            return 'red';
        default:
            throw new InvalidArgumentException("Unknown status: {$status}");
    }
}

// NEW: match expression (exhaustive, returns value)
function getStatusColor(UserStatus $status): string
{
    return match ($status) {
        UserStatus::Active => 'green',
        UserStatus::Inactive => 'gray',
        UserStatus::Suspended => 'orange',
        UserStatus::Deleted => 'red',
    };
}
```

#### Named Arguments (PHP 8.0+)
```php
<?php

declare(strict_types=1);

function createUser(
    string $name,
    string $email,
    string $password,
    bool $isAdmin = false,
    bool $isVerified = false
): User {
    // Implementation
}

// OLD: positional arguments (hard to read)
$user = createUser('John', 'john@example.com', 'secret123', false, true);

// NEW: named arguments (clear and readable)
$user = createUser(
    name: 'John',
    email: 'john@example.com',
    password: 'secret123',
    isVerified: true
);
```

#### Nullsafe Operator (PHP 8.0+)
```php
<?php

declare(strict_types=1);

// OLD: verbose null checks
$country = null;
if ($user !== null) {
    $address = $user->getAddress();
    if ($address !== null) {
        $country = $address->getCountry();
    }
}

// NEW: nullsafe operator
$country = $user?->getAddress()?->getCountry();
```

#### Union Types (PHP 8.0+)
```php
<?php

declare(strict_types=1);

namespace App\Services;

final class ResponseFormatter
{
    /**
     * Format response data.
     *
     * @param array<string, mixed>|string|null $data
     */
    public function format(array|string|null $data): string
    {
        return match (true) {
            is_array($data) => json_encode($data),
            is_string($data) => $data,
            default => '',
        };
    }
}
```

#### Intersection Types (PHP 8.1+)
```php
<?php

declare(strict_types=1);

namespace App\Contracts;

interface Loggable
{
    public function getLogContext(): array;
}

interface Cacheable
{
    public function getCacheKey(): string;
}

final class Service
{
    /**
     * Process an entity that is both loggable and cacheable.
     */
    public function process(Loggable&Cacheable $entity): void
    {
        $this->logger->info('Processing', $entity->getLogContext());
        $this->cache->set($entity->getCacheKey(), $entity);
    }
}
```

#### Attributes (PHP 8.0+)
```php
<?php

declare(strict_types=1);

namespace App\Attributes;

use Attribute;

#[Attribute(Attribute::TARGET_CLASS)]
final class Route
{
    public function __construct(
        public readonly string $path,
        public readonly string $method = 'GET'
    ) {}
}

#[Attribute(Attribute::TARGET_METHOD)]
final class Cache
{
    public function __construct(
        public readonly int $ttl = 3600
    ) {}
}

// Usage
#[Route('/users', 'GET')]
final class UserController
{
    #[Cache(ttl: 300)]
    public function index(): array
    {
        return $this->userService->getAll();
    }
}
```

### Idiomatic PHP Patterns

#### Repository Pattern
```php
<?php

declare(strict_types=1);

namespace App\Repositories;

use App\Models\User;
use App\Enums\UserStatus;

interface UserRepositoryInterface
{
    public function find(int $id): ?User;

    /**
     * @return list<User>
     */
    public function findByStatus(UserStatus $status): array;

    public function save(User $user): bool;

    public function delete(int $id): bool;
}

final class UserRepository implements UserRepositoryInterface
{
    public function __construct(
        private readonly Database $db
    ) {}

    public function find(int $id): ?User
    {
        $row = $this->db->query(
            'SELECT * FROM users WHERE id = :id',
            ['id' => $id]
        )->fetch();

        return $row ? $this->hydrate($row) : null;
    }

    public function findByStatus(UserStatus $status): array
    {
        $rows = $this->db->query(
            'SELECT * FROM users WHERE status = :status',
            ['status' => $status->value]
        )->fetchAll();

        return array_map([$this, 'hydrate'], $rows);
    }

    public function save(User $user): bool
    {
        // Implementation
        return true;
    }

    public function delete(int $id): bool
    {
        return $this->db->execute(
            'DELETE FROM users WHERE id = :id',
            ['id' => $id]
        );
    }

    private function hydrate(array $row): User
    {
        return new User(
            id: (int) $row['id'],
            name: $row['name'],
            email: $row['email'],
            status: UserStatus::from($row['status'])
        );
    }
}
```

#### Service Layer Pattern
```php
<?php

declare(strict_types=1);

namespace App\Services;

use App\DTOs\CreateUserDTO;
use App\Models\User;
use App\Repositories\UserRepositoryInterface;
use App\Exceptions\ValidationException;
use App\Exceptions\UserNotFoundException;

final class UserService
{
    public function __construct(
        private readonly UserRepositoryInterface $repository,
        private readonly PasswordHasher $hasher,
        private readonly Validator $validator
    ) {}

    /**
     * Create a new user.
     *
     * @throws ValidationException
     */
    public function createUser(CreateUserDTO $dto): User
    {
        $this->validator->validate($dto);

        $user = new User(
            id: 0, // Will be set by database
            name: $dto->name,
            email: $dto->email,
            status: UserStatus::Active
        );

        $user->setPassword($this->hasher->hash($dto->password));

        $this->repository->save($user);

        return $user;
    }

    /**
     * Get user by ID.
     *
     * @throws UserNotFoundException
     */
    public function getUser(int $id): User
    {
        $user = $this->repository->find($id);

        if ($user === null) {
            throw new UserNotFoundException("User {$id} not found");
        }

        return $user;
    }
}
```

#### Data Transfer Objects (DTOs)
```php
<?php

declare(strict_types=1);

namespace App\Models\DTOs;

use InvalidArgumentException;

// Immutable DTO with validation - located in Models/DTOs
readonly final class CreateUserDTO
{
    public function __construct(
        public string $name,
        public string $email,
        public string $password
    ) {
        $this->validate();
    }

    /**
     * Create from request array.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        return new self(
            name: $data['name'] ?? '',
            email: $data['email'] ?? '',
            password: $data['password'] ?? ''
        );
    }

    /**
     * @return array<string, string>
     */
    public function toArray(): array
    {
        return [
            'name' => $this->name,
            'email' => $this->email,
        ];
    }

    private function validate(): void
    {
        if (empty($this->name)) {
            throw new InvalidArgumentException('Name is required');
        }

        if (!filter_var($this->email, FILTER_VALIDATE_EMAIL)) {
            throw new InvalidArgumentException('Invalid email format');
        }

        if (strlen($this->password) < 8) {
            throw new InvalidArgumentException('Password must be at least 8 characters');
        }
    }
}
```

### Error Handling

#### Custom Exceptions
```php
<?php

declare(strict_types=1);

namespace App\Exceptions;

use RuntimeException;
use Throwable;

/**
 * Base application exception.
 */
abstract class ApplicationException extends RuntimeException
{
    public function __construct(
        string $message = '',
        int $code = 0,
        ?Throwable $previous = null,
        private readonly array $context = []
    ) {
        parent::__construct($message, $code, $previous);
    }

    /**
     * @return array<string, mixed>
     */
    public function getContext(): array
    {
        return $this->context;
    }
}

final class UserNotFoundException extends ApplicationException
{
    public function __construct(int $userId, ?Throwable $previous = null)
    {
        parent::__construct(
            message: "User with ID {$userId} not found",
            code: 404,
            previous: $previous,
            context: ['user_id' => $userId]
        );
    }
}

final class ValidationException extends ApplicationException
{
    /**
     * @param array<string, string> $errors
     */
    public function __construct(
        array $errors,
        ?Throwable $previous = null
    ) {
        parent::__construct(
            message: 'Validation failed',
            code: 422,
            previous: $previous,
            context: ['errors' => $errors]
        );
    }

    /**
     * @return array<string, string>
     */
    public function getErrors(): array
    {
        return $this->getContext()['errors'];
    }
}

final class AuthenticationException extends ApplicationException
{
    public function __construct(
        string $message = 'Authentication failed',
        ?Throwable $previous = null
    ) {
        parent::__construct($message, 401, $previous);
    }
}
```

#### Exception Handling Best Practices
```php
<?php

declare(strict_types=1);

namespace App\Services;

use App\Exceptions\UserNotFoundException;
use App\Exceptions\DatabaseException;
use PDOException;

final class UserService
{
    /**
     * GOOD: Specific exception types, proper context.
     */
    public function getUser(int $id): User
    {
        try {
            $user = $this->repository->find($id);

            if ($user === null) {
                throw new UserNotFoundException($id);
            }

            return $user;
        } catch (PDOException $e) {
            // Wrap infrastructure exceptions in domain exceptions
            throw new DatabaseException(
                'Failed to fetch user from database',
                previous: $e
            );
        }
    }

    /**
     * BAD: Catching generic Exception, losing context.
     */
    public function badExample(int $id): ?User
    {
        try {
            return $this->repository->find($id);
        } catch (Exception $e) {
            // Too broad, loses information
            return null;
        }
    }
}
```

### Dependency Injection

#### Constructor Injection (Preferred)
```php
<?php

declare(strict_types=1);

namespace App\Services;

use App\Repositories\UserRepositoryInterface;
use Psr\Log\LoggerInterface;

final class UserService
{
    public function __construct(
        private readonly UserRepositoryInterface $userRepository,
        private readonly LoggerInterface $logger,
        private readonly CacheInterface $cache
    ) {}

    public function getUser(int $id): User
    {
        $cacheKey = "user:{$id}";

        $user = $this->cache->get($cacheKey);
        if ($user !== null) {
            return $user;
        }

        $user = $this->userRepository->find($id);
        if ($user === null) {
            $this->logger->warning("User {$id} not found");
            throw new UserNotFoundException($id);
        }

        $this->cache->set($cacheKey, $user, 3600);

        return $user;
    }
}
```

#### Interface Segregation
```php
<?php

declare(strict_types=1);

namespace App\Contracts;

// BAD: Fat interface
interface BadUserRepository
{
    public function find(int $id): ?User;
    public function save(User $user): bool;
    public function delete(int $id): bool;
    public function sendEmail(User $user, string $subject): void; // Wrong!
    public function generateReport(User $user): string; // Wrong!
}

// GOOD: Focused interfaces
interface UserRepositoryInterface
{
    public function find(int $id): ?User;
    public function save(User $user): bool;
    public function delete(int $id): bool;
}

interface EmailSenderInterface
{
    public function send(string $to, string $subject, string $body): void;
}

interface ReportGeneratorInterface
{
    public function generate(User $user): string;
}
```

### Testing

#### PHPUnit Tests (Following Service Directory Structure)
```php
<?php

declare(strict_types=1);

namespace Tests\Unit\Services;

use App\Services\UserService;
use App\Repositories\Contracts\UserRepositoryInterface;
use App\Models\User;
use App\Models\Enums\UserStatus;
use App\Exceptions\UserNotFoundException;
use PHPUnit\Framework\TestCase;
use PHPUnit\Framework\MockObject\MockObject;

/**
 * Tests for App\Services\UserService
 *
 * Test file structure mirrors service structure:
 * - Service: app/Services/UserService.php
 * - Test: tests/Unit/Services/UserServiceTest.php
 */
final class UserServiceTest extends TestCase
{
    private UserService $service;
    private UserRepositoryInterface&MockObject $repository;

    protected function setUp(): void
    {
        $this->repository = $this->createMock(UserRepositoryInterface::class);
        $this->service = new UserService($this->repository);
    }

    public function testGetUserReturnsUserWhenExists(): void
    {
        // Arrange
        $expectedUser = new User(
            id: 1,
            name: 'John Doe',
            email: 'john@example.com',
            status: UserStatus::Active
        );

        $this->repository
            ->expects($this->once())
            ->method('find')
            ->with(1)
            ->willReturn($expectedUser);

        // Act
        $result = $this->service->getUser(1);

        // Assert
        $this->assertSame($expectedUser, $result);
    }

    public function testGetUserThrowsExceptionWhenNotFound(): void
    {
        // Arrange
        $this->repository
            ->expects($this->once())
            ->method('find')
            ->with(999)
            ->willReturn(null);

        // Assert
        $this->expectException(UserNotFoundException::class);
        $this->expectExceptionMessage('User with ID 999 not found');

        // Act
        $this->service->getUser(999);
    }

    /**
     * @dataProvider statusProvider
     */
    public function testGetUsersByStatus(UserStatus $status, int $expectedCount): void
    {
        $users = array_fill(0, $expectedCount, $this->createUser());

        $this->repository
            ->method('findByStatus')
            ->with($status)
            ->willReturn($users);

        $result = $this->service->getUsersByStatus($status);

        $this->assertCount($expectedCount, $result);
    }

    /**
     * @return array<string, array{UserStatus, int}>
     */
    public static function statusProvider(): array
    {
        return [
            'active users' => [UserStatus::Active, 5],
            'inactive users' => [UserStatus::Inactive, 3],
            'suspended users' => [UserStatus::Suspended, 0],
        ];
    }

    private function createUser(): User
    {
        return new User(
            id: 1,
            name: 'Test User',
            email: 'test@example.com',
            status: UserStatus::Active
        );
    }
}
```

**Test Naming Convention**:
```
Service File                    → Test File
app/Services/UserService.php   → tests/Unit/Services/UserServiceTest.php
app/Services/AuthService.php   → tests/Unit/Services/AuthServiceTest.php
app/Services/EmailService.php  → tests/Unit/Services/EmailServiceTest.php

Repository File                              → Test File
app/Repositories/UserRepository.php          → tests/Unit/Repositories/UserRepositoryTest.php
app/Repositories/PostRepository.php          → tests/Unit/Repositories/PostRepositoryTest.php

Model File                      → Test File
app/Models/User.php            → tests/Unit/Models/UserTest.php
app/Models/Post.php            → tests/Unit/Models/PostTest.php
```

#### Pest Tests (Alternative)
```php
<?php

declare(strict_types=1);

use App\Services\UserService;
use App\Models\User;
use App\Enums\UserStatus;
use App\Exceptions\UserNotFoundException;

beforeEach(function () {
    $this->repository = Mockery::mock(UserRepositoryInterface::class);
    $this->service = new UserService($this->repository);
});

it('returns user when exists', function () {
    $user = new User(1, 'John', 'john@example.com', UserStatus::Active);

    $this->repository
        ->shouldReceive('find')
        ->once()
        ->with(1)
        ->andReturn($user);

    $result = $this->service->getUser(1);

    expect($result)->toBe($user);
});

it('throws exception when user not found', function () {
    $this->repository
        ->shouldReceive('find')
        ->once()
        ->with(999)
        ->andReturn(null);

    $this->service->getUser(999);
})->throws(UserNotFoundException::class, 'User with ID 999 not found');

test('user status can be checked', function () {
    $user = new User(1, 'John', 'john@example.com', UserStatus::Active);

    expect($user->getStatus())->toBe(UserStatus::Active)
        ->and($user->getStatus()->canLogin())->toBeTrue();
});
```

### Performance Best Practices

#### Opcode Caching
```php
// Enable OPcache in php.ini (production)
opcache.enable=1
opcache.memory_consumption=128
opcache.interned_strings_buffer=8
opcache.max_accelerated_files=10000
opcache.revalidate_freq=2
opcache.fast_shutdown=1
```

#### Array Functions (Fast)
```php
<?php

declare(strict_types=1);

// GOOD: Use native array functions
$doubled = array_map(fn($x) => $x * 2, $numbers);
$filtered = array_filter($numbers, fn($x) => $x > 0);
$sum = array_sum($numbers);
$exists = in_array($needle, $haystack, true);

// BAD: Manual loops for simple operations
$doubled = [];
foreach ($numbers as $num) {
    $doubled[] = $num * 2;
}
```

#### Generator Functions (Memory Efficient)
```php
<?php

declare(strict_types=1);

namespace App\Services;

// BAD: Loads all users into memory
public function getAllUsers(): array
{
    return $this->repository->findAll(); // Could be millions of records!
}

// GOOD: Use generator to yield users one at a time
public function iterateUsers(): \Generator
{
    $offset = 0;
    $limit = 100;

    while (true) {
        $users = $this->repository->findWithPagination($offset, $limit);

        if (empty($users)) {
            break;
        }

        foreach ($users as $user) {
            yield $user;
        }

        $offset += $limit;
    }
}

// Usage
foreach ($userService->iterateUsers() as $user) {
    // Process one user at a time, low memory usage
    $this->processUser($user);
}
```

#### Database Best Practices
```php
<?php

declare(strict_types=1);

namespace App\Repositories;

final class UserRepository
{
    // GOOD: Use prepared statements (security + performance)
    public function find(int $id): ?User
    {
        $stmt = $this->pdo->prepare('SELECT * FROM users WHERE id = :id');
        $stmt->execute(['id' => $id]);

        return $stmt->fetch() ?: null;
    }

    // BAD: String concatenation (SQL injection risk + no caching)
    public function badFind(int $id): ?User
    {
        $query = "SELECT * FROM users WHERE id = {$id}"; // NEVER DO THIS!
        return $this->pdo->query($query)->fetch();
    }

    // GOOD: Batch operations
    public function saveMany(array $users): void
    {
        $this->pdo->beginTransaction();

        try {
            $stmt = $this->pdo->prepare(
                'INSERT INTO users (name, email) VALUES (:name, :email)'
            );

            foreach ($users as $user) {
                $stmt->execute([
                    'name' => $user->getName(),
                    'email' => $user->getEmail(),
                ]);
            }

            $this->pdo->commit();
        } catch (\Exception $e) {
            $this->pdo->rollBack();
            throw $e;
        }
    }
}
```

### Security Best Practices

#### Input Validation
```php
<?php

declare(strict_types=1);

namespace App\Validators;

final class UserValidator
{
    /**
     * Validate user input.
     *
     * @param array<string, mixed> $data
     * @return array<string, string> Validation errors
     */
    public function validate(array $data): array
    {
        $errors = [];

        // Name validation
        if (!isset($data['name']) || trim($data['name']) === '') {
            $errors['name'] = 'Name is required';
        } elseif (strlen($data['name']) > 255) {
            $errors['name'] = 'Name must not exceed 255 characters';
        }

        // Email validation
        if (!isset($data['email'])) {
            $errors['email'] = 'Email is required';
        } elseif (!filter_var($data['email'], FILTER_VALIDATE_EMAIL)) {
            $errors['email'] = 'Invalid email format';
        }

        // Password validation
        if (!isset($data['password'])) {
            $errors['password'] = 'Password is required';
        } elseif (strlen($data['password']) < 8) {
            $errors['password'] = 'Password must be at least 8 characters';
        } elseif (!preg_match('/[A-Z]/', $data['password'])) {
            $errors['password'] = 'Password must contain uppercase letter';
        } elseif (!preg_match('/[0-9]/', $data['password'])) {
            $errors['password'] = 'Password must contain number';
        }

        return $errors;
    }
}
```

#### Password Hashing
```php
<?php

declare(strict_types=1);

namespace App\Services;

final class PasswordHasher
{
    // GOOD: Use password_hash with bcrypt or argon2
    public function hash(string $password): string
    {
        return password_hash($password, PASSWORD_ARGON2ID, [
            'memory_cost' => 65536,
            'time_cost' => 4,
            'threads' => 3,
        ]);
    }

    public function verify(string $password, string $hash): bool
    {
        return password_verify($password, $hash);
    }

    public function needsRehash(string $hash): bool
    {
        return password_needs_rehash($hash, PASSWORD_ARGON2ID, [
            'memory_cost' => 65536,
            'time_cost' => 4,
            'threads' => 3,
        ]);
    }

    // BAD: Never use MD5 or SHA1 for passwords!
    public function badHash(string $password): string
    {
        return md5($password); // INSECURE!
    }
}
```

#### SQL Injection Prevention
```php
<?php

declare(strict_types=1);

// GOOD: Always use prepared statements
$stmt = $pdo->prepare('SELECT * FROM users WHERE email = :email');
$stmt->execute(['email' => $userInput]);

// GOOD: Using PDO with named parameters
$stmt = $pdo->prepare('
    INSERT INTO users (name, email, password)
    VALUES (:name, :email, :password)
');
$stmt->execute([
    'name' => $data['name'],
    'email' => $data['email'],
    'password' => $hashedPassword,
]);

// BAD: String interpolation (SQL injection vulnerability!)
$query = "SELECT * FROM users WHERE email = '{$userInput}'"; // NEVER!
$pdo->query($query);

// BAD: String concatenation
$query = "SELECT * FROM users WHERE id = " . $_GET['id']; // NEVER!
```

#### XSS Prevention
```php
<?php

declare(strict_types=1);

// GOOD: Escape output
echo htmlspecialchars($userInput, ENT_QUOTES, 'UTF-8');

// GOOD: Using template engine with auto-escaping (Twig, Blade)
// {{ user.name }} - automatically escaped

// BAD: Raw output
echo $userInput; // XSS vulnerability!

// BAD: Only escaping HTML tags
echo strip_tags($userInput); // Still vulnerable!
```

#### CSRF Protection
```php
<?php

declare(strict_types=1);

namespace App\Security;

final class CsrfTokenManager
{
    public function generateToken(): string
    {
        if (!isset($_SESSION['csrf_token'])) {
            $_SESSION['csrf_token'] = bin2hex(random_bytes(32));
        }

        return $_SESSION['csrf_token'];
    }

    public function validateToken(string $token): bool
    {
        if (!isset($_SESSION['csrf_token'])) {
            return false;
        }

        return hash_equals($_SESSION['csrf_token'], $token);
    }
}

// In forms
// <input type="hidden" name="csrf_token" value="<?= $csrfToken ?>" />

// In handlers
if (!$csrf->validateToken($_POST['csrf_token'] ?? '')) {
    throw new SecurityException('Invalid CSRF token');
}
```

## Valid Code Requirements

Code is considered valid when:
- [x] Declares `strict_types=1` in all files
- [x] Passes `composer validate` (composer.json is valid)
- [x] Passes `php-cs-fixer fix --dry-run` (PSR-12 compliant)
- [x] Passes `phpstan analyse --level=9` (no errors)
- [x] Passes `psalm --no-cache` (no errors)
- [x] Passes `phpunit` (all tests pass)
- [x] Has type hints for all parameters and return types
- [x] Has PHPDoc comments for all public methods
- [x] Follows PSR-4 autoloading standard
- [x] No security vulnerabilities (prepared statements, password hashing)
- [x] Test coverage >= 80% for critical code

### Pre-commit Checklist
```bash
#!/bin/bash
# Run these commands before committing

# 1. Validate composer.json
composer validate --strict

# 2. Check code style
php-cs-fixer fix --dry-run --diff

# 3. Run static analysis
phpstan analyse --level=9

# 4. Run Psalm
psalm --no-cache

# 5. Run tests
phpunit

# 6. Check test coverage
phpunit --coverage-text --coverage-filter=src

# If all pass, auto-fix code style
php-cs-fixer fix
```

## Code Verification Workflow

### Overview

**MANDATORY**: Every code change in PHP MUST be verified by a dedicated PHP Verification Agent before being committed. This is a **HARD REQUIREMENT** with **ZERO TOLERANCE** for violations.

### Verification Agent Responsibility

There can only be **ONE PHP Verification Agent** active at any time for a given set of changes. The Main Agent is responsible for:

1. **Delegating** to the PHP Verification Agent after implementation is complete
2. **Waiting** for verification results before proceeding
3. **Not committing** any PHP code until verification passes
4. **Reporting** verification results to the user

### When Verification Must Run

Verification MUST run:
- ✅ After ANY code changes to `.php` files
- ✅ After changes to `composer.json` or `composer.lock`
- ✅ After adding new dependencies
- ✅ After updating dependencies
- ✅ Before ANY commit containing PHP code
- ✅ After merging or rebasing branches

### Verification Agent Workflow

#### Step 1: Agent Delegation

**Main Agent** responsibilities:
```
1. Implementation agent completes PHP code changes
2. Implementation agent reports completion to Main Agent
3. Main Agent spawns ONE PHP Verification Agent
4. Main Agent provides verification agent with:
   - List of changed files
   - Description of changes made
   - Specification reference (if applicable)
5. Main Agent WAITS for verification results
```

**Verification Agent** receives:
- Context about what was changed
- Why it was changed
- Expected behavior
- Files modified

#### Step 2: Verification Agent Execution

The **PHP Verification Agent** MUST execute ALL of the following checks in order:

##### 1. Composer Validation
```bash
composer validate --strict
```
- **MUST PASS**: composer.json must be valid
- **On Failure**: Report validation errors
- **Zero Tolerance**: Fix composer.json before proceeding

##### 2. Code Style Check
```bash
php-cs-fixer fix --dry-run --diff
# or
phpcs --standard=PSR12 app/
```
- **MUST PASS**: Code must follow PSR-12
- **On Failure**: Run `php-cs-fixer fix` and report issues
- **Zero Tolerance**: No style violations allowed

##### 3. PHPStan Analysis
```bash
phpstan analyse --level=9 --no-progress
```
- **MUST PASS**: Zero PHPStan errors at level 9
- **On Failure**: Report ALL errors with file locations
- **Zero Tolerance**: Fix all type errors before proceeding

##### 4. Psalm Analysis
```bash
psalm --no-cache --show-info=false
```
- **MUST PASS**: Zero Psalm errors
- **On Failure**: Report ALL errors with file locations
- **Zero Tolerance**: Fix all issues before proceeding

##### 5. Test Execution
```bash
phpunit --testdox
```
- **MUST PASS**: All tests must pass
- **On Failure**: Report which tests failed and why
- **Verify**: Check test coverage meets requirements (80%+)

##### 6. Test Coverage Check
```bash
phpunit --coverage-text --coverage-filter=src
```
- **MUST PASS**: Coverage >= 80% for critical paths
- **On Warning**: Report coverage percentage
- **Action**: Add tests for uncovered code

##### 7. Security Check (if composer-audit available)
```bash
composer audit
```
- **Check**: Known security vulnerabilities in dependencies
- **On Warning**: Report vulnerabilities with severity
- **Action**: Update dependencies or document risks

#### Step 3: Standards Compliance Verification

The Verification Agent MUST also verify compliance with this stack file:

##### Code Quality Checks
- [ ] All files have `declare(strict_types=1)`
  ```bash
  find app -name "*.php" -type f -exec grep -L "declare(strict_types=1)" {} \;
  ```
  - Report any files missing strict types declaration

- [ ] All public methods have PHPDoc comments
  ```bash
  # PHPStan will catch this with level 9
  phpstan analyse --level=9
  ```

- [ ] No raw SQL queries (must use prepared statements)
  ```bash
  grep -r "\$pdo->query" app/
  grep -r "mysqli_query" app/
  ```
  - Report any unsafe database queries

- [ ] No eval() or similar dangerous functions
  ```bash
  grep -r "eval(" app/
  grep -r "exec(" app/
  grep -r "system(" app/
  grep -r "shell_exec(" app/
  ```
  - Report any dangerous function usage

- [ ] Passwords use password_hash()
  ```bash
  grep -r "md5(" app/ | grep -i password
  grep -r "sha1(" app/ | grep -i password
  ```
  - Report insecure password hashing

##### PHP-Specific Checks
- [ ] Proper namespace declarations
- [ ] PSR-4 autoloading compliance
- [ ] Readonly properties used where appropriate
- [ ] Enums used instead of constants where applicable
- [ ] Type hints present for all parameters and returns

#### Step 4: Verification Report

The Verification Agent MUST generate a comprehensive report:

##### Report Format
```markdown
# PHP Verification Report

## Summary
- **Status**: PASS ✅ / FAIL ❌
- **Files Changed**: [list of files]
- **Verification Time**: [timestamp]

## Check Results

### 1. Composer Validation
- **Status**: PASS ✅ / FAIL ❌
- **Details**: [any issues found]

### 2. Code Style Check
- **Status**: PASS ✅ / FAIL ❌
- **Violations**: [count]
- **Details**: [style issues]

### 3. PHPStan Analysis
- **Status**: PASS ✅ / FAIL ❌
- **Errors**: [count]
- **Details**: [error messages]

### 4. Psalm Analysis
- **Status**: PASS ✅ / FAIL ❌
- **Errors**: [count]
- **Details**: [error messages]

### 5. Tests
- **Tests Run**: [count]
- **Tests Passed**: [count]
- **Tests Failed**: [count]
- **Details**: [failure details]

### 6. Test Coverage
- **Coverage**: [percentage]
- **Status**: PASS ✅ / FAIL ❌
- **Details**: [uncovered files]

### 7. Security Check
- **Status**: PASS ✅ / FAIL ❌
- **Vulnerabilities**: [count]
- **Details**: [vulnerability list]

### 8. Standards Compliance
- **Strict Types**: PASS ✅ / FAIL ❌
- **PHPDoc Comments**: PASS ✅ / FAIL ❌
- **Safe SQL Queries**: PASS ✅ / FAIL ❌
- **No Dangerous Functions**: PASS ✅ / FAIL ❌
- **Password Security**: PASS ✅ / FAIL ❌

## Overall Assessment

[Detailed explanation of verification results]

## Recommendations

[Any suggestions for improvement]

## Blockers

[Any issues that prevent commit]
```

#### Step 5: Main Agent Response

Based on Verification Agent report:

##### If Verification PASSES (✅)
```
Main Agent actions:
1. Receives PASS report from Verification Agent
2. Reviews report for any warnings or recommendations
3. Commits the changes following Rule 03 (Work Commit Rules)
4. Includes verification summary in commit message:
   "Verified by PHP Verification Agent: All checks passed"
5. Pushes to remote following Rule 05 (Git Auto-Approval)
6. Reports success to user
```

##### If Verification FAILS (❌)
```
Main Agent actions:
1. Receives FAIL report from Verification Agent
2. DOES NOT COMMIT any code
3. Reports failures to implementation agent or user
4. Lists all issues that must be fixed:
   - Code style violations
   - PHPStan errors
   - Psalm errors
   - Test failures
   - Security issues
   - Standards violations
5. Implementation agent fixes issues
6. Repeats verification process
7. ONLY proceeds after PASS
```

### Verification Agent Requirements

The Verification Agent MUST:
- ✅ Be spawned by Main Agent ONLY
- ✅ Run ALL checks in order
- ✅ Generate comprehensive report
- ✅ Report results to Main Agent
- ✅ NOT commit any code (Main Agent's responsibility)
- ✅ NOT proceed with partial passes (all checks must pass)

The Verification Agent MUST NOT:
- ❌ Skip any verification checks
- ❌ Ignore failures ("we'll fix it later")
- ❌ Commit code directly
- ❌ Proceed when checks fail
- ❌ Run concurrently (only one per language stack)

### Enforcement

#### Zero Tolerance Policy

**VIOLATIONS** are treated with **ZERO TOLERANCE**:

- ❌ **FORBIDDEN**: Committing PHP code without verification
- ❌ **FORBIDDEN**: Skipping verification checks
- ❌ **FORBIDDEN**: Ignoring verification failures
- ❌ **FORBIDDEN**: Running verification after commit
- ❌ **FORBIDDEN**: Multiple concurrent verification agents

#### User Impact

Violations have serious consequences:
- ❌ **Runtime errors** in production from type issues
- ❌ **Security vulnerabilities** (SQL injection, XSS)
- ❌ **Failed tests** discovered too late
- ❌ **Code quality degradation** over time
- ❌ **Technical debt** accumulation
- ❌ **User trust** in agent reliability lost

**THE USER WILL BE UPSET** if code is committed without proper verification!

### Verification Commands Quick Reference

```bash
# Complete verification suite (run in order)

# 1. Validate composer
composer validate --strict

# 2. Code style
php-cs-fixer fix --dry-run --diff

# 3. Static analysis
phpstan analyse --level=9

# 4. Psalm
psalm --no-cache

# 5. Tests
phpunit

# 6. Coverage
phpunit --coverage-text --coverage-filter=app

# 7. Security
composer audit

# 8. Standards check
find app -name "*.php" -type f -exec grep -L "declare(strict_types=1)" {} \;
grep -r "eval(" app/
grep -r "\$pdo->query" app/

# All checks must PASS before commit
```

## Common Pitfalls

### Pitfall 1: Not Using Strict Types
**Problem**: Type coercion can cause unexpected bugs.
**Solution**: Always use `declare(strict_types=1)` at the top of every file.

### Pitfall 2: Missing Type Hints
**Problem**: No static analysis benefits, runtime errors.
**Solution**: Add type hints to all parameters, returns, and properties.

### Pitfall 3: SQL Injection
**Problem**: String concatenation in SQL queries.
**Solution**: Always use prepared statements with parameter binding.

### Pitfall 4: Weak Password Hashing
**Problem**: Using MD5 or SHA1 for passwords.
**Solution**: Use `password_hash()` with PASSWORD_ARGON2ID or PASSWORD_BCRYPT.

### Pitfall 5: Not Validating Input
**Problem**: Accepting user input without validation.
**Solution**: Validate and sanitize all input before use.

### Pitfall 6: Catching Generic Exceptions
**Problem**: `catch (Exception $e)` hides specific errors.
**Solution**: Catch specific exception types, use custom exceptions.

### Pitfall 7: Mutable DTOs
**Problem**: DTOs that can be modified after creation.
**Solution**: Use readonly properties and classes for DTOs.

### Pitfall 8: Not Using OPcache
**Problem**: Slow performance in production.
**Solution**: Enable OPcache in production environments.

## Learning Log

### 2026-01-16: Comprehensive PHP Standards Established
**Issue**: Initial creation of comprehensive PHP coding standards.
**Learning**: Established expert-level standards covering:
- Modern PHP 8.3+ features (readonly, enums, attributes, union types)
- Strict type declarations (mandatory)
- PSR-12 code style compliance
- Static analysis with PHPStan level 9 and Psalm
- Comprehensive testing with PHPUnit
- Security best practices (SQL injection, XSS, CSRF, password hashing)
- Dependency injection and SOLID principles
- Repository and service layer patterns
- Performance optimization techniques

**Corrective Action**: None (initial creation).
**New Standard**: All PHP code must follow these expert-level standards with zero tolerance for deviations.

---
*Created: 2026-01-16*
*Last Updated: 2026-01-16*
