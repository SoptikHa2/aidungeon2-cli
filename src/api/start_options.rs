use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct StartModesContainer {
    pub modes: HashMap<String, StartMode>,
}

#[derive(Deserialize, Debug)]
/// Each mode either has settings and characters,
/// or has userDefined (set to true) and instructions.
pub struct StartMode {
    pub settings: Option<Vec<String>>,
    pub characters: Option<HashMap<String, StartCharacter>>,
    pub instructions: Option<String>,
    pub userDefined: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct StartCharacter {
    pub prompts: Vec<String>,
    pub items1: Vec<String>,
    pub items2: Vec<String>,
}

#[derive(Serialize)]
/// These are start options passed to API.
/// In case of "custom" storyMode, everything
/// but "customPrompt" should be null.
pub struct StartOptions<'a> {
    pub characterType: Option<&'a str>,
    pub customPrompt: Option<&'a str>,
    pub name: Option<&'a str>,
    pub storyMode: &'a str,
}
