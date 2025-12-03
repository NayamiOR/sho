use crate::elements::context::{Dynasty, Location, ReignMotto, Time};
use crate::elements::entities::{Group, Item, Person};
use crate::elements::events::{Deed, Episode, Fact, Utterance};
use crate::elements::sources::{Assessment, Literature, Source};
use crate::elements::structures::{Institution, Relation, State};
use crate::id::Id;

#[derive(Clone)]
pub struct Object {
    pub id: Id,
    pub content: ObjectContent,
}

#[derive(Clone)]
pub enum ObjectContent {
    // 核心实体 (Core Entities)
    Person(Person),
    Item(Item),
    Group(Group),
    // 时地 (Temporal & Spatial Context)
    Dynasty(Dynasty),
    ReignMotto(ReignMotto),
    Location(Location),
    Time(Time),
    // 事件与行为 (Events & Actions)
    Deed(Deed),
    Fact(Fact),
    Episode(Episode),
    Utterance(Utterance),
    // 情势与体系 (Situations & Systems)
    State(State),
    Relation(Relation),
    Institution(Institution),
    // 史料与解读 (Historical Sources & Interpretation)
    Assessment(Assessment),
    Literature(Literature),
    Source(Source),
}

impl From<Person> for Object {
    fn from(value: Person) -> Self {
        Object {
            id: Id::new(),
            content: ObjectContent::Person(value),
        }
    }
}

impl From<Dynasty> for Object {
    fn from(value: Dynasty) -> Self {
        Object {
            id: Id::new(),
            content: ObjectContent::Dynasty(value),
        }
    }
}

impl From<Time> for Object {
    fn from(value: Time) -> Self {
        Object {
            id: Id::new(),
            content: ObjectContent::Time(value),
        }
    }
}

impl From<Deed> for Object {
    fn from(value: Deed) -> Self {
        Object {
            id: Id::new(),
            content: ObjectContent::Deed(value),
        }
    }
}

impl From<State> for Object {
    fn from(value: State) -> Self {
        Object {
            id: Id::new(),
            content: ObjectContent::State(value),
        }
    }
}

impl From<Utterance> for Object {
    fn from(value: Utterance) -> Self {
        Object {
            id: Id::new(),
            content: ObjectContent::Utterance(value),
        }
    }
}

impl From<Relation> for Object {
    fn from(value: Relation) -> Self {
        Object {
            id: Id::new(),
            content: ObjectContent::Relation(value),
        }
    }
}

impl From<Institution> for Object {
    fn from(value: Institution) -> Self {
        Object {
            id: Id::new(),
            content: ObjectContent::Institution(value),
        }
    }
}

impl From<Fact> for Object {
    fn from(value: Fact) -> Self {
        Object {
            id: Id::new(),
            content: ObjectContent::Fact(value),
        }
    }
}

impl From<Episode> for Object {
    fn from(value: Episode) -> Self {
        Object {
            id: Id::new(),
            content: ObjectContent::Episode(value),
        }
    }
}

impl From<Assessment> for Object {
    fn from(value: Assessment) -> Self {
        Object {
            id: Id::new(),
            content: ObjectContent::Assessment(value),
        }
    }
}

impl From<Item> for Object {
    fn from(value: Item) -> Self {
        Object {
            id: Id::new(),
            content: ObjectContent::Item(value),
        }
    }
}

impl From<Group> for Object {
    fn from(value: Group) -> Self {
        Object {
            id: Id::new(),
            content: ObjectContent::Group(value),
        }
    }
}

impl From<ReignMotto> for Object {
    fn from(value: ReignMotto) -> Self {
        Object {
            id: Id::new(),
            content: ObjectContent::ReignMotto(value),
        }
    }
}

impl From<Location> for Object {
    fn from(value: Location) -> Self {
        Object {
            id: Id::new(),
            content: ObjectContent::Location(value),
        }
    }
}

impl From<Literature> for Object {
    fn from(value: Literature) -> Self {
        Object {
            id: Id::new(),
            content: ObjectContent::Literature(value),
        }
    }
}

impl From<Source> for Object {
    fn from(value: Source) -> Self {
        Object {
            id: Id::new(),
            content: ObjectContent::Source(value),
        }
    }
}
