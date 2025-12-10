use alox_48::Value;
use serde::de::Error;

pub struct DeserializeValue(pub Value);

struct Visitor;

impl<'de> serde::de::Visitor<'de> for Visitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("any value")
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::Nil)
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::Bool(v))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::Float(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Integer(v as _))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Integer(v as _))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::String(alox_48::RbString::from(v)))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::String(alox_48::RbString::from(v)))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut array = alox_48::RbArray::new();
        while let Some(value) = seq.next_element::<DeserializeValue>()? {
            array.push(value.0);
        }
        Ok(Value::Array(array))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let Some(key): Option<String> = map.next_key()? else {
            return Err(A::Error::custom("expected a key"));
        };
        let value = match key.as_str() {
            "$symbol" => Value::Symbol(map.next_value::<String>()?.into()),
            "$hash" => Value::Hash(map.next_value::<DeserializeHash>()?.0),
            "$userdata" => Value::Userdata(map.next_value::<DeserializeUserdata>()?.into()),
            "$object" => Value::Object(map.next_value::<DeserializeObject>()?.into()),
            "$instance" => Value::Instance(map.next_value::<DeserializeInstance>()?.into()),
            "$regex" => map.next_value::<DeserializeRegex>()?.into(),
            "$struct" => Value::RbStruct(map.next_value::<DeserializeStruct>()?.into()),
            "$class" => Value::Class(map.next_value::<String>()?.into()),
            "$module" => Value::Module(map.next_value::<String>()?.into()),
            "$extended" => map.next_value::<DeserializeExtended>()?.into(),
            "$userclass" => map.next_value::<DeserializeUsertype>()?.into_uclass(),
            "$usermarshal" => map.next_value::<DeserializeUsertype>()?.into_umarshal(),
            "$cdata" => map.next_value::<DeserializeUsertype>()?.into_cdata(),
            "$string" => Value::String(map.next_value::<DeserializeBytes>()?.0.into()),
            _ => return Err(A::Error::custom("invalid data type")),
        };

        Ok(value)
    }
}

impl<'de> serde::Deserialize<'de> for DeserializeValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(Visitor).map(Self)
    }
}

struct DeserializeBytes(Vec<u8>);

struct BytesVisitor;

impl<'de> serde::de::Visitor<'de> for BytesVisitor {
    type Value = Vec<u8>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a byte buffer")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v.to_vec())
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut buf = vec![];
        while let Some(next) = seq.next_element()? {
            buf.push(next);
        }
        Ok(buf)
    }
}

impl<'de> serde::Deserialize<'de> for DeserializeBytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if crate::binary_bytes_allowed() {
            deserializer.deserialize_byte_buf(BytesVisitor).map(Self)
        } else {
            <Vec<u8>>::deserialize(deserializer).map(Self)
        }
    }
}

struct DeserializeString(alox_48::RbString);

struct StringVisitor;

impl<'de> serde::de::Visitor<'de> for StringVisitor {
    type Value = alox_48::RbString;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a ruby string")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v.into())
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v.into())
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let Some(key): Option<String> = map.next_key()? else {
            return Err(A::Error::custom("expected a key"));
        };

        if key != "$string" {
            return Err(A::Error::custom("expected key to be $string"));
        }

        Ok(map.next_value::<DeserializeBytes>()?.0.into())
    }
}

impl<'de> serde::Deserialize<'de> for DeserializeString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(StringVisitor).map(Self)
    }
}

struct DeserializeHash(alox_48::RbHash);
struct HashVisitor;

#[derive(serde::Deserialize)]
struct DeserializeKV {
    key: DeserializeValue,
    value: DeserializeValue,
}

impl<'de> serde::de::Visitor<'de> for HashVisitor {
    type Value = alox_48::RbHash;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("an array of key value pairs")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut hash = alox_48::RbHash::new();
        while let Some(DeserializeKV { key, value }) = seq.next_element()? {
            hash.insert(key.0, value.0);
        }
        Ok(hash)
    }
}

impl<'de> serde::Deserialize<'de> for DeserializeHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(HashVisitor).map(Self)
    }
}

struct DeserializeFields(alox_48::RbFields);

struct FieldsVisitor;

impl<'de> serde::de::Visitor<'de> for FieldsVisitor {
    type Value = alox_48::RbFields;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut fields = alox_48::RbFields::new();
        while let Some((k, v)) = map.next_entry::<String, DeserializeValue>()? {
            fields.insert(alox_48::Symbol::from(k), v.0);
        }
        Ok(fields)
    }
}

impl<'de> serde::Deserialize<'de> for DeserializeFields {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(FieldsVisitor).map(Self)
    }
}

#[derive(serde::Deserialize)]
#[serde(rename = "Userdata")]
struct DeserializeUserdata {
    class: String,
    data: DeserializeBytes,
}

impl From<DeserializeUserdata> for alox_48::Userdata {
    fn from(val: DeserializeUserdata) -> Self {
        alox_48::Userdata {
            class: val.class.into(),
            data: val.data.0,
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(rename = "Object")]
struct DeserializeObject {
    class: String,
    fields: DeserializeFields,
}

impl From<DeserializeObject> for alox_48::Object {
    fn from(val: DeserializeObject) -> Self {
        alox_48::Object {
            class: val.class.into(),
            fields: val.fields.0,
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(rename = "Instance")]
struct DeserializeInstance {
    value: DeserializeValue,
    fields: DeserializeFields,
}

impl From<DeserializeInstance> for alox_48::Instance<Box<Value>> {
    fn from(val: DeserializeInstance) -> Self {
        alox_48::Instance {
            value: Box::new(val.value.0),
            fields: val.fields.0,
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(rename = "Regex")]
struct DeserializeRegex {
    data: DeserializeString,
    flags: u8,
}

impl From<DeserializeRegex> for alox_48::Value {
    fn from(val: DeserializeRegex) -> Self {
        alox_48::Value::Regex {
            data: val.data.0,
            flags: val.flags,
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(rename = "Struct")]
struct DeserializeStruct {
    class: String,
    fields: DeserializeFields,
}

impl From<DeserializeStruct> for alox_48::RbStruct {
    fn from(val: DeserializeStruct) -> Self {
        alox_48::RbStruct {
            class: val.class.into(),
            fields: val.fields.0,
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(rename = "Extended")]
struct DeserializeExtended {
    module: String,
    value: DeserializeValue,
}

impl From<DeserializeExtended> for alox_48::Value {
    fn from(val: DeserializeExtended) -> Self {
        alox_48::Value::Extended {
            module: val.module.into(),
            value: Box::new(val.value.0),
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(rename = "Usertype")]
struct DeserializeUsertype {
    class: String,
    value: DeserializeValue,
}

impl DeserializeUsertype {
    fn into_uclass(self) -> alox_48::Value {
        alox_48::Value::UserClass {
            class: self.class.into(),
            value: Box::new(self.value.0),
        }
    }

    fn into_umarshal(self) -> alox_48::Value {
        alox_48::Value::UserMarshal {
            class: self.class.into(),
            value: Box::new(self.value.0),
        }
    }

    fn into_cdata(self) -> alox_48::Value {
        alox_48::Value::Data {
            class: self.class.into(),
            value: Box::new(self.value.0),
        }
    }
}
