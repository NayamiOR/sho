use crate::id::Id;

#[derive(Debug, Clone)]
pub enum Time {
    Timepoint(Timepoint),
    TimeRange(TimeRange),
}

#[derive(Debug, Clone)]
pub enum Timepoint {
    ExactTime(ExactTime),
    RoughTime(RoughTime),
}

#[derive(Debug, Clone)]
pub struct ExactTime {
    pub year: Year,
    pub month_day: MonthDay,
}

type Year = i32; // Use i32 as Year, change to struct or enum if  needed

#[derive(Debug, Clone)]
pub struct MonthDay {
    pub month: Option<u32>,
    pub day: Option<u32>,
}

#[derive(Debug, Clone)]
pub enum TimeRange {
    BetweenFact(Id, Id),
    BetweenTime(Timepoint, Timepoint),
}

#[derive(Debug, Clone)]
pub enum RoughTime {
    Century(i8),
}
