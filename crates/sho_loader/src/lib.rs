use sho_core::elements::context::Time;
use sho_core::elements::entities::person::{Gender, Person};
use sho_core::elements::events::{Deed, Episode, Fact, Utterance};
use sho_core::elements::sources::Assessment;
use sho_core::elements::structures::{Institution, Relation, RelationShip};
use sho_core::id::Id;
use sho_core::object::{Object, ObjectContent};
use sho_ir::person::IrGender;
use sho_ir::records::IrRelationship;
use sho_ir::time::IrTime;
use sho_ir::IrNode;
use std::collections::HashMap;
use std::hash::Hash;

type SymbolTable = HashMap<String, (Id, IrNode)>;

// Return SymbolTable and orphan nodes
pub fn parse_symbol_table(input: Vec<IrNode>) -> (SymbolTable, Vec<IrNode>) {
    let mut symbol_table = HashMap::new();
    let mut orphans = vec![];
    input.into_iter().for_each(|node| match node.get_label() {
        Some(label) => {
            symbol_table.insert(label.into(), (Id::new(), node));
        }
        None => {
            orphans.push(node);
        }
    });
    (symbol_table, orphans)
}

pub fn parse_entity_table(
    input: Vec<IrNode>,
    symbol_table: SymbolTable,
    orphans: Vec<IrNode>,
) -> Result<HashMap<Id, Object>, Vec<Box<dyn std::error::Error>>> {
    let mut errors = vec![];
    let mut entity_table = HashMap::new();
    orphans
        .into_iter()
        .map(|node| evaluate(node, &symbol_table))
        .for_each(|entity: ObjectContent| {
            let id = Id::new();
            entity_table.insert(
                id,
                Object {
                    id,
                    content: entity.into(),
                },
            );
        });

    if errors.is_empty() {
        Ok(entity_table)
    } else {
        Err(errors)
    }
}

// IR -> Core
fn evaluate(node: IrNode, symbol_table: &SymbolTable) -> ObjectContent {
    // 把IrNode变成Core结构
    match node {
        IrNode::Person(n) => {
            // let person = Person::builder().label(n.label);
            ObjectContent::Person(Person {
                label: n.label,
                gender: n.gender.map(|x| match x {
                    IrGender::Male => Gender::Male,
                    IrGender::Female => Gender::Female,
                    _ => Gender::Other,
                }),
                surname: n.surname,
                forename: n.forename,
                pseudonym: n.pseudonym,
                courtesy_name: n.courtesy_name,
                other_names: n.other_names,
                nickname: n.nickname,
                birth_time: n.birth_time.map(parse_time),
                death_time: n.death_time.map(parse_time),
            })
        }
        IrNode::Deed(n) => ObjectContent::Deed(Deed {
            label: n.label,
            subject: symbol_table.get(&n.subject).unwrap().0,
            related: n
                .related
                .iter()
                .map(|x| symbol_table.get(x).unwrap().0)
                .collect(),
            content: n.content,
            time: n.time.map(parse_time),
            result: n.result.map(|x| symbol_table.get((&x).into()).unwrap().0),
            location: None,
            same: None, // IrDeed 中暂无 same 字段
        }),
        IrNode::Utterance(n) => ObjectContent::Utterance(Utterance {
            subject: symbol_table.get(&n.subject).unwrap().0,
            content: n.content,
            time: n.time.map(parse_time),
            related: n
                .related
                .iter()
                .map(|x| symbol_table.get(x).unwrap().0)
                .collect(),
            source: symbol_table.get(&n.source).unwrap().0,
        }),
        IrNode::Relation(n) => ObjectContent::Relation(Relation {
            subject: symbol_table.get(&n.subject).unwrap().0,
            object: n
                .object
                .iter()
                .map(|x| symbol_table.get(x).unwrap().0)
                .collect(),
            relationship: link_relationship(n.relationship),
            relation_text: n.relation_text,
            time: n.time.map(parse_time),
            description: n.description,
        }),
        IrNode::Institution(n) => ObjectContent::Institution(Institution {
            label: n.label,
            subject: symbol_table.get(&n.subject).unwrap().0,
            content: n.content,
            start_time: n.start_time.map(parse_time),
            source: symbol_table.get(&n.source).unwrap().0,
        }),
        IrNode::Fact(n) => ObjectContent::Fact(Fact {
            time: parse_time(n.time),
            related: n
                .related
                .iter()
                .map(|x| symbol_table.get(x).unwrap().0)
                .collect(),
            content: n.content,
            location: None,
            same: n.same.map(|x| symbol_table.get(&x).unwrap().0),
        }),
        IrNode::Episode(n) => ObjectContent::Episode(Episode {
            time: parse_time(n.time),
            related: n
                .related
                .iter()
                .map(|x| symbol_table.get(x).unwrap().0)
                .collect(),
            sub_facts: n
                .sub_facts
                .iter()
                .map(|x| symbol_table.get(x).unwrap().0)
                .collect(),
            result: symbol_table.get(&n.result).unwrap().0,
            same: n.same.map(|x| symbol_table.get(&x).unwrap().0),
        }),
        IrNode::Assessment(n) => ObjectContent::Assessment(Assessment {
            subject: symbol_table.get(&n.subject).unwrap().0,
            object: symbol_table.get(&n.object).unwrap().0,
            content: n.content,
            origin: symbol_table.get(&n.origin).unwrap().0,
        }),
    }
}

fn parse_time(ir_time: IrTime) -> Time {
    todo!()
}

fn link_relationship(ir_relationship: IrRelationship) -> RelationShip {
    match ir_relationship {
        IrRelationship::Parent => RelationShip::Parent,
        IrRelationship::Child => RelationShip::Child,
        IrRelationship::Sibling => RelationShip::Sibling,
        IrRelationship::Cousin => RelationShip::Cousin,
        IrRelationship::FellowTown => RelationShip::FellowTown,
    }
}
