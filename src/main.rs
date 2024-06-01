//DONE: A way to represnt the state of the game (piece posiotions, whose turn, who can castle ...) (FEN Strings)
//TODO: A way to generate legal moves
//TODO: A way to search legal moves
//TODO: A way to select the "best" moves
mod game;
mod utils;
mod rayattacks;
mod knightattacks;
mod pawnattacks;
mod movegeneration;
use game::*;
fn main() {
    let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let not_alot = "5k2/8/8/4N3/8/8/8/5K2 w - - 0 1";
    let not_alot2 = "5k2/8/8/4N3/2N5/8/8/5K2 w - - 0 1";
    let game = Game::read_fen(&not_alot2);
    println!("{}", game.to_string());
    println!("{:?} {:?} {}", game.active_color, game.en_passant, game.fullmove_clock);
}


