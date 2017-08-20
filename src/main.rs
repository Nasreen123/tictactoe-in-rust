
#![allow(dead_code)]
use std::fmt;
use std::io;

#[derive(Copy, Clone)]
enum Square {
    Naught,
    Cross,
    Empty,
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self {
	    &Square::Cross => f.write_str("X"),
	    &Square::Naught => f.write_str("O"),
	    &Square::Empty => f.write_str(" "),
	}
    }
}


#[derive(Copy, Clone)]
enum Player {
    Naughts,
    Crosses,
}

fn print_board(board: &[Square]) {
    println!("╭───┬───┬───╮");
    println!("│ {} │ {} │ {} │", &board[0], &board[1], &board[2]);
    println!("├───┼───┼───┤");
    println!("│ {} │ {} │ {} │", &board[3], &board[4], &board[5]);
    println!("├───┼───┼───┤");
    println!("│ {} │ {} │ {} │", &board[6], &board[7], &board[8]);
    println!("╰───┴───┴───╯");
}

fn take_turn() -> usize {
    let mut play = String::new();

    io::stdin().read_line(&mut play)
        .expect("Failed to read line");

    let play: usize = play.trim().parse()
        .expect("Please type a number!");

    println!("you played: {}", play);

    play
}


fn main() {


    println!("Let's play tictactoe!");

    let mut player: Player = Player::Crosses;
    let mut board: [Square; 9] = [Square::Empty; 9];

    print_board(&board);

    loop {

        let mut play: usize = take_turn();

        match player {
            Player::Crosses => board[play] = Square::Cross,
            Player::Naughts => board[play] = Square::Naught,
        }

        print_board(&board);

        match player {
            Player::Crosses => player = Player::Naughts,
            Player::Naughts => player = Player::Crosses,
        }

    }




}
