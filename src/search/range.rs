use std::cmp::Ordering;
use std::fmt;
use std::ops::Bound;
use std::ops::RangeBounds;


const MAX_HOURS: u16 = 4999;

#[macro_export]
macro_rules! pages {
    (..) => {
        Range::<u16>{ start: std::ops::Bound::Unbounded, end: std::ops::Bound::Unbounded}
    };
    (=>) => {
        Range::<u16>{ start: std::ops::Bound::Unbounded, end: std::ops::Bound::Unbounded}
    };
    ($start:expr => $end:expr) => {
        Range::<u16>{ start: std::ops::Bound::Included($start), end: std::ops::Bound::Excluded($end)}
    };
    (=> $end:expr) => {
        Range::<u16>{ start: std::ops::Bound::Unbounded, end: std::ops::Bound::Excluded($end)}
    };
    ($start:expr =>) => {
        Range::<u16>{ start: std::ops::Bound::Included($start), end: std::ops::Bound::Unbounded}
    };
    ($start:expr => =$end:expr) => {
        Range::<u16>{ start: std::ops::Bound::Included($start), end: std::ops::Bound::Included($end)}
    };
    (=> =$end:expr) => {
        Range::<u16>{ start: std::ops::Bound::Unbounded, end: std::ops::Bound::Included($end)}
    };
    ($unique:expr) => {
        Range::<u16>{ start: std::ops::Bound::Included($unique), end: std::ops::Bound::Included($unique)}
    };
}

// ! I haven't done any test for this feature yet
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Range<T> {
    pub start: Bound<T>,
    pub end: Bound<T>,
}

impl<T> RangeBounds<T> for Range<T> {
    fn start_bound(&self) -> Bound<&T> {
        self.start.as_ref()
    }

    fn end_bound(&self) -> Bound<&T> {
        self.end.as_ref()
    }

    fn contains<U>(&self, _: &U) -> bool
    where
            T: PartialOrd<U>,
            U: ?Sized + PartialOrd<T>, {
        panic!("method containe` should not be use on `happi::search::Range`")
    }
}

impl fmt::Display for Range<u16> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let from =             match self.start_bound() {
            Bound::Included(ref mg) => format!("pages:>={}", mg),
            Bound::Excluded(ref mg) => format!("pages:>{}", mg),
            Bound::Unbounded => "".to_owned()
        };
        let to =             match self.end_bound() {
            Bound::Included(ref mg) => format!("pages:<={}", mg),
            Bound::Excluded(ref mg) => format!("pages:<{}", mg),
            Bound::Unbounded => "".to_owned()
        };
        
        write!(f, "{}{}{}", from, if !from.is_empty() && !to.is_empty() { "+" } else { "" },  to)
    }
}

impl fmt::Display for Range<Magnitude> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let from =             match self.start_bound() {
            Bound::Included(ref mg) => format!("uploaded:>={}", mg),
            Bound::Excluded(ref mg) => format!("uploaded:>{}", mg),
            Bound::Unbounded => "".to_owned()
        };
        let to =             match self.end_bound() {
            Bound::Included(ref mg) => format!("uploaded:<={}", mg),
            Bound::Excluded(ref mg) => format!("uploaded:<{}", mg),
            Bound::Unbounded => "".to_owned()
        };
        
        write!(f, "{}{}{}", from, if !from.is_empty() && !to.is_empty() { "+" } else { "" },  to)
    }
}

impl From<Magnitude> for Range<Magnitude> {
    #[inline]
    fn from(magnitude: Magnitude) -> Self {
        Self {
            start: Bound::Included(magnitude),
            end: Bound::Unbounded,
        }
    }
}

impl From<(Magnitude, Magnitude)> for Range<Magnitude> {
    fn from((mut start, mut end): (Magnitude, Magnitude)) -> Self {
        if start < end {
            let temp = start;
            start = end;
            end = temp;
        }
        Self {
            start: Bound::Included(start),
            end: Bound::Excluded(end),
        }
    }
}

impl Range<Magnitude> {
    pub fn new(start: Bound<Magnitude>, end: Bound<Magnitude>) -> Self {
        Self { start, end }
    }

    #[inline]
    fn more_than_years(years: u16) -> Self {
        Self {
            start: Bound::Included(Magnitude::Year(years)),
            end: Bound::Unbounded,
        }
    }

    #[inline]
    fn more_than_months(months: u16) -> Self {
        Self {
            start: Bound::Included(Magnitude::Month(months)),
            end: Bound::Unbounded,
        }
    }

    #[inline]
    fn more_than_weeks(weeks: u16) -> Self {
        Self {
            start: Bound::Included(Magnitude::Week(weeks)),
            end: Bound::Unbounded,
        }
    }

