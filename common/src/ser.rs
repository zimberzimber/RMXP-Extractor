use alox_48::Value;
use serde::ser::SerializeMap;

pub struct SerializeValue<'a>(pub &'a Value);

struct SerializeHash<'a>(&'a alox_48::RbHash);

impl<'a> serde::Serialize for SerializeHash<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (k, v) in self.0 {
            map.serialize_entry(&SerializeValue(k), &SerializeValue(v))?;
        }
        map.end()
    }
}

struct SerializeFields<'a>(&'a alox_48::RbFields);

impl<'a> serde::Serialize for SerializeFields<'a> {
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

struct SerializeUserdata<'a>(&'a alox_48::Userdata);

impl<'a> serde::Serialize for SerializeUserdata<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("class", self.0.class.as_str())?;
        map.serialize_entry("data", &self.0.data)?;
        map.end()
    }
}

struct SerializeObject<'a>(&'a alox_48::Object);

impl<'a> serde::Serialize for SerializeObject<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("class", self.0.class.as_str())?;
        map.serialize_entry("fields", &SerializeFields(&self.0.fields))?;
        map.end()
    }
}

struct SerializeStruct<'a>(&'a alox_48::RbStruct);

impl<'a> serde::Serialize for SerializeStruct<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("class", self.0.class.as_str())?;
        map.serialize_entry("fields", &SerializeFields(&self.0.fields))?;
        map.end()
    }
}

struct SerializeInstance<'a>(&'a alox_48::Instance<Box<Value>>);

impl<'a> serde::Serialize for SerializeInstance<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("value", &SerializeValue(&self.0.value))?;
        map.serialize_entry("fields", &SerializeFields(&self.0.fields))?;
        map.end()
    }
}

struct SerializeRegex<'a>(&'a alox_48::RbString, &'a u8);

impl<'a> serde::Serialize for SerializeRegex<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        let string = self.0.to_string_lossy(); // FIXME BAD DOES NOT ACCOUNT FOR NON-UTF8
        map.serialize_entry("data", &string)?;
        map.serialize_entry("flags", self.1)?;
        map.end()
    }
}

struct SerializeExtended<'a>(&'a alox_48::Symbol, &'a Value);

impl<'a> serde::Serialize for SerializeExtended<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("module", self.0.as_str())?;
        map.serialize_entry("value", &SerializeValue(self.1))?;
        map.end()
    }
}

struct SerializeUsertype<'a>(&'a alox_48::Symbol, &'a Value);

impl<'a> serde::Serialize for SerializeUsertype<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("class", self.0.as_str())?;
        map.serialize_entry("value", &SerializeValue(self.1))?;
        map.end()
    }
}

#[allow(unused)]
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
            Value::String(rb_string) => {
                let string = rb_string.to_string_lossy(); // FIXME BAD DOES NOT ACCOUNT FOR NON-UTF8
                serializer.serialize_str(&string)
            }
            Value::Symbol(symbol) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("$symbol", symbol.as_str())?;
                map.end()
            }
            Value::Array(values) => serializer.collect_seq(values.iter().map(Self)),
            Value::Hash(index_map) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_key("$hash")?;
                map.serialize_value(&SerializeHash(index_map));
                map.end()
            }
            Value::Userdata(userdata) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_key("$userdata")?;
                map.serialize_value(&SerializeUserdata(userdata));
                map.end()
            }
            Value::Object(object) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_key("$object")?;
                map.serialize_value(&SerializeObject(object));
                map.end()
            }
            Value::Instance(instance) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_key("$instance")?;
                map.serialize_value(&SerializeInstance(instance));
                map.end()
            }
            Value::Regex { data, flags } => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_key("$regex")?;
                map.serialize_value(&SerializeRegex(data, flags))?;
                map.end()
            }
            Value::RbStruct(rb_struct) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_key("$struct")?;
                map.serialize_value(&SerializeStruct(rb_struct));
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
                map.serialize_entry("$extended", &SerializeExtended(module, value))?;
                map.end()
            }
            Value::UserClass { class, value } => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("$userclass", &SerializeUsertype(class, value))?;
                map.end()
            }
            Value::UserMarshal { class, value } => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("$usermarshal", &SerializeUsertype(class, value))?;
                map.end()
            }
            Value::Data { class, value } => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("$cdata", &SerializeUsertype(class, value))?;
                map.end()
            }
        }
    }
}
