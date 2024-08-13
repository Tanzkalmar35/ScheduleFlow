use uuid::Uuid;

#[derive(Debug)]
pub struct Calendar {
    pub(crate) uuid: Uuid,
}

impl Calendar {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
        }
    }

    pub fn from(uuid: Uuid) -> Self {
        Self { uuid }
    }
}
