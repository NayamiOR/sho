#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum IrTime {
    Exact { year: Year, month_day: MonthDay },
    YearSeason(Year, Season),
    Century(i8),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct MonthDay {
    pub month: Option<u32>,
    pub day: Option<u32>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[derive(Debug, Clone)]
pub struct ReignMotto {
    name: String,
    emperor: String,
    dynasty: String,
    start_bc_year: Option<u32>,
    end_bc_year: Option<u32>,
}
