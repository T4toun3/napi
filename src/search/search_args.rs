use std::fmt;
use std::str::FromStr;

use super::range::Magnitude;
use super::range::Range;
use crate::tag::Tag;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SearchArgs {
    Page(u16),
    Sort(Sort),
    Text(String, bool), // text, used
    // ! not finish
    Uploaded(Range<Magnitude>),
    // use the marco page!() to generate Range<u16>
    GalleryPages(Range<u16>),
    Tag(Tag, bool), // tag, used
}

#[test]
fn correct_work_maybe() {
    let args1 = vec![SearchArgs::Page(10), SearchArgs::Page(2), SearchArgs::Page(3)];
    let args2 = vec![SearchArgs::Sort(Sort::PopularWeek), SearchArgs::Sort(Sort::PopularToday), SearchArgs::Sort(Sort::PopularWeek)];
    assert_eq!(
        SearchArgs::correct(args1),
        vec![SearchArgs::Page(10), SearchArgs::Sort(Sort::Recent), SearchArgs::Text("\"\"".to_owned(), true)]
    );
    assert_eq!(
        SearchArgs::correct(args2),
        vec![SearchArgs::Page(1), SearchArgs::Sort(Sort::PopularWeek), SearchArgs::Text("\"\"".to_owned(), true)]
    );

    let args3 = vec![SearchArgs::Text("oui".to_owned(), true), SearchArgs::Text("non".to_owned(), false)];
    assert_eq!(
        SearchArgs::correct(args3),
        vec![SearchArgs::Page(1), SearchArgs::Sort(Sort::Recent), SearchArgs::Text("oui+-non".to_owned(), true)]
    );
}

impl SearchArgs {
    pub fn correct(mut vec_args: Vec<Self>) -> Vec<Self> {
        let mut args: Vec<Self> = Vec::new();
        // Page
        if let Some(i) = vec_args.iter().position(|x| matches!(x, Self::Page(_))) {
            args.push(vec_args.remove(i));
        } else {
            args.push(Self::Page(1));
        }

        // Sort
        if let Some(i) = vec_args.iter().position(|x| matches!(x, Self::Sort(_))) {
            args.push(vec_args.remove(i));
        } else {
            args.push(Self::Sort(Sort::Recent));
        }

        let mut buffer_uploaded_and_page = vec![];
        // Uploaded
        if let Some(i) = vec_args.iter().position(|x| matches!(x, Self::Uploaded(_))) {
            buffer_uploaded_and_page.push(vec_args.remove(i));
        }

        // GalleryPage
        if let Some(i) = vec_args
            .iter()
            .position(|x| matches!(x, Self::GalleryPages(_)))
        {
            buffer_uploaded_and_page.push(vec_args.remove(i));
        }

        // Text
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
            .chain(buffer_uploaded_and_page.iter().map(|arg| arg.to_string()))
            .collect::<Vec<_>>()
            .join("+");
        if text.is_empty() {
            text = "\"\"".to_owned();
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

    pub fn get_page(&self) -> Option<u16> {
        match self {
            SearchArgs::Page(page) => Some(*page),
            _ => None,
        }
    }
}

impl FromStr for SearchArgs {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = s.split('=');

        Ok(match i.next().ok_or(())? {
            "page" => Self::Page(i.next().ok_or(())?.parse().map_err(|_| ())?),
            // TODO: Parse 'q' to split text into the differents Arg
            "q" => Self::Text(i.next().ok_or(())?.to_owned(), true),
            "sort" => Self::Sort(i.next().ok_or(())?.parse().map_err(|_| ())?),
            _ => return Err(()),
        })
    }
}

impl fmt::Display for SearchArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Page(e) => write!(f, "page={}", e),
            Self::Sort(e) => write!(f, "sort={}", e),
            Self::Text(e, _) => write!(f, "q={}", e),
            Self::Tag(e, _) => match e._type {
                _ => write!(f, "{}", e.name),
            },
            Self::Uploaded(time_range) => write!(f, "{}", time_range),
            Self::GalleryPages(range) => write!(f, "{}", range),
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

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
