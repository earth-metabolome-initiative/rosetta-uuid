//! Tests for the `wasm32-unknown-unknown` target.
//!
//! These exercise the library feature set (not dev-only features): random and
//! timestamp UUID generation must have a working RNG backend in the browser, and
//! the SQLite `DEFAULT (uuidv7())` path must mint a distinct id per row.
#![cfg(target_arch = "wasm32")]

use rosetta_uuid::Uuid;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_v4_generation() {
    let uuid = Uuid::new_v4();
    assert_eq!(uuid.get_version(), Some(uuid::Version::Random));
    assert_eq!(uuid.as_bytes().len(), 16);
}

#[wasm_bindgen_test]
fn test_v4_distinct() {
    // A working RNG backend must yield different values across calls.
    assert_ne!(Uuid::new_v4(), Uuid::new_v4());
}

#[wasm_bindgen_test]
fn test_v7_generation() {
    let uuid = Uuid::utc_v7();
    // V7 is time-based, so not nil.
    assert_eq!(uuid.get_version(), Some(uuid::Version::SortRand));
    assert_eq!(uuid.as_bytes().len(), 16);
}

#[wasm_bindgen_test]
fn test_v7_distinct() {
    assert_ne!(Uuid::utc_v7(), Uuid::utc_v7());
}

/// Round-trip through an in-browser SQLite connection (sqlite-wasm-rs) exactly as
/// connetto uses the crate: a column `DEFAULT (uuidv7())` backed by the
/// nondeterministic registrar. Two rows inserted without an explicit id must
/// receive two distinct 16-byte ids. A deterministic registration would let
/// SQLite constant-fold the default, giving both rows the same id.
#[cfg(feature = "sqlite")]
#[wasm_bindgen_test]
fn test_sqlite_default_uuidv7_mints_distinct_ids() {
    use diesel::connection::SimpleConnection;
    use diesel::prelude::*;

    #[diesel::declare_sql_function]
    extern "SQL" {
        /// Generates a UUID v7.
        fn uuidv7() -> Binary;
    }

    diesel::table! {
        items (id) {
            id -> rosetta_uuid::sql_types::Uuid,
        }
    }

    let mut connection =
        SqliteConnection::establish(":memory:").expect("failed to open in-memory SQLite");

    uuidv7_utils::register_nondeterministic_impl(&connection, Uuid::utc_v7)
        .expect("failed to register uuidv7");

    // DDL: the typed DSL cannot express `CREATE TABLE ... DEFAULT (uuidv7())`.
    connection
        .batch_execute(
            "CREATE TABLE items (id BLOB DEFAULT (uuidv7()) CHECK (length(id) = 16) NOT NULL);",
        )
        .expect("failed to create table");

    // Insert twice omitting the id so the column DEFAULT mints it.
    for _ in 0..2 {
        diesel::insert_into(items::table)
            .default_values()
            .execute(&mut connection)
            .expect("failed to insert row");
    }

    let ids: Vec<Uuid> = items::table
        .select(items::id)
        .load(&mut connection)
        .expect("failed to load ids");

    assert_eq!(ids.len(), 2);
    assert_ne!(ids[0], ids[1]);
    assert_eq!(ids[0].as_bytes().len(), 16);
    assert_eq!(ids[1].as_bytes().len(), 16);
}
