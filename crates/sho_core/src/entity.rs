use crate::elements::dynasty::Dynasty;
use crate::elements::person::Person;
use crate::elements::records::*;
use crate::elements::time::Time;
use crate::id::Id;

#[derive(Clone)]
pub struct Entity {
    id: Id,
    content: EntityContent,
}

#[derive(Clone)]
pub enum EntityContent {
    Person(Person),
    Dynasty(Dynasty),
    Time(Time),
    Deed(Deed),
    State(State),
    Utterance(Utterance),
    Relation(Relation),
    Institution(Institution),
    Fact(Fact),
    Episode(Episode),
    Assessment(Assessment),
}

impl From<Person> for Entity {
    fn from(value: Person) -> Self {
        Entity {
            id: Id::new(),
            content: EntityContent::Person(value),
        }
    }
}

impl From<Time> for Entity {
    fn from(value: Time) -> Self {
        Entity {
            id: Id::new(),
            content: EntityContent::Time(value),
        }
    }
}

impl From<Deed> for Entity {
    fn from(value: Deed) -> Self {
        Entity {
            id: Id::new(),
            content: EntityContent::Deed(value),
        }
    }
}

impl From<State> for Entity {
    fn from(value: State) -> Self {
        Entity {
            id: Id::new(),
            content: EntityContent::State(value),
        }
    }
}

impl From<Utterance> for Entity {
    fn from(value: Utterance) -> Self {
        Entity {
            id: Id::new(),
            content: EntityContent::Utterance(value),
        }
    }
}

impl From<Relation> for Entity {
    fn from(value: Relation) -> Self {
        Entity {
            id: Id::new(),
            content: EntityContent::Relation(value),
        }
    }
}

impl From<Institution> for Entity {
    fn from(value: Institution) -> Self {
        Entity {
            id: Id::new(),
            content: EntityContent::Institution(value),
        }
    }
}

impl From<Fact> for Entity {
    fn from(value: Fact) -> Self {
        Entity {
            id: Id::new(),
            content: EntityContent::Fact(value),
        }
    }
}

impl From<Episode> for Entity {
    fn from(value: Episode) -> Self {
        Entity {
            id: Id::new(),
            content: EntityContent::Episode(value),
        }
    }
}

impl From<Assessment> for Entity {
    fn from(value: Assessment) -> Self {
        Entity {
            id: Id::new(),
            content: EntityContent::Assessment(value),
        }
    }
}
