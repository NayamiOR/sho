use crate::id::Id;

pub enum HistoricalTime {
    ExactTime(ExactTime),
    RoughTime(RoughTime),
    TimeRange(TimeRange),
}
pub struct ExactTime {
    pub year: Year,
    pub month_day: MonthDay,
}

pub enum Year {
    CommonEra(i32),
    ChineseEra {
        motto: Id, // 年号年
        year: u32,
    },
    Ganzhi(Ganzhi), // 干支年
    Regnal {
        ruler: String,
        year: u32,
    },
}

pub struct MonthDay {
    pub month: Option<u32>,
    pub day: Option<u32>,
}

pub enum TimeRange {
    BetweenTime(ExactTime, ExactTime),
    BetweenFact(Id, Id),
}

pub enum RoughTime {
    YearSeason(Year, Season),
    Century(i8),
}

pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

pub struct ReignMotto {
    id: Id,
    name: String,
    emperor: Id,
    dynasty: Id,
    start_bc_year: Option<u32>,
    end_bc_year: Option<u32>,
}

pub enum Ganzhi {
    JiaZi,    // 甲子
    YiChou,   // 乙丑
    BingYin,  // 丙寅
    DingMao,  // 丁卯
    WuChen,   // 戊辰
    JiSi,     // 己巳
    GengWu,   // 庚午
    XinWei,   // 辛未
    RenShen,  // 壬申
    GuiYou,   // 癸酉
    JiaXu,    // 甲戌
    YiHai,    // 乙亥
    BingZi,   // 丙子
    DingChou, // 丁丑
    WuYin,    // 戊寅
    JiMao,    // 己卯
    GengChen, // 庚辰
    XinSi,    // 辛巳
    RenWu,    // 壬午
    GuiWei,   // 癸未
    JiaShen,  // 甲申
    YiYou,    // 乙酉
    BingXu,   // 丙戌
    DingHai,  // 丁亥
    WuZi,     // 戊子
    JiChou,   // 己丑
    GengYin,  // 庚寅
    XinMao,   // 辛卯
    RenChen,  // 壬辰
    GuiSi,    // 癸巳
    JiaWu,    // 甲午
    YiWei,    // 乙未
    BingShen, // 丙申
    DingYou,  // 丁酉
    WuXu,     // 戊戌
    JiHai,    // 己亥
    GengZi,   // 庚子
    XinChou,  // 辛丑
    RenYin,   // 壬寅
    GuiMao,   // 癸卯
    JiaChen,  // 甲辰
    YiSi,     // 乙巳
    BingWu,   // 丙午
    DingWei,  // 丁未
    WuShen,   // 戊申
    JiYou,    // 己酉
    GengXu,   // 庚戌
    XinHai,   // 辛亥
    RenZi,    // 壬子
    GuiChou,  // 癸丑
    JiaYin,   // 甲寅
    YiMao,    // 乙卯
    BingChen, // 丙辰
    DingSi,   // 丁巳
    WuWu,     // 戊午
    JiWei,    // 己未
    GengShen, // 庚申
    XinYou,   // 辛酉
    RenXu,    // 壬戌
    GuiHai,   // 癸亥
}
