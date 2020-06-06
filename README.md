# Tic-Tac-Toe
```
 ○ │ ⨯ │
───┼───┼───
 ⨯ │ ○ │
───┼───┼───
 ⨯ │ ○ │ ○
```

It seems to me that writing a functioning tic-tac-toe game is the new *fizzbuzz* interview question.
This package contains an beautiful tic-tac-toe implementation written in about half an hour.
If you're interviewing me and ask me to implement another tic-tac-toe game,
I'll refer you to this and begin looking for companies who actually want good software engineers (jk 😉).

## Installation
You'll need to have the Rust toolchain installed on your system.
Then, compiling and running `tictactoe` is as simple as:

```
$ git clone https://github.com/slightknack/Tic-Tac-Toe.git
$ cd Tic-Tac-Toe
$ cargo run --release
```

Why run in release mode, you ask? Why not, I respond.

## Challenges
Given that I only spent half an hour on this, I didn't do much more than needed to make this as cool as it could be.
Right now, it supports play between two human players, though it is architected for easy extension.
If you'd like, try implementing your own AI using the `Agent` trait.

See ya!
