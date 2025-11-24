pub enum Time {
    Exact { year: Year, month_day: MonthDay },
    YearSeason(Year, Season),
    Century(i8),
}

pub enum Year {
    CommonEra(i32),
    ChineseEra {
        motto: String, // 年号年
        year: u32,
    },
    Regnal {
        ruler: String,
        year: u32,
    },
}

pub struct MonthDay {
    pub month: Option<u32>,
    pub day: Option<u32>,
}

pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}
