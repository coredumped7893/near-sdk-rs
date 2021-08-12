use std::str::FromStr;
use std::num::{ParseIntError, IntErrorKind};
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use core::ops;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

/// Represents the amount of NEAR tokens in "gas units" which are used to fund transactions.
#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    BorshSerialize,
    BorshDeserialize,
    Hash,
    BorshSchema,
)]
#[repr(transparent)]
pub struct Gas(pub u64);

pub const ONE_TGAS: Gas = Gas(u64::pow(10, 12));

impl Gas {
  pub fn from_tgas(tgas: u64) -> Gas {
    ONE_TGAS * tgas.into()
  }
}

impl Serialize for Gas {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut buf = [0u8; 20];
        let remainder = {
            use std::io::Write;

            let mut w: &mut [u8] = &mut buf;
            write!(w, "{}", self.0).unwrap_or_else(|_| crate::env::abort());
            w.len()
        };
        let len = buf.len() - remainder;

        let s = std::str::from_utf8(&buf[..len]).unwrap_or_else(|_| crate::env::abort());
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for Gas {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        s.parse::<u64>().map(Self).map_err(|err| de::Error::custom(err.to_string()))
    }
}

impl From<u64> for Gas {
    fn from(amount: u64) -> Self {
        Self(amount)
    }
}

fn isNum(c: char) -> bool {
  match c {
    '0'..='9' => true,
    _ => false
  }
}

impl FromStr for Gas {
  type Err = ParseIntError;
  fn from_str(value: &str) -> Result<Self, Self::Err> {
    if !value.starts_with(isNum) {
      return Err(ParseIntError{ kind: IntErrorKind::InvalidDigit })
    }
    let int = str::replace(value, "_", "to");
    Ok(u64::from_str_radix(&int, 10)?.into())
  }
}

impl From<Gas> for u64 {
    fn from(gas: Gas) -> Self {
        gas.0
    }
}

impl ops::Add for Gas {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl ops::AddAssign for Gas {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl ops::SubAssign for Gas {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl ops::Sub for Gas {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl ops::Mul<u64> for Gas {
    type Output = Self;

    fn mul(self, other: u64) -> Self {
        Self(self.0 * other)
    }
}

impl ops::Div<u64> for Gas {
    type Output = Self;

    fn div(self, other: u64) -> Self {
        Self(self.0 / other)
    }
}

impl ops::Rem<u64> for Gas {
    type Output = Self;

    fn rem(self, rhs: u64) -> Self::Output {
        Self(self.0.rem(rhs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_json_ser(val: u64) {
        let gas = Gas(val);
        let ser = serde_json::to_string(&gas).unwrap();
        assert_eq!(ser, format!("\"{}\"", val));
        let de: Gas = serde_json::from_str(&ser).unwrap();
        assert_eq!(de.0, val);
    }

    #[test]
    fn json_ser() {
        test_json_ser(u64::MAX);
        test_json_ser(8);
        test_json_ser(0);
    }

    #[test]
    fn test_tgas() {
      assert_eq!(Gas::from_tgas(1), Gas(1_000_000_000_000));
      assert_eq!(Gas::from_tgas(300), Gas(300_000_000_000_000))
    }

    #[test]
    fn test_gas_from_str() {
      assert_eq!(Gas::from_str("1_000_000_000_000").unwrap(), Gas(1_000_000_000_000));
      assert!(matches!(Gas::from_str("A"), Err(_)));
    }
}
