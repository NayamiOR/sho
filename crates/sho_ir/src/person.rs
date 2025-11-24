pub struct Person {
    name: String,
    alias: Vec<String>,
    description: Option<String>,
}

// Relationship between two Person
pub struct Relation {
    main_person: String,
    related_person: String,
    relationship: String,
    description: Option<String>,
}
