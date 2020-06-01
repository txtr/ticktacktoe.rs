const BOARD_SIZE: usize = 3;

struct Player {
    name: String,
    score: i8,
}

#[derive(PartialEq, Copy, Clone)]
enum Owner {
    X,
    O,
    Empty,
}

struct Board {
    state: [[Owner; BOARD_SIZE]; BOARD_SIZE],
}

struct Game {
    board: Board,
    player_x: &'static Player,
    player_o: &'static Player,
    move_count: u8,
}

impl Player {
    fn new() -> Player {
        let mut name: String = String::new();
        std::io::stdin()
            .read_line(&mut name)
            .expect("Failed to read your tag");
        let name: String = name; // Mutability Shadowing
        let name: String = name.trim().to_string();
        Player { name, score: 0 }
    }

    fn scored(&mut self) {
        self.score = self.score + 1;
    }
}

impl Board {
    fn new() -> Board {
        Board {
            state: [[Owner::Empty; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    fn set_state(&mut self, cell_coordinate: (usize, usize), owner: Owner) {
        self.state[cell_coordinate.0][cell_coordinate.1] = owner;
    }

    fn who_won(&self) -> Owner {
        let mut ended: bool = false;
        for row_i in 0..BOARD_SIZE {
            if let Owner::Empty = self.state[row_i][0] {
                continue;
            } else {
                let mut row_won: bool = true;
                for col_i in 1..BOARD_SIZE {
                    if self.state[row_i][col_i] != self.state[row_i][col_i - 1] {
                        row_won = false;
                    }
                }
                if row_won {
                    return self.state[row_i][0];
                }
            }
        }

        for col_i in 0..BOARD_SIZE {
            if let Owner::Empty = self.state[0][col_i] {
                continue;
            } else {
                let mut col_won: bool = true;
                for row_i in 1..BOARD_SIZE {
                    if self.state[row_i][col_i] != self.state[row_i - 1][col_i] {
                        col_won = false;
                    }
                }
                if col_won {
                    return self.state[0][col_i];
                }
            }
        }

        if let Owner::Empty = self.state[0][0] {
        } else {
            let mut diag_ended: bool = true;
            for cr_i in 1..BOARD_SIZE {
                if self.state[cr_i][cr_i] != self.state[cr_i - 1][cr_i - 1] {
                    diag_ended = false;
                }
            }
            if diag_ended {
                return self.state[0][0];
            }
        }

        if let Owner::Empty = self.state[BOARD_SIZE - 1][BOARD_SIZE - 1] {
        } else {
            let mut diag_ended: bool = true;
            for cr_i in 1..BOARD_SIZE {
                if self.state[cr_i][BOARD_SIZE - cr_i - 1]
                    != self.state[cr_i - 1][BOARD_SIZE - cr_i]
                {
                    diag_ended = false;
                }
            }
            if diag_ended {
                return self.state[BOARD_SIZE - 1][BOARD_SIZE - 1];
            }
        }
        return Owner::Empty;
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut repr: String = String::from("┌───┬───┬───┐\n│");
        for row_i in 0..BOARD_SIZE {
            for col_i in 0..BOARD_SIZE {
                let symbol: String = match self.state[row_i][col_i] {
                    Owner::X => String::from("⨯"),
                    Owner::O => String::from("○"),
                    Owner::Empty => String::from(" "),
                };
                repr = [repr, (*(" ")).to_string(), symbol, (*(" │")).to_string()].concat();
            }
            if (row_i != BOARD_SIZE - 1) {
                repr = [repr, (*("\n├───┼───┼───┤\n│")).to_string()].concat();
            }
        }
        repr = [repr, (*("\n└───┴───┴───┘\n")).to_string()].concat();
        write!(f, "{}", repr)
    }
}

impl Game {
    fn new(player_x: &'static Player, player_o: &'static Player) -> Game {
        let mut board = Board::new();
        Game {
            board,
            player_x,
            player_o,
            move_count: 0,
        }
    }

    fn play(&self) {}
}

fn main() {
    println!("Hi Player 1, enter your tag here --> ");
    let mut player_1: Player = Player::new();
    println!("Hi Player 2, enter your tag here --> ");
    let mut player_2: Player = Player::new();
    let players: (&'static Player, &'static Player) = (&mut player_1, &mut player_2);
    println!(
        "{} & {} has entered the competition.",
        players.0.name, players.1.name
    );

    let forfeit: bool = false;
    let game_i: usize = 1;
    while !forfeit {
        let game: Game = Game::new(&mut players.0, &mut players.1);
    }
}
