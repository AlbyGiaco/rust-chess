use std::clone::Clone;
fn main() {

    struct ChessPiece {
        name: String,
        position: (i32, i32),
    }
    impl Clone for ChessPiece {
        fn clone(&self) -> ChessPiece {
            ChessPiece {
                name: self.name.clone(),
                position: self.position.clone(),
            }
        }
    }
    struct BoardRow {
        row: Vec<Option<ChessPiece>>
        //(Option<ChessPiece>, Option<ChessPiece>, Option<ChessPiece>, Option<ChessPiece>, Option<ChessPiece>, Option<ChessPiece>, Option<ChessPiece>, Option<ChessPiece>)
    }
    struct Board {
        boardColumn: [BoardRow;8]
        //(BoardRow, BoardRow, BoardRow, BoardRow, BoardRow, BoardRow, BoardRow, BoardRow)
    }

    impl Board {
        fn new() -> Board {
            Board {
                boardColumn: (
                    BoardRow {
                        row: [Some(ChessPiece {name: "Rook".to_string(), position: (0,0)}), Some(ChessPiece {name: "Knight".to_string(), position: (0,1)}), Some(ChessPiece {name: "Bishop".to_string(), position: (0,2)}), Some(ChessPiece {name: "Queen".to_string(), position: (0,3)}), Some(ChessPiece {name: "King".to_string(), position: (0,4)}), Some(ChessPiece {name: "Bishop".to_string(), position: (0,5)}), Some(ChessPiece {name: "Knight".to_string(), position: (0,6)}), Some(ChessPiece {name: "Rook".to_string(), position: (0,7)})].to_vec()
                    },
                    BoardRow {
                        row: [Some(ChessPiece {name: "Pawn".to_string(), position: (1,0)}), Some(ChessPiece {name: "Pawn".to_string(), position: (1,1)}), Some(ChessPiece {name: "Pawn".to_string(), position: (1,2)}), Some(ChessPiece {name: "Pawn".to_string(), position: (1,3)}), Some(ChessPiece {name: "Pawn".to_string(), position: (1,4)}), Some(ChessPiece {name: "Pawn".to_string(), position: (1,5)}), Some(ChessPiece {name: "Pawn".to_string(), position: (1,6)}), Some(ChessPiece {name: "Pawn".to_string(), position: (1,7)})].to_vec()
                    },
                    BoardRow {
                        row: [None, None, None, None, None, None, None, None]
                    },
                    BoardRow {
                        row: [None, None, None, None, None, None, None, None]
                    },
                    BoardRow {
                        row: [None, None, None, None, None, None, None, None]
                    },
                    BoardRow {
                        row: [None, None, None, None, None, None, None, None]
                    },
                    BoardRow {
                        row: [Some(ChessPiece {name: "Pawn".to_string(), position: (6,0)}), Some(ChessPiece {name: "Pawn".to_string(), position: (6,1)}), Some(ChessPiece {name: "Pawn".to_string(), position: (6,2)}), Some(ChessPiece {name: "Pawn".to_string(), position: (6,3)}), Some(ChessPiece {name: "Pawn".to_string(), position: (6,4)}), Some(ChessPiece {name: "Pawn".to_string(), position: (6,5)}), Some(ChessPiece {name: "Pawn".to_string(), position: (6,6)}), Some(ChessPiece {name: "Pawn".to_string(), position: (6,7)})].to_vec()
                    },
                    BoardRow {
                        row: [Some(ChessPiece {name: "Rook".to_string(), position: (7,0)}), Some(ChessPiece {name: "Knight".to_string(), position: (7,1)}), Some(ChessPiece {name: "Bishop".to_string(), position: (7,2)}), Some(ChessPiece {name: "Queen".to_string(), position: (7,3)}), Some(ChessPiece {name: "King".to_string(), position: (7,4)}), Some(ChessPiece {name: "Bishop".to_string(), position: (7,5)}), Some(ChessPiece {name: "Knight".to_string(), position: (7,6)}), Some(ChessPiece {name: "Rook".to_string(), position: (7,7)})].to_vec()
                    
                    }
                )
                
            }
        }
    }

    let mut board = Board::new();
    for i in 0..8 {
        for j in 0..8 {
            
            match board.boardColumn[i]row[j] {
                Some(ref piece) => println!("{} at ({},{})", piece.name, piece.position.0, piece.position.1),
                None => println!("None"),
            }
        }
    }


}