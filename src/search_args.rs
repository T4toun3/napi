use crate::tag::Tag;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SearchArgs {
    Page(u16),
    Sort(Sort),
    Text(String, bool),
    Tag(Tag, bool),
}

// Tag,
// Artist,
// Character,
// Parodie,
// Group,
// Language,
// Category

use std::str::FromStr;

impl FromStr for SearchArgs {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = s.split('=');

        Ok(match i.next().ok_or(())? {
            "page" => Self::Page(i.next().ok_or(())?.parse().map_err(|_| ())?),
            "q" => Self::Text(i.next().ok_or(())?.to_owned(), true),
            "sort" => Self::Sort(i.next().ok_or(())?.parse().map_err(|_| ())?),
            _ => return Err(()),
        })
    }
}

impl std::fmt::Display for SearchArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Page(e) => write!(f, "page={}", e),
            Self::Sort(e) => write!(f, "sort={}", e),
            Self::Text(e, _) => write!(f, "q={}", e),
            Self::Tag(e, _) => match e._type {
                _ => write!(f, "{}", e.name),
            },
        }
    }
}

impl SearchArgs {
    pub fn correct(mut vec_args: Vec<Self>) -> Vec<Self> {
        let mut args: Vec<Self> = Vec::new();
        if let Some(i) = vec_args.iter().position(|x| matches!(x, Self::Page(_))) {
            args.push(vec_args.remove(i));
        } else {
            args.push(Self::Page(1));
        }
        if let Some(i) = vec_args.iter().position(|x| matches!(x, Self::Sort(_))) {
            args.push(vec_args.remove(i));
        } else {
            args.push(Self::Sort(Sort::Recent));
        }

        let mut text = vec_args
            .iter()
            .filter(|x| match x {
                SearchArgs::Text(_, _) | SearchArgs::Tag(_, _) => true,
                _ => false,
            })
            .map(|arg| {
                let mut string = String::new();
                if !arg.used() {
                    string.push('-')
                }
                match arg {
                    SearchArgs::Text(text, _) => string.push_str(text.as_str()),
                    SearchArgs::Tag(tag, _) => string
                        .push_str(format!("{}:\"{}\"", tag._type.to_string(), tag.name).as_str()),
                    _ => {}
                }
                string
            })
            .collect::<Vec<_>>()
            .join("+");
        if text.is_empty() {
            text = "\"\"".to_string();
        }

        args.push(SearchArgs::Text(text, true));
        args
    }

    pub fn used(&self) -> bool {
        match self {
            SearchArgs::Text(_, used) => *used,
            SearchArgs::Tag(_, used) => *used,
            _ => true,
        }
    }

    pub fn parse(s: &str) -> Vec<Self> {
        s.split('&')
            .flat_map(|x| {
                let mut i = x.split('=');

                Some(match i.next()? {
                    "page" => vec![Self::Page(i.next()?.parse::<u16>().ok()?)],
                    "sort" => vec![Self::Sort(i.next()?.parse().ok()?)],
                    "q" => {
                        let content = i.next()?;

                        // TODO: splite les ':'
                        //       remplacer les 'true' temporaire

                        // pub struct Tag {
                        //     pub id: u32,
                        //     #[serde(rename = "type")]
                        //     pub _type: TagType,
                        //     pub name: String,
                        //     pub url: String,
                        //     pub count: u32,
                        // }
                        vec![Self::Text(i.next()?.to_owned(), true)]
                    }
                    _ => return None,
                })
            })
            .flatten()
            .collect::<Vec<Self>>()
    }

    pub fn get_page(&self) -> Option<u16> {
        match self {
            SearchArgs::Page(page) => Some(*page),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Sort {
    PopularWeek,
    PopularToday,
    PopularAllTime,
    Recent,
}

impl std::fmt::Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::PopularWeek => "popular-week",
                Self::PopularToday => "popular-today",
                Self::PopularAllTime => "popular",
                Self::Recent => "",
            }
        )
    }
}

impl FromStr for Sort {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "popular" => Self::PopularAllTime,
            "popular-today" => Self::PopularToday,
            "popular-week" => Self::PopularWeek,
            _ => Self::Recent,
        })
    }
}
