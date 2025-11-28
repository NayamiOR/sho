use crate::id::Id;
use crate::time::Time;
use bon::Builder;

// 事迹（带有某人的主观能动性）
#[derive(Debug, Builder)]
pub struct Deed {
    pub label: String,
    pub subject: Id,
    pub related: Vec<Id>,
    pub content: String,
    pub time: Option<Time>,
    pub result: Option<Id>,
    pub location: Option<Id>, // 预留给地点功能
}

// 状态（或者属性变化）
#[derive(Debug, Builder)]
pub struct State {} // TODO: 目前没有好设计，先预留

// 言论
#[derive(Debug, Builder)]
pub struct Utterance {
    pub subject: Id,
    pub content: String,
    pub time: Option<Time>,
    pub related: Vec<Id>,
    pub source: Id,
    // NOTE: 考虑添加语境
}

// 关系
#[derive(Debug, Builder)]
pub struct Relation {
    pub subject: Id,
    pub object: Vec<Id>,
    pub relationship: RelationShip,
    pub relation_text: String, // 具体描述，比如选择Sibling的时候可以备注具体关系
    pub time: Option<Time>,
    pub description: String,
}

// 关系分类
#[derive(Debug)]
pub enum RelationShip {
    Parent,
    Child,
    Sibling,
    Cousin,
    FellowTown,
    // TODO: Complete
}

// 制度
#[derive(Debug, Builder)]
pub struct Institution {
    pub label: String,
    pub subject: Id,
    pub content: String,
    pub start_time: Option<Time>,
    pub source: Id,
}

// NOTE: Fact和Episode替代Event，一个是原子事件一个是大事件（事件集合）

// 原子事件
#[derive(Debug, Builder)]
pub struct Fact {
    pub time: Time,
    pub related: Vec<Id>,
    pub content: String,
    pub location: Option<Id>, // 预留给地点功能
}

// 大事件（原子事件集合）
#[derive(Debug, Builder)]
pub struct Episode {
    pub time: Time,
    pub related: Vec<Id>,
    pub sub_facts: Vec<Id>,
    pub result: Id, // 表示结果的Fact的ID
}

// 评价
#[derive(Debug, Builder)]
pub struct Assessment {
    pub subject: Id,
    pub object: Id,
    pub content: String,
    pub origin: Id, // 来源，Id应指向史料节点
}
