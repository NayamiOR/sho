use crate::elements::context::Time;
use crate::id::Id;
use bon::Builder;

/// 物品
/// 指历史事件中涉及的物品、器物等客体
#[derive(Debug, Builder, Clone)]
pub struct Item {
    pub name: String,
    pub item_type: ItemType,
    pub owners: Option<Vec<Ownership>>, // 所有者（可为多个并注明时间）
    pub origin: Option<String>,         // 出处/来源（可选）
    pub description: Option<String>,    // 描述（外观、功能、相关典故等）
}

/// 物品类型
#[derive(Debug, Clone)]
pub enum ItemType {
    Weapon,        // 兵器
    Mount,         // 坐骑
    Treasure,      // 宝物
    Other(String), // 其他类型
}

/// 所有权记录
#[derive(Debug, Clone)]
pub struct Ownership {
    pub owner: Id,                // 所有者ID
    pub start_time: Option<Time>, // 开始拥有时间
    pub end_time: Option<Time>,   // 结束拥有时间
}
