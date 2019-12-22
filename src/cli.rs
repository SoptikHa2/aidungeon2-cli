use aidungeon2_api::api::{start_options::*, story::*, AIDungeon, AIDungeonError};
use ansi_term::*;
use std::cmp::PartialOrd;
use std::io::Stdin;
use std::str::FromStr;

fn main() {
    println!("AI Dungeon 2");
    println!("--------------------------------");
    println!("CLI version by Petr Šťastný soptikha2/aidungeon2-cli");
    println!("AI Dungeon itself: AIDungeon/AIDungeon");
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
        let story: Vec<StoryText> = start_story_with_choice(&mut stdin, &mut game);
        println!("Starting game. Type /restart to start again.");
        println!("{}", Colour::Green.paint(&story.last().unwrap().value));
        loop {
            let prompt = read_line(&mut stdin);
            if prompt == "/restart" {
                break;
            }
            let story = game.send_reply(&prompt);
            if let Ok(story) = story {
                println!("{}", Colour::Green.paint(&story.last().unwrap().value));
            } else {
                println!("{}", Colour::Red.paint(format!("{:?}", story)));
            }
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
        }
        'R' | 'r' => {
            return register(stdin);
        }
        _ => {
            // Start again
            return get_access(stdin);
        }
    }
}

fn start_story_with_choice(stdin: &mut Stdin, game: &mut AIDungeon) -> Vec<StoryText> {
    let recommended_stories: Result<StartModesContainer, AIDungeonError> =
        game.get_recommended_story();

    if let Ok(stories) = recommended_stories {
        let mut game_modes = stories.modes.keys();
        for i in 0..game_modes.len() {
            println!("[{}] {}", i, game_modes.next().unwrap());
        }

        let input: usize = read_number(stdin, Some(0), Some(stories.modes.len() - 1));

        let story_mode = &stories.modes[stories.modes.keys().nth(input).unwrap()];

        // Custom prompt
        if story_mode.userDefined.unwrap_or(false) {
            println!("{}", story_mode.instructions.as_ref().unwrap());
            let result = game.start_story(
                Some(&read_line(stdin)),
                stories.modes.keys().nth(input).unwrap(),
                None,
                None,
            );
            if let Err(error) = result {
                println!("{}", Colour::Red.paint(format!("{:?}", error)));
                return start_story_with_choice(stdin, game);
            } else {
                return result.unwrap();
            }
        } else {
            println!("Select your starting character");
            let mut character_names = story_mode.characters.as_ref().unwrap().keys();
            for i in 0..character_names.len() {
                println!("[{}] {}", i, character_names.next().unwrap());
            }

            let character_input: usize = read_number(
                stdin,
                Some(0),
                Some(story_mode.characters.as_ref().unwrap().len() - 1),
            );

            println!("What is your name?");
            let name = read_line(stdin);

            let result = game.start_story(
                None,
                stories.modes.keys().nth(input).unwrap(),
                Some(&name),
                Some(
                    story_mode
                        .characters
                        .as_ref()
                        .unwrap()
                        .keys()
                        .nth(character_input)
                        .unwrap(),
                ),
            );
            if let Err(error) = result {
                println!("{}", Colour::Red.paint(format!("{:?}", error)));
                return start_story_with_choice(stdin, game);
            } else {
                return result.unwrap();
            }
        }
    } else {
        println!(
            "{}",
            Colour::Red.paint(format!("{:?}", recommended_stories.err().unwrap()))
        );
        return start_story_with_choice(stdin, game);
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
    } else {
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
    } else {
        println!("Could not register: {:?}", dungeon_result.err().unwrap());
        return None;
    }
}

fn read_line(stdin: &mut Stdin) -> String {
    let mut buffer: String = String::new();
    stdin.read_line(&mut buffer).unwrap();
    String::from(buffer.trim())
}

fn read_number<T>(stdin: &mut Stdin, minimum: Option<T>, maximum: Option<T>) -> T
where
    T: PartialOrd + FromStr + Copy,
{
    loop {
        let line = read_line(stdin);
        let number = line.parse::<T>();
        if let Ok(number) = number {
            if (minimum.is_none() || minimum.unwrap() <= number)
                && (maximum.is_none() || maximum.unwrap() >= number)
            {
                return number;
            }
        }
    }
}