    #[inline]
    fn more_than_days(days: u16) -> Self {
        Self {
            start: Bound::Included(Magnitude::Day(days)),
            end: Bound::Unbounded,
        }
    }

    #[inline]
    fn more_than_hours(hours: u16) -> Self {
        Self {
            start: Bound::Included(Magnitude::Hour(hours).correct()),
            end: Bound::Unbounded,
        }
    }
    #[inline]
    fn less_than_years(years: u16) -> Self {
        Self {
            start: Bound::Unbounded,
            end: Bound::Excluded(Magnitude::Year(years)),
        }
    }

    #[inline]
    fn less_than_months(months: u16) -> Self {
        Self {
            start: Bound::Unbounded,
            end: Bound::Excluded(Magnitude::Month(months)),
        }
    }

    #[inline]
    fn less_than_weeks(weeks: u16) -> Self {
        Self {
            start: Bound::Unbounded,
            end: Bound::Excluded(Magnitude::Week(weeks)),
        }
    }

    #[inline]
    fn less_than_days(days: u16) -> Self {
        Self {
            start: Bound::Unbounded,
            end: Bound::Excluded(Magnitude::Day(days)),
        }
    }

    #[inline]
    fn less_than_hours(hours: u16) -> Self {
        Self {
            start: Bound::Unbounded,
            end: Bound::Excluded(Magnitude::Hour(hours).correct()),
        }
    }

    #[inline]
    fn is_empty(&self) -> bool {
        (match self.start {
            Bound::Included(ref mg) => mg.is_empty(),
            Bound::Excluded(ref mg) => mg.is_empty(),
            Bound::Unbounded => true,
        } && match self.end {
            Bound::Included(ref mg) => mg.is_empty(),
            Bound::Excluded(ref mg) => mg.is_empty(),
            Bound::Unbounded => true,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Magnitude {
    Year(u16),
    Month(u16),
    Week(u16),
    Day(u16),
    Hour(u16),
}

impl fmt::Display for Magnitude {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Magnitude::Year(value) => write!(f, "{}y", value),
            Magnitude::Month(value) => write!(f, "{}m", value),
            Magnitude::Week(value) => write!(f, "{}w", value),
            Magnitude::Day(value) => write!(f, "{}d", value),
            Magnitude::Hour(value) => write!(f, "{}h", value),
        }
    }
}

impl Magnitude {
    fn correct(self) -> Self {
        match self {
            Self::Hour(value) => {
                if value > MAX_HOURS {
                    Magnitude::Day(value / 24)
                } else {
                    self
                }
            }
            _ => self,
        }
    }

    fn as_hours(&self) -> u32 {
        match self {
            Magnitude::Year(value) => *value as u32 * 8760,
            Magnitude::Month(value) => *value as u32 * 730,
            Magnitude::Week(value) => *value as u32 * 168,
            Magnitude::Day(value) => *value as u32 * 24,
            Magnitude::Hour(value) => *value as u32,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Magnitude::Year(value) => value == &0u16 || value == &1u16,
            Magnitude::Month(value) => value == &0u16 || value == &1u16,
            Magnitude::Week(value) => value == &0u16 || value == &1u16,
            Magnitude::Day(value) => value == &0u16 || value == &1u16,
            Magnitude::Hour(value) => value == &0u16 || value == &1u16,
        }
    }
}

impl PartialOrd for Magnitude {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.as_hours().cmp(&other.as_hours()))
    }
}

impl Ord for Magnitude {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_hours().cmp(&other.as_hours())
    }
}

mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn macro_page_full() {
        let range_1 = pages!(..);
        let range_2 = pages!(=>);
        assert_eq!(range_1, range_2);
        assert_eq!(range_1.to_string(), "".to_string())
    }

    #[test] 
    fn macro_page_from_to() {
        let range = pages!(0 => 5);
        assert_eq!(range.to_string(), "pages:>=0+pages:<5")
    }

    #[test] 
    fn macro_page_from() {
        let range = pages!(0 =>);
        assert_eq!(range.to_string(), "pages:>=0")
    }

    #[test] 
    fn macro_page_to() {
        let range = pages!(=> 5);
        assert_eq!(range.to_string(), "pages:<5")
    }

    #[test] 
    fn macro_page_from_to_equal() {
        let range = pages!(0 => =5);
        assert_eq!(range.to_string(), "pages:>=0+pages:<=5")
    }

    #[test] 
    fn macro_page_to_equal() {
        let range = pages!(=> =5);
        assert_eq!(range.to_string(), "pages:<=5")
    }

    #[test] 
    fn macro_page_unique() {
        let range = pages!(5 + 10);
        assert_eq!(range.to_string(), "pages:>=15+pages:<=15")
    }
}
