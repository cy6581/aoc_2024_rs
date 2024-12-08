/// Commentary
///
/// Having seen so many Grid problems, I was frustrated at writing the same code over and over
/// again and decided to create a reusable GridExplorer, and finally get some practice with
/// writing structs.
///
/// As part of an optimization to use the same grid in memory in every brute-force search, I was
/// forced to take a ref to the grid and use dreaded lifetimes.

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

struct GridExplorer<'a> {
    data: &'a mut Vec<Vec<char>>,
    /*
    Using a special notation to mark explored, 'X' means not explored, 'N', 'S', 'E', 'W',
    indicates the direction it was explored form, to help check for cycles
    */
    explored: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    next_move: (isize, isize),
    cur_pos: (usize, usize),
    explored_count: u32,
}

impl<'a> GridExplorer<'a> {
    //
    // Getters
    //
    fn explored_count(&self) -> u32 {
        self.explored_count
    }

    fn is_explored_pos(&self, (r, c): (usize, usize)) -> bool {
        self.explored[r][c] != 'X'
    }

    fn is_invalid_pos(&self, (r, c): (usize, usize)) -> bool {
        self.data[r][c] == '#'
    }

    fn is_next_pos_in_bounds(&self) -> bool {
        // trying out a neater way to check boundaries without casting
        let (r, c) = self.cur_pos;
        let (dr, dc) = self.next_move;
        if r == 0 && dr < 0 {
            false
        } else if c == 0 && dc < 0 {
            false
        } else if r == self.rows - 1 && dr > 0 {
            false
        } else if c == self.cols - 1 && dc > 0 {
            false
        } else {
            true
        }
    }

    fn next_pos(&self) -> Option<(usize, usize)> {
        if self.is_next_pos_in_bounds() {
            // we are confident that the result will fit within usize,
            // and confident that isize will fit the rows and cols
            let (dr, dc) = self.next_move;
            let r = self.cur_pos.0 as isize;
            let c = self.cur_pos.1 as isize;
            let next = ((r + dr) as usize, (c + dc) as usize);
            Some(next)
        } else {
            None
        }
    }

    // Helper
    fn incoming_direction_char(&self) -> char {
        match self.next_move {
            (0, 1) => 'E',  // E
            (1, 0) => 'S',  // S
            (0, -1) => 'W', // W
            (-1, 0) => 'N', // N
            _ => unreachable!(),
        }
    }

    //
    // Setters
    //
    // Returns whether there is a cycle
    fn go_pos(&mut self, pos: (usize, usize)) -> bool {
        self.cur_pos = pos;
        if !self.is_explored_pos(pos) {
            self.set_explored(pos);
            self.explored_count += 1;
            false // no cycle
        } else {
            let (r, c) = pos;
            self.explored[r][c] == self.incoming_direction_char()
        }
    }

    fn set_explored(&mut self, (row, col): (usize, usize)) {
        // record the incoming direction
        self.explored[row][col] = self.incoming_direction_char();
    }

    fn switch_next_move(&mut self) {
        // non-elegant but it works
        let new_move = match self.next_move {
            (0, 1) => (1, 0),   // E to S
            (1, 0) => (0, -1),  // S to W
            (0, -1) => (-1, 0), // W to N
            (-1, 0) => (0, 1),  // N to E
            _ => unreachable!(),
        };
        self.next_move = new_move;
    }
}

// this started as an impl of the 'From' trait but while trying to optimise I realised that it
// only accepts owned input.
impl<'a> GridExplorer<'a> {
    fn from(data: &'a mut Vec<Vec<char>>) -> GridExplorer {
        // TODO, improve the initialization
        // TODO check that rows and cols fit in isize..., or the math may be unsafe
        let mut cur_pos = (0, 0);
        let rows = data.len();
        let cols = data[0].len();
        let mut explored = vec![vec!['X'; cols]; rows];
        for r in 0..rows {
            for c in 0..cols {
                if data[r][c] == '^' {
                    cur_pos = (r, c);
                    explored[r][c] = 'X';
                    break;
                }
            }
        }

        GridExplorer {
            data,
            explored,
            rows,
            cols,
            next_move: (-1, 0), // default to North
            cur_pos,
            explored_count: 1,
        }
    }
}

fn solve_part_one(grid: &Vec<Vec<char>>) -> u32 {
    let mut owned_grid = grid.clone();
    let mut explorer = GridExplorer::from(&mut owned_grid);
    while let Some(pos) = explorer.next_pos() {
        if explorer.is_invalid_pos(pos) {
            explorer.switch_next_move();
        } else {
            explorer.go_pos(pos);
        }
    }
    explorer.explored_count()
}

// clearly this is extremely extremely inefficient... but it works...
// TODO, optimize
// UPDATE 01 - using the same grid in memory for every iteration of the search, saves time
// reallocating the memory
fn solve_part_two(grid: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    let mut mutated_grid = grid.clone();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '.' {
                mutated_grid[row][col] = '#'; // mutate
                let mut explorer = GridExplorer::from(&mut mutated_grid);
                loop {
                    if let Some(pos) = explorer.next_pos() {
                        if explorer.is_invalid_pos(pos) {
                            explorer.switch_next_move();
                        } else {
                            let is_cycle = explorer.go_pos(pos);
                            if is_cycle {
                                count += 1;
                                // println!("Obstructor found {}. {}", row, col);
                                break;
                            }
                        }
                    } else {
                        break; // out of grid
                    }
                }
                mutated_grid[row][col] = '.'; // restore grid to backtrack
            }
        }
    }
    count
}

fn main() {
    // see day_01 for the reason to import the text string this way
    // const INPUT: &'static str = include_str!("../input/day_06_test.txt");
    const INPUT: &'static str = include_str!("../input/day_06.txt");

    let grid = parse_input(INPUT);
    let part_one_answer = solve_part_one(&grid);

    println!("Part one answer: {part_one_answer}");
    assert_eq!(part_one_answer, 5177);

    let part_two_answer = solve_part_two(&grid);

    println!("Part two answer: {part_two_answer}");
    assert_eq!(part_two_answer, 1686);
}
