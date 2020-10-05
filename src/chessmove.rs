use crate::pieces::piece;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ChessMove {
    pub from: (u8, u8),
    pub to: (u8, u8),
    pub promotion: Option<piece::PromotionPiece>,
}
