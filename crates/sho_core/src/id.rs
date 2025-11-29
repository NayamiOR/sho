use uuid::Uuid;

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub struct Id(pub Uuid);

impl Id {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from(n: u128) -> Self {
        Self(Uuid::from_u128(n))
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    pub fn nil() -> Self {
        Self(Uuid::nil())
    }
}
