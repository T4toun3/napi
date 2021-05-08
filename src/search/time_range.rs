
use std::cmp::Ordering;
use std::fmt;

const MAX_HOURS: u16 = 4999;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TimeRange {
    start: Magnitude,
    end: Magnitude,
}

impl fmt::Display for TimeRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.start.is_empty() {
            write!(f, "uploaded:<{}", self.end)
        } else if self.end.is_empty() {
            write!(f, "uploaded:>{}", self.start)
        } else {
            write!(f, "uploaded:>{}+uploaded:<{}", self.start, self.end)
        }
    }
}

impl From<Magnitude> for TimeRange {
    #[inline]
    fn from(magnitude: Magnitude) -> Self {
        Self {
            start: magnitude,
            end: Magnitude::Year(u16::MAX),
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
            start,
            end
        }
    }
}

impl TimeRange {
    #[inline]
    fn from_years(years: u16) -> Self {
        Self {
            start: Magnitude::Year(years),
            end: Magnitude::Year(u16::MAX),
        }
    }

    #[inline]
    fn from_months(months: u16) -> Self {
        Self {
            start: Magnitude::Month(months),
            end: Magnitude::Year(u16::MAX),
        }
    }

    #[inline]
    fn from_weeks(weeks: u16) -> Self {
        Self {
            start: Magnitude::Week(weeks),
            end: Magnitude::Year(u16::MAX),
        }
    }
    
    #[inline]
    fn from_days(days: u16) -> Self {
        Self {
            start: Magnitude::Day(days),
            end: Magnitude::Year(u16::MAX),
        }
    }

    #[inline]
    fn from_hours(hours: u16) -> Self {
        Self {
            start: Magnitude::Hour(hours).correct(),
            end: Magnitude::Year(u16::MAX),
        }
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.start.is_empty() && self.end.is_empty()
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
            _ => self
        }
    }

    fn as_hours(&self) -> u32 {
        match self {
            Magnitude::Year(value) => *value as u32 * 8760,
            Magnitude::Month(value) => *value as u32  * 730,
            Magnitude::Week(value) => *value as u32  * 168,
            Magnitude::Day(value) => *value as u32  * 24,
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

