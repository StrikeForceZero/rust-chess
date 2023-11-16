use bevy::prelude::*;
use crate::gui::systems::placed::PlacedPiece;
use crate::piece::chess_piece::ChessPiece;

#[derive(Resource)]
pub struct SpriteMapResource {
    white_king: Handle<Image>,
    white_queen: Handle<Image>,
    white_rook: Handle<Image>,
    white_bishop: Handle<Image>,
    white_knight: Handle<Image>,
    white_pawn: Handle<Image>,
    black_king: Handle<Image>,
    black_queen: Handle<Image>,
    black_rook: Handle<Image>,
    black_bishop: Handle<Image>,
    black_knight: Handle<Image>,
    black_pawn: Handle<Image>,
}

impl SpriteMapResource {
    fn path_from_piece(piece: ChessPiece) -> String {
        format!("sprites/{}/{}.png", piece.as_color(), piece.as_piece())
    }
    pub fn new(asset_server: &Res<AssetServer>) -> SpriteMapResource {
        SpriteMapResource {
            white_king: asset_server.load(Self::path_from_piece(ChessPiece::WhiteKing)),
            white_queen: asset_server.load(Self::path_from_piece(ChessPiece::WhiteQueen)),
            white_rook: asset_server.load(Self::path_from_piece(ChessPiece::WhiteRook)),
            white_bishop: asset_server.load(Self::path_from_piece(ChessPiece::WhiteBishop)),
            white_knight: asset_server.load(Self::path_from_piece(ChessPiece::WhiteKnight)),
            white_pawn: asset_server.load(Self::path_from_piece(ChessPiece::WhitePawn)),
            black_king: asset_server.load(Self::path_from_piece(ChessPiece::BlackKing)),
            black_queen: asset_server.load(Self::path_from_piece(ChessPiece::BlackQueen)),
            black_rook: asset_server.load(Self::path_from_piece(ChessPiece::BlackRook)),
            black_bishop: asset_server.load(Self::path_from_piece(ChessPiece::BlackBishop)),
            black_knight: asset_server.load(Self::path_from_piece(ChessPiece::BlackKnight)),
            black_pawn: asset_server.load(Self::path_from_piece(ChessPiece::BlackPawn)),
        }
    }
    pub fn image_from_piece(&self, piece: PlacedPiece) -> Option<Handle<Image>> {
        use crate::gui::systems::placed::PlacedPiece::*;
        use crate::piece::piece::Piece::*;
        let handle = match piece {
            Empty => return None,
            White(King) => &self.white_king,
            White(Queen) => &self.white_queen,
            White(Rook) => &self.white_rook,
            White(Bishop) => &self.white_bishop,
            White(Knight) => &self.white_knight,
            White(Pawn) => &self.white_pawn,
            Black(King) => &self.black_king,
            Black(Queen) => &self.black_queen,
            Black(Rook) => &self.black_rook,
            Black(Bishop) => &self.black_bishop,
            Black(Knight) => &self.black_knight,
            Black(Pawn) => &self.black_pawn,
        };
        Some((*handle).clone())
    }
}
