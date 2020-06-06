pub mod marking;
pub mod position;
pub mod board;
pub mod game;
pub mod agent;
pub mod human;
pub mod input;

/// Runs a standard game between two human players.
/// Implementing an AI is left as an exercise to the reader;
/// I've wasted enough time on this.
fn main() {
    println!(
        "\n\n\n{}",
        "\tWelcome to Ultimate\n".to_owned()
      + "\t _   _      _             _\n"
      + "\t| |_(_) ___| |_ __ _  ___| |_ ___   ___\n"
      + "\t| __| |/ __| __/ _` |/ __| __/ _ \\ / _ \\\n"
      + "\t| |_| | (__| || (_| | (__| || (_) |  __/\n"
      + "\t \\__|_|\\___|\\__\\__,_|\\___|\\__\\___/ \\___|\n\n"
      + "\tpress ^C to quit\n",
    );

    let p1_name = input::read("player 1, enter your name: ");
    let p2_name = input::read("player 2, enter your name: ");
    println!();

    let p1 = human::Human::proxy(p1_name);
    let p2 = human::Human::proxy(p2_name);

    let g = game::Game::new(Box::new(p1), Box::new(p2));
    g.run();

    println!("thanks for playing!\n\n\n");
}
