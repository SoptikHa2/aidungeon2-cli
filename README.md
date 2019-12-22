# AI Dungeon 2 CLI

This is CLI application (and library) for AI Dungeon 2 mobile API. Everything runs on AI Dungeon servers, so no special hardware is needed.

Original game: [AIDungeon/AIDungeon](https://github.com/AIDungeon/AIDungeon/)

## Install

Install rust and cargo.

When the package is available in central crate repository, you can just do `cargo install aidungeon2-cli`, but it's not possible for now.

Clone the repository and use `cargo build --release` to build release binary (or `cargo run` for quick debug). Binary will be in `target/release/aidungeon2-cli`.

## Usage

Once you run binary, follow prompts. You'll be asked to login or register (same credentials as mobile app), but you'll be able to play afterwards as normal.

To use this as library, look at `src/cli.rs` for sample implementation. In short:

```rust
use aidungeon2_api::api::AIDungeon; //{start_options::*, story::*, AIDungeon, AIDungeonError}

fn main() {
    // ...
    let mut game_with_associated_account = AIDungeon::login(&email, &password).unwrap();
    // or
    let mut game_with_associated_account = AIDungeon::register(&email, &username, &password).unwrap();

    // Start game, either with pre-made prompt (see AIDungeon::get_recommended_story()) or with custom prompt:
    let start_prompt = game_with_associated_account.start_story(Some("My super awesome custom prompt. I'm king Arthur and I'm looking for the Holy Grail."), "custom", None, None).unwrap();
    // or
    let start_prompt = game_with_associated_account.start_story(None, "apocalyptic", Some("my-character-name"), "soldier").unwrap();

    // And now just send prompts
    let story = game_with_associated_account.send_reply("Stab sir Lancelot.").unwrap(); // Returns full story (all inputs and outputs)
}
```