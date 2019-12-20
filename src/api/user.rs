use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Use as is sent from API
#[derive(Serialize, Deserialize)]
pub struct User {
    pub accessToken: String,
    pub createdAt: SystemTime,
    pub deletedAt: Option<SystemTime>,
    pub email: String,
    pub facebookAccessToken: Option<String>,
    pub facebookAccountId: Option<String>,
    pub id: u64,
    pub isSetup: bool,
    pub password: Option<String>,
    pub updatedAt: SystemTime,
    pub username: String,
}

/// Send this to authenticate to API
#[derive(Serialize, Deserialize)]
pub struct UserAuth<'a> {
    pub email: &'a str,
    pub password: Option<&'a str>,
}
