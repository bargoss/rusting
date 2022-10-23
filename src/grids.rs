pub mod board_logic {

    #[derive(Clone, Copy)]
    pub enum CellState {
        Empty,
        Occupied,
        Hit
    }
    #[derive(Clone, Copy)]
    pub enum Player{
        Player1,
        Player2
    }


    #[derive(Copy, Clone)]
    pub struct Grid{
        pub state : CellState,
        pub player : Player
    }


    //pub struct Coordinate{
    //    pub x : i32,
    //    pub y : i32
    //}

    pub struct Board{
        pub grids : [Grid; 100],
    }
    impl Board{


        pub fn new() -> Board{
            Board{
                grids : [Grid{state : CellState::Empty, player : Player::Player1}; 100]
            }
        }

        // access by x and y
        pub fn get_grid(&self, x : usize, y : usize) -> &Grid{
            &self.grids[x + y * 10]
        }
        pub fn set_grid(&mut self, x : usize, y : usize, value : Grid){
            let index = x + y * 10;
            self.grids[index] = value;
        }
        pub fn print_board(&self){
            for boardI in 0..12{
                for boardJ in 0..12{
                    // draw frame
                    if boardI == 0 || boardI == 11 || boardJ == 0 || boardJ == 11{
                        print!("**");
                        continue;
                    }
                    let i = boardI - 1;
                    let j = boardJ - 1;


                    let cell_state = self.get_grid(i, j).state;
                    match cell_state{
                        CellState::Empty => print!("--"),
                        CellState::Occupied => match self.get_grid(i, j).player{
                            // if player 1, print P1 blue color, if player 2, print P2 red color
                            Player::Player1 => print!("\x1b[34mP1\x1b[0m"),
                            Player::Player2 => print!("\x1b[31mP2\x1b[0m")
                        },
                        CellState::Hit => match self.get_grid(i, j).player{
                            // same but print ..
                            Player::Player1 => print!("\x1b[34m..\x1b[0m"),
                            Player::Player2 => print!("\x1b[31m..\x1b[0m")
                        }
                    }
                }
                println!("");
            }
        }
        pub fn place_ship(x: usize, y: usize, ship_len: u8, board: &mut Board, player: Player, rotation: i32){
            let mut x = x;
            let mut y = y;

            for i in 0..ship_len {
                if rotation == 0{
                    board.set_grid(x, y, Grid{state: CellState::Occupied, player });
                    x += 1;
                }
                else if rotation == 1{
                    board.set_grid(x, y, Grid{state: CellState::Occupied, player });
                    y += 1;
                }
                else if rotation == 2{
                    board.set_grid(x, y, Grid{state: CellState::Occupied, player });
                    x -= 1;
                }
                else if rotation == 3{
                    board.set_grid(x, y, Grid{state: CellState::Occupied, player });
                    y -= 1;
                }
            }
        }
    }

    pub fn grid_experiment() {
        let mut board = Board::new();

        Board::place_ship(0, 0, 5, &mut board, Player::Player1, 0);
        Board::place_ship(3, 2, 3, &mut board, Player::Player2, 1);
        Board::place_ship(6, 6, 3, &mut board, Player::Player1, 1);
        Board::place_ship(9, 9, 1, &mut board, Player::Player2, 0);


        board.print_board();

        // println!("Hello, world!");
    }
}