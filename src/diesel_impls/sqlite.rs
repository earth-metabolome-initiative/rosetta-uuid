#![cfg(feature = "sqlite")]
//! Implementation for the [`SQLite` Backend](diesel::sqlite::Sqlite).

impl diesel::deserialize::FromSql<crate::diesel_impls::Uuid, diesel::sqlite::Sqlite>
    for crate::Uuid
{
    fn from_sql(
        mut value: diesel::sqlite::SqliteValue<'_, '_, '_>,
    ) -> diesel::deserialize::Result<Self> {
        uuid::Uuid::from_slice(value.read_blob())
            .map_err(Into::into)
            .map(Self::from)
    }
}

impl diesel::serialize::ToSql<crate::diesel_impls::Uuid, diesel::sqlite::Sqlite> for crate::Uuid {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::sqlite::Sqlite>,
    ) -> diesel::serialize::Result {
        out.set_value(self.as_bytes().as_slice());
        Ok(diesel::serialize::IsNull::No)
    }
}

impl diesel::deserialize::FromSql<diesel::sql_types::Binary, diesel::sqlite::Sqlite>
    for crate::Uuid
{
    fn from_sql(
        mut value: diesel::sqlite::SqliteValue<'_, '_, '_>,
    ) -> diesel::deserialize::Result<Self> {
        uuid::Uuid::from_slice(value.read_blob())
            .map_err(Into::into)
            .map(Self::from)
    }
}

impl diesel::serialize::ToSql<diesel::sql_types::Binary, diesel::sqlite::Sqlite> for crate::Uuid {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::sqlite::Sqlite>,
    ) -> diesel::serialize::Result {
        out.set_value(self.as_bytes().as_slice());
        Ok(diesel::serialize::IsNull::No)
    }
}

impl diesel::deserialize::FromSql<diesel::sql_types::Text, diesel::sqlite::Sqlite> for crate::Uuid {
    fn from_sql(
        mut value: diesel::sqlite::SqliteValue<'_, '_, '_>,
    ) -> diesel::deserialize::Result<Self> {
        uuid::Uuid::parse_str(value.read_text())
            .map_err(Into::into)
            .map(Self::from)
    }
}

impl diesel::serialize::ToSql<diesel::sql_types::Text, diesel::sqlite::Sqlite> for crate::Uuid {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::sqlite::Sqlite>,
    ) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(diesel::serialize::IsNull::No)
    }
}

#[cfg(test)]
mod tests {
    use crate::Uuid;
    use diesel::prelude::*;
    use diesel::sqlite::SqliteConnection;

    #[derive(QueryableByName, Debug, PartialEq)]
    struct Row {
        #[diesel(sql_type = crate::diesel_impls::Uuid)]
        id: Uuid,
    }

    #[test]
    fn test_sqlite_roundtrip() {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();

        diesel::sql_query("CREATE TABLE test_table (id BLOB PRIMARY KEY)")
            .execute(&mut conn)
            .unwrap();

        let uuid = Uuid::new_v4();

        diesel::sql_query("INSERT INTO test_table (id) VALUES (?)")
            .bind::<crate::diesel_impls::Uuid, _>(uuid)
            .execute(&mut conn)
            .unwrap();

        let result = diesel::sql_query("SELECT id FROM test_table")
            .get_result::<Row>(&mut conn)
            .unwrap();

        assert_eq!(result.id, uuid);
    }

    #[derive(QueryableByName, Debug, PartialEq)]
    struct RowBinary {
        #[diesel(sql_type = diesel::sql_types::Binary)]
        id: Uuid,
    }

    #[test]
    fn test_sqlite_binary_roundtrip() {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();

        diesel::sql_query("CREATE TABLE test_table_binary (id BLOB PRIMARY KEY)")
            .execute(&mut conn)
            .unwrap();

        let uuid = Uuid::new_v4();

        diesel::sql_query("INSERT INTO test_table_binary (id) VALUES (?)")
            .bind::<diesel::sql_types::Binary, _>(uuid)
            .execute(&mut conn)
            .unwrap();

        let result = diesel::sql_query("SELECT id FROM test_table_binary")
            .get_result::<RowBinary>(&mut conn)
            .unwrap();

        assert_eq!(result.id, uuid);
    }

    #[derive(QueryableByName, Debug, PartialEq)]
    struct RowText {
        #[diesel(sql_type = diesel::sql_types::Text)]
        id: Uuid,
    }

    #[test]
    fn test_sqlite_text_roundtrip() {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();

        diesel::sql_query("CREATE TABLE test_table_text (id TEXT PRIMARY KEY)")
            .execute(&mut conn)
            .unwrap();

        let uuid = Uuid::new_v4();

        diesel::sql_query("INSERT INTO test_table_text (id) VALUES (?)")
            .bind::<diesel::sql_types::Text, _>(uuid)
            .execute(&mut conn)
            .unwrap();

        let result = diesel::sql_query("SELECT id FROM test_table_text")
            .get_result::<RowText>(&mut conn)
            .unwrap();

        assert_eq!(result.id, uuid);
    }
}
