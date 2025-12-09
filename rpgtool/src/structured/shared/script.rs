#[derive(serde::Serialize, serde::Deserialize)]
pub struct Script {
    pub id: u32, // no idea how the editor generates these and they don't seem to be used
    pub name: String,
    pub text: String,
}

impl<'de> alox_48::Deserialize<'de> for Script {
    fn deserialize<D>(deserializer: D) -> alox_48::DeResult<Self>
    where
        D: alox_48::DeserializerTrait<'de>,
    {
        struct Visitor;

        impl<'de> alox_48::de::Visitor<'de> for Visitor {
            type Value = Script;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("an array")
            }

            #[allow(clippy::unwrap_used)]
            fn visit_array<A>(self, mut array: A) -> alox_48::DeResult<Self::Value>
            where
                A: alox_48::ArrayAccess<'de>,
            {
                use std::io::Read;
                if array.len() != 3 {
                    let error =
                        alox_48::DeError::invalid_length(array.len(), &"an array of length 3");
                    return Err(error);
                }

                // we validated the array length earlier
                let id = array.next_element()?.unwrap();
                let name = array.next_element()?.unwrap();
                let data = array.next_element::<alox_48::RbString>()?.unwrap();

                let mut decoder = flate2::bufread::ZlibDecoder::new(data.as_slice());
                let mut text = String::new();
                decoder
                    .read_to_string(&mut text)
                    .map_err(alox_48::DeError::custom)?;

                Ok(Script { id, name, text })
            }
        }

        deserializer.deserialize(Visitor)
    }
}

impl alox_48::Serialize for Script {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, alox_48::SerError>
    where
        S: alox_48::SerializerTrait,
    {
        use alox_48::SerializeArray;
        use std::io::Write;

        let mut array = serializer.serialize_array(3)?;

        let mut encoder =
            flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        let data = encoder
            .write_all(self.text.as_bytes())
            .and_then(|()| encoder.finish())
            .map_err(alox_48::SerError::custom)?;

        array.serialize_element(&self.id)?;
        array.serialize_element(&self.name)?;
        array.serialize_element(&alox_48::RbString { data })?;

        array.end()
    }
}
