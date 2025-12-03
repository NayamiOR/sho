use crate::elements::context::time::Time;
use crate::id::Id;
use bon::Builder;

/// 制度
/// 政治/军事/经济规则
#[derive(Debug, Builder, Clone)]
pub struct Institution {
    pub label: String,
    pub subject: Id,
    pub content: String,
    pub start_time: Option<Time>,
    pub source: Id,
}
