pub mod constants;

pub use constants::*;
pub use rand::seq::SliceRandom;

pub fn generate_word() -> String {
    WORDS.choose(&mut rand::thread_rng()).unwrap().to_string()
}

pub fn get_input() -> String {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    input.to_string().trim().to_lowercase()
}

fn get_first_char(input: &str) -> char {
    input.chars().nth(0).unwrap()
}

pub fn validate_input(input: &str) -> bool {
    input.len() != 0 && get_first_char(input).is_alphabetic()
}

pub fn run() {
    let mut lives = INITIAL_LIVES;
    let word = generate_word();
    let mut word_display = "_".repeat(word.len());
    let mut already_guessed = Vec::new();

    while lives > 0 {
        println!("You have {} lives", lives);
        println!("Word: {}", word_display);
        println!("Enter your guess: ");

        let input = get_input();

        if !validate_input(&input) {
            println!("Invalid input!");
            continue;
        }

        if input == word {
            println!("You win!");
            break;
        }

        if input.len() > 1 {
            lives -= 1;
            println!("{input} is not the word!");
            continue;
        }

        let guessed_letter = get_first_char(&input);

        if already_guessed.contains(&guessed_letter) {
            println!("You already guessed that letter!");
            continue;
        }

        already_guessed.push(guessed_letter);

        if word.contains(guessed_letter) {
            println!("Correct!");
            word_display = word
                .chars()
                .map(|letter| {
                    if already_guessed.contains(&letter) {
                        letter
                    } else {
                        '_'
                    }
                })
                .collect();
        } else {
            lives -= 1;
            println!("Incorrect!");
        }
    }

    if lives == 0 {
        println!("You lose! :(");
        println!("The word was: {}", word);
    }
}
