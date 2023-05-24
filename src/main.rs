use std::clone::Clone;
use std::io;
fn main() {
    let mut stdin = io::stdin();
    #[derive(Debug, Clone, Copy)]
    enum ChessType {
        Rook,
        Knight,
        Bishop,
        King,
        Queen,
        Pawn
    }

    #[derive(Debug, Clone)]
    struct ChessPiece {
        name: String,
        chessType: ChessType,
        position: (i32, i32),
    }

    struct BoardRow {
        row: Vec<Option<ChessPiece>>
        //row: [ChessPiece;8]
        //(Option<ChessPiece>, Option<ChessPiece>, Option<ChessPiece>, Option<ChessPiece>, Option<ChessPiece>, Option<ChessPiece>, Option<ChessPiece>, Option<ChessPiece>)
    }
    struct Board {
        boardColumn: [BoardRow;8]
        //(BoardRow, BoardRow, BoardRow, BoardRow, BoardRow, BoardRow, BoardRow, BoardRow)
    }

    impl Board {
        fn new() -> Board {
            Board {
                boardColumn: [
                    BoardRow {
                        row: [Some(ChessPiece {chessType: ChessType::Rook, name: "Rook".to_string(), position: (0,0)}), Some(ChessPiece {chessType: ChessType::Knight, name: "Knight".to_string(), position: (0,1)}), Some(ChessPiece {chessType: ChessType::Bishop, name: "Bishop".to_string(), position: (0,2)}), Some(ChessPiece {chessType: ChessType::King, name: "King".to_string(), position: (0,3)}), Some(ChessPiece {chessType: ChessType::Queen, name: "Queen".to_string(), position: (0,4)}), Some(ChessPiece {chessType: ChessType::Bishop, name: "Bishop".to_string(), position: (0,5)}), Some(ChessPiece {chessType: ChessType::Knight, name: "Knight".to_string(), position: (0,6)}), Some(ChessPiece {chessType: ChessType::Rook, name: "Rook".to_string(), position: (0,7)})].to_vec()
                    },
                    BoardRow {
                        row: [Some(ChessPiece {chessType: ChessType::Pawn,name: "Pawn".to_string(), position: (1,0)}), Some(ChessPiece {chessType: ChessType::Pawn,name: "Pawn".to_string(), position: (1,1)}), Some(ChessPiece {chessType: ChessType::Pawn,name: "Pawn".to_string(), position: (1,2)}), Some(ChessPiece {chessType: ChessType::Pawn,name: "Pawn".to_string(), position: (1,3)}), Some(ChessPiece {chessType: ChessType::Pawn,name: "Pawn".to_string(), position: (1,4)}), Some(ChessPiece {chessType: ChessType::Pawn,name: "Pawn".to_string(), position: (1,5)}), Some(ChessPiece {chessType: ChessType::Pawn,name: "Pawn".to_string(), position: (1,6)}), Some(ChessPiece {chessType: ChessType::Pawn,name: "Pawn".to_string(), position: (1,7)})].to_vec()
                    },
                    BoardRow {
                        row: vec![None, None, None, None, None, None, None, None]
                    },
                    BoardRow {
                        row: [None, None, None, None, None, None, None, None].to_vec()
                    },
                    BoardRow {
                        row: [None, None, None, None, None, None, None, None].to_vec()
                    },
                    BoardRow {
                        row: [None, None, None, None, None, None, None, None].to_vec()
                    },
                    BoardRow {
                        row: [Some(ChessPiece {chessType: ChessType::Pawn,name: "Pawn".to_string(), position: (6,0)}), Some(ChessPiece {chessType: ChessType::Pawn,name: "Pawn".to_string(), position: (6,1)}), Some(ChessPiece {chessType: ChessType::Pawn,name: "Pawn".to_string(), position: (6,2)}), Some(ChessPiece {chessType: ChessType::Pawn,name: "Pawn".to_string(), position: (6,3)}), Some(ChessPiece {chessType: ChessType::Pawn,name: "Pawn".to_string(), position: (6,4)}), Some(ChessPiece {chessType: ChessType::Pawn,name: "Pawn".to_string(), position: (6,5)}), Some(ChessPiece {chessType: ChessType::Pawn,name: "Pawn".to_string(), position: (6,6)}), Some(ChessPiece {chessType: ChessType::Pawn,name: "Pawn".to_string(), position: (6,7)})].to_vec()
                    },
                    BoardRow {
                        row: [Some(ChessPiece {chessType: ChessType::Rook,name: "Rook".to_string(), position: (7,0)}), Some(ChessPiece {chessType: ChessType::Knight,name: "Knight".to_string(), position: (7,1)}), Some(ChessPiece {chessType: ChessType::Bishop,name: "Bishop".to_string(), position: (7,2)}), Some(ChessPiece {chessType: ChessType::Queen,name: "Queen".to_string(), position: (7,3)}), Some(ChessPiece {chessType: ChessType::King,name: "King".to_string(), position: (7,4)}), Some(ChessPiece {chessType: ChessType::Bishop,name: "Bishop".to_string(), position: (7,5)}), Some(ChessPiece {chessType: ChessType::Knight,name: "Knight".to_string(), position: (7,6)}), Some(ChessPiece {chessType: ChessType::Rook,name: "Rook".to_string(), position: (7,7)})].to_vec()
                    
                    }
                ]
            }
        }

        fn move_pawn(&mut self, piece: ChessPiece) {
            self.boardColumn[piece.position.0 as usize].row[piece.position.1 as usize] = None;
            self.boardColumn[piece.position.0 as usize + 1 as usize].row[piece.position.1 as usize + 1 as usize] = Some(piece);
        }

        // fn move_piece(self, piece: ChessPiece) {
        //     self.boardColumn[piece.position.0 as usize].row[piece.position.1 as usize] = None;
        // }

    }

    let mut board = Board::new();
    for i in 0..8 {
        for j in 0..8 {
            
            match board.boardColumn[i].row[j] {
                Some(ref piece) => {
                    print!("{} ", piece.name.chars().next().unwrap(),);
                }
                None => print!("X "),
            }
            
        }
        println!("\n")
    }
    let mut user_input = String::new();
    let mut chessPieceTemp = ChessPiece {chessType: ChessType::Pawn, name: "Queen".to_string(), position: (0,0)};
     stdin.read_line(&mut user_input);
     board.move_pawn(board.boardColumn[1].row[0].clone().unwrap());
}