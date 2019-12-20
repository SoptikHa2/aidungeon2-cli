use serde::{Deserialize, Serialize};

/// Use as is sent from API
#[derive(Serialize, Deserialize)]
pub struct User {
    pub accessToken: String,
    pub createdAt: String,
    pub deletedAt: Option<String>,
    pub email: String,
    pub facebookAccessToken: Option<String>,
    pub facebookAccountId: Option<String>,
    pub id: u64,
    pub isSetup: bool,
    pub password: Option<String>,
    pub updatedAt: String,
    pub username: Option<String>,
}

/// Send this to authenticate to API
#[derive(Serialize, Deserialize)]
pub struct UserAuth<'a> {
    pub email: Option<&'a str>,
    pub password: Option<&'a str>,
    pub username: Option<&'a str>,
}
