extern crate derive_macro;
use derive_macro::{NewTable, Table};

use crate::string_utils::*;

use crate::doujin::Tag;

pub trait Table {
    fn get(&self, index: usize) -> Option<&Tag>;
    fn get_mut(&mut self, index: usize) -> Option<&mut Tag>;
    fn get_by_id(&self, id: u32) -> Option<&Tag>;
    fn get_by_name(&self, name: &str) -> Option<&Tag>;
    fn max(&self) -> Option<&Tag>;
    fn min(&self) -> Option<&Tag>;
    fn sort_by_popularity(&mut self);
    fn sort_by_alphabetical(&mut self);
}


// TagTable
#[derive(Debug, NewTable, Table)]
pub struct TagTable {
    pub tags: Vec<Tag>,
}


// ArtistTable
#[derive(Debug, NewTable, Table)]
pub struct ArtistTable {
    pub artists: Vec<Tag>
}


// CharacterTable
#[derive(Debug, NewTable, Table)]
pub struct CharacterTable {
    pub characters: Vec<Tag>,
}


// ParodieTable
#[derive(Debug, NewTable, Table)]
pub struct ParodieTable {
    pub parodies: Vec<Tag>,
}


// ParodieTable
#[derive(Debug, NewTable, Table)]
pub struct GroupTable {
    pub groups: Vec<Tag>,
}



// LanguageTable
#[derive(Debug, Table)]
pub struct LanguageTable {
    pub languages: Vec<Tag>
}

impl LanguageTable {
    pub fn new() -> Option<Self> {
        use rayon::prelude::*;
        use std::fs::File;
        use std::io::prelude::*;

        #[derive(serde::Deserialize)]
        struct Json {
            language: Vec<String>,
        }

        let mut file = File::open("data/ungetable.json").ok()?;

        let mut string = String::new();
        file.read_to_string(&mut string).ok()?;

        let json: Json = serde_json::from_str(&string).ok()?;

        let vec_html = json.language.iter().flat_map(|l| {
            Some(reqwest::blocking::get(&format!("https://nhentai.net/language/{}/", l)).ok()?.text().ok()?)
        }).collect::<Vec<String>>();

        Some(
            Self {
                languages: vec_html
                    .par_iter()
                    .flat_map(|x| {
                        if let Some(t) = Self::search_tag(x) {
                            Some(t)
                        } else {
                            println!("Error while searshing language {}", x);
                            None
                        }
                    })
                    .collect::<Vec<Tag>>()
            }
        )
    }

    pub fn new_by_popularity() -> Option<Self> {
        use rayon::prelude::*;
        use std::fs::File;
        use std::io::prelude::*;

        #[derive(serde::Deserialize)]
        struct Json {
            language: Vec<String>,
        }

        let mut file = File::open("data/ungetable.json").ok()?;

        let mut string = String::new();
        file.read_to_string(&mut string).ok()?;

        let json: Json = serde_json::from_str(&string).ok()?;

        let vec_html = json.language.iter().flat_map(|l| {
            Some(reqwest::blocking::get(&format!("https://nhentai.net/language/{}/", l)).ok()?.text().ok()?)
        }).collect::<Vec<String>>();

        let mut table = Self {
                languages: vec_html
                    .par_iter()
                    .flat_map(|x| {
                        if let Some(t) = Self::search_tag(x) {
                            Some(t)
                        } else {
                            println!("Error while searshing language {}", x);
                            None
                        }
                    })
                    .collect::<Vec<Tag>>()
            };
        table.sort_by_popularity();
        Some(table)
    }

    fn search_tag(html: &str) -> Option<Tag> {
        let html = html.between("<h1>", "</h1>").between(r#"<a href=""#, "</span></a>")?.to_owned();
        Some(
            Tag {
                id: html.between("tag tag-", r#" "><span"#)?.parse::<u32>().ok()?,
                _type: "language".to_owned(),
                name: html.between("/language/", r#"/" class="tag"#)?.to_owned(),
                url: html.before(r#"" class="tag"#)?.to_owned(),
                count: html.after(r#"="count">"#)?.replace("K", "000").parse::<u32>().ok()?
            }
        )
    }

}


// LanguageTable
#[derive(Debug, Table)]
pub struct CategoryTable {
    pub categories: Vec<Tag>
}

impl CategoryTable {
    pub fn new() -> Option<Self> {
        use rayon::prelude::*;
        use std::fs::File;
        use std::io::prelude::*;

        #[derive(serde::Deserialize)]
        struct Json {
            category: Vec<String>,
        }

        let mut file = File::open("data/ungetable.json").ok()?;

        let mut string = String::new();
        file.read_to_string(&mut string).ok()?;

        let json: Json = serde_json::from_str(&string).ok()?;

        let vec_html = json.category.iter().flat_map(|l| {
            Some(reqwest::blocking::get(&format!("https://nhentai.net/category/{}/", l)).ok()?.text().ok()?)
        }).collect::<Vec<String>>();

        Some(
            Self {
                categories: vec_html
                    .par_iter()
                    .flat_map(|x| {
                        if let Some(t) = Self::search_tag(x) {
                            Some(t)
                        } else {
                            println!("Error while searshing category {}", x);
                            None
                        }
                    })
                    .collect::<Vec<Tag>>()
            }
        )
    }

    pub fn new_by_popularity() -> Option<Self> {
        use rayon::prelude::*;
        use std::fs::File;
        use std::io::prelude::*;

        #[derive(serde::Deserialize)]
        struct Json {
            category: Vec<String>,
        }

        let mut file = File::open("data/ungetable.json").ok()?;

        let mut string = String::new();
        file.read_to_string(&mut string).ok()?;

        let json: Json = serde_json::from_str(&string).ok()?;

        let vec_html = json.category.iter().flat_map(|l| {
            Some(reqwest::blocking::get(&format!("https://nhentai.net/category/{}/", l)).ok()?.text().ok()?)
        }).collect::<Vec<String>>();

        let mut table = Self {
                categories: vec_html
                    .par_iter()
                    .flat_map(|x| {
                        if let Some(t) = Self::search_tag(x) {
                            Some(t)
                        } else {
                            println!("Error while searshing language {}", x);
                            None
                        }
                    })
                    .collect::<Vec<Tag>>()
            };
        table.sort_by_popularity();
        Some(table)
    }

    fn search_tag(html: &str) -> Option<Tag> {
        let html = html.between("<h1>", "</h1>").between(r#"<a href=""#, "</span></a>")?.to_owned();
        Some(
            Tag {
                id: html.between("tag tag-", r#" "><span"#)?.parse::<u32>().ok()?,
                _type: "language".to_owned(),
                name: html.between("/language/", r#"/" class="tag"#)?.to_owned(),
                url: html.before(r#"" class="tag"#)?.to_owned(),
                count: html.after(r#"="count">"#)?.replace("K", "000").parse::<u32>().ok()?
            }
        )
    }

}

