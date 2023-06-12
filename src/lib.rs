use stilo::{style, stylize};

pub type GridLine = String;
pub type Grid = Vec<GridLine>;

/// Game state
pub enum State {
    Play,
    Loss,
    Win,
}

/// Get random item of vector
pub fn random_item<'a, T>(vec: &'a Vec<T>) -> &'a T {
    use rand::seq::SliceRandom;

    vec.choose(&mut rand::thread_rng())
        .expect("Could not get random value of vector")
}

/// Generate lines to print
pub fn get_lines(grid: &mut Grid, answer: &str) -> Vec<String> {
    let mut print: Vec<String> = vec![];

    //? Move inside loop ? Might not affect performance - Also change `get_greens`
    let answer_vec = answer.chars().collect::<Vec<char>>();

    for line in grid {
        let mut curr = stylize!("│": Cyan);

        for (i, ch) in line.chars().enumerate() {
            let style = if answer_vec.get(i).unwrap_or(&' ') == &ch {
                style!(Green)
            } else if answer_vec.contains(&ch) {
                style!(Yellow)
            } else {
                style!()
            };

            curr += &style.format(&ch.to_string());
        }

        curr += &stylize!("│": Cyan);
        print.push(curr);
    }

    print
}

/// Get user input as String
pub fn input(prompt: &str) -> Result<String, std::io::Error> {
    use std::io::{stdin, stdout, Write};

    let mut s = String::new();
    print!("{}", prompt);

    let _ = stdout().flush();
    stdin().read_line(&mut s)?;

    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    Ok(s)
}

/// Remove first character of string
pub fn remove_first(s: &str) -> &str {
    let mut chars = s.chars();
    chars.next();
    chars.as_str()
}

/// Algorithm to get valid guesses based on grid state
/// Returns list of all valid words, from list of total answers
pub fn smart_guess<'a>(grid: &Grid, answer: &str, answers: &Vec<&'a str>) -> Vec<&'a str> {
    let mut valids = Vec::<&str>::new();

    'Guess: for &guess in answers {
        for row in grid {
            // Guess must not be in grid already
            if guess == row {
                continue 'Guess;
            }

            // Loop characters
            for (i, row_ch) in row.chars().enumerate() {
                let answer_ch = answer
                    .chars()
                    .nth(i)
                    .expect("Row and answer should be same length");
                let guess_ch = guess
                    .chars()
                    .nth(i)
                    .expect("Row and guess should be same length");

                // Green
                if answer_ch == row_ch {
                    // Green must be same character
                    if guess_ch != row_ch {
                        continue 'Guess;
                    }
                }
                // Yellow
                else if answer.contains(row_ch) {
                    // Yellow must not be same character
                    if guess_ch == row_ch {
                        continue 'Guess;
                    }
                    // Yellow must be in word
                    if !guess.contains(row_ch) {
                        continue 'Guess;
                    }
                }
                // White
                else {
                    // White must not be in word
                    if guess.contains(row_ch) {
                        continue 'Guess;
                    }
                }
            }
        }

        // Add to valid guesses
        valids.push(guess);
    }

    valids
}
