use crate::elements::context::time::Time;
use crate::id::Id;
use bon::Builder;

/// 原子事件
/// 有明确起止的动态过程
#[derive(Debug, Builder, Clone)]
pub struct Fact {
    pub time: Time,
    pub related: Vec<Id>,
    pub content: String,
    pub location: Option<Id>, // 预留给地点功能
    pub same: Option<Id>,     // 预留给重复事件
}
