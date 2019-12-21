use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct StartModesContainer {
    pub modes: HashMap<String, StartMode>,
}

#[derive(Deserialize)]
pub struct StartMode {
    pub settings: Vec<String>,
    pub characters: HashMap<String, StartCharacter>,
}

#[derive(Deserialize)]
pub struct StartCharacter {
    pub prompts: Vec<String>,

}

#[derive(Serialize)]
/// These are start options passed to API.
/// In case of "custom" storyMode, everything
/// but "customPrompt" should be null.
pub struct StartOptions<'a> {
    pub characterType: Option<&'a str>,
    pub customPrompt: &'a str,
    pub name: Option<&'a str>,
    pub storyMode: &'a str,
}