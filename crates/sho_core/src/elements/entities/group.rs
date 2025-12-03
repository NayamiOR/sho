use crate::elements::context::Time;
use crate::id::Id;
use bon::Builder;

/// 团体/势力/意识形态
/// 指历史事件中涉及的团体、势力、意识形态等组织形式
#[derive(Debug, Builder, Clone)]
pub struct Group {
    pub name: String,
    pub group_type: GroupType,
    pub representatives: Option<Vec<Id>>,     // 代表人物/领袖
    pub establishment_time: Option<Time>,     // 建立时间（可选）
    pub end_time: Option<Time>,               // 结束时间（可选）
    pub core_ideology: Option<String>,        // 核心理念/宗旨（可选）
    pub main_activity_region: Option<String>, // 主要活动区域（可选）
    pub description: Option<String>,          // 描述
}

/// 团体类型
#[derive(Debug, Clone)]
pub enum GroupType {
    Regime,            // 政权
    MilitaryGroup,     // 军事集团
    AcademicSchool,    // 学术流派
    CivilOrganization, // 民间组织
    Other(String),     // 其他类型
}
