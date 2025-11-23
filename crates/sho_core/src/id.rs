use uuid::Uuid;
pub struct Id(pub Uuid);

impl Id {
    fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}
