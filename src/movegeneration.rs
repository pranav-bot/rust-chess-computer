use crate::{utils::{_bit_scan, extract_bits}, Color, Game, Piece, PieceType, Position};

fn generate_moves(game: &Game) -> Vec<Position> {
    let mut new_positions = vec![];
    for piece in &game.position.pieces {
        if piece.color == game.position.active_color {
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

fn generate_knight_moves(piece: &Piece, game: &Game) -> Vec<Position> {
    let mut attacks = game.knight_attacks.0[_bit_scan(piece.position)];
    let position = &game.position;
    
    let own_occupancy = match piece.color {
        Color::White => position.white_occupancy,
        Color::Black => position.black_occupancy
    };
    attacks &= !own_occupancy;
    let potential_moves = extract_bits(attacks);
    let mut new_positions = vec![];
    for pmove in potential_moves {
        let mut new_position = (*game).position.clone();
        new_position.move_piece(piece.position, pmove);
        new_positions.push(new_position);

    } 

    new_positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_knight_moves_test() {
        let not_alot = "8/8/8/4N3/2N5/8/8/8 w - - 0 1";
        let game = Game::read_fen(&not_alot);
        println!("{}", game.position.to_string());
        let moves = generate_knight_moves(&game.position.pieces[0], &game);
        let new_positions = [19,21, 30, 42, 46, 51, 53];
        assert_eq!(moves.len(), 7);
        for v in moves {
            assert_eq!(v.pieces.len(), 2);
            let piece = &v.pieces[0];
            let index = _bit_scan(piece.position);
            assert!(new_positions.contains(&index));
        }
    }
}