use std::time::Duration;
use std::thread;
use std::fmt;

// TODO: tokenization for convinient code alignment (for picture)
// TODO: code alignment as picture's pattern (conway's glider basically)

const SIZE: usize = 10;
const DEBUG: bool = false;

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Dead, Alive,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Dead  => write!(f, "{}", if DEBUG { "." } else { "  " }),
            Cell::Alive => write!(f, "{}", if DEBUG { "#" } else { "██" }),
        }
    }
}

struct Board {
    cells: [[Cell; SIZE]; SIZE],
    generation: usize,
}

impl Board {
    #[allow(dead_code)]
    fn empty() -> Self {
        Self {
            cells: [[Cell::Dead; SIZE]; SIZE],
            generation: 0,
        }
    }

    fn with_glider() -> Self {
        let mut cells = [[Cell::Dead; SIZE]; SIZE];
        
        cells[0][1] = Cell::Alive;
        cells[1][2] = Cell::Alive;
        cells[2][0] = Cell::Alive;
        cells[2][1] = Cell::Alive;
        cells[2][2] = Cell::Alive;

        Self { cells, generation: 0 }
    }

    fn display(&self) {
        for row in 0..SIZE {
            for col in 0..SIZE {
                print!("{}", self.cells[row][col]);
            }
            println!();
        }
        self.display_stats();
    }

    fn display_stats(&self) {
        println!("Population: {}", self.count_population());
        println!("Generation: {}", self.generation);
    }

    fn count_neighboors(&self, row: usize, col: usize) -> usize {
        let mut n: usize = 0;
        for drow in -1..=1 {
            for dcol in -1..=1 {
                if drow == 0 && dcol == 0 { continue; }
                let new_col = (((col as i32 + dcol) + SIZE as i32) % SIZE as i32) as usize;
                let new_row = (((row as i32 + drow) + SIZE as i32) % SIZE as i32) as usize;

                if self.cells[new_row][new_col] == Cell::Alive { n += 1 }
            }
        }
        return n;
    }

    fn next(&mut self) {
        let mut next = [[Cell::Dead; SIZE]; SIZE];

        for row in 0..SIZE {
            for col in 0..SIZE {
                let n = self.count_neighboors(row, col);
                match self.cells[row][col] {
                    Cell::Dead => {
                        if n == 3 {
                            next[row][col] = Cell::Alive;
                        }
                    },
                    Cell::Alive => {
                        if n == 2 || n == 3 {
                            next[row][col] = Cell::Alive;
                        }
                    },
                }
            }
        }

        self.cells = next;
        self.generation += 1;
    }

    fn count_population(&self) -> usize {
        let mut ppl: usize = 0;
        for row in 0..SIZE {
            for col in 0..SIZE {
                if self.cells[row][col] == Cell::Alive { ppl += 1 }
            }
        }
        return ppl;
    }
}

fn main() {
    let mut board = Board::with_glider();

    loop {
        board.display();
        board.next();
        thread::sleep(Duration::from_millis(100));
        print!("\x1b[{}A", SIZE + 2);
        print!("\x1b[{}D", SIZE);
    }
}
