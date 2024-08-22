use uuid::Uuid;

pub struct ComponentPropertyCombination {
    pub(crate) component_uuid: Uuid,
    pub(crate) property_uuid: Uuid,
}

impl ComponentPropertyCombination {
    pub fn new(component_uuid: Uuid, property_uuid: Uuid) -> Self {
        Self {
            component_uuid,
            property_uuid,
        }
    }
}
