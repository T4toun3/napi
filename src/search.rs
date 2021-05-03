use crate::string_utils::*;

#[derive(Debug, PartialEq)]
pub struct Search {
    pub pages: u16,
    pub current_args: Vec<SearchArgs>,
    pub entries: Vec<SearchEntry>,
}

impl Search {
    pub fn new(url: &str) -> Option<Search> {
        let text = reqwest::blocking::get(url).ok()?.text().ok()?;
        let mut current_args: Vec<SearchArgs> = if let Some(e) = url.after("?") {
            e.split('&').flat_map(|x| x.parse().ok()).collect()
        } else {
            Vec::new()
        };

        current_args = SearchArgs::dedup(current_args);

        let pages = text
            .after("<section class=\"pagination\">")
            .before("</section>")?
            .split("page=")
            .last()?
            .before("&")
            .before("\"")
            .to_type::<u16>()?;
        Some(Self {
            pages,
            current_args,
            entries: text
                .split(r#"<div class="gallery" data-tags=""#)
                .skip(1)
                .flat_map(|x| SearchEntry::new(x))
                .collect(),
        })
    }

    pub fn build_url_with_args(mut to_add: Vec<SearchArgs>) -> String {
        to_add = SearchArgs::dedup(to_add);
        format!(
            "https://nhentai.net/search/?{}",
            to_add
                .iter()
                .filter(|x| !matches!(x, SearchArgs::Sort(Sort::Recent)))
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("&")
        )
    }

    pub fn search_populars(sort: Sort) -> Option<Self> {
        Self::new(&Self::build_url_with_args(vec![SearchArgs::Sort(sort), SearchArgs::Text(r#""""#.to_owned())]))
    }

    pub fn get_current_page(&self) -> u16 {
        if let Some(SearchArgs::Page(e)) = self
            .current_args
            .iter()
            .find(|x| matches!(x, SearchArgs::Page(x) if x != &0))
        {
            *e
        } else {
            1
        }
    }

    pub fn merge_search_all_pages(&mut self) {
        let o = self.get_current_page();
        let pages: Vec<SearchEntry> = (1..self.pages + 1)
            .into_iter()
            .flat_map(|x| {
                if x == o {
                    None
                } else if let Some(e) = self.search_page(x).map(|x| x.entries) {
                    Some(e)
                } else {
                    println!("Error while searching page {}", x);
                    None
                }
            })
            .flatten()
            .collect();
        self.entries.extend(pages);
    }

    pub fn merge_search_pages(&mut self, limit: u16) {
        use std::cmp;
        let o = self.get_current_page();
        let pages: Vec<SearchEntry> = (1..cmp::min(limit, self.pages) + 1)
            .into_iter()
            .flat_map(|x| {
                if x == o {
                    None
                } else if let Some(e) = self.search_page(x).map(|x| x.entries) {
                    Some(e)
                } else {
                    println!("Error while searching page {}", x);
                    None
                }
            })
            .flatten()
            .collect();
        self.entries.extend(pages);
    }

    pub fn search_page(&self, page: u16) -> Option<Search> {
        if self.pages >= page {
            let mut map = self.current_args.clone();
            map.retain(|x| !matches!(x, SearchArgs::Page(_)));
            map.push(SearchArgs::Page(page));
            Search::new(&Self::build_url_with_args(map))
        } else {
            None
        }
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct SearchEntry {
    pub thumb: String,
    pub id: u32,
    pub tags: Vec<u32>,
    pub name: String,
}

impl SearchEntry {
    pub fn new(text: &str) -> Option<Self> {
        let text = text.before("</div>");
        Some(Self {
            tags: text
                .before("\"")?
                .split_whitespace()
                .flat_map(|x| x.parse().ok())
                .collect(),
            thumb: text.after(" data-src=\"").before("\"")?.to_owned(),
            id: text.after("<a href=\"/g/").before("/\"").to_type()?,
            name: text.after("<div class=\"caption\">")?.to_owned(),
        })
    }

    pub fn fetch(&self) -> Option<crate::doujin::Doujin> {
        crate::doujin::Doujin::new(self.id)
    }
}
