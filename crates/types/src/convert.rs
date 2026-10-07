//! Conversions between generated field types and [`Value`].
//!
//! `#[derive(SuperQueryEntity)]` lowers each field with [`ToValue`] and
//! rebuilds it with [`FromValue`]. They are traits rather than `From`/`TryFrom`
//! impls because `Vec<T>` needs a blanket impl, and `From<Vec<u8>> for Value`
//! already claims that spelling for raw bytes.

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

use crate::newtypes::{BigDecimal, BigInt, Bytes, Json, Timestamp};
use crate::scalar::Value;

/// Lower a field to the value that crosses the mapping ABI.
pub trait ToValue {
    /// The encoded value.
    fn to_value(&self) -> Value;
}

/// Rebuild a field from the value the host handed back.
pub trait FromValue: Sized {
    /// Decode, failing if the value has the wrong shape.
    fn from_value(value: &Value) -> Result<Self, FromValueError>;
}

/// A [`Value`] could not become the requested field type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FromValueError {
    /// The variant was not the one the field expects.
    Mismatch {
        /// The variant the field expects.
        expected: &'static str,
        /// The variant that arrived.
        found: &'static str,
    },
    /// The variant was right but its contents were not, e.g. a `BigInt`
    /// carrying `"12x"`.
    Invalid(String),
}

impl fmt::Display for FromValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mismatch { expected, found } => write!(f, "expected {expected}, found {found}"),
            Self::Invalid(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for FromValueError {}

fn mismatch(expected: &'static str, found: &Value) -> FromValueError {
    FromValueError::Mismatch {
        expected,
        found: found.type_name(),
    }
}

macro_rules! scalar_conversions {
    ($($ty:ty => $variant:ident ($bind:ident) { to: $to:expr, from: $from:expr $(,)? }),* $(,)?) => {$(
        impl ToValue for $ty {
            fn to_value(&self) -> Value {
                let $bind = self;
                Value::$variant($to)
            }
        }

        impl FromValue for $ty {
            fn from_value(value: &Value) -> Result<Self, FromValueError> {
                match value {
                    Value::$variant($bind) => $from,
                    other => Err(mismatch(stringify!($variant), other)),
                }
            }
        }
    )*};
}

scalar_conversions! {
    String => String(v) { to: v.clone(), from: Ok(v.clone()) },
    bool => Boolean(v) { to: *v, from: Ok(*v) },
    i32 => Int(v) { to: *v, from: Ok(*v) },
    f64 => Float(v) { to: *v, from: Ok(*v) },
    BigInt => BigInt(v) {
        to: v.as_str().into(),
        from: BigInt::new(v).map_err(|e| FromValueError::Invalid(e.to_string())),
    },
    BigDecimal => BigDecimal(v) {
        to: v.as_str().into(),
        from: BigDecimal::new(v).map_err(|e| FromValueError::Invalid(e.to_string())),
    },
    Bytes => Bytes(v) { to: v.as_slice().to_vec(), from: Ok(Bytes::new(v.clone())) },
    Timestamp => Date(v) { to: v.as_millis(), from: Ok(Timestamp::from_millis(*v)) },
    Json => Json(v) { to: v.get().clone(), from: Ok(Json::new(v.clone())) },
}

impl<T: ToValue> ToValue for Option<T> {
    fn to_value(&self) -> Value {
        self.as_ref().map_or(Value::Null, ToValue::to_value)
    }
}

impl<T: FromValue> FromValue for Option<T> {
    fn from_value(value: &Value) -> Result<Self, FromValueError> {
        match value {
            Value::Null => Ok(None),
            other => T::from_value(other).map(Some),
        }
    }
}

impl<T: ToValue> ToValue for Vec<T> {
    fn to_value(&self) -> Value {
        Value::List(self.iter().map(ToValue::to_value).collect())
    }
}

impl<T: FromValue> FromValue for Vec<T> {
    fn from_value(value: &Value) -> Result<Self, FromValueError> {
        match value {
            Value::List(items) => items.iter().map(T::from_value).collect(),
            other => Err(mismatch("List", other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_trip<T: ToValue + FromValue + PartialEq + fmt::Debug>(v: T) {
        assert_eq!(T::from_value(&v.to_value()).unwrap(), v);
    }

    #[test]
    fn every_field_type_survives_a_round_trip() {
        round_trip(String::from("0xabc"));
        round_trip(true);
        round_trip(-7i32);
        round_trip(1.5f64);
        round_trip(BigInt::from(u128::MAX));
        round_trip(BigDecimal::new("-0.25").unwrap());
        round_trip(Bytes::from_hex("0xdead").unwrap());
        round_trip(Timestamp::from_secs(1_700_000_000));
        round_trip(Json::new(serde_json::json!({"a": [1, 2]})));
        round_trip(Some(3i32));
        round_trip(Option::<i32>::None);
        round_trip(alloc::vec![Some(BigInt::from(1u8)), None]);
    }

    #[test]
    fn a_wrong_variant_names_both_sides() {
        let err = i32::from_value(&Value::String("1".into())).unwrap_err();
        assert_eq!(err.to_string(), "expected Int, found String");
    }

    #[test]
    fn a_malformed_bigint_from_the_host_is_rejected() {
        let err = BigInt::from_value(&Value::BigInt("12x".into())).unwrap_err();
        assert!(matches!(err, FromValueError::Invalid(_)), "{err:?}");
    }

    #[test]
    fn null_only_decodes_into_an_option() {
        assert!(String::from_value(&Value::Null).is_err());
        assert_eq!(Option::<String>::from_value(&Value::Null).unwrap(), None);
    }
}
