# Deterministic Simulation Testing (DST) Documentation

Comprehensive documentation for implementing Deterministic Simulation Testing in Rust.

## Contents

| File | Description |
|------|-------------|
| [dst-rust-guide.md](./dst-rust-guide.md) | Main tutorial on DST concepts, the four pillars of determinism, and using libraries like turmoil/madsim |
| [dst-internals-guide.md](./dst-internals-guide.md) | Deep dive into building your own DST framework from scratch |
| [examples/basic-dst-setup.rs](./examples/basic-dst-setup.rs) | Complete working examples of DST test setups |

## Quick Start

1. **New to DST?** Start with [dst-rust-guide.md](./dst-rust-guide.md)
2. **Want to build custom infrastructure?** Read [dst-internals-guide.md](./dst-internals-guide.md)
3. **Need working code?** Check [examples/](./examples/)

## The Four Pillars of Determinism

For reproducible distributed system tests, control these sources of non-determinism:

| Pillar | Problem | Solution |
|--------|---------|----------|
| **Execution** | Thread scheduling varies | Single-threaded async |
| **Entropy** | Random uses system entropy | Seeded PRNG |
| **Time** | Real clocks advance unpredictably | Simulated time |
| **I/O** | Network has variable latency | In-memory simulation |

## Key Libraries

- **turmoil** - Network simulation for Tokio
- **madsim** - Full libc interception for comprehensive control
- **proptest** - Property-based input generation

## Related Skill

See the corresponding skill file for agent usage: [.agents/skills/dst-rust/skill.md](../../skills/dst-rust/skill.md)

---

*Created: 2026-01-19*
