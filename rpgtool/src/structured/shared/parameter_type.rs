#![allow(clippy::unwrap_in_result, clippy::unwrap_used)]

use alox_48::Value;

use crate::structured::{AudioFile, Color, MoveCommand, MoveRoute, Tone};

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(from = "alox_48::Value", into = "alox_48::Value")] // TODO make this serde compatible
#[allow(missing_docs)]
pub enum ParameterType {
    Integer(i32),
    String(String),
    Color(Color),
    Tone(Tone),
    AudioFile(AudioFile),
    Float(f64),
    MoveRoute(MoveRoute),
    MoveCommand(MoveCommand),
    Array(Vec<ParameterType>),
    Bool(bool),
    None,
}

// FIXME this really should be try_from and try_into
impl From<alox_48::Value> for ParameterType {
    fn from(value: alox_48::Value) -> Self {
        match value {
            Value::Nil => Self::None,
            Value::Integer(v) => Self::Integer(v),
            Value::Float(v) => Self::Float(v),
            Value::String(v) => Self::String(String::from_utf8(v.data).unwrap()),
            Value::Array(v) => Self::Array(v.into_iter().map(Into::into).collect()),
            Value::Bool(v) => Self::Bool(v),
            Value::Userdata(userdata) => match userdata.class.as_str() {
                "Color" => Self::Color(Color::from(userdata)),
                "Tone" => Self::Tone(Tone::from(userdata)),
                _ => panic!("Unsupported userdata type: {userdata:#?}"),
            },
            Value::Object(alox_48::Object { ref class, .. }) => match class.as_str() {
                "RPG::AudioFile" => Self::AudioFile(alox_48::from_value(&value).unwrap()),
                "RPG::MoveRoute" => Self::MoveRoute(alox_48::from_value(&value).unwrap()),
                "RPG::MoveCommand" => Self::MoveCommand(alox_48::from_value(&value).unwrap()),
                _ => panic!("Unsupported object type: {value:#?}"),
            },
            Value::Instance(i) => (*i.value).into(),
            _ => panic!("Unsupported value type: {value:#?}"),
        }
    }
}

impl From<ParameterType> for alox_48::Value {
    fn from(value: ParameterType) -> Self {
        match value {
            ParameterType::None => Value::Nil,
            ParameterType::Integer(v) => Value::Integer(v),
            ParameterType::Float(v) => Value::Float(v),
            ParameterType::String(v) => Value::String(v.into()),
            ParameterType::Array(v) => Value::Array(v.into_iter().map(Into::into).collect()),
            ParameterType::Bool(v) => Value::Bool(v),
            ParameterType::Color(v) => Value::Userdata(v.into()),
            ParameterType::Tone(v) => Value::Userdata(v.into()),
            ParameterType::AudioFile(v) => alox_48::to_value(v).unwrap(),
            ParameterType::MoveRoute(v) => alox_48::to_value(v).unwrap(),
            ParameterType::MoveCommand(v) => alox_48::to_value(v).unwrap(),
        }
    }
}
