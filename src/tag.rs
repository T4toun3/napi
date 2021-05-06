
use serde::Deserialize;

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

impl Default for Tag {
    fn default() -> Self {
        Self {
            id: 0,
            _type: TagType::Tag,
            name: "".to_owned(),
            url: "".to_owned(),
            count: 1,
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(from = "String")]
pub enum TagType {
    Parody,
    Character,
    Tag,
    Artist,
    Group,
    Language,
    Category,
}

impl Display for TagType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TagType::Parody => write!(f, "parodie"),
            TagType::Character => write!(f, "character"),
            TagType::Artist => write!(f, "artist"),
            TagType::Tag => write!(f, "tag"),
            TagType::Group => write!(f, "group"),
            TagType::Language => write!(f, "language"),
            TagType::Category => write!(f, "category"),
        }
    }
}

impl From<String> for TagType {
    fn from(s: String) -> Self {
        match s.as_ref() {
            "parody" => Self::Parody,
            "character" => Self::Character,
            "artist" => Self::Artist,
            "group" => Self::Group,
            "category" => Self::Category,
            "language" => Self::Language,
            _ => Self::Tag,
        }
    }
}

impl From<&str> for TagType {
    fn from(s: &str) -> Self {
        match s {
            "artist" => Self::Artist,
            "character" => Self::Character,
            "parody" => Self::Parody,
            "group" => Self::Group,
            "category" => Self::Category,
            _ => Self::Tag,
        }
    }
}
