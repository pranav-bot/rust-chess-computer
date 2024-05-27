//TODO: A way to represnt the state of the game (piece posiotions, whose turn, who can castle ...)
//TODO: A way to generate legal moves
//TODO: A way to search legal moves
//TODO: A way to select the "best" moves

type PiecePosition = u64;

fn bit_position(bit: PiecePosition) -> Result<String, String>{
    if bit == 0 {
        return Err("No piece present".to_string());
    }
    else {
        let onebit_index = bit_scan(bit);
        return Ok(index_to_position(onebit_index));
    }
}

static COL_MAP: [char;8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

fn index_to_position(index: usize) -> String {
    let column = index % 8;
    let row = (index/8)+1;
    return format!("{}{}", COL_MAP[column], row);
}

fn bit_scan(mut bit: u64) -> usize {
    let mut leading_zeroes = 0;
    while bit & 1 == 0 {
        bit >>= 1;
        leading_zeroes += 1;
    }
    return leading_zeroes;
}

fn main() {
    for i in 0..64 {
        println!("{} -> {}", i, index_to_position(i));
    }
    for i in 0..10 {
        println!("{} -> {}", i, 1<<i)
    }
}


