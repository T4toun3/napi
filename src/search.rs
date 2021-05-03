use crate::doujin::Doujin;
use crate::search_args::*;
use crate::string_utils::*;

use std::ops::Range;

#[derive(Debug, PartialEq)]
pub struct Search {
    pub pages: u16,
    pub current_page: u16,
    pub current_args: Vec<SearchArgs>,
    pub entries: Vec<SearchEntry>,
}

impl Search {
    pub fn new(args: Vec<SearchArgs>) -> Option<Search> {
        let current_args = SearchArgs::correct(args);

        let url = Self::build_url_with_args(current_args.clone());

        let text = reqwest::blocking::get(url.as_str()).ok()?.text().ok()?;

        let current_page = current_args[0].get_page().unwrap();

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

    pub fn build_url_with_args(args: Vec<SearchArgs>) -> String {
        let args = SearchArgs::correct(args);
        format!(
            "https://nhentai.net/search/?{}",
            args.iter()
                .filter(|x| !matches!(x, SearchArgs::Sort(Sort::Recent)))
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("&")
        )
    }

    pub fn search_populars(sort: Sort) -> Option<Self> {
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
        Self::new(vec![
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

    pub fn merge_search_pages(&mut self, range: Range<u16>) {
        let pages: Vec<SearchEntry> = (range)
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
            let mut args = self.current_args.clone();
            args.retain(|x| !matches!(x, SearchArgs::Page(_)));
            args.push(SearchArgs::Page(page));
            Search::new(args)
        } else {
            None
        }
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct SearchEntry {
    pub thumb: String,
    pub id: u32,
    pub tags_by_id: Vec<u32>,
    pub name: String,
}

impl SearchEntry {
    pub fn new(text: &str) -> Option<Self> {
        let text = text.before("</div>");
        Some(Self {
            tags_by_id: text
                .before("\"")?
                .split_whitespace()
                .flat_map(|x| x.parse().ok())
                .collect(),
            thumb: text.after(" data-src=\"").before("\"")?.to_owned(),
            id: text.after("<a href=\"/g/").before("/\"").to_type()?,
            name: text.after("<div class=\"caption\">")?.to_owned(),
        })
    }

    pub fn fetch(&self) -> Option<Doujin> {
        Doujin::new(self.id)
    }
}
