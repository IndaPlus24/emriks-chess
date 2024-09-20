// Template Author: Viola Söderlund
// Template Modified by: Isak Larsson

use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
}

#[derive(Clone, Debug)]
pub enum Colour {
    White,
    Black,
}

#[derive(Clone, Debug)]
struct Piece {
    color: Colour,
    piece_type: PieceType,
}

#[derive(Clone, Debug)]
enum PieceType {
    PAWN,
    ROOK,
    BISHOP,
    KNIGHT,
    QUEEN,
    KING
}


/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 */

pub struct Game {
    /* save board, active colour, ... */
    board: Vec<Vec<Option<Piece>>>,//[[Option<Piece>, 8] ,8]//[Option<Piece>; 64],
    active_colour: Colour,
    state: GameState,
    /*black: u64,
    white: u64,*/ // what are these for?
    kings: u64,
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        let mut game = Game {
            /* initialise board, set active colour to white, ... */
            board: vec![vec![None; 8]; 8],
            active_colour: Colour::White,

            state: GameState::InProgress,
            kings: 1,
        };
        //Setting the correct pieces:
        //Pawns
        for x in 0..8 {
            game.board[1][x] = Some(Piece{
                color: Colour::Black,
                piece_type: PieceType::PAWN
            });
            game.board[6][x] = Some(Piece{
                color: Colour::White,
                piece_type: PieceType::PAWN
            });
        }
        let colours = [Colour::Black, Colour::White];
        let pieces = [PieceType::ROOK, PieceType::KNIGHT, PieceType::BISHOP, PieceType::QUEEN, PieceType::KING, PieceType::BISHOP, PieceType::KNIGHT, PieceType::ROOK];

        //colours.into_iter().map()

        for (c_index, color) in colours.iter().enumerate() {
            for (p_index, piece) in pieces.iter().enumerate() {
                game.board[c_index * 7][p_index] = Some(Piece{
                    color: color.clone(),
                    piece_type: piece.clone()
                });
            }
        }


        return game
    }

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, from: String, to: String) -> Option<GameState> {
        let mut vec: Vec<String> = Vec::with_capacity(60);

        None
    }

    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, piece: String) -> () {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece. Don't forget to the rules for check.
    ///
    /// (optional) Don't forget to include en passent and castling.
    pub fn get_possible_moves(&self, postion: String) -> Option<Vec<String>> {
        None
    }
}

/// Implement print routine for Game.
///
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */

        write!(f, "")
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {
        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }

    #[test]
    fn display_board_starting_position() {
        let game = Game::new();

        //Prints the board:
        println!("");
        for i in 0..8 {
            print!("|-----------|\t");
        }
        for item in &game.board {
            println!("\n");
            for item_inner in item {
                match item_inner {
                    Some(piece) => {
                        print!(" {:?}", piece.color);
                        print!(" ");
                        print!("{:?}", piece.piece_type);
                    }, 
                    None => print!("\t\t"),
                }
                
                print!("\t");
            }
            println!("\n");
            for item_inner in item {
                print!("|-----------|\t")
            }
            
        }

        assert_eq!("fill with data", "fill with data");
    }
}