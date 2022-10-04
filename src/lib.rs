pub type Grid = Vec<String>;

/// Game state
pub enum State {
  Play,
  Loss,
  Win,
}

/// Get random item of vector
pub fn random_item<'a, T>(vec: &'a Vec<T>) -> &'a T {
  use rand::seq::SliceRandom;

  vec
    .choose(&mut rand::thread_rng())
    .expect("Could not get random value of vector")
}

/// Generate lines to print
pub fn get_lines(grid: &mut Grid, answer: &str) -> Vec<String> {
  let mut print: Vec<String> = vec![];

  //? Move inside loop ? Might not affect performance
  let answer_vec = answer.chars().collect::<Vec<char>>();

  for line in grid {
    let mut curr = String::from("\x1b[36m│\x1b[0m");

    for (i, ch) in line.chars().enumerate() {
      if answer_vec.get(i).unwrap_or(&' ') == &ch {
        curr += "\x1b[32m"; // Green
      } else if answer_vec.contains(&ch) {
        curr += "\x1b[33m"; // Yellow
      } else {
        curr += "\x1b[0m"; // White / Default
      }

      //? Add color reset here ?
      curr.push(ch);
    }

    curr += "\x1b[36m│\x1b[0m";
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
