testing
```rust
#![allow(warnings)]
#![allow(dead_code)]

use std::fmt;
use std::io;

use std::env;
```

testing

```rust
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
enum SquareStatus {
    Naught,
    Cross,
    Empty,
}
struct Square {
    status: SquareStatus,
    id: u32,
}
impl Square {
    fn new(id: u32) -> Square {
        Square {
            status: SquareStatus::Empty,
            id,
        }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self.status {
	    SquareStatus::Cross => f.write_str("X"),
	    SquareStatus::Naught => f.write_str("O"),
	    SquareStatus::Empty => f.write_str(" "),
	}
    }
}
```


enums are really useful
they are a data structure that has to be one of a set of choices


```rust
#[derive(Copy, Clone)]
enum Player {
    Naughts,
    Crosses,
}

fn print_board(board: &[Square]) {
    println!("╭───┬───┬───╮  ");
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


fn has_won(sum: u32) -> bool {
    let primes: [u32; 9] = [2, 3, 5, 7, 11, 13, 17, 19, 23];
    let combos: [[usize; 3]; 8] = [[0, 1, 2], [3, 4, 5], [6, 7, 8], [0, 3, 6], [1, 4, 7], [2, 5, 8], [0, 4, 8], [2, 4, 6]];

    let mut won: bool = false;

    for n in 0..8 {
        let mut win: u32 = primes[combos[n][0]] * primes[combos[n][1]] * primes[combos[n][2]];
        if sum % win == 0 {
            println!("You won!");
            won = true;
        }
    }
    won
}


fn has_lost(board: &[Square]) -> bool {
    let mut lost: bool = true;
    for n in 0..9 {
        if board[n].status == SquareStatus::Empty {
            lost = false;


        }
    }
    if lost {
        println!("Game over! No empty squares");
    }
    lost
}






fn main() {

    println!("\n\nLet's play tictactoe!");

    let mut x: u32 = 1;
    let mut o: u32 = 1;
    let mut game_over: bool = false;

    let mut player: Player = Player::Crosses;
    //let mut board: [Square; 9] = [Square::Empty; 9];
    let mut board: [Square; 9] = [Square::new(2), Square::new(3), Square::new(5), Square::new(7), Square::new(11), Square::new(13), Square::new(17), Square::new(19), Square::new(23)];
    print_board(&board);

    while game_over == false {

        let mut play: usize = take_turn();

        match player {
                Player::Crosses => {
                    board[play].status = SquareStatus::Cross;
                    x = x * board[play].id;
                    player = Player::Naughts;
                },
                Player::Naughts => {
                    board[play].status = SquareStatus::Naught;
                    o = o * board[play].id;
                    player = Player::Crosses;
                },
            }

            print_board(&board);
            game_over = has_won(x);
            game_over = has_won(o);
            game_over = has_lost(&board);
        }





}
```
