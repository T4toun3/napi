use std::collections::HashMap;

use crate::search::SearchEntry;

const fn none() -> Option<Vec<SearchEntry>> {
    None
}

#[derive(serde::Deserialize, Debug)]
pub struct Doujin {
    pub id: u32,
    pub media_id: String,
    pub title: HashMap<String, String>, // Lang - Title
    pub upload_date: u64,
    pub tags: Vec<Tag>,
    pub num_pages: u16,
    pub images: Images,
    pub num_favorites: u32,
    #[serde(default = "none")]
    pub similars: Option<Vec<SearchEntry>>,
}

impl Doujin {
    pub fn new(id: u32) -> Option<Self> {
        use crate::string_utils::StringUtils;
        reqwest::blocking::get(&format!("http://nhentai.net/g/{}/",id))
            .ok()?
            .text()
            .ok()
            .after("JSON.parse(\"")
            .before("\");")
            .map(|x| serde_json::from_str(&x.replace("\\u0022","\"").replace("\\u002F","/")).ok())
            .flatten()
    }

    pub fn generate_similars(&mut self) {
        use crate::string_utils::StringUtils;
        self.similars = Some(reqwest::blocking::get(&format!("http://nhentai.net/g/{}", self.id)).ok().unwrap().text().ok()
            .after(r#"<h2>More Like This<h2>"#)
            .before(r#"id="comment-post-container""#).unwrap()
            .split(r#"<div class="gallery" data-tags=""#)
            .flat_map(|x| SearchEntry::new(x)).collect::<Vec<SearchEntry>>());
    }

    pub fn get_page_image_url_small(&self, page: u16) -> Option<String> {
        Some(format!(
            "https://t.nhentai.net/galleries/{}/{}t.{}",
            self.media_id,
            page,
            self.images.get_page_extension(page)?
        ))
    }

    pub fn get_pages_image_urls_small(&self) -> Vec<String> {
        (1..self.num_pages + 1)
            .flat_map(|x| {
                Some(format!(
                    "https://t.nhentai.net/galleries/{}/{}t.{}",
                    self.media_id,
                    x,
                    self.images.get_page_extension(x)?
                ))
            })
            .collect()
    }

    pub fn get_page_image_url(&self, page: u16) -> Option<String> {
        Some(format!(
            "https://i.nhentai.net/galleries/{}/{}.{}",
            self.media_id,
            page,
            self.images.get_page_extension(page)?
        ))
    }

    pub fn get_pages_image_urls(&self) -> Vec<String> {
        (1..self.num_pages + 1)
            .flat_map(|x| {
                Some(format!(
                    "https://i.nhentai.net/galleries/{}/{}.{}",
                    self.media_id,
                    x,
                    self.images.get_page_extension(x)?
                ))
            })
            .collect()
    }

    pub fn get_image_cover(&self) -> String {
        format!(
            "https://t.nhentai.net/galleries/{}/cover.{}",
            self.media_id,
            self.images.get_cover_extension()
        )
    }

    pub fn get_image_thumbnail(&self) -> String {
        format!(
            "https://t.nhentai.net/galleries/{}/thumb.{}",
            self.media_id,
            self.images.get_thumbnail_extension()
        )
    }

    pub fn get_page_url(&self, page: u16) -> String {
        format!("https://nhentai.net/g/{}/{}/", self.id, page)
    }

    pub fn get_pages_urls(&self) -> Vec<String> {
        (1..self.num_pages + 1)
            .map(|x| format!("https://nhentai.net/g/{}/{}/", self.id, x))
            .collect()
    }

    pub fn get_artists(&self) -> Vec<&Tag> {
        self.tags
            .iter()
            .filter(|tag| tag._type == "artist")
            .collect::<Vec<&Tag>>()
    }

    pub fn get_languages(&self) -> Vec<&Tag> {
        self.tags
            .iter()
            .filter(|tag| tag._type == "language")
            .collect::<Vec<&Tag>>()
    }

    pub fn get_characters(&self) -> Vec<&Tag> {
        self.tags
            .iter()
            .filter(|tag| tag._type == "character")
            .collect::<Vec<&Tag>>()
    }

    pub fn get_groups(&self) -> Vec<&Tag> {
        self.tags
            .iter()
            .filter(|tag| tag._type == "group")
            .collect::<Vec<&Tag>>()
    }

    pub fn get_tags(&self) -> Vec<&Tag> {
        self.tags
            .iter()
            .filter(|tag| tag._type == "tag")
            .collect::<Vec<&Tag>>()
    }

    pub fn get_category(&self) -> Vec<&Tag> {
        self.tags
            .iter()
            .filter(|tag| tag._type == "category")
            .collect::<Vec<&Tag>>()
    }

    pub fn random() -> Self {
        Self::from_str(
            reqwest::blocking::get("https://nhentai.net/random/")
                .unwrap()
                .text()
                .unwrap()
                .as_ref(),
        )
        .unwrap()
    }

}

use std::str::FromStr;

impl FromStr for Doujin {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::string_utils::*;
        Ok(
            if let Some(e) = s
                .after("JSON.parse(\"")
                .before("\");")
                .map(|x| {
                    serde_json::from_str(&x.replace("\\u0022", "\"").replace("\\u002F", "/"))
                        .ok()?
                })
                .flatten()
            {
                e
            } else {
                return Err(());
            },
        )
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct Images {
    pub pages: Vec<ImageFormat>,
    pub cover: ImageFormat,
    pub thumbnail: ImageFormat,
}

impl Images {
    pub fn get_page_extension(&self, page: u16) -> Option<&str> {
        Some(match self.pages.get(page as usize)?.t.as_ref() {
            "j" => "jpg",
            "p" => "png",
            _ => "jpg",
        })
    }

    pub fn get_thumbnail_extension(&self) -> &str {
        match self.thumbnail.t.as_ref() {
            "j" => "jpg",
            "p" => "png",
            _ => "jpg",
        }
    }

    pub fn get_cover_extension(&self) -> &str {
        match self.cover.t.as_ref() {
            "j" => "jpg",
            "p" => "png",
            _ => "jpg",
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct ImageFormat {
    pub t: String,
    pub w: u16,
    pub h: u16,
}

#[derive(serde::Deserialize, Debug)]
pub struct Tag {
    pub id: u32,
    #[serde(rename = "type")]
    pub _type: String,
    pub name: String,
    pub url: String,
    pub count: u32,
}
