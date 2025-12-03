use crate::id::Id;

/// 朝代
/// 界定历史事件发生的朝代背景
#[derive(Clone, Debug)]
pub struct Dynasty {
    pub name: String,
    pub start_bc_year: Option<u32>,
    pub first_motto: Option<Id>, // 最早的年号，起到类似链表头的作用
}
