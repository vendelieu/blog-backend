// Messages
pub const MESSAGE_OK: &str = "ok";
pub const MESSAGE_CAN_NOT_FETCH_DATA: &str = "Can not fetch data";
pub const MESSAGE_CAN_NOT_INSERT_DATA: &str = "Can not insert data";
pub const MESSAGE_CAN_NOT_UPDATE_DATA: &str = "Can not update data";
pub const MESSAGE_CAN_NOT_DELETE_DATA: &str = "Can not delete data";
pub const MESSAGE_SIGNUP_SUCCESS: &str = "Signup successfully";
pub const MESSAGE_LOGIN_SUCCESS: &str = "Login successfully";
pub const MESSAGE_USER_NOT_FOUND: &str = "User not found, please signup";
pub const MESSAGE_LOGOUT_SUCCESS: &str = "Logout successfully";
pub const MESSAGE_AUTH_PROCESS_ERROR: &str = "Error while processing auth data";

pub const MESSAGE_NOT_ALLOWED: &str = "Not allowed for this action";

// Bad request messages
pub const MESSAGE_INVALID_AUTH_DATA: &str = "Invalid auth data";

// Misc
pub const EMPTY: &str = "";

// Default number of items per page
pub const DEFAULT_PER_PAGE: i64 = 10;

// Default page number
pub const DEFAULT_PAGE_NUM: i64 = 1;

pub const EMPTY_STR: &str = "";

// Secret Key
pub static SECRET_KEY: [u8; 32] = *include_bytes!("secret.key");
