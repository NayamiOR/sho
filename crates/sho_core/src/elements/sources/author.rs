use crate::id::Id;

/// 作者信息
#[derive(Debug, Clone)]
pub enum Author {
    Named(Id), // 有名作者
    Anonymous, // 佚名
}
