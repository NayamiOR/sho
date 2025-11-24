use crate::time::Time;

pub struct Fact {
    pub label: String,
    pub alias: Vec<String>,
    pub person: Vec<String>,
    pub content: String,
    pub time: Time,
}

pub struct Episode {
    pub label: String,
    pub alias: Vec<String>,
    pub facts: Vec<String>,
    pub time: Time,
}
