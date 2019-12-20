use std::time::SytemTime;

/// Use as is sent from API
struct User {
    accessToken: String,
    createdAt: SystemTime,
    deletedAt: Option<SystemTime>,
    email: String,
    facebookAccessToken: Option<String>,
    facebookAccountId: Option<String>,
    id: u64,
    isSetup: bool,
    password: Option<String>,
    updatedAt: SystemTime,
    username: String
}