use std::{
    fmt::{self, Display},
    str::FromStr,
};

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Debug, Clone, Copy)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

/// Gets the current version as a string.
#[macro_export]
macro_rules! version(
  () => ( env!( "CARGO_PKG_VERSION" ) )
);

impl Version {
    pub fn major(&self) -> u32 {
        self.major
    }
    pub fn minor(&self) -> u32 {
        self.minor
    }
    pub fn patch(&self) -> u32 {
        self.patch
    }
    pub fn is_compatible_with(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor
    }
}

impl Display for Version {
    fn fmt(&self, fmtr: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmtr, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FromStr for Version {
    type Err = String;

    fn from_str(s: &str) -> Result<Version, Self::Err> {
        let parts: Vec<Result<u32, &str>> = s
            .split('.')
            .map(|elm| elm.parse::<u32>().map_err(|_| elm))
            .collect();

        if parts.len() != 3 {
            return Err(format!(
                "Invalid version format: expected 3 components, got {}.",
                parts.len()
            ));
        }

        for part in &parts {
            match part {
                &Err(err) => {
                    return Err(format!(
                        "Invalid version format: expected integer, got '{}'.",
                        err
                    ))
                }
                _ => {}
            }
        }

        Ok(Version {
            major: parts[0].unwrap(),
            minor: parts[1].unwrap(),
            patch: parts[2].unwrap(),
        })
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.major > other.major {
            return Some(std::cmp::Ordering::Greater);
        } else if self.major < other.major {
            return Some(std::cmp::Ordering::Less);
        }

        if self.minor > other.minor {
            return Some(std::cmp::Ordering::Greater);
        } else if self.minor < other.minor {
            return Some(std::cmp::Ordering::Less);
        }

        if self.patch > other.patch {
            return Some(std::cmp::Ordering::Greater);
        } else if self.patch < other.patch {
            return Some(std::cmp::Ordering::Less);
        }

        Some(std::cmp::Ordering::Equal)
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

struct VersionVisitor;
impl<'de> Visitor<'de> for VersionVisitor {
    type Value = Version;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a version string (<major>.<minor>.<patch>)")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match Version::from_str(&value) {
            Ok(v) => Ok(v),
            Err(err) => Err(E::custom(err)),
        }
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(VersionVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_de_tokens, assert_ser_tokens, Token};

    use super::*;

    #[test]
    fn test_cmp() {
        let v = Version::from_str("1.0.0").unwrap();
        let v_g1 = Version::from_str("1.0.1").unwrap();
        let v_g2 = Version::from_str("1.1.0").unwrap();
        let v_g3 = Version::from_str("2.0.0").unwrap();
        let v_g4 = Version::from_str("2.1.1").unwrap();

        let v_l1 = Version::from_str("0.0.9").unwrap();
        let v_l2 = Version::from_str("0.9.0").unwrap();
        let v_l3 = Version::from_str("0.9.9").unwrap();

        assert!(v < v_g1);
        assert!(v < v_g2);
        assert!(v < v_g3);
        assert!(v < v_g4);

        assert!(v > v_l1);
        assert!(v > v_l2);
        assert!(v > v_l3);
    }

    #[test]
    fn test_eq() {
        let v = Version::from_str("1.0.0").unwrap();
        let v2 = Version::from_str("1.0.0").unwrap();

        assert_eq!(v, v2);
    }

    #[test]
    fn test_serialization() {
        let v = Version::from_str("1.0.0").unwrap();

        assert_ser_tokens(&v, &[Token::Str("1.0.0")]);
        assert_de_tokens(&v, &[Token::Str("1.0.0")]);
    }
}
