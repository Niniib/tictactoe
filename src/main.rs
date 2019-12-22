use joinery::JoinableIterator;
use std::fmt::{Display, Error, Formatter};
use std::io;
use std::io::Write;
use std::num::ParseIntError;

#[derive(Copy, Clone, PartialEq)]
enum Field {
    Empty,
    O,
    X,
}

#[derive(Copy, Clone)]
enum Player {
    O,
    X,
}

impl Player {
    fn field(&self) -> Field {
        match self {
            Player::O => Field::O,
            Player::X => Field::X,
        }
    }

    fn switch_player(&mut self) {
        match self {
            Player::O => {
                *self = Player::X;
            }
            Player::X => {
                *self = Player::O;
            }
        }
    }
}

impl Display for Field {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Field::Empty => write!(formatter, " "),
            Field::O => write!(formatter, "O"),
            Field::X => write!(formatter, "X"),
        }
    }
}

fn main() {
    let mut game_field = [[Field::Empty; 3]; 3];

    print_field(&game_field);

    let mut current_player = Player::X;

    loop {
        let input = loop {
            let field_number_result = choose_field("Please enter a number between 1 and 9: ");

            if let Ok(field_number) = field_number_result {
                if field_number >= 1 && field_number <= 9 {
                    break field_number;
                }
            }
        } as usize
            - 1;

        let x = input / 3;
        let y = input % 3;

        if let Field::Empty = game_field[x][y] {
            game_field[x][y] = current_player.field();
            print_field(&game_field);
            let win_option = check_win(&game_field);

            match win_option {
                Some(Player::X) => {
                    println!("Player X wins");
                    break;
                }
                Some(Player::O) => {
                    println!("Player O wins");
                    break;
                }
                _ => {}
            }

            current_player.switch_player();
            if is_full_field(&game_field) {
                println!("Field is full!");
                break;
            }
        } else {
            println!("This field is occupied!");
        }
    }
}

fn is_full_field(game_field: &[[Field; 3]; 3]) -> bool {
    game_field
        .iter()
        .all(|row| row.iter().all(|field| *field != Field::Empty))
}

fn check_win(game_field: &[[Field; 3]; 3]) -> Option<Player> {
    for row in game_field.into_iter() {
        if row.iter().all(|f| *f == Field::O) {
            return Some(Player::O);
        }
        if row.iter().all(|f| *f == Field::X) {
            return Some(Player::X);
        }
    }
    for column in 0..3 {
        if game_field.into_iter().all(|row| row[column] == Field::O) {
            return Some(Player::O);
        }
        if game_field.into_iter().all(|row| row[column] == Field::X) {
            return Some(Player::X);
        }
    }

    /*
    0 0
    1 1
    2 2
    */
    if (0..3).all(|diagonal| game_field[diagonal][diagonal] == Field::O) {
        return Some(Player::O);
    }
    if (0..3).all(|diagonal| game_field[diagonal][diagonal] == Field::X) {
        return Some(Player::X);
    }

    if (0..3).all(|diagonal| game_field[diagonal][2 - diagonal] == Field::O) {
        return Some(Player::O);
    }
    if (0..3).all(|diagonal| game_field[diagonal][2 - diagonal] == Field::X) {
        return Some(Player::X);
    }

    None
}

fn print_field(game_field: &[[Field; 3]; 3]) {
    let map = game_field
        .iter()
        .map(|&[first, second, third]| format!("{} | {} | {}\n", first, second, third))
        .join_with("--+---+--\n".to_string())
        .to_string();

    print!("{}", map);
}

fn choose_field(prompt: &str) -> Result<u8, ParseIntError> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse()
}
