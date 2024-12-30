use uuid::Uuid;

pub struct UserCalendarCombination {
    pub user_uuid: Uuid,
    pub calendar_uuid: Uuid,
}

impl UserCalendarCombination {
    pub fn new(user_uuid: Uuid, calendar_uuid: Uuid) -> Self {
        Self {
            user_uuid,
            calendar_uuid,
        }
    }
}
