use crate::utils::{print_bitboard_to_string, BitBoard, set_bit_knight, print_bitboard};
pub struct KnightAttacks(Vec<BitBoard>);

impl KnightAttacks {
    fn initialize() -> Self {
        let mut attacks = vec![];

        for row in 1..=8 {
            for col in 1..=8 {
                let attacks_from_this_square = knight_attacks(row, col);
                attacks.push(attacks_from_this_square);
            }
        } 
        Self(attacks)
    }
}

fn knight_attacks(row: i32, col: i32) -> BitBoard {
    //row -2, col -1
    //row-2, col+1
    //row-1, col-2
    //row-1, col+2
    //..
    let attack_pairs = [(1,2), (1,-2), (-1,2), (-1,-2), (2,1), (2,-1), (-2,1), (-2,-1)];
    let mut bitboard = 0;
    for (r,c) in attack_pairs {
        bitboard |= set_bit_knight(row+r, col+c);
    }
    bitboard
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knight_attacks_can_initialize() {
        let knight_attacks = KnightAttacks::initialize();

    }

    #[test]
    fn print_knight_attacks() {
        let knight_attacks = KnightAttacks::initialize();
        print_bitboard(knight_attacks.0[0], Some(0));
        print_bitboard(knight_attacks.0[40], Some(40));
        print_bitboard(knight_attacks.0[17], Some(17));
        print_bitboard(knight_attacks.0[55], Some(55));
    }
}