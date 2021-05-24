
use std::ops::Range;
use std::str::FromStr;
use std::fmt;
use std::ops::RangeBounds;
use std::ops::Bound;

use crate::tag::Tag;
use crate::search::time_range::TimeRange;


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SearchArgs {
    Page(u16),
    Sort(Sort),
    Text(String, bool),
    Uploaded(TimeRange),
    GalleryPages(Range<u16>),
    Tag(Tag, bool),
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
            Self::GalleryPages(range) => write!(f, "{}", range.display()),
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

        let mut buffer= vec![];
        if let Some(i) = vec_args.iter().position(|x| matches!(x, Self::Uploaded(_))) {
            buffer.push(vec_args.remove(i));
        }

        if let Some(i) = vec_args.iter().position(|x| matches!(x, Self::GalleryPages(_))) {
            buffer.push(vec_args.remove(i));
        }
        vec_args.append(&mut buffer);

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

trait DisplayRange {
    fn display(&self) -> String;
}

impl DisplayRange for Range<u16> {
    fn display(&self) -> String {
        format!("{}+{}",
            match self.start_bound() {
                Bound::Included(x) => format!("pages:>={}", x),
                Bound::Excluded(x) => format!("pages:>{}", x),
                Bound::Unbounded => "".to_owned()
            },
            match self.end_bound() {
                Bound::Included(x) => format!("pages:<={}", x),
                Bound::Excluded(x) => format!("pages:<{}", x),
                Bound::Unbounded => "".to_owned()
            },
        )
    }
}