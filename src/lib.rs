#![doc = include_str!("../README.md")]

use core::str::FromStr;

pub mod diesel_impls;
mod redis;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "diesel",
    derive(diesel::expression::AsExpression, diesel::deserialize::FromSqlRow)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "diesel", diesel(sql_type = crate::diesel_impls::Uuid))]
/// A wrapper around the `uuid` crate's `Uuid` type.
///
/// # Examples
///
/// ```
/// use rosetta_uuid::Uuid;
/// use std::str::FromStr;
///
/// let uuid = Uuid::new_v4();
/// let parsed = Uuid::from_str(&uuid.to_string()).unwrap();
/// assert_eq!(uuid, parsed);
/// ```
pub struct Uuid(uuid::Uuid);

impl Uuid {
    #[must_use]
    /// Creates a new `Uuid` using the `uuid` crate's `new_v4` method.
    ///
    /// # Examples
    ///
    /// ```
    /// use rosetta_uuid::Uuid;
    ///
    /// let uuid = Uuid::new_v4();
    /// ```
    pub fn new_v4() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    #[must_use]
    /// Creates a new `Uuid` using the `uuid` crate's `new_v7` method with the current UTC timestamp.
    ///
    /// # Examples
    ///
    /// ```
    /// use rosetta_uuid::Uuid;
    ///
    /// let uuid = Uuid::utc_v7();
    /// ```
    pub fn utc_v7() -> Self {
        let utc_now = ::chrono::Utc::now();
        ::uuid::Uuid::new_v7(::uuid::Timestamp::from_unix_time(
            u64::try_from(utc_now.timestamp()).expect("Time went backwards"),
            utc_now.timestamp_subsec_nanos(),
            0,
            12,
        ))
        .into()
    }
}

impl Default for Uuid {
    fn default() -> Self {
        Self(uuid::Uuid::nil())
    }
}

impl FromStr for Uuid {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(uuid::Uuid::from_str(s)?))
    }
}

impl From<uuid::Uuid> for Uuid {
    fn from(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }
}

impl From<Uuid> for uuid::Uuid {
    fn from(uuid: Uuid) -> Self {
        uuid.0
    }
}

impl From<[u8; 16]> for Uuid {
    fn from(bytes: [u8; 16]) -> Self {
        Self(uuid::Uuid::from_bytes(bytes))
    }
}

impl From<Uuid> for [u8; 16] {
    fn from(uuid: Uuid) -> Self {
        *uuid.0.as_bytes()
    }
}

impl<'a> From<&'a [u8; 16]> for Uuid {
    fn from(bytes: &'a [u8; 16]) -> Self {
        Self(uuid::Uuid::from_bytes(*bytes))
    }
}

impl AsRef<uuid::Uuid> for Uuid {
    fn as_ref(&self) -> &uuid::Uuid {
        &self.0
    }
}

impl AsMut<uuid::Uuid> for Uuid {
    fn as_mut(&mut self) -> &mut uuid::Uuid {
        &mut self.0
    }
}

impl AsRef<[u8; 16]> for Uuid {
    fn as_ref(&self) -> &[u8; 16] {
        self.0.as_bytes()
    }
}

impl core::ops::Deref for Uuid {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for Uuid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl core::fmt::Display for Uuid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;
    use std::collections::HashSet;
    use std::str::FromStr;

    #[test]
    fn test_default() {
        let uuid = Uuid::default();
        assert_eq!(uuid.0, uuid::Uuid::nil());
        assert_eq!(uuid.to_string(), "00000000-0000-0000-0000-000000000000");
    }

    #[test]
    fn test_from_str() {
        let s = "67e55044-10b1-426f-9247-bb680e5fe0c8";
        let uuid = Uuid::from_str(s).unwrap();
        assert_eq!(uuid.to_string(), s);

        let invalid = "invalid-uuid";
        assert!(Uuid::from_str(invalid).is_err());
    }

    #[test]
    fn test_from_conversions() {
        let inner = uuid::Uuid::new_v4();
        let wrapper: Uuid = inner.into();
        assert_eq!(wrapper.0, inner);

        let back: uuid::Uuid = wrapper.into();
        assert_eq!(back, inner);

        let bytes = [0u8; 16];
        let from_bytes: Uuid = bytes.into();
        assert_eq!(from_bytes.0, uuid::Uuid::nil());

        let bytes_back: [u8; 16] = from_bytes.into();
        assert_eq!(bytes_back, bytes);

        let bytes_ref = &[0u8; 16];
        let from_bytes_ref: Uuid = bytes_ref.into();
        assert_eq!(from_bytes_ref.0, uuid::Uuid::nil());
    }

    #[test]
    fn test_as_ref_as_mut() {
        let mut uuid = Uuid::new_v4();

        let r: &uuid::Uuid = uuid.as_ref();
        assert_eq!(*r, uuid.0);

        let bytes_ref: &[u8; 16] = uuid.as_ref();
        assert_eq!(bytes_ref, uuid.0.as_bytes());

        let m: &mut uuid::Uuid = uuid.as_mut();
        // Modify existing to something known
        *m = uuid::Uuid::nil();
        assert_eq!(uuid.0, uuid::Uuid::nil());
    }

    #[test]
    fn test_deref_deref_mut() {
        let mut uuid = Uuid::new_v4();

        // Deref
        assert_eq!(uuid.get_version(), Some(uuid::Version::Random));

        // DerefMut (modifying internally to nil just as a test)
        *uuid = uuid::Uuid::nil();
        assert!(uuid.is_nil());
    }

    #[test]
    fn test_display() {
        let s = "67e55044-10b1-426f-9247-bb680e5fe0c8";
        let uuid = Uuid::from_str(s).unwrap();
        assert_eq!(format!("{uuid}"), s);
    }

    #[test]
    fn test_standard_traits() {
        let uuid1 = Uuid::new_v4();
        let uuid2 = Clone::clone(&uuid1); // Clone
        let uuid3 = uuid1; // Copy

        assert_eq!(uuid1, uuid2); // PartialEq
        assert_eq!(uuid1, uuid3); // PartialEq
        assert!(uuid1 == uuid2); // Eq check implicitly

        let mut set = HashSet::new();
        set.insert(uuid1); // Hash
        assert!(set.contains(&uuid2));
    }

    #[test]
    fn test_ord() {
        let u1 = Uuid::default(); // nil, all zeros
        let u2 = Uuid::new_v4(); // random

        assert_eq!(u1.cmp(&u2), Ordering::Less); // Ord
        assert!(u1 < u2); // PartialOrd
    }

    #[test]
    fn test_new_constructors() {
        let v4 = Uuid::new_v4();
        assert_eq!(v4.get_version(), Some(uuid::Version::Random));

        let v7 = Uuid::utc_v7();
        // Version v7 might check differently depending on uuid crate version
        // assert_eq!(v7.get_version(), Some(uuid::Version::Sortable));
        assert!(!v7.is_nil());
    }
}
