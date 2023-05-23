use std::clone::Clone;
fn main() {
    // create a struct to memorize the position of a chess board
    #[derive(Debug, Clone, Copy)]
    struct position {
        x: i32,
        y: i32,
    }

    #[derive(Debug, Clone, Copy)]
    enum color {
        Black,
        White
    } 

    #[derive(Debug, Clone, Copy)]
    enum chessType {
        Pawn,
        Rook,
        Knight,
        Bishop,
        Queen,
        King
    }

    struct chessPiece  {
        Type: chessType,
        Color: color,
        Position: position,
        alive: bool,
    }

    impl Clone for chessPiece {
        fn clone(&self) -> chessPiece {
            chessPiece {
            Type: self.Type,
            Color: self.Color,
            Position: self.Position,
            alive: self.alive,
         }
        }
    }

  
    // there will be 8x8 Position on the board

    // initialize the board with 32 chess pieces
    // 16 white chess pieces and 16 black chess pieces
    // 8 pawns, 2 rooks, 2 knights, 2 bishops, 1 queen, 1 king
    // the white chess pieces are on the top of the board
    // the black chess pieces are on the bottom of the board
    let mut chessPieces: Vec<chessPiece> = Vec::new();
    for i in 1..9 {
        let mut whitePawn = chessPiece {
            Type: chessType::Pawn,
            Color: color::White,
            Position: position {
                x: i,
                y: 2,
            },
            alive: true,
        };
        let mut blackPawn = chessPiece {
            Type: chessType::Pawn,
            Color: color::Black,
            Position: position {
                x: i,
                y: 7,
            },
            alive: true,
        };
        chessPieces.push(whitePawn);
    }
    for i in 1..2 {
        let mut whiteRook1 = chessPiece {
            Type: chessType::Rook,
            Color: color::White,
            Position: position {
                x: 1,
                y: 1,
            },
            alive: true,
        };
        let mut whiteRook2 = chessPiece {
            Type: chessType::Rook,
            Color: color::White,
            Position: position {
                x: 8,
                y: 1,
            },
            alive: true,
        };
        let mut blackRook1 = chessPiece {
            Type: chessType::Rook,
            Color: color::Black,
            Position: position {
                x: 2,
                y: 8,
            },
            alive: true,
        };
        let mut blackRook2 = chessPiece {
            Type: chessType::Rook,
            Color: color::Black,
            Position: position {
                x: 7,
                y: 8,
            },
            alive: true,
        };
        chessPieces.push(whiteRook1);
        chessPieces.push(whiteRook2);
        chessPieces.push(blackRook1);
        chessPieces.push(blackRook2);

        let mut whiteKnight1 = chessPiece {
            Type: chessType::Knight,
            Color: color::White,
            Position: position {
                x: 3,
                y: 1,
            },
            alive: true,
        };
        let mut whiteKnight2 = chessPiece {
            Type: chessType::Knight,
            Color: color::White,
            Position: position {
                x: 6,
                y: 1,
            },
            alive: true,
        };
        let mut blackKnight1 = chessPiece {
            Type: chessType::Knight,
            Color: color::Black,
            Position: position {
                x: 3,
                y: 8,
            },
            alive: true,
        };
        let mut blackKnight2 = chessPiece {
            Type: chessType::Knight,
            Color: color::Black,
            Position: position {
                x: 6,
                y: 8,
            },
            alive: true,
        };
        chessPieces.push(whiteKnight1);
        chessPieces.push(whiteKnight2);
        chessPieces.push(blackKnight1);
        chessPieces.push(blackKnight2);

        let mut whiteBishop1 = chessPiece {
            Type: chessType::Bishop,
            Color: color::White,
            Position: position {
                x: 4,
                y: 1,
            },
            alive: true,
        };
        let mut whiteBishop2 = chessPiece {
            Type: chessType::Bishop,
            Color: color::White,
            Position: position {
                x: 5,
                y: 1,
            },
            alive: true,
        };
        let mut blackBishop1 = chessPiece {
            Type: chessType::Bishop,
            Color: color::Black,
            Position: position {
                x: 4,
                y: 8,
            },
            alive: true,
        }; 
        let mut blackBishop2 = chessPiece {
            Type: chessType::Bishop,
            Color: color::Black,
            Position: position {
                x: 5,
                y: 8,
            },
            alive: true,
        };
        chessPieces.push(whiteBishop1);
        chessPieces.push(whiteBishop2);
        chessPieces.push(blackBishop1);
        chessPieces.push(blackBishop2);

        let mut whiteQueen = chessPiece {
            Type: chessType::Queen,
            Color: color::White,
            Position: position {
                x: i,
                y: 1,
            },
            alive: true,
        };
        let mut blackQueen = chessPiece {
            Type: chessType::Queen,
            Color: color::Black,
            Position: position {
                x: i,
                y: 8,
            },
            alive: true,
        };
        chessPieces.push(whiteQueen);
        chessPieces.push(blackQueen);

        let mut whiteKing = chessPiece {
            Type: chessType::King,
            Color: color::White,
            Position: position {
                x: i,
                y: 1,
            },
            alive: true,
        };
        let mut blackKing = chessPiece {
            Type: chessType::King,
            Color: color::Black,
            Position: position {
                x: i,
                y: 8,
            },
            alive: true,
        };
        chessPieces.push(whiteKing);
        chessPieces.push(blackKing);
    }

    struct Knight {
        Color: color,
        Position: position,
        alive: bool,
    }

    impl Knight{
        fn movePiece(&mut self, newPosition: position) {
            if(newPosition.x < 1 || newPosition.x > 8 || newPosition.y < 1 || newPosition.y > 8){
                return;
            }
            self.Position = newPosition;
        }
    }

struct Board {
    knights: (Knight, Knight, Knight, Knight)

}

impl Board {
    fn new() -> Board {
        Board {
            knights: (Knight {
                Color: color::White,
                Position: position {
                    x: 1,
                    y: 1,
                },
                alive: true,
            },
            Knight {
                Color: color::White,
                Position: position {
                    x: 1,
                    y: 1,
                },
                alive: true,
            },
            Knight {
                Color: color::Black,
                Position: position {
                    x: 1,
                    y: 1,
                },
                alive: true,
            },
            Knight {
                Color: color::Black,
                Position: position {
                    x: 1,
                    y: 1,
                },
                alive: true,
            })
        }
    }
}
    // generate a the board on the console printing 9 line and 9 column and see if the position is on the board
    // if the position is on the board, print the chess piece on the board
    // if the position is not on the board, print nothing
    println!("a b c d e f g h");
    for i in 1..9 {  // cicle 8 row
        let mut row: Vec<chessPiece> = Vec::new();
       
        
        for j in 1..9 {  // cicle 8 column
           let pieceInPosition: Option<&chessPiece> = chessPieces.iter()
           .find(|&p| p.Position.x == j && p.Position.y == i);
           match pieceInPosition {

            Some(x) => row.push(x.clone()),
            None    => {}
        }
        }


        match row.get(0){
            Some(x) => println!("{:?} ", x.Type),
            None    => {},
        }
      
    }





    println!("Hello, world!");
}
