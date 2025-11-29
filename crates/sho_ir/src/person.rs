use crate::time::IrTime;

#[derive(Debug, Clone)]
pub struct IrPerson {
    pub label: String,
    pub gender: Option<String>,
    pub surname: Option<String>,
    pub forename: Option<String>,
    pub pseudonym: Option<Vec<String>>, // 号
    pub courtesy_name: Option<String>,  // 字
    pub other_names: Option<Vec<String>>,
    pub nickname: Option<String>,
    pub birth_time: Option<IrTime>,
    pub death_time: Option<IrTime>,
}
