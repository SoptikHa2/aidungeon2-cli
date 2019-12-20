extern crate http;

use http::{Request, Response};


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
        pub fn register_new_user(email: &str, username: &str, password: &str) -> Result<AIDungeon, AIDungeonAuthError> {
            // Send a post request with email field
            
            
            unimplemented!();
        }
    }