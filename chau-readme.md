# Chau - Blockchain-Based Estate Planning Platform

Chau is a decentralized estate planning platform built on the Stellar network using Soroban smart contracts. It provides a secure, transparent, and efficient way to create, manage, and execute digital wills.

## Features

- Secure will creation and management
- Multi-signature support
- Advanced execution conditions
- Token-based ownership representation
- Client-side encryption
- Legal compliance framework

## Prerequisites

- Rust 1.69 or higher
- Soroban CLI
- Node.js 16 or higher (for web interface)
- Stellar account and wallet

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/chau.git
cd chau
```

2. Install dependencies:
```bash
cargo build
```

3. Run tests:
```bash
cargo test
```

## Usage

### Creating a New Will

```rust
use chau::contracts::will_registry::WillRegistry;

// Initialize the contract
let contract = WillRegistry::initialize(env, owner);

// Create a new will
let will_id = contract.create_will(
    env,
    owner,
    content_hash,
    beneficiaries,
    execution_conditions,
);
```

### Managing Wills

```rust
// Activate a will
contract.activate_will(env, will_id);

// Update a will
contract.update_will(
    env,
    will_id,
    new_content_hash,
    new_beneficiaries,
    new_execution_conditions,
);

// Execute a will
contract.execute_will(env, will_id);
```

## Documentation

For detailed documentation, please refer to the [docs](./docs) directory:

- [Architecture Overview](./docs/ARCHITECTURE.md)
- [Contributing Guidelines](./docs/CONTRIBUTING.md)
- [Security Policy](./docs/SECURITY.md)

## Contributing

We welcome contributions! Please see our [Contributing Guidelines](./docs/CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Security

For security concerns, please refer to our [Security Policy](./docs/SECURITY.md).
