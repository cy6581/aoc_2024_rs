/// # Commentary
///
/// I was trying to reuse, as much as possible, the grid from day 06. But it didnt' work out, as I
/// go into more detail below.
///
/// # Things I tried and learnt
///
/// - Data structure choice. Using a Hashmap<char<Vec, (row, col)>> to aggregate each group of
/// antennas, where each entry maps the antenna type, e.g. 'A', to a Vec of antenna positions.
/// Actually, all we need is a Vec<Vec<(row, col)>>, without the identifier for the antenna type.
/// However, including the antenna type actually makes it easier to aggregate the data in one pass.
/// I eventually called this HashMap 'antenna_groups'.
///
/// - Who owns the 'antenna_groups' HashMap? Initially, my first thought was to have the Grid struct
/// own it, but turns out it's a bad idea. Assuming that I want to encapsulate the data, I would use
/// a getter to return a reference to 'antenna_groups'. But this would mean that my getter method
/// would need to borrow the entire Grid struct to read 'antenna_groups'. In turn, this would deny
/// me a mutable ref to update state in the Grid struct.
///
/// The problem is that Rust doesn't look at the implementation of the methods where I update state,
/// so it doesn't know that I don't intend to mutate 'antenna_groups' only a separate counter field.
/// One way of solving this would be to in-line everything so that the compiler can figure out that
/// the fields I read and write from are disjoint - but clearly a no-no. Another would be to remove
/// encapsulation and directly allow the caller to read 'antenna_groups' as a field - but I don't
/// like this either. Finally, I decided to have the caller own 'antenna_groups', and write a getter
/// on Grid to return an owned instance of it. Maybe, the larger problem here is, should the Grid
/// struct and 'antenna_groups' be part of the same data structure? Perhaps not.
///
/// - Grid struct less reusable than I thought. One limitation was that my boundary checking fn,
/// `is_next_pos_in_bounds`, made the assumption that we only move one sqaure at a time. This is
/// a valid assumption in many grid exploration problems, but not here. In hindsight, it was a poor
/// fit because we aren't exactly doing an exploration here. So I eventually removed the
/// `is_next_pos_in_bounds` fn. I could have refactored it, if not for the next learning point,
/// which is about issues with integer types.
///
/// - Over-engineering my code to avoid casting between integer types.
/// TLDR: I think for purposes of this challenge, it makes thing so much easier to use casting and
/// accept that it is fine for the nature of this input. We can still be aware with the limits of
/// our code.
///
/// I struggled with this in previous days as well. Looking back, it was obviously a mistake to
/// make things too complicated. My concern was this: Due to the nature of using Vecs to represent
/// the Grid, we have to work with `usize` to represent the respective rows and columns. I think
/// this is in fact the correct and safe abstraction. However, due to the need to represent 4 (or
/// more) directions of movement in a grid, we need to use negative numbers (i.e. `isize`). There
/// are cases where the math operations involving `usize` and `isize` could be unsafe, in particular:
///
/// 1. When we silently cast `row` or `col`, which are of type `usize`, to type `isize`. If the    
/// value is too big to fit into isize (e.g. because the value is close to usize::MAX), an overflow
/// will silently happen, resulting in a negative number.  
/// Solution: Perform a range check before casting, or use TryFrom.
/// 2. When we add 2 `isize`s. If result exceeds an `isize`, it will overflow and either panic, or
/// silently wrapping around (depending on which mode).
/// Solution: Using checked arithmetic, e.g. `checked_add`, or other similar methods.
/// 3. When we cast `isize` back to `usize`. If the value is negative, the bits will be
/// reinterpreted into a (very large) `usize` which is incorrect (sometimes referred to as
/// wraparound).
/// Solution: Perform a check before casting, or use TryFrom.
///
/// I tried to solve problem #1 in Day 6 by writing the `is_next_pos_in_bounds`, but this wasn't
/// sufficiently extendable since it assumed that we move 1 square at a time. When solving Day 8,
/// I realised that there is no way to avoid a conversion if we want to make a generally reusable
/// fn. Finally, my conclusion is: we aren't writing a payment module, and the input sizes are
/// bounded and not expected to go anywhere the danger zones, it's probably fine to just cast away.
///
/// - Finding an elegant way of comparing all-to-all antennas while using nested for-loops: One way
/// was to start the inner loop at index 1 greater than the outer loop, to ensure that we would not
/// repeat the same combination. This had the downside of having to repeat some code twice, since
/// each combination of `tower_a` and `tower_b` can appear in two separate permutations (i.e. the
/// antinode for `tower_a` due to `tower_b` and vice versa). The second way to actually iterate
/// through the entire array of antennas in the inner loop again, but add check for whether
/// `tower_a` != `tower_b`. Although this is less efficient, it is certainly more elegant, since
/// all permutations are taken care of automatically.
///
/// - Using geometric vectors to model the antenna calculations. Here, I'm referring not to the Vec
/// data type in Rust, but the concept of geometric (or Euclidean) vectors. What I did was to
/// iterate through various examples to generalize a formula to calculate the antinodes, which was
/// tedious to do on the fly. Instead, I learnt that you could use a Vector to model the
/// displacement from antenna A to antenna B (Vector A->B (aka from A to B) = B - A). Then it would
/// be a matter of applying the vector to the point B (or the negative to point A) to calculate the
/// antinodes. An even easier method would be to double the Vector from the source antenna A, which
/// would give the antinode further from A. In part 2, we could transform the vector by a linearly
/// increasing scalar and apply it multiple times until we leave the grid to get the series of
/// antinodes. I think this is a much cleaner way to think about it.
use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

