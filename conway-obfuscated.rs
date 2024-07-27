use std::time::Duration;
use std::thread;
use std::fmt;

const sz: usize = 10;
const ms: u64 = 150;

#[derive(Copy, Clone, PartialEq)]
enum c {
    d, a,
}

impl fmt::Display for c {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            c::d  => write!(f, "  "),
            c::a => write!(f, "██"),
        }
    }
}

struct brd {
    cls: [[c; sz]; sz],
    g: usize,
}

impl brd {
    fn e() -> Self {
        Self {
            cls: [[c::d; sz]; sz],
            g: 0,
        }
    }

    fn wgl() -> Self {
        let mut cls = [[c::d; sz]; sz];
        
        cls[0][1] = c::a;
        cls[1][2] = c::a;
        cls[2][0] = c::a;
        cls[2][1] = c::a;
        cls[2][2] = c::a;

        Self { cls, g: 0 }
    }

    fn d(&self) {
        for r in 0..sz {
            for c in 0..sz {
                print!("{}", self.cls[r][c]);
            }
            println!();
        }
        self.ds();
    }

    fn ds(&self) {
        println!("Population: {}", self.cp());
        println!("Generation: {}", self.g);
    }

    fn cn(&self, r: usize, c: usize) -> usize {
        let mut n: usize = 0;
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 { continue; }
                let n_c = (((c as i32 + dc) + sz as i32) % sz as i32) as usize;
                let n_r = (((r as i32 + dr) + sz as i32) % sz as i32) as usize;

                if self.cls[n_r][n_c] == c::a { n += 1 }
            }
        }
        return n;
    }

    fn n(&mut self) {
        let mut nx = [[c::d; sz]; sz];

        for r in 0..sz {
            for c in 0..sz {
                let n = self.cn(r, c);
                match self.cls[r][c] {
                    c::d => {
                        if n == 3 {
                            nx[r][c] = c::a;
                        }
                    },
                    c::a => {
                        if n == 2 || n == 3 {
                            nx[r][c] = c::a;
                        }
                    },
                }
            }
        }

        self.cls = nx;
        self.g += 1;
    }

    fn cp(&self) -> usize {
        let mut ppl: usize = 0;
        for r in 0..sz {
            for c in 0..sz {
                if self.cls[r][c] == c::a { ppl += 1 }
            }
        }
        return ppl;
    }
}

fn main() {
    let mut board = brd::wgl();

    loop {
        board.d();
        board.n();
        thread::sleep(Duration::from_millis(ms));
        print!("\x1b[{}A", sz + 2);
        print!("\x1b[{}D", sz);
    }
}
