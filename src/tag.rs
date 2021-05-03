use std::convert::From;
use std::fmt::{Display, Formatter};

#[derive(serde::Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Tag {
    pub id: u32,
    #[serde(rename = "type")]
    pub _type: TagType,
    pub name: String,
    pub url: String,
    pub count: u32,
}

#[derive(serde::Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(from = "String")]
pub enum TagType {
    Tag,
    Artist,
    Character,
    Parodie,
    Group,
    Language,
    Category,
}

impl From<String> for TagType {
    fn from(s: String) -> Self {
        match s.as_ref() {
            "artist" => Self::Artist,
            "character" => Self::Character,
            "parodie" => Self::Parodie,
            "group" => Self::Group,
            "category" => Self::Category,
            _ => Self::Tag,
        }
    }
}

impl From<&str> for TagType {
    fn from(s: &str) -> Self {
        match s {
            "artist" => Self::Artist,
            "character" => Self::Character,
            "parodie" => Self::Parodie,
            "group" => Self::Group,
            "category" => Self::Category,
            _ => Self::Tag,
        }
    }
}
