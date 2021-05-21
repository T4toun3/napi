use std::cmp::Ordering;
use std::fmt;
use std::ops::Bound;

const MAX_HOURS: u16 = 4999;

// ! I haven't done any test for this feature yet
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TimeRange {
    start: Bound<Magnitude>,
    end: Bound<Magnitude>,
}

impl fmt::Display for TimeRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            vec![
                match self.start {
                    Bound::Included(ref mg) => format!("uploaded:>={}", mg),
                    Bound::Excluded(ref mg) => format!("uploaded:>{}", mg),
                    Bound::Unbounded => "".to_owned(),
                },
                match self.end {
                    Bound::Included(ref mg) => format!("uploaded:<={}", mg),
                    Bound::Excluded(ref mg) => format!("uploaded:<{}", mg),
                    Bound::Unbounded => "".to_owned(),
                },
            ]
            .join("+")
            .as_str(),
        )
    }
}

impl From<Magnitude> for TimeRange {
    #[inline]
    fn from(magnitude: Magnitude) -> Self {
        Self {
            start: Bound::Included(magnitude),
            end: Bound::Unbounded,
        }
    }
}

impl From<(Magnitude, Magnitude)> for TimeRange {
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

impl TimeRange {
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
