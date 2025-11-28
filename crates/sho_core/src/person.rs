use crate::id::Id;
use crate::time::Time;

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

impl Default for Person {
    // 不打算实现new，推荐使用default配合自定义字段来初始化
    fn default() -> Self {
        Person {
            label: String::new(),
            gender: None,
            surname: None,
            forename: None,
            pseudonym: None,
            courtesy_name: None,
            other_names: None,
            nickname: None,
            birth_time: None,
            death_time: None,
        }
    }
}

pub enum Gender {
    Male,
    Female,
    Other,
}
