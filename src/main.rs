use vorto::{Grid, State::*};

//TODO Add timing ¿ how ?
//TODO Move string literal warnings to const's
fn main() -> ! {
  // Handle CTRLC exit
  ctrlc::set_handler(|| {
    println!("\nĜis!");
    std::process::exit(0);
  }).expect("Could not set CTRLC handler");

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
      // Title
      print!("\x1bc\x1b[36;3m┌VORTO┐\x1b[0m");

      // Warning
      if warning.len() > 0 {
        print!(" \x1b[33m{warning}");
      }
      if grid.len() > 0 {
        println!();
      }

      // Generate lines
      let print = vorto::get_lines(&mut grid, &answer);
      println!("{}", print.join("\n"));

      // Win or loss display
      if let Win | Loss = state {
        vorto::input(if let Win = state {
          "\x1b[36m└\x1b[32;1mVENKO\x1b[0m\x1b[36m┘\x1b[0m"
        } else {
          "\x1b[36m└\x1b[31;1mPERDO\x1b[0m\x1b[36m┘\x1b[0m"
        })
        .expect("Could not continue from input");

        warning = "".to_string();
        break; // Break guess loop, start new game
      }

      // Win check
      if Some(&answer.to_string()) == grid.last() {
        state = Win;
        warning = format!(
          "\x1b[32m{}",
          [
            "GENIULO!",
            "Lerta!",
            "Bonega!",
            "Bona",
            "Boneta",
            "Ne malbona...",
          ]
          .last()
          .unwrap_or(&"Kio?")
        );
        continue; // Skip rest of guess loop
      }

      // Loss check
      if grid.len() >= 6 {
        state = Loss;
        warning = format!("Estis: \x1b[3m'{}'", answer);
        continue; // Skip rest of guess loop
      }

      // Make guess
      let guess = vorto::input("\x1b[36m└\x1b[0m").expect("Could not read standard input");

      // Command starts with '/'
      if guess.starts_with('/') {
        match guess.chars().nth(1).unwrap_or(' ') {
          // New game '/x'
          'x' => {
            warning = format!("Estis: \x1b[3m'{}'", answer);
            break; // Break guess loop, start new game
          }
          // Give answer '/!'
          '!' => {
            warning = format!("\x1b[31mEstas: \x1b[3m'{}'", answer);
          }
          // Random word '/?'
          '?' => {
            grid.push(vorto::random_item(&answers).to_string());
            warning = "Hazarda tre".to_string();
          }
          // Unknown command
          _ => {
            warning = "Ne estas ordono".to_string();
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
