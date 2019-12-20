pub mod api {
    use reqwest::header;
    use serde::{Deserialize, Serialize};
    use serde_json::from_str;

    mod user;
    use user::*;

    const URI_USERINFO: &str = "https://api.aidungeon.io/users";
    const USERAGENT: &str = "soptikha2/aidungeon2-cli";

    /// This remembers runtime stuff
    /// such as auth tokens
    ///
    /// Use this to interact with AI Dungeons 2 API
    pub struct AIDungeon {
        /// Access token required to access the API
        ///
        /// This is obtained after login or registration
        access_token: String,
    }

    pub enum AIDungeonAuthError {
        UserAlreadyExists,
        InvalidPassword,
        RequestFailed(reqwest::Error),
        UnexpectedError(String),
    }
    impl From<reqwest::Error> for AIDungeonAuthError {
        fn from(err: reqwest::Error) -> Self {
            AIDungeonAuthError::RequestFailed(err)
        }
    }

    impl AIDungeon {
        /// Register new user
        ///
        /// First of all, a POST request will be sent to https://api.aidungeon.io/users
        /// This will contain JSON with email field.
        ///
        /// If this user does already exist, we will receive JSON field with "Incorrect password." and HTTP status 406/Not Acceptable.
        /// and halt registration.
        ///
        /// If user doesn't exist, we will receive JSON with user info, particulary access token and HTTP status code 200/Ok.
        ///
        /// From now on, we will use header `x-access-token` with access token provided by the API.
        /// We send PATCH request to https://api.aidungeon.io/users/@me with JSON contining two fields,
        /// username and password.
        ///
        /// We expect HTTP 200/Ok and bunch of user info (such as id or hashed password).
        pub fn register_new_user(
            email: &str,
            username: &str,
            password: &str,
        ) -> Result<AIDungeon, AIDungeonAuthError> {
            let mut headers = header::HeaderMap::new();
            headers.append(
                header::USER_AGENT,
                header::HeaderValue::from_static(USERAGENT),
            );

            let client = reqwest::Client::builder()
                .gzip(true)
                .default_headers(headers)
                .build()?;

            // Send POST request with email field only
            let mut does_user_exist_response: reqwest::Response = client
                .post(URI_USERINFO)
                .json(&UserAuth {
                    email: email,
                    password: None,
                })
                .send()?;

            let mut user: User;
            match does_user_exist_response.status() {
                reqwest::StatusCode::NOT_ACCEPTABLE => {
                    // User already exists
                    return Err(AIDungeonAuthError::UserAlreadyExists);
                }
                reqwest::StatusCode::OK => {
                    user = does_user_exist_response.json()?;
                }
                _ => {
                    return Err(AIDungeonAuthError::UnexpectedError(String::from(format!(
                        "Bad request status code while checking whether user account exists: {}",
                        does_user_exist_response.status()
                    ))));
                }
            }

            unimplemented!();
        }
    }
}
