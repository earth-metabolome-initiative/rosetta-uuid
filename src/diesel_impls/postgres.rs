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

#[cfg(test)]
mod tests {
    use crate::Uuid;
    use std::env;
    use diesel::pg::PgConnection;
    use diesel::prelude::*;

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
}
