use crate::dynasty::Dynasty;
use crate::id::Id;
use crate::person::Person;

pub struct Entity{
    id: Id,
    content: EntityContent,
}

pub enum EntityContent{
    Person(Person),
    Dynasty(Dynasty),
    
}