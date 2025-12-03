use crate::elements::context::time::Time;
use crate::id::Id;
use bon::Builder;

/// 关系
/// 实体间的持久关联
#[derive(Debug, Builder, Clone)]
pub struct Relation {
    pub subject: Id,
    pub object: Vec<Id>,
    pub relationship: RelationShip,
    pub relation_text: String, // 具体描述，比如选择Sibling的时候可以备注具体关系
    pub time: Option<Time>,
    pub description: String,
}

/// 关系分类
#[derive(Debug, Clone)]
pub enum RelationShip {
    Parent,
    Child,
    Sibling,
    Cousin,
    FellowTown,
    // TODO: Complete
}
