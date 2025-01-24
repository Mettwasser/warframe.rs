use core::str;
use std::fmt;

use serde::{
    de,
    de::Visitor,
    Deserialize,
    Deserializer,
    Serialize,
};

#[derive(Debug, Clone)]
pub struct PlatformName {
    /// name
    pub name: String,

    /// platform
    pub platform: Platform,
}

impl<'de> Deserialize<'de> for PlatformName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SplitFieldVisitor;

        impl<'de> Visitor<'de> for SplitFieldVisitor {
            type Value = PlatformName;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string with a byte code for platform at the end")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let mut chars = value.chars();
                let platform_char = chars.next_back().ok_or_else(|| E::custom("value cannot be empty"))?;

                let mut platform = Platform::from_char(platform_char);

                let mut name = chars.collect();
                if platform.is_none() {
                    // Older profiles don't have this field, so we default to PC
                    platform = Some(Platform::PC);
                    name = format!("{}{}", name, platform_char);
                }

                Ok(PlatformName { name, platform: platform.unwrap() })
            }
        }

        deserializer.deserialize_str(SplitFieldVisitor)
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Platform {
    PC,
    Xbox,
    PS,
    Switch,
    Ios,
}

impl Platform {
    pub fn to_char(&self) -> char {
        match self {
            Self::PC => '\u{e000}',
            Self::Xbox => '\u{e001}',
            Self::PS => '\u{e002}',
            Self::Switch => '\u{e003}',
            Self::Ios => '\u{e004}',
        }
    }
    
    pub fn from_char(char: char) -> Option<Self> {
        match char {
            '\u{e000}' => Some(Self::PC),
            '\u{e001}' => Some(Self::Xbox),
            '\u{e002}' => Some(Self::PS),
            '\u{e003}' => Some(Self::Switch),
            '\u{e004}' => Some(Self::Ios),
            _ => None,
        }
    }
}
