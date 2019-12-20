use aidungeon2_api::api::{AIDungeon, AIDungeonAuthError};

fn main() {
    let output: Result<AIDungeon, AIDungeonAuthError> =
        AIDungeon::register_new_user("existing.email@gmail.com", "username-23781297846138647617", "1234");
    if let Ok(dungeon) = output {
        println!("User registered.");
    } else {
        println!("{:#?}", output.err().unwrap());
    }
}
