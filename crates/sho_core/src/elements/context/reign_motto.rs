use crate::elements::context::Time;
use crate::id::Id;
use bon::Builder;

/// 年号
/// 界定历史事件发生的年号背景
#[derive(Debug, Builder, Clone)]
pub struct ReignMotto {
    pub name: String,
    pub emperor: Id,
    pub dynasty: Id,
    pub start_time: Option<Time>,
    pub end_time: Option<Time>,
    pub previous_reign_motto: Option<Id>, // 上一个年号（可选）
    pub next_reign_motto: Option<Id>,     // 下一个年号（可选）
}
