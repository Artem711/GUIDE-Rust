use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    id: Uuid,
    name: String
}

impl User {
    pub fn new(id: Uuid, name: &str) -> Self {
        Self {
            id,
            name: String::from(name),
        }
    }
}