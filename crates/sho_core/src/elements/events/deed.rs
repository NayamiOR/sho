use crate::elements::context::time::Time;
use crate::id::Id;
use bon::Builder;

/// 事迹
/// 人物主动行为，有明确动作
#[derive(Debug, Builder, Clone)]
pub struct Deed {
    pub label: String,
    pub subject: Id,
    pub related: Vec<Id>,
    pub content: String,
    pub time: Option<Time>,
    pub result: Option<Id>,
    pub location: Option<Id>, // 预留给地点功能
    pub same: Option<Id>,     // 预留给重复事件
}
