use alox_48::Value;
use serde::ser::{SerializeMap, SerializeSeq};

pub struct SerializeValue<'a>(pub &'a Value);

struct SerializeHash<'a>(&'a alox_48::RbHash);

struct SerializeKV<'a>(&'a alox_48::Value, &'a alox_48::Value);

impl serde::Serialize for SerializeKV<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("key", &SerializeValue(self.0))?;
        map.serialize_entry("value", &SerializeValue(self.1))?;
        map.end()
    }
}

impl serde::Serialize for SerializeHash<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for (k, v) in self.0 {
            seq.serialize_element(&SerializeKV(k, v))?;
        }
        seq.end()
    }
}

struct SerializeFields<'a>(&'a alox_48::RbFields);

impl serde::Serialize for SerializeFields<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (k, v) in self.0 {
            map.serialize_entry(k.as_str(), &SerializeValue(v))?;
        }
        map.end()
    }
}

struct SerializeString<'a>(&'a alox_48::RbString);

// FIXME figure out how to recover from serialization errors to try using serialize_seq instead
struct SerializeBytes<'a>(&'a [u8]);

impl serde::Serialize for SerializeBytes<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if crate::binary_bytes_allowed() {
            serializer.serialize_bytes(self.0)
        } else {
            serde::Serialize::serialize(self.0, serializer) // serializes as a seq instead
        }
    }
}

impl serde::Serialize for SerializeString<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Ok(string) = str::from_utf8(&self.0.data) {
            serializer.serialize_str(string)
        } else {
            let mut map = serializer.serialize_map(Some(1))?;
            map.serialize_entry("$string", &SerializeBytes(&self.0.data))?;
            map.end()
        }
    }
}

#[derive(serde::Serialize)]
#[serde(rename = "Userdata")]
struct SerializeUserdata<'a> {
    class: &'a str,
    data: SerializeBytes<'a>,
}

impl<'a> From<&'a alox_48::Userdata> for SerializeUserdata<'a> {
    fn from(value: &'a alox_48::Userdata) -> Self {
        Self {
            class: value.class.as_str(),
            data: SerializeBytes(&value.data),
        }
    }
}

#[derive(serde::Serialize)]
#[serde(rename = "Object")]
struct SerializeObject<'a> {
    class: &'a str,
    fields: SerializeFields<'a>,
}

impl<'a> From<&'a alox_48::Object> for SerializeObject<'a> {
    fn from(value: &'a alox_48::Object) -> Self {
        Self {
            class: value.class.as_str(),
            fields: SerializeFields(&value.fields),
        }
    }
}

#[derive(serde::Serialize)]
#[serde(rename = "Struct")]
struct SerializeStruct<'a> {
    class: &'a str,
    fields: SerializeFields<'a>,
}

impl<'a> From<&'a alox_48::RbStruct> for SerializeStruct<'a> {
    fn from(value: &'a alox_48::RbStruct) -> Self {
        Self {
            class: value.class.as_str(),
            fields: SerializeFields(&value.fields),
        }
    }
}

#[derive(serde::Serialize)]
#[serde(rename = "Instance")]
struct SerializeInstance<'a> {
    value: SerializeValue<'a>,
    fields: SerializeFields<'a>,
}

impl<'a> From<&'a alox_48::Instance<Box<Value>>> for SerializeInstance<'a> {
    fn from(value: &'a alox_48::Instance<Box<Value>>) -> Self {
        Self {
            value: SerializeValue(&value.value),
            fields: SerializeFields(&value.fields),
        }
    }
}

#[derive(serde::Serialize)]
#[serde(rename = "Regex")]
struct SerializeRegex<'a> {
    data: SerializeString<'a>,
    flags: u8,
}

#[derive(serde::Serialize)]
#[serde(rename = "Extended")]
struct SerializeExtended<'a> {
    module: &'a str,
    value: SerializeValue<'a>,
}

#[derive(serde::Serialize)]
#[serde(rename = "Usertype")]
struct SerializeUsertype<'a> {
    class: &'a str,
    value: SerializeValue<'a>,
}

impl serde::Serialize for SerializeValue<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self.0 {
            Value::Nil => serializer.serialize_unit(),
            Value::Bool(v) => serializer.serialize_bool(*v),
            Value::Float(v) => serializer.serialize_f64(*v),
            Value::Integer(v) => serializer.serialize_i32(*v),
            Value::String(rb_string) => SerializeString(rb_string).serialize(serializer),
            Value::Symbol(symbol) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("$symbol", symbol.as_str())?;
                map.end()
            }
            Value::Array(values) => serializer.collect_seq(values.iter().map(Self)),
            Value::Hash(index_map) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_key("$hash")?;
                map.serialize_value(&SerializeHash(index_map))?;
                map.end()
            }
            Value::Userdata(userdata) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_key("$userdata")?;
                map.serialize_value::<SerializeUserdata<'_>>(&userdata.into())?;
                map.end()
            }
            Value::Object(object) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_key("$object")?;
                map.serialize_value::<SerializeObject<'_>>(&object.into())?;
                map.end()
            }
            Value::Instance(instance) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_key("$instance")?;
                map.serialize_value::<SerializeInstance<'_>>(&instance.into())?;
                map.end()
            }
            Value::Regex { data, flags } => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_key("$regex")?;
                map.serialize_value(&SerializeRegex {
                    data: SerializeString(data),
                    flags: *flags,
                })?;
                map.end()
            }
            Value::RbStruct(rb_struct) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_key("$struct")?;
                map.serialize_value::<SerializeStruct<'_>>(&rb_struct.into())?;
                map.end()
            }
            Value::Class(symbol) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("$class", symbol.as_str())?;
                map.end()
            }
            Value::Module(symbol) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("$module", symbol.as_str())?;
                map.end()
            }
            Value::Extended { module, value } => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_key("$extended")?;
                map.serialize_value(&SerializeExtended {
                    module: module.as_str(),
                    value: SerializeValue(value),
                })?;
                map.end()
            }
            Value::UserClass { class, value } => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_key("$userclass")?;
                map.serialize_value(&SerializeUsertype {
                    class: class.as_str(),
                    value: SerializeValue(value),
                })?;
                map.end()
            }
            Value::UserMarshal { class, value } => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_key("$usermarshal")?;
                map.serialize_value(&SerializeUsertype {
                    class: class.as_str(),
                    value: SerializeValue(value),
                })?;
                map.end()
            }
            Value::Data { class, value } => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_key("$cdata")?;
                map.serialize_value(&SerializeUsertype {
                    class: class.as_str(),
                    value: SerializeValue(value),
                })?;
                map.end()
            }
        }
    }
}
