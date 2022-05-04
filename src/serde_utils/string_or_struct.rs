use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;
use void::Void;

/// Support 'short' and 'long' versions of objects
///
/// Often, an object can be expressed as a single string or
/// as a full object with attributes.  This function enable
/// parsing any struct that implements FromStr.  The semantics
/// of the parsing from string to struct is managed by the struct
/// itself.  
pub fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Void>,
    D: Deserializer<'de>,
{
    struct StringOrStruct<T>(PhantomData<T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = Void>,
    {
        type Value = T;

        // If the value is a string, use the objects FromStr impl
        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        // If the value is a map, pass it to Serde's Map deserializer
        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }

        // If the value is neither a string or a map, present an appropriate error
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}
