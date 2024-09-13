use std::{io, ops};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Color {
    Yellow,
    Red,
}

impl ops::Not for Color {
    type Output = Color;

    fn not(self) -> Color {
        match self {
            Color::Red => Color::Yellow,
            Color::Yellow => Color::Red,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Coin {
    x: usize,
    y: usize,
    color: Color,
}

// 7x6
type Board = [[Option<Coin>; 6]; 7];

#[derive(Debug, PartialEq, Clone)]
struct Game {
    board: Board,
    turn: Color,
}

#[derive(Debug, PartialEq, Clone)]
enum DropResult {
    Valid,
    Invalid,
    Win,
}

impl Game {
    fn new() -> Self {
        Game {
            board: [[None; 6]; 7],
            turn: Color::Yellow,
        }
    }

    fn drop_coin(&mut self, x: usize) -> DropResult {
        if x > 6 {
            return DropResult::Invalid;
        }

        let mut placed = false;

        for y in 0..6 {
            if self.board[x][y].is_none() {
                self.board[x][y] = Some(Coin {
                    x,
                    y,
                    color: self.turn,
                });
                placed = true;
                break;
            }
        }

        if !placed {
            return DropResult::Invalid;
        }

        if self.is_win() {
            return DropResult::Win;
        }

        self.turn = !self.turn;

        DropResult::Valid
    }

    fn win_in_direction(&self, start: (usize, usize), direction: (i32, i32)) -> bool {
        let target_color = self.board[start.0][start.1].unwrap().color;

        for i in 1..4 {
            let x = start.0 as i32 + direction.0 * i;
            let y = start.1 as i32 + direction.1 * i;

            //println!("x {} y {} i {}", x, y, i);

            if x < 0 || y < 0 || x > 6 || y > 5 {
                //println!("BAD POS");
                return false;
            }

            //println!("{:?}", self.board[x as usize][y as usize]);
            if !self.board[x as usize][y as usize].is_some_and(|coin| coin.color == target_color) {
                //println!("NOT WIN XD");
                return false;
            }
        }

        //println!("Skibidi true");

        true
    }

    fn is_win(&self) -> bool {
        let directions = [
            (1, 0),
            (0, 1),
            (-1, 0),
            (0, -1),
            (1, 1),
            (-1, 1),
            (1, -1),
            (-1, -1),
        ];
        //let directions = [(0, 1)];

        let coins = self
            .board
            .iter()
            .flatten()
            .filter_map(|x| *x)
            .collect::<Vec<Coin>>();

        for coin in coins {
            for direction in directions {
                if self.win_in_direction((coin.x, coin.y), direction) {
                    return true;
                }
            }
        }

        false
    }

    fn print_board(&self) {
        for y in (0..6).rev() {
            for x in 0..7 {
                let coin = self.board[x][y];
                match coin {
                    Some(c) => match c.color {
                        Color::Red => print!("😡 "),
                        Color::Yellow => print!("😠 "),
                    },
                    None => print!(" . "),
                }
            }
            println!();
        }
    }
}

fn main() {
    let mut game = Game::new();
    loop {
        game.print_board();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let x: usize = input.trim().parse().unwrap();
        let status = game.drop_coin(x);
        match status {
            DropResult::Invalid => println!("Invalid drop, try again!"),
            DropResult::Win => {
                println!("You won!");
                break;
            }
            DropResult::Valid => continue,
        }
    }
    game.print_board();
}
