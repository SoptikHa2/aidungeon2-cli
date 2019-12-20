use aidungeon2_api::api::{AIDungeon, AIDungeonAuthError};

fn main() {
    let output: Result<AIDungeon, AIDungeonAuthError> =
        AIDungeon::register_new_user("email+that+doesnt+exist@example.com", "", "");
    println!("{:#?}", output.err().unwrap());
}
