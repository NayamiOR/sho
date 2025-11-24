use crate::id::Id;

pub enum Time {
    Timepoint(Timepoint),
    TimeRange(TimeRange),
}

pub enum Timepoint {
    ExactTime(ExactTime),
    RoughTime(RoughTime),
}

pub struct ExactTime {
    pub year: Year,
    pub month_day: MonthDay,
}

type Year = i32; // Use i32 as Year, change to struct or enum if  needed

pub struct MonthDay {
    pub month: Option<u32>,
    pub day: Option<u32>,
}

pub enum TimeRange {
    BetweenFact(Id, Id),
    BetweenTime(Timepoint, Timepoint),
}

pub enum RoughTime {
    Century(i8),
}
