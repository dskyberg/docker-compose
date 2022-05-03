use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;
use void::Void;

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct Service {
    pub image: Option<String>,
    #[serde(deserialize_with = "string_or_struct")]
    pub build: Option<Build>,
}

type Args = std::collections::HashMap<String, String>;

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct Build {
    pub context: String,
    pub dockerfile: Option<String>,
    #[serde(default)]
    pub args: Args,
}

impl FromStr for Build {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Build {
            context: s.to_string(),
            dockerfile: None,
            args: Args::new(),
        })
    }
}

pub fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Void>,
    D: Deserializer<'de>,
{
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = Void>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        let s = r#"---
        image: busybox
        build: build_1
        "#;

        let yaml: Service = serde_yaml::from_str(s).unwrap();
        dbg!(&yaml);
    }
}