struct GridExplorer {
    data: Vec<Vec<char>>,
    marked: Vec<Vec<bool>>,
    rows: usize,
    cols: usize,
    cur_pos: (usize, usize),
    marked_count: u32,
}

impl From<Vec<Vec<char>>> for GridExplorer {
    fn from(data: Vec<Vec<char>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();
        let marked = vec![vec![false; cols]; rows];

        GridExplorer {
            data,
            marked,
            rows,
            cols,
            cur_pos: (0, 0),
            marked_count: 0,
        }
    }
}

impl GridExplorer {
    //
    // Getters
    //
    fn antenna_groups(&self) -> HashMap<char, Vec<(usize, usize)>> {
        let mut antenna_groups = HashMap::new();
        for r in 0..self.rows {
            for c in 0..self.cols {
                let char = self.data[r][c];
                if char != '.' {
                    let vec = antenna_groups.entry(char).or_insert(Vec::new());
                    vec.push((r, c));
                }
            }
        }
        antenna_groups
    }

    fn marked_count(&self) -> u32 {
        self.marked_count
    }

    // UPDATE: there is a much simpler way using the concept of Vectors, i.e.,
    // Vector A->B (aka from A to B) = B - A
    // We basically wish to find the Vector from Dest to Source, which will then be added to
    // source position to produce the antinode closer to source, so subtract Dest from Source:
    // let (srcx, srcy) = (self.cur_pos.0 as isize, self.cur_pos.1 as isize);
    // let (destx, desty) = (dest.0 as isize, dest.1 as isize);
    // (srcx - destx, srcy - desty)
    fn current_antinode_delta_due_to(&self, dest: &(usize, usize)) -> (isize, isize) {
        /*
        Deriving logic from examples
        source: (3, 6)
        dest: (4, 9)
        antinode for source =
        -(4 - 3) because dest > source
        -(9 - 6) because dest > source

        source: (3, 6)
        dest: (4, 5)
        antinode for source =
        -(4 - 3) because dest > source -(dest - source)
        (6 - 5) because source > dest (soruce - dest)
        */

        let (destr, destc) = dest;
        let (sourcer, sourcec) = &self.cur_pos;
        let dr: isize = if sourcer > destr {
            (sourcer - destr) as isize
        } else {
            -((destr - sourcer) as isize)
        };
        let dc: isize = if sourcec > destc {
            (sourcec - destc) as isize
        } else {
            -((destc - sourcec) as isize)
        };
        (dr, dc)
    }

    //
    // Setters
    //
    fn set_current(&mut self, cur: (usize, usize)) {
        self.cur_pos = cur;
    }

    fn mark_antinode_if_inbound(&mut self, delta: (isize, isize)) -> Option<(usize, usize)> {
        // we are confident that the result will fit within usize,
        // and confident that isize will fit the rows and cols
        let (dr, dc) = delta;
        let r = self.cur_pos.0 as isize;
        let c = self.cur_pos.1 as isize;

        if (r + dr) >= 0
            && ((r + dr) as usize) < self.rows
            && (c + dc) >= 0
            && ((c + dc) as usize) < self.cols
        {
            let antinode = ((r + dr) as usize, (c + dc) as usize);
            self.set_marked(antinode);
            Some(antinode)
        } else {
            None
        }
    }

    fn set_marked(&mut self, (row, col): (usize, usize)) {
        if !self.marked[row][col] {
            self.marked[row][col] = true;
            self.marked_count += 1;
        }
    }
}

fn solve_part_one(grid: &Vec<Vec<char>>) -> u32 {
    let mut explorer = GridExplorer::from(grid.clone());

    let antenna_groups = explorer.antenna_groups();
    for group in antenna_groups.values() {
        for (i, &tower_a) in group.iter().enumerate() {
            for &tower_b in &group[i + 1..] {
                explorer.set_current(tower_a);
                let delta = explorer.current_antinode_delta_due_to(&tower_b);
                explorer.mark_antinode_if_inbound(delta);
                // flip around
                explorer.set_current(tower_b);
                let delta = explorer.current_antinode_delta_due_to(&tower_a);
                explorer.mark_antinode_if_inbound(delta);
            }
        }
    }
    explorer.marked_count()
}

fn solve_part_two(grid: &Vec<Vec<char>>) -> u32 {
    let mut explorer = GridExplorer::from(grid.clone());
    let antenna_groups = explorer.antenna_groups();
    for group in antenna_groups.values() {
        for &tower_a in group {
            for &tower_b in group {
                if tower_a != tower_b {
                    // if we have at least a pair, then tower a and tower b are themselves antinodes
                    explorer.set_marked(tower_a);
                    explorer.set_marked(tower_b);
                    explorer.set_current(tower_a);
                    let delta = explorer.current_antinode_delta_due_to(&tower_b);
                    while let Some(antinode) = explorer.mark_antinode_if_inbound(delta) {
                        explorer.set_current(antinode);
                    }
                }
            }
        }
    }
    explorer.marked_count()
}

fn main() {
    // see day_01 for the reason to import the text string this way
    // const INPUT: &'static str = include_str!("../input/day_08_test.txt");
    const INPUT: &'static str = include_str!("../input/day_08.txt");

    let grid = parse_input(INPUT);
    let part_one_answer = solve_part_one(&grid);

    println!("Part one answer: {part_one_answer}");
    assert_eq!(part_one_answer, 269);

    let part_two_answer = solve_part_two(&grid);

    println!("Part two answer: {part_two_answer}");
    assert_eq!(part_two_answer, 949);
}
