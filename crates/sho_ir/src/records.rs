use crate::time::IrTime;

#[derive(Debug, Clone, PartialEq)]
pub enum IrSeason {
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum IrRelationship {
    Parent,
    Child,
    Sibling,
    Cousin,
    FellowTown,
    // ... 其他关系
}

// --- IR 节点定义 ---

// 对应 core::Deed
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct IrDeed {
    pub label: String,            // 事迹的标签，例如 "官渡之战"
    pub subject: String,          // 发起者的标签
    pub related: Vec<String>,     // 其他相关实体的标签，用关键词字面量表示
    pub content: String,          // 内容描述
    pub time: Option<IrTime>,     // 发生时间
    pub result: Option<String>,   // 结果的标签 (通常指向一个 Fact)
    pub location: Option<String>, // 地点标签, 先预留好
}

// 对应 core::State (保持为 TODO)
#[derive(Debug, Clone)]
pub struct IrState {
    // 待设计
}

// 对应 core::Utterance
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct IrUtterance {
    pub label: Option<String>, // 言论可选的标签，用于被引用
    pub subject: String,       // 发表者的标签
    pub content: String,       // 内容
    pub time: Option<IrTime>,  // 发表时间
    pub related: Vec<String>,  // 相关实体的标签
    pub source: String,        // 来源的标签 (指向史料)
}

// 对应 core::Relation
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct IrRelation {
    pub label: Option<String>, // 关系可选的标签，例如 "曹操与曹丕的父子关系"
    pub subject: String,       // 关系主体
    pub object: Vec<String>,   // 关系客体
    pub relationship: IrRelationship, // 关系类型
    pub relation_text: String, // 关系的具体文本描述
    pub time: Option<IrTime>,  // 关系存续的时间
    pub description: String,   // 补充描述
}

// 对应 core::Institution
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct IrInstitution {
    pub label: String,              // 制度的标签，例如 "九品中正制"
    pub subject: String,            // 相关主体的标签 (例如 "曹魏")
    pub content: String,            // 内容描述
    pub start_time: Option<IrTime>, // 开始时间
    pub source: String,             // 来源的标签
}

// 对应 core::Fact
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct IrFact {
    pub label: Option<String>,    // 原子事件可选的标签，用于被引用
    pub time: IrTime,             // 发生时间 (Fact 的时间是必须的)
    pub related: Vec<String>,     // 相关实体的标签
    pub content: String,          // 内容描述
    pub location: Option<String>, // 地点标签
    pub same: Option<String>,     // 指向相同事件的标签
}

// 对应 core::Episode
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct IrEpisode {
    pub label: String,          // 大事件的标签，例如 "赤壁之战始末"
    pub time: IrTime,           // 整个事件的时间跨度
    pub related: Vec<String>,   // 相关实体的标签
    pub sub_facts: Vec<String>, // 包含的原子事件的标签
    pub result: String,         // 指向作为结果的 Fact 的标签
    pub same: Option<String>,   // 指向相同事件的标签
}

// 对应 core::Assessment
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct IrAssessment {
    pub label: Option<String>, // 评价可选的标签
    pub subject: String,       // 评价者的标签 (谁做的评价)
    pub object: String,        // 被评价对象的标签
    pub content: String,       // 评价内容
    pub origin: String,        // 来源的标签 (例如 "《三国志》的评价")
}
