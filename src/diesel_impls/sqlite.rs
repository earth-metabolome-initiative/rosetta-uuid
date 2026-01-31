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

#[cfg(test)]
mod tests {
    use crate::Uuid;
    use diesel::prelude::*;
    use diesel::sqlite::SqliteConnection;

    #[test]
    fn test_sqlite_roundtrip() {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();

        diesel::sql_query("CREATE TABLE test_table (id BLOB PRIMARY KEY)")
            .execute(&mut conn)
            .unwrap();

        #[derive(QueryableByName, Debug, PartialEq)]
        struct Row {
            #[diesel(sql_type = crate::diesel_impls::Uuid)]
            id: Uuid,
        }

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
}
