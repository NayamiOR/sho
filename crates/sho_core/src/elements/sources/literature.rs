use crate::elements::context::Time;
use crate::elements::sources::author::Author;
use bon::Builder;
/// 文献典籍
/// 历史研究的原始文献材料
#[derive(Debug, Builder, Clone)]
pub struct Literature {
    pub name: String,
    pub alias: Option<Vec<String>>,      // 别名（可选）
    pub authors: Vec<Author>,            // 作者（可为多个或"佚名"）
    pub completion_time: Option<Time>,   // 成书时间
    pub literature_type: LiteratureType, // 类型
    pub summary: Option<String>,         // 主要内容/摘要
    pub preservation_status: Option<PreservationStatus>, // 流传情况（可选）
}

/// 文献类型
#[derive(Debug, Clone)]
pub enum LiteratureType {
    History,       // 史书
    Philosophy,    // 子部（经史子集中的子）
    Military,      // 兵书
    Literature,    // 文学作品
    Other(String), // 其他类型
}

/// 流传情况
#[derive(Debug, Clone)]
pub enum PreservationStatus {
    Complete,    // 完本
    Fragmentary, // 残本
    Lost,        // 已佚
}
