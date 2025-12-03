use crate::elements::context::time::Time;
use crate::id::Id;
use bon::Builder;

/// 大事件（原子事件集合）
/// 有明确起止的动态过程，由多个原子事件组成
#[derive(Debug, Builder, Clone)]
pub struct Episode {
    pub time: Time,
    pub related: Vec<Id>,
    pub sub_facts: Vec<Id>,
    pub result: Id,       // 表示结果的Fact的ID
    pub same: Option<Id>, // 预留给重复事件
}
