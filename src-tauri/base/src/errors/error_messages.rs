pub const SUCCESS: &str = "Success";

// General
pub const ENV_VAR_NOT_SET: &str = "The environment variable attempted to access is not specified.";

// User errors
pub const USER_ALREADY_EXISTING_ERR: &str =
    "User is already existing. Try logging in with your account";
pub const USER_NOT_FOUND_ERR: &str = "Email or password is incorrect";

// Encoding
pub const BCRYPT_ENCODING_ERR: &str = "There was an error while an encoding process.";
pub const BCRYPT_DECODING_ERR: &str = "There was an error while an decoding process.";

// Database errors

pub const JWT_COOKIE_ERR: &str = "There was an error storing your session. Please try again.";
pub const NO_DB_CONNECTION_ERR: &str =
    "Unable to connect to the database. Please connect a database.";
pub const ERROR_QUEUE_NOT_INITIALIZED_ERR: &str = "No error queue running at the moment";
pub const QUERY_FAILED_ERR: &str = "The attempted database operation failed. Please try again.";
