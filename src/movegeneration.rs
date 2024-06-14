use crate::{utils::{_bit_scan, extract_bits, print_bitboard}, Color, Game, Piece, PieceType};

fn generate_moves(game: &Game) -> Vec<Game> {
    let mut new_positions = vec![];
    for piece in &game.pieces {
        if piece.color == game.active_color {
            match &piece.piece_type {
                PieceType::Knight => {
               let positions = generate_knight_moves(&piece, &game);
               new_positions.extend(positions);
                },
                _ => panic!("Piece type not yet supported")
            }
        }
    }
    new_positions
}

fn generate_knight_moves(piece: &Piece, game: &Game) -> Vec<Game> {
    let mut attacks = game.knight_attacks.0[_bit_scan(piece.position)];
    
    let own_occupancy = match piece.color {
        Color::White => game.white_occupancy,
        Color::Black => game.black_occupancy
    };
    attacks &= !own_occupancy;
    let potential_moves = extract_bits(attacks);
    print_bitboard(attacks, None);
    let enemy_occupancy = match piece.color {
        Color::White => game.white_occupancy,
        Color::Black=> game.black_occupancy
    };
    for pmove in potential_moves {
        let mut new_position = (*game).clone();
        new_position.move_piece(piece.position, pmove);

    } 

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_knight_moves_test() {
        let not_alot = "8/8/8/4N3/2N5/8/8/8 w - - 0 1";
        let game = Game::read_fen(&not_alot);
        println!("{}", game.to_string());
        let moves = generate_knight_moves(&game.pieces[0], &game);
        //println!("{:?}", moves);
        print!("hello");
    }
}