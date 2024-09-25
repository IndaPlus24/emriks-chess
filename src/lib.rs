// Template Author: Viola Söderlund
// Template Modified by: Isak Larsson

use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Colour {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    color: Colour,
    piece_type: PieceType,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
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
    /// I have changed from it having String parameters to Vec parameters
    pub fn make_move(&mut self, from: Vec<usize>, to: Vec<usize>) -> Option<GameState> {
        //let mut vec: Vec<String> = Vec::with_capacity(60);
        
        // Check gamestate
        if self.get_game_state() == GameState::GameOver {
            return Some(GameState::GameOver);
        }

        // Get piece from to
        let piece_option = self.board[from[0]][from[1]];

        // Check if there is a piece there
        if piece_option == None {
            return None;
        }
        let piece = piece_option.unwrap();

        // Check if legal
        let legal_moves_option = self.get_possible_moves(&from);
        match legal_moves_option {
            Some(legal_moves) => {
                if !legal_moves.contains(&to) {
                    //to isn't part of legal moves
                    return None;
                }
            },
            None => return None,
        }

        // Update board
        self.board[to[0]][to[1]] = Some(piece);
        self.board[from[0]][from[1]] = None;


        return Some(GameState::InProgress);
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
    /// I have changed from params being String to &Vec<usize> and return value to Option<Vec<Vec<usize>>>
    pub fn get_possible_moves(&self, position: &Vec<usize>) -> Option<Vec<Vec<usize>>> {
        let piece = self.board[position[0]][position[1]].unwrap();
        
        // ==========================
        // ROOK:

        if piece.piece_type == PieceType::ROOK {

            let mut movement_positions: Vec<Vec<usize>> = vec![];
            let mut blocking_pieces_positions: Vec<Vec<usize>> = vec![];
            let mut forbidden_positions: Vec<Vec<usize>> = vec![];

            for x in 0..8 {
                for y in 0..8 {
                    //movement:
                    if position[0] == y || position[1] == x {
                        //It is on the same row or column
                        movement_positions.push(vec![y,x]);

                        match self.board[y][x] {
                            Some(value) => blocking_pieces_positions.push(vec![y,x]),
                            None => {},   
                        }
                    }
                }
            }

            let piece_index_block = blocking_pieces_positions.iter().position(|pos| pos == position).unwrap();
            blocking_pieces_positions.remove(piece_index_block);
            let piece_index_movment = movement_positions.iter().position(|pos| pos == position).unwrap();
            movement_positions.remove(piece_index_movment);

            for blocking_position in blocking_pieces_positions {
                if blocking_position[0] == position[0] {
                    //On the same row
                    if blocking_position[1] > position[1] {
                        for x in blocking_position[1]+1..8 {
                            if let Some(index) = movement_positions.iter().position(|pos| pos == &vec![blocking_position[0], x]) { 
                                movement_positions.remove(index);
                            }
                        }
                    }
                    else {
                        for x in 0..blocking_position[1] {
                            if let Some(index) = movement_positions.iter().position(|pos| pos == &vec![blocking_position[0], x]) { 
                                movement_positions.remove(index);
                            }
                        }
                    }
                }
                else if blocking_position[1] == position[1] {
                    //On the same column
                    if blocking_position[0] > position[0] {
                        for y in blocking_position[0]+1..8 {
                            if let Some(index) = movement_positions.iter().position(|pos| pos == &vec![y, blocking_position[1]]) { 
                                movement_positions.remove(index);
                            }
                        }
                    }
                    else {
                        for y in 0..blocking_position[0] {
                            if let Some(index) = movement_positions.iter().position(|pos| pos == &vec![y, blocking_position[1]]) { 
                                movement_positions.remove(index);
                            }
                        }
                    }
                }
            }

            // Find forbidden positions

            //Shouldn't be able to attack own color
            for pos in &movement_positions {
                if let Some(board_piece) = self.board[pos[0]][pos[1]] {
                    if board_piece.color == piece.color {
                        forbidden_positions.push(pos.clone());
                    }
                }
            }

            

            //remove forbidden

            movement_positions.retain(|pos| !forbidden_positions.contains(pos));

            return Some(movement_positions);

        }


        // ==========================
        // PAWN:

        if piece.piece_type == PieceType::PAWN {

            let mut movement_positions: Vec<Vec<usize>> = vec![];
            let mut attack_positions: Vec<Vec<usize>> = vec![];
            let mut blocking_pieces_positions: Vec<Vec<usize>> = vec![];
            let mut forbidden_positions: Vec<Vec<usize>> = vec![];

            for x in 0..8 {
                for y in 0..8 {
                    //movement:
                    if position[1] == x && ((piece.color == Colour::Black && position[0] < y) || (piece.color == Colour::White && position[0] > y)) {
                        //If is on the same column and ahead of the pawn
                        //Black can move go downwards and white can only move upwards

                        let dist = (position[0] as i32 - y as i32).abs();
                        print!("pos0: {}", position[0]);
                        print!(" y: {}", y);
                        print!(" dist: {}", dist);
                        print!("  ");

                        
                        
                        match dist {
                            1 => movement_positions.push(vec![y,x]),
                            2 => {
                                if (piece.color == Colour::Black && position[0] == 1 && self.board[y-1][x].is_none()) || (piece.color == Colour::White && position[0] == 6 && self.board[y+1][x].is_none()){
                                    //The double movement is not blocked by a piece right in front of the pawn
                                    movement_positions.push(vec![y,x]);
                                }
                                else {
                                    continue;
                                }  
                            },
                            _ => continue
                        }

                        match self.board[y][x] {
                            Some(value) => blocking_pieces_positions.push(vec![y,x]),
                            None => {},   
                        }
                    }
                    
                    //diagonal (pawn attack squares)
                    else if position[1] == x+1 || position[1] == ((x as i32)-1) as usize {
                        if piece.color == Colour::Black && position[0] == ((y as i32)-1) as usize {
                            if let Some(board_piece) = self.board[y][x] {
                                attack_positions.push(vec![y,x]);
                            }
                            
                        }
                        else if piece.color == Colour::White && position[0] == y+1 {
                            if let Some(board_piece) = self.board[y][x] {
                                attack_positions.push(vec![y,x]);
                            }
                        } 
                    }
                }
            }

            //remove blocking
            movement_positions.retain(|pos| !blocking_pieces_positions.contains(pos));


            //add attack 
            movement_positions.extend(attack_positions);

            // Find forbidden positions

            //Shouldn't be able to attack own color
            for pos in &movement_positions {
                if let Some(board_piece) = self.board[pos[0]][pos[1]] {
                    if board_piece.color == piece.color {
                        forbidden_positions.push(pos.clone());
                    }
                }
            }

            

            //remove forbidden

            movement_positions.retain(|pos| !forbidden_positions.contains(pos));

            return Some(movement_positions);

        }

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

    //Auxilirary functions
    fn print_board(game: &Game){
        //Prints the board:
        /*println!("");
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
            
        }*/
        println!("");
        print!("|");
        for i in 0..8 {
            print!("-----");
        }
        print!("|");
        for item in &game.board {
            println!("");
            print!("|");
            for item_inner in item {
                print!(" ");
                match item_inner {
                    Some(piece) => {
                        print!("{}", format!("{:?}", piece.color).chars().next().unwrap());
                        print!("-");
                        print!("{}", format!("{:?}", piece.piece_type).chars().next().unwrap());
                    }, 
                    None => print!("---"),
                }
                
                print!(" ");
            }
            print!("|");
            
        }
        println!("");
        print!("|");
        for i in 0..8 {
            print!("-----");
        }
        print!("|");

        println!("\n\n")

    }

    fn print_board_moves(game: &Game, position: &Vec<usize>){

        let legal_moves = game.get_possible_moves(position);


        println!("");
        print!("|");
        for i in 0..8 {
            print!("-----");
        }

        for y in 0..8 {
            println!("");
            print!("|");
            for x in 0..8 {
                if y == position[0] && x == position[1] {
                    print!("(");
                }
                else {
                    print!(" ");
                }
            
                if let Some(index) = legal_moves.clone().unwrap().iter().position(|pos| pos == &vec![y as usize, x as usize]){
                    print!("[-]");
                }
                else {
                    match game.board[y][x] {
                        Some(piece) => {
                            print!("{}", format!("{:?}", piece.color).chars().next().unwrap());
                            print!("-");
                            print!("{}", format!("{:?}", piece.piece_type).chars().next().unwrap());
                        }, 
                        None => print!("---"),
                    }
                }
                if y == position[0] && x == position[1] {
                    print!(")");
                }
                else {
                    print!(" ");
                }
                
            }
            print!("|");
        }
        println!("");
        print!("|");
        for i in 0..8 {
            print!("-----");
        }
        print!("|");

        println!("\n\n");

    }

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

        print_board(&game);

        assert_eq!("fill with data", "fill with data");
    }

    #[test]
    fn move_piece() {
        /*let mut game = Game::new();
        print_board(&game);
        match game.make_move(vec![0 as usize, 0 as usize], vec![3 as usize, 3 as usize]) {
            Some(action) => {
                if action != GameState::InProgress {
                    println!("SOMETHING IS GOING ON");
                    return;
                }
            },
            None => {
                println!("Illegal move!");
                return;
            }
        }
        print_board(&game);
        game.make_move(vec![3 as usize, 3 as usize], vec![6 as usize, 3 as usize]);
        print_board(&game);
        game.make_move(vec![6 as usize, 3 as usize], vec![2 as usize, 5 as usize]);
        print_board(&game);*/
    }

    #[test]
    fn movement_options_rook() {
        let mut game = Game::new();

        print_board(&game);
        print_board_moves(&game, &vec![1,0]);
        game.make_move(vec![1, 0], vec![3, 0]);
        print_board(&game);
        print_board_moves(&game, &vec![3,0]);
        print_board_moves(&game, &vec![6,0]);
        game.make_move(vec![3, 0], vec![4, 0]);
        print_board(&game);
        print_board_moves(&game, &vec![4,0]);
        print_board_moves(&game, &vec![6,0]);
        game.make_move(vec![4, 0], vec![5, 0]);
        print_board(&game);
        print_board_moves(&game, &vec![5,0]);
        print_board_moves(&game, &vec![6,0]);
        game.make_move(vec![5, 0], vec![6, 1]);
        print_board(&game);
        print_board_moves(&game, &vec![6,1]);
        print_board_moves(&game, &vec![6,0]);
        
        
        

        /*print_board(&game);
        print_board_moves(&game, &vec![0,0]);
        game.make_move(vec![1, 0], vec![2, 0]);
        print_board(&game);
        print_board_moves(&game, &vec![0,0]);
        game.make_move(vec![2, 0], vec![3, 0]);
        print_board(&game);
        print_board_moves(&game, &vec![0,0]);
        game.make_move(vec![0, 0], vec![2, 0]);
        print_board(&game);
        print_board_moves(&game, &vec![2,0]);*/

        /*print_board(&game);
        print_board_moves(&game, &vec![0,0]);
        game.make_move(vec![0 as usize, 0 as usize], vec![3 as usize, 6 as usize]);
        print_board(&game);
        print_board_moves(&game, &vec![3,6]);
        game.make_move(vec![3 as usize, 6 as usize], vec![6 as usize, 3 as usize]);
        print_board(&game);
        print_board_moves(&game, &vec![6,3]);
        game.make_move(vec![6 as usize, 3 as usize], vec![7 as usize, 5 as usize]);
        print_board(&game);
        print_board_moves(&game, &vec![7,5]);*/
    }
}