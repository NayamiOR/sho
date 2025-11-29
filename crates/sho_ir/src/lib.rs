use crate::person::IrPerson;
use crate::records::{
    IrAssessment, IrDeed, IrEpisode, IrFact, IrInstitution, IrRelation, IrUtterance,
};

pub mod person;
pub mod possibility;
pub mod records;
pub mod time;

/// --- 总的 IR 节点枚举 ---
/// Parser 的最终输出是一个 `Vec<IrNode>`。
/// 这个枚举囊括了所有可以在 .sho 文件中定义的顶级实体。
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum IrNode {
    Person(IrPerson),
    Deed(IrDeed),
    Utterance(IrUtterance),
    Relation(IrRelation),
    Institution(IrInstitution),
    Fact(IrFact),
    Episode(IrEpisode),
    Assessment(IrAssessment),
    // State(IrState) // 如果 State 也可被创建，则加入
}

impl IrNode {
    /// 辅助函数，用于 Loader 方便地获取任何节点的标签。
    pub fn get_label(&self) -> Option<&str> {
        match self {
            IrNode::Person(n) => Some(&n.label),
            IrNode::Deed(n) => Some(&n.label),
            IrNode::Utterance(n) => n.label.as_deref(),
            IrNode::Relation(n) => n.label.as_deref(),
            IrNode::Institution(n) => Some(&n.label),
            IrNode::Fact(n) => n.label.as_deref(),
            IrNode::Episode(n) => Some(&n.label),
            IrNode::Assessment(n) => n.label.as_deref(),
        }
    }
}
