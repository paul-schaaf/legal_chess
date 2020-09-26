pub mod attack;
mod chessmove;
mod color;
mod pieces;

fn legal_moves(board: [[&str; 8]; 8]) -> Vec<chessmove::ChessMove> {
    vec![chessmove::ChessMove {
        source_rank: 2,
        source_file: 2,
        target_rank: 4,
        target_file: 2,
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legal_moves_ok() {
        let moves = [["p"; 8]; 8];
        let result = vec![chessmove::ChessMove {
            source_rank: 2,
            source_file: 2,
            target_rank: 4,
            target_file: 2,
        }];

        assert_eq!(result, legal_moves(moves));
    }
}
