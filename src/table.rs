extern crate derive_macro;
use derive_macro::{NewTable};

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
#[derive(Debug, NewTable)]
pub struct TagTable {
    pub tags: Vec<Tag>,
}

impl Table for TagTable {
    fn get(&self, index: usize) -> Option<&Tag> {
        self.tags.get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut Tag> {
        self.tags.get_mut(index)
    }

    fn get_by_id(&self, id: u32) -> Option<&Tag> {
        self.tags.iter().find(|&t| t.id == id)
    }

    fn get_by_name(&self, name: &str) -> Option<&Tag> {
        self.tags.iter().find(|&t| t.name == name)
    }

    fn max(&self) -> Option<&Tag> {
        self.tags.iter().max_by_key(|t| t.count)
    }

    fn min(&self) -> Option<&Tag> {
        self.tags.iter().min_by_key(|t| t.count)
    }

    fn sort_by_popularity(&mut self) {
        self.tags.sort_unstable_by(|a, b| a.count.cmp(&b.count).reverse());
    }

    fn sort_by_alphabetical(&mut self) {
        self.tags.sort_unstable_by_key(|x| x.name.to_owned())
    }
}


// ArtistTable
#[derive(Debug, NewTable)]
pub struct ArtistTable {
    pub artists: Vec<Tag>
}

impl Table for ArtistTable {
    fn get(&self, index: usize) -> Option<&Tag> {
        self.artists.get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut Tag> {
        self.artists.get_mut(index)
    }
    
    fn get_by_id(&self, id: u32) -> Option<&Tag> {
        self.artists.iter().find(|&t| t.id == id)
    }

    fn get_by_name(&self, name: &str) -> Option<&Tag> {
        self.artists.iter().find(|&t| t.name == name)
    }

    fn max(&self) -> Option<&Tag> {
        self.artists.iter().max_by_key(|t| t.count)
    }

    fn min(&self) -> Option<&Tag> {
        self.artists.iter().min_by_key(|t| t.count)
    }

    fn sort_by_popularity(&mut self) {
        self.artists.sort_unstable_by(|a, b| a.count.cmp(&b.count).reverse());
    }

    fn sort_by_alphabetical(&mut self) {
        self.artists.sort_unstable_by_key(|x| x.name.to_owned())
    }
}

// CharacterTable
#[derive(Debug, NewTable)]
pub struct CharacterTable {
    pub characters: Vec<Tag>,
}

impl Table for CharacterTable {
    fn get(&self, index: usize) -> Option<&Tag> {
        self.characters.get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut Tag> {
        self.characters.get_mut(index)
    }
    
    fn get_by_id(&self, id: u32) -> Option<&Tag> {
        self.characters.iter().find(|&t| t.id == id)
    }

    fn get_by_name(&self, name: &str) -> Option<&Tag> {
        self.characters.iter().find(|&t| t.name == name)
    }

    fn max(&self) -> Option<&Tag> {
        self.characters.iter().max_by_key(|t| t.count)
    }

    fn min(&self) -> Option<&Tag> {
        self.characters.iter().min_by_key(|t| t.count)
    }

    fn sort_by_popularity(&mut self) {
        self.characters.sort_unstable_by(|a, b| a.count.cmp(&b.count).reverse());
    }

    fn sort_by_alphabetical(&mut self) {
        self.characters.sort_unstable_by_key(|x| x.name.to_owned())
    }
}


// ParodieTable
#[derive(Debug, NewTable)]
pub struct ParodieTable {
    pub parodies: Vec<Tag>,
}

impl Table for ParodieTable {
    fn get(&self, index: usize) -> Option<&Tag> {
        self.parodies.get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut Tag> {
        self.parodies.get_mut(index)
    }
    
    fn get_by_id(&self, id: u32) -> Option<&Tag> {
        self.parodies.iter().find(|&t| t.id == id)
    }

    fn get_by_name(&self, name: &str) -> Option<&Tag> {
        self.parodies.iter().find(|&t| t.name == name)
    }

    fn max(&self) -> Option<&Tag> {
        self.parodies.iter().max_by_key(|t| t.count)
    }

    fn min(&self) -> Option<&Tag> {
        self.parodies.iter().min_by_key(|t| t.count)
    }

    fn sort_by_popularity(&mut self) {
        self.parodies.sort_unstable_by(|a, b| a.count.cmp(&b.count).reverse());
    }

    fn sort_by_alphabetical(&mut self) {
        self.parodies.sort_unstable_by_key(|x| x.name.to_owned())
    }
}



// ParodieTable
#[derive(Debug, NewTable)]
pub struct GroupTable {
    pub groups: Vec<Tag>,
}

impl Table for GroupTable {
    fn get(&self, index: usize) -> Option<&Tag> {
        self.groups.get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut Tag> {
        self.groups.get_mut(index)
    }
    
    fn get_by_id(&self, id: u32) -> Option<&Tag> {
        self.groups.iter().find(|&t| t.id == id)
    }

    fn get_by_name(&self, name: &str) -> Option<&Tag> {
        self.groups.iter().find(|&t| t.name == name)
    }

    fn max(&self) -> Option<&Tag> {
        self.groups.iter().max_by_key(|t| t.count)
    }

    fn min(&self) -> Option<&Tag> {
        self.groups.iter().min_by_key(|t| t.count)
    }

    fn sort_by_popularity(&mut self) {
        self.groups.sort_unstable_by(|a, b| a.count.cmp(&b.count).reverse());
    }

    fn sort_by_alphabetical(&mut self) {
        self.groups.sort_unstable_by_key(|x| x.name.to_owned())
    }
}


// LanguageTable
#[derive(Debug)]
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

impl Table for LanguageTable {
    fn get(&self, index: usize) -> Option<&Tag> {
        self.languages.get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut Tag> {
        self.languages.get_mut(index)
    }
    
    fn get_by_id(&self, id: u32) -> Option<&Tag> {
        self.languages.iter().find(|&t| t.id == id)
    }

    fn get_by_name(&self, name: &str) -> Option<&Tag> {
        self.languages.iter().find(|&t| t.name == name)
    }

    fn max(&self) -> Option<&Tag> {
        self.languages.iter().max_by_key(|t| t.count)
    }

    fn min(&self) -> Option<&Tag> {
        self.languages.iter().min_by_key(|t| t.count)
    }

    fn sort_by_popularity(&mut self) {
        self.languages.sort_unstable_by(|a, b| a.count.cmp(&b.count).reverse());
    }

    fn sort_by_alphabetical(&mut self) {
        self.languages.sort_unstable_by_key(|x| x.name.to_owned())
    }
}


// LanguageTable
#[derive(Debug)]
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

impl Table for CategoryTable {
    fn get(&self, index: usize) -> Option<&Tag> {
        self.categories.get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut Tag> {
        self.categories.get_mut(index)
    }
    
    fn get_by_id(&self, id: u32) -> Option<&Tag> {
        self.categories.iter().find(|&t| t.id == id)
    }

    fn get_by_name(&self, name: &str) -> Option<&Tag> {
        self.categories.iter().find(|&t| t.name == name)
    }

    fn max(&self) -> Option<&Tag> {
        self.categories.iter().max_by_key(|t| t.count)
    }

    fn min(&self) -> Option<&Tag> {
        self.categories.iter().min_by_key(|t| t.count)
    }

    fn sort_by_popularity(&mut self) {
        self.categories.sort_unstable_by(|a, b| a.count.cmp(&b.count).reverse());
    }

    fn sort_by_alphabetical(&mut self) {
        self.categories.sort_unstable_by_key(|x| x.name.to_owned())
    }
}
