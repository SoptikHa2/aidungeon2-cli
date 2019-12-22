use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Story {
    /// If the story ended, here will be the end. Known values: "lose" (probably also "win")
    pub conclusion: Option<String>,
    pub createdAt: String,
    pub deletedAt: Option<String>,
    pub id: u64,
    // TODO: Unknown field "rating"
    /// List of input/outputs of this story
    pub story: Vec<StoryText>,
    pub storyPublicId: Option<String>,
    pub updatedAt: String,
    pub userId: u64,
}

#[derive(Deserialize)]
pub struct ListOfStoryTexts {
    pub texts: Vec<StoryText>,
}

#[derive(Deserialize, Debug)]
pub struct StoryText {
    /// Type of value. Known values: "input", "output"
    #[serde(rename = "type")]
    pub text_type: String,
    /// Text itself
    pub value: String,
    /// If the story ended, here will be the end. Known values: "lose" (probably also "win")
    pub conclusion: Option<String>,
}

/// User text input sent to API
#[derive(Serialize)]
pub struct StoryTextInput<'a> {
    pub text: &'a str,
}
