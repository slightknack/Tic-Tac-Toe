use std::io;
use std::io::Write;

/// Reads a single line after prompting the player.
pub fn read(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("unable to prompt player");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("unable to read player input");

    return input.trim().to_string();
}
