use crate::id::Id;
use bon::Builder;

/// 评价
/// 史家或时人的价值判断
#[derive(Debug, Builder, Clone)]
pub struct Assessment {
    pub subject: Id,
    pub object: Id,
    pub content: String,
    pub origin: Id, // 来源，Id应指向史料节点
}
