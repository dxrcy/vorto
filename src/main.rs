use stilo::{print_styles, stylize, stylize_many};
use vorto::{Grid, State::*};

//TODO Add timing ¿ how ?
fn main() -> ! {
    // Handle CTRLC exit
    ctrlc::set_handler(|| {
        println!("\nĜis!");
        std::process::exit(0);
    })
    .expect("Could not set CTRLC handler");

    // Include text files in binary
    let answers = include_str!("answers.txt").lines().collect::<Vec<&str>>();
    let accepted = include_str!("accepted.txt").lines().collect::<Vec<&str>>();
    let mut warning = String::from("");

    // Game
    loop {
        let answer = vorto::random_item(&answers);
        let mut grid: Grid = Vec::new();
        let mut state = Play;

        // Guess
        loop {
            // Clear terminal
            print!("\x1bc");
            // Title
            print_styles!("┌VORTO┐": Cyan + italic);

            // Warning
            if warning.len() > 0 {
                print_styles!(" {warning}": Yellow);
            }
            if grid.len() > 0 {
                println!();
            }

            // Generate lines
            let print = vorto::get_lines(&mut grid, &answer);
            println!("{}", print.join("\n"));

            // Win or loss display
            if let Win | Loss = state {
                vorto::input(&if let Win = state {
                    stylize_many!(
                        "└": Cyan;
                        "VENKO": Green + italic;
                        "┘": Cyan;
                    )
                } else {
                    stylize_many!(
                        "└": Cyan;
                        "PERDO": Red + italic;
                        "┘": Cyan;
                    )
                })
                .expect("Could not continue from input");

                warning = "".to_string();
                break; // Break guess loop, start new game
            }

            // Win check
            if Some(&answer.to_string()) == grid.last() {
                state = Win;
                warning = stylize!(
                    "{}": Green,
                    [
                        "GENIULO!",
                        "Lerta!",
                        "Bonega!",
                        "Bona",
                        "Boneta",
                        "Ne malbona...",
                    ]
                    .get(grid.len() - 1)
                    .unwrap_or(&"Kio?")
                );
                continue; // Skip rest of guess loop
            }

            // Loss check
            if grid.len() >= 6 {
                state = Loss;
                warning = stylize_many!(
                    "Estis: ";
                    "'{}'": +italic, answer;
                );
                continue; // Skip rest of guess loop
            }

            // Make guess
            let guess = vorto::input(&stylize!("└": Cyan))
                .expect("Could not read standard input")
                .to_lowercase();

            // Command starts with '/'
            if guess.starts_with('/') {
                match vorto::remove_first(&guess) {
                    // Commands help
                    "helpu" => {
                        warning = stylize_many!(
                            "Eblaj ordonoj: ";
                            "eliru":  Yellow+bold; ", ": Yellow;
                            "trompu": Yellow+bold; ", ": Yellow;
                            "divenu": Yellow+bold; ", ": Yellow;
                            "pensu":  Yellow+bold; ", ": Yellow;
                            "riparu": Yellow+bold;
                        );
                    }

                    // New game (restart)
                    "eliru" => {
                        warning = stylize_many!(
                            "Estis: ";
                            "'{}'": +italic, answer;
                        );
                        break; // Break guess loop, start new game
                    }

                    // Give answer (cheat)
                    "trompu" => {
                        warning = stylize_many!(
                            "Estas: ";
                            "'{}'": +italic, answer;
                        );
                    }

                    // Random word (guess)
                    "divenu" => {
                        grid.push(vorto::random_item(&answers).to_string());
                        warning = "Hazarda tre".to_string();
                    }

                    // Reasonable guess (think)
                    "pensu" => {
                        if grid.len() < 1 {
                            grid.push("salto".to_string());
                            warning = "Boneta diveno".to_string();
                        } else {
                            let valids = vorto::smart_guess(&grid, answer, &answers);
                            if valids.len() > 0 {
                                grid.push(vorto::random_item(&valids).to_string());
                                warning = format!("Eblaj vortoj: {}", valids.len());
                            } else {
                                warning = "Ne povas trovi la solvon!".to_string();
                            }
                        }
                    }

                    // Show valid guesses
                    "sciigu" => {
                        if grid.len() < 1 {
                            warning = "Ne eblas scii!".to_string();
                        } else {
                            let valids = vorto::smart_guess(&grid, answer, &answers);
                            if valids.len() > 0 {
                                warning = "Eblaj vortoj: ".to_string();
                                for (i, valid) in valids.into_iter().enumerate() {
                                    if i > 0 {
                                        warning += &stylize!(", ":  Yellow);
                                    }
                                    warning += &stylize!("{}":  Yellow+bold, valid);
                                    if i >= 5 {
                                        warning += &stylize!("...":  Yellow);
                                        break;
                                    }
                                }
                            } else {
                                warning = "Ne povas trovi la solvon!".to_string();
                            }
                        }
                    }

                    // Remove last guess (fix)
                    "riparu" => {
                        if grid.len() > 0 {
                            grid.pop().expect("Could not remove last item of grid");
                            warning = "Uŭps!".to_string();
                        } else {
                            warning = "Ne povas ripari".to_string();
                        }
                    }

                    // Unknown command
                    _ => {
                        warning = "Nekonata ordono".to_string();
                    }
                }

                continue; // Skip rest of guess loop
            }

            // Guess must be 5 letters
            if guess.chars().count() != 5 {
                warning = "Devas esti 5 literoj".to_string();
                continue; // Skip rest of guess loop
            }
            // Must be in either word list
            if !(answers.contains(&guess.as_str()) || accepted.contains(&guess.as_str())) {
                warning = "Ne estas vorto".to_string();
                continue; // Skip rest of guess loop
            }

            // Guess accepted
            grid.push(guess);
            warning = "".to_string();
        }
    }
}
