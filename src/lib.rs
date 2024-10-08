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
    pub color: Colour,
    pub piece_type: PieceType,
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
    pub board: Vec<Vec<Option<Piece>>>,
    pub active_colour: Colour,
    pub state: GameState,
    pub promotion_type: PieceType,
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        let mut game = Game {
            /* initialise board, set active colour to white, ... */
            board: vec![vec![None; 8]; 8],
            active_colour: Colour::White,
            state: GameState::InProgress,
            promotion_type: PieceType::QUEEN,
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

        // Check if it is that color's turn
        if piece.color != self.active_colour {
            return None;
        }

        // Check if legal
        let legal_moves_option = self.get_possible_moves(&self.board, &from, false);
        match legal_moves_option {
            Some(legal_moves) => {
                if !legal_moves.contains(&to) {
                    //to isn't part of legal moves
                    return None;
                }
            },
            None => return None,
        }

        // Check if it wins the game
        if let Some(board_piece) = self.board[to[0]][to[1]] {
            if board_piece.piece_type == PieceType::KING {
                self.board[to[0]][to[1]] = Some(piece);
                self.board[from[0]][from[1]] = None;
                return Some(GameState::GameOver);
            }
        }

        // Update board
        self.board[to[0]][to[1]] = Some(piece);
        self.board[from[0]][from[1]] = None;


        // Change active_color
        if self.active_colour == Colour::White {
            self.active_colour = Colour::Black;
        }
        else {
            self.active_colour = Colour::White;
        }
        

        // Check if it is Check
        //find king
        let mut king = vec![10,10]; //Value out of range on purpose
        for y in 0..8 {
            for x in 0..8 {
                if let Some(board_piece) = self.board[y][x] {
                    if board_piece.piece_type == PieceType::KING && board_piece.color != piece.color {
                        king = vec![y, x];
                    }
                }
            }
        }
        for y in 0..8 {
            for x in 0..8 {
                if let Some(board_piece) = self.board[y][x] {
                    if board_piece.color == piece.color {
                        //It is one of your pieces
                        let moves_option = self.get_possible_moves(&self.board, &vec![y, x], true);
                        match moves_option {
                            Some(moves) => {
                                if moves.contains(&king) {
                                    //Check detected!
                                    return Some(GameState::Check);
                                }
                            },
                            None => continue,
                        }   
                    }
                }
            }
        }




        return Some(GameState::InProgress);
    }

    /// Set the piece type that a peasant becames following a promotion.
    /// I've changed it from a string to a PieceType
    pub fn set_promotion(&mut self, piece: PieceType) -> () {
        if piece != PieceType::KING {
            self.promotion_type = piece;
        }
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
    pub fn get_possible_moves(&self, board: &Vec<Vec<Option<Piece>>>, position: &Vec<usize>, call_is_recursive: bool) -> Option<Vec<Vec<usize>>> {
        let piece = board[position[0]][position[1]].unwrap();
        
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

                        match board[y][x] {
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
                if let Some(board_piece) = board[pos[0]][pos[1]] {
                    if board_piece.color == piece.color {
                        forbidden_positions.push(pos.clone());
                    }
                }
            }

            //remove forbidden
            movement_positions.retain(|pos| !forbidden_positions.contains(pos));


            //Check if move leads to a check on your king, it shouldn't be allowed in that case
            if !call_is_recursive {
                //find king
                let mut king = vec![10,10]; //Value out of range on purpose
                for y in 0..8 {
                    for x in 0..8 {
                        if let Some(board_piece) = board[y][x] {
                            if board_piece.piece_type == PieceType::KING && board_piece.color == piece.color {
                                king = vec![y, x];
                            }
                        }
                    }
                }
                for action in movement_positions.clone() {
                    let mut temp_board = board.clone();
                    temp_board[action[0]][action[1]] = Some(piece);
                    temp_board[position[0]][position[1]] = None;
                    for y in 0..8 {
                        for x in 0..8 {
                            if let Some(board_piece) = temp_board[y][x] {
                                if board_piece.color != piece.color {
                                    //It is an enemy piece
                                    let moves_option = self.get_possible_moves(&temp_board, &vec![y, x], true);
                                    match moves_option {
                                        Some(moves) => {
                                            if moves.contains(&king) {
                                                //Check detected!
                                                //Remove that move from movement_positions since it would lead to a check 
                                                if let Some(check_move_index) = movement_positions.iter().position(|pos| pos == &action){
                                                    movement_positions.remove(check_move_index);
                                                }
                                            }
                                        },
                                        None => continue,
                                    }   
                                }
                            }
                        }
                    }
                }
            }


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
        
                        match dist {
                            1 => movement_positions.push(vec![y,x]),
                            2 => {
                                if (piece.color == Colour::Black && position[0] == 1 && board[y-1][x].is_none()) || (piece.color == Colour::White && position[0] == 6 && board[y+1][x].is_none()){
                                    //The double movement is not blocked by a piece right in front of the pawn
                                    movement_positions.push(vec![y,x]);
                                }
                                else {
                                    continue;
                                }  
                            },
                            _ => continue
                        }

                        match board[y][x] {
                            Some(value) => blocking_pieces_positions.push(vec![y,x]),
                            None => {},   
                        }
                    }
                    
                    //diagonal (pawn attack squares)
                    else if position[1] == x+1 || position[1] == ((x as i32)-1) as usize {
                        if piece.color == Colour::Black && position[0] == ((y as i32)-1) as usize {
                            if let Some(board_piece) = board[y][x] {
                                attack_positions.push(vec![y,x]);
                            }
                            
                        }
                        else if piece.color == Colour::White && position[0] == y+1 {
                            if let Some(board_piece) = board[y][x] {
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
                if let Some(board_piece) = board[pos[0]][pos[1]] {
                    if board_piece.color == piece.color {
                        forbidden_positions.push(pos.clone());
                    }
                }
            }

            //remove forbidden
            movement_positions.retain(|pos| !forbidden_positions.contains(pos));

            //Check if move leads to a check on your king, it shouldn't be allowed in that case
            if !call_is_recursive {
                //find king
                let mut king = vec![10,10]; //Value out of range on purpose
                for y in 0..8 {
                    for x in 0..8 {
                        if let Some(board_piece) = board[y][x] {
                            if board_piece.piece_type == PieceType::KING && board_piece.color == piece.color {
                                king = vec![y, x];
                            }
                        }
                    }
                }
                for action in movement_positions.clone() {
                    let mut temp_board = board.clone();
                    temp_board[action[0]][action[1]] = Some(piece);
                    temp_board[position[0]][position[1]] = None;
                    for y in 0..8 {
                        for x in 0..8 {
                            if let Some(board_piece) = temp_board[y][x] {
                                if board_piece.color != piece.color {
                                    //It is an enemy piece
                                    let moves_option = self.get_possible_moves(&temp_board, &vec![y, x], true);
                                    match moves_option {
                                        Some(moves) => {
                                            if moves.contains(&king) {
                                                //Check detected!
                                                //Remove that move from movement_positions since it would lead to a check 
                                                if let Some(check_move_index) = movement_positions.iter().position(|pos| pos == &action){
                                                    movement_positions.remove(check_move_index);
                                                }
                                            }
                                        },
                                        None => continue,
                                    }   
                                }
                            }
                        }
                    }
                }
            }

            return Some(movement_positions);
        }


        // ==========================
        // BISHOP:

        if piece.piece_type == PieceType::BISHOP {

            let mut movement_positions: Vec<Vec<usize>> = vec![];
            let mut blocking_pieces_positions: Vec<Vec<usize>> = vec![];
            let mut forbidden_positions: Vec<Vec<usize>> = vec![];

            for x in 0..8 {
                for y in 0..8 {
                    //movement:
                    if y+x == position[0]+position[1] || (y as i32)-(x as i32) == ((position[0] as i32) - (position[1] as i32)) {
                        //It is on the same diagonal
                        movement_positions.push(vec![y,x]);

                        match board[y][x] {
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
                if blocking_position[0]+blocking_position[1] == position[0]+position[1] {
                    //The blocking piece is on the bottom-left to top-right diagonal
                    if blocking_position[0] < position[0] {
                        let mut y = blocking_position[0];
                        for x in blocking_position[1]+1..8 {
                            if y > 0 {
                                y-=1;
                                if let Some(index) = movement_positions.iter().position(|pos| pos == &vec![y, x]) { 
                                    movement_positions.remove(index);
                                }
                            }
                        }
                    }
                    else {
                        let mut x = blocking_position[1];
                        for y in blocking_position[0]+1..8 {
                            if x > 0 {
                                x-=1;
                                if let Some(index) = movement_positions.iter().position(|pos| pos == &vec![y, x]) { 
                                    movement_positions.remove(index);
                                }
                            }
                        }
                    }
                }
                else if (blocking_position[0] as i32)-(blocking_position[1] as i32) == ((position[0] as i32) - (position[1] as i32)) {
                    //The blocking piece is on the top-left to bottom-right diagonal
                    if blocking_position[0] < position[0] {
                        let mut y = (blocking_position[0] as i32)-(blocking_position[1] as i32);
                        for x in 0..blocking_position[1] {
                            if y >= 0 {
                                if let Some(index) = movement_positions.iter().position(|pos| pos == &vec![y as usize, x]) { 
                                    movement_positions.remove(index);
                                }
                            }
                            y+=1;
                        }
                    }
                    else {
                        let mut x = blocking_position[1];
                        for y in blocking_position[0]+1..8 {
                            x+=1;
                            if let Some(index) = movement_positions.iter().position(|pos| pos == &vec![y, x]) { 
                                movement_positions.remove(index);
                            }
                        }
                    }
                }
            }

            // Find forbidden positions
            //Shouldn't be able to attack own color
            for pos in &movement_positions {
                if let Some(board_piece) = board[pos[0]][pos[1]] {
                    if board_piece.color == piece.color {
                        forbidden_positions.push(pos.clone());
                    }
                }
            }

            //remove forbidden
            movement_positions.retain(|pos| !forbidden_positions.contains(pos));

            //Check if move leads to a check on your king, it shouldn't be allowed in that case
            if !call_is_recursive {
                //find king
                let mut king = vec![10,10]; //Value out of range on purpose
                for y in 0..8 {
                    for x in 0..8 {
                        if let Some(board_piece) = board[y][x] {
                            if board_piece.piece_type == PieceType::KING && board_piece.color == piece.color {
                                king = vec![y, x];
                            }
                        }
                    }
                }
                for action in movement_positions.clone() {
                    let mut temp_board = board.clone();
                    temp_board[action[0]][action[1]] = Some(piece);
                    temp_board[position[0]][position[1]] = None;
                    for y in 0..8 {
                        for x in 0..8 {
                            if let Some(board_piece) = temp_board[y][x] {
                                if board_piece.color != piece.color {
                                    //It is an enemy piece
                                    let moves_option = self.get_possible_moves(&temp_board, &vec![y, x], true);
                                    match moves_option {
                                        Some(moves) => {
                                            if moves.contains(&king) {
                                                //Check detected!
                                                //Remove that move from movement_positions since it would lead to a check 
                                                if let Some(check_move_index) = movement_positions.iter().position(|pos| pos == &action){
                                                    movement_positions.remove(check_move_index);
                                                }
                                            }
                                        },
                                        None => continue,
                                    }   
                                }
                            }
                        }
                    }
                }
            }
            
            return Some(movement_positions);
        }

        // ==========================
        // QUEEN:

        if piece.piece_type == PieceType::QUEEN {

            let mut movement_positions: Vec<Vec<usize>> = vec![];
            let mut blocking_pieces_positions: Vec<Vec<usize>> = vec![];
            let mut forbidden_positions: Vec<Vec<usize>> = vec![];

            for x in 0..8 {
                for y in 0..8 {
                    //movement:
                    if y+x == position[0]+position[1] || (y as i32)-(x as i32) == ((position[0] as i32) - (position[1] as i32)) {
                        //It is on the same diagonal
                        movement_positions.push(vec![y,x]);

                        match board[y][x] {
                            Some(value) => blocking_pieces_positions.push(vec![y,x]),
                            None => {},   
                        }
                    }
                    else if position[0] == y || position[1] == x {
                        //It is on the same row or column
                        movement_positions.push(vec![y,x]);

                        match board[y][x] {
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
                //diagonal:
                if blocking_position[0]+blocking_position[1] == position[0]+position[1] {
                    //The blocking piece is on the bottom-left to top-right diagonal
                    if blocking_position[0] < position[0] {
                        let mut y = blocking_position[0];
                        for x in blocking_position[1]+1..8 {
                            if y > 0 {
                                y-=1;
                                if let Some(index) = movement_positions.iter().position(|pos| pos == &vec![y, x]) { 
                                    movement_positions.remove(index);
                                }
                            }
                        }
                    }
                    else {
                        let mut x = blocking_position[1];
                        for y in blocking_position[0]+1..8 {
                            if x > 0 {
                                x-=1;
                                if let Some(index) = movement_positions.iter().position(|pos| pos == &vec![y, x]) { 
                                    movement_positions.remove(index);
                                }
                            }
                        }
                    }
                }
                else if (blocking_position[0] as i32)-(blocking_position[1] as i32) == ((position[0] as i32) - (position[1] as i32)) {
                    //The blocking piece is on the top-left to bottom-right diagonal
                    if blocking_position[0] < position[0] {
                        let mut y = (blocking_position[0] as i32)-(blocking_position[1] as i32);
                        for x in 0..blocking_position[1] {
                            if y >= 0 {
                                if let Some(index) = movement_positions.iter().position(|pos| pos == &vec![y as usize, x]) { 
                                    movement_positions.remove(index);
                                }
                            }
                            y+=1;
                        }
                    }
                    else {
                        let mut x = blocking_position[1];
                        for y in blocking_position[0]+1..8 {
                            x+=1;
                            if let Some(index) = movement_positions.iter().position(|pos| pos == &vec![y, x]) { 
                                movement_positions.remove(index);
                            }
                        }
                    }
                }
                //straight:
                else if blocking_position[0] == position[0] {
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
                if let Some(board_piece) = board[pos[0]][pos[1]] {
                    if board_piece.color == piece.color {
                        forbidden_positions.push(pos.clone());
                    }
                }
            }

            //remove forbidden
            movement_positions.retain(|pos| !forbidden_positions.contains(pos));

            //Check if move leads to a check on your king, it shouldn't be allowed in that case
            if !call_is_recursive {
                //find king
                let mut king = vec![10,10]; //Value out of range on purpose
                for y in 0..8 {
                    for x in 0..8 {
                        if let Some(board_piece) = board[y][x] {
                            if board_piece.piece_type == PieceType::KING && board_piece.color == piece.color {
                                king = vec![y, x];
                            }
                        }
                    }
                }
                for action in movement_positions.clone() {
                    let mut temp_board = board.clone();
                    temp_board[action[0]][action[1]] = Some(piece);
                    temp_board[position[0]][position[1]] = None;
                    for y in 0..8 {
                        for x in 0..8 {
                            if let Some(board_piece) = temp_board[y][x] {
                                if board_piece.color != piece.color {
                                    //It is an enemy piece
                                    let moves_option = self.get_possible_moves(&temp_board, &vec![y, x], true);
                                    match moves_option {
                                        Some(moves) => {
                                            if moves.contains(&king) {
                                                //Check detected!
                                                //Remove that move from movement_positions since it would lead to a check 
                                                if let Some(check_move_index) = movement_positions.iter().position(|pos| pos == &action){
                                                    movement_positions.remove(check_move_index);
                                                }
                                            }
                                        },
                                        None => continue,
                                    }   
                                }
                            }
                        }
                    }
                }
            }

            return Some(movement_positions);
        }


        // ==========================
        // KING:

        if piece.piece_type == PieceType::KING {

            let mut movement_positions: Vec<Vec<usize>> = vec![];
            let mut forbidden_positions: Vec<Vec<usize>> = vec![];

            for x in 0..8 {
                for y in 0..8 {
                    //movement:
                    if y+x == position[0]+position[1] || (y as i32)-(x as i32) == ((position[0] as i32) - (position[1] as i32)) {
                        //It is on the same diagonal
                        let dist = (position[1] as i32 - x as i32).abs();
                        if dist == 1 {
                            movement_positions.push(vec![y,x]);
                        }
                    }
                    else if position[0] == y || position[1] == x {
                        //It is on the same row or column
                        let dist = ((position[0] as i32 - y as i32) + (position[1] as i32 - x as i32)).abs();
                        if dist == 1 {
                            movement_positions.push(vec![y,x]);
                        }
                    }
                }
            }

            // Find forbidden positions
            //Shouldn't be able to attack own color
            for pos in &movement_positions {
                if let Some(board_piece) = board[pos[0]][pos[1]] {
                    if board_piece.color == piece.color {
                        forbidden_positions.push(pos.clone());
                    }
                }
            }

            //remove forbidden
            movement_positions.retain(|pos| !forbidden_positions.contains(pos));

            //Check if move leads to a check on your king, it shouldn't be allowed in that case
            if !call_is_recursive {
                for action in movement_positions.clone() {
                    let king = &action;
                    let mut temp_board = board.clone();
                    temp_board[action[0]][action[1]] = Some(piece);
                    temp_board[position[0]][position[1]] = None;
                    for y in 0..8 {
                        for x in 0..8 {
                            if let Some(board_piece) = temp_board[y][x] {
                                if board_piece.color != piece.color {
                                    //It is an enemy piece
                                    let moves_option = self.get_possible_moves(&temp_board, &vec![y, x], true);
                                    match moves_option {
                                        Some(moves) => {
                                            if moves.contains(king) {
                                                //Check detected!
                                                //Remove that move from movement_positions since it would lead to a check 
                                                if let Some(check_move_index) = movement_positions.iter().position(|pos| pos == &action){
                                                    movement_positions.remove(check_move_index);
                                                }
                                                
                                            }
                                        },
                                        None => continue,
                                    }   
                                }
                            }
                        }
                    }
                }
            }

            return Some(movement_positions);
        }
        
        // ==========================
        // KNIGHT:

        if piece.piece_type == PieceType::KNIGHT {

            let mut movement_positions: Vec<Vec<usize>> = vec![];
            let mut blocking_pieces_positions: Vec<Vec<usize>> = vec![];
            let mut forbidden_positions: Vec<Vec<usize>> = vec![];

            for x in 0..8 {
                for y in 0..8 {
                    //movement:
                    let dist_x = (position[1] as i32 - x as i32).abs();
                    let dist_y = (position[0] as i32 - y as i32).abs();
                    if dist_x + dist_y == 3 && dist_x < 3 && dist_y < 3 {
                        movement_positions.push(vec![y,x]);
                    }
                }
            }

            // Find forbidden positions
            //Shouldn't be able to attack own color
            for pos in &movement_positions {
                if let Some(board_piece) = board[pos[0]][pos[1]] {
                    if board_piece.color == piece.color {
                        forbidden_positions.push(pos.clone());
                    }
                }
            }

            //remove forbidden
            movement_positions.retain(|pos| !forbidden_positions.contains(pos));

            //Check if move leads to a check on your king, it shouldn't be allowed in that case
            if !call_is_recursive {
                //find king
                let mut king = vec![10,10]; //Value out of range on purpose
                for y in 0..8 {
                    for x in 0..8 {
                        if let Some(board_piece) = board[y][x] {
                            if board_piece.piece_type == PieceType::KING && board_piece.color == piece.color {
                                king = vec![y, x];
                            }
                        }
                    }
                }
                for action in movement_positions.clone() {
                    let mut temp_board = board.clone();
                    temp_board[action[0]][action[1]] = Some(piece);
                    temp_board[position[0]][position[1]] = None;
                    for y in 0..8 {
                        for x in 0..8 {
                            if let Some(board_piece) = temp_board[y][x] {
                                if board_piece.color != piece.color {
                                    //It is an enemy piece
                                    let moves_option = self.get_possible_moves(&temp_board, &vec![y, x], true);
                                    match moves_option {
                                        Some(moves) => {
                                            if moves.contains(&king) {
                                                //Check detected!
                                                //Remove that move from movement_positions since it would lead to a check 
                                                if let Some(check_move_index) = movement_positions.iter().position(|pos| pos == &action){
                                                    movement_positions.remove(check_move_index);
                                                }
                                            }
                                        },
                                        None => continue,
                                    }   
                                }
                            }
                        }
                    }
                }
            }

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
        
        let mut output: String = "".to_string();

        output += "\n";
        output += "|";
        for i in 0..8 {
            output += "-----";
        }
        output += "|";
        for item in &self.board {
            output += "\n";
            output += "|";
            for item_inner in item {
                output += " ";
                match item_inner {
                    Some(piece) => {
                        output.push(format!("{:?}", piece.color).chars().next().unwrap());
                        output += "-";
                        output.push(format!("{:?}", piece.piece_type).chars().next().unwrap());
                    }, 
                    None => output += "---",
                }
                
                output += " ";
            }
            output += "|";
            
        }
        output += "\n";
        output += "|";
        for i in 0..8 {
            output += "-----";
        }
        output += "|";

        output += "\n\n\n";


        write!(f, "{}", output)
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use crate::Colour;
    use crate::Piece;
    use crate::PieceType;

    use super::Game;
    use super::GameState;

    //Auxilirary functions
    fn print_board(game: &Game){
        //Prints the board:
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

        let legal_moves = game.get_possible_moves(&game.board, position, false);
        //println!("{:?}", legal_moves);


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
    fn white_active_color_after_init() {
        let game = Game::new();
        assert_eq!(game.active_colour, Colour::White);
    }

    #[test]
    fn set_promotion_type() {
        let mut game = Game::new();
        game.set_promotion(PieceType::BISHOP);
        game.set_promotion(PieceType::KING);
        assert_eq!(game.promotion_type, PieceType::BISHOP);
    }

    #[test]
    fn possible_moves_rook() {
        let mut game = Game::new();
        game.board[4][4] = Some(Piece {
            color: Colour::Black,
            piece_type: PieceType::ROOK
        });
        //print_board_moves(&game, &vec![4,4]);
        let moves = game.get_possible_moves(&game.board, &vec![4,4], false);
        //println!("{:?}", moves.unwrap());
        assert_eq!(moves.unwrap(), [[4, 0], [4, 1], [4, 2], [4, 3], [2, 4], [3, 4], [5, 4], [6, 4], [4, 5], [4, 6], [4, 7]]);
    }
    #[test]
    fn possible_moves_pawn() {
        let mut game = Game::new();
        game.board[5][3] = Some(Piece {
            color: Colour::Black,
            piece_type: PieceType::ROOK
        });
        //print_board_moves(&game, &vec![6,2]);
        let moves = game.get_possible_moves(&game.board, &vec![6,2], false);
        //println!("{:?}", moves.unwrap());
        assert_eq!(moves.unwrap(), [[4, 2], [5, 2], [5, 3]]);
    }
    #[test]
    fn possible_moves_bishop() {
        let mut game = Game::new();
        game.board[5][6] = Some(Piece {
            color: Colour::Black,
            piece_type: PieceType::BISHOP
        });
        //print_board_moves(&game, &vec![5,6]);
        let moves = game.get_possible_moves(&game.board, &vec![5,6], false);
        //println!("{:?}", moves.unwrap());
        assert_eq!(moves.unwrap(), [[2, 3], [3, 4], [4, 5], [6, 5], [4, 7], [6, 7]]);
    }
    

    #[test]
    fn move_pieces() {
        let mut game = Game::new();
        println!("{:?}", &game);
        println!("{:?}", game.make_move(vec![6,4], vec![4,4]));
        println!("{:?}", &game);
        println!("{:?}", game.make_move(vec![1,3], vec![3,3]));
        println!("{:?}", &game);
        println!("{:?}", game.make_move(vec![7,3], vec![4,6]));
        println!("{:?}", &game);
        println!("{:?}", game.make_move(vec![0,2], vec![4,6]));
        println!("{:?}", &game);
        assert_eq!(game.board[4][6], Some(Piece {color: Colour::Black, piece_type: PieceType::BISHOP}));
    }

    
}