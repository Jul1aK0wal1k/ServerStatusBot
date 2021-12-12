pub mod u16_as_i32 {
    use serde::{de, Deserialize, Deserializer, Serializer};

    /// Deserializes a u16 from an i32 (BSON int). Errors if an exact conversion is not possible.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<u16, D::Error>
    where
        D: Deserializer<'de>,
    {
        let f = i32::deserialize(deserializer)?;
        if f - f as u16 as i32 == 0 {
            Ok(f as u16)
        } else {
            Err(de::Error::custom(format!("cannot convert i32 (BSON int) {} to u16", f)))
        }
    }

    /// Serializes a u16 as an i32 (BSON int).
    pub fn serialize<S: Serializer>(val: &u16, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(*val as i32)
    }
}
