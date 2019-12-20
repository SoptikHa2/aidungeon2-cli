use aidungeon2_api::api::{AIDungeon, AIDungeonAuthError};

fn main() {
    // let output: Result<AIDungeon, AIDungeonAuthError> =
        // AIDungeon::register_new_user("existing.email.2@gmail.com", "username-2378129784613864761", "1234");
    let output: Result<AIDungeon, AIDungeonAuthError> = 
        AIDungeon::login("existing.email.2@gmail.com", "1234");
    if let Ok(dungeon) = output {
        println!("User logged in.");
    } else {
        println!("{:#?}", output.err().unwrap());
    }
}
