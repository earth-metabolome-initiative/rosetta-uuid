# rosetta-uuid

[![CI](https://github.com/earth-metabolome-initiative/rosetta-uuid/actions/workflows/ci.yml/badge.svg)](https://github.com/earth-metabolome-initiative/rosetta-uuid/actions/workflows/ci.yml)
[![Security Audit](https://github.com/earth-metabolome-initiative/rosetta-uuid/workflows/Security%20Audit/badge.svg)](https://github.com/earth-metabolome-initiative/rosetta-uuid/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Codecov](https://codecov.io/gh/earth-metabolome-initiative/rosetta-uuid/branch/main/graph/badge.svg)](https://codecov.io/gh/earth-metabolome-initiative/rosetta-uuid)

A wrapper implementation of UUID providing binary `diesel` bindings for SQLite and PostgreSQL, and `redis` serialization support.

## Features

This crate provides a `Uuid` wrapper type that implements various traits based on enabled features:

* **[`diesel`](https://crates.io/crates/diesel)**: Enables Diesel integration.
  * **`postgres`**: Enables binary `Uuid` support for [PostgreSQL](https://www.postgresql.org/docs/current/datatype-uuid.html).
  * **`sqlite`**: Enables binary `Uuid` support for [SQLite](https://www.sqlite.org/datatype3.html) (stored as BLOB).
* **[`redis`](https://crates.io/crates/redis)**: Enables `ToRedisArgs` and `FromRedisValue` for easy [Redis](https://redis.io/) storage and retrieval.
* **[`serde`](https://crates.io/crates/serde)**: Enables serialization and deserialization via [Serde](https://serde.rs/).

## Usage

Add this to your `Cargo.toml`. Select the features matching your database or storage requirements.

```toml
[dependencies]
rosetta-uuid = { version = "0.1", features = ["diesel", "postgres", "redis", "serde"] }
```

### Example

```rust
use rosetta_uuid::Uuid;
use std::str::FromStr;

// Create a new random UUID (v4)
let id = Uuid::new_v4();

// Create a new timestamp-based UUID (v7) with the current UTC timestamp
let id_v7 = Uuid::utc_v7();

// Parse from string
let parsed = Uuid::from_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap();

// Access underlying uuid::Uuid methods via Deref
println!("Version: {:?}", id.get_version());
```

## Traits

The `Uuid` type implements:

* `FromStr`
* `Display`, `Debug`
* `Deref`, `DerefMut` (to `uuid::Uuid`)
* `AsRef<[u8; 16]>`, `AsRef<uuid::Uuid>`
* `From<uuid::Uuid>`, `Into<uuid::Uuid>`
* `From<[u8; 16]>`, `Into<[u8; 16]>`
* `Default` (returns nil UUID)
