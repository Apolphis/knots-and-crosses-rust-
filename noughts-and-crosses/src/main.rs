
use std::fmt;
use std::io;

// Enum for storing states a square can be in: { X, O, Unused(Blank) }
#[derive(Copy, Clone, Debug)]
enum SquareState {
    X,
    O,
    Blank
}

// Implementation of fmt::Display so SquareStates can be printed
impl fmt::Display for SquareState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// The game state
struct GameState {
    board: [SquareState; 9],
    is_player_turn: bool,

}

impl GameState {
    // updates square at given [row, col] to give SquareState { X, O } (Blank gives error)
    fn update_square(&mut self, row: usize, col: usize, sq_state: SquareState) {
        let index = self.get_board_index(row,col);

        match sq_state {
            SquareState::Blank => {
                println!("error trying to modify a square to be blank");
                std::process::exit(1);
            },
            _ => ()
        }

        match self.board[index] {
            SquareState::Blank => {
              self.board[index] = sq_state;
            },
            _ => println!("Error, square has already been modified");
            std::process::exit(1);
        }
    }

    // Convert { Row, Col } -> Index in board array
    fn get_board_index(&self, row: usize, col: usize) -> usize {
        return row * 3 + col;
    }
}

// Initialises a GameState object to the starting state
fn init_game_state(is_player_turn_: bool) -> GameState {
    return GameState {
       board: [SquareState::Blank; 9],
       is_player_turn: is_player_turn_
    };
}


fn setup_game() -> GameState {
    let mut input = String::new();

    println!("Please select who will go first:\n1. player\n2. ai");

    let starting_player: bool = loop {
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

        println!("");
        break match input.trim() {
            "1" | "player" => true,
            "2" | "ai" => false,
            _ => {
                input.clear();
                continue;
            }
        };
    };
        
    let state = init_game_state(starting_player);

    println!("{}", state.is_player_turn);
    println!("{}", state.board[0]);

    return state;
}

fn print_board(state: &GameState) {
    let mut output = String::new();

    let mut count = 0;
    for state in state.board {
     count = count + 1;
     
     let square: &str = match state {
         SquareState::Blank => " ",
         SquareState::X => "X",
         SquareState::O => "O"
     };

     
     output.push_str(&square);
     if count % 3 == 0 {
         output.push_str("\n");
     }
    }

    println!("{}", &output);
}




fn main() {
  let mut state: GameState = setup_game();
  print_board(&state);

  state.update_square(0,2,SquareState::X);
  state.update_square(1,1,SquareState::O);
  //state.update_square(2,2,SquareState::Blank); // changing square to blank -> Error (handled)
  state.update_square(2,2,SquareState::X);
 // state.update_square(1,1,SquareState::X); -> // changing square already changed -> Error (handled)
  println!("--------");
  print_board(&state);
}
