use crate::id::Id;
use crate::time::Time;
use bon::{builder, Builder};

#[derive(Debug, Builder)]
pub struct Person {
    pub label: String,
    pub gender: Option<Gender>,
    pub surname: Option<String>,
    pub forename: Option<String>,
    pub pseudonym: Option<Vec<String>>, // 号
    pub courtesy_name: Option<String>,  // 字
    pub other_names: Option<Vec<String>>,
    pub nickname: Option<String>,
    pub birth_time: Option<Time>,
    pub death_time: Option<Time>,
}

#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
    Other,
}
