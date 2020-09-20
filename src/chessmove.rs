#[derive(Debug, PartialEq, Eq)]
pub struct ChessMove {
    pub source_file: u8,
    pub source_rank: u8,
    pub target_file: u8,
    pub target_rank: u8,
}
