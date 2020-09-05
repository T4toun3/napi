use crate::string_utils::*;

use crate::doujishi::Tag;

pub trait Table {
    fn search_tag(html: &str) -> Option<Vec<Tag>>;
    fn get_by_id(&self, id: u32) -> Option<&Tag>;
    fn get_by_name(&self, name: &str) -> Option<&Tag>;
    fn max(&self) -> Option<&Tag>;
    fn min(&self) -> Option<&Tag>;
}


// TagTable
#[derive(Debug)]
pub struct TagTable {
    pub tags: Vec<Tag>,
}

impl TagTable {
    pub fn new() -> Option<Self> {
        use rayon::prelude::*;
        let nbr_page = reqwest::blocking::get("https://nhentai.net/tags/")
            .ok()?
            .text()
            .ok()?
            .after(r#"<section class="pagination">"#)
            .before(r#"</section>"#)
            .between(r#"class="next""#, r#"" class="last""#)
            .after("page=")?
            .parse::<u32>()
            .ok()?;

        let vec_html = (1..nbr_page + 1)
            .flat_map(|x| {
                let url: String = format!("https://nhentai.net/tags/?page={}", x);
                Some(reqwest::blocking::get(&url).ok()?.text().ok()?)
            })
            .collect::<Vec<String>>();

        Some(Self {
            tags: vec_html
                .par_iter()
                .flat_map(|x| {
                    if let Some(t) = Self::search_tag(x) {
                        //println!("Succes to searching tags page {}", x);
                        Some(t)
                    } else {
                        println!("Error while searching tags page {}", x);
                        None
                    }
                })
                .flatten()
                .collect::<Vec<Tag>>()
        })
    }
}

impl Table for TagTable {
    fn search_tag(html: &str) -> Option<Vec<Tag>> {
        // Comment mettre le tout en une seul ligne ?
        let after_tags: Option<&str> = html.after(r#"<div class="container" id="tag-container">"#);
        let html_tags = after_tags
            .before(r#"</div>"#)?
            .split(r#"<section"#)
            .map(|x| x.split(r#"<a href="#).collect::<Vec<&str>>())
            .flatten()
            .collect::<Vec<&str>>();

        Some(
            html_tags
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
}


// ArtistTable
#[derive(Debug)]
pub struct ArtistTable {
    pub artists: Vec<Tag>,
}

impl ArtistTable {
    pub fn new() -> Option<Self> {
        use rayon::prelude::*;
        let nbr_page = reqwest::blocking::get("https://nhentai.net/artists/")
            .ok()?
            .text()
            .ok()?
            .after(r#"<section class="pagination">"#)
            .before(r#"</section>"#)
            .between(r#"class="next""#, r#"" class="last""#)
            .after("page=")?
            .parse::<u32>()
            .ok()?;

        let vec_html = (1..nbr_page + 1)
            .flat_map(|x| {
                let url: String = format!("https://nhentai.net/artists/?page={}", x);
                Some(reqwest::blocking::get(&url).ok()?.text().ok()?)
            })
            .collect::<Vec<String>>();

        Some(Self {
            artists: vec_html
                .par_iter()
                .flat_map(|x| {
                    if let Some(t) = Self::search_tag(x) {
                        Some(t)
                    } else {
                        println!("Error while searching tags page {}", x);
                        None
                    }
                })
                .flatten()
                .collect::<Vec<Tag>>()
        })
    }
}

impl Table for ArtistTable {
    fn search_tag(html: &str) -> Option<Vec<Tag>> {
        let after_artists: Option<&str> = html.after(r#"<div class="container" id="tag-container">"#);
        let html_artists = after_artists
            .before(r#"</div>"#)?
            .split(r#"<section"#)
            .map(|x| x.split(r#"<a href="#).collect::<Vec<&str>>())
            .flatten()
            .collect::<Vec<&str>>();

        Some(
            html_artists
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
}


// CharacterTable
#[derive(Debug)]
pub struct CharacterTable {
    pub characters: Vec<Tag>,
}

impl CharacterTable {
    pub fn new() -> Option<Self> {
        use rayon::prelude::*;
        let nbr_page = reqwest::blocking::get("https://nhentai.net/artists/")
            .ok()?
            .text()
            .ok()?
            .after(r#"<section class="pagination">"#)
            .before(r#"</section>"#)
            .between(r#"class="next""#, r#"" class="last""#)
            .after("page=")?
            .parse::<u32>()
            .ok()?;

        let vec_html = (1..nbr_page + 1)
            .flat_map(|x| {
                let url: String = format!("https://nhentai.net/artists/?page={}", x);
                Some(reqwest::blocking::get(&url).ok()?.text().ok()?)
            })
            .collect::<Vec<String>>();

        Some(Self {
            characters: vec_html
                .par_iter()
                .flat_map(|x| {
                    if let Some(t) = Self::search_tag(x) {
                        Some(t)
                    } else {
                        println!("Error while searching tags page {}", x);
                        None
                    }
                })
                .flatten()
                .collect::<Vec<Tag>>()
        })
    }
}

impl Table for CharacterTable {
    fn search_tag(html: &str) -> Option<Vec<Tag>> {
        let after_characters: Option<&str> = html.after(r#"<div class="container" id="tag-container">"#);
        let html_characters = after_characters
            .before(r#"</div>"#)?
            .split(r#"<section"#)
            .map(|x| x.split(r#"<a href="#).collect::<Vec<&str>>())
            .flatten()
            .collect::<Vec<&str>>();

        Some(
            html_characters
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
}


// ParodieTable
#[derive(Debug)]
pub struct ParodieTable {
    pub parodies: Vec<Tag>,
}

impl ParodieTable {
    pub fn new() -> Option<Self> {
        use rayon::prelude::*;
        let nbr_page = reqwest::blocking::get("https://nhentai.net/parodies/")
            .ok()?
            .text()
            .ok()?
            .after(r#"<section class="pagination">"#)
            .before(r#"</section>"#)
            .between(r#"class="next""#, r#"" class="last""#)
            .after("page=")?
            .parse::<u32>()
            .ok()?;

        let vec_html = (1..nbr_page + 1)
            .flat_map(|x| {
                let url: String = format!("https://nhentai.net/parodies/?page={}", x);
                Some(reqwest::blocking::get(&url).ok()?.text().ok()?)
            })
            .collect::<Vec<String>>();

        Some(Self {
            parodies: vec_html
                .par_iter()
                .flat_map(|x| {
                    if let Some(t) = Self::search_tag(x) {
                        Some(t)
                    } else {
                        println!("Error while searching tags page {}", x);
                        None
                    }
                })
                .flatten()
                .collect::<Vec<Tag>>()
        })
    }
}

impl Table for ParodieTable {
    fn search_tag(html: &str) -> Option<Vec<Tag>> {
        let after_parodies: Option<&str> = html.after(r#"<div class="container" id="tag-container">"#);
        let html_parodies = after_parodies
            .before(r#"</div>"#)?
            .split(r#"<section"#)
            .map(|x| x.split(r#"<a href="#).collect::<Vec<&str>>())
            .flatten()
            .collect::<Vec<&str>>();

        Some(
            html_parodies
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
}



// ParodieTable
#[derive(Debug)]
pub struct GroupTable {
    pub groups: Vec<Tag>,
}

impl GroupTable {
    pub fn new() -> Option<Self> {
        use rayon::prelude::*;
        let nbr_page = reqwest::blocking::get("https://nhentai.net/groups/")
            .ok()?
            .text()
            .ok()?
            .after(r#"<section class="pagination">"#)
            .before(r#"</section>"#)
            .between(r#"class="next""#, r#"" class="last""#)
            .after("page=")?
            .parse::<u32>()
            .ok()?;

        let vec_html = (1..nbr_page + 1)
            .flat_map(|x| {
                let url: String = format!("https://nhentai.net/groups/?page={}", x);
                Some(reqwest::blocking::get(&url).ok()?.text().ok()?)
            })
            .collect::<Vec<String>>();

        Some(Self {
            groups: vec_html
                .par_iter()
                .flat_map(|x| {
                    if let Some(t) = Self::search_tag(x) {
                        Some(t)
                    } else {
                        println!("Error while searching tags page {}", x);
                        None
                    }
                })
                .flatten()
                .collect::<Vec<Tag>>()
        })
    }
}

impl Table for GroupTable {
    fn search_tag(html: &str) -> Option<Vec<Tag>> {
        let after_groups: Option<&str> = html.after(r#"<div class="container" id="tag-container">"#);
        let html_groups = after_groups
            .before(r#"</div>"#)?
            .split(r#"<section"#)
            .map(|x| x.split(r#"<a href="#).collect::<Vec<&str>>())
            .flatten()
            .collect::<Vec<&str>>();

        Some(
            html_groups
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
}
