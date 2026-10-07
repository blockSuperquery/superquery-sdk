//! The Rust types a generated entity field takes.
//!
//! Each one wraps the encoding [`crate::Value`] uses, so generated structs are
//! self-describing and a mapping cannot accidentally put a `String` where a
//! `BigInt` belongs. They are intentionally thin: arithmetic on `BigInt` is
//! deferred until a mapping actually needs it (see the note on each type).
//!
//! The decimal types validate on the way in. A malformed `BigInt` is cheap to
//! reject in the guest and expensive to discover as a failed column write in
//! the node, several blocks after the mapping that produced it.

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;
use core::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::scalar::Value;

/// An arbitrary-precision integer, carried as a decimal string.
///
/// Stored as a string rather than an `i128` because token amounts routinely
/// exceed 128 bits once decimals are involved. Arithmetic lands with the
/// bignum backend in a later milestone; for now the type exists so schemas,
/// generated code and the ABI agree on the encoding.
///
/// The encoding is canonical: an optional `-`, then digits with no leading
/// zeros, and never `-0`. Two equal numbers therefore always have equal
/// strings, which is what lets entity hashing compare them byte-wise.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct BigInt(String);

impl BigInt {
    /// Parse a base-10 integer, normalising it to canonical form.
    ///
    /// Accepts a leading `+` or `-` and redundant leading zeros, because that
    /// is what `U256::to_string` and hand-written fixtures produce; rejects
    /// everything else.
    pub fn new(decimal: impl AsRef<str>) -> Result<Self, InvalidDecimal> {
        let raw = decimal.as_ref();
        let invalid = || InvalidDecimal::new("BigInt", raw);

        let (negative, digits) = split_sign(raw);
        if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
            return Err(invalid());
        }

        let digits = digits.trim_start_matches('0');
        Ok(Self(match (negative, digits.is_empty()) {
            (_, true) => "0".to_owned(),
            (true, false) => alloc::format!("-{digits}"),
            (false, false) => digits.to_owned(),
        }))
    }

    /// The decimal representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for BigInt {
    type Err = InvalidDecimal;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl<'de> Deserialize<'de> for BigInt {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(de)?;
        Self::new(&raw).map_err(serde::de::Error::custom)
    }
}

// Primitive integers are canonical by construction, so these skip the parser.
macro_rules! bigint_from_primitive {
    ($($t:ty),*) => {$(
        impl From<$t> for BigInt {
            fn from(v: $t) -> Self {
                Self(v.to_string())
            }
        }
    )*};
}

bigint_from_primitive!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

impl From<BigInt> for Value {
    fn from(v: BigInt) -> Self {
        Value::BigInt(v.0)
    }
}

/// An arbitrary-precision decimal, carried as a string.
///
/// Plain positional notation only (`-12.5`, `0.001`); exponents are rejected
/// so the node never has to guess how a column should round them. Unlike
/// [`BigInt`] the digits are kept as written, because trailing zeros can carry
/// meaning (a price quoted to six places).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct BigDecimal(String);

impl BigDecimal {
    /// Parse a base-10 decimal.
    pub fn new(decimal: impl AsRef<str>) -> Result<Self, InvalidDecimal> {
        let raw = decimal.as_ref();
        let (_, body) = split_sign(raw);
        let (int, frac) = body.split_once('.').unwrap_or((body, ""));

        let digits = |s: &str| s.bytes().all(|b| b.is_ascii_digit());
        let has_digits = !int.is_empty() || !frac.is_empty();
        let dot_needs_fraction = !body.contains('.') || !frac.is_empty();

        if !has_digits || !dot_needs_fraction || !digits(int) || !digits(frac) {
            return Err(InvalidDecimal::new("BigDecimal", raw));
        }
        Ok(Self(raw.to_owned()))
    }

    /// The decimal representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for BigDecimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for BigDecimal {
    type Err = InvalidDecimal;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl<'de> Deserialize<'de> for BigDecimal {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(de)?;
        Self::new(&raw).map_err(serde::de::Error::custom)
    }
}

impl From<BigInt> for BigDecimal {
    fn from(v: BigInt) -> Self {
        Self(v.0)
    }
}

/// A string was not a valid [`BigInt`] or [`BigDecimal`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidDecimal {
    /// Which type rejected it.
    pub expected: &'static str,
    /// The rejected input.
    pub found: String,
}

impl InvalidDecimal {
    fn new(expected: &'static str, found: &str) -> Self {
        Self {
            expected,
            found: found.to_owned(),
        }
    }
}

impl fmt::Display for InvalidDecimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`{}` is not a valid {}", self.found, self.expected)
    }
}

impl std::error::Error for InvalidDecimal {}

