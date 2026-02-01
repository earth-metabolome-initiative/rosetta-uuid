#![cfg(feature = "postgres")]
//! Implementation for the [Postgres Backend](diesel::pg::Pg).

impl diesel::deserialize::FromSql<crate::diesel_impls::Uuid, diesel::pg::Pg> for crate::Uuid {
    fn from_sql(value: diesel::pg::PgValue<'_>) -> diesel::deserialize::Result<Self> {
        <uuid::Uuid as diesel::deserialize::FromSql<diesel::sql_types::Uuid, diesel::pg::Pg>>::from_sql(value).map(Self::from)
    }
}

impl diesel::serialize::ToSql<crate::diesel_impls::Uuid, diesel::pg::Pg> for crate::Uuid {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        <uuid::Uuid as diesel::serialize::ToSql<diesel::sql_types::Uuid, diesel::pg::Pg>>::to_sql(
            self.as_ref(),
            out,
        )
    }
}

impl diesel::deserialize::FromSql<diesel::sql_types::Binary, diesel::pg::Pg> for crate::Uuid {
    fn from_sql(value: diesel::pg::PgValue<'_>) -> diesel::deserialize::Result<Self> {
        uuid::Uuid::from_slice(value.as_bytes())
            .map_err(Into::into)
            .map(Self::from)
    }
}

impl diesel::serialize::ToSql<diesel::sql_types::Binary, diesel::pg::Pg> for crate::Uuid {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        use std::io::Write;
        out.write_all(self.as_bytes())
            .map(|()| diesel::serialize::IsNull::No)
            .map_err(Into::into)
    }
}

impl diesel::deserialize::FromSql<diesel::sql_types::Text, diesel::pg::Pg> for crate::Uuid {
    fn from_sql(value: diesel::pg::PgValue<'_>) -> diesel::deserialize::Result<Self> {
        use std::str::FromStr;
        let string = std::str::from_utf8(value.as_bytes())?;
        uuid::Uuid::from_str(string)
            .map_err(Into::into)
            .map(Self::from)
    }
}

impl diesel::serialize::ToSql<diesel::sql_types::Text, diesel::pg::Pg> for crate::Uuid {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        use std::io::Write;
        out.write_all(self.to_string().as_bytes())
            .map(|()| diesel::serialize::IsNull::No)
            .map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use crate::Uuid;
    use diesel::pg::PgConnection;
    use diesel::prelude::*;
    use std::env;

    #[derive(QueryableByName, Debug, PartialEq)]
    struct Row {
        #[diesel(sql_type = crate::diesel_impls::Uuid)]
        id: Uuid,
    }

    #[test]
    fn test_postgres_roundtrip() {
        let Ok(database_url) = env::var("DATABASE_URL") else {
            eprintln!("Skipping test_postgres_roundtrip: DATABASE_URL not set");
            return;
        };

        let mut conn = PgConnection::establish(&database_url).unwrap();

        diesel::sql_query("CREATE TEMPORARY TABLE test_table (id UUID PRIMARY KEY)")
            .execute(&mut conn)
            .unwrap();

        let uuid = Uuid::new_v4();

        diesel::sql_query("INSERT INTO test_table (id) VALUES ($1)")
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
    fn test_postgres_binary_roundtrip() {
        let Ok(database_url) = env::var("DATABASE_URL") else {
            eprintln!("Skipping test_postgres_binary_roundtrip: DATABASE_URL not set");
            return;
        };

        let mut conn = PgConnection::establish(&database_url).unwrap();

        diesel::sql_query("CREATE TEMPORARY TABLE test_table_binary (id BYTEA PRIMARY KEY)")
            .execute(&mut conn)
            .unwrap();

        let uuid = Uuid::new_v4();

        diesel::sql_query("INSERT INTO test_table_binary (id) VALUES ($1)")
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
    fn test_postgres_text_roundtrip() {
        let Ok(database_url) = env::var("DATABASE_URL") else {
            eprintln!("Skipping test_postgres_text_roundtrip: DATABASE_URL not set");
            return;
        };

        let mut conn = PgConnection::establish(&database_url).unwrap();

        diesel::sql_query("CREATE TEMPORARY TABLE test_table_text (id TEXT PRIMARY KEY)")
            .execute(&mut conn)
            .unwrap();

        let uuid = Uuid::new_v4();

        diesel::sql_query("INSERT INTO test_table_text (id) VALUES ($1)")
            .bind::<diesel::sql_types::Text, _>(uuid)
            .execute(&mut conn)
            .unwrap();

        let result = diesel::sql_query("SELECT id FROM test_table_text")
            .get_result::<RowText>(&mut conn)
            .unwrap();

        assert_eq!(result.id, uuid);
    }
}
