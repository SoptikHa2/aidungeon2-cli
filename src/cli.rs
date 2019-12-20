use aidungeon2_api::api::{AIDungeon, AIDungeonError};
use std::io::{Stdin};

fn main() {
    println!("AI Dungeon 2");
    println!("--------------------------------");
    println!("CLI version by Petr Šťastný (soptikha2/aidungeon2-cli");
    println!("Original AI dungeon: AIDungeon/AIDungeon");
    println!("--------------------------------");

    let mut stdin = std::io::stdin();

    let mut game: AIDungeon;

    loop {
        if let Some(game_access) = get_access(&mut stdin) {
            game = game_access;
            break;
        }
    }

    println!("Logged in!");
    loop {
        println!("Enter custom starting prompt. Prepared starting prompts TODO.");
        let prompt = read_line(&mut stdin);
        println!("Starting game. Type /restart to start again.");
        let mut story = game.start_custom_story(&prompt);
        loop {
            if let Ok(story) = story {
                println!("{}", story.last().unwrap().value);
            }else{
                println!("{:?}", story);
            }
            let prompt = read_line(&mut stdin);
            if prompt == "/restart" {
                break;
            }
            story = game.send_reply(&prompt);
        }
    }
}

/// Ask user for email.
/// If it exists, ask user for password and log him in via login()
/// If not, register user.
fn get_access(stdin: &mut Stdin) -> Option<AIDungeon> {
    // TODO: Test for saved credentials
    
    // Ask user for email
    println!("You need to have an account to play this game.");
    println!("[L]ogin\n[R]egister");
    let choice = read_line(stdin);
    match choice.trim_start().chars().nth(0).unwrap_or('x') {
        'L' | 'l' => {
            return login(stdin);
        },
        'R' | 'r' => {
            return register(stdin);
        }
        _ => {
            // Start again
            return get_access(stdin);
        }
    }
}

fn login(stdin: &mut Stdin) -> Option<AIDungeon> {
    println!("Email:");
    let email = read_line(stdin);
    println!("Password:");
    let password = read_line(stdin);
    let dungeon_result = AIDungeon::login(&email, &password);
    if let Ok(dungeon) = dungeon_result {
        return Some(dungeon);
    }else{
        println!("Failed to login: {:?}", dungeon_result.err().unwrap());
        return None;
    }
}

fn register(stdin: &mut Stdin) -> Option<AIDungeon> {
    println!("Email:");
    let email = read_line(stdin);
    println!("Username:");
    let username = read_line(stdin);
    println!("Password:");
    let password = read_line(stdin);
    let dungeon_result = AIDungeon::register(&email, &username, &password);
    if let Ok(result) = dungeon_result {
        return Some(result);
    }else{
        println!("Could not register: {:?}", dungeon_result.err().unwrap());
        return None;
    }
}

fn read_line(stdin: &mut Stdin) -> String {
    let mut buffer: String = String::new();
    stdin.read_line(&mut buffer).unwrap();
    String::from(buffer.trim())
}
