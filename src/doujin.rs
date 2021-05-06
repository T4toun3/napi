
use chrono::naive::NaiveDateTime;
use serde::Deserialize;

use crate::search::SearchEntry;
use crate::string_utils::StringUtils;
use crate::tag::*;

use crate::serde_utils::*;

#[derive(Deserialize, Debug)]
pub struct Doujin {
    pub id: u32,
    #[serde(deserialize_with = "string_to_u32")]
    pub media_id: u32,
    // TODO: change this because it's all the time "english" + "japaness" + "pretty" (struct)
    pub title: HashMap<String, String>, // Lang - Title
    #[serde(deserialize_with = "unix_to_date")]
    pub upload_date: NaiveDateTime,
    pub tags: Vec<Tag>,
    pub num_pages: u16,
    pub images: Images,
    pub num_favorites: u32,
    #[serde(default = "empty_vec")]
    pub similars: Vec<SearchEntry>,
}

impl Doujin {
    pub fn new(id: u32) -> Option<Self> {
        let html = reqwest::blocking::get(&format!("http://nhentai.net/g/{}", id))
            .ok()?
            .text()
            .ok()?;

        let mut doujin: Doujin = html
            .after("JSON.parse(\"")
            .before("\");")
            .map(|x| serde_json::from_str(&x.replace("\\u0022", "\"").replace("\\u002F", "/")).ok())
            .flatten()?;

        doujin.similars = html
            .after("<h2>More Like This</h2>")
            .before("</div></div>")?
            .split(r#"<div class="gallery" data-tags=""#)
            .flat_map(|x| SearchEntry::new(x))
            .collect::<Vec<SearchEntry>>();

        Some(doujin)

        // let text = html
        //     .after("JSON.parse(\"")
        //     .before("\");").unwrap().replace("\\u0022", "\"").replace("\\u002F", "/");

        // let dunjin = &mut serde_json::Deserializer::from_str(&text);

        // let result: Result<Doujin, _> = serde_path_to_error::deserialize(dunjin);
        // match result {
        //     Ok(_) => panic!("expected a type error"),
        //     Err(err) => {
        //         let path = err.into_inner().to_string();
        //         assert_eq!(path, "dependencies.serde.version");
        //     }
        // }
        // None
    }

    pub fn get_image_url_small(&self, page: u16) -> Option<String> {
        if page > self.num_pages {
            return None;
        }
        Some(format!(
            "https://t.nhentai.net/galleries/{}/{}t.{}",
            self.media_id,
            page,
            self.images.get_page_extension(page)?
        ))
    }

    pub fn get_images_urls_small(&self) -> Vec<String> {
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

    pub fn get_image_url(&self, page: u16) -> Option<String> {
        Some(format!(
            "https://i.nhentai.net/galleries/{}/{}.{}",
            self.media_id,
            page,
            self.images.get_page_extension(page)?
        ))
    }

    pub fn get_images_urls(&self) -> Vec<String> {
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

    pub fn get_parodies(&self) -> Vec<&Tag> {
        self.tags
            .iter()
            .filter(|tag| matches!(tag._type, TagType::Parody))
            .collect::<Vec<&Tag>>()
    }

    pub fn get_characters(&self) -> Vec<&Tag> {
        self.tags
            .iter()
            .filter(|tag| matches!(tag._type, TagType::Character))
            .collect::<Vec<&Tag>>()
    }

    pub fn get_tags(&self) -> Vec<&Tag> {
        self.tags
            .iter()
            .filter(|tag| matches!(tag._type, TagType::Tag))
            .collect::<Vec<&Tag>>()
    }

    pub fn get_groups(&self) -> Vec<&Tag> {
        self.tags
            .iter()
            .filter(|tag| matches!(tag._type, TagType::Group))
            .collect::<Vec<&Tag>>()
    }

    pub fn get_languages(&self) -> Vec<&Tag> {
        self.tags
            .iter()
            .filter(|tag| matches!(tag._type, TagType::Language))
            .collect::<Vec<&Tag>>()
    }

    pub fn get_artists(&self) -> Vec<&Tag> {
        self.tags
            .iter()
            .filter(|tag| matches!(tag._type, TagType::Artist))
            .collect::<Vec<&Tag>>()
    }

    pub fn get_category(&self) -> Vec<&Tag> {
        self.tags
            .iter()
            .filter(|tag| matches!(tag._type, TagType::Category))
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

#[derive(Deserialize, Debug)]
pub struct ImageFormat {
    pub t: String,
    pub w: u16,
    pub h: u16,
}

// #[test]
// fn deserd() {
//     let data = r#"{"id":141506,"media_id":"844469","title":{"english":"(C88) [In The Sky (Nakano Sora)] Shuuya ni Omou (Kantai Collection -KanColle-) [Chinese] [\u005Cu5c4f\u005Cu5e55\u005Cu9ad2\u005Cu4e86\u005Cu6f22\u005Cu5316\u005Cu7d44]","japanese":"(C88) [In The Sky (\u005Cu4e2d\u005Cu4e43\u005Cu7a7a)] \u005Cu79cb\u005Cu591c\u005Cu30cb\u005Cu60f3\u005Cu30d5 (\u005Cu8266\u005Cu968a\u005Cu3053\u005Cu308c\u005Cu304f\u005Cu3057\u005Cu3087\u005Cu3093 -\u005Cu8266\u005Cu3053\u005Cu308c-) [\u005Cu4e2d\u005Cu56fd\u005Cu7ffb\u005Cu8a33]","pretty":"Shuuya ni Omou"},"images":{"pages":[{"t":"j","w":1200,"h":1689},{"t":"j","w":1200,"h":1689},{"t":"j","w":1200,"h":1693},{"t":"j","w":1200,"h":1716},{"t":"j","w":1200,"h":1735},{"t":"j","w":1200,"h":1716},{"t":"j","w":1200,"h":1751},{"t":"j","w":1200,"h":1716},{"t":"j","w":1200,"h":1716},{"t":"j","w":1200,"h":1716},{"t":"j","w":1200,"h":1716},{"t":"j","w":1200,"h":1716},{"t":"j","w":1200,"h":1716},{"t":"j","w":1200,"h":1716},{"t":"j","w":1200,"h":1716},{"t":"j","w":1200,"h":1716},{"t":"j","w":1200,"h":1716},{"t":"j","w":1200,"h":1716},{"t":"j","w":1200,"h":1716},{"t":"j","w":1200,"h":847},{"t":"j","w":1200,"h":1716},{"t":"j","w":1200,"h":1756},{"t":"j","w":1200,"h":1761},{"t":"p","w":1024,"h":711}],"cover":{"t":"j","w":350,"h":493},"thumbnail":{"t":"j","w":250,"h":352}},"scanlator":"","upload_date":1440162796,"tags":[{"id":1841,"type":"parody","name":"kantai collection","url":"\u002Fparody\u002Fkantai-collection\u002F","count":13889},{"id":10248,"type":"artist","name":"nakano sora","url":"\u002Fartist\u002Fnakano-sora\u002F","count":180},{"id":10314,"type":"tag","name":"schoolgirl uniform","url":"\u002Ftag\u002Fschoolgirl-uniform\u002F","count":56614},{"id":13720,"type":"tag","name":"nakadashi","url":"\u002Ftag\u002Fnakadashi\u002F","count":43099},{"id":17249,"type":"language","name":"translated","url":"\u002Flanguage\u002Ftranslated\u002F","count":111427},{"id":17488,"type":"group","name":"in the sky","url":"\u002Fgroup\u002Fin-the-sky\u002F","count":125},{"id":20035,"type":"tag","name":"x-ray","url":"\u002Ftag\u002Fx-ray\u002F","count":21189},{"id":26980,"type":"character","name":"akizuki","url":"\u002Fcharacter\u002Fakizuki\u002F","count":90},{"id":29859,"type":"tag","name":"blowjob","url":"\u002Ftag\u002Fblowjob\u002F","count":37977},{"id":29922,"type":"character","name":"teitoku","url":"\u002Fcharacter\u002Fteitoku\u002F","count":7823},{"id":29963,"type":"language","name":"chinese","url":"\u002Flanguage\u002Fchinese\u002F","count":45212},{"id":33172,"type":"category","name":"doujinshi","url":"\u002Fcategory\u002Fdoujinshi\u002F","count":231184},{"id":35762,"type":"tag","name":"sole female","url":"\u002Ftag\u002Fsole-female\u002F","count":67502},{"id":35763,"type":"tag","name":"sole male","url":"\u002Ftag\u002Fsole-male\u002F","count":61245}],"num_pages":24,"num_favorites":424}"#;
// }