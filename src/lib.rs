pub mod api {
    use http;
    use reqwest::header;
    use serde::{Deserialize, Serialize};
    use serde_json::from_str;

    mod user;
    use user::*;
    mod story;
    use story::*;
    mod startOptions;
    use startOptions::*;

    const URI_USERINFO: &str = "https://api.aidungeon.io/users";
    const URI_REGISTERUSER: &str = "https://api.aidungeon.io/users/@me";
    const URI_NEW_SESSION: &str = "https://api.aidungeon.io/sessions";
    const URI_CURRENT_SESSION: &str = "https://api.aidungeon.io/sessions/[SESSIONID]/inputs";
    const URI_START_OPTIONS: &str = "https://api.aidungeon.io/sessions/*/config";
    const USERAGENT: &str = "soptikha2/aidungeon2-cli";

    /// This remembers runtime stuff
    /// such as auth tokens
    ///
    /// Use this to interact with AI Dungeons 2 API
    pub struct AIDungeon {
        /// Http client used to make requests.
        /// Already contains all necessary headers.
        http_client: reqwest::Client,
        story_id: Option<u64>,
    }

    #[derive(Debug)]
    pub enum AIDungeonError {
        EmailAlreadyExists,
        UsernameAlreadyExists,
        InvalidPassword,
        RequestFailed(reqwest::Error),
        InvalidResponseFromServer(serde_json::error::Error),
        UnexpectedError(String),
    }
    impl From<reqwest::Error> for AIDungeonError {
        fn from(err: reqwest::Error) -> Self {
            AIDungeonError::RequestFailed(err)
        }
    }
    impl From<http::header::InvalidHeaderValue> for AIDungeonError {
        fn from(err: http::header::InvalidHeaderValue) -> Self {
            AIDungeonError::UnexpectedError(format!(
                "Received invalid data when trying to register: {}",
                err
            ))
        }
    }
    impl From<serde_json::error::Error> for AIDungeonError {
        fn from(err: serde_json::error::Error) -> Self {
            AIDungeonError::InvalidResponseFromServer(err)
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
        /// If we received 400 Bad Request, the username is already taken.
        ///
        /// We expect HTTP 200/Ok and bunch of user info (such as id or hashed password).
        pub fn register_new_user(
            email: &str,
            username: &str,
            password: &str,
        ) -> Result<AIDungeon, AIDungeonError> {// Construct new client with access token in it
            let mut headers = header::HeaderMap::new();
            headers.append(
                header::USER_AGENT,
                header::HeaderValue::from_static(USERAGENT),
            );

            let client: reqwest::Client = reqwest::Client::builder()
                .gzip(true)
                .default_headers(headers)
                .build()?;

            // Send POST request with email field only
            let mut does_user_exist_response: reqwest::Response = client
                .post(URI_USERINFO)
                .json(&UserAuth {
                    email: Some(email),
                    username: None,
                    password: None,
                })
                .send()?;

            let mut user: User;
            match does_user_exist_response.status() {
                reqwest::StatusCode::NOT_ACCEPTABLE => {
                    // User already exists
                    return Err(AIDungeonError::EmailAlreadyExists);
                }
                reqwest::StatusCode::OK => {
                    user = does_user_exist_response.json()?;
                }
                _ => {
                    return Err(AIDungeonError::UnexpectedError(String::from(format!(
                        "Bad request status code while checking whether user account exists: {}",
                        does_user_exist_response.status()
                    ))));
                }
            }

            // Now we know user doesn't exist. So we can register it

            // Construct new client with access token in it
            let mut headers = header::HeaderMap::new();
            headers.append(
                header::USER_AGENT,
                header::HeaderValue::from_static(USERAGENT),
            );
            {
                let header_value_access_token = header::HeaderValue::from_str(&user.accessToken);
                if let Ok(access_token) = header_value_access_token {
                    headers.append("x-access-token", access_token);
                } else {
                    return Err(AIDungeonError::UnexpectedError(String::from(format!(
                        "Bad access token received from server while registering new user: {}",
                        header_value_access_token.unwrap_err()
                    ))));
                }
            }
            let client: reqwest::Client = reqwest::Client::builder()
                .gzip(true)
                .default_headers(headers)
                .build()?;

            // Send PATCH request with specified access token and credentials
            let mut user_register_reponse = client
                .patch(URI_REGISTERUSER)
                .json(&UserAuth {
                    username: Some(username),
                    password: Some(password),
                    email: None
                })
                .send()?;

            match user_register_reponse.status() {
                reqwest::StatusCode::OK => {
                    user = user_register_reponse.json()?;
                }
                reqwest::StatusCode::BAD_REQUEST => {
                    return Err(AIDungeonError::UsernameAlreadyExists);
                }
                _ => {
                    return Err(AIDungeonError::UnexpectedError(String::from(format!(
                        "Bad request status code while trying to register user: {}",
                        user_register_reponse.status()
                    ))));
                }
            }

            // Return prepared client with correct access token
            Ok(AIDungeon {
                http_client: client,
                story_id: None
            })
        }

        /// Login with existing user account
        /// 
        /// Send POST request to https://api.aidungeon/users
        /// This will contain JSON with email and password.
        /// 
        /// We expect to receive access token together with other various user info (and status code 200/OK).
        pub fn login(
            email: &str,
            password: &str
        ) -> Result<AIDungeon, AIDungeonError> {
            let mut headers = header::HeaderMap::new();
            headers.append(
                header::USER_AGENT,
                header::HeaderValue::from_static(USERAGENT),
            );

            let client: reqwest::Client = reqwest::Client::builder()
                .gzip(true)
                .default_headers(headers)
                .build()?;

            // Send POST request with email field only
            let mut does_user_exist_response: reqwest::Response = client
                .post(URI_USERINFO)
                .json(&UserAuth {
                    email: Some(email),
                    password: Some(password),
                    username: None,
                })
                .send()?;

            let mut user: User;
            match does_user_exist_response.status() {
                reqwest::StatusCode::OK => {
                    user = does_user_exist_response.json()?;
                }
                _ => {
                    return Err(AIDungeonError::UnexpectedError(String::from(format!(
                        "Bad request status code while trying to log in: {}",
                        does_user_exist_response.status()
                    ))));
                }
            }

            // Construct new client with access token in it
            let mut headers = header::HeaderMap::new();
            headers.append(
                header::USER_AGENT,
                header::HeaderValue::from_static(USERAGENT),
            );
            {
                let header_value_access_token = header::HeaderValue::from_str(&user.accessToken);
                if let Ok(access_token) = header_value_access_token {
                    headers.append("x-access-token", access_token);
                } else {
                    return Err(AIDungeonError::UnexpectedError(String::from(format!(
                        "Bad access token received from server while registering new user: {}",
                        header_value_access_token.unwrap_err()
                    ))));
                }
            }
            let client: reqwest::Client = reqwest::Client::builder()
                .gzip(true)
                .default_headers(headers)
                .build()?;

            Ok(AIDungeon {
                http_client: client,
                story_id: None,
            })
        }

        pub fn start_custom_story(&mut self, custom_prompt: &str) -> Result<(), AIDungeonError> {
            let mut user_input_reply: reqwest::Response = self.http_client
                .post(URI_NEW_SESSION)
                .json(&StartOptions{
                    characterType: None,
                    customPrompt: custom_prompt,
                    name: None,
                    storyMode: "custom"
                })
                .send()?;

            let mut response: Story;
            match user_input_reply.status() {
                reqwest::StatusCode::OK => {
                    response = user_input_reply.json()?;
                }
                _ => {
                    return Err(AIDungeonError::UnexpectedError(format!("Unexpected status code while sending reply: {}", user_input_reply.status())));
                }
            }

            self.story_id = Some(response.id);

            Ok(())
        }

        /// Send text prompt to currently running story.
        /// This returns full story text
        pub fn send_reply<'a>(&self, text: &str) -> Result<Vec<StoryText>, AIDungeonError> {
            if self.story_id.is_none() {
                return Err(AIDungeonError::UnexpectedError(String::from("There is no running story, but tried to send reply.")));
            }

            let mut user_input_reply: reqwest::Response = self.http_client
                .post(&URI_CURRENT_SESSION.replace("[SESSIONID]", &self.story_id.unwrap().to_string()))
                .json(&StoryTextInput{
                    text
                })
                .send()?;

            let mut response: ListOfStoryTexts;
            match user_input_reply.status() {
                reqwest::StatusCode::OK => {
                    response = user_input_reply.json()?;
                }
                _ => {
                    return Err(AIDungeonError::UnexpectedError(format!("Unexpected status code while sending reply: {}", user_input_reply.status())));
                }
            }

            Ok(response.texts)
        }
    }
}
