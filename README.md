# emriks-chess

<h2>Game</h2>
Has board: Vec<Vec<Option<Piece>>>,
promotion_type: PieceType,

<h2>Enums and Structs</h2>
Piece: color, piece_type
PieceType: PAWN, ROOK, BISHOP...

<h2>Vec<usize> instead of String</h2>
To store positions I've used a Vec with one y value and one x value instead of a String like B4 for example. 0,0 is in the top left corner of the board, 7,7 is in the bottom right corner.

<h2>Make move function</h2>
I changed so that it doesn't take Strings as parameters but rather Vec with the cordinates in it (x,y).

<h2>Get possible moves function</h2>
I've changed so that it takes &Vec<usize> as param and returns Option<Vec<Vec<usize>>>.

<h2>Y is [0] and X is [1]</h2>
Didn't really intend on doing it like this but when I noticed, it was to late to change it to x,y. So y is first and x second. 

<h2>set_promotion()</h2>
I changed it from piece: String to piece: PieceType.

