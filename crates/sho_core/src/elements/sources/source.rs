use crate::elements::context::Time;
use crate::elements::sources::author::Author;
use crate::id::Id;
use bon::Builder;

/// 史料/来源
/// 历史研究的原始材料及其来源
#[derive(Debug, Builder, Clone)]
pub struct Source {
    pub name: String,                  // 名称（如：《三国志·卷三十六·蜀书六·关羽传》）
    pub literature: Option<Id>,        // 所属典籍（引用自"文献典籍"元素）
    pub author: Vec<Author>,           // 作者
    pub completion_time: Option<Time>, // 成书年代
    pub source_nature: Option<SourceNature>, // 史料性质（可选）
    pub reliability_assessment: Option<String>, // 可靠性评估（可选，简要描述其价值与局限性）
}

/// 史料性质
#[derive(Debug, Clone)]
pub enum SourceNature {
    PrimarySource,     // 一手史料
    SecondarySource,   // 二手史料
    OfficialHistory,   // 官方史书
    UnofficialHistory, // 野史笔记
    Other(String),     // 其他类型
}
