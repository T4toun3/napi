extern crate derive_macro;
use derive_macro::NewTable;

use crate::string_utils::*;

use crate::doujin::Tag;

pub trait Table {
    fn search_tag(html: &str) -> Option<Vec<Tag>>;
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
    fn search_tag(html: &str) -> Option<Vec<Tag>> {
        Some(
            html
           .after(r#"<div class="container" id="tag-container">"#)
           .before(r#"</div>"#)?
           .split(r#"<section"#)
           .map(|x| x.split(r#"<a href="#).collect::<Vec<&str>>())
           .flatten()
           .collect::<Vec<&str>>()
                .into_iter()
                .flat_map(|x| {
                    Some(Tag {
                        id: x[x.find("tag tag-")? + 8..x.find(r#" "><span"#)?]
                            .parse::<u32>()
                            .ok()?,
                        _type: "tag".to_owned(),
                        name: x[x.find(r#""name">"#)? + 7..x.find("</span><span")?].to_owned(),
                        url: x[x.find("\"")? + 1..x.find(r#"" class="tag"#)?].to_owned(),
                        count: x[x.find(r#""count">"#)? + 8..x.find("</span></a>")?]
                            .replace("K", "000")
                            .parse::<u32>()
                            .ok()?,
                    })
                })
                .collect::<Vec<Tag>>(),
        )
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
        self.tags.sort_by_key(|x| x.count)
    }

    fn sort_by_alphabetical(&mut self) {
        self.tags.sort_by_cached_key(|x| x.name.to_owned())
    }
}


// ArtistTable
#[derive(Debug, NewTable)]
pub struct ArtistTable {
    pub artists: Vec<Tag>
}

impl Table for ArtistTable {
    fn search_tag(html: &str) -> Option<Vec<Tag>> {
        Some(
            html
            .after(r#"<div class="container" id="tag-container">"#)
            .before(r#"</div>"#)?
            .split(r#"<section"#)
            .map(|x| x.split(r#"<a href="#).collect::<Vec<&str>>())
            .flatten()
            .collect::<Vec<&str>>()
                .into_iter()
                .flat_map(|x| {
                    Some(Tag {
                        id: x[x.find("tag tag-")? + 8..x.find(r#" "><span"#)?]
                            .parse::<u32>()
                            .ok()?,
                        _type: "artist".to_owned(),
                        name: x[x.find(r#""name">"#)? + 7..x.find("</span><span")?].to_owned(),
                        url: x[x.find("\"")? + 1..x.find(r#"" class="tag"#)?].to_owned(),
                        count: x[x.find(r#""count">"#)? + 8..x.find("</span></a>")?]
                            .replace("K", "000")
                            .parse::<u32>()
                            .ok()?,
                    })
                })
                .collect::<Vec<Tag>>(),
        )
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
        self.artists.sort_by_key(|x| x.count)
    }

    fn sort_by_alphabetical(&mut self) {
        self.artists.sort_by_key(|x| x.name.to_owned())
    }
}

// CharacterTable
#[derive(Debug, NewTable)]
pub struct CharacterTable {
    pub characters: Vec<Tag>,
}

impl Table for CharacterTable {
    fn search_tag(html: &str) -> Option<Vec<Tag>> {
        Some(
            html
            .after(r#"<div class="container" id="tag-container">"#)
            .before(r#"</div>"#)?
            .split(r#"<section"#)
            .map(|x| x.split(r#"<a href="#).collect::<Vec<&str>>())
            .flatten()
            .collect::<Vec<&str>>()
                .into_iter()
                .flat_map(|x| {
                    Some(Tag {
                        id: x[x.find("tag tag-")? + 8..x.find(r#" "><span"#)?]
                            .parse::<u32>()
                            .ok()?,
                        _type: "character".to_owned(),
                        name: x[x.find(r#""name">"#)? + 7..x.find("</span><span")?].to_owned(),
                        url: x[x.find("\"")? + 1..x.find(r#"" class="tag"#)?].to_owned(),
                        count: x[x.find(r#""count">"#)? + 8..x.find("</span></a>")?]
                            .replace("K", "000")
                            .parse::<u32>()
                            .ok()?,
                    })
                })
                .collect::<Vec<Tag>>(),
        )
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
        self.characters.sort_by_key(|x| x.count)
    }

    fn sort_by_alphabetical(&mut self) {
        self.characters.sort_by_key(|x| x.name.to_owned())
    }
}


// ParodieTable
#[derive(Debug, NewTable)]
pub struct ParodieTable {
    pub parodies: Vec<Tag>,
}

impl Table for ParodieTable {
    fn search_tag(html: &str) -> Option<Vec<Tag>> {
        Some(
            html
            .after(r#"<div class="container" id="tag-container">"#)
            .before(r#"</div>"#)?
            .split(r#"<section"#)
            .map(|x| x.split(r#"<a href="#).collect::<Vec<&str>>())
            .flatten()
            .collect::<Vec<&str>>()
                .into_iter()
                .flat_map(|x| {
                    Some(Tag {
                        id: x[x.find("tag tag-")? + 8..x.find(r#" "><span"#)?]
                            .parse::<u32>()
                            .ok()?,
                        _type: "parodie".to_owned(),
                        name: x[x.find(r#""name">"#)? + 7..x.find("</span><span")?].to_owned(),
                        url: x[x.find("\"")? + 1..x.find(r#"" class="tag"#)?].to_owned(),
                        count: x[x.find(r#""count">"#)? + 8..x.find("</span></a>")?]
                            .replace("K", "000")
                            .parse::<u32>()
                            .ok()?,
                    })
                })
                .collect::<Vec<Tag>>(),
        )
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
        self.parodies.sort_by_key(|x| x.count)
    }

    fn sort_by_alphabetical(&mut self) {
        self.parodies.sort_by_key(|x| x.name.to_owned())
    }
}



// ParodieTable
#[derive(Debug, NewTable)]
pub struct GroupTable {
    pub groups: Vec<Tag>,
}

impl Table for GroupTable {
    fn search_tag(html: &str) -> Option<Vec<Tag>> {
        Some(
            html
            .after(r#"<div class="container" id="tag-container">"#)
            .before(r#"</div>"#)?
            .split(r#"<section"#)
            .map(|x| x.split(r#"<a href="#).collect::<Vec<&str>>())
            .flatten()
            .collect::<Vec<&str>>()
                .into_iter()
                .flat_map(|x| {
                    Some(Tag {
                        id: x[x.find("tag tag-")? + 8..x.find(r#" "><span"#)?]
                            .parse::<u32>()
                            .ok()?,
                        _type: "parodie".to_owned(),
                        name: x[x.find(r#""name">"#)? + 7..x.find("</span><span")?].to_owned(),
                        url: x[x.find("\"")? + 1..x.find(r#"" class="tag"#)?].to_owned(),
                        count: x[x.find(r#""count">"#)? + 8..x.find("</span></a>")?]
                            .replace("K", "000")
                            .parse::<u32>()
                            .ok()?,
                    })
                })
                .collect::<Vec<Tag>>(),
        )
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
        self.groups.sort_by_key(|x| x.count)
    }

    fn sort_by_alphabetical(&mut self) {
        self.groups.sort_by_key(|x| x.name.to_owned())
    }
}
