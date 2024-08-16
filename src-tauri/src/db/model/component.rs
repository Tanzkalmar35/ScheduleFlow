use crate::db::model::model::Model;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum ComponentType {
    EVENT,
    TODO,
    VENUE,
    OTHER,
}

impl ComponentType {
    pub fn parse(c_type: &str) -> Self {
        match c_type.to_lowercase().as_str() {
            "event" => Self::EVENT,
            "todo" => Self::TODO,
            "venue" => Self::VENUE,
            _ => Self::OTHER,
        }
    }
}

#[derive(Clone)]
pub struct Component {
    pub(crate) uuid: Uuid,
    pub(crate) c_type: ComponentType,
}

impl Component {
    pub fn new(c_type: ComponentType) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            c_type,
        }
    }

    pub fn from(uuid: Uuid, c_type: ComponentType) -> Self {
        Self { uuid, c_type }
    }
}


impl Model for Component {}