fn split_sign(raw: &str) -> (bool, &str) {
    match raw.as_bytes().first() {
        Some(b'-') => (true, &raw[1..]),
        Some(b'+') => (false, &raw[1..]),
        _ => (false, raw),
    }
}

impl From<BigDecimal> for Value {
    fn from(v: BigDecimal) -> Self {
        Value::BigDecimal(v.0)
    }
}

/// Raw bytes, rendered as `0x`-prefixed hex everywhere user-visible.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Bytes(Vec<u8>);

impl Bytes {
    /// Wrap raw bytes.
    pub const fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    /// Parse `0x`-prefixed or bare hex.
    pub fn from_hex(s: &str) -> Result<Self, hex::FromHexError> {
        hex::decode(s.strip_prefix("0x").unwrap_or(s)).map(Self)
    }

    /// The `0x`-prefixed hex form.
    pub fn to_hex(&self) -> String {
        let mut out = String::from("0x");
        out.push_str(&hex::encode(&self.0));
        out
    }

    /// The underlying bytes.
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl From<Bytes> for Value {
    fn from(v: Bytes) -> Self {
        Value::Bytes(v.0)
    }
}

/// A UTC instant, in milliseconds since the Unix epoch.
///
/// Milliseconds rather than seconds because sub-second ordering matters on
/// fast chains, and rather than nanoseconds because no chain in scope reports
/// them.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Timestamp(i64);

impl Timestamp {
    /// Wrap a millisecond epoch value.
    pub const fn from_millis(millis: i64) -> Self {
        Self(millis)
    }

    /// Build from a whole-second epoch value, as chains usually report.
    pub const fn from_secs(secs: i64) -> Self {
        Self(secs * 1_000)
    }

    /// Milliseconds since the epoch.
    pub const fn as_millis(self) -> i64 {
        self.0
    }
}

impl From<Timestamp> for Value {
    fn from(v: Timestamp) -> Self {
        Value::Date(v.0)
    }
}

/// An opaque JSON document stored in a `Json` field.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Json(serde_json::Value);

impl Json {
    /// Wrap a JSON value.
    pub const fn new(value: serde_json::Value) -> Self {
        Self(value)
    }

    /// The underlying value.
    pub const fn get(&self) -> &serde_json::Value {
        &self.0
    }
}

impl From<Json> for Value {
    fn from(v: Json) -> Self {
        Value::Json(v.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newtypes_lower_to_the_matching_value_variant() {
        assert_eq!(Value::from(BigInt::from(7u64)), Value::BigInt("7".into()));
        assert_eq!(
            Value::from(Bytes::from_hex("0xff").unwrap()),
            Value::Bytes(alloc::vec![0xff])
        );
        assert_eq!(Value::from(Timestamp::from_secs(2)), Value::Date(2_000));
    }

    #[test]
    fn bigint_normalises_to_one_canonical_spelling() {
        assert_eq!(BigInt::new("000123").unwrap().as_str(), "123");
        assert_eq!(BigInt::new("+7").unwrap().as_str(), "7");
        assert_eq!(BigInt::new("-0").unwrap().as_str(), "0");
        assert_eq!(BigInt::new("-00").unwrap(), BigInt::from(0u8));
        assert_eq!(BigInt::new("-42").unwrap().as_str(), "-42");
    }

    #[test]
    fn bigint_rejects_anything_that_is_not_an_integer() {
        for bad in ["", "-", "1.5", "0x10", "1e9", " 1", "12a"] {
            assert!(BigInt::new(bad).is_err(), "expected `{bad}` to be rejected");
        }
    }

    #[test]
    fn bigint_accepts_values_wider_than_any_primitive() {
        let u256_max =
            "115792089237316195423570985008687907853269984665640564039457584007913129639935";
        assert_eq!(BigInt::new(u256_max).unwrap().as_str(), u256_max);
    }

    #[test]
    fn bigdecimal_accepts_positional_notation_only() {
        for good in ["0", "-12.5", "0.001", ".5", "10.000000"] {
            assert!(BigDecimal::new(good).is_ok(), "expected `{good}` to parse");
        }
        for bad in ["", ".", "1.", "1e9", "1.2.3", "--1", "NaN"] {
            assert!(
                BigDecimal::new(bad).is_err(),
                "expected `{bad}` to be rejected"
            );
        }
    }

    #[test]
    fn deserializing_runs_the_same_validation() {
        assert!(serde_json::from_str::<BigInt>(r#""12x""#).is_err());
        assert_eq!(
            serde_json::from_str::<BigInt>(r#""007""#).unwrap().as_str(),
            "7"
        );
    }

    #[test]
    fn bytes_round_trip_through_hex() {
        let b = Bytes::from_hex("0xdeadbeef").unwrap();
        assert_eq!(b.to_hex(), "0xdeadbeef");
        assert_eq!(Bytes::from_hex("deadbeef").unwrap(), b);
    }
}
