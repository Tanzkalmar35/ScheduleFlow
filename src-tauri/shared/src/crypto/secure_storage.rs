use keyring::Entry;

/// A service for storing and retrieving system keys.
pub struct SecureStorage;

/// Implementation of the SecureStorage.
impl SecureStorage {
    /// Store the system key for the given user email.
    ///
    /// # Arguments
    ///
    /// * `key` - The system key to store.
    /// * `user_email` - The email of the user to store the system key for.
    ///
    /// # Fails
    ///
    /// * If the user_email can not form a valid keyring entry.
    /// * If there is more than one keyring entry for the given user_email.
    pub fn store_system_key(
        key: &String,
        entry_id: &String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let service = "ScheduleFlow";
        let entry = Entry::new(service, entry_id)?;
        let res = entry.set_password(key);
        if let Err(e) = res {
            return Err(e.into());
        }
        Ok(())
    }

    /// Get the system key for the given user email.
    ///
    /// # Arguments
    ///
    /// * `user_email` - The email of the user to get the system key for.
    ///
    /// # Returns
    ///
    /// The system key for the given user email.
    ///
    /// # Fails
    ///
    /// * If the user_email can not form a valid keyring entry.
    /// * If the entry contains no password.
    pub fn get_system_key(entry_id: &String) -> Result<String, Box<dyn std::error::Error>> {
        let service = "ScheduleFlow";
        let entry = Entry::new(service, entry_id)?;
        let res = entry.get_password();
        if let Err(e) = res {
            return Err(e.into());
        }
        Ok(res.unwrap())
    }

    /// Remove the system key for the given user email.
    ///
    /// # Arguments
    ///
    /// * `user_email` - The email of the user to remove the system key for.
    ///
    /// # Fails
    ///
    /// * If the user_email can not form a valid keyring entry.
    /// * If the entry contains no password.
    pub fn remove_system_key(user_email: &String) -> Result<(), Box<dyn std::error::Error>> {
        let service = "ScheduleFlow";
        let entry = Entry::new(service, user_email)?;
        let res = entry.delete_credential();
        if let Err(e) = res {
            return Err(e.into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_system_key() {
        let key = "test_key".to_string();
        let user_email = "adminmail".to_string();

        let res = SecureStorage::store_system_key(&key, &user_email);
        if let Err(e) = res {
            panic!("Failed to store system key: {}", e);
        }

        let stored_key = SecureStorage::get_system_key(&user_email);
        if let Err(e) = stored_key {
            panic!("Failed to get system key: {}", e);
        }

        assert_eq!(key, stored_key.unwrap());

        if let Err(e) = SecureStorage::remove_system_key(&user_email) {
            panic!("Failed to remove system key: {}", e);
        }
    }
}

