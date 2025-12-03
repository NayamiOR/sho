use crate::elements::context::time::Time;
use crate::id::Id;
use bon::Builder;

/// 言论
/// 人物直接或转述的言论
#[derive(Debug, Builder, Clone)]
pub struct Utterance {
    pub subject: Id,
    pub content: String,
    pub time: Option<Time>,
    pub related: Vec<Id>,
    pub source: Id,
    // NOTE: 考虑添加语境
}
