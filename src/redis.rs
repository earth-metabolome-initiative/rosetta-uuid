#![cfg(feature = "redis")]
//! Submodule implementing the `ToRedisArgs` and `FromRedisValue` traits for UUIDs.

use redis::{FromRedisValue, RedisWrite, ToRedisArgs, Value};

impl ToRedisArgs for crate::Uuid {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(self.0.as_bytes());
    }
}

impl FromRedisValue for crate::Uuid {
    fn from_redis_value(v: Value) -> Result<Self, redis::ParsingError> {
        match v {
            Value::BulkString(bytes) => {
                // If it's exactly 16 bytes, assume it is the raw UUID bytes.
                if bytes.len() == 16 {
                    return uuid::Uuid::from_slice(&bytes)
                        .map(crate::Uuid)
                        .map_err(|_| redis::ParsingError::from("Invalid UUID bytes"));
                }

                // Otherwise try to interpret as a string.
                let s = std::str::from_utf8(&bytes).map_err(|_| {
                    redis::ParsingError::from("Invalid UTF-8 sequence for UUID string")
                })?;

                s.parse::<crate::Uuid>()
                    .map_err(|_| redis::ParsingError::from("Invalid UUID string"))
            }
            Value::SimpleString(s) => s
                .parse::<crate::Uuid>()
                .map_err(|_| redis::ParsingError::from("Invalid UUID string")),
            _ => Err(redis::ParsingError::from(
                "Response type not a UUID compatible value",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        let id = crate::Uuid::new_v4();
        let args = id.to_redis_args();
        assert_eq!(args.len(), 1);
        // Assuming to_redis_args produces BulkString (Data in old versions)
        // We'll see what it produces by inspecting or letting it fail if mismatches
        // Check if we can reconstruct Value from args.
        // Redis args are usually Vec<Vec<u8>>.
        let bytes = args[0].clone();
        let val = Value::BulkString(bytes);
        let parsed = crate::Uuid::from_redis_value(val).unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn test_explicit_valid_bytes() {
        let id_uuid = uuid::Uuid::new_v4();
        let id = crate::Uuid::from(id_uuid);
        let bytes = id_uuid.as_bytes().to_vec();

        // Ensure strictly 16 bytes
        assert_eq!(bytes.len(), 16);

        let val = Value::BulkString(bytes);
        let parsed = crate::Uuid::from_redis_value(val).unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn test_explicit_valid_string() {
        let id_uuid = uuid::Uuid::new_v4();
        let id = crate::Uuid::from(id_uuid);
        let s = id_uuid.to_string();

        // 1. As BulkString
        let val_data = Value::BulkString(s.as_bytes().to_vec());
        let parsed_data = crate::Uuid::from_redis_value(val_data).unwrap();
        assert_eq!(id, parsed_data);

        // 2. As SimpleString
        let val_status = Value::SimpleString(s);
        let parsed_status = crate::Uuid::from_redis_value(val_status).unwrap();
        assert_eq!(id, parsed_status);
    }
}
